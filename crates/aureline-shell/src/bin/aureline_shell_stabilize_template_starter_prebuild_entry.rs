//! Headless emitter for the template, starter, and prebuild entry disclosure
//! drill corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ux/m4/stabilize-template-starter-prebuild-entry/` so the support
//! export, diagnostics packet, Help/About, and reviewer-facing report all quote
//! the same record renders as the in-code corpus. The fixture-replay test in
//! `crates/aureline-shell/tests/stabilize_template_starter_prebuild_entry_fixtures.rs`
//! enforces that the disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's record JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_stabilize_template_starter_prebuild_entry -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_stabilize_template_starter_prebuild_entry -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_stabilize_template_starter_prebuild_entry -- plaintext
//!
//! # Stable corpus index — scenario id, kind, mode, honesty, bypass count.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_stabilize_template_starter_prebuild_entry -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_stabilize_template_starter_prebuild_entry -- emit-fixtures \
//!   fixtures/ux/m4/stabilize-template-starter-prebuild-entry
//! ```

use std::path::PathBuf;

use aureline_shell::stabilize_template_starter_prebuild_entry::{
    template_starter_prebuild_entry_corpus, TemplateStarterPrebuildEntryScenario,
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
    let scenarios = template_starter_prebuild_entry_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = template_starter_prebuild_entry_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in template_starter_prebuild_entry_corpus().iter().enumerate() {
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
        "{:<42} {:<12} {:<28} {:<7} {}",
        "scenario_id", "entry_kind", "resulting_mode", "honesty", "bypass_count"
    );
    for scenario in template_starter_prebuild_entry_corpus() {
        let record = scenario.record();
        println!(
            "{:<42} {:<12} {:<28} {:<7} {}",
            scenario.scenario_id,
            record.accelerator_identity.entry_kind.as_str(),
            record.resulting_mode.as_str(),
            record.honesty_marker_present,
            record.bypass_paths.len(),
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in template_starter_prebuild_entry_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &TemplateStarterPrebuildEntryScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
