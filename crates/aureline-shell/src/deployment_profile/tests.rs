use super::*;

const EMITTED_AT: &str = "2026-05-19T00:00:00Z";

fn open_details_action(summary_id: &str) -> InspectOnlyAction {
    InspectOnlyAction::open_details(
        format!("action.summary.{summary_id}.open_details"),
        format!("route.deployment.details.{summary_id}"),
    )
}

fn baseline_residual_row(
    summary_id: &str,
    dependency: DependencyClass,
    posture: PostureClass,
    impact: AbsenceImpactClass,
    fallback: ContinuityFallbackClass,
) -> ResidualDependencyRow {
    ResidualDependencyRow {
        record_kind: RESIDUAL_DEPENDENCY_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        row_id: format!("row.dep.{summary_id}.{}", dependency.as_str()),
        profile_summary_ref: summary_id.to_owned(),
        dependency_class: dependency,
        posture_class: posture,
        vendor_or_public_dependence: matches!(
            dependency,
            DependencyClass::AiProvider
                | DependencyClass::BrowserHandoff
                | DependencyClass::CompanionNotificationChannel
                | DependencyClass::HostedControlPlaneReachability
        ),
        dependent_feature_label: format!("{} dependent feature", dependency.as_str()),
        dependent_feature_refs: Vec::new(),
        unreachable_impact_class: impact,
        unreachable_impact_label: format!("{} impact when unreachable", dependency.as_str()),
        continuity_fallback_class: fallback,
        ledger_row_ref: format!("ledger.{}", dependency.as_str()),
        freshness_label: None,
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_outage_notice_ref: None,
        linked_continuity_packet_ref: None,
        evidence_links: Vec::new(),
        notes: None,
    }
}

fn verify_action(label: &str, revalidation: &str) -> VerifyOrOpenManifestAction {
    VerifyOrOpenManifestAction {
        action_id: format!("action.verify.{label}"),
        label: label.to_owned(),
        scope_class: "scope_local_only".to_owned(),
        authority_class: "user_local_authority".to_owned(),
        consent_class: "no_consent_required_safe_default".to_owned(),
        side_effects: vec!["no_side_effect_inspect_only".to_owned()],
        preserves_evidence_context: true,
        modal_prohibited: true,
        revalidation_on_open: revalidation.to_owned(),
        evidence_links: Vec::new(),
    }
}

fn baseline_artifact_row(
    summary_id: &str,
    artifact: ArtifactClass,
    signer: SignerStateClass,
    digest: DigestStateClass,
    freshness: MirrorFreshnessClass,
    cache: OfflineCachePostureClass,
    source: MirrorSourceClass,
) -> MirrorOfflineArtifactRow {
    let verify_revalidation = if digest == DigestStateClass::DigestMismatch {
        "blocked_until_fresh"
    } else {
        "snapshot_open_read_only"
    };
    MirrorOfflineArtifactRow {
        record_kind: MIRROR_OFFLINE_ARTIFACT_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        row_id: format!("row.artifact.{summary_id}.{}", artifact.as_str()),
        profile_summary_ref: summary_id.to_owned(),
        artifact_class: artifact,
        artifact_label: format!("{} artifact", artifact.as_str()),
        signer_state_class: signer,
        signer_fingerprint_ref: if signer.requires_fingerprint() {
            Some(format!("fp:{}:abcdef", artifact.as_str()))
        } else {
            None
        },
        digest_state_class: digest,
        digest_ref: if matches!(
            digest,
            DigestStateClass::DigestVerified | DigestStateClass::DigestPending
        ) {
            Some(format!("sha256:{}:0011223344", artifact.as_str()))
        } else {
            None
        },
        mirror_freshness_class: freshness,
        last_refresh_at: if matches!(
            freshness,
            MirrorFreshnessClass::MirrorFreshWithinWindow
                | MirrorFreshnessClass::MirrorWithinExtendedWindow
                | MirrorFreshnessClass::MirrorPastExtendedWindow
        ) {
            Some(EMITTED_AT.to_owned())
        } else {
            None
        },
        offline_cache_posture_class: cache,
        mirror_source_class: source,
        verify_action: verify_action(
            &format!("{summary_id}.{}", artifact.as_str()),
            verify_revalidation,
        ),
        open_manifest_action: verify_action(
            &format!("manifest.{summary_id}.{}", artifact.as_str()),
            "snapshot_open_read_only",
        ),
        evidence_links: Vec::new(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        notes: None,
    }
}

#[test]
fn individual_local_baseline_passes_audit_and_keeps_local_safe_action() {
    let page = individual_local_baseline_page(EMITTED_AT);
    let defects = page.audit();
    assert!(
        defects.is_empty(),
        "baseline page emitted defects: {defects:?}"
    );
    assert_eq!(
        page.plane_status_strip.safest_next_action.action_class,
        SafestNextActionClass::ContinueLocal,
    );
    assert!(page.summary.local_safe_remains);
    assert!(!page.summary.honesty_marker_present);
    // Companion surface is not routed for any local-only profile by default.
    assert!(!page
        .profile_summary
        .consumer_surfaces
        .contains(&ConsumerSurfaceClass::CompanionSurface));
}

#[test]
fn managed_cloud_page_requires_guardrails_and_vendor_dependencies() {
    let summary_id = "summary.deployment.managed_cloud_baseline";
    let strip_id = "strip.deployment.managed_cloud_baseline";
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::ManagedCloud,
        product_facing_label_class: ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowDefault,
        key_mode_class: KeyModeClass::VendorManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Vendor-operated control plane reachable.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: None,
            freshness_floor_ref: None,
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Healthy,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: vec![
            format!("row.dep.{summary_id}.ai_provider"),
            format!("row.dep.{summary_id}.hosted_control_plane_reachability"),
        ],
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.to_owned(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
            ProhibitedImpliedClaimClass::ImpliedNoResidualDependencyWhenRequiredPresent,
        ],
        open_details_action: open_details_action(summary_id),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::SupportPacketExport,
            ConsumerSurfaceClass::AdminAuditExport,
            ConsumerSurfaceClass::ReleaseEvidenceExcerpt,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::CompanionSurface,
            ConsumerSurfaceClass::CliTextFormatter,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Healthy,
            summary_label: "All managed services healthy.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::AuthIdentityService,
                ControlPlaneServiceClass::PolicyService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Local-core capabilities available.".to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline(
            )
            .to_vec(),
        },
        safest_next_action: SafestNextAction::continue_local(format!(
            "action.strip.{strip_id}.continue_local"
        )),
        alternate_actions: Vec::new(),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.to_owned()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let residual = vec![
        baseline_residual_row(
            summary_id,
            DependencyClass::AiProvider,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
        ),
        baseline_residual_row(
            summary_id,
            DependencyClass::HostedControlPlaneReachability,
            PostureClass::Required,
            AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
            ContinuityFallbackClass::ContinueLocalNoRestore,
        ),
    ];
    let (page, defects) = DeploymentProfilePage::compose(
        "page.deployment.managed_cloud_baseline",
        profile_summary,
        plane_status_strip,
        residual,
        Vec::new(),
    );
    assert!(
        defects.is_empty(),
        "managed_cloud baseline defects: {defects:?}"
    );
    assert_eq!(page.summary.residual_dependency_required_count, 2);

    // Drop a guardrail and confirm the page is held honest.
    let mut narrowed = page.clone();
    narrowed
        .profile_summary
        .prohibited_implied_claim_classes
        .retain(|c| {
            *c != ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent
        });
    let narrowed_defects = narrowed.audit();
    assert!(
        narrowed_defects
            .iter()
            .any(|d| matches!(d, DeploymentProfileDefect::ManagedCloudMissingGuardrails)),
        "expected ManagedCloudMissingGuardrails, got {narrowed_defects:?}"
    );
}

#[test]
fn self_hosted_cannot_silently_carry_vendor_managed_keys() {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    page.profile_summary.deployment_profile = DeploymentProfileClass::SelfHosted;
    page.profile_summary.product_facing_label_class = ProductFacingLabelClass::SelfHostedSovereign;
    page.profile_summary.tenant_org_scope_class = TenantOrgScopeClass::CustomerTenant;
    page.profile_summary.region_scope_class = RegionScopeClass::CustomerRegionPinned;
    page.profile_summary.key_mode_class = KeyModeClass::VendorManaged;
    page.profile_summary.retention_class = RetentionClass::CustomerRetentionWindow;
    let defects = page.audit();
    assert!(defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::SelfHostedClaimedVendorManagedKeys
    )));
}

#[test]
fn air_gapped_must_declare_offline_state_and_artifact_row_and_no_companion() {
    let summary_id = "summary.deployment.air_gapped";
    let strip_id = "strip.deployment.air_gapped";
    let row = baseline_artifact_row(
        summary_id,
        ArtifactClass::DocsPack,
        SignerStateClass::SignedOfflineTrustRoot,
        DigestStateClass::DigestVerified,
        MirrorFreshnessClass::MirrorFreshWithinWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
    );
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::AirGapped,
        product_facing_label_class: ProductFacingLabelClass::AirGappedMirrorOnly,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::OfflineTrustRoot,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineAirGapped,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Offline bundle inside extended freshness window.".to_owned(),
            last_control_plane_sync_at: None,
            cache_age_label: Some("Within bundle freshness window".to_owned()),
            freshness_floor_ref: Some("freshness.air_gapped.bounded".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: Vec::new(),
        mirror_offline_artifact_row_refs: vec![row.row_id.clone()],
        plane_status_strip_ref: strip_id.to_owned(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAirGappedWhenEgressAllowed,
        ],
        open_details_action: open_details_action(summary_id),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::SupportPacketExport,
            ConsumerSurfaceClass::AdminAuditExport,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::CliTextFormatter,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
            summary_label: "Air-gapped install; no live control plane in scope.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: Vec::new(),
            last_sync_at: None,
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Local-core capabilities available on offline bundle.".to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline(
            )
            .to_vec(),
        },
        safest_next_action: SafestNextAction::continue_local(format!(
            "action.strip.{strip_id}.continue_local"
        )),
        alternate_actions: Vec::new(),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.to_owned()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, defects) = DeploymentProfilePage::compose(
        "page.deployment.air_gapped_baseline",
        profile_summary,
        plane_status_strip,
        Vec::new(),
        vec![row],
    );
    assert!(
        defects.is_empty(),
        "air_gapped baseline defects: {defects:?}"
    );

    // Force a companion surface and confirm rejection.
    let mut narrowed = page.clone();
    narrowed
        .profile_summary
        .consumer_surfaces
        .push(ConsumerSurfaceClass::CompanionSurface);
    let narrowed_defects = narrowed.audit();
    assert!(narrowed_defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::AirGappedRoutedThroughCompanionSurface
    )));

    // Drop the offline_air_gapped state and confirm rejection.
    let mut narrowed = page.clone();
    narrowed.profile_summary.mirror_offline_state_class =
        MirrorOfflineStateClass::OnlineLiveAllowed;
    let narrowed_defects = narrowed.audit();
    assert!(narrowed_defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::AirGappedMissingOfflineAirGappedState { .. }
    )));
}

#[test]
fn mirror_only_must_emit_artifact_rows_and_offline_parity_guardrail() {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    page.profile_summary.mirror_offline_state_class = MirrorOfflineStateClass::OnlineMirrorOnly;
    let defects = page.audit();
    assert!(defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::MirrorOrAirGappedMissingArtifactRow { .. }
    )));
    assert!(defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::MirrorOnlyMissingOfflineParityGuardrail
    )));
}

#[test]
fn relay_outage_keeps_local_safe_next_action() {
    let summary_id = "summary.deployment.managed_cloud_relay_disconnect";
    let strip_id = "strip.deployment.managed_cloud_relay_disconnect";
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::ManagedCloud,
        product_facing_label_class: ProductFacingLabelClass::BrowserCompanionHandoffDefaultHome,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::VendorRetentionWindowDefault,
        key_mode_class: KeyModeClass::VendorManaged,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineGracePreserved,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::BoundedStale,
            summary_label: "Last managed sync within grace window.".to_owned(),
            last_control_plane_sync_at: Some(EMITTED_AT.to_owned()),
            cache_age_label: Some("Within grace window".to_owned()),
            freshness_floor_ref: Some("freshness.managed_cloud.grace".to_owned()),
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::Unavailable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: Vec::new(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.to_owned(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
            ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
        ],
        open_details_action: open_details_action(summary_id),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::CompanionSurface,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::Unavailable,
            summary_label: "Relay service unavailable; sync and registry healthy.".to_owned(),
            impaired_service_classes: vec![ControlPlaneServiceClass::RelayService],
            healthy_service_classes: vec![
                ControlPlaneServiceClass::SyncService,
                ControlPlaneServiceClass::RegistryService,
            ],
            last_sync_at: Some(EMITTED_AT.to_owned()),
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Local-core capabilities remain available; remote attach blocked."
                .to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline(
            )
            .to_vec(),
        },
        safest_next_action: SafestNextAction {
            action_id: format!("action.strip.{strip_id}.continue_local"),
            action_class: SafestNextActionClass::ContinueLocal,
            label: SafestNextActionClass::ContinueLocal.label().to_owned(),
            scope_class: "scope_local_only".to_owned(),
            authority_class: "user_local_authority".to_owned(),
            consent_class: "no_consent_required_safe_default".to_owned(),
            side_effects: vec!["no_side_effect_inspect_only".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        },
        alternate_actions: vec![SafestNextAction {
            action_id: format!("action.strip.{strip_id}.reconnect_managed_session"),
            action_class: SafestNextActionClass::ReconnectManagedSession,
            label: SafestNextActionClass::ReconnectManagedSession
                .label()
                .to_owned(),
            scope_class: "scope_local_with_managed_recovery".to_owned(),
            authority_class: "user_managed_authority".to_owned(),
            consent_class: "explicit_consent_required_managed_recovery".to_owned(),
            side_effects: vec!["session_reconnect_attempt".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }],
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::BannerNotice,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.to_owned()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, defects) = DeploymentProfilePage::compose(
        "page.deployment.managed_cloud_relay_disconnect",
        profile_summary,
        plane_status_strip,
        Vec::new(),
        Vec::new(),
    );
    assert!(defects.is_empty(), "relay disconnect defects: {defects:?}");
    assert!(page.summary.control_plane_impaired);
    assert!(page.summary.local_safe_remains);
    assert!(page.summary.honesty_marker_present);
    assert_eq!(
        page.plane_status_strip.safest_next_action.action_class,
        SafestNextActionClass::ContinueLocal,
    );
}

#[test]
fn generic_service_degraded_action_is_rejected_when_local_remains_safe() {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    // Force the data plane local-safe and control plane healthy, but swap
    // the action for one that does not preserve local continuity.
    page.plane_status_strip.safest_next_action.action_class =
        SafestNextActionClass::ReconnectManagedSession;
    let defects = page.audit();
    assert!(defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::GenericServiceDegradedWhereLocalSafeRemains { .. }
    )));
}

#[test]
fn digest_mismatch_must_block_verify_until_fresh() {
    let summary_id = "summary.deployment.air_gapped_digest_mismatch";
    let strip_id = "strip.deployment.air_gapped_digest_mismatch";
    let mut row = baseline_artifact_row(
        summary_id,
        ArtifactClass::PolicyBundle,
        SignerStateClass::SignedOfflineTrustRoot,
        DigestStateClass::DigestMismatch,
        MirrorFreshnessClass::MirrorPastExtendedWindow,
        OfflineCachePostureClass::OfflineBundlePresent,
        MirrorSourceClass::OfflineBundleDerivedMirror,
    );
    // Sabotage the verify action so the audit fires.
    row.verify_action.revalidation_on_open = "snapshot_open_read_only".to_owned();
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        deployment_profile: DeploymentProfileClass::AirGapped,
        product_facing_label_class: ProductFacingLabelClass::AirGappedMirrorOnly,
        tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
        region_scope_class: RegionScopeClass::CustomerRegionPinned,
        retention_class: RetentionClass::CustomerRetentionWindow,
        key_mode_class: KeyModeClass::OfflineTrustRoot,
        mirror_offline_state_class: MirrorOfflineStateClass::OfflineAirGapped,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::UnboundedStale,
            summary_label: "Offline bundle past extended window.".to_owned(),
            last_control_plane_sync_at: None,
            cache_age_label: Some("Past extended freshness window".to_owned()),
            freshness_floor_ref: None,
            staleness_rationale: Some("No recent offline refresh recorded.".to_owned()),
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: Vec::new(),
        mirror_offline_artifact_row_refs: vec![row.row_id.clone()],
        plane_status_strip_ref: strip_id.to_owned(),
        prohibited_implied_claim_classes: vec![
            ProhibitedImpliedClaimClass::ImpliedAirGappedWhenEgressAllowed,
            ProhibitedImpliedClaimClass::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
        ],
        open_details_action: open_details_action(summary_id),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::CliTextFormatter,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.to_owned(),
        emitted_at: EMITTED_AT.to_owned(),
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
            summary_label: "Air-gapped install; no control plane in scope.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: Vec::new(),
            last_sync_at: None,
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Local-core capabilities available.".to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline(
            )
            .to_vec(),
        },
        safest_next_action: SafestNextAction::continue_local(format!(
            "action.strip.{strip_id}.continue_local"
        )),
        alternate_actions: Vec::new(),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.to_owned()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (_page, defects) = DeploymentProfilePage::compose(
        "page.deployment.air_gapped_digest_mismatch",
        profile_summary,
        plane_status_strip,
        Vec::new(),
        vec![row],
    );
    assert!(defects.iter().any(|d| matches!(
        d,
        DeploymentProfileDefect::DigestMismatchVerifyActionNotBlocked { .. }
    )));
}

#[test]
fn support_export_drops_non_export_safe_rows() {
    let mut page = individual_local_baseline_page(EMITTED_AT);
    let summary_id = page.profile_summary.summary_id.clone();
    // Add one export-safe and one widened row.
    let mut widened = baseline_residual_row(
        &summary_id,
        DependencyClass::PackageRegistry,
        PostureClass::Optional,
        AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
        ContinuityFallbackClass::ContinueLocalNoRestore,
    );
    widened.row_id = format!("row.dep.{summary_id}.package_registry.widened");
    widened.export_safe = true;
    widened.redaction_class = RedactionClass::InternalSupportRestricted;
    page.residual_dependency_rows.push(widened);
    let export_safe = baseline_residual_row(
        &summary_id,
        DependencyClass::DocsPack,
        PostureClass::Optional,
        AbsenceImpactClass::NarrowsToLocalCoreCapabilities,
        ContinuityFallbackClass::ContinueLocalNoRestore,
    );
    page.residual_dependency_rows.push(export_safe);

    let export = page.project_support_export();
    assert_eq!(export.residual_dependency_rows.len(), 1);
    assert_eq!(
        export.residual_dependency_rows[0].dependency_class,
        DependencyClass::DocsPack,
    );
    assert!(export.is_redaction_consistent());
}

#[test]
fn render_plaintext_is_deterministic_for_baseline() {
    let a = individual_local_baseline_page(EMITTED_AT).render_plaintext();
    let b = individual_local_baseline_page(EMITTED_AT).render_plaintext();
    assert_eq!(a, b);
    assert!(a.contains("individual_local"));
    assert!(a.contains("Continue local"));
}

#[test]
fn consumer_surfaces_present_unions_summary_and_strip_surfaces() {
    let page = individual_local_baseline_page(EMITTED_AT);
    let surfaces = consumer_surfaces_present(&page);
    assert!(surfaces.contains(&ConsumerSurfaceClass::AboutPanel));
    assert!(surfaces.contains(&ConsumerSurfaceClass::DiagnosticsView));
    assert!(surfaces.contains(&ConsumerSurfaceClass::StatusBarCell));
}

#[test]
fn record_kind_tags_round_trip_through_serde() {
    let page = individual_local_baseline_page(EMITTED_AT);
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: DeploymentProfilePage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.record_kind, DEPLOYMENT_PROFILE_PAGE_RECORD_KIND);
    assert_eq!(
        parsed.profile_summary.record_kind,
        PROFILE_SUMMARY_RECORD_KIND
    );
    assert_eq!(
        parsed.plane_status_strip.record_kind,
        PLANE_STATUS_STRIP_RECORD_KIND,
    );
}
