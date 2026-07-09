//! Headless emitter for the M5 notification-row / mobile-review-card controls.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-notification-row-mobile-review-card-proof/`, its matrix CSV, the
//! Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-notification-row-mobile-review-card-controls/`. The notification-triage
//! and review-queue UIs read this packet so the first glance at a companion event or review
//! item names the object, the scope, the freshness, the severity, and the companion-versus-
//! desktop capability boundary before a tap.
//!
//! ```text
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- support-export
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- report
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- csv
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- fixture-notification-row-stale
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- fixture-mobile-review-card-desktop-required
//! cargo run -p aureline-companion --example dump_notification_row_mobile_review_card_controls -- validate
//! ```

use aureline_companion::implement_notification_rows_and_mobile_review_cards_with_object_identity_client_scope_freshness_severity_unread_and_desktop_handoff_truth::{
    seeded_notification_row_mobile_review_card_controls,
    seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required,
    seeded_notification_row_mobile_review_card_controls_notification_row_stale,
    NotificationRowMobileReviewCardControlsPacket,
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
            let packet = seeded_notification_row_mobile_review_card_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_notification_row_mobile_review_card_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_notification_row_mobile_review_card_controls().render_matrix_csv()
            );
        }
        Some("fixture-notification-row-stale") => {
            let packet =
                seeded_notification_row_mobile_review_card_controls_notification_row_stale();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-mobile-review-card-desktop-required") => {
            let packet =
                seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_notification_row_mobile_review_card_controls(),
                seeded_notification_row_mobile_review_card_controls_notification_row_stale(),
                seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required(),
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
    packet: &NotificationRowMobileReviewCardControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls failed validation: {}", tokens.join(",")).into())
    }
}
