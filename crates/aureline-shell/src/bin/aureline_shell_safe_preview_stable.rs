//! Headless emitter for the stable safe-preview corpus.
//!
//! Mints the on-disk fixtures under
//! `fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/` so
//! the shell surface, the activity center, the CLI inspector, Help/About, and the
//! diagnostics support export all render the same record as the in-code corpus,
//! which is itself a projection of the live content-safety detector and the
//! trust-class / representation vocabulary. The fixture-replay test in
//! `crates/aureline-shell/tests/safe_preview_stable_fixtures.rs` enforces that the
//! disk content stays in sync with the code.
//!
//! Subcommands:
//!
//! ```sh
//! # Print one scenario's record JSON.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_safe_preview_stable -- scenario <scenario_id>
//!
//! # Print every scenario, separated by --- markers.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_safe_preview_stable -- all
//!
//! # Plaintext truth block (per-scenario, suitable for support export).
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_safe_preview_stable -- plaintext
//!
//! # Stable corpus index — scenario id, surface class, claim, marker.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_safe_preview_stable -- index
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_safe_preview_stable -- emit-fixtures \
//!   fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and
//! ```

use std::path::PathBuf;

use aureline_shell::shell_safe_preview_stable::{safe_preview_corpus, SafePreviewScenario};

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
    let scenarios = safe_preview_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = safe_preview_corpus()
        .into_iter()
        .find(|s| s.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in safe_preview_corpus().iter().enumerate() {
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
        "{:<46} {:<22} {:<10} {:<10} marker",
        "scenario_id", "surface_class", "claim", "qualifies"
    );
    for scenario in safe_preview_corpus() {
        let record = scenario.record();
        println!(
            "{:<46} {:<22} {:<10} {:<10} {}",
            record.record_id,
            record.surface_class.as_str(),
            record.stable_qualification.claim_class.as_str(),
            record.stable_qualification.qualifies_stable,
            record.surface_lifecycle_marker.as_str(),
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in safe_preview_corpus() {
        let path = dir.join(&scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &SafePreviewScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
