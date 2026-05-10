//! Fixture-driven coverage for the status-bar projection.
//!
//! Each case under `fixtures/ux/status_bar_cases/*.json` exercises one row
//! of the seed contract: the protected walk where every status item is
//! synchronized with the upstream truth, the failure drill where an
//! upstream truth source flips to a degraded state, and a multi-degraded
//! case where several rows must remain truthful at once. The tests drive
//! the canonical [`StatusBarSnapshot::project`] so the projected vocabulary
//! cannot drift between the upstream truth and the bar.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::state_cards::DegradedStateToken;
use aureline_shell::status_bar::{
    BackgroundStateSnapshot, EncodingSnapshot, ProfileSnapshot, StatusBarInputs, StatusBarItemKind,
    StatusBarSnapshot, StatusItemClass, TargetSnapshot,
};
use aureline_workspace::save::{
    BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent, FinalNewlineDetected,
    NewlineModeDetected, SourceFidelityRecord,
};
use aureline_workspace::TrustState as WorkspaceTrustState;

#[derive(Debug, Clone, Deserialize)]
struct StatusBarFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    inputs: FixtureInputs,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureInputs {
    workspace_id: String,
    workspace_trust_state: String,
    target: FixtureTarget,
    profile: FixtureProfile,
    #[serde(default)]
    encoding: Option<FixtureEncoding>,
    background: FixtureBackground,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureTarget {
    target_class_token: String,
    target_label: String,
    reachability_token: String,
    #[serde(default)]
    execution_context_ref: Option<String>,
    has_degraded_field: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureProfile {
    profile_label: String,
    profile_mode_token: String,
    deployment_profile_token: String,
    identity_mode_token: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureEncoding {
    detected_encoding: String,
    detection_source: String,
    bom_state_detected: String,
    newline_mode_detected: String,
    final_newline_detected: String,
    executable_intent: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureBackground {
    active_owners: Vec<String>,
    #[serde(default)]
    aggregate_degraded: Option<String>,
    observed_at: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    has_recovery_critical: bool,
    has_degraded_state: bool,
    items: Vec<ExpectedItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedItem {
    item_kind: String,
    item_class: String,
    priority_rank: u32,
    stable_slot_key: String,
    current_value_label: String,
    #[serde(default)]
    degraded_token: Option<String>,
    is_recovery_critical: bool,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/status_bar_cases")
}

fn trust_state_from(token: &str) -> WorkspaceTrustState {
    match token {
        "trusted" => WorkspaceTrustState::Trusted,
        "restricted" => WorkspaceTrustState::Restricted,
        "pending_evaluation" => WorkspaceTrustState::PendingEvaluation,
        other => panic!("unsupported trust_state token: {other}"),
    }
}

fn detected_encoding_from(token: &str) -> DetectedEncoding {
    match token {
        "utf8" => DetectedEncoding::Utf8,
        "utf8_bom" => DetectedEncoding::Utf8Bom,
        "utf16le_bom" => DetectedEncoding::Utf16LeBom,
        "utf16be_bom" => DetectedEncoding::Utf16BeBom,
        "utf32le_bom" => DetectedEncoding::Utf32LeBom,
        "utf32be_bom" => DetectedEncoding::Utf32BeBom,
        "unknown_binary_like" => DetectedEncoding::UnknownBinaryLike,
        other => panic!("unsupported encoding token: {other}"),
    }
}

fn detection_source_from(token: &str) -> DetectionSource {
    match token {
        "bom" => DetectionSource::Bom,
        "utf8_heuristic" => DetectionSource::Utf8Heuristic,
        "decode_failed_no_choice" => DetectionSource::DecodeFailedNoChoice,
        other => panic!("unsupported detection_source token: {other}"),
    }
}

fn bom_state_from(token: &str) -> BomStateDetected {
    match token {
        "present" => BomStateDetected::Present,
        "absent" => BomStateDetected::Absent,
        "unknown_or_degraded" => BomStateDetected::UnknownOrDegraded,
        other => panic!("unsupported bom_state token: {other}"),
    }
}

fn newline_mode_from(token: &str) -> NewlineModeDetected {
    match token {
        "lf" => NewlineModeDetected::Lf,
        "crlf" => NewlineModeDetected::Crlf,
        "cr_only" => NewlineModeDetected::CrOnly,
        "mixed" => NewlineModeDetected::Mixed,
        "unknown_or_degraded" => NewlineModeDetected::UnknownOrDegraded,
        other => panic!("unsupported newline_mode token: {other}"),
    }
}

fn final_newline_from(token: &str) -> FinalNewlineDetected {
    match token {
        "present" => FinalNewlineDetected::Present,
        "absent" => FinalNewlineDetected::Absent,
        "unknown_or_degraded" => FinalNewlineDetected::UnknownOrDegraded,
        other => panic!("unsupported final_newline token: {other}"),
    }
}

fn executable_intent_from(token: &str) -> ExecutableIntent {
    match token {
        "executable" => ExecutableIntent::Executable,
        "non_executable" => ExecutableIntent::NonExecutable,
        "unknown_or_degraded" => ExecutableIntent::UnknownOrDegraded,
        other => panic!("unsupported executable_intent token: {other}"),
    }
}

fn degraded_token_from(token: &str) -> DegradedStateToken {
    match token {
        "Warming" => DegradedStateToken::Warming,
        "Cached" => DegradedStateToken::Cached,
        "Partial" => DegradedStateToken::Partial,
        "Stale" => DegradedStateToken::Stale,
        "Offline" => DegradedStateToken::Offline,
        "PolicyBlocked" => DegradedStateToken::PolicyBlocked,
        "Limited" => DegradedStateToken::Limited,
        "Unsupported" => DegradedStateToken::Unsupported,
        "Experimental" => DegradedStateToken::Experimental,
        "RetestPending" => DegradedStateToken::RetestPending,
        other => panic!("unsupported degraded token: {other}"),
    }
}

fn item_kind_from(token: &str) -> StatusBarItemKind {
    match token {
        "target" => StatusBarItemKind::Target,
        "profile" => StatusBarItemKind::Profile,
        "trust" => StatusBarItemKind::Trust,
        "encoding" => StatusBarItemKind::Encoding,
        "background_state" => StatusBarItemKind::BackgroundState,
        other => panic!("unsupported item_kind token: {other}"),
    }
}

fn item_class_from(token: &str) -> StatusItemClass {
    match token {
        "recovery_critical" => StatusItemClass::RecoveryCritical,
        "active_context_truth" => StatusItemClass::ActiveContextTruth,
        "ongoing_work" => StatusItemClass::OngoingWork,
        "ambient_metadata" => StatusItemClass::AmbientMetadata,
        other => panic!("unsupported item_class token: {other}"),
    }
}

fn build_fidelity(record: &FixtureEncoding) -> SourceFidelityRecord {
    SourceFidelityRecord {
        detected_encoding: detected_encoding_from(&record.detected_encoding),
        detection_source: detection_source_from(&record.detection_source),
        bom_state_detected: bom_state_from(&record.bom_state_detected),
        newline_mode_detected: newline_mode_from(&record.newline_mode_detected),
        final_newline_detected: final_newline_from(&record.final_newline_detected),
        executable_intent: executable_intent_from(&record.executable_intent),
    }
}

fn run_fixture(path: &Path, fixture: &StatusBarFixture) {
    assert_eq!(
        fixture.record_kind, "status_bar_state_case",
        "unexpected record_kind in {path:?}"
    );
    assert_eq!(
        fixture.schema_version, 1,
        "unexpected schema_version in {path:?}"
    );

    let owner_strings: Vec<String> = fixture.inputs.background.active_owners.clone();
    let owner_refs: Vec<&str> = owner_strings.iter().map(String::as_str).collect();
    let aggregate_degraded = fixture
        .inputs
        .background
        .aggregate_degraded
        .as_deref()
        .map(degraded_token_from);

    let fidelity = fixture.inputs.encoding.as_ref().map(build_fidelity);

    let inputs = StatusBarInputs {
        workspace_id: fixture.inputs.workspace_id.as_str(),
        workspace_trust_state: trust_state_from(&fixture.inputs.workspace_trust_state),
        target: TargetSnapshot {
            target_class_token: fixture.inputs.target.target_class_token.as_str(),
            target_label: fixture.inputs.target.target_label.as_str(),
            reachability_token: fixture.inputs.target.reachability_token.as_str(),
            execution_context_ref: fixture.inputs.target.execution_context_ref.as_deref(),
            has_degraded_field: fixture.inputs.target.has_degraded_field,
        },
        profile: ProfileSnapshot {
            profile_label: fixture.inputs.profile.profile_label.as_str(),
            profile_mode_token: fixture.inputs.profile.profile_mode_token.as_str(),
            deployment_profile_token: fixture.inputs.profile.deployment_profile_token.as_str(),
            identity_mode_token: fixture.inputs.profile.identity_mode_token.as_str(),
        },
        encoding: EncodingSnapshot {
            source_fidelity: fidelity.as_ref(),
        },
        background: BackgroundStateSnapshot {
            active_owners: &owner_refs,
            aggregate_degraded,
            observed_at: fixture.inputs.background.observed_at.as_str(),
        },
    };

    let snapshot = StatusBarSnapshot::project(&inputs);

    assert_eq!(
        snapshot.workspace_id, fixture.inputs.workspace_id,
        "workspace_id mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        snapshot.has_recovery_critical(),
        fixture.expect.has_recovery_critical,
        "has_recovery_critical mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        snapshot.has_degraded_state(),
        fixture.expect.has_degraded_state,
        "has_degraded_state mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        snapshot.items.len(),
        fixture.expect.items.len(),
        "items count mismatch in {path:?} ({})",
        fixture.case_name
    );

    for (actual, expected) in snapshot.items.iter().zip(fixture.expect.items.iter()) {
        let expected_kind = item_kind_from(&expected.item_kind);
        assert_eq!(
            actual.item_kind, expected_kind,
            "item_kind mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.item_class,
            item_class_from(&expected.item_class),
            "item_class mismatch on {} in {path:?} ({})",
            expected.item_kind,
            fixture.case_name
        );
        assert_eq!(
            actual.priority_rank, expected.priority_rank,
            "priority_rank mismatch on {} in {path:?} ({})",
            expected.item_kind, fixture.case_name
        );
        assert_eq!(
            actual.stable_slot_key, expected.stable_slot_key,
            "stable_slot_key mismatch on {} in {path:?} ({})",
            expected.item_kind, fixture.case_name
        );
        assert_eq!(
            actual.current_value_label, expected.current_value_label,
            "current_value_label mismatch on {} in {path:?} ({})",
            expected.item_kind, fixture.case_name
        );
        assert_eq!(
            actual.degraded_token, expected.degraded_token,
            "degraded_token mismatch on {} in {path:?} ({})",
            expected.item_kind, fixture.case_name
        );
        assert_eq!(
            actual.is_recovery_critical, expected.is_recovery_critical,
            "is_recovery_critical mismatch on {} in {path:?} ({})",
            expected.item_kind, fixture.case_name
        );
    }
}

fn load_fixture(path: &Path) -> StatusBarFixture {
    let raw = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("failed to deserialize {path:?}: {err}"))
}

#[test]
fn protected_walk_local_workspace_case() {
    let path = fixtures_dir().join("protected_walk_local_workspace.json");
    run_fixture(&path, &load_fixture(&path));
}

#[test]
fn failure_drill_restricted_trust_case() {
    let path = fixtures_dir().join("failure_drill_restricted_trust.json");
    run_fixture(&path, &load_fixture(&path));
}

#[test]
fn multi_degraded_remote_offline_case() {
    let path = fixtures_dir().join("multi_degraded_remote_offline.json");
    run_fixture(&path, &load_fixture(&path));
}
