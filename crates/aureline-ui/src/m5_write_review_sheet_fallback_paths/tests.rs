use super::*;

fn seed() -> M5WriteReviewSheetFallbackPathsPacket {
    seeded_m5_write_review_sheet_fallback_paths()
}

fn violations_of(packet: &M5WriteReviewSheetFallbackPathsPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_RECORD_KIND
    );
    assert_eq!(packet.review_bindings.len(), 15);
}

#[test]
fn all_five_fallback_paths_are_reviewable_before_commit() {
    // AC1: at least one duplicate, one detach, one overlay, one request-approval, and one regenerate-first path
    // can be reviewed before commit with explicit retained-versus-lost behaviour.
    let packet = seed();
    for action in WriteReviewFallbackAction::ALL {
        let full = packet.review_bindings.iter().find(|b| {
            b.fallback_action == action && b.posture == ReviewSheetPosture::FullReviewSheet
        });
        let full = full.unwrap_or_else(|| {
            panic!(
                "fallback action {} has no full review sheet",
                action.as_str()
            )
        });
        assert!(full.reviewed_before_commit);
        assert!(full
            .allowed_actions
            .contains(&WriteReviewAction::CommitReviewedTransition));
        assert!(
            full.review_content.preserved_versus_lost.is_explicit(),
            "fallback {} lacks explicit retained-versus-lost",
            action.as_str()
        );
        assert!(!full
            .review_content
            .preserved_versus_lost
            .retained
            .is_empty());
    }
}

#[test]
fn every_fallback_action_reviewed_by_two_or_more_flows() {
    let packet = seed();
    let mut action_flows: BTreeMap<
        WriteReviewFallbackAction,
        BTreeSet<WriteReviewOriginatingFlow>,
    > = BTreeMap::new();
    for binding in &packet.review_bindings {
        action_flows
            .entry(binding.fallback_action)
            .or_default()
            .insert(binding.originating_flow);
    }
    assert_eq!(action_flows.len(), 5, "all five fallback actions present");
    for (action, flows) in &action_flows {
        assert!(
            flows.len() >= 2,
            "fallback action {} only reviewed by {} flows",
            action.as_str(),
            flows.len()
        );
    }
}

#[test]
fn every_originating_flow_and_posture_is_exercised() {
    let packet = seed();
    let flows: BTreeSet<_> = packet
        .review_bindings
        .iter()
        .map(|b| b.originating_flow)
        .collect();
    for flow in WriteReviewOriginatingFlow::ALL {
        assert!(flows.contains(&flow), "flow {} missing", flow.as_str());
    }
    let postures: BTreeSet<_> = packet.review_bindings.iter().map(|b| b.posture).collect();
    for posture in ReviewSheetPosture::ALL {
        assert!(
            postures.contains(&posture),
            "posture {} missing",
            posture.as_str()
        );
    }
}

#[test]
fn recovery_class_is_visible_before_commit_on_every_path() {
    // AC3: recovery or undo class is visible before commit on every seeded fallback path.
    let packet = seed();
    for binding in &packet.review_bindings {
        assert!(
            binding.recovery_visible_before_commit,
            "binding {} does not expose recovery before commit",
            binding.binding_id
        );
        assert!(
            binding.checkpoint_matches_action(),
            "binding {} recovery class mismatches its fallback action",
            binding.binding_id
        );
    }
}

#[test]
fn no_constrained_write_silently_mutates_the_current_object() {
    // AC2: no constrained write path silently mutates the current object through a lossy fallback.
    let packet = seed();
    for binding in &packet.review_bindings {
        assert!(!binding.silently_mutates_current_object_through_lossy_fallback);
        assert!(binding.reviewed_before_commit);
        // No direct-write action can even be represented; the only write-capable action is the reviewed commit.
        for action in &binding.allowed_actions {
            assert!(matches!(
                action,
                WriteReviewAction::InspectWriteTarget
                    | WriteReviewAction::CopyPreservedVersusLost
                    | WriteReviewAction::RevealCanonicalSource
                    | WriteReviewAction::CommitReviewedTransition
            ));
        }
    }
}

#[test]
fn write_disposition_matches_action_and_is_write_constrained() {
    let packet = seed();
    for binding in &packet.review_bindings {
        assert!(
            binding.write_disposition_matches_action(),
            "binding {} write disposition mismatches action",
            binding.binding_id
        );
        assert!(binding
            .fallback_action
            .required_write_disposition()
            .is_write_constrained());
        assert!(binding.review_content.write_disposition_satisfied());
    }
}

#[test]
fn multi_state_objects_keep_every_state_visible() {
    let packet = seed();
    let multi: Vec<_> = packet
        .review_bindings
        .iter()
        .filter(|b| b.is_multi_state())
        .collect();
    assert!(!multi.is_empty(), "at least one multi-state binding");
    for binding in &multi {
        assert!(binding.multi_state_facets_consistent());
        assert_eq!(
            binding.co_applicable_states.len(),
            binding.review_content.co_applicable_state_labels.len()
        );
    }
    let has_generated_plus_policy = packet.review_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Generated
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::PolicyLocked)
    });
    let has_managed_plus_snapshot = packet.review_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Managed
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::CapturedSnapshot)
    });
    assert!(
        has_generated_plus_policy,
        "Generated + Policy locked present"
    );
    assert!(
        has_managed_plus_snapshot,
        "Managed + Captured snapshot present"
    );
}

#[test]
fn hidden_multi_state_facet_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| b.is_multi_state())
        .unwrap();
    packet.review_bindings[target]
        .review_content
        .co_applicable_state_labels
        .clear();
    assert!(violations_of(&packet).contains(&"multi_state_facet_hidden"));
}

#[test]
fn same_profile_carries_identical_content_across_flows() {
    let packet = seed();
    let mut profile_content: BTreeMap<&str, &WriteReviewSheetContent> = BTreeMap::new();
    for binding in &packet.review_bindings {
        match profile_content.get(binding.object_profile_id.as_str()) {
            None => {
                profile_content.insert(binding.object_profile_id.as_str(), &binding.review_content);
            }
            Some(existing) => assert_eq!(
                **existing, binding.review_content,
                "content drift on {}",
                binding.object_profile_id
            ),
        }
    }
    assert_eq!(profile_content.len(), 6);
}

#[test]
fn actions_are_safe_and_commit_matches_posture() {
    let packet = seed();
    for binding in &packet.review_bindings {
        assert!(binding.has_safe_base_actions());
        assert!(binding.commit_action_matches_posture());
        let offers = binding
            .allowed_actions
            .contains(&WriteReviewAction::CommitReviewedTransition);
        assert_eq!(offers, !binding.is_narrowed(), "on {}", binding.binding_id);
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.review_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn narrowed_bindings_disclose_and_full_bindings_do_not() {
    let packet = seed();
    for binding in &packet.review_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.parity_state,
                ReviewParityState::ContentDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_content_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(binding.parity_state, ReviewParityState::ContentPreserved);
            assert!(binding.narrow_note.is_none());
        }
        if matches!(binding.posture, ReviewSheetPosture::ExportRedacted) {
            assert!(!binding.export_detail_note.trim().is_empty());
        }
    }
}

#[test]
fn export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.review_bindings {
        if posture_must_reference_canonical(binding.posture) {
            assert!(
                binding.points_at_canonical_contracts(),
                "binding {} must point at canonical contracts",
                binding.binding_id
            );
        }
    }
}

#[test]
fn disclosure_resolver_matches_posture() {
    let full = resolve_review_render_disclosure(ReviewSheetPosture::FullReviewSheet);
    assert!(!full.needs_narrow_note);
    assert!(full.offers_reviewed_commit);

    let notice = resolve_review_render_disclosure(ReviewSheetPosture::PreconditionNoticeCompact);
    assert_eq!(
        notice.narrow_reason,
        Some(ReviewNarrowReason::CompactedToPreconditionNotice)
    );
    assert!(notice.needs_narrow_note);
    assert!(!notice.offers_reviewed_commit);

    let exported = resolve_review_render_disclosure(ReviewSheetPosture::ExportRedacted);
    assert!(exported.needs_export_detail_note);
    assert!(!exported.offers_reviewed_commit);
}

#[test]
fn content_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| b.binding_id == "wrs-generated-batch")
        .unwrap();
    packet.review_bindings[target]
        .review_content
        .write_target_word = "some_other_target".to_owned();
    assert!(violations_of(&packet).contains(&"review_content_drift_across_flows"));
}

#[test]
fn dropped_write_disposition_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0]
        .review_content
        .write_disposition_word = "directly_writable".to_owned();
    let v = violations_of(&packet);
    assert!(v.contains(&"write_disposition_missing_for_constrained_object"));
    assert!(v.contains(&"write_disposition_action_mismatch"));
}

#[test]
fn checkpoint_action_mismatch_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0]
        .review_content
        .checkpoint_undo_class = CheckpointUndoClass::OverlayPatchRevertible;
    assert!(violations_of(&packet).contains(&"checkpoint_action_mismatch"));
}

#[test]
fn not_reviewed_before_commit_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0].reviewed_before_commit = false;
    assert!(violations_of(&packet).contains(&"not_reviewed_before_commit"));
}

#[test]
fn recovery_not_visible_before_commit_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0].recovery_visible_before_commit = false;
    assert!(violations_of(&packet).contains(&"recovery_not_visible_before_commit"));
}

#[test]
fn commit_action_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.review_bindings[target]
        .allowed_actions
        .push(WriteReviewAction::CommitReviewedTransition);
    assert!(violations_of(&packet).contains(&"commit_action_posture_mismatch"));
}

#[test]
fn missing_safe_base_action_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0]
        .allowed_actions
        .retain(|a| *a != WriteReviewAction::CopyPreservedVersusLost);
    assert!(violations_of(&packet).contains(&"safe_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.review_bindings[0].accessibility_routes =
        vec![M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_flows_is_rejected() {
    let mut packet = seed();
    // Drop every RequestApproval binding except one, leaving that action reviewed by a single flow.
    let mut kept_one = false;
    packet.review_bindings.retain(|b| {
        if b.fallback_action == WriteReviewFallbackAction::RequestApproval {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"fallback_action_reuse_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| posture_must_reference_canonical(b.posture))
        .unwrap();
    packet.review_bindings[target].source_contract_refs =
        vec![M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"export_reference_missing"));
}

#[test]
fn missing_narrow_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.review_bindings[target].narrow_note = None;
    assert!(violations_of(&packet).contains(&"narrow_note_missing"));
}

#[test]
fn unexpected_narrow_note_on_full_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .review_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.review_bindings[target].narrow_note = Some(ReviewNarrowNote {
        reason: ReviewNarrowReason::CompactedToPreconditionNotice,
        preserved_content_note: "x".to_owned(),
        next_action: ReviewNarrowNextAction::OpenFullReviewSheet,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut WriteReviewSheetBinding), &str); 5] = [
        (
            |b| b.silently_mutates_current_object_through_lossy_fallback = true,
            "silently_mutates_current_object_through_lossy_fallback",
        ),
        (
            |b| b.gives_ai_automation_import_or_repair_flows_a_hidden_bypass = true,
            "gives_ai_automation_import_or_repair_flows_a_hidden_bypass",
        ),
        (
            |b| b.leaves_exact_write_target_or_preserved_versus_lost_sync_unstated = true,
            "leaves_exact_write_target_or_preserved_versus_lost_sync_unstated",
        ),
        (
            |b| b.hides_recovery_or_undo_class_before_commit = true,
            "hides_recovery_or_undo_class_before_commit",
        ),
        (
            |b| b.lets_one_state_class_hide_another_when_both_materially_affect_behavior = true,
            "lets_one_state_class_hide_another_when_both_materially_affect_behavior",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.review_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn fallback_action_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .review_bindings
        .retain(|b| b.fallback_action != WriteReviewFallbackAction::CreateOverlayPatch);
    assert!(violations_of(&packet).contains(&"fallback_action_coverage_missing"));
}

#[test]
fn originating_flow_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .review_bindings
        .retain(|b| b.originating_flow != WriteReviewOriginatingFlow::Repair);
    let v = violations_of(&packet);
    assert!(v.contains(&"originating_flow_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.review_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,co_applicable_states,fallback_action,originating_flow,posture,checkpoint_undo_class,parity_state"
    ));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.review_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn actor_parity_flows_are_recognized() {
    assert!(WriteReviewOriginatingFlow::AiApply.is_actor_parity_mutation_flow());
    assert!(WriteReviewOriginatingFlow::Importer.is_actor_parity_mutation_flow());
    assert!(WriteReviewOriginatingFlow::Repair.is_actor_parity_mutation_flow());
    assert!(!WriteReviewOriginatingFlow::DirectSave.is_actor_parity_mutation_flow());
    // Every actor-parity mutation flow is exercised by the seed, so no AI/automation/import/repair path is
    // missing its reviewed sheet.
    let packet = seed();
    let flows: BTreeSet<_> = packet
        .review_bindings
        .iter()
        .map(|b| b.originating_flow)
        .collect();
    for flow in WriteReviewOriginatingFlow::ALL {
        if flow.is_actor_parity_mutation_flow() {
            assert!(flows.contains(&flow));
        }
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_write_review_sheet_fallback_paths_export()
        .expect("checked M5 write-review-sheet fallback-path export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WRITE_REVIEW_SHEET_FALLBACK_PATHS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let notice = seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed();
    assert!(notice.validate().is_empty(), "{:?}", violations_of(&notice));
    assert_eq!(notice.review_bindings.len(), 15);

    let export = seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed();
    assert!(export.validate().is_empty(), "{:?}", violations_of(&export));
    assert_eq!(export.review_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let notice: M5WriteReviewSheetFallbackPathsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-write-review-sheet-fallback-paths/precondition_notice_narrowed.json"
    )))
    .expect("precondition-notice fixture parses");
    assert!(notice.validate().is_empty());
    assert_eq!(
        notice,
        seeded_m5_write_review_sheet_fallback_paths_precondition_notice_narrowed()
    );

    let export: M5WriteReviewSheetFallbackPathsPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-write-review-sheet-fallback-paths/export_redacted_narrowed.json"
    )))
        .expect("export-redacted fixture parses");
    assert!(export.validate().is_empty());
    assert_eq!(
        export,
        seeded_m5_write_review_sheet_fallback_paths_export_redacted_narrowed()
    );
}
