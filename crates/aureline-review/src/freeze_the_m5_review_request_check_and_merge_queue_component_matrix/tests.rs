use super::*;

const PACKET_ID: &str = "m5-review-component-matrix:stable:0001";

fn component_rows() -> Vec<M5ReviewComponentMatrixRow> {
    vec![
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::ReviewRequestRow,
            maturity: M5ReviewComponentMaturityClass::Stable,
            scope_summary: "Review-request row listing a provider-linked pull/merge request with provider identity, base/head relation, and freshness always explicit".to_owned(),
            provider_local_distinction: "Provider-authored title, author, and review state are labeled provider-backed; base/head and diff summary are local-computed and never presented as provider truth".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderRefreshing,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            ],
            browser_handoff_boundary: "Opening the request on the provider host is an explicit browser handoff with a labeled return path to the local review workspace".to_owned(),
            local_continue_fallback: "When provider state is stale the row keeps last-known provider fields labeled and lets local diff review continue offline".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:review-request-row-provider-identity:m5".to_owned(),
                "evidence:review-request-row-base-head-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::ProviderFreshnessStale,
                M5ReviewComponentDowngradeTrigger::TrustNarrowing,
                M5ReviewComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_REVIEW_REQUEST_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::ReviewWorkspace,
                M5ReviewComponentConsumerSurface::CliHeadless,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::ChecksSummaryCard,
            maturity: M5ReviewComponentMaturityClass::Stable,
            scope_summary: "Checks-summary card rolling up check-class truth and freshness without collapsing distinct check classes into one status pill".to_owned(),
            provider_local_distinction: "Provider-reported check verdicts are labeled provider-backed; locally re-run check results are labeled local and never merged silently into the provider roll-up".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderRefreshing,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::ProviderConflict,
            ],
            browser_handoff_boundary: "Opening full check logs on the provider is an explicit browser handoff; ordinary triage stays in-product and never forces raw-provider navigation".to_owned(),
            local_continue_fallback: "When provider check status is stale the card shows last-known verdicts labeled stale and offers local re-run to continue triage".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:checks-summary-check-class-truth:m5".to_owned(),
                "evidence:checks-summary-freshness:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::CheckClassUnverified,
                M5ReviewComponentDowngradeTrigger::ProviderFreshnessStale,
                M5ReviewComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_CHECKS_SUMMARY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::ReviewWorkspace,
                M5ReviewComponentConsumerSurface::CliHeadless,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::PendingReviewTray,
            maturity: M5ReviewComponentMaturityClass::Stable,
            scope_summary: "Pending-review tray listing reviews awaiting the current owner with owner identity and local-versus-provider origin always explicit".to_owned(),
            provider_local_distinction: "Provider-assigned review requests are labeled provider-backed; locally queued publish-later reviews are labeled local and never counted as provider-confirmed".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderRefreshing,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            ],
            browser_handoff_boundary: "Opening a pending review on the provider is an explicit browser handoff with a labeled return path to the tray".to_owned(),
            local_continue_fallback: "When provider assignment state is stale the tray keeps last-known items labeled stale and lets locally queued reviews continue offline".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:pending-review-tray-owner-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::ProviderFreshnessStale,
                M5ReviewComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::LocalContinuePreserved,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_PENDING_TRAY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::PendingReviewTray,
                M5ReviewComponentConsumerSurface::CliHeadless,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::MergeReadinessPanel,
            maturity: M5ReviewComponentMaturityClass::Stable,
            scope_summary: "Merge-readiness panel summarizing blocking state, required checks, and ownership; blocking reasons stay explicit rather than collapsed into a single ready/not-ready pill".to_owned(),
            provider_local_distinction: "Provider-enforced branch policy and required-check gates are labeled provider-backed; local merge-readiness estimate is labeled local and never presented as the provider's final gate".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::ProviderUnreachable,
                M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            ],
            browser_handoff_boundary: "Resolving a provider-enforced block on the host is an explicit browser handoff with a labeled return path to the panel".to_owned(),
            local_continue_fallback: "When provider policy state is stale the panel shows last-known gates labeled stale and continues local readiness estimation without asserting provider approval".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-readiness-blocking-truth:m5".to_owned(),
                "evidence:merge-readiness-ownership:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::ProviderFreshnessStale,
                M5ReviewComponentDowngradeTrigger::ApprovalInvalidated,
                M5ReviewComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_MERGE_READINESS_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::MergeReadinessPanel,
                M5ReviewComponentConsumerSurface::CliHeadless,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::MergeQueueEntry,
            maturity: M5ReviewComponentMaturityClass::Stable,
            scope_summary: "Merge-queue entry with queue owner, position, and gating-check truth; provider-managed queue state is never flattened into a local estimate".to_owned(),
            provider_local_distinction: "Provider-managed queue position and owner are labeled provider-backed; local pre-merge estimate is labeled local and never presented as the queue's authoritative position".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderRefreshing,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::ProviderUnreachable,
            ],
            browser_handoff_boundary: "Managing the queue entry on the provider is an explicit browser handoff with a labeled return path to the merge-queue panel".to_owned(),
            local_continue_fallback: "When queue status is stale the entry shows last-known position and owner labeled stale and continues local review without asserting a fresh queue position".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-queue-entry-owner-truth:m5".to_owned(),
                "evidence:merge-queue-entry-position-freshness:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::QueueOwnershipUnresolved,
                M5ReviewComponentDowngradeTrigger::ProviderFreshnessStale,
                M5ReviewComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::ProviderMutationAttributable,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_MERGE_QUEUE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::MergeQueuePanel,
                M5ReviewComponentConsumerSurface::CliHeadless,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::StackDependencyChip,
            maturity: M5ReviewComponentMaturityClass::Beta,
            scope_summary: "Stack-dependency chip showing stack relation and parent-blocked state so a blocked stack parent is visible rather than hidden".to_owned(),
            provider_local_distinction: "Provider-reported stack relations are labeled provider-backed; locally derived stack topology is labeled local and never presented as the provider's confirmed stack order".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            ],
            browser_handoff_boundary: "Opening a stack parent on the provider is an explicit browser handoff with a labeled return path to the stack view".to_owned(),
            local_continue_fallback: "When provider stack state is stale the chip shows last-known relation labeled stale and continues local topology from change lineage".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:stack-dependency-chip-relation-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::StackParentBlocked,
                M5ReviewComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_STACK_DEPENDENCY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::ReviewWorkspace,
                M5ReviewComponentConsumerSurface::BrowserCompanion,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::HelpAbout,
            ],
        },
        M5ReviewComponentMatrixRow {
            component: M5ReviewComponent::ApprovalInvalidationBanner,
            maturity: M5ReviewComponentMaturityClass::Preview,
            scope_summary: "Approval-invalidation banner naming why approvals were recomputed (stale base, new commits, policy change) rather than showing a generic warning pill".to_owned(),
            provider_local_distinction: "Provider-recomputed approval state is labeled provider-backed; local prediction that approvals will invalidate is labeled local and never asserted as the provider's decision".to_owned(),
            stale_provider_downgrade_vocab: vec![
                M5ReviewComponentStaleProviderState::ProviderFresh,
                M5ReviewComponentStaleProviderState::ProviderStale,
                M5ReviewComponentStaleProviderState::ProviderConflict,
                M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            ],
            browser_handoff_boundary: "Re-requesting approval on the provider is an explicit browser handoff with a labeled return path to the review workspace".to_owned(),
            local_continue_fallback: "When provider approval state is stale the banner shows the last-known invalidation reason labeled stale and continues local review without asserting fresh approval".to_owned(),
            evidence_requirement: M5ReviewComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:approval-invalidation-reason-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewComponentDowngradeTrigger::ProofStale,
                M5ReviewComponentDowngradeTrigger::ApprovalInvalidated,
                M5ReviewComponentDowngradeTrigger::PolicyBlocked,
                M5ReviewComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ReviewComponentRollbackPosture::EvidencePreservedNoRevert,
            source_contract_refs: vec![
                M5_REVIEW_COMPONENT_MATRIX_APPROVAL_INVALIDATION_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewComponentConsumerSurface::ReviewWorkspace,
                M5ReviewComponentConsumerSurface::BrowserCompanion,
                M5ReviewComponentConsumerSurface::SupportExport,
                M5ReviewComponentConsumerSurface::HelpAbout,
            ],
        },
    ]
}

fn trust_review() -> M5ReviewComponentMatrixTrustReview {
    M5ReviewComponentMatrixTrustReview {
        provider_local_estimate_distinct: true,
        stale_provider_downgrade_explicit: true,
        approval_invalidation_never_generic_warning: true,
        browser_handoff_explicit: true,
        local_continue_preserved_on_degraded_freshness: true,
        stack_blocking_explicit: true,
        queue_ownership_explicit: true,
        check_class_explicit: true,
        no_forced_raw_provider_navigation_for_triage: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ReviewComponentMatrixConsumerProjection {
    M5ReviewComponentMatrixConsumerProjection {
        review_request_row_shows_provider_and_base_head: true,
        checks_summary_card_shows_check_class_and_freshness: true,
        pending_review_tray_shows_owner_and_local_provider: true,
        merge_readiness_panel_shows_blocking_and_ownership: true,
        merge_queue_entry_shows_queue_owner_and_position: true,
        stack_dependency_chip_shows_stack_relation_and_blocking: true,
        approval_invalidation_banner_shows_reason_not_generic: true,
        cli_headless_shows_component_truth: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> M5ReviewComponentMatrixProofFreshness {
    M5ReviewComponentMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_REVIEW_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_REVIEW_REQUEST_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_CHECKS_SUMMARY_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_PENDING_TRAY_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_MERGE_READINESS_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_STACK_DEPENDENCY_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_MATRIX_APPROVAL_INVALIDATION_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5ReviewComponentMatrixPacket {
    M5ReviewComponentMatrixPacket::new(M5ReviewComponentMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Review-Request, Checks, and Merge-Queue Component Matrix".to_owned(),
        component_rows: component_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_review_component_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_component_fails_validation() {
    let mut packet = packet();
    packet
        .component_rows
        .retain(|row| row.component != M5ReviewComponent::MergeQueueEntry);
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn stable_component_missing_evidence_fails() {
    let mut packet = packet();
    packet.component_rows[0]
        .required_evidence_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::StableComponentMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.component_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.component_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_provider_local_distinction_fails() {
    let mut packet = packet();
    packet.component_rows[0].provider_local_distinction = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::ProviderLocalDistinctionMissing));
}

#[test]
fn missing_stale_provider_vocab_fails() {
    let mut packet = packet();
    packet.component_rows[4]
        .stale_provider_downgrade_vocab
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::StaleProviderVocabMissing));
}

#[test]
fn missing_browser_handoff_boundary_fails() {
    let mut packet = packet();
    packet.component_rows[3].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::BrowserHandoffBoundaryMissing));
}

#[test]
fn missing_local_continue_fallback_fails() {
    let mut packet = packet();
    packet.component_rows[5].local_continue_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::LocalContinueFallbackMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .approval_invalidation_never_generic_warning = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .merge_queue_entry_shows_queue_owner_and_position = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReviewComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for component in M5ReviewComponent::ALL {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_review_component_matrix_export()
        .expect("checked M5 review-component matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-check-queue-components/merge_queue_provider_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-check-queue-components/approval_invalidation_banner_held.json"
        )),
    ] {
        let packet: M5ReviewComponentMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
