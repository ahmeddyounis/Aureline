//! Inline unit coverage for the docs-pack manager packet.

use super::*;

fn stable_packet() -> DocsPackManagerPacket {
    DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input())
}

#[test]
fn seeded_packet_is_stable_with_no_findings() {
    let packet = stable_packet();
    assert_eq!(packet.record_kind, DOCS_PACK_MANAGER_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_PACK_MANAGER_SCHEMA_VERSION);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::Stable
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_stable());
}

#[test]
fn seeded_packet_covers_every_lifecycle_flow() {
    let packet = stable_packet();
    let tokens = packet.lifecycle_flow_tokens();
    for flow in DocsPackLifecycleFlow::REQUIRED {
        assert!(
            tokens.contains(&flow.as_str()),
            "missing lifecycle flow {}",
            flow.as_str()
        );
    }
}

#[test]
fn seeded_packet_projects_every_profile() {
    let packet = stable_packet();
    for profile in DocsPackManagerProfile::REQUIRED {
        assert!(
            packet.has_projection_for(profile),
            "missing projection for {}",
            profile.as_str()
        );
    }
}

#[test]
fn rows_keep_signer_channel_mirror_version_visible() {
    let packet = stable_packet();
    for row in &packet.rows {
        assert!(row.shows_signer);
        assert!(row.shows_channel);
        assert!(row.shows_mirror_source);
        assert!(row.shows_version_range);
        assert!(row.shows_refresh_state);
        assert!(row.shows_pin_offline_posture);
    }
}

#[test]
fn unavailable_row_discloses_payload_and_signature() {
    let packet = stable_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.row_id == "manager-row:extension-pack-unavailable")
        .expect("unavailable row present");
    assert!(row
        .manifest
        .local_availability
        .content_unavailable_locally());
    assert!(row.unavailable_payload_disclosed);
    assert!(row.signature_state_visible);
    assert!(row.pack_size_bytes.is_none());
}

#[test]
fn hiding_mirror_source_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:std-mirror")
        .expect("mirror row present")
        .shows_mirror_source = false;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsPackManagerFindingKind::ManagerRowHidesManifestTruth
            || finding.finding_kind == DocsPackManagerFindingKind::MirrorOfflineDegraded
    }));
}

#[test]
fn hiding_unavailable_payload_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:extension-pack-unavailable")
        .expect("unavailable row present")
        .unavailable_payload_disclosed = false;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet.validation_findings.iter().any(
        |finding| finding.finding_kind == DocsPackManagerFindingKind::UnavailablePayloadHidden
    ));
}

#[test]
fn degrading_offline_pack_to_cache_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == "manager-row:support-runbook")
        .expect("air-gapped row present")
        .degraded_to_opaque_cache = true;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == DocsPackManagerFindingKind::MirrorOfflineDegraded));
}

#[test]
fn losing_import_export_continuity_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .first_mut()
        .expect("row present")
        .import_export_continuity
        .preserves_identity_on_export = false;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPackManagerFindingKind::ImportExportContinuityLost));
}

#[test]
fn disabled_action_without_reason_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    let row = input.rows.first_mut().expect("row present");
    row.actions.push(DocsPackManagerActionState {
        action: DocsPackManagerAction::ChangeMirrorSource,
        availability: DocsPackManagerActionAvailability::DisabledByPolicy,
        disabled_reason: None,
    });
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPackManagerFindingKind::ManagerActionReasonMissing));
}

#[test]
fn missing_required_action_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .first_mut()
        .expect("row present")
        .actions
        .retain(|state| state.action != DocsPackManagerAction::ExportPack);
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPackManagerFindingKind::RequiredManagerActionMissing));
}

#[test]
fn dropping_a_lifecycle_flow_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .retain(|row| row.lifecycle_flow != DocsPackLifecycleFlow::AirGapped);
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet.validation_findings.iter().any(|finding| {
        finding.finding_kind == DocsPackManagerFindingKind::RequiredLifecycleFlowCoverageMissing
    }));
}

#[test]
fn profile_projection_drift_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .profile_projections
        .first_mut()
        .expect("projection present")
        .preserves_import_export_continuity = false;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == DocsPackManagerFindingKind::ProfileProjectionDrift));
}

#[test]
fn lifecycle_flow_origin_mismatch_blocks_stable() {
    let mut input = seeded_stable_docs_pack_manager_input();
    input
        .rows
        .iter_mut()
        .find(|row| row.lifecycle_flow == DocsPackLifecycleFlow::AirGapped)
        .expect("air-gapped row present")
        .import_export_continuity
        .import_origin = DocsPackImportOrigin::FreshlyInstalled;
    let packet = DocsPackManagerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DocsPackManagerFindingKind::LifecycleFlowOriginMismatch));
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = stable_packet();
    let export = packet.support_export("export:docs_pack_manager:001", "2026-06-26T00:00:00Z");
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: DocsPackManagerSupportExport = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
}

#[test]
fn promotion_state_mismatch_is_detected_on_validate() {
    let mut packet = stable_packet();
    packet.promotion_state = DocsPackManagerPromotionState::BlocksStable;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsPackManagerFindingKind::PromotionStateMismatch));
}

#[test]
fn finding_tokens_are_pinned() {
    assert_eq!(
        DocsPackManagerFindingKind::ManagerRowHidesManifestTruth.as_str(),
        "manager_row_hides_manifest_truth"
    );
    assert_eq!(
        DocsPackManagerFindingKind::UnavailablePayloadHidden.as_str(),
        "unavailable_payload_hidden"
    );
    assert_eq!(
        DocsPackManagerFindingKind::MirrorOfflineDegraded.as_str(),
        "mirror_offline_degraded"
    );
    assert_eq!(
        DocsPackManagerFindingKind::ImportExportContinuityLost.as_str(),
        "import_export_continuity_lost"
    );
}
