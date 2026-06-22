//! Headless emitter for the canonical assist-support / provider-debug packet.
//!
//! Prints the one frozen, export-safe packet that explains completion / hint /
//! hover / peek decisions for support, Project Doctor, and the CLI. With
//! `--lines`, prints the human-readable projection instead of JSON. The packet
//! takes no input: it is the single canonical record.
//!
//! ```sh
//! cargo run --bin aureline_m5_assist_support          # JSON
//! cargo run --bin aureline_m5_assist_support -- --lines
//! ```

use aureline_editor::{assist_support_packet, assist_support_packet_lines};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_assist_support [--lines]\n\
                     prints the canonical assist-support packet as JSON, or the \
                     human-readable projection with --lines."
                );
                return;
            }
            other => {
                eprintln!("unexpected argument: {other}");
                std::process::exit(2);
            }
        }
    }

    let packet = assist_support_packet();

    if want_lines {
        for line in assist_support_packet_lines(&packet) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&packet).expect("packet must serialize")
        );
    }
}
