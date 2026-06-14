use super::*;

use crate::bind_batch_review_sheets_and_action_descriptors_with_undo_class_and_policy_review::BATCH_REVIEW_SHEET_RECORD_KIND;
use crate::implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence::M5_COLLECTION_PERSISTENCE_RECORD_KIND;
use crate::implement_selection_bars_range_anchor_and_stale_snapshot_guards::SELECTION_BAR_CONTINUITY_RECORD_KIND;
use crate::ship_result_scope_counters_and_hidden_narrowing_chips::RESULT_SCOPE_COUNTER_RECORD_KIND;

const PACKET_ID: &str = "m5-collection-certification:release:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn backing_kind(dimension: CertificationProofDimension) -> &'static str {
    match dimension {
        CertificationProofDimension::FilterAst => {
            crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND
        }
        CertificationProofDimension::SavedView => M5_COLLECTION_PERSISTENCE_RECORD_KIND,
        CertificationProofDimension::ResultCount => RESULT_SCOPE_COUNTER_RECORD_KIND,
        CertificationProofDimension::SelectionScope => SELECTION_BAR_CONTINUITY_RECORD_KIND,
        CertificationProofDimension::BatchAction => BATCH_REVIEW_SHEET_RECORD_KIND,
    }
}

fn proof(
    row_id: &str,
    dimension: CertificationProofDimension,
    status: ProofStatus,
) -> CertificationProof {
    match status {
        ProofStatus::Missing => CertificationProof {
            dimension,
            status,
            backing_record_kind: None,
            proof_ref: None,
        },
        ProofStatus::Current | ProofStatus::Stale => CertificationProof {
            dimension,
            status,
            backing_record_kind: Some(backing_kind(dimension).to_owned()),
            proof_ref: Some(format!("evidence:proof:{row_id}:{}", dimension.as_str())),
        },
    }
}

fn all_current_proofs(row_id: &str) -> Vec<CertificationProof> {
    CertificationProofDimension::ALL
        .iter()
        .map(|dimension| proof(row_id, *dimension, ProofStatus::Current))
        .collect()
}

fn certified_row(
    row_id: &str,
    surface: DenseCollectionSurface,
    label: &str,
    claimed: CollectionMatrixQualificationClass,
) -> CertificationRow {
    CertificationRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        matrix_row_ref: format!("matrix-row:{row_id}"),
        claimed_qualification: claimed,
        certified_qualification: claimed,
        proofs: all_current_proofs(row_id),
        verdict: CertificationVerdict::Certified,
        regression: None,
        narrowed_label: None,
        selection_survives_by_stable_identity: true,
        provider_policy_narrowing_disclosed: true,
        visible_distinct_from_all_matching: true,
        broad_actions_preview_before_commit: true,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_COLLECTION_CERTIFICATION_DOC_REF]),
    }
}

fn auto_narrowed_support_export_row() -> CertificationRow {
    let row_id = "certification-row:support-export:0001";
    let mut proofs = all_current_proofs(row_id);
    proofs.retain(|p| p.dimension != CertificationProofDimension::BatchAction);
    proofs.push(proof(
        row_id,
        CertificationProofDimension::BatchAction,
        ProofStatus::Missing,
    ));
    CertificationRow {
        row_id: row_id.to_owned(),
        surface: DenseCollectionSurface::SupportExportProjection,
        label_summary:
            "Support/export projection whose batch-action review proof is not yet published"
                .to_owned(),
        matrix_row_ref: format!("matrix-row:{row_id}"),
        claimed_qualification: CollectionMatrixQualificationClass::Beta,
        certified_qualification: CollectionMatrixQualificationClass::Held,
        proofs,
        verdict: CertificationVerdict::AutoNarrowed,
        regression: None,
        narrowed_label: Some(
            "Batch-action review proof missing for this projected row; auto-narrowed to held until a batch review sheet is published"
                .to_owned(),
        ),
        selection_survives_by_stable_identity: true,
        provider_policy_narrowing_disclosed: true,
        visible_distinct_from_all_matching: true,
        broad_actions_preview_before_commit: true,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_COLLECTION_CERTIFICATION_DOC_REF]),
    }
}

fn blocked_provider_admin_row() -> CertificationRow {
    let row_id = "certification-row:provider-admin-table:0001";
    CertificationRow {
        row_id: row_id.to_owned(),
        surface: DenseCollectionSurface::ProviderAdminTable,
        label_summary:
            "Provider/admin table whose candidate build erased provider/policy narrowing disclosure"
                .to_owned(),
        matrix_row_ref: format!("matrix-row:{row_id}"),
        claimed_qualification: CollectionMatrixQualificationClass::Beta,
        certified_qualification: CollectionMatrixQualificationClass::Held,
        proofs: all_current_proofs(row_id),
        verdict: CertificationVerdict::Blocked,
        regression: Some(CertificationRegressionClass::ProviderPolicyNarrowingErased),
        narrowed_label: Some(
            "Candidate build hid provider/policy narrowing inside a generic filter chip; promotion blocked and the claim is held below beta"
                .to_owned(),
        ),
        selection_survives_by_stable_identity: true,
        provider_policy_narrowing_disclosed: true,
        visible_distinct_from_all_matching: true,
        broad_actions_preview_before_commit: true,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_COLLECTION_CERTIFICATION_DOC_REF]),
    }
}

fn rows() -> Vec<CertificationRow> {
    vec![
        certified_row(
            "certification-row:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            "Pipeline run list certified across filter, saved-view, count, selection, and rerun/export batch truth",
            CollectionMatrixQualificationClass::Beta,
        ),
        certified_row(
            "certification-row:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            "Provider-backed review queue certified across all-matching count, selection, and approval batch truth",
            CollectionMatrixQualificationClass::Beta,
        ),
        certified_row(
            "certification-row:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            "Incident list certified across filter, count, selection, and destructive gated delete truth",
            CollectionMatrixQualificationClass::Beta,
        ),
        certified_row(
            "certification-row:graph-list:0001",
            DenseCollectionSurface::GraphList,
            "Graph/reference list certified across explicit custom selection and local copy/export truth",
            CollectionMatrixQualificationClass::Preview,
        ),
        certified_row(
            "certification-row:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            "Marketplace results certified across provider-delegated query and mixed install/update truth",
            CollectionMatrixQualificationClass::Beta,
        ),
        certified_row(
            "certification-row:activity-rows:0001",
            DenseCollectionSurface::ActivityRows,
            "Activity rows certified across scoped free-text filter, streaming counts, and local export/copy truth",
            CollectionMatrixQualificationClass::Stable,
        ),
        certified_row(
            "certification-row:query-backed-result-set:0001",
            DenseCollectionSurface::QueryBackedResultSet,
            "Query-backed result set certified across saved-query snapshot, all-matching scope, and export/share truth",
            CollectionMatrixQualificationClass::Beta,
        ),
        blocked_provider_admin_row(),
        auto_narrowed_support_export_row(),
    ]
}

fn guardrails() -> CertificationGuardrails {
    CertificationGuardrails {
        row_highlight_never_substitutes_durable_selection: true,
        provider_policy_narrowing_never_hidden: true,
        visible_never_all_matching_without_explicit_step: true,
        broad_actions_never_bypass_preview: true,
        rows_without_current_proof_auto_narrow: true,
        regressions_block_or_visibly_narrow: true,
    }
}

fn consumer_projection() -> CertificationConsumerProjection {
    CertificationConsumerProjection {
        product_ingests_certification: true,
        docs_help_ingests_certification: true,
        accessibility_ingests_certification: true,
        release_control_ingests_certification: true,
        narrowed_rows_labeled_below_current: true,
    }
}

fn release_gate() -> CertificationReleaseGate {
    CertificationReleaseGate {
        blocks_promotion_on_blocked_row: true,
        blocks_promotion_on_uncertified_claim: true,
        stale_evidence_auto_narrows: true,
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_COLLECTION_CERTIFICATION_SCHEMA_REF,
        M5_COLLECTION_CERTIFICATION_DOC_REF,
        M5_COLLECTION_CERTIFICATION_ARTIFACT_REF,
        "schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json",
        "schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json",
        "schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json",
        "schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json",
    ])
}

fn packet() -> M5CollectionCertificationPacket {
    M5CollectionCertificationPacket::new(M5CollectionCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "M5 Dense Collection Certification".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        release_gate: release_gate(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_claimed_surface_is_present() {
    let surfaces = packet().represented_surfaces();
    for surface in DenseCollectionSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.surface != DenseCollectionSurface::QueryBackedResultSet);
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::RequiredSurfaceMissing));
}

#[test]
fn verdict_counts_are_consistent() {
    let packet = packet();
    assert_eq!(packet.certified_row_count(), 7);
    assert_eq!(packet.narrowed_row_count(), 1);
    assert_eq!(packet.blocked_row_count(), 1);
}

#[test]
fn missing_auto_narrowed_case_fails_validation() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::SupportExportProjection)
        .expect("support-export row");
    // Restore the missing batch-action proof and certify the row, erasing the
    // only auto-narrowed drill.
    row.proofs
        .retain(|p| p.dimension != CertificationProofDimension::BatchAction);
    row.proofs.push(CertificationProof {
        dimension: CertificationProofDimension::BatchAction,
        status: ProofStatus::Current,
        backing_record_kind: Some(BATCH_REVIEW_SHEET_RECORD_KIND.to_owned()),
        proof_ref: Some("evidence:proof:support-export:batch".to_owned()),
    });
    row.verdict = CertificationVerdict::Certified;
    row.certified_qualification = row.claimed_qualification;
    row.narrowed_label = None;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::AutoNarrowedCaseMissing));
}

#[test]
fn missing_blocked_case_fails_validation() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::ProviderAdminTable)
        .expect("provider-admin row");
    row.verdict = CertificationVerdict::Certified;
    row.certified_qualification = row.claimed_qualification;
    row.regression = None;
    row.narrowed_label = None;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::BlockedRegressionCaseMissing));
}

#[test]
fn uncovered_claim_without_narrowing_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::SupportExportProjection)
        .expect("support-export row");
    // Keep the missing proof but pretend the claim still holds at beta.
    row.certified_qualification = row.claimed_qualification;
    let violations = packet.validate();
    assert!(violations.contains(&M5CollectionCertificationViolation::UncoveredClaimNotNarrowed));
}

#[test]
fn blocked_row_without_regression_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::ProviderAdminTable)
        .expect("provider-admin row");
    row.regression = None;
    let violations = packet.validate();
    assert!(violations.contains(&M5CollectionCertificationViolation::BlockedRowMissingRegression));
}

#[test]
fn narrowed_row_with_generic_label_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == DenseCollectionSurface::SupportExportProjection)
        .expect("support-export row");
    row.narrowed_label = Some("narrowed".to_owned());
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::NarrowedRowMissingLabel));
}

#[test]
fn proof_dimension_missing_fails() {
    let mut packet = packet();
    packet.rows[0]
        .proofs
        .retain(|p| p.dimension != CertificationProofDimension::ResultCount);
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::ProofDimensionMissing));
}

#[test]
fn selection_not_durable_fails() {
    let mut packet = packet();
    packet.rows[0].selection_survives_by_stable_identity = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::SelectionNotDurableByStableIdentity));
}

#[test]
fn hidden_provider_policy_narrowing_fails() {
    let mut packet = packet();
    packet.rows[0].provider_policy_narrowing_disclosed = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::ProviderPolicyNarrowingHidden));
}

#[test]
fn visible_treated_as_all_matching_fails() {
    let mut packet = packet();
    packet.rows[1].visible_distinct_from_all_matching = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::VisibleTreatedAsAllMatching));
}

#[test]
fn broad_action_bypassing_preview_fails() {
    let mut packet = packet();
    packet.rows[2].broad_actions_preview_before_commit = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::BroadActionBypassesPreview));
}

#[test]
fn current_proof_without_backing_kind_fails() {
    let mut packet = packet();
    packet.rows[0].proofs[0].backing_record_kind = None;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::RowIncomplete));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != M5_COLLECTION_CERTIFICATION_DOC_REF);
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.regressions_block_or_visibly_narrow = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .accessibility_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn non_enforcing_release_gate_fails() {
    let mut packet = packet();
    packet.release_gate.blocks_promotion_on_blocked_row = false;
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::ReleaseGateNotEnforcing));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CollectionCertificationViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5CollectionCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Dense Collection Certification"));
    assert!(summary.contains("pipeline_run_list"));
    assert!(summary.contains("Narrowed:"));
    assert!(summary.contains("Regression:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_collection_certification_export()
        .expect("checked collection certification export validates");
    assert_eq!(checked, packet());
}
