use super::*;

const PACKET_ID: &str = "review-component-consumer:stable:0001";

fn trust_review() -> ReviewComponentConsumerTrustReview {
    ReviewComponentConsumerTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_object_same_language_across_surfaces: true,
        provider_state_never_flattened_to_local_estimate: true,
        approval_invalidation_never_hidden_behind_generic_pill: true,
        primary_actions_identical_across_surfaces: true,
        queue_readiness_status_language_identical_across_surfaces: true,
        browser_handoff_kept_explicit: true,
        local_continue_preserved_under_degraded_freshness: true,
        no_forced_raw_provider_navigation_for_triage: true,
        help_support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ReviewComponentConsumerProjection {
    ReviewComponentConsumerProjection {
        desktop_list_reuses_shared_components: true,
        detail_pane_reuses_shared_components: true,
        companion_triage_reuses_shared_components: true,
        help_surface_reuses_shared_components: true,
        support_export_reuses_shared_components: true,
        exported_evidence_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        parity_facets_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_preserves_provider_and_queue_identity: true,
    }
}

fn proof_freshness() -> ReviewComponentConsumerProofFreshness {
    ReviewComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ReviewComponentConsumerDowngradeTrigger> {
    vec![
        ReviewComponentConsumerDowngradeTrigger::ProofStale,
        ReviewComponentConsumerDowngradeTrigger::ProviderFreshnessStale,
        ReviewComponentConsumerDowngradeTrigger::ApprovalInvalidationPending,
        ReviewComponentConsumerDowngradeTrigger::BrowserHandoffUnavailable,
        ReviewComponentConsumerDowngradeTrigger::ParityDriftDetected,
        ReviewComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ReviewComponentConsumer> {
    ReviewComponentConsumer::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REVIEW_COMPONENT_CONSUMER_SCHEMA_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_DOC_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_REVIEW_REQUEST_ROW_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_CHECKS_SUMMARY_CARD_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_MERGE_READINESS_PANEL_CONTRACT_REF.to_owned(),
        REVIEW_COMPONENT_CONSUMER_PENDING_REVIEW_TRAY_CONTRACT_REF.to_owned(),
    ]
}

fn binding_refs(component: M5ReviewComponent) -> Vec<String> {
    vec![
        REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

/// Builds one binding, deriving render mode, parity state, narrow banner, and
/// disclosure notes from the object's provider freshness so the fixture stays
/// self-consistent by construction.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    object_id: &str,
    object_label: &str,
    component: M5ReviewComponent,
    consumer: ReviewComponentConsumer,
    freshness: M5ReviewComponentStaleProviderState,
    facets: &ReviewComponentParityFacetValues,
) -> ReviewComponentConsumerBinding {
    let disclosure = resolve_review_component_render_disclosure(freshness);

    let narrow_banner = disclosure.narrow_reason.map(|reason| {
        let (next_action, next_action_label) = match reason {
            ReviewComponentNarrowReason::ProviderFreshnessDegraded => (
                ReviewComponentNarrowNextAction::RefreshProviderFreshness,
                "Refresh provider freshness to restore full parity".to_owned(),
            ),
            ReviewComponentNarrowReason::BrowserHandoffRequired => (
                ReviewComponentNarrowNextAction::OpenBrowserHandoff,
                "Open in browser to inspect the hosted review".to_owned(),
            ),
            ReviewComponentNarrowReason::LocalContinueEngaged => (
                ReviewComponentNarrowNextAction::ContinueLocalReview,
                "Continue reviewing locally while sync is degraded".to_owned(),
            ),
        };
        ReviewComponentNarrowBanner {
            reason,
            preserved_facets_note:
                "Label, action, and status language are preserved; only freshness narrowed"
                    .to_owned(),
            next_action,
            next_action_label,
        }
    });

    let local_continue_note = if disclosure.needs_local_continue_note {
        "Your local review continues here while provider freshness is degraded".to_owned()
    } else {
        String::new()
    };
    let browser_handoff_boundary = if disclosure.needs_browser_handoff_boundary {
        "Provider is unreachable; open in browser to inspect the hosted review".to_owned()
    } else {
        String::new()
    };

    ReviewComponentConsumerBinding {
        binding_id: binding_id.to_owned(),
        review_object_id: object_id.to_owned(),
        review_object_label: object_label.to_owned(),
        component,
        consumer,
        provider_freshness: freshness,
        render_mode: disclosure.expected_mode,
        parity_facets: facets.clone(),
        parity_state: parity_state_for_mode(disclosure.expected_mode),
        narrow_banner,
        local_continue_note,
        browser_handoff_boundary,
        forces_raw_provider_navigation_for_triage: false,
        flattens_provider_state_into_local_estimate: false,
        hides_approval_invalidation_behind_generic_pill: false,
        rewords_labels_per_surface: false,
        drops_handoff_reason_or_local_continue: false,
        source_contract_refs: binding_refs(component),
    }
}

fn facets(
    label: &str,
    action: &str,
    status: &str,
    handoff: &str,
) -> ReviewComponentParityFacetValues {
    ReviewComponentParityFacetValues {
        label: label.to_owned(),
        primary_action: action.to_owned(),
        queue_readiness_status_language: status.to_owned(),
        handoff_reason: handoff.to_owned(),
    }
}

/// The canonical binding set: seven components, each adopted by >= 2 consumers,
/// covering all six consumer surfaces. Objects sharing an id share parity facets.
fn consumer_bindings() -> Vec<ReviewComponentConsumerBinding> {
    // Object 1: review-request row, fresh, on list + detail + exported evidence.
    let rr = facets(
        "Add retry backoff to queue worker",
        "Open review",
        "Ready to review · 2 of 3 approvals",
        "in_product_no_handoff",
    );
    // Object 2: checks-summary card, stale, on list + detail.
    let cs = facets(
        "CI checks: 6 required, 1 optional",
        "Open checks",
        "1 required check pending · provider stale",
        "in_product_no_handoff",
    );
    // Object 3: pending-review tray, local-only, on companion + detail.
    let pt = facets(
        "You still owe a review",
        "Submit your review",
        "Awaiting your review · working locally",
        "in_product_no_handoff",
    );
    // Object 4: merge-readiness panel, fresh, on detail + support.
    let mr = facets(
        "Ready to merge once base updates",
        "Update base",
        "Blocked: base is behind by 3 commits",
        "in_product_no_handoff",
    );
    // Object 5: merge-queue entry, refreshing, on list + companion.
    let mq = facets(
        "Queued behind 2 changes",
        "View queue position",
        "Position 3 · queue managed by provider",
        "in_product_no_handoff",
    );
    // Object 6: stack-dependency chip, fresh, on detail + help.
    let sd = facets(
        "Depends on parent PR #4810",
        "Open parent change",
        "Parent merged · this change is unblocked",
        "in_product_no_handoff",
    );
    // Object 7: approval-invalidation banner, unreachable, on list + detail + support.
    let ai = facets(
        "Approvals invalidated: stale base",
        "Compare invalidating change",
        "Prior approvals reset · recompute required",
        "Provider unreachable; open browser to inspect hosted approvals",
    );

    vec![
        binding(
            "bind:rr-4821:list",
            "obj:rr-4821",
            "PR #4821",
            M5ReviewComponent::ReviewRequestRow,
            ReviewComponentConsumer::DesktopList,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &rr,
        ),
        binding(
            "bind:rr-4821:detail",
            "obj:rr-4821",
            "PR #4821",
            M5ReviewComponent::ReviewRequestRow,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &rr,
        ),
        binding(
            "bind:rr-4821:export",
            "obj:rr-4821",
            "PR #4821",
            M5ReviewComponent::ReviewRequestRow,
            ReviewComponentConsumer::ExportedEvidence,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &rr,
        ),
        binding(
            "bind:cs-3310:list",
            "obj:cs-3310",
            "Run #3310",
            M5ReviewComponent::ChecksSummaryCard,
            ReviewComponentConsumer::DesktopList,
            M5ReviewComponentStaleProviderState::ProviderStale,
            &cs,
        ),
        binding(
            "bind:cs-3310:detail",
            "obj:cs-3310",
            "Run #3310",
            M5ReviewComponent::ChecksSummaryCard,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::ProviderStale,
            &cs,
        ),
        binding(
            "bind:pt-771:companion",
            "obj:pt-771",
            "Review #771",
            M5ReviewComponent::PendingReviewTray,
            ReviewComponentConsumer::CompanionTriage,
            M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            &pt,
        ),
        binding(
            "bind:pt-771:detail",
            "obj:pt-771",
            "Review #771",
            M5ReviewComponent::PendingReviewTray,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            &pt,
        ),
        binding(
            "bind:mr-559:detail",
            "obj:mr-559",
            "Candidate #559",
            M5ReviewComponent::MergeReadinessPanel,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &mr,
        ),
        binding(
            "bind:mr-559:support",
            "obj:mr-559",
            "Candidate #559",
            M5ReviewComponent::MergeReadinessPanel,
            ReviewComponentConsumer::SupportExport,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &mr,
        ),
        binding(
            "bind:mq-88:list",
            "obj:mq-88",
            "Queue entry #88",
            M5ReviewComponent::MergeQueueEntry,
            ReviewComponentConsumer::DesktopList,
            M5ReviewComponentStaleProviderState::ProviderRefreshing,
            &mq,
        ),
        binding(
            "bind:mq-88:companion",
            "obj:mq-88",
            "Queue entry #88",
            M5ReviewComponent::MergeQueueEntry,
            ReviewComponentConsumer::CompanionTriage,
            M5ReviewComponentStaleProviderState::ProviderRefreshing,
            &mq,
        ),
        binding(
            "bind:sd-12:detail",
            "obj:sd-12",
            "Stack #12",
            M5ReviewComponent::StackDependencyChip,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &sd,
        ),
        binding(
            "bind:sd-12:help",
            "obj:sd-12",
            "Stack #12",
            M5ReviewComponent::StackDependencyChip,
            ReviewComponentConsumer::HelpSurface,
            M5ReviewComponentStaleProviderState::ProviderFresh,
            &sd,
        ),
        binding(
            "bind:ai-90:list",
            "obj:ai-90",
            "PR #90",
            M5ReviewComponent::ApprovalInvalidationBanner,
            ReviewComponentConsumer::DesktopList,
            M5ReviewComponentStaleProviderState::ProviderUnreachable,
            &ai,
        ),
        binding(
            "bind:ai-90:detail",
            "obj:ai-90",
            "PR #90",
            M5ReviewComponent::ApprovalInvalidationBanner,
            ReviewComponentConsumer::DetailPane,
            M5ReviewComponentStaleProviderState::ProviderUnreachable,
            &ai,
        ),
        binding(
            "bind:ai-90:support",
            "obj:ai-90",
            "PR #90",
            M5ReviewComponent::ApprovalInvalidationBanner,
            ReviewComponentConsumer::SupportExport,
            M5ReviewComponentStaleProviderState::ProviderUnreachable,
            &ai,
        ),
    ]
}

fn packet_with(bindings: Vec<ReviewComponentConsumerBinding>) -> ReviewComponentConsumerPacket {
    ReviewComponentConsumerPacket::new(ReviewComponentConsumerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Shared review-component consumers".to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> ReviewComponentConsumerPacket {
    packet_with(consumer_bindings())
}

#[test]
fn consumer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn render_disclosure_maps_freshness_to_mode() {
    let fresh = resolve_review_component_render_disclosure(
        M5ReviewComponentStaleProviderState::ProviderFresh,
    );
    assert_eq!(fresh.expected_mode, ReviewComponentRenderMode::FullParity);
    assert!(!fresh.needs_narrow_banner);
    assert!(!fresh.needs_local_continue_note);
    assert!(!fresh.needs_browser_handoff_boundary);

    let stale = resolve_review_component_render_disclosure(
        M5ReviewComponentStaleProviderState::ProviderStale,
    );
    assert_eq!(
        stale.expected_mode,
        ReviewComponentRenderMode::FreshnessNarrowed
    );
    assert!(stale.needs_narrow_banner);
    assert!(stale.needs_local_continue_note);
    assert!(!stale.needs_browser_handoff_boundary);

    let refreshing = resolve_review_component_render_disclosure(
        M5ReviewComponentStaleProviderState::ProviderRefreshing,
    );
    assert_eq!(
        refreshing.expected_mode,
        ReviewComponentRenderMode::FreshnessNarrowed
    );
    assert!(refreshing.needs_narrow_banner);
    // Plain refreshing does not force a local-continue path.
    assert!(!refreshing.needs_local_continue_note);

    let unreachable = resolve_review_component_render_disclosure(
        M5ReviewComponentStaleProviderState::ProviderUnreachable,
    );
    assert_eq!(
        unreachable.expected_mode,
        ReviewComponentRenderMode::HandoffRequired
    );
    assert!(unreachable.needs_browser_handoff_boundary);
    assert!(unreachable.needs_local_continue_note);

    let local = resolve_review_component_render_disclosure(
        M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
    );
    assert_eq!(
        local.expected_mode,
        ReviewComponentRenderMode::LocalContinueFallback
    );
    assert!(local.needs_local_continue_note);
    assert!(!local.needs_browser_handoff_boundary);
}

#[test]
fn parity_drift_across_surfaces_fails() {
    let mut packet = packet();
    // Reword the label on one surface for a shared object; the other surfaces disagree.
    packet.consumer_bindings[1].parity_facets.label =
        "Reworded label for the detail pane".to_owned();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn primary_action_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[2].parity_facets.primary_action = "Different action".to_owned();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn component_reuse_by_single_consumer_fails() {
    let mut bindings = consumer_bindings();
    // Drop the second stack-dependency-chip binding so it is adopted by one consumer.
    bindings.retain(|b| b.binding_id != "bind:sd-12:help");
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ReviewComponentReuseUnproven));
}

#[test]
fn missing_component_coverage_fails() {
    let mut bindings = consumer_bindings();
    bindings.retain(|b| b.component != M5ReviewComponent::StackDependencyChip);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ComponentCoverageMissing));
}

#[test]
fn missing_consumer_coverage_fails() {
    let mut bindings = consumer_bindings();
    // Remove the only Help-surface binding.
    bindings.retain(|b| b.consumer != ReviewComponentConsumer::HelpSurface);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ConsumerCoverageMissing));
}

#[test]
fn help_support_export_without_canonical_refs_fails() {
    let mut packet = packet();
    // The exported-evidence binding drops its canonical component ref.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.consumer == ReviewComponentConsumer::ExportedEvidence)
        .expect("exported-evidence binding present");
    packet.consumer_bindings[index].source_contract_refs =
        vec![REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn render_mode_mismatch_fails() {
    let mut packet = packet();
    // Claim full parity on a stale-provider checks card.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.provider_freshness == M5ReviewComponentStaleProviderState::ProviderStale)
        .expect("stale binding present");
    packet.consumer_bindings[index].render_mode = ReviewComponentRenderMode::FullParity;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::RenderModeMismatch));
}

#[test]
fn parity_state_mismatch_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_state = ReviewComponentParityState::FacetsDisclosedNarrowed;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ParityStateMismatch));
}

#[test]
fn narrowed_binding_without_banner_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    packet.consumer_bindings[index].narrow_banner = None;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn full_parity_binding_with_banner_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].narrow_banner = Some(ReviewComponentNarrowBanner {
        reason: ReviewComponentNarrowReason::ProviderFreshnessDegraded,
        preserved_facets_note: "note".to_owned(),
        next_action: ReviewComponentNarrowNextAction::RefreshProviderFreshness,
        next_action_label: "Refresh".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn narrow_reason_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.provider_freshness == M5ReviewComponentStaleProviderState::ProviderStale)
        .expect("stale binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.reason = ReviewComponentNarrowReason::BrowserHandoffRequired;
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::NarrowReasonMismatch));
}

#[test]
fn narrow_banner_missing_preserved_facets_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.preserved_facets_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::NarrowBannerPreservedFacetsMissing));
}

#[test]
fn local_continue_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.provider_freshness == M5ReviewComponentStaleProviderState::LocalOnlyContinuation
        })
        .expect("local-only binding present");
    packet.consumer_bindings[index].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::LocalContinueNoteMissing));
}

#[test]
fn browser_handoff_boundary_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.provider_freshness == M5ReviewComponentStaleProviderState::ProviderUnreachable
        })
        .expect("unreachable binding present");
    packet.consumer_bindings[index].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::BrowserHandoffBoundaryMissing));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].forces_raw_provider_navigation_for_triage = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ForcedRawProviderNavigation));
}

#[test]
fn provider_state_flattened_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].flattens_provider_state_into_local_estimate = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ProviderStateFlattenedToLocalEstimate));
}

#[test]
fn approval_invalidation_hidden_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].hides_approval_invalidation_behind_generic_pill = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ApprovalInvalidationHiddenBehindGenericPill));
}

#[test]
fn labels_reworded_per_surface_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].rewords_labels_per_surface = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::LabelsRewordedPerSurface));
}

#[test]
fn handoff_or_local_continue_dropped_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].drops_handoff_reason_or_local_continue = true;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::HandoffOrLocalContinueDropped));
}

#[test]
fn parity_facet_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_facets.handoff_reason = String::new();
    // Rewording one surface also trips drift; assert the incomplete facet is reported.
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ParityFacetIncomplete));
}

#[test]
fn incomplete_binding_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].review_object_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::BindingIncomplete));
}

#[test]
fn missing_bindings_fails() {
    let mut packet = packet();
    packet.consumer_bindings.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ConsumerBindingsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .same_object_same_language_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .every_component_adopted_by_two_or_more_consumers = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ReviewComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_bindings() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Consumer bindings"));
    assert!(summary.contains("review_request_row"));
    assert!(summary.contains("approval_invalidation_banner"));
    assert!(summary.contains("handoff_required"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_review_component_consumer_export()
        .expect("checked review-component consumer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-component-consumers/degraded_provider_freshness_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-component-consumers/provider_unreachable_browser_handoff.json"
        )),
    ] {
        let packet: ReviewComponentConsumerPacket =
            serde_json::from_str(raw).expect("fixture parses as review-component consumer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// Re-derives the canonical bindings after overriding some objects' freshness,
/// keeping the parity facets identical per object so the packet still validates.
fn bindings_with_freshness_overrides(
    overrides: &[(&str, M5ReviewComponentStaleProviderState)],
) -> Vec<ReviewComponentConsumerBinding> {
    consumer_bindings()
        .into_iter()
        .map(|mut existing| {
            if let Some((_, freshness)) = overrides
                .iter()
                .find(|(object_id, _)| *object_id == existing.review_object_id)
            {
                existing = binding(
                    &existing.binding_id,
                    &existing.review_object_id,
                    &existing.review_object_label,
                    existing.component,
                    existing.consumer,
                    *freshness,
                    &existing.parity_facets,
                );
            }
            existing
        })
        .collect()
}

fn fixture_degraded_provider_freshness_narrowed() -> ReviewComponentConsumerPacket {
    let bindings = bindings_with_freshness_overrides(&[
        (
            "obj:rr-4821",
            M5ReviewComponentStaleProviderState::ProviderStale,
        ),
        (
            "obj:mr-559",
            M5ReviewComponentStaleProviderState::ProviderConflict,
        ),
    ]);
    ReviewComponentConsumerPacket::new(ReviewComponentConsumerPacketInput {
        packet_id: "review-component-consumer:fixture:degraded-freshness-narrowed".to_owned(),
        surface_label: "Shared review-component consumers: degraded provider, freshness narrowed"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            ReviewComponentConsumerDowngradeTrigger::ProviderFreshnessStale,
            ReviewComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn fixture_provider_unreachable_browser_handoff() -> ReviewComponentConsumerPacket {
    let bindings = bindings_with_freshness_overrides(&[
        (
            "obj:mr-559",
            M5ReviewComponentStaleProviderState::ProviderUnreachable,
        ),
        (
            "obj:sd-12",
            M5ReviewComponentStaleProviderState::ProviderUnreachable,
        ),
    ]);
    ReviewComponentConsumerPacket::new(ReviewComponentConsumerPacketInput {
        packet_id: "review-component-consumer:fixture:provider-unreachable-browser-handoff"
            .to_owned(),
        surface_label: "Shared review-component consumers: provider unreachable, browser handoff"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            ReviewComponentConsumerDowngradeTrigger::BrowserHandoffUnavailable,
            ReviewComponentConsumerDowngradeTrigger::ProviderFreshnessStale,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_REVIEW_COMPONENT_CONSUMER_ARTIFACTS` so it never writes during
/// a normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_review_component_consumer_artifacts() {
    if std::env::var("GEN_REVIEW_COMPONENT_CONSUMER_ARTIFACTS").is_err() {
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
        "{root}/artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity"
    );
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-review-component-consumers");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "degraded_provider_freshness_narrowed.json",
            fixture_degraded_provider_freshness_narrowed(),
        ),
        (
            "provider_unreachable_browser_handoff.json",
            fixture_provider_unreachable_browser_handoff(),
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
