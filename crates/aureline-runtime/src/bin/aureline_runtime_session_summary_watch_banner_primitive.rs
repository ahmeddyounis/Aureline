//! Headless emitter for the M5 session-summary-bar / watch-mode-banner primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-session-summary-watch-banner-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-session-summary-watch-banner-primitive/`. The test-explorer status bar,
//! the editor status bar, the run-panel status, the headless/CLI status, and the
//! session/watch report export consumers read this matrix so one session-summary bar names
//! its session mode, exact selection, target/environment shorthand, running/backlog/retry
//! counts, and current watch state without collapsing distinct pending work into one
//! spinner, and one watch-mode banner names its live/reduced/polling/unavailable fidelity,
//! explains why it degraded, and preserves its last successful cycle.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- fixture-run-panel-status-preview-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- fixture-headless-cli-status-beta-narrowed
//! cargo run -q -p aureline-runtime --bin aureline_runtime_session_summary_watch_banner_primitive -- validate
//! ```

use aureline_runtime::implement_session_summary_bars_and_watch_mode_banners_with_exact_selection_running_backlog_retry_counts_live_reduced_polling_unavailable_state_last_successful_cycle_and_recover_pause_truth_across_claimed_m5_test_lanes::{
    seeded_m5_session_watch_status_headless_cli_status_beta_narrowed,
    seeded_m5_session_watch_status_packet,
    seeded_m5_session_watch_status_run_panel_status_preview_narrowed, M5SessionWatchStatusPacket,
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
            let packet = seeded_m5_session_watch_status_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_session_watch_status_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_session_watch_status_packet().render_matrix_csv()
            );
        }
        Some("fixture-run-panel-status-preview-narrowed") => {
            let packet = seeded_m5_session_watch_status_run_panel_status_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-headless-cli-status-beta-narrowed") => {
            let packet = seeded_m5_session_watch_status_headless_cli_status_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_session_watch_status_packet(),
                seeded_m5_session_watch_status_run_panel_status_preview_narrowed(),
                seeded_m5_session_watch_status_headless_cli_status_beta_narrowed(),
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

fn assert_valid(packet: &M5SessionWatchStatusPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "session watch status primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
