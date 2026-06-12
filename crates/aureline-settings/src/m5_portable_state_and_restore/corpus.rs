//! Deterministic portable-state export/import and restore corpus.
//!
//! The corpus pins one exact-restore baseline and four downgrade drills — a
//! stale package, a lossy cross-platform mapping, a redacted export, and an
//! unsupported-client import with missing dependencies — plus one compatible
//! cross-version restore. Export, import, support, and docs/help surfaces replay
//! the same evidence so a change to the model, the gate, or the fixtures is
//! caught against frozen records.

use super::model::{
    ExclusionReason, M5PortableStateRestoreCertification, M5PortableStateRestoreInput,
    MigrationLabel, MissingDependencyKind, MissingDependencyPlaceholder, PortabilityDisposition,
    PortableArtifactClass, PortablePackageClassRow, PortableRestoreClaim, RestoreProvenanceCard,
    SurfaceClass, SurfaceTruthRow,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-12T08:00:00Z";

/// One deterministic scenario in the portable-state restore corpus.
#[derive(Debug, Clone)]
pub struct PortableStateRestoreScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: PortableRestoreClaim,
    /// Expected weakest migration label across the package's cards.
    pub expected_fidelity_ceiling: MigrationLabel,
    record: M5PortableStateRestoreCertification,
}

impl PortableStateRestoreScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> M5PortableStateRestoreCertification {
        self.record.clone()
    }
}

struct ScenarioSpec {
    scenario_id: &'static str,
    summary: &'static str,
}

/// Returns the deterministic corpus for the portable-state restore contract.
pub fn portable_state_restore_corpus() -> Vec<PortableStateRestoreScenario> {
    [
        ScenarioSpec {
            scenario_id: "exact_local_restore",
            summary: "A same-machine, same-schema restore reapplies every M5 artifact class exactly with rollback checkpoints.",
        },
        ScenarioSpec {
            scenario_id: "cross_version_compatible",
            summary: "A package produced on an older schema restores under a compatible migration and is labeled compatible, not exact.",
        },
        ScenarioSpec {
            scenario_id: "stale_package_drill",
            summary: "A stale package restores layout-only because some values no longer map; unmappable values are preserved in a sidecar.",
        },
        ScenarioSpec {
            scenario_id: "lossy_mapping_drill",
            summary: "A cross-platform import cannot map every value and recovers the affected content as editable drafts.",
        },
        ScenarioSpec {
            scenario_id: "redacted_export_drill",
            summary: "A redacted export carries only evidence references; the restore is labeled evidence-only rather than implying live content.",
        },
        ScenarioSpec {
            scenario_id: "unsupported_client_drill",
            summary: "An import on an unsupported client surfaces missing-extension, missing-remote-target, and unsupported-client placeholders instead of dropping the affected surfaces.",
        },
    ]
    .into_iter()
    .map(build_scenario)
    .collect()
}

fn build_scenario(spec: ScenarioSpec) -> PortableStateRestoreScenario {
    let record = M5PortableStateRestoreCertification::build(M5PortableStateRestoreInput {
        record_id: format!("m5_portable_state_restore:{id}", id = spec.scenario_id),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: spec.summary.to_owned(),
        package_classes: package_classes(),
        restore_cards: restore_cards(spec.scenario_id),
        surface_truth: surface_truth(),
    })
    .expect("scenario builds");

    PortableStateRestoreScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id.replace('_', "-")),
        expected_claim_class: record.fidelity_qualification.claim_class,
        expected_fidelity_ceiling: record.fidelity_qualification.effective_fidelity_ceiling,
        record,
    }
}

/// The stable package-class disposition table shared by every scenario.
fn package_classes() -> Vec<PortablePackageClassRow> {
    vec![
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::SelectedSettings,
            disposition: PortabilityDisposition::Portable,
            content_ref: "aureline://package/selected-settings-body".to_owned(),
            exclusion_reason: None,
            rationale: "Selected scalar and structured settings are carried in full.".to_owned(),
        },
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::Profiles,
            disposition: PortabilityDisposition::Portable,
            content_ref: "aureline://package/profile-definitions".to_owned(),
            exclusion_reason: None,
            rationale: "Profile definitions and overlays round-trip through the package."
                .to_owned(),
        },
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::Manifests,
            disposition: PortabilityDisposition::Portable,
            content_ref: "aureline://package/workflow-manifests".to_owned(),
            exclusion_reason: None,
            rationale: "Workflow and bundle manifests are carried as documents.".to_owned(),
        },
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::BundleSelections,
            disposition: PortabilityDisposition::RestoreOnly,
            content_ref: "aureline://package/bundle-selection-refs".to_owned(),
            exclusion_reason: None,
            rationale: "Bundle selections are re-resolved on import; binaries stay machine-local."
                .to_owned(),
        },
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::DocsPacks,
            disposition: PortabilityDisposition::Portable,
            content_ref: "aureline://package/docs-packs".to_owned(),
            exclusion_reason: None,
            rationale: "Docs packs are carried as portable content.".to_owned(),
        },
        PortablePackageClassRow {
            artifact_class: PortableArtifactClass::EvidenceReferences,
            disposition: PortabilityDisposition::Redacted,
            content_ref: "aureline://package/evidence-reference-pointers".to_owned(),
            exclusion_reason: Some(ExclusionReason::SecretMaterial),
            rationale: "Evidence is carried as references; secret-bearing bodies are stripped."
                .to_owned(),
        },
    ]
}

fn restore_cards(scenario_id: &str) -> Vec<RestoreProvenanceCard> {
    match scenario_id {
        "exact_local_restore" => vec![
            card(
                "exact-settings",
                PortableArtifactClass::SelectedSettings,
                MigrationLabel::Exact,
                "settings:v3",
                "settings:v3",
                true,
                None,
                Vec::new(),
            ),
            card(
                "exact-profiles",
                PortableArtifactClass::Profiles,
                MigrationLabel::Exact,
                "profile:v2",
                "profile:v2",
                true,
                None,
                Vec::new(),
            ),
        ],
        "cross_version_compatible" => vec![card(
            "compatible-settings",
            PortableArtifactClass::SelectedSettings,
            MigrationLabel::Compatible,
            "settings:v2",
            "settings:v3",
            true,
            None,
            Vec::new(),
        )],
        "stale_package_drill" => vec![card(
            "stale-manifests",
            PortableArtifactClass::Manifests,
            MigrationLabel::LayoutOnly,
            "manifest:v1",
            "manifest:v3",
            true,
            Some("aureline://sidecar/stale-manifest-unmappable-values"),
            Vec::new(),
        )],
        "lossy_mapping_drill" => vec![card(
            "lossy-profiles",
            PortableArtifactClass::Profiles,
            MigrationLabel::RecoveredDrafts,
            "profile:v2",
            "profile:v3",
            true,
            Some("aureline://sidecar/cross-platform-unmappable-values"),
            Vec::new(),
        )],
        "redacted_export_drill" => vec![card(
            "redacted-evidence",
            PortableArtifactClass::EvidenceReferences,
            MigrationLabel::EvidenceOnly,
            "evidence:v1",
            "evidence:v1",
            true,
            None,
            Vec::new(),
        )],
        "unsupported_client_drill" => vec![card(
            "unsupported-bundles",
            PortableArtifactClass::BundleSelections,
            MigrationLabel::LayoutOnly,
            "bundle:v2",
            "bundle:v2",
            true,
            None,
            vec![
                placeholder(
                    MissingDependencyKind::MissingExtension,
                    PortableArtifactClass::BundleSelections,
                    "aureline://placeholder/missing-extension-rust-analyzer",
                    "Install the referenced extension to restore this bundle.",
                ),
                placeholder(
                    MissingDependencyKind::MissingRemoteTarget,
                    PortableArtifactClass::Manifests,
                    "aureline://placeholder/missing-remote-origin",
                    "Reconnect the remote origin to restore the referenced manifest.",
                ),
                placeholder(
                    MissingDependencyKind::UnsupportedClient,
                    PortableArtifactClass::DocsPacks,
                    "aureline://placeholder/unsupported-client-docs-pack",
                    "Open this package in a client that supports the docs-pack format.",
                ),
            ],
        )],
        other => panic!("unknown scenario id {other:?}"),
    }
}

#[allow(clippy::too_many_arguments)]
fn card(
    suffix: &str,
    artifact_class: PortableArtifactClass,
    migration_label: MigrationLabel,
    source_schema_version: &str,
    target_schema_version: &str,
    overwrites_local_state: bool,
    unmappable_sidecar_ref: Option<&str>,
    placeholders: Vec<MissingDependencyPlaceholder>,
) -> RestoreProvenanceCard {
    RestoreProvenanceCard {
        card_id: format!("restore-card:{suffix}"),
        source_package_ref: "aureline://package/m5-portable-state-export".to_owned(),
        artifact_class,
        migration_label,
        source_schema_version: source_schema_version.to_owned(),
        target_schema_version: target_schema_version.to_owned(),
        integrity_ref: format!("aureline://integrity/{suffix}"),
        rollback_checkpoint_ref: if overwrites_local_state {
            "aureline://snapshot/local-rollback-before-import".to_owned()
        } else {
            String::new()
        },
        unmappable_sidecar_ref: unmappable_sidecar_ref.map(str::to_owned),
        placeholders,
        previewable_before_apply: true,
        overwrites_local_state,
    }
}

fn placeholder(
    kind: MissingDependencyKind,
    affected_artifact_class: PortableArtifactClass,
    placeholder_ref: &str,
    recovery_hint: &str,
) -> MissingDependencyPlaceholder {
    MissingDependencyPlaceholder {
        kind,
        affected_artifact_class,
        placeholder_ref: placeholder_ref.to_owned(),
        visible_in_layout: true,
        silently_dropped: false,
        recovery_hint: recovery_hint.to_owned(),
    }
}

fn surface_truth() -> Vec<SurfaceTruthRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface_class| SurfaceTruthRow {
            surface_class,
            consumes_shared_record: true,
            shows_disposition: true,
            shows_migration_label: true,
            shows_placeholders: true,
            shows_rollback_checkpoint: true,
        })
        .collect()
}
