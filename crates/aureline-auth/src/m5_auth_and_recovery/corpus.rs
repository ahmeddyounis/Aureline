//! Deterministic managed-auth-and-recovery corpus for M5 surfaces.
//!
//! The corpus pins one calm managed baseline that discloses every required
//! event kind across desktop, companion, and browser-adjacent surfaces with the
//! full credential-store account and the complete drill set, plus seven degraded
//! drills — step-up passkey-unavailable, sign-in browser-handoff failure,
//! re-auth offline identity, policy-forced sign-out, deprovision on active local
//! work, passkey-first account recovery, and a companion browser-handoff
//! failure. The desktop shell, CLI inspect, docs/help, and support surfaces
//! replay the same evidence so a change to the model, the fail-closed gate, or
//! the fixtures is caught against frozen records.

use super::model::{
    AuthCondition, AuthDrill, AuthEventKind, AuthEventRow, AuthRecoveryClaim, AuthSurface,
    BrowserHandoff, ConditionDisposition, ContinuityCeiling, CredentialClass, CredentialStorageRow,
    CredentialStoreClass, DrillCategory, DrillKind, FallbackPosture, HandoffMethod, HandoffReason,
    LocalCapabilityClass, LocalContinuityBlock, M5AuthAndRecovery, M5AuthAndRecoveryInput,
    ManagedCapabilityClass, PasskeyPosture, ProfileChannel, SurfaceClass, SurfaceTruthRow,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-12T08:00:00Z";

/// Every local capability the seeded events preserve.
const ALL_LOCAL_CAPABILITIES: [LocalCapabilityClass; 5] = [
    LocalCapabilityClass::LocalEditing,
    LocalCapabilityClass::LocalFiles,
    LocalCapabilityClass::LocalHistory,
    LocalCapabilityClass::LocalOnlyWorkflows,
    LocalCapabilityClass::ByokProviders,
];

/// One deterministic scenario in the M5 auth-and-recovery corpus.
#[derive(Debug, Clone)]
pub struct M5AuthAndRecoveryScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: AuthRecoveryClaim,
    /// Expected weakest event continuity across the record.
    pub expected_continuity_ceiling: ContinuityCeiling,
    record: M5AuthAndRecovery,
}

impl M5AuthAndRecoveryScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> M5AuthAndRecovery {
        self.record.clone()
    }
}

struct ScenarioSpec {
    scenario_id: &'static str,
    summary: &'static str,
}

/// Returns the deterministic corpus for the M5 auth-and-recovery contract.
pub fn m5_auth_and_recovery_corpus() -> Vec<M5AuthAndRecoveryScenario> {
    [
        ScenarioSpec {
            scenario_id: "calm_managed_baseline",
            summary: "Every M5 managed event — sign-in, step-up, re-auth, revocation, deprovision, and recovery — is disclosed across desktop, companion, and browser-adjacent surfaces with provider/owner labels, system-browser passkey-capable handoff, the full credential-store account, and the complete security/accessibility/recovery drill set.",
        },
        ScenarioSpec {
            scenario_id: "step_up_passkey_unavailable",
            summary: "The step-up challenge finds no passkey on this device; an explicit keyboard-complete security-key fallback is offered, the gated managed actions are paused, and local work stays usable.",
        },
        ScenarioSpec {
            scenario_id: "sign_in_browser_handoff_failure",
            summary: "The sign-in system-browser handoff fails to return; managed sync and hosted AI pause behind a device-code fallback while local editing, files, and local-only workflows continue.",
        },
        ScenarioSpec {
            scenario_id: "reauth_offline_identity",
            summary: "Re-auth cannot reach the identity authority while offline; managed capabilities pause and the row is labeled offline, but local durable work keeps running and recovers on reconnect.",
        },
        ScenarioSpec {
            scenario_id: "policy_forced_sign_out",
            summary: "Policy forces the active session to sign out; every managed capability pauses with an explicit re-sign-in path while unsaved local work, files, and history remain intact.",
        },
        ScenarioSpec {
            scenario_id: "deprovision_active_local_work",
            summary: "The seat is deprovisioned while local work is active; all managed capabilities are removed, yet unsaved local work, local files, local history, and local-only workflows are explicitly preserved.",
        },
        ScenarioSpec {
            scenario_id: "account_recovery_passkey_first",
            summary: "Account recovery runs passkey-first; where the passkey is unavailable it falls back to a system-browser security key — never embedded password-first or CAPTCHA-only collection on a stable profile.",
        },
        ScenarioSpec {
            scenario_id: "companion_browser_handoff_failure",
            summary: "A companion re-auth handoff fails; companion control and managed sync pause behind a keyboard-complete device-code fallback while the companion's local view stays usable.",
        },
    ]
    .into_iter()
    .map(build_scenario)
    .collect()
}

fn build_scenario(spec: ScenarioSpec) -> M5AuthAndRecoveryScenario {
    let record = M5AuthAndRecovery::build(M5AuthAndRecoveryInput {
        record_id: format!("m5_auth_and_recovery:{id}", id = spec.scenario_id),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: spec.summary.to_owned(),
        profile_channel: ProfileChannel::Stable,
        events: events(spec.scenario_id),
        credential_stores: credential_stores(),
        drills: drills(),
        surface_truth: surface_truth(),
    })
    .expect("scenario builds");

    M5AuthAndRecoveryScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id.replace('_', "-")),
        expected_claim_class: record.trust_qualification.claim_class,
        expected_continuity_ceiling: record.trust_qualification.effective_continuity_ceiling,
        record,
    }
}

fn events(scenario_id: &str) -> Vec<AuthEventRow> {
    let mut events = base_events();
    match scenario_id {
        "calm_managed_baseline" => {}
        "step_up_passkey_unavailable" => {
            let event = find_mut(&mut events, "auth.step_up");
            event.managed_healthy = false;
            event.passkey_posture = PasskeyPosture::UnavailableFallbackExplicit;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::PasskeyUnavailable,
                paused_capabilities: vec![
                    ManagedCapabilityClass::HostedAi,
                    ManagedCapabilityClass::MarketplacePublish,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserSecurityKey,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::RecoveryOffered,
                detail: "No passkey is enrolled; the step-up offers a keyboard-complete security-key path in the system browser. Hosted AI and marketplace publish stay paused until step-up completes; local work is untouched.".to_owned(),
            });
        }
        "sign_in_browser_handoff_failure" => {
            let event = find_mut(&mut events, "auth.managed_sign_in");
            event.managed_healthy = false;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::BrowserHandoffFailure,
                paused_capabilities: vec![
                    ManagedCapabilityClass::HostedAi,
                    ManagedCapabilityClass::ManagedSync,
                    ManagedCapabilityClass::OrgCollab,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserDeviceCode,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::ManagedPausedLocalPreserved,
                detail: "The system browser failed to return the sign-in callback; a keyboard-complete device-code path is offered. Managed sync, hosted AI, and org collaboration pause; local editing, files, and local-only workflows continue.".to_owned(),
            });
        }
        "reauth_offline_identity" => {
            let event = find_mut(&mut events, "auth.reauth");
            event.managed_healthy = false;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::OfflineIdentity,
                paused_capabilities: vec![
                    ManagedCapabilityClass::HostedAi,
                    ManagedCapabilityClass::ManagedSync,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserDeviceCode,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::ManagedPausedLocalPreserved,
                detail: "The identity authority is unreachable offline; the row is labeled offline and re-auth resumes on reconnect. Managed sync and hosted AI pause; local durable work keeps running.".to_owned(),
            });
        }
        "policy_forced_sign_out" => {
            let event = find_mut(&mut events, "auth.session_revocation");
            event.managed_healthy = false;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::PolicyForcedSignOut,
                paused_capabilities: vec![
                    ManagedCapabilityClass::HostedAi,
                    ManagedCapabilityClass::ManagedSync,
                    ManagedCapabilityClass::MarketplacePublish,
                    ManagedCapabilityClass::OrgCollab,
                    ManagedCapabilityClass::CompanionControl,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserPasskey,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::ManagedPausedLocalPreserved,
                detail: "An org policy revoked the active session; an explicit system-browser re-sign-in path is offered. Every managed capability pauses; unsaved local work, files, and history remain intact.".to_owned(),
            });
        }
        "deprovision_active_local_work" => {
            let event = find_mut(&mut events, "auth.deprovision");
            event.managed_healthy = false;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::DeprovisionOnActiveLocalWork,
                paused_capabilities: vec![
                    ManagedCapabilityClass::HostedAi,
                    ManagedCapabilityClass::ManagedSync,
                    ManagedCapabilityClass::MarketplacePublish,
                    ManagedCapabilityClass::CompanionControl,
                    ManagedCapabilityClass::PolicyDistribution,
                    ManagedCapabilityClass::OrgCollab,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserPasskey,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::ManagedExitLocalPreserved,
                detail: "The managed seat was deprovisioned while local work was active; all org capabilities are removed. Unsaved local work, local files, local history, and local-only workflows are explicitly preserved and were never at risk.".to_owned(),
            });
        }
        "account_recovery_passkey_first" => {
            let event = find_mut(&mut events, "auth.account_recovery");
            event.managed_healthy = false;
            event.passkey_posture = PasskeyPosture::UnavailableFallbackExplicit;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::PasskeyUnavailable,
                paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserSecurityKey,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::RecoveryOffered,
                detail: "Recovery runs passkey-first; with no passkey available it falls back to a system-browser security key. Embedded password-first and CAPTCHA-only collection are never required on this stable profile.".to_owned(),
            });
        }
        "companion_browser_handoff_failure" => {
            let event = find_mut(&mut events, "auth.reauth");
            event.managed_healthy = false;
            event.active_condition = Some(AuthCondition {
                drill: DrillKind::BrowserHandoffFailure,
                paused_capabilities: vec![
                    ManagedCapabilityClass::CompanionControl,
                    ManagedCapabilityClass::ManagedSync,
                ],
                local_capabilities_remaining: ALL_LOCAL_CAPABILITIES.to_vec(),
                fallback_posture: FallbackPosture::SystemBrowserDeviceCode,
                keyboard_complete_fallback: true,
                local_work_threatened: false,
                governed_policy_exception_ref: None,
                disposition: ConditionDisposition::ManagedPausedLocalPreserved,
                detail: "The companion's system-browser re-auth handoff failed; a keyboard-complete device-code path is offered. Companion control and managed sync pause; the companion's local view stays usable.".to_owned(),
            });
        }
        other => panic!("unknown scenario id {other:?}"),
    }
    events
}

fn find_mut<'a>(events: &'a mut [AuthEventRow], event_id: &str) -> &'a mut AuthEventRow {
    events
        .iter_mut()
        .find(|event| event.event_id == event_id)
        .unwrap_or_else(|| panic!("missing base event {event_id:?}"))
}

/// The calm base events shared by every scenario, one per managed event kind.
fn base_events() -> Vec<AuthEventRow> {
    vec![
        event(
            "auth.managed_sign_in",
            AuthEventKind::ManagedSignIn,
            AuthSurface::Desktop,
            HandoffMethod::SystemBrowserPasskey,
            HandoffReason::InteractiveSignIn,
            PasskeyPosture::Preferred,
            "Interactive managed sign-in runs in the system browser with a platform passkey; provider and owning org are disclosed before any handoff.",
        ),
        event(
            "auth.step_up",
            AuthEventKind::StepUp,
            AuthSurface::Desktop,
            HandoffMethod::SystemBrowserPasskey,
            HandoffReason::StepUpChallenge,
            PasskeyPosture::Preferred,
            "Higher-assurance managed actions trigger a passkey step-up in the system browser; only the requested managed action is gated.",
        ),
        event(
            "auth.reauth",
            AuthEventKind::ReAuth,
            AuthSurface::Companion,
            HandoffMethod::SystemBrowserPasskey,
            HandoffReason::PolicyReAuth,
            PasskeyPosture::Available,
            "Policy-driven re-authentication refreshes the companion session via the system browser without disturbing local work.",
        ),
        event(
            "auth.session_revocation",
            AuthEventKind::SessionRevocation,
            AuthSurface::BrowserAdjacent,
            HandoffMethod::SystemBrowserDeviceCode,
            HandoffReason::PolicyReAuth,
            PasskeyPosture::Available,
            "Session revocation surfaces an explicit re-sign-in path on the browser-adjacent return surface; local work is never signed out with the session.",
        ),
        event(
            "auth.deprovision",
            AuthEventKind::Deprovision,
            AuthSurface::Desktop,
            HandoffMethod::SystemBrowserPasskey,
            HandoffReason::PolicyReAuth,
            PasskeyPosture::Available,
            "Deprovision removes managed seat and entitlements while disclosing exactly which managed capabilities end and confirming local work stays on the device.",
        ),
        event(
            "auth.account_recovery",
            AuthEventKind::AccountRecovery,
            AuthSurface::BrowserAdjacent,
            HandoffMethod::SystemBrowserPasskey,
            HandoffReason::RecoveryFlow,
            PasskeyPosture::Preferred,
            "Account recovery runs passkey-first in the system browser on a stable profile, never requiring embedded password-first or CAPTCHA-only collection.",
        ),
    ]
}

fn event(
    event_id: &str,
    kind: AuthEventKind,
    surface: AuthSurface,
    method: HandoffMethod,
    reason: HandoffReason,
    passkey_posture: PasskeyPosture,
    detail: &str,
) -> AuthEventRow {
    let slug = kind.as_str();
    AuthEventRow {
        event_id: event_id.to_owned(),
        kind,
        surface,
        provider_label: "Acme Identity (system-browser SSO)".to_owned(),
        org_label: Some("Acme Corp".to_owned()),
        issuer_ref: format!("aureline://issuer/{slug}"),
        handoff: BrowserHandoff {
            method,
            reason,
            return_route_ref: format!("aureline://route/{slug}-return"),
            keyboard_complete_fallback: true,
        },
        passkey_posture,
        local_continuity: LocalContinuityBlock {
            local_editing_preserved: true,
            local_files_preserved: true,
            local_history_preserved: true,
            local_only_workflows_preserved: true,
            statement: "Local editing, files, history, and local-only workflows stay usable through this event regardless of managed identity state.".to_owned(),
        },
        managed_healthy: true,
        active_condition: None,
        detail: detail.to_owned(),
    }
}

fn credential_stores() -> Vec<CredentialStorageRow> {
    vec![
        credential_store(
            "cred.refresh_token",
            CredentialClass::RefreshToken,
            CredentialStoreClass::OsKeychain,
            "The refresh token lives in the OS keychain and is excluded from portable-state, sync, and support export.",
        ),
        credential_store(
            "cred.delegated_handle",
            CredentialClass::DelegatedHandle,
            CredentialStoreClass::EnterpriseStore,
            "Delegated capability handles live in the approved enterprise store and never enter portable-state, sync, or support export.",
        ),
        credential_store(
            "cred.session_broker",
            CredentialClass::SessionBroker,
            CredentialStoreClass::SessionBrokerMemory,
            "Session broker references are held in session-only memory, never persisted, and never exported.",
        ),
    ]
}

fn credential_store(
    store_id: &str,
    credential_class: CredentialClass,
    store_class: CredentialStoreClass,
    detail: &str,
) -> CredentialStorageRow {
    CredentialStorageRow {
        store_id: store_id.to_owned(),
        credential_class,
        store_class,
        excluded_from_portable_state: true,
        excluded_from_sync: true,
        excluded_from_support_export: true,
        detail: detail.to_owned(),
    }
}

fn drills() -> Vec<AuthDrill> {
    vec![
        drill(
            DrillKind::PasskeyUnavailable,
            vec![DrillCategory::Security, DrillCategory::Accessibility],
            vec![ManagedCapabilityClass::HostedAi],
            "No passkey is available; an explicit keyboard-complete fallback is offered instead of an embedded password prompt.",
            "Use a system-browser security key or device-code path to complete the managed challenge.",
        ),
        drill(
            DrillKind::BrowserHandoffFailure,
            vec![DrillCategory::Security, DrillCategory::Recovery],
            vec![
                ManagedCapabilityClass::ManagedSync,
                ManagedCapabilityClass::HostedAi,
            ],
            "The system-browser handoff failed to launch or return; the failure is labeled and managed capabilities pause.",
            "Retry the handoff or switch to keyboard-complete device-code entry; local work continues meanwhile.",
        ),
        drill(
            DrillKind::OfflineIdentity,
            vec![DrillCategory::Recovery],
            vec![
                ManagedCapabilityClass::ManagedSync,
                ManagedCapabilityClass::HostedAi,
            ],
            "Identity verification is offline; the row is labeled offline and managed capabilities pause.",
            "Reconnect to refresh managed identity; local durable work keeps running offline.",
        ),
        drill(
            DrillKind::PolicyForcedSignOut,
            vec![DrillCategory::Security],
            vec![
                ManagedCapabilityClass::ManagedSync,
                ManagedCapabilityClass::HostedAi,
                ManagedCapabilityClass::MarketplacePublish,
                ManagedCapabilityClass::OrgCollab,
            ],
            "Policy forced the active session to sign out; the paused managed capabilities are named explicitly.",
            "Re-sign-in via the system browser to restore managed capabilities; unsaved local work was never lost.",
        ),
        drill(
            DrillKind::DeprovisionOnActiveLocalWork,
            vec![DrillCategory::Recovery, DrillCategory::Accessibility],
            vec![
                ManagedCapabilityClass::HostedAi,
                ManagedCapabilityClass::ManagedSync,
                ManagedCapabilityClass::MarketplacePublish,
                ManagedCapabilityClass::CompanionControl,
                ManagedCapabilityClass::PolicyDistribution,
                ManagedCapabilityClass::OrgCollab,
            ],
            "The seat was deprovisioned while local work was active; unsaved work, files, and history are preserved and labeled.",
            "Keep working locally or export; sign into another account to restore managed features.",
        ),
    ]
}

fn drill(
    kind: DrillKind,
    categories: Vec<DrillCategory>,
    paused_capabilities: Vec<ManagedCapabilityClass>,
    expected_signal: &str,
    recovery_path: &str,
) -> AuthDrill {
    AuthDrill {
        kind,
        categories,
        paused_capabilities,
        local_preserved: true,
        local_labeled: true,
        keyboard_complete: true,
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
            shows_provider_disclosure: true,
            shows_paused_capabilities: true,
            shows_local_continuity: true,
            shows_fallback_posture: true,
            shows_drills: true,
        })
        .collect()
}
