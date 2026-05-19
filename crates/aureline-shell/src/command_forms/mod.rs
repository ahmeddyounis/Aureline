//! Command-schema-driven parameter forms and invocation review sheets.
//!
//! This module is the single Rust-side projection every shell surface that
//! collects typed parameters and previews a high-impact apply reads from.
//! Palette parameter forms, CLI inspect surfaces, AI tool envelopes,
//! automation-recipe step editors, request / run / debug / template / repair
//! workspaces, and voice grammars all consume the same
//! [`ParameterFormStateRecord`] for typed fields and the same
//! [`InvocationReviewSheetRecord`] for preview-before-run, so defaults,
//! validation, redaction, source-layer labels, and review semantics cannot
//! drift between lanes.
//!
//! The records pin to the canonical command descriptor via
//! `schemas/commands/command_descriptor.schema.json` and project a closed
//! taxonomy for:
//!
//! - typed field state ([`FieldStateClass`]) and source-layer labels
//!   ([`SourceLayerClass`]) so users can tell whether a value came from
//!   descriptor default, user override, workspace value, org policy,
//!   runtime prompt, or a secret-handle reference;
//! - validation findings ([`ValidationFindingClass`]) with frozen severity
//!   ([`ValidationSeverity`]);
//! - restart / reload truth ([`RestartOrReloadClass`]) so users know whether
//!   an edit invalidates a running task, debug session, request, or shell
//!   surface;
//! - target scope, side-effect class, process / network / remote actions,
//!   rollback posture, blocked prerequisites with typed repair hooks, and
//!   preview / dry-run availability on the review sheet.
//!
//! Secret-bearing fields are always handle-first: literal display is masked
//! by default, value visibility rides [`ValueVisibilityClass`], and the
//! sheet's `secret_handling_summary` advertises whether any literal reveal
//! was armed. Run history and support exports inherit the sheet's
//! redaction class.
//!
//! The module is intentionally read-only: it does not execute commands or
//! mutate state. Callers (palette form host, CLI inspect, AI envelope,
//! request workspace, run/debug profiles, template generator, repair flow)
//! project these records to render fields and review sheets that are
//! bit-for-bit equivalent across lanes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod render;
pub mod seed;
pub mod validation;

/// Schema version exported with every parameter-form-state record.
pub const PARAMETER_FORM_STATE_SCHEMA_VERSION: u32 = 1;

/// Schema version exported with every invocation-review-sheet record.
pub const INVOCATION_REVIEW_SHEET_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`ParameterFormStateRecord`] payloads.
pub const PARAMETER_FORM_STATE_RECORD_KIND: &str = "parameter_form_state_record";

/// Stable record kind for [`InvocationReviewSheetRecord`] payloads.
pub const INVOCATION_REVIEW_SHEET_RECORD_KIND: &str = "invocation_review_sheet_record";

/// Stable record kind for [`CommandFormsCatalog`] payloads.
pub const COMMAND_FORMS_CATALOG_RECORD_KIND: &str = "command_forms_catalog_record";

/// Stable shared contract ref every projection quotes.
pub const COMMAND_FORMS_SHARED_CONTRACT_REF: &str = "shell:command_forms_beta:v1";

/// Stable catalog id quoted by the parity report and the beta contract doc.
pub const COMMAND_FORMS_CATALOG_ID: &str = "shell:command_forms_beta:catalog:v1";

/// Path to the parameter-form-state schema this module projects.
pub const PARAMETER_FORM_STATE_SCHEMA_REF: &str =
    "schemas/commands/parameter_form_state.schema.json";

/// Path to the invocation-review-sheet schema this module projects.
pub const INVOCATION_REVIEW_SHEET_SCHEMA_REF: &str =
    "schemas/commands/invocation_review_sheet.schema.json";

/// Path to the canonical command descriptor schema the records pin to.
pub const COMMAND_DESCRIPTOR_SCHEMA_REF: &str = "schemas/commands/command_descriptor.schema.json";

/// Path of the published companion beta contract doc.
pub const COMMAND_FORMS_PUBLISHED_DOC_REF: &str = "docs/ux/m3/command_parameter_form_beta.md";

/// Generation timestamp captured in every seeded record.
pub const COMMAND_FORMS_GENERATED_AT: &str = "2026-05-19T00:00:00Z";

/// Re-exported argument-kind vocabulary from the command descriptor schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgumentKind {
    StringFreeForm,
    StringEnum,
    BooleanFlag,
    IntegerBounded,
    DecimalBounded,
    OpaqueIdRef,
    PathRefOpaque,
    WorkspaceScopeRef,
    SelectionRef,
    RangeRef,
    ProviderRef,
    CapabilityRef,
    CredentialHandleRef,
    DocsAnchorRef,
    FileSetRef,
    GlobExpression,
    DurationSeconds,
    LocaleTag,
    SemverRange,
    MultiValueList,
}

impl ArgumentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StringFreeForm => "string_free_form",
            Self::StringEnum => "string_enum",
            Self::BooleanFlag => "boolean_flag",
            Self::IntegerBounded => "integer_bounded",
            Self::DecimalBounded => "decimal_bounded",
            Self::OpaqueIdRef => "opaque_id_ref",
            Self::PathRefOpaque => "path_ref_opaque",
            Self::WorkspaceScopeRef => "workspace_scope_ref",
            Self::SelectionRef => "selection_ref",
            Self::RangeRef => "range_ref",
            Self::ProviderRef => "provider_ref",
            Self::CapabilityRef => "capability_ref",
            Self::CredentialHandleRef => "credential_handle_ref",
            Self::DocsAnchorRef => "docs_anchor_ref",
            Self::FileSetRef => "file_set_ref",
            Self::GlobExpression => "glob_expression",
            Self::DurationSeconds => "duration_seconds",
            Self::LocaleTag => "locale_tag",
            Self::SemverRange => "semver_range",
            Self::MultiValueList => "multi_value_list",
        }
    }
}

/// Frozen client-scope vocabulary the form is rendered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    DesktopProduct,
    Cli,
    CompanionSurface,
    RemoteAgent,
    SdkOrApi,
    ManagedAdminSurface,
}

impl ClientScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopProduct => "desktop_product",
            Self::Cli => "cli",
            Self::CompanionSurface => "companion_surface",
            Self::RemoteAgent => "remote_agent",
            Self::SdkOrApi => "sdk_or_api",
            Self::ManagedAdminSurface => "managed_admin_surface",
        }
    }
}

/// Frozen form-surface class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormSurfaceClass {
    DesktopParameterForm,
    CliInspectSurface,
    AiToolEnvelope,
    AutomationRecipeStepEditor,
    RequestWorkspaceForm,
    RunDebugProfileForm,
    TemplateOrGeneratorForm,
    RepairOrDoctorForm,
    VoiceGrammarSurface,
    RemoteCompanionForm,
}

impl FormSurfaceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopParameterForm => "desktop_parameter_form",
            Self::CliInspectSurface => "cli_inspect_surface",
            Self::AiToolEnvelope => "ai_tool_envelope",
            Self::AutomationRecipeStepEditor => "automation_recipe_step_editor",
            Self::RequestWorkspaceForm => "request_workspace_form",
            Self::RunDebugProfileForm => "run_debug_profile_form",
            Self::TemplateOrGeneratorForm => "template_or_generator_form",
            Self::RepairOrDoctorForm => "repair_or_doctor_form",
            Self::VoiceGrammarSurface => "voice_grammar_surface",
            Self::RemoteCompanionForm => "remote_companion_form",
        }
    }
}

/// Frozen source-layer class for a resolved field value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLayerClass {
    DescriptorDefault,
    UserOverride,
    WorkspaceValue,
    OrgPolicyPinned,
    RuntimePrompt,
    SecretHandleReference,
    AiProposedRequiresReview,
    AutomationRecipeSupplied,
    ExtensionSupplied,
    KeybindingArgsSupplied,
    CliArgvSupplied,
    SelectionInferred,
    FocusedContextInferred,
}

impl SourceLayerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorDefault => "descriptor_default",
            Self::UserOverride => "user_override",
            Self::WorkspaceValue => "workspace_value",
            Self::OrgPolicyPinned => "org_policy_pinned",
            Self::RuntimePrompt => "runtime_prompt",
            Self::SecretHandleReference => "secret_handle_reference",
            Self::AiProposedRequiresReview => "ai_proposed_requires_review",
            Self::AutomationRecipeSupplied => "automation_recipe_supplied",
            Self::ExtensionSupplied => "extension_supplied",
            Self::KeybindingArgsSupplied => "keybinding_args_supplied",
            Self::CliArgvSupplied => "cli_argv_supplied",
            Self::SelectionInferred => "selection_inferred",
            Self::FocusedContextInferred => "focused_context_inferred",
        }
    }

    /// Controlled label displayed on every surface for this source layer.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::DescriptorDefault => "Default",
            Self::UserOverride => "User override",
            Self::WorkspaceValue => "Workspace value",
            Self::OrgPolicyPinned => "Org policy",
            Self::RuntimePrompt => "Runtime prompt",
            Self::SecretHandleReference => "Secret reference",
            Self::AiProposedRequiresReview => "AI proposed (review)",
            Self::AutomationRecipeSupplied => "Recipe supplied",
            Self::ExtensionSupplied => "Extension supplied",
            Self::KeybindingArgsSupplied => "Keybinding args",
            Self::CliArgvSupplied => "CLI args",
            Self::SelectionInferred => "From selection",
            Self::FocusedContextInferred => "From focus",
        }
    }
}

/// Frozen field-state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldStateClass {
    Editable,
    EditableWithWarning,
    PolicyPinnedReadOnly,
    SecretMaskedReadOnly,
    SecretHandleSwapOnly,
    RuntimePromptRequired,
    UnsupportedInClientScope,
    UnsupportedInRestrictedTrust,
    HiddenUntilDependencyResolved,
    DeprecatedUseReplacement,
}

impl FieldStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editable => "editable",
            Self::EditableWithWarning => "editable_with_warning",
            Self::PolicyPinnedReadOnly => "policy_pinned_read_only",
            Self::SecretMaskedReadOnly => "secret_masked_read_only",
            Self::SecretHandleSwapOnly => "secret_handle_swap_only",
            Self::RuntimePromptRequired => "runtime_prompt_required",
            Self::UnsupportedInClientScope => "unsupported_in_client_scope",
            Self::UnsupportedInRestrictedTrust => "unsupported_in_restricted_trust",
            Self::HiddenUntilDependencyResolved => "hidden_until_dependency_resolved",
            Self::DeprecatedUseReplacement => "deprecated_use_replacement",
        }
    }
}

/// Frozen redaction class re-exported from the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

impl RedactionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Frozen value-visibility class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueVisibilityClass {
    ValueVisible,
    ValueMasked,
    ValueHandleOnly,
    ValueOmittedForRedaction,
}

impl ValueVisibilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValueVisible => "value_visible",
            Self::ValueMasked => "value_masked",
            Self::ValueHandleOnly => "value_handle_only",
            Self::ValueOmittedForRedaction => "value_omitted_for_redaction",
        }
    }
}

/// Frozen restart-or-reload class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartOrReloadClass {
    NoRestartRequired,
    PreviewRecomputeRequired,
    RequestReissueRequired,
    TaskRestartRequired,
    DebuggerRestartRequired,
    ExtensionReloadRequired,
    ShellWindowReloadRequired,
    WorkspaceReloadRequired,
    HostProcessRestartRequired,
}

impl RestartOrReloadClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRestartRequired => "no_restart_required",
            Self::PreviewRecomputeRequired => "preview_recompute_required",
            Self::RequestReissueRequired => "request_reissue_required",
            Self::TaskRestartRequired => "task_restart_required",
            Self::DebuggerRestartRequired => "debugger_restart_required",
            Self::ExtensionReloadRequired => "extension_reload_required",
            Self::ShellWindowReloadRequired => "shell_window_reload_required",
            Self::WorkspaceReloadRequired => "workspace_reload_required",
            Self::HostProcessRestartRequired => "host_process_restart_required",
        }
    }
}

/// Frozen validation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Informational,
    Warning,
    Blocking,
}

impl ValidationSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }
}

/// Frozen validation-finding class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFindingClass {
    RequiredValueMissing,
    ValueBelowMinimum,
    ValueAboveMaximum,
    ValueNotInEnum,
    ValueFailedPattern,
    ValueFailedSemverRange,
    ValueFailedDurationBound,
    ValueFailedLocaleTag,
    ValueFailedGlobGrammar,
    ValueFailedProviderHandshake,
    ValueFailedWorkspaceScopeResolution,
    ValueFailedCredentialHandleResolution,
    ValueFailedCapabilityGrant,
    ValueFailedPolicyPin,
    ValueFailedFreshnessFloor,
    ValueFailedBasisSnapshot,
    ValueFailedRedactionAdmission,
    ValueFailedClientScopeAdmission,
    ValueFailedTrustStateAdmission,
}

impl ValidationFindingClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredValueMissing => "required_value_missing",
            Self::ValueBelowMinimum => "value_below_minimum",
            Self::ValueAboveMaximum => "value_above_maximum",
            Self::ValueNotInEnum => "value_not_in_enum",
            Self::ValueFailedPattern => "value_failed_pattern",
            Self::ValueFailedSemverRange => "value_failed_semver_range",
            Self::ValueFailedDurationBound => "value_failed_duration_bound",
            Self::ValueFailedLocaleTag => "value_failed_locale_tag",
            Self::ValueFailedGlobGrammar => "value_failed_glob_grammar",
            Self::ValueFailedProviderHandshake => "value_failed_provider_handshake",
            Self::ValueFailedWorkspaceScopeResolution => {
                "value_failed_workspace_scope_resolution"
            }
            Self::ValueFailedCredentialHandleResolution => {
                "value_failed_credential_handle_resolution"
            }
            Self::ValueFailedCapabilityGrant => "value_failed_capability_grant",
            Self::ValueFailedPolicyPin => "value_failed_policy_pin",
            Self::ValueFailedFreshnessFloor => "value_failed_freshness_floor",
            Self::ValueFailedBasisSnapshot => "value_failed_basis_snapshot",
            Self::ValueFailedRedactionAdmission => "value_failed_redaction_admission",
            Self::ValueFailedClientScopeAdmission => "value_failed_client_scope_admission",
            Self::ValueFailedTrustStateAdmission => "value_failed_trust_state_admission",
        }
    }
}

/// Frozen unsupported-field class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFieldClass {
    NotSupportedOnClientScope,
    NotSupportedInRestrictedTrust,
    NotSupportedInBrowserCompanion,
    NotSupportedInHeadlessInspect,
    NotSupportedInRemoteAgent,
    NotSupportedForFreshnessFloor,
    NotSupportedByProviderCapability,
}

impl UnsupportedFieldClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSupportedOnClientScope => "not_supported_on_client_scope",
            Self::NotSupportedInRestrictedTrust => "not_supported_in_restricted_trust",
            Self::NotSupportedInBrowserCompanion => "not_supported_in_browser_companion",
            Self::NotSupportedInHeadlessInspect => "not_supported_in_headless_inspect",
            Self::NotSupportedInRemoteAgent => "not_supported_in_remote_agent",
            Self::NotSupportedForFreshnessFloor => "not_supported_for_freshness_floor",
            Self::NotSupportedByProviderCapability => "not_supported_by_provider_capability",
        }
    }
}

/// Workspace trust state quoted by the policy context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Restricted,
}

impl TrustState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
        }
    }
}

/// Policy context snapshot quoted by the form and the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub policy_epoch: String,
    pub trust_state: TrustState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
}

/// One validation finding attached to a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub finding_class: ValidationFindingClass,
    pub severity: ValidationSeverity,
    pub narration_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_anchor_ref: Option<String>,
}

/// One field's state in a parameter form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldStateRecord {
    pub argument_name: String,
    pub argument_kind: ArgumentKind,
    pub is_required: bool,
    pub field_state_class: FieldStateClass,
    pub source_layer_class: SourceLayerClass,
    pub source_layer_label_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originating_layer_chain: Vec<SourceLayerClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_value_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value_ref: Option<String>,
    pub value_visibility: ValueVisibilityClass,
    pub redaction_class: RedactionClass,
    pub restart_or_reload_class: RestartOrReloadClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enum_value_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_inclusive: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_inclusive: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported_field_class: Option<UnsupportedFieldClass>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_for_client_scopes: Vec<ClientScope>,
    pub narration_label_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_findings: Vec<ValidationFinding>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub supports_runtime_reveal: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_pinned_when_trust_state_is: Vec<TrustState>,
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// Form-level rollup of all field validation findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormLevelValidationRollup {
    pub overall_severity: ValidationSeverity,
    pub blocking_finding_count: u32,
    pub warning_finding_count: u32,
    #[serde(default)]
    pub informational_finding_count: u32,
}

/// Canonical parameter-form-state projection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterFormStateRecord {
    pub record_kind: String,
    pub parameter_form_state_schema_version: u32,
    pub form_state_id: String,
    pub command_id: String,
    pub command_revision_ref: String,
    pub form_surface_class: FormSurfaceClass,
    pub client_scope: ClientScope,
    pub fields: Vec<FieldStateRecord>,
    pub validation_rollup: FormLevelValidationRollup,
    pub policy_context: PolicyContext,
    pub redaction_class: RedactionClass,
    #[serde(default, skip_serializing_if = "is_false")]
    pub supports_secret_handle_swap: bool,
    pub minted_at: String,
}

impl ParameterFormStateRecord {
    /// Returns the field for an argument name, if any.
    pub fn field(&self, argument_name: &str) -> Option<&FieldStateRecord> {
        self.fields.iter().find(|f| f.argument_name == argument_name)
    }

    /// Returns true when no blocking validation finding is present.
    pub fn is_invocable(&self) -> bool {
        !matches!(self.validation_rollup.overall_severity, ValidationSeverity::Blocking)
    }
}

/// Frozen capability-scope class re-exported from the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityScopeClass {
    InertMetadataOnly,
    ReversibleLocalRead,
    ReversibleLocalMutation,
    RecoverableDurableMutation,
    ExternallyVisibleMutation,
    IrreversibleHighBlastMutation,
    CredentialOrSecretBearing,
    ManagedWorkspaceControl,
    PolicyAuthoringOrWaiver,
}

impl CapabilityScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::ExternallyVisibleMutation => "externally_visible_mutation",
            Self::IrreversibleHighBlastMutation => "irreversible_high_blast_mutation",
            Self::CredentialOrSecretBearing => "credential_or_secret_bearing",
            Self::ManagedWorkspaceControl => "managed_workspace_control",
            Self::PolicyAuthoringOrWaiver => "policy_authoring_or_waiver",
        }
    }
}

/// Frozen preview-class re-exported from the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewClass {
    NoPreviewRequired,
    InlineSummaryPreview,
    StructuredDiffPreview,
    BatchScopePreview,
    DestructiveBulkMutationPreview,
    BroadWorkspaceScopePreview,
    IrreversiblePublishPreview,
    ExternallyMutatingPreview,
    CredentialOrSecretAccessPreview,
    PolicyAuthoringOrWaiverPreview,
    ManagedWorkspaceControlPreview,
    RemoteAttachPreview,
    InstallOrUpdatePreview,
    CollaborationInvitePreview,
    BrowserHandoffPreview,
    RichActiveContentPreview,
    BidiOrInvisibleFormattingRevealPreview,
    ConfusableIdentifierRevealPreview,
}

impl PreviewClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewRequired => "no_preview_required",
            Self::InlineSummaryPreview => "inline_summary_preview",
            Self::StructuredDiffPreview => "structured_diff_preview",
            Self::BatchScopePreview => "batch_scope_preview",
            Self::DestructiveBulkMutationPreview => "destructive_bulk_mutation_preview",
            Self::BroadWorkspaceScopePreview => "broad_workspace_scope_preview",
            Self::IrreversiblePublishPreview => "irreversible_publish_preview",
            Self::ExternallyMutatingPreview => "externally_mutating_preview",
            Self::CredentialOrSecretAccessPreview => "credential_or_secret_access_preview",
            Self::PolicyAuthoringOrWaiverPreview => "policy_authoring_or_waiver_preview",
            Self::ManagedWorkspaceControlPreview => "managed_workspace_control_preview",
            Self::RemoteAttachPreview => "remote_attach_preview",
            Self::InstallOrUpdatePreview => "install_or_update_preview",
            Self::CollaborationInvitePreview => "collaboration_invite_preview",
            Self::BrowserHandoffPreview => "browser_handoff_preview",
            Self::RichActiveContentPreview => "rich_active_content_preview",
            Self::BidiOrInvisibleFormattingRevealPreview => {
                "bidi_or_invisible_formatting_reveal_preview"
            }
            Self::ConfusableIdentifierRevealPreview => "confusable_identifier_reveal_preview",
        }
    }

    pub const fn requires_preview(self) -> bool {
        !matches!(self, Self::NoPreviewRequired)
    }
}

/// Frozen approval-posture class re-exported from the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalPostureClass {
    NoApprovalRequired,
    ExplicitConfirmationRequired,
    StepUpAuthenticationRequired,
    AdminPolicyApprovalRequired,
    SecondPartyReviewRequired,
    ManagedOnlyApprovalRequired,
}

impl ApprovalPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalRequired => "no_approval_required",
            Self::ExplicitConfirmationRequired => "explicit_confirmation_required",
            Self::StepUpAuthenticationRequired => "step_up_authentication_required",
            Self::AdminPolicyApprovalRequired => "admin_policy_approval_required",
            Self::SecondPartyReviewRequired => "second_party_review_required",
            Self::ManagedOnlyApprovalRequired => "managed_only_approval_required",
        }
    }
}

/// Frozen execution-intent class re-exported from the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionIntentClass {
    QueryOnlyNoMutation,
    ProposePreviewOnly,
    ApplyAfterPreview,
    ApplyWithApproval,
    ApplyDirectTrustedPath,
    RollbackOrRevert,
    SimulateOrDryRun,
    ScheduleForBackgroundExecution,
    CancelPendingInvocation,
}

impl ExecutionIntentClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryOnlyNoMutation => "query_only_no_mutation",
            Self::ProposePreviewOnly => "propose_preview_only",
            Self::ApplyAfterPreview => "apply_after_preview",
            Self::ApplyWithApproval => "apply_with_approval",
            Self::ApplyDirectTrustedPath => "apply_direct_trusted_path",
            Self::RollbackOrRevert => "rollback_or_revert",
            Self::SimulateOrDryRun => "simulate_or_dry_run",
            Self::ScheduleForBackgroundExecution => "schedule_for_background_execution",
            Self::CancelPendingInvocation => "cancel_pending_invocation",
        }
    }
}

/// Frozen review-surface class for the invocation review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSurfaceClass {
    DesktopReviewSheet,
    CliInspectReview,
    AiApplyReview,
    AutomationRecipeReview,
    RequestWorkspaceReview,
    RunDebugReview,
    TemplateOrGeneratorReview,
    RepairOrDoctorReview,
    RemoteCompanionReview,
}

impl ReviewSurfaceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReviewSheet => "desktop_review_sheet",
            Self::CliInspectReview => "cli_inspect_review",
            Self::AiApplyReview => "ai_apply_review",
            Self::AutomationRecipeReview => "automation_recipe_review",
            Self::RequestWorkspaceReview => "request_workspace_review",
            Self::RunDebugReview => "run_debug_review",
            Self::TemplateOrGeneratorReview => "template_or_generator_review",
            Self::RepairOrDoctorReview => "repair_or_doctor_review",
            Self::RemoteCompanionReview => "remote_companion_review",
        }
    }
}

/// Frozen scope-axis class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeAxisClass {
    WorkspaceRoot,
    SelectedFilesOrGlobs,
    SelectedBuffers,
    SelectedRunsOrSessions,
    SelectedRequestsOrEndpoints,
    SelectedCollections,
    SelectedBranchesOrWorktrees,
    SelectedRemotesOrOrigins,
    SelectedRecipeSteps,
    SelectedTemplateTargets,
    SelectedRepairTargets,
    SelectedSecretHandles,
    SelectedPolicyDocuments,
    SelectedManagedResources,
}

impl ScopeAxisClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace_root",
            Self::SelectedFilesOrGlobs => "selected_files_or_globs",
            Self::SelectedBuffers => "selected_buffers",
            Self::SelectedRunsOrSessions => "selected_runs_or_sessions",
            Self::SelectedRequestsOrEndpoints => "selected_requests_or_endpoints",
            Self::SelectedCollections => "selected_collections",
            Self::SelectedBranchesOrWorktrees => "selected_branches_or_worktrees",
            Self::SelectedRemotesOrOrigins => "selected_remotes_or_origins",
            Self::SelectedRecipeSteps => "selected_recipe_steps",
            Self::SelectedTemplateTargets => "selected_template_targets",
            Self::SelectedRepairTargets => "selected_repair_targets",
            Self::SelectedSecretHandles => "selected_secret_handles",
            Self::SelectedPolicyDocuments => "selected_policy_documents",
            Self::SelectedManagedResources => "selected_managed_resources",
        }
    }
}

/// Frozen side-effect class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    NoSideEffect,
    LocalBufferMutation,
    LocalFilesystemMutation,
    WorkspaceStateMutation,
    GitLocalMutation,
    GitRemoteMutation,
    PackageInstallMutation,
    PackageUninstallMutation,
    BuildArtifactEmission,
    TestRunnerExecution,
    ProcessSpawnLocal,
    ProcessSpawnRemote,
    NetworkRequestOutbound,
    NetworkListenInbound,
    CredentialOrSecretUse,
    CredentialOrSecretEmission,
    RemoteAttachMutation,
    CollaborationInviteMutation,
    PolicyAuthoringMutation,
    ManagedWorkspaceMutation,
    BrowserHandoffEmission,
    TelemetryEmissionHighVolume,
    ExtensionLifecycleMutation,
    WorkspaceSettingsMutation,
    PolicyPackInstallOrUpdate,
    SystemBrowserLaunch,
}

impl SideEffectClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::LocalBufferMutation => "local_buffer_mutation",
            Self::LocalFilesystemMutation => "local_filesystem_mutation",
            Self::WorkspaceStateMutation => "workspace_state_mutation",
            Self::GitLocalMutation => "git_local_mutation",
            Self::GitRemoteMutation => "git_remote_mutation",
            Self::PackageInstallMutation => "package_install_mutation",
            Self::PackageUninstallMutation => "package_uninstall_mutation",
            Self::BuildArtifactEmission => "build_artifact_emission",
            Self::TestRunnerExecution => "test_runner_execution",
            Self::ProcessSpawnLocal => "process_spawn_local",
            Self::ProcessSpawnRemote => "process_spawn_remote",
            Self::NetworkRequestOutbound => "network_request_outbound",
            Self::NetworkListenInbound => "network_listen_inbound",
            Self::CredentialOrSecretUse => "credential_or_secret_use",
            Self::CredentialOrSecretEmission => "credential_or_secret_emission",
            Self::RemoteAttachMutation => "remote_attach_mutation",
            Self::CollaborationInviteMutation => "collaboration_invite_mutation",
            Self::PolicyAuthoringMutation => "policy_authoring_mutation",
            Self::ManagedWorkspaceMutation => "managed_workspace_mutation",
            Self::BrowserHandoffEmission => "browser_handoff_emission",
            Self::TelemetryEmissionHighVolume => "telemetry_emission_high_volume",
            Self::ExtensionLifecycleMutation => "extension_lifecycle_mutation",
            Self::WorkspaceSettingsMutation => "workspace_settings_mutation",
            Self::PolicyPackInstallOrUpdate => "policy_pack_install_or_update",
            Self::SystemBrowserLaunch => "system_browser_launch",
        }
    }
}

/// Frozen rollback class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    NoRollbackRequired,
    AutoRevertOnFailure,
    CheckpointBackedRevert,
    NamedUndoGroupRevert,
    CompensatingActionRequired,
    ManualRepairRequiredNoAutomaticRevert,
    IrreversibleNoRevertPossible,
}

impl RollbackClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRollbackRequired => "no_rollback_required",
            Self::AutoRevertOnFailure => "auto_revert_on_failure",
            Self::CheckpointBackedRevert => "checkpoint_backed_revert",
            Self::NamedUndoGroupRevert => "named_undo_group_revert",
            Self::CompensatingActionRequired => "compensating_action_required",
            Self::ManualRepairRequiredNoAutomaticRevert => {
                "manual_repair_required_no_automatic_revert"
            }
            Self::IrreversibleNoRevertPossible => "irreversible_no_revert_possible",
        }
    }
}

/// Frozen preview-or-dry-run class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewOrDryRunClass {
    PreviewUnavailableByDesign,
    PreviewInlineSummary,
    PreviewStructuredDiff,
    PreviewBatchScope,
    PreviewDestructiveBulk,
    PreviewSimulateDryRun,
    PreviewHandoffPacket,
    PreviewOnlyWithProviderOffline,
}

impl PreviewOrDryRunClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewUnavailableByDesign => "preview_unavailable_by_design",
            Self::PreviewInlineSummary => "preview_inline_summary",
            Self::PreviewStructuredDiff => "preview_structured_diff",
            Self::PreviewBatchScope => "preview_batch_scope",
            Self::PreviewDestructiveBulk => "preview_destructive_bulk",
            Self::PreviewSimulateDryRun => "preview_simulate_dry_run",
            Self::PreviewHandoffPacket => "preview_handoff_packet",
            Self::PreviewOnlyWithProviderOffline => "preview_only_with_provider_offline",
        }
    }
}

/// Frozen blocked-prerequisite class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedPrerequisiteClass {
    MissingRequiredArgument,
    MissingCredentialOrSecret,
    MissingProviderLink,
    MissingWorkspaceScope,
    MissingBasisSnapshot,
    MissingApprovalPath,
    MissingPreviewPath,
    MissingCapabilityGrant,
    MissingFreshnessFloor,
    MissingManagedChannel,
    MissingExecutionContext,
    MissingPolicyAdmission,
    MissingRuntimePromptValue,
    MissingTrustStateAdmission,
}

impl BlockedPrerequisiteClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequiredArgument => "missing_required_argument",
            Self::MissingCredentialOrSecret => "missing_credential_or_secret",
            Self::MissingProviderLink => "missing_provider_link",
            Self::MissingWorkspaceScope => "missing_workspace_scope",
            Self::MissingBasisSnapshot => "missing_basis_snapshot",
            Self::MissingApprovalPath => "missing_approval_path",
            Self::MissingPreviewPath => "missing_preview_path",
            Self::MissingCapabilityGrant => "missing_capability_grant",
            Self::MissingFreshnessFloor => "missing_freshness_floor",
            Self::MissingManagedChannel => "missing_managed_channel",
            Self::MissingExecutionContext => "missing_execution_context",
            Self::MissingPolicyAdmission => "missing_policy_admission",
            Self::MissingRuntimePromptValue => "missing_runtime_prompt_value",
            Self::MissingTrustStateAdmission => "missing_trust_state_admission",
        }
    }
}

/// Frozen process-action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessActionClass {
    NoProcessAction,
    SpawnLocalProcess,
    SpawnRemoteProcess,
    AttachToRunningProcess,
    KillRunningProcess,
    RestartRunningProcess,
    LaunchSystemBrowser,
}

impl ProcessActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoProcessAction => "no_process_action",
            Self::SpawnLocalProcess => "spawn_local_process",
            Self::SpawnRemoteProcess => "spawn_remote_process",
            Self::AttachToRunningProcess => "attach_to_running_process",
            Self::KillRunningProcess => "kill_running_process",
            Self::RestartRunningProcess => "restart_running_process",
            Self::LaunchSystemBrowser => "launch_system_browser",
        }
    }
}

/// Frozen network-action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkActionClass {
    NoNetworkAction,
    OutboundRequestInternalProvider,
    OutboundRequestExternalProvider,
    OutboundRequestRemoteWorkspace,
    OutboundRequestCollaborationRemote,
    OutboundRequestManagedAdminPlane,
    InboundListenLocalLoopback,
    InboundListenRemoteAccessible,
}

impl NetworkActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoNetworkAction => "no_network_action",
            Self::OutboundRequestInternalProvider => "outbound_request_internal_provider",
            Self::OutboundRequestExternalProvider => "outbound_request_external_provider",
            Self::OutboundRequestRemoteWorkspace => "outbound_request_remote_workspace",
            Self::OutboundRequestCollaborationRemote => "outbound_request_collaboration_remote",
            Self::OutboundRequestManagedAdminPlane => "outbound_request_managed_admin_plane",
            Self::InboundListenLocalLoopback => "inbound_listen_local_loopback",
            Self::InboundListenRemoteAccessible => "inbound_listen_remote_accessible",
        }
    }
}

/// Frozen remote-action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteActionClass {
    NoRemoteAction,
    RemoteAttachPersistent,
    RemoteWorkspaceMutation,
    RemotePublishArtifact,
    RemoteCollaborationInvite,
    RemoteManagedControlAction,
    RemoteBrowserHandoff,
}

impl RemoteActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRemoteAction => "no_remote_action",
            Self::RemoteAttachPersistent => "remote_attach_persistent",
            Self::RemoteWorkspaceMutation => "remote_workspace_mutation",
            Self::RemotePublishArtifact => "remote_publish_artifact",
            Self::RemoteCollaborationInvite => "remote_collaboration_invite",
            Self::RemoteManagedControlAction => "remote_managed_control_action",
            Self::RemoteBrowserHandoff => "remote_browser_handoff",
        }
    }
}

/// Frozen scope-count-truth class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountTruthClass {
    Exact,
    ApproximateCapped,
    ApproximateStreaming,
    UnknownPendingResolution,
}

impl CountTruthClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ApproximateCapped => "approximate_capped",
            Self::ApproximateStreaming => "approximate_streaming",
            Self::UnknownPendingResolution => "unknown_pending_resolution",
        }
    }
}

/// One scope-axis row in the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeAxisRecord {
    pub axis_class: ScopeAxisClass,
    pub included_count: u32,
    #[serde(default)]
    pub excluded_count: u32,
    #[serde(default)]
    pub hidden_or_blocked_count: u32,
    pub count_truth_class: CountTruthClass,
    pub scope_narration_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_scope_refs: Vec<String>,
}

/// Repair-hook ref attached to a blocked prerequisite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairHookRef {
    pub hook_kind: String,
    pub hook_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
}

/// One blocked prerequisite the review sheet lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedPrerequisite {
    pub class: BlockedPrerequisiteClass,
    pub narration_ref: String,
    pub repair_hook_ref: RepairHookRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_code: Option<String>,
}

/// One field's provenance summary as quoted by the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldProvenanceSummaryEntry {
    pub argument_name: String,
    pub source_layer_label_ref: String,
    pub value_visibility: ValueVisibilityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narration_ref: Option<String>,
}

/// Secret-handling rollup the review sheet quotes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretHandlingSummary {
    pub any_secret_bearing_field: bool,
    pub all_handle_only: bool,
    #[serde(default)]
    pub any_runtime_reveal_armed: bool,
    pub redaction_class: RedactionClass,
}

/// Canonical invocation-review-sheet projection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvocationReviewSheetRecord {
    pub record_kind: String,
    pub invocation_review_sheet_schema_version: u32,
    pub review_sheet_id: String,
    pub command_id: String,
    pub command_revision_ref: String,
    pub form_state_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation_session_id: Option<String>,
    pub review_surface_class: ReviewSurfaceClass,
    pub capability_scope_class: CapabilityScopeClass,
    pub preview_class_declared: PreviewClass,
    pub approval_posture_class: ApprovalPostureClass,
    pub execution_intent: ExecutionIntentClass,
    pub scope_axes: Vec<ScopeAxisRecord>,
    pub side_effects: Vec<SideEffectClass>,
    pub process_actions: Vec<ProcessActionClass>,
    pub network_actions: Vec<NetworkActionClass>,
    pub remote_actions: Vec<RemoteActionClass>,
    pub rollback_class: RollbackClass,
    pub preview_or_dry_run_class: PreviewOrDryRunClass,
    pub preview_or_dry_run_available: bool,
    pub blocked_prerequisites: Vec<BlockedPrerequisite>,
    pub field_provenance_summary: Vec<FieldProvenanceSummaryEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_handling_summary: Option<SecretHandlingSummary>,
    pub policy_context: PolicyContext,
    pub redaction_class: RedactionClass,
    pub minted_at: String,
}

impl InvocationReviewSheetRecord {
    /// True when the apply can proceed: no blocked prerequisites, and a
    /// preview / dry-run path is available when the preview class demands
    /// it.
    pub fn is_invocable(&self) -> bool {
        if !self.blocked_prerequisites.is_empty() {
            return false;
        }
        if self.preview_class_declared.requires_preview() && !self.preview_or_dry_run_available {
            return false;
        }
        true
    }
}

/// Paired projection: one parameter form + one review sheet derived from the
/// same command and parameter snapshot. The pair is the unit every surface
/// consumes; rendering one without the other is non-conforming for any
/// non-trivial scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandFormBundle {
    pub bundle_id: String,
    pub scenario_id: String,
    pub form_state: ParameterFormStateRecord,
    pub review_sheet: InvocationReviewSheetRecord,
}

impl CommandFormBundle {
    /// Returns the scenario id for indexing.
    pub fn scenario(&self) -> &str {
        &self.scenario_id
    }
}

/// Catalog of bundles. The catalog is the single mint-from-truth source
/// consumed by the live form host, the markdown report, and the JSON
/// fixtures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandFormsCatalog {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub catalog_id: String,
    pub source_descriptor_schema_ref: String,
    pub source_form_schema_ref: String,
    pub source_review_schema_ref: String,
    pub bundles: Vec<CommandFormBundle>,
    pub generated_at: String,
}

impl CommandFormsCatalog {
    /// Returns the bundle for a scenario id.
    pub fn bundle(&self, scenario_id: &str) -> Option<&CommandFormBundle> {
        self.bundles.iter().find(|b| b.scenario_id == scenario_id)
    }

    /// Returns a map of scenario id to invocability decision.
    pub fn invocability_map(&self) -> BTreeMap<String, bool> {
        self.bundles
            .iter()
            .map(|b| (b.scenario_id.clone(), b.review_sheet.is_invocable()))
            .collect()
    }
}

pub use render::render_catalog_markdown;
pub use seed::seeded_command_forms_catalog;
pub use validation::{
    validate_command_forms_catalog, validate_invocation_review_sheet,
    validate_parameter_form_state, CommandFormsValidationError,
};
