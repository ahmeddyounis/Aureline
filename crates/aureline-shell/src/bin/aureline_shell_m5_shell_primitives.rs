//! Headless emitter for the frozen M5 status-bar, transient-inspect,
//! pane-control, and durable-progress-component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-shell-primitives-proof/`, its matrix CSV, the
//! Markdown report `artifacts/shell/m5-shell-primitives.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-shell-primitives/`. Status bars, hovercards,
//! peek panels, splitters, and activity/progress centers read this matrix so
//! ambient instrumentation stays overflow-safe, transient inspect keeps its
//! source/freshness truth, panes resize serializably from the keyboard, and
//! progress rows stay durable and reopenable.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- fixture-pane-resize-preset-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- fixture-pinned-preview-promotion-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitives -- validate
//! ```

use aureline_shell::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix,
    seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed,
    seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed,
    M5ShellPrimitivesMatrixPacket,
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
            let packet = seeded_m5_shell_primitives_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_shell_primitives_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_shell_primitives_matrix().render_matrix_csv()
            );
        }
        Some("fixture-pane-resize-preset-beta-narrowed") => {
            let packet = seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-pinned-preview-promotion-preview-narrowed") => {
            let packet =
                seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_shell_primitives_matrix(),
                seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed(),
                seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed(),
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

fn assert_valid(packet: &M5ShellPrimitivesMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
