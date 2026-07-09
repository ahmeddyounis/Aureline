//! Headless emitter for the M5 companion degraded-state continuity controls.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-companion-degraded-state-continuity-proof/`, its matrix CSV, the
//! Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-companion-degraded-state-continuity-controls/`. The notification/triage and
//! desktop-handoff UIs read this packet so the first glance at a degraded surface names its
//! availability state, scope, freshness, and next-safe-action, and no surface routes blindly
//! into a broken or over-privileged path before a tap.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- support-export
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- report
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- csv
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- fixture-notification-surface-blocked
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- fixture-handoff-surface-deleted-object
//! cargo run -p aureline-companion --example dump_companion_degraded_state_continuity_controls -- validate
//! ```

use aureline_companion::ship_cached_offline_auth_blocked_and_policy_blocked_companion_states_with_summary_first_object_continuity_safe_triage_verbs_and_no_blind_tap_routing::{
    seeded_companion_degraded_state_continuity_controls,
    seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object,
    seeded_companion_degraded_state_continuity_controls_notification_surface_blocked,
    CompanionDegradedStateContinuityPacket,
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
            let packet = seeded_companion_degraded_state_continuity_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_companion_degraded_state_continuity_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_companion_degraded_state_continuity_controls().render_matrix_csv()
            );
        }
        Some("fixture-notification-surface-blocked") => {
            let packet =
                seeded_companion_degraded_state_continuity_controls_notification_surface_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-handoff-surface-deleted-object") => {
            let packet =
                seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_companion_degraded_state_continuity_controls(),
                seeded_companion_degraded_state_continuity_controls_notification_surface_blocked(),
                seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object(
                ),
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
    packet: &CompanionDegradedStateContinuityPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls failed validation: {}", tokens.join(",")).into())
    }
}
