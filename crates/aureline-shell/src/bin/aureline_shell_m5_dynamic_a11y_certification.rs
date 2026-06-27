//! Headless emitter for the M5 dynamic-surface assistive-tech certification capstone.
//!
//! The bin is the only mint-from-truth path for the certification support export and
//! Markdown proof checked in under `artifacts/release/m5-dynamic-a11y-certification/`, the
//! published green/yellow/red dashboard checked in at
//! `artifacts/a11y/m5-dynamic-a11y-dashboard.json`, and the stale-proof / regression /
//! waiver drill fixtures under `fixtures/a11y/m5-dynamic-a11y-certification/`. Release
//! center, support exports, docs/help, onboarding, presentation, the stable-claim matrix,
//! and the shell/editor/notebook/data/review surfaces consume this certification so each
//! claimed dynamic surface either carries a current assistive-tech proof row or is
//! auto-narrowed before Stable promotion, and screen-reader/focus-return/live-announcement
//! regressions on custom dynamic surfaces are named in the release packet rather than left
//! invisible.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- fixture-stale-proof-retest-pending
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- fixture-regression-blocked
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- fixture-waived-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- validate
//! ```

use aureline_shell::accessibility::certification::{
    seeded_m5_dynamic_a11y_certification, seeded_m5_dynamic_a11y_certification_regression_blocked,
    seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending,
    seeded_m5_dynamic_a11y_certification_waived_narrowed, M5DynamicA11yCertificationPacket,
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
            let packet = seeded_m5_dynamic_a11y_certification();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_dynamic_a11y_certification();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_dynamic_a11y_certification().render_markdown_summary()
            );
        }
        Some("fixture-stale-proof-retest-pending") => {
            let packet = seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-regression-blocked") => {
            let packet = seeded_m5_dynamic_a11y_certification_regression_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-waived-narrowed") => {
            let packet = seeded_m5_dynamic_a11y_certification_waived_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_dynamic_a11y_certification(),
                seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending(),
                seeded_m5_dynamic_a11y_certification_regression_blocked(),
                seeded_m5_dynamic_a11y_certification_waived_narrowed(),
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
    packet: &M5DynamicA11yCertificationPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "certification packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
