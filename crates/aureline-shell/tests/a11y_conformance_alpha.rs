//! Alpha conformance check for shell accessibility-tree surfaces.
//!
//! Every claimed launch-wedge shell surface (application root, shell zones,
//! Start Center entry surface, command palette overlay, embedded docs/help
//! boundary chrome, and the placeholder terminal panel) must produce an
//! accessibility-tree projection that exposes a non-default role, a
//! non-empty accessible name, and an explicit state block on its primary
//! controls. The projection built by `materialize_shell_accessibility_tree`
//! is the contract the shell promises platform assistive technologies via
//! the export-safe taxonomy described in
//! `docs/accessibility/accessibility_tree_contract.md`. Surfaces missing a
//! role or name on any primary control must fail this test.

use std::collections::HashMap;

use aureline_commands::registry::seeded_registry;
use aureline_shell::a11y::shell_bridge::{
    materialize_shell_accessibility_tree, ShellA11yEnablementContext, ShellA11yTreeRecord,
};
use aureline_shell::a11y::tree_contract::{
    AccessibilityTreeNodeRecord, GenericRole, NodeKind, RoleConfidence, SurfaceFamily,
};
use aureline_shell::app_frame::desktop_frame::DesktopFrame;
use aureline_shell::embedded::boundary_card::EmbeddedBoundaryCardRecord;
use aureline_shell::embedded::docs_help::seeded_docs_help_boundary_card;
use aureline_shell::palette::CommandPaletteState;
use aureline_shell::start_center::StartCenterState;

fn baseline_enablement() -> ShellA11yEnablementContext<'static> {
    ShellA11yEnablementContext {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    }
}

fn baseline_docs_help_card() -> EmbeddedBoundaryCardRecord {
    seeded_docs_help_boundary_card("id:build:test:01")
}

fn temp_palette_root() -> std::path::PathBuf {
    let pid = std::process::id();
    let now = aureline_commands::invocation::now_rfc3339().replace(':', "-");
    let root = std::env::temp_dir().join(format!("aureline_a11y_conformance_{pid}_{now}"));
    let _ = std::fs::create_dir_all(&root);
    root
}

fn materialize_with_start_center() -> ShellA11yTreeRecord {
    let registry = seeded_registry();
    let shortcuts: HashMap<String, Vec<String>> = HashMap::new();
    let frame = DesktopFrame::new(1920, 1080);
    let palette = CommandPaletteState::new(registry);
    let start_center = StartCenterState::new();
    let docs_help = baseline_docs_help_card();

    materialize_shell_accessibility_tree(
        registry,
        &shortcuts,
        &frame,
        &palette,
        &start_center,
        &docs_help,
        baseline_enablement(),
    )
}

fn materialize_with_palette_open() -> ShellA11yTreeRecord {
    let registry = seeded_registry();
    let shortcuts: HashMap<String, Vec<String>> = HashMap::new();
    let frame = DesktopFrame::new(1920, 1080);
    let mut palette = CommandPaletteState::new(registry);
    palette.open(registry, temp_palette_root());
    let start_center = StartCenterState::new();
    let docs_help = baseline_docs_help_card();

    materialize_shell_accessibility_tree(
        registry,
        &shortcuts,
        &frame,
        &palette,
        &start_center,
        &docs_help,
        baseline_enablement(),
    )
}

fn find_node<'a>(
    tree: &'a ShellA11yTreeRecord,
    node_id: &str,
) -> &'a AccessibilityTreeNodeRecord {
    tree.nodes
        .iter()
        .find(|node| node.node_id == node_id)
        .unwrap_or_else(|| panic!("expected accessibility tree node {node_id}"))
}

fn assert_role_name_state(node: &AccessibilityTreeNodeRecord) {
    assert_ne!(
        node.role_mapping.generic_role,
        GenericRole::None,
        "node {} ({:?}) must expose a generic role",
        node.node_id,
        node.node_kind
    );
    assert!(
        !node.accessible_name.name.trim().is_empty(),
        "node {} ({:?}) must expose a non-empty accessible name",
        node.node_id,
        node.node_kind
    );

    let states = &node.states;
    let has_positive_state = states.visible
        || states.offscreen
        || states.virtualized
        || states.degraded
        || states.disabled
        || states.busy;
    assert!(
        has_positive_state,
        "node {} ({:?}) must carry at least one explicit state signal",
        node.node_id,
        node.node_kind
    );
}

#[test]
fn every_surface_node_exposes_role_name_and_state_for_start_center_scenario() {
    let tree = materialize_with_start_center();
    assert!(
        !tree.nodes.is_empty(),
        "materialized tree must contain shell surfaces"
    );
    for node in &tree.nodes {
        assert_role_name_state(node);
    }
}

#[test]
fn every_surface_node_exposes_role_name_and_state_for_palette_scenario() {
    let tree = materialize_with_palette_open();
    assert!(
        !tree.nodes.is_empty(),
        "materialized tree must contain shell surfaces"
    );
    for node in &tree.nodes {
        assert_role_name_state(node);
    }
}

#[test]
fn application_root_and_shell_zones_expose_primary_control_semantics() {
    let tree = materialize_with_start_center();

    let root = find_node(&tree, "node.app.root");
    assert_eq!(root.node_kind, NodeKind::ApplicationRoot);
    assert_eq!(root.role_mapping.generic_role, GenericRole::Application);
    assert_eq!(root.role_mapping.role_confidence, RoleConfidence::Exact);
    assert_eq!(root.surface_family, SurfaceFamily::ShellZone);
    assert!(
        !root.accessible_name.name.is_empty(),
        "application root must carry an accessible name"
    );
    assert_eq!(root.states.expanded, Some(true));

    let title_bar = find_node(&tree, "node.shell.zone.title_context_bar");
    assert_eq!(title_bar.role_mapping.generic_role, GenericRole::Toolbar);
    assert!(title_bar.states.focusable);

    let status_bar = find_node(&tree, "node.shell.zone.status_bar");
    assert_eq!(status_bar.role_mapping.generic_role, GenericRole::Status);
    assert!(status_bar.states.focusable);

    for landmark_id in [
        "node.shell.zone.activity_rail",
        "node.shell.zone.left_sidebar",
        "node.shell.zone.main_workspace",
        "node.shell.zone.right_inspector",
        "node.shell.zone.bottom_panel",
    ] {
        let zone = find_node(&tree, landmark_id);
        assert_eq!(
            zone.role_mapping.generic_role,
            GenericRole::Landmark,
            "{} must project a landmark role",
            landmark_id
        );
        assert_eq!(zone.node_kind, NodeKind::ShellZone);
        assert!(
            zone.states.focusable,
            "{} must be keyboard reachable",
            landmark_id
        );
    }
}

#[test]
fn start_center_primary_controls_expose_role_name_and_state() {
    let tree = materialize_with_start_center();

    let region = find_node(&tree, "node.start_center.region");
    assert_eq!(region.role_mapping.generic_role, GenericRole::Landmark);
    assert_eq!(region.node_kind, NodeKind::LandmarkRegion);
    assert_eq!(region.accessible_name.name, "Start Center");

    let list = find_node(&tree, "node.start_center.action_list");
    assert_eq!(list.role_mapping.generic_role, GenericRole::List);
    assert_eq!(list.node_kind, NodeKind::ListContainer);
    assert!(!list.accessible_name.name.is_empty());

    let action_rows: Vec<&AccessibilityTreeNodeRecord> = tree
        .nodes
        .iter()
        .filter(|node| {
            node.node_id.starts_with("node.start_center.action.")
                && node.node_kind == NodeKind::ListRow
        })
        .collect();
    assert!(
        !action_rows.is_empty(),
        "Start Center must materialize at least one action row"
    );

    for row in &action_rows {
        assert_eq!(row.role_mapping.generic_role, GenericRole::Button);
        assert_eq!(row.role_mapping.role_confidence, RoleConfidence::Exact);
        assert!(
            !row.accessible_name.name.is_empty(),
            "Start Center action {} must expose a name",
            row.node_id
        );
        assert!(row.states.focusable);
        assert!(row.position.position_in_set.is_some());
        assert!(row.position.set_size.is_some());
    }

    let first_action = find_node(&tree, "node.start_center.action.1");
    assert!(first_action.states.selected);
    assert!(first_action.states.focused);
}

#[test]
fn embedded_docs_help_boundary_exposes_role_name_and_state() {
    let tree = materialize_with_start_center();
    let docs = find_node(&tree, "node.embedded.docs_help.boundary_card");
    assert_eq!(docs.role_mapping.generic_role, GenericRole::Landmark);
    assert_eq!(docs.node_kind, NodeKind::LandmarkRegion);
    assert!(!docs.accessible_name.name.is_empty());
    assert!(docs.states.focusable);
    assert_eq!(
        docs.relationships.parent_node_ref.as_deref(),
        Some("node.shell.zone.right_inspector")
    );
}

#[test]
fn terminal_placeholder_panel_exposes_role_name_and_state() {
    let tree = materialize_with_start_center();
    let terminal = find_node(&tree, "node.panel.terminal.placeholder");
    assert_eq!(terminal.role_mapping.generic_role, GenericRole::Log);
    assert_eq!(terminal.node_kind, NodeKind::TerminalRegion);
    assert!(!terminal.accessible_name.name.is_empty());
    assert!(terminal.states.read_only);
    assert!(terminal.states.degraded);
    assert_eq!(
        terminal.relationships.parent_node_ref.as_deref(),
        Some("node.shell.zone.bottom_panel")
    );
}

#[test]
fn command_palette_primary_controls_expose_role_name_and_state() {
    let tree = materialize_with_palette_open();

    let dialog = find_node(&tree, "node.palette.dialog");
    assert_eq!(dialog.role_mapping.generic_role, GenericRole::Dialog);
    assert!(dialog.states.modal);
    assert!(!dialog.accessible_name.name.is_empty());

    let searchbox = find_node(&tree, "node.palette.searchbox");
    assert_eq!(searchbox.role_mapping.generic_role, GenericRole::Searchbox);
    assert_eq!(searchbox.node_kind, NodeKind::Searchbox);
    assert_eq!(searchbox.accessible_name.name, "Search");
    assert!(searchbox.states.focused);
    assert!(searchbox.states.focusable);

    let results = find_node(&tree, "node.palette.results");
    assert_eq!(results.role_mapping.generic_role, GenericRole::List);
    assert_eq!(results.node_kind, NodeKind::ListContainer);
    assert!(!results.accessible_name.name.is_empty());

    let palette_rows: Vec<&AccessibilityTreeNodeRecord> = tree
        .nodes
        .iter()
        .filter(|node| {
            node.node_kind == NodeKind::ListRow
                && node.relationships.parent_node_ref.as_deref() == Some("node.palette.results")
        })
        .collect();
    assert!(
        !palette_rows.is_empty(),
        "command palette must materialize at least one result row"
    );
    for row in &palette_rows {
        assert!(
            matches!(
                row.role_mapping.generic_role,
                GenericRole::Button | GenericRole::Listitem
            ),
            "palette row {} should expose a button or listitem role",
            row.node_id
        );
        assert!(
            !row.accessible_name.name.is_empty(),
            "palette row {} must expose a name",
            row.node_id
        );
        let states = &row.states;
        assert!(
            states.visible || states.degraded || states.virtualized,
            "palette row {} must expose a state signal",
            row.node_id
        );
    }
}
