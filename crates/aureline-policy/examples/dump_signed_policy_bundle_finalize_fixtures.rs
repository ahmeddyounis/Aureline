//! Emits the seeded signed-policy-bundle finalize fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-staleness-disguised-withdrawn
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-missing-import-flow-preview
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-no-affected-surfaces-beta
//! ```

use aureline_auth::offline_entitlements::seeded_offline_entitlement_verifier_beta_page;
use aureline_policy::{
    seeded_finalize_signed_policy_bundle_page, BundleImportFlowClass, BundleKindClass,
    FinalizeSignedPolicyBundlePage, FinalizeSignedPolicyBundleRow,
    FinalizeSignedPolicyBundleSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_finalize_signed_policy_bundle_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = FinalizeSignedPolicyBundleSupportExport::from_page(
                "policy:signed-policy-bundle-finalize:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-staleness-disguised-withdrawn") => {
            // Clears staleness label on the offline_grace policy row to trigger
            // the staleness_disguised_as_auth_failure withdrawal guardrail.
            let verifier_page = seeded_offline_entitlement_verifier_beta_page();
            let mut rows: Vec<FinalizeSignedPolicyBundleRow> = page.rows;
            if let Some(row) = rows.iter_mut().find(|r| {
                r.import_flow == BundleImportFlowClass::OfflineGrace
                    && r.bundle_kind == BundleKindClass::PolicyBundle
            }) {
                row.grace_state.staleness_label.clear();
            }
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:staleness-disguised",
                "Drill — staleness disguised as auth failure (withdrawn)",
                "2026-06-01T00:00:00Z",
                rows,
                verifier_page,
            );
            print_json(&drill)?;
        }
        Some("drill-missing-import-flow-preview") => {
            // Removes all mirror and air-gapped rows to trigger import-flow coverage gap.
            let verifier_page = seeded_offline_entitlement_verifier_beta_page();
            let rows: Vec<FinalizeSignedPolicyBundleRow> = page
                .rows
                .into_iter()
                .filter(|r| {
                    r.import_flow != BundleImportFlowClass::Mirror
                        && r.import_flow != BundleImportFlowClass::AirGapped
                })
                .collect();
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:missing-import-flow",
                "Drill — required import flow missing (preview)",
                "2026-06-01T00:00:00Z",
                rows,
                verifier_page,
            );
            print_json(&drill)?;
        }
        Some("drill-no-affected-surfaces-beta") => {
            // Clears affected_surfaces on the first row's simulation packet to
            // trigger simulation_packet_missing_before_apply beta narrowing.
            let verifier_page = seeded_offline_entitlement_verifier_beta_page();
            let mut rows: Vec<FinalizeSignedPolicyBundleRow> = page.rows;
            if let Some(row) = rows.first_mut() {
                row.simulation_packet.affected_surfaces.clear();
            }
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:no-affected-surfaces",
                "Drill — simulation packet missing affected surfaces (beta)",
                "2026-06-01T00:00:00Z",
                rows,
                verifier_page,
            );
            print_json(&drill)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
