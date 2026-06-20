//! Generated-artifact provenance, regeneration, writable-boundary, and
//! reversible-checkpoint governance for claimed M5 artifact classes.
//!
//! The M5 line already covers generated-artifact lineage surfaces,
//! template lineage, refactor transaction policy, mutation journals, and
//! restore provenance. What it still leaves implicit is the actual
//! *generated-artifact contract*: the canonical source a derived file
//! points back to, the generator that produced it, the provenance class
//! it carries, the writable-boundary policy that decides whether a direct
//! edit is safe, the regeneration route that rebuilds it, the drift state
//! that says whether the bytes still match their source, and the
//! reversible-checkpoint lineage that captured the change. M5 now spans
//! scaffolded projects, notebook outputs, preview/runtime derivatives,
//! API/request artifacts, framework codegen, AI-assisted edits, and
//! exportable support packets; without one typed generated-artifact model
//! those surfaces can each guess differently about what is authoritative,
//! what may be written directly, what must be regenerated, and what local
//! history actually captured.
//!
//! This crate freezes that matrix. The single
//! [`m5_generated_governance`] module models one artifact row per claimed
//! [`m5_generated_governance::ArtifactClass`], each carrying the required
//! [`m5_generated_governance::ProvenanceDimension`]s — canonical source,
//! generator identity, provenance class, writable boundary, regeneration
//! route, drift state, and checkpoint lineage — and the evidence backing
//! each. One [`m5_generated_governance::certify_artifact_outcome`] engine
//! folds the per-dimension evidence into a single promotion-grade verdict,
//! an effective claim maturity, **and** a narrowed
//! [`m5_generated_governance::EditPosture`], so a `stable` or `beta`
//! claim — and any `direct_edit_allowed` promise — can never outrun the
//! provenance evidence behind it. A generated artifact is never presented
//! as ordinary authoritative source merely because it looks like a file on
//! disk: when its canonical-source or writable-boundary evidence goes
//! partial, stale, or missing, the direct-edit claim narrows to a reviewed
//! override or a regenerate-only boundary instead.
//!
//! The packet is mirrored, byte-for-byte, by the checked-in schema,
//! reviewer doc, proof packet, certification report, and fixture corpus
//! named on the module's [`m5_generated_governance`] constants, so release,
//! support, docs, and help consume one source of truth instead of
//! re-describing generated-file behavior manually.
//!
//! The [`generated_timeline`] lane extends that model into reversible
//! history: each timeline entry records how a generated artifact's bytes were
//! captured — full snapshot, metadata-plus-reference, regenerated candidate,
//! or omitted — plus its redaction class and lineage links, and one engine
//! decides the restore fidelity a compare/restore/export/support flow may
//! claim. Exact generated-byte continuity is claimed only when a full,
//! unredacted snapshot was captured, so restore never implies ordinary
//! full-source history for a derived file.

#![doc(html_root_url = "https://docs.rs/aureline-generated/0.0.0")]

pub mod descriptor;
pub mod generated_timeline;
pub mod m5_generated_governance;
pub mod mutation_guardrails;
pub mod regeneration_plan;
pub mod write_boundary;

pub use descriptor::{
    derive_descriptor_presentation, descriptor_copy_line,
    seeded_generated_artifact_descriptor_fixtures, seeded_generated_artifact_descriptor_packet,
    validate_generated_artifact_descriptor_fixture, validate_generated_artifact_descriptor_packet,
    CanonicalSourceRef, CanonicalSourceState, DescriptorPresentation, DescriptorSourceContractRefs,
    DescriptorSurfaceBinding, DriftState, GeneratedArtifactDescriptor,
    GeneratedArtifactDescriptorFixture, GeneratedArtifactDescriptorPacket, GeneratorIdentity,
    GeneratorKind, IdentityFields, PresentedAuthority, SurfaceKind, SurfaceProjection,
    GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF, GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_DIR,
    GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_RECORD_KIND, GENERATED_ARTIFACT_DESCRIPTOR_PACKET_ID,
    GENERATED_ARTIFACT_DESCRIPTOR_PACKET_RECORD_KIND, GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF, GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_VERSION, IDENTITY_FIELD_NAMES,
};

pub use generated_timeline::{
    classify_generated_history, seeded_generated_timeline_fixtures,
    seeded_generated_timeline_packet, timeline_copy_line,
    validate_generated_timeline_entry_fixture, validate_generated_timeline_packet, ByteProvenance,
    CaptureMode, CompareBasis, GeneratedHistoryOutcome, GeneratedTimelineEntry,
    GeneratedTimelineEntryFixture, GeneratedTimelinePacket, RedactionClass, RestoreAvailability,
    RestoreFidelity, TimelineExportProjection, TimelineSourceContractRefs, TimelineSurface,
    TimelineSurfaceBinding, GENERATED_TIMELINE_DOC_REF, GENERATED_TIMELINE_FIXTURE_DIR,
    GENERATED_TIMELINE_FIXTURE_MANIFEST_REF, GENERATED_TIMELINE_FIXTURE_RECORD_KIND,
    GENERATED_TIMELINE_PACKET_ID, GENERATED_TIMELINE_PACKET_RECORD_KIND,
    GENERATED_TIMELINE_PACKET_REF, GENERATED_TIMELINE_REPORT_REF, GENERATED_TIMELINE_SCHEMA_REF,
    GENERATED_TIMELINE_SCHEMA_VERSION,
};

pub use m5_generated_governance::{
    certify_artifact_outcome, seeded_m5_generated_governance_fixtures,
    seeded_m5_generated_governance_packet, validate_m5_generated_governance_fixture,
    validate_m5_generated_governance_packet, ArtifactClass, ArtifactDrill, ArtifactDrillStep,
    ArtifactOutcome, ArtifactRow, AuthorityClass, ClaimMaturity, DimensionEvidence,
    DrillFailureClass, DrillPhase, EditBoundaryRule, EditPosture, EvidenceFreshnessRule,
    EvidenceState, M5GeneratedGovernanceFixture, M5GeneratedGovernancePacket, ProvenanceDimension,
    PublicationChannel, RowVerdict, SourceContractRefs, SurfaceBinding, ValidationReport,
    ValidationViolation, M5_GENERATED_GOVERNANCE_DOC_REF, M5_GENERATED_GOVERNANCE_FIXTURE_DIR,
    M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF, M5_GENERATED_GOVERNANCE_FIXTURE_RECORD_KIND,
    M5_GENERATED_GOVERNANCE_PACKET_RECORD_KIND, M5_GENERATED_GOVERNANCE_PACKET_REF,
    M5_GENERATED_GOVERNANCE_REPORT_REF, M5_GENERATED_GOVERNANCE_SCHEMA_REF,
    M5_GENERATED_GOVERNANCE_SCHEMA_VERSION,
};

// The mutation-guardrails lane defines its own `ValidationReport` and
// `ValidationViolation`; those names already resolve to the sibling governance
// lane at the crate root, so they are reached through the
// [`mutation_guardrails`] module path instead of being re-exported here. The
// reused `WriteBoundaryDecision`, `WriteBoundarySubject`, `SideEffectClass`,
// `SideEffectDisclosure`, and `RollbackCoverage` it embeds are likewise reached
// through their owning lanes.
pub use mutation_guardrails::{
    decide_mutation_guardrail, mutation_guardrails_copy_line, seeded_mutation_guardrails_fixtures,
    seeded_mutation_guardrails_packet, validate_mutation_guardrails_fixture,
    validate_mutation_guardrails_packet, ActorLineage, BoundaryDataState, GuardrailOutcome,
    MutationAttempt, MutationClass, MutationGuardrailCase, MutationGuardrailDecision,
    MutationGuardrailFixture, MutationGuardrailPacket, MutationGuardrailSourceContractRefs,
    MutationGuardrailSurface, MutationGuardrailSurfaceBinding, MutationRoute,
    MutationSafetyEnvelope, MutationSourceClass, ReversalClass, SafetyRequirement,
    MUTATION_GUARDRAILS_DOC_REF, MUTATION_GUARDRAILS_FIXTURE_DIR,
    MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF, MUTATION_GUARDRAILS_FIXTURE_RECORD_KIND,
    MUTATION_GUARDRAILS_PACKET_ID, MUTATION_GUARDRAILS_PACKET_RECORD_KIND,
    MUTATION_GUARDRAILS_PACKET_REF, MUTATION_GUARDRAILS_REPORT_REF, MUTATION_GUARDRAILS_SCHEMA_REF,
    MUTATION_GUARDRAILS_SCHEMA_VERSION,
};

// The regeneration-plan lane defines its own `RecoveryClass`, `RecoveryStep`,
// `ValidationReport`, and `ValidationViolation`; those names already resolve to
// the sibling lanes at the crate root, so they are reached through the
// [`regeneration_plan`] module path instead of being re-exported here.
pub use regeneration_plan::{
    plan_regeneration, regeneration_plan_copy_line, seeded_regeneration_plan_fixtures,
    seeded_regeneration_plan_packet, validate_regeneration_plan_fixture,
    validate_regeneration_plan_packet, PlanReadiness, PreconditionKind, PreconditionState,
    PreconditionStatus, RegenerationPlan, RegenerationPlanCase, RegenerationPlanFixture,
    RegenerationPlanPacket, RegenerationPlanSourceContractRefs, RegenerationPlanSurface,
    RegenerationPlanSurfaceBinding, RegenerationRequest, RegenerationTarget, RollbackBoundary,
    RollbackCoverage, SideEffect, SideEffectBoundary, SideEffectClass, SideEffectDisclosure,
    TargetOutcome, TargetPlan, REGENERATION_PLAN_DOC_REF, REGENERATION_PLAN_FIXTURE_DIR,
    REGENERATION_PLAN_FIXTURE_MANIFEST_REF, REGENERATION_PLAN_FIXTURE_RECORD_KIND,
    REGENERATION_PLAN_PACKET_ID, REGENERATION_PLAN_PACKET_RECORD_KIND,
    REGENERATION_PLAN_PACKET_REF, REGENERATION_PLAN_REPORT_REF, REGENERATION_PLAN_SCHEMA_REF,
    REGENERATION_PLAN_SCHEMA_VERSION,
};

pub use write_boundary::{
    decide_write_boundary, seeded_write_boundary_fixtures, seeded_write_boundary_packet,
    validate_write_boundary_fixture, validate_write_boundary_packet, write_boundary_copy_line,
    AttemptOutcome, BoundaryState, CanonicalSourceJump, CompareLeg, CompareLegKind,
    DivergedFromGenerator, LegAvailability, RecoveryClass, RecoveryStep, RegenerationAvailability,
    ThreeWayCompare, WriteBoundaryCase, WriteBoundaryDecision, WriteBoundaryFixture,
    WriteBoundaryPacket, WriteBoundarySourceContractRefs, WriteBoundarySubject,
    WriteBoundarySurface, WriteBoundarySurfaceBinding, WRITE_BOUNDARY_DOC_REF,
    WRITE_BOUNDARY_FIXTURE_DIR, WRITE_BOUNDARY_FIXTURE_MANIFEST_REF,
    WRITE_BOUNDARY_FIXTURE_RECORD_KIND, WRITE_BOUNDARY_PACKET_ID,
    WRITE_BOUNDARY_PACKET_RECORD_KIND, WRITE_BOUNDARY_PACKET_REF, WRITE_BOUNDARY_REPORT_REF,
    WRITE_BOUNDARY_SCHEMA_REF, WRITE_BOUNDARY_SCHEMA_VERSION,
};
