//! Fixture-driven tests for admin delete, legal-hold, chronology, and policy diff truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_shell::admin_alpha::{
    AdminAlphaInput, AdminAlphaInspector, AdminAlphaPacket, AdminAlphaResultClass,
    ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
};
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{DiagnosticDataClass, ExactBuildCapture, ReleaseChannelClass};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/admin/delete_hold_policy_alpha")
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:admin-alpha",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

#[test]
fn admin_delete_hold_policy_fixture_projects_required_truth() {
    let manifest = load_manifest();
    assert_eq!(manifest.schema_version, 1);

    let inspector = AdminAlphaInspector::new();
    for case in manifest.cases {
        let fixture = load_case(&case.fixture_ref);
        assert_eq!(fixture.case_id, case.case_id);

        let packet = inspector
            .inspect(fixture.input)
            .unwrap_or_else(|err| panic!("case {} failed: {err}", case.case_id));

        assert!(packet.all_chronology_is_timezone_aware());
        assert!(packet.all_delete_rows_surface_required_honesty());
        assert!(packet.has_result_vocabulary_floor());
        assert!(packet.support_export_preserves_result_vocabulary());
        assert!(packet.has_durable_destruction_receipt());
        assert!(packet
            .policy_diff_preview
            .is_pre_apply_preview_for_current_source());
        assert_eq!(
            packet.support_export.raw_payloads_excluded,
            fixture.expect.raw_payloads_excluded
        );
        assert!(
            packet.delete_review_rows.len() >= fixture.expect.min_delete_rows,
            "case {} delete row count",
            case.case_id
        );

        assert_required_results(&case, &packet);
        assert_required_archive_search_postures(&case, &packet);
        assert_required_receipt_postures(&case, &packet);
        assert_required_policy_diffs(&case, &packet);
        assert_required_sources(&case, &packet);
        assert_support_export_counts_match_rows(&packet);
    }
}

#[test]
fn support_seed_consumes_admin_delete_hold_policy_export() {
    let inspector = AdminAlphaInspector::new();
    let fixture =
        load_case("fixtures/admin/delete_hold_policy_alpha/support_offboarding_delete_review.yaml");
    let packet = inspector
        .inspect(fixture.input)
        .expect("admin packet builds");

    let surface = SupportSeedSurface::admin_delete_hold_policy_preview(
        fixture_capture(),
        "2026-05-14T09:20:00Z",
        &packet.support_export,
    )
    .expect("support preview builds");

    assert!(surface.has_exact_build_identity());
    assert_eq!(surface.preview_row_count(), 3);

    let admin_row = surface
        .preview
        .manifest
        .preview_items
        .iter()
        .find(|item| {
            item.parity_binding.support_pack_item_id
                == "support.item.admin_delete_hold_policy_truth"
        })
        .expect("admin delete/hold support row");

    assert_eq!(
        admin_row.file_section_identity.artifact_kind_class,
        ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(
        admin_row.redaction.data_class,
        DiagnosticDataClass::MetadataOnly
    );
    assert!(admin_row
        .file_section_identity
        .source_refs
        .iter()
        .any(|item| item == "docs/admin/policy_diff_alpha.md"));
}

#[test]
fn blocked_hold_without_hold_ref_is_rejected() {
    let mut fixture =
        load_case("fixtures/admin/delete_hold_policy_alpha/support_offboarding_delete_review.yaml");
    let row = fixture.input.delete_flows[0]
        .rows
        .iter_mut()
        .find(|row| row.result_class == AdminAlphaResultClass::BlockedByHold)
        .expect("blocked by hold row");
    row.hold_scope.matched_hold_refs.clear();

    let err = AdminAlphaInspector::new()
        .inspect(fixture.input)
        .unwrap_err();
    assert!(err
        .to_string()
        .contains("blocked_by_hold rows must cite a matched hold"));
}

#[test]
fn stale_policy_diff_preview_is_rejected() {
    let mut fixture =
        load_case("fixtures/admin/delete_hold_policy_alpha/support_offboarding_delete_review.yaml");
    fixture.input.policy_diff_preview.apply_requires_preview_ack = false;

    let err = AdminAlphaInspector::new()
        .inspect(fixture.input)
        .unwrap_err();
    assert!(err.to_string().contains("previewed before apply"));
}

fn assert_required_results(case: &ManifestCase, packet: &AdminAlphaPacket) {
    let observed: BTreeSet<_> = packet
        .delete_review_rows
        .iter()
        .map(|row| row.result_class.as_str())
        .collect();
    for required in &case.required_result_classes {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing result class {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_archive_search_postures(case: &ManifestCase, packet: &AdminAlphaPacket) {
    let observed: BTreeSet<_> = packet
        .delete_review_rows
        .iter()
        .map(|row| row.archive_search.posture_class.as_str())
        .collect();
    for required in &case.required_archive_search_postures {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing archive-search posture {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_receipt_postures(case: &ManifestCase, packet: &AdminAlphaPacket) {
    let observed: BTreeSet<_> = packet
        .delete_review_rows
        .iter()
        .map(|row| row.destruction_receipt.availability_class.as_str())
        .collect();
    for required in &case.required_receipt_postures {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing receipt posture {}",
            case.case_id,
            required
        );
    }
}

fn assert_required_policy_diffs(case: &ManifestCase, packet: &AdminAlphaPacket) {
    for required in &case.required_policy_diff_refs {
        assert_eq!(&packet.policy_diff_preview.diff_id, required);
        assert!(packet.delete_review_rows.iter().any(|row| row
            .linked_policy_diff_refs
            .iter()
            .any(|item| item == required)));
    }
}

fn assert_required_sources(case: &ManifestCase, packet: &AdminAlphaPacket) {
    let observed: BTreeSet<_> = packet.source_refs.iter().map(String::as_str).collect();
    for required in &case.required_source_refs {
        assert!(
            observed.contains(required.as_str()),
            "case {} missing source ref {}",
            case.case_id,
            required
        );
    }
}

fn assert_support_export_counts_match_rows(packet: &AdminAlphaPacket) {
    for result in AdminAlphaResultClass::vocabulary() {
        let expected = packet
            .delete_review_rows
            .iter()
            .filter(|row| row.result_class == result)
            .count();
        let actual = packet
            .support_export
            .result_counts
            .iter()
            .find(|count| count.result_class == result)
            .map(|count| count.row_count)
            .unwrap_or_default();
        assert_eq!(actual, expected, "count mismatch for {}", result.as_str());
    }
}

fn load_manifest() -> FixtureManifest {
    let path = fixture_root().join("manifest.yaml");
    load_yaml(&path)
}

fn load_case(path: &str) -> AdminAlphaFixture {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    load_yaml(&repo_root.join(path))
}

fn load_yaml<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let payload = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    cases: Vec<ManifestCase>,
}

#[derive(Debug, Deserialize)]
struct ManifestCase {
    case_id: String,
    fixture_ref: String,
    required_result_classes: Vec<String>,
    required_archive_search_postures: Vec<String>,
    required_receipt_postures: Vec<String>,
    required_policy_diff_refs: Vec<String>,
    required_source_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AdminAlphaFixture {
    case_id: String,
    input: AdminAlphaInput,
    expect: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    min_delete_rows: usize,
    raw_payloads_excluded: bool,
}
