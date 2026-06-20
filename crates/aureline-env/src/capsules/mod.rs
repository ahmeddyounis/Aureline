//! The typed environment-capsule object and its why-this-environment
//! inspector.
//!
//! The sibling [`crate::m5_env_governance`] module froze the *governance
//! matrix* that certifies environment-capsule truth per claimed M5
//! profile. What it deliberately left implicit is the capsule object
//! itself: the concrete, typed environment definition a template
//! hydrates, a prebuild fingerprints, and a runtime materializes. This
//! module materializes that object.
//!
//! An [`EnvironmentCapsule`] carries every field the environment-truth
//! lane needs as inspectable, diffable, serde-serializable data:
//!
//! - a [`CapsuleIdentity`] (id, version, profile, materialization class,
//!   transport, and a versioned [`CapsuleDigest`]),
//! - typed [`CapsuleSourceRef`]s, each pinned by a digest,
//! - a [`TargetPlan`] declaring how the environment materializes,
//! - a [`ServiceGraph`] of the services it stands up,
//! - a [`ToolchainPlan`] pinning language and runtime versions,
//! - trust-gated lifecycle [`TrustHook`]s,
//! - a [`CompatibilityFingerprint`] over the inputs warm start reuses,
//! - a [`MaterializationStatus`] for cross-surface parity, and
//! - [`ObservabilityMetadata`].
//!
//! The capsule never stores secrets or raw environment bodies: lifecycle
//! commands and environment-variable values are reduced to digests, so
//! the object is metadata-first by construction.
//!
//! [`inspect_environment`] folds a capsule's own typed fields into the
//! seven governance [`CapsuleDimension`]s and runs the **same**
//! [`certify_capsule_outcome`] narrowing engine the governance matrix
//! uses, producing one [`WhyThisEnvironment`] report. Desktop
//! ([`desktop_environment_inspection`]), CLI / headless
//! ([`headless_environment_inspection`]), and support
//! ([`support_environment_inspection`]) all read that one object instead
//! of cloning a private explainability format, so a stale prebuild or an
//! ungated hook downgrades visibly and identically on every surface.
//!
//! [`diff_capsules`] compares two capsules field-by-field and
//! [`export_capsule_metadata`] projects a redaction-safe support view, so
//! the capsule can be inspected, diffed, exported, and tested across both
//! local and non-local paths.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m5_env_governance::{
    certify_capsule_outcome, CapsuleDimension, ClaimMaturity, DimensionEvidence,
    EnvironmentProfile, EvidenceState, MaterializationClass, RowVerdict, ValidationReport,
    ValidationViolation, WarmStartPosture,
};

/// Schema version stamped onto capsules, inspections, and fixtures.
pub const ENVIRONMENT_CAPSULE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by a serialized [`EnvironmentCapsule`].
pub const ENVIRONMENT_CAPSULE_RECORD_KIND: &str = "environment_capsule_record";

/// Stable record-kind tag carried by a [`WhyThisEnvironment`] inspection.
pub const ENVIRONMENT_CAPSULE_INSPECTION_RECORD_KIND: &str =
    "environment_capsule_inspection_record";

/// Stable record-kind tag carried by a [`CapsuleExport`].
pub const ENVIRONMENT_CAPSULE_EXPORT_RECORD_KIND: &str = "environment_capsule_export_record";

/// Stable record-kind tag carried by a [`CapsuleDiff`].
pub const ENVIRONMENT_CAPSULE_DIFF_RECORD_KIND: &str = "environment_capsule_diff_record";

/// Stable record-kind tag carried by an [`EnvironmentCapsuleFixture`].
pub const ENVIRONMENT_CAPSULE_FIXTURE_RECORD_KIND: &str = "environment_capsule_fixture_record";

/// Repo-relative schema ref for the capsule and its fixtures.
pub const ENVIRONMENT_CAPSULE_SCHEMA_REF: &str = "schemas/env/environment-capsule.schema.json";

/// Repo-relative reviewer doc ref.
pub const ENVIRONMENT_CAPSULE_DOC_REF: &str = "docs/env/environment-capsule.md";

/// Repo-relative human-readable proof report.
pub const ENVIRONMENT_CAPSULE_PROOF_REF: &str = "artifacts/env/environment-capsule-proof.md";

/// Repo-relative fixture directory.
pub const ENVIRONMENT_CAPSULE_FIXTURE_DIR: &str = "fixtures/env/environment-capsule";

/// Repo-relative fixture manifest.
pub const ENVIRONMENT_CAPSULE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/env/environment-capsule/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The claimed target class a capsule fixture exercises. These are the
/// main environment target classes the corpus must cover; each maps onto
/// a reused [`MaterializationClass`] and a [`TargetTransport`] rather than
/// minting a parallel target vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleTargetClass {
    /// A capsule materialized natively on the local host.
    Local,
    /// A capsule reached over SSH on a remote host.
    Ssh,
    /// A capsule materialized in a local container.
    Container,
    /// A capsule materialized from a devcontainer definition.
    Devcontainer,
    /// A capsule materialized inside a virtual machine.
    Vm,
    /// A capsule materialized in a managed cloud workspace.
    ManagedWorkspace,
}

impl CapsuleTargetClass {
    /// Every claimed target class in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Local,
        Self::Ssh,
        Self::Container,
        Self::Devcontainer,
        Self::Vm,
        Self::ManagedWorkspace,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Ssh => "ssh",
            Self::Container => "container",
            Self::Devcontainer => "devcontainer",
            Self::Vm => "vm",
            Self::ManagedWorkspace => "managed_workspace",
        }
    }
}

/// How a capsule's materialization target is reached. This refines the
/// reused [`MaterializationClass`] with the concrete transport without
/// forking the execution model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetTransport {
    /// Reached as a local process on the host.
    LocalProcess,
    /// Reached over an SSH connection to a remote host.
    Ssh,
    /// Reached inside a local container runtime.
    Container,
    /// Reached inside a virtual machine.
    VirtualMachine,
    /// Reached through a managed cloud control plane.
    CloudManaged,
}

impl TargetTransport {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProcess => "local_process",
            Self::Ssh => "ssh",
            Self::Container => "container",
            Self::VirtualMachine => "virtual_machine",
            Self::CloudManaged => "cloud_managed",
        }
    }
}

/// Where a capsule roots its working tree at materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkingRootKind {
    /// A path on the local filesystem.
    LocalPath,
    /// A container-managed volume.
    ContainerVolume,
    /// A mount on a remote host.
    RemoteMount,
    /// A managed-workspace volume.
    ManagedVolume,
}

impl WorkingRootKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPath => "local_path",
            Self::ContainerVolume => "container_volume",
            Self::RemoteMount => "remote_mount",
            Self::ManagedVolume => "managed_volume",
        }
    }
}

/// The kind of defining input a [`CapsuleSourceRef`] identifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// A workspace template the capsule hydrates from.
    WorkspaceTemplate,
    /// A dependency lockfile.
    Lockfile,
    /// A devcontainer configuration.
    DevcontainerConfig,
    /// A toolchain manifest pinning language and runtime versions.
    ToolchainManifest,
    /// A service / compose manifest defining the service graph.
    ServiceManifest,
    /// A prebuilt environment snapshot.
    PrebuildSnapshot,
}

impl SourceKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTemplate => "workspace_template",
            Self::Lockfile => "lockfile",
            Self::DevcontainerConfig => "devcontainer_config",
            Self::ToolchainManifest => "toolchain_manifest",
            Self::ServiceManifest => "service_manifest",
            Self::PrebuildSnapshot => "prebuild_snapshot",
        }
    }
}

/// The kind of a pinned toolchain component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolchainKind {
    /// A language or runtime (e.g. a compiler or interpreter).
    LanguageRuntime,
    /// A package manager.
    PackageManager,
    /// A build tool.
    BuildTool,
    /// A system-level dependency.
    SystemDependency,
}

impl ToolchainKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageRuntime => "language_runtime",
            Self::PackageManager => "package_manager",
            Self::BuildTool => "build_tool",
            Self::SystemDependency => "system_dependency",
        }
    }
}

/// The role a service plays in a capsule's service graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceRole {
    /// The primary application service.
    Primary,
    /// A backing dependency (database, cache, queue).
    Dependency,
    /// A sidecar that augments the primary service.
    Sidecar,
    /// An external endpoint the capsule depends on but does not own.
    ExternalEndpoint,
}

impl ServiceRole {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Dependency => "dependency",
            Self::Sidecar => "sidecar",
            Self::ExternalEndpoint => "external_endpoint",
        }
    }
}

/// The lifecycle phase a trust-gated hook runs in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecyclePhase {
    /// Runs once when the capsule is first created.
    OnCreate,
    /// Runs after creation completes.
    PostCreate,
    /// Runs each time the capsule starts.
    OnStart,
    /// Runs after start completes.
    PostStart,
    /// Runs when a client attaches to the capsule.
    OnAttach,
}

impl LifecyclePhase {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnCreate => "on_create",
            Self::PostCreate => "post_create",
            Self::OnStart => "on_start",
            Self::PostStart => "post_start",
            Self::OnAttach => "on_attach",
        }
    }
}

/// Whether a lifecycle hook has cleared its trust gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustGateState {
    /// The hook is bound to its authority contract and may run.
    Gated,
    /// The hook is awaiting review and must not run yet.
    PendingReview,
    /// The hook has no trust gate; running it would bypass the contract.
    Ungated,
}

impl TrustGateState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gated => "gated",
            Self::PendingReview => "pending_review",
            Self::Ungated => "ungated",
        }
    }
}

/// The redaction posture a capsule object and its projections carry. The
/// capsule is metadata-first by construction, so the only legal class is
/// [`RedactionClass::MetadataOnly`]; any other value is a contract
/// violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Only ids, digests, versions, and review-safe prose cross the
    /// boundary; never secrets or raw environment bodies.
    MetadataOnly,
}

impl RedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Typed capsule fields.
// ---------------------------------------------------------------------------

/// A content digest identifying a capsule input. The value is a digest,
/// never the body it digests, so it is safe to inspect and export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleDigest {
    /// Digest algorithm (e.g. `sha256`).
    pub algorithm: String,
    /// Lowercase hex digest value.
    pub value: String,
}

/// One typed source input that defines a capsule, pinned by a digest so
/// its contribution to capsule identity is inspectable and diffable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleSourceRef {
    /// Stable source id.
    pub source_id: String,
    /// Kind of defining input.
    pub kind: SourceKind,
    /// Repo-relative or workspace-relative reference (metadata, not a body).
    pub reference: String,
    /// Digest pinning this source.
    pub digest: CapsuleDigest,
    /// Freshness / coverage of this source's digest.
    pub coverage: EvidenceState,
    /// Review-safe summary of the source.
    pub summary: String,
}

/// The plan declaring how a capsule materializes its environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetPlan {
    /// Reused materialization class.
    pub materialization_class: MaterializationClass,
    /// Concrete transport reaching the target.
    pub transport: TargetTransport,
    /// Review-safe target label.
    pub target_label: String,
    /// Host-boundary contract this target obeys (metadata ref).
    pub host_boundary_ref: String,
    /// Where the working tree roots.
    pub working_root_kind: WorkingRootKind,
    /// Freshness / coverage of the declared target plan.
    pub coverage: EvidenceState,
    /// Review-safe summary of the target plan.
    pub summary: String,
}

/// One service node in a capsule's service graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceNode {
    /// Stable service id.
    pub service_id: String,
    /// Role this service plays.
    pub role: ServiceRole,
    /// Ports the service exposes inside the capsule.
    pub exposed_ports: Vec<u16>,
    /// Other service ids this service depends on.
    pub depends_on: Vec<String>,
    /// Review-safe summary of the service.
    pub summary: String,
}

/// The service graph a capsule materializes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceGraph {
    /// Services in the graph.
    pub services: Vec<ServiceNode>,
    /// Freshness / coverage of the declared service graph.
    pub coverage: EvidenceState,
    /// Review-safe summary of the service graph.
    pub summary: String,
}

/// One pinned toolchain component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainComponent {
    /// Stable component id.
    pub component_id: String,
    /// Kind of component.
    pub kind: ToolchainKind,
    /// Pinned version string.
    pub pinned_version: String,
    /// Source id (within the capsule) that pins this component.
    pub source_id: String,
    /// Review-safe summary of the component.
    pub summary: String,
}

/// The deterministic toolchain plan a capsule pins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainPlan {
    /// Pinned components.
    pub components: Vec<ToolchainComponent>,
    /// Freshness / coverage of the pinned toolchain plan.
    pub coverage: EvidenceState,
    /// Review-safe summary of the toolchain plan.
    pub summary: String,
}

/// One declared, trust-gated lifecycle hook. The hook's command is
/// reduced to a digest; the capsule never carries the command body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustHook {
    /// Stable hook id.
    pub hook_id: String,
    /// Lifecycle phase the hook runs in.
    pub phase: LifecyclePhase,
    /// Whether the hook has cleared its trust gate.
    pub gate_state: TrustGateState,
    /// Authority / execution-scope contract the gate binds to (metadata ref).
    pub authority_ref: String,
    /// Digest of the hook command (never the command body).
    pub command_digest: CapsuleDigest,
    /// Review-safe summary of the hook.
    pub summary: String,
}

/// One named environment-variable binding. Only the name and a digest of
/// the value are carried, so secret values never cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvVarBinding {
    /// Variable name.
    pub name: String,
    /// Digest of the value (never the value itself).
    pub value_digest: CapsuleDigest,
    /// Source id (within the capsule) that supplies the value.
    pub source_id: String,
}

/// One input to a capsule's compatibility fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FingerprintInput {
    /// Stable input id.
    pub input_id: String,
    /// Source id (within the capsule) the input draws from.
    pub source_id: String,
    /// Digest of the input.
    pub digest: CapsuleDigest,
}

/// The compatibility fingerprint warm start validates a prebuild against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFingerprint {
    /// The combined fingerprint digest.
    pub fingerprint: CapsuleDigest,
    /// Inputs that compose the fingerprint.
    pub inputs: Vec<FingerprintInput>,
    /// Freshness / coverage of the fingerprint relative to the source digest.
    pub coverage: EvidenceState,
    /// Review-safe summary of the fingerprint.
    pub summary: String,
}

/// The runtime materialization status of a capsule, used to prove that
/// the materialized environment stays aligned with the capsule object
/// across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializationStatus {
    /// Reused materialization class.
    pub materialization_class: MaterializationClass,
    /// Parity of the materialized environment with the capsule object.
    pub parity_state: EvidenceState,
    /// Surfaces the materialization is verified aligned against (metadata refs).
    pub aligned_surface_refs: Vec<String>,
    /// Review-safe summary of the materialization status.
    pub summary: String,
}

/// Observability metadata describing where a capsule's runtime signals
/// are surfaced. All fields are metadata references, never bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservabilityMetadata {
    /// Capsule lifecycle event stream reference.
    pub capsule_event_stream_ref: String,
    /// Materialization span / trace reference.
    pub materialization_span_ref: String,
    /// Health-probe references for the materialized services.
    pub health_probe_refs: Vec<String>,
    /// Redaction posture for emitted observability data.
    pub redaction_class: RedactionClass,
    /// Review-safe summary of the observability metadata.
    pub summary: String,
}

/// The identity of a capsule: a stable id, a version, the claimed
/// profile, the reused materialization class and transport, and a
/// versioned digest of its defining inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleIdentity {
    /// Stable capsule id.
    pub capsule_id: String,
    /// Monotonic capsule version.
    pub capsule_version: u32,
    /// Claimed environment profile (reused governance vocabulary).
    pub profile: EnvironmentProfile,
    /// Review-safe capsule label.
    pub label: String,
    /// Reused materialization class.
    pub materialization_class: MaterializationClass,
    /// Concrete transport reaching the capsule.
    pub transport: TargetTransport,
    /// Versioned digest of the capsule's defining inputs.
    pub capsule_digest: CapsuleDigest,
    /// Review-safe summary of the capsule identity.
    pub summary: String,
}

/// The typed environment-capsule object: one inspectable, diffable,
/// serde-serializable environment definition all M5 environment surfaces
/// point at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsule {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Capsule identity.
    pub identity: CapsuleIdentity,
    /// Typed source refs, each pinned by a digest.
    pub source_refs: Vec<CapsuleSourceRef>,
    /// Declared materialization target plan.
    pub target_plan: TargetPlan,
    /// Declared service graph.
    pub service_graph: ServiceGraph,
    /// Pinned toolchain plan.
    pub toolchain_plan: ToolchainPlan,
    /// Declared, trust-gated lifecycle hooks.
    pub trust_hooks: Vec<TrustHook>,
    /// Declared environment-variable bindings (digested, never raw values).
    pub declared_env: Vec<EnvVarBinding>,
    /// Compatibility fingerprint warm start validates against.
    pub compatibility_fingerprint: CompatibilityFingerprint,
    /// Runtime materialization parity status.
    pub materialization: MaterializationStatus,
    /// Observability metadata.
    pub observability: ObservabilityMetadata,
    /// Maturity claimed for this capsule.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed for this capsule.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Review-safe "why this environment" headline.
    pub why_this_environment: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Dimension derivation: the capsule object is the single source the engine
// reads. Each governance dimension is computed from the capsule's typed
// fields, so the inspector can never disagree with the capsule.
// ---------------------------------------------------------------------------

const BUILD_IDENTITY_REF: &str = "artifacts/build/build_identity.json";
const ARCHETYPE_CONFIDENCE_REF: &str = "artifacts/workspace/archetype_confidence_rows.yaml";
const HOST_BOUNDARY_REF: &str = "artifacts/remote/host_boundary_matrix.yaml";
const STATE_ROOT_REF: &str = "artifacts/install/state_root_matrix.yaml";
const EXECUTION_SCOPE_REF: &str = "artifacts/runtime/execution_scope_matrix.yaml";
const AUTHORITY_CLASSES_REF: &str = "artifacts/runtime/authority_classes.yaml";
const MANAGED_LIFECYCLE_REF: &str = "artifacts/runtime/managed_workspace_lifecycle.yaml";
const WARM_START_CHOOSER_REF: &str = "artifacts/entry/warm_start_chooser_contract.md";
const ENV_STARTER_SUMMARY_REF: &str = "artifacts/entry/environment_starter_summary_contract.md";

fn state_rank(state: EvidenceState) -> u8 {
    match state {
        EvidenceState::Current | EvidenceState::NotApplicable => 0,
        EvidenceState::Partial => 1,
        EvidenceState::Stale => 2,
        EvidenceState::Missing => 3,
    }
}

fn worst_state(states: impl IntoIterator<Item = EvidenceState>) -> EvidenceState {
    states
        .into_iter()
        .fold(EvidenceState::Current, |acc, state| {
            if state_rank(state) > state_rank(acc) {
                state
            } else {
                acc
            }
        })
}

fn dedup_refs(refs: impl IntoIterator<Item = String>) -> Vec<String> {
    refs.into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn dimension_contribution(dimension: CapsuleDimension) -> &'static str {
    match dimension {
        CapsuleDimension::SourceDigest => {
            "Pins capsule identity to a versioned digest of its defining inputs."
        }
        CapsuleDimension::TargetPlan => "Declares how and where the environment materializes.",
        CapsuleDimension::ToolchainPlan => "Pins the deterministic language and runtime toolchain.",
        CapsuleDimension::TrustHooks => {
            "Keeps lifecycle hooks declared and trust-gated rather than silently run."
        }
        CapsuleDimension::ServiceGraph => {
            "Declares the services, ports, and dependencies the capsule stands up."
        }
        CapsuleDimension::PrebuildFingerprint => {
            "Validates warm reuse against the source-digest fingerprint."
        }
        CapsuleDimension::MaterializationParity => {
            "Keeps the materialized environment aligned with the capsule object across surfaces."
        }
    }
}

fn dimension_rationale(dimension: CapsuleDimension) -> &'static str {
    match dimension {
        CapsuleDimension::SourceDigest => {
            "The capsule is identified by a typed, versioned digest of its source refs, so its identity is inspectable, diffable, and mirrorable rather than implied by side effects."
        }
        CapsuleDimension::TargetPlan => {
            "The capsule declares its materialization target plan and transport instead of inferring the target from whatever happened to run."
        }
        CapsuleDimension::ToolchainPlan => {
            "The capsule pins a deterministic toolchain plan of language and runtime versions, so the same capsule resolves the same toolchain across surfaces."
        }
        CapsuleDimension::TrustHooks => {
            "Lifecycle hooks are declared and trust-gated against the execution-scope and authority contracts, never silently executed during hydration or warm start."
        }
        CapsuleDimension::ServiceGraph => {
            "The capsule declares the service graph it materializes, so a partial graph is labeled partial rather than presented as the whole environment."
        }
        CapsuleDimension::PrebuildFingerprint => {
            "Prebuild reuse is validated against the source-digest fingerprint and invalidates when the fingerprint drifts, so a prebuild stays an accelerator rather than an authority."
        }
        CapsuleDimension::MaterializationParity => {
            "Runtime materialization stays semantically aligned with the same capsule object consumed by desktop, CLI, AI, support, and managed rows instead of forking a parallel model."
        }
    }
}

impl EnvironmentCapsule {
    /// Aggregate source-digest evidence state across the capsule's source
    /// refs (missing when no source refs are declared).
    fn source_digest_state(&self) -> EvidenceState {
        if self.source_refs.is_empty() {
            return EvidenceState::Missing;
        }
        worst_state(self.source_refs.iter().map(|source| source.coverage))
    }

    /// Trust-hook evidence state: an ungated hook withholds the claim, a
    /// pending hook is partial, and otherwise the hooks are current.
    fn trust_hooks_state(&self) -> EvidenceState {
        if self
            .trust_hooks
            .iter()
            .any(|hook| hook.gate_state == TrustGateState::Ungated)
        {
            EvidenceState::Missing
        } else if self
            .trust_hooks
            .iter()
            .any(|hook| hook.gate_state == TrustGateState::PendingReview)
        {
            EvidenceState::Partial
        } else {
            EvidenceState::Current
        }
    }

    fn dimension(
        &self,
        dimension: CapsuleDimension,
        state: EvidenceState,
        refs: Vec<String>,
    ) -> DimensionEvidence {
        let evidence_refs = if state == EvidenceState::Missing {
            refs
        } else if refs.is_empty() {
            // A non-missing dimension must cite at least one ref.
            vec![BUILD_IDENTITY_REF.to_owned()]
        } else {
            refs
        };
        DimensionEvidence {
            dimension,
            evidence_state: state,
            evidence_refs,
            rationale: dimension_rationale(dimension).to_owned(),
        }
    }

    /// Folds the capsule's typed fields into the seven governance
    /// dimensions the [`certify_capsule_outcome`] engine reads.
    pub fn dimension_evidence(&self) -> Vec<DimensionEvidence> {
        let source_refs = dedup_refs(
            self.source_refs
                .iter()
                .map(|source| source.reference.clone())
                .chain([ARCHETYPE_CONFIDENCE_REF.to_owned()]),
        );
        let toolchain_refs = dedup_refs([STATE_ROOT_REF.to_owned(), BUILD_IDENTITY_REF.to_owned()]);
        let trust_refs = dedup_refs(
            self.trust_hooks
                .iter()
                .map(|hook| hook.authority_ref.clone())
                .chain([
                    EXECUTION_SCOPE_REF.to_owned(),
                    AUTHORITY_CLASSES_REF.to_owned(),
                ]),
        );
        let fingerprint_refs = dedup_refs([
            WARM_START_CHOOSER_REF.to_owned(),
            BUILD_IDENTITY_REF.to_owned(),
        ]);
        let parity_refs = dedup_refs(
            self.materialization
                .aligned_surface_refs
                .iter()
                .cloned()
                .chain([ENV_STARTER_SUMMARY_REF.to_owned()]),
        );

        vec![
            self.dimension(
                CapsuleDimension::SourceDigest,
                self.source_digest_state(),
                source_refs,
            ),
            self.dimension(
                CapsuleDimension::TargetPlan,
                self.target_plan.coverage,
                dedup_refs([
                    self.target_plan.host_boundary_ref.clone(),
                    STATE_ROOT_REF.to_owned(),
                ]),
            ),
            self.dimension(
                CapsuleDimension::ToolchainPlan,
                self.toolchain_plan.coverage,
                toolchain_refs,
            ),
            self.dimension(
                CapsuleDimension::TrustHooks,
                self.trust_hooks_state(),
                trust_refs,
            ),
            self.dimension(
                CapsuleDimension::ServiceGraph,
                self.service_graph.coverage,
                dedup_refs([
                    MANAGED_LIFECYCLE_REF.to_owned(),
                    HOST_BOUNDARY_REF.to_owned(),
                ]),
            ),
            self.dimension(
                CapsuleDimension::PrebuildFingerprint,
                self.compatibility_fingerprint.coverage,
                fingerprint_refs,
            ),
            self.dimension(
                CapsuleDimension::MaterializationParity,
                self.materialization.parity_state,
                parity_refs,
            ),
        ]
    }
}

// ---------------------------------------------------------------------------
// Why-this-environment inspector.
// ---------------------------------------------------------------------------

/// One per-dimension reason line in a [`WhyThisEnvironment`] report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorReason {
    /// Capsule dimension this reason explains.
    pub dimension: CapsuleDimension,
    /// Observed evidence state for the dimension.
    pub evidence_state: EvidenceState,
    /// What this dimension contributes to the environment.
    pub contribution: String,
    /// Metadata refs backing the dimension.
    pub evidence_refs: Vec<String>,
}

/// The why-this-environment inspection: the single explainability object
/// desktop, CLI / headless, and support surfaces all consume. It folds
/// the capsule through the governance narrowing engine and reports the
/// effective maturity, verdict, warm-start posture, and the per-dimension
/// reasons behind them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhyThisEnvironment {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Capsule id under inspection.
    pub capsule_id: String,
    /// Capsule version under inspection.
    pub capsule_version: u32,
    /// Capsule digest under inspection.
    pub capsule_digest: CapsuleDigest,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Reused materialization class.
    pub materialization_class: MaterializationClass,
    /// Concrete transport.
    pub transport: TargetTransport,
    /// Maturity claimed for the capsule.
    pub claimed_maturity: ClaimMaturity,
    /// Effective maturity after narrowing.
    pub effective_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// Warm-start posture claimed for the capsule.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Effective warm-start posture after narrowing.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claim.
    pub warm_start_downgraded: bool,
    /// Tokens naming every dimension that forced maturity narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Tokens naming every warm-start-governing dimension that forced a
    /// colder posture.
    pub warm_start_downgrade_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// Per-dimension reasons behind the verdict.
    pub reasons: Vec<InspectorReason>,
    /// Review-safe headline summarizing why this environment is what it is.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// Inspects a capsule and produces the canonical why-this-environment
/// report. This is the single inspection path every surface shares; it
/// reuses [`certify_capsule_outcome`] so the inspector and the governance
/// matrix can never disagree about when a claim narrows.
pub fn inspect_environment(capsule: &EnvironmentCapsule) -> WhyThisEnvironment {
    let dimensions = capsule.dimension_evidence();
    let outcome = certify_capsule_outcome(
        capsule.claimed_maturity,
        capsule.claimed_warm_start_posture,
        &dimensions,
    );
    let reasons = dimensions
        .iter()
        .map(|evidence| InspectorReason {
            dimension: evidence.dimension,
            evidence_state: evidence.evidence_state,
            contribution: dimension_contribution(evidence.dimension).to_owned(),
            evidence_refs: evidence.evidence_refs.clone(),
        })
        .collect();
    WhyThisEnvironment {
        record_kind: ENVIRONMENT_CAPSULE_INSPECTION_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
        capsule_id: capsule.identity.capsule_id.clone(),
        capsule_version: capsule.identity.capsule_version,
        capsule_digest: capsule.identity.capsule_digest.clone(),
        profile: capsule.identity.profile,
        materialization_class: capsule.identity.materialization_class,
        transport: capsule.identity.transport,
        claimed_maturity: capsule.claimed_maturity,
        effective_maturity: outcome.effective_maturity,
        verdict: outcome.verdict,
        claimed_warm_start_posture: capsule.claimed_warm_start_posture,
        effective_warm_start_posture: outcome.effective_warm_start_posture,
        warm_start_downgraded: outcome.warm_start_downgraded,
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        stale_or_missing_dimension_tokens: outcome.stale_or_missing_dimension_tokens,
        reasons,
        headline: capsule.why_this_environment.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

/// The desktop why-this-environment inspector. Desktop reads the same
/// [`WhyThisEnvironment`] object as every other surface.
pub fn desktop_environment_inspection(capsule: &EnvironmentCapsule) -> WhyThisEnvironment {
    inspect_environment(capsule)
}

/// The headless / CLI why-this-environment inspector. Headless reads the
/// same [`WhyThisEnvironment`] object as every other surface.
pub fn headless_environment_inspection(capsule: &EnvironmentCapsule) -> WhyThisEnvironment {
    inspect_environment(capsule)
}

/// The support-path inspection: the metadata-first export wrapping the
/// same [`WhyThisEnvironment`] object support and release surfaces read.
pub fn support_environment_inspection(capsule: &EnvironmentCapsule) -> CapsuleExport {
    export_capsule_metadata(capsule)
}

// ---------------------------------------------------------------------------
// Metadata-first export.
// ---------------------------------------------------------------------------

/// One exported source digest (id plus digest, never a body).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedDigest {
    /// Source id.
    pub source_id: String,
    /// Source kind.
    pub kind: SourceKind,
    /// Pinned digest.
    pub digest: CapsuleDigest,
    /// Coverage of the source digest.
    pub coverage: EvidenceState,
}

/// One exported toolchain version (id plus version, never a body).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedToolchain {
    /// Component id.
    pub component_id: String,
    /// Component kind.
    pub kind: ToolchainKind,
    /// Pinned version.
    pub pinned_version: String,
}

/// One exported trust-hook state (id, phase, gate — never the command).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedHook {
    /// Hook id.
    pub hook_id: String,
    /// Lifecycle phase.
    pub phase: LifecyclePhase,
    /// Trust-gate state.
    pub gate_state: TrustGateState,
}

/// A metadata-first export of a capsule for support and release surfaces.
/// It wraps the canonical [`WhyThisEnvironment`] inspection and projects
/// only ids, digests, versions, and gate states — never secrets, raw
/// environment bodies, hook commands, or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Capsule id.
    pub capsule_id: String,
    /// Capsule digest.
    pub capsule_digest: CapsuleDigest,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
    /// The canonical inspection this export wraps.
    pub inspection: WhyThisEnvironment,
    /// Exported source digests.
    pub source_digests: Vec<ExportedDigest>,
    /// Exported toolchain versions.
    pub toolchain_versions: Vec<ExportedToolchain>,
    /// Exported trust-hook states.
    pub trust_hook_states: Vec<ExportedHook>,
    /// Exported environment-variable names (names only).
    pub declared_env_names: Vec<String>,
    /// Review-safe summary of the export.
    pub summary: String,
}

/// Projects a redaction-safe, metadata-first export of a capsule.
pub fn export_capsule_metadata(capsule: &EnvironmentCapsule) -> CapsuleExport {
    let inspection = inspect_environment(capsule);
    CapsuleExport {
        record_kind: ENVIRONMENT_CAPSULE_EXPORT_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
        capsule_id: capsule.identity.capsule_id.clone(),
        capsule_digest: capsule.identity.capsule_digest.clone(),
        redaction_class: RedactionClass::MetadataOnly,
        inspection,
        source_digests: capsule
            .source_refs
            .iter()
            .map(|source| ExportedDigest {
                source_id: source.source_id.clone(),
                kind: source.kind,
                digest: source.digest.clone(),
                coverage: source.coverage,
            })
            .collect(),
        toolchain_versions: capsule
            .toolchain_plan
            .components
            .iter()
            .map(|component| ExportedToolchain {
                component_id: component.component_id.clone(),
                kind: component.kind,
                pinned_version: component.pinned_version.clone(),
            })
            .collect(),
        trust_hook_states: capsule
            .trust_hooks
            .iter()
            .map(|hook| ExportedHook {
                hook_id: hook.hook_id.clone(),
                phase: hook.phase,
                gate_state: hook.gate_state,
            })
            .collect(),
        declared_env_names: capsule
            .declared_env
            .iter()
            .map(|binding| binding.name.clone())
            .collect(),
        summary: format!(
            "Metadata-first export of capsule {} ({}); no secrets, raw env bodies, or hook commands cross the boundary.",
            capsule.identity.capsule_id,
            capsule.identity.profile.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Diff.
// ---------------------------------------------------------------------------

/// How a field changed between two capsules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleChangeKind {
    /// The field exists only in the target capsule.
    Added,
    /// The field exists only in the base capsule.
    Removed,
    /// The field changed value between the two capsules.
    Changed,
}

impl CapsuleChangeKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

/// One field-level change between two capsules. Values are metadata
/// tokens (ids, digests, versions, states), never secrets or bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleFieldChange {
    /// Dotted path naming the changed field.
    pub path: String,
    /// Kind of change.
    pub change_kind: CapsuleChangeKind,
    /// Metadata token before the change (empty for additions).
    pub before: String,
    /// Metadata token after the change (empty for removals).
    pub after: String,
}

/// The diff between two capsules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleDiff {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Base capsule id.
    pub base_capsule_id: String,
    /// Target capsule id.
    pub target_capsule_id: String,
    /// True when the two capsules are field-identical.
    pub identical: bool,
    /// Ordered field-level changes.
    pub changes: Vec<CapsuleFieldChange>,
    /// Review-safe summary of the diff.
    pub summary: String,
}

fn push_change(changes: &mut Vec<CapsuleFieldChange>, path: &str, before: String, after: String) {
    if before != after {
        changes.push(CapsuleFieldChange {
            path: path.to_owned(),
            change_kind: CapsuleChangeKind::Changed,
            before,
            after,
        });
    }
}

/// Diffs two capsules field-by-field, surfacing identity, maturity,
/// posture, source-digest, toolchain, transport, trust-hook, and
/// fingerprint changes as metadata tokens.
pub fn diff_capsules(base: &EnvironmentCapsule, target: &EnvironmentCapsule) -> CapsuleDiff {
    let mut changes = Vec::new();

    push_change(
        &mut changes,
        "identity.capsule_version",
        base.identity.capsule_version.to_string(),
        target.identity.capsule_version.to_string(),
    );
    push_change(
        &mut changes,
        "identity.capsule_digest",
        base.identity.capsule_digest.value.clone(),
        target.identity.capsule_digest.value.clone(),
    );
    push_change(
        &mut changes,
        "identity.transport",
        base.identity.transport.as_str().to_owned(),
        target.identity.transport.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "claimed_maturity",
        base.claimed_maturity.as_str().to_owned(),
        target.claimed_maturity.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "claimed_warm_start_posture",
        base.claimed_warm_start_posture.as_str().to_owned(),
        target.claimed_warm_start_posture.as_str().to_owned(),
    );
    push_change(
        &mut changes,
        "compatibility_fingerprint.fingerprint",
        base.compatibility_fingerprint.fingerprint.value.clone(),
        target.compatibility_fingerprint.fingerprint.value.clone(),
    );
    push_change(
        &mut changes,
        "compatibility_fingerprint.coverage",
        base.compatibility_fingerprint.coverage.as_str().to_owned(),
        target
            .compatibility_fingerprint
            .coverage
            .as_str()
            .to_owned(),
    );

    diff_source_refs(&mut changes, base, target);
    diff_toolchain(&mut changes, base, target);
    diff_trust_hooks(&mut changes, base, target);

    let identical = changes.is_empty();
    let summary = if identical {
        format!(
            "Capsules {} and {} are field-identical.",
            base.identity.capsule_id, target.identity.capsule_id
        )
    } else {
        format!(
            "{} field change(s) between capsules {} and {}.",
            changes.len(),
            base.identity.capsule_id,
            target.identity.capsule_id
        )
    };

    CapsuleDiff {
        record_kind: ENVIRONMENT_CAPSULE_DIFF_RECORD_KIND.to_owned(),
        schema_version: ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
        base_capsule_id: base.identity.capsule_id.clone(),
        target_capsule_id: target.identity.capsule_id.clone(),
        identical,
        changes,
        summary,
    }
}

fn diff_source_refs(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &EnvironmentCapsule,
    target: &EnvironmentCapsule,
) {
    let base_ids: BTreeSet<&str> = base
        .source_refs
        .iter()
        .map(|s| s.source_id.as_str())
        .collect();
    let target_ids: BTreeSet<&str> = target
        .source_refs
        .iter()
        .map(|s| s.source_id.as_str())
        .collect();
    for source in &base.source_refs {
        match target
            .source_refs
            .iter()
            .find(|other| other.source_id == source.source_id)
        {
            Some(other) => push_change(
                changes,
                &format!("source_refs.{}.digest", source.source_id),
                source.digest.value.clone(),
                other.digest.value.clone(),
            ),
            None => changes.push(CapsuleFieldChange {
                path: format!("source_refs.{}", source.source_id),
                change_kind: CapsuleChangeKind::Removed,
                before: source.digest.value.clone(),
                after: String::new(),
            }),
        }
    }
    for source in &target.source_refs {
        if !base_ids.contains(source.source_id.as_str()) {
            changes.push(CapsuleFieldChange {
                path: format!("source_refs.{}", source.source_id),
                change_kind: CapsuleChangeKind::Added,
                before: String::new(),
                after: source.digest.value.clone(),
            });
        }
    }
    let _ = target_ids;
}

fn diff_toolchain(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &EnvironmentCapsule,
    target: &EnvironmentCapsule,
) {
    for component in &base.toolchain_plan.components {
        if let Some(other) = target
            .toolchain_plan
            .components
            .iter()
            .find(|c| c.component_id == component.component_id)
        {
            push_change(
                changes,
                &format!("toolchain_plan.{}.pinned_version", component.component_id),
                component.pinned_version.clone(),
                other.pinned_version.clone(),
            );
        }
    }
}

fn diff_trust_hooks(
    changes: &mut Vec<CapsuleFieldChange>,
    base: &EnvironmentCapsule,
    target: &EnvironmentCapsule,
) {
    for hook in &base.trust_hooks {
        if let Some(other) = target
            .trust_hooks
            .iter()
            .find(|h| h.hook_id == hook.hook_id)
        {
            push_change(
                changes,
                &format!("trust_hooks.{}.gate_state", hook.hook_id),
                hook.gate_state.as_str().to_owned(),
                other.gate_state.as_str().to_owned(),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Fixture record.
// ---------------------------------------------------------------------------

/// One checked-in fixture: a capsule of a given target class plus the
/// inspection outcome the engine must reach for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Target class the fixture exercises.
    pub target_class: CapsuleTargetClass,
    /// The capsule under test.
    pub capsule: EnvironmentCapsule,
    /// Expected engine verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_effective_maturity: ClaimMaturity,
    /// Expected effective warm-start posture.
    pub expected_warm_start_posture: WarmStartPosture,
    /// Expected maturity-narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// Expected warm-start-downgrade tokens.
    pub expected_warm_start_downgrade_tokens: Vec<String>,
    /// One consumer surface that ingests this capsule.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

fn violation(report: &mut ValidationReport, check_id: &'static str, message: impl Into<String>) {
    report.violations.push(ValidationViolation {
        check_id,
        message: message.into(),
    });
}

fn validate_digest(report: &mut ValidationReport, owner: &str, digest: &CapsuleDigest) {
    if digest.algorithm.trim().is_empty() {
        violation(
            report,
            "capsule.digest_algorithm",
            format!("{owner} digest must name an algorithm"),
        );
    }
    let value_ok = digest.value.len() == 64 && digest.value.chars().all(|c| c.is_ascii_hexdigit());
    if !value_ok {
        violation(
            report,
            "capsule.digest_value",
            format!("{owner} digest value must be a 64-char hex string"),
        );
    }
}

/// Validates a checked-in capsule object against the frozen contract.
pub fn validate_environment_capsule(capsule: &EnvironmentCapsule) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if capsule.record_kind != ENVIRONMENT_CAPSULE_RECORD_KIND {
        violation(
            &mut report,
            "capsule.record_kind",
            "capsule record_kind does not match the frozen token",
        );
    }
    if capsule.schema_version != ENVIRONMENT_CAPSULE_SCHEMA_VERSION {
        violation(
            &mut report,
            "capsule.schema_version",
            "capsule schema_version must be 1",
        );
    }
    if capsule.identity.capsule_id.trim().is_empty() {
        violation(&mut report, "capsule.id", "capsule must carry a stable id");
    }
    if capsule.identity.capsule_version == 0 {
        violation(
            &mut report,
            "capsule.version",
            "capsule version must be at least 1",
        );
    }
    if capsule.identity.label.trim().is_empty() {
        violation(&mut report, "capsule.label", "capsule must carry a label");
    }
    validate_digest(
        &mut report,
        "capsule.identity",
        &capsule.identity.capsule_digest,
    );

    // Identity must agree with the target plan and materialization status.
    if capsule.identity.materialization_class != capsule.target_plan.materialization_class {
        violation(
            &mut report,
            "capsule.materialization_class_agrees",
            "identity materialization class must match the target plan",
        );
    }
    if capsule.identity.materialization_class != capsule.materialization.materialization_class {
        violation(
            &mut report,
            "capsule.materialization_status_agrees",
            "identity materialization class must match the materialization status",
        );
    }
    if capsule.identity.transport != capsule.target_plan.transport {
        violation(
            &mut report,
            "capsule.transport_agrees",
            "identity transport must match the target plan transport",
        );
    }

    if capsule.source_refs.is_empty() {
        violation(
            &mut report,
            "capsule.source_refs",
            "capsule must declare at least one source ref",
        );
    }
    let mut source_ids = BTreeSet::new();
    for source in &capsule.source_refs {
        if source.source_id.trim().is_empty() {
            violation(
                &mut report,
                "capsule.source_id",
                "capsule source ref must carry an id",
            );
        } else if !source_ids.insert(source.source_id.as_str()) {
            violation(
                &mut report,
                "capsule.source_id_unique",
                format!("capsule repeats source id {}", source.source_id),
            );
        }
        if source.reference.trim().is_empty() {
            violation(
                &mut report,
                "capsule.source_reference",
                format!("source ref {} must carry a reference", source.source_id),
            );
        }
        validate_digest(
            &mut report,
            &format!("source ref {}", source.source_id),
            &source.digest,
        );
    }

    // Toolchain components must reference real source ids.
    for component in &capsule.toolchain_plan.components {
        if component.pinned_version.trim().is_empty() {
            violation(
                &mut report,
                "capsule.toolchain_version",
                format!(
                    "toolchain component {} must pin a version",
                    component.component_id
                ),
            );
        }
        if !source_ids.contains(component.source_id.as_str()) {
            violation(
                &mut report,
                "capsule.toolchain_source",
                format!(
                    "toolchain component {} references unknown source {}",
                    component.component_id, component.source_id
                ),
            );
        }
    }

    // Service-graph edges must reference declared services.
    let service_ids: BTreeSet<&str> = capsule
        .service_graph
        .services
        .iter()
        .map(|service| service.service_id.as_str())
        .collect();
    for service in &capsule.service_graph.services {
        for dependency in &service.depends_on {
            if !service_ids.contains(dependency.as_str()) {
                violation(
                    &mut report,
                    "capsule.service_edge",
                    format!(
                        "service {} depends on unknown service {}",
                        service.service_id, dependency
                    ),
                );
            }
        }
    }

    // Trust hooks: digests well-formed; authority ref present.
    for hook in &capsule.trust_hooks {
        if hook.authority_ref.trim().is_empty() {
            violation(
                &mut report,
                "capsule.hook_authority",
                format!("trust hook {} must cite an authority ref", hook.hook_id),
            );
        }
        validate_digest(
            &mut report,
            &format!("trust hook {}", hook.hook_id),
            &hook.command_digest,
        );
    }

    // Declared env bindings reference real source ids; digests well-formed.
    for binding in &capsule.declared_env {
        if binding.name.trim().is_empty() {
            violation(
                &mut report,
                "capsule.env_name",
                "declared env binding must carry a name",
            );
        }
        if !source_ids.contains(binding.source_id.as_str()) {
            violation(
                &mut report,
                "capsule.env_source",
                format!(
                    "declared env binding {} references unknown source {}",
                    binding.name, binding.source_id
                ),
            );
        }
        validate_digest(
            &mut report,
            &format!("env binding {}", binding.name),
            &binding.value_digest,
        );
    }

    // Fingerprint inputs reference real source ids.
    validate_digest(
        &mut report,
        "capsule.fingerprint",
        &capsule.compatibility_fingerprint.fingerprint,
    );
    for input in &capsule.compatibility_fingerprint.inputs {
        if !source_ids.contains(input.source_id.as_str()) {
            violation(
                &mut report,
                "capsule.fingerprint_source",
                format!(
                    "fingerprint input {} references unknown source {}",
                    input.input_id, input.source_id
                ),
            );
        }
        validate_digest(
            &mut report,
            &format!("fingerprint input {}", input.input_id),
            &input.digest,
        );
    }

    // Metadata-first posture: observability must be metadata-only.
    if capsule.observability.redaction_class != RedactionClass::MetadataOnly {
        violation(
            &mut report,
            "capsule.redaction_class",
            "capsule observability must declare a metadata-only redaction class",
        );
    }
    if capsule.why_this_environment.trim().is_empty() {
        violation(
            &mut report,
            "capsule.why_this_environment",
            "capsule must carry a why-this-environment headline",
        );
    }
    if capsule.notes.trim().is_empty() {
        violation(
            &mut report,
            "capsule.notes",
            "capsule must carry a reviewer note",
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates a checked-in capsule fixture: the capsule itself, and that
/// the recorded expectations equal what the inspector computes.
pub fn validate_environment_capsule_fixture(
    fixture: &EnvironmentCapsuleFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != ENVIRONMENT_CAPSULE_FIXTURE_RECORD_KIND {
        violation(
            &mut report,
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != ENVIRONMENT_CAPSULE_SCHEMA_VERSION {
        violation(
            &mut report,
            "fixture.schema_version",
            "fixture schema_version must be 1",
        );
    }
    if fixture.fixture_id.trim().is_empty() {
        violation(&mut report, "fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        violation(
            &mut report,
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        violation(
            &mut report,
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    if let Err(capsule_report) = validate_environment_capsule(&fixture.capsule) {
        for inner in capsule_report.violations {
            report.violations.push(inner);
        }
    }

    let inspection = inspect_environment(&fixture.capsule);
    if fixture.expected_verdict != inspection.verdict {
        violation(
            &mut report,
            "fixture.expected_verdict",
            format!(
                "fixture {} expected verdict {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_verdict.as_str(),
                inspection.verdict.as_str()
            ),
        );
    }
    if fixture.expected_effective_maturity != inspection.effective_maturity {
        violation(
            &mut report,
            "fixture.expected_effective_maturity",
            format!(
                "fixture {} expected maturity {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_effective_maturity.as_str(),
                inspection.effective_maturity.as_str()
            ),
        );
    }
    if fixture.expected_warm_start_posture != inspection.effective_warm_start_posture {
        violation(
            &mut report,
            "fixture.expected_warm_start_posture",
            format!(
                "fixture {} expected warm-start posture {} disagrees with the inspector ({})",
                fixture.fixture_id,
                fixture.expected_warm_start_posture.as_str(),
                inspection.effective_warm_start_posture.as_str()
            ),
        );
    }
    if fixture.expected_narrow_reason_tokens != inspection.narrow_reason_tokens {
        violation(
            &mut report,
            "fixture.expected_narrow_reason_tokens",
            format!(
                "fixture {} expected narrowing tokens disagree with the inspector",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_warm_start_downgrade_tokens != inspection.warm_start_downgrade_tokens {
        violation(
            &mut report,
            "fixture.expected_warm_start_downgrade_tokens",
            format!(
                "fixture {} expected warm-start downgrade tokens disagree with the inspector",
                fixture.fixture_id
            ),
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Seeded corpus.
// ---------------------------------------------------------------------------

mod seed;

pub use seed::{seeded_environment_capsule_fixtures, seeded_environment_capsules};

#[cfg(test)]
mod tests;
