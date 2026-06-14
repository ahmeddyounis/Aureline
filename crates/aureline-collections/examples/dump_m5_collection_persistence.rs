//! Conformance dump for the M5 privacy-scoped collection persistence packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence::*;
use aureline_collections::stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm;
use aureline_collections::DenseCollectionSurface;
use aureline_search::{
    CollectionFilterAst, CollectionFilterClause, CollectionFilterLiteral, CollectionFilterOperator,
    CollectionFilterSourceClass, SavedCollectionView, SavedViewFallbackBehavior, SavedViewOwnerScope,
    SavedViewPrivacyClass,
};

const PACKET_ID: &str = "m5-collection-persistence:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";
const CREATED_AT: &str = "2026-06-10T00:00:00Z";

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

fn clause(clause_id: &str, field_id: &str, label: &str, value: &str) -> CollectionFilterClause {
    CollectionFilterClause::new(
        clause_id,
        field_id,
        label,
        CollectionFilterOperator::Equals,
        Some(CollectionFilterLiteral::redacted(value)),
        CollectionFilterSourceClass::User,
    )
}

fn filter_ast(filter_ast_id: &str, scope_label: &str) -> CollectionFilterAst {
    CollectionFilterAst::from_clauses(
        filter_ast_id,
        scope_label,
        vec![
            clause(
                &format!("{filter_ast_id}:state"),
                "state",
                "State",
                "active",
            ),
            clause(
                &format!("{filter_ast_id}:owner"),
                "owner",
                "Owner",
                "current-user",
            ),
        ],
        "aureline.collections.privacy_scoped_persistence",
        CREATED_AT,
    )
}

#[allow(clippy::too_many_arguments)]
fn saved_view(
    saved_view_id: &str,
    name: &str,
    owner_scope: SavedViewOwnerScope,
    privacy_class: SavedViewPrivacyClass,
    fallback_behavior: SavedViewFallbackBehavior,
    scope_label: &str,
    columns: &[&str],
) -> SavedCollectionView {
    SavedCollectionView::new(
        saved_view_id,
        name,
        owner_scope,
        privacy_class,
        fallback_behavior,
        filter_ast(&format!("filter:{saved_view_id}"), scope_label),
        columns.iter().map(|column| (*column).to_owned()).collect(),
        columns
            .iter()
            .take(1)
            .map(|column| (*column).to_owned())
            .collect(),
        CREATED_AT,
    )
}

fn column_preset(column_preset_id: &str, columns: &[&str]) -> PersistedColumnPreset {
    PersistedColumnPreset {
        column_preset_id: column_preset_id.to_owned(),
        visible_column_ids: refs(columns),
        pinned_column_ids: refs(&columns[..1]),
        required_identity_column_ids: refs(&columns[..1]),
        density_mode_token: "compact".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    label: &str,
    saved_view: SavedCollectionView,
    columns: &[&str],
    persisted_schema_version: u32,
    compatibility: PersistenceCompatibility,
    incompatibility_resolution: Option<IncompatibilityResolution>,
    incompatibility_label: Option<&str>,
) -> PersistedCollectionState {
    PersistedCollectionState {
        binding_id: binding_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        saved_view,
        column_preset: column_preset(&format!("columns:{binding_id}"), columns),
        scope_vocabulary_terms: REQUIRED_SCOPE_VOCABULARY_TERMS.to_vec(),
        persisted_schema_version,
        compatibility,
        incompatibility_resolution,
        incompatibility_label: incompatibility_label.map(str::to_owned),
        reopen_rebind_supported: true,
        excludes_transient_selection: true,
        excludes_provider_cursor: true,
        excludes_secret_material: true,
        evidence_refs: refs(&[&format!("evidence:binding:{binding_id}")]),
    }
}

fn bindings() -> Vec<PersistedCollectionState> {
    let current = M5_PERSISTED_STATE_SCHEMA_VERSION;
    let legacy = current - 1;
    vec![
        binding(
            "persist:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            "Pipeline run list filter, saved view, and column preset persisted in workspace scope",
            saved_view(
                "view:pipeline-run-list",
                "Active pipeline runs",
                SavedViewOwnerScope::Workspace,
                SavedViewPrivacyClass::WorkspacePortable,
                SavedViewFallbackBehavior::PreserveAndLabelDegraded,
                "Pipeline runs",
                &["run", "state", "owner", "updated"],
            ),
            &["run", "state", "owner", "updated"],
            current,
            PersistenceCompatibility::Current,
            None,
            None,
        ),
        binding(
            "persist:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            "Review queue saved view shared across the team after redaction",
            saved_view(
                "view:review-queue",
                "My review queue",
                SavedViewOwnerScope::Shared,
                SavedViewPrivacyClass::SharedRedacted,
                SavedViewFallbackBehavior::PreserveAndLabelDegraded,
                "Review queue",
                &["item", "reviewer", "state", "age"],
            ),
            &["item", "reviewer", "state", "age"],
            current,
            PersistenceCompatibility::Current,
            None,
            None,
        ),
        binding(
            "persist:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            "Incident list state persisted under an older schema, migrated forward on reopen",
            saved_view(
                "view:incident-list",
                "Open incidents",
                SavedViewOwnerScope::Workspace,
                SavedViewPrivacyClass::WorkspacePortable,
                SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
                "Incidents",
                &["incident", "severity", "owner", "opened"],
            ),
            &["incident", "severity", "owner", "opened"],
            legacy,
            PersistenceCompatibility::MigratableForward,
            Some(IncompatibilityResolution::MigrateForward),
            Some(
                "Saved under an earlier collection schema; migrated forward to the current schema on reopen with all filter, view, and column choices preserved",
            ),
        ),
        binding(
            "persist:graph-list:0001",
            DenseCollectionSurface::GraphList,
            "Graph/reference list saved view kept private to the local profile",
            saved_view(
                "view:graph-list",
                "Reference graph",
                SavedViewOwnerScope::User,
                SavedViewPrivacyClass::LocalOnlyPrivate,
                SavedViewFallbackBehavior::PreserveAndLabelDegraded,
                "References",
                &["symbol", "kind", "path", "refs"],
            ),
            &["symbol", "kind", "path", "refs"],
            current,
            PersistenceCompatibility::Current,
            None,
            None,
        ),
        binding(
            "persist:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            "Marketplace results saved view owned by the provider catalog",
            saved_view(
                "view:marketplace-results",
                "Recommended extensions",
                SavedViewOwnerScope::ProviderOwned,
                SavedViewPrivacyClass::ProviderOwned,
                SavedViewFallbackBehavior::ProviderRebindRequired,
                "Marketplace",
                &["extension", "publisher", "rating", "version"],
            ),
            &["extension", "publisher", "rating", "version"],
            current,
            PersistenceCompatibility::Current,
            None,
            None,
        ),
        binding(
            "persist:provider-admin-table:0001",
            DenseCollectionSurface::ProviderAdminTable,
            "Provider/admin table state persisted under an incompatible schema; reset to default on reopen",
            saved_view(
                "view:provider-admin-table",
                "Provider accounts",
                SavedViewOwnerScope::PolicyPinned,
                SavedViewPrivacyClass::PolicyGoverned,
                SavedViewFallbackBehavior::RefuseUntilRebound,
                "Admin accounts",
                &["account", "role", "provider", "status"],
            ),
            &["account", "role", "provider", "status"],
            legacy,
            PersistenceCompatibility::IncompatibleNeedsReset,
            Some(IncompatibilityResolution::ResetToDefault),
            Some(
                "Saved under an incompatible admin-table schema that cannot be migrated; reset to the default view and disclosed the dropped filter and column choices",
            ),
        ),
    ]
}

fn guardrails() -> PersistenceGuardrails {
    PersistenceGuardrails {
        reopen_through_shared_objects: true,
        saved_views_preserve_owner_privacy_scope: true,
        transient_selection_and_cursors_never_persisted: true,
        incompatible_state_fails_visibly: true,
        support_can_reconstruct_active_state: true,
    }
}

fn consumer_projection() -> PersistenceConsumerProjection {
    PersistenceConsumerProjection {
        product_reopens_from_records: true,
        diagnostics_reconstructs_from_records: true,
        support_export_reuses_records: true,
        sync_reuses_records: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_COLLECTION_PERSISTENCE_SCHEMA_REF,
        M5_COLLECTION_PERSISTENCE_DOC_REF,
        M5_COLLECTION_PERSISTENCE_ARTIFACT_REF,
        "schemas/collections/filter_ast.schema.json",
        "schemas/collections/saved_view.schema.json",
        "schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json",
    ])
}

fn packet() -> M5CollectionPersistencePacket {
    M5CollectionPersistencePacket::new(M5CollectionPersistencePacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Collection Privacy-Scoped Persistence".to_owned(),
        bindings: bindings(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
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
