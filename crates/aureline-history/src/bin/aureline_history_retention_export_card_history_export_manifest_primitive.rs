//! Headless emitter for the M5 retention/export-card & history-export-manifest primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-retention-export-card-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under
//! `fixtures/ui/m5-retention-export-card-primitive/`. Local-history, refactor, import/migration,
//! AI-apply, recovery, and support flows all read this primitive so one retention/export card
//! names its retention posture, redaction posture, cross-baseline comparisons, and survival
//! truth, and one history-export manifest names its class, disposition, primary baseline, and
//! lineage — never a bare download.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- support-export
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- report
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- csv
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- fixture-import-migration-session-preview-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- fixture-ai-apply-evidence-beta-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_retention_export_card_history_export_manifest_primitive -- validate
//! ```

use aureline_history::ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths::{
    seeded_m5_compare_export_ai_apply_evidence_beta_narrowed,
    seeded_m5_compare_export_import_migration_session_preview_narrowed,
    seeded_m5_compare_export_packet, M5CompareExportPacket,
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
            let packet = seeded_m5_compare_export_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_compare_export_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_compare_export_packet().render_matrix_csv());
        }
        Some("fixture-import-migration-session-preview-narrowed") => {
            let packet = seeded_m5_compare_export_import_migration_session_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-apply-evidence-beta-narrowed") => {
            let packet = seeded_m5_compare_export_ai_apply_evidence_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_compare_export_packet(),
                seeded_m5_compare_export_import_migration_session_preview_narrowed(),
                seeded_m5_compare_export_ai_apply_evidence_beta_narrowed(),
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

fn assert_valid(packet: &M5CompareExportPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
