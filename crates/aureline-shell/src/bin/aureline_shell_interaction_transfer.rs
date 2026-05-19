//! Headless inspector for the interaction-transfer beta packet.
//!
//! The bin emits the same packet consumed by the fixture test and support
//! evidence. It is the mint-from-truth path for
//! `fixtures/ux/m3/clipboard_dragdrop_history/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- clipboard-payloads
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- drop-intents
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- undo-groups
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- back-forward
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- reopen-history
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- validate
//! ```

use aureline_shell::interaction_transfer::{
    seeded_interaction_transfer_beta_packet, validate_interaction_transfer_beta_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_interaction_transfer_beta_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("clipboard-payloads") => print_json(&packet.clipboard_payload_classes)?,
        Some("drop-intents") => print_json(&packet.drop_intents)?,
        Some("undo-groups") => print_json(&packet.undo_groups)?,
        Some("back-forward") => print_json(&packet.back_forward_entries)?,
        Some("reopen-history") => print_json(&packet.reopen_history_entries)?,
        Some("support-export") => print_json(&packet.support_export)?,
        Some("summary") => print_json(&packet.summary)?,
        Some("validate") => match validate_interaction_transfer_beta_packet(&packet) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("{err}");
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
