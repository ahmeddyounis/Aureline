//! Headless inspector for the deployment-profile continuity corpus.
//!
//! Emits the same corpus packet consumed by the corpus fixture test and
//! the release-evidence excerpt. It is the mint-from-truth path for
//! `fixtures/deployment/m3/profile_truth/`,
//! `fixtures/deployment/m3/control_plane_vs_data_plane/`,
//! `artifacts/release/m3/deployment_profile_conformance_report.md`, and
//! `artifacts/release/m3/residual_dependency_matrix.json`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- drills
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- matrix-json
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- case <case_id>
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- drill <drill_id>
//! cargo run -q -p aureline-shell --bin aureline_shell_deployment_profile_corpus -- validate
//! ```

use aureline_shell::deployment_profile::corpus::{
    render_deployment_profile_conformance_report_markdown,
    render_residual_dependency_matrix_json, seeded_deployment_profile_corpus_packet,
    validate_deployment_profile_corpus_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_deployment_profile_corpus_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("cases") => print_json(&packet.corpus_cases)?,
        Some("drills") => print_json(&packet.outage_drills)?,
        Some("matrix-json") => print!("{}", render_residual_dependency_matrix_json(&packet)),
        Some("report-md") => print!(
            "{}",
            render_deployment_profile_conformance_report_markdown(&packet)
        ),
        Some("case") => {
            let id = args.get(1).ok_or("case <case_id> requires an id argument")?;
            let case = packet
                .corpus_cases
                .iter()
                .find(|c| c.case_id == *id)
                .ok_or_else(|| format!("no corpus case with id {id}"))?;
            print_json(case)?;
        }
        Some("drill") => {
            let id = args
                .get(1)
                .ok_or("drill <drill_id> requires an id argument")?;
            let drill = packet
                .outage_drills
                .iter()
                .find(|d| d.drill_id == *id)
                .ok_or_else(|| format!("no outage drill with id {id}"))?;
            print_json(drill)?;
        }
        Some("validate") => match validate_deployment_profile_corpus_packet(&packet) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("{err}");
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
