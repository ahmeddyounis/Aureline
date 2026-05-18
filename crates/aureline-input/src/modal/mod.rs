//! Modal editing state, register routing, and macro replay safety contracts.
//!
//! This module models the input-facing records that shell, help, migration, and
//! support surfaces use to explain modal keyboard behavior without inventing a
//! second command language. Records carry stable command ids and resolver refs,
//! but never raw key history, clipboard contents, register contents, or edited
//! text.

use std::collections::BTreeSet;

use aureline_commands::{CommandId, CommandRevisionRef};
use serde::{Deserialize, Serialize};

/// Schema version exported with modal state snapshots.
pub const MODAL_STATE_SCHEMA_VERSION: u32 = 1;

/// Schema version exported with macro replay preview sheets.
pub const MACRO_REPLAY_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ModalStateSnapshot`] payloads.
pub const MODAL_STATE_RECORD_KIND: &str = "modal_state_snapshot_record";

/// Stable record-kind tag for [`MacroReplayPreviewRecord`] payloads.
pub const MACRO_REPLAY_PREVIEW_RECORD_KIND: &str = "macro_replay_preview_record";

/// Stable record-kind tag for [`ModalStateValidationReport`] payloads.
pub const MODAL_STATE_VALIDATION_RECORD_KIND: &str = "modal_state_validation_report";

/// Stable record-kind tag for [`MacroReplayPreviewValidationReport`] payloads.
pub const MACRO_REPLAY_PREVIEW_VALIDATION_RECORD_KIND: &str =
    "macro_replay_preview_validation_report";

/// Canonical visible editor mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalMode {
    /// Normal mode where operator and motion keys change editing state.
    Normal,
    /// Insert mode where printable input edits text.
    Insert,
    /// Visual selection mode.
    Visual,
    /// Visual line selection mode.
    VisualLine,
    /// Visual block selection mode.
    VisualBlock,
    /// Replace mode.
    Replace,
    /// Select mode.
    Select,
    /// Command-line entry mode.
    Command,
    /// Modeless fallback mode.
    Modeless,
    /// Unsupported mode on the current surface.
    Unsupported,
}

impl ModalMode {
    /// Returns the stable token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Insert => "insert",
            Self::Visual => "visual",
            Self::VisualLine => "visual_line",
            Self::VisualBlock => "visual_block",
            Self::Replace => "replace",
            Self::Select => "select",
            Self::Command => "command",
            Self::Modeless => "modeless",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Source class for the active modal or keymap layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeymapSourceKind {
    /// Core product modal profile.
    CoreProfile,
    /// Product-native keyboard profile.
    ProductNativeProfile,
    /// Imported bridge from an incumbent keymap.
    ImportedBridge,
    /// Extension-provided profile.
    ExtensionProfile,
    /// Workspace override.
    WorkspaceOverride,
    /// Policy-narrowed profile.
    PolicyNarrowed,
}

/// Surface family where modal input is being interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalSurfaceKind {
    /// Source editor.
    SourceEditor,
    /// Diff editor.
    DiffEditor,
    /// Commit message editor.
    CommitMessage,
    /// Notebook code cell.
    NotebookCodeCell,
    /// Terminal surface.
    Terminal,
    /// Rendered documentation surface.
    RenderedDocs,
    /// Browser companion surface.
    BrowserCompanion,
    /// Large-file limited editor surface.
    LargeFileEditor,
    /// Restricted-mode editor surface.
    RestrictedModeEditor,
    /// Command palette.
    CommandPalette,
}

/// Fidelity posture for the current modal surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFidelityClass {
    /// Full modal fidelity is available.
    FullFidelity,
    /// Restricted trust narrowed available modal behavior.
    RestrictedModeNarrowed,
    /// Browser companion host limits narrowed modal behavior.
    BrowserCompanionLimited,
    /// Large-file mode narrowed modal behavior.
    LargeFileLimited,
    /// Remote or clipboard bridge limits narrowed modal behavior.
    RemoteClipboardLimited,
    /// Surface cannot honor modal behavior faithfully.
    UnsupportedSurface,
    /// Policy blocks modal behavior.
    PolicyBlocked,
}

impl SurfaceFidelityClass {
    /// True when the surface must disclose a narrowed-fidelity label.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::FullFidelity)
    }
}

/// Recording state shown in the mode strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRecordingState {
    /// No macro is being recorded.
    NotRecording,
    /// Recording is active for the named register.
    Recording,
    /// Recording is paused.
    Paused,
}

/// Resolution state shown by a leader or sequence guide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalSequenceState {
    /// The sequence is ready to resolve.
    Ready,
    /// The sequence is waiting for another key.
    PartialWaiting,
    /// Multiple candidates remain possible.
    AmbiguousWaiting,
    /// A conflict must be reviewed.
    ConflictRequiresReview,
    /// Host platform blocked dispatch.
    BlockedByHost,
    /// Admin or trust policy blocked dispatch.
    BlockedByPolicy,
    /// Current surface cannot honor the sequence.
    UnsupportedSurface,
    /// Sequence timed out visibly.
    TimedOut,
    /// Sequence was cancelled.
    Cancelled,
    /// Sequence resolved to a command.
    Resolved,
}

impl ModalSequenceState {
    /// True when the state must include a diagnostic or blocked reason.
    pub const fn requires_diagnostic(self) -> bool {
        matches!(
            self,
            Self::ConflictRequiresReview
                | Self::BlockedByHost
                | Self::BlockedByPolicy
                | Self::UnsupportedSurface
                | Self::TimedOut
        )
    }
}

/// Availability state for one possible next key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextKeyAvailability {
    /// The next key is available on this surface.
    Available,
    /// The next key is shadowed by a higher-precedence binding.
    Shadowed,
    /// The next key is blocked by the host.
    BlockedByHost,
    /// The next key is blocked by policy.
    BlockedByPolicy,
    /// The next key is unsupported on this surface.
    UnsupportedOnSurface,
    /// The next key requires conflict review.
    RequiresReview,
}

impl NextKeyAvailability {
    /// True when a next-key row must explain why it cannot dispatch.
    pub const fn requires_unavailable_reason(self) -> bool {
        !matches!(self, Self::Available)
    }
}

/// Register and clipboard scope vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisterScopeKind {
    /// Editor-local unnamed or numbered register.
    EditorLocal,
    /// System clipboard route.
    SystemClipboard,
    /// Remote clipboard bridge route.
    RemoteClipboardBridge,
    /// Named register.
    NamedRegister,
    /// Search-history register.
    SearchHistory,
    /// Macro register.
    Macro,
    /// Special read-only register.
    SpecialRegister,
    /// Policy-blocked register route.
    PolicyBlocked,
    /// Unknown register route.
    Unknown,
}

/// Clipboard route observed for the current register action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardRouteKind {
    /// Editor-local clipboard/register only.
    LocalEditorRegister,
    /// Local desktop system clipboard.
    LocalSystemClipboard,
    /// Remote clipboard bridge.
    RemoteClipboardBridge,
    /// Clipboard bridge suppressed by remote policy.
    RemoteBridgeSuppressed,
    /// Clipboard route blocked by admin policy.
    AdminBlocked,
    /// Clipboard route unsupported on this surface.
    UnsupportedOnSurface,
}

/// Policy posture for a register or clipboard route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisterPolicyState {
    /// Route is allowed.
    Allowed,
    /// Route is allowed only after review.
    RequiresReview,
    /// Remote policy suppressed the bridge.
    SuppressedByRemotePolicy,
    /// Admin policy blocked the route.
    BlockedByAdminPolicy,
    /// Current surface cannot support the route.
    UnsupportedOnSurface,
}

impl RegisterPolicyState {
    /// True when the route must be explained before paste or replay.
    pub const fn requires_diagnostic(self) -> bool {
        !matches!(self, Self::Allowed)
    }
}

/// Operator vocabulary used by operator-pending overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorKind {
    /// Delete operator.
    Delete,
    /// Yank or copy operator.
    Yank,
    /// Change operator.
    Change,
    /// Format operator.
    Format,
    /// Indent operator.
    Indent,
    /// Replace operator.
    Replace,
    /// Unknown imported operator.
    Unknown,
}

/// Target object vocabulary for operator-pending state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorObjectClass {
    /// Target is a motion.
    Motion,
    /// Target is a text object.
    TextObject,
    /// Target is a line range.
    LineRange,
    /// Target is an active selection.
    Selection,
    /// Target is a search result.
    SearchResult,
    /// Target is unresolved.
    Pending,
}

/// Scope vocabulary for operator or macro replay targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditTargetScope {
    /// Current buffer only.
    CurrentBuffer,
    /// Multiple buffers.
    MultipleBuffers,
    /// Workspace scope.
    Workspace,
    /// Settings or profile scope.
    Settings,
    /// Run-capable command scope.
    RunCapableCommand,
    /// Unknown or imported scope.
    Unknown,
}

impl EditTargetScope {
    /// True when scope widening requires review.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::CurrentBuffer)
    }
}

/// Recovery action vocabulary shown from modal input surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalRecoveryActionClass {
    /// Open command palette search.
    OpenCommandPalette,
    /// Open keymap diagnostics.
    OpenKeymapDiagnostics,
    /// Open migration help.
    OpenMigrationHelp,
    /// Switch to insert mode.
    SwitchToInsert,
    /// Reset to default modeless mode.
    ResetKeyboardMode,
    /// Retry on a supported surface.
    RetryOnSupportedSurface,
    /// Cancel the pending sequence.
    CancelSequence,
    /// Review register or clipboard route.
    ReviewRegisterRoute,
    /// Review macro replay target.
    ReviewMacroReplay,
}

/// Write class vocabulary used by macro replay preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayWriteClass {
    /// Read-only step.
    ReadOnly,
    /// Mutates the current editor buffer.
    EditorBufferMutation,
    /// Mutates multiple editor buffers.
    EditorMultiFileMutation,
    /// Mutates settings or profile state.
    SettingsMutation,
    /// Invokes a process, task, test, debug, or terminal run path.
    RunCapableCommand,
    /// Uses network-capable command behavior.
    NetworkCapableCommand,
}

impl MacroReplayWriteClass {
    /// True when this write class requires replay review.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::ReadOnly | Self::EditorBufferMutation)
    }
}

/// Risk class vocabulary for macro replay preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayRiskClass {
    /// Replay would leave the current buffer.
    CrossFileScope,
    /// Replay would mutate settings or profiles.
    SettingsMutation,
    /// Replay would invoke a run-capable command.
    RunCapableCommand,
    /// Replay depends on a changed register or clipboard route.
    ClipboardRouteChange,
    /// Remote clipboard bridge is suppressed.
    RemoteClipboardSuppressed,
    /// Admin policy blocked the route.
    AdminPolicyBlocked,
    /// Imported sequence has no supported command mapping.
    UnsupportedImportedSequence,
    /// Replay depends on timing-sensitive behavior.
    UnstableTiming,
}

/// Replay decision vocabulary for macro review sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayDecisionClass {
    /// Replay may proceed without additional review.
    ProceedLocalEditorOnly,
    /// Review is required before replay.
    ReviewRequiredBeforeReplay,
    /// Replay should be saved as a declarative recipe.
    PromoteToRecipe,
    /// Replay should be downgraded to no-mutation observation.
    DowngradeToObserverNoMutation,
    /// Replay is denied closed.
    DeniedUnsafeReplay,
}

impl MacroReplayDecisionClass {
    /// True when this decision silently dispatches the replay.
    pub const fn is_silent_proceed(self) -> bool {
        matches!(self, Self::ProceedLocalEditorOnly)
    }
}

/// Optional fixture metadata carried by modal input fixture files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Review-safe scenario summary.
    pub scenario: String,
}

/// One compact mode strip rendered near the editor status area.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStripRecord {
    /// Current editor mode.
    pub current_mode: ModalMode,
    /// Source kind for the active keymap or modal layer.
    pub keymap_source_kind: KeymapSourceKind,
    /// Opaque source ref for the active keymap or modal layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keymap_source_ref: Option<String>,
    /// Current surface family.
    pub surface_kind: ModalSurfaceKind,
    /// Opaque surface ref.
    pub surface_ref: String,
    /// Recording state.
    pub recording_state: MacroRecordingState,
    /// Macro register shown while recording.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording_register_name: Option<String>,
    /// Pending count shown in the strip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<u32>,
    /// Short pending-state summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_state_label: Option<String>,
    /// Read/write posture label.
    pub read_write_posture_label: String,
}

/// One possible next key in the leader or sequence guide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceNextKeyRecord {
    /// Key label shown to the user.
    pub key_label: String,
    /// Human group label.
    pub group_label: String,
    /// Canonical command id when the row can resolve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<CommandId>,
    /// Human command title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_title: Option<String>,
    /// Source provenance ref.
    pub source_ref: String,
    /// Availability state on the current surface.
    pub availability: NextKeyAvailability,
    /// Required when availability is not `available`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason_label: Option<String>,
}

/// Inline guide for a partial, ambiguous, or leader sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceGuideRecord {
    /// Typed prefix shown locally in the focused surface.
    pub typed_prefix_display: String,
    /// Resolution state.
    pub sequence_state: ModalSequenceState,
    /// Timeout in milliseconds when the sequence waits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    /// Candidate next keys.
    #[serde(default)]
    pub next_keys: Vec<SequenceNextKeyRecord>,
    /// Resolver packet ref backing the guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolver_packet_ref: Option<String>,
    /// Conflict review ref when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_review_ref: Option<String>,
    /// Unsupported or blocked reason shown without stealing focus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_label: Option<String>,
}

/// Register target selected for an edit, paste, search, or macro action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterTargetRecord {
    /// Register scope class.
    pub scope_kind: RegisterScopeKind,
    /// Register name when the route is named.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_name: Option<String>,
    /// Reviewable display label.
    pub display_label: String,
}

/// Inspector row for the active register and clipboard route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterRouteRecord {
    /// Active register target.
    pub active_register: RegisterTargetRecord,
    /// Effective clipboard route.
    pub clipboard_route: ClipboardRouteKind,
    /// Policy state for the route.
    pub policy_state: RegisterPolicyState,
    /// True when this is the profile default route.
    pub is_default_route: bool,
    /// True when the route changes risk or result.
    pub route_changes_result: bool,
    /// Opaque remote boundary ref when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_boundary_ref: Option<String>,
    /// Opaque policy ref when relevant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    /// Reviewable route label.
    pub route_label: String,
    /// Diagnostics shown before destructive paste or replay.
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// Operator-pending overlay for scope-changing commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPendingRecord {
    /// Pending operator.
    pub operator: OperatorKind,
    /// Count applied to the operator.
    pub count: u32,
    /// Pending object class.
    pub object_class: OperatorObjectClass,
    /// Target scope.
    pub target_scope: EditTargetScope,
    /// True when the pending operation crosses file boundaries.
    pub crosses_files: bool,
    /// Replay implication label.
    pub replay_implication_label: String,
    /// True when review is required before execution.
    pub review_required: bool,
}

/// Current surface support row for modal behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceFidelityRecord {
    /// Surface fidelity class.
    pub fidelity_class: SurfaceFidelityClass,
    /// Labels explaining any narrowed behavior.
    #[serde(default)]
    pub narrowed_fidelity_labels: Vec<String>,
    /// Support matrix refs.
    #[serde(default)]
    pub support_matrix_refs: Vec<String>,
}

/// Keyboard-first recovery action from a modal surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalRecoveryActionRecord {
    /// Action class.
    pub action_class: ModalRecoveryActionClass,
    /// Button or command label.
    pub label: String,
    /// Target ref for the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
}

/// Complete modal state snapshot for a focused workflow surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalStateSnapshot {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ModalFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub state_id: String,
    /// Mode strip projection.
    pub mode_strip: ModeStripRecord,
    /// Optional leader or sequence guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_guide: Option<SequenceGuideRecord>,
    /// Register and clipboard route inspector.
    pub register_route: RegisterRouteRecord,
    /// Optional operator-pending overlay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_pending: Option<OperatorPendingRecord>,
    /// Current surface fidelity posture.
    pub surface_fidelity: SurfaceFidelityRecord,
    /// Keyboard-first recovery actions.
    #[serde(default)]
    pub recovery_actions: Vec<ModalRecoveryActionRecord>,
    /// Reviewable diagnostics.
    #[serde(default)]
    pub diagnostics: Vec<String>,
    /// Guardrail: no raw key history crosses this boundary.
    pub raw_key_history_present: bool,
    /// Guardrail: no raw clipboard contents cross this boundary.
    pub raw_clipboard_contents_present: bool,
    /// Guardrail: no raw register contents cross this boundary.
    pub raw_register_contents_present: bool,
    /// Redaction class applied to exportable text.
    pub redaction_class: String,
    /// Timestamp at which the snapshot was emitted.
    pub emitted_at: String,
}

impl ModalStateSnapshot {
    /// Validates this snapshot against modal inspectability invariants.
    pub fn validate(&self) -> ModalStateValidationReport {
        let mut validator = ModalStateValidator::new(self);
        validator.run();
        validator.finish()
    }
}

/// One command step shown in macro replay preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayCommandStep {
    /// Stable step id.
    pub step_id: String,
    /// Stable command id.
    pub command_id: CommandId,
    /// Stable command revision ref.
    pub command_revision_ref: CommandRevisionRef,
    /// Reviewable display label.
    pub display_label: String,
    /// Source keymap or macro register ref.
    pub source_ref: String,
    /// Write classes touched by the step.
    #[serde(default)]
    pub write_classes: Vec<MacroReplayWriteClass>,
    /// True when the step crosses files.
    pub crosses_files: bool,
    /// True when the step mutates settings or profile state.
    pub mutates_settings: bool,
    /// True when the step invokes a run-capable command.
    pub run_capable: bool,
}

impl MacroReplayCommandStep {
    fn requires_review(&self) -> bool {
        self.crosses_files
            || self.mutates_settings
            || self.run_capable
            || self
                .write_classes
                .iter()
                .any(|class| class.requires_review())
    }
}

/// One risk row shown in macro replay preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayRiskRecord {
    /// Risk class.
    pub risk_class: MacroReplayRiskClass,
    /// Reviewable risk label.
    pub label: String,
    /// Optional command id associated with the risk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<CommandId>,
}

/// Macro replay preview sheet shown before unsafe or broad replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayPreviewRecord {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ModalFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable preview id.
    pub preview_id: String,
    /// Source macro register.
    pub source_register: RegisterTargetRecord,
    /// Target replay scope.
    pub target_scope: EditTargetScope,
    /// Command steps that would replay.
    #[serde(default)]
    pub command_steps: Vec<MacroReplayCommandStep>,
    /// Register route observed for replay.
    pub register_route: RegisterRouteRecord,
    /// Risk rows.
    #[serde(default)]
    pub risks: Vec<MacroReplayRiskRecord>,
    /// Replay decision.
    pub decision: MacroReplayDecisionClass,
    /// True when the user must review before replay.
    pub review_required: bool,
    /// True when replay can dispatch without a review sheet.
    pub can_silently_proceed: bool,
    /// Promoted recipe ref when replay leaves macro scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_recipe_ref: Option<String>,
    /// Keyboard-first recovery actions.
    #[serde(default)]
    pub recovery_actions: Vec<ModalRecoveryActionRecord>,
    /// Reviewable diagnostics.
    #[serde(default)]
    pub diagnostics: Vec<String>,
    /// Guardrail: no raw macro body crosses this boundary.
    pub raw_macro_body_present: bool,
    /// Guardrail: no raw clipboard contents cross this boundary.
    pub raw_clipboard_contents_present: bool,
    /// Guardrail: no raw register contents cross this boundary.
    pub raw_register_contents_present: bool,
    /// Redaction class applied to exportable text.
    pub redaction_class: String,
    /// Timestamp at which the preview was emitted.
    pub emitted_at: String,
}

impl MacroReplayPreviewRecord {
    /// Validates this preview against macro replay safety invariants.
    pub fn validate(&self) -> MacroReplayPreviewValidationReport {
        let mut validator = MacroReplayPreviewValidator::new(self);
        validator.run();
        validator.finish()
    }
}

/// Validation report emitted for modal state snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalStateValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Snapshot id under validation.
    pub state_id: String,
    /// True when no error-severity finding failed.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ModalStateValidationCoverage,
    /// Validation findings.
    pub findings: Vec<ModalValidationFinding>,
}

/// Coverage observed while validating modal state.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ModalStateValidationCoverage {
    /// Modes observed.
    pub modes: BTreeSet<ModalMode>,
    /// Sequence states observed.
    pub sequence_states: BTreeSet<ModalSequenceState>,
    /// Register scopes observed.
    pub register_scope_kinds: BTreeSet<RegisterScopeKind>,
    /// Clipboard routes observed.
    pub clipboard_routes: BTreeSet<ClipboardRouteKind>,
    /// Surface fidelity classes observed.
    pub surface_fidelity_classes: BTreeSet<SurfaceFidelityClass>,
    /// Recovery action classes observed.
    pub recovery_action_classes: BTreeSet<ModalRecoveryActionClass>,
}

/// Validation report emitted for macro replay previews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayPreviewValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Preview id under validation.
    pub preview_id: String,
    /// True when no error-severity finding failed.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: MacroReplayPreviewValidationCoverage,
    /// Validation findings.
    pub findings: Vec<ModalValidationFinding>,
}

/// Coverage observed while validating macro replay previews.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MacroReplayPreviewValidationCoverage {
    /// Replay target scopes observed.
    pub target_scopes: BTreeSet<EditTargetScope>,
    /// Write classes observed.
    pub write_classes: BTreeSet<MacroReplayWriteClass>,
    /// Risk classes observed.
    pub risk_classes: BTreeSet<MacroReplayRiskClass>,
    /// Replay decisions observed.
    pub decisions: BTreeSet<MacroReplayDecisionClass>,
    /// Register scopes observed.
    pub register_scope_kinds: BTreeSet<RegisterScopeKind>,
    /// Clipboard routes observed.
    pub clipboard_routes: BTreeSet<ClipboardRouteKind>,
}

/// One modal validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalValidationFinding {
    /// Finding severity.
    pub severity: ModalValidationFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Reviewable message.
    pub message: String,
}

/// Modal validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalValidationFindingSeverity {
    /// Error that blocks the record.
    Error,
    /// Warning that keeps the record reviewable.
    Warning,
}

struct ModalStateValidator<'a> {
    snapshot: &'a ModalStateSnapshot,
    findings: Vec<ModalValidationFinding>,
    coverage: ModalStateValidationCoverage,
}

impl<'a> ModalStateValidator<'a> {
    fn new(snapshot: &'a ModalStateSnapshot) -> Self {
        Self {
            snapshot,
            findings: Vec::new(),
            coverage: ModalStateValidationCoverage::default(),
        }
    }

    fn run(&mut self) {
        self.validate_header();
        self.validate_mode_strip();
        self.validate_sequence_guide();
        self.validate_register_route();
        self.validate_operator_pending();
        self.validate_surface_fidelity();
        self.validate_recovery_actions();
        self.validate_raw_guards();
    }

    fn finish(self) -> ModalStateValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ModalValidationFindingSeverity::Error);
        ModalStateValidationReport {
            record_kind: MODAL_STATE_VALIDATION_RECORD_KIND.to_string(),
            schema_version: MODAL_STATE_SCHEMA_VERSION,
            state_id: self.snapshot.state_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_header(&mut self) {
        self.expect(
            self.snapshot.record_kind == MODAL_STATE_RECORD_KIND,
            "modal_state.record_kind",
            "record_kind must be modal_state_snapshot_record",
        );
        self.expect(
            self.snapshot.schema_version == MODAL_STATE_SCHEMA_VERSION,
            "modal_state.schema_version",
            "schema_version must match the crate constant",
        );
        self.expect(
            !self.snapshot.state_id.trim().is_empty()
                && !self.snapshot.redaction_class.trim().is_empty()
                && !self.snapshot.emitted_at.trim().is_empty(),
            "modal_state.required_field_missing",
            "state_id, redaction_class, and emitted_at must be non-empty",
        );
    }

    fn validate_mode_strip(&mut self) {
        let strip = &self.snapshot.mode_strip;
        self.coverage.modes.insert(strip.current_mode);
        self.expect(
            !strip.surface_ref.trim().is_empty()
                && !strip.read_write_posture_label.trim().is_empty(),
            "modal_state.mode_strip_required_field_missing",
            "mode strip must include surface_ref and read_write_posture_label",
        );
        if strip.recording_state == MacroRecordingState::Recording {
            self.expect(
                strip
                    .recording_register_name
                    .as_deref()
                    .is_some_and(|name| !name.trim().is_empty()),
                "modal_state.recording_register_missing",
                "recording mode must disclose the macro register name",
            );
        }
        if strip.current_mode == ModalMode::Unsupported {
            self.expect(
                self.snapshot
                    .surface_fidelity
                    .fidelity_class
                    .requires_disclosure()
                    && !self.snapshot.diagnostics.is_empty(),
                "modal_state.unsupported_mode_not_disclosed",
                "unsupported mode must carry narrowed-fidelity disclosure and diagnostics",
            );
        }
    }

    fn validate_sequence_guide(&mut self) {
        let Some(guide) = &self.snapshot.sequence_guide else {
            return;
        };
        self.coverage.sequence_states.insert(guide.sequence_state);
        self.expect(
            !guide.typed_prefix_display.trim().is_empty(),
            "modal_state.sequence_prefix_missing",
            "sequence guide must disclose the typed prefix",
        );
        if matches!(
            guide.sequence_state,
            ModalSequenceState::PartialWaiting
                | ModalSequenceState::AmbiguousWaiting
                | ModalSequenceState::Ready
        ) {
            self.expect(
                !guide.next_keys.is_empty(),
                "modal_state.sequence_next_keys_missing",
                "waiting or ready sequence guides must list next valid keys",
            );
        }
        if guide.sequence_state.requires_diagnostic() {
            self.expect(
                guide
                    .diagnostic_label
                    .as_deref()
                    .is_some_and(|label| !label.trim().is_empty()),
                "modal_state.sequence_diagnostic_missing",
                "blocked, unsupported, timed-out, or conflicted sequences must carry a diagnostic",
            );
        }
        for row in &guide.next_keys {
            self.expect(
                !row.key_label.trim().is_empty()
                    && !row.group_label.trim().is_empty()
                    && !row.source_ref.trim().is_empty(),
                "modal_state.sequence_next_key_required_field_missing",
                "next-key rows must include key_label, group_label, and source_ref",
            );
            if row.availability == NextKeyAvailability::Available {
                self.expect(
                    row.command_id
                        .as_deref()
                        .is_some_and(|command_id| !command_id.trim().is_empty()),
                    "modal_state.sequence_available_command_missing",
                    "available next-key rows must carry a stable command_id",
                );
            }
            if row.availability.requires_unavailable_reason() {
                self.expect(
                    row.unavailable_reason_label
                        .as_deref()
                        .is_some_and(|label| !label.trim().is_empty()),
                    "modal_state.sequence_unavailable_reason_missing",
                    "unavailable next-key rows must explain why they cannot dispatch",
                );
            }
        }
    }

    fn validate_register_route(&mut self) {
        validate_register_route_common(
            &self.snapshot.register_route,
            &mut self.coverage.register_scope_kinds,
            &mut self.coverage.clipboard_routes,
            &mut self.findings,
            "modal_state",
        );
        if self
            .snapshot
            .register_route
            .policy_state
            .requires_diagnostic()
            || self.snapshot.register_route.route_changes_result
        {
            self.expect(
                self.snapshot.recovery_actions.iter().any(|action| {
                    matches!(
                        action.action_class,
                        ModalRecoveryActionClass::ReviewRegisterRoute
                            | ModalRecoveryActionClass::OpenKeymapDiagnostics
                    )
                }),
                "modal_state.register_route_recovery_missing",
                "changed or policy-limited register routes must offer review or diagnostics",
            );
        }
    }

    fn validate_operator_pending(&mut self) {
        let Some(pending) = &self.snapshot.operator_pending else {
            return;
        };
        self.expect(
            pending.count > 0 && !pending.replay_implication_label.trim().is_empty(),
            "modal_state.operator_pending_required_field_missing",
            "operator-pending overlay must include positive count and replay implication",
        );
        if pending.crosses_files || pending.target_scope.requires_review() {
            self.expect(
                pending.review_required,
                "modal_state.operator_pending_review_missing",
                "cross-file or widened operator-pending scope must require review",
            );
        }
    }

    fn validate_surface_fidelity(&mut self) {
        self.coverage
            .surface_fidelity_classes
            .insert(self.snapshot.surface_fidelity.fidelity_class);
        if self
            .snapshot
            .surface_fidelity
            .fidelity_class
            .requires_disclosure()
        {
            self.expect(
                !self
                    .snapshot
                    .surface_fidelity
                    .narrowed_fidelity_labels
                    .is_empty(),
                "modal_state.narrowed_fidelity_label_missing",
                "narrowed modal fidelity must be labeled",
            );
        }
    }

    fn validate_recovery_actions(&mut self) {
        self.expect(
            !self.snapshot.recovery_actions.is_empty(),
            "modal_state.recovery_actions_missing",
            "modal state must expose at least one keyboard-first recovery action",
        );
        let actions = self.snapshot.recovery_actions.clone();
        for action in &actions {
            self.coverage
                .recovery_action_classes
                .insert(action.action_class);
            self.expect(
                !action.label.trim().is_empty(),
                "modal_state.recovery_action_label_missing",
                "recovery actions must carry labels",
            );
        }
    }

    fn validate_raw_guards(&mut self) {
        self.expect(
            !self.snapshot.raw_key_history_present,
            "modal_state.raw_key_history_present",
            "modal snapshots must not export raw key history",
        );
        self.expect(
            !self.snapshot.raw_clipboard_contents_present,
            "modal_state.raw_clipboard_contents_present",
            "modal snapshots must not export raw clipboard contents",
        );
        self.expect(
            !self.snapshot.raw_register_contents_present,
            "modal_state.raw_register_contents_present",
            "modal snapshots must not export raw register contents",
        );
    }

    fn expect(&mut self, condition: bool, check_id: &str, message: &str) {
        if !condition {
            self.findings.push(ModalValidationFinding {
                severity: ModalValidationFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

struct MacroReplayPreviewValidator<'a> {
    preview: &'a MacroReplayPreviewRecord,
    findings: Vec<ModalValidationFinding>,
    coverage: MacroReplayPreviewValidationCoverage,
}

impl<'a> MacroReplayPreviewValidator<'a> {
    fn new(preview: &'a MacroReplayPreviewRecord) -> Self {
        Self {
            preview,
            findings: Vec::new(),
            coverage: MacroReplayPreviewValidationCoverage::default(),
        }
    }

    fn run(&mut self) {
        self.validate_header();
        self.validate_source_register();
        self.validate_steps_and_risks();
        self.validate_register_route();
        self.validate_decision();
        self.validate_recovery_actions();
        self.validate_raw_guards();
    }

    fn finish(self) -> MacroReplayPreviewValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ModalValidationFindingSeverity::Error);
        MacroReplayPreviewValidationReport {
            record_kind: MACRO_REPLAY_PREVIEW_VALIDATION_RECORD_KIND.to_string(),
            schema_version: MACRO_REPLAY_PREVIEW_SCHEMA_VERSION,
            preview_id: self.preview.preview_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_header(&mut self) {
        self.expect(
            self.preview.record_kind == MACRO_REPLAY_PREVIEW_RECORD_KIND,
            "macro_replay_preview.record_kind",
            "record_kind must be macro_replay_preview_record",
        );
        self.expect(
            self.preview.schema_version == MACRO_REPLAY_PREVIEW_SCHEMA_VERSION,
            "macro_replay_preview.schema_version",
            "schema_version must match the crate constant",
        );
        self.expect(
            !self.preview.preview_id.trim().is_empty()
                && !self.preview.redaction_class.trim().is_empty()
                && !self.preview.emitted_at.trim().is_empty(),
            "macro_replay_preview.required_field_missing",
            "preview_id, redaction_class, and emitted_at must be non-empty",
        );
        self.coverage
            .target_scopes
            .insert(self.preview.target_scope);
        self.coverage.decisions.insert(self.preview.decision);
    }

    fn validate_source_register(&mut self) {
        self.coverage
            .register_scope_kinds
            .insert(self.preview.source_register.scope_kind);
        self.expect(
            self.preview.source_register.scope_kind == RegisterScopeKind::Macro,
            "macro_replay_preview.source_register_not_macro",
            "macro replay preview must identify a macro register as the source",
        );
        self.expect(
            !self.preview.source_register.display_label.trim().is_empty(),
            "macro_replay_preview.source_register_label_missing",
            "source register must carry a display label",
        );
    }

    fn validate_steps_and_risks(&mut self) {
        self.expect(
            !self.preview.command_steps.is_empty(),
            "macro_replay_preview.steps_missing",
            "macro replay preview must list command steps",
        );
        for step in &self.preview.command_steps {
            self.expect(
                !step.step_id.trim().is_empty()
                    && !step.command_id.trim().is_empty()
                    && !step.command_revision_ref.trim().is_empty()
                    && !step.display_label.trim().is_empty()
                    && !step.source_ref.trim().is_empty(),
                "macro_replay_preview.step_required_field_missing",
                "macro replay steps must include ids, display label, and source ref",
            );
            self.expect(
                !step.write_classes.is_empty(),
                "macro_replay_preview.step_write_classes_missing",
                "macro replay steps must declare write classes",
            );
            for class in &step.write_classes {
                self.coverage.write_classes.insert(*class);
            }
        }
        for risk in &self.preview.risks {
            self.coverage.risk_classes.insert(risk.risk_class);
            self.expect(
                !risk.label.trim().is_empty(),
                "macro_replay_preview.risk_label_missing",
                "risk rows must carry labels",
            );
        }
        if self
            .preview
            .risks
            .iter()
            .any(|risk| risk.risk_class == MacroReplayRiskClass::UnsupportedImportedSequence)
        {
            self.expect(
                self.preview.decision == MacroReplayDecisionClass::DeniedUnsafeReplay
                    && !self.preview.diagnostics.is_empty(),
                "macro_replay_preview.unsupported_import_not_denied",
                "unsupported imported sequences must fail closed with diagnostics",
            );
        }
    }

    fn validate_register_route(&mut self) {
        validate_register_route_common(
            &self.preview.register_route,
            &mut self.coverage.register_scope_kinds,
            &mut self.coverage.clipboard_routes,
            &mut self.findings,
            "macro_replay_preview",
        );
    }

    fn validate_decision(&mut self) {
        let step_requires_review = self
            .preview
            .command_steps
            .iter()
            .any(MacroReplayCommandStep::requires_review);
        let widened_scope = self.preview.target_scope.requires_review();
        let register_requires_review = self.preview.register_route.route_changes_result
            || self
                .preview
                .register_route
                .policy_state
                .requires_diagnostic();
        let must_review = step_requires_review
            || widened_scope
            || register_requires_review
            || !self.preview.risks.is_empty();

        if must_review {
            self.expect(
                self.preview.review_required,
                "macro_replay_preview.review_required_missing",
                "widened, policy-limited, or risky replay must require review",
            );
            self.expect(
                self.preview.decision != MacroReplayDecisionClass::ProceedLocalEditorOnly,
                "macro_replay_preview.silent_widening_denied",
                "widened, policy-limited, or risky replay must not silently proceed",
            );
            self.expect(
                !self.preview.can_silently_proceed,
                "macro_replay_preview.can_silently_proceed_invalid",
                "widened, policy-limited, or risky replay cannot silently proceed",
            );
        } else {
            self.expect(
                self.preview.decision == MacroReplayDecisionClass::ProceedLocalEditorOnly
                    && self.preview.can_silently_proceed,
                "macro_replay_preview.safe_replay_not_proceeding",
                "single-buffer macro replay without risks should use the silent proceed lane",
            );
        }

        if self.preview.decision == MacroReplayDecisionClass::PromoteToRecipe {
            self.expect(
                self.preview
                    .promoted_recipe_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()),
                "macro_replay_preview.promoted_recipe_missing",
                "recipe promotion decisions must cite a promoted_recipe_ref",
            );
        } else {
            self.expect(
                self.preview.promoted_recipe_ref.is_none(),
                "macro_replay_preview.promoted_recipe_unexpected",
                "non-promotion decisions must not cite a promoted_recipe_ref",
            );
        }
        if self.preview.decision == MacroReplayDecisionClass::DeniedUnsafeReplay {
            self.expect(
                !self.preview.diagnostics.is_empty(),
                "macro_replay_preview.denial_diagnostic_missing",
                "denied replay decisions must carry diagnostics",
            );
        }
    }

    fn validate_recovery_actions(&mut self) {
        if self.preview.decision != MacroReplayDecisionClass::ProceedLocalEditorOnly {
            self.expect(
                self.preview.recovery_actions.iter().any(|action| {
                    matches!(
                        action.action_class,
                        ModalRecoveryActionClass::ReviewMacroReplay
                            | ModalRecoveryActionClass::CancelSequence
                            | ModalRecoveryActionClass::OpenCommandPalette
                    )
                }),
                "macro_replay_preview.recovery_action_missing",
                "non-proceed replay decisions must offer review, cancel, or palette recovery",
            );
        }
    }

    fn validate_raw_guards(&mut self) {
        self.expect(
            !self.preview.raw_macro_body_present,
            "macro_replay_preview.raw_macro_body_present",
            "macro replay preview must not export raw macro bodies",
        );
        self.expect(
            !self.preview.raw_clipboard_contents_present,
            "macro_replay_preview.raw_clipboard_contents_present",
            "macro replay preview must not export raw clipboard contents",
        );
        self.expect(
            !self.preview.raw_register_contents_present,
            "macro_replay_preview.raw_register_contents_present",
            "macro replay preview must not export raw register contents",
        );
    }

    fn expect(&mut self, condition: bool, check_id: &str, message: &str) {
        if !condition {
            self.findings.push(ModalValidationFinding {
                severity: ModalValidationFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

fn validate_register_route_common(
    route: &RegisterRouteRecord,
    register_scope_kinds: &mut BTreeSet<RegisterScopeKind>,
    clipboard_routes: &mut BTreeSet<ClipboardRouteKind>,
    findings: &mut Vec<ModalValidationFinding>,
    prefix: &str,
) {
    register_scope_kinds.insert(route.active_register.scope_kind);
    clipboard_routes.insert(route.clipboard_route);
    let mut expect = |condition: bool, check_suffix: &str, message: &str| {
        if !condition {
            findings.push(ModalValidationFinding {
                severity: ModalValidationFindingSeverity::Error,
                check_id: format!("{prefix}.{check_suffix}"),
                message: message.to_string(),
            });
        }
    };
    expect(
        !route.active_register.display_label.trim().is_empty()
            && !route.route_label.trim().is_empty(),
        "register_route_required_field_missing",
        "register routes must include active register and route labels",
    );
    if matches!(
        route.active_register.scope_kind,
        RegisterScopeKind::NamedRegister
            | RegisterScopeKind::Macro
            | RegisterScopeKind::SearchHistory
    ) {
        expect(
            route
                .active_register
                .register_name
                .as_deref()
                .is_some_and(|name| !name.trim().is_empty()),
            "register_name_missing",
            "named, macro, or search register routes must disclose register_name",
        );
    }
    if route.policy_state.requires_diagnostic() || route.route_changes_result {
        expect(
            !route.diagnostics.is_empty(),
            "register_route_diagnostic_missing",
            "policy-limited or changed register routes must carry diagnostics",
        );
    }
    if matches!(
        route.clipboard_route,
        ClipboardRouteKind::RemoteBridgeSuppressed
            | ClipboardRouteKind::AdminBlocked
            | ClipboardRouteKind::UnsupportedOnSurface
    ) {
        expect(
            route.policy_state != RegisterPolicyState::Allowed,
            "register_route_policy_state_inconsistent",
            "suppressed, blocked, or unsupported clipboard routes cannot be policy-allowed",
        );
    }
}
