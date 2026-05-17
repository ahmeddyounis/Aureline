use std::fs;

use aureline_content_safety::{ContentIntegrityBetaCase, ContentIntegrityBetaPacket};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("usage: content_integrity_beta [--packet] <case.json>");
        std::process::exit(2);
    }

    let (emit_packet, path) = if args.len() == 3 {
        if args[1] != "--packet" {
            eprintln!("usage: content_integrity_beta [--packet] <case.json>");
            std::process::exit(2);
        }
        (true, &args[2])
    } else {
        (false, &args[1])
    };

    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            std::process::exit(1);
        }
    };
    let case: ContentIntegrityBetaCase = match serde_json::from_str(&raw) {
        Ok(case) => case,
        Err(err) => {
            eprintln!("failed to parse {path}: {err}");
            std::process::exit(1);
        }
    };

    let packet = ContentIntegrityBetaPacket::from_case(case);
    if emit_packet {
        match serde_json::to_string_pretty(&packet) {
            Ok(json) => println!("{json}"),
            Err(err) => {
                eprintln!("failed to serialize packet: {err}");
                std::process::exit(1);
            }
        }
        return;
    }

    let report = packet.validate();
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize validation report: {err}");
            std::process::exit(1);
        }
    }

    if !report.is_green() {
        std::process::exit(1);
    }
}
