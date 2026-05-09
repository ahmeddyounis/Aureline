//! Watcher event vocabulary.
//!
//! Change events intentionally stay small and structured: they carry the root id
//! plus normalized change kinds (`create`, `modify`, `delete`, `rename`, or
//! `rescan`). When a backend cannot provide enough fidelity (for example a
//! polling scanner without stable rename pairing), it emits `rescan`.

/// Normalized change taxonomy emitted by the watcher service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsChangeKind {
    Created {
        uri: crate::uri_model::VfsUri,
    },
    Modified {
        uri: crate::uri_model::VfsUri,
    },
    Deleted {
        uri: crate::uri_model::VfsUri,
    },
    Renamed {
        from: crate::uri_model::VfsUri,
        to: crate::uri_model::VfsUri,
    },
    Rescan,
}

/// One normalized change event emitted by the watcher service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VfsChangeEvent {
    pub root_id: String,
    pub kind: VfsChangeKind,
}

/// Unified event stream emitted by [`crate::watchers::WatcherService`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatcherEvent {
    Change(VfsChangeEvent),
    Health(crate::watcher::WatcherHealthFrame),
}
