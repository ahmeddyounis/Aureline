use std::collections::HashMap;
use std::path::PathBuf;

use aureline_commands::registry::seeded_registry;
use aureline_shell::palette::{CommandPaletteState, PaletteItemKey};
use aureline_shell::wedge_inspector::{
    WedgeInspectorInputs, WedgeInspectorOverlay, WEDGE_INSPECTOR_COMMAND_ID,
};

fn palette_command_ids(palette: &CommandPaletteState) -> Vec<String> {
    palette
        .groups()
        .iter()
        .flat_map(|group| group.items.iter())
        .filter_map(|item| match &item.key {
            PaletteItemKey::Command { command_id } => Some(command_id.clone()),
            PaletteItemKey::File { .. } => None,
        })
        .collect()
}

fn type_query(palette: &mut CommandPaletteState, query: &str) {
    let registry = seeded_registry();
    let shortcuts: HashMap<String, Vec<String>> = HashMap::new();
    for ch in query.chars() {
        assert!(palette.handle_text_input(ch, registry, &shortcuts));
    }
}

#[test]
fn inspector_lists_all_wedges_and_each_panel_renders_claim_limits() {
    let mut inspector = WedgeInspectorOverlay::new(WedgeInspectorInputs::default());
    let expected = [
        "ai_context_inspector",
        "ai_truth_strip",
        "host_boundary_cues",
        "managed_workspace_labels",
        "notebook_trust_badges",
        "notebook_preview_truth",
        "structured_config_preview_truth",
        "install_review_fact_grid",
        "permission_prompts",
        "restricted_mode_launch_wedge",
        "review_preview",
        "safe_preview_card",
        "graph_state_card",
    ];

    assert_eq!(inspector.rows().len(), expected.len());
    for expected_id in expected {
        assert!(
            inspector
                .rows()
                .iter()
                .any(|row| row.wedge_id == expected_id),
            "missing wedge row {expected_id}"
        );
    }

    for _ in 0..inspector.rows().len() {
        let row = inspector.selected_row().expect("selected row");
        assert!(!row.prototype_label_token.trim().is_empty());
        assert!(!row.claim_limits.is_empty(), "{}", row.wedge_id);
        assert!(row.panel_plaintext.contains("source_binding:"));
        assert!(row.panel_plaintext.contains("prototype_label:"));
        assert!(row.panel_plaintext.contains(&row.prototype_label_token));
        assert!(row.panel_plaintext.contains("claim_limits:"));
        for limit in &row.claim_limits {
            assert!(
                row.panel_plaintext.contains(&limit.token),
                "{}",
                row.wedge_id
            );
        }
        let rendered = inspector.render_lines();
        assert!(rendered.iter().any(|line| line.contains(&row.display_name)));
        inspector.select_next();
    }

    let restricted_row = inspector
        .rows()
        .iter()
        .find(|row| row.wedge_id == "restricted_mode_launch_wedge")
        .expect("restricted mode row");
    assert!(restricted_row.panel_plaintext.contains("editor_read_write"));
    assert!(restricted_row.panel_plaintext.contains("tasks_run"));
    assert!(restricted_row.panel_plaintext.contains("blocked_or_review"));
    assert!(restricted_row
        .panel_plaintext
        .contains("visible_after_open=true"));
}

#[test]
fn labs_palette_gate_hides_and_reveals_wedge_inspector_command() {
    let registry = seeded_registry();
    let cwd = PathBuf::from(".");
    let mut palette = CommandPaletteState::new(registry);
    palette.open(registry, cwd.clone());
    type_query(&mut palette, "wedge");
    assert!(
        !palette_command_ids(&palette)
            .iter()
            .any(|id| id == WEDGE_INSPECTOR_COMMAND_ID),
        "Labs command must stay absent while Labs is disabled"
    );

    let mut labs_palette = CommandPaletteState::new(registry);
    labs_palette.set_labs_enabled(registry, true);
    labs_palette.open(registry, cwd);
    type_query(&mut labs_palette, "wedge");
    assert!(
        palette_command_ids(&labs_palette)
            .iter()
            .any(|id| id == WEDGE_INSPECTOR_COMMAND_ID),
        "Labs command must be reachable when Labs is enabled"
    );
}
