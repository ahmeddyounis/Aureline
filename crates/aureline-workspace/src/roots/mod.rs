//! Multi-root workspace model.
//!
//! A workspace can reference more than one filesystem root. The model here
//! holds the stable, opaque references the rest of the IDE consults when it
//! must reason about whether a path or scope belongs to the active workspace.
//! The workset/sparse-slice artifact in [`crate::worksets`] then narrows that
//! membership for search, graph, refactor, and export surfaces.
//!
//! This module deliberately avoids re-deriving the five-layer filesystem
//! identity model. A [`WorkspaceRootRef`] carries a stable opaque id that
//! resolves through the VFS root registry; raw absolute paths never appear on
//! this surface.
//!
//! Source refs:
//! - `docs/adr/0006-vfs-save-cache-identity.md`
//! - `schemas/workspace/workset_artifact.schema.json` (filesystem_root_ref)

use serde::{Deserialize, Serialize};

/// Schema version for the persisted multi-root workspace shape.
pub type MultiRootWorkspaceSchemaVersion = u32;

/// Identifies the `multi_root_workspace_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiRootWorkspaceRecordKind {
    /// `multi_root_workspace_record`
    MultiRootWorkspaceRecord,
}

/// Stable kind for a workspace root, mirrored on the chrome badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRootKind {
    LocalFolder,
    LocalRepoRoot,
    RemoteRepository,
    SshWorkspace,
    ContainerRoot,
    DevcontainerRoot,
    ManagedCloudRoot,
    VirtualDocumentRoot,
    ArchiveRoot,
}

impl WorkspaceRootKind {
    /// Returns the stable string vocabulary for this root kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFolder => "local_folder",
            Self::LocalRepoRoot => "local_repo_root",
            Self::RemoteRepository => "remote_repository",
            Self::SshWorkspace => "ssh_workspace",
            Self::ContainerRoot => "container_root",
            Self::DevcontainerRoot => "devcontainer_root",
            Self::ManagedCloudRoot => "managed_cloud_root",
            Self::VirtualDocumentRoot => "virtual_document_root",
            Self::ArchiveRoot => "archive_root",
        }
    }

    /// Returns the chrome badge for this root kind.
    pub const fn root_badge(self) -> &'static str {
        match self {
            Self::LocalFolder | Self::LocalRepoRoot => "local",
            Self::RemoteRepository | Self::SshWorkspace => "remote",
            Self::ContainerRoot | Self::DevcontainerRoot => "container",
            Self::ManagedCloudRoot => "managed",
            Self::VirtualDocumentRoot => "virtual",
            Self::ArchiveRoot => "archive",
        }
    }
}

/// Re-export of the TAD 12.6 partial-truth labels for a root reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootPartialTruth {
    Loaded,
    ManifestKnown,
    Cached,
    Unavailable,
}

impl RootPartialTruth {
    /// Returns the stable string vocabulary for this label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loaded => "loaded",
            Self::ManifestKnown => "manifest_known",
            Self::Cached => "cached",
            Self::Unavailable => "unavailable",
        }
    }
}

/// One filesystem root attached to a workspace.
///
/// `root_id` is opaque (safe in logs, RPC, manifests, support bundles) and
/// resolves to a VFS root through the filesystem-identity surface. Raw absolute
/// paths never appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRootRef {
    pub root_id: String,
    pub root_kind: WorkspaceRootKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
    pub partial_truth: RootPartialTruth,
}

impl WorkspaceRootRef {
    /// Constructs a fully loaded local-folder root reference.
    pub fn local_folder(root_id: impl Into<String>, presentation_label: impl Into<String>) -> Self {
        Self {
            root_id: root_id.into(),
            root_kind: WorkspaceRootKind::LocalFolder,
            presentation_label: Some(presentation_label.into()),
            partial_truth: RootPartialTruth::Loaded,
        }
    }
}

/// Errors returned while constructing a [`MultiRootWorkspace`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiRootWorkspaceError {
    EmptyRoots,
    DuplicateRootId(String),
    EmptyWorkspaceId,
    EmptyWorkspaceName,
}

impl std::fmt::Display for MultiRootWorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRoots => write!(f, "workspace must declare at least one root"),
            Self::DuplicateRootId(id) => write!(f, "duplicate root_id in workspace: {id}"),
            Self::EmptyWorkspaceId => write!(f, "workspace_id must not be empty"),
            Self::EmptyWorkspaceName => write!(f, "workspace_name must not be empty"),
        }
    }
}

impl std::error::Error for MultiRootWorkspaceError {}

/// One serialized workspace file with one or more roots.
///
/// The same record carries single-root and multi-root workspaces; consumers
/// query [`is_multi_root`](Self::is_multi_root) instead of inventing a parallel
/// "multi-root mode" flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiRootWorkspace {
    pub record_kind: MultiRootWorkspaceRecordKind,
    pub multi_root_workspace_schema_version: MultiRootWorkspaceSchemaVersion,
    pub workspace_id: String,
    pub workspace_name: String,
    pub roots: Vec<WorkspaceRootRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_workset_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub saved_workset_refs: Vec<String>,
}

impl MultiRootWorkspace {
    /// Constructs a workspace from a non-empty, unique-id list of roots.
    pub fn try_new(
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        roots: Vec<WorkspaceRootRef>,
    ) -> Result<Self, MultiRootWorkspaceError> {
        let workspace_id = workspace_id.into();
        if workspace_id.is_empty() {
            return Err(MultiRootWorkspaceError::EmptyWorkspaceId);
        }
        let workspace_name = workspace_name.into();
        if workspace_name.is_empty() {
            return Err(MultiRootWorkspaceError::EmptyWorkspaceName);
        }
        if roots.is_empty() {
            return Err(MultiRootWorkspaceError::EmptyRoots);
        }
        let mut seen: Vec<&str> = Vec::with_capacity(roots.len());
        for root in &roots {
            if seen.iter().any(|id| *id == root.root_id.as_str()) {
                return Err(MultiRootWorkspaceError::DuplicateRootId(
                    root.root_id.clone(),
                ));
            }
            seen.push(root.root_id.as_str());
        }
        Ok(Self {
            record_kind: MultiRootWorkspaceRecordKind::MultiRootWorkspaceRecord,
            multi_root_workspace_schema_version: 1,
            workspace_id,
            workspace_name,
            roots,
            default_workset_ref: None,
            saved_workset_refs: Vec::new(),
        })
    }

    /// Returns the number of roots attached to the workspace.
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    /// Returns true when the workspace exposes more than one root.
    pub fn is_multi_root(&self) -> bool {
        self.roots.len() >= 2
    }

    /// Returns true when the workspace claims `root_id` as a member.
    pub fn contains_root(&self, root_id: &str) -> bool {
        self.roots.iter().any(|r| r.root_id == root_id)
    }

    /// Returns the root reference with `root_id`, if any.
    pub fn root(&self, root_id: &str) -> Option<&WorkspaceRootRef> {
        self.roots.iter().find(|r| r.root_id == root_id)
    }

    /// Records a saved workset reference (e.g., a workspace-shared workset).
    /// Returns `false` when `workset_ref` is already registered.
    pub fn register_saved_workset(&mut self, workset_ref: impl Into<String>) -> bool {
        let workset_ref = workset_ref.into();
        if self.saved_workset_refs.iter().any(|w| *w == workset_ref) {
            return false;
        }
        self.saved_workset_refs.push(workset_ref);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(id: &str, label: &str) -> WorkspaceRootRef {
        WorkspaceRootRef::local_folder(id, label)
    }

    #[test]
    fn try_new_rejects_empty_root_list() {
        let err =
            MultiRootWorkspace::try_new("wksp:test", "test", Vec::new()).expect_err("must reject");
        assert_eq!(err, MultiRootWorkspaceError::EmptyRoots);
    }

    #[test]
    fn try_new_rejects_duplicate_root_ids() {
        let err = MultiRootWorkspace::try_new(
            "wksp:test",
            "test",
            vec![root("r-0", "a"), root("r-0", "b")],
        )
        .expect_err("must reject");
        assert_eq!(err, MultiRootWorkspaceError::DuplicateRootId("r-0".into()));
    }

    #[test]
    fn multi_root_predicates_track_root_count() {
        let single = MultiRootWorkspace::try_new("wksp:single", "s", vec![root("r-0", "a")])
            .expect("single root must construct");
        assert_eq!(single.root_count(), 1);
        assert!(!single.is_multi_root());
        assert!(single.contains_root("r-0"));
        assert!(!single.contains_root("r-1"));

        let multi = MultiRootWorkspace::try_new(
            "wksp:multi",
            "m",
            vec![root("r-0", "a"), root("r-1", "b"), root("r-2", "c")],
        )
        .expect("multi-root must construct");
        assert_eq!(multi.root_count(), 3);
        assert!(multi.is_multi_root());
        assert!(multi.contains_root("r-2"));
    }

    #[test]
    fn register_saved_workset_is_idempotent() {
        let mut ws = MultiRootWorkspace::try_new("wksp:multi", "m", vec![root("r-0", "a")])
            .expect("must construct");
        assert!(ws.register_saved_workset("wks:hot_path"));
        assert!(!ws.register_saved_workset("wks:hot_path"));
        assert_eq!(ws.saved_workset_refs.len(), 1);
    }

    #[test]
    fn round_trips_through_json() {
        let ws = MultiRootWorkspace::try_new(
            "wksp:multi",
            "Payments monorepo",
            vec![
                root("r-api", "payments-api"),
                root("r-shared", "payments-shared"),
            ],
        )
        .expect("must construct");

        let payload = serde_json::to_string(&ws).expect("must serialize");
        let parsed: MultiRootWorkspace = serde_json::from_str(&payload).expect("must parse");
        assert_eq!(parsed, ws);
    }
}
