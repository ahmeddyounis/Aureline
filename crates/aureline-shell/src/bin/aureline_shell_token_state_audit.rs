//! Headless inspector for the beta token / state / density / motion / theme audit.
//!
//! The bin emits the same audited records consumed by the live shell,
//! by the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- validate
//! ```

use aureline_shell::token_state_audit::{
    seeded_token_state_audit_page, validate_token_state_audit_page, TokenStateAuditSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_token_state_audit_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("rows") => {
            print_json(&page.rows)?;
        }
        Some("defects") => {
            print_json(&page.defects)?;
        }
        Some("support-export") => {
            let export = TokenStateAuditSupportExport::from_page(
                "support-export:token-state-audit-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_token_state_audit_page(&page) {
            Ok(()) => {
                println!("ok");
            }
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} surface={} row_id={} field={} note={}",
                        defect.defect_kind_token,
                        defect.surface.as_str(),
                        defect.row_id,
                        defect.field,
                        defect.note,
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
