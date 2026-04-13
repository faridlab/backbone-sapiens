//! CLI Module for Sapiens Bounded Context
//!
//! Provides command-line interface for user management operations.
//! Used for administrative tasks, testing, and automation.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod user_commands;

pub use user_commands::{UserCommands, UserAction, UserCliHandler};
