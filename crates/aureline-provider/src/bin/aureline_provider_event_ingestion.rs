use std::env;
use std::process;

use aureline_provider::seeded_provider_event_ingestion_packet;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = match args.first().map(String::as_str) {
        None | Some("packet") => Mode::Packet,
        Some("validate") => Mode::Validate,
        Some("support-export") => Mode::SupportExport,
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!("usage: aureline_provider_event_ingestion [packet|validate|support-export]");
            process::exit(2);
        }
    };

    let packet = seeded_provider_event_ingestion_packet();
    match mode {
        Mode::Packet => {
            println!(
                "{}",
                serde_json::to_string_pretty(&packet).expect("packet serializes")
            );
        }
        Mode::Validate => {
            let report = packet.validate();
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("report serializes")
            );
            if !report.passed {
                process::exit(1);
            }
        }
        Mode::SupportExport => {
            println!(
                "{}",
                serde_json::to_string_pretty(&packet.support_export).expect("export serializes")
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Packet,
    Validate,
    SupportExport,
}
