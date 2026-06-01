//! Dump stable SDK author-lane records for every canonical fixture.
//!
//! Used by the checked fixture, support-export, and docs/schema validation lanes
//! so a materialized packet can be validated against
//! `schemas/extensions/stable_sdk_author_lane.schema.json` with an independent
//! Draft 2020-12 validator:
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_stable_sdk_author_lane_records -- packets
//! cargo run -q -p aureline-extensions --example dump_stable_sdk_author_lane_records -- support-exports
//! cargo run -q -p aureline-extensions --example dump_stable_sdk_author_lane_records -- validate
//! ```

use aureline_extensions::{
    project_stable_sdk_author_lane_support_export, StableSdkAuthorLaneInput,
    StableSdkAuthorLanePacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableSdkAuthorLaneInput,
}

const FIXTURES: &[&str] = &[
    include_str!("../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/verified_publisher_stable_current.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/artifact_above_published_version_narrows_to_beta.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/catalog_asserted_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/ambient_template_privilege_withdrawn.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/nonconformant_sample_withdrawn.json"),
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
        let packet = StableSdkAuthorLanePacket::from_input(fixture.packet_input.clone())
            .map_err(|e| format!("fixture {} must build: {e}", fixture.case_name))?;
        packets.push(packet);
    }

    match mode {
        "packets" => print_json(&packets)?,
        "support-exports" => {
            let exports: Vec<_> = packets
                .iter()
                .map(project_stable_sdk_author_lane_support_export)
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
