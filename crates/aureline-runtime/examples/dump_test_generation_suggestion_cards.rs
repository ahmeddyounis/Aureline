//! Conformance dump for the M5 test-generation proposal packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the sandbox-gate drill fixture (`fixture` argument) so
//! the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::test_generation_suggestion_cards_and_diff_first_apply::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;

const PACKET_ID: &str = "test-generation:stable:0001";
const MINTED_AT: &str = "2026-06-14T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn target(target_id: &str, file_ref: &str, symbol_label: &str) -> TargetReference {
    TargetReference {
        target_id: target_id.to_owned(),
        file_ref: file_ref.to_owned(),
        symbol_label: symbol_label.to_owned(),
        target_fingerprint_token: format!("fingerprint:{target_id}"),
    }
}

fn evidence(
    evidence_kind: EvidenceObjectKind,
    evidence_ref: &str,
    summary: &str,
) -> EvidenceReference {
    EvidenceReference {
        evidence_kind,
        evidence_ref: evidence_ref.to_owned(),
        evidence_fingerprint_token: format!("fingerprint:{evidence_ref}"),
        summary: summary.to_owned(),
    }
}

fn assumption(summary: &str, requires_confirmation: bool) -> AssumptionEntry {
    AssumptionEntry {
        summary: summary.to_owned(),
        requires_confirmation,
    }
}

fn generated_file(file_ref: &str, change_kind: GeneratedFileKind) -> GeneratedFileEntry {
    GeneratedFileEntry {
        file_ref: file_ref.to_owned(),
        change_kind,
        diff_summary_ref: format!("diff:{file_ref}"),
    }
}

/// A locally generated test for an uncovered line path, sandbox-validated and applied
/// through the preview-first diff pipeline with a follow-on rerun link.
fn applied_uncovered_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:applied:uncovered-line".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:auth::login_rejects_expired_token".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:login-expired".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::UncoveredCoveragePath,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target(
            "target:auth-login",
            "crates/aureline-auth/src/login.rs",
            "fn verify_token",
        )],
        evidence_basis: vec![evidence(
            EvidenceObjectKind::CoverageOverlay,
            "overlay:verified:auth",
            "The expired-token branch of verify_token is uncovered in the verified run.",
        )],
        assumptions: vec![assumption(
            "An expired token is represented by a past `exp` claim relative to the run clock.",
            true,
        )],
        generated_files: vec![generated_file(
            "crates/aureline-auth/tests/login_expired.rs",
            GeneratedFileKind::NewTestFile,
        )],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::SandboxValidatedPass,
            sandbox_run_ref: Some("sandbox:run:auth-1".to_owned()),
            tests_executed: 3,
            tests_passed: 3,
            isolated: true,
            summary: "All 3 generated cases passed in an isolated sandbox.".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Applied,
            preview_first: true,
            diff_ref: "diff:card:applied:uncovered-line".to_owned(),
            revert_ref: Some("revert:card:applied:uncovered-line".to_owned()),
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: Some("session:local:rerun-auth-expired".to_owned()),
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Generated test for the uncovered expired-token branch; sandbox-validated and applied preview-first."
                .to_owned(),
    }
}

/// A bug-reproduction proposal whose apply is blocked until sandbox validation
/// succeeds.
fn blocked_bug_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:blocked:bug-repro".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:checkout::tax_rounding[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:tax-rounding".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::BugReproduction,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target(
            "target:checkout-tax",
            "crates/aureline-commerce/src/tax.rs",
            "fn round_tax",
        )],
        evidence_basis: vec![
            evidence(
                EvidenceObjectKind::BugReport,
                "bug:checkout-4821",
                "Reported rounding error on half-cent tax totals.",
            ),
            evidence(
                EvidenceObjectKind::DiagnosticRecord,
                "diagnostic:checkout-tax-fail",
                "Failing assertion captured on the half-cent input.",
            ),
        ],
        assumptions: vec![
            assumption("Tax is rounded half-up at two decimal places.", true),
            assumption("The reported total uses minor currency units.", false),
        ],
        generated_files: vec![generated_file(
            "crates/aureline-commerce/tests/tax_rounding.rs",
            GeneratedFileKind::NewTestFile,
        )],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::NotValidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "Not yet run in the sandbox; apply is blocked until validation.".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::BlockedNeedsValidation,
            preview_first: true,
            diff_ref: "diff:card:blocked:bug-repro".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Bug-reproduction template bound to the report and diagnostic; apply blocked pending sandbox validation."
                .to_owned(),
    }
}

/// A previewed branch-coverage proposal whose sandbox run failed, so it offers an
/// apply path for review but can never be accepted as a passing test.
fn previewed_branch_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:previewed:branch".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:render::pipeline_partial_branch".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:pipeline-branch".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::UncoveredBranchPath,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target(
            "target:render-pipeline",
            "crates/aureline-render/src/pipeline.rs",
            "fn schedule_pass",
        )],
        evidence_basis: vec![
            evidence(
                EvidenceObjectKind::CoverageOverlay,
                "overlay:verified:checkout-branch",
                "The error arm of schedule_pass is a partially covered branch.",
            ),
            evidence(
                EvidenceObjectKind::SessionPlan,
                "session:plan:render",
                "Session plan that surfaced the partial-branch gap.",
            ),
        ],
        assumptions: vec![assumption(
            "The error arm is reachable with a malformed pass descriptor.",
            true,
        )],
        generated_files: vec![
            generated_file(
                "crates/aureline-render/tests/pipeline_branch.rs",
                GeneratedFileKind::NewTestFile,
            ),
            generated_file(
                "crates/aureline-render/tests/fixtures/malformed_pass.json",
                GeneratedFileKind::NewFixtureFile,
            ),
        ],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::SandboxValidatedFail,
            sandbox_run_ref: Some("sandbox:run:render-1".to_owned()),
            tests_executed: 2,
            tests_passed: 1,
            isolated: true,
            summary: "1 of 2 generated cases failed in the sandbox; cannot be applied as passing."
                .to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Previewed,
            preview_first: true,
            diff_ref: "diff:card:previewed:branch".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Branch-gap proposal previewed for review; its sandbox run failed, so it is not accept-eligible."
                .to_owned(),
    }
}

/// An imported proposal held read-only: it is never sandbox-validated locally and can
/// never be applied as a local result.
fn imported_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:imported:smoke".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:imported::smoke_regression".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:imported-smoke".to_owned(),
            identity_class: TestItemIdentityClass::ImportedReadOnly,
        },
        source_kind: GenerationSourceKind::RegressionGuard,
        provenance: ProposalProvenance::ImportedProposal,
        targets: vec![target(
            "target:imported-smoke",
            "crates/aureline-shell/src/startup.rs",
            "fn boot",
        )],
        evidence_basis: vec![evidence(
            EvidenceObjectKind::AttemptRecord,
            "attempt:ci:smoke-7741",
            "Imported attempt record that motivated the regression guard.",
        )],
        assumptions: vec![assumption(
            "Boot order is stable across the imported provider's environment.",
            true,
        )],
        generated_files: vec![generated_file(
            "crates/aureline-shell/tests/smoke_regression.rs",
            GeneratedFileKind::NewTestFile,
        )],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::ImportedUnvalidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "Imported proposal; not locally validated and held read-only.".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::PendingPreview,
            preview_first: true,
            diff_ref: "diff:card:imported:smoke".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: false,
        },
        origin_provider_ref: Some("provider:ci-smoke".to_owned()),
        follow_on_rerun_ref: None,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Imported regression-guard proposal held read-only; never a local apply."
            .to_owned(),
    }
}

/// A proposal that would widen the test scope beyond its evidenced basis; it is routed
/// to review and may never be applied while it widens.
fn widening_review_card() -> TestGenerationSuggestionCard {
    TestGenerationSuggestionCard {
        card_id: "card:review:widening".to_owned(),
        subject: GeneratedTestSubject {
            subject_id: "test:data::serialize_cases[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:serialize-cases".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        source_kind: GenerationSourceKind::ChangedCodeGap,
        provenance: ProposalProvenance::LocallyGenerated,
        targets: vec![target(
            "target:data-serialize",
            "crates/aureline-data/src/serialize.rs",
            "fn encode",
        )],
        evidence_basis: vec![evidence(
            EvidenceObjectKind::DiscoverySnapshot,
            "discovery:snapshot:data",
            "Changed encode() in the diff has no test in the discovery snapshot.",
        )],
        assumptions: vec![assumption(
            "Only the changed encode path needs a new test.",
            true,
        )],
        generated_files: vec![generated_file(
            "crates/aureline-data/tests/serialize_cases.rs",
            GeneratedFileKind::NewTestFile,
        )],
        sandbox_validation: SandboxValidation {
            posture: ValidationPosture::NotValidated,
            sandbox_run_ref: None,
            tests_executed: 0,
            tests_passed: 0,
            isolated: false,
            summary: "Routed to review: the proposal exceeds the evidenced changed scope.".to_owned(),
        },
        apply_posture: ApplyPosture {
            state: ApplyState::Rejected,
            preview_first: true,
            diff_ref: "diff:card:review:widening".to_owned(),
            revert_ref: None,
            widens_beyond_evidence: true,
        },
        origin_provider_ref: None,
        follow_on_rerun_ref: None,
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Proposal that widens beyond the evidenced changed scope; rejected rather than silently applied."
                .to_owned(),
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
        TEST_GENERATION_SUMMARY_REF,
    ])
}

fn cards() -> Vec<TestGenerationSuggestionCard> {
    vec![
        applied_uncovered_card(),
        blocked_bug_card(),
        previewed_branch_card(),
        imported_card(),
        widening_review_card(),
    ]
}

fn packet() -> TestGenerationProposalPacket {
    TestGenerationProposalPacket::new(TestGenerationProposalPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Test-Generation Suggestion Cards And Diff-First Apply".to_owned(),
        cards: cards(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

/// Builds the sandbox-gate drill fixture: an unvalidated bug-reproduction proposal
/// stays blocked from apply, an imported proposal is held read-only, and a widening
/// proposal is rejected rather than silently applied, while a sandbox-validated
/// proposal applies through the preview-first diff pipeline.
fn sandbox_gate_fixture() -> TestGenerationProposalPacket {
    TestGenerationProposalPacket::new(TestGenerationProposalPacketInput {
        packet_id: "test-generation:fixture:sandbox-gate".to_owned(),
        label: "Sandbox gate blocks apply".to_owned(),
        cards: cards(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    let packet = if which == "fixture" {
        sandbox_gate_fixture()
    } else {
        packet()
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
