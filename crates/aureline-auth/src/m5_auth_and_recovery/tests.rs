//! Unit tests for the M5 auth-and-recovery gate and its fail-closed guards.

use super::corpus::m5_auth_and_recovery_corpus;
use super::model::{
    AuthEventKind, AuthRecoveryClaim, AuthSurface, BuildError, ContinuityCeiling, CredentialClass,
    DrillCategory, DrillKind, FallbackPosture, M5AuthAndRecovery, M5AuthAndRecoveryInput,
    ManagedCapabilityClass, PasskeyPosture,
};

fn local_caps() -> Vec<super::model::LocalCapabilityClass> {
    use super::model::LocalCapabilityClass::*;
    vec![
        LocalEditing,
        LocalFiles,
        LocalHistory,
        LocalOnlyWorkflows,
        ByokProviders,
    ]
}

fn baseline_input() -> M5AuthAndRecoveryInput {
    let scenario = m5_auth_and_recovery_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "calm_managed_baseline")
        .expect("baseline scenario exists");
    let record = scenario.record();
    M5AuthAndRecoveryInput {
        record_id: record.record_id,
        as_of: record.as_of,
        summary: record.summary,
        profile_channel: record.profile_channel,
        events: record.events,
        credential_stores: record.credential_stores,
        drills: record.drills,
        surface_truth: record.surface_truth,
    }
}

#[test]
fn baseline_qualifies_local_first_managed_safe() {
    let record = M5AuthAndRecovery::build(baseline_input()).expect("builds");
    assert_eq!(
        record.trust_qualification.claim_class,
        AuthRecoveryClaim::LocalFirstManagedSafe
    );
    assert!(record.trust_qualification.qualifies_local_first_safe);
    assert_eq!(
        record.trust_qualification.effective_continuity_ceiling,
        ContinuityCeiling::LocalFirstFull
    );
    assert!(record.trust_qualification.narrowing_reasons.is_empty());
}

#[test]
fn every_required_event_kind_must_be_present() {
    let mut input = baseline_input();
    input
        .events
        .retain(|event| event.kind != AuthEventKind::Deprovision);
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingEventKind {
            kind: AuthEventKind::Deprovision
        }
    );
}

#[test]
fn every_required_surface_must_be_represented() {
    let mut input = baseline_input();
    // Drop both browser-adjacent events so the surface vanishes entirely.
    input
        .events
        .retain(|event| event.surface != AuthSurface::BrowserAdjacent);
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    // Removing those rows also drops their event kinds; the kind check runs
    // first, so assert we get a structural rejection either way.
    assert!(matches!(
        err,
        BuildError::MissingEventKind { .. } | BuildError::MissingSurface { .. }
    ));
}

#[test]
fn event_must_disclose_provider() {
    let mut input = baseline_input();
    let event = input.events.iter_mut().next().expect("an event");
    event.provider_label = "   ".to_owned();
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::ProviderUndisclosed { event_id });
}

#[test]
fn ungoverned_local_work_threat_is_rejected() {
    let mut input = baseline_input();
    let event = input
        .events
        .iter_mut()
        .find(|e| e.kind == AuthEventKind::Deprovision)
        .expect("deprovision event");
    event.managed_healthy = false;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::DeprovisionOnActiveLocalWork,
        paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
        local_capabilities_remaining: vec![],
        fallback_posture: FallbackPosture::SystemBrowserPasskey,
        keyboard_complete_fallback: true,
        local_work_threatened: true,
        governed_policy_exception_ref: None,
        disposition: super::model::ConditionDisposition::ManagedExitLocalPreserved,
        detail: "Would discard unsaved local work.".to_owned(),
    });
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::LocalWorkThreatened { event_id });
}

#[test]
fn governed_local_work_impact_is_allowed() {
    let mut input = baseline_input();
    let event = input
        .events
        .iter_mut()
        .find(|e| e.kind == AuthEventKind::Deprovision)
        .expect("deprovision event");
    event.managed_healthy = false;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::DeprovisionOnActiveLocalWork,
        paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
        local_capabilities_remaining: local_caps(),
        fallback_posture: FallbackPosture::SystemBrowserPasskey,
        keyboard_complete_fallback: true,
        local_work_threatened: true,
        governed_policy_exception_ref: Some("aureline://policy/wipe-on-deprovision".to_owned()),
        disposition: super::model::ConditionDisposition::ManagedExitLocalPreserved,
        detail: "A governed policy row authorizes a managed-data wipe on deprovision.".to_owned(),
    });
    // A named governed policy row is the only way a local-work impact passes the
    // gate; without it the same row is a build error (see the test above).
    let record = M5AuthAndRecovery::build(input).expect("builds with governed exception");
    assert_eq!(
        record.trust_qualification.effective_continuity_ceiling,
        ContinuityCeiling::ManagedNarrowedLocalIntact
    );
}

#[test]
fn implicit_passkey_fallback_is_rejected() {
    let mut input = baseline_input();
    let event = input.events.iter_mut().next().expect("an event");
    event.passkey_posture = PasskeyPosture::UnavailableFallbackImplicit;
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::FallbackPostureImplicit { event_id });
}

#[test]
fn embedded_recovery_is_rejected_on_stable_profile() {
    let mut input = baseline_input();
    let event = input
        .events
        .iter_mut()
        .find(|e| e.kind == AuthEventKind::AccountRecovery)
        .expect("recovery event");
    event.managed_healthy = false;
    event.passkey_posture = PasskeyPosture::UnavailableFallbackExplicit;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::PasskeyUnavailable,
        paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
        local_capabilities_remaining: vec![],
        fallback_posture: FallbackPosture::EmbeddedPasswordFirst,
        keyboard_complete_fallback: true,
        local_work_threatened: false,
        governed_policy_exception_ref: None,
        disposition: super::model::ConditionDisposition::RecoveryOffered,
        detail: "Embedded password-first recovery (withdrawn).".to_owned(),
    });
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::EmbeddedRecoveryRequiredOnStable { event_id }
    );
}

#[test]
fn embedded_recovery_is_allowed_off_stable() {
    let mut input = baseline_input();
    input.profile_channel = super::model::ProfileChannel::Managed;
    let event = input
        .events
        .iter_mut()
        .find(|e| e.kind == AuthEventKind::AccountRecovery)
        .expect("recovery event");
    event.managed_healthy = false;
    event.passkey_posture = PasskeyPosture::UnavailableFallbackExplicit;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::PasskeyUnavailable,
        paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
        local_capabilities_remaining: local_caps(),
        fallback_posture: FallbackPosture::EmbeddedPasswordFirst,
        keyboard_complete_fallback: true,
        local_work_threatened: false,
        governed_policy_exception_ref: None,
        disposition: super::model::ConditionDisposition::RecoveryOffered,
        detail: "A managed profile may mandate embedded recovery.".to_owned(),
    });
    // Off a stable profile the embedded-recovery guard does not fire.
    M5AuthAndRecovery::build(input).expect("builds off stable");
}

#[test]
fn condition_without_keyboard_fallback_is_rejected() {
    let mut input = baseline_input();
    let event = input.events.iter_mut().next().expect("an event");
    event.managed_healthy = false;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::BrowserHandoffFailure,
        paused_capabilities: vec![ManagedCapabilityClass::HostedAi],
        local_capabilities_remaining: local_caps(),
        fallback_posture: FallbackPosture::SystemBrowserDeviceCode,
        keyboard_complete_fallback: false,
        local_work_threatened: false,
        governed_policy_exception_ref: None,
        disposition: super::model::ConditionDisposition::ManagedPausedLocalPreserved,
        detail: "No keyboard-complete path (withdrawn).".to_owned(),
    });
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::ConditionNotKeyboardComplete { event_id });
}

#[test]
fn condition_pausing_nothing_is_rejected() {
    let mut input = baseline_input();
    let event = input.events.iter_mut().next().expect("an event");
    event.managed_healthy = false;
    event.active_condition = Some(super::model::AuthCondition {
        drill: DrillKind::OfflineIdentity,
        paused_capabilities: vec![],
        local_capabilities_remaining: local_caps(),
        fallback_posture: FallbackPosture::SystemBrowserDeviceCode,
        keyboard_complete_fallback: true,
        local_work_threatened: false,
        governed_policy_exception_ref: None,
        disposition: super::model::ConditionDisposition::ManagedPausedLocalPreserved,
        detail: "Opaque condition pausing nothing (withdrawn).".to_owned(),
    });
    let event_id = event.event_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::ConditionPausesNothing { event_id });
}

#[test]
fn refresh_token_must_be_export_excluded() {
    let mut input = baseline_input();
    let store = input
        .credential_stores
        .iter_mut()
        .find(|s| s.credential_class == CredentialClass::RefreshToken)
        .expect("refresh store");
    store.excluded_from_support_export = false;
    let store_id = store.store_id.clone();
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::CredentialExportLeak { store_id });
}

#[test]
fn every_required_credential_class_must_be_present() {
    let mut input = baseline_input();
    input
        .credential_stores
        .retain(|s| s.credential_class != CredentialClass::DelegatedHandle);
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingCredentialClass {
            class: CredentialClass::DelegatedHandle
        }
    );
}

#[test]
fn drill_must_keep_local_preserved() {
    let mut input = baseline_input();
    let drill = input.drills.iter_mut().next().expect("a drill");
    drill.local_preserved = false;
    let kind = drill.kind;
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(err, BuildError::DrillNotLocalPreserving { kind });
}

#[test]
fn every_required_drill_must_be_present() {
    let mut input = baseline_input();
    input
        .drills
        .retain(|drill| drill.kind != DrillKind::OfflineIdentity);
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingDrill {
            kind: DrillKind::OfflineIdentity
        }
    );
}

#[test]
fn every_drill_category_must_be_covered() {
    let mut input = baseline_input();
    // Strip the recovery category from the only drills that carry it.
    for drill in &mut input.drills {
        drill.categories.retain(|c| *c != DrillCategory::Recovery);
        if drill.categories.is_empty() {
            drill.categories.push(DrillCategory::Security);
        }
    }
    let err = M5AuthAndRecovery::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingDrillCategory {
            category: DrillCategory::Recovery
        }
    );
}

#[test]
fn drills_stay_narrowed_but_remain_sound() {
    for scenario in m5_auth_and_recovery_corpus() {
        if scenario.scenario_id == "calm_managed_baseline" {
            continue;
        }
        let record = scenario.record();
        assert_eq!(
            record.trust_qualification.effective_continuity_ceiling,
            ContinuityCeiling::ManagedNarrowedLocalIntact,
            "{} should be narrowed below full",
            scenario.scenario_id
        );
        assert_eq!(
            record.trust_qualification.claim_class,
            AuthRecoveryClaim::NarrowedManagedDegraded,
            "{} should remain a sound narrowed record",
            scenario.scenario_id
        );
        // Local work is preserved in every narrowed scenario.
        assert!(record.pillars.local_first_preserved);
    }
}

#[test]
fn corpus_covers_every_kind_surface_capability_and_drill() {
    use std::collections::BTreeSet;
    let mut kinds = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    let mut capabilities = BTreeSet::new();
    let mut credentials = BTreeSet::new();
    let mut drills = BTreeSet::new();
    let mut categories = BTreeSet::new();
    for scenario in m5_auth_and_recovery_corpus() {
        let record = scenario.record();
        kinds.extend(record.event_kind_coverage);
        surfaces.extend(record.surface_coverage);
        capabilities.extend(record.paused_capability_coverage);
        credentials.extend(record.credential_class_coverage);
        drills.extend(record.drill_coverage);
        categories.extend(record.drill_category_coverage);
    }
    for kind in AuthEventKind::REQUIRED {
        assert!(kinds.contains(&kind), "missing event kind {kind:?}");
    }
    for surface in AuthSurface::REQUIRED {
        assert!(surfaces.contains(&surface), "missing surface {surface:?}");
    }
    for capability in [
        ManagedCapabilityClass::HostedAi,
        ManagedCapabilityClass::ManagedSync,
        ManagedCapabilityClass::MarketplacePublish,
        ManagedCapabilityClass::CompanionControl,
        ManagedCapabilityClass::PolicyDistribution,
        ManagedCapabilityClass::OrgCollab,
    ] {
        assert!(
            capabilities.contains(&capability),
            "missing capability {capability:?}"
        );
    }
    for class in [
        CredentialClass::RefreshToken,
        CredentialClass::DelegatedHandle,
        CredentialClass::SessionBroker,
    ] {
        assert!(credentials.contains(&class), "missing credential {class:?}");
    }
    for kind in DrillKind::REQUIRED {
        assert!(drills.contains(&kind), "missing drill {kind:?}");
    }
    for category in DrillCategory::REQUIRED {
        assert!(
            categories.contains(&category),
            "missing category {category:?}"
        );
    }
}
