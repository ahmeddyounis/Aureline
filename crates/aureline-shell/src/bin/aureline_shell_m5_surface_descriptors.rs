//! Headless emitter for the M5 accessibility-surface descriptor catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/a11y/m5-bridge-descriptor-proof/` and the narrowed fixtures under
//! `fixtures/a11y/m5-surface-descriptors/`. Diagnostics, support exports, docs/help,
//! and assistive-tech conformance automation read this catalog so claimed M5 custom
//! surfaces map into the OS accessibility bridge through one governed descriptor
//! rather than per-surface hand wiring.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- fixture-bridge-degraded
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- fixture-proof-stale-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- validate
//! ```

use aureline_shell::accessibility::{
    seeded_m5_surface_descriptor_catalog, seeded_m5_surface_descriptor_catalog_bridge_degraded,
    seeded_m5_surface_descriptor_catalog_proof_stale_narrowed, M5SurfaceDescriptorCatalogPacket,
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
            let packet = seeded_m5_surface_descriptor_catalog();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_surface_descriptor_catalog().render_markdown_summary()
            );
        }
        Some("fixture-bridge-degraded") => {
            let packet = seeded_m5_surface_descriptor_catalog_bridge_degraded();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-proof-stale-narrowed") => {
            let packet = seeded_m5_surface_descriptor_catalog_proof_stale_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_surface_descriptor_catalog(),
                seeded_m5_surface_descriptor_catalog_bridge_degraded(),
                seeded_m5_surface_descriptor_catalog_proof_stale_narrowed(),
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
    packet: &M5SurfaceDescriptorCatalogPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("descriptor catalog failed validation: {}", tokens.join(",")).into())
    }
}
