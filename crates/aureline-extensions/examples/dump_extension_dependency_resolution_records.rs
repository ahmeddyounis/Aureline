//! Dump stable dependency-resolution and publisher-continuity records for every
//! canonical fixture.
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_extension_dependency_resolution_records -- packets
//! cargo run -q -p aureline-extensions --example dump_extension_dependency_resolution_records -- support-exports
//! cargo run -q -p aureline-extensions --example dump_extension_dependency_resolution_records -- validate
//! ```

use aureline_extensions::{
    project_extension_dependency_resolution_support_export, ExtensionDependencyResolutionInput,
    ExtensionDependencyResolutionPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: ExtensionDependencyResolutionInput,
}

const FIXTURES: &[&str] = &[
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/public_install_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/mirrored_update_permission_widening_reconsent_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/enterprise_curated_install_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/rollback_last_known_good_stable.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/key_rotation_cooldown_narrows_to_beta.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/ownership_transfer_pending_notification_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/namespace_dispute_withdrawn.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/maintainer_removal_pending_review_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/orphan_adoption_pending_review_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/approved_mirror_succession_stable.json"),
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
        let packet = ExtensionDependencyResolutionPacket::from_input(fixture.packet_input)
            .map_err(|err| format!("fixture {} must build: {err}", fixture.case_name))?;
        packets.push(packet);
    }
    match mode {
        "packets" => print_json(&packets)?,
        "support-exports" => {
            let exports: Vec<_> = packets
                .iter()
                .map(project_extension_dependency_resolution_support_export)
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
