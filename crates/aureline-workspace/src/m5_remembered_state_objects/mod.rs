//! Materialized M5 remembered-state objects: the explicit, versioned state objects that restorable
//! M5 surfaces resolve against instead of an implicit, opaque layout blob.
//!
//! The serialization-and-restore matrix classifies *what* M5 is allowed to remember. This packet
//! implements the underlying state objects themselves, kept deliberately separate rather than
//! flattened into one convenience payload:
//!
//! - [`WorkspaceAuthorityCheckpoint`] — a versioned, re-resolvable record of the workspace
//!   authority that was granted. It preserves dirty-buffer identity, journal linkage, trusted
//!   roots, active worksets, and a restore class, and it never serializes a live authority ticket
//!   ([`AuthorityHandleClass::LiveTicket`] exists only so the gate can reject it).
//! - [`WindowTopologySnapshot`] — a versioned snapshot of one window's [`PaneTree`], chrome, and
//!   the boundary [`ScopeRefs`] that point at workspace authority, profile defaults, and
//!   machine-local hints by reference. Window-local topology never embeds authority state.
//! - [`ProfileDefaults`] — versioned, portable profile-level defaults that seed new windows. They
//!   never carry machine-local anchors.
//! - [`MachineLocalHints`] — versioned, machine-bound display geometry and install anchors. This is
//!   the one object that holds machine-unique state, and it is never exportable.
//!
//! [`PaneTree`] gives every pane a stable [`PaneLeaf::pane_id`] and a versioned schema
//! ([`PANE_TREE_SCHEMA_VERSION`]) so split, move/float, pin, close, and placeholder substitution are
//! serializable, diffable ([`PaneTree::diff`]), and migratable ([`migrate_pane_tree`]). A missing
//! dependency never silently deletes a slot: a degraded pane keeps its `pane_id` and original role
//! behind a [`PlaceholderCard`] whose [`SubstitutionBehavior::SilentDelete`] variant is reject-only.
//!
//! [`RememberedStateBundle`] wires the four objects together purely by reference, which is what
//! keeps authority, topology, profile, and machine-local state from collapsing back into one blob:
//! a bundle whose authority ref points at a window snapshot id is a flattening error
//! ([`M5RememberedStateViolation::FlattenedAuthorityTopology`]).
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-remembered-state-objects.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, live authority handles, or workspace
//! contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::mem;

use serde::{Deserialize, Serialize};

/// Supported top-level remembered-state-objects packet schema version.
pub const M5_REMEMBERED_STATE_SCHEMA_VERSION: u32 = 1;

/// Versioned schema id for [`WorkspaceAuthorityCheckpoint`].
pub const WORKSPACE_AUTHORITY_CHECKPOINT_SCHEMA_VERSION: u32 = 1;

/// Versioned schema id for [`WindowTopologySnapshot`].
pub const WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Versioned schema id for [`PaneTree`]. Bumped only on a breaking pane-tree payload change.
pub const PANE_TREE_SCHEMA_VERSION: u32 = 1;

/// Versioned schema id for [`ProfileDefaults`].
pub const PROFILE_DEFAULTS_SCHEMA_VERSION: u32 = 1;

/// Versioned schema id for [`MachineLocalHints`].
pub const MACHINE_LOCAL_HINTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_REMEMBERED_STATE_RECORD_KIND: &str = "m5_remembered_state_objects";

/// Repo-relative path to the checked-in packet.
pub const M5_REMEMBERED_STATE_PATH: &str =
    "artifacts/workspace/m5/m5-remembered-state-objects.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_REMEMBERED_STATE_SCHEMA_REF: &str =
    "schemas/workspace/m5-remembered-state.schema.json";

/// Repo-relative path to the companion document.
pub const M5_REMEMBERED_STATE_DOC_REF: &str = "docs/workspace/m5/m5-remembered-state-objects.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_REMEMBERED_STATE_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-remembered-state-objects.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_REMEMBERED_STATE_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-remembered-state-objects";

/// Embedded checked-in packet JSON.
pub const M5_REMEMBERED_STATE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-remembered-state-objects.json"
));

// --- Shared vocabularies -------------------------------------------------------------------------

/// The restore class a remembered-state object claims for itself.
///
/// Ordered best to worst by [`RestoreClass::rank`]: an exact restore reproduces prior state
/// value-for-value, a compatible restore reproduces it through a forward migration, a layout-only
/// restore reproduces the slots while contents reopen as context or show a placeholder, and a
/// manual-review restore cannot be applied automatically and is surfaced for a human.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClass {
    /// Restored value-for-value.
    ExactRestore,
    /// Restored through a forward schema migration; semantics preserved.
    CompatibleRestore,
    /// Only the pane/window slots are restored; contents reopen as context or show a placeholder.
    LayoutOnly,
    /// Cannot be restored automatically; surfaced for review with the slot preserved.
    ManualReview,
}

impl RestoreClass {
    /// Every restore class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::ManualReview,
    ];

    /// Stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::ManualReview => "manual_review",
        }
    }

    /// Monotonic rank; higher means more of the remembered state restores automatically.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ManualReview => 0,
            Self::LayoutOnly => 1,
            Self::CompatibleRestore => 2,
            Self::ExactRestore => 3,
        }
    }
}

/// How portable a remembered-state object is.
///
/// Only [`StateOwnership::Portable`] and [`StateOwnership::Shared`] state may travel in a
/// portable-state package; [`StateOwnership::Local`] and [`StateOwnership::MachineLocal`] state never
/// leaves the machine it was remembered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateOwnership {
    /// Portable across machines and users.
    Portable,
    /// Shared within a team/sharing scope; portable inside that scope.
    Shared,
    /// Local to this machine/install; restorable across restarts but never exported.
    Local,
    /// Bound to this machine/install; never serialized into a portable package.
    MachineLocal,
}

impl StateOwnership {
    /// Stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::Shared => "shared",
            Self::Local => "local",
            Self::MachineLocal => "machine_local",
        }
    }

    /// Whether state with this ownership may travel in a portable-state package.
    pub const fn exportable_into_portable_package(self) -> bool {
        matches!(self, Self::Portable | Self::Shared)
    }
}

/// How a workspace-authority reference is stored in a checkpoint.
///
/// A checkpoint stores authority as a re-resolvable reference that is re-evaluated at restore time.
/// A serialized live ticket is never legal; the variant exists only so the gate can reject it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityHandleClass {
    /// A re-resolvable reference; the live authority is re-acquired at restore, never replayed.
    ReResolvableReference,
    /// A serialized live authority ticket. Forbidden — present only for fail-closed rejection.
    LiveTicket,
}

impl AuthorityHandleClass {
    /// Stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReResolvableReference => "re_resolvable_reference",
            Self::LiveTicket => "live_ticket",
        }
    }

    /// Whether this handle class is safe to persist.
    pub const fn is_persistable(self) -> bool {
        matches!(self, Self::ReResolvableReference)
    }
}

// --- Workspace-authority checkpoint --------------------------------------------------------------

/// Kind of journal a checkpoint links to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalKind {
    /// The durable mutation journal.
    MutationJournal,
    /// A recovered-draft journal backing a dirty buffer.
    DraftRecovery,
    /// A command-history journal.
    CommandHistory,
}

/// Trust posture recorded for a trusted-root reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClass {
    /// The root is trusted.
    Trusted,
    /// The root is restricted pending an explicit decision at restore.
    Restricted,
}

/// The identity of a dirty buffer carried by a checkpoint.
///
/// A checkpoint records buffer *identity* and a draft-journal link, never buffer content. The
/// recovered draft (if any) lives behind [`DirtyBufferIdentity::draft_journal_ref`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyBufferIdentity {
    /// Stable buffer identity, distinct from any document path.
    pub buffer_id: String,
    /// Opaque reference to the document the buffer edits; never a raw path.
    pub document_ref: String,
    /// Optional link to the recovered-draft journal entry for this buffer.
    pub draft_journal_ref: Option<String>,
    /// Whether the buffer has unsaved changes.
    pub dirty: bool,
}

/// A link from a checkpoint to a durable journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalLink {
    /// Opaque reference to the journal.
    pub journal_ref: String,
    /// Kind of journal linked.
    pub kind: JournalKind,
}

/// A trusted-root reference carried by a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedRootRef {
    /// Opaque, re-resolvable reference to a trusted root; never a raw path.
    pub root_ref: String,
    /// Trust posture recorded for the root.
    pub trust_class: TrustClass,
}

/// A workset reference carried by a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetRef {
    /// Opaque reference to the workset.
    pub workset_ref: String,
    /// Whether the workset was active at checkpoint time.
    pub active: bool,
}

/// A versioned, re-resolvable checkpoint of the workspace authority that was granted.
///
/// The checkpoint preserves dirty-buffer identity, journal linkage, trusted roots, active worksets,
/// and a restore class. It never serializes a live authority ticket: the authority is captured only
/// as a re-resolvable reference ([`AuthorityHandleClass::ReResolvableReference`]) that is
/// re-evaluated at restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAuthorityCheckpoint {
    /// Schema version; must equal [`WORKSPACE_AUTHORITY_CHECKPOINT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable checkpoint id, referenced by [`RememberedStateBundle::workspace_authority_ref`].
    pub checkpoint_id: String,
    /// Opaque, re-resolvable reference to the granted workspace authority; never a live handle.
    pub workspace_authority_ref: String,
    /// How the authority reference is stored. Must be re-resolvable.
    pub authority_handle_class: AuthorityHandleClass,
    /// The restore class the checkpoint claims for the authority it remembers.
    pub restore_class: RestoreClass,
    /// Dirty-buffer identities (never contents) preserved by the checkpoint.
    pub dirty_buffers: Vec<DirtyBufferIdentity>,
    /// Journals this checkpoint links to.
    pub journal_links: Vec<JournalLink>,
    /// Trusted roots remembered by the checkpoint.
    pub trusted_roots: Vec<TrustedRootRef>,
    /// Worksets remembered by the checkpoint.
    pub active_worksets: Vec<WorksetRef>,
    /// Attestation that no live authority handle is serialized. Must be true.
    pub excludes_live_authority: bool,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
}

impl WorkspaceAuthorityCheckpoint {
    /// Whether the checkpoint keeps live authority out of the payload.
    pub fn is_authority_safe(&self) -> bool {
        self.authority_handle_class.is_persistable() && self.excludes_live_authority
    }
}

// --- Pane tree -----------------------------------------------------------------------------------

/// Orientation of a [`SplitNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitOrientation {
    /// Children laid out left-to-right.
    Horizontal,
    /// Children laid out top-to-bottom.
    Vertical,
}

/// User-facing role a pane occupies. A placeholder pane keeps its original role rather than
/// collapsing to a generic unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceRole {
    /// A text editor.
    Editor,
    /// A diff editor.
    Diff,
    /// A terminal.
    Terminal,
    /// A notebook.
    Notebook,
    /// A docs pane.
    Docs,
    /// A preview pane.
    Preview,
    /// An AI panel.
    AiPanel,
    /// An explorer pane.
    Explorer,
    /// A placeholder card standing in for a surface that could not be restored.
    Placeholder,
}

/// Concrete surface class a pane resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// A text editor surface.
    TextEditor,
    /// A diff editor surface.
    DiffEditor,
    /// A terminal surface.
    TerminalView,
    /// A notebook surface.
    NotebookView,
    /// A docs browser surface.
    DocsBrowser,
    /// A preview canvas surface.
    PreviewCanvas,
    /// An AI panel surface.
    AiPanel,
    /// An explorer tree surface.
    ExplorerTree,
    /// A placeholder card surface.
    PlaceholderCard,
}

/// Current availability of a pane's surface. A pane may degrade without changing its `pane_id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    /// The surface is live and attached.
    Ready,
    /// The surface is known but not yet hydrated.
    NeedsHydration,
    /// The surface is unavailable; a placeholder occupies the slot.
    Placeholder,
    /// The surface is unavailable; only evidence is retained.
    EvidenceOnly,
}

/// Why a placeholder card occupies a pane slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderReason {
    /// A required extension is missing.
    MissingExtension,
    /// A required remote is unreachable.
    MissingRemote,
    /// A required permission was revoked.
    RevokedPermission,
    /// The workspace authority is missing.
    MissingWorkspaceAuthority,
    /// The surface is a non-reentrant live surface that must not be rerun silently.
    NonReentrantLiveSurface,
    /// The display topology cannot host the surface.
    UnsupportedDisplayTopology,
    /// A schema migration needs manual review before the surface can restore.
    SchemaMigrationReviewRequired,
    /// Manual recovery is required.
    ManualRecoveryRequired,
}

/// A typed, safe recovery action rendered on a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderAction {
    /// Retry hydration.
    RetryHydrate,
    /// Install the missing extension.
    InstallExtension,
    /// Reauthenticate.
    Reauthenticate,
    /// Reconnect the remote.
    ReconnectRemote,
    /// Recover the draft.
    RecoverDraft,
    /// Relocate the missing dependency.
    RelocateDependency,
    /// Open the surface without the missing dependency.
    OpenWithout,
    /// Export the retained evidence.
    ExportEvidence,
    /// Remove the pane.
    RemovePane,
    /// Rerun the live surface explicitly.
    RerunExplicitly,
    /// Rebind an existing live session.
    RebindExistingSession,
}

/// How a missing dependency is handled when it would otherwise empty a pane slot.
///
/// [`SubstitutionBehavior::SilentDelete`] is reject-only: it exists in the vocabulary so the gate
/// can refuse a state object that would drop layout instead of preserving the slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstitutionBehavior {
    /// The slot is preserved and a placeholder occupies it.
    PlaceholderSlotPreserved,
    /// The contents reopen as context while the slot is preserved.
    ReopenAsContext,
    /// The slot is deleted. Forbidden — present only for fail-closed rejection.
    SilentDelete,
}

impl SubstitutionBehavior {
    /// Stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceholderSlotPreserved => "placeholder_slot_preserved",
            Self::ReopenAsContext => "reopen_as_context",
            Self::SilentDelete => "silent_delete",
        }
    }

    /// Whether this behavior preserves the pane slot.
    pub const fn preserves_slot(self) -> bool {
        !matches!(self, Self::SilentDelete)
    }
}

/// A diagnostic placeholder occupying a pane slot when the live surface cannot hydrate.
///
/// The card preserves the original role and surface class so the slot reads as the surface it
/// stands in for, not a generic unknown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderCard {
    /// Why the placeholder occupies the slot.
    pub reason: PlaceholderReason,
    /// The role the original surface occupied.
    pub original_role: SurfaceRole,
    /// The surface class the original surface resolved to.
    pub original_surface_class: SurfaceClass,
    /// How the missing dependency was handled. Must preserve the slot.
    pub substitution_behavior: SubstitutionBehavior,
    /// Safe recovery actions offered on the placeholder.
    pub safe_actions: Vec<PlaceholderAction>,
    /// Whether evidence was retained for the missing surface.
    pub evidence_retained: bool,
}

/// The surface payload attached to a [`PaneLeaf`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneSurface {
    /// User-facing role the pane occupies.
    pub surface_role: SurfaceRole,
    /// Concrete surface class the pane resolves to.
    pub surface_class: SurfaceClass,
    /// Current availability of the surface.
    pub availability: Availability,
    /// Placeholder card present when the surface degraded out of [`Availability::Ready`].
    pub placeholder: Option<PlaceholderCard>,
}

impl PaneSurface {
    /// Whether the surface is currently standing behind a placeholder.
    pub fn is_placeholder(&self) -> bool {
        matches!(self.availability, Availability::Placeholder)
    }
}

/// A leaf pane carrying one surface descriptor and a stable pane id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneLeaf {
    /// Stable pane identifier. Survives split, move/float, pin, close-sibling, and placeholder
    /// substitution.
    pub pane_id: String,
    /// The surface attached to the pane.
    pub surface: PaneSurface,
}

/// A split node in a pane tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitNode {
    /// Stable split id.
    pub split_id: String,
    /// Orientation of the split.
    pub orientation: SplitOrientation,
    /// Child nodes, index-aligned with [`SplitNode::weight_permille`].
    pub children: Vec<PaneNode>,
    /// Relative child weights in permille. Integer to keep the tree comparable and diffable.
    pub weight_permille: Vec<u32>,
}

/// One tab in a [`TabGroupNode`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabRecord {
    /// Stable tab id.
    pub tab_id: String,
    /// Whether the tab is pinned.
    pub pinned: bool,
    /// The pane shown by the tab.
    pub pane: PaneLeaf,
}

/// A tab-group node in a pane tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabGroupNode {
    /// Stable group id.
    pub group_id: String,
    /// Tabs in the group, in order.
    pub tabs: Vec<TabRecord>,
    /// The active tab's id.
    pub active_tab_id: String,
}

/// A recursive pane-tree node, discriminated by `node_kind`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "node_kind", rename_all = "snake_case")]
pub enum PaneNode {
    /// A split of two or more children.
    Split(SplitNode),
    /// A tab group of one or more tabs.
    TabGroup(TabGroupNode),
    /// A single leaf pane.
    Leaf(PaneLeaf),
}

/// A versioned pane tree for one window, with stable ids throughout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneTree {
    /// Schema version; must equal [`PANE_TREE_SCHEMA_VERSION`] in the canonical packet.
    pub schema_version: u32,
    /// Instance revision, incremented when the topology mutates.
    pub tree_revision: u32,
    /// Root node of the tree.
    pub root: PaneNode,
}

/// The set difference between two pane trees by stable pane id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneTreeDiff {
    /// Pane ids present in the new tree but not the old.
    pub added: Vec<String>,
    /// Pane ids present in the old tree but not the new.
    pub removed: Vec<String>,
    /// Pane ids present in both trees.
    pub retained: Vec<String>,
}

impl PaneNode {
    fn collect_pane_ids(&self, out: &mut Vec<String>) {
        match self {
            Self::Split(s) => {
                for child in &s.children {
                    child.collect_pane_ids(out);
                }
            }
            Self::TabGroup(g) => {
                for tab in &g.tabs {
                    out.push(tab.pane.pane_id.clone());
                }
            }
            Self::Leaf(l) => out.push(l.pane_id.clone()),
        }
    }

    fn collect_tab_ids(&self, out: &mut Vec<String>) {
        match self {
            Self::Split(s) => {
                for child in &s.children {
                    child.collect_tab_ids(out);
                }
            }
            Self::TabGroup(g) => {
                for tab in &g.tabs {
                    out.push(tab.tab_id.clone());
                }
            }
            Self::Leaf(_) => {}
        }
    }

    fn find_leaf(&self, pane_id: &str) -> Option<&PaneLeaf> {
        match self {
            Self::Split(s) => s.children.iter().find_map(|c| c.find_leaf(pane_id)),
            Self::TabGroup(g) => g
                .tabs
                .iter()
                .map(|t| &t.pane)
                .find(|p| p.pane_id == pane_id),
            Self::Leaf(l) => (l.pane_id == pane_id).then_some(l),
        }
    }

    fn substitute(&mut self, pane_id: &str, card: &PlaceholderCard) -> bool {
        match self {
            Self::Split(s) => s.children.iter_mut().any(|c| c.substitute(pane_id, card)),
            Self::TabGroup(g) => g.tabs.iter_mut().any(|t| {
                if t.pane.pane_id == pane_id {
                    apply_placeholder(&mut t.pane, card);
                    true
                } else {
                    false
                }
            }),
            Self::Leaf(l) => {
                if l.pane_id == pane_id {
                    apply_placeholder(l, card);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn set_tab_pinned(&mut self, tab_id: &str, pinned: bool) -> bool {
        match self {
            Self::Split(s) => s
                .children
                .iter_mut()
                .any(|c| c.set_tab_pinned(tab_id, pinned)),
            Self::TabGroup(g) => g.tabs.iter_mut().any(|t| {
                if t.tab_id == tab_id {
                    t.pinned = pinned;
                    true
                } else {
                    false
                }
            }),
            Self::Leaf(_) => false,
        }
    }

    /// Prunes `pane_id`, collapsing emptied splits and tab groups. Returns `None` when the node
    /// should disappear entirely so the slot never lingers as an empty container.
    fn prune(self, pane_id: &str) -> Option<Self> {
        match self {
            Self::Leaf(l) => {
                if l.pane_id == pane_id {
                    None
                } else {
                    Some(Self::Leaf(l))
                }
            }
            Self::Split(s) => {
                let SplitNode {
                    split_id,
                    orientation,
                    children,
                    weight_permille,
                } = s;
                let mut kept_children = Vec::new();
                let mut kept_weights = Vec::new();
                for (idx, child) in children.into_iter().enumerate() {
                    if let Some(pruned) = child.prune(pane_id) {
                        kept_children.push(pruned);
                        kept_weights.push(weight_permille.get(idx).copied().unwrap_or(0));
                    }
                }
                match kept_children.len() {
                    0 => None,
                    1 => Some(kept_children.into_iter().next().expect("len checked")),
                    _ => Some(Self::Split(SplitNode {
                        split_id,
                        orientation,
                        weight_permille: normalize_weights(kept_weights, kept_children.len()),
                        children: kept_children,
                    })),
                }
            }
            Self::TabGroup(g) => {
                let TabGroupNode {
                    group_id,
                    tabs,
                    active_tab_id,
                } = g;
                let kept: Vec<TabRecord> = tabs
                    .into_iter()
                    .filter(|t| t.pane.pane_id != pane_id)
                    .collect();
                if kept.is_empty() {
                    return None;
                }
                let active = if kept.iter().any(|t| t.tab_id == active_tab_id) {
                    active_tab_id
                } else {
                    kept[0].tab_id.clone()
                };
                Some(Self::TabGroup(TabGroupNode {
                    group_id,
                    tabs: kept,
                    active_tab_id: active,
                }))
            }
        }
    }

    /// Replaces the leaf `pane_id` with a split of `[old, new_leaf]`, modeling an interactive split.
    fn split_leaf(
        &mut self,
        pane_id: &str,
        split_id: &str,
        orientation: SplitOrientation,
        new_leaf: &PaneLeaf,
    ) -> bool {
        match self {
            Self::Leaf(l) if l.pane_id == pane_id => {
                let old = mem::replace(
                    l,
                    PaneLeaf {
                        pane_id: String::new(),
                        surface: PaneSurface {
                            surface_role: SurfaceRole::Placeholder,
                            surface_class: SurfaceClass::PlaceholderCard,
                            availability: Availability::Placeholder,
                            placeholder: None,
                        },
                    },
                );
                *self = Self::Split(SplitNode {
                    split_id: split_id.to_owned(),
                    orientation,
                    children: vec![Self::Leaf(old), Self::Leaf(new_leaf.clone())],
                    weight_permille: vec![500, 500],
                });
                true
            }
            Self::Leaf(_) => false,
            Self::Split(s) => s
                .children
                .iter_mut()
                .any(|c| c.split_leaf(pane_id, split_id, orientation, new_leaf)),
            Self::TabGroup(_) => false,
        }
    }
}

fn apply_placeholder(leaf: &mut PaneLeaf, card: &PlaceholderCard) {
    leaf.surface.availability = Availability::Placeholder;
    leaf.surface.placeholder = Some(card.clone());
}

fn normalize_weights(weights: Vec<u32>, len: usize) -> Vec<u32> {
    if weights.len() == len && weights.iter().all(|w| *w > 0) {
        weights
    } else {
        let each = (1000 / len.max(1)) as u32;
        vec![each.max(1); len]
    }
}

impl PaneTree {
    /// Every stable pane id in the tree, in traversal order.
    pub fn pane_ids(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.root.collect_pane_ids(&mut out);
        out
    }

    /// Every stable tab id in the tree, in traversal order.
    pub fn tab_ids(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.root.collect_tab_ids(&mut out);
        out
    }

    /// Whether every pane id in the tree is unique. Stable ids are only useful if they are unique.
    pub fn has_unique_pane_ids(&self) -> bool {
        let ids = self.pane_ids();
        let unique: BTreeSet<&String> = ids.iter().collect();
        unique.len() == ids.len()
    }

    /// The pane ids currently standing behind a placeholder.
    pub fn placeholder_pane_ids(&self) -> Vec<String> {
        self.pane_ids()
            .into_iter()
            .filter(|id| {
                self.find_leaf(id)
                    .map(|l| l.surface.is_placeholder())
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Looks up a leaf pane by stable id.
    pub fn find_leaf(&self, pane_id: &str) -> Option<&PaneLeaf> {
        self.root.find_leaf(pane_id)
    }

    /// Substitutes a placeholder for `pane_id`, preserving the slot and the pane id. Returns whether
    /// the pane was found. Bumps [`PaneTree::tree_revision`] on success.
    pub fn substitute_placeholder(&mut self, pane_id: &str, card: PlaceholderCard) -> bool {
        let found = self.root.substitute(pane_id, &card);
        if found {
            self.tree_revision = self.tree_revision.saturating_add(1);
        }
        found
    }

    /// Sets the pinned state of a tab, modeling a pin/unpin. Bumps the revision on success.
    pub fn set_tab_pinned(&mut self, tab_id: &str, pinned: bool) -> bool {
        let found = self.root.set_tab_pinned(tab_id, pinned);
        if found {
            self.tree_revision = self.tree_revision.saturating_add(1);
        }
        found
    }

    /// Closes a pane, collapsing emptied containers. Returns whether the pane was found. Bumps the
    /// revision on success.
    pub fn close_pane(&mut self, pane_id: &str) -> bool {
        if self.find_leaf(pane_id).is_none() {
            return false;
        }
        let root = mem::replace(
            &mut self.root,
            PaneNode::Leaf(PaneLeaf {
                pane_id: String::new(),
                surface: PaneSurface {
                    surface_role: SurfaceRole::Placeholder,
                    surface_class: SurfaceClass::PlaceholderCard,
                    availability: Availability::Placeholder,
                    placeholder: None,
                },
            }),
        );
        match root.clone().prune(pane_id) {
            Some(pruned) => {
                self.root = pruned;
                self.tree_revision = self.tree_revision.saturating_add(1);
                true
            }
            None => {
                // The tree would be empty; closing the last pane is a no-op that keeps the slot.
                self.root = root;
                false
            }
        }
    }

    /// Detaches a pane, returning the leaf so it can be re-homed in another window (a move or float).
    /// The detached leaf keeps its stable `pane_id`.
    pub fn detach_pane(&mut self, pane_id: &str) -> Option<PaneLeaf> {
        let leaf = self.find_leaf(pane_id)?.clone();
        if self.close_pane(pane_id) {
            Some(leaf)
        } else {
            None
        }
    }

    /// Splits the leaf `pane_id` into a split of `[old, new_leaf]`. Returns whether it was found.
    pub fn split_pane(
        &mut self,
        pane_id: &str,
        split_id: &str,
        orientation: SplitOrientation,
        new_leaf: PaneLeaf,
    ) -> bool {
        let found = self
            .root
            .split_leaf(pane_id, split_id, orientation, &new_leaf);
        if found {
            self.tree_revision = self.tree_revision.saturating_add(1);
        }
        found
    }

    /// Diffs this tree against `other` by stable pane id, treating `self` as the old tree.
    pub fn diff(&self, other: &PaneTree) -> PaneTreeDiff {
        let old: BTreeSet<String> = self.pane_ids().into_iter().collect();
        let new: BTreeSet<String> = other.pane_ids().into_iter().collect();
        PaneTreeDiff {
            added: new.difference(&old).cloned().collect(),
            removed: old.difference(&new).cloned().collect(),
            retained: old.intersection(&new).cloned().collect(),
        }
    }
}

/// The outcome of migrating a pane tree across schema versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneTreeMigrationOutcome {
    /// The tree was already at the current schema version.
    Exact,
    /// The tree was forward-migrated from an older schema version.
    ForwardMigrated,
    /// The tree is from a newer schema version this build cannot migrate.
    Unmigratable,
}

impl PaneTreeMigrationOutcome {
    /// The restore class implied by the migration outcome.
    pub const fn restore_class(self) -> RestoreClass {
        match self {
            Self::Exact => RestoreClass::ExactRestore,
            Self::ForwardMigrated => RestoreClass::CompatibleRestore,
            Self::Unmigratable => RestoreClass::ManualReview,
        }
    }
}

/// Migrates a pane tree from `from_version` to [`PANE_TREE_SCHEMA_VERSION`].
///
/// An equal version is exact; an older version is stamped current and forward-migrated; a newer
/// version is left untouched and reported as manual-review so the caller preserves the slots rather
/// than guessing at a payload it cannot read.
pub fn migrate_pane_tree(
    from_version: u32,
    mut tree: PaneTree,
) -> (PaneTree, PaneTreeMigrationOutcome) {
    use std::cmp::Ordering;
    match from_version.cmp(&PANE_TREE_SCHEMA_VERSION) {
        Ordering::Equal => {
            tree.schema_version = PANE_TREE_SCHEMA_VERSION;
            (tree, PaneTreeMigrationOutcome::Exact)
        }
        Ordering::Less => {
            tree.schema_version = PANE_TREE_SCHEMA_VERSION;
            (tree, PaneTreeMigrationOutcome::ForwardMigrated)
        }
        Ordering::Greater => (tree, PaneTreeMigrationOutcome::Unmigratable),
    }
}

// --- Window topology snapshot --------------------------------------------------------------------

/// Role of a window in a multi-window topology. Topology-only; never implies shared authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRole {
    /// The primary window.
    Primary,
    /// An auxiliary window.
    Auxiliary,
    /// A presentation window.
    Presentation,
    /// A review window.
    Review,
    /// An incident window.
    Incident,
    /// A companion window.
    Companion,
}

/// Top-level window chrome state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    /// Normal windowed state.
    Normal,
    /// Maximized.
    Maximized,
    /// Fullscreen.
    Fullscreen,
    /// Zen mode.
    Zen,
    /// Minimized.
    Minimized,
}

/// Window density preset. Window-local topology state, not workspace authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityPreset {
    /// Comfortable density.
    Comfortable,
    /// Compact density.
    Compact,
    /// Presentation density.
    Presentation,
}

/// The boundary refs that keep workspace authority, profile defaults, and machine-local hints
/// separate from window-local topology.
///
/// A window points at its authority, profile defaults, and machine-local hints by reference; it
/// never embeds them. This is the structural guard against flattening the four objects into one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRefs {
    /// Reference to the [`WorkspaceAuthorityCheckpoint`] backing this window.
    pub workspace_authority_ref: String,
    /// Optional reference to the [`ProfileDefaults`] that seeded the window.
    pub profile_defaults_ref: Option<String>,
    /// Optional reference to the [`MachineLocalHints`] this window quotes.
    pub machine_local_hints_ref: Option<String>,
}

/// Window-level chrome state separate from the pane tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowChromeState {
    /// Window chrome state.
    pub window_state: WindowState,
    /// Window density preset.
    pub density: DensityPreset,
}

/// A versioned snapshot of one window's topology.
///
/// The snapshot carries the window's [`PaneTree`], chrome, and the boundary [`ScopeRefs`]. It points
/// at workspace authority, profile defaults, and machine-local hints by reference rather than
/// embedding their state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowTopologySnapshot {
    /// Schema version; must equal [`WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_VERSION`] in the canonical packet.
    pub schema_version: u32,
    /// Stable snapshot id, referenced by [`RememberedStateBundle::window_snapshot_refs`].
    pub snapshot_id: String,
    /// Stable window id.
    pub window_id: String,
    /// Role of the window in a multi-window topology.
    pub window_role: WindowRole,
    /// Boundary refs to authority, profile defaults, and machine-local hints.
    pub scope_refs: ScopeRefs,
    /// The window's pane tree.
    pub pane_tree: PaneTree,
    /// The window's chrome state.
    pub window_chrome: WindowChromeState,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
}

// --- Profile defaults ----------------------------------------------------------------------------

/// An inspector that profile defaults open by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorKind {
    /// The outline inspector.
    Outline,
    /// The problems inspector.
    Problems,
    /// The search inspector.
    Search,
    /// The AI-evidence inspector.
    AiEvidence,
    /// The restore-diagnostics inspector.
    RestoreDiagnostics,
}

/// Versioned, portable profile-level defaults that seed new windows.
///
/// Profile defaults are portable and never carry machine-local anchors; those live in
/// [`MachineLocalHints`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileDefaults {
    /// Schema version; must equal [`PROFILE_DEFAULTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable profile-defaults id.
    pub profile_defaults_id: String,
    /// Ownership; must be portable or shared.
    pub ownership: StateOwnership,
    /// Default window density.
    pub default_density: DensityPreset,
    /// Default window role for new windows.
    pub default_window_role: WindowRole,
    /// Inspectors opened by default.
    pub default_inspectors: Vec<InspectorKind>,
    /// Attestation that no machine-local anchor is carried. Must be true.
    pub excludes_machine_local_anchors: bool,
}

// --- Machine-local hints -------------------------------------------------------------------------

/// Best-effort display class for a monitor-affinity hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayClass {
    /// An internal panel.
    InternalPanel,
    /// An external monitor.
    ExternalMonitor,
    /// A virtual display.
    VirtualDisplay,
    /// A projector or presentation display.
    ProjectorOrPresentation,
    /// Unknown display class.
    Unknown,
}

/// Best-effort display scale bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ScaleBucket {
    /// 1x scale.
    #[serde(rename = "1x")]
    One,
    /// 1.25x scale.
    #[serde(rename = "1_25x")]
    OneQuarter,
    /// 1.5x scale.
    #[serde(rename = "1_5x")]
    OneHalf,
    /// 2x scale.
    #[serde(rename = "2x")]
    Two,
    /// Some other scale.
    #[serde(rename = "other")]
    Other,
}

/// A best-effort monitor-affinity hint. Machine-local; never authoritative across machines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorAffinityHint {
    /// Opaque reference to the last-known display.
    pub display_ref: String,
    /// Best-effort display class.
    pub display_class: DisplayClass,
    /// Best-effort scale bucket.
    pub scale_bucket: ScaleBucket,
}

/// Versioned, machine-bound display geometry and install anchors.
///
/// This is the one object that holds machine-unique state — display topology, monitor affinity, and
/// install-root anchors — so the other three objects stay portable. It is never exportable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineLocalHints {
    /// Schema version; must equal [`MACHINE_LOCAL_HINTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable machine-local-hints id.
    pub machine_local_hints_id: String,
    /// Ownership; must be machine-local.
    pub ownership: StateOwnership,
    /// Whether the hints are exportable. Must be false.
    pub exportable: bool,
    /// Opaque hash of the last-known display topology.
    pub display_topology_hash: String,
    /// Best-effort monitor-affinity hints.
    pub monitor_affinity: Vec<MonitorAffinityHint>,
    /// Opaque reference to the install root anchor; never a raw path.
    pub install_root_anchor_ref: String,
}

// --- Remembered-state bundle ---------------------------------------------------------------------

/// Wires the four state objects together purely by reference.
///
/// A bundle is how a restorable surface resolves the explicit state objects it depends on without
/// flattening them: authority, topology, profile defaults, and machine-local hints are each a
/// reference into the packet, never inline state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateBundle {
    /// Stable bundle id.
    pub bundle_id: String,
    /// The overall restore class the bundle claims.
    pub restore_class: RestoreClass,
    /// Reference to the [`WorkspaceAuthorityCheckpoint`] backing the bundle.
    pub workspace_authority_ref: String,
    /// References to the [`WindowTopologySnapshot`] members of the bundle.
    pub window_snapshot_refs: Vec<String>,
    /// Optional reference to the [`ProfileDefaults`] seeding the bundle.
    pub profile_defaults_ref: Option<String>,
    /// Optional reference to the [`MachineLocalHints`] the bundle quotes.
    pub machine_local_hints_ref: Option<String>,
}

// --- Packet --------------------------------------------------------------------------------------

/// The versioned schema-id registry the packet pins for each state object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectSchemaVersions {
    /// Schema version pinned for [`WorkspaceAuthorityCheckpoint`].
    pub workspace_authority_checkpoint: u32,
    /// Schema version pinned for [`WindowTopologySnapshot`].
    pub window_topology_snapshot: u32,
    /// Schema version pinned for [`PaneTree`].
    pub pane_tree: u32,
    /// Schema version pinned for [`ProfileDefaults`].
    pub profile_defaults: u32,
    /// Schema version pinned for [`MachineLocalHints`].
    pub machine_local_hints: u32,
}

impl ObjectSchemaVersions {
    /// The schema-id registry this build expects.
    pub const fn current() -> Self {
        Self {
            workspace_authority_checkpoint: WORKSPACE_AUTHORITY_CHECKPOINT_SCHEMA_VERSION,
            window_topology_snapshot: WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_VERSION,
            pane_tree: PANE_TREE_SCHEMA_VERSION,
            profile_defaults: PROFILE_DEFAULTS_SCHEMA_VERSION,
            machine_local_hints: MACHINE_LOCAL_HINTS_SCHEMA_VERSION,
        }
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RememberedStateSummary {
    /// Number of workspace-authority checkpoints.
    pub workspace_authority_checkpoints: usize,
    /// Number of window-topology snapshots.
    pub window_topology_snapshots: usize,
    /// Number of profile-defaults objects.
    pub profile_defaults: usize,
    /// Number of machine-local-hints objects.
    pub machine_local_hints: usize,
    /// Number of bundles.
    pub bundles: usize,
    /// Total panes across all snapshots.
    pub total_panes: usize,
    /// Panes currently standing behind a placeholder.
    pub placeholder_panes: usize,
}

/// The typed M5 remembered-state-objects packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RememberedStateObjects {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Versioned schema-id registry for each state object.
    pub object_schema_versions: ObjectSchemaVersions,
    /// Worked workspace-authority checkpoints.
    #[serde(default)]
    pub workspace_authority_checkpoints: Vec<WorkspaceAuthorityCheckpoint>,
    /// Worked window-topology snapshots.
    #[serde(default)]
    pub window_topology_snapshots: Vec<WindowTopologySnapshot>,
    /// Worked profile-defaults objects.
    #[serde(default)]
    pub profile_defaults: Vec<ProfileDefaults>,
    /// Worked machine-local-hints objects.
    #[serde(default)]
    pub machine_local_hints: Vec<MachineLocalHints>,
    /// Worked bundles tying the four objects together by reference.
    #[serde(default)]
    pub bundles: Vec<RememberedStateBundle>,
    /// Summary counts.
    pub summary: M5RememberedStateSummary,
}

impl M5RememberedStateObjects {
    /// Looks up a checkpoint by id.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&WorkspaceAuthorityCheckpoint> {
        self.workspace_authority_checkpoints
            .iter()
            .find(|c| c.checkpoint_id == checkpoint_id)
    }

    /// Looks up a window snapshot by id.
    pub fn snapshot(&self, snapshot_id: &str) -> Option<&WindowTopologySnapshot> {
        self.window_topology_snapshots
            .iter()
            .find(|s| s.snapshot_id == snapshot_id)
    }

    /// Looks up profile defaults by id.
    pub fn profile(&self, profile_defaults_id: &str) -> Option<&ProfileDefaults> {
        self.profile_defaults
            .iter()
            .find(|p| p.profile_defaults_id == profile_defaults_id)
    }

    /// Looks up machine-local hints by id.
    pub fn machine_hints(&self, machine_local_hints_id: &str) -> Option<&MachineLocalHints> {
        self.machine_local_hints
            .iter()
            .find(|m| m.machine_local_hints_id == machine_local_hints_id)
    }

    /// Recomputes the summary block from the objects.
    pub fn computed_summary(&self) -> M5RememberedStateSummary {
        let total_panes = self
            .window_topology_snapshots
            .iter()
            .map(|s| s.pane_tree.pane_ids().len())
            .sum();
        let placeholder_panes = self
            .window_topology_snapshots
            .iter()
            .map(|s| s.pane_tree.placeholder_pane_ids().len())
            .sum();
        M5RememberedStateSummary {
            workspace_authority_checkpoints: self.workspace_authority_checkpoints.len(),
            window_topology_snapshots: self.window_topology_snapshots.len(),
            profile_defaults: self.profile_defaults.len(),
            machine_local_hints: self.machine_local_hints.len(),
            bundles: self.bundles.len(),
            total_panes,
            placeholder_panes,
        }
    }

    /// Builds an export-safe support packet preserving the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5RememberedStateSupportExport {
        M5RememberedStateSupportExport {
            record_kind: M5_REMEMBERED_STATE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_REMEMBERED_STATE_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            packet: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5RememberedStateViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_REMEMBERED_STATE_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_REMEMBERED_STATE_RECORD_KIND {
            violations.push(M5RememberedStateViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RememberedStateViolation::EmptyField {
                    id: self.packet_id.clone(),
                    field_name: field,
                });
            }
        }
        if self.object_schema_versions != ObjectSchemaVersions::current() {
            violations.push(M5RememberedStateViolation::SchemaVersionRegistryMismatch);
        }

        for ckpt in &self.workspace_authority_checkpoints {
            self.validate_checkpoint(ckpt, &mut violations);
        }
        for snap in &self.window_topology_snapshots {
            self.validate_snapshot(snap, &mut violations);
        }
        for profile in &self.profile_defaults {
            self.validate_profile(profile, &mut violations);
        }
        for hints in &self.machine_local_hints {
            self.validate_machine_hints(hints, &mut violations);
        }
        for bundle in &self.bundles {
            self.validate_bundle(bundle, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5RememberedStateViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_checkpoint(
        &self,
        ckpt: &WorkspaceAuthorityCheckpoint,
        violations: &mut Vec<M5RememberedStateViolation>,
    ) {
        if ckpt.schema_version != WORKSPACE_AUTHORITY_CHECKPOINT_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::ObjectSchemaVersionMismatch {
                id: ckpt.checkpoint_id.clone(),
                object: "workspace_authority_checkpoint",
                expected: WORKSPACE_AUTHORITY_CHECKPOINT_SCHEMA_VERSION,
                actual: ckpt.schema_version,
            });
        }
        for (field, value) in [
            ("checkpoint_id", &ckpt.checkpoint_id),
            ("workspace_authority_ref", &ckpt.workspace_authority_ref),
            ("emitted_at", &ckpt.emitted_at),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RememberedStateViolation::EmptyField {
                    id: ckpt.checkpoint_id.clone(),
                    field_name: field,
                });
            }
        }
        // The core guardrail: a checkpoint never serializes live authority.
        if !ckpt.is_authority_safe() {
            violations.push(M5RememberedStateViolation::LiveAuthoritySerialized {
                checkpoint_id: ckpt.checkpoint_id.clone(),
                handle_class: ckpt.authority_handle_class.as_str(),
            });
        }
        // A dirty buffer carries identity, not content; both refs must be present.
        for buf in &ckpt.dirty_buffers {
            if buf.buffer_id.trim().is_empty() || buf.document_ref.trim().is_empty() {
                violations.push(M5RememberedStateViolation::EmptyField {
                    id: ckpt.checkpoint_id.clone(),
                    field_name: "dirty_buffers",
                });
            }
        }
    }

    fn validate_snapshot(
        &self,
        snap: &WindowTopologySnapshot,
        violations: &mut Vec<M5RememberedStateViolation>,
    ) {
        if snap.schema_version != WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::ObjectSchemaVersionMismatch {
                id: snap.snapshot_id.clone(),
                object: "window_topology_snapshot",
                expected: WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_VERSION,
                actual: snap.schema_version,
            });
        }
        if snap.pane_tree.schema_version != PANE_TREE_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::ObjectSchemaVersionMismatch {
                id: snap.snapshot_id.clone(),
                object: "pane_tree",
                expected: PANE_TREE_SCHEMA_VERSION,
                actual: snap.pane_tree.schema_version,
            });
        }
        for (field, value) in [
            ("snapshot_id", &snap.snapshot_id),
            ("window_id", &snap.window_id),
            (
                "scope_refs.workspace_authority_ref",
                &snap.scope_refs.workspace_authority_ref,
            ),
            ("emitted_at", &snap.emitted_at),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RememberedStateViolation::EmptyField {
                    id: snap.snapshot_id.clone(),
                    field_name: field,
                });
            }
        }
        // Stable pane ids are only useful if unique.
        if !snap.pane_tree.has_unique_pane_ids() {
            violations.push(M5RememberedStateViolation::DuplicatePaneId {
                snapshot_id: snap.snapshot_id.clone(),
            });
        }
        // Placeholder slots must never silently delete layout, and a placeholder pane must carry a
        // card that preserves the slot.
        for pane_id in snap.pane_tree.pane_ids() {
            let Some(leaf) = snap.pane_tree.find_leaf(&pane_id) else {
                continue;
            };
            if leaf.surface.is_placeholder() {
                match &leaf.surface.placeholder {
                    None => violations.push(M5RememberedStateViolation::PlaceholderWithoutCard {
                        snapshot_id: snap.snapshot_id.clone(),
                        pane_id: pane_id.clone(),
                    }),
                    Some(card) => {
                        if !card.substitution_behavior.preserves_slot() {
                            violations.push(M5RememberedStateViolation::SilentLayoutDelete {
                                snapshot_id: snap.snapshot_id.clone(),
                                pane_id: pane_id.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    fn validate_profile(
        &self,
        profile: &ProfileDefaults,
        violations: &mut Vec<M5RememberedStateViolation>,
    ) {
        if profile.schema_version != PROFILE_DEFAULTS_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::ObjectSchemaVersionMismatch {
                id: profile.profile_defaults_id.clone(),
                object: "profile_defaults",
                expected: PROFILE_DEFAULTS_SCHEMA_VERSION,
                actual: profile.schema_version,
            });
        }
        // Profile defaults are portable and never carry machine-local anchors.
        if !profile.ownership.exportable_into_portable_package()
            || !profile.excludes_machine_local_anchors
        {
            violations.push(M5RememberedStateViolation::MachineLocalAnchorInPortable {
                id: profile.profile_defaults_id.clone(),
                ownership: profile.ownership.as_str(),
            });
        }
    }

    fn validate_machine_hints(
        &self,
        hints: &MachineLocalHints,
        violations: &mut Vec<M5RememberedStateViolation>,
    ) {
        if hints.schema_version != MACHINE_LOCAL_HINTS_SCHEMA_VERSION {
            violations.push(M5RememberedStateViolation::ObjectSchemaVersionMismatch {
                id: hints.machine_local_hints_id.clone(),
                object: "machine_local_hints",
                expected: MACHINE_LOCAL_HINTS_SCHEMA_VERSION,
                actual: hints.schema_version,
            });
        }
        // Machine-local hints are bound to this machine and never leave it.
        if hints.ownership != StateOwnership::MachineLocal || hints.exportable {
            violations.push(M5RememberedStateViolation::NonPortableExport {
                id: hints.machine_local_hints_id.clone(),
                ownership: hints.ownership.as_str(),
            });
        }
    }

    fn validate_bundle(
        &self,
        bundle: &RememberedStateBundle,
        violations: &mut Vec<M5RememberedStateViolation>,
    ) {
        if bundle.bundle_id.trim().is_empty() {
            violations.push(M5RememberedStateViolation::EmptyField {
                id: bundle.bundle_id.clone(),
                field_name: "bundle_id",
            });
        }
        // Authority and topology must stay distinct objects: an authority ref that points at a
        // window snapshot is a flattening of the two.
        if self.snapshot(&bundle.workspace_authority_ref).is_some() {
            violations.push(M5RememberedStateViolation::FlattenedAuthorityTopology {
                bundle_id: bundle.bundle_id.clone(),
            });
        }
        if self.checkpoint(&bundle.workspace_authority_ref).is_none() {
            violations.push(M5RememberedStateViolation::DanglingRef {
                bundle_id: bundle.bundle_id.clone(),
                referent: "workspace_authority_ref",
                value: bundle.workspace_authority_ref.clone(),
            });
        }
        if bundle.window_snapshot_refs.is_empty() {
            violations.push(M5RememberedStateViolation::EmptyField {
                id: bundle.bundle_id.clone(),
                field_name: "window_snapshot_refs",
            });
        }
        for snap_ref in &bundle.window_snapshot_refs {
            if self.snapshot(snap_ref).is_none() {
                violations.push(M5RememberedStateViolation::DanglingRef {
                    bundle_id: bundle.bundle_id.clone(),
                    referent: "window_snapshot_refs",
                    value: snap_ref.clone(),
                });
            }
        }
        if let Some(profile_ref) = &bundle.profile_defaults_ref {
            if self.profile(profile_ref).is_none() {
                violations.push(M5RememberedStateViolation::DanglingRef {
                    bundle_id: bundle.bundle_id.clone(),
                    referent: "profile_defaults_ref",
                    value: profile_ref.clone(),
                });
            }
        }
        if let Some(hints_ref) = &bundle.machine_local_hints_ref {
            if self.machine_hints(hints_ref).is_none() {
                violations.push(M5RememberedStateViolation::DanglingRef {
                    bundle_id: bundle.bundle_id.clone(),
                    referent: "machine_local_hints_ref",
                    value: hints_ref.clone(),
                });
            }
        }
        // A bundle must not overstate its restore class: any referenced snapshot carrying a
        // placeholder pane caps the bundle below an exact restore.
        let any_placeholder = bundle.window_snapshot_refs.iter().any(|r| {
            self.snapshot(r)
                .map(|s| !s.pane_tree.placeholder_pane_ids().is_empty())
                .unwrap_or(false)
        });
        if any_placeholder && bundle.restore_class == RestoreClass::ExactRestore {
            violations.push(M5RememberedStateViolation::BundleOverstatesFidelity {
                bundle_id: bundle.bundle_id.clone(),
            });
        }
    }
}

/// A validation violation for [`M5RememberedStateObjects`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum M5RememberedStateViolation {
    /// The packet schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Observed version.
        actual: u32,
    },
    /// The packet record kind is unsupported.
    UnsupportedRecordKind {
        /// Observed record kind.
        actual: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Owning object id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The pinned schema-id registry disagrees with this build.
    SchemaVersionRegistryMismatch,
    /// An object carries a schema version this build does not expect.
    ObjectSchemaVersionMismatch {
        /// Object id.
        id: String,
        /// Object kind token.
        object: &'static str,
        /// Expected version.
        expected: u32,
        /// Observed version.
        actual: u32,
    },
    /// A checkpoint serializes live authority instead of a re-resolvable reference.
    LiveAuthoritySerialized {
        /// Checkpoint id.
        checkpoint_id: String,
        /// Offending handle class token.
        handle_class: &'static str,
    },
    /// A window snapshot carries duplicate pane ids.
    DuplicatePaneId {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A placeholder pane carries no placeholder card.
    PlaceholderWithoutCard {
        /// Snapshot id.
        snapshot_id: String,
        /// Pane id.
        pane_id: String,
    },
    /// A placeholder would silently delete layout instead of preserving the slot.
    SilentLayoutDelete {
        /// Snapshot id.
        snapshot_id: String,
        /// Pane id.
        pane_id: String,
    },
    /// A portable object carries machine-local ownership or anchors.
    MachineLocalAnchorInPortable {
        /// Object id.
        id: String,
        /// Ownership token.
        ownership: &'static str,
    },
    /// A machine-local object is marked exportable or is not machine-local.
    NonPortableExport {
        /// Object id.
        id: String,
        /// Ownership token.
        ownership: &'static str,
    },
    /// A bundle points its authority ref at a window snapshot, flattening the two objects.
    FlattenedAuthorityTopology {
        /// Bundle id.
        bundle_id: String,
    },
    /// A bundle reference does not resolve to an object of the expected kind.
    DanglingRef {
        /// Bundle id.
        bundle_id: String,
        /// Which reference field.
        referent: &'static str,
        /// Offending value.
        value: String,
    },
    /// A bundle claims an exact restore over a snapshot that carries a placeholder pane.
    BundleOverstatesFidelity {
        /// Bundle id.
        bundle_id: String,
    },
    /// The summary counts disagree with the objects.
    SummaryMismatch,
}

impl fmt::Display for M5RememberedStateViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::SchemaVersionRegistryMismatch => {
                write!(f, "object_schema_versions disagrees with this build")
            }
            Self::ObjectSchemaVersionMismatch {
                id,
                object,
                expected,
                actual,
            } => write!(
                f,
                "{object} {id} carries schema version {actual} but this build expects {expected}"
            ),
            Self::LiveAuthoritySerialized {
                checkpoint_id,
                handle_class,
            } => write!(
                f,
                "checkpoint {checkpoint_id} serializes live authority ({handle_class})"
            ),
            Self::DuplicatePaneId { snapshot_id } => {
                write!(f, "snapshot {snapshot_id} carries duplicate pane ids")
            }
            Self::PlaceholderWithoutCard {
                snapshot_id,
                pane_id,
            } => write!(
                f,
                "snapshot {snapshot_id} pane {pane_id} is a placeholder with no card"
            ),
            Self::SilentLayoutDelete {
                snapshot_id,
                pane_id,
            } => write!(
                f,
                "snapshot {snapshot_id} pane {pane_id} would silently delete layout"
            ),
            Self::MachineLocalAnchorInPortable { id, ownership } => write!(
                f,
                "portable object {id} carries machine-local state ({ownership})"
            ),
            Self::NonPortableExport { id, ownership } => write!(
                f,
                "machine-local object {id} is exportable or not machine-local ({ownership})"
            ),
            Self::FlattenedAuthorityTopology { bundle_id } => write!(
                f,
                "bundle {bundle_id} flattens workspace authority into window topology"
            ),
            Self::DanglingRef {
                bundle_id,
                referent,
                value,
            } => write!(f, "bundle {bundle_id} {referent} {value} does not resolve"),
            Self::BundleOverstatesFidelity { bundle_id } => write!(
                f,
                "bundle {bundle_id} claims exact restore over a placeholder pane"
            ),
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the objects"),
        }
    }
}

impl Error for M5RememberedStateViolation {}

/// Stable record-kind tag for [`M5RememberedStateSupportExport`].
pub const M5_REMEMBERED_STATE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_remembered_state_objects_support_export";

/// Support-export wrapper preserving the packet verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RememberedStateSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact packet preserved by the export.
    pub packet: M5RememberedStateObjects,
}

impl M5RememberedStateSupportExport {
    /// Whether the export preserves the same packet id and a clean packet.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_REMEMBERED_STATE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_REMEMBERED_STATE_SCHEMA_VERSION
            && self.packet_id_ref == self.packet.packet_id
            && self.raw_private_material_excluded
            && self.packet.validate().is_empty()
    }
}

/// Loads the embedded M5 remembered-state-objects packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5RememberedStateObjects`].
pub fn current_m5_remembered_state_objects() -> Result<M5RememberedStateObjects, serde_json::Error>
{
    serde_json::from_str(M5_REMEMBERED_STATE_JSON)
}

#[cfg(test)]
mod tests;
