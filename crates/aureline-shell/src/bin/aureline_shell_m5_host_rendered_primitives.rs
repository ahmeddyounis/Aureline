//! Headless emitter for the M5 host-rendered primitive layer.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-host-rendered-primitives-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-host-rendered-primitives.md`, and the
//! narrowed fixtures under `fixtures/ui/m5-host-rendered-primitives/`. Every M5 first
//! consumer that renders a settings row, a capability sheet, an event / history row, a
//! timeline group, or a chronology export preview reads this layer so canonical
//! host-rendered primitives and shared token / state wiring stay consistent across the
//! desktop, companion, and extension hosts, and so the support export reconstructs the
//! binding verdict from one shared model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- fixture-capability-sheet-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- fixture-chronology-export-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- validate
//! ```

use aureline_shell::implement_the_m5_host_rendered_trust_component_primitives_and_token_state_wiring::{
    seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed,
    seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed,
    seeded_m5_host_rendered_primitive_packet, M5HostRenderedPrimitivePacket,
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
            let packet = seeded_m5_host_rendered_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_host_rendered_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_host_rendered_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-capability-sheet-beta-narrowed") => {
            let packet = seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-chronology-export-preview-narrowed") => {
            let packet = seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_host_rendered_primitive_packet(),
                seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed(),
                seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed(),
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

fn assert_valid(packet: &M5HostRenderedPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
