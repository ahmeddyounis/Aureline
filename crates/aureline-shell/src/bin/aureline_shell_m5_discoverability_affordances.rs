//! Headless emitter for the frozen M5 menu-affordance, keybinding-resolver, and
//! command-documentation matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-discoverability-affordances-proof/`, its matrix
//! CSV, the Markdown report `artifacts/commands/m5-discoverability-affordances.md`,
//! and the narrowed fixtures under
//! `fixtures/commands/m5-discoverability-affordances/`. Menus, context menus,
//! command bars, keybinding inspectors, leader/sequence help, and
//! command-documentation surfaces read this matrix so the same action keeps the
//! same label, shortcut truth, disabled-state reason, and authority posture on
//! every surface.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- fixture-imported-keymap-approximated-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- fixture-leader-sequence-help-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordances -- validate
//! ```

use aureline_shell::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix::{
    seeded_m5_discoverability_matrix,
    seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed,
    seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed,
    M5DiscoverabilityMatrixPacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_discoverability_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_discoverability_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_discoverability_matrix().render_matrix_csv());
        }
        Some("fixture-imported-keymap-approximated-narrowed") => {
            let packet = seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-leader-sequence-help-preview-narrowed") => {
            let packet = seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_discoverability_matrix(),
                seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed(),
                seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5DiscoverabilityMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
