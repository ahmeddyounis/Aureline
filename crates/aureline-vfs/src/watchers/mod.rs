//! Workspace watcher service.
//!
//! The watcher service normalizes filesystem change notifications through one
//! event stream. Consumers should subscribe to this stream rather than wiring
//! their own per-feature watcher or polling loop (ADR 0006).

mod polling;
mod service;
mod types;

pub use service::{WatcherService, WatcherServiceError, WatcherServiceOptions};
pub use types::{VfsChangeEvent, VfsChangeKind, WatcherEvent};
