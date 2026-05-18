//! Headless inspector for the activity-timeline + attention-inbox packet.
//!
//! The bin emits the same packet consumed by the fixture test and by
//! reviewers comparing the live build to the checked-in corpus.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- snapshot
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- groups
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- summary-cards
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- inbox
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- validate
//! ```

use aureline_shell::activity_timeline::{
    seeded_activity_timeline_and_inbox_packet, validate_activity_timeline_and_inbox_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_activity_timeline_and_inbox_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("snapshot") => print_json(&packet.snapshot)?,
        Some("rows") => print_json(&packet.snapshot.rows)?,
        Some("groups") => print_json(&packet.snapshot.groups)?,
        Some("summary-cards") => print_json(&packet.snapshot.summary_cards)?,
        Some("inbox") => print_json(&packet.snapshot.inbox)?,
        Some("summary") => print_json(&packet.summary)?,
        Some("validate") => match validate_activity_timeline_and_inbox_packet(&packet) {
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
