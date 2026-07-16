//! Canonical seed builders for the frozen M5 constrained-file-state matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical constrained-file-state matrix.
pub const M5_CONSTRAINED_FILE_STATE_MATRIX_PACKET_ID: &str =
    "m5-constrained-file-state:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5ConstrainedFileStateRequiredLabel> {
    M5ConstrainedFileStateRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(
    extra: &[M5ConstrainedFileStateRequiredLabel],
) -> Vec<M5ConstrainedFileStateRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5ConstrainedFileStateObject,
    qualification: M5ConstrainedFileStateQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5ConstrainedFileStateVisibleState,
) -> M5ConstrainedFileStateRow {
    M5ConstrainedFileStateRow {
        object_class,
        qualification,
        write_disposition: M5ConstrainedFileStateWriteDisposition::ReadOnlyBlocked,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5ConstrainedFileStateSurfaceFamily::ALL.to_vec(),
        classification_stages: M5ConstrainedFileStateClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        read_only_roles: vec![],
        generated_roles: vec![],
        policy_locked_roles: vec![],
        managed_roles: vec![],
        projection_roles: vec![],
        captured_snapshot_roles: vec![],
        degraded_reasons: M5ConstrainedFileStateDegradedReason::ALL.to_vec(),
        accessibility_routes: M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ConstrainedFileStateConsumerSurface::TabChrome,
            M5ConstrainedFileStateConsumerSurface::StatusBar,
        ],
        downgrade_triggers: vec![M5ConstrainedFileStateDowngradeTrigger::ConstrainedFileStateDescriptorStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior: false,
        lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write: false,
        gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules: false,
        leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated: false,
        presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path: false,
    }
}

fn txn(f: [&str; 7]) -> M5ConstrainedFileStateVisibleState {
    M5ConstrainedFileStateVisibleState {
        state_badge: f[0].to_owned(),
        reason: f[1].to_owned(),
        canonical_source_or_live_target: f[2].to_owned(),
        exact_write_target: f[3].to_owned(),
        allowed_actions: f[4].to_owned(),
        blocked_actions: f[5].to_owned(),
        export_retain_notes: f[6].to_owned(),
    }
}

fn constrained_file_state_rows() -> Vec<M5ConstrainedFileStateRow> {
    use M5ConstrainedFileStateConsumerSurface as C;
    use M5ConstrainedFileStateDowngradeTrigger as D;
    use M5ConstrainedFileStateObject as O;
    use M5ConstrainedFileStateQualificationClass as Q;
    use M5ConstrainedFileStateRequiredLabel as L;
    use M5ConstrainedFileStateRole as R;

    let mut rows = Vec::new();

    // 1. ReadOnly.
    let mut row = base_row(
        O::ReadOnly,
        Q::Stable,
        "Read-only object-state owner",
        "Editor-governance backup owner",
        "One read-only current object is shown as read-only, not directly writable: it carries the read-only badge, the blocked-write reason, the canonical owning source, the exact write target a write would touch, an explicit allowed-versus-blocked action set, and a duplicate-to-editable-copy safe next step so no surface lets it look writable by omission",
        "evidence:m5-read-only-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "read-only",
            "object is read-only in this context and cannot be written in place",
            "canonical source is the owning read-only object; no live-target handoff needed",
            "no direct write target; a write would be refused at the read-only object",
            "inspect, copy, duplicate-to-editable-copy",
            "in-place edit, save-over, direct write",
            "read-only object exports metadata only; nothing is lost because nothing is written",
        ]),
    );
    row.read_only_roles = M5ConstrainedFileStateReadOnlyRole::ALL.to_vec();
    row.semantic_roles = vec![R::StateBadgeClassification, R::BlockedWriteReason];
    row.required_labels = labels_with(&[L::StateBadge]);
    row.consumer_surfaces = vec![
        C::TabChrome,
        C::BreadcrumbTrail,
        C::StatusBar,
        C::EditorBanner,
        C::CommandPalette,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::ReadOnlyBlocked;
    row.downgrade_triggers = vec![
        D::ConstrainedObjectShownAsWritable,
        D::StateBadgeMissing,
        D::BlockedWriteReasonMissing,
        D::NearestSafeActionMissing,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    // 2. Generated.
    let mut row = base_row(
        O::Generated,
        Q::Stable,
        "Generated-artifact object-state owner",
        "Build-governance backup owner",
        "One generated / derived artifact object names its generator as the canonical source, flags any diverged-from-generator state, states the exact write target, and offers a regenerate-from-source safe next step instead of a silent lossy direct write so edits flow through the generator, never over the artifact",
        "evidence:m5-generated-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "generated",
            "object is a generated artifact; direct edits would be overwritten on the next regenerate",
            "canonical source is the generator input and its provenance manifest",
            "exact write target is the generator source, not the generated artifact",
            "inspect, diff-against-generator, regenerate-from-source",
            "direct edit without regenerate, save-over",
            "generated artifact exports its source relation; local edits are lost on regenerate unless applied to the source",
        ]),
    );
    row.generated_roles = M5ConstrainedFileStateGeneratedRole::ALL.to_vec();
    row.semantic_roles = vec![R::CanonicalSourceRelation, R::SafeNextStepGuidance];
    row.required_labels = labels_with(&[L::NearestSafeAction]);
    row.consumer_surfaces = vec![
        C::EditorBanner,
        C::DiffReviewHeader,
        C::WriteReviewSheet,
        C::AiAutomationPath,
        C::StatusBar,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::RegenerateOnly;
    row.downgrade_triggers = vec![
        D::SilentLossyDirectWriteFallback,
        D::CanonicalSourceUnstated,
        D::RecoveryOrRegeneratePathMissing,
        D::PreservedVersusLostSyncUnstated,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    // 3. PolicyLocked.
    let mut row = base_row(
        O::PolicyLocked,
        Q::Stable,
        "Policy-locked object-state owner",
        "Policy-governance backup owner",
        "One policy-locked object shows its policy-lock badge, names the lock reason and governing policy, names the canonical policy owner, states the exact write target, and offers a request-approval safe next step so a locked write is gated behind an approval rather than a silent override",
        "evidence:m5-policy-locked-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "policy-locked",
            "object is locked by policy and cannot be written without approval",
            "canonical source is the policy owner named on the lock",
            "exact write target is the locked object, gated behind an approval token",
            "inspect, request-approval, view-policy",
            "direct write, silent override, automation bypass",
            "policy-locked object exports its lock state; no bytes change until an approval clears",
        ]),
    );
    row.policy_locked_roles = M5ConstrainedFileStatePolicyLockedRole::ALL.to_vec();
    row.semantic_roles = vec![R::BlockedWriteReason, R::AllowedBlockedActionSet];
    row.required_labels = labels_with(&[L::NearestSafeAction]);
    row.consumer_surfaces = vec![
        C::TabChrome,
        C::StatusBar,
        C::EditorBanner,
        C::WriteReviewSheet,
        C::CommandPalette,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::ApprovalGated;
    row.downgrade_triggers = vec![
        D::ConstrainedObjectShownAsWritable,
        D::BlockedWriteReasonMissing,
        D::AiAutomationBypassedConstraint,
        D::NearestSafeActionMissing,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    // 4. Managed.
    let mut row = base_row(
        O::Managed,
        Q::Stable,
        "Managed object-state owner",
        "Ecosystem-governance backup owner",
        "One managed, externally-owned object shows its managed badge, names the managing owner as canonical source, states the exact write target for any managed-change request, and offers a request-managed-change safe next step so a local divergent write never silently masquerades as an accepted change",
        "evidence:m5-managed-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "managed",
            "object is managed by an external owner; local writes do not become the source of truth",
            "canonical source is the managing owner named on the managed badge",
            "exact write target is a managed-change request against the managing owner",
            "inspect, request-managed-change, view-owner",
            "local divergent write, save-over, automation bypass",
            "managed object exports its owner relation; a local write is preserved only as a change request",
        ]),
    );
    row.managed_roles = M5ConstrainedFileStateManagedRole::ALL.to_vec();
    row.semantic_roles = vec![R::CanonicalSourceRelation, R::ExactWriteTarget];
    row.required_labels = labels_with(&[L::ExactWriteTarget]);
    row.consumer_surfaces = vec![
        C::TabChrome,
        C::EditorBanner,
        C::DiffReviewHeader,
        C::WriteReviewSheet,
        C::AiAutomationPath,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::ApprovalGated;
    row.downgrade_triggers = vec![
        D::SilentLossyDirectWriteFallback,
        D::ExactWriteTargetUnstated,
        D::CanonicalSourceUnstated,
        D::AiAutomationBypassedConstraint,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    // 5. Projection.
    let mut row = base_row(
        O::Projection,
        Q::Stable,
        "Projection object-state owner",
        "Editor-governance backup owner",
        "One projection / virtual view object shows its projection badge, names the backing source object as canonical source, resolves the exact write target back to that backing object, and offers a detach-or-overlay safe next step so a write is never silently dropped into a virtual view",
        "evidence:m5-projection-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "projection",
            "object is a virtual projection view, not a concrete file on disk",
            "canonical source is the backing source object the projection is derived from",
            "exact write target resolves to the backing source object, not the view",
            "inspect, detach-to-source, overlay-edit",
            "write-into-view, save-over, silent best-effort write",
            "projection exports its backing relation; edits are preserved only after detach or overlay onto the source",
        ]),
    );
    row.projection_roles = M5ConstrainedFileStateProjectionRole::ALL.to_vec();
    row.semantic_roles = vec![R::ExactWriteTarget, R::SafeNextStepGuidance];
    row.required_labels = labels_with(&[L::ExactWriteTarget]);
    row.consumer_surfaces = vec![
        C::BreadcrumbTrail,
        C::StatusBar,
        C::EditorBanner,
        C::DiffReviewHeader,
        C::CommandPalette,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::DetachRequired;
    row.downgrade_triggers = vec![
        D::OneStateClassHidesAnother,
        D::ExactWriteTargetUnstated,
        D::SilentLossyDirectWriteFallback,
        D::RecoveryOrRegeneratePathMissing,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    // 6. CapturedSnapshot.
    let mut row = base_row(
        O::CapturedSnapshot,
        Q::Stable,
        "Captured-snapshot object-state owner",
        "Recovery-governance backup owner",
        "One captured-snapshot object shows its captured-snapshot badge, names the capture time and source object, names the live target or metadata-only exit, and offers a restore-or-open-live safe next step so a snapshot is restored or handed off, never mutated in place as if it were the current live object",
        "evidence:m5-captured-snapshot-closure:001",
        &[
            M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "captured snapshot",
            "object is a captured snapshot of a past state, not the current live object",
            "canonical source is the live object captured at snapshot time",
            "exact write target is a restore into the live object, never the snapshot itself",
            "inspect, restore-to-live, open-current-live-object",
            "mutate-in-place, save-over the snapshot",
            "captured snapshot exports its capture metadata; restoring preserves the snapshot and updates the live object",
        ]),
    );
    row.captured_snapshot_roles = M5ConstrainedFileStateCapturedSnapshotRole::ALL.to_vec();
    row.semantic_roles = vec![R::StateBadgeClassification, R::ExportRetainDisclosure];
    row.required_labels = labels_with(&[L::StateBadge]);
    row.consumer_surfaces = vec![
        C::TabChrome,
        C::BreadcrumbTrail,
        C::StatusBar,
        C::EditorBanner,
        C::WriteReviewSheet,
        C::SupportExportPacket,
    ];
    row.write_disposition = M5ConstrainedFileStateWriteDisposition::RestoreOnly;
    row.downgrade_triggers = vec![
        D::ConstrainedObjectShownAsWritable,
        D::StateBadgeMissing,
        D::PreservedVersusLostSyncUnstated,
        D::OneStateClassHidesAnother,
        D::ConstrainedFileStateDescriptorStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ConstrainedFileStateGovernanceReview {
    M5ConstrainedFileStateGovernanceReview {
        no_constrained_object_looks_directly_writable_by_omission: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        write_constrained_state_is_mechanically_distinct_from_directly_writable_state: true,
        every_constrained_object_names_its_state_badge_and_blocked_write_reason: true,
        every_constrained_object_names_its_canonical_source_or_live_target: true,
        every_constrained_object_names_its_exact_write_target: true,
        nearest_safe_action_is_named_for_every_constrained_object: true,
        no_generated_managed_projection_or_archived_object_falls_back_to_lossy_direct_write: true,
        no_ai_automation_import_or_repair_flow_bypasses_constrained_state_rules: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_constrained_file_state_source: true,
        shell_editor_review_ai_help_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_constrained_file_state_vocabulary: true,
        constrained_file_state_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5ConstrainedFileStateConsumerProjection {
    M5ConstrainedFileStateConsumerProjection {
        shell_and_editor_consume_shared_constrained_file_state_truth: true,
        review_and_ai_consume_shared_write_target_and_canonical_source_truth: true,
        help_and_support_export_consume_shared_blocked_write_truth: true,
        docs_help_and_screenshots_read_single_constrained_file_state_source: true,
        constrained_objects_bind_to_shared_canonical_source_relation: true,
        support_export_reads_single_constrained_file_state_source: true,
    }
}

fn proof_freshness() -> M5ConstrainedFileStateProofFreshness {
    M5ConstrainedFileStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ConstrainedFileStateReleasePosture {
    M5ConstrainedFileStateReleasePosture {
        proof_packet_ref: M5_CONSTRAINED_FILE_STATE_ARTIFACT_REF.to_owned(),
        constrained_file_state_audit_ref: M5_CONSTRAINED_FILE_STATE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF,
        M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
        M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 constrained-file-state matrix packet.
pub fn seeded_m5_constrained_file_state_matrix() -> M5ConstrainedFileStateMatrixPacket {
    M5ConstrainedFileStateMatrixPacket::new(M5ConstrainedFileStateMatrixPacketInput {
        packet_id: M5_CONSTRAINED_FILE_STATE_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 constrained-file-state, canonical-source-relation, and write-target-review matrix"
                .to_owned(),
        constrained_file_state_rows: constrained_file_state_rows(),
        vocabulary_set: M5ConstrainedFileStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the managed object class is held at Beta because its request-managed-change and
/// exact-write-target review are not yet fully proven; every object class stays visible.
pub fn seeded_m5_constrained_file_state_matrix_managed_beta_narrowed(
) -> M5ConstrainedFileStateMatrixPacket {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.packet_id = "m5-constrained-file-state:managed-beta:0001".to_owned();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Managed)
        .expect("managed row present");
    row.qualification = M5ConstrainedFileStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the projection object class is narrowed to Preview pending detach / overlay and
/// backing-source write-target proof; every object class stays visible.
pub fn seeded_m5_constrained_file_state_matrix_projection_preview_narrowed(
) -> M5ConstrainedFileStateMatrixPacket {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.packet_id = "m5-constrained-file-state:projection-preview:0001".to_owned();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Projection)
        .expect("projection row present");
    row.qualification = M5ConstrainedFileStateQualificationClass::Preview;
    packet
}
