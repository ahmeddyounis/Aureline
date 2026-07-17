//! Canonical seed builders for the frozen M5 change-intent matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical change-intent matrix.
pub const M5_CHANGE_INTENT_MATRIX_PACKET_ID: &str = "m5-change-intent-lifecycle:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5ChangeIntentRequiredLabel> {
    M5ChangeIntentRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(extra: &[M5ChangeIntentRequiredLabel]) -> Vec<M5ChangeIntentRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5ChangeIntentObject,
    qualification: M5ChangeIntentQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5ChangeIntentVisibleState,
) -> M5ChangeIntentRow {
    M5ChangeIntentRow {
        object_class,
        qualification,
        commit_state: M5ChangeIntentCommitState::LocalOnlyDraft,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5ChangeIntentSurfaceFamily::ALL.to_vec(),
        classification_stages: M5ChangeIntentClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        change_intent_record_roles: vec![],
        start_work_roles: vec![],
        linked_change_roles: vec![],
        handoff_roles: vec![],
        resolve_roles: vec![],
        blocked_escalate_roles: vec![],
        degraded_reasons: M5ChangeIntentDegradedReason::ALL.to_vec(),
        accessibility_routes: M5ChangeIntentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ChangeIntentConsumerSurface::WorkItemDetail,
            M5ChangeIntentConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![M5ChangeIntentDowngradeTrigger::ChangeIntentMatrixStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_start_work_silently_create_a_side_effect_without_disclosure: false,
        lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update:
            false,
        flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge:
            false,
        auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: false,
        drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails: false,
    }
}

fn txn(f: [&str; 7]) -> M5ChangeIntentVisibleState {
    M5ChangeIntentVisibleState {
        surface_label: f[0].to_owned(),
        provider_ownership: f[1].to_owned(),
        local_versus_provider_state: f[2].to_owned(),
        linked_engineering_identity: f[3].to_owned(),
        relation_source_state: f[4].to_owned(),
        blocker_state: f[5].to_owned(),
        validation_evidence: f[6].to_owned(),
    }
}

fn change_intent_rows() -> Vec<M5ChangeIntentRow> {
    use M5ChangeIntentConsumerSurface as C;
    use M5ChangeIntentDowngradeTrigger as D;
    use M5ChangeIntentObject as O;
    use M5ChangeIntentQualificationClass as Q;
    use M5ChangeIntentRequiredLabel as L;
    use M5ChangeIntentRole as R;

    let mut rows = Vec::new();

    // 1. ChangeIntentRecord.
    let mut row = base_row(
        O::ChangeIntentRecord,
        Q::Stable,
        "Change-intent-record owner",
        "Work-item-governance backup owner",
        "One durable change-intent record names its provider ownership, shows local-versus-provider state, names the linked branch / worktree / review identity, shows its intent lifecycle stage, and never lets its provider link or ownership be swapped without disclosure",
        "evidence:m5-change-intent-record-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "change intent record",
            "provider-owned tracked work item on the connected provider",
            "local-only draft captured on this machine, not yet a provider-committed update",
            "linked branch, worktree, and review draft identity bound to this intent",
            "linked locally until the provider confirms the link",
            "ready to resolve once the linked change lands",
            "validation evidence captured with the intent for later handoff",
        ]),
    );
    row.change_intent_record_roles = M5ChangeIntentRecordRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::ProviderOwnershipDisclosure,
        R::LocalVersusProviderStateDisclosure,
        R::LinkedEngineeringIdentityDisclosure,
    ];
    row.required_labels = labels_with(&[L::ProviderCommitState]);
    row.consumer_surfaces = vec![
        C::WorkItemDetail,
        C::StartWorkSheet,
        C::ReviewDetail,
        C::SupportExportPacket,
    ];
    row.commit_state = M5ChangeIntentCommitState::LocalOnlyDraft;
    row.downgrade_triggers = vec![
        D::ProviderOwnershipUnstated,
        D::LinkedEngineeringIdentityUnstated,
        D::LocalVersusProviderStateUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    // 2. StartWorkSheet.
    let mut row = base_row(
        O::StartWorkSheet,
        Q::Stable,
        "Start-work-sheet owner",
        "Work-item-governance backup owner",
        "One start-work sheet discloses the branch, worktree, review draft, and provider link it would create as separate side effects, names the tracked item it launches from, and never silently creates a branch, worktree, review draft, or provider link without disclosing each side effect",
        "evidence:m5-start-work-sheet-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_START_WORK_SHEET_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "start work sheet",
            "provider-owned tracked item this sheet launches work from",
            "local-only draft: no side effect is created until each is disclosed and confirmed",
            "branch, worktree, review draft, and provider link disclosed as four separate side effects",
            "linked locally at start; the provider link is disclosed before it is created",
            "ready to resolve is not claimed until work is actually started",
            "start-work checklist captured as the initial validation evidence",
        ]),
    );
    row.start_work_roles = M5ChangeIntentStartWorkRole::ALL.to_vec();
    row.semantic_roles = vec![R::SideEffectDisclosure];
    row.required_labels = labels_with(&[L::RelationSource]);
    row.consumer_surfaces = vec![C::StartWorkSheet, C::WorkItemDetail, C::SupportExportPacket];
    row.commit_state = M5ChangeIntentCommitState::LocalOnlyDraft;
    row.downgrade_triggers = vec![
        D::SilentSideEffectCreated,
        D::LinkedEngineeringIdentityUnstated,
        D::ProviderOwnershipUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    // 3. LinkedChangePanel.
    let mut row = base_row(
        O::LinkedChangePanel,
        Q::Stable,
        "Linked-change-panel owner",
        "Work-item-governance backup owner",
        "One linked-change panel shows the relation source for each linked change, keeps linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken distinct, flags stale-or-broken relations, and never flattens the four relation sources into one generic relation badge",
        "evidence:m5-linked-change-panel-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_LINKED_CHANGE_PANEL_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "linked change panel",
            "provider-owned tracked item whose linked changes are shown",
            "provider-committed link confirmed by the connected provider",
            "the linked branch and review targets, each with its own relation source",
            "linked by provider, distinct from a locally linked or suggested relation",
            "ready to resolve when every linked change is confirmed",
            "linked-change provenance captured as validation evidence",
        ]),
    );
    row.linked_change_roles = M5ChangeIntentLinkedChangeRole::ALL.to_vec();
    row.semantic_roles = vec![R::LinkedEngineeringIdentityDisclosure];
    row.required_labels = labels_with(&[L::RelationSource]);
    row.consumer_surfaces = vec![
        C::LinkedChangePanel,
        C::WorkItemDetail,
        C::ReviewDetail,
        C::SupportExportPacket,
    ];
    row.commit_state = M5ChangeIntentCommitState::ProviderCommitted;
    row.downgrade_triggers = vec![
        D::RelationSourcesFlattened,
        D::RelationSourceUnstated,
        D::LinkedEngineeringIdentityUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    // 4. ReadyForReviewHandoffSheet.
    let mut row = base_row(
        O::ReadyForReviewHandoffSheet,
        Q::Stable,
        "Ready-for-review-handoff owner",
        "Review-governance backup owner",
        "One ready-for-review handoff sheet packages the validation evidence backing the handoff, shows the publish-later fallback, labels a local handoff packet as local, names the handoff destination, and never lets a local handoff packet or queued publish masquerade as a provider-committed update",
        "evidence:m5-ready-for-review-handoff-sheet-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_READY_FOR_REVIEW_HANDOFF_SHEET_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "ready for review handoff sheet",
            "provider-owned tracked item this handoff is prepared for",
            "queued for publish: a deferred handoff waiting to reach the provider, not yet committed",
            "linked branch and review draft the handoff packages",
            "linked locally until the queued publish reaches the provider",
            "ready to resolve is deferred until the handoff is committed and reviewed",
            "the validation checks and publish-later fallback packaged with the handoff",
        ]),
    );
    row.handoff_roles = M5ChangeIntentHandoffRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::ValidationEvidenceDisclosure,
        R::PublishLaterFallbackDisclosure,
    ];
    row.required_labels = labels_with(&[L::ProviderCommitState]);
    row.consumer_surfaces = vec![
        C::ReadyForReviewHandoff,
        C::ReviewDetail,
        C::SupportExportPacket,
    ];
    row.commit_state = M5ChangeIntentCommitState::QueuedForPublish;
    row.downgrade_triggers = vec![
        D::LocalHandoffShownAsProviderCommitted,
        D::ValidationEvidenceUnstated,
        D::LocalVersusProviderStateUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    // 5. ResolveCloseSheet.
    let mut row = base_row(
        O::ResolveCloseSheet,
        Q::Stable,
        "Resolve-close-sheet owner",
        "Work-item-governance backup owner",
        "One resolve-or-close sheet shows the final-resolution authority, shows any unresolved engineering blocker, names the resolution outcome, shows provider-write-pending state, and never auto-resolves tracked work while engineering blockers remain unresolved",
        "evidence:m5-resolve-close-sheet-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_RESOLVE_CLOSE_SHEET_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "resolve or close sheet",
            "provider-owned tracked item being resolved or closed",
            "provider-committed once the resolution is written to the connected provider",
            "linked branch and review whose merge confirms the resolution",
            "linked by provider for the confirmed resolution",
            "ready to resolve only when no engineering blocker remains",
            "the resolution outcome and final-resolution authority recorded as evidence",
        ]),
    );
    row.resolve_roles = M5ChangeIntentResolveRole::ALL.to_vec();
    row.semantic_roles = vec![R::FinalResolutionAuthorityDisclosure];
    row.required_labels = labels_with(&[L::BlockerState]);
    row.consumer_surfaces = vec![
        C::ResolveCloseSheet,
        C::WorkItemDetail,
        C::SupportExportPacket,
    ];
    row.commit_state = M5ChangeIntentCommitState::ProviderCommitted;
    row.downgrade_triggers = vec![
        D::AutoResolvedWithOpenBlocker,
        D::BlockerStateUnstated,
        D::ProviderOwnershipUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    // 6. BlockedEscalateCard.
    let mut row = base_row(
        O::BlockedEscalateCard,
        Q::Stable,
        "Blocked-escalate-card owner",
        "Incident-governance backup owner",
        "One blocked-or-escalate card names the blocker cause, shows the escalation path, retains local notes and linked evidence, shows the blocker state, and never drops local notes, handoff packets, or linked evidence when provider write fails",
        "evidence:m5-blocked-escalate-card-closure:001",
        &[
            M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
            M5_BLOCKED_ESCALATE_CARD_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "blocked or escalate card",
            "provider-owned tracked item with an open engineering blocker",
            "publish failed and retained: local notes and linked evidence kept for retry, not committed",
            "linked branch and review the blocker is attached to",
            "linked locally; the provider write that failed is retried without dropping evidence",
            "blocked by engineering with an open escalation",
            "local notes, the handoff packet, and linked evidence retained through the failure",
        ]),
    );
    row.blocked_escalate_roles = M5ChangeIntentBlockedEscalateRole::ALL.to_vec();
    row.semantic_roles = vec![R::FinalResolutionAuthorityDisclosure];
    row.required_labels = labels_with(&[L::BlockerState]);
    row.consumer_surfaces = vec![
        C::BlockedEscalateCard,
        C::WorkItemDetail,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.commit_state = M5ChangeIntentCommitState::PublishFailedRetained;
    row.downgrade_triggers = vec![
        D::LocalEvidenceDroppedOnProviderWriteFailure,
        D::BlockerStateUnstated,
        D::LocalVersusProviderStateUnstated,
        D::ChangeIntentMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ChangeIntentGovernanceReview {
    M5ChangeIntentGovernanceReview {
        no_local_handoff_packet_is_shown_as_a_provider_committed_update: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        provider_committed_state_is_mechanically_distinct_from_local_only_draft: true,
        every_change_intent_names_its_provider_ownership: true,
        every_start_work_sheet_discloses_each_side_effect_separately: true,
        every_linked_change_names_its_relation_source: true,
        no_start_work_side_effect_is_created_without_disclosure: true,
        every_handoff_discloses_its_publish_later_fallback: true,
        no_tracked_work_is_auto_resolved_while_engineering_blockers_remain: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_change_intent_source: true,
        work_item_start_work_review_provider_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_change_intent_vocabulary: true,
        change_intent_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5ChangeIntentConsumerProjection {
    M5ChangeIntentConsumerProjection {
        work_item_detail_and_start_work_consume_shared_change_intent_truth: true,
        ready_for_review_handoff_and_provider_handoff_consume_shared_commit_state_truth: true,
        help_and_support_export_consume_shared_relation_and_blocker_truth: true,
        docs_help_and_screenshots_read_single_change_intent_source: true,
        change_intents_bind_to_shared_linked_change_relation: true,
        support_export_reads_single_change_intent_source: true,
    }
}

fn proof_freshness() -> M5ChangeIntentProofFreshness {
    M5ChangeIntentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ChangeIntentReleasePosture {
    M5ChangeIntentReleasePosture {
        proof_packet_ref: M5_CHANGE_INTENT_ARTIFACT_REF.to_owned(),
        change_intent_audit_ref: M5_CHANGE_INTENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
        M5_CHANGE_INTENT_MATRIX_DOC_REF,
        M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
        M5_START_WORK_SHEET_DOMAIN_SCHEMA_REF,
        M5_LINKED_CHANGE_PANEL_DOMAIN_SCHEMA_REF,
        M5_READY_FOR_REVIEW_HANDOFF_SHEET_DOMAIN_SCHEMA_REF,
        M5_RESOLVE_CLOSE_SHEET_DOMAIN_SCHEMA_REF,
        M5_BLOCKED_ESCALATE_CARD_DOMAIN_SCHEMA_REF,
        M5_WORK_ITEM_HANDOFF_PACKET_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 change-intent matrix packet.
pub fn seeded_m5_change_intent_matrix() -> M5ChangeIntentMatrixPacket {
    M5ChangeIntentMatrixPacket::new(M5ChangeIntentMatrixPacketInput {
        packet_id: M5_CHANGE_INTENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 change-intent, start-work-sheet, linked-change-panel, ready-for-review-handoff, resolve-close-sheet, and blocked-escalate-card matrix"
            .to_owned(),
        change_intent_rows: change_intent_rows(),
        vocabulary_set: M5ChangeIntentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the start-work sheet is held at Beta because its side-effect disclosure is not yet
/// fully proven across every provider; every object class stays visible.
pub fn seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed() -> M5ChangeIntentMatrixPacket
{
    let mut packet = seeded_m5_change_intent_matrix();
    packet.packet_id = "m5-change-intent-lifecycle:start-work-sheet-beta:0001".to_owned();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::StartWorkSheet)
        .expect("start-work-sheet row present");
    row.qualification = M5ChangeIntentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the blocked-or-escalate card is narrowed to Preview pending durable escalation-path
/// and evidence-retention proof; every object class stays visible.
pub fn seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed(
) -> M5ChangeIntentMatrixPacket {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.packet_id = "m5-change-intent-lifecycle:blocked-escalate-card-preview:0001".to_owned();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::BlockedEscalateCard)
        .expect("blocked-escalate-card row present");
    row.qualification = M5ChangeIntentQualificationClass::Preview;
    packet
}
