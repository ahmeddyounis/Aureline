//! Fixture-driven coverage for claimed identity rows that default to
//! system-browser auth and expose device-code or stay-local fallback paths.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    BrowserLaunchPolicyClass, ClaimedIdentityDefaultActionClass, ClaimedIdentityStateClass,
    EmbeddedFallbackPosture, RetryPathClass, SystemBrowserAlphaPacket,
    SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND, SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/auth/system_browser_alpha")
}

fn load_packet(file_name: &str) -> SystemBrowserAlphaPacket {
    let path = fixture_dir().join(file_name);
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

#[test]
fn managed_claim_fixture_defaults_to_system_browser_and_keeps_fallbacks() {
    let packet = load_packet("managed_claim_prefers_system_browser.json");
    assert_eq!(packet.record_kind, SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION);
    assert!(packet.all_claimed_rows_have_fallback());
    assert!(packet.prevents_dead_end_auth_failure());

    let row = &packet.claimed_identity_rows[0];
    assert!(row.defaults_to_system_browser());
    assert!(row.has_device_code_alternative());
    assert!(row.has_stay_local_alternative());
    assert_eq!(
        row.auth_policy.embedded_fallback_posture,
        EmbeddedFallbackPosture::EmbeddedFallbackForbidden
    );
    assert_eq!(
        row.session_window.expires_at.as_deref(),
        Some("2026-05-13T08:10:00Z")
    );

    let surface = packet.surface_rows();
    assert_eq!(surface.len(), 1);
    assert_eq!(surface[0].default_action_token, "open_system_browser");
    assert_eq!(surface[0].provider_domain_label, "login.acme.example");
    assert_eq!(surface[0].provider_scope_label, "payments-prod tenant");
    assert!(surface[0].device_code_available);
    assert!(surface[0].stay_local_available);
    assert!(surface[0].local_work_available);
}

#[test]
fn browser_blocked_fixture_defaults_to_device_code_and_stay_local() {
    let packet = load_packet("browser_blocked_device_code_or_stay_local.json");
    let row = &packet.claimed_identity_rows[0];
    assert_eq!(
        row.state_class,
        ClaimedIdentityStateClass::BrowserLaunchBlocked
    );
    assert_eq!(
        row.auth_policy.browser_launch_policy_class,
        BrowserLaunchPolicyClass::BrowserLaunchPolicyBlocked
    );
    assert_eq!(
        row.default_action,
        ClaimedIdentityDefaultActionClass::UseDeviceCode
    );
    assert_eq!(
        row.primary_recovery_action,
        RetryPathClass::SwitchToDeviceCode
    );
    assert!(row.has_device_code_alternative());
    assert!(row.has_stay_local_alternative());
    assert!(row.visible_recovery_required);
    assert!(!row.dead_end_without_local_continuation());
}

#[test]
fn scope_denied_fixture_defaults_to_stay_local_without_dead_end() {
    let packet = load_packet("scope_denied_stay_local.json");
    let row = &packet.claimed_identity_rows[0];
    assert_eq!(row.state_class, ClaimedIdentityStateClass::AuthDenied);
    assert_eq!(
        row.default_action,
        ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn
    );
    assert_eq!(
        row.primary_recovery_action,
        RetryPathClass::ContinueLocalWithoutSignIn
    );
    assert!(row.local_work_available());
    assert!(row.has_stay_local_alternative());
    assert!(!row.dead_end_without_local_continuation());
    assert!(packet.prevents_dead_end_auth_failure());
}

#[test]
fn every_system_browser_alpha_fixture_parses_and_projects() {
    let dir = fixture_dir();
    let mut parsed = 0_usize;
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let entry = entry.expect("readable directory entry");
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("json") {
            continue;
        }
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let packet: SystemBrowserAlphaPacket = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        assert_eq!(packet.record_kind, SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND);
        assert_eq!(packet.schema_version, SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION);
        assert!(
            packet.all_claimed_rows_have_fallback(),
            "fixture {} must expose device-code or stay-local fallback",
            path.display()
        );
        assert!(
            packet.prevents_dead_end_auth_failure(),
            "fixture {} must not strand local continuation",
            path.display()
        );
        let surface_rows = packet.surface_rows();
        assert_eq!(surface_rows.len(), packet.claimed_identity_rows.len());
        for row in surface_rows {
            assert!(!row.provider_domain_label.is_empty());
            assert!(!row.provider_scope_label.is_empty());
            assert!(!row.expiry_summary_label.is_empty());
            assert!(
                row.device_code_available || row.stay_local_available,
                "surface row {} needs a device-code or stay-local path",
                row.row_id
            );
            assert!(!row.dead_end_without_local_continuation);
        }
        parsed += 1;
    }
    assert!(
        parsed >= 3,
        "expected at least three system-browser alpha fixtures; found {parsed}"
    );
}
