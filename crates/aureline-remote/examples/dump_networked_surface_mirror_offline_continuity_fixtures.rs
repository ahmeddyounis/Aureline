//! Emits the seeded networked-surface mirror/offline continuity fixtures.
//!
//! This is the CLI/headless consumer of the mirror/offline continuity layer:
//! the same per-family route handling, stale-mirror warnings, and
//! public-fallback rules the product surfaces show are emitted here as
//! machine-readable JSON, and the CLI continuity view (`continuity-cli`) renders
//! each record through the shared field catalog so headless output quotes
//! identical route-handling tokens and field names.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- records
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- continuity-cli
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-missing-family-preview
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-silent-fallback-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-non-idempotent-withdrawn
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-blocked-no-reason-beta
//! cargo run -q -p aureline-remote --example dump_networked_surface_mirror_offline_continuity_fixtures -- drill-stale-mirror-beta
//! ```

use aureline_remote::{
    seeded_mirror_offline_continuity_page, seeded_mirror_offline_continuity_snapshot,
    ArtifactFamilyClass, ContinuityRouteClass, MirrorOfflineContinuityPage,
    MirrorOfflineContinuitySupportExport, StaleMirrorWarningClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_mirror_offline_continuity_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("records") => print_json(&page.continuity_snapshot.records)?,
        Some("support-export") => {
            let export = MirrorOfflineContinuitySupportExport::from_page(
                "remote:networked_surface_mirror_offline_continuity:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("continuity-cli") => {
            // Headless continuity view: print the shared field catalog as
            // `key=value` lines per family, proving CLI/support/product parity.
            for record in &page.continuity_snapshot.records {
                println!(
                    "# {} ({})",
                    record.artifact_family_label, record.continuity_route_token
                );
                for line in record.render_cli_lines() {
                    println!("{line}");
                }
                println!();
            }
        }
        Some("drill-missing-family-preview") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            snapshot
                .records
                .retain(|r| r.artifact_family != ArtifactFamilyClass::DocsPack);
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:missing-family",
                "Drill — required artifact family absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.artifact_family == ArtifactFamilyClass::Registry {
                    r.raw_private_material_excluded = false;
                }
            }
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:raw-material",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-silent-fallback-withdrawn") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.artifact_family == ArtifactFamilyClass::Registry {
                    // A mirror-only profile flips to a public-direct route — a
                    // silent public fall-through.
                    r.continuity_route = ContinuityRouteClass::PublicDirect;
                    r.continuity_route_token =
                        ContinuityRouteClass::PublicDirect.as_str().to_owned();
                }
            }
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:silent-fallback",
                "Drill — mirror-only silent public fall-through (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-non-idempotent-withdrawn") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.artifact_family == ArtifactFamilyClass::CompanionHandoff {
                    r.action_is_idempotent = false;
                }
            }
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:non-idempotent",
                "Drill — non-idempotent action queued for replay (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-blocked-no-reason-beta") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.artifact_family == ArtifactFamilyClass::ModelPack {
                    r.denial_reason = None;
                    r.denial_reason_token = String::new();
                }
            }
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:blocked-no-reason",
                "Drill — blocked without a typed reason (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill)?;
        }
        Some("drill-stale-mirror-beta") => {
            let mut snapshot = seeded_mirror_offline_continuity_snapshot();
            for r in snapshot.records.iter_mut() {
                if r.artifact_family == ArtifactFamilyClass::Registry {
                    r.stale_mirror_warning = StaleMirrorWarningClass::StaleBeyondGrace;
                    r.stale_mirror_warning_token = StaleMirrorWarningClass::StaleBeyondGrace
                        .as_str()
                        .to_owned();
                }
            }
            let drill = MirrorOfflineContinuityPage::new(
                "remote:networked_surface_mirror_offline_continuity:drill:stale-mirror",
                "Drill — stale mirror served beyond grace (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
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
