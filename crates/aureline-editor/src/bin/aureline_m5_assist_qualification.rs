//! Headless emitter for the editor-assist qualification packet.
//!
//! Prints the canonical packet that binds the editor-assist micro-surface proof
//! sources into one per-family claim verdict. About/help, service-health,
//! compatibility, release automation, and support export all consume this
//! packet instead of restating assist-quality claims by hand. With `--lines`,
//! prints the human-readable projection instead of JSON. The packet takes no
//! input: it reads the real in-code proof sources.
//!
//! ```sh
//! cargo run --bin aureline_m5_assist_qualification          # JSON
//! cargo run --bin aureline_m5_assist_qualification -- --lines
//! ```

use aureline_editor::{assist_qualification_lines, assist_qualification_packet};

fn main() {
    let mut want_lines = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--lines" => want_lines = true,
            "--help" | "-h" => {
                eprintln!(
                    "usage: aureline_m5_assist_qualification [--lines]\n\
                     prints the canonical editor-assist qualification packet as JSON, or the \
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

    let packet = assist_qualification_packet();

    if want_lines {
        for line in assist_qualification_lines(&packet) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&packet).expect("packet must serialize")
        );
    }
}
