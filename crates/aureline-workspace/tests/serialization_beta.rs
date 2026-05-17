//! Protected fixtures for workspace serialization beta packages.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    PortableStateAlphaPackage, ReviewSheetPurpose, WorkspacePortableStatePackage,
    WorkspaceRestoreFidelity, WorkspaceRestoreProvenanceCard, WorkspaceSchemaOutcome,
    WorkspaceSerializationBetaError, WorkspaceStateLayer,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    let path = repo_root().join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn alpha_fixture() -> PortableStateAlphaPackage {
    let payload =
        read_text("fixtures/workspace/portable_state_alpha/core_round_trip_changed_display.json");
    serde_json::from_str(&payload).expect("alpha fixture must parse")
}

fn beta_package() -> WorkspacePortableStatePackage {
    let alpha = alpha_fixture();
    let card = WorkspaceRestoreProvenanceCard::for_alpha_package(
        &alpha,
        "restore-card:workspace:beta:layout-only",
        aureline_workspace::RestoreSourceEvent::Import,
        WorkspaceRestoreFidelity::LayoutOnly,
        "diagnostics:workspace:beta:layout-only",
        "support-export:workspace:beta:layout-only",
        "crash-recovery:workspace:beta:layout-only",
    );
    WorkspacePortableStatePackage::from_alpha_package(&alpha, card)
        .expect("beta package should validate")
}

#[test]
fn beta_package_preserves_separated_layers_placeholders_and_exclusions() {
    let package = beta_package();
    package.validate().expect("package validates");

    let layers = package
        .state_layers
        .iter()
        .map(|row| row.layer)
        .collect::<BTreeSet<_>>();
    for required in [
        WorkspaceStateLayer::WorkspaceAuthority,
        WorkspaceStateLayer::WindowTopology,
        WorkspaceStateLayer::ProfileDefaults,
        WorkspaceStateLayer::MachineLocalHints,
    ] {
        assert!(layers.contains(&required), "missing {required:?}");
    }

    let topology = package
        .state_layers
        .iter()
        .find(|row| row.layer == WorkspaceStateLayer::WindowTopology)
        .expect("topology layer");
    assert!(topology
        .pane_states
        .iter()
        .any(|pane| pane.placeholder.is_some()));
    assert!(package.redaction_manifest.path_redaction_available);
    assert!(package.redaction_manifest.host_redaction_available);
    assert!(package
        .restore_provenance_card
        .support_export_ref
        .starts_with("support-export:"));
    assert!(package
        .exclusions
        .iter()
        .all(|row| { row.named_in_export_summary && row.named_in_restore_summary }));
}

#[test]
fn remembered_state_inspection_and_review_sheets_are_reviewable() {
    let package = beta_package();
    let inspection = package.inspection().expect("inspection");
    let rendered = inspection.render_plaintext();

    assert!(rendered.contains("workspace_authority"));
    assert!(rendered.contains("window_topology"));
    assert!(rendered.contains("machine_local"));
    assert!(rendered.contains("Layout only"));

    let export_sheet = package
        .export_review_sheet("review-sheet:export:beta")
        .expect("export review");
    assert_eq!(export_sheet.purpose, ReviewSheetPurpose::Export);
    assert!(export_sheet.path_redaction_enabled);
    assert!(!export_sheet.exclusions.is_empty());

    let import_sheet = package
        .import_review_sheet("review-sheet:import:beta")
        .expect("import review");
    assert_eq!(import_sheet.purpose, ReviewSheetPurpose::Import);
    assert_eq!(
        export_sheet.selected_layer_ids,
        import_sheet.selected_layer_ids
    );
}

#[test]
fn restore_provenance_fixture_validates() {
    let payload = read_text(
        "fixtures/workspace/m3/portable_state_and_restore/restore_provenance_card_layout_only.json",
    );
    let card: WorkspaceRestoreProvenanceCard =
        serde_json::from_str(&payload).expect("fixture parses");
    card.validate().expect("fixture validates");
    assert_eq!(
        card.resulting_fidelity,
        WorkspaceRestoreFidelity::LayoutOnly
    );
    assert_eq!(card.schema_outcome, WorkspaceSchemaOutcome::LayoutOnly);
}

#[test]
fn validator_rejects_exclusion_that_is_not_named_in_restore_summary() {
    let mut package = beta_package();
    package.exclusions[0].named_in_restore_summary = false;

    assert!(matches!(
        package.validate(),
        Err(WorkspaceSerializationBetaError::ExclusionNotNamed { .. })
    ));
}

#[test]
fn manual_review_schema_outcome_requires_compare_and_export_refs() {
    let mut package = beta_package();
    package.restore_provenance_card.schema_outcome = WorkspaceSchemaOutcome::ManualReview;
    package.restore_provenance_card.compare_ref = None;

    assert!(matches!(
        package.validate(),
        Err(WorkspaceSerializationBetaError::MissingField {
            field: "restore_card.compare_ref"
        })
    ));
}
