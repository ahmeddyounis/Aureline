//! Deterministic sync-and-device review corpus for M5 feature families.
//!
//! The corpus pins one fully-synced baseline that spans every M5 family with
//! every device action and degraded-state drill, plus seven field-aware conflict
//! drills — same-key divergent, policy-locked, missing-capability, machine-only,
//! delete-versus-modify, stale-remote, and a trust-widening blocked review. The
//! settings UI, CLI inspect, docs/help, and support surfaces replay the same
//! evidence so a change to the model, the fail-closed gate, or the fixtures is
//! caught against frozen records.

use super::model::{
    BundleSyncTrust, ConflictClass, ConflictDisposition, DeviceAction, DeviceActionRecord,
    DeviceClass, DeviceParticipationState, DrillKind, FieldConflict, M5SyncAndDeviceReview,
    M5SyncAndDeviceReviewInput, RedactionMode, ScopeCapabilityDependency, ScopeRevisionSets,
    SurfaceClass, SurfaceTruthRow, SyncDrill, SyncReviewClaim, SyncScopeBundle, SyncScopeFamily,
    SyncTransportState, TrustWideningClass,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-12T08:00:00Z";

/// One deterministic scenario in the M5 sync-and-device review corpus.
#[derive(Debug, Clone)]
pub struct M5SyncAndDeviceReviewScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: SyncReviewClaim,
    /// Expected weakest bundle trust across the record.
    pub expected_trust_ceiling: BundleSyncTrust,
    record: M5SyncAndDeviceReview,
}

impl M5SyncAndDeviceReviewScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> M5SyncAndDeviceReview {
        self.record.clone()
    }
}

struct ScenarioSpec {
    scenario_id: &'static str,
    summary: &'static str,
}

/// Returns the deterministic corpus for the M5 sync-and-device review contract.
pub fn m5_sync_and_device_review_corpus() -> Vec<M5SyncAndDeviceReviewScenario> {
    [
        ScenarioSpec {
            scenario_id: "fully_synced_baseline",
            summary: "Every M5 feature family resolves to a clean-synced scope bundle with explicit revision sets, the full pause/resume/revoke/forget/rotate device-action catalog, and the offline/stale-remote/blocked-apply/E2EE-unavailable/local-only drill set.",
        },
        ScenarioSpec {
            scenario_id: "same_key_divergent_drill",
            summary: "Local and remote both set the notebook trust key; the divergence is held field-aware and local stays authoritative pending review rather than last-writer-wins.",
        },
        ScenarioSpec {
            scenario_id: "policy_locked_drill",
            summary: "An admin policy locks the data/API egress key locally; the remote value is blocked instead of silently winning.",
        },
        ScenarioSpec {
            scenario_id: "missing_capability_drill",
            summary: "The profiler bundle's remote value needs a capability that is absent on this device, so the apply is held and local stays authoritative.",
        },
        ScenarioSpec {
            scenario_id: "machine_only_drill",
            summary: "The companion bundle carries a machine-only field that is never reconciled across devices and stays excluded from sync.",
        },
        ScenarioSpec {
            scenario_id: "delete_versus_modify_drill",
            summary: "The extension-bundle selection was deleted remotely while modified locally; the conflict is held for review rather than auto-resolved.",
        },
        ScenarioSpec {
            scenario_id: "stale_remote_drill",
            summary: "The notebook bundle's remote revision is older than the local common ancestor, so the stale payload is rejected and local stays authoritative.",
        },
        ScenarioSpec {
            scenario_id: "trust_widening_blocked_drill",
            summary: "The extension bundle's remote payload would widen trust, extension permissions, AI egress, and a managed entitlement; every such field is blocked and requires explicit review.",
        },
    ]
    .into_iter()
    .map(build_scenario)
    .collect()
}

fn build_scenario(spec: ScenarioSpec) -> M5SyncAndDeviceReviewScenario {
    let record = M5SyncAndDeviceReview::build(M5SyncAndDeviceReviewInput {
        record_id: format!("m5_sync_and_device_review:{id}", id = spec.scenario_id),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: spec.summary.to_owned(),
        scope_bundles: scope_bundles(spec.scenario_id),
        device_actions: device_actions(),
        drills: drills(),
        surface_truth: surface_truth(),
    })
    .expect("scenario builds");

    M5SyncAndDeviceReviewScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id.replace('_', "-")),
        expected_claim_class: record.trust_qualification.claim_class,
        expected_trust_ceiling: record.trust_qualification.effective_trust_ceiling,
        record,
    }
}

fn scope_bundles(scenario_id: &str) -> Vec<SyncScopeBundle> {
    let mut bundles = base_bundles();
    match scenario_id {
        "fully_synced_baseline" => {}
        "same_key_divergent_drill" => {
            let bundle = find_mut(&mut bundles, "sync.notebooks");
            bundle.remote_synced = false;
            bundle.conflicts.push(FieldConflict {
                field_path: "execution.trusted_kernels".to_owned(),
                class: ConflictClass::SameKeyDivergent,
                disposition: ConflictDisposition::AwaitingFieldReview,
                local_value_ref: "aureline://value/notebooks-trusted-kernels-local".to_owned(),
                remote_value_ref: Some(
                    "aureline://value/notebooks-trusted-kernels-remote".to_owned(),
                ),
                widens_trust: None,
                requires_explicit_review: true,
                detail: "Local and remote disagree on the trusted-kernel list; local is kept until reviewed.".to_owned(),
            });
        }
        "policy_locked_drill" => {
            let bundle = find_mut(&mut bundles, "sync.data_api");
            bundle.remote_synced = false;
            bundle.conflicts.push(FieldConflict {
                field_path: "egress.allowlist".to_owned(),
                class: ConflictClass::PolicyLocked,
                disposition: ConflictDisposition::RemoteApplyBlocked,
                local_value_ref: "aureline://value/data-api-egress-local".to_owned(),
                remote_value_ref: Some("aureline://value/data-api-egress-remote".to_owned()),
                widens_trust: None,
                requires_explicit_review: true,
                detail: "An admin policy locks the egress allowlist; the remote value cannot win."
                    .to_owned(),
            });
        }
        "missing_capability_drill" => {
            let bundle = find_mut(&mut bundles, "sync.profiler");
            bundle.remote_synced = false;
            bundle
                .capability_dependencies
                .push(ScopeCapabilityDependency {
                    capability_ref: "aureline://capability/sampling-profiler-backend".to_owned(),
                    present_locally: false,
                    narrows_apply: true,
                });
            bundle.conflicts.push(FieldConflict {
                field_path: "sampling.mode".to_owned(),
                class: ConflictClass::MissingCapability,
                disposition: ConflictDisposition::AwaitingFieldReview,
                local_value_ref: "aureline://value/profiler-sampling-local".to_owned(),
                remote_value_ref: Some("aureline://value/profiler-sampling-remote".to_owned()),
                widens_trust: None,
                requires_explicit_review: true,
                detail:
                    "The remote sampling mode needs the profiler backend, which is absent here."
                        .to_owned(),
            });
        }
        "machine_only_drill" => {
            let bundle = find_mut(&mut bundles, "sync.companion");
            bundle.conflicts.push(FieldConflict {
                field_path: "pairing.local_token".to_owned(),
                class: ConflictClass::MachineOnly,
                disposition: ConflictDisposition::LocalAuthoritativeKept,
                local_value_ref: "aureline://value/companion-pairing-token-local".to_owned(),
                remote_value_ref: None,
                widens_trust: None,
                requires_explicit_review: false,
                detail: "The companion pairing token is machine-only and never reconciles across devices.".to_owned(),
            });
        }
        "delete_versus_modify_drill" => {
            let bundle = find_mut(&mut bundles, "sync.extension_bundles");
            bundle.remote_synced = false;
            bundle.conflicts.push(FieldConflict {
                field_path: "selections.recommended_set".to_owned(),
                class: ConflictClass::DeleteVersusModify,
                disposition: ConflictDisposition::AwaitingFieldReview,
                local_value_ref: "aureline://value/extension-selection-local-modified".to_owned(),
                remote_value_ref: None,
                widens_trust: None,
                requires_explicit_review: true,
                detail: "The recommended set was deleted remotely but modified locally; held for review.".to_owned(),
            });
        }
        "stale_remote_drill" => {
            let bundle = find_mut(&mut bundles, "sync.notebooks");
            bundle.remote_synced = false;
            bundle.revisions.remote_revision_ref =
                Some("aureline://revision/notebooks-remote-stale".to_owned());
            bundle.conflicts.push(FieldConflict {
                field_path: "execution.kernel_timeout_ms".to_owned(),
                class: ConflictClass::StaleRemote,
                disposition: ConflictDisposition::LocalAuthoritativeKept,
                local_value_ref: "aureline://value/notebooks-timeout-local".to_owned(),
                remote_value_ref: Some("aureline://value/notebooks-timeout-remote-stale".to_owned()),
                widens_trust: None,
                requires_explicit_review: false,
                detail: "The remote revision predates the local common ancestor; the stale value is rejected.".to_owned(),
            });
        }
        "trust_widening_blocked_drill" => {
            let bundle = find_mut(&mut bundles, "sync.extension_bundles");
            bundle.remote_synced = false;
            bundle.conflicts.extend([
                trust_widening_conflict(
                    "trust.run_untrusted_content",
                    TrustWideningClass::TrustElevation,
                    "extension-trust-elevation",
                ),
                trust_widening_conflict(
                    "permissions.granted_scopes",
                    TrustWideningClass::ExtensionPermission,
                    "extension-permission-grant",
                ),
                trust_widening_conflict(
                    "ai.egress_hosts",
                    TrustWideningClass::AiEgress,
                    "extension-ai-egress",
                ),
                trust_widening_conflict(
                    "managed.entitlement_tier",
                    TrustWideningClass::ManagedEntitlement,
                    "extension-managed-entitlement",
                ),
            ]);
        }
        other => panic!("unknown scenario id {other:?}"),
    }
    bundles
}

fn trust_widening_conflict(
    field_path: &str,
    widens_trust: TrustWideningClass,
    value_slug: &str,
) -> FieldConflict {
    FieldConflict {
        field_path: field_path.to_owned(),
        class: ConflictClass::SameKeyDivergent,
        disposition: ConflictDisposition::RemoteApplyBlocked,
        local_value_ref: format!("aureline://value/{value_slug}-local"),
        remote_value_ref: Some(format!("aureline://value/{value_slug}-remote")),
        widens_trust: Some(widens_trust),
        requires_explicit_review: true,
        detail: format!(
            "The remote value would widen {}; it is blocked and requires explicit review.",
            widens_trust.as_str().replace('_', " ")
        ),
    }
}

fn find_mut<'a>(bundles: &'a mut [SyncScopeBundle], bundle_id: &str) -> &'a mut SyncScopeBundle {
    bundles
        .iter_mut()
        .find(|bundle| bundle.bundle_id == bundle_id)
        .unwrap_or_else(|| panic!("missing base bundle {bundle_id:?}"))
}

/// The clean-synced base bundles shared by every scenario, one per M5 family.
fn base_bundles() -> Vec<SyncScopeBundle> {
    vec![
        bundle(
            "sync.notebooks",
            SyncScopeFamily::Notebooks,
            "Notebook execution and trust settings",
            RedactionMode::None,
            "notebooks",
            vec![],
        ),
        bundle(
            "sync.data_api",
            SyncScopeFamily::DataApi,
            "Data/API connection and egress settings",
            RedactionMode::RedactSecrets,
            "data-api",
            vec![ScopeCapabilityDependency {
                capability_ref: "aureline://capability/data-api-connector".to_owned(),
                present_locally: true,
                narrows_apply: false,
            }],
        ),
        bundle(
            "sync.profiler",
            SyncScopeFamily::Profiler,
            "Profiler sampling and retention settings",
            RedactionMode::None,
            "profiler",
            vec![],
        ),
        bundle(
            "sync.extension_bundles",
            SyncScopeFamily::ExtensionBundles,
            "Extension and bundle selection settings",
            RedactionMode::None,
            "extension-bundles",
            vec![],
        ),
        bundle(
            "sync.companion",
            SyncScopeFamily::Companion,
            "Companion device control settings",
            RedactionMode::MachineLocalExcluded,
            "companion",
            vec![],
        ),
    ]
}

fn bundle(
    bundle_id: &str,
    family: SyncScopeFamily,
    title: &str,
    redaction_mode: RedactionMode,
    slug: &str,
    capability_dependencies: Vec<ScopeCapabilityDependency>,
) -> SyncScopeBundle {
    SyncScopeBundle {
        bundle_id: bundle_id.to_owned(),
        family,
        title: title.to_owned(),
        bundle_schema_version: 1,
        redaction_mode,
        source_device_ref: "aureline://device/primary-desktop".to_owned(),
        source_profile_ref: "aureline://profile/default".to_owned(),
        revisions: ScopeRevisionSets {
            local_revision_ref: format!("aureline://revision/{slug}-local"),
            remote_revision_ref: Some(format!("aureline://revision/{slug}-remote")),
            last_common_revision_ref: Some(format!("aureline://revision/{slug}-common")),
        },
        capability_dependencies,
        local_authoritative: true,
        remote_synced: true,
        conflicts: vec![],
    }
}

fn device_actions() -> Vec<DeviceActionRecord> {
    vec![
        device_action(
            DeviceAction::Pause,
            DeviceClass::Laptop,
            DeviceParticipationState::Paused,
            "Paused sync on the laptop; its local durable state is untouched.",
        ),
        device_action(
            DeviceAction::Resume,
            DeviceClass::Laptop,
            DeviceParticipationState::Active,
            "Resumed sync on the laptop; no local state was reset on resume.",
        ),
        device_action(
            DeviceAction::Revoke,
            DeviceClass::Companion,
            DeviceParticipationState::Revoked,
            "Revoked the companion's sync credentials; its local state stays on the device.",
        ),
        device_action(
            DeviceAction::Forget,
            DeviceClass::Headless,
            DeviceParticipationState::Forgotten,
            "Forgot the headless device from the registry; nothing local was deleted.",
        ),
        device_action(
            DeviceAction::Rotate,
            DeviceClass::Desktop,
            DeviceParticipationState::Rotating,
            "Rotated the desktop's sync keys in place; local durable state is intact.",
        ),
    ]
}

fn device_action(
    action: DeviceAction,
    device_class: DeviceClass,
    participation_after: DeviceParticipationState,
    detail: &str,
) -> DeviceActionRecord {
    DeviceActionRecord {
        device_ref: format!("aureline://device/{}", device_class.as_str()),
        device_class,
        action,
        audit_ref: format!("aureline://audit/device-{}", action.as_str()),
        actor_ref: "aureline://actor/account-owner".to_owned(),
        participation_after,
        local_state_intact: true,
        detail: detail.to_owned(),
    }
}

fn drills() -> Vec<SyncDrill> {
    vec![
        drill(
            DrillKind::Offline,
            SyncTransportState::Offline,
            "The device is offline; sync is suspended and the local value is labeled offline.",
            "Sync resumes automatically when the device reconnects.",
        ),
        drill(
            DrillKind::StaleRemote,
            SyncTransportState::StaleRemote,
            "A stale remote payload is rejected and the row is labeled stale-remote.",
            "Pull a fresh remote revision to clear the stale label.",
        ),
        drill(
            DrillKind::BlockedSyncApply,
            SyncTransportState::PolicyBlocked,
            "Applying a remote change is blocked by policy and labeled apply-blocked.",
            "Review the policy lock or request an entitlement to unblock the apply.",
        ),
        drill(
            DrillKind::E2eeUnavailable,
            SyncTransportState::E2eeUnavailable,
            "End-to-end encryption is unavailable; secret-bearing fields are held local-only.",
            "Restore the encryption key to resume secret-bearing sync.",
        ),
        drill(
            DrillKind::LocalOnlyFallback,
            SyncTransportState::LocalOnly,
            "Sync is unavailable; the device runs local-only and the banner labels the state.",
            "Re-authenticate or reconnect to leave local-only fallback.",
        ),
    ]
}

fn drill(
    kind: DrillKind,
    transport_state: SyncTransportState,
    expected_signal: &str,
    recovery_path: &str,
) -> SyncDrill {
    SyncDrill {
        kind,
        transport_state,
        local_authoritative: true,
        local_state_labeled: true,
        expected_signal: expected_signal.to_owned(),
        recovery_path: recovery_path.to_owned(),
    }
}

fn surface_truth() -> Vec<SurfaceTruthRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface_class| SurfaceTruthRow {
            surface_class,
            consumes_shared_record: true,
            shows_scope_bundles: true,
            shows_field_conflicts: true,
            shows_device_actions: true,
            shows_local_only_fallback: true,
        })
        .collect()
}
