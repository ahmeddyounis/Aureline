//! Headless emitter for the M5 docs-pack-row / stale-example-finding-row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/`, its
//! matrix CSV, the Markdown report, and the narrowed fixtures under
//! `fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/`.
//! Docs-pack managers, help pack panels, onboarding pack steps, AI pack contexts, and
//! support pack evidence read these primitives so one pack row keeps a pack's pin /
//! mirror / offline / quarantine / update / stale state distinct and never shows a
//! quarantined or stale pack as trusted, and one stale-example finding row turns "docs
//! may be old" into an actionable, version-anchored finding.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- report
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- csv
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- fixture-onboarding-pack-beta-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- fixture-ai-pack-context-preview-narrowed
//! cargo run -q -p aureline-docs --bin aureline_docs_pack_row_stale_example_finding_primitive -- validate
//! ```

use aureline_docs::implement_docs_pack_rows_and_stale_example_finding_rows_with_pin_offline_refresh_quarantine_update_remove_actions_and_version_drift_truth::{
    seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed,
    seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed,
    seeded_m5_pack_finding_primitive_packet, M5DocsPackFindingPrimitivePacket,
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
            let packet = seeded_m5_pack_finding_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_pack_finding_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_pack_finding_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-onboarding-pack-beta-narrowed") => {
            let packet = seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-pack-context-preview-narrowed") => {
            let packet = seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_pack_finding_primitive_packet(),
                seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed(),
                seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed(),
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
    packet: &M5DocsPackFindingPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
