// Canonical seed for the M5 build / run confidence primitive. Included from
// `mod.rs` so the seeded builder, its worked cases, the fixture-emitting bin, and
// the on-disk support export all stay byte-aligned.

/// A stable build-target identity.
fn build_target_identity(
    node_kind: M5TargetGraphNodeKind,
    uid: &str,
    module: &str,
) -> M5TargetIdentity {
    M5TargetIdentity {
        node_kind,
        stable_id: format!("target:{uid}"),
        owning_module: format!("module:{module}"),
        workspace_root: "root:workspace".to_owned(),
    }
}

fn cap(verb: M5BuildVerb, state: M5CapabilityState) -> M5CapabilityCell {
    M5CapabilityCell { verb, state }
}

/// A native build-server (BSP) target: fully structured, high-confidence.
fn native_build_server_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:web-api:build:0001".to_owned(),
        target_ref: "target:bsp/web-api/build".to_owned(),
        target_label: "web-api (build)".to_owned(),
        identity: build_target_identity(M5TargetGraphNodeKind::BuildTarget, "web-api", "web-api"),
        truth_mode: TruthMode::Live,
        adapter_source: M5AdapterSourceKind::NativeBuildServer,
        adapter_version: "bsp:2.1.0".to_owned(),
        confidence: M5DiscoveryConfidence::High,
        freshness: M5ResourceFreshness::LiveFresh,
        required_environment: vec!["env:NODE_ENV".to_owned(), "env:CI".to_owned()],
        event_channel: M5RawEventChannel::NativeBuildServer,
        payload_lineage: vec![M5RawEventChannel::NativeBuildServer],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Supported),
            cap(M5BuildVerb::Test, M5CapabilityState::Supported),
            cap(M5BuildVerb::Run, M5CapabilityState::Supported),
            cap(M5BuildVerb::Debug, M5CapabilityState::Supported),
        ],
        fallback_state: M5FallbackConfidenceState::StructuredHigh,
        fallback_reason: None,
        recovery_route: M5FallbackRecoveryRoute::RerunDiscovery,
        fallback_note: None,
        target_identity_ref: "target:web-api".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::ViewRawEvents,
            M5BuildActionKind::CopyExport,
            M5BuildActionKind::OpenTargetGraph,
        ],
        degraded: None,
    }
}

/// A native build-event (BEP) test target: structured but degraded, with some
/// downgraded verbs.
fn native_build_event_partial_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:web-api:test:0002".to_owned(),
        target_ref: "target:bep/web-api/test".to_owned(),
        target_label: "web-api (test)".to_owned(),
        identity: build_target_identity(M5TargetGraphNodeKind::TestTarget, "web-api", "web-api"),
        truth_mode: TruthMode::Live,
        adapter_source: M5AdapterSourceKind::NativeBuildEvent,
        adapter_version: "bep:6.4.0".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::LiveFresh,
        required_environment: vec!["env:TEST_SHARD".to_owned()],
        event_channel: M5RawEventChannel::NativeBuildEvent,
        payload_lineage: vec![
            M5RawEventChannel::NativeBuildEvent,
            M5RawEventChannel::TaskEventBus,
        ],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Supported),
            cap(M5BuildVerb::Test, M5CapabilityState::Supported),
            cap(M5BuildVerb::Debug, M5CapabilityState::Partial),
            cap(M5BuildVerb::Coverage, M5CapabilityState::Unsupported),
        ],
        fallback_state: M5FallbackConfidenceState::StructuredDegraded,
        fallback_reason: None,
        recovery_route: M5FallbackRecoveryRoute::RerunDiscovery,
        fallback_note: None,
        target_identity_ref: "target:web-api".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::ViewRawEvents,
            M5BuildActionKind::CopyExport,
        ],
        degraded: None,
    }
}

/// A heuristic-parse fallback run target: low confidence, disclosed as a fallback.
fn heuristic_fallback_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:legacy-service:run:0003".to_owned(),
        target_ref: "target:heuristic/legacy-service/run".to_owned(),
        target_label: "legacy-service (run)".to_owned(),
        identity: build_target_identity(
            M5TargetGraphNodeKind::RunTarget,
            "legacy-service",
            "legacy",
        ),
        truth_mode: TruthMode::Plan,
        adapter_source: M5AdapterSourceKind::HeuristicParse,
        adapter_version: "heuristic:0.9.3".to_owned(),
        confidence: M5DiscoveryConfidence::Low,
        freshness: M5ResourceFreshness::Unknown,
        required_environment: vec!["env:PORT".to_owned()],
        event_channel: M5RawEventChannel::HeuristicParse,
        payload_lineage: vec![M5RawEventChannel::HeuristicParse],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Partial),
            cap(M5BuildVerb::Run, M5CapabilityState::Partial),
            cap(M5BuildVerb::Test, M5CapabilityState::Unknown),
            cap(M5BuildVerb::Debug, M5CapabilityState::Unsupported),
        ],
        fallback_state: M5FallbackConfidenceState::HeuristicFallback,
        fallback_reason: Some(M5FallbackReason::AdapterUnavailable),
        recovery_route: M5FallbackRecoveryRoute::ReattachAdapter,
        fallback_note: Some(
            "no build-server adapter attached; verbs inferred from parsed scripts".to_owned(),
        ),
        target_identity_ref: "target:legacy-service".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::ViewRawEvents,
            M5BuildActionKind::CopyExport,
            M5BuildActionKind::OpenSourceTruth,
        ],
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
            degraded_label:
                "build-server adapter unavailable; confidence fell to a heuristic parse of build scripts"
                    .to_owned(),
        }),
    }
}

/// An imported-snapshot fallback build target: reconstructed from a prior run.
fn imported_snapshot_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:reporting:build:0004".to_owned(),
        target_ref: "target:snapshot/reporting/build".to_owned(),
        target_label: "reporting (build, imported)".to_owned(),
        identity: build_target_identity(
            M5TargetGraphNodeKind::BuildTarget,
            "reporting",
            "reporting",
        ),
        truth_mode: TruthMode::Plan,
        adapter_source: M5AdapterSourceKind::ImportedSnapshot,
        adapter_version: "snapshot:2026-06-30".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::ImportedSnapshot,
        required_environment: vec!["env:JAVA_HOME".to_owned()],
        event_channel: M5RawEventChannel::ImportedLog,
        payload_lineage: vec![
            M5RawEventChannel::ImportedLog,
            M5RawEventChannel::NativeBuildEvent,
        ],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Supported),
            cap(M5BuildVerb::Test, M5CapabilityState::Partial),
            cap(M5BuildVerb::Package, M5CapabilityState::Unknown),
        ],
        fallback_state: M5FallbackConfidenceState::ImportedOnly,
        fallback_reason: Some(M5FallbackReason::StructuredChannelLost),
        recovery_route: M5FallbackRecoveryRoute::RerunDiscovery,
        fallback_note: Some(
            "live build-event channel lost; showing an imported snapshot from the last run".to_owned(),
        ),
        target_identity_ref: "target:reporting".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::CopyExport,
            M5BuildActionKind::OpenSourceTruth,
        ],
        degraded: None,
    }
}

/// A provider-overlay container target: gated capabilities, structured-degraded.
fn provider_overlay_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:web-api:container:0005".to_owned(),
        target_ref: "target:overlay/web-api/container".to_owned(),
        target_label: "web-api (container)".to_owned(),
        identity: build_target_identity(
            M5TargetGraphNodeKind::ContainerTarget,
            "web-api-image",
            "web-api",
        ),
        truth_mode: TruthMode::ProviderOverlay,
        adapter_source: M5AdapterSourceKind::ProviderOverlay,
        adapter_version: "overlay:registry-v2".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::CachedStale,
        required_environment: vec!["env:REGISTRY_TOKEN_REF".to_owned()],
        event_channel: M5RawEventChannel::TaskEventBus,
        payload_lineage: vec![M5RawEventChannel::TaskEventBus],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Supported),
            cap(M5BuildVerb::Run, M5CapabilityState::Supported),
            cap(M5BuildVerb::Test, M5CapabilityState::ProviderGated),
        ],
        fallback_state: M5FallbackConfidenceState::StructuredDegraded,
        fallback_reason: None,
        recovery_route: M5FallbackRecoveryRoute::InspectOnly,
        fallback_note: None,
        target_identity_ref: "target:web-api".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::CopyExport,
            M5BuildActionKind::OpenTargetGraph,
        ],
        degraded: None,
    }
}

/// A support / export replay reconstructed from an imported log.
fn support_replay_input() -> M5BuildConfidenceInput {
    M5BuildConfidenceInput {
        target_id: "target:reporting:replay:0006".to_owned(),
        target_ref: "target:snapshot/reporting/replay".to_owned(),
        target_label: "reporting (support replay)".to_owned(),
        identity: build_target_identity(
            M5TargetGraphNodeKind::BuildTarget,
            "reporting",
            "reporting",
        ),
        truth_mode: TruthMode::Plan,
        adapter_source: M5AdapterSourceKind::ImportedSnapshot,
        adapter_version: "snapshot:2026-06-30".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::ImportedSnapshot,
        required_environment: vec!["env:JAVA_HOME".to_owned()],
        event_channel: M5RawEventChannel::ImportedLog,
        payload_lineage: vec![M5RawEventChannel::ImportedLog],
        capabilities: vec![
            cap(M5BuildVerb::Build, M5CapabilityState::Partial),
            cap(M5BuildVerb::Test, M5CapabilityState::Unknown),
        ],
        fallback_state: M5FallbackConfidenceState::ImportedOnly,
        fallback_reason: Some(M5FallbackReason::ConnectorLoss),
        recovery_route: M5FallbackRecoveryRoute::RetryConnector,
        fallback_note: Some(
            "offline replay reconstructed from an imported build log; live target not reachable"
                .to_owned(),
        ),
        target_identity_ref: "target:reporting".to_owned(),
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::CopyExport,
            M5BuildActionKind::OpenSourceTruth,
        ],
        degraded: None,
    }
}

fn case(input: M5BuildConfidenceInput) -> M5BuildConfidenceCase {
    M5BuildConfidenceCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5BuildConfidenceSurfaceRow> {
    let base_source_refs = vec![
        M5_BUILD_CONFIDENCE_SCHEMA_REF.to_owned(),
        M5_BUILD_CONFIDENCE_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5BuildConfidenceExportField::ALL.to_vec();

    vec![
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::AdapterSourceBadge,
            owner_role: "Build-adapter confidence guild".to_owned(),
            scope_summary:
                "Adapter-source badges and confidence chips naming native versus fallback lanes"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::NativeBuildServer,
                M5AdapterSourceKind::HeuristicParse,
            ],
            truth_modes: vec![TruthMode::Live, TruthMode::Plan],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Run],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
            ],
            consumer_surfaces: vec!["run_test_debug_launcher".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(native_build_server_input()),
                case(heuristic_fallback_input()),
            ],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::TargetGraphRow,
            owner_role: "Target-graph guild".to_owned(),
            scope_summary:
                "Target-graph rows preserving stable target id, owning module / root, freshness, verbs, and required env"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::NativeBuildEvent,
                M5AdapterSourceKind::NativeBuildServer,
            ],
            truth_modes: vec![TruthMode::Live],
            build_verbs: vec![
                M5BuildVerb::Build,
                M5BuildVerb::Test,
                M5BuildVerb::Debug,
                M5BuildVerb::Coverage,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
                M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
            ],
            consumer_surfaces: vec!["target_graph_view".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(native_build_event_partial_input()),
                case(native_build_server_input()),
            ],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::CapabilityMatrixSheet,
            owner_role: "Capability-matrix guild".to_owned(),
            scope_summary:
                "Capability-matrix sheets explaining supported verbs and downgraded actions before any run"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::NativeBuildEvent,
                M5AdapterSourceKind::ProviderOverlay,
            ],
            truth_modes: vec![TruthMode::Live, TruthMode::ProviderOverlay],
            build_verbs: vec![
                M5BuildVerb::Build,
                M5BuildVerb::Test,
                M5BuildVerb::Run,
                M5BuildVerb::Debug,
                M5BuildVerb::Coverage,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
            ],
            consumer_surfaces: vec!["capability_matrix".to_owned(), "ai_review".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(native_build_event_partial_input()),
                case(provider_overlay_input()),
            ],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::RawEventDrawer,
            owner_role: "Event-interoperability guild".to_owned(),
            scope_summary:
                "Raw-event drawers disclosing payload lineage, adapter version, and export / copy actions"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::HeuristicParse,
                M5AdapterSourceKind::NativeBuildServer,
            ],
            truth_modes: vec![TruthMode::Plan, TruthMode::Live],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Run],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
                M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
            ],
            consumer_surfaces: vec!["raw_event_drawer".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(heuristic_fallback_input()),
                case(native_build_server_input()),
            ],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::FallbackConfidenceDrawer,
            owner_role: "Fallback-confidence guild".to_owned(),
            scope_summary:
                "Fallback-confidence drawers naming why confidence fell and the recovery route"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::ImportedSnapshot,
                M5AdapterSourceKind::HeuristicParse,
            ],
            truth_modes: vec![TruthMode::Plan],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Test, M5BuildVerb::Package],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
                M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            ],
            consumer_surfaces: vec!["fallback_drawer".to_owned(), "ai_review".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(imported_snapshot_input()),
                case(heuristic_fallback_input()),
            ],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
        M5BuildConfidenceSurfaceRow {
            surface_family: M5BuildConfidenceSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing build confidence from an imported log for support and AI"
                    .to_owned(),
            adapter_source_kinds: vec![M5AdapterSourceKind::ImportedSnapshot],
            truth_modes: vec![TruthMode::Plan],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Test],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "ai_review".to_owned()],
            source_contract_refs: base_source_refs,
            example_confidence: vec![case(support_replay_input())],
            hides_adapter_source: false,
            blurs_structured_and_fallback: false,
            hides_target_identity: false,
            presents_fallback_as_structured: false,
        },
    ]
}

fn seeded_governance_review() -> M5BuildConfidenceGovernanceReview {
    M5BuildConfidenceGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        target_identity_preserved_across_surfaces: true,
        adapter_source_never_hidden: true,
        structured_and_fallback_never_blurred: true,
        identity_and_confidence_inspectable_before_action: true,
        support_export_reconstructs_confidence: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BuildConfidenceConsumerProjection {
    M5BuildConfidenceConsumerProjection {
        confidence_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        capability_matrix_reads_single_source: true,
        support_and_ai_reuse_shared_component: true,
    }
}

fn seeded_release_posture() -> M5BuildConfidenceReleasePosture {
    M5BuildConfidenceReleasePosture {
        release_packet_ref: M5_BUILD_CONFIDENCE_ARTIFACT_REF.to_owned(),
        confidence_audit_ref: M5_BUILD_CONFIDENCE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 build / run confidence primitive packet.
/// This is the one source of truth shared by the tests, the fixture-emitting bin,
/// and the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_build_confidence_packet() -> M5BuildConfidencePrimitivePacket {
    M5BuildConfidencePrimitivePacket::new(M5BuildConfidencePrimitivePacketInput {
        packet_id: "m5-build-confidence-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Build / Run Confidence Primitive: Adapter Badge, Target-Graph Row, Capability Matrix, Raw-Event Drawer, and Fallback Drawer"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BuildConfidenceVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_BUILD_CONFIDENCE_SCHEMA_REF.to_owned(),
            M5_BUILD_CONFIDENCE_DOC_REF.to_owned(),
            M5_BUILD_CONFIDENCE_COMPONENT_MATRIX_REF.to_owned(),
            M5_BUILD_CONFIDENCE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "infra_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
