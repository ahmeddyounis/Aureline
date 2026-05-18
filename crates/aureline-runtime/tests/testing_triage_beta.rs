use std::path::{Path, PathBuf};

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, FlakyVerdictState, IdentityMode,
    ImportedCiProjectionClass, PytestDiscoverer, PytestDiscovererConfig,
    PythonEnvironmentDetectorConfig, ScopeClass, SnapshotFileChangePreview, SnapshotMutationReview,
    SnapshotMutationReviewState, TargetClass, TestAttemptAlphaPacket, TestEvidenceTrustClass,
    TestQualityProjection, TestQuarantineReason, TestQuarantineRecord,
    TestQuarantineReopenBehavior, TestQuarantineScopeClass, TestQuarantineStatus,
    TestQuarantineTreatmentKind, TestReleaseDebtClass, TestRunnerBetaProjection,
    TestRunnerBetaSupportExport, TestTriageIdentity, TestTrustPacket, TestWatchState, TrustState,
    WatchModeDowngradeReason, WatchModeState, FLAKY_VERDICT_PACKET_RECORD_KIND,
    SNAPSHOT_MUTATION_REVIEW_RECORD_KIND, TEST_QUARANTINE_RECORD_KIND,
    TEST_TRIAGE_TRUST_SCHEMA_VERSION, TEST_TRUST_PACKET_RECORD_KIND,
    WATCH_STATE_PACKET_RECORD_KIND,
};
use aureline_runtime::{PytestDiscovery, PytestRunContract};

fn ready_uv_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/python_task_discovery_alpha/ready_uv")
}

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "workspace:python".to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace/python".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:workspace:python".to_owned(),
            capsule_hash: "sha256:workspace:python".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "test-triage-beta-it-resolver".to_owned(),
    })
}

fn pytest_discoverer() -> PytestDiscoverer {
    PytestDiscoverer::new(PytestDiscovererConfig {
        python_detector: PythonEnvironmentDetectorConfig {
            ambient_python_version: Some("3.12.7".to_owned()),
            ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
            ambient_uv_version: Some("0.5.7".to_owned()),
            ambient_poetry_version: Some("1.8.4".to_owned()),
            ..PythonEnvironmentDetectorConfig::default()
        },
        workspace_revision: Some("rev:python".to_owned()),
    })
}

fn discovery_and_context(time_prefix: &str) -> (PytestDiscovery, ExecutionContext) {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        &format!("{time_prefix}:00Z"),
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        &format!("{time_prefix}:01Z"),
    );
    (discovery, context)
}

fn projection_with_attempts(
    discovery: &PytestDiscovery,
    context: &ExecutionContext,
    attempt_packets: Vec<TestAttemptAlphaPacket>,
    time_prefix: &str,
) -> (TestRunnerBetaProjection, TestQualityProjection) {
    let runner = TestRunnerBetaProjection::from_pytest_discovery(
        discovery,
        context,
        attempt_packets,
        None,
        None,
        &format!("{time_prefix}:03Z"),
    );
    let quality = TestQualityProjection::from_beta_projection(&runner);
    (runner, quality)
}

fn first_contract(discovery: &PytestDiscovery) -> PytestRunContract {
    discovery
        .run_contracts
        .iter()
        .find(|contract| contract.selection.test_item_id.is_some())
        .expect("pytest case contract")
        .clone()
}

fn first_identity(quality: &TestQualityProjection) -> TestTriageIdentity {
    TestTriageIdentity::from_quality_row(
        quality
            .row_truths
            .first()
            .expect("quality row for first pytest case"),
    )
}

#[test]
fn watch_state_packets_use_release_vocabulary_and_preserve_downgrade_reason() {
    let (discovery, context) = discovery_and_context("2026-05-18T10:00");
    let attempt_packets = discovery
        .run_contracts
        .iter()
        .map(|contract| {
            TestAttemptAlphaPacket::from_pytest_watch_contract(
                contract,
                &context,
                TestWatchState::Degraded,
                "2026-05-18T10:00:02Z",
            )
        })
        .collect();
    let (runner, quality) =
        projection_with_attempts(&discovery, &context, attempt_packets, "2026-05-18T10:00");

    let packet = TestTrustPacket::from_projection(
        &runner,
        &quality,
        Vec::new(),
        Vec::new(),
        "2026-05-18T10:00:05Z",
    );

    assert_eq!(packet.record_kind, TEST_TRUST_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, TEST_TRIAGE_TRUST_SCHEMA_VERSION);
    assert!(packet.watch_degraded_row_count > 0);
    for watch in &packet.watch_state_packets {
        assert_eq!(watch.record_kind, WATCH_STATE_PACKET_RECORD_KIND);
        assert_eq!(watch.watch_state, WatchModeState::Reduced);
        assert_eq!(watch.watch_state_token, "reduced");
        assert!(watch
            .downgrade_reasons
            .contains(&WatchModeDowngradeReason::PartialDiscovery));
        assert!(watch
            .downgrade_reason_tokens
            .contains(&"partial_discovery".to_owned()));
        assert!(!watch.current_truth_claim_allowed);
        assert!(watch.reconnect_preserves_state);
    }

    let rendered = packet.render_plaintext();
    assert!(rendered.contains("Watch degraded rows:"));
    assert!(rendered.contains("watch=reduced"));
}

#[test]
fn flaky_verdict_packet_traces_attempt_history_targets_and_environment() {
    let (discovery, context) = discovery_and_context("2026-05-18T10:10");
    let target = first_contract(&discovery);
    let attempt_packets = discovery
        .run_contracts
        .iter()
        .map(|contract| {
            if contract.attempt_id == target.attempt_id {
                TestAttemptAlphaPacket::imported_ci_failure_projection(
                    contract,
                    &context,
                    "provider-run:pytest:failure-window",
                    "rerun-plan:pytest:fresh-local",
                    "2026-05-18T10:10:02Z",
                )
            } else {
                TestAttemptAlphaPacket::from_pytest_contract(
                    contract,
                    &context,
                    "2026-05-18T10:10:02Z",
                )
            }
        })
        .collect();
    let (runner, quality) =
        projection_with_attempts(&discovery, &context, attempt_packets, "2026-05-18T10:10");

    let packet = TestTrustPacket::from_projection(
        &runner,
        &quality,
        Vec::new(),
        Vec::new(),
        "2026-05-18T10:10:05Z",
    );
    let target_item = target
        .selection
        .test_item_id
        .as_deref()
        .expect("target test item");
    let flaky = packet
        .flaky_verdict_packets
        .iter()
        .find(|flaky| flaky.identity.canonical_test_item_ref == target_item)
        .expect("flaky packet for imported failure projection");

    assert_eq!(flaky.record_kind, FLAKY_VERDICT_PACKET_RECORD_KIND);
    assert_eq!(flaky.verdict_state, FlakyVerdictState::StableAgain);
    assert_eq!(flaky.observation_window_attempts, 3);
    assert_eq!(flaky.evidence_attempts.len(), 3);
    assert!(flaky
        .evidence_attempts
        .iter()
        .all(|attempt| !attempt.execution_context_ref.is_empty()));
    assert!(flaky
        .evidence_attempts
        .iter()
        .all(|attempt| !attempt.target_id.is_empty()));
    assert!(flaky.evidence_attempts.iter().any(|attempt| {
        attempt.imported_ci_projection_token
            == ImportedCiProjectionClass::FreshLocalReconfirmation.as_str()
    }));
}

#[test]
fn snapshot_mutations_cannot_land_without_preview_and_grouped_rollback() {
    let (discovery, context) = discovery_and_context("2026-05-18T10:20");
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-18T10:20:02Z");
    let (runner, quality) =
        projection_with_attempts(&discovery, &context, attempt_packets, "2026-05-18T10:20");
    let identity = first_identity(&quality);

    let blocked = SnapshotMutationReview::new(
        "snapshot-review:blocked",
        identity.clone(),
        SnapshotMutationReviewState::ReviewReady,
        "actor:test-owner",
        Vec::new(),
        None,
        true,
        Vec::new(),
        "Snapshot mutation lacks the review evidence needed to land.",
    );
    assert_eq!(blocked.record_kind, SNAPSHOT_MUTATION_REVIEW_RECORD_KIND);
    assert!(!blocked.can_land);
    assert_eq!(
        blocked.review_state,
        SnapshotMutationReviewState::BlockedMissingPreview
    );
    assert!(blocked
        .denial_reason_tokens
        .contains(&"missing_file_change_preview".to_owned()));
    assert!(blocked
        .denial_reason_tokens
        .contains(&"missing_grouped_rollback_checkpoint".to_owned()));

    let ready = SnapshotMutationReview::new(
        "snapshot-review:ready",
        identity,
        SnapshotMutationReviewState::ReviewReady,
        "actor:test-owner",
        vec![SnapshotFileChangePreview {
            preview_id: "preview:snapshot:1".to_owned(),
            subject_ref: "snapshot:file:health".to_owned(),
            before_artifact_ref: "artifact:snapshot:before".to_owned(),
            after_artifact_ref: "artifact:snapshot:after".to_owned(),
            change_class_token: "artifact_updated_minor".to_owned(),
            preview_artifact_ref: "artifact:snapshot:preview".to_owned(),
            summary: "One snapshot artifact changes with rendered preview.".to_owned(),
        }],
        Some("rollback:snapshot-group:1".to_owned()),
        true,
        vec!["policy-hook:release-bearing-snapshot".to_owned()],
        "Snapshot mutation has preview, actor, rollback, and policy hook.",
    );
    assert!(ready.can_land);
    assert_eq!(ready.review_state, SnapshotMutationReviewState::ReviewReady);

    let packet = TestTrustPacket::from_projection(
        &runner,
        &quality,
        vec![blocked],
        Vec::new(),
        "2026-05-18T10:20:05Z",
    );
    assert_eq!(packet.snapshot_mutation_count, 1);
    assert_eq!(packet.release_blocking_debt_count, 1);
}

#[test]
fn expired_quarantine_records_reopen_and_stay_release_visible() {
    let (discovery, context) = discovery_and_context("2026-05-18T10:30");
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-18T10:30:02Z");
    let (runner, quality) =
        projection_with_attempts(&discovery, &context, attempt_packets, "2026-05-18T10:30");
    let identity = first_identity(&quality);
    let record = TestQuarantineRecord::active(
        "test-quarantine:expired",
        TestQuarantineTreatmentKind::Quarantine,
        "owner:test-tooling",
        TestQuarantineReason::ReproducedFlaky,
        TestQuarantineScopeClass::ExactTestItem,
        vec![identity.canonical_test_item_ref.clone()],
        "2026-05-17T00:00:00Z",
        "2026-05-18T09:00:00Z",
        vec!["test-attempt:evidence:1".to_owned()],
        TestQuarantineReopenBehavior::ReopenFailure,
        TestReleaseDebtClass::ClaimNarrowingRequired,
        "Known flaky test is quarantined with owner and expiry.",
    );
    assert_eq!(record.record_kind, TEST_QUARANTINE_RECORD_KIND);

    let reopened = record.evaluated_at("2026-05-18T10:30:05Z", Some("attempt:next".to_owned()));
    assert_eq!(reopened.status, TestQuarantineStatus::ExpiredReopened);
    assert!(reopened.release_visible);
    assert_eq!(
        reopened.release_debt_class,
        TestReleaseDebtClass::ReleaseBlocking
    );

    let packet = TestTrustPacket::from_projection(
        &runner,
        &quality,
        Vec::new(),
        vec![record],
        "2026-05-18T10:30:05Z",
    );
    assert_eq!(packet.expired_reopened_count, 1);
    assert_eq!(packet.quarantined_scope_count, 1);
    assert_eq!(packet.release_blocking_debt_count, 1);
    assert!(packet
        .row_summaries
        .iter()
        .any(|row| row.row_debt_tokens.contains(&"expired_reopened".to_owned())));
}

#[test]
fn trust_packet_summarizes_mute_quarantine_imported_only_and_watch_degradation() {
    let (discovery, context) = discovery_and_context("2026-05-18T10:40");
    let target = first_contract(&discovery);
    let target_attempt_id = target.attempt_id.clone();
    let target_item = target
        .selection
        .test_item_id
        .clone()
        .expect("target test item");
    let attempt_packets = discovery
        .run_contracts
        .iter()
        .map(|contract| {
            if contract.attempt_id == target_attempt_id {
                let mut packet = TestAttemptAlphaPacket::imported_ci_failure_projection(
                    contract,
                    &context,
                    "provider-run:pytest:imported-only",
                    "rerun-plan:pytest:fresh-local",
                    "2026-05-18T10:40:02Z",
                );
                packet.attempts.truncate(1);
                let imported_attempt_ref = packet.attempts[0].test_attempt_id.clone();
                packet.watch_controller.latest_attempt_ref = Some(imported_attempt_ref.clone());
                packet.watch_controller.last_successful_attempt_ref = None;
                packet.stability_verdict.evidence_attempt_refs = vec![imported_attempt_ref];
                packet.stability_verdict.verdict_state = FlakyVerdictState::SuspectedFlaky;
                packet.stability_verdict.verdict_state_token =
                    FlakyVerdictState::SuspectedFlaky.as_str().to_owned();
                packet
            } else {
                TestAttemptAlphaPacket::from_pytest_watch_contract(
                    contract,
                    &context,
                    TestWatchState::Stale,
                    "2026-05-18T10:40:02Z",
                )
            }
        })
        .collect();
    let (runner, quality) =
        projection_with_attempts(&discovery, &context, attempt_packets, "2026-05-18T10:40");

    let mute = TestQuarantineRecord::active(
        "test-mute:imported-row",
        TestQuarantineTreatmentKind::Mute,
        "owner:test-tooling",
        TestQuarantineReason::ManualNoiseReduction,
        TestQuarantineScopeClass::ExactTestItem,
        vec![target_item.clone()],
        "2026-05-18T09:30:00Z",
        "2026-05-19T09:30:00Z",
        vec!["test-attempt:mute:evidence".to_owned()],
        TestQuarantineReopenBehavior::ManualOwnerReview,
        TestReleaseDebtClass::WaiverLinked,
        "Manual mute remains visible in release packets.",
    );
    let packet = TestTrustPacket::from_projection(
        &runner,
        &quality,
        Vec::new(),
        vec![mute],
        "2026-05-18T10:40:05Z",
    );

    assert!(packet.watch_degraded_row_count > 0);
    assert_eq!(packet.imported_only_row_count, 1);
    assert_eq!(packet.active_mute_record_count, 1);
    let imported_row = packet
        .row_summaries
        .iter()
        .find(|row| row.identity.canonical_test_item_ref == target_item)
        .expect("imported row summary");
    assert_eq!(
        imported_row.evidence_trust_class,
        TestEvidenceTrustClass::ImportedOnly
    );
    assert!(imported_row
        .row_debt_tokens
        .contains(&"imported_only_evidence".to_owned()));
    assert!(packet.watch_state_packets.iter().any(|watch| {
        watch.identity.canonical_test_item_ref == target_item
            && watch.watch_state == WatchModeState::ImportedOnly
            && watch
                .downgrade_reasons
                .contains(&WatchModeDowngradeReason::ImportedReadOnlyEvidence)
    }));

    let json = serde_json::to_string(&packet).expect("serialize trust packet");
    let round: TestTrustPacket = serde_json::from_str(&json).expect("deserialize trust packet");
    assert_eq!(round, packet);

    let rendered = packet.render_plaintext();
    assert!(rendered.contains("Imported-only rows: 1"));
    assert!(rendered.contains("Active mute records: 1"));

    let mut support_export = TestRunnerBetaSupportExport::from_projection(
        &runner,
        "test-runner-beta:canonical",
        "2026-05-18T10:40:06Z",
    );
    support_export.attach_test_trust_packet_ref(packet.trust_packet_id.clone());
    let support_plaintext = support_export.render_plaintext();
    assert!(support_plaintext.contains("Test trust packet:"));
    assert!(support_plaintext.contains(&packet.trust_packet_id));
}
