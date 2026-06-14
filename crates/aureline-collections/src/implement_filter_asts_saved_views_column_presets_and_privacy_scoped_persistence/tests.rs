use super::*;

use aureline_search::{
    CollectionFilterAst, CollectionFilterClause, CollectionFilterLiteral, CollectionFilterOperator,
    CollectionFilterSourceClass, SavedCollectionView, SavedViewFallbackBehavior,
    SavedViewOwnerScope, SavedViewPrivacyClass,
};

const PACKET_ID: &str = "m5-collection-persistence:test:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";
const CREATED_AT: &str = "2026-06-10T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn all_scope_terms() -> Vec<ScopeCounterVocabularyTerm> {
    REQUIRED_SCOPE_VOCABULARY_TERMS.to_vec()
}

fn filter_ast(
    filter_ast_id: &str,
    source_class: CollectionFilterSourceClass,
) -> CollectionFilterAst {
    CollectionFilterAst::from_clauses(
        filter_ast_id,
        "Scope",
        vec![CollectionFilterClause::new(
            format!("{filter_ast_id}:state"),
            "state",
            "State",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted("active")),
            source_class,
        )],
        "test",
        CREATED_AT,
    )
}

fn saved_view(
    saved_view_id: &str,
    owner_scope: SavedViewOwnerScope,
    privacy_class: SavedViewPrivacyClass,
) -> SavedCollectionView {
    SavedCollectionView::new(
        saved_view_id,
        "View",
        owner_scope,
        privacy_class,
        SavedViewFallbackBehavior::PreserveAndLabelDegraded,
        filter_ast(
            &format!("filter:{saved_view_id}"),
            CollectionFilterSourceClass::User,
        ),
        refs(&["identity", "state", "owner"]),
        refs(&["identity"]),
        CREATED_AT,
    )
}

fn column_preset(column_preset_id: &str) -> PersistedColumnPreset {
    PersistedColumnPreset {
        column_preset_id: column_preset_id.to_owned(),
        visible_column_ids: refs(&["identity", "state", "owner"]),
        pinned_column_ids: refs(&["identity"]),
        required_identity_column_ids: refs(&["identity"]),
        density_mode_token: "compact".to_owned(),
    }
}

fn current_binding(binding_id: &str, surface: DenseCollectionSurface) -> PersistedCollectionState {
    PersistedCollectionState {
        binding_id: binding_id.to_owned(),
        surface,
        label_summary: "Persisted state".to_owned(),
        saved_view: saved_view(
            &format!("view:{binding_id}"),
            SavedViewOwnerScope::Workspace,
            SavedViewPrivacyClass::WorkspacePortable,
        ),
        column_preset: column_preset(&format!("columns:{binding_id}")),
        scope_vocabulary_terms: all_scope_terms(),
        persisted_schema_version: M5_PERSISTED_STATE_SCHEMA_VERSION,
        compatibility: PersistenceCompatibility::Current,
        incompatibility_resolution: None,
        incompatibility_label: None,
        reopen_rebind_supported: true,
        excludes_transient_selection: true,
        excludes_provider_cursor: true,
        excludes_secret_material: true,
        evidence_refs: refs(&[&format!("evidence:{binding_id}")]),
    }
}

fn incompatible_binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
) -> PersistedCollectionState {
    let mut binding = current_binding(binding_id, surface);
    binding.persisted_schema_version = M5_PERSISTED_STATE_SCHEMA_VERSION - 1;
    binding.compatibility = PersistenceCompatibility::MigratableForward;
    binding.incompatibility_resolution = Some(IncompatibilityResolution::MigrateForward);
    binding.incompatibility_label = Some(
        "Saved under an earlier schema; migrated forward on reopen with all choices preserved"
            .to_owned(),
    );
    binding
}

fn baseline_bindings() -> Vec<PersistedCollectionState> {
    vec![
        current_binding("persist:pipeline", DenseCollectionSurface::PipelineRunList),
        current_binding("persist:review", DenseCollectionSurface::ReviewQueue),
        incompatible_binding("persist:incident", DenseCollectionSurface::IncidentList),
        current_binding("persist:graph", DenseCollectionSurface::GraphList),
        current_binding(
            "persist:marketplace",
            DenseCollectionSurface::MarketplaceResults,
        ),
        current_binding("persist:admin", DenseCollectionSurface::ProviderAdminTable),
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
    ])
}

fn baseline_packet() -> M5CollectionPersistencePacket {
    M5CollectionPersistencePacket::new(M5CollectionPersistencePacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test persistence packet".to_owned(),
        bindings: baseline_bindings(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn baseline_packet_validates() {
    assert!(baseline_packet().validate().is_empty());
}

#[test]
fn reopen_outcomes_track_compatibility_and_resolution() {
    assert_eq!(
        current_binding("b", DenseCollectionSurface::PipelineRunList).reopen(),
        ReopenOutcome::RestoredExact
    );

    let mut migrate = incompatible_binding("b", DenseCollectionSurface::IncidentList);
    assert_eq!(migrate.reopen(), ReopenOutcome::RestoredAfterMigration);
    assert!(migrate.reopen().restored_state());

    migrate.incompatibility_resolution = Some(IncompatibilityResolution::ResetToDefault);
    assert_eq!(migrate.reopen(), ReopenOutcome::ResetToDefault);

    let mut reset = incompatible_binding("b", DenseCollectionSurface::ProviderAdminTable);
    reset.compatibility = PersistenceCompatibility::IncompatibleNeedsReset;
    reset.incompatibility_resolution = Some(IncompatibilityResolution::ResetToDefault);
    assert_eq!(reset.reopen(), ReopenOutcome::ResetToDefault);

    reset.incompatibility_resolution = Some(IncompatibilityResolution::RefuseUntilRebound);
    assert_eq!(reset.reopen(), ReopenOutcome::RefusedNeedsRebind);
    assert!(!reset.reopen().restored_state());
}

#[test]
fn incompatible_state_without_label_is_inconsistent() {
    let mut binding = incompatible_binding("b", DenseCollectionSurface::IncidentList);
    binding.incompatibility_label = None;
    assert!(!binding.compatibility_consistent());
}

#[test]
fn incompatible_state_with_generic_label_is_inconsistent() {
    let mut binding = incompatible_binding("b", DenseCollectionSurface::IncidentList);
    binding.incompatibility_label = Some("incompatible".to_owned());
    assert!(!binding.compatibility_consistent());
}

#[test]
fn incompatible_needs_reset_cannot_claim_forward_migration() {
    let mut binding = incompatible_binding("b", DenseCollectionSurface::ProviderAdminTable);
    binding.compatibility = PersistenceCompatibility::IncompatibleNeedsReset;
    binding.incompatibility_resolution = Some(IncompatibilityResolution::MigrateForward);
    assert!(!binding.compatibility_consistent());
}

#[test]
fn current_state_with_resolution_is_inconsistent() {
    let mut binding = current_binding("b", DenseCollectionSurface::PipelineRunList);
    binding.incompatibility_resolution = Some(IncompatibilityResolution::MigrateForward);
    assert!(!binding.compatibility_consistent());
}

#[test]
fn shared_view_must_not_be_local_only_private() {
    let mut binding = current_binding("b", DenseCollectionSurface::ReviewQueue);
    binding.saved_view = saved_view(
        "view:shared-local",
        SavedViewOwnerScope::Shared,
        SavedViewPrivacyClass::LocalOnlyPrivate,
    );
    assert!(!binding.privacy_scope_consistent());

    let mut packet = baseline_packet();
    packet.bindings[1] = binding;
    let violations = packet.validate();
    assert!(violations.contains(&M5CollectionPersistenceViolation::PrivacyScopeInconsistent));
}

#[test]
fn portable_view_must_carry_portable_filter() {
    let mut binding = current_binding("b", DenseCollectionSurface::PipelineRunList);
    binding.saved_view.filter_ast = CollectionFilterAst::from_clauses(
        "filter:local",
        "Scope",
        vec![CollectionFilterClause::new(
            "filter:local:state",
            "state",
            "State",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::raw_local_only("active")),
            CollectionFilterSourceClass::User,
        )],
        "test",
        CREATED_AT,
    );
    assert!(!binding.privacy_scope_consistent());
}

#[test]
fn persisted_selection_is_rejected() {
    let mut binding = current_binding("b", DenseCollectionSurface::PipelineRunList);
    binding.excludes_transient_selection = false;
    assert!(!binding.excludes_transient_state());

    let mut packet = baseline_packet();
    packet.bindings[0] = binding;
    assert!(packet
        .validate()
        .contains(&M5CollectionPersistenceViolation::PersistsTransientState));
}

#[test]
fn saved_view_capturing_cursor_is_rejected() {
    let mut binding = current_binding("b", DenseCollectionSurface::MarketplaceResults);
    binding.saved_view.captures_provider_cursor = true;
    assert!(!binding.excludes_transient_state());
}

#[test]
fn column_preset_dropping_identity_column_is_rejected() {
    let mut binding = current_binding("b", DenseCollectionSurface::PipelineRunList);
    binding.column_preset.required_identity_column_ids = refs(&["identity", "missing"]);
    assert!(!binding.column_preset.is_valid());

    let mut packet = baseline_packet();
    packet.bindings[0] = binding;
    assert!(packet
        .validate()
        .contains(&M5CollectionPersistenceViolation::ColumnPresetDropsIdentityColumn));
}

#[test]
fn incomplete_scope_vocabulary_is_rejected() {
    let mut binding = current_binding("b", DenseCollectionSurface::PipelineRunList);
    binding.scope_vocabulary_terms = vec![ScopeCounterVocabularyTerm::Visible];
    assert!(!binding.scope_vocabulary_ok());
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .bindings
        .retain(|binding| binding.surface != DenseCollectionSurface::ProviderAdminTable);
    assert!(packet
        .validate()
        .contains(&M5CollectionPersistenceViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_incompatible_case_is_rejected() {
    let mut packet = baseline_packet();
    for binding in &mut packet.bindings {
        *binding = current_binding(&binding.binding_id, binding.surface);
    }
    assert!(packet
        .validate()
        .contains(&M5CollectionPersistenceViolation::IncompatibleCaseMissing));
}

#[test]
fn support_reconstruction_recovers_active_state() {
    let packet = baseline_packet();
    let reconstructions = packet.support_reconstructions();
    assert_eq!(reconstructions.len(), packet.bindings.len());

    let pipeline = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.surface_token == "pipeline_run_list")
        .expect("pipeline reconstruction present");
    assert_eq!(pipeline.reopen_outcome_token, "restored_exact");
    assert!(pipeline.restored_exact_state);
    assert_eq!(pipeline.filter_clause_count, 1);
    assert!(pipeline.visible_column_ids.contains(&"identity".to_owned()));

    let incident = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.surface_token == "incident_list")
        .expect("incident reconstruction present");
    assert_eq!(incident.compatibility_token, "migratable_forward");
    assert_eq!(
        incident.resolution_token.as_deref(),
        Some("migrate_forward")
    );
    assert_eq!(incident.reopen_outcome_token, "restored_after_migration");
}

#[test]
fn export_is_metadata_safe() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(packet.validate().is_empty());
}

#[test]
fn record_kind_and_schema_version_are_pinned() {
    let packet = baseline_packet();
    assert_eq!(packet.record_kind, M5_COLLECTION_PERSISTENCE_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION
    );
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_collection_persistence_export()
        .expect("checked-in persistence export parses and validates");
    assert_eq!(packet.packet_id, "m5-collection-persistence:stable:0001");
    assert!(packet.validate().is_empty());
    for required in REQUIRED_PERSISTED_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    assert!(packet.incompatible_binding_count() >= 1);
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: M5CollectionPersistencePacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}
