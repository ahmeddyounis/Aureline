//! Headless inspector and regeneration path for the voice/dictation
//! conformance corpus.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- manifest
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- write-corpus
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- privacy-report
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- equivalence-audit
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- qualification
//! cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- run
//! ```

use std::path::PathBuf;

use aureline_shell::voice::conformance::{
    corpus_dir_from_repo_root, render_command_equivalence_audit, render_privacy_and_parity_report,
    run_corpus, seeded_voice_conformance_corpus, seeded_voice_qualification_packet, write_corpus,
    DrillOutcome,
};
use aureline_shell::voice::seeded_voice_preview_beta_page;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("manifest") | None => {
            let corpus = seeded_voice_conformance_corpus();
            println!("{}", serde_json::to_string_pretty(&corpus.manifest)?);
        }
        Some("write-corpus") => {
            let dir = args
                .get(1)
                .map(PathBuf::from)
                .unwrap_or_else(|| corpus_dir_from_repo_root(&repo_root()));
            let corpus = seeded_voice_conformance_corpus();
            write_corpus(&dir, &corpus)?;
            eprintln!(
                "wrote {} positive + {} negative fixtures to {}",
                corpus.positives.len(),
                corpus.negatives.len(),
                dir.display()
            );
        }
        Some("privacy-report") => {
            let page = seeded_voice_preview_beta_page();
            let packet = seeded_voice_qualification_packet(&page);
            print!("{}", render_privacy_and_parity_report(&page, &packet));
        }
        Some("equivalence-audit") => {
            let page = seeded_voice_preview_beta_page();
            print!("{}", render_command_equivalence_audit(&page));
        }
        Some("qualification") => {
            let page = seeded_voice_preview_beta_page();
            let packet = seeded_voice_qualification_packet(&page);
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        Some("run") => {
            let report = run_corpus(&corpus_dir_from_repo_root(&repo_root()));
            for drill in &report.drills {
                let status = match &drill.outcome {
                    DrillOutcome::Pass => "pass".to_owned(),
                    DrillOutcome::Fail(reason) => format!("FAIL {reason:?}"),
                };
                println!(
                    "{} [{}] {}",
                    if drill.positive {
                        "positive"
                    } else {
                        "negative"
                    },
                    status,
                    drill.drill_id
                );
            }
            if report.all_passed() {
                eprintln!(
                    "voice_conformance: ok ({} positive, {} negative)",
                    report.positive_count(),
                    report.negative_count()
                );
            } else {
                eprintln!("voice_conformance: {} failure(s)", report.failures().len());
                std::process::exit(1);
            }
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
