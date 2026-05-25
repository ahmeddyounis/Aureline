//! Headless emitter for the stable Start Center / recent-work / workspace-switcher
//! target-kind disclosure corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/` so the
//! shell rows, command palette, menus, diagnostics packet, support export,
//! Help/About, and reviewer-facing report all quote the same record renders as
//! the in-code corpus. The fixture-replay test in
//! `crates/aureline-shell/tests/start_center_stable_fixtures.rs` enforces that the
//! disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's record JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_start_center_stable -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_start_center_stable -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_start_center_stable -- plaintext
//!
//! # Stable corpus index — scenario id, target kind, class, failure, trust.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_start_center_stable -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_start_center_stable -- emit-fixtures \
//!   fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace
//! ```

use std::path::PathBuf;

use aureline_shell::start_center_stable::{
    entry_target_disclosure_corpus, EntryTargetDisclosureScenario,
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
        None | Some("all") => {
            print_all();
            Ok(())
        }
        Some("scenario") => {
            let id = args
                .get(1)
                .ok_or("scenario <scenario_id> requires an id argument")?;
            print_scenario(id)
        }
        Some("plaintext") => {
            print_plaintext();
            Ok(())
        }
        Some("index") => {
            print_index();
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures <dir> requires a target directory")?;
            emit_fixtures(PathBuf::from(dir))
        }
        Some(other) => Err(format!("unknown subcommand: {other}").into()),
    }
}

fn print_all() {
    let scenarios = entry_target_disclosure_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = entry_target_disclosure_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in entry_target_disclosure_corpus().iter().enumerate() {
        if idx > 0 {
            println!();
        }
        for line in scenario.record().support_export_lines() {
            println!("{line}");
        }
    }
}

fn print_index() {
    println!(
        "{:<46} {:<24} {:<14} {:<20} {:<18} restore honesty",
        "scenario_id", "target_kind", "class", "failure_state", "trust_state"
    );
    for scenario in entry_target_disclosure_corpus() {
        let record = scenario.record();
        println!(
            "{:<46} {:<24} {:<14} {:<20} {:<18} {:<8} {}",
            record.record_id,
            record.target_kind.as_str(),
            record.target_class.as_str(),
            record.failure_state.as_str(),
            record.trust_state.as_str(),
            record.restore_availability.as_str(),
            record.honesty_marker_present,
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in entry_target_disclosure_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &EntryTargetDisclosureScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
