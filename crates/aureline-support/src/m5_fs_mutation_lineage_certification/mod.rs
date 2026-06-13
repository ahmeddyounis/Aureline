//! Canonical M5 filesystem-identity, watch/save, mutation-lineage, and
//! deferred-intent certification.
//!
//! This module composes the already-frozen state and VFS packets into one
//! certification object that downstream help, diagnostics, release-center, and
//! support-bundle consumers can quote verbatim instead of deriving parallel
//! status text.
//!
//! The certification sits above four canonical packets:
//!
//! - [`aureline_vfs::FilesystemMutationLineageMatrixPacket`]
//! - [`aureline_vfs::FilesystemTruthReviewPacket`]
//! - [`aureline_reactive_state::M5MutationLineagePacket`]
//! - [`aureline_reactive_state::StateClassRecoveryPacket`]
//!
//! Each certification row answers one publication question for one claimed M5
//! surface row: may this row still publish a full filesystem-continuity claim,
//! or must it narrow to `limited`, `stale`, or `reconcile_required` because the
//! row is generator-owned, inspect-only, provider-backed, or dependent on
//! deferred-intent review?
//!
//! The packet keeps the certification non-inheriting. A row that lacks
//! canonical identity, lacks a writable save target, runs with non-live watch
//! truth, or requires reconnect review narrows explicitly instead of borrowing a
//! greener label from notebook or local-file rows beside it.
//!
//! The same packet also proves surface reuse. Help, diagnostics,
//! support-bundle, and release-center projections all bind to this one object
//! and preserve the same row ids, published states, downgrade rules, and root
//! or connectivity posture.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use aureline_reactive_state::{
    seeded_m5_mutation_lineage_packet, seeded_state_class_recovery_packet,
    validate_m5_mutation_lineage_packet, validate_state_class_recovery_packet,
    M5MutationLineagePacket, StateClassRecoveryPacket,
};
use aureline_vfs::{
    seeded_filesystem_mutation_lineage_matrix_packet, seeded_filesystem_truth_review_packet,
    validate_filesystem_mutation_lineage_matrix, validate_filesystem_truth_review_packet,
    FilesystemMutationLineageMatrixPacket, FilesystemTruthReviewPacket, MatrixConnectivityState,
    MatrixCorruptionState, MatrixPathIdentityClass, MatrixReconciliationPosture, MatrixRootClass,
    MatrixRow, MatrixSaveFallback, MatrixSurfaceClass, MatrixUndoClass, MatrixWatchState,
};
use serde::{Deserialize, Serialize};

use crate::{
    compile_m5_mutation_lineage_support_export_envelope,
    compile_state_class_recovery_support_export_envelope,
};

/// Stable record-kind tag carried by [`M5FsMutationLineageCertificationPacket`].
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "m5_fs_mutation_lineage_certification_packet";

/// Frozen schema version for this certification packet.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/state/m5-fs-mutation-lineage-certification.schema.json";

/// Repository-relative path of the reviewer-facing document.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF: &str =
    "docs/state/m5-fs-mutation-lineage-certification.md";

/// Repository-relative path of the checked JSON artifact.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/state/m5-fs-mutation-lineage-certification.json";

/// Repository-relative path of the checked reviewer summary.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/state/m5-fs-mutation-lineage-certification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_FS_MUTATION_LINEAGE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/state/m5-fs-mutation-lineage-certification";

const FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/m5-fs-mutation-lineage-certification/manifest.yaml";
const HELP_CONSUMER_REF: &str = "crates/aureline-shell/src/help/filesystem_continuity.rs";
const DIAGNOSTICS_CONSUMER_REF: &str =
    "crates/aureline-shell/src/diagnostics/filesystem_continuity.rs";
const RELEASE_CENTER_CONSUMER_REF: &str =
    "crates/aureline-shell/src/release_center/filesystem_continuity.rs";
const SUPPORT_BUNDLE_CONSUMER_REF: &str =
    "crates/aureline-support/src/m5_fs_mutation_lineage_certification/mod.rs::support_bundle_projection";
const REQUIRED_ROW_FIELDS: &[&str] = &[
    "certification_row_id",
    "surface_row_id",
    "published_state",
    "primary_root_class",
    "connectivity_state",
    "stale_or_narrow_tokens",
    "downgrade_rule_ids",
];

/// Published certification state for one claimed M5 surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStateClass {
    /// The row keeps full filesystem identity, save, lineage, and recovery
    /// truth on its claimed path.
    Qualified,
    /// The row is real and supported, but it may not publish a broad ordinary
    /// file claim.
    Limited,
    /// The row stays visible, but its freshness or watch truth is inherently
    /// weaker than a fully live editable file.
    Stale,
    /// The row remains usable only with explicit reconnect or drift review.
    ReconcileRequired,
}

impl CertificationStateClass {
    /// Every published state in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Qualified,
        Self::Limited,
        Self::Stale,
        Self::ReconcileRequired,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Limited => "limited",
            Self::Stale => "stale",
            Self::ReconcileRequired => "reconcile_required",
        }
    }
}

/// Automatic downgrade trigger encoded by the certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeTriggerClass {
    /// The row does not own canonical filesystem identity.
    CanonicalIdentityUnavailable,
    /// The row does not own a canonical direct save target.
    CanonicalSaveTargetUnavailable,
    /// The row cannot claim fully live watch truth.
    WatchTruthDegraded,
    /// The row's mutation lineage is narrower than an ordinary exact-undo edit.
    MutationLineageScoped,
    /// The row's repair or corruption path is narrower than a normal file edit.
    RecoveryPathScoped,
    /// Deferred-intent or publish-later work must revalidate or reconcile.
    DeferredIntentNeedsReview,
    /// A downstream consumer stopped ingesting the packet by reference.
    ConsumerBindingMissing,
}

impl CertificationDowngradeTriggerClass {
    /// Every downgrade trigger in canonical order.
    pub const ALL: [Self; 7] = [
        Self::CanonicalIdentityUnavailable,
        Self::CanonicalSaveTargetUnavailable,
        Self::WatchTruthDegraded,
        Self::MutationLineageScoped,
        Self::RecoveryPathScoped,
        Self::DeferredIntentNeedsReview,
        Self::ConsumerBindingMissing,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalIdentityUnavailable => "canonical_identity_unavailable",
            Self::CanonicalSaveTargetUnavailable => "canonical_save_target_unavailable",
            Self::WatchTruthDegraded => "watch_truth_degraded",
            Self::MutationLineageScoped => "mutation_lineage_scoped",
            Self::RecoveryPathScoped => "recovery_path_scoped",
            Self::DeferredIntentNeedsReview => "deferred_intent_needs_review",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }
}

/// Stable consumer surface that ingests the certification verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurfaceClass {
    /// Help and continuity explanation surfaces.
    HelpSurface,
    /// Diagnostics export surfaces.
    DiagnosticsExport,
    /// Local-first support-bundle preview and export surfaces.
    SupportBundle,
    /// Release-center and shiproom truth surfaces.
    ReleaseCenter,
}

impl CertificationSurfaceClass {
    /// Every surface in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HelpSurface,
        Self::DiagnosticsExport,
        Self::SupportBundle,
        Self::ReleaseCenter,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpSurface => "help_surface",
            Self::DiagnosticsExport => "diagnostics_export",
            Self::SupportBundle => "support_bundle",
            Self::ReleaseCenter => "release_center",
        }
    }
}

/// One certified surface row drawn from the canonical filesystem matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCertificationRow {
    /// Stable certification row id.
    pub certification_row_id: String,
    /// Stable row id in the filesystem mutation-lineage matrix.
    pub surface_row_id: String,
    /// Surface class carried by the matrix row.
    pub surface_class: MatrixSurfaceClass,
    /// Published state for the row.
    pub published_state: CertificationStateClass,
    /// Primary root class the surface claims first.
    pub primary_root_class: MatrixRootClass,
    /// All root classes this row admits.
    pub supported_root_classes: Vec<MatrixRootClass>,
    /// Identity class preserved by the row.
    pub path_identity_class: MatrixPathIdentityClass,
    /// Watch-state posture preserved by the row.
    pub watch_state: MatrixWatchState,
    /// Save posture preserved by the row.
    pub save_fallback: MatrixSaveFallback,
    /// Undo honesty preserved by the row.
    pub undo_class: MatrixUndoClass,
    /// Corruption-routing posture preserved by the row.
    pub corruption_state: MatrixCorruptionState,
    /// Connectivity posture preserved by the row.
    pub connectivity_state: MatrixConnectivityState,
    /// Reconnect or reconcile posture preserved by the row.
    pub reconciliation_posture: MatrixReconciliationPosture,
    /// Linked truth-review scenarios quoted by the row when they exist.
    pub review_scenario_ids: Vec<String>,
    /// Linked lineage roots proving the mutation-family claim.
    pub lineage_root_ids: Vec<String>,
    /// Linked recovery families proving the recovery claim.
    pub recovery_family_ids: Vec<String>,
    /// Canonical source packet refs this row depends on.
    pub source_packet_refs: Vec<String>,
    /// Active narrow or stale tokens describing why the row is not fully qualified.
    pub stale_or_narrow_tokens: Vec<String>,
    /// Active downgrade rules explaining the publication state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary.
    pub summary: String,
}

/// One downgrade rule published by the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDowngradeRuleRow {
    /// Stable rule id.
    pub rule_id: String,
    /// Trigger that fires this rule.
    pub trigger_class: CertificationDowngradeTriggerClass,
    /// Source state before the downgrade.
    pub source_state: CertificationStateClass,
    /// Resulting state after the downgrade.
    pub downgraded_state: CertificationStateClass,
    /// User-visible effect.
    pub required_effect: String,
    /// Review-safe rationale.
    pub rationale: String,
    /// Supporting refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer binding proving the same certification object is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceBinding {
    /// Surface that ingests this certification.
    pub surface: CertificationSurfaceClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet id ingested by the consumer.
    pub ingested_packet_id: String,
    /// Number of rows the consumer exposes by reference.
    pub certification_row_count: usize,
    /// Fields the consumer preserves verbatim.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on non-qualified rows.
    pub narrow_on_narrowed_rows: bool,
    /// True when limited, stale, and reconcile-required labels remain explicit.
    pub explicit_state_labels_required: bool,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Top-level certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Checked JSON artifact ref.
    pub artifact_ref: String,
    /// Checked Markdown summary ref.
    pub summary_ref: String,
    /// Authoritative spec and design refs.
    pub source_spec_refs: Vec<String>,
    /// Existing packets and contracts this certification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Root classes covered by the packet.
    pub root_classes: Vec<MatrixRootClass>,
    /// Connectivity states covered by the packet.
    pub connectivity_states: Vec<MatrixConnectivityState>,
    /// Mutation-lineage roots quoted by the packet.
    pub lineage_root_ids: Vec<String>,
    /// Recovery families quoted by the packet.
    pub recovery_family_ids: Vec<String>,
    /// Canonical surface rows under certification.
    pub certification_rows: Vec<SurfaceCertificationRow>,
    /// Automatic downgrade rules encoded by the packet.
    pub downgrade_rules: Vec<CertificationDowngradeRuleRow>,
    /// Consumer bindings proving downstream reuse.
    pub surface_bindings: Vec<CertificationSurfaceBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5FsMutationLineageCertificationPacket {
    /// Validates source refs, row coverage, downgrade automation, and consumer reuse.
    pub fn validate(&self) -> Vec<M5FsMutationLineageCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "record_kind".to_owned(),
                message: "unexpected record_kind".to_owned(),
            });
        }
        if self.schema_version != M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "schema_version".to_owned(),
                message: "unexpected schema_version".to_owned(),
            });
        }
        if self.schema_ref != M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "schema_ref".to_owned(),
                message: "packet must quote the canonical schema ref".to_owned(),
            });
        }
        if self.doc_ref != M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "doc_ref".to_owned(),
                message: "packet must quote the canonical reviewer doc".to_owned(),
            });
        }
        if self.artifact_ref != M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "artifact_ref".to_owned(),
                message: "packet must quote the checked JSON artifact".to_owned(),
            });
        }
        if self.summary_ref != M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF {
            violations.push(M5FsMutationLineageCertificationViolation {
                path: "summary_ref".to_owned(),
                message: "packet must quote the checked Markdown summary".to_owned(),
            });
        }

        for required in [
            MatrixRootClass::LocalFilesystem,
            MatrixRootClass::RemoteAgent,
            MatrixRootClass::ContainerMount,
            MatrixRootClass::ArchivePackaged,
            MatrixRootClass::GeneratedManaged,
            MatrixRootClass::VirtualProviderBacked,
            MatrixRootClass::ManagedOfflineBundle,
        ] {
            if !self.root_classes.contains(&required) {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: "root_classes".to_owned(),
                    message: format!("missing root class {}", required.as_str()),
                });
            }
        }

        let matrix = seeded_filesystem_mutation_lineage_matrix_packet();
        for row in &matrix.rows {
            let Some(cert_row) = self
                .certification_rows
                .iter()
                .find(|candidate| candidate.surface_row_id == row.row_id)
            else {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: "certification_rows".to_owned(),
                    message: format!("missing certification row for {}", row.row_id),
                });
                continue;
            };

            let base = format!("certification_rows.{}", cert_row.certification_row_id);
            for (field, value) in [
                ("surface_row_id", cert_row.surface_row_id.as_str()),
                ("summary", cert_row.summary.as_str()),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5FsMutationLineageCertificationViolation {
                        path: format!("{base}.{field}"),
                        message: "row field may not be empty".to_owned(),
                    });
                }
            }

            if cert_row.published_state == CertificationStateClass::Qualified
                && (!cert_row.stale_or_narrow_tokens.is_empty()
                    || !cert_row.downgrade_rule_ids.is_empty())
            {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: base.clone(),
                    message: "qualified rows may not carry narrow tokens or downgrade rules"
                        .to_owned(),
                });
            }

            if cert_row.published_state != CertificationStateClass::Qualified
                && (cert_row.stale_or_narrow_tokens.is_empty()
                    || cert_row.downgrade_rule_ids.is_empty())
            {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: base.clone(),
                    message: "non-qualified rows must cite narrow tokens and downgrade rules"
                        .to_owned(),
                });
            }

            if row.coverage.mutation_journal_coverage && cert_row.lineage_root_ids.is_empty() {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: format!("{base}.lineage_root_ids"),
                    message: "mutation-covered rows must cite lineage roots".to_owned(),
                });
            }

            let recovery_expected = matches!(
                row.corruption_state,
                MatrixCorruptionState::RepairFlow
                    | MatrixCorruptionState::BackupRollback
                    | MatrixCorruptionState::FailClosedForPrivilegedOperations
            );
            if recovery_expected
                && cert_row.recovery_family_ids.is_empty()
                && !cert_row
                    .stale_or_narrow_tokens
                    .iter()
                    .any(|token| token == "recovery_mapping_missing")
            {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: format!("{base}.recovery_family_ids"),
                    message: "repair-sensitive rows must cite recovery families".to_owned(),
                });
            }

            if row.coverage.deferred_intent_or_reconcile_exposure
                && matches!(
                    cert_row.primary_root_class,
                    MatrixRootClass::VirtualProviderBacked | MatrixRootClass::ManagedOfflineBundle
                )
                && cert_row.published_state != CertificationStateClass::ReconcileRequired
            {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: base,
                    message:
                        "provider-backed and managed deferred-write rows must publish reconcile_required"
                            .to_owned(),
                });
            }
        }

        for required in CertificationDowngradeTriggerClass::ALL {
            if !self
                .downgrade_rules
                .iter()
                .any(|rule| rule.trigger_class == required)
            {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: "downgrade_rules".to_owned(),
                    message: format!("missing downgrade trigger {}", required.as_str()),
                });
            }
        }

        for required in CertificationSurfaceClass::ALL {
            let Some(binding) = self
                .surface_bindings
                .iter()
                .find(|binding| binding.surface == required)
            else {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: "surface_bindings".to_owned(),
                    message: format!("missing surface binding {}", required.as_str()),
                });
                continue;
            };

            if binding.ingested_packet_id != self.packet_id {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: format!("surface_bindings.{}", required.as_str()),
                    message: "surface binding must ingest the canonical packet id".to_owned(),
                });
            }
            if binding.certification_row_count != self.certification_rows.len() {
                violations.push(M5FsMutationLineageCertificationViolation {
                    path: format!("surface_bindings.{}", required.as_str()),
                    message: "surface binding row count must match certification rows".to_owned(),
                });
            }
            for field in REQUIRED_ROW_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|candidate| candidate == field)
                {
                    violations.push(M5FsMutationLineageCertificationViolation {
                        path: format!("surface_bindings.{}", required.as_str()),
                        message: format!("surface binding must preserve {}", field),
                    });
                }
            }
        }

        violations
    }

    /// Returns true when the packet remains metadata-safe and every binding narrows with it.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .surface_bindings
                .iter()
                .all(|binding| binding.narrow_on_narrowed_rows)
    }

    /// Builds the help-surface projection.
    pub fn help_surface_projection(&self) -> M5FsMutationLineageHelpSurfaceProjection {
        M5FsMutationLineageHelpSurfaceProjection::from_packet(self)
    }

    /// Builds the diagnostics export projection.
    pub fn diagnostics_export_projection(&self) -> M5FsMutationLineageDiagnosticsExportProjection {
        M5FsMutationLineageDiagnosticsExportProjection::from_packet(self)
    }

    /// Builds the support-bundle projection.
    pub fn support_bundle_projection(
        &self,
    ) -> Result<M5FsMutationLineageSupportBundleProjection, M5FsMutationLineageCertificationError>
    {
        M5FsMutationLineageSupportBundleProjection::from_packet(self)
    }

    /// Builds the release-center projection.
    pub fn release_center_projection(&self) -> M5FsMutationLineageReleaseCenterProjection {
        M5FsMutationLineageReleaseCenterProjection::from_packet(self)
    }
}

/// Validation error for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageCertificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Review-safe explanation of the failure.
    pub message: String,
}

/// Row shown on help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpProjectionRow {
    /// Stable certification row id.
    pub certification_row_id: String,
    /// Stable matrix row id.
    pub surface_row_id: String,
    /// Surface class token.
    pub surface_class: MatrixSurfaceClass,
    /// Published certification state.
    pub published_state: CertificationStateClass,
    /// Primary root class.
    pub primary_root_class: MatrixRootClass,
    /// Short user-visible summary.
    pub summary: String,
}

/// Help-surface projection proving help reads the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageHelpSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Canonical packet id.
    pub packet_id: String,
    /// Consumer ref that should render this projection.
    pub consumer_ref: String,
    /// Total visible row count.
    pub certification_row_count: usize,
    /// Rows still fully qualified.
    pub qualified_row_count: usize,
    /// Rows narrowed to limited.
    pub limited_row_count: usize,
    /// Rows narrowed to stale.
    pub stale_row_count: usize,
    /// Rows narrowed to reconcile_required.
    pub reconcile_required_row_count: usize,
    /// Review-safe rows.
    pub rows: Vec<HelpProjectionRow>,
}

impl M5FsMutationLineageHelpSurfaceProjection {
    fn from_packet(packet: &M5FsMutationLineageCertificationPacket) -> Self {
        Self {
            record_kind: "m5_fs_mutation_lineage_help_surface_projection".to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_ref: HELP_CONSUMER_REF.to_owned(),
            certification_row_count: packet.certification_rows.len(),
            qualified_row_count: count_rows(packet, CertificationStateClass::Qualified),
            limited_row_count: count_rows(packet, CertificationStateClass::Limited),
            stale_row_count: count_rows(packet, CertificationStateClass::Stale),
            reconcile_required_row_count: count_rows(
                packet,
                CertificationStateClass::ReconcileRequired,
            ),
            rows: packet
                .certification_rows
                .iter()
                .map(|row| HelpProjectionRow {
                    certification_row_id: row.certification_row_id.clone(),
                    surface_row_id: row.surface_row_id.clone(),
                    surface_class: row.surface_class,
                    published_state: row.published_state,
                    primary_root_class: row.primary_root_class,
                    summary: row.summary.clone(),
                })
                .collect(),
        }
    }
}

/// Diagnostics-export projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsProjectionRow {
    /// Stable certification row id.
    pub certification_row_id: String,
    /// Stable matrix row id.
    pub surface_row_id: String,
    /// Published certification state.
    pub published_state: CertificationStateClass,
    /// Connectivity state preserved by the row.
    pub connectivity_state: MatrixConnectivityState,
    /// Active narrow tokens.
    pub stale_or_narrow_tokens: Vec<String>,
    /// Active downgrade rules.
    pub downgrade_rule_ids: Vec<String>,
}

/// Diagnostics export projection proving diagnostics reads the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageDiagnosticsExportProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Canonical packet id.
    pub packet_id: String,
    /// Consumer ref that should render this projection.
    pub consumer_ref: String,
    /// Total visible row count.
    pub certification_row_count: usize,
    /// Count of rows with explicit narrow tokens.
    pub narrowed_row_count: usize,
    /// Rows quoted verbatim by diagnostics export.
    pub rows: Vec<DiagnosticsProjectionRow>,
}

impl M5FsMutationLineageDiagnosticsExportProjection {
    fn from_packet(packet: &M5FsMutationLineageCertificationPacket) -> Self {
        Self {
            record_kind: "m5_fs_mutation_lineage_diagnostics_export_projection".to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_ref: DIAGNOSTICS_CONSUMER_REF.to_owned(),
            certification_row_count: packet.certification_rows.len(),
            narrowed_row_count: packet
                .certification_rows
                .iter()
                .filter(|row| row.published_state != CertificationStateClass::Qualified)
                .count(),
            rows: packet
                .certification_rows
                .iter()
                .map(|row| DiagnosticsProjectionRow {
                    certification_row_id: row.certification_row_id.clone(),
                    surface_row_id: row.surface_row_id.clone(),
                    published_state: row.published_state,
                    connectivity_state: row.connectivity_state,
                    stale_or_narrow_tokens: row.stale_or_narrow_tokens.clone(),
                    downgrade_rule_ids: row.downgrade_rule_ids.clone(),
                })
                .collect(),
        }
    }
}

/// Support-bundle projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleProjectionRow {
    /// Stable certification row id.
    pub certification_row_id: String,
    /// Stable matrix row id.
    pub surface_row_id: String,
    /// Published certification state.
    pub published_state: CertificationStateClass,
    /// Linked lineage roots preserved in support export.
    pub lineage_root_ids: Vec<String>,
    /// Linked recovery families preserved in support export.
    pub recovery_family_ids: Vec<String>,
    /// Support-safe summary.
    pub summary: String,
}

/// Support-bundle projection proving support export reads the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageSupportBundleProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Canonical packet id.
    pub packet_id: String,
    /// Consumer ref that should render this projection.
    pub consumer_ref: String,
    /// Linked mutation-lineage support envelope id.
    pub mutation_lineage_support_envelope_id: String,
    /// Linked recovery support envelope id.
    pub recovery_support_envelope_id: String,
    /// Raw payloads stay excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority stays excluded.
    pub ambient_authority_excluded: bool,
    /// Rows quoted verbatim by the support-bundle path.
    pub rows: Vec<SupportBundleProjectionRow>,
}

impl M5FsMutationLineageSupportBundleProjection {
    fn from_packet(
        packet: &M5FsMutationLineageCertificationPacket,
    ) -> Result<Self, M5FsMutationLineageCertificationError> {
        let mutation_envelope = compile_m5_mutation_lineage_support_export_envelope(
            "support:m5_fs_mutation_lineage:mutation",
            packet.generated_at.clone(),
        )
        .map_err(M5FsMutationLineageCertificationError::MutationLineageSupportExport)?;
        let recovery_envelope = compile_state_class_recovery_support_export_envelope(
            "support:m5_fs_mutation_lineage:recovery",
            packet.generated_at.clone(),
        )
        .map_err(M5FsMutationLineageCertificationError::StateClassRecoverySupportExport)?;

        Ok(Self {
            record_kind: "m5_fs_mutation_lineage_support_bundle_projection".to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_ref: SUPPORT_BUNDLE_CONSUMER_REF.to_owned(),
            mutation_lineage_support_envelope_id: mutation_envelope.envelope_id,
            recovery_support_envelope_id: recovery_envelope.envelope_id,
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
            rows: packet
                .certification_rows
                .iter()
                .map(|row| SupportBundleProjectionRow {
                    certification_row_id: row.certification_row_id.clone(),
                    surface_row_id: row.surface_row_id.clone(),
                    published_state: row.published_state,
                    lineage_root_ids: row.lineage_root_ids.clone(),
                    recovery_family_ids: row.recovery_family_ids.clone(),
                    summary: row.summary.clone(),
                })
                .collect(),
        })
    }
}

/// Release-center projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCenterProjectionRow {
    /// Stable certification row id.
    pub certification_row_id: String,
    /// Stable matrix row id.
    pub surface_row_id: String,
    /// Published certification state.
    pub published_state: CertificationStateClass,
    /// Root class anchored by the row.
    pub primary_root_class: MatrixRootClass,
    /// Short publication-safe summary.
    pub summary: String,
}

/// Release-center projection proving release surfaces read the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FsMutationLineageReleaseCenterProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Canonical packet id.
    pub packet_id: String,
    /// Consumer ref that should render this projection.
    pub consumer_ref: String,
    /// Total visible row count.
    pub certification_row_count: usize,
    /// Count of rows still fully qualified.
    pub publishable_qualified_row_count: usize,
    /// Count of rows that must narrow before publication.
    pub narrowed_row_count: usize,
    /// Rows quoted verbatim by release-center truth.
    pub rows: Vec<ReleaseCenterProjectionRow>,
}

impl M5FsMutationLineageReleaseCenterProjection {
    fn from_packet(packet: &M5FsMutationLineageCertificationPacket) -> Self {
        Self {
            record_kind: "m5_fs_mutation_lineage_release_center_projection".to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_ref: RELEASE_CENTER_CONSUMER_REF.to_owned(),
            certification_row_count: packet.certification_rows.len(),
            publishable_qualified_row_count: count_rows(packet, CertificationStateClass::Qualified),
            narrowed_row_count: packet
                .certification_rows
                .iter()
                .filter(|row| row.published_state != CertificationStateClass::Qualified)
                .count(),
            rows: packet
                .certification_rows
                .iter()
                .map(|row| ReleaseCenterProjectionRow {
                    certification_row_id: row.certification_row_id.clone(),
                    surface_row_id: row.surface_row_id.clone(),
                    published_state: row.published_state,
                    primary_root_class: row.primary_root_class,
                    summary: row.summary.clone(),
                })
                .collect(),
        }
    }
}

/// Fixture packet variant preserved on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixtureVariantClass {
    /// Canonical checked packet.
    Canonical,
    /// Recovery mappings are missing for rows that otherwise qualified.
    MissingRecoveryLinkage,
}

impl FixtureVariantClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::MissingRecoveryLinkage => "missing_recovery_linkage",
        }
    }
}

/// Error returned when support-export projections cannot be built.
#[derive(Debug)]
pub enum M5FsMutationLineageCertificationError {
    /// The mutation-lineage support export failed to compile.
    MutationLineageSupportExport(crate::M5MutationLineageSupportExportError),
    /// The recovery support export failed to compile.
    StateClassRecoverySupportExport(crate::StateClassRecoverySupportExportError),
}

impl std::fmt::Display for M5FsMutationLineageCertificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MutationLineageSupportExport(err) => {
                write!(f, "mutation-lineage support export failed: {err}")
            }
            Self::StateClassRecoverySupportExport(err) => {
                write!(f, "state-class recovery support export failed: {err}")
            }
        }
    }
}

impl std::error::Error for M5FsMutationLineageCertificationError {}

/// Returns the canonical seeded certification packet.
pub fn seeded_m5_fs_mutation_lineage_certification_packet() -> M5FsMutationLineageCertificationPacket
{
    build_packet(FixtureVariantClass::Canonical)
}

/// Returns a packet with recovery mappings deliberately narrowed.
pub fn seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet(
) -> M5FsMutationLineageCertificationPacket {
    build_packet(FixtureVariantClass::MissingRecoveryLinkage)
}

fn build_packet(variant: FixtureVariantClass) -> M5FsMutationLineageCertificationPacket {
    let matrix = seeded_filesystem_mutation_lineage_matrix_packet();
    validate_filesystem_mutation_lineage_matrix(&matrix)
        .expect("seeded filesystem mutation-lineage matrix validates");
    let truth_review = seeded_filesystem_truth_review_packet();
    validate_filesystem_truth_review_packet(&truth_review)
        .expect("seeded filesystem truth review validates");
    let mutation_packet = seeded_m5_mutation_lineage_packet();
    validate_m5_mutation_lineage_packet(&mutation_packet)
        .expect("seeded mutation-lineage packet validates");
    let recovery_packet = seeded_state_class_recovery_packet();
    validate_state_class_recovery_packet(&recovery_packet)
        .expect("seeded state-class recovery packet validates");

    let certification_rows = matrix
        .rows
        .iter()
        .map(|row| {
            certify_row(
                row,
                &truth_review,
                &mutation_packet,
                &recovery_packet,
                variant,
            )
        })
        .collect::<Vec<_>>();

    let packet_id = "state.m5_fs_mutation_lineage_certification.v1".to_owned();

    M5FsMutationLineageCertificationPacket {
        record_kind: M5_FS_MUTATION_LINEAGE_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_VERSION,
        packet_id: packet_id.clone(),
        generated_at: "2026-06-13T09:15:00Z".to_owned(),
        schema_ref: M5_FS_MUTATION_LINEAGE_CERTIFICATION_SCHEMA_REF.to_owned(),
        doc_ref: M5_FS_MUTATION_LINEAGE_CERTIFICATION_DOC_REF.to_owned(),
        artifact_ref: M5_FS_MUTATION_LINEAGE_CERTIFICATION_ARTIFACT_REF.to_owned(),
        summary_ref: M5_FS_MUTATION_LINEAGE_CERTIFICATION_SUMMARY_REF.to_owned(),
        source_spec_refs: vec![
            ".plans/M05-265.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#1221-real-filesystem-identity-canonical-path-and-save-coordination-architecture".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#2111-connectivity-state-deferred-intent-and-reconciliation-architecture".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#2722-verification-lanes-for-reactive-state-truth-filesystem-identity-mutation-journal-honesty-and-generated-artifact-lineage".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#1117-canonical-filesystem-identity-alias-sets-and-save-coordination".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#1120-virtual-file-system-watch-fidelity-ignore-resolution-and-external-change-truth".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#2332-connectivity-deferred-intent-and-reconciliation-drills".to_owned(),
        ],
        supporting_contract_refs: vec![
            matrix.source_contract_refs.schema_ref.clone(),
            matrix.source_contract_refs.packet_ref.clone(),
            "schemas/state/filesystem_truth_review.schema.json".to_owned(),
            "artifacts/state/filesystem_truth_review.json".to_owned(),
            mutation_packet.source_contract_refs.schema_ref.clone(),
            mutation_packet.source_contract_refs.packet_ref.clone(),
            recovery_packet.source_contract_refs.schema_ref.clone(),
            recovery_packet.source_contract_refs.packet_ref.clone(),
            FIXTURE_MANIFEST_REF.to_owned(),
        ],
        root_classes: collect_roots(&matrix),
        connectivity_states: collect_connectivity_states(&matrix),
        lineage_root_ids: collect_lineage_roots(&mutation_packet),
        recovery_family_ids: collect_recovery_families(&recovery_packet),
        certification_rows,
        downgrade_rules: seeded_downgrade_rules(),
        surface_bindings: seeded_surface_bindings(&packet_id, matrix.rows.len()),
        export_safe_summary: "This metadata-safe certification packet composes filesystem identity, watch/save truth, mutation lineage, and state-class recovery into one publish-or-narrow decision per claimed M5 surface row; generated, imported, provider-backed, and deferred-write rows stay explicit instead of inheriting notebook-local green status.".to_owned(),
    }
}

fn certify_row(
    row: &MatrixRow,
    truth_review: &FilesystemTruthReviewPacket,
    mutation_packet: &M5MutationLineagePacket,
    recovery_packet: &StateClassRecoveryPacket,
    variant: FixtureVariantClass,
) -> SurfaceCertificationRow {
    let review_scenario_ids = linked_review_scenarios(row, truth_review);
    let lineage_root_ids = linked_lineage_roots(row, mutation_packet);
    let mut recovery_family_ids = linked_recovery_families(row, recovery_packet);
    if variant == FixtureVariantClass::MissingRecoveryLinkage
        && matches!(
            row.row_id.as_str(),
            "notebook_document" | "request_workspace_document"
        )
    {
        recovery_family_ids.clear();
    }

    let mut stale_or_narrow_tokens = Vec::new();
    let mut downgrade_rule_ids = Vec::new();
    let published_state = classify_row(
        row,
        &lineage_root_ids,
        &recovery_family_ids,
        &mut stale_or_narrow_tokens,
        &mut downgrade_rule_ids,
    );

    SurfaceCertificationRow {
        certification_row_id: format!("m5_fs_mutation_lineage:{}", row.row_id),
        surface_row_id: row.row_id.clone(),
        surface_class: row.surface_class,
        published_state,
        primary_root_class: row.primary_root_class,
        supported_root_classes: row.supported_root_classes.clone(),
        path_identity_class: row.path_identity_class,
        watch_state: row.watch_state,
        save_fallback: row.save_fallback,
        undo_class: row.undo_class,
        corruption_state: row.corruption_state,
        connectivity_state: row.connectivity_state,
        reconciliation_posture: row.reconciliation_posture,
        review_scenario_ids,
        lineage_root_ids,
        recovery_family_ids,
        source_packet_refs: vec![
            "artifacts/state/filesystem_mutation_lineage_matrix.json".to_owned(),
            "artifacts/state/filesystem_truth_review.json".to_owned(),
            "artifacts/state/m5_mutation_lineage.json".to_owned(),
            "artifacts/state/state_class_recovery.json".to_owned(),
        ],
        stale_or_narrow_tokens,
        downgrade_rule_ids,
        summary: row_summary(row, published_state),
    }
}

fn classify_row(
    row: &MatrixRow,
    lineage_root_ids: &[String],
    recovery_family_ids: &[String],
    stale_or_narrow_tokens: &mut Vec<String>,
    downgrade_rule_ids: &mut Vec<String>,
) -> CertificationStateClass {
    let has_canonical_identity = row.coverage.canonical_filesystem_identity;
    let has_direct_save_target = row.coverage.writable_save_target;
    let watch_is_live = row.watch_state == MatrixWatchState::LiveWatch;
    let recovery_expected = matches!(
        row.corruption_state,
        MatrixCorruptionState::RepairFlow
            | MatrixCorruptionState::BackupRollback
            | MatrixCorruptionState::FailClosedForPrivilegedOperations
    );

    if !has_canonical_identity {
        stale_or_narrow_tokens.push("canonical_identity_unavailable".to_owned());
        downgrade_rule_ids.push("canonical_identity_unavailable_narrows_claim".to_owned());
    }
    if !has_direct_save_target {
        stale_or_narrow_tokens.push(format!("save_target_{}", row.save_fallback.as_str()));
        downgrade_rule_ids.push("canonical_save_target_unavailable_narrows_claim".to_owned());
    }
    if !watch_is_live {
        stale_or_narrow_tokens.push(format!("watch_truth_{}", row.watch_state.as_str()));
        downgrade_rule_ids
            .push("watch_truth_degraded_stales_generated_or_imported_rows".to_owned());
    }
    if row.coverage.mutation_journal_coverage
        && (lineage_root_ids.is_empty()
            || !matches!(
                row.undo_class,
                MatrixUndoClass::ExactUndo | MatrixUndoClass::RestoreFromCheckpoint
            ))
    {
        stale_or_narrow_tokens.push(format!("mutation_scope_{}", row.undo_class.as_str()));
        downgrade_rule_ids.push("mutation_lineage_scope_narrows_nonordinary_rows".to_owned());
    }
    if recovery_expected && recovery_family_ids.is_empty() {
        stale_or_narrow_tokens.push("recovery_mapping_missing".to_owned());
        downgrade_rule_ids.push("recovery_path_scope_narrows_claim".to_owned());
    } else if recovery_expected
        && !matches!(
            row.corruption_state,
            MatrixCorruptionState::RepairFlow | MatrixCorruptionState::BackupRollback
        )
    {
        stale_or_narrow_tokens.push(format!("recovery_scope_{}", row.corruption_state.as_str()));
        downgrade_rule_ids.push("recovery_path_scope_narrows_claim".to_owned());
    }
    if row.coverage.deferred_intent_or_reconcile_exposure {
        stale_or_narrow_tokens.push(format!("reconcile_{}", row.reconciliation_posture.as_str()));
        downgrade_rule_ids.push("deferred_intent_requires_review_or_revalidation".to_owned());
    }

    if row.coverage.deferred_intent_or_reconcile_exposure
        && matches!(
            row.primary_root_class,
            MatrixRootClass::VirtualProviderBacked | MatrixRootClass::ManagedOfflineBundle
        )
    {
        return CertificationStateClass::ReconcileRequired;
    }

    if has_canonical_identity
        && has_direct_save_target
        && watch_is_live
        && row.coverage.mutation_journal_coverage
        && (!recovery_expected || !recovery_family_ids.is_empty())
    {
        stale_or_narrow_tokens.clear();
        downgrade_rule_ids.clear();
        return CertificationStateClass::Qualified;
    }

    if row.coverage.mutation_journal_coverage && !watch_is_live {
        return CertificationStateClass::Stale;
    }

    CertificationStateClass::Limited
}

fn row_summary(row: &MatrixRow, published_state: CertificationStateClass) -> String {
    match published_state {
        CertificationStateClass::Qualified => format!(
            "{} keeps canonical identity, live watch truth, explicit save coordination, attributable lineage, and scoped repair on its claimed roots.",
            row.title
        ),
        CertificationStateClass::Limited => format!(
            "{} stays explicit about inspect-only or generated or copied-save posture and does not inherit an ordinary file claim.",
            row.title
        ),
        CertificationStateClass::Stale => format!(
            "{} remains attributable, but its refresh or watch truth is weaker than a fully live editable file and must stay labeled stale.",
            row.title
        ),
        CertificationStateClass::ReconcileRequired => format!(
            "{} remains available only with explicit reconnect or drift review; deferred work never replays invisibly.",
            row.title
        ),
    }
}

fn linked_review_scenarios(
    row: &MatrixRow,
    truth_review: &FilesystemTruthReviewPacket,
) -> Vec<String> {
    let expected = match row.row_id.as_str() {
        "notebook_document" => Some("scenario.local.notebook"),
        "request_workspace_document" => Some("scenario.remote.request"),
        "preview_output_artifact" => Some("scenario.container.preview"),
        "provider_local_draft" => Some("scenario.generated.draft"),
        _ => None,
    };

    truth_review
        .scenarios
        .iter()
        .filter(|scenario| Some(scenario.scenario_id.as_str()) == expected)
        .map(|scenario| scenario.scenario_id.clone())
        .collect()
}

fn linked_lineage_roots(row: &MatrixRow, mutation_packet: &M5MutationLineagePacket) -> Vec<String> {
    let roots = match row.row_id.as_str() {
        "notebook_document" | "notebook_output_artifact" => {
            vec!["lineage:m5:notebook_execution:0001"]
        }
        "request_workspace_document" | "database_export_artifact" => {
            vec!["lineage:m5:request_batch:0001"]
        }
        "preview_output_artifact" => vec!["lineage:m5:preview_publish:0001"],
        "sync_packet_artifact" | "provider_local_draft" => {
            vec!["lineage:m5:provider_sync:0001"]
        }
        "profiler_trace_artifact" => vec!["lineage:m5:incident_capture:0001"],
        _ => Vec::new(),
    };

    roots
        .into_iter()
        .filter(|root_id| {
            mutation_packet
                .history_inspector_rows
                .iter()
                .any(|row| row.lineage_root_id == *root_id)
        })
        .map(str::to_owned)
        .collect()
}

fn linked_recovery_families(
    row: &MatrixRow,
    recovery_packet: &StateClassRecoveryPacket,
) -> Vec<String> {
    let families = match row.row_id.as_str() {
        "notebook_document" => vec!["notebook_workspace"],
        "notebook_output_artifact" | "database_export_artifact" | "profiler_trace_artifact" => {
            vec!["generated_artifacts"]
        }
        "request_workspace_document" => vec!["request_workspace"],
        "preview_output_artifact" => vec!["preview_cache", "generated_artifacts"],
        "sync_packet_artifact" => vec!["sync_journal"],
        "provider_local_draft" => vec!["provider_local_draft"],
        _ => Vec::new(),
    };

    families
        .into_iter()
        .filter(|family_id| {
            recovery_packet
                .families
                .iter()
                .any(|row| row.family_id == *family_id)
        })
        .map(str::to_owned)
        .collect()
}

fn collect_roots(matrix: &FilesystemMutationLineageMatrixPacket) -> Vec<MatrixRootClass> {
    let mut roots = BTreeSet::new();
    for row in &matrix.rows {
        roots.insert(row.primary_root_class);
        for root in &row.supported_root_classes {
            roots.insert(*root);
        }
    }
    roots.into_iter().collect()
}

fn collect_connectivity_states(
    matrix: &FilesystemMutationLineageMatrixPacket,
) -> Vec<MatrixConnectivityState> {
    matrix
        .rows
        .iter()
        .map(|row| row.connectivity_state)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn collect_lineage_roots(packet: &M5MutationLineagePacket) -> Vec<String> {
    packet
        .history_inspector_rows
        .iter()
        .map(|row| row.lineage_root_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn collect_recovery_families(packet: &StateClassRecoveryPacket) -> Vec<String> {
    packet
        .families
        .iter()
        .map(|row| row.family_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn count_rows(
    packet: &M5FsMutationLineageCertificationPacket,
    state: CertificationStateClass,
) -> usize {
    packet
        .certification_rows
        .iter()
        .filter(|row| row.published_state == state)
        .count()
}

fn seeded_downgrade_rules() -> Vec<CertificationDowngradeRuleRow> {
    vec![
        CertificationDowngradeRuleRow {
            rule_id: "canonical_identity_unavailable_narrows_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::CanonicalIdentityUnavailable,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Limited,
            required_effect: "Rows without canonical filesystem identity must stop publishing ordinary file-target truth.".to_owned(),
            rationale: "Generated, imported, and provider-owned rows may not inherit local-file identity guarantees.".to_owned(),
            evidence_refs: vec![
                "artifacts/state/filesystem_mutation_lineage_matrix.json".to_owned(),
                "artifacts/state/filesystem_truth_review.json".to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "canonical_save_target_unavailable_narrows_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::CanonicalSaveTargetUnavailable,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Limited,
            required_effect: "Rows that only copy or regenerate may not publish direct-save claims.".to_owned(),
            rationale: "Inspect-only, save-as, regenerate-only, and compare-only rows need explicit narrower labels.".to_owned(),
            evidence_refs: vec!["artifacts/state/filesystem_mutation_lineage_matrix.json".to_owned()],
        },
        CertificationDowngradeRuleRow {
            rule_id: "watch_truth_degraded_stales_generated_or_imported_rows".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::WatchTruthDegraded,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Stale,
            required_effect: "Generated, imported, and cached rows must surface stale or reduced-fidelity watch truth instead of implying timely external-change detection.".to_owned(),
            rationale: "Non-live watch posture is a real publication ceiling for generated and imported surfaces.".to_owned(),
            evidence_refs: vec!["artifacts/state/filesystem_truth_review.json".to_owned()],
        },
        CertificationDowngradeRuleRow {
            rule_id: "mutation_lineage_scope_narrows_nonordinary_rows".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::MutationLineageScoped,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Limited,
            required_effect: "Rows whose honest reversal class is compensate, regenerate, manual, or audit-only must name that narrower lineage posture.".to_owned(),
            rationale: "Undo honesty is part of the certification; non-exact reversal classes may not look like ordinary text edits.".to_owned(),
            evidence_refs: vec!["artifacts/state/m5_mutation_lineage.json".to_owned()],
        },
        CertificationDowngradeRuleRow {
            rule_id: "recovery_path_scope_narrows_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::RecoveryPathScoped,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Limited,
            required_effect: "Rows missing scoped repair linkage or rollback routing must narrow before publication.".to_owned(),
            rationale: "State-class repair is part of the M5 continuity claim and cannot be inferred by adjacency.".to_owned(),
            evidence_refs: vec!["artifacts/state/state_class_recovery.json".to_owned()],
        },
        CertificationDowngradeRuleRow {
            rule_id: "deferred_intent_requires_review_or_revalidation".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::DeferredIntentNeedsReview,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::ReconcileRequired,
            required_effect: "Provider-backed and managed deferred-write rows must show reconcile-required until target, auth, and policy are revalidated.".to_owned(),
            rationale: "Deferred managed actions are allowed only with explicit revalidation or manual review, never silent replay.".to_owned(),
            evidence_refs: vec![
                "artifacts/state/filesystem_mutation_lineage_matrix.json".to_owned(),
                "artifacts/state/state_class_recovery.json".to_owned(),
            ],
        },
        CertificationDowngradeRuleRow {
            rule_id: "consumer_binding_missing_blocks_broad_claim".to_owned(),
            trigger_class: CertificationDowngradeTriggerClass::ConsumerBindingMissing,
            source_state: CertificationStateClass::Qualified,
            downgraded_state: CertificationStateClass::Limited,
            required_effect: "If help, diagnostics, support-bundle, or release-center surfaces stop ingesting this packet by reference, broad filesystem-continuity claims must narrow.".to_owned(),
            rationale: "The packet is canonical only if downstream surfaces reuse it instead of cloning status text.".to_owned(),
            evidence_refs: vec![
                HELP_CONSUMER_REF.to_owned(),
                DIAGNOSTICS_CONSUMER_REF.to_owned(),
                SUPPORT_BUNDLE_CONSUMER_REF.to_owned(),
                RELEASE_CENTER_CONSUMER_REF.to_owned(),
            ],
        },
    ]
}

fn seeded_surface_bindings(packet_id: &str, row_count: usize) -> Vec<CertificationSurfaceBinding> {
    vec![
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::HelpSurface,
            consumer_ref: HELP_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: REQUIRED_ROW_FIELDS.iter().map(|item| (*item).to_owned()).collect(),
            narrow_on_narrowed_rows: true,
            explicit_state_labels_required: true,
            summary: "Help surfaces quote the same row ids, states, and downgrade reasons so documentation and in-product continuity explanations stay aligned.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::DiagnosticsExport,
            consumer_ref: DIAGNOSTICS_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: REQUIRED_ROW_FIELDS.iter().map(|item| (*item).to_owned()).collect(),
            narrow_on_narrowed_rows: true,
            explicit_state_labels_required: true,
            summary: "Diagnostics export preserves the same publication state, connectivity posture, and narrow tokens the certification computed.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::SupportBundle,
            consumer_ref: SUPPORT_BUNDLE_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: REQUIRED_ROW_FIELDS.iter().map(|item| (*item).to_owned()).collect(),
            narrow_on_narrowed_rows: true,
            explicit_state_labels_required: true,
            summary: "Support-bundle preview and export carry the same certification rows plus the existing mutation-lineage and recovery support envelopes.".to_owned(),
        },
        CertificationSurfaceBinding {
            surface: CertificationSurfaceClass::ReleaseCenter,
            consumer_ref: RELEASE_CENTER_CONSUMER_REF.to_owned(),
            ingested_packet_id: packet_id.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: REQUIRED_ROW_FIELDS.iter().map(|item| (*item).to_owned()).collect(),
            narrow_on_narrowed_rows: true,
            explicit_state_labels_required: true,
            summary: "Release-center truth uses the same certification packet so shiproom and public-truth lanes cannot overclaim filesystem continuity breadth.".to_owned(),
        },
    ]
}
