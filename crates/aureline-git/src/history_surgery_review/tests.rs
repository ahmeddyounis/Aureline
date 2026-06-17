use super::*;

const CANONICAL_PACKET: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json"
));

const FORCE_PUSH_BLOCKED_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/rebase-cherry-pick-reset/force_push_protected_blocked.json"
));

const REBASE_RAW_FALLBACK_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/rebase-cherry-pick-reset/rebase_raw_todo_fallback.json"
));

const PROVIDER_OUTAGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/rebase-cherry-pick-reset/reset_provider_outage_local_only.json"
));

fn baseline() -> HistorySurgeryReviewPacket {
    serde_json::from_str(CANONICAL_PACKET).expect("canonical packet deserializes")
}

fn sheet<'a>(packet: &'a HistorySurgeryReviewPacket, id: &str) -> &'a HistorySurgeryReviewSheet {
    packet
        .sheets
        .iter()
        .find(|sheet| sheet.sheet_id == id)
        .expect("sheet present")
}

fn sheet_mut<'a>(
    packet: &'a mut HistorySurgeryReviewPacket,
    id: &str,
) -> &'a mut HistorySurgeryReviewSheet {
    packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.sheet_id == id)
        .expect("sheet present")
}

#[test]
fn checked_artifact_validates() {
    let packet = current_history_surgery_review_sheets().expect("checked packet validates clean");
    assert_eq!(packet.packet_id, "git-history-surgery-review:0001");
}

#[test]
fn canonical_packet_validates_clean() {
    let packet = baseline();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_packet_round_trips() {
    let packet = baseline();
    let reparsed = HistorySurgeryReviewPacket::parse_json(&packet.export_safe_json())
        .expect("export round-trips through parse_json");
    assert_eq!(packet, reparsed);
}

#[test]
fn every_verb_has_a_distinct_sheet() {
    let packet = baseline();
    for verb in HistorySurgeryVerb::ALL {
        assert!(
            packet.sheets.iter().any(|sheet| sheet.verb == verb),
            "canonical packet missing verb {}",
            verb.as_str()
        );
    }
    // Distinct verbs are never collapsed: six verbs, six sheets.
    assert_eq!(packet.sheets.len(), HistorySurgeryVerb::ALL.len());
}

#[test]
fn fixtures_validate() {
    for raw in [
        FORCE_PUSH_BLOCKED_FIXTURE,
        REBASE_RAW_FALLBACK_FIXTURE,
        PROVIDER_OUTAGE_FIXTURE,
    ] {
        let packet =
            HistorySurgeryReviewPacket::parse_json(raw).expect("fixture parses and validates");
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}

#[test]
fn every_sheet_names_an_exact_target_and_recovery() {
    let packet = baseline();
    for sheet in &packet.sheets {
        assert!(
            !sheet.primary_target_ref.trim().is_empty(),
            "sheet {} has no target",
            sheet.sheet_id
        );
        assert!(
            HISTORY_SURGERY_TARGET_KINDS.contains(&sheet.target_kind.as_str()),
            "sheet {} target kind out of vocabulary",
            sheet.sheet_id
        );
        // Every allowed mutation keeps a visible recovery path.
        assert!(
            sheet.decision.recovery_visible,
            "sheet {} hides recovery",
            sheet.sheet_id
        );
    }
}

#[test]
fn rebase_and_patch_apply_preserve_raw_source_text() {
    let packet = baseline();
    for id in ["rebase-0001", "patch-apply-0001"] {
        let sheet = sheet(&packet, id);
        assert!(
            sheet.raw_source_text_ref.is_some(),
            "{id} drops raw source text"
        );
        assert!(sheet.verb.requires_source_text());
    }
}

#[test]
fn force_push_carries_lease_and_divergence() {
    let packet = baseline();
    let force = sheet(&packet, "force-push-0001");
    assert!(force.force_lease_ref.is_some());
    assert!(force.divergence_class.is_some());
    assert!(force.verb.is_publish_class());
}

#[test]
fn stored_decision_matches_derived_decision() {
    let packet = baseline();
    for sheet in &packet.sheets {
        assert_eq!(
            sheet.decision,
            sheet.derive_decision(),
            "sheet {} decision drifted from its gates",
            sheet.sheet_id
        );
    }
}

#[test]
fn protected_branch_blocks_force_push() {
    let packet = HistorySurgeryReviewPacket::parse_json(FORCE_PUSH_BLOCKED_FIXTURE)
        .expect("force-push-blocked fixture parses");
    let blocked = sheet(&packet, "force-push-blocked-0001");
    assert_eq!(blocked.decision.outcome, ReviewDecisionOutcome::Blocked);
    assert_eq!(blocked.decision.primary_reason, "blocked_protected_branch");
    // A blocked network mutation still keeps local preview/abort/restore truth.
    assert!(blocked.decision.local_truth_available_offline);
    for required in HISTORY_SURGERY_REQUIRED_LOCAL_ACTIONS {
        assert!(blocked
            .local_actions
            .iter()
            .any(|action| action == required));
    }
    assert!(blocked
        .local_actions
        .iter()
        .any(|a| a == "restore_checkpoint"));
}

#[test]
fn rebase_downgrades_to_raw_todo_when_structured_parsing_fails() {
    let packet = HistorySurgeryReviewPacket::parse_json(REBASE_RAW_FALLBACK_FIXTURE)
        .expect("rebase-raw-fallback fixture parses");
    let rebase = sheet(&packet, "rebase-raw-fallback-0001");
    assert_eq!(rebase.decision.outcome, ReviewDecisionOutcome::Downgraded);
    assert_eq!(
        rebase.decision.primary_reason,
        "downgraded_raw_inspection_only"
    );
    // The raw todo stays inspectable even though structured cards are absent.
    assert!(rebase.raw_source_text_ref.is_some());
    assert!(rebase.structured_cards_ref.is_none());
}

#[test]
fn provider_outage_downgrades_but_keeps_local_truth() {
    let packet = HistorySurgeryReviewPacket::parse_json(PROVIDER_OUTAGE_FIXTURE)
        .expect("provider-outage fixture parses");
    let reset = sheet(&packet, "reset-provider-outage-0001");
    assert_eq!(
        reset.provider_overlay_state,
        "overlay_unavailable_local_only"
    );
    // A provider outage downgrades, never blocks; local truth stays available.
    assert_ne!(reset.decision.outcome, ReviewDecisionOutcome::Blocked);
    assert!(reset.decision.local_truth_available_offline);
}

#[test]
fn provider_outage_can_never_block_alone() {
    // Build a sheet whose only non-clear gate is an unavailable provider overlay.
    let mut packet = baseline();
    let reset = sheet_mut(&mut packet, "reset-0001");
    reset.provider_overlay_state = "overlay_unavailable_local_only".to_owned();
    let rebuilt = rebuild(reset);
    *reset = rebuilt;
    let reset = sheet(&packet, "reset-0001");
    assert_ne!(reset.decision.outcome, ReviewDecisionOutcome::Blocked);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

/// Re-derives a sheet's decision after a gate was mutated in a test.
fn rebuild(sheet: &HistorySurgeryReviewSheet) -> HistorySurgeryReviewSheet {
    let mut next = sheet.clone();
    next.decision = next.derive_decision();
    next
}

#[test]
fn tampered_decision_fails_validation() {
    let mut packet = baseline();
    // Forge an allowed outcome onto a sheet whose gates would block it.
    let reset = sheet_mut(&mut packet, "reset-0001");
    reset.conflict_source_state = "conflicts_present_blocks_continue".to_owned();
    // Leave the stored decision claiming "allowed".
    let violations = packet.validate();
    assert!(violations.iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::DecisionDoesNotMatchGates { .. }
    )));
}

#[test]
fn allowed_without_recovery_fails() {
    let mut packet = baseline();
    let reset = sheet_mut(&mut packet, "reset-0001");
    reset.checkpoint_lineage_refs.clear();
    reset.reflog_only_fallback = false;
    let rebuilt = rebuild(reset);
    *reset = rebuilt;
    // With no recovery the gate blocks; the outcome can never be "allowed".
    let reset = sheet(&packet, "reset-0001");
    assert_eq!(reset.decision.outcome, ReviewDecisionOutcome::Blocked);
    assert_eq!(reset.decision.primary_reason, "blocked_no_recovery_path");
}

#[test]
fn reset_without_mode_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "reset-0001").reset_mode = None;
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::FieldOutOfVocabulary { field, .. } if field == "reset_mode"
    )));
}

#[test]
fn cherry_pick_without_source_commits_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "cherry-pick-0001")
        .secondary_refs
        .clear();
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::MissingSourceCommits { .. }
    )));
}

#[test]
fn force_push_without_lease_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "force-push-0001").force_lease_ref = None;
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::ForcePushMissingLease { .. }
    )));
}

#[test]
fn missing_target_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "rebase-0001").primary_target_ref = String::new();
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::MissingTarget { .. }
    )));
}

#[test]
fn missing_local_action_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "rebase-0001")
        .local_actions
        .retain(|action| action != "abort");
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::MissingLocalAction { action, .. } if action == "abort"
    )));
}

#[test]
fn duplicate_sheet_fails() {
    let mut packet = baseline();
    let dup = sheet(&packet, "rebase-0001").clone();
    packet.sheets.push(dup);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::DuplicateSheetId { .. }
    )));
}

#[test]
fn support_export_covers_every_sheet() {
    let packet = baseline();
    for sheet in &packet.sheets {
        assert!(
            packet.support_export.sheet_refs.contains(&sheet.sheet_id),
            "support export omits {}",
            sheet.sheet_id
        );
    }
}

#[test]
fn support_export_missing_field_fails() {
    let mut packet = baseline();
    packet
        .support_export
        .reconstruction_fields
        .retain(|field| field != "decision_outcome");
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        HistorySurgeryReviewValidationError::SupportExportMissingField { .. }
    )));
}

#[test]
fn support_export_unredacted_fails() {
    let mut packet = baseline();
    packet.support_export.raw_patch_bodies_redacted = false;
    assert!(packet
        .validate()
        .contains(&HistorySurgeryReviewValidationError::SupportExportEmbedsRawMaterial));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "rebase-0001").summary_label = "leak bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&HistorySurgeryReviewValidationError::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_verb() {
    let summary = baseline().render_markdown_summary();
    for verb in HistorySurgeryVerb::ALL {
        assert!(
            summary.contains(verb.as_str()),
            "summary missing verb {}",
            verb.as_str()
        );
    }
}
