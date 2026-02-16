//! Todoist Sync API Client
//!
//! This module provides a client for Todoist's Sync API v1.
//! The Sync API allows for efficient batch operations and incremental synchronization.

mod cache;
mod client;
mod commands;
mod models;

pub use cache::{Cache, CacheManager};

pub use client::{CacheStatus, TodoistSyncClient};
pub use commands::{
    Command, CommandBuilder, FilterAddArgs, FilterOrderArgs, ItemAddArgs, ItemUpdateArgs,
    LabelAddArgs, ProjectAddArgs, SectionAddArgs,
};
pub use models::{SyncDue, SyncFilter, SyncLabel, SyncProject, SyncSection, SyncTask};
