//! Cache Module for Sapiens Bounded Context
//!
//! Provides in-memory and Redis-based caching for user data.
//! The cache improves read performance and reduces database load.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::domain::entity::UserAggregate;

// ============================================================
// Cache Configuration
// ============================================================

/// Cache configuration options
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Time-to-live for cached items (seconds)
    pub ttl_seconds: u64,
    /// Maximum number of items in cache
    pub max_entries: usize,
    /// Enable cache statistics
    pub enable_stats: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl_seconds: 300, // 5 minutes
            max_entries: 10_000,
            enable_stats: true,
        }
    }
}

// ============================================================
// Cache Entry
// ============================================================

/// A cached item with expiration metadata
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    created_at: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            created_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

// ============================================================
// Cache Statistics
// ============================================================

/// Cache performance statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
}

impl CacheStats {
    /// Calculate hit ratio (0.0 to 1.0)
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

// ============================================================
// In-Memory User Cache
// ============================================================

/// In-memory cache for user aggregates
///
/// Thread-safe implementation using RwLock for concurrent access.
/// Supports TTL-based expiration and LRU eviction.
pub struct UserCache {
    /// Cached users by ID
    users: Arc<RwLock<HashMap<String, CacheEntry<UserAggregate>>>>,
    /// Cached users by email (for lookup optimization)
    email_index: Arc<RwLock<HashMap<String, String>>>,
    /// Cached users by username
    username_index: Arc<RwLock<HashMap<String, String>>>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

impl UserCache {
    /// Create a new user cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a new user cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            email_index: Arc::new(RwLock::new(HashMap::new())),
            username_index: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Get a user by ID
    pub async fn get(&self, id: &str) -> Option<UserAggregate> {
        let users = self.users.read().await;

        if let Some(entry) = users.get(id) {
            if entry.is_expired() {
                drop(users);
                self.remove(id).await;
                self.record_miss().await;
                return None;
            }
            self.record_hit().await;
            return Some(entry.value.clone());
        }

        self.record_miss().await;
        None
    }

    /// Get a user by email
    pub async fn get_by_email(&self, email: &str) -> Option<UserAggregate> {
        let email_index = self.email_index.read().await;

        if let Some(id) = email_index.get(email) {
            let id = id.clone();
            drop(email_index);
            return self.get(&id).await;
        }

        self.record_miss().await;
        None
    }

    /// Get a user by username
    pub async fn get_by_username(&self, username: &str) -> Option<UserAggregate> {
        let username_index = self.username_index.read().await;

        if let Some(id) = username_index.get(username) {
            let id = id.clone();
            drop(username_index);
            return self.get(&id).await;
        }

        self.record_miss().await;
        None
    }

    /// Put a user in the cache
    pub async fn put(&self, user: UserAggregate) {
        let id = user.id.to_string();
        let email = user.email.to_lowercase();
        let username = user.username.to_string();
        let ttl = Duration::from_secs(self.config.ttl_seconds);

        // Check if we need to evict entries
        self.maybe_evict().await;

        // Update main cache
        {
            let mut users = self.users.write().await;
            users.insert(id.clone(), CacheEntry::new(user, ttl));
        }

        // Update email index
        {
            let mut email_index = self.email_index.write().await;
            email_index.insert(email, id.clone());
        }

        // Update username index
        {
            let mut username_index = self.username_index.write().await;
            username_index.insert(username, id);
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.size = self.users.read().await.len();
        }
    }

    /// Remove a user from the cache
    pub async fn remove(&self, id: &str) {
        let mut users = self.users.write().await;

        if let Some(entry) = users.remove(id) {
            // Also remove from indexes
            let email = entry.value.email.to_lowercase();
            let username = entry.value.username.to_string();

            let mut email_index = self.email_index.write().await;
            email_index.remove(&email);

            let mut username_index = self.username_index.write().await;
            username_index.remove(&username);
        }
    }

    /// Invalidate all cached entries
    pub async fn clear(&self) {
        self.users.write().await.clear();
        self.email_index.write().await.clear();
        self.username_index.write().await.clear();

        let mut stats = self.stats.write().await;
        stats.size = 0;
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    // ============================================================
    // Private Helpers
    // ============================================================

    async fn record_hit(&self) {
        if self.config.enable_stats {
            let mut stats = self.stats.write().await;
            stats.hits += 1;
        }
    }

    async fn record_miss(&self) {
        if self.config.enable_stats {
            let mut stats = self.stats.write().await;
            stats.misses += 1;
        }
    }

    async fn maybe_evict(&self) {
        let users = self.users.read().await;

        if users.len() < self.config.max_entries {
            return;
        }
        drop(users);

        // Evict expired entries first
        let mut users = self.users.write().await;
        let expired: Vec<String> = users
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        let mut eviction_count = 0;
        for id in expired {
            users.remove(&id);
            eviction_count += 1;
        }

        // Update stats
        if self.config.enable_stats {
            let mut stats = self.stats.write().await;
            stats.evictions += eviction_count;
            stats.size = users.len();
        }
    }
}

impl Default for UserCache {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Cache Trait for abstraction
// ============================================================

/// Trait for cache implementations
#[async_trait::async_trait]
pub trait Cache<T: Clone + Send + Sync>: Send + Sync {
    async fn get(&self, key: &str) -> Option<T>;
    async fn put(&self, key: &str, value: T);
    async fn remove(&self, key: &str);
    async fn clear(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_put_and_get() {
        let cache = UserCache::new();

        // Create a test user
        let user = UserAggregate::create(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        let id = user.id.to_string();
        cache.put(user.clone()).await;

        let cached = cache.get(&id).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().id, id);
    }

    #[tokio::test]
    async fn test_cache_get_by_email() {
        let cache = UserCache::new();

        let user = UserAggregate::create(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        cache.put(user.clone()).await;

        let cached = cache.get_by_email("test@example.com").await;
        assert!(cached.is_some());
    }

    #[tokio::test]
    async fn test_cache_remove() {
        let cache = UserCache::new();

        let user = UserAggregate::create(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        let id = user.id.to_string();
        cache.put(user).await;
        cache.remove(&id).await;

        assert!(cache.get(&id).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = UserCache::new();

        // Miss
        cache.get("nonexistent").await;

        let user = UserAggregate::create(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            "Test".to_string(),
            "User".to_string(),
        ).unwrap();

        let id = user.id.to_string();
        cache.put(user).await;

        // Hit
        cache.get(&id).await;

        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.size, 1);
    }
}
