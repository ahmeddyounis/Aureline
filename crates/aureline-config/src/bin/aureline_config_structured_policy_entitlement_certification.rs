//! Emits the canonical structured-config / policy / entitlement certification packet.
//!
//! Examples:
//!
//! ```text
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- json
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- markdown
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- stale_policy
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- reauth_required
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_certification -- signer_rotation
//! ```

use std::env;

use aureline_config::structured_config_policy_entitlement_certification::{
    seeded_structured_config_policy_entitlement_certification,
    seeded_structured_config_policy_entitlement_certification_scenario, CertificationScenario,
    CertificationState,
};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = env::args().nth(1).unwrap_or_else(|| "json".to_owned());
    let packet = match cmd.as_str() {
        "json" | "markdown" => seeded_structured_config_policy_entitlement_certification(),
        "stale_policy" => seeded_structured_config_policy_entitlement_certification_scenario(
            CertificationScenario::StalePolicy,
        ),
        "reauth_required" => seeded_structured_config_policy_entitlement_certification_scenario(
            CertificationScenario::ReauthRequired,
        ),
        "signer_rotation" => seeded_structured_config_policy_entitlement_certification_scenario(
            CertificationScenario::SignerRotation,
        ),
        other => {
            return Err(format!(
                "unknown subcommand `{other}`; use `json`, `markdown`, `stale_policy`, `reauth_required`, or `signer_rotation`"
            )
            .into())
        }
    };

    match cmd.as_str() {
        "markdown" => {
            println!("# Structured config, policy, and entitlement certification");
            println!();
            println!("- Record kind: `{}`", packet.record_kind);
            println!("- Schema: `{}`", packet.schema_ref);
            println!("- Config doc: `{}`", packet.docs_ref);
            println!("- Help doc: `{}`", packet.help_doc_ref);
            println!("- As of: `{}`", packet.as_of);
            println!();
            println!("## Artifact families");
            println!();
            println!("| Family | Ceiling | Published state | Evidence age (days) |");
            println!("|---|---|---|---|");
            for row in &packet.artifact_rows {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.family),
                    token(&row.claim_ceiling),
                    token(&row.published_state),
                    row.evidence_age_days
                );
            }
            println!();
            println!("## Profiles");
            println!();
            println!("| Profile | Published state | Local-safe floor | Evidence age (days) |");
            println!("|---|---|---|---|");
            for row in &packet.profile_rows {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.profile),
                    token(&row.published_state),
                    token(&row.local_safe_label),
                    row.evidence_age_days
                );
            }
            println!();
            println!("## Publication surfaces");
            println!();
            for row in &packet.surface_rows {
                println!("- `{}` ingests `{}`", token(&row.surface), row.packet_ref);
            }
            println!();
            println!("## Summary");
            println!();
            println!(
                "- Certified artifact families: {}",
                packet.summary.certified_artifact_family_count
            );
            println!(
                "- Narrowed artifact families: {}",
                packet.summary.narrowed_artifact_family_count
            );
            println!(
                "- Certified profiles: {}",
                packet.summary.certified_profile_count
            );
        }
        "json" | "stale_policy" | "reauth_required" | "signer_rotation" => {
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        _ => unreachable!(),
    }

    let _ = CertificationState::CLAIMABLE;
    Ok(())
}

fn token<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("serializable token")
        .trim_matches('"')
        .to_owned()
}
