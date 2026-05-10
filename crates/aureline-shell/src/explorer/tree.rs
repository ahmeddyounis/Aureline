//! Virtualized file-tree state.
//!
//! [`ExplorerTree`] is the canonical truth source for explorer node identity,
//! expansion state, selection, and the active filter. Virtualization is
//! provided by [`ExplorerTree::viewport`], which materializes a stable slice
//! of node ids for the requested row range. **Row positions are derived; node
//! identity is canonical.** Selection and expansion never depend on a row
//! remaining mounted in memory, so filtering, expansion churn, restore, and
//! support export all keep working over the same handles.
//!
//! The tree intentionally does not stream filesystem events; it consumes
//! pre-built nodes from upstream VFS roots (M01-057) and projects them into
//! the explorer surface.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::node::{ExplorerNode, ExplorerNodeId, ExplorerNodeKind, NodeReadinessClass};

/// Errors returned while inserting nodes or applying filter / expansion edits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorerTreeError {
    DuplicateNode(ExplorerNodeId),
    UnknownNode(ExplorerNodeId),
    UnknownParent(ExplorerNodeId),
    DepthMismatch {
        node: ExplorerNodeId,
        declared: u32,
        expected: u32,
    },
    LeafCannotHaveChildren(ExplorerNodeId),
}

impl std::fmt::Display for ExplorerTreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateNode(id) => write!(f, "duplicate explorer node id: {id}"),
            Self::UnknownNode(id) => write!(f, "unknown explorer node id: {id}"),
            Self::UnknownParent(id) => write!(f, "unknown parent for explorer node id: {id}"),
            Self::DepthMismatch {
                node,
                declared,
                expected,
            } => write!(
                f,
                "depth mismatch for {node}: declared={declared}, expected={expected}"
            ),
            Self::LeafCannotHaveChildren(id) => {
                write!(f, "node kind for {id} cannot have children")
            }
        }
    }
}

impl std::error::Error for ExplorerTreeError {}

/// One row in a materialized viewport.
///
/// `position_index` is a transient render hint. **Identity is `node_id`.**
/// Callers that persist selection or restore state must keep `node_id` and
/// re-resolve to a `position_index` on the next viewport materialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerViewportRow {
    pub node_id: ExplorerNodeId,
    pub position_index: u32,
    pub depth: u32,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub matches_filter: bool,
    pub kind: ExplorerNodeKind,
    pub readiness: NodeReadinessClass,
    pub display_label: String,
}

/// A materialized slice of the visible row set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerViewport {
    pub total_visible_rows: u32,
    pub start_index: u32,
    pub rows: Vec<ExplorerViewportRow>,
    pub filter_query: Option<String>,
    pub selection_node_id: Option<ExplorerNodeId>,
    pub selection_in_viewport: bool,
}

/// Active filter state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerFilter {
    pub query: Option<String>,
}

impl ExplorerFilter {
    fn matches(&self, label: &str) -> bool {
        match &self.query {
            None => true,
            Some(q) if q.is_empty() => true,
            Some(q) => label.to_ascii_lowercase().contains(&q.to_ascii_lowercase()),
        }
    }
}

/// Virtualized explorer tree state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerTree {
    nodes: BTreeMap<ExplorerNodeId, ExplorerNode>,
    children: BTreeMap<ExplorerNodeId, Vec<ExplorerNodeId>>,
    insertion_order: Vec<ExplorerNodeId>,
    root_ids: Vec<ExplorerNodeId>,
    expansion: BTreeSet<ExplorerNodeId>,
    selection: Option<ExplorerNodeId>,
    filter: ExplorerFilter,
}

impl ExplorerTree {
    /// Empty tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a single node, validating parent linkage and depth.
    pub fn insert(&mut self, node: ExplorerNode) -> Result<(), ExplorerTreeError> {
        if self.nodes.contains_key(&node.node_id) {
            return Err(ExplorerTreeError::DuplicateNode(node.node_id.clone()));
        }
        match &node.parent_id {
            Some(parent_id) => {
                let parent = self
                    .nodes
                    .get(parent_id)
                    .ok_or_else(|| ExplorerTreeError::UnknownParent(node.node_id.clone()))?;
                if !parent.kind.may_have_children() {
                    return Err(ExplorerTreeError::LeafCannotHaveChildren(parent_id.clone()));
                }
                let expected = parent.depth + 1;
                if node.depth != expected {
                    return Err(ExplorerTreeError::DepthMismatch {
                        node: node.node_id.clone(),
                        declared: node.depth,
                        expected,
                    });
                }
            }
            None => {
                if node.depth != 0 {
                    return Err(ExplorerTreeError::DepthMismatch {
                        node: node.node_id.clone(),
                        declared: node.depth,
                        expected: 0,
                    });
                }
            }
        }

        let id = node.node_id.clone();
        if let Some(parent_id) = node.parent_id.clone() {
            self.children.entry(parent_id).or_default().push(id.clone());
        } else {
            self.root_ids.push(id.clone());
        }
        self.insertion_order.push(id.clone());
        self.nodes.insert(id, node);
        Ok(())
    }

    /// Insert many nodes in one call. Order matters: each node's parent must
    /// have already been inserted (or appear earlier in `nodes`).
    pub fn insert_many<I>(&mut self, nodes: I) -> Result<(), ExplorerTreeError>
    where
        I: IntoIterator<Item = ExplorerNode>,
    {
        for node in nodes {
            self.insert(node)?;
        }
        Ok(())
    }

    /// Returns the node with the given id, if present.
    pub fn node(&self, id: &ExplorerNodeId) -> Option<&ExplorerNode> {
        self.nodes.get(id)
    }

    /// Returns the children of the given parent in insertion order.
    pub fn children_of(&self, id: &ExplorerNodeId) -> &[ExplorerNodeId] {
        self.children.get(id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Returns the root mount node ids.
    pub fn root_ids(&self) -> &[ExplorerNodeId] {
        &self.root_ids
    }

    /// Total number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// True when the node id is currently expanded.
    pub fn is_expanded(&self, id: &ExplorerNodeId) -> bool {
        self.expansion.contains(id)
    }

    /// Currently selected node id.
    pub fn selected(&self) -> Option<&ExplorerNodeId> {
        self.selection.as_ref()
    }

    /// Active filter.
    pub fn filter(&self) -> &ExplorerFilter {
        &self.filter
    }

    /// Expand the node. Root mounts are always treated as expanded but may
    /// also be added to the expansion set explicitly.
    pub fn expand(&mut self, id: &ExplorerNodeId) -> Result<(), ExplorerTreeError> {
        let node = self
            .nodes
            .get(id)
            .ok_or_else(|| ExplorerTreeError::UnknownNode(id.clone()))?;
        if node.kind.may_have_children() {
            self.expansion.insert(id.clone());
        }
        Ok(())
    }

    /// Collapse the node, but never collapse a persistent root mount.
    pub fn collapse(&mut self, id: &ExplorerNodeId) -> Result<(), ExplorerTreeError> {
        let node = self
            .nodes
            .get(id)
            .ok_or_else(|| ExplorerTreeError::UnknownNode(id.clone()))?;
        if node.is_persistent_mount() {
            return Ok(());
        }
        self.expansion.remove(id);
        Ok(())
    }

    /// Toggle expansion. Returns the new state.
    pub fn toggle(&mut self, id: &ExplorerNodeId) -> Result<bool, ExplorerTreeError> {
        if self.is_expanded(id) {
            self.collapse(id)?;
            Ok(false)
        } else {
            self.expand(id)?;
            Ok(true)
        }
    }

    /// Persist a selection. Selection survives filtering even when the row is
    /// hidden from the current viewport.
    pub fn select(&mut self, id: &ExplorerNodeId) -> Result<(), ExplorerTreeError> {
        if !self.nodes.contains_key(id) {
            return Err(ExplorerTreeError::UnknownNode(id.clone()));
        }
        self.selection = Some(id.clone());
        Ok(())
    }

    /// Clear the active selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Apply a new filter. The active selection is not cleared even when the
    /// selected row is filtered out of the viewport.
    pub fn set_filter(&mut self, query: Option<String>) {
        self.filter = ExplorerFilter { query };
    }

    /// Reveal the path to a node by expanding all ancestors. Selection is
    /// updated to the revealed node.
    pub fn reveal(
        &mut self,
        id: &ExplorerNodeId,
    ) -> Result<Vec<ExplorerNodeId>, ExplorerTreeError> {
        if !self.nodes.contains_key(id) {
            return Err(ExplorerTreeError::UnknownNode(id.clone()));
        }
        let mut chain: Vec<ExplorerNodeId> = Vec::new();
        let mut current = self.nodes.get(id).and_then(|n| n.parent_id.clone());
        while let Some(parent) = current {
            chain.push(parent.clone());
            current = self.nodes.get(&parent).and_then(|n| n.parent_id.clone());
        }
        chain.reverse();
        for ancestor in &chain {
            self.expand(ancestor)?;
        }
        self.select(id)?;
        Ok(chain)
    }

    /// Refresh the readiness class of an existing node.
    pub fn set_readiness(
        &mut self,
        id: &ExplorerNodeId,
        readiness: NodeReadinessClass,
    ) -> Result<(), ExplorerTreeError> {
        let node = self
            .nodes
            .get_mut(id)
            .ok_or_else(|| ExplorerTreeError::UnknownNode(id.clone()))?;
        node.readiness = readiness;
        Ok(())
    }

    /// Remove a node and all its descendants. Selection and expansion entries
    /// referencing removed nodes are cleared.
    pub fn remove_subtree(&mut self, id: &ExplorerNodeId) -> Result<(), ExplorerTreeError> {
        if !self.nodes.contains_key(id) {
            return Err(ExplorerTreeError::UnknownNode(id.clone()));
        }
        let mut to_remove: Vec<ExplorerNodeId> = Vec::new();
        let mut stack: Vec<ExplorerNodeId> = vec![id.clone()];
        while let Some(node) = stack.pop() {
            if let Some(children) = self.children.get(&node) {
                stack.extend(children.iter().cloned());
            }
            to_remove.push(node);
        }

        let parent_id = self.nodes.get(id).and_then(|n| n.parent_id.clone());
        if let Some(parent_id) = parent_id {
            if let Some(siblings) = self.children.get_mut(&parent_id) {
                siblings.retain(|child| child != id);
            }
        } else {
            self.root_ids.retain(|root| root != id);
        }

        for node_id in &to_remove {
            self.nodes.remove(node_id);
            self.children.remove(node_id);
            self.expansion.remove(node_id);
        }
        self.insertion_order
            .retain(|id| self.nodes.contains_key(id));
        if let Some(selected) = self.selection.clone() {
            if !self.nodes.contains_key(&selected) {
                self.selection = None;
            }
        }
        Ok(())
    }

    /// Materialize all currently visible rows in document order, honoring
    /// expansion and filter. Filter matches are tracked per row but the row
    /// is included whenever any descendant matches so the filtered view does
    /// not orphan deep matches.
    pub fn visible_rows(&self) -> Vec<ExplorerViewportRow> {
        let filter_active = self.filter.query.is_some();
        let mut rows: Vec<ExplorerViewportRow> = Vec::with_capacity(self.nodes.len());
        for root_id in &self.root_ids {
            self.collect_visible(root_id, filter_active, &mut rows);
        }
        for (idx, row) in rows.iter_mut().enumerate() {
            row.position_index = idx as u32;
        }
        rows
    }

    fn collect_visible(
        &self,
        id: &ExplorerNodeId,
        filter_active: bool,
        rows: &mut Vec<ExplorerViewportRow>,
    ) {
        let Some(node) = self.nodes.get(id) else {
            return;
        };
        let label_matches = self.filter.matches(&node.display_label);
        let is_root_mount = node.is_persistent_mount();
        let is_expanded = is_root_mount || self.expansion.contains(id);

        let mut child_rows: Vec<ExplorerViewportRow> = Vec::new();
        if is_expanded {
            for child_id in self.children_of(id) {
                self.collect_visible(child_id, filter_active, &mut child_rows);
            }
        }

        let descendant_matched = !child_rows.is_empty();
        let include_self = if filter_active {
            label_matches || descendant_matched || is_root_mount
        } else {
            true
        };

        if !include_self {
            return;
        }

        rows.push(ExplorerViewportRow {
            node_id: id.clone(),
            position_index: 0,
            depth: node.depth,
            is_expanded,
            is_selected: self.selection.as_ref() == Some(id),
            matches_filter: label_matches,
            kind: node.kind,
            readiness: node.readiness,
            display_label: node.display_label.clone(),
        });
        rows.extend(child_rows);
    }

    /// Materialize a virtualized viewport slice.
    ///
    /// `start` and `len` are interpreted against the visible row set. The
    /// returned [`ExplorerViewport`] reports the total visible count so the
    /// chrome can render scroll affordances honestly. `node_id` survives even
    /// when the row falls outside the slice.
    pub fn viewport(&self, start: u32, len: u32) -> ExplorerViewport {
        let visible = self.visible_rows();
        let total = visible.len() as u32;
        let start_clamped = start.min(total);
        let end = start_clamped.saturating_add(len).min(total);
        let rows: Vec<ExplorerViewportRow> = visible[start_clamped as usize..end as usize].to_vec();
        let selection_in_viewport = match (&self.selection, &rows) {
            (Some(sel), rows) => rows.iter().any(|r| r.node_id == *sel),
            _ => false,
        };
        ExplorerViewport {
            total_visible_rows: total,
            start_index: start_clamped,
            rows,
            filter_query: self.filter.query.clone(),
            selection_node_id: self.selection.clone(),
            selection_in_viewport,
        }
    }

    /// True when the node id is currently visible (in any viewport slice)
    /// under the active filter and expansion state.
    pub fn is_visible(&self, id: &ExplorerNodeId) -> bool {
        self.visible_rows().iter().any(|row| row.node_id == *id)
    }
}
