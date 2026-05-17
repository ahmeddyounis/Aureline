//! Headless inspector for the beta passkey-capable step-up, reauth, and
//! recovery lane projection.
//!
//! The bin emits the same audited records consumed by the live shell, by the
//! support-export wrapper, and by the integration test that replays the
//! checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- support-rows
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- validate
//! ```

use aureline_shell::passkey_step_up_beta::{
    audit_passkey_step_up_beta_rows, seeded_passkey_step_up_beta_page,
    validate_passkey_step_up_beta_page, PasskeyAuthorityScopeClass, PasskeyBetaLaneClass,
    PasskeyFallbackClass, PasskeyStepUpBetaPage, PasskeyStepUpBetaRenderSummary,
    PasskeyStepUpBetaSupportExport, PasskeyTargetActionPreservationClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_passkey_step_up_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("support-rows") => print_json(&page.support_rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = PasskeyStepUpBetaSupportExport::from_page(
                "support-export:passkey-step-up-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = PasskeyStepUpBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-recovery-rerouted") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::RecoveryLane.as_str())
                .ok_or("seeded page must include a recovery row")?;
            row.target_action_preservation.preservation_token =
                PasskeyTargetActionPreservationClass::TargetActionRerouted
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-reauth-widened") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::ReauthLane.as_str())
                .ok_or("seeded page must include a reauth row")?;
            row.target_action_preservation.preservation_token =
                PasskeyTargetActionPreservationClass::TargetActionWidened
                    .as_str()
                    .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-reauth-without-fallback") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::ReauthLane.as_str())
                .ok_or("seeded page must include a reauth row")?;
            row.outcome.fallback_token =
                PasskeyFallbackClass::NoFallbackRequired.as_str().to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-step-up-widens-authority") => {
            let mut page = page;
            let row = page
                .rows
                .iter_mut()
                .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::StepUpLane.as_str())
                .ok_or("seeded page must include a step-up row")?;
            row.granted_authority_scope_token = PasskeyAuthorityScopeClass::TenantAdminScope
                .as_str()
                .to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_passkey_step_up_beta_page(&page) {
            Ok(()) => println!("ok"),
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
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn rebuild_with_defects(mut page: PasskeyStepUpBetaPage) -> PasskeyStepUpBetaPage {
    page.defects = audit_passkey_step_up_beta_rows(&page.rows, &page.support_rows);
    page.summary.defect_count = page.defects.len();
    page
}
