//! Protected-walk integration tests for the file-tree filter, reveal-current
//! behavior, and the path-ancestry breadcrumb trail.
//!
//! Walks the protected dogfood path named in the spec:
//!
//!   filter or reveal a nested file
//!     -> inspect breadcrumbs and ancestry
//!     -> confirm the explorer never loses the canonical path.
//!
//! Plus the failure drill:
//!
//!   filter then reveal a deep path
//!     -> confirm breadcrumbs / ancestry do not jump to a different node
//!        identity.
//!
//! Each fixture under `/fixtures/explorer/breadcrumb_cases/*.json` describes
//! a synthetic workspace shape, an action sequence (`apply_filter` or
//! `reveal`), and the expected reveal outcome plus breadcrumb projection.
//! The tests load the fixture, materialize a tree, dispatch the actions,
//! and assert the observed runtime projection matches.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::breadcrumbs::materialize_breadcrumb_path;
use aureline_shell::explorer::{
    apply_filter, reveal, ExplorerNode, ExplorerNodeId, ExplorerNodeKind, ExplorerTree,
    NodeReadinessClass,
};
use aureline_workspace::{RootPartialTruth, WorkspaceRootKind};

#[derive(Debug, Deserialize)]
struct BreadcrumbFixture {
    #[serde(default)]
    case_id: String,
    #[serde(default)]
    title: String,
    workspace: FixtureWorkspace,
    actions: Vec<FixtureAction>,
    expected_outcomes: ExpectedOutcomes,
}

#[derive(Debug, Deserialize)]
struct FixtureWorkspace {
    workspace_id: String,
    root: FixtureRoot,
    #[serde(default)]
    extra_nodes: Vec<FixtureExtraNode>,
}

#[derive(Debug, Deserialize)]
struct FixtureRoot {
    root_id: String,
    root_kind: String,
    presentation_label: String,
    partial_truth: String,
}

#[derive(Debug, Deserialize)]
struct FixtureExtraNode {
    logical_path: String,
    kind: String,
    display_label: String,
    readiness: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum FixtureAction {
    ApplyFilter { query: Option<String> },
    Reveal { logical_path: String },
}

#[derive(Debug, Deserialize)]
struct ExpectedOutcomes {
    #[serde(default)]
    filter: Option<ExpectedFilter>,
    #[serde(default)]
    reveal: Option<ExpectedReveal>,
}

#[derive(Debug, Deserialize)]
struct ExpectedFilter {
    query: Option<String>,
    query_is_empty: bool,
    total_nodes: u32,
    selection_matches_filter: bool,
}

#[derive(Debug, Deserialize)]
struct ExpectedReveal {
    revealed_logical_path: String,
    ancestry_logical_paths: Vec<String>,
    matches_filter: bool,
    filter_query: Option<String>,
    breadcrumb_path: ExpectedBreadcrumbPath,
}

#[derive(Debug, Deserialize)]
struct ExpectedBreadcrumbPath {
    workspace_id: String,
    root_id: String,
    root_badge: String,
    leaf_logical_path: String,
    segments: Vec<ExpectedSegment>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSegment {
    logical_path: String,
    display_label: String,
    kind: String,
    is_root: bool,
    is_leaf: bool,
}

fn parse_root_kind(value: &str) -> WorkspaceRootKind {
    match value {
        "local_folder" => WorkspaceRootKind::LocalFolder,
        "local_repo_root" => WorkspaceRootKind::LocalRepoRoot,
        "remote_repository" => WorkspaceRootKind::RemoteRepository,
        "ssh_workspace" => WorkspaceRootKind::SshWorkspace,
        "container_root" => WorkspaceRootKind::ContainerRoot,
        "devcontainer_root" => WorkspaceRootKind::DevcontainerRoot,
        "managed_cloud_root" => WorkspaceRootKind::ManagedCloudRoot,
        "virtual_document_root" => WorkspaceRootKind::VirtualDocumentRoot,
        "archive_root" => WorkspaceRootKind::ArchiveRoot,
        other => panic!("unsupported root_kind: {other}"),
    }
}

fn parse_partial_truth(value: &str) -> RootPartialTruth {
    match value {
        "loaded" => RootPartialTruth::Loaded,
        "manifest_known" => RootPartialTruth::ManifestKnown,
        "cached" => RootPartialTruth::Cached,
        "unavailable" => RootPartialTruth::Unavailable,
        other => panic!("unsupported partial_truth: {other}"),
    }
}

fn parse_node_kind(value: &str) -> ExplorerNodeKind {
    match value {
        "root_mount" => ExplorerNodeKind::RootMount,
        "directory" => ExplorerNodeKind::Directory,
        "file" => ExplorerNodeKind::File,
        "generated_artifact" => ExplorerNodeKind::GeneratedArtifact,
        "virtual_document" => ExplorerNodeKind::VirtualDocument,
        "special_file" => ExplorerNodeKind::SpecialFile,
        other => panic!("unsupported node kind: {other}"),
    }
}

fn parse_readiness(value: &str) -> NodeReadinessClass {
    match value {
        "loaded" => NodeReadinessClass::Loaded,
        "partially_enumerated" => NodeReadinessClass::PartiallyEnumerated,
        "manifest_known" => NodeReadinessClass::ManifestKnown,
        "cached" => NodeReadinessClass::Cached,
        "unavailable" => NodeReadinessClass::Unavailable,
        other => panic!("unsupported readiness: {other}"),
    }
}

fn id_for(
    workspace_id: &str,
    root_id: &str,
    root_logical: &str,
    logical_path: &str,
) -> ExplorerNodeId {
    let logical_uri = if logical_path.is_empty() {
        root_logical.to_string()
    } else {
        format!("{root_logical}{logical_path}")
    };
    ExplorerNodeId::from_logical(workspace_id, root_id, &logical_uri)
}

fn build_tree(fixture: &BreadcrumbFixture) -> ExplorerTree {
    let mut tree = ExplorerTree::new();
    let workspace_id = fixture.workspace.workspace_id.clone();
    let root_id = fixture.workspace.root.root_id.clone();
    let root_kind = parse_root_kind(&fixture.workspace.root.root_kind);
    let root_readiness = NodeReadinessClass::from_root_partial_truth(parse_partial_truth(
        &fixture.workspace.root.partial_truth,
    ));
    let root_node = ExplorerNode::root_mount(
        &workspace_id,
        &root_id,
        root_kind,
        &fixture.workspace.root.presentation_label,
        root_readiness,
    );
    let root_logical_uri = root_node.logical_uri.clone();
    let root_node_id = root_node.node_id.clone();
    let root_badge = root_node.root_badge.clone();
    tree.insert(root_node).expect("root must insert");

    let mut sorted: Vec<&FixtureExtraNode> = fixture.workspace.extra_nodes.iter().collect();
    sorted.sort_by_key(|n| n.logical_path.matches('/').count());

    for extra in sorted {
        let kind = parse_node_kind(&extra.kind);
        let readiness = parse_readiness(&extra.readiness);
        let logical_uri = format!("{}{}", root_logical_uri, extra.logical_path);
        let parent_id = match extra.logical_path.rsplit_once('/') {
            Some((parent_path, _)) => {
                let parent_logical_uri = format!("{}{}", root_logical_uri, parent_path);
                ExplorerNodeId::from_logical(&workspace_id, &root_id, &parent_logical_uri)
            }
            None => root_node_id.clone(),
        };
        let depth = (extra.logical_path.matches('/').count() as u32) + 1;
        let node = ExplorerNode {
            node_id: ExplorerNodeId::from_logical(&workspace_id, &root_id, &logical_uri),
            workspace_id: workspace_id.clone(),
            root_id: root_id.clone(),
            root_kind,
            kind,
            depth,
            display_label: extra.display_label.clone(),
            presentation_uri: logical_uri.clone(),
            canonical_uri: logical_uri.clone(),
            logical_uri,
            root_badge: root_badge.clone(),
            parent_id: Some(parent_id),
            readiness,
            generated_artifact_hint: None,
            special_file_hint: None,
        };
        tree.insert(node).unwrap_or_else(|e| {
            panic!(
                "fixture {} could not insert node {}: {e}",
                fixture.case_id, extra.logical_path
            )
        });
    }
    tree
}

#[test]
fn breadcrumb_fixture_corpus_drives_protected_walk_and_failure_drill() {
    let root_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/explorer/breadcrumb_cases");

    let mut count = 0usize;
    for entry in std::fs::read_dir(&root_dir).expect("breadcrumb_cases directory must exist") {
        let entry = entry.expect("breadcrumb_cases directory entry must read");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let payload = std::fs::read_to_string(&path).expect("breadcrumb fixture must read");
        let fixture: BreadcrumbFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|e| panic!("breadcrumb fixture must parse: {} : {e}", path.display()));

        let mut tree = build_tree(&fixture);
        let workspace_id = fixture.workspace.workspace_id.clone();
        let root_id = fixture.workspace.root.root_id.clone();
        let root_logical = format!("aureline-ws://{workspace_id}/{root_id}/");

        let mut last_filter_outcome = None;
        let mut last_reveal_outcome = None;

        for action in &fixture.actions {
            match action {
                FixtureAction::ApplyFilter { query } => {
                    let outcome = apply_filter(&mut tree, query.clone());
                    last_filter_outcome = Some(outcome);
                }
                FixtureAction::Reveal { logical_path } => {
                    let id = id_for(&workspace_id, &root_id, &root_logical, logical_path);
                    let outcome = reveal(&mut tree, &id).unwrap_or_else(|e| {
                        panic!(
                            "case {} ({}): reveal {} must succeed: {e}",
                            fixture.case_id, fixture.title, logical_path
                        )
                    });
                    last_reveal_outcome = Some(outcome);
                }
            }
        }

        if let Some(expected) = fixture.expected_outcomes.filter.as_ref() {
            let outcome = last_filter_outcome
                .as_ref()
                .unwrap_or_else(|| panic!("case {}: expected filter outcome", fixture.case_id));
            assert_eq!(
                outcome.query, expected.query,
                "case {}: filter query mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.query_is_empty, expected.query_is_empty,
                "case {}: filter query_is_empty mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.total_nodes, expected.total_nodes,
                "case {}: filter total_nodes mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.selection_matches_filter, expected.selection_matches_filter,
                "case {}: filter selection_matches_filter mismatch",
                fixture.case_id
            );
        }

        if let Some(expected) = fixture.expected_outcomes.reveal.as_ref() {
            let outcome = last_reveal_outcome
                .as_ref()
                .unwrap_or_else(|| panic!("case {}: expected reveal outcome", fixture.case_id));
            let revealed_id = id_for(
                &workspace_id,
                &root_id,
                &root_logical,
                &expected.revealed_logical_path,
            );
            assert_eq!(
                outcome.revealed_node_id, revealed_id,
                "case {}: revealed_node_id mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.selection_node_id, revealed_id,
                "case {}: selection_node_id mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.matches_filter, expected.matches_filter,
                "case {}: matches_filter mismatch",
                fixture.case_id
            );
            assert_eq!(
                outcome.filter_query, expected.filter_query,
                "case {}: filter_query mismatch",
                fixture.case_id
            );

            let expected_chain: Vec<ExplorerNodeId> = expected
                .ancestry_logical_paths
                .iter()
                .map(|p| id_for(&workspace_id, &root_id, &root_logical, p))
                .collect();
            assert_eq!(
                outcome.ancestry_chain, expected_chain,
                "case {}: ancestry_chain mismatch",
                fixture.case_id
            );

            let bc = &outcome.breadcrumb_path;
            assert_eq!(bc.workspace_id, expected.breadcrumb_path.workspace_id);
            assert_eq!(bc.root_id, expected.breadcrumb_path.root_id);
            assert_eq!(bc.root_badge, expected.breadcrumb_path.root_badge);
            assert_eq!(
                bc.leaf_node_id,
                id_for(
                    &workspace_id,
                    &root_id,
                    &root_logical,
                    &expected.breadcrumb_path.leaf_logical_path
                ),
                "case {}: breadcrumb leaf_node_id mismatch",
                fixture.case_id
            );
            assert_eq!(
                bc.segments.len(),
                expected.breadcrumb_path.segments.len(),
                "case {}: breadcrumb segment count mismatch",
                fixture.case_id
            );
            for (i, (got, expected_segment)) in bc
                .segments
                .iter()
                .zip(&expected.breadcrumb_path.segments)
                .enumerate()
            {
                let segment_id = id_for(
                    &workspace_id,
                    &root_id,
                    &root_logical,
                    &expected_segment.logical_path,
                );
                assert_eq!(
                    got.node_id, segment_id,
                    "case {} segment {}: node_id mismatch",
                    fixture.case_id, i
                );
                assert_eq!(
                    got.display_label, expected_segment.display_label,
                    "case {} segment {}: display_label mismatch",
                    fixture.case_id, i
                );
                assert_eq!(
                    got.kind.as_str(),
                    expected_segment.kind,
                    "case {} segment {}: kind mismatch",
                    fixture.case_id,
                    i
                );
                assert_eq!(
                    got.is_root, expected_segment.is_root,
                    "case {} segment {}: is_root mismatch",
                    fixture.case_id, i
                );
                assert_eq!(
                    got.is_leaf, expected_segment.is_leaf,
                    "case {} segment {}: is_leaf mismatch",
                    fixture.case_id, i
                );
            }

            // The reveal outcome's breadcrumb_path must agree with a fresh
            // materialize_breadcrumb_path call against the same tree — the
            // explorer reveal walk and the editor breadcrumb chrome consume
            // the same identity model.
            let rederived = materialize_breadcrumb_path(&tree, &outcome.revealed_node_id)
                .expect("breadcrumb path must rederive after reveal");
            assert_eq!(
                rederived, outcome.breadcrumb_path,
                "case {}: breadcrumb projection desynchronized between reveal outcome and tree rederive",
                fixture.case_id
            );
        }

        count += 1;
    }
    assert!(
        count >= 3,
        "expected at least 3 breadcrumb fixtures, found {count}"
    );
}

#[test]
fn filter_then_reveal_keeps_node_identity_in_runtime() {
    // Failure-drill in code: independent of fixtures, build a deep tree,
    // apply a non-matching filter, reveal a leaf, and confirm the breadcrumb
    // path still resolves to the same canonical ancestry.
    let mut tree = ExplorerTree::new();
    let workspace_id = "wksp:runtime-drill";
    let root_id = "root:src";
    let root = ExplorerNode::root_mount(
        workspace_id,
        root_id,
        WorkspaceRootKind::LocalRepoRoot,
        "runtime-drill",
        NodeReadinessClass::Loaded,
    );
    let root_logical = root.logical_uri.clone();
    let root_node_id = root.node_id.clone();
    let root_badge = root.root_badge.clone();
    tree.insert(root).unwrap();

    let mut parent_id = root_node_id.clone();
    let segments = ["src", "core", "engine", "runner.rs"];
    let mut leaf_id = root_node_id.clone();
    for (i, segment) in segments.iter().enumerate() {
        let logical_path = segments[..=i].join("/");
        let logical_uri = format!("{root_logical}{logical_path}");
        let id = ExplorerNodeId::from_logical(workspace_id, root_id, &logical_uri);
        let kind = if i == segments.len() - 1 {
            ExplorerNodeKind::File
        } else {
            ExplorerNodeKind::Directory
        };
        tree.insert(ExplorerNode {
            node_id: id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind,
            depth: (i as u32) + 1,
            display_label: segment.to_string(),
            presentation_uri: logical_uri.clone(),
            canonical_uri: logical_uri.clone(),
            logical_uri,
            root_badge: root_badge.clone(),
            parent_id: Some(parent_id.clone()),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        })
        .unwrap();
        parent_id = id.clone();
        if i == segments.len() - 1 {
            leaf_id = id;
        }
    }

    // Apply a filter that does not match the leaf's label.
    let _ = apply_filter(&mut tree, Some("zzz_unmatched".to_string()));

    let outcome = reveal(&mut tree, &leaf_id).expect("reveal must succeed under active filter");
    assert_eq!(outcome.revealed_node_id, leaf_id);
    assert!(!outcome.matches_filter);
    let bc = &outcome.breadcrumb_path;
    assert_eq!(bc.segments.len(), segments.len() + 1);
    assert_eq!(bc.segments.first().unwrap().node_id, root_node_id);
    assert_eq!(bc.segments.last().unwrap().node_id, leaf_id);

    // The breadcrumb projection must round-trip through serde without
    // changing identity (support exports replay the same truth).
    let json = serde_json::to_string(&outcome.breadcrumb_path).unwrap();
    let roundtrip: aureline_shell::breadcrumbs::BreadcrumbPath =
        serde_json::from_str(&json).unwrap();
    assert_eq!(roundtrip, outcome.breadcrumb_path);
}
