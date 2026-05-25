//! Headless emitter for the warm-startup / warm-restore / first-useful-work
//! drill corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/` so the
//! support export, diagnostics packet, Help/About, and reviewer-facing report
//! all quote the same record renders as the in-code corpus. The fixture-replay
//! test in `crates/aureline-shell/tests/warm_continuity_fixtures.rs` enforces
//! that the disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's record JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_warm_continuity_corpus -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_warm_continuity_corpus -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_warm_continuity_corpus -- plaintext
//!
//! # Stable corpus index — scenario id, cause, restore class, rollups.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_warm_continuity_corpus -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_warm_continuity_corpus -- emit-fixtures \
//!   fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful
//! ```

use std::path::PathBuf;

use aureline_shell::warm_continuity::{warm_continuity_corpus, WarmContinuityScenario};

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
    let scenarios = warm_continuity_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = warm_continuity_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in warm_continuity_corpus().iter().enumerate() {
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
        "{:<26} {:<26} {:<20} {:<22} honesty exact/partial/review",
        "scenario_id", "entry_cause", "restore_class", "landing_route"
    );
    for scenario in warm_continuity_corpus() {
        let record = scenario.record();
        println!(
            "{:<26} {:<26} {:<20} {:<22} {:<7} {}/{}/{}",
            scenario.scenario_id,
            record.entry_cause.as_str(),
            record.restore.restore_class.as_str(),
            record.landing.selected_route.as_str(),
            record.honesty_marker_present,
            record.summary_counts.restored_exactly_count,
            record.summary_counts.restored_partially_count,
            record.summary_counts.needs_review_count,
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in warm_continuity_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &WarmContinuityScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
