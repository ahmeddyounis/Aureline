//! Deterministic portable-state export/import review corpus.
//!
//! The corpus pins six review sheets: a clean export, an exact import, a redacted
//! export (every redaction technique), a lossy import (recovered drafts), a
//! foreign-machine import (path/host redaction and an untrusted signature), and a
//! stale-schema import (compatible migration across a schema-version mismatch).
//! Export, import, diagnostics, and support surfaces replay the same evidence so
//! a change to the model, the gate, or the fixtures is caught against frozen
//! records.

use crate::m5_portable_state_and_restore::model::{
    ExclusionReason, MigrationLabel, MissingDependencyKind, PortableArtifactClass,
};

use super::model::{
    ChangeCounts, ChecksumState, CompareSummary, DataClassLabel, HostProvenanceClass,
    M5PortableStateReviewInput, M5PortableStateReviewSheet, ProducerProvenance,
    RedactionManifestEntry, RedactionTechnique, ReviewClassRow, ReviewConsumerSurface,
    ReviewDirection, ReviewReadiness, ReviewSurfaceRow, SignatureState,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-16T08:00:00Z";

/// One deterministic scenario in the portable-state review corpus.
#[derive(Debug, Clone)]
pub struct PortableStateReviewScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Review direction.
    pub direction: ReviewDirection,
    /// Expected derived readiness class.
    pub expected_readiness: ReviewReadiness,
    record: M5PortableStateReviewSheet,
}

impl PortableStateReviewScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> M5PortableStateReviewSheet {
        self.record.clone()
    }
}

/// Returns the deterministic corpus for the portable-state review contract.
pub fn portable_state_review_corpus() -> Vec<PortableStateReviewScenario> {
    vec![
        scenario(
            "export_clean_review",
            ReviewDirection::Export,
            "An export review lists every selected class with its data-class label, redaction status, machine-local exclusions, estimated size, integrity state, and producer build before the package is written.",
            clean_export_rows(),
            clean_export_manifest(),
            None,
            same_machine_provenance("settings-package:v3", "settings-package:v3"),
        ),
        scenario(
            "import_exact_review",
            ReviewDirection::Import,
            "An import review compares an exact same-schema package against current state, shows no material pane or surface change, and is ready to restore.",
            clean_export_rows(),
            clean_export_manifest(),
            Some(exact_compare()),
            same_machine_provenance("settings-package:v3", "settings-package:v3"),
        ),
        scenario(
            "redacted_export_review",
            ReviewDirection::Export,
            "A redacted export records every redaction technique — secret omission, handle omission, path redaction, and host redaction — so what is stripped is visible, not silently dropped.",
            redacted_export_rows(),
            redacted_export_manifest(),
            None,
            same_machine_provenance("settings-package:v3", "settings-package:v3"),
        ),
        scenario(
            "lossy_import_review",
            ReviewDirection::Import,
            "A lossy import compare shows added, removed, and changed panes and surfaces; the fidelity ceiling is recovered-drafts, so the import requires review before restore.",
            clean_export_rows(),
            clean_export_manifest(),
            Some(lossy_compare()),
            same_machine_provenance("settings-package:v3", "settings-package:v3"),
        ),
        scenario(
            "foreign_machine_import_review",
            ReviewDirection::Import,
            "A foreign-machine import surfaces path and host redaction, a machine-local trust anchor held back, a missing remote target, and an untrusted signature, so it requires review before restore.",
            foreign_machine_rows(),
            foreign_machine_manifest(),
            Some(foreign_machine_compare()),
            ProducerProvenance {
                host_class: HostProvenanceClass::ForeignMachine,
                platform: "linux-x86_64".to_owned(),
                ..same_machine_provenance("settings-package:v3", "settings-package:v3")
            },
        ),
        scenario(
            "stale_schema_import_review",
            ReviewDirection::Import,
            "A stale-schema import compares a package written under an older schema; the comparison is labeled compatible and the schema-version mismatch requires review before restore.",
            clean_export_rows(),
            clean_export_manifest(),
            Some(stale_schema_compare()),
            same_machine_provenance("settings-package:v1", "settings-package:v3"),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn scenario(
    scenario_id: &'static str,
    direction: ReviewDirection,
    summary: &str,
    class_rows: Vec<ReviewClassRow>,
    redaction_manifest: Vec<RedactionManifestEntry>,
    compare: Option<CompareSummary>,
    provenance: ProducerProvenance,
) -> PortableStateReviewScenario {
    let record = M5PortableStateReviewSheet::build(M5PortableStateReviewInput {
        record_id: format!("m5_portable_state_review:{scenario_id}"),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: summary.to_owned(),
        direction,
        package_ref: "aureline://package/m5-portable-state-export".to_owned(),
        provenance,
        class_rows,
        redaction_manifest,
        compare,
        surfaces: surfaces(),
    })
    .expect("scenario builds");

    PortableStateReviewScenario {
        scenario_id,
        fixture_filename: format!("{}.json", scenario_id.replace('_', "-")),
        direction,
        expected_readiness: record.qualification.readiness,
        record,
    }
}

fn same_machine_provenance(package_schema: &str, target_schema: &str) -> ProducerProvenance {
    ProducerProvenance {
        product_build_label: "Aureline 0.0.0".to_owned(),
        build_commit_short: "abc1234".to_owned(),
        build_channel: "stable".to_owned(),
        build_dirty: false,
        package_schema_version: package_schema.to_owned(),
        target_schema_version: target_schema.to_owned(),
        host_class: HostProvenanceClass::SameMachine,
        platform: "macos-aarch64".to_owned(),
    }
}

/// The clean class table: portable, shared, local-only, and two redacted classes.
fn clean_export_rows() -> Vec<ReviewClassRow> {
    vec![
        row(
            PortableArtifactClass::SelectedSettings,
            DataClassLabel::Portable,
            None,
            4096,
            ChecksumState::Verified,
            SignatureState::Verified,
            "selected-settings-body",
            "Selected scalar and structured settings are carried in full.",
        ),
        row(
            PortableArtifactClass::Profiles,
            DataClassLabel::Shared,
            None,
            2048,
            ChecksumState::Verified,
            SignatureState::Verified,
            "profile-definitions",
            "Profiles are cleared for cross-user/fleet sharing and carried in full.",
        ),
        row(
            PortableArtifactClass::Manifests,
            DataClassLabel::Portable,
            None,
            1024,
            ChecksumState::Verified,
            SignatureState::Verified,
            "workflow-manifests",
            "Workflow and bundle manifests are carried as documents.",
        ),
        row(
            PortableArtifactClass::BundleSelections,
            DataClassLabel::LocalOnly,
            Some(ExclusionReason::VolatileMachineState),
            512,
            ChecksumState::Unavailable,
            SignatureState::Unavailable,
            "bundle-selection-refs",
            "Resolved bundle binaries stay machine-local and are re-resolved on import.",
        ),
        row(
            PortableArtifactClass::DocsPacks,
            DataClassLabel::Redacted,
            Some(ExclusionReason::VolatileMachineState),
            256,
            ChecksumState::Present,
            SignatureState::Present,
            "docs-packs",
            "Docs packs cross with local file paths redacted.",
        ),
        row(
            PortableArtifactClass::EvidenceReferences,
            DataClassLabel::Redacted,
            Some(ExclusionReason::SecretMaterial),
            128,
            ChecksumState::Present,
            SignatureState::Present,
            "evidence-reference-pointers",
            "Evidence crosses as references; secret-bearing bodies are stripped.",
        ),
    ]
}

fn clean_export_manifest() -> Vec<RedactionManifestEntry> {
    vec![
        manifest(
            PortableArtifactClass::DocsPacks,
            RedactionTechnique::PathRedaction,
            ExclusionReason::VolatileMachineState,
            3,
            "Local file-system paths inside docs packs are rewritten to portable anchors.",
        ),
        manifest(
            PortableArtifactClass::EvidenceReferences,
            RedactionTechnique::SecretOmission,
            ExclusionReason::SecretMaterial,
            2,
            "Secret-bearing evidence bodies are omitted; only reference pointers cross.",
        ),
    ]
}

/// The redacted export table: every redaction technique exercised, plus a
/// machine-local trust anchor held back.
fn redacted_export_rows() -> Vec<ReviewClassRow> {
    vec![
        row(
            PortableArtifactClass::SelectedSettings,
            DataClassLabel::Portable,
            None,
            4096,
            ChecksumState::Verified,
            SignatureState::Verified,
            "selected-settings-body",
            "Selected scalar and structured settings are carried in full.",
        ),
        row(
            PortableArtifactClass::Profiles,
            DataClassLabel::Redacted,
            Some(ExclusionReason::VolatileMachineState),
            2048,
            ChecksumState::Present,
            SignatureState::Present,
            "profile-definitions",
            "Profiles cross with embedded host identities redacted.",
        ),
        row(
            PortableArtifactClass::Manifests,
            DataClassLabel::Redacted,
            Some(ExclusionReason::VolatileMachineState),
            1024,
            ChecksumState::Present,
            SignatureState::Present,
            "workflow-manifests",
            "Manifests cross with local file paths redacted.",
        ),
        row(
            PortableArtifactClass::BundleSelections,
            DataClassLabel::MachineLocal,
            Some(ExclusionReason::LiveAuthorityHandle),
            512,
            ChecksumState::Unavailable,
            SignatureState::Unavailable,
            "bundle-selection-refs",
            "Bundle live handles are machine-local and never serialized.",
        ),
        row(
            PortableArtifactClass::DocsPacks,
            DataClassLabel::Portable,
            None,
            256,
            ChecksumState::Verified,
            SignatureState::Verified,
            "docs-packs",
            "Docs packs without local references are carried in full.",
        ),
        row(
            PortableArtifactClass::EvidenceReferences,
            DataClassLabel::Redacted,
            Some(ExclusionReason::SecretMaterial),
            128,
            ChecksumState::Present,
            SignatureState::Present,
            "evidence-reference-pointers",
            "Evidence crosses as references; secret-bearing bodies are stripped.",
        ),
    ]
}

fn redacted_export_manifest() -> Vec<RedactionManifestEntry> {
    vec![
        manifest(
            PortableArtifactClass::Profiles,
            RedactionTechnique::HostRedaction,
            ExclusionReason::VolatileMachineState,
            4,
            "Host names embedded in profile overlays are stripped before sharing.",
        ),
        manifest(
            PortableArtifactClass::Manifests,
            RedactionTechnique::PathRedaction,
            ExclusionReason::VolatileMachineState,
            6,
            "Local file-system paths inside manifests are rewritten to portable anchors.",
        ),
        manifest(
            PortableArtifactClass::BundleSelections,
            RedactionTechnique::HandleOmission,
            ExclusionReason::LiveAuthorityHandle,
            2,
            "Live bundle authority handles are omitted entirely.",
        ),
        manifest(
            PortableArtifactClass::EvidenceReferences,
            RedactionTechnique::SecretOmission,
            ExclusionReason::SecretMaterial,
            3,
            "Secret-bearing evidence bodies are omitted; only reference pointers cross.",
        ),
    ]
}

/// The foreign-machine table: a machine-unique trust anchor held back.
fn foreign_machine_rows() -> Vec<ReviewClassRow> {
    let mut rows = clean_export_rows();
    for row in &mut rows {
        if row.artifact_class == PortableArtifactClass::BundleSelections {
            row.data_class = DataClassLabel::MachineLocal;
            row.exclusion_reason = Some(ExclusionReason::MachineUniqueTrustAnchor);
            row.rationale =
                "A machine-unique trust anchor cannot be transplanted and stays machine-local."
                    .to_owned();
        }
        // The foreign signer is not trusted on this machine.
        if row.artifact_class == PortableArtifactClass::SelectedSettings {
            row.signature_state = SignatureState::Untrusted;
        }
    }
    rows
}

fn foreign_machine_manifest() -> Vec<RedactionManifestEntry> {
    vec![
        manifest(
            PortableArtifactClass::DocsPacks,
            RedactionTechnique::PathRedaction,
            ExclusionReason::VolatileMachineState,
            3,
            "Local file-system paths inside docs packs are rewritten to portable anchors.",
        ),
        manifest(
            PortableArtifactClass::EvidenceReferences,
            RedactionTechnique::SecretOmission,
            ExclusionReason::SecretMaterial,
            2,
            "Secret-bearing evidence bodies are omitted; only reference pointers cross.",
        ),
        manifest(
            PortableArtifactClass::BundleSelections,
            RedactionTechnique::HandleOmission,
            ExclusionReason::MachineUniqueTrustAnchor,
            1,
            "The machine-unique trust anchor is withheld and noted, not silently dropped.",
        ),
    ]
}

fn exact_compare() -> CompareSummary {
    CompareSummary {
        pane_delta: ChangeCounts::ZERO,
        surface_delta: ChangeCounts::ZERO,
        missing_dependency_classes: Vec::new(),
        excluded_secret_handle_count: 1,
        excluded_exclusion_reasons: vec![ExclusionReason::SecretMaterial],
        path_redaction_count: 3,
        host_redaction_count: 0,
        fidelity_ceiling: MigrationLabel::Exact,
    }
}

fn lossy_compare() -> CompareSummary {
    CompareSummary {
        pane_delta: ChangeCounts {
            added: 1,
            removed: 2,
            changed: 3,
        },
        surface_delta: ChangeCounts {
            added: 1,
            removed: 0,
            changed: 1,
        },
        missing_dependency_classes: Vec::new(),
        excluded_secret_handle_count: 1,
        excluded_exclusion_reasons: vec![ExclusionReason::SecretMaterial],
        path_redaction_count: 3,
        host_redaction_count: 0,
        fidelity_ceiling: MigrationLabel::RecoveredDrafts,
    }
}

fn foreign_machine_compare() -> CompareSummary {
    CompareSummary {
        pane_delta: ChangeCounts {
            added: 1,
            removed: 1,
            changed: 0,
        },
        surface_delta: ChangeCounts {
            added: 0,
            removed: 0,
            changed: 1,
        },
        missing_dependency_classes: vec![MissingDependencyKind::MissingRemoteTarget],
        excluded_secret_handle_count: 1,
        excluded_exclusion_reasons: vec![ExclusionReason::MachineUniqueTrustAnchor],
        path_redaction_count: 5,
        host_redaction_count: 2,
        fidelity_ceiling: MigrationLabel::LayoutOnly,
    }
}

fn stale_schema_compare() -> CompareSummary {
    CompareSummary {
        pane_delta: ChangeCounts {
            added: 0,
            removed: 0,
            changed: 2,
        },
        surface_delta: ChangeCounts::ZERO,
        missing_dependency_classes: Vec::new(),
        excluded_secret_handle_count: 1,
        excluded_exclusion_reasons: vec![ExclusionReason::SecretMaterial],
        path_redaction_count: 3,
        host_redaction_count: 0,
        fidelity_ceiling: MigrationLabel::Compatible,
    }
}

fn surfaces() -> Vec<ReviewSurfaceRow> {
    ReviewConsumerSurface::REQUIRED
        .into_iter()
        .map(|surface| ReviewSurfaceRow {
            surface,
            consumes_shared_record: true,
            shows_data_class_labels: true,
            shows_redaction_manifest: true,
            shows_machine_local_exclusions: true,
            shows_provenance: true,
            shows_compare: true,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    artifact_class: PortableArtifactClass,
    data_class: DataClassLabel,
    exclusion_reason: Option<ExclusionReason>,
    estimated_size_bytes: u64,
    checksum_state: ChecksumState,
    signature_state: SignatureState,
    content_suffix: &str,
    rationale: &str,
) -> ReviewClassRow {
    ReviewClassRow {
        artifact_class,
        data_class,
        exclusion_reason,
        estimated_size_bytes,
        checksum_state,
        signature_state,
        content_ref: format!("aureline://package/{content_suffix}"),
        visible_in_review: true,
        rationale: rationale.to_owned(),
    }
}

fn manifest(
    artifact_class: PortableArtifactClass,
    technique: RedactionTechnique,
    reason: ExclusionReason,
    redacted_field_count: u32,
    detail: &str,
) -> RedactionManifestEntry {
    RedactionManifestEntry {
        artifact_class,
        technique,
        reason,
        redacted_field_count,
        detail: detail.to_owned(),
    }
}
