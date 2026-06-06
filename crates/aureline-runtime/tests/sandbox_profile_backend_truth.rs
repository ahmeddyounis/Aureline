use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_sandbox_profile_packet, SandboxBackendPlatformClass, SandboxConsumerSurface,
    SandboxFindingKind, SandboxProfileId, SandboxProfilePacket, SandboxPromotionState,
    SANDBOX_BACKEND_CROSSWALK_REF, SANDBOX_PROFILE_DOC_REF, SANDBOX_PROFILE_HELP_DOC_REF,
    SANDBOX_PROFILE_PACKET_ARTIFACT_REF, SANDBOX_PROFILE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected {rel} to exist at {}",
        path.display()
    );
}

fn load_artifact_packet() -> SandboxProfilePacket {
    let path = repo_root().join(SANDBOX_PROFILE_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"))
}

fn finding_tokens(packet: &SandboxProfilePacket) -> BTreeSet<&'static str> {
    packet
        .validation_findings
        .iter()
        .map(|finding| finding.finding_kind.as_str())
        .collect()
}

#[test]
fn schema_docs_crosswalk_and_packet_exist() {
    assert_exists(SANDBOX_PROFILE_SCHEMA_REF);
    assert_exists(SANDBOX_PROFILE_DOC_REF);
    assert_exists(SANDBOX_PROFILE_HELP_DOC_REF);
    assert_exists(SANDBOX_BACKEND_CROSSWALK_REF);
    assert_exists(SANDBOX_PROFILE_PACKET_ARTIFACT_REF);
}

#[test]
fn current_packet_is_stable_and_support_export_safe() {
    let packet = current_stable_sandbox_profile_packet();
    assert_eq!(packet.promotion_state, SandboxPromotionState::Stable);
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());

    let profile_ids: BTreeSet<SandboxProfileId> = packet
        .profiles
        .iter()
        .map(|profile| profile.profile_id)
        .collect();
    assert_eq!(
        profile_ids,
        SandboxProfileId::ALL.into_iter().collect(),
        "stable packet must publish every claimed sandbox profile"
    );

    let platforms: BTreeSet<SandboxBackendPlatformClass> = packet
        .backend_rows
        .iter()
        .map(|row| row.platform_class)
        .collect();
    assert_eq!(
        platforms,
        SandboxBackendPlatformClass::ALL.into_iter().collect(),
        "stable packet must publish every backend platform"
    );

    let consumers: BTreeSet<SandboxConsumerSurface> = packet
        .consumer_projections
        .iter()
        .map(|projection| projection.consumer_surface)
        .collect();
    assert_eq!(
        consumers,
        SandboxConsumerSurface::REQUIRED.into_iter().collect(),
        "stable packet must project to every required surface"
    );

    let export = packet.support_export(
        "support-export:sandbox-profile:test",
        "2026-06-06T12:05:00Z",
    );
    assert!(export.raw_secret_material_excluded);
    assert!(export.raw_command_material_excluded);
    assert!(export.ambient_authority_excluded);
}

#[test]
fn checked_in_packet_matches_runtime_constructor() {
    let artifact = load_artifact_packet();
    let current = current_stable_sandbox_profile_packet();
    assert_eq!(
        artifact, current,
        "checked-in sandbox packet drifted from runtime constructor"
    );
}

#[test]
fn unsafe_backend_fallback_blocks_stable() {
    let mut packet = current_stable_sandbox_profile_packet();
    packet.backend_rows[0].ambient_fallback_denied = false;
    packet.validation_findings = packet.validate();
    packet.promotion_state = SandboxPromotionState::BlocksStable;
    let tokens = finding_tokens(&packet);
    assert!(tokens.contains(SandboxFindingKind::AmbientFallbackAdmitted.as_str()));
    assert!(!packet.is_stable());
}

#[test]
fn browser_companion_local_execution_blocks_stable() {
    let mut input = aureline_runtime::current_stable_sandbox_profile_packet_input();
    let browser_row = input
        .backend_rows
        .iter_mut()
        .find(|row| row.platform_class == SandboxBackendPlatformClass::BrowserCompanion)
        .expect("browser companion row exists");
    browser_row.hidden_local_execution_denied = false;
    let packet = SandboxProfilePacket::materialize(input);
    let tokens = finding_tokens(&packet);
    assert!(tokens.contains(SandboxFindingKind::BrowserCompanionLocalExecutionAllowed.as_str()));
    assert_eq!(packet.promotion_state, SandboxPromotionState::BlocksStable);
}

#[test]
fn remembered_high_risk_approval_without_fresh_ticket_blocks_stable() {
    let mut input = aureline_runtime::current_stable_sandbox_profile_packet_input();
    input.approval_bindings[0].remembered_approval_mints_fresh_ticket = false;
    input.approval_bindings[0].revalidation_triggers.clear();
    let packet = SandboxProfilePacket::materialize(input);
    let tokens = finding_tokens(&packet);
    assert!(tokens.contains(SandboxFindingKind::RememberedApprovalBypassesFreshTicket.as_str()));
    assert!(tokens.contains(SandboxFindingKind::RevalidationTriggerMissing.as_str()));
    assert_eq!(packet.promotion_state, SandboxPromotionState::BlocksStable);
}
