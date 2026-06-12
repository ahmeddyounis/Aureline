use std::path::{Path, PathBuf};

use aureline_docs::{
    DocsDerivedExplanationKind, DocsKnowledgeSurfaceEvidencePacket, DocsKnowledgeSurfaceKind,
    DocsMirrorOfflinePosture, DocsPack, DocsSearchIndex, DocsTruthLabelClass,
};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

fn sample_packet() -> DocsKnowledgeSurfaceEvidencePacket {
    let payload = std::fs::read_to_string(repo_path(
        "artifacts/docs/docs_evidence_packets/provenance_and_citation_truth_packet.json",
    ))
    .expect("docs evidence packet fixture reads");
    serde_json::from_str(&payload).expect("docs evidence packet fixture parses")
}

#[test]
fn evidence_packet_preserves_current_offline_and_stale_generated_truth() {
    let packet = sample_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let glossary = packet
        .surface_projections
        .iter()
        .find(|projection| projection.surface_kind == DocsKnowledgeSurfaceKind::GlossaryCard)
        .expect("glossary projection exists");
    assert_eq!(
        glossary.provenance.mirror_offline_posture,
        DocsMirrorOfflinePosture::OfflinePinnedPack
    );
    assert_eq!(
        glossary.provenance.truth_label.label_class,
        DocsTruthLabelClass::CurrentDocs
    );
    assert_eq!(
        glossary.source_strip.citation_anchor_refs,
        vec!["docs-anchor:glossary:workspace-trust".to_owned()]
    );
    assert!(glossary.keyboard_accessible_actions);

    let generated = packet
        .surface_projections
        .iter()
        .find(|projection| projection.surface_kind == DocsKnowledgeSurfaceKind::CodebaseExplainer)
        .expect("generated explainer projection exists");
    assert_eq!(
        generated.provenance.derived_explanation_kind,
        Some(DocsDerivedExplanationKind::Generated)
    );
    assert_eq!(
        generated.provenance.truth_label.label_class,
        DocsTruthLabelClass::RetestPending
    );
    assert!(generated
        .provenance
        .truth_label
        .reason_tokens
        .contains(&"stale".to_owned()));
    assert_eq!(generated.source_strip.freshness_class_token, "stale");
    assert_eq!(
        generated
            .provenance
            .infrastructure_lineage
            .as_ref()
            .expect("infra lineage present")
            .truth_layer_tokens(),
        vec![
            "authored_desired".to_owned(),
            "rendered_expanded".to_owned()
        ]
    );
    assert_eq!(
        generated
            .source_strip
            .infrastructure_unavailable_truth_layer_tokens,
        vec!["observed_live".to_owned(), "provider_overlay".to_owned()]
    );

    let explanation = packet
        .derived_explanations
        .first()
        .expect("derived explanation exists");
    assert_eq!(
        explanation.derived_explanation_kind,
        DocsDerivedExplanationKind::Generated
    );
    assert!(explanation
        .claims
        .iter()
        .all(|claim| claim.has_supporting_citation()));
    assert!(explanation
        .upstream_citation_anchor_refs
        .contains(&"docs-anchor:glossary:workspace-trust".to_owned()));
}

#[test]
fn docs_search_entries_project_the_shared_source_strip() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("fixture docs pack loads");
    let index = DocsSearchIndex::from_pack("ws-docs-beta", "docs-index:beta:01", pack);

    let result = index.query("setup");
    let hit = result
        .entries
        .iter()
        .find(|entry| entry.title == "TypeScript web launch setup")
        .expect("setup docs hit appears");

    assert!(
        hit.knowledge_surface_projection.validate().is_empty(),
        "{:?}",
        hit.knowledge_surface_projection.validate()
    );
    assert_eq!(
        hit.knowledge_surface_projection.surface_kind,
        DocsKnowledgeSurfaceKind::DocsBackedSearch
    );
    assert_eq!(
        hit.knowledge_surface_projection
            .source_strip
            .source_class_token,
        "curated_knowledge_pack"
    );
    assert_eq!(
        hit.knowledge_surface_projection
            .source_strip
            .truth_label_token,
        "current_docs"
    );
    assert!(hit.knowledge_surface_projection.keyboard_accessible_actions);
}

#[test]
fn fixture_manifest_points_at_the_export_and_schemas() {
    let manifest = std::fs::read_to_string(repo_path(
        "fixtures/docs/provenance_and_citation_truth/manifest.yaml",
    ))
    .expect("fixture manifest reads");

    assert!(manifest.contains("docs_node_provenance.schema.json"));
    assert!(manifest.contains("docs_derived_explanation.schema.json"));
    assert!(manifest.contains("current-offline-glossary-preserves-citations"));
    assert!(manifest.contains("stale-generated-explainer-downgrades"));

    let export_json = sample_packet().export_safe_json();
    assert!(!export_json.contains("://"));
    assert!(export_json.contains("\"truth_label\": \"Retest pending\""));
    assert!(export_json.contains("\"mirror_offline_posture_token\": \"offline_pinned_pack\""));
    assert!(export_json.contains(
        "\"infrastructure_lineage_ref\": \"infra-lineage:docs-evidence:checkout-drift\""
    ));
}
