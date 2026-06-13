//! Deterministic effective-settings corpus for M5-added settings families.
//!
//! The corpus pins one fully-active baseline plus five drills: a locked row, a
//! constrained row, a missing-capability row, a Labs/Preview-dependent row, and
//! a stale-schema row. Each scenario carries the same per-row
//! source/effective/live review sheet and the same per-family
//! admin-distribution audit surface so the settings UI, CLI inspect,
//! Help/About, admin audit, policy explainers, and support exports replay one
//! record instead of cloning their own explanation vocabulary.

use super::model::{
    AdminDistributionAuditRow, AuditFreshnessState, EffectiveSettingsClaim,
    EffectiveValueReviewSheet, HighImpactClass, LifecycleDependencyKind, LifecycleDependencyMarker,
    M5EffectiveSettingRow, M5EffectiveSettingsCertification, M5EffectiveSettingsInput,
    M5SettingFamily, PolicyConstraintState, PolicyDistributionSource, PolicyLockState,
    ProjectionMode, RestartPosture, ReviewAction, ReviewExportPosture, RowTrust,
    ScopeExplicitWritePreview, SettingScope, ShadowReason, ShadowedCandidate, SurfaceClass,
    SurfaceTruthRow, ValidationState, WinningValue, WriteConstraintExplanation, WriteEffect,
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
            summary: "Every M5 settings family resolves to a fully-active value with an explicit winning scope, shadow chain, source/effective/live review sheet, and a checkpointed write preview for each high-impact row.",
        },
        ScenarioSpec {
            scenario_id: "policy_locked_drill",
            summary: "The companion remote-control setting is locked by a stale last-known-good policy bundle; the row explains the owner, last apply, review/expiry posture, and local-safe continuation instead of returning a generic write denial.",
        },
        ScenarioSpec {
            scenario_id: "policy_constrained_drill",
            summary: "The data/API egress allowlist is constrained by a mirrored admin policy; the write preview explains that the proposed value would land in a shadowed constrained posture rather than silently widening behavior.",
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
            summary: "The auto-install bundle setting is read from a stale schema and its governing bundle is expired, so both the resolved value and the distribution audit narrow to a local-safe posture instead of claiming fresh live state.",
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
        distribution_audit: distribution_audit(spec.scenario_id),
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
                constraint_state: PolicyConstraintState::Locked,
                policy_ref: Some("aureline://policy/companion-remote-control-lock".to_owned()),
                source_bundle_ref: Some(
                    "aureline://policy-bundle/companion-remote-control-cache".to_owned(),
                ),
                source_scope_ref: Some("aureline://policy-scope/tenant-managed-alpha".to_owned()),
                bundle_owner_ref: Some("aureline://owner/policy-review-board".to_owned()),
                distribution_source: Some(PolicyDistributionSource::LastKnownGoodCache),
                last_applied_at: Some("2026-06-11T23:40:00Z".to_owned()),
                review_due_at: Some("2026-06-13T09:00:00Z".to_owned()),
                expires_at: Some("2026-06-13T17:00:00Z".to_owned()),
                constraint_summary: Some(
                    "Remote control is pinned off until a fresh signed bundle is validated."
                        .to_owned(),
                ),
                local_safe_continuation: vec![
                    "Inspect the current companion posture locally.".to_owned(),
                    "Export a redacted support packet.".to_owned(),
                    "Keep local companion pairing disabled.".to_owned(),
                ],
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
            row.effective_value_review.available_actions = vec![
                ReviewAction::CopyRedactedSummary,
                ReviewAction::OpenEffective,
                ReviewAction::OpenPolicyBundle,
                ReviewAction::ContinueLocal,
                ReviewAction::RetryPolicySync,
            ];
            row.effective_value_review.unresolved_values =
                vec!["fresh live policy verification pending".to_owned()];
            if let Some(preview) = row.write_preview.as_mut() {
                preview.effective_after_write = WriteEffect::DeniedByLock;
                preview.explanation = Some(lock_explanation(
                    "aureline://policy-bundle/companion-remote-control-cache",
                    "aureline://policy-scope/tenant-managed-alpha",
                    "Admin policy pins companion remote control off until a fresh signed bundle is validated.",
                    "Retry policy sync or continue in the local-safe posture.",
                ));
            }
        }
        "policy_constrained_drill" => {
            let row = find_mut(&mut rows, "data_api.outbound_egress_allowlist");
            row.policy_lock = PolicyLockState {
                constraint_state: PolicyConstraintState::Constrained,
                policy_ref: Some("aureline://policy/data-api-egress-allowlist".to_owned()),
                source_bundle_ref: Some(
                    "aureline://policy-bundle/data-api-egress-mirror".to_owned(),
                ),
                source_scope_ref: Some("aureline://policy-scope/workspace-network".to_owned()),
                bundle_owner_ref: Some("aureline://owner/network-governance".to_owned()),
                distribution_source: Some(PolicyDistributionSource::MirrorPublication),
                last_applied_at: Some("2026-06-12T07:35:00Z".to_owned()),
                review_due_at: Some("2026-06-20T17:00:00Z".to_owned()),
                expires_at: None,
                constraint_summary: Some(
                    "Only hosts published by the mirrored network policy may be added.".to_owned(),
                ),
                local_safe_continuation: vec![
                    "Inspect the current allowlist locally.".to_owned(),
                    "Copy a redacted key/source summary.".to_owned(),
                    "Continue with the current approved egress set.".to_owned(),
                ],
            };
            row.winning_value.display = "allowlist: 2 approved mirror hosts".to_owned();
            if let Some(preview) = row.write_preview.as_mut() {
                preview.effective_after_write = WriteEffect::ShadowedByPolicy;
                preview.explanation = Some(lock_explanation(
                    "aureline://policy-bundle/data-api-egress-mirror",
                    "aureline://policy-scope/workspace-network",
                    "The proposed host would be written locally but remain shadowed by the mirrored policy allowlist.",
                    "Review the mirrored policy bundle or keep the approved hosts.",
                ));
            }
            row.effective_value_review.available_actions = vec![
                ReviewAction::CopyRedactedSummary,
                ReviewAction::OpenSource,
                ReviewAction::OpenEffective,
                ReviewAction::OpenPolicyBundle,
                ReviewAction::ContinueLocal,
            ];
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
            row.effective_value_review.unresolved_values =
                vec!["live profiler backend unavailable".to_owned()];
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
            row.effective_value_review.unresolved_values =
                vec!["live Labs kernel posture not yet active".to_owned()];
        }
        "stale_schema_drill" => {
            let row = find_mut(&mut rows, "bundle.auto_install_recommended");
            row.validation_state = ValidationState::SchemaStale;
            row.winning_value.display = "off (stale schema)".to_owned();
            row.effective_value_review.active_projection_mode = ProjectionMode::Source;
            row.effective_value_review.export_posture = ReviewExportPosture::MetadataOnly;
            row.effective_value_review.unresolved_values =
                vec!["effective/live migration blocked on schema refresh".to_owned()];
            row.effective_value_review.available_actions = vec![
                ReviewAction::CopyRedactedSummary,
                ReviewAction::OpenSource,
                ReviewAction::OpenPolicyBundle,
                ReviewAction::ContinueLocal,
            ];
        }
        other => panic!("unknown scenario id {other:?}"),
    }
    rows
}

fn distribution_audit(scenario_id: &str) -> Vec<AdminDistributionAuditRow> {
    let mut rows = base_distribution_audit();
    match scenario_id {
        "fully_active_baseline" => {}
        "policy_locked_drill" => {
            let row = find_audit_mut(&mut rows, M5SettingFamily::Companion);
            row.bundle_ref = "aureline://policy-bundle/companion-remote-control-cache".to_owned();
            row.distribution_source = PolicyDistributionSource::LastKnownGoodCache;
            row.last_applied_at = "2026-06-11T23:40:00Z".to_owned();
            row.last_validated_at = "2026-06-11T23:39:30Z".to_owned();
            row.freshness_state = AuditFreshnessState::Stale;
            row.local_safe_continuation = vec![
                "Inspect the current companion posture locally.".to_owned(),
                "Open the cached bundle review.".to_owned(),
                "Export a redacted support packet.".to_owned(),
            ];
        }
        "policy_constrained_drill" => {
            let row = find_audit_mut(&mut rows, M5SettingFamily::DataApi);
            row.bundle_ref = "aureline://policy-bundle/data-api-egress-mirror".to_owned();
            row.bundle_owner_ref = "aureline://owner/network-governance".to_owned();
            row.policy_scope_ref = "aureline://policy-scope/workspace-network".to_owned();
            row.distribution_source = PolicyDistributionSource::MirrorPublication;
            row.last_applied_at = "2026-06-12T07:35:00Z".to_owned();
            row.last_validated_at = "2026-06-12T07:34:30Z".to_owned();
            row.active_projection_mode = ProjectionMode::Effective;
        }
        "missing_capability_drill" => {}
        "labs_preview_dependent_drill" => {}
        "stale_schema_drill" => {
            let row = find_audit_mut(&mut rows, M5SettingFamily::Bundle);
            row.bundle_ref = "aureline://policy-bundle/bundle-autoinstall-mdm".to_owned();
            row.distribution_source = PolicyDistributionSource::MdmFleetDrop;
            row.last_applied_at = "2026-06-08T10:15:00Z".to_owned();
            row.last_validated_at = "2026-06-08T10:15:00Z".to_owned();
            row.freshness_state = AuditFreshnessState::Expired;
            row.local_safe_continuation = vec![
                "Review the authored workspace source.".to_owned(),
                "Copy metadata-only export fields.".to_owned(),
                "Keep auto-install disabled until schema refresh lands.".to_owned(),
            ];
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

fn find_audit_mut(
    rows: &mut [AdminDistributionAuditRow],
    family: M5SettingFamily,
) -> &mut AdminDistributionAuditRow {
    rows.iter_mut()
        .find(|row| row.family == family)
        .unwrap_or_else(|| panic!("missing audit row for family {family:?}"))
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
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "notebooks.execution.untrusted_kernel_autostart",
                ProjectionMode::Live,
                vec![SettingScope::Workspace, SettingScope::BuiltInDefault],
                ReviewExportPosture::KeysSourcesAndLiveRefs,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenSource,
                    ReviewAction::OpenEffective,
                    ReviewAction::OpenLive,
                ],
            ),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::DestructiveAutomation),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/notebooks-untrusted-kernel-off",
                "aureline://value/notebooks-untrusted-kernel-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartExtensions,
                None,
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
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "data_api.outbound_egress_allowlist",
                ProjectionMode::Effective,
                vec![SettingScope::UserGlobal, SettingScope::BuiltInDefault],
                ReviewExportPosture::KeysAndSourcesOnly,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenSource,
                    ReviewAction::OpenEffective,
                ],
            ),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::AiNetworkEgress),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/data-api-egress-allowlist-two-hosts",
                "aureline://value/data-api-egress-allowlist-three-hosts",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartProcess,
                None,
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
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "profiler.sampling_interval_ms",
                ProjectionMode::Live,
                vec![SettingScope::UserGlobal, SettingScope::BuiltInDefault],
                ReviewExportPosture::KeysSourcesAndLiveRefs,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenEffective,
                    ReviewAction::OpenLive,
                ],
            ),
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
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "bundle.auto_install_recommended",
                ProjectionMode::Source,
                vec![
                    SettingScope::Workspace,
                    SettingScope::UserGlobal,
                    SettingScope::BuiltInDefault,
                ],
                ReviewExportPosture::KeysAndSourcesOnly,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenSource,
                    ReviewAction::OpenEffective,
                ],
            ),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::ExtensionBehavior),
            write_preview: Some(preview(
                SettingScope::Workspace,
                "aureline://value/bundle-auto-install-off",
                "aureline://value/bundle-auto-install-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::ReloadWorkspace,
                None,
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
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "sync.device_participation",
                ProjectionMode::Effective,
                vec![SettingScope::MachineSpecific, SettingScope::UserGlobal],
                ReviewExportPosture::KeysAndSourcesOnly,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenEffective,
                    ReviewAction::ContinueLocal,
                ],
            ),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::RemoteExposure),
            write_preview: Some(preview(
                SettingScope::MachineSpecific,
                "aureline://value/sync-device-participation-on",
                "aureline://value/sync-device-participation-off",
                WriteEffect::BecomesWinningValue,
                RestartPosture::NoRestart,
                None,
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
            restart_posture: RestartPosture::RestartExtensions,
            validation_state: ValidationState::Valid,
            policy_lock: unlocked_policy(),
            effective_value_review: review_sheet(
                "companion.remote_control_enabled",
                ProjectionMode::Live,
                vec![SettingScope::UserGlobal, SettingScope::BuiltInDefault],
                ReviewExportPosture::KeysSourcesAndLiveRefs,
                vec![
                    ReviewAction::CopyRedactedSummary,
                    ReviewAction::OpenEffective,
                    ReviewAction::OpenLive,
                ],
            ),
            lifecycle_dependency: None,
            high_impact_class: Some(HighImpactClass::TrustBoundary),
            write_preview: Some(preview(
                SettingScope::UserGlobal,
                "aureline://value/companion-remote-control-off",
                "aureline://value/companion-remote-control-on",
                WriteEffect::BecomesWinningValue,
                RestartPosture::RestartExtensions,
                None,
            )),
        },
    ]
}

fn base_distribution_audit() -> Vec<AdminDistributionAuditRow> {
    vec![
        audit_row(
            "audit:notebooks",
            M5SettingFamily::Notebooks,
            "aureline://policy-bundle/notebooks-managed",
            "aureline://owner/notebook-runtime",
            "aureline://policy-scope/notebooks",
            PolicyDistributionSource::ManagedPull,
            "2026-06-12T07:30:00Z",
            "2026-06-12T07:29:30Z",
            ProjectionMode::Live,
        ),
        audit_row(
            "audit:data_api",
            M5SettingFamily::DataApi,
            "aureline://policy-bundle/data-api-mirror",
            "aureline://owner/network-governance",
            "aureline://policy-scope/data-api",
            PolicyDistributionSource::MirrorPublication,
            "2026-06-12T07:35:00Z",
            "2026-06-12T07:34:30Z",
            ProjectionMode::Effective,
        ),
        audit_row(
            "audit:profiler",
            M5SettingFamily::Profiler,
            "aureline://policy-bundle/profiler-file-import",
            "aureline://owner/perf-tooling",
            "aureline://policy-scope/profiler",
            PolicyDistributionSource::FileImport,
            "2026-06-12T06:45:00Z",
            "2026-06-12T06:44:30Z",
            ProjectionMode::Live,
        ),
        audit_row(
            "audit:bundle",
            M5SettingFamily::Bundle,
            "aureline://policy-bundle/bundle-mdm",
            "aureline://owner/extension-governance",
            "aureline://policy-scope/bundle",
            PolicyDistributionSource::MdmFleetDrop,
            "2026-06-12T05:15:00Z",
            "2026-06-12T05:15:00Z",
            ProjectionMode::Source,
        ),
        audit_row(
            "audit:sync",
            M5SettingFamily::Sync,
            "aureline://policy-bundle/sync-air-gap",
            "aureline://owner/identity-governance",
            "aureline://policy-scope/sync",
            PolicyDistributionSource::AirGapTransfer,
            "2026-06-12T04:50:00Z",
            "2026-06-12T04:49:30Z",
            ProjectionMode::Effective,
        ),
        audit_row(
            "audit:companion",
            M5SettingFamily::Companion,
            "aureline://policy-bundle/companion-managed",
            "aureline://owner/policy-review-board",
            "aureline://policy-scope/companion",
            PolicyDistributionSource::ManagedPull,
            "2026-06-12T07:10:00Z",
            "2026-06-12T07:09:30Z",
            ProjectionMode::Live,
        ),
    ]
}

fn unlocked_policy() -> PolicyLockState {
    PolicyLockState {
        constraint_state: PolicyConstraintState::Unlocked,
        policy_ref: None,
        source_bundle_ref: None,
        source_scope_ref: None,
        bundle_owner_ref: None,
        distribution_source: None,
        last_applied_at: None,
        review_due_at: None,
        expires_at: None,
        constraint_summary: None,
        local_safe_continuation: Vec::new(),
    }
}

fn shadow(scope: SettingScope, value_ref: &str, reason: ShadowReason) -> ShadowedCandidate {
    ShadowedCandidate {
        scope,
        value_ref: value_ref.to_owned(),
        reason,
    }
}

fn review_sheet(
    setting_id: &str,
    active_projection_mode: ProjectionMode,
    winning_layers: Vec<SettingScope>,
    export_posture: ReviewExportPosture,
    available_actions: Vec<ReviewAction>,
) -> EffectiveValueReviewSheet {
    EffectiveValueReviewSheet {
        selected_keys: vec![setting_id.to_owned()],
        active_projection_mode,
        available_projection_modes: vec![
            ProjectionMode::Source,
            ProjectionMode::Effective,
            ProjectionMode::Live,
        ],
        winning_layers,
        unresolved_values: Vec::new(),
        export_posture,
        available_actions,
    }
}

fn preview(
    target_scope: SettingScope,
    current_value_ref: &str,
    proposed_value_ref: &str,
    effective_after_write: WriteEffect,
    restart_posture_after: RestartPosture,
    explanation: Option<WriteConstraintExplanation>,
) -> ScopeExplicitWritePreview {
    ScopeExplicitWritePreview {
        target_scope,
        current_value_ref: current_value_ref.to_owned(),
        proposed_value_ref: proposed_value_ref.to_owned(),
        effective_after_write,
        restart_posture_after,
        requires_confirmation: true,
        rollback_checkpoint_ref: format!(
            "aureline://checkpoint/{}",
            current_value_ref
                .trim_start_matches("aureline://value/")
                .replace('/', "-")
        ),
        explanation,
    }
}

fn lock_explanation(
    bundle_ref: &str,
    scope_ref: &str,
    denied_reason: &str,
    repair_hint: &str,
) -> WriteConstraintExplanation {
    WriteConstraintExplanation {
        source_bundle_ref: Some(bundle_ref.to_owned()),
        source_scope_ref: Some(scope_ref.to_owned()),
        bundle_owner_ref: Some("aureline://owner/policy-review-board".to_owned()),
        review_ref: Some("aureline://review/policy-bundle-refresh".to_owned()),
        review_due_at: Some("2026-06-13T09:00:00Z".to_owned()),
        expires_at: Some("2026-06-13T17:00:00Z".to_owned()),
        denied_reason: denied_reason.to_owned(),
        local_safe_continuation: vec![
            "Inspect current effective value locally.".to_owned(),
            "Copy a redacted summary for support.".to_owned(),
            "Continue with the narrower posture.".to_owned(),
        ],
        repair_hint: repair_hint.to_owned(),
    }
}

fn audit_row(
    audit_id: &str,
    family: M5SettingFamily,
    bundle_ref: &str,
    bundle_owner_ref: &str,
    policy_scope_ref: &str,
    distribution_source: PolicyDistributionSource,
    last_applied_at: &str,
    last_validated_at: &str,
    active_projection_mode: ProjectionMode,
) -> AdminDistributionAuditRow {
    AdminDistributionAuditRow {
        audit_id: audit_id.to_owned(),
        family,
        bundle_ref: bundle_ref.to_owned(),
        bundle_owner_ref: bundle_owner_ref.to_owned(),
        policy_scope_ref: policy_scope_ref.to_owned(),
        distribution_source,
        last_applied_at: last_applied_at.to_owned(),
        last_validated_at: last_validated_at.to_owned(),
        active_projection_mode,
        freshness_state: AuditFreshnessState::Current,
        local_safe_continuation: vec![
            "Inspect the current effective posture locally.".to_owned(),
            "Open the governing bundle or audit row.".to_owned(),
            "Export redacted metadata for support.".to_owned(),
        ],
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
            shows_projection_mode: true,
            shows_lifecycle_dependency: true,
            shows_write_preview: true,
            shows_write_explanation: true,
            shows_distribution_audit: true,
            shows_last_applied: true,
            shows_local_safe_continuation: true,
        })
        .collect()
}
