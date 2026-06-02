//! Fixture generator helper for the workset / scope UX lineage
//! replay gate.
//!
//! Only runs when
//! `WORKSET_SCOPE_UX_LINEAGE_GEN_FIXTURES=1` is set in the
//! environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/workset_scope_ux_lineage/` so the replay
//! gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_workset_scope_ux_inspection_hooks, project_workset_scope_ux_lineage_with_hooks,
    ScopeKind, ScopeObservation, SupportExportPosture, SurfaceObservation, WidenActionClass,
    WidenPreservationPosture, WidenPreviewObservation, WorksetScopeUxInputs,
    WorksetScopeUxInspectionHook, WorksetScopeUxInspectionHookClass, WorksetScopeUxLineageRecord,
    WorksetScopeUxNarrowingCause, WorksetScopeUxReadinessState, WorksetScopeUxSupportExportInputs,
    WorksetScopeUxSurfaceKind,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/workset_scope_ux_lineage")
}

fn safe_support() -> WorksetScopeUxSupportExportInputs {
    WorksetScopeUxSupportExportInputs::metadata_safe_baseline(
        SupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_scope(
    scope_id: &str,
    workset_ref: &str,
    workset_name: &str,
    scope_class: ScopeKind,
    root_refs: Vec<String>,
    excluded_root_classes: Vec<String>,
    policy_limitation_ref: Option<String>,
    narrowing_cause: Option<WorksetScopeUxNarrowingCause>,
    hidden_member_list_visible: bool,
    readiness_state: WorksetScopeUxReadinessState,
    hidden_count_known: bool,
    hidden_count: Option<u64>,
    widen_actions: Vec<WidenActionClass>,
    captured_at: &str,
) -> ScopeObservation {
    ScopeObservation {
        scope_id: scope_id.to_owned(),
        workset_ref: workset_ref.to_owned(),
        workset_name: workset_name.to_owned(),
        scope_class,
        root_refs,
        excluded_root_classes,
        policy_limitation_ref,
        narrowing_cause,
        hidden_member_list_visible,
        readiness_state,
        hidden_result_count_known: hidden_count_known,
        hidden_result_count: hidden_count,
        widen_actions_offered: widen_actions,
        support_export: safe_support(),
        captured_at: captured_at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn make_surface(
    surface_id: &str,
    title: &str,
    surface_kind: WorksetScopeUxSurfaceKind,
    scope_id: &str,
    outside: bool,
    omitted: bool,
    policy_hidden: bool,
    discloses_hidden: bool,
    deep_link_slice: bool,
    export_slice: bool,
    captured_at: &str,
) -> SurfaceObservation {
    SurfaceObservation {
        surface_id: surface_id.to_owned(),
        title: title.to_owned(),
        surface_kind,
        scope_id: scope_id.to_owned(),
        shows_outside_current_slice: outside,
        shows_omitted_path: omitted,
        shows_policy_hidden: policy_hidden,
        discloses_hidden_result_count: discloses_hidden,
        carries_slice_ref_into_deep_links: deep_link_slice,
        carries_slice_ref_into_export: export_slice,
        support_export: safe_support(),
        captured_at: captured_at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn make_widen_preview(
    preview_id: &str,
    base_scope_id: &str,
    candidate_scope_id: &str,
    posture: WidenPreservationPosture,
    captured_at: &str,
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
        preservation_posture: posture,
        apply_action_id: format!("action.{preview_id}.apply"),
        apply_disclosure_id: format!("disclosure.{preview_id}.apply"),
        support_export: safe_support(),
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_scopes(captured_at: &str) -> Vec<ScopeObservation> {
    vec![
        make_scope(
            "scope.named_workset.hot_path",
            "wks.named_workset.hot_path",
            "Hot Path",
            ScopeKind::SelectedWorkset,
            vec![
                "fs-r-service-api".to_owned(),
                "fs-r-service-core".to_owned(),
            ],
            vec!["vendor_directories".to_owned()],
            None,
            None,
            false,
            WorksetScopeUxReadinessState::Warm,
            true,
            Some(4),
            vec![
                WidenActionClass::WidenWithReview,
                WidenActionClass::WidenToFullWorkspace,
            ],
            captured_at,
        ),
        make_scope(
            "scope.sparse_slice.service_api",
            "wks.sparse_slice.service_api",
            "Service API Slice",
            ScopeKind::SparseSlice,
            vec!["fs-r-service-api".to_owned()],
            vec![
                "test_only_fixtures".to_owned(),
                "vendor_directories".to_owned(),
            ],
            None,
            None,
            false,
            WorksetScopeUxReadinessState::Warm,
            true,
            Some(12),
            vec![
                WidenActionClass::WidenWithReview,
                WidenActionClass::WidenToFullWorkspace,
            ],
            captured_at,
        ),
        make_scope(
            "scope.policy_limited.sandbox",
            "wks.policy_limited.sandbox",
            "Sandbox Policy-Limited View",
            ScopeKind::PolicyLimitedView,
            vec![
                "fs-r-service-api".to_owned(),
                "fs-r-service-core".to_owned(),
            ],
            vec!["sensitive_modules".to_owned()],
            Some("policy.sandbox.trust_overlay".to_owned()),
            Some(WorksetScopeUxNarrowingCause::TrustPolicy),
            false,
            WorksetScopeUxReadinessState::Ready,
            true,
            Some(5),
            vec![
                WidenActionClass::KeepCurrentScope,
                WidenActionClass::WidenWithReview,
            ],
            captured_at,
        ),
        make_scope(
            "scope.full_workspace.root",
            "wks.full_workspace.root",
            "Full Workspace",
            ScopeKind::FullWorkspace,
            vec![
                "fs-r-service-api".to_owned(),
                "fs-r-service-core".to_owned(),
                "fs-r-internal-lib".to_owned(),
            ],
            Vec::new(),
            None,
            None,
            false,
            WorksetScopeUxReadinessState::Ready,
            true,
            Some(0),
            vec![WidenActionClass::NarrowToCurrentRepo],
            captured_at,
        ),
    ]
}

fn baseline_surfaces(captured_at: &str) -> Vec<SurfaceObservation> {
    vec![
        make_surface(
            "surface.workset_switcher",
            "Workset switcher",
            WorksetScopeUxSurfaceKind::WorksetSwitcher,
            "scope.named_workset.hot_path",
            false,
            false,
            false,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.scope_chip",
            "Active scope chip",
            WorksetScopeUxSurfaceKind::ScopeChip,
            "scope.sparse_slice.service_api",
            false,
            false,
            false,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.search",
            "Search results",
            WorksetScopeUxSurfaceKind::Search,
            "scope.sparse_slice.service_api",
            true,
            true,
            true,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.tree",
            "Explorer tree",
            WorksetScopeUxSurfaceKind::Tree,
            "scope.sparse_slice.service_api",
            true,
            true,
            true,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.graph",
            "Dependency graph",
            WorksetScopeUxSurfaceKind::Graph,
            "scope.sparse_slice.service_api",
            true,
            true,
            true,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.review",
            "Review / diff",
            WorksetScopeUxSurfaceKind::Review,
            "scope.policy_limited.sandbox",
            true,
            true,
            true,
            true,
            true,
            false,
            captured_at,
        ),
        make_surface(
            "surface.support_export",
            "Support export header",
            WorksetScopeUxSurfaceKind::SupportExport,
            "scope.full_workspace.root",
            false,
            false,
            false,
            true,
            true,
            true,
            captured_at,
        ),
    ]
}

fn baseline_widen_previews(captured_at: &str) -> Vec<WidenPreviewObservation> {
    vec![
        make_widen_preview(
            "preview.sparse_to_full",
            "scope.sparse_slice.service_api",
            "scope.full_workspace.root",
            WidenPreservationPosture::PreservesIdentityAndContinuity,
            captured_at,
        ),
        make_widen_preview(
            "preview.policy_limited_to_workset",
            "scope.policy_limited.sandbox",
            "scope.named_workset.hot_path",
            WidenPreservationPosture::PreservedAfterReviewWithDisclosure,
            captured_at,
        ),
    ]
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    scopes: Vec<ScopeObservation>,
    surfaces: Vec<SurfaceObservation>,
    widen_previews: Vec<WidenPreviewObservation>,
) -> WorksetScopeUxInputs {
    WorksetScopeUxInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        scopes,
        surfaces,
        widen_previews,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a WorksetScopeUxInputs,
    inspection_hooks: &'a Vec<WorksetScopeUxInspectionHook>,
    expected: &'a WorksetScopeUxLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: WorksetScopeUxInputs,
    inspection_hooks: Vec<WorksetScopeUxInspectionHook>,
) {
    let record =
        project_workset_scope_ux_lineage_with_hooks(posture_id, &inputs, inspection_hooks.clone());
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("WORKSET_SCOPE_UX_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline Stable: every required scope class, every required UX
    // surface, every required widen-preview field present, every
    // widen preserves identity / query / restore.
    write_fixture(
        "baseline_workset_scope_ux_stable",
        "posture:baseline_workset_scope_ux",
        base_inputs(
            "workspace-rust-service-0001",
            "workset-scope-ux-corpus-baseline-0001",
            "mono:1700000700",
            baseline_scopes("mono:1700000700"),
            baseline_surfaces("mono:1700000700"),
            baseline_widen_previews("mono:1700000700"),
        ),
        default_workset_scope_ux_inspection_hooks(),
    );

    // Extended Stable: adds the optional `current_repo` scope plus
    // refactor-scope-footer, AI-context-inspector, export-scope-footer,
    // and deep-link-dispatcher surfaces. Still Stable.
    let mut extended_scopes = baseline_scopes("mono:1700000710");
    extended_scopes.push(make_scope(
        "scope.current_repo.service_api",
        "wks.current_repo.service_api",
        "Service API (current repo)",
        ScopeKind::CurrentRepo,
        vec!["fs-r-service-api".to_owned()],
        Vec::new(),
        None,
        None,
        false,
        WorksetScopeUxReadinessState::Warm,
        true,
        Some(2),
        vec![WidenActionClass::WidenToFullWorkspace],
        "mono:1700000710",
    ));
    let mut extended_surfaces = baseline_surfaces("mono:1700000710");
    extended_surfaces.push(make_surface(
        "surface.refactor_scope_footer",
        "Refactor scope footer",
        WorksetScopeUxSurfaceKind::RefactorScopeFooter,
        "scope.sparse_slice.service_api",
        false,
        false,
        false,
        true,
        true,
        false,
        "mono:1700000710",
    ));
    extended_surfaces.push(make_surface(
        "surface.ai_context_inspector",
        "AI context inspector",
        WorksetScopeUxSurfaceKind::AiContextInspector,
        "scope.named_workset.hot_path",
        false,
        false,
        false,
        true,
        true,
        false,
        "mono:1700000710",
    ));
    extended_surfaces.push(make_surface(
        "surface.export_scope_footer",
        "Export scope footer",
        WorksetScopeUxSurfaceKind::ExportScopeFooter,
        "scope.full_workspace.root",
        false,
        false,
        false,
        true,
        true,
        true,
        "mono:1700000710",
    ));
    extended_surfaces.push(make_surface(
        "surface.deep_link_dispatcher",
        "Deep-link dispatcher",
        WorksetScopeUxSurfaceKind::DeepLinkDispatcher,
        "scope.policy_limited.sandbox",
        false,
        false,
        false,
        false,
        true,
        true,
        "mono:1700000710",
    ));
    write_fixture(
        "extended_with_optional_surfaces_stable",
        "posture:extended_with_optional_surfaces",
        base_inputs(
            "workspace-rust-service-0001",
            "workset-scope-ux-corpus-extended-0001",
            "mono:1700000710",
            extended_scopes,
            extended_surfaces,
            baseline_widen_previews("mono:1700000710"),
        ),
        default_workset_scope_ux_inspection_hooks(),
    );

    // Stable: a `policy_limited_view` overlay carrying an admin
    // policy that correctly excludes the hidden member list and a
    // widen preview that preserves identity-and-continuity after
    // review.
    let mut admin_scopes = baseline_scopes("mono:1700000720");
    let policy_limited = admin_scopes
        .iter_mut()
        .find(|s| s.scope_class == ScopeKind::PolicyLimitedView)
        .expect("policy_limited seeded");
    policy_limited.narrowing_cause = Some(WorksetScopeUxNarrowingCause::AdminPolicy);
    policy_limited.hidden_member_list_visible = false;
    policy_limited.policy_limitation_ref = Some("policy.admin.export_control".to_owned());
    write_fixture(
        "policy_limited_admin_redacted_hidden_list_stable",
        "posture:policy_limited_admin_redacted_hidden_list",
        base_inputs(
            "workspace-rust-service-0001",
            "workset-scope-ux-corpus-admin-redacted-0001",
            "mono:1700000720",
            admin_scopes,
            baseline_surfaces("mono:1700000720"),
            baseline_widen_previews("mono:1700000720"),
        ),
        default_workset_scope_ux_inspection_hooks(),
    );

    // Narrowed: a widen preview drops the omitted-root-classes
    // disclosure so the preview cannot truthfully preview implications
    // before apply; the contract narrows with
    // `widen_preview_field_missing`.
    let mut narrowed_previews = baseline_widen_previews("mono:1700000730");
    narrowed_previews[0].previews_omitted_root_classes = false;
    write_fixture(
        "widen_preview_missing_omitted_root_classes_narrowed",
        "posture:widen_preview_missing_omitted_root_classes",
        base_inputs(
            "workspace-rust-service-0001",
            "workset-scope-ux-corpus-narrowed-preview-0001",
            "mono:1700000730",
            baseline_scopes("mono:1700000730"),
            baseline_surfaces("mono:1700000730"),
            narrowed_previews,
        ),
        default_workset_scope_ux_inspection_hooks(),
    );

    // Narrowed: required `preview_widen` inspection hook is
    // unavailable on this posture (degraded headless runner).
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "workset-scope-ux-corpus-narrowed-hook-0001",
        "mono:1700000740",
        baseline_scopes("mono:1700000740"),
        baseline_surfaces("mono:1700000740"),
        baseline_widen_previews("mono:1700000740"),
    );
    let mut narrowed_hooks = default_workset_scope_ux_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == WorksetScopeUxInspectionHookClass::PreviewWiden {
            hook.available = false;
            hook.disclosure = "Preview-widen unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_preview_widen_hook_narrowed",
        "posture:missing_preview_widen_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
