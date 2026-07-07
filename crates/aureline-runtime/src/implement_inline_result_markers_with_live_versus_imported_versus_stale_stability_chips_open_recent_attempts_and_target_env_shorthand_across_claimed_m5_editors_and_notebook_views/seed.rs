//! Canonical seed builders for the M5 inline-result-marker primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical inline-marker primitive packet.
pub const M5_INLINE_RESULT_MARKER_PACKET_ID: &str = "m5-inline-result-marker-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked inline-result-marker resolution case from a full marker state.
#[allow(clippy::too_many_arguments)]
fn marker_case(
    verdict: M5InlineMarkerVerdict,
    failure_category: Option<M5FailureCategory>,
    stability_chip: M5MarkerStabilityChip,
    result_origin: M5TestResultOrigin,
    result_freshness: M5TestResultFreshness,
    source_mapping: M5MarkerSourceMapping,
    target_class: M5TestTargetClass,
    environment_lane: M5TestEnvironmentLane,
    attempt_lineage: M5AttemptLineageKind,
    quarantine_ownership: M5QuarantineOwnership,
    release_impact: M5TestReleaseImpact,
    recent_attempt_count: u32,
    item_muted: bool,
    marker_label: &str,
    marker_identity_ref: &str,
) -> M5InlineMarkerResolutionCase {
    M5InlineMarkerResolutionCase::resolved(M5InlineMarkerResolutionInput {
        verdict,
        failure_category,
        stability_chip,
        result_origin,
        result_freshness,
        source_mapping,
        target_class,
        environment_lane,
        attempt_lineage,
        quarantine_ownership,
        release_impact,
        recent_attempt_count,
        item_muted,
        marker_label: marker_label.to_owned(),
        marker_identity_ref: marker_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full inline-marker anatomy, verdict,
/// stability-chip, result-origin, freshness, source-mapping, posture, attempt-lineage,
/// action, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5InlineMarkerConsumerSurface,
    qualification: M5TestQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    marker_examples: Vec<M5InlineMarkerResolutionCase>,
) -> M5InlineMarkerConsumerRow {
    M5InlineMarkerConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5InlineMarkerAnatomyPart::ALL.to_vec(),
        marker_verdicts: M5InlineMarkerVerdict::ALL.to_vec(),
        stability_chips: M5MarkerStabilityChip::ALL.to_vec(),
        result_origins: M5TestResultOrigin::ALL.to_vec(),
        result_freshness: M5TestResultFreshness::ALL.to_vec(),
        source_mappings: M5MarkerSourceMapping::ALL.to_vec(),
        marker_postures: M5InlineMarkerPosture::ALL.to_vec(),
        attempt_lineage_kinds: M5AttemptLineageKind::ALL.to_vec(),
        marker_actions: M5InlineMarkerAction::ALL.to_vec(),
        export_fields: M5InlineMarkerExportField::ALL.to_vec(),
        accessibility_routes: M5TestAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestDowngradeTrigger::ResultOriginUnstated,
            M5TestDowngradeTrigger::ResultFreshnessUndisclosed,
            M5TestDowngradeTrigger::AttemptLineageUnstated,
            M5TestDowngradeTrigger::QuarantineReleaseImpactHidden,
            M5TestDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_INLINE_RESULT_MARKER_SCHEMA_REF,
            M5_INLINE_RESULT_MARKER_TEST_ITEM_IDENTITY_REF,
            M5_INLINE_RESULT_MARKER_TEST_ATTEMPT_REF,
        ]),
        marker_examples,
        masks_verdict_or_origin: false,
        hides_quarantine_release_impact: false,
        overstates_imported_or_stale_as_live: false,
        drops_attempt_lineage: false,
    }
}

fn rows() -> Vec<M5InlineMarkerConsumerRow> {
    use M5AttemptLineageKind as Lineage;
    use M5FailureCategory as Failure;
    use M5InlineMarkerVerdict as Verdict;
    use M5MarkerSourceMapping as Mapping;
    use M5MarkerStabilityChip as Chip;
    use M5QuarantineOwnership as Owner;
    use M5TestEnvironmentLane as Env;
    use M5TestReleaseImpact as Impact;
    use M5TestResultFreshness as Fresh;
    use M5TestResultOrigin as Origin;
    use M5TestTargetClass as Target;

    vec![
        // 1. Editor-gutter marker — a fresh live-local pass that maps exactly to the buffer
        //    (the only posture that may imply a current local result), and a stale live-local
        //    assertion failure that still reruns but no longer reads as live.
        base_row(
            M5InlineMarkerConsumerSurface::EditorGutterMarker,
            M5TestQualificationClass::Stable,
            "Editor gutter marker owner",
            "The editor-gutter marker renders the shared inline result marker so a fresh live-local pass that maps exactly to the buffer reads as the only live-local marker that may imply a current local result and exposes rerun and open-recent-attempts, and a stale live-local assertion failure degrades to a stale-result marker that still reruns but never reads as live",
            "evidence:m5-inline-marker-editor-gutter:001",
            vec![
                marker_case(
                    Verdict::Passed,
                    None,
                    Chip::StableChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::ExactMapping,
                    Target::UnitTest,
                    Env::LocalHost,
                    Lineage::RetriedPass,
                    Owner::Unowned,
                    Impact::NoImpact,
                    3,
                    false,
                    "token refresh returns a fresh token",
                    "marker:auth-unit::token-refresh",
                ),
                marker_case(
                    Verdict::Failed,
                    Some(Failure::AssertionFailure),
                    Chip::FlakySuspectedChip,
                    Origin::LiveLocal,
                    Fresh::Stale,
                    Mapping::ExactMapping,
                    Target::UnitTest,
                    Env::LocalHost,
                    Lineage::RetriedFail,
                    Owner::Unowned,
                    Impact::NoImpact,
                    2,
                    false,
                    "price rounds half to even",
                    "marker:pricing::round-half-even",
                ),
            ],
        ),
        // 2. Editor inline marker — a live-local pass that maps only approximately after the
        //    source drifted (degrades to an approximate-mapping marker), and a live-local
        //    runtime error whose source no longer maps to the buffer at all and cannot rerun
        //    or open attempts.
        base_row(
            M5InlineMarkerConsumerSurface::EditorInlineMarker,
            M5TestQualificationClass::Stable,
            "Editor inline marker owner",
            "The editor inline marker renders the shared inline result marker so a live-local pass whose source drifted maps only approximately and degrades to an approximate-mapping marker rather than implying a current local result, and a live-local runtime error whose source no longer maps to the buffer reads as an unmapped marker that withholds the rerun and open-recent-attempts it cannot honestly offer",
            "evidence:m5-inline-marker-editor-inline:001",
            vec![
                marker_case(
                    Verdict::Passed,
                    None,
                    Chip::StableChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::ApproximateMapping,
                    Target::IntegrationTest,
                    Env::Container,
                    Lineage::RerunSelected,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "matrix parse case eleven",
                    "marker:integration::matrix-parse-11",
                ),
                marker_case(
                    Verdict::Errored,
                    Some(Failure::RuntimeError),
                    Chip::UnknownStabilityChip,
                    Origin::LiveLocal,
                    Fresh::OutdatedSource,
                    Mapping::UnmappedToBuffer,
                    Target::IntegrationTest,
                    Env::LocalHost,
                    Lineage::RerunSelected,
                    Owner::Unowned,
                    Impact::NoImpact,
                    0,
                    false,
                    "connection pool teardown",
                    "marker:integration::pool-teardown",
                ),
            ],
        ),
        // 3. Notebook-cell marker — an imported-CI timeout that is replay-only (reduced
        //    certainty, no local rerun) but still opens its recent attempts, and a fresh
        //    live-local pass mapped exactly to the cell.
        base_row(
            M5InlineMarkerConsumerSurface::NotebookCellMarker,
            M5TestQualificationClass::Stable,
            "Notebook cell marker owner",
            "The notebook-cell marker renders the shared inline result marker so an imported-CI timeout reads as an imported-evidence marker that withholds the local rerun it cannot honestly offer yet still opens its recent attempts, and a fresh live-local pass mapped exactly to the cell reads as a live-local marker — so imported evidence never inherits live certainty",
            "evidence:m5-inline-marker-notebook-cell:001",
            vec![
                marker_case(
                    Verdict::Failed,
                    Some(Failure::Timeout),
                    Chip::KnownFlakyChip,
                    Origin::ImportedCi,
                    Fresh::Fresh,
                    Mapping::ExactMapping,
                    Target::EndToEndTest,
                    Env::CiMatrix,
                    Lineage::ReplayedImport,
                    Owner::Unowned,
                    Impact::NoImpact,
                    4,
                    false,
                    "notebook: checkout flow smoke (from CI)",
                    "marker:notebook::checkout-flow@ci",
                ),
                marker_case(
                    Verdict::Passed,
                    None,
                    Chip::StableChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::ExactMapping,
                    Target::UiSnapshotTest,
                    Env::LocalHost,
                    Lineage::FirstAttempt,
                    Owner::Unowned,
                    Impact::NoImpact,
                    1,
                    false,
                    "notebook: data-load smoke",
                    "marker:notebook::data-load-smoke",
                ),
            ],
        ),
        // 4. Headless / CLI marker — a fresh live-local pass with no local buffer to map to
        //    (still live certainty, still rerunnable), and a replayed-snapshot flaky-suspected
        //    result that opens its recent attempts but stays reduced certainty; proves the
        //    same grammar works without an editor buffer.
        base_row(
            M5InlineMarkerConsumerSurface::HeadlessCliMarker,
            M5TestQualificationClass::Stable,
            "Headless CLI marker owner",
            "The headless / CLI marker renders the shared inline result marker so a fresh live-local pass with no local buffer to decorate still reads as a live-local marker that reruns, and a replayed-snapshot flaky-suspected result reads as an imported-evidence marker that opens its recent attempts yet stays reduced certainty — proving the same marker grammar works headless",
            "evidence:m5-inline-marker-headless-cli:001",
            vec![
                marker_case(
                    Verdict::Passed,
                    None,
                    Chip::StableChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::NoLocalBuffer,
                    Target::ContractTest,
                    Env::RemoteRunner,
                    Lineage::FirstAttempt,
                    Owner::Unowned,
                    Impact::NoImpact,
                    0,
                    false,
                    "contract: schema stays backward compatible",
                    "marker:contract::schema-back-compat",
                ),
                marker_case(
                    Verdict::FlakySuspected,
                    Some(Failure::FlakyUnderReview),
                    Chip::FlakySuspectedChip,
                    Origin::ReplayedSnapshot,
                    Fresh::Fresh,
                    Mapping::NoLocalBuffer,
                    Target::EndToEndTest,
                    Env::CiMatrix,
                    Lineage::ReplayedImport,
                    Owner::Unowned,
                    Impact::NoImpact,
                    5,
                    false,
                    "replayed: nightly regression sweep",
                    "marker:e2e::nightly-regression@replay",
                ),
            ],
        ),
        // 5. Marker report export — a team-owned quarantined live-local failure whose
        //    hidden-from-release impact heads the marker (still reruns and opens attempts and
        //    exposes review-quarantine), and a fresh live-local benchmark pass; proves
        //    quarantine coverage and the same marker a reviewer reads elsewhere.
        base_row(
            M5InlineMarkerConsumerSurface::MarkerReportExport,
            M5TestQualificationClass::Stable,
            "Marker report export owner",
            "The marker-report export renders the shared inline result marker so a team-owned quarantined live-local failure reads as a quarantined marker whose hidden-from-release impact heads it while still exposing rerun, open-recent-attempts, and review-quarantine, and a fresh live-local benchmark pass reads as a live-local marker — the same marker a reviewer reads elsewhere",
            "evidence:m5-inline-marker-report-export:001",
            vec![
                marker_case(
                    Verdict::Failed,
                    Some(Failure::FlakyUnderReview),
                    Chip::QuarantinedChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::ExactMapping,
                    Target::UnitTest,
                    Env::LocalHost,
                    Lineage::RetriedFail,
                    Owner::TeamOwned,
                    Impact::HiddenFromRelease,
                    6,
                    true,
                    "flaky login redirect",
                    "marker:auth::login-redirect-quarantined",
                ),
                marker_case(
                    Verdict::Passed,
                    None,
                    Chip::StableChip,
                    Origin::LiveLocal,
                    Fresh::Fresh,
                    Mapping::ExactMapping,
                    Target::BenchmarkTest,
                    Env::BrowserMatrix,
                    Lineage::RerunFailedOnly,
                    Owner::Unowned,
                    Impact::NoImpact,
                    2,
                    false,
                    "render benchmark holds budget",
                    "marker:bench::render-budget",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5InlineMarkerGovernanceReview {
    M5InlineMarkerGovernanceReview {
        marker_shows_verdict_state: true,
        marker_shows_stability_chip: true,
        marker_shows_origin_class_and_freshness: true,
        marker_shows_target_and_environment: true,
        marker_shows_source_mapping: true,
        marker_exposes_recent_attempts: true,
        marker_shows_mute_and_release_impact: true,
        imported_or_stale_never_reads_as_live: true,
        markers_keep_parity_with_tree_and_triage: true,
        markers_stable_across_deployment_lines: true,
        markers_stable_across_consumer_surfaces: true,
        every_marker_declares_accessibility_route: true,
        support_export_reconstructs_marker_truth: true,
        later_markers_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5InlineMarkerConsumerProjection {
    M5InlineMarkerConsumerProjection {
        editor_and_notebook_surfaces_consume_marker_vocabulary: true,
        marker_posture_reads_single_source: true,
        tree_and_triage_read_same_labels: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5InlineMarkerProofFreshness {
    M5InlineMarkerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InlineMarkerReleasePosture {
    M5InlineMarkerReleasePosture {
        release_packet_ref: M5_INLINE_RESULT_MARKER_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_INLINE_RESULT_MARKER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INLINE_RESULT_MARKER_SCHEMA_REF,
        M5_INLINE_RESULT_MARKER_DOC_REF,
        M5_INLINE_RESULT_MARKER_COMPONENT_MATRIX_REF,
        M5_INLINE_RESULT_MARKER_TEST_ITEM_IDENTITY_REF,
        M5_INLINE_RESULT_MARKER_TEST_ATTEMPT_REF,
    ])
}

/// Builds the canonical M5 inline-result-marker packet.
pub fn seeded_m5_inline_result_marker_packet() -> M5InlineResultMarkerPacket {
    M5InlineResultMarkerPacket::new(M5InlineResultMarkerPacketInput {
        packet_id: M5_INLINE_RESULT_MARKER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 inline-result-marker primitive: pass/fail/error/timeout verdict, stability-or-flaky chip, imported/live origin class, last-result freshness, source-mapping fidelity, target/environment shorthand, attempt lineage, mute/quarantine and release impact, derived marker posture, and bounded reveal-evidence/open-recent-attempts/rerun/review-quarantine/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5InlineMarkerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the notebook-cell marker consumer is narrowed to Preview pending
/// imported-versus-live certainty parity proof across every cell state; every consumer
/// stays visible.
pub fn seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed(
) -> M5InlineResultMarkerPacket {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.packet_id =
        "m5-inline-result-marker-primitive:notebook-cell-marker-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InlineMarkerConsumerSurface::NotebookCellMarker)
        .expect("notebook-cell-marker row present");
    row.qualification = M5TestQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI marker consumer is held at Beta because a slice of
/// headless markers do not yet render the keyboard route cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed(
) -> M5InlineResultMarkerPacket {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.packet_id = "m5-inline-result-marker-primitive:headless-cli-marker-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InlineMarkerConsumerSurface::HeadlessCliMarker)
        .expect("headless-cli-marker row present");
    row.qualification = M5TestQualificationClass::Beta;
    packet
}
