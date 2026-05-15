use std::path::{Path, PathBuf};

use aureline_docs::{
    CitationAnchorAvailability, CitationLocalityClass, CitationSourceClass, DocsFreshnessClass,
    DocsPack, DocsScopeClass, DocsSearchIndex, VersionMatchState,
};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

#[test]
fn docs_pack_index_returns_anchor_backed_hits() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("fixture docs pack loads");
    let index = DocsSearchIndex::from_pack("ws-docs-alpha", "docs-index:alpha:01", pack);

    let result = index.query("setup");
    let hit = result
        .entries
        .iter()
        .find(|entry| entry.title == "TypeScript web launch setup")
        .expect("setup docs hit appears");

    assert_eq!(hit.result_kind_token, "docs_anchor");
    assert_eq!(hit.canonical_ref, "docs-anchor:bundle:typescript-web:start");
    assert_eq!(
        hit.primary_anchor_ref.as_deref(),
        Some("docs-anchor:bundle:typescript-web:start")
    );
    assert_eq!(hit.freshness_class_token, "warm_cached");
    assert_eq!(hit.freshness_badge, "Cached");
    assert_eq!(hit.version_match_state_token, "exact_build_match");
    assert_eq!(hit.version_match_badge, "Exact build");
    assert!(hit.opens_exact_anchor());
    assert!(!hit.degrades_result());
}

#[test]
fn anchor_unavailability_degrades_docs_index_entry() {
    let raw = r#"
schema_version: 1
pack_id: docs_pack.alpha.anchor.missing
pack_revision_ref: rev:2026-05-12
pack_label: Missing anchor docs pack
source_locale: en-US
source_truth:
  source_class: project_docs
  scope_class: docs_help
  version_or_revision_ref: workspace-main
  version_match_state: compatible_minor_drift
  freshness_class: degraded_cached
  locality_class: local_project_pack
  citation_availability: anchor_unavailable_disclosed
  running_build_identity_ref: id:build:aureline:running
  hidden_or_omitted_note: Exact anchor is unavailable in this pack snapshot.
nodes:
  - docs_node_id: docs-node:anchor-missing
    doc_kind: product_help
    title: Anchor missing
    summary: The result remains searchable with an explicit missing-anchor disclosure.
    body_markdown: Missing anchor body.
"#;

    let pack = DocsPack::from_yaml_str(raw).expect("missing-anchor pack loads");
    assert_eq!(
        pack.source_truth.source_class,
        CitationSourceClass::ProjectDocs
    );
    assert_eq!(pack.source_truth.scope_class, DocsScopeClass::DocsHelp);
    assert_eq!(
        pack.source_truth.citation_availability,
        CitationAnchorAvailability::AnchorUnavailableDisclosed
    );
    assert_eq!(
        pack.source_truth.version_match_state,
        VersionMatchState::CompatibleMinorDrift
    );
    assert_eq!(
        pack.source_truth.freshness_class,
        DocsFreshnessClass::DegradedCached
    );
    assert_eq!(
        pack.source_truth.locality_class,
        CitationLocalityClass::LocalProjectPack
    );

    let index = DocsSearchIndex::from_pack("ws-docs-alpha", "docs-index:alpha:missing", pack);
    let result = index.query("anchor");
    let hit = result.entries.first().expect("missing-anchor hit appears");

    assert_eq!(
        hit.citation_anchor_availability_token,
        "anchor_unavailable_disclosed"
    );
    assert_eq!(hit.canonical_ref, "docs-node:anchor-missing");
    assert!(hit.primary_anchor_ref.is_none());
    assert!(!hit.opens_exact_anchor());
    assert!(hit.degrades_result());
    assert_eq!(
        hit.partial_truth_causes(),
        vec![
            "anchor_unavailable_disclosed".to_string(),
            "degraded_cached".to_string(),
            "compatible_minor_drift".to_string()
        ]
    );
}
