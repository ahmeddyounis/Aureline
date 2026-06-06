//! Stable safe-automation qualification records for command labels and manifests.
//!
//! This module is the command-lane bridge between the descriptor-owned
//! `automation_labels` field, saved automation manifests, and user-facing
//! surfaces such as Add to recipe, Inspect descriptor, and Replay as macro. It
//! does not implement a broad recipe runner. It qualifies the labels, manifest
//! fields, preview policy, export posture, and narrowing rules those surfaces
//! must consume before they can claim automation support.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SafeAutomationQualificationPacket`].
pub const SAFE_AUTOMATION_QUALIFICATION_RECORD_KIND: &str = "safe_automation_qualification_packet";

/// Schema version for safe-automation qualification packets.
pub const SAFE_AUTOMATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the automation manifest boundary schema.
pub const SAFE_AUTOMATION_MANIFEST_SCHEMA_REF: &str =
    "schemas/automation/automation-manifest.schema.json";

/// Repo-relative path of the preview and lifecycle automation contract.
pub const SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF: &str =
    "docs/automation/preview-and-lifecycle.md";

/// Repo-relative path of the recorded macro and recipe contract.
pub const SAFE_AUTOMATION_RECIPE_MACRO_CONTRACT_REF: &str =
    "docs/automation/recipe_and_macro_contract.md";

/// Repo-relative path of the command shareability contract.
pub const SAFE_AUTOMATION_SHAREABILITY_CONTRACT_REF: &str =
    "docs/commands/shareability_and_automation_contract.md";

/// Repo-relative path of the safe automation matrix artifact.
pub const SAFE_AUTOMATION_MATRIX_REF: &str = "artifacts/automation/m4/safe-automation-matrix.md";

/// Repo-relative path of the checked safe-automation support export.
pub const SAFE_AUTOMATION_SUPPORT_EXPORT_REF: &str =
    "artifacts/automation/m4/safe_automation_qualification/support_export.json";

/// Closed automation-label vocabulary shared by command surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledAutomationLabel {
    /// Command can be captured and replayed as a local recorded macro.
    MacroSafe,
    /// Command can be inserted as a typed declarative recipe step.
    RecipeSafe,
    /// Command has a supported CLI or headless contract.
    HeadlessSafe,
    /// Command needs an interactive surface and is not portable automation.
    UiOnly,
    /// Command requires explicit approval before execution.
    ApprovalRequired,
    /// Command may write local files or buffers.
    WritesFiles,
    /// Command may spawn a process or terminal-backed execution path.
    RunsProcess,
    /// Command may call a network service.
    NetworkCall,
    /// Command may mutate a remote target.
    RemoteMutation,
    /// Command or extension did not declare automation support.
    UnknownAutomationSupport,
}

impl ControlledAutomationLabel {
    /// Stable token used in descriptors, manifests, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacroSafe => "macro_safe",
            Self::RecipeSafe => "recipe_safe",
            Self::HeadlessSafe => "headless_safe",
            Self::UiOnly => "ui_only",
            Self::ApprovalRequired => "approval_required",
            Self::WritesFiles => "writes_files",
            Self::RunsProcess => "runs_process",
            Self::NetworkCall => "network_call",
            Self::RemoteMutation => "remote_mutation",
            Self::UnknownAutomationSupport => "unknown_automation_support",
        }
    }

    /// Human-facing controlled term rendered by palette, help, and diagnostics.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::MacroSafe => "Macro-safe",
            Self::RecipeSafe => "Recipe-safe",
            Self::HeadlessSafe => "Headless-safe",
            Self::UiOnly => "UI-only",
            Self::ApprovalRequired => "Approval required",
            Self::WritesFiles => "Writes files",
            Self::RunsProcess => "Runs process",
            Self::NetworkCall => "Network call",
            Self::RemoteMutation => "Remote mutation",
            Self::UnknownAutomationSupport => "Unknown automation support",
        }
    }

    /// Parses a stable descriptor token.
    pub fn from_token(value: &str) -> Option<Self> {
        match value {
            "macro_safe" => Some(Self::MacroSafe),
            "recipe_safe" => Some(Self::RecipeSafe),
            "headless_safe" => Some(Self::HeadlessSafe),
            "ui_only" => Some(Self::UiOnly),
            "approval_required" => Some(Self::ApprovalRequired),
            "writes_files" => Some(Self::WritesFiles),
            "runs_process" => Some(Self::RunsProcess),
            "network_call" => Some(Self::NetworkCall),
            "remote_mutation" => Some(Self::RemoteMutation),
            "unknown_automation_support" => Some(Self::UnknownAutomationSupport),
            _ => None,
        }
    }

    /// Controlled labels that must be documented by the stable qualification lane.
    pub const fn required_coverage() -> [Self; 9] {
        [
            Self::MacroSafe,
            Self::RecipeSafe,
            Self::HeadlessSafe,
            Self::UiOnly,
            Self::ApprovalRequired,
            Self::WritesFiles,
            Self::RunsProcess,
            Self::NetworkCall,
            Self::RemoteMutation,
        ]
    }
}

/// Returns true when the label token set includes the requested label.
pub fn labels_include(labels: &[String], label: ControlledAutomationLabel) -> bool {
    labels.iter().any(|candidate| candidate == label.as_str())
}

/// Converts descriptor automation labels into controlled display labels.
pub fn automation_display_labels(labels: &[String]) -> Vec<String> {
    if labels.is_empty() {
        return vec![ControlledAutomationLabel::UnknownAutomationSupport
            .display_label()
            .to_owned()];
    }

    labels
        .iter()
        .map(|label| {
            ControlledAutomationLabel::from_token(label)
                .map(ControlledAutomationLabel::display_label)
                .unwrap_or(label)
                .to_owned()
        })
        .collect()
}

/// Returns the first stable reason a command cannot be treated as fully automatable.
pub fn why_not_automatable_reason(
    labels: &[String],
    approval_posture_class: &str,
) -> Option<String> {
    if labels_include(labels, ControlledAutomationLabel::UnknownAutomationSupport) {
        Some("unknown_automation_support".to_owned())
    } else if labels_include(labels, ControlledAutomationLabel::UiOnly) {
        Some("ui_only".to_owned())
    } else if labels_include(labels, ControlledAutomationLabel::ApprovalRequired)
        || approval_posture_class != "no_approval_required"
    {
        Some("approval_required".to_owned())
    } else if !labels_include(labels, ControlledAutomationLabel::RecipeSafe) {
        Some("not_recipe_safe".to_owned())
    } else if !labels_include(labels, ControlledAutomationLabel::HeadlessSafe) {
        Some("not_headless_safe".to_owned())
    } else if !labels_include(labels, ControlledAutomationLabel::MacroSafe) {
        Some("not_macro_safe".to_owned())
    } else {
        None
    }
}

/// Qualified automation object classes surfaced by the stable automation lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationObjectClass {
    /// Profile-local recorded editor or review macro.
    RecordedMacro,
    /// Workspace-authored declarative recipe.
    WorkspaceRecipe,
    /// Recipe contributed by an extension package.
    ExtensionRecipe,
    /// Signed admin, policy, or curated recipe pack.
    AdminCuratedRecipePack,
    /// Transient AI-generated plan optionally saved as a recipe draft.
    EphemeralAiGeneratedRecipe,
}

impl AutomationObjectClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordedMacro => "recorded_macro",
            Self::WorkspaceRecipe => "workspace_recipe",
            Self::ExtensionRecipe => "extension_recipe",
            Self::AdminCuratedRecipePack => "admin_curated_recipe_pack",
            Self::EphemeralAiGeneratedRecipe => "ephemeral_ai_generated_recipe",
        }
    }

    /// Object classes every qualification packet must cover.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::RecordedMacro,
            Self::WorkspaceRecipe,
            Self::ExtensionRecipe,
            Self::AdminCuratedRecipePack,
            Self::EphemeralAiGeneratedRecipe,
        ]
    }

    const fn is_recorded_macro(self) -> bool {
        matches!(self, Self::RecordedMacro)
    }
}

/// Storage or distribution form used by an automation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationStorageFormClass {
    /// Profile-local user artifact.
    LocalUserArtifact,
    /// Versioned manifest stored in a workspace.
    VersionedTextManifest,
    /// Signed extension package or extension-owned manifest.
    SignedExtensionPackageManifest,
    /// Signed admin or policy bundle.
    SignedPolicyBundle,
    /// In-memory generated plan with no saved authority.
    TransientGeneratedPlan,
    /// Redacted support-safe projection.
    SupportSafeProjection,
}

impl AutomationStorageFormClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUserArtifact => "local_user_artifact",
            Self::VersionedTextManifest => "versioned_text_manifest",
            Self::SignedExtensionPackageManifest => "signed_extension_package_manifest",
            Self::SignedPolicyBundle => "signed_policy_bundle",
            Self::TransientGeneratedPlan => "transient_generated_plan",
            Self::SupportSafeProjection => "support_safe_projection",
        }
    }
}

/// Declared capability classes an automation object may require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationCapabilityClass {
    /// Replay explicit editor or review state.
    EditorReviewStateReplay,
    /// Inspect workspace state without mutation.
    WorkspaceRead,
    /// Write workspace files or editor buffers.
    FilesystemWrite,
    /// Launch a process or terminal-backed command.
    ProcessLaunch,
    /// Call a network service.
    NetworkAccess,
    /// Touch a remote target or remote helper.
    RemoteTargetAccess,
    /// Request an AI tool invocation.
    AiToolInvocation,
    /// Use a secret-broker handle without raw secret export.
    SecretHandleReference,
    /// Mutate admin, policy, or managed-fleet state.
    AdminPolicyMutation,
    /// Invoke an extension-provided command within its permission envelope.
    ExtensionInvocation,
}

impl AutomationCapabilityClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorReviewStateReplay => "editor_review_state_replay",
            Self::WorkspaceRead => "workspace_read",
            Self::FilesystemWrite => "filesystem_write",
            Self::ProcessLaunch => "process_launch",
            Self::NetworkAccess => "network_access",
            Self::RemoteTargetAccess => "remote_target_access",
            Self::AiToolInvocation => "ai_tool_invocation",
            Self::SecretHandleReference => "secret_handle_reference",
            Self::AdminPolicyMutation => "admin_policy_mutation",
            Self::ExtensionInvocation => "extension_invocation",
        }
    }

    const fn is_forbidden_for_recorded_macro(self) -> bool {
        matches!(
            self,
            Self::FilesystemWrite
                | Self::ProcessLaunch
                | Self::NetworkAccess
                | Self::RemoteTargetAccess
                | Self::AiToolInvocation
                | Self::SecretHandleReference
                | Self::AdminPolicyMutation
                | Self::ExtensionInvocation
        )
    }
}

/// Trust posture required before an automation object can be admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrustRequirementClass {
    /// Local profile artifact only; no workspace or repo import.
    LocalOnly,
    /// Workspace trust is required.
    TrustedWorkspace,
    /// Signed source or publisher admission is required.
    SignedSource,
    /// Admin allowlist or policy admission is required.
    AdminAllowlist,
    /// Extension permission envelope bounds the recipe.
    ExtensionPermissionEnvelope,
    /// Policy-provided source governs admission.
    PolicyProvided,
}

impl AutomationTrustRequirementClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TrustedWorkspace => "trusted_workspace",
            Self::SignedSource => "signed_source",
            Self::AdminAllowlist => "admin_allowlist",
            Self::ExtensionPermissionEnvelope => "extension_permission_envelope",
            Self::PolicyProvided => "policy_provided",
        }
    }
}

/// Preview or dry-run policy required before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationPreviewPolicyClass {
    /// Read-only object requires no mutation preview.
    NoMutationPreviewNotApplicable,
    /// Recorded macro replay requires an edit-history checkpoint before apply.
    CheckpointRequiredBeforeReplay,
    /// The user must see a displayable plan before mutation.
    DisplayablePlanRequiredBeforeMutation,
    /// Underlying command dry-run is required before apply.
    DryRunRequiredBeforeApply,
    /// A diff preview is required before apply.
    DiffPreviewRequired,
    /// An impact summary is required before apply.
    ImpactSummaryRequired,
    /// Non-simulatable steps must be disclosed explicitly.
    NonSimulatableStepMustExplain,
}

impl AutomationPreviewPolicyClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutationPreviewNotApplicable => "no_mutation_preview_not_applicable",
            Self::CheckpointRequiredBeforeReplay => "checkpoint_required_before_replay",
            Self::DisplayablePlanRequiredBeforeMutation => {
                "displayable_plan_required_before_mutation"
            }
            Self::DryRunRequiredBeforeApply => "dry_run_required_before_apply",
            Self::DiffPreviewRequired => "diff_preview_required",
            Self::ImpactSummaryRequired => "impact_summary_required",
            Self::NonSimulatableStepMustExplain => "non_simulatable_step_must_explain",
        }
    }
}

/// Rerun and retry hint preserved in support-safe manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationIdempotencyHintClass {
    /// Replay is deterministic when the target context still matches.
    DeterministicGivenSameTargetContext,
    /// Retry is safe only after re-resolving current context and policy.
    RetryRequiresFreshContextResolution,
    /// Retry requires reconciliation receipt or bounded replay window.
    RetryRequiresReconciliationReceipt,
    /// Non-idempotent step requires explicit user review on every run.
    NonIdempotentRequiresExplicitReview,
}

impl AutomationIdempotencyHintClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicGivenSameTargetContext => "deterministic_given_same_target_context",
            Self::RetryRequiresFreshContextResolution => "retry_requires_fresh_context_resolution",
            Self::RetryRequiresReconciliationReceipt => "retry_requires_reconciliation_receipt",
            Self::NonIdempotentRequiresExplicitReview => "non_idempotent_requires_explicit_review",
        }
    }
}

/// Provenance class preserved on saved or support-safe automation manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationProvenanceClass {
    /// Captured by the current user profile.
    UserRecorded,
    /// Authored in the workspace.
    WorkspaceAuthored,
    /// Published by an extension publisher.
    ExtensionPublisher,
    /// Signed by an admin or curated policy source.
    AdminCuratedSigner,
    /// Derived from an AI evidence packet.
    AiEvidencePacket,
    /// Redacted support-safe projection.
    SupportRedactedExport,
}

impl AutomationProvenanceClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRecorded => "user_recorded",
            Self::WorkspaceAuthored => "workspace_authored",
            Self::ExtensionPublisher => "extension_publisher",
            Self::AdminCuratedSigner => "admin_curated_signer",
            Self::AiEvidencePacket => "ai_evidence_packet",
            Self::SupportRedactedExport => "support_redacted_export",
        }
    }
}

/// Lifecycle ceiling applied to an automation object or surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationLifecycleLabelClass {
    /// Qualified for the Stable lane in the declared narrow scope.
    StableQualified,
    /// Stable command labels are qualified, but runtime breadth is narrower than Stable.
    StableLabelsOnlyNarrowedRunner,
    /// Available only behind Labs.
    LabsOnly,
    /// Preview-only until a later proof row qualifies it.
    PreviewOnly,
    /// Blocked on a named dependency or policy proof.
    DependencyGated,
    /// Denied from Stable surfaces.
    DeniedForStable,
}

impl AutomationLifecycleLabelClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableQualified => "stable_qualified",
            Self::StableLabelsOnlyNarrowedRunner => "stable_labels_only_narrowed_runner",
            Self::LabsOnly => "labs_only",
            Self::PreviewOnly => "preview_only",
            Self::DependencyGated => "dependency_gated",
            Self::DeniedForStable => "denied_for_stable",
        }
    }

    /// True when the automation object may claim the Stable lane.
    pub const fn is_stable_claim(self) -> bool {
        matches!(self, Self::StableQualified)
    }

    /// True when the lifecycle label narrows below Stable.
    pub const fn is_narrowed_below_stable(self) -> bool {
        matches!(
            self,
            Self::StableLabelsOnlyNarrowedRunner
                | Self::LabsOnly
                | Self::PreviewOnly
                | Self::DependencyGated
                | Self::DeniedForStable
        )
    }
}

/// Authority class preserved by export, import, and support-safe manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationArtifactAuthorityClass {
    /// User-local artifact with no signer.
    LocalOnly,
    /// Signed artifact or package-owned manifest.
    SignedArtifact,
    /// Policy-provided artifact.
    PolicyProvided,
    /// Redacted support projection that is not executable.
    SupportProjectionOnly,
}

impl AutomationArtifactAuthorityClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SignedArtifact => "signed_artifact",
            Self::PolicyProvided => "policy_provided",
            Self::SupportProjectionOnly => "support_projection_only",
        }
    }
}

/// Surface action that consumes command automation labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSurfaceActionClass {
    /// Insert a typed command step into a recipe draft.
    AddToRecipe,
    /// Inspect the command descriptor and automation backing fields.
    InspectDescriptor,
    /// Replay the command through the local recorded-macro lane.
    ReplayAsMacro,
}

impl AutomationSurfaceActionClass {
    /// Stable token used in exports and manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AddToRecipe => "add_to_recipe",
            Self::InspectDescriptor => "inspect_descriptor",
            Self::ReplayAsMacro => "replay_as_macro",
        }
    }

    /// Surface actions every qualification packet must cover.
    pub const fn required_coverage() -> [Self; 3] {
        [
            Self::AddToRecipe,
            Self::InspectDescriptor,
            Self::ReplayAsMacro,
        ]
    }
}

/// Manifest row defining one automation object class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationClassManifestRow {
    /// Automation object class.
    pub object_class: AutomationObjectClass,
    /// Storage or distribution form.
    pub storage_form: AutomationStorageFormClass,
    /// Capability union required by this class.
    pub required_capabilities: Vec<AutomationCapabilityClass>,
    /// Trust or source requirement.
    pub trust_requirement: AutomationTrustRequirementClass,
    /// Preview or dry-run policy.
    pub preview_policy: AutomationPreviewPolicyClass,
    /// Rerun and retry hint.
    pub idempotency_hint: AutomationIdempotencyHintClass,
    /// Provenance field class.
    pub provenance: AutomationProvenanceClass,
    /// Lifecycle ceiling for this class.
    pub lifecycle_label: AutomationLifecycleLabelClass,
    /// Authority class preserved on import and export.
    pub artifact_authority: AutomationArtifactAuthorityClass,
    /// True when the object's declared scope is narrower than general automation.
    pub scope_narrower_than_stable: bool,
    /// True when a deterministic replay boundary is declared.
    pub deterministic_replay_boundary: bool,
    /// True when raw secrets are forbidden from manifest and support export.
    pub forbids_raw_secret_capture: bool,
    /// True when network and process access must be declared up front.
    pub forbids_undeclared_network_or_process_access: bool,
    /// True when hidden authority cannot be captured from the invoking surface.
    pub forbids_hidden_authority: bool,
    /// True when mutation creates or cites an edit-history checkpoint.
    pub edit_history_checkpoint_required: bool,
    /// True when this class claims Stable.
    pub claimed_stable: bool,
    /// Optional proof or dependency marker ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_or_dependency_ref: Option<String>,
}

/// Surface contract proving command actions consume the typed automation model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAutomationSurfaceContract {
    /// Surface action being qualified.
    pub action_class: AutomationSurfaceActionClass,
    /// Required controlled labels before the action may be enabled.
    pub required_labels: Vec<ControlledAutomationLabel>,
    /// Manifest fields the surface reads before enabling the action.
    pub backing_manifest_fields: Vec<String>,
    /// Lifecycle ceiling applied to the action.
    pub lifecycle_ceiling: AutomationLifecycleLabelClass,
    /// True when the action never runs the command while adding or inspecting.
    pub insertion_or_inspection_only: bool,
    /// True when the action reuses command descriptors rather than ambient state.
    pub consumes_command_descriptor: bool,
    /// True when the action preserves preview policy from the manifest.
    pub consumes_preview_policy: bool,
    /// True when saved artifacts cannot capture raw secrets.
    pub no_saved_artifact_captures_secrets: bool,
    /// True when undeclared network or process access is rejected.
    pub no_undeclared_network_or_process_access: bool,
    /// True when hidden authority is rejected.
    pub no_hidden_authority: bool,
    /// True when deterministic replay boundaries are preserved.
    pub deterministic_replay_boundary_preserved: bool,
    /// True when the action can be shown on Stable command surfaces.
    pub claimed_stable: bool,
}

/// Export and import contract for safe automation manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationManifestExportImportContract {
    /// True when local-only, signed, and policy-provided authority classes are distinct.
    pub distinguishes_local_signed_policy_artifacts: bool,
    /// True when support export produces non-executable redacted projections.
    pub support_safe_projection_only: bool,
    /// True when export strips raw secret values and prompt bodies.
    pub strips_raw_secrets_and_prompts: bool,
    /// True when deterministic replay boundaries survive export and import.
    pub preserves_deterministic_replay_boundaries: bool,
    /// True when import revalidates trust and policy.
    pub import_revalidates_trust_and_policy: bool,
    /// True when import preserves content or manifest identity refs.
    pub preserves_manifest_identity_refs: bool,
    /// True when signed or policy-provided artifacts cannot downgrade to local-only.
    pub prevents_authority_downgrade_on_import: bool,
}

/// Evidence refs that connect the packet to docs, schema, matrix, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeAutomationEvidenceExport {
    /// Stable evidence id.
    pub evidence_id: String,
    /// JSON support export ref.
    pub support_export_ref: String,
    /// Markdown matrix ref.
    pub matrix_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Preview and lifecycle doc ref.
    pub preview_lifecycle_doc_ref: String,
}

/// Constructor input for [`SafeAutomationQualificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeAutomationQualificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Whether this packet qualifies Stable labels only in the declared scope.
    pub claimed_stable_label_truth: bool,
    /// Policy epoch ref used for evaluation.
    pub policy_epoch_ref: String,
    /// Controlled labels documented by this packet.
    pub controlled_labels: Vec<ControlledAutomationLabel>,
    /// Automation object-class rows.
    pub automation_classes: Vec<AutomationClassManifestRow>,
    /// Command surface rows.
    pub surface_contracts: Vec<CommandAutomationSurfaceContract>,
    /// Export/import contract.
    pub export_import_contract: AutomationManifestExportImportContract,
    /// Evidence export refs.
    pub evidence_export: SafeAutomationEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stable qualification packet for safe automation surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeAutomationQualificationPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Whether this packet qualifies Stable labels only in the declared scope.
    pub claimed_stable_label_truth: bool,
    /// Policy epoch ref used for evaluation.
    pub policy_epoch_ref: String,
    /// Controlled labels documented by this packet.
    pub controlled_labels: Vec<ControlledAutomationLabel>,
    /// Automation object-class rows.
    pub automation_classes: Vec<AutomationClassManifestRow>,
    /// Command surface rows.
    pub surface_contracts: Vec<CommandAutomationSurfaceContract>,
    /// Export/import contract.
    pub export_import_contract: AutomationManifestExportImportContract,
    /// Evidence export refs.
    pub evidence_export: SafeAutomationEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SafeAutomationQualificationPacket {
    /// Builds a safe-automation qualification packet from canonical rows.
    pub fn new(input: SafeAutomationQualificationPacketInput) -> Self {
        Self {
            record_kind: SAFE_AUTOMATION_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: SAFE_AUTOMATION_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            claimed_stable_label_truth: input.claimed_stable_label_truth,
            policy_epoch_ref: input.policy_epoch_ref,
            controlled_labels: input.controlled_labels,
            automation_classes: input.automation_classes,
            surface_contracts: input.surface_contracts,
            export_import_contract: input.export_import_contract,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's safe-automation and narrowing invariants.
    pub fn validate(&self) -> Vec<SafeAutomationQualificationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != SAFE_AUTOMATION_QUALIFICATION_RECORD_KIND {
            violations.push(SafeAutomationQualificationViolation::WrongRecordKind);
        }
        if self.schema_version != SAFE_AUTOMATION_QUALIFICATION_SCHEMA_VERSION {
            violations.push(SafeAutomationQualificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SafeAutomationQualificationViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_label_coverage(self, &mut violations);
        validate_automation_classes(self, &mut violations);
        validate_surface_contracts(self, &mut violations);
        validate_export_import(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("safe automation packet serializes"),
        ) {
            violations.push(SafeAutomationQualificationViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("safe automation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_classes = self
            .automation_classes
            .iter()
            .filter(|row| row.lifecycle_label.is_stable_claim())
            .count();
        let narrowed_classes = self
            .automation_classes
            .iter()
            .filter(|row| row.lifecycle_label.is_narrowed_below_stable())
            .count();
        let stable_surfaces = self
            .surface_contracts
            .iter()
            .filter(|row| row.claimed_stable)
            .count();
        let mut out = String::new();
        out.push_str("# Safe Automation Qualification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Stable label truth: {}\n",
            self.claimed_stable_label_truth
        ));
        out.push_str(&format!(
            "- Controlled labels: {}\n",
            self.controlled_labels.len()
        ));
        out.push_str(&format!(
            "- Automation classes: {} ({} stable, {} narrowed below Stable)\n",
            self.automation_classes.len(),
            stable_classes,
            narrowed_classes
        ));
        out.push_str(&format!(
            "- Command surface actions: {} ({} claim Stable label truth)\n",
            self.surface_contracts.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out
    }
}

/// Errors emitted when reading the checked-in safe-automation support export.
#[derive(Debug)]
pub enum SafeAutomationQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SafeAutomationQualificationViolation>),
}

impl fmt::Display for SafeAutomationQualificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "safe automation export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "safe automation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SafeAutomationQualificationArtifactError {}

/// Validation failures emitted by [`SafeAutomationQualificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafeAutomationQualificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Controlled label coverage is incomplete.
    MissingControlledLabelCoverage,
    /// Automation object-class coverage is incomplete.
    MissingAutomationClassCoverage,
    /// A manifest row is missing its required field values.
    ManifestRowMissingRequiredField,
    /// Recorded macro row declares forbidden capability or storage/trust posture.
    RecordedMacroTooBroad,
    /// A non-macro automation class claims Stable without qualification.
    BroadAutomationClaimsStable,
    /// An unqualified automation class lacks Preview, Labs, or dependency gating.
    UnqualifiedAutomationNotNarrowed,
    /// Surface action coverage is incomplete.
    MissingSurfaceActionCoverage,
    /// Surface action does not consume typed manifest fields.
    SurfaceDoesNotConsumeTypedModel,
    /// Add to recipe is not gated by the Recipe-safe label.
    AddToRecipeNotGatedByRecipeSafe,
    /// Replay as macro is not gated by the Macro-safe label.
    ReplayAsMacroNotGatedByMacroSafe,
    /// Export/import contract does not preserve authority and support-safe posture.
    ExportImportContractIncomplete,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl SafeAutomationQualificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingControlledLabelCoverage => "missing_controlled_label_coverage",
            Self::MissingAutomationClassCoverage => "missing_automation_class_coverage",
            Self::ManifestRowMissingRequiredField => "manifest_row_missing_required_field",
            Self::RecordedMacroTooBroad => "recorded_macro_too_broad",
            Self::BroadAutomationClaimsStable => "broad_automation_claims_stable",
            Self::UnqualifiedAutomationNotNarrowed => "unqualified_automation_not_narrowed",
            Self::MissingSurfaceActionCoverage => "missing_surface_action_coverage",
            Self::SurfaceDoesNotConsumeTypedModel => "surface_does_not_consume_typed_model",
            Self::AddToRecipeNotGatedByRecipeSafe => "add_to_recipe_not_gated_by_recipe_safe",
            Self::ReplayAsMacroNotGatedByMacroSafe => "replay_as_macro_not_gated_by_macro_safe",
            Self::ExportImportContractIncomplete => "export_import_contract_incomplete",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in safe-automation support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_safe_automation_qualification_export(
) -> Result<SafeAutomationQualificationPacket, SafeAutomationQualificationArtifactError> {
    let packet: SafeAutomationQualificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/automation/m4/safe_automation_qualification/support_export.json"
    )))
    .map_err(SafeAutomationQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SafeAutomationQualificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    for required in [
        SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF,
        SAFE_AUTOMATION_MANIFEST_SCHEMA_REF,
        SAFE_AUTOMATION_RECIPE_MACRO_CONTRACT_REF,
        SAFE_AUTOMATION_SHAREABILITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(SafeAutomationQualificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_label_coverage(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    for required in ControlledAutomationLabel::required_coverage() {
        if !packet
            .controlled_labels
            .iter()
            .any(|label| *label == required)
        {
            violations.push(SafeAutomationQualificationViolation::MissingControlledLabelCoverage);
            break;
        }
    }
}

fn validate_automation_classes(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    for required in AutomationObjectClass::required_coverage() {
        if !packet
            .automation_classes
            .iter()
            .any(|row| row.object_class == required)
        {
            violations.push(SafeAutomationQualificationViolation::MissingAutomationClassCoverage);
            break;
        }
    }

    for row in &packet.automation_classes {
        if row.required_capabilities.is_empty()
            || row
                .proof_or_dependency_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
                && !row.lifecycle_label.is_stable_claim()
        {
            violations.push(SafeAutomationQualificationViolation::ManifestRowMissingRequiredField);
            break;
        }

        if row.object_class.is_recorded_macro() {
            let macro_too_broad = row.storage_form != AutomationStorageFormClass::LocalUserArtifact
                || row.trust_requirement != AutomationTrustRequirementClass::LocalOnly
                || !row.deterministic_replay_boundary
                || !row.forbids_raw_secret_capture
                || !row.forbids_undeclared_network_or_process_access
                || !row.forbids_hidden_authority
                || !row.edit_history_checkpoint_required
                || row
                    .required_capabilities
                    .iter()
                    .any(|capability| capability.is_forbidden_for_recorded_macro());
            if macro_too_broad {
                violations.push(SafeAutomationQualificationViolation::RecordedMacroTooBroad);
                break;
            }
        } else if row.claimed_stable || row.lifecycle_label.is_stable_claim() {
            violations.push(SafeAutomationQualificationViolation::BroadAutomationClaimsStable);
            break;
        } else if !row.lifecycle_label.is_narrowed_below_stable()
            || row
                .proof_or_dependency_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
        {
            violations.push(SafeAutomationQualificationViolation::UnqualifiedAutomationNotNarrowed);
            break;
        }
    }
}

fn validate_surface_contracts(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    for required in AutomationSurfaceActionClass::required_coverage() {
        if !packet
            .surface_contracts
            .iter()
            .any(|row| row.action_class == required)
        {
            violations.push(SafeAutomationQualificationViolation::MissingSurfaceActionCoverage);
            break;
        }
    }

    for row in &packet.surface_contracts {
        let required_fields: BTreeSet<&str> = row
            .backing_manifest_fields
            .iter()
            .map(String::as_str)
            .collect();
        let consumes_required_fields = [
            "storage_form",
            "required_capabilities",
            "trust_requirement",
            "preview_policy",
            "idempotency_hint",
            "provenance",
            "lifecycle_label",
        ]
        .iter()
        .all(|field| required_fields.contains(field));
        if !consumes_required_fields
            || !row.consumes_command_descriptor
            || !row.consumes_preview_policy
            || !row.no_saved_artifact_captures_secrets
            || !row.no_undeclared_network_or_process_access
            || !row.no_hidden_authority
            || !row.deterministic_replay_boundary_preserved
        {
            violations.push(SafeAutomationQualificationViolation::SurfaceDoesNotConsumeTypedModel);
            break;
        }
        if row.action_class == AutomationSurfaceActionClass::AddToRecipe
            && !row
                .required_labels
                .iter()
                .any(|label| *label == ControlledAutomationLabel::RecipeSafe)
        {
            violations.push(SafeAutomationQualificationViolation::AddToRecipeNotGatedByRecipeSafe);
            break;
        }
        if row.action_class == AutomationSurfaceActionClass::ReplayAsMacro
            && !row
                .required_labels
                .iter()
                .any(|label| *label == ControlledAutomationLabel::MacroSafe)
        {
            violations.push(SafeAutomationQualificationViolation::ReplayAsMacroNotGatedByMacroSafe);
            break;
        }
    }
}

fn validate_export_import(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    let contract = &packet.export_import_contract;
    if !(contract.distinguishes_local_signed_policy_artifacts
        && contract.support_safe_projection_only
        && contract.strips_raw_secrets_and_prompts
        && contract.preserves_deterministic_replay_boundaries
        && contract.import_revalidates_trust_and_policy
        && contract.preserves_manifest_identity_refs
        && contract.prevents_authority_downgrade_on_import)
    {
        violations.push(SafeAutomationQualificationViolation::ExportImportContractIncomplete);
    }
}

fn validate_evidence_export(
    packet: &SafeAutomationQualificationPacket,
    violations: &mut Vec<SafeAutomationQualificationViolation>,
) {
    let evidence = &packet.evidence_export;
    if evidence.evidence_id.trim().is_empty()
        || evidence.support_export_ref != SAFE_AUTOMATION_SUPPORT_EXPORT_REF
        || evidence.matrix_ref != SAFE_AUTOMATION_MATRIX_REF
        || evidence.schema_ref != SAFE_AUTOMATION_MANIFEST_SCHEMA_REF
        || evidence.preview_lifecycle_doc_ref != SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF
    {
        violations.push(SafeAutomationQualificationViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let normalized = text.to_ascii_lowercase();
            normalized.contains("raw_secret")
                || normalized.contains("raw credential")
                || normalized.contains("private_key")
                || normalized.contains("bearer ")
                || normalized.contains("sk-")
                || normalized.contains("raw_prompt")
                || normalized.contains("raw_clipboard")
        }
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

#[cfg(test)]
mod tests;
