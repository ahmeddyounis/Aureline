//! Headless emitter for the M5 marketplace/account boundary-card and open-in-browser handoff-row
//! controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- report
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- fixture-marketplace-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- fixture-account-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_marketplace_handoff_controls -- validate
//! ```

use aureline_shell::implement_the_m5_marketplace_account_boundary_card_and_open_in_browser_handoff_row_origin_account_scope_profile_region_tenant_network_state_browser_fallback_and_local_safe_continuity_primitive::{
    seeded_m5_marketplace_handoff_controls,
    seeded_m5_marketplace_handoff_controls_account_preview_narrowed,
    seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed,
    M5MarketplaceHandoffControlsPacket,
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
            let packet = seeded_m5_marketplace_handoff_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_marketplace_handoff_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_marketplace_handoff_controls().render_matrix_csv()
            );
        }
        Some("fixture-marketplace-beta-narrowed") => {
            let packet = seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-account-preview-narrowed") => {
            let packet = seeded_m5_marketplace_handoff_controls_account_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_marketplace_handoff_controls(),
                seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed(),
                seeded_m5_marketplace_handoff_controls_account_preview_narrowed(),
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

fn assert_valid(packet: &M5MarketplaceHandoffControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
