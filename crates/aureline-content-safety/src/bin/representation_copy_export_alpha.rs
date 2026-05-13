use std::fs;

use aureline_content_safety::{RepresentationCopyExportAlphaPacket, RepresentationCopyExportCase};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: representation_copy_export_alpha <case.json>");
        std::process::exit(2);
    }

    let raw = match fs::read_to_string(&args[1]) {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!("failed to read {}: {err}", args[1]);
            std::process::exit(1);
        }
    };
    let case: RepresentationCopyExportCase = match serde_json::from_str(&raw) {
        Ok(case) => case,
        Err(err) => {
            eprintln!("failed to parse {}: {err}", args[1]);
            std::process::exit(1);
        }
    };

    let packet = RepresentationCopyExportAlphaPacket::from_case(case);
    let report = packet.validate();
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize validation report: {err}");
            std::process::exit(1);
        }
    }

    if !report.passed() {
        std::process::exit(1);
    }
}
