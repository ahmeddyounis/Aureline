//! M5 macro-replay review packet: run-capable / cross-file macro downgrades and
//! recipe-promotion paths with exact command lineage across M5 modal workflows.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable
//! interaction across every new M5 surface — editor, notebook, data/API,
//! preview, docs, review, runtime, and companion-adjacent panes. The frozen
//! keyboard-continuity matrix
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! pins those surfaces to their canonical interaction vocabulary and requires
//! that *mode changes and macro replay are explicit*. This module discharges the
//! macro-replay half of that contract: it takes the keyboard-mode packets and
//! makes macro replay **safe** on the new M5 surfaces by routing broad,
//! run-capable, or cross-file sequences through review, downgrade, or
//! recipe-promotion instead of replaying hidden side effects.
//!
//! * a [`MacroReplayReviewRecord`] binds a claimed M5 surface (keyed by a
//!   [`KeyboardSurfaceKind`] and a non-display [`KeyboardSurfaceSubject`]) to one
//!   replay attempt: the [`MacroSourceRegister`] it was recorded into, the
//!   [`MacroScopeClass`] (with file / surface span counts) it targets, the
//!   [`MacroTimingClass`] it depends on, an ordered exact [`MacroCommandStep`]
//!   command lineage, and the resolved [`MacroReplayOutcomeClass`];
//! * macro replay is **never silent when it is broad**: a record whose scope
//!   crosses files or surfaces, whose lineage invokes a run-capable / elevated
//!   command, whose timing is unstable, whose lineage carries an unmapped or
//!   unsafe step, or whose review proof is stale or missing fires one or more
//!   [`MacroReplayTrigger`]s. Each trigger imposes a minimum-safety floor on the
//!   outcome, so a triggered record can never resolve to
//!   [`MacroReplayOutcomeClass::ExactReplayLocalEditorOnly`]; it must open
//!   review, downgrade to observe-no-mutation, promote to a declarative recipe,
//!   or be rejected;
//! * exact command lineage is **preserved, not collapsed**: every record keeps
//!   the invoked `command_id` / `command_revision_ref` chain and the source
//!   register so help, migration, and support tooling can tell exactly what
//!   command sequence a replay invoked and whether it ran exactly, in a
//!   downgraded form, or as a promoted recipe — instead of an opaque text blob.
//!
//! [`MacroReplayReviewPacket::validate`] refuses a packet that lets a broad macro
//! replay silently, that lowers an outcome below its required safety floor, that
//! collapses the lineage into opaque text, that drops the source register or
//! target scope, or that lets a provider-linked surface read as a locally
//! verified replay.
//!
//! Raw keystroke buffers, raw editor buffer bytes, raw shell fragments, raw
//! provider payloads, file contents, private paths, and credentials never cross
//! this boundary; the packet carries only typed class tokens, booleans, opaque
//! ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json`](../../../../schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md`](../../../../docs/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/`](../../../../fixtures/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this consumer binds, so product, help, support,
// and migration surfaces can name those types through this module rather than
// reaching into the matrix module by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, KeyboardSurfaceKind, KeyboardSurfaceSubject,
    SurfaceOriginClass,
};

/// Stable record-kind tag carried by [`MacroReplayReviewPacket`].
pub const MACRO_REPLAY_REVIEW_RECORD_KIND: &str =
    "m5_macro_replay_review_run_capable_cross_file_downgrade_recipe_packet";

/// Schema version for the macro-replay review packet.
pub const MACRO_REPLAY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MACRO_REPLAY_REVIEW_SCHEMA_REF: &str =
    "schemas/interaction/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.schema.json";

/// Repo-relative path of the contract doc.
pub const MACRO_REPLAY_REVIEW_DOC_REF: &str =
    "docs/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md";

/// Repo-relative path of the checked support-export artifact.
pub const MACRO_REPLAY_REVIEW_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MACRO_REPLAY_REVIEW_SUMMARY_REF: &str =
    "artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion.md";

/// Repo-relative path of the protected fixture directory.
pub const MACRO_REPLAY_REVIEW_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion";

/// Source contract ref of the frozen keyboard-continuity matrix this packet binds.
pub const KEYBOARD_CONTINUITY_MATRIX_DOC_REF: &str =
    "docs/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md";

/// Source contract ref of the recorded-macro / declarative-recipe boundary.
pub const RECIPE_AND_MACRO_CONTRACT_REF: &str = "docs/automation/recipe_and_macro_contract.md";

/// Resolved outcome of one macro-replay attempt on a claimed M5 surface.
///
/// Only [`Self::ExactReplayLocalEditorOnly`] dispatches the recorded sequence
/// silently against the live surface; every other outcome opens review,
/// downgrades to a no-mutation observe pass, promotes the macro into a reviewable
/// declarative recipe, or rejects the replay. The [`Self::safety_rank`] orders
/// the outcomes so a triggered record can be held at or above a required floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayOutcomeClass {
    /// Replay proceeds silently against a single editor buffer on one surface.
    ExactReplayLocalEditorOnly,
    /// Replay opens a review / preview pass before any apply.
    ReviewRequiredBeforeApply,
    /// Replay is downgraded to an observe-only run that performs no mutation.
    DowngradedToObserverNoMutation,
    /// Replay is promoted to a declarative recipe and re-authored with lineage.
    PromotedToDeclarativeRecipe,
    /// Replay is rejected as unsafe.
    RejectedUnsafeReplay,
}

impl MacroReplayOutcomeClass {
    /// Every outcome class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactReplayLocalEditorOnly,
        Self::ReviewRequiredBeforeApply,
        Self::DowngradedToObserverNoMutation,
        Self::PromotedToDeclarativeRecipe,
        Self::RejectedUnsafeReplay,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReplayLocalEditorOnly => "exact_replay_local_editor_only",
            Self::ReviewRequiredBeforeApply => "review_required_before_apply",
            Self::DowngradedToObserverNoMutation => "downgraded_to_observer_no_mutation",
            Self::PromotedToDeclarativeRecipe => "promoted_to_declarative_recipe",
            Self::RejectedUnsafeReplay => "rejected_unsafe_replay",
        }
    }

    /// Monotonic safety rank; higher is a stronger / more restrictive outcome, so
    /// a triggered record must hold an outcome whose rank meets its floor.
    pub const fn safety_rank(self) -> u8 {
        match self {
            Self::ExactReplayLocalEditorOnly => 0,
            Self::ReviewRequiredBeforeApply => 1,
            Self::DowngradedToObserverNoMutation => 2,
            Self::PromotedToDeclarativeRecipe => 3,
            Self::RejectedUnsafeReplay => 4,
        }
    }

    /// Whether this outcome dispatches the recorded sequence silently.
    pub const fn is_silent_exact(self) -> bool {
        matches!(self, Self::ExactReplayLocalEditorOnly)
    }

    /// Whether this outcome must cite a `review_reason_label`.
    pub const fn requires_review_reason(self) -> bool {
        matches!(self, Self::ReviewRequiredBeforeApply)
    }

    /// Whether this outcome must cite a `downgrade_target_label`.
    pub const fn requires_downgrade_target(self) -> bool {
        matches!(self, Self::DowngradedToObserverNoMutation)
    }

    /// Whether this outcome must cite a `promoted_recipe_ref`.
    pub const fn requires_promoted_recipe_ref(self) -> bool {
        matches!(self, Self::PromotedToDeclarativeRecipe)
    }

    /// Whether this outcome must cite a `rejection_reason_label`.
    pub const fn requires_rejection_reason(self) -> bool {
        matches!(self, Self::RejectedUnsafeReplay)
    }
}

/// Why a recorded register was opened: the register class a macro was captured
/// into. Preserving the register keeps the replay attributable rather than
/// collapsing it into opaque text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRegisterClass {
    /// A named (Vim-style `"a`..`"z`) register.
    NamedRegister,
    /// A numbered / yank-ring register.
    NumberedRegister,
    /// An append register that accreted across recordings.
    AppendRegister,
    /// A register linked to the system clipboard route.
    ClipboardLinkedRegister,
    /// The transient recording buffer (the `q`-style live capture).
    RecordingBufferRegister,
}

impl MacroRegisterClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedRegister => "named_register",
            Self::NumberedRegister => "numbered_register",
            Self::AppendRegister => "append_register",
            Self::ClipboardLinkedRegister => "clipboard_linked_register",
            Self::RecordingBufferRegister => "recording_buffer_register",
        }
    }
}

/// The target scope a macro replay touches. A scope that crosses files or
/// surfaces can never replay silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroScopeClass {
    /// One selection in one file on one surface.
    SingleSurfaceSingleFile,
    /// A multi-selection / multi-cursor edit, still in one file on one surface.
    SingleSurfaceMultiSelection,
    /// Multiple files within one surface.
    CrossFileWithinSurface,
    /// A span that crosses surfaces (e.g. notebook cell into editor file).
    CrossSurfaceSpan,
    /// A workspace-wide span (broad project-level rewrite).
    WorkspaceWideSpan,
}

impl MacroScopeClass {
    /// Every scope class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SingleSurfaceSingleFile,
        Self::SingleSurfaceMultiSelection,
        Self::CrossFileWithinSurface,
        Self::CrossSurfaceSpan,
        Self::WorkspaceWideSpan,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSurfaceSingleFile => "single_surface_single_file",
            Self::SingleSurfaceMultiSelection => "single_surface_multi_selection",
            Self::CrossFileWithinSurface => "cross_file_within_surface",
            Self::CrossSurfaceSpan => "cross_surface_span",
            Self::WorkspaceWideSpan => "workspace_wide_span",
        }
    }

    /// Whether this scope crosses more than one file.
    pub const fn is_cross_file(self) -> bool {
        matches!(
            self,
            Self::CrossFileWithinSurface | Self::CrossSurfaceSpan | Self::WorkspaceWideSpan
        )
    }

    /// Whether this scope crosses surfaces or spans the workspace, so the replay
    /// must take a recipe-promotion or reject path rather than a guarded replay.
    pub const fn is_cross_surface_or_workspace(self) -> bool {
        matches!(self, Self::CrossSurfaceSpan | Self::WorkspaceWideSpan)
    }

    /// Whether `files_touched` and `surfaces_spanned` are consistent with this
    /// scope class.
    pub fn counts_consistent(self, files_touched: u32, surfaces_spanned: u32) -> bool {
        match self {
            Self::SingleSurfaceSingleFile => files_touched == 1 && surfaces_spanned == 1,
            Self::SingleSurfaceMultiSelection => files_touched == 1 && surfaces_spanned == 1,
            Self::CrossFileWithinSurface => files_touched >= 2 && surfaces_spanned == 1,
            Self::CrossSurfaceSpan => surfaces_spanned >= 2,
            Self::WorkspaceWideSpan => files_touched >= 2 && surfaces_spanned >= 2,
        }
    }
}

/// Timing posture a macro replay depends on. Unstable timing cannot replay
/// exactly because a later run may diverge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroTimingClass {
    /// Deterministic timing — every replay reproduces the recorded steps.
    StableDeterministic,
    /// Depends on asynchronous output (a cell result, a build, a fetch).
    DependsOnAsyncOutput,
    /// Depends on external state outside the recorded sequence.
    DependsOnExternalState,
}

impl MacroTimingClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableDeterministic => "stable_deterministic",
            Self::DependsOnAsyncOutput => "depends_on_async_output",
            Self::DependsOnExternalState => "depends_on_external_state",
        }
    }

    /// Whether this timing is unstable, so exact replay would be unsafe.
    pub const fn is_unstable(self) -> bool {
        !matches!(self, Self::StableDeterministic)
    }
}

/// Command-graph lineage class of one recorded step. An unmapped or unsafe step
/// forces the whole replay to reject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroCommandLineageClass {
    /// Resolves to a core Aureline command id on the command graph.
    CoreCommand,
    /// Resolves to a command imported via a bridge / preset.
    ImportedCommand,
    /// Resolves to a command published by an extension.
    ExtensionCommand,
    /// Resolves to an AI-tool handle on the command graph.
    AiToolHandle,
    /// Resolves to a CLI verb known to the command graph.
    CliVerb,
    /// Resolves to a run-capable command (executes code / a build / a request).
    RunCapableCommand,
    /// No mapping found — the raw key chord is unsafe and forces a reject.
    UnmappedKeystrokeUnsafe,
}

impl MacroCommandLineageClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreCommand => "core_command",
            Self::ImportedCommand => "imported_command",
            Self::ExtensionCommand => "extension_command",
            Self::AiToolHandle => "ai_tool_handle",
            Self::CliVerb => "cli_verb",
            Self::RunCapableCommand => "run_capable_command",
            Self::UnmappedKeystrokeUnsafe => "unmapped_keystroke_unsafe",
        }
    }

    /// Whether this lineage class requires the step to cite an `ai_tool_handle_ref`.
    pub const fn requires_ai_tool_handle_ref(self) -> bool {
        matches!(self, Self::AiToolHandle)
    }

    /// Whether this lineage class is intrinsically run-capable.
    pub const fn is_run_capable(self) -> bool {
        matches!(self, Self::RunCapableCommand)
    }

    /// Whether this lineage class is an unmapped / unsafe step.
    pub const fn is_unmapped_or_unsafe(self) -> bool {
        matches!(self, Self::UnmappedKeystrokeUnsafe)
    }
}

/// The write class one recorded step touches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroWriteClass {
    /// No write surface is touched.
    ReadOnly,
    /// One editor buffer on one surface is mutated.
    EditorBufferMutation,
    /// Multiple files / buffers are mutated.
    MultiFileMutation,
    /// Code / a build / a request is executed.
    RunCapableExecution,
    /// A network mutation is performed.
    NetworkMutation,
    /// A settings mutation is performed.
    SettingsMutation,
}

impl MacroWriteClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::EditorBufferMutation => "editor_buffer_mutation",
            Self::MultiFileMutation => "multi_file_mutation",
            Self::RunCapableExecution => "run_capable_execution",
            Self::NetworkMutation => "network_mutation",
            Self::SettingsMutation => "settings_mutation",
        }
    }

    /// Whether this write class executes code / a build / a request.
    pub const fn is_run_capable(self) -> bool {
        matches!(self, Self::RunCapableExecution)
    }

    /// Whether this write class is elevated beyond a local editor buffer mutation,
    /// so it cannot ride the silent exact-replay lane.
    pub const fn is_elevated(self) -> bool {
        matches!(
            self,
            Self::MultiFileMutation
                | Self::RunCapableExecution
                | Self::NetworkMutation
                | Self::SettingsMutation
        )
    }
}

/// Why a record's replay was held off the silent exact lane. Each trigger imposes
/// a minimum-safety floor; the chrome quotes the trigger verbatim instead of a
/// generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayTrigger {
    /// The macro scope crosses more than one file.
    CrossFileScope,
    /// The macro scope crosses surfaces or spans the workspace.
    CrossSurfaceOrWorkspaceScope,
    /// The lineage invokes a run-capable / elevated command.
    RunCapableOrElevatedCommand,
    /// The macro depends on unstable timing.
    UnstableTiming,
    /// The lineage carries an unmapped or unsafe step.
    UnmappedOrUnsafeStep,
    /// The review proof backing this record is stale or missing.
    StaleOrMissingReviewProof,
}

impl MacroReplayTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrossFileScope => "cross_file_scope",
            Self::CrossSurfaceOrWorkspaceScope => "cross_surface_or_workspace_scope",
            Self::RunCapableOrElevatedCommand => "run_capable_or_elevated_command",
            Self::UnstableTiming => "unstable_timing",
            Self::UnmappedOrUnsafeStep => "unmapped_or_unsafe_step",
            Self::StaleOrMissingReviewProof => "stale_or_missing_review_proof",
        }
    }

    /// Minimum outcome safety rank this trigger imposes.
    ///
    /// A cross-file scope, a run-capable command, or stale review proof requires at
    /// least a review pass; a cross-surface / workspace span or unstable timing
    /// requires recipe-promotion or a reject (a guarded in-place replay is not
    /// enough); an unmapped or unsafe step requires an outright reject.
    pub const fn minimum_outcome_rank(self) -> u8 {
        match self {
            Self::CrossFileScope
            | Self::RunCapableOrElevatedCommand
            | Self::StaleOrMissingReviewProof => {
                MacroReplayOutcomeClass::ReviewRequiredBeforeApply.safety_rank()
            }
            Self::CrossSurfaceOrWorkspaceScope | Self::UnstableTiming => {
                MacroReplayOutcomeClass::PromotedToDeclarativeRecipe.safety_rank()
            }
            Self::UnmappedOrUnsafeStep => {
                MacroReplayOutcomeClass::RejectedUnsafeReplay.safety_rank()
            }
        }
    }
}

/// One step in a macro's exact command lineage. The step keeps the command-graph
/// `command_id` / `command_revision_ref` so the invoked sequence is never reduced
/// to opaque text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroCommandStep {
    /// Stable opaque step id, unique within a record.
    pub step_id: String,
    /// Command-graph lineage class of this step.
    pub lineage_class: MacroCommandLineageClass,
    /// Stable command id on the command graph.
    pub command_id: String,
    /// Stable command revision ref on the command graph.
    pub command_revision_ref: String,
    /// Write class this step touches.
    pub write_class: MacroWriteClass,
    /// Whether this step executes code / a build / a request beyond a buffer edit.
    pub run_capable: bool,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Required when `lineage_class` is `ai_tool_handle`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_tool_handle_ref: Option<String>,
}

impl MacroCommandStep {
    /// Whether this step is run-capable / elevated, so it forces a non-exact
    /// outcome.
    pub fn is_run_capable_or_elevated(&self) -> bool {
        self.run_capable
            || self.lineage_class.is_run_capable()
            || self.write_class.is_run_capable()
            || self.write_class.is_elevated()
    }

    /// Whether this step is unmapped / unsafe, so it forces a reject.
    pub fn is_unmapped_or_unsafe(&self) -> bool {
        self.lineage_class.is_unmapped_or_unsafe()
    }

    /// Whether every required field is present and the lineage is exact.
    pub fn is_well_formed(&self) -> bool {
        if self.step_id.trim().is_empty()
            || self.command_id.trim().is_empty()
            || self.command_revision_ref.trim().is_empty()
            || self.display_label.trim().is_empty()
        {
            return false;
        }
        if self.lineage_class.requires_ai_tool_handle_ref() {
            return self
                .ai_tool_handle_ref
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty());
        }
        true
    }
}

/// The source register a macro was recorded into. Preserved so a replay can be
/// reconstructed from the same register a user typed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroSourceRegister {
    /// Opaque register token (e.g. `register:a`). Never a raw keystroke buffer.
    pub register_token: String,
    /// Register class.
    pub register_class: MacroRegisterClass,
    /// Reviewable label.
    pub display_label: String,
}

impl MacroSourceRegister {
    /// Whether the register carries the identity a reconstruction needs.
    pub fn is_valid(&self) -> bool {
        !self.register_token.trim().is_empty() && !self.display_label.trim().is_empty()
    }
}

/// Constructor input for [`MacroReplayReviewRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroReplayReviewRecordInput {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Source register the macro was recorded into.
    pub source_register: MacroSourceRegister,
    /// Target scope class.
    pub scope: MacroScopeClass,
    /// Files touched by the macro.
    pub files_touched: u32,
    /// Surfaces the macro spans.
    pub surfaces_spanned: u32,
    /// Timing posture the macro depends on.
    pub timing: MacroTimingClass,
    /// Ordered, exact command lineage.
    pub command_lineage: Vec<MacroCommandStep>,
    /// Reviewable lineage summary safe for export.
    pub lineage_summary: String,
    /// Reopenable verification proof backing the review outcome.
    pub verification: AxisVerification,
    /// The resolved replay outcome.
    pub outcome: MacroReplayOutcomeClass,
    /// Triggers recorded as firing for this record.
    pub fired_triggers: Vec<MacroReplayTrigger>,
    /// Required when `outcome` is `review_required_before_apply`.
    pub review_reason_label: Option<String>,
    /// Required when `outcome` is `downgraded_to_observer_no_mutation`.
    pub downgrade_target_label: Option<String>,
    /// Required when `outcome` is `promoted_to_declarative_recipe`.
    pub promoted_recipe_ref: Option<String>,
    /// Required when `outcome` is `rejected_unsafe_replay`.
    pub rejection_reason_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One macro-replay review record binding a claimed M5 surface to one resolved
/// replay outcome with exact command lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayReviewRecord {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Source register the macro was recorded into.
    pub source_register: MacroSourceRegister,
    /// Target scope class.
    pub scope: MacroScopeClass,
    /// Files touched by the macro.
    pub files_touched: u32,
    /// Surfaces the macro spans.
    pub surfaces_spanned: u32,
    /// Timing posture the macro depends on.
    pub timing: MacroTimingClass,
    /// Ordered, exact command lineage.
    pub command_lineage: Vec<MacroCommandStep>,
    /// Reviewable lineage summary safe for export.
    pub lineage_summary: String,
    /// Reopenable verification proof backing the review outcome.
    pub verification: AxisVerification,
    /// The resolved replay outcome.
    pub outcome: MacroReplayOutcomeClass,
    /// Triggers recorded as firing for this record. Must equal the computed set.
    pub fired_triggers: Vec<MacroReplayTrigger>,
    /// Required when `outcome` is `review_required_before_apply`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_reason_label: Option<String>,
    /// Required when `outcome` is `downgraded_to_observer_no_mutation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_target_label: Option<String>,
    /// Required when `outcome` is `promoted_to_declarative_recipe`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_recipe_ref: Option<String>,
    /// Required when `outcome` is `rejected_unsafe_replay`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejection_reason_label: Option<String>,
    /// Guardrail: record does not carry raw keystroke bytes.
    pub raw_keystroke_bytes_present: bool,
    /// Guardrail: record does not carry raw editor buffer bytes.
    pub raw_buffer_bytes_present: bool,
    /// Guardrail: record does not carry raw shell fragments.
    pub raw_shell_fragment_present: bool,
    /// Guardrail: the command lineage was not collapsed into opaque text.
    pub lineage_collapsed_to_opaque_text: bool,
    /// Guardrail: the record did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl MacroReplayReviewRecord {
    /// Builds a record from its input, defaulting the redaction guardrail flags to
    /// their safe values.
    pub fn new(input: MacroReplayReviewRecordInput) -> Self {
        Self {
            record_id: input.record_id,
            surface_kind: input.surface_kind,
            subject: input.subject,
            label_summary: input.label_summary,
            source_register: input.source_register,
            scope: input.scope,
            files_touched: input.files_touched,
            surfaces_spanned: input.surfaces_spanned,
            timing: input.timing,
            command_lineage: input.command_lineage,
            lineage_summary: input.lineage_summary,
            verification: input.verification,
            outcome: input.outcome,
            fired_triggers: input.fired_triggers,
            review_reason_label: input.review_reason_label,
            downgrade_target_label: input.downgrade_target_label,
            promoted_recipe_ref: input.promoted_recipe_ref,
            rejection_reason_label: input.rejection_reason_label,
            raw_keystroke_bytes_present: false,
            raw_buffer_bytes_present: false,
            raw_shell_fragment_present: false,
            lineage_collapsed_to_opaque_text: false,
            silent_authority_widening_taken: false,
            evidence_refs: input.evidence_refs,
            minted_at: input.minted_at,
        }
    }

    /// Whether replay for this record is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.origin_class.is_provider_or_imported()
    }

    /// Whether the verification proof backs a current review claim for this
    /// record's origin posture.
    pub fn review_proof_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// The set of triggers that actually fire for this record, computed from its
    /// scope, lineage, timing, and proof.
    pub fn computed_triggers(&self) -> BTreeSet<MacroReplayTrigger> {
        let mut triggers = BTreeSet::new();
        if self.scope.is_cross_file() {
            triggers.insert(MacroReplayTrigger::CrossFileScope);
        }
        if self.scope.is_cross_surface_or_workspace() {
            triggers.insert(MacroReplayTrigger::CrossSurfaceOrWorkspaceScope);
        }
        if self
            .command_lineage
            .iter()
            .any(MacroCommandStep::is_run_capable_or_elevated)
        {
            triggers.insert(MacroReplayTrigger::RunCapableOrElevatedCommand);
        }
        if self.timing.is_unstable() {
            triggers.insert(MacroReplayTrigger::UnstableTiming);
        }
        if self
            .command_lineage
            .iter()
            .any(MacroCommandStep::is_unmapped_or_unsafe)
        {
            triggers.insert(MacroReplayTrigger::UnmappedOrUnsafeStep);
        }
        if !self.review_proof_current() {
            triggers.insert(MacroReplayTrigger::StaleOrMissingReviewProof);
        }
        triggers
    }

    /// The recorded triggers as a set.
    pub fn recorded_triggers(&self) -> BTreeSet<MacroReplayTrigger> {
        self.fired_triggers.iter().copied().collect()
    }

    /// The minimum outcome safety rank this record must meet, given its triggers.
    pub fn required_floor_rank(&self) -> u8 {
        self.computed_triggers()
            .iter()
            .map(|trigger| trigger.minimum_outcome_rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether the replay must be held off the silent exact lane.
    pub fn must_not_replay_silently(&self) -> bool {
        self.required_floor_rank() > 0
    }

    /// Whether the recorded outcome meets the required safety floor.
    pub fn outcome_meets_floor(&self) -> bool {
        self.outcome.safety_rank() >= self.required_floor_rank()
    }

    /// Whether the recorded outcome silently replays a macro that must not.
    pub fn silently_replays_unsafe(&self) -> bool {
        self.outcome.is_silent_exact() && self.must_not_replay_silently()
    }

    /// Whether the recorded trigger set matches the computed set.
    pub fn triggers_consistent(&self) -> bool {
        self.recorded_triggers() == self.computed_triggers()
    }

    /// Whether the outcome carries exactly the detail field it requires.
    pub fn outcome_detail_consistent(&self) -> bool {
        let present = |opt: &Option<String>| {
            opt.as_deref()
                .is_some_and(|value| !value.trim().is_empty() && !label_is_generic(value))
        };
        let review_ok = if self.outcome.requires_review_reason() {
            present(&self.review_reason_label)
        } else {
            self.review_reason_label.is_none()
        };
        let downgrade_ok = if self.outcome.requires_downgrade_target() {
            present(&self.downgrade_target_label)
        } else {
            self.downgrade_target_label.is_none()
        };
        let promote_ok = if self.outcome.requires_promoted_recipe_ref() {
            present(&self.promoted_recipe_ref)
        } else {
            self.promoted_recipe_ref.is_none()
        };
        let reject_ok = if self.outcome.requires_rejection_reason() {
            present(&self.rejection_reason_label)
        } else {
            self.rejection_reason_label.is_none()
        };
        review_ok && downgrade_ok && promote_ok && reject_ok
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified replay, and a local surface never leans
    /// on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether the lineage is exact and not collapsed into opaque text.
    pub fn lineage_exact(&self) -> bool {
        !self.lineage_collapsed_to_opaque_text
            && !self.command_lineage.is_empty()
            && self
                .command_lineage
                .iter()
                .all(MacroCommandStep::is_well_formed)
    }

    /// Whether no raw boundary material is flagged present.
    pub fn no_raw_boundary_material(&self) -> bool {
        !self.raw_keystroke_bytes_present
            && !self.raw_buffer_bytes_present
            && !self.raw_shell_fragment_present
            && !self.silent_authority_widening_taken
    }

    /// Whether every field required to record this record is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.lineage_summary.trim().is_empty()
            && !self.minted_at.trim().is_empty()
            && self.subject.is_valid()
            && self.source_register.is_valid()
            && self
                .scope
                .counts_consistent(self.files_touched, self.surfaces_spanned)
            && self.lineage_exact()
            && self.verification.is_well_formed()
            && self.triggers_consistent()
            && !self.silently_replays_unsafe()
            && self.outcome_meets_floor()
            && self.outcome_detail_consistent()
            && self.imported_posture_consistent()
            && self.no_raw_boundary_material()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayGuardrails {
    /// Cross-file / run-capable macros never replay silently.
    pub broad_macros_never_replay_silently: bool,
    /// Exact command lineage is preserved rather than collapsed into opaque text.
    pub exact_command_lineage_preserved: bool,
    /// The source register and target scope are preserved on every record.
    pub source_register_and_scope_preserved: bool,
    /// Unstable-timing and cross-surface macros take the recipe-promotion path.
    pub unstable_or_cross_surface_promoted_to_recipe: bool,
    /// Provider-linked replays never read as a locally verified replay.
    pub provider_replays_never_read_as_local: bool,
    /// No general macro language or new editor core is introduced here.
    pub no_new_macro_language_introduced: bool,
}

impl MacroReplayGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.broad_macros_never_replay_silently
            && self.exact_command_lineage_preserved
            && self.source_register_and_scope_preserved
            && self.unstable_or_cross_surface_promoted_to_recipe
            && self.provider_replays_never_read_as_local
            && self.no_new_macro_language_introduced
    }
}

/// Consumer projection block: the surfaces that read this packet without cloning
/// macro-replay safety language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayConsumerProjection {
    /// Product surfaces ingest this packet.
    pub product_ingests_packet: bool,
    /// Help / migration guidance ingests the same packet.
    pub help_migration_ingests_packet: bool,
    /// Support / export tooling ingests the same packet.
    pub support_export_ingests_packet: bool,
    /// Release-control surfaces ingest the same packet.
    pub release_control_ingests_packet: bool,
    /// Help / migration / support can distinguish exact, downgraded, and promoted
    /// replay paths from this packet.
    pub exact_downgraded_promoted_distinguishable: bool,
}

impl MacroReplayConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_packet
            && self.help_migration_ingests_packet
            && self.support_export_ingests_packet
            && self.release_control_ingests_packet
            && self.exact_downgraded_promoted_distinguishable
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically forces records off exact replay.
    pub auto_review_on_stale: bool,
}

impl MacroReplayFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`MacroReplayReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroReplayReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface macro-replay review records.
    pub records: Vec<MacroReplayReviewRecord>,
    /// Guardrail invariants block.
    pub guardrails: MacroReplayGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MacroReplayConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: MacroReplayFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe macro-replay review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayReviewPacket {
    /// Record kind; must equal [`MACRO_REPLAY_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MACRO_REPLAY_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface macro-replay review records.
    pub records: Vec<MacroReplayReviewRecord>,
    /// Guardrail invariants block.
    pub guardrails: MacroReplayGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MacroReplayConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: MacroReplayFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MacroReplayReviewPacket {
    /// Builds a macro-replay review packet.
    pub fn new(input: MacroReplayReviewPacketInput) -> Self {
        Self {
            record_kind: MACRO_REPLAY_REVIEW_RECORD_KIND.to_owned(),
            schema_version: MACRO_REPLAY_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            records: input.records,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some record in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.records
            .iter()
            .map(|record| record.surface_kind)
            .collect()
    }

    /// Outcome classes represented across records.
    pub fn represented_outcomes(&self) -> BTreeSet<MacroReplayOutcomeClass> {
        self.records.iter().map(|record| record.outcome).collect()
    }

    /// Count of records held off the silent exact lane.
    pub fn forced_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.must_not_replay_silently())
            .count()
    }

    /// Count of records resolved to an exact replay.
    pub fn exact_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.outcome.is_silent_exact())
            .count()
    }

    /// Count of records promoted to a declarative recipe.
    pub fn promoted_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.outcome == MacroReplayOutcomeClass::PromotedToDeclarativeRecipe)
            .count()
    }

    /// Count of provider-linked / imported records.
    pub fn provider_or_imported_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.provider_or_imported())
            .count()
    }

    /// Resolves a record by its id.
    pub fn record(&self, record_id: &str) -> Option<&MacroReplayReviewRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    /// Validates the macro-replay review invariants.
    pub fn validate(&self) -> Vec<MacroReplayReviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MACRO_REPLAY_REVIEW_RECORD_KIND {
            violations.push(MacroReplayReviewViolation::WrongRecordKind);
        }
        if self.schema_version != MACRO_REPLAY_REVIEW_SCHEMA_VERSION {
            violations.push(MacroReplayReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MacroReplayReviewViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(MacroReplayReviewViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(MacroReplayReviewViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(MacroReplayReviewViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("macro replay review packet serializes"),
        ) {
            violations.push(MacroReplayReviewViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("macro replay review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Macro-Replay Review: Run-Capable / Cross-File Downgrades and Recipe-Promotion Paths\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Records: {} ({} exact, {} forced off exact, {} promoted, {} provider/imported)\n",
            self.records.len(),
            self.exact_record_count(),
            self.forced_record_count(),
            self.promoted_record_count(),
            self.provider_or_imported_record_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Outcome classes: {} / {}\n",
            self.represented_outcomes().len(),
            MacroReplayOutcomeClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Records\n\n");
        for record in &self.records {
            out.push_str(&format!(
                "- **{}** ({}): outcome `{}`\n",
                record.record_id,
                record.surface_kind.as_str(),
                record.outcome.as_str()
            ));
            out.push_str(&format!("  - {}\n", record.label_summary));
            out.push_str(&format!(
                "  - source register `{}` ({}), scope `{}` ({} files / {} surfaces)\n",
                record.source_register.register_token,
                record.source_register.register_class.as_str(),
                record.scope.as_str(),
                record.files_touched,
                record.surfaces_spanned
            ));
            out.push_str(&format!("  - timing `{}`\n", record.timing.as_str()));
            let triggers = record
                .fired_triggers
                .iter()
                .map(|trigger| trigger.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - triggers: [{}]\n",
                if triggers.is_empty() {
                    "none"
                } else {
                    &triggers
                }
            ));
            out.push_str("  - command lineage:\n");
            for step in &record.command_lineage {
                out.push_str(&format!(
                    "    - `{}` -> `{}` ({}, write `{}`, run_capable={})\n",
                    step.command_id,
                    step.command_revision_ref,
                    step.lineage_class.as_str(),
                    step.write_class.as_str(),
                    step.run_capable
                ));
            }
            if let Some(label) = &record.review_reason_label {
                out.push_str(&format!("  - Review required: {label}\n"));
            }
            if let Some(label) = &record.downgrade_target_label {
                out.push_str(&format!("  - Downgraded to: {label}\n"));
            }
            if let Some(reference) = &record.promoted_recipe_ref {
                out.push_str(&format!("  - Promoted to recipe: {reference}\n"));
            }
            if let Some(label) = &record.rejection_reason_label {
                out.push_str(&format!("  - Rejected: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum MacroReplayReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MacroReplayReviewViolation>),
}

impl fmt::Display for MacroReplayReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "macro replay review export parse failed: {error}"
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
                    "macro replay review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MacroReplayReviewArtifactError {}

/// Validation failures emitted by [`MacroReplayReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacroReplayReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no record.
    RequiredSurfaceKindMissing,
    /// The required replay-outcome classes are not all represented.
    OutcomeCoverageMissing,
    /// No record demonstrates a broad macro held off the silent exact lane.
    ForcedRecordCaseMissing,
    /// No clean exact-replay baseline record is present.
    ExactReplayBaselineMissing,
    /// No provider-linked / imported record is present.
    ProviderOrImportedCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A broad macro was allowed to replay silently on the exact lane.
    SilentReplayOfUnsafeMacro,
    /// A record's outcome ranks below its required safety floor.
    OutcomeBelowRequiredFloor,
    /// A record's recorded triggers do not match the computed set.
    TriggerSetInconsistent,
    /// A record's outcome detail field is missing, generic, or unexpected.
    OutcomeDetailInconsistent,
    /// A record's command lineage was collapsed into opaque text.
    LineageCollapsedToOpaqueText,
    /// A record dropped its source register.
    SourceRegisterMissing,
    /// A record's scope counts contradict its scope class.
    ScopeCountsInconsistent,
    /// A provider/imported record reads as a locally verified replay.
    ImportedReadsAsLocal,
    /// A record's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A record lacks evidence refs.
    RecordEvidenceMissing,
    /// A record's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A record flags raw boundary material present.
    RawBoundaryMaterialPresent,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MacroReplayReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::OutcomeCoverageMissing => "outcome_coverage_missing",
            Self::ForcedRecordCaseMissing => "forced_record_case_missing",
            Self::ExactReplayBaselineMissing => "exact_replay_baseline_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::SilentReplayOfUnsafeMacro => "silent_replay_of_unsafe_macro",
            Self::OutcomeBelowRequiredFloor => "outcome_below_required_floor",
            Self::TriggerSetInconsistent => "trigger_set_inconsistent",
            Self::OutcomeDetailInconsistent => "outcome_detail_inconsistent",
            Self::LineageCollapsedToOpaqueText => "lineage_collapsed_to_opaque_text",
            Self::SourceRegisterMissing => "source_register_missing",
            Self::ScopeCountsInconsistent => "scope_counts_inconsistent",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::RecordEvidenceMissing => "record_evidence_missing",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_macro_replay_review_export(
) -> Result<MacroReplayReviewPacket, MacroReplayReviewArtifactError> {
    let packet: MacroReplayReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/ship-macro-replay-review-run-capable-or-cross-file-macro-downgrades-and-recipe-promotion/support_export.json"
    )))
    .map_err(MacroReplayReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MacroReplayReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MacroReplayReviewPacket,
    violations: &mut Vec<MacroReplayReviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MACRO_REPLAY_REVIEW_SCHEMA_REF,
        MACRO_REPLAY_REVIEW_DOC_REF,
        MACRO_REPLAY_REVIEW_ARTIFACT_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        RECIPE_AND_MACRO_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MacroReplayReviewViolation::MissingSourceContracts);
            break;
        }
    }
}

/// Surface kinds that must appear so the packet proves macro-replay safety across
/// the new M5 modal workflows, plus the editor-core exact baseline.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 6] = [
    KeyboardSurfaceKind::EditorCore,
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::DocsSurface,
    KeyboardSurfaceKind::ReviewSurface,
];

fn validate_coverage(
    packet: &MacroReplayReviewPacket,
    violations: &mut Vec<MacroReplayReviewViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(MacroReplayReviewViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let outcomes = packet.represented_outcomes();
    for required in MacroReplayOutcomeClass::ALL {
        if !outcomes.contains(&required) {
            violations.push(MacroReplayReviewViolation::OutcomeCoverageMissing);
            break;
        }
    }

    if !packet
        .records
        .iter()
        .any(|record| record.must_not_replay_silently() && record.is_complete())
    {
        violations.push(MacroReplayReviewViolation::ForcedRecordCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.outcome.is_silent_exact()
            && !record.must_not_replay_silently()
            && record.is_complete()
    }) {
        violations.push(MacroReplayReviewViolation::ExactReplayBaselineMissing);
    }

    if packet.provider_or_imported_record_count() == 0 {
        violations.push(MacroReplayReviewViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_records(
    packet: &MacroReplayReviewPacket,
    violations: &mut Vec<MacroReplayReviewViolation>,
) {
    for record in &packet.records {
        if !record.is_complete() {
            violations.push(MacroReplayReviewViolation::RecordIncomplete);
        }
        if record.silently_replays_unsafe() {
            violations.push(MacroReplayReviewViolation::SilentReplayOfUnsafeMacro);
        }
        if !record.outcome_meets_floor() {
            violations.push(MacroReplayReviewViolation::OutcomeBelowRequiredFloor);
        }
        if !record.triggers_consistent() {
            violations.push(MacroReplayReviewViolation::TriggerSetInconsistent);
        }
        if !record.outcome_detail_consistent() {
            violations.push(MacroReplayReviewViolation::OutcomeDetailInconsistent);
        }
        if record.lineage_collapsed_to_opaque_text || !record.lineage_exact() {
            violations.push(MacroReplayReviewViolation::LineageCollapsedToOpaqueText);
        }
        if !record.source_register.is_valid() {
            violations.push(MacroReplayReviewViolation::SourceRegisterMissing);
        }
        if !record
            .scope
            .counts_consistent(record.files_touched, record.surfaces_spanned)
        {
            violations.push(MacroReplayReviewViolation::ScopeCountsInconsistent);
        }
        if !record.imported_posture_consistent() {
            violations.push(MacroReplayReviewViolation::ImportedReadsAsLocal);
        }
        if !record.verification.is_well_formed() {
            violations.push(MacroReplayReviewViolation::VerificationProofNotReopenable);
        }
        if record.evidence_refs.is_empty()
            || record.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(MacroReplayReviewViolation::RecordEvidenceMissing);
        }
        if !record.subject.fingerprint_independent_of_id() {
            violations.push(MacroReplayReviewViolation::FingerprintSubstitutesIdentity);
        }
        if !record.no_raw_boundary_material() {
            violations.push(MacroReplayReviewViolation::RawBoundaryMaterialPresent);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "downgraded"
            | "rejected"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_macro_replay_review_packet`].
pub const SEED_MACRO_REPLAY_REVIEW_PACKET_ID: &str = "m5-macro-replay-review:stable:0001";

/// Mint timestamp used by [`seeded_macro_replay_review_packet`].
pub const SEED_MACRO_REPLAY_REVIEW_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating macro-replay review packet that the checked-in
/// support export, the Markdown summary, and the conformance tests all share, so
/// the in-crate builder stays byte-aligned with the artifact.
///
/// The seed anchors a clean editor-core exact-replay baseline, then exercises each
/// non-exact outcome on a distinct M5 surface: a run-capable notebook macro held
/// for review, a cross-file data/API macro downgraded to observe-no-mutation, a
/// cross-surface run-capable preview macro promoted to a declarative recipe, a
/// docs macro carrying an unmapped step that is rejected, a workspace-wide review
/// macro promoted to a recipe, a runtime macro downgraded to observe-only, and a
/// provider-linked companion macro held for review whose imported proof never
/// reads as a local replay.
pub fn seeded_macro_replay_review_packet() -> MacroReplayReviewPacket {
    MacroReplayReviewPacket::new(MacroReplayReviewPacketInput {
        packet_id: SEED_MACRO_REPLAY_REVIEW_PACKET_ID.to_owned(),
        label: "M5 Macro-Replay Review: Run-Capable / Cross-File Downgrades and Recipe-Promotion"
            .to_owned(),
        records: seeded_records(),
        guardrails: MacroReplayGuardrails {
            broad_macros_never_replay_silently: true,
            exact_command_lineage_preserved: true,
            source_register_and_scope_preserved: true,
            unstable_or_cross_surface_promoted_to_recipe: true,
            provider_replays_never_read_as_local: true,
            no_new_macro_language_introduced: true,
        },
        consumer_projection: MacroReplayConsumerProjection {
            product_ingests_packet: true,
            help_migration_ingests_packet: true,
            support_export_ingests_packet: true,
            release_control_ingests_packet: true,
            exact_downgraded_promoted_distinguishable: true,
        },
        verification_freshness: MacroReplayFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
            auto_review_on_stale: true,
        },
        source_contract_refs: vec![
            MACRO_REPLAY_REVIEW_SCHEMA_REF.to_owned(),
            MACRO_REPLAY_REVIEW_DOC_REF.to_owned(),
            MACRO_REPLAY_REVIEW_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            RECIPE_AND_MACRO_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn seeded_records() -> Vec<MacroReplayReviewRecord> {
    vec![
        editor_core_exact_record(),
        notebook_review_record(),
        data_api_downgrade_record(),
        preview_promote_record(),
        docs_reject_record(),
        review_promote_record(),
        runtime_downgrade_record(),
        companion_review_record(),
    ]
}

/// Builds a verification proof keyed by a non-display fingerprint distinct from
/// the record id.
fn proof_for(record_id: &str, currency: AxisProofCurrency, summary: &str) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{record_id}")),
            Some(format!("fp:proof:{record_id}")),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: summary.to_owned(),
    }
}

/// Builds a subject whose fingerprint is independent of its surface id.
fn subject_for(record_id: &str, origin_class: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{record_id}"),
        origin_class,
        surface_fingerprint_token: format!("fp:surface:{record_id}"),
    }
}

fn step(
    step_id: &str,
    lineage_class: MacroCommandLineageClass,
    command_id: &str,
    write_class: MacroWriteClass,
    run_capable: bool,
    display_label: &str,
) -> MacroCommandStep {
    MacroCommandStep {
        step_id: step_id.to_owned(),
        lineage_class,
        command_id: command_id.to_owned(),
        command_revision_ref: format!("{command_id}@rev1"),
        write_class,
        run_capable,
        display_label: display_label.to_owned(),
        ai_tool_handle_ref: if lineage_class.requires_ai_tool_handle_ref() {
            Some(format!("ai-tool:{command_id}"))
        } else {
            None
        },
    }
}

fn editor_core_exact_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:editor-core:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor-core single-file refactor macro that replays exactly against one buffer"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:a".to_owned(),
            register_class: MacroRegisterClass::NamedRegister,
            display_label: "Named register a".to_owned(),
        },
        scope: MacroScopeClass::SingleSurfaceSingleFile,
        files_touched: 1,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "editor.motion.word_forward",
                MacroWriteClass::ReadOnly,
                false,
                "Move to next word",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::CoreCommand,
                "editor.edit.replace_token",
                MacroWriteClass::EditorBufferMutation,
                false,
                "Replace token under cursor",
            ),
        ],
        lineage_summary: "Two core editor commands mutating one buffer".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor-core exact replay verified against a single-buffer fixture",
        ),
        outcome: MacroReplayOutcomeClass::ExactReplayLocalEditorOnly,
        fired_triggers: vec![],
        review_reason_label: None,
        downgrade_target_label: None,
        promoted_recipe_ref: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn notebook_review_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:notebook:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Notebook macro that edits and re-runs a cell; the run-capable step opens review"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:q".to_owned(),
            register_class: MacroRegisterClass::RecordingBufferRegister,
            display_label: "Recording buffer q".to_owned(),
        },
        scope: MacroScopeClass::SingleSurfaceSingleFile,
        files_touched: 1,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "notebook.cell.edit_source",
                MacroWriteClass::EditorBufferMutation,
                false,
                "Edit cell source",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::RunCapableCommand,
                "notebook.cell.run",
                MacroWriteClass::RunCapableExecution,
                true,
                "Run the edited cell",
            ),
        ],
        lineage_summary: "Edit then run one notebook cell".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Notebook run-capable replay verified to open review before executing",
        ),
        outcome: MacroReplayOutcomeClass::ReviewRequiredBeforeApply,
        fired_triggers: vec![MacroReplayTrigger::RunCapableOrElevatedCommand],
        review_reason_label: Some(
            "Macro re-runs a notebook cell; review the run before it executes".to_owned(),
        ),
        downgrade_target_label: None,
        promoted_recipe_ref: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn data_api_downgrade_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:data-api:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Data/API macro that rewrites rows across files; downgraded to observe-no-mutation"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:1".to_owned(),
            register_class: MacroRegisterClass::NumberedRegister,
            display_label: "Numbered register 1".to_owned(),
        },
        scope: MacroScopeClass::CrossFileWithinSurface,
        files_touched: 3,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "data_grid.row.select_matching",
                MacroWriteClass::ReadOnly,
                false,
                "Select matching rows",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::CoreCommand,
                "data_grid.row.rewrite_cell",
                MacroWriteClass::MultiFileMutation,
                false,
                "Rewrite cells across files",
            ),
        ],
        lineage_summary: "Select then rewrite cells across three files".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::CachedWithinWindow,
            "Cross-file data/API replay verified to downgrade to observe-no-mutation",
        ),
        outcome: MacroReplayOutcomeClass::DowngradedToObserverNoMutation,
        fired_triggers: vec![
            MacroReplayTrigger::CrossFileScope,
            MacroReplayTrigger::RunCapableOrElevatedCommand,
        ],
        review_reason_label: None,
        downgrade_target_label: Some(
            "Replay observes the cross-file rewrite without mutating; apply requires explicit confirm"
                .to_owned(),
        ),
        promoted_recipe_ref: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn preview_promote_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:preview:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Preview macro that edits source and rebuilds across surfaces; promoted to a recipe"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:b".to_owned(),
            register_class: MacroRegisterClass::NamedRegister,
            display_label: "Named register b".to_owned(),
        },
        scope: MacroScopeClass::CrossSurfaceSpan,
        files_touched: 2,
        surfaces_spanned: 2,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "editor.edit.apply_patch",
                MacroWriteClass::EditorBufferMutation,
                false,
                "Patch the source file",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::RunCapableCommand,
                "preview.runtime.rebuild",
                MacroWriteClass::RunCapableExecution,
                true,
                "Rebuild the preview runtime",
            ),
        ],
        lineage_summary: "Patch source on the editor surface then rebuild the preview".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Cross-surface preview replay verified to promote to a declarative recipe",
        ),
        outcome: MacroReplayOutcomeClass::PromotedToDeclarativeRecipe,
        fired_triggers: vec![
            MacroReplayTrigger::CrossFileScope,
            MacroReplayTrigger::CrossSurfaceOrWorkspaceScope,
            MacroReplayTrigger::RunCapableOrElevatedCommand,
        ],
        review_reason_label: None,
        downgrade_target_label: None,
        promoted_recipe_ref: Some("recipe:preview-edit-and-rebuild@v1".to_owned()),
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn docs_reject_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:docs:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Docs macro carrying an unmapped keystroke that cannot resolve; rejected as unsafe"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:c".to_owned(),
            register_class: MacroRegisterClass::NamedRegister,
            display_label: "Named register c".to_owned(),
        },
        scope: MacroScopeClass::SingleSurfaceSingleFile,
        files_touched: 1,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "docs.edit.insert_heading",
                MacroWriteClass::EditorBufferMutation,
                false,
                "Insert a heading",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::UnmappedKeystrokeUnsafe,
                "docs.unmapped.chord",
                MacroWriteClass::ReadOnly,
                false,
                "Unmapped key chord with no command-graph resolution",
            ),
        ],
        lineage_summary: "Insert a heading then an unmapped chord that cannot resolve".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Docs replay verified to reject because a step does not resolve to a command",
        ),
        outcome: MacroReplayOutcomeClass::RejectedUnsafeReplay,
        fired_triggers: vec![MacroReplayTrigger::UnmappedOrUnsafeStep],
        review_reason_label: None,
        downgrade_target_label: None,
        promoted_recipe_ref: None,
        rejection_reason_label: Some(
            "A recorded keystroke does not resolve to any command; replay is rejected rather than approximated"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn review_promote_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:review:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::ReviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Review-panel macro that applies suggestions workspace-wide; promoted to a recipe"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:plus".to_owned(),
            register_class: MacroRegisterClass::ClipboardLinkedRegister,
            display_label: "Clipboard-linked register +".to_owned(),
        },
        scope: MacroScopeClass::WorkspaceWideSpan,
        files_touched: 5,
        surfaces_spanned: 3,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![
            step(
                "step-1",
                MacroCommandLineageClass::CoreCommand,
                "review.suggestion.accept",
                MacroWriteClass::MultiFileMutation,
                false,
                "Accept suggestion across files",
            ),
            step(
                "step-2",
                MacroCommandLineageClass::AiToolHandle,
                "review.ai.apply_fixups",
                MacroWriteClass::MultiFileMutation,
                false,
                "Apply AI-proposed fixups",
            ),
        ],
        lineage_summary: "Accept suggestions and apply AI fixups across the workspace".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Workspace-wide review replay verified to promote to a declarative recipe",
        ),
        outcome: MacroReplayOutcomeClass::PromotedToDeclarativeRecipe,
        fired_triggers: vec![
            MacroReplayTrigger::CrossFileScope,
            MacroReplayTrigger::CrossSurfaceOrWorkspaceScope,
            MacroReplayTrigger::RunCapableOrElevatedCommand,
        ],
        review_reason_label: None,
        downgrade_target_label: None,
        promoted_recipe_ref: Some("recipe:review-accept-and-fixup@v1".to_owned()),
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn runtime_downgrade_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:runtime:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::RuntimeSurface,
        subject: subject_for(record_id, SurfaceOriginClass::EmbeddedRuntimeSurface),
        label_summary:
            "Embedded-runtime macro depending on async output; downgraded to observe-only"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:2".to_owned(),
            register_class: MacroRegisterClass::NumberedRegister,
            display_label: "Numbered register 2".to_owned(),
        },
        scope: MacroScopeClass::CrossFileWithinSurface,
        files_touched: 2,
        surfaces_spanned: 1,
        timing: MacroTimingClass::DependsOnAsyncOutput,
        command_lineage: vec![step(
            "step-1",
            MacroCommandLineageClass::RunCapableCommand,
            "runtime.task.invoke",
            MacroWriteClass::RunCapableExecution,
            true,
            "Invoke a runtime task that awaits async output",
        )],
        lineage_summary: "Invoke a runtime task whose result arrives asynchronously".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Unstable-timing runtime replay verified to promote to a recipe rather than replay",
        ),
        // Cross-file (floor review=1), run-capable (floor review=1), and unstable
        // timing (floor promote=3) all fire, so the only safe outcomes are promote
        // or reject. The runtime macro is re-authored as a deterministic recipe.
        outcome: MacroReplayOutcomeClass::PromotedToDeclarativeRecipe,
        fired_triggers: vec![
            MacroReplayTrigger::CrossFileScope,
            MacroReplayTrigger::RunCapableOrElevatedCommand,
            MacroReplayTrigger::UnstableTiming,
        ],
        review_reason_label: None,
        downgrade_target_label: None,
        promoted_recipe_ref: Some("recipe:runtime-task-deterministic@v1".to_owned()),
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

fn companion_review_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:companion:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(record_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Provider-linked companion macro held for review; imported proof never reads as local"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:d".to_owned(),
            register_class: MacroRegisterClass::NamedRegister,
            display_label: "Named register d".to_owned(),
        },
        scope: MacroScopeClass::SingleSurfaceSingleFile,
        files_touched: 1,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![step(
            "step-1",
            MacroCommandLineageClass::ExtensionCommand,
            "companion.provider.invoke_action",
            MacroWriteClass::RunCapableExecution,
            true,
            "Invoke a provider-backed companion action",
        )],
        lineage_summary: "Invoke one provider-backed companion action".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::ImportedCurrent,
            "Provider-backed companion replay verified with imported proof, never a local rerun",
        ),
        outcome: MacroReplayOutcomeClass::ReviewRequiredBeforeApply,
        fired_triggers: vec![MacroReplayTrigger::RunCapableOrElevatedCommand],
        review_reason_label: Some(
            "Provider-backed action runs off-device; review before it is invoked".to_owned(),
        ),
        downgrade_target_label: None,
        promoted_recipe_ref: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}

/// Packet id minted by [`fixture_macro_replay_review_packet`].
pub const FIXTURE_MACRO_REPLAY_REVIEW_PACKET_ID: &str =
    "m5-macro-replay-review:fixture:stale-proof-forces-review:0001";

/// Builds the protected fixture variant: it keeps the full seeded record set —
/// including the clean editor-core exact baseline — and adds one drill record for
/// a single-file editor macro that would otherwise replay exactly but is forced
/// off the silent exact lane because its review proof aged outside the freshness
/// window.
///
/// The fixture is a *valid* packet: the drill record correctly records the
/// [`MacroReplayTrigger::StaleOrMissingReviewProof`] trigger and resolves to
/// [`MacroReplayOutcomeClass::ReviewRequiredBeforeApply`] with a precise review
/// reason, so it validates while demonstrating that stale evidence — not just
/// scope or run-capability — forces macro replay through review.
pub fn fixture_macro_replay_review_packet() -> MacroReplayReviewPacket {
    let mut packet = seeded_macro_replay_review_packet();
    packet.packet_id = FIXTURE_MACRO_REPLAY_REVIEW_PACKET_ID.to_owned();
    packet.label =
        "M5 Macro-Replay Review fixture: stale review proof forces an exact macro into review"
            .to_owned();
    packet.records.push(stale_proof_drill_record());
    packet
}

/// A single-file editor macro that would replay exactly, but whose review proof
/// has aged outside its freshness window, so it is forced through review.
fn stale_proof_drill_record() -> MacroReplayReviewRecord {
    let record_id = "macro-replay:editor-core:stale-proof:0001";
    MacroReplayReviewRecord::new(MacroReplayReviewRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor-core single-file macro whose stale review proof forces it through review"
                .to_owned(),
        source_register: MacroSourceRegister {
            register_token: "register:e".to_owned(),
            register_class: MacroRegisterClass::NamedRegister,
            display_label: "Named register e".to_owned(),
        },
        scope: MacroScopeClass::SingleSurfaceSingleFile,
        files_touched: 1,
        surfaces_spanned: 1,
        timing: MacroTimingClass::StableDeterministic,
        command_lineage: vec![step(
            "step-1",
            MacroCommandLineageClass::CoreCommand,
            "editor.edit.reformat_block",
            MacroWriteClass::EditorBufferMutation,
            false,
            "Reformat the current block",
        )],
        lineage_summary: "Reformat one block in one buffer".to_owned(),
        verification: proof_for(
            record_id,
            AxisProofCurrency::StaleExpired,
            "Editor-core replay proof aged outside its freshness window",
        ),
        outcome: MacroReplayOutcomeClass::ReviewRequiredBeforeApply,
        fired_triggers: vec![MacroReplayTrigger::StaleOrMissingReviewProof],
        review_reason_label: Some(
            "Editor-core replay proof aged outside its freshness window; review before replaying"
                .to_owned(),
        ),
        downgrade_target_label: None,
        promoted_recipe_ref: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_MACRO_REPLAY_REVIEW_MINTED_AT.to_owned(),
    })
}
