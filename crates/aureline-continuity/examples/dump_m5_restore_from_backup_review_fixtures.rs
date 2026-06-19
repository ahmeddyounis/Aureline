//! Emits the canonical restore-from-backup review continuity fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- registry
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-full-normal-status-overclaim-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-restore-lane-conflated-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-privileged-lane-auto-replayed-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-affected-slice-unnamed-beta
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-replay-fence-review-missing-beta
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-compare-parity-missing-preview
//! cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures -- case-review-evidence-missing-preview
//! ```

use aureline_continuity::{
    seeded_restore_review_input, seeded_restore_review_page, AffectedSliceClass,
    ReplayFenceStateClass, RestoreReviewEntry, RestoreReviewInput, RestoreReviewPage,
    RestoreReviewSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_restore_review_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("registry") => print_json(&page.registry)?,
        Some("support-export") => {
            let export = RestoreReviewSupportExport::from_page(
                "continuity:restore-from-backup-review:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-full-normal-status-overclaim-withdrawn") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:policy-bundle", |review| {
                review.identity_summary.asserts_full_normal_status = true;
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:full-normal-status-overclaim",
                "Case - a narrower-than-normal restore claims full normal status (withdrawn)",
                input,
            ))?;
        }
        Some("case-restore-lane-conflated-withdrawn") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:managed-records", |review| {
                review.restore_lane =
                    aureline_continuity::RestoreLaneClass::OrdinaryWorkspaceRestore;
                review.restore_lane_token =
                    aureline_continuity::RestoreLaneClass::OrdinaryWorkspaceRestore
                        .as_str()
                        .to_owned();
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:restore-lane-conflated",
                "Case - a managed row presents an ordinary workspace restore as continuity restore (withdrawn)",
                input,
            ))?;
        }
        Some("case-privileged-lane-auto-replayed-withdrawn") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:policy-bundle", |review| {
                let fence = &mut review.replay_fences[0];
                fence.fence_state = ReplayFenceStateClass::NoFenceLocalSafe;
                fence.fence_state_token =
                    ReplayFenceStateClass::NoFenceLocalSafe.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:privileged-lane-auto-replayed",
                "Case - a privileged lane is left unfenced and would auto-replay (withdrawn)",
                input,
            ))?;
        }
        Some("case-affected-slice-unnamed-beta") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:policy-bundle", |review| {
                review.identity_summary.affected_slice = AffectedSliceClass::NoneNarrowed;
                review.identity_summary.affected_slice_token =
                    AffectedSliceClass::NoneNarrowed.as_str().to_owned();
                review.identity_summary.affected_slice_note = String::new();
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:affected-slice-unnamed",
                "Case - a narrower-than-normal restore does not name the affected slice (beta)",
                input,
            ))?;
        }
        Some("case-replay-fence-review-missing-beta") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:policy-bundle", |review| {
                review.replay_fences[0].reviewed_step_ref = String::new();
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:replay-fence-review-missing",
                "Case - a cleared fence names no explicit reviewed step (beta)",
                input,
            ))?;
        }
        Some("case-compare-parity-missing-preview") => {
            let mut input = seeded_restore_review_input();
            with_review(&mut input, "continuity-restore:managed-records", |review| {
                review.compare_export.restored_vs_current_available = false;
                review.compare_export.compare_ref = String::new();
            });
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:compare-parity-missing",
                "Case - a managed-continuity review cannot compare restored-vs-current (preview)",
                input,
            ))?;
        }
        Some("case-review-evidence-missing-preview") => {
            let mut input = seeded_restore_review_input();
            input
                .reviews
                .retain(|review| review.review_id != "continuity-restore:sync-metadata");
            print_json(&case_page(
                "continuity:restore-from-backup-review:case:review-evidence-missing",
                "Case - a claimed restored row carries no restore review (preview)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_review(
    input: &mut RestoreReviewInput,
    review_id: &str,
    mutate: impl FnOnce(&mut RestoreReviewEntry),
) {
    let review = input
        .reviews
        .iter_mut()
        .find(|review| review.review_id == review_id)
        .unwrap_or_else(|| panic!("missing seeded review: {review_id}"));
    mutate(review);
}

fn case_page(page_id: &str, page_label: &str, input: RestoreReviewInput) -> RestoreReviewPage {
    RestoreReviewPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
