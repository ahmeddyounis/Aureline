//! Shell accessibility-tree bridge.
//!
//! The bridge converts live shell state into `accessibility_tree_node_record`
//! instances (see `schemas/accessibility/tree_node.schema.json`) so fixtures and
//! early review captures can reason about shell semantics without scraping paint
//! output.

use std::collections::HashMap;
use std::path::PathBuf;

use aureline_commands::{CommandEnablementContext, CommandRegistry, EnablementDecisionClass};

use crate::app_frame::desktop_frame::DesktopFrame;
use crate::embedded::boundary_card::EmbeddedBoundaryCardRecord;
use crate::layout::zone_registry::ShellZoneId;
use crate::palette::{CommandPaletteState, PaletteItemKey};
use crate::start_center::{
    build_action_rows as start_center_action_rows, StartCenterRuntimeInputs, StartCenterState,
};

use super::tree_contract::{
    AccessibilityTreeNodeRecord, AccessibleName, DegradationReasonClass, GenericRole,
    IndexTruthClass, NameSource, NativeRoleHints, NodeKind, NodePosition, NodeStates,
    RoleConfidence, RoleMapping, RoleSource, StateLabel, StateLabelClass, SupportClass,
    SupportState, SupportStatus, SurfaceFamily, ACCESSIBILITY_TREE_CONTRACT_ID,
    ACCESSIBILITY_TREE_CONTRACT_REVISION, TREE_NODE_SCHEMA_VERSION,
};

/// Minimal runtime context for evaluating command enablement in accessibility projections.
#[derive(Debug, Clone, Copy)]
pub struct ShellA11yEnablementContext<'a> {
    pub client_scope: &'a str,
    pub workspace_trust_state: &'a str,
    pub execution_context_available: bool,
    pub provider_linked: Option<bool>,
    pub credential_available: Option<bool>,
    pub policy_disabled: bool,
    pub policy_blocked_in_context: bool,
    pub labs_enabled: bool,
}

impl<'a> ShellA11yEnablementContext<'a> {}

/// Materialized accessibility-tree snapshot for the current shell state.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShellA11yTreeRecord {
    pub root_node_id: String,
    pub tree_epoch_ref: String,
    pub minted_at: String,
    pub nodes: Vec<AccessibilityTreeNodeRecord>,
}

impl ShellA11yTreeRecord {}

const DEFAULT_TREE_EPOCH_REF: &str = "tree.epoch.shell.bridge.1";
const ROOT_NODE_ID: &str = "node.app.root";
const SHELL_A11Y_LOG_DIR: &str = "accessibility_trees";

/// Writes a shell accessibility-tree snapshot into `.logs/accessibility_trees/`.
pub fn write_shell_accessibility_tree_log(record: &ShellA11yTreeRecord) {
    let root = PathBuf::from(".logs").join(SHELL_A11Y_LOG_DIR);
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }

    let filename = format!(
        "{}.shell_accessibility_tree.json",
        sanitize_for_node_id(&record.minted_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

/// Builds an export-safe accessibility-tree mapping for the core shell surfaces.
pub fn materialize_shell_accessibility_tree(
    registry: &CommandRegistry,
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    start_center: &StartCenterState,
    docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
    enablement: ShellA11yEnablementContext<'_>,
) -> ShellA11yTreeRecord {
    let minted_at = aureline_commands::invocation::now_rfc3339();
    let tree_epoch_ref = DEFAULT_TREE_EPOCH_REF.to_string();

    let mut nodes: Vec<AccessibilityTreeNodeRecord> = Vec::new();
    let mut root_children: Vec<String> = Vec::new();

    let mut root = base_node(
        ROOT_NODE_ID,
        &tree_epoch_ref,
        SurfaceFamily::ShellZone,
        NodeKind::ApplicationRoot,
        RoleMapping {
            generic_role: GenericRole::Application,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Window".to_string()),
                nsaccessibility: Some("AXApplication".to_string()),
                at_spi: Some("application".to_string()),
            },
        },
        AccessibleName {
            name: "Aureline shell window".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.shell.window".to_string()),
            placeholder_names: Vec::new(),
            description: Some("Root accessibility tree for the active shell window.".to_string()),
            description_message_id: Some("a11y.tree.shell.window.description".to_string()),
        },
        NodeStates {
            expanded: Some(true),
            ..AccessibilityTreeNodeRecord::baseline_states(false)
        },
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: None,
            index_truth_class: IndexTruthClass::NotApplicable,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );

    // Zones (stable ordering, but include only visible zones per layout).
    let mut zone_node_ids: Vec<String> = Vec::new();
    for zone in ShellZoneId::ALL {
        let zone = *zone;
        if frame.layout().zone(zone).is_none() {
            continue;
        }

        let (zone_node_id, zone_name, zone_description, zone_role) = zone_identity(zone);
        let mut zone_node = base_node(
            zone_node_id,
            &tree_epoch_ref,
            SurfaceFamily::ShellZone,
            NodeKind::ShellZone,
            zone_role,
            AccessibleName {
                name: zone_name.to_string(),
                name_source: NameSource::VisibleLabel,
                message_id: Some(format!("a11y.tree.shell.zone.{}", zone.name())),
                placeholder_names: Vec::new(),
                description: Some(zone_description.to_string()),
                description_message_id: Some(format!(
                    "a11y.tree.shell.zone.{}.description",
                    zone.name()
                )),
            },
            AccessibilityTreeNodeRecord::baseline_states(true),
            NodePosition {
                position_in_set: Some((zone_node_ids.len() as u32).saturating_add(1)),
                set_size: None,
                row_index: None,
                row_count: None,
                column_index: None,
                column_count: None,
                level: Some(1),
                index_truth_class: IndexTruthClass::Exact,
            },
            SupportStatus {
                support_class: SupportClass::Supported,
                support_state: SupportState::FullAccessible,
                degradation_reason_classes: vec![DegradationReasonClass::None],
                user_visible_notice_required: false,
                support_language: "Supported".to_string(),
                recovery_action_label: None,
            },
            minted_at.clone(),
        );
        zone_node.relationships.parent_node_ref = Some(ROOT_NODE_ID.to_string());

        let mut child_refs: Vec<String> = Vec::new();

        // Docs/help boundary chrome (seeded card lives in the inspector slot).
        if zone == ShellZoneId::RightInspector {
            let docs_node_id = "node.embedded.docs_help.boundary_card";
            child_refs.push(docs_node_id.to_string());
            nodes.push(materialize_docs_help_boundary(
                docs_node_id,
                &tree_epoch_ref,
                docs_help_boundary_card,
                minted_at.clone(),
            ));
        }

        // Tool-panel placeholder (terminal seed) lives in the bottom panel slot.
        if zone == ShellZoneId::BottomPanel {
            let terminal_node_id = "node.panel.terminal.placeholder";
            child_refs.push(terminal_node_id.to_string());
            nodes.push(materialize_terminal_placeholder(
                terminal_node_id,
                &tree_epoch_ref,
                minted_at.clone(),
            ));
        }

        // Main workspace Start Center when the focused editor group is empty.
        if zone == ShellZoneId::MainWorkspace && focused_group_is_empty(frame) && !palette.is_open()
        {
            let start_center_region_id = "node.start_center.region";
            child_refs.push(start_center_region_id.to_string());
            let start_center_nodes = materialize_start_center_nodes(
                registry,
                start_center,
                enablement,
                &tree_epoch_ref,
                start_center_region_id,
                minted_at.clone(),
            );
            nodes.extend(start_center_nodes.nodes);
            zone_node.relationships.active_descendant_ref =
                start_center_nodes.active_descendant_ref;
        }

        zone_node.relationships.child_node_refs = child_refs.clone();
        zone_node.relationships.owns_refs = child_refs;
        nodes.push(zone_node);
        zone_node_ids.push(zone_node_id.to_string());
        root_children.push(zone_node_id.to_string());
    }

    // Transient overlays (command palette) are modeled as children of the overlay zone.
    if palette.is_open() {
        let overlay_zone_id = "node.shell.zone.transient_overlay";
        if !zone_node_ids.iter().any(|id| id == overlay_zone_id) {
            // If the overlay zone is not present in the visible layout, attach palette to root.
            let palette_nodes = materialize_palette_nodes(
                registry,
                shortcuts_by_command_id,
                palette,
                enablement,
                &tree_epoch_ref,
                ROOT_NODE_ID,
                minted_at.clone(),
            );
            root_children.extend(palette_nodes.child_refs.clone());
            root.relationships
                .owns_refs
                .extend(palette_nodes.owns_refs.clone());
            root.relationships.active_descendant_ref = palette_nodes.active_descendant_ref.clone();
            nodes.extend(palette_nodes.nodes);
        } else {
            let palette_nodes = materialize_palette_nodes(
                registry,
                shortcuts_by_command_id,
                palette,
                enablement,
                &tree_epoch_ref,
                overlay_zone_id,
                minted_at.clone(),
            );

            for node in nodes.iter_mut() {
                if node.node_id == overlay_zone_id {
                    node.relationships
                        .child_node_refs
                        .extend(palette_nodes.child_refs.clone());
                    node.relationships
                        .owns_refs
                        .extend(palette_nodes.owns_refs.clone());
                    node.relationships.active_descendant_ref =
                        palette_nodes.active_descendant_ref.clone();
                }
            }
            root.relationships.active_descendant_ref = palette_nodes.active_descendant_ref.clone();
            nodes.extend(palette_nodes.nodes);
        }
    }

    // Root node children are zones plus any transient overlays when present.
    root.relationships.child_node_refs = root_children;
    root.relationships.owns_refs = root.relationships.child_node_refs.clone();
    root.relationships.active_descendant_ref =
        active_descendant_for_shell(frame, palette, start_center);
    nodes.insert(0, root);

    ShellA11yTreeRecord {
        root_node_id: ROOT_NODE_ID.to_string(),
        tree_epoch_ref,
        minted_at,
        nodes,
    }
}

fn base_node(
    node_id: &str,
    tree_epoch_ref: &str,
    surface_family: SurfaceFamily,
    node_kind: NodeKind,
    role_mapping: RoleMapping,
    accessible_name: AccessibleName,
    states: NodeStates,
    position: NodePosition,
    support_status: SupportStatus,
    minted_at: String,
) -> AccessibilityTreeNodeRecord {
    AccessibilityTreeNodeRecord {
        record_kind: "accessibility_tree_node_record".to_string(),
        tree_node_schema_version: TREE_NODE_SCHEMA_VERSION,
        accessibility_tree_contract_id: ACCESSIBILITY_TREE_CONTRACT_ID.to_string(),
        accessibility_tree_contract_revision: ACCESSIBILITY_TREE_CONTRACT_REVISION,
        node_id: node_id.to_string(),
        tree_epoch_ref: tree_epoch_ref.to_string(),
        surface_family,
        node_kind,
        role_mapping,
        accessible_name,
        states,
        position,
        relationships: AccessibilityTreeNodeRecord::empty_relationships(None),
        virtualization: AccessibilityTreeNodeRecord::default_virtualization(),
        support_status,
        privacy: AccessibilityTreeNodeRecord::default_privacy_posture(),
        minted_at,
    }
}

fn zone_identity(zone: ShellZoneId) -> (&'static str, &'static str, &'static str, RoleMapping) {
    match zone {
        ShellZoneId::TitleContextBar => (
            "node.shell.zone.title_context_bar",
            "Title and context bar",
            "Top-level chrome that exposes identity, workspace context, and primary window actions.",
            RoleMapping {
                generic_role: GenericRole::Toolbar,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.ToolBar".to_string()),
                    nsaccessibility: Some("AXToolbar".to_string()),
                    at_spi: Some("toolbar".to_string()),
                },
            },
        ),
        ShellZoneId::ActivityRail => (
            "node.shell.zone.activity_rail",
            "Activity rail",
            "Primary navigation rail for switching major shell areas.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
        ShellZoneId::LeftSidebar => (
            "node.shell.zone.left_sidebar",
            "Left sidebar",
            "Secondary navigation and context panels adjacent to the main workspace.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
        ShellZoneId::MainWorkspace => (
            "node.shell.zone.main_workspace",
            "Main workspace",
            "Dominant task surface containing the focused editor group or Start Center entry surface.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
        ShellZoneId::RightInspector => (
            "node.shell.zone.right_inspector",
            "Right inspector",
            "Contextual detail inspector and embedded boundary chrome.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
        ShellZoneId::BottomPanel => (
            "node.shell.zone.bottom_panel",
            "Bottom panel",
            "Tool panels such as tasks and terminal output.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
        ShellZoneId::StatusBar => (
            "node.shell.zone.status_bar",
            "Status bar",
            "Status and recovery surface for the active shell session.",
            RoleMapping {
                generic_role: GenericRole::Status,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.StatusBar".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("status bar".to_string()),
                },
            },
        ),
        ShellZoneId::TransientOverlay => (
            "node.shell.zone.transient_overlay",
            "Transient overlay",
            "Transient overlays such as the command palette or sheets.",
            RoleMapping {
                generic_role: GenericRole::Landmark,
                role_source: RoleSource::ExplicitSurfaceContract,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Pane".to_string()),
                    nsaccessibility: Some("AXGroup".to_string()),
                    at_spi: Some("landmark".to_string()),
                },
            },
        ),
    }
}

fn focused_group_is_empty(frame: &DesktopFrame) -> bool {
    frame
        .editor_group_layouts()
        .iter()
        .find(|layout| layout.group_id == frame.focused_editor_group())
        .is_some_and(|layout| layout.tab_count == 0)
}

fn active_descendant_for_shell(
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    start_center: &StartCenterState,
) -> Option<String> {
    if palette.is_open() {
        return Some("node.palette.searchbox".to_string());
    }
    if frame.focused_zone() == ShellZoneId::MainWorkspace && focused_group_is_empty(frame) {
        return Some(format!(
            "node.start_center.action.{}",
            start_center.selection().saturating_add(1)
        ));
    }
    Some(format!("node.shell.zone.{}", frame.focused_zone().name()))
}

#[derive(Debug)]
struct StartCenterNodeSet {
    nodes: Vec<AccessibilityTreeNodeRecord>,
    active_descendant_ref: Option<String>,
}

fn materialize_start_center_nodes(
    registry: &CommandRegistry,
    start_center: &StartCenterState,
    enablement: ShellA11yEnablementContext<'_>,
    tree_epoch_ref: &str,
    region_node_id: &str,
    minted_at: String,
) -> StartCenterNodeSet {
    let runtime = StartCenterRuntimeInputs {
        client_scope: enablement.client_scope,
        workspace_trust_state: enablement.workspace_trust_state,
        execution_context_available: enablement.execution_context_available,
        provider_linked: enablement.provider_linked,
        credential_available: enablement.credential_available,
        policy_disabled: enablement.policy_disabled,
        policy_blocked_in_context: enablement.policy_blocked_in_context,
        labs_enabled: enablement.labs_enabled,
    };
    let rows = start_center_action_rows(registry, runtime);
    let selected = start_center.selection().min(rows.len().saturating_sub(1));

    let mut nodes = Vec::new();
    let mut region = base_node(
        region_node_id,
        tree_epoch_ref,
        SurfaceFamily::ShellZone,
        NodeKind::LandmarkRegion,
        RoleMapping {
            generic_role: GenericRole::Landmark,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Pane".to_string()),
                nsaccessibility: Some("AXGroup".to_string()),
                at_spi: Some("landmark".to_string()),
            },
        },
        AccessibleName {
            name: "Start Center".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.start_center.region".to_string()),
            placeholder_names: Vec::new(),
            description: Some(
                "Project entry surface with governed open/restore/import actions.".to_string(),
            ),
            description_message_id: Some("a11y.tree.start_center.region.description".to_string()),
        },
        AccessibilityTreeNodeRecord::baseline_states(true),
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(2),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );
    region.relationships.parent_node_ref = Some("node.shell.zone.main_workspace".to_string());

    let list_node_id = "node.start_center.action_list";
    region
        .relationships
        .child_node_refs
        .push(list_node_id.to_string());
    region
        .relationships
        .owns_refs
        .push(list_node_id.to_string());
    region.relationships.active_descendant_ref =
        Some(format!("node.start_center.action.{}", selected + 1));
    nodes.push(region);

    let mut list = base_node(
        list_node_id,
        tree_epoch_ref,
        SurfaceFamily::List,
        NodeKind::ListContainer,
        RoleMapping {
            generic_role: GenericRole::List,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.List".to_string()),
                nsaccessibility: Some("AXList".to_string()),
                at_spi: Some("list".to_string()),
            },
        },
        AccessibleName {
            name: "Start Center actions".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.start_center.actions".to_string()),
            placeholder_names: Vec::new(),
            description: Some(
                "Primary project entry actions resolved through the command registry.".to_string(),
            ),
            description_message_id: Some("a11y.tree.start_center.actions.description".to_string()),
        },
        AccessibilityTreeNodeRecord::baseline_states(true),
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: Some(rows.len() as u32),
            column_index: None,
            column_count: None,
            level: Some(3),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );
    list.relationships.parent_node_ref = Some(region_node_id.to_string());
    list.relationships.active_descendant_ref =
        Some(format!("node.start_center.action.{}", selected + 1));

    for (idx, row) in rows.iter().enumerate() {
        let node_id = format!("node.start_center.action.{}", idx + 1);
        list.relationships.child_node_refs.push(node_id.clone());
        list.relationships.owns_refs.push(node_id.clone());

        let (enabled, disabled_reason) = start_center_action_enablement(row, enablement);
        let (support_class, support_state, degraded, degradation_reason) = if enabled {
            (
                SupportClass::Supported,
                SupportState::FullAccessible,
                false,
                DegradationReasonClass::None,
            )
        } else {
            (
                SupportClass::Supported,
                SupportState::DegradedAccessible,
                true,
                DegradationReasonClass::PolicyOrTrustRestriction,
            )
        };

        let mut states = AccessibilityTreeNodeRecord::baseline_states(true);
        states.selected = idx == selected;
        states.focused = idx == selected;
        states.degraded = degraded;
        if !enabled {
            states.disabled = true;
            if let Some(reason) = disabled_reason.as_deref() {
                states.state_labels.push(StateLabel {
                    state_class: StateLabelClass::DisabledReason,
                    label: reason.to_string(),
                    message_id: None,
                });
            }
        }

        let mut node = base_node(
            &node_id,
            tree_epoch_ref,
            SurfaceFamily::List,
            NodeKind::ListRow,
            RoleMapping {
                generic_role: GenericRole::Button,
                role_source: RoleSource::CommandDescriptor,
                role_confidence: RoleConfidence::Exact,
                native_role_hints: NativeRoleHints {
                    uia: Some("UIA.Button".to_string()),
                    nsaccessibility: Some("AXButton".to_string()),
                    at_spi: Some("push button".to_string()),
                },
            },
            AccessibleName {
                name: row.title.to_string(),
                name_source: NameSource::VisibleLabel,
                message_id: Some(format!(
                    "a11y.tree.start_center.action.{}",
                    row.action_id.token()
                )),
                placeholder_names: Vec::new(),
                description: Some(row.summary.to_string()),
                description_message_id: None,
            },
            states,
            NodePosition {
                position_in_set: Some((idx + 1) as u32),
                set_size: Some(rows.len() as u32),
                row_index: Some((idx + 1) as u32),
                row_count: Some(rows.len() as u32),
                column_index: None,
                column_count: None,
                level: Some(4),
                index_truth_class: IndexTruthClass::Exact,
            },
            SupportStatus {
                support_class,
                support_state,
                degradation_reason_classes: vec![degradation_reason],
                user_visible_notice_required: !enabled,
                support_language: if enabled {
                    "Supported".to_string()
                } else {
                    "Degraded".to_string()
                },
                recovery_action_label: None,
            },
            minted_at.clone(),
        );
        node.relationships.parent_node_ref = Some(list_node_id.to_string());
        node.relationships
            .source_anchor_refs
            .push(row.command_id.to_string());
        nodes.push(node);
    }

    nodes.push(list);

    StartCenterNodeSet {
        nodes,
        active_descendant_ref: Some(format!("node.start_center.action.{}", selected + 1)),
    }
}

fn start_center_action_enablement(
    row: &crate::start_center::StartCenterActionRow,
    enablement: ShellA11yEnablementContext<'_>,
) -> (bool, Option<String>) {
    let Some(preflight) = row.preflight.as_ref() else {
        return (true, None);
    };
    if preflight.decision_class == aureline_commands::PreflightDecisionClass::Allowed {
        return (true, None);
    }

    if enablement.policy_blocked_in_context || enablement.policy_disabled {
        return (false, Some("blocked by policy".to_string()));
    }
    if !enablement.execution_context_available {
        return (false, Some("execution context unavailable".to_string()));
    }
    if enablement.workspace_trust_state != "trusted" {
        return (false, Some("restricted workspace trust state".to_string()));
    }
    (false, Some("unavailable".to_string()))
}

fn materialize_docs_help_boundary(
    node_id: &str,
    tree_epoch_ref: &str,
    docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
    minted_at: String,
) -> AccessibilityTreeNodeRecord {
    let title = if docs_help_boundary_card.owner_identity.label.is_empty() {
        "Docs and help".to_string()
    } else {
        docs_help_boundary_card.owner_identity.label.clone()
    };
    let summary = if docs_help_boundary_card.plain_language_summary.is_empty() {
        "Embedded docs/help surface boundary chrome."
    } else {
        docs_help_boundary_card.plain_language_summary.as_str()
    };

    let mut node = base_node(
        node_id,
        tree_epoch_ref,
        SurfaceFamily::ShellZone,
        NodeKind::LandmarkRegion,
        RoleMapping {
            generic_role: GenericRole::Landmark,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Pane".to_string()),
                nsaccessibility: Some("AXGroup".to_string()),
                at_spi: Some("landmark".to_string()),
            },
        },
        AccessibleName {
            name: title,
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.docs_help.boundary".to_string()),
            placeholder_names: Vec::new(),
            description: Some(summary.to_string()),
            description_message_id: Some("a11y.tree.docs_help.boundary.description".to_string()),
        },
        AccessibilityTreeNodeRecord::baseline_states(true),
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(2),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at,
    );
    node.relationships.parent_node_ref = Some("node.shell.zone.right_inspector".to_string());
    node
}

fn materialize_terminal_placeholder(
    node_id: &str,
    tree_epoch_ref: &str,
    minted_at: String,
) -> AccessibilityTreeNodeRecord {
    let mut states = AccessibilityTreeNodeRecord::baseline_states(true);
    states.read_only = true;
    states.degraded = true;
    states.state_labels.push(StateLabel {
        state_class: StateLabelClass::Support,
        label: "terminal placeholder".to_string(),
        message_id: Some("a11y.state.terminal.placeholder".to_string()),
    });

    let mut node = base_node(
        node_id,
        tree_epoch_ref,
        SurfaceFamily::LogTerminal,
        NodeKind::TerminalRegion,
        RoleMapping {
            generic_role: GenericRole::Log,
            role_source: RoleSource::UnsupportedPlaceholder,
            role_confidence: RoleConfidence::Degraded,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Document".to_string()),
                nsaccessibility: Some("AXTextArea".to_string()),
                at_spi: Some("terminal".to_string()),
            },
        },
        AccessibleName {
            name: "Terminal (placeholder)".to_string(),
            name_source: NameSource::DegradedSupportLabel,
            message_id: Some("a11y.tree.terminal.placeholder".to_string()),
            placeholder_names: Vec::new(),
            description: Some(
                "Terminal output surface placeholder; full semantics not yet available."
                    .to_string(),
            ),
            description_message_id: Some("a11y.tree.terminal.placeholder.description".to_string()),
        },
        states,
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(2),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Experimental,
            support_state: SupportState::SummaryOnly,
            degradation_reason_classes: vec![DegradationReasonClass::UnsupportedSurface],
            user_visible_notice_required: true,
            support_language: "Experimental".to_string(),
            recovery_action_label: Some("Open task output in a supported surface".to_string()),
        },
        minted_at,
    );
    node.relationships.parent_node_ref = Some("node.shell.zone.bottom_panel".to_string());
    node
}

#[derive(Debug)]
struct PaletteNodeSet {
    nodes: Vec<AccessibilityTreeNodeRecord>,
    child_refs: Vec<String>,
    owns_refs: Vec<String>,
    active_descendant_ref: Option<String>,
}

fn materialize_palette_nodes(
    registry: &CommandRegistry,
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    palette: &CommandPaletteState,
    enablement: ShellA11yEnablementContext<'_>,
    tree_epoch_ref: &str,
    parent_node_id: &str,
    minted_at: String,
) -> PaletteNodeSet {
    let mut nodes = Vec::new();
    let palette_root_id = "node.palette.dialog";
    let searchbox_id = "node.palette.searchbox";
    let results_id = "node.palette.results";

    let mut dialog_states = AccessibilityTreeNodeRecord::baseline_states(true);
    dialog_states.modal = true;
    let mut dialog = base_node(
        palette_root_id,
        tree_epoch_ref,
        SurfaceFamily::ShellZone,
        NodeKind::LandmarkRegion,
        RoleMapping {
            generic_role: GenericRole::Dialog,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Window".to_string()),
                nsaccessibility: Some("AXWindow".to_string()),
                at_spi: Some("dialog".to_string()),
            },
        },
        AccessibleName {
            name: "Command palette".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.palette.dialog".to_string()),
            placeholder_names: Vec::new(),
            description: Some(
                "Search and invoke commands through the governed command palette surface."
                    .to_string(),
            ),
            description_message_id: Some("a11y.tree.palette.dialog.description".to_string()),
        },
        dialog_states,
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(2),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );
    dialog.relationships.parent_node_ref = Some(parent_node_id.to_string());
    dialog.relationships.child_node_refs = vec![searchbox_id.to_string(), results_id.to_string()];
    dialog.relationships.owns_refs = dialog.relationships.child_node_refs.clone();
    dialog.relationships.active_descendant_ref = Some(searchbox_id.to_string());
    nodes.push(dialog);

    let mut search_states = AccessibilityTreeNodeRecord::baseline_states(true);
    search_states.focused = true;
    let mut searchbox = base_node(
        searchbox_id,
        tree_epoch_ref,
        SurfaceFamily::ShellZone,
        NodeKind::Searchbox,
        RoleMapping {
            generic_role: GenericRole::Searchbox,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.Edit".to_string()),
                nsaccessibility: Some("AXSearchField".to_string()),
                at_spi: Some("entry".to_string()),
            },
        },
        AccessibleName {
            name: "Search".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.palette.searchbox".to_string()),
            placeholder_names: Vec::new(),
            description: Some("Command palette query input.".to_string()),
            description_message_id: Some("a11y.tree.palette.searchbox.description".to_string()),
        },
        search_states,
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(3),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );
    searchbox.relationships.parent_node_ref = Some(palette_root_id.to_string());
    nodes.push(searchbox);

    let mut results = base_node(
        results_id,
        tree_epoch_ref,
        SurfaceFamily::List,
        NodeKind::ListContainer,
        RoleMapping {
            generic_role: GenericRole::List,
            role_source: RoleSource::ExplicitSurfaceContract,
            role_confidence: RoleConfidence::Exact,
            native_role_hints: NativeRoleHints {
                uia: Some("UIA.List".to_string()),
                nsaccessibility: Some("AXList".to_string()),
                at_spi: Some("list".to_string()),
            },
        },
        AccessibleName {
            name: "Results".to_string(),
            name_source: NameSource::VisibleLabel,
            message_id: Some("a11y.tree.palette.results".to_string()),
            placeholder_names: Vec::new(),
            description: Some("Command palette grouped results.".to_string()),
            description_message_id: Some("a11y.tree.palette.results.description".to_string()),
        },
        AccessibilityTreeNodeRecord::baseline_states(true),
        NodePosition {
            position_in_set: None,
            set_size: None,
            row_index: None,
            row_count: None,
            column_index: None,
            column_count: None,
            level: Some(3),
            index_truth_class: IndexTruthClass::Exact,
        },
        SupportStatus {
            support_class: SupportClass::Supported,
            support_state: SupportState::FullAccessible,
            degradation_reason_classes: vec![DegradationReasonClass::None],
            user_visible_notice_required: false,
            support_language: "Supported".to_string(),
            recovery_action_label: None,
        },
        minted_at.clone(),
    );
    results.relationships.parent_node_ref = Some(palette_root_id.to_string());

    let mut row_nodes: Vec<AccessibilityTreeNodeRecord> = Vec::new();
    let mut row_ids: Vec<String> = Vec::new();
    let mut selected_row: Option<String> = None;
    let mut row_index = 0u32;

    for group in palette.groups() {
        for item in &group.items {
            let key = &item.key;
            let (row_id, row) = match key {
                PaletteItemKey::Command { command_id } => {
                    let Some(entry) = registry.get(command_id) else {
                        continue;
                    };
                    let enablement_context = CommandEnablementContext {
                        client_scope: enablement.client_scope.to_string(),
                        workspace_trust_state: enablement.workspace_trust_state.to_string(),
                        execution_context_available: enablement.execution_context_available,
                        provider_linked: enablement.provider_linked,
                        credential_available: enablement.credential_available,
                        policy_disabled: enablement.policy_disabled,
                        policy_blocked_in_context: enablement.policy_blocked_in_context,
                        labs_enabled: enablement.labs_enabled,
                        argument_provenance_map: crate::commands::argument_provenance_map_for(
                            entry,
                        ),
                    };
                    let snapshot = entry.evaluate_enablement(&enablement_context);
                    let disabled = snapshot.decision_class != EnablementDecisionClass::Enabled;
                    let disabled_reason = snapshot
                        .disabled_reason_code
                        .map(|code| code.as_str().to_string());

                    let mut states = AccessibilityTreeNodeRecord::baseline_states(true);
                    let is_selected = palette
                        .selected_key()
                        .is_some_and(|selected| selected == key);
                    states.selected = is_selected;
                    if disabled {
                        states.disabled = true;
                        states.degraded = true;
                        if let Some(reason) = disabled_reason.as_deref() {
                            states.state_labels.push(StateLabel {
                                state_class: StateLabelClass::DisabledReason,
                                label: reason.to_string(),
                                message_id: None,
                            });
                        }
                    }

                    let label = entry.title.clone();
                    let description = format!(
                        "Command id: {}. Shortcut: {}.",
                        entry.command_id(),
                        shortcuts_by_command_id
                            .get(entry.command_id())
                            .map(|seqs| seqs.join(", "))
                            .unwrap_or_else(|| "unbound".to_string())
                    );

                    let mut node = base_node(
                        &format!(
                            "node.palette.result.command.{}",
                            sanitize_for_node_id(entry.command_id())
                        ),
                        tree_epoch_ref,
                        SurfaceFamily::List,
                        NodeKind::ListRow,
                        RoleMapping {
                            generic_role: GenericRole::Button,
                            role_source: RoleSource::CommandDescriptor,
                            role_confidence: RoleConfidence::Exact,
                            native_role_hints: NativeRoleHints {
                                uia: Some("UIA.Button".to_string()),
                                nsaccessibility: Some("AXButton".to_string()),
                                at_spi: Some("push button".to_string()),
                            },
                        },
                        AccessibleName {
                            name: label,
                            name_source: NameSource::VisibleLabel,
                            message_id: Some("a11y.tree.palette.result.command".to_string()),
                            placeholder_names: Vec::new(),
                            description: Some(description),
                            description_message_id: None,
                        },
                        states,
                        NodePosition {
                            position_in_set: Some(row_index.saturating_add(1)),
                            set_size: None,
                            row_index: Some(row_index.saturating_add(1)),
                            row_count: None,
                            column_index: None,
                            column_count: None,
                            level: Some(4),
                            index_truth_class: IndexTruthClass::Exact,
                        },
                        SupportStatus {
                            support_class: SupportClass::Supported,
                            support_state: if disabled {
                                SupportState::DegradedAccessible
                            } else {
                                SupportState::FullAccessible
                            },
                            degradation_reason_classes: vec![if disabled {
                                DegradationReasonClass::PolicyOrTrustRestriction
                            } else {
                                DegradationReasonClass::None
                            }],
                            user_visible_notice_required: disabled,
                            support_language: if disabled {
                                "Degraded".to_string()
                            } else {
                                "Supported".to_string()
                            },
                            recovery_action_label: None,
                        },
                        minted_at.clone(),
                    );
                    node.relationships.parent_node_ref = Some(results_id.to_string());
                    node.relationships
                        .source_anchor_refs
                        .push(entry.command_id().to_string());
                    (node.node_id.clone(), node)
                }
                PaletteItemKey::File { .. } => {
                    let mut states = AccessibilityTreeNodeRecord::baseline_states(false);
                    states.degraded = true;
                    let is_selected = palette
                        .selected_key()
                        .is_some_and(|selected| selected == key);
                    states.selected = is_selected;
                    let node_id = format!(
                        "node.palette.result.file.redacted.{}",
                        row_index.saturating_add(1)
                    );
                    let node = base_node(
                        &node_id,
                        tree_epoch_ref,
                        SurfaceFamily::List,
                        NodeKind::ListRow,
                        RoleMapping {
                            generic_role: GenericRole::Listitem,
                            role_source: RoleSource::FallbackSummary,
                            role_confidence: RoleConfidence::SummaryOnly,
                            native_role_hints: NativeRoleHints {
                                uia: Some("UIA.ListItem".to_string()),
                                nsaccessibility: Some("AXGroup".to_string()),
                                at_spi: Some("list item".to_string()),
                            },
                        },
                        AccessibleName {
                            name: "File result (redacted)".to_string(),
                            name_source: NameSource::DegradedSupportLabel,
                            message_id: Some("a11y.tree.palette.result.file_redacted".to_string()),
                            placeholder_names: Vec::new(),
                            description: Some(
                                "File paths are redacted from export-safe accessibility snapshots."
                                    .to_string(),
                            ),
                            description_message_id: None,
                        },
                        states,
                        NodePosition {
                            position_in_set: Some(row_index.saturating_add(1)),
                            set_size: None,
                            row_index: Some(row_index.saturating_add(1)),
                            row_count: None,
                            column_index: None,
                            column_count: None,
                            level: Some(4),
                            index_truth_class: IndexTruthClass::Exact,
                        },
                        SupportStatus {
                            support_class: SupportClass::Experimental,
                            support_state: SupportState::SummaryOnly,
                            degradation_reason_classes: vec![
                                DegradationReasonClass::UnsupportedSurface,
                            ],
                            user_visible_notice_required: true,
                            support_language: "Experimental".to_string(),
                            recovery_action_label: None,
                        },
                        minted_at.clone(),
                    );
                    (node.node_id.clone(), node)
                }
            };

            row_ids.push(row_id.clone());
            if palette
                .selected_key()
                .is_some_and(|selected| selected == key)
            {
                selected_row = Some(row_id.clone());
            }
            row_nodes.push(row);
            row_index = row_index.saturating_add(1);
        }
    }

    results.relationships.child_node_refs = row_ids.clone();
    results.relationships.owns_refs = row_ids.clone();
    results.relationships.active_descendant_ref =
        selected_row.clone().or(Some(searchbox_id.to_string()));
    nodes.push(results);
    nodes.extend(row_nodes);

    PaletteNodeSet {
        nodes,
        child_refs: vec![palette_root_id.to_string()],
        owns_refs: vec![palette_root_id.to_string()],
        active_descendant_ref: Some(searchbox_id.to_string()),
    }
}

fn sanitize_for_node_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use aureline_commands::registry::seeded_registry;

    use super::*;

    #[test]
    fn start_center_tree_includes_placeholder_surfaces() {
        let registry = seeded_registry();
        let shortcuts_by_command_id: HashMap<String, Vec<String>> = HashMap::new();
        let frame = DesktopFrame::new(1920, 1080);
        let palette = CommandPaletteState::new(registry);
        let start_center = StartCenterState::new();
        let docs_help_boundary_card =
            crate::embedded::docs_help::seeded_docs_help_boundary_card("id:build:test:01");

        let enablement = ShellA11yEnablementContext {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
            labs_enabled: false,
        };

        let snapshot = materialize_shell_accessibility_tree(
            registry,
            &shortcuts_by_command_id,
            &frame,
            &palette,
            &start_center,
            &docs_help_boundary_card,
            enablement,
        );

        let root = snapshot
            .nodes
            .iter()
            .find(|node| node.node_id == "node.app.root")
            .expect("root node must exist");
        assert_eq!(
            root.relationships.active_descendant_ref.as_deref(),
            Some("node.start_center.action.1")
        );

        for required in [
            "node.start_center.region",
            "node.start_center.action_list",
            "node.start_center.action.1",
            "node.embedded.docs_help.boundary_card",
            "node.panel.terminal.placeholder",
        ] {
            assert!(
                snapshot.nodes.iter().any(|node| node.node_id == required),
                "expected node {required}"
            );
        }
    }

    #[test]
    fn palette_tree_keeps_searchbox_as_focus_owner() {
        let registry = seeded_registry();
        let shortcuts_by_command_id: HashMap<String, Vec<String>> = HashMap::new();
        let frame = DesktopFrame::new(1920, 1080);
        let mut palette = CommandPaletteState::new(registry);
        palette.open(registry, temp_palette_root());
        let start_center = StartCenterState::new();
        let docs_help_boundary_card =
            crate::embedded::docs_help::seeded_docs_help_boundary_card("id:build:test:01");

        let enablement = ShellA11yEnablementContext {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
            labs_enabled: false,
        };

        let snapshot = materialize_shell_accessibility_tree(
            registry,
            &shortcuts_by_command_id,
            &frame,
            &palette,
            &start_center,
            &docs_help_boundary_card,
            enablement,
        );

        let root = snapshot
            .nodes
            .iter()
            .find(|node| node.node_id == "node.app.root")
            .expect("root node must exist");
        assert_eq!(
            root.relationships.active_descendant_ref.as_deref(),
            Some("node.palette.searchbox")
        );

        assert!(
            snapshot
                .nodes
                .iter()
                .any(|node| node.node_id == "node.palette.dialog"),
            "palette dialog node must exist"
        );
        let searchbox = snapshot
            .nodes
            .iter()
            .find(|node| node.node_id == "node.palette.searchbox")
            .expect("palette searchbox must exist");
        assert!(searchbox.states.focused, "searchbox should own focus");

        assert!(
            !snapshot
                .nodes
                .iter()
                .any(|node| node.node_id == "node.start_center.region"),
            "start center should not be materialized while the palette is open"
        );

        let selected_command_id = match palette.selected_key() {
            Some(PaletteItemKey::Command { command_id }) => command_id.clone(),
            Some(PaletteItemKey::File { .. }) => {
                // The palette test does not require file results; command results
                // are sufficient to validate focus ownership.
                return;
            }
            None => return,
        };
        let selected_node_id = format!(
            "node.palette.result.command.{}",
            sanitize_for_node_id(&selected_command_id)
        );
        let selected_node = snapshot
            .nodes
            .iter()
            .find(|node| node.node_id == selected_node_id)
            .expect("selected command row must be present in the tree");
        assert!(
            selected_node.states.selected,
            "selected row must be marked selected"
        );
        assert!(
            !selected_node.states.focused,
            "selected result rows should not be the focus owner while the searchbox is focused"
        );
    }

    fn temp_palette_root() -> PathBuf {
        let pid = std::process::id();
        let now = aureline_commands::invocation::now_rfc3339().replace(':', "-");
        let root = std::env::temp_dir().join(format!("aureline_palette_smoke_{pid}_{now}"));
        let _ = std::fs::create_dir_all(&root);
        root
    }
}
