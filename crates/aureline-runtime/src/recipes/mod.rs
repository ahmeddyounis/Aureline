//! Declarative recipe alpha record family with reviewability, approval
//! fences, and command-graph parity.
//!
//! A recipe is a reusable, declarative workflow expressed as an ordered
//! sequence of typed steps. Each step resolves to a stable
//! `command_id` and `approval_class` on the shared command graph;
//! recipes never invent bespoke automation shortcuts. Reviewers, support
//! exports, and the activity history read one truth: definitions are
//! diffable, mutating or provider-facing steps preserve preview and
//! approval behavior at run time, and every run resolves to exactly one
//! [`RecipeRunDispositionClass`].
//!
//! The cross-tool boundary lives at
//! [`/schemas/commands/recipe.schema.json`](../../../../schemas/commands/recipe.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/recipe_alpha.md`](../../../../docs/runtime/m3/recipe_alpha.md).
//! The reviewer fixture is
//! [`/fixtures/runtime/recipe_alpha/page.json`](../../../../fixtures/runtime/recipe_alpha/page.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Alpha schema version exported with every recipe record.
pub const RECIPE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record in this family.
pub const RECIPE_ALPHA_SHARED_CONTRACT_REF: &str = "commands:recipe_alpha:v1";

/// Stable record-kind tag for [`RecipeAlphaPage`] payloads.
pub const RECIPE_ALPHA_PAGE_RECORD_KIND: &str = "recipe_alpha_page_record";

/// Stable record-kind tag for [`RecipeDefinition`] payloads.
pub const RECIPE_ALPHA_DEFINITION_RECORD_KIND: &str = "recipe_alpha_definition_record";

/// Stable record-kind tag for [`RecipeRun`] payloads.
pub const RECIPE_ALPHA_RUN_RECORD_KIND: &str = "recipe_alpha_run_record";

/// Stable record-kind tag for [`RecipeAuditEvent`] payloads.
pub const RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND: &str = "recipe_alpha_audit_event_record";

/// Stable record-kind tag for [`RecipeAttribution`] payloads.
pub const RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND: &str = "recipe_alpha_attribution_record";

/// Stable record-kind tag for [`RecipeAlphaValidationReport`] payloads.
pub const RECIPE_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str = "recipe_alpha_validation_report";

/// Stable record-kind tag for the redaction-safe support-export
/// projection.
pub const RECIPE_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str = "recipe_alpha_support_export";

/// Closed vocabulary for one step's command lineage on the shared
/// command graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepCommandLineageClass {
    CoreCommand,
    ImportedCommand,
    ExtensionCommand,
    AiToolHandle,
    CliVerb,
    ProviderAction,
    UnmappedCommandDenied,
}

impl StepCommandLineageClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreCommand => "core_command",
            Self::ImportedCommand => "imported_command",
            Self::ExtensionCommand => "extension_command",
            Self::AiToolHandle => "ai_tool_handle",
            Self::CliVerb => "cli_verb",
            Self::ProviderAction => "provider_action",
            Self::UnmappedCommandDenied => "unmapped_command_denied",
        }
    }

    /// True when the step must cite an `ai_tool_handle_ref`.
    pub const fn requires_ai_tool_handle(self) -> bool {
        matches!(self, Self::AiToolHandle)
    }

    /// True when the step must cite a `provider_action_ref`.
    pub const fn requires_provider_action_ref(self) -> bool {
        matches!(self, Self::ProviderAction)
    }
}

/// Closed vocabulary for one step's mode requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepModeRequirementClass {
    EditorNormalModeRequired,
    EditorAnyModeAdmissible,
    PaletteModeRequired,
    TerminalModeDenied,
    BackgroundAdmissible,
}

impl StepModeRequirementClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorNormalModeRequired => "editor_normal_mode_required",
            Self::EditorAnyModeAdmissible => "editor_any_mode_admissible",
            Self::PaletteModeRequired => "palette_mode_required",
            Self::TerminalModeDenied => "terminal_mode_denied",
            Self::BackgroundAdmissible => "background_admissible",
        }
    }

    /// True when the mode requirement denies execution from a recipe.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::TerminalModeDenied)
    }
}

/// Closed vocabulary for one write-scope class on a recipe step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeWriteClass {
    ReadOnly,
    EditorBufferMutation,
    EditorMultiFileMutation,
    BranchMutation,
    WorktreeMutation,
    ProviderMutation,
    SettingsMutation,
    NetworkMutation,
    ProcessMutation,
    CredentialMutationDenied,
}

impl RecipeWriteClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::EditorBufferMutation => "editor_buffer_mutation",
            Self::EditorMultiFileMutation => "editor_multi_file_mutation",
            Self::BranchMutation => "branch_mutation",
            Self::WorktreeMutation => "worktree_mutation",
            Self::ProviderMutation => "provider_mutation",
            Self::SettingsMutation => "settings_mutation",
            Self::NetworkMutation => "network_mutation",
            Self::ProcessMutation => "process_mutation",
            Self::CredentialMutationDenied => "credential_mutation_denied",
        }
    }

    /// True when this write class is the closed denial lane (must never
    /// appear on an admitted step).
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::CredentialMutationDenied)
    }

    /// True when the write class is mutating (anything other than
    /// read-only or the denied lane).
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::ReadOnly | Self::CredentialMutationDenied)
    }

    /// True when the write class names a provider-facing mutation.
    pub const fn is_provider_facing(self) -> bool {
        matches!(self, Self::ProviderMutation)
    }

    /// True when the write class names a branch/worktree mutation.
    pub const fn is_branch_or_worktree(self) -> bool {
        matches!(self, Self::BranchMutation | Self::WorktreeMutation)
    }
}

/// Closed vocabulary for one approval class on a recipe step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeApprovalClass {
    NoApprovalRequired,
    SingleStepApprovalRequired,
    RecipeApprovalRequired,
    AdminSignedApprovalRequired,
    ApprovalDeniedUnsafe,
}

impl RecipeApprovalClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalRequired => "no_approval_required",
            Self::SingleStepApprovalRequired => "single_step_approval_required",
            Self::RecipeApprovalRequired => "recipe_approval_required",
            Self::AdminSignedApprovalRequired => "admin_signed_approval_required",
            Self::ApprovalDeniedUnsafe => "approval_denied_unsafe",
        }
    }

    /// True when this approval class is the closed denial lane.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::ApprovalDeniedUnsafe)
    }
}

/// Closed vocabulary for one preview requirement on a recipe step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipePreviewRequirementClass {
    NoPreviewRequired,
    PreviewRequiredBeforeApply,
    PreviewOptionalLocalOnly,
    PreviewRequiredProviderMutation,
    PreviewRequiredBranchMutation,
    PreviewRequiredWorktreeMutation,
}

impl RecipePreviewRequirementClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewRequired => "no_preview_required",
            Self::PreviewRequiredBeforeApply => "preview_required_before_apply",
            Self::PreviewOptionalLocalOnly => "preview_optional_local_only",
            Self::PreviewRequiredProviderMutation => "preview_required_provider_mutation",
            Self::PreviewRequiredBranchMutation => "preview_required_branch_mutation",
            Self::PreviewRequiredWorktreeMutation => "preview_required_worktree_mutation",
        }
    }

    /// True when this preview requirement preserves preview behavior.
    pub const fn requires_preview(self) -> bool {
        matches!(
            self,
            Self::PreviewRequiredBeforeApply
                | Self::PreviewRequiredProviderMutation
                | Self::PreviewRequiredBranchMutation
                | Self::PreviewRequiredWorktreeMutation
        )
    }
}

/// Closed vocabulary for one workspace-trust posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeTrustGateClass {
    RestrictedWorkspaceAdmissible,
    TrustedWorkspaceRequired,
    AdminPolicyObserved,
    ManagedOnlyDenied,
}

impl RecipeTrustGateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestrictedWorkspaceAdmissible => "restricted_workspace_admissible",
            Self::TrustedWorkspaceRequired => "trusted_workspace_required",
            Self::AdminPolicyObserved => "admin_policy_observed",
            Self::ManagedOnlyDenied => "managed_only_denied",
        }
    }

    /// True when this gate is the closed denial lane.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::ManagedOnlyDenied)
    }
}

/// Closed vocabulary for one per-step run disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeStepDispositionClass {
    ProceedNoApproval,
    ProceedAfterPreview,
    ProceedAfterApproval,
    PreviewRequiredBeforeApply,
    DowngradedToObserverNoMutation,
    DeniedUnsafeStep,
}

impl RecipeStepDispositionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedNoApproval => "proceed_no_approval",
            Self::ProceedAfterPreview => "proceed_after_preview",
            Self::ProceedAfterApproval => "proceed_after_approval",
            Self::PreviewRequiredBeforeApply => "preview_required_before_apply",
            Self::DowngradedToObserverNoMutation => "downgraded_to_observer_no_mutation",
            Self::DeniedUnsafeStep => "denied_unsafe_step",
        }
    }
}

/// Closed vocabulary for one per-recipe-run disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeRunDispositionClass {
    ProceedLocalEditorOnly,
    ProceedAfterRecipeApproval,
    PreviewRequiredBeforeApply,
    DowngradedToObserverNoMutation,
    PromotedToFullRecipeRun,
    DeniedUnsafeRecipe,
}

impl RecipeRunDispositionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedLocalEditorOnly => "proceed_local_editor_only",
            Self::ProceedAfterRecipeApproval => "proceed_after_recipe_approval",
            Self::PreviewRequiredBeforeApply => "preview_required_before_apply",
            Self::DowngradedToObserverNoMutation => "downgraded_to_observer_no_mutation",
            Self::PromotedToFullRecipeRun => "promoted_to_full_recipe_run",
            Self::DeniedUnsafeRecipe => "denied_unsafe_recipe",
        }
    }

    /// True when the run must cite an `approval_ticket_ref`.
    pub const fn requires_approval_ticket(self) -> bool {
        matches!(self, Self::ProceedAfterRecipeApproval)
    }

    /// True when the run must cite a `preview_ticket_ref`.
    pub const fn requires_preview_ticket(self) -> bool {
        matches!(self, Self::PreviewRequiredBeforeApply)
    }

    /// True when the run must cite a `promoted_run_ref`.
    pub const fn requires_promoted_run_ref(self) -> bool {
        matches!(self, Self::PromotedToFullRecipeRun)
    }

    /// True when the run must cite a `downgrade_target_label`.
    pub const fn requires_downgrade_target_label(self) -> bool {
        matches!(self, Self::DowngradedToObserverNoMutation)
    }

    /// True when the run must cite a `denial_reason` and label.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::DeniedUnsafeRecipe)
    }
}

/// Closed vocabulary for one audit-event class on the recipe stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeAuditEventClass {
    RecipeAdmitted,
    RecipeDenied,
    RecipeStepPreviewMinted,
    RecipeStepApprovalMinted,
    RecipeStepAdmitted,
    RecipeStepDenied,
    RecipeRunStarted,
    RecipeRunCompleted,
    RecipeRunAborted,
    RecipeRunPromotedToApprovedRun,
    AttributionMinted,
    AuditDenialEmitted,
}

impl RecipeAuditEventClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecipeAdmitted => "recipe_admitted",
            Self::RecipeDenied => "recipe_denied",
            Self::RecipeStepPreviewMinted => "recipe_step_preview_minted",
            Self::RecipeStepApprovalMinted => "recipe_step_approval_minted",
            Self::RecipeStepAdmitted => "recipe_step_admitted",
            Self::RecipeStepDenied => "recipe_step_denied",
            Self::RecipeRunStarted => "recipe_run_started",
            Self::RecipeRunCompleted => "recipe_run_completed",
            Self::RecipeRunAborted => "recipe_run_aborted",
            Self::RecipeRunPromotedToApprovedRun => "recipe_run_promoted_to_approved_run",
            Self::AttributionMinted => "attribution_minted",
            Self::AuditDenialEmitted => "audit_denial_emitted",
        }
    }

    /// True when the audit event must cite a `denial_reason_label`.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(
            self,
            Self::RecipeDenied | Self::RecipeStepDenied | Self::AuditDenialEmitted
        )
    }

    /// True when the audit event must cite a `run_ref`.
    pub const fn requires_run_ref(self) -> bool {
        matches!(
            self,
            Self::RecipeRunStarted
                | Self::RecipeRunCompleted
                | Self::RecipeRunAborted
                | Self::RecipeRunPromotedToApprovedRun
        )
    }

    /// True when the audit event must cite an `attribution_ref`.
    pub const fn requires_attribution_ref(self) -> bool {
        matches!(self, Self::AttributionMinted)
    }
}

/// Closed vocabulary for one attribution surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeAttributionSurfaceClass {
    SupportExport,
    ActivityHistory,
    AdminAuditExport,
}

impl RecipeAttributionSurfaceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExport => "support_export",
            Self::ActivityHistory => "activity_history",
            Self::AdminAuditExport => "admin_audit_export",
        }
    }
}

/// Closed vocabulary for one denial reason on a recipe run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeDenialReasonClass {
    UnmappedCommandDenied,
    CredentialMutationDenied,
    TerminalModeDenied,
    ManagedOnlyDenied,
    AdminPolicyDenied,
    PreviewMissingForMutation,
    ApprovalMissingForProviderMutation,
    ApprovalMissingForBranchMutation,
    ApprovalMissingForWorktreeMutation,
    TrustGateNarrowed,
    RemoteAttachDegradedUnmasked,
}

impl RecipeDenialReasonClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnmappedCommandDenied => "unmapped_command_denied",
            Self::CredentialMutationDenied => "credential_mutation_denied",
            Self::TerminalModeDenied => "terminal_mode_denied",
            Self::ManagedOnlyDenied => "managed_only_denied",
            Self::AdminPolicyDenied => "admin_policy_denied",
            Self::PreviewMissingForMutation => "preview_missing_for_mutation",
            Self::ApprovalMissingForProviderMutation => "approval_missing_for_provider_mutation",
            Self::ApprovalMissingForBranchMutation => "approval_missing_for_branch_mutation",
            Self::ApprovalMissingForWorktreeMutation => "approval_missing_for_worktree_mutation",
            Self::TrustGateNarrowed => "trust_gate_narrowed",
            Self::RemoteAttachDegradedUnmasked => "remote_attach_degraded_unmasked",
        }
    }
}

/// Upstream schemas this recipe-alpha page composes with by reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaContractRefs {
    pub command_descriptor_schema_ref: String,
    pub recorded_macro_schema_ref: String,
    pub approval_ticket_schema_ref: String,
    pub shareability_metadata_schema_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_object_schema_ref: Option<String>,
}

impl RecipeAlphaContractRefs {
    fn required_refs(&self) -> [&str; 4] {
        [
            &self.command_descriptor_schema_ref,
            &self.recorded_macro_schema_ref,
            &self.approval_ticket_schema_ref,
            &self.shareability_metadata_schema_ref,
        ]
    }
}

/// One typed step in a declarative recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeStep {
    pub step_id: String,
    pub step_command_lineage: StepCommandLineageClass,
    pub command_id: String,
    pub command_revision_ref: String,
    pub display_label: String,
    pub mode_requirement: StepModeRequirementClass,
    pub write_classes: Vec<RecipeWriteClass>,
    pub approval_class: RecipeApprovalClass,
    pub preview_requirement: RecipePreviewRequirementClass,
    pub shareability_record_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai_tool_handle_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_action_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale_summary: Option<String>,
}

/// One typed declarative-recipe definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeDefinition {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub definition_id: String,
    pub recipe_id: String,
    pub recipe_revision_ref: String,
    pub display_label: String,
    pub author_identity_ref: String,
    pub trust_gate: RecipeTrustGateClass,
    pub steps: Vec<RecipeStep>,
    pub declared_write_classes: Vec<RecipeWriteClass>,
    pub declared_approval_classes: Vec<RecipeApprovalClass>,
    pub lineage_summary: String,
    pub raw_branch_mutation_present: bool,
    pub raw_worktree_mutation_present: bool,
    pub raw_provider_payload_present: bool,
    pub raw_credential_present: bool,
    pub silent_authority_widening_taken: bool,
    pub remote_attach_degraded_state_masked: bool,
    pub support_attribution_minted: bool,
    pub activity_attribution_minted: bool,
    pub recorded_at: String,
}

/// One per-step disposition observed during a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeStepDisposition {
    pub step_ref: String,
    pub step_disposition: RecipeStepDispositionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<RecipeDenialReasonClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale_summary: Option<String>,
}

/// One typed recipe-run record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeRun {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub run_id: String,
    pub definition_ref: String,
    pub display_label: String,
    pub actor_identity_ref: String,
    pub trust_gate_observed: RecipeTrustGateClass,
    pub run_disposition: RecipeRunDispositionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_ticket_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_run_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<RecipeDenialReasonClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_target_label: Option<String>,
    pub step_dispositions: Vec<RecipeStepDisposition>,
    pub audit_event_refs: Vec<String>,
    pub support_attribution_minted: bool,
    pub activity_attribution_minted: bool,
    pub silent_authority_widening_taken: bool,
    pub remote_attach_degraded_state_masked: bool,
    pub started_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
}

/// One audit-event row on the recipe-alpha audit stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAuditEvent {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub audit_event_id: String,
    pub display_label: String,
    pub event_class: RecipeAuditEventClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    pub minted_at: String,
}

/// One redaction-safe attribution row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAttribution {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub attribution_id: String,
    pub display_label: String,
    pub attribution_surface: RecipeAttributionSurfaceClass,
    pub definition_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_ref: Option<String>,
    pub rationale_summary: String,
    pub minted_at: String,
}

/// Optional fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaFixtureMetadata {
    pub name: String,
    pub scenario: String,
}

/// One alpha page: recipe definitions + runs + audit events + attributions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaPage {
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<RecipeAlphaFixtureMetadata>,
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub contract_refs: RecipeAlphaContractRefs,
    pub definitions: Vec<RecipeDefinition>,
    pub runs: Vec<RecipeRun>,
    pub audit_events: Vec<RecipeAuditEvent>,
    pub attributions: Vec<RecipeAttribution>,
    pub support_export_summary: String,
}

impl RecipeAlphaPage {
    /// Validate the page against alpha invariants. The page is valid when
    /// the returned report's `passed` is true.
    pub fn validate(&self) -> RecipeAlphaValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a redaction-safe support-export projection.
    pub fn support_export_projection(&self) -> RecipeAlphaSupportExport {
        let definition_summaries = self
            .definitions
            .iter()
            .map(|definition| RecipeDefinitionSummary {
                definition_id: definition.definition_id.clone(),
                recipe_id: definition.recipe_id.clone(),
                recipe_revision_ref: definition.recipe_revision_ref.clone(),
                display_label: definition.display_label.clone(),
                trust_gate: definition.trust_gate,
                declared_write_classes: definition.declared_write_classes.clone(),
                declared_approval_classes: definition.declared_approval_classes.clone(),
                lineage_summary: definition.lineage_summary.clone(),
                step_count: definition.steps.len(),
            })
            .collect();
        let run_summaries = self
            .runs
            .iter()
            .map(|run| RecipeRunSummary {
                run_id: run.run_id.clone(),
                definition_ref: run.definition_ref.clone(),
                display_label: run.display_label.clone(),
                trust_gate_observed: run.trust_gate_observed,
                run_disposition: run.run_disposition,
                denial_reason: run.denial_reason,
                denial_reason_label: run.denial_reason_label.clone(),
                downgrade_target_label: run.downgrade_target_label.clone(),
                approval_ticket_ref: run.approval_ticket_ref.clone(),
                preview_ticket_ref: run.preview_ticket_ref.clone(),
                promoted_run_ref: run.promoted_run_ref.clone(),
            })
            .collect();
        let audit_summaries = self
            .audit_events
            .iter()
            .map(|event| RecipeAuditEventSummary {
                audit_event_id: event.audit_event_id.clone(),
                display_label: event.display_label.clone(),
                event_class: event.event_class,
                denial_reason_label: event.denial_reason_label.clone(),
            })
            .collect();
        let attribution_summaries = self
            .attributions
            .iter()
            .map(|attribution| RecipeAttributionSummary {
                attribution_id: attribution.attribution_id.clone(),
                display_label: attribution.display_label.clone(),
                attribution_surface: attribution.attribution_surface,
                definition_ref: attribution.definition_ref.clone(),
                run_ref: attribution.run_ref.clone(),
                rationale_summary: attribution.rationale_summary.clone(),
            })
            .collect();
        RecipeAlphaSupportExport {
            record_kind: RECIPE_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            definition_summaries,
            run_summaries,
            audit_summaries,
            attribution_summaries,
        }
    }
}

/// Validation report emitted by the recipe-alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaValidationReport {
    pub record_kind: String,
    pub schema_version: u32,
    pub page_id: String,
    pub passed: bool,
    pub coverage: RecipeAlphaCoverage,
    pub findings: Vec<RecipeAlphaFinding>,
}

/// Coverage observed during validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RecipeAlphaCoverage {
    pub step_lineages: BTreeSet<StepCommandLineageClass>,
    pub write_classes: BTreeSet<RecipeWriteClass>,
    pub approval_classes: BTreeSet<RecipeApprovalClass>,
    pub preview_requirements: BTreeSet<RecipePreviewRequirementClass>,
    pub trust_gates: BTreeSet<RecipeTrustGateClass>,
    pub step_dispositions: BTreeSet<RecipeStepDispositionClass>,
    pub run_dispositions: BTreeSet<RecipeRunDispositionClass>,
    pub audit_event_classes: BTreeSet<RecipeAuditEventClass>,
    pub attribution_surfaces: BTreeSet<RecipeAttributionSurfaceClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaFinding {
    pub severity: RecipeAlphaFindingSeverity,
    pub check_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeAlphaFindingSeverity {
    Error,
    Warning,
}

/// Redaction-safe support-export projection of one recipe-alpha page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAlphaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub page_id: String,
    pub definition_summaries: Vec<RecipeDefinitionSummary>,
    pub run_summaries: Vec<RecipeRunSummary>,
    pub audit_summaries: Vec<RecipeAuditEventSummary>,
    pub attribution_summaries: Vec<RecipeAttributionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeDefinitionSummary {
    pub definition_id: String,
    pub recipe_id: String,
    pub recipe_revision_ref: String,
    pub display_label: String,
    pub trust_gate: RecipeTrustGateClass,
    pub declared_write_classes: Vec<RecipeWriteClass>,
    pub declared_approval_classes: Vec<RecipeApprovalClass>,
    pub lineage_summary: String,
    pub step_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeRunSummary {
    pub run_id: String,
    pub definition_ref: String,
    pub display_label: String,
    pub trust_gate_observed: RecipeTrustGateClass,
    pub run_disposition: RecipeRunDispositionClass,
    pub denial_reason: Option<RecipeDenialReasonClass>,
    pub denial_reason_label: Option<String>,
    pub downgrade_target_label: Option<String>,
    pub approval_ticket_ref: Option<String>,
    pub preview_ticket_ref: Option<String>,
    pub promoted_run_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAuditEventSummary {
    pub audit_event_id: String,
    pub display_label: String,
    pub event_class: RecipeAuditEventClass,
    pub denial_reason_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeAttributionSummary {
    pub attribution_id: String,
    pub display_label: String,
    pub attribution_surface: RecipeAttributionSurfaceClass,
    pub definition_ref: String,
    pub run_ref: Option<String>,
    pub rationale_summary: String,
}

struct Validator<'a> {
    page: &'a RecipeAlphaPage,
    definition_ids: BTreeSet<&'a str>,
    step_ids: BTreeSet<String>,
    run_ids: BTreeSet<&'a str>,
    audit_event_ids: BTreeSet<&'a str>,
    attribution_ids: BTreeSet<&'a str>,
    coverage: RecipeAlphaCoverage,
    findings: Vec<RecipeAlphaFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a RecipeAlphaPage) -> Self {
        Self {
            page,
            definition_ids: BTreeSet::new(),
            step_ids: BTreeSet::new(),
            run_ids: BTreeSet::new(),
            audit_event_ids: BTreeSet::new(),
            attribution_ids: BTreeSet::new(),
            coverage: RecipeAlphaCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_definitions();
        self.validate_runs();
        self.validate_audit_events();
        self.validate_attributions();
        self.validate_required_coverage();
    }

    fn finish(self) -> RecipeAlphaValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != RecipeAlphaFindingSeverity::Error);
        RecipeAlphaValidationReport {
            record_kind: RECIPE_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == RECIPE_ALPHA_PAGE_RECORD_KIND,
            "recipe_alpha.page_record_kind",
            "page.record_kind must be recipe_alpha_page_record",
        );
        self.expect(
            page.schema_version == RECIPE_ALPHA_SCHEMA_VERSION,
            "recipe_alpha.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            page.shared_contract_ref == RECIPE_ALPHA_SHARED_CONTRACT_REF,
            "recipe_alpha.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "recipe_alpha.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.support_export_summary.trim().is_empty(),
            "recipe_alpha.page_support_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.required_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "recipe_alpha.contract_ref_missing",
                "every required upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.definitions.is_empty(),
            "recipe_alpha.definitions_missing",
            "page must contain at least one recipe definition",
        );
        self.expect(
            !page.runs.is_empty(),
            "recipe_alpha.runs_missing",
            "page must contain at least one recipe run",
        );
        self.expect(
            !page.audit_events.is_empty(),
            "recipe_alpha.audit_events_missing",
            "page must contain at least one audit event",
        );
        self.expect(
            !page.attributions.is_empty(),
            "recipe_alpha.attributions_missing",
            "page must contain at least one attribution row",
        );
    }

    fn validate_definitions(&mut self) {
        for definition in &self.page.definitions {
            self.expect(
                definition.record_kind == RECIPE_ALPHA_DEFINITION_RECORD_KIND,
                "recipe_alpha.definition_record_kind",
                "definition.record_kind is wrong",
            );
            self.expect(
                definition.schema_version == RECIPE_ALPHA_SCHEMA_VERSION,
                "recipe_alpha.definition_schema_version",
                "definition.schema_version is wrong",
            );
            self.expect(
                definition.shared_contract_ref == RECIPE_ALPHA_SHARED_CONTRACT_REF,
                "recipe_alpha.definition_shared_contract_ref",
                "definition.shared_contract_ref must match the shared contract id",
            );
            let unique = self.definition_ids.insert(&definition.definition_id);
            self.expect(
                unique,
                "recipe_alpha.definition_duplicate",
                "definition.definition_id values must be unique within a page",
            );
            self.expect(
                !definition.display_label.trim().is_empty(),
                "recipe_alpha.definition_display_label_missing",
                "definition.display_label must be non-empty",
            );
            self.expect(
                !definition.recipe_id.trim().is_empty()
                    && !definition.recipe_revision_ref.trim().is_empty()
                    && !definition.author_identity_ref.trim().is_empty()
                    && !definition.lineage_summary.trim().is_empty()
                    && !definition.recorded_at.trim().is_empty(),
                "recipe_alpha.definition_required_fields_missing",
                "definition must name recipe_id, recipe_revision_ref, author_identity_ref, \
                 lineage_summary, and recorded_at",
            );
            self.expect(
                !definition.steps.is_empty(),
                "recipe_alpha.definition_steps_missing",
                "definition must contain at least one step",
            );
            self.expect(
                !definition.declared_write_classes.is_empty(),
                "recipe_alpha.definition_declared_write_classes_missing",
                "definition.declared_write_classes must be non-empty",
            );
            self.expect(
                !definition.declared_approval_classes.is_empty(),
                "recipe_alpha.definition_declared_approval_classes_missing",
                "definition.declared_approval_classes must be non-empty",
            );
            self.expect(
                !definition.trust_gate.is_denied(),
                "recipe_alpha.definition_trust_gate_denied",
                "definition.trust_gate must not be managed_only_denied; admitted definitions \
                 never claim the denial lane",
            );
            self.expect(
                !definition.raw_branch_mutation_present,
                "recipe_alpha.definition_raw_branch_mutation_present",
                "definition.raw_branch_mutation_present must be false; raw branch mutation \
                 never crosses the boundary",
            );
            self.expect(
                !definition.raw_worktree_mutation_present,
                "recipe_alpha.definition_raw_worktree_mutation_present",
                "definition.raw_worktree_mutation_present must be false; raw worktree \
                 mutation never crosses the boundary",
            );
            self.expect(
                !definition.raw_provider_payload_present,
                "recipe_alpha.definition_raw_provider_payload_present",
                "definition.raw_provider_payload_present must be false",
            );
            self.expect(
                !definition.raw_credential_present,
                "recipe_alpha.definition_raw_credential_present",
                "definition.raw_credential_present must be false",
            );
            self.expect(
                !definition.silent_authority_widening_taken,
                "recipe_alpha.definition_silent_authority_widening",
                "definition.silent_authority_widening_taken must be false",
            );
            self.expect(
                !definition.remote_attach_degraded_state_masked,
                "recipe_alpha.definition_remote_attach_degraded_masked",
                "definition.remote_attach_degraded_state_masked must be false; degraded \
                 remote-attach state is never masked from a recipe",
            );
            self.expect(
                definition.support_attribution_minted,
                "recipe_alpha.definition_support_attribution_missing",
                "definition.support_attribution_minted must be true",
            );
            self.expect(
                definition.activity_attribution_minted,
                "recipe_alpha.definition_activity_attribution_missing",
                "definition.activity_attribution_minted must be true",
            );

            self.coverage.trust_gates.insert(definition.trust_gate);

            let mut step_ids_in_def: BTreeSet<&str> = BTreeSet::new();
            let mut declared_write_set: BTreeSet<RecipeWriteClass> =
                definition.declared_write_classes.iter().copied().collect();
            let mut declared_approval_set: BTreeSet<RecipeApprovalClass> = definition
                .declared_approval_classes
                .iter()
                .copied()
                .collect();
            let mut accumulated_writes: BTreeSet<RecipeWriteClass> = BTreeSet::new();
            let mut accumulated_approvals: BTreeSet<RecipeApprovalClass> = BTreeSet::new();

            for step in &definition.steps {
                self.expect(
                    !step.step_id.trim().is_empty(),
                    "recipe_alpha.step_id_missing",
                    "step.step_id must be non-empty",
                );
                let unique_in_def = step_ids_in_def.insert(step.step_id.as_str());
                self.expect(
                    unique_in_def,
                    "recipe_alpha.step_id_duplicate_in_definition",
                    "step.step_id values must be unique within a definition",
                );
                let unique_global = self.step_ids.insert(step.step_id.clone());
                self.expect(
                    unique_global,
                    "recipe_alpha.step_id_duplicate_across_definitions",
                    "step.step_id values must be unique across all definitions in the page",
                );
                self.expect(
                    !step.command_id.trim().is_empty()
                        && !step.command_revision_ref.trim().is_empty()
                        && !step.display_label.trim().is_empty()
                        && !step.shareability_record_ref.trim().is_empty(),
                    "recipe_alpha.step_required_fields_missing",
                    "step must name command_id, command_revision_ref, display_label, and \
                     shareability_record_ref",
                );
                self.expect(
                    !step.write_classes.is_empty(),
                    "recipe_alpha.step_write_classes_missing",
                    "step.write_classes must be non-empty",
                );
                self.expect(
                    !step.mode_requirement.is_denied(),
                    "recipe_alpha.step_mode_requirement_denied",
                    "step.mode_requirement must not be terminal_mode_denied; recipes never \
                     execute raw terminal text without an explicit cli_verb step",
                );
                self.expect(
                    !matches!(
                        step.step_command_lineage,
                        StepCommandLineageClass::UnmappedCommandDenied
                    ),
                    "recipe_alpha.step_lineage_unmapped",
                    "step.step_command_lineage must not be unmapped_command_denied; unmapped \
                     commands are denied on the command graph",
                );
                self.expect(
                    !step.approval_class.is_denied(),
                    "recipe_alpha.step_approval_class_denied",
                    "step.approval_class must not be approval_denied_unsafe on an admitted \
                     definition",
                );
                for write_class in &step.write_classes {
                    self.expect(
                        !write_class.is_denied(),
                        "recipe_alpha.step_write_class_denied",
                        "step.write_classes must not include credential_mutation_denied; \
                         credential mutation is the closed denial lane",
                    );
                    accumulated_writes.insert(*write_class);
                    self.coverage.write_classes.insert(*write_class);
                }
                accumulated_approvals.insert(step.approval_class);
                self.coverage.approval_classes.insert(step.approval_class);
                self.coverage
                    .preview_requirements
                    .insert(step.preview_requirement);
                self.coverage
                    .step_lineages
                    .insert(step.step_command_lineage);

                if step.step_command_lineage.requires_ai_tool_handle() {
                    self.expect(
                        step.ai_tool_handle_ref
                            .as_deref()
                            .is_some_and(|value| !value.trim().is_empty()),
                        "recipe_alpha.step_ai_tool_handle_missing",
                        "ai_tool_handle steps must cite an ai_tool_handle_ref",
                    );
                }
                if step.step_command_lineage.requires_provider_action_ref() {
                    self.expect(
                        step.provider_action_ref
                            .as_deref()
                            .is_some_and(|value| !value.trim().is_empty()),
                        "recipe_alpha.step_provider_action_ref_missing",
                        "provider_action steps must cite a provider_action_ref",
                    );
                }

                let has_mutation = step.write_classes.iter().any(|w| w.is_mutating());
                if has_mutation {
                    self.expect(
                        step.preview_requirement.requires_preview(),
                        "recipe_alpha.step_mutation_missing_preview",
                        "mutating steps must carry a preview_required_* preview_requirement; \
                         recipes preserve preview behavior for mutating steps",
                    );
                    self.expect(
                        !matches!(step.approval_class, RecipeApprovalClass::NoApprovalRequired),
                        "recipe_alpha.step_mutation_missing_approval",
                        "mutating steps must declare an approval class other than \
                         no_approval_required; recipes preserve approval behavior for \
                         mutating steps",
                    );
                }
                let has_provider = step.write_classes.iter().any(|w| w.is_provider_facing());
                if has_provider {
                    self.expect(
                        matches!(
                            step.preview_requirement,
                            RecipePreviewRequirementClass::PreviewRequiredProviderMutation
                                | RecipePreviewRequirementClass::PreviewRequiredBeforeApply
                        ),
                        "recipe_alpha.step_provider_mutation_missing_preview",
                        "provider-facing steps must carry preview_required_provider_mutation or \
                         preview_required_before_apply",
                    );
                    self.expect(
                        matches!(
                            step.approval_class,
                            RecipeApprovalClass::RecipeApprovalRequired
                                | RecipeApprovalClass::AdminSignedApprovalRequired
                                | RecipeApprovalClass::SingleStepApprovalRequired
                        ),
                        "recipe_alpha.step_provider_mutation_missing_approval",
                        "provider-facing steps must require single-step, recipe, or \
                         admin-signed approval; no provider mutation without review",
                    );
                }
                if step.write_classes.iter().any(|w| w.is_branch_or_worktree()) {
                    self.expect(
                        matches!(
                            step.preview_requirement,
                            RecipePreviewRequirementClass::PreviewRequiredBranchMutation
                                | RecipePreviewRequirementClass::PreviewRequiredWorktreeMutation
                                | RecipePreviewRequirementClass::PreviewRequiredBeforeApply
                        ),
                        "recipe_alpha.step_branch_or_worktree_missing_preview",
                        "branch/worktree mutations must carry a preview_required_branch_mutation, \
                         preview_required_worktree_mutation, or preview_required_before_apply \
                         preview_requirement; no hidden branch or worktree writes",
                    );
                }
            }

            // declared_write_classes must equal the union of step write_classes.
            for declared in &declared_write_set {
                if !accumulated_writes.contains(declared) {
                    self.findings.push(RecipeAlphaFinding {
                        severity: RecipeAlphaFindingSeverity::Error,
                        check_id: "recipe_alpha.definition_declared_write_overdeclared".to_string(),
                        message: format!(
                            "definition.declared_write_classes lists {} but no step declares it",
                            declared.as_str()
                        ),
                    });
                }
            }
            for observed in &accumulated_writes {
                if !declared_write_set.remove(observed)
                    && !definition
                        .declared_write_classes
                        .iter()
                        .any(|c| c == observed)
                {
                    self.findings.push(RecipeAlphaFinding {
                        severity: RecipeAlphaFindingSeverity::Error,
                        check_id: "recipe_alpha.definition_declared_write_underdeclared"
                            .to_string(),
                        message: format!(
                            "definition.declared_write_classes is missing {}; declared write \
                             scope must equal the union of step write_classes",
                            observed.as_str()
                        ),
                    });
                }
            }
            for declared in &declared_approval_set {
                if !accumulated_approvals.contains(declared) {
                    self.findings.push(RecipeAlphaFinding {
                        severity: RecipeAlphaFindingSeverity::Error,
                        check_id: "recipe_alpha.definition_declared_approval_overdeclared"
                            .to_string(),
                        message: format!(
                            "definition.declared_approval_classes lists {} but no step declares \
                             it",
                            declared.as_str()
                        ),
                    });
                }
            }
            for observed in &accumulated_approvals {
                if !declared_approval_set.remove(observed)
                    && !definition
                        .declared_approval_classes
                        .iter()
                        .any(|c| c == observed)
                {
                    self.findings.push(RecipeAlphaFinding {
                        severity: RecipeAlphaFindingSeverity::Error,
                        check_id: "recipe_alpha.definition_declared_approval_underdeclared"
                            .to_string(),
                        message: format!(
                            "definition.declared_approval_classes is missing {}; declared \
                             approval scope must equal the union of step approval_classes",
                            observed.as_str()
                        ),
                    });
                }
            }
        }
    }

    fn validate_runs(&mut self) {
        for run in &self.page.runs {
            self.expect(
                run.record_kind == RECIPE_ALPHA_RUN_RECORD_KIND,
                "recipe_alpha.run_record_kind",
                "run.record_kind is wrong",
            );
            self.expect(
                run.schema_version == RECIPE_ALPHA_SCHEMA_VERSION,
                "recipe_alpha.run_schema_version",
                "run.schema_version is wrong",
            );
            self.expect(
                run.shared_contract_ref == RECIPE_ALPHA_SHARED_CONTRACT_REF,
                "recipe_alpha.run_shared_contract_ref",
                "run.shared_contract_ref must match the shared contract id",
            );
            let unique = self.run_ids.insert(&run.run_id);
            self.expect(
                unique,
                "recipe_alpha.run_duplicate",
                "run.run_id values must be unique within a page",
            );
            self.expect(
                self.definition_ids.contains(run.definition_ref.as_str()),
                "recipe_alpha.run_definition_ref_unknown",
                "run.definition_ref must reference a definition in the page",
            );
            self.expect(
                !run.display_label.trim().is_empty(),
                "recipe_alpha.run_display_label_missing",
                "run.display_label must be non-empty",
            );
            self.expect(
                !run.actor_identity_ref.trim().is_empty() && !run.started_at.trim().is_empty(),
                "recipe_alpha.run_required_fields_missing",
                "run must name actor_identity_ref and started_at",
            );
            self.expect(
                !run.silent_authority_widening_taken,
                "recipe_alpha.run_silent_authority_widening",
                "run.silent_authority_widening_taken must be false",
            );
            self.expect(
                !run.remote_attach_degraded_state_masked,
                "recipe_alpha.run_remote_attach_degraded_masked",
                "run.remote_attach_degraded_state_masked must be false; degraded \
                 remote-attach state is never masked from a recipe run",
            );
            self.expect(
                run.support_attribution_minted,
                "recipe_alpha.run_support_attribution_missing",
                "run.support_attribution_minted must be true",
            );
            self.expect(
                run.activity_attribution_minted,
                "recipe_alpha.run_activity_attribution_missing",
                "run.activity_attribution_minted must be true",
            );
            self.expect(
                !run.trust_gate_observed.is_denied(),
                "recipe_alpha.run_trust_gate_denied",
                "run.trust_gate_observed must not be managed_only_denied",
            );
            self.expect(
                !run.step_dispositions.is_empty(),
                "recipe_alpha.run_step_dispositions_missing",
                "run.step_dispositions must be non-empty",
            );
            self.expect(
                !run.audit_event_refs.is_empty(),
                "recipe_alpha.run_audit_event_refs_missing",
                "run.audit_event_refs must be non-empty",
            );
            for step_disposition in &run.step_dispositions {
                self.expect(
                    self.step_ids.contains(&step_disposition.step_ref),
                    "recipe_alpha.run_step_ref_unknown",
                    "run.step_dispositions[].step_ref must reference a step on a definition",
                );
                self.coverage
                    .step_dispositions
                    .insert(step_disposition.step_disposition);
                if matches!(
                    step_disposition.step_disposition,
                    RecipeStepDispositionClass::DeniedUnsafeStep
                ) {
                    self.expect(
                        step_disposition.denial_reason.is_some(),
                        "recipe_alpha.run_step_denied_missing_reason",
                        "denied step dispositions must cite a denial_reason",
                    );
                }
            }

            let non_empty =
                |opt: &Option<String>| opt.as_deref().is_some_and(|v| !v.trim().is_empty());

            if run.run_disposition.requires_approval_ticket() {
                self.expect(
                    non_empty(&run.approval_ticket_ref),
                    "recipe_alpha.run_approval_ticket_missing",
                    "proceed_after_recipe_approval runs must cite an approval_ticket_ref",
                );
            }
            if run.run_disposition.requires_preview_ticket() {
                self.expect(
                    non_empty(&run.preview_ticket_ref),
                    "recipe_alpha.run_preview_ticket_missing",
                    "preview_required_before_apply runs must cite a preview_ticket_ref",
                );
            }
            if run.run_disposition.requires_promoted_run_ref() {
                self.expect(
                    non_empty(&run.promoted_run_ref),
                    "recipe_alpha.run_promoted_run_ref_missing",
                    "promoted_to_full_recipe_run runs must cite a promoted_run_ref",
                );
            }
            if run.run_disposition.requires_downgrade_target_label() {
                self.expect(
                    non_empty(&run.downgrade_target_label),
                    "recipe_alpha.run_downgrade_target_label_missing",
                    "downgraded_to_observer_no_mutation runs must cite a \
                     downgrade_target_label",
                );
            }
            if run.run_disposition.requires_denial_reason() {
                self.expect(
                    run.denial_reason.is_some() && non_empty(&run.denial_reason_label),
                    "recipe_alpha.run_denial_reason_missing",
                    "denied_unsafe_recipe runs must cite a denial_reason and \
                     denial_reason_label",
                );
            }

            self.coverage.run_dispositions.insert(run.run_disposition);
            self.coverage.trust_gates.insert(run.trust_gate_observed);
        }
    }

    fn validate_audit_events(&mut self) {
        for event in &self.page.audit_events {
            self.expect(
                event.record_kind == RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND,
                "recipe_alpha.audit_event_record_kind",
                "audit_event.record_kind is wrong",
            );
            self.expect(
                event.schema_version == RECIPE_ALPHA_SCHEMA_VERSION,
                "recipe_alpha.audit_event_schema_version",
                "audit_event.schema_version is wrong",
            );
            self.expect(
                event.shared_contract_ref == RECIPE_ALPHA_SHARED_CONTRACT_REF,
                "recipe_alpha.audit_event_shared_contract_ref",
                "audit_event.shared_contract_ref must match the shared contract id",
            );
            let unique = self.audit_event_ids.insert(&event.audit_event_id);
            self.expect(
                unique,
                "recipe_alpha.audit_event_duplicate",
                "audit_event.audit_event_id values must be unique within a page",
            );
            self.expect(
                !event.display_label.trim().is_empty() && !event.minted_at.trim().is_empty(),
                "recipe_alpha.audit_event_required_fields_missing",
                "audit_event must name display_label and minted_at",
            );

            let non_empty =
                |opt: &Option<String>| opt.as_deref().is_some_and(|v| !v.trim().is_empty());

            if event.event_class.requires_denial_reason() {
                self.expect(
                    non_empty(&event.denial_reason_label),
                    "recipe_alpha.audit_event_denial_reason_missing",
                    "denial audit events must cite a denial_reason_label",
                );
            }
            if event.event_class.requires_run_ref() {
                self.expect(
                    non_empty(&event.run_ref),
                    "recipe_alpha.audit_event_run_ref_missing",
                    "run-bound audit events must cite a run_ref",
                );
                if let Some(run_ref) = event.run_ref.as_deref() {
                    self.expect(
                        self.run_ids.contains(run_ref),
                        "recipe_alpha.audit_event_run_ref_unknown",
                        "audit_event.run_ref must reference a run in the page",
                    );
                }
            }
            if event.event_class.requires_attribution_ref() {
                self.expect(
                    non_empty(&event.attribution_ref),
                    "recipe_alpha.audit_event_attribution_ref_missing",
                    "attribution-minted audit events must cite an attribution_ref",
                );
            }
            if let Some(def_ref) = event.definition_ref.as_deref() {
                self.expect(
                    self.definition_ids.contains(def_ref),
                    "recipe_alpha.audit_event_definition_ref_unknown",
                    "audit_event.definition_ref must reference a definition in the page",
                );
            }

            self.coverage.audit_event_classes.insert(event.event_class);
        }
    }

    fn validate_attributions(&mut self) {
        for attribution in &self.page.attributions {
            self.expect(
                attribution.record_kind == RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND,
                "recipe_alpha.attribution_record_kind",
                "attribution.record_kind is wrong",
            );
            self.expect(
                attribution.schema_version == RECIPE_ALPHA_SCHEMA_VERSION,
                "recipe_alpha.attribution_schema_version",
                "attribution.schema_version is wrong",
            );
            self.expect(
                attribution.shared_contract_ref == RECIPE_ALPHA_SHARED_CONTRACT_REF,
                "recipe_alpha.attribution_shared_contract_ref",
                "attribution.shared_contract_ref must match the shared contract id",
            );
            let unique = self.attribution_ids.insert(&attribution.attribution_id);
            self.expect(
                unique,
                "recipe_alpha.attribution_duplicate",
                "attribution.attribution_id values must be unique within a page",
            );
            self.expect(
                !attribution.display_label.trim().is_empty()
                    && !attribution.rationale_summary.trim().is_empty()
                    && !attribution.minted_at.trim().is_empty(),
                "recipe_alpha.attribution_required_fields_missing",
                "attribution must name display_label, rationale_summary, and minted_at",
            );
            self.expect(
                self.definition_ids
                    .contains(attribution.definition_ref.as_str()),
                "recipe_alpha.attribution_definition_ref_unknown",
                "attribution.definition_ref must reference a definition in the page",
            );
            if let Some(run_ref) = attribution.run_ref.as_deref() {
                self.expect(
                    self.run_ids.contains(run_ref),
                    "recipe_alpha.attribution_run_ref_unknown",
                    "attribution.run_ref must reference a run in the page",
                );
            }

            self.coverage
                .attribution_surfaces
                .insert(attribution.attribution_surface);
        }
    }

    fn validate_required_coverage(&mut self) {
        for surface in [
            RecipeAttributionSurfaceClass::SupportExport,
            RecipeAttributionSurfaceClass::ActivityHistory,
        ] {
            self.expect(
                self.coverage.attribution_surfaces.contains(&surface),
                "recipe_alpha.coverage_attribution_surface_missing",
                "page must cover support_export and activity_history attribution surfaces",
            );
        }
        for disposition in [
            RecipeRunDispositionClass::ProceedLocalEditorOnly,
            RecipeRunDispositionClass::PreviewRequiredBeforeApply,
            RecipeRunDispositionClass::DeniedUnsafeRecipe,
        ] {
            self.expect(
                self.coverage.run_dispositions.contains(&disposition),
                "recipe_alpha.coverage_run_disposition_missing",
                "page must cover proceed_local_editor_only, preview_required_before_apply, and \
                 denied_unsafe_recipe run dispositions",
            );
        }
        for class in [
            RecipeAuditEventClass::RecipeRunStarted,
            RecipeAuditEventClass::RecipeRunCompleted,
            RecipeAuditEventClass::AttributionMinted,
        ] {
            self.expect(
                self.coverage.audit_event_classes.contains(&class),
                "recipe_alpha.coverage_audit_event_missing",
                "page must cover recipe_run_started, recipe_run_completed, and \
                 attribution_minted audit events",
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(RecipeAlphaFinding {
                severity: RecipeAlphaFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_step(
        id: &str,
        lineage: StepCommandLineageClass,
        command_id: &str,
        writes: Vec<RecipeWriteClass>,
        approval: RecipeApprovalClass,
        preview: RecipePreviewRequirementClass,
    ) -> RecipeStep {
        RecipeStep {
            step_id: id.to_string(),
            step_command_lineage: lineage,
            command_id: command_id.to_string(),
            command_revision_ref: format!("command-rev:{command_id}:v1"),
            display_label: format!("Step {id}"),
            mode_requirement: StepModeRequirementClass::EditorNormalModeRequired,
            write_classes: writes,
            approval_class: approval,
            preview_requirement: preview,
            shareability_record_ref: format!("shareability:{command_id}:v1"),
            ai_tool_handle_ref: lineage
                .requires_ai_tool_handle()
                .then(|| format!("ai_tool_handle:{id}")),
            provider_action_ref: lineage
                .requires_provider_action_ref()
                .then(|| format!("provider_action:{id}")),
            rationale_summary: None,
        }
    }

    fn baseline_page() -> RecipeAlphaPage {
        let normalize_step = make_step(
            "recipe.def.normalize.step.sort",
            StepCommandLineageClass::CoreCommand,
            "command:editor.imports.sort_alphabetical",
            vec![RecipeWriteClass::EditorBufferMutation],
            RecipeApprovalClass::SingleStepApprovalRequired,
            RecipePreviewRequirementClass::PreviewRequiredBeforeApply,
        );
        let normalize = RecipeDefinition {
            record_kind: RECIPE_ALPHA_DEFINITION_RECORD_KIND.to_string(),
            schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
            definition_id: "recipe.def.normalize".to_string(),
            recipe_id: "recipe:editor.normalize_imports".to_string(),
            recipe_revision_ref: "recipe-rev:editor.normalize_imports:2026-05-16".to_string(),
            display_label: "Normalize imports".to_string(),
            author_identity_ref: "identity:local_user:01".to_string(),
            trust_gate: RecipeTrustGateClass::RestrictedWorkspaceAdmissible,
            steps: vec![normalize_step],
            declared_write_classes: vec![RecipeWriteClass::EditorBufferMutation],
            declared_approval_classes: vec![RecipeApprovalClass::SingleStepApprovalRequired],
            lineage_summary: "Single editor buffer mutation; preview required.".to_string(),
            raw_branch_mutation_present: false,
            raw_worktree_mutation_present: false,
            raw_provider_payload_present: false,
            raw_credential_present: false,
            silent_authority_widening_taken: false,
            remote_attach_degraded_state_masked: false,
            support_attribution_minted: true,
            activity_attribution_minted: true,
            recorded_at: "2026-05-16T18:00:00Z".to_string(),
        };

        let pr_step = make_step(
            "recipe.def.publish_pr.step.draft",
            StepCommandLineageClass::ProviderAction,
            "command:provider.pull_request.open_draft",
            vec![RecipeWriteClass::ProviderMutation],
            RecipeApprovalClass::RecipeApprovalRequired,
            RecipePreviewRequirementClass::PreviewRequiredProviderMutation,
        );
        let publish_pr = RecipeDefinition {
            record_kind: RECIPE_ALPHA_DEFINITION_RECORD_KIND.to_string(),
            schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
            definition_id: "recipe.def.publish_pr".to_string(),
            recipe_id: "recipe:provider.publish_pr".to_string(),
            recipe_revision_ref: "recipe-rev:provider.publish_pr:2026-05-16".to_string(),
            display_label: "Open a draft PR on the code host".to_string(),
            author_identity_ref: "identity:local_user:01".to_string(),
            trust_gate: RecipeTrustGateClass::TrustedWorkspaceRequired,
            steps: vec![pr_step],
            declared_write_classes: vec![RecipeWriteClass::ProviderMutation],
            declared_approval_classes: vec![RecipeApprovalClass::RecipeApprovalRequired],
            lineage_summary: "One provider-mutation step; preview and recipe approval required."
                .to_string(),
            raw_branch_mutation_present: false,
            raw_worktree_mutation_present: false,
            raw_provider_payload_present: false,
            raw_credential_present: false,
            silent_authority_widening_taken: false,
            remote_attach_degraded_state_masked: false,
            support_attribution_minted: true,
            activity_attribution_minted: true,
            recorded_at: "2026-05-16T18:00:00Z".to_string(),
        };

        let runs = vec![
            RecipeRun {
                record_kind: RECIPE_ALPHA_RUN_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                run_id: "recipe.run.normalize.local".to_string(),
                definition_ref: "recipe.def.normalize".to_string(),
                display_label: "Normalize imports on local buffer".to_string(),
                actor_identity_ref: "identity:local_user:01".to_string(),
                trust_gate_observed: RecipeTrustGateClass::RestrictedWorkspaceAdmissible,
                run_disposition: RecipeRunDispositionClass::ProceedLocalEditorOnly,
                approval_ticket_ref: None,
                preview_ticket_ref: None,
                promoted_run_ref: None,
                denial_reason: None,
                denial_reason_label: None,
                downgrade_target_label: None,
                step_dispositions: vec![RecipeStepDisposition {
                    step_ref: "recipe.def.normalize.step.sort".to_string(),
                    step_disposition: RecipeStepDispositionClass::ProceedAfterPreview,
                    denial_reason: None,
                    rationale_summary: None,
                }],
                audit_event_refs: vec![
                    "recipe.audit.run.normalize.started".to_string(),
                    "recipe.audit.run.normalize.completed".to_string(),
                ],
                support_attribution_minted: true,
                activity_attribution_minted: true,
                silent_authority_widening_taken: false,
                remote_attach_degraded_state_masked: false,
                started_at: "2026-05-16T18:01:00Z".to_string(),
                resolved_at: Some("2026-05-16T18:01:05Z".to_string()),
            },
            RecipeRun {
                record_kind: RECIPE_ALPHA_RUN_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                run_id: "recipe.run.publish_pr.preview".to_string(),
                definition_ref: "recipe.def.publish_pr".to_string(),
                display_label: "Publish PR preview required".to_string(),
                actor_identity_ref: "identity:local_user:01".to_string(),
                trust_gate_observed: RecipeTrustGateClass::TrustedWorkspaceRequired,
                run_disposition: RecipeRunDispositionClass::PreviewRequiredBeforeApply,
                approval_ticket_ref: None,
                preview_ticket_ref: Some("preview.recipe.publish_pr.01".to_string()),
                promoted_run_ref: None,
                denial_reason: None,
                denial_reason_label: None,
                downgrade_target_label: None,
                step_dispositions: vec![RecipeStepDisposition {
                    step_ref: "recipe.def.publish_pr.step.draft".to_string(),
                    step_disposition: RecipeStepDispositionClass::PreviewRequiredBeforeApply,
                    denial_reason: None,
                    rationale_summary: None,
                }],
                audit_event_refs: vec![
                    "recipe.audit.run.publish_pr.started".to_string(),
                    "recipe.audit.run.publish_pr.completed".to_string(),
                ],
                support_attribution_minted: true,
                activity_attribution_minted: true,
                silent_authority_widening_taken: false,
                remote_attach_degraded_state_masked: false,
                started_at: "2026-05-16T18:02:00Z".to_string(),
                resolved_at: Some("2026-05-16T18:02:05Z".to_string()),
            },
            RecipeRun {
                record_kind: RECIPE_ALPHA_RUN_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                run_id: "recipe.run.publish_pr.denied".to_string(),
                definition_ref: "recipe.def.publish_pr".to_string(),
                display_label: "Publish PR denied".to_string(),
                actor_identity_ref: "identity:local_user:01".to_string(),
                trust_gate_observed: RecipeTrustGateClass::TrustedWorkspaceRequired,
                run_disposition: RecipeRunDispositionClass::DeniedUnsafeRecipe,
                approval_ticket_ref: None,
                preview_ticket_ref: None,
                promoted_run_ref: None,
                denial_reason: Some(RecipeDenialReasonClass::ApprovalMissingForProviderMutation),
                denial_reason_label: Some(
                    "Provider mutation requires recipe approval; ticket absent.".to_string(),
                ),
                downgrade_target_label: None,
                step_dispositions: vec![RecipeStepDisposition {
                    step_ref: "recipe.def.publish_pr.step.draft".to_string(),
                    step_disposition: RecipeStepDispositionClass::DeniedUnsafeStep,
                    denial_reason: Some(
                        RecipeDenialReasonClass::ApprovalMissingForProviderMutation,
                    ),
                    rationale_summary: Some("Approval ticket missing.".to_string()),
                }],
                audit_event_refs: vec!["recipe.audit.run.publish_pr.denied".to_string()],
                support_attribution_minted: true,
                activity_attribution_minted: true,
                silent_authority_widening_taken: false,
                remote_attach_degraded_state_masked: false,
                started_at: "2026-05-16T18:03:00Z".to_string(),
                resolved_at: Some("2026-05-16T18:03:01Z".to_string()),
            },
        ];

        let audit_events = vec![
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.run.normalize.started".to_string(),
                display_label: "Run started".to_string(),
                event_class: RecipeAuditEventClass::RecipeRunStarted,
                definition_ref: Some("recipe.def.normalize".to_string()),
                run_ref: Some("recipe.run.normalize.local".to_string()),
                attribution_ref: None,
                denial_reason_label: None,
                minted_at: "2026-05-16T18:01:00Z".to_string(),
            },
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.run.normalize.completed".to_string(),
                display_label: "Run completed".to_string(),
                event_class: RecipeAuditEventClass::RecipeRunCompleted,
                definition_ref: Some("recipe.def.normalize".to_string()),
                run_ref: Some("recipe.run.normalize.local".to_string()),
                attribution_ref: None,
                denial_reason_label: None,
                minted_at: "2026-05-16T18:01:05Z".to_string(),
            },
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.run.publish_pr.started".to_string(),
                display_label: "Publish PR run started".to_string(),
                event_class: RecipeAuditEventClass::RecipeRunStarted,
                definition_ref: Some("recipe.def.publish_pr".to_string()),
                run_ref: Some("recipe.run.publish_pr.preview".to_string()),
                attribution_ref: None,
                denial_reason_label: None,
                minted_at: "2026-05-16T18:02:00Z".to_string(),
            },
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.run.publish_pr.completed".to_string(),
                display_label: "Publish PR run completed (preview)".to_string(),
                event_class: RecipeAuditEventClass::RecipeRunCompleted,
                definition_ref: Some("recipe.def.publish_pr".to_string()),
                run_ref: Some("recipe.run.publish_pr.preview".to_string()),
                attribution_ref: None,
                denial_reason_label: None,
                minted_at: "2026-05-16T18:02:05Z".to_string(),
            },
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.run.publish_pr.denied".to_string(),
                display_label: "Publish PR run denied".to_string(),
                event_class: RecipeAuditEventClass::RecipeDenied,
                definition_ref: Some("recipe.def.publish_pr".to_string()),
                run_ref: Some("recipe.run.publish_pr.denied".to_string()),
                attribution_ref: None,
                denial_reason_label: Some(
                    "Approval ticket missing for provider mutation.".to_string(),
                ),
                minted_at: "2026-05-16T18:03:01Z".to_string(),
            },
            RecipeAuditEvent {
                record_kind: RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                audit_event_id: "recipe.audit.attribution.normalize.support".to_string(),
                display_label: "Support attribution minted".to_string(),
                event_class: RecipeAuditEventClass::AttributionMinted,
                definition_ref: Some("recipe.def.normalize".to_string()),
                run_ref: None,
                attribution_ref: Some("recipe.attribution.normalize.support".to_string()),
                denial_reason_label: None,
                minted_at: "2026-05-16T18:00:01Z".to_string(),
            },
        ];

        let attributions = vec![
            RecipeAttribution {
                record_kind: RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                attribution_id: "recipe.attribution.normalize.support".to_string(),
                display_label: "Support attribution for normalize recipe".to_string(),
                attribution_surface: RecipeAttributionSurfaceClass::SupportExport,
                definition_ref: "recipe.def.normalize".to_string(),
                run_ref: None,
                rationale_summary: "Normalize recipe attributed on support exports.".to_string(),
                minted_at: "2026-05-16T18:00:01Z".to_string(),
            },
            RecipeAttribution {
                record_kind: RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND.to_string(),
                schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
                shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
                attribution_id: "recipe.attribution.normalize.activity".to_string(),
                display_label: "Activity attribution for normalize recipe".to_string(),
                attribution_surface: RecipeAttributionSurfaceClass::ActivityHistory,
                definition_ref: "recipe.def.normalize".to_string(),
                run_ref: Some("recipe.run.normalize.local".to_string()),
                rationale_summary: "Normalize recipe attributed on activity history.".to_string(),
                minted_at: "2026-05-16T18:01:05Z".to_string(),
            },
        ];

        RecipeAlphaPage {
            fixture_metadata: None,
            record_kind: RECIPE_ALPHA_PAGE_RECORD_KIND.to_string(),
            schema_version: RECIPE_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: RECIPE_ALPHA_SHARED_CONTRACT_REF.to_string(),
            page_id: "recipe_alpha.page.unit_test".to_string(),
            contract_refs: RecipeAlphaContractRefs {
                command_descriptor_schema_ref: "schemas/commands/command_descriptor.schema.json"
                    .to_string(),
                recorded_macro_schema_ref: "schemas/commands/recorded_macro.schema.json"
                    .to_string(),
                approval_ticket_schema_ref: "schemas/runtime/approval_ticket.schema.json"
                    .to_string(),
                shareability_metadata_schema_ref:
                    "schemas/commands/shareability_metadata.schema.json".to_string(),
                provider_object_schema_ref: Some(
                    "schemas/providers/local_object.schema.json".to_string(),
                ),
            },
            definitions: vec![normalize, publish_pr],
            runs,
            audit_events,
            attributions,
            support_export_summary:
                "Recipe alpha unit-test page with two definitions and three runs across proceed, \
                 preview-required, and denied lanes."
                    .to_string(),
        }
    }

    #[test]
    fn baseline_page_validates() {
        let page = baseline_page();
        let report = page.validate();
        assert!(report.passed, "baseline must pass: {:#?}", report.findings);
    }

    #[test]
    fn mutating_step_without_preview_is_rejected() {
        let mut page = baseline_page();
        let definition = page
            .definitions
            .iter_mut()
            .find(|d| d.definition_id == "recipe.def.normalize")
            .expect("definition present");
        definition.steps[0].preview_requirement = RecipePreviewRequirementClass::NoPreviewRequired;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "recipe_alpha.step_mutation_missing_preview"));
    }

    #[test]
    fn provider_step_without_approval_is_rejected() {
        let mut page = baseline_page();
        let definition = page
            .definitions
            .iter_mut()
            .find(|d| d.definition_id == "recipe.def.publish_pr")
            .expect("definition present");
        definition.steps[0].approval_class = RecipeApprovalClass::NoApprovalRequired;
        definition.declared_approval_classes = vec![RecipeApprovalClass::NoApprovalRequired];
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "recipe_alpha.step_provider_mutation_missing_approval"));
    }

    #[test]
    fn denied_run_must_cite_reason() {
        let mut page = baseline_page();
        let run = page
            .runs
            .iter_mut()
            .find(|r| r.run_id == "recipe.run.publish_pr.denied")
            .expect("denied run present");
        run.denial_reason = None;
        run.denial_reason_label = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|f| f.check_id == "recipe_alpha.run_denial_reason_missing"));
    }

    #[test]
    fn support_export_omits_raw_fields() {
        let page = baseline_page();
        let projection = page.support_export_projection();
        let json = serde_json::to_string(&projection).expect("projection serializes");
        assert_eq!(projection.record_kind, "recipe_alpha_support_export");
        assert!(!json.contains("raw_branch_mutation"));
        assert!(!json.contains("raw_worktree_mutation"));
        assert!(!json.contains("raw_provider_payload"));
        assert!(!json.contains("raw_credential"));
        assert_eq!(
            projection.definition_summaries.len(),
            page.definitions.len()
        );
        assert_eq!(projection.run_summaries.len(), page.runs.len());
    }
}
