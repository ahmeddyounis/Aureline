//! Headless emitter for the M5 local-history / write-scope component-consumer lane.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-local-history-write-scope-component-consumer-proof/`, its
//! matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/ui/m5-local-history-write-scope-component-consumers/`. The editor rename /
//! refactor transaction, the replace-in-files apply, the import / migration session, the
//! repair transaction, the generated-artifact provenance surface, the AI apply / review
//! surface, and the support / export desk all read this adoption lane so one shared set
//! of local-history and write-scope components keeps checkpoint, rollback, restore, and
//! export language aligned — never a parallel local variant.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- support-export
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- report
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- csv
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- fixture-import-migration-preview-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- fixture-ai-review-beta-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_consumers -- validate
//! ```

use aureline_history::add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces::{
    seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed,
    seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed,
    seeded_m5_local_history_write_scope_component_consumer_packet, M5HistoryComponentConsumerPacket,
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
            let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_local_history_write_scope_component_consumer_packet()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_local_history_write_scope_component_consumer_packet().render_matrix_csv()
            );
        }
        Some("fixture-import-migration-preview-narrowed") => {
            let packet =
                seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-review-beta-narrowed") => {
            let packet = seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_local_history_write_scope_component_consumer_packet(),
                seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed(),
                seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed(),
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

fn assert_valid(packet: &M5HistoryComponentConsumerPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("consumer lane failed validation: {}", tokens.join(",")).into())
    }
}
