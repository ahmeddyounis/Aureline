//! Conformance dump for the M5 dense-collection qualification matrix packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::*;
use aureline_collections::stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm;
use aureline_collections::stabilize_selection_scope_and_batch_result_truth::SelectionScopeClass;

const PACKET_ID: &str = "m5-collection-qualification-matrix:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

const REQUIRED_SCOPE_VOCABULARY_TERMS: [ScopeCounterVocabularyTerm; 8] = [
    ScopeCounterVocabularyTerm::Visible,
    ScopeCounterVocabularyTerm::Loaded,
    ScopeCounterVocabularyTerm::Matching,
    ScopeCounterVocabularyTerm::Selected,
    ScopeCounterVocabularyTerm::Approx,
    ScopeCounterVocabularyTerm::Exact,
    ScopeCounterVocabularyTerm::HiddenByPolicy,
    ScopeCounterVocabularyTerm::OutsideCurrentFilter,
];

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn descriptor(
    action_id: &str,
    action_kind: BatchActionKind,
    mutates_state: bool,
    provider_backed: bool,
    reversible: bool,
) -> BatchActionDescriptor {
    BatchActionDescriptor {
        action_id: action_id.to_owned(),
        action_kind,
        mutates_state,
        provider_backed,
        reversible,
        requires_preview: true,
        scope_receipt_required: true,
    }
}

fn saved_view(saved_view_id: &str) -> SavedViewDeclaration {
    SavedViewDeclaration {
        saved_view_id: saved_view_id.to_owned(),
        owner_scope_token: "workspace".to_owned(),
        privacy_class_token: "shared_redacted".to_owned(),
        fallback_behavior_token: "preserve_and_label_degraded".to_owned(),
        captures_selection: false,
        captures_provider_cursor: false,
        reopen_rebind_supported: true,
    }
}

fn column_preset(column_preset_id: &str) -> ColumnPresetDeclaration {
    ColumnPresetDeclaration {
        column_preset_id: column_preset_id.to_owned(),
        visible_column_ids: refs(&["identity", "owner", "state", "updated"]),
        required_identity_column_ids: refs(&["identity", "owner"]),
        pinned_column_ids: refs(&["identity"]),
        density_mode_token: "compact".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: DenseCollectionSurface,
    label: &str,
    filter_ast_class: Option<FilterAstClass>,
    selection_scope_class: Option<SelectionScopeClass>,
    result_counter_class: Option<ResultCounterClass>,
    batch_action_class: Option<BatchActionScopeClass>,
    claimed: CollectionMatrixQualificationClass,
    batch_action_descriptors: Vec<BatchActionDescriptor>,
) -> CollectionQualificationRow {
    CollectionQualificationRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        filter_ast_class,
        selection_scope_class,
        result_counter_class,
        batch_action_class,
        scope_vocabulary_terms: REQUIRED_SCOPE_VOCABULARY_TERMS.to_vec(),
        saved_view: saved_view(&format!("view:{row_id}")),
        column_preset: column_preset(&format!("columns:{row_id}")),
        batch_action_descriptors,
        selection_survives_by_stable_identity: true,
        provider_policy_narrowing_disclosed: true,
        visible_distinct_from_all_matching: true,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: refs(&[M5_COLLECTION_QUALIFICATION_MATRIX_DOC_REF]),
    }
}

fn downgraded_support_export_row() -> CollectionQualificationRow {
    let mut export_row = row(
        "collection-row:support-export:0001",
        DenseCollectionSurface::SupportExportProjection,
        "Support/export projection of a collection row whose batch-action scope class is not yet identified",
        Some(FilterAstClass::SavedQuerySnapshot),
        Some(SelectionScopeClass::ExplicitCustomSet),
        Some(ResultCounterClass::ExactCount),
        None,
        CollectionMatrixQualificationClass::Beta,
        Vec::new(),
    );
    export_row.effective_qualification = CollectionMatrixQualificationClass::Held;
    export_row.downgrade_trigger = Some(CollectionMatrixDowngradeTrigger::UnidentifiedBatchAction);
    export_row.degraded_label = Some(
        "Batch-action scope class not yet identified for this projected row; held below preview until a batch-action descriptor is published"
            .to_owned(),
    );
    export_row
}

fn rows() -> Vec<CollectionQualificationRow> {
    vec![
        row(
            "collection-row:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            "Pipeline run list with a typed clause filter, loaded-set selection, and local rerun/export",
            Some(FilterAstClass::TypedClauseAst),
            Some(SelectionScopeClass::LoadedSet),
            Some(ResultCounterClass::ExactCount),
            Some(BatchActionScopeClass::LocalReversibleBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("pipeline.rerun", BatchActionKind::Rerun, true, false, true),
                descriptor("pipeline.export", BatchActionKind::Export, false, false, true),
            ],
        ),
        row(
            "collection-row:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            "Provider-backed review queue with all-matching query scope and provider-authoritative approval",
            Some(FilterAstClass::TypedClauseAst),
            Some(SelectionScopeClass::AllMatchingQuery),
            Some(ResultCounterClass::ProviderLimitedCount),
            Some(BatchActionScopeClass::ProviderAuthoritativeBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("review.approve", BatchActionKind::Approve, true, true, false),
                descriptor("review.suppress", BatchActionKind::Suppress, true, true, true),
            ],
        ),
        row(
            "collection-row:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            "Incident list with a typed clause filter, visible-range selection, and destructive gated delete",
            Some(FilterAstClass::TypedClauseAst),
            Some(SelectionScopeClass::VisibleRange),
            Some(ResultCounterClass::ApproximateCount),
            Some(BatchActionScopeClass::DestructiveGatedBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("incident.suppress", BatchActionKind::Suppress, true, false, true),
                descriptor("incident.delete", BatchActionKind::Delete, true, false, false),
            ],
        ),
        row(
            "collection-row:graph-list:0001",
            DenseCollectionSurface::GraphList,
            "Graph/reference list with an explicit custom selection and local copy/export",
            Some(FilterAstClass::TypedClauseAst),
            Some(SelectionScopeClass::ExplicitCustomSet),
            Some(ResultCounterClass::ExactCount),
            Some(BatchActionScopeClass::LocalReversibleBatch),
            CollectionMatrixQualificationClass::Preview,
            vec![
                descriptor("graph.copy", BatchActionKind::Copy, false, false, true),
                descriptor("graph.export", BatchActionKind::Export, false, false, true),
            ],
        ),
        row(
            "collection-row:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            "Marketplace results with a provider-delegated query and mixed client/provider install/update",
            Some(FilterAstClass::ProviderDelegatedQuery),
            Some(SelectionScopeClass::LoadedSet),
            Some(ResultCounterClass::ProviderLimitedCount),
            Some(BatchActionScopeClass::MixedClientProviderBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("marketplace.install", BatchActionKind::Install, true, true, true),
                descriptor("marketplace.update", BatchActionKind::Update, true, true, true),
            ],
        ),
        row(
            "collection-row:activity-rows:0001",
            DenseCollectionSurface::ActivityRows,
            "Activity rows with a scoped free-text filter, streaming counts, and local export/copy",
            Some(FilterAstClass::FreeTextScoped),
            Some(SelectionScopeClass::LoadedSet),
            Some(ResultCounterClass::PartialStreamingCount),
            Some(BatchActionScopeClass::LocalReversibleBatch),
            CollectionMatrixQualificationClass::Stable,
            vec![
                descriptor("activity.export", BatchActionKind::Export, false, false, true),
                descriptor("activity.copy", BatchActionKind::Copy, false, false, true),
            ],
        ),
        row(
            "collection-row:provider-admin-table:0001",
            DenseCollectionSurface::ProviderAdminTable,
            "Provider/admin table with a provider-delegated query and provider-authoritative update/delete",
            Some(FilterAstClass::ProviderDelegatedQuery),
            Some(SelectionScopeClass::AllMatchingQuery),
            Some(ResultCounterClass::ProviderLimitedCount),
            Some(BatchActionScopeClass::ProviderAuthoritativeBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("admin.update", BatchActionKind::Update, true, true, true),
                descriptor("admin.delete", BatchActionKind::Delete, true, true, false),
            ],
        ),
        row(
            "collection-row:query-backed-result-set:0001",
            DenseCollectionSurface::QueryBackedResultSet,
            "Query-backed result set with a saved query snapshot, all-matching scope, and export/share",
            Some(FilterAstClass::SavedQuerySnapshot),
            Some(SelectionScopeClass::AllMatchingQuery),
            Some(ResultCounterClass::ApproximateCount),
            Some(BatchActionScopeClass::MixedClientProviderBatch),
            CollectionMatrixQualificationClass::Beta,
            vec![
                descriptor("query.export", BatchActionKind::Export, false, false, true),
                descriptor("query.share", BatchActionKind::Share, false, false, true),
            ],
        ),
        downgraded_support_export_row(),
    ]
}

fn guardrails() -> MatrixGuardrails {
    MatrixGuardrails {
        selection_durable_by_stable_identity: true,
        provider_policy_narrowing_always_visible: true,
        visible_loaded_matching_counts_never_blur: true,
        visible_never_all_matching_without_explicit_step: true,
        broad_batch_actions_never_bypass_preview: true,
        rows_auto_downgrade_on_unidentified_semantics: true,
    }
}

fn consumer_projection() -> MatrixConsumerProjection {
    MatrixConsumerProjection {
        product_ingests_matrix: true,
        docs_help_ingests_matrix: true,
        diagnostics_ingests_matrix: true,
        accessibility_ingests_matrix: true,
        release_control_ingests_matrix: true,
        downgraded_rows_labeled_below_current: true,
    }
}

fn evidence_freshness() -> MatrixEvidenceFreshness {
    MatrixEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_downgrade_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_REF,
        M5_COLLECTION_QUALIFICATION_MATRIX_DOC_REF,
        M5_COLLECTION_QUALIFICATION_MATRIX_ARTIFACT_REF,
        "schemas/collections/filter_ast.schema.json",
        "schemas/collections/saved_view.schema.json",
        "schemas/collections/batch_review_packet.schema.json",
        "schemas/collections/selection-scope.schema.json",
    ])
}

fn packet() -> CollectionQualificationMatrixPacket {
    CollectionQualificationMatrixPacket::new(CollectionQualificationMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Dense Collection Qualification Matrix".to_owned(),
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
