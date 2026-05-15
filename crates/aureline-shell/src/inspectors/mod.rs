//! Inspector surfaces that quote canonical governance and runtime evidence.

use aureline_settings::inspector::conflict::{
    project_sync_conflict_review_surface, SyncConflictPacket, SyncConflictReviewSurface,
};

pub mod schema_registry;

/// Presents a settings-sync conflict through the canonical settings review surface.
pub fn present_settings_sync_conflict(packet: &SyncConflictPacket) -> SyncConflictReviewSurface {
    project_sync_conflict_review_surface(packet)
}
