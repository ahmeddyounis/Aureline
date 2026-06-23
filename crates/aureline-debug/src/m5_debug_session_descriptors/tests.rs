//! Unit tests for the M5 debug-session descriptor set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_debug_session_descriptor_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_debug_session_descriptor_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: DebugSessionDescriptorSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_session_mode_is_materialized() {
    let set = m5_debug_session_descriptor_set();
    for mode in DebugSessionModeClass::ALL {
        assert!(
            set.session_in_mode(mode).is_some(),
            "missing session for mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn inspect_only_modes_never_hold_live_authority() {
    let set = m5_debug_session_descriptor_set();
    for session in &set.sessions {
        if session.mode.is_inspect_only() {
            assert!(
                !session.holds_live_authority,
                "inspect-only session {} must not hold live authority",
                session.session_id
            );
        }
    }
}

#[test]
fn restored_layout_session_holds_no_authority() {
    let set = m5_debug_session_descriptor_set();
    let restored = set
        .sessions
        .iter()
        .find(|s| s.reentry_posture == ReentryPosture::RestoredLayoutOnly)
        .expect("a restored-layout session exists");
    assert!(!restored.holds_live_authority);
    assert_eq!(restored.run_state, SessionRunStateClass::AwaitingReattach);
    assert_eq!(restored.adapter_drift, AdapterDriftClass::ReconnectRequired);
}

#[test]
fn explicit_reattach_reacquires_authority() {
    let set = m5_debug_session_descriptor_set();
    let reattached = set
        .sessions
        .iter()
        .find(|s| s.reentry_posture == ReentryPosture::ReattachedReacquiredAuthority)
        .expect("a reattached session exists");
    assert!(reattached.holds_live_authority);
    // Re-entry reuses the canonical session/target identity rather than a new one.
    let restored = set
        .sessions
        .iter()
        .find(|s| s.reentry_posture == ReentryPosture::RestoredLayoutOnly)
        .expect("a restored session exists");
    assert_eq!(
        reattached.target_descriptor_ref, restored.target_descriptor_ref,
        "reattach reuses the same canonical target identity"
    );
}

#[test]
fn live_authority_is_derived_consistently() {
    let set = m5_debug_session_descriptor_set();
    for session in &set.sessions {
        let derived = DebugSessionDescriptor::derive_holds_live_authority(
            session.mode,
            session.reentry_posture,
            session.adapter_drift,
        );
        assert_eq!(
            session.holds_live_authority, derived,
            "session {} live authority must equal its derivation",
            session.session_id
        );
    }
}

#[test]
fn session_echoes_match_referenced_targets() {
    let set = m5_debug_session_descriptor_set();
    for session in &set.sessions {
        let target = set
            .target(&session.target_descriptor_ref)
            .expect("session resolves its target");
        assert!(
            session.target_identity_echo.matches(target),
            "session {} echo must match target {}",
            session.session_id,
            target.descriptor_id
        );
    }
}

#[test]
fn adapter_drift_states_are_all_present() {
    let set = m5_debug_session_descriptor_set();
    let session_states: std::collections::BTreeSet<_> =
        set.sessions.iter().map(|s| s.adapter_drift).collect();
    let target_states: std::collections::BTreeSet<_> =
        set.targets.iter().map(|t| t.adapter_drift).collect();
    for state in [
        AdapterDriftClass::AdapterCurrent,
        AdapterDriftClass::AdapterDrifted,
        AdapterDriftClass::ReconnectRequired,
        AdapterDriftClass::InspectOnlyNoAdapter,
        AdapterDriftClass::UnsupportedSkew,
    ] {
        assert!(
            session_states.contains(&state) || target_states.contains(&state),
            "adapter drift state {} must appear somewhere",
            state.as_str()
        );
    }
}

#[test]
fn tampering_with_an_echo_fails_validation() {
    let mut set = m5_debug_session_descriptor_set();
    set.sessions[0].target_identity_echo.privilege_token = "system".to_owned();
    assert!(
        set.validate().is_err(),
        "tampered echo must fail validation"
    );
}

#[test]
fn tampering_with_a_live_authority_flag_fails_validation() {
    let mut set = m5_debug_session_descriptor_set();
    // Force a core-file session to claim live authority.
    let core = set
        .sessions
        .iter_mut()
        .find(|s| s.mode == DebugSessionModeClass::CoreFile)
        .expect("core-file session exists");
    core.holds_live_authority = true;
    assert!(
        set.validate().is_err(),
        "a static session claiming live authority must fail validation"
    );
}

#[test]
fn unknown_target_reference_fails_validation() {
    let mut set = m5_debug_session_descriptor_set();
    set.sessions[0].target_descriptor_ref = "debug.attach_target:does_not_exist".to_owned();
    assert!(set.validate().is_err());
}

#[test]
fn lines_projection_covers_targets_sessions_and_invariants() {
    let set = m5_debug_session_descriptor_set();
    let lines = m5_debug_session_descriptor_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Attach targets:")));
    assert!(lines.iter().any(|l| l.contains("Sessions:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    assert!(lines.iter().any(|l| l.contains("live_authority=false")));
    assert!(lines.iter().any(|l| l.contains("live_authority=true")));
}

#[test]
fn enum_tokens_are_stable_and_unique() {
    fn unique<const N: usize>(tokens: [&str; N]) -> bool {
        tokens
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == N
    }
    assert!(unique(DebugSessionModeClass::ALL.map(|m| m.as_str())));
    assert!(unique(DebugEntrypointClass::ALL.map(|e| e.as_str())));
    assert!(unique(TargetBoundaryClass::ALL.map(|b| b.as_str())));
    assert!(unique(TargetMutabilityClass::ALL.map(|m| m.as_str())));
    assert!(unique(TargetPrivilegeClass::ALL.map(|p| p.as_str())));
    assert!(unique(TargetKindClass::ALL.map(|k| k.as_str())));
    assert!(unique(AdapterDriftClass::ALL.map(|d| d.as_str())));
    assert!(unique(ReentryPosture::ALL.map(|r| r.as_str())));
    assert!(unique(SessionRunStateClass::ALL.map(|r| r.as_str())));
}
