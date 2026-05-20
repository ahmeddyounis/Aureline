//! Integration coverage for the provider target-mapping beta page.
//!
//! These conformance checks prove the exit-gate anchor for the
//! provider-account/board-mapping lane: every claimed beta provider row
//! discloses who Aureline acts as, which board/project/space/repository the
//! next action will touch, and whether that action is a local draft, a queued
//! publish, or a live provider mutation — and never silently widens a target
//! remap or a live mutation.

use std::collections::BTreeSet;

use aureline_provider::{
    audit_target_mapping_beta_page, seeded_target_mapping_beta_page,
    validate_target_mapping_beta_page, AccountScopeBetaProfileClass,
    MappingInvalidationTriggerClass, MappingLaneClass, MappingNextActionClass,
    MappingResolutionStateClass, ProviderSessionStateClass, PublishPostureClass,
    TargetMappingBetaDefectKind, TargetMappingBetaPage, TargetMappingBetaSupportExport,
    TARGET_MAPPING_BETA_PAGE_RECORD_KIND, TARGET_MAPPING_BETA_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_target_mapping_beta_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: TargetMappingBetaPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.record_kind, TARGET_MAPPING_BETA_PAGE_RECORD_KIND);
    assert_eq!(parsed.schema_version, TARGET_MAPPING_BETA_SCHEMA_VERSION);
    assert_eq!(parsed.review_rows.len(), page.review_rows.len());
    assert_eq!(
        parsed.invalidation_events.len(),
        page.invalidation_events.len()
    );
}

#[test]
fn seeded_page_validates_clean() {
    let page = seeded_target_mapping_beta_page();
    assert!(page.defects.is_empty(), "{:#?}", page.defects);
    validate_target_mapping_beta_page(&page).expect("seeded page validates");
}

#[test]
fn seeded_page_covers_all_profiles_and_lanes() {
    let page = seeded_target_mapping_beta_page();
    let profiles: BTreeSet<&str> = page
        .summary
        .profiles_present
        .iter()
        .map(String::as_str)
        .collect();
    for required in AccountScopeBetaProfileClass::ALL {
        assert!(
            profiles.contains(required.as_str()),
            "missing profile coverage: {}",
            required.as_str()
        );
    }
    let lanes: BTreeSet<&str> = page
        .summary
        .lanes_present
        .iter()
        .map(String::as_str)
        .collect();
    for required in MappingLaneClass::ALL {
        assert!(
            lanes.contains(required.as_str()),
            "missing lane coverage: {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_page_keeps_session_states_first_class() {
    let page = seeded_target_mapping_beta_page();
    let states: BTreeSet<&str> = page
        .summary
        .session_states_present
        .iter()
        .map(String::as_str)
        .collect();
    // Limited-scope, stale-credential, read-only, offline-capture, and
    // publish-later-only are preserved as distinct lanes, not collapsed into a
    // generic "unavailable" message.
    for required in [
        ProviderSessionStateClass::Live,
        ProviderSessionStateClass::LimitedScope,
        ProviderSessionStateClass::StaleCredential,
        ProviderSessionStateClass::ReadOnly,
        ProviderSessionStateClass::OfflineCapture,
        ProviderSessionStateClass::PublishLaterOnly,
    ] {
        assert!(
            states.contains(required.as_str()),
            "missing session-state coverage: {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_page_distinguishes_publish_postures() {
    let page = seeded_target_mapping_beta_page();
    let postures: BTreeSet<&str> = page
        .summary
        .publish_postures_present
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PublishPostureClass::LocalDraft,
        PublishPostureClass::QueuedPublishLater,
        PublishPostureClass::LiveProviderMutation,
        PublishPostureClass::ReadOnlyInspection,
    ] {
        assert!(
            postures.contains(required.as_str()),
            "missing publish-posture coverage: {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_page_exposes_policy_locked_and_unsupported_remap() {
    let page = seeded_target_mapping_beta_page();
    let states: BTreeSet<&str> = page
        .summary
        .resolution_states_present
        .iter()
        .map(String::as_str)
        .collect();
    // Policy-locked mappings and unsupported-remap cases are concrete states,
    // not silently disabled controls.
    assert!(states.contains(MappingResolutionStateClass::PolicyLockedTarget.as_str()));
    assert!(states.contains(MappingResolutionStateClass::UnsupportedRemap.as_str()));
    assert!(states.contains(MappingResolutionStateClass::AmbiguousNeedsSelection.as_str()));
    assert!(states.contains(MappingResolutionStateClass::StaleNeedsRefresh.as_str()));
    assert!(states.contains(MappingResolutionStateClass::Invalidated.as_str()));
}

#[test]
fn every_row_names_who_acts_and_what_target_and_what_posture() {
    let page = seeded_target_mapping_beta_page();
    for row in &page.review_rows {
        // Who acts: a governed account/session binding is always present and
        // resolves to an account-scope identity row.
        assert!(
            !row.account_session.bound_identity_row_ref.is_empty(),
            "row {} must bind an account-scope identity",
            row.row_id
        );
        assert_eq!(
            row.account_session.acting_identity_class_token,
            row.account_session.acting_identity_class.as_str()
        );

        // What posture: a live mutation must name the exact target it will
        // touch, run on a live session, and hold effective write scope. A
        // deferred/ambiguous row must name a concrete next action.
        if row.publish_posture == PublishPostureClass::LiveProviderMutation {
            assert!(
                row.selected_target.is_some(),
                "live row {} must name a target",
                row.row_id
            );
            assert!(row.resolution_state.admits_live_mutation());
            assert_eq!(
                row.account_session.session_state,
                ProviderSessionStateClass::Live
            );
            assert!(!row.account_session.effective_write_scope_refs.is_empty());
        } else {
            assert_ne!(
                row.next_action,
                MappingNextActionClass::NoneProceed,
                "non-live row {} must name a concrete next action",
                row.row_id
            );
        }
    }
}

#[test]
fn validator_blocks_silent_target_remap() {
    let mut page = seeded_target_mapping_beta_page();
    page.review_rows[0].silent_target_remap_taken = true;
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::SilentTargetRemapTaken));
}

#[test]
fn validator_blocks_silent_live_mutation_widening() {
    let mut page = seeded_target_mapping_beta_page();
    page.review_rows[0].silent_live_mutation_widened = true;
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::SilentLiveMutationWidened));
}

#[test]
fn validator_blocks_live_mutation_on_non_live_session() {
    let mut page = seeded_target_mapping_beta_page();
    // Find the live review row and force its session to stale-credential.
    let idx = page
        .review_rows
        .iter()
        .position(|r| r.publish_posture == PublishPostureClass::LiveProviderMutation)
        .expect("a live row exists");
    let row = &mut page.review_rows[idx];
    row.account_session.session_state = ProviderSessionStateClass::StaleCredential;
    row.account_session.session_state_token = ProviderSessionStateClass::StaleCredential
        .as_str()
        .to_owned();
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::LiveMutationOnNonLiveSession));
}

#[test]
fn validator_blocks_ambiguous_row_without_candidates() {
    let mut page = seeded_target_mapping_beta_page();
    let idx = page
        .review_rows
        .iter()
        .position(|r| r.resolution_state == MappingResolutionStateClass::AmbiguousNeedsSelection)
        .expect("an ambiguous row exists");
    page.review_rows[idx].candidate_target_refs.clear();
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::AmbiguousWithoutCandidates));
}

#[test]
fn validator_blocks_policy_lock_mismatch() {
    let mut page = seeded_target_mapping_beta_page();
    let idx = page
        .review_rows
        .iter()
        .position(|r| r.resolution_state == MappingResolutionStateClass::PolicyLockedTarget)
        .expect("a policy-locked row exists");
    page.review_rows[idx].policy_locked_target = false;
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::PolicyLockMismatch));
}

#[test]
fn validator_blocks_invalidation_forcing_live_posture() {
    let mut page = seeded_target_mapping_beta_page();
    page.invalidation_events[0].forced_posture = PublishPostureClass::LiveProviderMutation;
    page.invalidation_events[0].forced_posture_token = PublishPostureClass::LiveProviderMutation
        .as_str()
        .to_owned();
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == TargetMappingBetaDefectKind::InvalidationEventForcedLivePosture));
}

#[test]
fn invalidation_events_keep_drafts_and_evidence_durable() {
    let page = seeded_target_mapping_beta_page();
    assert!(!page.invalidation_events.is_empty());
    let row_ids: BTreeSet<&str> = page.review_rows.iter().map(|r| r.row_id.as_str()).collect();
    for event in &page.invalidation_events {
        // Each invalidation references a real row, keeps drafts/queued/evidence
        // durable, never forces a live posture, and names a next-safe action.
        assert!(row_ids.contains(event.affected_row_ref.as_str()));
        assert!(event.local_draft_preserved);
        assert!(event.queued_transitions_preserved);
        assert!(event.evidence_preserved);
        assert!(!event.silent_live_mutation_after_invalidation);
        assert_ne!(
            event.forced_posture,
            PublishPostureClass::LiveProviderMutation
        );
        assert_ne!(event.next_action, MappingNextActionClass::NoneProceed);
    }
    // The archived-target and stale-credential drills are both represented.
    let triggers: BTreeSet<&str> = page
        .invalidation_events
        .iter()
        .map(|e| e.trigger_token.as_str())
        .collect();
    assert!(triggers.contains(MappingInvalidationTriggerClass::TargetArchived.as_str()));
    assert!(triggers.contains(MappingInvalidationTriggerClass::CredentialWentStale.as_str()));
}

#[test]
fn support_export_excludes_raw_material() {
    let page = seeded_target_mapping_beta_page();
    let export = TargetMappingBetaSupportExport::from_page(
        "target-mapping-beta:support-export:test",
        "2026-05-18T12:00:00Z",
        page,
    );
    assert!(export.raw_tokens_excluded);
    assert!(export.fail_closed_invariant);
    assert!(export.identity_lineage_preserved);
    assert!(export.target_lineage_preserved);
    assert!(export.posture_lineage_preserved);
    assert!(export.invalidation_lineage_preserved);

    // Metadata-only: no raw URLs or bearer markers survive the round-trip.
    let json = serde_json::to_string(&export).expect("serialize export");
    assert!(!json.contains("https://"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("ssh://"));
}
