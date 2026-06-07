//! CLI/headless inspector for the stable service-health destination truth descriptor.
//!
//! The binary emits the same descriptor, vocabulary, validation report, or
//! support-export projection consumed by Help/About, service health, diagnostics,
//! release notes, migration notices, community handoff, and support export.

use aureline_service_health::{
    canonical_service_health_destination_truth_descriptor, canonical_service_health_feed,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let descriptor = canonical_service_health_destination_truth_descriptor();
    let canonical_feed = canonical_service_health_feed();

    match args.first().map(String::as_str) {
        Some("descriptor") | None => print_json(&descriptor)?,
        Some("canonical-feed") => print_json(&canonical_feed)?,
        Some("canonical-feed-support-export") => {
            print_json(&canonical_feed.support_export_projection())?
        }
        Some("canonical-feed-validation") => print_json(&canonical_feed.validate())?,
        Some("shared-feed") => print_json(&descriptor.shared_service_health_feed())?,
        Some("support-export") => print_json(&descriptor.support_export_projection())?,
        Some("shared-support-export") => {
            print_json(&descriptor.shared_service_health_feed().support_export_projection())?
        }
        Some("validation") => print_json(&descriptor.validate())?,
        Some("vocabulary") => {
            println!("service_contract_state:");
            for token in descriptor.service_contract_state_vocabulary() {
                println!("  {token}");
            }
            println!("destination_trust_class:");
            for token in descriptor.destination_trust_class_vocabulary() {
                println!("  {token}");
            }
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
