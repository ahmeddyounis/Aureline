use std::fs;
use std::path::Path;

use aureline_continuity::{
    admit_deferred_intent, replay_decision, seeded_connectivity_continuity_page,
    validate_connectivity_continuity_page, AuthScopeSnapshot, ConnectivityContinuityPage,
    ConnectivityState, DriftDimension, QueueAdmissionOutcome, ReplayOutcome,
    ReplayRevalidationInput, TargetIdentity,
};

#[test]
fn seeded_page_covers_required_states_and_validates() {
    let page = seeded_connectivity_continuity_page();
    assert!(validate_connectivity_continuity_page(&page));

    for state in [
        ConnectivityState::Connected,
        ConnectivityState::Constrained,
        ConnectivityState::OfflineLocalSafe,
        ConnectivityState::ReauthRequired,
        ConnectivityState::ReconciliationPending,
        ConnectivityState::ServiceUnavailable,
    ] {
        assert!(page.covered_states().contains(&state), "missing {state:?}");
    }
}

#[test]
fn queue_admission_allows_only_explicit_idempotent_reviewable_intent() {
    let page = seeded_connectivity_continuity_page();
    let queueable = page
        .command_declarations
        .iter()
        .find(|declaration| declaration.command_id == "cmd:provider.review_comment.save_draft")
        .expect("queueable command present");
    assert_eq!(
        admit_deferred_intent(queueable).outcome,
        QueueAdmissionOutcome::Queued
    );

    let git_push = page
        .command_declarations
        .iter()
        .find(|declaration| declaration.command_id == "cmd:git.push")
        .expect("git push declaration present");
    assert_eq!(
        admit_deferred_intent(git_push).outcome,
        QueueAdmissionOutcome::BlockedNeverQueue
    );
}

#[test]
fn replay_blocks_on_target_policy_auth_region_endpoint_version_and_context_drift() {
    let page = seeded_connectivity_continuity_page();
    let intent = page.deferred_intents.first().expect("intent present");
    let revalidation = ReplayRevalidationInput {
        current_target_identity: TargetIdentity {
            target_ref: "provider:github:pr:42:comment-thread:9".to_string(),
            target_class: "pull_request_comment_thread".to_string(),
            tenant_ref: "org:other".to_string(),
            region_ref: "region:eu".to_string(),
            endpoint_ref: "endpoint:github.enterprise.example".to_string(),
            version_ref: "head:def456".to_string(),
        },
        current_auth_scope: AuthScopeSnapshot {
            subject_ref: "subject:alice".to_string(),
            scope_refs: vec!["pull_request:read".to_string()],
            auth_epoch: "auth-epoch-18".to_string(),
        },
        current_policy_epoch: "policy-epoch-43".to_string(),
        entitlement_current: false,
        current_service_family: intent.service_family,
        current_context_hash: "sha256:current-context".to_string(),
        command_metadata_complete: true,
        expired: false,
    };

    let decision = replay_decision(intent, &revalidation);
    assert_eq!(
        decision.outcome,
        ReplayOutcome::ReconciliationReviewRequired
    );
    for dimension in [
        DriftDimension::Policy,
        DriftDimension::Auth,
        DriftDimension::Tenant,
        DriftDimension::Region,
        DriftDimension::Endpoint,
        DriftDimension::Target,
        DriftDimension::Version,
        DriftDimension::Entitlement,
        DriftDimension::Context,
    ] {
        assert!(
            decision.drift_dimensions.contains(&dimension),
            "missing {dimension:?}"
        );
    }
}

#[test]
fn replay_allows_only_when_lineage_matches() {
    let page = seeded_connectivity_continuity_page();
    let intent = page.deferred_intents.first().expect("intent present");
    let revalidation = ReplayRevalidationInput {
        current_target_identity: intent.target_identity.clone(),
        current_auth_scope: intent.auth_scope.clone(),
        current_policy_epoch: intent.policy_epoch.clone(),
        entitlement_current: true,
        current_service_family: intent.service_family,
        current_context_hash: intent.context_hash.clone(),
        command_metadata_complete: true,
        expired: false,
    };

    assert_eq!(
        replay_decision(intent, &revalidation).outcome,
        ReplayOutcome::ReplayAllowed
    );
}

#[test]
fn fixture_page_round_trips_and_export_stays_redacted() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/continuity/m4/connectivity-state-and-deferred-intent/page.json");
    let text = fs::read_to_string(path).expect("read fixture");
    let page: ConnectivityContinuityPage = serde_json::from_str(&text).expect("parse fixture");
    assert!(validate_connectivity_continuity_page(&page));

    let exported = serde_json::to_string(&page.support_export).expect("serialize support export");
    assert!(page.support_export.raw_sensitive_payloads_excluded);
    assert!(!exported.contains("raw_token"));
    assert!(!exported.contains("raw_payload"));
}

#[test]
fn schema_file_is_valid_json() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/continuity/deferred-intent-and-reconciliation.schema.json");
    let text = fs::read_to_string(path).expect("read schema");
    serde_json::from_str::<serde_json::Value>(&text).expect("parse schema");
}
