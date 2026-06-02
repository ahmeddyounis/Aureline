//! Unit tests for the workset / scope UX lineage projection.

use super::*;

fn make_support_export() -> WorksetScopeUxSupportExportInputs {
    WorksetScopeUxSupportExportInputs::metadata_safe_baseline(
        SupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn scope(
    scope_id: &str,
    workset_ref: &str,
    workset_name: &str,
    scope_class: ScopeKind,
    readiness_state: ReadinessState,
    narrowing_cause: Option<NarrowingCause>,
    hidden_member_list_visible: bool,
    hidden_count_known: bool,
    hidden_count: Option<u64>,
) -> ScopeObservation {
    ScopeObservation {
        scope_id: scope_id.to_owned(),
        workset_ref: workset_ref.to_owned(),
        workset_name: workset_name.to_owned(),
        scope_class,
        root_refs: vec!["fs-r-0".to_owned()],
        excluded_root_classes: Vec::new(),
        policy_limitation_ref: None,
        narrowing_cause,
        hidden_member_list_visible,
        readiness_state,
        hidden_result_count_known: hidden_count_known,
        hidden_result_count: hidden_count,
        widen_actions_offered: vec![
            WidenActionClass::WidenWithReview,
            WidenActionClass::WidenToFullWorkspace,
        ],
        support_export: make_support_export(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn surface(
    surface_id: &str,
    surface_kind: SurfaceKind,
    scope_id: &str,
    outside: bool,
    omitted: bool,
    policy_hidden: bool,
    discloses_hidden: bool,
    deep_link_slice: bool,
    export_slice: bool,
) -> SurfaceObservation {
    SurfaceObservation {
        surface_id: surface_id.to_owned(),
        title: surface_id.to_owned(),
        surface_kind,
        scope_id: scope_id.to_owned(),
        shows_outside_current_slice: outside,
        shows_omitted_path: omitted,
        shows_policy_hidden: policy_hidden,
        discloses_hidden_result_count: discloses_hidden,
        carries_slice_ref_into_deep_links: deep_link_slice,
        carries_slice_ref_into_export: export_slice,
        support_export: make_support_export(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

fn widen_preview(
    preview_id: &str,
    base_scope_id: &str,
    candidate_scope_id: &str,
) -> WidenPreviewObservation {
    WidenPreviewObservation {
        preview_id: preview_id.to_owned(),
        base_scope_id: base_scope_id.to_owned(),
        candidate_scope_id: candidate_scope_id.to_owned(),
        previews_hidden_result_count: true,
        previews_omitted_root_classes: true,
        previews_fetch_deepen_implications: true,
        previews_blame_history_search_consequences: true,
        preserves_root_identity: true,
        preserves_query_session_continuity: true,
        preserves_restore_provenance: true,
        preservation_posture: WidenPreservationPosture::PreservesIdentityAndContinuity,
        apply_action_id: format!("action.{preview_id}.apply"),
        apply_disclosure_id: format!("disclosure.{preview_id}.apply"),
        support_export: make_support_export(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

fn baseline_inputs() -> WorksetScopeUxInputs {
    let scopes = vec![
        scope(
            "scope.named_workset.0",
            "wks.named_workset.0",
            "Hot Path",
            ScopeKind::SelectedWorkset,
            ReadinessState::Warm,
            None,
            false,
            true,
            Some(3),
        ),
        scope(
            "scope.sparse_slice.0",
            "wks.sparse_slice.0",
            "Service Sparse Slice",
            ScopeKind::SparseSlice,
            ReadinessState::Warm,
            None,
            false,
            true,
            Some(7),
        ),
        scope(
            "scope.policy_limited.0",
            "wks.policy_limited.0",
            "Sandbox Policy-Limited View",
            ScopeKind::PolicyLimitedView,
            ReadinessState::Ready,
            Some(NarrowingCause::TrustPolicy),
            false,
            true,
            Some(2),
        ),
        scope(
            "scope.full_workspace.0",
            "wks.full_workspace.0",
            "Full Workspace",
            ScopeKind::FullWorkspace,
            ReadinessState::Ready,
            None,
            false,
            true,
            Some(0),
        ),
    ];
    let surfaces = vec![
        surface(
            "surface.workset_switcher",
            SurfaceKind::WorksetSwitcher,
            "scope.named_workset.0",
            false,
            false,
            false,
            true,
            true,
            false,
        ),
        surface(
            "surface.scope_chip",
            SurfaceKind::ScopeChip,
            "scope.sparse_slice.0",
            false,
            false,
            false,
            true,
            true,
            false,
        ),
        surface(
            "surface.search",
            SurfaceKind::Search,
            "scope.sparse_slice.0",
            true,
            true,
            true,
            true,
            true,
            false,
        ),
        surface(
            "surface.tree",
            SurfaceKind::Tree,
            "scope.sparse_slice.0",
            true,
            true,
            true,
            true,
            true,
            false,
        ),
        surface(
            "surface.graph",
            SurfaceKind::Graph,
            "scope.sparse_slice.0",
            true,
            true,
            true,
            true,
            true,
            false,
        ),
        surface(
            "surface.review",
            SurfaceKind::Review,
            "scope.policy_limited.0",
            true,
            true,
            true,
            true,
            true,
            false,
        ),
        surface(
            "surface.support_export",
            SurfaceKind::SupportExport,
            "scope.full_workspace.0",
            false,
            false,
            false,
            true,
            true,
            true,
        ),
    ];
    let widen_previews = vec![
        widen_preview(
            "preview.sparse_to_full",
            "scope.sparse_slice.0",
            "scope.full_workspace.0",
        ),
        widen_preview(
            "preview.policy_limited_to_workset",
            "scope.policy_limited.0",
            "scope.named_workset.0",
        ),
    ];

    WorksetScopeUxInputs {
        workspace_ref: "workspace-rust-service-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "workset-scope-ux-corpus-0001".to_owned(),
        captured_at: "mono:1700000700".to_owned(),
        scopes,
        surfaces,
        widen_previews,
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_workset_scope_ux_lineage("posture.clean", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF);
    assert!(record.scope_coverage.all_required_scope_classes_present);
    assert!(record.surface_coverage.all_required_surface_kinds_present);
    assert!(
        record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_outside_current_slice
    );
    assert!(
        record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_omitted_path
    );
    assert!(
        record
            .outside_marker_honesty
            .all_result_bearing_surfaces_show_policy_hidden
    );
    assert!(
        record
            .hidden_result_disclosure
            .all_surfaces_disclose_hidden_count
    );
    assert!(
        record
            .slice_ref_propagation
            .all_surfaces_carry_slice_ref_into_deep_links
    );
    assert!(
        record
            .slice_ref_propagation
            .all_export_propagating_surfaces_carry_slice_ref_into_export
    );
    assert!(record.widen_preview_truth.all_previews_have_required_fields);
    assert!(
        record
            .widen_preview_truth
            .all_previews_have_apply_action_metadata
    );
    assert!(
        record
            .widen_preservation_truth
            .all_widens_preserve_root_identity
    );
    assert!(
        record
            .widen_preservation_truth
            .all_widens_preserve_query_session_continuity
    );
    assert!(
        record
            .widen_preservation_truth
            .all_widens_preserve_restore_provenance
    );
    assert!(
        record
            .widen_preservation_truth
            .all_preservation_postures_safe
    );
    assert_eq!(
        record.policy_limited_disclosure.policy_limited_scope_count,
        1
    );
    assert!(
        record
            .policy_limited_disclosure
            .all_policy_limited_have_narrowing_cause
    );
    assert!(
        record
            .policy_limited_disclosure
            .no_admin_or_license_policy_exposes_hidden_list
    );
    assert!(
        record
            .readiness_truth
            .all_ready_scopes_disclose_hidden_count_known
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("wsx:"));
}

#[test]
fn missing_required_scope_class_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .scopes
        .retain(|s| s.scope_class != ScopeKind::SparseSlice);
    let record = project_workset_scope_ux_lineage("posture.missing_sparse_class", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::RequiredScopeClassMissing));
}

#[test]
fn missing_required_surface_kind_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .surfaces
        .retain(|s| s.surface_kind != SurfaceKind::Graph);
    let record = project_workset_scope_ux_lineage("posture.missing_graph", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::RequiredSurfaceKindMissing));
}

#[test]
fn surface_referencing_unknown_scope_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.surfaces[2].scope_id = "scope.nonexistent".to_owned();
    let record = project_workset_scope_ux_lineage("posture.unknown_scope", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::SurfaceReferencesUnknownScope));
}

#[test]
fn result_bearing_surface_missing_outside_marker_narrows_record() {
    let mut inputs = baseline_inputs();
    let search = inputs
        .surfaces
        .iter_mut()
        .find(|s| s.surface_kind == SurfaceKind::Search)
        .expect("seeded");
    search.shows_outside_current_slice = false;
    let record = project_workset_scope_ux_lineage("posture.no_outside_marker", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::OutsideMarkerMissing));
}

#[test]
fn result_bearing_surface_missing_omitted_marker_narrows_record() {
    let mut inputs = baseline_inputs();
    let tree = inputs
        .surfaces
        .iter_mut()
        .find(|s| s.surface_kind == SurfaceKind::Tree)
        .expect("seeded");
    tree.shows_omitted_path = false;
    let record = project_workset_scope_ux_lineage("posture.no_omitted_marker", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::OmittedMarkerMissing));
}

#[test]
fn result_bearing_surface_missing_policy_hidden_marker_narrows_record() {
    let mut inputs = baseline_inputs();
    let review = inputs
        .surfaces
        .iter_mut()
        .find(|s| s.surface_kind == SurfaceKind::Review)
        .expect("seeded");
    review.shows_policy_hidden = false;
    let record = project_workset_scope_ux_lineage("posture.no_policy_hidden_marker", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::PolicyHiddenMarkerMissing));
}

#[test]
fn surface_without_hidden_count_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.surfaces[0].discloses_hidden_result_count = false;
    let record = project_workset_scope_ux_lineage("posture.no_hidden_count", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::HiddenResultCountNotDisclosed));
}

#[test]
fn surface_without_deep_link_slice_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.surfaces[0].carries_slice_ref_into_deep_links = false;
    let record = project_workset_scope_ux_lineage("posture.no_deep_link_slice", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::SliceRefNotPropagatedIntoDeepLinks));
}

#[test]
fn export_surface_without_export_slice_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    let support = inputs
        .surfaces
        .iter_mut()
        .find(|s| s.surface_kind == SurfaceKind::SupportExport)
        .expect("seeded");
    support.carries_slice_ref_into_export = false;
    let record = project_workset_scope_ux_lineage("posture.no_export_slice", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::SliceRefNotPropagatedIntoExport));
}

#[test]
fn widen_preview_missing_field_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].previews_fetch_deepen_implications = false;
    let record = project_workset_scope_ux_lineage("posture.no_fetch_deepen_preview", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::WidenPreviewFieldMissing));
}

#[test]
fn widen_losing_root_identity_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].preserves_root_identity = false;
    let record = project_workset_scope_ux_lineage("posture.widen_loses_identity", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::WidenLosesRootIdentity));
}

#[test]
fn widen_losing_query_session_continuity_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].preserves_query_session_continuity = false;
    let record = project_workset_scope_ux_lineage("posture.widen_loses_query", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::WidenLosesQuerySessionContinuity));
}

#[test]
fn widen_losing_restore_provenance_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].preserves_restore_provenance = false;
    let record = project_workset_scope_ux_lineage("posture.widen_loses_restore", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::WidenLosesRestoreProvenance));
}

#[test]
fn policy_limited_without_narrowing_cause_narrows_record() {
    let mut inputs = baseline_inputs();
    let policy = inputs
        .scopes
        .iter_mut()
        .find(|s| s.scope_class == ScopeKind::PolicyLimitedView)
        .expect("seeded");
    policy.narrowing_cause = None;
    let record = project_workset_scope_ux_lineage("posture.no_narrowing_cause", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::PolicyLimitedNarrowingCauseMissing));
}

#[test]
fn policy_limited_admin_exposing_hidden_list_narrows_record() {
    let mut inputs = baseline_inputs();
    let policy = inputs
        .scopes
        .iter_mut()
        .find(|s| s.scope_class == ScopeKind::PolicyLimitedView)
        .expect("seeded");
    policy.narrowing_cause = Some(NarrowingCause::AdminPolicy);
    policy.hidden_member_list_visible = true;
    let record = project_workset_scope_ux_lineage("posture.admin_hidden_list_exposed", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::PolicyAdminHiddenListExposed));
}

#[test]
fn ready_scope_with_unknown_hidden_count_narrows_record() {
    let mut inputs = baseline_inputs();
    let full = inputs
        .scopes
        .iter_mut()
        .find(|s| s.scope_class == ScopeKind::FullWorkspace)
        .expect("seeded");
    full.hidden_result_count_known = false;
    let record = project_workset_scope_ux_lineage("posture.unknown_hidden_count", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::ReadinessHiddenCountUnknown));
}

#[test]
fn apply_action_metadata_missing_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].apply_action_id = "".to_owned();
    let record = project_workset_scope_ux_lineage("posture.no_apply_action", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::ApplyActionMetadataMissing));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = baseline_inputs();
    let mut hooks = default_workset_scope_ux_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == WorksetScopeUxInspectionHookClass::PreviewWiden {
            hook.available = false;
        }
    }
    let record = project_workset_scope_ux_lineage_with_hooks(
        "posture.no_preview_widen_hook",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_dropping_field_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.scopes[0].support_export.includes_hidden_result_count = false;
    let record = project_workset_scope_ux_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn support_export_admin_hidden_leak_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.scopes[0].support_export.admin_hidden_list_excluded = false;
    let record = project_workset_scope_ux_lineage("posture.support_admin_hidden_leak", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref = "".to_owned();
    let record = project_workset_scope_ux_lineage("posture.no_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.scopes.clear();
    inputs.surfaces.clear();
    inputs.widen_previews.clear();
    let record = project_workset_scope_ux_lineage("posture.empty_corpus", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::CorpusEmpty));
}

#[test]
fn producer_attribution_incomplete_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref = "".to_owned();
    let record = project_workset_scope_ux_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn widen_preview_referencing_unknown_scope_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.widen_previews[0].candidate_scope_id = "scope.nonexistent".to_owned();
    let record = project_workset_scope_ux_lineage("posture.widen_unknown_scope", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&WorksetScopeUxLineageNarrowReason::WidenPreviewReferencesUnknownScope));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = baseline_inputs();
    let record = project_workset_scope_ux_lineage("posture.lines", &inputs);
    let lines = workset_scope_ux_lineage_lines(&record);
    assert!(lines
        .iter()
        .any(|line| line.contains("Workset/scope UX lineage")));
    assert!(lines.iter().any(|line| line.contains("scope_coverage")));
    assert!(lines.iter().any(|line| line == "Scopes:"));
    assert!(lines.iter().any(|line| line.contains("surface_coverage")));
    assert!(lines.iter().any(|line| line == "Surfaces:"));
    assert!(lines.iter().any(|line| line.contains("Outside-vs-omitted")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Hidden-result disclosure")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Slice-ref propagation")));
    assert!(lines
        .iter()
        .any(|line| line.contains("widen_preview_truth")));
    assert!(lines.iter().any(|line| line == "Widen previews:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Widen-preservation truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Policy-limited disclosure")));
    assert!(lines.iter().any(|line| line.contains("Readiness truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = baseline_inputs();
    let record = project_workset_scope_ux_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: WorksetScopeUxLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
