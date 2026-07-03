//! Headless emitter for the M5 capability-sheet primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-capability-sheet-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-capability-sheet-primitive.md`, and
//! the narrowed fixtures under `fixtures/ui/m5-capability-sheet-primitive/`. Every
//! M5 trust lane that asks for meaningful access (extension install, AI tool,
//! provider route, remote connector, automation flow, and privileged helper) reads
//! this primitive so consequence grouping, transitive scope, reduced-mode choices,
//! and the stable revoke / re-consent paths stay consistent, and so the support
//! export reconstructs capability truth from one shared sheet model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- fixture-automation-flow-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- fixture-privileged-helper-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_capability_sheet_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_capability_sheet_consequence_grouping_transitive_scope_and_reconsent_primitive::{
    seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed,
    seeded_m5_capability_sheet_primitive_packet,
    seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed,
    M5CapabilitySheetPrimitivePacket,
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
            let packet = seeded_m5_capability_sheet_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_capability_sheet_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_capability_sheet_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-automation-flow-beta-narrowed") => {
            let packet = seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-privileged-helper-preview-narrowed") => {
            let packet = seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_capability_sheet_primitive_packet(),
                seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed(),
                seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed(),
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
    packet: &M5CapabilitySheetPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
