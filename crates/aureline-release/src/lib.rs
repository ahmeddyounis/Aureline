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
//! The benchmark-lab-governance module is the performance-evidence layer beside those
//! gates: where the hot-path-performance-budgets register protects the published p50/p95
//! numbers for each individual hot path, this register governs the benchmark-lab
//! automation lanes, corpus governance assets, and public benchmark publication packs that
//! *produce* those numbers. For every such asset it records one row binding the asset to
//! the public claim it backs and to the proof packet that grounds it (a CI lane health
//! record, a corpus manifest, a protected-metrics revision, or a publication pack), protects
//! each benchmark publication's published p50/p95 budget against the measured numbers (with
//! corpus metadata, lab trace, and a waiver hook for intentionally tightened thresholds),
//! and narrows below the cutline any asset whose proof packet aged out or is missing, whose
//! corpus metadata or benchmark-lab trace is missing, whose waiver expired, whose evidence
//! is incomplete, or whose backing public claim is itself below the cutline — while the
//! nightly-ci/self-capture/corpus/metrics/hardware/image/ledger/publication-pack asset kinds
//! and the release-blocking asset set both stay fully covered, so shiproom and release
//! tooling can fail qualification directly from the register.
//! The cohort-scoreboards module is the signoff-loop layer beside those gates: it
//! finalizes the design-partner, certified-archetype, and stable-cohort
//! scoreboards as one canonical packet, binds every scoreboard row to a public
//! claim ceiling and proof packet, and narrows any row whose packet is stale,
//! metric fails, waiver expires, or required signoff loop is incomplete before the
//! row can widen release, docs, Help/About, or support-export language.
//! The certified-reference-workspaces module is the certification-evidence layer
//! that hardens every marketed Certified archetype: it publishes one current
//! reference-workspace report per archetype, binds each report to the archetype
//! pass-matrix row that carries it, and automates the downgrade that narrows a
//! Certified claim when its report goes stale, missing, or manually edited.
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
//! The open-paid-boundary-audit module is the governance-fact layer beside those gates:
//! where the manifest, proof index, and version windows speak for product capabilities and
//! interface surfaces, this audit governs the governance facts the stable launch rests on —
//! where the open-source core ends and the paid/managed tier begins, the licensing posture,
//! the build provenance, and the contribution policy. For every audited subject it records
//! one row binding the subject to the public claim it backs and to its attestation packet,
//! its required audit controls, and an owner sign-off, so a subject whose attestation packet
//! aged out or is missing, whose required audit control is unsatisfied, whose evidence is
//! incomplete, whose owner sign-off is missing, whose waiver expired, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent attested row — while the open-paid-boundary/licensing/provenance/contribution-
//! policy domains and the release-line audit set both stay fully covered, so shiproom and
//! release tooling can fail promotion directly from the audit.
//! The go-no-go-rehearsal module is the launch-rehearsal layer that closes the loop over all
//! of the above: where the manifest, proof index, version windows, and audit govern what the
//! release line *is*, this rehearsal governs whether the release train was actually
//! *exercised* before the go/no-go — the explicit launch cutline signed off, the promotion
//! publish step dry-run, each rollback checkpoint verified to a restore point, and every open
//! exception packet reviewed. For every rehearsal stage it records one row binding the stage
//! to the public claim it backs and to its rehearsal packet, its required rollback
//! checkpoints, an exception packet (if any) holding it provisionally, and an owner sign-off,
//! so a stage whose rehearsal packet aged out or is missing, whose rollback checkpoint is
//! unverified, whose evidence is incomplete, whose owner sign-off is missing, whose exception
//! packet expired, or whose backing public claim is itself below the cutline narrows to a
//! No-Go below the launch cutline and never inherits an adjacent rehearsed stage — while the
//! cutline-review/promotion-step/rollback-checkpoint/exception-review stage kinds and the
//! release-line rehearsal set both stay fully covered, so shiproom and release tooling can
//! fail the go/no-go directly from the rehearsal.
//! The hot-path-performance-budgets module is the performance-layer register beside those
//! gates: for every hot path — startup, restore, quick open, typing, scrolling, search, and
//! Git status — it records one row binding the path to the stable claim manifest entry whose
//! lifecycle label it backs, the benchmark budget that protects the published p50/p95 numbers,
//! the proof packet that grounds them, and the waiver (if any) holding a tightened threshold
//! provisionally, so a path whose measured numbers regressed beyond the published budget,
//! whose proof packet aged out or is missing, whose corpus metadata or benchmark-lab trace is
//! absent, whose waiver expired, whose owner sign-off is missing, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent backed budget — while the seven hot path kinds and the release-blocking path set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The accessibility-surface-signoffs module is the accessibility-layer register beside those
//! gates: for every touched surface — shell, tree, palette, diff, terminal, debugger, settings,
//! auth, and recovery — it records one row binding the surface to the stable claim manifest
//! entry whose lifecycle label it backs, the per-dimension checks that validate keyboard,
//! screen-reader, IME/grapheme/bidi, zoom, high-contrast, and reduced-motion behavior, the
//! proof packet that grounds them, and the waiver (if any) holding a provisional signoff, so
//! a surface whose dimension checks are blocked or pending, whose proof packet aged out or is
//! missing, whose owner sign-off is absent, or whose backing public claim is itself below the
//! cutline narrows below the launch cutline and never inherits an adjacent qualified surface —
//! while the nine surface kinds and the release-blocking surface set both stay fully covered,
//! so shiproom and release tooling can fail promotion directly from the register.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod correction_train;
pub mod finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack;
pub mod finalize_design_partner_certified_archetype_and_stable_cohort;
pub mod finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance;
pub mod finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills;
pub mod go_no_go_rehearsal;
pub mod harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation;
pub mod harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity;
pub mod maintenance_control_packet;
pub mod open_paid_boundary_audit;
pub mod optional_surface_qualification;
pub mod release_center_model;
pub mod shiproom_dashboard;
pub mod stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery;
pub mod stabilize_hot_path_performance_against_published_budgets_for;
pub mod stabilize_the_release_center_promotion_evidence_canary_pilot;
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

pub use finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack::{
    current_benchmark_lab_governance, AssetAction, AssetState, BenchmarkLabGovernance,
    BenchmarkLabGovernanceExportProjection, BenchmarkLabGovernanceExportRow,
    BenchmarkLabGovernanceSummary, BenchmarkLabGovernanceViolation, GovernanceAssetKind,
    GovernanceAssetRow, GovernanceRule, GapReason as BenchmarkLabGapReason, QualificationRecord,
    BENCHMARK_LAB_GOVERNANCE_JSON, BENCHMARK_LAB_GOVERNANCE_PATH,
    BENCHMARK_LAB_GOVERNANCE_RECORD_KIND, BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION,
};

pub use finalize_design_partner_certified_archetype_and_stable_cohort::{
    current_cohort_scoreboards, CohortScoreboardRow, CohortScoreboards,
    CohortScoreboardsExportProjection, CohortScoreboardsExportRow, CohortScoreboardsSummary,
    CohortScoreboardsViolation, RequiredSignoff, ScoreboardAction, ScoreboardGapReason,
    ScoreboardLane, ScoreboardMetric, ScoreboardPublicationRecord, ScoreboardRule, ScoreboardState,
    SignoffLoop, COHORT_SCOREBOARDS_JSON, COHORT_SCOREBOARDS_PATH, COHORT_SCOREBOARDS_RECORD_KIND,
    COHORT_SCOREBOARDS_SCHEMA_VERSION,
};

pub use finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance::{
    current_desktop_platform_conformance, CheckKind, CheckState, ConformanceAction, ConformanceDomain,
    ConformanceState, DesktopPlatformConformance, DesktopPlatformConformanceRule,
    DesktopPlatformConformanceRow, DesktopPlatformConformanceSummary,
    DesktopPlatformConformanceViolation, GapReason as ConformanceGapReason,
    DESKTOP_PLATFORM_CONFORMANCE_JSON, DESKTOP_PLATFORM_CONFORMANCE_PATH,
    DESKTOP_PLATFORM_CONFORMANCE_RECORD_KIND, DESKTOP_PLATFORM_CONFORMANCE_SCHEMA_VERSION,
};

pub use finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills::{
    current_security_response_packet, EmergencyControl, GapReason as ResponseGapReason,
    MirrorDrillCheckpoint, ResponseAction, ResponseExportProjection, ResponseExportRow,
    ResponseKind, ResponsePublicationRecord, ResponseRule, ResponseRow, ResponseState,
    SecurityResponsePacket, SecurityResponsePacketSummary, SecurityResponsePacketViolation,
    SECURITY_RESPONSE_PACKET_JSON, SECURITY_RESPONSE_PACKET_PATH,
    SECURITY_RESPONSE_PACKET_RECORD_KIND, SECURITY_RESPONSE_PACKET_SCHEMA_VERSION,
};

pub use go_no_go_rehearsal::{
    current_go_no_go_rehearsal, GoNoGoRehearsal, GoNoGoRehearsalSummary, GoNoGoRehearsalViolation,
    RehearsalAction, RehearsalExportProjection, RehearsalExportRow, RehearsalGapReason,
    RehearsalPublicationRecord, RehearsalRule, RehearsalStageRow, RehearsalState,
    RollbackCheckpoint, StageKind, GO_NO_GO_REHEARSAL_JSON, GO_NO_GO_REHEARSAL_PATH,
    GO_NO_GO_REHEARSAL_RECORD_KIND, GO_NO_GO_REHEARSAL_SCHEMA_VERSION,
};

pub use harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation::{
    current_certified_reference_workspaces, ArchetypePassMatrixExportRow, ArchetypePassMatrixRow,
    CertifiedReferenceWorkspaces, CertifiedReferenceWorkspacesExportProjection,
    CertifiedReferenceWorkspacesSummary, CertifiedReferenceWorkspacesViolation,
    DowngradeReason as ReferenceWorkspaceDowngradeReason,
    DowngradeRule as ReferenceWorkspaceDowngradeRule, MatrixAction, MatrixRowState,
    PublicationDecision as ReferenceWorkspacePublicationDecision,
    PublicationDecisionRecord as ReferenceWorkspacePublicationDecisionRecord,
    ReferenceWorkspaceExportRow, ReferenceWorkspaceReport, ReportState, ValidityWindow,
    CERTIFIED_REFERENCE_WORKSPACES_JSON, CERTIFIED_REFERENCE_WORKSPACES_PATH,
    CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND, CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION,
};

pub use harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity::{
    current_harden_release_artifact_graph, ArtifactFamilyAction, ArtifactFamilyExportRow,
    ArtifactFamilyGapReason, ArtifactFamilyKind, ArtifactFamilyRow, ArtifactFamilyRule,
    ArtifactFamilyState, HardenReleaseArtifactGraph, HardenReleaseArtifactGraphExportProjection,
    HardenReleaseArtifactGraphSummary, HardenReleaseArtifactGraphViolation,
    PublicationDecision as ArtifactGraphPublicationDecision,
    PublicationDecisionRecord as ArtifactGraphPublicationDecisionRecord,
    HARDEN_RELEASE_ARTIFACT_GRAPH_JSON, HARDEN_RELEASE_ARTIFACT_GRAPH_PATH,
    HARDEN_RELEASE_ARTIFACT_GRAPH_RECORD_KIND, HARDEN_RELEASE_ARTIFACT_GRAPH_SCHEMA_VERSION,
};

pub use maintenance_control_packet::{
    current_maintenance_control_packet, ControlAction, ControlPublicationRecord, ControlRule,
    ControlState, GapReason as MaintenanceGapReason, LaneKind, MaintenanceControlPacket,
    MaintenanceControlPacketSummary, MaintenanceControlPacketViolation,
    MaintenanceExportProjection, MaintenanceExportRow, MaintenanceRow, SupportPosture,
    SupportWindow, MAINTENANCE_CONTROL_PACKET_JSON, MAINTENANCE_CONTROL_PACKET_PATH,
    MAINTENANCE_CONTROL_PACKET_RECORD_KIND, MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use open_paid_boundary_audit::{
    current_open_paid_boundary_audit, AuditAction, AuditControl, AuditDomain,
    AuditExportProjection, AuditExportRow, AuditGapReason, AuditPublicationRecord, AuditRow,
    AuditRule, AuditState, OpenPaidBoundaryAudit, OpenPaidBoundaryAuditSummary,
    OpenPaidBoundaryAuditViolation, OPEN_PAID_BOUNDARY_AUDIT_JSON, OPEN_PAID_BOUNDARY_AUDIT_PATH,
    OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND, OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION,
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

pub use stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery::{
    current_accessibility_surface_signoffs, AccessibilitySurfaceSignoffExportProjection,
    AccessibilitySurfaceSignoffExportRow, AccessibilitySurfaceSignoffRule,
    AccessibilitySurfaceSignoffRow, AccessibilitySurfaceSignoffs,
    AccessibilitySurfaceSignoffsSummary, AccessibilitySurfaceSignoffsViolation,
    DimensionCheck, DimensionKind, DimensionState, GapReason as AccessibilityGapReason,
    SignoffAction, SignoffState, SurfaceKind as AccessibilitySurfaceKind,
    ACCESSIBILITY_SURFACE_SIGNOFFS_JSON, ACCESSIBILITY_SURFACE_SIGNOFFS_PATH,
    ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND, ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION,
};

pub use stabilize_hot_path_performance_against_published_budgets_for::{
    current_hot_path_performance_budgets, BudgetAction, BudgetState, GapReason as HotPathGapReason,
    HotPathBudget, HotPathBudgetRow, HotPathBudgetRule, HotPathExportProjection, HotPathExportRow,
    HotPathKind, HotPathPerformanceBudgets, HotPathPerformanceBudgetsSummary,
    HotPathPerformanceBudgetsViolation, PromotionRecord, HOT_PATH_PERFORMANCE_BUDGETS_JSON,
    HOT_PATH_PERFORMANCE_BUDGETS_PATH, HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND,
    HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION,
};

pub use stabilize_the_release_center_promotion_evidence_canary_pilot::{
    current_ring_promotion_control, Action as PromotionAction,
    GapReason as PromotionGapReason, KillSwitchPosture, PromotionDecision as RingPromotionDecision,
    PromotionPublicationRecord, PromotionRule, PromotionState, PromotionSubjectExportRow,
    PromotionSubjectKind, PromotionSubjectRow, Ring, RingPromotionControl,
    RingPromotionControlExportProjection, RingPromotionControlSummary,
    RingPromotionControlViolation, RollbackStopTrigger, RollbackTriggerKind, SoakWindow,
    RING_PROMOTION_CONTROL_JSON, RING_PROMOTION_CONTROL_PATH, RING_PROMOTION_CONTROL_RECORD_KIND,
    RING_PROMOTION_CONTROL_SCHEMA_VERSION,
};
