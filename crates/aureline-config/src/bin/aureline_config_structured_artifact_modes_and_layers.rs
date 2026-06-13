//! Emits the canonical config artifact header/mode/layer packet.
//!
//! Examples:
//!
//! ```text
//! cargo run -q -p aureline-config --bin aureline_config_structured_artifact_modes_and_layers -- json
//! cargo run -q -p aureline-config --bin aureline_config_structured_artifact_modes_and_layers -- markdown
//! ```

use std::env;

use aureline_config::structured_config_artifact_modes_and_layers::{
    seeded_structured_config_artifact_modes_and_layers, ModeClass,
};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = env::args().nth(1).unwrap_or_else(|| "json".to_owned());
    let packet = seeded_structured_config_artifact_modes_and_layers();

    match cmd.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        "markdown" => {
            println!("# Structured config artifact modes and layers");
            println!();
            println!("- Record kind: `{}`", packet.record_kind);
            println!("- Schema: `{}`", packet.schema_ref);
            println!("- Docs: `{}`", packet.docs_ref);
            println!(
                "- Artifact families: {}",
                packet.summary.artifact_surface_count
            );
            println!(
                "- Environment-bearing families: {}",
                packet.summary.environment_stack_count
            );
            println!();
            println!("## Shared vocabulary");
            println!();
            println!("### Modes");
            println!();
            println!("| Mode | Canonical writable | Description |");
            println!("|---|---|---|");
            for row in &packet.mode_vocabulary {
                println!(
                    "| `{}` | `{}` | {} |",
                    token(&row.mode),
                    row.canonical_writable_truth,
                    row.description
                );
            }
            println!();
            println!("### Surface bindings");
            println!();
            println!("| Surface | Header fields | Modes | Layer fields |");
            println!("|---|---|---|---|");
            for row in &packet.surface_vocabulary {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.surface),
                    row.header_fields.len(),
                    row.mode_labels.len(),
                    row.layer_fields.len()
                );
            }
            println!();
            println!("## Artifact families");
            println!();
            println!("| Family | Qualification | Active mode | Layer stack |");
            println!("|---|---|---|---|");
            for row in &packet.artifact_surfaces {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&row.family),
                    token(&row.qualification_label),
                    token(&row.header.active_mode),
                    row.environment_stack_required
                );
            }
            println!();
            println!("## Active mode counts");
            println!();
            for mode in ModeClass::ALL {
                let count = packet
                    .artifact_surfaces
                    .iter()
                    .filter(|row| row.header.active_mode == mode)
                    .count();
                println!("- `{}`: {}", token(&mode), count);
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
