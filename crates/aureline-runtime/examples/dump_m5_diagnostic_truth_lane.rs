//! Conformance dump for the M5 diagnostic-truth lane matrix packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticSourceKind,
};
use aureline_runtime::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::*;
use aureline_runtime::quality::QualitySessionOutcomeClass;

const PACKET_ID: &str = "m5-diagnostic-truth-lane:stable:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: M5DiagnosticSurface,
    label: &str,
    source_kind: Option<DiagnosticSourceKind>,
    origin_class: Option<DiagnosticOriginClass>,
    freshness_class: Option<DiagnosticFreshnessClass>,
    remap_state_class: Option<DiagnosticAnchorRemapStateClass>,
    collection_completeness_class: Option<DiagnosticCollectionCompletenessClass>,
    cluster_meaning_class: DiagnosticClusterMeaningClass,
    quality_session_outcome_class: Option<QualitySessionOutcomeClass>,
    claimed: DiagnosticLaneQualificationClass,
) -> DiagnosticLaneRow {
    DiagnosticLaneRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        source_kind,
        origin_class,
        freshness_class,
        remap_state_class,
        collection_completeness_class,
        cluster_meaning_class,
        quality_session_outcome_class,
        provenance_preserved_in_clustering: true,
        imported_not_shown_as_live: true,
        freshness_and_remap_disclosed: true,
        anchor_remap_append_only: true,
        collection_completeness_disclosed: true,
        target_environment_refs_preserved: true,
        mutating_fix_is_typed_proposal: true,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_DIAGNOSTIC_TRUTH_LANE_DOC_REF]),
    }
}

fn downgraded_data_tooling_row() -> DiagnosticLaneRow {
    let mut data_row = row(
        "diag-row:data-tooling:0001",
        M5DiagnosticSurface::DataToolingDiagnostics,
        "Data-tooling findings whose mutating fix routes are not yet bound to a governed quality session",
        Some(DiagnosticSourceKind::BuildOrTask),
        Some(DiagnosticOriginClass::LiveLocalSession),
        Some(DiagnosticFreshnessClass::Recent),
        Some(DiagnosticAnchorRemapStateClass::Exact),
        Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
        DiagnosticClusterMeaningClass::NoClustering,
        None,
        DiagnosticLaneQualificationClass::Beta,
    );
    data_row.effective_qualification = DiagnosticLaneQualificationClass::Held;
    data_row.downgrade_trigger = Some(DiagnosticLaneDowngradeTrigger::UnlinkedQualitySession);
    data_row.degraded_label = Some(
        "No governing quality session yet binds the data-tooling fix routes; held below preview until a quality-session outcome and rollback boundary are published"
            .to_owned(),
    );
    data_row
}

fn rows() -> Vec<DiagnosticLaneRow> {
    vec![
        row(
            "diag-row:notebook-cell:0001",
            M5DiagnosticSurface::NotebookCellDiagnostics,
            "Notebook cell diagnostics from a live local run with partial-but-visible cell discovery",
            Some(DiagnosticSourceKind::RuntimeOrTest),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Exact),
            Some(DiagnosticCollectionCompletenessClass::PartialVisibleScan),
            DiagnosticClusterMeaningClass::NoClustering,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Beta,
        ),
        row(
            "diag-row:framework-pack:0001",
            M5DiagnosticSurface::FrameworkPackDiagnostics,
            "Framework-pack analyzer findings corroborated across the language and framework sources",
            Some(DiagnosticSourceKind::LanguageService),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Exact),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::CrossSourceCorroboration,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Beta,
        ),
        row(
            "diag-row:request-tooling:0001",
            M5DiagnosticSurface::RequestToolingDiagnostics,
            "Request/API tooling assertions whose fixes preview a diff before explicit apply",
            Some(DiagnosticSourceKind::BuildOrTask),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Exact),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::NoClustering,
            Some(QualitySessionOutcomeClass::PreviewRequired),
            DiagnosticLaneQualificationClass::Beta,
        ),
        downgraded_data_tooling_row(),
        row(
            "diag-row:preview-runtime:0001",
            M5DiagnosticSurface::PreviewRuntimeDiagnostics,
            "Preview-runtime render/drift findings whose ranges are contextually remapped after re-render",
            Some(DiagnosticSourceKind::BuildOrTask),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Contextual),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::RelatedByLocation,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Beta,
        ),
        row(
            "diag-row:package-lane:0001",
            M5DiagnosticSurface::PackageLaneDiagnostics,
            "Package-lane policy findings that are not line-anchored and disclose an unmapped range",
            Some(DiagnosticSourceKind::Policy),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Unmapped),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::NoClustering,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Beta,
        ),
        row(
            "diag-row:language-provider:0001",
            M5DiagnosticSurface::LanguageProviderDiagnostics,
            "Language-service findings with exact anchors and exact-duplicate clustering that preserves provenance",
            Some(DiagnosticSourceKind::LanguageService),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Exact),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::ExactDuplicate,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Stable,
        ),
        row(
            "diag-row:editor-structural:0001",
            M5DiagnosticSurface::EditorStructuralDiagnostics,
            "Editor-structural parser/encoding guards with a complete local enumeration and exact anchors",
            Some(DiagnosticSourceKind::EditorStructural),
            Some(DiagnosticOriginClass::LiveLocalSession),
            Some(DiagnosticFreshnessClass::Current),
            Some(DiagnosticAnchorRemapStateClass::Exact),
            Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
            DiagnosticClusterMeaningClass::NoClustering,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Stable,
        ),
        row(
            "diag-row:imported-scanner:0001",
            M5DiagnosticSurface::ImportedScannerDiagnostics,
            "Imported scanner snapshot held read-only with imported class, static anchors, and a display roll-up",
            Some(DiagnosticSourceKind::ScannerImport),
            Some(DiagnosticOriginClass::ImportedSnapshot),
            Some(DiagnosticFreshnessClass::ImportedSnapshot),
            Some(DiagnosticAnchorRemapStateClass::ImportedStatic),
            Some(DiagnosticCollectionCompletenessClass::ImportedSnapshotSet),
            DiagnosticClusterMeaningClass::DisplayRollupOnly,
            Some(QualitySessionOutcomeClass::Applied),
            DiagnosticLaneQualificationClass::Beta,
        ),
    ]
}

fn guardrails() -> DiagnosticLaneGuardrails {
    DiagnosticLaneGuardrails {
        unlike_sources_never_flattened: true,
        anchors_never_silently_repaired: true,
        clustering_never_erases_class: true,
        imported_live_class_explicit: true,
        freshness_and_remap_explicit: true,
        collection_completeness_exportable: true,
        mutating_fixes_are_typed_proposals: true,
        rows_auto_downgrade_on_unidentified_lane: true,
    }
}

fn consumer_projection() -> DiagnosticLaneConsumerProjection {
    DiagnosticLaneConsumerProjection {
        editor_ingests_lane: true,
        problems_ingests_lane: true,
        review_ingests_lane: true,
        cli_headless_ingests_lane: true,
        ai_evidence_ingests_lane: true,
        support_export_ingests_lane: true,
        downgraded_rows_labeled_below_current: true,
    }
}

fn evidence_freshness() -> DiagnosticLaneEvidenceFreshness {
    DiagnosticLaneEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_downgrade_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_REF,
        M5_DIAGNOSTIC_TRUTH_LANE_DOC_REF,
        M5_DIAGNOSTIC_TRUTH_LANE_ARTIFACT_REF,
        "schemas/quality/quality_session.schema.json",
        "schemas/quality/quality_action_proposal.schema.json",
        "schemas/quality/scanner_import_session.schema.json",
    ])
}

fn packet() -> DiagnosticTruthLaneMatrixPacket {
    DiagnosticTruthLaneMatrixPacket::new(DiagnosticTruthLaneMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Diagnostic-Truth Lane Matrix".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
