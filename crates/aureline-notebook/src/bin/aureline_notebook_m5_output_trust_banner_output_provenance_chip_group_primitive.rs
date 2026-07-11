//! Headless emitter for the M5 output-trust-banner / output-provenance-chip-group controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-output-trust-banner-output-provenance-chip-group-proof/`, its matrix CSV,
//! the Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-output-trust-banner-output-provenance-chip-group-controls/`. The notebook,
//! output-viewer, AI-context, review, and support surfaces read these components so one output trust
//! banner names each output's trust class (plain text, sanitized rich, trusted local active, or
//! isolated remote active content), its raw-versus-rendered representation, and whether it is live
//! or stale — and offers open-raw / export / copy — and one output provenance chip group names an
//! output's cell / run identity, origin class, attached artifacts, and persistence / retention cues
//! — and offers inspect / view-artifacts / copy — so a stale output never reads as live and copy /
//! export never flattens the output into ambiguous evidence.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- fixture-output-trust-banner-stale
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- fixture-output-provenance-chip-group-drifted
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_output_trust_banner_output_provenance_chip_group_primitive -- validate
//! ```

use aureline_notebook::implement_output_trust_banners_and_output_provenance_chip_groups_with_plaintext_sanitizedrich_trustedlocalactive_isolatedremoteactive_class_stale_output_honesty_and_copy_export_choice_across_claimed_m5_notebook_outputs::{
    seeded_output_trust_banner_output_provenance_chip_group_controls,
    seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted,
    seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale,
    OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
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
            let packet = seeded_output_trust_banner_output_provenance_chip_group_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_output_trust_banner_output_provenance_chip_group_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_output_trust_banner_output_provenance_chip_group_controls()
                    .render_matrix_csv()
            );
        }
        Some("fixture-output-trust-banner-stale") => {
            let packet =
                seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-output-provenance-chip-group-drifted") => {
            let packet =
                seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_output_trust_banner_output_provenance_chip_group_controls(),
                seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale(),
                seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted(),
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
    packet: &OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "output trust banner output provenance chip group primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
