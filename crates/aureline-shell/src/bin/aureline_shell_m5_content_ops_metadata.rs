//! Headless emitter for the content-ops metadata catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-content-ops-proof/` and the localized / offline-mirror
//! fixtures under `fixtures/content/m5-content-ops-metadata/`. Docs/help, release
//! notes, support exports, and the screenshot/demo pipeline resolve content-ops
//! provenance — source, command, version, build, placeholder semantics, and locale
//! fallback posture — through this catalog rather than maintaining parallel,
//! uncited, versionless captions and headings.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- provenance "<entry id>"
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_content_ops_metadata -- validate
//! ```

use aureline_shell::content::content_ops_metadata::{
    seeded_content_ops_metadata_catalog, seeded_content_ops_metadata_catalog_localized,
    seeded_content_ops_metadata_catalog_offline_mirror, ContentOpsMetadataCatalog,
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
            let catalog = seeded_content_ops_metadata_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_content_ops_metadata_catalog().render_markdown_summary()
            );
        }
        Some("fixture-localized") => {
            let catalog = seeded_content_ops_metadata_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_content_ops_metadata_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("provenance") => {
            let entry_id = args.get(1).map(String::as_str).unwrap_or_default();
            let catalog = seeded_content_ops_metadata_catalog();
            match catalog.render_provenance(entry_id) {
                Some(line) => println!("{line}"),
                None => return Err(format!("unknown entry id: {entry_id}").into()),
            }
        }
        Some("validate") => {
            for catalog in [
                seeded_content_ops_metadata_catalog(),
                seeded_content_ops_metadata_catalog_localized(),
                seeded_content_ops_metadata_catalog_offline_mirror(),
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

fn assert_valid(catalog: &ContentOpsMetadataCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
