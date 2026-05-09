//! Root-capability-aware write-strategy selection.
//!
//! The write strategy is derived from the VFS root's capability envelope and
//! the pinned save-target token. The selection is deterministic: identical
//! tokens always produce the same write strategy.

use aureline_vfs::{AtomicWriteMode, SaveTargetToken};

/// Write strategy derived from the root capability envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteStrategy {
    /// Temp-write + durability barrier + atomic rename over the target.
    AtomicReplace,
    /// Direct write to the target (degraded guarantee).
    InPlaceWrite,
    /// Remote write guarded by a revision precondition.
    ConditionalRemoteWrite,
    /// No write is legal under the advertised capability envelope.
    Blocked,
}

impl WriteStrategy {
    /// Returns the stable string vocabulary for this strategy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicReplace => "atomic_replace",
            Self::InPlaceWrite => "in_place_write",
            Self::ConditionalRemoteWrite => "conditional_remote_write",
            Self::Blocked => "blocked",
        }
    }
}

/// Deterministically selects the write strategy for `token`.
pub fn select_write_strategy(token: &SaveTargetToken) -> WriteStrategy {
    // Policy gates are surfaced as `blocked` before any bytes move, regardless
    // of which save mode the root prefers.
    if token.capability_flags.read_only || token.capability_flags.policy_constrained {
        return WriteStrategy::Blocked;
    }
    if token.review_required_before_save {
        return WriteStrategy::Blocked;
    }
    if token.atomic_write_mode == AtomicWriteMode::AtomicReplace
        && token.review_required_before_rename
    {
        return WriteStrategy::Blocked;
    }

    match token.atomic_write_mode {
        AtomicWriteMode::AtomicReplace => WriteStrategy::AtomicReplace,
        AtomicWriteMode::InPlaceWrite => WriteStrategy::InPlaceWrite,
        AtomicWriteMode::ConditionalRemoteWrite => WriteStrategy::ConditionalRemoteWrite,
        AtomicWriteMode::Blocked => WriteStrategy::Blocked,
    }
}
