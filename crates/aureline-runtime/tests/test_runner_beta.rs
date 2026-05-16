use std::path::{Path, PathBuf};

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, PytestDiscoverer, PytestDiscovererConfig,
    PythonEnvironmentDetectorConfig, RerunLane, RerunLastLoop, RerunTargetMode, ScopeClass,
    TargetClass, TestArtifactKind, TestRunnerBetaCoverageManifest, TestRunnerBetaFramework,
    TestRunnerBetaParityState, TestRunnerBetaProjection, TestRunnerBetaSupportExport,
    TestTreeRowKind, TrustState, TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_RUNNER_BETA_SCHEMA_VERSION,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/testing/m3/test_runner")
}

fn ready_uv_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/python_task_discovery_alpha/ready_uv")
}

fn read_fixture(name: &str) -> String {
    let path = fixture_dir().join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
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
        resolver_version: "test-runner-beta-it-resolver".to_owned(),
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

#[test]
fn checked_in_coverage_manifest_matches_canonical_runtime_manifest() {
    let payload = read_fixture("coverage_manifest.json");
    let fixture: TestRunnerBetaCoverageManifest =
        serde_json::from_str(&payload).expect("parse coverage manifest fixture");
    assert_eq!(
        fixture.record_kind,
        TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    assert_eq!(
        fixture.test_attempt_schema_version,
        TEST_RUNNER_BETA_SCHEMA_VERSION
    );
    let canonical = TestRunnerBetaCoverageManifest::canonical(
        fixture.manifest_id.clone(),
        fixture.generated_at.clone(),
    );
    assert_eq!(canonical, fixture);
    assert_eq!(
        canonical.frameworks.len(),
        TestRunnerBetaFramework::ALL.len()
    );
    let pytest = canonical
        .row_for_framework(TestRunnerBetaFramework::Pytest)
        .expect("pytest row");
    assert_eq!(pytest.rerun_lane, RerunLane::Test);
    assert_eq!(pytest.rerun_last_command_id, "cmd:test.rerun_last");
}

#[test]
fn beta_projection_yields_one_tree_row_per_file_and_one_per_test_case() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T11:00:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T11:00:01Z",
    );
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T11:00:02Z");
    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        "2026-05-15T11:00:03Z",
    );

    let workspace_rows: Vec<_> = projection
        .tree
        .rows
        .iter()
        .filter(|row| row.row_kind == TestTreeRowKind::WorkspaceRoot)
        .collect();
    assert_eq!(workspace_rows.len(), 1);

    let file_rows: Vec<_> = projection
        .tree
        .rows
        .iter()
        .filter(|row| row.row_kind == TestTreeRowKind::TestFile)
        .collect();
    assert_eq!(file_rows.len(), discovery.test_files.len());
    assert!(file_rows.iter().all(|row| row
        .parent_row_id
        .as_deref()
        .map(|parent| parent == workspace_rows[0].tree_row_id)
        .unwrap_or(false)));

    let case_rows: Vec<_> = projection
        .tree
        .rows
        .iter()
        .filter(|row| row.row_kind == TestTreeRowKind::TestCase)
        .collect();
    assert_eq!(case_rows.len(), discovery.test_items.len());
    for case in &case_rows {
        assert!(case.canonical_test_item_ref.is_some());
        assert!(case.selector_ref.is_some());
        assert_eq!(
            case.rerun_last_command_id.as_deref(),
            Some("cmd:test.rerun_last")
        );
    }

    assert_eq!(projection.inline.rows.len(), discovery.test_items.len());
    for inline in &projection.inline.rows {
        let case = projection
            .tree
            .case_row_for_test_item(&inline.canonical_test_item_ref)
            .expect("case row for inline ref");
        assert_eq!(inline.tree_row_ref, case.tree_row_id);
    }
}

#[test]
fn parity_rows_are_unset_before_rerun_last_remembers_an_attempt() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T11:10:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T11:10:01Z",
    );
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T11:10:02Z");
    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        "2026-05-15T11:10:03Z",
    );

    assert_eq!(projection.parity_rows.len(), discovery.test_items.len());
    for parity in &projection.parity_rows {
        assert_eq!(
            parity.agreement_state,
            TestRunnerBetaParityState::RerunLaneUnset
        );
        assert_eq!(parity.rerun_last_command_id, "cmd:test.rerun_last");
        assert!(parity.rerun_last_prior_attempt_id.is_none());
        assert!(parity.rerun_last_prepared_attempt_id.is_none());
    }
    assert!(projection.parity_rows_safe_to_dispatch());
    assert!(!projection.parity_rows_all_agree());
}

#[test]
fn parity_row_transitions_to_rows_agree_after_rerun_last_remembers_one_case() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T11:20:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T11:20:01Z",
    );
    let target_contract = discovery
        .run_contracts
        .iter()
        .find(|contract| {
            contract
                .selection
                .test_item_id
                .as_deref()
                .map(|item_id| item_id.contains("test_health"))
                .unwrap_or(false)
        })
        .expect("a pytest contract for test_health")
        .clone();

    let mut loop_state = RerunLastLoop::new();
    loop_state.remember_pytest(
        target_contract.clone(),
        context.clone(),
        "2026-05-15T11:20:02Z",
    );
    let prepared = loop_state.prepare_last_test(
        &context,
        RerunTargetMode::ExactPriorTarget,
        "2026-05-15T11:20:03Z",
    );

    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T11:20:04Z");
    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        loop_state.last_launch(RerunLane::Test),
        Some(&prepared),
        "2026-05-15T11:20:05Z",
    );

    let remembered_canonical = target_contract
        .selection
        .test_item_id
        .clone()
        .expect("pytest contract has test_item_id");
    let remembered_parity = projection
        .parity_rows
        .iter()
        .find(|parity| parity.canonical_test_item_ref == remembered_canonical)
        .expect("parity row for remembered case");
    assert_eq!(
        remembered_parity.agreement_state,
        TestRunnerBetaParityState::RowsAgree
    );
    assert!(remembered_parity.rerun_last_prior_attempt_id.is_some());
    assert!(remembered_parity.rerun_last_prepared_attempt_id.is_some());
    assert_eq!(remembered_parity.rerun_lane_token, "test");

    for parity in &projection.parity_rows {
        if parity.canonical_test_item_ref != remembered_canonical {
            assert_eq!(
                parity.agreement_state,
                TestRunnerBetaParityState::RerunLaneUnset
            );
        }
    }
}

#[test]
fn artifact_identities_quote_the_same_session_and_canonical_id_as_the_attempt() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T11:30:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T11:30:01Z",
    );
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T11:30:02Z");
    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        "2026-05-15T11:30:03Z",
    );

    assert!(!projection.artifact_identities.is_empty());
    let manifest = TestRunnerBetaCoverageManifest::canonical(
        "test-runner-beta:m3:canonical",
        "2026-05-15T11:30:04Z",
    );
    let pytest_row = manifest
        .row_for_framework(TestRunnerBetaFramework::Pytest)
        .expect("pytest row");
    for identity in &projection.artifact_identities {
        assert_eq!(identity.framework, TestRunnerBetaFramework::Pytest);
        assert!(pytest_row
            .claimed_artifact_kinds
            .contains(&identity.artifact_kind));
        // every artifact identity MUST quote the parent session ref of one of
        // the alpha attempt packets the projection retains.
        let session_match = projection
            .attempt_packets
            .iter()
            .any(|packet| packet.session_plan.test_session_id == identity.test_session_ref);
        assert!(
            session_match,
            "session ref {} not found",
            identity.test_session_ref
        );
    }

    let raw_event_count = projection
        .artifact_identities
        .iter()
        .filter(|identity| identity.artifact_kind == TestArtifactKind::RawEventEnvelope)
        .count();
    assert!(raw_event_count > 0, "raw_event_envelope must be exposed");
}

#[test]
fn support_export_packet_round_trips_and_carries_manifest_and_parity_rows() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T11:40:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T11:40:01Z",
    );
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T11:40:02Z");
    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        "2026-05-15T11:40:03Z",
    );
    let export = TestRunnerBetaSupportExport::from_projection(
        &projection,
        "test-runner-beta:m3:canonical",
        "2026-05-15T11:40:04Z",
    );

    let json = serde_json::to_string(&export).expect("serialize support export");
    let round: TestRunnerBetaSupportExport =
        serde_json::from_str(&json).expect("deserialize support export");
    assert_eq!(round, export);
    assert_eq!(
        round.coverage_manifest.record_kind,
        TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    assert!(round
        .coverage_manifest
        .covers_wedges(&[aureline_runtime::TaskWedgeClass::Test]));
    assert_eq!(round.tree_projection_refs.len(), 1);
    assert_eq!(round.inline_projection_refs.len(), 1);
    assert_eq!(round.parity_rows.len(), discovery.test_items.len());
    assert!(!round.attempt_packet_refs.is_empty());

    let plaintext = export.render_plaintext();
    assert!(plaintext.contains("Test runner support export"));
    assert!(plaintext.contains("framework=pytest"));
    assert!(plaintext.contains("command=cmd:test.rerun_last"));
}
