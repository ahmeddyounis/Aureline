//! Headless emitter for the stable safety-critical string catalog and
//! glossary-linked controlled terms.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-terminology-proof/` and the localized / offline-mirror
//! fixtures under `fixtures/content/m5-safety-critical-strings/`. UI, CLI/help,
//! docs, support exports, AI surfaces, and narrated/durable surfaces resolve copy
//! through this catalog so a literal string never becomes the source of truth for
//! protected terminology.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_safety_critical_strings -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_safety_critical_strings -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_safety_critical_strings -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_safety_critical_strings -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_safety_critical_strings -- validate
//! ```

use aureline_shell::m5_safety_critical_string_catalog::{
    seeded_safety_critical_string_catalog, seeded_safety_critical_string_catalog_localized,
    seeded_safety_critical_string_catalog_offline_mirror, SafetyCriticalStringCatalog,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let catalog = seeded_safety_critical_string_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_safety_critical_string_catalog().render_markdown_summary()
            );
        }
        Some("fixture-localized") => {
            let catalog = seeded_safety_critical_string_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_safety_critical_string_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("validate") => {
            for catalog in [
                seeded_safety_critical_string_catalog(),
                seeded_safety_critical_string_catalog_localized(),
                seeded_safety_critical_string_catalog_offline_mirror(),
            ] {
                assert_valid(&catalog)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(catalog: &SafetyCriticalStringCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
