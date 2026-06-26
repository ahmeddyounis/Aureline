//! Headless emitter for the error/recovery copy catalog and degraded-state
//! reason chips.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-recovery-copy-proof/` and the localized / offline-mirror
//! fixtures under `fixtures/content/m5-error-recovery-copy/`. Dynamic banners,
//! inline blockers, Project Doctor, CLI/help summaries, support exports, and
//! screenshot/demo captions resolve recovery copy through this catalog so a literal
//! failure string never becomes the source of truth.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_error_recovery_copy -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_error_recovery_copy -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_error_recovery_copy -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_error_recovery_copy -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_error_recovery_copy -- validate
//! ```

use aureline_shell::content::error_patterns::{
    seeded_error_recovery_copy_catalog, seeded_error_recovery_copy_catalog_localized,
    seeded_error_recovery_copy_catalog_offline_mirror, ErrorRecoveryCopyCatalog,
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
            let catalog = seeded_error_recovery_copy_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_error_recovery_copy_catalog().render_markdown_summary()
            );
        }
        Some("fixture-localized") => {
            let catalog = seeded_error_recovery_copy_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_error_recovery_copy_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("validate") => {
            for catalog in [
                seeded_error_recovery_copy_catalog(),
                seeded_error_recovery_copy_catalog_localized(),
                seeded_error_recovery_copy_catalog_offline_mirror(),
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

fn assert_valid(catalog: &ErrorRecoveryCopyCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
