//! Headless emitter for the M5 descriptor/badge matrix.
//!
//! The bin is the only mint-from-truth path for the descriptor parity proof and Markdown
//! governance matrix checked in under `artifacts/release/m5-descriptor-parity-proof/` and
//! `artifacts/public-truth/`, the published inventory at
//! `artifacts/public-truth/m5-descriptor-badge-matrix.json`, the standalone descriptor
//! objects under `artifacts/public-truth/descriptors/`, and the stale / missing-proof
//! consumer drill fixtures under `fixtures/public-truth/m5-badge-consumers/`. The matrix
//! freezes the four shared public-truth descriptor objects — provenance, freshness,
//! qualification, and client-scope — their badge families, explanation drawers, and
//! downgrade rules, and binds each claimed consumer surface to the descriptors it renders:
//! a consumer either maps to current descriptor proofs or is auto-narrowed / blocked before
//! Stable promotion, with the gap named rather than hidden.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- support-export
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- descriptor <family>
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- fixture-stale-proof-narrowed
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- fixture-missing-proof-blocked
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- consumer <token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- validate
//! ```

use aureline_release::m5_descriptor_badge::{
    seeded_descriptor_contract, seeded_m5_descriptor_badge_matrix,
    seeded_m5_descriptor_badge_matrix_missing_proof_blocked,
    seeded_m5_descriptor_badge_matrix_stale_proof_narrowed, DescriptorFamily, FreshnessState,
    M5DescriptorBadgeMatrix,
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
        Some("support-export") | None => {
            let packet = seeded_m5_descriptor_badge_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_descriptor_badge_matrix().render_markdown_summary()
            );
        }
        Some("descriptor") => {
            let family = parse_family(args.get(1).map(String::as_str).unwrap_or(""))?;
            let contract = seeded_descriptor_contract(family, FreshnessState::Current);
            let violations = contract.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(format!("descriptor failed validation: {}", tokens.join(",")).into());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&contract).expect("descriptor serializes")
            );
        }
        Some("fixture-stale-proof-narrowed") => {
            let packet = seeded_m5_descriptor_badge_matrix_stale_proof_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-missing-proof-blocked") => {
            let packet = seeded_m5_descriptor_badge_matrix_missing_proof_blocked();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let packet = seeded_m5_descriptor_badge_matrix();
            let binding = packet
                .consumer_bindings
                .iter()
                .find(|b| b.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(binding).expect("consumer serializes")
            );
        }
        Some("validate") => {
            for packet in [
                seeded_m5_descriptor_badge_matrix(),
                seeded_m5_descriptor_badge_matrix_stale_proof_narrowed(),
                seeded_m5_descriptor_badge_matrix_missing_proof_blocked(),
            ] {
                assert_valid(&packet)?;
            }
            for family in DescriptorFamily::ALL {
                let contract = seeded_descriptor_contract(family, FreshnessState::Current);
                if !contract.validate().is_empty() {
                    return Err(format!("descriptor {} failed validation", family.as_str()).into());
                }
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_family(token: &str) -> Result<DescriptorFamily, Box<dyn std::error::Error>> {
    DescriptorFamily::ALL
        .iter()
        .copied()
        .find(|f| f.as_str() == token)
        .ok_or_else(|| format!("unknown descriptor family: {token}").into())
}

fn assert_valid(packet: &M5DescriptorBadgeMatrix) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "descriptor/badge matrix failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
