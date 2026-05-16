//! Dump serialized extension incident communication and support-export
//! records for the checked advisory / emergency-disable fixture suite.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_revocation_records -p aureline-extensions
//! cargo run --example dump_revocation_records -p aureline-extensions -- incident primary_registry_emergency_disable
//! cargo run --example dump_revocation_records -p aureline-extensions -- support-export primary_registry_emergency_disable
//! ```

use aureline_extensions::{
    evaluate_extension_incident_communication, project_extension_incident_support_export,
    ExtensionIncidentCommunicationInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: ExtensionIncidentCommunicationInput,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode, fixture_name] if mode == "incident" || mode == "support-export" => {
            let record = evaluate_fixture(fixture_name)?;
            if mode == "incident" {
                print_json(&record)?;
            } else {
                let export = project_extension_incident_support_export(
                    &record,
                    &format!("extension_incident_support_export:{}", record.incident_id),
                );
                print_json(&export)?;
            }
        }
        [] => dump_all()?,
        _ => {
            return Err(
                "usage: dump_revocation_records [incident|support-export <fixture>]".into(),
            );
        }
    }
    Ok(())
}

fn dump_all() -> Result<(), Box<dyn std::error::Error>> {
    for (name, _) in fixtures() {
        let record = evaluate_fixture(name)?;
        let export = project_extension_incident_support_export(
            &record,
            &format!("extension_incident_support_export:{}", record.incident_id),
        );
        println!("=== {name} / incident ===");
        print_json(&record)?;
        println!("=== {name} / support_export ===");
        print_json(&export)?;
    }
    Ok(())
}

fn evaluate_fixture(
    fixture_name: &str,
) -> Result<aureline_extensions::ExtensionIncidentCommunicationRecord, Box<dyn std::error::Error>> {
    for (name, raw) in fixtures() {
        if name != fixture_name {
            continue;
        }
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        return Ok(evaluate_extension_incident_communication(fixture.input));
    }
    Err(format!("unknown fixture: {fixture_name}").into())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "primary_registry_emergency_disable",
            include_str!(
                "../../../fixtures/extensions/m3/revocation_and_emergency_disable/primary_registry_emergency_disable.json"
            ),
        ),
        (
            "mirror_quarantine_pending_reverify",
            include_str!(
                "../../../fixtures/extensions/m3/revocation_and_emergency_disable/mirror_quarantine_pending_reverify.json"
            ),
        ),
        (
            "artifact_revoked_mirror_parity",
            include_str!(
                "../../../fixtures/extensions/m3/revocation_and_emergency_disable/artifact_revoked_mirror_parity.json"
            ),
        ),
    ]
}
