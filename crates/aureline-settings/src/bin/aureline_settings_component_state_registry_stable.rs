//! Headless emitter for the stable component-state registry certification
//! corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/` so
//! the settings / design-system panel, command palette, status bar, diagnostics
//! packet, support export, Help/About, design-QA packets, and reviewer-facing
//! reports all quote the same record renders as the in-code corpus. The
//! fixture-replay test in
//! `crates/aureline-settings/tests/component_state_registry_stable_fixtures.rs`
//! enforces that the disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's record JSON.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_component_state_registry_stable -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_component_state_registry_stable -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_component_state_registry_stable -- plaintext
//!
//! # Stable corpus index — scenario id, posture, claim, surface marker.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_component_state_registry_stable -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-settings \
//!   --bin aureline_settings_component_state_registry_stable -- emit-fixtures \
//!   fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity
//! ```

use std::path::PathBuf;

use aureline_settings::component_state_registry_stable::{
    component_state_registry_corpus, ComponentStateRegistryScenario,
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
    let scenarios = component_state_registry_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = component_state_registry_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in component_state_registry_corpus().iter().enumerate() {
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
        "{:<52} {:<52} {:<10} {:<14} marker",
        "scenario_id", "posture", "claim", "qualifies"
    );
    for scenario in component_state_registry_corpus() {
        let record = scenario.record();
        println!(
            "{:<52} {:<52} {:<10} {:<14} {}",
            record.record_id,
            record.posture_id,
            record.stable_qualification.claim_class.as_str(),
            record.stable_qualification.qualifies_stable,
            record.surface_lifecycle_marker.as_str(),
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in component_state_registry_corpus() {
        let path = dir.join(&scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &ComponentStateRegistryScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
