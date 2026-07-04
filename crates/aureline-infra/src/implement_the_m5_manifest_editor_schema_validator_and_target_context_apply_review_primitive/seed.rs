// Canonical seed for the M5 manifest-authoring primitive. Included from `mod.rs`
// so the seeded builder, its worked cases, the fixture-emitting bin, and the
// on-disk support export all stay byte-aligned.

/// A full cluster / project / namespace / account context.
fn full_context(tag: &str) -> M5TargetContextChips {
    M5TargetContextChips {
        account: Some(format!("account:{tag}")),
        project: Some(format!("project:{tag}")),
        cluster: Some(format!("cluster:{tag}")),
        namespace: Some(format!("namespace:{tag}")),
    }
}

/// An apply-ready authored manifest on a connected cluster.
fn apply_ready_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:web-deployment:0001".to_owned(),
        manifest_ref: "manifest:web-deployment.yaml".to_owned(),
        manifest_label: "web-deployment.yaml".to_owned(),
        source_type: M5ManifestSourceType::AuthoredFile,
        truth_mode: TruthMode::Desired,
        schema_source: M5SchemaSourceKind::BundledWithApp,
        schema_version_label: Some("v1.29 (2026-05-01)".to_owned()),
        schema_freshness: M5SchemaFreshness::Fresh,
        validation_state: M5SchemaValidationState::Valid,
        policy_offline_note: None,
        docs_ref: Some("docs:schema/apps-v1".to_owned()),
        edit_posture: M5ManifestEditPosture::PreviewApplyReview,
        execution_origin: M5ExecutionOrigin::ConnectedCluster,
        target_identity_ref: "target:prod-us-east".to_owned(),
        target_context: full_context("prod-us-east"),
        mutation_counts: Some(M5MutationCounts {
            creates: 2,
            updates: 1,
            deletes: 0,
        }),
        dry_run: M5DryRunAvailability::Available,
        rollback: M5RollbackPosture::CheckpointAvailable,
        degraded: None,
    }
}

/// A read-only rendered artifact view.
fn read_only_rendered_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:web-deployment-rendered:0002".to_owned(),
        manifest_ref: "manifest:web-deployment.rendered.yaml".to_owned(),
        manifest_label: "web-deployment.rendered.yaml".to_owned(),
        source_type: M5ManifestSourceType::RenderedArtifact,
        truth_mode: TruthMode::Rendered,
        schema_source: M5SchemaSourceKind::ClusterDiscovered,
        schema_version_label: Some("v1.29".to_owned()),
        schema_freshness: M5SchemaFreshness::Fresh,
        validation_state: M5SchemaValidationState::Valid,
        policy_offline_note: None,
        docs_ref: Some("docs:schema/apps-v1".to_owned()),
        edit_posture: M5ManifestEditPosture::ReadOnly,
        execution_origin: M5ExecutionOrigin::LocalWorkspace,
        target_identity_ref: "target:prod-us-east".to_owned(),
        target_context: full_context("prod-us-east"),
        mutation_counts: None,
        dry_run: M5DryRunAvailability::NotApplicable,
        rollback: M5RollbackPosture::Unknown,
        degraded: None,
    }
}

/// A plan-preview pane with a disclosed stale schema (apply still permitted).
fn plan_preview_stale_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:plan-preview:0003".to_owned(),
        manifest_ref: "manifest:ingress.yaml".to_owned(),
        manifest_label: "ingress.yaml".to_owned(),
        source_type: M5ManifestSourceType::AuthoredFile,
        truth_mode: TruthMode::Plan,
        schema_source: M5SchemaSourceKind::RemoteRegistry,
        schema_version_label: Some("v1.27 (2025-11-02)".to_owned()),
        schema_freshness: M5SchemaFreshness::Stale,
        validation_state: M5SchemaValidationState::Warnings,
        policy_offline_note: Some("schema snapshot older than target api version".to_owned()),
        docs_ref: Some("docs:schema/networking-v1".to_owned()),
        edit_posture: M5ManifestEditPosture::PreviewApplyReview,
        execution_origin: M5ExecutionOrigin::ConnectedCluster,
        target_identity_ref: "target:staging-eu".to_owned(),
        target_context: full_context("staging-eu"),
        mutation_counts: Some(M5MutationCounts {
            creates: 0,
            updates: 3,
            deletes: 0,
        }),
        dry_run: M5DryRunAvailability::Available,
        rollback: M5RollbackPosture::RollbackSupported,
        degraded: None,
    }
}

/// A live cluster / resource explorer view (read-only live truth).
fn live_explorer_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:live-explorer:0004".to_owned(),
        manifest_ref: "manifest:live/deployment/web".to_owned(),
        manifest_label: "web (live)".to_owned(),
        source_type: M5ManifestSourceType::RenderedArtifact,
        truth_mode: TruthMode::Live,
        schema_source: M5SchemaSourceKind::ClusterDiscovered,
        schema_version_label: Some("v1.29".to_owned()),
        schema_freshness: M5SchemaFreshness::Fresh,
        validation_state: M5SchemaValidationState::Valid,
        policy_offline_note: None,
        docs_ref: Some("docs:schema/apps-v1".to_owned()),
        edit_posture: M5ManifestEditPosture::ReadOnly,
        execution_origin: M5ExecutionOrigin::ConnectedCluster,
        target_identity_ref: "target:prod-us-east".to_owned(),
        target_context: full_context("prod-us-east"),
        mutation_counts: None,
        dry_run: M5DryRunAvailability::NotApplicable,
        rollback: M5RollbackPosture::Unknown,
        degraded: None,
    }
}

/// An apply-review dialog narrowed by a lost live connector (write path gated
/// before execution).
fn apply_review_degraded_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:apply-review:0005".to_owned(),
        manifest_ref: "manifest:statefulset.yaml".to_owned(),
        manifest_label: "statefulset.yaml".to_owned(),
        source_type: M5ManifestSourceType::AuthoredFile,
        truth_mode: TruthMode::Plan,
        schema_source: M5SchemaSourceKind::BundledWithApp,
        schema_version_label: Some("v1.29 (2026-05-01)".to_owned()),
        schema_freshness: M5SchemaFreshness::Fresh,
        validation_state: M5SchemaValidationState::Valid,
        policy_offline_note: None,
        docs_ref: Some("docs:schema/apps-v1".to_owned()),
        edit_posture: M5ManifestEditPosture::PreviewApplyReview,
        execution_origin: M5ExecutionOrigin::ConnectedCluster,
        target_identity_ref: "target:prod-eu-west".to_owned(),
        target_context: full_context("prod-eu-west"),
        mutation_counts: Some(M5MutationCounts {
            creates: 1,
            updates: 0,
            deletes: 1,
        }),
        dry_run: M5DryRunAvailability::UnavailableConnectorLost,
        rollback: M5RollbackPosture::CheckpointAvailable,
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
            degraded_label: "live cluster connector dropped mid-review; apply held".to_owned(),
        }),
    }
}

/// A provider-console handoff view (read-only provider overlay).
fn provider_console_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:provider-console:0006".to_owned(),
        manifest_ref: "manifest:overlay/load-balancer".to_owned(),
        manifest_label: "load-balancer (provider overlay)".to_owned(),
        source_type: M5ManifestSourceType::ProviderOverlay,
        truth_mode: TruthMode::ProviderOverlay,
        schema_source: M5SchemaSourceKind::ProviderOverlay,
        schema_version_label: None,
        schema_freshness: M5SchemaFreshness::Unversioned,
        validation_state: M5SchemaValidationState::Unversioned,
        policy_offline_note: Some("provider overlay carries no resolvable schema version".to_owned()),
        docs_ref: Some("docs:provider/console-handoff".to_owned()),
        edit_posture: M5ManifestEditPosture::ReadOnly,
        execution_origin: M5ExecutionOrigin::ProviderConsole,
        target_identity_ref: "target:prod-us-east".to_owned(),
        target_context: full_context("prod-us-east"),
        mutation_counts: None,
        dry_run: M5DryRunAvailability::NotApplicable,
        rollback: M5RollbackPosture::Unknown,
        degraded: None,
    }
}

/// A support / export replay view reconstructed from an imported snapshot.
fn support_replay_input() -> M5ManifestAuthoringInput {
    M5ManifestAuthoringInput {
        authoring_id: "authoring:support-replay:0007".to_owned(),
        manifest_ref: "manifest:snapshot/web-deployment".to_owned(),
        manifest_label: "web-deployment (imported snapshot)".to_owned(),
        source_type: M5ManifestSourceType::ImportedSnapshot,
        truth_mode: TruthMode::Plan,
        schema_source: M5SchemaSourceKind::ImportedSnapshot,
        schema_version_label: Some("v1.28 (imported 2026-02-14)".to_owned()),
        schema_freshness: M5SchemaFreshness::Stale,
        validation_state: M5SchemaValidationState::Valid,
        policy_offline_note: Some("offline replay; live target not reachable".to_owned()),
        docs_ref: Some("docs:support/export-replay".to_owned()),
        edit_posture: M5ManifestEditPosture::ReadOnly,
        execution_origin: M5ExecutionOrigin::ImportedReplay,
        target_identity_ref: "target:prod-us-east".to_owned(),
        target_context: full_context("prod-us-east"),
        mutation_counts: None,
        dry_run: M5DryRunAvailability::NotApplicable,
        rollback: M5RollbackPosture::NoRollback,
        degraded: None,
    }
}

fn case(input: M5ManifestAuthoringInput) -> M5ManifestAuthoringCase {
    M5ManifestAuthoringCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5ManifestAuthoringSurfaceRow> {
    let base_source_refs = vec![
        M5_MANIFEST_AUTHORING_SCHEMA_REF.to_owned(),
        M5_MANIFEST_AUTHORING_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5ManifestAuthoringExportField::ALL.to_vec();

    vec![
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::DesktopManifestEditor,
            owner_role: "Infrastructure editor guild".to_owned(),
            scope_summary: "Authoring headers over source files with preview/apply entry points"
                .to_owned(),
            source_types: vec![
                M5ManifestSourceType::AuthoredFile,
                M5ManifestSourceType::RenderedArtifact,
            ],
            truth_modes: vec![TruthMode::Desired, TruthMode::Rendered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::SchemaStale,
                M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
            ],
            consumer_surfaces: vec!["desktop_editor".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_authoring: vec![case(apply_ready_input()), case(read_only_rendered_input())],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::PlanPreviewPane,
            owner_role: "Plan / dry-run guild".to_owned(),
            scope_summary: "Plan diffs with disclosed schema freshness before apply".to_owned(),
            source_types: vec![M5ManifestSourceType::AuthoredFile],
            truth_modes: vec![TruthMode::Plan],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::SchemaStale,
                M5ManifestBuildDowngradeTrigger::DriftFromSource,
            ],
            consumer_surfaces: vec!["plan_preview".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_authoring: vec![case(plan_preview_stale_input())],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::ClusterResourceExplorer,
            owner_role: "Live-resource guild".to_owned(),
            scope_summary: "Read-only live explorer keeping target context and truth class visible"
                .to_owned(),
            source_types: vec![M5ManifestSourceType::RenderedArtifact],
            truth_modes: vec![TruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
            ],
            consumer_surfaces: vec!["resource_explorer".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_authoring: vec![case(live_explorer_input())],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::ApplyReviewDialog,
            owner_role: "Apply-safety guild".to_owned(),
            scope_summary: "Apply-review banner gating mutation on target, validation, and connector health"
                .to_owned(),
            source_types: vec![M5ManifestSourceType::AuthoredFile],
            truth_modes: vec![TruthMode::Plan, TruthMode::Desired],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
                M5ManifestBuildDowngradeTrigger::SchemaStale,
            ],
            consumer_surfaces: vec!["apply_review".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_authoring: vec![
                case(apply_review_degraded_input()),
                case(apply_ready_input()),
            ],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::ProviderConsoleHandoff,
            owner_role: "Provider-overlay guild".to_owned(),
            scope_summary: "Provider-console handoff naming overlay source and unversioned schema"
                .to_owned(),
            source_types: vec![M5ManifestSourceType::ProviderOverlay],
            truth_modes: vec![TruthMode::ProviderOverlay],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::SchemaStale,
                M5ManifestBuildDowngradeTrigger::PolicyBlock,
            ],
            consumer_surfaces: vec!["provider_console".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_authoring: vec![case(provider_console_input())],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
        M5ManifestAuthoringSurfaceRow {
            surface_family: M5ManifestAuthoringSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary: "Offline replay reconstructing authoring truth from an imported snapshot"
                .to_owned(),
            source_types: vec![M5ManifestSourceType::ImportedSnapshot],
            truth_modes: vec![TruthMode::Plan],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5ManifestBuildDowngradeTrigger::SchemaStale,
                M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_authoring: vec![case(support_replay_input())],
            hides_environment_or_schema_source: false,
            blurs_truth_states: false,
            hides_schema_freshness: false,
            offers_apply_before_review: false,
        },
    ]
}

fn seeded_governance_review() -> M5ManifestAuthoringGovernanceReview {
    M5ManifestAuthoringGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        authoring_identity_preserved_across_surfaces: true,
        environment_and_schema_source_never_hidden: true,
        states_explicit_before_mutation: true,
        schema_freshness_visible_where_trusted: true,
        support_export_reconstructs_authoring: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5ManifestAuthoringConsumerProjection {
    M5ManifestAuthoringConsumerProjection {
        authoring_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        apply_banner_reads_single_gate_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5ManifestAuthoringReleasePosture {
    M5ManifestAuthoringReleasePosture {
        release_packet_ref: M5_MANIFEST_AUTHORING_ARTIFACT_REF.to_owned(),
        authoring_audit_ref: M5_MANIFEST_AUTHORING_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 manifest-authoring primitive packet. This
/// is the one source of truth shared by the tests, the fixture-emitting bin, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_manifest_authoring_packet() -> M5ManifestAuthoringPrimitivePacket {
    M5ManifestAuthoringPrimitivePacket::new(M5ManifestAuthoringPrimitivePacketInput {
        packet_id: "m5-manifest-authoring-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Manifest-Authoring Primitive: Header, Schema/Validator Row, Chips, and Apply-Review Banner"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5ManifestAuthoringVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_MANIFEST_AUTHORING_SCHEMA_REF.to_owned(),
            M5_MANIFEST_AUTHORING_DOC_REF.to_owned(),
            M5_MANIFEST_AUTHORING_COMPONENT_MATRIX_REF.to_owned(),
            M5_MANIFEST_AUTHORING_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "infra_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
