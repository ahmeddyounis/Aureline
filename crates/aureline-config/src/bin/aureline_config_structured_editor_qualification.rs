//! Headless emitter for the structured editor qualification corpus.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- all
//! cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- index
//! cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- plaintext
//! cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- scenario source_effective_live_preserving
//! cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- emit-fixtures fixtures/config/m4/structured-config-manifest-environment-editor-qualification
//! ```

use std::path::PathBuf;

use aureline_config::structured_config_manifest_environment_editor_qualification::{
    structured_editor_corpus, StructuredEditorScenario,
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
        Some("index") => {
            print_index();
            Ok(())
        }
        Some("plaintext") => {
            print_plaintext();
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
    for (idx, scenario) in structured_editor_corpus().iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = structured_editor_corpus()
        .into_iter()
        .find(|scenario| scenario.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_index() {
    println!(
        "{:<36} {:<22} {:<10} qualifies",
        "scenario_id", "artifact_class", "claim"
    );
    for scenario in structured_editor_corpus() {
        let record = scenario.record();
        println!(
            "{:<36} {:<22} {:<10} {}",
            scenario.scenario_id,
            record.artifact_class.as_str(),
            record.qualification.claim_class.as_str(),
            record.qualification.qualifies_stable,
        );
    }
}

fn print_plaintext() {
    for (idx, scenario) in structured_editor_corpus().iter().enumerate() {
        if idx > 0 {
            println!();
        }
        for line in scenario.record().support_export_lines() {
            println!("{line}");
        }
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in structured_editor_corpus() {
        let path = dir.join(&scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &StructuredEditorScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
