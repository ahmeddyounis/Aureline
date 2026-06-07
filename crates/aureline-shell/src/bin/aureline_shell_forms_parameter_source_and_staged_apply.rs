//! Headless emitter for the stable structured-input truth corpus.
//!
//! ```sh
//! cargo run -q -p aureline-shell \
//!   --bin aureline_shell_forms_parameter_source_and_staged_apply -- emit-fixtures \
//!   fixtures/forms/m4/forms-parameter-source-and-staged-apply
//! ```

use std::path::PathBuf;

use aureline_shell::forms_parameter_source_and_staged_apply::{
    forms_parameter_source_and_staged_apply_corpus, FormTruthScenario,
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
    for (idx, scenario) in forms_parameter_source_and_staged_apply_corpus()
        .iter()
        .enumerate()
    {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = forms_parameter_source_and_staged_apply_corpus()
        .into_iter()
        .find(|scenario| scenario.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_index() {
    println!(
        "{:<48} {:<22} {:<20} {}",
        "scenario_id", "surface", "client_scope", "apply_timing"
    );
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let record = scenario.record();
        println!(
            "{:<48} {:<22} {:<20} {}",
            scenario.scenario_id,
            record.surface_class.as_str(),
            record.client_scope.as_str(),
            record.staged_apply.apply_timing.as_str()
        );
    }
}

fn print_plaintext() {
    for (idx, scenario) in forms_parameter_source_and_staged_apply_corpus()
        .iter()
        .enumerate()
    {
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
    for scenario in forms_parameter_source_and_staged_apply_corpus() {
        let path = dir.join(scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &FormTruthScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
