//! Mock Repository Implementations
//!
//! Simple mock implementations for testing without external dependencies.

use backbone_sapiens::domain::entity::{User, Session, MFADevice, EmailVerificationToken, Role, Permission};
use backbone_sapiens::domain::repositories::UserRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Mock User Repository
#[derive(Clone)]
pub struct MockUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
    email_index: Arc<RwLock<HashMap<String, String>>>, // email -> id
    username_index: Arc<RwLock<HashMap<String, String>>, // username -> id
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            email_index: Arc::new(RwLock::new(HashMap::new())),
            username_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_user(&self, user: User) {
        let id = user.id.to_string();
        let email = user.email.clone();
        let username = user.username.clone();

        self.users.write().await.insert(id.clone(), user);
        self.email_index.write().await.insert(email, id.clone());
        self.username_index.write().await.insert(username, id);
    }

    pub async fn clear(&self) {
        self.users.write().await.clear();
        self.email_index.write().await.clear();
        self.username_index.write().await.clear();
    }
}

impl Default for MockUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.users.read().await.get(id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(id) = self.email_index.read().await.get(email) {
            Ok(self.users.read().await.get(id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(id) = self.username_index.read().await.get(username) {
            Ok(self.users.read().await.get(id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.email_index.read().await.contains_key(email))
    }

    async fn save(&self, user: &User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let id = user.id.to_string();
        let email = user.email.clone();
        let username = user.username.clone();

        self.users.write().await.insert(id.clone(), user.clone());
        self.email_index.write().await.insert(email, id.clone());
        self.username_index.write().await.insert(username, id);

        Ok(user.clone())
    }

    async fn update(&self, id: &str, user: &User) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        if self.users.read().await.contains_key(id) {
            self.users.write().await.insert(id.to_string(), user.clone());
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }
}

/// Mock Session Repository
#[derive(Clone)]
pub struct MockSessionRepository {
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    user_sessions: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>, // user_id -> session_ids
}

impl MockSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            user_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_session(&self, session: Session) {
        let id = session.id;
        let user_id = session.user_id;

        self.sessions.write().await.insert(id, session);
        self.user_sessions
            .write()
            .await
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(id);
    }

    pub async fn clear(&self) {
        self.sessions.write().await.clear();
        self.user_sessions.write().await.clear();
    }
}

impl Default for MockSessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock MFADevice Repository
#[derive(Clone)]
pub struct MockMFADeviceRepository {
    devices: Arc<RwLock<HashMap<Uuid, MFADevice>>>,
    user_devices: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
}

impl MockMFADeviceRepository {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            user_devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_device(&self, device: MFADevice) {
        let id = device.id;
        let user_id = device.user_id;

        self.devices.write().await.insert(id, device);
        self.user_devices
            .write()
            .await
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(id);
    }
}

impl Default for MockMFADeviceRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock EmailVerificationToken Repository
#[derive(Clone)]
pub struct MockEmailVerificationTokenRepository {
    tokens: Arc<RwLock<HashMap<String, EmailVerificationToken>>>,
}

impl MockEmailVerificationTokenRepository {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_token(&self, token: EmailVerificationToken) {
        let token_str = token.token.clone();
        self.tokens.write().await.insert(token_str, token);
    }
}

impl Default for MockEmailVerificationTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock Role Repository
#[derive(Clone)]
pub struct MockRoleRepository {
    roles: Arc<RwLock<HashMap<String, Role>>>,
}

impl MockRoleRepository {
    pub fn new() -> Self {
        Self {
            roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_role(&self, role: Role) {
        let name = role.name.clone();
        self.roles.write().await.insert(name, role);
    }
}

impl Default for MockRoleRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock Permission Repository
#[derive(Clone)]
pub struct MockPermissionRepository {
    permissions: Arc<RwLock<HashMap<String, Permission>>>,
}

impl MockPermissionRepository {
    pub fn new() -> Self {
        Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_permission(&self, permission: Permission) {
        let key = format!("{}:{}", permission.resource, permission.action);
        self.permissions.write().await.insert(key, permission);
    }
}

impl Default for MockPermissionRepository {
    fn default() -> Self {
        Self::new()
    }
}
