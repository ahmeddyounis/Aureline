//! Headless emitter for the M5 content-design certification capstone.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard,
//! and support export checked in under
//! `artifacts/release/m5-content-design-certification/` and
//! `artifacts/content/m5-content-truth-dashboard.json`, plus the protected
//! fixtures under `fixtures/release/m5-content-design-certification/`. Release /
//! public-truth automation, release-center, docs/help, and support exports resolve
//! the same content-truth rows — green/yellow/red status, active waivers, and the
//! exact stale-proof causes — through this certification rather than restating
//! wording-governance posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- validate
//! ```

use aureline_shell::content_design_certification::{
    seeded_content_design_certification_packet, validate_content_design_certification_packet,
    ContentDesignCertificationPacket, ContentDesignCertificationSupportExport,
    M5_CONTENT_CERT_SUPPORT_EXPORT_ID,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("packet") | None => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            let export = ContentDesignCertificationSupportExport::from_packet(
                M5_CONTENT_CERT_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("markdown") => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_content_design_certification_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &ContentDesignCertificationPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    match validate_content_design_certification_packet(packet) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let tokens: Vec<String> = errors
                .iter()
                .map(|error| serde_json::to_string(error).unwrap_or_default())
                .collect();
            Err(format!("packet failed validation: {}", tokens.join(",")).into())
        }
    }
}
