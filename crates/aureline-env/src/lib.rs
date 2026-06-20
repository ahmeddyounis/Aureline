//! Environment-capsule, template, prebuild, and runtime-materialization
//! governance for claimed M5 environment profiles.
//!
//! The M5 line already covers workflow bundles, project entry, install
//! topology, build intelligence, managed-workspace lifecycle, remote
//! boundaries, and runtime authority. What it still leaves implicit is
//! the actual *environment-definition contract*: the typed capsule a
//! template hydrates, a prebuild fingerprints, and a runtime
//! materializes. Without one governed matrix, a template, starter,
//! prebuild, devcontainer, remote container, or managed workspace can
//! imply trustworthy environment reuse while it only knows an
//! approximate or stale warm snapshot.
//!
//! This crate freezes that matrix. The single
//! [`m5_env_governance`] module models one capsule row per claimed
//! environment profile, each carrying the required
//! [`m5_env_governance::CapsuleDimension`]s — source digest, target
//! plan, toolchain plan, trust hooks, service graph, prebuild
//! fingerprint, and materialization parity — and the evidence backing
//! each. One [`m5_env_governance::certify_capsule_outcome`] engine folds
//! the per-dimension evidence into a single promotion-grade verdict, an
//! effective claim maturity, and a narrowed warm-start posture, so a
//! `stable` or `beta` claim — and any `warm_full_reuse` promise — can
//! never outrun the environment evidence behind it.
//!
//! The packet is mirrored, byte-for-byte, by the checked-in schema,
//! reviewer doc, proof packet, certification report, and fixture corpus
//! named on the module's [`m5_env_governance`] constants.
//!
//! The sibling [`capsules`] module materializes the typed
//! [`capsules::EnvironmentCapsule`] object that governance certifies: the
//! concrete environment definition a template hydrates, a prebuild
//! fingerprints, and a runtime materializes. Its
//! [`capsules::inspect_environment`] why-this-environment inspector folds
//! the capsule through the **same** narrowing engine, so desktop, CLI,
//! and support all read one explainability object instead of cloning a
//! private format.
//!
//! The [`workspace_templates`] module turns starter flows into declarative,
//! reviewable launch artifacts. A [`workspace_templates::WorkspaceTemplate`]
//! composes the **same** typed [`capsules::EnvironmentCapsule`] with
//! workflow-bundle references, certified-archetype defaults, and docs /
//! onboarding references, and its
//! [`workspace_templates::inspect_template`] inspector folds the embedded
//! capsule through the same [`capsules::inspect_environment`] path before
//! narrowing it by the composition layers, so template hydration cannot
//! fork the runtime or trust semantics from the core execution model.
//!
//! The [`prebuilds`] module makes the capsule's prebuild fingerprint
//! operational. It keys a [`prebuilds::PrebuildFingerprint`] on the six
//! inputs that decide compatibility — source/tree identity, capsule hash,
//! platform/arch, policy epoch, extension lock, and critical toolchain
//! digests — and its single [`prebuilds::evaluate_prebuild_reuse`] engine
//! turns a [`prebuilds::PrebuildSnapshot`] and the current fingerprint
//! into one explicit [`prebuilds::StartOutcome`] — warm, partially warm,
//! cold, or invalidated — with the reason and per-key evidence behind it,
//! mapping every outcome onto the same governance
//! [`m5_env_governance::WarmStartPosture`] so prebuilds stay accelerators
//! rather than authorities.

#![doc(html_root_url = "https://docs.rs/aureline-env/0.0.0")]

pub mod capsules;
pub mod m5_env_governance;
pub mod prebuilds;
pub mod workspace_templates;

pub use capsules::{
    desktop_environment_inspection, diff_capsules, export_capsule_metadata,
    headless_environment_inspection, inspect_environment, seeded_environment_capsule_fixtures,
    seeded_environment_capsules, support_environment_inspection, validate_environment_capsule,
    validate_environment_capsule_fixture, CapsuleChangeKind, CapsuleDiff, CapsuleDigest,
    CapsuleExport, CapsuleFieldChange, CapsuleIdentity, CapsuleSourceRef, CapsuleTargetClass,
    CompatibilityFingerprint, EnvVarBinding, EnvironmentCapsule, EnvironmentCapsuleFixture,
    ExportedDigest, ExportedHook, ExportedToolchain, FingerprintInput, InspectorReason,
    LifecyclePhase, MaterializationStatus, ObservabilityMetadata, RedactionClass, ServiceGraph,
    ServiceNode, ServiceRole, SourceKind, TargetPlan, TargetTransport, ToolchainComponent,
    ToolchainKind, ToolchainPlan, TrustGateState, TrustHook, WhyThisEnvironment, WorkingRootKind,
    ENVIRONMENT_CAPSULE_DIFF_RECORD_KIND, ENVIRONMENT_CAPSULE_DOC_REF,
    ENVIRONMENT_CAPSULE_EXPORT_RECORD_KIND, ENVIRONMENT_CAPSULE_FIXTURE_DIR,
    ENVIRONMENT_CAPSULE_FIXTURE_MANIFEST_REF, ENVIRONMENT_CAPSULE_FIXTURE_RECORD_KIND,
    ENVIRONMENT_CAPSULE_INSPECTION_RECORD_KIND, ENVIRONMENT_CAPSULE_PROOF_REF,
    ENVIRONMENT_CAPSULE_RECORD_KIND, ENVIRONMENT_CAPSULE_SCHEMA_REF,
    ENVIRONMENT_CAPSULE_SCHEMA_VERSION,
};

pub use m5_env_governance::{
    certify_capsule_outcome, seeded_m5_env_governance_fixtures, seeded_m5_env_governance_packet,
    validate_m5_env_governance_fixture, validate_m5_env_governance_packet, CapsuleDimension,
    CapsuleDrill, CapsuleDrillStep, CapsuleOutcome, CapsuleRow, ClaimMaturity, DimensionEvidence,
    DrillFailureClass, DrillPhase, EnvironmentProfile, EvidenceFreshnessRule, EvidenceState,
    M5EnvGovernanceFixture, M5EnvGovernancePacket, MaterializationClass, PublicationChannel,
    RowVerdict, SourceContractRefs, SurfaceBinding, ValidationReport, ValidationViolation,
    WarmStartPosture, WarmStartRule, M5_ENV_GOVERNANCE_DOC_REF, M5_ENV_GOVERNANCE_FIXTURE_DIR,
    M5_ENV_GOVERNANCE_FIXTURE_MANIFEST_REF, M5_ENV_GOVERNANCE_FIXTURE_RECORD_KIND,
    M5_ENV_GOVERNANCE_PACKET_RECORD_KIND, M5_ENV_GOVERNANCE_PACKET_REF,
    M5_ENV_GOVERNANCE_REPORT_REF, M5_ENV_GOVERNANCE_SCHEMA_REF, M5_ENV_GOVERNANCE_SCHEMA_VERSION,
};

pub use prebuilds::{
    desktop_prebuild_decision, evaluate_prebuild_reuse, export_prebuild_decision,
    headless_prebuild_decision, seeded_prebuild_fingerprint_fixtures,
    seeded_prebuild_fingerprint_packet, seeded_prebuild_snapshots, support_prebuild_decision,
    validate_prebuild_fingerprint, validate_prebuild_fingerprint_fixture,
    validate_prebuild_fingerprint_packet, validate_prebuild_snapshot, ActionGate, ActionPosture,
    ArtifactEvaluation, ArtifactIntegrity, ArtifactInvalidationRule, ArtifactLayer,
    CapsuleActionClass, CompatibilityClass, FingerprintKey, FingerprintKeyDigest, KeyEvaluation,
    KeyInvalidationRule, KeyMatch, PrebuildArtifact, PrebuildCase, PrebuildDecision, PrebuildDrill,
    PrebuildDrillStep, PrebuildExport, PrebuildFingerprint, PrebuildFingerprintFixture,
    PrebuildFingerprintPacket, PrebuildReason, PrebuildSnapshot, StartOutcome,
    PREBUILD_DECISION_RECORD_KIND, PREBUILD_EXPORT_RECORD_KIND, PREBUILD_FINGERPRINT_DOC_REF,
    PREBUILD_FINGERPRINT_FIXTURE_DIR, PREBUILD_FINGERPRINT_FIXTURE_MANIFEST_REF,
    PREBUILD_FINGERPRINT_FIXTURE_RECORD_KIND, PREBUILD_FINGERPRINT_PACKET_ID,
    PREBUILD_FINGERPRINT_PACKET_RECORD_KIND, PREBUILD_FINGERPRINT_PACKET_REF,
    PREBUILD_FINGERPRINT_PROOF_REF, PREBUILD_FINGERPRINT_SCHEMA_REF,
    PREBUILD_FINGERPRINT_SCHEMA_VERSION, PREBUILD_REUSE_DRILLS_REF, PREBUILD_SNAPSHOT_RECORD_KIND,
};

pub use workspace_templates::{
    desktop_template_inspection, diff_templates, export_template_metadata,
    headless_template_inspection, inspect_template, plan_template_change,
    seeded_workspace_template_fixtures, seeded_workspace_templates, support_template_inspection,
    validate_workspace_template, validate_workspace_template_fixture, ArchetypeDefault,
    CompositionGuardrails, CompositionLayerKind, DocsOnboardingRef, DocsRefKind, ExportedLayer,
    MirrorClass, PlannedLayer, SignerClass, SupportClass, SupportPosture, TemplateChangePlan,
    TemplateDiff, TemplateExport, TemplateIdentity, TemplateLayerReason, TemplateLifecycleOp,
    TemplateOutcome, TemplateSourceClass, TemplateTrust, WhyThisTemplate, WorkflowBundleRef,
    WorkspaceTemplate, WorkspaceTemplateFixture, WORKSPACE_TEMPLATE_DIFF_RECORD_KIND,
    WORKSPACE_TEMPLATE_DOC_REF, WORKSPACE_TEMPLATE_EXPORT_RECORD_KIND,
    WORKSPACE_TEMPLATE_FIXTURE_DIR, WORKSPACE_TEMPLATE_FIXTURE_MANIFEST_REF,
    WORKSPACE_TEMPLATE_FIXTURE_RECORD_KIND, WORKSPACE_TEMPLATE_INSPECTION_RECORD_KIND,
    WORKSPACE_TEMPLATE_PLAN_RECORD_KIND, WORKSPACE_TEMPLATE_PROOF_REF,
    WORKSPACE_TEMPLATE_RECORD_KIND, WORKSPACE_TEMPLATE_SCHEMA_REF,
    WORKSPACE_TEMPLATE_SCHEMA_VERSION,
};
