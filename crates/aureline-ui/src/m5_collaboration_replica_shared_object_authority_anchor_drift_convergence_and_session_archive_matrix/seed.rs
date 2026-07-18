//! Canonical seed builders for the frozen M5 collaboration-state matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical collaboration-state matrix.
pub const M5_COLLABORATION_STATE_MATRIX_PACKET_ID: &str =
    "m5-collaboration-convergence:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-18T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5CollaborationStateRequiredLabel> {
    M5CollaborationStateRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(
    extra: &[M5CollaborationStateRequiredLabel],
) -> Vec<M5CollaborationStateRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5CollaborationStateObject,
    qualification: M5CollaborationStateQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5CollaborationStateVisibleState,
) -> M5CollaborationStateRow {
    M5CollaborationStateRow {
        object_class,
        qualification,
        convergence_state: M5ConvergenceState::Converged,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5CollaborationStateSurfaceFamily::ALL.to_vec(),
        classification_stages: M5CollaborationStateClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        crdt_backed_shared_text_roles: vec![],
        sampled_presence_cursors_selections_roles: vec![],
        server_ordered_comments_annotations_review_pins_roles: vec![],
        presenter_follow_state_roles: vec![],
        higher_risk_control_plane_roles: vec![],
        sealed_session_archive_roles: vec![],
        degraded_reasons: M5CollaborationStateDegradedReason::ALL.to_vec(),
        accessibility_routes: M5CollaborationStateAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5CollaborationStateConsumerSurface::SharedEditorReplicaView,
            M5CollaborationStateConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![
            M5CollaborationStateDowngradeTrigger::CollaborationStateMatrixStale,
        ],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_a_replica_overwrite_local_buffer_vfs_or_git_truth_implicitly: false,
        discards_unsent_local_edits_on_permission_downgrade_relay_failure_or_leave: false,
        rebinds_comments_annotations_or_review_pins_without_drift_history: false,
        collapses_convergence_or_awareness_degraded_state_into_a_generic_stale_badge: false,
        exports_op_logs_snapshots_or_archives_without_policy_labeled_redaction_and_lineage: false,
    }
}

fn txn(f: [&str; 7]) -> M5CollaborationStateVisibleState {
    M5CollaborationStateVisibleState {
        surface_label: f[0].to_owned(),
        authority_model: f[1].to_owned(),
        convergence_state: f[2].to_owned(),
        local_truth_disposition: f[3].to_owned(),
        merge_and_drift_summary: f[4].to_owned(),
        export_posture: f[5].to_owned(),
        provenance_and_freshness: f[6].to_owned(),
    }
}

fn collaboration_state_rows() -> Vec<M5CollaborationStateRow> {
    use M5CollaborationStateConsumerSurface as C;
    use M5CollaborationStateDowngradeTrigger as D;
    use M5CollaborationStateObject as O;
    use M5CollaborationStateQualificationClass as Q;
    use M5CollaborationStateRequiredLabel as L;
    use M5CollaborationStateRole as R;
    use M5ConvergenceState as S;

    let mut rows = Vec::new();

    // 1. CrdtBackedSharedText.
    let mut row = base_row(
        O::CrdtBackedSharedText,
        Q::Stable,
        "Crdt-backed-shared-text owner",
        "Collaboration-state backup owner",
        "One CRDT-backed shared-text object is the convergent replica of shared text that merges concurrent edits, shows whether it has converged, keeps the local buffer, VFS, and Git truth canonical, shows its merge semantics, and preserves unsent local edits on downgrade, never letting the replica overwrite local, VFS, or Git truth implicitly",
        "evidence:m5-crdt-backed-shared-text-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "shared editor replica view",
            "crdt-convergent replica: peers converge on one value without a central ordering authority",
            "converged: every replica has converged on one agreed value",
            "the local buffer, VFS, and Git truth stay canonical and are never replaced by the replica",
            "concurrent edits merge under the declared CRDT semantics; there is no anchor drift for plain text",
            "op-logs and snapshots export only with policy-labeled redaction and actor lineage",
            "the replica's convergence and freshness are shown so search and AI never read a stale buffer as current",
        ]),
    );
    row.crdt_backed_shared_text_roles = M5CrdtBackedSharedTextRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::AuthorityModelDisclosure,
        R::LocalTruthPreservationDisclosure,
        R::MergeAndDriftSemanticsDisclosure,
    ];
    row.required_labels = labels_with(&[L::AuthorityModel]);
    row.consumer_surfaces = vec![
        C::SharedEditorReplicaView,
        C::SearchAndAiProvenanceConsumer,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::Converged;
    row.downgrade_triggers = vec![
        D::ReplicaOverwroteLocalCanonicalTruth,
        D::UnsentLocalEditsDiscardedOnDowngrade,
        D::AuthorityModelUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    // 2. SampledPresenceCursorsSelections.
    let mut row = base_row(
        O::SampledPresenceCursorsSelections,
        Q::Stable,
        "Sampled-presence owner",
        "Collaboration-state backup owner",
        "One sampled presence / cursors / selections object samples non-authoritative presence, shows its sampling rate, shows that presence is non-authoritative, expires stale cursors and selections, and never edits the buffer, never treating sampled presence as converged truth",
        "evidence:m5-sampled-presence-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_SAMPLED_PRESENCE_CURSORS_SELECTIONS_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "presence / cursor layer",
            "sampled, non-authoritative presence: cursors and selections are sampled, not a converged value",
            "sampled-presence only: only sampled, non-authoritative presence is available for the object",
            "presence never edits the local buffer, VFS, or Git truth",
            "there is nothing to merge; stale cursors and selections expire rather than mislead",
            "presence metadata exports as sampled counts and roles, never raw cursor payloads",
            "the sampling rate and last-seen freshness are shown so a stale cursor is never read as live",
        ]),
    );
    row.sampled_presence_cursors_selections_roles =
        M5SampledPresenceCursorsSelectionsRole::ALL.to_vec();
    row.semantic_roles = vec![R::ProvenanceAndFreshnessDisclosure];
    row.required_labels = labels_with(&[L::ConvergenceState]);
    row.consumer_surfaces = vec![
        C::PresenceCursorLayer,
        C::SharedEditorReplicaView,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::SampledPresenceOnly;
    row.downgrade_triggers = vec![
        D::ProvenanceOrFreshnessUnstated,
        D::ConvergenceStateUnstated,
        D::AuthorityModelUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    // 3. ServerOrderedCommentsAnnotationsReviewPins.
    let mut row = base_row(
        O::ServerOrderedCommentsAnnotationsReviewPins,
        Q::Stable,
        "Server-ordered-comments owner",
        "Collaboration-review backup owner",
        "One server-ordered comments / annotations / review-pins object shows its server ordering, records anchor drift append-only, shows every rebind as reviewable, and shows pin-resolution provenance, never rebinding a comment, annotation, or review pin without append-only drift history",
        "evidence:m5-server-ordered-comments-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_SERVER_ORDERED_COMMENTS_ANNOTATIONS_REVIEW_PINS_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "comment / annotation / review-pin layer",
            "server-ordered sequence: pin and comment order is fixed by a server total order, not peer convergence",
            "anchor-rebound (append-only): an anchor rebound to a new position, recorded append-only in drift history",
            "comments and pins reference the buffer but never replace the canonical local, VFS, or Git truth",
            "server ordering fixes sequence; anchor drift is append-only and every rebind is reviewable",
            "pins and anchor-drift history export as metadata with actor lineage, never raw comment bodies",
            "each pin's resolution provenance and drift history are shown so a rebind is never silent",
        ]),
    );
    row.server_ordered_comments_annotations_review_pins_roles =
        M5ServerOrderedCommentsAnnotationsReviewPinsRole::ALL.to_vec();
    row.semantic_roles = vec![R::AnchorDriftHistoryDisclosure];
    row.required_labels = labels_with(&[L::AuthorityModel]);
    row.consumer_surfaces = vec![
        C::CommentAnnotationReviewPinLayer,
        C::SearchAndAiProvenanceConsumer,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::AnchorReboundAppendOnly;
    row.downgrade_triggers = vec![
        D::CommentOrPinReboundWithoutDriftHistory,
        D::AnchorDriftHistoryUnstated,
        D::ConvergenceStateUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    // 4. PresenterFollowState.
    let mut row = base_row(
        O::PresenterFollowState,
        Q::Stable,
        "Presenter-follow-state owner",
        "Collaboration-state backup owner",
        "One presenter / follow state names its presenter holder, names its follow target, shows that following is view-only, and shows its handoff provenance, never letting follow imply control or convergence",
        "evidence:m5-presenter-follow-state-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_PRESENTER_FOLLOW_STATE_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "presenter / follow banner",
            "host-authoritative state: the presenter owns the followed viewport; followers observe, not merge",
            "host-authoritative: the followed viewport is owned by the presenter, not merged by followers",
            "following another viewport never mutates the follower's local buffer, VFS, or Git truth",
            "there is no merge; the presenter's viewport is authoritative and the handoff chain is provenance-tracked",
            "presenter and follow state export as roles and handoff lineage, never raw viewport contents",
            "the current presenter, follow target, and handoff provenance are shown so follow never reads as control",
        ]),
    );
    row.presenter_follow_state_roles = M5PresenterFollowStateRole::ALL.to_vec();
    row.semantic_roles = vec![R::AuthorityModelDisclosure];
    row.required_labels = labels_with(&[L::AuthorityModel]);
    row.consumer_surfaces = vec![
        C::PresenterFollowBanner,
        C::SearchAndAiProvenanceConsumer,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::HostAuthoritative;
    row.downgrade_triggers = vec![
        D::AuthorityModelUnstated,
        D::ProvenanceOrFreshnessUnstated,
        D::ConvergenceStateUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    // 5. HigherRiskControlPlane.
    let mut row = base_row(
        O::HigherRiskControlPlane,
        Q::Stable,
        "Higher-risk-control-plane owner",
        "Collaboration-safety backup owner",
        "One higher-risk control plane is kept separate from the convergent objects, distinguishes convergence-degraded from awareness-degraded, shows the anchor-unresolved state, and preserves local unsent work first on downgrade, never collapsing a convergence-degraded or awareness-degraded state into a generic stale or broken badge",
        "evidence:m5-higher-risk-control-plane-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_HIGHER_RISK_CONTROL_PLANE_DOMAIN_SCHEMA_REF,
            M5_COLLABORATION_CONTROL_MATRIX_LANDED_SCHEMA_REF,
        ],
        txn([
            "collaboration degradation banner",
            "a separate higher-risk control plane, distinct from the convergent shared objects it guards",
            "convergence-degraded: replicas cannot currently converge and the object is not agreed",
            "on downgrade the local buffer, VFS, and Git truth are preserved before anything else",
            "no merge happens while degraded; convergence-degraded and awareness-degraded stay distinct",
            "degradation events export as named states with actor lineage, never raw op-log payloads",
            "the banner names the exact degraded state and its freshness so it is never a generic stale pill",
        ]),
    );
    row.higher_risk_control_plane_roles = M5HigherRiskControlPlaneRole::ALL.to_vec();
    row.semantic_roles = vec![R::DowngradeBehaviorDisclosure];
    row.required_labels = labels_with(&[L::ConvergenceState]);
    row.consumer_surfaces = vec![
        C::CollaborationDegradationBanner,
        C::SharedEditorReplicaView,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::ConvergenceDegraded;
    row.downgrade_triggers = vec![
        D::ConvergenceOrAwarenessDegradedCollapsedIntoGenericStale,
        D::UnsentLocalEditsDiscardedOnDowngrade,
        D::LocalTruthPreservationUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    // 6. SealedSessionArchive.
    let mut row = base_row(
        O::SealedSessionArchive,
        Q::Stable,
        "Sealed-session-archive owner",
        "Collaboration-privacy backup owner",
        "One sealed session archive shows its bounded compaction lineage, shows its retention and export posture, shows its actor lineage, and shows its policy-labeled redaction, never exporting op-logs, snapshots, or archives without policy-labeled redaction and actor lineage",
        "evidence:m5-sealed-session-archive-closure:001",
        &[
            M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "session archive and compaction view",
            "a sealed archive of prior collaboration state; it observes and retains, it does not converge",
            "sealed / archived: the session object is sealed into a retained, policy-labeled archive",
            "the archive is a copy; the canonical local buffer, VFS, and Git truth are unaffected",
            "compaction lineage is bounded and append-only; nothing is merged back into a live object",
            "the archive exports only with policy-labeled redaction and actor lineage, never raw snapshots",
            "the compaction and actor lineage and seal time are shown so provenance is never ambiguous",
        ]),
    );
    row.sealed_session_archive_roles = M5SealedSessionArchiveRole::ALL.to_vec();
    row.semantic_roles = vec![R::ExportPostureDisclosure];
    row.required_labels = labels_with(&[L::ExportPosture]);
    row.consumer_surfaces = vec![
        C::SessionArchiveAndCompactionView,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.convergence_state = S::SealedArchived;
    row.downgrade_triggers = vec![
        D::OpLogOrArchiveExportedWithoutRedactionOrLineage,
        D::ExportPostureUnstated,
        D::ProvenanceOrFreshnessUnstated,
        D::CollaborationStateMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5CollaborationStateGovernanceReview {
    M5CollaborationStateGovernanceReview {
        no_replica_overwrites_local_buffer_vfs_or_git_truth: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        converged_state_is_mechanically_distinct_from_degraded: true,
        every_shared_object_declares_its_authority_model: true,
        permission_or_relay_downgrade_preserves_local_unsent_work_first: true,
        every_comment_or_pin_rebind_carries_append_only_drift_history: true,
        convergence_and_awareness_degraded_states_are_never_collapsed_into_generic_stale: true,
        presence_and_follow_never_imply_convergence_or_control: true,
        no_op_log_or_archive_exports_without_policy_labeled_redaction_and_lineage: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_collaboration_state_source: true,
        editor_terminal_review_companion_search_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_collaboration_state_vocabulary: true,
        collaboration_state_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5CollaborationStateConsumerProjection {
    M5CollaborationStateConsumerProjection {
        shared_editor_and_presence_layers_consume_shared_collaboration_state_truth: true,
        comment_pin_and_presenter_follow_consume_shared_authority_and_drift_truth: true,
        help_and_support_export_consume_shared_convergence_and_export_truth: true,
        docs_help_and_screenshots_read_single_collaboration_state_source: true,
        companion_and_search_ai_surfaces_bind_to_shared_convergence_state_source: true,
        support_export_reads_single_collaboration_state_source: true,
    }
}

fn proof_freshness() -> M5CollaborationStateProofFreshness {
    M5CollaborationStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CollaborationStateReleasePosture {
    M5CollaborationStateReleasePosture {
        proof_packet_ref: M5_COLLABORATION_STATE_ARTIFACT_REF.to_owned(),
        collaboration_state_audit_ref: M5_COLLABORATION_STATE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_STATE_MATRIX_DOC_REF,
        M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
        M5_SAMPLED_PRESENCE_CURSORS_SELECTIONS_DOMAIN_SCHEMA_REF,
        M5_SERVER_ORDERED_COMMENTS_ANNOTATIONS_REVIEW_PINS_DOMAIN_SCHEMA_REF,
        M5_PRESENTER_FOLLOW_STATE_DOMAIN_SCHEMA_REF,
        M5_HIGHER_RISK_CONTROL_PLANE_DOMAIN_SCHEMA_REF,
        M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_MATRIX_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 collaboration-state matrix packet.
pub fn seeded_m5_collaboration_state_matrix() -> M5CollaborationStateMatrixPacket {
    M5CollaborationStateMatrixPacket::new(M5CollaborationStateMatrixPacketInput {
        packet_id: M5_COLLABORATION_STATE_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 collaboration-replica, shared-object-authority, anchor-drift, convergence-state, and session-archive matrix"
            .to_owned(),
        collaboration_state_rows: collaboration_state_rows(),
        vocabulary_set: M5CollaborationStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the higher-risk control plane is held at Beta because its convergence-versus-awareness
/// degraded distinction is not yet fully proven across every companion and search / AI surface; every object
/// class stays visible.
pub fn seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed(
) -> M5CollaborationStateMatrixPacket {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.packet_id =
        "m5-collaboration-convergence:higher-risk-control-plane-beta:0001".to_owned();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::HigherRiskControlPlane)
        .expect("higher-risk-control-plane row present");
    row.qualification = M5CollaborationStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sealed session archive is narrowed to Preview pending durable bounded-compaction-lineage
/// and policy-labeled-redaction proof; every object class stays visible.
pub fn seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed(
) -> M5CollaborationStateMatrixPacket {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.packet_id =
        "m5-collaboration-convergence:sealed-session-archive-preview:0001".to_owned();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::SealedSessionArchive)
        .expect("sealed-session-archive row present");
    row.qualification = M5CollaborationStateQualificationClass::Preview;
    packet
}
