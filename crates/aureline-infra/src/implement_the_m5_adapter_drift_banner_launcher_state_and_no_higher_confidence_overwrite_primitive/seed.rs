// Canonical seed for the M5 execution-confidence primitive. Included from
// `mod.rs` so the seeded builder, its worked cases, the fixture-emitting bin, and
// the on-disk support export all stay byte-aligned.

/// A stable execution-target identity.
fn execution_target_identity(
    node_kind: crate::M5TargetGraphNodeKind,
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

fn verb(
    verb: M5BuildVerb,
    prior: M5CapabilityState,
    current: M5CapabilityState,
) -> M5ExecutionVerbInput {
    M5ExecutionVerbInput {
        verb,
        prior_state: prior,
        current_state: current,
    }
}

/// A native build-server target with no adapter drift: the confidence baseline.
fn native_no_drift_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:web-api:build:0001".to_owned(),
        target_ref: "target:bsp/web-api/build".to_owned(),
        target_label: "web-api (build)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::BuildTarget,
            "web-api",
            "web-api",
        ),
        truth_mode: TruthMode::Live,
        prior_adapter: M5AdapterSourceKind::NativeBuildServer,
        current_adapter: M5AdapterSourceKind::NativeBuildServer,
        adapter_version: "bsp:2.1.0".to_owned(),
        confidence: M5DiscoveryConfidence::High,
        freshness: M5ResourceFreshness::LiveFresh,
        fallback_state: M5FallbackConfidenceState::StructuredHigh,
        verbs: vec![
            verb(
                M5BuildVerb::Build,
                M5CapabilityState::Supported,
                M5CapabilityState::Supported,
            ),
            verb(
                M5BuildVerb::Test,
                M5CapabilityState::Supported,
                M5CapabilityState::Supported,
            ),
            verb(
                M5BuildVerb::Run,
                M5CapabilityState::Supported,
                M5CapabilityState::Supported,
            ),
            verb(
                M5BuildVerb::Debug,
                M5CapabilityState::Supported,
                M5CapabilityState::Supported,
            ),
        ],
        affected_targets: vec![],
        divergence_note: None,
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::ArtifactView,
            M5ExecutionParitySurface::FollowOnAutomation,
        ],
        existing_confidence: M5DiscoveryConfidence::High,
        existing_adapter: M5AdapterSourceKind::NativeBuildServer,
        downgrade_acknowledged: false,
        downgrade_note: None,
        available_actions: vec![
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
        ],
        degraded: None,
    }
}

/// A native build-server target whose adapter dropped to a heuristic parse: verbs
/// downgraded and lost, confidence fell, and the overwrite is an explicit downgrade.
fn adapter_dropped_to_heuristic_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:web-api:run:0002".to_owned(),
        target_ref: "target:heuristic/web-api/run".to_owned(),
        target_label: "web-api (run)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::RunTarget,
            "web-api",
            "web-api",
        ),
        truth_mode: TruthMode::Plan,
        prior_adapter: M5AdapterSourceKind::NativeBuildServer,
        current_adapter: M5AdapterSourceKind::HeuristicParse,
        adapter_version: "heuristic:0.9.3".to_owned(),
        confidence: M5DiscoveryConfidence::Low,
        freshness: M5ResourceFreshness::Unknown,
        fallback_state: M5FallbackConfidenceState::HeuristicFallback,
        verbs: vec![
            verb(
                M5BuildVerb::Build,
                M5CapabilityState::Supported,
                M5CapabilityState::Partial,
            ),
            verb(
                M5BuildVerb::Test,
                M5CapabilityState::Supported,
                M5CapabilityState::Unknown,
            ),
            verb(
                M5BuildVerb::Run,
                M5CapabilityState::Supported,
                M5CapabilityState::Partial,
            ),
            verb(
                M5BuildVerb::Debug,
                M5CapabilityState::Supported,
                M5CapabilityState::Unsupported,
            ),
        ],
        affected_targets: vec![
            execution_target_identity(
                crate::M5TargetGraphNodeKind::RunTarget,
                "web-api",
                "web-api",
            ),
            execution_target_identity(
                crate::M5TargetGraphNodeKind::TestTarget,
                "web-api-e2e",
                "web-api",
            ),
        ],
        divergence_note: Some(
            "build-server adapter dropped; debug lost and build / test / run inferred from parsed scripts"
                .to_owned(),
        ),
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::ArtifactView,
            M5ExecutionParitySurface::FollowOnAutomation,
            M5ExecutionParitySurface::AiAction,
        ],
        existing_confidence: M5DiscoveryConfidence::High,
        existing_adapter: M5AdapterSourceKind::NativeBuildServer,
        downgrade_acknowledged: true,
        downgrade_note: Some(
            "keeping the prior native build-server truth; the heuristic parse is shown as a downgrade"
                .to_owned(),
        ),
        available_actions: vec![
            M5ExecutionActionKind::Recompute,
            M5ExecutionActionKind::OpenDiagnostics,
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
            M5ExecutionActionKind::AcknowledgeDowngrade,
        ],
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
            degraded_label:
                "build-server adapter unavailable; execution confidence fell to a heuristic parse of build scripts"
                    .to_owned(),
        }),
    }
}

/// A native build-event target whose structured channel was lost, forcing an
/// imported snapshot: a lower-confidence downgrade over prior native truth.
fn structured_channel_lost_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:web-api:test:0003".to_owned(),
        target_ref: "target:snapshot/web-api/test".to_owned(),
        target_label: "web-api (test, imported)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::TestTarget,
            "web-api",
            "web-api",
        ),
        truth_mode: TruthMode::Plan,
        prior_adapter: M5AdapterSourceKind::NativeBuildEvent,
        current_adapter: M5AdapterSourceKind::ImportedSnapshot,
        adapter_version: "snapshot:2026-06-30".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::ImportedSnapshot,
        fallback_state: M5FallbackConfidenceState::ImportedOnly,
        verbs: vec![
            verb(
                M5BuildVerb::Test,
                M5CapabilityState::Supported,
                M5CapabilityState::Partial,
            ),
            verb(
                M5BuildVerb::Coverage,
                M5CapabilityState::Partial,
                M5CapabilityState::Unsupported,
            ),
        ],
        affected_targets: vec![execution_target_identity(
            crate::M5TargetGraphNodeKind::TestTarget,
            "web-api",
            "web-api",
        )],
        divergence_note: Some(
            "live build-event channel lost; showing an imported snapshot with coverage no longer available"
                .to_owned(),
        ),
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::ArtifactView,
        ],
        existing_confidence: M5DiscoveryConfidence::High,
        existing_adapter: M5AdapterSourceKind::NativeBuildEvent,
        downgrade_acknowledged: true,
        downgrade_note: Some(
            "prior native build-event truth is preserved; the imported snapshot is labelled a downgrade"
                .to_owned(),
        ),
        available_actions: vec![
            M5ExecutionActionKind::Recompute,
            M5ExecutionActionKind::OpenDiagnostics,
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
            M5ExecutionActionKind::OpenSourceTruth,
        ],
        degraded: None,
    }
}

/// A provider-overlay container target whose test verb became provider-gated: a
/// capability drop that drifts without a confidence downgrade (adapter unchanged).
fn provider_overlay_gated_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:web-api:container:0004".to_owned(),
        target_ref: "target:overlay/web-api/container".to_owned(),
        target_label: "web-api (container)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::ContainerTarget,
            "web-api-image",
            "web-api",
        ),
        truth_mode: TruthMode::ProviderOverlay,
        prior_adapter: M5AdapterSourceKind::ProviderOverlay,
        current_adapter: M5AdapterSourceKind::ProviderOverlay,
        adapter_version: "overlay:registry-v2".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::CachedStale,
        fallback_state: M5FallbackConfidenceState::StructuredDegraded,
        verbs: vec![
            verb(
                M5BuildVerb::Build,
                M5CapabilityState::Supported,
                M5CapabilityState::Supported,
            ),
            verb(
                M5BuildVerb::Test,
                M5CapabilityState::Supported,
                M5CapabilityState::ProviderGated,
            ),
        ],
        affected_targets: vec![execution_target_identity(
            crate::M5TargetGraphNodeKind::ContainerTarget,
            "web-api-image",
            "web-api",
        )],
        divergence_note: Some(
            "registry policy now gates the container test verb; build remains available".to_owned(),
        ),
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::FollowOnAutomation,
        ],
        existing_confidence: M5DiscoveryConfidence::Medium,
        existing_adapter: M5AdapterSourceKind::ProviderOverlay,
        downgrade_acknowledged: false,
        downgrade_note: None,
        available_actions: vec![
            M5ExecutionActionKind::Recompute,
            M5ExecutionActionKind::OpenDiagnostics,
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
        ],
        degraded: None,
    }
}

/// A run target whose adapter recovered from a heuristic parse back to a native
/// build server: verbs regained, confidence promoted.
fn recompute_recovered_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:legacy-service:run:0005".to_owned(),
        target_ref: "target:bsp/legacy-service/run".to_owned(),
        target_label: "legacy-service (run)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::RunTarget,
            "legacy-service",
            "legacy",
        ),
        truth_mode: TruthMode::Live,
        prior_adapter: M5AdapterSourceKind::HeuristicParse,
        current_adapter: M5AdapterSourceKind::NativeBuildServer,
        adapter_version: "bsp:2.1.0".to_owned(),
        confidence: M5DiscoveryConfidence::High,
        freshness: M5ResourceFreshness::LiveFresh,
        fallback_state: M5FallbackConfidenceState::StructuredHigh,
        verbs: vec![
            verb(
                M5BuildVerb::Build,
                M5CapabilityState::Partial,
                M5CapabilityState::Supported,
            ),
            verb(
                M5BuildVerb::Run,
                M5CapabilityState::Unknown,
                M5CapabilityState::Supported,
            ),
        ],
        affected_targets: vec![execution_target_identity(
            crate::M5TargetGraphNodeKind::RunTarget,
            "legacy-service",
            "legacy",
        )],
        divergence_note: Some(
            "build-server adapter re-attached after recompute; verbs restored to native confidence"
                .to_owned(),
        ),
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::ArtifactView,
            M5ExecutionParitySurface::AiAction,
        ],
        existing_confidence: M5DiscoveryConfidence::Low,
        existing_adapter: M5AdapterSourceKind::HeuristicParse,
        downgrade_acknowledged: false,
        downgrade_note: None,
        available_actions: vec![
            M5ExecutionActionKind::Recompute,
            M5ExecutionActionKind::OpenDiagnostics,
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
        ],
        degraded: None,
    }
}

/// A support / export replay reconstructed from an imported log: verbs downgraded,
/// confidence held as an explicit downgrade for offline review.
fn support_replay_input() -> M5ExecutionConfidenceInput {
    M5ExecutionConfidenceInput {
        target_id: "target:reporting:replay:0006".to_owned(),
        target_ref: "target:snapshot/reporting/replay".to_owned(),
        target_label: "reporting (support replay)".to_owned(),
        identity: execution_target_identity(
            crate::M5TargetGraphNodeKind::BuildTarget,
            "reporting",
            "reporting",
        ),
        truth_mode: TruthMode::Plan,
        prior_adapter: M5AdapterSourceKind::ImportedSnapshot,
        current_adapter: M5AdapterSourceKind::ImportedSnapshot,
        adapter_version: "snapshot:2026-06-30".to_owned(),
        confidence: M5DiscoveryConfidence::Medium,
        freshness: M5ResourceFreshness::ImportedSnapshot,
        fallback_state: M5FallbackConfidenceState::ImportedOnly,
        verbs: vec![
            verb(
                M5BuildVerb::Build,
                M5CapabilityState::Supported,
                M5CapabilityState::Partial,
            ),
            verb(
                M5BuildVerb::Test,
                M5CapabilityState::Supported,
                M5CapabilityState::Unknown,
            ),
        ],
        affected_targets: vec![execution_target_identity(
            crate::M5TargetGraphNodeKind::BuildTarget,
            "reporting",
            "reporting",
        )],
        divergence_note: Some(
            "offline replay reconstructed from an imported build log; live target not reachable"
                .to_owned(),
        ),
        parity_consumers: vec![
            M5ExecutionParitySurface::ProblemSurface,
            M5ExecutionParitySurface::AiAction,
        ],
        existing_confidence: M5DiscoveryConfidence::High,
        existing_adapter: M5AdapterSourceKind::ImportedSnapshot,
        downgrade_acknowledged: true,
        downgrade_note: Some(
            "the higher-confidence prior run is preserved; this imported replay is a downgrade"
                .to_owned(),
        ),
        available_actions: vec![
            M5ExecutionActionKind::Recompute,
            M5ExecutionActionKind::OpenDiagnostics,
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
            M5ExecutionActionKind::OpenSourceTruth,
        ],
        degraded: None,
    }
}

fn case(input: M5ExecutionConfidenceInput) -> M5ExecutionConfidenceCase {
    M5ExecutionConfidenceCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5ExecutionSurfaceRow> {
    let base_source_refs = vec![
        M5_EXECUTION_CONFIDENCE_SCHEMA_REF.to_owned(),
        M5_EXECUTION_CONFIDENCE_COMPONENT_MATRIX_REF.to_owned(),
        M5_EXECUTION_CONFIDENCE_BUILD_PRIMITIVE_REF.to_owned(),
    ];
    let all_export_fields = M5ExecutionExportField::ALL.to_vec();

    vec![
        M5ExecutionSurfaceRow {
            surface_family: M5ExecutionSurfaceFamily::AdapterDriftBanner,
            owner_role: "Build-adapter confidence guild".to_owned(),
            scope_summary:
                "Adapter-drift banners naming prior versus current adapter, capability delta, affected targets, and recompute / diagnostics actions"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::NativeBuildServer,
                M5AdapterSourceKind::HeuristicParse,
                M5AdapterSourceKind::ImportedSnapshot,
            ],
            truth_modes: vec![TruthMode::Live, TruthMode::Plan],
            build_verbs: vec![
                M5BuildVerb::Build,
                M5BuildVerb::Test,
                M5BuildVerb::Run,
                M5BuildVerb::Debug,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
            ],
            consumer_surfaces: vec!["run_test_debug_launcher".to_owned(), "problem_surface".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(adapter_dropped_to_heuristic_input()),
                case(structured_channel_lost_input()),
            ],
            hides_adapter_source: false,
            narrows_after_launch: false,
            hides_drift: false,
            allows_silent_overwrite: false,
        },
        M5ExecutionSurfaceRow {
            surface_family: M5ExecutionSurfaceFamily::ExecutionLauncher,
            owner_role: "Execution-launcher guild".to_owned(),
            scope_summary:
                "Run / test / debug launchers narrowing affordances before launch when adapter capability drops"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::NativeBuildServer,
                M5AdapterSourceKind::HeuristicParse,
            ],
            truth_modes: vec![TruthMode::Live, TruthMode::Plan],
            build_verbs: vec![
                M5BuildVerb::Build,
                M5BuildVerb::Test,
                M5BuildVerb::Run,
                M5BuildVerb::Debug,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
            ],
            consumer_surfaces: vec!["run_test_debug_launcher".to_owned(), "artifact_view".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(adapter_dropped_to_heuristic_input()),
                case(native_no_drift_input()),
            ],
            hides_adapter_source: false,
            narrows_after_launch: false,
            hides_drift: false,
            allows_silent_overwrite: false,
        },
        M5ExecutionSurfaceRow {
            surface_family: M5ExecutionSurfaceFamily::LauncherStateParity,
            owner_role: "Execution-parity guild".to_owned(),
            scope_summary:
                "Launcher-state parity carrying adapter source and confidence into problem surfaces, artifact views, and follow-on automation / AI"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::ProviderOverlay,
                M5AdapterSourceKind::NativeBuildServer,
            ],
            truth_modes: vec![TruthMode::ProviderOverlay, TruthMode::Live],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Test, M5BuildVerb::Run],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
            ],
            consumer_surfaces: vec!["problem_surface".to_owned(), "ai_review".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(provider_overlay_gated_input()),
                case(recompute_recovered_input()),
            ],
            hides_adapter_source: false,
            narrows_after_launch: false,
            hides_drift: false,
            allows_silent_overwrite: false,
        },
        M5ExecutionSurfaceRow {
            surface_family: M5ExecutionSurfaceFamily::OverwriteGuard,
            owner_role: "Confidence-integrity guild".to_owned(),
            scope_summary:
                "No-higher-confidence overwrite guard refusing to replace existing native / higher truth without an explicit downgrade"
                    .to_owned(),
            adapter_source_kinds: vec![
                M5AdapterSourceKind::ImportedSnapshot,
                M5AdapterSourceKind::NativeBuildEvent,
            ],
            truth_modes: vec![TruthMode::Plan, TruthMode::Live],
            build_verbs: vec![M5BuildVerb::Test, M5BuildVerb::Coverage, M5BuildVerb::Build],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
            ],
            consumer_surfaces: vec!["overwrite_guard".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_confidence: vec![
                case(structured_channel_lost_input()),
                case(support_replay_input()),
            ],
            hides_adapter_source: false,
            narrows_after_launch: false,
            hides_drift: false,
            allows_silent_overwrite: false,
        },
        M5ExecutionSurfaceRow {
            surface_family: M5ExecutionSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing execution confidence from an imported log for support and AI"
                    .to_owned(),
            adapter_source_kinds: vec![M5AdapterSourceKind::ImportedSnapshot],
            truth_modes: vec![TruthMode::Plan],
            build_verbs: vec![M5BuildVerb::Build, M5BuildVerb::Test],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "ai_review".to_owned()],
            source_contract_refs: base_source_refs,
            example_confidence: vec![case(support_replay_input())],
            hides_adapter_source: false,
            narrows_after_launch: false,
            hides_drift: false,
            allows_silent_overwrite: false,
        },
    ]
}

fn seeded_governance_review() -> M5ExecutionConfidenceGovernanceReview {
    M5ExecutionConfidenceGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        target_identity_preserved_across_surfaces: true,
        adapter_drift_visible_before_action: true,
        affordances_narrow_before_launch: true,
        lower_confidence_never_overwrites_silently: true,
        launcher_state_parity_carries_source_and_confidence: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5ExecutionConfidenceConsumerProjection {
    M5ExecutionConfidenceConsumerProjection {
        execution_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        launchers_read_single_confidence_source: true,
        support_and_ai_reuse_shared_component: true,
    }
}

fn seeded_release_posture() -> M5ExecutionConfidenceReleasePosture {
    M5ExecutionConfidenceReleasePosture {
        release_packet_ref: M5_EXECUTION_CONFIDENCE_ARTIFACT_REF.to_owned(),
        confidence_audit_ref: M5_EXECUTION_CONFIDENCE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 execution-confidence primitive packet. This
/// is the one source of truth shared by the tests, the fixture-emitting bin, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_execution_confidence_packet() -> M5ExecutionConfidencePrimitivePacket {
    M5ExecutionConfidencePrimitivePacket::new(M5ExecutionConfidencePrimitivePacketInput {
        packet_id: "m5-execution-confidence-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Execution-Confidence Primitive: Adapter-Drift Banner, Launcher State, Launcher-State Parity, and Overwrite Guard"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5ExecutionConfidenceVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_EXECUTION_CONFIDENCE_SCHEMA_REF.to_owned(),
            M5_EXECUTION_CONFIDENCE_DOC_REF.to_owned(),
            M5_EXECUTION_CONFIDENCE_COMPONENT_MATRIX_REF.to_owned(),
            M5_EXECUTION_CONFIDENCE_BUILD_PRIMITIVE_REF.to_owned(),
            M5_EXECUTION_CONFIDENCE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "infra_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
