//! Headless emitter for the M5 runbook execution history.
//!
//! The bin is the only mint-from-truth path for the execution-history support export and Markdown
//! proof checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-execution-history.json`, and the operator-scenario execution-record
//! fixtures under `fixtures/runbooks/m5-operator-scenarios/`. Operator history, support exports, and
//! incident packets all read this one history, so every execution row shows the same step class,
//! actor, target, outcome, approval, preview-hash reuse, and evidence wherever it is rendered or
//! exported, and no row mints a hidden privileged mutate channel.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_executions -- history
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_executions -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_executions -- scenario <id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_executions -- scenarios
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_executions -- validate
//! ```

use aureline_runbooks::m5_runbook_executions::{
    seeded_m5_runbook_execution_history, seeded_runbook_execution_records,
    M5RunbookExecutionHistory,
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
        Some("history") | None => {
            let history = seeded_m5_runbook_execution_history();
            assert_valid(&history)?;
            println!("{}", history.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_execution_history().render_markdown_summary()
            );
        }
        Some("scenario") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let record = seeded_runbook_execution_records()
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
            for record in seeded_runbook_execution_records() {
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
            let history = seeded_m5_runbook_execution_history();
            assert_valid(&history)?;
            for record in seeded_runbook_execution_records() {
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

fn assert_valid(history: &M5RunbookExecutionHistory) -> Result<(), Box<dyn std::error::Error>> {
    let violations = history.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("execution history failed validation: {}", tokens.join(",")).into())
    }
}
