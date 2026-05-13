//! Protected fixture checks for Git/review activity-center and support export.

use std::path::{Path, PathBuf};

use aureline_shell::activity_center::git_review::{
    GitReviewEventFamily, GitReviewEventSnapshot, GitReviewSupportExport,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

#[test]
fn git_review_activity_fixture_preserves_branch_target_action_identity() {
    let path = repo_root()
        .join("fixtures")
        .join("review")
        .join("git_activity_alpha")
        .join("git_review_activity_snapshot.json");
    let bytes = std::fs::read(&path).expect("read Git/review activity fixture");
    let snapshot: GitReviewEventSnapshot =
        serde_json::from_slice(&bytes).expect("parse Git/review activity fixture");

    assert_eq!(snapshot.events.len(), 3);
    assert!(snapshot.all_activity_rows_preserve_context());
    assert!(snapshot.publish_and_review_actions_have_exact_reopen());
    assert_eq!(snapshot.branch_target_action_complete_count, 3);
    assert_eq!(snapshot.exact_reopen_link_count, 4);
    assert!(snapshot.events.iter().any(|event| {
        event.event_family == GitReviewEventFamily::GitPublish
            && event.review_or_publish_has_exact_reopen()
    }));
    assert!(snapshot.events.iter().any(|event| {
        event.event_family == GitReviewEventFamily::ReviewWorkspace
            && event.review_or_publish_has_exact_reopen()
    }));
}

#[test]
fn git_review_support_fixture_exports_structured_rows() {
    let path = repo_root()
        .join("fixtures")
        .join("review")
        .join("git_activity_alpha")
        .join("support_export_git_review_events.json");
    let bytes = std::fs::read(&path).expect("read Git/review support fixture");
    let export: GitReviewSupportExport =
        serde_json::from_slice(&bytes).expect("parse Git/review support fixture");

    assert_eq!(export.row_count(), 3);
    assert_eq!(export.source_event_count, 3);
    assert!(export.raw_private_material_excluded);
    assert!(export.all_rows_preserve_branch_target_action_identity());
    assert!(export.rows.iter().any(|row| {
        row.event_family == GitReviewEventFamily::GitPublish
            && row
                .exact_reopen_links
                .iter()
                .any(|link| link.command_id == "cmd:git.publish.review.reopen")
    }));
}
