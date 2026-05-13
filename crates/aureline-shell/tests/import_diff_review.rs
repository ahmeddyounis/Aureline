//! Protected fixture checks for import diff review and retained reports.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_shell::import::{
    materialize_import_diff_review_packet, reopen_retained_migration_report,
    CompetitorConfigClassifier, ImportMappingClassification, ImportReportReopenSurface,
    ImportReviewDomain,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ImportDiffCase {
    case_id: String,
    source_fixture_ref: String,
    destination_workspace_target: String,
    required_domains: Vec<ImportReviewDomain>,
    required_classifications: Vec<ImportMappingClassification>,
    required_reopen_surfaces: Vec<ImportReportReopenSurface>,
    required_conflict_inspector_ref: String,
    acceptance: Acceptance,
}

#[derive(Debug, Deserialize)]
struct Acceptance {
    every_row_has_before_after_diff: bool,
    every_row_uses_one_checkpoint: bool,
    checkpoint_created_before_apply: bool,
    retained_report_survives_first_run: bool,
    shortcut_delta_report_reopenable: bool,
    lossy_and_unsupported_visible_after_apply: bool,
    shortcut_conflicts_visible_before_apply: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_case() -> ImportDiffCase {
    let path =
        repo_root().join("fixtures/migration/import_diff_cases/vscode_common_retained_report.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn import_diff_review_fixture_proves_checkpoint_report_and_shortcut_delta() {
    let case = load_case();
    let review = CompetitorConfigClassifier::new().build_review(
        repo_root().join(&case.source_fixture_ref),
        &case.destination_workspace_target,
    );
    let packet = materialize_import_diff_review_packet(&review);

    assert!(
        !packet.rows.is_empty(),
        "case {} produced no diff rows",
        case.case_id
    );
    assert_eq!(
        packet.every_row_has_before_after_diff(),
        case.acceptance.every_row_has_before_after_diff
    );
    assert_eq!(
        packet.every_row_uses_one_checkpoint(),
        case.acceptance.every_row_uses_one_checkpoint
    );
    assert_eq!(
        packet.rollback_checkpoint.clear_pre_apply_checkpoint(),
        case.acceptance.checkpoint_created_before_apply
    );
    assert_eq!(
        packet.retained_migration_report.retained_after_first_run,
        case.acceptance.retained_report_survives_first_run
    );
    assert_eq!(
        packet.caveats_are_retained(),
        case.acceptance.lossy_and_unsupported_visible_after_apply
    );

    let domains: BTreeSet<ImportReviewDomain> = packet.rows.iter().map(|row| row.domain).collect();
    for required in &case.required_domains {
        assert!(
            domains.contains(required),
            "missing required domain {required:?}"
        );
    }

    for required in &case.required_classifications {
        assert!(
            packet
                .retained_migration_report
                .classifications_present
                .contains(required),
            "missing required classification {required:?}"
        );
    }

    assert_eq!(
        !packet.shortcut_delta_report.rows.is_empty(),
        case.acceptance.shortcut_delta_report_reopenable
    );
    assert_eq!(
        packet
            .shortcut_delta_report
            .conflicts_visible_before_apply
            .iter()
            .all(|conflict| {
                conflict.visible_before_apply
                    && conflict.conflict_inspector_ref == case.required_conflict_inspector_ref
                    && !conflict.conflict_review_ref.trim().is_empty()
            }),
        case.acceptance.shortcut_conflicts_visible_before_apply
    );

    for surface in &case.required_reopen_surfaces {
        let projection = reopen_retained_migration_report(&packet, *surface)
            .unwrap_or_else(|| panic!("missing retained report projection for {surface:?}"));
        assert_eq!(
            projection.rollback_checkpoint_ref,
            packet.rollback_checkpoint.checkpoint_ref
        );
        assert_eq!(
            projection.shortcut_delta_report_ref,
            packet.shortcut_delta_report.shortcut_delta_report_id
        );
    }
}
