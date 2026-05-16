//! Headless inspector for marketplace truth rows.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- validate
//! ```

use aureline_shell::extensions::marketplace::{
    seeded_marketplace_truth_page, validate_marketplace_truth_page,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_marketplace_truth_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("rows") => {
            print_json(&page.rows)?;
        }
        Some("support-rows") => {
            print_json(&page.support_rows)?;
        }
        Some("validate") => match validate_marketplace_truth_page(&page) {
            Ok(()) => println!("ok"),
            Err(errors) => {
                for err in errors {
                    eprintln!("marketplace truth error: {err}");
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
