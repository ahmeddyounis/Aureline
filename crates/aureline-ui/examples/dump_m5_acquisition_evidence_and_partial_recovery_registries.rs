//! Headless emitter for the M5 acquisition-evidence and partial-recovery registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/`, its matrix CSV, the
//! Markdown summary, and the narrowed fixtures under
//! `fixtures/workspaces/m5-acquisition-evidence-and-partial-recovery-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- report
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- partial-recovery-table
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- fixture-resume-partial-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- fixture-discard-cleanup-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- validate
//! ```

use aureline_ui::m5_acquisition_evidence_and_partial_recovery_registries::{
    seeded_m5_acquisition_evidence_and_partial_recovery_registries,
    seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed,
    seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed,
    M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
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
            let packet = seeded_m5_acquisition_evidence_and_partial_recovery_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_acquisition_evidence_and_partial_recovery_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_acquisition_evidence_and_partial_recovery_registries()
                    .render_matrix_csv()
            );
        }
        Some("partial-recovery-table") => {
            print!(
                "{}",
                seeded_m5_acquisition_evidence_and_partial_recovery_registries()
                    .render_partial_recovery_table()
            );
        }
        Some("fixture-resume-partial-beta-narrowed") => {
            let packet =
                seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-discard-cleanup-preview-narrowed") => {
            let packet =
                seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_acquisition_evidence_and_partial_recovery_registries(),
                seeded_m5_acquisition_evidence_and_partial_recovery_registries_resume_partial_beta_narrowed(),
                seeded_m5_acquisition_evidence_and_partial_recovery_registries_discard_cleanup_preview_narrowed(),
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
    packet: &M5AcquisitionEvidencePartialRecoveryRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
