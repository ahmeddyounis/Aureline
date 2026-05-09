//! Staged save coordinator and root-capability-aware write strategies.
//!
//! This module owns the canonical save sequencing for workspace-backed
//! documents:
//!
//! 1. stage a buffer snapshot,
//! 2. run save participants on staged content,
//! 3. compare-before-write against the pinned save target,
//! 4. select a write strategy using the root capability envelope, and
//! 5. commit the write through an atomic or declared degraded lane.
//!
//! Consumers MUST NOT bypass this coordinator with ad hoc filesystem writes,
//! because doing so breaks conflict safety, journaling, and later support/export
//! attribution.

pub mod coordinator;
pub mod write_strategy;

pub use coordinator::{
    SaveParticipant, SaveParticipantError, SaveResult, StagedSaveCoordinator, StagedSaveRequest,
};
pub use write_strategy::WriteStrategy;
