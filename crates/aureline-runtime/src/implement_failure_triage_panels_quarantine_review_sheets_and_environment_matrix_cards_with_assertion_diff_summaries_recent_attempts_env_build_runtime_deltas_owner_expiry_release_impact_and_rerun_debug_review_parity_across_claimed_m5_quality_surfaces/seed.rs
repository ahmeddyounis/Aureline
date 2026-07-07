//! Canonical seed builders for the M5 failure-triage-panel / quarantine-review-sheet /
//! environment-matrix-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical quality-triage-status primitive packet.
pub const M5_QUALITY_TRIAGE_STATUS_PACKET_ID: &str =
    "m5-failure-triage-quarantine-environment-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked failure-triage-panel resolution case from a full failure state.
#[allow(clippy::too_many_arguments)]
fn triage_case(
    failure_category: M5FailureCategory,
    triage_disposition: M5TriageDisposition,
    result_origin: M5TestResultOrigin,
    classifier_confidence: M5ClassifierConfidence,
    recent_attempts: &[M5AttemptLineageKind],
    has_assertion_or_diff_summary: bool,
    has_env_build_runtime_delta: bool,
    assertion_summary_label: &str,
    panel_identity_ref: &str,
) -> M5TriagePanelResolutionCase {
    M5TriagePanelResolutionCase::resolved(M5TriagePanelResolutionInput {
        failure_category,
        triage_disposition,
        result_origin,
        classifier_confidence,
        recent_attempts: recent_attempts.to_vec(),
        has_assertion_or_diff_summary,
        has_env_build_runtime_delta,
        assertion_summary_label: assertion_summary_label.to_owned(),
        panel_identity_ref: panel_identity_ref.to_owned(),
    })
}

/// Builds a worked quarantine-review-sheet resolution case from a full suppression state.
#[allow(clippy::too_many_arguments)]
fn quarantine_case(
    suppression_kind: M5SuppressionKind,
    suppression_scope: M5SuppressionScope,
    ownership: M5QuarantineOwnership,
    release_impact: M5TestReleaseImpact,
    expiry_state: M5QuarantineExpiry,
    has_linked_artifacts: bool,
    reason_label: &str,
    owner_label: &str,
    sheet_identity_ref: &str,
) -> M5QuarantineReviewResolutionCase {
    M5QuarantineReviewResolutionCase::resolved(M5QuarantineReviewResolutionInput {
        suppression_kind,
        suppression_scope,
        ownership,
        release_impact,
        expiry_state,
        has_linked_artifacts,
        reason_label: reason_label.to_owned(),
        owner_label: owner_label.to_owned(),
        sheet_identity_ref: sheet_identity_ref.to_owned(),
    })
}

/// Builds one compared environment leg.
fn leg(
    environment_lane: M5TestEnvironmentLane,
    target_compatibility: M5EnvCompatibilityClass,
    runtime_compatibility: M5EnvCompatibilityClass,
    toolchain_compatibility: M5EnvCompatibilityClass,
    build_compatibility: M5EnvCompatibilityClass,
    leg_label: &str,
) -> M5EnvironmentCompatibilityLeg {
    M5EnvironmentCompatibilityLeg {
        environment_lane,
        target_compatibility,
        runtime_compatibility,
        toolchain_compatibility,
        build_compatibility,
        leg_label: leg_label.to_owned(),
    }
}

/// Builds a worked environment-matrix-card resolution case.
fn environment_case(
    target_class: M5TestTargetClass,
    primary_environment_lane: M5TestEnvironmentLane,
    legs: Vec<M5EnvironmentCompatibilityLeg>,
    card_identity_ref: &str,
) -> M5EnvironmentCardResolutionCase {
    M5EnvironmentCardResolutionCase::resolved(M5EnvironmentCardResolutionInput {
        target_class,
        primary_environment_lane,
        legs,
        card_identity_ref: card_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full triage/quarantine/environment
/// anatomy, vocabulary, posture, action, export-field, and accessibility parity every consumer
/// carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5QualityTriageConsumerSurface,
    qualification: M5TestQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    triage_examples: Vec<M5TriagePanelResolutionCase>,
    quarantine_examples: Vec<M5QuarantineReviewResolutionCase>,
    environment_examples: Vec<M5EnvironmentCardResolutionCase>,
) -> M5QualityTriageConsumerRow {
    M5QualityTriageConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestDeploymentLine::ALL.to_vec(),
        triage_anatomy_parts: M5TriagePanelAnatomyPart::ALL.to_vec(),
        quarantine_anatomy_parts: M5QuarantineReviewAnatomyPart::ALL.to_vec(),
        environment_anatomy_parts: M5EnvironmentCardAnatomyPart::ALL.to_vec(),
        failure_categories: M5FailureCategory::ALL.to_vec(),
        triage_dispositions: M5TriageDisposition::ALL.to_vec(),
        classifier_confidences: M5ClassifierConfidence::ALL.to_vec(),
        triage_postures: M5TriagePanelPosture::ALL.to_vec(),
        suppression_kinds: M5SuppressionKind::ALL.to_vec(),
        suppression_scopes: M5SuppressionScope::ALL.to_vec(),
        quarantine_ownership_classes: M5QuarantineOwnership::ALL.to_vec(),
        release_impacts: M5TestReleaseImpact::ALL.to_vec(),
        expiry_states: M5QuarantineExpiry::ALL.to_vec(),
        quarantine_postures: M5QuarantineReviewPosture::ALL.to_vec(),
        target_classes: M5TestTargetClass::ALL.to_vec(),
        environment_lanes: M5TestEnvironmentLane::ALL.to_vec(),
        compatibility_classes: M5EnvCompatibilityClass::ALL.to_vec(),
        environment_postures: M5EnvironmentCardPosture::ALL.to_vec(),
        attempt_lineage_kinds: M5AttemptLineageKind::ALL.to_vec(),
        result_origins: M5TestResultOrigin::ALL.to_vec(),
        triage_actions: M5TriagePanelAction::ALL.to_vec(),
        quarantine_actions: M5QuarantineReviewAction::ALL.to_vec(),
        environment_actions: M5EnvironmentCardAction::ALL.to_vec(),
        triage_export_fields: M5TriagePanelExportField::ALL.to_vec(),
        quarantine_export_fields: M5QuarantineReviewExportField::ALL.to_vec(),
        environment_export_fields: M5EnvironmentCardExportField::ALL.to_vec(),
        accessibility_routes: M5TestAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestDowngradeTrigger::AttemptLineageUnstated,
            M5TestDowngradeTrigger::QuarantineOwnershipUnstated,
            M5TestDowngradeTrigger::QuarantineReleaseImpactHidden,
            M5TestDowngradeTrigger::EnvironmentOrTargetUnstated,
            M5TestDowngradeTrigger::AlternateStateLabelInvented,
            M5TestDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF,
            M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF,
            M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF,
            M5_QUALITY_TRIAGE_STATUS_QUARANTINE_RECORD_REF,
            M5_QUALITY_TRIAGE_STATUS_RELEASE_VISIBILITY_REF,
            M5_QUALITY_TRIAGE_STATUS_ATTEMPT_RECORDS_REF,
        ]),
        triage_examples,
        quarantine_examples,
        environment_examples,
        offers_suppression_without_evidence: false,
        hides_owner_expiry_or_release_impact: false,
        implies_safe_environment_equivalence: false,
        drops_recent_attempts_or_deltas: false,
    }
}

fn rows() -> Vec<M5QualityTriageConsumerRow> {
    use M5AttemptLineageKind as Lineage;
    use M5ClassifierConfidence as Confidence;
    use M5EnvCompatibilityClass as Compat;
    use M5FailureCategory as Category;
    use M5QuarantineExpiry as Expiry;
    use M5QuarantineOwnership as Owner;
    use M5SuppressionKind as Kind;
    use M5SuppressionScope as SuppScope;
    use M5TestEnvironmentLane as Env;
    use M5TestReleaseImpact as Impact;
    use M5TestResultOrigin as Origin;
    use M5TestTargetClass as Target;
    use M5TriageDisposition as Disposition;

    vec![
        // 1. Test-explorer triage view — a live-local assertion failure (debuggable) and an
        //    imported timeout, a blocking quarantine and a hidden-from-release quarantine that
        //    both stay visible, and a fully compatible unit-test environment matrix.
        base_row(
            M5QualityTriageConsumerSurface::TestExplorerTriageView,
            M5TestQualificationClass::Stable,
            "Test explorer triage owner",
            "The test-explorer triage view renders the shared failure-triage panel so a live-local assertion failure reads with its assertion/diff summary, recent attempt sequence, environment/build/runtime deltas, and classifier confidence before it can escalate to the quarantine review, and it renders the quarantine review sheet so a blocking or hidden-from-release suppression stays visible with its owner, expiry, and release impact and always offers restore, and it renders the environment-matrix card so a fully compatible unit-test matrix compares every leg without implying safe equivalence",
            "evidence:m5-quality-triage-explorer:001",
            vec![
                triage_case(
                    Category::AssertionFailure,
                    Disposition::ProductBug,
                    Origin::LiveLocal,
                    Confidence::HighConfidence,
                    &[Lineage::FirstAttempt, Lineage::RetriedFail],
                    true,
                    true,
                    "assert eq: expected 200 got 500 on auth callback",
                    "triage:explorer::auth-assert",
                ),
                triage_case(
                    Category::Timeout,
                    Disposition::EnvironmentIssue,
                    Origin::ImportedCi,
                    Confidence::MediumConfidence,
                    &[Lineage::ReplayedImport],
                    false,
                    true,
                    "timeout after 30s on imported ci leg",
                    "triage:explorer::ci-timeout",
                ),
            ],
            vec![
                quarantine_case(
                    Kind::Quarantined,
                    SuppScope::SingleCase,
                    Owner::TeamOwned,
                    Impact::BlocksRelease,
                    Expiry::ExpiresScheduled,
                    true,
                    "flaky auth callback under active investigation; still gates release",
                    "team: payments-quality",
                    "quarantine:explorer::auth-blocking",
                ),
                quarantine_case(
                    Kind::Quarantined,
                    SuppScope::WholeFile,
                    Owner::SelfOwned,
                    Impact::HiddenFromRelease,
                    Expiry::NoExpiry,
                    false,
                    "pricing snapshot suite hidden from gating pending redesign, stays visible in triage",
                    "self: current reviewer",
                    "quarantine:explorer::pricing-hidden",
                ),
            ],
            vec![environment_case(
                Target::UnitTest,
                Env::LocalHost,
                vec![
                    leg(
                        Env::LocalHost,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::NotApplicable,
                        "local host: full compatibility",
                    ),
                    leg(
                        Env::Container,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "container: full compatibility",
                    ),
                ],
                "environment:explorer::unit-compatible",
            )],
        ),
        // 2. Editor inline triage — a live-local runtime error under investigation with low
        //    confidence, an unowned muted suppression that stays visible while hidden from
        //    release, and a mixed integration-test environment matrix.
        base_row(
            M5QualityTriageConsumerSurface::EditorInlineTriage,
            M5TestQualificationClass::Stable,
            "Editor inline triage owner",
            "The editor inline triage renders the shared failure-triage panel so a low-confidence runtime error reads as provisional with its evidence before rerun/debug/review, renders the quarantine review sheet so an unowned muted suppression stays visible with a reassign-owner action rather than disappearing into a filter, and renders the environment-matrix card so a mixed integration matrix warns instead of implying safe equivalence",
            "evidence:m5-quality-triage-editor:001",
            vec![triage_case(
                Category::RuntimeError,
                Disposition::NeedsInvestigation,
                Origin::LiveLocal,
                Confidence::LowConfidence,
                &[Lineage::FirstAttempt],
                true,
                false,
                "panic: index out of bounds in reducer",
                "triage:editor::reducer-panic",
            )],
            vec![quarantine_case(
                Kind::Muted,
                SuppScope::WholeFile,
                Owner::Unowned,
                Impact::HiddenFromRelease,
                Expiry::NoExpiry,
                false,
                "muted legacy import spec, no owner assigned yet; stays visible for reassignment",
                "unassigned",
                "quarantine:editor::legacy-unowned",
            )],
            vec![environment_case(
                Target::IntegrationTest,
                Env::Container,
                vec![
                    leg(
                        Env::Container,
                        Compat::FullyCompatible,
                        Compat::PartiallyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "container: partial runtime compatibility",
                    ),
                    leg(
                        Env::CiMatrix,
                        Compat::PartiallyCompatible,
                        Compat::PartiallyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "ci matrix: partial compatibility",
                    ),
                ],
                "environment:editor::integration-mixed",
            )],
        ),
        // 3. Notebook triage view — a replayed environment error, an expired owner-expired
        //    skip suppression (renew + reassign) with linked artifacts, and an incompatible
        //    end-to-end environment matrix that withholds safe equivalence.
        base_row(
            M5QualityTriageConsumerSurface::NotebookTriageView,
            M5TestQualificationClass::Stable,
            "Notebook triage owner",
            "The notebook triage view renders the shared failure-triage panel so a replayed environment error reads with its deltas and recent attempts, renders the quarantine review sheet so an expired, owner-expired skip suppression reads with renew and reassign actions and its linked artifacts while staying visible, and renders the environment-matrix card so an incompatible browser end-to-end matrix warns and never implies safe equivalence",
            "evidence:m5-quality-triage-notebook:001",
            vec![triage_case(
                Category::EnvironmentError,
                Disposition::EnvironmentIssue,
                Origin::ReplayedSnapshot,
                Confidence::MediumConfidence,
                &[Lineage::RerunSelected],
                false,
                true,
                "missing GPU device on replayed notebook kernel",
                "triage:notebook::gpu-missing",
            )],
            vec![quarantine_case(
                Kind::Skipped,
                SuppScope::ParametrizationSubset,
                Owner::OwnerExpired,
                Impact::SoftGated,
                Expiry::Expired,
                true,
                "skipped wide-locale parametrization; suppression expired and owner lapsed",
                "team: platform-i18n (lapsed)",
                "quarantine:notebook::locale-expired",
            )],
            vec![environment_case(
                Target::EndToEndTest,
                Env::BrowserMatrix,
                vec![
                    leg(
                        Env::BrowserMatrix,
                        Compat::Incompatible,
                        Compat::PartiallyCompatible,
                        Compat::FullyCompatible,
                        Compat::PartiallyCompatible,
                        "browser matrix: incompatible target engine",
                    ),
                    leg(
                        Env::EmulatedDevice,
                        Compat::Incompatible,
                        Compat::Incompatible,
                        Compat::PartiallyCompatible,
                        Compat::FullyCompatible,
                        "emulated device: incompatible target and runtime",
                    ),
                ],
                "environment:notebook::e2e-incompatible",
            )],
        ),
        // 4. Run-panel triage — a live-local flaky failure under review (high confidence),
        //    a CI-enforced review-due quarantine (renew), and an unverified benchmark matrix.
        base_row(
            M5QualityTriageConsumerSurface::RunPanelTriage,
            M5TestQualificationClass::Stable,
            "Run panel triage owner",
            "The run-panel triage renders the shared failure-triage panel so a flaky failure under review reads with its recent pass/fail attempt sequence and confidence, renders the quarantine review sheet so a CI-enforced review-due quarantine reads with a renew action while staying visible with its informational release impact, and renders the environment-matrix card so an unverified benchmark matrix reads as unverified rather than compatible",
            "evidence:m5-quality-triage-run-panel:001",
            vec![triage_case(
                Category::FlakyUnderReview,
                Disposition::KnownFlaky,
                Origin::LiveLocal,
                Confidence::HighConfidence,
                &[Lineage::RetriedPass, Lineage::RetriedFail],
                true,
                true,
                "intermittent race in scheduler flush",
                "triage:run-panel::scheduler-flaky",
            )],
            vec![quarantine_case(
                Kind::Quarantined,
                SuppScope::TaggedGroup,
                Owner::CiEnforced,
                Impact::Informational,
                Expiry::ReviewDue,
                false,
                "ci-enforced quarantine of tagged integration group, review window elapsed",
                "ci: nightly-quality-bot",
                "quarantine:run-panel::tagged-review-due",
            )],
            vec![environment_case(
                Target::BenchmarkTest,
                Env::RemoteRunner,
                vec![
                    leg(
                        Env::RemoteRunner,
                        Compat::Unverified,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "remote runner: target compatibility unverified",
                    ),
                    leg(
                        Env::LocalHost,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "local host: full compatibility",
                    ),
                ],
                "environment:run-panel::benchmark-unverified",
            )],
        ),
        // 5. Quality report export — an unclassified unknown-origin failure (unknown
        //    confidence), a governed self-owned permanent mute, and a compatible contract
        //    matrix; the same triage a reviewer reads elsewhere.
        base_row(
            M5QualityTriageConsumerSurface::QualityReportExport,
            M5TestQualificationClass::Stable,
            "Quality report export owner",
            "The quality report export renders the shared failure-triage panel so an unclassified failure with unknown confidence still reads as provisional with its recent attempts, renders the quarantine review sheet so a governed self-owned permanent mute reads as governed with no outstanding review while still preserving its reason and restore, and renders the environment-matrix card so a compatible contract matrix compares every leg — the same triage a reviewer reads in the tree and run-panel consumers",
            "evidence:m5-quality-triage-report-export:001",
            vec![triage_case(
                Category::UnknownFailure,
                Disposition::NeedsInvestigation,
                Origin::UnknownOrigin,
                Confidence::UnknownConfidence,
                &[Lineage::FirstAttempt],
                false,
                false,
                "unclassified nonzero exit with no captured assertion",
                "triage:report::unclassified-exit",
            )],
            vec![quarantine_case(
                Kind::Muted,
                SuppScope::WholeSuite,
                Owner::SelfOwned,
                Impact::NoImpact,
                Expiry::PermanentPolicy,
                false,
                "permanently muted vendor smoke suite by policy; no release impact, owner accountable",
                "self: current reviewer",
                "quarantine:report::vendor-governed",
            )],
            vec![environment_case(
                Target::ContractTest,
                Env::CiMatrix,
                vec![
                    leg(
                        Env::CiMatrix,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        "ci matrix: full compatibility",
                    ),
                    leg(
                        Env::RemoteRunner,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::FullyCompatible,
                        Compat::NotApplicable,
                        "remote runner: full compatibility",
                    ),
                ],
                "environment:report::contract-compatible",
            )],
        ),
    ]
}

fn governance_review() -> M5QualityTriageGovernanceReview {
    M5QualityTriageGovernanceReview {
        panel_shows_assertion_and_recent_attempts: true,
        panel_shows_deltas_and_confidence: true,
        panel_offers_rerun_debug_review: true,
        no_suppression_without_evidence_context: true,
        sheet_preserves_scope_reason_owner_expiry_artifacts_impact: true,
        sheet_keeps_suppressed_test_visible: true,
        sheet_offers_restore_action: true,
        card_compares_target_runtime_toolchain_build: true,
        card_never_implies_safe_equivalence: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_quality_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5QualityTriageConsumerProjection {
    M5QualityTriageConsumerProjection {
        tree_and_triage_surfaces_consume_quality_vocabulary: true,
        triage_posture_reads_single_source: true,
        quarantine_posture_reads_single_source: true,
        environment_posture_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5QualityTriageProofFreshness {
    M5QualityTriageProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5QualityTriageReleasePosture {
    M5QualityTriageReleasePosture {
        release_packet_ref: M5_QUALITY_TRIAGE_STATUS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_QUALITY_TRIAGE_STATUS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_DOC_REF,
        M5_QUALITY_TRIAGE_STATUS_COMPONENT_MATRIX_REF,
        M5_QUALITY_TRIAGE_STATUS_QUARANTINE_RECORD_REF,
        M5_QUALITY_TRIAGE_STATUS_RELEASE_VISIBILITY_REF,
        M5_QUALITY_TRIAGE_STATUS_ATTEMPT_RECORDS_REF,
    ])
}

/// Builds the canonical M5 quality-triage-status packet.
pub fn seeded_m5_quality_triage_status_packet() -> M5QualityTriageStatusPacket {
    M5QualityTriageStatusPacket::new(M5QualityTriageStatusPacketInput {
        packet_id: M5_QUALITY_TRIAGE_STATUS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 failure-triage-panel / quarantine-review-sheet / environment-matrix-card primitive: assertion/diff summaries, recent attempt sequences, environment/build/runtime deltas, classifier confidence, evidence-gated rerun/debug/open-review actions, preserved suppression scope/kind/reason/owner/expiry/linked-artifacts/release-impact with always-visible quarantines and a restore action, and target/runtime/toolchain/build compatibility comparison that never implies safe equivalence across incompatible environments"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5QualityTriageVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the notebook triage view consumer is narrowed to Preview pending
/// environment-matrix-card compatibility parity proof across every deployment line; every
/// consumer stays visible.
pub fn seeded_m5_quality_triage_status_notebook_triage_preview_narrowed(
) -> M5QualityTriageStatusPacket {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.packet_id =
        "m5-failure-triage-quarantine-environment-primitive:notebook-triage-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5QualityTriageConsumerSurface::NotebookTriageView)
        .expect("notebook-triage row present");
    row.qualification = M5TestQualificationClass::Preview;
    packet
}

/// Narrowed variant: the editor inline triage consumer is held at Beta because a slice of
/// editor surfaces do not yet render the classifier-confidence cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed(
) -> M5QualityTriageStatusPacket {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.packet_id =
        "m5-failure-triage-quarantine-environment-primitive:editor-inline-triage-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5QualityTriageConsumerSurface::EditorInlineTriage)
        .expect("editor-inline-triage row present");
    row.qualification = M5TestQualificationClass::Beta;
    packet
}
