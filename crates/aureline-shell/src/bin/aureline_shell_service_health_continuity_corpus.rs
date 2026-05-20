//! Headless emitter for the service-health continuity-drill corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ops/m3/service_health_continuity/` so the support export,
//! release-truth packet, claim-matrix, and reviewer-facing report all
//! quote the same drill renders as the in-code corpus. The fixture
//! replay test in
//! `crates/aureline-shell/tests/service_health_continuity_fixtures.rs`
//! enforces that the disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one drill's aggregator JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_service_health_continuity_corpus -- drill <drill_id>
//!
//! # Print every drill, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_service_health_continuity_corpus -- all
//!
//! # Plaintext continuity block (per-drill, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_service_health_continuity_corpus -- plaintext
//!
//! # Stable corpus index — drill id, plane, narrative, expected rollup.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_service_health_continuity_corpus -- index
//!
//! # Refresh the on-disk fixtures under fixtures/ops/m3/service_health_continuity/.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures \
//!   fixtures/ops/m3/service_health_continuity
//! ```

use std::path::PathBuf;

use aureline_shell::service_health::continuity_corpus::{continuity_corpus, ContinuityScenario};

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
        Some("drill") => {
            let id = args
                .get(1)
                .ok_or("drill <drill_id> requires an id argument")?;
            print_drill(id)?;
            Ok(())
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
            emit_fixtures(out_dir)?;
            Ok(())
        }
        Some(other) => Err(format!("unknown subcommand: {other}").into()),
    }
}

fn print_all() -> Result<(), Box<dyn std::error::Error>> {
    for scenario in continuity_corpus() {
        println!("--- {} ---", scenario.drill_id);
        print_drill_record(&scenario)?;
    }
    Ok(())
}

fn print_drill(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = continuity_corpus()
        .into_iter()
        .find(|s| s.drill_id == id)
        .ok_or_else(|| format!("no continuity drill with id {id}"))?;
    print_drill_record(&scenario)
}

fn print_drill_record(scenario: &ContinuityScenario) -> Result<(), Box<dyn std::error::Error>> {
    let aggregator = scenario.aggregator();
    let body = serde_json::to_string_pretty(&aggregator)?;
    println!("{body}");
    Ok(())
}

fn print_plaintext() {
    for scenario in continuity_corpus() {
        println!("== {} ({}) ==", scenario.drill_id, scenario.plane.as_str());
        println!("{}", scenario.narrative);
        println!();
        println!("{}", scenario.aggregator().render_plaintext());
    }
}

fn print_index() {
    for scenario in continuity_corpus() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            scenario.drill_id,
            scenario.plane.as_str(),
            scenario.fixture_filename,
            scenario.expected_overall_contract_state.as_str(),
            scenario.expected_overall_local_continuity.as_str(),
            if scenario.expected_honesty_marker_present {
                "honesty=present"
            } else {
                "honesty=none"
            },
        );
    }
}

fn emit_fixtures(out_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(out_dir);
    std::fs::create_dir_all(&dir)?;
    for scenario in continuity_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let aggregator = scenario.aggregator();
        let mut body = serde_json::to_string_pretty(&aggregator)?;
        body.push('\n');
        std::fs::write(&path, body)?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
