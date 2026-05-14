use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use aureline_provider::ConnectedProviderAlphaPacket;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let fixture_path = match parse_fixture_path(&args) {
        Ok(path) => path,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("usage: aureline_provider_alpha [--fixture PATH] [--validate-only]");
            process::exit(2);
        }
    };

    let validate_only = args.iter().any(|arg| arg == "--validate-only");
    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(error) => {
            eprintln!("failed to read {}: {error}", fixture_path.display());
            process::exit(2);
        }
    };
    let packet: ConnectedProviderAlphaPacket = match serde_json::from_str(&payload) {
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

fn parse_fixture_path(args: &[String]) -> Result<PathBuf, String> {
    let mut path = default_fixture_path();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--fixture" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("--fixture requires a path".to_string());
                };
                path = PathBuf::from(value);
            }
            "--validate-only" => {}
            other => return Err(format!("unknown argument: {other}")),
        }
        index += 1;
    }
    Ok(path)
}

fn default_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/connected_provider_alpha/registry_packet.json")
}
