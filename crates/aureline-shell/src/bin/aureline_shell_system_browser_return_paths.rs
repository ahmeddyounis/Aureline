//! Headless inspector for the beta system-browser default + passkey step-up +
//! return-path labeling projection.
//!
//! The bin emits the same audited records consumed by the live shell, by the
//! support-export wrapper, and by the integration test that replays the
//! checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- validate
//! ```

use aureline_shell::system_browser_return_paths::{
    audit_system_browser_return_paths_beta_rows, seeded_system_browser_return_paths_beta_page,
    validate_system_browser_return_paths_beta_page, AuthorityScopeClass, PasskeyStepUpPostureClass,
    SystemBrowserPolicyExceptionClass, SystemBrowserReturnPathsBetaPage,
    SystemBrowserReturnPathsBetaSupportExport, SystemBrowserReturnPathsRenderSummary,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_system_browser_return_paths_beta_page();

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
            let export = SystemBrowserReturnPathsBetaSupportExport::from_page(
                "support-export:system-browser-return-paths-beta:001",
                "2026-05-15T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = SystemBrowserReturnPathsRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-widening-scope") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.passkey_capability_claimed)
                .ok_or("seeded page must include a passkey-capable row")?;
            row.granted_authority_scope_token =
                AuthorityScopeClass::TenantAdminScope.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-passkey-no-fallback") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.passkey_capability_claimed)
                .ok_or("seeded page must include a passkey-capable row")?;
            row.passkey_step_up.posture_token =
                PasskeyStepUpPostureClass::PasskeyUnavailableWithFallback
                    .as_str()
                    .to_owned();
            row.passkey_step_up.fallback_retry_path_token = None;
            row.passkey_step_up.fallback_retry_path_label = None;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-system-browser-not-default") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.passkey_capability_claimed)
                .ok_or("seeded page must include a passkey-capable row")?;
            row.system_browser_default = false;
            row.policy_exception_token =
                SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException
                    .as_str()
                    .to_owned();
            row.default_action_token = "use_device_code".to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_system_browser_return_paths_beta_page(&page) {
            Ok(()) => {
                println!("ok");
            }
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row_id={} field={} note={}",
                        defect.defect_kind_token, defect.row_id, defect.field, defect.note,
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

/// Rebuild a mutated page so its `defects` array is freshly recomputed by the
/// validator instead of being inherited from the seeded source.
fn rebuild_with_defects(mut page: SystemBrowserReturnPathsBetaPage) -> SystemBrowserReturnPathsBetaPage {
    page.defects = audit_system_browser_return_paths_beta_rows(&page.rows, &page.support_rows);
    page.summary.defect_count = page.defects.len();
    page
}
