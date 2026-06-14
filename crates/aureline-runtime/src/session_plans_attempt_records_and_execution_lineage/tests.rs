use super::*;

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

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
        runtime_token: "runtime:local".to_owned(),
        toolchain_token: "toolchain:local".to_owned(),
        env_capsule_token: "env:local".to_owned(),
        target_class: TargetClass::LocalProcess,
        provenance_class: LineageProvenanceClass::LocalAuthoritative,
        host_ref: None,
        provider_token: None,
        imported: false,
    }
}

fn remote_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:remote".to_owned(),
        toolchain_token: "toolchain:remote".to_owned(),
        env_capsule_token: "env:remote".to_owned(),
        target_class: TargetClass::RemoteHost,
        provenance_class: LineageProvenanceClass::RemoteAuthoritative,
        host_ref: Some("host:remote".to_owned()),
        provider_token: None,
        imported: false,
    }
}

fn notebook_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:notebook".to_owned(),
        toolchain_token: "toolchain:notebook".to_owned(),
        env_capsule_token: "env:notebook".to_owned(),
        target_class: TargetClass::NotebookKernel,
        provenance_class: LineageProvenanceClass::NotebookAuthoritative,
        host_ref: None,
        provider_token: None,
        imported: false,
    }
}

fn imported_lineage() -> ExecutionLineage {
    ExecutionLineage {
        runtime_token: "runtime:imported".to_owned(),
        toolchain_token: "toolchain:imported".to_owned(),
        env_capsule_token: "env:imported".to_owned(),
        target_class: TargetClass::ProviderBackend,
        provenance_class: LineageProvenanceClass::ImportedReadOnly,
        host_ref: None,
        provider_token: Some("provider:ci".to_owned()),
        imported: true,
    }
}

fn local_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:local".to_owned(),
        plan_id: "plan:local".to_owned(),
        label: "Local rerun-failed".to_owned(),
        flow: SessionFlow::LocalWorkspace,
        mode: SessionPlanMode::RerunFailed,
        selection_ref: "selection:local".to_owned(),
        snapshot_ref: "snapshot:local".to_owned(),
        execution_context_ref: "ctx:local".to_owned(),
        lineage: local_lineage(),
        retry_policy: RetryPolicyClass::RetryFailedUpToLimit,
        watch_policy: WatchPolicyClass::WatchDisabled,
        targets: vec![
            local_target("case:add", DurableTestNodeKind::ConcreteCase),
            local_target(
                "template:totals",
                DurableTestNodeKind::ParameterizedTemplate,
            ),
            local_target("invocation:usd", DurableTestNodeKind::ConcreteInvocation),
        ],
        evidence_refs: refs(&["evidence:session:local"]),
    }
}

fn remote_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:remote".to_owned(),
        plan_id: "plan:remote".to_owned(),
        label: "Remote run".to_owned(),
        flow: SessionFlow::RemoteTarget,
        mode: SessionPlanMode::RunSelected,
        selection_ref: "selection:remote".to_owned(),
        snapshot_ref: "snapshot:remote".to_owned(),
        execution_context_ref: "ctx:remote".to_owned(),
        lineage: remote_lineage(),
        retry_policy: RetryPolicyClass::NoRetry,
        watch_policy: WatchPolicyClass::WatchDebounced,
        targets: vec![local_target("case:flow", DurableTestNodeKind::ConcreteCase)],
        evidence_refs: refs(&["evidence:session:remote"]),
    }
}

fn notebook_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:notebook".to_owned(),
        plan_id: "plan:notebook".to_owned(),
        label: "Notebook run".to_owned(),
        flow: SessionFlow::NotebookKernel,
        mode: SessionPlanMode::RunSelected,
        selection_ref: "selection:notebook".to_owned(),
        snapshot_ref: "snapshot:notebook".to_owned(),
        execution_context_ref: "ctx:notebook".to_owned(),
        lineage: notebook_lineage(),
        retry_policy: RetryPolicyClass::NoRetry,
        watch_policy: WatchPolicyClass::WatchDisabled,
        targets: vec![local_target(
            "notebook:test",
            DurableTestNodeKind::NotebookLinkedTest,
        )],
        evidence_refs: refs(&["evidence:session:notebook"]),
    }
}

fn imported_session() -> SessionPlan {
    SessionPlan {
        session_id: "session:imported".to_owned(),
        plan_id: "plan:imported".to_owned(),
        label: "Imported CI join".to_owned(),
        flow: SessionFlow::ImportedProvider,
        mode: SessionPlanMode::ImportProviderJoin,
        selection_ref: "selection:imported".to_owned(),
        snapshot_ref: "snapshot:imported".to_owned(),
        execution_context_ref: "ctx:imported".to_owned(),
        lineage: imported_lineage(),
        retry_policy: RetryPolicyClass::ImportedNoRetry,
        watch_policy: WatchPolicyClass::ImportedNotWatchable,
        targets: vec![target(
            "ci:case:smoke",
            DurableTestNodeKind::ConcreteCase,
            TestItemIdentityClass::ImportedReadOnly,
        )],
        evidence_refs: refs(&["evidence:session:imported"]),
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
        execution_attempt_ref: Some(format!("exec:{attempt_id}")),
        covered_target_ids: refs(covered),
        artifact_refs: refs(&[&format!("artifact:{attempt_id}")]),
        evidence_refs: refs(&[&format!("evidence:{attempt_id}")]),
        support_summary: format!("Attempt {attempt_id}."),
    }
}

fn attempts() -> Vec<AttemptRecord> {
    let local_initial = attempt(
        "attempt:local:1",
        "session:local",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Failed,
        local_lineage(),
        &["invocation:usd"],
    );
    let mut local_rerun = attempt(
        "attempt:local:2",
        "session:local",
        2,
        AttemptKind::RerunFailed,
        AttemptOutcome::Passed,
        local_lineage(),
        &["invocation:usd"],
    );
    local_rerun.predecessor_attempt_ref = Some("attempt:local:1".to_owned());

    let remote_initial = attempt(
        "attempt:remote:1",
        "session:remote",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Passed,
        remote_lineage(),
        &["case:flow"],
    );

    let notebook_initial = attempt(
        "attempt:notebook:1",
        "session:notebook",
        1,
        AttemptKind::Initial,
        AttemptOutcome::Passed,
        notebook_lineage(),
        &["notebook:test"],
    );

    let mut imported_join = attempt(
        "attempt:imported:1",
        "session:imported",
        1,
        AttemptKind::ImportedJoin,
        AttemptOutcome::Imported,
        imported_lineage(),
        &["ci:case:smoke"],
    );
    imported_join.origin_provider_ref = Some("provider-run:4821".to_owned());
    imported_join.execution_attempt_ref = None;

    let mut local_parity = attempt(
        "attempt:imported:2",
        "session:imported",
        2,
        AttemptKind::LocalParityRerun,
        AttemptOutcome::Failed,
        local_lineage(),
        &["ci:case:smoke"],
    );
    local_parity.predecessor_attempt_ref = Some("attempt:imported:1".to_owned());

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
    ])
}

fn valid_packet() -> SessionAttemptLedgerPacket {
    SessionAttemptLedgerPacket::new(SessionAttemptLedgerPacketInput {
        packet_id: "session-attempt-ledger:test".to_owned(),
        label: "Test ledger".to_owned(),
        sessions: vec![
            local_session(),
            remote_session(),
            notebook_session(),
            imported_session(),
        ],
        attempts: attempts(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn valid_packet_has_no_violations() {
    let packet = valid_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn all_flows_and_required_modes_are_represented() {
    let packet = valid_packet();
    assert_eq!(packet.represented_flows().len(), SessionFlow::ALL.len());
    for mode in [
        SessionPlanMode::RunSelected,
        SessionPlanMode::RerunFailed,
        SessionPlanMode::ImportProviderJoin,
    ] {
        assert!(packet.represented_modes().contains(&mode));
    }
}

#[test]
fn session_attempts_are_ordered_by_index() {
    let packet = valid_packet();
    let local = packet.session_attempts("session:local");
    assert_eq!(local.len(), 2);
    assert_eq!(local[0].attempt_index, 1);
    assert_eq!(local[1].attempt_index, 2);
}

#[test]
fn round_trips_through_export_json() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: SessionAttemptLedgerPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn imported_attempt_carries_read_only_lineage_and_origin() {
    let packet = valid_packet();
    let imported = packet
        .attempt("attempt:imported:1")
        .expect("imported attempt");
    assert!(imported.is_imported_attempt());
    assert_eq!(
        imported.lineage.provenance_class,
        LineageProvenanceClass::ImportedReadOnly
    );
    assert!(imported.origin_provider_ref.is_some());
    assert!(imported.outcome.is_imported_outcome());
}

#[test]
fn local_parity_rerun_stays_local_truth() {
    let packet = valid_packet();
    let parity = packet
        .attempt("attempt:imported:2")
        .expect("parity attempt");
    // A local parity rerun is recorded on the imported session ledger, but it
    // carries local lineage and a local outcome — it is never an imported verdict.
    assert!(!parity.is_imported_attempt());
    assert_eq!(parity.flow, SessionFlow::LocalWorkspace);
    assert_eq!(
        parity.lineage.provenance_class,
        LineageProvenanceClass::LocalAuthoritative
    );
    assert_eq!(
        parity.predecessor_attempt_ref.as_deref(),
        Some("attempt:imported:1")
    );
}

#[test]
fn imported_attempt_with_passing_outcome_is_rejected() {
    let mut packet = valid_packet();
    // Force the imported join to read like a local pass.
    let join = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:imported:1")
        .unwrap();
    join.outcome = AttemptOutcome::Passed;
    let violations = packet.validate();
    assert!(
        violations.contains(&SessionAttemptLedgerViolation::ImportedAttemptReadsAsLocal)
            || violations.contains(&SessionAttemptLedgerViolation::GreenOverStaleOrUnknown)
            || violations.contains(&SessionAttemptLedgerViolation::AttemptInvalid),
        "an imported attempt must not read as a local pass: {violations:?}"
    );
}

#[test]
fn imported_attempt_without_origin_ref_is_rejected() {
    let mut packet = valid_packet();
    let join = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:imported:1")
        .unwrap();
    join.origin_provider_ref = None;
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::ImportedAttemptReadsAsLocal));
}

#[test]
fn local_attempt_with_imported_origin_ref_is_rejected() {
    let mut packet = valid_packet();
    let initial = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:local:1")
        .unwrap();
    initial.origin_provider_ref = Some("provider-run:smuggled".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::ImportedAttemptReadsAsLocal));
}

#[test]
fn lineage_target_class_must_match_attempt_target_class() {
    let mut packet = valid_packet();
    let remote = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:remote:1")
        .unwrap();
    remote.target_class = TargetClass::LocalProcess;
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::AttemptLineageInconsistent));
}

#[test]
fn imported_session_with_local_retry_policy_is_rejected() {
    let mut packet = valid_packet();
    let session = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "session:imported")
        .unwrap();
    session.retry_policy = RetryPolicyClass::RetryUntilStable;
    let violations = packet.validate();
    assert!(
        violations.contains(&SessionAttemptLedgerViolation::SessionImportedMarkersInconsistent)
            || violations.contains(&SessionAttemptLedgerViolation::SessionInvalid)
    );
}

#[test]
fn target_fingerprint_cannot_substitute_id() {
    let mut packet = valid_packet();
    let session = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "session:local")
        .unwrap();
    session.targets[0].target_fingerprint_token = session.targets[0].target_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn template_and_invocation_kinds_must_both_appear() {
    let mut packet = valid_packet();
    // Drop the parameterized template everywhere so only invocations remain.
    for session in &mut packet.sessions {
        session
            .targets
            .retain(|t| t.node_kind != DurableTestNodeKind::ParameterizedTemplate);
    }
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn non_contiguous_attempt_indices_are_rejected() {
    let mut packet = valid_packet();
    let rerun = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:local:2")
        .unwrap();
    rerun.attempt_index = 5;
    rerun.predecessor_attempt_ref = Some("attempt:local:1".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::AttemptIndicesNotContiguous));
}

#[test]
fn predecessor_must_resolve_to_earlier_attempt_in_same_session() {
    let mut packet = valid_packet();
    let rerun = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:local:2")
        .unwrap();
    rerun.predecessor_attempt_ref = Some("attempt:remote:1".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::PredecessorChainInvalid));
}

#[test]
fn rerun_kind_requires_a_predecessor() {
    let mut packet = valid_packet();
    let rerun = packet
        .attempts
        .iter_mut()
        .find(|a| a.attempt_id == "attempt:local:2")
        .unwrap();
    rerun.predecessor_attempt_ref = None;
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::AttemptInvalid));
}

#[test]
fn attempt_referencing_unknown_session_is_rejected() {
    let mut packet = valid_packet();
    packet.attempts[0].session_ref = "session:ghost".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::AttemptSessionUnresolved));
}

#[test]
fn attempt_covering_a_target_outside_its_session_is_rejected() {
    let mut packet = valid_packet();
    packet.attempts[0]
        .covered_target_ids
        .push("case:not-in-session".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::AttemptTargetUnresolved));
}

#[test]
fn missing_required_flow_is_rejected() {
    let mut packet = valid_packet();
    packet
        .sessions
        .retain(|s| s.flow != SessionFlow::NotebookKernel);
    packet
        .attempts
        .retain(|a| a.session_ref != "session:notebook");
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::FlowCoverageMissing));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = valid_packet();
    packet.source_contract_refs = refs(&["schemas/testing/unrelated.schema.json"]);
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_are_rejected() {
    let mut packet = valid_packet();
    packet.guardrails.imported_never_local_rerun = false;
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::GuardrailsIncomplete));
}

#[test]
fn raw_boundary_material_in_export_is_rejected() {
    let mut packet = valid_packet();
    packet.attempts[0].support_summary = "contains api_key leak".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&SessionAttemptLedgerViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_sessions_and_attempts() {
    let packet = valid_packet();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("session:local"));
    assert!(summary.contains("attempt #2"));
    assert!(summary.contains("imported_provider"));
}
