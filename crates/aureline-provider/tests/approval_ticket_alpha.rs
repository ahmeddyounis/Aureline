//! Fixture-driven coverage for approval-ticket alpha records.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    ApprovalActorClass, ApprovalAuthorityKind, ApprovalTicketAlphaPacket, HighRiskActionClass,
    NativeReapprovalRoute, TicketEvaluationOutcome,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/approval_ticket_alpha/baseline_packet.json")
}

fn load_packet() -> ApprovalTicketAlphaPacket {
    let text = fs::read_to_string(fixture_path()).expect("read approval ticket alpha fixture");
    serde_json::from_str(&text).expect("parse approval ticket alpha fixture")
}

#[test]
fn baseline_packet_covers_authority_and_fail_closed_states() {
    let packet = load_packet();
    let report = packet.validate();

    assert!(
        report.passed,
        "approval ticket alpha fixture failed validation: {:#?}",
        report.findings
    );
    for actor_class in [
        ApprovalActorClass::HumanAccount,
        ApprovalActorClass::InstallationOrAppGrant,
        ApprovalActorClass::DelegatedCredential,
        ApprovalActorClass::LocalOnlyAuthority,
    ] {
        assert!(report.coverage.actor_classes.contains(&actor_class));
    }
    assert!(report
        .coverage
        .authority_kinds
        .contains(&ApprovalAuthorityKind::ApprovalTicket));
    assert!(report
        .coverage
        .authority_kinds
        .contains(&ApprovalAuthorityKind::ReviewedScope));
    assert!(report
        .coverage
        .high_risk_action_classes
        .contains(&HighRiskActionClass::ExternalProviderMutation));
    assert!(report
        .coverage
        .high_risk_action_classes
        .contains(&HighRiskActionClass::HelperBackedRemoteMutation));
    for outcome in [
        TicketEvaluationOutcome::Admitted,
        TicketEvaluationOutcome::DeniedExpired,
        TicketEvaluationOutcome::DeniedTargetDrift,
        TicketEvaluationOutcome::DeniedTrustProfileDrift,
        TicketEvaluationOutcome::DeniedSandboxProfileDrift,
        TicketEvaluationOutcome::DeniedPolicyEpochDrift,
        TicketEvaluationOutcome::DeniedActorScopeMismatch,
    ] {
        assert!(report.coverage.evaluation_outcomes.contains(&outcome));
    }
}

#[test]
fn high_risk_mutation_without_ticket_or_scope_is_rejected() {
    let mut packet = load_packet();
    let binding = packet
        .mutation_bindings
        .iter_mut()
        .find(|binding| binding.mutation_id == "mutation.provider.comment.publish.0001")
        .expect("fixture has provider comment mutation");
    binding.approval_ticket_ref = None;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| { finding.check_id == "approval_ticket_alpha.mutation_authority_missing" }));
}

#[test]
fn expired_or_drifted_ticket_cannot_claim_admission() {
    let mut packet = load_packet();
    let attempt = packet
        .spend_attempts
        .iter_mut()
        .find(|attempt| attempt.spend_attempt_id == "spend.provider.comment.target_drift.0001")
        .expect("fixture has target drift attempt");
    attempt.evaluation_outcome = TicketEvaluationOutcome::Admitted;
    attempt.native_reapproval_route = NativeReapprovalRoute::NotRequired;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "approval_ticket_alpha.spend_expected_outcome_mismatch"
    }));
}

#[test]
fn denied_spend_requires_native_reapproval_route() {
    let mut packet = load_packet();
    let attempt = packet
        .spend_attempts
        .iter_mut()
        .find(|attempt| attempt.spend_attempt_id == "spend.provider.issue.expired.0001")
        .expect("fixture has expired attempt");
    attempt.native_reapproval_route = NativeReapprovalRoute::NotRequired;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.check_id == "approval_ticket_alpha.denied_reapproval_route_missing"
    }));
}

#[test]
fn support_admin_projection_reconstructs_lineage_without_raw_payloads() {
    let packet = load_packet();
    let projection = packet.support_admin_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");

    assert_eq!(
        projection.record_kind,
        "approval_ticket_support_admin_packet"
    );
    assert!(!json.contains("raw_url"));
    assert!(!json.contains("token"));
    assert!(!json.contains("secret"));

    let actor_classes: BTreeSet<ApprovalActorClass> = projection
        .lineage_summaries
        .iter()
        .map(|summary| summary.actor_class)
        .collect();
    assert!(actor_classes.contains(&ApprovalActorClass::HumanAccount));
    assert!(actor_classes.contains(&ApprovalActorClass::InstallationOrAppGrant));
    assert!(actor_classes.contains(&ApprovalActorClass::DelegatedCredential));
    assert!(actor_classes.contains(&ApprovalActorClass::LocalOnlyAuthority));
    assert!(projection
        .lineage_summaries
        .iter()
        .all(|summary| !summary.actor_lineage.is_empty()));
}
