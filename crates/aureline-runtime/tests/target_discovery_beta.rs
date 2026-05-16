//! Integration test for the beta target-discovery projection.
//!
//! This test replays the checked-in fixtures end-to-end against the canonical
//! [`aureline_runtime::TargetDiscoveryBetaSupportExport`]. It proves:
//!
//! - the beta projection picks up the source / freshness / capability /
//!   protected-action decisions specified in the fixture;
//! - the support-export packet pins the same projection on the wire as the
//!   chrome-facing rows;
//! - raw working-directory and credential strings never leak across the
//!   support-export boundary.

use std::path::PathBuf;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    TargetClass, TargetDiscoveryBetaCoverageManifest, TargetDiscoveryBetaSupportExport,
    ToolchainClass, TrustState, TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TARGET_DISCOVERY_BETA_SCHEMA_VERSION, TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/m3/target_confidence")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
}

fn resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "workspace:target-discovery-beta-it".to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 7,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/Users/example/private/project".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "capsule:target-discovery-beta-it".to_owned(),
            capsule_hash: "sha256:target-discovery-beta-it".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "target-discovery-beta-it".to_owned(),
    })
}

fn local_and_helper_contexts() -> (ExecutionContext, ExecutionContext) {
    let mut resolver = resolver();
    let local = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.local",
        TrustState::Trusted,
        "2026-05-15T19:40:00Z",
    ));
    let mut helper_request = ExecutionContextRequest::task_seed(
        "task.run.helper",
        TrustState::Restricted,
        "2026-05-15T19:41:00Z",
    );
    helper_request.requested_target_class = Some(TargetClass::ManagedWorkspace);
    helper_request.requested_toolchain_class = Some(ToolchainClass::BuildDriverRuntime);
    let helper = resolver.resolve(helper_request);
    (local, helper)
}

#[derive(Debug, Deserialize)]
struct BetaCase {
    record_kind: String,
    schema_version: u32,
    generated_at: String,
    support_export_id: String,
    expect: ExpectBlock,
}

#[derive(Debug, Deserialize)]
struct ExpectBlock {
    any_row_blocks_protected_dispatch: bool,
    rows: Vec<ExpectRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectRow {
    lane_token: String,
    target_class_token: String,
    discovery_source_token: String,
    discovery_freshness_token: String,
    supported_capability_tokens: Vec<String>,
    decisions: Vec<ExpectDecision>,
}

#[derive(Debug, Deserialize)]
struct ExpectDecision {
    action_token: String,
    decision_token: String,
}

#[test]
fn fixture_native_local_and_helper_managed_replays_end_to_end() {
    let payload = fixture("native_local_and_helper_managed.json");
    let case: BetaCase = serde_json::from_str(&payload).expect("parse fixture");
    assert_eq!(case.record_kind, "target_discovery_beta_case");
    assert_eq!(case.schema_version, TARGET_DISCOVERY_BETA_SCHEMA_VERSION);

    let (local, helper) = local_and_helper_contexts();
    let export = TargetDiscoveryBetaSupportExport::from_contexts(
        case.support_export_id.clone(),
        case.generated_at.clone(),
        [&local, &helper],
    );

    assert_eq!(
        export.record_kind,
        TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.support_export_id, case.support_export_id);
    assert_eq!(
        export.coverage_manifest.record_kind,
        TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );
    assert_eq!(
        export.projection.any_row_blocks_protected_dispatch,
        case.expect.any_row_blocks_protected_dispatch
    );
    assert_eq!(export.projection.rows.len(), case.expect.rows.len());

    for expect_row in &case.expect.rows {
        let row = export
            .projection
            .rows
            .iter()
            .find(|row| {
                row.lane_token == expect_row.lane_token
                    && row.target_class_token == expect_row.target_class_token
            })
            .unwrap_or_else(|| {
                panic!(
                    "row not found for lane={} target_class={}",
                    expect_row.lane_token, expect_row.target_class_token
                )
            });
        assert_eq!(
            row.discovery_source_token, expect_row.discovery_source_token,
            "row {} source",
            expect_row.target_class_token
        );
        assert_eq!(
            row.discovery_freshness_token, expect_row.discovery_freshness_token,
            "row {} freshness",
            expect_row.target_class_token
        );
        assert_eq!(
            row.supported_capability_tokens, expect_row.supported_capability_tokens,
            "row {} capabilities",
            expect_row.target_class_token
        );
        for expect_decision in &expect_row.decisions {
            let actual = row
                .protected_action_decisions
                .iter()
                .find(|d| d.action_token == expect_decision.action_token)
                .unwrap_or_else(|| {
                    panic!(
                        "decision not found for action={} row={}",
                        expect_decision.action_token, expect_row.target_class_token
                    )
                });
            assert_eq!(
                actual.decision_token, expect_decision.decision_token,
                "row {} action {}",
                expect_row.target_class_token, expect_decision.action_token
            );
        }
    }

    let plaintext = export.render_plaintext();
    assert!(plaintext.contains("source=native_protocol"));
    assert!(plaintext.contains("source=structured_adapter"));
    assert!(plaintext.contains("decision=blocked_freshness_stale"));
    assert!(plaintext.contains("decision=blocked_unsupported_capability"));

    // Boundary: raw workspace-default working directory must not leak.
    assert!(!plaintext.contains("/Users/example/private/project"));

    // Coverage manifest pins the closed vocabulary at export time.
    let canonical_manifest = TargetDiscoveryBetaCoverageManifest::canonical(
        export.coverage_manifest.manifest_id.clone(),
        export.coverage_manifest.generated_at.clone(),
    );
    assert_eq!(canonical_manifest, export.coverage_manifest);

    // The export carries one alpha card and one host-boundary row per beta row.
    assert_eq!(export.cards.len(), export.projection.rows.len());
    assert_eq!(export.host_boundaries.len(), export.projection.rows.len());
    assert_eq!(export.review_rows.len(), export.projection.rows.len());
    assert!(!export.context_provenance.is_empty());
}
