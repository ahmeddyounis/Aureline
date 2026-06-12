//! Emits the seeded managed-workspace lifecycle fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-missing-state-flagged
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-raw-material-withheld
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-continuity-overclaim-narrowed
//! cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-expiry-no-local-safe-narrowed
//! ```

use aureline_remote::{
    seeded_lifecycle_snapshot, seeded_managed_workspace_lifecycle_page, CaveatClass,
    ContinuityClass, LifecycleSupportExport, ManagedWorkspaceLifecyclePage, ProvenanceClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_managed_workspace_lifecycle_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = LifecycleSupportExport::from_page(
                "remote:managed-workspace-lifecycle:support-export:fixture-001",
                "2026-06-11T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-state-flagged") => {
            let mut snapshot = seeded_lifecycle_snapshot();
            snapshot
                .records
                .retain(|r| r.lifecycle_state_token != "resumed");
            let drill_page = ManagedWorkspaceLifecyclePage::new(
                "remote:managed-workspace-lifecycle:drill:missing-state",
                "Drill — required lifecycle state absent (flagged)",
                "2026-06-11T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-raw-material-withheld") => {
            let mut snapshot = seeded_lifecycle_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.lifecycle_state_token == "ready" {
                    rec.raw_private_material_excluded = false;
                }
            }
            let drill_page = ManagedWorkspaceLifecyclePage::new(
                "remote:managed-workspace-lifecycle:drill:raw-material-withheld",
                "Drill — raw private material exposed (withheld)",
                "2026-06-11T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-continuity-overclaim-narrowed") => {
            let mut snapshot = seeded_lifecycle_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.lifecycle_state_token == "resumed" {
                    rec.image_provenance = ProvenanceClass::SuccessorImage;
                    rec.image_provenance_token =
                        ProvenanceClass::SuccessorImage.as_str().to_owned();
                    rec.provenance_changed = true;
                    rec.continuity_class = ContinuityClass::ExactContinuity;
                    rec.continuity_class_token =
                        ContinuityClass::ExactContinuity.as_str().to_owned();
                    rec.caveat_history = vec![CaveatClass::ImageChanged];
                }
            }
            let drill_page = ManagedWorkspaceLifecyclePage::new(
                "remote:managed-workspace-lifecycle:drill:continuity-overclaim",
                "Drill — resume claims exact continuity over a changed image (narrowed)",
                "2026-06-11T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-expiry-no-local-safe-narrowed") => {
            let mut snapshot = seeded_lifecycle_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.lifecycle_state_token == "expired" {
                    rec.local_safe_continuation_available = false;
                    rec.recovery_options.clear();
                }
            }
            let drill_page = ManagedWorkspaceLifecyclePage::new(
                "remote:managed-workspace-lifecycle:drill:expiry-no-local-safe",
                "Drill — expiry offers no local-safe continuation (narrowed)",
                "2026-06-11T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
