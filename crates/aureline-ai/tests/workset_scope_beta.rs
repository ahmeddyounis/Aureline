//! Beta coverage for the AI composer / context-inspector workset /
//! sparse-slice / policy-limited-view scope boundaries.
//!
//! Named-workset, sparse-slice, and policy-limited-view scope truth is already
//! beta on the workspace, search, graph, and refactor surfaces. This drill
//! promotes it on the AI surface: it replays a frozen corpus through the AI
//! composer / context-inspector projection and proves the AI surface honours the
//! declared scope the same way the sibling surfaces do — in-scope context stays
//! drawn, context that escapes the scope is *labeled* (blocked, with a typed
//! omission reason) rather than silently drawn in or silently dropped,
//! policy-hidden members are disclosed through the scope truth, and the same
//! scope truth flows into the AI evidence handoff and spend receipt.
//!
//! The corpus reuses the `aureline-workspace` `ScopeClass` vocabulary verbatim,
//! so the AI surface shares one scope-truth vocabulary instead of a divergent
//! label set. `aureline-ai` does not depend on `aureline-workspace` in
//! production; the workspace crate is a test-only dependency here (the edge is
//! acyclic — `aureline-workspace` does not depend on `aureline-ai`) so the test
//! can build the canonical scope artifact and project the same scope truth the
//! product surfaces consume. `ci/check_beta_ai_workset_scope.py` re-derives the
//! workspace vocabulary from crate source and fails closed if the shared map or
//! the required scope-class coverage drift.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_ai::routing::RegionPostureClass;
use aureline_ai::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingPacket, ComposerContextAlphaInput,
    ComposerContextAlphaSnapshot, ComposerContextAlphaViolation, ComposerContextItem,
    ComposerContextReviewLock, ComposerDraft, CostEnvelopeClass, CostVisibilityClass,
    DeploymentProfileClass, ExecutionBoundaryClass, ExecutionLocusClass, ExhaustionStateClass,
    IntentModeClass, LatencyCostEnvelope, LatencyEnvelopeClass, PolicyTrustState, QuotaFamilyClass,
    QuotaInspector, QuotaScopeClass, QuotaStateClass, RetentionStanceClass, ReviewLockClass,
    RouteOriginClass, RouteSelectionOverrideReasonClass, RouteSelectionReasonClass,
    RoutingPolicyContext, RoutingRunStateClass, SelectedOutcomeClass, SourceClass,
    SpendReceiptRecord, TokenCeilingClass, ToolCallCeilingClass, WallTimeCeilingClass,
};
use aureline_ai::{
    ContextFreshnessClass, ContextGroupClass, ContextItemStateClass, ContextLocalityClass,
    ContextOmissionReasonClass, ContextTrustClass,
};
use aureline_workspace::{
    BetaConsumerSurface, BroadActionClass, ExcludedRootReason, ScopeClass, ScopeObservationInputs,
    WorksetArtifactRecord, WorksetScopeBetaTruth,
};

/// Canonical `aureline-workspace` scope-class vocabulary, read directly from the
/// dependency. `ci/check_beta_ai_workset_scope.py` re-derives the same set from
/// crate source and fails closed if the corpus mirror drifts from it.
const WORKSPACE_SCOPE_CLASSES: [ScopeClass; 5] = [
    ScopeClass::CurrentRepo,
    ScopeClass::SelectedWorkset,
    ScopeClass::SparseSlice,
    ScopeClass::FullWorkspace,
    ScopeClass::PolicyLimitedView,
];

#[derive(Debug, Clone, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    scope_class_vocabulary_map: BTreeMap<String, String>,
    ai_only_scope_classes: Vec<String>,
    required_scope_classes: Vec<String>,
    workspace_scope_class_vocabulary: Vec<String>,
    cases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Case {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    ai_scope_class: String,
    workspace_scope_class: String,
    workspace: WorkspaceBlock,
    artifact: WorksetArtifactRecord,
    context_items: Vec<CaseContextItem>,
    expect: CaseExpect,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceBlock {
    workspace_root_refs: Vec<String>,
    #[serde(default)]
    workspace_root_labels: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseContextItem {
    context_item_id: String,
    membership: String,
    group: String,
    stable_identity_ref: String,
    display_label: String,
    estimated_byte_size: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct CaseExpect {
    ai_scope_label: String,
    in_scope_count: usize,
    out_of_scope_count: usize,
    policy_limited_count: usize,
    #[serde(default)]
    excluded_root_refs: Vec<String>,
    policy_hidden_excluded: bool,
    evidence_survivor_ids: Vec<String>,
    ai_apply_decision: String,
}

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ai/workset_scope_beta")
}

fn load_manifest() -> Manifest {
    let path = corpus_dir().join("manifest.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let manifest: Manifest =
        serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    assert_eq!(manifest.record_kind, "ai_workset_scope_beta_manifest");
    assert_eq!(manifest.schema_version, 1);
    manifest
}

fn load_case(name: &str) -> Case {
    let path = corpus_dir().join(name);
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let case: Case =
        serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
    assert_eq!(
        case.record_kind, "ai_workset_scope_beta_case",
        "unexpected record_kind in {name}"
    );
    assert_eq!(
        case.schema_version, 1,
        "unexpected schema_version in {name}"
    );
    case
}

fn load_cases(manifest: &Manifest) -> Vec<Case> {
    manifest.cases.iter().map(|name| load_case(name)).collect()
}

fn scope_class_from_token(token: &str) -> ScopeClass {
    WORKSPACE_SCOPE_CLASSES
        .iter()
        .copied()
        .find(|class| class.as_str() == token)
        .unwrap_or_else(|| panic!("unknown workspace scope class token {token}"))
}

/// Derives the AI composer scope label from the shared `ScopeClass` chip-label
/// vocabulary — exactly the labeling `WorksetArtifactRecord::project_chip` uses
/// — so the AI surface never mints a private scope label.
fn derive_scope_label(scope_class: ScopeClass, workset_name: &str) -> String {
    match scope_class {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
            scope_class.chip_label_family().to_owned()
        }
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            format!("{} · {}", scope_class.chip_label_family(), workset_name)
        }
    }
}

fn group_from_token(token: &str) -> ContextGroupClass {
    match token {
        "open_files" => ContextGroupClass::OpenFiles,
        "symbols_graph_entities" => ContextGroupClass::SymbolsGraphEntities,
        "diagnostics_tests" => ContextGroupClass::DiagnosticsTests,
        "diffs_history" => ContextGroupClass::DiffsHistory,
        "runtime_artifacts" => ContextGroupClass::RuntimeArtifacts,
        "instruction_sources" => ContextGroupClass::InstructionSources,
        "external_tool_results" => ContextGroupClass::ExternalToolResults,
        other => panic!("unknown context group token {other}"),
    }
}

/// Builds a typed composer context row from the corpus membership label. The
/// labeling rules are the contract under test: in-scope rows are drawn (no
/// omission reason), out-of-scope rows are blocked + outside-current-scope +
/// scope_excluded, and policy-limited rows are blocked + policy.
fn build_context_item(raw: &CaseContextItem) -> ComposerContextItem {
    let (state_class, locality_class, omission_reason_class, trust_class) =
        match raw.membership.as_str() {
            "in_scope" => (
                ContextItemStateClass::Included,
                ContextLocalityClass::LocalWorkspace,
                None,
                ContextTrustClass::TrustedFirstParty,
            ),
            "out_of_scope" => (
                ContextItemStateClass::Blocked,
                ContextLocalityClass::OutsideCurrentScope,
                Some(ContextOmissionReasonClass::ScopeExcluded),
                ContextTrustClass::TrustedFirstParty,
            ),
            "policy_limited" => (
                ContextItemStateClass::Blocked,
                ContextLocalityClass::OutsideCurrentScope,
                Some(ContextOmissionReasonClass::Policy),
                ContextTrustClass::PolicyQuarantined,
            ),
            other => panic!("unknown membership token {other}"),
        };
    ComposerContextItem {
        context_item_id: raw.context_item_id.clone(),
        group_class: group_from_token(&raw.group),
        state_class,
        source_class: SourceClass::WorkspaceFileSlice,
        stable_identity_ref: raw.stable_identity_ref.clone(),
        display_label: raw.display_label.clone(),
        freshness_class: ContextFreshnessClass::AuthoritativeLive,
        trust_class,
        locality_class,
        estimated_byte_size: raw.estimated_byte_size,
        omission_reason_class,
        source_attachment_ref: None,
        source_mention_ref: None,
        docs_identity: None,
    }
}

fn context_snapshot_ref(stable_scope_id: &str) -> String {
    format!("context-snapshot:{stable_scope_id}")
}

fn workflow_id(stable_scope_id: &str) -> String {
    format!("workflow.ai.workset_scope.{stable_scope_id}")
}

/// Projects the AI composer / context-inspector snapshot for a case, with the
/// scope label derived from the shared `ScopeClass` vocabulary and the review
/// lock bound to the scope-stable context-snapshot ref.
fn project_snapshot(case: &Case) -> ComposerContextAlphaSnapshot {
    let scope_class = scope_class_from_token(&case.workspace_scope_class);
    let scope_label = derive_scope_label(scope_class, &case.artifact.workset_name);
    let stable_scope_id = case.artifact.stable_scope_id().to_owned();
    let draft = ComposerDraft::new(
        format!("turn-draft:{stable_scope_id}"),
        format!("composer-session:{stable_scope_id}"),
        "ws:aureline",
        "Work within the active workset scope.",
    );
    let routing = routing_packet(&workflow_id(&stable_scope_id));
    let context_items: Vec<ComposerContextItem> =
        case.context_items.iter().map(build_context_item).collect();
    ComposerContextAlphaSnapshot::project(
        &draft,
        &routing,
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::DraftPatch,
            scope_label,
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: Some("cmd:ai.draft_patch".to_owned()),
            mention_previews: Vec::new(),
            attachment_pills: Vec::new(),
            context_items,
            graph_cue_packets: Vec::new(),
            review_lock: ComposerContextReviewLock {
                lock_class: ReviewLockClass::FrozenForReview,
                context_snapshot_ref: context_snapshot_ref(&stable_scope_id),
                route_snapshot_ref: format!("route-snapshot:{stable_scope_id}"),
                review_started_at: Some("2026-05-21T00:00:00Z".to_owned()),
            },
        },
    )
}

#[test]
fn every_case_keeps_in_scope_context_and_labels_out_of_scope_and_policy_limited() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    assert!(!cases.is_empty(), "corpus must declare at least one case");

    for case in &cases {
        let scope_class = scope_class_from_token(&case.workspace_scope_class);
        assert_eq!(
            case.artifact.scope_class, scope_class,
            "artifact scope_class disagrees with declared workspace_scope_class in {}",
            case.case_id
        );

        // The AI scope label is derived from the shared workspace vocabulary.
        let scope_label = derive_scope_label(scope_class, &case.artifact.workset_name);
        assert_eq!(
            scope_label, case.expect.ai_scope_label,
            "scope label mismatch in {}",
            case.case_id
        );

        // The canonical scope artifact validates and projects an AI scope truth.
        case.artifact
            .validate()
            .unwrap_or_else(|err| panic!("artifact in {} must validate: {err}", case.case_id));
        let observation = ScopeObservationInputs {
            workspace_root_refs: &case.workspace.workspace_root_refs,
            workspace_root_labels: &case.workspace.workspace_root_labels,
            parent_artifact: None,
        };
        let truth: WorksetScopeBetaTruth = case.artifact.project_beta_truth(
            BetaConsumerSurface::Ai,
            observation,
            "mono:ai:beta:test",
        );
        truth.validate().unwrap_or_else(|err| {
            panic!("ai scope truth must validate in {}: {err}", case.case_id)
        });
        assert_eq!(
            truth.scope_class, scope_class,
            "ai scope truth scope_class mismatch in {}",
            case.case_id
        );

        // The scope truth narrows or blocks ai_apply for the active scope class.
        let ai_apply = truth
            .admission_for(BroadActionClass::AiApply)
            .expect("ai_apply admission must exist");
        assert_eq!(
            ai_apply.decision.as_str(),
            case.expect.ai_apply_decision.as_str(),
            "ai_apply decision mismatch in {}",
            case.case_id
        );

        // Out-of-scope workspace roots are labeled in the scope truth, never
        // silently dropped.
        for excluded in &case.expect.excluded_root_refs {
            assert!(
                truth.excluded_roots.iter().any(|entry| {
                    &entry.root_ref == excluded
                        && entry.reason == ExcludedRootReason::NotInWorksetRootList
                }),
                "excluded root {excluded} must be labeled in the scope truth in {}",
                case.case_id
            );
        }
        if case.expect.policy_hidden_excluded {
            assert!(
                truth
                    .excluded_roots
                    .iter()
                    .any(|entry| entry.reason == ExcludedRootReason::PolicyHidden),
                "policy-hidden members must be disclosed in the scope truth in {}",
                case.case_id
            );
        }

        // The composer / context-inspector snapshot draws only in-scope context
        // and labels everything else.
        let snapshot = project_snapshot(case);
        assert!(
            snapshot.validate().is_empty(),
            "composer context snapshot must validate in {}: {:?}",
            case.case_id,
            snapshot.validate()
        );
        assert_eq!(
            snapshot.scope_label, case.expect.ai_scope_label,
            "snapshot scope label mismatch in {}",
            case.case_id
        );

        let mut in_scope = 0_usize;
        let mut out_of_scope = 0_usize;
        let mut policy_limited = 0_usize;
        for item in &snapshot.context_items {
            match item.state_class {
                ContextItemStateClass::Included | ContextItemStateClass::Pinned => {
                    assert!(
                        item.omission_reason_class.is_none(),
                        "in-scope context {} must not carry an omission reason in {}",
                        item.context_item_id,
                        case.case_id
                    );
                    assert_ne!(
                        item.locality_class,
                        ContextLocalityClass::OutsideCurrentScope,
                        "in-scope context {} must not be outside current scope in {}",
                        item.context_item_id,
                        case.case_id
                    );
                    in_scope += 1;
                }
                ContextItemStateClass::Blocked => {
                    // Every blocked (escaping) row carries an explicit label and
                    // is marked outside the current scope.
                    let reason = item.omission_reason_class.unwrap_or_else(|| {
                        panic!(
                            "blocked context {} must carry an omission reason in {}",
                            item.context_item_id, case.case_id
                        )
                    });
                    assert_eq!(
                        item.locality_class,
                        ContextLocalityClass::OutsideCurrentScope,
                        "blocked context {} must be outside current scope in {}",
                        item.context_item_id,
                        case.case_id
                    );
                    match reason {
                        ContextOmissionReasonClass::ScopeExcluded => out_of_scope += 1,
                        ContextOmissionReasonClass::Policy => policy_limited += 1,
                        other => panic!(
                            "unexpected blocked-context reason {} in {}",
                            other.as_str(),
                            case.case_id
                        ),
                    }
                }
                other => panic!(
                    "unexpected context state {} in {}",
                    other.as_str(),
                    case.case_id
                ),
            }
        }
        assert_eq!(
            in_scope, case.expect.in_scope_count,
            "in-scope context count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            out_of_scope, case.expect.out_of_scope_count,
            "out-of-scope context count mismatch in {}",
            case.case_id
        );
        assert_eq!(
            policy_limited, case.expect.policy_limited_count,
            "policy-limited context count mismatch in {}",
            case.case_id
        );

        // The evidence handoff preserves the labeled out-of-scope / policy rows.
        let stable_scope_id = case.artifact.stable_scope_id().to_owned();
        let handoff = snapshot.evidence_handoff(format!("handoff:{stable_scope_id}"));
        assert!(
            handoff.validate().is_empty(),
            "evidence handoff must validate in {}: {:?}",
            case.case_id,
            handoff.validate()
        );
        assert_eq!(
            handoff.composer_context_snapshot_ref,
            context_snapshot_ref(&stable_scope_id),
            "evidence handoff must reference the scope-bound context snapshot in {}",
            case.case_id
        );
        let survivors: BTreeSet<&str> = handoff
            .context_rows
            .iter()
            .map(|row| row.context_item_id.as_str())
            .collect();
        for expected in &case.expect.evidence_survivor_ids {
            assert!(
                survivors.contains(expected.as_str()),
                "labeled row {expected} must survive into the evidence handoff in {}",
                case.case_id
            );
            let row = handoff
                .context_rows
                .iter()
                .find(|row| &row.context_item_id == expected)
                .expect("survivor row resolves");
            assert!(
                row.omission_reason_token.is_some(),
                "evidence row {expected} must keep its omission-reason label in {}",
                case.case_id
            );
            assert_eq!(
                row.locality_token,
                ContextLocalityClass::OutsideCurrentScope.as_str(),
                "evidence row {expected} must keep its outside-scope locality in {}",
                case.case_id
            );
        }

        // The spend receipt is attributed to the same scope-bound context
        // assembly, so spend truth reflects the active scope.
        let routing = routing_packet(&workflow_id(&stable_scope_id));
        let spend = SpendReceiptRecord::from_routing_packet(
            &routing,
            format!("spend:{stable_scope_id}"),
            format!("route-receipt:{stable_scope_id}"),
            context_snapshot_ref(&stable_scope_id),
            "2026-05-21T00:00:00Z",
        );
        let spend_violations = spend.validate_against_routing_packet(&routing);
        assert!(
            spend_violations.is_empty(),
            "spend receipt must validate in {}: {:?}",
            case.case_id,
            spend_violations
        );
        assert_eq!(
            spend.assembly_id_ref, handoff.composer_context_snapshot_ref,
            "spend receipt must cite the scope-bound context assembly in {}",
            case.case_id
        );
        assert!(
            spend.workflow_or_surface_id.contains(&stable_scope_id),
            "spend receipt workflow must carry the active scope id in {}",
            case.case_id
        );
    }
}

#[test]
fn unlabeled_out_of_scope_context_item_fails_validation() {
    let case = load_case("named_workset_scope.json");
    let mut snapshot = project_snapshot(&case);
    // Injecting an out-of-scope context row without a scope label must fail the
    // pre-send review — an out-of-scope item can never pass silently.
    snapshot.context_items.push(ComposerContextItem {
        context_item_id: "ctx:unlabeled_out_of_scope".to_owned(),
        group_class: ContextGroupClass::OpenFiles,
        state_class: ContextItemStateClass::Blocked,
        source_class: SourceClass::WorkspaceFileSlice,
        stable_identity_ref: "file:other-root/src/lib.rs".to_owned(),
        display_label: "other-root/src/lib.rs".to_owned(),
        freshness_class: ContextFreshnessClass::AuthoritativeLive,
        trust_class: ContextTrustClass::TrustedFirstParty,
        locality_class: ContextLocalityClass::OutsideCurrentScope,
        estimated_byte_size: 512,
        omission_reason_class: None,
        source_attachment_ref: None,
        source_mention_ref: None,
        docs_identity: None,
    });
    let violations = snapshot.validate();
    assert!(
        violations.contains(&ComposerContextAlphaViolation::OmittedContextReasonMissing),
        "unlabeled out-of-scope context item must fail validation; got {violations:?}"
    );
}

#[test]
fn corpus_covers_named_workset_sparse_slice_and_policy_limited_view() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    let covered: BTreeSet<&str> = cases
        .iter()
        .map(|case| case.workspace_scope_class.as_str())
        .collect();
    for required in &manifest.required_scope_classes {
        assert!(
            covered.contains(required.as_str()),
            "corpus is missing a case for required scope class {required}"
        );
    }
    for required in ["selected_workset", "sparse_slice", "policy_limited_view"] {
        assert!(
            covered.contains(required),
            "corpus must cover scope class {required}"
        );
    }
}

#[test]
fn ai_scope_classes_reuse_the_workspace_scope_vocabulary() {
    let manifest = load_manifest();

    // The canonical workspace vocabulary, read directly from the dependency.
    let workspace_vocab: BTreeSet<String> = WORKSPACE_SCOPE_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();

    // The manifest mirror of the workspace vocabulary agrees with crate source.
    let mirror: BTreeSet<String> = manifest
        .workspace_scope_class_vocabulary
        .iter()
        .cloned()
        .collect();
    assert_eq!(
        mirror, workspace_vocab,
        "manifest workspace_scope_class_vocabulary drifted from aureline-workspace ScopeClass"
    );

    // Every graph token is either mapped to a workspace class or declared
    // AI-only — nothing is left unaccounted-for.
    let mut accounted: BTreeSet<String> = manifest
        .scope_class_vocabulary_map
        .keys()
        .cloned()
        .collect();
    for token in &manifest.ai_only_scope_classes {
        assert!(
            !manifest.scope_class_vocabulary_map.contains_key(token),
            "AI-only class {token} must not also appear in the mapping"
        );
        accounted.insert(token.clone());
    }

    // The mapping is injective: no two AI classes collapse onto one workspace
    // class.
    let mapped_values: Vec<&String> = manifest.scope_class_vocabulary_map.values().collect();
    let unique_values: BTreeSet<&String> = mapped_values.iter().copied().collect();
    assert_eq!(
        mapped_values.len(),
        unique_values.len(),
        "scope-class mapping must be injective"
    );

    // The mapped workspace classes are exactly the workspace ScopeClass
    // vocabulary — a 1:1 bijection over the shared scope vocabulary.
    let mapped_set: BTreeSet<String> = manifest
        .scope_class_vocabulary_map
        .values()
        .cloned()
        .collect();
    assert_eq!(
        mapped_set, workspace_vocab,
        "mapped scope classes must cover the aureline-workspace ScopeClass vocabulary exactly"
    );

    // Every mapped value resolves to a real ScopeClass and round-trips.
    for value in manifest.scope_class_vocabulary_map.values() {
        assert_eq!(
            scope_class_from_token(value).as_str(),
            value,
            "mapped workspace class {value} must round-trip through ScopeClass"
        );
    }

    // Each case's declared workspace scope class agrees with the shared map, so
    // a case cannot quote a private label divergent from the vocabulary.
    for case in load_cases(&manifest) {
        let mapped = manifest
            .scope_class_vocabulary_map
            .get(&case.ai_scope_class)
            .unwrap_or_else(|| {
                panic!(
                    "case {} uses unmapped ai scope class {}",
                    case.case_id, case.ai_scope_class
                )
            });
        assert_eq!(
            mapped, &case.workspace_scope_class,
            "case {} workspace_scope_class disagrees with the vocabulary map",
            case.case_id
        );
    }
}

#[allow(clippy::too_many_lines)]
fn routing_packet(workflow_or_surface_id: &str) -> AiRoutingPacket {
    let quota = QuotaInspector {
        quota_family_class: QuotaFamilyClass::VendorHostedEntitlementQuota,
        quota_state_class: QuotaStateClass::WithinLimit,
        quota_scope_class: QuotaScopeClass::VendorHostedEntitlement,
        budget_owner_ref: "quota-owner:ai:workset-scope".to_owned(),
        quota_meter_ref: Some("quota-meter:ai:workset-scope".to_owned()),
        quota_forecast_ref: Some("quota-forecast:ai:workset-scope".to_owned()),
        usage_export_ref: Some("usage-export:ai:workset-scope".to_owned()),
        explanation_label: "Hosted AI route is available.".to_owned(),
        local_continuity_label: "Local review stays available without hosted AI.".to_owned(),
        recovery_action_ref: Some("action:ai:view-quota".to_owned()),
    };
    let envelope = LatencyCostEnvelope {
        latency_envelope_class: LatencyEnvelopeClass::StreamingFirstTokenUnder500Ms,
        cost_envelope_class: CostEnvelopeClass::VendorHostedEntitlementBand,
        cost_visibility_class: CostVisibilityClass::BundledNoIncrementalCost,
        token_ceiling_class: TokenCeilingClass::TokensUnder32K,
        tool_call_ceiling_class: ToolCallCeilingClass::BoundedToolCallsUnder4,
        wall_time_ceiling_class: WallTimeCeilingClass::WallTimeUnder30S,
        budget_routing_policy_ref: "budget-policy:ai:workset-scope".to_owned(),
        graduation_packet_ref: "graduation-packet:ai:workset-scope".to_owned(),
        envelope_evidence_ref: "envelope-evidence:ai:workset-scope".to_owned(),
        explanation_label: "Route uses the hosted preview band.".to_owned(),
    };
    let candidate = AiRouteCandidate {
        candidate_id: "candidate:hosted:ai:workset-scope".to_owned(),
        provider_entry_ref: "provider-entry:first-party:ai:workset-scope".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartyManaged,
        model_entry_ref: "model-entry:hosted:ai:workset-scope".to_owned(),
        model_label: "Hosted context review preview".to_owned(),
        execution_locus_class: ExecutionLocusClass::VendorHostedFirstPartyManaged,
        route_origin_class: RouteOriginClass::VendorHostedManaged,
        region_posture_class: RegionPostureClass::SingleRegionPinned,
        retention_stance_class: RetentionStanceClass::NoRetentionPromisedBodyDiscarded,
        quota,
        envelope,
        route_selection_reason_class: RouteSelectionReasonClass::NoCheaperQualifyingRouteExisted,
        route_selection_override_reason_class:
            RouteSelectionOverrideReasonClass::NoOverrideCheapestWasUsed,
        exhaustion_state_class: ExhaustionStateClass::NotExhaustedRouteAdmitted,
        selected_outcome_class: SelectedOutcomeClass::SelectedThisPath,
        route_selection_disclosure_ref: None,
        originating_approval_ticket_ref: Some("approval-ticket:ai:workset-scope".to_owned()),
        explanation_label: "Hosted route selected for context review.".to_owned(),
    };

    AiRoutingPacket::new(
        format!("ai_routing_packet:{workflow_or_surface_id}"),
        workflow_or_surface_id.to_owned(),
        "ws:aureline",
        RoutingRunStateClass::PreviewPreDispatch,
        RoutingPolicyContext {
            policy_epoch_ref: "policy-epoch:ai:workset-scope".to_owned(),
            trust_state: PolicyTrustState::Trusted,
            deployment_profile_class: DeploymentProfileClass::ManagedCloud,
            execution_context_ref: Some("execution-context:ai:workset-scope".to_owned()),
        },
        "capability-lifecycle:ai.workset-scope",
        None,
        vec![candidate.clone()],
        candidate.candidate_id,
        Vec::new(),
        vec!["docs/ai/context_assembly_contract.md".to_owned()],
        "2026-05-21T00:00:00Z",
    )
}
