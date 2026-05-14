//! Keybinding inspector projection for help and diagnostics surfaces.

use std::collections::BTreeMap;

use aureline_commands::CommandRegistry;
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::{preset_binding_rows, preset_conflicts, KeymapPresetId};

use crate::keybindings::build_alpha_keybinding_truth_lines;

use super::keyboard_gap_audit::build_audit_summary_lines;
use super::mode_state_orientation::build_alpha_mode_orientation_lines;

fn stable_sort_key(command_id: &str, title: &str) -> (String, String) {
    (title.to_lowercase(), command_id.to_string())
}

/// Builds inspector lines describing the active preset bindings and conflicts.
pub fn build_inspector_lines(
    registry: &CommandRegistry,
    preset: KeymapPresetId,
    platform: PlatformClass,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Keybindings — preset: {} ({})",
        preset.display_name(),
        preset.preset_ref()
    ));
    lines.push("Left/Right: switch preset   1-4: jump preset   Esc: close".to_string());
    lines.push("".to_string());

    let rows = match preset_binding_rows(preset, platform) {
        Ok(rows) => rows,
        Err(err) => {
            lines.push(format!("Failed to load preset bindings: {err}"));
            return lines;
        }
    };

    let mut by_command: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for row in rows {
        let title = registry
            .get(row.command_id.as_str())
            .map(|entry| entry.title.as_str())
            .unwrap_or("<unknown command>");
        by_command
            .entry(stable_sort_key(&row.command_id, title))
            .or_default()
            .push(row.literal_sequence);
    }

    lines.push("Bindings (command_id => shortcut(s))".to_string());
    for ((_title_key, command_id), sequences) in by_command {
        let title = registry
            .get(command_id.as_str())
            .map(|entry| entry.title.as_str())
            .unwrap_or("<unknown command>");
        let mut seqs = sequences;
        seqs.sort();
        seqs.dedup();
        lines.push(format!(
            "- {}  —  {}  =>  {}",
            title,
            command_id,
            seqs.join(", ")
        ));
    }

    let conflicts = match preset_conflicts(preset, platform) {
        Ok(conflicts) => conflicts,
        Err(err) => {
            lines.push("".to_string());
            lines.push(format!("Failed to inspect preset conflicts: {err}"));
            return lines;
        }
    };

    lines.push("".to_string());
    lines.push("Conflicts (requires review)".to_string());
    if conflicts.is_empty() {
        lines.push("- none".to_string());
    } else {
        for conflict in conflicts {
            lines.push(format!(
                "- {}  —  {}",
                conflict.inspected_sequence.literal_sequence, conflict.conflict_review_id
            ));
            for losing in conflict.losing_candidates.iter() {
                let command_id = losing.candidate.command.command_id.as_str();
                let title = registry
                    .get(command_id)
                    .map(|entry| entry.title.as_str())
                    .unwrap_or("<unknown command>");
                lines.push(format!("  * {}  —  {}", title, command_id));
            }
        }
    }

    lines.extend(build_audit_summary_lines(registry, preset, platform));
    lines.extend(build_alpha_keybinding_truth_lines(
        registry, preset, platform,
    ));
    lines.extend(build_alpha_mode_orientation_lines(preset, platform));

    lines
}
