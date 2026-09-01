//! Durable rate limiting for the public auth forms.
//!
//! Fixed-window counters in `sapiens.auth_throttle_buckets`, keyed by
//! (scope, key, window_start). Both dimensions the posture requires are first
//! class: per-IDENTITY buckets (the submitted email address) and per-IP
//! buckets (the connecting client). The counters live in Postgres, so limits
//! hold across restarts and are shared by every replica — deliberately NOT a
//! process-local limiter.
//!
//! Counting is one atomic upsert per attempt (`ON CONFLICT ... count + 1
//! RETURNING count`), so concurrent requests cannot race past the limit.

use chrono::{TimeDelta, Utc};
use sqlx::PgPool;

/// Scope dimension a bucket counts on. The key format differs per scope so
/// buckets can never collide across dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleScope {
    /// Per submitted identity (normalized email address).
    Identity,
    /// Per connecting client address.
    Ip,
}

impl ThrottleScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Ip => "ip",
        }
    }
}

/// One limit declaration: how many events per window on a scope.
#[derive(Clone, Copy, Debug)]
pub struct ThrottleLimit {
    pub scope: ThrottleScope,
    pub max: i64,
    pub window: TimeDelta,
}

impl ThrottleLimit {
    pub fn identity(max: i64, window: TimeDelta) -> Self {
        Self { scope: ThrottleScope::Identity, max, window }
    }

    pub fn ip(max: i64, window: TimeDelta) -> Self {
        Self { scope: ThrottleScope::Ip, max, window }
    }

    /// Const constructor for declaring fixed postures at their definition
    /// site (same shape as [`ThrottleLimit::identity`], usable in a `const`).
    pub const fn identity_const(max: i64, window: TimeDelta) -> Self {
        Self { scope: ThrottleScope::Identity, max, window }
    }

    /// Const constructor for the per-IP dimension (see
    /// [`ThrottleLimit::identity_const`]).
    pub const fn ip_const(max: i64, window: TimeDelta) -> Self {
        Self { scope: ThrottleScope::Ip, max, window }
    }

    fn bucket_key(self, subject: &str) -> String {
        match self.scope {
            ThrottleScope::Identity => format!("identity:{}", subject.trim().to_lowercase()),
            ThrottleScope::Ip => format!("ip:{}", subject),
        }
    }
}

/// Outcome of an attempt against one limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleDecision {
    /// The attempt is within the limit and was counted.
    Allowed,
    /// The window's budget is exhausted; the attempt was NOT counted (the
    /// requester is already refused, so it must not also spend budget).
    Refused,
}

/// Durable fixed-window throttler.
pub struct AuthThrottleService {
    pool: PgPool,
}

impl AuthThrottleService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Count one attempt against one limit and report whether it fits.
    ///
    /// A database failure is ALLOWING (fail open): the throttle is a
    /// load/abuse control, and taking the whole auth surface down because the
    /// counter table is unavailable would trade a soft limit for a hard
    /// outage. The underlying forms still have their own defenses
    /// (verification-token attempt caps, login lockouts, de-oracled replies).
    pub async fn check_and_increment(
        &self,
        limit: ThrottleLimit,
        subject: &str,
    ) -> ThrottleDecision {
        // Fixed windows aligned to the epoch: for a 1-hour window every bucket
        // starts on the hour, for a 1-minute window on the minute. Cheap,
        // deterministic, and shared by every replica. The start is TRUNCATED
        // to the whole-second boundary — subtracting an offset from `now`
        // would keep its microseconds and give every attempt its own bucket.
        let now = Utc::now();
        let window_secs = limit.window.num_seconds().max(1);
        let window_start_secs = now.timestamp() - now.timestamp().rem_euclid(window_secs);
        let window_start = chrono::DateTime::from_timestamp(window_start_secs, 0)
            .unwrap_or(now);
        let key = limit.bucket_key(subject);
        let scope = limit.scope.as_str();

        let counted: i64 = match sqlx::query_scalar(
            "INSERT INTO sapiens.auth_throttle_buckets (scope, bucket_key, window_start, count) \
             VALUES ($1, $2, $3, 1) \
             ON CONFLICT (scope, bucket_key, window_start) \
             DO UPDATE SET count = sapiens.auth_throttle_buckets.count + 1 \
             RETURNING count",
        )
        .bind(scope)
        .bind(&key)
        .bind(window_start)
        .fetch_one(&self.pool)
        .await
        {
            Ok(n) => n,
            Err(_) => return ThrottleDecision::Allowed,
        };

        if counted > limit.max {
            return ThrottleDecision::Refused;
        }
        ThrottleDecision::Allowed
    }

    /// Best-effort cleanup of long-dead windows. Called opportunistically by
    /// the hot path; failures are ignored (rows are small and window_start is
    /// indexed).
    pub async fn prune(&self) {
        let _ = sqlx::query(
            "DELETE FROM sapiens.auth_throttle_buckets WHERE window_start < NOW() - INTERVAL '2 days'",
        )
        .execute(&self.pool)
        .await;
    }
}
