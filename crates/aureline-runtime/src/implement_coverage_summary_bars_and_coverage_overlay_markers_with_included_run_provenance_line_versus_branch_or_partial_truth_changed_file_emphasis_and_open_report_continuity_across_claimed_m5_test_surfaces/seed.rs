//! Canonical seed builders for the M5 coverage-summary-bar / coverage-overlay-marker
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical coverage-components primitive packet.
pub const M5_COVERAGE_COMPONENTS_PACKET_ID: &str =
    "m5-coverage-summary-overlay-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked coverage-summary-bar resolution case from a full coverage state.
#[allow(clippy::too_many_arguments)]
fn summary_case(
    scope_class: M5CoverageScopeClass,
    metric_kind: M5CoverageMetricKind,
    provenance_class: M5TestIntelligenceProvenanceClass,
    freshness_state: M5CoverageFreshnessState,
    source_note: M5CoverageSourceNote,
    included_run_count: u32,
    covered_units: u32,
    total_units: u32,
    has_shard_omission: bool,
    scope_label: &str,
    summary_identity_ref: &str,
) -> M5CoverageSummaryResolutionCase {
    M5CoverageSummaryResolutionCase::resolved(M5CoverageSummaryResolutionInput {
        scope_class,
        metric_kind,
        provenance_class,
        freshness_state,
        source_note,
        included_run_count,
        covered_units,
        total_units,
        has_shard_omission,
        scope_label: scope_label.to_owned(),
        summary_identity_ref: summary_identity_ref.to_owned(),
    })
}

/// Builds a worked coverage-overlay-marker resolution case from a full overlay state.
fn overlay_case(
    overlay_state: M5CoverageOverlayState,
    emphasis_class: M5OverlayEmphasisClass,
    provenance_class: M5TestIntelligenceProvenanceClass,
    is_changed_line: bool,
    source_run_set_ref: &str,
    evidence_object_ref: &str,
    line_reference: &str,
) -> M5OverlayMarkerResolutionCase {
    M5OverlayMarkerResolutionCase::resolved(M5OverlayMarkerResolutionInput {
        overlay_state,
        emphasis_class,
        provenance_class,
        is_changed_line,
        source_run_set_ref: source_run_set_ref.to_owned(),
        evidence_object_ref: evidence_object_ref.to_owned(),
        line_reference: line_reference.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full summary/overlay anatomy, scope,
/// metric, provenance, freshness, source-note, posture, overlay-state, emphasis, action,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5CoverageComponentConsumerSurface,
    qualification: M5TestIntelligenceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    summary_examples: Vec<M5CoverageSummaryResolutionCase>,
    overlay_examples: Vec<M5OverlayMarkerResolutionCase>,
) -> M5CoverageComponentConsumerRow {
    M5CoverageComponentConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestIntelligenceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestIntelligenceDeploymentLine::ALL.to_vec(),
        summary_anatomy_parts: M5CoverageSummaryAnatomyPart::ALL.to_vec(),
        overlay_anatomy_parts: M5OverlayMarkerAnatomyPart::ALL.to_vec(),
        coverage_scope_classes: M5CoverageScopeClass::ALL.to_vec(),
        coverage_metric_kinds: M5CoverageMetricKind::ALL.to_vec(),
        provenance_classes: M5TestIntelligenceProvenanceClass::ALL.to_vec(),
        freshness_states: M5CoverageFreshnessState::ALL.to_vec(),
        source_notes: M5CoverageSourceNote::ALL.to_vec(),
        coverage_postures: M5CoverageSummaryPosture::ALL.to_vec(),
        overlay_states: M5CoverageOverlayState::ALL.to_vec(),
        overlay_emphasis_classes: M5OverlayEmphasisClass::ALL.to_vec(),
        overlay_postures: M5OverlayMarkerPosture::ALL.to_vec(),
        summary_actions: M5CoverageSummaryAction::ALL.to_vec(),
        overlay_actions: M5OverlayMarkerAction::ALL.to_vec(),
        summary_export_fields: M5CoverageSummaryExportField::ALL.to_vec(),
        overlay_export_fields: M5OverlayMarkerExportField::ALL.to_vec(),
        accessibility_routes: M5TestIntelligenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestIntelligenceConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated,
            M5TestIntelligenceDowngradeTrigger::FreshnessClassUndisclosed,
            M5TestIntelligenceDowngradeTrigger::ShardOmissionHidden,
            M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated,
            M5TestIntelligenceDowngradeTrigger::AlternateStateLabelInvented,
            M5TestIntelligenceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF,
            M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF,
            M5_COVERAGE_COMPONENTS_COVERAGE_MERGE_REF,
            M5_COVERAGE_COMPONENTS_COVERAGE_OVERLAY_REF,
        ]),
        summary_examples,
        overlay_examples,
        collapses_multi_run_into_single_percentage: false,
        hides_shard_omission_or_stale_provenance: false,
        drops_line_versus_branch_dimension: false,
        invents_alternate_coverage_state_label: false,
    }
}

fn rows() -> Vec<M5CoverageComponentConsumerRow> {
    use M5CoverageFreshnessState as Fresh;
    use M5CoverageMetricKind as Metric;
    use M5CoverageOverlayState as Overlay;
    use M5CoverageScopeClass as Scope;
    use M5CoverageSourceNote as Note;
    use M5OverlayEmphasisClass as Emphasis;
    use M5TestIntelligenceProvenanceClass as Prov;

    vec![
        // 1. Coverage report panel — a fresh full-suite line-coverage number with uncovered
        //    lines and a merged multi-shard number that requires an included-run label and
        //    offers a rerun; a stably-covered marker and an emphasized newly-uncovered changed
        //    line.
        base_row(
            M5CoverageComponentConsumerSurface::CoverageReportPanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Coverage report panel owner",
            "The coverage-report panel renders the shared coverage-summary bar so a fresh full-suite line-coverage number reads as a full-suite summary whose uncovered lines can be opened, and a merged multi-shard number reads as a distinct merged-multi-shard summary that always shows its included run set rather than collapsing four shards into one percentage; it renders the shared overlay marker so a stably-covered line and an emphasized newly-uncovered changed line each keep their exact coverage state and a path back to the evidence object",
            "evidence:m5-coverage-report-panel:001",
            vec![
                summary_case(
                    Scope::FullSuite,
                    Metric::LineCoverage,
                    Prov::VerifiedCurrentRun,
                    Fresh::FreshCurrentRun,
                    Note::LiveLocalRun,
                    1,
                    880,
                    1000,
                    false,
                    "full suite: line coverage",
                    "coverage:report::full-suite-line",
                ),
                summary_case(
                    Scope::MergedMultiShard,
                    Metric::CombinedMetric,
                    Prov::VerifiedCurrentRun,
                    Fresh::RecentlyMeasured,
                    Note::MergedMultiRun,
                    4,
                    3200,
                    4000,
                    false,
                    "merged: four shards combined",
                    "coverage:report::merged-four-shard",
                ),
            ],
            vec![
                overlay_case(
                    Overlay::CoveredLine,
                    Emphasis::StableCovered,
                    Prov::VerifiedCurrentRun,
                    false,
                    "run-set:report::current-full",
                    "coverage-object:report::covered-line-42",
                    "src/pricing.rs:42",
                ),
                overlay_case(
                    Overlay::UncoveredLine,
                    Emphasis::NewlyUncovered,
                    Prov::VerifiedCurrentRun,
                    true,
                    "run-set:report::current-full",
                    "coverage-object:report::uncovered-line-88",
                    "src/pricing.rs:88",
                ),
            ],
        ),
        // 2. Editor gutter overlay — a fresh changed-files line-coverage summary, plus a
        //    regression-hotspot partial marker and a branch-missed marker, both on changed
        //    lines, so changed-line emphasis survives into the editor gutter.
        base_row(
            M5CoverageComponentConsumerSurface::EditorGutterOverlay,
            M5TestIntelligenceQualificationClass::Stable,
            "Editor gutter overlay owner",
            "The editor gutter overlay renders the shared coverage-summary bar so a fresh changed-files line-coverage summary reads as a changed-files summary, and it renders the shared overlay marker so a partially-covered regression hotspot and a branch-missed marker on changed lines keep their exact partial / branch-missed meaning, stay emphasized as changed lines, and preserve a durable path back to the coverage evidence — the editor-to-report continuity",
            "evidence:m5-coverage-editor-gutter:001",
            vec![summary_case(
                Scope::ChangedFilesOnly,
                Metric::LineCoverage,
                Prov::VerifiedCurrentRun,
                Fresh::FreshCurrentRun,
                Note::LiveLocalRun,
                1,
                140,
                160,
                false,
                "changed files: since last green",
                "coverage:editor::changed-files-line",
            )],
            vec![
                overlay_case(
                    Overlay::PartiallyCovered,
                    Emphasis::RegressionHotspot,
                    Prov::VerifiedCurrentRun,
                    true,
                    "run-set:editor::current-changed",
                    "coverage-object:editor::partial-line-17",
                    "src/auth.rs:17",
                ),
                overlay_case(
                    Overlay::BranchMissed,
                    Emphasis::ChangedLineEmphasis,
                    Prov::VerifiedCurrentRun,
                    true,
                    "run-set:editor::current-changed",
                    "coverage-object:editor::branch-line-23",
                    "src/auth.rs:23",
                ),
            ],
        ),
        // 3. CI coverage summary — an imported branch-coverage report from a CI artifact that
        //    reads as a distinct imported-report summary (always labeled, never collapsed into
        //    a local number) and offers a rerun; an excluded-line marker.
        base_row(
            M5CoverageComponentConsumerSurface::CiCoverageSummary,
            M5TestIntelligenceQualificationClass::Stable,
            "CI coverage summary owner",
            "The CI coverage summary renders the shared coverage-summary bar so an imported branch-coverage report from a CI artifact reads as a distinct imported-report summary that names its imported source note and included run set rather than passing as a fresh local number, and it renders the shared overlay marker so an excluded line keeps its excluded meaning",
            "evidence:m5-coverage-ci-summary:001",
            vec![summary_case(
                Scope::ImportedReport,
                Metric::BranchCoverage,
                Prov::ImportedCiArtifact,
                Fresh::ImportedSnapshot,
                Note::ImportedReport,
                1,
                720,
                900,
                false,
                "imported: nightly ci branch report",
                "coverage:ci::imported-branch-report",
            )],
            vec![overlay_case(
                Overlay::ExcludedLine,
                Emphasis::SuppressedRegion,
                Prov::ImportedCiArtifact,
                false,
                "run-set:ci::nightly-import",
                "coverage-object:ci::excluded-line-5",
                "src/generated.rs:5",
            )],
        ),
        // 4. Headless / CLI coverage — a cached single-shard region-coverage summary that reads
        //    as a distinct single-shard summary and offers a rerun of the non-current number; a
        //    no-overlay-data marker; proves the same grammar works headless.
        base_row(
            M5CoverageComponentConsumerSurface::HeadlessCliCoverage,
            M5TestIntelligenceQualificationClass::Stable,
            "Headless CLI coverage owner",
            "The headless / CLI coverage surface renders the shared coverage-summary bar so a cached single-shard region-coverage summary reads as a distinct single-shard summary that names its cached source note and offers a rerun of the non-current number, and it renders the shared overlay marker so a line with no overlay data reads as an unknown marker rather than a covered one — proving the same coverage grammar works without a desktop surface",
            "evidence:m5-coverage-headless-cli:001",
            vec![summary_case(
                Scope::SingleShard,
                Metric::RegionCoverage,
                Prov::CachedLocalResult,
                Fresh::RecentlyMeasured,
                Note::CachedReuse,
                1,
                410,
                500,
                false,
                "single shard: cached region coverage",
                "coverage:headless::single-shard-region",
            )],
            vec![overlay_case(
                Overlay::NoOverlayData,
                Emphasis::ContextLine,
                Prov::Unknown,
                false,
                "run-set:headless::cached-shard",
                "coverage-object:headless::no-data-line-9",
                "src/vendor.rs:9",
            )],
        ),
        // 5. Coverage report export — a stale partial-incomplete summary that discloses a shard
        //    omission and stale provenance rather than a green number, spans two runs, and
        //    offers a rerun; a covered marker the reviewer reads elsewhere with the same
        //    vocabulary.
        base_row(
            M5CoverageComponentConsumerSurface::CoverageReportExport,
            M5TestIntelligenceQualificationClass::Stable,
            "Coverage report export owner",
            "The coverage report export renders the shared coverage-summary bar so a stale partial-incomplete summary discloses its shard omission and stale provenance instead of presenting a green number, names its included run set, and offers a rerun, and it renders the shared overlay marker so a covered line reads with the same covered vocabulary a reviewer sees in the report and the editor",
            "evidence:m5-coverage-report-export:001",
            vec![summary_case(
                Scope::PartialIncomplete,
                Metric::StatementCoverage,
                Prov::StalePriorResult,
                Fresh::StaleNeedsRerun,
                Note::StaleReplay,
                2,
                300,
                600,
                true,
                "partial: incomplete, one shard omitted",
                "coverage:export::partial-incomplete",
            )],
            vec![overlay_case(
                Overlay::CoveredLine,
                Emphasis::StableCovered,
                Prov::VerifiedCurrentRun,
                false,
                "run-set:export::current-full",
                "coverage-object:export::covered-line-101",
                "src/report.rs:101",
            )],
        ),
    ]
}

fn governance_review() -> M5CoverageComponentGovernanceReview {
    M5CoverageComponentGovernanceReview {
        bar_shows_scope_and_metric_dimension: true,
        bar_shows_included_run_set: true,
        bar_shows_freshness_and_source_note: true,
        bar_never_collapses_multi_run_into_one_percentage: true,
        bar_exposes_open_uncovered_lines: true,
        overlay_preserves_exact_state_meaning: true,
        overlay_preserves_changed_line_emphasis: true,
        overlay_preserves_source_run_set_identity: true,
        overlay_offers_durable_path_back_to_evidence: true,
        shard_omission_and_stale_never_hidden: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_coverage_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CoverageComponentConsumerProjection {
    M5CoverageComponentConsumerProjection {
        report_and_editor_surfaces_consume_coverage_vocabulary: true,
        summary_posture_reads_single_source: true,
        overlay_posture_reads_single_source: true,
        ci_and_support_read_same_coverage_vocabulary: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5CoverageComponentProofFreshness {
    M5CoverageComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CoverageComponentReleasePosture {
    M5CoverageComponentReleasePosture {
        release_packet_ref: M5_COVERAGE_COMPONENTS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_COVERAGE_COMPONENTS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF,
        M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF,
        M5_COVERAGE_COMPONENTS_DOC_REF,
        M5_COVERAGE_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_COVERAGE_COMPONENTS_COVERAGE_MERGE_REF,
        M5_COVERAGE_COMPONENTS_COVERAGE_OVERLAY_REF,
    ])
}

/// Builds the canonical M5 coverage-components packet.
pub fn seeded_m5_coverage_components_packet() -> M5CoverageComponentsPacket {
    M5CoverageComponentsPacket::new(M5CoverageComponentsPacketInput {
        packet_id: M5_COVERAGE_COMPONENTS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 coverage-summary-bar / coverage-overlay-marker primitive: coverage scope, line-versus-branch-or-combined metric dimension, included run set, freshness, imported/merged/live source note, distinct full-suite/changed-files/single-shard/merged-multi-shard/imported-report/partial-incomplete coverage postures, controlled covered/uncovered/partial/branch-missed/excluded/unknown overlay postures, preserved changed-line emphasis and source run-set identity, durable path back to the evidence object, and bounded reveal/open-uncovered-lines/open-report/rerun and reveal/open-report/open-uncovered-context/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5CoverageComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the coverage-report panel consumer is narrowed to Preview pending
/// merged-multi-shard-versus-imported-report posture parity proof across every deployment line;
/// every consumer stays visible.
pub fn seeded_m5_coverage_components_report_panel_preview_narrowed() -> M5CoverageComponentsPacket {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.packet_id = "m5-coverage-summary-overlay-primitive:report-panel-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoverageComponentConsumerSurface::CoverageReportPanel)
        .expect("coverage-report-panel row present");
    row.qualification = M5TestIntelligenceQualificationClass::Preview;
    packet
}

/// Narrowed variant: the editor gutter overlay consumer is held at Beta because a slice of
/// editor surfaces do not yet render the changed-line emphasis cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed(
) -> M5CoverageComponentsPacket {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.packet_id =
        "m5-coverage-summary-overlay-primitive:editor-gutter-overlay-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CoverageComponentConsumerSurface::EditorGutterOverlay)
        .expect("editor-gutter-overlay row present");
    row.qualification = M5TestIntelligenceQualificationClass::Beta;
    packet
}
