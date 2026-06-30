//! Headless emitter for the M5 handoff-continuity scenario set.
//!
//! The bin is the only mint-from-truth path for the support export checked in at
//! `artifacts/help/m5-handoff-continuity-proof/draft_state_set.json`, the
//! governance Markdown summary
//! `artifacts/help/m5-handoff-continuity-governance.md`, the matrix CSV
//! `artifacts/help/m5-handoff-continuity-drafts.csv`, and the narrowed fixtures
//! under `fixtures/help/handoff-continuity/`. Help/About, support, and
//! community-handoff surfaces read this set so a drafted report — its text,
//! attachments, redaction choices, and intended target class — survives a blocked,
//! offline, or failed handoff instead of being discarded.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- fixture-offline-security-draft
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- fixture-cleared-draft
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_handoff_continuity -- validate
//! ```

use aureline_shell::m5_handoff_continuity::{
    seeded_cleared_draft_state, seeded_m5_handoff_continuity_scenario_set,
    seeded_offline_security_draft_state,
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
            let set = seeded_m5_handoff_continuity_scenario_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("governance") => {
            let set = seeded_m5_handoff_continuity_scenario_set();
            set.validate()?;
            print!("{}", set.render_markdown_summary());
        }
        Some("csv") => {
            let set = seeded_m5_handoff_continuity_scenario_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("fixture-offline-security-draft") => {
            let draft = seeded_offline_security_draft_state();
            draft.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&draft).expect("draft serializes")
            );
        }
        Some("fixture-cleared-draft") => {
            let draft = seeded_cleared_draft_state();
            draft.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&draft).expect("draft serializes")
            );
        }
        Some("validate") => {
            seeded_m5_handoff_continuity_scenario_set().validate()?;
            seeded_offline_security_draft_state().validate()?;
            seeded_cleared_draft_state().validate()?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
