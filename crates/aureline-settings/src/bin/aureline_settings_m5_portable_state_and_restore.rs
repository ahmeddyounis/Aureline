//! Headless emitter for the M5 portable-state export/import and restore corpus.

use std::path::PathBuf;

use aureline_settings::m5_portable_state_and_restore::{
    portable_state_restore_corpus, PortableStateRestoreScenario,
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
    let scenarios = portable_state_restore_corpus();
    for (idx, scenario) in scenarios.iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = portable_state_restore_corpus()
        .into_iter()
        .find(|scenario| scenario.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_plaintext() {
    for (idx, scenario) in portable_state_restore_corpus().iter().enumerate() {
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
        "{:<48} {:<18} {:<18} ceiling",
        "scenario_id", "claim", "qualifies_exact"
    );
    for scenario in portable_state_restore_corpus() {
        let record = scenario.record();
        let claim = format!("{:?}", record.fidelity_qualification.claim_class);
        println!(
            "{:<48} {:<18} {:<18} {}",
            record.record_id,
            claim,
            record.fidelity_qualification.qualifies_exact,
            record
                .fidelity_qualification
                .effective_fidelity_ceiling
                .as_str(),
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in portable_state_restore_corpus() {
        let path = dir.join(&scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &PortableStateRestoreScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
