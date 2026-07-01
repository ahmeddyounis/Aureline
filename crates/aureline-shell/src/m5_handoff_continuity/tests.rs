use super::*;

#[test]
fn seeded_set_validates() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.scenario_set_id, M5_HANDOFF_CONTINUITY_SCENARIO_SET_ID);
}

#[test]
fn seeded_set_names_every_failure_class() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for failure in HandoffFailureClass::ALL {
        assert!(
            set.drafts.iter().any(|d| d.failure_class == failure),
            "failure {} not represented",
            failure.as_str()
        );
    }
}

#[test]
fn seeded_set_exercises_every_continuity_state() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for state in DraftContinuityState::ALL {
        assert!(
            set.drafts.iter().any(|d| d.continuity_state == state),
            "state {} not exercised",
            state.as_str()
        );
    }
}

#[test]
fn seeded_set_preserves_every_trust_class() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for trust in DestinationTrustClass::ALL {
        assert!(
            set.drafts.iter().any(|d| d.intended_trust_class == trust),
            "trust {} not preserved",
            trust.as_str()
        );
    }
}

#[test]
fn seeded_set_offers_every_continuity_action() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for action in ContinuityActionClass::ALL {
        assert!(
            set.drafts
                .iter()
                .any(|d| d.available_actions.contains(&action)),
            "action {} never offered",
            action.as_str()
        );
    }
}

#[test]
fn every_redactable_field_is_covered() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for field in RedactableFieldClass::ALL {
        let covered = set
            .drafts
            .iter()
            .any(|d| d.redaction_state.iter().any(|r| r.field_class == field));
        assert!(covered, "field {} not covered", field.as_str());
    }
}

#[test]
fn live_drafts_keep_work_and_offer_all_actions() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for draft in &set.drafts {
        if draft.continuity_state.is_cleared() {
            continue;
        }
        assert!(
            draft.drafted_text.is_some(),
            "{} lost its text",
            draft.draft_id
        );
        assert!(!draft.redaction_state.is_empty());
        assert!(draft.draft_reusable_offline);
        for action in ContinuityActionClass::ALL {
            assert!(
                draft.available_actions.contains(&action),
                "{} missing action {}",
                draft.draft_id,
                action.as_str()
            );
        }
    }
}

#[test]
fn preserved_drafts_never_leave_the_product() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for draft in &set.drafts {
        assert_eq!(
            draft.current_data_exit_boundary,
            DataExitBoundary::NoPayloadLeavesProduct,
            "{} reports a non-local current data exit",
            draft.draft_id
        );
        assert!(draft.offline_capture_first_class);
        assert!(draft.preserves_target_class_on_retry);
        assert!(draft.preserves_visibility_boundary_on_export);
        assert!(!draft.auto_redirect_to_reachable_target_allowed);
        assert!(draft.persisted_state_visible_to_user);
    }
}

#[test]
fn tokens_are_always_removed() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for draft in &set.drafts {
        for row in &draft.redaction_state {
            if row.field_class == RedactableFieldClass::Token {
                assert_eq!(row.default_action, RedactionActionClass::RemovedEntirely);
                assert_eq!(row.chosen_action, RedactionActionClass::RemovedEntirely);
                assert!(row.mandatory_redaction);
            }
        }
    }
}

#[test]
fn cleared_draft_retains_nothing() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    let cleared = set
        .drafts
        .iter()
        .find(|d| d.continuity_state == DraftContinuityState::Cleared)
        .expect("cleared draft present");
    assert!(cleared.drafted_text.is_none());
    assert!(cleared.attachments.is_empty());
    assert!(cleared.redaction_state.is_empty());
    assert!(cleared.available_actions.is_empty());
    assert!(!cleared.draft_reusable_offline);
}

#[test]
fn security_route_is_preserved_and_not_redirected() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    let security = set
        .drafts
        .iter()
        .find(|d| d.intended_trust_class == DestinationTrustClass::PrivateSecurity)
        .expect("security draft present");
    assert_eq!(
        security.visibility_boundary,
        VisibilityBoundaryClass::PrivateSecurityChannel
    );
    assert_eq!(
        security.intended_data_exit_boundary,
        DataExitBoundary::SecurityPayloadsOnly
    );
    assert!(!security.auto_redirect_to_reachable_target_allowed);
    assert!(security.preserves_target_class_on_retry);
}

#[test]
fn silent_redirect_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    set.drafts[0].auto_redirect_to_reachable_target_allowed = true;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::SilentRouteRedirectAllowed { .. })
    ));
}

#[test]
fn losing_a_continuity_action_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    set.drafts[0]
        .available_actions
        .retain(|a| *a != ContinuityActionClass::ClearDraft);
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::MissingContinuityAction { .. })
    ));
}

#[test]
fn preserved_draft_that_left_product_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    set.drafts[0].current_data_exit_boundary = DataExitBoundary::MetadataSafeObjectRefs;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::PreservedDraftLeftProduct { .. })
    ));
}

#[test]
fn loosening_a_redaction_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    let row = set.drafts[0]
        .redaction_state
        .iter_mut()
        .find(|r| r.field_class == RedactableFieldClass::Hostname)
        .expect("hostname row present");
    row.chosen_action = RedactionActionClass::IncludedAsObjectRef;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::ChosenLoosensRedaction { .. })
    ));
}

#[test]
fn token_kept_as_ref_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    let row = set.drafts[0]
        .redaction_state
        .iter_mut()
        .find(|r| r.field_class == RedactableFieldClass::Token)
        .expect("token row present");
    row.default_action = RedactionActionClass::IncludedAsObjectRef;
    row.chosen_action = RedactionActionClass::IncludedAsObjectRef;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::FieldActionNotAllowed { .. })
    ));
}

#[test]
fn cleared_draft_with_actions_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    let cleared = set
        .drafts
        .iter_mut()
        .find(|d| d.continuity_state == DraftContinuityState::Cleared)
        .expect("cleared draft present");
    cleared.available_actions = vec![ContinuityActionClass::Retry];
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::ClearedDraftHasActions { .. })
    ));
}

#[test]
fn trust_visibility_mismatch_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    // The official-public draft cannot use a community-visible boundary.
    set.drafts[0].visibility_boundary = VisibilityBoundaryClass::CommunityVisible;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::TrustVisibilityMismatch { .. })
    ));
}

#[test]
fn missing_trust_class_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    // Reassign the private/security draft to a (duplicate) official-authenticated
    // route — and keep the draft itself valid — so the private/security trust class
    // is the only coverage that goes missing, without also dropping a unique
    // failure class or continuity state.
    let draft = set
        .drafts
        .iter_mut()
        .find(|d| d.intended_trust_class == DestinationTrustClass::PrivateSecurity)
        .expect("security draft present");
    draft.intended_trust_class = DestinationTrustClass::OfficialAuthenticated;
    draft.visibility_boundary = VisibilityBoundaryClass::OfficialAccountVisible;
    draft.intended_data_exit_boundary = DataExitBoundary::RedactedSupportPacket;
    draft.redaction_posture = RedactionPostureClass::RedactedSupportScoped;
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::TrustClassMissing { .. })
    ));
}

#[test]
fn missing_source_contract_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    set.source_contract_refs
        .retain(|r| r != M5_HANDOFF_CONTINUITY_PUBLIC_MATRIX_REF);
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::MissingSourceContracts)
    ));
}

#[test]
fn raw_ref_leak_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    set.drafts[0].object_anchor.object_ref = "https://example.com/object".to_owned();
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::RawRefLeak { .. })
            | Err(HandoffContinuityError::RawMaterialInExport)
    ));
}

#[test]
fn duplicate_draft_id_fails() {
    let mut set = seeded_m5_handoff_continuity_scenario_set();
    let dup = set.drafts[0].clone();
    set.drafts.push(dup);
    assert!(matches!(
        set.validate(),
        Err(HandoffContinuityError::DuplicateDraftId { .. })
    ));
}

#[test]
fn matrix_csv_has_a_row_per_draft() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + set.drafts.len());
    assert!(lines[0].starts_with("draft,failure_class,"));
    for draft in &set.drafts {
        assert!(
            csv.contains(&draft.draft_id),
            "csv missing {}",
            draft.draft_id
        );
    }
}

#[test]
fn markdown_summary_lists_every_draft() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    let summary = set.render_markdown_summary();
    for draft in &set.drafts {
        assert!(
            summary.contains(&draft.draft_id),
            "summary missing {}",
            draft.draft_id
        );
    }
}

#[test]
fn continuity_summary_lists_every_redaction_row() {
    let set = seeded_m5_handoff_continuity_scenario_set();
    for draft in &set.drafts {
        let text = draft.render_continuity_summary();
        for row in &draft.redaction_state {
            assert!(
                text.contains(row.field_class.as_str()),
                "continuity summary for {} missing {}",
                draft.draft_id,
                row.field_class.as_str()
            );
        }
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_handoff_continuity_scenario_set().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

#[test]
fn narrowed_fixture_drafts_validate() {
    for draft in [
        seeded_offline_security_draft_state(),
        seeded_cleared_draft_state(),
    ] {
        assert!(draft.validate().is_ok(), "{:?}", draft.validate());
    }
}

#[test]
fn offline_security_draft_keeps_private_route() {
    let draft = seeded_offline_security_draft_state();
    assert_eq!(
        draft.intended_trust_class,
        DestinationTrustClass::PrivateSecurity
    );
    assert_eq!(
        draft.continuity_state,
        DraftContinuityState::CapturedOffline
    );
    assert_eq!(
        draft.current_data_exit_boundary,
        DataExitBoundary::NoPayloadLeavesProduct
    );
    assert!(!draft.auto_redirect_to_reachable_target_allowed);
    assert!(draft.draft_reusable_offline);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_handoff_continuity_scenario_set()
        .expect("checked handoff continuity scenario set validates");
    assert_eq!(
        from_disk,
        seeded_m5_handoff_continuity_scenario_set(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_match_seed_builders() {
    let security: HandoffDraftState = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/handoff-continuity/offline_security_draft.json"
    )))
    .expect("offline-security fixture parses");
    assert_eq!(security, seeded_offline_security_draft_state());

    let cleared: HandoffDraftState = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/handoff-continuity/cleared_draft.json"
    )))
    .expect("cleared-draft fixture parses");
    assert_eq!(cleared, seeded_cleared_draft_state());
}
