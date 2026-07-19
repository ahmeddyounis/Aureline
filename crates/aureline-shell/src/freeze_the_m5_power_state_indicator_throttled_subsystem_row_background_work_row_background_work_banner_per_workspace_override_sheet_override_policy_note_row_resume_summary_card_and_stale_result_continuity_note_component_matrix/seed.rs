// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the frozen M5 efficiency component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical efficiency component matrix.
pub const M5_EFFICIENCY_COMPONENT_MATRIX_PACKET_ID: &str = "m5-efficiency-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5EfficiencyRequiredLabel> {
    M5EfficiencyRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5EfficiencyRequiredLabel]) -> Vec<M5EfficiencyRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5EfficiencyComponentFamily,
    qualification: M5EfficiencyQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5EfficiencyComponentRow {
    M5EfficiencyComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5EfficiencySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5EfficiencyDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        work_dispositions: M5EfficiencyWorkDisposition::ALL.to_vec(),
        pressure_sources: vec![],
        efficiency_states: vec![],
        affected_workloads: vec![],
        override_postures: vec![],
        policy_owners: vec![],
        recovery_states: vec![],
        stale_result_states: vec![],
        degraded_reasons: M5EfficiencyDegradedReason::ALL.to_vec(),
        accessibility_routes: M5EfficiencyAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5EfficiencyConsumerSurface::SupportExport,
            M5EfficiencyConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5EfficiencyDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        collapses_pressure_sources_into_generic_warning: false,
        hides_paused_work_behind_toast_only: false,
        presents_override_available_when_policy_blocks: false,
        clears_stale_context_on_resume: false,
    }
}

fn component_rows() -> Vec<M5EfficiencyComponentRow> {
    use EfficiencyPressureSource as PS;
    use EfficiencyRecoveryState as RC;
    use EfficiencyState as ES;
    use M5EfficiencyComponentFamily as F;
    use M5EfficiencyConsumerSurface as C;
    use M5EfficiencyDowngradeTrigger as D;
    use M5EfficiencyPolicyOwner as PO;
    use M5EfficiencyQualificationClass as Q;
    use M5EfficiencyRequiredLabel as L;
    use M5EfficiencyStaleResultState as SR;
    use M5EfficiencyWorkDisposition as WD;
    use OverridePosture as OP;

    let mut rows = Vec::new();

    // 1. Power-state indicator.
    let mut row = base_row(
        F::PowerStateIndicator,
        Q::Stable,
        "Shell efficiency status owner",
        "One power-state-indicator model naming the source of change (AC power, battery, OS battery saver, user low-power mode, low or critical battery, thermal pressure, frame-miss pressure, policy cap, or pressure cleared) and the active efficiency state, so a user never has to infer why Aureline slowed down and battery saver, thermal pressure, low-power mode, and policy cap are never collapsed into one generic warning",
        "evidence:m5-power-state-indicator-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_POWER_STATE_INDICATOR_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.pressure_sources = PS::ALL.to_vec();
    row.efficiency_states = ES::ALL.to_vec();
    row.work_dispositions = vec![
        WD::RunningFull,
        WD::Slowed,
        WD::Paused,
        WD::PolicyBlocked,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::SourceOfChange]);
    row.consumer_surfaces = vec![
        C::ShellStatusUi,
        C::DiagnosticsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SourceOfChangeUnstated,
        D::EfficiencyStateUnstated,
        D::GenericLowPowerWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Throttled-subsystem row.
    let mut row = base_row(
        F::ThrottledSubsystemRow,
        Q::Stable,
        "Shell efficiency status owner",
        "One throttled-subsystem-row model naming exactly which subsystem's work is slowed or paused (AI warmups, prefetch, uploads, non-essential animation, indexing refresh, extension polling, preview refresh, graph enrichment, or remote/session helpers), so a user always knows which work was reduced and what still works",
        "evidence:m5-throttled-subsystem-row-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.affected_workloads = AFFECTED_WORKLOADS.to_vec();
    row.work_dispositions = vec![WD::RunningFull, WD::Slowed, WD::Paused, WD::NotEvaluated];
    row.required_labels = labels_with(&[L::SourceOfChange]);
    row.consumer_surfaces = vec![
        C::ShellStatusUi,
        C::ActivityCenterUi,
        C::DiagnosticsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SlowedVersusPausedAmbiguous,
        D::WhatStillWorksUnstated,
        D::GenericLowPowerWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Background-work row.
    let mut row = base_row(
        F::BackgroundWorkRow,
        Q::Stable,
        "Activity-center owner",
        "One background-work-row model naming a single deferred or slowed job's disposition (running full, slowed, paused, policy-blocked, resuming, or not evaluated), so a paused job is always shown explicitly and slowed-versus-paused work is never ambiguous",
        "evidence:m5-background-work-row-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.affected_workloads = AFFECTED_WORKLOADS.to_vec();
    row.work_dispositions = vec![
        WD::RunningFull,
        WD::Slowed,
        WD::Paused,
        WD::PolicyBlocked,
        WD::Resuming,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::SourceOfChange, L::ResumeAndStaleContinuity]);
    row.consumer_surfaces = vec![
        C::ActivityCenterUi,
        C::BackgroundWorkUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SlowedVersusPausedAmbiguous,
        D::PausedWorkToastOnly,
        D::WhatStillWorksUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Background-work banner.
    let mut row = base_row(
        F::BackgroundWorkBanner,
        Q::Stable,
        "Activity-center owner",
        "One background-work-banner model naming aggregate paused or slowed work explicitly and never behind toast-only messaging, so a user is never left to infer that background work paused when pressure is active",
        "evidence:m5-background-work-banner-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.affected_workloads = AFFECTED_WORKLOADS.to_vec();
    row.work_dispositions = vec![
        WD::Slowed,
        WD::Paused,
        WD::PolicyBlocked,
        WD::Resuming,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::SourceOfChange]);
    row.consumer_surfaces = vec![
        C::ShellStatusUi,
        C::ActivityCenterUi,
        C::BackgroundWorkUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PausedWorkToastOnly,
        D::SlowedVersusPausedAmbiguous,
        D::GenericLowPowerWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Per-workspace override sheet.
    let mut row = base_row(
        F::PerWorkspaceOverrideSheet,
        Q::Stable,
        "Policy-aware settings owner",
        "One per-workspace-override-sheet model naming whether an adaptation can be overridden and by whom (not overridable, user override for this session, user override persistent, policy blocked, or admin controlled) and the policy owner, so an override never reads as available when policy blocks it",
        "evidence:m5-efficiency-override-sheet-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.override_postures = OP::ALL.to_vec();
    row.policy_owners = PO::ALL.to_vec();
    row.work_dispositions = vec![
        WD::RunningFull,
        WD::Slowed,
        WD::Paused,
        WD::PolicyBlocked,
        WD::OverrideAvailable,
        WD::OverrideBlocked,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OverrideAndPolicyOwner]);
    row.consumer_surfaces = vec![
        C::OverrideSettingsUi,
        C::ShellStatusUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OverrideAvailabilityUnstated,
        D::PolicyOwnerUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Override-policy note row.
    let mut row = base_row(
        F::OverridePolicyNoteRow,
        Q::Stable,
        "Policy-aware settings owner",
        "One override-policy-note-row model naming the policy owner behind an adaptation (user controlled, local policy, admin policy, provider policy, or no owner resolved), so the accountable policy owner is never left implicit next to an override",
        "evidence:m5-override-policy-note-row-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.override_postures = OP::ALL.to_vec();
    row.policy_owners = PO::ALL.to_vec();
    row.work_dispositions = vec![
        WD::PolicyBlocked,
        WD::OverrideAvailable,
        WD::OverrideBlocked,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::OverrideAndPolicyOwner]);
    row.consumer_surfaces = vec![
        C::OverrideSettingsUi,
        C::HelpAboutUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PolicyOwnerUnstated,
        D::OverrideAvailabilityUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Resume-summary card.
    let mut row = base_row(
        F::ResumeSummaryCard,
        Q::Stable,
        "Activity-center owner",
        "One resume-summary-card model naming the resumed-work backlog after pressure cleared and the recovery state (not in recovery, staged resume, awaiting user restore power, awaiting reconnect, awaiting admin policy, or recovered), so a user always sees what resumed and what still waits when pressure ends",
        "evidence:m5-resume-summary-card-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.recovery_states = RC::ALL.to_vec();
    row.work_dispositions = vec![
        WD::RunningFull,
        WD::Paused,
        WD::Resuming,
        WD::StaleResultShown,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::ResumeAndStaleContinuity]);
    row.consumer_surfaces = vec![
        C::ActivityCenterUi,
        C::BackgroundWorkUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ResumeBacklogHidden,
        D::StaleResultContinuityCleared,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Stale-result continuity note.
    let mut row = base_row(
        F::StaleResultContinuityNote,
        Q::Stable,
        "Activity-center owner",
        "One stale-result-continuity-note model naming whether a result is fresh, retained-but-stale, refreshing, superseded, or of unknown continuity, so stale-result context is never cleared merely because background work resumed",
        "evidence:m5-stale-result-continuity-note-parity:001",
        &[
            M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
            M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ],
    );
    row.stale_result_states = SR::ALL.to_vec();
    row.work_dispositions = vec![
        WD::RunningFull,
        WD::Resuming,
        WD::StaleResultShown,
        WD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::ResumeAndStaleContinuity]);
    row.consumer_surfaces = vec![
        C::ActivityCenterUi,
        C::DiagnosticsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StaleResultContinuityCleared,
        D::ResumeBacklogHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5EfficiencyComponentGovernanceReview {
    M5EfficiencyComponentGovernanceReview {
        power_state_indicator_shows_source_and_state: true,
        throttled_subsystem_row_shows_affected_workload: true,
        background_work_row_shows_slowed_versus_paused: true,
        background_work_banner_shows_paused_work_explicitly: true,
        per_workspace_override_sheet_shows_override_availability: true,
        override_policy_note_row_shows_policy_owner: true,
        resume_summary_card_shows_resumed_backlog: true,
        stale_result_continuity_note_keeps_stale_context: true,
        no_surface_collapses_pressure_into_generic_warning: true,
        source_of_change_always_explicit: true,
        active_efficiency_state_always_explicit: true,
        slowed_versus_paused_always_explicit: true,
        override_availability_and_policy_owner_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5EfficiencyComponentConsumerProjection {
    M5EfficiencyComponentConsumerProjection {
        shell_surfaces_consume_state_vocabulary: true,
        activity_surfaces_consume_disposition_vocabulary: true,
        override_surfaces_consume_policy_vocabulary: true,
        resume_surfaces_consume_recovery_vocabulary: true,
        diagnostics_surfaces_consume_source_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5EfficiencyComponentProofFreshness {
    M5EfficiencyComponentProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EfficiencyComponentReleasePosture {
    M5EfficiencyComponentReleasePosture {
        proof_packet_ref: M5_EFFICIENCY_COMPONENT_ARTIFACT_REF.to_owned(),
        efficiency_audit_ref: M5_EFFICIENCY_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_POWER_STATE_INDICATOR_SCHEMA_REF,
        M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
        M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
        M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
        M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
        M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_MATRIX_REF,
    ])
}

/// Builds the canonical frozen M5 efficiency component matrix packet.
pub fn seeded_m5_efficiency_component_matrix() -> M5EfficiencyComponentMatrixPacket {
    M5EfficiencyComponentMatrixPacket::new(M5EfficiencyComponentMatrixPacketInput {
        packet_id: M5_EFFICIENCY_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 power-state-indicator, throttled-subsystem-row, background-work-row, background-work-banner, per-workspace-override-sheet, override-policy-note-row, resume-summary-card, and stale-result-continuity-note component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5EfficiencyComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the per-workspace override sheet is held at Beta because persistent
/// override round-trips are not yet proven across every deployment line; every component stays
/// visible.
pub fn seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed(
) -> M5EfficiencyComponentMatrixPacket {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.packet_id = "m5-efficiency-components:override-sheet-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet)
        .expect("per-workspace-override-sheet row present");
    row.qualification = M5EfficiencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the stale-result continuity note is narrowed to Preview pending
/// continuity-across-resume parity on every surface; every component stays visible.
pub fn seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed(
) -> M5EfficiencyComponentMatrixPacket {
    let mut packet = seeded_m5_efficiency_component_matrix();
    packet.packet_id = "m5-efficiency-components:stale-result-note-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5EfficiencyComponentFamily::StaleResultContinuityNote)
        .expect("stale-result-continuity-note row present");
    row.qualification = M5EfficiencyQualificationClass::Preview;
    packet
}
