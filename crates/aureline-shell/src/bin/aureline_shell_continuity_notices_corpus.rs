//! Headless emitter for the maintenance & failover continuity-notice drill
//! corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ops/m3/maintenance_and_failover_notices/` so the support export,
//! durable history, diagnostics packet, and reviewer-facing report all quote
//! the same view renders as the in-code corpus. The fixture-replay test in
//! `crates/aureline-shell/tests/continuity_notices_fixtures.rs` enforces that
//! the disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's view JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_continuity_notices_corpus -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_continuity_notices_corpus -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_continuity_notices_corpus -- plaintext
//!
//! # Stable corpus index — scenario id, category, effective freshness, rollups.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_continuity_notices_corpus -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_continuity_notices_corpus -- emit-fixtures \
//!   fixtures/ops/m3/maintenance_and_failover_notices
//! ```

use std::path::PathBuf;

use aureline_shell::continuity_notices::{continuity_notice_corpus, ContinuityNoticeScenario};

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
    for scenario in continuity_notice_corpus() {
        println!("--- {} ---", scenario.scenario_id);
        print_view(&scenario)?;
    }
    Ok(())
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = continuity_notice_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no continuity-notice scenario with id {id}"))?;
    print_view(&scenario)
}

fn print_view(scenario: &ContinuityNoticeScenario) -> Result<(), Box<dyn std::error::Error>> {
    let view = scenario.view();
    println!("{}", serde_json::to_string_pretty(&view)?);
    Ok(())
}

fn print_plaintext() {
    for scenario in continuity_notice_corpus() {
        println!(
            "== {} ({}) ==",
            scenario.scenario_id,
            scenario.expected_category.as_str()
        );
        println!("{}", scenario.narrative);
        println!();
        println!("{}", scenario.view().render_plaintext());
    }
}

fn print_index() {
    for scenario in continuity_notice_corpus() {
        println!(
            "{}\t{}\t{}\t{}\t{}\tpreserved={}\tchanged_axes={}\tunresolved={}",
            scenario.scenario_id,
            scenario.fixture_filename,
            scenario.expected_category.as_str(),
            scenario.expected_effective_freshness.as_str(),
            if scenario.expected_honesty_marker_present {
                "honesty=present"
            } else {
                "honesty=none"
            },
            scenario.expected_preserved_intent_count,
            scenario.expected_changed_boundary_axis_count,
            scenario.expected_boundary_change_unresolved,
        );
    }
}

fn emit_fixtures(out_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(out_dir);
    std::fs::create_dir_all(&dir)?;
    for scenario in continuity_notice_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let view = scenario.view();
        let mut body = serde_json::to_string_pretty(&view)?;
        body.push('\n');
        std::fs::write(&path, body)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
