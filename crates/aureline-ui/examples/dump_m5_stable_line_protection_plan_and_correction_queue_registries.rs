//! Headless emitter for the M5 line-protection_plan and line-correction-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-stable-line-protection-plan-and-correction-queue-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-stable-line-protection-plan-and-correction-queue-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- report
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- protection-plan-table
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- fixture-protection-plan-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- fixture-correction-queue-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_stable_line_protection_plan_and_correction_queue_registries -- validate
//! ```

use aureline_ui::m5_stable_line_protection_plan_and_correction_queue_registries::{
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries,
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries_correction_queue_preview_narrowed,
    seeded_m5_stable_line_protection_plan_and_correction_queue_registries_protection_plan_beta_narrowed,
    M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
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
            let packet = seeded_m5_stable_line_protection_plan_and_correction_queue_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries()
                    .render_matrix_csv()
            );
        }
        Some("protection-plan-table") => {
            print!(
                "{}",
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries()
                    .render_protection_plan_table()
            );
        }
        Some("fixture-protection-plan-beta-narrowed") => {
            let packet =
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries_protection_plan_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-correction-queue-preview-narrowed") => {
            let packet =
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries_correction_queue_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries(),
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries_protection_plan_beta_narrowed(),
                seeded_m5_stable_line_protection_plan_and_correction_queue_registries_correction_queue_preview_narrowed(),
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
    packet: &M5StableLineProtectionPlanCorrectionQueueRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
