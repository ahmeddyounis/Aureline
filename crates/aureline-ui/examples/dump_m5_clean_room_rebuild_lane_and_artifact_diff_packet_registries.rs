//! Headless emitter for the M5 clean-room-rebuild-lane and artifact-diff-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-clean-room-rebuild-lane-and-artifact-diff-packet-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-clean-room-rebuild-lane-and-artifact-diff-packet-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- report
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- clean-room-rebuild-lane-table
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- fixture-hermetic-rebuild-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- fixture-artifact-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- validate
//! ```

use aureline_ui::m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries::{
    seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries,
    seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_artifact_diff_preview_narrowed,
    seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_hermetic_rebuild_beta_narrowed,
    M5CleanRoomRebuildArtifactDiffRegistriesPacket,
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
            let packet = seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries()
                    .render_matrix_csv()
            );
        }
        Some("clean-room-rebuild-lane-table") => {
            print!(
                "{}",
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries()
                    .render_clean_room_rebuild_lane_table()
            );
        }
        Some("fixture-hermetic-rebuild-beta-narrowed") => {
            let packet =
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_hermetic_rebuild_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-artifact-diff-preview-narrowed") => {
            let packet =
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_artifact_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries(),
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_hermetic_rebuild_beta_narrowed(),
                seeded_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries_artifact_diff_preview_narrowed(),
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
    packet: &M5CleanRoomRebuildArtifactDiffRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
