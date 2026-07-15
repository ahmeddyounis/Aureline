//! Headless emitter for the M5 setting-definition and effective-setting registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/config/m5-setting-definition-and-effective-setting-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- report
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- setting-definition-table
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- fixture-setting-definition-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- fixture-effective-setting-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_setting_definition_and_effective_setting_registries -- validate
//! ```

use aureline_ui::m5_setting_definition_and_effective_setting_registries::{
    seeded_m5_setting_definition_and_effective_setting_registries,
    seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed,
    seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed,
    M5SettingDefinitionEffectiveSettingRegistriesPacket,
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
            let packet = seeded_m5_setting_definition_and_effective_setting_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_setting_definition_and_effective_setting_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_setting_definition_and_effective_setting_registries().render_matrix_csv()
            );
        }
        Some("setting-definition-table") => {
            print!(
                "{}",
                seeded_m5_setting_definition_and_effective_setting_registries()
                    .render_setting_definition_table()
            );
        }
        Some("fixture-setting-definition-beta-narrowed") => {
            let packet =
                seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-effective-setting-preview-narrowed") => {
            let packet =
                seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_setting_definition_and_effective_setting_registries(),
                seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed(),
                seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed(),
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
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
