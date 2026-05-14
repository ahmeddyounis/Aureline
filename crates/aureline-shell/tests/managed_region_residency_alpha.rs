//! Fixture-driven tests for managed/provider-linked region and key truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_shell::managed_truth::{
    ManagedTruthClaimClass, ManagedTruthSnapshot, ManagedTruthValidationError,
    SovereigntyBoundaryClass, MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND,
};
use aureline_shell::support_seed::SupportSeedSurface;
use aureline_support::bundle::{DiagnosticDataClass, ExactBuildCapture, ReleaseChannelClass};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/managed/region_residency_alpha")
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:managed-truth-alpha",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

#[test]
fn managed_region_residency_fixture_projects_required_truth() {
    let manifest = load_manifest();
    assert_eq!(manifest.status, "protected");
    assert_eq!(manifest.schema_version, 1);

    for case in manifest.case_files {
        let fixture = load_case(&case.file);
        assert_eq!(fixture.record_kind, case.record_kind);
        assert_eq!(fixture.schema_version, 1);
        fixture.snapshot.validate().expect("snapshot validates");
        assert_eq!(fixture.snapshot.rows.len(), fixture.expect.row_count);
        assert!(fixture.snapshot.has_acceptance_coverage());
        assert!(fixture.snapshot.covers_managed_and_provider_rows());
        assert!(fixture.snapshot.covers_plane_impairment_split());
        assert!(fixture.snapshot.all_claimed_rows_disclose_boundary_truth());
        assert!(fixture
            .snapshot
            .provider_rows_do_not_overclaim_sovereignty());

        let claim_tokens: BTreeSet<_> = fixture
            .snapshot
            .claim_classes()
            .into_iter()
            .map(ManagedTruthClaimClass::as_str)
            .collect();
        for required in &fixture.expect.required_claim_classes {
            assert!(
                claim_tokens.contains(required.as_str()),
                "missing claim class {required}"
            );
        }

        let display_rows = fixture.snapshot.display_rows();
        assert_eq!(display_rows.len(), fixture.expect.row_count);
        for row in &display_rows {
            assert!(!row.region_label.trim().is_empty());
            assert!(!row.tenant_label.trim().is_empty());
            assert!(!row.storage_label.trim().is_empty());
            assert!(!row.copy_label.trim().is_empty());
            assert!(!row.key_mode_label.trim().is_empty());
            assert!(!row.control_plane_label.trim().is_empty());
            assert!(!row.data_plane_label.trim().is_empty());
            assert!(!row.primary_action.trim().is_empty());
        }

        let plane_split = fixture
            .snapshot
            .rows
            .iter()
            .find(|row| row.row_id == fixture.expect.plane_split_row_id)
            .expect("fixture has plane split row");
        assert_ne!(
            plane_split.planes.control_plane_state,
            plane_split.planes.data_plane_state
        );
        assert!(!plane_split.planes.affected_action_families.is_empty());

        let provider = fixture
            .snapshot
            .rows
            .iter()
            .find(|row| row.row_id == fixture.expect.provider_row_id)
            .expect("fixture has provider row");
        assert_eq!(
            provider.sovereignty.sovereignty_boundary,
            SovereigntyBoundaryClass::ProviderDefaultDisclosed
        );
        assert!(!provider.display_copy.stronger_sovereignty_boundary_implied);

        let export = fixture.snapshot.export_packet();
        assert!(export.is_export_safe());
        assert_eq!(export.rows.len(), fixture.expect.row_count);

        let rendered = fixture.snapshot.render_plaintext();
        for required in &fixture.expect.required_region_scopes {
            assert!(rendered.contains(required));
        }
        for required in &fixture.expect.required_key_modes {
            assert!(rendered.contains(required));
        }
        assert!(rendered.contains("planes: control=unavailable data=healthy"));
    }
}

#[test]
fn support_seed_consumes_managed_truth_export_packet() {
    let fixture = load_case("claimed_managed_provider_boundary.yaml");
    fixture.snapshot.validate().expect("snapshot validates");

    let surface = SupportSeedSurface::managed_truth_preview(
        fixture_capture(),
        "2026-05-14T01:35:00Z",
        &fixture.snapshot,
    )
    .expect("support preview builds");

    assert!(surface.has_exact_build_identity());
    assert_eq!(surface.preview_row_count(), 3);

    let managed_row = surface
        .preview
        .manifest
        .preview_items
        .iter()
        .find(|item| {
            item.parity_binding.support_pack_item_id == fixture.expect.support_pack_item_id
        })
        .expect("managed truth support row");

    assert_eq!(
        managed_row.file_section_identity.artifact_kind_class,
        MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND
    );
    assert_eq!(
        managed_row.redaction.data_class,
        DiagnosticDataClass::MetadataOnly
    );
    assert!(managed_row
        .file_section_identity
        .source_refs
        .iter()
        .any(|item| item == "docs/managed/region_residency_alpha.md"));
}

#[test]
fn provider_linked_sovereignty_overclaim_is_rejected() {
    let mut fixture = load_case("claimed_managed_provider_boundary.yaml");
    let provider = fixture
        .snapshot
        .rows
        .iter_mut()
        .find(|row| row.row_id == fixture.expect.provider_row_id)
        .expect("fixture has provider row");
    provider.sovereignty.sovereignty_boundary = SovereigntyBoundaryClass::RegulatedJurisdiction;

    let err = fixture.snapshot.validate().unwrap_err();
    assert!(matches!(
        err,
        ManagedTruthValidationError::ProviderLinkedOverclaimsSovereignty { .. }
    ));
}

#[test]
fn plane_impairment_without_action_scope_is_rejected() {
    let mut fixture = load_case("claimed_managed_provider_boundary.yaml");
    let split = fixture
        .snapshot
        .rows
        .iter_mut()
        .find(|row| row.row_id == fixture.expect.plane_split_row_id)
        .expect("fixture has plane split row");
    split.planes.affected_action_families.clear();

    let err = fixture.snapshot.validate().unwrap_err();
    assert!(matches!(
        err,
        ManagedTruthValidationError::PlaneImpairmentMissingScope { .. }
    ));
}

fn load_manifest() -> FixtureManifest {
    let path = fixture_root().join("manifest.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn load_case(file: &str) -> ManagedTruthFixture {
    let path = fixture_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    status: String,
    case_files: Vec<FixtureCaseFile>,
}

#[derive(Debug, Deserialize)]
struct FixtureCaseFile {
    file: String,
    record_kind: String,
}

#[derive(Debug, Deserialize)]
struct ManagedTruthFixture {
    record_kind: String,
    schema_version: u32,
    snapshot: ManagedTruthSnapshot,
    expect: ManagedTruthExpect,
}

#[derive(Debug, Deserialize)]
struct ManagedTruthExpect {
    row_count: usize,
    required_claim_classes: Vec<String>,
    required_region_scopes: Vec<String>,
    required_key_modes: Vec<String>,
    plane_split_row_id: String,
    provider_row_id: String,
    support_pack_item_id: String,
}
