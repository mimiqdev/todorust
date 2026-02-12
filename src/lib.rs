//! # TodoRust - Todoist CLI Client
//!
//! A modern CLI client for Todoist built with Rust.
//!
//! ## Features
//!
//! - **Sync API**: Efficient batch operations using Todoist's Sync API
//! - **Multiple Output Formats**: JSON, Checklist, and Structured views
//! - **Task Management**: Create, complete, reopen, and organize tasks
//!
//! ## Usage
//!
//! ```bash
//! # Initialize configuration
//! todorust init --api-token YOUR_TOKEN
//!
//! # List tasks
//! todorust tasks
//! todorust tasks --filter "today"
//!
//! # Create a task
//! todorust create --title "Buy milk"
//!
//! # Complete a task
//! todorust complete --task-id 12345
//! ```
//!
//! ## Modules
//!
//! - [`sync`]: Todoist Sync API client for efficient batch operations
//! - [`api`]: Legacy REST API client (deprecated, use [`sync`] instead)
//! - [`formatter`]: Output formatting utilities

pub mod api;
pub mod config;
pub mod error;
pub mod formatter;
pub mod models;
pub mod sync;

pub use formatter::{Formattable, OutputFormat};
pub use models::Project;
pub use sync::{SyncFilter, SyncLabel, SyncProject, SyncSection, SyncTask, TodoistSyncClient};
