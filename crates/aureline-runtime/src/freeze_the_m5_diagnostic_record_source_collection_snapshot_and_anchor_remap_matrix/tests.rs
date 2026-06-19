use super::*;

use crate::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticSourceKind,
};
use crate::quality::QualitySessionOutcomeClass;

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

#[test]
fn diagnostic_truth_lane_packet_validates() {
    assert!(packet().validate().is_empty());
}

#[test]
fn every_claimed_surface_is_present() {
    let represented = packet().represented_surfaces();
    for surface in M5DiagnosticSurface::ALL {
        assert!(represented.contains(&surface), "missing {surface:?}");
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.surface != M5DiagnosticSurface::EditorStructuralDiagnostics);
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::RequiredSurfaceMissing));
}

#[test]
fn auto_downgrade_case_is_present() {
    assert_eq!(packet().downgraded_row_count(), 1);
}

#[test]
fn missing_downgraded_case_fails_validation() {
    let mut packet = packet();
    packet.rows.retain(|row| !row.needs_downgrade());
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::DowngradedRowCaseMissing));
}

#[test]
fn unidentified_dimension_without_downgrade_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5DiagnosticSurface::NotebookCellDiagnostics)
        .expect("notebook row");
    row.source_kind = None;
    // Leaves effective == claimed despite an unidentified lane dimension.
    assert!(row.needs_downgrade());
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticTruthLaneViolation::RowNotDowngradedOnUnidentifiedLane));
}

#[test]
fn unproven_freshness_forces_downgrade() {
    let mut row = row(
        "diag-row:notebook-cell:0001",
        M5DiagnosticSurface::NotebookCellDiagnostics,
        "Notebook row whose freshness is unverified",
        Some(DiagnosticSourceKind::RuntimeOrTest),
        Some(DiagnosticOriginClass::LiveLocalSession),
        Some(DiagnosticFreshnessClass::Unverified),
        Some(DiagnosticAnchorRemapStateClass::Exact),
        Some(DiagnosticCollectionCompletenessClass::CompleteEnumeration),
        DiagnosticClusterMeaningClass::NoClustering,
        Some(QualitySessionOutcomeClass::Applied),
        DiagnosticLaneQualificationClass::Beta,
    );
    assert!(row.needs_downgrade());
    // Mark the downgrade properly so only the freshness rule is exercised.
    row.effective_qualification = DiagnosticLaneQualificationClass::Held;
    row.downgrade_trigger = Some(DiagnosticLaneDowngradeTrigger::UnprovenFreshness);
    row.degraded_label = Some("Freshness could not be proven for this scope".to_owned());
    assert!(row.downgrade_consistent());
}

#[test]
fn unknown_collection_completeness_forces_downgrade() {
    let row = row(
        "diag-row:data-tooling:0002",
        M5DiagnosticSurface::DataToolingDiagnostics,
        "Data row whose collection completeness is unknown",
        Some(DiagnosticSourceKind::BuildOrTask),
        Some(DiagnosticOriginClass::LiveLocalSession),
        Some(DiagnosticFreshnessClass::Current),
        Some(DiagnosticAnchorRemapStateClass::Exact),
        Some(DiagnosticCollectionCompletenessClass::UnknownRequiresReview),
        DiagnosticClusterMeaningClass::NoClustering,
        Some(QualitySessionOutcomeClass::Applied),
        DiagnosticLaneQualificationClass::Beta,
    );
    assert!(row.needs_downgrade());
}

#[test]
fn imported_shown_as_live_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5DiagnosticSurface::ImportedScannerDiagnostics)
        .expect("imported scanner row");
    row.imported_not_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::ImportedShownAsLive));
}

#[test]
fn hidden_freshness_or_remap_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5DiagnosticSurface::PreviewRuntimeDiagnostics)
        .expect("preview runtime row");
    // Contextual remap requires disclosure.
    row.freshness_and_remap_disclosed = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::FreshnessOrRemapHidden));
}

#[test]
fn hidden_collection_completeness_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5DiagnosticSurface::NotebookCellDiagnostics)
        .expect("notebook row");
    // Partial-visible scan requires disclosure.
    row.collection_completeness_disclosed = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::CollectionCompletenessHidden));
}

#[test]
fn clustering_erasing_provenance_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5DiagnosticSurface::FrameworkPackDiagnostics)
        .expect("framework row");
    // Cross-source corroboration groups multiple findings.
    row.provenance_preserved_in_clustering = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::ClusteringErasesProvenance));
}

#[test]
fn silent_anchor_repair_fails() {
    let mut packet = packet();
    packet.rows[0].anchor_remap_append_only = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::AnchorRemapNotAppendOnly));
}

#[test]
fn dropped_target_env_refs_fail() {
    let mut packet = packet();
    packet.rows[0].target_environment_refs_preserved = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::TargetEnvironmentRefsDropped));
}

#[test]
fn untyped_mutating_fix_fails() {
    let mut packet = packet();
    packet.rows[0].mutating_fix_is_typed_proposal = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::MutatingFixNotTypedProposal));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.anchors_never_silently_repaired = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet.consumer_projection.support_export_ingests_lane = false;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_evidence_freshness_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.needs_downgrade())
        .expect("downgraded row");
    row.degraded_label = Some("unavailable".to_owned());
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::DowngradedRowMissingLabelOrTrigger));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&DiagnosticTruthLaneViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: DiagnosticTruthLaneMatrixPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Diagnostic-Truth Lane Matrix"));
    assert!(summary.contains("notebook_cell_diagnostics"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_diagnostic_truth_lane_export()
        .expect("checked diagnostic-truth lane export validates");
    assert_eq!(checked, packet());
}
