//! Deterministic effective-settings corpus for M5-added settings families.
//!
//! The corpus pins one fully-active baseline that spans every M5 family and
//! four lifecycle drills — a policy-locked row, a missing-capability row, a
//! Labs/Preview-dependent row, and a stale-schema row. The settings UI, CLI
//! inspect, help, policy, and support surfaces replay the same evidence so a
//! change to the model, the fail-closed gate, or the fixtures is caught against
//! frozen records.

use super::model::{
    EffectiveSettingsClaim, HighImpactClass, LifecycleDependencyKind, LifecycleDependencyMarker,
    M5EffectiveSettingRow, M5EffectiveSettingsCertification, M5EffectiveSettingsInput,
    M5SettingFamily, PolicyLockState, RestartPosture, RowTrust, ScopeExplicitWritePreview,
    SettingScope, ShadowReason, ShadowedCandidate, SurfaceClass, SurfaceTruthRow, ValidationState,
    WinningValue, WriteEffect,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-12T08:00:00Z";

/// One deterministic scenario in the M5 effective-settings corpus.
#[derive(Debug, Clone)]
pub struct M5EffectiveSettingsScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: EffectiveSettingsClaim,
    /// Expected weakest row trust across the record.
    pub expected_trust_ceiling: RowTrust,
    record: M5EffectiveSettingsCertification,
}

impl M5EffectiveSettingsScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> M5EffectiveSettingsCertification {
        self.record.clone()
    }
}

struct ScenarioSpec {
    scenario_id: &'static str,
    summary: &'static str,
}

/// Returns the deterministic corpus for the M5 effective-settings contract.
pub fn m5_effective_settings_corpus() -> Vec<M5EffectiveSettingsScenario> {
    [
        ScenarioSpec {
            scenario_id: "fully_active_baseline",
            summary: "Every M5 settings family resolves to a fully-active value with an explicit winning scope, shadow chain, restart posture, and a checkpointed write preview for each high-impact row.",
        },
        ScenarioSpec {
            scenario_id: "policy_locked_drill",
            summary: "An admin policy locks the companion remote-control setting; the winning value comes from the policy ceiling and the write preview is denied by the lock rather than silently winning.",
        },
        ScenarioSpec {
            scenario_id: "missing_capability_drill",
            summary: "The profiler sampling setting is narrowed by a missing profiler-backend capability and the dependency stays a visible marker with a recovery hint.",
        },
        ScenarioSpec {
            scenario_id: "labs_preview_dependent_drill",
            summary: "Untrusted-kernel autostart is narrowed because it depends on a Labs/Preview feature lifecycle, surfaced as a visible lifecycle-dependency marker.",
        },
        ScenarioSpec {
            scenario_id: "stale_schema_drill",
            summary: "The auto-install bundle setting is read from a stale schema, so its winning value is withheld until migrated instead of being trusted as active.",
        },
    ]
    .into_iter()
    .map(build_scenario)
    .collect()
}

fn build_scenario(spec: ScenarioSpec) -> M5EffectiveSettingsScenario {
    let record = M5EffectiveSettingsCertification::build(M5EffectiveSettingsInput {
        record_id: format!("m5_effective_settings:{id}", id = spec.scenario_id),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: spec.summary.to_owned(),
        setting_rows: setting_rows(spec.scenario_id),
        surface_truth: surface_truth(),
    })
    .expect("scenario builds");

    M5EffectiveSettingsScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id.replace('_', "-")),
        expected_claim_class: record.trust_qualification.claim_class,
        expected_trust_ceiling: record.trust_qualification.effective_trust_ceiling,
        record,
    }
}

fn setting_rows(scenario_id: &str) -> Vec<M5EffectiveSettingRow> {
    let mut rows = base_rows();
    match scenario_id {
        "fully_active_baseline" => {}
        "policy_locked_drill" => {
            let row = find_mut(&mut rows, "companion.remote_control_enabled");
            row.policy_lock = PolicyLockState {
                locked: true,
                policy_ref: Some("aureline://policy/companion-remote-control-lock".to_owned()),
            };
            row.winning_value = WinningValue {
                scope: SettingScope::AdminPolicyNarrowing,
                value_ref: "aureline://value/companion-remote-control-policy-off".to_owned(),
                display: "off (policy-locked)".to_owned(),
            };
            row.shadow_chain = vec![
                shadow(
                    SettingScope::UserGlobal,
                    "aureline://value/companion-remote-control-user-on",
                    ShadowReason::PolicyNarrowed,
                ),
                shadow(
                    SettingScope::BuiltInDefault,
                    "aureline://value/companion-remote-control-default-off",
                    ShadowReason::LowerPrecedence,
                ),
            ];
            if let Some(preview) = row.write_preview.as_mut() {
                preview.effective_after_write = WriteEffect::DeniedByLock;
            }
        }
        "missing_capability_drill" => {
            let row = find_mut(&mut rows, "profiler.sampling_interval_ms");
            row.lifecycle_dependency = Some(LifecycleDependencyMarker {
                kind: LifecycleDependencyKind::MissingCapability,
                depends_on_ref: "aureline://capability/sampling-profiler-backend".to_owned(),
                narrows_behavior:
                    "Sampling is suspended until the profiler-backend capability is present."
                        .to_owned(),
                recovery_hint: "Install the profiler backend to resume sampling.".to_owned(),
                visible: true,
            });
        }
        "labs_preview_dependent_drill" => {
            let row = find_mut(&mut rows, "notebooks.execution.untrusted_kernel_autostart");
            row.lifecycle_dependency = Some(LifecycleDependencyMarker {
                kind: LifecycleDependencyKind::LabsOrPreviewDependent,
                depends_on_ref: "aureline://feature/labs-notebook-kernels".to_owned(),
                narrows_behavior:
                    "Untrusted-kernel autostart only applies while the Labs notebook-kernels feature is enabled."
                        .to_owned(),
                recovery_hint:
                    "Enable the Labs notebook-kernels feature or keep the stable default.".to_owned(),
                visible: true,
            });
        }
        "stale_schema_drill" => {
            let row = find_mut(&mut rows, "bundle.auto_install_recommended");
            row.validation_state = ValidationState::SchemaStale;
            row.winning_value.display = "off (stale schema)".to_owned();
        }
        other => panic!("unknown scenario id {other:?}"),
    }
    rows
}

fn find_mut<'a>(
    rows: &'a mut [M5EffectiveSettingRow],
    setting_id: &str,
) -> &'a mut M5EffectiveSettingRow {
    rows.iter_mut()
        .find(|row| row.setting_id == setting_id)
        .unwrap_or_else(|| panic!("missing base row {setting_id:?}"))
}

/// The fully-active base rows shared by every scenario, one per M5 family.
fn base_rows() -> Vec<M5EffectiveSettingRow> {
    vec![
        M5EffectiveSettingRow {
            setting_id: "notebooks.execution.untrusted_kernel_autostart".to_owned(),
            family: M5SettingFamily::Notebooks,
            title: "Auto-start untrusted notebook kernels".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::Workspace,
                value_ref: "aureline://value/notebooks-untrusted-kernel-off".to_owned(),
                display: "off".to_owned(),
            },
            shadow_chain: vec![shadow(
                SettingScope::BuiltInDefault,
                "aureline://value/notebooks-untrusted-kernel-default-off",
                ShadowReason::LowerPrecedence,
            )],
            restart_posture: RestartPosture::RestartExtensions,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::DestructiveAutomation),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/notebooks-untrusted-kernel-off",
                "aureline://value/notebooks-untrusted-kernel-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartExtensions,
            )),
        },
        M5EffectiveSettingRow {
            setting_id: "data_api.outbound_egress_allowlist".to_owned(),
            family: M5SettingFamily::DataApi,
            title: "Outbound data/API egress allowlist".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::UserGlobal,
                value_ref: "aureline://value/data-api-egress-allowlist-two-hosts".to_owned(),
                display: "allowlist: 2 hosts".to_owned(),
            },
            shadow_chain: vec![shadow(
                SettingScope::BuiltInDefault,
                "aureline://value/data-api-egress-allowlist-empty",
                ShadowReason::LowerPrecedence,
            )],
            restart_posture: RestartPosture::RestartProcess,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::AiNetworkEgress),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/data-api-egress-allowlist-two-hosts",
                "aureline://value/data-api-egress-allowlist-three-hosts",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartProcess,
            )),
        },
        M5EffectiveSettingRow {
            setting_id: "profiler.sampling_interval_ms".to_owned(),
            family: M5SettingFamily::Profiler,
            title: "Profiler sampling interval".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::UserGlobal,
                value_ref: "aureline://value/profiler-sampling-10ms".to_owned(),
                display: "10 ms".to_owned(),
            },
            shadow_chain: vec![shadow(
                SettingScope::BuiltInDefault,
                "aureline://value/profiler-sampling-default-25ms",
                ShadowReason::LowerPrecedence,
            )],
            restart_posture: RestartPosture::ReloadView,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: None,
            write_preview: None,
        },
        M5EffectiveSettingRow {
            setting_id: "bundle.auto_install_recommended".to_owned(),
            family: M5SettingFamily::Bundle,
            title: "Auto-install recommended bundles".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::Workspace,
                value_ref: "aureline://value/bundle-auto-install-off".to_owned(),
                display: "off".to_owned(),
            },
            shadow_chain: vec![
                shadow(
                    SettingScope::UserGlobal,
                    "aureline://value/bundle-auto-install-user-on",
                    ShadowReason::LowerPrecedence,
                ),
                shadow(
                    SettingScope::BuiltInDefault,
                    "aureline://value/bundle-auto-install-default-off",
                    ShadowReason::LowerPrecedence,
                ),
            ],
            restart_posture: RestartPosture::ReloadWorkspace,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::ExtensionBehavior),
            write_preview: Some(preview(
                SettingScope::Workspace,
                "aureline://value/bundle-auto-install-off",
                "aureline://value/bundle-auto-install-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::ReloadWorkspace,
            )),
        },
        M5EffectiveSettingRow {
            setting_id: "sync.device_participation".to_owned(),
            family: M5SettingFamily::Sync,
            title: "Participate in settings sync on this device".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::MachineSpecific,
                value_ref: "aureline://value/sync-device-participation-on".to_owned(),
                display: "enabled".to_owned(),
            },
            shadow_chain: vec![shadow(
                SettingScope::UserGlobal,
                "aureline://value/sync-device-participation-user-default",
                ShadowReason::LowerPrecedence,
            )],
            restart_posture: RestartPosture::NoRestart,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::RemoteExposure),
            write_preview: Some(preview(
                SettingScope::MachineSpecific,
                "aureline://value/sync-device-participation-on",
                "aureline://value/sync-device-participation-off",
                WriteEffect::BecomesWinningValue,
                RestartPosture::NoRestart,
            )),
        },
        M5EffectiveSettingRow {
            setting_id: "companion.remote_control_enabled".to_owned(),
            family: M5SettingFamily::Companion,
            title: "Allow companion remote control".to_owned(),
            winning_value: WinningValue {
                scope: SettingScope::UserGlobal,
                value_ref: "aureline://value/companion-remote-control-off".to_owned(),
                display: "off".to_owned(),
            },
            shadow_chain: vec![shadow(
                SettingScope::BuiltInDefault,
                "aureline://value/companion-remote-control-default-off",
                ShadowReason::LowerPrecedence,
            )],
            restart_posture: RestartPosture::RestartProcess,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked(),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::TrustBoundary),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/companion-remote-control-off",
                "aureline://value/companion-remote-control-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartProcess,
            )),
        },
    ]
}

fn shadow(scope: SettingScope, value_ref: &str, reason: ShadowReason) -> ShadowedCandidate {
    ShadowedCandidate {
        scope,
        value_ref: value_ref.to_owned(),
        reason,
    }
}

fn preview(
    target_scope: SettingScope,
    current_value_ref: &str,
    proposed_value_ref: &str,
    effective_after_write: WriteEffect,
    restart_posture_after: RestartPosture,
) -> ScopeExplicitWritePreview {
    ScopeExplicitWritePreview {
        target_scope,
        current_value_ref: current_value_ref.to_owned(),
        proposed_value_ref: proposed_value_ref.to_owned(),
        effective_after_write,
        restart_posture_after,
        requires_confirmation: true,
        rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-write".to_owned(),
    }
}

const fn unlocked() -> PolicyLockState {
    PolicyLockState {
        locked: false,
        policy_ref: None,
    }
}

fn surface_truth() -> Vec<SurfaceTruthRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface_class| SurfaceTruthRow {
            surface_class,
            consumes_shared_record: true,
            shows_winning_scope: true,
            shows_shadow_chain: true,
            shows_restart_posture: true,
            shows_lifecycle_dependency: true,
            shows_write_preview: true,
        })
        .collect()
}
