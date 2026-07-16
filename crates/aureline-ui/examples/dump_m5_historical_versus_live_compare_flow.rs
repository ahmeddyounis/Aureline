//! Headless emitter for the M5 historical-versus-live compare-flow packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-historical-versus-live-compare/`, its matrix CSV, the Markdown summary, and the
//! narrowed fixtures under `fixtures/recovery/m5-historical-versus-live-compare/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- support-export
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- report
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- csv
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- fixture-missing-target-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- fixture-policy-blocked-narrowed
//! cargo run -p aureline-ui --example dump_m5_historical_versus_live_compare_flow -- validate
//! ```

use aureline_ui::m5_historical_versus_live_compare_flow::{
    seeded_m5_historical_versus_live_compare_flow,
    seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed,
    seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed,
    M5HistoricalVersusLiveCompareFlowPacket,
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
            let packet = seeded_m5_historical_versus_live_compare_flow();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_historical_versus_live_compare_flow().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_historical_versus_live_compare_flow().render_matrix_csv()
            );
        }
        Some("fixture-missing-target-narrowed") => {
            let packet = seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-policy-blocked-narrowed") => {
            let packet = seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_historical_versus_live_compare_flow(),
                seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed(),
                seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed(),
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
    packet: &M5HistoricalVersusLiveCompareFlowPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "compare-flow packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
