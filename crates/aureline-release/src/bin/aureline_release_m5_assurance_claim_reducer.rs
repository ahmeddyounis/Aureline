//! Headless emitter for the M5 assurance-claim reducer.
//!
//! The bin is the only mint-from-truth path for the published reducer inventory checked in at
//! `artifacts/public-truth/m5-assurance-claim-reducer.json`, the rendered narrowing overview document
//! at `artifacts/public-truth/m5-assurance-claim-reducer.md`, the machine-readable claim /
//! precondition matrix at `artifacts/public-truth/m5-assurance-claim-reducer-claims.csv`, the
//! release-grade narrowing proof under `artifacts/public-truth/m5-assurance-narrowing-proof/` (and its
//! Markdown report and exported redaction-safe preview), and the per-state drill fixtures under
//! `fixtures/public-truth/assurance-claim-narrowing/`. The reducer narrows every regulated /
//! self-hosted / sovereign / no-vendor / no-telemetry / customer-managed-key claim the moment a
//! supporting precondition drifts and drives every consumer surface from that one output, so a claim
//! can never read stronger than the trust facts behind it.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- overview
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- export
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- variant <canonical|stale-evidence|hosted-dependency|key-residency|policy-path>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- claim <claim-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- validate
//! ```

use aureline_release::m5_assurance_claim_reducer::{
    seeded_m5_assurance_claim_reducer,
    seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed,
    seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked,
    seeded_m5_assurance_claim_reducer_policy_path_regression_blocked,
    seeded_m5_assurance_claim_reducer_stale_evidence_narrowed, M5AssuranceClaimReducer,
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
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("overview") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_overview_markdown());
        }
        Some("csv") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_claims_csv());
        }
        Some("markdown") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("export") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            println!("{}", packet.render_export_preview());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("claim") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let claim = packet
                .reduced_claims
                .iter()
                .find(|c| c.subject.as_str() == token)
                .ok_or_else(|| format!("unknown claim token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(claim)?);
        }
        Some("validate") => {
            let packet = seeded_m5_assurance_claim_reducer();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_assurance_claim_reducer_stale_evidence_narrowed())?;
            assert_packet_valid(
                &seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed(),
            )?;
            assert_packet_valid(
                &seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked(),
            )?;
            assert_packet_valid(
                &seeded_m5_assurance_claim_reducer_policy_path_regression_blocked(),
            )?;
            println!(
                "ok: assurance-claim reducer valid ({} claims)",
                packet.reduced_claims.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<M5AssuranceClaimReducer, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_assurance_claim_reducer()),
        "stale-evidence" => Ok(seeded_m5_assurance_claim_reducer_stale_evidence_narrowed()),
        "hosted-dependency" => {
            Ok(seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed())
        }
        "key-residency" => Ok(seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked()),
        "policy-path" => Ok(seeded_m5_assurance_claim_reducer_policy_path_regression_blocked()),
        other => Err(format!(
            "unknown variant: {other} (canonical|stale-evidence|hosted-dependency|key-residency|policy-path)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &M5AssuranceClaimReducer) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
