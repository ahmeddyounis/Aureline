use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_browser_runtime_inspection_qualification_input as current_stable_browser_runtime_input,
    current_stable_browser_runtime_inspection_qualification_packet, BrowserRuntimeFindingKind,
    BrowserRuntimeInspectionDataState, BrowserRuntimeInspectionQualificationPacket,
    BrowserRuntimeMutationActionClass, BrowserRuntimeObjectClass, BrowserRuntimePromotionState,
    BrowserRuntimeQualificationRowClass, BrowserRuntimeSourceMapQualityState,
    BrowserRuntimeTargetKind, BROWSER_RUNTIME_INSPECTION_QUALIFICATION_ARTIFACT_DOC_REF,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_DOC_REF,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_FIXTURE_DIR,
    BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(path.exists(), "expected path to exist: {}", path.display());
}

fn finding_kinds(packet: &BrowserRuntimeInspectionQualificationPacket) -> BTreeSet<&'static str> {
    packet
        .validation_findings
        .iter()
        .map(|finding| finding.finding_kind.as_str())
        .collect()
}

#[test]
fn schema_docs_and_fixture_pack_exist() {
    assert_exists(BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_REF);
    assert_exists(BROWSER_RUNTIME_INSPECTION_QUALIFICATION_DOC_REF);
    assert_exists(BROWSER_RUNTIME_INSPECTION_QUALIFICATION_ARTIFACT_DOC_REF);
    assert_exists(BROWSER_RUNTIME_INSPECTION_QUALIFICATION_FIXTURE_DIR);
    assert_exists(
        "fixtures/runtime/browser_runtime_cases/session_live_local_browser_workspace_trusted.yaml",
    );
    assert_exists("fixtures/runtime/browser_inspection_cases/console_live_exact_mapping.yaml");
}

#[test]
fn baseline_packet_materializes_stable() {
    let packet = current_stable_browser_runtime_inspection_qualification_packet();
    assert_eq!(packet.promotion_state, BrowserRuntimePromotionState::Stable);
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());

    let target_tokens: BTreeSet<&str> = packet.target_kind_tokens().into_iter().collect();
    for target_kind in BrowserRuntimeTargetKind::REQUIRED {
        assert!(target_tokens.contains(target_kind.as_str()));
    }
    let source_map_tokens: BTreeSet<&str> =
        packet.source_map_quality_tokens().into_iter().collect();
    for quality in BrowserRuntimeSourceMapQualityState::REQUIRED {
        assert!(source_map_tokens.contains(quality.as_str()));
    }
    let inspection_tokens: BTreeSet<&str> =
        packet.inspection_data_state_tokens().into_iter().collect();
    for state in BrowserRuntimeInspectionDataState::REQUIRED {
        assert!(inspection_tokens.contains(state.as_str()));
    }
    let mutation_tokens: BTreeSet<&str> = packet.mutation_action_tokens().into_iter().collect();
    for action in BrowserRuntimeMutationActionClass::REQUIRED {
        assert!(mutation_tokens.contains(action.as_str()));
    }

    let export = packet.support_export(
        "support:runtime:browser_runtime_inspection_qualification",
        "2026-06-04T18:40:00Z",
    );
    assert!(export.is_export_safe());
}

#[test]
fn missing_target_kind_blocks_stable() {
    let mut input = current_stable_browser_runtime_input();
    input.rows.retain(|row| {
        !(row.row_class == BrowserRuntimeQualificationRowClass::TargetKindAdmission
            && row.target_kind == BrowserRuntimeTargetKind::DeviceWebview)
    });
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(finding_kinds(&packet)
        .contains(BrowserRuntimeFindingKind::MissingTargetKindCoverage.as_str()));
}

#[test]
fn stable_target_without_runtime_truth_blocks_stable() {
    let mut input = current_stable_browser_runtime_input();
    let row = input
        .rows
        .iter_mut()
        .find(|row| {
            row.row_class == BrowserRuntimeQualificationRowClass::TargetKindAdmission
                && row.target_kind == BrowserRuntimeTargetKind::EmbeddedPreview
        })
        .expect("embedded preview target row exists");
    row.origin_scope_bound = false;
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(finding_kinds(&packet)
        .contains(BrowserRuntimeFindingKind::StableTargetMissingRuntimeTruth.as_str()));
}

#[test]
fn stable_target_with_stale_source_map_blocks_stable() {
    let mut input = current_stable_browser_runtime_input();
    let row = input
        .rows
        .iter_mut()
        .find(|row| {
            row.row_class == BrowserRuntimeQualificationRowClass::TargetKindAdmission
                && row.target_kind == BrowserRuntimeTargetKind::EmbeddedPreview
        })
        .expect("embedded preview target row exists");
    row.source_map_quality = BrowserRuntimeSourceMapQualityState::Stale;
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(finding_kinds(&packet)
        .contains(BrowserRuntimeFindingKind::StableTargetWithInsufficientSourceMap.as_str()));
}

#[test]
fn mutation_without_review_lineage_blocks_stable() {
    let mut input = current_stable_browser_runtime_input();
    let row = input
        .rows
        .iter_mut()
        .find(|row| {
            row.row_class == BrowserRuntimeQualificationRowClass::MutationActionReview
                && row.mutation_action == BrowserRuntimeMutationActionClass::ReplayRequest
        })
        .expect("replay request row exists");
    row.approval_review_ref = None;
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(
        finding_kinds(&packet).contains(BrowserRuntimeFindingKind::MutationReviewUnsafe.as_str())
    );
}

#[test]
fn object_class_collapse_blocks_stable() {
    let mut input = current_stable_browser_runtime_input();
    let row = input
        .rows
        .iter_mut()
        .find(|row| {
            row.row_class == BrowserRuntimeQualificationRowClass::ObjectClassAdmission
                && row.runtime_object_class == BrowserRuntimeObjectClass::CrossOriginFrame
        })
        .expect("cross-origin frame row exists");
    row.object_class_distinction_preserved = false;
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(finding_kinds(&packet)
        .contains(BrowserRuntimeFindingKind::RuntimeObjectClassCollapsed.as_str()));
}

#[test]
fn raw_material_blocks_support_export_safety() {
    let mut input = current_stable_browser_runtime_input();
    input.rows[0].raw_source_material_excluded = false;
    let packet = BrowserRuntimeInspectionQualificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserRuntimePromotionState::BlocksStable
    );
    assert!(finding_kinds(&packet)
        .contains(BrowserRuntimeFindingKind::UnsafeExportMaterialPresent.as_str()));
    let export = packet.support_export(
        "support:runtime:browser_runtime_inspection_qualification:unsafe",
        "2026-06-04T18:40:00Z",
    );
    assert!(!export.is_export_safe());
}
