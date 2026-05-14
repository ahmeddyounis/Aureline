//! Fixture-driven validation for boundary fallback alpha packets.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::SystemBrowserAlphaPacket;
use aureline_provider::ApprovalTicketAlphaPacket;
use aureline_shell::deeplink::native_handoff::seeded_native_boundary_handoff_packet;
use aureline_shell::embedded::boundary_alpha::seeded_embedded_boundary_alpha_snapshot;
use aureline_shell::embedded::boundary_fallback_alpha::{
    BoundaryFallbackAlphaPacket, BoundaryFallbackSourceEvidence, CallbackInterruptionClass,
    NativeApprovalOwnerClass,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate manifest must live under crates/aureline-shell")
        .to_path_buf()
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let payload = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("{} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("{} must parse: {err}", path.display()))
}

fn load_boundary_packet() -> BoundaryFallbackAlphaPacket {
    load_json(&repo_root().join("fixtures/auth/boundary_fallback_alpha/baseline_packet.json"))
}

fn load_system_browser_packet() -> SystemBrowserAlphaPacket {
    load_json(
        &repo_root()
            .join("fixtures/auth/system_browser_alpha/managed_claim_prefers_system_browser.json"),
    )
}

fn load_approval_packet() -> ApprovalTicketAlphaPacket {
    load_json(&repo_root().join("fixtures/security/approval_ticket_alpha/baseline_packet.json"))
}

#[test]
fn baseline_packet_validates_against_source_artifacts() {
    let packet = load_boundary_packet();
    let system_browser_packet = load_system_browser_packet();
    let embedded_boundary_snapshot =
        seeded_embedded_boundary_alpha_snapshot("id:build:test:boundary-fallback");
    let native_boundary_handoff_packet =
        seeded_native_boundary_handoff_packet("id:build:test:boundary-fallback");
    let approval_packet = load_approval_packet();
    let approval_ticket_report = approval_packet.validate();
    let approval_ticket_support_projection = approval_packet.support_admin_projection();

    let report = packet.validate_against_sources(BoundaryFallbackSourceEvidence {
        system_browser_packet: &system_browser_packet,
        embedded_boundary_snapshot: &embedded_boundary_snapshot,
        native_boundary_handoff_packet: &native_boundary_handoff_packet,
        approval_ticket_report: &approval_ticket_report,
        approval_ticket_support_projection: &approval_ticket_support_projection,
    });

    assert!(
        report.passed,
        "boundary fallback packet failed validation: {:#?}",
        report.findings
    );
    assert!(report.coverage.has_system_browser_default);
    assert!(report.coverage.has_native_reapproval);
    assert!(report
        .coverage
        .callback_interruption_classes
        .contains(&CallbackInterruptionClass::CallbackReplay));
}

#[test]
fn fixture_rejects_embedded_approval_ownership() {
    let mut packet = load_boundary_packet();
    let row = packet
        .claimed_rows
        .iter_mut()
        .find(|row| row.row_id == "boundary-row:extension-webview")
        .expect("fixture has extension row");
    row.native_approval.owner_class = NativeApprovalOwnerClass::EmbeddedOwnedDenied;
    row.native_approval.high_risk_approval_product_owned = false;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "boundary_fallback.native_approval_owner_invalid" }));
}

#[test]
fn fixture_rejects_missing_browser_fallback() {
    let mut packet = load_boundary_packet();
    let row = packet
        .claimed_rows
        .iter_mut()
        .find(|row| row.row_id == "boundary-row:marketplace-account")
        .expect("fixture has marketplace row");
    row.browser_fallback.available = false;
    row.browser_fallback.exact_target_reopen = false;
    row.browser_fallback.truthful_placeholder_recovery = false;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "boundary_fallback.browser_fallback_missing"));
}

#[test]
fn fixture_rejects_interrupted_callback_without_native_reapproval() {
    let mut packet = load_boundary_packet();
    let row = packet
        .callback_rows
        .iter_mut()
        .find(|row| row.callback_id == "callback-review:sleep-wake:trust-store-change")
        .expect("fixture has trust-store-change callback row");
    row.native_reapproval_required = false;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "boundary_fallback.interrupted_callback_reapproval_missing"
    }));
}
