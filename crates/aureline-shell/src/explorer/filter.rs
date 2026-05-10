//! Explorer filtering and reveal-current-item projections.
//!
//! [`ExplorerTree`] already owns the canonical filter / reveal state machine.
//! This module is the projection layer the live shell, command palette, and
//! breadcrumbs surface consume. Every projection is derived from
//! [`ExplorerNodeId`]s that survive virtualization, filter cycles, and
//! restore — so filter cycles and reveal walks never desynchronize from the
//! workspace identity model.
//!
//! Two record types are produced here:
//!
//! - [`ExplorerFilterOutcome`] — what changed when a filter query was applied,
//!   including total visible / hidden counts and whether the active selection
//!   is still a filter match.
//! - [`ExplorerRevealOutcome`] — the canonical ancestry chain and the
//!   breadcrumb path the editor chrome should render after a reveal.
//!
//! Both records are serializable so support exports replay the same truth the
//! live runtime saw on the protected reveal walk.

use serde::{Deserialize, Serialize};

use super::node::ExplorerNodeId;
use super::tree::{ExplorerTree, ExplorerTreeError};
use crate::breadcrumbs::{materialize_breadcrumb_path, BreadcrumbPath};

const EXPLORER_FILTER_SCHEMA_VERSION: u32 = 1;
const EXPLORER_REVEAL_SCHEMA_VERSION: u32 = 1;

/// Outcome of an [`apply_filter`] call against the tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerFilterOutcome {
    pub record_kind: String,
    pub schema_version: u32,
    pub query: Option<String>,
    pub query_is_empty: bool,
    pub total_nodes: u32,
    pub total_visible_rows: u32,
    pub total_matching_rows: u32,
    pub total_hidden_rows: u32,
    pub selection_node_id: Option<ExplorerNodeId>,
    pub selection_in_viewport: bool,
    pub selection_matches_filter: bool,
}

/// Outcome of a [`reveal`] call against the tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerRevealOutcome {
    pub record_kind: String,
    pub schema_version: u32,
    pub revealed_node_id: ExplorerNodeId,
    pub selection_node_id: ExplorerNodeId,
    /// Ancestor chain ordered from the root mount down to the immediate
    /// parent of the revealed node. Empty when the revealed node is itself a
    /// root mount.
    pub ancestry_chain: Vec<ExplorerNodeId>,
    pub matches_filter: bool,
    pub filter_query: Option<String>,
    pub breadcrumb_path: BreadcrumbPath,
}

/// Apply a filter query and report what changed.
///
/// Selection is preserved across filter changes. The returned outcome reports
/// whether the selected row still matches the new query so the chrome can
/// honestly disclose "selection retained but currently filtered out".
pub fn apply_filter(tree: &mut ExplorerTree, query: Option<String>) -> ExplorerFilterOutcome {
    let normalized = query.as_ref().map(|q| q.to_owned());
    tree.set_filter(normalized.clone());

    let total_nodes = tree.node_count() as u32;
    let visible = tree.visible_rows();
    let total_visible_rows = visible.len() as u32;
    let total_matching_rows = visible.iter().filter(|row| row.matches_filter).count() as u32;
    let total_hidden_rows = total_nodes.saturating_sub(total_visible_rows);

    let selection_node_id = tree.selected().cloned();
    let selection_matches_filter = match selection_node_id.as_ref() {
        Some(id) => tree
            .node(id)
            .map(|node| query_matches(normalized.as_deref(), &node.display_label))
            .unwrap_or(false),
        None => false,
    };
    let selection_in_viewport = match selection_node_id.as_ref() {
        Some(id) => visible.iter().any(|row| &row.node_id == id),
        None => false,
    };

    ExplorerFilterOutcome {
        record_kind: "explorer_filter_outcome".to_string(),
        schema_version: EXPLORER_FILTER_SCHEMA_VERSION,
        query: normalized.clone(),
        query_is_empty: normalized.as_deref().map(str::is_empty).unwrap_or(true),
        total_nodes,
        total_visible_rows,
        total_matching_rows,
        total_hidden_rows,
        selection_node_id,
        selection_in_viewport,
        selection_matches_filter,
    }
}

/// Reveal a node and project the canonical ancestry / breadcrumb truth.
///
/// The reveal expands every ancestor, selects the revealed node, and returns
/// a [`BreadcrumbPath`] derived from the same stable [`ExplorerNodeId`]s the
/// explorer chrome already renders. The outcome is the canonical handoff to
/// the editor breadcrumb surface; both surfaces stay synchronized as long as
/// they consume this record.
///
/// When a filter is active, the reveal still expands ancestors; the outcome's
/// `matches_filter` reports whether the revealed node would also be visible
/// under the current query so callers can decide whether to clear the filter.
pub fn reveal(
    tree: &mut ExplorerTree,
    id: &ExplorerNodeId,
) -> Result<ExplorerRevealOutcome, ExplorerTreeError> {
    let chain = tree.reveal(id)?;
    let breadcrumb_path = materialize_breadcrumb_path(tree, id)
        .ok_or_else(|| ExplorerTreeError::UnknownNode(id.clone()))?;
    let filter_query = tree.filter().query.clone();
    let matches_filter = match (filter_query.as_deref(), tree.node(id)) {
        (None, _) => true,
        (Some(q), _) if q.is_empty() => true,
        (Some(q), Some(node)) => {
            node.display_label
                .to_ascii_lowercase()
                .contains(&q.to_ascii_lowercase())
        }
        (Some(_), None) => false,
    };

    Ok(ExplorerRevealOutcome {
        record_kind: "explorer_reveal_outcome".to_string(),
        schema_version: EXPLORER_REVEAL_SCHEMA_VERSION,
        revealed_node_id: id.clone(),
        selection_node_id: id.clone(),
        ancestry_chain: chain,
        matches_filter,
        filter_query,
        breadcrumb_path,
    })
}

/// Walk the parent chain for `id`, returning the ancestor node ids ordered
/// from the root mount down to the immediate parent. Returns an empty vector
/// when `id` is itself a root mount or when the node is unknown.
pub fn ancestry_chain(tree: &ExplorerTree, id: &ExplorerNodeId) -> Vec<ExplorerNodeId> {
    let mut chain: Vec<ExplorerNodeId> = Vec::new();
    let mut current = tree.node(id).and_then(|node| node.parent_id.clone());
    while let Some(parent_id) = current {
        chain.push(parent_id.clone());
        current = tree.node(&parent_id).and_then(|node| node.parent_id.clone());
    }
    chain.reverse();
    chain
}

fn query_matches(query: Option<&str>, label: &str) -> bool {
    match query {
        None => true,
        Some(q) if q.is_empty() => true,
        Some(q) => label.to_ascii_lowercase().contains(&q.to_ascii_lowercase()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::explorer::{ExplorerNode, ExplorerNodeKind, NodeReadinessClass};
    use aureline_workspace::WorkspaceRootKind;

    fn build_tree() -> (ExplorerTree, ExplorerNodeId, ExplorerNodeId, ExplorerNodeId) {
        let mut tree = ExplorerTree::new();
        let workspace_id = "wksp:filter";
        let root_id = "root:filter";
        let root = ExplorerNode::root_mount(
            workspace_id,
            root_id,
            WorkspaceRootKind::LocalRepoRoot,
            "filter",
            NodeReadinessClass::Loaded,
        );
        let root_node_id = root.node_id.clone();
        let root_badge = root.root_badge.clone();
        let root_logical = root.logical_uri.clone();
        tree.insert(root).unwrap();

        let dir_logical = format!("{root_logical}src");
        let dir_id = ExplorerNodeId::from_logical(workspace_id, root_id, &dir_logical);
        tree.insert(ExplorerNode {
            node_id: dir_id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::Directory,
            depth: 1,
            display_label: "src".to_string(),
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

        let nested_logical = format!("{root_logical}src/payments/charge.rs");
        let nested_dir_logical = format!("{root_logical}src/payments");
        let nested_dir_id = ExplorerNodeId::from_logical(workspace_id, root_id, &nested_dir_logical);
        tree.insert(ExplorerNode {
            node_id: nested_dir_id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::Directory,
            depth: 2,
            display_label: "payments".to_string(),
            presentation_uri: nested_dir_logical.clone(),
            canonical_uri: nested_dir_logical.clone(),
            logical_uri: nested_dir_logical,
            root_badge: root_badge.clone(),
            parent_id: Some(dir_id.clone()),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        })
        .unwrap();

        let leaf_id = ExplorerNodeId::from_logical(workspace_id, root_id, &nested_logical);
        tree.insert(ExplorerNode {
            node_id: leaf_id.clone(),
            workspace_id: workspace_id.to_string(),
            root_id: root_id.to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            kind: ExplorerNodeKind::File,
            depth: 3,
            display_label: "charge.rs".to_string(),
            presentation_uri: nested_logical.clone(),
            canonical_uri: nested_logical.clone(),
            logical_uri: nested_logical,
            root_badge,
            parent_id: Some(nested_dir_id.clone()),
            readiness: NodeReadinessClass::Loaded,
            generated_artifact_hint: None,
            special_file_hint: None,
        })
        .unwrap();

        (tree, root_node_id, nested_dir_id, leaf_id)
    }

    #[test]
    fn reveal_after_filter_keeps_canonical_node_identity() {
        let (mut tree, root_id, _dir_id, leaf_id) = build_tree();

        let filter = apply_filter(&mut tree, Some("orders".to_string()));
        assert_eq!(filter.total_matching_rows, 0);
        assert_eq!(
            filter.total_hidden_rows,
            filter.total_nodes - filter.total_visible_rows
        );

        let outcome = reveal(&mut tree, &leaf_id).expect("reveal must succeed");
        assert_eq!(outcome.revealed_node_id, leaf_id);
        assert_eq!(outcome.selection_node_id, leaf_id);
        assert_eq!(
            outcome
                .ancestry_chain
                .first()
                .expect("ancestry must include the root mount"),
            &root_id,
            "first ancestry segment must be the root mount, not a renamed alias"
        );
        assert!(!outcome.matches_filter);
        assert_eq!(outcome.filter_query.as_deref(), Some("orders"));

        let leaf_segment = outcome
            .breadcrumb_path
            .segments
            .last()
            .expect("breadcrumb must include the revealed node");
        assert_eq!(leaf_segment.node_id, leaf_id);
        assert!(leaf_segment.is_leaf);
    }

    #[test]
    fn apply_filter_then_clear_restores_full_visible_set() {
        let (mut tree, _root_id, _dir_id, leaf_id) = build_tree();
        let _ = reveal(&mut tree, &leaf_id).unwrap();
        let total_nodes = tree.node_count() as u32;

        let filtered = apply_filter(&mut tree, Some("zzzz".to_string()));
        assert_eq!(filtered.total_matching_rows, 0);
        assert!(filtered.selection_node_id.is_some());
        assert!(!filtered.selection_matches_filter);

        let cleared = apply_filter(&mut tree, None);
        assert!(cleared.query.is_none());
        assert_eq!(cleared.total_visible_rows, total_nodes);
        assert!(cleared.selection_matches_filter);
    }

    #[test]
    fn ancestry_chain_returns_empty_for_root_mount() {
        let (tree, root_id, _, _) = build_tree();
        assert!(ancestry_chain(&tree, &root_id).is_empty());
    }

    #[test]
    fn reveal_unknown_node_returns_typed_error() {
        let (mut tree, _root_id, _dir_id, _leaf_id) = build_tree();
        let bogus =
            ExplorerNodeId::from_logical("wksp:filter", "root:filter", "logical://missing");
        let err = reveal(&mut tree, &bogus).expect_err("unknown node must error");
        assert!(matches!(err, ExplorerTreeError::UnknownNode(_)));
    }
}
