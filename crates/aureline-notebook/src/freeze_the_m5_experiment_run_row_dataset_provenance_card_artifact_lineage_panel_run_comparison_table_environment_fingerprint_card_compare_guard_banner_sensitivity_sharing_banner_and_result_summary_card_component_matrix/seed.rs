//! Canonical seed builders for the frozen M5 experiment-component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical experiment-component matrix.
pub const M5_EXPERIMENT_COMPONENT_MATRIX_PACKET_ID: &str = "m5-experiment-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5ExperimentRequiredLabel> {
    M5ExperimentRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5ExperimentRequiredLabel]) -> Vec<M5ExperimentRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5ExperimentComponentFamily,
    qualification: M5ExperimentQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5ExperimentComponentRow {
    M5ExperimentComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        run_origin_kinds: vec![],
        run_status_states: vec![],
        dataset_source_classes: vec![],
        dataset_provenance_states: vec![],
        artifact_kind_classes: vec![],
        lineage_states: vec![],
        comparison_axis_classes: vec![],
        comparability_states: vec![],
        fingerprint_scope_classes: vec![],
        fingerprint_states: vec![],
        compare_guard_reasons: vec![],
        compare_guard_states: vec![],
        sensitivity_classes: vec![],
        share_scope_states: vec![],
        summary_content_classes: vec![],
        summary_export_scopes: vec![],
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ExperimentConsumerSurface::NotebookUi,
            M5ExperimentConsumerSurface::SupportExport,
            M5ExperimentConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ExperimentDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_provenance_or_sensitivity_state: false,
        hides_run_origin_or_revision: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

fn component_rows() -> Vec<M5ExperimentComponentRow> {
    use M5ArtifactKindClass as AK;
    use M5ComparabilityState as CS;
    use M5CompareGuardReason as GR;
    use M5CompareGuardState as GS;
    use M5ComparisonAxisClass as CA;
    use M5DatasetProvenanceState as DP;
    use M5DatasetSourceClass as DS;
    use M5ExperimentComponentFamily as F;
    use M5ExperimentConsumerSurface as C;
    use M5ExperimentDisposition as DI;
    use M5ExperimentDowngradeTrigger as D;
    use M5ExperimentQualificationClass as Q;
    use M5ExperimentRequiredLabel as L;
    use M5FingerprintScopeClass as FS;
    use M5FingerprintState as FT;
    use M5LineageState as LS;
    use M5RunOriginKind as RO;
    use M5RunStatusState as RS;
    use M5SensitivityClass as SC;
    use M5ShareScopeState as SS;
    use M5SummaryContentClass as UC;
    use M5SummaryExportScope as UX;

    let mut rows = Vec::new();

    // 1. Experiment run row.
    let mut row = base_row(
        F::ExperimentRunRow,
        Q::Stable,
        "Experiment run row owner",
        "One experiment-run-row model naming where a run came from (a notebook cell, a script task, a scheduled task, a manual attach, an imported run, or an unknown origin) and where it stands (queued, running, succeeded, failed, canceled, or stale), so a run never hides its notebook / script / task origin, its code revision, or that a run was imported or manually attached",
        "evidence:m5-experiment-run-row-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_EXPERIMENT_RUN_ROW_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::LocalRun,
        DI::ManagedRun,
        DI::ImportedRun,
        DI::ManualAttach,
    ];
    row.run_origin_kinds = RO::ALL.to_vec();
    row.run_status_states = RS::ALL.to_vec();
    row.required_labels = labels_with(&[L::RunOriginAndRevision]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::ExperimentDashboardUi,
        C::ComparisonUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RunOriginUnstated,
        D::CodeRevisionUnstated,
        D::ImportedRunUnmarked,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Dataset provenance card.
    let mut row = base_row(
        F::DatasetProvenanceCard,
        Q::Stable,
        "Dataset provenance card owner",
        "One dataset-provenance-card model naming what data a run used (a tracked dataset, a local file, a remote snapshot, synthetic data, a redacted sample, or an unknown source) and how completely it is provenanced (complete, partial, missing, version pinned, version drifted, or access restricted), so a card never severs its canonical provenance and never masks a restricted or drifted dataset",
        "evidence:m5-dataset-provenance-card-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_DATASET_PROVENANCE_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::ImportedRun, DI::ContextIncomplete];
    row.dataset_source_classes = DS::ALL.to_vec();
    row.dataset_provenance_states = DP::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndSensitivity]);
    row.consumer_surfaces = vec![
        C::DataCatalogUi,
        C::NotebookUi,
        C::LineageUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DatasetProvenanceSevered,
        D::SensitivityClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Artifact lineage panel.
    let mut row = base_row(
        F::ArtifactLineagePanel,
        Q::Stable,
        "Artifact lineage panel owner",
        "One artifact-lineage-panel model naming what a generated artifact is (a model checkpoint, a metrics table, a plot or figure, an exported report, a log bundle, or an unknown artifact) and how completely its lineage resolves (complete, partial, broken, derived from a known upstream, derived from an unknown upstream, or regenerated), so a panel never hides a broken or unknown lineage and always names its upstream and downstream",
        "evidence:m5-artifact-lineage-panel-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Reproducible, DI::NeedsRerun];
    row.artifact_kind_classes = AK::ALL.to_vec();
    row.lineage_states = LS::ALL.to_vec();
    row.required_labels = labels_with(&[L::RunOriginAndRevision]);
    row.consumer_surfaces = vec![
        C::LineageUi,
        C::NotebookUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RunOriginUnstated,
        D::CachedStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Run comparison table.
    let mut row = base_row(
        F::RunComparisonTable,
        Q::Stable,
        "Run comparison table owner",
        "One run-comparison-table model naming along which axis it compares runs (a metric delta, a parameter diff, a dataset diff, an environment diff, a code revision diff, or an artifact diff) and whether two runs are actually comparable (comparable, comparable with caveats, not comparable, confounded, insufficient overlap, or unknown comparability), so a table never implies an apples-to-apples comparison without parity evidence and always discloses confounders",
        "evidence:m5-run-comparison-table-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_RUN_COMPARISON_TABLE_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::LikelyReproducible,
        DI::NeedsRerun,
        DI::ContextIncomplete,
    ];
    row.comparison_axis_classes = CA::ALL.to_vec();
    row.comparability_states = CS::ALL.to_vec();
    row.required_labels = labels_with(&[L::RunOriginAndRevision]);
    row.consumer_surfaces = vec![
        C::ComparisonUi,
        C::ExperimentDashboardUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ComparabilityOverstated,
        D::CodeRevisionUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Environment fingerprint card.
    let mut row = base_row(
        F::EnvironmentFingerprintCard,
        Q::Stable,
        "Environment fingerprint card owner",
        "One environment-fingerprint-card model naming which slice of the environment it captures (the interpreter, the kernel spec, the installed packages, the hardware accelerator, the OS or platform, or the container image) and how completely it was captured (captured complete, captured partial, captured missing, pinned, drifted, or unavailable), so a card never leaves the environment behind a result implicit and never hides a drifted or missing capture",
        "evidence:m5-environment-fingerprint-card-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Reproducible, DI::LikelyReproducible];
    row.fingerprint_scope_classes = FS::ALL.to_vec();
    row.fingerprint_states = FT::ALL.to_vec();
    row.required_labels = labels_with(&[L::RunOriginAndRevision]);
    row.consumer_surfaces = vec![
        C::NotebookUi,
        C::ExperimentDashboardUi,
        C::ComparisonUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EnvironmentFingerprintUnstated,
        D::CodeRevisionUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Compare guard banner.
    let mut row = base_row(
        F::CompareGuardBanner,
        Q::Stable,
        "Compare guard banner owner",
        "One compare-guard-banner model naming why a comparison is guarded (a dataset mismatch, environment drift, a code revision gap, a metric definition change, a sample size imbalance, or a confounder present) and what the guard permits (comparison permitted, comparison caveated, comparison blocked, guard acknowledged, guard overridden by choice, or guard unavailable), so a banner never silently allows an apples-to-apples comparison the guard should block",
        "evidence:m5-compare-guard-banner-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_COMPARE_GUARD_BANNER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::NeedsRerun, DI::ContextIncomplete];
    row.compare_guard_reasons = GR::ALL.to_vec();
    row.compare_guard_states = GS::ALL.to_vec();
    row.required_labels = labels_with(&[L::RunOriginAndRevision]);
    row.consumer_surfaces = vec![
        C::ComparisonUi,
        C::ReviewUi,
        C::ExperimentDashboardUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ComparabilityOverstated,
        D::EnvironmentFingerprintUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Sensitivity / sharing banner.
    let mut row = base_row(
        F::SensitivitySharingBanner,
        Q::Stable,
        "Sensitivity sharing banner owner",
        "One sensitivity-sharing-banner model naming how sensitive a result or dataset is (public-safe, internal, confidential, regulated, production-like, or unknown sensitivity) and what a share will actually include (summary only, summary plus metadata, evidence included, raw payload included, a redacted share, or share blocked), so a banner never leaves sensitivity implicit before a share and never exposes raw production-like data by default",
        "evidence:m5-sensitivity-sharing-banner-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::ManagedRun, DI::LocalRun];
    row.sensitivity_classes = SC::ALL.to_vec();
    row.share_scope_states = SS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndSensitivity, L::ExportScope]);
    row.consumer_surfaces = vec![
        C::ReviewUi,
        C::DataCatalogUi,
        C::ExperimentDashboardUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SensitivityClassUnstated,
        D::RawPayloadExposedByDefault,
        D::ExportScopeUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Result summary card.
    let mut row = base_row(
        F::ResultSummaryCard,
        Q::Stable,
        "Result summary card owner",
        "One result-summary-card model naming what it is showing (a headline metric, a metric table, a narrative summary, an evidence link, a raw payload reference, or no result) and what scope it exports (summary, metadata, evidence, raw, redacted, or export withheld), so a shared summary never silently widens from summary to raw scope and always names whether it includes evidence or only metadata",
        "evidence:m5-result-summary-card-parity:001",
        &[M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_RESULT_SUMMARY_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Reproducible, DI::ContextIncomplete];
    row.summary_content_classes = UC::ALL.to_vec();
    row.summary_export_scopes = UX::ALL.to_vec();
    row.required_labels = labels_with(&[L::ExportScope]);
    row.consumer_surfaces = vec![
        C::ReviewUi,
        C::ExperimentDashboardUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExportScopeUnstated,
        D::RawPayloadExposedByDefault,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ExperimentComponentGovernanceReview {
    M5ExperimentComponentGovernanceReview {
        run_row_shows_origin_and_revision: true,
        dataset_card_shows_provenance_and_source: true,
        lineage_panel_shows_upstream_and_downstream: true,
        comparison_table_shows_comparability_and_confounders: true,
        fingerprint_card_shows_environment_capture: true,
        compare_guard_shows_reason_and_state: true,
        sensitivity_banner_shows_class_and_share_scope: true,
        result_summary_shows_export_scope: true,
        no_surface_invents_alternate_state_label: true,
        comparison_never_implies_apples_to_apples_without_parity: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        run_identity_and_revision_always_explicit: true,
        sensitivity_and_provenance_always_visible: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ExperimentComponentConsumerProjection {
    M5ExperimentComponentConsumerProjection {
        notebook_surfaces_consume_run_row_and_fingerprint_vocabulary: true,
        comparison_surfaces_consume_comparability_and_guard_vocabulary: true,
        data_surfaces_consume_provenance_and_lineage_vocabulary: true,
        share_surfaces_consume_sensitivity_and_export_scope_vocabulary: true,
        result_surfaces_consume_summary_scope_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ExperimentComponentProofFreshness {
    M5ExperimentComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ExperimentComponentReleasePosture {
    M5ExperimentComponentReleasePosture {
        proof_packet_ref: M5_EXPERIMENT_COMPONENT_ARTIFACT_REF.to_owned(),
        experiment_component_audit_ref: M5_EXPERIMENT_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
        M5_DATASET_PROVENANCE_CARD_SCHEMA_REF,
        M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF,
        M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
        M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF,
        M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
        M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 experiment-component matrix packet.
pub fn seeded_m5_experiment_component_matrix() -> M5ExperimentComponentMatrixPacket {
    M5ExperimentComponentMatrixPacket::new(M5ExperimentComponentMatrixPacketInput {
        packet_id: M5_EXPERIMENT_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 experiment-run-row, dataset-provenance-card, artifact-lineage-panel, run-comparison-table, environment-fingerprint-card, compare-guard-banner, sensitivity-sharing-banner, and result-summary-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5ExperimentComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run comparison table is held at Beta because parity evidence for a
/// slice of the comparability disclosures does not yet round-trip across every comparison
/// surface; every component stays visible.
pub fn seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed(
) -> M5ExperimentComponentMatrixPacket {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.packet_id = "m5-experiment-components:run-comparison-table-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ExperimentComponentFamily::RunComparisonTable)
        .expect("run-comparison-table row present");
    row.qualification = M5ExperimentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sensitivity / sharing banner is narrowed to Preview pending
/// redaction and export-scope parity proof across every share surface; every component stays
/// visible.
pub fn seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed(
) -> M5ExperimentComponentMatrixPacket {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.packet_id =
        "m5-experiment-components:sensitivity-sharing-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ExperimentComponentFamily::SensitivitySharingBanner)
        .expect("sensitivity-sharing-banner row present");
    row.qualification = M5ExperimentQualificationClass::Preview;
    packet
}
