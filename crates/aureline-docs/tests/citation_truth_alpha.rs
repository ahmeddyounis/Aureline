use std::path::Path;

use aureline_docs::{
    CitationAnchorAvailability, CitationConfidenceClass, CitationEvidenceExport,
    CitationInferenceMarker, CitationSourceClass, LocaleOverlayState, SourcePrecedenceClass,
};

fn repo_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

fn sample_export() -> CitationEvidenceExport {
    let payload = std::fs::read_to_string(repo_path(
        "artifacts/docs/citation_export_sample_alpha.json",
    ))
    .expect("citation export fixture reads");
    serde_json::from_str(&payload).expect("citation export fixture parses")
}

#[test]
fn sample_export_validates_and_preserves_required_citation_truth() {
    let export = sample_export();
    assert!(export.validate().is_empty(), "{:?}", export.validate());

    let exact_drawer = export
        .citation_drawers
        .iter()
        .find(|drawer| drawer.docs_node.docs_node_id == "docs-node:project-entry.open-folder")
        .expect("exact-anchor drawer exists");
    assert_eq!(
        exact_drawer.rows[0].citation_availability,
        CitationAnchorAvailability::ExactAnchorAvailable
    );
    assert_eq!(
        exact_drawer.rows[0].exact_anchor_ref.as_deref(),
        Some("docs:anchor:workspace:open_folder_overview")
    );
    assert!(!exact_drawer.degrades_certainty());

    let missing_drawer = export
        .citation_drawers
        .iter()
        .find(|drawer| {
            drawer.docs_node.docs_node_id == "docs-node:onboarding.deep-dive.not-installed"
        })
        .expect("missing-anchor drawer exists");
    assert_eq!(
        missing_drawer.rows[0].citation_availability,
        CitationAnchorAvailability::AnchorUnavailableDisclosed
    );
    assert!(missing_drawer.rows[0].hidden_or_omitted_note.is_some());
    assert!(missing_drawer.degrades_certainty());

    assert!(export.docs_nodes.iter().any(|node| node.docs_node_id
        == "docs-node:vendor.reference.open-folder"
        && node.source_class == CitationSourceClass::VendorProviderDocs));
    assert_eq!(
        exact_drawer.project_vendor_precedence,
        SourcePrecedenceClass::ProjectVendorDisagreementInspectable
    );

    let derived_row = export
        .citation_drawers
        .iter()
        .flat_map(|drawer| &drawer.rows)
        .find(|row| row.citation_ref == "citation:derived:open-folder-impact:basis")
        .expect("derived inference row exists");
    assert_eq!(
        derived_row.source_class,
        CitationSourceClass::DerivedExplanation
    );
    assert_eq!(
        derived_row.inference_marker,
        CitationInferenceMarker::Inference
    );
    assert_eq!(
        derived_row.confidence_class,
        CitationConfidenceClass::Inferred
    );

    let onboarding_item = export
        .help_pack_items
        .iter()
        .find(|item| item.item_id == "docs-node:onboarding.keymap-bridge")
        .expect("onboarding item exists");
    assert_eq!(
        onboarding_item.locale_overlay_state,
        LocaleOverlayState::LocaleMissingFallbackToSourceLanguage
    );
    assert_eq!(
        onboarding_item.source_language_fallback_ref.as_deref(),
        Some("docs-node:onboarding.keymap-bridge#en-US")
    );
    assert!(!onboarding_item.citation_anchor_refs.is_empty());
}

#[test]
fn fixture_manifest_points_at_exportable_support_reconstruction_inputs() {
    let manifest = std::fs::read_to_string(repo_path(
        "fixtures/docs/citation_truth_alpha/manifest.yaml",
    ))
    .expect("citation truth manifest reads");
    assert!(manifest.contains("exact-anchor-opens"));
    assert!(manifest.contains("missing-anchor-disclosed"));
    assert!(manifest.contains("project-vendor-precedence-visible"));
    assert!(manifest.contains("derived-claim-marked-as-inference"));
    assert!(manifest.contains("onboarding-locale-fallback-exportable"));

    let export_json = sample_export().export_safe_json();
    assert!(!export_json.contains("://"));
    assert!(export_json.contains("\"pack_id\": \"pack:project:aureline:alpha\""));
    assert!(export_json.contains("\"item_id\": \"docs-node:onboarding.keymap-bridge\""));
    assert!(export_json.contains("\"citation_anchor_refs\""));
    assert!(export_json.contains("\"source_language_fallback_ref\""));
}
