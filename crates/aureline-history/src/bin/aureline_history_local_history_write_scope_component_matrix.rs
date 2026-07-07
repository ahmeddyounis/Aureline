//! Headless emitter for the frozen M5 local-history / write-scope component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-local-history-write-scope-component-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-local-history-write-scope-components/`. Editor local-history,
//! checkpoint-inspector, restore-review, refactor-preview, AI-apply-review,
//! recovery-center, and support-desk surfaces read this matrix so one local-history
//! row names its snapshot origin, actor, and capture fidelity, one checkpoint-group
//! card names its lineage and mutation class, one restore-preview card names its
//! granularity and drift, one retention/export card names its retention and
//! redaction, one write-scope preview tree names its scope and managed-file caveat,
//! one restore-granularity selector names its selectable apply scope, and one
//! history-export manifest names its class and redaction.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- support-export
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- report
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- csv
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- fixture-write-scope-preview-tree-beta-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- fixture-history-export-manifest-preview-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- validate
//! ```

use aureline_history::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    seeded_m5_local_history_write_scope_component_matrix,
    seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed,
    seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed,
    M5LocalHistoryWriteScopeComponentMatrixPacket,
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
            let packet = seeded_m5_local_history_write_scope_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_local_history_write_scope_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_local_history_write_scope_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-write-scope-preview-tree-beta-narrowed") => {
            let packet =
                seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-history-export-manifest-preview-narrowed") => {
            let packet =
                seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_local_history_write_scope_component_matrix(),
                seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed(),
                seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed(),
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
    packet: &M5LocalHistoryWriteScopeComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
