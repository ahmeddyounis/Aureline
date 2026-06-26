//! Headless emitter for the action-label and count/scope-language parity catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-action-label-proof/` and the localized / offline-mirror
//! fixtures under `fixtures/content/m5-action-label-scope/`. UI, CLI/help, docs,
//! support exports, and narrated/activity surfaces resolve action labels and
//! count/scope phrases through this catalog so a literal scope word never becomes the
//! source of truth and no primary action hides its scope behind a vague verb.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_action_label_scope -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_action_label_scope -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_action_label_scope -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_action_label_scope -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_action_label_scope -- validate
//! ```

use aureline_shell::m5_action_label_scope_parity::{
    seeded_action_label_scope_catalog, seeded_action_label_scope_catalog_localized,
    seeded_action_label_scope_catalog_offline_mirror, ActionLabelScopeCatalog,
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
            let catalog = seeded_action_label_scope_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_action_label_scope_catalog().render_markdown_summary()
            );
        }
        Some("fixture-localized") => {
            let catalog = seeded_action_label_scope_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_action_label_scope_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("validate") => {
            for catalog in [
                seeded_action_label_scope_catalog(),
                seeded_action_label_scope_catalog_localized(),
                seeded_action_label_scope_catalog_offline_mirror(),
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

fn assert_valid(catalog: &ActionLabelScopeCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
