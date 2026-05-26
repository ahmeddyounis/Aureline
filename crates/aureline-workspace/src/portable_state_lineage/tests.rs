//! Unit tests for the portable-state lineage projection.

use super::*;
use crate::profiles::{ArtifactPortabilityLabel, PortableArtifactClass};
use crate::state_packages::{
    DisplayAdjustmentClass, ExclusionSubstituteClass, ExportMode, LinkedProfileArtifactRef,
    MachineLocalExclusion, MachineLocalExclusionReason, PaneRestorePosture,
    PersistenceClassification, PlaceholderAction, PlaceholderCard, PlaceholderReason,
    PortableStateAlphaPackage, PortableStateAlphaRecordKind, PortableStateClassRecord,
    PortableStateRestoreProvenance, RedactionManifest, RedactionRuleClass, RememberedStateAction,
    RememberedStateActionKind, RestoreCandidateClass, SerializedStateClass, StateSchemaBinding,
    SurfaceRestorePosture, TopologyAdjustment, PANE_TREE_SCHEMA_REF, PORTABLE_PROFILE_SCHEMA_REF,
    PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
};

fn stable_package() -> PortableStateAlphaPackage {
    PortableStateAlphaPackage {
        record_kind: PortableStateAlphaRecordKind::PortableStateAlphaPackage,
        schema_version: PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
        package_id: "package-stable-0001".to_owned(),
        manifest_id: "manifest-stable-0001".to_owned(),
        workspace_ref: "workspace-stable-0001".to_owned(),
        created_at: "mono:1700000000".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        state_classes: vec![
            PortableStateClassRecord {
                class_id: "class-workspace-authority".to_owned(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                classification: PersistenceClassification::Shared,
                export_mode: ExportMode::ReferencedBody,
                schema_binding: StateSchemaBinding {
                    schema_ref: "schemas/config/workspace_manifest.schema.json".to_owned(),
                    schema_version: 1,
                    artifact_refs: vec!["workspace-authority-checkpoint-0001".to_owned()],
                },
                last_written_at: "mono:1700000001".to_owned(),
                restore_candidate: RestoreCandidateClass::CompatibleRestore,
                export_allowed: true,
                clear_allowed: false,
                pane_restore_postures: vec![],
                linked_profile_artifact_refs: vec![],
                local_only_reason: None,
                notes: "workspace authority via opaque ref".to_owned(),
            },
            PortableStateClassRecord {
                class_id: "class-window-topology".to_owned(),
                class_kind: SerializedStateClass::WindowTopology,
                classification: PersistenceClassification::Portable,
                export_mode: ExportMode::CarriedBody,
                schema_binding: StateSchemaBinding {
                    schema_ref: PANE_TREE_SCHEMA_REF.to_owned(),
                    schema_version: 1,
                    artifact_refs: vec!["window-topology-snapshot-0001".to_owned()],
                },
                last_written_at: "mono:1700000002".to_owned(),
                restore_candidate: RestoreCandidateClass::CompatibleRestore,
                export_allowed: true,
                clear_allowed: true,
                pane_restore_postures: vec![
                    PaneRestorePosture {
                        stable_pane_id: "pane-editor-0001".to_owned(),
                        surface_role: "editor".to_owned(),
                        surface_class: "text_editor".to_owned(),
                        restore_posture: SurfaceRestorePosture::Live,
                        placeholder_card: None,
                        no_rerun_guardrails: vec![],
                        last_written_at: "mono:1700000003".to_owned(),
                    },
                    PaneRestorePosture {
                        stable_pane_id: "pane-terminal-0001".to_owned(),
                        surface_role: "terminal".to_owned(),
                        surface_class: "terminal_view".to_owned(),
                        restore_posture: SurfaceRestorePosture::ContextOnly,
                        placeholder_card: Some(PlaceholderCard {
                            reason: PlaceholderReason::NonReentrantLiveSurface,
                            safe_actions: vec![
                                PlaceholderAction::OpenContextOnly,
                                PlaceholderAction::RerunExplicitly,
                            ],
                            evidence_retained: true,
                            last_known_label: Some("deploy shell".to_owned()),
                        }),
                        no_rerun_guardrails: vec![
                            NoRerunGuardrail::NoCommandRerun,
                            NoRerunGuardrail::ExplicitUserActionRequired,
                            NoRerunGuardrail::PlaceholderPreserved,
                        ],
                        last_written_at: "mono:1700000004".to_owned(),
                    },
                    PaneRestorePosture {
                        stable_pane_id: "pane-preview-ext-0001".to_owned(),
                        surface_role: "preview".to_owned(),
                        surface_class: "extension_view".to_owned(),
                        restore_posture: SurfaceRestorePosture::PlaceholderOnly,
                        placeholder_card: Some(PlaceholderCard {
                            reason: PlaceholderReason::MissingExtension,
                            safe_actions: vec![
                                PlaceholderAction::InstallExtension,
                                PlaceholderAction::ExportEvidence,
                            ],
                            evidence_retained: true,
                            last_known_label: Some("service topology preview".to_owned()),
                        }),
                        no_rerun_guardrails: vec![
                            NoRerunGuardrail::NoPreviewServerRestart,
                            NoRerunGuardrail::ExplicitUserActionRequired,
                            NoRerunGuardrail::PlaceholderPreserved,
                        ],
                        last_written_at: "mono:1700000005".to_owned(),
                    },
                ],
                linked_profile_artifact_refs: vec![],
                local_only_reason: None,
                notes: "pane tree".to_owned(),
            },
            PortableStateClassRecord {
                class_id: "class-profile-defaults".to_owned(),
                class_kind: SerializedStateClass::ProfileDefaults,
                classification: PersistenceClassification::Portable,
                export_mode: ExportMode::LinkedArtifactRef,
                schema_binding: StateSchemaBinding {
                    schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_owned(),
                    schema_version: 1,
                    artifact_refs: vec!["profile-artifact-density-0001".to_owned()],
                },
                last_written_at: "mono:1700000006".to_owned(),
                restore_candidate: RestoreCandidateClass::CompatibleRestore,
                export_allowed: true,
                clear_allowed: false,
                pane_restore_postures: vec![],
                linked_profile_artifact_refs: vec!["profile-artifact-density-0001".to_owned()],
                local_only_reason: None,
                notes: "profile defaults via linked artifact".to_owned(),
            },
            PortableStateClassRecord {
                class_id: "class-machine-local-hints".to_owned(),
                class_kind: SerializedStateClass::MachineLocalHints,
                classification: PersistenceClassification::MachineLocal,
                export_mode: ExportMode::MetadataOnly,
                schema_binding: StateSchemaBinding {
                    schema_ref: PANE_TREE_SCHEMA_REF.to_owned(),
                    schema_version: 1,
                    artifact_refs: vec!["display-hints-0001".to_owned()],
                },
                last_written_at: "mono:1700000007".to_owned(),
                restore_candidate: RestoreCandidateClass::Excluded,
                export_allowed: false,
                clear_allowed: true,
                pane_restore_postures: vec![],
                linked_profile_artifact_refs: vec![],
                local_only_reason: Some(
                    "Display geometry and DPI hints are local to this machine.".to_owned(),
                ),
                notes: "machine-local hints listed for honesty".to_owned(),
            },
        ],
        linked_profile_artifacts: vec![LinkedProfileArtifactRef {
            artifact_ref: "profile-artifact-density-0001".to_owned(),
            artifact_class: PortableArtifactClass::LayoutPreset,
            portability_label: ArtifactPortabilityLabel::Portable,
            schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_owned(),
            source_ref: "portable-profile-daily-driver-0001".to_owned(),
            notes: "density preset travels through the portable profile".to_owned(),
        }],
        redaction_manifest: RedactionManifest {
            manifest_id: "redaction-manifest-0001".to_owned(),
            rules: vec![
                RedactionRuleClass::RawSecretMaterialExcluded,
                RedactionRuleClass::ApprovalTicketExcluded,
                RedactionRuleClass::DelegatedCredentialExcluded,
                RedactionRuleClass::LiveAuthorityHandleExcluded,
                RedactionRuleClass::MachineUniqueHandleExcluded,
                RedactionRuleClass::StateRootExcluded,
                RedactionRuleClass::RawPathExcluded,
                RedactionRuleClass::RawHostExcluded,
            ],
            machine_local_exclusions_reviewed: true,
            notes: "all required rules present".to_owned(),
        },
        machine_local_exclusions: vec![
            MachineLocalExclusion {
                exclusion_id: "exclusion-live-handle-0001".to_owned(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                artifact_ref: "live-authority-handle-class-0001".to_owned(),
                reason: MachineLocalExclusionReason::ContainsLiveHandle,
                substitute_class: ExclusionSubstituteClass::OpaqueRef,
                notes: "live workspace authority handle".to_owned(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-display-hints-0001".to_owned(),
                class_kind: SerializedStateClass::MachineLocalHints,
                artifact_ref: "display-hints-0001".to_owned(),
                reason: MachineLocalExclusionReason::DisplayHintBestEffortOnly,
                substitute_class: ExclusionSubstituteClass::MetadataOnly,
                notes: "display hints".to_owned(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-state-root-0001".to_owned(),
                class_kind: SerializedStateClass::MachineLocalHints,
                artifact_ref: "state-root-class-0001".to_owned(),
                reason: MachineLocalExclusionReason::StateRootOnly,
                substitute_class: ExclusionSubstituteClass::Omitted,
                notes: "state root locations".to_owned(),
            },
            MachineLocalExclusion {
                exclusion_id: "exclusion-credential-store-0001".to_owned(),
                class_kind: SerializedStateClass::WorkspaceAuthority,
                artifact_ref: "credential-store-class-0001".to_owned(),
                reason: MachineLocalExclusionReason::CredentialStoreOnly,
                substitute_class: ExclusionSubstituteClass::Omitted,
                notes: "credential store".to_owned(),
            },
        ],
        restore_provenance: PortableStateRestoreProvenance {
            source_snapshot_refs: vec!["window-topology-snapshot-0001".to_owned()],
            restore_provenance_refs: vec!["layout-restore-provenance-0001".to_owned()],
            topology_adjustments: vec![TopologyAdjustment {
                adjustment_id: "topology-adjustment-0001".to_owned(),
                adjustment_class: DisplayAdjustmentClass::MovedToPrimaryDisplay,
                display_topology_changed: true,
                affected_pane_ids: vec![
                    "pane-editor-0001".to_owned(),
                    "pane-terminal-0001".to_owned(),
                    "pane-preview-ext-0001".to_owned(),
                ],
                visible_bounds_verified: true,
                restore_fidelity_after_adjustment: RestoreCandidateClass::CompatibleRestore,
                notes: "moved to visible primary display".to_owned(),
            }],
            placeholder_summary: vec![
                PaneRestorePosture {
                    stable_pane_id: "pane-terminal-0001".to_owned(),
                    surface_role: "terminal".to_owned(),
                    surface_class: "terminal_view".to_owned(),
                    restore_posture: SurfaceRestorePosture::ContextOnly,
                    placeholder_card: Some(PlaceholderCard {
                        reason: PlaceholderReason::NonReentrantLiveSurface,
                        safe_actions: vec![PlaceholderAction::OpenContextOnly],
                        evidence_retained: true,
                        last_known_label: Some("deploy shell".to_owned()),
                    }),
                    no_rerun_guardrails: vec![
                        NoRerunGuardrail::NoCommandRerun,
                        NoRerunGuardrail::ExplicitUserActionRequired,
                        NoRerunGuardrail::PlaceholderPreserved,
                    ],
                    last_written_at: "mono:1700000008".to_owned(),
                },
                PaneRestorePosture {
                    stable_pane_id: "pane-preview-ext-0001".to_owned(),
                    surface_role: "preview".to_owned(),
                    surface_class: "extension_view".to_owned(),
                    restore_posture: SurfaceRestorePosture::PlaceholderOnly,
                    placeholder_card: Some(PlaceholderCard {
                        reason: PlaceholderReason::MissingExtension,
                        safe_actions: vec![PlaceholderAction::InstallExtension],
                        evidence_retained: true,
                        last_known_label: Some("service topology preview".to_owned()),
                    }),
                    no_rerun_guardrails: vec![
                        NoRerunGuardrail::NoPreviewServerRestart,
                        NoRerunGuardrail::ExplicitUserActionRequired,
                        NoRerunGuardrail::PlaceholderPreserved,
                    ],
                    last_written_at: "mono:1700000009".to_owned(),
                },
            ],
            notes: "placeholder summary covers terminal and preview".to_owned(),
        },
        actions: vec![
            RememberedStateAction {
                action: RememberedStateActionKind::Inspect,
                enabled: true,
                target_ref: Some("package-stable-0001".to_owned()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Export,
                enabled: true,
                target_ref: Some("manifest-stable-0001".to_owned()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Compare,
                enabled: true,
                target_ref: Some("layout-restore-provenance-0001".to_owned()),
                disabled_reason: None,
            },
            RememberedStateAction {
                action: RememberedStateActionKind::Clear,
                enabled: true,
                target_ref: Some("window-topology-snapshot-0001".to_owned()),
                disabled_reason: None,
            },
        ],
        notes: "clean stable package".to_owned(),
    }
}

#[test]
fn clean_package_projects_stable_compatible_fidelity() {
    let package = stable_package();
    let record = project_portable_state_lineage("posture.clean", &package);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, PORTABLE_STATE_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, PORTABLE_STATE_LINEAGE_SCHEMA_REF);
    assert_eq!(
        record.restore_provenance.restore_fidelity_class,
        RestoreFidelityClass::Compatible
    );
    assert!(record.state_class_separation.workspace_authority_present);
    assert!(record.state_class_separation.window_topology_present);
    assert!(record.state_class_separation.profile_defaults_present);
    assert!(record.state_class_separation.machine_local_hints_present);
    assert!(
        record
            .state_class_separation
            .machine_local_hints_classified_correctly
    );
    assert!(record.no_rerun_honesty.all_non_live_panes_no_rerun_guarded);
    assert!(record.no_rerun_honesty.remembered_state_actions_complete);
    assert!(record.exclusion_honesty.redaction_rules_complete);
    assert!(record.exclusion_honesty.paths_and_hosts_excluded);
    assert!(
        record
            .exclusion_honesty
            .machine_local_exclusion_reasons_complete
    );
    assert_eq!(record.inspection_hooks.len(), 7);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("psl:"));
}

#[test]
fn missing_redaction_rule_narrows_record() {
    let mut package = stable_package();
    package
        .redaction_manifest
        .rules
        .retain(|rule| *rule != RedactionRuleClass::LiveAuthorityHandleExcluded);

    let record = project_portable_state_lineage("posture.no_redaction", &package);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&PortableStateLineageNarrowReason::RedactionRuleMissing));
}

#[test]
fn missing_machine_local_exclusion_reason_narrows() {
    let mut package = stable_package();
    package
        .machine_local_exclusions
        .retain(|row| row.reason != MachineLocalExclusionReason::ContainsLiveHandle);

    let record = project_portable_state_lineage("posture.no_exclusion_reason", &package);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&PortableStateLineageNarrowReason::MachineLocalExclusionReasonMissing));
}

#[test]
fn topology_adjustment_without_visible_bounds_narrows() {
    let mut package = stable_package();
    package.restore_provenance.topology_adjustments[0].visible_bounds_verified = false;

    let record = project_portable_state_lineage("posture.unverified_bounds", &package);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&PortableStateLineageNarrowReason::TopologyAdjustmentUnverified));
}

#[test]
fn missing_compare_inspection_hook_narrows_record() {
    let package = stable_package();
    let mut hooks = default_portable_state_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == PortableStateInspectionHookClass::CompareBeforeApply {
            hook.available = false;
        }
    }

    let record = project_portable_state_lineage_with_hooks("posture.no_compare", &package, hooks);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&PortableStateLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn placeholder_only_pane_missing_no_rerun_guardrail_narrows() {
    let mut package = stable_package();
    let topology = package
        .state_classes
        .iter_mut()
        .find(|row| row.class_kind == SerializedStateClass::WindowTopology)
        .expect("topology row");
    let pane = topology
        .pane_restore_postures
        .iter_mut()
        .find(|pane| pane.restore_posture == SurfaceRestorePosture::PlaceholderOnly)
        .expect("placeholder pane");
    pane.no_rerun_guardrails = vec![NoRerunGuardrail::NoPreviewServerRestart];

    let record = project_portable_state_lineage("posture.no_guardrail", &package);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&PortableStateLineageNarrowReason::PlaceholderMissingNoRerunGuardrail));
}

#[test]
fn evidence_only_fidelity_when_every_pane_is_placeholder_only() {
    let mut package = stable_package();
    let topology = package
        .state_classes
        .iter_mut()
        .find(|row| row.class_kind == SerializedStateClass::WindowTopology)
        .expect("topology row");
    topology.restore_candidate = RestoreCandidateClass::Excluded;
    for pane in topology.pane_restore_postures.iter_mut() {
        pane.restore_posture = SurfaceRestorePosture::PlaceholderOnly;
        pane.placeholder_card = Some(PlaceholderCard {
            reason: PlaceholderReason::MissingExtension,
            safe_actions: vec![PlaceholderAction::InstallExtension],
            evidence_retained: true,
            last_known_label: Some(format!("legacy {}", pane.stable_pane_id)),
        });
        pane.no_rerun_guardrails = vec![
            NoRerunGuardrail::ExplicitUserActionRequired,
            NoRerunGuardrail::PlaceholderPreserved,
        ];
    }
    let workspace = package
        .state_classes
        .iter_mut()
        .find(|row| row.class_kind == SerializedStateClass::WorkspaceAuthority)
        .expect("workspace row");
    workspace.restore_candidate = RestoreCandidateClass::Excluded;
    let profile = package
        .state_classes
        .iter_mut()
        .find(|row| row.class_kind == SerializedStateClass::ProfileDefaults)
        .expect("profile row");
    profile.restore_candidate = RestoreCandidateClass::Excluded;

    let record = project_portable_state_lineage("posture.evidence_only", &package);
    assert_eq!(
        record.restore_provenance.restore_fidelity_class,
        RestoreFidelityClass::EvidenceOnly
    );
}

#[test]
fn lineage_lines_render_every_pillar() {
    let package = stable_package();
    let record = project_portable_state_lineage("posture.lines", &package);
    let lines = portable_state_lineage_lines(&record);

    assert!(lines.iter().any(|l| l.contains("Portable-state lineage")));
    assert!(lines.iter().any(|l| l.contains("State class rows:")));
    assert!(lines.iter().any(|l| l.contains("Restore provenance:")));
    assert!(lines.iter().any(|l| l.contains("Exclusion honesty:")));
    assert!(lines.iter().any(|l| l.contains("No-rerun honesty:")));
    assert!(lines.iter().any(|l| l.contains("Inspection hooks:")));
    assert!(lines.iter().any(|l| l.contains("integrity_hash=psl:")));
}

#[test]
fn record_round_trips_through_json() {
    let package = stable_package();
    let record = project_portable_state_lineage("posture.json", &package);
    let json = serde_json::to_string(&record).expect("record serializes");
    let restored: PortableStateLineageRecord =
        serde_json::from_str(&json).expect("record deserializes");
    assert_eq!(record, restored);
}
