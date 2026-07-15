//! Headless emitter for the M5 line-refresh_policy and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-refresh-policy-and-claim-downgrade-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-refresh-policy-and-claim-downgrade-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- refresh-policy-table
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- fixture-refresh-policy-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- fixture-claim-downgrade-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_refresh_policy_and_claim_downgrade_registries -- validate
//! ```

use aureline_ui::m5_stable_line_refresh_policy_and_claim_downgrade_registries::{
    seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries,
    seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_claim_downgrade_preview_narrowed,
    seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_refresh_policy_beta_narrowed,
    M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket,
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
            let packet = seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries()
                    .render_matrix_csv()
            );
        }
        Some("refresh-policy-table") => {
            print!(
                "{}",
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries()
                    .render_refresh_policy_table()
            );
        }
        Some("fixture-refresh-policy-beta-narrowed") => {
            let packet =
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_refresh_policy_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-claim-downgrade-preview-narrowed") => {
            let packet =
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_claim_downgrade_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries(),
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_refresh_policy_beta_narrowed(),
                seeded_m5_stable_line_refresh_policy_and_claim_downgrade_registries_claim_downgrade_preview_narrowed(),
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

fn assert_valid(
    packet: &M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
