//! Headless emitter for the M5 advisory-claim downgrade certification proof.
//!
//! The bin is the only mint-from-truth path for the published packet, dashboard, support export,
//! and CSV checked in under
//! `artifacts/release/m5-advisory-claim-downgrade-certification-proof/` and the markdown report
//! under `artifacts/security/m5-advisory-claim-downgrade-certification.md`, plus the protected
//! fixtures under `fixtures/security/m5-advisory-claim-downgrade-certification/`. Release,
//! help/about, procurement, evaluation, and support surfaces resolve the same certification rows —
//! green/yellow/red status, controlled badge, active waivers, the distinct claim states, and the
//! exact claim causes with their restore actions — through this proof rather than restating
//! advisory-freshness, mirror-propagation, distribution-signature, or local-continuity posture by
//! hand.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- dashboard
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- validate
//! ```

use aureline_shell::m5_advisory_claim_downgrade_certification::{
    seeded_m5_advisory_claim_downgrade_certification_packet,
    validate_m5_advisory_claim_downgrade_certification_packet, AdvisoryClaimPacket,
    AdvisoryClaimSupportExport, M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_ID,
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
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("dashboard") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            println!("{}", packet.dashboard().export_safe_json());
        }
        Some("support-export") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            let export = AdvisoryClaimSupportExport::from_packet(
                M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_ID,
                packet,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("support export serializes")
            );
        }
        Some("csv") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown());
        }
        Some("compact") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => {
            let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
            assert_valid(&packet)?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &AdvisoryClaimPacket) -> Result<(), Box<dyn std::error::Error>> {
    match validate_m5_advisory_claim_downgrade_certification_packet(packet) {
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
