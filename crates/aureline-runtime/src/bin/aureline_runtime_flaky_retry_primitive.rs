//! Headless emitter for the M5 flaky-state-badge / retry-history-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-flaky-retry-primitive-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-flaky-retry-primitive/`. The flaky
//! dashboard, the editor / test-tree badge, the retry-history panel, the headless/CLI
//! flaky-retry surface, and the flaky-retry export consumers read this matrix so one flaky-state
//! badge names its classification, confidence, retry window, classifier source, last outcome,
//! and mute status without letting one intermittent failure masquerade as reproduced flakiness,
//! and one retry-history row preserves its ordered outcomes, environment/build/runtime deltas,
//! attempt origin, and a rerun-or-open-logs path back to the raw attempt.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- fixture-flaky-dashboard-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- fixture-editor-badge-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_flaky_retry_primitive -- validate
//! ```

use aureline_runtime::implement_flaky_state_badges_and_retry_history_rows_with_controlled_verdict_vocabulary_classifier_confidence_retry_window_visibility_environment_drift_notes_and_rerun_or_open_logs_parity_across_claimed_m5_quality_surfaces::{
    seeded_m5_flaky_retry_components_editor_badge_beta_narrowed,
    seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed,
    seeded_m5_flaky_retry_components_packet, M5FlakyRetryComponentsPacket,
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
            let packet = seeded_m5_flaky_retry_components_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_flaky_retry_components_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_flaky_retry_components_packet().render_matrix_csv()
            );
        }
        Some("fixture-flaky-dashboard-preview-narrowed") => {
            let packet = seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-editor-badge-beta-narrowed") => {
            let packet = seeded_m5_flaky_retry_components_editor_badge_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_flaky_retry_components_packet(),
                seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed(),
                seeded_m5_flaky_retry_components_editor_badge_beta_narrowed(),
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

fn assert_valid(packet: &M5FlakyRetryComponentsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "flaky retry components primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
