//! Headless emitter for the M5 archived-object expiry / removal state packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-archived-evidence-state/`, its matrix CSV, the Markdown summary, and the narrowed
//! fixtures under `fixtures/recovery/m5-archived-evidence-state/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- support-export
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- report
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- csv
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- fixture-expired-narrowed
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- fixture-removed-narrowed
//! cargo run -p aureline-ui --example dump_m5_archived_object_expiry_removal_state_and_metadata_fallback -- validate
//! ```

use aureline_ui::m5_archived_object_expiry_removal_state_and_metadata_fallback::{
    seeded_m5_archived_evidence_state, seeded_m5_archived_evidence_state_expired_narrowed,
    seeded_m5_archived_evidence_state_removed_narrowed, M5ArchivedEvidenceStatePacket,
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
            let packet = seeded_m5_archived_evidence_state();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_archived_evidence_state().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_archived_evidence_state().render_matrix_csv()
            );
        }
        Some("fixture-expired-narrowed") => {
            let packet = seeded_m5_archived_evidence_state_expired_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-removed-narrowed") => {
            let packet = seeded_m5_archived_evidence_state_removed_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_archived_evidence_state(),
                seeded_m5_archived_evidence_state_expired_narrowed(),
                seeded_m5_archived_evidence_state_removed_narrowed(),
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

fn assert_valid(packet: &M5ArchivedEvidenceStatePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "archived-state packet failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
