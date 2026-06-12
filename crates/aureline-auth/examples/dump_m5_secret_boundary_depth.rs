//! Emits the canonical M5 secret-boundary depth packet or support export.
//!
//! Usage:
//! `cargo run -q -p aureline-auth --example dump_m5_secret_boundary_depth -- packet`
//! `cargo run -q -p aureline-auth --example dump_m5_secret_boundary_depth -- support-export`

use std::error::Error;

use aureline_auth::{seeded_m5_secret_boundary_depth_packet, SecretBoundarySupportExport};

fn main() -> Result<(), Box<dyn Error>> {
    let packet = seeded_m5_secret_boundary_depth_packet();
    match std::env::args().nth(1).as_deref() {
        Some("support-export") => print_json(&SecretBoundarySupportExport::from_packet(
            "m5-secret-boundary-depth:support-export",
            "2026-06-12T00:00:00Z",
            &packet,
        ))?,
        _ => print_json(&packet)?,
    }
    Ok(())
}

fn print_json(value: &impl serde::Serialize) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
