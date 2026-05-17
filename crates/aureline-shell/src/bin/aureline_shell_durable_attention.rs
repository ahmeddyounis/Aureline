//! Headless inspector for the durable-attention beta conformance packet.
//!
//! The bin emits the same packet consumed by the fixture test and the
//! support-review artifacts. It is the mint-from-truth path for
//! `fixtures/ux/m3/activity_center_corpus/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- state-machine
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- cases
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- badge-audit
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- quiet-hours-audit
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- reopen-proofs
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- support-export-lineage
//! cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- validate
//! ```

use aureline_shell::durable_attention_beta::{
    seeded_durable_attention_beta_packet, validate_durable_attention_beta_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_durable_attention_beta_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => {
            print_json(&packet)?;
        }
        Some("state-machine") => {
            print_json(&packet.state_machine)?;
        }
        Some("cases") => {
            print_json(&packet.cases)?;
        }
        Some("badge-audit") => {
            print_json(&packet.badge_audit)?;
        }
        Some("quiet-hours-audit") => {
            print_json(&packet.quiet_hours_audit)?;
        }
        Some("reopen-proofs") => {
            print_json(&packet.exact_reopen_proofs)?;
        }
        Some("support-export-lineage") => {
            print_json(&packet.support_export_lineage)?;
        }
        Some("validate") => match validate_durable_attention_beta_packet(&packet) {
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
