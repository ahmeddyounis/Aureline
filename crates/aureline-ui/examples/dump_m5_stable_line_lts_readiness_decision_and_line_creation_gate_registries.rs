//! Headless emitter for the M5 LTS-readiness-decision and line-creation-gate registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- lts-readiness-decision-table
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- fixture-lts-readiness-decision-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- fixture-line-creation-gate-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- validate
//! ```

use aureline_ui::m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries::{
    seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries,
    seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_line_creation_gate_preview_narrowed,
    seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_lts_readiness_decision_beta_narrowed,
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket,
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
            let packet =
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries()
                    .render_matrix_csv()
            );
        }
        Some("lts-readiness-decision-table") => {
            print!(
                "{}",
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries()
                    .render_lts_readiness_decision_table()
            );
        }
        Some("fixture-lts-readiness-decision-beta-narrowed") => {
            let packet =
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_lts_readiness_decision_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-line-creation-gate-preview-narrowed") => {
            let packet =
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_line_creation_gate_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries(),
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_lts_readiness_decision_beta_narrowed(),
                seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_line_creation_gate_preview_narrowed(),
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
    packet: &M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
