use std::env;
use std::process;

use aureline_provider::seeded_provider_event_reconciliation_page;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = match args.first().map(String::as_str) {
        None | Some("page") => Mode::Page,
        Some("validate") => Mode::Validate,
        Some("support-export") => Mode::SupportExport,
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!(
                "usage: aureline_provider_event_reconciliation [page|validate|support-export]"
            );
            process::exit(2);
        }
    };

    let page = seeded_provider_event_reconciliation_page();
    match mode {
        Mode::Page => {
            println!(
                "{}",
                serde_json::to_string_pretty(&page).expect("page serializes")
            );
        }
        Mode::Validate => {
            let report = page.validate();
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("report serializes")
            );
            if !report.passed {
                process::exit(1);
            }
        }
        Mode::SupportExport => {
            let export = page.support_export_projection();
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
