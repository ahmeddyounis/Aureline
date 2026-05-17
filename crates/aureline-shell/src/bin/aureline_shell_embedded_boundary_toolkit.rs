//! Headless inspector for the embedded-boundary toolkit projection.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- events
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- validate
//! ```

use aureline_shell::embedded_boundary::{
    seeded_embedded_boundary_toolkit_page, validate_embedded_boundary_toolkit_page,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_embedded_boundary_toolkit_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("rows") => {
            print_json(&page.rows)?;
        }
        Some("events") => {
            print_json(&page.event_log)?;
        }
        Some("support-export") => {
            print_json(&page.support_export)?;
        }
        Some("defects") => {
            print_json(&page.defects)?;
        }
        Some("validate") => match validate_embedded_boundary_toolkit_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} field={} note={}",
                        defect.defect_kind_token, defect.toolkit_row_id, defect.field, defect.note,
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
