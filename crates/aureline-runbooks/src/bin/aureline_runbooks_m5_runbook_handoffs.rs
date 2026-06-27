//! Headless emitter for the M5 runbook control-plane handoff register.
//!
//! The bin is the only mint-from-truth path for the handoff-register support export and Markdown
//! proof checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-handoff-register.json`, and the per-handoff projection fixtures
//! under `fixtures/runbooks/m5-handoff-packets/`. The incident workspace, operator history, support
//! exports, and docs/help all read this one register, so every console/browser pivot shows the same
//! destination class, reason, reference-plane state, and return anchor wherever it is rendered or
//! exported, and no pivot is a hidden escape or a reference doc masquerading as in-product control.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_handoffs -- register
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_handoffs -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_handoffs -- handoff <id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_handoffs -- handoffs
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_handoffs -- validate
//! ```

use aureline_runbooks::m5_runbook_handoffs::{
    seeded_m5_runbook_handoff_register, seeded_runbook_handoff_packets, M5RunbookHandoffRegister,
    RunbookHandoffProjection,
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
        Some("register") | None => {
            let register = seeded_m5_runbook_handoff_register();
            assert_valid(&register)?;
            println!("{}", register.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_handoff_register().render_markdown_summary()
            );
        }
        Some("handoff") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_runbook_handoff_packets()
                .into_iter()
                .find(|p| p.handoff_id == id)
                .ok_or_else(|| format!("unknown handoff id: {id}"))?;
            let violations = packet.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(format!("handoff {id} failed validation: {}", tokens.join(",")).into());
            }
            let projection = RunbookHandoffProjection::derive(&packet);
            println!(
                "{}",
                serde_json::to_string_pretty(&projection).expect("projection serializes")
            );
        }
        Some("handoffs") => {
            for packet in seeded_runbook_handoff_packets() {
                let violations = packet.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "handoff {} failed validation: {}",
                        packet.handoff_id,
                        tokens.join(",")
                    )
                    .into());
                }
                println!("{}", packet.handoff_id);
            }
        }
        Some("validate") => {
            let register = seeded_m5_runbook_handoff_register();
            assert_valid(&register)?;
            for packet in seeded_runbook_handoff_packets() {
                let violations = packet.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "handoff {} failed validation: {}",
                        packet.handoff_id,
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

fn assert_valid(register: &M5RunbookHandoffRegister) -> Result<(), Box<dyn std::error::Error>> {
    let violations = register.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("handoff register failed validation: {}", tokens.join(",")).into())
    }
}
