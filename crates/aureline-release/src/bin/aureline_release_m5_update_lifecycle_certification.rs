//! Headless emitter for the M5 update / support-lifecycle certification packet.
//!
//! The bin is the only mint-from-truth path for the published certification inventory checked in at
//! `artifacts/release/m5-update-lifecycle-certification.json`, the rendered certification document
//! at `artifacts/release/m5-update-lifecycle-certification.md`, the machine-readable grid CSV at
//! `artifacts/release/m5-update-lifecycle-certification.csv`, the release-grade parity proof under
//! `artifacts/release/m5-update-lifecycle-proof/` (and its Markdown report), and the per-state packet
//! fixtures under `fixtures/release/m5-update-lifecycle-certification/`. It qualifies every claimed
//! M5 channel × deployment profile against the update / support-lifecycle contract — update
//! communication, migration guidance, lifecycle windows, and stale-data behavior — and narrows or
//! blocks a claim deterministically when the backing governance proof is stale, expired, or missing.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- document
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- variant <all-certified|stale-narrowed|missing-blocked>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- validate
//! ```

use aureline_release::m5_update_lifecycle_certification::{
    seeded_m5_update_lifecycle_certification,
    seeded_m5_update_lifecycle_certification_missing_proof_blocked,
    seeded_m5_update_lifecycle_certification_stale_proof_narrowed, M5UpdateLifecycleCertification,
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
        Some("registry") | Some("proof") | None => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("document") => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_certification_markdown());
        }
        Some("markdown") => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_grid_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let consumer = packet
                .consumers
                .iter()
                .find(|c| c.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(consumer)?);
        }
        Some("validate") => {
            let packet = seeded_m5_update_lifecycle_certification();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_update_lifecycle_certification_stale_proof_narrowed())?;
            assert_packet_valid(&seeded_m5_update_lifecycle_certification_missing_proof_blocked())?;
            println!(
                "ok: update-lifecycle certification valid ({} claims, {} consumers)",
                packet.claims.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5UpdateLifecycleCertification, Box<dyn std::error::Error>> {
    match token {
        "all-certified" | "" => Ok(seeded_m5_update_lifecycle_certification()),
        "stale-narrowed" => Ok(seeded_m5_update_lifecycle_certification_stale_proof_narrowed()),
        "missing-blocked" => Ok(seeded_m5_update_lifecycle_certification_missing_proof_blocked()),
        other => Err(format!(
            "unknown variant: {other} (all-certified|stale-narrowed|missing-blocked)"
        )
        .into()),
    }
}

fn assert_packet_valid(
    packet: &M5UpdateLifecycleCertification,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
