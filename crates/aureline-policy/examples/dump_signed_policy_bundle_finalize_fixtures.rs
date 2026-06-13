//! Emits the seeded signed-bundle finalize fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- page
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- rows
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- lifecycle-events
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- defects
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- summary
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- support-export
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-staleness-disguised-withdrawn
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-missing-import-flow-preview
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-no-affected-surfaces-beta
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-missing-bundle-kind-preview
//! cargo run -q -p aureline-policy --example dump_signed_policy_bundle_finalize_fixtures -- drill-missing-minimum-version-beta
//! ```

use aureline_auth::offline_entitlements::seeded_offline_entitlement_verifier_beta_page;
use aureline_policy::{
    seeded_finalize_signed_policy_bundle_page, BundleKindClass, BundleLifecycleEventClass,
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
        Some("lifecycle-events") => print_json(&page.lifecycle_events)?,
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
            let mut rows: Vec<FinalizeSignedPolicyBundleRow> = page.rows.clone();
            if let Some(row) = rows.iter_mut().find(|row| {
                row.bundle_kind == BundleKindClass::AdminPolicyBundle
                    && row.import_flow_token == "offline_grace"
            }) {
                row.grace_state.staleness_label.clear();
            }
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:staleness-disguised",
                "Drill - staleness disguised as auth failure",
                "2026-06-01T00:00:00Z",
                rows,
                page.lifecycle_events.clone(),
                seeded_offline_entitlement_verifier_beta_page(),
            );
            print_json(&drill)?;
        }
        Some("drill-missing-import-flow-preview") => {
            let rows: Vec<_> = page
                .rows
                .clone()
                .into_iter()
                .filter(|row| row.import_flow_token != "mirror")
                .collect();
            let lifecycle_events: Vec<_> = page
                .lifecycle_events
                .clone()
                .into_iter()
                .filter(|event| event.delivery_source_token != "mirror_publication")
                .collect();
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:missing-import-flow",
                "Drill - missing mirror import flow",
                "2026-06-01T00:00:00Z",
                rows,
                lifecycle_events,
                seeded_offline_entitlement_verifier_beta_page(),
            );
            print_json(&drill)?;
        }
        Some("drill-no-affected-surfaces-beta") => {
            let mut rows: Vec<FinalizeSignedPolicyBundleRow> = page.rows.clone();
            if let Some(row) = rows.first_mut() {
                row.simulation_packet.affected_surfaces.clear();
            }
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:no-affected-surfaces",
                "Drill - simulation packet missing affected surfaces",
                "2026-06-01T00:00:00Z",
                rows,
                page.lifecycle_events.clone(),
                seeded_offline_entitlement_verifier_beta_page(),
            );
            print_json(&drill)?;
        }
        Some("drill-missing-bundle-kind-preview") => {
            let rows: Vec<_> = page
                .rows
                .clone()
                .into_iter()
                .filter(|row| row.bundle_kind != BundleKindClass::TrustRootSignerUpdate)
                .collect();
            let lifecycle_events: Vec<_> = page
                .lifecycle_events
                .clone()
                .into_iter()
                .filter(|event| event.bundle_kind != BundleKindClass::TrustRootSignerUpdate)
                .collect();
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:missing-bundle-kind",
                "Drill - missing required bundle kind",
                "2026-06-01T00:00:00Z",
                rows,
                lifecycle_events,
                seeded_offline_entitlement_verifier_beta_page(),
            );
            print_json(&drill)?;
        }
        Some("drill-missing-minimum-version-beta") => {
            let mut rows: Vec<FinalizeSignedPolicyBundleRow> = page.rows.clone();
            if let Some(row) = rows.iter_mut().find(|row| {
                row.bundle_kind == BundleKindClass::EmergencyDisableBundle
                    && row.import_flow_token == "manual_import"
            }) {
                row.envelope_review.required_minimum_version.clear();
            }
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:missing-minimum-version",
                "Drill - emergency disable missing minimum version",
                "2026-06-01T00:00:00Z",
                rows,
                page.lifecycle_events.clone(),
                seeded_offline_entitlement_verifier_beta_page(),
            );
            print_json(&drill)?;
        }
        Some("drill-missing-lifecycle-preview") => {
            let lifecycle_events: Vec<_> = page
                .lifecycle_events
                .clone()
                .into_iter()
                .filter(|event| {
                    event.event_class != BundleLifecycleEventClass::SignerRotationReview
                })
                .collect();
            let drill = FinalizeSignedPolicyBundlePage::new(
                "policy:signed-policy-bundle-finalize:drill:missing-lifecycle",
                "Drill - missing signer rotation lifecycle event",
                "2026-06-01T00:00:00Z",
                page.rows.clone(),
                lifecycle_events,
                seeded_offline_entitlement_verifier_beta_page(),
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
