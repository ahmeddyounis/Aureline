use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use aureline_provider::{ApprovalTicketAlphaPacket, ConnectedProviderAlphaPacket};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = match parse_options(&args) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            eprintln!(
                "usage: aureline_provider_alpha [--fixture PATH] [--approval-ticket-alpha] [--validate-only]"
            );
            process::exit(2);
        }
    };
    let fixture_path = options
        .fixture_path
        .unwrap_or_else(|| default_fixture_path(options.mode));

    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(error) => {
            eprintln!("failed to read {}: {error}", fixture_path.display());
            process::exit(2);
        }
    };

    match options.mode {
        ProviderAlphaMode::ConnectedProvider => {
            run_connected_provider_alpha(&payload, &fixture_path, options.validate_only)
        }
        ProviderAlphaMode::ApprovalTicket => {
            run_approval_ticket_alpha(&payload, &fixture_path, options.validate_only)
        }
    }
}

fn run_connected_provider_alpha(payload: &str, fixture_path: &Path, validate_only: bool) {
    let packet: ConnectedProviderAlphaPacket = match serde_json::from_str(payload) {
        Ok(packet) => packet,
        Err(error) => {
            eprintln!("failed to parse {}: {error}", fixture_path.display());
            process::exit(2);
        }
    };

    let report = packet.validate();
    if !report.passed {
        let output = serde_json::to_string_pretty(&report).expect("validation report serializes");
        eprintln!("{output}");
        process::exit(1);
    }

    if validate_only {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report serializes")
        );
    } else {
        let projection = packet.support_export_projection();
        println!(
            "{}",
            serde_json::to_string_pretty(&projection).expect("projection serializes")
        );
    }
}

fn run_approval_ticket_alpha(payload: &str, fixture_path: &Path, validate_only: bool) {
    let packet: ApprovalTicketAlphaPacket = match serde_json::from_str(payload) {
        Ok(packet) => packet,
        Err(error) => {
            eprintln!("failed to parse {}: {error}", fixture_path.display());
            process::exit(2);
        }
    };

    let report = packet.validate();
    if !report.passed {
        let output = serde_json::to_string_pretty(&report).expect("validation report serializes");
        eprintln!("{output}");
        process::exit(1);
    }

    if validate_only {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report serializes")
        );
    } else {
        let projection = packet.support_admin_projection();
        println!(
            "{}",
            serde_json::to_string_pretty(&projection).expect("projection serializes")
        );
    }
}

#[derive(Debug, Clone, Copy)]
enum ProviderAlphaMode {
    ConnectedProvider,
    ApprovalTicket,
}

struct ProviderAlphaOptions {
    fixture_path: Option<PathBuf>,
    mode: ProviderAlphaMode,
    validate_only: bool,
}

fn parse_options(args: &[String]) -> Result<ProviderAlphaOptions, String> {
    let mut fixture_path = None;
    let mut mode = ProviderAlphaMode::ConnectedProvider;
    let mut validate_only = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--fixture" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("--fixture requires a path".to_string());
                };
                fixture_path = Some(PathBuf::from(value));
            }
            "--approval-ticket-alpha" => {
                mode = ProviderAlphaMode::ApprovalTicket;
            }
            "--validate-only" => {
                validate_only = true;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        index += 1;
    }
    Ok(ProviderAlphaOptions {
        fixture_path,
        mode,
        validate_only,
    })
}

fn default_fixture_path(mode: ProviderAlphaMode) -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match mode {
        ProviderAlphaMode::ConnectedProvider => manifest_dir
            .join("../../fixtures/providers/connected_provider_alpha/registry_packet.json"),
        ProviderAlphaMode::ApprovalTicket => {
            manifest_dir.join("../../fixtures/security/approval_ticket_alpha/baseline_packet.json")
        }
    }
}
