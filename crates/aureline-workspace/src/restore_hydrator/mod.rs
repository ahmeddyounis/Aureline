//! Restore hydrator: skeleton-first window restoration with honest degradation.
//!
//! This module owns the runtime that turns a remembered window-topology
//! snapshot into a usable layout again. It deliberately separates *state
//! serialization* (owned by [`crate::serialization`] and
//! [`crate::state_packages`]) from *restore orchestration*: serialization
//! records what was remembered; the hydrator decides how to bring it back.
//!
//! The hydrator runs in two ordered passes per window:
//!
//! 1. **Skeleton first.** Window shells, the pane topology, and focus anchors
//!    are recreated before any heavy dependency is touched, and window bounds
//!    are remapped into safe visible bounds for the current monitor topology.
//!    A window therefore stays visible and structurally recognizable even when
//!    nothing else can hydrate.
//! 2. **Lazy hydration.** Remote sessions, terminals, notebooks, debuggers,
//!    preview servers, and extension panes are hydrated second. When a
//!    dependency is missing, revoked, or unavailable the pane reopens as a
//!    truthful placeholder that preserves its slot — it never collapses the
//!    surrounding layout and never impersonates a live, ready surface.
//!
//! Privileged or mutating sessions (terminal commands, tasks, notebook
//! kernels, preview servers, debuggers, remote sessions, collaboration
//! authority) are never replayed automatically. Every live-surface outcome
//! records explicit no-rerun guardrails so resuming live behavior always
//! requires a deliberate user action.
//!
//! The produced [`LayoutRestoreProvenanceRecord`] mirrors the
//! `layout_restore_provenance_record` shape in
//! `schemas/workspace/pane_tree.schema.json`, so diagnostics, support export,
//! and crash recovery read the same restore class, missing-dependency class,
//! and remaining-manual-action vocabulary shown in-product.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for the restore-hydration request and outcome records.
pub const RESTORE_HYDRATION_SCHEMA_VERSION: u32 = 1;

/// Pane-tree schema version pinned by emitted layout-restore provenance.
pub const RESTORE_PANE_TREE_SCHEMA_VERSION: u32 = 1;

/// Schema path for the restore-hydration request bundle.
pub const WINDOW_TOPOLOGY_SNAPSHOT_SCHEMA_REF: &str =
    "schemas/workspace/window_topology_snapshot.schema.json";

/// Schema path for the pane-tree and layout-restore provenance vocabulary.
pub const RESTORE_PANE_TREE_SCHEMA_REF: &str = "schemas/workspace/pane_tree.schema.json";

// ---------------------------------------------------------------------------
// Shared snapshot vocabulary (mirrors pane_tree.schema.json)
// ---------------------------------------------------------------------------

/// Why a window-topology snapshot was emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotReason {
    /// Snapshot taken during a graceful shutdown.
    GracefulShutdown,
    /// Snapshot taken at a crash-recovery checkpoint.
    CrashRecoveryCheckpoint,
    /// Snapshot taken for a manual export.
    ManualExport,
    /// Snapshot taken for a diagnostic capture.
    DiagnosticCapture,
    /// Snapshot taken for a support capture.
    SupportCapture,
}

/// Role of a window inside a multi-window topology family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRole {
    /// Primary window.
    Primary,
    /// Auxiliary window.
    Auxiliary,
    /// Presentation window.
    Presentation,
    /// Review window.
    Review,
    /// Incident window.
    Incident,
    /// Companion window.
    Companion,
}

/// Top-level window chrome state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    /// Normal windowed state.
    Normal,
    /// Maximized state.
    Maximized,
    /// Fullscreen state.
    Fullscreen,
    /// Zen state.
    Zen,
    /// Minimized state.
    Minimized,
}

/// Where the restore event originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSourceClass {
    /// Restore from an automatic checkpoint.
    AutoCheckpoint,
    /// Restore from a graceful-shutdown checkpoint.
    GracefulShutdownCheckpoint,
    /// Restore from a manual window export.
    ManualWindowExport,
    /// Restore from a portable-state package.
    PortableStatePackage,
    /// Restore from a profile-sync snapshot.
    ProfileSyncSnapshot,
    /// Restore from an imported handoff.
    ImportedHandoff,
    /// Restore from a support-bundle replay.
    SupportBundleReplay,
}

/// Surface role a pane occupies in the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceRole {
    /// Editor.
    Editor,
    /// Diff editor.
    Diff,
    /// Terminal.
    Terminal,
    /// Debugger.
    Debugger,
    /// Notebook.
    Notebook,
    /// Search.
    Search,
    /// Problems.
    Problems,
    /// Source control.
    Scm,
    /// Docs.
    Docs,
    /// Preview.
    Preview,
    /// AI panel.
    AiPanel,
    /// Explorer.
    Explorer,
    /// Test.
    Test,
    /// Custom extension surface.
    CustomExtension,
    /// Placeholder.
    Placeholder,
}

/// Concrete surface class a pane resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Text editor.
    TextEditor,
    /// Diff editor.
    DiffEditor,
    /// Terminal view.
    TerminalView,
    /// Debug view.
    DebugView,
    /// Notebook view.
    NotebookView,
    /// Search results.
    SearchResults,
    /// Problems panel.
    ProblemsPanel,
    /// SCM panel.
    ScmPanel,
    /// Docs browser.
    DocsBrowser,
    /// Preview canvas.
    PreviewCanvas,
    /// AI panel.
    AiPanel,
    /// Explorer tree.
    ExplorerTree,
    /// Test results.
    TestResults,
    /// Extension view.
    ExtensionView,
    /// Placeholder card.
    PlaceholderCard,
}

/// Live-surface class with reattach or no-rerun implications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSurfaceClass {
    /// Terminal.
    Terminal,
    /// Debug session.
    DebugSession,
    /// Notebook.
    Notebook,
    /// Remote shell.
    RemoteShell,
    /// Task runner.
    TaskRunner,
    /// Pipeline view.
    PipelineView,
    /// Extension view.
    ExtensionView,
    /// Preview runtime.
    PreviewRuntime,
}

impl LiveSurfaceClass {
    /// Returns true when this live surface can run or replay commands.
    const fn command_bearing(self) -> bool {
        matches!(
            self,
            Self::Terminal
                | Self::RemoteShell
                | Self::TaskRunner
                | Self::PipelineView
                | Self::PreviewRuntime
                | Self::Notebook
                | Self::DebugSession
        )
    }
}

/// How restore should treat a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HydrationBehavior {
    /// Eager lightweight surface.
    EagerLightweight,
    /// Lazy heavy surface.
    LazyHeavy,
    /// Placeholder only.
    PlaceholderOnly,
    /// Evidence only.
    EvidenceOnly,
}

/// Current availability of a pane surface in the snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityState {
    /// Ready.
    Ready,
    /// Needs hydration.
    NeedsHydration,
    /// Placeholder.
    Placeholder,
    /// Evidence only.
    EvidenceOnly,
}

/// Best-effort display scale bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ScaleBucket {
    /// 1x scale.
    #[serde(rename = "1x")]
    One,
    /// 1.25x scale.
    #[serde(rename = "1_25x")]
    OneTwentyFive,
    /// 1.5x scale.
    #[serde(rename = "1_5x")]
    OneFifty,
    /// 2x scale.
    #[serde(rename = "2x")]
    Two,
    /// Other scale bucket.
    #[serde(rename = "other")]
    Other,
}

/// Best-effort display class for monitor-affinity hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayClass {
    /// Internal panel.
    InternalPanel,
    /// External monitor.
    ExternalMonitor,
    /// Virtual display.
    VirtualDisplay,
    /// Projector or presentation display.
    ProjectorOrPresentation,
    /// Unknown display class.
    Unknown,
}

/// Strength of a monitor-affinity hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorAffinityStrength {
    /// No affinity.
    None,
    /// Prefer the same display.
    PreferSameDisplay,
    /// Prefer the same display class.
    PreferSameClass,
    /// Prefer the same region.
    PreferSameRegion,
}

/// What a focus-chain entry points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusTargetKind {
    /// A pane.
    Pane,
    /// A tab.
    Tab,
    /// An inspector.
    Inspector,
    /// A follow banner.
    FollowBanner,
    /// Window chrome.
    WindowChrome,
}

/// Orientation of a split node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitOrientation {
    /// Horizontal split.
    Horizontal,
    /// Vertical split.
    Vertical,
}

// ---------------------------------------------------------------------------
// Restore-provenance vocabulary (mirrors pane_tree.schema.json)
// ---------------------------------------------------------------------------

/// Shared restore-phase vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePhase {
    /// Chooser phase.
    Chooser,
    /// Skeleton phase: shells, topology, and focus anchors.
    Skeleton,
    /// Hydrate phase: heavy dependencies.
    Hydrate,
    /// Rebind phase: workspace authority.
    Rebind,
    /// Evidence-only fallback phase.
    EvidenceOnlyFallback,
}

/// Outcome of a restore phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseOutcome {
    /// Phase completed.
    Completed,
    /// Phase skipped.
    Skipped,
    /// Phase completed in a degraded state.
    Degraded,
    /// Phase blocked.
    Blocked,
    /// Phase rerouted to evidence-only.
    ReroutedToEvidenceOnly,
}

/// How workspace authority resolved during restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRebindResult {
    /// Bound existing authority.
    BoundExistingAuthority,
    /// Reevaluated and bound authority.
    ReevaluatedAndBound,
    /// Degraded to local-only.
    DegradedLocalOnly,
    /// Missing authority placeholder.
    MissingAuthorityPlaceholder,
}

/// Material display-topology adjustment applied during restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayAdjustmentClass {
    /// Window snapped into safe visible bounds.
    SnappedToSafeBounds,
    /// Window moved to the primary display.
    MovedToPrimaryDisplay,
    /// Window normalized across scale buckets.
    ScaleNormalized,
    /// Fullscreen state cleared.
    FullscreenCleared,
    /// Window stacking repaired.
    StackingRepaired,
}

impl DisplayAdjustmentClass {
    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::SnappedToSafeBounds => "Snapped to safe bounds",
            Self::MovedToPrimaryDisplay => "Moved to primary display",
            Self::ScaleNormalized => "Scale normalized",
            Self::FullscreenCleared => "Fullscreen cleared",
            Self::StackingRepaired => "Stacking repaired",
        }
    }
}

/// Resulting restore posture for one live surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceRestorePosture {
    /// Live surface attached and visible (no command rerun).
    LiveAttachVisible,
    /// Metadata-only placeholder.
    MetadataOnlyPlaceholder,
    /// Evidence-only placeholder.
    EvidenceOnlyPlaceholder,
    /// Placeholder until manual rebind.
    PlaceholderUntilManualRebind,
    /// Surface not present.
    NotPresent,
}

impl SurfaceRestorePosture {
    /// Returns true when this posture must not imply live readiness.
    pub const fn is_placeholder(self) -> bool {
        matches!(
            self,
            Self::MetadataOnlyPlaceholder
                | Self::EvidenceOnlyPlaceholder
                | Self::PlaceholderUntilManualRebind
        )
    }
}

/// Authority posture for one live surface after restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceAuthorityPosture {
    /// Not applicable.
    NotApplicable,
    /// Existing authority still valid.
    ExistingAuthorityStillValid,
    /// Manual rebind required.
    ManualRebindRequired,
    /// Reauthentication required.
    ReauthRequired,
}

/// Explicit guardrail honored for one live surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoRerunGuardrail {
    /// Terminal or task commands must not rerun.
    NoCommandRerun,
    /// Authority must not be reacquired silently.
    NoHiddenAuthorityReacquire,
    /// Only transcript or snapshot evidence is shown.
    TranscriptOrSnapshotOnly,
    /// Explicit user action is required to resume live behavior.
    ExplicitUserActionRequired,
    /// A placeholder card keeps the pane slot visible.
    PlaceholderPreserved,
}

/// Why a placeholder card occupies a pane slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderReasonClass {
    /// Required extension or feature pack is missing.
    MissingExtension,
    /// Required remote target is unavailable.
    MissingRemote,
    /// Required remote authority is missing or expired.
    MissingRemoteAuthority,
    /// Permission was revoked or expired.
    RevokedPermission,
    /// Display topology is unsupported for the saved bounds.
    UnsupportedDisplayTopology,
    /// Live surface cannot safely resume without explicit action.
    NonReentrantLiveSurface,
    /// A schema migration needs review before hydration.
    SchemaMigrationReviewRequired,
    /// Manual recovery is required.
    ManualRecoveryRequired,
}

impl PlaceholderReasonClass {
    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::MissingExtension => "Missing extension",
            Self::MissingRemote => "Missing remote",
            Self::MissingRemoteAuthority => "Missing remote authority",
            Self::RevokedPermission => "Revoked permission",
            Self::UnsupportedDisplayTopology => "Unsupported display topology",
            Self::NonReentrantLiveSurface => "Non-reentrant live surface",
            Self::SchemaMigrationReviewRequired => "Schema migration review required",
            Self::ManualRecoveryRequired => "Manual recovery required",
        }
    }
}

/// Typed safe recovery action rendered on a placeholder or provenance surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderActionClass {
    /// Try hydrating again.
    RetryHydrate,
    /// Locate the missing extension.
    LocateExtension,
    /// Install or enable the missing extension.
    InstallExtension,
    /// Reauthenticate.
    Reauthenticate,
    /// Reconnect the remote.
    ReconnectRemote,
    /// Recover a draft.
    RecoverDraft,
    /// Open without the missing dependency.
    OpenWithout,
    /// Export retained evidence.
    ExportEvidence,
    /// Remove the pane slot deliberately.
    RemovePane,
    /// Rerun only after explicit user action.
    RerunExplicitly,
    /// Rebind an existing session.
    RebindExistingSession,
}

impl PlaceholderActionClass {
    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::RetryHydrate => "Retry hydrate",
            Self::LocateExtension => "Locate extension",
            Self::InstallExtension => "Install extension",
            Self::Reauthenticate => "Reauthenticate",
            Self::ReconnectRemote => "Reconnect remote",
            Self::RecoverDraft => "Recover draft",
            Self::OpenWithout => "Open without",
            Self::ExportEvidence => "Export evidence",
            Self::RemovePane => "Remove pane",
            Self::RerunExplicitly => "Rerun explicitly",
            Self::RebindExistingSession => "Rebind existing session",
        }
    }
}

/// Re-export of the restore-level (restore-class) vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreLevel {
    /// Exact restore.
    ExactRestore,
    /// Compatible restore.
    CompatibleRestore,
    /// Layout-only restore.
    LayoutOnly,
    /// Recovered drafts.
    RecoveredDrafts,
    /// Evidence-only restore.
    EvidenceOnly,
    /// No restore.
    NoRestore,
}

impl RestoreLevel {
    /// Worst-first severity rank, used to fold a window or run to one class.
    const fn rank(self) -> u8 {
        match self {
            Self::ExactRestore => 0,
            Self::CompatibleRestore => 1,
            Self::LayoutOnly => 2,
            Self::RecoveredDrafts => 3,
            Self::EvidenceOnly => 4,
            Self::NoRestore => 5,
        }
    }

    /// Returns the worse (more degraded) of two restore levels.
    fn worst(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactRestore => "Exact restore",
            Self::CompatibleRestore => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::RecoveredDrafts => "Recovered drafts",
            Self::EvidenceOnly => "Evidence only",
            Self::NoRestore => "No restore",
        }
    }
}

// ---------------------------------------------------------------------------
// Input: window-topology snapshot
// ---------------------------------------------------------------------------

/// Best-effort geometry for a window or display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bounds {
    /// Left edge.
    pub x: i64,
    /// Top edge.
    pub y: i64,
    /// Width (must be positive).
    pub width: i64,
    /// Height (must be positive).
    pub height: i64,
}

impl Bounds {
    /// Returns true when `self` is fully contained within `outer`.
    fn within(&self, outer: &Bounds) -> bool {
        self.width > 0
            && self.height > 0
            && self.x >= outer.x
            && self.y >= outer.y
            && self.x + self.width <= outer.x + outer.width
            && self.y + self.height <= outer.y + outer.height
    }
}

/// Boundary refs keeping authority, profile defaults, and hints separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRefs {
    /// Authoritative workspace state backing this window.
    pub workspace_authority_ref: String,
    /// Optional profile-defaults ref that seeded the window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_defaults_ref: Option<String>,
    /// Optional machine-local display-hint cache row ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machine_display_hint_ref: Option<String>,
}

/// Best-effort monitor-affinity metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorAffinityHint {
    /// Strength of the affinity hint.
    pub affinity_strength: MonitorAffinityStrength,
    /// Best-effort display class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_class: Option<DisplayClass>,
    /// Last known display ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_display_ref: Option<String>,
    /// Preferred scale bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_scale_bucket: Option<ScaleBucket>,
    /// Preferred bounds hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_bounds_hint: Option<Bounds>,
}

/// One entry in the remembered focus chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusChainEntry {
    /// What the entry points at.
    pub target_kind: FocusTargetKind,
    /// Opaque target ref.
    pub target_ref: String,
    /// Optional redaction-aware note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Window-level chrome state separate from the pane tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowChromeState {
    /// Top-level window state.
    pub window_state: WindowState,
}

/// Placeholder card captured in the snapshot for an already-degraded pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotPlaceholderCard {
    /// Placeholder reason class.
    pub placeholder_reason: PlaceholderReasonClass,
    /// Safe recovery actions.
    pub safe_actions: Vec<PlaceholderActionClass>,
    /// Whether redacted evidence remains.
    pub evidence_retained: bool,
    /// Last known redaction-aware label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_provenance_label: Option<String>,
}

/// Surface payload attached to a leaf pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneSurfaceDescriptor {
    /// User-facing role.
    pub surface_role: SurfaceRole,
    /// Concrete surface class.
    pub surface_class: SurfaceClass,
    /// Live-surface class when the pane is a live or expensive surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_surface_class: Option<LiveSurfaceClass>,
    /// How restore should treat the surface.
    pub hydration_behavior: HydrationBehavior,
    /// Availability state recorded in the snapshot.
    pub availability_state: AvailabilityState,
    /// Last useful title or label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
    /// Opaque surface binding or restore-token ref (never a raw ticket).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_binding_ref: Option<String>,
    /// Placeholder card already present in the snapshot, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_card: Option<SnapshotPlaceholderCard>,
}

/// One tab in a tab group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabRecord {
    /// Stable tab id.
    pub tab_id: String,
    /// Redaction-aware tab label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_label: Option<String>,
    /// Whether the tab is pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// The leaf pane carried by this tab.
    pub pane: PaneNode,
}

/// Recursive pane-tree node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "node_kind", rename_all = "snake_case")]
pub enum PaneNode {
    /// Split node.
    Split {
        /// Stable split id.
        split_id: String,
        /// Split orientation.
        orientation: SplitOrientation,
        /// Child nodes (at least two).
        children: Vec<PaneNode>,
    },
    /// Tab-group node.
    TabGroup {
        /// Stable group id.
        group_id: String,
        /// Tabs in the group (at least one).
        tabs: Vec<TabRecord>,
        /// Active tab id.
        active_tab_id: String,
    },
    /// Leaf pane carrying one surface.
    Leaf {
        /// Stable pane id (survives placeholder insertion and restore).
        pane_id: String,
        /// Surface descriptor.
        surface: PaneSurfaceDescriptor,
    },
}

impl PaneNode {
    /// Collects every leaf pane in deterministic tree order.
    fn collect_leaves<'a>(&'a self, out: &mut Vec<(&'a str, &'a PaneSurfaceDescriptor)>) {
        match self {
            Self::Split { children, .. } => {
                for child in children {
                    child.collect_leaves(out);
                }
            }
            Self::TabGroup { tabs, .. } => {
                for tab in tabs {
                    tab.pane.collect_leaves(out);
                }
            }
            Self::Leaf { pane_id, surface } => out.push((pane_id.as_str(), surface)),
        }
    }
}

/// Versioned pane tree for one window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneTree {
    /// Tree revision within the snapshot's lifetime.
    pub tree_revision: u32,
    /// Root node.
    pub root_node: PaneNode,
}

/// One remembered window-topology snapshot to restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowTopologySnapshot {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Why the snapshot was taken.
    pub snapshot_reason: SnapshotReason,
    /// Stable window id.
    pub window_id: String,
    /// Window role in the topology family.
    pub window_role: WindowRole,
    /// Boundary refs.
    pub scope_refs: ScopeRefs,
    /// Pane tree.
    pub pane_tree: PaneTree,
    /// Remembered focus chain (at least one entry).
    pub focus_chain: Vec<FocusChainEntry>,
    /// Window chrome state.
    pub window_chrome_state: WindowChromeState,
    /// Monitor-affinity hint.
    pub monitor_affinity_hint: MonitorAffinityHint,
    /// Producer-local emit time.
    pub emitted_at: String,
    /// Redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl WindowTopologySnapshot {
    fn leaves(&self) -> Vec<(&str, &PaneSurfaceDescriptor)> {
        let mut out = Vec::new();
        self.pane_tree.root_node.collect_leaves(&mut out);
        out
    }
}

// ---------------------------------------------------------------------------
// Input: restore environment
// ---------------------------------------------------------------------------

/// Dependency class a pane needs to hydrate as a live surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// Extension or feature pack.
    Extension,
    /// Remote target.
    Remote,
    /// Remote authority.
    RemoteAuthority,
    /// Provider connection.
    Provider,
    /// Managed service.
    Service,
    /// Local permission grant.
    Permission,
    /// Workspace authority.
    WorkspaceAuthority,
}

impl DependencyClass {
    /// Maps a missing dependency to the placeholder reason class shown in-product.
    const fn missing_reason(self) -> PlaceholderReasonClass {
        match self {
            Self::Extension => PlaceholderReasonClass::MissingExtension,
            Self::Remote => PlaceholderReasonClass::MissingRemote,
            Self::RemoteAuthority => PlaceholderReasonClass::MissingRemoteAuthority,
            Self::Permission => PlaceholderReasonClass::RevokedPermission,
            Self::Provider | Self::Service | Self::WorkspaceAuthority => {
                PlaceholderReasonClass::ManualRecoveryRequired
            }
        }
    }
}

/// A per-pane heavy-dependency requirement resolved at restore time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneDependency {
    /// Pane this requirement belongs to.
    pub pane_id: String,
    /// Dependency class.
    pub dependency_class: DependencyClass,
    /// Opaque dependency ref.
    pub dependency_ref: String,
    /// Whether the live session can be reattached without rerun.
    #[serde(default)]
    pub reattachable: bool,
}

/// A display connected at restore time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedDisplay {
    /// Opaque display ref.
    pub display_ref: String,
    /// Display class.
    pub display_class: DisplayClass,
    /// Scale bucket.
    pub scale_bucket: ScaleBucket,
    /// Visible usable bounds for this display.
    pub visible_bounds: Bounds,
}

/// The runtime restore environment: which dependencies and displays exist now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreEnvironment {
    /// Whether workspace authority is present at all.
    pub workspace_authority_present: bool,
    /// Authority refs currently bound and valid.
    #[serde(default)]
    pub present_authority_refs: BTreeSet<String>,
    /// Available extension refs.
    #[serde(default)]
    pub available_extensions: BTreeSet<String>,
    /// Reachable remote refs.
    #[serde(default)]
    pub reachable_remotes: BTreeSet<String>,
    /// Available provider/service refs.
    #[serde(default)]
    pub available_providers: BTreeSet<String>,
    /// Granted permission and remote-authority refs.
    #[serde(default)]
    pub granted_permissions: BTreeSet<String>,
    /// Displays connected at restore time (at least one).
    pub connected_displays: Vec<ConnectedDisplay>,
    /// Primary display ref (must be connected).
    pub primary_display_ref: String,
    /// Whether fullscreen is cleared on restore.
    #[serde(default)]
    pub clear_fullscreen_on_restore: bool,
    /// Per-pane heavy-dependency requirements.
    #[serde(default)]
    pub pane_dependencies: Vec<PaneDependency>,
}

impl RestoreEnvironment {
    fn dependency_available(&self, dep: &PaneDependency) -> bool {
        match dep.dependency_class {
            DependencyClass::Extension => self.available_extensions.contains(&dep.dependency_ref),
            DependencyClass::Remote => self.reachable_remotes.contains(&dep.dependency_ref),
            DependencyClass::RemoteAuthority | DependencyClass::Permission => {
                self.granted_permissions.contains(&dep.dependency_ref)
            }
            DependencyClass::Provider | DependencyClass::Service => {
                self.available_providers.contains(&dep.dependency_ref)
            }
            DependencyClass::WorkspaceAuthority => {
                self.workspace_authority_present
                    && self.present_authority_refs.contains(&dep.dependency_ref)
            }
        }
    }

    fn display(&self, display_ref: &str) -> Option<&ConnectedDisplay> {
        self.connected_displays
            .iter()
            .find(|display| display.display_ref == display_ref)
    }

    fn primary_display(&self) -> Option<&ConnectedDisplay> {
        self.display(&self.primary_display_ref)
    }
}

// ---------------------------------------------------------------------------
// Input: restore-hydration request
// ---------------------------------------------------------------------------

/// Record discriminator for restore-hydration request and outcome records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreHydrationRecordKind {
    /// A restore-hydration request bundle.
    RestoreHydrationRequest,
    /// A restore-hydration outcome.
    RestoreHydrationOutcome,
}

/// A bundled multi-window restore request consumed by the hydrator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreHydrationRequest {
    /// Record discriminator.
    pub record_kind: RestoreHydrationRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Stable request id.
    pub request_id: String,
    /// Workspace ref this request restores.
    pub workspace_ref: String,
    /// Where the restore originated.
    pub restore_source_class: RestoreSourceClass,
    /// Producer build or instance ref.
    pub producer_ref: String,
    /// Window snapshots to restore (at least one).
    pub snapshots: Vec<WindowTopologySnapshot>,
    /// Runtime restore environment.
    pub environment: RestoreEnvironment,
    /// Redaction-aware notes.
    pub notes: String,
}

impl RestoreHydrationRequest {
    /// Validates request-level invariants before orchestration.
    pub fn validate(&self) -> Result<(), RestoreHydrationError> {
        if self.record_kind != RestoreHydrationRecordKind::RestoreHydrationRequest {
            return Err(RestoreHydrationError::WrongRecordKind);
        }
        if self.schema_version != RESTORE_HYDRATION_SCHEMA_VERSION {
            return Err(RestoreHydrationError::WrongSchemaVersion {
                expected: RESTORE_HYDRATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        require_non_empty("request.request_id", &self.request_id)?;
        require_non_empty("request.workspace_ref", &self.workspace_ref)?;
        require_non_empty("request.producer_ref", &self.producer_ref)?;
        require_non_empty("request.notes", &self.notes)?;

        if self.snapshots.is_empty() {
            return Err(RestoreHydrationError::NoWindows);
        }
        if self.environment.connected_displays.is_empty() {
            return Err(RestoreHydrationError::NoConnectedDisplays);
        }
        if self.environment.primary_display().is_none() {
            return Err(RestoreHydrationError::PrimaryDisplayNotConnected {
                display_ref: self.environment.primary_display_ref.clone(),
            });
        }

        let mut window_ids = BTreeSet::new();
        let mut pane_ids = BTreeSet::new();
        for snapshot in &self.snapshots {
            require_non_empty("window.window_id", &snapshot.window_id)?;
            require_non_empty("window.snapshot_id", &snapshot.snapshot_id)?;
            require_non_empty(
                "window.workspace_authority_ref",
                &snapshot.scope_refs.workspace_authority_ref,
            )?;
            if snapshot.focus_chain.is_empty() {
                return Err(RestoreHydrationError::WindowMissingFocusChain {
                    window_id: snapshot.window_id.clone(),
                });
            }
            if !window_ids.insert(snapshot.window_id.clone()) {
                return Err(RestoreHydrationError::DuplicateWindowId {
                    window_id: snapshot.window_id.clone(),
                });
            }
            let leaves = snapshot.leaves();
            if leaves.is_empty() {
                return Err(RestoreHydrationError::WindowMissingPanes {
                    window_id: snapshot.window_id.clone(),
                });
            }
            for (pane_id, _) in &leaves {
                if !pane_ids.insert((*pane_id).to_string()) {
                    return Err(RestoreHydrationError::DuplicatePaneId {
                        pane_id: (*pane_id).to_string(),
                    });
                }
            }
        }

        for dep in &self.environment.pane_dependencies {
            require_non_empty("dependency.dependency_ref", &dep.dependency_ref)?;
            if !pane_ids.contains(&dep.pane_id) {
                return Err(RestoreHydrationError::DependencyForUnknownPane {
                    pane_id: dep.pane_id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Runs the restore hydrator and returns a validated outcome.
    pub fn hydrate(&self) -> Result<RestoreHydrationOutcome, RestoreHydrationError> {
        self.validate()?;
        let mut windows = Vec::with_capacity(self.snapshots.len());
        for snapshot in &self.snapshots {
            windows.push(hydrate_window(self, snapshot)?);
        }
        let summary = RestoreHydrationSummary::from_windows(self, &windows);
        let outcome = RestoreHydrationOutcome {
            record_kind: RestoreHydrationRecordKind::RestoreHydrationOutcome,
            schema_version: RESTORE_HYDRATION_SCHEMA_VERSION,
            request_id: self.request_id.clone(),
            workspace_ref: self.workspace_ref.clone(),
            restore_source_class: self.restore_source_class,
            windows,
            summary,
            notes: "Restore reopened window shells and pane topology first, then hydrated heavy \
                    dependencies lazily; missing surfaces are placeholders and mutating sessions \
                    are never replayed automatically."
                .to_string(),
        };
        outcome.validate()?;
        Ok(outcome)
    }
}

// ---------------------------------------------------------------------------
// Output: layout-restore provenance (mirrors pane_tree.schema.json)
// ---------------------------------------------------------------------------

/// Record discriminator for the layout-restore provenance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutRestoreProvenanceRecordKind {
    /// A layout-restore provenance record.
    LayoutRestoreProvenanceRecord,
}

/// One phase entry in a layout-restore provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePhaseRecord {
    /// Phase.
    pub phase: RestorePhase,
    /// Phase outcome.
    pub outcome: PhaseOutcome,
    /// Affected pane ids.
    #[serde(default)]
    pub affected_pane_ids: Vec<String>,
    /// Redaction-aware note.
    pub note: String,
}

/// One material display or monitor-topology adjustment applied during restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayAdjustmentRecord {
    /// Adjustment class.
    pub adjustment_class: DisplayAdjustmentClass,
    /// Affected pane ids.
    #[serde(default)]
    pub affected_pane_ids: Vec<String>,
    /// Redaction-aware note.
    pub note: String,
}

/// Degraded placeholder result for one pane during restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderResultRecord {
    /// Pane id (slot preserved).
    pub pane_id: String,
    /// Original surface role.
    pub surface_role: SurfaceRole,
    /// Original surface class.
    pub surface_class: SurfaceClass,
    /// Why the placeholder occupies the slot.
    pub placeholder_reason: PlaceholderReasonClass,
    /// Safe recovery actions (at least one).
    pub safe_actions: Vec<PlaceholderActionClass>,
    /// Whether redacted evidence remains.
    pub evidence_retained: bool,
    /// Last known redaction-aware label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_provenance_label: Option<String>,
    /// Redaction-aware note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Restore outcome for one live surface, where no-rerun is recorded explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveSurfaceOutcomeRecord {
    /// Pane id.
    pub pane_id: String,
    /// Live-surface class.
    pub live_surface_class: LiveSurfaceClass,
    /// Restore posture.
    pub restore_posture: SurfaceRestorePosture,
    /// Authority posture.
    pub authority_posture: SurfaceAuthorityPosture,
    /// No-rerun guardrails honored (at least one).
    pub no_rerun_guardrails: Vec<NoRerunGuardrail>,
    /// Whether redacted evidence remains.
    pub evidence_retained: bool,
    /// Redaction-aware note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One restore event's layout-focused provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutRestoreProvenanceRecord {
    /// Record discriminator.
    pub record_kind: LayoutRestoreProvenanceRecordKind,
    /// Pane-tree schema version.
    pub pane_tree_schema_version: u32,
    /// Stable restore-event id.
    pub restore_event_id: String,
    /// Source snapshot ref.
    pub source_snapshot_ref: String,
    /// Restore source class.
    pub restore_source_class: RestoreSourceClass,
    /// Workspace authority ref.
    pub workspace_authority_ref: String,
    /// Window id.
    pub window_id: String,
    /// Restore level (restore class) for this window.
    pub restore_level: RestoreLevel,
    /// Phase trace (all five phases, in order).
    pub phase_trace: Vec<RestorePhaseRecord>,
    /// Authority rebind result.
    pub authority_rebind_result: AuthorityRebindResult,
    /// Display adjustments applied.
    #[serde(default)]
    pub display_adjustments: Vec<DisplayAdjustmentRecord>,
    /// Placeholder results.
    #[serde(default)]
    pub placeholder_results: Vec<PlaceholderResultRecord>,
    /// Live-surface outcomes.
    #[serde(default)]
    pub live_surface_outcomes: Vec<LiveSurfaceOutcomeRecord>,
    /// Producer-local emit time.
    pub emitted_at: String,
    /// Redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// The restored result for one window: skeleton, bounds, and provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowRestoreResult {
    /// Window id.
    pub window_id: String,
    /// Window role.
    pub window_role: WindowRole,
    /// Whether the window shell and pane topology were recreated.
    pub shell_restored: bool,
    /// Display the window was placed on.
    pub chosen_display_ref: String,
    /// Final safe visible bounds.
    pub applied_bounds: Bounds,
    /// Pane ids preserved in tree order.
    pub preserved_pane_ids: Vec<String>,
    /// Focus anchor pane/target ref after restore.
    pub focus_anchor_ref: String,
    /// Layout-restore provenance for this window.
    pub provenance: LayoutRestoreProvenanceRecord,
}

// ---------------------------------------------------------------------------
// Output: aggregate outcome and support/diagnostics summary
// ---------------------------------------------------------------------------

/// Support- and diagnostics-facing projection of a restore run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreHydrationSummary {
    /// Aggregate restore level (worst across windows).
    pub aggregate_restore_level: RestoreLevel,
    /// Number of windows restored.
    pub window_count: usize,
    /// Number of panes preserved.
    pub pane_count: usize,
    /// Panes that hydrated live (attached and visible).
    pub live_pane_count: usize,
    /// Panes that reopened as placeholders.
    pub placeholder_pane_count: usize,
    /// Panes that reopened as evidence only.
    pub evidence_only_pane_count: usize,
    /// Missing-dependency classes observed (in-product vocabulary).
    pub missing_dependency_classes: Vec<PlaceholderReasonClass>,
    /// Remaining manual actions across placeholders and live surfaces.
    pub remaining_manual_actions: Vec<PlaceholderActionClass>,
    /// Display-adjustment classes applied.
    pub display_adjustment_classes: Vec<DisplayAdjustmentClass>,
    /// Diagnostics surface ref where this summary remains visible.
    pub diagnostics_ref: String,
    /// Support-export ref where this summary remains visible.
    pub support_export_ref: String,
    /// Crash-recovery ref where this summary remains visible.
    pub crash_recovery_ref: String,
    /// Redaction-aware notes.
    pub notes: String,
}

impl RestoreHydrationSummary {
    fn from_windows(request: &RestoreHydrationRequest, windows: &[WindowRestoreResult]) -> Self {
        let mut aggregate = RestoreLevel::ExactRestore;
        let mut pane_count = 0usize;
        // Pane-id sets keep per-pane counts deterministic and free of double
        // counting when a pane appears in both the placeholder and live lanes.
        let mut live_panes: BTreeSet<&str> = BTreeSet::new();
        let mut evidence_panes: BTreeSet<&str> = BTreeSet::new();
        let mut placeholder_panes: BTreeSet<&str> = BTreeSet::new();
        let mut missing = BTreeSet::new();
        let mut actions = BTreeSet::new();
        let mut adjustments = BTreeSet::new();

        for window in windows {
            aggregate = aggregate.worst(window.provenance.restore_level);
            pane_count += window.preserved_pane_ids.len();
            for placeholder in &window.provenance.placeholder_results {
                placeholder_panes.insert(placeholder.pane_id.as_str());
                missing.insert(placeholder.placeholder_reason);
                for action in &placeholder.safe_actions {
                    actions.insert(*action);
                }
            }
            for outcome in &window.provenance.live_surface_outcomes {
                match outcome.restore_posture {
                    SurfaceRestorePosture::LiveAttachVisible => {
                        live_panes.insert(outcome.pane_id.as_str());
                    }
                    SurfaceRestorePosture::EvidenceOnlyPlaceholder => {
                        evidence_panes.insert(outcome.pane_id.as_str());
                        actions.insert(PlaceholderActionClass::RerunExplicitly);
                    }
                    SurfaceRestorePosture::MetadataOnlyPlaceholder
                    | SurfaceRestorePosture::PlaceholderUntilManualRebind => {
                        placeholder_panes.insert(outcome.pane_id.as_str());
                        actions.insert(PlaceholderActionClass::RerunExplicitly);
                    }
                    SurfaceRestorePosture::NotPresent => {}
                }
            }
            for adjustment in &window.provenance.display_adjustments {
                adjustments.insert(adjustment.adjustment_class);
            }
        }

        // A pane recorded as evidence-only or live-attached is not also a placeholder.
        for pane_id in evidence_panes.iter().chain(live_panes.iter()) {
            placeholder_panes.remove(pane_id);
        }

        Self {
            aggregate_restore_level: aggregate,
            window_count: windows.len(),
            pane_count,
            live_pane_count: live_panes.len(),
            placeholder_pane_count: placeholder_panes.len(),
            evidence_only_pane_count: evidence_panes.len(),
            missing_dependency_classes: missing.into_iter().collect(),
            remaining_manual_actions: actions.into_iter().collect(),
            display_adjustment_classes: adjustments.into_iter().collect(),
            diagnostics_ref: format!("diagnostics:restore:{}", request.request_id),
            support_export_ref: format!("support-export:restore:{}", request.request_id),
            crash_recovery_ref: format!("crash-recovery:restore:{}", request.request_id),
            notes: "Restore class, missing-dependency class, and remaining manual actions use the \
                    same vocabulary shown in-product."
                .to_string(),
        }
    }

    /// Renders a support-safe plaintext view of the restore summary.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![format!(
            "Restore Hydration Summary restore_class={} windows={} panes={} live={} placeholders={} evidence_only={}",
            self.aggregate_restore_level.display_label(),
            self.window_count,
            self.pane_count,
            self.live_pane_count,
            self.placeholder_pane_count,
            self.evidence_only_pane_count,
        )];
        lines.push(format!(
            "- missing_dependencies={}",
            join_labels(
                self.missing_dependency_classes
                    .iter()
                    .map(|reason| reason.display_label())
            )
        ));
        lines.push(format!(
            "- remaining_manual_actions={}",
            join_labels(
                self.remaining_manual_actions
                    .iter()
                    .map(|action| action.display_label())
            )
        ));
        lines.push(format!(
            "- display_adjustments={}",
            join_labels(
                self.display_adjustment_classes
                    .iter()
                    .map(|class| class.display_label())
            )
        ));
        lines.push(format!("- diagnostics_ref={}", self.diagnostics_ref));
        lines.push(format!("- support_export_ref={}", self.support_export_ref));
        lines.push(format!("- crash_recovery_ref={}", self.crash_recovery_ref));
        lines.join("\n")
    }
}

/// The validated outcome of a restore-hydration run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreHydrationOutcome {
    /// Record discriminator.
    pub record_kind: RestoreHydrationRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Request id this outcome answers.
    pub request_id: String,
    /// Workspace ref.
    pub workspace_ref: String,
    /// Where the restore originated.
    pub restore_source_class: RestoreSourceClass,
    /// Per-window restore results.
    pub windows: Vec<WindowRestoreResult>,
    /// Support- and diagnostics-facing summary.
    pub summary: RestoreHydrationSummary,
    /// Redaction-aware notes.
    pub notes: String,
}

impl RestoreHydrationOutcome {
    /// Validates restore honesty and safety invariants.
    pub fn validate(&self) -> Result<(), RestoreHydrationError> {
        if self.record_kind != RestoreHydrationRecordKind::RestoreHydrationOutcome {
            return Err(RestoreHydrationError::WrongRecordKind);
        }
        if self.schema_version != RESTORE_HYDRATION_SCHEMA_VERSION {
            return Err(RestoreHydrationError::WrongSchemaVersion {
                expected: RESTORE_HYDRATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        for window in &self.windows {
            window.validate()?;
        }
        Ok(())
    }
}

impl WindowRestoreResult {
    /// Validates one restored window.
    pub fn validate(&self) -> Result<(), RestoreHydrationError> {
        if !self.shell_restored {
            return Err(RestoreHydrationError::ShellNotRestored {
                window_id: self.window_id.clone(),
            });
        }
        if self.preserved_pane_ids.is_empty() {
            return Err(RestoreHydrationError::WindowMissingPanes {
                window_id: self.window_id.clone(),
            });
        }

        let pane_ids = self
            .preserved_pane_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();

        // Off-screen recovery: the final bounds must be within the chosen display.
        // (Bounds are computed against the chosen display, so width/height > 0 too.)
        if self.applied_bounds.width <= 0 || self.applied_bounds.height <= 0 {
            return Err(RestoreHydrationError::TrappedWindowBounds {
                window_id: self.window_id.clone(),
            });
        }

        let provenance = &self.provenance;
        if provenance.record_kind
            != LayoutRestoreProvenanceRecordKind::LayoutRestoreProvenanceRecord
        {
            return Err(RestoreHydrationError::WrongRecordKind);
        }

        // Skeleton must be the first non-chooser phase and must have completed.
        let skeleton_completed = provenance.phase_trace.iter().any(|phase| {
            phase.phase == RestorePhase::Skeleton && phase.outcome == PhaseOutcome::Completed
        });
        if !skeleton_completed {
            return Err(RestoreHydrationError::SkeletonNotCompleted {
                window_id: self.window_id.clone(),
            });
        }
        let phases = provenance
            .phase_trace
            .iter()
            .map(|phase| phase.phase)
            .collect::<BTreeSet<_>>();
        for required in [
            RestorePhase::Chooser,
            RestorePhase::Skeleton,
            RestorePhase::Hydrate,
            RestorePhase::Rebind,
            RestorePhase::EvidenceOnlyFallback,
        ] {
            if !phases.contains(&required) {
                return Err(RestoreHydrationError::PhaseTraceIncomplete {
                    window_id: self.window_id.clone(),
                });
            }
        }

        // Placeholder honesty.
        for placeholder in &provenance.placeholder_results {
            if !pane_ids.contains(placeholder.pane_id.as_str()) {
                return Err(RestoreHydrationError::OutcomeForUnknownPane {
                    pane_id: placeholder.pane_id.clone(),
                });
            }
            if placeholder.safe_actions.is_empty() {
                return Err(RestoreHydrationError::PlaceholderMissingAction {
                    pane_id: placeholder.pane_id.clone(),
                });
            }
        }

        // No-silent-rerun: every live-surface outcome must require explicit action,
        // and a placeholder posture must never imply live readiness.
        for outcome in &provenance.live_surface_outcomes {
            if !pane_ids.contains(outcome.pane_id.as_str()) {
                return Err(RestoreHydrationError::OutcomeForUnknownPane {
                    pane_id: outcome.pane_id.clone(),
                });
            }
            if outcome.no_rerun_guardrails.is_empty()
                || !outcome
                    .no_rerun_guardrails
                    .contains(&NoRerunGuardrail::ExplicitUserActionRequired)
            {
                return Err(RestoreHydrationError::LiveOutcomeMissingNoRerun {
                    pane_id: outcome.pane_id.clone(),
                });
            }
            if outcome.restore_posture == SurfaceRestorePosture::LiveAttachVisible
                && outcome.authority_posture == SurfaceAuthorityPosture::NotApplicable
            {
                return Err(RestoreHydrationError::LiveOutcomeMissingAuthority {
                    pane_id: outcome.pane_id.clone(),
                });
            }
        }

        // Exact restore must not carry any placeholder or evidence-only outcome.
        if provenance.restore_level == RestoreLevel::ExactRestore {
            let degraded = !provenance.placeholder_results.is_empty()
                || provenance
                    .live_surface_outcomes
                    .iter()
                    .any(|outcome| outcome.restore_posture.is_placeholder());
            if degraded {
                return Err(RestoreHydrationError::ExactWindowHasPlaceholder {
                    window_id: self.window_id.clone(),
                });
            }
        }

        // Display adjustments must keep pane-id provenance.
        for adjustment in &provenance.display_adjustments {
            if adjustment.affected_pane_ids.is_empty() {
                return Err(RestoreHydrationError::DisplayAdjustmentLostPaneIds {
                    window_id: self.window_id.clone(),
                });
            }
        }

        // Focus anchor must point at a preserved pane slot when it is pane-bound.
        if self.focus_anchor_ref.trim().is_empty() {
            return Err(RestoreHydrationError::MissingField {
                field: "window.focus_anchor_ref",
            });
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Orchestration
// ---------------------------------------------------------------------------

/// Per-pane fold of restore decisions for one pane.
struct PaneDecision {
    fidelity: RestoreLevel,
    placeholder: Option<PlaceholderResultRecord>,
    live_outcome: Option<LiveSurfaceOutcomeRecord>,
    hydrate_touched: bool,
    evidence_only: bool,
}

fn hydrate_window(
    request: &RestoreHydrationRequest,
    snapshot: &WindowTopologySnapshot,
) -> Result<WindowRestoreResult, RestoreHydrationError> {
    let env = &request.environment;
    let leaves = snapshot.leaves();
    let all_pane_ids: Vec<String> = leaves.iter().map(|(id, _)| (*id).to_string()).collect();

    // --- Phase: skeleton + safe bounds remap (runs before any hydration) ---
    let bounds_plan = remap_bounds(snapshot, env);

    // --- Phase: lazy hydration of each pane ---
    let mut window_level = RestoreLevel::ExactRestore;
    let mut placeholder_results = Vec::new();
    let mut live_surface_outcomes = Vec::new();
    let mut hydrated_pane_ids = Vec::new();
    let mut evidence_only_pane_ids = Vec::new();
    let mut authority_required_missing = false;

    for (pane_id, surface) in &leaves {
        let dep = env
            .pane_dependencies
            .iter()
            .find(|candidate| candidate.pane_id == *pane_id);
        if let Some(dep) = dep {
            if dep.dependency_class == DependencyClass::WorkspaceAuthority
                && !env.dependency_available(dep)
            {
                authority_required_missing = true;
            }
        }
        let decision = decide_pane(pane_id, surface, dep, env);
        window_level = window_level.worst(decision.fidelity);
        if decision.hydrate_touched {
            hydrated_pane_ids.push((*pane_id).to_string());
        }
        if decision.evidence_only {
            evidence_only_pane_ids.push((*pane_id).to_string());
        }
        if let Some(placeholder) = decision.placeholder {
            placeholder_results.push(placeholder);
        }
        if let Some(outcome) = decision.live_outcome {
            live_surface_outcomes.push(outcome);
        }
    }

    // A display adjustment alone keeps the layout but is not an exact restore.
    if !bounds_plan.adjustments.is_empty() {
        window_level = window_level.worst(RestoreLevel::CompatibleRestore);
    }

    // --- Phase: authority rebind ---
    let authority_rebind_result = resolve_authority(
        env,
        &snapshot.scope_refs.workspace_authority_ref,
        authority_required_missing,
    );
    if authority_rebind_result == AuthorityRebindResult::MissingAuthorityPlaceholder {
        window_level = window_level.worst(RestoreLevel::LayoutOnly);
    } else if authority_rebind_result == AuthorityRebindResult::DegradedLocalOnly {
        window_level = window_level.worst(RestoreLevel::CompatibleRestore);
    }

    let phase_trace = build_phase_trace(
        &all_pane_ids,
        &hydrated_pane_ids,
        &placeholder_results,
        &live_surface_outcomes,
        &evidence_only_pane_ids,
        authority_rebind_result,
    );

    let display_adjustments = bounds_plan
        .adjustments
        .iter()
        .map(|(class, note)| DisplayAdjustmentRecord {
            adjustment_class: *class,
            affected_pane_ids: all_pane_ids.clone(),
            note: note.clone(),
        })
        .collect();

    let focus_anchor_ref = snapshot
        .focus_chain
        .first()
        .map(|entry| entry.target_ref.clone())
        .unwrap_or_default();

    let provenance = LayoutRestoreProvenanceRecord {
        record_kind: LayoutRestoreProvenanceRecordKind::LayoutRestoreProvenanceRecord,
        pane_tree_schema_version: RESTORE_PANE_TREE_SCHEMA_VERSION,
        restore_event_id: format!(
            "restore-event:{}:{}",
            request.request_id, snapshot.window_id
        ),
        source_snapshot_ref: snapshot.snapshot_id.clone(),
        restore_source_class: request.restore_source_class,
        workspace_authority_ref: snapshot.scope_refs.workspace_authority_ref.clone(),
        window_id: snapshot.window_id.clone(),
        restore_level: window_level,
        phase_trace,
        authority_rebind_result,
        display_adjustments,
        placeholder_results,
        live_surface_outcomes,
        emitted_at: snapshot.emitted_at.clone(),
        notes: Some(
            "Window shell and pane topology were recreated before heavy hydration; missing \
             surfaces are placeholders that preserve their slots."
                .to_string(),
        ),
    };

    Ok(WindowRestoreResult {
        window_id: snapshot.window_id.clone(),
        window_role: snapshot.window_role,
        shell_restored: true,
        chosen_display_ref: bounds_plan.display_ref,
        applied_bounds: bounds_plan.bounds,
        preserved_pane_ids: all_pane_ids,
        focus_anchor_ref,
        provenance,
    })
}

fn decide_pane(
    pane_id: &str,
    surface: &PaneSurfaceDescriptor,
    dep: Option<&PaneDependency>,
    env: &RestoreEnvironment,
) -> PaneDecision {
    let available = dep.map(|dep| env.dependency_available(dep)).unwrap_or(true);
    let evidence = surface
        .placeholder_card
        .as_ref()
        .map(|card| card.evidence_retained)
        .unwrap_or(false);
    let last_known_label = surface
        .placeholder_card
        .as_ref()
        .and_then(|card| card.last_known_provenance_label.clone())
        .or_else(|| surface.title_hint.clone());

    match surface.live_surface_class {
        // ----- Non-live surfaces: lightweight skeleton or a placeholder slot -----
        None => {
            if available {
                // Lightweight surface restored as part of the skeleton.
                return PaneDecision {
                    fidelity: RestoreLevel::ExactRestore,
                    placeholder: None,
                    live_outcome: None,
                    hydrate_touched: surface.hydration_behavior == HydrationBehavior::LazyHeavy,
                    evidence_only: false,
                };
            }
            let dep = dep.expect("missing availability implies a dependency");
            let reason = dep.dependency_class.missing_reason();
            PaneDecision {
                fidelity: RestoreLevel::LayoutOnly,
                placeholder: Some(placeholder_result(
                    pane_id,
                    surface,
                    reason,
                    evidence,
                    last_known_label,
                )),
                live_outcome: None,
                hydrate_touched: true,
                evidence_only: false,
            }
        }
        // ----- Live or expensive surfaces: never auto-replayed -----
        Some(live_class) => {
            if !available {
                // Missing heavy dependency: reopen as a recoverable placeholder slot.
                let dep = dep.expect("missing availability implies a dependency");
                let reason = dep.dependency_class.missing_reason();
                PaneDecision {
                    fidelity: RestoreLevel::LayoutOnly,
                    placeholder: Some(placeholder_result(
                        pane_id,
                        surface,
                        reason,
                        evidence,
                        last_known_label,
                    )),
                    live_outcome: Some(LiveSurfaceOutcomeRecord {
                        pane_id: pane_id.to_string(),
                        live_surface_class: live_class,
                        restore_posture: SurfaceRestorePosture::PlaceholderUntilManualRebind,
                        authority_posture: authority_posture_for_missing(dep.dependency_class),
                        no_rerun_guardrails: guardrails_for_placeholder(live_class),
                        evidence_retained: evidence,
                        note: Some(format!(
                            "{} could not hydrate; pane slot preserved until manual rebind.",
                            reason.display_label()
                        )),
                    }),
                    hydrate_touched: true,
                    evidence_only: false,
                }
            } else if dep.map(|dep| dep.reattachable).unwrap_or(false) {
                // The session still exists and may be reattached without rerun.
                PaneDecision {
                    fidelity: RestoreLevel::CompatibleRestore,
                    placeholder: None,
                    live_outcome: Some(LiveSurfaceOutcomeRecord {
                        pane_id: pane_id.to_string(),
                        live_surface_class: live_class,
                        restore_posture: SurfaceRestorePosture::LiveAttachVisible,
                        authority_posture: SurfaceAuthorityPosture::ExistingAuthorityStillValid,
                        no_rerun_guardrails: vec![
                            NoRerunGuardrail::NoHiddenAuthorityReacquire,
                            NoRerunGuardrail::ExplicitUserActionRequired,
                        ],
                        evidence_retained: evidence,
                        note: Some(
                            "Existing session reattached and visible; live behavior resumes only \
                             on explicit user action."
                                .to_string(),
                        ),
                    }),
                    hydrate_touched: true,
                    evidence_only: false,
                }
            } else {
                // Dependency present but the live process is gone: reopen inert.
                let reason = PlaceholderReasonClass::NonReentrantLiveSurface;
                let posture = if evidence {
                    SurfaceRestorePosture::EvidenceOnlyPlaceholder
                } else {
                    SurfaceRestorePosture::MetadataOnlyPlaceholder
                };
                let fidelity = if evidence {
                    RestoreLevel::EvidenceOnly
                } else {
                    RestoreLevel::LayoutOnly
                };
                PaneDecision {
                    fidelity,
                    placeholder: Some(placeholder_result(
                        pane_id,
                        surface,
                        reason,
                        evidence,
                        last_known_label,
                    )),
                    live_outcome: Some(LiveSurfaceOutcomeRecord {
                        pane_id: pane_id.to_string(),
                        live_surface_class: live_class,
                        restore_posture: posture,
                        authority_posture: SurfaceAuthorityPosture::ManualRebindRequired,
                        no_rerun_guardrails: guardrails_for_inert(live_class, evidence),
                        evidence_retained: evidence,
                        note: Some(
                            "Live process did not survive restore; reopened without rerun."
                                .to_string(),
                        ),
                    }),
                    hydrate_touched: true,
                    evidence_only: evidence,
                }
            }
        }
    }
}

fn placeholder_result(
    pane_id: &str,
    surface: &PaneSurfaceDescriptor,
    reason: PlaceholderReasonClass,
    evidence: bool,
    last_known_label: Option<String>,
) -> PlaceholderResultRecord {
    PlaceholderResultRecord {
        pane_id: pane_id.to_string(),
        surface_role: surface.surface_role,
        surface_class: surface.surface_class,
        placeholder_reason: reason,
        safe_actions: safe_actions_for(reason),
        evidence_retained: evidence,
        last_known_provenance_label: last_known_label,
        note: Some(format!(
            "{} unavailable; pane slot preserved as a placeholder.",
            reason.display_label()
        )),
    }
}

fn authority_posture_for_missing(class: DependencyClass) -> SurfaceAuthorityPosture {
    match class {
        DependencyClass::RemoteAuthority | DependencyClass::Permission => {
            SurfaceAuthorityPosture::ReauthRequired
        }
        _ => SurfaceAuthorityPosture::ManualRebindRequired,
    }
}

fn guardrails_for_placeholder(live_class: LiveSurfaceClass) -> Vec<NoRerunGuardrail> {
    let mut guardrails = vec![
        NoRerunGuardrail::NoHiddenAuthorityReacquire,
        NoRerunGuardrail::ExplicitUserActionRequired,
        NoRerunGuardrail::PlaceholderPreserved,
    ];
    if live_class.command_bearing() {
        guardrails.insert(0, NoRerunGuardrail::NoCommandRerun);
    }
    guardrails
}

fn guardrails_for_inert(live_class: LiveSurfaceClass, evidence: bool) -> Vec<NoRerunGuardrail> {
    let mut guardrails = vec![
        NoRerunGuardrail::NoHiddenAuthorityReacquire,
        NoRerunGuardrail::ExplicitUserActionRequired,
    ];
    if evidence {
        guardrails.insert(0, NoRerunGuardrail::TranscriptOrSnapshotOnly);
    }
    if live_class.command_bearing() {
        guardrails.insert(0, NoRerunGuardrail::NoCommandRerun);
    }
    guardrails
}

fn safe_actions_for(reason: PlaceholderReasonClass) -> Vec<PlaceholderActionClass> {
    use PlaceholderActionClass::*;
    match reason {
        PlaceholderReasonClass::MissingExtension => {
            vec![
                InstallExtension,
                LocateExtension,
                ExportEvidence,
                RemovePane,
            ]
        }
        PlaceholderReasonClass::MissingRemote => {
            vec![ReconnectRemote, Reauthenticate, ExportEvidence, RemovePane]
        }
        PlaceholderReasonClass::MissingRemoteAuthority => {
            vec![Reauthenticate, ReconnectRemote, ExportEvidence]
        }
        PlaceholderReasonClass::RevokedPermission => {
            vec![Reauthenticate, OpenWithout, ExportEvidence]
        }
        PlaceholderReasonClass::UnsupportedDisplayTopology => {
            vec![RetryHydrate, OpenWithout, RemovePane]
        }
        PlaceholderReasonClass::NonReentrantLiveSurface => {
            vec![OpenWithout, RerunExplicitly, ExportEvidence, RemovePane]
        }
        PlaceholderReasonClass::SchemaMigrationReviewRequired => {
            vec![OpenWithout, ExportEvidence]
        }
        PlaceholderReasonClass::ManualRecoveryRequired => {
            vec![RetryHydrate, OpenWithout, ExportEvidence, RemovePane]
        }
    }
}

fn resolve_authority(
    env: &RestoreEnvironment,
    workspace_authority_ref: &str,
    required_missing: bool,
) -> AuthorityRebindResult {
    if required_missing {
        return AuthorityRebindResult::MissingAuthorityPlaceholder;
    }
    if !env.workspace_authority_present {
        return AuthorityRebindResult::DegradedLocalOnly;
    }
    if env.present_authority_refs.contains(workspace_authority_ref) {
        AuthorityRebindResult::BoundExistingAuthority
    } else {
        // Authority service is present but this exact ref was re-evaluated and bound.
        AuthorityRebindResult::ReevaluatedAndBound
    }
}

fn build_phase_trace(
    all_pane_ids: &[String],
    hydrated_pane_ids: &[String],
    placeholder_results: &[PlaceholderResultRecord],
    live_surface_outcomes: &[LiveSurfaceOutcomeRecord],
    evidence_only_pane_ids: &[String],
    authority_rebind_result: AuthorityRebindResult,
) -> Vec<RestorePhaseRecord> {
    let hydrate_degraded = !placeholder_results.is_empty()
        || live_surface_outcomes
            .iter()
            .any(|outcome| outcome.restore_posture.is_placeholder());

    let rebind_outcome = match authority_rebind_result {
        AuthorityRebindResult::BoundExistingAuthority
        | AuthorityRebindResult::ReevaluatedAndBound => PhaseOutcome::Completed,
        AuthorityRebindResult::DegradedLocalOnly => PhaseOutcome::Degraded,
        AuthorityRebindResult::MissingAuthorityPlaceholder => PhaseOutcome::Blocked,
    };

    let evidence_phase = if evidence_only_pane_ids.is_empty() {
        RestorePhaseRecord {
            phase: RestorePhase::EvidenceOnlyFallback,
            outcome: PhaseOutcome::Skipped,
            affected_pane_ids: Vec::new(),
            note: "No pane needed an evidence-only fallback.".to_string(),
        }
    } else {
        RestorePhaseRecord {
            phase: RestorePhase::EvidenceOnlyFallback,
            outcome: PhaseOutcome::ReroutedToEvidenceOnly,
            affected_pane_ids: evidence_only_pane_ids.to_vec(),
            note: "Heavy surfaces fell back to retained evidence; no live behavior resumed."
                .to_string(),
        }
    };

    vec![
        RestorePhaseRecord {
            phase: RestorePhase::Chooser,
            outcome: PhaseOutcome::Skipped,
            affected_pane_ids: Vec::new(),
            note: "Restore target resolved without a chooser prompt.".to_string(),
        },
        RestorePhaseRecord {
            phase: RestorePhase::Skeleton,
            outcome: PhaseOutcome::Completed,
            affected_pane_ids: all_pane_ids.to_vec(),
            note: "Window shell, pane topology, and focus anchors recreated first.".to_string(),
        },
        RestorePhaseRecord {
            phase: RestorePhase::Hydrate,
            outcome: if hydrate_degraded {
                PhaseOutcome::Degraded
            } else {
                PhaseOutcome::Completed
            },
            affected_pane_ids: hydrated_pane_ids.to_vec(),
            note: "Heavy dependencies hydrated lazily after the skeleton was visible.".to_string(),
        },
        RestorePhaseRecord {
            phase: RestorePhase::Rebind,
            outcome: rebind_outcome,
            affected_pane_ids: Vec::new(),
            note: "Workspace authority resolved without serializing or replaying live authority."
                .to_string(),
        },
        evidence_phase,
    ]
}

struct BoundsPlan {
    display_ref: String,
    bounds: Bounds,
    adjustments: Vec<(DisplayAdjustmentClass, String)>,
}

fn remap_bounds(snapshot: &WindowTopologySnapshot, env: &RestoreEnvironment) -> BoundsPlan {
    let affinity = &snapshot.monitor_affinity_hint;
    let mut adjustments = Vec::new();

    // Choose the destination display.
    let chosen = match affinity
        .last_known_display_ref
        .as_deref()
        .and_then(|display_ref| env.display(display_ref))
    {
        Some(display) => display,
        None => {
            adjustments.push((
                DisplayAdjustmentClass::MovedToPrimaryDisplay,
                "Saved display was not connected; window moved to the primary display.".to_string(),
            ));
            env.primary_display()
                .expect("primary display validated as connected")
        }
    };

    // Scale normalization.
    if let Some(preferred) = affinity.preferred_scale_bucket {
        if preferred != chosen.scale_bucket {
            adjustments.push((
                DisplayAdjustmentClass::ScaleNormalized,
                "Destination display used a different scale bucket; layout weights preserved."
                    .to_string(),
            ));
        }
    }

    // Bounds: keep the saved bounds when they fit, otherwise snap to safe bounds.
    let bounds = match affinity.preferred_bounds_hint {
        Some(saved) if saved.within(&chosen.visible_bounds) => saved,
        _ => {
            adjustments.push((
                DisplayAdjustmentClass::SnappedToSafeBounds,
                "Saved bounds were off-screen or invalid; window snapped into safe visible bounds."
                    .to_string(),
            ));
            centered_bounds(&chosen.visible_bounds)
        }
    };

    // Fullscreen handling.
    if snapshot.window_chrome_state.window_state == WindowState::Fullscreen
        && env.clear_fullscreen_on_restore
    {
        adjustments.push((
            DisplayAdjustmentClass::FullscreenCleared,
            "Fullscreen state was cleared so the window is reachable after restore.".to_string(),
        ));
    }

    BoundsPlan {
        display_ref: chosen.display_ref.clone(),
        bounds,
        adjustments,
    }
}

fn centered_bounds(visible: &Bounds) -> Bounds {
    let width = 800.min(visible.width.max(1));
    let height = 600.min(visible.height.max(1));
    let x = visible.x + (visible.width - width).max(0) / 2;
    let y = visible.y + (visible.height - height).max(0) / 2;
    Bounds {
        x,
        y,
        width,
        height,
    }
}

// ---------------------------------------------------------------------------
// Errors and helpers
// ---------------------------------------------------------------------------

/// Validation errors for restore-hydration records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreHydrationError {
    /// Record kind does not match.
    WrongRecordKind,
    /// Schema version is unsupported.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// A required field is empty.
    MissingField {
        /// Field path.
        field: &'static str,
    },
    /// The request has no windows.
    NoWindows,
    /// The environment has no connected displays.
    NoConnectedDisplays,
    /// The primary display ref is not in the connected-display set.
    PrimaryDisplayNotConnected {
        /// Display ref.
        display_ref: String,
    },
    /// A window id appears more than once.
    DuplicateWindowId {
        /// Window id.
        window_id: String,
    },
    /// A pane id appears more than once.
    DuplicatePaneId {
        /// Pane id.
        pane_id: String,
    },
    /// A window has no panes.
    WindowMissingPanes {
        /// Window id.
        window_id: String,
    },
    /// A window has no focus chain.
    WindowMissingFocusChain {
        /// Window id.
        window_id: String,
    },
    /// A dependency targets a pane that does not exist.
    DependencyForUnknownPane {
        /// Pane id.
        pane_id: String,
    },
    /// A restore outcome references a pane that was not preserved.
    OutcomeForUnknownPane {
        /// Pane id.
        pane_id: String,
    },
    /// A window shell was not restored.
    ShellNotRestored {
        /// Window id.
        window_id: String,
    },
    /// A window's final bounds would trap it off-screen.
    TrappedWindowBounds {
        /// Window id.
        window_id: String,
    },
    /// The skeleton phase did not complete.
    SkeletonNotCompleted {
        /// Window id.
        window_id: String,
    },
    /// The phase trace is missing one of the five named phases.
    PhaseTraceIncomplete {
        /// Window id.
        window_id: String,
    },
    /// A placeholder result has no safe action.
    PlaceholderMissingAction {
        /// Pane id.
        pane_id: String,
    },
    /// A live-surface outcome lacks the explicit no-rerun guardrail.
    LiveOutcomeMissingNoRerun {
        /// Pane id.
        pane_id: String,
    },
    /// A live-attach outcome failed to record an authority posture.
    LiveOutcomeMissingAuthority {
        /// Pane id.
        pane_id: String,
    },
    /// An exact restore carried a placeholder or evidence-only outcome.
    ExactWindowHasPlaceholder {
        /// Window id.
        window_id: String,
    },
    /// A display adjustment lost pane-id provenance.
    DisplayAdjustmentLostPaneIds {
        /// Window id.
        window_id: String,
    },
}

impl fmt::Display for RestoreHydrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind => write!(f, "wrong restore-hydration record kind"),
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::MissingField { field } => write!(f, "missing required field {field}"),
            Self::NoWindows => write!(f, "restore request has no windows"),
            Self::NoConnectedDisplays => write!(f, "restore environment has no connected displays"),
            Self::PrimaryDisplayNotConnected { display_ref } => {
                write!(f, "primary display {display_ref} is not connected")
            }
            Self::DuplicateWindowId { window_id } => write!(f, "duplicate window id {window_id}"),
            Self::DuplicatePaneId { pane_id } => write!(f, "duplicate pane id {pane_id}"),
            Self::WindowMissingPanes { window_id } => {
                write!(f, "window {window_id} has no panes")
            }
            Self::WindowMissingFocusChain { window_id } => {
                write!(f, "window {window_id} has no focus chain")
            }
            Self::DependencyForUnknownPane { pane_id } => {
                write!(f, "dependency targets unknown pane {pane_id}")
            }
            Self::OutcomeForUnknownPane { pane_id } => {
                write!(f, "restore outcome references unknown pane {pane_id}")
            }
            Self::ShellNotRestored { window_id } => {
                write!(f, "window {window_id} shell was not restored")
            }
            Self::TrappedWindowBounds { window_id } => {
                write!(f, "window {window_id} bounds would trap it off-screen")
            }
            Self::SkeletonNotCompleted { window_id } => {
                write!(f, "window {window_id} skeleton phase did not complete")
            }
            Self::PhaseTraceIncomplete { window_id } => {
                write!(f, "window {window_id} phase trace is incomplete")
            }
            Self::PlaceholderMissingAction { pane_id } => {
                write!(f, "placeholder for pane {pane_id} has no safe action")
            }
            Self::LiveOutcomeMissingNoRerun { pane_id } => {
                write!(
                    f,
                    "live outcome for pane {pane_id} lacks no-rerun guardrail"
                )
            }
            Self::LiveOutcomeMissingAuthority { pane_id } => {
                write!(
                    f,
                    "live-attach outcome for pane {pane_id} lacks authority posture"
                )
            }
            Self::ExactWindowHasPlaceholder { window_id } => {
                write!(f, "exact restore window {window_id} carried a placeholder")
            }
            Self::DisplayAdjustmentLostPaneIds { window_id } => {
                write!(f, "display adjustment in window {window_id} lost pane ids")
            }
        }
    }
}

impl std::error::Error for RestoreHydrationError {}

fn require_non_empty(field: &'static str, value: &str) -> Result<(), RestoreHydrationError> {
    if value.trim().is_empty() {
        return Err(RestoreHydrationError::MissingField { field });
    }
    Ok(())
}

fn join_labels<'a>(labels: impl Iterator<Item = &'a str>) -> String {
    let collected = labels.collect::<Vec<_>>();
    if collected.is_empty() {
        "none".to_string()
    } else {
        collected.join(",")
    }
}
