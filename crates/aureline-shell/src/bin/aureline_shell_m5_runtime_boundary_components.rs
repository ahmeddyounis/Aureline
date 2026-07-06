//! Headless emitter for the frozen M5 terminal-tab, remote-target-pill,
//! environment-status-strip, toolchain-pin-row, presence-avatar-stack, and
//! repair-action-card component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-runtime-boundary-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-runtime-boundary-components.md`, and
//! the narrowed fixtures under `fixtures/ui/m5-runtime-boundary-components/`.
//! Terminal/session surfaces, remote and environment surfaces, collaboration
//! surfaces, and repair surfaces read this matrix so one terminal-tab model
//! carries session title, host boundary, and shell-integration quality, one
//! remote pill names host boundary and connection state, one environment strip
//! names the winning runtime source, one toolchain row explains why a toolchain
//! won, one presence stack shows role and follow state, and one repair card shows
//! blast radius and reversibility before approval.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- fixture-presence-avatar-stack-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- fixture-repair-action-card-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_runtime_boundary_components -- validate
//! ```

use aureline_shell::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    seeded_m5_runtime_boundary_component_matrix,
    seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed,
    seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed,
    M5RuntimeBoundaryMatrixPacket,
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
            let packet = seeded_m5_runtime_boundary_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_runtime_boundary_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_runtime_boundary_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-presence-avatar-stack-beta-narrowed") => {
            let packet =
                seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-repair-action-card-preview-narrowed") => {
            let packet =
                seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_runtime_boundary_component_matrix(),
                seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed(),
                seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed(),
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

fn assert_valid(packet: &M5RuntimeBoundaryMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
