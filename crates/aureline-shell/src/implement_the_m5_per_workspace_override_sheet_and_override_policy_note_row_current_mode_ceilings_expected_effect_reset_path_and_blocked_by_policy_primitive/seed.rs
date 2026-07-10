//! Canonical seed builders for the M5 override sheet / policy-note controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift. Every resolved example is built by calling the
//! real resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_OVERRIDE_CONTROLS_PACKET_ID: &str =
    "m5-override-sheet-policy-note-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn sheet(input: M5OverrideSheetResolutionInput) -> M5ResolvedOverrideSheet {
    resolve_override_sheet(input).expect("seed override-sheet input resolves")
}

fn note(input: M5PolicyNoteResolutionInput) -> M5ResolvedPolicyNoteRow {
    resolve_policy_note_row(input).expect("seed policy-note input resolves")
}

// -- Canonical override-sheet examples --------------------------------------------------------

/// Clean sheet: a user-overridable adaptation that previews the current mode, allowed ceilings,
/// expected effect, trade-off, and reset path — the honest baseline.
fn sheet_clean_user_override() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:workspace-user".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![
            WorkloadFamily::IndexingRefresh,
            WorkloadFamily::AiWarmup,
            WorkloadFamily::ExtensionPolling,
        ],
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Clean sheet: a policy-blocked adaptation shown as blocked-by-policy rather than as an
/// actionable control — proves AC1's positive half.
fn sheet_clean_blocked_shown() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:workspace-blocked".to_owned(),
        current_mode: EfficiencyState::ThermalConstrained,
        expected_effect_workloads: vec![WorkloadFamily::ExtensionPolling],
        override_posture: OverridePosture::PolicyBlocked,
        override_presented_actionable: false,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: a policy-blocked override is still presented as an actionable control — proves
/// AC1's negative half (a dead / misleading control).
fn sheet_dead_control() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:dead-control".to_owned(),
        current_mode: EfficiencyState::ProtectCore,
        expected_effect_workloads: vec![WorkloadFamily::IndexingRefresh],
        override_posture: OverridePosture::PolicyBlocked,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: the performance-versus-freshness trade-off is unstated — proves AC2's first
/// half.
fn sheet_tradeoff_unstated() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:tradeoff-unstated".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![WorkloadFamily::AiWarmup],
        override_posture: OverridePosture::UserOverridePersistent,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: false,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: side effects are hidden behind generic efficiency language — proves AC2's
/// second half.
fn sheet_generic_language() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:generic-language".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![WorkloadFamily::PreviewRefresh],
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: true,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: no expected effect on any workload was named.
fn sheet_effect_unstated() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:effect-unstated".to_owned(),
        current_mode: EfficiencyState::ThermalConstrained,
        expected_effect_workloads: vec![],
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: the allowed policy ceiling is unstated.
fn sheet_ceiling_unstated() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:ceiling-unstated".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![WorkloadFamily::GraphEnrichment],
        override_posture: OverridePosture::UserOverridePersistent,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::LocalPolicy,
        allowed_ceiling_stated: false,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset-to-policy-default".to_owned()),
        proof_fresh: true,
    })
}

/// Degraded sheet: the exact reset path is unstated.
fn sheet_no_reset() -> M5ResolvedOverrideSheet {
    sheet(M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:no-reset".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![WorkloadFamily::SpeculativePrefetch],
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: None,
        proof_fresh: true,
    })
}

// -- Canonical override-policy note-row examples ----------------------------------------------

/// Clean note: a user-overridable adaptation with a named owner and locally-changeable lanes.
fn note_clean_user() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:workspace-user".to_owned(),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        block_reason_explained: true,
        locally_changeable: vec![
            WorkloadFamily::IndexingRefresh,
            WorkloadFamily::PreviewRefresh,
        ],
        proof_fresh: true,
    })
}

/// Clean note: an admin-blocked adaptation explained as blocked-by-policy with the owner named and
/// the locally-changeable lanes stated — proves AC1's positive half for the note.
fn note_clean_blocked_explained() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:workspace-blocked".to_owned(),
        override_posture: OverridePosture::AdminControlled,
        override_presented_actionable: false,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        block_reason_explained: true,
        locally_changeable: vec![WorkloadFamily::PreviewRefresh],
        proof_fresh: true,
    })
}

/// Degraded note: a policy-blocked override is still presented as an actionable control — proves
/// AC1's negative half for the note.
fn note_dead_control() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:dead-control".to_owned(),
        override_posture: OverridePosture::PolicyBlocked,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        block_reason_explained: true,
        locally_changeable: vec![WorkloadFamily::PreviewRefresh],
        proof_fresh: true,
    })
}

/// Degraded note: no policy owner could be resolved.
fn note_owner_unresolved() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:owner-unresolved".to_owned(),
        override_posture: OverridePosture::AdminControlled,
        override_presented_actionable: false,
        policy_owner: M5EfficiencyPolicyOwner::NoOwnerResolved,
        block_reason_explained: true,
        locally_changeable: vec![WorkloadFamily::PreviewRefresh],
        proof_fresh: true,
    })
}

/// Degraded note: the override is blocked but the note does not explain when or why.
fn note_block_unexplained() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:block-unexplained".to_owned(),
        override_posture: OverridePosture::AdminControlled,
        override_presented_actionable: false,
        policy_owner: M5EfficiencyPolicyOwner::AdminPolicy,
        block_reason_explained: false,
        locally_changeable: vec![WorkloadFamily::PreviewRefresh],
        proof_fresh: true,
    })
}

/// Degraded note: what remains changeable locally is unstated.
fn note_locally_unstated() -> M5ResolvedPolicyNoteRow {
    note(M5PolicyNoteResolutionInput {
        note_id: "policy-note:locally-unstated".to_owned(),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        block_reason_explained: true,
        locally_changeable: vec![],
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5OverrideConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    override_sheet_examples: Vec<M5ResolvedOverrideSheet>,
    policy_note_examples: Vec<M5ResolvedPolicyNoteRow>,
) -> M5OverrideControlsRow {
    M5OverrideControlsRow {
        consumer_surface,
        qualification: M5EfficiencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EfficiencyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EfficiencyRequiredLabel::Identity,
            M5EfficiencyRequiredLabel::State,
            M5EfficiencyRequiredLabel::KeyboardRoute,
            M5EfficiencyRequiredLabel::OverrideAndPolicyOwner,
        ],
        accessibility_routes: M5EfficiencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5OverrideAnatomyPart::ALL.to_vec(),
        export_fields: M5OverrideExportField::ALL.to_vec(),
        downgrade_triggers,
        override_sheet_examples,
        policy_note_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_OVERRIDE_CONTROLS_SCHEMA_REF,
            M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
            M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ]),
        presents_override_available_when_policy_blocks: false,
        hides_side_effects_behind_generic_efficiency_language: false,
        collapses_pressure_sources_into_generic_warning: false,
        hides_what_remains_changeable_locally: false,
    }
}

fn controls_rows() -> Vec<M5OverrideControlsRow> {
    use M5EfficiencyConsumerSurface as C;
    use M5EfficiencyDowngradeTrigger as D;

    vec![
        base_row(
            C::OverrideSettingsUi,
            "Override / policy-aware settings owner",
            "The override / policy-aware settings surface renders the per-workspace override sheet that previews the current efficiency mode, the allowed policy ceilings, the expected effect on indexing, AI, and extensions, and the exact reset path, next to the policy note that names the owner and what stays changeable locally",
            "evidence:m5-override-settings:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::PolicyOwnerUnstated,
                D::GenericLowPowerWordingUsed,
                D::ProofStale,
            ],
            vec![sheet_clean_user_override(), sheet_clean_blocked_shown()],
            vec![note_clean_user(), note_clean_blocked_explained()],
        ),
        base_row(
            C::ShellStatusUi,
            "Shell efficiency status owner",
            "The shell status surface links to the per-workspace override sheet and renders the compact policy note explaining who owns an active adaptation and what remains changeable locally",
            "evidence:m5-override-shell-status:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::PolicyOwnerUnstated,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![sheet_clean_user_override()],
            vec![note_clean_blocked_explained()],
        ),
        base_row(
            C::ActivityCenterUi,
            "Activity-center owner",
            "The activity center surfaces the override sheet for an adapting job and the policy note that keeps a blocked override shown as blocked-by-policy rather than as an actionable control",
            "evidence:m5-override-activity-center:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::PolicyOwnerUnstated,
                D::ProofStale,
            ],
            vec![sheet_clean_blocked_shown()],
            vec![note_clean_user()],
        ),
        base_row(
            C::DiagnosticsUi,
            "Shell diagnostics owner",
            "Diagnostics surfaces the same override and policy truth, degrading honestly when a blocked override is presented as a dead control, when the performance-versus-freshness trade-off is unstated, when side effects are hidden behind generic language, or when the expected effect is unnamed",
            "evidence:m5-override-diagnostics:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::GenericLowPowerWordingUsed,
                D::WhatStillWorksUnstated,
                D::PolicyOwnerUnstated,
                D::ProofStale,
            ],
            vec![
                sheet_dead_control(),
                sheet_tradeoff_unstated(),
                sheet_generic_language(),
                sheet_effect_unstated(),
            ],
            vec![note_dead_control(), note_owner_unresolved()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved override and policy truth, so an unstated ceiling, a missing reset path, an unexplained block, or an unstated locally-changeable list is visible in evidence rather than hidden",
            "evidence:m5-override-support-export:001",
            vec![
                D::OverrideAvailabilityUnstated,
                D::PolicyOwnerUnstated,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![sheet_ceiling_unstated(), sheet_no_reset()],
            vec![note_block_unexplained(), note_locally_unstated()],
        ),
    ]
}

fn governance_review() -> M5OverrideGovernanceReview {
    M5OverrideGovernanceReview {
        sheet_previews_current_mode: true,
        sheet_states_allowed_ceilings: true,
        sheet_states_expected_effect: true,
        sheet_states_reset_path: true,
        sheet_states_performance_freshness_tradeoff: true,
        no_dead_override_control_when_policy_blocks: true,
        note_names_policy_owner: true,
        note_states_local_changeability: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5OverrideConsumerProjection {
    M5OverrideConsumerProjection {
        override_settings_consumes_shared_sheet: true,
        shell_and_activity_consume_shared_note: true,
        diagnostics_consumes_override_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5OverrideProofFreshness {
    M5OverrideProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OverrideReleasePosture {
    M5OverrideReleasePosture {
        proof_packet_ref: M5_OVERRIDE_CONTROLS_ARTIFACT_REF.to_owned(),
        efficiency_audit_ref: M5_OVERRIDE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_OVERRIDE_CONTROLS_SCHEMA_REF,
        M5_OVERRIDE_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
        M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 override sheet / policy-note controls packet.
pub fn seeded_m5_override_controls() -> M5OverrideControlsPacket {
    M5OverrideControlsPacket::new(M5OverrideControlsPacketInput {
        packet_id: M5_OVERRIDE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 per-workspace override-sheet and override-policy note-row controls with current mode, allowed ceilings, expected effect, reset path, and blocked-by-policy truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5OverrideVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the override-settings row is held at Beta pending reset-path parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_override_controls_override_settings_beta_narrowed() -> M5OverrideControlsPacket {
    let mut packet = seeded_m5_override_controls();
    packet.packet_id =
        "m5-override-sheet-policy-note-controls:override-settings-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::OverrideSettingsUi)
        .expect("override-settings row present");
    row.qualification = M5EfficiencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the activity-center row is narrowed to Preview pending policy-note parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_override_controls_activity_center_preview_narrowed() -> M5OverrideControlsPacket {
    let mut packet = seeded_m5_override_controls();
    packet.packet_id =
        "m5-override-sheet-policy-note-controls:activity-center-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .expect("activity-center row present");
    row.qualification = M5EfficiencyQualificationClass::Preview;
    packet
}
