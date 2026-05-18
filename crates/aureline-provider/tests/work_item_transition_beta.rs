//! Integration coverage for provider-backed work-item transition beta records.

use std::collections::BTreeSet;

use aureline_provider::{
    seeded_work_item_transition_beta_page, validate_work_item_transition_beta_page,
    HandoffProviderAcceptanceClass, TransitionActionClass, TransitionAdmissibilityClass,
    WorkItemMutationMode, WorkItemRowPostureClass, WorkItemTransitionBetaDefectKind,
    WorkItemTransitionBetaPage, WorkItemTransitionBetaSupportExport,
    WORK_ITEM_TRANSITION_BETA_PAGE_RECORD_KIND, WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_work_item_transition_beta_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: WorkItemTransitionBetaPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(
        parsed.record_kind,
        WORK_ITEM_TRANSITION_BETA_PAGE_RECORD_KIND
    );
    assert_eq!(
        parsed.schema_version,
        WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION
    );
    assert_eq!(parsed.detail_records.len(), page.detail_records.len());
    assert_eq!(
        parsed.transition_packets.len(),
        page.transition_packets.len()
    );
    assert_eq!(
        parsed.offline_handoff_packets.len(),
        page.offline_handoff_packets.len()
    );
}

#[test]
fn seeded_page_validates_and_covers_required_truth_axes() {
    let page = seeded_work_item_transition_beta_page();
    validate_work_item_transition_beta_page(&page).expect("seeded page validates");
    let report = page.validate();
    assert!(report.passed, "seeded page defects: {:#?}", report.defects);

    for posture in [
        WorkItemRowPostureClass::ProviderAuthoritative,
        WorkItemRowPostureClass::CachedStale,
        WorkItemRowPostureClass::ReadOnly,
        WorkItemRowPostureClass::PolicyBlocked,
        WorkItemRowPostureClass::LocalDraft,
        WorkItemRowPostureClass::Queued,
        WorkItemRowPostureClass::OfflineCaptured,
    ] {
        assert!(
            report.coverage.row_postures.contains(&posture),
            "missing posture coverage: {posture:?}"
        );
    }

    for mode in [
        WorkItemMutationMode::PublishNow,
        WorkItemMutationMode::DeferredPublish,
        WorkItemMutationMode::OpenInProvider,
        WorkItemMutationMode::LocalDraft,
    ] {
        assert!(
            report.coverage.mutation_modes.contains(&mode),
            "missing mutation-mode coverage: {mode:?}"
        );
    }

    for action in [
        TransitionActionClass::MutateProviderStatePublishNow,
        TransitionActionClass::QueueForPublishLaterDeferred,
        TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider,
        TransitionActionClass::SaveLocalDraftOnlyNoProviderPath,
        TransitionActionClass::CapturedOfflinePendingDrain,
    ] {
        assert!(
            report.coverage.transition_action_classes.contains(&action),
            "missing transition-action coverage: {action:?}"
        );
    }
}

#[test]
fn support_export_preserves_actor_target_and_publish_posture_without_raw_material() {
    let page = seeded_work_item_transition_beta_page();
    let export = WorkItemTransitionBetaSupportExport::from_page(
        "work-item-transition-beta:support-export:test",
        "2026-05-18T09:30:00Z",
        &page,
    );
    assert!(export.raw_provider_material_excluded);
    assert_eq!(export.detail_summaries.len(), page.detail_records.len());

    let postures: BTreeSet<_> = export
        .detail_summaries
        .iter()
        .map(|summary| summary.publish_posture)
        .collect();
    assert!(!postures.is_empty());
    for summary in &export.detail_summaries {
        assert!(!summary.canonical_id.trim().is_empty());
        assert!(!summary.project_or_space_ref.trim().is_empty());
    }

    let json = serde_json::to_string(&export).expect("serialize export");
    assert!(!json.contains("https://"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("raw_provider_payload"));
}

#[test]
fn offline_handoff_cannot_claim_acceptance_without_callback() {
    let mut page = seeded_work_item_transition_beta_page();
    let packet = page
        .offline_handoff_packets
        .iter_mut()
        .find(|packet| packet.packet_id == "work_items:offline_handoff:provider-unreachable")
        .expect("provider unreachable packet");
    packet.handoff_provider_acceptance_class =
        HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained;
    packet
        .linked_provider_callback_envelope_record_id_refs
        .clear();

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent
            && defect.check_id == "work_item_transition_beta.offline_accept_without_callback"
    }));
}

#[test]
fn local_draft_transition_cannot_claim_provider_mutation() {
    let mut page = seeded_work_item_transition_beta_page();
    let packet = page
        .transition_packets
        .iter_mut()
        .find(|packet| packet.packet_id == "work_items:transition_packet:local-draft")
        .expect("local draft packet");
    packet.transition_entries[0].transition_action_class =
        TransitionActionClass::MutateProviderStatePublishNow;
    packet.packet_admissibility_class = TransitionAdmissibilityClass::AdmissibleNowPublishNow;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.check_id
            == "work_item_transition_beta.transition_silent_provider_mutation_local_draft"
    }));
}

#[test]
fn transition_review_requires_side_effect_and_escape_actions() {
    let mut page = seeded_work_item_transition_beta_page();
    let review = page
        .transition_reviews
        .iter_mut()
        .find(|review| review.review_id == "work_items:transition_review:publish-now")
        .expect("publish-now review");
    review.side_effect_fanout_rows.clear();
    review.action_affordances.export_action_available = false;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "work_item_transition_beta.review_fanout_missing"));
    assert!(report.defects.iter().any(|defect| {
        defect.check_id == "work_item_transition_beta.review_export_cancel_missing"
    }));
}
