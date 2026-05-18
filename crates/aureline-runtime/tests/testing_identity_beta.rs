use std::path::{Path, PathBuf};

use aureline_runtime::{
    CanonicalTestAttempt, CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode,
    ImportedCiProjectionClass, ImportedCiTruthClass, PytestDiscoverer, PytestDiscovererConfig,
    PythonEnvironmentDetectorConfig, ScopeClass, TargetClass, TestAttemptKind,
    TestAttemptResultState, TestEvidenceClass, TestIdentityBetaBundle, TestIdentityLedgerError,
    TestIdentityStability, TestIdentitySurface, TestResultFreshnessClass, TestRunnerBetaProjection,
    TestSelectionOrigin, TestTargetEnvironmentClass, TrustState,
};

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
        resolver_version: "test-identity-beta-it-resolver".to_owned(),
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

fn runner_projection(time_prefix: &str) -> TestRunnerBetaProjection {
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
fn promoted_bundle_resolves_same_identity_across_all_test_surfaces() {
    let projection = runner_projection("2026-05-18T10:00");
    let bundle =
        TestIdentityBetaBundle::from_runner_projection(&projection, "2026-05-18T10:00:04Z");

    assert_eq!(bundle.items.len(), projection.inline.rows.len());
    assert!(bundle.attempt_history_is_append_only());
    assert!(bundle.anonymous_or_display_text_only_items().is_empty());
    assert!(bundle.imported_evidence_preserves_truth_class());

    for item in &bundle.items {
        assert!(bundle.surface_bindings_resolve_to_same_ids(&item.canonical_test_item_id));
        let bindings = bundle.surface_bindings_for_item(&item.canonical_test_item_id);
        let surface_tokens: Vec<_> = bindings
            .iter()
            .map(|binding| binding.surface_token.as_str())
            .collect();
        assert!(surface_tokens.contains(&TestIdentitySurface::EditorInlineMarker.as_str()));
        assert!(surface_tokens.contains(&TestIdentitySurface::TestTreeRow.as_str()));
        assert!(surface_tokens.contains(&TestIdentitySurface::CliSelector.as_str()));
        assert!(surface_tokens.contains(&TestIdentitySurface::ReviewPacket.as_str()));
        assert!(surface_tokens.contains(&TestIdentitySurface::SupportExport.as_str()));
        assert!(surface_tokens.contains(&TestIdentitySurface::ImportedCiOverlay.as_str()));
        assert!(bundle
            .selectors
            .iter()
            .any(
                |selector| selector.canonical_test_item_id == item.canonical_test_item_id
                    && selector.display_text_matching_forbidden
                    && selector
                        .readdressable_surface_tokens
                        .contains(&TestIdentitySurface::CliSelector.as_str().to_owned())
            ));
    }

    assert!(bundle.sessions.iter().all(|session| session
        .target_environment
        .target_environment_class
        == TestTargetEnvironmentClass::Local));
    let plaintext = bundle.support_export.render_plaintext();
    assert!(plaintext.contains("Test identity support export"));
    assert!(plaintext.contains("Imported CI overlays:"));
}

#[test]
fn append_attempt_extends_history_without_overwriting_prior_attempts() {
    let projection = runner_projection("2026-05-18T10:10");
    let mut bundle =
        TestIdentityBetaBundle::from_runner_projection(&projection, "2026-05-18T10:10:04Z");
    let session = bundle.sessions[0].clone();
    let previous = bundle
        .attempts
        .iter()
        .find(|attempt| attempt.parent_test_session_ref == session.test_session_id)
        .expect("session attempt")
        .clone();
    let next = CanonicalTestAttempt::follow_up(
        &session,
        &previous,
        aureline_runtime::TestAttemptLineageClass::Rerun,
        "passed",
        "2026-05-18T10:10:05Z",
    );
    let next_id = next.test_attempt_id.clone();

    bundle.append_attempt(next).expect("append attempt");
    let updated = bundle
        .sessions
        .iter()
        .find(|candidate| candidate.test_session_id == session.test_session_id)
        .expect("updated session");
    assert_eq!(
        updated.latest_attempt_ref.as_deref(),
        Some(next_id.as_str())
    );
    assert_eq!(updated.attempt_refs.len(), session.attempt_refs.len() + 1);
    assert!(updated.attempt_refs.contains(&previous.test_attempt_id));
    assert!(bundle.attempt_history_is_append_only());

    let duplicate = bundle
        .attempts
        .iter()
        .find(|attempt| attempt.test_attempt_id == next_id)
        .expect("new attempt")
        .clone();
    assert_eq!(
        bundle.append_attempt(duplicate),
        Err(TestIdentityLedgerError::DuplicateAttempt(next_id))
    );
}

#[test]
fn imported_ci_overlay_stays_imported_until_local_confirmation() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-18T10:20:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-18T10:20:01Z",
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
        .expect("target test contract")
        .clone();
    let target_item = target_contract
        .selection
        .test_item_id
        .clone()
        .expect("canonical test item");
    let mut packets = discovery.test_attempt_alpha_packets("2026-05-18T10:20:02Z");
    let imported_packet = packets
        .iter_mut()
        .find(|packet| {
            packet
                .identity_projection
                .canonical_test_item_ref
                .as_deref()
                == Some(target_item.as_str())
        })
        .expect("packet for target item");
    imported_packet.identity_projection.identity_stability =
        TestIdentityStability::ImportedReadOnly;
    imported_packet.identity_projection.identity_stability_token =
        TestIdentityStability::ImportedReadOnly.as_str().to_owned();
    imported_packet.imported_ci_projection.projection_class =
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly;
    imported_packet.imported_ci_projection.projection_token =
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly
            .as_str()
            .to_owned();
    imported_packet
        .imported_ci_projection
        .read_only_imported_evidence = true;
    imported_packet.imported_ci_projection.provider_run_ref =
        Some("provider-run:ci:pytest:readonly".to_owned());
    imported_packet.imported_ci_projection.imported_attempt_refs =
        vec![imported_packet.attempts[0].test_attempt_id.clone()];
    imported_packet
        .imported_ci_projection
        .does_not_claim_live_local_truth = true;
    imported_packet.attempts[0].attempt_kind = TestAttemptKind::ProviderCiImport;
    imported_packet.attempts[0].attempt_kind_token =
        TestAttemptKind::ProviderCiImport.as_str().to_owned();
    imported_packet.attempts[0].result_state = TestAttemptResultState::ImportedFailed;
    imported_packet.attempts[0].result_state_token =
        TestAttemptResultState::ImportedFailed.as_str().to_owned();
    imported_packet.attempts[0].imported_ci_projection_class =
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly;
    imported_packet.attempts[0].imported_ci_projection_token =
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly
            .as_str()
            .to_owned();

    let projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        packets,
        None,
        None,
        "2026-05-18T10:20:03Z",
    );
    let bundle =
        TestIdentityBetaBundle::from_runner_projection(&projection, "2026-05-18T10:20:04Z");
    let overlay = bundle
        .imported_ci_overlays
        .iter()
        .find(|overlay| overlay.canonical_test_item_id == target_item)
        .expect("imported overlay");

    assert_eq!(
        overlay.imported_ci_truth_class,
        ImportedCiTruthClass::ImportedCurrentReadOnly
    );
    assert_eq!(overlay.freshness_class, TestResultFreshnessClass::Imported);
    assert!(!overlay.current_local_truth_claim_allowed);
    assert!(bundle.imported_evidence_preserves_truth_class());

    let binding = bundle
        .surface_bindings_for_item(&target_item)
        .into_iter()
        .find(|binding| binding.surface == TestIdentitySurface::ImportedCiOverlay)
        .expect("ci overlay binding");
    assert_eq!(
        binding.evidence_class,
        TestEvidenceClass::ImportedCiReadOnly
    );
    assert_eq!(
        binding.selection_origin_token,
        TestSelectionOrigin::ImportedCiOverlay.as_str()
    );
}
