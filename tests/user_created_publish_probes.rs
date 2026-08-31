//! UserCreated publish probes — the user-creation event contract against a REAL Postgres.
//! Requires DATABASE_URL pointing at a DB with the `sapiens` schema migrated (the .up.sql
//! files in order). The connecting user needs INSERT on users, organization_users and
//! sapiens.outbox_events.
//!
//! UCP-1 the admin/CRUD create path stages a durable outbox row: create → exactly one
//!      `sapiens.outbox_events` row with event_type 'UserCreated', aggregate_type 'User',
//!      aggregate_id = the new user id, payload = the serialized UserDomainEvent::Created.
//! UCP-2 a re-fired create (same email) does not double-publish: the users table's
//!      soft-delete-aware email index does not actually refuse live duplicates (NULLs
//!      are distinct), so the publish seam itself suppresses the duplicate's event —
//!      one email ⇒ at most one UserCreated; a soft-deleted email re-created is a new
//!      account and publishes normally.
//! UCP-3 self-registration publishes EXACTLY once: one outbox row staged in the
//!      registration transaction, one envelope on the typed domain bus; a re-register of
//!      the same email is refused with no additional row or envelope.
//! UCP-4 the typed bus delivers through the SapiensIntegrationEventPublisher translator:
//!      a registered handler's publish surfaces as the cross-module integration event
//!      `sapiens.user.created` on the integration bus.
//! UCP-5 the internal-user definition: a user with no organization_users membership is
//!      NOT internal; an ACTIVE membership makes them internal; a non-active membership
//!      does not; the batch helper agrees with the single-user predicate.

use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use backbone_sapiens::UserCreatedOutboxPublisher;
use backbone_sapiens::application::service::{AuthService, RegisterInput, UserService};
use backbone_sapiens::domain::entity::UserStatus;
use backbone_sapiens::infrastructure::auth::email::AuthEmailService;
use backbone_sapiens::infrastructure::auth::jwt::{JwtConfig, JwtService};
use backbone_sapiens::infrastructure::messaging::{
    EventBus as UserDomainEventBus, IntegrationEventBus, SapiensIntegrationEventPublisher,
    create_sapiens_event_bus,
};
use backbone_sapiens::infrastructure::persistence::{
    EmailVerificationTokenRepository, PasswordResetTokenRepository, SessionRepository,
    UserRepository,
};
use backbone_sapiens::{internal_user_ids, is_internal_user};
use backbone_sapiens::presentation::dto::CreateUserDto;

fn d(s: &str) -> String {
    format!("{}-{}", s, &Uuid::new_v4().simple().to_string()[..8])
}

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5433/backbone_sapiens".to_string()
    });
    PgPool::connect(&url).await.expect("connect DB")
}

async fn outbox_rows(pool: &PgPool, aggregate_id: Uuid) -> i64 {
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events \
         WHERE aggregate_id = $1 AND event_type = 'UserCreated'",
    )
    .bind(aggregate_id.to_string())
    .fetch_one(pool)
    .await
    .expect("count outbox rows");
    n
}

async fn outbox_rows_for_email(pool: &PgPool, email: &str) -> i64 {
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sapiens.outbox_events o \
         JOIN public.users u ON u.id::text = o.aggregate_id \
         WHERE u.email = $1 AND o.event_type = 'UserCreated'",
    )
    .bind(email)
    .fetch_one(pool)
    .await
    .expect("count outbox rows for email");
    n
}

/// A CRUD user service wired exactly like the module build wires it: the outbox publisher
/// attached, no integration bus (the in-process publish then drops silently — staging
/// still happens).
async fn crud_service(pool: &PgPool) -> Arc<UserService> {
    Arc::new(
        UserService::with_repository(Arc::new(UserRepository::new(pool.clone())))
            .with_event_publisher(Arc::new(UserCreatedOutboxPublisher::new(pool.clone(), None))),
    )
}

fn create_user_dto(email: &str, username: &str) -> CreateUserDto {
    CreateUserDto {
        username: username.to_string(),
        email: email.to_string(),
        password_hash: d("argon2idph").repeat(6),
        status: UserStatus::Active,
        email_verified: false,
        failed_login_attempts: 0,
        locked_until: None,
        last_login: None,
    }
}

#[tokio::test]
async fn ucp1_admin_crud_create_stages_outbox_row() {
    let pool = pool().await;
    let service = crud_service(&pool).await;

    let user = service
        .create(create_user_dto(&d("probe@example.com"), &d("probe").repeat(2)))
        .await
        .expect("create user");

    let rows = outbox_rows(&pool, user.id).await;
    assert_eq!(rows, 1, "one UserCreated outbox row per created user");

    let (aggregate_type, payload_event, payload_user): (String, String, String) =
        sqlx::query_as(
            "SELECT aggregate_type, payload->>'event_type', payload->>'user_id' \
             FROM sapiens.outbox_events WHERE aggregate_id = $1",
        )
        .bind(user.id.to_string())
        .fetch_one(&pool)
        .await
        .expect("fetch outbox row");
    assert_eq!(aggregate_type, "User");
    assert_eq!(payload_event, "Created", "payload is the serialized domain event");
    assert_eq!(payload_user, user.id.to_string());
}

#[tokio::test]
async fn ucp2_refired_admin_create_does_not_double_publish() {
    let pool = pool().await;
    let service = crud_service(&pool).await;

    let email = d("refire@example.com");
    let first = service
        .create(create_user_dto(&email, &d("refire").repeat(2)))
        .await
        .expect("first create");
    assert_eq!(outbox_rows(&pool, first.id).await, 1);
    assert_eq!(outbox_rows_for_email(&pool, &email).await, 1);

    // Re-fire the same create. The users table's soft-delete-aware email unique index
    // (email, metadata->>'deleted_at') treats NULLs as distinct, so the database does
    // NOT refuse the second live row — it inserts a duplicate account with a new id.
    // The publish seam must absorb that: the duplicate's UserCreated is suppressed, so
    // the email never gains a second event and the original user's event is untouched.
    let second = service
        .create(create_user_dto(&email, &d("other").repeat(2)))
        .await;
    match second {
        Err(_) => { /* refused by the database — even better; nothing was published */ }
        Ok(dup) => {
            assert_ne!(dup.id, first.id, "expected the re-fire to insert a new row");
            assert_eq!(
                outbox_rows(&pool, dup.id).await,
                0,
                "a re-fired duplicate must not stage its own UserCreated"
            );
        }
    }
    assert_eq!(
        outbox_rows(&pool, first.id).await,
        1,
        "the original user's event count is unchanged"
    );
    assert_eq!(
        outbox_rows_for_email(&pool, &email).await,
        1,
        "one email ⇒ one UserCreated, no matter how many times the create re-fires"
    );

    // Creating a previously soft-deleted email is a genuinely new account and MUST
    // still publish: soft-delete every live row for the email, re-create, expect a new
    // event.
    sqlx::query("UPDATE public.users SET metadata = jsonb_set(metadata, '{deleted_at}', to_jsonb(clock_timestamp()::text)) WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await
        .expect("soft-delete all rows for the email");
    let recreated = service
        .create(create_user_dto(&email, &d("fresh").repeat(2)))
        .await
        .expect("re-create after soft delete");
    assert_eq!(
        outbox_rows(&pool, recreated.id).await,
        1,
        "a fresh account for a soft-deleted email publishes normally"
    );
}

/// Build the AuthService with the typed domain bus wired the way the module build wires
/// it when an integration bus is supplied: bus constructed, translator registered.
async fn auth_service_with_bus(
    pool: &PgPool,
) -> (Arc<AuthService>, Arc<UserDomainEventBus>, Arc<IntegrationEventBus>) {
    let integration_bus = Arc::new(IntegrationEventBus::new());
    let domain_bus = Arc::new(create_sapiens_event_bus());
    domain_bus
        .register_handler(Arc::new(SapiensIntegrationEventPublisher::new(
            integration_bus.clone(),
        )))
        .await;

    let auth = Arc::new(
        AuthService::new(
            Arc::new(UserRepository::new(pool.clone())),
            Arc::new(SessionRepository::new(pool.clone())),
            Arc::new(EmailVerificationTokenRepository::new(pool.clone())),
            Arc::new(PasswordResetTokenRepository::new(pool.clone())),
            Arc::new(JwtService::new(JwtConfig::from_env())),
            Arc::new(AuthEmailService::from_env()),
            pool.clone(),
        )
        .with_domain_event_bus(Some(domain_bus.clone())),
    );
    (auth, domain_bus, integration_bus)
}

fn register_input(email: &str) -> RegisterInput {
    RegisterInput {
        email: email.to_string(),
        password: "Sup3rSecret".to_string(),
        confirm_password: "Sup3rSecret".to_string(),
        first_name: None,
        last_name: None,
        accept_terms: true,
        username: None,
    }
}

#[tokio::test]
async fn ucp3_self_registration_publishes_exactly_once() {
    let pool = pool().await;
    let (auth, domain_bus, _integration_bus) = auth_service_with_bus(&pool).await;
    let mut envelopes = domain_bus.subscribe();

    let email = d("selfreg@example.com");
    let result = auth.register(register_input(&email)).await.expect("register");

    // Durable: one outbox row staged in the registration transaction.
    assert_eq!(outbox_rows(&pool, result.user_id).await, 1);

    // Immediate: exactly one envelope on the typed bus, and no second one.
    let envelope = envelopes.recv().await.expect("one UserCreated envelope");
    assert_eq!(envelope.event_type, "UserCreated");
    assert_eq!(envelope.aggregate_id, result.user_id.to_string());
    assert!(
        envelopes.try_recv().is_err(),
        "self-registration must publish exactly once"
    );

    // A re-register of the same email is refused and publishes nothing further.
    let again = auth.register(register_input(&email)).await;
    assert!(again.is_err(), "duplicate self-registration must be refused");
    assert_eq!(outbox_rows(&pool, result.user_id).await, 1);
    assert!(
        envelopes.try_recv().is_err(),
        "a refused re-register must not publish"
    );
}

#[tokio::test]
async fn ucp4_typed_bus_delivers_integration_event() {
    let pool = pool().await;
    let (auth, _domain_bus, integration_bus) = auth_service_with_bus(&pool).await;
    let mut integration_rx = integration_bus.subscribe();

    let result = auth
        .register(register_input(&d("integration@example.com")))
        .await
        .expect("register");

    let envelope = integration_rx
        .recv()
        .await
        .expect("integration envelope via the translator");
    assert_eq!(envelope.event_type, "sapiens.user.created");
    assert_eq!(envelope.source_context, "sapiens");
    assert_eq!(envelope.aggregate_id, result.user_id.to_string());
}

#[tokio::test]
async fn ucp5_internal_user_definition() {
    let pool = pool().await;
    let service = crud_service(&pool).await;

    let user = service
        .create(create_user_dto(&d("internal@example.com"), &d("internal").repeat(2)))
        .await
        .expect("create user");

    // A create with no organization membership is external by definition — this is also
    // the recorded behavior for a bare CRUD create: the event fires regardless, and the
    // internal/external decision belongs to the consumer applying the predicate.
    assert!(
        !is_internal_user(&pool, user.id).await.unwrap(),
        "no membership ⇒ not internal"
    );

    // A non-active membership does not make the user internal.
    sqlx::query(
        "INSERT INTO sapiens.organization_users (organization_id, user_id, status) \
         VALUES ($1, $2, 'inactive')",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .execute(&pool)
    .await
    .expect("insert inactive membership");
    assert!(
        !is_internal_user(&pool, user.id).await.unwrap(),
        "inactive membership ⇒ not internal"
    );

    // An ACTIVE membership is the definition of internal.
    sqlx::query(
        "INSERT INTO sapiens.organization_users (organization_id, user_id, status) \
         VALUES ($1, $2, 'active')",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .execute(&pool)
    .await
    .expect("insert active membership");
    assert!(
        is_internal_user(&pool, user.id).await.unwrap(),
        "active membership ⇒ internal"
    );

    // The batch helper agrees with the single-user predicate.
    let batch = internal_user_ids(&pool, &[user.id, Uuid::new_v4()])
        .await
        .unwrap();
    assert_eq!(batch, vec![user.id]);
}
