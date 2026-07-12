use super::*;

fn clean_diff_input() -> M5DiffViewResolutionInput {
    M5DiffViewResolutionInput {
        diff_id: "diff:test".to_owned(),
        hunk_label: "fn parse_header".to_owned(),
        change_kind: M5DiffChangeKind::Added,
        change_kind_stated: true,
        context_visibility: M5DiffContextVisibility::FullContext,
        moved_disclosed: true,
        hidden_context_disclosed: true,
        source_rendering: M5DiffSourceRendering::SourceExact,
        rendering_disclosed: true,
        hunk_identity: M5DiffHunkIdentity::StableHunkId,
        hunk_reidentification_disclosed: true,
        export_summary_structured: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_thread_input() -> M5ReviewThreadResolutionInput {
    M5ReviewThreadResolutionInput {
        thread_id: "thread:test".to_owned(),
        comment_label: "please add a test".to_owned(),
        thread_state: M5ReviewThreadState::Published,
        thread_state_stated: true,
        outdated_resolved_distinguished: true,
        anchor_durability: M5AnchorDurability::AnchoredExact,
        anchor_drift_disclosed: true,
        provider_locality: M5ReviewProviderLocality::ProviderHosted,
        provider_distinction_explicit: true,
        pending_send_disclosed: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_diff_review_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DIFF_REVIEW_CONTROLS_PACKET_ID);
}

#[test]
fn diff_clean_names_change_and_context_and_is_legible() {
    let resolved = resolve_diff_view(clean_diff_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.diff_legible_at_a_glance);
    assert!(resolved.change_kind_stated);
    assert_eq!(resolved.change_kind, "added");
    assert_eq!(resolved.context_visibility, "full_context");
    assert_eq!(resolved.source_rendering, "source_exact");
    assert_eq!(resolved.hunk_identity, "stable_hunk_id");
    assert!(resolved.hunk_is_stable);
    assert_eq!(
        resolved.next_action,
        M5DiffReviewNextAction::OpenComponentDetail
    );
}

#[test]
fn diff_identity_unstated_and_change_kind_collapsed_degrade() {
    let mut input = clean_diff_input();
    input.hunk_label = "   ".to_owned();
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::DiffIdentityUnstated)
    );

    let mut input = clean_diff_input();
    input.change_kind_stated = false;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::ChangeKindCollapsed)
    );
}

#[test]
fn diff_context_unresolved_and_moved_hidden_degrade() {
    let mut input = clean_diff_input();
    input.context_visibility = M5DiffContextVisibility::VisibilityUnresolved;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::ContextVisibilityUnresolved)
    );

    let mut input = clean_diff_input();
    input.context_visibility = M5DiffContextVisibility::MovedContext;
    input.moved_disclosed = false;
    let resolved = resolve_diff_view(input).unwrap();
    assert!(resolved.context_is_moved);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiffViewDegradeReason::MovedContextHidden)
    );
}

#[test]
fn diff_hidden_context_degrades_but_disclosed_is_clean() {
    let mut input = clean_diff_input();
    input.context_visibility = M5DiffContextVisibility::CollapsedContext;
    input.hidden_context_disclosed = false;
    let hidden = resolve_diff_view(input).unwrap();
    assert!(hidden.context_is_hidden);
    assert_eq!(
        hidden.degrade_reason,
        Some(M5DiffViewDegradeReason::HiddenContextNotDisclosed)
    );

    let mut input = clean_diff_input();
    input.context_visibility = M5DiffContextVisibility::ElidedContext;
    input.hidden_context_disclosed = true;
    let disclosed = resolve_diff_view(input).unwrap();
    assert!(disclosed.is_clean());
    assert!(disclosed.context_is_hidden);
}

#[test]
fn diff_source_rendering_unresolved_and_blurred_degrade() {
    let mut input = clean_diff_input();
    input.source_rendering = M5DiffSourceRendering::RenderingUnresolved;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::SourceRenderingUnresolved)
    );

    let mut input = clean_diff_input();
    input.source_rendering = M5DiffSourceRendering::RenderedTransformed;
    input.rendering_disclosed = false;
    let resolved = resolve_diff_view(input).unwrap();
    assert!(resolved.source_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiffViewDegradeReason::SourceVersusRenderedBlurred)
    );
}

#[test]
fn diff_hunk_identity_unresolved_and_drift_degrade() {
    let mut input = clean_diff_input();
    input.hunk_identity = M5DiffHunkIdentity::HunkIdUnresolved;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::HunkIdentityUnresolved)
    );

    let mut input = clean_diff_input();
    input.hunk_identity = M5DiffHunkIdentity::UnstableHunkId;
    input.hunk_reidentification_disclosed = false;
    let resolved = resolve_diff_view(input).unwrap();
    assert!(!resolved.hunk_is_stable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DiffViewDegradeReason::HunkIdentityDrifted)
    );

    // A re-identified hunk disclosed as such stays clean.
    let mut input = clean_diff_input();
    input.hunk_identity = M5DiffHunkIdentity::RebasedHunkId;
    input.hunk_reidentification_disclosed = true;
    assert!(resolve_diff_view(input).unwrap().is_clean());
}

#[test]
fn diff_opaque_summary_and_detail_missing_degrade() {
    let mut input = clean_diff_input();
    input.export_summary_structured = false;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::StructuralSummaryOpaque)
    );

    let mut input = clean_diff_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_diff_view(input).unwrap().degrade_reason,
        Some(M5DiffViewDegradeReason::DiffDetailPathMissing)
    );
}

#[test]
fn diff_empty_id_and_forbidden_material_error() {
    let mut input = clean_diff_input();
    input.diff_id = "".to_owned();
    assert_eq!(
        resolve_diff_view(input).unwrap_err(),
        M5DiffReviewResolutionError::EmptyDiffId
    );

    let mut input = clean_diff_input();
    input.hunk_label = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_diff_view(input).unwrap_err(),
        M5DiffReviewResolutionError::ForbiddenMaterial
    );
}

#[test]
fn thread_clean_names_state_and_is_legible() {
    let resolved = resolve_review_thread(clean_thread_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.thread_legible_at_a_glance);
    assert!(resolved.thread_state_stated);
    assert_eq!(resolved.thread_state, "published");
    assert_eq!(resolved.anchor_durability, "anchored_exact");
    assert_eq!(resolved.provider_locality, "provider_hosted");
    assert!(resolved.provider_is_hosted);
    assert_eq!(
        resolved.next_action,
        M5DiffReviewNextAction::ReviewThreadState
    );
}

#[test]
fn thread_draft_pending_and_locked_states_are_named() {
    for state in [
        M5ReviewThreadState::Draft,
        M5ReviewThreadState::PendingSend,
        M5ReviewThreadState::Locked,
        M5ReviewThreadState::ReAnchored,
    ] {
        let mut input = clean_thread_input();
        input.thread_state = state;
        let resolved = resolve_review_thread(input).unwrap();
        assert!(resolved.is_clean(), "{state:?} should resolve clean");
        assert_eq!(resolved.thread_state, state.as_str());
    }
}

#[test]
fn thread_identity_unstated_and_state_unresolved_degrade() {
    let mut input = clean_thread_input();
    input.comment_label = "   ".to_owned();
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ThreadIdentityUnstated)
    );

    let mut input = clean_thread_input();
    input.thread_state = M5ReviewThreadState::StateUnknown;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ThreadStateUnresolved)
    );
}

#[test]
fn thread_state_color_only_and_outdated_resolved_blurred_degrade() {
    let mut input = clean_thread_input();
    input.thread_state_stated = false;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ThreadStateEncodedByColorAlone)
    );

    let mut input = clean_thread_input();
    input.thread_state = M5ReviewThreadState::Outdated;
    input.outdated_resolved_distinguished = false;
    let resolved = resolve_review_thread(input).unwrap();
    assert!(resolved.is_outdated_or_resolved);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReviewThreadDegradeReason::OutdatedResolvedBlurred)
    );

    // An outdated thread that distinguishes itself from resolved stays clean.
    let mut input = clean_thread_input();
    input.thread_state = M5ReviewThreadState::Resolved;
    input.outdated_resolved_distinguished = true;
    assert!(resolve_review_thread(input).unwrap().is_clean());
}

#[test]
fn thread_anchor_unresolved_and_drift_hidden_degrade() {
    let mut input = clean_thread_input();
    input.anchor_durability = M5AnchorDurability::AnchorUnresolved;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::AnchorDurabilityUnresolved)
    );

    let mut input = clean_thread_input();
    input.anchor_durability = M5AnchorDurability::OutdatedAnchor;
    input.anchor_drift_disclosed = false;
    let resolved = resolve_review_thread(input).unwrap();
    assert!(resolved.anchor_is_drifted);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReviewThreadDegradeReason::AnchorDriftHidden)
    );
}

#[test]
fn thread_provider_locality_and_distinction_degrade() {
    let mut input = clean_thread_input();
    input.provider_locality = M5ReviewProviderLocality::LocalityUnresolved;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ProviderLocalityUnresolved)
    );

    let mut input = clean_thread_input();
    input.provider_distinction_explicit = false;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ProviderDistinctionImplicit)
    );
}

#[test]
fn thread_pending_send_hidden_degrades_but_disclosed_is_clean() {
    let mut input = clean_thread_input();
    input.thread_state = M5ReviewThreadState::Draft;
    input.pending_send_disclosed = false;
    let resolved = resolve_review_thread(input).unwrap();
    assert!(resolved.needs_send);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReviewThreadDegradeReason::PendingSendHidden)
    );

    let mut input = clean_thread_input();
    input.thread_state = M5ReviewThreadState::PendingSend;
    input.pending_send_disclosed = true;
    let disclosed = resolve_review_thread(input).unwrap();
    assert!(disclosed.is_clean());
    assert!(disclosed.needs_send);
}

#[test]
fn thread_detail_missing_degrades() {
    let mut input = clean_thread_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_review_thread(input).unwrap().degrade_reason,
        Some(M5ReviewThreadDegradeReason::ThreadDetailPathMissing)
    );
}

#[test]
fn thread_empty_id_and_forbidden_material_error() {
    let mut input = clean_thread_input();
    input.thread_id = "   ".to_owned();
    assert_eq!(
        resolve_review_thread(input).unwrap_err(),
        M5DiffReviewResolutionError::EmptyThreadId
    );

    let mut input = clean_thread_input();
    input.comment_label = "connect to internal://host".to_owned();
    assert_eq!(
        resolve_review_thread(input).unwrap_err(),
        M5DiffReviewResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_diff_review_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.vocabulary_set.review_thread_states.pop();
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REVIEW_THREAD_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DiffReviewAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5DiffReviewExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.controls_rows[0].thread_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    // Force a clean thread to also read as outdated-resolved-blurred — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.thread_examples[0].degrade_reason = None;
    row.thread_examples[0].is_outdated_or_resolved = true;
    row.thread_examples[0].outdated_resolved_distinguished = false;
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_diff_review_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.diff_moved_or_hidden_context_pretends_immutable_view = true,
            1 => row.diff_hunk_identity_or_source_rendering_silently_drifts = true,
            2 => row.review_outdated_and_resolved_state_blurred = true,
            _ => row.review_anchor_or_provider_locality_silently_drifts = true,
        }
        assert!(packet
            .validate()
            .contains(&M5DiffReviewControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn thread_state_grammar_not_proven_when_color_only_example_removed() {
    let mut packet = seeded_m5_diff_review_controls();
    for row in &mut packet.controls_rows {
        row.thread_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ReviewThreadDegradeReason::ThreadStateEncodedByColorAlone)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ThreadStateGrammarAndAnchorNotProven));
}

#[test]
fn thread_state_grammar_not_proven_when_localities_collapse() {
    let mut packet = seeded_m5_diff_review_controls();
    // Drop every clean thread that is not provider-hosted so the locality grammar collapses to one.
    for row in &mut packet.controls_rows {
        row.thread_examples
            .retain(|ex| !(ex.is_clean() && ex.provider_locality != "provider_hosted"));
    }
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ThreadStateGrammarAndAnchorNotProven));
}

#[test]
fn diff_context_honesty_not_proven_when_moved_hidden_removed() {
    let mut packet = seeded_m5_diff_review_controls();
    for row in &mut packet.controls_rows {
        row.diff_examples
            .retain(|ex| ex.degrade_reason != Some(M5DiffViewDegradeReason::MovedContextHidden));
    }
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::DiffContextHonestyNotProven));
}

#[test]
fn outdated_versus_resolved_not_proven_when_blurred_example_removed() {
    let mut packet = seeded_m5_diff_review_controls();
    for row in &mut packet.controls_rows {
        row.thread_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ReviewThreadDegradeReason::OutdatedResolvedBlurred)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::OutdatedVersusResolvedDistinctionNotProven));
}

#[test]
fn outdated_versus_resolved_not_proven_when_clean_lose_detail_path() {
    let mut packet = seeded_m5_diff_review_controls();
    for row in &mut packet.controls_rows {
        for d in &mut row.diff_examples {
            if d.is_clean() {
                d.detail_command_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&M5DiffReviewControlsViolation::OutdatedVersusResolvedDistinctionNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.governance_review.outdated_and_resolved_never_blurred = false;
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet
        .consumer_projection
        .browser_handoff_and_export_preserve_provider_locality = false;
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_diff_review_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DiffReviewControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_diff_review_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_diff_review_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_diff_review_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_diff_review_controls_export()
        .expect("checked M5 diff-view / review-thread controls export validates");
    assert_eq!(from_disk.packet_id, M5_DIFF_REVIEW_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_diff_review_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_diff_review_controls_diff_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::DiffUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Beta);

    let preview = seeded_m5_diff_review_controls_review_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5DiffReviewControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-diff-view-review-thread-controls/diff_ui_beta_narrowed.json"
    )))
    .expect("diff-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(beta, seeded_m5_diff_review_controls_diff_ui_beta_narrowed());

    let preview: M5DiffReviewControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-diff-view-review-thread-controls/review_ui_preview_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_diff_review_controls_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_diff_view_and_review_thread() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5EditorInlineComponentFamily::DiffView,
            M5EditorInlineComponentFamily::ReviewThread,
        ]
    );
}
