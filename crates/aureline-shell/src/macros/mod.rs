//! Recorded-macro alpha record family bound to the command graph, mode
//! state, and trust policy.
//!
//! The recorded-macro / declarative-recipe boundary at
//! [`/docs/automation/recipe_and_macro_contract.md`](../../../../docs/automation/recipe_and_macro_contract.md)
//! already names recorded macros as the deliberately narrow,
//! UI / editor-state-only authoring shape — declarative recipes are the
//! only form admitted to invoke command descriptors at full breadth and
//! the only form admitted on the managed-only channel.
//!
//! This module owns the bounded alpha record family that wires the
//! shell-side projection of recorded macros to:
//!
//! - the **command graph** — every recorded step resolves to exactly one
//!   stable `command_id` and `command_revision_ref` on the command
//!   descriptor contract; unmapped keystrokes resolve to the
//!   [`StepCommandLineageClass::UnmappedKeystrokeDenied`] class and
//!   force the bound definition to a denied or promote-to-recipe
//!   disposition;
//! - the **mode state** — every step pins the editor or palette mode the
//!   replay MUST observe before dispatching, drawn from
//!   [`ModeRequirementClass`]; raw terminal input replay is a closed
//!   denial class;
//! - the **trust policy** — every definition projects the workspace
//!   trust posture it requires through [`TrustGateClass`];
//!   `managed_only_denied` is a closed denial class consistent with the
//!   automation contract.
//!
//! Every replay attempt against a definition mints exactly one
//! [`RecordedMacroAlphaReplayDisposition`]. The only disposition that
//! silently dispatches is
//! [`ReplayDispositionClass::ProceedLocalEditorOnly`]; every other class
//! ([`ReplayDispositionClass::PreviewRequiredBeforeApply`],
//! [`ReplayDispositionClass::DowngradedToObserverNoMutation`],
//! [`ReplayDispositionClass::PromotedToDeclarativeRecipe`],
//! [`ReplayDispositionClass::DeniedUnsafeReplay`]) forces preview,
//! downgrade, recipe promotion, or denial. Every disposition mints
//! support-export and activity-history attribution rows so the replay
//! remains attributable in support exports and activity history.
//!
//! The cross-tool boundary lives at
//! [`/schemas/commands/recorded_macro.schema.json`](../../../../schemas/commands/recorded_macro.schema.json).
//! The reviewer-facing landing page lives at
//! [`/docs/ux/m3/recorded_macro_alpha.md`](../../../../docs/ux/m3/recorded_macro_alpha.md).
//! The reviewer fixture lives at
//! [`/fixtures/commands/recorded_macro_alpha/page.json`](../../../../fixtures/commands/recorded_macro_alpha/page.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Alpha schema version exported with every recorded-macro record.
pub const RECORDED_MACRO_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record in this family.
pub const RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF: &str = "commands:recorded_macro_alpha:v1";

/// Stable record-kind tag for [`RecordedMacroAlphaPage`] payloads.
pub const RECORDED_MACRO_ALPHA_PAGE_RECORD_KIND: &str = "recorded_macro_alpha_page_record";

/// Stable record-kind tag for [`RecordedMacroAlphaDefinition`] payloads.
pub const RECORDED_MACRO_ALPHA_DEFINITION_RECORD_KIND: &str =
    "recorded_macro_alpha_definition_record";

/// Stable record-kind tag for [`RecordedMacroAlphaReplayDisposition`]
/// payloads.
pub const RECORDED_MACRO_ALPHA_REPLAY_DISPOSITION_RECORD_KIND: &str =
    "recorded_macro_alpha_replay_disposition_record";

/// Stable record-kind tag for [`RecordedMacroAlphaAuditEvent`] payloads.
pub const RECORDED_MACRO_ALPHA_AUDIT_EVENT_RECORD_KIND: &str =
    "recorded_macro_alpha_audit_event_record";

/// Stable record-kind tag for [`RecordedMacroAlphaAttribution`] payloads.
pub const RECORDED_MACRO_ALPHA_ATTRIBUTION_RECORD_KIND: &str =
    "recorded_macro_alpha_attribution_record";

/// Stable record-kind tag for [`RecordedMacroAlphaValidationReport`]
/// payloads.
pub const RECORDED_MACRO_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str =
    "recorded_macro_alpha_validation_report";

/// Stable record-kind tag for the redaction-safe support-export
/// projection.
pub const RECORDED_MACRO_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "recorded_macro_alpha_support_export";

/// Closed vocabulary for one recorded-macro step's command-graph lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepCommandLineageClass {
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
    /// No mapping found — the raw key chord is recorded but the step is
    /// denied. The bound definition is forced to a denied or
    /// promote-to-recipe disposition.
    UnmappedKeystrokeDenied,
}

impl StepCommandLineageClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreCommand => "core_command",
            Self::ImportedCommand => "imported_command",
            Self::ExtensionCommand => "extension_command",
            Self::AiToolHandle => "ai_tool_handle",
            Self::CliVerb => "cli_verb",
            Self::UnmappedKeystrokeDenied => "unmapped_keystroke_denied",
        }
    }

    /// True when this lineage class requires the step to cite an
    /// `ai_tool_handle_ref`.
    pub const fn requires_ai_tool_handle_ref(self) -> bool {
        matches!(self, Self::AiToolHandle)
    }

    /// True when this lineage class forces the bound definition off the
    /// silent `proceed_local_editor_only` lane.
    pub const fn forces_non_proceed_disposition(self) -> bool {
        matches!(self, Self::UnmappedKeystrokeDenied)
    }
}

/// Closed vocabulary for the editor / palette / shell mode a step
/// requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeRequirementClass {
    /// Editor must be in normal (operator-pending) mode.
    EditorNormalModeRequired,
    /// Editor must be in insert mode.
    EditorInsertModeRequired,
    /// Editor must be in visual (selection) mode.
    EditorVisualModeRequired,
    /// Editor may be in any mode.
    EditorAnyModeAdmissible,
    /// Palette must be focused.
    PaletteModeRequired,
    /// Terminal mode is denied for macro replay.
    TerminalModeDenied,
}

impl ModeRequirementClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorNormalModeRequired => "editor_normal_mode_required",
            Self::EditorInsertModeRequired => "editor_insert_mode_required",
            Self::EditorVisualModeRequired => "editor_visual_mode_required",
            Self::EditorAnyModeAdmissible => "editor_any_mode_admissible",
            Self::PaletteModeRequired => "palette_mode_required",
            Self::TerminalModeDenied => "terminal_mode_denied",
        }
    }

    /// True when this mode requirement forces the bound definition off
    /// the silent `proceed_local_editor_only` lane.
    pub const fn forces_non_proceed_disposition(self) -> bool {
        matches!(self, Self::TerminalModeDenied)
    }
}

/// Closed vocabulary for one write class a recorded-macro step touches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteClass {
    /// No write surface is touched.
    ReadOnly,
    /// One editor buffer is mutated.
    EditorBufferMutation,
    /// Multiple files / buffers are mutated.
    EditorMultiFileMutation,
    /// Settings mutation — denied for recorded macros.
    SettingsMutationDenied,
    /// Network mutation — denied for recorded macros.
    NetworkMutationDenied,
    /// Process mutation — denied for recorded macros.
    ProcessMutationDenied,
    /// Credential mutation — denied for recorded macros.
    CredentialMutationDenied,
}

impl WriteClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::EditorBufferMutation => "editor_buffer_mutation",
            Self::EditorMultiFileMutation => "editor_multi_file_mutation",
            Self::SettingsMutationDenied => "settings_mutation_denied",
            Self::NetworkMutationDenied => "network_mutation_denied",
            Self::ProcessMutationDenied => "process_mutation_denied",
            Self::CredentialMutationDenied => "credential_mutation_denied",
        }
    }

    /// True when this write class is a closed denial marker.
    pub const fn is_denied(self) -> bool {
        matches!(
            self,
            Self::SettingsMutationDenied
                | Self::NetworkMutationDenied
                | Self::ProcessMutationDenied
                | Self::CredentialMutationDenied
        )
    }

    /// True when this write class forces the bound definition off the
    /// silent `proceed_local_editor_only` lane.
    pub const fn forces_non_proceed_disposition(self) -> bool {
        !matches!(self, Self::ReadOnly | Self::EditorBufferMutation)
    }
}

/// Closed vocabulary for the replay limitation a definition carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLimitationClass {
    /// Replay is confined to a single editor buffer; admissible on the
    /// silent `proceed_local_editor_only` lane.
    SingleBufferSafe,
    /// Replay crosses multiple buffers; preview is required.
    MultiBufferRequiresPreview,
    /// Replay crosses a workspace boundary; recipe promotion is required.
    CrossesWorkspaceBoundaryRequiresRecipePromotion,
    /// Replay depends on unstable timing; replay is denied.
    NonReplayableUnstableTiming,
    /// Replay contains an unmapped step; replay is denied.
    DeniedUnmappedCommandPresent,
}

impl ReplayLimitationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBufferSafe => "single_buffer_safe",
            Self::MultiBufferRequiresPreview => "multi_buffer_requires_preview",
            Self::CrossesWorkspaceBoundaryRequiresRecipePromotion => {
                "crosses_workspace_boundary_requires_recipe_promotion"
            }
            Self::NonReplayableUnstableTiming => "non_replayable_unstable_timing",
            Self::DeniedUnmappedCommandPresent => "denied_unmapped_command_present",
        }
    }

    /// The replay disposition this limitation forces. None means the
    /// limitation is admissible on the silent
    /// `proceed_local_editor_only` lane.
    pub const fn forced_disposition(self) -> Option<ReplayDispositionClass> {
        match self {
            Self::SingleBufferSafe => None,
            Self::MultiBufferRequiresPreview => {
                Some(ReplayDispositionClass::PreviewRequiredBeforeApply)
            }
            Self::CrossesWorkspaceBoundaryRequiresRecipePromotion => {
                Some(ReplayDispositionClass::PromotedToDeclarativeRecipe)
            }
            Self::NonReplayableUnstableTiming | Self::DeniedUnmappedCommandPresent => {
                Some(ReplayDispositionClass::DeniedUnsafeReplay)
            }
        }
    }
}

/// Closed vocabulary for the trust gate the definition projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustGateClass {
    /// Admissible on a restricted workspace.
    RestrictedWorkspaceAdmissible,
    /// Trusted workspace required.
    TrustedWorkspaceRequired,
    /// Admin policy observed (rides under admin policy observation).
    AdminPolicyObserved,
    /// Managed-only channel — denied for recorded macros.
    ManagedOnlyDenied,
}

impl TrustGateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestrictedWorkspaceAdmissible => "restricted_workspace_admissible",
            Self::TrustedWorkspaceRequired => "trusted_workspace_required",
            Self::AdminPolicyObserved => "admin_policy_observed",
            Self::ManagedOnlyDenied => "managed_only_denied",
        }
    }
}

/// Closed vocabulary for the replay disposition a replay attempt
/// resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDispositionClass {
    /// Replay proceeds silently against a single editor buffer.
    ProceedLocalEditorOnly,
    /// Replay requires a preview pass before apply.
    PreviewRequiredBeforeApply,
    /// Replay is downgraded to an observer-only run (no mutation).
    DowngradedToObserverNoMutation,
    /// Replay is promoted to a declarative recipe and re-authored.
    PromotedToDeclarativeRecipe,
    /// Replay is denied as unsafe.
    DeniedUnsafeReplay,
}

impl ReplayDispositionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedLocalEditorOnly => "proceed_local_editor_only",
            Self::PreviewRequiredBeforeApply => "preview_required_before_apply",
            Self::DowngradedToObserverNoMutation => "downgraded_to_observer_no_mutation",
            Self::PromotedToDeclarativeRecipe => "promoted_to_declarative_recipe",
            Self::DeniedUnsafeReplay => "denied_unsafe_replay",
        }
    }

    /// True when this disposition silently dispatches the replay.
    pub const fn is_silent_proceed(self) -> bool {
        matches!(self, Self::ProceedLocalEditorOnly)
    }

    /// True when this disposition must cite a
    /// `preview_required_reason_label`.
    pub const fn requires_preview_reason(self) -> bool {
        matches!(self, Self::PreviewRequiredBeforeApply)
    }

    /// True when this disposition must cite a `downgrade_target_label`.
    pub const fn requires_downgrade_target(self) -> bool {
        matches!(self, Self::DowngradedToObserverNoMutation)
    }

    /// True when this disposition must cite a `promoted_recipe_ref`.
    pub const fn requires_promoted_recipe_ref(self) -> bool {
        matches!(self, Self::PromotedToDeclarativeRecipe)
    }

    /// True when this disposition must cite a `denial_reason_label`.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::DeniedUnsafeReplay)
    }
}

/// Closed vocabulary for one audit-event on the recorded-macro lifecycle
/// stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventClass {
    /// Definition was recorded.
    MacroRecorded,
    /// Replay was requested.
    MacroReplayRequested,
    /// Replay was admitted on the silent proceed lane.
    MacroReplayAdmitted,
    /// Replay was denied closed.
    MacroReplayDenied,
    /// Replay was deferred behind a preview pass.
    MacroReplayPreviewRequired,
    /// Replay was downgraded to observer-only.
    MacroReplayDowngraded,
    /// Replay was promoted to a declarative recipe.
    MacroReplayPromotedToRecipe,
    /// An attribution row was minted.
    MacroAttributionMinted,
}

impl AuditEventClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacroRecorded => "macro_recorded",
            Self::MacroReplayRequested => "macro_replay_requested",
            Self::MacroReplayAdmitted => "macro_replay_admitted",
            Self::MacroReplayDenied => "macro_replay_denied",
            Self::MacroReplayPreviewRequired => "macro_replay_preview_required",
            Self::MacroReplayDowngraded => "macro_replay_downgraded",
            Self::MacroReplayPromotedToRecipe => "macro_replay_promoted_to_recipe",
            Self::MacroAttributionMinted => "macro_attribution_minted",
        }
    }

    /// True when the event must cite a `denial_reason_label`.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::MacroReplayDenied)
    }

    /// True when the event must cite a `replay_disposition_ref`.
    pub const fn requires_disposition_ref(self) -> bool {
        matches!(
            self,
            Self::MacroReplayAdmitted
                | Self::MacroReplayDenied
                | Self::MacroReplayPreviewRequired
                | Self::MacroReplayDowngraded
                | Self::MacroReplayPromotedToRecipe
        )
    }

    /// True when the event must cite an `attribution_ref`.
    pub const fn requires_attribution_ref(self) -> bool {
        matches!(self, Self::MacroAttributionMinted)
    }
}

/// Closed vocabulary for the attribution-surface class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionSurfaceClass {
    /// Support export.
    SupportExport,
    /// Activity history.
    ActivityHistory,
    /// Admin-audit export.
    AdminAuditExport,
}

impl AttributionSurfaceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExport => "support_export",
            Self::ActivityHistory => "activity_history",
            Self::AdminAuditExport => "admin_audit_export",
        }
    }
}

/// References to the upstream contracts this alpha page composes with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaContractRefs {
    /// Reference to the command descriptor boundary schema.
    pub command_descriptor_schema_ref: String,
    /// Reference to the shareability-metadata boundary schema.
    pub shareability_metadata_schema_ref: String,
    /// Reference to the recipe-manifest boundary schema.
    pub recipe_manifest_schema_ref: String,
    /// Reference to the run-record boundary schema.
    pub run_record_schema_ref: String,
    /// Reference to the trust-state matrix.
    pub trust_state_matrix_ref: String,
}

impl RecordedMacroAlphaContractRefs {
    fn all_refs(&self) -> [&str; 5] {
        [
            &self.command_descriptor_schema_ref,
            &self.shareability_metadata_schema_ref,
            &self.recipe_manifest_schema_ref,
            &self.run_record_schema_ref,
            &self.trust_state_matrix_ref,
        ]
    }
}

/// One step in a recorded-macro definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandLineageStep {
    /// Stable opaque step id.
    pub step_id: String,
    /// Step command-graph lineage class.
    pub step_command_lineage: StepCommandLineageClass,
    /// Stable command id on the command graph.
    pub command_id: String,
    /// Stable command revision ref on the command graph.
    pub command_revision_ref: String,
    /// Mode requirement for this step.
    pub mode_requirement: ModeRequirementClass,
    /// All write classes this step touches.
    pub write_classes: Vec<WriteClass>,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Optional opaque ref to the shareability-metadata row that admits
    /// this step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shareability_record_ref: Option<String>,
    /// Required when `step_command_lineage` is `ai_tool_handle`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_tool_handle_ref: Option<String>,
}

/// One typed recorded-macro definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaDefinition {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque definition id.
    pub definition_id: String,
    /// Macro id on the recipe-manifest contract.
    pub macro_id: String,
    /// Macro revision ref on the recipe-manifest contract.
    pub macro_revision_ref: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the author identity.
    pub author_identity_ref: String,
    /// Trust gate class.
    pub trust_gate: TrustGateClass,
    /// Replay limitation class.
    pub replay_limitation: ReplayLimitationClass,
    /// Ordered command-graph steps.
    pub steps: Vec<CommandLineageStep>,
    /// Reviewable lineage summary.
    pub lineage_summary: String,
    /// Guardrail: row does not carry raw keystroke bytes.
    pub raw_keystroke_bytes_present: bool,
    /// Guardrail: row does not carry raw editor buffer bytes.
    pub raw_buffer_bytes_present: bool,
    /// Guardrail: row does not carry raw shell fragments.
    pub raw_shell_fragment_present: bool,
    /// Guardrail: row does not carry raw credential material.
    pub raw_credential_present: bool,
    /// Guardrail: definition did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Guardrail: support-export attribution was minted at recording
    /// time.
    pub support_attribution_minted: bool,
    /// Guardrail: activity-history attribution was minted at recording
    /// time.
    pub activity_attribution_minted: bool,
    /// Timestamp at which the macro was recorded.
    pub recorded_at: String,
}

/// One typed replay-disposition row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaReplayDisposition {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque disposition id.
    pub disposition_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound definition.
    pub definition_ref: String,
    /// Macro id (denormalized for reviewers).
    pub macro_id: String,
    /// Macro revision ref (denormalized for reviewers).
    pub macro_revision_ref: String,
    /// Disposition class.
    pub disposition: ReplayDispositionClass,
    /// Reviewable rationale.
    pub rationale_summary: String,
    /// Trust gate observed at replay time.
    pub trust_gate_observed: TrustGateClass,
    /// Opaque refs to audit events minted for this disposition.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Opaque ref to the support-export attribution row.
    pub support_attribution_ref: String,
    /// Opaque ref to the activity-history attribution row.
    pub activity_attribution_ref: String,
    /// Required when `disposition` is `preview_required_before_apply`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_required_reason_label: Option<String>,
    /// Required when `disposition` is `downgraded_to_observer_no_mutation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_target_label: Option<String>,
    /// Required when `disposition` is `promoted_to_declarative_recipe`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_recipe_ref: Option<String>,
    /// Required when `disposition` is `denied_unsafe_replay`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// Guardrail: disposition did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Timestamp at which the disposition was minted.
    pub minted_at: String,
}

/// One typed audit-event row on the recorded-macro lifecycle stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaAuditEvent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque audit-event id.
    pub audit_event_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound definition.
    pub definition_ref: String,
    /// Optional ref to the bound replay disposition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_disposition_ref: Option<String>,
    /// Optional ref to the bound attribution row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution_ref: Option<String>,
    /// Typed audit-event class.
    pub event_class: AuditEventClass,
    /// Reviewable denial-reason label. Required when event_class is
    /// `macro_replay_denied`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// Timestamp at which the audit event was minted.
    pub minted_at: String,
}

/// One typed attribution row binding a replay disposition to a
/// support / activity / admin-audit surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaAttribution {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque attribution id.
    pub attribution_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound definition.
    pub definition_ref: String,
    /// Opaque ref to the bound replay disposition.
    pub replay_disposition_ref: String,
    /// Attribution-surface class.
    pub attribution_surface: AttributionSurfaceClass,
    /// Reviewable support-export summary.
    pub support_export_summary: String,
    /// Guardrail: no raw payload bytes exported.
    pub raw_payload_exported: bool,
    /// Timestamp at which the attribution was minted.
    pub minted_at: String,
}

/// Optional fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

/// One alpha page: definitions + replay dispositions + audit events +
/// attributions under one workspace surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<RecordedMacroAlphaFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contracts this page composes with by reference.
    pub contract_refs: RecordedMacroAlphaContractRefs,
    /// Recorded-macro definitions on this page.
    pub definitions: Vec<RecordedMacroAlphaDefinition>,
    /// Replay-disposition rows.
    #[serde(default)]
    pub replay_dispositions: Vec<RecordedMacroAlphaReplayDisposition>,
    /// Audit-event rows.
    #[serde(default)]
    pub audit_events: Vec<RecordedMacroAlphaAuditEvent>,
    /// Attribution rows.
    #[serde(default)]
    pub attributions: Vec<RecordedMacroAlphaAttribution>,
    /// Reviewable summary safe for support export.
    pub support_export_summary: String,
}

impl RecordedMacroAlphaPage {
    /// Validate the page against alpha invariants. Returns a structured
    /// report; the page is valid when `report.passed` is true.
    pub fn validate(&self) -> RecordedMacroAlphaValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a redaction-safe support-export projection.
    pub fn support_export_projection(&self) -> RecordedMacroAlphaSupportExport {
        let definition_summaries = self
            .definitions
            .iter()
            .map(|def| RecordedMacroDefinitionSummary {
                definition_id: def.definition_id.clone(),
                macro_id: def.macro_id.clone(),
                macro_revision_ref: def.macro_revision_ref.clone(),
                display_label: def.display_label.clone(),
                author_identity_ref: def.author_identity_ref.clone(),
                trust_gate: def.trust_gate,
                replay_limitation: def.replay_limitation,
                step_count: def.steps.len(),
                command_ids: def
                    .steps
                    .iter()
                    .map(|step| step.command_id.clone())
                    .collect(),
                mode_requirements: def
                    .steps
                    .iter()
                    .map(|step| step.mode_requirement)
                    .collect(),
                write_classes_union: write_class_union(&def.steps),
                lineage_summary: def.lineage_summary.clone(),
            })
            .collect();
        let disposition_summaries = self
            .replay_dispositions
            .iter()
            .map(|disposition| ReplayDispositionSummary {
                disposition_id: disposition.disposition_id.clone(),
                display_label: disposition.display_label.clone(),
                definition_ref: disposition.definition_ref.clone(),
                macro_id: disposition.macro_id.clone(),
                disposition: disposition.disposition,
                trust_gate_observed: disposition.trust_gate_observed,
                preview_required_reason_label: disposition.preview_required_reason_label.clone(),
                downgrade_target_label: disposition.downgrade_target_label.clone(),
                promoted_recipe_ref: disposition.promoted_recipe_ref.clone(),
                denial_reason_label: disposition.denial_reason_label.clone(),
                rationale_summary: disposition.rationale_summary.clone(),
            })
            .collect();
        let audit_summaries = self
            .audit_events
            .iter()
            .map(|event| AuditEventSummary {
                audit_event_id: event.audit_event_id.clone(),
                display_label: event.display_label.clone(),
                definition_ref: event.definition_ref.clone(),
                event_class: event.event_class,
                denial_reason_label: event.denial_reason_label.clone(),
            })
            .collect();
        let attribution_summaries = self
            .attributions
            .iter()
            .map(|attribution| AttributionSummary {
                attribution_id: attribution.attribution_id.clone(),
                display_label: attribution.display_label.clone(),
                definition_ref: attribution.definition_ref.clone(),
                replay_disposition_ref: attribution.replay_disposition_ref.clone(),
                attribution_surface: attribution.attribution_surface,
                support_export_summary: attribution.support_export_summary.clone(),
            })
            .collect();
        RecordedMacroAlphaSupportExport {
            record_kind: RECORDED_MACRO_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            definition_summaries,
            disposition_summaries,
            audit_summaries,
            attribution_summaries,
        }
    }
}

fn write_class_union(steps: &[CommandLineageStep]) -> Vec<WriteClass> {
    let mut union: BTreeSet<WriteClass> = BTreeSet::new();
    for step in steps {
        for class in &step.write_classes {
            union.insert(*class);
        }
    }
    union.into_iter().collect()
}

/// Validation report emitted by the alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// True when no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed while validating the page.
    pub coverage: RecordedMacroAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<RecordedMacroAlphaFinding>,
}

/// Coverage observed during alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RecordedMacroAlphaCoverage {
    /// Step-command-lineage classes observed.
    pub step_command_lineage_classes: BTreeSet<StepCommandLineageClass>,
    /// Mode-requirement classes observed.
    pub mode_requirement_classes: BTreeSet<ModeRequirementClass>,
    /// Write classes observed.
    pub write_classes: BTreeSet<WriteClass>,
    /// Replay-limitation classes observed.
    pub replay_limitation_classes: BTreeSet<ReplayLimitationClass>,
    /// Trust-gate classes observed.
    pub trust_gate_classes: BTreeSet<TrustGateClass>,
    /// Replay-disposition classes observed.
    pub replay_disposition_classes: BTreeSet<ReplayDispositionClass>,
    /// Audit-event classes observed.
    pub audit_event_classes: BTreeSet<AuditEventClass>,
    /// Attribution-surface classes observed.
    pub attribution_surface_classes: BTreeSet<AttributionSurfaceClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaFinding {
    /// Severity.
    pub severity: RecordedMacroAlphaFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedMacroAlphaFindingSeverity {
    /// Error that blocks the page.
    Error,
    /// Warning that keeps the page reviewable but visibly degraded.
    Warning,
}

/// Redaction-safe support-export projection of one alpha page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroAlphaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Definition summaries.
    pub definition_summaries: Vec<RecordedMacroDefinitionSummary>,
    /// Disposition summaries.
    pub disposition_summaries: Vec<ReplayDispositionSummary>,
    /// Audit-event summaries.
    pub audit_summaries: Vec<AuditEventSummary>,
    /// Attribution summaries.
    pub attribution_summaries: Vec<AttributionSummary>,
}

/// Redaction-safe summary of one definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedMacroDefinitionSummary {
    /// Definition id.
    pub definition_id: String,
    /// Macro id.
    pub macro_id: String,
    /// Macro revision ref.
    pub macro_revision_ref: String,
    /// Reviewable label.
    pub display_label: String,
    /// Author identity ref.
    pub author_identity_ref: String,
    /// Trust gate.
    pub trust_gate: TrustGateClass,
    /// Replay limitation.
    pub replay_limitation: ReplayLimitationClass,
    /// Number of steps.
    pub step_count: usize,
    /// Command ids on the command graph (one per step, in order).
    pub command_ids: Vec<String>,
    /// Mode requirements (one per step, in order).
    pub mode_requirements: Vec<ModeRequirementClass>,
    /// Union of write classes touched by the definition.
    pub write_classes_union: Vec<WriteClass>,
    /// Reviewable lineage summary.
    pub lineage_summary: String,
}

/// Redaction-safe summary of one replay disposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayDispositionSummary {
    /// Disposition id.
    pub disposition_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Definition ref.
    pub definition_ref: String,
    /// Macro id.
    pub macro_id: String,
    /// Disposition class.
    pub disposition: ReplayDispositionClass,
    /// Trust gate observed.
    pub trust_gate_observed: TrustGateClass,
    /// Optional preview-required reason label.
    pub preview_required_reason_label: Option<String>,
    /// Optional downgrade-target label.
    pub downgrade_target_label: Option<String>,
    /// Optional promoted-recipe ref.
    pub promoted_recipe_ref: Option<String>,
    /// Optional denial-reason label.
    pub denial_reason_label: Option<String>,
    /// Reviewable rationale.
    pub rationale_summary: String,
}

/// Redaction-safe summary of one audit-event row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEventSummary {
    /// Audit event id.
    pub audit_event_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Definition ref.
    pub definition_ref: String,
    /// Event class.
    pub event_class: AuditEventClass,
    /// Optional denial reason label.
    pub denial_reason_label: Option<String>,
}

/// Redaction-safe summary of one attribution row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributionSummary {
    /// Attribution id.
    pub attribution_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Definition ref.
    pub definition_ref: String,
    /// Replay disposition ref.
    pub replay_disposition_ref: String,
    /// Attribution-surface class.
    pub attribution_surface: AttributionSurfaceClass,
    /// Reviewable support-export summary.
    pub support_export_summary: String,
}

struct Validator<'a> {
    page: &'a RecordedMacroAlphaPage,
    definition_ids: BTreeSet<&'a str>,
    disposition_ids: BTreeSet<&'a str>,
    audit_event_ids: BTreeSet<&'a str>,
    attribution_ids: BTreeSet<&'a str>,
    coverage: RecordedMacroAlphaCoverage,
    findings: Vec<RecordedMacroAlphaFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a RecordedMacroAlphaPage) -> Self {
        Self {
            page,
            definition_ids: BTreeSet::new(),
            disposition_ids: BTreeSet::new(),
            audit_event_ids: BTreeSet::new(),
            attribution_ids: BTreeSet::new(),
            coverage: RecordedMacroAlphaCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_definitions();
        self.validate_replay_dispositions();
        self.validate_audit_events();
        self.validate_attributions();
        self.validate_required_coverage();
    }

    fn finish(self) -> RecordedMacroAlphaValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != RecordedMacroAlphaFindingSeverity::Error);
        RecordedMacroAlphaValidationReport {
            record_kind: RECORDED_MACRO_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == RECORDED_MACRO_ALPHA_PAGE_RECORD_KIND,
            "recorded_macro_alpha.page_record_kind",
            "page.record_kind must be recorded_macro_alpha_page_record",
        );
        self.expect(
            page.schema_version == RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
            "recorded_macro_alpha.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            page.shared_contract_ref == RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF,
            "recorded_macro_alpha.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "recorded_macro_alpha.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.support_export_summary.trim().is_empty(),
            "recorded_macro_alpha.page_support_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "recorded_macro_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.definitions.is_empty(),
            "recorded_macro_alpha.definitions_missing",
            "page must contain at least one definition",
        );
    }

    fn validate_definitions(&mut self) {
        for def in &self.page.definitions {
            self.expect(
                def.record_kind == RECORDED_MACRO_ALPHA_DEFINITION_RECORD_KIND,
                "recorded_macro_alpha.definition_record_kind",
                "definition.record_kind is wrong",
            );
            self.expect(
                def.schema_version == RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
                "recorded_macro_alpha.definition_schema_version",
                "definition.schema_version is wrong",
            );
            self.expect(
                def.shared_contract_ref == RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF,
                "recorded_macro_alpha.definition_shared_contract_ref",
                "definition.shared_contract_ref must match the shared contract id",
            );
            let unique = self.definition_ids.insert(&def.definition_id);
            self.expect(
                unique,
                "recorded_macro_alpha.definition_duplicate",
                "definition.definition_id values must be unique within a page",
            );
            self.expect(
                !def.macro_id.trim().is_empty()
                    && !def.macro_revision_ref.trim().is_empty()
                    && !def.display_label.trim().is_empty()
                    && !def.author_identity_ref.trim().is_empty()
                    && !def.lineage_summary.trim().is_empty()
                    && !def.recorded_at.trim().is_empty(),
                "recorded_macro_alpha.definition_required_field_missing",
                "definition must name macro_id, macro_revision_ref, display_label, \
                 author_identity_ref, lineage_summary, and recorded_at",
            );
            self.expect(
                !def.steps.is_empty(),
                "recorded_macro_alpha.definition_steps_missing",
                "definition must carry at least one command-lineage step",
            );
            self.expect(
                !def.raw_keystroke_bytes_present,
                "recorded_macro_alpha.definition_raw_keystroke_bytes",
                "definition.raw_keystroke_bytes_present must be false",
            );
            self.expect(
                !def.raw_buffer_bytes_present,
                "recorded_macro_alpha.definition_raw_buffer_bytes",
                "definition.raw_buffer_bytes_present must be false",
            );
            self.expect(
                !def.raw_shell_fragment_present,
                "recorded_macro_alpha.definition_raw_shell_fragment",
                "definition.raw_shell_fragment_present must be false",
            );
            self.expect(
                !def.raw_credential_present,
                "recorded_macro_alpha.definition_raw_credential",
                "definition.raw_credential_present must be false",
            );
            self.expect(
                !def.silent_authority_widening_taken,
                "recorded_macro_alpha.definition_silent_authority_widening",
                "definition.silent_authority_widening_taken must be false",
            );
            self.expect(
                def.support_attribution_minted,
                "recorded_macro_alpha.definition_support_attribution_missing",
                "definition.support_attribution_minted must be true; macros must be \
                 attributable in support exports",
            );
            self.expect(
                def.activity_attribution_minted,
                "recorded_macro_alpha.definition_activity_attribution_missing",
                "definition.activity_attribution_minted must be true; macros must be \
                 attributable in activity history",
            );
            self.expect(
                def.trust_gate != TrustGateClass::ManagedOnlyDenied,
                "recorded_macro_alpha.definition_managed_only_denied",
                "definitions may not record a managed_only_denied trust gate; recorded \
                 macros are never admissible on the managed-only channel",
            );

            self.coverage.trust_gate_classes.insert(def.trust_gate);
            self.coverage
                .replay_limitation_classes
                .insert(def.replay_limitation);

            self.validate_definition_steps(def);
        }
    }

    fn validate_definition_steps(&mut self, def: &'a RecordedMacroAlphaDefinition) {
        let mut step_ids: BTreeSet<&str> = BTreeSet::new();
        let mut any_forces_non_proceed = false;
        for step in &def.steps {
            self.expect(
                !step.step_id.trim().is_empty()
                    && !step.command_id.trim().is_empty()
                    && !step.command_revision_ref.trim().is_empty()
                    && !step.display_label.trim().is_empty(),
                "recorded_macro_alpha.step_required_field_missing",
                "step must name step_id, command_id, command_revision_ref, and display_label",
            );
            self.expect(
                step_ids.insert(step.step_id.as_str()),
                "recorded_macro_alpha.step_duplicate",
                "step.step_id values must be unique within a definition",
            );
            self.expect(
                !step.write_classes.is_empty(),
                "recorded_macro_alpha.step_write_classes_missing",
                "step must declare at least one write_class",
            );
            if step.step_command_lineage.requires_ai_tool_handle_ref() {
                self.expect(
                    step.ai_tool_handle_ref
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty()),
                    "recorded_macro_alpha.step_ai_tool_handle_ref_missing",
                    "ai_tool_handle steps must cite an ai_tool_handle_ref",
                );
            }

            self.coverage
                .step_command_lineage_classes
                .insert(step.step_command_lineage);
            self.coverage
                .mode_requirement_classes
                .insert(step.mode_requirement);
            for class in &step.write_classes {
                self.coverage.write_classes.insert(*class);
            }
            if step.step_command_lineage.forces_non_proceed_disposition()
                || step.mode_requirement.forces_non_proceed_disposition()
                || step
                    .write_classes
                    .iter()
                    .any(|class| class.forces_non_proceed_disposition())
            {
                any_forces_non_proceed = true;
            }
        }
        if any_forces_non_proceed {
            self.expect(
                def.replay_limitation != ReplayLimitationClass::SingleBufferSafe,
                "recorded_macro_alpha.definition_silent_proceed_denied",
                "definition contains a step whose lineage, mode requirement, or write class \
                 forces a non-proceed disposition; replay_limitation must not be \
                 single_buffer_safe",
            );
        }
    }

    fn validate_replay_dispositions(&mut self) {
        for disposition in &self.page.replay_dispositions {
            self.expect(
                disposition.record_kind == RECORDED_MACRO_ALPHA_REPLAY_DISPOSITION_RECORD_KIND,
                "recorded_macro_alpha.disposition_record_kind",
                "disposition.record_kind is wrong",
            );
            self.expect(
                disposition.schema_version == RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
                "recorded_macro_alpha.disposition_schema_version",
                "disposition.schema_version is wrong",
            );
            self.expect(
                disposition.shared_contract_ref == RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF,
                "recorded_macro_alpha.disposition_shared_contract_ref",
                "disposition.shared_contract_ref must match the shared contract id",
            );
            let unique = self.disposition_ids.insert(&disposition.disposition_id);
            self.expect(
                unique,
                "recorded_macro_alpha.disposition_duplicate",
                "disposition.disposition_id values must be unique within a page",
            );
            self.expect(
                self.definition_ids
                    .contains(disposition.definition_ref.as_str()),
                "recorded_macro_alpha.disposition_definition_ref_unknown",
                "disposition.definition_ref must resolve to a definition on the page",
            );
            self.expect(
                !disposition.display_label.trim().is_empty()
                    && !disposition.macro_id.trim().is_empty()
                    && !disposition.macro_revision_ref.trim().is_empty()
                    && !disposition.rationale_summary.trim().is_empty()
                    && !disposition.support_attribution_ref.trim().is_empty()
                    && !disposition.activity_attribution_ref.trim().is_empty()
                    && !disposition.minted_at.trim().is_empty(),
                "recorded_macro_alpha.disposition_required_field_missing",
                "disposition must name display_label, macro_id, macro_revision_ref, \
                 rationale_summary, support_attribution_ref, activity_attribution_ref, and \
                 minted_at",
            );
            self.expect(
                !disposition.silent_authority_widening_taken,
                "recorded_macro_alpha.disposition_silent_authority_widening",
                "disposition.silent_authority_widening_taken must be false",
            );
            let non_empty = |opt: &Option<String>| {
                opt.as_deref().is_some_and(|value| !value.trim().is_empty())
            };
            if disposition.disposition.requires_preview_reason() {
                self.expect(
                    non_empty(&disposition.preview_required_reason_label),
                    "recorded_macro_alpha.disposition_preview_reason_missing",
                    "preview_required_before_apply dispositions must cite a \
                     preview_required_reason_label",
                );
            } else {
                self.expect(
                    disposition.preview_required_reason_label.is_none(),
                    "recorded_macro_alpha.disposition_preview_reason_unexpected",
                    "non-preview dispositions must not cite a preview_required_reason_label",
                );
            }
            if disposition.disposition.requires_downgrade_target() {
                self.expect(
                    non_empty(&disposition.downgrade_target_label),
                    "recorded_macro_alpha.disposition_downgrade_target_missing",
                    "downgraded_to_observer_no_mutation dispositions must cite a \
                     downgrade_target_label",
                );
            } else {
                self.expect(
                    disposition.downgrade_target_label.is_none(),
                    "recorded_macro_alpha.disposition_downgrade_target_unexpected",
                    "non-downgrade dispositions must not cite a downgrade_target_label",
                );
            }
            if disposition.disposition.requires_promoted_recipe_ref() {
                self.expect(
                    non_empty(&disposition.promoted_recipe_ref),
                    "recorded_macro_alpha.disposition_promoted_recipe_missing",
                    "promoted_to_declarative_recipe dispositions must cite a \
                     promoted_recipe_ref",
                );
            } else {
                self.expect(
                    disposition.promoted_recipe_ref.is_none(),
                    "recorded_macro_alpha.disposition_promoted_recipe_unexpected",
                    "non-promotion dispositions must not cite a promoted_recipe_ref",
                );
            }
            if disposition.disposition.requires_denial_reason() {
                self.expect(
                    non_empty(&disposition.denial_reason_label),
                    "recorded_macro_alpha.disposition_denial_reason_missing",
                    "denied_unsafe_replay dispositions must cite a denial_reason_label",
                );
            } else {
                self.expect(
                    disposition.denial_reason_label.is_none(),
                    "recorded_macro_alpha.disposition_denial_reason_unexpected",
                    "non-denial dispositions must not cite a denial_reason_label",
                );
            }
            self.expect(
                disposition.trust_gate_observed != TrustGateClass::ManagedOnlyDenied,
                "recorded_macro_alpha.disposition_managed_only_denied",
                "dispositions may not observe a managed_only_denied trust gate",
            );

            if let Some(def) = self
                .page
                .definitions
                .iter()
                .find(|d| d.definition_id == disposition.definition_ref)
            {
                if let Some(forced) = def.replay_limitation.forced_disposition() {
                    self.expect(
                        disposition.disposition == forced,
                        "recorded_macro_alpha.disposition_does_not_match_limitation",
                        "disposition.disposition must match the limitation-forced disposition \
                         on the bound definition",
                    );
                }
            }

            self.coverage
                .replay_disposition_classes
                .insert(disposition.disposition);
        }
    }

    fn validate_audit_events(&mut self) {
        for event in &self.page.audit_events {
            self.expect(
                event.record_kind == RECORDED_MACRO_ALPHA_AUDIT_EVENT_RECORD_KIND,
                "recorded_macro_alpha.audit_event_record_kind",
                "audit_event.record_kind is wrong",
            );
            self.expect(
                event.schema_version == RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
                "recorded_macro_alpha.audit_event_schema_version",
                "audit_event.schema_version is wrong",
            );
            self.expect(
                event.shared_contract_ref == RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF,
                "recorded_macro_alpha.audit_event_shared_contract_ref",
                "audit_event.shared_contract_ref must match the shared contract id",
            );
            let unique = self.audit_event_ids.insert(&event.audit_event_id);
            self.expect(
                unique,
                "recorded_macro_alpha.audit_event_duplicate",
                "audit_event.audit_event_id values must be unique within a page",
            );
            self.expect(
                !event.display_label.trim().is_empty() && !event.minted_at.trim().is_empty(),
                "recorded_macro_alpha.audit_event_required_field_missing",
                "audit_event must name display_label and minted_at",
            );
            self.expect(
                self.definition_ids
                    .contains(event.definition_ref.as_str()),
                "recorded_macro_alpha.audit_event_definition_ref_unknown",
                "audit_event.definition_ref must resolve to a definition on the page",
            );
            if event.event_class.requires_disposition_ref() {
                let resolves = event
                    .replay_disposition_ref
                    .as_deref()
                    .is_some_and(|value| self.disposition_ids.contains(value));
                self.expect(
                    resolves,
                    "recorded_macro_alpha.audit_event_disposition_ref_unknown",
                    "disposition-bound audit events must cite a replay_disposition_ref \
                     resolving to a disposition on the page",
                );
            }
            if event.event_class.requires_attribution_ref() {
                let resolves = event
                    .attribution_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty());
                self.expect(
                    resolves,
                    "recorded_macro_alpha.audit_event_attribution_ref_missing",
                    "macro_attribution_minted events must cite an attribution_ref",
                );
            }
            if event.event_class.requires_denial_reason() {
                self.expect(
                    event
                        .denial_reason_label
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty()),
                    "recorded_macro_alpha.audit_event_denial_reason_missing",
                    "macro_replay_denied events must cite a denial_reason_label",
                );
            }

            self.coverage.audit_event_classes.insert(event.event_class);
        }
    }

    fn validate_attributions(&mut self) {
        for attribution in &self.page.attributions {
            self.expect(
                attribution.record_kind == RECORDED_MACRO_ALPHA_ATTRIBUTION_RECORD_KIND,
                "recorded_macro_alpha.attribution_record_kind",
                "attribution.record_kind is wrong",
            );
            self.expect(
                attribution.schema_version == RECORDED_MACRO_ALPHA_SCHEMA_VERSION,
                "recorded_macro_alpha.attribution_schema_version",
                "attribution.schema_version is wrong",
            );
            self.expect(
                attribution.shared_contract_ref == RECORDED_MACRO_ALPHA_SHARED_CONTRACT_REF,
                "recorded_macro_alpha.attribution_shared_contract_ref",
                "attribution.shared_contract_ref must match the shared contract id",
            );
            let unique = self.attribution_ids.insert(&attribution.attribution_id);
            self.expect(
                unique,
                "recorded_macro_alpha.attribution_duplicate",
                "attribution.attribution_id values must be unique within a page",
            );
            self.expect(
                !attribution.display_label.trim().is_empty()
                    && !attribution.support_export_summary.trim().is_empty()
                    && !attribution.minted_at.trim().is_empty(),
                "recorded_macro_alpha.attribution_required_field_missing",
                "attribution must name display_label, support_export_summary, and minted_at",
            );
            self.expect(
                self.definition_ids
                    .contains(attribution.definition_ref.as_str()),
                "recorded_macro_alpha.attribution_definition_ref_unknown",
                "attribution.definition_ref must resolve to a definition on the page",
            );
            self.expect(
                self.disposition_ids
                    .contains(attribution.replay_disposition_ref.as_str()),
                "recorded_macro_alpha.attribution_disposition_ref_unknown",
                "attribution.replay_disposition_ref must resolve to a disposition on the page",
            );
            self.expect(
                !attribution.raw_payload_exported,
                "recorded_macro_alpha.attribution_raw_payload_exported",
                "attribution.raw_payload_exported must be false",
            );

            self.coverage
                .attribution_surface_classes
                .insert(attribution.attribution_surface);
        }
    }

    fn validate_required_coverage(&mut self) {
        for required in [
            ReplayDispositionClass::ProceedLocalEditorOnly,
            ReplayDispositionClass::PreviewRequiredBeforeApply,
            ReplayDispositionClass::DowngradedToObserverNoMutation,
            ReplayDispositionClass::PromotedToDeclarativeRecipe,
            ReplayDispositionClass::DeniedUnsafeReplay,
        ] {
            self.expect(
                self.coverage.replay_disposition_classes.contains(&required),
                "recorded_macro_alpha.coverage_replay_disposition_missing",
                "page must cover every required replay-disposition class",
            );
        }
        for required in [
            AttributionSurfaceClass::SupportExport,
            AttributionSurfaceClass::ActivityHistory,
        ] {
            self.expect(
                self.coverage
                    .attribution_surface_classes
                    .contains(&required),
                "recorded_macro_alpha.coverage_attribution_surface_missing",
                "page must cover support_export and activity_history attribution surfaces",
            );
        }
        for required in [
            AuditEventClass::MacroRecorded,
            AuditEventClass::MacroReplayRequested,
            AuditEventClass::MacroReplayAdmitted,
            AuditEventClass::MacroReplayDenied,
            AuditEventClass::MacroAttributionMinted,
        ] {
            self.expect(
                self.coverage.audit_event_classes.contains(&required),
                "recorded_macro_alpha.coverage_audit_event_missing",
                "page must cover every required audit-event class",
            );
        }
    }

    fn expect(&mut self, predicate: bool, check_id: &str, message: &str) {
        if !predicate {
            self.findings.push(RecordedMacroAlphaFinding {
                severity: RecordedMacroAlphaFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forced_disposition_table_matches_limitation_class() {
        assert_eq!(
            ReplayLimitationClass::SingleBufferSafe.forced_disposition(),
            None
        );
        assert_eq!(
            ReplayLimitationClass::MultiBufferRequiresPreview.forced_disposition(),
            Some(ReplayDispositionClass::PreviewRequiredBeforeApply)
        );
        assert_eq!(
            ReplayLimitationClass::CrossesWorkspaceBoundaryRequiresRecipePromotion
                .forced_disposition(),
            Some(ReplayDispositionClass::PromotedToDeclarativeRecipe)
        );
        assert_eq!(
            ReplayLimitationClass::NonReplayableUnstableTiming.forced_disposition(),
            Some(ReplayDispositionClass::DeniedUnsafeReplay)
        );
        assert_eq!(
            ReplayLimitationClass::DeniedUnmappedCommandPresent.forced_disposition(),
            Some(ReplayDispositionClass::DeniedUnsafeReplay)
        );
    }

    #[test]
    fn denied_write_classes_force_non_proceed() {
        assert!(WriteClass::SettingsMutationDenied.forces_non_proceed_disposition());
        assert!(WriteClass::NetworkMutationDenied.forces_non_proceed_disposition());
        assert!(WriteClass::ProcessMutationDenied.forces_non_proceed_disposition());
        assert!(WriteClass::CredentialMutationDenied.forces_non_proceed_disposition());
        assert!(!WriteClass::ReadOnly.forces_non_proceed_disposition());
        assert!(!WriteClass::EditorBufferMutation.forces_non_proceed_disposition());
        assert!(WriteClass::EditorMultiFileMutation.forces_non_proceed_disposition());
    }

    #[test]
    fn unmapped_keystroke_forces_non_proceed() {
        assert!(StepCommandLineageClass::UnmappedKeystrokeDenied.forces_non_proceed_disposition());
        assert!(!StepCommandLineageClass::CoreCommand.forces_non_proceed_disposition());
    }

    #[test]
    fn terminal_mode_forces_non_proceed() {
        assert!(ModeRequirementClass::TerminalModeDenied.forces_non_proceed_disposition());
        assert!(!ModeRequirementClass::EditorAnyModeAdmissible.forces_non_proceed_disposition());
    }
}
