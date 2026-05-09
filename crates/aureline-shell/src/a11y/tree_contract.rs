//! Accessibility-tree contract records used by the shell.
//!
//! These structs mirror the stable boundary schemas in
//! `schemas/accessibility/tree_node.schema.json`. They are intentionally
//! stringly-typed at the edges so the shell can emit fixtures and inspection
//! captures without depending on platform adapters.

use serde::{Deserialize, Serialize};

/// Canonical schema version for `accessibility_tree_node_record`.
pub const TREE_NODE_SCHEMA_VERSION: u32 = 1;

/// Canonical contract id for the accessibility-tree taxonomy.
pub const ACCESSIBILITY_TREE_CONTRACT_ID: &str = "aureline.accessibility.tree_node_taxonomy";

/// Canonical contract revision for the accessibility-tree taxonomy.
pub const ACCESSIBILITY_TREE_CONTRACT_REVISION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    ShellZone,
    EditorContent,
    EditorGutter,
    Diagnostics,
    InlineWidget,
    List,
    Tree,
    TableGrid,
    LogTerminal,
    Notebook,
    StatusNotification,
    DiffReview,
    DataSurface,
    GraphCanvasEquivalent,
    SupportExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    ApplicationRoot,
    WindowRoot,
    ShellZone,
    LandmarkRegion,
    Toolbar,
    Tablist,
    Tab,
    Splitter,
    Button,
    Link,
    MenuItem,
    Searchbox,
    TextInput,
    EditorDocument,
    EditorLine,
    EditorTextRun,
    EditorCaret,
    EditorSelectionRange,
    EditorGutterLane,
    EditorGutterMarker,
    DiagnosticMarker,
    DiagnosticRow,
    InlineWidget,
    ListContainer,
    ListRow,
    TreeContainer,
    TreeRow,
    TableGrid,
    Row,
    ColumnHeader,
    RowHeader,
    Cell,
    LogRegion,
    LogEntry,
    TerminalRegion,
    CommandBoundary,
    Notebook,
    NotebookCell,
    NotebookInput,
    NotebookOutput,
    OutputSummary,
    StatusBar,
    StatusItem,
    Notification,
    Banner,
    LiveRegion,
    ReviewHunk,
    Placeholder,
    SupportDowngradeNotice,
    UnknownDegradedSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericRole {
    Application,
    Window,
    Group,
    Landmark,
    Toolbar,
    Tablist,
    Tab,
    Button,
    Link,
    Menuitem,
    Searchbox,
    Textbox,
    Document,
    Code,
    Line,
    Text,
    List,
    Listitem,
    Tree,
    Treeitem,
    Table,
    Grid,
    Row,
    Rowheader,
    Columnheader,
    Cell,
    Gridcell,
    Log,
    Status,
    Alert,
    Progressbar,
    Separator,
    Checkbox,
    Switch,
    Combobox,
    Dialog,
    Note,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleSource {
    ExplicitSurfaceContract,
    CommandDescriptor,
    DocumentModel,
    CollectionModel,
    DiagnosticsModel,
    RendererSemanticHook,
    GeneratedSummary,
    FallbackSummary,
    PlatformAdapter,
    UnsupportedPlaceholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleConfidence {
    Exact,
    Degraded,
    SummaryOnly,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeRoleHints {
    pub uia: Option<String>,
    pub nsaccessibility: Option<String>,
    pub at_spi: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleMapping {
    pub generic_role: GenericRole,
    pub role_source: RoleSource,
    pub role_confidence: RoleConfidence,
    pub native_role_hints: NativeRoleHints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NameSource {
    VisibleLabel,
    CommandDescriptor,
    DocumentTitle,
    FileOrSymbolLabel,
    RowIdentity,
    ColumnHeader,
    DiagnosticMessageId,
    StatusMessageId,
    AnnouncementMessageId,
    AriaLabelEquivalent,
    GeneratedSummary,
    DegradedSupportLabel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderValueClass {
    BoundedLabel,
    Count,
    StateLabel,
    SupportClass,
    ActionLabel,
    SeverityLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderName {
    pub name: String,
    pub value_class: PlaceholderValueClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibleName {
    pub name: String,
    pub name_source: NameSource,
    pub message_id: Option<String>,
    pub placeholder_names: Vec<PlaceholderName>,
    pub description: Option<String>,
    pub description_message_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckedState {
    True,
    False,
    Mixed,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentState {
    None,
    Current,
    Active,
    Unavailable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityClass {
    None,
    Info,
    Warning,
    Error,
    Blocking,
    Security,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    Fresh,
    Warming,
    Cached,
    Stale,
    Degraded,
    Unavailable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateLabelClass {
    Severity,
    Freshness,
    Support,
    DisabledReason,
    ReadOnlyReason,
    SelectionScope,
    Virtualization,
    TrustOrPolicy,
    LiveRegion,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateLabel {
    pub state_class: StateLabelClass,
    pub label: String,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeStates {
    pub focused: bool,
    pub focusable: bool,
    pub selected: bool,
    pub current: CurrentState,
    pub disabled: bool,
    pub read_only: bool,
    pub expanded: Option<bool>,
    pub checked: CheckedState,
    pub busy: bool,
    pub invalid: bool,
    pub required: bool,
    pub modal: bool,
    pub visible: bool,
    pub offscreen: bool,
    pub virtualized: bool,
    pub stale: bool,
    pub degraded: bool,
    pub severity: SeverityClass,
    pub freshness: FreshnessClass,
    pub state_labels: Vec<StateLabel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexTruthClass {
    Exact,
    Estimated,
    Stale,
    Degraded,
    Unknown,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodePosition {
    pub position_in_set: Option<u32>,
    pub set_size: Option<u32>,
    pub row_index: Option<u32>,
    pub row_count: Option<u32>,
    pub column_index: Option<u32>,
    pub column_count: Option<u32>,
    pub level: Option<u32>,
    pub index_truth_class: IndexTruthClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeRelationships {
    pub parent_node_ref: Option<String>,
    pub child_node_refs: Vec<String>,
    pub labelled_by_refs: Vec<String>,
    pub described_by_refs: Vec<String>,
    pub controls_refs: Vec<String>,
    pub owns_refs: Vec<String>,
    pub active_descendant_ref: Option<String>,
    pub details_refs: Vec<String>,
    pub error_message_refs: Vec<String>,
    pub flow_to_refs: Vec<String>,
    pub source_anchor_refs: Vec<String>,
    pub related_node_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VirtualizationState {
    NotVirtualized,
    MountedVisible,
    MountedOffscreen,
    VirtualPlaceholder,
    UnmountedRepresented,
    SummaryForUnmounted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizedWindow {
    pub visible_start_index: Option<u32>,
    pub visible_end_index: Option<u32>,
    pub total_count: Option<u32>,
    pub index_truth_class: IndexTruthClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeVirtualization {
    pub virtualization_state: VirtualizationState,
    pub stable_item_ref: Option<String>,
    pub mounted_node_ref: Option<String>,
    pub visible_window: VirtualizedWindow,
    pub recycle_safe: bool,
    pub hidden_selected_count: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    Certified,
    Supported,
    Community,
    Experimental,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportState {
    FullAccessible,
    DegradedAccessible,
    SummaryOnly,
    InspectOnly,
    UnsupportedBlocked,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationReasonClass {
    None,
    PlatformBridgeUnavailable,
    MissingRole,
    MissingName,
    MissingState,
    MissingRelationship,
    VirtualizationTruthUnavailable,
    LiveRegionUnavailable,
    PlatformApiGap,
    ExtensionBoundary,
    ResourceProtectCore,
    PolicyOrTrustRestriction,
    UnsupportedSurface,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportStatus {
    pub support_class: SupportClass,
    pub support_state: SupportState,
    pub degradation_reason_classes: Vec<DegradationReasonClass>,
    pub user_visible_notice_required: bool,
    pub support_language: String,
    pub recovery_action_label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    PublicSafe,
    OperatorOnlyRestricted,
    MetadataOnly,
    SupportPacketRedacted,
    PolicyForbidden,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyPosture {
    pub redaction_class: RedactionClass,
    pub raw_private_material_excluded: bool,
    pub allowed_export_fields: Vec<String>,
    pub schema_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityTreeNodeRecord {
    pub record_kind: String,
    pub tree_node_schema_version: u32,
    pub accessibility_tree_contract_id: String,
    pub accessibility_tree_contract_revision: u32,
    pub node_id: String,
    pub tree_epoch_ref: String,
    pub surface_family: SurfaceFamily,
    pub node_kind: NodeKind,
    pub role_mapping: RoleMapping,
    pub accessible_name: AccessibleName,
    pub states: NodeStates,
    pub position: NodePosition,
    pub relationships: NodeRelationships,
    pub virtualization: NodeVirtualization,
    pub support_status: SupportStatus,
    pub privacy: PrivacyPosture,
    pub minted_at: String,
}

impl AccessibilityTreeNodeRecord {
    /// Returns a default privacy posture for export-safe shell fixtures.
    pub fn default_privacy_posture() -> PrivacyPosture {
        PrivacyPosture {
            redaction_class: RedactionClass::SupportPacketRedacted,
            raw_private_material_excluded: true,
            allowed_export_fields: vec![
                "node_id".to_string(),
                "role_mapping".to_string(),
                "accessible_name".to_string(),
                "states".to_string(),
                "position".to_string(),
                "relationships".to_string(),
                "virtualization".to_string(),
                "support_status".to_string(),
            ],
            schema_refs: vec!["schemas/accessibility/tree_node.schema.json".to_string()],
        }
    }

    /// Returns a default non-virtualized projection.
    pub fn default_virtualization() -> NodeVirtualization {
        NodeVirtualization {
            virtualization_state: VirtualizationState::NotVirtualized,
            stable_item_ref: None,
            mounted_node_ref: None,
            visible_window: VirtualizedWindow {
                visible_start_index: None,
                visible_end_index: None,
                total_count: None,
                index_truth_class: IndexTruthClass::NotApplicable,
            },
            recycle_safe: true,
            hidden_selected_count: None,
        }
    }

    /// Returns a default empty relationship block.
    pub fn empty_relationships(parent_node_ref: Option<String>) -> NodeRelationships {
        NodeRelationships {
            parent_node_ref,
            child_node_refs: Vec::new(),
            labelled_by_refs: Vec::new(),
            described_by_refs: Vec::new(),
            controls_refs: Vec::new(),
            owns_refs: Vec::new(),
            active_descendant_ref: None,
            details_refs: Vec::new(),
            error_message_refs: Vec::new(),
            flow_to_refs: Vec::new(),
            source_anchor_refs: Vec::new(),
            related_node_refs: Vec::new(),
        }
    }

    /// Returns a baseline state block for visible non-degraded nodes.
    pub fn baseline_states(focusable: bool) -> NodeStates {
        NodeStates {
            focused: false,
            focusable,
            selected: false,
            current: CurrentState::None,
            disabled: false,
            read_only: false,
            expanded: Some(true),
            checked: CheckedState::NotApplicable,
            busy: false,
            invalid: false,
            required: false,
            modal: false,
            visible: true,
            offscreen: false,
            virtualized: false,
            stale: false,
            degraded: false,
            severity: SeverityClass::None,
            freshness: FreshnessClass::Fresh,
            state_labels: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionModel {
    None,
    Single,
    Multi,
    Range,
    EditorText,
    TableCells,
    NotebookCells,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionAssertion {
    pub selection_model: SelectionModel,
    pub selected_node_refs: Vec<String>,
    pub anchor_node_ref: Option<String>,
    pub active_node_ref: Option<String>,
    pub visible_selected_count: Option<u32>,
    pub hidden_selected_count: Option<u32>,
    pub selection_truth_class: IndexTruthClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualizedTruthCheck {
    pub container_node_ref: String,
    pub expected_total_count: Option<u32>,
    pub expected_visible_start_index: Option<u32>,
    pub expected_visible_end_index: Option<u32>,
    pub index_truth_class: IndexTruthClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeExpectation {
    pub node_ref: String,
    pub support_class: SupportClass,
    pub support_state: SupportState,
    pub degradation_reason_classes: Vec<DegradationReasonClass>,
    pub user_visible_notice_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseAssertions {
    pub required_node_refs: Vec<String>,
    pub focus_chain_node_refs: Vec<String>,
    pub selection_state: SelectionAssertion,
    pub virtualized_truth_checks: Vec<VirtualizedTruthCheck>,
    pub downgrade_expectations: Vec<DowngradeExpectation>,
    pub raw_private_material_excluded: bool,
}

impl CaseAssertions {}

/// A fixture-safe case record containing a snapshot of the accessibility tree and assertions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityTreeCaseRecord {
    pub record_kind: String,
    pub tree_node_schema_version: u32,
    pub accessibility_tree_contract_id: String,
    pub accessibility_tree_contract_revision: u32,
    pub case_id: String,
    pub title: String,
    pub scenario: String,
    pub contract_refs: Vec<String>,
    pub root_node_id: String,
    pub nodes: Vec<AccessibilityTreeNodeRecord>,
    pub assertions: CaseAssertions,
    pub minted_at: String,
}
