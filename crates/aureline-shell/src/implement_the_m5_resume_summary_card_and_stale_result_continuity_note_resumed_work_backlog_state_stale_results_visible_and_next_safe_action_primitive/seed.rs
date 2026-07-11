//! Canonical seed builders for the M5 resume-summary / stale-result-note controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift. Every resolved example is built by calling the
//! real resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

use crate::efficiency::governance::EfficiencyRecoveryState;

/// Stable packet id for the canonical controls packet.
pub const M5_RESUME_CONTROLS_PACKET_ID: &str = "m5-resume-summary-stale-note-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(input: M5ResumeSummaryCardResolutionInput) -> M5ResolvedResumeSummaryCard {
    resolve_resume_summary_card(input).expect("seed resume-summary-card input resolves")
}

fn note(input: M5StaleResultNoteResolutionInput) -> M5ResolvedStaleResultNote {
    resolve_stale_result_continuity_note(input).expect("seed stale-result-note input resolves")
}

// -- Canonical resume-summary-card examples ---------------------------------------------------

/// Clean card: a staged resume that lists resumed work, the remaining backlog, keeps the retained
/// stale result visible, and states the next safe action — the honest baseline.
fn card_clean_staged_resume() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:workspace-staged".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![WorkloadFamily::IndexingRefresh, WorkloadFamily::AiWarmup],
        backlog_workloads: vec![WorkloadFamily::PreviewRefresh],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        stale_results_visible: true,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

/// Clean card: a fully recovered task with no remaining backlog and a fresh result — proves the
/// nominal-recovery path stays honest.
fn card_clean_recovered() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:workspace-recovered".to_owned(),
        recovery_state: EfficiencyRecoveryState::Recovered,
        resumed_workloads: vec![
            WorkloadFamily::IndexingRefresh,
            WorkloadFamily::AiWarmup,
            WorkloadFamily::ExtensionPolling,
        ],
        backlog_workloads: vec![],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::FreshResult,
        stale_results_visible: false,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

/// Degraded card: a live stale result is dropped from the summary once recovery completed — proves
/// AC1's negative half for the card.
fn card_stale_dropped() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:stale-dropped".to_owned(),
        recovery_state: EfficiencyRecoveryState::Recovered,
        resumed_workloads: vec![WorkloadFamily::IndexingRefresh],
        backlog_workloads: vec![],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        stale_results_visible: false,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

/// Degraded card: the recovery summary is not durable, so recovery must be inferred from a
/// disappearing banner — proves AC2's first half.
fn card_not_durable() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:not-durable".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![WorkloadFamily::AiWarmup],
        backlog_workloads: vec![WorkloadFamily::IndexingRefresh],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::FreshResult,
        stale_results_visible: false,
        durable_summary_present: false,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

/// Degraded card: the resumed-work backlog is hidden, so it must be inferred from background queue
/// motion — proves AC2's second half.
fn card_backlog_hidden() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:backlog-hidden".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![WorkloadFamily::IndexingRefresh],
        backlog_workloads: vec![],
        backlog_known: false,
        stale_result_state: M5EfficiencyStaleResultState::FreshResult,
        stale_results_visible: false,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

/// Degraded card: the safest next action for the current task is unstated.
fn card_next_action_unstated() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:next-unstated".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![WorkloadFamily::IndexingRefresh],
        backlog_workloads: vec![],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::FreshResult,
        stale_results_visible: false,
        durable_summary_present: true,
        next_safe_action_stated: false,
        proof_fresh: true,
    })
}

/// Degraded card: no resumed workload was named, so nothing can be summarized.
fn card_resumed_unnamed() -> M5ResolvedResumeSummaryCard {
    card(M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:resumed-unnamed".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![],
        backlog_workloads: vec![WorkloadFamily::IndexingRefresh],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::FreshResult,
        stale_results_visible: false,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    })
}

// -- Canonical stale-result continuity-note examples ------------------------------------------

/// Clean note: a retained stale result is kept visible after recovery, stated to be based on a
/// prior constrained state — proves AC1's positive half for the note.
fn note_clean_retained() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:retained".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        returned_to_nominal: true,
        stale_results_visible: true,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

/// Clean note: a stale result is refreshing, kept visible, with its refresh path stated.
fn note_clean_refreshing() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:refreshing".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRefreshing,
        returned_to_nominal: true,
        stale_results_visible: true,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

/// Clean note: a stale result has been superseded by a fresh one, so continuity is resolved.
fn note_clean_superseded() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:superseded".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultSuperseded,
        returned_to_nominal: true,
        stale_results_visible: false,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

/// Degraded note: a live stale result is silently removed from view on resume — proves AC1's
/// negative half for the note.
fn note_silently_removed() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:silently-removed".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        returned_to_nominal: true,
        stale_results_visible: false,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

/// Degraded note: a still-visible stale result does not state that it is based on a prior
/// constrained state.
fn note_prior_unstated() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:prior-unstated".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        returned_to_nominal: true,
        stale_results_visible: true,
        based_on_constrained_state_stated: false,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

/// Degraded note: a refreshing stale result does not state its refresh path.
fn note_refresh_unstated() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:refresh-unstated".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRefreshing,
        returned_to_nominal: true,
        stale_results_visible: true,
        based_on_constrained_state_stated: true,
        refresh_path_stated: false,
        proof_fresh: true,
    })
}

/// Degraded note: continuity of the result cannot be determined.
fn note_continuity_unknown() -> M5ResolvedStaleResultNote {
    note(M5StaleResultNoteResolutionInput {
        note_id: "stale-note:continuity-unknown".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::ContinuityUnknown,
        returned_to_nominal: true,
        stale_results_visible: false,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ResumeConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    resume_summary_examples: Vec<M5ResolvedResumeSummaryCard>,
    stale_result_note_examples: Vec<M5ResolvedStaleResultNote>,
) -> M5ResumeControlsRow {
    M5ResumeControlsRow {
        consumer_surface,
        qualification: M5EfficiencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EfficiencyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EfficiencyRequiredLabel::Identity,
            M5EfficiencyRequiredLabel::State,
            M5EfficiencyRequiredLabel::KeyboardRoute,
            M5EfficiencyRequiredLabel::ResumeAndStaleContinuity,
        ],
        accessibility_routes: M5EfficiencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ResumeAnatomyPart::ALL.to_vec(),
        export_fields: M5ResumeExportField::ALL.to_vec(),
        downgrade_triggers,
        resume_summary_examples,
        stale_result_note_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RESUME_CONTROLS_SCHEMA_REF,
            M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
            M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ]),
        clears_stale_result_context_on_resume: false,
        requires_inferring_recovery_from_transient_banners: false,
        hides_resumed_work_backlog: false,
        collapses_pressure_sources_into_generic_warning: false,
    }
}

fn controls_rows() -> Vec<M5ResumeControlsRow> {
    use M5EfficiencyConsumerSurface as C;
    use M5EfficiencyDowngradeTrigger as D;

    vec![
        base_row(
            C::ActivityCenterUi,
            "Activity-center owner",
            "The activity center renders the durable resume-summary card that lists what resumed, what backlog remains, whether stale results are still visible, and the safest next action, next to the stale-result continuity note that keeps a retained or refreshing result visible after recovery",
            "evidence:m5-resume-activity-center:001",
            vec![
                D::ResumeBacklogHidden,
                D::StaleResultContinuityCleared,
                D::PausedWorkToastOnly,
                D::ProofStale,
            ],
            vec![card_clean_staged_resume(), card_clean_recovered()],
            vec![note_clean_retained(), note_clean_refreshing()],
        ),
        base_row(
            C::ShellStatusUi,
            "Shell efficiency status owner",
            "The shell status surface links to the durable resume summary and renders the compact stale-result continuity note explaining that a still-visible result is based on a prior constrained state",
            "evidence:m5-resume-shell-status:001",
            vec![
                D::StaleResultContinuityCleared,
                D::ResumeBacklogHidden,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![card_clean_staged_resume()],
            vec![note_clean_superseded()],
        ),
        base_row(
            C::BackgroundWorkUi,
            "Background-work owner",
            "The background-work surface pairs the resumed-work backlog with the stale-result continuity note so a resumed job never clears the evidence that its last result is still stale",
            "evidence:m5-resume-background-work:001",
            vec![
                D::ResumeBacklogHidden,
                D::StaleResultContinuityCleared,
                D::ProofStale,
            ],
            vec![card_clean_recovered()],
            vec![note_clean_retained()],
        ),
        base_row(
            C::DiagnosticsUi,
            "Shell diagnostics owner",
            "Diagnostics surfaces the same resume and stale-result truth, degrading honestly when a live stale result is dropped on resume, when the recovery summary is not durable, when the resumed-work backlog is hidden, or when the next safe action is unstated",
            "evidence:m5-resume-diagnostics:001",
            vec![
                D::StaleResultContinuityCleared,
                D::PausedWorkToastOnly,
                D::ResumeBacklogHidden,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![
                card_stale_dropped(),
                card_not_durable(),
                card_backlog_hidden(),
                card_next_action_unstated(),
                card_resumed_unnamed(),
            ],
            vec![
                note_silently_removed(),
                note_prior_unstated(),
                note_refresh_unstated(),
                note_continuity_unknown(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved resume and stale-result truth, so a dropped stale result, a hidden backlog, a non-durable summary, or an unstated prior-constrained-state caveat is visible in evidence rather than hidden",
            "evidence:m5-resume-support-export:001",
            vec![
                D::StaleResultContinuityCleared,
                D::ResumeBacklogHidden,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![card_clean_recovered()],
            vec![note_clean_retained()],
        ),
    ]
}

fn governance_review() -> M5ResumeGovernanceReview {
    M5ResumeGovernanceReview {
        card_lists_resumed_work: true,
        card_states_remaining_backlog: true,
        card_states_stale_result_visibility: true,
        card_states_next_safe_action: true,
        recovery_summary_is_durable: true,
        no_stale_result_context_cleared_on_resume: true,
        note_states_prior_constrained_state: true,
        note_keeps_stale_result_visible: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ResumeConsumerProjection {
    M5ResumeConsumerProjection {
        activity_center_consumes_shared_card: true,
        shell_and_background_consume_shared_note: true,
        diagnostics_consumes_resume_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ResumeProofFreshness {
    M5ResumeProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ResumeReleasePosture {
    M5ResumeReleasePosture {
        proof_packet_ref: M5_RESUME_CONTROLS_ARTIFACT_REF.to_owned(),
        efficiency_audit_ref: M5_RESUME_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RESUME_CONTROLS_SCHEMA_REF,
        M5_RESUME_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
        M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 resume-summary / stale-result-note controls packet.
pub fn seeded_m5_resume_controls() -> M5ResumeControlsPacket {
    M5ResumeControlsPacket::new(M5ResumeControlsPacketInput {
        packet_id: M5_RESUME_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 resume-summary card and stale-result continuity note controls with resumed work, remaining backlog, stale-results-still-visible truth, and next safe action"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ResumeVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the activity-center row is held at Beta pending durable-summary parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_resume_controls_activity_center_beta_narrowed() -> M5ResumeControlsPacket {
    let mut packet = seeded_m5_resume_controls();
    packet.packet_id = "m5-resume-summary-stale-note-controls:activity-center-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .expect("activity-center row present");
    row.qualification = M5EfficiencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the background-work row is narrowed to Preview pending stale-result-note
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_resume_controls_background_work_preview_narrowed() -> M5ResumeControlsPacket {
    let mut packet = seeded_m5_resume_controls();
    packet.packet_id =
        "m5-resume-summary-stale-note-controls:background-work-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::BackgroundWorkUi)
        .expect("background-work row present");
    row.qualification = M5EfficiencyQualificationClass::Preview;
    packet
}
