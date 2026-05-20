//! Headless emitter for the dashboard & queue truth drill corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ops/m3/dashboard_and_queue_truth/` so the support export,
//! diagnostics packet, claim matrix, and reviewer-facing report all quote the
//! same view renders as the in-code corpus. The fixture-replay test in
//! `crates/aureline-shell/tests/dashboard_truth_fixtures.rs` enforces that the
//! disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's view JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_dashboard_truth_corpus -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_dashboard_truth_corpus -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_dashboard_truth_corpus -- plaintext
//!
//! # Stable corpus index — scenario id, surface, expected rollups.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_dashboard_truth_corpus -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures \
//!   fixtures/ops/m3/dashboard_and_queue_truth
//! ```

use std::path::PathBuf;

use aureline_shell::dashboard_truth::{dashboard_truth_corpus, DashboardTruthScenario};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("all") => print_all(),
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
            let out_dir = args
                .get(1)
                .ok_or("emit-fixtures <out_dir> requires an output directory argument")?;
            emit_fixtures(out_dir)
        }
        Some(other) => Err(format!("unknown subcommand: {other}").into()),
    }
}

fn print_all() -> Result<(), Box<dyn std::error::Error>> {
    for scenario in dashboard_truth_corpus() {
        println!("--- {} ---", scenario.scenario_id);
        print_view(&scenario)?;
    }
    Ok(())
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = dashboard_truth_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no dashboard-truth scenario with id {id}"))?;
    print_view(&scenario)
}

fn print_view(scenario: &DashboardTruthScenario) -> Result<(), Box<dyn std::error::Error>> {
    let view = scenario.view();
    println!("{}", serde_json::to_string_pretty(&view)?);
    Ok(())
}

fn print_plaintext() {
    for scenario in dashboard_truth_corpus() {
        println!(
            "== {} ({}) ==",
            scenario.scenario_id,
            scenario.surface.as_str()
        );
        println!("{}", scenario.narrative);
        println!();
        println!("{}", scenario.view().render_plaintext());
    }
}

fn print_index() {
    for scenario in dashboard_truth_corpus() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\thidden={}",
            scenario.scenario_id,
            scenario.surface.as_str(),
            scenario.fixture_filename,
            scenario.expected_overall_effective_state.as_str(),
            scenario.expected_overall_freshness.as_str(),
            if scenario.expected_honesty_marker_present {
                "honesty=present"
            } else {
                "honesty=none"
            },
            scenario.expected_hidden_total,
        );
    }
}

fn emit_fixtures(out_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(out_dir);
    std::fs::create_dir_all(&dir)?;
    for scenario in dashboard_truth_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let view = scenario.view();
        let mut body = serde_json::to_string_pretty(&view)?;
        body.push('\n');
        std::fs::write(&path, body)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
