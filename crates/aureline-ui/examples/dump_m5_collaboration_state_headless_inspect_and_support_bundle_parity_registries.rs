//! Headless emitter for the M5 line-review_pack_record and line-downgrade-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-collaboration-state-headless-inspect-and-support-bundle-parity-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/collaboration/m5-collaboration-state-headless-inspect-and-support-bundle-parity-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- report
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- review-pack-record-table
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- fixture-review-pack-record-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- fixture-review-pack-result-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries -- validate
//! ```

use aureline_ui::m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries::{
    seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries,
    seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_record_beta_narrowed,
    seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_result_preview_narrowed,
    M5CollaborationStateHeadlessInspectParityRecordAndResultRegistriesPacket,
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
            let packet =
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries().render_matrix_csv()
            );
        }
        Some("review-pack-record-table") => {
            print!(
                "{}",
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries()
                    .render_review_pack_record_table()
            );
        }
        Some("fixture-review-pack-record-beta-narrowed") => {
            let packet =
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_record_beta_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-pack-result-preview-narrowed") => {
            let packet =
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_result_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries(),
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_record_beta_narrowed(),
                seeded_m5_collaboration_state_headless_inspect_and_support_bundle_parity_registries_review_pack_result_preview_narrowed(),
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
    packet: &M5CollaborationStateHeadlessInspectParityRecordAndResultRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
