//! Headless emitter for the M5 relation-strip / sync-pending-pill controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-relation-strip-sync-pending-proof/`, its matrix
//! CSV, the Markdown summary, and the scenario fixtures under
//! `fixtures/ui/m5-relation-strip-sync-pending-controls/`. Issue-inbox, detail,
//! review, incident, support, and CLI work-item surfaces read these controls so
//! one relation strip names each linked branch/review/test/incident context with a
//! derived stale/broken relation label and metadata-safe copy/open actions, and one
//! sync-pending pill discloses its pending change, last sync attempt, and a
//! retry-or-export recovery path that reads visibly differently from a
//! provider-confirmed state.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- support-export
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- report
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- csv
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- fixture-relation-strip-stale-relation
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- fixture-sync-pending-recoverable-failure
//! cargo run -q -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- validate
//! ```

use aureline_provider::implement_relation_strips_and_sync_pending_pills_with_linked_context_stale_labeling_and_retry_or_export_continuity::{
    seeded_relation_strip_sync_pending_controls,
    seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation,
    seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure,
    RelationStripSyncPendingControlsPacket,
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
            let packet = seeded_relation_strip_sync_pending_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_relation_strip_sync_pending_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_relation_strip_sync_pending_controls().render_matrix_csv()
            );
        }
        Some("fixture-relation-strip-stale-relation") => {
            let packet =
                seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sync-pending-recoverable-failure") => {
            let packet =
                seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_relation_strip_sync_pending_controls(),
                seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation(),
                seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure(),
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
    packet: &RelationStripSyncPendingControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "relation strip sync pending controls failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
