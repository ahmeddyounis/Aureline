//! Headless inspector for the beta debugger daily-use surfaces.
//!
//! Emits the same projection records consumed by the live shell, the
//! support-export wrapper, and the integration tests that replay the
//! checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- projection
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- surfaces
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- reconnect-drill
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- no-session-drill
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_debug_ui -- validate
//! ```

use aureline_shell::debug_ui::{
    seeded_no_session_drill_projection, seeded_protected_walk_projection,
    seeded_reconnect_drill_projection, validate_debug_ui_projection, DebugUiSupportExport,
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
        Some("projection") | None => {
            print_json(&seeded_protected_walk_projection())?;
        }
        Some("surfaces") => {
            print_json(&seeded_protected_walk_projection().surfaces)?;
        }
        Some("reconnect-drill") => {
            print_json(&seeded_reconnect_drill_projection())?;
        }
        Some("no-session-drill") => {
            print_json(&seeded_no_session_drill_projection())?;
        }
        Some("support-export") => {
            let export = DebugUiSupportExport::from_projection(
                "support-export:debug-ui-beta:001",
                "2026-05-15T00:00:11Z",
                seeded_protected_walk_projection(),
            );
            print_json(&export)?;
        }
        Some("validate") => {
            for (label, projection) in [
                ("protected_walk", seeded_protected_walk_projection()),
                ("reconnect_drill", seeded_reconnect_drill_projection()),
                ("no_session_drill", seeded_no_session_drill_projection()),
            ] {
                match validate_debug_ui_projection(&projection) {
                    Ok(()) => println!("ok {label}"),
                    Err(defects) => {
                        for defect in defects {
                            eprintln!(
                                "defect: kind={} surface={} field={} note={}",
                                defect.defect_kind_token,
                                defect.surface_class_token.unwrap_or_default(),
                                defect.field,
                                defect.note,
                            );
                        }
                        std::process::exit(3);
                    }
                }
            }
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
