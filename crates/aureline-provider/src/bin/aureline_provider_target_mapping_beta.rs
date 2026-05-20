use std::env;
use std::process;

use aureline_provider::{
    audit_target_mapping_beta_page, seeded_target_mapping_beta_page, TargetMappingBetaSupportExport,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = match args.first().map(String::as_str) {
        None | Some("page") => Mode::Page,
        Some("validate") => Mode::Validate,
        Some("support-export") => Mode::SupportExport,
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!(
                "usage: aureline_provider_target_mapping_beta [page|validate|support-export]"
            );
            process::exit(2);
        }
    };

    let page = seeded_target_mapping_beta_page();
    match mode {
        Mode::Page => {
            println!(
                "{}",
                serde_json::to_string_pretty(&page).expect("page serializes")
            );
        }
        Mode::Validate => {
            let defects =
                audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
            println!(
                "{}",
                serde_json::to_string_pretty(&defects).expect("defects serialize")
            );
            if !defects.is_empty() {
                process::exit(1);
            }
        }
        Mode::SupportExport => {
            let export = TargetMappingBetaSupportExport::from_page(
                "target-mapping-beta:support-export:001",
                "2026-05-18T12:00:00Z",
                page,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&export).expect("export serializes")
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Page,
    Validate,
    SupportExport,
}
