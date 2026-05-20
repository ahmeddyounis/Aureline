//! Headless inspector for the beta attention-routing corpus.
//!
//! The bin emits the same governed route outcomes consumed by the live shell,
//! by the support-export wrapper, and by the integration test that replays the
//! checked-in fixtures under `fixtures/ux/m3/notification_routing/`. It is the
//! only mint-from-truth path for those fixtures, so the JSON cannot drift from
//! the Rust types.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- corpus
//! cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- validate
//! ```

use aureline_shell::attention_router::{
    seeded_attention_routing_corpus, validate_attention_routing_corpus, AttentionRouteSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus = seeded_attention_routing_corpus();

    match args.first().map(String::as_str) {
        Some("corpus") | None => {
            print_json(&corpus)?;
        }
        Some("cases") => {
            print_json(&corpus.cases)?;
        }
        Some("summary") => {
            print_json(&corpus.summary)?;
        }
        Some("support-export") => {
            let export = AttentionRouteSupportExport::from_corpus(
                "support-export:attention-router-beta:001",
                "2026-05-20T00:00:00Z",
                &corpus,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_attention_routing_corpus(&corpus) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("corpus error: {err}");
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
