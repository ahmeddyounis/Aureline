use super::*;

const PACKET_ID: &str = "review-component-accessibility:stable:0001";

fn trust_review() -> ReviewComponentAccessibilityTrustReview {
    ReviewComponentAccessibilityTrustReview {
        keyboard_reachable_on_every_claim: true,
        screen_reader_labeled_on_every_claim: true,
        cli_enum_exposed_on_every_claim: true,
        export_enum_exposed_on_every_claim: true,
        explanation_field_present_on_every_claim: true,
        no_component_pointer_only: true,
        no_component_export_opaque: true,
        desktop_never_stronger_than_cli: true,
        claim_narrows_when_provider_backing_weakens: true,
        provider_backed_never_overstated_under_weakening: true,
        browser_handoff_kept_explicit: true,
        local_continue_preserved_under_degraded_backing: true,
    }
}

fn projection() -> ReviewComponentAccessibilityProjection {
    ReviewComponentAccessibilityProjection {
        exposes_keyboard_and_screen_reader_labels: true,
        exposes_cli_and_export_enums: true,
        exposes_explanation_fields: true,
        auto_narrows_on_stale_freshness: true,
        auto_narrows_on_local_estimate_queue_authority: true,
        auto_narrows_on_missing_approval_lineage: true,
        auto_narrows_on_required_browser_handoff: true,
        desktop_cli_export_semantics_identical: true,
        narrowing_prevents_overstated_provider_truth: true,
        every_component_reachable_non_visually: true,
    }
}

fn proof_freshness() -> ReviewComponentAccessibilityProofFreshness {
    ReviewComponentAccessibilityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ReviewComponentAccessibilityDowngradeTrigger> {
    vec![
        ReviewComponentAccessibilityDowngradeTrigger::ProofStale,
        ReviewComponentAccessibilityDowngradeTrigger::ProviderFreshnessStale,
        ReviewComponentAccessibilityDowngradeTrigger::QueueAuthorityDroppedToLocalEstimate,
        ReviewComponentAccessibilityDowngradeTrigger::ApprovalLineageMissing,
        ReviewComponentAccessibilityDowngradeTrigger::BrowserHandoffRequired,
        ReviewComponentAccessibilityDowngradeTrigger::ClaimOverstated,
    ]
}

fn rendering_surfaces() -> Vec<ReviewComponentRenderingSurface> {
    ReviewComponentRenderingSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_DOC_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_REVIEW_REQUEST_ROW_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_CHECKS_SUMMARY_CARD_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_MERGE_READINESS_PANEL_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_ACCESSIBILITY_PENDING_REVIEW_TRAY_CONTRACT_REF.to_owned(),
    ]
}

fn row_refs(component: M5ReviewComponent) -> Vec<String> {
    vec![
        REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn human_component(component: M5ReviewComponent) -> &'static str {
    match component {
        M5ReviewComponent::ReviewRequestRow => "Review request row",
        M5ReviewComponent::ChecksSummaryCard => "Checks summary card",
        M5ReviewComponent::PendingReviewTray => "Pending review tray",
        M5ReviewComponent::MergeReadinessPanel => "Merge readiness panel",
        M5ReviewComponent::MergeQueueEntry => "Merge queue entry",
        M5ReviewComponent::StackDependencyChip => "Stack dependency chip",
        M5ReviewComponent::ApprovalInvalidationBanner => "Approval invalidation banner",
    }
}

fn claim_phrase(tier: ReviewComponentClaimTier) -> &'static str {
    match tier {
        ReviewComponentClaimTier::ProviderBacked => "provider-backed and current",
        ReviewComponentClaimTier::LocallyReviewable => {
            "reviewable locally while provider sync is stale"
        }
        ReviewComponentClaimTier::EstimateOnly => "a local estimate, not provider-owned ordering",
        ReviewComponentClaimTier::ApprovalUnverified => "shown with approval lineage unverified",
        ReviewComponentClaimTier::HandoffRequired => "available only via browser handoff",
    }
}

fn condition_phrase(condition: ReviewComponentClaimCondition) -> &'static str {
    match condition {
        ReviewComponentClaimCondition::ProviderFresh => "provider truth is fresh",
        ReviewComponentClaimCondition::ProviderFreshnessStale => "provider freshness is stale",
        ReviewComponentClaimCondition::QueueAuthorityLocalEstimate => {
            "queue authority dropped to a local estimate"
        }
        ReviewComponentClaimCondition::ApprovalLineageMissing => "approval lineage is missing",
        ReviewComponentClaimCondition::BrowserHandoffRequired => {
            "an out-of-scope action needs a browser handoff"
        }
    }
}

fn next_action_label(action: ReviewComponentClaimNextAction) -> String {
    match action {
        ReviewComponentClaimNextAction::RefreshProviderFreshness => {
            "Refresh provider freshness to restore the full claim".to_owned()
        }
        ReviewComponentClaimNextAction::ReconcileQueueAuthority => {
            "Reconcile queue authority against the provider".to_owned()
        }
        ReviewComponentClaimNextAction::RestoreApprovalLineage => {
            "Recompute the approval lineage before merging".to_owned()
        }
        ReviewComponentClaimNextAction::OpenBrowserHandoff => {
            "Open in browser to complete the out-of-scope action".to_owned()
        }
        ReviewComponentClaimNextAction::ContinueLocalReview => {
            "Continue reviewing locally while sync is degraded".to_owned()
        }
    }
}

/// Builds one accessibility row, deriving the claim, narrowing, notes, and labels
/// from the component and condition so the fixture stays self-consistent.
fn row(
    row_id: &str,
    component: M5ReviewComponent,
    condition: ReviewComponentClaimCondition,
) -> ReviewComponentAccessibilityRow {
    let resolution = resolve_review_component_claim_narrowing(condition);
    let effective_claim = resolution.permitted_ceiling;

    let narrowing = if resolution.requires_narrowing {
        Some(ReviewComponentClaimNarrowing {
            trigger: resolution
                .expected_trigger
                .expect("weakening condition has a trigger"),
            narrowed_to: resolution.permitted_ceiling,
            preserved_truth_note: format!(
                "{} stays keyboard-reachable, screen-reader labelled, and export-legible; only the provider-backed claim is narrowed",
                human_component(component)
            ),
            next_action: resolution.expected_next_action,
            next_action_label: next_action_label(resolution.expected_next_action),
        })
    } else {
        None
    };

    let local_continue_note = if resolution.needs_local_continue_note {
        format!(
            "Your local review of the {} continues here while provider backing is degraded",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let browser_handoff_note = if resolution.needs_browser_handoff_note {
        "Open in browser to complete this out-of-scope action; local review stays available here"
            .to_owned()
    } else {
        String::new()
    };

    ReviewComponentAccessibilityRow {
        row_id: row_id.to_owned(),
        component,
        condition,
        effective_claim,
        keyboard_label: format!(
            "{}: focusable, Enter opens, Space toggles detail",
            human_component(component)
        ),
        screen_reader_label: format!(
            "{}, {}",
            human_component(component),
            claim_phrase(effective_claim)
        ),
        cli_enum_token: format!("{}:{}", component.as_str(), effective_claim.as_str()),
        export_enum_token: effective_claim.as_str().to_owned(),
        explanation_field: format!(
            "{} — {}",
            claim_phrase(effective_claim),
            condition_phrase(condition)
        ),
        rendering_surfaces: rendering_surfaces(),
        narrowing,
        local_continue_note,
        browser_handoff_note,
        is_pointer_only: false,
        is_export_opaque: false,
        desktop_stronger_than_cli: false,
        source_contract_refs: row_refs(component),
    }
}

/// The canonical row set: all seven components, covering all five conditions and
/// all five claim tiers.
fn accessibility_rows() -> Vec<ReviewComponentAccessibilityRow> {
    vec![
        row(
            "row:rr-fresh",
            M5ReviewComponent::ReviewRequestRow,
            ReviewComponentClaimCondition::ProviderFresh,
        ),
        row(
            "row:cs-stale",
            M5ReviewComponent::ChecksSummaryCard,
            ReviewComponentClaimCondition::ProviderFreshnessStale,
        ),
        row(
            "row:mr-estimate",
            M5ReviewComponent::MergeReadinessPanel,
            ReviewComponentClaimCondition::QueueAuthorityLocalEstimate,
        ),
        row(
            "row:mq-handoff",
            M5ReviewComponent::MergeQueueEntry,
            ReviewComponentClaimCondition::BrowserHandoffRequired,
        ),
        row(
            "row:sd-fresh",
            M5ReviewComponent::StackDependencyChip,
            ReviewComponentClaimCondition::ProviderFresh,
        ),
        row(
            "row:ai-approval-missing",
            M5ReviewComponent::ApprovalInvalidationBanner,
            ReviewComponentClaimCondition::ApprovalLineageMissing,
        ),
        row(
            "row:pt-stale",
            M5ReviewComponent::PendingReviewTray,
            ReviewComponentClaimCondition::ProviderFreshnessStale,
        ),
    ]
}

fn packet_with(rows: Vec<ReviewComponentAccessibilityRow>) -> ReviewComponentAccessibilityPacket {
    ReviewComponentAccessibilityPacket::new(ReviewComponentAccessibilityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Review-component accessibility, headless, and export parity".to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: downgrade_triggers(),
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> ReviewComponentAccessibilityPacket {
    packet_with(accessibility_rows())
}

#[test]
fn accessibility_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_canonical_row_is_honest() {
    for row in accessibility_rows() {
        assert!(row.claim_is_honest(), "row not honest: {}", row.row_id);
    }
}

#[test]
fn claim_narrowing_maps_condition_to_ceiling() {
    let fresh =
        resolve_review_component_claim_narrowing(ReviewComponentClaimCondition::ProviderFresh);
    assert_eq!(
        fresh.permitted_ceiling,
        ReviewComponentClaimTier::ProviderBacked
    );
    assert!(!fresh.requires_narrowing);
    assert!(fresh.expected_trigger.is_none());
    assert!(!fresh.needs_local_continue_note);
    assert!(!fresh.needs_browser_handoff_note);

    let stale = resolve_review_component_claim_narrowing(
        ReviewComponentClaimCondition::ProviderFreshnessStale,
    );
    assert_eq!(
        stale.permitted_ceiling,
        ReviewComponentClaimTier::LocallyReviewable
    );
    assert!(stale.requires_narrowing);
    assert!(stale.needs_local_continue_note);
    assert!(!stale.needs_browser_handoff_note);

    let estimate = resolve_review_component_claim_narrowing(
        ReviewComponentClaimCondition::QueueAuthorityLocalEstimate,
    );
    assert_eq!(
        estimate.permitted_ceiling,
        ReviewComponentClaimTier::EstimateOnly
    );
    assert_eq!(
        estimate.expected_trigger,
        Some(ReviewComponentAccessibilityDowngradeTrigger::QueueAuthorityDroppedToLocalEstimate)
    );

    let approval = resolve_review_component_claim_narrowing(
        ReviewComponentClaimCondition::ApprovalLineageMissing,
    );
    assert_eq!(
        approval.permitted_ceiling,
        ReviewComponentClaimTier::ApprovalUnverified
    );

    let handoff = resolve_review_component_claim_narrowing(
        ReviewComponentClaimCondition::BrowserHandoffRequired,
    );
    assert_eq!(
        handoff.permitted_ceiling,
        ReviewComponentClaimTier::HandoffRequired
    );
    assert!(handoff.needs_browser_handoff_note);
    assert!(handoff.needs_local_continue_note);
}

// --- AC2: narrowing prevents overstated provider-backed truth -----------------

#[test]
fn provider_backed_claim_never_survives_a_weakening_condition() {
    // A component that keeps asserting the full provider-backed claim while provider
    // freshness is stale overstates its truth and must be caught.
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::ProviderFreshnessStale)
        .expect("stale row present");
    packet.accessibility_rows[index].effective_claim = ReviewComponentClaimTier::ProviderBacked;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn claim_ceiling_exceeded_on_local_estimate_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::QueueAuthorityLocalEstimate)
        .expect("estimate row present");
    // Claim locally-reviewable (rank 4) above the estimate-only ceiling (rank 3).
    packet.accessibility_rows[index].effective_claim = ReviewComponentClaimTier::LocallyReviewable;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn weakening_condition_without_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].narrowing = None;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ClaimNarrowingMissing));
}

#[test]
fn baseline_condition_with_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::ProviderFresh)
        .expect("fresh row present");
    packet.accessibility_rows[index].narrowing = Some(ReviewComponentClaimNarrowing {
        trigger: ReviewComponentAccessibilityDowngradeTrigger::ProviderFreshnessStale,
        narrowed_to: ReviewComponentClaimTier::ProviderBacked,
        preserved_truth_note: "note".to_owned(),
        next_action: ReviewComponentClaimNextAction::RefreshProviderFreshness,
        next_action_label: "Refresh".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ClaimNarrowingUnexpected));
}

#[test]
fn narrowed_to_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.narrowed_to = ReviewComponentClaimTier::HandoffRequired;
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::NarrowedToMismatch));
}

#[test]
fn narrow_trigger_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::ProviderFreshnessStale)
        .expect("stale row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.trigger = ReviewComponentAccessibilityDowngradeTrigger::BrowserHandoffRequired;
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::NarrowTriggerMismatch));
}

#[test]
fn narrow_next_action_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::BrowserHandoffRequired)
        .expect("handoff row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action = ReviewComponentClaimNextAction::ContinueLocalReview;
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::NarrowNextActionMismatch));
}

#[test]
fn narrow_missing_preserved_truth_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.preserved_truth_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::NarrowPreservedTruthMissing));
}

#[test]
fn narrow_missing_next_action_label_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action_label = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::NarrowNextActionMissing));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ---------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ExplanationFieldMissing));
}

#[test]
fn rendering_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].rendering_surfaces =
        vec![ReviewComponentRenderingSurface::DesktopFull];
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::RenderingSurfaceCoverageMissing));
}

#[test]
fn pointer_only_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::PointerOnlyComponent));
}

#[test]
fn export_opaque_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_export_opaque = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ExportOpaqueComponent));
}

#[test]
fn desktop_stronger_than_cli_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].desktop_stronger_than_cli = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::DesktopStrongerThanCli));
}

#[test]
fn browser_handoff_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == ReviewComponentClaimCondition::BrowserHandoffRequired)
        .expect("handoff row present");
    packet.accessibility_rows[index].browser_handoff_note = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::BrowserHandoffNoteMissing));
}

#[test]
fn local_continue_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::LocalContinueNoteMissing));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].source_contract_refs =
        vec![REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn missing_component_coverage_fails() {
    let mut rows = accessibility_rows();
    rows.retain(|r| r.component != M5ReviewComponent::StackDependencyChip);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_condition_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only browser-handoff row.
    rows.retain(|r| r.condition != ReviewComponentClaimCondition::BrowserHandoffRequired);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ConditionCoverageMissing));
}

#[test]
fn missing_claim_tier_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only estimate-only row; that tier is then unreachable.
    rows.retain(|r| r.effective_claim != ReviewComponentClaimTier::EstimateOnly);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ClaimTierCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.accessibility_rows.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::AccessibilityRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .provider_backed_never_overstated_under_weakening = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::TrustReviewIncomplete));
}

#[test]
fn projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .projection
        .narrowing_prevents_overstated_provider_truth = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ReviewComponentAccessibilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Accessibility rows"));
    assert!(summary.contains("review_request_row"));
    assert!(summary.contains("approval_invalidation_banner"));
    assert!(summary.contains("handoff_required"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_review_component_accessibility_export()
        .expect("checked review-component accessibility export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-component-accessibility-parity/provider_freshness_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-component-accessibility-parity/queue_authority_and_approval_lineage_narrowed.json"
        )),
    ] {
        let packet: ReviewComponentAccessibilityPacket = serde_json::from_str(raw)
            .expect("fixture parses as review-component accessibility packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// The canonical rows plus extra scenario rows that demonstrate a normally-fresh
/// component auto-narrowing under a weakening condition. The base rows keep full
/// component / condition / tier coverage; the extra rows show the narrowing.
fn fixture_provider_freshness_stale_auto_narrowed() -> ReviewComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:rr-stale-narrowed",
        M5ReviewComponent::ReviewRequestRow,
        ReviewComponentClaimCondition::ProviderFreshnessStale,
    ));
    rows.push(row(
        "row:sd-stale-narrowed",
        M5ReviewComponent::StackDependencyChip,
        ReviewComponentClaimCondition::ProviderFreshnessStale,
    ));
    ReviewComponentAccessibilityPacket::new(ReviewComponentAccessibilityPacketInput {
        packet_id: "review-component-accessibility:fixture:provider-freshness-stale-auto-narrowed"
            .to_owned(),
        surface_label:
            "Review-component accessibility: provider freshness stale, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            ReviewComponentAccessibilityDowngradeTrigger::ProviderFreshnessStale,
            ReviewComponentAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// The canonical rows plus extra scenario rows for a merge-queue entry dropping to a
/// local estimate and a review-request row losing its approval lineage.
fn fixture_queue_authority_and_approval_lineage_narrowed() -> ReviewComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:mq-estimate-narrowed",
        M5ReviewComponent::MergeQueueEntry,
        ReviewComponentClaimCondition::QueueAuthorityLocalEstimate,
    ));
    rows.push(row(
        "row:rr-approval-missing-narrowed",
        M5ReviewComponent::ReviewRequestRow,
        ReviewComponentClaimCondition::ApprovalLineageMissing,
    ));
    ReviewComponentAccessibilityPacket::new(ReviewComponentAccessibilityPacketInput {
        packet_id:
            "review-component-accessibility:fixture:queue-authority-and-approval-lineage-narrowed"
                .to_owned(),
        surface_label:
            "Review-component accessibility: queue authority local estimate and approval lineage missing"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            ReviewComponentAccessibilityDowngradeTrigger::QueueAuthorityDroppedToLocalEstimate,
            ReviewComponentAccessibilityDowngradeTrigger::ApprovalLineageMissing,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_REVIEW_COMPONENT_ACCESSIBILITY_ARTIFACTS` so it never writes
/// during a normal test run. Run with the env var set to refresh the artifacts after
/// a contract change, then review the diff.
#[test]
fn regenerate_review_component_accessibility_artifacts() {
    if std::env::var("GEN_REVIEW_COMPONENT_ACCESSIBILITY_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir = format!(
        "{root}/artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components"
    );
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-review-component-accessibility-parity");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "provider_freshness_stale_auto_narrowed.json",
            fixture_provider_freshness_stale_auto_narrowed(),
        ),
        (
            "queue_authority_and_approval_lineage_narrowed.json",
            fixture_queue_authority_and_approval_lineage_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
