//! Headless emitter for the frozen M5 settings-row, capability-sheet,
//! evidence-chronology, and chronology-export component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-trust-chronology-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-trust-chronology-components.md`, and
//! the narrowed fixtures under `fixtures/ui/m5-trust-chronology-components/`.
//! Settings surfaces, capability sheets, activity/evidence timelines, and
//! chronology export previews read this matrix so one settings-row model carries
//! effective-versus-configured truth, one capability-sheet model groups by
//! consequence and preserves re-consent, one evidence/chronology model uses
//! stable verbs and provenance badges, and every export stays reconstructable.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- fixture-narrative-summary-card-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- fixture-chronology-export-preview-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_chronology_components -- validate
//! ```

use aureline_shell::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    seeded_m5_trust_chronology_component_matrix,
    seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed,
    seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed,
    M5TrustComponentMatrixPacket,
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
            let packet = seeded_m5_trust_chronology_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_trust_chronology_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_trust_chronology_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-narrative-summary-card-beta-narrowed") => {
            let packet =
                seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-chronology-export-preview-preview-narrowed") => {
            let packet =
                seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_trust_chronology_component_matrix(),
                seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed(),
                seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed(),
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

fn assert_valid(packet: &M5TrustComponentMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
