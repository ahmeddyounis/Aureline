//! Headless emitter for the M5 descriptor-certification packet.
//!
//! The bin is the only mint-from-truth path for the published certification inventory checked in at
//! `artifacts/public-truth/m5-descriptor-certification.json`, the release-grade parity proof under
//! `artifacts/release/m5-descriptor-parity-proof/descriptor-certification.json` (and its Markdown
//! report), and the per-state certification fixtures under `fixtures/public-truth/m5-badge-consumers/`.
//! The packet certifies every claimed M5 consumer against the shared runtime lanes it reads —
//! mapping each to current descriptor schemas, badge families, downgrade rules, and parity-proof
//! fixtures — and auto-narrows a consumer's claim the moment any lane it reads goes stale or failing.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- variant <canonical|stale|missing>
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_certification -- validate
//! ```

use aureline_release::m5_descriptor_certification::{
    seeded_m5_descriptor_certification, seeded_m5_descriptor_certification_missing_proof_blocked,
    seeded_m5_descriptor_certification_stale_proof_narrowed, M5DescriptorCertification,
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
        Some("registry") | None => {
            let packet = seeded_m5_descriptor_certification();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_descriptor_certification();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_descriptor_certification();
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
            let packet = seeded_m5_descriptor_certification();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_descriptor_certification_stale_proof_narrowed())?;
            assert_packet_valid(&seeded_m5_descriptor_certification_missing_proof_blocked())?;
            println!(
                "ok: descriptor-certification packet valid ({} lanes, {} consumers)",
                packet.lanes.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5DescriptorCertification, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_descriptor_certification()),
        "stale" => Ok(seeded_m5_descriptor_certification_stale_proof_narrowed()),
        "missing" => Ok(seeded_m5_descriptor_certification_missing_proof_blocked()),
        other => Err(format!("unknown variant: {other} (canonical|stale|missing)").into()),
    }
}

fn assert_packet_valid(
    packet: &M5DescriptorCertification,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
