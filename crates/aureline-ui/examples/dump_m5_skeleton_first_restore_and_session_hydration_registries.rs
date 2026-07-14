//! Headless emitter for the M5 skeleton-first-restore and session-hydration registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-skeleton-first-restore-and-session-hydration-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- report
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- restore-fidelity-table
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- fixture-placeholder-pane-continuity-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- fixture-context-only-hydration-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_skeleton_first_restore_and_session_hydration_registries -- validate
//! ```

use aureline_ui::m5_skeleton_first_restore_and_session_hydration_registries::{
    seeded_m5_skeleton_first_restore_and_session_hydration_registries,
    seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed,
    seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed,
    M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
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
            let packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_skeleton_first_restore_and_session_hydration_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_skeleton_first_restore_and_session_hydration_registries()
                    .render_matrix_csv()
            );
        }
        Some("restore-fidelity-table") => {
            print!(
                "{}",
                seeded_m5_skeleton_first_restore_and_session_hydration_registries()
                    .render_restore_fidelity_table()
            );
        }
        Some("fixture-placeholder-pane-continuity-beta-narrowed") => {
            let packet =
                seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-context-only-hydration-preview-narrowed") => {
            let packet =
                seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_skeleton_first_restore_and_session_hydration_registries(),
                seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed(),
                seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed(),
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

fn assert_valid(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
