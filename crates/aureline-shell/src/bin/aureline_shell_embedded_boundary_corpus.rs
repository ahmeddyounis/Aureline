//! Headless inspector for the embedded-surface boundary audit corpus.
//!
//! Emits the same corpus packet consumed by the corpus fixture replay
//! test, the audit report artifact, and the beta audit doc. It is the
//! mint-from-truth path for `fixtures/ux/m3/embedded_boundary_corpus/`,
//! `artifacts/ux/m3/embedded_boundary_audit_report.md`, and
//! `docs/ux/m3/embedded_boundary_audit_beta.md`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- matrix-json
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- validate
//! ```

use aureline_shell::embedded_boundary_corpus::{
    render_embedded_boundary_corpus_doc_markdown, render_embedded_boundary_corpus_report_markdown,
    seeded_embedded_boundary_corpus_packet, validate_embedded_boundary_corpus_packet_summary,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_embedded_boundary_corpus_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("cases") => print_json(&packet.corpus_cases)?,
        Some("matrix-json") => print_json(&packet.matrix)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("report-md") => print!(
            "{}",
            render_embedded_boundary_corpus_report_markdown(&packet)
        ),
        Some("doc-md") => print!("{}", render_embedded_boundary_corpus_doc_markdown(&packet)),
        Some("validate") => match validate_embedded_boundary_corpus_packet_summary(&packet) {
            Ok(summary) => println!("{summary}"),
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
