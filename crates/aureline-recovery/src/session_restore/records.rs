//! Schema-shaped session-restore record types.
//!
//! These record types mirror the boundary schemas for the restore artifact
//! family:
//!
//! - `schemas/state/workspace_authority_checkpoint.schema.json`
//! - `schemas/state/window_topology_snapshot.schema.json`
//! - `schemas/workspace/pane_tree.schema.json` (canonical pane-tree body)

use serde::{Deserialize, Serialize};

/// Schema version for workspace-authority checkpoint records.
pub type CheckpointSchemaVersion = u32;

/// Schema version for window-topology snapshot packets.
pub type TopologyPacketSchemaVersion = u32;

/// Schema version for canonical pane-tree bodies.
pub type PaneTreeSchemaVersion = u32;

/// Producer build stamp carried on restore artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerBuildStamp {
    pub producer_name: String,
    pub producer_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_platform_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_instance_handle: Option<String>,
}

/// Closed restore-fidelity vocabulary shared across restore artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClass {
    ExactRestore,
    CompatibleRestore,
    LayoutOnly,
    RecoveredDrafts,
    EvidenceOnly,
    NoRestore,
}

/// Downgrade triggers that explain narrowed restore fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTriggerClass {
    SchemaTranslationRequired,
    SchemaMeaningChanged,
    MissingExtensionDependency,
    MissingRemoteSession,
    MissingRemoteAuthority,
    UnsupportedDisplayTopology,
    ExcludedSecretMaterial,
    ExcludedLiveHandle,
    WorkspaceManifestConflict,
    PolicyNarrowing,
    ManualRepairRequired,
    ProducerSchemaDowngradeRefused,
}

/// Live-authority classes excluded from restore artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludedLiveAuthorityClass {
    RawSecretMaterial,
    LiveTokenOrCookie,
    DelegatedApprovalOrUnspentTicket,
    MachineUniqueHandle,
    LiveProcessOrSessionHandle,
    RawProviderPayload,
    RawUrlPathCommandOrLog,
    RawSourceOrUserContent,
    LiveRemoteOrKernelBinding,
}

/// Trusted-root entry captured for a workspace checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedRootRecord {
    pub root_id: String,
    pub trust_state: String,
    pub scope_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Dirty-buffer journal identity carried on a workspace checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyBufferJournalIdentity {
    pub journal_id: String,
    pub journal_kind: String,
    pub last_known_revision_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Downgrade trigger record included with restore artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeTriggerRecord {
    pub trigger_class: DowngradeTriggerClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_root_refs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_workset_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_pane_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Workspace-authority checkpoint record (schema/state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAuthorityCheckpointRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<serde_json::Value>,
    pub record_kind: String,
    pub checkpoint_schema_version: CheckpointSchemaVersion,
    pub checkpoint_id: String,
    pub workspace_authority_ref: String,
    pub producer_build: ProducerBuildStamp,
    pub source_schema_version: String,
    pub restore_class: RestoreClass,
    pub trusted_root_refs: Vec<TrustedRootRecord>,
    pub active_workset_ids: Vec<String>,
    pub dirty_buffer_journal_identities: Vec<DirtyBufferJournalIdentity>,
    pub recovery_journal_refs: Vec<String>,
    pub local_history_snapshot_refs: Vec<String>,
    pub evidence_bundle_refs: Vec<String>,
    pub excluded_live_authority_classes: Vec<ExcludedLiveAuthorityClass>,
    pub downgrade_triggers: Vec<DowngradeTriggerRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    pub preserved_prior_artifact_refs: Vec<String>,
    pub emitted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Window role within a topology family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRole {
    Primary,
    Auxiliary,
    Presentation,
    Review,
    Incident,
    Companion,
}

/// Surface role recorded on pane inventory entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceRole {
    Editor,
    Diff,
    Terminal,
    Debugger,
    Notebook,
    Search,
    Problems,
    Scm,
    Docs,
    Preview,
    AiPanel,
    Explorer,
    Test,
    CustomExtension,
    Placeholder,
}

/// Surface class recorded on pane inventory entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    TextEditor,
    DiffEditor,
    TerminalView,
    DebugView,
    NotebookView,
    SearchResults,
    ProblemsPanel,
    ScmPanel,
    DocsBrowser,
    PreviewCanvas,
    AiPanel,
    ExplorerTree,
    TestResults,
    ExtensionView,
    PlaceholderCard,
}

/// Hydration behavior hint for a pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HydrationBehavior {
    EagerLightweight,
    LazyHeavy,
    PlaceholderOnly,
    EvidenceOnly,
}

/// Availability state of a pane at capture time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityState {
    Ready,
    NeedsHydration,
    Placeholder,
    EvidenceOnly,
}

/// Stable pane inventory entry included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePaneInventoryEntry {
    pub pane_id: String,
    pub surface_role: SurfaceRole,
    pub surface_class: SurfaceClass,
    pub hydration_behavior: HydrationBehavior,
    pub availability_state: AvailabilityState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_spotlighted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_anchor_candidate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
}

/// Tab-group topology summary entry included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabGroupInventoryEntry {
    pub group_id: String,
    pub ordered_tab_ids: Vec<String>,
    pub active_tab_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tab_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_empty_group: Option<bool>,
}

/// Inspector kind inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorKind {
    Outline,
    Problems,
    Search,
    TerminalDetails,
    DebugVariables,
    NotebookVariables,
    AiEvidence,
    RestoreDiagnostics,
    PreviewDom,
    PropertyToken,
    CustomExtension,
}

/// Dock position hint for inspectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DockPosition {
    Left,
    Right,
    Bottom,
    Floating,
    Overlay,
}

/// Inspector inventory entry included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibleInspectorInventoryEntry {
    pub inspector_id: String,
    pub inspector_kind: InspectorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pane_ref: Option<String>,
    pub dock_position: DockPosition,
    pub visible: bool,
}

/// Focus-target kind recorded in focus chains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusTargetKind {
    Pane,
    Tab,
    Inspector,
    FollowBanner,
    WindowChrome,
}

/// Focus chain entry included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusChainEntry {
    pub target_kind: FocusTargetKind,
    pub target_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Follow mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowMode {
    Independent,
    FollowingParticipant,
    FollowingPresenter,
    BroadcastingFollow,
}

/// Presentation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationMode {
    Inactive,
    LocalPresenting,
    SharedPresenter,
    SharedAudience,
}

/// Collaboration role badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationBadge {
    Viewer,
    Editor,
    Driver,
    Observer,
    Presenter,
    CoPresenter,
    Approver,
    Scribe,
}

/// Follow/presentation state captured with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowPresentationState {
    pub follow_mode: FollowMode,
    pub presentation_mode: PresentationMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presenter_participant_ref: Option<String>,
    pub visible_role_badges: Vec<CollaborationBadge>,
    pub shared_control_badge_visible: bool,
    pub audience_breakaway_allowed: bool,
}

/// Monitor-affinity strength hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorAffinityStrength {
    None,
    PreferSameDisplay,
    PreferSameClass,
    PreferSameRegion,
}

/// Monitor affinity hint carried with topology packets and pane-tree bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorAffinityHint {
    pub affinity_strength: MonitorAffinityStrength,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_display_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_topology_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_scale_bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_bounds_hint: Option<String>,
    pub best_effort_only: bool,
}

/// Placeholder reason class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderReasonClass {
    MissingExtension,
    MissingRemote,
    MissingRemoteAuthority,
    RevokedPermission,
    UnsupportedDisplayTopology,
    NonReentrantLiveSurface,
    SchemaMigrationReviewRequired,
    ManualRecoveryRequired,
}

/// Placeholder safe action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderAction {
    RetryHydrate,
    LocateExtension,
    InstallExtension,
    Reauthenticate,
    ReconnectRemote,
    OpenWithout,
    OpenRestricted,
    RerunExplicitly,
    RebindExistingSession,
    ReflowToSafeBounds,
    CompareWithPreservedArtifact,
    OpenRepairInstructions,
    EscalateToManualRepair,
    ExportEvidence,
    RemovePane,
}

/// Placeholder behavior record included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderBehaviorRecord {
    pub pane_id: String,
    pub placeholder_reason: PlaceholderReasonClass,
    pub safe_actions: Vec<PlaceholderAction>,
    pub evidence_retained: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_provenance_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Topology adjustment class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyAdjustmentClass {
    SnappedToSafeBounds,
    MovedToPrimaryDisplay,
    ScaleNormalized,
    FullscreenCleared,
    StackingRepaired,
    RecenteredToVisibleRegion,
    RedockedToSafePane,
}

/// Topology adjustment record included with topology packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyAdjustmentRecord {
    pub adjustment_class: TopologyAdjustmentClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_pane_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Window-topology snapshot packet record (schema/state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowTopologySnapshotRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<serde_json::Value>,
    pub record_kind: String,
    pub topology_packet_schema_version: TopologyPacketSchemaVersion,
    pub snapshot_id: String,
    pub window_id: String,
    pub window_role: WindowRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_family_ref: Option<String>,
    pub sibling_window_refs: Vec<String>,
    pub producer_build: ProducerBuildStamp,
    pub source_schema_version: String,
    pub workspace_authority_checkpoint_ref: String,
    pub pane_tree_schema_version: PaneTreeSchemaVersion,
    pub pane_tree_record_ref: String,
    pub stable_pane_id_inventory: Vec<StablePaneInventoryEntry>,
    pub tab_group_topology: Vec<TabGroupInventoryEntry>,
    pub visible_inspectors: Vec<VisibleInspectorInventoryEntry>,
    pub focus_chain: Vec<FocusChainEntry>,
    pub follow_presentation_state: FollowPresentationState,
    pub monitor_affinity_hint: MonitorAffinityHint,
    pub placeholder_behaviors: Vec<PlaceholderBehaviorRecord>,
    pub topology_adjustments: Vec<TopologyAdjustmentRecord>,
    pub restore_class: RestoreClass,
    pub downgrade_triggers: Vec<DowngradeTriggerRecord>,
    pub emitted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Snapshot reason (schema/workspace).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotReason {
    GracefulShutdown,
    CrashRecoveryCheckpoint,
    ManualExport,
    DiagnosticCapture,
    SupportCapture,
}

/// Split orientation (schema/workspace).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// Placeholder-card payload attached to pane leaves (schema/workspace).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderCard {
    pub placeholder_class: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_action: Option<String>,
    pub secondary_actions: Vec<String>,
    pub note: String,
}

/// Surface payload attached to a leaf pane (schema/workspace).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneSurfaceDescriptor {
    pub surface_role: SurfaceRole,
    pub surface_class: SurfaceClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_surface_class: Option<String>,
    pub hydration_behavior: HydrationBehavior,
    pub availability_state: AvailabilityState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_binding_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_anchor_candidate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_spotlighted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_card: Option<PlaceholderCard>,
}

/// Leaf-pane node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneLeafNode {
    pub node_kind: String,
    pub pane_id: String,
    pub surface: PaneSurfaceDescriptor,
}

/// One tab in a tab group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabRecord {
    pub tab_id: String,
    pub tab_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dirty_badge_visible: Option<bool>,
    pub pane: PaneLeafNode,
}

/// Recursive pane-tree node (schema/workspace).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "node_kind", rename_all = "snake_case")]
pub enum PaneNode {
    Leaf { pane_id: String, surface: PaneSurfaceDescriptor },
    Split {
        split_id: String,
        orientation: SplitOrientation,
        children: Vec<PaneNode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        weights: Option<Vec<f64>>,
    },
    TabGroup {
        group_id: String,
        tabs: Vec<TabRecord>,
        active_tab_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        close_empty_group: Option<bool>,
    },
}

/// Pane-tree body (schema/workspace).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaneTree {
    pub tree_revision: u32,
    pub root_node: PaneNode,
}

/// Density preset (schema/workspace).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityPreset {
    Comfortable,
    Compact,
    Presentation,
}

/// Window state (schema/workspace).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowState {
    Normal,
    Maximized,
    Fullscreen,
    Zen,
    Minimized,
}

/// Window chrome state (schema/workspace).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowChromeState {
    pub window_state: WindowState,
    pub zoom_percent: f64,
    pub density_preset: DensityPreset,
    pub activity_strip_visible: bool,
    pub sidebar_visible: bool,
    pub bottom_panel_visible: bool,
}

/// Visible inspector record (schema/workspace).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibleInspectorRecord {
    pub inspector_id: String,
    pub inspector_kind: InspectorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pane_ref: Option<String>,
    pub dock_position: DockPosition,
    pub visible: bool,
}

/// Workspace scope references carried on pane-tree bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRefs {
    pub workspace_authority_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_defaults_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_display_hint_ref: Option<String>,
}

/// Canonical window-topology snapshot body (schema/workspace).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowTopologySnapshotBodyRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<serde_json::Value>,
    pub record_kind: String,
    pub pane_tree_schema_version: PaneTreeSchemaVersion,
    pub snapshot_id: String,
    pub snapshot_reason: SnapshotReason,
    pub window_id: String,
    pub window_role: WindowRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_family_ref: Option<String>,
    pub sibling_window_refs: Vec<String>,
    pub scope_refs: ScopeRefs,
    pub pane_tree: PaneTree,
    pub focus_chain: Vec<FocusChainEntry>,
    pub visible_inspectors: Vec<VisibleInspectorRecord>,
    pub follow_presentation_state: FollowPresentationState,
    pub window_chrome_state: WindowChromeState,
    pub monitor_affinity_hint: MonitorAffinityHint,
    pub emitted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
