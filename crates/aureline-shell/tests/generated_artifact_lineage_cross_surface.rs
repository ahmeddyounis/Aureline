//! Cross-surface protected-walk for generated-artifact lineage hints.
//!
//! Builds a synthetic workspace with one canonical source (`Cargo.toml`) and
//! one resolver-generated artifact (`Cargo.lock`). Then drives **both**
//! protected-row consumers — the explorer and the workspace search shell —
//! and asserts each surface labels the generated row distinctly without ever
//! implying the lockfile is the canonical edit target.

use aureline_reactive_state::ReadinessLabel;
use aureline_shell::explorer::{
    ExplorerNode, ExplorerNodeId, ExplorerNodeKind, ExplorerTree, GeneratedArtifactHint,
    NodeReadinessClass,
};
use aureline_shell::search_shell::WorkspaceSearchSurfaceState;
use aureline_workspace::{
    GeneratedArtifactClass, ScopeClass as WorkspaceScopeClass, TrustState,
    WorkspaceLifecycleMachine,
};

const WORKSPACE_ID: &str = "wksp:lineage";
const ROOT_ID: &str = "root:repo";

fn ready_lifecycle() -> WorkspaceLifecycleMachine {
    let mut machine = WorkspaceLifecycleMachine::discovered(WORKSPACE_ID, "mono:0");
    machine.open_workspace("mono:1");
    machine.resolve_trust(TrustState::Trusted, "mono:2");
    machine.mark_shell_interactive("mono:3");
    machine.update_readiness_gates(
        Some(aureline_vfs::WatcherHealth::Healthy),
        Some(true),
        Some(true),
        "mono:4",
        None,
    );
    machine
}

fn root_logical_uri(workspace_id: &str, root_id: &str) -> String {
    format!("aureline-ws://{workspace_id}/{root_id}/")
}

fn child_node(
    parent_id: &ExplorerNodeId,
    relative_path: &str,
    display_label: &str,
    kind: ExplorerNodeKind,
    readiness: NodeReadinessClass,
) -> ExplorerNode {
    let logical_uri = format!("{}{relative_path}", root_logical_uri(WORKSPACE_ID, ROOT_ID));
    let canonical_uri = logical_uri.clone();
    let presentation_uri = logical_uri.clone();
    let depth = (relative_path.matches('/').count() as u32) + 1;
    ExplorerNode {
        node_id: ExplorerNodeId::from_logical(WORKSPACE_ID, ROOT_ID, &logical_uri),
        workspace_id: WORKSPACE_ID.to_string(),
        root_id: ROOT_ID.to_string(),
        root_kind: aureline_workspace::WorkspaceRootKind::LocalRepoRoot,
        kind,
        depth,
        display_label: display_label.to_string(),
        presentation_uri,
        canonical_uri,
        logical_uri,
        root_badge: aureline_workspace::WorkspaceRootKind::LocalRepoRoot
            .root_badge()
            .to_string(),
        parent_id: Some(parent_id.clone()),
        readiness,
        generated_artifact_hint: GeneratedArtifactHint::detect_for(
            relative_path,
            Some(WORKSPACE_ID),
            Some(ROOT_ID),
        ),
        special_file_hint: None,
    }
}

#[test]
fn explorer_and_search_shell_jointly_label_generated_lockfile() {
    // ---- Build the explorer tree ------------------------------------------
    let mut tree = ExplorerTree::new();
    let root = ExplorerNode::root_mount(
        WORKSPACE_ID,
        ROOT_ID,
        aureline_workspace::WorkspaceRootKind::LocalRepoRoot,
        "lineage-repo",
        NodeReadinessClass::Loaded,
    );
    let root_id_handle = root.node_id.clone();
    tree.insert(root).expect("root mount must insert");

    let lockfile_node = child_node(
        &root_id_handle,
        "Cargo.lock",
        "Cargo.lock",
        ExplorerNodeKind::GeneratedArtifact,
        NodeReadinessClass::Loaded,
    );
    let lockfile_id = lockfile_node.node_id.clone();
    let manifest_node = child_node(
        &root_id_handle,
        "Cargo.toml",
        "Cargo.toml",
        ExplorerNodeKind::File,
        NodeReadinessClass::Loaded,
    );
    let main_rs_node = child_node(
        &root_id_handle,
        "src/main.rs",
        "main.rs",
        ExplorerNodeKind::File,
        NodeReadinessClass::Loaded,
    );

    tree.insert(lockfile_node).expect("lockfile node");
    tree.insert(manifest_node).expect("manifest node");
    // src dir node so main.rs has a parent (we don't need it in this test
    // since src is not under the explorer's interesting subtree, but skip it
    // to keep the assertions focused).
    let _ = main_rs_node;

    // The lockfile node carries a hint that points at Cargo.toml; the
    // canonical sibling carries no hint.
    let lockfile = tree.node(&lockfile_id).expect("lockfile node present");
    let hint = lockfile
        .generated_artifact_hint
        .as_ref()
        .expect("lockfile must carry lineage hint in explorer");
    assert_eq!(hint.lineage_class(), Some(GeneratedArtifactClass::Lockfile));
    assert_eq!(
        hint.generated_from_uri.as_deref(),
        Some("aureline-ws://wksp:lineage/root:repo/Cargo.toml"),
    );
    assert!(hint.has_source_canonical());

    let manifest_id = ExplorerNodeId::from_logical(
        WORKSPACE_ID,
        ROOT_ID,
        &format!(
            "{}{}",
            root_logical_uri(WORKSPACE_ID, ROOT_ID),
            "Cargo.toml"
        ),
    );
    let manifest = tree.node(&manifest_id).expect("manifest node present");
    assert!(
        manifest.generated_artifact_hint.is_none(),
        "canonical source must not carry a generated lineage hint",
    );

    // ---- Drive the workspace search shell ---------------------------------
    let lifecycle = ready_lifecycle();
    let mut surface = WorkspaceSearchSurfaceState::open(
        &lifecycle,
        ReadinessLabel::Exact,
        WorkspaceScopeClass::CurrentRepo,
        None,
        vec![
            "Cargo.lock".to_string(),
            "Cargo.toml".to_string(),
            "src/main.rs".to_string(),
        ],
    );
    surface.set_query("cargo");

    let card = surface.render_card();
    assert_eq!(card.readiness_class_token, "ready");

    let lockfile_item = card
        .rows
        .iter()
        .flat_map(|row| row.items.iter())
        .find(|item| item.relative_path == "Cargo.lock")
        .expect("Cargo.lock row must surface in the search card");
    let lockfile_hint = lockfile_item
        .generated_artifact_hint
        .as_ref()
        .expect("lockfile row must carry a search-shell lineage hint");
    assert_eq!(lockfile_hint.generated_class_token, "lockfile");
    assert_eq!(
        lockfile_hint.source_canonical_relative_path.as_deref(),
        Some("Cargo.toml")
    );
    assert_eq!(
        lockfile_hint.freshness_class_token,
        "derived_from_canonical"
    );
    assert_eq!(lockfile_hint.rule_id, "lockfile.cargo");

    let manifest_item = card
        .rows
        .iter()
        .flat_map(|row| row.items.iter())
        .find(|item| item.relative_path == "Cargo.toml")
        .expect("Cargo.toml row must surface in the search card");
    assert!(
        manifest_item.generated_artifact_hint.is_none(),
        "canonical source row must NOT carry a search-shell lineage hint",
    );

    // The cross-surface assertion: the URI the explorer points at MUST
    // reference the same source-canonical relative path the search shell
    // surfaces. The two consumers cite the same lineage truth without
    // forking the rule set.
    let explorer_canonical_uri = hint
        .generated_from_uri
        .as_deref()
        .expect("explorer hint URI");
    let search_canonical_relative = lockfile_hint
        .source_canonical_relative_path
        .as_deref()
        .expect("search hint relative path");
    assert!(
        explorer_canonical_uri.ends_with(search_canonical_relative),
        "explorer canonical URI {explorer_canonical_uri} must end with the same relative path the search shell surfaces ({search_canonical_relative})",
    );
}

#[test]
fn ordinary_files_keep_no_lineage_hint_on_either_surface() {
    // Failure-drill rule: a hand-authored row never gets a generated label
    // just because the catalog is wired up.
    let lifecycle = ready_lifecycle();
    let mut surface = WorkspaceSearchSurfaceState::open(
        &lifecycle,
        ReadinessLabel::Exact,
        WorkspaceScopeClass::CurrentRepo,
        None,
        vec!["src/main.rs".to_string(), "README.md".to_string()],
    );
    surface.set_query("main");
    let card = surface.render_card();
    for row in &card.rows {
        for item in &row.items {
            assert!(
                item.generated_artifact_hint.is_none(),
                "ordinary row {} must not carry a lineage hint",
                item.relative_path
            );
        }
    }

    assert!(
        GeneratedArtifactHint::detect_for("src/main.rs", Some(WORKSPACE_ID), Some(ROOT_ID))
            .is_none(),
        "explorer detector must not produce a hint for hand-authored sources",
    );
}
