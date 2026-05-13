use std::collections::BTreeMap;
use std::path::Path;

use aureline_language::{
    CodeActionAdmissionRecord, CodeActionAlphaAggregateCounts, CodeActionAlphaSnapshot,
    CodeActionCatalog, CodeActionClass, CodeActionContentIntegrityReview, CodeActionEpochBinding,
    CodeActionEpochRoleClass, CodeActionMutationCounts, CodeActionMutationScopeClass,
    CodeActionPolicyContext, CodeActionPreviewRequirementClass, CodeActionProviderDescriptor,
    CodeActionRecord, CodeActionReplayHintClass, CodeActionSafetyClass, CodeActionSideEffectClass,
    CodeActionSnapshotRequest, CodeActionSurfaceClass, CodeActionTrustState, CodeActionUndoGroup,
    CodeActionValidationHintClass, CodeActionValidationPlan, DiagnosticEvidencePlaneClass,
    DiagnosticFreshness, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticSourceDescriptor, DiagnosticSourceFamily, RedactionClass, RouterLocalityClass,
    RouterSupportClass, CODE_ACTION_ALPHA_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    workspace_id: String,
    collection_id: String,
    snapshot_id: String,
    execution_context_id: String,
    policy_epoch: String,
    captured_at: String,
    provider_sources: Vec<ProviderSourceCase>,
    actions: Vec<ActionCase>,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct ProviderSourceCase {
    provider_key: String,
    source_family: DiagnosticSourceFamily,
    evidence_plane_class: DiagnosticEvidencePlaneClass,
    origin_class: DiagnosticOriginClass,
    producer_ref: String,
    producer_version_ref: Option<String>,
    provider_id: String,
    support_class: RouterSupportClass,
    locality_class: RouterLocalityClass,
    freshness_class: DiagnosticFreshnessClass,
    semantic_layer_state_class: aureline_language::CodeActionSemanticLayerStateClass,
    provider_display_label: String,
    epoch_role_class: CodeActionEpochRoleClass,
    epoch_ref: String,
}

#[derive(Debug, Deserialize)]
struct ActionCase {
    case_id: String,
    code_action_id: String,
    action_class: CodeActionClass,
    action_label: String,
    provider_key: String,
    triggering_diagnostic_refs: Vec<String>,
    side_effect_class: CodeActionSideEffectClass,
    safety_class: CodeActionSafetyClass,
    mutation_scope_class: CodeActionMutationScopeClass,
    preview_requirement_class: CodeActionPreviewRequirementClass,
    apply_posture_class: aureline_language::CodeActionApplyPostureClass,
    blocking_reason_classes: Vec<aureline_language::CodeActionBlockingReasonClass>,
    mutation_counts: CodeActionMutationCounts,
    validation_hint_classes: Vec<CodeActionValidationHintClass>,
    replay_hint_classes: Vec<CodeActionReplayHintClass>,
    undo_group: Option<CodeActionUndoGroup>,
    checkpoint_ref: Option<String>,
    review_packet_ref: Option<String>,
    content_integrity_review: CodeActionContentIntegrityReview,
    expected_preview_required: bool,
    expected_silent_apply_allowed: bool,
    expected_named_undo: bool,
}

#[derive(Debug, Deserialize)]
struct Expected {
    total_count: usize,
    quick_fix_count: usize,
    preview_required_count: usize,
    silent_apply_allowed_count: usize,
    blocked_count: usize,
    multi_file_or_configuration_refused_count: usize,
    generated_or_protected_count: usize,
    content_integrity_preview_count: usize,
    named_undo_group_count: usize,
    side_effect_disclosure_count: usize,
    editor_inline_apply_count: usize,
    editor_preview_required_count: usize,
    editor_blocked_count: usize,
    undo_group_ref_count: usize,
    disclosure_required: bool,
}

#[test]
fn code_actions_disclose_side_effects_preview_and_undo_groups() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "code_action_alpha_cases");
    assert_eq!(fixture.schema_version, CODE_ACTION_ALPHA_SCHEMA_VERSION);

    let providers = provider_descriptors(&fixture);
    let mut catalog = CodeActionCatalog::new();

    for case in &fixture.actions {
        let action = action_from_case(&fixture, case, &providers);
        assert_eq!(
            action.preview_required(),
            case.expected_preview_required,
            "preview requirement mismatch for {}",
            case.case_id
        );
        assert_eq!(
            action.silent_apply_allowed(),
            case.expected_silent_apply_allowed,
            "silent-apply admission mismatch for {}",
            case.case_id
        );
        assert_eq!(
            action.has_named_undo_group(),
            case.expected_named_undo,
            "undo group attribution mismatch for {}",
            case.case_id
        );

        let admission = action.admission();
        assert_eq!(
            admission.record_kind,
            CodeActionAdmissionRecord::RECORD_KIND
        );
        assert_eq!(admission.side_effect_class, action.side_effect_class);
        assert_eq!(admission.preview_required, action.preview_required());
        assert_eq!(
            admission.silent_apply_allowed,
            action.silent_apply_allowed()
        );
        if action.is_multi_file_or_configuration_changing() {
            assert!(
                !admission.silent_apply_allowed,
                "{} must refuse silent apply when broad or configuration-changing",
                case.case_id
            );
            assert!(
                !admission.refused_silent_apply_reason_refs.is_empty(),
                "{} should explain why silent apply was refused",
                case.case_id
            );
        }

        catalog.publish(action);
    }

    let snapshot = catalog.snapshot(CodeActionSnapshotRequest {
        snapshot_id: fixture.snapshot_id.clone(),
        workspace_id: fixture.workspace_id.clone(),
        collection_id: fixture.collection_id.clone(),
        captured_at: fixture.captured_at.clone(),
    });
    assert_eq!(snapshot.record_kind, CodeActionAlphaSnapshot::RECORD_KIND);
    assert_eq!(
        snapshot.aggregate_counts,
        CodeActionAlphaAggregateCounts {
            total_count: fixture.expected.total_count,
            quick_fix_count: fixture.expected.quick_fix_count,
            preview_required_count: fixture.expected.preview_required_count,
            silent_apply_allowed_count: fixture.expected.silent_apply_allowed_count,
            blocked_count: fixture.expected.blocked_count,
            multi_file_or_configuration_refused_count: fixture
                .expected
                .multi_file_or_configuration_refused_count,
            generated_or_protected_count: fixture.expected.generated_or_protected_count,
            content_integrity_preview_count: fixture.expected.content_integrity_preview_count,
            named_undo_group_count: fixture.expected.named_undo_group_count,
            side_effect_disclosure_count: fixture.expected.side_effect_disclosure_count,
        }
    );
    assert_eq!(
        snapshot.disclosure_required(),
        fixture.expected.disclosure_required
    );

    let editor_projection = snapshot.surface_projection(
        CodeActionSurfaceClass::EditorActionPicker,
        &fixture.captured_at,
    );
    assert_eq!(
        editor_projection.inline_apply_action_ids.len(),
        fixture.expected.editor_inline_apply_count
    );
    assert_eq!(
        editor_projection.preview_required_action_ids.len(),
        fixture.expected.editor_preview_required_count
    );
    assert_eq!(
        editor_projection.blocked_action_ids.len(),
        fixture.expected.editor_blocked_count
    );
    assert_eq!(
        editor_projection.undo_group_refs.len(),
        fixture.expected.undo_group_ref_count
    );
    assert!(editor_projection.disclosure_required);

    let support_projection =
        snapshot.surface_projection(CodeActionSurfaceClass::SupportExport, &fixture.captured_at);
    assert_eq!(
        support_projection.included_action_ids,
        editor_projection.included_action_ids
    );

    let serialized = serde_json::to_string(&snapshot).expect("snapshot serializes");
    let round_trip: CodeActionAlphaSnapshot =
        serde_json::from_str(&serialized).expect("snapshot deserializes");
    assert_eq!(round_trip, snapshot);
}

fn provider_descriptors(fixture: &Fixture) -> BTreeMap<String, CodeActionProviderDescriptor> {
    fixture
        .provider_sources
        .iter()
        .map(|case| {
            let source = DiagnosticSourceDescriptor {
                source_descriptor_id: format!("source:code_action:{}", case.provider_key),
                source_family: case.source_family,
                evidence_plane_class: case.evidence_plane_class,
                origin_class: case.origin_class,
                producer_ref: case.producer_ref.clone(),
                producer_version_ref: case.producer_version_ref.clone(),
                provider_id: Some(case.provider_id.clone()),
                router_host_ref: None,
                locality_class: case.locality_class,
                support_class: case.support_class,
                summary: format!(
                    "{} source descriptor feeds code-action alpha.",
                    case.provider_key
                ),
            };
            let freshness = DiagnosticFreshness {
                freshness_class: case.freshness_class,
                observed_at: fixture.captured_at.clone(),
                epoch_ref: Some(case.epoch_ref.clone()),
                invalidation_ref: None,
                summary: format!(
                    "{} source freshness is {:?}.",
                    case.provider_key, case.freshness_class
                ),
            };
            let provider = CodeActionProviderDescriptor::from_diagnostic_source(
                &source,
                &freshness,
                case.provider_display_label.clone(),
                case.semantic_layer_state_class,
                vec![CodeActionEpochBinding {
                    epoch_role_class: case.epoch_role_class,
                    epoch_ref: case.epoch_ref.clone(),
                }],
            )
            .unwrap_or_else(|err| panic!("build provider {}: {err}", case.provider_key));
            (case.provider_key.clone(), provider)
        })
        .collect()
}

fn action_from_case(
    fixture: &Fixture,
    case: &ActionCase,
    providers: &BTreeMap<String, CodeActionProviderDescriptor>,
) -> CodeActionRecord {
    let acting_provider = providers
        .get(&case.provider_key)
        .unwrap_or_else(|| panic!("missing provider {}", case.provider_key))
        .clone();
    CodeActionRecord {
        record_kind: CodeActionRecord::RECORD_KIND.into(),
        code_action_alpha_schema_version: CODE_ACTION_ALPHA_SCHEMA_VERSION,
        code_action_id: case.code_action_id.clone(),
        action_class: case.action_class,
        action_label: case.action_label.clone(),
        acting_provider,
        triggering_diagnostic_refs: case.triggering_diagnostic_refs.clone(),
        side_effect_class: case.side_effect_class,
        safety_class: case.safety_class,
        mutation_scope_class: case.mutation_scope_class,
        preview_requirement_class: case.preview_requirement_class,
        apply_posture_class: case.apply_posture_class,
        blocking_reason_classes: case.blocking_reason_classes.clone(),
        mutation_counts: case.mutation_counts.clone(),
        current_epoch_bindings: providers
            .get(&case.provider_key)
            .expect("provider exists")
            .current_epoch_bindings
            .clone(),
        validation_plan: CodeActionValidationPlan {
            validation_hint_classes: case.validation_hint_classes.clone(),
            replay_hint_classes: case.replay_hint_classes.clone(),
            validation_summary: format!("{} validation plan is explicit.", case.case_id),
        },
        undo_group: case.undo_group.clone(),
        checkpoint_ref: case.checkpoint_ref.clone(),
        review_packet_ref: case.review_packet_ref.clone(),
        content_integrity_review: case.content_integrity_review.clone(),
        policy_context: CodeActionPolicyContext {
            policy_epoch: fixture.policy_epoch.clone(),
            trust_state: CodeActionTrustState::Trusted,
            execution_context_id: fixture.execution_context_id.clone(),
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        captured_at: fixture.captured_at.clone(),
        export_safe_summary: format!("{} code action preserves apply truth.", case.case_id),
    }
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/code_action_alpha/action_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
