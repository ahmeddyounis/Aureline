use super::*;

const CANONICAL_PACKET: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/stash_recovery/stash_recovery.json"
));

const POP_CONFLICT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/stash-recovery/stash_pop_conflict_blocked.json"
));

const REFLOG_ONLY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/stash-recovery/reflog_restore_only_caveats.json"
));

const PROVIDER_OUTAGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/stash-recovery/stash_apply_provider_outage_local_only.json"
));

fn baseline() -> StashRecoveryPacket {
    serde_json::from_str(CANONICAL_PACKET).expect("canonical packet deserializes")
}

fn sheet<'a>(packet: &'a StashRecoveryPacket, id: &str) -> &'a StashRecoverySheet {
    packet
        .sheets
        .iter()
        .find(|sheet| sheet.sheet_id == id)
        .expect("sheet present")
}

fn sheet_mut<'a>(packet: &'a mut StashRecoveryPacket, id: &str) -> &'a mut StashRecoverySheet {
    packet
        .sheets
        .iter_mut()
        .find(|sheet| sheet.sheet_id == id)
        .expect("sheet present")
}

/// Re-derives a sheet's decision after a gate was mutated in a test.
fn rebuild(sheet: &StashRecoverySheet) -> StashRecoverySheet {
    let mut next = sheet.clone();
    next.decision = next.derive_decision();
    next
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stash_recovery_sheets().expect("checked packet validates clean");
    assert_eq!(packet.packet_id, "git-stash-recovery:0001");
}

#[test]
fn canonical_packet_validates_clean() {
    let packet = baseline();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_packet_round_trips() {
    let packet = baseline();
    let reparsed = StashRecoveryPacket::parse_json(&packet.export_safe_json())
        .expect("export round-trips through parse_json");
    assert_eq!(packet, reparsed);
}

#[test]
fn every_verb_has_a_distinct_sheet() {
    let packet = baseline();
    for verb in StashRecoveryVerb::ALL {
        assert!(
            packet.sheets.iter().any(|sheet| sheet.verb == verb),
            "canonical packet missing verb {}",
            verb.as_str()
        );
    }
    // Distinct verbs are never collapsed: six verbs, six sheets.
    assert_eq!(packet.sheets.len(), StashRecoveryVerb::ALL.len());
}

#[test]
fn stash_verbs_differ_in_consumption() {
    // Apply preserves the entry; pop, drop, and create-branch consume it. This is
    // the parity distinction that keeps the four stash verbs from collapsing.
    assert!(!StashRecoveryVerb::StashApply.consumes_stash_entry());
    assert!(StashRecoveryVerb::StashPop.consumes_stash_entry());
    assert!(StashRecoveryVerb::StashDrop.consumes_stash_entry());
    assert!(StashRecoveryVerb::StashCreateBranch.consumes_stash_entry());
}

#[test]
fn fixtures_validate() {
    for raw in [
        POP_CONFLICT_FIXTURE,
        REFLOG_ONLY_FIXTURE,
        PROVIDER_OUTAGE_FIXTURE,
    ] {
        let packet = StashRecoveryPacket::parse_json(raw).expect("fixture parses and validates");
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
            STASH_RECOVERY_TARGET_KINDS.contains(&sheet.target_kind.as_str()),
            "sheet {} target kind out of vocabulary",
            sheet.sheet_id
        );
        // Every allowed verb keeps a visible recovery path.
        assert!(
            sheet.decision.recovery_visible,
            "sheet {} hides recovery",
            sheet.sheet_id
        );
    }
}

#[test]
fn stash_sheets_name_an_exact_entry() {
    let packet = baseline();
    for sheet in packet
        .sheets
        .iter()
        .filter(|sheet| sheet.verb.is_stash_verb())
    {
        assert!(
            sheet.stash_entry_ref.is_some(),
            "{} drops its stash entry",
            sheet.sheet_id
        );
        assert!(sheet.stash_index.is_some());
        assert!(sheet.recovery_anchor.is_none());
        assert!(sheet
            .local_actions
            .iter()
            .any(|action| action == "inspect_stash"));
    }
}

#[test]
fn recovery_sheets_carry_anchor_with_compare_and_open_diff() {
    let packet = baseline();
    for sheet in packet
        .sheets
        .iter()
        .filter(|sheet| sheet.verb.is_recovery_verb())
    {
        let anchor = sheet
            .recovery_anchor
            .as_ref()
            .expect("recovery anchor present");
        assert!(STASH_RECOVERY_ANCHOR_KINDS.contains(&anchor.anchor_kind.as_str()));
        assert!(!anchor.compare_action_ref.trim().is_empty());
        assert!(!anchor.open_diff_action_ref.trim().is_empty());
        for required in ["compare", "open_diff"] {
            assert!(
                sheet.local_actions.iter().any(|action| action == required),
                "{} omits {}",
                sheet.sheet_id,
                required
            );
        }
    }
}

#[test]
fn create_branch_names_a_new_branch() {
    let packet = baseline();
    let create = sheet(&packet, "stash-create-branch-0001");
    assert_eq!(create.verb, StashRecoveryVerb::StashCreateBranch);
    assert!(create.new_branch_ref.is_some());
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
fn conflict_blocks_pop_but_keeps_local_truth() {
    let packet =
        StashRecoveryPacket::parse_json(POP_CONFLICT_FIXTURE).expect("pop-conflict fixture parses");
    let blocked = sheet(&packet, "stash-pop-conflict-0001");
    assert_eq!(blocked.decision.outcome, StashRecoveryOutcome::Blocked);
    assert_eq!(
        blocked.decision.primary_reason,
        "blocked_unresolved_conflict"
    );
    // A blocked verb still keeps local preview/abort/inspect/restore truth.
    assert!(blocked.decision.local_truth_available_offline);
    for required in STASH_RECOVERY_REQUIRED_LOCAL_ACTIONS {
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
fn reflog_only_restore_preserves_caveats() {
    let packet =
        StashRecoveryPacket::parse_json(REFLOG_ONLY_FIXTURE).expect("reflog-only fixture parses");
    let reflog = sheet(&packet, "reflog-restore-only-0001");
    assert_eq!(reflog.decision.outcome, StashRecoveryOutcome::Downgraded);
    // Only a reflog-based recovery exists, so the restore preserves its caveats
    // rather than pretending to be a durable checkpoint.
    assert!(reflog.reflog_only_fallback);
    assert!(reflog.checkpoint_lineage_refs.is_empty());
    assert!(!reflog.restore_caveats.is_empty());
    assert!(reflog
        .restore_caveats
        .iter()
        .any(|caveat| caveat == "anchor_expiring_soon"));
}

#[test]
fn provider_outage_downgrades_but_keeps_local_truth() {
    let packet = StashRecoveryPacket::parse_json(PROVIDER_OUTAGE_FIXTURE)
        .expect("provider-outage fixture parses");
    let apply = sheet(&packet, "stash-apply-outage-0001");
    assert_eq!(
        apply.provider_overlay_state,
        "overlay_unavailable_local_only"
    );
    // A provider outage downgrades, never blocks; local truth stays available.
    assert_ne!(apply.decision.outcome, StashRecoveryOutcome::Blocked);
    assert!(apply.decision.local_truth_available_offline);
}

#[test]
fn provider_outage_can_never_block_alone() {
    // Build a sheet whose only non-clear gate is an unavailable provider overlay.
    let mut packet = baseline();
    let apply = sheet_mut(&mut packet, "stash-apply-0001");
    apply.provider_overlay_state = "overlay_unavailable_local_only".to_owned();
    let rebuilt = rebuild(apply);
    *apply = rebuilt;
    let apply = sheet(&packet, "stash-apply-0001");
    assert_ne!(apply.decision.outcome, StashRecoveryOutcome::Blocked);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn tampered_decision_fails_validation() {
    let mut packet = baseline();
    // Force a blocking gate while leaving the stored decision claiming "allowed".
    sheet_mut(&mut packet, "stash-pop-0001").conflict_source_state =
        "conflicts_present_blocks_continue".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::DecisionDoesNotMatchGates { .. }
    )));
}

#[test]
fn allowed_without_recovery_fails() {
    let mut packet = baseline();
    let drop = sheet_mut(&mut packet, "stash-drop-0001");
    drop.checkpoint_lineage_refs.clear();
    drop.reflog_only_fallback = false;
    let rebuilt = rebuild(drop);
    *drop = rebuilt;
    // With no recovery the gate blocks; the outcome can never be "allowed".
    let drop = sheet(&packet, "stash-drop-0001");
    assert_eq!(drop.decision.outcome, StashRecoveryOutcome::Blocked);
    assert_eq!(drop.decision.primary_reason, "blocked_no_recovery_path");
}

#[test]
fn reflog_only_without_caveats_fails() {
    let mut packet = baseline();
    let drop = sheet_mut(&mut packet, "stash-drop-0001");
    drop.checkpoint_lineage_refs.clear();
    drop.reflog_only_fallback = true;
    drop.restore_caveats.clear();
    let rebuilt = rebuild(drop);
    *drop = rebuilt;
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::MissingReflogCaveat { .. }
    )));
}

#[test]
fn stash_verb_without_entry_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "stash-apply-0001").stash_entry_ref = None;
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::MissingStashEntry { .. }
    )));
}

#[test]
fn create_branch_without_branch_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "stash-create-branch-0001").new_branch_ref = None;
    assert!(packet
        .validate()
        .iter()
        .any(|error| matches!(error, StashRecoveryValidationError::MissingNewBranch { .. })));
}

#[test]
fn recovery_verb_without_anchor_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "reflog-restore-0001").recovery_anchor = None;
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::MissingRecoveryAnchor { .. }
    )));
}

#[test]
fn anchor_kind_mismatch_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "reflog-restore-0001")
        .recovery_anchor
        .as_mut()
        .expect("anchor present")
        .anchor_kind = "checkpoint".to_owned();
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::AnchorKindMismatch { .. }
    )));
}

#[test]
fn anchor_without_compare_actions_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "checkpoint-restore-0001")
        .recovery_anchor
        .as_mut()
        .expect("anchor present")
        .compare_action_ref = String::new();
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::AnchorMissingCompareActions { .. }
    )));
}

#[test]
fn stash_field_on_recovery_verb_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "reflog-restore-0001").stash_entry_ref =
        Some("stash-ref:stash@{0}".to_owned());
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::VerbFieldNotApplicable { field, .. } if field == "stash_entry_ref"
    )));
}

#[test]
fn missing_target_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "stash-apply-0001").primary_target_ref = String::new();
    assert!(packet
        .validate()
        .iter()
        .any(|error| matches!(error, StashRecoveryValidationError::MissingTarget { .. })));
}

#[test]
fn missing_local_action_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "stash-apply-0001")
        .local_actions
        .retain(|action| action != "abort");
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::MissingLocalAction { action, .. } if action == "abort"
    )));
}

#[test]
fn recovery_verb_without_compare_action_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "reflog-restore-0001")
        .local_actions
        .retain(|action| action != "compare");
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        StashRecoveryValidationError::MissingLocalAction { action, .. } if action == "compare"
    )));
}

#[test]
fn duplicate_sheet_fails() {
    let mut packet = baseline();
    let dup = sheet(&packet, "stash-apply-0001").clone();
    packet.sheets.push(dup);
    assert!(packet
        .validate()
        .iter()
        .any(|error| matches!(error, StashRecoveryValidationError::DuplicateSheetId { .. })));
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
        StashRecoveryValidationError::SupportExportMissingField { .. }
    )));
}

#[test]
fn support_export_unredacted_fails() {
    let mut packet = baseline();
    packet.support_export.raw_patch_bodies_redacted = false;
    assert!(packet
        .validate()
        .contains(&StashRecoveryValidationError::SupportExportEmbedsRawMaterial));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    sheet_mut(&mut packet, "stash-apply-0001").summary_label = "leak bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&StashRecoveryValidationError::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_verb() {
    let summary = baseline().render_markdown_summary();
    for verb in StashRecoveryVerb::ALL {
        assert!(
            summary.contains(verb.as_str()),
            "summary missing verb {}",
            verb.as_str()
        );
    }
}
