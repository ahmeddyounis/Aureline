// Canonical seed for the M5 live-resource navigation primitive. Included from
// `mod.rs` so the seeded builder, its worked cases, the fixture-emitting bin, and
// the on-disk support export all stay byte-aligned.

/// A stable workload identity.
fn workload_identity(uid: &str, tag: &str) -> M5ResourceIdentity {
    M5ResourceIdentity {
        resource_kind: M5ResourceKind::Workload,
        stable_id: format!("uid:{uid}"),
        namespace: Some(format!("namespace:{tag}")),
        project: Some(format!("project:{tag}")),
    }
}

/// A source-to-live navigator over an in-sync, live-fresh workload.
fn source_to_live_in_sync_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:web-deployment:0001".to_owned(),
        resource_ref: "resource:live/apps/deployments/web".to_owned(),
        resource_label: "web (deployment)".to_owned(),
        identity: workload_identity("web-deployment", "prod"),
        link_class: M5ResourceLinkClass::RenderedToLive,
        from_truth: TruthMode::Rendered,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::Live,
        freshness: M5ResourceFreshness::LiveFresh,
        confidence: M5DiscoveryConfidence::High,
        permission: M5PermissionPosture::FullAccess,
        health: M5ResourceHealth::Healthy,
        compare_verdict: M5CompareVerdict::InSync,
        target_identity_ref: "target:prod-us-east".to_owned(),
        permission_connection_note: None,
        divergence_note: None,
        available_actions: vec![
            M5ResourceActionKind::OpenDetail,
            M5ResourceActionKind::InspectDiff,
            M5ResourceActionKind::ViewLogs,
        ],
        degraded: None,
    }
}

/// A rendered / live compare card disclosing drift on a live-fresh network object.
fn rendered_live_drift_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:api-gateway:0002".to_owned(),
        resource_ref: "resource:live/networking/ingress/api".to_owned(),
        resource_label: "api-gateway (ingress)".to_owned(),
        identity: M5ResourceIdentity {
            resource_kind: M5ResourceKind::Network,
            stable_id: "uid:api-gateway".to_owned(),
            namespace: Some("namespace:prod".to_owned()),
            project: Some("project:prod".to_owned()),
        },
        link_class: M5ResourceLinkClass::RenderedToLive,
        from_truth: TruthMode::Rendered,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::Live,
        freshness: M5ResourceFreshness::LiveFresh,
        confidence: M5DiscoveryConfidence::High,
        permission: M5PermissionPosture::FullAccess,
        health: M5ResourceHealth::Degraded,
        compare_verdict: M5CompareVerdict::Drifted,
        target_identity_ref: "target:prod-us-east".to_owned(),
        permission_connection_note: None,
        divergence_note: Some(
            "live replica count 5 diverges from rendered desired 3".to_owned(),
        ),
        available_actions: vec![
            M5ResourceActionKind::OpenDetail,
            M5ResourceActionKind::InspectDiff,
            M5ResourceActionKind::ViewEvents,
        ],
        degraded: None,
    }
}

/// A cluster / resource explorer row showing cached, permission-limited data
/// (disclosed as not-current).
fn cluster_explorer_cached_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:cache-store:0003".to_owned(),
        resource_ref: "resource:live/apps/statefulset/cache".to_owned(),
        resource_label: "cache-store (statefulset)".to_owned(),
        identity: workload_identity("cache-store", "staging"),
        link_class: M5ResourceLinkClass::RenderedToLive,
        from_truth: TruthMode::Rendered,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::Live,
        freshness: M5ResourceFreshness::CachedStale,
        confidence: M5DiscoveryConfidence::Medium,
        permission: M5PermissionPosture::PermissionLimited,
        health: M5ResourceHealth::Progressing,
        compare_verdict: M5CompareVerdict::InSync,
        target_identity_ref: "target:staging-eu".to_owned(),
        permission_connection_note: Some(
            "namespace list restricted by RBAC; some resources hidden".to_owned(),
        ),
        divergence_note: None,
        available_actions: vec![
            M5ResourceActionKind::OpenDetail,
            M5ResourceActionKind::ViewLogs,
            M5ResourceActionKind::ViewEvents,
        ],
        degraded: None,
    }
}

/// A drift / unavailable banner narrowed by a lost live connector.
fn drift_banner_unavailable_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:payments-db:0004".to_owned(),
        resource_ref: "resource:live/apps/statefulset/payments".to_owned(),
        resource_label: "payments-db (statefulset)".to_owned(),
        identity: M5ResourceIdentity {
            resource_kind: M5ResourceKind::Storage,
            stable_id: "uid:payments-db".to_owned(),
            namespace: Some("namespace:prod".to_owned()),
            project: Some("project:prod".to_owned()),
        },
        link_class: M5ResourceLinkClass::RenderedToLive,
        from_truth: TruthMode::Rendered,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::Live,
        freshness: M5ResourceFreshness::CachedStale,
        confidence: M5DiscoveryConfidence::Low,
        permission: M5PermissionPosture::ConnectionLost,
        health: M5ResourceHealth::Unknown,
        compare_verdict: M5CompareVerdict::ComparisonUnavailable,
        target_identity_ref: "target:prod-eu-west".to_owned(),
        permission_connection_note: Some(
            "live cluster connector dropped mid-browse; showing last cached snapshot".to_owned(),
        ),
        divergence_note: None,
        available_actions: vec![
            M5ResourceActionKind::OpenDetail,
            M5ResourceActionKind::ViewEvents,
        ],
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            degraded_label: "live cluster connector dropped; resource browser fell back to last cached snapshot"
                .to_owned(),
        }),
    }
}

/// A provider-console handoff over an overlay-authoritative resource.
fn provider_console_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:load-balancer:0005".to_owned(),
        resource_ref: "resource:overlay/networking/lb".to_owned(),
        resource_label: "load-balancer (provider overlay)".to_owned(),
        identity: M5ResourceIdentity {
            resource_kind: M5ResourceKind::Network,
            stable_id: "uid:load-balancer".to_owned(),
            namespace: None,
            project: Some("project:prod".to_owned()),
        },
        link_class: M5ResourceLinkClass::CrossTarget,
        from_truth: TruthMode::ProviderOverlay,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::ProviderOverlay,
        freshness: M5ResourceFreshness::Unknown,
        confidence: M5DiscoveryConfidence::Medium,
        permission: M5PermissionPosture::ReadOnly,
        health: M5ResourceHealth::Unknown,
        compare_verdict: M5CompareVerdict::OverlayAuthoritative,
        target_identity_ref: "target:prod-us-east".to_owned(),
        permission_connection_note: None,
        divergence_note: None,
        available_actions: vec![
            M5ResourceActionKind::OpenInProviderConsole,
            M5ResourceActionKind::OpenDetail,
        ],
        degraded: None,
    }
}

/// A support / export replay reconstructed from an offline imported snapshot.
fn support_replay_input() -> M5LiveResourceInput {
    M5LiveResourceInput {
        resource_id: "resource:web-deployment-replay:0006".to_owned(),
        resource_ref: "resource:snapshot/apps/deployment/web".to_owned(),
        resource_label: "web-deployment (imported snapshot)".to_owned(),
        identity: workload_identity("web-deployment", "prod"),
        link_class: M5ResourceLinkClass::PlanToLive,
        from_truth: TruthMode::Plan,
        to_truth: TruthMode::Live,
        truth_mode: TruthMode::Plan,
        freshness: M5ResourceFreshness::ImportedSnapshot,
        confidence: M5DiscoveryConfidence::Medium,
        permission: M5PermissionPosture::Offline,
        health: M5ResourceHealth::Unknown,
        compare_verdict: M5CompareVerdict::ComparisonUnavailable,
        target_identity_ref: "target:prod-us-east".to_owned(),
        permission_connection_note: Some("offline replay; live target not reachable".to_owned()),
        divergence_note: None,
        available_actions: vec![
            M5ResourceActionKind::OpenDetail,
            M5ResourceActionKind::ViewEvents,
        ],
        degraded: None,
    }
}

fn case(input: M5LiveResourceInput) -> M5LiveResourceCase {
    M5LiveResourceCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5LiveResourceSurfaceRow> {
    let base_source_refs = vec![
        M5_LIVE_RESOURCE_SCHEMA_REF.to_owned(),
        M5_LIVE_RESOURCE_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5LiveResourceExportField::ALL.to_vec();

    vec![
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::SourceToLiveNavigator,
            owner_role: "Live-resource navigation guild".to_owned(),
            scope_summary:
                "Source-to-live links joining authored / rendered / live truth without blur"
                    .to_owned(),
            resource_kinds: vec![M5ResourceKind::Workload, M5ResourceKind::Network],
            truth_modes: vec![TruthMode::Rendered, TruthMode::Live],
            link_classes: vec![
                M5ResourceLinkClass::RenderedToLive,
                M5ResourceLinkClass::AuthoredToRendered,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            ],
            consumer_surfaces: vec!["resource_navigator".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_navigation: vec![
                case(source_to_live_in_sync_input()),
                case(rendered_live_drift_input()),
            ],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::RenderedLiveCompare,
            owner_role: "Rendered / live compare guild".to_owned(),
            scope_summary: "Compare cards naming exactly what diverged and what stays inspectable"
                .to_owned(),
            resource_kinds: vec![M5ResourceKind::Network, M5ResourceKind::Workload],
            truth_modes: vec![TruthMode::Rendered, TruthMode::Live],
            link_classes: vec![M5ResourceLinkClass::RenderedToLive],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
            ],
            consumer_surfaces: vec!["compare_card".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_navigation: vec![
                case(rendered_live_drift_input()),
                case(source_to_live_in_sync_input()),
            ],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::ClusterResourceExplorer,
            owner_role: "Cluster explorer guild".to_owned(),
            scope_summary:
                "Explorer rows with kind, identity, freshness, health, and permission notes"
                    .to_owned(),
            resource_kinds: vec![
                M5ResourceKind::Workload,
                M5ResourceKind::Config,
                M5ResourceKind::Storage,
            ],
            truth_modes: vec![TruthMode::Live],
            link_classes: vec![M5ResourceLinkClass::RenderedToLive],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            ],
            consumer_surfaces: vec!["resource_explorer".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_navigation: vec![case(cluster_explorer_cached_input())],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::DriftUnavailableBanner,
            owner_role: "Action-safety guild".to_owned(),
            scope_summary: "Drift / unavailable banners disclosing loss before any live action"
                .to_owned(),
            resource_kinds: vec![M5ResourceKind::Storage, M5ResourceKind::Workload],
            truth_modes: vec![TruthMode::Live],
            link_classes: vec![M5ResourceLinkClass::RenderedToLive],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
            ],
            consumer_surfaces: vec!["drift_banner".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_navigation: vec![case(drift_banner_unavailable_input())],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::ProviderConsoleHandoff,
            owner_role: "Provider-overlay guild".to_owned(),
            scope_summary: "Provider-console handoff naming overlay-authoritative live truth"
                .to_owned(),
            resource_kinds: vec![M5ResourceKind::Network, M5ResourceKind::CustomResource],
            truth_modes: vec![TruthMode::ProviderOverlay],
            link_classes: vec![M5ResourceLinkClass::CrossTarget],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            ],
            consumer_surfaces: vec!["provider_console".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_navigation: vec![case(provider_console_input())],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
        M5LiveResourceSurfaceRow {
            surface_family: M5LiveResourceSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing navigation truth from an imported snapshot"
                    .to_owned(),
            resource_kinds: vec![M5ResourceKind::Workload],
            truth_modes: vec![TruthMode::Plan],
            link_classes: vec![M5ResourceLinkClass::PlanToLive],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_navigation: vec![case(support_replay_input())],
            hides_truth_class: false,
            blurs_source_and_live: false,
            hides_drift_or_unavailability: false,
            presents_partial_as_current: false,
        },
    ]
}

fn seeded_governance_review() -> M5LiveResourceGovernanceReview {
    M5LiveResourceGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        resource_identity_preserved_across_surfaces: true,
        source_and_live_never_blurred: true,
        drift_and_unavailability_visible_before_action: true,
        partial_or_limited_never_shown_as_current: true,
        support_export_reconstructs_navigation: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5LiveResourceConsumerProjection {
    M5LiveResourceConsumerProjection {
        navigation_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        drift_banner_reads_single_status_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5LiveResourceReleasePosture {
    M5LiveResourceReleasePosture {
        release_packet_ref: M5_LIVE_RESOURCE_ARTIFACT_REF.to_owned(),
        navigation_audit_ref: M5_LIVE_RESOURCE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 live-resource navigation primitive packet.
/// This is the one source of truth shared by the tests, the fixture-emitting bin,
/// and the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_live_resource_packet() -> M5LiveResourcePrimitivePacket {
    M5LiveResourcePrimitivePacket::new(M5LiveResourcePrimitivePacketInput {
        packet_id: "m5-live-resource-navigation-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Live-Resource Navigation Primitive: Link Row, Compare Card, Explorer Row, and Drift Banner"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5LiveResourceVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_LIVE_RESOURCE_SCHEMA_REF.to_owned(),
            M5_LIVE_RESOURCE_DOC_REF.to_owned(),
            M5_LIVE_RESOURCE_COMPONENT_MATRIX_REF.to_owned(),
            M5_LIVE_RESOURCE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "infra_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
