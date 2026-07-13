//! Headless emitter for the M5 system-appearance live-apply and appearance-source-provenance packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/`, its matrix
//! CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/platform/m5-system-appearance-live-apply-and-source-provenance-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- report
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- posture-table
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- fixture-docs-help-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- fixture-restart-posture-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- validate
//! ```

use aureline_ui::m5_system_appearance_live_apply_and_source_provenance_registries::{
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries,
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed,
    seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed,
    M5SystemAppearanceRegistriesPacket,
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
            let packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries()
                    .render_matrix_csv()
            );
        }
        Some("posture-table") => {
            print!(
                "{}",
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries()
                    .render_appearance_posture_table()
            );
        }
        Some("fixture-docs-help-beta-narrowed") => {
            let packet =
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-restart-posture-preview-narrowed") => {
            let packet =
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries(),
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed(),
                seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed(),
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
    packet: &M5SystemAppearanceRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
