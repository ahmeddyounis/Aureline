//! Macro-recorder session and replay object and its first consumers.
//!
//! The recipe-and-macro contract
//! ([`/docs/m5/recipe-builder-and-macro-contract.md`]) and the frozen
//! macro-session boundary schema
//! ([`/schemas/automation/macro-session.schema.json`]) froze *what* a recorded
//! macro is: a deliberately narrow capture of UI or editor state, projected only
//! the `macro_safe` and `ui_only` safety labels, never admissible on the
//! managed-only channel. This module makes the recorder session model concrete: a
//! live [`MacroRecorderSession`] that captures one editing flow as a reviewable,
//! profile-local macro, an [`ActiveRecordingStrip`] the recorder renders while it
//! captures, a [`CapturedCommandReview`] the user inspects before they save or
//! discard, and a [`MacroRecorderSession::resolve_replay`] that resolves replay
//! **against the current context, the declared target scope, and the
//! supported-command set every time** — never against the scope the macro was
//! captured under.
//!
//! The session never asserts that a recording carries authority on its own.
//! [`MacroRecorderSession::resolved_replay_class`] derives the replay action from
//! the session's repository-import state and the [`MacroReplayBlocker`]s the
//! resolver observed *now*, so a macro **fails closed** the moment the active
//! document, the declared target scope, or the supported-command set no longer
//! matches what the macro needs. A macro that crosses files, commands, or
//! side-effect classes is never silently replayed with a widened reach: it carries
//! [`MacroReplayBlocker::PromotionRequiredCrossesScope`] and must be promoted to a
//! declarative recipe, an explicit step the user takes — never a silent forward.
//! Repository content can never define an executable macro: an imported recording
//! always resolves to [`MacroReplayActionClass::BlockedImportedFromRepositoryContent`].
//!
//! [`MacroRecorderFirstConsumersPacket`] binds the first M5 automation families that
//! render a macro recorder — notebook, task/test/debug, request/API, package,
//! incident, and the AI assistant — each to a seeded panel of sessions, and
//! [`MacroRecorderFirstConsumersPacket::validate`] enforces the freeze mechanically:
//! every entrypoint binds a non-empty panel, every session declares its target and
//! storage scope, recorded macros stay profile-local, an unsupported command is
//! flagged and blocks save, replay fails closed when context or scope no longer
//! matches, repository content never defines a macro, promotion to a recipe is
//! explicit when a macro crosses scope, and every session captures UI or editor
//! state only. A dropped entrypoint, an empty panel, a replay that implies stale
//! context, an unsupported command that does not block save, a repository-imported
//! macro, a non-explicit cross-scope promotion, an ambient or managed-only capture,
//! an inconsistent replay projection, or a violated invariant *blocks stable*.
//!
//! The reviewer-facing landing page is
//! [`/docs/m5/macro-recorder-and-replay.md`]; the cross-tool boundary schema for
//! the first-consumers packet is
//! [`/schemas/automation/macro-recorder.schema.json`]; the frozen macro-session and
//! recipe-manifest boundary schemas it reuses are named in
//! [`canonical_reused_contract_refs`].
//!
//! [`/docs/m5/recipe-builder-and-macro-contract.md`]: ../../../docs/m5/recipe-builder-and-macro-contract.md
//! [`/docs/m5/macro-recorder-and-replay.md`]: ../../../docs/m5/macro-recorder-and-replay.md
//! [`/schemas/automation/macro-session.schema.json`]: ../../../schemas/automation/macro-session.schema.json
//! [`/schemas/automation/macro-recorder.schema.json`]: ../../../schemas/automation/macro-recorder.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    AutomationBaselinePromotionState, AutomationSafetyLabelId, ContentAddress, MacroCaptureStep,
    MacroPromotionAffordanceClass, MacroRecorderStateClass, MacroSession,
    AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF, CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
    MACRO_SESSION_SCHEMA_REF, RECIPE_MANIFEST_SCHEMA_REF, RUN_RECORD_SCHEMA_REF,
};
use crate::recipe_builder::RecipeBuilderEntrypoint;

/// Stable record-kind tag for [`MacroRecorderFirstConsumersPacket`].
pub const MACRO_RECORDER_FIRST_CONSUMERS_RECORD_KIND: &str =
    "m5_macro_recorder_first_consumers_packet";

/// Stable record-kind tag for [`MacroRecorderFirstConsumersSupportExport`].
pub const MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_macro_recorder_first_consumers_support_export";

/// Stable record-kind tag for [`MacroRecorderFirstConsumersCliHeadlessView`].
pub const MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_macro_recorder_first_consumers_cli_headless";

/// Stable record-kind tag for [`MacroSessionExport`].
pub const MACRO_SESSION_EXPORT_RECORD_KIND: &str = "macro_session_export_record";

/// Stable record-kind tag for [`MacroReplayResolution`].
pub const MACRO_REPLAY_RESOLUTION_RECORD_KIND: &str = "macro_replay_resolution";

/// Integer schema version for the macro-recorder first-consumers family.
pub const MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/automation/macro-recorder.schema.json";

/// Repo-relative path of the reviewer contract doc for the macro-recorder lane.
pub const MACRO_RECORDER_DOC_REF: &str = "docs/m5/macro-recorder-and-replay.md";

/// Repo-relative path of the checked-in first-consumers packet artifact.
pub const MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/macro-recorder/packet.json";

/// Repo-relative root the worked-example macro-recorder fixtures live under.
pub const MACRO_RECORDER_FIXTURE_DIR: &str = "fixtures/automation/m5/macro-recorder";

/// Stable packet id minted by the seed.
pub const MACRO_RECORDER_FIRST_CONSUMERS_ID: &str =
    "automation:m5:macro-recorder-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:macro-recorder-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:macro-recorder-first-consumers";

// ---------------------------------------------------------------------------
// Recorded surface and replay posture
// ---------------------------------------------------------------------------

/// The UI or editor surface one captured command quotes.
///
/// The set is the closed `recorded_macro_surface_class` vocabulary frozen in the
/// macro-session schema; minting a parallel surface (a shell command, a network
/// fetch, a credential read) is non-conforming and is the schema's enforcement that
/// a recorded macro is constrained to explicit UI or editor state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedSurfaceClass {
    /// Editor document state.
    EditorDocumentState,
    /// Editor selection and cursor state.
    EditorSelectionAndCursorState,
    /// Editor multi-cursor edits.
    EditorMultiCursorEdits,
    /// Editor find-and-replace state.
    EditorFindAndReplaceState,
    /// Command-palette selection state.
    CommandPaletteSelectionState,
    /// UI panel open/close state.
    UiPanelOpenCloseState,
    /// UI focus-move state.
    UiFocusMoveState,
    /// UI layout-reshape state.
    UiLayoutReshapeState,
    /// Keybinding chord replay state.
    KeybindingChordReplayState,
}

impl RecordedSurfaceClass {
    /// Stable snake_case token, identical to the schema's surface vocabulary.
    pub fn as_str(self) -> &'static str {
        match self {
            RecordedSurfaceClass::EditorDocumentState => "editor_document_state",
            RecordedSurfaceClass::EditorSelectionAndCursorState => {
                "editor_selection_and_cursor_state"
            }
            RecordedSurfaceClass::EditorMultiCursorEdits => "editor_multi_cursor_edits",
            RecordedSurfaceClass::EditorFindAndReplaceState => "editor_find_and_replace_state",
            RecordedSurfaceClass::CommandPaletteSelectionState => "command_palette_selection_state",
            RecordedSurfaceClass::UiPanelOpenCloseState => "ui_panel_open_close_state",
            RecordedSurfaceClass::UiFocusMoveState => "ui_focus_move_state",
            RecordedSurfaceClass::UiLayoutReshapeState => "ui_layout_reshape_state",
            RecordedSurfaceClass::KeybindingChordReplayState => "keybinding_chord_replay_state",
        }
    }
}

/// The replay posture one captured command admits.
///
/// Every captured step is strictly UI or editor state; invoking a command
/// descriptor from a macro step is non-conforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPostureClass {
    /// Replay UI or editor state only.
    ReplayUiOrEditorStateOnly,
    /// Replay a keybinding chord only.
    ReplayKeybindingChordOnly,
    /// Replay a command-palette selection only.
    ReplayCommandPaletteSelectionOnly,
}

impl ReplayPostureClass {
    /// Stable snake_case token, identical to the schema's replay-posture vocabulary.
    pub fn as_str(self) -> &'static str {
        match self {
            ReplayPostureClass::ReplayUiOrEditorStateOnly => "replay_ui_or_editor_state_only",
            ReplayPostureClass::ReplayKeybindingChordOnly => "replay_keybinding_chord_only",
            ReplayPostureClass::ReplayCommandPaletteSelectionOnly => {
                "replay_command_palette_selection_only"
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Captured-command support
// ---------------------------------------------------------------------------

/// Whether a captured command is supported in a recorded macro, and if not, why.
///
/// A recorded macro is constrained to UI or editor state. A command that runs a
/// process, performs a network call, mutates remote state, writes files, reads a
/// secret, requires an approval, or hands off to an external runner is **not**
/// recordable as a macro step: the recorder flags it as an unsupported-command
/// warning, and an unsupported command blocks save. Such a flow belongs in a
/// declarative recipe, not a macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedCommandSupportClass {
    /// Supported: the command quotes UI or editor state only.
    SupportedUiOrEditorState,
    /// Unsupported: the command launches or controls a process.
    UnsupportedRunsProcess,
    /// Unsupported: the command performs a network call.
    UnsupportedNetworkCall,
    /// Unsupported: the command mutates remote state.
    UnsupportedRemoteMutation,
    /// Unsupported: the command writes files in the workspace or on the device.
    UnsupportedWritesFiles,
    /// Unsupported: the command reads a secret.
    UnsupportedSecretRead,
    /// Unsupported: the command requires an approval ticket before any apply.
    UnsupportedRequiresApproval,
    /// Unsupported: the command hands off to an extension or external runner.
    UnsupportedExternalRunner,
}

impl CapturedCommandSupportClass {
    /// Every support class in canonical order.
    pub const ALL: [CapturedCommandSupportClass; 8] = [
        CapturedCommandSupportClass::SupportedUiOrEditorState,
        CapturedCommandSupportClass::UnsupportedRunsProcess,
        CapturedCommandSupportClass::UnsupportedNetworkCall,
        CapturedCommandSupportClass::UnsupportedRemoteMutation,
        CapturedCommandSupportClass::UnsupportedWritesFiles,
        CapturedCommandSupportClass::UnsupportedSecretRead,
        CapturedCommandSupportClass::UnsupportedRequiresApproval,
        CapturedCommandSupportClass::UnsupportedExternalRunner,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            CapturedCommandSupportClass::SupportedUiOrEditorState => "supported_ui_or_editor_state",
            CapturedCommandSupportClass::UnsupportedRunsProcess => "unsupported_runs_process",
            CapturedCommandSupportClass::UnsupportedNetworkCall => "unsupported_network_call",
            CapturedCommandSupportClass::UnsupportedRemoteMutation => "unsupported_remote_mutation",
            CapturedCommandSupportClass::UnsupportedWritesFiles => "unsupported_writes_files",
            CapturedCommandSupportClass::UnsupportedSecretRead => "unsupported_secret_read",
            CapturedCommandSupportClass::UnsupportedRequiresApproval => {
                "unsupported_requires_approval"
            }
            CapturedCommandSupportClass::UnsupportedExternalRunner => "unsupported_external_runner",
        }
    }

    /// Whether the command is supported as a recorded-macro step.
    pub fn is_supported(self) -> bool {
        matches!(self, CapturedCommandSupportClass::SupportedUiOrEditorState)
    }
}

// ---------------------------------------------------------------------------
// Target scope
// ---------------------------------------------------------------------------

/// The scope a recorded macro declares it replays against.
///
/// A macro that stays within the active document, selection, or editor group is a
/// profile-local recording. A macro whose declared scope crosses files
/// ([`TargetScopeClass::MultiFileScope`]) or spans the workspace
/// ([`TargetScopeClass::WorkspaceScope`]) needs broader review: it must be promoted
/// to a declarative recipe rather than silently replayed with a widened reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetScopeClass {
    /// The active document only.
    ActiveDocumentScope,
    /// The active selection only.
    ActiveSelectionScope,
    /// The active editor group only.
    ActiveEditorGroupScope,
    /// A single named file.
    SingleFileScope,
    /// Multiple files (crosses files, needs promotion).
    MultiFileScope,
    /// The whole workspace (needs promotion).
    WorkspaceScope,
}

impl TargetScopeClass {
    /// Every target scope in canonical order.
    pub const ALL: [TargetScopeClass; 6] = [
        TargetScopeClass::ActiveDocumentScope,
        TargetScopeClass::ActiveSelectionScope,
        TargetScopeClass::ActiveEditorGroupScope,
        TargetScopeClass::SingleFileScope,
        TargetScopeClass::MultiFileScope,
        TargetScopeClass::WorkspaceScope,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            TargetScopeClass::ActiveDocumentScope => "active_document_scope",
            TargetScopeClass::ActiveSelectionScope => "active_selection_scope",
            TargetScopeClass::ActiveEditorGroupScope => "active_editor_group_scope",
            TargetScopeClass::SingleFileScope => "single_file_scope",
            TargetScopeClass::MultiFileScope => "multi_file_scope",
            TargetScopeClass::WorkspaceScope => "workspace_scope",
        }
    }

    /// Whether the scope crosses files and so requires explicit recipe promotion.
    pub fn requires_promotion(self) -> bool {
        matches!(
            self,
            TargetScopeClass::MultiFileScope | TargetScopeClass::WorkspaceScope
        )
    }
}

// ---------------------------------------------------------------------------
// Storage scope
// ---------------------------------------------------------------------------

/// Where a recorded macro is stored.
///
/// Recorded macros are profile-local by default
/// ([`MacroStorageScopeClass::UserScopeLocalOnly`] /
/// [`MacroStorageScopeClass::WorkspaceScopeLocalOnly`]); the portable and support
/// export scopes are explicit, user-initiated exports. The organization /
/// managed-only channel is **not** in this vocabulary: an
/// organization-distributed automation must be a declarative recipe, never a macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroStorageScopeClass {
    /// Stored in the user profile, local only (the default).
    UserScopeLocalOnly,
    /// Stored in the workspace, local only.
    WorkspaceScopeLocalOnly,
    /// Exported as a portable profile artifact only (explicit user action).
    PortableProfileExportOnly,
    /// Exported into a support bundle only (explicit user action).
    SupportBundleExportOnly,
}

impl MacroStorageScopeClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroStorageScopeClass::UserScopeLocalOnly => "user_scope_local_only",
            MacroStorageScopeClass::WorkspaceScopeLocalOnly => "workspace_scope_local_only",
            MacroStorageScopeClass::PortableProfileExportOnly => "portable_profile_export_only",
            MacroStorageScopeClass::SupportBundleExportOnly => "support_bundle_export_only",
        }
    }

    /// Whether this is a profile-local resident storage scope (the default).
    pub fn is_local_only(self) -> bool {
        matches!(
            self,
            MacroStorageScopeClass::UserScopeLocalOnly
                | MacroStorageScopeClass::WorkspaceScopeLocalOnly
        )
    }
}

// ---------------------------------------------------------------------------
// Redaction class
// ---------------------------------------------------------------------------

/// The redaction mode a macro session's safe summary carries.
///
/// A recorded macro is UI/editor state only, so the default is the metadata-safe
/// floor; an export may still require redaction before the session crosses a tenant
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRedactionClass {
    /// The metadata-safe default redaction floor.
    MetadataSafeDefault,
    /// Redaction is required before the session crosses an export boundary.
    RedactionRequiredOnExport,
}

impl MacroRedactionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroRedactionClass::MetadataSafeDefault => "metadata_safe_default",
            MacroRedactionClass::RedactionRequiredOnExport => "redaction_required_on_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Session disposition
// ---------------------------------------------------------------------------

/// The save-or-discard disposition a recorder session resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDispositionClass {
    /// Still recording or paused; no disposition yet.
    StillRecording,
    /// Saved as a profile-local recorded macro.
    SavedAsProfileLocalMacro,
    /// Saved and promoted to a declarative recipe (explicit cross-scope promotion).
    SavedAndPromotedToRecipe,
    /// Discarded; no macro manifest was minted.
    DiscardedNoMacroMinted,
}

impl SessionDispositionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            SessionDispositionClass::StillRecording => "still_recording",
            SessionDispositionClass::SavedAsProfileLocalMacro => "saved_as_profile_local_macro",
            SessionDispositionClass::SavedAndPromotedToRecipe => "saved_and_promoted_to_recipe",
            SessionDispositionClass::DiscardedNoMacroMinted => "discarded_no_macro_minted",
        }
    }

    /// Whether this disposition mints a recorded-macro manifest.
    pub fn mints_manifest(self) -> bool {
        matches!(
            self,
            SessionDispositionClass::SavedAsProfileLocalMacro
                | SessionDispositionClass::SavedAndPromotedToRecipe
        )
    }
}

// ---------------------------------------------------------------------------
// Replay blocker and disposition
// ---------------------------------------------------------------------------

/// The disposition a [`MacroReplayBlocker`] forces on replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacroReplayDisposition {
    /// Replay is admissible in the declared scope with no reconciliation.
    AdmissibleInScope,
    /// Replay is admissible only after an explicit scope reconciliation.
    RequiresScopeReconciliation,
    /// Replay fails closed until the blocker clears.
    FailsClosed,
}

/// One blocker the replay resolver observed at projection time.
///
/// The blocker list is the authoritative reason a macro would not replay *today*.
/// [`MacroReplayBlocker::NoBlockerPresent`] is the only blocker that pairs with
/// [`MacroReplayActionClass::ReplayInDeclaredScope`]; every other replay class cites
/// at least one non-no-blocker entry. A recorded macro **fails closed**: any context
/// drift the user has not explicitly reconciled, any change to the declared target
/// scope or the supported-command set, and any repository-import refuses replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayBlocker {
    /// No blocker is present; replay is admissible in the declared scope.
    NoBlockerPresent,
    /// The active context drifted but can be explicitly reconciled before replay.
    ActiveContextReconcilable,
    /// The declared target scope no longer matches the current context.
    TargetScopeNoLongerMatches,
    /// The active document or selection drifted from the captured state.
    ActiveDocumentOrSelectionDrift,
    /// The supported-command set the macro needs has changed.
    SupportedCommandSetChanged,
    /// The macro captured an unsupported command and is not safely replayable.
    UnsupportedCommandCaptured,
    /// The macro's resident profile / workspace scope no longer matches.
    ProfileScopeMismatch,
    /// The macro crosses scope and must be promoted to a recipe before replay.
    PromotionRequiredCrossesScope,
    /// The kill switch is engaged.
    KillSwitchEngaged,
    /// Replay is disabled by admin policy.
    ReplayDisabledByPolicy,
    /// The macro revision has been retired.
    MacroRevisionRetired,
    /// The macro was imported from repository content and is never auto-replayable.
    ImportedFromRepositoryContent,
}

impl MacroReplayBlocker {
    /// Every blocker in canonical order.
    pub const ALL: [MacroReplayBlocker; 12] = [
        MacroReplayBlocker::NoBlockerPresent,
        MacroReplayBlocker::ActiveContextReconcilable,
        MacroReplayBlocker::TargetScopeNoLongerMatches,
        MacroReplayBlocker::ActiveDocumentOrSelectionDrift,
        MacroReplayBlocker::SupportedCommandSetChanged,
        MacroReplayBlocker::UnsupportedCommandCaptured,
        MacroReplayBlocker::ProfileScopeMismatch,
        MacroReplayBlocker::PromotionRequiredCrossesScope,
        MacroReplayBlocker::KillSwitchEngaged,
        MacroReplayBlocker::ReplayDisabledByPolicy,
        MacroReplayBlocker::MacroRevisionRetired,
        MacroReplayBlocker::ImportedFromRepositoryContent,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroReplayBlocker::NoBlockerPresent => "no_blocker_present",
            MacroReplayBlocker::ActiveContextReconcilable => "active_context_reconcilable",
            MacroReplayBlocker::TargetScopeNoLongerMatches => "target_scope_no_longer_matches",
            MacroReplayBlocker::ActiveDocumentOrSelectionDrift => {
                "active_document_or_selection_drift"
            }
            MacroReplayBlocker::SupportedCommandSetChanged => "supported_command_set_changed",
            MacroReplayBlocker::UnsupportedCommandCaptured => "unsupported_command_captured",
            MacroReplayBlocker::ProfileScopeMismatch => "profile_scope_mismatch",
            MacroReplayBlocker::PromotionRequiredCrossesScope => "promotion_required_crosses_scope",
            MacroReplayBlocker::KillSwitchEngaged => "kill_switch_engaged",
            MacroReplayBlocker::ReplayDisabledByPolicy => "replay_disabled_by_policy",
            MacroReplayBlocker::MacroRevisionRetired => "macro_revision_retired",
            MacroReplayBlocker::ImportedFromRepositoryContent => "imported_from_repository_content",
        }
    }

    /// The replay disposition this blocker forces.
    pub fn disposition(self) -> MacroReplayDisposition {
        match self {
            MacroReplayBlocker::NoBlockerPresent => MacroReplayDisposition::AdmissibleInScope,
            MacroReplayBlocker::ActiveContextReconcilable => {
                MacroReplayDisposition::RequiresScopeReconciliation
            }
            _ => MacroReplayDisposition::FailsClosed,
        }
    }

    /// The replay action class this blocker maps to one-to-one.
    pub fn replay_action_class(self) -> MacroReplayActionClass {
        match self {
            MacroReplayBlocker::NoBlockerPresent => MacroReplayActionClass::ReplayInDeclaredScope,
            MacroReplayBlocker::ActiveContextReconcilable => {
                MacroReplayActionClass::AdmissibleAfterScopeReconciliation
            }
            MacroReplayBlocker::TargetScopeNoLongerMatches => {
                MacroReplayActionClass::BlockedTargetScopeMismatch
            }
            MacroReplayBlocker::ActiveDocumentOrSelectionDrift => {
                MacroReplayActionClass::BlockedActiveContextDrift
            }
            MacroReplayBlocker::SupportedCommandSetChanged => {
                MacroReplayActionClass::BlockedSupportedSetChanged
            }
            MacroReplayBlocker::UnsupportedCommandCaptured => {
                MacroReplayActionClass::BlockedUnsupportedCommand
            }
            MacroReplayBlocker::ProfileScopeMismatch => {
                MacroReplayActionClass::BlockedProfileScopeMismatch
            }
            MacroReplayBlocker::PromotionRequiredCrossesScope => {
                MacroReplayActionClass::BlockedPromotionRequired
            }
            MacroReplayBlocker::KillSwitchEngaged => MacroReplayActionClass::BlockedKillSwitch,
            MacroReplayBlocker::ReplayDisabledByPolicy => MacroReplayActionClass::BlockedByPolicy,
            MacroReplayBlocker::MacroRevisionRetired => {
                MacroReplayActionClass::BlockedRevisionRetired
            }
            MacroReplayBlocker::ImportedFromRepositoryContent => {
                MacroReplayActionClass::BlockedImportedFromRepositoryContent
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Replay action class
// ---------------------------------------------------------------------------

/// The resolved replay-under-current-context action a macro session offers.
///
/// The class is derived from the session's repository-import state and the
/// [`MacroReplayBlocker`]s observed now — never from the scope the macro was
/// captured under. Every blocked class refuses replay: a recorded macro fails closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MacroReplayActionClass {
    /// Replay is admissible today in the declared scope with no reconciliation.
    #[serde(rename = "macro_replay_admissible_in_declared_scope")]
    ReplayInDeclaredScope,
    /// Replay is admissible after an explicit scope reconciliation.
    #[serde(rename = "macro_replay_admissible_after_scope_reconciliation")]
    AdmissibleAfterScopeReconciliation,
    /// Replay is blocked: the declared target scope no longer matches.
    #[serde(rename = "macro_replay_blocked_target_scope_mismatch")]
    BlockedTargetScopeMismatch,
    /// Replay is blocked: the active document or selection drifted.
    #[serde(rename = "macro_replay_blocked_active_context_drift")]
    BlockedActiveContextDrift,
    /// Replay is blocked: the supported-command set changed.
    #[serde(rename = "macro_replay_blocked_supported_command_set_changed")]
    BlockedSupportedSetChanged,
    /// Replay is blocked: the macro captured an unsupported command.
    #[serde(rename = "macro_replay_blocked_unsupported_command_captured")]
    BlockedUnsupportedCommand,
    /// Replay is blocked: the macro's profile / workspace scope no longer matches.
    #[serde(rename = "macro_replay_blocked_profile_scope_mismatch")]
    BlockedProfileScopeMismatch,
    /// Replay is blocked: the macro crosses scope and must be promoted to a recipe.
    #[serde(rename = "macro_replay_blocked_promotion_required_crosses_scope")]
    BlockedPromotionRequired,
    /// Replay is blocked: the kill switch is engaged.
    #[serde(rename = "macro_replay_blocked_kill_switch_engaged")]
    BlockedKillSwitch,
    /// Replay is blocked: replay is disabled by policy.
    #[serde(rename = "macro_replay_blocked_disabled_by_policy")]
    BlockedByPolicy,
    /// Replay is blocked: the macro revision was retired.
    #[serde(rename = "macro_replay_blocked_revision_retired")]
    BlockedRevisionRetired,
    /// Replay is blocked: the macro was imported from repository content.
    #[serde(rename = "macro_replay_blocked_imported_from_repository_content")]
    BlockedImportedFromRepositoryContent,
}

impl MacroReplayActionClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroReplayActionClass::ReplayInDeclaredScope => {
                "macro_replay_admissible_in_declared_scope"
            }
            MacroReplayActionClass::AdmissibleAfterScopeReconciliation => {
                "macro_replay_admissible_after_scope_reconciliation"
            }
            MacroReplayActionClass::BlockedTargetScopeMismatch => {
                "macro_replay_blocked_target_scope_mismatch"
            }
            MacroReplayActionClass::BlockedActiveContextDrift => {
                "macro_replay_blocked_active_context_drift"
            }
            MacroReplayActionClass::BlockedSupportedSetChanged => {
                "macro_replay_blocked_supported_command_set_changed"
            }
            MacroReplayActionClass::BlockedUnsupportedCommand => {
                "macro_replay_blocked_unsupported_command_captured"
            }
            MacroReplayActionClass::BlockedProfileScopeMismatch => {
                "macro_replay_blocked_profile_scope_mismatch"
            }
            MacroReplayActionClass::BlockedPromotionRequired => {
                "macro_replay_blocked_promotion_required_crosses_scope"
            }
            MacroReplayActionClass::BlockedKillSwitch => "macro_replay_blocked_kill_switch_engaged",
            MacroReplayActionClass::BlockedByPolicy => "macro_replay_blocked_disabled_by_policy",
            MacroReplayActionClass::BlockedRevisionRetired => {
                "macro_replay_blocked_revision_retired"
            }
            MacroReplayActionClass::BlockedImportedFromRepositoryContent => {
                "macro_replay_blocked_imported_from_repository_content"
            }
        }
    }

    /// Whether the class admits replay (in scope, or after reconciliation).
    pub fn is_admissible(self) -> bool {
        matches!(
            self,
            MacroReplayActionClass::ReplayInDeclaredScope
                | MacroReplayActionClass::AdmissibleAfterScopeReconciliation
        )
    }

    /// Whether the class fails closed (refuses replay until the blocker clears).
    pub fn is_fail_closed(self) -> bool {
        !self.is_admissible()
    }
}

// ---------------------------------------------------------------------------
// Captured command
// ---------------------------------------------------------------------------

/// One captured command in a recorder session.
///
/// A captured command is strictly UI or editor state: it carries an opaque command
/// id, the surface it quotes, its support class, its replay posture, the
/// content-address of the captured state, and a reviewable label. Raw buffer bytes,
/// raw DOM, raw shell commands, and raw secrets never cross this boundary — the
/// state is named by a content-address digest only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedCommand {
    /// Opaque command id; never a raw path, argv, or secret.
    pub command_id: String,
    /// The surface the captured command quotes.
    pub surface_class: RecordedSurfaceClass,
    /// Whether the command is supported, and if not, why.
    pub support_class: CapturedCommandSupportClass,
    /// The replay posture the command admits.
    pub replay_posture_class: ReplayPostureClass,
    /// Content-address of the captured UI / editor state.
    pub state_digest: ContentAddress,
    /// Monotonic capture timestamp.
    pub captured_at: String,
    /// Reviewable label for the captured-command review.
    pub label: String,
}

impl CapturedCommand {
    /// Whether the command is a supported recorded-macro step.
    pub fn is_supported(&self) -> bool {
        self.support_class.is_supported()
    }

    /// Projects a supported command onto a frozen macro-session capture step.
    ///
    /// Only a supported command is admissible to a saved macro record; an
    /// unsupported command is flagged and blocks save, so it never becomes a step.
    pub fn to_capture_step(&self) -> MacroCaptureStep {
        MacroCaptureStep {
            step_id: self.command_id.clone(),
            surface_class: self.surface_class.as_str().to_owned(),
            state_digest: self.state_digest.clone(),
            replay_posture_class: self.replay_posture_class.as_str().to_owned(),
            captured_at: self.captured_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Active recording strip
// ---------------------------------------------------------------------------

/// The live strip the recorder renders while a session captures.
///
/// The strip is a projection of the session's current state: whether it is
/// capturing, how many commands it has captured, how many of those are unsupported
/// (and so flagged), the declared target scope, and a reviewable summary. It is the
/// at-a-glance surface a user reads to know what the recorder is doing right now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveRecordingStrip {
    /// The recorder lifecycle state.
    pub recorder_state_class: MacroRecorderStateClass,
    /// Whether the recorder is actively capturing.
    pub is_capturing: bool,
    /// Count of captured commands.
    pub captured_command_count: u32,
    /// Count of captured commands that are supported macro steps.
    pub supported_command_count: u32,
    /// Count of captured commands flagged as unsupported.
    pub unsupported_command_count: u32,
    /// The declared target scope.
    pub target_scope_class: TargetScopeClass,
    /// Reviewable strip summary.
    pub strip_summary: String,
}

// ---------------------------------------------------------------------------
// Captured-command review
// ---------------------------------------------------------------------------

/// One unsupported-command warning surfaced in the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedCommandWarning {
    /// The opaque command id the warning is about.
    pub command_id: String,
    /// The support class explaining why the command is unsupported.
    pub support_class: CapturedCommandSupportClass,
    /// Reviewable warning sentence.
    pub warning: String,
}

/// One reviewable row in the captured-command review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedCommandReviewRow {
    /// The opaque command id.
    pub command_id: String,
    /// The surface the command quotes.
    pub surface_class: RecordedSurfaceClass,
    /// The support class.
    pub support_class: CapturedCommandSupportClass,
    /// Whether the command is supported.
    pub supported: bool,
    /// Reviewable label.
    pub label: String,
}

/// The review a user inspects before saving or discarding a recording.
///
/// The review lists every captured command with its support class, surfaces every
/// unsupported-command warning, and resolves whether the recording is safe to save.
/// A recording with an unsupported command is **not** savable as a macro
/// ([`CapturedCommandReview::save_admissible`] is false); the user must remove the
/// unsupported command or author the flow as a declarative recipe instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedCommandReview {
    /// One row per captured command.
    pub command_rows: Vec<CapturedCommandReviewRow>,
    /// Every unsupported-command warning.
    pub unsupported_command_warnings: Vec<UnsupportedCommandWarning>,
    /// Count of supported commands.
    pub supported_command_count: u32,
    /// Count of unsupported commands.
    pub unsupported_command_count: u32,
    /// Whether any captured command is unsupported.
    pub has_unsupported_command: bool,
    /// Whether the recording is admissible to save as a macro.
    pub save_admissible: bool,
    /// Reviewable review summary.
    pub review_summary: String,
}

// ---------------------------------------------------------------------------
// Macro replay resolution
// ---------------------------------------------------------------------------

/// An explicit replay-under-current-context resolution minted from a session.
///
/// The record is the enforcement point for "a recording is not authority": it
/// carries the derived [`MacroReplayActionClass`], the blockers observed now, the
/// declared target scope, and the assertions that replay declares its target scope,
/// refuses unsafe reuse when the context no longer matches, and re-resolves the
/// supported-command set every time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayResolution {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Monotonic resolution timestamp.
    pub resolved_at: String,
    /// Opaque session id the resolution is for.
    pub session_id: String,
    /// Opaque ref to the resolved macro manifest, or null when none was minted.
    pub macro_manifest_ref: Option<String>,
    /// The declared target scope the replay is bound to.
    pub declared_target_scope_class: TargetScopeClass,
    /// The resolved replay action class.
    pub replay_action_class: MacroReplayActionClass,
    /// The blockers observed at resolution time.
    pub current_replay_blockers: Vec<MacroReplayBlocker>,
    /// Whether replay is admissible today (in scope, or after reconciliation).
    pub admissible: bool,
    /// Whether the resolution fails closed (refuses replay).
    pub fails_closed: bool,
    /// Always true: the replay declares its target scope.
    pub declares_target_scope: bool,
    /// Always true: the replay refuses unsafe reuse on a context mismatch.
    pub refuses_on_context_mismatch: bool,
    /// Always true: the supported-command set is re-resolved.
    pub reresolves_supported_command_set: bool,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl MacroReplayResolution {
    /// Whether the resolution declares scope and refuses unsafe reuse.
    pub fn is_fail_closed_safe(&self) -> bool {
        self.declares_target_scope
            && self.refuses_on_context_mismatch
            && self.reresolves_supported_command_set
    }
}

// ---------------------------------------------------------------------------
// Recorder session (the live object)
// ---------------------------------------------------------------------------

/// One macro-recorder session: a recorded editing flow captured as a macro.
///
/// The session records a single editing flow with the commands it captured, the
/// target scope it declares, the profile-local storage it lives in, the safety
/// labels it projects (always `macro_safe` / `ui_only`), its promotion affordance,
/// its replay count, and the disposition the user resolved it to. The replay action
/// is **derived**, never stored as authority:
/// [`MacroRecorderSession::resolved_replay_class`] resolves it from the
/// repository-import state and the observed [`MacroReplayBlocker`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderSession {
    /// Opaque session id.
    pub session_id: String,
    /// The M5 automation family the session belongs to.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Integer schema version of the underlying macro-session record.
    pub macro_session_schema_version: u32,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// The recorder lifecycle state.
    pub recorder_state_class: MacroRecorderStateClass,
    /// The save-or-discard disposition the session resolved to.
    pub disposition_class: SessionDispositionClass,
    /// The captured commands, in capture order.
    pub captured_commands: Vec<CapturedCommand>,
    /// The declared target scope.
    pub declared_target_scope_class: TargetScopeClass,
    /// The profile-local storage scope (the default; never the managed channel).
    pub storage_scope_class: MacroStorageScopeClass,
    /// The safety labels projected onto the session (always `macro_safe` / `ui_only`).
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
    /// The promotion affordance for the captured macro.
    pub promotion_affordance_class: MacroPromotionAffordanceClass,
    /// How many times the macro has been replayed.
    pub replay_count: u32,
    /// The blockers the resolver observed at projection time.
    pub current_replay_blockers: Vec<MacroReplayBlocker>,
    /// Whether the macro was imported from repository content (never auto-replayable).
    pub imported_from_repository_content: bool,
    /// The redaction mode the session's safe summary carries.
    pub redaction_class: MacroRedactionClass,
    /// Opaque ref to the resulting macro manifest, or null when discarded.
    pub resulting_macro_manifest_ref: Option<String>,
    /// Schema the recorder mints a recorded-macro manifest against on save.
    pub manifest_target_schema_ref: String,
    /// Monotonic record timestamp.
    pub recorded_at: String,
    /// Monotonic last-replay timestamp, or null when never replayed.
    pub last_replayed_at: Option<String>,
}

impl MacroRecorderSession {
    /// The resolved replay action, derived from import state and observed blockers.
    ///
    /// A repository-imported macro always resolves to
    /// [`MacroReplayActionClass::BlockedImportedFromRepositoryContent`]. Otherwise a
    /// fail-closed blocker dominates a reconcilable one, which dominates no blocker;
    /// among same-disposition blockers the canonical order wins.
    pub fn resolved_replay_class(&self) -> MacroReplayActionClass {
        derive_replay_class(
            self.imported_from_repository_content,
            &self.current_replay_blockers,
        )
    }

    /// Whether the resolved replay admits a replay today (after reconciliation, if any).
    pub fn replay_admissible(&self) -> bool {
        self.resolved_replay_class().is_admissible()
    }

    /// Whether any captured command is unsupported.
    pub fn has_unsupported_command(&self) -> bool {
        self.captured_commands
            .iter()
            .any(|command| !command.is_supported())
    }

    /// The live recording strip projected from the session.
    pub fn active_recording_strip(&self) -> ActiveRecordingStrip {
        let supported = self
            .captured_commands
            .iter()
            .filter(|command| command.is_supported())
            .count() as u32;
        let total = self.captured_commands.len() as u32;
        let unsupported = total - supported;
        let is_capturing = matches!(
            self.recorder_state_class,
            MacroRecorderStateClass::Recording
        );
        ActiveRecordingStrip {
            recorder_state_class: self.recorder_state_class,
            is_capturing,
            captured_command_count: total,
            supported_command_count: supported,
            unsupported_command_count: unsupported,
            target_scope_class: self.declared_target_scope_class,
            strip_summary: format!(
                "{} command(s) captured ({} unsupported) in {}",
                total,
                unsupported,
                self.declared_target_scope_class.as_str()
            ),
        }
    }

    /// The captured-command review the user inspects before saving or discarding.
    pub fn captured_command_review(&self) -> CapturedCommandReview {
        let command_rows = self
            .captured_commands
            .iter()
            .map(|command| CapturedCommandReviewRow {
                command_id: command.command_id.clone(),
                surface_class: command.surface_class,
                support_class: command.support_class,
                supported: command.is_supported(),
                label: command.label.clone(),
            })
            .collect();
        let unsupported_command_warnings = self.unsupported_command_warnings();
        let unsupported_command_count = unsupported_command_warnings.len() as u32;
        let supported_command_count =
            self.captured_commands.len() as u32 - unsupported_command_count;
        let has_unsupported_command = unsupported_command_count > 0;
        CapturedCommandReview {
            command_rows,
            unsupported_command_warnings,
            supported_command_count,
            unsupported_command_count,
            has_unsupported_command,
            save_admissible: !has_unsupported_command,
            review_summary: format!(
                "{} supported, {} unsupported; {}",
                supported_command_count,
                unsupported_command_count,
                if has_unsupported_command {
                    "remove the unsupported command(s) or author a recipe before saving"
                } else {
                    "safe to save as a profile-local macro"
                }
            ),
        }
    }

    /// The unsupported-command warnings the recorder surfaces.
    pub fn unsupported_command_warnings(&self) -> Vec<UnsupportedCommandWarning> {
        self.captured_commands
            .iter()
            .filter(|command| !command.is_supported())
            .map(|command| UnsupportedCommandWarning {
                command_id: command.command_id.clone(),
                support_class: command.support_class,
                warning: format!(
                    "command {} is {} and cannot be recorded as a macro step",
                    command.command_id,
                    command.support_class.as_str()
                ),
            })
            .collect()
    }

    /// Resolves replay-under-current-context into an explicit, attributable record.
    pub fn resolve_replay(&self, resolved_at: impl Into<String>) -> MacroReplayResolution {
        let replay_action_class = self.resolved_replay_class();
        MacroReplayResolution {
            record_kind: MACRO_REPLAY_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            resolved_at: resolved_at.into(),
            session_id: self.session_id.clone(),
            macro_manifest_ref: self.resulting_macro_manifest_ref.clone(),
            declared_target_scope_class: self.declared_target_scope_class,
            replay_action_class,
            current_replay_blockers: self.current_replay_blockers.clone(),
            admissible: replay_action_class.is_admissible(),
            fails_closed: replay_action_class.is_fail_closed(),
            declares_target_scope: true,
            refuses_on_context_mismatch: true,
            reresolves_supported_command_set: true,
            summary: format!(
                "replay resolves current context in {}; {} blocker(s) observed",
                self.declared_target_scope_class.as_str(),
                self.current_replay_blockers.len()
            ),
        }
    }

    /// Whether the session's replay derivation is internally consistent.
    ///
    /// The no-blocker pairing holds (admissible-in-scope pairs with exactly
    /// `[NoBlockerPresent]`; any other class cites a non-no-blocker entry and no
    /// `NoBlockerPresent`), a repository-imported macro resolves to the
    /// imported-blocked class, and a session that captured an unsupported command
    /// fails closed.
    pub fn replay_consistent(&self) -> bool {
        let resolved = self.resolved_replay_class();
        let has_no_blocker = self
            .current_replay_blockers
            .contains(&MacroReplayBlocker::NoBlockerPresent);
        let pairing_ok = if resolved == MacroReplayActionClass::ReplayInDeclaredScope {
            self.current_replay_blockers == [MacroReplayBlocker::NoBlockerPresent]
        } else {
            !has_no_blocker
                && self
                    .current_replay_blockers
                    .iter()
                    .any(|blocker| blocker.replay_action_class() == resolved)
        };
        let imported_ok = !self.imported_from_repository_content
            || resolved == MacroReplayActionClass::BlockedImportedFromRepositoryContent;
        let unsupported_ok = !self.has_unsupported_command() || resolved.is_fail_closed();
        pairing_ok && imported_ok && unsupported_ok
    }

    /// Whether the promotion affordance is explicit for a cross-scope macro.
    ///
    /// A macro whose declared scope crosses files must not be promotable-as-UI-only,
    /// and it must carry the cross-scope promotion blocker so direct replay fails
    /// closed pending an explicit promotion to a declarative recipe. A macro that
    /// stays in scope must not carry that blocker.
    pub fn promotion_consistent(&self) -> bool {
        let crosses = self.declared_target_scope_class.requires_promotion();
        let blocked_for_promotion = self
            .current_replay_blockers
            .contains(&MacroReplayBlocker::PromotionRequiredCrossesScope);
        let affordance_ok = !crosses
            || self.promotion_affordance_class
                != MacroPromotionAffordanceClass::NotPromotableUiOnly;
        crosses == blocked_for_promotion && affordance_ok
    }

    /// Whether the macro is profile-local by default and not repository-defined.
    pub fn profile_local_default_consistent(&self) -> bool {
        !self.imported_from_repository_content && self.storage_scope_class.is_local_only()
    }

    /// Whether every projected safety label is `macro_safe` or `ui_only`.
    pub fn safety_labels_constrained(&self) -> bool {
        !self.projected_safety_labels.is_empty()
            && self.projected_safety_labels.iter().all(|label| {
                matches!(
                    label,
                    AutomationSafetyLabelId::MacroSafe | AutomationSafetyLabelId::UiOnly
                )
            })
    }

    /// Whether the save-or-discard disposition matches the minted manifest ref.
    ///
    /// A discarded session mints no manifest; a saved session mints one. A still
    /// recording session has not resolved a disposition and mints none.
    pub fn disposition_consistent(&self) -> bool {
        self.disposition_class.mints_manifest() == self.resulting_macro_manifest_ref.is_some()
    }

    /// Whether every captured command id and digest is an opaque, redaction-safe handle.
    pub fn captures_are_opaque(&self) -> bool {
        self.captured_commands.iter().all(|command| {
            reference_is_opaque(&command.command_id)
                && reference_is_opaque(&command.state_digest.digest_hex)
        })
    }

    /// Projects a saved session onto a frozen macro-session record.
    ///
    /// Only supported commands become capture steps; an unsupported command is
    /// flagged and blocks save, so it never reaches a record. The record conforms to
    /// [`/schemas/automation/macro-session.schema.json`](../../../schemas/automation/macro-session.schema.json).
    pub fn to_session_record(&self) -> MacroSession {
        MacroSession {
            record_kind: "macro_session_record".to_owned(),
            macro_session_schema_version: self.macro_session_schema_version,
            session_id: self.session_id.clone(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            recorder_state_class: self.recorder_state_class,
            storage_scope_class: self.storage_scope_class.as_str().to_owned(),
            projected_safety_labels: self.projected_safety_labels.clone(),
            captured_steps: self
                .captured_commands
                .iter()
                .filter(|command| command.is_supported())
                .map(CapturedCommand::to_capture_step)
                .collect(),
            redaction_class: self.redaction_class.as_str().to_owned(),
            promotion_affordance_class: self.promotion_affordance_class,
            resulting_macro_manifest_ref: self.resulting_macro_manifest_ref.clone(),
            manifest_target_schema_ref: self.manifest_target_schema_ref.clone(),
            minted_at: self.recorded_at.clone(),
        }
    }

    /// Exports the session, carrying its macro record, replay resolution, and review.
    pub fn export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> MacroSessionExport {
        let exported_at = exported_at.into();
        MacroSessionExport {
            record_kind: MACRO_SESSION_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.clone(),
            session_record: self.to_session_record(),
            replay_resolution: self.resolve_replay(exported_at),
            captured_command_review: self.captured_command_review(),
            session: self.clone(),
            export_digest: self.session_digest(),
        }
    }

    /// Order-stable digest over the session's identity, scope, and blockers.
    pub fn session_digest(&self) -> String {
        fnv1a64(&self.digest_tokens())
    }

    fn digest_tokens(&self) -> Vec<String> {
        let mut tokens = vec![
            self.session_id.clone(),
            self.recorder_state_class.as_str().to_owned(),
            self.disposition_class.as_str().to_owned(),
            self.declared_target_scope_class.as_str().to_owned(),
            self.storage_scope_class.as_str().to_owned(),
            self.promotion_affordance_class.as_str().to_owned(),
            self.resolved_replay_class().as_str().to_owned(),
        ];
        for blocker in &self.current_replay_blockers {
            tokens.push(blocker.as_str().to_owned());
        }
        for command in &self.captured_commands {
            tokens.push(command.command_id.clone());
            tokens.push(command.support_class.as_str().to_owned());
        }
        tokens
    }
}

/// Whether a reference looks like an opaque, redaction-safe handle.
fn reference_is_opaque(reference: &str) -> bool {
    !reference.is_empty()
        && !reference.contains("raw:")
        && !reference.contains("://")
        && !reference.starts_with('/')
}

// ---------------------------------------------------------------------------
// Macro session export
// ---------------------------------------------------------------------------

/// A macro session exported for replay review, comparison, or support bundles.
///
/// The export nests the whole [`MacroRecorderSession`] verbatim alongside the
/// derived macro-session record, the resolved replay resolution, and the
/// captured-command review, plus an order-stable digest.
/// [`MacroSessionExport::import`] reconstructs the identical session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroSessionExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// The frozen macro-session record the export carries.
    pub session_record: MacroSession,
    /// The replay resolution resolved at export time.
    pub replay_resolution: MacroReplayResolution,
    /// The captured-command review carried for support and comparison.
    pub captured_command_review: CapturedCommandReview,
    /// The session, preserved verbatim for round-trip import.
    pub session: MacroRecorderSession,
    /// Order-stable digest over the session.
    pub export_digest: String,
}

impl MacroSessionExport {
    /// Reconstructs the session from the export.
    pub fn import(&self) -> MacroRecorderSession {
        self.session.clone()
    }

    /// Whether the export preserves replay and scope truth across the boundary.
    ///
    /// The replay resolution must quote the same action the session derives, declare
    /// the same target scope, fail closed safely, and agree on the digest — so a
    /// macro stays comparable and refuses unsafe reuse after export, history, and
    /// support without ever reading as fresh authority.
    pub fn replay_and_scope_preserved(&self) -> bool {
        self.replay_resolution.replay_action_class == self.session.resolved_replay_class()
            && self.replay_resolution.declared_target_scope_class
                == self.session.declared_target_scope_class
            && self.replay_resolution.is_fail_closed_safe()
            && self.session.replay_consistent()
            && self.export_digest == self.session.session_digest()
    }
}

// ---------------------------------------------------------------------------
// First-consumer binding
// ---------------------------------------------------------------------------

/// One entrypoint binding: the seeded macro-recorder panel a consumer renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderConsumerBinding {
    /// The entrypoint this binding describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// The ordered macro-recorder sessions the panel renders (newest first).
    pub sessions: Vec<MacroRecorderSession>,
    /// The replay resolutions the panel projects, index-aligned with sessions.
    pub replay_resolutions: Vec<MacroReplayResolution>,
    /// Count of sessions.
    pub session_count: u32,
    /// Count of sessions whose replay is admissible today.
    pub replayable_count: u32,
    /// Count of sessions whose replay is blocked today.
    pub blocked_replay_count: u32,
    /// Count of sessions promotable to a declarative recipe.
    pub promotable_count: u32,
    /// Count of sessions that captured an unsupported command.
    pub unsupported_command_session_count: u32,
    /// The session id of the latest session.
    pub latest_session_id: String,
    /// The recorder state of the latest session.
    pub latest_state_class: MacroRecorderStateClass,
    /// The resolved replay action of the latest session.
    pub latest_replay_action_class: MacroReplayActionClass,
    /// Reviewable summary of what the consumer renders.
    pub binding_summary: String,
}

impl MacroRecorderConsumerBinding {
    /// Builds a binding from a consumer's seeded panel of sessions.
    ///
    /// The sessions are rendered newest-first; the first session is the latest.
    pub fn from_sessions(
        entrypoint: RecipeBuilderEntrypoint,
        sessions: Vec<MacroRecorderSession>,
        binding_summary: impl Into<String>,
    ) -> Self {
        let latest = sessions
            .first()
            .expect("a binding must carry at least one session");
        let replay_resolutions = sessions
            .iter()
            .map(|session| session.resolve_replay(session.recorded_at.clone()))
            .collect();
        let replayable_count = sessions
            .iter()
            .filter(|session| session.replay_admissible())
            .count() as u32;
        let blocked_replay_count = sessions.len() as u32 - replayable_count;
        let promotable_count = sessions
            .iter()
            .filter(|session| {
                session.promotion_affordance_class
                    == MacroPromotionAffordanceClass::PromotableToDeclarativeRecipe
            })
            .count() as u32;
        let unsupported_command_session_count = sessions
            .iter()
            .filter(|session| session.has_unsupported_command())
            .count() as u32;
        MacroRecorderConsumerBinding {
            entrypoint,
            title: entrypoint.title().to_owned(),
            session_count: sessions.len() as u32,
            replayable_count,
            blocked_replay_count,
            promotable_count,
            unsupported_command_session_count,
            latest_session_id: latest.session_id.clone(),
            latest_state_class: latest.recorder_state_class,
            latest_replay_action_class: latest.resolved_replay_class(),
            binding_summary: binding_summary.into(),
            replay_resolutions,
            sessions,
        }
    }
}

// ---------------------------------------------------------------------------
// Invariants and findings
// ---------------------------------------------------------------------------

/// Frozen invariants the first-consumers packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderInvariantsBlock {
    /// Every first-consumer entrypoint binds a non-empty session panel.
    pub every_entrypoint_binds_a_session_panel: bool,
    /// Every session declares its target scope and storage scope.
    pub every_session_declares_target_and_storage_scope: bool,
    /// Recorded macros are profile-local by default.
    pub recorded_macros_are_profile_local_by_default: bool,
    /// Unsupported commands are flagged and block save.
    pub unsupported_commands_are_flagged_and_block_save: bool,
    /// Replay fails closed when the context or scope no longer matches.
    pub replay_fails_closed_when_context_or_scope_no_longer_matches: bool,
    /// Repository content never defines an executable macro.
    pub repository_content_never_defines_an_executable_macro: bool,
    /// Promotion to a recipe is explicit when a macro crosses scope.
    pub promotion_to_recipe_is_explicit_when_macro_crosses_scope: bool,
    /// Macro sessions capture UI or editor state only.
    pub macro_sessions_capture_ui_or_editor_state_only: bool,
    /// Macro sessions never use the managed-only channel.
    pub macro_sessions_never_use_the_managed_only_channel: bool,
}

impl MacroRecorderInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        MacroRecorderInvariantsBlock {
            every_entrypoint_binds_a_session_panel: true,
            every_session_declares_target_and_storage_scope: true,
            recorded_macros_are_profile_local_by_default: true,
            unsupported_commands_are_flagged_and_block_save: true,
            replay_fails_closed_when_context_or_scope_no_longer_matches: true,
            repository_content_never_defines_an_executable_macro: true,
            promotion_to_recipe_is_explicit_when_macro_crosses_scope: true,
            macro_sessions_capture_ui_or_editor_state_only: true,
            macro_sessions_never_use_the_managed_only_channel: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 9] {
        [
            (
                "every_entrypoint_binds_a_session_panel",
                self.every_entrypoint_binds_a_session_panel,
            ),
            (
                "every_session_declares_target_and_storage_scope",
                self.every_session_declares_target_and_storage_scope,
            ),
            (
                "recorded_macros_are_profile_local_by_default",
                self.recorded_macros_are_profile_local_by_default,
            ),
            (
                "unsupported_commands_are_flagged_and_block_save",
                self.unsupported_commands_are_flagged_and_block_save,
            ),
            (
                "replay_fails_closed_when_context_or_scope_no_longer_matches",
                self.replay_fails_closed_when_context_or_scope_no_longer_matches,
            ),
            (
                "repository_content_never_defines_an_executable_macro",
                self.repository_content_never_defines_an_executable_macro,
            ),
            (
                "promotion_to_recipe_is_explicit_when_macro_crosses_scope",
                self.promotion_to_recipe_is_explicit_when_macro_crosses_scope,
            ),
            (
                "macro_sessions_capture_ui_or_editor_state_only",
                self.macro_sessions_capture_ui_or_editor_state_only,
            ),
            (
                "macro_sessions_never_use_the_managed_only_channel",
                self.macro_sessions_never_use_the_managed_only_channel,
            ),
        ]
    }
}

/// Severity of a macro-recorder validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRecorderFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a macro-recorder validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRecorderFindingKind {
    /// A required first-consumer entrypoint is absent.
    MissingEntrypoint,
    /// An entrypoint binds a panel with no sessions.
    EntrypointPanelEmpty,
    /// A replay resolution implies stale context (the no-blocker pairing is violated).
    ReplayImpliesStaleContext,
    /// A replay does not fail closed when the context or scope no longer matches.
    ReplayNotFailClosedOnContextMismatch,
    /// An unsupported command is not flagged or does not block save.
    UnsupportedCommandNotBlocked,
    /// A macro was imported from repository content.
    RepositoryContentDefinesMacro,
    /// A cross-scope macro's promotion is not explicit.
    PromotionNotExplicitForCrossScope,
    /// A session captures ambient state or uses the managed-only channel.
    AmbientOrManagedOnlyCapture,
    /// The projected replay resolution disagrees with the live session.
    ReplayResolutionProjectionInconsistent,
    /// A recorded macro is not profile-local by default.
    ProfileLocalDefaultViolated,
    /// A raw secret value appears in a macro session.
    RawSecretMaterialInSession,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl MacroRecorderFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroRecorderFindingKind::MissingEntrypoint => "missing_entrypoint",
            MacroRecorderFindingKind::EntrypointPanelEmpty => "entrypoint_panel_empty",
            MacroRecorderFindingKind::ReplayImpliesStaleContext => "replay_implies_stale_context",
            MacroRecorderFindingKind::ReplayNotFailClosedOnContextMismatch => {
                "replay_not_fail_closed_on_context_mismatch"
            }
            MacroRecorderFindingKind::UnsupportedCommandNotBlocked => {
                "unsupported_command_not_blocked"
            }
            MacroRecorderFindingKind::RepositoryContentDefinesMacro => {
                "repository_content_defines_macro"
            }
            MacroRecorderFindingKind::PromotionNotExplicitForCrossScope => {
                "promotion_not_explicit_for_cross_scope"
            }
            MacroRecorderFindingKind::AmbientOrManagedOnlyCapture => {
                "ambient_or_managed_only_capture"
            }
            MacroRecorderFindingKind::ReplayResolutionProjectionInconsistent => {
                "replay_resolution_projection_inconsistent"
            }
            MacroRecorderFindingKind::ProfileLocalDefaultViolated => {
                "profile_local_default_violated"
            }
            MacroRecorderFindingKind::RawSecretMaterialInSession => {
                "raw_secret_material_in_session"
            }
            MacroRecorderFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the first-consumers gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderFinding {
    /// The finding kind.
    pub finding_kind: MacroRecorderFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: MacroRecorderFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl MacroRecorderFinding {
    fn blocker(
        finding_kind: MacroRecorderFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        MacroRecorderFinding {
            finding_kind,
            severity: MacroRecorderFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// First-consumers packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderFirstConsumersInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<MacroRecorderConsumerBinding>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: MacroRecorderInvariantsBlock,
}

/// Canonical M5 macro-recorder first-consumers packet.
///
/// The packet binds every first-consumer entrypoint to a seeded panel and pins the
/// freeze invariants. [`MacroRecorderFirstConsumersPacket::validate`] recomputes the
/// findings so the fail-closed gate and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderFirstConsumersPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub schema_ref: String,
    /// Reused macro-session boundary schema ref.
    pub macro_session_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<MacroRecorderConsumerBinding>,
    /// Frozen invariants block.
    pub invariants: MacroRecorderInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<MacroRecorderFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over entrypoint and session tokens.
    pub packet_digest: String,
}

impl MacroRecorderFirstConsumersPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: MacroRecorderFirstConsumersInput) -> Self {
        let findings = validate_parts(&input.consumer_bindings, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.consumer_bindings);
        MacroRecorderFirstConsumersPacket {
            record_kind: MACRO_RECORDER_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            macro_session_schema_ref: MACRO_SESSION_SCHEMA_REF.to_owned(),
            doc_ref: MACRO_RECORDER_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            consumer_bindings: input.consumer_bindings,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<MacroRecorderFinding> {
        validate_parts(&self.consumer_bindings, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The binding for one entrypoint, if present.
    pub fn binding(
        &self,
        entrypoint: RecipeBuilderEntrypoint,
    ) -> Option<&MacroRecorderConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
    }

    /// Entrypoint tokens in the order the packet stores them.
    pub fn entrypoint_tokens(&self) -> Vec<&'static str> {
        self.consumer_bindings
            .iter()
            .map(|binding| binding.entrypoint.as_str())
            .collect()
    }

    /// Every replay resolution across every binding, for support and comparison.
    pub fn all_replay_resolutions(&self) -> Vec<MacroReplayResolution> {
        self.consumer_bindings
            .iter()
            .flat_map(|binding| binding.replay_resolutions.iter().cloned())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> MacroRecorderFirstConsumersSupportExport {
        MacroRecorderFirstConsumersSupportExport {
            record_kind: MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            consumer_rows: self
                .consumer_bindings
                .iter()
                .map(MacroRecorderSupportConsumerRow::from_binding)
                .collect(),
            replay_resolutions: self.all_replay_resolutions(),
            invariants: self.invariants.clone(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind)
                .collect(),
        }
    }

    /// Builds the compact CLI / headless projection.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> MacroRecorderFirstConsumersCliHeadlessView {
        MacroRecorderFirstConsumersCliHeadlessView {
            record_kind: MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: MACRO_RECORDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            consumer_lines: self
                .consumer_bindings
                .iter()
                .map(|binding| {
                    format!(
                        "{} sessions={} latest={} state={} replay={} replayable={} blocked={} promotable={} unsupported={}",
                        binding.entrypoint.as_str(),
                        binding.session_count,
                        binding.latest_session_id,
                        binding.latest_state_class.as_str(),
                        binding.latest_replay_action_class.as_str(),
                        binding.replayable_count,
                        binding.blocked_replay_count,
                        binding.promotable_count,
                        binding.unsupported_command_session_count,
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} consumers={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.consumer_bindings.len(),
            self.packet_digest,
        )];
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} sessions={} latest={} state={} replayable={} blocked={} promotable={} unsupported={}",
                binding.entrypoint.as_str(),
                binding.session_count,
                binding.latest_session_id,
                binding.latest_state_class.as_str(),
                binding.replayable_count,
                binding.blocked_replay_count,
                binding.promotable_count,
                binding.unsupported_command_session_count,
            ));
            for session in &binding.sessions {
                lines.push(format!(
                    "  session {} state={} disposition={} scope={} storage={} replay={} promotion={} replays={} unsupported={} imported={}",
                    session.session_id,
                    session.recorder_state_class.as_str(),
                    session.disposition_class.as_str(),
                    session.declared_target_scope_class.as_str(),
                    session.storage_scope_class.as_str(),
                    session.resolved_replay_class().as_str(),
                    session.promotion_affordance_class.as_str(),
                    session.replay_count,
                    session.has_unsupported_command(),
                    session.imported_from_repository_content,
                ));
            }
        }
        lines
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// One redacted support-export session row (no raw path, URL, or content).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderSupportSessionRow {
    /// Opaque session id.
    pub session_id: String,
    /// The recorder state.
    pub recorder_state_class: MacroRecorderStateClass,
    /// The save-or-discard disposition.
    pub disposition_class: SessionDispositionClass,
    /// The declared target scope.
    pub declared_target_scope_class: TargetScopeClass,
    /// The profile-local storage scope.
    pub storage_scope_class: MacroStorageScopeClass,
    /// The resolved replay action class.
    pub replay_action_class: MacroReplayActionClass,
    /// Whether replay is admissible today.
    pub replay_admissible: bool,
    /// The promotion affordance.
    pub promotion_affordance_class: MacroPromotionAffordanceClass,
    /// The replay count.
    pub replay_count: u32,
    /// Whether the session captured an unsupported command.
    pub has_unsupported_command: bool,
    /// The current replay blockers observed.
    pub current_replay_blockers: Vec<MacroReplayBlocker>,
    /// Whether the macro was imported from repository content.
    pub imported_from_repository_content: bool,
}

impl MacroRecorderSupportSessionRow {
    fn from_session(session: &MacroRecorderSession) -> Self {
        let replay_action_class = session.resolved_replay_class();
        MacroRecorderSupportSessionRow {
            session_id: session.session_id.clone(),
            recorder_state_class: session.recorder_state_class,
            disposition_class: session.disposition_class,
            declared_target_scope_class: session.declared_target_scope_class,
            storage_scope_class: session.storage_scope_class,
            replay_action_class,
            replay_admissible: replay_action_class.is_admissible(),
            promotion_affordance_class: session.promotion_affordance_class,
            replay_count: session.replay_count,
            has_unsupported_command: session.has_unsupported_command(),
            current_replay_blockers: session.current_replay_blockers.clone(),
            imported_from_repository_content: session.imported_from_repository_content,
        }
    }
}

/// One redacted support-export consumer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderSupportConsumerRow {
    /// The entrypoint this row describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Count of sessions.
    pub session_count: u32,
    /// The latest session id.
    pub latest_session_id: String,
    /// The latest recorder state.
    pub latest_state_class: MacroRecorderStateClass,
    /// The latest resolved replay action.
    pub latest_replay_action_class: MacroReplayActionClass,
    /// Count of sessions whose replay is admissible today.
    pub replayable_count: u32,
    /// Count of sessions whose replay is blocked today.
    pub blocked_replay_count: u32,
    /// Count of sessions promotable to a recipe.
    pub promotable_count: u32,
    /// Count of sessions that captured an unsupported command.
    pub unsupported_command_session_count: u32,
    /// Per-session redacted rows.
    pub session_rows: Vec<MacroRecorderSupportSessionRow>,
}

impl MacroRecorderSupportConsumerRow {
    fn from_binding(binding: &MacroRecorderConsumerBinding) -> Self {
        MacroRecorderSupportConsumerRow {
            entrypoint: binding.entrypoint,
            title: binding.title.clone(),
            session_count: binding.session_count,
            latest_session_id: binding.latest_session_id.clone(),
            latest_state_class: binding.latest_state_class,
            latest_replay_action_class: binding.latest_replay_action_class,
            replayable_count: binding.replayable_count,
            blocked_replay_count: binding.blocked_replay_count,
            promotable_count: binding.promotable_count,
            unsupported_command_session_count: binding.unsupported_command_session_count,
            session_rows: binding
                .sessions
                .iter()
                .map(MacroRecorderSupportSessionRow::from_session)
                .collect(),
        }
    }
}

/// Redacted support-export projection of the first-consumers packet.
///
/// The export carries the per-session scope, state, replay action, and blocker
/// classes plus the resolved replay resolutions, so a macro-recorder panel is
/// reviewable in a support bundle — and a macro stays comparable and refuses unsafe
/// reuse across surfaces — without a raw path, URL, or secret ever crossing the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderFirstConsumersSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Packet id this export was minted from.
    pub packet_id: String,
    /// Packet digest carried for verification.
    pub packet_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Consumer rows.
    pub consumer_rows: Vec<MacroRecorderSupportConsumerRow>,
    /// Resolved replay resolutions carried for support and comparison.
    pub replay_resolutions: Vec<MacroReplayResolution>,
    /// Frozen invariants block.
    pub invariants: MacroRecorderInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<MacroRecorderFindingKind>,
}

impl MacroRecorderFirstConsumersSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.consumer_rows.is_empty()
            && !self.replay_resolutions.is_empty()
    }
}

/// Compact CLI / headless projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroRecorderFirstConsumersCliHeadlessView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// Packet id this view was minted from.
    pub packet_id: String,
    /// Promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// One line per consumer entrypoint.
    pub consumer_lines: Vec<String>,
}

impl MacroRecorderFirstConsumersCliHeadlessView {
    /// Whether the view explains every entrypoint.
    pub fn every_entrypoint_explained(&self) -> bool {
        self.consumer_lines.len() == RecipeBuilderEntrypoint::ALL.len()
    }
}

// ---------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------

/// Derives the resolved replay class from the import state and observed blockers.
fn derive_replay_class(imported: bool, blockers: &[MacroReplayBlocker]) -> MacroReplayActionClass {
    if imported {
        return MacroReplayActionClass::BlockedImportedFromRepositoryContent;
    }
    // A fail-closed blocker dominates a reconcilable one, which dominates no blocker;
    // among same-disposition blockers the canonical MacroReplayBlocker::ALL order wins.
    let mut fail_closed: Option<MacroReplayBlocker> = None;
    let mut reconcilable: Option<MacroReplayBlocker> = None;
    for candidate in MacroReplayBlocker::ALL {
        if !blockers.contains(&candidate) {
            continue;
        }
        match candidate.disposition() {
            MacroReplayDisposition::FailsClosed if fail_closed.is_none() => {
                fail_closed = Some(candidate)
            }
            MacroReplayDisposition::RequiresScopeReconciliation if reconcilable.is_none() => {
                reconcilable = Some(candidate)
            }
            _ => {}
        }
    }
    if let Some(blocker) = fail_closed {
        return blocker.replay_action_class();
    }
    if let Some(blocker) = reconcilable {
        return blocker.replay_action_class();
    }
    MacroReplayActionClass::ReplayInDeclaredScope
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    consumer_bindings: &[MacroRecorderConsumerBinding],
    invariants: &MacroRecorderInvariantsBlock,
) -> Vec<MacroRecorderFinding> {
    let mut findings = Vec::new();

    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let Some(binding) = consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
        else {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::MissingEntrypoint,
                Some(entrypoint.as_str().to_owned()),
                format!(
                    "the {} entrypoint binds no macro-recorder panel",
                    entrypoint.as_str()
                ),
            ));
            continue;
        };
        validate_binding(binding, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_binding(
    binding: &MacroRecorderConsumerBinding,
    findings: &mut Vec<MacroRecorderFinding>,
) {
    let entrypoint = binding.entrypoint.as_str();
    let sessions = &binding.sessions;

    if sessions.is_empty() {
        findings.push(MacroRecorderFinding::blocker(
            MacroRecorderFindingKind::EntrypointPanelEmpty,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} entrypoint binds a panel with no sessions"),
        ));
        return;
    }

    // The panel must project one replay resolution per session.
    if binding.replay_resolutions.len() != sessions.len() {
        findings.push(MacroRecorderFinding::blocker(
            MacroRecorderFindingKind::ReplayResolutionProjectionInconsistent,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} panel projects {} resolutions for {} sessions",
                binding.replay_resolutions.len(),
                sessions.len()
            ),
        ));
    }

    for (index, session) in sessions.iter().enumerate() {
        let subject = format!("{entrypoint}:{}", session.session_id);
        let resolved = session.resolved_replay_class();

        // Replay must resolve current context, never implying stale authority.
        if !session.replay_consistent() {
            let has_no_blocker = session
                .current_replay_blockers
                .contains(&MacroReplayBlocker::NoBlockerPresent);
            let pairing_ok = if resolved == MacroReplayActionClass::ReplayInDeclaredScope {
                session.current_replay_blockers == [MacroReplayBlocker::NoBlockerPresent]
            } else {
                !has_no_blocker
            };
            if !pairing_ok {
                findings.push(MacroRecorderFinding::blocker(
                    MacroRecorderFindingKind::ReplayImpliesStaleContext,
                    Some(subject.clone()),
                    format!(
                        "session {} on {entrypoint} resolves {} but its blockers imply stale context",
                        session.session_id,
                        resolved.as_str()
                    ),
                ));
            }
            if session.imported_from_repository_content
                && resolved != MacroReplayActionClass::BlockedImportedFromRepositoryContent
            {
                findings.push(MacroRecorderFinding::blocker(
                    MacroRecorderFindingKind::RepositoryContentDefinesMacro,
                    Some(subject.clone()),
                    format!(
                        "repository-imported session {} on {entrypoint} offers a replay",
                        session.session_id
                    ),
                ));
            }
            if session.has_unsupported_command() && resolved.is_admissible() {
                findings.push(MacroRecorderFinding::blocker(
                    MacroRecorderFindingKind::ReplayNotFailClosedOnContextMismatch,
                    Some(subject.clone()),
                    format!(
                        "session {} on {entrypoint} captured an unsupported command but does not fail closed",
                        session.session_id
                    ),
                ));
            }
        }

        // A repository-imported macro is never an executable macro.
        if session.imported_from_repository_content {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::RepositoryContentDefinesMacro,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} was imported from repository content",
                    session.session_id
                ),
            ));
        }

        // An unsupported command must block save: a saved macro carries none.
        if session.has_unsupported_command() && session.disposition_class.mints_manifest() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::UnsupportedCommandNotBlocked,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} saved a macro that carries an unsupported command",
                    session.session_id
                ),
            ));
        }

        // Promotion to a recipe must be explicit when a macro crosses scope.
        if !session.promotion_consistent() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::PromotionNotExplicitForCrossScope,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} crosses scope without an explicit promotion path",
                    session.session_id
                ),
            ));
        }

        // The macro must be profile-local by default and never repository-defined.
        if !session.profile_local_default_consistent() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::ProfileLocalDefaultViolated,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} is not profile-local by default",
                    session.session_id
                ),
            ));
        }

        // The session must capture UI / editor state only with constrained labels.
        if !session.safety_labels_constrained() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::AmbientOrManagedOnlyCapture,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} projects a label outside macro_safe / ui_only",
                    session.session_id
                ),
            ));
        }

        // The save-or-discard disposition must match the minted manifest.
        if !session.disposition_consistent() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::ReplayResolutionProjectionInconsistent,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} disposition disagrees with its minted manifest",
                    session.session_id
                ),
            ));
        }

        // No raw secret may appear in a macro session.
        if !session.captures_are_opaque() {
            findings.push(MacroRecorderFinding::blocker(
                MacroRecorderFindingKind::RawSecretMaterialInSession,
                Some(subject.clone()),
                format!(
                    "session {} on {entrypoint} carries a non-opaque capture reference",
                    session.session_id
                ),
            ));
        }

        // The projected replay resolution must quote the same resolution as the session.
        if let Some(resolution) = binding.replay_resolutions.get(index) {
            let expected = session.resolve_replay(resolution.resolved_at.clone());
            if resolution != &expected {
                findings.push(MacroRecorderFinding::blocker(
                    MacroRecorderFindingKind::ReplayResolutionProjectionInconsistent,
                    Some(subject.clone()),
                    format!(
                        "the projected replay resolution for {} on {entrypoint} disagrees with the session",
                        session.session_id
                    ),
                ));
            }
        }
    }
}

fn promotion_state_for_findings(
    findings: &[MacroRecorderFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == MacroRecorderFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == MacroRecorderFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn packet_digest(consumer_bindings: &[MacroRecorderConsumerBinding]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for binding in consumer_bindings {
        tokens.push(binding.entrypoint.as_str().to_owned());
        for session in &binding.sessions {
            tokens.push(session.session_id.clone());
        }
    }
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[String]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

// ---------------------------------------------------------------------------
// Seeds
// ---------------------------------------------------------------------------

fn s(value: &str) -> String {
    value.to_owned()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn digest(hex: &str) -> ContentAddress {
    ContentAddress {
        digest_algorithm: s("sha256"),
        digest_hex: s(hex),
        digest_size_bytes: 32,
    }
}

fn command(
    command_id: &str,
    surface_class: RecordedSurfaceClass,
    support_class: CapturedCommandSupportClass,
    replay_posture_class: ReplayPostureClass,
    hex: &str,
    captured_at: &str,
    label: &str,
) -> CapturedCommand {
    CapturedCommand {
        command_id: s(command_id),
        surface_class,
        support_class,
        replay_posture_class,
        state_digest: digest(hex),
        captured_at: s(captured_at),
        label: s(label),
    }
}

/// The two safety labels every recorded macro projects.
fn macro_safety_labels() -> Vec<AutomationSafetyLabelId> {
    vec![
        AutomationSafetyLabelId::MacroSafe,
        AutomationSafetyLabelId::UiOnly,
    ]
}

/// Existing contracts the first-consumers packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        MACRO_SESSION_SCHEMA_REF,
        RECIPE_MANIFEST_SCHEMA_REF,
        RUN_RECORD_SCHEMA_REF,
        CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
        AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF,
        "docs/m5/recipe-builder-and-macro-contract.md",
    ])
}

/// Builds the seeded macro-recorder panel one first consumer renders.
///
/// Each panel carries the latest session first. Across the six panels every recorder
/// state appears, the replayable, reconcilable, and a representative set of
/// fail-closed replay states appear, and the discard, unsupported-command,
/// cross-scope-promotion, and repository-import paths are exercised, so the freeze
/// covers the cross-surface vocabulary.
pub fn seeded_consumer_panel(entrypoint: RecipeBuilderEntrypoint) -> Vec<MacroRecorderSession> {
    use CapturedCommandSupportClass::{SupportedUiOrEditorState, UnsupportedRunsProcess};
    use MacroPromotionAffordanceClass::{NotPromotableUiOnly, PromotableToDeclarativeRecipe};
    use MacroRecorderStateClass::{Discarded, PromotedToRecipe, Recording, Stopped};
    use MacroReplayBlocker::{
        ActiveContextReconcilable, NoBlockerPresent, PromotionRequiredCrossesScope,
        SupportedCommandSetChanged, UnsupportedCommandCaptured,
    };
    use RecordedSurfaceClass::{
        EditorMultiCursorEdits, EditorSelectionAndCursorState, UiPanelOpenCloseState,
    };
    use ReplayPostureClass::ReplayUiOrEditorStateOnly;
    use SessionDispositionClass::{
        DiscardedNoMacroMinted, SavedAndPromotedToRecipe, SavedAsProfileLocalMacro, StillRecording,
    };

    match entrypoint {
        // Notebook: a saved profile-local macro that replays clean, plus a still
        // recording session demonstrating the active strip; both stay in scope.
        RecipeBuilderEntrypoint::Notebook => vec![
            MacroRecorderSession {
                session_id: s("macro-session:notebook-tidy-cells:v1"),
                entrypoint,
                macro_session_schema_version: 1,
                title: s("Tidy notebook cells"),
                summary: s("Records the selection and multi-cursor edits that tidy the visible cells."),
                recorder_state_class: Stopped,
                disposition_class: SavedAsProfileLocalMacro,
                captured_commands: vec![
                    command(
                        "capture:select-cells",
                        EditorSelectionAndCursorState,
                        SupportedUiOrEditorState,
                        ReplayUiOrEditorStateOnly,
                        "1111111111111111111111111111111111111111111111111111111111111111",
                        "2026-06-18T00:00:00Z",
                        "Select the visible notebook cells",
                    ),
                    command(
                        "capture:tidy-edits",
                        EditorMultiCursorEdits,
                        SupportedUiOrEditorState,
                        ReplayUiOrEditorStateOnly,
                        "2222222222222222222222222222222222222222222222222222222222222222",
                        "2026-06-18T00:00:01Z",
                        "Apply the multi-cursor tidy edits",
                    ),
                ],
                declared_target_scope_class: TargetScopeClass::ActiveDocumentScope,
                storage_scope_class: MacroStorageScopeClass::UserScopeLocalOnly,
                projected_safety_labels: macro_safety_labels(),
                promotion_affordance_class: PromotableToDeclarativeRecipe,
                replay_count: 4,
                current_replay_blockers: vec![NoBlockerPresent],
                imported_from_repository_content: false,
                redaction_class: MacroRedactionClass::MetadataSafeDefault,
                resulting_macro_manifest_ref: Some(s("macro:notebook-tidy-cells:1")),
                manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
                recorded_at: s("2026-06-18T00:00:02Z"),
                last_replayed_at: Some(s("2026-06-18T01:00:00Z")),
            },
            MacroRecorderSession {
                session_id: s("macro-session:notebook-live:v1"),
                entrypoint,
                macro_session_schema_version: 1,
                title: s("Recording in progress"),
                summary: s("An active recording capturing panel and selection state in the notebook."),
                recorder_state_class: Recording,
                disposition_class: StillRecording,
                captured_commands: vec![command(
                    "capture:open-outline",
                    UiPanelOpenCloseState,
                    SupportedUiOrEditorState,
                    ReplayUiOrEditorStateOnly,
                    "3333333333333333333333333333333333333333333333333333333333333333",
                    "2026-06-18T00:00:00Z",
                    "Open the notebook outline panel",
                )],
                declared_target_scope_class: TargetScopeClass::ActiveEditorGroupScope,
                storage_scope_class: MacroStorageScopeClass::UserScopeLocalOnly,
                projected_safety_labels: macro_safety_labels(),
                promotion_affordance_class: NotPromotableUiOnly,
                replay_count: 0,
                current_replay_blockers: vec![ActiveContextReconcilable],
                imported_from_repository_content: false,
                redaction_class: MacroRedactionClass::MetadataSafeDefault,
                resulting_macro_manifest_ref: None,
                manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
                recorded_at: s("2026-06-18T00:00:01Z"),
                last_replayed_at: None,
            },
        ],
        // Task/test/debug: a discarded recording that captured an unsupported
        // command (a process launch), so the panel proves unsupported commands are
        // flagged, block save, and the discard mints no macro.
        RecipeBuilderEntrypoint::TaskTestDebug => vec![MacroRecorderSession {
            session_id: s("macro-session:debug-scratch:v1"),
            entrypoint,
            macro_session_schema_version: 1,
            title: s("Scratch debug recording"),
            summary: s("A recording the user discarded after the recorder flagged a process launch."),
            recorder_state_class: Discarded,
            disposition_class: DiscardedNoMacroMinted,
            captured_commands: vec![
                command(
                    "capture:focus-editor",
                    EditorSelectionAndCursorState,
                    SupportedUiOrEditorState,
                    ReplayUiOrEditorStateOnly,
                    "4444444444444444444444444444444444444444444444444444444444444444",
                    "2026-06-18T00:00:00Z",
                    "Focus the editor on the failing line",
                ),
                command(
                    "capture:run-debug-process",
                    UiPanelOpenCloseState,
                    UnsupportedRunsProcess,
                    ReplayUiOrEditorStateOnly,
                    "5555555555555555555555555555555555555555555555555555555555555555",
                    "2026-06-18T00:00:01Z",
                    "Launch the debug process (unsupported in a macro)",
                ),
            ],
            declared_target_scope_class: TargetScopeClass::ActiveDocumentScope,
            storage_scope_class: MacroStorageScopeClass::WorkspaceScopeLocalOnly,
            projected_safety_labels: macro_safety_labels(),
            promotion_affordance_class: NotPromotableUiOnly,
            replay_count: 0,
            current_replay_blockers: vec![UnsupportedCommandCaptured],
            imported_from_repository_content: false,
            redaction_class: MacroRedactionClass::MetadataSafeDefault,
            resulting_macro_manifest_ref: None,
            manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
            recorded_at: s("2026-06-18T00:00:02Z"),
            last_replayed_at: None,
        }],
        // Request/API: a saved macro whose supported-command set changed since
        // capture, so replay fails closed until the user reconciles.
        RecipeBuilderEntrypoint::RequestApi => vec![MacroRecorderSession {
            session_id: s("macro-session:request-template-fill:v1"),
            entrypoint,
            macro_session_schema_version: 1,
            title: s("Fill the request template"),
            summary: s("Records the editor edits that fill a request template; replay needs the command set re-resolved."),
            recorder_state_class: Stopped,
            disposition_class: SavedAsProfileLocalMacro,
            captured_commands: vec![command(
                "capture:fill-template",
                EditorMultiCursorEdits,
                SupportedUiOrEditorState,
                ReplayUiOrEditorStateOnly,
                "6666666666666666666666666666666666666666666666666666666666666666",
                "2026-06-18T00:00:00Z",
                "Fill the request body template fields",
            )],
            declared_target_scope_class: TargetScopeClass::SingleFileScope,
            storage_scope_class: MacroStorageScopeClass::WorkspaceScopeLocalOnly,
            projected_safety_labels: macro_safety_labels(),
            promotion_affordance_class: PromotableToDeclarativeRecipe,
            replay_count: 2,
            current_replay_blockers: vec![SupportedCommandSetChanged],
            imported_from_repository_content: false,
            redaction_class: MacroRedactionClass::MetadataSafeDefault,
            resulting_macro_manifest_ref: Some(s("macro:request-template-fill:1")),
            manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
            recorded_at: s("2026-06-18T00:00:01Z"),
            last_replayed_at: Some(s("2026-06-17T00:00:00Z")),
        }],
        // Package: a macro that crosses files, so it is promoted to a declarative
        // recipe and direct replay fails closed pending the explicit promotion.
        RecipeBuilderEntrypoint::Package => vec![MacroRecorderSession {
            session_id: s("macro-session:bump-versions:v1"),
            entrypoint,
            macro_session_schema_version: 1,
            title: s("Bump versions across manifests"),
            summary: s("Records edits that span multiple manifests; crossing files requires recipe promotion."),
            recorder_state_class: PromotedToRecipe,
            disposition_class: SavedAndPromotedToRecipe,
            captured_commands: vec![command(
                "capture:bump-edits",
                EditorMultiCursorEdits,
                SupportedUiOrEditorState,
                ReplayUiOrEditorStateOnly,
                "7777777777777777777777777777777777777777777777777777777777777777",
                "2026-06-18T00:00:00Z",
                "Apply the version-bump edits across manifests",
            )],
            declared_target_scope_class: TargetScopeClass::MultiFileScope,
            storage_scope_class: MacroStorageScopeClass::WorkspaceScopeLocalOnly,
            projected_safety_labels: macro_safety_labels(),
            promotion_affordance_class: PromotableToDeclarativeRecipe,
            replay_count: 1,
            current_replay_blockers: vec![PromotionRequiredCrossesScope],
            imported_from_repository_content: false,
            redaction_class: MacroRedactionClass::MetadataSafeDefault,
            resulting_macro_manifest_ref: Some(s("macro:bump-versions:1")),
            manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
            recorded_at: s("2026-06-18T00:00:01Z"),
            last_replayed_at: None,
        }],
        // Incident: a saved macro whose replay is admissible after the user
        // explicitly reconciles the drifted active context.
        RecipeBuilderEntrypoint::Incident => vec![MacroRecorderSession {
            session_id: s("macro-session:incident-layout:v1"),
            entrypoint,
            macro_session_schema_version: 1,
            title: s("Lay out the incident workspace"),
            summary: s("Records the panel and focus moves that lay out the incident workspace."),
            recorder_state_class: Stopped,
            disposition_class: SavedAsProfileLocalMacro,
            captured_commands: vec![command(
                "capture:layout-panels",
                UiPanelOpenCloseState,
                SupportedUiOrEditorState,
                ReplayUiOrEditorStateOnly,
                "8888888888888888888888888888888888888888888888888888888888888888",
                "2026-06-18T00:00:00Z",
                "Open and arrange the incident panels",
            )],
            declared_target_scope_class: TargetScopeClass::ActiveEditorGroupScope,
            storage_scope_class: MacroStorageScopeClass::UserScopeLocalOnly,
            projected_safety_labels: macro_safety_labels(),
            promotion_affordance_class: NotPromotableUiOnly,
            replay_count: 6,
            current_replay_blockers: vec![ActiveContextReconcilable],
            imported_from_repository_content: false,
            redaction_class: MacroRedactionClass::MetadataSafeDefault,
            resulting_macro_manifest_ref: Some(s("macro:incident-layout:1")),
            manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
            recorded_at: s("2026-06-18T00:00:01Z"),
            last_replayed_at: Some(s("2026-06-18T02:00:00Z")),
        }],
        // AI assistant: an AI-suggested recording promotable to a recipe; it stays
        // in scope and replays clean, proving AI macros use the same lane.
        RecipeBuilderEntrypoint::AiAssistant => vec![MacroRecorderSession {
            session_id: s("macro-session:ai-rename-symbol:v1"),
            entrypoint,
            macro_session_schema_version: 1,
            title: s("Rename the symbol in view"),
            summary: s("An AI-suggested recording of the rename edits across the visible block."),
            recorder_state_class: Stopped,
            disposition_class: SavedAsProfileLocalMacro,
            captured_commands: vec![command(
                "capture:rename-edits",
                EditorMultiCursorEdits,
                SupportedUiOrEditorState,
                ReplayUiOrEditorStateOnly,
                "9999999999999999999999999999999999999999999999999999999999999999",
                "2026-06-18T00:00:00Z",
                "Apply the rename edits across the visible block",
            )],
            declared_target_scope_class: TargetScopeClass::ActiveSelectionScope,
            storage_scope_class: MacroStorageScopeClass::UserScopeLocalOnly,
            projected_safety_labels: macro_safety_labels(),
            promotion_affordance_class: PromotableToDeclarativeRecipe,
            replay_count: 3,
            current_replay_blockers: vec![NoBlockerPresent],
            imported_from_repository_content: false,
            redaction_class: MacroRedactionClass::MetadataSafeDefault,
            resulting_macro_manifest_ref: Some(s("macro:ai-rename-symbol:1")),
            manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
            recorded_at: s("2026-06-18T00:00:01Z"),
            last_replayed_at: Some(s("2026-06-18T03:00:00Z")),
        }],
    }
}

/// The reviewable summary one consumer's panel carries.
fn panel_summary(entrypoint: RecipeBuilderEntrypoint) -> &'static str {
    match entrypoint {
        RecipeBuilderEntrypoint::Notebook => {
            "A saved tidy macro that replays clean and a live recording with an active strip."
        }
        RecipeBuilderEntrypoint::TaskTestDebug => {
            "A discarded recording whose unsupported process launch was flagged and blocked save."
        }
        RecipeBuilderEntrypoint::RequestApi => {
            "A saved template-fill macro whose replay fails closed until the command set is reconciled."
        }
        RecipeBuilderEntrypoint::Package => {
            "A cross-file macro promoted to a recipe; direct replay fails closed pending promotion."
        }
        RecipeBuilderEntrypoint::Incident => {
            "A saved layout macro whose replay is admissible after an explicit context reconciliation."
        }
        RecipeBuilderEntrypoint::AiAssistant => {
            "An AI-suggested rename macro that stays in scope and replays clean."
        }
    }
}

/// Builds the canonical stable first-consumers input.
pub fn current_macro_recorder_first_consumers_input() -> MacroRecorderFirstConsumersInput {
    let consumer_bindings = RecipeBuilderEntrypoint::ALL
        .into_iter()
        .map(|entrypoint| {
            MacroRecorderConsumerBinding::from_sessions(
                entrypoint,
                seeded_consumer_panel(entrypoint),
                panel_summary(entrypoint),
            )
        })
        .collect();
    MacroRecorderFirstConsumersInput {
        packet_id: MACRO_RECORDER_FIRST_CONSUMERS_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        consumer_bindings,
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: MacroRecorderInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_macro_recorder_first_consumers_packet() -> MacroRecorderFirstConsumersPacket {
    MacroRecorderFirstConsumersPacket::materialize(current_macro_recorder_first_consumers_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_macro_recorder_first_consumers_packet(
    packet: &MacroRecorderFirstConsumersPacket,
) -> Result<(), Vec<MacroRecorderFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Convenience accessor for a single seeded session by entrypoint (latest session).
pub fn seeded_macro_recorder_session(entrypoint: RecipeBuilderEntrypoint) -> MacroRecorderSession {
    seeded_consumer_panel(entrypoint)
        .into_iter()
        .next()
        .expect("panel has at least one session")
}

/// Worked example: the latest notebook macro exported for round-trip review.
///
/// The notebook macro is a saved, profile-local recording that replays clean, so the
/// round-trip proves replay and scope truth survive export, history, and support.
pub fn seeded_macro_session_export_roundtrip() -> MacroSessionExport {
    seeded_macro_recorder_session(RecipeBuilderEntrypoint::Notebook)
        .export("export:notebook-tidy-cells:v1", "2026-06-18T00:01:00Z")
}

/// Worked example: the package macro whose cross-file scope forces recipe promotion.
pub fn seeded_cross_scope_promotion_session() -> MacroRecorderSession {
    seeded_macro_recorder_session(RecipeBuilderEntrypoint::Package)
}

/// Worked example: the discarded debug recording whose unsupported command blocked save.
pub fn seeded_unsupported_command_session() -> MacroRecorderSession {
    seeded_macro_recorder_session(RecipeBuilderEntrypoint::TaskTestDebug)
}
