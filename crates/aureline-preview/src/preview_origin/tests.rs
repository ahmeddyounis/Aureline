use super::*;

fn sample_origin(class: PreviewOriginClass) -> PreviewOriginDescriptor {
    PreviewOriginDescriptor {
        record_kind: PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND.to_owned(),
        preview_origin_descriptor_schema_version: PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION,
        preview_origin_descriptor_id: "preview.origin.0001".to_owned(),
        observed_at: "2026-05-19T13:54:59Z".to_owned(),
        preview_lane_class: PreviewLaneClass::BrowserPreviewLane,
        origin_class: class,
        lifecycle_phase: match class {
            PreviewOriginClass::ImportedOrStaticEvidence => {
                PreviewOriginLifecyclePhase::NotApplicableStaticEvidence
            }
            _ => PreviewOriginLifecyclePhase::Running,
        },
        sharing_posture: PreviewOriginSharingPosture::LocalOnly,
        runtime_handle_ref: Some("runtime.handle.0001".to_owned()),
        exposure_record_ref: None,
        preview_snapshot_record_ref: Some("preview.snapshot.0001".to_owned()),
        managed_workspace_approval_ref: match class {
            PreviewOriginClass::ManagedPreviewService => Some("approval.0001".to_owned()),
            _ => None,
        },
        redacted_runtime_label: "Local dev server".to_owned(),
        summary: "Local dev server producing the current view.".to_owned(),
    }
}

fn sample_origin_local() -> PreviewOriginDescriptor {
    sample_origin(PreviewOriginClass::LocalDevServer)
}

fn sample_origin_remote() -> PreviewOriginDescriptor {
    let mut origin = sample_origin(PreviewOriginClass::RemoteOrContainerRuntime);
    origin.sharing_posture = PreviewOriginSharingPosture::SameDeviceOrLan;
    origin
}

fn sample_origin_managed() -> PreviewOriginDescriptor {
    let mut origin = sample_origin(PreviewOriginClass::ManagedPreviewService);
    origin.sharing_posture = PreviewOriginSharingPosture::AuthenticatedOrgRoute;
    origin.exposure_record_ref = Some("exposure.0001".to_owned());
    origin
}

fn sample_origin_static() -> PreviewOriginDescriptor {
    let mut origin = sample_origin(PreviewOriginClass::ImportedOrStaticEvidence);
    origin.sharing_posture = PreviewOriginSharingPosture::NotApplicableNoNetworkSurface;
    origin.runtime_handle_ref = None;
    origin
}

fn sample_target(class: PreviewTargetClass) -> PreviewTargetDescriptor {
    let participates = class.participates_in_browser_runtime();
    PreviewTargetDescriptor {
        record_kind: PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND.to_owned(),
        preview_target_descriptor_schema_version: PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION,
        preview_target_descriptor_id: "preview.target.0001".to_owned(),
        observed_at: "2026-05-19T13:54:59Z".to_owned(),
        preview_target_class: class,
        device_capability_class: DeviceCapabilityClass::DesktopDefault,
        reduced_capability_reason: PreviewTargetReducedCapabilityReason::None,
        preview_origin_descriptor_ref: "preview.origin.0001".to_owned(),
        device_target_descriptor_ref: None,
        browser_runtime_session_origin_ref: if participates {
            Some("browser.session.0001".to_owned())
        } else {
            None
        },
        viewport_pixel_width: Some(1440),
        viewport_pixel_height: Some(900),
        redacted_label: "Default viewport".to_owned(),
        summary: "Desktop default viewport on the design renderer.".to_owned(),
    }
}

fn sample_mapping(class: SourceMappingQualityClass) -> SourceMappingDescriptor {
    match class {
        SourceMappingQualityClass::Exact => SourceMappingDescriptor {
            source_mapping_quality_class: SourceMappingQualityClass::Exact,
            total_node_count: 12,
            exact_mapped_node_count: 12,
            unmappable_node_count: 0,
            summary: "All 12 nodes map exactly to canonical source.".to_owned(),
        },
        SourceMappingQualityClass::Heuristic => SourceMappingDescriptor {
            source_mapping_quality_class: SourceMappingQualityClass::Heuristic,
            total_node_count: 12,
            exact_mapped_node_count: 8,
            unmappable_node_count: 0,
            summary: "Heuristic mapping over an unminified bundle.".to_owned(),
        },
        SourceMappingQualityClass::Stale => SourceMappingDescriptor {
            source_mapping_quality_class: SourceMappingQualityClass::Stale,
            total_node_count: 12,
            exact_mapped_node_count: 12,
            unmappable_node_count: 0,
            summary: "Mapping is stale relative to the latest source change.".to_owned(),
        },
        SourceMappingQualityClass::Partial => SourceMappingDescriptor {
            source_mapping_quality_class: SourceMappingQualityClass::Partial,
            total_node_count: 12,
            exact_mapped_node_count: 9,
            unmappable_node_count: 3,
            summary: "Three nodes have no canonical-source mapping.".to_owned(),
        },
        SourceMappingQualityClass::Unavailable => SourceMappingDescriptor {
            source_mapping_quality_class: SourceMappingQualityClass::Unavailable,
            total_node_count: 0,
            exact_mapped_node_count: 0,
            unmappable_node_count: 0,
            summary: "Captured screenshot; no source mapping.".to_owned(),
        },
    }
}

fn sample_session(class: BrowserSessionOriginClass) -> BrowserRuntimeSessionOrigin {
    let admits = class.admits_inspection();
    let is_handoff = matches!(class, BrowserSessionOriginClass::ExternalHandoffBrowser);
    let is_no_session = matches!(class, BrowserSessionOriginClass::NoSessionAttached);

    BrowserRuntimeSessionOrigin {
        record_kind: BROWSER_SESSION_ORIGIN_RECORD_KIND.to_owned(),
        browser_runtime_session_origin_schema_version: BROWSER_SESSION_ORIGIN_SCHEMA_VERSION,
        browser_runtime_session_origin_id: "browser.session.0001".to_owned(),
        preview_origin_descriptor_ref: "preview.origin.0001".to_owned(),
        preview_target_descriptor_ref: "preview.target.0001".to_owned(),
        session_origin_class: class,
        session_scope_class: if is_handoff {
            BrowserSessionScopeClass::HandoffOnly
        } else if is_no_session {
            BrowserSessionScopeClass::NotApplicable
        } else {
            BrowserSessionScopeClass::PerTab
        },
        cross_origin_posture: if is_no_session {
            CrossOriginPostureClass::NotApplicable
        } else {
            CrossOriginPostureClass::SameOrigin
        },
        protocol_posture: if is_no_session {
            ProtocolPostureClass::NotApplicable
        } else {
            ProtocolPostureClass::SecureContext
        },
        source_mapping: sample_mapping(SourceMappingQualityClass::Exact),
        session_handle_ref: if admits {
            Some("session.handle.0001".to_owned())
        } else {
            None
        },
        handoff_record_ref: if is_handoff {
            Some("handoff.record.0001".to_owned())
        } else {
            None
        },
        redacted_session_label: "Local Chromium tab".to_owned(),
        summary: "Attached local browser session.".to_owned(),
    }
}

fn sample_plan(action: MutationActionKind) -> RuntimeMutationActionPlan {
    let inspect = action.is_inspect_only();
    let touches_browser = matches!(
        action,
        MutationActionKind::ClearBrowserStorage
            | MutationActionKind::NavigateBrowserTab
            | MutationActionKind::ReplayNetworkRequest
            | MutationActionKind::LiveStyleEdit
    );
    RuntimeMutationActionPlan {
        record_kind: RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND.to_owned(),
        runtime_mutation_action_plan_schema_version: RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION,
        runtime_mutation_action_plan_id: "preview.mutation.plan.0001".to_owned(),
        observed_at: "2026-05-19T13:55:00Z".to_owned(),
        preview_origin_descriptor_ref: "preview.origin.0001".to_owned(),
        preview_target_descriptor_ref: "preview.target.0001".to_owned(),
        browser_runtime_session_origin_ref: if touches_browser {
            Some("browser.session.0001".to_owned())
        } else {
            None
        },
        managed_workspace_approval_ref: None,
        action_kind: action,
        blast_class: if inspect {
            MutationBlastClass::NoMutationInspectOnly
        } else {
            MutationBlastClass::LocalRuntimeOnly
        },
        review_requirement: if inspect {
            MutationReviewRequirement::NoReviewRequiredInspectOnly
        } else {
            MutationReviewRequirement::ExplicitConfirmBeforeApply
        },
        side_effect_summary: if inspect {
            String::new()
        } else {
            "Reloads the document; in-memory state may be lost.".to_owned()
        },
        support_export_summary: if inspect {
            "Inspect-only.".to_owned()
        } else {
            "Reload preview on local dev server.".to_owned()
        },
        block_reason_summary: None,
    }
}

// ---------------------------------------------------------------------
// PreviewOriginDescriptor
// ---------------------------------------------------------------------

#[test]
fn protected_walk_local_dev_server_origin_validates_clean() {
    let origin = sample_origin_local();
    assert!(origin.validate().is_empty());
    assert!(origin.implies_live_runtime());
}

#[test]
fn failure_drill_managed_origin_requires_approval_ref() {
    let mut origin = sample_origin_managed();
    origin.managed_workspace_approval_ref = None;
    let findings = origin.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_origin_descriptor.managed_requires_approval"));
}

#[test]
fn failure_drill_static_evidence_requires_static_lifecycle() {
    let mut origin = sample_origin_static();
    origin.lifecycle_phase = PreviewOriginLifecyclePhase::Running;
    let findings = origin.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_origin_descriptor.static_evidence_lifecycle"));
    // Also: non-runtime origin in live phase rejected separately.
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_origin_descriptor.live_phase_without_runtime"));
}

#[test]
fn failure_drill_runtime_origin_cannot_claim_static_lifecycle() {
    let mut origin = sample_origin_local();
    origin.lifecycle_phase = PreviewOriginLifecyclePhase::NotApplicableStaticEvidence;
    let findings = origin.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_origin_descriptor.lifecycle_static_only"));
}

#[test]
fn failure_drill_remote_audience_requires_exposure_record() {
    let mut origin = sample_origin_local();
    origin.sharing_posture = PreviewOriginSharingPosture::SignedPreviewLink;
    let findings = origin.validate();
    assert!(findings.iter().any(
        |f| f.check_id == "preview_origin_descriptor.remote_audience_requires_exposure_record"
    ));
}

#[test]
fn failure_drill_remote_origin_cannot_claim_local_only_sharing() {
    let mut origin = sample_origin_remote();
    origin.sharing_posture = PreviewOriginSharingPosture::LocalOnly;
    let findings = origin.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_origin_descriptor.local_only_safety_overclaim"));
}

#[test]
fn origin_serde_round_trip() {
    let origin = sample_origin_managed();
    let json = serde_json::to_string(&origin).expect("serialize");
    let back: PreviewOriginDescriptor = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(origin, back);
}

// ---------------------------------------------------------------------
// PreviewTargetDescriptor
// ---------------------------------------------------------------------

#[test]
fn protected_walk_viewport_target_validates_clean() {
    let target = sample_target(PreviewTargetClass::ViewportPresetOnly);
    assert!(target.validate().is_empty());
}

#[test]
fn protected_walk_browser_tab_target_validates_clean() {
    let target = sample_target(PreviewTargetClass::BrowserTabTarget);
    assert!(target.validate().is_empty());
}

#[test]
fn failure_drill_reduced_capability_requires_reason() {
    let mut target = sample_target(PreviewTargetClass::SimulatorTarget);
    target.device_capability_class = DeviceCapabilityClass::ReducedCapability;
    let findings = target.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_target_descriptor.reduced_capability_requires_reason"));
}

#[test]
fn failure_drill_non_reduced_capability_forbids_reason() {
    let mut target = sample_target(PreviewTargetClass::DesignRendererTarget);
    target.device_capability_class = DeviceCapabilityClass::DesktopDefault;
    target.reduced_capability_reason = PreviewTargetReducedCapabilityReason::PolicyNarrowed;
    let findings = target.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "preview_target_descriptor.unreduced_capability_forbids_reason"));
}

#[test]
fn failure_drill_browser_runtime_target_requires_session_origin() {
    let mut target = sample_target(PreviewTargetClass::BrowserTabTarget);
    target.browser_runtime_session_origin_ref = None;
    let findings = target.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id
                == "preview_target_descriptor.browser_runtime_requires_session_origin")
    );
}

#[test]
fn failure_drill_non_browser_runtime_target_forbids_session_origin() {
    let mut target = sample_target(PreviewTargetClass::ViewportPresetOnly);
    target.browser_runtime_session_origin_ref = Some("browser.session.x".to_owned());
    let findings = target.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "preview_target_descriptor.non_browser_runtime_must_not_publish_session_origin"));
}

// ---------------------------------------------------------------------
// HotReloadStateDescriptor
// ---------------------------------------------------------------------

fn sample_hot_reload(event: HotReloadEventClass) -> HotReloadStateDescriptor {
    use HotReloadEventClass as E;
    use HotReloadStateRecoveryRoute as R;
    use HotReloadUnderlyingStateClass as U;
    let (under, routes, preserved): (U, Vec<R>, bool) = match event {
        E::HotReload => (U::Applied, vec![R::NoRecoveryRequiredApplied], false),
        E::FastRefresh => (U::Applied, vec![R::NoRecoveryRequiredApplied], true),
        E::Reconnect => (
            U::Applied,
            vec![R::NoRecoveryRequiredApplied, R::OpenRuntimeLogsRecovery],
            false,
        ),
        E::FullRestart => (U::RestartRequired, vec![R::RestartRuntimeRecovery], false),
        E::StaleOutput => (
            U::Partial,
            vec![
                R::RebuildThenReloadRecovery,
                R::InspectOnlyWithDiffAgainstSourceRecovery,
            ],
            false,
        ),
        E::Unavailable => (
            U::Unavailable,
            vec![R::NoRecoveryRequiredStaticPreviewUnavailable],
            false,
        ),
    };
    HotReloadStateDescriptor {
        record_kind: HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND.to_owned(),
        hot_reload_state_descriptor_schema_version: HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION,
        hot_reload_state_descriptor_id: "preview.hot_reload.descriptor.0001".to_owned(),
        preview_origin_descriptor_ref: "preview.origin.0001".to_owned(),
        preview_snapshot_record_ref: "preview.snapshot.0001".to_owned(),
        observed_at: "2026-05-19T13:55:00Z".to_owned(),
        event_class: event,
        underlying_state_class: under,
        recovery_routes: routes,
        component_state_preserved: preserved,
        summary: "Hot reload applied; view matches latest source.".to_owned(),
    }
}

#[test]
fn protected_walk_hot_reload_validates_clean() {
    let descriptor = sample_hot_reload(HotReloadEventClass::HotReload);
    assert!(descriptor.validate().is_empty());
}

#[test]
fn protected_walk_fast_refresh_validates_clean() {
    let descriptor = sample_hot_reload(HotReloadEventClass::FastRefresh);
    assert!(descriptor.validate().is_empty());
}

#[test]
fn failure_drill_fast_refresh_must_preserve_component_state() {
    let mut descriptor = sample_hot_reload(HotReloadEventClass::FastRefresh);
    descriptor.component_state_preserved = false;
    let findings = descriptor.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "hot_reload_state_descriptor.fast_refresh_implies_preserved_state"));
}

#[test]
fn failure_drill_full_restart_forbids_preserved_state() {
    let mut descriptor = sample_hot_reload(HotReloadEventClass::FullRestart);
    descriptor.component_state_preserved = true;
    let findings = descriptor.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "hot_reload_state_descriptor.full_restart_forbids_preserved_state"));
}

#[test]
fn failure_drill_projection_map_rejects_invalid_pair() {
    let mut descriptor = sample_hot_reload(HotReloadEventClass::HotReload);
    descriptor.underlying_state_class = HotReloadUnderlyingStateClass::Unavailable;
    descriptor.recovery_routes = vec![HotReloadStateRecoveryRoute::OpenCanonicalSourceRecovery];
    let findings = descriptor.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "hot_reload_state_descriptor.projection_map"));
}

#[test]
fn failure_drill_recovery_route_must_match_event() {
    let mut descriptor = sample_hot_reload(HotReloadEventClass::FastRefresh);
    descriptor.recovery_routes = vec![HotReloadStateRecoveryRoute::RestartRuntimeRecovery];
    let findings = descriptor.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "hot_reload_state_descriptor.recovery_route_for_event"));
}

#[test]
fn failure_drill_recovery_routes_cannot_be_empty() {
    let mut descriptor = sample_hot_reload(HotReloadEventClass::HotReload);
    descriptor.recovery_routes = Vec::new();
    let findings = descriptor.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "hot_reload_state_descriptor.recovery_routes_not_empty"));
}

// ---------------------------------------------------------------------
// BrowserRuntimeSessionOrigin
// ---------------------------------------------------------------------

#[test]
fn protected_walk_attached_browser_session_validates_clean() {
    let session = sample_session(BrowserSessionOriginClass::AttachedLocalBrowser);
    assert!(session.validate().is_empty());
}

#[test]
fn failure_drill_handoff_session_forbids_session_handle() {
    let mut session = sample_session(BrowserSessionOriginClass::ExternalHandoffBrowser);
    session.session_handle_ref = Some("session.handle.x".to_owned());
    let findings = session.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.handoff_forbids_session_handle"));
}

#[test]
fn failure_drill_handoff_session_requires_handoff_record() {
    let mut session = sample_session(BrowserSessionOriginClass::ExternalHandoffBrowser);
    session.handoff_record_ref = None;
    let findings = session.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.handoff_requires_handoff_record"));
}

#[test]
fn failure_drill_no_session_forbids_handle_and_requires_na_postures() {
    let mut session = sample_session(BrowserSessionOriginClass::NoSessionAttached);
    session.session_handle_ref = Some("session.handle.x".to_owned());
    session.cross_origin_posture = CrossOriginPostureClass::SameOrigin;
    session.protocol_posture = ProtocolPostureClass::SecureContext;
    let findings = session.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.no_session_forbids_handle"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.no_session_cross_origin"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.no_session_protocol"));
}

#[test]
fn failure_drill_inspectable_session_requires_handle() {
    let mut session = sample_session(BrowserSessionOriginClass::AttachedLocalBrowser);
    session.session_handle_ref = None;
    let findings = session.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id
                == "browser_runtime_session_origin.inspection_requires_session_handle")
    );
}

#[test]
fn failure_drill_cross_origin_blocked_cannot_claim_exact_mapping() {
    let mut session = sample_session(BrowserSessionOriginClass::AttachedLocalBrowser);
    session.cross_origin_posture = CrossOriginPostureClass::CrossOriginBlocked;
    // sample_mapping(Exact) is still on the session.
    let findings = session.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "browser_runtime_session_origin.cross_origin_blocked_exact_jump"));
}

// ---------------------------------------------------------------------
// SourceMappingDescriptor
// ---------------------------------------------------------------------

#[test]
fn protected_walk_exact_mapping_validates_clean() {
    let mapping = sample_mapping(SourceMappingQualityClass::Exact);
    assert!(mapping.validate("preview.target.0001").is_empty());
}

#[test]
fn failure_drill_exact_mapping_forbids_unmappable() {
    let mut mapping = sample_mapping(SourceMappingQualityClass::Exact);
    mapping.unmappable_node_count = 1;
    mapping.total_node_count = 13;
    let findings = mapping.validate("subject");
    assert!(findings
        .iter()
        .any(|f| f.check_id == "source_mapping_descriptor.exact_forbids_unmappable"));
}

#[test]
fn failure_drill_partial_requires_unmappable_and_mapped() {
    let mapping = SourceMappingDescriptor {
        source_mapping_quality_class: SourceMappingQualityClass::Partial,
        total_node_count: 10,
        exact_mapped_node_count: 10,
        unmappable_node_count: 0,
        summary: "Bad partial.".to_owned(),
    };
    let findings = mapping.validate("subject");
    assert!(findings
        .iter()
        .any(|f| f.check_id == "source_mapping_descriptor.partial_requires_unmappable"));
}

#[test]
fn failure_drill_unavailable_mapping_forbids_mapped_nodes() {
    let mapping = SourceMappingDescriptor {
        source_mapping_quality_class: SourceMappingQualityClass::Unavailable,
        total_node_count: 5,
        exact_mapped_node_count: 5,
        unmappable_node_count: 0,
        summary: "Bad unavailable.".to_owned(),
    };
    let findings = mapping.validate("subject");
    assert!(findings
        .iter()
        .any(|f| f.check_id == "source_mapping_descriptor.unavailable_forbids_mapped"));
}

#[test]
fn failure_drill_mapping_node_overflow_rejected() {
    let mapping = SourceMappingDescriptor {
        source_mapping_quality_class: SourceMappingQualityClass::Partial,
        total_node_count: 4,
        exact_mapped_node_count: 3,
        unmappable_node_count: 3,
        summary: "Overflow.".to_owned(),
    };
    let findings = mapping.validate("subject");
    assert!(findings
        .iter()
        .any(|f| f.check_id == "source_mapping_descriptor.node_count_overflow"));
}

// ---------------------------------------------------------------------
// RuntimeMutationActionPlan
// ---------------------------------------------------------------------

#[test]
fn protected_walk_inspect_plan_validates_clean() {
    let plan = sample_plan(MutationActionKind::InspectOnly);
    assert!(plan.validate().is_empty());
}

#[test]
fn protected_walk_local_reload_plan_validates_clean() {
    let plan = sample_plan(MutationActionKind::ReloadPreview);
    assert!(plan.validate().is_empty());
}

#[test]
fn failure_drill_inspect_plan_cannot_claim_mutation_blast() {
    let mut plan = sample_plan(MutationActionKind::InspectOnly);
    plan.blast_class = MutationBlastClass::LocalRuntimeOnly;
    let findings = plan.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.inspect_blast_mismatch"));
}

#[test]
fn failure_drill_mutation_action_must_declare_blast() {
    let mut plan = sample_plan(MutationActionKind::ReloadPreview);
    plan.blast_class = MutationBlastClass::NoMutationInspectOnly;
    let findings = plan.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.mutation_action_must_declare_blast"));
}

#[test]
fn failure_drill_remote_blast_cannot_claim_local_confirm_review() {
    let mut plan = sample_plan(MutationActionKind::RestartRuntime);
    plan.blast_class = MutationBlastClass::RemoteRuntimeReachable;
    plan.review_requirement = MutationReviewRequirement::ExplicitConfirmBeforeApply;
    let findings = plan.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.local_confirm_for_remote_blast"));
}

#[test]
fn failure_drill_managed_review_requires_approval_ref() {
    let mut plan = sample_plan(MutationActionKind::RestartRuntime);
    plan.blast_class = MutationBlastClass::ManagedPreviewServiceState;
    plan.review_requirement = MutationReviewRequirement::ManagedApprovalRequiredBeforeApply;
    plan.managed_workspace_approval_ref = None;
    let findings = plan.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id
                == "runtime_mutation_action_plan.managed_review_requires_approval_ref")
    );
}

#[test]
fn failure_drill_blocked_plan_requires_block_reason() {
    let mut plan = sample_plan(MutationActionKind::ClearBrowserStorage);
    plan.browser_runtime_session_origin_ref = Some("browser.session.0001".to_owned());
    plan.blast_class = MutationBlastClass::LocalBrowserStateOnly;
    plan.review_requirement = MutationReviewRequirement::BlockedNotAdmissible;
    plan.block_reason_summary = None;
    let findings = plan.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.blocked_requires_block_reason"));
}

#[test]
fn failure_drill_block_reason_only_when_blocked() {
    let mut plan = sample_plan(MutationActionKind::ReloadPreview);
    plan.block_reason_summary = Some("not allowed".to_owned());
    let findings = plan.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.block_reason_only_when_blocked"));
}

#[test]
fn failure_drill_browser_action_requires_session_origin() {
    let mut plan = sample_plan(MutationActionKind::ClearBrowserStorage);
    plan.browser_runtime_session_origin_ref = None;
    plan.blast_class = MutationBlastClass::LocalBrowserStateOnly;
    let findings = plan.validate();
    assert!(findings.iter().any(
        |f| f.check_id == "runtime_mutation_action_plan.browser_action_requires_session_origin"
    ));
}

#[test]
fn cross_validate_static_evidence_forbids_mutation() {
    let origin = sample_origin_static();
    let target = sample_target(PreviewTargetClass::ViewportPresetOnly);
    let mut plan = sample_plan(MutationActionKind::ReloadPreview);
    plan.preview_origin_descriptor_ref = origin.preview_origin_descriptor_id.clone();
    plan.preview_target_descriptor_ref = target.preview_target_descriptor_id.clone();
    let findings = plan.cross_validate(&origin, &target, None);
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.static_evidence_forbids_mutation"));
}

#[test]
fn cross_validate_remote_origin_cannot_host_local_blast() {
    let origin = sample_origin_remote();
    let target = sample_target(PreviewTargetClass::RemotePreviewTarget);
    let mut plan = sample_plan(MutationActionKind::ReloadPreview);
    plan.preview_origin_descriptor_ref = origin.preview_origin_descriptor_id.clone();
    plan.preview_target_descriptor_ref = target.preview_target_descriptor_id.clone();
    plan.browser_runtime_session_origin_ref = Some("browser.session.0001".to_owned());
    plan.blast_class = MutationBlastClass::LocalRuntimeOnly;
    let findings = plan.cross_validate(&origin, &target, None);
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.local_only_safety_overclaim"));
}

#[test]
fn cross_validate_managed_origin_requires_managed_review() {
    let origin = sample_origin_managed();
    let target = sample_target(PreviewTargetClass::RemotePreviewTarget);
    let mut plan = sample_plan(MutationActionKind::RestartRuntime);
    plan.preview_origin_descriptor_ref = origin.preview_origin_descriptor_id.clone();
    plan.preview_target_descriptor_ref = target.preview_target_descriptor_id.clone();
    plan.browser_runtime_session_origin_ref = Some("browser.session.0001".to_owned());
    plan.blast_class = MutationBlastClass::ManagedPreviewServiceState;
    plan.review_requirement = MutationReviewRequirement::ExplicitConfirmBeforeApply;
    let findings = plan.cross_validate(&origin, &target, None);
    assert!(findings.iter().any(
        |f| f.check_id == "runtime_mutation_action_plan.managed_origin_requires_managed_review"
    ));
}

#[test]
fn cross_validate_session_must_admit_mutation() {
    let origin = sample_origin_local();
    let target = sample_target(PreviewTargetClass::BrowserTabTarget);
    let session = sample_session(BrowserSessionOriginClass::ExternalHandoffBrowser);
    let mut plan = sample_plan(MutationActionKind::ClearBrowserStorage);
    plan.preview_origin_descriptor_ref = origin.preview_origin_descriptor_id.clone();
    plan.preview_target_descriptor_ref = target.preview_target_descriptor_id.clone();
    plan.browser_runtime_session_origin_ref =
        Some(session.browser_runtime_session_origin_id.clone());
    plan.blast_class = MutationBlastClass::LocalBrowserStateOnly;
    let findings = plan.cross_validate(&origin, &target, Some(&session));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "runtime_mutation_action_plan.session_does_not_admit_mutation"));
}

#[test]
fn plan_render_plaintext_redacted() {
    let plan = sample_plan(MutationActionKind::ReloadPreview);
    let rendered = plan.render_plaintext();
    assert!(rendered.contains("action=reload_preview"));
    assert!(rendered.contains("blast=local_runtime_only"));
    assert!(rendered.contains("review=explicit_confirm_before_apply"));
}

#[test]
fn origin_render_plaintext_redacted() {
    let origin = sample_origin_managed();
    let rendered = origin.render_plaintext();
    assert!(rendered.contains("origin=managed_preview_service"));
    assert!(rendered.contains("sharing=authenticated_org_route"));
}

#[test]
fn target_render_plaintext_redacted() {
    let target = sample_target(PreviewTargetClass::SimulatorTarget);
    let rendered = target.render_plaintext();
    assert!(rendered.contains("kind=simulator_target"));
}

#[test]
fn hot_reload_render_plaintext_redacted() {
    let descriptor = sample_hot_reload(HotReloadEventClass::FullRestart);
    let rendered = descriptor.render_plaintext();
    assert!(rendered.contains("event=full_restart"));
    assert!(rendered.contains("underlying=restart_required"));
}

#[test]
fn session_origin_serde_round_trip() {
    let session = sample_session(BrowserSessionOriginClass::RemoteDevtoolsBridge);
    let json = serde_json::to_string(&session).expect("serialize");
    let back: BrowserRuntimeSessionOrigin = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(session, back);
}

#[test]
fn plan_serde_round_trip() {
    let plan = sample_plan(MutationActionKind::ReloadPreview);
    let json = serde_json::to_string(&plan).expect("serialize");
    let back: RuntimeMutationActionPlan = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(plan, back);
}
