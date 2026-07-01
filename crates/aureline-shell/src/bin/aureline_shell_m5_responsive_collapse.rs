//! Headless emitter for the M5 responsive-collapse (compact/standard/expanded) proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard,
//! support export, and CSV checked in under
//! `artifacts/release/m5-responsive-collapse-proof/` and the markdown report under
//! `artifacts/shell/m5-responsive-collapse.md`, plus the protected fixtures under
//! `fixtures/ui/m5-responsive-collapse/`. Shell / windowing / layout / release
//! automation, release-center, docs/help, and support exports resolve the same
//! collapse rows — green/yellow/red status, active waivers, and the exact collapse
//! causes — through this proof rather than restating collapse, identity, or zoom
//! posture by hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- validate
//! ```

use aureline_shell::m5_responsive_collapse::{
    seeded_m5_responsive_collapse_packet, validate_m5_responsive_collapse_packet,
    ResponsiveCollapsePacket, ResponsiveCollapseSupportExport,
    M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            let export = ResponsiveCollapseSupportExport::from_packet(
                M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_responsive_collapse_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &ResponsiveCollapsePacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_responsive_collapse_packet(packet) {
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
