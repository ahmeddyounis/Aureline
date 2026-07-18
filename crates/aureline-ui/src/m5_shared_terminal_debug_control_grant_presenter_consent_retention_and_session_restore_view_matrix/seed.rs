//! Canonical seed builders for the frozen M5 collaboration-control matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical collaboration-control matrix.
pub const M5_COLLABORATION_CONTROL_MATRIX_PACKET_ID: &str = "m5-collaboration-control:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-18T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5CollaborationControlRequiredLabel> {
    M5CollaborationControlRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(
    extra: &[M5CollaborationControlRequiredLabel],
) -> Vec<M5CollaborationControlRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5CollaborationControlObject,
    qualification: M5CollaborationControlQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5CollaborationControlVisibleState,
) -> M5CollaborationControlRow {
    M5CollaborationControlRow {
        object_class,
        qualification,
        session_state: M5CollaborationControlState::Viewer,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5CollaborationControlSurfaceFamily::ALL.to_vec(),
        classification_stages: M5CollaborationControlClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        shared_terminal_debug_view_roles: vec![],
        control_grant_roles: vec![],
        presenter_token_roles: vec![],
        consent_envelope_roles: vec![],
        retention_review_roles: vec![],
        session_restore_view_roles: vec![],
        degraded_reasons: M5CollaborationControlDegradedReason::ALL.to_vec(),
        accessibility_routes: M5CollaborationControlAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5CollaborationControlConsumerSurface::SharedTerminalDebugView,
            M5CollaborationControlConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![
            M5CollaborationControlDowngradeTrigger::CollaborationControlMatrixStale,
        ],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        acquires_control_from_presence_or_follow_without_an_explicit_grant: false,
        allows_more_than_one_active_driver_on_a_sensitive_surface: false,
        starts_recording_transcript_retention_or_guest_scope_widening_silently: false,
        replays_prior_terminal_or_debug_input_on_join_or_restore: false,
        reveals_raw_secrets_command_text_or_clipboard_without_a_guard_and_consent_posture: false,
    }
}

fn txn(f: [&str; 7]) -> M5CollaborationControlVisibleState {
    M5CollaborationControlVisibleState {
        surface_label: f[0].to_owned(),
        control_authority: f[1].to_owned(),
        active_driver: f[2].to_owned(),
        participant_roster_and_roles: f[3].to_owned(),
        session_state_summary: f[4].to_owned(),
        consent_and_retention_state: f[5].to_owned(),
        guard_and_restore_evidence: f[6].to_owned(),
    }
}

fn collaboration_control_rows() -> Vec<M5CollaborationControlRow> {
    use M5CollaborationControlConsumerSurface as C;
    use M5CollaborationControlDowngradeTrigger as D;
    use M5CollaborationControlObject as O;
    use M5CollaborationControlQualificationClass as Q;
    use M5CollaborationControlRequiredLabel as L;
    use M5CollaborationControlRole as R;
    use M5CollaborationControlState as S;

    let mut rows = Vec::new();

    // 1. SharedTerminalDebugView.
    let mut row = base_row(
        O::SharedTerminalDebugView,
        Q::Stable,
        "Shared-terminal-debug-view owner",
        "Collaboration-control backup owner",
        "One shared terminal / debug view streams a live shared terminal or debugger, begins view-first, names its single active driver, and shows the provenance of every input, never letting presence, follow mode, browser handoff, or companion resume acquire terminal / debug control without an explicit grant and never replaying prior input on join",
        "evidence:m5-shared-terminal-debug-view-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_SHARED_TERMINAL_DEBUG_VIEW_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "shared terminal / debug view",
            "control authority is named; no participant holds write control until an explicit grant is made",
            "one active driver is named, or explicitly none, and presence never reads as control",
            "viewers, commenters, editors, and navigators are listed with their roles alongside the driver",
            "driver: a single participant holds live terminal / debug write control",
            "recording and retention state disclosed; nothing is recorded without a consent posture",
            "the paste / secret guard is active and no prior input is replayed on join",
        ]),
    );
    row.shared_terminal_debug_view_roles = M5SharedTerminalDebugViewRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::ControlAuthorityDisclosure,
        R::ActiveDriverDisclosure,
        R::ViewFirstDefaultDisclosure,
    ];
    row.required_labels = labels_with(&[L::SessionState]);
    row.consumer_surfaces = vec![
        C::SharedTerminalDebugView,
        C::CollaborationJoinReviewSheet,
        C::ControlGrantPrompt,
        C::SupportExportPacket,
    ];
    row.session_state = S::Driver;
    row.downgrade_triggers = vec![
        D::ControlAcquiredWithoutExplicitGrant,
        D::PriorInputReplayedOnJoinOrRestore,
        D::ControlAuthorityUnstated,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    // 2. ControlGrant.
    let mut row = base_row(
        O::ControlGrant,
        Q::Stable,
        "Control-grant owner",
        "Collaboration-control backup owner",
        "One control grant is the explicit grant of terminal / debug write control that names its granted authority, enforces a single active driver, shows its scope and expiry, and shows its revoke / reclaim path, never acquiring control from presence or follow alone and never allowing more than one active driver on a sensitive surface",
        "evidence:m5-control-grant-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_CONTROL_GRANT_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "control grant",
            "the granted authority is named: this grant makes exactly one participant the driver",
            "the single active driver named by this grant, with any prior driver reverted to view-first",
            "the requesting participant and the granting authority, plus the roster held at view-first",
            "control granted: write control is explicitly granted to a single active driver",
            "the grant's consent posture and retention scope disclosed before control begins",
            "the grant scope, expiry, and revoke / reclaim path retained as evidence",
        ]),
    );
    row.control_grant_roles = M5ControlGrantRole::ALL.to_vec();
    row.semantic_roles = vec![R::ActiveDriverDisclosure];
    row.required_labels = labels_with(&[L::ControlAuthoritySource]);
    row.consumer_surfaces = vec![
        C::ControlGrantPrompt,
        C::SharedTerminalDebugView,
        C::SupportExportPacket,
    ];
    row.session_state = S::ControlGranted;
    row.downgrade_triggers = vec![
        D::ControlAcquiredWithoutExplicitGrant,
        D::MoreThanOneActiveDriverOnASensitiveSurface,
        D::ActiveDriverUnstated,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    // 3. PresenterToken.
    let mut row = base_row(
        O::PresenterToken,
        Q::Stable,
        "Presenter-token owner",
        "Collaboration-moderation backup owner",
        "One presenter token names its current holder, names its handoff target, shows its moderation scope, and shows its expiry and reclaim path, never letting two presenters drive one sensitive surface at once",
        "evidence:m5-presenter-token-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_PRESENTER_TOKEN_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "presenter handoff sheet",
            "control authority is named separately from the presenter token; presenting is not driving",
            "the active driver, if any, named alongside the current presenter / moderator",
            "the current presenter, the handoff target, and the moderated roster",
            "presenter / moderator: the token holder moderates handoff without implying write control",
            "recording and retention state disclosed for the presented session",
            "the token expiry and reclaim path retained so a stale presenter never lingers",
        ]),
    );
    row.presenter_token_roles = M5PresenterTokenRole::ALL.to_vec();
    row.semantic_roles = vec![R::ControlAuthorityDisclosure];
    row.required_labels = labels_with(&[L::ControlAuthoritySource]);
    row.consumer_surfaces = vec![
        C::PresenterHandoffSheet,
        C::SharedTerminalDebugView,
        C::SupportExportPacket,
    ];
    row.session_state = S::PresenterModerator;
    row.downgrade_triggers = vec![
        D::MoreThanOneActiveDriverOnASensitiveSurface,
        D::ControlAuthorityUnstated,
        D::ActiveDriverUnstated,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    // 4. ConsentEnvelope.
    let mut row = base_row(
        O::ConsentEnvelope,
        Q::Stable,
        "Consent-envelope owner",
        "Collaboration-privacy backup owner",
        "One consent envelope discloses its join-time consent scope, discloses the guest scope and route visibility, discloses the recording and retention consequences, and shows the consent renewal requirement, never widening guest scope or route visibility silently",
        "evidence:m5-consent-envelope-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_CONSENT_ENVELOPE_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "collaboration join-review sheet",
            "control authority is disclosed at join so a joiner knows control is grant-gated, not presence-based",
            "the active driver, if any, disclosed at join alongside the view-first default",
            "the joining participant, the guest scope granted, and the route visibility offered",
            "consent renewal required: the join-time consent scope must be accepted or renewed before joining",
            "recording, retention, guest scope, and route visibility consequences shown before joining",
            "the accepted consent scope retained as evidence; scope never widens without renewed consent",
        ]),
    );
    row.consent_envelope_roles = M5ConsentEnvelopeRole::ALL.to_vec();
    row.semantic_roles = vec![R::ConsentScopeDisclosure];
    row.required_labels = labels_with(&[L::ConsentRetentionGate]);
    row.consumer_surfaces = vec![
        C::CollaborationJoinReviewSheet,
        C::CollaborationRetentionSheet,
        C::SupportExportPacket,
    ];
    row.session_state = S::ConsentRenewalRequired;
    row.downgrade_triggers = vec![
        D::RecordingOrRetentionStartedSilently,
        D::ConsentScopeUnstated,
        D::RetentionStateUnstated,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    // 5. RetentionReview.
    let mut row = base_row(
        O::RetentionReview,
        Q::Stable,
        "Retention-review owner",
        "Collaboration-privacy backup owner",
        "One retention review shows the recording state, shows the retention mode and duration, shows the replayable-archive scope, shows the export and support-evidence scope, and never silently starts or widens recording, transcript retention, replayable archives, or route visibility, and never reveals raw secrets, command text, variable bodies, or clipboard contents without a guard and consent posture",
        "evidence:m5-retention-review-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_RETENTION_REVIEW_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "collaboration retention sheet",
            "control authority is named so an evidence view never implies control over the live session",
            "the active driver, if any, named on the retained record for audit",
            "the participants covered by the retained record, with their roles",
            "recording active: recording and retention are active under a disclosed consent posture",
            "the recording state, retention mode and duration, and replayable-archive scope shown explicitly",
            "the paste / secret guard log and sealed-archive scope retained; no raw secret is exported",
        ]),
    );
    row.retention_review_roles = M5RetentionReviewRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::RecordingRetentionStateDisclosure,
        R::PasteSecretGuardDisclosure,
    ];
    row.required_labels = labels_with(&[L::ConsentRetentionGate]);
    row.consumer_surfaces = vec![
        C::CollaborationRetentionSheet,
        C::PasteSecretGuard,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.session_state = S::RecordingActive;
    row.downgrade_triggers = vec![
        D::RecordingOrRetentionStartedSilently,
        D::RawSecretOrClipboardRevealedWithoutGuard,
        D::RetentionStateUnstated,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    // 6. SessionRestoreView.
    let mut row = base_row(
        O::SessionRestoreView,
        Q::Stable,
        "Session-restore-view owner",
        "Collaboration-control backup owner",
        "One session-restore view reattaches read-only, shows the restored scrollback read-only, replays no prior terminal / debug input on restore, preserves retention scope on restore, and requires a fresh control grant before write control resumes, never replaying prior input and never widening retention on restore",
        "evidence:m5-session-restore-view-closure:001",
        &[
            M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            M5_SESSION_RESTORE_VIEW_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "session-restore view",
            "control authority is reset on restore; a fresh grant is required before write control resumes",
            "no active driver on restore until a fresh control grant is made; the session reattaches view-only",
            "the participants reattaching to the restored session, listed at view-first",
            "restore view-only: the restored session is view-only until a fresh grant is made",
            "the retention scope preserved on restore, never widened by the restore itself",
            "the restored scrollback is read-only evidence; no prior input is replayed on restore",
        ]),
    );
    row.session_restore_view_roles = M5SessionRestoreViewRole::ALL.to_vec();
    row.semantic_roles = vec![R::ReplayFreeRestoreDisclosure];
    row.required_labels = labels_with(&[L::SessionState]);
    row.consumer_surfaces = vec![
        C::SessionRestoreView,
        C::SharedTerminalDebugView,
        C::HelpDocs,
        C::SupportExportPacket,
    ];
    row.session_state = S::RestoreViewOnly;
    row.downgrade_triggers = vec![
        D::PriorInputReplayedOnJoinOrRestore,
        D::RestoreReplaySafetyUnstated,
        D::ControlAcquiredWithoutExplicitGrant,
        D::CollaborationControlMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5CollaborationControlGovernanceReview {
    M5CollaborationControlGovernanceReview {
        no_presence_or_follow_implies_terminal_or_debug_control: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        active_driver_state_is_mechanically_distinct_from_viewer: true,
        every_sensitive_session_begins_view_first: true,
        every_control_grant_names_a_single_active_driver: true,
        every_join_discloses_recording_retention_and_guest_scope: true,
        no_recording_or_retention_starts_silently: true,
        every_presenter_handoff_names_holder_and_target: true,
        no_input_replay_on_join_or_restore: true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_collaboration_control_source: true,
        desktop_terminal_companion_incident_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_collaboration_control_vocabulary: true,
        collaboration_control_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5CollaborationControlConsumerProjection {
    M5CollaborationControlConsumerProjection {
        shared_terminal_debug_view_and_join_review_consume_shared_collaboration_control_truth: true,
        presenter_handoff_and_control_grant_consume_shared_authority_truth: true,
        help_and_support_export_consume_shared_consent_and_retention_truth: true,
        docs_help_and_screenshots_read_single_collaboration_control_source: true,
        companion_and_incident_surfaces_bind_to_shared_session_state_source: true,
        support_export_reads_single_collaboration_control_source: true,
    }
}

fn proof_freshness() -> M5CollaborationControlProofFreshness {
    M5CollaborationControlProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CollaborationControlReleasePosture {
    M5CollaborationControlReleasePosture {
        proof_packet_ref: M5_COLLABORATION_CONTROL_ARTIFACT_REF.to_owned(),
        collaboration_control_audit_ref: M5_COLLABORATION_CONTROL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_MATRIX_DOC_REF,
        M5_SHARED_TERMINAL_DEBUG_VIEW_DOMAIN_SCHEMA_REF,
        M5_CONTROL_GRANT_DOMAIN_SCHEMA_REF,
        M5_PRESENTER_TOKEN_DOMAIN_SCHEMA_REF,
        M5_CONSENT_ENVELOPE_DOMAIN_SCHEMA_REF,
        M5_RETENTION_REVIEW_DOMAIN_SCHEMA_REF,
        M5_SESSION_RESTORE_VIEW_DOMAIN_SCHEMA_REF,
        M5_PASTE_SECRET_GUARD_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 collaboration-control matrix packet.
pub fn seeded_m5_collaboration_control_matrix() -> M5CollaborationControlMatrixPacket {
    M5CollaborationControlMatrixPacket::new(M5CollaborationControlMatrixPacketInput {
        packet_id: M5_COLLABORATION_CONTROL_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 shared-terminal/debug-view, control-grant, presenter-token, consent-envelope, retention-review, and session-restore-view matrix"
            .to_owned(),
        collaboration_control_rows: collaboration_control_rows(),
        vocabulary_set: M5CollaborationControlVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the control grant is held at Beta because its single-active-driver enforcement is not yet
/// fully proven across every companion and incident surface; every object class stays visible.
pub fn seeded_m5_collaboration_control_matrix_control_grant_beta_narrowed(
) -> M5CollaborationControlMatrixPacket {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.packet_id = "m5-collaboration-control:control-grant-beta:0001".to_owned();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::ControlGrant)
        .expect("control-grant row present");
    row.qualification = M5CollaborationControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the session-restore view is narrowed to Preview pending durable replay-free and
/// retention-preserving restore proof; every object class stays visible.
pub fn seeded_m5_collaboration_control_matrix_session_restore_view_preview_narrowed(
) -> M5CollaborationControlMatrixPacket {
    let mut packet = seeded_m5_collaboration_control_matrix();
    packet.packet_id = "m5-collaboration-control:session-restore-view-preview:0001".to_owned();
    let row = packet
        .collaboration_control_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationControlObject::SessionRestoreView)
        .expect("session-restore-view row present");
    row.qualification = M5CollaborationControlQualificationClass::Preview;
    packet
}
