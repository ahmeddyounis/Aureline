//! Headless inspector for the beta embedded-surface boundary audit.
//!
//! The bin emits the same audited records consumed by the live shell,
//! by the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_audit -- validate
//! ```

use aureline_shell::embedded_boundary_audit::{
    seeded_embedded_boundary_audit_page, validate_embedded_boundary_audit_page,
    EmbeddedBoundaryAuditSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_embedded_boundary_audit_page();

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
        Some("defects") => {
            print_json(&page.defects)?;
        }
        Some("support-export") => {
            let export = EmbeddedBoundaryAuditSupportExport::from_page(
                "support-export:embedded-boundary-audit-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_embedded_boundary_audit_page(&page) {
            Ok(()) => {
                println!("ok");
            }
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} surface={} row_id={} field={} note={}",
                        defect.defect_kind_token,
                        defect.surface_family_token,
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
