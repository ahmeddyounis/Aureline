//! Shared transfer and recovery integrity records for shell-owned surfaces.
//!
//! This module joins the existing editor clipboard, terminal paste, project
//! entry drag/drop, diff reopen, query reopen, terminal restore, and support
//! export vocabularies into one contract-shaped alpha packet. The records are
//! intentionally metadata-first: raw clipboard bodies, terminal payloads, file
//! contents, private paths, and secrets stay out of support/export packets.

use std::collections::{BTreeMap, BTreeSet};

use aureline_editor::clipboard::{
    CopyPayload, CopyVariantId, RepresentationClass as EditorRepresentationClass,
};
use aureline_review::{DiffClosedSessionRecord, DiffReopenProjection};
use aureline_search::SavedQueryReopenProjection;
use aureline_terminal::{
    evaluate_paste_review, HostClass, RestoredTerminalRecord, TerminalPastePolicyResult,
    TerminalPasteReviewInput, TerminalPasteSubmitBehavior,
};
use aureline_workspace::{
    AdmissionAction, AdmissionReviewPacket,
    TransferProgressClass as AdmissionTransferProgressClass, WriteScopeClass,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`TransferActionRecord`].
pub const TRANSFER_ACTION_RECORD_KIND: &str = "transfer_action_record";
/// Stable record-kind tag for [`InteractionIntegrityAlphaPacket`].
pub const INTERACTION_INTEGRITY_ALPHA_PACKET_RECORD_KIND: &str =
    "interaction_integrity_alpha_packet";
/// Stable record-kind tag for [`InteractionIntegrityValidationReport`].
pub const INTERACTION_INTEGRITY_VALIDATION_REPORT_RECORD_KIND: &str =
    "interaction_integrity_validation_report";
/// Schema version for the transfer-action event contract.
pub const TRANSFER_ACTION_SCHEMA_VERSION: u32 = 1;

/// Surface that emitted or consumed a transfer action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferSurfaceClass {
    /// Editor canvas, buffer, or tab.
    Editor,
    /// Diff or review surface.
    ReviewDiff,
    /// Search, query, or quick-open surface.
    Search,
    /// Terminal canvas, paste review, or restored transcript row.
    Terminal,
    /// Start Center, file association, CLI entry, or drag/drop entry surface.
    ProjectEntry,
    /// Support or diagnostic export surface.
    SupportExport,
}

impl TransferSurfaceClass {
    /// Returns the stable schema token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::ReviewDiff => "review_diff",
            Self::Search => "search",
            Self::Terminal => "terminal",
            Self::ProjectEntry => "project_entry",
            Self::SupportExport => "support_export",
        }
    }
}

/// User action family represented by a transfer record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferActionKind {
    /// Copy or export-adjacent clipboard transfer.
    Copy,
    /// Paste into an editor, terminal, query, or other target.
    Paste,
    /// Drag/drop, attach, import, open, move, copy, or split action.
    Drop,
    /// Reopen an intentionally closed surface.
    Reopen,
    /// Recover a surface after crash, disconnect, or degraded restart.
    Recover,
}

impl TransferActionKind {
    /// Returns the stable schema token for this action kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Paste => "paste",
            Self::Drop => "drop",
            Self::Reopen => "reopen",
            Self::Recover => "recover",
        }
    }
}

/// Material representation class of the payload or reopened state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferRepresentationClass {
    /// Plain text suitable for default clipboard writes.
    PlainText,
    /// Raw source or raw identifier that has passed safety review.
    RawSafe,
    /// Rendered representation rather than source identity.
    Rendered,
    /// Payload includes target, hunk, query, or provenance context.
    WithContext,
    /// Source representation with controls or metacharacters escaped.
    Escaped,
    /// Sanitized inert snapshot.
    Sanitized,
    /// Model-produced or generated content.
    Generated,
    /// Metadata envelope with the raw body withheld.
    MetadataOnly,
}

impl TransferRepresentationClass {
    /// Returns the stable schema token for this representation class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::RawSafe => "raw_safe",
            Self::Rendered => "rendered",
            Self::WithContext => "with_context",
            Self::Escaped => "escaped",
            Self::Sanitized => "sanitized",
            Self::Generated => "generated",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Boundary crossed or targeted by a transfer action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferBoundaryClass {
    /// Local editor or workspace authority.
    LocalWorkspace,
    /// Local system clipboard.
    LocalClipboard,
    /// Remote or managed host.
    RemoteHost,
    /// Local container or sandbox host.
    ContainerHost,
    /// Review, diff, or compare target.
    ReviewSurface,
    /// Query or search result target.
    SearchQuery,
    /// Project entry or admission target.
    ProjectEntry,
    /// Support or diagnostics export boundary.
    SupportExport,
}

impl TransferBoundaryClass {
    /// Returns the stable schema token for this boundary class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::LocalClipboard => "local_clipboard",
            Self::RemoteHost => "remote_host",
            Self::ContainerHost => "container_host",
            Self::ReviewSurface => "review_surface",
            Self::SearchQuery => "search_query",
            Self::ProjectEntry => "project_entry",
            Self::SupportExport => "support_export",
        }
    }
}

/// Clipboard route used by a copy or paste boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardRouteClass {
    /// Local desktop system clipboard.
    LocalSystemClipboard,
    /// Remote clipboard bridge.
    RemoteClipboardBridge,
    /// Editor named register.
    NamedRegister,
    /// Search/query register.
    SearchRegister,
    /// Clipboard route blocked by policy or trust.
    PolicyBlocked,
}

impl ClipboardRouteClass {
    /// Returns the stable schema token for this clipboard route.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSystemClipboard => "local_system_clipboard",
            Self::RemoteClipboardBridge => "remote_clipboard_bridge",
            Self::NamedRegister => "named_register",
            Self::SearchRegister => "search_register",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Undo or recovery class associated with a transfer action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferRecoveryClass {
    /// Exact undo is available if authority state still matches.
    ExactUndo,
    /// Revert requires compensation and may not silently auto-run.
    CompensatingRollback,
    /// Restore checkpoint is the declared recovery path.
    RestoreCheckpoint,
    /// Evidence can reopen but live effects never rerun.
    EvidenceOnlyNoRerun,
    /// No mutation happened, but an audit/support row exists.
    AuditOnlyNoMutation,
    /// No recovery is available.
    NoRecoveryAvailable,
}

impl TransferRecoveryClass {
    /// Returns the stable schema token for this recovery class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingRollback => "compensating_rollback",
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::EvidenceOnlyNoRerun => "evidence_only_no_rerun",
            Self::AuditOnlyNoMutation => "audit_only_no_mutation",
            Self::NoRecoveryAvailable => "no_recovery_available",
        }
    }
}

/// Visible drag/drop or transfer verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferVerb {
    /// Copy into the target scope.
    Copy,
    /// Move into the target scope.
    Move,
    /// Open the dropped item.
    Open,
    /// Import or extract the dropped artifact.
    Import,
    /// Split into another editor group or window.
    Split,
    /// Attach evidence or payload to a target surface.
    Attach,
    /// Paste into the target surface.
    Paste,
    /// Reopen a prior surface.
    Reopen,
    /// Recover a prior surface.
    Recover,
    /// Blocked before commit.
    Blocked,
}

impl TransferVerb {
    /// Returns the stable schema token for this verb.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Move => "move",
            Self::Open => "open",
            Self::Import => "import",
            Self::Split => "split",
            Self::Attach => "attach",
            Self::Paste => "paste",
            Self::Reopen => "reopen",
            Self::Recover => "recover",
            Self::Blocked => "blocked",
        }
    }
}

/// Progress disclosure required for large or durable transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferProgressClass {
    /// No visible progress is required.
    NotRequired,
    /// Inline progress is sufficient.
    InlineProgress,
    /// Durable progress with cancel is required.
    DurableProgressWithCancel,
}

impl TransferProgressClass {
    /// Returns the stable schema token for this progress class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::InlineProgress => "inline_progress",
            Self::DurableProgressWithCancel => "durable_progress_with_cancel",
        }
    }
}

/// Submit posture for paste-like paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferSubmitPosture {
    /// Insert only; do not submit automatically.
    NoAutoSubmit,
    /// Submit only after explicit review acceptance.
    SubmitAfterReview,
}

impl TransferSubmitPosture {
    /// Returns the stable schema token for this submit posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAutoSubmit => "no_auto_submit",
            Self::SubmitAfterReview => "submit_after_review",
        }
    }
}

/// Policy or trust gate result exposed before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferGateResult {
    /// Gate allows the action.
    Allowed,
    /// Gate requires a review surface.
    ReviewRequired,
    /// Gate blocks the action.
    Blocked,
}

impl TransferGateResult {
    /// Returns the stable schema token for this gate result.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ReviewRequired => "review_required",
            Self::Blocked => "blocked",
        }
    }
}

/// Source reason for reopen or recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenRecoverySourceClass {
    /// The user closed the surface intentionally and later reopened it.
    ClosedIntentionally,
    /// The surface was recovered after an abnormal termination.
    CrashRecovery,
    /// The surface was recovered after transport or runtime disconnect.
    DisconnectRecovery,
}

impl ReopenRecoverySourceClass {
    /// Returns the stable schema token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClosedIntentionally => "closed_intentionally",
            Self::CrashRecovery => "crash_recovery",
            Self::DisconnectRecovery => "disconnect_recovery",
        }
    }
}

/// High-frequency surface covered by reopen or recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenRecoverySurface {
    /// Editor buffer or editor group.
    Editor,
    /// Diff or compare surface.
    Diff,
    /// Search query, quick-open, or query result surface.
    QuerySearch,
    /// Terminal tab, transcript, or session row.
    Terminal,
}

impl ReopenRecoverySurface {
    /// Returns the stable schema token for this reopen surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::QuerySearch => "query_search",
            Self::Terminal => "terminal",
        }
    }
}

/// Scope of a named undo group or recovery checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoRecoveryGroupScope {
    /// Single editor buffer or single surface.
    SingleSurface,
    /// Paste or drop path that changes durable local state.
    PasteOrDropMutation,
    /// Multi-file or multi-step import/transfer.
    MultiFileTransferImport,
}

impl UndoRecoveryGroupScope {
    /// Returns the stable schema token for this group scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSurface => "single_surface",
            Self::PasteOrDropMutation => "paste_or_drop_mutation",
            Self::MultiFileTransferImport => "multi_file_transfer_import",
        }
    }
}

/// Target and boundary disclosed for a transfer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferTargetBoundary {
    /// Opaque target ref safe for support exports.
    pub target_ref: String,
    /// Redaction-aware target label.
    pub target_label: String,
    /// Boundary class crossed or targeted by the transfer.
    pub boundary_class: TransferBoundaryClass,
    /// Stable token for [`Self::boundary_class`].
    pub boundary_class_token: String,
    /// Boundary label shown before commit.
    pub boundary_label: String,
    /// Host label when the target is terminal or remote backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_label: Option<String>,
}

impl TransferTargetBoundary {
    /// Builds a target-boundary projection with stable tokens included.
    pub fn new(
        target_ref: impl Into<String>,
        target_label: impl Into<String>,
        boundary_class: TransferBoundaryClass,
        boundary_label: impl Into<String>,
        host_label: Option<String>,
    ) -> Self {
        Self {
            target_ref: target_ref.into(),
            target_label: target_label.into(),
            boundary_class,
            boundary_class_token: boundary_class.as_str().to_string(),
            boundary_label: boundary_label.into(),
            host_label,
        }
    }
}

/// Sensitive-copy label-first review posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitiveCopyReview {
    /// Sensitive classes detected in the pending transfer.
    pub sensitive_value_classes: Vec<String>,
    /// True when a preview is required before clipboard write.
    pub preview_required: bool,
    /// True when labels render before bytes leave the product.
    pub label_first_path: bool,
    /// Command or route that opens the preview.
    pub preview_action_id: String,
    /// True when clipboard write is deferred until review continues.
    pub clipboard_write_deferred_until_review: bool,
}

/// Named undo group or recovery checkpoint bound to a transfer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedUndoRecoveryGroup {
    /// Stable group or checkpoint ref.
    pub group_ref: String,
    /// Human-readable group name.
    pub display_name: String,
    /// Group scope class.
    pub group_scope: UndoRecoveryGroupScope,
    /// Stable token for [`Self::group_scope`].
    pub group_scope_token: String,
    /// Command id that created or owns the group.
    pub command_id: String,
    /// Recovery class exposed by the group.
    pub recovery_class: TransferRecoveryClass,
    /// Stable token for [`Self::recovery_class`].
    pub recovery_class_token: String,
    /// Mutation-journal ref when a durable mutation exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Checkpoint ref when recovery uses a checkpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
}

impl NamedUndoRecoveryGroup {
    /// Builds a named group with stable tokens included.
    pub fn new(
        group_ref: impl Into<String>,
        display_name: impl Into<String>,
        group_scope: UndoRecoveryGroupScope,
        command_id: impl Into<String>,
        recovery_class: TransferRecoveryClass,
        mutation_journal_ref: Option<String>,
        checkpoint_ref: Option<String>,
    ) -> Self {
        Self {
            group_ref: group_ref.into(),
            display_name: display_name.into(),
            group_scope,
            group_scope_token: group_scope.as_str().to_string(),
            command_id: command_id.into(),
            recovery_class,
            recovery_class_token: recovery_class.as_str().to_string(),
            mutation_journal_ref,
            checkpoint_ref,
        }
    }
}

/// Progress, cancellation, and completion-summary disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferProgressDisclosure {
    /// Progress class for the transfer.
    pub progress_class: TransferProgressClass,
    /// Stable token for [`Self::progress_class`].
    pub progress_class_token: String,
    /// Redaction-aware progress label.
    pub progress_label: String,
    /// True when cancellation is offered.
    pub cancel_available: bool,
    /// Cancel command or route id when cancellation is offered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_action_id: Option<String>,
    /// Completion or result summary route.
    pub summary_action_id: String,
}

impl TransferProgressDisclosure {
    /// Builds a progress disclosure with stable tokens included.
    pub fn new(
        progress_class: TransferProgressClass,
        progress_label: impl Into<String>,
        cancel_available: bool,
        cancel_action_id: Option<String>,
        summary_action_id: impl Into<String>,
    ) -> Self {
        Self {
            progress_class,
            progress_class_token: progress_class.as_str().to_string(),
            progress_label: progress_label.into(),
            cancel_available,
            cancel_action_id,
            summary_action_id: summary_action_id.into(),
        }
    }
}

/// High-risk terminal paste review projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPasteBoundaryReview {
    /// Stable terminal session ref.
    pub session_ref: String,
    /// Host label shown in the paste review.
    pub target_host_label: String,
    /// Clipboard route used by the paste.
    pub clipboard_route: ClipboardRouteClass,
    /// Stable token for [`Self::clipboard_route`].
    pub clipboard_route_token: String,
    /// Number of lines disclosed before paste.
    pub line_count: u32,
    /// Bulk hint shown before commit.
    pub bulk_hint: String,
    /// True when bracketed paste posture is shown.
    pub bracketed_paste_available: bool,
    /// Submit posture for the paste.
    pub submit_posture: TransferSubmitPosture,
    /// Stable token for [`Self::submit_posture`].
    pub submit_posture_token: String,
    /// Policy result shown before commit.
    pub policy_result: TransferGateResult,
    /// Stable token for [`Self::policy_result`].
    pub policy_result_token: String,
    /// Trust result shown before commit.
    pub trust_result: TransferGateResult,
    /// Stable token for [`Self::trust_result`].
    pub trust_result_token: String,
    /// Cancel command id.
    pub cancel_action_id: String,
    /// Continue command id.
    pub continue_action_id: String,
    /// True when review cannot be bypassed.
    pub commit_without_review_forbidden: bool,
    /// True when paste cannot auto-submit to the shell.
    pub auto_submit_forbidden: bool,
    /// Stable risk reason tokens from the terminal evaluator.
    pub risk_reason_tokens: Vec<String>,
}

/// Drag/drop verb and destination-scope disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragDropVerbDisclosure {
    /// Visible transfer verb.
    pub verb: TransferVerb,
    /// Stable token for [`Self::verb`].
    pub verb_token: String,
    /// Redaction-aware destination scope.
    pub destination_scope: String,
    /// True when the drop mutates broad workspace state.
    pub broad_workspace_mutation: bool,
    /// True when checkpoint is created or verified before commit.
    pub checkpoint_before_commit: bool,
    /// True when collision or overwrite review is part of the packet.
    pub collision_or_overwrite_review: bool,
    /// Modifier-key cue shown with the verb.
    pub modifier_cue: String,
    /// Target slot receiving the drop.
    pub target_slot_ref: String,
}

impl DragDropVerbDisclosure {
    /// Builds a drag/drop disclosure with stable tokens included.
    pub fn new(
        verb: TransferVerb,
        destination_scope: impl Into<String>,
        broad_workspace_mutation: bool,
        checkpoint_before_commit: bool,
        collision_or_overwrite_review: bool,
        modifier_cue: impl Into<String>,
        target_slot_ref: impl Into<String>,
    ) -> Self {
        Self {
            verb,
            verb_token: verb.as_str().to_string(),
            destination_scope: destination_scope.into(),
            broad_workspace_mutation,
            checkpoint_before_commit,
            collision_or_overwrite_review,
            modifier_cue: modifier_cue.into(),
            target_slot_ref: target_slot_ref.into(),
        }
    }
}

/// Reopen or recovery continuity state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenRecoveryDisclosure {
    /// Surface being reopened or recovered.
    pub surface: ReopenRecoverySurface,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Source class for the reopen/recovery.
    pub source_class: ReopenRecoverySourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_class_token: String,
    /// Closed, saved-query, restored-terminal, or recovered source ref.
    pub closed_or_recovered_ref: String,
    /// Reopen command id.
    pub reopen_command_id: String,
    /// Target identity restored by the command.
    pub restored_target_identity_ref: String,
    /// Cursor or hunk selection ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_selection_ref: Option<String>,
    /// Scroll anchor ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_scroll_anchor_ref: Option<String>,
    /// Last working-directory hint restored with a terminal row, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_working_directory: Option<String>,
    /// Shell identity restored with a terminal row, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_shell_identity: Option<String>,
    /// Environment-scope token restored with a terminal row, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_environment_scope_token: Option<String>,
    /// Last command class token restored with a terminal row, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_last_command_class_token: Option<String>,
    /// True only when live authority truly survived.
    pub restored_live_authority: bool,
    /// True when rerun/replay is forbidden.
    pub auto_rerun_forbidden: bool,
    /// Redaction-aware continuity label.
    pub continuity_label: String,
}

impl ReopenRecoveryDisclosure {
    /// Builds a reopen/recovery disclosure with stable tokens included.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        surface: ReopenRecoverySurface,
        source_class: ReopenRecoverySourceClass,
        closed_or_recovered_ref: impl Into<String>,
        reopen_command_id: impl Into<String>,
        restored_target_identity_ref: impl Into<String>,
        restored_selection_ref: Option<String>,
        restored_scroll_anchor_ref: Option<String>,
        restored_live_authority: bool,
        auto_rerun_forbidden: bool,
        continuity_label: impl Into<String>,
    ) -> Self {
        Self {
            surface,
            surface_token: surface.as_str().to_string(),
            source_class,
            source_class_token: source_class.as_str().to_string(),
            closed_or_recovered_ref: closed_or_recovered_ref.into(),
            reopen_command_id: reopen_command_id.into(),
            restored_target_identity_ref: restored_target_identity_ref.into(),
            restored_selection_ref,
            restored_scroll_anchor_ref,
            restored_working_directory: None,
            restored_shell_identity: None,
            restored_environment_scope_token: None,
            restored_last_command_class_token: None,
            restored_live_authority,
            auto_rerun_forbidden,
            continuity_label: continuity_label.into(),
        }
    }
}

/// Canonical transfer action record consumed by shell, support, and fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferActionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque transfer action id.
    pub action_id: String,
    /// Action family.
    pub action_kind: TransferActionKind,
    /// Stable token for [`Self::action_kind`].
    pub action_kind_token: String,
    /// Surface that emitted the action.
    pub source_surface: TransferSurfaceClass,
    /// Stable token for [`Self::source_surface`].
    pub source_surface_token: String,
    /// Source object or row ref.
    pub source_ref: String,
    /// Transfer representation class.
    pub representation_class: TransferRepresentationClass,
    /// Stable token for [`Self::representation_class`].
    pub representation_class_token: String,
    /// Target and boundary truth.
    pub target_boundary: TransferTargetBoundary,
    /// True when this is the default copy/transfer for its surface.
    pub default_transfer: bool,
    /// Sensitive-copy review posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive_copy: Option<SensitiveCopyReview>,
    /// Recovery class for this action.
    pub recovery_class: TransferRecoveryClass,
    /// Stable token for [`Self::recovery_class`].
    pub recovery_class_token: String,
    /// Named undo or recovery group when the action mutates durable state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_recovery_group: Option<NamedUndoRecoveryGroup>,
    /// Progress and cancellation disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<TransferProgressDisclosure>,
    /// Terminal paste review disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_paste_review: Option<TerminalPasteBoundaryReview>,
    /// Drag/drop verb disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_drop: Option<DragDropVerbDisclosure>,
    /// Reopen or recovery disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reopen_recovery: Option<ReopenRecoveryDisclosure>,
    /// Policy epoch or policy ref visible in support export.
    pub policy_epoch_ref: String,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
}

impl TransferActionRecord {
    /// Builds a plain-text editor copy action from the editor clipboard payload.
    pub fn editor_copy_from_payload(
        action_id: impl Into<String>,
        payload: &CopyPayload,
        source_ref: impl Into<String>,
        target_ref: impl Into<String>,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
        sensitive_value_classes: Vec<String>,
    ) -> Self {
        let action_id = action_id.into();
        let sensitive_copy = sensitive_review_for(&action_id, sensitive_value_classes);
        let representation_class = match payload.copy_variant_id {
            CopyVariantId::SelectionRaw | CopyVariantId::Line => {
                TransferRepresentationClass::PlainText
            }
        };
        Self::base(
            action_id,
            TransferActionKind::Copy,
            TransferSurfaceClass::Editor,
            source_ref.into(),
            representation_class,
            TransferTargetBoundary::new(
                target_ref,
                "Local clipboard",
                TransferBoundaryClass::LocalClipboard,
                "local system clipboard",
                None,
            ),
            true,
            TransferRecoveryClass::AuditOnlyNoMutation,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_sensitive_copy(sensitive_copy)
    }

    /// Builds an explicit non-default editor copy action.
    pub fn editor_explicit_copy_action(
        action_id: impl Into<String>,
        representation_class: EditorRepresentationClass,
        source_ref: impl Into<String>,
        target_ref: impl Into<String>,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
        sensitive_value_classes: Vec<String>,
    ) -> Self {
        let action_id = action_id.into();
        let sensitive_copy = sensitive_review_for(&action_id, sensitive_value_classes);
        Self::base(
            action_id,
            TransferActionKind::Copy,
            TransferSurfaceClass::Editor,
            source_ref.into(),
            editor_representation_to_transfer(representation_class),
            TransferTargetBoundary::new(
                target_ref,
                "Local clipboard",
                TransferBoundaryClass::LocalClipboard,
                "local system clipboard",
                None,
            ),
            false,
            TransferRecoveryClass::AuditOnlyNoMutation,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_sensitive_copy(sensitive_copy)
    }

    /// Builds a durable editor paste action with a named undo group.
    pub fn editor_paste_action(input: EditorPasteActionInput) -> Self {
        let undo_group = NamedUndoRecoveryGroup::new(
            input.undo_group_ref.clone(),
            input.undo_group_name,
            UndoRecoveryGroupScope::PasteOrDropMutation,
            "cmd:editor.paste",
            TransferRecoveryClass::ExactUndo,
            Some(input.mutation_journal_ref),
            None,
        );
        Self::base(
            input.action_id,
            TransferActionKind::Paste,
            TransferSurfaceClass::Editor,
            input.source_ref,
            TransferRepresentationClass::PlainText,
            TransferTargetBoundary::new(
                input.target_buffer_ref,
                input.target_buffer_label,
                TransferBoundaryClass::LocalWorkspace,
                "editor buffer",
                None,
            ),
            false,
            TransferRecoveryClass::ExactUndo,
            input.policy_epoch_ref,
            input.minted_at,
        )
        .with_undo_recovery_group(Some(undo_group))
    }

    /// Builds a high-risk terminal paste review action from terminal-owned evaluation.
    pub fn terminal_paste_boundary_action(input: TerminalPasteBoundaryInput) -> Self {
        let report = evaluate_paste_review(&input.review_input);
        let policy_result = terminal_policy_to_transfer(input.review_input.policy_result);
        let submit_posture = terminal_submit_to_transfer(input.review_input.submit_behavior);
        let boundary_class = boundary_for_host(input.review_input.host_class);
        let target_label = input.review_input.target_label.clone();
        let review = TerminalPasteBoundaryReview {
            session_ref: input.review_input.session_id.clone(),
            target_host_label: target_label.clone(),
            clipboard_route: input.clipboard_route,
            clipboard_route_token: input.clipboard_route.as_str().to_string(),
            line_count: input.review_input.line_count,
            bulk_hint: line_count_hint(input.review_input.line_count),
            bracketed_paste_available: input.review_input.bracketed_paste_available,
            submit_posture,
            submit_posture_token: submit_posture.as_str().to_string(),
            policy_result,
            policy_result_token: policy_result.as_str().to_string(),
            trust_result: input.trust_result,
            trust_result_token: input.trust_result.as_str().to_string(),
            cancel_action_id: "cmd:terminal.paste_review.cancel".to_string(),
            continue_action_id: "cmd:terminal.paste_review.continue".to_string(),
            commit_without_review_forbidden: report.commit_without_review_forbidden,
            auto_submit_forbidden: report.auto_submit_forbidden,
            risk_reason_tokens: report.risk_reason_tokens,
        };

        Self::base(
            input.action_id,
            TransferActionKind::Paste,
            TransferSurfaceClass::Terminal,
            input.source_ref,
            TransferRepresentationClass::PlainText,
            TransferTargetBoundary::new(
                input.review_input.session_id,
                target_label.clone(),
                boundary_class,
                host_boundary_label(input.review_input.host_class),
                Some(target_label),
            ),
            false,
            TransferRecoveryClass::EvidenceOnlyNoRerun,
            input.policy_epoch_ref,
            input.minted_at,
        )
        .with_terminal_paste_review(Some(review))
    }

    /// Builds a project-entry drag/drop action from the workspace admission packet.
    pub fn drag_drop_admission_action(
        action_id: impl Into<String>,
        packet: &AdmissionReviewPacket,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Result<Self, TransferActionError> {
        let drop = packet
            .drag_drop_review
            .as_ref()
            .ok_or(TransferActionError::MissingDragDropReview)?;
        let verb = admission_action_to_transfer_verb(drop.advertised_verb);
        let broad_workspace_mutation = matches!(
            packet.write_scope.write_scope_class,
            WriteScopeClass::ImportExtraction
                | WriteScopeClass::AddRootWorkspaceMutation
                | WriteScopeClass::RestoreStateMutation
        );
        let checkpoint_before_commit = packet.write_scope.checkpoint_required
            || drop.checkpoint_or_undo_group.is_some()
            || packet
                .write_scope
                .recovery_checkpoint_or_undo_group
                .is_some();
        let progress = progress_from_admission(drop.progress_class, &drop.cancel_action_label);
        let recovery_class = if checkpoint_before_commit {
            TransferRecoveryClass::RestoreCheckpoint
        } else if broad_workspace_mutation {
            TransferRecoveryClass::CompensatingRollback
        } else {
            TransferRecoveryClass::ExactUndo
        };
        let undo_recovery_group = drop
            .checkpoint_or_undo_group
            .clone()
            .or_else(|| packet.write_scope.recovery_checkpoint_or_undo_group.clone())
            .map(|checkpoint| {
                NamedUndoRecoveryGroup::new(
                    checkpoint.clone(),
                    format!("{} transfer checkpoint", drop.advertised_verb.as_str()),
                    UndoRecoveryGroupScope::MultiFileTransferImport,
                    "cmd:workspace.entry.drop",
                    recovery_class,
                    Some(format!("mutation:entry:{}", packet.admission_review_id)),
                    Some(checkpoint),
                )
            });
        let drag_drop = DragDropVerbDisclosure::new(
            verb,
            packet.write_scope.affected_scope_label.clone(),
            broad_workspace_mutation,
            checkpoint_before_commit,
            drop.collision_review_included || packet.requires_collision_choice(),
            "none",
            "project_entry/drop_target",
        );

        Ok(Self::base(
            action_id.into(),
            TransferActionKind::Drop,
            TransferSurfaceClass::ProjectEntry,
            packet.admission_review_id.clone(),
            TransferRepresentationClass::MetadataOnly,
            TransferTargetBoundary::new(
                packet.normalized_target_identity.identity_ref.clone(),
                packet.normalized_target_identity.normalized_label.clone(),
                TransferBoundaryClass::ProjectEntry,
                packet.destination_review.destination_label.clone(),
                None,
            ),
            false,
            recovery_class,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_progress(Some(progress))
        .with_drag_drop(Some(drag_drop))
        .with_undo_recovery_group(undo_recovery_group))
    }

    /// Builds a reopen action from a closed diff-session record.
    pub fn diff_reopen_action(
        action_id: impl Into<String>,
        closed: &DiffClosedSessionRecord,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        Self::diff_reopen_projection_action(
            action_id,
            &closed.reopen(),
            policy_epoch_ref,
            minted_at,
        )
    }

    /// Builds a reopen action from a diff reopen projection.
    pub fn diff_reopen_projection_action(
        action_id: impl Into<String>,
        projection: &DiffReopenProjection,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let selection_ref = projection
            .restored_selected_hunk_ref
            .clone()
            .or_else(|| projection.restored_selected_row_ref.clone());
        let scroll_anchor_ref = Some(format!(
            "{}@{}",
            projection.restored_scroll_anchor.first_visible_row_ref,
            projection.restored_scroll_anchor.scroll_offset
        ));
        let reopen = ReopenRecoveryDisclosure::new(
            ReopenRecoverySurface::Diff,
            ReopenRecoverySourceClass::ClosedIntentionally,
            projection.reopened_from_closed_session_ref.clone(),
            "cmd:git.diff.reopen_closed",
            projection.restored_compare_target_ref.clone(),
            selection_ref,
            scroll_anchor_ref,
            false,
            false,
            projection.continuity_label.clone(),
        );

        Self::base(
            action_id.into(),
            TransferActionKind::Reopen,
            TransferSurfaceClass::ReviewDiff,
            projection.reopened_from_closed_session_ref.clone(),
            TransferRepresentationClass::WithContext,
            TransferTargetBoundary::new(
                projection.reopened_surface_ref.clone(),
                projection.restored_compare_target_label.clone(),
                TransferBoundaryClass::ReviewSurface,
                "diff compare target",
                None,
            ),
            false,
            TransferRecoveryClass::AuditOnlyNoMutation,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_reopen_recovery(Some(reopen))
    }

    /// Builds a recover action from a restored terminal transcript or ended session.
    pub fn terminal_recover_action(
        action_id: impl Into<String>,
        restored: &RestoredTerminalRecord,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let source_class = match restored.prior_lifecycle_state_token.as_str() {
            "session_lost_transport" => ReopenRecoverySourceClass::DisconnectRecovery,
            _ => ReopenRecoverySourceClass::CrashRecovery,
        };
        let scroll_anchor_ref = restored.transcript.as_ref().map(|snapshot| {
            format!(
                "terminal-scrollback:{}-lines",
                snapshot.retained_line_count()
            )
        });
        let mut reopen = ReopenRecoveryDisclosure::new(
            ReopenRecoverySurface::Terminal,
            source_class,
            restored.session_id.to_string(),
            restored.open_fresh_session_command_id.clone(),
            restored.execution_context_ref.clone(),
            None,
            scroll_anchor_ref,
            false,
            restored.auto_rerun_forbidden,
            "terminal transcript restored; command not rerun",
        );
        reopen.restored_working_directory = restored.restore_metadata.working_directory.clone();
        reopen.restored_shell_identity = Some(restored.restore_metadata.shell_identity.clone());
        reopen.restored_environment_scope_token =
            Some(restored.restore_metadata.environment_scope_token.clone());
        reopen.restored_last_command_class_token =
            Some(restored.restore_metadata.last_command_class_token.clone());
        let boundary_class = boundary_for_host(restored.host_class);

        Self::base(
            action_id.into(),
            TransferActionKind::Recover,
            TransferSurfaceClass::Terminal,
            restored.session_id.to_string(),
            TransferRepresentationClass::MetadataOnly,
            TransferTargetBoundary::new(
                restored.session_id.to_string(),
                restored.display_title.clone(),
                boundary_class,
                restored.boundary_cue_token.clone(),
                Some(restored.target_badge.clone()),
            ),
            false,
            TransferRecoveryClass::EvidenceOnlyNoRerun,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_reopen_recovery(Some(reopen))
    }

    /// Builds a reopen action from a saved query reopen projection.
    pub fn saved_query_reopen_action(
        action_id: impl Into<String>,
        projection: &SavedQueryReopenProjection,
        policy_epoch_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let reopen = ReopenRecoveryDisclosure::new(
            ReopenRecoverySurface::QuerySearch,
            ReopenRecoverySourceClass::ClosedIntentionally,
            projection.saved_query_id_ref.clone(),
            "cmd:search.saved_query.reopen",
            projection.effective_stable_scope_id.clone(),
            Some(projection.query_session_id_ref.clone()),
            Some(projection.scope_honesty_state.as_str().to_string()),
            projection.rerun_allowed,
            !projection.rerun_allowed,
            "saved query reopened with captured scope truth",
        );
        Self::base(
            action_id.into(),
            TransferActionKind::Reopen,
            TransferSurfaceClass::Search,
            projection.saved_query_id_ref.clone(),
            TransferRepresentationClass::WithContext,
            TransferTargetBoundary::new(
                projection.query_session_id_ref.clone(),
                projection.current_scope_label.clone(),
                TransferBoundaryClass::SearchQuery,
                projection.scope_honesty_state.as_str(),
                None,
            ),
            false,
            TransferRecoveryClass::AuditOnlyNoMutation,
            policy_epoch_ref.into(),
            minted_at.into(),
        )
        .with_reopen_recovery(Some(reopen))
    }

    fn base(
        action_id: String,
        action_kind: TransferActionKind,
        source_surface: TransferSurfaceClass,
        source_ref: String,
        representation_class: TransferRepresentationClass,
        target_boundary: TransferTargetBoundary,
        default_transfer: bool,
        recovery_class: TransferRecoveryClass,
        policy_epoch_ref: String,
        minted_at: String,
    ) -> Self {
        Self {
            record_kind: TRANSFER_ACTION_RECORD_KIND.to_string(),
            schema_version: TRANSFER_ACTION_SCHEMA_VERSION,
            action_id,
            action_kind,
            action_kind_token: action_kind.as_str().to_string(),
            source_surface,
            source_surface_token: source_surface.as_str().to_string(),
            source_ref,
            representation_class,
            representation_class_token: representation_class.as_str().to_string(),
            target_boundary,
            default_transfer,
            sensitive_copy: None,
            recovery_class,
            recovery_class_token: recovery_class.as_str().to_string(),
            undo_recovery_group: None,
            progress: None,
            terminal_paste_review: None,
            drag_drop: None,
            reopen_recovery: None,
            policy_epoch_ref,
            minted_at,
        }
    }

    fn with_sensitive_copy(mut self, sensitive_copy: Option<SensitiveCopyReview>) -> Self {
        self.sensitive_copy = sensitive_copy;
        self
    }

    fn with_undo_recovery_group(
        mut self,
        undo_recovery_group: Option<NamedUndoRecoveryGroup>,
    ) -> Self {
        self.undo_recovery_group = undo_recovery_group;
        self
    }

    fn with_progress(mut self, progress: Option<TransferProgressDisclosure>) -> Self {
        self.progress = progress;
        self
    }

    fn with_terminal_paste_review(
        mut self,
        terminal_paste_review: Option<TerminalPasteBoundaryReview>,
    ) -> Self {
        self.terminal_paste_review = terminal_paste_review;
        self
    }

    fn with_drag_drop(mut self, drag_drop: Option<DragDropVerbDisclosure>) -> Self {
        self.drag_drop = drag_drop;
        self
    }

    fn with_reopen_recovery(mut self, reopen_recovery: Option<ReopenRecoveryDisclosure>) -> Self {
        self.reopen_recovery = reopen_recovery;
        self
    }
}

/// Input for a durable editor paste action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorPasteActionInput {
    /// Opaque transfer action id.
    pub action_id: String,
    /// Source clipboard/register ref.
    pub source_ref: String,
    /// Target buffer ref.
    pub target_buffer_ref: String,
    /// Target buffer label.
    pub target_buffer_label: String,
    /// Named undo group ref.
    pub undo_group_ref: String,
    /// Named undo group label.
    pub undo_group_name: String,
    /// Mutation-journal ref for the paste.
    pub mutation_journal_ref: String,
    /// Policy epoch or policy ref visible in support export.
    pub policy_epoch_ref: String,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
}

/// Input for a high-risk terminal paste boundary action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPasteBoundaryInput {
    /// Opaque transfer action id.
    pub action_id: String,
    /// Source clipboard/register ref.
    pub source_ref: String,
    /// Terminal-owned paste-review input.
    pub review_input: TerminalPasteReviewInput,
    /// Clipboard route disclosed before commit.
    pub clipboard_route: ClipboardRouteClass,
    /// Trust result disclosed before commit.
    pub trust_result: TransferGateResult,
    /// Policy epoch or policy ref visible in support export.
    pub policy_epoch_ref: String,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
}

/// Error returned when a transfer projection cannot be built from source records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferActionError {
    /// The admission packet did not carry drag/drop review fields.
    MissingDragDropReview,
}

impl std::fmt::Display for TransferActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDragDropReview => {
                write!(f, "admission packet is missing drag/drop review")
            }
        }
    }
}

impl std::error::Error for TransferActionError {}

/// Support/export projection for interaction-integrity packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegritySupportExport {
    /// Support packet id.
    pub support_packet_ref: String,
    /// Action ids included in the packet.
    pub included_action_ids: Vec<String>,
    /// True only after a separate high-friction raw-body path.
    pub raw_payload_bodies_included: bool,
    /// Omitted payload classes named explicitly for support.
    pub omitted_payload_classes: Vec<String>,
    /// Schema refs support tooling can use to decode actions.
    pub schema_refs: Vec<String>,
}

impl InteractionIntegritySupportExport {
    /// Builds a metadata-only support export from action ids.
    pub fn metadata_only(
        support_packet_ref: impl Into<String>,
        included_action_ids: Vec<String>,
    ) -> Self {
        Self {
            support_packet_ref: support_packet_ref.into(),
            included_action_ids,
            raw_payload_bodies_included: false,
            omitted_payload_classes: vec![
                "raw_clipboard_body".to_string(),
                "raw_terminal_paste_body".to_string(),
                "raw_file_body".to_string(),
                "raw_private_path".to_string(),
            ],
            schema_refs: vec![
                "schemas/events/transfer_action.schema.json".to_string(),
                "schemas/ux/interaction_safety.schema.json".to_string(),
                "schemas/terminal/session_restore_metadata.schema.json".to_string(),
                "schemas/history/local_history_alpha.schema.json".to_string(),
            ],
        }
    }
}

/// Canonical alpha packet tying transfer, paste, drop, reopen, and support truth together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
    /// Source docs, schemas, and artifacts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Transfer actions covered by this alpha packet.
    pub actions: Vec<TransferActionRecord>,
    /// Metadata-only support export projection.
    pub support_export: InteractionIntegritySupportExport,
}

impl InteractionIntegrityAlphaPacket {
    /// Builds an alpha packet with a metadata-only support projection.
    pub fn new(
        packet_id: impl Into<String>,
        minted_at: impl Into<String>,
        actions: Vec<TransferActionRecord>,
    ) -> Self {
        let packet_id = packet_id.into();
        let support_ids = actions
            .iter()
            .map(|action| action.action_id.clone())
            .collect();
        Self {
            record_kind: INTERACTION_INTEGRITY_ALPHA_PACKET_RECORD_KIND.to_string(),
            schema_version: TRANSFER_ACTION_SCHEMA_VERSION,
            packet_id: packet_id.clone(),
            minted_at: minted_at.into(),
            source_contract_refs: default_source_contract_refs(),
            actions,
            support_export: InteractionIntegritySupportExport::metadata_only(
                format!("support:interaction-integrity:{packet_id}"),
                support_ids,
            ),
        }
    }

    /// Validates the alpha minimum-slice invariants.
    pub fn validate(&self) -> InteractionIntegrityValidationReport {
        let mut violations = Vec::new();
        validate_default_copy(self, &mut violations);
        validate_sensitive_copy(self, &mut violations);
        validate_terminal_paste(self, &mut violations);
        validate_drag_drop(self, &mut violations);
        validate_reopen_recover(self, &mut violations);
        validate_named_groups(self, &mut violations);
        validate_support_export(self, &mut violations);

        let coverage = InteractionIntegrityCoverage::from_packet(self);
        let status = if violations.is_empty() {
            "passed"
        } else {
            "failed"
        };

        InteractionIntegrityValidationReport {
            record_kind: INTERACTION_INTEGRITY_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: TRANSFER_ACTION_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            status: status.to_string(),
            coverage,
            violations,
        }
    }
}

/// Coverage summary computed by [`InteractionIntegrityAlphaPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityCoverage {
    /// Number of transfer actions.
    pub action_count: usize,
    /// Surface tokens represented in the packet.
    pub surface_tokens: Vec<String>,
    /// Reopen/recover surface tokens represented in the packet.
    pub reopen_recovery_surface_tokens: Vec<String>,
    /// True when at least one high-risk terminal paste is covered.
    pub high_risk_terminal_paste_covered: bool,
    /// True when at least one drag/drop verb disclosure is covered.
    pub drag_drop_verb_truth_covered: bool,
    /// True when at least one large transfer has progress and cancel.
    pub large_transfer_progress_cancel_covered: bool,
    /// True when support export carries action ids and no raw bodies.
    pub support_export_metadata_only: bool,
}

impl InteractionIntegrityCoverage {
    fn from_packet(packet: &InteractionIntegrityAlphaPacket) -> Self {
        let surface_tokens = packet
            .actions
            .iter()
            .map(|action| action.source_surface_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let reopen_recovery_surface_tokens = packet
            .actions
            .iter()
            .filter_map(|action| action.reopen_recovery.as_ref())
            .map(|reopen| reopen.surface_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let high_risk_terminal_paste_covered = packet
            .actions
            .iter()
            .any(|action| action.terminal_paste_review.is_some());
        let drag_drop_verb_truth_covered = packet
            .actions
            .iter()
            .any(|action| action.drag_drop.is_some());
        let large_transfer_progress_cancel_covered = packet.actions.iter().any(|action| {
            action.progress.as_ref().is_some_and(|progress| {
                progress.progress_class == TransferProgressClass::DurableProgressWithCancel
                    && progress.cancel_available
            })
        });
        let support_export_metadata_only = !packet.support_export.raw_payload_bodies_included
            && !packet.support_export.included_action_ids.is_empty();
        Self {
            action_count: packet.actions.len(),
            surface_tokens,
            reopen_recovery_surface_tokens,
            high_risk_terminal_paste_covered,
            drag_drop_verb_truth_covered,
            large_transfer_progress_cancel_covered,
            support_export_metadata_only,
        }
    }
}

/// One validation violation for the interaction-integrity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityViolation {
    /// Stable violation id.
    pub violation_id: String,
    /// Action id when the violation is action-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// Reviewable summary.
    pub summary: String,
}

/// Validation report for the interaction-integrity alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityValidationReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id validated by this report.
    pub packet_id: String,
    /// `passed` when no violations were found.
    pub status: String,
    /// Coverage summary.
    pub coverage: InteractionIntegrityCoverage,
    /// Violations found during validation.
    pub violations: Vec<InteractionIntegrityViolation>,
}

impl InteractionIntegrityValidationReport {
    /// Returns true when validation passed.
    pub fn passed(&self) -> bool {
        self.status == "passed" && self.violations.is_empty()
    }
}

fn default_source_contract_refs() -> Vec<String> {
    vec![
        "docs/ux/clipboard_history_contract.md".to_string(),
        "docs/ux/copy_export_representation_parity.md".to_string(),
        "docs/ux/cross_window_transfer_contract.md".to_string(),
        "docs/ux/shell_close_reopen_contract.md".to_string(),
        "docs/ux/restore_fidelity_classes.md".to_string(),
        "schemas/events/transfer_action.schema.json".to_string(),
    ]
}

fn validate_default_copy(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let mut found_default_copy = false;
    for action in &packet.actions {
        if action.action_kind == TransferActionKind::Copy && action.default_transfer {
            found_default_copy = true;
            if action.representation_class != TransferRepresentationClass::PlainText {
                violations.push(violation(
                    "default_copy_not_plain_text",
                    Some(&action.action_id),
                    "default copy action is not plain text",
                ));
            }
        }
    }
    if !found_default_copy {
        violations.push(violation(
            "default_copy_missing",
            None,
            "no default copy action was present",
        ));
    }
}

fn validate_sensitive_copy(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    for action in &packet.actions {
        if let Some(sensitive) = action.sensitive_copy.as_ref() {
            if sensitive.sensitive_value_classes.is_empty()
                || !sensitive.preview_required
                || !sensitive.label_first_path
                || !sensitive.clipboard_write_deferred_until_review
            {
                violations.push(violation(
                    "sensitive_copy_preview_incomplete",
                    Some(&action.action_id),
                    "sensitive copy does not require label-first preview before clipboard write",
                ));
            }
        }
    }
}

fn validate_terminal_paste(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let terminal_pastes = packet
        .actions
        .iter()
        .filter(|action| action.terminal_paste_review.is_some())
        .collect::<Vec<_>>();
    if terminal_pastes.is_empty() {
        violations.push(violation(
            "terminal_paste_review_missing",
            None,
            "no terminal paste review action was present",
        ));
        return;
    }
    for action in terminal_pastes {
        let paste = action.terminal_paste_review.as_ref().expect("filtered");
        if paste.line_count == 0
            || paste.target_host_label.trim().is_empty()
            || !paste.commit_without_review_forbidden
            || !paste.auto_submit_forbidden
            || paste.cancel_action_id.trim().is_empty()
            || paste.continue_action_id.trim().is_empty()
        {
            violations.push(violation(
                "terminal_paste_review_incomplete",
                Some(&action.action_id),
                "terminal paste review is missing boundary, line count, cancel/continue, or no-bypass truth",
            ));
        }
    }
}

fn validate_drag_drop(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let drops = packet
        .actions
        .iter()
        .filter(|action| action.drag_drop.is_some())
        .collect::<Vec<_>>();
    if drops.is_empty() {
        violations.push(violation(
            "drag_drop_verb_truth_missing",
            None,
            "no drag/drop verb disclosure action was present",
        ));
        return;
    }
    for action in drops {
        let drop = action.drag_drop.as_ref().expect("filtered");
        if matches!(drop.verb, TransferVerb::Blocked)
            || drop.destination_scope.trim().is_empty()
            || drop.target_slot_ref.trim().is_empty()
        {
            violations.push(violation(
                "drag_drop_verb_truth_incomplete",
                Some(&action.action_id),
                "drag/drop disclosure is missing an allowed verb, destination scope, or target slot",
            ));
        }
        if action.progress.as_ref().is_some_and(|progress| {
            progress.progress_class == TransferProgressClass::DurableProgressWithCancel
                && !progress.cancel_available
        }) {
            violations.push(violation(
                "large_transfer_cancel_missing",
                Some(&action.action_id),
                "large transfer progress does not expose cancel",
            ));
        }
        if drop.broad_workspace_mutation && !drop.checkpoint_before_commit {
            violations.push(violation(
                "broad_drop_checkpoint_missing",
                Some(&action.action_id),
                "broad workspace drop does not create or verify a checkpoint",
            ));
        }
    }
}

fn validate_reopen_recover(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let mut surfaces = BTreeSet::new();
    let mut source_classes = BTreeSet::new();
    for action in &packet.actions {
        if let Some(reopen) = action.reopen_recovery.as_ref() {
            surfaces.insert(reopen.surface);
            source_classes.insert(reopen.source_class);
            if reopen.restored_target_identity_ref.trim().is_empty()
                || reopen.reopen_command_id.trim().is_empty()
            {
                violations.push(violation(
                    "reopen_target_identity_missing",
                    Some(&action.action_id),
                    "reopen/recover action is missing target identity or command id",
                ));
            }
            if matches!(
                reopen.source_class,
                ReopenRecoverySourceClass::CrashRecovery
                    | ReopenRecoverySourceClass::DisconnectRecovery
            ) && !reopen.auto_rerun_forbidden
            {
                violations.push(violation(
                    "recovery_allows_auto_rerun",
                    Some(&action.action_id),
                    "crash/disconnect recovery allows automatic rerun",
                ));
            }
        }
    }
    if surfaces.len() < 2 {
        violations.push(violation(
            "reopen_recover_surface_coverage_missing",
            None,
            "reopen/recover coverage does not include at least two high-frequency surfaces",
        ));
    }
    if !source_classes.contains(&ReopenRecoverySourceClass::ClosedIntentionally)
        || !source_classes.iter().any(|class| {
            matches!(
                class,
                ReopenRecoverySourceClass::CrashRecovery
                    | ReopenRecoverySourceClass::DisconnectRecovery
            )
        })
    {
        violations.push(violation(
            "reopen_recover_source_distinction_missing",
            None,
            "reopen/recover coverage does not distinguish intentional close from crash/disconnect recovery",
        ));
    }
}

fn validate_named_groups(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let mut scopes = BTreeSet::new();
    for action in &packet.actions {
        if let Some(group) = action.undo_recovery_group.as_ref() {
            scopes.insert(group.group_scope);
            if group.display_name.trim().is_empty() || group.group_ref.trim().is_empty() {
                violations.push(violation(
                    "named_group_identity_missing",
                    Some(&action.action_id),
                    "named undo/recovery group is missing identity or display name",
                ));
            }
        }
    }
    if !scopes.contains(&UndoRecoveryGroupScope::MultiFileTransferImport) {
        violations.push(violation(
            "multi_file_transfer_group_missing",
            None,
            "no named undo/recovery group covers a multi-file or multi-step transfer/import path",
        ));
    }
    if !scopes.contains(&UndoRecoveryGroupScope::PasteOrDropMutation) {
        violations.push(violation(
            "paste_drop_group_missing",
            None,
            "no named undo/recovery group covers a durable paste/drop mutation",
        ));
    }
}

fn validate_support_export(
    packet: &InteractionIntegrityAlphaPacket,
    violations: &mut Vec<InteractionIntegrityViolation>,
) {
    let action_ids = packet
        .actions
        .iter()
        .map(|action| action.action_id.as_str())
        .collect::<BTreeSet<_>>();
    let export_ids = packet
        .support_export
        .included_action_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if packet.support_export.raw_payload_bodies_included {
        violations.push(violation(
            "support_export_raw_body_included",
            None,
            "support export includes raw payload bodies",
        ));
    }
    if action_ids != export_ids {
        violations.push(violation(
            "support_export_action_ids_mismatch",
            None,
            "support export action id list does not match packet actions",
        ));
    }
}

fn violation(
    violation_id: impl Into<String>,
    action_id: Option<&str>,
    summary: impl Into<String>,
) -> InteractionIntegrityViolation {
    InteractionIntegrityViolation {
        violation_id: violation_id.into(),
        action_id: action_id.map(ToOwned::to_owned),
        summary: summary.into(),
    }
}

fn sensitive_review_for(
    action_id: &str,
    sensitive_value_classes: Vec<String>,
) -> Option<SensitiveCopyReview> {
    if sensitive_value_classes.is_empty() {
        None
    } else {
        Some(SensitiveCopyReview {
            sensitive_value_classes,
            preview_required: true,
            label_first_path: true,
            preview_action_id: format!("cmd:clipboard.preview_sensitive:{action_id}"),
            clipboard_write_deferred_until_review: true,
        })
    }
}

fn editor_representation_to_transfer(
    class: EditorRepresentationClass,
) -> TransferRepresentationClass {
    match class {
        EditorRepresentationClass::Raw => TransferRepresentationClass::RawSafe,
        EditorRepresentationClass::Rendered => TransferRepresentationClass::Rendered,
        EditorRepresentationClass::Escaped => TransferRepresentationClass::Escaped,
        EditorRepresentationClass::Sanitized => TransferRepresentationClass::Sanitized,
        EditorRepresentationClass::BlockedMetadataOnly => TransferRepresentationClass::MetadataOnly,
        EditorRepresentationClass::Generated => TransferRepresentationClass::Generated,
    }
}

fn terminal_policy_to_transfer(result: TerminalPastePolicyResult) -> TransferGateResult {
    match result {
        TerminalPastePolicyResult::Allowed => TransferGateResult::Allowed,
        TerminalPastePolicyResult::ReviewRequired => TransferGateResult::ReviewRequired,
        TerminalPastePolicyResult::Blocked => TransferGateResult::Blocked,
    }
}

fn terminal_submit_to_transfer(behavior: TerminalPasteSubmitBehavior) -> TransferSubmitPosture {
    match behavior {
        TerminalPasteSubmitBehavior::NoAutoSubmit => TransferSubmitPosture::NoAutoSubmit,
        TerminalPasteSubmitBehavior::SubmitAfterReview => TransferSubmitPosture::SubmitAfterReview,
    }
}

fn boundary_for_host(host_class: HostClass) -> TransferBoundaryClass {
    match host_class {
        HostClass::HostDesktop => TransferBoundaryClass::LocalWorkspace,
        HostClass::RemoteAgentPrimary => TransferBoundaryClass::RemoteHost,
        HostClass::LocalContainer => TransferBoundaryClass::ContainerHost,
    }
}

fn host_boundary_label(host_class: HostClass) -> String {
    match host_class {
        HostClass::HostDesktop => "local terminal".to_string(),
        HostClass::RemoteAgentPrimary => "remote terminal".to_string(),
        HostClass::LocalContainer => "container terminal".to_string(),
    }
}

fn line_count_hint(line_count: u32) -> String {
    match line_count {
        0 => "empty paste".to_string(),
        1 => "single line".to_string(),
        count => format!("{count} lines"),
    }
}

fn admission_action_to_transfer_verb(action: AdmissionAction) -> TransferVerb {
    match action {
        AdmissionAction::Copy => TransferVerb::Copy,
        AdmissionAction::Move => TransferVerb::Move,
        AdmissionAction::Open
        | AdmissionAction::OpenMinimal
        | AdmissionAction::AddRoot
        | AdmissionAction::AddExistingAsRoot => TransferVerb::Open,
        AdmissionAction::Import
        | AdmissionAction::CloneHere
        | AdmissionAction::CloneOnly
        | AdmissionAction::CloneAndReview
        | AdmissionAction::CloneAndOpen
        | AdmissionAction::CloneAndAdd
        | AdmissionAction::ReuseExisting => TransferVerb::Import,
        AdmissionAction::Split => TransferVerb::Split,
        AdmissionAction::InspectOnly | AdmissionAction::RevealTarget => TransferVerb::Open,
        AdmissionAction::Cancel | AdmissionAction::SetUpLater | AdmissionAction::CloneElsewhere => {
            TransferVerb::Blocked
        }
    }
}

fn progress_from_admission(
    class: AdmissionTransferProgressClass,
    cancel_label: &str,
) -> TransferProgressDisclosure {
    let progress_class = match class {
        AdmissionTransferProgressClass::NotRequired => TransferProgressClass::NotRequired,
        AdmissionTransferProgressClass::InlineProgress => TransferProgressClass::InlineProgress,
        AdmissionTransferProgressClass::DurableProgressWithCancel => {
            TransferProgressClass::DurableProgressWithCancel
        }
    };
    TransferProgressDisclosure::new(
        progress_class,
        class.as_str(),
        class == AdmissionTransferProgressClass::DurableProgressWithCancel,
        if class == AdmissionTransferProgressClass::DurableProgressWithCancel {
            Some(format!("cmd:transfer.cancel:{}", sanitize_id(cancel_label)))
        } else {
            None
        },
        "cmd:transfer.open_summary",
    )
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

/// Returns action counts grouped by kind for support summaries.
pub fn action_counts_by_kind(actions: &[TransferActionRecord]) -> BTreeMap<String, usize> {
    actions.iter().fold(BTreeMap::new(), |mut counts, action| {
        *counts.entry(action.action_kind_token.clone()).or_default() += 1;
        counts
    })
}
