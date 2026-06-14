use super::*;

fn doc_ref() -> String {
    PROVIDER_STATUS_SURFACE_TRUTH_DOC_REF.to_owned()
}

fn fixture_ref() -> String {
    PROVIDER_STATUS_SURFACE_TRUTH_FIXTURE_DIR.to_owned()
}

/// Per-surface posture used to seed a fully covered, stable surface.
struct LaneSpec {
    surface: SurfaceClass,
    prefix: &'static str,
    provider: ProviderFamilyClass,
    locality: ProviderLocalityClass,
    lifecycle: ProviderLifecycleStateClass,
    capability: CapabilityNegotiationClass,
    detail_route: CapabilityDetailRouteClass,
    conflict: ConflictClass,
    has_loser: bool,
    result_form: SelectedResultFormClass,
    scope: ScopeLimitClass,
    freshness: FreshnessClass,
    recovery: RecoveryActionClass,
    anchor: ProvenanceAnchorTargetClass,
    provenance: ResultProvenanceClass,
    pill_completeness: CompletenessClass,
    downgrade_label: DowngradeLabelClass,
}

fn lane_specs() -> Vec<LaneSpec> {
    vec![
        LaneSpec {
            surface: SurfaceClass::FrameworkSurface,
            prefix: "framework",
            provider: ProviderFamilyClass::FrameworkAnalyzer,
            locality: ProviderLocalityClass::WorkspaceLocalProcess,
            lifecycle: ProviderLifecycleStateClass::ReadyLive,
            capability: CapabilityNegotiationClass::FullSemanticNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenNegotiationDrawer,
            conflict: ConflictClass::ArbitratedWinnerLoserPreserved,
            has_loser: true,
            result_form: SelectedResultFormClass::ArbitratedWinnerResult,
            scope: ScopeLimitClass::FullWorkspaceScope,
            freshness: FreshnessClass::FreshLive,
            recovery: RecoveryActionClass::RetryRequest,
            anchor: ProvenanceAnchorTargetClass::FrameworkAwareResult,
            provenance: ResultProvenanceClass::LiveSemantic,
            pill_completeness: CompletenessClass::NotApplicable,
            downgrade_label: DowngradeLabelClass::FullToPartialCompleteness,
        },
        LaneSpec {
            surface: SurfaceClass::NotebookSurface,
            prefix: "notebook",
            provider: ProviderFamilyClass::NotebookAdapter,
            locality: ProviderLocalityClass::NotebookKernelSession,
            lifecycle: ProviderLifecycleStateClass::DegradedPartial,
            capability: CapabilityNegotiationClass::PartialSemanticNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenCapabilityInspector,
            conflict: ConflictClass::SingleProviderNoConflict,
            has_loser: false,
            result_form: SelectedResultFormClass::SingleProviderResult,
            scope: ScopeLimitClass::OpenCellsScope,
            freshness: FreshnessClass::CachedRecent,
            recovery: RecoveryActionClass::RestartProvider,
            anchor: ProvenanceAnchorTargetClass::CompletionResult,
            provenance: ResultProvenanceClass::CachedSemantic,
            pill_completeness: CompletenessClass::NotApplicable,
            downgrade_label: DowngradeLabelClass::SemanticToTextFallback,
        },
        LaneSpec {
            surface: SurfaceClass::GeneratedSourceSurface,
            prefix: "generated",
            provider: ProviderFamilyClass::GeneratedSourceBridge,
            locality: ProviderLocalityClass::InProcessEngine,
            lifecycle: ProviderLifecycleStateClass::ReadyLive,
            capability: CapabilityNegotiationClass::TextFallbackNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenScopeLimitDetail,
            conflict: ConflictClass::PolicyOverrideRecorded,
            has_loser: false,
            result_form: SelectedResultFormClass::PolicyOverrideResult,
            scope: ScopeLimitClass::SparseIndexScope,
            freshness: FreshnessClass::ImportedSnapshot,
            recovery: RecoveryActionClass::RegenerateFromSource,
            anchor: ProvenanceAnchorTargetClass::DefinitionResult,
            provenance: ResultProvenanceClass::ImportedScan,
            pill_completeness: CompletenessClass::NotApplicable,
            downgrade_label: DowngradeLabelClass::GeneratedEditToRegenerateFirst,
        },
        LaneSpec {
            surface: SurfaceClass::PreviewSurface,
            prefix: "preview",
            provider: ProviderFamilyClass::LspProvider,
            locality: ProviderLocalityClass::LocalHostSubprocess,
            lifecycle: ProviderLifecycleStateClass::ReadyLive,
            capability: CapabilityNegotiationClass::FullSemanticNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenNegotiationDrawer,
            conflict: ConflictClass::SingleProviderNoConflict,
            has_loser: false,
            result_form: SelectedResultFormClass::SingleProviderResult,
            scope: ScopeLimitClass::FullWorkspaceScope,
            freshness: FreshnessClass::FreshLive,
            recovery: RecoveryActionClass::RerunPreview,
            anchor: ProvenanceAnchorTargetClass::RenamePreview,
            provenance: ResultProvenanceClass::LiveSemantic,
            pill_completeness: CompletenessClass::Complete,
            downgrade_label: DowngradeLabelClass::PreviewableToCompareOnly,
        },
        LaneSpec {
            surface: SurfaceClass::DocsLinkedSurface,
            prefix: "docs",
            provider: ProviderFamilyClass::AiOverlay,
            locality: ProviderLocalityClass::RemoteManagedService,
            lifecycle: ProviderLifecycleStateClass::Restarting,
            capability: CapabilityNegotiationClass::PartialSemanticNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenProviderHealthPanel,
            conflict: ConflictClass::SingleProviderNoConflict,
            has_loser: false,
            result_form: SelectedResultFormClass::FusedResult,
            scope: ScopeLimitClass::SingleFileScope,
            freshness: FreshnessClass::StalePendingRefresh,
            recovery: RecoveryActionClass::RefreshResult,
            anchor: ProvenanceAnchorTargetClass::HoverDocResult,
            provenance: ResultProvenanceClass::StalePendingRefresh,
            pill_completeness: CompletenessClass::NotApplicable,
            downgrade_label: DowngradeLabelClass::ProviderUnavailableTextOnly,
        },
        LaneSpec {
            surface: SurfaceClass::StructuredArtifactSurface,
            prefix: "structured",
            provider: ProviderFamilyClass::LspProvider,
            locality: ProviderLocalityClass::LocalHostSubprocess,
            lifecycle: ProviderLifecycleStateClass::ReadyLive,
            capability: CapabilityNegotiationClass::FullSemanticNegotiated,
            detail_route: CapabilityDetailRouteClass::OpenCapabilityInspector,
            conflict: ConflictClass::UnresolvedDisagreementSurfaced,
            has_loser: true,
            result_form: SelectedResultFormClass::UnresolvedDisagreementResult,
            scope: ScopeLimitClass::WorksetSubsetScope,
            freshness: FreshnessClass::FreshLive,
            recovery: RecoveryActionClass::RetryRequest,
            anchor: ProvenanceAnchorTargetClass::ReferenceResult,
            provenance: ResultProvenanceClass::LiveSemantic,
            pill_completeness: CompletenessClass::NotApplicable,
            downgrade_label: DowngradeLabelClass::FullToPartialCompleteness,
        },
    ]
}

fn base_row(
    row_id: &str,
    surface: SurfaceClass,
    object_kind: SurfaceObjectKind,
    row_class: ObjectRowClass,
) -> SurfaceObjectRow {
    SurfaceObjectRow {
        row_id: row_id.to_owned(),
        surface_lane: surface,
        object_kind,
        row_class,
        support_class: SupportClass::Certified,
        provider_family_class: ProviderFamilyClass::NotApplicable,
        provider_locality_class: ProviderLocalityClass::NotApplicable,
        provider_lifecycle_state_class: ProviderLifecycleStateClass::NotApplicable,
        provider_display_label_class: ProviderDisplayLabelClass::NotApplicable,
        capability_negotiation_class: CapabilityNegotiationClass::NotApplicable,
        capability_detail_route_class: CapabilityDetailRouteClass::NotApplicable,
        participant_role_class: ParticipantRoleClass::NotApplicable,
        conflict_class: ConflictClass::NotApplicable,
        selected_result_form_class: SelectedResultFormClass::NotApplicable,
        scope_limit_class: ScopeLimitClass::NotApplicable,
        freshness_class: FreshnessClass::NotApplicable,
        recovery_action_class: RecoveryActionClass::NotApplicable,
        provenance_anchor_target_class: ProvenanceAnchorTargetClass::NotApplicable,
        result_provenance_class: ResultProvenanceClass::NotApplicable,
        completeness_class: CompletenessClass::NotApplicable,
        downgrade_label_class: DowngradeLabelClass::NotApplicable,
        evidence_class: EvidenceClass::FixtureRepoEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
        confidence_class: ConfidenceClass::HighConfidence,
        evidence_refs: vec![fixture_ref()],
        disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
        provenance_requires_raw_logs: false,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-06-14T12:00:00Z".to_owned(),
    }
}

fn lane_rows(spec: &LaneSpec) -> Vec<SurfaceObjectRow> {
    let prefix = spec.prefix;
    let mut rows = Vec::new();

    let mut strip_presence = base_row(
        &format!("row:{prefix}:strip:presence"),
        spec.surface,
        SurfaceObjectKind::ProviderStatusStrip,
        ObjectRowClass::SurfaceObjectPresence,
    );
    strip_presence.provider_family_class = spec.provider;
    strip_presence.provider_display_label_class = ProviderDisplayLabelClass::HumanReadableLaneLabel;
    rows.push(strip_presence);

    let mut lane_state = base_row(
        &format!("row:{prefix}:strip:lane_state"),
        spec.surface,
        SurfaceObjectKind::ProviderStatusStrip,
        ObjectRowClass::ProviderLaneStateAdmission,
    );
    lane_state.provider_family_class = spec.provider;
    lane_state.provider_display_label_class =
        ProviderDisplayLabelClass::ProviderFamilyWithLocalityLabel;
    lane_state.provider_locality_class = spec.locality;
    lane_state.provider_lifecycle_state_class = spec.lifecycle;
    rows.push(lane_state);

    let mut route = base_row(
        &format!("row:{prefix}:strip:detail_route"),
        spec.surface,
        SurfaceObjectKind::ProviderStatusStrip,
        ObjectRowClass::CapabilityDetailRouteAdmission,
    );
    route.capability_detail_route_class = spec.detail_route;
    route.capability_negotiation_class = spec.capability;
    rows.push(route);

    let mut drawer_presence = base_row(
        &format!("row:{prefix}:drawer:presence"),
        spec.surface,
        SurfaceObjectKind::CapabilityNegotiationDrawer,
        ObjectRowClass::SurfaceObjectPresence,
    );
    drawer_presence.provider_family_class = spec.provider;
    drawer_presence.provider_display_label_class =
        ProviderDisplayLabelClass::HumanReadableLaneLabel;
    rows.push(drawer_presence);

    let mut winner = base_row(
        &format!("row:{prefix}:drawer:winner"),
        spec.surface,
        SurfaceObjectKind::CapabilityNegotiationDrawer,
        ObjectRowClass::ParticipatingProviderAdmission,
    );
    winner.participant_role_class = ParticipantRoleClass::SelectedWinner;
    winner.conflict_class = spec.conflict;
    rows.push(winner);

    if spec.has_loser {
        let mut loser = base_row(
            &format!("row:{prefix}:drawer:loser"),
            spec.surface,
            SurfaceObjectKind::CapabilityNegotiationDrawer,
            ObjectRowClass::ParticipatingProviderAdmission,
        );
        loser.participant_role_class = ParticipantRoleClass::PreservedLoser;
        loser.conflict_class = spec.conflict;
        rows.push(loser);
    }

    let mut result = base_row(
        &format!("row:{prefix}:drawer:result"),
        spec.surface,
        SurfaceObjectKind::CapabilityNegotiationDrawer,
        ObjectRowClass::NegotiationResultAdmission,
    );
    result.selected_result_form_class = spec.result_form;
    rows.push(result);

    let mut scope = base_row(
        &format!("row:{prefix}:drawer:scope_freshness"),
        spec.surface,
        SurfaceObjectKind::CapabilityNegotiationDrawer,
        ObjectRowClass::ScopeAndFreshnessAdmission,
    );
    scope.scope_limit_class = spec.scope;
    scope.freshness_class = spec.freshness;
    rows.push(scope);

    let mut recovery = base_row(
        &format!("row:{prefix}:drawer:recovery"),
        spec.surface,
        SurfaceObjectKind::CapabilityNegotiationDrawer,
        ObjectRowClass::DrawerRecoveryActionAdmission,
    );
    recovery.recovery_action_class = spec.recovery;
    rows.push(recovery);

    let mut pill_presence = base_row(
        &format!("row:{prefix}:pill:presence"),
        spec.surface,
        SurfaceObjectKind::ResultProvenancePill,
        ObjectRowClass::SurfaceObjectPresence,
    );
    pill_presence.provider_family_class = spec.provider;
    pill_presence.provider_display_label_class = ProviderDisplayLabelClass::HumanReadableLaneLabel;
    rows.push(pill_presence);

    let mut anchor = base_row(
        &format!("row:{prefix}:pill:anchor"),
        spec.surface,
        SurfaceObjectKind::ResultProvenancePill,
        ObjectRowClass::ProvenanceAnchorAdmission,
    );
    anchor.provenance_anchor_target_class = spec.anchor;
    anchor.result_provenance_class = spec.provenance;
    anchor.completeness_class = spec.pill_completeness;
    anchor.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
    rows.push(anchor);

    let mut downgrade = base_row(
        &format!("row:{prefix}:pill:downgrade"),
        spec.surface,
        SurfaceObjectKind::ResultProvenancePill,
        ObjectRowClass::ProvenanceDowngradeAdmission,
    );
    downgrade.downgrade_label_class = spec.downgrade_label;
    rows.push(downgrade);

    rows
}

fn projection(surface: ConsumerSurface, packet_id: &str) -> SurfaceObjectConsumerProjection {
    SurfaceObjectConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}:stable", surface.as_str()),
        surface_packet_id_ref: packet_id.to_owned(),
        rendered_at: "2026-06-14T12:00:00Z".to_owned(),
        preserves_same_packet: true,
        preserves_surface_lane_vocabulary: true,
        preserves_object_kind_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_provider_family_vocabulary: true,
        preserves_provider_locality_vocabulary: true,
        preserves_provider_lifecycle_state_vocabulary: true,
        preserves_provider_display_label_vocabulary: true,
        preserves_capability_negotiation_vocabulary: true,
        preserves_capability_detail_route_vocabulary: true,
        preserves_participant_role_vocabulary: true,
        preserves_conflict_vocabulary: true,
        preserves_selected_result_form_vocabulary: true,
        preserves_scope_limit_vocabulary: true,
        preserves_freshness_vocabulary: true,
        preserves_recovery_action_vocabulary: true,
        preserves_provenance_anchor_target_vocabulary: true,
        preserves_result_provenance_vocabulary: true,
        preserves_completeness_vocabulary: true,
        preserves_downgrade_label_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn sample_input() -> ProviderStatusSurfaceTruthPacketInput {
    let packet_id = "packet:test:provider_status_surface:stable".to_owned();
    let mut rows = Vec::new();
    for spec in lane_specs() {
        rows.extend(lane_rows(&spec));
    }
    let projections = ConsumerSurface::REQUIRED
        .iter()
        .map(|surface| projection(*surface, &packet_id))
        .collect();
    ProviderStatusSurfaceTruthPacketInput {
        packet_id,
        workflow_or_surface_id: "workflow.test.provider_status_surface".to_owned(),
        generated_at: "2026-06-14T12:00:00Z".to_owned(),
        covered_surfaces: SurfaceClass::REQUIRED.to_vec(),
        rows,
        consumer_projections: projections,
        source_contract_refs: vec![PROVIDER_STATUS_SURFACE_MATRIX_SOURCE_REF.to_owned()],
    }
}

fn finding_kinds(packet: &ProviderStatusSurfaceTruthPacket) -> Vec<FindingKind> {
    packet
        .validation_findings
        .iter()
        .map(|finding| finding.finding_kind)
        .collect()
}

#[test]
fn sample_input_materializes_stable() {
    let packet = ProviderStatusSurfaceTruthPacket::materialize(sample_input());
    assert_eq!(packet.promotion_state, PromotionState::Stable);
    assert!(packet.validation_findings.is_empty());
    assert!(packet.is_stable());
    assert!(packet.validate().is_empty());
}

#[test]
fn every_required_surface_carries_all_three_objects() {
    let packet = ProviderStatusSurfaceTruthPacket::materialize(sample_input());
    for surface in SurfaceClass::REQUIRED {
        for kind in SurfaceObjectKind::REQUIRED {
            assert!(
                packet.rows.iter().any(|row| row.surface_lane == surface
                    && row.object_kind == kind
                    && matches!(row.row_class, ObjectRowClass::SurfaceObjectPresence)),
                "surface {} must carry a {} presence row",
                surface.as_str(),
                kind.as_str()
            );
        }
    }
}

#[test]
fn all_required_consumer_projections_present() {
    let packet = ProviderStatusSurfaceTruthPacket::materialize(sample_input());
    for surface in ConsumerSurface::REQUIRED {
        assert!(packet.has_projection_for(surface));
    }
}

#[test]
fn certified_with_unbound_evidence_blocks() {
    let mut input = sample_input();
    input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    let kinds = finding_kinds(&packet);
    assert!(kinds.contains(&FindingKind::MissingEvidenceClass));
    assert!(kinds.contains(&FindingKind::CertifiedWithUnboundBinding));
}

#[test]
fn opaque_spinner_detail_route_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if matches!(
            row.row_class,
            ObjectRowClass::CapabilityDetailRouteAdmission
        ) && row.surface_lane == SurfaceClass::FrameworkSurface
        {
            row.capability_detail_route_class = CapabilityDetailRouteClass::OpaqueSpinnerOnly;
        }
    }
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::CapabilityDetailRouteIsOpaqueSpinner));
}

#[test]
fn raw_process_name_only_label_blocks() {
    let mut input = sample_input();
    input.rows[0].provider_display_label_class = ProviderDisplayLabelClass::RawProcessNameOnly;
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::RawProcessNameOnlyLabel));
}

#[test]
fn dropping_preserved_loser_blocks() {
    let mut input = sample_input();
    input.rows.retain(|row| {
        !(row.surface_lane == SurfaceClass::StructuredArtifactSurface
            && matches!(
                row.row_class,
                ObjectRowClass::ParticipatingProviderAdmission
            )
            && matches!(
                row.participant_role_class,
                ParticipantRoleClass::PreservedLoser
            ))
    });
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::LosingProviderNotPreserved));
}

#[test]
fn scope_limit_on_lane_state_row_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if matches!(row.row_class, ObjectRowClass::ProviderLaneStateAdmission)
            && row.surface_lane == SurfaceClass::FrameworkSurface
        {
            row.scope_limit_class = ScopeLimitClass::FullWorkspaceScope;
        }
    }
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::ScopeLimitNotPermittedOnRowClass));
}

#[test]
fn rename_preview_anchor_without_completeness_bypasses_preview() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if matches!(row.row_class, ObjectRowClass::ProvenanceAnchorAdmission)
            && matches!(
                row.provenance_anchor_target_class,
                ProvenanceAnchorTargetClass::RenamePreview
            )
        {
            row.completeness_class = CompletenessClass::NotApplicable;
        }
    }
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::PreviewAnchorBypassesTypedPreview));
}

#[test]
fn provenance_requiring_raw_logs_blocks() {
    let mut input = sample_input();
    for row in &mut input.rows {
        if matches!(row.row_class, ObjectRowClass::ProvenanceAnchorAdmission)
            && row.surface_lane == SurfaceClass::DocsLinkedSurface
        {
            row.provenance_requires_raw_logs = true;
        }
    }
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::ProvenanceRequiresRawLogs));
}

#[test]
fn missing_object_kind_presence_blocks() {
    let mut input = sample_input();
    input.rows.retain(|row| {
        !(row.surface_lane == SurfaceClass::NotebookSurface
            && row.object_kind == SurfaceObjectKind::ResultProvenancePill)
    });
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    assert!(finding_kinds(&packet).contains(&FindingKind::MissingObjectKindPresence));
}

#[test]
fn projection_collapse_blocks() {
    let mut input = sample_input();
    for projection in &mut input.consumer_projections {
        if projection.consumer_surface == ConsumerSurface::HelpAbout {
            projection.preserves_result_provenance_vocabulary = false;
        }
    }
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
    let kinds = finding_kinds(&packet);
    assert!(kinds.contains(&FindingKind::ResultProvenanceVocabularyCollapsed));
    assert!(kinds.contains(&FindingKind::ConsumerProjectionDrift));
    assert!(kinds.contains(&FindingKind::MissingConsumerProjection));
}

#[test]
fn support_export_round_trips_when_stable() {
    let packet = ProviderStatusSurfaceTruthPacket::materialize(sample_input());
    let export = packet.support_export("export:test", "2026-06-14T12:00:10Z");
    assert!(export.is_export_safe());
    assert_eq!(export.surface_packet_id_ref, packet.packet_id);
}

#[test]
fn support_export_unsafe_when_packet_blocks() {
    let mut input = sample_input();
    input.rows[0].secrets_excluded = false;
    let packet = ProviderStatusSurfaceTruthPacket::materialize(input);
    let export = packet.support_export("export:test", "2026-06-14T12:00:10Z");
    assert!(!export.is_export_safe());
    assert!(finding_kinds(&packet).contains(&FindingKind::SecretsPresent));
}

#[test]
fn token_methods_observe_expected_values() {
    let packet = ProviderStatusSurfaceTruthPacket::materialize(sample_input());
    assert!(packet
        .object_kind_tokens()
        .contains(&SurfaceObjectKind::ProviderStatusStrip.as_str()));
    assert!(packet
        .provenance_anchor_target_tokens()
        .contains(&ProvenanceAnchorTargetClass::RenamePreview.as_str()));
    assert!(packet
        .participant_role_tokens()
        .contains(&ParticipantRoleClass::PreservedLoser.as_str()));
    assert!(packet
        .provider_locality_tokens()
        .contains(&ProviderLocalityClass::NotebookKernelSession.as_str()));
}
