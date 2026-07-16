//! Headless emitter for the M5 imported / offline evidence lineage propagation packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/support/m5-imported-offline-lineage/`, its matrix CSV, the Markdown summary, and the narrowed
//! fixtures under `fixtures/recovery/m5-imported-offline-lineage/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- support-export
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- report
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- csv
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- fixture-imported-offline-narrowed
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- fixture-metadata-only-narrowed
//! cargo run -p aureline-ui --example dump_m5_imported_offline_evidence_lineage_propagation -- validate
//! ```

use aureline_ui::m5_imported_offline_evidence_lineage_propagation::{
    seeded_m5_imported_offline_lineage,
    seeded_m5_imported_offline_lineage_imported_offline_narrowed,
    seeded_m5_imported_offline_lineage_metadata_only_narrowed, M5ImportedOfflineLineagePacket,
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
            let packet = seeded_m5_imported_offline_lineage();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_imported_offline_lineage().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_imported_offline_lineage().render_matrix_csv()
            );
        }
        Some("fixture-imported-offline-narrowed") => {
            let packet = seeded_m5_imported_offline_lineage_imported_offline_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-metadata-only-narrowed") => {
            let packet = seeded_m5_imported_offline_lineage_metadata_only_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_imported_offline_lineage(),
                seeded_m5_imported_offline_lineage_imported_offline_narrowed(),
                seeded_m5_imported_offline_lineage_metadata_only_narrowed(),
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

fn assert_valid(packet: &M5ImportedOfflineLineagePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("lineage packet failed validation: {}", tokens.join(",")).into())
    }
}
