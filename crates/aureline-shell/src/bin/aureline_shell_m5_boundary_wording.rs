//! Headless emitter for the boundary-wording catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-boundary-wording-proof/` and the localized / offline-mirror
//! fixtures under `fixtures/content/m5-boundary-wording/`. Settings, onboarding,
//! marketplace, help/About, release notes, and account/upgrade prompts resolve the
//! same boundary facts — term, actual posture, identity/network/data/export/rollback
//! implications, disclosed local/open alternatives, and support metadata — through this
//! catalog rather than maintaining parallel, drifting boundary prose.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- parity-report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- explain "<entry id>"
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_boundary_wording -- validate
//! ```

use aureline_shell::content::boundary_wording::{
    seeded_boundary_wording_catalog, seeded_boundary_wording_catalog_localized,
    seeded_boundary_wording_catalog_offline_mirror, BoundaryWordingCatalog,
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
            let catalog = seeded_boundary_wording_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_boundary_wording_catalog().render_markdown_summary()
            );
        }
        Some("parity-report") => {
            let catalog = seeded_boundary_wording_catalog();
            assert_valid(&catalog)?;
            print!("{}", catalog.render_parity_report());
        }
        Some("fixture-localized") => {
            let catalog = seeded_boundary_wording_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_boundary_wording_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("explain") => {
            let entry_id = args.get(1).map(String::as_str).unwrap_or_default();
            let catalog = seeded_boundary_wording_catalog();
            match catalog.render_boundary_explanation(entry_id) {
                Some(line) => println!("{line}"),
                None => return Err(format!("unknown entry id: {entry_id}").into()),
            }
        }
        Some("validate") => {
            for catalog in [
                seeded_boundary_wording_catalog(),
                seeded_boundary_wording_catalog_localized(),
                seeded_boundary_wording_catalog_offline_mirror(),
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

fn assert_valid(catalog: &BoundaryWordingCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
