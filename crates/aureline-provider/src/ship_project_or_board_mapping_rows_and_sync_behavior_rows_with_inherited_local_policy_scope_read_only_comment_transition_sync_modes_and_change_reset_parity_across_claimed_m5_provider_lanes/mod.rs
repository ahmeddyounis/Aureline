//! Two reusable M5 provider primitives — the project/board mapping row and the
//! sync-behavior row — so a user can tell, from the row alone, *which* provider project or
//! board a lookup or write will target and *how* Aureline keeps local and provider truth in
//! step, before any live read, write, or mirror is attempted.
//!
//! Aureline's frozen provider-account / mapping / offline-capture component matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`])
//! names the project/board mapping row and the sync-behavior row as two governed component
//! families and freezes their controlled vocabulary — the mapping origin classes, the
//! mapping target kinds, the provider sync modes, the effective write scopes, and the
//! queued-draft states, plus the surface families, the deployment lines, the consumer
//! surfaces, the accessibility routes, the qualification classes, and the downgrade
//! triggers. This module *ships* that contract as two reusable resolvers so a user never
//! has to guess which provider project/board a lookup or write will target, and provider
//! surfaces stop using one ambiguous `synced` label for materially different write and
//! mirroring behaviors.
//!
//! The module has two resolvers, one per family:
//!
//! 1. [`resolve_project_board_mapping_row`] — takes one mapping's provider project/space
//!    label, its repo/workspace relation, its mapping target kind, its mapping origin, and
//!    an optional lock note, and produces one [`M5ResolvedMappingRow`] carrying the derived
//!    mapping scope (inherited, local, policy, or unmapped), the derived row posture (one
//!    per mapping origin), whether the row points at an explicit destination or must flag
//!    itself unmapped, whether the mapping is policy-locked, and the bounded reveal / change
//!    / reset / export actions. It never masks the mapping origin or lock, never collapses
//!    the target kinds into one generic "mapped" chip, and — above all — never assumes a
//!    default publish destination silently.
//! 2. [`resolve_sync_behavior_row`] — takes one row's provider sync mode, its effective
//!    write scope, and its queued-draft state, and produces one [`M5ResolvedSyncRow`]
//!    carrying the derived sync-behavior class (full bidirectional, comment/link,
//!    status-transition, read-only metadata, offline-capture-only, or paused), whether
//!    Aureline can write live, whether the row is a read-only mirror, and whether local
//!    work remains queued — with the bounded reveal / change-mode / view-queue /
//!    retry-publish / export actions. It never collapses the six sync modes into one generic
//!    `synced` label and never hides the local-draft queue state.
//!
//! A single parity matrix — [`M5ProviderMappingSyncRowPacket`] — binds one row per claimed
//! M5 provider surface consumer (the mapping-picker panel, the sync-behavior panel, the
//! provider status bar, the headless/CLI mappings surface, and the support mapping export)
//! to the shared mapping-row and sync-row anatomy, the same mapping origins, target kinds,
//! mapping scopes, sync modes, write scopes, sync-behavior classes, queued-draft states,
//! bounded actions, export fields, and non-visual accessibility routes, so the destination
//! and publication-mode vocabulary stays identical across desktop, headless/export, and
//! support consumers.
//!
//! The mapping origin class ([`M5MappingOriginClass`]), mapping target kind
//! ([`M5MappingTargetKind`]), provider sync mode ([`M5ProviderSyncMode`]), effective write
//! scope ([`M5ProviderWriteScope`]), queued-draft state ([`M5QueuedDraftState`]), surface
//! family ([`M5ProviderSurfaceFamily`]), deployment line ([`M5ProviderDeploymentLine`]),
//! consumer surface ([`M5ProviderConsumerSurface`]), accessibility route
//! ([`M5ProviderAccessibilityRoute`]), qualification class
//! ([`M5ProviderQualificationClass`]), and downgrade trigger
//! ([`M5ProviderDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the two rows
//! themselves: the mapping scope, the two derived row postures, the sync-behavior class,
//! their bounded actions, their anatomy parts, and their export fields. No M5 provider
//! surface invents a second mapping-row or sync-row grammar.
//!
//! Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every project label, relation, and mapping/sync identity is carried only
//! as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed,
    seeded_m5_provider_mapping_sync_row_packet,
    seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed,
    M5_PROVIDER_MAPPING_SYNC_ROW_PACKET_ID,
};

// The mapping origin class, mapping target kind, provider sync mode, effective write scope,
// queued-draft state, surface family, deployment line, consumer surface, accessibility
// route, qualification class, and downgrade triggers are frozen once, in the
// provider-account / offline-capture component matrix. This primitive reuses them verbatim
// so it never invents a parallel mapping or sync vocabulary.
pub use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5MappingOriginClass, M5MappingTargetKind, M5ProviderAccessibilityRoute,
    M5ProviderConsumerSurface, M5ProviderDeploymentLine, M5ProviderDowngradeTrigger,
    M5ProviderQualificationClass, M5ProviderSurfaceFamily, M5ProviderSyncMode,
    M5ProviderWriteScope, M5QueuedDraftState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderMappingSyncRowPacket`].
pub const M5_PROVIDER_MAPPING_SYNC_ROW_RECORD_KIND: &str =
    "ship_m5_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes";

/// Schema version for M5 provider mapping/sync-row records.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the mapping/sync-row boundary schema.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-mapping-sync-behavior-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_DOC_REF: &str =
    "docs/providers/m5_provider_mapping_sync_behavior_row_primitive.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix this
/// primitive narrows from.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the provider-target-mapping contract this primitive binds its
/// mapping / destination truth against.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_TARGET_MAPPING_REF: &str =
    "schemas/providers/provider_target_mapping.schema.json";

/// Repo-relative path of the provider-sync-health contract this primitive binds its sync /
/// write-scope truth against.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_SYNC_HEALTH_REF: &str =
    "schemas/providers/provider_sync_health_view.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-mapping-sync-behavior-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_CSV_REF: &str =
    "artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_REPORT_REF: &str =
    "artifacts/design/m5-provider-mapping-sync-behavior-row-primitive.md";

/// One claimed M5 provider-surface consumer that renders the shared mapping and sync-behavior
/// rows. These are the consumers the acceptance criteria name — the mapping-picker panel, the
/// sync-behavior panel, the provider status bar, the headless/CLI mappings surface, and the
/// support mapping export — so the same mapping/sync grammar works across every claimed
/// provider surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingSyncConsumerSurface {
    /// The mapping-picker panel surface.
    MappingPickerPanel,
    /// The sync-behavior panel surface.
    SyncBehaviorPanel,
    /// The provider status-bar surface.
    ProviderStatusBar,
    /// The headless / CLI mappings surface.
    HeadlessCliMappings,
    /// The support mapping-export surface.
    SupportMappingExport,
}

impl M5MappingSyncConsumerSurface {
    /// Every claimed provider-surface consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MappingPickerPanel,
        Self::SyncBehaviorPanel,
        Self::ProviderStatusBar,
        Self::HeadlessCliMappings,
        Self::SupportMappingExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MappingPickerPanel => "mapping_picker_panel",
            Self::SyncBehaviorPanel => "sync_behavior_panel",
            Self::ProviderStatusBar => "provider_status_bar",
            Self::HeadlessCliMappings => "headless_cli_mappings",
            Self::SupportMappingExport => "support_mapping_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MappingPickerPanel => "Mapping Picker Panel",
            Self::SyncBehaviorPanel => "Sync-Behavior Panel",
            Self::ProviderStatusBar => "Provider Status Bar",
            Self::HeadlessCliMappings => "Headless / CLI Mappings",
            Self::SupportMappingExport => "Support Mapping Export",
        }
    }
}

// ---- project/board mapping row vocabulary --------------------------------

/// Controlled mapping scope — where the mapping the row shows came from, grouped into the
/// inherited / local / policy / unmapped scopes the task calls out, so a user can tell at a
/// glance whether a mapping is their own local choice, an inherited default, an admin-pinned
/// policy, or not yet mapped at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingScopeClass {
    /// The mapping is an inherited default (a default, an auto-match, or an imported config).
    InheritedScope,
    /// The mapping is the user's own explicit local choice.
    LocalScope,
    /// The mapping is pinned by an admin policy.
    PolicyScope,
    /// The row has no mapping scope yet.
    UnmappedScope,
}

impl M5MappingScopeClass {
    /// Every mapping scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InheritedScope,
        Self::LocalScope,
        Self::PolicyScope,
        Self::UnmappedScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InheritedScope => "inherited_scope",
            Self::LocalScope => "local_scope",
            Self::PolicyScope => "policy_scope",
            Self::UnmappedScope => "unmapped_scope",
        }
    }

    /// True when the scope is a policy pin, so the mapping is locked and change is blocked.
    pub const fn is_policy_locked(self) -> bool {
        matches!(self, Self::PolicyScope)
    }
}

/// The derived posture of a project/board mapping row — the resolver's verdict about the
/// mapping's origin. Derived one-to-one from the frozen mapping origin class, so the six
/// governed origins are never collapsed into one generic "mapped" chip and an explicit user
/// choice never reads the same as an auto-match, an inherited default, a policy pin, or an
/// unmapped row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingRowPosture {
    /// The user chose the mapping explicitly.
    ExplicitUserChoiceRow,
    /// The mapping was inherited from a default.
    InheritedDefaultRow,
    /// The mapping was auto-matched by heuristics.
    AutoMatchedRow,
    /// The mapping was imported from external config.
    ImportedConfigRow,
    /// The mapping is pinned by policy and locked.
    PolicyPinnedRow,
    /// The row has no mapping yet.
    UnmappedRow,
}

impl M5MappingRowPosture {
    /// Every mapping-row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExplicitUserChoiceRow,
        Self::InheritedDefaultRow,
        Self::AutoMatchedRow,
        Self::ImportedConfigRow,
        Self::PolicyPinnedRow,
        Self::UnmappedRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserChoiceRow => "explicit_user_choice_row",
            Self::InheritedDefaultRow => "inherited_default_row",
            Self::AutoMatchedRow => "auto_matched_row",
            Self::ImportedConfigRow => "imported_config_row",
            Self::PolicyPinnedRow => "policy_pinned_row",
            Self::UnmappedRow => "unmapped_row",
        }
    }

    /// True when the row points at an explicit destination rather than flagging itself
    /// unmapped.
    pub const fn shows_explicit_destination(self) -> bool {
        !matches!(self, Self::UnmappedRow)
    }

    /// True when the row's mapping is pinned by policy and therefore locked against change.
    pub const fn is_policy_locked(self) -> bool {
        matches!(self, Self::PolicyPinnedRow)
    }
}

/// One bounded action a project/board mapping row offers, so a row never hides its reveal /
/// change / reset / export affordances and a user can retarget or reset a mapping without
/// leaving the row — while a policy-pinned mapping is never silently changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingRowAction {
    /// Reveal the mapping's target kind, origin, scope, relation, and lock note.
    RevealMapping,
    /// Change the mapping to a different provider project / board.
    ChangeMapping,
    /// Reset the mapping back to its inherited default.
    ResetMapping,
    /// Export the mapping row as provider evidence.
    ExportRow,
}

impl M5MappingRowAction {
    /// Every mapping-row action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealMapping,
        Self::ChangeMapping,
        Self::ResetMapping,
        Self::ExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealMapping => "reveal_mapping",
            Self::ChangeMapping => "change_mapping",
            Self::ResetMapping => "reset_mapping",
            Self::ExportRow => "export_row",
        }
    }
}

/// Controlled mapping-row anatomy part the shared row surfaces. The parts in
/// [`M5MappingRowAnatomyPart::MANDATORY`] are required on every row so the provider project,
/// mapping origin, destination target, and mapping action cue are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingRowAnatomyPart {
    /// The provider project / space label cue.
    ProviderProjectCue,
    /// The repo / workspace relation cue.
    RepoWorkspaceRelationCue,
    /// The mapping origin cue.
    MappingOriginCue,
    /// The mapping scope cue.
    MappingScopeCue,
    /// The lock-note cue.
    LockNoteCue,
    /// The destination target-kind cue.
    DestinationTargetCue,
    /// The change / reset action cue.
    MappingActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5MappingRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderProjectCue,
        Self::RepoWorkspaceRelationCue,
        Self::MappingOriginCue,
        Self::MappingScopeCue,
        Self::LockNoteCue,
        Self::DestinationTargetCue,
        Self::MappingActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every mapping row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ProviderProjectCue,
        Self::RepoWorkspaceRelationCue,
        Self::MappingOriginCue,
        Self::DestinationTargetCue,
        Self::MappingActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderProjectCue => "provider_project_cue",
            Self::RepoWorkspaceRelationCue => "repo_workspace_relation_cue",
            Self::MappingOriginCue => "mapping_origin_cue",
            Self::MappingScopeCue => "mapping_scope_cue",
            Self::LockNoteCue => "lock_note_cue",
            Self::DestinationTargetCue => "destination_target_cue",
            Self::MappingActionCue => "mapping_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the mapping-row export carries so mapping-row truth is reconstructable. The fields
/// in [`M5MappingRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingRowExportField {
    /// The mapping target kind.
    MappingTargetKind,
    /// The mapping origin.
    MappingOrigin,
    /// The mapping scope.
    MappingScope,
    /// The provider project / space label.
    ProviderProjectLabel,
    /// The repo / workspace relation.
    RepoWorkspaceRelation,
    /// The lock state / note.
    LockState,
    /// The derived mapping-row posture.
    RowPosture,
    /// The bounded available actions.
    AvailableActions,
}

impl M5MappingRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MappingTargetKind,
        Self::MappingOrigin,
        Self::MappingScope,
        Self::ProviderProjectLabel,
        Self::RepoWorkspaceRelation,
        Self::LockState,
        Self::RowPosture,
        Self::AvailableActions,
    ];

    /// The export fields every mapping row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::MappingTargetKind,
        Self::MappingOrigin,
        Self::MappingScope,
        Self::RowPosture,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MappingTargetKind => "mapping_target_kind",
            Self::MappingOrigin => "mapping_origin",
            Self::MappingScope => "mapping_scope",
            Self::ProviderProjectLabel => "provider_project_label",
            Self::RepoWorkspaceRelation => "repo_workspace_relation",
            Self::LockState => "lock_state",
            Self::RowPosture => "row_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- sync-behavior row vocabulary ----------------------------------------

/// The derived sync-behavior class — what a sync-behavior row *actually does*, so a provider
/// surface stops using one ambiguous `synced` label for materially different write and
/// mirroring behaviors. Derived from the frozen sync mode and effective write scope so a
/// read-only metadata mirror, a comment/link sync, a status-transition sync, an
/// offline-capture-only queue, and a paused sync are never confused for one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncBehaviorClass {
    /// Live, full two-way sync — reads and writes the full object.
    FullBidirectionalSync,
    /// Comment / link sync only — writes comments and links, not the object body.
    CommentLinkSync,
    /// Status-transition sync only — writes status transitions, not comments or the body.
    StatusTransitionSync,
    /// Read-only metadata mirror — reads live, never writes.
    ReadOnlyMetadata,
    /// Offline-capture-only — nothing syncs; changes are captured locally.
    OfflineCaptureOnly,
    /// Sync is paused — no read or write flows until resumed.
    SyncPaused,
}

impl M5SyncBehaviorClass {
    /// Every sync-behavior class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullBidirectionalSync,
        Self::CommentLinkSync,
        Self::StatusTransitionSync,
        Self::ReadOnlyMetadata,
        Self::OfflineCaptureOnly,
        Self::SyncPaused,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBidirectionalSync => "full_bidirectional_sync",
            Self::CommentLinkSync => "comment_link_sync",
            Self::StatusTransitionSync => "status_transition_sync",
            Self::ReadOnlyMetadata => "read_only_metadata",
            Self::OfflineCaptureOnly => "offline_capture_only",
            Self::SyncPaused => "sync_paused",
        }
    }

    /// True when the behavior writes live to the provider (full, comment/link, or
    /// status-transition).
    pub const fn can_write_live(self) -> bool {
        matches!(
            self,
            Self::FullBidirectionalSync | Self::CommentLinkSync | Self::StatusTransitionSync
        )
    }

    /// True when the behavior is a read-only metadata mirror — never writes.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::ReadOnlyMetadata)
    }

    /// True when the behavior is offline-capture-only — nothing syncs to the provider.
    pub const fn is_offline_capture_only(self) -> bool {
        matches!(self, Self::OfflineCaptureOnly)
    }
}

/// One bounded action a sync-behavior row offers, so a row never hides its reveal /
/// change-mode / view-queue / retry-publish / export affordances and a user can inspect or
/// recover queued local work without leaving the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncRowAction {
    /// Reveal the sync mode, behavior class, write scope, and queued-draft state.
    RevealSyncBehavior,
    /// Change the sync mode.
    ChangeSyncMode,
    /// View the local-draft queue behind this row.
    ViewLocalQueue,
    /// Retry a failed queued publish.
    RetryQueuedPublish,
    /// Export the sync-behavior row as provider evidence.
    ExportRow,
}

impl M5SyncRowAction {
    /// Every sync-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealSyncBehavior,
        Self::ChangeSyncMode,
        Self::ViewLocalQueue,
        Self::RetryQueuedPublish,
        Self::ExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealSyncBehavior => "reveal_sync_behavior",
            Self::ChangeSyncMode => "change_sync_mode",
            Self::ViewLocalQueue => "view_local_queue",
            Self::RetryQueuedPublish => "retry_queued_publish",
            Self::ExportRow => "export_row",
        }
    }
}

/// Controlled sync-row anatomy part the shared row surfaces. The parts in
/// [`M5SyncRowAnatomyPart::MANDATORY`] are required on every row so the sync mode, behavior
/// class, effective write scope, queued-draft state, and sync action cue are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncRowAnatomyPart {
    /// The sync-mode cue.
    SyncModeCue,
    /// The derived sync-behavior-class cue.
    SyncBehaviorClassCue,
    /// The effective write-scope cue.
    WriteScopeCue,
    /// The queued-draft-state cue.
    QueuedDraftStateCue,
    /// The local-draft-queue cue.
    LocalDraftQueueCue,
    /// The offline-capture cue.
    OfflineCaptureCue,
    /// The change-mode / retry action cue.
    SyncActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5SyncRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SyncModeCue,
        Self::SyncBehaviorClassCue,
        Self::WriteScopeCue,
        Self::QueuedDraftStateCue,
        Self::LocalDraftQueueCue,
        Self::OfflineCaptureCue,
        Self::SyncActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every sync row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::SyncModeCue,
        Self::SyncBehaviorClassCue,
        Self::WriteScopeCue,
        Self::QueuedDraftStateCue,
        Self::SyncActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncModeCue => "sync_mode_cue",
            Self::SyncBehaviorClassCue => "sync_behavior_class_cue",
            Self::WriteScopeCue => "write_scope_cue",
            Self::QueuedDraftStateCue => "queued_draft_state_cue",
            Self::LocalDraftQueueCue => "local_draft_queue_cue",
            Self::OfflineCaptureCue => "offline_capture_cue",
            Self::SyncActionCue => "sync_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the sync-row export carries so sync-behavior-row truth is reconstructable. The
/// fields in [`M5SyncRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncRowExportField {
    /// The sync mode.
    SyncMode,
    /// The derived sync-behavior class.
    SyncBehaviorClass,
    /// The effective write scope.
    WriteScope,
    /// The queued-draft state.
    QueuedDraftState,
    /// Whether Aureline can write live.
    CanWriteLive,
    /// Whether local work remains queued.
    HasPendingLocalWork,
    /// The bounded available actions.
    AvailableActions,
    /// Whether the row is offline-capture-only.
    OfflineCaptureOnly,
}

impl M5SyncRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SyncMode,
        Self::SyncBehaviorClass,
        Self::WriteScope,
        Self::QueuedDraftState,
        Self::CanWriteLive,
        Self::HasPendingLocalWork,
        Self::AvailableActions,
        Self::OfflineCaptureOnly,
    ];

    /// The export fields every sync row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::SyncMode,
        Self::SyncBehaviorClass,
        Self::WriteScope,
        Self::QueuedDraftState,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncMode => "sync_mode",
            Self::SyncBehaviorClass => "sync_behavior_class",
            Self::WriteScope => "write_scope",
            Self::QueuedDraftState => "queued_draft_state",
            Self::CanWriteLive => "can_write_live",
            Self::HasPendingLocalWork => "has_pending_local_work",
            Self::AvailableActions => "available_actions",
            Self::OfflineCaptureOnly => "offline_capture_only",
        }
    }
}

// ---- project/board mapping row resolver ----------------------------------

/// The full input to the project/board mapping-row resolver for one mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingRowResolutionInput {
    /// The mapping target kind.
    pub target_kind: M5MappingTargetKind,
    /// The mapping origin.
    pub mapping_origin: M5MappingOriginClass,
    /// The opaque provider project / space label (must be non-empty).
    pub provider_project_label: String,
    /// The opaque repo / workspace relation (must be non-empty).
    pub repo_workspace_relation: String,
    /// The lock note, present when the mapping is locked. Required when the mapping is
    /// policy-pinned.
    pub lock_note: Option<String>,
    /// The opaque stable mapping identity (must be non-empty).
    pub mapping_ref: String,
}

/// The resolved project/board mapping-row truth for one mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMappingRow {
    /// The mapping target kind.
    pub target_kind: M5MappingTargetKind,
    /// The mapping origin.
    pub mapping_origin: M5MappingOriginClass,
    /// The opaque provider project / space label, preserved exactly from the input.
    pub provider_project_label: String,
    /// The opaque repo / workspace relation, preserved exactly from the input.
    pub repo_workspace_relation: String,
    /// The lock note, preserved exactly from the input.
    pub lock_note: Option<String>,
    /// The opaque stable mapping identity, preserved exactly from the input.
    pub mapping_ref: String,
    /// The derived mapping scope.
    pub mapping_scope: M5MappingScopeClass,
    /// The derived mapping-row posture.
    pub row_posture: M5MappingRowPosture,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5MappingRowAction>,
    /// True when the row points at an explicit destination rather than flagging itself
    /// unmapped.
    pub shows_explicit_destination: bool,
    /// True when the mapping is policy-locked and change is blocked.
    pub is_policy_locked: bool,
    /// The mapping row never assumes a default publish destination silently. ALWAYS `false`.
    pub assumes_default_destination_silently: bool,
}

/// Errors returned by [`resolve_project_board_mapping_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MappingRowResolutionError {
    /// The provider project label was empty.
    EmptyProjectLabel,
    /// The repo / workspace relation was empty.
    EmptyRelation,
    /// The mapping ref was empty.
    EmptyMappingRef,
    /// A policy-pinned mapping did not carry the required lock note.
    MissingLockNoteForPolicyLock,
    /// A mapping descriptor carried forbidden material.
    ForbiddenMappingMaterial,
}

impl M5MappingRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyProjectLabel => "empty_project_label",
            Self::EmptyRelation => "empty_relation",
            Self::EmptyMappingRef => "empty_mapping_ref",
            Self::MissingLockNoteForPolicyLock => "missing_lock_note_for_policy_lock",
            Self::ForbiddenMappingMaterial => "forbidden_mapping_material",
        }
    }
}

impl fmt::Display for M5MappingRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mapping row resolution error: {}", self.as_str())
    }
}

impl Error for M5MappingRowResolutionError {}

/// Resolves one project/board mapping row from its declared mapping state.
///
/// The derived mapping scope groups the six frozen mapping origins into the inherited /
/// local / policy / unmapped scopes; the derived row posture is taken one-to-one from the
/// frozen mapping origin so the six governed origins never collapse into one generic "mapped"
/// chip. The row always offers reveal and export; it offers change unless the mapping is
/// policy-locked; and it offers reset back to the inherited default when the current mapping
/// is a user override, an auto-match, or an imported config that can be cleared. An unmapped
/// row is flagged as unmapped rather than resolving to a silent default destination.
pub fn resolve_project_board_mapping_row(
    input: &M5MappingRowResolutionInput,
) -> Result<M5ResolvedMappingRow, M5MappingRowResolutionError> {
    if input.provider_project_label.trim().is_empty() {
        return Err(M5MappingRowResolutionError::EmptyProjectLabel);
    }
    if input.repo_workspace_relation.trim().is_empty() {
        return Err(M5MappingRowResolutionError::EmptyRelation);
    }
    if input.mapping_ref.trim().is_empty() {
        return Err(M5MappingRowResolutionError::EmptyMappingRef);
    }
    if matches!(input.mapping_origin, M5MappingOriginClass::PolicyPinned)
        && input
            .lock_note
            .as_ref()
            .map(|note| note.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(M5MappingRowResolutionError::MissingLockNoteForPolicyLock);
    }
    if value_repr_is_forbidden(&input.provider_project_label)
        || value_repr_is_forbidden(&input.repo_workspace_relation)
        || value_repr_is_forbidden(&input.mapping_ref)
        || input
            .lock_note
            .as_ref()
            .is_some_and(|note| value_repr_is_forbidden(note))
    {
        return Err(M5MappingRowResolutionError::ForbiddenMappingMaterial);
    }

    let mapping_scope = derive_mapping_scope(input.mapping_origin);
    let row_posture = derive_mapping_posture(input.mapping_origin);
    let available_actions = derive_mapping_actions(input.mapping_origin);

    Ok(M5ResolvedMappingRow {
        target_kind: input.target_kind,
        mapping_origin: input.mapping_origin,
        provider_project_label: input.provider_project_label.clone(),
        repo_workspace_relation: input.repo_workspace_relation.clone(),
        lock_note: input.lock_note.clone(),
        mapping_ref: input.mapping_ref.clone(),
        mapping_scope,
        row_posture,
        available_actions,
        shows_explicit_destination: row_posture.shows_explicit_destination()
            && !matches!(input.target_kind, M5MappingTargetKind::UnmappedTarget),
        is_policy_locked: row_posture.is_policy_locked(),
        // The acceptance criterion: never assume a default publish destination silently.
        assumes_default_destination_silently: false,
    })
}

/// Groups the frozen mapping origin into the inherited / local / policy / unmapped scope.
fn derive_mapping_scope(origin: M5MappingOriginClass) -> M5MappingScopeClass {
    use M5MappingOriginClass as Origin;
    use M5MappingScopeClass as Scope;
    match origin {
        Origin::ExplicitUserChoice => Scope::LocalScope,
        Origin::InheritedDefault | Origin::AutoMatched | Origin::ImportedConfig => {
            Scope::InheritedScope
        }
        Origin::PolicyPinned => Scope::PolicyScope,
        Origin::UnmappedOrigin => Scope::UnmappedScope,
    }
}

/// Derives the mapping-row posture one-to-one from the frozen mapping origin, so no surface
/// collapses the six governed origins into a generic "mapped" chip.
fn derive_mapping_posture(origin: M5MappingOriginClass) -> M5MappingRowPosture {
    use M5MappingOriginClass as Origin;
    use M5MappingRowPosture as Posture;
    match origin {
        Origin::ExplicitUserChoice => Posture::ExplicitUserChoiceRow,
        Origin::InheritedDefault => Posture::InheritedDefaultRow,
        Origin::AutoMatched => Posture::AutoMatchedRow,
        Origin::ImportedConfig => Posture::ImportedConfigRow,
        Origin::PolicyPinned => Posture::PolicyPinnedRow,
        Origin::UnmappedOrigin => Posture::UnmappedRow,
    }
}

/// Derives the bounded mapping action set from the mapping origin.
///
/// Reveal and export are always offered; change is offered unless the mapping is
/// policy-locked; reset back to the inherited default is offered when the current mapping is a
/// user override, an auto-match, or an imported config that can be cleared.
fn derive_mapping_actions(origin: M5MappingOriginClass) -> Vec<M5MappingRowAction> {
    use M5MappingOriginClass as Origin;
    use M5MappingRowAction as Action;
    let mut actions = vec![Action::RevealMapping];
    let locked = matches!(origin, Origin::PolicyPinned);
    if !locked {
        actions.push(Action::ChangeMapping);
        if matches!(
            origin,
            Origin::ExplicitUserChoice | Origin::AutoMatched | Origin::ImportedConfig
        ) {
            actions.push(Action::ResetMapping);
        }
    }
    actions.push(Action::ExportRow);
    actions
}

// ---- sync-behavior row resolver ------------------------------------------

/// The full input to the sync-behavior-row resolver for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyncRowResolutionInput {
    /// The provider sync mode.
    pub sync_mode: M5ProviderSyncMode,
    /// The effective write scope.
    pub write_scope: M5ProviderWriteScope,
    /// The queued-draft state.
    pub queued_draft_state: M5QueuedDraftState,
    /// The opaque user-facing sync-row label (must be non-empty).
    pub sync_label: String,
    /// The opaque stable sync-row identity (must be non-empty).
    pub sync_ref: String,
}

/// The resolved sync-behavior-row truth for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSyncRow {
    /// The provider sync mode.
    pub sync_mode: M5ProviderSyncMode,
    /// The effective write scope.
    pub write_scope: M5ProviderWriteScope,
    /// The queued-draft state.
    pub queued_draft_state: M5QueuedDraftState,
    /// The opaque sync-row label, preserved exactly from the input.
    pub sync_label: String,
    /// The opaque stable sync-row identity, preserved exactly from the input.
    pub sync_ref: String,
    /// The derived sync-behavior class.
    pub behavior_class: M5SyncBehaviorClass,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5SyncRowAction>,
    /// True when Aureline can write live to the provider from this row.
    pub can_write_live: bool,
    /// True when the row is a read-only metadata mirror.
    pub is_read_only: bool,
    /// True when the row is offline-capture-only.
    pub is_offline_capture_only: bool,
    /// True when local work remains queued behind this row.
    pub has_pending_local_work: bool,
    /// The sync row never collapses the six sync modes into one generic `synced` label.
    /// ALWAYS `false`.
    pub collapses_into_generic_synced: bool,
    /// The sync row never hides its local-draft queue state. ALWAYS `false`.
    pub hides_local_draft_queue_state: bool,
}

/// Errors returned by [`resolve_sync_behavior_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SyncRowResolutionError {
    /// The sync-row label was empty.
    EmptySyncLabel,
    /// The sync-row ref was empty.
    EmptySyncRef,
    /// A sync descriptor carried forbidden material.
    ForbiddenSyncMaterial,
}

impl M5SyncRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySyncLabel => "empty_sync_label",
            Self::EmptySyncRef => "empty_sync_ref",
            Self::ForbiddenSyncMaterial => "forbidden_sync_material",
        }
    }
}

impl fmt::Display for M5SyncRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "sync row resolution error: {}", self.as_str())
    }
}

impl Error for M5SyncRowResolutionError {}

/// Resolves one sync-behavior row from its declared sync state.
///
/// The derived sync-behavior class is taken from the frozen sync mode and effective write
/// scope so a provider surface stops using one ambiguous `synced` label: an offline-only mode
/// is offline-capture-only, a paused sync is paused, a read-only mirror is read-only metadata,
/// and a live / manual / scheduled sync separates into a full bidirectional, a comment/link,
/// or a status-transition behavior by its write scope. The row always reveals its behavior
/// and exports; it always offers a change-mode affordance; it offers view-queue when local
/// work is pending and retry-publish when a prior publish failed. The queued-draft state is
/// always visible.
pub fn resolve_sync_behavior_row(
    input: &M5SyncRowResolutionInput,
) -> Result<M5ResolvedSyncRow, M5SyncRowResolutionError> {
    if input.sync_label.trim().is_empty() {
        return Err(M5SyncRowResolutionError::EmptySyncLabel);
    }
    if input.sync_ref.trim().is_empty() {
        return Err(M5SyncRowResolutionError::EmptySyncRef);
    }
    if value_repr_is_forbidden(&input.sync_label) || value_repr_is_forbidden(&input.sync_ref) {
        return Err(M5SyncRowResolutionError::ForbiddenSyncMaterial);
    }

    let behavior_class = derive_sync_behavior_class(input.sync_mode, input.write_scope);
    let has_pending_local_work = queued_draft_is_pending(input.queued_draft_state);
    let available_actions = derive_sync_actions(input.queued_draft_state, has_pending_local_work);

    Ok(M5ResolvedSyncRow {
        sync_mode: input.sync_mode,
        write_scope: input.write_scope,
        queued_draft_state: input.queued_draft_state,
        sync_label: input.sync_label.clone(),
        sync_ref: input.sync_ref.clone(),
        behavior_class,
        available_actions,
        can_write_live: behavior_class.can_write_live(),
        is_read_only: behavior_class.is_read_only(),
        is_offline_capture_only: behavior_class.is_offline_capture_only(),
        has_pending_local_work,
        // The acceptance criterion: never one generic `synced` label, never a hidden queue.
        collapses_into_generic_synced: false,
        hides_local_draft_queue_state: false,
    })
}

/// Derives the sync-behavior class from the frozen sync mode and effective write scope, so a
/// read-only mirror, a comment/link sync, a status-transition sync, an offline-capture-only
/// queue, and a paused sync are never confused for one another.
fn derive_sync_behavior_class(
    sync_mode: M5ProviderSyncMode,
    write_scope: M5ProviderWriteScope,
) -> M5SyncBehaviorClass {
    use M5ProviderSyncMode as Mode;
    use M5ProviderWriteScope as Scope;
    use M5SyncBehaviorClass as Behavior;
    match sync_mode {
        // Explicit offline / paused / read-only mirror modes are their own behavior class.
        Mode::OfflineOnly => Behavior::OfflineCaptureOnly,
        Mode::PausedSync => Behavior::SyncPaused,
        Mode::ReadOnlyMirror => Behavior::ReadOnlyMetadata,
        // A live, manual, or scheduled sync separates by its effective write scope.
        Mode::LiveBidirectional | Mode::ManualPush | Mode::ScheduledSync => match write_scope {
            Scope::FullWrite => Behavior::FullBidirectionalSync,
            Scope::CommentOnly => Behavior::CommentLinkSync,
            Scope::StatusOnly => Behavior::StatusTransitionSync,
            Scope::ReadOnly | Scope::NoWrite | Scope::ScopeUnknown => Behavior::ReadOnlyMetadata,
        },
    }
}

/// True when the queued-draft state holds unpublished local work.
fn queued_draft_is_pending(state: M5QueuedDraftState) -> bool {
    use M5QueuedDraftState as Draft;
    matches!(
        state,
        Draft::DraftPending | Draft::QueuedPublish | Draft::PublishBlocked | Draft::PublishFailed
    )
}

/// Derives the bounded sync action set from the queued-draft state.
///
/// Reveal, change-mode, and export are always offered; view-queue is offered when local work
/// is pending; retry-publish is offered when a prior publish failed.
fn derive_sync_actions(
    state: M5QueuedDraftState,
    has_pending_local_work: bool,
) -> Vec<M5SyncRowAction> {
    use M5SyncRowAction as Action;
    let mut actions = vec![Action::RevealSyncBehavior, Action::ChangeSyncMode];
    if has_pending_local_work {
        actions.push(Action::ViewLocalQueue);
    }
    if matches!(state, M5QueuedDraftState::PublishFailed) {
        actions.push(Action::RetryQueuedPublish);
    }
    actions.push(Action::ExportRow);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked project/board mapping-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingRowResolutionCase {
    /// The resolver input.
    pub input: M5MappingRowResolutionInput,
    /// The resolved truth. Must equal `resolve_project_board_mapping_row(&input)`.
    pub resolved: M5ResolvedMappingRow,
}

impl M5MappingRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MappingRowResolutionInput) -> Self {
        let resolved =
            resolve_project_board_mapping_row(&input).expect("seed mapping row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_project_board_mapping_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved mapping identity preserves the input identity exactly.
    pub fn preserves_mapping_identity(&self) -> bool {
        self.resolved.mapping_ref == self.input.mapping_ref
            && self.resolved.provider_project_label == self.input.provider_project_label
            && self.resolved.repo_workspace_relation == self.input.repo_workspace_relation
    }

    /// True when the case never assumes a default destination silently.
    pub fn never_assumes_default(&self) -> bool {
        !self.resolved.assumes_default_destination_silently
    }
}

/// One worked sync-behavior-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyncRowResolutionCase {
    /// The resolver input.
    pub input: M5SyncRowResolutionInput,
    /// The resolved truth. Must equal `resolve_sync_behavior_row(&input)`.
    pub resolved: M5ResolvedSyncRow,
}

impl M5SyncRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SyncRowResolutionInput) -> Self {
        let resolved = resolve_sync_behavior_row(&input).expect("seed sync row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_sync_behavior_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved sync identity preserves the input identity exactly.
    pub fn preserves_sync_identity(&self) -> bool {
        self.resolved.sync_ref == self.input.sync_ref
            && self.resolved.sync_label == self.input.sync_label
    }

    /// True when the case never collapses into a generic `synced` label and never hides its
    /// queue.
    pub fn distinguishes_behavior_and_shows_queue(&self) -> bool {
        !self.resolved.collapses_into_generic_synced && !self.resolved.hides_local_draft_queue_state
    }
}

/// One row in the primitive matrix: one provider-surface consumer bound to the shared
/// mapping-row and sync-row anatomy, the mapping origins, target kinds, scopes, sync modes,
/// write scopes, behavior classes, queued-draft states, bounded actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncConsumerRow {
    /// Provider-surface consumer family.
    pub consumer_surface: M5MappingSyncConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this row.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Mapping-row anatomy parts this row renders (must include the mandatory parts).
    pub mapping_anatomy_parts: Vec<M5MappingRowAnatomyPart>,
    /// Sync-row anatomy parts this row renders (must include the mandatory parts).
    pub sync_anatomy_parts: Vec<M5SyncRowAnatomyPart>,
    /// Mapping target kinds this consumer distinguishes.
    pub mapping_target_kinds: Vec<M5MappingTargetKind>,
    /// Mapping origins this consumer distinguishes.
    pub mapping_origins: Vec<M5MappingOriginClass>,
    /// Mapping scopes this consumer distinguishes.
    pub mapping_scopes: Vec<M5MappingScopeClass>,
    /// Mapping-row postures this consumer distinguishes.
    pub mapping_row_postures: Vec<M5MappingRowPosture>,
    /// Bounded mapping-row actions this consumer offers.
    pub mapping_row_actions: Vec<M5MappingRowAction>,
    /// Sync modes this consumer distinguishes.
    pub sync_modes: Vec<M5ProviderSyncMode>,
    /// Write scopes this consumer distinguishes.
    pub write_scopes: Vec<M5ProviderWriteScope>,
    /// Sync-behavior classes this consumer distinguishes.
    pub sync_behavior_classes: Vec<M5SyncBehaviorClass>,
    /// Queued-draft states this consumer distinguishes.
    pub queued_draft_states: Vec<M5QueuedDraftState>,
    /// Bounded sync-row actions this consumer offers.
    pub sync_row_actions: Vec<M5SyncRowAction>,
    /// Mapping-row export fields this row carries (must include the mandatory fields).
    pub mapping_export_fields: Vec<M5MappingRowExportField>,
    /// Sync-row export fields this row carries (must include the mandatory fields).
    pub sync_export_fields: Vec<M5SyncRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked mapping-row resolutions proving the mapping resolver on this consumer.
    pub mapping_examples: Vec<M5MappingRowResolutionCase>,
    /// Worked sync-row resolutions proving the sync resolver on this consumer.
    pub sync_examples: Vec<M5SyncRowResolutionCase>,
    /// Hard invariant: this consumer never assumes a default destination silently. MUST be
    /// `false`.
    pub assumes_default_destination_silently: bool,
    /// Hard invariant: this consumer never masks its mapping origin or lock. MUST be `false`.
    pub masks_mapping_origin_or_lock: bool,
    /// Hard invariant: this consumer never collapses its sync behavior into one generic
    /// `synced` label. MUST be `false`.
    pub collapses_sync_into_generic_synced: bool,
    /// Hard invariant: this consumer never hides its local-draft queue state. MUST be
    /// `false`.
    pub hides_local_draft_queue_state: bool,
}

impl M5MappingSyncConsumerRow {
    /// True when the row declares every mandatory mapping anatomy part.
    fn declares_mandatory_mapping_anatomy(&self) -> bool {
        let present: BTreeSet<M5MappingRowAnatomyPart> =
            self.mapping_anatomy_parts.iter().copied().collect();
        M5MappingRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory sync anatomy part.
    fn declares_mandatory_sync_anatomy(&self) -> bool {
        let present: BTreeSet<M5SyncRowAnatomyPart> =
            self.sync_anatomy_parts.iter().copied().collect();
        M5SyncRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory mapping export field.
    fn declares_mandatory_mapping_export(&self) -> bool {
        let present: BTreeSet<M5MappingRowExportField> =
            self.mapping_export_fields.iter().copied().collect();
        M5MappingRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory sync export field.
    fn declares_mandatory_sync_export(&self) -> bool {
        let present: BTreeSet<M5SyncRowExportField> =
            self.sync_export_fields.iter().copied().collect();
        M5SyncRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.assumes_default_destination_silently
            && !self.masks_mapping_origin_or_lock
            && !self.collapses_sync_into_generic_synced
            && !self.hides_local_draft_queue_state
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncRowVocabularySet {
    /// Provider-surface-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Mapping-scope tokens.
    pub mapping_scopes: Vec<String>,
    /// Mapping-row-posture tokens.
    pub mapping_row_postures: Vec<String>,
    /// Mapping-row-action tokens.
    pub mapping_row_actions: Vec<String>,
    /// Mapping-anatomy-part tokens.
    pub mapping_anatomy_parts: Vec<String>,
    /// Mapping-export-field tokens.
    pub mapping_export_fields: Vec<String>,
    /// Sync-behavior-class tokens.
    pub sync_behavior_classes: Vec<String>,
    /// Sync-row-action tokens.
    pub sync_row_actions: Vec<String>,
    /// Sync-anatomy-part tokens.
    pub sync_anatomy_parts: Vec<String>,
    /// Sync-export-field tokens.
    pub sync_export_fields: Vec<String>,
    /// Mapping-origin-class tokens (reused from the frozen matrix).
    pub mapping_origins: Vec<String>,
    /// Mapping-target-kind tokens (reused from the frozen matrix).
    pub mapping_target_kinds: Vec<String>,
    /// Sync-mode tokens (reused from the frozen matrix).
    pub sync_modes: Vec<String>,
    /// Write-scope tokens (reused from the frozen matrix).
    pub write_scopes: Vec<String>,
    /// Queued-draft-state tokens (reused from the frozen matrix).
    pub queued_draft_states: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5MappingSyncRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5MappingSyncConsumerSurface::ALL, |v| v.as_str()),
            mapping_scopes: tokens(&M5MappingScopeClass::ALL, |v| v.as_str()),
            mapping_row_postures: tokens(&M5MappingRowPosture::ALL, |v| v.as_str()),
            mapping_row_actions: tokens(&M5MappingRowAction::ALL, |v| v.as_str()),
            mapping_anatomy_parts: tokens(&M5MappingRowAnatomyPart::ALL, |v| v.as_str()),
            mapping_export_fields: tokens(&M5MappingRowExportField::ALL, |v| v.as_str()),
            sync_behavior_classes: tokens(&M5SyncBehaviorClass::ALL, |v| v.as_str()),
            sync_row_actions: tokens(&M5SyncRowAction::ALL, |v| v.as_str()),
            sync_anatomy_parts: tokens(&M5SyncRowAnatomyPart::ALL, |v| v.as_str()),
            sync_export_fields: tokens(&M5SyncRowExportField::ALL, |v| v.as_str()),
            mapping_origins: tokens(&M5MappingOriginClass::ALL, |v| v.as_str()),
            mapping_target_kinds: tokens(&M5MappingTargetKind::ALL, |v| v.as_str()),
            sync_modes: tokens(&M5ProviderSyncMode::ALL, |v| v.as_str()),
            write_scopes: tokens(&M5ProviderWriteScope::ALL, |v| v.as_str()),
            queued_draft_states: tokens(&M5QueuedDraftState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ProviderSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ProviderDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ProviderAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncRowGovernanceReview {
    /// The mapping row shows its provider project / space label.
    pub mapping_row_shows_provider_project: bool,
    /// The mapping row shows its repo / workspace relation.
    pub mapping_row_shows_repo_workspace_relation: bool,
    /// The mapping row shows its mapping origin and scope.
    pub mapping_row_shows_mapping_origin_and_scope: bool,
    /// The mapping row shows its lock note when locked.
    pub mapping_row_shows_lock_note: bool,
    /// The mapping row offers change and reset actions.
    pub mapping_row_offers_change_and_reset: bool,
    /// The mapping row never assumes a default destination silently.
    pub mapping_never_assumes_default_destination: bool,
    /// The sync row separates read-only, comment/link, status-transition, and
    /// offline-capture modes.
    pub sync_row_separates_read_comment_status_offline: bool,
    /// The sync row shows its local-draft queue state.
    pub sync_row_shows_local_draft_queue_state: bool,
    /// The sync row never uses one generic `synced` label.
    pub sync_never_uses_one_generic_synced_label: bool,
    /// Rows keep the same truth across every deployment line.
    pub rows_stable_across_deployment_lines: bool,
    /// Rows keep the same truth across desktop, headless/export, and support consumers.
    pub rows_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs mapping and sync truth.
    pub support_export_reconstructs_mapping_and_sync_truth: bool,
    /// Later M5 rows cannot invent parallel mapping or sync vocabulary.
    pub later_rows_cannot_invent_parallel_mapping_or_sync_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncRowConsumerProjection {
    /// Provider surfaces consume the shared mapping/sync vocabulary.
    pub provider_surfaces_consume_mapping_sync_vocabulary: bool,
    /// The mapping-posture resolver reads a single canonical source.
    pub mapping_posture_reads_single_source: bool,
    /// The sync-behavior derivation reads a single canonical source.
    pub sync_behavior_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop rows read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the mapping/sync rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MappingSyncRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provider mapping/sync audit.
    pub provider_mapping_sync_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderMappingSyncRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderMappingSyncRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5MappingSyncConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MappingSyncRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MappingSyncRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MappingSyncRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MappingSyncRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MappingSyncRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provider mapping/sync-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderMappingSyncRowPacket {
    /// Record kind; must equal [`M5_PROVIDER_MAPPING_SYNC_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5MappingSyncConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MappingSyncRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MappingSyncRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MappingSyncRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MappingSyncRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MappingSyncRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderMappingSyncRowPacket {
    /// Builds an M5 mapping/sync-row-primitive packet from stable-lane input.
    pub fn new(input: M5ProviderMappingSyncRowPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_MAPPING_SYNC_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 mapping/sync-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5ProviderMappingSyncRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_MAPPING_SYNC_ROW_RECORD_KIND {
            violations.push(M5ProviderMappingSyncRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_VERSION {
            violations.push(M5ProviderMappingSyncRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderMappingSyncRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_mapping_origin_coverage(self, &mut violations);
        validate_sync_mode_coverage(self, &mut violations);
        validate_sync_behavior_separation(self, &mut violations);
        validate_mapping_action_coverage(self, &mut violations);
        validate_destination_explicitness(self, &mut violations);
        validate_queued_draft_visibility(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 mapping/sync row primitive packet serializes"),
        ) {
            violations.push(M5ProviderMappingSyncRowViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 mapping/sync row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per provider-surface consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,mapping_origins,mapping_scopes,mapping_actions,sync_modes,sync_behavior_classes,queued_draft_states,mapping_examples,sync_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.mapping_origins, |v| v.as_str()),
                join_tokens(&row.mapping_scopes, |v| v.as_str()),
                join_tokens(&row.mapping_row_actions, |v| v.as_str()),
                join_tokens(&row.sync_modes, |v| v.as_str()),
                join_tokens(&row.sync_behavior_classes, |v| v.as_str()),
                join_tokens(&row.queued_draft_states, |v| v.as_str()),
                row.mapping_examples.len(),
                row.sync_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Provider Mapping / Sync-Behavior Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provider-surface consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Mapping scopes: {}\n",
            self.vocabulary_set.mapping_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Sync-behavior classes: {}\n",
            self.vocabulary_set.sync_behavior_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Sync modes: {}\n",
            self.vocabulary_set.sync_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Provider-surface consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str("  - Mapping rows:\n");
            for case in &row.mapping_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (scope `{}`, explicit `{}`, locked `{}`)\n",
                    case.resolved.mapping_ref,
                    case.resolved.mapping_origin.as_str(),
                    case.resolved.target_kind.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.mapping_scope.as_str(),
                    case.resolved.shows_explicit_destination,
                    case.resolved.is_policy_locked,
                ));
            }
            out.push_str("  - Sync rows:\n");
            for case in &row.sync_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (write-live `{}`, pending `{}`, draft `{}`)\n",
                    case.resolved.sync_ref,
                    case.resolved.sync_mode.as_str(),
                    case.resolved.write_scope.as_str(),
                    case.resolved.behavior_class.as_str(),
                    case.resolved.can_write_live,
                    case.resolved.has_pending_local_work,
                    case.resolved.queued_draft_state.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 mapping/sync-row-primitive export.
#[derive(Debug)]
pub enum M5ProviderMappingSyncRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderMappingSyncRowViolation>),
}

impl fmt::Display for M5ProviderMappingSyncRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 mapping/sync row primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 mapping/sync row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderMappingSyncRowArtifactError {}

/// Validation failures emitted by [`M5ProviderMappingSyncRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderMappingSyncRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required provider-surface consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A provider-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory mapping anatomy parts.
    MandatoryMappingAnatomyMissing,
    /// A row omits one of the mandatory sync anatomy parts.
    MandatorySyncAnatomyMissing,
    /// A row omits one of the mandatory mapping export fields.
    MandatoryMappingExportMissing,
    /// A row omits one of the mandatory sync export fields.
    MandatorySyncExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked mapping resolutions.
    MappingExampleMissing,
    /// A row declares no worked sync resolutions.
    SyncExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every mapping origin.
    MappingOriginCoverageUnproven,
    /// The worked resolutions do not exercise every sync mode.
    SyncModeCoverageUnproven,
    /// The worked resolutions do not prove the read-only, comment/link, status-transition, and
    /// offline-capture separations.
    SyncBehaviorSeparationUnproven,
    /// The worked resolutions do not prove change, reset, and a policy-locked-blocks-change
    /// mapping.
    MappingActionCoverageUnproven,
    /// The worked resolutions do not prove both an explicit destination and an unmapped row.
    DestinationExplicitnessUnproven,
    /// The worked resolutions do not prove both a pending and a cleared local-draft queue
    /// state.
    QueuedDraftVisibilityUnproven,
    /// A worked resolution does not preserve its exact mapping / sync identity.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ProviderMappingSyncRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryMappingAnatomyMissing => "mandatory_mapping_anatomy_missing",
            Self::MandatorySyncAnatomyMissing => "mandatory_sync_anatomy_missing",
            Self::MandatoryMappingExportMissing => "mandatory_mapping_export_missing",
            Self::MandatorySyncExportMissing => "mandatory_sync_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::MappingExampleMissing => "mapping_example_missing",
            Self::SyncExampleMissing => "sync_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::MappingOriginCoverageUnproven => "mapping_origin_coverage_unproven",
            Self::SyncModeCoverageUnproven => "sync_mode_coverage_unproven",
            Self::SyncBehaviorSeparationUnproven => "sync_behavior_separation_unproven",
            Self::MappingActionCoverageUnproven => "mapping_action_coverage_unproven",
            Self::DestinationExplicitnessUnproven => "destination_explicitness_unproven",
            Self::QueuedDraftVisibilityUnproven => "queued_draft_visibility_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 mapping/sync-row-primitive export.
pub fn current_stable_m5_provider_mapping_sync_row_export(
) -> Result<M5ProviderMappingSyncRowPacket, M5ProviderMappingSyncRowArtifactError> {
    let packet: M5ProviderMappingSyncRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/support_export.json"
    )))
    .map_err(M5ProviderMappingSyncRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderMappingSyncRowArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_DOC_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_TARGET_MAPPING_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_SYNC_HEALTH_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderMappingSyncRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderMappingSyncRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let present: BTreeSet<M5MappingSyncConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5MappingSyncConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderMappingSyncRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.mapping_anatomy_parts.is_empty()
            || row.sync_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.mapping_target_kinds.is_empty()
            || row.mapping_origins.is_empty()
            || row.mapping_scopes.is_empty()
            || row.mapping_row_postures.is_empty()
            || row.mapping_row_actions.is_empty()
            || row.sync_modes.is_empty()
            || row.write_scopes.is_empty()
            || row.sync_behavior_classes.is_empty()
            || row.queued_draft_states.is_empty()
            || row.sync_row_actions.is_empty()
            || row.mapping_export_fields.is_empty()
            || row.sync_export_fields.is_empty()
        {
            violations.push(M5ProviderMappingSyncRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_mapping_anatomy() {
            violations.push(M5ProviderMappingSyncRowViolation::MandatoryMappingAnatomyMissing);
        }
        if !row.declares_mandatory_sync_anatomy() {
            violations.push(M5ProviderMappingSyncRowViolation::MandatorySyncAnatomyMissing);
        }
        if !row.declares_mandatory_mapping_export() {
            violations.push(M5ProviderMappingSyncRowViolation::MandatoryMappingExportMissing);
        }
        if !row.declares_mandatory_sync_export() {
            violations.push(M5ProviderMappingSyncRowViolation::MandatorySyncExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProviderMappingSyncRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderMappingSyncRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderMappingSyncRowViolation::DowngradeTriggersMissing);
        }
        if row.mapping_examples.is_empty() {
            violations.push(M5ProviderMappingSyncRowViolation::MappingExampleMissing);
        }
        if row.sync_examples.is_empty() {
            violations.push(M5ProviderMappingSyncRowViolation::SyncExampleMissing);
        }
        if row
            .mapping_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .sync_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ProviderMappingSyncRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProviderMappingSyncRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProviderMappingSyncRowViolation::RowInvariantViolated);
        }
    }
}

/// Every mapping origin must be exercised by some worked mapping resolution — the
/// implementation requirement that mapping rows show their origin without collapsing the six
/// origins into one generic mapped state.
fn validate_mapping_origin_coverage(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let exercised: BTreeSet<M5MappingOriginClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.mapping_examples.iter())
        .map(|case| case.resolved.mapping_origin)
        .collect();
    let covered = M5MappingOriginClass::ALL
        .iter()
        .all(|origin| exercised.contains(origin));
    if !covered {
        violations.push(M5ProviderMappingSyncRowViolation::MappingOriginCoverageUnproven);
    }
}

/// Every sync mode must be exercised by some worked sync resolution.
fn validate_sync_mode_coverage(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let exercised: BTreeSet<M5ProviderSyncMode> = packet
        .rows
        .iter()
        .flat_map(|row| row.sync_examples.iter())
        .map(|case| case.resolved.sync_mode)
        .collect();
    let covered = M5ProviderSyncMode::ALL
        .iter()
        .all(|mode| exercised.contains(mode));
    if !covered {
        violations.push(M5ProviderMappingSyncRowViolation::SyncModeCoverageUnproven);
    }
}

/// At least one worked sync resolution must prove each of the read-only-metadata,
/// comment/link, status-transition, and offline-capture-only separations — the
/// acceptance-criterion that a provider surface stops using one ambiguous `synced` label for
/// materially different write and mirroring behaviors.
fn validate_sync_behavior_separation(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let exercised: BTreeSet<M5SyncBehaviorClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.sync_examples.iter())
        .map(|case| case.resolved.behavior_class)
        .collect();
    let required = [
        M5SyncBehaviorClass::ReadOnlyMetadata,
        M5SyncBehaviorClass::CommentLinkSync,
        M5SyncBehaviorClass::StatusTransitionSync,
        M5SyncBehaviorClass::OfflineCaptureOnly,
    ];
    if !required.iter().all(|class| exercised.contains(class)) {
        violations.push(M5ProviderMappingSyncRowViolation::SyncBehaviorSeparationUnproven);
    }
}

/// At least one worked mapping resolution must prove change offered, one reset offered, and
/// one a policy-locked mapping that blocks change — the implementation requirement that a user
/// can change or reset a mapping without leaving the row while a policy pin stays locked.
fn validate_mapping_action_coverage(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.mapping_examples.iter())
    };
    let has_change = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5MappingRowAction::ChangeMapping)
    });
    let has_reset = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5MappingRowAction::ResetMapping)
    });
    let has_locked = cases().any(|case| {
        case.resolved.is_policy_locked
            && !case
                .resolved
                .available_actions
                .contains(&M5MappingRowAction::ChangeMapping)
    });
    if !(has_change && has_reset && has_locked) {
        violations.push(M5ProviderMappingSyncRowViolation::MappingActionCoverageUnproven);
    }
}

/// At least one worked mapping resolution must show an explicit destination and one must flag
/// itself unmapped — the acceptance-criterion that a user never has to guess which provider
/// project/board a lookup or write will target, and no default is ever assumed silently.
fn validate_destination_explicitness(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.mapping_examples.iter())
    };
    let has_explicit = cases().any(|case| case.resolved.shows_explicit_destination);
    let has_unmapped = cases().any(|case| !case.resolved.shows_explicit_destination);
    let never_defaults = cases().all(|case| case.never_assumes_default());
    if !(has_explicit && has_unmapped && never_defaults) {
        violations.push(M5ProviderMappingSyncRowViolation::DestinationExplicitnessUnproven);
    }
}

/// At least one worked sync resolution must show pending local work and one a state with no
/// pending work — the implementation requirement that the local-draft queue state is always
/// visible.
fn validate_queued_draft_visibility(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.sync_examples.iter());
    let has_pending = cases().any(|case| case.resolved.has_pending_local_work);
    let has_cleared = cases().any(|case| !case.resolved.has_pending_local_work);
    let shows_queue = cases().all(|case| case.distinguishes_behavior_and_shows_queue());
    if !(has_pending && has_cleared && shows_queue) {
        violations.push(M5ProviderMappingSyncRowViolation::QueuedDraftVisibilityUnproven);
    }
}

/// Every worked resolution must preserve its exact mapping / sync identity — the invariant
/// that neither row ever rewrites the user's provider project, relation, or identity.
fn validate_identity_preservation(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let mapping_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.mapping_examples.iter())
        .all(|case| case.preserves_mapping_identity());
    let sync_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.sync_examples.iter())
        .all(|case| case.preserves_sync_identity());
    if !(mapping_ok && sync_ok) {
        violations.push(M5ProviderMappingSyncRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.mapping_row_shows_provider_project,
        review.mapping_row_shows_repo_workspace_relation,
        review.mapping_row_shows_mapping_origin_and_scope,
        review.mapping_row_shows_lock_note,
        review.mapping_row_offers_change_and_reset,
        review.mapping_never_assumes_default_destination,
        review.sync_row_separates_read_comment_status_offline,
        review.sync_row_shows_local_draft_queue_state,
        review.sync_never_uses_one_generic_synced_label,
        review.rows_stable_across_deployment_lines,
        review.rows_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_mapping_and_sync_truth,
        review.later_rows_cannot_invent_parallel_mapping_or_sync_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProviderMappingSyncRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.provider_surfaces_consume_mapping_sync_vocabulary,
        projection.mapping_posture_reads_single_source,
        projection.sync_behavior_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ProviderMappingSyncRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderMappingSyncRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderMappingSyncRowPacket,
    violations: &mut Vec<M5ProviderMappingSyncRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.provider_mapping_sync_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderMappingSyncRowViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
