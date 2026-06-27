//! Headless emitter for the M5 runbook governance matrix.
//!
//! The bin is the only mint-from-truth path for the governance support export and Markdown proof
//! checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-governance.json`, the stale / missing-proof / waived drill
//! fixtures under `fixtures/runbooks/m5-governance-drills/`, and the operator-scenario execution
//! records under `fixtures/runbooks/m5-operator-scenarios/`. Incident workspaces, operator
//! dashboards, docs/help, companions, support exports, and the release center consume this one
//! inventory so each claimed runbook-backed surface either binds a mapped object with current proof
//! or is auto-narrowed / blocked before Stable promotion, with the gap named rather than hidden.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- support-export
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- matrix
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- fixture-stale-proof-narrowed
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- fixture-missing-proof-blocked
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- fixture-waived-narrowed
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- scenario <id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance -- validate
//! ```

use aureline_runbooks::m5_runbook_governance::{
    seeded_m5_runbook_governance_packet, seeded_m5_runbook_governance_packet_missing_proof_blocked,
    seeded_m5_runbook_governance_packet_stale_proof_narrowed,
    seeded_m5_runbook_governance_packet_waived_narrowed, seeded_operator_scenario_records,
    M5RunbookGovernancePacket,
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
            let packet = seeded_m5_runbook_governance_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("matrix") => {
            let packet = seeded_m5_runbook_governance_packet();
            assert_valid(&packet)?;
            println!("{}", packet.matrix_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_governance_packet().render_markdown_summary()
            );
        }
        Some("fixture-stale-proof-narrowed") => {
            let packet = seeded_m5_runbook_governance_packet_stale_proof_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-missing-proof-blocked") => {
            let packet = seeded_m5_runbook_governance_packet_missing_proof_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-waived-narrowed") => {
            let packet = seeded_m5_runbook_governance_packet_waived_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("scenario") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let record = seeded_operator_scenario_records()
                .into_iter()
                .find(|r| r.execution_id == id)
                .ok_or_else(|| format!("unknown scenario id: {id}"))?;
            let violations = record.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(
                    format!("scenario {id} failed validation: {}", tokens.join(",")).into(),
                );
            }
            println!("{}", record.export_safe_json());
        }
        Some("scenarios") => {
            for record in seeded_operator_scenario_records() {
                let violations = record.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "scenario {} failed validation: {}",
                        record.execution_id,
                        tokens.join(",")
                    )
                    .into());
                }
                println!("{}", record.execution_id);
            }
        }
        Some("validate") => {
            for packet in [
                seeded_m5_runbook_governance_packet(),
                seeded_m5_runbook_governance_packet_stale_proof_narrowed(),
                seeded_m5_runbook_governance_packet_missing_proof_blocked(),
                seeded_m5_runbook_governance_packet_waived_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            for record in seeded_operator_scenario_records() {
                let violations = record.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "scenario {} failed validation: {}",
                        record.execution_id,
                        tokens.join(",")
                    )
                    .into());
                }
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5RunbookGovernancePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("governance packet failed validation: {}", tokens.join(",")).into())
    }
}
