use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_content_safety::{
    stable_safe_preview_trust_packet, SafePreviewConsumerSurface, SafePreviewTransferCaseKind,
    StableSafePreviewTrustPacket, REQUIRED_STABLE_CONSUMER_SURFACES, REQUIRED_TRANSFER_CASE_KINDS,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/trust/m4/stabilize-safe-preview-trust-classes/canonical_packet.json")
}

fn load_fixture() -> StableSafePreviewTrustPacket {
    let path = fixture_path();
    let raw =
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn checked_in_packet_matches_canonical_builder() {
    assert_eq!(load_fixture(), stable_safe_preview_trust_packet());
}

#[test]
fn checked_in_packet_validates_stable_surface_and_transfer_coverage() {
    let packet = load_fixture();
    let report = packet.validate();
    assert!(report.is_green(), "{:#?}", report.violations);

    let observed_surfaces = packet
        .surface_matrix
        .iter()
        .filter(|row| row.qualification.as_str() == "stable")
        .map(|row| row.surface)
        .collect::<BTreeSet<_>>();
    for surface in REQUIRED_STABLE_CONSUMER_SURFACES {
        assert!(
            observed_surfaces.contains(&surface),
            "missing stable consumer surface {}",
            surface.as_str()
        );
    }

    let observed_case_kinds = packet
        .transfer_cases
        .iter()
        .map(|case| case.case_kind)
        .collect::<BTreeSet<_>>();
    for case_kind in REQUIRED_TRANSFER_CASE_KINDS {
        assert!(
            observed_case_kinds.contains(&case_kind),
            "missing transfer case kind {}",
            case_kind.as_str()
        );
    }
}

#[test]
fn embedded_and_browser_rows_disclose_origin_and_downgrade_without_raw_source() {
    let packet = load_fixture();
    for surface in [
        SafePreviewConsumerSurface::MarketplaceAccountWebview,
        SafePreviewConsumerSurface::BrowserRuntimeViewer,
    ] {
        let row = packet
            .surface_matrix
            .iter()
            .find(|row| row.surface == surface && row.qualification.as_str() == "stable")
            .unwrap_or_else(|| panic!("missing stable row for {}", surface.as_str()));
        assert!(!row.source_representation_available);
        assert!(row.owner_identity_visible);
        assert!(row.origin_identity_visible);
        assert!(row.permission_summary_visible);
        assert!(row.auto_upgrade_blocked);
        assert!(!row.handled_downgrade_triggers.is_empty());
    }
}

#[test]
fn downgrade_and_blocked_cases_keep_truth_lineage() {
    let packet = load_fixture();
    for kind in [
        SafePreviewTransferCaseKind::Downgrade,
        SafePreviewTransferCaseKind::Blocked,
    ] {
        let case = packet
            .transfer_cases
            .iter()
            .find(|case| case.case_kind == kind)
            .unwrap_or_else(|| panic!("missing transfer case {}", kind.as_str()));
        assert!(case.downgrade_trigger.is_some());
        assert_ne!(case.effective_downgrade_state.as_str(), "none");
        assert!(case.trust_class_lineage_preserved);
        assert!(case.origin_truth_preserved);
        assert!(case.permission_truth_preserved);
    }
}
