//! Conformance dump for the M5 quality-session ledger packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::m5_quality_action_proposals_and_sessions::*;
use aureline_runtime::{
    QualityActionClass, QualityActionProposal, QualityActionProposalRequest,
    QualityMutationScopeClass, QualitySafetyClass, QualitySession, QualitySessionRequest,
    QualitySessionTriggerClass, QualityTargetScopeClass,
};

const PACKET_ID: &str = "m5-quality-session-ledger:stable:0001";
const WORKSPACE_ID: &str = "workspace:m5:quality-actions";
const PROFILE_REF: &str = "effective-profile:m5:quality:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Builds one quality-action proposal from pre-run facts. Defaults model a single
/// local, semantically-current mutation; callers override only what differs.
#[allow(clippy::too_many_arguments)]
fn proposal(
    id: &str,
    action_class: QualityActionClass,
    target_scope_class: QualityTargetScopeClass,
    mutation_scope_class: QualityMutationScopeClass,
    safety_class: QualitySafetyClass,
    affected_file_count: usize,
    generated_path_count: usize,
    protected_path_count: usize,
    policy_lock_refs: &[&str],
    profile_policy_locked: bool,
    checkpoint_ref: Option<&str>,
    summary: &str,
) -> QualityActionProposal {
    QualityActionProposal::from_request(QualityActionProposalRequest {
        proposal_id: id.to_owned(),
        action_class,
        target_scope_class,
        mutation_scope_class,
        safety_class,
        effective_profile_ref: PROFILE_REF.to_owned(),
        triggering_finding_refs: refs(&[&format!("finding:{id}")]),
        rule_refs: refs(&[&format!("rule:{id}")]),
        policy_lock_refs: refs(policy_lock_refs),
        affected_file_count,
        affected_anchor_count: affected_file_count.max(1),
        generated_path_count,
        protected_path_count,
        blocked_path_count: 0,
        semantic_current: true,
        profile_policy_locked,
        checkpoint_ref: checkpoint_ref.map(str::to_owned),
        preview_ref: Some(format!("preview:{id}")),
        revert_plan_ref: Some(format!("revert-plan:{id}")),
        validation_refs: refs(&[&format!("validation:{id}")]),
        summary: summary.to_owned(),
    })
}

#[allow(clippy::too_many_arguments)]
fn session(
    id: &str,
    trigger_class: QualitySessionTriggerClass,
    target_scope_class: QualityTargetScopeClass,
    execution_context_ref: Option<&str>,
    proposals: Vec<QualityActionProposal>,
    validation_refs: &[&str],
    rollback_refs: &[&str],
    summary: &str,
) -> QualitySession {
    QualitySession::from_request(QualitySessionRequest {
        session_id: id.to_owned(),
        trigger_class,
        target_scope_class,
        effective_profile_ref: PROFILE_REF.to_owned(),
        execution_context_ref: execution_context_ref.map(str::to_owned),
        started_at: MINTED_AT.to_owned(),
        ended_at: Some(MINTED_AT.to_owned()),
        proposals,
        validation_refs: refs(validation_refs),
        rollback_refs: refs(rollback_refs),
        summary: summary.to_owned(),
    })
}

/// On-type, as-you-edit format pass over a notebook cell selection — trivia-safe
/// and auto-applied, yet still a typed proposal serialized in a session.
fn on_type_session() -> QualitySession {
    session(
        "session:m5:on-type:notebook:0001",
        QualitySessionTriggerClass::OnType,
        QualityTargetScopeClass::CurrentSelection,
        Some("execution-context:m5:notebook"),
        vec![proposal(
            "proposal:on-type:format-range",
            QualityActionClass::FormatRange,
            QualityTargetScopeClass::CurrentSelection,
            QualityMutationScopeClass::SingleFileLocalized,
            QualitySafetyClass::TriviaSafe,
            1,
            0,
            0,
            &[],
            false,
            None,
            "Format the edited notebook-cell range as you type.",
        )],
        &["validation:on-type:reparse"],
        &[],
        "On-type formatting auto-applied a trivia-safe range edit in a notebook cell.",
    )
}

/// On-save participant pass over a framework source file — whole-document format
/// plus organize-imports, both routed through preview-first.
fn on_save_session() -> QualitySession {
    session(
        "session:m5:on-save:framework:0001",
        QualitySessionTriggerClass::OnSave,
        QualityTargetScopeClass::CurrentFile,
        Some("execution-context:m5:framework"),
        vec![
            proposal(
                "proposal:on-save:format-document",
                QualityActionClass::FormatDocument,
                QualityTargetScopeClass::CurrentFile,
                QualityMutationScopeClass::SingleFileWholeDocument,
                QualitySafetyClass::LocalSyntaxSafe,
                1,
                0,
                0,
                &[],
                false,
                Some("checkpoint:on-save:format-document"),
                "Format the whole framework source file on save.",
            ),
            proposal(
                "proposal:on-save:organize-imports",
                QualityActionClass::OrganizeImports,
                QualityTargetScopeClass::CurrentFile,
                QualityMutationScopeClass::SingleFileLocalized,
                QualitySafetyClass::SemanticLocal,
                1,
                0,
                0,
                &[],
                false,
                Some("checkpoint:on-save:organize-imports"),
                "Organize imports for the framework source file on save.",
            ),
        ],
        &[
            "validation:on-save:format-check",
            "validation:on-save:build",
        ],
        &["revert:on-save:checkpoint"],
        "On-save formatting and organize-imports require a preview before apply.",
    )
}

/// Manual quick-fix plus fix-all-rule over request and data tooling — one local
/// semantic fix and one cross-file rule sweep, both preview-first.
fn manual_session() -> QualitySession {
    session(
        "session:m5:manual:request-data:0001",
        QualitySessionTriggerClass::ManualCommand,
        QualityTargetScopeClass::CurrentRoot,
        Some("execution-context:m5:request"),
        vec![
            proposal(
                "proposal:manual:quick-fix",
                QualityActionClass::QuickFixSingle,
                QualityTargetScopeClass::CurrentSelection,
                QualityMutationScopeClass::SingleAnchor,
                QualitySafetyClass::SemanticLocal,
                1,
                0,
                0,
                &[],
                false,
                Some("checkpoint:manual:quick-fix"),
                "Apply a single quick fix to a request-tooling finding.",
            ),
            proposal(
                "proposal:manual:fix-all-rule",
                QualityActionClass::FixAllRule,
                QualityTargetScopeClass::CurrentRoot,
                QualityMutationScopeClass::MultiFileSameModule,
                QualitySafetyClass::CrossFileSemantic,
                3,
                0,
                0,
                &[],
                false,
                Some("checkpoint:manual:fix-all-rule"),
                "Fix every occurrence of one rule across the data module.",
            ),
        ],
        &["validation:manual:typecheck", "validation:manual:test"],
        &["revert:manual:checkpoint"],
        "Manual quick-fix and fix-all-rule require preview before apply.",
    )
}

/// Headless lint autofix over a package lane — a safe, localized batch the CLI may
/// auto-apply, reported through the same vocabulary as the interactive paths.
fn headless_session() -> QualitySession {
    session(
        "session:m5:headless:package:0001",
        QualitySessionTriggerClass::CliHeadless,
        QualityTargetScopeClass::CurrentFile,
        Some("execution-context:m5:package"),
        vec![proposal(
            "proposal:headless:lint-autofix",
            QualityActionClass::LintAutofixBatch,
            QualityTargetScopeClass::CurrentFile,
            QualityMutationScopeClass::SingleFileLocalized,
            QualitySafetyClass::LocalSyntaxSafe,
            1,
            0,
            0,
            &[],
            false,
            Some("checkpoint:headless:lint-autofix"),
            "Auto-apply a localized lint fix batch in headless mode.",
        )],
        &["validation:headless:lint-recheck"],
        &["revert:headless:checkpoint"],
        "Headless lint autofix auto-applied a localized, syntax-safe batch.",
    )
}

/// Review-apply governance — a suppression renewal and a baseline update, both
/// policy-bearing and blocked pending policy or trust.
fn review_session() -> QualitySession {
    session(
        "session:m5:review:governance:0001",
        QualitySessionTriggerClass::Review,
        QualityTargetScopeClass::BaselineFamily,
        Some("execution-context:m5:review"),
        vec![
            proposal(
                "proposal:review:suppression",
                QualityActionClass::SuppressionProposal,
                QualityTargetScopeClass::BaselineFamily,
                QualityMutationScopeClass::ProtectedOrPolicyScoped,
                QualitySafetyClass::SemanticLocal,
                0,
                0,
                0,
                &["policy:suppression-requires-issue"],
                false,
                None,
                "Renew a governed suppression behind a policy lock.",
            ),
            proposal(
                "proposal:review:baseline-update",
                QualityActionClass::BaselineUpdate,
                QualityTargetScopeClass::BaselineFamily,
                QualityMutationScopeClass::ProtectedOrPolicyScoped,
                QualitySafetyClass::SemanticLocal,
                0,
                0,
                0,
                &[],
                true,
                None,
                "Update a governed baseline under a policy-locked profile.",
            ),
        ],
        &["validation:review:policy-check"],
        &[],
        "Review-apply suppression and baseline updates are blocked pending policy or trust.",
    )
}

/// Imported-scan comparison — read-only scanner and validation recheck that
/// compare an imported snapshot against the local revision; never a local apply.
fn import_comparison_session() -> QualitySession {
    session(
        "session:m5:import-comparison:scanner:0001",
        QualitySessionTriggerClass::ImportComparison,
        QualityTargetScopeClass::Workspace,
        Some("execution-context:m5:import"),
        vec![
            proposal(
                "proposal:import:scanner-read-only",
                QualityActionClass::ScannerReadOnly,
                QualityTargetScopeClass::Workspace,
                QualityMutationScopeClass::NoMutationReadOnly,
                QualitySafetyClass::LocalSyntaxSafe,
                0,
                0,
                0,
                &[],
                false,
                None,
                "Compare an imported scanner snapshot against the local revision.",
            ),
            proposal(
                "proposal:import:validation-recheck",
                QualityActionClass::ValidationRecheck,
                QualityTargetScopeClass::Workspace,
                QualityMutationScopeClass::NoMutationReadOnly,
                QualitySafetyClass::LocalSyntaxSafe,
                0,
                0,
                0,
                &[],
                false,
                None,
                "Re-validate imported findings against the local revision.",
            ),
        ],
        &["validation:import:snapshot-compare"],
        &[],
        "Imported-scan comparison stayed read-only and never read as a local apply.",
    )
}

/// Generated, lockfile, manifest, and protected paths — a regenerated family
/// reused the same preview/apply/validate/revert lifecycle, not a weaker bar.
fn generated_protected_session() -> QualitySession {
    session(
        "session:m5:generated-protected:0001",
        QualitySessionTriggerClass::ManualCommand,
        QualityTargetScopeClass::SelectedWorkset,
        Some("execution-context:m5:generated"),
        vec![proposal(
            "proposal:generated:format-document",
            QualityActionClass::FormatDocument,
            QualityTargetScopeClass::SelectedWorkset,
            QualityMutationScopeClass::GeneratedFamily,
            QualitySafetyClass::GeneratedOrProtected,
            3,
            2,
            1,
            &[],
            false,
            Some("checkpoint:generated:format-document"),
            "Format a regenerated artifact family including a lockfile and manifest.",
        )],
        &["validation:generated:regenerate-check"],
        &["revert:generated:grouped-checkpoint"],
        "A generated, lockfile, and manifest family reused the preview-first lifecycle.",
    )
}

/// Unknown or unstable fix — held for user review rather than silently applied.
fn unknown_unstable_session() -> QualitySession {
    session(
        "session:m5:unknown-unstable:0001",
        QualitySessionTriggerClass::ManualCommand,
        QualityTargetScopeClass::CurrentFile,
        Some("execution-context:m5:unstable"),
        vec![proposal(
            "proposal:unknown:quick-fix",
            QualityActionClass::QuickFixSingle,
            QualityTargetScopeClass::CurrentFile,
            QualityMutationScopeClass::SingleAnchor,
            QualitySafetyClass::UnknownOrUnstable,
            1,
            0,
            0,
            &[],
            false,
            None,
            "A provider-ambiguous quick fix held for user review.",
        )],
        &["validation:unknown:manual-review"],
        &[],
        "An unknown or unstable fix was blocked pending user review, not silently applied.",
    )
}

fn sessions() -> Vec<QualitySession> {
    vec![
        on_type_session(),
        on_save_session(),
        manual_session(),
        headless_session(),
        review_session(),
        import_comparison_session(),
        generated_protected_session(),
        unknown_unstable_session(),
    ]
}

fn guardrails() -> QualityActionGuardrails {
    QualityActionGuardrails {
        every_mutating_action_is_typed_proposal: true,
        every_proposal_serialized_in_session: true,
        one_result_vocabulary_across_paths: true,
        generated_and_protected_reuse_lifecycle: true,
        rollback_notes_inspectable: true,
        validation_refs_inspectable: true,
        safety_classes_inspectable: true,
        import_comparison_stays_read_only: true,
    }
}

fn consumer_projection() -> QualityActionConsumerProjection {
    QualityActionConsumerProjection {
        ui_shows_proposal_and_session: true,
        problems_shows_proposal_and_session: true,
        review_shows_proposal_and_session: true,
        cli_shows_proposal_and_session: true,
        support_export_preserves_sessions: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        QUALITY_ACTION_PROPOSAL_SCHEMA_REF,
        QUALITY_SESSION_SCHEMA_REF,
        M5_QUALITY_SESSION_LEDGER_SCHEMA_REF,
        M5_QUALITY_SESSION_LEDGER_DOC_REF,
        M5_QUALITY_SESSION_LEDGER_ARTIFACT_REF,
        "schemas/quality/effective_quality_profile.schema.json",
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
    ])
}

fn packet() -> QualitySessionLedgerPacket {
    QualitySessionLedgerPacket::new(QualitySessionLedgerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        ledger_label: "M5 Quality-Session Ledger".to_owned(),
        workspace_id: WORKSPACE_ID.to_owned(),
        sessions: sessions(),
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
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );
    assert!(
        packet.coverage.covers_required_trigger_paths(),
        "packet must cover every required trigger path"
    );
    assert!(
        packet.coverage.covers_required_action_classes(),
        "packet must cover every required action class"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
