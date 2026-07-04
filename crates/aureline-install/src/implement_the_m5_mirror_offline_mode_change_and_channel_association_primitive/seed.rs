// Canonical seed for the M5 mirror-transition primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support
// export all stay byte-aligned.

/// Convenience constructor for one mirror/offline artifact input. Verification and
/// manifests are always reachable (AC2) and stale content is never shown as current
/// (AC1); the manifest ref is derived from the artifact ref.
#[allow(clippy::too_many_arguments)]
fn artifact(
    artifact_ref: &str,
    artifact_class: M5MirrorArtifactClass,
    source_class: M5MirrorSourceClass,
    freshness: M5DeploymentTruthMode,
    signature_state: M5MirrorSignatureState,
    mirror_reachable: bool,
    pinned_offline: bool,
    needs_refresh: bool,
) -> M5MirrorArtifactInput {
    M5MirrorArtifactInput {
        artifact_ref: artifact_ref.to_owned(),
        artifact_class,
        source_class,
        freshness,
        signature_state,
        mirror_reachable,
        pinned_offline,
        needs_refresh,
        manifest_ref: format!("{artifact_ref}:manifest"),
        verify_available: true,
        open_manifest_available: true,
        stale_not_shown_as_current: true,
    }
}

/// An online update-center switching release channels while both artifacts stay current
/// and verified.
fn update_center_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:update-center:0001".to_owned(),
        surface_label: "Update-center mirror artifacts during a release-channel switch".to_owned(),
        deployment_mode: M5DeploymentMode::Managed,
        artifacts: vec![
            artifact(
                "artifact:update-bundle",
                M5MirrorArtifactClass::Updates,
                M5MirrorSourceClass::FirstPartyMirror,
                M5DeploymentTruthMode::Live,
                M5MirrorSignatureState::Verified,
                true,
                false,
                false,
            ),
            artifact(
                "artifact:docs-pack",
                M5MirrorArtifactClass::Docs,
                M5MirrorSourceClass::VendorCdn,
                M5DeploymentTruthMode::Live,
                M5MirrorSignatureState::Verified,
                true,
                false,
                false,
            ),
        ],
        from_mode: M5DeploymentMode::Managed,
        to_mode: M5DeploymentMode::Managed,
        boundary_change: M5BoundaryChangeClass::ChannelSwitch,
        preserved_local_state_ref: "state:workspace:update-center".to_owned(),
        affected_managed_feature_refs: vec!["feature:auto-update".to_owned()],
        cache_disposition: M5CacheDisposition::ReuseValid,
        rollback_path_state: M5RollbackPathState::Available,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:release-channel:0001".to_owned(),
        handler_association_ref: "handler:aureline-updater".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: None,
    }
}

/// A mirror-manager whose self-hosted mirror is stale and must be refreshed before a
/// mirror re-attach.
fn mirror_manager_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:mirror-manager:0002".to_owned(),
        surface_label: "Mirror-manager reviewing a stale self-hosted model mirror".to_owned(),
        deployment_mode: M5DeploymentMode::SelfHosted,
        artifacts: vec![artifact(
            "artifact:model-weights",
            M5MirrorArtifactClass::Models,
            M5MirrorSourceClass::SelfHostedMirror,
            M5DeploymentTruthMode::Mirrored,
            M5MirrorSignatureState::Verified,
            true,
            false,
            true,
        )],
        from_mode: M5DeploymentMode::SelfHosted,
        to_mode: M5DeploymentMode::SelfHosted,
        boundary_change: M5BoundaryChangeClass::MirrorReattach,
        preserved_local_state_ref: "state:workspace:mirror-manager".to_owned(),
        affected_managed_feature_refs: vec!["feature:model-inference".to_owned()],
        cache_disposition: M5CacheDisposition::RebuildRequired,
        rollback_path_state: M5RollbackPathState::RequiresCheckpoint,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:mirror-source:0002".to_owned(),
        handler_association_ref: "handler:self-hosted-mirror".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::MirrorStale,
            degraded_label:
                "The self-hosted model mirror is stale relative to its live source; the row marks it needs-refresh, keeps verify and open-manifest reachable, and names the rebuild-required cache disposition before the re-attach"
                    .to_owned(),
        }),
    }
}

/// An admin console reviewing a managed-to-air-gapped disconnect where cached-offline
/// artifacts remain usable but must never read as current.
fn admin_disconnect_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:admin-disconnect:0003".to_owned(),
        surface_label: "Admin console reviewing a managed-to-air-gapped disconnect".to_owned(),
        deployment_mode: M5DeploymentMode::Managed,
        artifacts: vec![
            artifact(
                "artifact:policy-bundle",
                M5MirrorArtifactClass::PolicyBundles,
                M5MirrorSourceClass::OfflineBundle,
                M5DeploymentTruthMode::CachedOffline,
                M5MirrorSignatureState::Verified,
                true,
                false,
                false,
            ),
            artifact(
                "artifact:extension-pack",
                M5MirrorArtifactClass::Extensions,
                M5MirrorSourceClass::PeerCache,
                M5DeploymentTruthMode::CachedOffline,
                M5MirrorSignatureState::VerificationDeferred,
                true,
                true,
                false,
            ),
        ],
        from_mode: M5DeploymentMode::Managed,
        to_mode: M5DeploymentMode::AirGapped,
        boundary_change: M5BoundaryChangeClass::OnlineOfflineTransition,
        preserved_local_state_ref: "state:workspace:admin-disconnect".to_owned(),
        affected_managed_feature_refs: vec![
            "feature:policy-sync".to_owned(),
            "feature:extension-updates".to_owned(),
        ],
        cache_disposition: M5CacheDisposition::PreservePinned,
        rollback_path_state: M5RollbackPathState::RequiresCheckpoint,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:protocol-handler:0003".to_owned(),
        handler_association_ref: "handler:managed-control-plane".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "Going air-gapped serves policy and extension bundles from an offline cache only; the sheet names exactly what will stale, what remains usable offline, the preserve-pinned cache, and the checkpoint-backed rollback path"
                    .to_owned(),
        }),
    }
}

/// A diagnostics pane surfacing an artifact whose signature verification failed.
fn diagnostics_verify_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:diagnostics-verify:0004".to_owned(),
        surface_label: "Diagnostics mirror pane surfacing a failed signature verification"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        artifacts: vec![artifact(
            "artifact:extension-candidate",
            M5MirrorArtifactClass::Extensions,
            M5MirrorSourceClass::VendorCdn,
            M5DeploymentTruthMode::Live,
            M5MirrorSignatureState::SignatureMismatch,
            true,
            false,
            false,
        )],
        from_mode: M5DeploymentMode::Desktop,
        to_mode: M5DeploymentMode::Desktop,
        boundary_change: M5BoundaryChangeClass::ChannelSwitch,
        preserved_local_state_ref: "state:workspace:diagnostics-verify".to_owned(),
        affected_managed_feature_refs: Vec::new(),
        cache_disposition: M5CacheDisposition::InvalidateStale,
        rollback_path_state: M5RollbackPathState::Available,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:extension-source:0004".to_owned(),
        handler_association_ref: "handler:extension-installer".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::SignatureUnverified,
            degraded_label:
                "The candidate extension's signature failed verification; the row marks it verification-failed, invalidates the stale cache, and keeps verify and open-manifest reachable so the artifact is never installed as trusted"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructing an imported-truth state-root migration.
fn support_replay_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:support-replay:0005".to_owned(),
        surface_label: "Support / export replay reconstructing an imported state-root migration"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Managed,
        artifacts: vec![artifact(
            "artifact:docs-snapshot",
            M5MirrorArtifactClass::Docs,
            M5MirrorSourceClass::OfflineBundle,
            M5DeploymentTruthMode::Imported,
            M5MirrorSignatureState::Verified,
            true,
            false,
            false,
        )],
        from_mode: M5DeploymentMode::Managed,
        to_mode: M5DeploymentMode::Managed,
        boundary_change: M5BoundaryChangeClass::StateRootMigration,
        preserved_local_state_ref: "state:workspace:support-replay".to_owned(),
        affected_managed_feature_refs: vec!["feature:docs-sync".to_owned()],
        cache_disposition: M5CacheDisposition::RebuildRequired,
        rollback_path_state: M5RollbackPathState::Available,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:state-root:0005".to_owned(),
        handler_association_ref: "handler:state-root-owner".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: None,
    }
}

/// A docs reference framing a desktop install with a current, verified first-party
/// mirror.
fn docs_reference_input() -> M5MirrorTransitionInput {
    M5MirrorTransitionInput {
        transition_id: "transition:docs-reference:0006".to_owned(),
        surface_label: "Docs mirror reference for a desktop install with a current mirror"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        artifacts: vec![artifact(
            "artifact:docs-current",
            M5MirrorArtifactClass::Docs,
            M5MirrorSourceClass::FirstPartyMirror,
            M5DeploymentTruthMode::Live,
            M5MirrorSignatureState::Verified,
            true,
            false,
            false,
        )],
        from_mode: M5DeploymentMode::Desktop,
        to_mode: M5DeploymentMode::Desktop,
        boundary_change: M5BoundaryChangeClass::ChannelSwitch,
        preserved_local_state_ref: "state:workspace:docs-reference".to_owned(),
        affected_managed_feature_refs: Vec::new(),
        cache_disposition: M5CacheDisposition::ReuseValid,
        rollback_path_state: M5RollbackPathState::NotApplicable,
        reviewed_before_change: true,
        export_before_change_available: true,
        channel_ref: "channel:docs-source:0006".to_owned(),
        handler_association_ref: "handler:docs-viewer".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
        degraded: None,
    }
}

fn case(input: M5MirrorTransitionInput) -> M5MirrorTransitionCase {
    M5MirrorTransitionCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5MirrorTransitionSurfaceRow> {
    let base_source_refs = vec![
        M5_MIRROR_TRANSITION_SCHEMA_REF.to_owned(),
        M5_MIRROR_TRANSITION_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5MirrorTransitionExportField::ALL.to_vec();

    vec![
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::UpdateCenter,
            owner_role: "Update-center guild".to_owned(),
            scope_summary: "Update-center mirror artifacts with verify / open-manifest actions during a release-channel switch"
                .to_owned(),
            artifact_classes: vec![
                M5MirrorArtifactClass::Updates,
                M5MirrorArtifactClass::Docs,
            ],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::MirrorStale,
                M5DeploymentDowngradeTrigger::SignatureUnverified,
            ],
            consumer_surfaces: vec!["update_center".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_transitions: vec![case(update_center_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::MirrorManager,
            owner_role: "Mirror-manager guild".to_owned(),
            scope_summary: "Mirror-manager marking a stale self-hosted mirror needs-refresh before a mirror re-attach"
                .to_owned(),
            artifact_classes: vec![M5MirrorArtifactClass::Models],
            truth_modes: vec![M5DeploymentTruthMode::Mirrored],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::MirrorStale,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["mirror_manager".to_owned(), "admin_console".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_transitions: vec![case(mirror_manager_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::AdminDeploymentConsole,
            owner_role: "Deployment-admin guild".to_owned(),
            scope_summary: "Admin console reviewing a managed-to-air-gapped disconnect with cached-offline artifacts and a checkpoint rollback"
                .to_owned(),
            artifact_classes: vec![
                M5MirrorArtifactClass::PolicyBundles,
                M5MirrorArtifactClass::Extensions,
            ],
            truth_modes: vec![M5DeploymentTruthMode::CachedOffline],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
                M5DeploymentDowngradeTrigger::StateRootUnavailable,
            ],
            consumer_surfaces: vec!["admin_console".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_transitions: vec![case(admin_disconnect_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::DiagnosticsMirror,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics mirror pane surfacing a failed signature verification and invalidating the stale cache"
                .to_owned(),
            artifact_classes: vec![M5MirrorArtifactClass::Extensions],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::SignatureUnverified,
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_transitions: vec![case(diagnostics_verify_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing an imported-truth state-root migration and its rollback path"
                .to_owned(),
            artifact_classes: vec![M5MirrorArtifactClass::Docs],
            truth_modes: vec![M5DeploymentTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_transitions: vec![case(support_replay_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
        M5MirrorTransitionSurfaceRow {
            surface_family: M5MirrorSurfaceFamily::DocsMirrorReference,
            owner_role: "Docs / help guild".to_owned(),
            scope_summary: "Docs mirror reference framing a desktop install with a current, verified first-party mirror"
                .to_owned(),
            artifact_classes: vec![M5MirrorArtifactClass::Docs],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::MirrorStale,
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            ],
            consumer_surfaces: vec!["docs_reference".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs,
            example_transitions: vec![case(docs_reference_input())],
            shows_stale_as_current: false,
            hides_verification: false,
            forces_blind_switch: false,
            captures_default_handler: false,
        },
    ]
}

fn seeded_governance_review() -> M5MirrorTransitionGovernanceReview {
    M5MirrorTransitionGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        transition_identity_preserved_across_surfaces: true,
        stale_never_shown_as_current: true,
        verification_manifest_always_accessible: true,
        export_before_change_and_rollback_always_preserved: true,
        support_export_reconstructs_transition: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5MirrorTransitionConsumerProjection {
    M5MirrorTransitionConsumerProjection {
        mirror_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        review_sheet_reads_single_transition_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5MirrorTransitionReleasePosture {
    M5MirrorTransitionReleasePosture {
        release_packet_ref: M5_MIRROR_TRANSITION_ARTIFACT_REF.to_owned(),
        deployment_audit_ref: M5_MIRROR_TRANSITION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 mirror-transition primitive packet. This is the
/// one source of truth shared by the tests, the fixture generator, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_mirror_transition_packet() -> M5MirrorTransitionPrimitivePacket {
    M5MirrorTransitionPrimitivePacket::new(M5MirrorTransitionPrimitivePacketInput {
        packet_id: "m5-mirror-transition-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Mirror-Transition Primitive: Mirror/Offline Artifact Rows, Mode-Change Review Sheet, and Channel-Association Review Row"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5MirrorTransitionVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_MIRROR_TRANSITION_SCHEMA_REF.to_owned(),
            M5_MIRROR_TRANSITION_DOC_REF.to_owned(),
            M5_MIRROR_TRANSITION_COMPONENT_MATRIX_REF.to_owned(),
            M5_MIRROR_TRANSITION_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "install_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
