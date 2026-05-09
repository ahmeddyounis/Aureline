//! Workspace entry vocabulary and recent-work registry.
//!
//! This crate owns the canonical target-kind model and the persisted recent-work
//! registry read by shell entry surfaces (Start Center, workspace switcher, and
//! `Open Recent`).
//!
//! Primary sources:
//! - `docs/workspace/entry_restore_object_model.md`
//! - `schemas/workspace/entry_and_restore_result.schema.json`

#![doc(html_root_url = "https://docs.rs/aureline-workspace/0.0.0")]

pub mod entry_flows;
pub mod lifecycle;
pub mod recent_work;
pub mod save;

pub use recent_work::{
    EntryAndRestoreSchemaVersion, PortabilityClass, RecentWorkEntryRecord,
    RecentWorkEntryRecordKind, RecentWorkRegistry, RecentWorkRegistryError,
    RecentWorkRegistryRecordKind, RecentWorkTargetState, RestoreAvailability, SafeRecoveryAction,
    TargetKind, TrustState,
};

pub use lifecycle::{
    WorkspaceLifecycleMachine, WorkspaceLifecycleSnapshot, WorkspaceLifecycleSnapshotRecordKind,
    WorkspaceLifecycleState, WorkspaceLifecycleTransitionFrame,
};

pub use entry_flows::{
    resolve_entry_flow, EntryFlowDenialCode, EntryFlowDenied, EntryFlowOutcome, EntryFlowRequest,
    EntryFlowResolved, EntryFlowTarget, EntryVerb, OpenFlowSheetClass, ResultingMode,
};

pub use save::{
    SaveParticipant, SaveParticipantError, SaveResult, StagedSaveCoordinator, StagedSaveRequest,
    WriteStrategy,
};
