//! Headless inspector for extension settings, permissions, and runtime placement.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- permissions
//! cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- settings
//! cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- validate
//! ```

use aureline_shell::extensions::inspectors::{
    seeded_extension_inspector_page, seeded_extension_inspector_support_export,
    validate_extension_inspector_page, validate_extension_inspector_support_export,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_extension_inspector_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("permissions") => print_json(&page.permission_inspector)?,
        Some("settings") => print_json(&page.settings_inspector)?,
        Some("support-export") => {
            let export = seeded_extension_inspector_support_export();
            print_json(&export)?;
        }
        Some("validate") => {
            if let Err(errors) = validate_extension_inspector_page(&page) {
                for err in errors {
                    eprintln!("extension inspector error: {err}");
                }
                std::process::exit(3);
            }
            let export = seeded_extension_inspector_support_export();
            if let Err(errors) = validate_extension_inspector_support_export(&export, &page) {
                for err in errors {
                    eprintln!("extension inspector support export error: {err}");
                }
                std::process::exit(3);
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
