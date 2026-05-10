//! Fixture-driven coverage for the seed [`BrowserCallbackPacket`] contract.
//!
//! Every JSON file under `fixtures/auth/browser_callback_cases/` parses into
//! the seed packet shape and projects to the same shell-auth chip vocabulary a
//! consumer would render. The failure-drill fixture additionally feeds the
//! seed validator with a returning-callback envelope that attempts a silent
//! embedded fallback and asserts the typed denial reason fires while the
//! preserved-local-work block stays intact.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    AccountBoundaryClass, AuthFlowClass, BrowserCallbackHandoff, BrowserCallbackPacket,
    EmbeddedFallbackPosture, IdentityModeAlias, PendingSessionDeniedReason, PendingSessionState,
    PreservedLocalWork, ReturnedCallbackInputs, RetryPathClass, ReturnModeClass,
    ReturnOriginValidationClass, ReturnTenantOrWorkspaceMatchRule, ShellAuthChip,
    ShellAuthVocabulary, StageSystemBrowserHandoffRequest, TrustState,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/auth/browser_callback_cases")
}

fn load_packet(file_name: &str) -> BrowserCallbackPacket {
    let path = fixture_dir().join(file_name);
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

#[test]
fn account_free_local_fixture_parses_and_projects_local_only_chip() {
    let packet = load_packet("account_free_local_no_sign_in.json");
    assert_eq!(packet.auth_flow_class, AuthFlowClass::NotApplicable);
    assert_eq!(
        packet.account_boundary_class,
        AccountBoundaryClass::LocalOnly
    );
    assert_eq!(packet.pending_session_state, PendingSessionState::Completed);
    assert_eq!(packet.identity_mode, IdentityModeAlias::AccountFreeLocal);
    assert_eq!(
        packet.embedded_fallback_posture,
        EmbeddedFallbackPosture::EmbeddedFallbackForbidden
    );
    assert!(packet.preserves_local_work());
    assert!(!packet.requires_visible_recovery());

    let chip = ShellAuthChip::from_packet(&packet);
    assert_eq!(chip.vocabulary, ShellAuthVocabulary::AccountFreeLocal);
    assert_eq!(chip.chip_label, "Local only");
    assert!(chip.local_path_available);
    assert!(!chip.visible_recovery_required);
    assert_eq!(
        chip.primary_recovery_action,
        RetryPathClass::ContinueLocalWithoutSignIn
    );
}

#[test]
fn managed_sign_in_outbound_fixture_parses_and_projects_reauth_chip() {
    let packet = load_packet("managed_sign_in_outbound_browser_handoff.json");
    assert_eq!(packet.auth_flow_class, AuthFlowClass::SystemBrowser);
    assert_eq!(packet.account_boundary_class, AccountBoundaryClass::Managed);
    assert_eq!(
        packet.pending_session_state,
        PendingSessionState::AwaitingBrowserReturn
    );
    assert_eq!(
        packet.embedded_fallback_posture,
        EmbeddedFallbackPosture::EmbeddedFallbackForbidden
    );
    assert_eq!(
        packet.return_route.return_mode_class,
        ReturnModeClass::LoopbackHttpReturn
    );
    assert_eq!(
        packet.return_route.return_origin_validation_class,
        ReturnOriginValidationClass::LoopbackPortPinned
    );
    assert_eq!(
        packet.return_route.return_tenant_or_workspace_match_rule,
        ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant
    );
    assert!(packet.recovery_path.local_continuity_offered());
    assert!(packet.requires_visible_recovery());

    let chip = ShellAuthChip::from_packet(&packet);
    assert_eq!(chip.vocabulary, ShellAuthVocabulary::ReauthRequired);
    assert!(chip.visible_recovery_required);
    assert!(
        chip.local_path_available,
        "managed sign-in pending must not block the no-account local path"
    );
}

#[test]
fn failure_drill_fixture_denies_silent_embedded_fallback_and_preserves_local_work() {
    // Failure drill from .plans/M01-079.md: complete auth in the system
    // browser with the app partially unavailable. A silent embedded webview
    // fallback is attempted; the seed packet must fail closed with the typed
    // denial reason and keep preserved-local-work readable so the no-account
    // local path stays truthful.
    let packet = load_packet("failure_drill_app_partially_unavailable.json");

    // Stage a fresh seed handoff using the fixture's outbound-state inputs so
    // the validator runs end-to-end against the same correlation envelope.
    let preserved_local_work = packet.preserved_local_work.clone();
    let mut handoff = BrowserCallbackHandoff::stage_system_browser_handoff(
        StageSystemBrowserHandoffRequest {
            packet_id: &packet.packet_id,
            identity_mode: packet.identity_mode,
            account_boundary_class: packet.account_boundary_class,
            trust_state: packet.trust_state,
            provider_domain_label: &packet.provider_domain_label,
            destination_class_label: &packet.destination_class_label,
            return_target_label: &packet.return_route.return_target_label,
            return_anchor_ref: &packet.return_route.return_anchor_ref,
            return_mode_class: packet.return_route.return_mode_class,
            return_origin_validation_class: packet.return_route.return_origin_validation_class,
            return_tenant_or_workspace_match_rule: packet
                .return_route
                .return_tenant_or_workspace_match_rule,
            return_policy_check_refs: &[],
            bound_workspace_ref: packet.callback_correlation.bound_workspace_ref.as_deref(),
            bound_tenant_or_org_ref: packet
                .callback_correlation
                .bound_tenant_or_org_ref
                .as_deref(),
            bound_actor_subject_ref: packet
                .callback_correlation
                .bound_actor_subject_ref
                .as_deref(),
            correlation_id: &packet.callback_correlation.correlation_id,
            pending_session_id: &packet.callback_correlation.pending_session_id,
            state_token_alias: &packet.callback_correlation.state_token_alias,
            nonce_alias: &packet.callback_correlation.nonce_alias,
            pkce_challenge_alias: packet
                .callback_correlation
                .pkce_challenge_alias
                .as_deref(),
            issued_at: &packet.callback_correlation.issued_at,
            expires_at: &packet.callback_correlation.expires_at,
            recovery_copy_label: &packet.recovery_path.recovery_copy_label,
            primary_recovery_action: packet.recovery_path.primary_recovery_action,
            fallback_recovery_actions: &packet.recovery_path.fallback_recovery_actions,
            repair_hook_ref: packet.recovery_path.repair_hook_ref.as_deref(),
            preserved_local_work,
            execution_context_ref: packet.execution_context_ref.as_deref(),
        },
    )
    .expect("staging the failure-drill handoff must succeed");

    // The fixture's `__fixture__.redeem_inputs` block names the returning
    // callback envelope. The validator sees `embedded_fallback_attempted` and
    // must fail closed with the typed denial reason.
    handoff.redeem(ReturnedCallbackInputs {
        returning_state_token_alias: "state_alias.managed_sign_in.failure_drill.0001",
        returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        returning_tenant_or_org_ref: Some("tenant.acme_prod"),
        returning_workspace_ref: Some("workspace.payments_prod"),
        embedded_fallback_attempted: true,
        policy_blocked: false,
        observed_at: "2026-04-23T10:12:00Z",
    });

    let after = handoff.packet();
    assert_eq!(after.pending_session_state, PendingSessionState::Denied);
    assert_eq!(
        after.pending_session_denied_reason,
        Some(PendingSessionDeniedReason::CallbackEmbeddedFallbackAttempted),
    );
    assert!(
        after.preserves_local_work(),
        "preserved-local-work block survives the typed denial"
    );
    assert!(after.recovery_path.local_continuity_offered());

    let chip = ShellAuthChip::from_packet(after);
    assert_eq!(chip.vocabulary, ShellAuthVocabulary::ReauthRequired);
    assert!(chip.visible_recovery_required);
    assert!(
        chip.local_path_available,
        "the no-account local path stays usable after the typed denial"
    );

    // The serialized fixture already records the post-deny shape; cross-
    // check it lines up with the validator output so the fixture remains a
    // reviewable artifact.
    assert_eq!(packet.pending_session_state, after.pending_session_state);
    assert_eq!(
        packet.pending_session_denied_reason,
        after.pending_session_denied_reason
    );
}

#[test]
fn every_fixture_in_browser_callback_cases_parses_into_the_seed_packet_shape() {
    let dir = fixture_dir();
    let entries = fs::read_dir(&dir)
        .unwrap_or_else(|err| panic!("fixture dir {} must read: {err}", dir.display()));
    let mut parsed = 0_usize;
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let packet: BrowserCallbackPacket = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        // Every seed packet declares the embedded-fallback posture and the
        // record-kind / schema-version envelope; the chip projection always
        // resolves to a non-empty token.
        assert_eq!(
            packet.embedded_fallback_posture,
            EmbeddedFallbackPosture::EmbeddedFallbackForbidden,
            "fixture {} must forbid embedded fallback",
            path.display()
        );
        assert!(packet.schema_version >= 1);
        let chip = ShellAuthChip::from_packet(&packet);
        assert!(!chip.vocabulary_token.is_empty());
        assert!(!chip.chip_label.is_empty());
        // We never collapse `local_only` into a `Connected` badge: the chip's
        // local-path availability stays truthful for every seed posture.
        let _ = TrustState::Trusted; // keep the import meaningful even on parse-only paths.
        parsed += 1;
    }
    assert!(
        parsed >= 3,
        "expected at least three seed fixtures (account-free local, managed outbound, failure drill); found {parsed}",
    );
    let _ = (
        AccountBoundaryClass::LocalOnly,
        IdentityModeAlias::AccountFreeLocal,
        PreservedLocalWork {
            posture_class: aureline_auth::PreservedLocalWorkPostureClass::LocalWorkIntact,
            note: String::new(),
            retained_capabilities: Vec::new(),
            blocked_capabilities: Vec::new(),
        },
    );
}
