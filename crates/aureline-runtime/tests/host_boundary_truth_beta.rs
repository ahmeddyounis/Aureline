//! Integration replay for host-boundary truth, wrong-target reapproval, and lifecycle export.
//!
//! The fixture corpus proves one shared truth record projects consistently to
//! task, terminal, debug, AI-tool, browser-handoff, transcript, CLI/headless,
//! and support/export surfaces. It also exercises wrong-target reapproval,
//! route drift, missing adapter capability narrowing, and managed-workspace
//! local-continuity export.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    evaluate_host_boundary_reapproval, ActionRouteClass, AuthorityLinkageClass, CapsuleDriftState,
    EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, HostBoundaryReviewBinding, HostBoundarySupportExport,
    HostBoundarySurfaceClass, HostBoundaryTruthOptions, HostBoundaryTruthRecord, IdentityMode,
    ManagedLifecycleLineageEntry, ManagedLifecyclePhaseClass, ManagedLifecycleStateClass,
    ManagedLocalEditingContinuityClass, ManagedWorkspaceLifecycleBetaRecord, ProtectedActionClass,
    ProtectedActionDecisionClass, ProtectedActionDecisionRow, ReapprovalRequirementClass,
    RouteChangeReasonCode, ScopeClass, SupportedCapabilityClass, TargetClass, TargetConfidenceCard,
    TargetDiscoveryBetaRow, TrustState, WrongTargetCorrectionClass,
    HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION, HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND,
    HOST_BOUNDARY_TRUTH_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/m3/host_boundary_and_wrong_target")
}

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "workspace:host-boundary-beta-it".to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 11,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/Users/example/private/project".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "capsule:host-boundary-beta-it".to_owned(),
            capsule_hash: "sha256:host-boundary-beta-it".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "host-boundary-beta-it".to_owned(),
    })
}

#[test]
fn host_boundary_truth_fixtures_replay_across_surfaces_and_export() {
    let manifest_path = fixture_root().join("manifest.yaml");
    let manifest_payload = std::fs::read_to_string(&manifest_path).expect("manifest reads");
    let manifest: FixtureManifest =
        serde_yaml::from_str(&manifest_payload).expect("manifest parses");
    assert_eq!(
        manifest.schema_version,
        HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION
    );
    assert!(!manifest.case_refs.is_empty());

    let mut records = Vec::new();
    let mut evaluations = Vec::new();

    for case_ref in &manifest.case_refs {
        let case_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../")
            .join(case_ref);
        let payload = std::fs::read_to_string(&case_path)
            .unwrap_or_else(|err| panic!("read {case_ref}: {err}"));
        let fixture: FixtureCase =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {case_ref}: {err}"));
        assert_eq!(fixture.record_kind, "host_boundary_truth_beta_case");
        assert_eq!(
            fixture.schema_version,
            HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION
        );

        let mut resolver = baseline_resolver();
        let context = resolve_fixture(&mut resolver, &fixture.input);
        let lifecycle = lifecycle_for_fixture(&fixture.input);
        let options = fixture_options(&fixture);
        let record = if fixture.input.custom_discovery.as_deref()
            == Some("missing_debug_adapter_capability")
        {
            let discovery = missing_debug_capability_row(&context);
            HostBoundaryTruthRecord::from_context_and_discovery(
                &context,
                &discovery,
                lifecycle.as_ref(),
                options,
            )
        } else {
            HostBoundaryTruthRecord::from_context(&context, lifecycle.as_ref(), options)
        };

        assert_eq!(record.record_kind, HOST_BOUNDARY_TRUTH_RECORD_KIND);
        assert!(
            record.validate().is_empty(),
            "{case_ref} must not overclaim truth: {:?}",
            record.validate()
        );
        assert_eq!(record.action_route_token, fixture.expect.action_route_token);
        assert_eq!(
            record.action_target_token,
            fixture.expect.action_target_token
        );
        assert_eq!(
            record.host_boundary_cue_stack_tokens,
            fixture.expect.host_boundary_cue_stack_tokens
        );
        assert_eq!(
            record.reapproval_requirement_token,
            fixture.expect.reapproval_requirement_token
        );
        assert_eq!(
            record.wrong_target_correction_token,
            fixture.expect.wrong_target_correction_token
        );

        if let Some(expected_label) = &fixture.expect.managed_lifecycle_label_token {
            let lifecycle = record
                .managed_lifecycle
                .as_ref()
                .expect("managed fixture carries lifecycle");
            assert_eq!(
                &lifecycle.managed_workspace_reviewer_label_token,
                expected_label
            );
            assert_eq!(
                Some(lifecycle.managed_workspace_lifecycle_state_token.as_str()),
                fixture.expect.managed_lifecycle_state_token.as_deref()
            );
            assert_eq!(
                Some(lifecycle.local_editing_continuity_token.as_str()),
                fixture.expect.local_editing_continuity_token.as_deref()
            );
            assert!(!lifecycle.lifecycle_lineage_tokens.is_empty());
        } else {
            assert!(record.managed_lifecycle.is_none());
        }

        let projections = record.all_surface_projections();
        assert_eq!(projections.len(), fixture.expect.surface_projection_count);
        let projection_tokens = projections
            .iter()
            .map(|projection| projection.surface_token.clone())
            .collect::<Vec<_>>();
        for required_surface in &manifest.required_surface_tokens {
            assert!(
                projection_tokens.contains(required_surface),
                "{case_ref} missing projection for {required_surface}"
            );
        }
        for projection in &projections {
            assert_eq!(projection.truth_record_ref, record.record_id);
            assert_eq!(
                projection.host_boundary_cue_stack_tokens,
                record.host_boundary_cue_stack_tokens
            );
            assert_eq!(
                projection.discovery_source_token,
                record.discovery.discovery_source_token
            );
            if let Some(expected_continue) = fixture.expect.continue_action_enabled {
                if projection.surface == fixture.input.surface {
                    assert_eq!(projection.continue_action_enabled, expected_continue);
                }
            }
        }

        if let Some(expected_blocked) = &fixture.expect.blocked_action_tokens {
            let blocked = record
                .discovery
                .protected_action_decisions
                .iter()
                .filter(|decision| decision.decision.is_blocked())
                .map(|decision| decision.action_token.clone())
                .collect::<Vec<_>>();
            assert_eq!(&blocked, expected_blocked);
            assert_eq!(
                record.discovery.authoritative_capability_subset_tokens,
                fixture
                    .expect
                    .authoritative_capability_subset_tokens
                    .clone()
                    .expect("fixture names authoritative subset")
            );
            for action in fixture
                .expect
                .allowed_or_review_action_tokens
                .as_ref()
                .expect("fixture names allowed or review actions")
            {
                assert!(
                    !blocked.contains(action),
                    "missing adapter capability must not block {action}"
                );
            }
        }

        if let Some(review) = &fixture.review_binding {
            let review_context = resolve_fixture(&mut resolver, review);
            let review_record = HostBoundaryTruthRecord::from_context(
                &review_context,
                None,
                HostBoundaryTruthOptions::new(
                    format!("{}:review", fixture.options.record_id),
                    format!("{}:review", fixture.options.invocation_session_id),
                    fixture.generated_at.clone(),
                    review.surface,
                ),
            );
            let binding =
                HostBoundaryReviewBinding::from_record("binding:fixture.review", &review_record);
            let evaluation = evaluate_host_boundary_reapproval(
                format!("eval:{}", fixture.case_id),
                &binding,
                &record,
            );
            assert!(evaluation.outcome.requires_reapproval());
            let drift_fields = evaluation
                .outcome
                .drift_rows()
                .iter()
                .map(|row| row.field.as_str().to_owned())
                .collect::<Vec<_>>();
            for expected in fixture
                .expect
                .expected_drift_fields
                .as_ref()
                .expect("review fixture names drift fields")
            {
                assert!(
                    drift_fields.contains(expected),
                    "expected drift field {expected} in {drift_fields:?}"
                );
            }
            evaluations.push(evaluation);
        }

        records.push(record);
    }

    let export = HostBoundarySupportExport::from_records(
        "support-export:host-boundary-truth:fixture",
        "2026-05-17T21:30:00Z",
        records,
        evaluations,
    );
    assert_eq!(export.record_kind, HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.records.len(), export.support_projections.len());
    assert!(export.any_record_requires_reapproval_or_review);
    let plaintext = export.render_plaintext();
    assert!(plaintext.contains("route=remote_rpc_route"));
    assert!(plaintext.contains("reapproval=approval_ticket_reissue_required"));
    assert!(plaintext.contains("lifecycle=recovering"));
    assert!(!plaintext.contains("/Users/example/private/project"));
}

fn resolve_fixture(
    resolver: &mut ExecutionContextResolver,
    input: &FixtureInput,
) -> ExecutionContext {
    match input.constructor.as_str() {
        "local_task_seed" => resolver.resolve(ExecutionContextRequest::task_seed(
            &input.command_id,
            input.trust_state,
            "2026-05-17T20:59:00Z",
        )),
        "remote_attach_task_seed" => {
            let target_class = input
                .target_class
                .expect("remote_attach_task_seed fixture needs target_class");
            resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
                &input.command_id,
                target_class,
                input.trust_state,
                "2026-05-17T20:59:00Z",
            ))
        }
        "request_workspace_task_seed" => {
            let target_class = input
                .target_class
                .expect("request_workspace_task_seed fixture needs target_class");
            resolver.resolve(ExecutionContextRequest::request_workspace_task_seed(
                &input.command_id,
                target_class,
                input.trust_state,
                "2026-05-17T20:59:00Z",
            ))
        }
        other => panic!("unknown fixture constructor {other}"),
    }
}

fn fixture_options(fixture: &FixtureCase) -> HostBoundaryTruthOptions {
    let mut options = HostBoundaryTruthOptions::new(
        fixture.options.record_id.clone(),
        fixture.options.invocation_session_id.clone(),
        fixture.generated_at.clone(),
        fixture.input.surface,
    );
    options.authority_linkage_class = fixture.options.authority_linkage_class;
    options.wrong_target_correction_class = fixture.options.wrong_target_correction_class;
    options.reapproval_requirement_class = fixture.options.reapproval_requirement_class;
    options.route_change_reason_code = fixture.options.route_change_reason_code;
    options.repair_hook_ref = fixture.options.repair_hook_ref.clone();
    options.prior_target_ref = fixture.options.prior_target_ref.clone();
    options.prior_route_class = fixture.options.prior_route_class;
    options
}

fn lifecycle_for_fixture(input: &FixtureInput) -> Option<ManagedWorkspaceLifecycleBetaRecord> {
    let case_ref = input.lifecycle_case_ref.as_ref()?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(case_ref);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read lifecycle fixture {case_ref}: {err}"));
    let fixture: ManagedLifecycleFixture =
        serde_json::from_str(&payload).expect("managed lifecycle fixture parses");
    Some(ManagedWorkspaceLifecycleBetaRecord::from_lineage(
        fixture.row_id,
        fixture.workspace_ref,
        fixture.workspace_instance_ref,
        fixture.generated_at,
        fixture.local_editing_continuity,
        fixture
            .lineage
            .into_iter()
            .map(|entry| {
                ManagedLifecycleLineageEntry::new(
                    entry.phase,
                    entry.state,
                    entry.reason,
                    entry.observed_at,
                    entry.summary,
                )
            })
            .collect(),
        fixture.visible_summary,
        fixture.safe_continuation,
        fixture.source_refs,
        fixture.support_packet_refs,
        Vec::new(),
    ))
}

fn missing_debug_capability_row(context: &ExecutionContext) -> TargetDiscoveryBetaRow {
    let card = TargetConfidenceCard::from_context(context);
    let mut row = TargetDiscoveryBetaRow::from_card_and_context(&card, context);
    row.discovery_freshness = aureline_runtime::DiscoveryFreshnessClass::RecentWithinSession;
    row.discovery_freshness_token = "recent_within_session".to_owned();
    row.supported_capabilities = vec![
        SupportedCapabilityClass::Run,
        SupportedCapabilityClass::Test,
        SupportedCapabilityClass::Build,
        SupportedCapabilityClass::InspectOnly,
    ];
    row.supported_capability_tokens = row
        .supported_capabilities
        .iter()
        .map(|capability| capability.as_str().to_owned())
        .collect();
    row.protected_action_decisions = ProtectedActionClass::ALL
        .into_iter()
        .map(|action| {
            let decision = match action {
                ProtectedActionClass::DispatchDebugLaunch
                | ProtectedActionClass::DispatchDebugAttach => {
                    ProtectedActionDecisionClass::BlockedUnsupportedCapability
                }
                ProtectedActionClass::ExportArtifact => ProtectedActionDecisionClass::Allowed,
                _ => ProtectedActionDecisionClass::RequiresReview,
            };
            ProtectedActionDecisionRow {
                action,
                action_token: action.as_str().to_owned(),
                decision,
                decision_token: decision.as_str().to_owned(),
                summary: format!(
                    "fixture decision {} => {}",
                    action.as_str(),
                    decision.as_str()
                ),
            }
        })
        .collect();
    row
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    case_refs: Vec<String>,
    required_surface_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    generated_at: String,
    input: FixtureInput,
    options: FixtureOptions,
    #[serde(default)]
    review_binding: Option<FixtureInput>,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureInput {
    constructor: String,
    command_id: String,
    surface: HostBoundarySurfaceClass,
    #[serde(default)]
    target_class: Option<TargetClass>,
    trust_state: TrustState,
    #[serde(default)]
    lifecycle_case_ref: Option<String>,
    #[serde(default)]
    custom_discovery: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureOptions {
    record_id: String,
    invocation_session_id: String,
    authority_linkage_class: AuthorityLinkageClass,
    wrong_target_correction_class: WrongTargetCorrectionClass,
    reapproval_requirement_class: ReapprovalRequirementClass,
    route_change_reason_code: RouteChangeReasonCode,
    #[serde(default)]
    repair_hook_ref: Option<String>,
    #[serde(default)]
    prior_target_ref: Option<String>,
    #[serde(default)]
    prior_route_class: Option<ActionRouteClass>,
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    action_route_token: String,
    action_target_token: String,
    host_boundary_cue_stack_tokens: Vec<String>,
    #[serde(default)]
    managed_lifecycle_label_token: Option<String>,
    #[serde(default)]
    managed_lifecycle_state_token: Option<String>,
    #[serde(default)]
    local_editing_continuity_token: Option<String>,
    reapproval_requirement_token: String,
    wrong_target_correction_token: String,
    surface_projection_count: usize,
    #[serde(default)]
    blocked_action_tokens: Option<Vec<String>>,
    #[serde(default)]
    allowed_or_review_action_tokens: Option<Vec<String>>,
    #[serde(default)]
    authoritative_capability_subset_tokens: Option<Vec<String>>,
    #[serde(default)]
    expected_drift_fields: Option<Vec<String>>,
    #[serde(default)]
    continue_action_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ManagedLifecycleFixture {
    row_id: String,
    workspace_ref: String,
    #[serde(default)]
    workspace_instance_ref: Option<String>,
    generated_at: String,
    local_editing_continuity: ManagedLocalEditingContinuityClass,
    lineage: Vec<ManagedLifecycleFixtureEntry>,
    visible_summary: String,
    safe_continuation: String,
    #[serde(default)]
    source_refs: Vec<String>,
    #[serde(default)]
    support_packet_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ManagedLifecycleFixtureEntry {
    phase: ManagedLifecyclePhaseClass,
    state: ManagedLifecycleStateClass,
    reason: aureline_runtime::ManagedWorkspaceTransitionReason,
    observed_at: String,
    summary: String,
}
