//! Headless inspector for beta design-system contract records.
//!
//! The binary emits the component-state registry, screenshot-diff matrix, token
//! conformance packet, or a validation result for all seeded records.

use aureline_design_system::{
    seeded_component_state_registry, seeded_screenshot_diff_packet,
    seeded_token_conformance_packet, validate_seeded_beta_contract,
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
        Some("registry") | None => print_json(&seeded_component_state_registry())?,
        Some("screenshot-diff") => print_json(&seeded_screenshot_diff_packet())?,
        Some("token-conformance") => print_json(&seeded_token_conformance_packet())?,
        Some("validate") => match validate_seeded_beta_contract() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for (lane, lane_findings) in findings {
                    for finding in lane_findings {
                        eprintln!(
                            "finding: lane={} check_id={} field={} note={}",
                            lane, finding.check_id, finding.field, finding.note
                        );
                    }
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
