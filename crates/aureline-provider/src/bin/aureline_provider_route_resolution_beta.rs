use std::env;
use std::process;

use aureline_provider::{
    audit_route_resolution_beta_page, seeded_route_resolution_beta_page,
    RouteResolutionBetaSupportExport,
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
                "usage: aureline_provider_route_resolution_beta [page|validate|support-export]"
            );
            process::exit(2);
        }
    };

    let page = seeded_route_resolution_beta_page();
    match mode {
        Mode::Page => {
            println!(
                "{}",
                serde_json::to_string_pretty(&page).expect("page serializes")
            );
        }
        Mode::Validate => {
            let defects = audit_route_resolution_beta_page(
                &page.rows,
                &page.browser_handoff_panels,
                &page.authority_truth_panels,
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&defects).expect("defects serialize")
            );
            if !defects.is_empty() {
                process::exit(1);
            }
        }
        Mode::SupportExport => {
            let export = RouteResolutionBetaSupportExport::from_page(
                "route-resolution-beta:support-export:001",
                "2026-05-16T11:00:00Z",
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
