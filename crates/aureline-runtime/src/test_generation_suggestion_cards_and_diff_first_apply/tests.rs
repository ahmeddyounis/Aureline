use super::*;

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn target(target_id: &str) -> TargetReference {
    TargetReference {
        target_id: target_id.to_owned(),
        file_ref: format!("src/{target_id}.rs"),
        symbol_label: format!("fn {target_id}"),
        target_fingerprint_token: format!("fingerprint:{target_id}"),
    }
}

fn evidence(evidence_kind: EvidenceObjectKind, evidence_ref: &str) -> EvidenceReference {
    EvidenceReference {
        evidence_kind,
        evidence_ref: evidence_ref.to_owned(),
        evidence_fingerprint_token: format!("fingerprint:{evidence_ref}"),
        summary: format!("evidence {evidence_ref}"),
    }
}

fn assumption(summary: &str) -> AssumptionEntry {
    AssumptionEntry {
        summary: summary.to_owned(),
        requires_confirmation: true,
    }
}

fn generated_file(file_ref: &str) -> GeneratedFileEntry {
    GeneratedFileEntry {
        file_ref: file_ref.to_owned(),
        change_kind: GeneratedFileKind::NewTestFile,
        diff_summary_ref: format!("diff:{file_ref}"),
    }
}

fn applied_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:applied".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:applied".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:applied".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::UncoveredCoveragePath,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target("applied")],
        evidence_basis: vec![evidence(EvidenceObjectKind::CoverageOverlay, "overlay:1")],
        assumptions: vec![assumption("the path is reachable")],
        generated_files: vec![generated_file("tests/applied.rs")],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::SandboxValidatedPass,
            sandbox_run_ref: Some("sandbox:applied".to_owned()),
            tests_executed: 2,
            tests_passed: 2,
            isolated: true,
            summary: "passed in sandbox".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Applied,
            preview_first: true,
            diff_ref: "diff:applied".to_owned(),
            revert_ref: Some("revert:applied".to_owned()),
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: Some("session:rerun:applied".to_owned()),
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
        support_summary: "applied card".to_owned(),
    }
}

fn blocked_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:blocked".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:blocked[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:blocked".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::BugReproduction,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target("blocked")],
        evidence_basis: vec![
            evidence(EvidenceObjectKind::BugReport, "bug:1"),
            evidence(EvidenceObjectKind::DiagnosticRecord, "diag:1"),
        ],
        assumptions: vec![assumption("reproduces the bug")],
        generated_files: vec![generated_file("tests/blocked.rs")],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::NotValidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "not validated yet".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::BlockedNeedsValidation,
            preview_first: true,
            diff_ref: "diff:blocked".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
        support_summary: "blocked card".to_owned(),
    }
}

fn previewed_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:previewed".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:previewed".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:previewed".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::UncoveredBranchPath,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target("previewed")],
        evidence_basis: vec![evidence(EvidenceObjectKind::SessionPlan, "session:1")],
        assumptions: vec![assumption("branch reachable")],
        generated_files: vec![generated_file("tests/previewed.rs")],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::SandboxValidatedFail,
            sandbox_run_ref: Some("sandbox:previewed".to_owned()),
            tests_executed: 2,
            tests_passed: 1,
            isolated: true,
            summary: "one case failed".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Previewed,
            preview_first: true,
            diff_ref: "diff:previewed".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
        support_summary: "previewed card".to_owned(),
    }
}

fn imported_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:imported".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:imported".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:imported".to_owned(),
            identity_class: TestItemIdentityClass::ImportedReadOnly,
        },
        source_kind: GenerationSourceKind::RegressionGuard,
        provenance: ProposalProvenance::ImportedProposal,
        targets: vec![target("imported")],
        evidence_basis: vec![evidence(EvidenceObjectKind::AttemptRecord, "attempt:1")],
        assumptions: vec![assumption("env stable")],
        generated_files: vec![generated_file("tests/imported.rs")],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::ImportedUnvalidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "imported, not local".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::PendingPreview,
            preview_first: true,
            diff_ref: "diff:imported".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: Some("provider:ci".to_owned()),
        follow_on_rerun_ref: None,
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
        support_summary: "imported card".to_owned(),
    }
}

fn widening_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:widening".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:widening[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:widening".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::ChangedCodeGap,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target("widening")],
        evidence_basis: vec![evidence(
            EvidenceObjectKind::DiscoverySnapshot,
            "discovery:1",
        )],
        assumptions: vec![assumption("changed path only")],
        generated_files: vec![generated_file("tests/widening.rs")],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::NotValidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "routed to review".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Rejected,
            preview_first: true,
            diff_ref: "diff:widening".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: true,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
        support_summary: "widening card".to_owned(),
    }
}

fn guardrails() -> TestGenerationGuardrails {
    TestGenerationGuardrails {
        disclosure_before_apply: true,
        evidence_bound_not_free_text: true,
        sandbox_validated_before_apply: true,
        preview_diff_apply_revert_parity: true,
        no_silent_scope_widening: true,
        imported_never_reads_as_local: true,
        template_invocation_distinct: true,
    }
}

fn consumer_projection() -> TestGenerationConsumerProjection {
    TestGenerationConsumerProjection {
        suggestion_card_ui_normalized: true,
        diff_apply_pipeline_normalized: true,
        evidence_reopen_normalized: true,
        rerun_diagnose_normalized: true,
        release_support_export_normalized: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        TEST_GENERATION_SCHEMA_REF,
        TEST_GENERATION_DOC_REF,
        TEST_GENERATION_ARTIFACT_REF,
    ])
}

fn valid_packet() -> TestGenerationProposalPacket {
    TestGenerationProposalPacket::new(TestGenerationProposalPacketInput {
        packet_id: "packet:test".to_owned(),
        label: "test".to_owned(),
        cards: vec![
            applied_card(),
            blocked_card(),
            previewed_card(),
            imported_card(),
            widening_card(),
        ],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-14T00:00:00Z".to_owned(),
    })
}

#[test]
fn valid_packet_validates() {
    let packet = valid_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn export_round_trips() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: TestGenerationProposalPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, packet);
}

#[test]
fn requires_uncovered_and_bug_sources() {
    let mut packet = valid_packet();
    packet.cards.retain(|c| !c.source_kind.is_bug_source());
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::SourceKindCaseMissing));
}

#[test]
fn requires_validated_and_unvalidated_postures() {
    let mut packet = valid_packet();
    packet
        .cards
        .retain(|c| c.sandbox_validation.posture != ValidationPosture::SandboxValidatedPass);
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ValidationPostureCaseMissing));
}

#[test]
fn requires_applied_and_blocked_states() {
    let mut packet = valid_packet();
    packet
        .cards
        .retain(|c| c.apply_posture.state != ApplyState::BlockedNeedsValidation);
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ApplyStateCaseMissing));
}

#[test]
fn requires_an_imported_proposal() {
    let mut packet = valid_packet();
    packet.cards.retain(|c| !c.is_imported());
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ImportedProposalCaseMissing));
}

#[test]
fn requires_a_widening_guard_case() {
    let mut packet = valid_packet();
    for card in &mut packet.cards {
        card.apply_posture.widens_beyond_evidence = false;
    }
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::WideningGuardCaseMissing));
}

#[test]
fn templates_and_invocations_stay_distinct() {
    let mut packet = valid_packet();
    for card in &mut packet.cards {
        if card.subject.node_kind == DurableTestNodeKind::ParameterizedTemplate {
            card.subject.node_kind = DurableTestNodeKind::ConcreteInvocation;
        }
    }
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn applied_card_requires_sandbox_pass() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:applied")
        .unwrap();
    card.sandbox_validation.posture = ValidationPosture::SandboxValidationPending;
    let violations = packet.validate();
    assert!(violations.contains(&TestGenerationViolation::AppliedWithoutSandboxValidation));
}

#[test]
fn applied_card_cannot_bypass_preview() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:applied")
        .unwrap();
    card.apply_posture.preview_first = false;
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::AppliedBypassesPreview));
}

#[test]
fn applied_card_cannot_widen_beyond_evidence() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:applied")
        .unwrap();
    card.apply_posture.widens_beyond_evidence = true;
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::AppliedWidensBeyondEvidence));
}

#[test]
fn applied_card_requires_rerun_linkage() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:applied")
        .unwrap();
    card.follow_on_rerun_ref = None;
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::AppliedWithoutRerunLinkage));
}

#[test]
fn imported_card_cannot_be_applied_locally() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:imported")
        .unwrap();
    card.apply_posture.state = ApplyState::Applied;
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ImportedReadsAsLocal));
}

#[test]
fn imported_card_must_carry_imported_markers() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:imported")
        .unwrap();
    card.origin_provider_ref = None;
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ImportedReadsAsLocal));
}

#[test]
fn apply_path_requires_disclosure() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:previewed")
        .unwrap();
    card.assumptions.clear();
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::ApplyPathWithoutDisclosure));
}

#[test]
fn evidence_must_be_reopenable() {
    let mut packet = valid_packet();
    let card = packet
        .cards
        .iter_mut()
        .find(|c| c.card_id == "card:blocked")
        .unwrap();
    card.evidence_basis.clear();
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::EvidenceNotReopenable));
}

#[test]
fn evidence_fingerprint_cannot_substitute_for_ref() {
    let mut packet = valid_packet();
    let card = &mut packet.cards[0];
    card.evidence_basis[0].evidence_fingerprint_token = card.evidence_basis[0].evidence_ref.clone();
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::EvidenceNotReopenable));
}

#[test]
fn subject_fingerprint_cannot_substitute_for_id() {
    let mut packet = valid_packet();
    let card = &mut packet.cards[0];
    card.subject.subject_fingerprint_token = card.subject.subject_id.clone();
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = valid_packet();
    packet.source_contract_refs = refs(&[TEST_GENERATION_SCHEMA_REF]);
    assert!(packet
        .validate()
        .contains(&TestGenerationViolation::MissingSourceContracts));
}

#[test]
fn counts_and_helpers_reflect_packet() {
    let packet = valid_packet();
    assert_eq!(packet.imported_card_count(), 1);
    assert_eq!(packet.applied_card_count(), 1);
    assert_eq!(packet.blocked_card_count(), 1);
    assert_eq!(
        packet.card("card:applied").unwrap().generated_file_count(),
        1
    );
}

#[test]
fn markdown_summary_renders_rows() {
    let packet = valid_packet();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("Test-Generation Suggestion Cards And Diff-First Apply"));
    assert!(summary.contains("card:applied"));
    assert!(summary.contains("card:imported"));
    assert!(summary.contains("card:widening"));
}
