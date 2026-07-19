//! Headless emitter for the M5 reproduction-packet set.
//!
//! The bin is the headless stdout projection path for the support export checked in at
//! `artifacts/help/m5-reproduction-packet-proof/packet_set.json`, the governance
//! Markdown summary `artifacts/help/m5-reproduction-packet-governance.md`, the
//! matrix CSV `artifacts/help/m5-reproduction-packet-packets.csv`, and the
//! narrowed fixtures under `fixtures/help/reproduction-packets/`. Help/About,
//! support, and community-handoff surfaces read this set so a report can be
//! previewed and redacted before a public/community/support route opens. The
//! guarded `generate_artifacts` module test refreshes every checked projection
//! from the same seed builders in one operation.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- fixture-save-local-offline-draft
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- fixture-tokens-and-approvals-removed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_reproduction_packets -- validate
//! ```

use aureline_shell::m5_reproduction_packets::{
    seeded_m5_reproduction_packet_set, seeded_save_local_offline_draft_packet,
    seeded_tokens_and_approvals_removed_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let set = seeded_m5_reproduction_packet_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("governance") => {
            let set = seeded_m5_reproduction_packet_set();
            set.validate()?;
            print!("{}", set.render_markdown_summary());
        }
        Some("csv") => {
            let set = seeded_m5_reproduction_packet_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("fixture-save-local-offline-draft") => {
            let packet = seeded_save_local_offline_draft_packet();
            packet.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&packet).expect("packet serializes")
            );
        }
        Some("fixture-tokens-and-approvals-removed") => {
            let packet = seeded_tokens_and_approvals_removed_packet();
            packet.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&packet).expect("packet serializes")
            );
        }
        Some("validate") => {
            seeded_m5_reproduction_packet_set().validate()?;
            seeded_save_local_offline_draft_packet().validate()?;
            seeded_tokens_and_approvals_removed_packet().validate()?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
