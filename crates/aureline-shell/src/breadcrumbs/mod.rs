//! Path-ancestry breadcrumbs for the editor chrome.
//!
//! The editor breadcrumb surface and the explorer reveal walk share one
//! identity model. Both sides consume the same [`ExplorerNodeId`] / canonical
//! `(workspace_id, root_id, logical_uri)` tuple the workspace surface owns —
//! so a reveal in the explorer and the breadcrumb trail in the editor never
//! disagree about which node is "current". This module is the projection
//! layer that turns an [`ExplorerTree`] node into a serializable
//! [`BreadcrumbPath`] without re-deriving filesystem identity locally.
//!
//! ## Why a separate module
//!
//! The editor chrome must render breadcrumbs for files that may currently be
//! filtered out of the explorer viewport. Re-using the explorer tree's
//! `parent_id` walk (rather than parsing a presentation URI) keeps both
//! surfaces honest about the canonical path — even after a filter cycle, a
//! virtualization churn, or a restore.
//!
//! ## Failure-drill posture
//!
//! Filter-then-reveal a deep path: the breadcrumb trail must still resolve
//! to the same node identities the explorer would re-mount on filter clear.
//! The fixtures under `/fixtures/explorer/breadcrumb_cases/*.json` exercise
//! this drill against the same identity vocabulary.

use serde::{Deserialize, Serialize};

use aureline_navigation::target_model::{
    ContinuityArtifactKind, ContinuityState, DowngradeReason, TargetContinuityRef,
};

use crate::explorer::{ExplorerNodeId, ExplorerNodeKind, ExplorerTree};

const BREADCRUMB_PATH_SCHEMA_VERSION: u32 = 1;

/// One segment of a breadcrumb trail. The segment carries the canonical /
/// logical / presentation URIs alongside the explorer's [`ExplorerNodeId`]
/// so editor chrome surfaces never have to parse paths to render or to
/// dispatch a "navigate to ancestor" action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbSegment {
    pub node_id: ExplorerNodeId,
    pub display_label: String,
    pub kind: ExplorerNodeKind,
    pub depth: u32,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub is_root: bool,
    pub is_leaf: bool,
}

/// Full breadcrumb path for a single revealed / opened node.
///
/// `segments` is ordered from the root mount down to the leaf node. The
/// `presentation_path` / `canonical_path` / `logical_path` strings are the
/// joined display forms — the canonical truth for "where the editor thinks
/// the user is". Every URI matches the corresponding URI in the trailing
/// segment so the editor chrome can render either a flat path string or a
/// chip stack interchangeably.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbPath {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub root_id: String,
    pub root_badge: String,
    pub leaf_node_id: ExplorerNodeId,
    pub presentation_path: String,
    pub canonical_path: String,
    pub logical_path: String,
    pub segments: Vec<BreadcrumbSegment>,
}

/// Materialize the breadcrumb path for `leaf` from the explorer tree's
/// stable parent linkage. Returns `None` when `leaf` is not present in the
/// tree.
pub fn materialize_breadcrumb_path(
    tree: &ExplorerTree,
    leaf: &ExplorerNodeId,
) -> Option<BreadcrumbPath> {
    let leaf_node = tree.node(leaf)?;
    let mut chain: Vec<ExplorerNodeId> = vec![leaf.clone()];
    let mut current = leaf_node.parent_id.clone();
    while let Some(parent_id) = current {
        let Some(parent) = tree.node(&parent_id) else {
            return None;
        };
        chain.push(parent_id.clone());
        current = parent.parent_id.clone();
    }
    chain.reverse();

    let last_index = chain.len() - 1;
    let mut segments: Vec<BreadcrumbSegment> = Vec::with_capacity(chain.len());
    for (idx, node_id) in chain.iter().enumerate() {
        let node = tree.node(node_id)?;
        segments.push(BreadcrumbSegment {
            node_id: node_id.clone(),
            display_label: node.display_label.clone(),
            kind: node.kind,
            depth: node.depth,
            presentation_uri: node.presentation_uri.clone(),
            canonical_uri: node.canonical_uri.clone(),
            logical_uri: node.logical_uri.clone(),
            is_root: idx == 0,
            is_leaf: idx == last_index,
        });
    }

    Some(BreadcrumbPath {
        record_kind: "breadcrumb_path_record".to_string(),
        schema_version: BREADCRUMB_PATH_SCHEMA_VERSION,
        workspace_id: leaf_node.workspace_id.clone(),
        root_id: leaf_node.root_id.clone(),
        root_badge: leaf_node.root_badge.clone(),
        leaf_node_id: leaf.clone(),
        presentation_path: leaf_node.presentation_uri.clone(),
        canonical_path: leaf_node.canonical_uri.clone(),
        logical_path: leaf_node.logical_uri.clone(),
        segments,
    })
}

/// Render a single-line presentation form of the breadcrumb path. Suitable
/// for terminal-style status bars and a11y exports; UI surfaces should
/// project [`BreadcrumbPath::segments`] directly.
pub fn breadcrumb_lines(path: &BreadcrumbPath) -> Vec<String> {
    let trail: Vec<String> = path
        .segments
        .iter()
        .map(|segment| segment.display_label.clone())
        .collect();
    vec![
        format!(
            "[{badge}] {trail}",
            badge = path.root_badge,
            trail = trail.join(" / ")
        ),
        format!("presentation: {}", path.presentation_path),
        format!("canonical:    {}", path.canonical_path),
        format!("logical:      {}", path.logical_path),
    ]
}

/// Projects a breadcrumb path into the shared semantic target-continuity model.
///
/// The returned target ref is the opaque explorer leaf id, not a raw path. A
/// drifted breadcrumb must supply either a disambiguation set or downgrade
/// reasons so reopen code does not silently jump to a guessed successor.
pub fn breadcrumb_target_continuity_ref(
    path: &BreadcrumbPath,
    continuity_state: ContinuityState,
    remapped_target_ref: Option<String>,
    disambiguation_set_ref: Option<String>,
    downgrade_reasons: Vec<DowngradeReason>,
) -> TargetContinuityRef {
    TargetContinuityRef {
        continuity_ref_id: format!("nav:continuity:breadcrumb:{}", path.leaf_node_id.as_str()),
        artifact_kind: ContinuityArtifactKind::Breadcrumb,
        target_ref: path.leaf_node_id.as_str().to_owned(),
        continuity_state,
        remapped_target_ref,
        disambiguation_set_ref,
        downgrade_reasons,
        summary: format!(
            "Breadcrumb leaf {} is projected through the shared navigation target continuity model.",
            path.leaf_node_id.as_str()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::explorer::{ExplorerNode, NodeReadinessClass};
    use aureline_workspace::WorkspaceRootKind;

    fn three_level_tree() -> (ExplorerTree, ExplorerNodeId, ExplorerNodeId) {
        let mut tree = ExplorerTree::new();
        let workspace_id = "wksp:bc";
        let root_id = "root:bc";
        let root = ExplorerNode::root_mount(
            workspace_id,
            root_id,
            WorkspaceRootKind::LocalRepoRoot,
            "bc",
            NodeReadinessClass::Loaded,
        );
        let root_node_id = root.node_id.clone();
        let root_logical = root.logical_uri.clone();
        let root_badge = root.root_badge.clone();
        tree.insert(root).unwrap();

        let dir_logical = format!("{root_logical}crates");
        let dir_id = ExplorerNodeId::from_logical(workspace_id, root_id, &dir_logical);
        tree.insert(ExplorerNode {
            node_id: dir_id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::Directory,
            depth: 1,
            display_label: "crates".to_string(),
            presentation_uri: dir_logical.clone(),
            canonical_uri: dir_logical.clone(),
            logical_uri: dir_logical,
            root_badge: root_badge.clone(),
            parent_id: Some(root_node_id.clone()),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        })
        .unwrap();

        let leaf_logical = format!("{root_logical}crates/lib.rs");
        let leaf_id = ExplorerNodeId::from_logical(workspace_id, root_id, &leaf_logical);
        tree.insert(ExplorerNode {
            node_id: leaf_id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::File,
            depth: 2,
            display_label: "lib.rs".to_string(),
            presentation_uri: leaf_logical.clone(),
            canonical_uri: leaf_logical.clone(),
            logical_uri: leaf_logical,
            root_badge,
            parent_id: Some(dir_id),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        })
        .unwrap();

        (tree, root_node_id, leaf_id)
    }

    #[test]
    fn materialize_returns_segments_root_to_leaf() {
        let (tree, root_id, leaf_id) = three_level_tree();
        let path = materialize_breadcrumb_path(&tree, &leaf_id).expect("path must exist");
        assert_eq!(path.segments.len(), 3);
        assert!(path.segments[0].is_root);
        assert_eq!(path.segments[0].node_id, root_id);
        assert!(path.segments.last().unwrap().is_leaf);
        assert_eq!(path.segments.last().unwrap().node_id, leaf_id);
        assert_eq!(path.segments[1].display_label, "crates");
    }

    #[test]
    fn materialize_unknown_node_returns_none() {
        let (tree, _root_id, _leaf_id) = three_level_tree();
        let bogus = ExplorerNodeId::from_logical("wksp:bc", "root:bc", "logical://missing");
        assert!(materialize_breadcrumb_path(&tree, &bogus).is_none());
    }

    #[test]
    fn breadcrumb_lines_includes_uri_truths() {
        let (tree, _root_id, leaf_id) = three_level_tree();
        let path = materialize_breadcrumb_path(&tree, &leaf_id).expect("path must exist");
        let lines = breadcrumb_lines(&path);
        assert!(lines.iter().any(|l| l.contains("presentation:")));
        assert!(lines.iter().any(|l| l.contains("canonical:")));
        assert!(lines.iter().any(|l| l.contains("logical:")));
        assert!(lines[0].contains("bc / crates / lib.rs"));
    }

    #[test]
    fn breadcrumb_projects_shared_continuity_target() {
        let (tree, _root_id, leaf_id) = three_level_tree();
        let path = materialize_breadcrumb_path(&tree, &leaf_id).expect("path must exist");
        let continuity =
            breadcrumb_target_continuity_ref(&path, ContinuityState::Bound, None, None, vec![]);
        assert_eq!(continuity.target_ref, leaf_id.as_str());
        assert_eq!(continuity.artifact_kind, ContinuityArtifactKind::Breadcrumb);
        assert_eq!(continuity.continuity_state, ContinuityState::Bound);
    }
}
