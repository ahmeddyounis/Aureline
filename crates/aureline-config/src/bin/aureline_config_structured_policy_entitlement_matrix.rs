//! Emits the canonical structured config / policy bundle / entitlement matrix.
//!
//! Examples:
//!
//! ```text
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_matrix -- json
//! cargo run -q -p aureline-config --bin aureline_config_structured_policy_entitlement_matrix -- markdown
//! ```

use std::env;

use aureline_config::structured_config_policy_bundle_and_entitlement_matrix::{
    seeded_structured_config_policy_bundle_and_entitlement_matrix, QualificationLabel,
};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = env::args().nth(1).unwrap_or_else(|| "json".to_owned());
    let packet = seeded_structured_config_policy_bundle_and_entitlement_matrix();

    match cmd.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        "markdown" => {
            println!("# Structured config, policy bundle, and offline entitlement matrix");
            println!();
            println!("- Record kind: `{}`", packet.record_kind);
            println!("- Schema: `{}`", packet.schema_ref);
            println!("- Docs: `{}`", packet.docs_ref);
            println!(
                "- Artifact families: {}",
                packet.summary.artifact_family_count
            );
            println!("- Bundle classes: {}", packet.summary.bundle_class_count);
            println!("- Profiles: {}", packet.summary.profile_count);
            println!();
            println!("## Artifact families");
            println!();
            println!("| Family | Qualification | Authored | Effective | Live state |");
            println!("|---|---|---|---|---|");
            for row in &packet.artifact_families {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` |",
                    token(&row.family),
                    match row.qualification_label {
                        QualificationLabel::Stable => "stable",
                        QualificationLabel::Beta => "beta",
                        QualificationLabel::Preview => "preview",
                    },
                    row.authored_source,
                    row.effective_projection,
                    token(&row.live_state_posture)
                );
            }
            println!();
            println!("## Bundle taxonomy");
            println!();
            println!("| Bundle class | Supersedes | Revokes | Distribution paths |");
            println!("|---|---|---|---|");
            for row in &packet.bundle_taxonomy {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.bundle_class),
                    row.supports_supersedes,
                    row.supports_revokes,
                    row.distribution_paths.len()
                );
            }
            println!();
            println!("## Deployment profiles");
            println!();
            println!(
                "| Profile | Qualification | Local-safe posture | Authoritative live observation |"
            );
            println!("|---|---|---|---|");
            for row in &packet.profile_qualifications {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.profile),
                    token(&row.qualification_label),
                    token(&row.local_safe_label),
                    row.supports_authoritative_live_observation
                );
            }
        }
        other => {
            return Err(format!("unknown subcommand `{other}`; use `json` or `markdown`").into());
        }
    }

    Ok(())
}

fn token<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("serializable token")
        .trim_matches('"')
        .to_owned()
}
