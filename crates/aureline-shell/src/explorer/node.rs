//! Explorer node identity, kinds, and visible hints.
//!
//! The explorer reasons about nodes by **stable identity**, not by transient
//! row position. A [`ExplorerNodeId`] is derived from the canonical
//! `(workspace_id, root_id, logical_uri)` tuple at construction time, which
//! means filtering, virtualization, expansion churn, restore, and support
//! export all refer to the same handle. Nodes survive being unmounted and
//! re-mounted as long as their underlying logical identity is unchanged.
//!
//! The hint vocabulary in [`GeneratedArtifactHint`] / [`SpecialFileHint`] is
//! intentionally narrow: it only carries enough state for the explorer chrome
//! to label a row distinctly from a hand-authored canonical source. Producing
//! the hints themselves is the job of upstream VFS roots and workset truth;
//! this surface only projects them.

use serde::{Deserialize, Serialize};

use aureline_vfs::{IdentityRecord, VfsUri};
use aureline_workspace::{
    detect_lineage, GeneratedArtifactClass, LineageHintRecord, RootPartialTruth, WorkspaceRootKind,
};

/// Stable identifier for a single explorer node.
///
/// The id is derived from the canonical `(workspace_id, root_id, logical_uri)`
/// tuple, so the same logical object always produces the same id even after
/// virtualization, filtering, or rebuild. The id is opaque on serialization
/// boundaries: callers must not parse it to recover paths.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ExplorerNodeId(String);

impl ExplorerNodeId {
    /// Construct a node id from the underlying logical identity.
    pub fn from_logical(workspace_id: &str, root_id: &str, logical_uri: &str) -> Self {
        Self(format!("node:{workspace_id}|{root_id}|{logical_uri}"))
    }

    /// Construct a node id directly from an [`IdentityRecord`].
    pub fn from_identity(record: &IdentityRecord) -> Self {
        Self::from_logical(
            &record.logical_workspace_identity.workspace_id,
            &record.logical_workspace_identity.root_id,
            record.logical_workspace_identity.logical_uri.as_str(),
        )
    }

    /// Stable string form. Safe to log and to ship in support bundles.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consume into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for ExplorerNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Closed vocabulary for explorer node kinds.
///
/// The kind controls default action enablement (e.g., `open` is offered for
/// `File` / `GeneratedArtifact` / `VirtualDocument`, but not `RootMount` or
/// `Directory`). The hint vocabulary in [`GeneratedArtifactHint`] /
/// [`SpecialFileHint`] adds one more layer of provenance disclosure on top of
/// the kind without forking the kind enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplorerNodeKind {
    /// Top-level workspace root mount. One per attached `WorkspaceRootRef`.
    RootMount,
    /// Directory under a root mount or another directory.
    Directory,
    /// Hand-authored canonical file.
    File,
    /// Document materialized by a producer (e.g., compiler output).
    GeneratedArtifact,
    /// In-memory or session-scoped virtual document.
    VirtualDocument,
    /// Hand-authored file flagged as special by the workspace (ignore-listed,
    /// hidden, or otherwise excluded from default surfaces).
    SpecialFile,
}

impl ExplorerNodeKind {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootMount => "root_mount",
            Self::Directory => "directory",
            Self::File => "file",
            Self::GeneratedArtifact => "generated_artifact",
            Self::VirtualDocument => "virtual_document",
            Self::SpecialFile => "special_file",
        }
    }

    /// True when an `open` action is offered by default for this kind.
    pub const fn opens_in_editor(self) -> bool {
        matches!(
            self,
            Self::File | Self::GeneratedArtifact | Self::VirtualDocument | Self::SpecialFile
        )
    }

    /// True when the node may have children.
    pub const fn may_have_children(self) -> bool {
        matches!(self, Self::RootMount | Self::Directory)
    }
}

/// Generated-artifact provenance hint.
///
/// The producer is named so the explorer chrome and downstream cues can
/// disclose lineage without re-deriving truth in every surface. Reuses the
/// docs/ux vocabulary for generated artifacts; see TAD §3938 and
/// `Aureline_UI_UX_Spec_Document.md` §6701.
///
/// The hint is the canonical projection of an
/// [`aureline_workspace::LineageHintRecord`] into explorer chrome. The
/// [`Self::from_lineage_record`] constructor maps the upstream record onto
/// the chrome-friendly fields without re-deriving lineage truth in this
/// surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactHint {
    pub producer_id: String,
    pub producer_label: String,
    pub generated_from_uri: Option<String>,
    pub freshness_class: String,
    /// Stable token from [`GeneratedArtifactClass::as_str`]; `None` when the
    /// hint was authored without a typed class (legacy fixtures).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_class: Option<String>,
    /// Short explainer suitable for tooltips and a11y exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_explainer: Option<String>,
    /// Stable rule identifier that produced the hint, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl GeneratedArtifactHint {
    /// Project a hint from the canonical lineage record. The optional
    /// `(workspace_id, root_id)` pair is used to mint a workspace logical URI
    /// for the source-canonical artifact; pass `None` when the consumer just
    /// needs the producer label and lineage class without a navigable URI.
    pub fn from_lineage_record(
        record: &LineageHintRecord,
        workspace_id: Option<&str>,
        root_id: Option<&str>,
    ) -> Self {
        let generated_from_uri = match (
            record.source_canonical_relative_path.as_deref(),
            workspace_id,
            root_id,
        ) {
            (Some(rel), Some(wid), Some(rid)) => {
                Some(format!("aureline-ws://{wid}/{rid}/{rel}"))
            }
            (Some(rel), _, _) => Some(rel.to_string()),
            _ => None,
        };
        Self {
            producer_id: record.producer_id.clone(),
            producer_label: record.producer_label.clone(),
            generated_from_uri,
            freshness_class: record.freshness_class.as_str().to_string(),
            generated_class: Some(record.generated_class.as_str().to_string()),
            lineage_explainer: Some(record.explainer.clone()),
            rule_id: Some(record.rule_id.clone()),
        }
    }

    /// Convenience: detect lineage for `relative_path` using the default
    /// catalog and project it into a hint, returning `None` when no rule
    /// matches.
    pub fn detect_for(
        relative_path: &str,
        workspace_id: Option<&str>,
        root_id: Option<&str>,
    ) -> Option<Self> {
        detect_lineage(relative_path)
            .map(|record| Self::from_lineage_record(&record, workspace_id, root_id))
    }

    /// True when the hint identifies a source-canonical pointer the user can
    /// pivot to.
    pub fn has_source_canonical(&self) -> bool {
        self.generated_from_uri.is_some()
    }

    /// Typed lineage class, when the hint carries one.
    pub fn lineage_class(&self) -> Option<GeneratedArtifactClass> {
        match self.generated_class.as_deref()? {
            "lockfile" => Some(GeneratedArtifactClass::Lockfile),
            "build_output" => Some(GeneratedArtifactClass::BuildOutput),
            "generated_source_sibling" => Some(GeneratedArtifactClass::GeneratedSourceSibling),
            "vendored_snapshot" => Some(GeneratedArtifactClass::VendoredSnapshot),
            _ => None,
        }
    }
}

/// Special-file hint: why this file is excluded or annotated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialFileHint {
    pub class: String,
    pub explainer: String,
}

/// Per-node readiness class.
///
/// Mirrors [`RootPartialTruth`] for root mounts and adds two additional
/// labels for directory enumeration so the explorer can be honest about
/// "we know this directory exists but have not enumerated it yet" without
/// inventing per-row euphemisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeReadinessClass {
    /// Children are fully enumerated for this node.
    Loaded,
    /// Children are partially enumerated; more rows may stream in.
    PartiallyEnumerated,
    /// We know the node exists but have not enumerated children yet.
    ManifestKnown,
    /// Snapshot is from a cached store; not yet refreshed against truth.
    Cached,
    /// Root is unavailable; we cannot enumerate children right now.
    Unavailable,
}

impl NodeReadinessClass {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loaded => "loaded",
            Self::PartiallyEnumerated => "partially_enumerated",
            Self::ManifestKnown => "manifest_known",
            Self::Cached => "cached",
            Self::Unavailable => "unavailable",
        }
    }

    /// Map a [`RootPartialTruth`] from the workspace surface to a readiness
    /// class for a root mount node.
    pub const fn from_root_partial_truth(label: RootPartialTruth) -> Self {
        match label {
            RootPartialTruth::Loaded => Self::Loaded,
            RootPartialTruth::ManifestKnown => Self::ManifestKnown,
            RootPartialTruth::Cached => Self::Cached,
            RootPartialTruth::Unavailable => Self::Unavailable,
        }
    }

    /// True when downstream surfaces should continue to display the row but
    /// label its children as not-yet-trustworthy.
    pub const fn is_partial(self) -> bool {
        matches!(
            self,
            Self::PartiallyEnumerated
                | Self::ManifestKnown
                | Self::Cached
                | Self::Unavailable
        )
    }
}

/// Canonical explorer node record.
///
/// Carries enough state for the explorer chrome to render a row, decide
/// whether it has children, project a path-truth chip, and restore selection
/// after virtualization churn — without re-deriving identity on each frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerNode {
    pub node_id: ExplorerNodeId,
    pub workspace_id: String,
    pub root_id: String,
    pub root_kind: WorkspaceRootKind,
    pub kind: ExplorerNodeKind,
    pub depth: u32,
    pub display_label: String,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub root_badge: String,
    pub parent_id: Option<ExplorerNodeId>,
    pub readiness: NodeReadinessClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_artifact_hint: Option<GeneratedArtifactHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_file_hint: Option<SpecialFileHint>,
}

impl ExplorerNode {
    /// Construct a root-mount node from workspace truth.
    pub fn root_mount(
        workspace_id: impl Into<String>,
        root_id: impl Into<String>,
        root_kind: WorkspaceRootKind,
        display_label: impl Into<String>,
        readiness: NodeReadinessClass,
    ) -> Self {
        let workspace_id: String = workspace_id.into();
        let root_id: String = root_id.into();
        let logical_uri = format!("aureline-ws://{workspace_id}/{root_id}/");
        let presentation_uri = format!("workspace-root://{workspace_id}/{root_id}");
        Self {
            node_id: ExplorerNodeId::from_logical(&workspace_id, &root_id, &logical_uri),
            workspace_id,
            root_id,
            root_kind,
            kind: ExplorerNodeKind::RootMount,
            depth: 0,
            display_label: display_label.into(),
            presentation_uri,
            canonical_uri: logical_uri.clone(),
            logical_uri,
            root_badge: root_kind.root_badge().to_string(),
            parent_id: None,
            readiness,
            generated_artifact_hint: None,
            special_file_hint: None,
        }
    }

    /// True when the node should not collapse its expansion state on filter
    /// changes (root mounts always remain mounted).
    pub fn is_persistent_mount(&self) -> bool {
        matches!(self.kind, ExplorerNodeKind::RootMount)
    }

    /// True when the row is hidden from default search/scope surfaces.
    pub fn is_hidden_by_default(&self) -> bool {
        self.special_file_hint.is_some()
    }

    /// Build a [`VfsUri`] from the canonical_uri string. Returns `None` when
    /// the stored canonical_uri is not a valid URI (e.g. a workspace-root
    /// presentation URI).
    pub fn canonical_vfs_uri(&self) -> Option<VfsUri> {
        VfsUri::parse(self.canonical_uri.clone()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_for_lockfile_returns_hint_with_workspace_uri() {
        let hint = GeneratedArtifactHint::detect_for(
            "Cargo.lock",
            Some("wksp:test"),
            Some("root:repo"),
        )
        .expect("Cargo.lock must produce a hint");
        assert_eq!(
            hint.generated_class.as_deref(),
            Some(GeneratedArtifactClass::Lockfile.as_str())
        );
        assert_eq!(
            hint.generated_from_uri.as_deref(),
            Some("aureline-ws://wksp:test/root:repo/Cargo.toml")
        );
        assert_eq!(hint.freshness_class, "derived_from_canonical");
        assert_eq!(hint.lineage_class(), Some(GeneratedArtifactClass::Lockfile));
    }

    #[test]
    fn detect_for_ordinary_file_returns_none() {
        assert!(
            GeneratedArtifactHint::detect_for("src/main.rs", None, None).is_none(),
            "hand-authored sources must not produce a hint"
        );
    }

    #[test]
    fn detect_for_build_output_omits_canonical_uri() {
        let hint = GeneratedArtifactHint::detect_for(
            "target/debug/build/foo/out/api.rs",
            Some("wksp:test"),
            Some("root:repo"),
        )
        .expect("build output must produce a hint");
        assert!(hint.generated_from_uri.is_none());
        assert_eq!(hint.freshness_class, "possibly_stale");
        assert_eq!(
            hint.lineage_class(),
            Some(GeneratedArtifactClass::BuildOutput)
        );
    }

    #[test]
    fn legacy_hint_without_class_token_round_trips_through_serde() {
        let json = r#"{
            "producer_id": "producer:proto-codegen",
            "producer_label": "proto-codegen",
            "generated_from_uri": "aureline-ws://wksp:codegen/root:codegen/proto/api.proto",
            "freshness_class": "fresh"
        }"#;
        let parsed: GeneratedArtifactHint =
            serde_json::from_str(json).expect("legacy hint must parse");
        assert!(parsed.generated_class.is_none());
        assert!(parsed.lineage_class().is_none());
        assert_eq!(parsed.freshness_class, "fresh");
    }
}
