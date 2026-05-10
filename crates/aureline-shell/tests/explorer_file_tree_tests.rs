//! Protected-walk integration tests for the virtualized file-tree model with
//! stable node ids and explorer actions.
//!
//! Walks the protected dogfood path named in the spec:
//!
//!   open a real workspace
//!     -> browse, reveal, and act in the explorer
//!     -> confirm node identity and actions remain stable while virtualized.
//!
//! Plus the failure drill:
//!
//!   rapidly expand / filter a large tree
//!     -> confirm node identity and explorer actions remain stable across
//!        virtualization churn.
//!
//! Each fixture under `/fixtures/explorer/file_tree_cases/*.json` describes a
//! synthetic workspace shape, a sequence of explorer actions to dispatch, and
//! the expected viewport projection plus per-action records. The tests load
//! the fixture, materialize a tree, dispatch the actions, and assert the
//! observed viewport / records match the expected truth.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::explorer::{
    dispatch, ExplorerAction, ExplorerNode, ExplorerNodeId, ExplorerNodeKind, ExplorerTree,
    GeneratedArtifactHint, NodeReadinessClass, SpecialFileHint,
};
use aureline_workspace::{RootPartialTruth, WorkspaceRootKind};

#[derive(Debug, Deserialize)]
struct FileTreeFixture {
    #[serde(default)]
    case_id: String,
    #[serde(default)]
    title: String,
    workspace: FixtureWorkspace,
    actions: Vec<ExplorerAction>,
    expected_viewport: ExpectedViewport,
    expected_records: Vec<ExpectedRecord>,
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
    #[serde(default)]
    generated_artifact_hint: Option<GeneratedArtifactHint>,
    #[serde(default)]
    special_file_hint: Option<SpecialFileHint>,
}

#[derive(Debug, Deserialize)]
struct ExpectedViewport {
    filter_query: Option<String>,
    selection_node_id: Option<String>,
    selection_in_viewport: bool,
    total_visible_rows: u32,
    rows: Vec<ExpectedRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    node_id: String,
    depth: u32,
    kind: String,
    is_expanded: bool,
    matches_filter: bool,
    readiness: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedRecord {
    action_class: String,
    command_id: String,
    outcome: String,
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
        other => panic!("unsupported root_kind in fixture: {other}"),
    }
}

fn parse_partial_truth(value: &str) -> RootPartialTruth {
    match value {
        "loaded" => RootPartialTruth::Loaded,
        "manifest_known" => RootPartialTruth::ManifestKnown,
        "cached" => RootPartialTruth::Cached,
        "unavailable" => RootPartialTruth::Unavailable,
        other => panic!("unsupported partial_truth in fixture: {other}"),
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
        other => panic!("unsupported node kind in fixture: {other}"),
    }
}

fn parse_readiness(value: &str) -> NodeReadinessClass {
    match value {
        "loaded" => NodeReadinessClass::Loaded,
        "partially_enumerated" => NodeReadinessClass::PartiallyEnumerated,
        "manifest_known" => NodeReadinessClass::ManifestKnown,
        "cached" => NodeReadinessClass::Cached,
        "unavailable" => NodeReadinessClass::Unavailable,
        other => panic!("unsupported readiness in fixture: {other}"),
    }
}

fn build_tree(fixture: &FileTreeFixture) -> ExplorerTree {
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
    tree.insert(root_node).expect("root mount must insert");

    // Sort by depth so parents come first.
    let mut sorted: Vec<&FixtureExtraNode> = fixture.workspace.extra_nodes.iter().collect();
    sorted.sort_by_key(|n| n.logical_path.matches('/').count());

    for extra in sorted {
        let kind = parse_node_kind(&extra.kind);
        let readiness = parse_readiness(&extra.readiness);
        let logical_uri = format!("{}{}", root_logical_uri, extra.logical_path);
        let canonical_uri = logical_uri.clone();
        let presentation_uri = logical_uri.clone();
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
            presentation_uri,
            canonical_uri,
            logical_uri,
            root_badge: root_badge.clone(),
            parent_id: Some(parent_id),
            readiness,
            generated_artifact_hint: extra.generated_artifact_hint.clone(),
            special_file_hint: extra.special_file_hint.clone(),
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
fn fixture_corpus_drives_protected_walk_and_failure_drill() {
    let root_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/explorer/file_tree_cases");

    let mut count = 0usize;
    for entry in std::fs::read_dir(&root_dir).expect("file_tree_cases directory must exist") {
        let entry = entry.expect("file_tree_cases directory entry must read");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let payload = std::fs::read_to_string(&path).expect("file_tree fixture must read");
        let fixture: FileTreeFixture = serde_json::from_str(&payload).unwrap_or_else(|e| {
            panic!(
                "file_tree fixture must parse: {} ({}): {e}",
                path.display(),
                payload.lines().next().unwrap_or("")
            )
        });

        let mut tree = build_tree(&fixture);
        let mut records = Vec::with_capacity(fixture.actions.len());
        for action in &fixture.actions {
            let record = dispatch(&mut tree, action).unwrap_or_else(|e| {
                panic!(
                    "case {} ({}): action {:?} must dispatch: {e}",
                    fixture.case_id, fixture.title, action
                )
            });
            records.push(record);
        }

        assert_eq!(
            records.len(),
            fixture.expected_records.len(),
            "case {} ({}): expected {} records, got {}",
            fixture.case_id,
            fixture.title,
            fixture.expected_records.len(),
            records.len()
        );
        for (i, (got, expected)) in records.iter().zip(&fixture.expected_records).enumerate() {
            assert_eq!(
                got.action_class, expected.action_class,
                "case {} step {}: action_class mismatch",
                fixture.case_id, i
            );
            assert_eq!(
                got.command_id, expected.command_id,
                "case {} step {}: command_id mismatch",
                fixture.case_id, i
            );
            assert_eq!(
                got.outcome, expected.outcome,
                "case {} step {}: outcome mismatch",
                fixture.case_id, i
            );
        }

        // Materialize the entire visible row set; total_visible_rows must
        // match and the rows must line up with the expected projection in
        // document order (depth + node id + kind + flags).
        let viewport = tree.viewport(
            0,
            fixture
                .expected_viewport
                .total_visible_rows
                .saturating_add(8),
        );
        assert_eq!(
            viewport.total_visible_rows, fixture.expected_viewport.total_visible_rows,
            "case {}: total_visible_rows mismatch",
            fixture.case_id
        );
        assert_eq!(
            viewport.filter_query, fixture.expected_viewport.filter_query,
            "case {}: filter_query mismatch",
            fixture.case_id
        );
        assert_eq!(
            viewport
                .selection_node_id
                .as_ref()
                .map(|id| id.as_str().to_string()),
            fixture.expected_viewport.selection_node_id.clone(),
            "case {}: selection_node_id mismatch",
            fixture.case_id
        );
        assert_eq!(
            viewport.selection_in_viewport, fixture.expected_viewport.selection_in_viewport,
            "case {}: selection_in_viewport mismatch",
            fixture.case_id
        );
        assert_eq!(
            viewport.rows.len(),
            fixture.expected_viewport.rows.len(),
            "case {}: row count mismatch (got {} rows, expected {})",
            fixture.case_id,
            viewport.rows.len(),
            fixture.expected_viewport.rows.len()
        );
        for (i, (got, expected)) in viewport
            .rows
            .iter()
            .zip(&fixture.expected_viewport.rows)
            .enumerate()
        {
            assert_eq!(
                got.node_id.as_str(),
                expected.node_id,
                "case {} row {}: node_id mismatch",
                fixture.case_id,
                i
            );
            assert_eq!(
                got.depth, expected.depth,
                "case {} row {}: depth mismatch",
                fixture.case_id, i
            );
            assert_eq!(
                got.kind.as_str(),
                expected.kind,
                "case {} row {}: kind mismatch",
                fixture.case_id,
                i
            );
            assert_eq!(
                got.is_expanded, expected.is_expanded,
                "case {} row {}: is_expanded mismatch",
                fixture.case_id, i
            );
            assert_eq!(
                got.matches_filter, expected.matches_filter,
                "case {} row {}: matches_filter mismatch",
                fixture.case_id, i
            );
            assert_eq!(
                got.readiness.as_str(),
                expected.readiness,
                "case {} row {}: readiness mismatch",
                fixture.case_id,
                i
            );
        }
        count += 1;
    }
    assert!(
        count >= 4,
        "expected at least 4 file_tree fixtures, found {count}"
    );
}

#[test]
fn node_ids_survive_filter_and_expansion_churn() {
    // Failure drill in code: rapid expand / filter cycle keeps node identity
    // stable. The selected node id must persist across churn even when the
    // row falls outside the visible set.
    let mut tree = ExplorerTree::new();
    let workspace_id = "wksp:churn-runtime";
    let root_id = "root:churn-runtime";
    let root = ExplorerNode::root_mount(
        workspace_id,
        root_id,
        WorkspaceRootKind::LocalRepoRoot,
        "churn-runtime",
        NodeReadinessClass::Loaded,
    );
    let root_node_id = root.node_id.clone();
    tree.insert(root).unwrap();

    let mut tracked_ids: Vec<ExplorerNodeId> = Vec::new();
    for i in 0..50 {
        let logical_uri = format!("aureline-ws://{workspace_id}/{root_id}/file_{i:03}.rs",);
        let node = ExplorerNode {
            node_id: ExplorerNodeId::from_logical(workspace_id, root_id, &logical_uri),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::File,
            depth: 1,
            display_label: format!("file_{i:03}.rs"),
            presentation_uri: logical_uri.clone(),
            canonical_uri: logical_uri.clone(),
            logical_uri,
            root_badge: "local".to_string(),
            parent_id: Some(root_node_id.clone()),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        };
        tracked_ids.push(node.node_id.clone());
        tree.insert(node).unwrap();
    }

    // Select a node that will fall outside the filter; identity must persist.
    let selected = tracked_ids[7].clone();
    dispatch(
        &mut tree,
        &ExplorerAction::Select {
            node_id: selected.clone(),
        },
    )
    .unwrap();

    // Now churn: rapid filter + clear cycle. Each cycle reconstructs the
    // visible row set from scratch but must never mutate node identities.
    for cycle in 0..10 {
        let q = format!("file_{:03}", cycle * 5);
        dispatch(
            &mut tree,
            &ExplorerAction::SetFilter {
                query: Some(q.clone()),
            },
        )
        .unwrap();
        let viewport_filtered = tree.viewport(0, 100);
        // Selection survives even when filtered out.
        assert_eq!(
            viewport_filtered.selection_node_id.as_ref(),
            Some(&selected)
        );

        dispatch(&mut tree, &ExplorerAction::SetFilter { query: None }).unwrap();
        let viewport_clear = tree.viewport(0, 100);
        assert_eq!(viewport_clear.total_visible_rows, 51);
        // Every tracked id is still present and resolves to the same node.
        for id in &tracked_ids {
            let row = viewport_clear
                .rows
                .iter()
                .find(|r| &r.node_id == id)
                .unwrap_or_else(|| panic!("node id {id} disappeared after churn"));
            assert_eq!(row.kind, ExplorerNodeKind::File);
        }
    }
}

#[test]
fn placeholder_lifecycle_creates_and_removes_without_touching_canonical_files() {
    let mut tree = ExplorerTree::new();
    let workspace_id = "wksp:placeholder";
    let root_id = "root:placeholder";
    let root = ExplorerNode::root_mount(
        workspace_id,
        root_id,
        WorkspaceRootKind::LocalRepoRoot,
        "placeholder",
        NodeReadinessClass::Loaded,
    );
    let root_node_id = root.node_id.clone();
    tree.insert(root).unwrap();

    let create_record = dispatch(
        &mut tree,
        &ExplorerAction::CreatePlaceholder {
            parent_id: root_node_id.clone(),
            display_label: "draft.md".to_string(),
            kind: ExplorerNodeKind::File,
        },
    )
    .unwrap();
    assert_eq!(create_record.outcome, "applied");
    let new_id = create_record
        .created_node_id
        .clone()
        .expect("placeholder must produce a new node id");
    assert!(tree.node(&new_id).is_some());
    assert!(tree.node(&new_id).unwrap().special_file_hint.is_some());
    assert_eq!(
        create_record.command_id,
        "cmd:workspace.explorer_create_placeholder"
    );

    let remove_record = dispatch(
        &mut tree,
        &ExplorerAction::RemovePlaceholder {
            node_id: new_id.clone(),
        },
    )
    .unwrap();
    assert_eq!(remove_record.outcome, "applied");
    assert!(tree.node(&new_id).is_none());

    // RemovePlaceholder on a non-placeholder must reject without touching
    // the tree.
    let reject = dispatch(
        &mut tree,
        &ExplorerAction::RemovePlaceholder {
            node_id: root_node_id.clone(),
        },
    )
    .unwrap();
    assert_eq!(reject.outcome, "rejected");
    assert!(tree.node(&root_node_id).is_some());
}

#[test]
fn collapse_does_not_remove_root_mount_from_tree() {
    let mut tree = ExplorerTree::new();
    let workspace_id = "wksp:collapse";
    let root_id = "root:collapse";
    let root = ExplorerNode::root_mount(
        workspace_id,
        root_id,
        WorkspaceRootKind::LocalRepoRoot,
        "collapse",
        NodeReadinessClass::Loaded,
    );
    let root_id_handle = root.node_id.clone();
    tree.insert(root).unwrap();

    let record = dispatch(
        &mut tree,
        &ExplorerAction::Collapse {
            node_id: root_id_handle.clone(),
        },
    )
    .unwrap();
    assert_eq!(record.outcome, "no_op");
    let viewport = tree.viewport(0, 8);
    assert_eq!(viewport.total_visible_rows, 1);
    assert_eq!(viewport.rows[0].node_id, root_id_handle);
    assert!(viewport.rows[0].is_expanded);
}

#[test]
fn refresh_records_new_readiness_class() {
    let mut tree = ExplorerTree::new();
    let root = ExplorerNode::root_mount(
        "wksp:refresh",
        "root:refresh",
        WorkspaceRootKind::RemoteRepository,
        "refresh",
        NodeReadinessClass::ManifestKnown,
    );
    let root_id = root.node_id.clone();
    tree.insert(root).unwrap();

    let record = dispatch(
        &mut tree,
        &ExplorerAction::Refresh {
            node_id: root_id.clone(),
            readiness: NodeReadinessClass::Loaded,
        },
    )
    .unwrap();
    assert_eq!(record.outcome, "applied");
    assert_eq!(
        tree.node(&root_id).unwrap().readiness,
        NodeReadinessClass::Loaded
    );
}

#[test]
fn unknown_node_dispatch_returns_typed_error() {
    let mut tree = ExplorerTree::new();
    let bogus = ExplorerNodeId::from_logical("wksp:unknown", "root:unknown", "logical://unknown");
    let err = dispatch(
        &mut tree,
        &ExplorerAction::Open {
            node_id: bogus.clone(),
        },
    )
    .expect_err("unknown node must error");
    assert!(format!("{err}").contains("unknown explorer node id"));
}
