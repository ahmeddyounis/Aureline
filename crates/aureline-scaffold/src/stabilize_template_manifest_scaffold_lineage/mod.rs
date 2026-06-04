//! Template manifest, scaffold preflight, run, health, and lineage truth.
//!
//! The records in this module keep scaffold-capable lanes reviewable:
//! template effects are declared before execution, scaffold plans are
//! inspectable before any write, scaffold runs remain attributable, template
//! health separates blockers from warnings and optimizations, and generated
//! projects carry enough lineage to update or rebase by three-way comparison
//! instead of blind regeneration.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`TemplateManifest`].
pub const TEMPLATE_MANIFEST_SCHEMA_VERSION: u32 = 1;
/// Schema version for [`ScaffoldPlan`].
pub const SCAFFOLD_PLAN_SCHEMA_VERSION: u32 = 1;
/// Schema version for [`ScaffoldRunRecord`].
pub const SCAFFOLD_RUN_SCHEMA_VERSION: u32 = 1;
/// Schema version for [`TemplateHealthReport`].
pub const TEMPLATE_HEALTH_REPORT_SCHEMA_VERSION: u32 = 1;
/// Schema version for [`GeneratedProjectLineageRecord`].
pub const GENERATED_PROJECT_LINEAGE_SCHEMA_VERSION: u32 = 1;
/// Record-kind discriminator for [`StableScaffoldPacket`].
pub const STABLE_SCAFFOLD_PACKET_RECORD_KIND: &str = "stable_scaffold_packet";

/// Stable support posture for a template or scaffold lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    OfficiallySupported,
    TeamManaged,
    CommunitySupported,
    Experimental,
    Unsupported,
    NarrowedBelowStable,
}

/// Template source class rendered on cards and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSourceClass {
    FirstParty,
    TeamManaged,
    Community,
    ExtensionProvided,
    LocalOnly,
    AiProposedPendingReview,
}

/// Project shape materialized by a template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateArchetype {
    WebApplication,
    BackendService,
    CliTool,
    LibraryOrSdk,
    ExtensionOrPlugin,
    MonorepoRoot,
    ModuleOrFeatureSlice,
}

/// Ecosystem a template declares support for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedEcosystem {
    RustCargo,
    NodePnpm,
    NodeNpm,
    PythonUv,
    PythonPip,
    GoModules,
    JavaGradle,
    DotnetNuget,
    Polyglot,
    PureFiles,
}

/// Runtime or platform scope a template health report can evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedPlatform {
    MacosArm64,
    MacosX86_64,
    LinuxX86_64,
    LinuxArm64,
    WindowsX86_64,
    WebBrowserRuntime,
    Devcontainer,
    ManagedWorkspace,
}

/// Parameter kind declared by a template manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterKind {
    Text,
    Identifier,
    RelativePath,
    Boolean,
    Enum,
    Number,
    SecretHandle,
}

/// Source used to resolve a scaffold parameter before preflight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterSource {
    TemplateDefault,
    UserInput,
    WorkspaceValue,
    PolicyValue,
    SecretBrokerHandle,
}

/// Secret-resolution state for a scaffold parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretResolution {
    NotSecret,
    BrokerHandleOnly,
    MissingBrokerHandle,
    RawSecretRejected,
}

/// Generated file class declared by a manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredFileClass {
    SourceFile,
    TestFile,
    ConfigFile,
    DependencyManifest,
    Lockfile,
    Documentation,
    CiWorkflow,
    LineageMetadata,
}

/// Hook class a scaffold run may invoke only when declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookClass {
    PreGenerate,
    PostGenerate,
    PostCreate,
    HealthCheck,
    ReopenPreflight,
}

/// Setup task class a scaffold run may enqueue only when declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskClass {
    PackageRestore,
    Build,
    Test,
    Lint,
    Format,
    DevcontainerBuild,
    ExtensionRecommendation,
    ManagedProvisioning,
}

/// Network and authority posture declared by a manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustEgressPosture {
    LocalNoEgress,
    PackageRegistryEgress,
    RemoteImagePull,
    ManagedServiceProvisioning,
    CredentialBrokerHandshake,
}

/// Target scope a scaffold plan may write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldScope {
    NewProject,
    NewService,
    NewModule,
    FeatureSlice,
    RepairExisting,
    CreateEmpty,
}

/// Rollback boundary visible before generation starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackBoundary {
    WorkspaceCheckpoint,
    LocalHistoryCheckpoint,
    DeleteGeneratedOutputs,
    ManualReviewOnly,
}

/// Scaffold outcome class captured after execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldOutcome {
    PreflightOnlyNoWrites,
    Applied,
    AppliedWithWarnings,
    BlockedBeforeWrite,
    RolledBackAfterPartialApply,
    FailedPartialApplyLeftForReview,
}

/// Actor class that dispatched a scaffold run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldActor {
    User,
    Cli,
    WorkflowBundle,
    Extension,
    AiProposedPendingAdmission,
}

/// Health-state freshness class for a single check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HealthFreshnessState {
    Live,
    Cached,
    PolicyEvaluated,
    Unchecked,
}

/// Health row severity rendered without collapsing to one pass/fail bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSeverity {
    BlockedPrerequisite,
    Warning,
    OptionalOptimization,
}

/// Divergence state for a generated project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceState {
    InSync,
    LocalOverrides,
    DivergedReviewRequired,
    UnlinkedByUser,
    UnknownImportedWithoutLineage,
}

/// Update or rebase compatibility for a generated project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateRebaseCompatibility {
    InSync,
    ThreeWayUpdateAvailable,
    ThreeWayRebaseRequired,
    BlockedByPolicy,
    BlockedByWorkspaceTrust,
    NoUpdatePath,
}

/// Publisher and signature identity for a template revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublisherSignature {
    /// Opaque publisher identity safe for support export.
    pub publisher_ref: String,
    /// Opaque signing identity or certificate chain reference.
    pub signature_ref: String,
    /// True when the signature was verified for the manifest revision.
    pub verified: bool,
}

/// Required parameter declaration for a template manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateParameter {
    /// Stable parameter key.
    pub parameter_id: String,
    /// Typed parameter kind.
    pub kind: ParameterKind,
    /// True when the user or policy must provide a value before apply.
    pub required: bool,
    /// True when raw values must never be persisted.
    pub secret_bearing: bool,
}

/// Declared hook or setup task a runner is allowed to execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredAction {
    /// Stable action id used by plans and runs.
    pub action_id: String,
    /// Hook class, when this action is a hook.
    pub hook_class: Option<HookClass>,
    /// Setup task class, when this action is a task.
    pub task_class: Option<TaskClass>,
    /// Network or authority posture required by this action.
    pub posture: TrustEgressPosture,
}

/// Versioned template manifest that bounds all scaffold effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateManifest {
    /// Schema version for this manifest.
    pub schema_version: u32,
    /// Stable template identity.
    pub template_id: String,
    /// Versioned template revision.
    pub template_version: String,
    /// Template source rendered on cards.
    pub source_class: TemplateSourceClass,
    /// Publisher and signature proof.
    pub publisher_signature: PublisherSignature,
    /// Project archetype produced by the template.
    pub archetype: TemplateArchetype,
    /// Supported ecosystems.
    pub supported_ecosystems: Vec<SupportedEcosystem>,
    /// Supported runtime/platform scopes.
    pub supported_platforms: Vec<SupportedPlatform>,
    /// Required scaffold parameters.
    pub required_parameters: Vec<TemplateParameter>,
    /// File classes this template may create or modify.
    pub declared_file_classes: Vec<DeclaredFileClass>,
    /// Hooks and setup tasks this template may execute.
    pub declared_actions: Vec<DeclaredAction>,
    /// Human-reviewable trust and egress note.
    pub trust_egress_notes: String,
    /// Support posture the lane may claim.
    pub support_class: SupportClass,
}

/// Resolved parameter source captured by a scaffold preflight plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedParameter {
    /// Parameter id from [`TemplateManifest::required_parameters`].
    pub parameter_id: String,
    /// Source used to resolve the parameter.
    pub source: ParameterSource,
    /// Secret-safe resolution state.
    pub secret_resolution: SecretResolution,
}

/// File and directory impact summary visible before writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileImpactSummary {
    /// Number of files planned for creation.
    pub create_count: u32,
    /// Number of files planned for modification.
    pub modify_count: u32,
    /// Number of files planned for deletion.
    pub delete_count: u32,
    /// Number of directories planned for creation.
    pub directory_create_count: u32,
}

/// Reviewable scaffold plan that must be inspectable before writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlan {
    /// Schema version for this plan.
    pub schema_version: u32,
    /// Stable plan id.
    pub plan_id: String,
    /// Manifest id this plan resolves.
    pub manifest_template_id_ref: String,
    /// Manifest version this plan resolves.
    pub manifest_version_ref: String,
    /// Opaque target identity; raw absolute paths stay outside this record.
    pub target_ref: String,
    /// Scaffold target scope.
    pub scope: ScaffoldScope,
    /// Resolved parameter sources.
    pub resolved_parameters: Vec<ResolvedParameter>,
    /// File and directory impact summary.
    pub file_impact: FileImpactSummary,
    /// Declared action ids planned for apply or optional setup.
    pub planned_action_ids: Vec<String>,
    /// Rollback boundary visible before write.
    pub rollback_boundary: RollbackBoundary,
    /// True when create-empty is offered at equal weight.
    pub create_empty_alternative: bool,
    /// True only after the plan was reviewed or exported.
    pub reviewed_or_exported_before_write: bool,
}

/// Attributable scaffold run record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldRunRecord {
    /// Schema version for this run record.
    pub schema_version: u32,
    /// Stable run id.
    pub run_id: String,
    /// Plan id this run executed.
    pub plan_id_ref: String,
    /// Manifest id this run executed.
    pub manifest_template_id_ref: String,
    /// Manifest version this run executed.
    pub manifest_version_ref: String,
    /// Workspace or workset identity.
    pub workspace_workset_ref: String,
    /// Created artifact opaque ids.
    pub created_artifact_refs: Vec<String>,
    /// Modified artifact opaque ids.
    pub modified_artifact_refs: Vec<String>,
    /// Invoked hook or setup action ids.
    pub invoked_action_ids: Vec<String>,
    /// Checkpoint ref used for rollback or recovery.
    pub checkpoint_ref: Option<String>,
    /// Run outcome.
    pub outcome: ScaffoldOutcome,
    /// Actor that dispatched the run.
    pub actor: ScaffoldActor,
}

/// Single template-health row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateHealthRow {
    /// Stable health check id.
    pub check_id: String,
    /// Human-reviewable check label.
    pub label: String,
    /// Severity partition for this row.
    pub severity: HealthSeverity,
    /// Freshness/source state.
    pub freshness_state: HealthFreshnessState,
    /// Runtime/toolchain/platform scope this row applies to.
    pub scope: Vec<SupportedPlatform>,
    /// True when this check was intentionally skipped.
    pub skipped: bool,
    /// Fix guidance that does not require hidden setup.
    pub fix_guidance: String,
}

/// Health report for a template under a stated runtime/toolchain/platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateHealthReport {
    /// Schema version for this health report.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Template id this report evaluates.
    pub manifest_template_id_ref: String,
    /// Template version this report evaluates.
    pub manifest_version_ref: String,
    /// Health rows split by severity and freshness.
    pub rows: Vec<TemplateHealthRow>,
}

/// Generated-project lineage record carried by the generated workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedProjectLineageRecord {
    /// Schema version for this lineage record.
    pub schema_version: u32,
    /// Stable lineage id.
    pub lineage_id: String,
    /// Generated project root or module identity.
    pub generated_root_ref: String,
    /// Originating scaffold run.
    pub originating_run_id_ref: String,
    /// Originating template id.
    pub manifest_template_id_ref: String,
    /// Originating template version.
    pub manifest_version_ref: String,
    /// Workspace or workset identity.
    pub workspace_workset_ref: String,
    /// Current divergence state.
    pub divergence_state: DivergenceState,
    /// Three-way update or rebase compatibility.
    pub update_rebase_compatibility: UpdateRebaseCompatibility,
    /// Latest health report used by support and migration surfaces.
    pub latest_health_report_ref: String,
    /// True when lineage metadata is a plain reviewable file.
    pub plain_reviewable_metadata: bool,
}

/// Stable packet consumed by scaffold-capable surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableScaffoldPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Template manifest.
    pub manifest: TemplateManifest,
    /// Reviewable scaffold plan.
    pub plan: ScaffoldPlan,
    /// Optional run record.
    pub run: Option<ScaffoldRunRecord>,
    /// Latest template health report.
    pub health_report: TemplateHealthReport,
    /// Optional generated-project lineage record.
    pub lineage: Option<GeneratedProjectLineageRecord>,
}

/// Validation failures that narrow a scaffold lane below stable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableScaffoldError {
    /// A schema-version field does not match this crate.
    SchemaVersionMismatch(&'static str),
    /// Packet record kind is not stable-scaffold-packet.
    RecordKindMismatch,
    /// The plan does not bind to the manifest.
    PlanManifestMismatch,
    /// The run does not bind to the plan or manifest.
    RunBindingMismatch,
    /// The health report does not bind to the manifest.
    HealthReportBindingMismatch,
    /// The lineage record does not bind to the run, manifest, workspace, or health report.
    LineageBindingMismatch,
    /// A required parameter was not resolved by the plan.
    RequiredParameterUnresolved(String),
    /// A secret-bearing parameter used a raw value or missing handle.
    RawSecretOrMissingSecretHandle(String),
    /// The plan or run referenced an undeclared hook or task.
    UndeclaredAction(String),
    /// The plan was not reviewable or exportable before write.
    WritesBeforeReview,
    /// Create-empty/create-without-starter parity is missing.
    MissingCreateEmptyAlternative,
    /// Health report rows do not contain the required severity partitions.
    HealthRowsNotPartitioned,
    /// Health report rows do not preserve live/cached/policy-evaluated/unchecked freshness states.
    HealthFreshnessNotPreserved,
    /// Generated-project update or rebase does not use three-way lineage truth.
    MissingThreeWayUpdateRebaseTruth,
    /// Generated-project lineage is hidden outside plain reviewable files.
    HiddenLineageAuthority,
}

impl std::fmt::Display for StableScaffoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(name) => write!(f, "{name} schema version is unsupported"),
            Self::RecordKindMismatch => write!(f, "stable scaffold packet record kind is invalid"),
            Self::PlanManifestMismatch => write!(f, "scaffold plan does not bind to manifest"),
            Self::RunBindingMismatch => {
                write!(f, "scaffold run does not bind to plan and manifest")
            }
            Self::HealthReportBindingMismatch => {
                write!(f, "template health report does not bind to manifest")
            }
            Self::LineageBindingMismatch => {
                write!(f, "generated-project lineage binding is invalid")
            }
            Self::RequiredParameterUnresolved(id) => {
                write!(f, "required parameter {id} is not resolved by the plan")
            }
            Self::RawSecretOrMissingSecretHandle(id) => {
                write!(f, "secret parameter {id} is not resolved by broker handle")
            }
            Self::UndeclaredAction(id) => write!(f, "action {id} is not declared by the manifest"),
            Self::WritesBeforeReview => {
                write!(f, "scaffold plan was not reviewed or exported before write")
            }
            Self::MissingCreateEmptyAlternative => write!(f, "create-empty alternative is missing"),
            Self::HealthRowsNotPartitioned => {
                write!(
                    f,
                    "template health rows do not separate blockers, warnings, and optimizations"
                )
            }
            Self::HealthFreshnessNotPreserved => {
                write!(
                    f,
                    "template health rows do not preserve all freshness states"
                )
            }
            Self::MissingThreeWayUpdateRebaseTruth => {
                write!(
                    f,
                    "generated-project update/rebase truth is not lineage-aware"
                )
            }
            Self::HiddenLineageAuthority => {
                write!(f, "lineage metadata is not a plain reviewable file")
            }
        }
    }
}

impl std::error::Error for StableScaffoldError {}

impl StableScaffoldPacket {
    /// Validates that a scaffold packet may back a stable scaffold-capable lane.
    ///
    /// # Errors
    ///
    /// Returns [`StableScaffoldError`] when the packet hides write impact,
    /// references undeclared actions, leaks secret-bearing parameter values,
    /// loses health-state granularity, or cannot prove generated-project
    /// lineage/update truth.
    pub fn validate_stable(&self) -> Result<(), StableScaffoldError> {
        self.validate_versions()?;
        if self.record_kind != STABLE_SCAFFOLD_PACKET_RECORD_KIND {
            return Err(StableScaffoldError::RecordKindMismatch);
        }
        self.validate_bindings()?;
        self.validate_parameters()?;
        self.validate_actions()?;
        self.validate_preflight_review()?;
        self.validate_health_rows()?;
        self.validate_lineage()
    }

    fn validate_versions(&self) -> Result<(), StableScaffoldError> {
        if self.manifest.schema_version != TEMPLATE_MANIFEST_SCHEMA_VERSION {
            return Err(StableScaffoldError::SchemaVersionMismatch(
                "template manifest",
            ));
        }
        if self.plan.schema_version != SCAFFOLD_PLAN_SCHEMA_VERSION {
            return Err(StableScaffoldError::SchemaVersionMismatch("scaffold plan"));
        }
        if let Some(run) = &self.run {
            if run.schema_version != SCAFFOLD_RUN_SCHEMA_VERSION {
                return Err(StableScaffoldError::SchemaVersionMismatch("scaffold run"));
            }
        }
        if self.health_report.schema_version != TEMPLATE_HEALTH_REPORT_SCHEMA_VERSION {
            return Err(StableScaffoldError::SchemaVersionMismatch(
                "template health report",
            ));
        }
        if let Some(lineage) = &self.lineage {
            if lineage.schema_version != GENERATED_PROJECT_LINEAGE_SCHEMA_VERSION {
                return Err(StableScaffoldError::SchemaVersionMismatch(
                    "generated project lineage",
                ));
            }
        }
        Ok(())
    }

    fn validate_bindings(&self) -> Result<(), StableScaffoldError> {
        if self.plan.manifest_template_id_ref != self.manifest.template_id
            || self.plan.manifest_version_ref != self.manifest.template_version
        {
            return Err(StableScaffoldError::PlanManifestMismatch);
        }
        if self.health_report.manifest_template_id_ref != self.manifest.template_id
            || self.health_report.manifest_version_ref != self.manifest.template_version
        {
            return Err(StableScaffoldError::HealthReportBindingMismatch);
        }
        if let Some(run) = &self.run {
            if run.plan_id_ref != self.plan.plan_id
                || run.manifest_template_id_ref != self.manifest.template_id
                || run.manifest_version_ref != self.manifest.template_version
            {
                return Err(StableScaffoldError::RunBindingMismatch);
            }
        }
        Ok(())
    }

    fn validate_parameters(&self) -> Result<(), StableScaffoldError> {
        for parameter in self
            .manifest
            .required_parameters
            .iter()
            .filter(|parameter| parameter.required)
        {
            let resolved = self
                .plan
                .resolved_parameters
                .iter()
                .find(|resolved| resolved.parameter_id == parameter.parameter_id)
                .ok_or_else(|| {
                    StableScaffoldError::RequiredParameterUnresolved(parameter.parameter_id.clone())
                })?;
            if parameter.secret_bearing
                && resolved.secret_resolution != SecretResolution::BrokerHandleOnly
            {
                return Err(StableScaffoldError::RawSecretOrMissingSecretHandle(
                    parameter.parameter_id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_actions(&self) -> Result<(), StableScaffoldError> {
        let declared: HashSet<&str> = self
            .manifest
            .declared_actions
            .iter()
            .map(|action| action.action_id.as_str())
            .collect();
        for action_id in &self.plan.planned_action_ids {
            if !declared.contains(action_id.as_str()) {
                return Err(StableScaffoldError::UndeclaredAction(action_id.clone()));
            }
        }
        if let Some(run) = &self.run {
            for action_id in &run.invoked_action_ids {
                if !declared.contains(action_id.as_str()) {
                    return Err(StableScaffoldError::UndeclaredAction(action_id.clone()));
                }
            }
        }
        Ok(())
    }

    fn validate_preflight_review(&self) -> Result<(), StableScaffoldError> {
        if !self.plan.reviewed_or_exported_before_write {
            return Err(StableScaffoldError::WritesBeforeReview);
        }
        if !self.plan.create_empty_alternative {
            return Err(StableScaffoldError::MissingCreateEmptyAlternative);
        }
        Ok(())
    }

    fn validate_health_rows(&self) -> Result<(), StableScaffoldError> {
        let has_blocker = self
            .health_report
            .rows
            .iter()
            .any(|row| row.severity == HealthSeverity::BlockedPrerequisite);
        let has_warning = self
            .health_report
            .rows
            .iter()
            .any(|row| row.severity == HealthSeverity::Warning);
        let has_optimization = self
            .health_report
            .rows
            .iter()
            .any(|row| row.severity == HealthSeverity::OptionalOptimization);
        if !(has_blocker && has_warning && has_optimization) {
            return Err(StableScaffoldError::HealthRowsNotPartitioned);
        }

        for freshness in [
            HealthFreshnessState::Live,
            HealthFreshnessState::Cached,
            HealthFreshnessState::PolicyEvaluated,
            HealthFreshnessState::Unchecked,
        ] {
            if !self
                .health_report
                .rows
                .iter()
                .any(|row| row.freshness_state == freshness)
            {
                return Err(StableScaffoldError::HealthFreshnessNotPreserved);
            }
        }
        Ok(())
    }

    fn validate_lineage(&self) -> Result<(), StableScaffoldError> {
        let Some(lineage) = &self.lineage else {
            return Ok(());
        };
        let Some(run) = &self.run else {
            return Err(StableScaffoldError::LineageBindingMismatch);
        };
        if lineage.originating_run_id_ref != run.run_id
            || lineage.manifest_template_id_ref != self.manifest.template_id
            || lineage.manifest_version_ref != self.manifest.template_version
            || lineage.workspace_workset_ref != run.workspace_workset_ref
            || lineage.latest_health_report_ref != self.health_report.report_id
        {
            return Err(StableScaffoldError::LineageBindingMismatch);
        }
        if !lineage.plain_reviewable_metadata {
            return Err(StableScaffoldError::HiddenLineageAuthority);
        }
        if matches!(
            lineage.update_rebase_compatibility,
            UpdateRebaseCompatibility::ThreeWayUpdateAvailable
                | UpdateRebaseCompatibility::ThreeWayRebaseRequired
                | UpdateRebaseCompatibility::InSync
        ) {
            Ok(())
        } else {
            Err(StableScaffoldError::MissingThreeWayUpdateRebaseTruth)
        }
    }
}

#[cfg(test)]
mod tests;
