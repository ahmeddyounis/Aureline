//! Headless emitter for the M5 setting-write-intent and policy-constraint registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/config/m5-setting-write-intent-and-policy-constraint-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- report
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- write-intent-table
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- fixture-write-intent-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- fixture-policy-constraint-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_write_intent_and_policy_constraint_registries -- validate
//! ```

use aureline_ui::m5_setting_write_intent_and_policy_constraint_registries::{
    seeded_m5_setting_write_intent_and_policy_constraint_registries,
    seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed,
    seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed,
    M5SettingWriteIntentPolicyConstraintRegistriesPacket,
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
            let packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_setting_write_intent_and_policy_constraint_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_setting_write_intent_and_policy_constraint_registries()
                    .render_matrix_csv()
            );
        }
        Some("write-intent-table") => {
            print!(
                "{}",
                seeded_m5_setting_write_intent_and_policy_constraint_registries()
                    .render_write_intent_table()
            );
        }
        Some("fixture-write-intent-beta-narrowed") => {
            let packet =
                seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-policy-constraint-preview-narrowed") => {
            let packet =
                seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_setting_write_intent_and_policy_constraint_registries(),
                seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed(),
                seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed(),
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
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
