//! Headless emitter for the M05-913 test-explorer / watch / triage component
//! consumer adoption lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-test-component-consumer-proof/`, its matrix CSV,
//! and the Markdown report. The status-bar summary, activity center, coverage /
//! flaky / snapshot intelligence, pipeline overlays, imported-CI views, and
//! support packets read this packet so every surface reuses one of the seven
//! frozen test components and keeps freshness, target class, watch state,
//! quarantine semantics, and imported-versus-live result origin aligned — and a
//! consumer whose result is imported, whose target drifted, whose watch degraded,
//! or whose quarantine visibility is restricted auto-narrows its claim rather than
//! letting an imported result read as a local rerun.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_component_consumers -- support-export
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_component_consumers -- report
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_component_consumers -- csv
//! cargo run -q -p aureline-runtime --bin aureline_runtime_test_component_consumers -- validate
//! ```

use aureline_runtime::add_shared_status_bar_activity_center_coverage_flaky_snapshot_pipeline_imported_ci_and_support_consumers_so_test_components_keep_freshness_target_watch_and_quarantine_language_aligned_across_claimed_m5_profiles::{
    seeded_m5_test_component_consumers_packet, TestConsumerPacket,
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
            let packet = seeded_m5_test_component_consumers_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_test_component_consumers_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_test_component_consumers_packet().render_matrix_csv()
            );
        }
        Some("validate") => {
            assert_valid(&seeded_m5_test_component_consumers_packet())?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &TestConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<String> = violations.iter().map(|v| v.to_string()).collect();
        Err(format!(
            "test component consumers failed validation: {}",
            tokens.join("; ")
        )
        .into())
    }
}
