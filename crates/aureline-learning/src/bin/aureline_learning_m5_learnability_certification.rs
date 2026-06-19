//! Headless emitter and validator for the M5 learnability certification packet.
//!
//! Builds the seeded certification packet, validates it against the learnability
//! certification invariants, and prints the canonical support export, the Markdown
//! summary, or the waiver-and-downgrade log so the checked-in artifacts stay
//! byte-aligned with the in-crate builder. It can also refresh an on-disk fixture.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the canonical support export as JSON (default).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_certification
//!
//! # Print the deterministic Markdown summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_certification -- summary
//!
//! # Print the release-visible waiver-and-downgrade log.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_certification -- waiver
//!
//! # Validate the seeded packet (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_certification -- validate
//!
//! # Emit the on-disk fixture / artifact JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learnability_certification \
//!   -- emit-fixture fixtures/help/m5/certification-corpus/learnability_certification_corpus.json
//! ```

use std::path::PathBuf;

use aureline_learning::seeded_m5_learnability_certification;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_m5_learnability_certification();

    match args.first().map(String::as_str) {
        None | Some("support") => {
            println!("{}", packet.export_safe_json());
            Ok(())
        }
        Some("summary") => {
            print!("{}", packet.render_markdown_summary());
            Ok(())
        }
        Some("waiver") => {
            print!("{}", packet.render_waiver_and_downgrade_log());
            Ok(())
        }
        Some("validate") => {
            let violations = packet.validate();
            if violations.is_empty() {
                println!("ok — all learnability certification invariants pass");
                Ok(())
            } else {
                for violation in &violations {
                    eprintln!("FAIL {}", violation.as_str());
                }
                Err(format!("{} validation error(s)", violations.len()).into())
            }
        }
        Some("emit-fixture") => {
            let path: PathBuf = args
                .get(1)
                .ok_or("emit-fixture requires a target path argument")?
                .into();
            let json = packet.export_safe_json();
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, format!("{json}\n"))?;
            println!("wrote {}", path.display());
            Ok(())
        }
        Some(unknown) => Err(format!("unknown subcommand: {unknown}").into()),
    }
}
