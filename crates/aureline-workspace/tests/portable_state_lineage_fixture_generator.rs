//! Fixture generator helper.
//!
//! Only runs when `PORTABLE_STATE_LINEAGE_GEN_FIXTURES=1` is set in the
//! environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/portable_state_lineage/` so the replay gate has a
//! deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_portable_state_inspection_hooks,
    profiles::ArtifactPortabilityLabel,
    profiles::PortableArtifactClass,
    project_portable_state_lineage_with_hooks,
    state_packages::{
        DisplayAdjustmentClass, ExclusionSubstituteClass, ExportMode, LinkedProfileArtifactRef,
        MachineLocalExclusion, MachineLocalExclusionReason, NoRerunGuardrail, PaneRestorePosture,
        PersistenceClassification, PlaceholderAction, PlaceholderCard, PlaceholderReason,
        PortableStateAlphaPackage, PortableStateAlphaRecordKind, PortableStateClassRecord,
        PortableStateRestoreProvenance, RedactionManifest, RedactionRuleClass,
        RememberedStateAction, RememberedStateActionKind, RestoreCandidateClass,
        SerializedStateClass, StateSchemaBinding, SurfaceRestorePosture, TopologyAdjustment,
        PANE_TREE_SCHEMA_REF, PORTABLE_PROFILE_SCHEMA_REF, PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
    },
    PortableStateInspectionHook, PortableStateInspectionHookClass,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m4/portable_state_lineage")
}

fn standard_redaction_manifest() -> RedactionManifest {
    RedactionManifest {
        manifest_id: "redaction-manifest-portable-state-lineage-0001".to_owned(),
        rules: vec![
            RedactionRuleClass::RawSecretMaterialExcluded,
            RedactionRuleClass::ApprovalTicketExcluded,
            RedactionRuleClass::DelegatedCredentialExcluded,
            RedactionRuleClass::LiveAuthorityHandleExcluded,
            RedactionRuleClass::MachineUniqueHandleExcluded,
            RedactionRuleClass::StateRootExcluded,
            RedactionRuleClass::RawPathExcluded,
            RedactionRuleClass::RawHostExcluded,
            RedactionRuleClass::RawCommandLineExcluded,
            RedactionRuleClass::RawLogExcluded,
            RedactionRuleClass::RawSourceContentExcluded,
            RedactionRuleClass::ProviderPayloadExcluded,
        ],
        machine_local_exclusions_reviewed: true,
        notes: "All required redaction rules attached.".to_owned(),
    }
}

fn standard_machine_local_exclusions() -> Vec<MachineLocalExclusion> {
    vec![
        MachineLocalExclusion {
            exclusion_id: "exclusion-live-authority-handle-0001".to_owned(),
            class_kind: SerializedStateClass::WorkspaceAuthority,
            artifact_ref: "live-authority-handle-class-0001".to_owned(),
            reason: MachineLocalExclusionReason::ContainsLiveHandle,
            substitute_class: ExclusionSubstituteClass::OpaqueRef,
            notes: "Live workspace authority handle.".to_owned(),
        },
        MachineLocalExclusion {
            exclusion_id: "exclusion-display-hints-0001".to_owned(),
            class_kind: SerializedStateClass::MachineLocalHints,
            artifact_ref: "display-hints-0001".to_owned(),
            reason: MachineLocalExclusionReason::DisplayHintBestEffortOnly,
            substitute_class: ExclusionSubstituteClass::MetadataOnly,
            notes: "Display geometry / DPI hints.".to_owned(),
        },
        MachineLocalExclusion {
            exclusion_id: "exclusion-state-root-0001".to_owned(),
            class_kind: SerializedStateClass::MachineLocalHints,
            artifact_ref: "state-root-class-0001".to_owned(),
            reason: MachineLocalExclusionReason::StateRootOnly,
            substitute_class: ExclusionSubstituteClass::Omitted,
            notes: "Concrete state root locations.".to_owned(),
        },
        MachineLocalExclusion {
            exclusion_id: "exclusion-credential-store-0001".to_owned(),
            class_kind: SerializedStateClass::WorkspaceAuthority,
            artifact_ref: "credential-store-class-0001".to_owned(),
            reason: MachineLocalExclusionReason::CredentialStoreOnly,
            substitute_class: ExclusionSubstituteClass::Omitted,
            notes: "Credential / approval state remains in platform stores.".to_owned(),
        },
    ]
}

fn standard_actions() -> Vec<RememberedStateAction> {
    vec![
        RememberedStateAction {
            action: RememberedStateActionKind::Inspect,
            enabled: true,
            target_ref: Some("portable-state-package-fixture-0001".to_owned()),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Export,
            enabled: true,
            target_ref: Some("portable-state-manifest-fixture-0001".to_owned()),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Compare,
            enabled: true,
            target_ref: Some("layout-restore-provenance-fixture-0001".to_owned()),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Clear,
            enabled: true,
            target_ref: Some("window-topology-snapshot-fixture-0001".to_owned()),
            disabled_reason: None,
        },
    ]
}

fn standard_linked_profile_artifacts() -> Vec<LinkedProfileArtifactRef> {
    vec![LinkedProfileArtifactRef {
        artifact_ref: "profile-artifact-layout-density-0001".to_owned(),
        artifact_class: PortableArtifactClass::LayoutPreset,
        portability_label: ArtifactPortabilityLabel::Portable,
        schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_owned(),
        source_ref: "portable-profile-daily-driver-0001".to_owned(),
        notes: "Layout density preset travels through the portable profile.".to_owned(),
    }]
}

fn workspace_authority_row(restore_candidate: RestoreCandidateClass) -> PortableStateClassRecord {
    PortableStateClassRecord {
        class_id: "state-class-workspace-authority-0001".to_owned(),
        class_kind: SerializedStateClass::WorkspaceAuthority,
        classification: PersistenceClassification::Shared,
        export_mode: ExportMode::ReferencedBody,
        schema_binding: StateSchemaBinding {
            schema_ref: "schemas/config/workspace_manifest.schema.json".to_owned(),
            schema_version: 1,
            artifact_refs: vec!["workspace-authority-checkpoint-fixture-0001".to_owned()],
        },
        last_written_at: "mono:1700000001".to_owned(),
        restore_candidate,
        export_allowed: true,
        clear_allowed: false,
        pane_restore_postures: vec![],
        linked_profile_artifact_refs: vec![],
        local_only_reason: None,
        notes: "Workspace authority referenced by opaque checkpoint.".to_owned(),
    }
}

fn profile_defaults_row(restore_candidate: RestoreCandidateClass) -> PortableStateClassRecord {
    PortableStateClassRecord {
        class_id: "state-class-profile-defaults-0001".to_owned(),
        class_kind: SerializedStateClass::ProfileDefaults,
        classification: PersistenceClassification::Portable,
        export_mode: ExportMode::LinkedArtifactRef,
        schema_binding: StateSchemaBinding {
            schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_owned(),
            schema_version: 1,
            artifact_refs: vec!["profile-artifact-layout-density-0001".to_owned()],
        },
        last_written_at: "mono:1700000002".to_owned(),
        restore_candidate,
        export_allowed: true,
        clear_allowed: false,
        pane_restore_postures: vec![],
        linked_profile_artifact_refs: vec!["profile-artifact-layout-density-0001".to_owned()],
        local_only_reason: None,
        notes: "Profile defaults linked as portable-profile artifact.".to_owned(),
    }
}

fn machine_local_hints_row() -> PortableStateClassRecord {
    PortableStateClassRecord {
        class_id: "state-class-machine-local-hints-0001".to_owned(),
        class_kind: SerializedStateClass::MachineLocalHints,
        classification: PersistenceClassification::MachineLocal,
        export_mode: ExportMode::MetadataOnly,
        schema_binding: StateSchemaBinding {
            schema_ref: PANE_TREE_SCHEMA_REF.to_owned(),
            schema_version: 1,
            artifact_refs: vec!["display-hints-0001".to_owned()],
        },
        last_written_at: "mono:1700000003".to_owned(),
        restore_candidate: RestoreCandidateClass::Excluded,
        export_allowed: false,
        clear_allowed: true,
        pane_restore_postures: vec![],
        linked_profile_artifact_refs: vec![],
        local_only_reason: Some(
            "Display geometry, monitor affinity, fullscreen state, and DPI hints are best-effort metadata for this machine only.".to_owned(),
        ),
        notes: "Machine-local hints listed for honesty and excluded from authority.".to_owned(),
    }
}

struct PaneSeed {
    id: &'static str,
    role: &'static str,
    class: &'static str,
    posture: SurfaceRestorePosture,
    placeholder: Option<(PlaceholderReason, &'static str, Vec<PlaceholderAction>)>,
    guardrails: Vec<NoRerunGuardrail>,
    timestamp: &'static str,
}

fn build_pane(seed: PaneSeed) -> PaneRestorePosture {
    PaneRestorePosture {
        stable_pane_id: seed.id.to_owned(),
        surface_role: seed.role.to_owned(),
        surface_class: seed.class.to_owned(),
        restore_posture: seed.posture,
        placeholder_card: seed
            .placeholder
            .map(|(reason, label, actions)| PlaceholderCard {
                reason,
                safe_actions: actions,
                evidence_retained: true,
                last_known_label: Some(label.to_owned()),
            }),
        no_rerun_guardrails: seed.guardrails,
        last_written_at: seed.timestamp.to_owned(),
    }
}

fn window_topology_row(
    restore_candidate: RestoreCandidateClass,
    panes: Vec<PaneRestorePosture>,
) -> PortableStateClassRecord {
    PortableStateClassRecord {
        class_id: "state-class-window-topology-0001".to_owned(),
        class_kind: SerializedStateClass::WindowTopology,
        classification: PersistenceClassification::Portable,
        export_mode: ExportMode::CarriedBody,
        schema_binding: StateSchemaBinding {
            schema_ref: PANE_TREE_SCHEMA_REF.to_owned(),
            schema_version: 1,
            artifact_refs: vec!["window-topology-snapshot-fixture-0001".to_owned()],
        },
        last_written_at: "mono:1700000004".to_owned(),
        restore_candidate,
        export_allowed: true,
        clear_allowed: true,
        pane_restore_postures: panes,
        linked_profile_artifact_refs: vec![],
        local_only_reason: None,
        notes: "Pane slots stay stable across restore.".to_owned(),
    }
}

fn exact_panes() -> Vec<PaneRestorePosture> {
    vec![
        build_pane(PaneSeed {
            id: "pane-editor-main-0001",
            role: "editor",
            class: "text_editor",
            posture: SurfaceRestorePosture::Live,
            placeholder: None,
            guardrails: vec![],
            timestamp: "mono:1700000010",
        }),
        build_pane(PaneSeed {
            id: "pane-terminal-0001",
            role: "terminal",
            class: "terminal_view",
            posture: SurfaceRestorePosture::ContextOnly,
            placeholder: Some((
                PlaceholderReason::NonReentrantLiveSurface,
                "deploy shell context",
                vec![
                    PlaceholderAction::OpenContextOnly,
                    PlaceholderAction::RerunExplicitly,
                    PlaceholderAction::ExportEvidence,
                ],
            )),
            guardrails: vec![
                NoRerunGuardrail::NoCommandRerun,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            timestamp: "mono:1700000011",
        }),
        build_pane(PaneSeed {
            id: "pane-preview-ext-0001",
            role: "preview",
            class: "extension_view",
            posture: SurfaceRestorePosture::PlaceholderOnly,
            placeholder: Some((
                PlaceholderReason::MissingExtension,
                "service topology preview",
                vec![
                    PlaceholderAction::InstallExtension,
                    PlaceholderAction::ExportEvidence,
                    PlaceholderAction::RemovePane,
                ],
            )),
            guardrails: vec![
                NoRerunGuardrail::NoPreviewServerRestart,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            timestamp: "mono:1700000012",
        }),
    ]
}

fn evidence_only_panes() -> Vec<PaneRestorePosture> {
    vec![
        build_pane(PaneSeed {
            id: "pane-editor-main-0001",
            role: "editor",
            class: "text_editor",
            posture: SurfaceRestorePosture::PlaceholderOnly,
            placeholder: Some((
                PlaceholderReason::PolicyBlockedPane,
                "policy-blocked source pane",
                vec![
                    PlaceholderAction::ExportEvidence,
                    PlaceholderAction::RemovePane,
                ],
            )),
            guardrails: vec![
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            timestamp: "mono:1700000020",
        }),
        build_pane(PaneSeed {
            id: "pane-terminal-0001",
            role: "terminal",
            class: "terminal_view",
            posture: SurfaceRestorePosture::PlaceholderOnly,
            placeholder: Some((
                PlaceholderReason::MissingRemoteTarget,
                "remote ssh terminal evidence",
                vec![
                    PlaceholderAction::ReconnectRemote,
                    PlaceholderAction::ExportEvidence,
                    PlaceholderAction::RemovePane,
                ],
            )),
            guardrails: vec![
                NoRerunGuardrail::NoRemoteSessionResume,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            timestamp: "mono:1700000021",
        }),
        build_pane(PaneSeed {
            id: "pane-preview-ext-0001",
            role: "preview",
            class: "extension_view",
            posture: SurfaceRestorePosture::PlaceholderOnly,
            placeholder: Some((
                PlaceholderReason::MissingExtension,
                "service topology preview",
                vec![
                    PlaceholderAction::InstallExtension,
                    PlaceholderAction::ExportEvidence,
                    PlaceholderAction::RemovePane,
                ],
            )),
            guardrails: vec![
                NoRerunGuardrail::NoPreviewServerRestart,
                NoRerunGuardrail::ExplicitUserActionRequired,
                NoRerunGuardrail::PlaceholderPreserved,
            ],
            timestamp: "mono:1700000022",
        }),
    ]
}

fn placeholder_summary_from_panes(panes: &[PaneRestorePosture]) -> Vec<PaneRestorePosture> {
    panes
        .iter()
        .filter(|pane| pane.restore_posture.requires_manual_action())
        .cloned()
        .collect()
}

fn default_topology_adjustments(pane_ids: Vec<String>) -> Vec<TopologyAdjustment> {
    vec![TopologyAdjustment {
        adjustment_id: "topology-adjustment-primary-display-0001".to_owned(),
        adjustment_class: DisplayAdjustmentClass::MovedToPrimaryDisplay,
        display_topology_changed: true,
        affected_pane_ids: pane_ids,
        visible_bounds_verified: true,
        restore_fidelity_after_adjustment: RestoreCandidateClass::CompatibleRestore,
        notes: "Saved external display absent; moved to visible primary-display bounds.".to_owned(),
    }]
}

fn base_package(
    package_id: &str,
    workspace_authority_restore: RestoreCandidateClass,
    window_topology_restore: RestoreCandidateClass,
    profile_defaults_restore: RestoreCandidateClass,
    panes: Vec<PaneRestorePosture>,
) -> PortableStateAlphaPackage {
    let placeholder_summary = placeholder_summary_from_panes(&panes);
    let pane_ids: Vec<String> = panes
        .iter()
        .map(|pane| pane.stable_pane_id.clone())
        .collect();
    PortableStateAlphaPackage {
        record_kind: PortableStateAlphaRecordKind::PortableStateAlphaPackage,
        schema_version: PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
        package_id: package_id.to_owned(),
        manifest_id: format!("manifest-{package_id}"),
        workspace_ref: "workspace-rust-service-0001".to_owned(),
        created_at: "mono:1700000000".to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        state_classes: vec![
            workspace_authority_row(workspace_authority_restore),
            window_topology_row(window_topology_restore, panes.clone()),
            profile_defaults_row(profile_defaults_restore),
            machine_local_hints_row(),
        ],
        linked_profile_artifacts: standard_linked_profile_artifacts(),
        redaction_manifest: standard_redaction_manifest(),
        machine_local_exclusions: standard_machine_local_exclusions(),
        restore_provenance: PortableStateRestoreProvenance {
            source_snapshot_refs: vec!["window-topology-snapshot-fixture-0001".to_owned()],
            restore_provenance_refs: vec!["layout-restore-provenance-fixture-0001".to_owned()],
            topology_adjustments: default_topology_adjustments(pane_ids),
            placeholder_summary,
            notes: "Restore provenance summary.".to_owned(),
        },
        actions: standard_actions(),
        notes: "Portable-state lineage fixture.".to_owned(),
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    package: &'a PortableStateAlphaPackage,
    inspection_hooks: &'a Vec<PortableStateInspectionHook>,
    expected: &'a aureline_workspace::PortableStateLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    package: PortableStateAlphaPackage,
    inspection_hooks: Vec<PortableStateInspectionHook>,
) {
    let record =
        project_portable_state_lineage_with_hooks(posture_id, &package, inspection_hooks.clone());
    let envelope = FixtureEnvelope {
        posture_id,
        package: &package,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("PORTABLE_STATE_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Exact: every required class restores exactly, no panes carry placeholders.
    let exact_pkg = base_package(
        "portable-state-package-exact-0001",
        RestoreCandidateClass::ExactRestore,
        RestoreCandidateClass::ExactRestore,
        RestoreCandidateClass::ExactRestore,
        vec![
            build_pane(PaneSeed {
                id: "pane-editor-main-0001",
                role: "editor",
                class: "text_editor",
                posture: SurfaceRestorePosture::Live,
                placeholder: None,
                guardrails: vec![],
                timestamp: "mono:1700000030",
            }),
            build_pane(PaneSeed {
                id: "pane-terminal-0001",
                role: "terminal",
                class: "terminal_view",
                posture: SurfaceRestorePosture::ContextOnly,
                placeholder: Some((
                    PlaceholderReason::NonReentrantLiveSurface,
                    "deploy shell context",
                    vec![
                        PlaceholderAction::OpenContextOnly,
                        PlaceholderAction::ExportEvidence,
                    ],
                )),
                guardrails: vec![
                    NoRerunGuardrail::NoCommandRerun,
                    NoRerunGuardrail::ExplicitUserActionRequired,
                    NoRerunGuardrail::PlaceholderPreserved,
                ],
                timestamp: "mono:1700000031",
            }),
            build_pane(PaneSeed {
                id: "pane-preview-ext-0001",
                role: "preview",
                class: "extension_view",
                posture: SurfaceRestorePosture::PlaceholderOnly,
                placeholder: Some((
                    PlaceholderReason::MissingExtension,
                    "service topology preview",
                    vec![
                        PlaceholderAction::InstallExtension,
                        PlaceholderAction::ExportEvidence,
                        PlaceholderAction::RemovePane,
                    ],
                )),
                guardrails: vec![
                    NoRerunGuardrail::NoPreviewServerRestart,
                    NoRerunGuardrail::ExplicitUserActionRequired,
                    NoRerunGuardrail::PlaceholderPreserved,
                ],
                timestamp: "mono:1700000032",
            }),
        ],
    );
    write_fixture(
        "exact_restore_stable",
        "posture:exact_restore",
        exact_pkg,
        default_portable_state_inspection_hooks(),
    );

    // Compatible: workspace authority + profile defaults are compatible (DPI re-bucketing).
    let compatible_pkg = base_package(
        "portable-state-package-compatible-0001",
        RestoreCandidateClass::CompatibleRestore,
        RestoreCandidateClass::CompatibleRestore,
        RestoreCandidateClass::CompatibleRestore,
        exact_panes(),
    );
    write_fixture(
        "compatible_restore_stable",
        "posture:compatible_restore",
        compatible_pkg,
        default_portable_state_inspection_hooks(),
    );

    // Layout-only: workspace + topology restore as layout only (no live state).
    let layout_pkg = base_package(
        "portable-state-package-layout-only-0001",
        RestoreCandidateClass::LayoutOnly,
        RestoreCandidateClass::LayoutOnly,
        RestoreCandidateClass::LayoutOnly,
        vec![
            build_pane(PaneSeed {
                id: "pane-editor-main-0001",
                role: "editor",
                class: "text_editor",
                posture: SurfaceRestorePosture::PlaceholderOnly,
                placeholder: Some((
                    PlaceholderReason::DisplayTopologyAdjusted,
                    "primary editor (layout placeholder)",
                    vec![
                        PlaceholderAction::OpenContextOnly,
                        PlaceholderAction::RemovePane,
                    ],
                )),
                guardrails: vec![
                    NoRerunGuardrail::ExplicitUserActionRequired,
                    NoRerunGuardrail::PlaceholderPreserved,
                ],
                timestamp: "mono:1700000040",
            }),
            build_pane(PaneSeed {
                id: "pane-terminal-0001",
                role: "terminal",
                class: "terminal_view",
                posture: SurfaceRestorePosture::ContextOnly,
                placeholder: Some((
                    PlaceholderReason::NonReentrantLiveSurface,
                    "previous terminal context",
                    vec![
                        PlaceholderAction::OpenContextOnly,
                        PlaceholderAction::ExportEvidence,
                    ],
                )),
                guardrails: vec![
                    NoRerunGuardrail::NoCommandRerun,
                    NoRerunGuardrail::ExplicitUserActionRequired,
                    NoRerunGuardrail::PlaceholderPreserved,
                ],
                timestamp: "mono:1700000041",
            }),
        ],
    );
    write_fixture(
        "layout_only_stable",
        "posture:layout_only",
        layout_pkg,
        default_portable_state_inspection_hooks(),
    );

    // Evidence-only: every pane is placeholder-only with retained evidence.
    let evidence_pkg = base_package(
        "portable-state-package-evidence-only-0001",
        RestoreCandidateClass::Excluded,
        RestoreCandidateClass::Excluded,
        RestoreCandidateClass::Excluded,
        evidence_only_panes(),
    );
    write_fixture(
        "evidence_only_stable",
        "posture:evidence_only",
        evidence_pkg,
        default_portable_state_inspection_hooks(),
    );

    // Narrowed: compare-before-apply hook unavailable on an otherwise-Stable
    // compatible package.
    let narrowed_pkg = base_package(
        "portable-state-package-narrowed-0001",
        RestoreCandidateClass::CompatibleRestore,
        RestoreCandidateClass::CompatibleRestore,
        RestoreCandidateClass::CompatibleRestore,
        exact_panes(),
    );
    let mut narrowed_hooks = default_portable_state_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == PortableStateInspectionHookClass::CompareBeforeApply {
            hook.available = false;
            hook.disclosure = "Compare-before-apply unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_hook_narrowed",
        "posture:missing_compare_hook",
        narrowed_pkg,
        narrowed_hooks,
    );
}
