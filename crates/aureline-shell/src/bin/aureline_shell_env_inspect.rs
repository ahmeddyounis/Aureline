//! Headless inspector for the env-inspect contract.
//!
//! Emits the same snapshot and support-export records consumed by the live
//! shell, the chrome panel projection, and the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- snapshot [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- plaintext [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- panel [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- scenarios
//! ```
//!
//! Valid scenarios: `local_terminal`, `remote_attach_pending_trust`,
//! `container_devcontainer`, `managed_workspace_restricted`.

use aureline_shell::env_inspect::{
    seeded_snapshot, seeded_support_export, EnvInspectPanelProjection, EnvInspectSeededScenario,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let scenario_arg = args.get(1).map(String::as_str);
    match args.first().map(String::as_str) {
        Some("scenarios") => {
            for scenario in EnvInspectSeededScenario::ALL {
                println!("{}", scenario.as_str());
            }
        }
        Some("snapshot") | None => {
            let scenario = parse_scenario(scenario_arg)?;
            print_json(&seeded_snapshot(scenario))?;
        }
        Some("plaintext") => {
            let scenario = parse_scenario(scenario_arg)?;
            let snapshot = seeded_snapshot(scenario);
            print!("{}", snapshot.render_plaintext());
        }
        Some("panel") => {
            let scenario = parse_scenario(scenario_arg)?;
            let snapshot = seeded_snapshot(scenario);
            print_json(&EnvInspectPanelProjection::from_snapshot(snapshot))?;
        }
        Some("support-export") => {
            let export = seeded_support_export(
                "support-export:env-inspect-beta:001",
                "2026-05-15T00:00:00Z",
            );
            print_json(&export)?;
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_scenario(
    raw: Option<&str>,
) -> Result<EnvInspectSeededScenario, Box<dyn std::error::Error>> {
    let raw = raw.unwrap_or("local_terminal");
    match raw {
        "local_terminal" => Ok(EnvInspectSeededScenario::LocalTerminal),
        "remote_attach_pending_trust" => Ok(EnvInspectSeededScenario::RemoteAttachPendingTrust),
        "container_devcontainer" => Ok(EnvInspectSeededScenario::ContainerDevcontainer),
        "managed_workspace_restricted" => Ok(EnvInspectSeededScenario::ManagedWorkspaceRestricted),
        other => Err(format!("unknown scenario: {other}").into()),
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
