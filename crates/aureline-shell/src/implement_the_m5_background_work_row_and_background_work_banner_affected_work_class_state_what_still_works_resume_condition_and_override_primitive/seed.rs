//! Canonical seed builders for the M5 background-work row / banner controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift. Every resolved example is built by calling the
//! real resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_BACKGROUND_WORK_CONTROLS_PACKET_ID: &str =
    "m5-background-work-row-banner-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row(input: M5BackgroundWorkRowResolutionInput) -> M5ResolvedBackgroundWorkRow {
    resolve_background_work_row(input).expect("seed background-work row input resolves")
}

fn banner(input: M5BackgroundWorkBannerResolutionInput) -> M5ResolvedBackgroundWorkBanner {
    resolve_background_work_banner(input).expect("seed background-work banner input resolves")
}

// -- Canonical background-work row examples ---------------------------------------------------

/// Clean row: paused indexing that stays reviewable in a durable surface — proves AC1.
fn row_clean_paused_indexing() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:indexing-paused".to_owned(),
        affected_work_class: Some(WorkloadFamily::IndexingRefresh),
        paused: true,
        slowed: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: strings(&["typing and editing", "save", "quick open"]),
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Clean row: slowed AI warmups that remain reviewable with a staged resume condition.
fn row_clean_slowed_ai() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:ai-warmup-slowed".to_owned(),
        affected_work_class: Some(WorkloadFamily::AiWarmup),
        paused: false,
        slowed: true,
        resume_condition: Some(EfficiencyRecoveryState::AwaitingUserRestorePower),
        override_posture: OverridePosture::NotOverridable,
        override_presented_available: false,
        policy_owner: M5EfficiencyPolicyOwner::LocalPolicy,
        preserved_protected_tasks: strings(&["save", "active preview"]),
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Clean row: a running-full preview job — no action needed.
fn row_clean_running() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:preview-running".to_owned(),
        affected_work_class: Some(WorkloadFamily::PreviewRefresh),
        paused: false,
        slowed: false,
        resume_condition: None,
        override_posture: OverridePosture::UserOverridePersistent,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: strings(&["save"]),
        adaptive_change_user_visible: false,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded row: adaptive change is user-visible but the row is only carried in a toast that
/// vanishes after dismissal — proves AC1's negative half.
fn row_toast_only() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:docs-sync-toast-only".to_owned(),
        affected_work_class: Some(WorkloadFamily::UploadTransfer),
        paused: true,
        slowed: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: strings(&["save"]),
        adaptive_change_user_visible: true,
        durable_surface_present: false,
        proof_fresh: true,
    })
}

/// Degraded row: a paused job with no stated resume condition.
fn row_no_resume() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:prebuild-no-resume".to_owned(),
        affected_work_class: Some(WorkloadFamily::GraphEnrichment),
        paused: true,
        slowed: false,
        resume_condition: None,
        override_posture: OverridePosture::AdminControlled,
        override_presented_available: false,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        preserved_protected_tasks: strings(&["save"]),
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded row: no affected work class was named at all.
fn row_unnamed() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:unnamed".to_owned(),
        affected_work_class: None,
        paused: true,
        slowed: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: strings(&["save"]),
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded row: an override is presented as available even though admin policy blocks it.
fn row_override_when_blocked() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:package-meta-override-blocked".to_owned(),
        affected_work_class: Some(WorkloadFamily::ExtensionPolling),
        paused: false,
        slowed: true,
        resume_condition: Some(EfficiencyRecoveryState::AwaitingAdminPolicy),
        override_posture: OverridePosture::PolicyBlocked,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        preserved_protected_tasks: strings(&["save"]),
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded row: what still works is unstated.
fn row_no_preserved() -> M5ResolvedBackgroundWorkRow {
    row(M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:no-preserved".to_owned(),
        affected_work_class: Some(WorkloadFamily::SpeculativePrefetch),
        paused: false,
        slowed: true,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: vec![],
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

// -- Canonical background-work banner examples -----------------------------------------------

/// Clean banner: repeated pressure coalesced into one durable banner naming the aggregate work.
fn banner_clean_coalesced() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:coalesced".to_owned(),
        slowed_workloads: vec![WorkloadFamily::AiWarmup, WorkloadFamily::GraphEnrichment],
        paused_workloads: vec![WorkloadFamily::IndexingRefresh],
        preserved_protected_tasks: strings(&["typing and editing", "save"]),
        pressure_event_count: 6,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded banner: repeated pressure emitted one toast each instead of one banner — proves AC2.
fn banner_duplicate_toast() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:toast-spam".to_owned(),
        slowed_workloads: vec![WorkloadFamily::PreviewRefresh],
        paused_workloads: vec![WorkloadFamily::IndexingRefresh],
        preserved_protected_tasks: strings(&["save"]),
        pressure_event_count: 5,
        coalesced_into_single_banner: false,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded banner: adaptive-efficiency truth collapsed into generic service-failure copy —
/// proves AC2's second half.
fn banner_generic_copy() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:generic-copy".to_owned(),
        slowed_workloads: vec![WorkloadFamily::AiWarmup],
        paused_workloads: vec![WorkloadFamily::UploadTransfer],
        preserved_protected_tasks: strings(&["save"]),
        pressure_event_count: 3,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: true,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded banner: paused work is present but hidden rather than shown explicitly.
fn banner_paused_hidden() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:paused-hidden".to_owned(),
        slowed_workloads: vec![WorkloadFamily::SpeculativePrefetch],
        paused_workloads: vec![WorkloadFamily::IndexingRefresh],
        preserved_protected_tasks: strings(&["save"]),
        pressure_event_count: 2,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: false,
        uses_generic_service_failure_copy: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded banner: an override is presented as available even though policy blocks it.
fn banner_override_blocked() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:override-blocked".to_owned(),
        slowed_workloads: vec![WorkloadFamily::ExtensionPolling],
        paused_workloads: vec![],
        preserved_protected_tasks: strings(&["save"]),
        pressure_event_count: 1,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: false,
        resume_condition: Some(EfficiencyRecoveryState::AwaitingAdminPolicy),
        override_posture: OverridePosture::PolicyBlocked,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

/// Degraded banner: no affected work was named across the aggregate.
fn banner_none_named() -> M5ResolvedBackgroundWorkBanner {
    banner(M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:none-named".to_owned(),
        slowed_workloads: vec![],
        paused_workloads: vec![],
        preserved_protected_tasks: strings(&["save"]),
        pressure_event_count: 1,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: false,
        resume_condition: None,
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5BackgroundWorkConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    background_work_row_examples: Vec<M5ResolvedBackgroundWorkRow>,
    background_work_banner_examples: Vec<M5ResolvedBackgroundWorkBanner>,
) -> M5BackgroundWorkControlsRow {
    M5BackgroundWorkControlsRow {
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
        anatomy_parts: M5BackgroundWorkAnatomyPart::ALL.to_vec(),
        export_fields: M5BackgroundWorkExportField::ALL.to_vec(),
        downgrade_triggers,
        background_work_row_examples,
        background_work_banner_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BACKGROUND_WORK_CONTROLS_SCHEMA_REF,
            M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
            M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ]),
        collapses_pressure_into_generic_service_failure: false,
        hides_paused_work_behind_toast_only: false,
        presents_override_available_when_policy_blocks: false,
        drops_background_work_after_toast_dismissal: false,
    }
}

fn controls_rows() -> Vec<M5BackgroundWorkControlsRow> {
    use M5EfficiencyConsumerSurface as C;
    use M5EfficiencyDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellStatusUi,
            "Shell efficiency status owner",
            "The shell status bar renders one durable background-work row per adapting job, naming the affected work class, its slowed-versus-paused state, what still works, and when it resumes, so paused indexing stays reviewable after the user looks away",
            "evidence:m5-background-work-shell-status:001",
            vec![
                D::PausedWorkToastOnly,
                D::ResumeBacklogHidden,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![row_clean_paused_indexing(), row_clean_running()],
            vec![banner_clean_coalesced()],
        ),
        base_row(
            C::ActivityCenterUi,
            "Activity-center owner",
            "The activity center renders the background-work banner that coalesces broad or repeated pressure into one durable surface and never spams a toast per event",
            "evidence:m5-background-work-activity-center:001",
            vec![
                D::PausedWorkToastOnly,
                D::GenericLowPowerWordingUsed,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![row_clean_slowed_ai()],
            vec![banner_clean_coalesced(), banner_duplicate_toast()],
        ),
        base_row(
            C::BackgroundWorkUi,
            "Background-work surface owner",
            "The background-work surface enumerates each adapting job and its aggregate banner, keeping paused work explicit and never hiding it behind toast-only messaging",
            "evidence:m5-background-work-surface:001",
            vec![
                D::PausedWorkToastOnly,
                D::SlowedVersusPausedAmbiguous,
                D::OverrideAvailabilityUnstated,
                D::ProofStale,
            ],
            vec![row_clean_paused_indexing()],
            vec![banner_clean_coalesced(), banner_paused_hidden()],
        ),
        base_row(
            C::DiagnosticsUi,
            "Shell diagnostics owner",
            "Diagnostics surfaces the same affected-work and resume truth, degrading honestly when a row is toast-only, a resume condition is unstated, a work class is unnamed, or a banner falls back to generic service-failure copy",
            "evidence:m5-background-work-diagnostics:001",
            vec![
                D::PausedWorkToastOnly,
                D::ResumeBacklogHidden,
                D::GenericLowPowerWordingUsed,
                D::ProofStale,
            ],
            vec![row_toast_only(), row_no_resume(), row_unnamed()],
            vec![banner_generic_copy(), banner_none_named()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved background-work truth, so a blocked override presented as available or an unstated preserved-work list is visible in evidence rather than hidden",
            "evidence:m5-background-work-support-export:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::WhatStillWorksUnstated,
                D::PolicyOwnerUnstated,
                D::ProofStale,
            ],
            vec![row_override_when_blocked(), row_no_preserved()],
            vec![banner_override_blocked()],
        ),
    ]
}

fn governance_review() -> M5BackgroundWorkGovernanceReview {
    M5BackgroundWorkGovernanceReview {
        row_names_affected_work_class: true,
        row_shows_slowed_versus_paused: true,
        always_names_what_still_works: true,
        resume_condition_stated_when_deferred: true,
        no_override_presented_when_policy_blocks: true,
        banner_shows_paused_work_explicitly: true,
        banner_coalesces_repeated_pressure: true,
        no_background_work_dropped_after_toast: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5BackgroundWorkConsumerProjection {
    M5BackgroundWorkConsumerProjection {
        shell_surfaces_consume_background_rows: true,
        activity_surfaces_consume_background_banner: true,
        diagnostics_surfaces_consume_work_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5BackgroundWorkProofFreshness {
    M5BackgroundWorkProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BackgroundWorkReleasePosture {
    M5BackgroundWorkReleasePosture {
        proof_packet_ref: M5_BACKGROUND_WORK_CONTROLS_ARTIFACT_REF.to_owned(),
        efficiency_audit_ref: M5_BACKGROUND_WORK_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BACKGROUND_WORK_CONTROLS_SCHEMA_REF,
        M5_BACKGROUND_WORK_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 background-work row / banner controls packet.
pub fn seeded_m5_background_work_controls() -> M5BackgroundWorkControlsPacket {
    M5BackgroundWorkControlsPacket::new(M5BackgroundWorkControlsPacketInput {
        packet_id: M5_BACKGROUND_WORK_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 background-work-row and background-work-banner controls with affected work class, slowed-versus-paused state, what-still-works, resume condition, and override truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5BackgroundWorkVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the activity-center row is held at Beta pending banner-coalescing parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_background_work_controls_activity_center_beta_narrowed(
) -> M5BackgroundWorkControlsPacket {
    let mut packet = seeded_m5_background_work_controls();
    packet.packet_id =
        "m5-background-work-row-banner-controls:activity-center-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .expect("activity-center row present");
    row.qualification = M5EfficiencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the background-work row is narrowed to Preview pending durable-surface
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_background_work_controls_background_work_preview_narrowed(
) -> M5BackgroundWorkControlsPacket {
    let mut packet = seeded_m5_background_work_controls();
    packet.packet_id =
        "m5-background-work-row-banner-controls:background-work-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::BackgroundWorkUi)
        .expect("background-work row present");
    row.qualification = M5EfficiencyQualificationClass::Preview;
    packet
}
