use aureline_recovery::crash_journal::{
    FrameIntegrityState, GuidedChoiceClass, ReplayPostureClass,
};
use aureline_recovery::portable_state::{
    DestinationClass, LayoutSerializationSnapshot, PackagePurpose, PortableProfileSnapshot,
    PortableStatePackageError, PortableStatePackageInput, PortableStatePackageRecord,
    PortableStateRedactionRule, RestoreCheckpointProjection, TransferIntegrityRecord,
    TransferIntegrityRecords, RESTORE_CHECKPOINT_COMMAND_ID, RESTORE_CHECKPOINT_SNAPSHOT_CLASS,
};
use aureline_recovery::session_restore::records::{
    ProducerBuildStamp, RestoreClass, SurfaceClass, SurfaceRole, TerminalPaneRestoreMetadata,
};
use aureline_recovery::session_restore::{
    RestoreProposal, RestoreProposalArtifactRefs, RestoreProposalCounts,
    RestoreProposalDirtyBufferEntry, RestoreProposalPanePlan, RestoreProposalPlanKind,
};
use aureline_workspace::{
    ArtifactOwnerScope, ArtifactPortabilityLabel, ArtifactPrivacyClass, DisplayAdjustmentClass,
    ExclusionSubstituteClass, ExportMode, LinkedProfileArtifactRef, MachineLocalExclusion,
    MachineLocalExclusionReason, NoRerunGuardrail, NonPortableExclusionReason, PaneRestorePosture,
    PersistenceClassification, PlaceholderAction, PlaceholderCard, PlaceholderReason,
    PortableArtifactClass, PortableProfileArtifact, PortableProfileExport,
    PortableStateAlphaPackage, PortableStateAlphaRecordKind, PortableStateClassRecord,
    PortableStateRestoreProvenance, RedactionManifest, RedactionRuleClass, RememberedStateAction,
    RememberedStateActionKind, RestoreCandidateClass, SerializedStateClass, StateSchemaBinding,
    StateSourcePosture, SurfaceRestorePosture, TopologyAdjustment, PANE_TREE_SCHEMA_REF,
    PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION, PORTABLE_PROFILE_SCHEMA_REF,
    PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
};

fn producer_build() -> ProducerBuildStamp {
    ProducerBuildStamp {
        producer_name: "aureline-recovery-portable-state-test".to_string(),
        producer_version: "0.0.0".to_string(),
        producer_channel: Some("experimental".to_string()),
        producer_platform_class: Some("linux".to_string()),
        producer_instance_handle: Some("producer-instance:fixture".to_string()),
    }
}

fn linked_profile_artifact() -> LinkedProfileArtifactRef {
    LinkedProfileArtifactRef {
        artifact_ref: "profile-layout-preset:fixture".to_string(),
        artifact_class: PortableArtifactClass::LayoutPreset,
        portability_label: ArtifactPortabilityLabel::Portable,
        schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_string(),
        source_ref: "portable-profile:fixture".to_string(),
        notes: "Fixture layout preset travels through the profile export.".to_string(),
    }
}

fn workspace_layout_package() -> PortableStateAlphaPackage {
    let pane_rows = vec![
        PaneRestorePosture {
            stable_pane_id: "pane-0001".to_string(),
            surface_role: "editor".to_string(),
            surface_class: "text_editor".to_string(),
            restore_posture: SurfaceRestorePosture::Live,
            placeholder_card: None,
            no_rerun_guardrails: Vec::new(),
            last_written_at: "mono:fixture:0001".to_string(),
        },
        PaneRestorePosture {
            stable_pane_id: "pane-0002".to_string(),
            surface_role: "terminal".to_string(),
            surface_class: "terminal_view".to_string(),
            restore_posture: SurfaceRestorePosture::ContextOnly,
            placeholder_card: Some(PlaceholderCard {
                reason: PlaceholderReason::NonReentrantLiveSurface,
                safe_actions: vec![
                    PlaceholderAction::OpenContextOnly,
                    PlaceholderAction::RerunExplicitly,
                    PlaceholderAction::ExportEvidence,
                ],
                evidence_retained: true,
                last_known_label: Some("terminal metadata retained".to_string()),
            }),
            no_rerun_guardrails: vec![
                NoRerunGuardrail::NoCommandRerun,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            last_written_at: "mono:fixture:0001".to_string(),
        },
        PaneRestorePosture {
            stable_pane_id: "pane-0003".to_string(),
            surface_role: "preview".to_string(),
            surface_class: "extension_view".to_string(),
            restore_posture: SurfaceRestorePosture::PlaceholderOnly,
            placeholder_card: Some(PlaceholderCard {
                reason: PlaceholderReason::MissingExtension,
                safe_actions: vec![
                    PlaceholderAction::InstallExtension,
                    PlaceholderAction::ExportEvidence,
                    PlaceholderAction::RemovePane,
                ],
                evidence_retained: true,
                last_known_label: Some("preview extension missing".to_string()),
            }),
            no_rerun_guardrails: vec![
                NoRerunGuardrail::NoPreviewServerRestart,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            last_written_at: "mono:fixture:0001".to_string(),
        },
    ];
    let profile_link = linked_profile_artifact();
    PortableStateAlphaPackage {
        record_kind: PortableStateAlphaRecordKind::PortableStateAlphaPackage,
        schema_version: PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
        package_id: "shell-layout-package:fixture".to_string(),
        manifest_id: "portable-state-manifest:fixture".to_string(),
        workspace_ref: "workspace:fixture".to_string(),
        created_at: "mono:fixture:0002".to_string(),
        producer_ref: "producer:aureline-shell-fixture".to_string(),
        state_classes: vec![
            PortableStateClassRecord {
                class_id: "state-class-workspace-authority:fixture".to_string(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                classification: PersistenceClassification::Shared,
                export_mode: ExportMode::ReferencedBody,
                schema_binding: StateSchemaBinding {
                    schema_ref: "schemas/config/workspace_manifest.schema.json".to_string(),
                    schema_version: 1,
                    artifact_refs: vec!["workspace-authority:fixture".to_string()],
                },
                last_written_at: "mono:fixture:0001".to_string(),
                restore_candidate: RestoreCandidateClass::CompatibleRestore,
                export_allowed: true,
                clear_allowed: false,
                pane_restore_postures: Vec::new(),
                linked_profile_artifact_refs: Vec::new(),
                local_only_reason: None,
                notes: "Workspace authority remains referenced by opaque ids.".to_string(),
            },
            PortableStateClassRecord {
                class_id: "state-class-window-topology:fixture".to_string(),
                class_kind: SerializedStateClass::WindowTopology,
                classification: PersistenceClassification::Portable,
                export_mode: ExportMode::CarriedBody,
                schema_binding: StateSchemaBinding {
                    schema_ref: PANE_TREE_SCHEMA_REF.to_string(),
                    schema_version: 1,
                    artifact_refs: vec!["window-topology-snapshot:fixture".to_string()],
                },
                last_written_at: "mono:fixture:0001".to_string(),
                restore_candidate: RestoreCandidateClass::LayoutOnly,
                export_allowed: true,
                clear_allowed: true,
                pane_restore_postures: pane_rows.clone(),
                linked_profile_artifact_refs: Vec::new(),
                local_only_reason: None,
                notes: "Window topology carries stable pane ids.".to_string(),
            },
            PortableStateClassRecord {
                class_id: "state-class-profile-defaults:fixture".to_string(),
                class_kind: SerializedStateClass::ProfileDefaults,
                classification: PersistenceClassification::Portable,
                export_mode: ExportMode::LinkedArtifactRef,
                schema_binding: StateSchemaBinding {
                    schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_string(),
                    schema_version: 1,
                    artifact_refs: vec![profile_link.artifact_ref.clone()],
                },
                last_written_at: "mono:fixture:0001".to_string(),
                restore_candidate: RestoreCandidateClass::CompatibleRestore,
                export_allowed: true,
                clear_allowed: false,
                pane_restore_postures: Vec::new(),
                linked_profile_artifact_refs: vec![profile_link.artifact_ref.clone()],
                local_only_reason: None,
                notes: "Profile defaults remain linked profile refs.".to_string(),
            },
            PortableStateClassRecord {
                class_id: "state-class-machine-local-hints:fixture".to_string(),
                class_kind: SerializedStateClass::MachineLocalHints,
                classification: PersistenceClassification::MachineLocal,
                export_mode: ExportMode::MetadataOnly,
                schema_binding: StateSchemaBinding {
                    schema_ref: PANE_TREE_SCHEMA_REF.to_string(),
                    schema_version: 1,
                    artifact_refs: vec!["machine-display-hints:fixture".to_string()],
                },
                last_written_at: "mono:fixture:0001".to_string(),
                restore_candidate: RestoreCandidateClass::Excluded,
                export_allowed: false,
                clear_allowed: true,
                pane_restore_postures: Vec::new(),
                linked_profile_artifact_refs: Vec::new(),
                local_only_reason: Some(
                    "Display geometry and state roots stay machine-local.".to_string(),
                ),
                notes: "Machine-local hints are metadata only.".to_string(),
            },
        ],
        linked_profile_artifacts: vec![profile_link],
        redaction_manifest: RedactionManifest {
            manifest_id: "layout-redaction-manifest:fixture".to_string(),
            rules: vec![
                RedactionRuleClass::RawSecretMaterialExcluded,
                RedactionRuleClass::ApprovalTicketExcluded,
                RedactionRuleClass::DelegatedCredentialExcluded,
                RedactionRuleClass::LiveAuthorityHandleExcluded,
                RedactionRuleClass::MachineUniqueHandleExcluded,
                RedactionRuleClass::StateRootExcluded,
            ],
            machine_local_exclusions_reviewed: true,
            notes: "Fixture redaction floor.".to_string(),
        },
        machine_local_exclusions: vec![
            MachineLocalExclusion {
                exclusion_id: "exclusion-live-handles:fixture".to_string(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                artifact_ref: "live-authority-handles:fixture".to_string(),
                reason: MachineLocalExclusionReason::ContainsLiveHandle,
                substitute_class: ExclusionSubstituteClass::OpaqueRef,
                notes: "Live authority handles are excluded.".to_string(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-display-hints:fixture".to_string(),
                class_kind: SerializedStateClass::MachineLocalHints,
                artifact_ref: "machine-display-hints:fixture".to_string(),
                reason: MachineLocalExclusionReason::DisplayHintBestEffortOnly,
                substitute_class: ExclusionSubstituteClass::MetadataOnly,
                notes: "Display hints are best effort.".to_string(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-state-roots:fixture".to_string(),
                class_kind: SerializedStateClass::MachineLocalHints,
                artifact_ref: "state-roots:fixture".to_string(),
                reason: MachineLocalExclusionReason::StateRootOnly,
                substitute_class: ExclusionSubstituteClass::Omitted,
                notes: "Concrete state roots do not travel.".to_string(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-credentials:fixture".to_string(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                artifact_ref: "credential-store:fixture".to_string(),
                reason: MachineLocalExclusionReason::CredentialStoreOnly,
                substitute_class: ExclusionSubstituteClass::Omitted,
                notes: "Credentials remain protected.".to_string(),
            },
        ],
        restore_provenance: PortableStateRestoreProvenance {
            source_snapshot_refs: vec!["window-topology-snapshot:fixture".to_string()],
            restore_provenance_refs: vec!["restore-provenance:layout:fixture".to_string()],
            topology_adjustments: vec![TopologyAdjustment {
                adjustment_id: "display-adjustment:fixture".to_string(),
                adjustment_class: DisplayAdjustmentClass::SnappedToSafeBounds,
                display_topology_changed: true,
                affected_pane_ids: vec![
                    "pane-0001".to_string(),
                    "pane-0002".to_string(),
                    "pane-0003".to_string(),
                ],
                visible_bounds_verified: true,
                restore_fidelity_after_adjustment: RestoreCandidateClass::LayoutOnly,
                notes: "Fixture display bounds were verified.".to_string(),
            }],
            placeholder_summary: pane_rows
                .into_iter()
                .filter(|pane| pane.restore_posture != SurfaceRestorePosture::Live)
                .collect(),
            notes: "Restore provenance carries placeholder summary.".to_string(),
        },
        actions: vec![
            RememberedStateAction {
                action: RememberedStateActionKind::Inspect,
                enabled: true,
                target_ref: Some("inspect:fixture".to_string()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Export,
                enabled: true,
                target_ref: Some("export:fixture".to_string()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Compare,
                enabled: true,
                target_ref: Some("compare:fixture".to_string()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Clear,
                enabled: true,
                target_ref: Some("clear:fixture".to_string()),
                disabled_reason: None,
            },
        ],
        notes: "Fixture workspace portable-state package.".to_string(),
    }
}

fn profile_artifact(
    artifact_id: &str,
    artifact_class: PortableArtifactClass,
) -> PortableProfileArtifact {
    PortableProfileArtifact {
        artifact_id: artifact_id.to_string(),
        artifact_class,
        owner_scope: ArtifactOwnerScope::User,
        privacy_class: ArtifactPrivacyClass::MetadataSafeDefault,
        portability_label: ArtifactPortabilityLabel::Portable,
        source_posture: StateSourcePosture::LocalOnly,
        source_device_ref: Some("device:source:fixture".to_string()),
        source_ref: format!("source:{artifact_id}"),
        source_revision_ref: format!("rev:{artifact_id}:0001"),
        capability_dependencies: Vec::new(),
        exclusion_reasons: Vec::new(),
        captures_transient_selection: false,
        captures_stale_provider_cursor: false,
        captures_secret_bearing_parameters: false,
        portability_note: "Fixture artifact is portable metadata.".to_string(),
    }
}

fn portable_profile_export() -> PortableProfileExport {
    PortableProfileExport {
        schema_version: PORTABLE_PROFILE_ALPHA_SCHEMA_VERSION,
        profile_id: "profile:fixture".to_string(),
        profile_scope: "profile_defaults,user_global_preferences".to_string(),
        profile_revision_ref: "profile-revision:fixture:0001".to_string(),
        source_device_ref: "device:source:fixture".to_string(),
        artifacts: vec![
            profile_artifact(
                "profile-artifact:keymap:fixture",
                PortableArtifactClass::Keymap,
            ),
            profile_artifact(
                "profile-artifact:saved-view:fixture",
                PortableArtifactClass::SavedView,
            ),
            profile_artifact(
                "profile-artifact:setting:fixture",
                PortableArtifactClass::Setting,
            ),
        ],
        non_portable_exclusions: vec![
            NonPortableExclusionReason::SecretMaterial,
            NonPortableExclusionReason::DelegatedCredential,
        ],
    }
}

fn restore_proposal() -> RestoreProposal {
    RestoreProposal {
        record_kind: "restore_proposal_record".to_string(),
        restore_proposal_schema_version: 1,
        prior_run_abnormal: true,
        restore_class: RestoreClass::RecoveredDrafts,
        counts: RestoreProposalCounts {
            windows: 1,
            tab_groups: 1,
            tabs: 2,
            dirty_buffer_journals: 1,
            transient_tasks: 0,
            terminals: 1,
            evidence_packets: 1,
            recovery_packets: 1,
        },
        artifact_refs: RestoreProposalArtifactRefs {
            checkpoint_id: Some("session-checkpoint:fixture".to_string()),
            snapshot_id: Some("session-snapshot:fixture".to_string()),
            workspace_authority_ref: Some("workspace-authority:fixture".to_string()),
            window_id: Some("window:fixture".to_string()),
        },
        pane_plans: vec![
            RestoreProposalPanePlan {
                pane_id: "pane-0001".to_string(),
                surface_role: SurfaceRole::Editor,
                surface_class: SurfaceClass::TextEditor,
                plan_kind: RestoreProposalPlanKind::LiveSkeleton,
                title_hint: Some("src/main.rs".to_string()),
                restore_metadata: None,
                note: "Editor can reopen as a lightweight skeleton.".to_string(),
            },
            RestoreProposalPanePlan {
                pane_id: "pane-0002".to_string(),
                surface_role: SurfaceRole::Terminal,
                surface_class: SurfaceClass::TerminalView,
                plan_kind: RestoreProposalPlanKind::BlockedSideEffectful,
                title_hint: Some("zsh".to_string()),
                restore_metadata: Some(TerminalPaneRestoreMetadata {
                    restore_metadata_ref: "terminal-restore-metadata:fixture".to_string(),
                    working_directory: Some("workspace-root-ref:fixture".to_string()),
                    environment_scope_token: "workspace".to_string(),
                    shell_identity: "zsh".to_string(),
                    shell_family_token: "zsh".to_string(),
                    last_command_class_token: "build".to_string(),
                    auto_rerun_forbidden: true,
                    raw_command_body_present: false,
                    raw_environment_body_present: false,
                }),
                note: "Terminal metadata survives, but commands never rerun.".to_string(),
            },
        ],
        dirty_buffer_entries: vec![RestoreProposalDirtyBufferEntry {
            journal_entry_id: "journal-entry:fixture".to_string(),
            journal_id: "journal:fixture".to_string(),
            object_ref: "logical-document:fixture".to_string(),
            presentation_hint: Some("main.rs".to_string()),
            replay_posture: ReplayPostureClass::RestoreRequiresReview,
            frame_integrity: FrameIntegrityState::Verified,
            recommended_choice: GuidedChoiceClass::InspectOnly,
        }],
        downgrade_triggers: Vec::new(),
        auto_rerun_forbidden: true,
        notes: Some("Fixture restore proposal.".to_string()),
    }
}

fn transfer_integrity() -> TransferIntegrityRecords {
    TransferIntegrityRecords::metadata_only(
        "transfer-integrity:fixture",
        "support:transfer-integrity:fixture",
        vec![
            TransferIntegrityRecord::metadata_only(
                "transfer-action:copy:fixture",
                "copy",
                "editor",
                "plain_text",
                "audit_only_no_mutation",
                "local-clipboard:fixture",
                "local_clipboard",
                None,
                false,
                "mono:fixture:0003",
            ),
            TransferIntegrityRecord::metadata_only(
                "transfer-action:recover-terminal:fixture",
                "recover",
                "terminal",
                "metadata_only",
                "evidence_only_no_rerun",
                "terminal-session:fixture",
                "local_workspace",
                Some("local-history-checkpoint:fixture".to_string()),
                true,
                "mono:fixture:0003",
            ),
        ],
    )
}

fn fixture_package() -> PortableStatePackageRecord {
    let layout_snapshot = LayoutSerializationSnapshot::from_workspace_package(
        "layout-serialization-snapshot:fixture",
        &workspace_layout_package(),
    )
    .expect("layout projection");
    let portable_profile = PortableProfileSnapshot::from_profile_export(
        "portable-profile-export:fixture",
        &portable_profile_export(),
    )
    .expect("profile projection");
    PortableStatePackageRecord::build(PortableStatePackageInput {
        package_id: "portable-state-package:fixture".to_string(),
        manifest_id: "portable-state-manifest:fixture".to_string(),
        package_label: "Fixture portable-state package".to_string(),
        package_purpose: PackagePurpose::RestoreCompareExport,
        destination_class: DestinationClass::LocalFile,
        created_at: "mono:fixture:0004".to_string(),
        producer_build: producer_build(),
        layout_snapshot,
        portable_profile,
        session_restore_proposal: restore_proposal(),
        transfer_integrity: transfer_integrity(),
        restore_checkpoint: RestoreCheckpointProjection::new(
            "local-history-checkpoint:fixture",
            "Fixture restore checkpoint",
            "mono:fixture:0003",
        ),
        notes: None,
    })
    .expect("fixture package builds")
}

#[test]
fn pack_and_unpack_round_trips_the_package_projection() {
    let package = fixture_package();
    let bytes = package.pack().expect("package packs");
    let unpacked = PortableStatePackageRecord::unpack(&bytes).expect("package unpacks");

    assert_eq!(unpacked, package);
    assert_eq!(
        unpacked.restore_checkpoint.restore_command_id,
        RESTORE_CHECKPOINT_COMMAND_ID
    );
    assert_eq!(
        unpacked.restore_checkpoint.resulting_snapshot_class,
        RESTORE_CHECKPOINT_SNAPSHOT_CLASS
    );
}

#[test]
fn redaction_defaults_apply_and_raw_payloads_are_rejected() {
    let package = fixture_package();
    assert!(package.redaction_manifest.redaction_defaults_applied);
    assert!(!package.redaction_manifest.raw_payload_bodies_included);
    assert!(package
        .redaction_manifest
        .rules
        .contains(&PortableStateRedactionRule::RawClipboardBodyExcluded));
    assert!(package
        .redaction_manifest
        .rules
        .contains(&PortableStateRedactionRule::RawTerminalPayloadExcluded));
    assert!(package
        .transfer_integrity
        .omitted_payload_classes
        .contains(&"raw_clipboard_body".to_string()));

    let mut raw_package = package;
    raw_package.transfer_integrity.actions[0].raw_clipboard_body_present = true;
    assert!(matches!(
        raw_package.validate(),
        Err(PortableStatePackageError::RawPayloadIncluded(
            "transfer_integrity.raw_clipboard_body_present"
        ))
    ));
}

#[test]
fn restore_provenance_lineage_preserves_layout_session_and_checkpoint_refs() {
    let package = fixture_package();

    assert!(package
        .restore_provenance_lineage
        .restore_provenance_refs
        .contains(&"restore-provenance:layout:fixture".to_string()));
    assert!(package
        .restore_provenance_lineage
        .source_snapshot_refs
        .contains(&"window-topology-snapshot:fixture".to_string()));
    assert_eq!(
        package
            .restore_provenance_lineage
            .session_restore_checkpoint_ref
            .as_deref(),
        Some("session-checkpoint:fixture")
    );
    assert!(package
        .restore_provenance_lineage
        .checkpoint_refs
        .contains(&"local-history-checkpoint:fixture".to_string()));

    let mut broken = package;
    broken
        .restore_provenance_lineage
        .restore_provenance_refs
        .clear();
    assert!(matches!(
        broken.validate(),
        Err(PortableStatePackageError::LineageMissingRef {
            field: "restore_provenance_refs",
            missing_ref,
        }) if missing_ref == "restore-provenance:layout:fixture"
    ));
}

#[test]
fn package_bytes_are_stable_across_two_runs_of_the_same_fixture() {
    let first = fixture_package();
    let second = fixture_package();

    assert_eq!(
        first.pack().expect("first pack"),
        second.pack().expect("second pack")
    );
    assert_eq!(
        first.stable_digest().expect("first digest"),
        second.stable_digest().expect("second digest")
    );
}
