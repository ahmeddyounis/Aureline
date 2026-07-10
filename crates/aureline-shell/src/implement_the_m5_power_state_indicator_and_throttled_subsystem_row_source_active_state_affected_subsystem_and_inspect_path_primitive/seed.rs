//! Canonical seed builders for the M5 power-state / throttled-subsystem controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift. Every resolved example is built by calling the
//! real resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_POWER_THROTTLE_CONTROLS_PACKET_ID: &str =
    "m5-power-state-throttled-subsystem-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn power_state(input: M5PowerStateResolutionInput) -> M5ResolvedPowerStateIndicator {
    resolve_power_state_indicator(input).expect("seed power-state input resolves")
}

fn throttled(input: M5ThrottledResolutionInput) -> M5ResolvedThrottledSubsystemRow {
    resolve_throttled_subsystem_row(input).expect("seed throttled input resolves")
}

// -- Canonical power-state examples ----------------------------------------------------------

/// Clean indicator naming two distinct causes (battery saver + thermal) — proves AC1.
fn power_clean_multi_cause() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:thermal-and-saver".to_owned(),
        pressure_sources: vec![
            EfficiencyPressureSource::OsBatterySaver,
            EfficiencyPressureSource::ThermalPressure,
        ],
        active_state: EfficiencyState::ThermalConstrained,
        pressure_signal_available: true,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

/// Clean indicator naming a single cause.
fn power_clean_single_cause() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:battery".to_owned(),
        pressure_sources: vec![EfficiencyPressureSource::Battery],
        active_state: EfficiencyState::EfficiencyAware,
        pressure_signal_available: true,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

/// Clean nominal indicator — full speed, no action needed.
fn power_clean_nominal() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:nominal".to_owned(),
        pressure_sources: vec![EfficiencyPressureSource::AcPower],
        active_state: EfficiencyState::Nominal,
        pressure_signal_available: true,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

/// Degraded indicator: distinct causes collapsed into one generic warning — proves AC1's
/// negative half (a collapsed cause never reads clean).
fn power_collapsed_generic() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:collapsed".to_owned(),
        pressure_sources: vec![
            EfficiencyPressureSource::OsBatterySaver,
            EfficiencyPressureSource::ThermalPressure,
            EfficiencyPressureSource::PolicyCap,
        ],
        active_state: EfficiencyState::EfficiencyAware,
        pressure_signal_available: true,
        distinct_causes_named: false,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

/// Degraded indicator: source of change unstated.
fn power_source_unstated() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:unstated".to_owned(),
        pressure_sources: vec![],
        active_state: EfficiencyState::EfficiencyAware,
        pressure_signal_available: true,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

/// Degraded indicator: the pressure signal could not be read at all.
fn power_signal_unavailable() -> M5ResolvedPowerStateIndicator {
    power_state(M5PowerStateResolutionInput {
        indicator_id: "power-state:signal-unavailable".to_owned(),
        pressure_sources: vec![],
        active_state: EfficiencyState::Nominal,
        pressure_signal_available: false,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency/power-state".to_owned(),
        proof_fresh: true,
    })
}

// -- Canonical throttled-subsystem examples --------------------------------------------------

/// Clean row enumerating slowed and paused lanes and what still works.
fn throttled_clean() -> M5ResolvedThrottledSubsystemRow {
    throttled(M5ThrottledResolutionInput {
        row_id: "throttled:core-preserved".to_owned(),
        slowed_workloads: vec![
            WorkloadFamily::SpeculativePrefetch,
            WorkloadFamily::IndexingRefresh,
        ],
        paused_workloads: vec![WorkloadFamily::AiWarmup],
        preserved_protected_tasks: strings(&["typing and editing", "save", "active preview"]),
        adaptive_behavior_user_visible: true,
        surface_hides_slowed_work: false,
        proof_fresh: true,
    })
}

/// Degraded row: slowed work already visible to the user is being hidden — proves AC2.
fn throttled_silently_hidden() -> M5ResolvedThrottledSubsystemRow {
    throttled(M5ThrottledResolutionInput {
        row_id: "throttled:upload-hidden".to_owned(),
        slowed_workloads: vec![WorkloadFamily::UploadTransfer],
        paused_workloads: vec![],
        preserved_protected_tasks: strings(&["save"]),
        adaptive_behavior_user_visible: true,
        surface_hides_slowed_work: true,
        proof_fresh: true,
    })
}

/// Degraded row: the same lane is both slowed and paused.
fn throttled_ambiguous() -> M5ResolvedThrottledSubsystemRow {
    throttled(M5ThrottledResolutionInput {
        row_id: "throttled:ambiguous-preview".to_owned(),
        slowed_workloads: vec![WorkloadFamily::PreviewRefresh],
        paused_workloads: vec![WorkloadFamily::PreviewRefresh],
        preserved_protected_tasks: strings(&["save"]),
        adaptive_behavior_user_visible: false,
        surface_hides_slowed_work: false,
        proof_fresh: true,
    })
}

/// Degraded row: what still works is unstated.
fn throttled_no_preserved() -> M5ResolvedThrottledSubsystemRow {
    throttled(M5ThrottledResolutionInput {
        row_id: "throttled:no-preserved".to_owned(),
        slowed_workloads: vec![WorkloadFamily::GraphEnrichment],
        paused_workloads: vec![],
        preserved_protected_tasks: vec![],
        adaptive_behavior_user_visible: false,
        surface_hides_slowed_work: false,
        proof_fresh: true,
    })
}

/// Degraded row: no affected subsystem was named at all.
fn throttled_none_named() -> M5ResolvedThrottledSubsystemRow {
    throttled(M5ThrottledResolutionInput {
        row_id: "throttled:none-named".to_owned(),
        slowed_workloads: vec![],
        paused_workloads: vec![],
        preserved_protected_tasks: strings(&["save"]),
        adaptive_behavior_user_visible: false,
        surface_hides_slowed_work: false,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

fn base_row(
    consumer_surface: M5PowerThrottleConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    power_state_examples: Vec<M5ResolvedPowerStateIndicator>,
    throttled_subsystem_examples: Vec<M5ResolvedThrottledSubsystemRow>,
) -> M5PowerThrottleControlsRow {
    M5PowerThrottleControlsRow {
        consumer_surface,
        qualification: M5EfficiencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EfficiencyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EfficiencyRequiredLabel::Identity,
            M5EfficiencyRequiredLabel::State,
            M5EfficiencyRequiredLabel::KeyboardRoute,
            M5EfficiencyRequiredLabel::SourceOfChange,
        ],
        accessibility_routes: M5EfficiencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PowerThrottleAnatomyPart::ALL.to_vec(),
        export_fields: M5PowerThrottleExportField::ALL.to_vec(),
        downgrade_triggers,
        power_state_examples,
        throttled_subsystem_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_POWER_THROTTLE_CONTROLS_SCHEMA_REF,
            M5_POWER_STATE_INDICATOR_SCHEMA_REF,
            M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
            M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
        ]),
        collapses_pressure_sources_into_generic_warning: false,
        hides_slowed_work_after_user_visible: false,
        hides_what_still_works: false,
        invents_alternate_state_label: false,
    }
}

fn controls_rows() -> Vec<M5PowerThrottleControlsRow> {
    use M5EfficiencyConsumerSurface as C;
    use M5EfficiencyDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellStatusUi,
            "Shell efficiency status owner",
            "The shell status bar renders one power-state indicator naming the source of change and active state, so a user reads why Aureline slowed down at a glance without opening logs",
            "evidence:m5-power-throttle-shell-status:001",
            vec![
                D::SourceOfChangeUnstated,
                D::GenericLowPowerWordingUsed,
                D::AlternateStateLabelInvented,
                D::ProofStale,
            ],
            vec![power_clean_multi_cause(), power_clean_nominal()],
            vec![throttled_clean()],
        ),
        base_row(
            C::ActivityCenterUi,
            "Activity-center owner",
            "The activity center renders throttled-subsystem rows that enumerate which lanes slowed or paused and never hide slowed work a user has already seen",
            "evidence:m5-power-throttle-activity-center:001",
            vec![
                D::SlowedVersusPausedAmbiguous,
                D::PausedWorkToastOnly,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![power_clean_single_cause()],
            vec![throttled_clean(), throttled_silently_hidden()],
        ),
        base_row(
            C::DiagnosticsUi,
            "Shell diagnostics owner",
            "Diagnostics surfaces the same source-of-change and affected-subsystem truth, degrading honestly when a signal is unavailable, a cause is unstated, or a lane is ambiguous",
            "evidence:m5-power-throttle-diagnostics:001",
            vec![
                D::SourceOfChangeUnstated,
                D::SlowedVersusPausedAmbiguous,
                D::WhatStillWorksUnstated,
                D::ProofStale,
            ],
            vec![power_signal_unavailable(), power_source_unstated()],
            vec![throttled_ambiguous(), throttled_none_named()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved power-state and throttled truth, so a collapsed generic warning or an unstated preserved-work list is visible in evidence rather than hidden",
            "evidence:m5-power-throttle-support-export:001",
            vec![
                D::GenericLowPowerWordingUsed,
                D::WhatStillWorksUnstated,
                D::AlternateStateLabelInvented,
                D::ProofStale,
            ],
            vec![power_collapsed_generic()],
            vec![throttled_no_preserved()],
        ),
        base_row(
            C::HelpAboutUi,
            "Help/About owner",
            "Help/About explains the same power-state and throttled-subsystem vocabulary a user sees in the shell, reusing the frozen matrix wording rather than inventing local prose",
            "evidence:m5-power-throttle-help-about:001",
            vec![
                D::SourceOfChangeUnstated,
                D::AlternateStateLabelInvented,
                D::ProofStale,
            ],
            vec![power_clean_single_cause()],
            vec![throttled_clean()],
        ),
    ]
}

fn governance_review() -> M5PowerThrottleGovernanceReview {
    M5PowerThrottleGovernanceReview {
        power_state_indicator_names_source_and_state: true,
        throttled_row_enumerates_affected_subsystems: true,
        throttled_row_names_preserved_work: true,
        no_indicator_collapses_into_generic_warning: true,
        no_surface_hides_slowed_work_after_user_visible: true,
        slowed_versus_paused_always_explicit: true,
        inspect_path_offered_or_degraded: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5PowerThrottleConsumerProjection {
    M5PowerThrottleConsumerProjection {
        shell_surfaces_consume_power_state: true,
        activity_surfaces_consume_throttled_rows: true,
        diagnostics_surfaces_consume_source_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PowerThrottleProofFreshness {
    M5PowerThrottleProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PowerThrottleReleasePosture {
    M5PowerThrottleReleasePosture {
        proof_packet_ref: M5_POWER_THROTTLE_CONTROLS_ARTIFACT_REF.to_owned(),
        efficiency_audit_ref: M5_POWER_THROTTLE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_POWER_THROTTLE_CONTROLS_SCHEMA_REF,
        M5_POWER_THROTTLE_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_POWER_STATE_INDICATOR_SCHEMA_REF,
        M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 power-state / throttled-subsystem controls packet.
pub fn seeded_m5_power_throttle_controls() -> M5PowerThrottleControlsPacket {
    M5PowerThrottleControlsPacket::new(M5PowerThrottleControlsPacketInput {
        packet_id: M5_POWER_THROTTLE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 power-state-indicator and throttled-subsystem-row controls with source-of-change, active state, affected subsystem, and inspect-path truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5PowerThrottleVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the activity-center row is held at Beta pending slowed-versus-paused parity
/// on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_power_throttle_controls_activity_center_beta_narrowed(
) -> M5PowerThrottleControlsPacket {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.packet_id =
        "m5-power-state-throttled-subsystem-controls:activity-center-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .expect("activity-center row present");
    row.qualification = M5EfficiencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending inspect-path parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_power_throttle_controls_diagnostics_preview_narrowed(
) -> M5PowerThrottleControlsPacket {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.packet_id =
        "m5-power-state-throttled-subsystem-controls:diagnostics-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EfficiencyConsumerSurface::DiagnosticsUi)
        .expect("diagnostics row present");
    row.qualification = M5EfficiencyQualificationClass::Preview;
    packet
}
