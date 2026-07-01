//! Headless emitter for the frozen M5 shell-zone, responsive-class, and
//! multi-window continuity matrix.
//!
//! The bin is the only mint-from-truth path for the release-proof support export
//! checked in under `artifacts/release/m5-shell-continuity-proof/`, the
//! governance Markdown summary, the matrix CSV, the human-readable matrix
//! Markdown under `artifacts/shell/`, and the narrowed fixtures under
//! `fixtures/ui/m5-shell-layouts/`. Shell, windowing, layout, status, docs/help,
//! and release-proof automation read this matrix so a claimed M5 surface cannot
//! assert shell maturity without mapping its slot, collapse, and multi-window
//! behavior into a governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- fixture-profiler-remote-held
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- fixture-companion-overlay-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    seeded_m5_shell_zone_matrix, seeded_m5_shell_zone_matrix_companion_overlay_narrowed,
    seeded_m5_shell_zone_matrix_profiler_remote_held, M5ShellZoneMatrixPacket,
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
            let packet = seeded_m5_shell_zone_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("governance") => {
            print!("{}", seeded_m5_shell_zone_matrix().render_markdown_summary());
        }
        Some("csv") => {
            print!("{}", seeded_m5_shell_zone_matrix().render_matrix_csv());
        }
        Some("markdown") => {
            print!("{}", seeded_m5_shell_zone_matrix().render_markdown_summary());
        }
        Some("fixture-profiler-remote-held") => {
            let packet = seeded_m5_shell_zone_matrix_profiler_remote_held();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-companion-overlay-narrowed") => {
            let packet = seeded_m5_shell_zone_matrix_companion_overlay_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_shell_zone_matrix(),
                seeded_m5_shell_zone_matrix_profiler_remote_held(),
                seeded_m5_shell_zone_matrix_companion_overlay_narrowed(),
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

fn assert_valid(packet: &M5ShellZoneMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
