//! Headless emitter for the M5 focus-and-selection contract.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-focus-return-proof/` and the narrowed fixtures under
//! `fixtures/a11y/m5-focus-return/`. Shell, search/palette, review, data-grid,
//! notification, and presentation surfaces consume this contract so each governed
//! focus zone — modal dialogs, sheets, palettes, popovers, rename fields, inspector
//! promotions, dense collections, streamed lists, shell zones, multi-window layouts,
//! and follow/presentation modes — returns focus predictably to a real owner,
//! preserves focus and selection by stable item identity across virtualization,
//! refresh, streaming inserts, filtering, sort changes, and multi-window restore, and
//! uses a roving single tab stop for dense collections, rather than per-surface ad hoc
//! focus handling.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- fixture-proof-stale-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- fixture-bridge-unavailable-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_focus_return -- validate
//! ```

use aureline_shell::focus::{
    seeded_m5_focus_selection_contract,
    seeded_m5_focus_selection_contract_bridge_unavailable_narrowed,
    seeded_m5_focus_selection_contract_proof_stale_narrowed, M5FocusSelectionContractPacket,
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
            let packet = seeded_m5_focus_selection_contract();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_focus_selection_contract().render_markdown_summary()
            );
        }
        Some("fixture-proof-stale-narrowed") => {
            let packet = seeded_m5_focus_selection_contract_proof_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-bridge-unavailable-narrowed") => {
            let packet = seeded_m5_focus_selection_contract_bridge_unavailable_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_focus_selection_contract(),
                seeded_m5_focus_selection_contract_proof_stale_narrowed(),
                seeded_m5_focus_selection_contract_bridge_unavailable_narrowed(),
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

fn assert_valid(packet: &M5FocusSelectionContractPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "focus selection contract failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
