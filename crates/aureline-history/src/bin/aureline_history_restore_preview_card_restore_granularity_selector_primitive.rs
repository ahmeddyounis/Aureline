//! Headless emitter for the M5 restore-preview-card / restore-granularity-selector
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/`,
//! its matrix CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-restore-preview-card-and-restore-granularity-selector-primitive/`.
//! Editor restore, AI-apply restore, import restore, repair restore, and recovery center
//! all read this primitive so one restore-preview card names its past-versus-current
//! comparison, object identity, external-drift baseline, generated/managed-file caveat,
//! restore granularity, and no-history-erasure truth, and one restore-granularity selector
//! names its selectable apply scope, default mode, and dry-run path.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- support-export
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- report
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- csv
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- fixture-import-restore-preview-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- fixture-ai-apply-restore-beta-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_restore_preview_card_restore_granularity_selector_primitive -- validate
//! ```

use aureline_history::implement_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes::{
    seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed,
    seeded_m5_restore_preview_granularity_import_restore_preview_narrowed,
    seeded_m5_restore_preview_granularity_packet, M5RestorePreviewGranularityPacket,
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
            let packet = seeded_m5_restore_preview_granularity_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_restore_preview_granularity_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_restore_preview_granularity_packet().render_matrix_csv()
            );
        }
        Some("fixture-import-restore-preview-narrowed") => {
            let packet = seeded_m5_restore_preview_granularity_import_restore_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-apply-restore-beta-narrowed") => {
            let packet = seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_restore_preview_granularity_packet(),
                seeded_m5_restore_preview_granularity_import_restore_preview_narrowed(),
                seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed(),
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
    packet: &M5RestorePreviewGranularityPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
