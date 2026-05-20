//! Boundary records for the scaffold-safety beta lane.
//!
//! Every record mirrors the matching seed schema in `schemas/workspace/`:
//!
//! - [`TemplateGeneratorDescriptor`] — `template_generator_descriptor.schema.json`
//! - [`ScaffoldPlanRecord`] — `scaffold_plan.schema.json`
//! - [`ScaffoldRunRecord`] — `scaffold_run.schema.json`
//!
//! The records carry only opaque refs and typed labels. Raw absolute
//! paths, raw credentials, raw remote URLs, raw archive bytes, raw
//! template bytes, and raw generated file content never appear. The closed
//! vocabulary is frozen by `docs/workspace/m3/scaffold_safety_beta.md`;
//! adding a new enum value is additive-minor, repurposing one is breaking.

use serde::{Deserialize, Serialize};

use super::shared::FixtureMetadata;

pub const TEMPLATE_GENERATOR_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;
pub const SCAFFOLD_PLAN_SCHEMA_VERSION: u32 = 1;
pub const SCAFFOLD_RUN_SCHEMA_VERSION: u32 = 1;

pub const TEMPLATE_GENERATOR_DESCRIPTOR_RECORD_KIND: &str = "template_generator_descriptor_record";
pub const SCAFFOLD_PLAN_RECORD_KIND: &str = "scaffold_plan_record";
pub const SCAFFOLD_RUN_RECORD_KIND: &str = "scaffold_run_record";

// ----------------------------------------------------------------------
// TemplateGeneratorDescriptor
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateGeneratorDescriptorRecordKind {
    TemplateGeneratorDescriptorRecord,
}

/// Closed provider-class set. Names who authored and ships the template or
/// generator. AI-assisted and extension-provided generation are first-class
/// providers, not a privileged bypass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateProviderClass {
    FirstParty,
    PartnerSigned,
    CommunitySigned,
    RepoLocal,
    AdHoc,
    AiAssisted,
    ExtensionProvided,
}

impl TemplateProviderClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::PartnerSigned => "partner_signed",
            Self::CommunitySigned => "community_signed",
            Self::RepoLocal => "repo_local",
            Self::AdHoc => "ad_hoc",
            Self::AiAssisted => "ai_assisted",
            Self::ExtensionProvided => "extension_provided",
        }
    }

    /// True when the provider is an AI assistant or an extension and the
    /// generation MUST reuse the governed scaffold-plan / diff-review
    /// surface rather than inventing IDE-only authority.
    pub const fn is_ai_or_extension(self) -> bool {
        matches!(self, Self::AiAssisted | Self::ExtensionProvided)
    }
}

/// Closed descriptor signature-state set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorSignatureState {
    SignedVerified,
    SignedUnverified,
    Unsigned,
    SignatureMissing,
    SignatureMismatch,
}

impl DescriptorSignatureState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::SignedUnverified => "signed_unverified",
            Self::Unsigned => "unsigned",
            Self::SignatureMissing => "signature_missing",
            Self::SignatureMismatch => "signature_mismatch",
        }
    }

    /// True when a surface that renders this descriptor as fully trusted
    /// would be lying: the descriptor is unsigned, unverified, or its
    /// signature is missing / mismatched.
    pub const fn is_not_verified(self) -> bool {
        !matches!(self, Self::SignedVerified)
    }
}

/// Closed generation-kind set. Names what the descriptor produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationKindClass {
    ProjectTemplate,
    FileOrComponentGenerator,
    MultiFileScaffold,
    UpdateMigrationGenerator,
}

impl GenerationKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectTemplate => "project_template",
            Self::FileOrComponentGenerator => "file_or_component_generator",
            Self::MultiFileScaffold => "multi_file_scaffold",
            Self::UpdateMigrationGenerator => "update_migration_generator",
        }
    }
}

/// Closed parameter-kind set for a declared required parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterKind {
    Text,
    Identifier,
    Path,
    Boolean,
    Enumerated,
    Number,
    SecretHandle,
}

/// One required-parameter declaration on the descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorParameter {
    pub parameter_key: String,
    pub parameter_kind: ParameterKind,
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_label: Option<String>,
}

/// Closed hook-trigger set. Names when a declared hook would run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookTriggerClass {
    PreGenerate,
    PostGenerate,
    PostCreate,
    PreCommit,
    PostCheckout,
    OnOpen,
}

/// Closed hook-execution set. Names what running a declared hook costs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookExecutionClass {
    BrowseSafe,
    LocalSideEffect,
    NetworkRequired,
    Privileged,
}

impl HookExecutionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowseSafe => "browse_safe",
            Self::LocalSideEffect => "local_side_effect",
            Self::NetworkRequired => "network_required",
            Self::Privileged => "privileged",
        }
    }
}

/// One declared hook on the descriptor. A scaffold run may invoke only
/// hooks that appear here; the run never invents an undeclared hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredHook {
    pub hook_id: String,
    pub trigger_class: HookTriggerClass,
    pub execution_class: HookExecutionClass,
    /// True when the hook touches the network on execution.
    pub network_egress: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Closed validation-task class set the descriptor declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationTaskClass {
    Build,
    Test,
    Lint,
    Typecheck,
    Format,
    DependencyAudit,
    HealthCheck,
    SmokeRun,
}

/// One declared validation task on the descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredValidationTask {
    pub task_id: String,
    pub task_class: ValidationTaskClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Closed trust-expectation set. Names the trust level the descriptor
/// asks the workspace to grant before generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustExpectationClass {
    NoElevation,
    WorkspaceTrustRequired,
    NetworkTrustRequired,
    CredentialAccessRequired,
    PrivilegedExecutionRequired,
}

impl TrustExpectationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoElevation => "no_elevation",
            Self::WorkspaceTrustRequired => "workspace_trust_required",
            Self::NetworkTrustRequired => "network_trust_required",
            Self::CredentialAccessRequired => "credential_access_required",
            Self::PrivilegedExecutionRequired => "privileged_execution_required",
        }
    }
}

/// Closed egress-posture set. Names what network egress the descriptor
/// expects during generation and setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressPostureClass {
    NoEgress,
    RegistryFetch,
    RemoteImagePull,
    GitFetch,
    ArbitraryNetwork,
}

impl EgressPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEgress => "no_egress",
            Self::RegistryFetch => "registry_fetch",
            Self::RemoteImagePull => "remote_image_pull",
            Self::GitFetch => "git_fetch",
            Self::ArbitraryNetwork => "arbitrary_network",
        }
    }

    /// True when generation expects to reach the network.
    pub const fn is_network_bearing(self) -> bool {
        !matches!(self, Self::NoEgress)
    }
}

/// Closed policy-constraint class the descriptor advertises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyConstraintClass {
    AdminApprovalRequired,
    SignatureRequired,
    NetworkBlockedByPolicy,
    RegistryAllowlistOnly,
    OfflineOnly,
    FleetPinnedVersion,
}

impl PolicyConstraintClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminApprovalRequired => "admin_approval_required",
            Self::SignatureRequired => "signature_required",
            Self::NetworkBlockedByPolicy => "network_blocked_by_policy",
            Self::RegistryAllowlistOnly => "registry_allowlist_only",
            Self::OfflineOnly => "offline_only",
            Self::FleetPinnedVersion => "fleet_pinned_version",
        }
    }
}

/// Typed provenance block. Names the producer, source distribution, and a
/// provenance attestation ref. Raw bytes never appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorProvenance {
    pub source_distribution_class: SourceDistributionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_attestation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_build: Option<String>,
}

/// Closed source-distribution class. Names how the descriptor reached the
/// workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDistributionClass {
    PublicRegistry,
    Mirror,
    OfflineBundle,
    RepoLocal,
    AdHoc,
    AiGenerated,
    ExtensionBundled,
}

impl SourceDistributionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::Mirror => "mirror",
            Self::OfflineBundle => "offline_bundle",
            Self::RepoLocal => "repo_local",
            Self::AdHoc => "ad_hoc",
            Self::AiGenerated => "ai_generated",
            Self::ExtensionBundled => "extension_bundled",
        }
    }
}

/// One template / generator descriptor. Mirrors
/// `template_generator_descriptor.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateGeneratorDescriptor {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: TemplateGeneratorDescriptorRecordKind,
    pub template_generator_descriptor_schema_version: u32,
    pub descriptor_id: String,

    pub template_or_generator_id: String,
    pub version: String,
    pub display_name: String,

    pub provider_class: TemplateProviderClass,
    pub signature_state: DescriptorSignatureState,
    pub generation_kind: GenerationKindClass,

    pub supported_ecosystems: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_archetypes: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_parameters: Vec<DescriptorParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_hooks: Vec<DeclaredHook>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declared_validation_tasks: Vec<DeclaredValidationTask>,

    pub trust_expectation: TrustExpectationClass,
    pub egress_posture: EgressPostureClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_constraints: Vec<PolicyConstraintClass>,

    pub provenance: DescriptorProvenance,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,

    pub observed_at: String,
}

impl TemplateGeneratorDescriptor {
    /// The set of declared hook ids on this descriptor.
    pub fn declared_hook_ids(&self) -> impl Iterator<Item = &str> {
        self.declared_hooks.iter().map(|hook| hook.hook_id.as_str())
    }

    /// The set of declared validation-task ids on this descriptor.
    pub fn declared_task_ids(&self) -> impl Iterator<Item = &str> {
        self.declared_validation_tasks
            .iter()
            .map(|task| task.task_id.as_str())
    }
}

// ----------------------------------------------------------------------
// ScaffoldPlanRecord
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldPlanRecordKind {
    ScaffoldPlanRecord,
}

/// Closed generation-verb set. Create-project, generate-into-existing, and
/// update-regenerate stay distinct verbs with distinct review semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationVerb {
    CreateProject,
    GenerateIntoExisting,
    UpdateRegenerate,
}

impl GenerationVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateProject => "create_project",
            Self::GenerateIntoExisting => "generate_into_existing",
            Self::UpdateRegenerate => "update_regenerate",
        }
    }

    /// True when the verb writes into an existing project rather than a
    /// fresh target, so the surface MUST disclose the merge / overwrite
    /// boundary.
    pub const fn writes_into_existing(self) -> bool {
        matches!(self, Self::GenerateIntoExisting | Self::UpdateRegenerate)
    }
}

/// Closed scaffold-scope set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldScopeClass {
    NewProjectRoot,
    Subdirectory,
    ExistingProjectRoot,
    SingleFileSet,
}

/// Typed scaffold target. Carries an opaque path ref, never a raw path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldTarget {
    pub scope: ScaffoldScopeClass,
    pub target_path_ref: String,
    /// True when the target already contains workspace content the plan
    /// will write alongside or modify.
    pub into_existing: bool,
}

/// Closed parameter-source set. Names where a resolved parameter value came
/// from so the review sheet shows resolved parameter sources, not just
/// values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterSourceClass {
    UserInput,
    Default,
    Inferred,
    PolicyPinned,
    AiSuggested,
    ExtensionSupplied,
}

impl ParameterSourceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserInput => "user_input",
            Self::Default => "default",
            Self::Inferred => "inferred",
            Self::PolicyPinned => "policy_pinned",
            Self::AiSuggested => "ai_suggested",
            Self::ExtensionSupplied => "extension_supplied",
        }
    }
}

/// One resolved parameter on the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedParameter {
    pub parameter_key: String,
    pub source_class: ParameterSourceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_label: Option<String>,
}

/// File / directory impact summary the preflight discloses before any
/// write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileImpactSummary {
    pub create_count: u32,
    pub modify_count: u32,
    pub delete_count: u32,
    pub directory_count: u32,
}

impl FileImpactSummary {
    /// True when the plan would write at least one file (create or modify).
    pub const fn writes_files(self) -> bool {
        self.create_count > 0 || self.modify_count > 0
    }
}

/// Closed dependency-action class for a dependency-plan entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyActionClass {
    AddManifestEntryOnly,
    RestoreNow,
    RestoreDeferred,
    PinLockfile,
    UpgradeExisting,
}

impl DependencyActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AddManifestEntryOnly => "add_manifest_entry_only",
            Self::RestoreNow => "restore_now",
            Self::RestoreDeferred => "restore_deferred",
            Self::PinLockfile => "pin_lockfile",
            Self::UpgradeExisting => "upgrade_existing",
        }
    }

    /// True when the action restores packages (reaches a registry / network)
    /// rather than only editing a manifest the user can review.
    pub const fn restores_packages(self) -> bool {
        matches!(self, Self::RestoreNow | Self::RestoreDeferred)
    }
}

/// One dependency-plan entry. Names the package action and its registry
/// class without raw commands or URLs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyPlanEntry {
    pub summary: String,
    pub action_class: DependencyActionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry_class: Option<String>,
}

/// Closed scaffold-task execution class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldTaskExecutionClass {
    BrowseSafe,
    LocalSideEffect,
    NetworkRequired,
    Privileged,
    Deferred,
}

impl ScaffoldTaskExecutionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowseSafe => "browse_safe",
            Self::LocalSideEffect => "local_side_effect",
            Self::NetworkRequired => "network_required",
            Self::Privileged => "privileged",
            Self::Deferred => "deferred",
        }
    }
}

/// One planned setup / validation task on the plan. A task that claims to
/// be declared in the descriptor must actually appear there; AI / extension
/// generation cannot smuggle a hidden bootstrap step through the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskPlanEntry {
    pub task_id: String,
    pub execution_class: ScaffoldTaskExecutionClass,
    /// True when this task corresponds to a `declared_validation_tasks` /
    /// `declared_hooks` entry on the descriptor.
    pub declared_in_descriptor: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Closed remote / bootstrap implication class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteImplicationClass {
    NetworkFetch,
    RegistryAccess,
    RemoteImagePull,
    DevcontainerBootstrap,
    PrebuildAttach,
    CredentialProvisioning,
    RemoteWorkspaceCreate,
}

impl RemoteImplicationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkFetch => "network_fetch",
            Self::RegistryAccess => "registry_access",
            Self::RemoteImagePull => "remote_image_pull",
            Self::DevcontainerBootstrap => "devcontainer_bootstrap",
            Self::PrebuildAttach => "prebuild_attach",
            Self::CredentialProvisioning => "credential_provisioning",
            Self::RemoteWorkspaceCreate => "remote_workspace_create",
        }
    }
}

/// One remote / bootstrap implication the plan declares before execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteImplication {
    pub class: RemoteImplicationClass,
    /// True when the implication is declared before execution. A plan with
    /// an undeclared implication is non-conforming.
    pub declared_before_execution: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Closed side-effect class. The five side-effect families the spec
/// requires to be declared before execution: hook, network, registry,
/// remote-image, and dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldSideEffectClass {
    Hook,
    Network,
    Registry,
    RemoteImage,
    Dependency,
}

impl ScaffoldSideEffectClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hook => "hook",
            Self::Network => "network",
            Self::Registry => "registry",
            Self::RemoteImage => "remote_image",
            Self::Dependency => "dependency",
        }
    }
}

/// One side-effect declaration on the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectDeclaration {
    pub class: ScaffoldSideEffectClass,
    /// True when the side effect is declared before execution. A plan that
    /// would run a side effect without declaring it first is non-conforming.
    pub declared_before_execution: bool,
    /// True when the side effect remains attributable to this plan after
    /// failure or rollback (it is bound to the checkpoint / lineage).
    pub attributable_after_rollback: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Closed setup-choice class. The explicit create-empty / set-up-later /
/// full-scaffold handoff choices the plan exposes where the source artifact
/// supports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupChoiceClass {
    CreateEmpty,
    SetUpLater,
    FullScaffold,
    ScaffoldWithoutDependencyRestore,
}

impl SetupChoiceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateEmpty => "create_empty",
            Self::SetUpLater => "set_up_later",
            Self::FullScaffold => "full_scaffold",
            Self::ScaffoldWithoutDependencyRestore => "scaffold_without_dependency_restore",
        }
    }
}

/// Closed rollback-boundary class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackBoundaryClass {
    Checkpoint,
    DeleteGeneratedFiles,
    GitInitialCommit,
    ManualOnly,
}

impl RollbackBoundaryClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Checkpoint => "checkpoint",
            Self::DeleteGeneratedFiles => "delete_generated_files",
            Self::GitInitialCommit => "git_initial_commit",
            Self::ManualOnly => "manual_only",
        }
    }

    /// True when the boundary is automatic (the user can undo without
    /// hand-deleting files).
    pub const fn is_automatic(self) -> bool {
        matches!(
            self,
            Self::Checkpoint | Self::DeleteGeneratedFiles | Self::GitInitialCommit
        )
    }
}

/// Typed rollback boundary the plan plants before any write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackBoundary {
    pub class: RollbackBoundaryClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Review-state block. Guards that no write happens before review and that
/// an export-safe preflight summary is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldReviewState {
    /// True when the plan guarantees no file is written before the user can
    /// review the plan or its exported preflight summary.
    pub no_writes_before_review: bool,
    /// True when the preflight summary can be exported before apply.
    pub preflight_export_available: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preflight_export_ref: Option<String>,
}

/// One scaffold-plan record. Mirrors `scaffold_plan.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlanRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: ScaffoldPlanRecordKind,
    pub scaffold_plan_schema_version: u32,
    pub scaffold_plan_id: String,

    pub descriptor_ref: String,

    pub generation_verb: GenerationVerb,
    pub target: ScaffoldTarget,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolved_parameters: Vec<ResolvedParameter>,
    pub file_impact: FileImpactSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_plan: Vec<DependencyPlanEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub task_plan: Vec<TaskPlanEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remote_bootstrap_implications: Vec<RemoteImplication>,
    pub side_effect_declarations: Vec<SideEffectDeclaration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub setup_choices: Vec<SetupChoiceClass>,

    pub rollback_boundary: RollbackBoundary,
    pub review_state: ScaffoldReviewState,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,

    pub emitted_at: String,
}

// ----------------------------------------------------------------------
// ScaffoldRunRecord
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldRunRecordKind {
    ScaffoldRunRecord,
}

/// Closed actor class for the actor that dispatched the run. AI assistant
/// and extension are first-class actors that reuse the governed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldActorClass {
    User,
    AiAssistant,
    Extension,
    Automation,
    SupportReplay,
}

impl ScaffoldActorClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::AiAssistant => "ai_assistant",
            Self::Extension => "extension",
            Self::Automation => "automation",
            Self::SupportReplay => "support_replay",
        }
    }

    /// True for the AI / extension actors whose runs MUST stay inside the
    /// governed scaffold-plan / diff-review surface.
    pub const fn is_ai_or_extension(self) -> bool {
        matches!(self, Self::AiAssistant | Self::Extension)
    }
}

/// Typed actor lineage for a scaffold run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldActor {
    pub class: ScaffoldActorClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ref: Option<String>,
}

/// Closed run-outcome class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldOutcomeClass {
    Succeeded,
    PartiallyApplied,
    FailedRolledBack,
    FailedLeftInPlace,
    Cancelled,
}

impl ScaffoldOutcomeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::PartiallyApplied => "partially_applied",
            Self::FailedRolledBack => "failed_rolled_back",
            Self::FailedLeftInPlace => "failed_left_in_place",
            Self::Cancelled => "cancelled",
        }
    }

    /// True when the run failed in a way that left partial artifacts the
    /// user must be able to attribute and clean up.
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::FailedRolledBack | Self::FailedLeftInPlace)
    }
}

/// Closed rollback-state class for the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackStateClass {
    NotNeeded,
    Available,
    Performed,
    UnavailableManual,
}

impl RollbackStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNeeded => "not_needed",
            Self::Available => "available",
            Self::Performed => "performed",
            Self::UnavailableManual => "unavailable_manual",
        }
    }
}

/// Typed rollback state carried by the run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldRollbackState {
    pub class: RollbackStateClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_ref: Option<String>,
}

/// One scaffold-run record. Mirrors `scaffold_run.schema.json`. The run
/// keeps generated-project updates replay-safe for support and migration
/// flows: it cites the plan and descriptor, lists the created / modified
/// artifacts, names the invoked declared hooks / tasks, and binds the
/// checkpoint and generated-lineage refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldRunRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: ScaffoldRunRecordKind,
    pub scaffold_run_schema_version: u32,
    pub scaffold_run_id: String,

    pub scaffold_plan_ref: String,
    pub descriptor_ref: String,

    pub actor: ScaffoldActor,
    pub outcome_class: ScaffoldOutcomeClass,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_artifact_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified_artifact_refs: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invoked_declared_hook_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invoked_declared_task_ids: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    pub rollback_state: ScaffoldRollbackState,

    /// True when no write happened before the user reviewed the plan.
    pub no_writes_before_review: bool,
    /// True when undeclared hooks / bootstrap steps were blocked by
    /// contract during the run.
    pub undeclared_actions_blocked: bool,

    /// Opaque ref to the plain-file generated-project lineage metadata.
    pub generated_lineage_ref: String,
    /// True when a plain workspace file remains the authoritative lineage
    /// record (no hidden project database).
    pub plain_file_authority: bool,
    /// True when no hidden project database is the authoritative result of
    /// generation.
    pub no_hidden_project_database: bool,

    pub emitted_at: String,
}
