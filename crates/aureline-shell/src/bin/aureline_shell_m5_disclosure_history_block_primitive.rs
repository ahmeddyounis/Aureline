//! Headless emitter for the M5 disclosure / history block primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-disclosure-history-block-proof/`, its matrix CSV, the Markdown
//! report `artifacts/security/m5-disclosure-history-block-primitive.md`, and the narrowed
//! fixtures under `fixtures/security/m5-disclosure-history-block-primitive/`. Every M5
//! surface that inspects an advisory's disclosure details and resolved-state history —
//! Help/About, the update center, and the support bundle — reads this primitive so the
//! current status, the affected versions / components, the copy-safe CVE / GHSA reference
//! ids, the resolved-state downgrade, the provenance, and the open-doc / open-browser
//! actions stay consistent, and so the support export reconstructs the disclosure from one
//! shared block model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- fixture-offline-imported-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- fixture-externally-linked-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_disclosure_and_history_block_primitive::{
    seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed,
    seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed,
    seeded_m5_disclosure_history_block_primitive_packet, M5DisclosureHistoryBlockPacket,
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
            let packet = seeded_m5_disclosure_history_block_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_disclosure_history_block_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_disclosure_history_block_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-offline-imported-beta-narrowed") => {
            let packet =
                seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-externally-linked-preview-narrowed") => {
            let packet =
                seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_disclosure_history_block_primitive_packet(),
                seeded_m5_disclosure_history_block_primitive_offline_imported_beta_narrowed(),
                seeded_m5_disclosure_history_block_primitive_externally_linked_preview_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5DisclosureHistoryBlockPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
