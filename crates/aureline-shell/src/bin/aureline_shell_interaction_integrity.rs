//! Headless inspector for the interaction-integrity beta packet.
//!
//! The bin emits the same packet consumed by the fixture test and support
//! evidence. It is the mint-from-truth path for
//! `fixtures/shell/m3/interaction_integrity/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- state-model
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- batch-reviews
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- identity-cues
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- focus-return
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- vocabulary
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- replay-fixtures
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- validate
//! ```

use aureline_shell::interaction_integrity::{
    seeded_interaction_integrity_beta_packet, validate_interaction_integrity_beta_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_interaction_integrity_beta_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => {
            print_json(&packet)?;
        }
        Some("state-model") => {
            print_json(&packet.object_states)?;
        }
        Some("batch-reviews") => {
            print_json(&packet.batch_scope_reviews)?;
        }
        Some("identity-cues") => {
            print_json(&packet.identity_cues)?;
        }
        Some("focus-return") => {
            print_json(&packet.focus_return_rules)?;
        }
        Some("vocabulary") => {
            print_json(&packet.vocabulary_parity)?;
        }
        Some("support-export") => {
            print_json(&packet.support_export)?;
        }
        Some("replay-fixtures") => {
            print_json(&packet.replay_fixtures)?;
        }
        Some("validate") => match validate_interaction_integrity_beta_packet(&packet) {
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
