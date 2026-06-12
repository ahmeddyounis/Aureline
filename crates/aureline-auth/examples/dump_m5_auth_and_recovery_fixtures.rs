//! Emits the seeded M5 managed auth-and-recovery fixtures.
//!
//! The example is a one-shot helper that lists the corpus scenarios, prints a
//! single scenario's canonical record, or regenerates the on-disk fixture
//! corpus so reviewer-facing fixtures stay byte-for-byte in step with the typed
//! model:
//!
//! ```sh
//! cargo run -q -p aureline-auth --example dump_m5_auth_and_recovery_fixtures -- list
//! cargo run -q -p aureline-auth --example dump_m5_auth_and_recovery_fixtures -- calm_managed_baseline
//! cargo run -q -p aureline-auth --example dump_m5_auth_and_recovery_fixtures -- write fixtures/auth/m5_auth_and_recovery
//! ```

use aureline_auth::m5_auth_and_recovery::m5_auth_and_recovery_corpus;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus = m5_auth_and_recovery_corpus();

    match args.first().map(String::as_str) {
        Some("list") | None => {
            for scenario in &corpus {
                println!(
                    "{}\t{}\t{:?}\t{:?}",
                    scenario.scenario_id,
                    scenario.fixture_filename,
                    scenario.expected_claim_class,
                    scenario.expected_continuity_ceiling,
                );
            }
        }
        Some("write") => {
            let dir = args.get(1).ok_or("usage: write <fixture-dir>")?.to_owned();
            for scenario in &corpus {
                let path = format!("{dir}/{}", scenario.fixture_filename);
                let json = serde_json::to_string_pretty(&scenario.record())?;
                std::fs::write(&path, format!("{json}\n"))?;
                println!("wrote {path}");
            }
        }
        Some(scenario_id) => {
            let scenario = corpus
                .iter()
                .find(|scenario| scenario.scenario_id == scenario_id)
                .ok_or_else(|| format!("unknown scenario: {scenario_id}"))?;
            let json = serde_json::to_string_pretty(&scenario.record())?;
            println!("{json}");
        }
    }
    Ok(())
}
