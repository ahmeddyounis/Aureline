//! Headless inspector for release-truth wiring across docs/help surfaces.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_truth_wiring -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_truth_wiring -- json
//! cargo run -q -p aureline-shell --bin aureline_shell_truth_wiring -- validate
//! ```

use aureline_shell::docs_browser::seeded_truth_wiring_report;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_truth_wiring_report();

    match args.first().map(String::as_str) {
        Some("markdown") | None => {
            print!("{}", report.render_markdown());
        }
        Some("json") => {
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Some("validate") => {
            if report.defects.is_empty() {
                println!("ok");
            } else {
                for defect in &report.defects {
                    eprintln!(
                        "defect: kind={} surface={} field={} note={}",
                        defect.defect_kind_token,
                        defect.surface_class_token,
                        defect.field,
                        defect.note,
                    );
                }
                std::process::exit(3);
            }
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }

    Ok(())
}
