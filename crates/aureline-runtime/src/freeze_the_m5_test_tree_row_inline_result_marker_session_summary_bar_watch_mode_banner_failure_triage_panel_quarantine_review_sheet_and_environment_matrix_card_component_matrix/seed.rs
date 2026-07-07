//! Canonical seed builders for the frozen M5 test-explorer / watch / triage component
//! matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical test-explorer / watch / triage component matrix.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-test-explorer-watch-triage-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5TestRequiredLabel> {
    M5TestRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5TestRequiredLabel]) -> Vec<M5TestRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5TestExplorerWatchTriageComponentFamily,
    qualification: M5TestQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5TestExplorerWatchTriageComponentRow {
    M5TestExplorerWatchTriageComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        test_identity_classes: vec![],
        result_origins: vec![],
        marker_verdicts: vec![],
        result_freshness: vec![],
        session_outcomes: vec![],
        attempt_lineage_kinds: vec![],
        watch_fidelity_states: vec![],
        watch_degrade_reasons: vec![],
        failure_categories: vec![],
        triage_dispositions: vec![],
        quarantine_ownership_classes: vec![],
        release_impacts: vec![],
        target_classes: vec![],
        environment_lanes: vec![],
        accessibility_routes: M5TestAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5TestConsumerSurface::TestTreeUi,
            M5TestConsumerSurface::SupportExport,
            M5TestConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5TestDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_identity_or_origin: false,
        hides_quarantine_release_impact: false,
        invents_alternate_state_label: false,
        widens_rerun_scope_silently: false,
    }
}

fn component_rows() -> Vec<M5TestExplorerWatchTriageComponentRow> {
    use M5AttemptLineageKind as AL;
    use M5FailureCategory as FC;
    use M5InlineMarkerVerdict as MV;
    use M5QuarantineOwnership as QO;
    use M5TestConsumerSurface as C;
    use M5TestDowngradeTrigger as D;
    use M5TestEnvironmentLane as EL;
    use M5TestExplorerWatchTriageComponentFamily as F;
    use M5TestIdentityClass as IC;
    use M5TestQualificationClass as Q;
    use M5TestReleaseImpact as RI;
    use M5TestRequiredLabel as L;
    use M5TestResultFreshness as RF;
    use M5TestResultOrigin as RO;
    use M5TestSessionOutcome as SO;
    use M5TestTargetClass as TC;
    use M5TriageDisposition as TD;
    use M5WatchDegradeReason as WD;
    use M5WatchFidelityState as WF;

    let mut rows = Vec::new();

    // 1. Test-tree row.
    let mut row = base_row(
        F::TestTreeRow,
        Q::Stable,
        "Test-tree row owner",
        "One test-tree-row model naming how a test is identified — a durable keyed identity, a path-derived identity, a discovery-only identity, an imported record, a parametrized case, or an ambiguous identity — and whether its latest result was produced live-locally or imported, so a user never has to guess whether a red mark is local or imported or which durable test a row represents",
        "evidence:m5-test-tree-row-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_ITEM_IDENTITY_REF,
        ],
    );
    row.test_identity_classes = IC::ALL.to_vec();
    row.result_origins = RO::ALL.to_vec();
    row.required_labels = labels_with(&[L::OriginAndFreshness]);
    row.consumer_surfaces = vec![
        C::TestTreeUi,
        C::EditorGutterUi,
        C::SessionSummaryUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::IdentityClassUnstated,
        D::ResultOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Inline result marker.
    let mut row = base_row(
        F::InlineResultMarker,
        Q::Stable,
        "Inline result marker owner",
        "One inline-result-marker model naming the verdict a marker asserts — passed, failed, errored, skipped, flaky-suspected, or not-run — how fresh that result is, and whether it was produced live-locally or imported, so a marker in the editor gutter never shows a stale or imported result as if it were freshly produced here",
        "evidence:m5-inline-result-marker-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_ITEM_IDENTITY_REF,
        ],
    );
    row.marker_verdicts = MV::ALL.to_vec();
    row.result_freshness = RF::ALL.to_vec();
    row.result_origins = RO::ALL.to_vec();
    row.required_labels = labels_with(&[L::OriginAndFreshness]);
    row.consumer_surfaces = vec![
        C::EditorGutterUi,
        C::TestTreeUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ResultFreshnessUndisclosed,
        D::ResultOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Session-summary bar.
    let mut row = base_row(
        F::SessionSummaryBar,
        Q::Stable,
        "Session-summary bar owner",
        "One session-summary-bar model naming the overall outcome of a run — all passed, some failed, errored, partial discovery, cancelled, or in progress — and how the current attempt relates to prior attempts, so retry lineage and rerun scope are explicit and a partial discovery is never shown as a complete green run",
        "evidence:m5-test-session-summary-bar-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF,
        ],
    );
    row.session_outcomes = SO::ALL.to_vec();
    row.attempt_lineage_kinds = AL::ALL.to_vec();
    row.required_labels = labels_with(&[L::OriginAndFreshness]);
    row.consumer_surfaces = vec![
        C::SessionSummaryUi,
        C::TestTreeUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AttemptLineageUnstated,
        D::RerunScopeWidened,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Watch-mode banner.
    let mut row = base_row(
        F::WatchModeBanner,
        Q::Stable,
        "Watch-mode banner owner",
        "One watch-mode-banner model naming how faithfully watch mode is observing — live, reduced, polling, unavailable, paused, or reconnecting — and why fidelity dropped, so a user never assumes results are current when watch has silently degraded and always sees why watch degraded",
        "evidence:m5-test-watch-mode-banner-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_WATCH_STATE_REF,
        ],
    );
    row.watch_fidelity_states = WF::ALL.to_vec();
    row.watch_degrade_reasons = WD::ALL.to_vec();
    row.required_labels = labels_with(&[L::WatchFidelity]);
    row.consumer_surfaces = vec![
        C::WatchBannerUi,
        C::SessionSummaryUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::WatchFidelityUnstated,
        D::WatchDegradeReasonHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Failure-triage panel.
    let mut row = base_row(
        F::FailureTriagePanel,
        Q::Stable,
        "Failure-triage panel owner",
        "One failure-triage-panel model naming what class of failure a test hit — assertion failure, runtime error, timeout, environment error, flaky-under-review, or unknown failure — and where it sits in triage, so a failure is never left uncategorized and its disposition is always explicit",
        "evidence:m5-test-failure-triage-panel-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF,
        ],
    );
    row.failure_categories = FC::ALL.to_vec();
    row.triage_dispositions = TD::ALL.to_vec();
    row.required_labels = labels_with(&[L::OriginAndFreshness]);
    row.consumer_surfaces = vec![
        C::TriagePanelUi,
        C::SessionSummaryUi,
        C::QuarantineSheetUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ResultOriginUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Quarantine-review sheet.
    let mut row = base_row(
        F::QuarantineReviewSheet,
        Q::Stable,
        "Quarantine-review sheet owner",
        "One quarantine-review-sheet model naming who owns a mute or quarantine — unowned, self-owned, team-owned, CI-enforced, imported from policy, or owner-expired — and what it hides from release and support surfaces, so a user always sees what a mute or quarantine will hide from release and never mistakes an unowned quarantine for a governed one",
        "evidence:m5-test-quarantine-review-sheet-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_QUARANTINE_RECORD_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_RELEASE_VISIBILITY_REF,
        ],
    );
    row.quarantine_ownership_classes = QO::ALL.to_vec();
    row.release_impacts = RI::ALL.to_vec();
    row.required_labels = labels_with(&[L::QuarantineAndReleaseImpact]);
    row.consumer_surfaces = vec![
        C::QuarantineSheetUi,
        C::TriagePanelUi,
        C::SessionSummaryUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::QuarantineOwnershipUnstated,
        D::QuarantineReleaseImpactHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Environment-matrix card.
    let mut row = base_row(
        F::EnvironmentMatrixCard,
        Q::Stable,
        "Environment-matrix card owner",
        "One environment-matrix-card model naming what kind of test a card represents — unit, integration, end-to-end, UI snapshot, benchmark, or contract — and where it runs, so the target and environment behind a result are always explicit and a local result is never confused with a remote or CI-matrix result",
        "evidence:m5-test-environment-matrix-card-parity:001",
        &[
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
            M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF,
        ],
    );
    row.target_classes = TC::ALL.to_vec();
    row.environment_lanes = EL::ALL.to_vec();
    row.required_labels = labels_with(&[L::OriginAndFreshness]);
    row.consumer_surfaces = vec![
        C::SessionSummaryUi,
        C::TestTreeUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EnvironmentOrTargetUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5TestExplorerWatchTriageComponentGovernanceReview {
    M5TestExplorerWatchTriageComponentGovernanceReview {
        test_tree_row_shows_identity_and_origin: true,
        inline_marker_shows_verdict_and_freshness: true,
        session_summary_shows_outcome_and_attempt_lineage: true,
        watch_banner_shows_fidelity_and_degrade_reason: true,
        failure_triage_shows_category_and_disposition: true,
        quarantine_sheet_shows_ownership_and_release_impact: true,
        environment_card_shows_target_and_environment: true,
        no_surface_invents_alternate_state_label: true,
        live_reduced_polling_unavailable_named_once: true,
        imported_versus_live_always_explicit: true,
        rerun_scope_always_explicit: true,
        quarantine_release_impact_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5TestExplorerWatchTriageComponentConsumerProjection {
    M5TestExplorerWatchTriageComponentConsumerProjection {
        tree_and_editor_surfaces_consume_identity_vocabulary: true,
        marker_surfaces_consume_freshness_and_origin_vocabulary: true,
        watch_surfaces_consume_fidelity_vocabulary: true,
        triage_surfaces_consume_failure_category_vocabulary: true,
        quarantine_surfaces_consume_ownership_and_release_impact_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5TestExplorerWatchTriageComponentProofFreshness {
    M5TestExplorerWatchTriageComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TestExplorerWatchTriageComponentReleasePosture {
    M5TestExplorerWatchTriageComponentReleasePosture {
        proof_packet_ref: M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_DOC_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_ITEM_IDENTITY_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_WATCH_STATE_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_QUARANTINE_RECORD_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_RELEASE_VISIBILITY_REF,
    ])
}

/// Builds the canonical frozen M5 test-explorer / watch / triage component matrix packet.
pub fn seeded_m5_test_explorer_watch_triage_component_matrix(
) -> M5TestExplorerWatchTriageComponentMatrixPacket {
    M5TestExplorerWatchTriageComponentMatrixPacket::new(
        M5TestExplorerWatchTriageComponentMatrixPacketInput {
            packet_id: M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_PACKET_ID.to_owned(),
            matrix_label:
                "M5 test-tree-row, inline-result-marker, session-summary-bar, watch-mode-banner, failure-triage-panel, quarantine-review-sheet, and environment-matrix-card component matrix"
                    .to_owned(),
            component_rows: component_rows(),
            vocabulary_set: M5TestExplorerWatchTriageComponentVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the watch-mode banner is held at Beta because a slice of the
/// reconnecting fidelity state does not yet round-trip across every test surface; every
/// component stays visible.
pub fn seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed(
) -> M5TestExplorerWatchTriageComponentMatrixPacket {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.packet_id =
        "m5-test-explorer-watch-triage-components:watch-mode-banner-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5TestExplorerWatchTriageComponentFamily::WatchModeBanner
        })
        .expect("watch-mode-banner row present");
    row.qualification = M5TestQualificationClass::Beta;
    packet
}

/// Narrowed variant: the quarantine-review sheet is narrowed to Preview pending
/// release-impact parity proof across every surface; every component stays visible.
pub fn seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed(
) -> M5TestExplorerWatchTriageComponentMatrixPacket {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.packet_id =
        "m5-test-explorer-watch-triage-components:quarantine-review-sheet-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5TestExplorerWatchTriageComponentFamily::QuarantineReviewSheet
        })
        .expect("quarantine-review-sheet row present");
    row.qualification = M5TestQualificationClass::Preview;
    packet
}
