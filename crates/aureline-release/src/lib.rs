//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records. The correction-train module formalizes the
//! shared correction-train, hotfix, and backport packet form on top of the
//! same rollback and release-candidate refs. The stable-claim-matrix module
//! freezes the stable claim matrix, launch cutline, qualification rows, and
//! shiproom stop rules that decide which surfaces may publish as Stable. The
//! support-class-ledger module is the publication layer on top of that matrix:
//! it publishes the v1.0 support-class assignments, the certified-archetype
//! manifest, and the downgrade automation that narrows a published support
//! class when its backing thins out. The stable-qualification-matrix module
//! finalizes the per-lane qualification rows (desktop, remote/helper,
//! ecosystem, state/schema, provider, accessibility) that ground those claims
//! and, for every cross-binary or cross-service boundary, publishes the
//! mixed-version section — negotiated fields, supported skew window, upgrade and
//! rollback order, and unsupported-state behavior — that decides whether the
//! boundary may inherit a Stable mixed-version claim or is coordinated-upgrade-only.
//! The stable-claim-manifest module is the publication layer that binds all
//! three of those records together: it assigns each published subject one
//! canonical lifecycle label, names the backing claim row, qualification rows, and
//! support-class entry that label depends on, and attaches a packet-freshness SLO
//! so a subject whose proof packet has breached its SLO narrows below the launch
//! cutline automatically before publication. The stable-boundary-manifest module
//! is the deployment-boundary layer on top of that manifest: for every published
//! subject it records, across the local-OSS, self-hosted, managed, and air-gapped
//! value lines, the lifecycle label each line can carry — never wider than the
//! subject's canonical manifest label — so a value line that lacks a capability,
//! whose line evidence is incomplete, or whose proof packet aged out narrows below
//! the cutline before publication while still ingesting the one canonical label.
//! The stable-proof-index module is the requirement-facing layer that closes the
//! loop: for every launch-blocking requirement it records one row binding the
//! requirement to the proof packet that proves it, the waiver (if any) holding it
//! provisionally, and the public claim (a stable-claim-manifest entry) whose
//! lifecycle label that proof backs — never wider than the claim's canonical label
//! — so a requirement whose proof packet aged out or is missing, whose waiver
//! expired, whose requirement evidence is incomplete, or whose backing public claim
//! is itself below the cutline narrows below the launch cutline and holds
//! publication, while the launch-blocking requirement set stays fully covered.
//! The stable-version-windows module is the interface-freeze layer alongside that
//! index: for every public interface surface — a CLI command surface, a wire/state
//! schema, an API, or a manifest format — it freezes the stable version window
//! (floor, current, ceiling, compatibility posture) and the deprecation packet that
//! governs how older versions leave the window, backs each surface against a public
//! claim whose canonical label is a hard ceiling, and narrows below the cutline any
//! surface whose freeze packet aged out or is missing, whose deprecation packet is
//! incomplete or carries an overdue removal, whose waiver expired, whose surface
//! evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the CLI/schema/API/manifest surface kinds and the release-line surface
//! set both stay fully covered.
//! The maintenance-control-packet module is the post-release maintenance layer that
//! sits alongside those freezes: for every maintenance lane — an emergency hotfix lane,
//! a supported-line backport lane, a planned correction-train lane, or a support-window
//! commitment — it records one row binding the lane to the control packet that proves
//! it is staffed, the support window it commits to, and the shared correction-train
//! packet form it rides, backs each lane against a public claim whose canonical label
//! is a hard ceiling, and narrows below the cutline any lane whose control packet aged
//! out or is missing, whose support window is incomplete or has passed its
//! end-of-support date, whose waiver expired, whose lane evidence is incomplete, or
//! whose backing public claim is itself below the cutline — while the
//! hotfix/backport/correction-train/support-window lane kinds and the release-line lane
//! set both stay fully covered.
//! The shiproom-dashboard module is the consuming dashboard layer over all of the above:
//! for every shiproom panel — a claim-truth, qualification, public-proof, or maintenance
//! panel — it records one row binding the panel to the upstream source it ingests, the
//! qualification rows it watches, the freshness packet that proves it is current, and the
//! measurable fitness functions it must clear, backs each panel against a public claim
//! whose canonical label is a hard ceiling, and narrows below the cutline any panel whose
//! freshness packet aged out or is missing, whose fitness function failed or is
//! unmeasured, whose watched qualification row regressed, whose waiver expired, whose
//! panel evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the claim-truth/qualification/public-proof/maintenance panel kinds and the
//! release-line panel set both stay fully covered, so shiproom and release tooling can
//! fail promotion directly from the dashboard.
//! The optional-surface-qualification module is the claim-narrowing automation alongside all
//! of the above: where the manifest, qualification matrix, and proof index speak for surfaces
//! meant to ship at the cutline, this register governs the *optional* surfaces — opt-in
//! capabilities, optional integrations, secondary platforms, and shipped-but-experimental
//! previews — whose default is *narrowed*. For every optional surface it records one row
//! binding the surface to the public claim it backs and to its qualification packet as an
//! optional value, so a surface that lacks a stable qualification packet entirely, whose
//! packet breached its freshness SLO, whose surface evidence or capability is incomplete,
//! whose waiver expired, or whose backing public claim is itself below the cutline narrows
//! below the launch cutline and never inherits an adjacent qualified surface — while the
//! opt-in/integration/platform/preview surface kinds and the release-relevant surface set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The cohort-scoreboards module is the signoff-loop layer beside those gates: it
//! finalizes the design-partner, certified-archetype, and stable-cohort
//! scoreboards as one canonical packet, binds every scoreboard row to a public
//! claim ceiling and proof packet, and narrows any row whose packet is stale,
//! metric fails, waiver expires, or required signoff loop is incomplete before the
//! row can widen release, docs, Help/About, or support-export language.
//! The stable-publication-pack module is the outward-facing publication layer over all of
//! the above: where the manifest, proof index, version windows, and maintenance packet
//! govern what the release line *is*, this pack governs what the release line *says about
//! itself* — its known-limits publications, its public benchmark publications, its
//! compatibility publications, and its migration publications. For every such publication
//! it records one row binding the publication to the public claim it backs and to the
//! proof packet that grounds it (a known-limits register, a benchmark-lab trace, a
//! compatibility report, or a migration guide), protects each benchmark publication's
//! published p50/p95 budget against the measured numbers (with corpus metadata, lab
//! trace, and a waiver hook for intentionally tightened thresholds), and narrows below
//! the cutline any publication whose proof packet aged out or is missing, whose measured
//! numbers regressed beyond the published budget, whose corpus metadata or trace is
//! missing, whose waiver expired, whose evidence is incomplete, or whose backing public
//! claim is itself below the cutline — while the known-limit/benchmark/compatibility/
//! migration publication kinds and the release-line publication set both stay fully
//! covered, so shiproom and release tooling can fail publication directly from the pack.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod correction_train;
pub mod finalize_design_partner_certified_archetype_and_stable_cohort;
pub mod maintenance_control_packet;
pub mod optional_surface_qualification;
pub mod release_center_model;
pub mod shiproom_dashboard;
pub mod stable_boundary_manifest;
pub mod stable_claim_manifest;
pub mod stable_claim_matrix;
pub mod stable_proof_index;
pub mod stable_publication_pack;
pub mod stable_qualification_matrix;
pub mod stable_version_windows;
pub mod support_class_ledger;

pub use correction_train::{
    BackportDecision, BackportMatrixRow, CorrectionEvidence, CorrectionItem, CorrectionRisk,
    CorrectionScope, CorrectionTrainPacket, CorrectionTrainViolation, CorrectionTriage,
    PacketTemplates, ReleaseNotesRefs, SupportProjection, TargetChannelUpdate, TriageLane,
    CORRECTION_TRAIN_PACKET_RECORD_KIND, CORRECTION_TRAIN_PACKET_SCHEMA_VERSION,
    SECURITY_OR_TRUST_ISSUE_CLASSES, SHARED_PACKET_FORM_TERMS, SUPPORTED_LINE_CLASSES,
};

pub use finalize_design_partner_certified_archetype_and_stable_cohort::{
    current_cohort_scoreboards, CohortScoreboardRow, CohortScoreboards,
    CohortScoreboardsExportProjection, CohortScoreboardsExportRow, CohortScoreboardsSummary,
    CohortScoreboardsViolation, RequiredSignoff, ScoreboardAction, ScoreboardGapReason,
    ScoreboardLane, ScoreboardMetric, ScoreboardPublicationRecord, ScoreboardRule, ScoreboardState,
    SignoffLoop, COHORT_SCOREBOARDS_JSON, COHORT_SCOREBOARDS_PATH, COHORT_SCOREBOARDS_RECORD_KIND,
    COHORT_SCOREBOARDS_SCHEMA_VERSION,
};

pub use maintenance_control_packet::{
    current_maintenance_control_packet, ControlAction, ControlPublicationRecord, ControlRule,
    ControlState, GapReason as MaintenanceGapReason, LaneKind, MaintenanceControlPacket,
    MaintenanceControlPacketSummary, MaintenanceControlPacketViolation,
    MaintenanceExportProjection, MaintenanceExportRow, MaintenanceRow, SupportPosture,
    SupportWindow, MAINTENANCE_CONTROL_PACKET_JSON, MAINTENANCE_CONTROL_PACKET_PATH,
    MAINTENANCE_CONTROL_PACKET_RECORD_KIND, MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use optional_surface_qualification::{
    current_optional_surface_qualification, NarrowAction, NarrowReason, OptionalSurface,
    OptionalSurfaceKind, OptionalSurfaceQualification, OptionalSurfaceQualificationSummary,
    OptionalSurfaceQualificationViolation, SurfaceExportProjection, SurfaceExportRow,
    SurfacePublicationRecord, SurfaceState, SurfaceStopRule, OPTIONAL_SURFACE_QUALIFICATION_JSON,
    OPTIONAL_SURFACE_QUALIFICATION_PATH, OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND,
    OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use release_center_model::{
    ArtifactBundleCard, ArtifactFamilyClass, ArtifactGraphConsistency, ArtifactPayloadRefs,
    AuthSourceClass, BlastRadiusClass, BreakGlassDisclosure, BreakGlassStateClass,
    CompatibilityImpactClass, CompatibilityNote, ContinuityClass, ContinuityNote,
    DryRunAvailabilityClass, DryRunDisclosure, EvidenceFreshnessClass, EvidenceRef,
    ImmutableDigest, PromotionEventClass, PromotionReadiness, PromotionStage,
    PromotionTimelineStep, PublishTargetClass, PublishTargetDescriptor, ReleaseCandidate,
    ReleaseCenterHeadlessPlan, ReleaseCenterModelValidationReport, ReleaseCenterModelViolation,
    ReleaseCenterObjectIdentityIndex, ReleaseCenterObjectModel, ReleaseCenterSupportAuditExport,
    ReleaseCenterUiState, RollbackOrRevocationKind, RollbackOrRevocationRecord, RolloutRing,
    SemanticChangeClass, SignatureStateClass, TargetMutabilityClass, TargetVisibilityClass,
    VersionBumpProposal, RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND,
    RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION,
};

pub use shiproom_dashboard::{
    current_shiproom_dashboard, Comparator, DashboardExportProjection, DashboardExportRow,
    DashboardPanel, DashboardPublicationRecord, FitnessFunction, FitnessStatus, PanelKind,
    PanelState, QualificationStopRule, ShiproomDashboard, ShiproomDashboardSummary,
    ShiproomDashboardViolation, StopAction as DashboardStopAction, StopReason,
    SHIPROOM_DASHBOARD_JSON, SHIPROOM_DASHBOARD_PATH, SHIPROOM_DASHBOARD_RECORD_KIND,
    SHIPROOM_DASHBOARD_SCHEMA_VERSION,
};

pub use stable_boundary_manifest::{
    current_stable_boundary_manifest, BoundaryAction, BoundaryExportProjection, BoundaryExportRow,
    BoundaryPublicationRecord, BoundaryRow, BoundaryRule, BoundaryState,
    NarrowingReason as BoundaryNarrowingReason, StableBoundaryManifest,
    StableBoundaryManifestSummary, StableBoundaryManifestViolation, ValueLine, ValueLineProfile,
    ValueLineRollup, STABLE_BOUNDARY_MANIFEST_JSON, STABLE_BOUNDARY_MANIFEST_PATH,
    STABLE_BOUNDARY_MANIFEST_RECORD_KIND, STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION,
};

pub use stable_claim_manifest::{
    current_stable_claim_manifest, FreshnessSlo, FreshnessSloState, ManifestEntry,
    ManifestExportProjection, ManifestExportRow, ManifestPublicationRecord, ManifestState,
    NarrowingReason, ProofPacket, PublicationAction, PublicationRule, StableClaimManifest,
    StableClaimManifestSummary, StableClaimManifestViolation, STABLE_CLAIM_MANIFEST_JSON,
    STABLE_CLAIM_MANIFEST_PATH, STABLE_CLAIM_MANIFEST_RECORD_KIND,
    STABLE_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use stable_proof_index::{
    current_stable_proof_index, GapReason, IndexAction, ProofIndexExportProjection,
    ProofIndexExportRow, ProofPublicationRecord, ProofRow, ProofRule, ProofState, StableProofIndex,
    StableProofIndexSummary, StableProofIndexViolation, STABLE_PROOF_INDEX_JSON,
    STABLE_PROOF_INDEX_PATH, STABLE_PROOF_INDEX_RECORD_KIND, STABLE_PROOF_INDEX_SCHEMA_VERSION,
};

pub use stable_publication_pack::{
    current_stable_publication_pack, BenchmarkBudget, GapReason as PublicationGapReason,
    PackPublicationRecord, PublicationAction as PackPublicationAction, PublicationKind,
    PublicationPackExportProjection, PublicationPackExportRow, PublicationRow,
    PublicationRule as PackPublicationRule, PublicationState, StablePublicationPack,
    StablePublicationPackSummary, StablePublicationPackViolation, STABLE_PUBLICATION_PACK_JSON,
    STABLE_PUBLICATION_PACK_PATH, STABLE_PUBLICATION_PACK_RECORD_KIND,
    STABLE_PUBLICATION_PACK_SCHEMA_VERSION,
};

pub use stable_claim_matrix::{
    current_stable_claim_matrix, DowngradeReason, LaunchCutline, OwnerSignoff, PromotionDecision,
    PromotionDecisionRecord, QualificationEvidence, QualificationState, QualificationWaiver,
    ShiproomStopRule, StableClaimExportProjection, StableClaimExportRow, StableClaimLevel,
    StableClaimMatrix, StableClaimMatrixSummary, StableClaimMatrixViolation, StableClaimRow,
    StopAction, STABLE_CLAIM_MATRIX_JSON, STABLE_CLAIM_MATRIX_PATH,
    STABLE_CLAIM_MATRIX_RECORD_KIND, STABLE_CLAIM_MATRIX_SCHEMA_VERSION,
};

pub use stable_qualification_matrix::{
    current_stable_qualification_matrix, BoundaryFamily,
    DowngradeReason as QualificationDowngradeReason, DowngradeRule as QualificationDowngradeRule,
    MixedVersionPosture, MixedVersionSection, OrderRecord, OutOfWindowPosture,
    PromotionDecisionRecord as QualificationPromotionDecisionRecord, QualificationAction,
    QualificationExportProjection, QualificationExportRow, QualificationRow, QualificationRowScope,
    SkewWindow, StableQualificationMatrix, StableQualificationMatrixSummary,
    StableQualificationMatrixViolation, UnsupportedStateBehavior, STABLE_QUALIFICATION_MATRIX_JSON,
    STABLE_QUALIFICATION_MATRIX_PATH, STABLE_QUALIFICATION_MATRIX_RECORD_KIND,
    STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
};

pub use stable_version_windows::{
    current_stable_version_windows, CompatibilityPosture, DeprecationNotice, DeprecationPacket,
    DeprecationStatus, FreezePublicationRecord, FreezeRule, GapReason as VersionWindowGapReason,
    StableVersionWindows, StableVersionWindowsSummary, StableVersionWindowsViolation, SurfaceKind,
    VersionWindow, VersionWindowExportProjection, VersionWindowExportRow, WindowAction, WindowRow,
    WindowState, STABLE_VERSION_WINDOWS_JSON, STABLE_VERSION_WINDOWS_PATH,
    STABLE_VERSION_WINDOWS_RECORD_KIND, STABLE_VERSION_WINDOWS_SCHEMA_VERSION,
};

pub use support_class_ledger::{
    current_support_class_ledger, ArchetypeCertification, CertificationStatus, CertifiedArchetype,
    CertifiedCutline, DowngradeAction, DowngradeReason as LedgerDowngradeReason, DowngradeRule,
    EvidencePathClass, LedgerOwnerSignoff, LedgerState, LedgerWaiver, PublicationDecision,
    PublicationDecisionRecord as SupportPublicationDecisionRecord, SupportClass, SupportClassEntry,
    SupportClassExportProjection, SupportClassExportRow, SupportClassLedger,
    SupportClassLedgerSummary, SupportClassLedgerViolation, SupportEvidence,
    SUPPORT_CLASS_LEDGER_JSON, SUPPORT_CLASS_LEDGER_PATH, SUPPORT_CLASS_LEDGER_RECORD_KIND,
    SUPPORT_CLASS_LEDGER_SCHEMA_VERSION,
};
