// Canonical seed for the M5 deployment-profile primitive. Included from `mod.rs`
// so the seeded builder, its worked cases, the fixture generator, and the on-disk
// support export all stay byte-aligned.

/// A per-user desktop install owning the running app on a self-managed updater.
fn desktop_per_user_running_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:desktop-per-user:0001".to_owned(),
        surface_label: "About / install-profile card for a per-user desktop install".to_owned(),
        install_id_ref: "install:desktop:0001".to_owned(),
        install_mode: M5DeploymentMode::Desktop,
        install_scope: M5InstallScope::PerUser,
        channel_ref: "channel:stable".to_owned(),
        updater_owner: M5UpdaterOwner::SelfManaged,
        state_root_ref: "state_root:desktop:0001".to_owned(),
        build_ref: "build:stable:2026.7.0".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        rollback_target: M5RollbackTargetState::CheckpointAvailable,
        rollback_target_ref: Some("build:stable:2026.6.3".to_owned()),
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::GeneralAvailability,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:local-device".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Live,
        rollout_rollback_available: true,
        degraded: None,
    }
}

/// A portable install whose durable state root is currently unavailable (narrows).
fn portable_offline_narrowed_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:portable-offline:0002".to_owned(),
        surface_label: "About / install-profile card for a portable offline install".to_owned(),
        install_id_ref: "install:portable:0002".to_owned(),
        install_mode: M5DeploymentMode::Portable,
        install_scope: M5InstallScope::Portable,
        channel_ref: "channel:preview".to_owned(),
        updater_owner: M5UpdaterOwner::OfflineMirror,
        state_root_ref: "state_root:portable:0002".to_owned(),
        build_ref: "build:preview:2026.7.0-rc1".to_owned(),
        truth_mode: M5DeploymentTruthMode::CachedOffline,
        rollback_target: M5RollbackTargetState::PriorBuildRetained,
        rollback_target_ref: Some("build:preview:2026.6.0".to_owned()),
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::GeneralAvailability,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:portable-media".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::CachedOffline,
        rollout_rollback_available: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::StateRootUnavailable,
            degraded_label:
                "The portable drive holding this install's durable state root is not mounted; the card names the expected root and offers a re-attach route"
                    .to_owned(),
        }),
    }
}

/// A side-by-side sheet copying state once from a stable sibling, isolation kept.
fn side_by_side_isolated_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:side-by-side-copy:0003".to_owned(),
        surface_label: "Side-by-side import sheet copying state once from stable".to_owned(),
        install_id_ref: "install:preview:0003".to_owned(),
        install_mode: M5DeploymentMode::Desktop,
        install_scope: M5InstallScope::PerUser,
        channel_ref: "channel:preview".to_owned(),
        updater_owner: M5UpdaterOwner::SelfManaged,
        state_root_ref: "state_root:preview:0003".to_owned(),
        build_ref: "build:preview:2026.7.0-rc1".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        rollback_target: M5RollbackTargetState::CheckpointAvailable,
        rollback_target_ref: Some("checkpoint:pre-import:0003".to_owned()),
        sibling_install_ref: Some("install:stable:0001".to_owned()),
        state_sharing: M5StateSharingModel::OneTimeCopy,
        import_choice: M5ImportChoice::OneTimeCopy,
        handler_capture: false,
        moves_state_across_channel: true,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::EarlyAdopter,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:local-device".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Live,
        rollout_rollback_available: true,
        degraded: None,
    }
}

/// A side-by-side sheet sharing state read-only from a sibling (isolation not
/// preserved, but explicitly disclosed, not assumed).
fn side_by_side_shared_readonly_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:side-by-side-shared:0004".to_owned(),
        surface_label: "Side-by-side import sheet linking read-only shared state".to_owned(),
        install_id_ref: "install:preview:0004".to_owned(),
        install_mode: M5DeploymentMode::Desktop,
        install_scope: M5InstallScope::PerUser,
        channel_ref: "channel:preview".to_owned(),
        updater_owner: M5UpdaterOwner::SelfManaged,
        state_root_ref: "state_root:preview:0004".to_owned(),
        build_ref: "build:preview:2026.7.0-rc2".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        rollback_target: M5RollbackTargetState::PriorBuildRetained,
        rollback_target_ref: Some("checkpoint:pre-link:0004".to_owned()),
        sibling_install_ref: Some("install:stable:0001".to_owned()),
        state_sharing: M5StateSharingModel::SharedReadOnly,
        import_choice: M5ImportChoice::LinkShared,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::EarlyAdopter,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:local-device".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Live,
        rollout_rollback_available: true,
        degraded: None,
    }
}

/// A managed canary ring holding promotion on an admin fleet console (narrows).
fn managed_canary_rollout_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:managed-canary:0005".to_owned(),
        surface_label: "Admin fleet console rollout-ring row for a held canary ring".to_owned(),
        install_id_ref: "install:managed:0005".to_owned(),
        install_mode: M5DeploymentMode::Managed,
        install_scope: M5InstallScope::PerMachine,
        channel_ref: "channel:managed-canary".to_owned(),
        updater_owner: M5UpdaterOwner::ManagedAdmin,
        state_root_ref: "state_root:managed:0005".to_owned(),
        build_ref: "build:canary:2026.7.1-canary".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        rollback_target: M5RollbackTargetState::CheckpointAvailable,
        rollback_target_ref: Some("build:broad:2026.7.0".to_owned()),
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: true,
        rollout_ring: M5RolloutRing::Canary,
        promotion_state: M5PromotionState::Held,
        ring_owner_ref: "ring_owner:fleet-canary:0005".to_owned(),
        platform_scope_ref: "platform_scope:win-x64-fleet:0005".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Live,
        rollout_rollback_available: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::RolloutPaused,
            degraded_label:
                "Promotion for this canary ring is held pending a gate; the row names the ring owner, platform scope, and keeps a rollback path to the broad build"
                    .to_owned(),
        }),
    }
}

/// A managed broad ring promoted on the update center.
fn managed_broad_promoted_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:managed-broad:0006".to_owned(),
        surface_label: "Update center rollout-ring row for a promoted broad ring".to_owned(),
        install_id_ref: "install:managed:0006".to_owned(),
        install_mode: M5DeploymentMode::Managed,
        install_scope: M5InstallScope::PerMachine,
        channel_ref: "channel:managed-broad".to_owned(),
        updater_owner: M5UpdaterOwner::ManagedAdmin,
        state_root_ref: "state_root:managed:0006".to_owned(),
        build_ref: "build:broad:2026.7.0".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        rollback_target: M5RollbackTargetState::PriorBuildRetained,
        rollback_target_ref: Some("build:broad:2026.6.4".to_owned()),
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: true,
        rollout_ring: M5RolloutRing::Broad,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:fleet-broad:0006".to_owned(),
        platform_scope_ref: "platform_scope:win-x64-fleet:0006".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Live,
        rollout_rollback_available: true,
        degraded: None,
    }
}

/// A self-hosted diagnostics view whose rollback target is not yet established
/// (running-app owner disclosed only up to an unknown rollback; narrows).
fn diagnostics_unknown_rollback_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:self-hosted-diagnostics:0007".to_owned(),
        surface_label: "Diagnostics deployment pane for a self-hosted install".to_owned(),
        install_id_ref: "install:self-hosted:0007".to_owned(),
        install_mode: M5DeploymentMode::SelfHosted,
        install_scope: M5InstallScope::PerMachine,
        channel_ref: "channel:self-hosted-stable".to_owned(),
        updater_owner: M5UpdaterOwner::SelfManaged,
        state_root_ref: "state_root:self-hosted:0007".to_owned(),
        build_ref: "build:self-hosted:2026.7.0".to_owned(),
        truth_mode: M5DeploymentTruthMode::Mirrored,
        rollback_target: M5RollbackTargetState::Unknown,
        rollback_target_ref: None,
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::GeneralAvailability,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:self-hosted-node".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Mirrored,
        rollout_rollback_available: false,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            degraded_label:
                "The rollback target for this self-hosted node has not yet been established; diagnostics names it as unknown rather than imply a safe revert exists"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructed from an imported deployment snapshot.
fn support_replay_input() -> M5DeploymentProfileInput {
    M5DeploymentProfileInput {
        deployment_id: "deployment:support-replay:0008".to_owned(),
        surface_label: "Support / export replay reconstructing a deployment snapshot".to_owned(),
        install_id_ref: "install:snapshot:0008".to_owned(),
        install_mode: M5DeploymentMode::Desktop,
        install_scope: M5InstallScope::PerUser,
        channel_ref: "channel:stable".to_owned(),
        updater_owner: M5UpdaterOwner::SelfManaged,
        state_root_ref: "state_root:snapshot:0008".to_owned(),
        build_ref: "build:stable:2026.6.3".to_owned(),
        truth_mode: M5DeploymentTruthMode::Imported,
        rollback_target: M5RollbackTargetState::PriorBuildRetained,
        rollback_target_ref: Some("build:stable:2026.6.2".to_owned()),
        sibling_install_ref: None,
        state_sharing: M5StateSharingModel::Isolated,
        import_choice: M5ImportChoice::Skip,
        handler_capture: false,
        moves_state_across_channel: false,
        managed_rollout: false,
        rollout_ring: M5RolloutRing::GeneralAvailability,
        promotion_state: M5PromotionState::Promoted,
        ring_owner_ref: "ring_owner:self".to_owned(),
        platform_scope_ref: "platform_scope:local-device".to_owned(),
        evidence_freshness: M5DeploymentTruthMode::Imported,
        rollout_rollback_available: true,
        degraded: None,
    }
}

fn case(input: M5DeploymentProfileInput) -> M5DeploymentProfileCase {
    M5DeploymentProfileCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5DeploymentProfileSurfaceRow> {
    let base_source_refs = vec![
        M5_DEPLOYMENT_PROFILE_SCHEMA_REF.to_owned(),
        M5_DEPLOYMENT_PROFILE_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5DeploymentProfileExportField::ALL.to_vec();

    vec![
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::AboutInstallCard,
            owner_role: "Install-profile guild".to_owned(),
            scope_summary: "About-page install-profile card naming mode, scope, channel, updater owner, roots, and rollback"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerUser, M5InstallScope::Portable],
            truth_modes: vec![M5DeploymentTruthMode::Live, M5DeploymentTruthMode::CachedOffline],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::StateRootUnavailable,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["about_page".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_profiles: vec![
                case(desktop_per_user_running_input()),
                case(portable_offline_narrowed_input()),
            ],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::UpdateCenter,
            owner_role: "Update-center guild".to_owned(),
            scope_summary: "Update-center rollout-ring row naming channel, ring, promotion state, and rollback path"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerMachine],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::RolloutPaused,
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            ],
            consumer_surfaces: vec!["update_center".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_profiles: vec![case(managed_broad_promoted_input())],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::AdminFleetConsole,
            owner_role: "Fleet-rollout guild".to_owned(),
            scope_summary: "Admin fleet console preserving ring owner, platform scope, and promotion evidence"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerMachine],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::RolloutPaused,
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            ],
            consumer_surfaces: vec!["admin_console".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_profiles: vec![case(managed_canary_rollout_input())],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::SideBySideReview,
            owner_role: "Side-by-side handoff guild".to_owned(),
            scope_summary: "Side-by-side import sheet naming shared-vs-isolated state and preserving a checkpoint before moves"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerUser],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
                M5DeploymentDowngradeTrigger::StateRootUnavailable,
            ],
            consumer_surfaces: vec!["side_by_side_review".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_profiles: vec![
                case(side_by_side_isolated_input()),
                case(side_by_side_shared_readonly_input()),
            ],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::DiagnosticsDeployment,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics deployment pane keeping the rollback target explicit even when unknown"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerMachine],
            truth_modes: vec![M5DeploymentTruthMode::Mirrored],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
                M5DeploymentDowngradeTrigger::ResidualVendorDependency,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_profiles: vec![case(diagnostics_unknown_rollback_input())],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
        M5DeploymentProfileSurfaceRow {
            surface_family: M5DeploymentSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing deployment truth from an imported snapshot"
                .to_owned(),
            install_scopes: vec![M5InstallScope::PerUser],
            truth_modes: vec![M5DeploymentTruthMode::Imported],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_profiles: vec![case(support_replay_input())],
            hides_install_ownership: false,
            assumes_hidden_state_sharing: false,
            flattens_rollout_identity: false,
            loses_rollback_target: false,
        },
    ]
}

fn seeded_governance_review() -> M5DeploymentProfileGovernanceReview {
    M5DeploymentProfileGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        deployment_identity_preserved_across_surfaces: true,
        install_ownership_and_rollback_never_hidden: true,
        state_sharing_explicit_before_handoff: true,
        rollout_ring_identity_preserved: true,
        support_export_reconstructs_deployment: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5DeploymentProfileConsumerProjection {
    M5DeploymentProfileConsumerProjection {
        deployment_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        side_by_side_reads_single_state_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5DeploymentProfileReleasePosture {
    M5DeploymentProfileReleasePosture {
        release_packet_ref: M5_DEPLOYMENT_PROFILE_ARTIFACT_REF.to_owned(),
        deployment_audit_ref: M5_DEPLOYMENT_PROFILE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 deployment-profile primitive packet. This is
/// the one source of truth shared by the tests, the fixture generator, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_deployment_profile_packet() -> M5DeploymentProfilePrimitivePacket {
    M5DeploymentProfilePrimitivePacket::new(M5DeploymentProfilePrimitivePacketInput {
        packet_id: "m5-deployment-profile-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Deployment-Profile Primitive: Install-Profile Card, Side-by-Side Import Sheet, and Rollout-Ring Row"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5DeploymentProfileVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_DEPLOYMENT_PROFILE_SCHEMA_REF.to_owned(),
            M5_DEPLOYMENT_PROFILE_DOC_REF.to_owned(),
            M5_DEPLOYMENT_PROFILE_COMPONENT_MATRIX_REF.to_owned(),
            M5_DEPLOYMENT_PROFILE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "install_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
