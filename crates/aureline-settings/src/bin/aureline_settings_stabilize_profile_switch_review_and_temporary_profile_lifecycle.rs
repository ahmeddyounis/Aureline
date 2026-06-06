//! Headless emitter for the profile-switch lifecycle corpus.

use std::path::PathBuf;

use aureline_settings::stabilize_profile_switch_review_and_temporary_profile_lifecycle::{
    profile_switch_lifecycle_corpus, ProfileSwitchLifecycleScenario,
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
    for (idx, scenario) in profile_switch_lifecycle_corpus().iter().enumerate() {
        if idx > 0 {
            println!("---");
        }
        println!("{}", render_json(scenario));
    }
}

fn print_scenario(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scenario = profile_switch_lifecycle_corpus()
        .into_iter()
        .find(|scenario| scenario.scenario_id == id)
        .ok_or_else(|| format!("no scenario with id {id:?}"))?;
    println!("{}", render_json(&scenario));
    Ok(())
}

fn print_index() {
    println!("{:<40} {:<10} qualifies", "scenario_id", "claim");
    for scenario in profile_switch_lifecycle_corpus() {
        let record = scenario.record();
        let claim = format!("{:?}", record.stable_qualification.claim_class);
        println!(
            "{:<40} {:<10} {}",
            record.record_id, claim, record.stable_qualification.qualifies_stable
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    for scenario in profile_switch_lifecycle_corpus() {
        let path = dir.join(&scenario.fixture_filename);
        let json = render_json(&scenario);
        std::fs::write(&path, format!("{json}\n"))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

fn render_json(scenario: &ProfileSwitchLifecycleScenario) -> String {
    serde_json::to_string_pretty(&scenario.record()).expect("record serializes")
}
