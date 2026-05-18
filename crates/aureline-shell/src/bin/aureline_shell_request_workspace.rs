//! Headless inspector for the request-workspace alpha contract.
//!
//! Emits the same canonical records, send-inspector reports, panel
//! projections, and support-export packets consumed by the live shell, the
//! chrome panel projection, and the integration test that replays the
//! checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- record [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- send-inspector [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- panel [scenario]
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- plaintext
//! cargo run -q -p aureline-shell --bin aureline_shell_request_workspace -- scenarios
//! ```
//!
//! Valid scenarios: `local_read_only_get`,
//! `remote_mutating_post_stale_schema`, `managed_delete_missing_schema`,
//! `remote_graphql_no_auth`, `imported_stale_assertion_export_truth`.

use aureline_shell::request_workspace::{
    render_support_export_plaintext, seeded_record, seeded_send_inspector_report,
    seeded_support_export, RequestWorkspacePanelProjection, RequestWorkspaceSeededScenario,
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
            for scenario in RequestWorkspaceSeededScenario::ALL {
                println!("{}", scenario.as_str());
            }
        }
        Some("record") | None => {
            let scenario = parse_scenario(scenario_arg)?;
            print_json(&seeded_record(scenario))?;
        }
        Some("send-inspector") => {
            let scenario = parse_scenario(scenario_arg)?;
            print_json(&seeded_send_inspector_report(scenario))?;
        }
        Some("panel") => {
            let scenario = parse_scenario(scenario_arg)?;
            let record = seeded_record(scenario);
            print_json(&RequestWorkspacePanelProjection::from_record(&record))?;
        }
        Some("support-export") => {
            let export = seeded_support_export(
                "support-export:request-workspace-alpha:001",
                "2026-05-15T00:00:00Z",
            );
            print_json(&export)?;
        }
        Some("plaintext") => {
            let export = seeded_support_export(
                "support-export:request-workspace-alpha:plaintext",
                "2026-05-15T00:00:00Z",
            );
            print!("{}", render_support_export_plaintext(&export));
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_scenario(
    raw: Option<&str>,
) -> Result<RequestWorkspaceSeededScenario, Box<dyn std::error::Error>> {
    let raw = raw.unwrap_or("local_read_only_get");
    match raw {
        "local_read_only_get" => Ok(RequestWorkspaceSeededScenario::LocalReadOnlyGet),
        "remote_mutating_post_stale_schema" => {
            Ok(RequestWorkspaceSeededScenario::RemoteMutatingPostStaleSchema)
        }
        "managed_delete_missing_schema" => {
            Ok(RequestWorkspaceSeededScenario::ManagedDeleteMissingSchema)
        }
        "remote_graphql_no_auth" => Ok(RequestWorkspaceSeededScenario::RemoteGraphqlNoAuth),
        "imported_stale_assertion_export_truth" => {
            Ok(RequestWorkspaceSeededScenario::ImportedStaleAssertionExportTruth)
        }
        other => Err(format!("unknown scenario: {other}").into()),
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
