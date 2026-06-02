//! Headless inspector for the collection-truth corpus packet.
//!
//! Emits the same corpus packet consumed by the corpus fixture test
//! and the QE evidence report. It is the mint-from-truth path for
//! `fixtures/ux/m3/collection_truth_corpus/`,
//! `artifacts/qe/m3/collection_truth_report.md`,
//! `artifacts/qe/m3/collection_truth_matrix.json`, and
//! `docs/qe/m3/collection_truth_drills.md`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- matrix-json
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills-md
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- migrations
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- validate
//! ```

use aureline_shell::collection_truth_corpus::{
    render_collection_truth_corpus_drills_markdown, render_collection_truth_corpus_report_markdown,
    seeded_collection_truth_corpus_packet, validate_collection_truth_corpus_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_collection_truth_corpus_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("matrix-json") => print_json(&packet.matrix)?,
        Some("report-md") => print!(
            "{}",
            render_collection_truth_corpus_report_markdown(&packet)
        ),
        Some("drills-md") => print!(
            "{}",
            render_collection_truth_corpus_drills_markdown(&packet)
        ),
        Some("migrations") => print_json(&packet.saved_view_migrations)?,
        Some("drills") => print_json(&packet.accessibility_drills)?,
        Some("cases") => print_json(&packet.corpus_cases)?,
        Some("validate") => match validate_collection_truth_corpus_packet(&packet) {
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
