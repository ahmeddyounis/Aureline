//! Headless emitter for the M5 notification / activity-center handoff routing primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-notification-activity-handoff-proof/`, its matrix CSV, the
//! Markdown report `artifacts/security/m5-notification-activity-handoff-primitive.md`, and
//! the narrowed fixtures under
//! `fixtures/security/m5-notification-activity-handoff-primitive/`. Every M5 route that
//! surfaces an advisory or revocation event — the durable activity center, a privacy-safe
//! native OS notification, Help/About, and the support bundle — reads this primitive so the
//! event identity, severity, affected scope, current status, delivery posture, and reopen
//! target stay consistent, and so the support export reconstructs the event from one shared
//! handoff model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- fixture-quiet-hours-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- fixture-offline-deferred-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_notification_and_activity_center_handoff_routing_primitive::{
    seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed,
    seeded_m5_notification_activity_handoff_primitive_packet,
    seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed,
    M5NotificationHandoffPacket,
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
            let packet = seeded_m5_notification_activity_handoff_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_notification_activity_handoff_primitive_packet()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_notification_activity_handoff_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-quiet-hours-beta-narrowed") => {
            let packet =
                seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-deferred-preview-narrowed") => {
            let packet =
                seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_notification_activity_handoff_primitive_packet(),
                seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed(),
                seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed(
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

fn assert_valid(packet: &M5NotificationHandoffPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
