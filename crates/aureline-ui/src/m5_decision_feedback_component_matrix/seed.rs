//! Canonical seed builders for the frozen M5 decision-feedback component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical decision-feedback component matrix.
pub const M5_DECISION_FEEDBACK_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-decision-feedback-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every primitive must be able to show.
fn mandatory_labels() -> Vec<M5DecisionFeedbackRequiredLabel> {
    M5DecisionFeedbackRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a primitive carries.
fn labels_with(extra: &[M5DecisionFeedbackRequiredLabel]) -> Vec<M5DecisionFeedbackRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every primitive filled in and every family-specific vocabulary
/// left empty for the caller to populate.
fn base_row(
    component_family: M5DecisionFeedbackFamily,
    qualification: M5DecisionFeedbackQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5DecisionFeedbackComponentRow {
    M5DecisionFeedbackComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DecisionFeedbackSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DecisionFeedbackDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        badge_expressions: vec![],
        popover_dismissals: vec![],
        dialog_action_models: vec![],
        notice_scopes: vec![],
        toast_durabilities: vec![],
        empty_state_purposes: vec![],
        loading_fidelities: vec![],
        consequence_disclosures: vec![],
        degraded_reasons: M5DecisionFeedbackDegradedReason::ALL.to_vec(),
        accessibility_routes: M5DecisionFeedbackAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5DecisionFeedbackConsumerSurface::SupportExport,
            M5DecisionFeedbackConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5DecisionFeedbackDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        relies_on_color_alone_for_meaning: false,
        lets_popover_carry_only_critical_instruction: false,
        uses_generic_yes_no_in_high_risk_dialog: false,
        represents_durable_work_as_toast_only: false,
        blanks_useful_pane_during_loading: false,
        uses_full_screen_spinner_when_partial_capable: false,
    }
}

fn component_rows() -> Vec<M5DecisionFeedbackComponentRow> {
    use M5BadgeExpression as BE;
    use M5ConsequenceDisclosure as CD;
    use M5DecisionFeedbackConsumerSurface as C;
    use M5DecisionFeedbackDisposition as ST;
    use M5DecisionFeedbackDowngradeTrigger as D;
    use M5DecisionFeedbackFamily as F;
    use M5DecisionFeedbackQualificationClass as Q;
    use M5DecisionFeedbackRequiredLabel as L;
    use M5DialogActionModel as DA;
    use M5EmptyStatePurpose as EP;
    use M5LoadingFidelity as LF;
    use M5NoticeScope as NS;
    use M5PopoverDismissal as PD;
    use M5ToastDurability as TD;

    let mut rows = Vec::new();

    // 1. Badge / chip / pill.
    let mut row = base_row(
        F::BadgeChipPill,
        Q::Stable,
        "Design-system feedback owner",
        "One badge / chip / pill model that always expands into plain language (text label, icon-with-text, count-with-label, status word, removable chip) so badge and status meaning is never conveyed by color alone",
        "evidence:m5-badge-chip-pill-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_BADGE_CHIP_PILL_SCHEMA_REF],
    );
    row.badge_expressions = BE::ALL.to_vec();
    row.dispositions = vec![
        ST::Info,
        ST::Success,
        ST::Warning,
        ST::Blocked,
        ST::Degraded,
    ];
    row.required_labels = labels_with(&[L::Rationale]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::SettingsUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ColorAloneUsedForMeaning,
        D::RationaleUnstated,
        D::StateTaxonomyDrifted,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Popover.
    let mut row = base_row(
        F::Popover,
        Q::Stable,
        "Design-system feedback owner",
        "One popover model naming how it dismisses and returns focus (outside-click, escape, explicit close, focus-returns-to-trigger, non-modal secondary) so a popover stays a lightweight secondary control and never carries the only critical workflow instruction",
        "evidence:m5-popover-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_POPOVER_SCHEMA_REF],
    );
    row.popover_dismissals = PD::ALL.to_vec();
    row.dispositions = vec![ST::Info, ST::Pending, ST::Acknowledged, ST::Dismissed];
    row.required_labels = labels_with(&[L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::SettingsUi,
        C::HelpUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PopoverCarriedOnlyCriticalInstruction,
        D::RecoveryPathUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Dialog / sheet.
    let mut row = base_row(
        F::DialogSheet,
        Q::Stable,
        "Shell surface owner",
        "One dialog / sheet model naming its rationale, scope, and explicit named actions (named-specific-actions, primary-and-cancel, destructive-confirm-named, rationale-and-scope-stated, dismissible-safe) so a high-risk dialog never uses generic Yes/No copy",
        "evidence:m5-dialog-sheet-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_DIALOG_SHEET_SCHEMA_REF],
    );
    row.dialog_action_models = DA::ALL.to_vec();
    row.dispositions = vec![
        ST::Info,
        ST::Warning,
        ST::Blocked,
        ST::Pending,
        ST::Acknowledged,
        ST::Dismissed,
    ];
    row.required_labels = labels_with(&[L::Rationale, L::Scope, L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::SettingsUi,
        C::UpdatesUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GenericYesNoUsedInHighRiskDialog,
        D::RationaleUnstated,
        D::ScopeUnstated,
        D::RecoveryPathUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Banner / inline notice.
    let mut row = base_row(
        F::BannerInlineNotice,
        Q::Stable,
        "Notification surface owner",
        "One banner / inline-notice model naming its scope and next step (page-scoped, section-scoped, field-inline, global-system, actionable-with-next-step) so a notice stays scoped and actionable and never relies on color alone",
        "evidence:m5-banner-inline-notice-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_BANNER_INLINE_NOTICE_SCHEMA_REF],
    );
    row.notice_scopes = NS::ALL.to_vec();
    row.dispositions = vec![
        ST::Info,
        ST::Success,
        ST::Warning,
        ST::Blocked,
        ST::Degraded,
        ST::Dismissed,
    ];
    row.required_labels = labels_with(&[L::Scope, L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::UpdatesUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ColorAloneUsedForMeaning,
        D::ScopeUnstated,
        D::RecoveryPathUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Toast.
    let mut row = base_row(
        F::Toast,
        Q::Stable,
        "Notification surface owner",
        "One toast model naming its durability (transient acknowledgment, mirrored-to-activity-center, dismissible, auto-dismiss, action-retained-elsewhere) so a toast acknowledges work without becoming the only durable truth for long-running or reviewable work",
        "evidence:m5-toast-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_TOAST_SCHEMA_REF],
    );
    row.toast_durabilities = TD::ALL.to_vec();
    row.dispositions = vec![
        ST::Info,
        ST::Success,
        ST::Warning,
        ST::Pending,
        ST::Acknowledged,
        ST::Dismissed,
    ];
    row.required_labels = labels_with(&[L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::UpdatesUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DurableWorkShownAsToastOnly,
        D::RecoveryPathUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Empty state.
    let mut row = base_row(
        F::EmptyState,
        Q::Stable,
        "Shell surface owner",
        "One empty-state model naming what it explains (purpose, current emptiness, next action, first-run guidance, filtered-no-results) so a pane never renders blank with no explanation",
        "evidence:m5-empty-state-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_EMPTY_STATE_SCHEMA_REF],
    );
    row.empty_state_purposes = EP::ALL.to_vec();
    row.dispositions = vec![ST::Info, ST::Degraded];
    row.required_labels = labels_with(&[L::Rationale, L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RationaleUnstated,
        D::RecoveryPathUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Loading state.
    let mut row = base_row(
        F::LoadingState,
        Q::Stable,
        "Shell surface owner",
        "One loading-state model naming its representation (skeleton-preserves-layout, partial-data-retained, inline-progress-scoped, determinate progress, indeterminate spinner scoped) so useful panes are never blanked and a full-screen spinner is never used where partial capability exists",
        "evidence:m5-loading-state-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_LOADING_STATE_SCHEMA_REF],
    );
    row.loading_fidelities = LF::ALL.to_vec();
    row.dispositions = vec![ST::Info, ST::Pending, ST::Degraded];
    row.required_labels = labels_with(&[L::Rationale]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::UpdatesUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::UsefulPaneBlankedDuringLoading,
        D::FullScreenSpinnerWhenPartialCapable,
        D::StateTaxonomyDrifted,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Consequence block.
    let mut row = base_row(
        F::ConsequenceBlock,
        Q::Stable,
        "Repair surface owner",
        "One consequence-block model naming its blast radius and rollback / help posture (named-blast-radius, rollback-available, rollback-unavailable-stated, help-path-present, explicit-named-actions) so a risky action never reduces to generic Yes/No ambiguity",
        "evidence:m5-consequence-block-parity:001",
        &[M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_CONSEQUENCE_BLOCK_SCHEMA_REF],
    );
    row.consequence_disclosures = CD::ALL.to_vec();
    row.dispositions = vec![ST::Warning, ST::Blocked, ST::Pending, ST::Acknowledged];
    row.required_labels = labels_with(&[L::Scope, L::RecoveryPath]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::UpdatesUi,
        C::SupportUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GenericYesNoUsedInHighRiskDialog,
        D::ScopeUnstated,
        D::RecoveryPathUnstated,
        D::RationaleUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5DecisionFeedbackGovernanceReview {
    M5DecisionFeedbackGovernanceReview {
        badge_meaning_never_color_alone: true,
        popover_never_carries_only_critical_instruction: true,
        dialog_names_rationale_scope_and_explicit_actions: true,
        banner_and_inline_notice_stay_scoped_and_actionable: true,
        toast_never_the_only_durable_truth: true,
        empty_state_explains_purpose_emptiness_and_next_action: true,
        loading_state_preserves_useful_partial_data: true,
        consequence_block_names_blast_radius_and_rollback_posture: true,
        state_taxonomy_means_the_same_everywhere: true,
        no_generic_yes_no_in_high_risk_confirmation: true,
        no_full_screen_spinner_where_partial_capable: true,
        blocked_and_degraded_never_hidden_behind_generic_chrome: true,
        every_primitive_binds_to_one_rationale_or_recovery_path: true,
        every_primitive_declares_deployment_lines: true,
        every_primitive_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_feedback_vocabulary: true,
    }
}

fn consumer_projection() -> M5DecisionFeedbackConsumerProjection {
    M5DecisionFeedbackConsumerProjection {
        shell_and_notification_consume_shared_feedback_vocabulary: true,
        entry_and_trust_consume_shared_decision_vocabulary: true,
        review_consumes_shared_decision_and_feedback_vocabulary: true,
        repair_consumes_shared_consequence_vocabulary: true,
        help_and_updates_consume_shared_state_vocabulary: true,
        support_export_reads_single_feedback_source: true,
    }
}

fn proof_freshness() -> M5DecisionFeedbackProofFreshness {
    M5DecisionFeedbackProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DecisionFeedbackReleasePosture {
    M5DecisionFeedbackReleasePosture {
        proof_packet_ref: M5_DECISION_FEEDBACK_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_DECISION_FEEDBACK_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BADGE_CHIP_PILL_SCHEMA_REF,
        M5_POPOVER_SCHEMA_REF,
        M5_DIALOG_SHEET_SCHEMA_REF,
        M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
        M5_TOAST_SCHEMA_REF,
        M5_EMPTY_STATE_SCHEMA_REF,
        M5_LOADING_STATE_SCHEMA_REF,
        M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 decision-feedback component matrix packet.
pub fn seeded_m5_decision_feedback_component_matrix() -> M5DecisionFeedbackComponentMatrixPacket {
    M5DecisionFeedbackComponentMatrixPacket::new(M5DecisionFeedbackComponentMatrixPacketInput {
        packet_id: M5_DECISION_FEEDBACK_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 badge-chip-pill, popover, dialog-sheet, banner-inline-notice, toast, empty-state, loading-state, and consequence-block component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5DecisionFeedbackVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the dialog / sheet is held at Beta because rationale/scope parity is not yet proven
/// across every deployment line; every primitive stays visible.
pub fn seeded_m5_decision_feedback_component_matrix_dialog_sheet_beta_narrowed(
) -> M5DecisionFeedbackComponentMatrixPacket {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.packet_id = "m5-decision-feedback-components:dialog-sheet-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::DialogSheet)
        .expect("dialog-sheet row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the loading state is narrowed to Preview pending partial-data-preservation parity
/// across every deployment line; every primitive stays visible.
pub fn seeded_m5_decision_feedback_component_matrix_loading_state_preview_narrowed(
) -> M5DecisionFeedbackComponentMatrixPacket {
    let mut packet = seeded_m5_decision_feedback_component_matrix();
    packet.packet_id = "m5-decision-feedback-components:loading-state-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DecisionFeedbackFamily::LoadingState)
        .expect("loading-state row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Preview;
    packet
}
