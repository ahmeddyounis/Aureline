//! Explorer action vocabulary.
//!
//! Every action the explorer surface offers — open, reveal, refresh, basic
//! create / remove placeholder, expand / collapse, select, and filter — is
//! dispatched through this module, and every dispatch produces a typed
//! [`ExplorerActionRecord`]. The record carries the canonical command id so
//! the live shell, the command palette, and downstream support exports all
//! reuse the same vocabulary instead of inventing per-surface action labels.
//!
//! The module is intentionally a record + dispatcher; it does not own the
//! tree state or perform IO. The dispatcher mutates the [`ExplorerTree`]
//! truth source for selection, expansion, filter, and placeholder-row
//! lifecycle, and reports an outcome class that the chrome can render
//! without re-deriving state.

use serde::{Deserialize, Serialize};

use super::node::{
    ExplorerNode, ExplorerNodeId, ExplorerNodeKind, NodeReadinessClass, SpecialFileHint,
};
use super::tree::{ExplorerTree, ExplorerTreeError};

/// Closed vocabulary for explorer actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplorerActionClass {
    /// Open the node in the editor (or the appropriate inspector).
    Open,
    /// Reveal the node in the explorer (expand all ancestors and select it).
    Reveal,
    /// Refresh the node's readiness state from upstream truth.
    Refresh,
    /// Create a placeholder row under the parent. The row is **not yet**
    /// committed to disk; the editor / save pipeline owns commit.
    CreatePlaceholder,
    /// Remove a placeholder row that was never committed to disk.
    RemovePlaceholder,
    /// Expand a directory or root mount node.
    Expand,
    /// Collapse a directory or root mount node.
    Collapse,
    /// Select a node (does not change expansion or scroll state).
    Select,
    /// Apply a new filter query.
    SetFilter,
}

impl ExplorerActionClass {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Reveal => "reveal",
            Self::Refresh => "refresh",
            Self::CreatePlaceholder => "create_placeholder",
            Self::RemovePlaceholder => "remove_placeholder",
            Self::Expand => "expand",
            Self::Collapse => "collapse",
            Self::Select => "select",
            Self::SetFilter => "set_filter",
        }
    }

    /// Canonical command id for this action class. Reuses the same
    /// `cmd:workspace.*` namespace the rest of the shell exposes.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Open => "cmd:workspace.explorer_open_node",
            Self::Reveal => "cmd:workspace.explorer_reveal_node",
            Self::Refresh => "cmd:workspace.explorer_refresh_node",
            Self::CreatePlaceholder => "cmd:workspace.explorer_create_placeholder",
            Self::RemovePlaceholder => "cmd:workspace.explorer_remove_placeholder",
            Self::Expand => "cmd:workspace.explorer_expand_node",
            Self::Collapse => "cmd:workspace.explorer_collapse_node",
            Self::Select => "cmd:workspace.explorer_select_node",
            Self::SetFilter => "cmd:workspace.explorer_set_filter",
        }
    }
}

/// Outcome class returned by [`dispatch`] without leaking error detail to the
/// chrome. The dispatcher's `Result::Err` carries the typed error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplorerActionOutcome {
    /// Action executed and tree state was mutated.
    Applied,
    /// Action executed; tree state was not changed (e.g., expand on a leaf).
    NoOp,
    /// Action could not run because the named node was not found or invalid.
    Rejected,
}

impl ExplorerActionOutcome {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::NoOp => "no_op",
            Self::Rejected => "rejected",
        }
    }
}

/// Typed explorer action request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ExplorerAction {
    Open {
        node_id: ExplorerNodeId,
    },
    Reveal {
        node_id: ExplorerNodeId,
    },
    Refresh {
        node_id: ExplorerNodeId,
        readiness: NodeReadinessClass,
    },
    CreatePlaceholder {
        parent_id: ExplorerNodeId,
        display_label: String,
        kind: ExplorerNodeKind,
    },
    RemovePlaceholder {
        node_id: ExplorerNodeId,
    },
    Expand {
        node_id: ExplorerNodeId,
    },
    Collapse {
        node_id: ExplorerNodeId,
    },
    Select {
        node_id: ExplorerNodeId,
    },
    SetFilter {
        query: Option<String>,
    },
}

impl ExplorerAction {
    /// Returns the action class.
    pub fn class(&self) -> ExplorerActionClass {
        match self {
            Self::Open { .. } => ExplorerActionClass::Open,
            Self::Reveal { .. } => ExplorerActionClass::Reveal,
            Self::Refresh { .. } => ExplorerActionClass::Refresh,
            Self::CreatePlaceholder { .. } => ExplorerActionClass::CreatePlaceholder,
            Self::RemovePlaceholder { .. } => ExplorerActionClass::RemovePlaceholder,
            Self::Expand { .. } => ExplorerActionClass::Expand,
            Self::Collapse { .. } => ExplorerActionClass::Collapse,
            Self::Select { .. } => ExplorerActionClass::Select,
            Self::SetFilter { .. } => ExplorerActionClass::SetFilter,
        }
    }
}

/// Dispatch result record. Carries the action class, command id, and outcome
/// along with any node ids the action touched. Designed to be persisted in
/// support exports and replayable against fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerActionRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub action_class: String,
    pub command_id: String,
    pub outcome: String,
    pub primary_node_id: Option<ExplorerNodeId>,
    pub created_node_id: Option<ExplorerNodeId>,
    pub revealed_chain: Vec<ExplorerNodeId>,
    pub filter_query: Option<String>,
    pub selection_node_id: Option<ExplorerNodeId>,
    pub explainer: String,
}

const EXPLORER_ACTION_SCHEMA_VERSION: u32 = 1;

/// Dispatch an action against the tree. The tree is mutated in place; the
/// returned record describes what happened and the resulting selection.
pub fn dispatch(
    tree: &mut ExplorerTree,
    action: &ExplorerAction,
) -> Result<ExplorerActionRecord, ExplorerTreeError> {
    let class = action.class();
    let command_id = class.command_id().to_string();

    let mut record = ExplorerActionRecord {
        record_kind: "explorer_action_record".to_string(),
        schema_version: EXPLORER_ACTION_SCHEMA_VERSION,
        action_class: class.as_str().to_string(),
        command_id,
        outcome: ExplorerActionOutcome::Applied.as_str().to_string(),
        primary_node_id: None,
        created_node_id: None,
        revealed_chain: Vec::new(),
        filter_query: tree.filter().query.clone(),
        selection_node_id: tree.selected().cloned(),
        explainer: String::new(),
    };

    match action {
        ExplorerAction::Open { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            let (label, kind) = {
                let node = tree
                    .node(node_id)
                    .ok_or_else(|| ExplorerTreeError::UnknownNode(node_id.clone()))?;
                (node.display_label.clone(), node.kind)
            };
            if !kind.opens_in_editor() {
                record.outcome = ExplorerActionOutcome::NoOp.as_str().to_string();
                record.explainer = format!(
                    "{} is a {} and does not open in the editor",
                    label,
                    kind.as_str()
                );
            } else {
                tree.select(node_id)?;
                record.selection_node_id = Some(node_id.clone());
                record.explainer = format!("opened {} ({})", label, kind.as_str());
            }
        }
        ExplorerAction::Reveal { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            let chain = tree.reveal(node_id)?;
            record.revealed_chain = chain;
            record.selection_node_id = Some(node_id.clone());
            record.explainer = format!("revealed {}", node_id);
        }
        ExplorerAction::Refresh { node_id, readiness } => {
            record.primary_node_id = Some(node_id.clone());
            tree.set_readiness(node_id, *readiness)?;
            record.explainer = format!(
                "refreshed readiness of {} -> {}",
                node_id,
                readiness.as_str()
            );
        }
        ExplorerAction::CreatePlaceholder {
            parent_id,
            display_label,
            kind,
        } => {
            record.primary_node_id = Some(parent_id.clone());
            let parent = tree
                .node(parent_id)
                .ok_or_else(|| ExplorerTreeError::UnknownNode(parent_id.clone()))?;
            if !parent.kind.may_have_children() {
                record.outcome = ExplorerActionOutcome::Rejected.as_str().to_string();
                record.explainer = format!(
                    "cannot create child under {}: parent kind {} is a leaf",
                    parent.display_label,
                    parent.kind.as_str()
                );
                return Ok(record);
            }
            let workspace_id = parent.workspace_id.clone();
            let root_id = parent.root_id.clone();
            let root_kind = parent.root_kind;
            let root_badge = parent.root_badge.clone();
            let depth = parent.depth + 1;
            let parent_logical = parent.logical_uri.clone();
            let parent_canonical = parent.canonical_uri.clone();
            let parent_presentation = parent.presentation_uri.clone();
            let logical_uri = join_logical(&parent_logical, display_label);
            let canonical_uri = join_logical(&parent_canonical, display_label);
            let presentation_uri = join_logical(&parent_presentation, display_label);
            let placeholder = ExplorerNode {
                node_id: ExplorerNodeId::from_logical(&workspace_id, &root_id, &logical_uri),
                workspace_id,
                root_id,
                root_kind,
                kind: *kind,
                depth,
                display_label: display_label.clone(),
                presentation_uri,
                canonical_uri,
                logical_uri,
                root_badge,
                parent_id: Some(parent_id.clone()),
                readiness: NodeReadinessClass::ManifestKnown,
                generated_artifact_hint: None,
                special_file_hint: Some(SpecialFileHint {
                    class: "placeholder_uncommitted".to_string(),
                    explainer: "Placeholder row; not yet committed to disk".to_string(),
                }),
            };
            let new_id = placeholder.node_id.clone();
            tree.insert(placeholder)?;
            tree.expand(parent_id)?;
            tree.select(&new_id)?;
            record.created_node_id = Some(new_id.clone());
            record.selection_node_id = Some(new_id);
            record.explainer = format!("created placeholder {} under {}", display_label, parent_id);
        }
        ExplorerAction::RemovePlaceholder { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            let node = tree
                .node(node_id)
                .ok_or_else(|| ExplorerTreeError::UnknownNode(node_id.clone()))?;
            let is_placeholder = node
                .special_file_hint
                .as_ref()
                .map(|hint| hint.class == "placeholder_uncommitted")
                .unwrap_or(false);
            if !is_placeholder {
                record.outcome = ExplorerActionOutcome::Rejected.as_str().to_string();
                record.explainer = format!(
                    "{} is not a placeholder row; remove must go through save pipeline",
                    node.display_label
                );
                return Ok(record);
            }
            tree.remove_subtree(node_id)?;
            record.selection_node_id = tree.selected().cloned();
            record.explainer = format!("removed placeholder {}", node_id);
        }
        ExplorerAction::Expand { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            let (label, kind) = {
                let node = tree
                    .node(node_id)
                    .ok_or_else(|| ExplorerTreeError::UnknownNode(node_id.clone()))?;
                (node.display_label.clone(), node.kind)
            };
            if !kind.may_have_children() {
                record.outcome = ExplorerActionOutcome::NoOp.as_str().to_string();
                record.explainer = format!("{} is a {} and cannot expand", label, kind.as_str());
            } else {
                let was_expanded = tree.is_expanded(node_id);
                tree.expand(node_id)?;
                if was_expanded {
                    record.outcome = ExplorerActionOutcome::NoOp.as_str().to_string();
                    record.explainer = format!("{} already expanded", label);
                } else {
                    record.explainer = format!("expanded {}", label);
                }
            }
        }
        ExplorerAction::Collapse { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            let (label, is_root) = {
                let node = tree
                    .node(node_id)
                    .ok_or_else(|| ExplorerTreeError::UnknownNode(node_id.clone()))?;
                (node.display_label.clone(), node.is_persistent_mount())
            };
            if is_root {
                record.outcome = ExplorerActionOutcome::NoOp.as_str().to_string();
                record.explainer =
                    format!("{} is a persistent root mount and cannot collapse", label);
            } else {
                let was_expanded = tree.is_expanded(node_id);
                tree.collapse(node_id)?;
                if was_expanded {
                    record.explainer = format!("collapsed {}", label);
                } else {
                    record.outcome = ExplorerActionOutcome::NoOp.as_str().to_string();
                    record.explainer = format!("{} already collapsed", label);
                }
            }
        }
        ExplorerAction::Select { node_id } => {
            record.primary_node_id = Some(node_id.clone());
            tree.select(node_id)?;
            record.selection_node_id = Some(node_id.clone());
            record.explainer = format!("selected {}", node_id);
        }
        ExplorerAction::SetFilter { query } => {
            tree.set_filter(query.clone());
            record.filter_query = query.clone();
            record.explainer = match query {
                Some(q) if !q.is_empty() => format!("filter query set to {q:?}"),
                _ => "filter cleared".to_string(),
            };
        }
    }

    if record.selection_node_id.is_none() {
        record.selection_node_id = tree.selected().cloned();
    }
    Ok(record)
}

fn join_logical(parent: &str, leaf: &str) -> String {
    let leaf = leaf.trim_start_matches('/');
    if parent.ends_with('/') {
        format!("{parent}{leaf}")
    } else {
        format!("{parent}/{leaf}")
    }
}
