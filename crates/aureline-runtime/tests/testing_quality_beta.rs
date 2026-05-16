use std::path::{Path, PathBuf};

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, PytestDiscoverer, PytestDiscovererConfig,
    PythonEnvironmentDetectorConfig, ScopeClass, TargetClass, TestArtifactKind,
    TestQualityBetaCoverageManifest, TestQualityBetaSupportExport, TestQualityFreshness,
    TestQualityKind, TestQualityProjection, TestQualitySupportClass, TestRunnerBetaFramework,
    TestRunnerBetaProjection, TestRunnerBetaSupportExport, TrustState,
    TEST_QUALITY_BASELINE_PACKET_RECORD_KIND, TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_QUALITY_BETA_PROJECTION_RECORD_KIND, TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND,
    TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND, TEST_QUALITY_FLAKY_PACKET_RECORD_KIND,
    TEST_QUALITY_ROW_TRUTH_RECORD_KIND, TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND,
    TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/testing/m3/quality_packets")
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
        resolver_version: "test-quality-beta-it-resolver".to_owned(),
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

fn build_runner_projection(time_prefix: &str) -> TestRunnerBetaProjection {
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
    let attempt_packets = discovery.test_attempt_alpha_packets(&format!("{time_prefix}:02Z"));
    TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        &format!("{time_prefix}:03Z"),
    )
}

#[test]
fn checked_in_coverage_manifest_matches_canonical_runtime_manifest() {
    let payload = read_fixture("coverage_manifest.json");
    let fixture: TestQualityBetaCoverageManifest =
        serde_json::from_str(&payload).expect("parse coverage manifest fixture");
    assert_eq!(
        fixture.record_kind,
        TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    assert_eq!(
        fixture.schema_version,
        TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION
    );

    let canonical = TestQualityBetaCoverageManifest::canonical(
        fixture.manifest_id.clone(),
        fixture.generated_at.clone(),
    );
    assert_eq!(canonical, fixture);

    let pytest = canonical
        .row_for_framework(TestRunnerBetaFramework::Pytest)
        .expect("pytest row");
    for kind in TestQualityKind::ALL {
        assert!(pytest
            .claimed_quality_kind_tokens
            .contains(&kind.as_str().to_owned()));
    }
    assert!(pytest
        .backing_artifact_kinds
        .contains(&TestArtifactKind::CoverageReport));
    assert!(pytest
        .backing_artifact_kinds
        .contains(&TestArtifactKind::SnapshotDiff));
}

#[test]
fn quality_projection_emits_four_packets_per_case_with_identity_parity() {
    let projection = build_runner_projection("2026-05-15T12:00");
    let quality = TestQualityProjection::from_beta_projection(&projection);

    let case_count = projection.inline.rows.len();
    assert_eq!(quality.coverage_packets.len(), case_count);
    assert_eq!(quality.flaky_packets.len(), case_count);
    assert_eq!(quality.snapshot_packets.len(), case_count);
    assert_eq!(quality.baseline_packets.len(), case_count);
    assert_eq!(quality.row_truths.len(), case_count);
    assert_eq!(
        quality.record_kind,
        TEST_QUALITY_BETA_PROJECTION_RECORD_KIND
    );
    assert_eq!(
        quality.tree_projection_ref,
        projection.tree.tree_projection_id
    );
    assert_eq!(
        quality.inline_projection_ref,
        projection.inline.inline_projection_id
    );

    for row in &quality.row_truths {
        assert_eq!(row.record_kind, TEST_QUALITY_ROW_TRUTH_RECORD_KIND);

        let tree_case = projection
            .tree
            .case_row_for_test_item(&row.identity.canonical_test_item_ref)
            .expect("tree case row for quality identity");
        let inline_row = projection
            .inline
            .row_for_test_item(&row.identity.canonical_test_item_ref)
            .expect("inline row for quality identity");
        assert_eq!(row.tree_row_ref, tree_case.tree_row_id);
        assert_eq!(row.inline_row_ref, inline_row.inline_row_id);
        assert_eq!(
            row.identity.selector_ref, inline_row.selector_ref,
            "quality identity must reuse the inline selector ref"
        );
        assert_eq!(
            row.identity.test_session_ref, inline_row.test_session_ref,
            "quality identity must reuse the inline session ref"
        );
        assert_eq!(row.identity.framework, projection.framework);
    }
}

#[test]
fn rows_without_current_packets_downgrade_away_from_stable_supported() {
    let projection = build_runner_projection("2026-05-15T12:10");
    let quality = TestQualityProjection::from_beta_projection(&projection);

    // The default pytest packets request neither coverage nor snapshot review,
    // produce an `Unknown` flaky verdict, and the latest attempt is still
    // `Running` (no baseline established). Every row MUST downgrade — the
    // product must not imply stable support without a current packet.
    assert!(quality.any_row_downgraded());
    assert!(!quality.every_row_stable_supported());

    for row in &quality.row_truths {
        assert_ne!(
            row.row_support_class,
            TestQualitySupportClass::StableSupported,
            "row {} must not claim stable support without current packets",
            row.identity.canonical_test_item_ref
        );
    }

    // Every baseline packet MUST flag the row as retest-pending because no
    // passing attempt has been recorded yet.
    for baseline in &quality.baseline_packets {
        assert_eq!(
            baseline.record_kind,
            TEST_QUALITY_BASELINE_PACKET_RECORD_KIND
        );
        assert_eq!(
            baseline.support_class,
            TestQualitySupportClass::RetestPendingNoCurrentPacket
        );
        assert!(baseline.baseline_attempt_ref.is_none());
        assert_eq!(
            baseline.baseline_state_token, "no_baseline_established_yet",
            "baseline must downgrade when no passing attempt exists"
        );
    }

    // Coverage rows MUST be labelled `out_of_scope` (CoverageMergeClass::
    // NotRequested) rather than implying current coverage truth.
    for coverage in &quality.coverage_packets {
        assert_eq!(
            coverage.record_kind,
            TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND
        );
        assert_eq!(coverage.support_class, TestQualitySupportClass::OutOfScope);
    }

    // Snapshot rows MUST be labelled `out_of_scope` (SnapshotReviewState::
    // NotRequired).
    for snapshot in &quality.snapshot_packets {
        assert_eq!(
            snapshot.record_kind,
            TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND
        );
        assert_eq!(snapshot.support_class, TestQualitySupportClass::OutOfScope);
    }

    // Flaky rows MUST report `unknown_requires_review` rather than asserting
    // stable support from an Unknown verdict.
    for flaky in &quality.flaky_packets {
        assert_eq!(flaky.record_kind, TEST_QUALITY_FLAKY_PACKET_RECORD_KIND);
        assert_eq!(
            flaky.support_class,
            TestQualitySupportClass::UnknownRequiresReview
        );
        assert_eq!(flaky.freshness, TestQualityFreshness::UnknownRequiresReview);
    }
}

#[test]
fn test_runner_support_export_can_point_to_the_same_quality_packet_the_in_product_flow_renders() {
    let projection = build_runner_projection("2026-05-15T12:20");
    let quality = TestQualityProjection::from_beta_projection(&projection);
    let quality_export = TestQualityBetaSupportExport::from_projection(
        &quality,
        "test-quality-truth-beta:m3:canonical",
        "2026-05-15T12:20:05Z",
    );
    let mut runner_export = TestRunnerBetaSupportExport::from_projection(
        &projection,
        "test-runner-beta:m3:canonical",
        "2026-05-15T12:20:06Z",
    );
    runner_export.attach_quality_truth_refs(
        quality.projection_id.clone(),
        quality_export.support_export_id.clone(),
    );

    assert_eq!(
        runner_export.quality_projection_ref.as_deref(),
        Some(quality.projection_id.as_str())
    );
    assert_eq!(
        runner_export.quality_support_export_ref.as_deref(),
        Some(quality_export.support_export_id.as_str())
    );

    // Same tree/inline projection refs on both packets — the support reviewer
    // joins one canonical identity across both lanes.
    assert_eq!(
        runner_export.tree_projection_refs,
        vec![quality_export.tree_projection_ref.clone()]
    );
    assert_eq!(
        runner_export.inline_projection_refs,
        vec![quality_export.inline_projection_ref.clone()]
    );
    assert_eq!(
        quality_export.record_kind,
        TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND
    );

    let plaintext = runner_export.render_plaintext();
    assert!(plaintext.contains("Quality projection:"));
    assert!(plaintext.contains(&quality.projection_id));
    assert!(plaintext.contains("Quality support export:"));
    assert!(plaintext.contains(&quality_export.support_export_id));
}

#[test]
fn quality_projection_round_trips_through_serde() {
    let projection = build_runner_projection("2026-05-15T12:30");
    let quality = TestQualityProjection::from_beta_projection(&projection);

    let json = serde_json::to_string(&quality).expect("serialize quality projection");
    let round: TestQualityProjection =
        serde_json::from_str(&json).expect("deserialize quality projection");
    assert_eq!(round, quality);

    let export = TestQualityBetaSupportExport::from_projection(
        &quality,
        "test-quality-truth-beta:m3:canonical",
        "2026-05-15T12:30:05Z",
    );
    let export_json = serde_json::to_string(&export).expect("serialize quality export");
    let export_round: TestQualityBetaSupportExport =
        serde_json::from_str(&export_json).expect("deserialize quality export");
    assert_eq!(export_round, export);

    let plaintext = export.render_plaintext();
    assert!(plaintext.contains("Test quality support export"));
    assert!(plaintext.contains("framework=pytest"));
    for kind in TestQualityKind::ALL {
        assert!(plaintext.contains(kind.as_str()));
    }
}
