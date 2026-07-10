//! Canonical seed builders for the M5 snapshot-review-card / coverage-import-merge-sheet
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical snapshot-merge-components primitive packet.
pub const M5_SNAPSHOT_MERGE_COMPONENTS_PACKET_ID: &str =
    "m5-snapshot-coverage-import-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked snapshot-review-card resolution case from a full review state.
#[allow(clippy::too_many_arguments)]
fn snapshot_case(
    artifact_kind: M5SnapshotArtifactKind,
    baseline_identity: M5SnapshotBaselineIdentity,
    diff_state: M5SnapshotDiffState,
    fallback_mode: M5SnapshotFallbackMode,
    scope_dimensions: &[M5SnapshotScopeDimension],
    diff_count: u32,
    provenance_class: M5TestIntelligenceProvenanceClass,
    card_identity_ref: &str,
    baseline_ref: &str,
) -> M5SnapshotCardResolutionCase {
    M5SnapshotCardResolutionCase::resolved(M5SnapshotCardResolutionInput {
        artifact_kind,
        baseline_identity,
        diff_state,
        fallback_mode,
        scope_dimensions: scope_dimensions.to_vec(),
        diff_count,
        provenance_class,
        card_identity_ref: card_identity_ref.to_owned(),
        baseline_ref: baseline_ref.to_owned(),
    })
}

/// Builds a worked coverage-import-merge-sheet resolution case from a full import / merge state.
#[allow(clippy::too_many_arguments)]
fn merge_case(
    import_source: M5CoverageImportSource,
    merge_resolution: M5MergeResolutionState,
    metric_kinds: &[M5CoverageMetricKind],
    included_runs: &[&str],
    excluded_runs: &[&str],
    provenance_class: M5TestIntelligenceProvenanceClass,
    is_stale: bool,
    is_incompatible: bool,
    claims_exact_current_truth: bool,
    commit_identity_ref: &str,
    build_identity_ref: &str,
    sheet_identity_ref: &str,
) -> M5MergeSheetResolutionCase {
    M5MergeSheetResolutionCase::resolved(M5MergeSheetResolutionInput {
        import_source,
        merge_resolution,
        metric_kinds: metric_kinds.to_vec(),
        included_runs: strings(included_runs),
        excluded_runs: strings(excluded_runs),
        provenance_class,
        is_stale,
        is_incompatible,
        claims_exact_current_truth,
        commit_identity_ref: commit_identity_ref.to_owned(),
        build_identity_ref: build_identity_ref.to_owned(),
        sheet_identity_ref: sheet_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full snapshot / merge anatomy, artifact
/// kind, baseline identity, diff state, fallback mode, scope, review posture, import source,
/// merge-resolution state, metric kind, merge posture, provenance, action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5SnapshotMergeComponentConsumerSurface,
    qualification: M5TestIntelligenceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    snapshot_examples: Vec<M5SnapshotCardResolutionCase>,
    merge_examples: Vec<M5MergeSheetResolutionCase>,
) -> M5SnapshotMergeComponentConsumerRow {
    M5SnapshotMergeComponentConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestIntelligenceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestIntelligenceDeploymentLine::ALL.to_vec(),
        snapshot_anatomy_parts: M5SnapshotCardAnatomyPart::ALL.to_vec(),
        merge_anatomy_parts: M5MergeSheetAnatomyPart::ALL.to_vec(),
        snapshot_artifact_kinds: M5SnapshotArtifactKind::ALL.to_vec(),
        snapshot_baseline_identities: M5SnapshotBaselineIdentity::ALL.to_vec(),
        snapshot_diff_states: M5SnapshotDiffState::ALL.to_vec(),
        snapshot_fallback_modes: M5SnapshotFallbackMode::ALL.to_vec(),
        snapshot_scope_dimensions: M5SnapshotScopeDimension::ALL.to_vec(),
        snapshot_review_postures: M5SnapshotReviewPosture::ALL.to_vec(),
        coverage_import_sources: M5CoverageImportSource::ALL.to_vec(),
        merge_resolution_states: M5MergeResolutionState::ALL.to_vec(),
        coverage_metric_kinds: M5CoverageMetricKind::ALL.to_vec(),
        merge_postures: M5CoverageMergePosture::ALL.to_vec(),
        provenance_classes: M5TestIntelligenceProvenanceClass::ALL.to_vec(),
        snapshot_actions: M5SnapshotCardAction::ALL.to_vec(),
        merge_actions: M5MergeSheetAction::ALL.to_vec(),
        snapshot_export_fields: M5SnapshotCardExportField::ALL.to_vec(),
        merge_export_fields: M5MergeSheetExportField::ALL.to_vec(),
        accessibility_routes: M5TestIntelligenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestIntelligenceConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestIntelligenceDowngradeTrigger::SnapshotBaselineUnstated,
            M5TestIntelligenceDowngradeTrigger::RawTextFallbackMissing,
            M5TestIntelligenceDowngradeTrigger::ShardOmissionHidden,
            M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated,
            M5TestIntelligenceDowngradeTrigger::AlternateStateLabelInvented,
            M5TestIntelligenceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF,
            M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF,
            M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_REVIEW_REF,
            M5_SNAPSHOT_MERGE_COMPONENTS_COVERAGE_MERGE_REF,
        ]),
        snapshot_examples,
        merge_examples,
        collapses_snapshot_accept_without_scope_or_fallback: false,
        hides_baseline_identity_or_artifact_count: false,
        hides_shard_omission_or_incompatible_warning: false,
        invents_alternate_snapshot_or_merge_state_label: false,
    }
}

fn rows() -> Vec<M5SnapshotMergeComponentConsumerRow> {
    use M5CoverageImportSource as Import;
    use M5CoverageMetricKind as Metric;
    use M5MergeResolutionState as Merge;
    use M5SnapshotArtifactKind as Artifact;
    use M5SnapshotBaselineIdentity as Baseline;
    use M5SnapshotDiffState as Diff;
    use M5SnapshotFallbackMode as Fallback;
    use M5SnapshotScopeDimension as Scope;
    use M5TestIntelligenceProvenanceClass as Prov;

    vec![
        // 1. Snapshot review panel — a detected diff against a committed baseline that reads as an
        //    acceptance decision only because it discloses its environment / viewport / theme scope
        //    and a side-by-side raw fallback, and a matched baseline that needs no acceptance; a
        //    clean local coverage merge that may be treated as exact current truth precisely
        //    because no run was omitted, nothing is stale, and nothing is incompatible.
        base_row(
            M5SnapshotMergeComponentConsumerSurface::SnapshotReviewPanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Snapshot review panel owner",
            "The snapshot review panel renders the shared snapshot / golden review card so a detected diff against a committed baseline reads as an acceptance decision only when it discloses its artifact count, its environment / viewport / theme scope, and a side-by-side raw fallback rather than collapsing to a blind Accept all, while a matched baseline needs no acceptance; it renders the shared coverage-import / merge sheet so a clean local merge may be treated as exact current truth precisely because no run was omitted, nothing is stale, and nothing is incompatible",
            "evidence:m5-snapshot-review-panel:001",
            vec![
                snapshot_case(
                    Artifact::ImageSnapshot,
                    Baseline::CommittedBaseline,
                    Diff::DiffDetected,
                    Fallback::SideBySide,
                    &[Scope::Environment, Scope::Viewport, Scope::Theme],
                    3,
                    Prov::VerifiedCurrentRun,
                    "snapshot-card:review-panel::checkout-visual",
                    "baseline:review-panel::checkout-visual-committed",
                ),
                snapshot_case(
                    Artifact::TextSerializerSnapshot,
                    Baseline::CommittedBaseline,
                    Diff::MatchesBaseline,
                    Fallback::RenderedDiff,
                    &[Scope::Serializer],
                    0,
                    Prov::VerifiedCurrentRun,
                    "snapshot-card:review-panel::settings-serializer",
                    "baseline:review-panel::settings-serializer-committed",
                ),
            ],
            vec![merge_case(
                Import::LocalRun,
                Merge::MergedClean,
                &[Metric::LineCoverage, Metric::BranchCoverage],
                &["run-alpha", "run-beta"],
                &[],
                Prov::VerifiedCurrentRun,
                false,
                false,
                true,
                "commit:review-panel::abc123",
                "build:review-panel::ci-4821",
                "merge-sheet:review-panel::checkout-coverage",
            )],
        ),
        // 2. Editor snapshot diff — a brand-new snapshot with a pending baseline that stays an
        //    acceptance decision with its scope shown, and a partial coverage merge from a cached
        //    report that names the excluded shard rather than hiding the omission behind the
        //    merged number.
        base_row(
            M5SnapshotMergeComponentConsumerSurface::EditorSnapshotDiff,
            M5TestIntelligenceQualificationClass::Stable,
            "Editor snapshot diff owner",
            "The editor snapshot-diff surface renders the shared snapshot / golden review card so a brand-new snapshot with a pending baseline stays an acceptance decision that discloses its environment / theme scope and a side-by-side raw fallback, and it renders the shared coverage-import / merge sheet so a partial merge drawn from a cached report names the excluded shard rather than presenting the merged number as exact current truth",
            "evidence:m5-editor-snapshot-diff:001",
            vec![snapshot_case(
                Artifact::DomSnapshot,
                Baseline::PendingNewBaseline,
                Diff::NewSnapshot,
                Fallback::SideBySide,
                &[Scope::Environment, Scope::Theme],
                1,
                Prov::VerifiedCurrentRun,
                "snapshot-card:editor::new-header-dom",
                "baseline:editor::new-header-dom-pending",
            )],
            vec![merge_case(
                Import::CachedLocal,
                Merge::PartialMerge,
                &[Metric::LineCoverage],
                &["editor-run-1"],
                &["editor-shard-2"],
                Prov::CachedLocalResult,
                false,
                false,
                false,
                "commit:editor::def456",
                "build:editor::local-77",
                "merge-sheet:editor::partial-coverage",
            )],
        ),
        // 3. Coverage import / merge panel — an obsolete snapshot against an updated baseline shown
        //    as a raw / text fallback, a shard omission imported from CI that names both omitted
        //    shards, and a conflicting overlap from an uploaded report flagged as incompatible.
        base_row(
            M5SnapshotMergeComponentConsumerSurface::CoverageImportMergePanel,
            M5TestIntelligenceQualificationClass::Stable,
            "Coverage import / merge panel owner",
            "The coverage-import / merge panel renders the shared snapshot / golden review card so an obsolete snapshot against an updated baseline reads as an obsolete-snapshot card shown through a raw / text fallback, and it renders the shared coverage-import / merge sheet so a shard omission imported from a CI artifact names both omitted shards and a conflicting overlap from an uploaded report is flagged incompatible before any merged number is treated as exact current truth",
            "evidence:m5-coverage-import-merge-panel:001",
            vec![snapshot_case(
                Artifact::JsonSnapshot,
                Baseline::UpdatedBaseline,
                Diff::ObsoleteSnapshot,
                Fallback::RawTextFallback,
                &[Scope::Serializer, Scope::Locale],
                0,
                Prov::StalePriorResult,
                "snapshot-card:import-panel::obsolete-json",
                "baseline:import-panel::obsolete-json-updated",
            )],
            vec![
                merge_case(
                    Import::ImportedCiArtifact,
                    Merge::ShardOmissionDetected,
                    &[Metric::LineCoverage, Metric::BranchCoverage, Metric::FunctionCoverage],
                    &["ci-run-1"],
                    &["ci-shard-b", "ci-shard-c"],
                    Prov::ImportedCiArtifact,
                    false,
                    false,
                    false,
                    "commit:import-panel::ghi789",
                    "build:import-panel::ci-5120",
                    "merge-sheet:import-panel::ci-shard-omission",
                ),
                merge_case(
                    Import::UploadedReport,
                    Merge::ConflictingOverlap,
                    &[Metric::LineCoverage],
                    &["upload-1", "upload-2"],
                    &[],
                    Prov::Unknown,
                    false,
                    true,
                    false,
                    "commit:import-panel::jkl012",
                    "build:import-panel::upload-33",
                    "merge-sheet:import-panel::conflicting-overlap",
                ),
            ],
        ),
        // 4. Headless / CLI review — a render-unavailable card for an opaque binary snapshot
        //    imported from a CI baseline that keeps its raw / text fallback, and a superseded
        //    coverage report from a stale prior run flagged stale.
        base_row(
            M5SnapshotMergeComponentConsumerSurface::HeadlessCliReview,
            M5TestIntelligenceQualificationClass::Stable,
            "Headless / CLI review owner",
            "The headless / CLI review surface renders the shared snapshot / golden review card so an opaque binary snapshot imported from a CI baseline whose rendered diff is unavailable still keeps a raw / text-only fallback rather than a blind accept, and it renders the shared coverage-import / merge sheet so a coverage report superseded by a newer run from a stale prior source is flagged stale — proving the same grammar works without a desktop surface",
            "evidence:m5-headless-cli-review:001",
            vec![snapshot_case(
                Artifact::BinarySnapshot,
                Baseline::ImportedBaseline,
                Diff::RenderUnavailable,
                Fallback::RawTextOnly,
                &[Scope::Environment],
                2,
                Prov::ImportedCiArtifact,
                "snapshot-card:headless::binary-render-unavailable",
                "baseline:headless::binary-imported",
            )],
            vec![merge_case(
                Import::StalePrior,
                Merge::SupersededByNewer,
                &[Metric::LineCoverage, Metric::BranchCoverage],
                &["prior-run"],
                &[],
                Prov::StalePriorResult,
                true,
                false,
                false,
                "commit:headless::mno345",
                "build:headless::ci-2044",
                "merge-sheet:headless::superseded-coverage",
            )],
        ),
        // 5. Review export — a raw / text fallback card with a missing baseline, and a
        //    merge-unavailable sheet from an unknown source flagged incompatible, both read
        //    elsewhere with the same vocabulary.
        base_row(
            M5SnapshotMergeComponentConsumerSurface::ReviewExport,
            M5TestIntelligenceQualificationClass::Stable,
            "Review export owner",
            "The review export renders the shared snapshot / golden review card so an inline snapshot with a missing baseline reads as a raw / text-fallback card rather than a settled accept, and it renders the shared coverage-import / merge sheet so a merge-unavailable sheet from an unknown source flagged incompatible reads with the same vocabulary a reviewer sees in the panel and the editor",
            "evidence:m5-review-export:001",
            vec![snapshot_case(
                Artifact::InlineSnapshot,
                Baseline::MissingBaseline,
                Diff::RawTextFallback,
                Fallback::RawTextFallback,
                &[Scope::Theme],
                1,
                Prov::Unknown,
                "snapshot-card:export::inline-missing-baseline",
                "baseline:export::inline-missing",
            )],
            vec![merge_case(
                Import::UnknownSource,
                Merge::MergeUnavailable,
                &[Metric::CombinedMetric],
                &["unknown-run"],
                &[],
                Prov::Unknown,
                false,
                true,
                false,
                "commit:export::pqr678",
                "build:export::unknown-9",
                "merge-sheet:export::merge-unavailable",
            )],
        ),
    ]
}

fn governance_review() -> M5SnapshotMergeComponentGovernanceReview {
    M5SnapshotMergeComponentGovernanceReview {
        card_shows_artifact_kind_and_baseline_identity: true,
        card_shows_diff_count: true,
        card_shows_fallback_mode_and_scope: true,
        card_offers_accept_reject_export: true,
        acceptance_never_blind_without_scope_and_fallback: true,
        merge_sheet_shows_included_and_excluded_runs: true,
        merge_sheet_shows_commit_and_build_identity: true,
        merge_sheet_shows_stale_or_incompatible_warnings: true,
        merge_sheet_shows_line_versus_branch_support: true,
        merge_never_exact_truth_with_unresolved_warnings: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_snapshot_merge_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SnapshotMergeComponentConsumerProjection {
    M5SnapshotMergeComponentConsumerProjection {
        snapshot_and_merge_surfaces_consume_shared_vocabulary: true,
        snapshot_posture_reads_single_source: true,
        merge_posture_reads_single_source: true,
        ci_and_support_read_same_snapshot_merge_vocabulary: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5SnapshotMergeComponentProofFreshness {
    M5SnapshotMergeComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SnapshotMergeComponentReleasePosture {
    M5SnapshotMergeComponentReleasePosture {
        release_packet_ref: M5_SNAPSHOT_MERGE_COMPONENTS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_SNAPSHOT_MERGE_COMPONENTS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_SCHEMA_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_MERGE_SCHEMA_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_DOC_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_SNAPSHOT_REVIEW_REF,
        M5_SNAPSHOT_MERGE_COMPONENTS_COVERAGE_MERGE_REF,
    ])
}

/// Builds the canonical M5 snapshot-merge-components packet.
pub fn seeded_m5_snapshot_merge_components_packet() -> M5SnapshotMergeComponentsPacket {
    M5SnapshotMergeComponentsPacket::new(M5SnapshotMergeComponentsPacketInput {
        packet_id: M5_SNAPSHOT_MERGE_COMPONENTS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 snapshot-review-card / coverage-import-merge-sheet primitive: controlled snapshot artifact kinds, baseline identities, diff states, render/raw fallback modes, environment/viewport/theme/serializer/locale scope, distinct matches-baseline/diff-detected/new-snapshot/obsolete/render-unavailable/raw-text-fallback review postures, controlled coverage import sources, merge-resolution states, line-versus-branch metric kinds, distinct merged-clean/shard-omission/conflicting-overlap/partial-merge/superseded/merge-unavailable merge postures, included and excluded run scope, commit/build identity, stale-or-incompatible warnings, a required scope disclosure before an acceptance decision, a required raw fallback for an opaque artifact, a required omitted-shard disclosure before exact current truth, and bounded reveal/accept-baseline/reject-change/open-raw-fallback/export and reveal/review-run-scope/open-incompatible/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5SnapshotMergeComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the snapshot-review-panel consumer is narrowed to Preview pending
/// scoped-acceptance-versus-blind-accept parity proof across every deployment line; every consumer
/// stays visible.
pub fn seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed(
) -> M5SnapshotMergeComponentsPacket {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.packet_id =
        "m5-snapshot-coverage-import-primitive:snapshot-review-panel-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5SnapshotMergeComponentConsumerSurface::SnapshotReviewPanel
        })
        .expect("snapshot-review-panel row present");
    row.qualification = M5TestIntelligenceQualificationClass::Preview;
    packet
}

/// Narrowed variant: the coverage-import / merge panel consumer is held at Beta because a slice of
/// merge surfaces do not yet render the excluded-run cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed(
) -> M5SnapshotMergeComponentsPacket {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.packet_id =
        "m5-snapshot-coverage-import-primitive:coverage-import-merge-panel-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface
                == M5SnapshotMergeComponentConsumerSurface::CoverageImportMergePanel
        })
        .expect("coverage-import-merge-panel row present");
    row.qualification = M5TestIntelligenceQualificationClass::Beta;
    packet
}
