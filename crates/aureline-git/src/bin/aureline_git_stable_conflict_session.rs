//! CLI mirror for inspecting stable conflict-session packets.

use std::path::PathBuf;

use aureline_git::{
    build_stable_conflict_session_packet, parse_stable_conflict_session_record,
    project_stable_conflict_session, StableConflictSessionInput,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut payload_path = None;
    let mut mode = "project".to_string();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--payload" => payload_path = args.next().map(PathBuf::from),
            "--mode" => mode = args.next().unwrap_or_else(|| "project".to_string()),
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => payload_path = Some(PathBuf::from(other)),
        }
    }

    let Some(payload_path) = payload_path else {
        eprintln!("missing --payload <json-file>");
        print_usage();
        std::process::exit(2);
    };

    let payload = match std::fs::read_to_string(&payload_path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("read payload {}: {err}", payload_path.display());
            std::process::exit(5);
        }
    };

    match mode.as_str() {
        "project" => match project_stable_conflict_session(&payload) {
            Ok(projection) => match serde_json::to_string_pretty(&projection) {
                Ok(json) => println!("{json}"),
                Err(err) => {
                    eprintln!("serialization failed: {err}");
                    std::process::exit(7);
                }
            },
            Err(err) => {
                eprintln!("stable conflict session projection failed: {err}");
                std::process::exit(6);
            }
        },
        "build" => {
            let input: StableConflictSessionInput = match serde_json::from_str(&payload) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("parse input failed: {err}");
                    std::process::exit(6);
                }
            };
            match build_stable_conflict_session_packet(input) {
                Ok(packet) => match serde_json::to_string_pretty(&packet) {
                    Ok(json) => println!("{json}"),
                    Err(err) => {
                        eprintln!("serialization failed: {err}");
                        std::process::exit(7);
                    }
                },
                Err(err) => {
                    eprintln!("build packet failed: {err}");
                    std::process::exit(6);
                }
            }
        }
        "validate" => match parse_stable_conflict_session_record(&payload) {
            Ok(record) => match serde_json::to_string_pretty(&record.project()) {
                Ok(json) => println!("{json}"),
                Err(err) => {
                    eprintln!("serialization failed: {err}");
                    std::process::exit(7);
                }
            },
            Err(err) => {
                eprintln!("validation failed: {err}");
                std::process::exit(6);
            }
        },
        other => {
            eprintln!("unknown mode: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_stable_conflict_session --payload <json-file> [--mode project|build|validate]"
    );
}
