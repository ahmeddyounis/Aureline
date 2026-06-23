use super::*;
use crate::target_model::RenameApplyPosture;

#[test]
fn canonical_set_validates_and_freezes() {
    let set = rename_preview_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 6);
    assert!(!set.invariants.is_empty());
}

#[test]
fn clean_rename_is_fully_editable_yet_inspect_before_mutate() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.clean_editable")
        .expect("scenario present")
        .preview;
    assert_eq!(preview.totals.total_count, 3);
    assert_eq!(preview.totals.will_change_count, 3);
    assert_eq!(preview.totals.held_count(), 0);
    assert_eq!(preview.groups.len(), 1);
    assert_eq!(
        preview.groups[0].group_kind,
        RenameCandidateGroupKind::Editable
    );
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::ReadyForApplyAfterPreview
    );
    assert!(preview.apply_gate.apply_allowed_after_preview);
    // Even a clean rename never permits a blind apply.
    assert!(preview.apply_gate.inspect_before_mutate_required);
    assert!(preview.apply_gate.blind_apply_blocked);
    assert!(preview.apply_gate.preconditions.is_empty());
}

#[test]
fn blocked_generated_readonly_candidates_stay_visible() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.blocked_generated_readonly")
        .expect("scenario present")
        .preview;
    let blocked = preview
        .group(RenameCandidateGroupKind::BlockedForReview)
        .expect("blocked group");
    let generated = preview
        .group(RenameCandidateGroupKind::GeneratedBoundary)
        .expect("generated group");
    let readonly = preview
        .group(RenameCandidateGroupKind::ReadOnlyOrExternal)
        .expect("read-only group");
    assert!(blocked
        .omission_reasons
        .contains(&RenameOmissionReason::PolicyLimited));
    assert!(generated
        .omission_reasons
        .contains(&RenameOmissionReason::GeneratedBoundary));
    assert!(readonly
        .omission_reasons
        .contains(&RenameOmissionReason::ReadOnlyOrProtected));
    // Exactly one candidate changes; the rest are held but visible.
    assert_eq!(preview.totals.will_change_count, 1);
    assert_eq!(preview.totals.held_count(), 4);
    assert_eq!(blocked.counts.total_count, 2);
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::BlockedPendingPolicyOrProtectedReview
    );
    assert!(!preview.apply_gate.apply_allowed_after_preview);
    assert!(preview
        .apply_gate
        .preconditions
        .contains(&RenameApplyPrecondition::ReviewBlockedCandidates));
}

#[test]
fn conflict_notes_are_preserved() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.conflict_shadowing")
        .expect("scenario present")
        .preview;
    let conflict = preview
        .group(RenameCandidateGroupKind::Conflict)
        .expect("conflict group");
    assert_eq!(conflict.counts.total_count, 2);
    assert_eq!(conflict.conflict_notes.len(), 2);
    assert_eq!(preview.conflict_notes.len(), 2);
    assert!(conflict
        .omission_reasons
        .contains(&RenameOmissionReason::ConflictPendingResolution));
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::BlockedPendingScopeReview
    );
    assert!(preview
        .apply_gate
        .preconditions
        .contains(&RenameApplyPrecondition::ResolveConflicts));
}

#[test]
fn stale_and_unresolved_demand_refresh() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.stale_unresolved_refresh")
        .expect("scenario present")
        .preview;
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::BlockedPendingRefresh
    );
    assert!(preview
        .apply_gate
        .preconditions
        .contains(&RenameApplyPrecondition::RefreshStaleScope));
    // Current-versus-captured counts split: the imported/runtime/stale candidates are
    // captured-only, the degraded-cache editable one is current.
    assert!(preview.totals.captured_scope_count > 0);
    assert!(preview.totals.reconciles());
    assert_eq!(preview.totals.unresolved_count, 1);
    let partial = preview
        .group(RenameCandidateGroupKind::PartialScopeOmitted)
        .expect("partial group");
    assert!(partial
        .omission_reasons
        .contains(&RenameOmissionReason::UnresolvedAnchor));
    assert_eq!(
        preview.captured_scope_ref.as_deref(),
        Some("aureline://scope/captured-trace")
    );
}

#[test]
fn lexical_fallback_is_disclosed_never_renamed_as_semantic() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.fallback_sparse_visible")
        .expect("scenario present")
        .preview;
    let partial = preview
        .group(RenameCandidateGroupKind::PartialScopeOmitted)
        .expect("partial group");
    assert_eq!(partial.evidence_class, RenameEvidenceClass::LexicalFallback);
    assert!(partial.evidence_class.is_fallback());
    assert!(!partial.fallback_notes.is_empty());
    assert!(partial
        .downgrade_reasons
        .contains(&DowngradeReason::LexicalFallbackOnly));
    assert!(partial
        .omission_reasons
        .contains(&RenameOmissionReason::OutOfScopeSparse));
    assert!(preview
        .labels
        .contains(&RenameCandidateLabel::LexicalFallback));
    // The editable definition still applies after preview, omissions visible.
    assert_eq!(preview.totals.will_change_count, 1);
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::ReadyForApplyAfterPreview
    );
    assert!(preview
        .apply_gate
        .preconditions
        .contains(&RenameApplyPrecondition::WidenSparseScope));
}

#[test]
fn nothing_editable_is_inspect_only_not_empty() {
    let set = rename_preview_set();
    let preview = &set
        .scenario("rename.inspect_only_nothing_editable")
        .expect("scenario present")
        .preview;
    assert_eq!(preview.totals.will_change_count, 0);
    assert_eq!(preview.totals.total_count, 3);
    assert_eq!(
        preview.apply_gate.apply_posture,
        RenameApplyPosture::InspectOnlyUnavailable
    );
    assert!(!preview.apply_gate.apply_allowed_after_preview);
    // Nothing is editable, but every candidate is still listed in a group.
    assert_eq!(total_grouped(preview), 3);
    assert!(preview.group(RenameCandidateGroupKind::Editable).is_none());
}

#[test]
fn preview_set_projection_is_consistent() {
    let set = rename_preview_set();
    for scenario in &set.scenarios {
        let preview = &scenario.preview;
        assert_eq!(
            preview.preview_set.apply_posture,
            preview.apply_gate.apply_posture
        );
        assert_eq!(
            preview.preview_set.count_summary.changed_count,
            preview.totals.will_change_count
        );
        assert_eq!(
            preview.preview_set.candidate_occurrence_refs.len(),
            scenario.input.candidates.len()
        );
        assert_eq!(
            preview.preview_set.blocked_refs.len(),
            preview.totals.held_count()
        );
    }
}

#[test]
fn every_consumer_projection_preserves_truth() {
    let set = rename_preview_set();
    for scenario in &set.scenarios {
        for projection in &scenario.preview.consumer_projections {
            assert!(projection.preserves_truth());
            assert!(!projection.flattens_to_single_apply_action);
            assert!(!projection.exports_code_bodies);
            assert!(projection.omitted_candidates_remain_visible);
        }
    }
}

#[test]
fn undo_checkpoint_is_bound_for_every_preview() {
    let set = rename_preview_set();
    for scenario in &set.scenarios {
        let gate = &scenario.preview.apply_gate;
        assert!(gate
            .undo_checkpoint_ref
            .starts_with("aureline://undo/rename/"));
    }
}

#[test]
fn set_round_trips_through_json() {
    let set = rename_preview_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let round_trip: RenamePreviewGovernanceSet = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn drifted_preview_fails_validation() {
    let mut set = rename_preview_set();
    set.scenarios[0].preview.summary = "tampered".to_owned();
    assert!(set.validate().is_err());
}

#[test]
fn flattening_to_single_apply_action_breaks_consumer_invariant() {
    let mut set = rename_preview_set();
    set.scenarios[0].preview.consumer_projections[0].flattens_to_single_apply_action = true;
    // The stored preview no longer matches the builder output.
    assert!(set.validate().is_err());
}

#[test]
fn smuggling_a_blocked_candidate_into_editable_would_break_grouping() {
    // A blocked candidate must never be grouped as editable.
    let blocked = RenameCandidate {
        candidate_id: "cand.x".to_owned(),
        target_ref: "aureline://object/symbol.x".to_owned(),
        anchor_ref: "aureline://anchor/cand.x".to_owned(),
        access_kind: AccessKind::Read,
        scope_ref: "aureline://scope/workspace".to_owned(),
        generated_or_external_state: GeneratedOrExternalState::AuthoredSource,
        proof_class: ProofClass::DirectSemantic,
        provider_class: ProviderClass::LanguageServer,
        confidence: NavigationConfidence::Exact,
        freshness: FreshnessClass::AuthoritativeLive,
        scope_completeness: ScopeCompleteness::CompleteForDeclaredScope,
        block_reason: Some(RenameOmissionReason::PolicyLimited),
        conflict_note: None,
        anchor_resolved: true,
        downgrade_reasons: vec![],
        evidence_refs: vec!["aureline://evidence/cand.x".to_owned()],
        summary: "blocked".to_owned(),
    };
    assert_eq!(
        blocked.group_kind(),
        RenameCandidateGroupKind::BlockedForReview
    );
}
