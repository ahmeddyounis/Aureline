use std::path::{Path, PathBuf};

use aureline_docs::{
    CitationAnchorAvailability, CitationLocalityClass, CitationSourceClass, DocsFreshnessClass,
    DocsNodeKind, DocsPack, DocsPackLoadError, DocsScopeClass, VersionMatchState,
};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

#[test]
fn yaml_pack_resolves_launch_bundle_docs_nodes() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("TS/JS launch docs pack fixture loads");

    assert_eq!(pack.pack_id, "docs_pack.alpha.ts_web.start");
    assert_eq!(pack.pack_revision_ref, "rev:2026-05-12");
    assert_eq!(
        pack.source_truth.source_class,
        CitationSourceClass::CuratedKnowledgePack
    );
    assert_eq!(pack.source_truth.scope_class, DocsScopeClass::DocsHelp);
    assert_eq!(
        pack.source_truth.version_match_state,
        VersionMatchState::ExactBuildMatch
    );
    assert_eq!(
        pack.source_truth.freshness_class,
        DocsFreshnessClass::WarmCached
    );
    assert_eq!(
        pack.source_truth.locality_class,
        CitationLocalityClass::MirroredOffline
    );
    assert_eq!(
        pack.source_truth.citation_availability,
        CitationAnchorAvailability::ExactAnchorAvailable
    );
    assert_eq!(pack.nodes.len(), 2);

    let setup = &pack.nodes[0];
    assert_eq!(
        setup.docs_node.docs_node_id,
        "docs-node:launch-bundle:typescript-web:start"
    );
    assert_eq!(setup.docs_node.doc_kind, DocsNodeKind::OnboardingCard);
    assert!(setup.docs_node.validate().is_empty());
    assert_eq!(
        setup.docs_node.version_match_state,
        VersionMatchState::ExactBuildMatch
    );
    assert_eq!(
        setup.docs_node.freshness_class,
        DocsFreshnessClass::WarmCached
    );
    assert!(setup.body_markdown.contains("TypeScript web launch bundle"));
}

#[test]
fn launch_bundle_declares_the_fixture_docs_pack() {
    let bundle =
        std::fs::read_to_string(repo_path("artifacts/bundles/tsjs_launch_bundle_alpha.yaml"))
            .expect("TS/JS launch bundle fixture reads");
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("TS/JS launch docs pack fixture loads");

    assert!(
        bundle.contains(&pack.pack_id),
        "launch bundle must declare docs pack {}",
        pack.pack_id
    );
    assert!(pack.nodes.iter().any(|node| {
        node.source_ref.as_deref().is_some_and(|source_ref| {
            source_ref.starts_with("artifacts/bundles/tsjs_launch_bundle_alpha.yaml")
        })
    }));
}

#[test]
fn markdown_front_matter_uses_body_as_single_node_content() {
    let markdown = r#"---
schema_version: 1
pack_id: docs_pack.alpha.markdown.example
pack_revision_ref: rev:2026-05-12
pack_label: Markdown docs pack example
source_locale: en-US
source_truth:
  source_class: project_docs
  scope_class: docs_help
  version_or_revision_ref: workspace-main
  version_match_state: compatible_minor_drift
  freshness_class: authoritative_live
  locality_class: local_project_pack
  citation_availability: exact_anchor_available
  running_build_identity_ref: id:build:aureline:running
node:
  docs_node_id: docs-node:markdown:example
  doc_kind: product_help
  title: Markdown example
  citation_anchor_refs:
    - docs-anchor:markdown:example
---
# Markdown example

Markdown body content.
"#;

    let pack = DocsPack::from_markdown_str(markdown).expect("markdown docs pack loads");

    assert_eq!(pack.nodes.len(), 1);
    let node = &pack.nodes[0];
    assert_eq!(node.title, "Markdown example");
    assert!(node.body_markdown.contains("Markdown body content."));
    assert_eq!(
        node.docs_node.version_match_state,
        VersionMatchState::CompatibleMinorDrift
    );
    assert_eq!(
        node.docs_node.freshness_class,
        DocsFreshnessClass::AuthoritativeLive
    );
    assert!(node.docs_node.has_exact_anchor());
}

#[test]
fn loader_rejects_pack_without_source_truth() {
    let raw = r#"
schema_version: 1
pack_id: docs_pack.alpha.invalid
pack_revision_ref: rev:2026-05-12
pack_label: Invalid docs pack
source_locale: en-US
nodes:
  - docs_node_id: docs-node:invalid
    title: Invalid
    citation_anchor_refs:
      - docs-anchor:invalid
"#;

    let err = DocsPack::from_yaml_str(raw).expect_err("source_truth is required");
    assert!(matches!(err, DocsPackLoadError::MissingSourceTruth));
}

#[test]
fn pack_metadata_does_not_surface_planning_ids() {
    let pack = DocsPack::load_path(repo_path(
        "fixtures/docs/packs/tsjs_launch_bundle_docs_pack.yaml",
    ))
    .expect("TS/JS launch docs pack fixture loads");
    let metadata = [
        pack.pack_id.as_str(),
        pack.pack_revision_ref.as_str(),
        pack.pack_label.as_str(),
        pack.source_truth.version_or_revision_ref.as_str(),
    ]
    .join("\n");

    for forbidden in ["G03", "M0", "WP-"] {
        assert!(
            !metadata.contains(forbidden),
            "pack metadata must not expose planning id {forbidden}"
        );
    }
}
