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

#![doc(html_root_url = "https://docs.rs/aureline-env/0.0.0")]

pub mod m5_env_governance;

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
