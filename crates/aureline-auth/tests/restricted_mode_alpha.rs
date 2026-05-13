use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::Deserialize;

use aureline_auth::{
    CapabilityAuthorityClass, IdentityModeAlias, LaunchWedgeCapabilityFamily,
    RememberedDecisionScopeClass, RestrictedModeAlphaPacket, RestrictedModeEntryTransitionClass,
    RestrictedModeTrustStateClass, StageRestrictedModeLaunchRequest, TrustDecisionSourceClass,
    TrustReasonClass, TrustRecoveryActionClass, RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND,
    RESTRICTED_MODE_ALPHA_SCHEMA_VERSION,
};

#[derive(Debug, Deserialize)]
struct RestrictedModeFixture {
    case_name: String,
    request: RestrictedModeFixtureRequest,
    expected: RestrictedModeFixtureExpected,
}

#[derive(Debug, Deserialize)]
struct RestrictedModeFixtureRequest {
    packet_id: String,
    workspace_root_ref: String,
    workspace_display_scope: String,
    identity_mode: IdentityModeAlias,
    prior_trust_state: Option<RestrictedModeTrustStateClass>,
    effective_trust_state: RestrictedModeTrustStateClass,
    entry_transition: RestrictedModeEntryTransitionClass,
    source_class: TrustDecisionSourceClass,
    source_ref: String,
    source_label: String,
    reason_class: TrustReasonClass,
    remembered_decision_scope: RememberedDecisionScopeClass,
    #[serde(default)]
    policy_epoch_ref: Option<String>,
    #[serde(default)]
    source_reason_refs: Vec<String>,
    #[serde(default)]
    recovery_action_ref: Option<String>,
    issued_at: String,
}

#[derive(Debug, Deserialize)]
struct RestrictedModeFixtureExpected {
    binary_trust_state: String,
    required_allowed_families: Vec<String>,
    required_blocked_or_review_families: Vec<String>,
    required_authorities: BTreeMap<String, String>,
    required_recovery_actions: Vec<String>,
    gate_visible_after_open: bool,
    guardrails_hold: bool,
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("trust")
        .join("restricted_mode_alpha")
}

fn load_fixture(file_name: &str) -> RestrictedModeFixture {
    let path = fixtures_dir().join(file_name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("invalid JSON in fixture {}: {err}", path.display()))
}

fn stage_from_fixture(fixture: &RestrictedModeFixture) -> RestrictedModeAlphaPacket {
    let request = &fixture.request;
    let reason_refs: Vec<&str> = request
        .source_reason_refs
        .iter()
        .map(String::as_str)
        .collect();
    RestrictedModeAlphaPacket::stage_launch_wedge(StageRestrictedModeLaunchRequest {
        packet_id: &request.packet_id,
        workspace_root_ref: &request.workspace_root_ref,
        workspace_display_scope: &request.workspace_display_scope,
        identity_mode: request.identity_mode,
        prior_trust_state: request.prior_trust_state,
        effective_trust_state: request.effective_trust_state,
        entry_transition: request.entry_transition,
        source_class: request.source_class,
        source_ref: &request.source_ref,
        source_label: &request.source_label,
        reason_class: request.reason_class,
        remembered_decision_scope: request.remembered_decision_scope,
        policy_epoch_ref: request.policy_epoch_ref.as_deref(),
        source_reason_refs: &reason_refs,
        recovery_action_ref: request.recovery_action_ref.as_deref(),
        issued_at: &request.issued_at,
    })
}

fn family_token(family: LaunchWedgeCapabilityFamily) -> &'static str {
    family.as_str()
}

fn authority_token(authority: CapabilityAuthorityClass) -> &'static str {
    authority.as_str()
}

fn recovery_token(action: TrustRecoveryActionClass) -> &'static str {
    action.as_str()
}

#[test]
fn restricted_mode_fixtures_validate_and_project_disclosure() {
    for file_name in [
        "unknown_workspace_open_restricted.json",
        "trust_revoked_after_open.json",
        "policy_degraded_ai_apply_review.json",
    ] {
        let fixture = load_fixture(file_name);
        let packet = stage_from_fixture(&fixture);
        assert_eq!(packet.record_kind, RESTRICTED_MODE_ALPHA_PACKET_RECORD_KIND);
        assert_eq!(packet.schema_version, RESTRICTED_MODE_ALPHA_SCHEMA_VERSION);
        assert_eq!(
            packet.binary_trust_state.as_str(),
            fixture.expected.binary_trust_state,
            "{}",
            fixture.case_name
        );

        let violations = packet.validate();
        let violation_tokens: Vec<&str> = violations
            .iter()
            .map(|violation| violation.token())
            .collect();
        assert!(
            violations.is_empty(),
            "{} violations: {violation_tokens:?}",
            fixture.case_name
        );
        assert!(packet.restricted_floor_available(), "{}", fixture.case_name);
        assert!(
            packet.has_allowed_and_blocked_disclosure(),
            "{}",
            fixture.case_name
        );
        assert!(
            packet.blocked_rows_explain_source_scope_and_recovery(),
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.trust_gate_persists_after_open(),
            fixture.expected.gate_visible_after_open,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.guardrails_hold(),
            fixture.expected.guardrails_hold,
            "{}",
            fixture.case_name
        );

        let allowed: BTreeSet<&str> = packet
            .allowed_rows()
            .iter()
            .map(|row| family_token(row.surface_family))
            .collect();
        for expected in &fixture.expected.required_allowed_families {
            assert!(
                allowed.contains(expected.as_str()),
                "{} missing allowed family {expected}",
                fixture.case_name
            );
        }

        let blocked_or_review: BTreeSet<&str> = packet
            .blocked_or_review_rows()
            .iter()
            .map(|row| family_token(row.surface_family))
            .collect();
        for expected in &fixture.expected.required_blocked_or_review_families {
            assert!(
                blocked_or_review.contains(expected.as_str()),
                "{} missing blocked/review family {expected}",
                fixture.case_name
            );
        }

        for (family, expected_authority) in &fixture.expected.required_authorities {
            let row = packet
                .capability_gates
                .iter()
                .find(|row| row.surface_family_token == *family)
                .unwrap_or_else(|| panic!("{} missing family {family}", fixture.case_name));
            assert_eq!(
                authority_token(row.authority),
                expected_authority,
                "{} authority mismatch for {family}",
                fixture.case_name
            );
        }

        let recovery_actions: BTreeSet<&str> = packet
            .blocked_or_review_rows()
            .iter()
            .flat_map(|row| row.recovery_actions.iter().copied())
            .map(recovery_token)
            .collect();
        for expected in &fixture.expected.required_recovery_actions {
            assert!(
                recovery_actions.contains(expected.as_str()),
                "{} missing recovery action {expected}",
                fixture.case_name
            );
        }

        let disclosure = packet.launch_wedge_disclosure();
        assert_eq!(
            disclosure.trust_gate_visible_after_open,
            fixture.expected.gate_visible_after_open
        );
        let plaintext = disclosure.render_plaintext();
        assert!(plaintext.contains("allowed:"));
        assert!(plaintext.contains("blocked_or_review:"));
        assert!(plaintext.contains("source="));
        assert!(plaintext.contains("scope="));
        assert!(plaintext.contains("recovery="));
    }
}
