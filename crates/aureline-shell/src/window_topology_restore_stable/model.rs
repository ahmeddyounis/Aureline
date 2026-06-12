//! Canonical stable truth model for **multi-window, pane-detach, split-layout,
//! mixed-DPI, and multi-monitor restore** on a claimed-stable desktop shell.
//!
//! ## Why one governed record per window reopen
//!
//! Restoring a desktop session is two separable problems that competitors
//! routinely fuse: rebuilding the **window topology** (which windows existed,
//! their pane trees, focus history, and visible surfaces) and resuming the
//! **session-scoped execution** behind a pane (a live terminal, a debug
//! session, a notebook kernel, a query console, a preview route, a profiler
//! capture, an incident workspace, or a remote-backed surface). When
//! they are fused, a restored window silently reacquires live authority: a
//! terminal re-runs a deploy, a debugger reattaches, a remote pane reconnects
//! without consent. When the display topology changed underneath the save —
//! a monitor unplugged, a mixed-DPI dock cycle, a wake-from-sleep bounds shift —
//! the window can also reopen off-screen, at the wrong scale, or collapse panes
//! whose extension or remote target is now absent.
//!
//! This module mints one governed [`WindowTopologyRestoreRecord`] per claimed
//! window reopen. The record binds, for a single window identity:
//!
//! - **Authority / topology separation** — workspace authority (dirty buffers,
//!   recovery journals, trust/policy, VFS identity, attached execution contexts)
//!   stays centralized and shared; pane-tree layout, focus history, zoom/follow
//!   state, and visible surfaces stay window-local. The record proves the two
//!   are not fused.
//! - **A versioned pane-tree with stable pane IDs** — split, move, float, pin,
//!   and close-pane mutate one versioned [`PaneTree`] keyed by stable pane IDs.
//! - **Skeleton-first / hydrate-second restore** — the pane structure is
//!   recreated first; session-scoped panes hydrate into a truthful placeholder
//!   or reconnect state and never silently reacquire live authority.
//! - **Restore-no-rerun honesty** — every session-scoped pane that did not
//!   survive keeps its slot with an in-place placeholder card
//!   ([`PanePlaceholderState`]) and forbids command rerun and authority
//!   reacquire until an explicit user action.
//! - **Display-topology and downgrade provenance** — monitor changes, missing-
//!   extension substitutions, expired managed / remote sessions, and any
//!   downgrade from Exact to Compatible, Layout-only, or placeholder-backed are
//!   recorded so the layout change is explainable, not surprising.
//! - **A public claim ceiling** and **automatic narrowing** — a reopen that
//!   cannot prove a pillar, or whose lowest binding surface marker is below
//!   Stable, narrows below Stable with a named reason instead of inheriting an
//!   adjacent green row.
//!
//! The desktop restore-review UI, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning
//! status text. The pane-tree vocabulary, the topology-change classes, and the
//! restore adjustments are **not** reinvented here: the record projects the live
//! [`crate::windows`] workspace-management page, the [`crate::restore`] provenance
//! contract, and the [`crate::layout`] split tree.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};
use crate::windows::{
    RestoreAdjustmentClass, SplitAxisClass, TopologyChangeClass, WindowRoleClass,
};

/// Stable record-kind tag carried in serialized records.
pub const WINDOW_TOPOLOGY_RESTORE_RECORD_KIND: &str = "window_topology_restore_record";

/// Schema version for the [`WindowTopologyRestoreRecord`] payload shape.
pub const WINDOW_TOPOLOGY_RESTORE_SCHEMA_VERSION: u32 = 1;

/// Current versioned pane-tree schema version.
pub const PANE_TREE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const WINDOW_TOPOLOGY_RESTORE_SHARED_CONTRACT_REF: &str =
    "shell:window_topology_restore_stable:v1";

/// Reviewer-facing notice rendered on every restore surface.
pub const WINDOW_TOPOLOGY_RESTORE_NOTICE: &str =
    "Window-restore truth: window-topology restore is separated from session-scoped execution \
     restore; restoring a window recreates pane shells, the pane tree, and focus history, but a \
     terminal, debug, notebook, query-console, preview-route, profiler-capture, incident-workspace, \
     or remote-backed pane hydrates into a truthful placeholder or reconnect state instead of \
     silently reacquiring live authority; workspace authority — dirty \
     buffers, recovery journals, trust and policy, VFS identity, and attached execution contexts — \
     stays centralized while pane-tree layout, focus history, zoom/follow state, and visible \
     surfaces stay window-local; split, move, float, pin, and close-pane mutate one versioned \
     pane-tree keyed by stable pane IDs and restore is skeleton-first then hydrate-second so a \
     missing extension, an expired remote or managed session, or an unsupported display topology \
     preserves the pane slot with an in-place placeholder card rather than collapsing the pane; \
     every reopen records its restore fidelity (Exact, Compatible, Layout-only, or placeholder-\
     backed) and any display-topology adjustment, missing-extension substitution, or expired-\
     managed-session downgrade so the layout change is explainable in diagnostics and support \
     export without scraping localized UI copy; command rerun and authority reacquire are forbidden \
     until an explicit user action; the desktop restore review, CLI inspect, Help/About, and \
     support export read one shared record; and a reopen that cannot prove a pillar, or whose \
     lowest binding surface marker is below Stable, narrows below Stable with a named reason rather \
     than inheriting an adjacent green row.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present ref.
const MAX_REF_CHARS: usize = 200;

// ---------------------------------------------------------------------------
// Authority / topology separation
// ---------------------------------------------------------------------------

/// Workspace-authority state class that must stay centralized and shared across
/// the windows that view one workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAuthorityClass {
    /// Unsaved buffer authority and its dirty-buffer journals.
    DirtyBuffers,
    /// Recovery journals and crash checkpoints.
    RecoveryJournals,
    /// Trust state and policy decisions.
    TrustAndPolicy,
    /// Canonical VFS / workspace identity.
    VfsIdentity,
    /// Attached execution contexts (run targets, kernels, debug sessions).
    AttachedExecutionContexts,
}

impl WorkspaceAuthorityClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirtyBuffers => "dirty_buffers",
            Self::RecoveryJournals => "recovery_journals",
            Self::TrustAndPolicy => "trust_and_policy",
            Self::VfsIdentity => "vfs_identity",
            Self::AttachedExecutionContexts => "attached_execution_contexts",
        }
    }

    /// Every workspace-authority class a separation disclosure must enumerate.
    pub const REQUIRED: [Self; 5] = [
        Self::DirtyBuffers,
        Self::RecoveryJournals,
        Self::TrustAndPolicy,
        Self::VfsIdentity,
        Self::AttachedExecutionContexts,
    ];
}

/// Window-local topology state class that stays scoped to one window and never
/// migrates into shared workspace authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowLocalTopologyClass {
    /// The pane-tree layout (splits, weights, slots).
    PaneTreeLayout,
    /// Focus history within the window.
    FocusHistory,
    /// Zoom and follow state.
    ZoomFollowState,
    /// The set of visible surfaces in the window.
    VisibleSurfaces,
}

impl WindowLocalTopologyClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PaneTreeLayout => "pane_tree_layout",
            Self::FocusHistory => "focus_history",
            Self::ZoomFollowState => "zoom_follow_state",
            Self::VisibleSurfaces => "visible_surfaces",
        }
    }

    /// Every window-local topology class a separation disclosure must enumerate.
    pub const REQUIRED: [Self; 4] = [
        Self::PaneTreeLayout,
        Self::FocusHistory,
        Self::ZoomFollowState,
        Self::VisibleSurfaces,
    ];
}

/// The explicit separation between centralized workspace authority and
/// window-local topology in the persisted model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySeparation {
    /// Canonical ref to the shared workspace authority object.
    pub workspace_authority_ref: String,
    /// Centralized workspace-authority classes, in canonical order.
    pub workspace_authority_classes: Vec<WorkspaceAuthorityClass>,
    /// Window-local topology classes, in canonical order.
    pub window_local_topology_classes: Vec<WindowLocalTopologyClass>,
    /// Whether workspace authority is actually centralized and shared.
    pub workspace_authority_centralized: bool,
    /// Whether window topology is actually window-local.
    pub topology_window_local: bool,
}

// ---------------------------------------------------------------------------
// Pane surfaces + hydration
// ---------------------------------------------------------------------------

/// Surface class a restored pane carries. Session-scoped classes own live
/// authority that must never be silently reacquired on restore; the remaining
/// classes re-read truthfully from durable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneSurfaceClass {
    /// Text editor surface (re-readable).
    Editor,
    /// Diff / review surface (re-readable).
    Diff,
    /// Search results surface (re-readable).
    Search,
    /// Problems panel (re-readable).
    Problems,
    /// Source-control panel (re-readable).
    Scm,
    /// Documentation browser or docs pane (re-readable).
    Docs,
    /// Explorer tree (re-readable).
    Explorer,
    /// Terminal pane (session-scoped live authority).
    Terminal,
    /// Debugger pane (session-scoped live authority).
    Debugger,
    /// Notebook pane (session-scoped live authority).
    Notebook,
    /// Query-console pane (session-scoped live authority).
    QueryConsole,
    /// Preview canvas (session-scoped live authority).
    Preview,
    /// Profiler capture or replay pane (session-scoped live authority).
    ProfilerCapture,
    /// Remote-backed surface (session-scoped live authority).
    RemoteBacked,
    /// AI panel (session-scoped live authority).
    AiPanel,
    /// Test runner pane (session-scoped live authority).
    Test,
    /// Pipeline / job pane (session-scoped live authority).
    Pipeline,
    /// Incident workspace pane (session-scoped live authority).
    IncidentWorkspace,
    /// Extension-contributed pane (may be missing on restore).
    CustomExtension,
}

impl PaneSurfaceClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Search => "search",
            Self::Problems => "problems",
            Self::Scm => "scm",
            Self::Docs => "docs",
            Self::Explorer => "explorer",
            Self::Terminal => "terminal",
            Self::Debugger => "debugger",
            Self::Notebook => "notebook",
            Self::QueryConsole => "query_console",
            Self::Preview => "preview",
            Self::ProfilerCapture => "profiler_capture",
            Self::RemoteBacked => "remote_backed",
            Self::AiPanel => "ai_panel",
            Self::Test => "test",
            Self::Pipeline => "pipeline",
            Self::IncidentWorkspace => "incident_workspace",
            Self::CustomExtension => "custom_extension",
        }
    }

    /// Whether this surface owns session-scoped live authority that must never
    /// be silently reacquired on restore.
    pub const fn is_session_scoped(self) -> bool {
        matches!(
            self,
            Self::Terminal
                | Self::Debugger
                | Self::Notebook
                | Self::QueryConsole
                | Self::Preview
                | Self::ProfilerCapture
                | Self::RemoteBacked
                | Self::AiPanel
                | Self::Test
                | Self::Pipeline
                | Self::IncidentWorkspace
        )
    }
}

/// Where a pane landed in the skeleton-first / hydrate-second restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneHydrationClass {
    /// The structural pane shell was recreated; hydration has not run yet.
    SkeletonRecreated,
    /// A re-readable surface hydrated truthfully from durable state with no side
    /// effect.
    HydratedLive,
    /// The pane slot is preserved by an in-place placeholder card; live
    /// authority was not reacquired.
    PlaceholderCard,
}

impl PaneHydrationClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SkeletonRecreated => "skeleton_recreated",
            Self::HydratedLive => "hydrated_live",
            Self::PlaceholderCard => "placeholder_card",
        }
    }
}

/// The restore-no-rerun state a placeholder card discloses in the pane slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanePlaceholderState {
    /// A transcript or snapshot was restored; the command was not rerun.
    TranscriptRestored,
    /// The prior session ended and was not reattached.
    SessionEnded,
    /// A reconnect is available but live authority has not resumed.
    ReconnectAvailable,
    /// An explicit rerun is required before live output resumes.
    RerunRequired,
    /// The context behind the pane is unavailable (e.g. missing extension).
    ContextUnavailable,
}

impl PanePlaceholderState {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptRestored => "transcript_restored",
            Self::SessionEnded => "session_ended",
            Self::ReconnectAvailable => "reconnect_available",
            Self::RerunRequired => "rerun_required",
            Self::ContextUnavailable => "context_unavailable",
        }
    }

    /// Controlled user-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::TranscriptRestored => "transcript restored; command not rerun",
            Self::SessionEnded => "session ended; command not rerun",
            Self::ReconnectAvailable => "reconnect available",
            Self::RerunRequired => "rerun required",
            Self::ContextUnavailable => "context unavailable",
        }
    }
}

/// Why a pane could not hydrate live and fell back to a placeholder card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneSubstitutionReason {
    /// The contributing extension is absent.
    MissingExtension,
    /// A remote session expired and was not reattached.
    ExpiredRemoteSession,
    /// A managed (admin-provisioned) session expired.
    ExpiredManagedSession,
    /// The display topology no longer supports the pane.
    UnsupportedDisplayTopology,
    /// A permission or grant was revoked.
    RevokedPermission,
    /// The underlying runtime did not survive the restart.
    RuntimeDidNotSurvive,
}

impl PaneSubstitutionReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingExtension => "missing_extension",
            Self::ExpiredRemoteSession => "expired_remote_session",
            Self::ExpiredManagedSession => "expired_managed_session",
            Self::UnsupportedDisplayTopology => "unsupported_display_topology",
            Self::RevokedPermission => "revoked_permission",
            Self::RuntimeDidNotSurvive => "runtime_did_not_survive",
        }
    }
}

/// Closed recovery action vocabulary offered inside a pane placeholder card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneRecoveryAction {
    /// Reconnect the remote / managed session.
    ReconnectSession,
    /// Rerun the command explicitly.
    RerunExplicitly,
    /// Install the missing extension.
    InstallExtension,
    /// Open without the missing dependency.
    OpenWithout,
    /// Compare with the preserved evidence.
    CompareEvidence,
    /// Export the retained evidence.
    ExportEvidence,
    /// Remove the pane deliberately.
    RemovePane,
}

impl PaneRecoveryAction {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconnectSession => "reconnect_session",
            Self::RerunExplicitly => "rerun_explicitly",
            Self::InstallExtension => "install_extension",
            Self::OpenWithout => "open_without",
            Self::CompareEvidence => "compare_evidence",
            Self::ExportEvidence => "export_evidence",
            Self::RemovePane => "remove_pane",
        }
    }
}

/// One restored pane slot and its honest hydration state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneSlot {
    /// Stable pane id (window-local, survives split / move / restore).
    pub pane_id: String,
    /// Surface class the pane carries.
    pub surface_class: PaneSurfaceClass,
    /// Best-effort title hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
    /// Where the pane landed in skeleton-first / hydrate-second restore.
    pub hydration: PaneHydrationClass,
    /// Placeholder restore-no-rerun state, when the pane is placeholder-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_state: Option<PanePlaceholderState>,
    /// Why the pane fell back to a placeholder, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitution_reason: Option<PaneSubstitutionReason>,
    /// Whether the underlying runtime survived the restart.
    pub runtime_survived: bool,
    /// Whether command rerun is forbidden until an explicit user action.
    pub command_rerun_forbidden: bool,
    /// Whether authority reacquire is forbidden until an explicit user action.
    pub authority_reacquire_forbidden: bool,
    /// Canonical reopen target this pane resolves to.
    pub reopen_target_ref: String,
    /// Opaque evidence ref retained for compare / export, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Recovery actions offered in the pane (non-empty for placeholder cards).
    pub recovery_actions: Vec<PaneRecoveryAction>,
    /// Reviewable note rendered in the slot and support export.
    pub note: String,
}

impl PaneSlot {
    /// Whether this slot is preserved by an in-place placeholder card.
    pub fn is_placeholder(&self) -> bool {
        self.hydration == PaneHydrationClass::PlaceholderCard
    }
}

// ---------------------------------------------------------------------------
// Versioned pane tree
// ---------------------------------------------------------------------------

/// A node in the versioned pane tree: a leaf pane or a split of two subtrees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "node_kind", rename_all = "snake_case")]
pub enum PaneTreeNode {
    /// A leaf referencing a stable pane id.
    Leaf {
        /// Stable pane id of the leaf.
        pane_id: String,
    },
    /// A split with an axis, weights, and two subtrees.
    Split {
        /// Split axis.
        axis: SplitAxisClass,
        /// First-side weight (must be positive).
        first_weight: u16,
        /// Second-side weight (must be positive).
        second_weight: u16,
        /// First subtree.
        first: Box<PaneTreeNode>,
        /// Second subtree.
        second: Box<PaneTreeNode>,
    },
}

impl PaneTreeNode {
    /// Collects the leaf pane ids in left-to-right order.
    pub fn leaf_ids(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_leaf_ids(&mut out);
        out
    }

    fn collect_leaf_ids(&self, out: &mut Vec<String>) {
        match self {
            Self::Leaf { pane_id } => out.push(pane_id.clone()),
            Self::Split { first, second, .. } => {
                first.collect_leaf_ids(out);
                second.collect_leaf_ids(out);
            }
        }
    }

    /// Returns the smallest weight found anywhere in the tree, if any split
    /// carries a zero weight.
    fn has_zero_weight(&self) -> bool {
        match self {
            Self::Leaf { .. } => false,
            Self::Split {
                first_weight,
                second_weight,
                first,
                second,
                ..
            } => {
                *first_weight == 0
                    || *second_weight == 0
                    || first.has_zero_weight()
                    || second.has_zero_weight()
            }
        }
    }
}

/// The versioned, stable-id pane tree and its leaf pane slots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneTree {
    /// Versioned pane-tree schema version.
    pub pane_tree_schema_version: u32,
    /// Root of the pane tree.
    pub root: PaneTreeNode,
    /// Leaf pane slots, sorted by stable pane id.
    pub slots: Vec<PaneSlot>,
}

// ---------------------------------------------------------------------------
// Restore fidelity + provenance
// ---------------------------------------------------------------------------

/// Closed restore fidelity class admitted on a reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Layout, focus, bounds, and live surfaces restored exactly.
    Exact,
    /// Layout intent preserved; bounds or scale adjusted, panes hydrate live.
    Compatible,
    /// Structure preserved; live surfaces replaced by placeholder cards.
    LayoutOnly,
    /// Recovery via placeholder cards and substitutions for absent dependencies.
    PlaceholderBacked,
}

impl RestoreFidelityClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::PlaceholderBacked => "placeholder_backed",
        }
    }

    /// Controlled user-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Exact => "Exact restore",
            Self::Compatible => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::PlaceholderBacked => "Placeholder-backed recovery",
        }
    }

    /// Fidelity rank where a higher number is a deeper downgrade.
    const fn rank(self) -> u8 {
        match self {
            Self::Exact => 0,
            Self::Compatible => 1,
            Self::LayoutOnly => 2,
            Self::PlaceholderBacked => 3,
        }
    }
}

/// Why a reopen downgraded from a higher fidelity to the resulting fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDowngradeReason {
    /// The display topology changed (monitor removed, scale, docking, wake).
    DisplayTopologyChange,
    /// A missing extension was substituted by a placeholder.
    MissingExtensionSubstitution,
    /// A managed session expired.
    ExpiredManagedSession,
    /// A remote session expired.
    ExpiredRemoteSession,
    /// The display topology is unsupported for an exact reopen.
    UnsupportedDisplayTopology,
    /// A session-scoped runtime did not survive the restart.
    RuntimeDidNotSurvive,
    /// A schema translation was required to load the layout.
    SchemaTranslationRequired,
}

impl RestoreDowngradeReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayTopologyChange => "display_topology_change",
            Self::MissingExtensionSubstitution => "missing_extension_substitution",
            Self::ExpiredManagedSession => "expired_managed_session",
            Self::ExpiredRemoteSession => "expired_remote_session",
            Self::UnsupportedDisplayTopology => "unsupported_display_topology",
            Self::RuntimeDidNotSurvive => "runtime_did_not_survive",
            Self::SchemaTranslationRequired => "schema_translation_required",
        }
    }
}

/// One restore downgrade from a higher fidelity to the resulting fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDowngrade {
    /// Fidelity the reopen would have had without the downgrade.
    pub from_fidelity: RestoreFidelityClass,
    /// Fidelity the reopen actually achieved.
    pub to_fidelity: RestoreFidelityClass,
    /// Reason for the downgrade.
    pub reason: RestoreDowngradeReason,
    /// Reviewable note.
    pub note: String,
}

/// Display-topology change provenance attached to the reopen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DisplayTopologyProvenance {
    /// Topology change classes detected at restore time, sorted.
    pub topology_change_classes: Vec<TopologyChangeClass>,
    /// Adjustments applied to reconcile the topology change, in applied order.
    pub adjustments: Vec<RestoreAdjustmentClass>,
    /// Whether a user-visible "layout adjusted" note is required.
    pub layout_adjusted_note_required: bool,
}

/// Restore fidelity and downgrade provenance for the reopen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenance {
    /// Canonical restore-provenance ref shared with the support export.
    pub restore_provenance_ref: String,
    /// Resulting fidelity admitted to the user.
    pub resulting_fidelity: RestoreFidelityClass,
    /// Downgrade record, when the reopen was not exact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade: Option<RestoreDowngrade>,
    /// Compare handle preserved for downgraded reopens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_ref: Option<String>,
    /// Export handle preserved for downgraded reopens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_ref: Option<String>,
    /// Reviewable summary.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Recovery chrome + surfaces
// ---------------------------------------------------------------------------

/// Recovery-critical chrome that must stay reachable after a restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryChromeAssurance {
    /// Title context / breadcrumb path remains visible.
    pub title_context_visible: bool,
    /// Restore details remain reachable.
    pub restore_details_reachable: bool,
    /// Command palette remains reachable via keyboard.
    pub command_palette_reachable: bool,
    /// Keyboard focus order remains reachable.
    pub keyboard_focus_reachable: bool,
    /// Activity center / status strip remains visible.
    pub activity_center_visible: bool,
}

impl RecoveryChromeAssurance {
    /// Returns true when every assurance is satisfied.
    pub const fn all_satisfied(&self) -> bool {
        self.title_context_visible
            && self.restore_details_reachable
            && self.command_palette_reachable
            && self.keyboard_focus_reachable
            && self.activity_center_visible
    }
}

/// A surface bound to the shared restore record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTruthSurface {
    /// The desktop restore-review surface.
    DesktopRestoreReview,
    /// The CLI restore inspector.
    CliInspect,
    /// The Help/About restore summary.
    HelpAbout,
    /// The diagnostics support export.
    DiagnosticsSupportExport,
}

impl RestoreTruthSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopRestoreReview => "desktop_restore_review",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::DiagnosticsSupportExport => "diagnostics_support_export",
        }
    }

    /// Every surface a reopen must bind, in canonical order.
    pub const REQUIRED: [Self; 4] = [
        Self::DesktopRestoreReview,
        Self::CliInspect,
        Self::HelpAbout,
        Self::DiagnosticsSupportExport,
    ];
}

/// One surface's binding to the shared record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreSurfaceProjectionInput {
    /// Surface this projection describes.
    pub surface: RestoreTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloned prose.
    pub reads_shared_record: bool,
}

/// One surface's reconciled binding to the shared record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreSurfaceProjection {
    /// Surface this projection describes.
    pub surface: RestoreTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloned prose.
    pub reads_shared_record: bool,
    /// Deterministic one-line summary shown on the surface.
    pub summary_line: String,
}

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed window-level recovery-action vocabulary on a reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreRecoveryAction {
    /// Open the restore-details surface.
    OpenRestoreDetails,
    /// Reconnect an expired remote / managed session.
    ReconnectSession,
    /// Rerun a session-scoped pane explicitly.
    RerunExplicitly,
    /// Install a missing extension.
    InstallMissingExtension,
    /// Compare the reopen with the preserved evidence.
    CompareWithEvidence,
    /// Export a redacted restore-support packet.
    ExportRestoreSupport,
}

impl WindowRestoreRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRestoreDetails => "open_restore_details",
            Self::ReconnectSession => "reconnect_session",
            Self::RerunExplicitly => "rerun_explicitly",
            Self::InstallMissingExtension => "install_missing_extension",
            Self::CompareWithEvidence => "compare_with_evidence",
            Self::ExportRestoreSupport => "export_restore_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenRestoreDetails => "Open restore details",
            Self::ReconnectSession => "Reconnect session",
            Self::RerunExplicitly => "Rerun explicitly",
            Self::InstallMissingExtension => "Install missing extension",
            Self::CompareWithEvidence => "Compare with evidence",
            Self::ExportRestoreSupport => "Export restore support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenRestoreDetails => RecoveryActionRole::Primary,
            Self::ReconnectSession | Self::RerunExplicitly | Self::InstallMissingExtension => {
                RecoveryActionRole::Recovery
            }
            Self::CompareWithEvidence | Self::ExportRestoreSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every reopen must expose.
    pub const REQUIRED: [Self; 3] = [
        Self::OpenRestoreDetails,
        Self::CompareWithEvidence,
        Self::ExportRestoreSupport,
    ];

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery routes a reopen exposes, in rendered order, given which
/// conditional recovery affordances the panes require.
pub fn required_recovery_routes(
    reconnect: bool,
    rerun: bool,
    install: bool,
) -> Vec<RecoveryRouteRecord> {
    let mut actions = vec![WindowRestoreRecoveryAction::OpenRestoreDetails];
    if reconnect {
        actions.push(WindowRestoreRecoveryAction::ReconnectSession);
    }
    if rerun {
        actions.push(WindowRestoreRecoveryAction::RerunExplicitly);
    }
    if install {
        actions.push(WindowRestoreRecoveryAction::InstallMissingExtension);
    }
    actions.push(WindowRestoreRecoveryAction::CompareWithEvidence);
    actions.push(WindowRestoreRecoveryAction::ExportRestoreSupport);
    actions.into_iter().map(|a| a.route()).collect()
}

// ---------------------------------------------------------------------------
// Claim ceiling + pillars + qualification
// ---------------------------------------------------------------------------

/// The public claim ceiling: what a reopen is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WindowRestoreClaimCeiling {
    /// May claim workspace authority and window topology are separated.
    pub asserts_authority_topology_separated: bool,
    /// May claim the pane tree is versioned with stable pane IDs.
    pub asserts_pane_tree_versioned: bool,
    /// May claim restore is skeleton-first then hydrate-second.
    pub asserts_skeleton_first_hydrate_second: bool,
    /// May claim no session-scoped pane silently reruns or reacquires authority.
    pub asserts_no_silent_rerun: bool,
    /// May claim restore provenance is export-safe.
    pub asserts_provenance_export_safe: bool,
    /// May claim recovery-critical chrome stays reachable.
    pub asserts_recovery_chrome_reachable: bool,
}

/// The derived pillar verdicts (what the reopen can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestorePillars {
    /// Workspace authority and window topology are separated.
    pub authority_topology_separated: bool,
    /// The pane tree is versioned with stable pane IDs.
    pub pane_tree_versioned_stable_ids: bool,
    /// Restore is skeleton-first then hydrate-second.
    pub skeleton_first_hydrate_second: bool,
    /// No session-scoped pane silently reruns or reacquires authority.
    pub no_silent_rerun_or_reacquire: bool,
    /// Restore provenance is export-safe.
    pub restore_provenance_export_safe: bool,
    /// Recovery-critical chrome stays reachable.
    pub recovery_chrome_reachable: bool,
}

/// Reason a reopen is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRestoreNarrowingReason {
    /// Workspace authority and window topology are not separated.
    AuthorityTopologyNotSeparated,
    /// The pane tree is not versioned or its ids are not stable.
    PaneTreeNotVersionedStable,
    /// Restore is not skeleton-first / hydrate-second.
    HydrationNotSkeletonFirst,
    /// A session-scoped pane could silently rerun or reacquire authority.
    SilentRerunPossible,
    /// Restore provenance is not export-safe.
    ProvenanceNotExportSafe,
    /// Recovery-critical chrome is not reachable.
    RecoveryChromeNotReachable,
    /// The lowest binding surface marker is below Stable.
    SurfaceNotYetStable,
}

impl WindowRestoreNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityTopologyNotSeparated => "authority_topology_not_separated",
            Self::PaneTreeNotVersionedStable => "pane_tree_not_versioned_stable",
            Self::HydrationNotSkeletonFirst => "hydration_not_skeleton_first",
            Self::SilentRerunPossible => "silent_rerun_possible",
            Self::ProvenanceNotExportSafe => "provenance_not_export_safe",
            Self::RecoveryChromeNotReachable => "recovery_chrome_not_reachable",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

/// The derived stable-claim verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreQualification {
    /// The derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the reopen qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// Reasons the reopen is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<WindowRestoreNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreUpstream {
    /// Windows workspace-management page ref this reopen projects from.
    pub windows_page_ref: String,
    /// Restore-provenance record-kind ref this reopen projects from.
    pub restore_provenance_kind_ref: String,
    /// Contributing case ids, sorted and deduped.
    pub contributing_case_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`WindowTopologyRestoreRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowTopologyRestoreInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token (the reopen scenario).
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Window id this reopen applies to.
    pub window_id: String,
    /// Window role.
    pub window_role: WindowRoleClass,
    /// Authority / topology separation.
    pub authority: AuthoritySeparation,
    /// Versioned pane tree.
    pub pane_tree: PaneTree,
    /// Display-topology provenance.
    pub display_topology: DisplayTopologyProvenance,
    /// Restore fidelity and downgrade provenance.
    pub restore_provenance: RestoreProvenance,
    /// Recovery-critical chrome assurance.
    pub recovery_chrome: RecoveryChromeAssurance,
    /// Per-surface bindings.
    pub surface_projections: Vec<RestoreSurfaceProjectionInput>,
    /// Public claim ceiling.
    pub claim_ceiling: WindowRestoreClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the reopen.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the reopen stays available without an account.
    pub available_without_account: bool,
    /// Whether the reopen stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: WindowRestoreUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed window-topology restore record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowTopologyRestoreRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Window id this reopen applies to.
    pub window_id: String,
    /// Window role.
    pub window_role: WindowRoleClass,
    /// The lowest binding surface marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Authority / topology separation.
    pub authority: AuthoritySeparation,
    /// Versioned pane tree.
    pub pane_tree: PaneTree,
    /// Display-topology provenance.
    pub display_topology: DisplayTopologyProvenance,
    /// Restore fidelity and downgrade provenance.
    pub restore_provenance: RestoreProvenance,
    /// Recovery-critical chrome assurance.
    pub recovery_chrome: RecoveryChromeAssurance,
    /// Per-surface bindings, in canonical order.
    pub surface_projections: Vec<RestoreSurfaceProjection>,
    /// The derived pillar verdicts.
    pub pillars: WindowRestorePillars,
    /// The public claim ceiling.
    pub claim_ceiling: WindowRestoreClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: WindowRestoreQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the reopen.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the reopen stays available without an account.
    pub available_without_account: bool,
    /// Whether the reopen stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: WindowRestoreUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`WindowTopologyRestoreRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// A required workspace-authority class was missing.
    MissingWorkspaceAuthorityClass { class: WorkspaceAuthorityClass },
    /// A required window-local topology class was missing.
    MissingWindowLocalTopologyClass { class: WindowLocalTopologyClass },
    /// The pane-tree schema version was absent (zero).
    PaneTreeSchemaAbsent,
    /// A pane-tree split carried a zero weight.
    PaneTreeZeroWeight,
    /// A pane-tree leaf referenced a pane id with no slot.
    PaneTreeLeafWithoutSlot { pane_id: String },
    /// A pane slot was not referenced by any leaf.
    PaneSlotNotInTree { pane_id: String },
    /// Two pane slots shared a stable pane id.
    DuplicatePaneId { pane_id: String },
    /// A session-scoped pane hydrated live, reacquiring authority silently.
    SessionPaneReacquiredLiveAuthority { pane_id: String },
    /// A placeholder pane did not forbid command rerun and authority reacquire.
    PlaceholderMissingNoRerunGuards { pane_id: String },
    /// A placeholder pane carried no placeholder state.
    PlaceholderMissingState { pane_id: String },
    /// A placeholder pane offered no recovery action.
    PlaceholderMissingRecovery { pane_id: String },
    /// A placeholder pane claimed the runtime survived.
    PlaceholderClaimsRuntimeSurvived { pane_id: String },
    /// An exact reopen carried a downgrade, placeholder, or adjustment.
    ExactRestoreHasDowngrade,
    /// A non-exact reopen carried no downgrade record.
    NonExactWithoutDowngrade,
    /// A reopen with placeholder panes claimed Exact or Compatible fidelity.
    PlaceholderFidelityTooHigh,
    /// A downgrade did not actually lower fidelity.
    DowngradeNotLower,
    /// A downgrade's target fidelity did not match the resulting fidelity.
    DowngradeFidelityMismatch,
    /// A required restore surface binding was missing.
    SurfaceProjectionMissing { surface: RestoreTruthSurface },
    /// A restore surface binding was duplicated.
    DuplicateSurfaceProjection { surface: RestoreTruthSurface },
    /// A surface cloned prose instead of reading the shared record.
    SurfaceClonesProse { surface: RestoreTruthSurface },
    /// The claim ceiling asserted authority/topology separation it cannot prove.
    OverclaimsAuthorityTopology,
    /// The claim ceiling asserted a versioned pane tree it cannot prove.
    OverclaimsPaneTreeVersioned,
    /// The claim ceiling asserted skeleton-first hydration it cannot prove.
    OverclaimsSkeletonFirst,
    /// The claim ceiling asserted no-silent-rerun it cannot prove.
    OverclaimsNoSilentRerun,
    /// The claim ceiling asserted export-safe provenance it cannot prove.
    OverclaimsProvenanceExportSafe,
    /// The claim ceiling asserted recovery-chrome reachability it cannot prove.
    OverclaimsRecoveryChrome,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: WindowRestoreRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: AttentionRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: AttentionRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: AttentionRouteSurface },
    /// An entry route did not activate the same reopen.
    RouteTargetsDifferentItem { surface: AttentionRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A reopen was hidden when no account was present.
    HiddenWithoutAccount,
    /// A reopen was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::MissingRef { field } => write!(f, "ref `{field}` must be present"),
            Self::MissingWorkspaceAuthorityClass { class } => write!(
                f,
                "authority separation must enumerate workspace-authority class `{}`",
                class.as_str()
            ),
            Self::MissingWindowLocalTopologyClass { class } => write!(
                f,
                "authority separation must enumerate window-local topology class `{}`",
                class.as_str()
            ),
            Self::PaneTreeSchemaAbsent => {
                write!(f, "pane tree must carry a versioned schema (got 0)")
            }
            Self::PaneTreeZeroWeight => write!(f, "pane tree split used a zero weight"),
            Self::PaneTreeLeafWithoutSlot { pane_id } => {
                write!(f, "pane tree leaf {pane_id:?} has no matching slot")
            }
            Self::PaneSlotNotInTree { pane_id } => {
                write!(
                    f,
                    "pane slot {pane_id:?} is not referenced by the pane tree"
                )
            }
            Self::DuplicatePaneId { pane_id } => {
                write!(f, "pane id {pane_id:?} is used by more than one slot")
            }
            Self::SessionPaneReacquiredLiveAuthority { pane_id } => write!(
                f,
                "session-scoped pane {pane_id:?} hydrated live, silently reacquiring authority"
            ),
            Self::PlaceholderMissingNoRerunGuards { pane_id } => write!(
                f,
                "placeholder pane {pane_id:?} must forbid command rerun and authority reacquire"
            ),
            Self::PlaceholderMissingState { pane_id } => {
                write!(
                    f,
                    "placeholder pane {pane_id:?} must carry a placeholder state"
                )
            }
            Self::PlaceholderMissingRecovery { pane_id } => {
                write!(
                    f,
                    "placeholder pane {pane_id:?} must offer a recovery action"
                )
            }
            Self::PlaceholderClaimsRuntimeSurvived { pane_id } => write!(
                f,
                "placeholder pane {pane_id:?} must not claim its runtime survived"
            ),
            Self::ExactRestoreHasDowngrade => {
                write!(
                    f,
                    "exact restore cannot carry a downgrade, placeholder, or adjustment"
                )
            }
            Self::NonExactWithoutDowngrade => {
                write!(f, "a non-exact reopen must carry a downgrade record")
            }
            Self::PlaceholderFidelityTooHigh => write!(
                f,
                "a reopen with placeholder panes must be layout-only or placeholder-backed"
            ),
            Self::DowngradeNotLower => write!(f, "a downgrade must actually lower fidelity"),
            Self::DowngradeFidelityMismatch => {
                write!(f, "a downgrade target must match the resulting fidelity")
            }
            Self::SurfaceProjectionMissing { surface } => {
                write!(f, "restore surface `{}` is missing", surface.as_str())
            }
            Self::DuplicateSurfaceProjection { surface } => {
                write!(f, "restore surface `{}` is duplicated", surface.as_str())
            }
            Self::SurfaceClonesProse { surface } => write!(
                f,
                "restore surface `{}` must read the shared record, not cloned prose",
                surface.as_str()
            ),
            Self::OverclaimsAuthorityTopology => write!(
                f,
                "claim ceiling may not assert authority/topology separation it cannot prove"
            ),
            Self::OverclaimsPaneTreeVersioned => write!(
                f,
                "claim ceiling may not assert a versioned pane tree it cannot prove"
            ),
            Self::OverclaimsSkeletonFirst => write!(
                f,
                "claim ceiling may not assert skeleton-first hydration it cannot prove"
            ),
            Self::OverclaimsNoSilentRerun => write!(
                f,
                "claim ceiling may not assert no-silent-rerun it cannot prove"
            ),
            Self::OverclaimsProvenanceExportSafe => write!(
                f,
                "claim ceiling may not assert export-safe provenance it cannot prove"
            ),
            Self::OverclaimsRecoveryChrome => write!(
                f,
                "claim ceiling may not assert recovery-chrome reachability it cannot prove"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(f, "reopen must expose recovery route `{}`", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(
                    f,
                    "entry route surface `{}` is duplicated",
                    surface.as_str()
                )
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentItem { surface } => write!(
                f,
                "entry route surface `{}` must activate the same reopen",
                surface.as_str()
            ),
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(
                    f,
                    "accessibility layout mode `{}` is missing",
                    mode.as_str()
                )
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::HiddenWithoutAccount => {
                write!(f, "a reopen must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => {
                write!(f, "a reopen must stay available without managed services")
            }
        }
    }
}

impl std::error::Error for BuildError {}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingRef { field })
    }
}

impl WindowTopologyRestoreRecord {
    /// Builds a governed window-topology restore record from validated input.
    ///
    /// Structural lies — a session-scoped pane silently reacquiring authority, a
    /// placeholder pane that fails to forbid rerun, an exact reopen carrying a
    /// downgrade, a surface that clones prose — are rejected outright. Provable-
    /// but-imperfect reopens (workspace authority fused with topology, recovery
    /// chrome unreachable, a below-Stable binding surface) are minted but
    /// narrowed below Stable with a named reason.
    pub fn build(input: WindowTopologyRestoreInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        for (field, value) in [
            ("title", &input.title),
            ("summary", &input.summary),
            ("posture_label", &input.posture_label),
            (
                "restore_provenance.summary",
                &input.restore_provenance.summary,
            ),
        ] {
            if !is_reviewable_sentence(value) {
                return Err(BuildError::InvalidSentence { field });
            }
        }
        require_present_ref("window_id", &input.window_id)?;
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        require_canonical_ref(
            "authority.workspace_authority_ref",
            &input.authority.workspace_authority_ref,
        )?;
        require_canonical_ref(
            "restore_provenance.restore_provenance_ref",
            &input.restore_provenance.restore_provenance_ref,
        )?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref(
            "upstream.windows_page_ref",
            &input.upstream.windows_page_ref,
        )?;
        require_present_ref(
            "upstream.restore_provenance_kind_ref",
            &input.upstream.restore_provenance_kind_ref,
        )?;

        // --- authority / topology separation ---------------------------------
        let authority_classes: BTreeSet<WorkspaceAuthorityClass> = input
            .authority
            .workspace_authority_classes
            .iter()
            .copied()
            .collect();
        for required in WorkspaceAuthorityClass::REQUIRED {
            if !authority_classes.contains(&required) {
                return Err(BuildError::MissingWorkspaceAuthorityClass { class: required });
            }
        }
        let topology_classes: BTreeSet<WindowLocalTopologyClass> = input
            .authority
            .window_local_topology_classes
            .iter()
            .copied()
            .collect();
        for required in WindowLocalTopologyClass::REQUIRED {
            if !topology_classes.contains(&required) {
                return Err(BuildError::MissingWindowLocalTopologyClass { class: required });
            }
        }
        let authority_topology_separated = input.authority.workspace_authority_centralized
            && input.authority.topology_window_local;

        // --- pane tree -------------------------------------------------------
        if input.pane_tree.pane_tree_schema_version == 0 {
            return Err(BuildError::PaneTreeSchemaAbsent);
        }
        if input.pane_tree.root.has_zero_weight() {
            return Err(BuildError::PaneTreeZeroWeight);
        }
        let mut slot_ids: BTreeSet<String> = BTreeSet::new();
        for slot in &input.pane_tree.slots {
            require_present_ref("pane_tree.slots.pane_id", &slot.pane_id)?;
            if !slot_ids.insert(slot.pane_id.clone()) {
                return Err(BuildError::DuplicatePaneId {
                    pane_id: slot.pane_id.clone(),
                });
            }
        }
        let leaf_ids = input.pane_tree.root.leaf_ids();
        let leaf_id_set: BTreeSet<String> = leaf_ids.iter().cloned().collect();
        for leaf in &leaf_id_set {
            if !slot_ids.contains(leaf) {
                return Err(BuildError::PaneTreeLeafWithoutSlot {
                    pane_id: leaf.clone(),
                });
            }
        }
        for slot in &slot_ids {
            if !leaf_id_set.contains(slot) {
                return Err(BuildError::PaneSlotNotInTree {
                    pane_id: slot.clone(),
                });
            }
        }
        let pane_tree_versioned_stable_ids =
            input.pane_tree.pane_tree_schema_version == PANE_TREE_SCHEMA_VERSION;

        // --- pane slots: skeleton-first / hydrate-second + no silent rerun ---
        let mut any_placeholder = false;
        for slot in &input.pane_tree.slots {
            require_canonical_ref("pane_tree.slots.reopen_target_ref", &slot.reopen_target_ref)?;
            if !is_reviewable_sentence(&slot.note) {
                return Err(BuildError::InvalidSentence {
                    field: "pane_tree.slots.note",
                });
            }
            if slot.surface_class.is_session_scoped()
                && slot.hydration == PaneHydrationClass::HydratedLive
            {
                return Err(BuildError::SessionPaneReacquiredLiveAuthority {
                    pane_id: slot.pane_id.clone(),
                });
            }
            if slot.is_placeholder() {
                any_placeholder = true;
                if slot.placeholder_state.is_none() {
                    return Err(BuildError::PlaceholderMissingState {
                        pane_id: slot.pane_id.clone(),
                    });
                }
                if !slot.command_rerun_forbidden || !slot.authority_reacquire_forbidden {
                    return Err(BuildError::PlaceholderMissingNoRerunGuards {
                        pane_id: slot.pane_id.clone(),
                    });
                }
                if slot.recovery_actions.is_empty() {
                    return Err(BuildError::PlaceholderMissingRecovery {
                        pane_id: slot.pane_id.clone(),
                    });
                }
                if slot.runtime_survived {
                    return Err(BuildError::PlaceholderClaimsRuntimeSurvived {
                        pane_id: slot.pane_id.clone(),
                    });
                }
            }
        }
        // Every restore starts from a recreated skeleton; no session-scoped pane
        // hydrated live (hard-errored above), so these pillars hold by
        // construction on a successfully built record.
        let skeleton_first_hydrate_second = true;
        let no_silent_rerun_or_reacquire = true;

        // --- restore provenance ----------------------------------------------
        let fidelity = input.restore_provenance.resulting_fidelity;
        let has_adjustments = !input.display_topology.adjustments.is_empty()
            || !input.display_topology.topology_change_classes.is_empty();
        match fidelity {
            RestoreFidelityClass::Exact => {
                if input.restore_provenance.downgrade.is_some()
                    || any_placeholder
                    || has_adjustments
                {
                    return Err(BuildError::ExactRestoreHasDowngrade);
                }
            }
            RestoreFidelityClass::Compatible => {
                if any_placeholder {
                    return Err(BuildError::PlaceholderFidelityTooHigh);
                }
                validate_downgrade(&input.restore_provenance, fidelity)?;
            }
            RestoreFidelityClass::LayoutOnly | RestoreFidelityClass::PlaceholderBacked => {
                validate_downgrade(&input.restore_provenance, fidelity)?;
            }
        }
        let restore_provenance_export_safe = match fidelity {
            RestoreFidelityClass::Exact => true,
            _ => {
                is_present_ref(
                    input
                        .restore_provenance
                        .compare_ref
                        .as_deref()
                        .unwrap_or(""),
                ) && is_present_ref(input.restore_provenance.export_ref.as_deref().unwrap_or(""))
            }
        };

        // --- recovery chrome -------------------------------------------------
        let recovery_chrome_reachable = input.recovery_chrome.all_satisfied();

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<RestoreTruthSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
            if !projection.reads_shared_record {
                return Err(BuildError::SurfaceClonesProse {
                    surface: projection.surface,
                });
            }
        }
        for required in RestoreTruthSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }
        let mut surface_projections: Vec<RestoreSurfaceProjection> = Vec::new();
        for required in RestoreTruthSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            surface_projections.push(RestoreSurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                reads_shared_record: projection.reads_shared_record,
                summary_line: surface_summary_line(required, &input, fidelity),
            });
        }
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input.claim_ceiling.asserts_authority_topology_separated && !authority_topology_separated
        {
            return Err(BuildError::OverclaimsAuthorityTopology);
        }
        if input.claim_ceiling.asserts_pane_tree_versioned && !pane_tree_versioned_stable_ids {
            return Err(BuildError::OverclaimsPaneTreeVersioned);
        }
        if input.claim_ceiling.asserts_skeleton_first_hydrate_second
            && !skeleton_first_hydrate_second
        {
            return Err(BuildError::OverclaimsSkeletonFirst);
        }
        if input.claim_ceiling.asserts_no_silent_rerun && !no_silent_rerun_or_reacquire {
            return Err(BuildError::OverclaimsNoSilentRerun);
        }
        if input.claim_ceiling.asserts_provenance_export_safe && !restore_provenance_export_safe {
            return Err(BuildError::OverclaimsProvenanceExportSafe);
        }
        if input.claim_ceiling.asserts_recovery_chrome_reachable && !recovery_chrome_reachable {
            return Err(BuildError::OverclaimsRecoveryChrome);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in WindowRestoreRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<AttentionRouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- pillars ---------------------------------------------------------
        let pillars = WindowRestorePillars {
            authority_topology_separated,
            pane_tree_versioned_stable_ids,
            skeleton_first_hydrate_second,
            no_silent_rerun_or_reacquire,
            restore_provenance_export_safe,
            recovery_chrome_reachable,
        };

        // --- normalise pane tree slots ---------------------------------------
        let mut pane_tree = input.pane_tree;
        pane_tree.slots.sort_by(|a, b| a.pane_id.cmp(&b.pane_id));

        // --- display topology normalisation ----------------------------------
        let mut display_topology = input.display_topology;
        display_topology
            .topology_change_classes
            .sort_by_key(|class| class.as_str());
        display_topology.topology_change_classes.dedup();

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !authority_topology_separated {
            narrowing_reasons.push(WindowRestoreNarrowingReason::AuthorityTopologyNotSeparated);
        }
        if !pane_tree_versioned_stable_ids {
            narrowing_reasons.push(WindowRestoreNarrowingReason::PaneTreeNotVersionedStable);
        }
        if !skeleton_first_hydrate_second {
            narrowing_reasons.push(WindowRestoreNarrowingReason::HydrationNotSkeletonFirst);
        }
        if !no_silent_rerun_or_reacquire {
            narrowing_reasons.push(WindowRestoreNarrowingReason::SilentRerunPossible);
        }
        if !restore_provenance_export_safe {
            narrowing_reasons.push(WindowRestoreNarrowingReason::ProvenanceNotExportSafe);
        }
        if !recovery_chrome_reachable {
            narrowing_reasons.push(WindowRestoreNarrowingReason::RecoveryChromeNotReachable);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(WindowRestoreNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == WindowRestoreNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = WindowRestoreQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable() || any_placeholder;

        // --- normalise upstream refs -----------------------------------------
        let mut contributing_case_refs = input.upstream.contributing_case_refs.clone();
        contributing_case_refs.sort();
        contributing_case_refs.dedup();

        Ok(Self {
            record_kind: WINDOW_TOPOLOGY_RESTORE_RECORD_KIND.to_string(),
            schema_version: WINDOW_TOPOLOGY_RESTORE_SCHEMA_VERSION,
            notice: WINDOW_TOPOLOGY_RESTORE_NOTICE.to_string(),
            shared_contract_ref: WINDOW_TOPOLOGY_RESTORE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            window_id: input.window_id,
            window_role: input.window_role,
            surface_lifecycle_marker,
            authority: input.authority,
            pane_tree,
            display_topology,
            restore_provenance: input.restore_provenance,
            recovery_chrome: input.recovery_chrome,
            surface_projections,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: WindowRestoreUpstream {
                windows_page_ref: input.upstream.windows_page_ref,
                restore_provenance_kind_ref: input.upstream.restore_provenance_kind_ref,
                contributing_case_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns the count of placeholder-backed pane slots.
    pub fn placeholder_pane_count(&self) -> usize {
        self.pane_tree
            .slots
            .iter()
            .filter(|slot| slot.is_placeholder())
            .count()
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("window_topology_restore: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!("window: {} role={}", self.window_id, self.window_role.as_str()),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "restore_fidelity: {} ({})",
                self.restore_provenance.resulting_fidelity.as_str(),
                self.restore_provenance.resulting_fidelity.display_label()
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: authority_topology_separated={} pane_tree_versioned={} skeleton_first={} no_silent_rerun={} provenance_export_safe={} recovery_chrome_reachable={}",
                self.pillars.authority_topology_separated,
                self.pillars.pane_tree_versioned_stable_ids,
                self.pillars.skeleton_first_hydrate_second,
                self.pillars.no_silent_rerun_or_reacquire,
                self.pillars.restore_provenance_export_safe,
                self.pillars.recovery_chrome_reachable
            ),
            format!(
                "authority: workspace_authority_ref={} centralized={} window_local_topology={}",
                self.authority.workspace_authority_ref,
                self.authority.workspace_authority_centralized,
                self.authority.topology_window_local
            ),
            format!(
                "pane_tree: schema_version={} leaves={}",
                self.pane_tree.pane_tree_schema_version,
                self.pane_tree.root.leaf_ids().len()
            ),
        ];
        if let Some(downgrade) = &self.restore_provenance.downgrade {
            lines.push(format!(
                "downgrade: {} -> {} reason={} :: {}",
                downgrade.from_fidelity.as_str(),
                downgrade.to_fidelity.as_str(),
                downgrade.reason.as_str(),
                downgrade.note
            ));
        }
        lines.push(format!(
            "display_topology: changes=[{}] adjustments=[{}] layout_adjusted_note={}",
            self.display_topology
                .topology_change_classes
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            self.display_topology
                .adjustments
                .iter()
                .map(|adjustment| adjustment.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            self.display_topology.layout_adjusted_note_required
        ));
        lines.push("pane_slots:".to_string());
        for slot in &self.pane_tree.slots {
            lines.push(format!(
                "  - {} class={} hydration={} placeholder={} substitution={} rerun_forbidden={} reacquire_forbidden={} :: {}",
                slot.pane_id,
                slot.surface_class.as_str(),
                slot.hydration.as_str(),
                slot.placeholder_state.map(|s| s.as_str()).unwrap_or("none"),
                slot.substitution_reason.map(|s| s.as_str()).unwrap_or("none"),
                slot.command_rerun_forbidden,
                slot.authority_reacquire_forbidden,
                slot.note
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} reads_shared_record={} :: {}",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.reads_shared_record,
                projection.summary_line
            ));
        }
        lines.push(format!(
            "recovery_chrome: title={} restore_details={} command_palette={} keyboard_focus={} activity_center={}",
            self.recovery_chrome.title_context_visible,
            self.recovery_chrome.restore_details_reachable,
            self.recovery_chrome.command_palette_reachable,
            self.recovery_chrome.keyboard_focus_reachable,
            self.recovery_chrome.activity_center_visible
        ));
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn validate_downgrade(
    provenance: &RestoreProvenance,
    fidelity: RestoreFidelityClass,
) -> Result<(), BuildError> {
    let Some(downgrade) = &provenance.downgrade else {
        return Err(BuildError::NonExactWithoutDowngrade);
    };
    if downgrade.from_fidelity.rank() >= downgrade.to_fidelity.rank() {
        return Err(BuildError::DowngradeNotLower);
    }
    if downgrade.to_fidelity != fidelity {
        return Err(BuildError::DowngradeFidelityMismatch);
    }
    if !is_present_ref(provenance.compare_ref.as_deref().unwrap_or(""))
        || !is_present_ref(provenance.export_ref.as_deref().unwrap_or(""))
    {
        return Err(BuildError::OverclaimsProvenanceExportSafe);
    }
    Ok(())
}

fn surface_summary_line(
    surface: RestoreTruthSurface,
    input: &WindowTopologyRestoreInput,
    fidelity: RestoreFidelityClass,
) -> String {
    let placeholder_count = input
        .pane_tree
        .slots
        .iter()
        .filter(|slot| slot.is_placeholder())
        .count();
    let prefix = match surface {
        RestoreTruthSurface::DesktopRestoreReview => "Restore review",
        RestoreTruthSurface::CliInspect => "CLI inspect",
        RestoreTruthSurface::HelpAbout => "Help/About",
        RestoreTruthSurface::DiagnosticsSupportExport => "Support export",
    };
    format!(
        "{prefix}: {} reopened {} with {} placeholder-backed pane(s).",
        input.window_id,
        fidelity.display_label(),
        placeholder_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_scoped_classification_is_correct() {
        assert!(PaneSurfaceClass::Terminal.is_session_scoped());
        assert!(PaneSurfaceClass::QueryConsole.is_session_scoped());
        assert!(PaneSurfaceClass::ProfilerCapture.is_session_scoped());
        assert!(PaneSurfaceClass::IncidentWorkspace.is_session_scoped());
        assert!(PaneSurfaceClass::RemoteBacked.is_session_scoped());
        assert!(!PaneSurfaceClass::Editor.is_session_scoped());
        assert!(!PaneSurfaceClass::Diff.is_session_scoped());
        assert!(!PaneSurfaceClass::Docs.is_session_scoped());
    }

    #[test]
    fn pane_tree_collects_leaf_ids_in_order() {
        let tree = PaneTreeNode::Split {
            axis: SplitAxisClass::Vertical,
            first_weight: 1,
            second_weight: 1,
            first: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-a".to_string(),
            }),
            second: Box::new(PaneTreeNode::Leaf {
                pane_id: "pane-b".to_string(),
            }),
        };
        assert_eq!(
            tree.leaf_ids(),
            vec!["pane-a".to_string(), "pane-b".to_string()]
        );
        assert!(!tree.has_zero_weight());
    }
}
