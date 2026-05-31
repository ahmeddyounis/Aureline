//! Dump stable extension runtime-ABI records for every canonical fixture.
//!
//! Used by the checked fixture, support-export, and docs/schema validation lanes
//! so a materialized packet can be validated against
//! `schemas/extensions/stable_runtime_abi.schema.json` with an independent
//! Draft 2020-12 validator:
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_stable_runtime_abi_records -- packets
//! cargo run -q -p aureline-extensions --example dump_stable_runtime_abi_records -- support-exports
//! cargo run -q -p aureline-extensions --example dump_stable_runtime_abi_records -- validate
//! ```

use aureline_extensions::{
    project_stable_runtime_abi_support_export, StableRuntimeAbiInput, StableRuntimeAbiPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableRuntimeAbiInput,
}

const FIXTURES: &[&str] = &[
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/wasm_capability_sandbox_stable_current.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/external_host_fail_closed_downgraded_narrows_to_beta.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/remote_side_component_unenforceable_withdrawn.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/compatibility_bridge_quarantined_contribution_narrows_to_preview.json"),
    include_str!("../../../fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/declarative_view_catalog_asserted_narrows_to_preview.json"),
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
        let packet = StableRuntimeAbiPacket::from_input(fixture.packet_input.clone())
            .map_err(|e| format!("fixture {} must build: {e}", fixture.case_name))?;
        packets.push(packet);
    }

    match mode {
        "packets" => print_json(&packets)?,
        "support-exports" => {
            let exports: Vec<_> = packets
                .iter()
                .map(project_stable_runtime_abi_support_export)
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
