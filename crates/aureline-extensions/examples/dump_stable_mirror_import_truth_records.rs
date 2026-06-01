//! Dump stable mirror/manual import-truth records for every canonical fixture.
//!
//! Used by the checked fixture, support-export, and docs/schema validation lanes so a
//! materialized packet can be validated against
//! `schemas/extensions/stable_mirror_import_truth.schema.json` with an independent
//! Draft 2020-12 validator:
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_stable_mirror_import_truth_records -- packets
//! cargo run -q -p aureline-extensions --example dump_stable_mirror_import_truth_records -- support-exports
//! cargo run -q -p aureline-extensions --example dump_stable_mirror_import_truth_records -- validate
//! ```

use aureline_extensions::{
    project_stable_mirror_import_truth_support_export, StableMirrorImportTruthInput,
    StableMirrorImportTruthPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableMirrorImportTruthInput,
}

const FIXTURES: &[&str] = &[
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/verified_publisher_offline_bundle_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/approved_mirror_promotion_settled_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/signer_key_rotation_in_cooldown_narrows_to_beta.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/ownership_transfer_pending_notification_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/namespace_dispute_withdrawn.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/orphan_succession_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/manual_artifact_shimmed_narrows_to_beta.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/unsupported_mapping_failed_withdrawn.json"),
];

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("packets");

    let mut packets = Vec::new();
    for raw in FIXTURES {
        let fixture: PacketFixture = serde_json::from_str(raw)?;
        let packet = StableMirrorImportTruthPacket::from_input(fixture.packet_input.clone())
            .map_err(|e| format!("fixture {} must build: {e}", fixture.case_name))?;
        packets.push(packet);
    }

    match mode {
        "packets" => print_json(&packets)?,
        "support-exports" => {
            let exports: Vec<_> = packets
                .iter()
                .map(project_stable_mirror_import_truth_support_export)
                .collect();
            print_json(&exports)?;
        }
        "validate" => {
            for packet in &packets {
                packet.validate()?;
            }
            println!("ok ({} packets)", packets.len());
        }
        other => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
