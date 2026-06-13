//! Emits the canonical structured-config parameter-source and round-trip-review
//! packet.
//!
//! Examples:
//!
//! ```text
//! cargo run -q -p aureline-config --bin aureline_config_structured_parameter_source_and_round_trip_review -- json
//! cargo run -q -p aureline-config --bin aureline_config_structured_parameter_source_and_round_trip_review -- markdown
//! ```

use std::env;

use aureline_config::structured_config_parameter_source_and_round_trip_review::{
    seeded_structured_config_parameter_source_and_round_trip_review, OutputDisclosureClass,
    ValueChipClass,
};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = env::args().nth(1).unwrap_or_else(|| "json".to_owned());
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();

    match cmd.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        "markdown" => {
            println!("# Structured config parameter-source and round-trip review");
            println!();
            println!("- Record kind: `{}`", packet.record_kind);
            println!("- Schema: `{}`", packet.schema_ref);
            println!("- Docs: `{}`", packet.docs_ref);
            println!(
                "- Artifact families: {}",
                packet.summary.artifact_review_count
            );
            println!(
                "- Compare-before-save families: {}",
                packet.summary.family_count_with_compare_before_save
            );
            println!(
                "- Raw secret export blocked everywhere: `{}`",
                packet.summary.raw_secret_export_blocked_everywhere
            );
            println!();
            println!("## Value chip classes");
            println!();
            println!("| Class | Default secret export blocked |");
            println!("|---|---|");
            for row in &packet.value_chip_vocabulary {
                println!(
                    "| `{}` | `{}` |",
                    token(&row.chip_class),
                    row.raw_secret_export_blocked_by_default
                );
            }
            println!();
            println!("## Artifact families");
            println!();
            println!("| Family | Qualification | Compare-before-save | Export disclosures |");
            println!("|---|---|---|---|");
            for review in &packet.artifact_reviews {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` |",
                    token(&review.family),
                    token(&review.qualification_label),
                    review.compare_before_save_sheet.is_some(),
                    review
                        .export_summary
                        .output_disclosure_classes
                        .iter()
                        .map(token)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!();
            println!("## Coverage");
            println!();
            let chip_classes: Vec<_> = ValueChipClass::ALL.iter().map(token).collect();
            let output_classes: Vec<_> = OutputDisclosureClass::ALL.iter().map(token).collect();
            println!("- Value chip classes: `{}`", chip_classes.join("`, `"));
            println!(
                "- Output disclosure classes: `{}`",
                output_classes.join("`, `")
            );
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
