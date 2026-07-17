//! Headless emitter for the M5 line-required_evidence_check and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/review/m5-required-evidence-check-and-local-ci-parity-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/review/m5-required-evidence-check-and-local-ci-parity-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- report
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- required-evidence-check-table
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- fixture-required-evidence-check-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- fixture-local-ci-parity-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_required_evidence_check_and_local_ci_parity_registries -- validate
//! ```

use aureline_ui::m5_required_evidence_check_and_local_ci_parity_registries::{
    seeded_m5_required_evidence_check_and_local_ci_parity_registries,
    seeded_m5_required_evidence_check_and_local_ci_parity_registries_local_ci_parity_preview_narrowed,
    seeded_m5_required_evidence_check_and_local_ci_parity_registries_required_evidence_check_beta_narrowed,
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
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
            let packet = seeded_m5_required_evidence_check_and_local_ci_parity_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_required_evidence_check_and_local_ci_parity_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_required_evidence_check_and_local_ci_parity_registries()
                    .render_matrix_csv()
            );
        }
        Some("required-evidence-check-table") => {
            print!(
                "{}",
                seeded_m5_required_evidence_check_and_local_ci_parity_registries()
                    .render_required_evidence_check_table()
            );
        }
        Some("fixture-required-evidence-check-beta-narrowed") => {
            let packet =
                seeded_m5_required_evidence_check_and_local_ci_parity_registries_required_evidence_check_beta_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-local-ci-parity-preview-narrowed") => {
            let packet =
                seeded_m5_required_evidence_check_and_local_ci_parity_registries_local_ci_parity_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_required_evidence_check_and_local_ci_parity_registries(),
                seeded_m5_required_evidence_check_and_local_ci_parity_registries_required_evidence_check_beta_narrowed(),
                seeded_m5_required_evidence_check_and_local_ci_parity_registries_local_ci_parity_preview_narrowed(),
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
    packet: &M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
