//! Headless emitter for the M5 dataset-provenance-card / sensitivity-sharing-banner controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/`, its matrix
//! CSV, the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls/`. The dataset
//! catalog and the share-review surfaces read these components so one dataset card names its
//! dataset / table, source class, version / snapshot / partition, row / file count or estimate,
//! sample / truncation state, sensitivity / redaction state, and local-versus-remote location —
//! and offers open / inspect-provenance / export-metadata — and one sharing banner names its
//! share class, blocked destinations, metadata-only-versus-raw-payload choice, copy / export
//! policy, and local-safe alternatives, and offers review / share-metadata-only paths, so a
//! remote or unknown-location dataset never reads as a local dataset and raw data is never
//! implied by default before a preview, compare, or share.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- fixture-dataset-card-remote
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- fixture-sharing-banner-raw-payload
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- validate
//! ```

use aureline_notebook::implement_dataset_provenance_cards_and_sensitivity_sharing_banners_with_snapshot_sample_redaction_and_local_remote_location_truth_across_claimed_m5_data_lanes::{
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls,
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote,
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload,
    DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
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
            let packet = seeded_dataset_provenance_card_sensitivity_sharing_banner_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls()
                    .render_matrix_csv()
            );
        }
        Some("fixture-dataset-card-remote") => {
            let packet =
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sharing-banner-raw-payload") => {
            let packet =
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls(),
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote(),
                seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload(),
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
    packet: &DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "dataset provenance sharing banner primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
