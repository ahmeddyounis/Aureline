//! Conformance dump for the M5 session-plan / attempt-record ledger packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::session_plans_attempt_records_and_execution_lineage::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;

const PACKET_ID: &str = "session-attempt-ledger:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn target(
    target_id: &str,
    node_kind: DurableTestNodeKind,
    identity_class: TestItemIdentityClass,
) -> LedgerTarget {
    LedgerTarget {
        target_id: target_id.to_owned(),
        node_kind,
        target_fingerprint_token: format!("fingerprint:{target_id}"),
        identity_class,
    }
}

fn local_target(target_id: &str, node_kind: DurableTestNodeKind) -> LedgerTarget {
    target(target_id, node_kind, TestItemIdentityClass::Stable)
}

fn local_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:cpython-3.12".to_owned(),
        toolchain_token: "toolchain:pytest-8".to_owned(),
        env_capsule_token: "env:workspace-venv".to_owned(),
        target_class: TargetClass::LocalProcess,
        provenance_class: LineageProvenanceClass::LocalAuthoritative,
        host_ref: None,
        provider_token: None,
        imported: false,
    }
}

fn remote_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:cpython-3.12".to_owned(),
        toolchain_token: "toolchain:pytest-8".to_owned(),
        env_capsule_token: "env:remote-builder".to_owned(),
        target_class: TargetClass::RemoteHost,
        provenance_class: LineageProvenanceClass::RemoteAuthoritative,
        host_ref: Some("host:remote-builder-a".to_owned()),
        provider_token: None,
        imported: false,
    }
}

fn notebook_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:ipykernel-6".to_owned(),
        toolchain_token: "toolchain:nbval".to_owned(),
        env_capsule_token: "env:notebook-kernel".to_owned(),
        target_class: TargetClass::NotebookKernel,
        provenance_class: LineageProvenanceClass::NotebookAuthoritative,
        host_ref: None,
        provider_token: None,
        imported: false,
    }
}

fn imported_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:provider-reported".to_owned(),
        toolchain_token: "toolchain:provider-reported".to_owned(),
        env_capsule_token: "env:provider-reported".to_owned(),
        target_class: TargetClass::ProviderBackend,
        provenance_class: LineageProvenanceClass::ImportedReadOnly,
        host_ref: None,
        provider_token: Some("provider:ci-smoke".to_owned()),
        imported: true,
    }
}

/// Local workspace session: a rerun-failed plan over the checkout suite, keeping a
/// parameterized template distinct from its two concrete invocations.
fn local_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:local:checkout".to_owned(),
        plan_id: "plan:local:checkout".to_owned(),
        label: "Local rerun-failed of the checkout suite".to_owned(),
        flow: SessionFlow::LocalWorkspace,
        mode: SessionPlanMode::RerunFailed,
        selection_ref: "selection:cli:rerun-failed".to_owned(),
        snapshot_ref: "snapshot:framework-pack:checkout".to_owned(),
        execution_context_ref: "execution-context:local:checkout".to_owned(),
        lineage: local_lineage(),
        retry_policy: RetryPolicyClass::RetryFailedUpToLimit,
        watch_policy: WatchPolicyClass::WatchDisabled,
        targets: vec![
            local_target("framework:case:add-item", DurableTestNodeKind::ConcreteCase),
            local_target(
                "framework:template:totals",
                DurableTestNodeKind::ParameterizedTemplate,
            ),
            local_target(
                "framework:invocation:totals:usd",
                DurableTestNodeKind::ConcreteInvocation,
            ),
            local_target(
                "framework:invocation:totals:eur",
                DurableTestNodeKind::ConcreteInvocation,
            ),
        ],
        evidence_refs: refs(&["evidence:session:local:checkout"]),
    }
}

/// Remote target session: a watch-armed run on a remote builder.
fn remote_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:remote:integration".to_owned(),
        plan_id: "plan:remote:integration".to_owned(),
        label: "Remote integration run on the shared builder".to_owned(),
        flow: SessionFlow::RemoteTarget,
        mode: SessionPlanMode::RunSelected,
        selection_ref: "selection:ui:integration".to_owned(),
        snapshot_ref: "snapshot:framework-pack:integration".to_owned(),
        execution_context_ref: "execution-context:remote:integration".to_owned(),
        lineage: remote_lineage(),
        retry_policy: RetryPolicyClass::NoRetry,
        watch_policy: WatchPolicyClass::WatchDebounced,
        targets: vec![local_target(
            "framework:case:checkout-flow",
            DurableTestNodeKind::ConcreteCase,
        )],
        evidence_refs: refs(&["evidence:session:remote:integration"]),
    }
}

/// Notebook session: a notebook-linked test run inside a kernel.
fn notebook_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:notebook:analysis".to_owned(),
        plan_id: "plan:notebook:analysis".to_owned(),
        label: "Notebook-linked analysis test run".to_owned(),
        flow: SessionFlow::NotebookKernel,
        mode: SessionPlanMode::RunSelected,
        selection_ref: "selection:ui:notebook".to_owned(),
        snapshot_ref: "snapshot:notebook:analysis".to_owned(),
        execution_context_ref: "execution-context:notebook:analysis".to_owned(),
        lineage: notebook_lineage(),
        retry_policy: RetryPolicyClass::NoRetry,
        watch_policy: WatchPolicyClass::WatchDisabled,
        targets: vec![local_target(
            "notebook:test:analysis",
            DurableTestNodeKind::NotebookLinkedTest,
        )],
        evidence_refs: refs(&["evidence:session:notebook:analysis"]),
    }
}

/// Imported-provider session: a CI failure join held read-only.
fn imported_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:imported:smoke".to_owned(),
        plan_id: "plan:imported:smoke".to_owned(),
        label: "Imported CI smoke evidence joined for triage".to_owned(),
        flow: SessionFlow::ImportedProvider,
        mode: SessionPlanMode::ImportProviderJoin,
        selection_ref: "selection:support:imported-ci".to_owned(),
        snapshot_ref: "snapshot:imported-ci:smoke".to_owned(),
        execution_context_ref: "execution-context:imported:smoke".to_owned(),
        lineage: imported_lineage(),
        retry_policy: RetryPolicyClass::ImportedNoRetry,
        watch_policy: WatchPolicyClass::ImportedNotWatchable,
        targets: vec![target(
            "ci:case:smoke",
            DurableTestNodeKind::ConcreteCase,
            TestItemIdentityClass::ImportedReadOnly,
        )],
        evidence_refs: refs(&["evidence:session:imported:smoke"]),
    }
}

fn attempt(
    attempt_id: &str,
    session_ref: &str,
    index: u32,
    kind: AttemptKind,
    outcome: AttemptOutcome,
    lineage: ExecutionLineage,
    covered: &[&str],
) -> AttemptRecord {
    let flow = lineage
        .agreed_flow()
        .expect("lineage target class and provenance agree on a flow");
    let target_class = lineage.target_class;
    AttemptRecord {
        attempt_id: attempt_id.to_owned(),
        session_ref: session_ref.to_owned(),
        attempt_index: index,
        kind,
        outcome,
        flow,
        target_class,
        lineage,
        predecessor_attempt_ref: None,
        origin_provider_ref: None,
        execution_attempt_ref: Some(format!("execution-attempt:{attempt_id}")),
        covered_target_ids: refs(covered),
        artifact_refs: refs(&[&format!("artifact:{attempt_id}")]),
        evidence_refs: refs(&[&format!("evidence:{attempt_id}")]),
        support_summary: format!("Attempt {attempt_id} recorded on the session ledger."),
    }
}

fn ledger_attempts() -> Vec<AttemptRecord> {
    // Local session: an initial failed run, then an append-only failed-only rerun.
    let local_initial = attempt(
        "attempt:local:checkout:1",
        "session:local:checkout",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Failed,
        local_lineage(),
        &["framework:invocation:totals:eur"],
    );
    let mut local_rerun = attempt(
        "attempt:local:checkout:2",
        "session:local:checkout",
        2,
        AttemptKind::RerunFailed,
        AttemptOutcome::Passed,
        local_lineage(),
        &["framework:invocation:totals:eur"],
    );
    local_rerun.predecessor_attempt_ref = Some("attempt:local:checkout:1".to_owned());

    // Remote session: a single passing initial run.
    let remote_initial = attempt(
        "attempt:remote:integration:1",
        "session:remote:integration",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Passed,
        remote_lineage(),
        &["framework:case:checkout-flow"],
    );

    // Notebook session: a single passing notebook-linked run.
    let notebook_initial = attempt(
        "attempt:notebook:analysis:1",
        "session:notebook:analysis",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Passed,
        notebook_lineage(),
        &["notebook:test:analysis"],
    );

    // Imported session: a provider CI failure joined read-only, then a local
    // parity rerun in the SAME ledger that carries local lineage so the imported
    // verdict never reads as a local rerun.
    let mut imported_join = attempt(
        "attempt:imported:smoke:1",
        "session:imported:smoke",
        1,
        AttemptKind::ImportedJoin,
        AttemptOutcome::Imported,
        imported_lineage(),
        &["ci:case:smoke"],
    );
    imported_join.origin_provider_ref = Some("provider-run:ci-smoke:4821".to_owned());
    imported_join.execution_attempt_ref = None;

    let mut local_parity = attempt(
        "attempt:imported:smoke:2",
        "session:imported:smoke",
        2,
        AttemptKind::LocalParityRerun,
        AttemptOutcome::Failed,
        local_lineage(),
        &["ci:case:smoke"],
    );
    local_parity.predecessor_attempt_ref = Some("attempt:imported:smoke:1".to_owned());

    vec![
        local_initial,
        local_rerun,
        remote_initial,
        notebook_initial,
        imported_join,
        local_parity,
    ]
}

fn guardrails() -> LedgerGuardrails {
    LedgerGuardrails {
        templates_distinct_from_invocations: true,
        imported_never_local_rerun: true,
        lineage_preserved_per_attempt: true,
        attempt_history_append_only: true,
        no_green_over_quarantine_or_stale: true,
        session_reopenable_from_export: true,
    }
}

fn consumer_projection() -> LedgerConsumerProjection {
    LedgerConsumerProjection {
        notifications_reopen_session: true,
        support_export_reopens_attempt: true,
        review_packet_reopens_session: true,
        release_gate_reads_ledger: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        SESSION_ATTEMPT_LEDGER_SCHEMA_REF,
        SESSION_ATTEMPT_LEDGER_DOC_REF,
        SESSION_ATTEMPT_LEDGER_ARTIFACT_REF,
        "schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json",
    ])
}

fn packet() -> SessionAttemptLedgerPacket {
    SessionAttemptLedgerPacket::new(SessionAttemptLedgerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Session Plans And Attempt-Record Ledger".to_owned(),
        sessions: vec![
            local_session(),
            remote_session(),
            notebook_session(),
            imported_session(),
        ],
        attempts: ledger_attempts(),
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

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
