//! Fixture-backed checks for alpha command review enforcement.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_commands::enablement::EnablementDecisionClass;
use aureline_commands::invocation::{
    AliasUsedBlock, ApprovalPostureBlock, CommandInvocationSession, ContextRefsBlock,
    EnablementDecisionBlock, InvocationContextSnapshot, PreviewPostureBlock,
};
use aureline_commands::registry::seeded_registry;
use aureline_commands::PreflightDecisionClass;
use aureline_shell::commands::review_enforcement::{
    enforce_invocation_review_path, materialize_alpha_review_enforcement_snapshot,
};
use aureline_shell::commands::{argument_provenance_map_for, CommandReviewRuntimeInputs};
use aureline_shell::palette::{
    materialize_command_deep_link_review, materialize_invocation_session_for_review,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReviewEnforcementFixture {
    fixture_id: String,
    acceptance: AcceptanceFixture,
    required_rows: Vec<RequiredRowFixture>,
    required_out_of_scope_rows: Vec<OutOfScopeRowFixture>,
    bypass_attempts: Vec<BypassAttemptFixture>,
}

#[derive(Debug, Deserialize)]
struct AcceptanceFixture {
    every_destructive_or_external_effect_command_reviewed_or_out_of_scope: bool,
    git_provider_install_ai_lanes_have_review_rows: bool,
    reviewed_command_bypass_attempts_are_denied: bool,
    explicit_out_of_scope_rows_are_visible: bool,
}

#[derive(Debug, Deserialize)]
struct RequiredRowFixture {
    command_id: String,
    lane_class: String,
    enforcement_status: String,
    review_requirement_class: String,
    preview_class: String,
    approval_posture_class: String,
    required_evidence_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OutOfScopeRowFixture {
    command_id: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct BypassAttemptFixture {
    case_id: String,
    command_id: String,
    issuing_surface: String,
    execution_intent: String,
    preview_shown: bool,
    approval_state: String,
    expected_decision_class: String,
    expected_disabled_reason_code: Option<String>,
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/alpha_preview_apply_revert/manifest.yaml")
}

fn load_fixture() -> ReviewEnforcementFixture {
    let path = fixture_path();
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn alpha_review_enforcement_snapshot_covers_required_lanes() {
    let fixture = load_fixture();
    let snapshot = materialize_alpha_review_enforcement_snapshot(seeded_registry());

    assert_eq!(
        snapshot.record_kind,
        "alpha_review_enforcement_snapshot_record"
    );
    assert!(
        fixture
            .acceptance
            .every_destructive_or_external_effect_command_reviewed_or_out_of_scope
    );
    assert!(
        snapshot.all_destructive_or_external_effect_commands_reviewed_or_out_of_scope(),
        "fixture {} found enforcement gaps: {:#?}",
        fixture.fixture_id,
        snapshot
            .rows
            .iter()
            .filter(|row| !row.finding_codes.is_empty())
            .collect::<Vec<_>>()
    );

    for expected in &fixture.required_rows {
        let row = snapshot
            .row_for_command(&expected.command_id)
            .unwrap_or_else(|| panic!("missing enforcement row {}", expected.command_id));
        assert_eq!(row.lane_class, expected.lane_class);
        assert_eq!(row.enforcement_status, expected.enforcement_status);
        assert_eq!(
            row.review_requirement_class,
            expected.review_requirement_class
        );
        assert_eq!(row.preview_class, expected.preview_class);
        assert_eq!(row.approval_posture_class, expected.approval_posture_class);
        assert!(
            row.is_conforming(),
            "{} should be conforming",
            row.command_id
        );
        for evidence_ref in &expected.required_evidence_refs {
            assert!(
                row.required_evidence_refs
                    .iter()
                    .any(|value| value == evidence_ref),
                "{} missing evidence ref {}",
                row.command_id,
                evidence_ref
            );
        }
    }

    assert!(
        fixture
            .acceptance
            .git_provider_install_ai_lanes_have_review_rows
    );
    let lane_classes: BTreeSet<&str> = snapshot
        .rows
        .iter()
        .map(|row| row.lane_class.as_str())
        .collect();
    for required in ["git", "provider", "install", "ai_mutation"] {
        assert!(lane_classes.contains(required), "missing lane {required}");
    }
}

#[test]
fn alpha_review_enforcement_lists_explicit_out_of_scope_rows() {
    let fixture = load_fixture();
    let snapshot = materialize_alpha_review_enforcement_snapshot(seeded_registry());

    assert!(fixture.acceptance.explicit_out_of_scope_rows_are_visible);
    for expected in &fixture.required_out_of_scope_rows {
        let row = snapshot
            .row_for_command(&expected.command_id)
            .unwrap_or_else(|| panic!("missing out-of-scope row {}", expected.command_id));
        assert_eq!(row.enforcement_status, "explicitly_out_of_scope");
        assert_eq!(row.review_requirement_class, "explicitly_out_of_scope");
        assert_eq!(
            row.explicit_out_of_scope_reason.as_deref(),
            Some(expected.reason.as_str())
        );
    }
}

#[test]
fn reviewed_command_invocation_bypass_attempts_are_denied() {
    let fixture = load_fixture();
    let registry = seeded_registry();

    assert!(
        fixture
            .acceptance
            .reviewed_command_bypass_attempts_are_denied
    );
    for attempt in &fixture.bypass_attempts {
        let entry = registry
            .get(&attempt.command_id)
            .unwrap_or_else(|| panic!("missing command {}", attempt.command_id));
        let session = session_for_attempt(entry, attempt);
        let decision = enforce_invocation_review_path(entry, &session);
        assert_eq!(
            decision.decision_class, attempt.expected_decision_class,
            "case {} decision mismatch",
            attempt.case_id
        );
        let observed_reason = decision
            .disabled_reason_code
            .map(|code| code.as_str().to_string());
        assert_eq!(
            observed_reason, attempt.expected_disabled_reason_code,
            "case {} disabled reason mismatch",
            attempt.case_id
        );
    }
}

#[test]
fn deep_link_review_surfaces_enforcement_row() {
    let registry = seeded_registry();
    let runtime = CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let review =
        materialize_command_deep_link_review(registry, "cmd:workspace.import_profile", runtime)
            .expect("deep-link review materializes");
    assert_eq!(review.route_outcome_class, "invocation_preview_required");
    assert_eq!(review.review_enforcement.enforcement_status, "enforced");
    assert_eq!(
        review.review_enforcement.bypass_protection_class,
        "descriptor_preflight_all_declared_surfaces"
    );
    assert!(review.no_bypass_guards.preview_path_preserved);
}

#[test]
fn materialized_review_session_is_admitted_after_preview_and_approval() {
    let registry = seeded_registry();
    let entry = registry
        .get("cmd:workspace.import_profile")
        .expect("import command exists");
    let runtime = CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let preflight = entry.preflight(&aureline_commands::CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: "trusted".to_string(),
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
        argument_provenance_map: argument_provenance_map_for(entry),
    });
    assert_eq!(
        preflight.decision_class,
        PreflightDecisionClass::PreviewRequired
    );
    let mut session = materialize_invocation_session_for_review(
        entry,
        runtime,
        "command_palette",
        "user_initiated_local",
        &preflight,
    );
    session.approval_posture.approval_state = "approval_granted".to_string();
    let decision = enforce_invocation_review_path(entry, &session);
    assert_eq!(decision.decision_class, "admitted_reviewed_apply");
    assert!(decision.disabled_reason_code.is_none());
}

fn session_for_attempt(
    entry: &aureline_commands::CommandRegistryEntryRecord,
    attempt: &BypassAttemptFixture,
) -> CommandInvocationSession {
    let approval_ticket_ref = (entry.descriptor.approval_posture_class != "no_approval_required")
        .then(|| "approval-ticket:fixture:01".to_string());
    let preview_record_ref = attempt
        .preview_shown
        .then(|| "preview:fixture:01".to_string());
    CommandInvocationSession {
        invocation_session_id: format!("inv:{}:fixture", entry.descriptor.canonical_verb),
        canonical_command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        issuing_surface: attempt.issuing_surface.clone(),
        authority_class: "user_initiated_local".to_string(),
        alias_used: AliasUsedBlock {
            alias_kind: "canonical".to_string(),
            alias_id: None,
            alias_state: "not_applicable".to_string(),
            resolves_to_canonical_command_id: entry.descriptor.command_id.clone(),
            migration_trace_ref: None,
            support_window_ref: None,
        },
        argument_provenance_map: argument_provenance_map_for(entry),
        context_snapshot: InvocationContextSnapshot {
            focused_entity_ref: Some("shell-zone:main_workspace".to_string()),
            selection_ref: None,
            workspace_trust_state: "trusted".to_string(),
            execution_context_id: entry.descriptor.policy_context.execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref: "basis:fixture:01".to_string(),
        },
        context_refs: ContextRefsBlock {
            focused_entity_ref: Some("shell-zone:main_workspace".to_string()),
            selection_ref: None,
            workspace_ref: None,
            workspace_trust_state: "trusted".to_string(),
            execution_context_id: entry.descriptor.policy_context.execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref: "basis:fixture:01".to_string(),
            context_object_refs: Vec::new(),
        },
        enablement_decision: EnablementDecisionBlock {
            decision_class: EnablementDecisionClass::Enabled,
            disabled_reason_code: None,
            repair_hook_ref: None,
        },
        preview_posture: PreviewPostureBlock {
            preview_class_declared: entry.descriptor.preview_class.clone(),
            preview_shown: attempt.preview_shown,
            preview_record_ref,
        },
        approval_posture: ApprovalPostureBlock {
            approval_posture_class_declared: entry.descriptor.approval_posture_class.clone(),
            approval_state: attempt.approval_state.clone(),
            approval_ticket_ref,
        },
        execution_intent: attempt.execution_intent.clone(),
        policy_context: entry.descriptor.policy_context.clone(),
        redaction_class: entry.descriptor.redaction_class.clone(),
    }
}
