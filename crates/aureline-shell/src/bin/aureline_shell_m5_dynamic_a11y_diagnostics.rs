//! Headless emitter for the M5 dynamic-surface assistive-tech diagnostics report.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-dynamic-a11y-diagnostics/` and the bridge/announcement/visual drill
//! fixtures under `fixtures/a11y/m5-bridge-and-announcement-drills/`. The shell, support
//! exports, help/docs, and release/public-truth automation consume this report so AT
//! health — bridge state, missing semantic nodes, announcement-spam budgets, focus-return
//! failures, and high-zoom/high-contrast/reduced-motion regressions — is diagnosable and
//! release-gated from the same support/export system used for other protected paths,
//! rather than reproduced by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-bridge-unavailable-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-bridge-regression-blocked
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-announcement-spam-blocked
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-visual-regression-blocked
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- validate
//! ```

use aureline_shell::accessibility::diagnostics::{
    seeded_m5_dynamic_a11y_diagnostics_report,
    seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked,
    seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked,
    seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed,
    seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked,
    M5DynamicA11yDiagnosticsPacket,
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
            let packet = seeded_m5_dynamic_a11y_diagnostics_report();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_dynamic_a11y_diagnostics_report().render_markdown_summary()
            );
        }
        Some("fixture-bridge-unavailable-narrowed") => {
            let packet = seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-bridge-regression-blocked") => {
            let packet = seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-announcement-spam-blocked") => {
            let packet = seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-visual-regression-blocked") => {
            let packet = seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_dynamic_a11y_diagnostics_report(),
                seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed(),
                seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked(),
                seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked(),
                seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked(),
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

fn assert_valid(packet: &M5DynamicA11yDiagnosticsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("diagnostics report failed validation: {}", tokens.join(",")).into())
    }
}
