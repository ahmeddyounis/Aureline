//! Headless inspector for the beta notification privacy / quiet-hours /
//! badge / cross-client dedupe projection.
//!
//! The bin emits the same beta records consumed by the live shell, by
//! the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- badges
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- validate
//! ```

use aureline_shell::notifications::beta::{
    seeded_notification_privacy_beta_page, validate_notification_privacy_beta_page,
    validate_notification_privacy_beta_support_export, NotificationPrivacyBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_notification_privacy_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("rows") => {
            print_json(&page.rows)?;
        }
        Some("badges") => {
            print_json(&page.badges)?;
        }
        Some("badge-projection") => {
            print_json(&page.badge_projection)?;
        }
        Some("support-export") => {
            let export = NotificationPrivacyBetaSupportExport::from_page(
                "support-export:notification-privacy-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => {
            let page_errors = validate_notification_privacy_beta_page(&page);
            let export = NotificationPrivacyBetaSupportExport::from_page(
                "support-export:notification-privacy-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            let export_errors = validate_notification_privacy_beta_support_export(&export);
            match (page_errors, export_errors) {
                (Ok(()), Ok(())) => {
                    println!("ok");
                }
                (page_result, export_result) => {
                    if let Err(errors) = page_result {
                        for err in errors {
                            eprintln!("page error: {err}");
                        }
                    }
                    if let Err(errors) = export_result {
                        for err in errors {
                            eprintln!("support-export error: {err}");
                        }
                    }
                    std::process::exit(3);
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
