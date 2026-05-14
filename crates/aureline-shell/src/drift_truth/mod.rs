//! Version-skew and drift-state projection for helper, provider, and saved artifacts.
//!
//! The shell needs one inspectable record that can say why a claimed alpha
//! surface is blocked, retry-only, stale, or waiting for migration review.
//! This module keeps that contract small and export-safe: rows carry typed
//! state, compatibility refs, repair refs, and redaction-safe summaries only.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DriftTruthSnapshot`].
pub const DRIFT_TRUTH_SNAPSHOT_RECORD_KIND: &str = "version_skew_drift_truth_snapshot_record";

/// Stable record-kind tag carried by [`DriftTruthSurfaceRow`].
pub const DRIFT_TRUTH_ROW_RECORD_KIND: &str = "version_skew_drift_truth_row_record";

/// Stable record-kind tag carried by [`DriftTruthExportPacket`].
pub const DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND: &str = "version_skew_drift_truth_export_packet";

/// Schema version for snapshot, row, and export packet records in this module.
pub const DRIFT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Surface family that consumes a drift truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftTruthSurfaceClass {
    /// Remote helper or remote agent capability negotiation.
    HelperAgent,
    /// Provider-linked cached or live snapshot surface.
    ProviderSnapshot,
    /// Saved, generated, restored, or migrated artifact surface.
    SavedArtifact,
}

impl DriftTruthSurfaceClass {
    /// Stable token used in fixtures, packets, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelperAgent => "helper_agent",
            Self::ProviderSnapshot => "provider_snapshot",
            Self::SavedArtifact => "saved_artifact",
        }
    }

    /// Short label suitable for dense shell rows.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HelperAgent => "Helper / agent",
            Self::ProviderSnapshot => "Provider snapshot",
            Self::SavedArtifact => "Saved artifact",
        }
    }
}

/// Closed drift-state vocabulary displayed by claimed alpha rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftStateClass {
    /// The producer and consumer are outside the declared support window.
    UnsupportedSkew,
    /// A retry, probe, reattach, or refresh must run before authority widens.
    RetryRequired,
    /// The surface is reading a captured provider or runtime snapshot.
    StaleSnapshot,
    /// A state or artifact migration stopped at a review gate.
    MigrationReviewNeeded,
}

impl DriftStateClass {
    /// Stable token used in fixtures, packets, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedSkew => "unsupported_skew",
            Self::RetryRequired => "retry_required",
            Self::StaleSnapshot => "stale_snapshot",
            Self::MigrationReviewNeeded => "migration_review_needed",
        }
    }

    /// Visible state label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UnsupportedSkew => "Unsupported skew",
            Self::RetryRequired => "Retry required",
            Self::StaleSnapshot => "Stale snapshot",
            Self::MigrationReviewNeeded => "Migration review needed",
        }
    }

    /// Default severity used by compact shell, support, and review rows.
    pub const fn severity(self) -> DriftSeverityClass {
        match self {
            Self::UnsupportedSkew => DriftSeverityClass::Blocking,
            Self::RetryRequired => DriftSeverityClass::Warning,
            Self::StaleSnapshot => DriftSeverityClass::Warning,
            Self::MigrationReviewNeeded => DriftSeverityClass::Blocking,
        }
    }

    /// Default first action for the visible row.
    pub const fn default_primary_action(self) -> &'static str {
        match self {
            Self::UnsupportedSkew => "Upgrade, repin, or continue local-only",
            Self::RetryRequired => "Retry probe or refresh capability state",
            Self::StaleSnapshot => "Refresh provider state",
            Self::MigrationReviewNeeded => "Open migration review",
        }
    }
}

/// Severity class rendered on drift rows and export packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftSeverityClass {
    /// Informational state.
    Info,
    /// Non-blocking degraded state.
    Warning,
    /// Blocks mutation or promotion until reviewed.
    Blocking,
}

impl DriftSeverityClass {
    /// Stable token used in fixtures, packets, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }
}

/// Mutation posture that pairs with a drift state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftMutationPostureClass {
    /// Unsupported skew blocks remote or provider mutation.
    BlockedUnsupportedSkew,
    /// Reads and inspection may continue while a retry or probe is pending.
    InspectOnlyPendingRetry,
    /// Cached provider data may be inspected but not treated as live authority.
    ReadOnlyStaleSnapshot,
    /// Saved state is available for review only until migration is accepted.
    ReviewOnlyMigrationRequired,
    /// Reserved for future rows whose compatibility proof admits mutation.
    VerifiedMutable,
}

impl DriftMutationPostureClass {
    /// Stable token used in fixtures, packets, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedUnsupportedSkew => "blocked_unsupported_skew",
            Self::InspectOnlyPendingRetry => "inspect_only_pending_retry",
            Self::ReadOnlyStaleSnapshot => "read_only_stale_snapshot",
            Self::ReviewOnlyMigrationRequired => "review_only_migration_required",
            Self::VerifiedMutable => "verified_mutable",
        }
    }

    /// True when this posture allows a remote, provider, or saved-artifact mutation.
    pub const fn permits_mutation(self) -> bool {
        matches!(self, Self::VerifiedMutable)
    }
}

/// Audience for a redaction-safe drift export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftTruthExportAudience {
    /// Support-bundle or support-center packet.
    Support,
    /// Review packet for migration, provider, or helper-state review.
    Review,
}

impl DriftTruthExportAudience {
    /// Stable token used in packet ids.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Support => "support",
            Self::Review => "review",
        }
    }
}

/// Shared version-skew refs a row quotes instead of redefining compatibility locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionSkewTruth {
    /// Boundary family from the mixed-version skew-window contract.
    pub boundary_family: String,
    /// Compatibility row this projection consumes.
    pub compatibility_row_ref: String,
    /// Version-skew register this projection consumes.
    pub version_skew_register_ref: String,
    /// Concrete skew case selected for this row.
    pub skew_case_ref: String,
    /// Skew-window declaration selected for this boundary.
    pub skew_window_declaration_ref: String,
    /// Schema, fixture, doc, or artifact refs that support this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
}

impl VersionSkewTruth {
    /// True when all canonical compatibility refs are present.
    pub fn has_required_refs(&self) -> bool {
        !self.boundary_family.trim().is_empty()
            && !self.compatibility_row_ref.trim().is_empty()
            && !self.version_skew_register_ref.trim().is_empty()
            && !self.skew_case_ref.trim().is_empty()
            && !self.skew_window_declaration_ref.trim().is_empty()
    }
}

/// One dense display row projected from drift truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthDisplayState {
    /// Stable row id the display was built from.
    pub row_id: String,
    /// Surface class token.
    pub surface_class: DriftTruthSurfaceClass,
    /// State class token.
    pub state_class: DriftStateClass,
    /// Visible state label.
    pub state_label: String,
    /// Severity class.
    pub severity: DriftSeverityClass,
    /// Summary shown on the row.
    pub visible_summary: String,
    /// Primary recovery or review action.
    pub primary_action: String,
    /// Mutation posture paired with the state.
    pub mutation_posture: DriftMutationPostureClass,
    /// Compatibility boundary family.
    pub boundary_family: String,
}

/// One helper, provider, or saved-artifact row in the drift truth snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthSurfaceRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Row schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Surface family.
    pub surface_class: DriftTruthSurfaceClass,
    /// Opaque ref to the source surface or packet.
    pub surface_ref: String,
    /// Short title for review and support surfaces.
    pub title: String,
    /// Drift state displayed by the shell.
    pub state_class: DriftStateClass,
    /// Mutation posture paired with the state.
    pub mutation_posture: DriftMutationPostureClass,
    /// Shared compatibility and skew refs.
    pub skew_truth: VersionSkewTruth,
    /// Redaction-safe summary for display and export.
    pub visible_summary: String,
    /// Safe continuation summary.
    pub safe_continuation: String,
    /// Primary user or reviewer action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_action: Option<String>,
    /// Retry, probe, refresh, or reattach ref when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_ref: Option<String>,
    /// Timestamp or epoch when a snapshot crossed the freshness floor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_since: Option<String>,
    /// Review packet or compare ref for migration-gated rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_review_ref: Option<String>,
    /// Repair actions that do not silently widen authority.
    #[serde(default)]
    pub repair_action_refs: Vec<String>,
    /// Mutations or actions blocked by this state.
    #[serde(default)]
    pub blocked_action_refs: Vec<String>,
    /// Preserved artifact refs used for compare, rollback, or export.
    #[serde(default)]
    pub preserved_artifact_refs: Vec<String>,
    /// Redaction-safe support packet refs.
    #[serde(default)]
    pub support_packet_refs: Vec<String>,
    /// Redaction-safe review packet refs.
    #[serde(default)]
    pub review_packet_refs: Vec<String>,
    /// Additional refs local to this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
}

impl DriftTruthSurfaceRow {
    /// Builds the display row shown by shell, CLI, or support summaries.
    pub fn display_state(&self) -> DriftTruthDisplayState {
        DriftTruthDisplayState {
            row_id: self.row_id.clone(),
            surface_class: self.surface_class,
            state_class: self.state_class,
            state_label: self.state_class.label().to_owned(),
            severity: self.state_class.severity(),
            visible_summary: self.visible_summary.clone(),
            primary_action: self
                .next_action
                .clone()
                .unwrap_or_else(|| self.state_class.default_primary_action().to_owned()),
            mutation_posture: self.mutation_posture,
            boundary_family: self.skew_truth.boundary_family.clone(),
        }
    }

    /// Validates row-level drift invariants.
    pub fn validate(&self) -> Result<(), DriftTruthValidationError> {
        if self.record_kind != DRIFT_TRUTH_ROW_RECORD_KIND {
            return Err(DriftTruthValidationError::WrongRowRecordKind {
                row_id: self.row_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != DRIFT_TRUTH_SCHEMA_VERSION {
            return Err(DriftTruthValidationError::WrongRowSchemaVersion {
                row_id: self.row_id.clone(),
                actual: self.schema_version,
            });
        }
        if self.row_id.trim().is_empty() {
            return Err(DriftTruthValidationError::MissingRequiredField {
                row_id: "<unknown>".to_owned(),
                field: "row_id",
            });
        }
        if self.surface_ref.trim().is_empty() {
            return Err(DriftTruthValidationError::MissingRequiredField {
                row_id: self.row_id.clone(),
                field: "surface_ref",
            });
        }
        if !self.skew_truth.has_required_refs() {
            return Err(DriftTruthValidationError::MissingSkewTruthRefs {
                row_id: self.row_id.clone(),
            });
        }
        if self.support_packet_refs.is_empty() {
            return Err(DriftTruthValidationError::MissingExportRefs {
                row_id: self.row_id.clone(),
                audience: DriftTruthExportAudience::Support,
            });
        }
        if self.review_packet_refs.is_empty() {
            return Err(DriftTruthValidationError::MissingExportRefs {
                row_id: self.row_id.clone(),
                audience: DriftTruthExportAudience::Review,
            });
        }
        if self.mutation_posture.permits_mutation() {
            return Err(DriftTruthValidationError::UnsafeMutationPosture {
                row_id: self.row_id.clone(),
                state_class: self.state_class,
                mutation_posture: self.mutation_posture,
            });
        }

        match self.state_class {
            DriftStateClass::UnsupportedSkew => {
                if self.repair_action_refs.is_empty() {
                    return Err(DriftTruthValidationError::MissingRepairActions {
                        row_id: self.row_id.clone(),
                    });
                }
            }
            DriftStateClass::RetryRequired => {
                if self.retry_ref.is_none() {
                    return Err(DriftTruthValidationError::MissingRequiredField {
                        row_id: self.row_id.clone(),
                        field: "retry_ref",
                    });
                }
            }
            DriftStateClass::StaleSnapshot => {
                if self.stale_since.is_none() {
                    return Err(DriftTruthValidationError::MissingRequiredField {
                        row_id: self.row_id.clone(),
                        field: "stale_since",
                    });
                }
            }
            DriftStateClass::MigrationReviewNeeded => {
                if self.migration_review_ref.is_none() {
                    return Err(DriftTruthValidationError::MissingRequiredField {
                        row_id: self.row_id.clone(),
                        field: "migration_review_ref",
                    });
                }
                if self.preserved_artifact_refs.is_empty() {
                    return Err(DriftTruthValidationError::MissingPreservedArtifacts {
                        row_id: self.row_id.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    fn combined_source_refs(&self) -> Vec<String> {
        let mut refs = BTreeSet::new();
        refs.extend(self.source_refs.iter().cloned());
        refs.extend(self.skew_truth.source_refs.iter().cloned());
        refs.into_iter().collect()
    }
}

/// Top-level snapshot of all drift truth rows currently claimed by this alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthSnapshot {
    /// Record discriminator.
    pub record_kind: String,
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Emission timestamp or monotonic test timestamp.
    pub emitted_at: String,
    /// Rows projected into display, support, and review surfaces.
    #[serde(default)]
    pub rows: Vec<DriftTruthSurfaceRow>,
    /// Redaction-safe note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl DriftTruthSnapshot {
    /// Validates snapshot and row invariants.
    pub fn validate(&self) -> Result<(), DriftTruthValidationError> {
        if self.record_kind != DRIFT_TRUTH_SNAPSHOT_RECORD_KIND {
            return Err(DriftTruthValidationError::WrongSnapshotRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != DRIFT_TRUTH_SCHEMA_VERSION {
            return Err(DriftTruthValidationError::WrongSnapshotSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.snapshot_id.trim().is_empty() {
            return Err(DriftTruthValidationError::MissingSnapshotId);
        }
        if self.rows.is_empty() {
            return Err(DriftTruthValidationError::EmptySnapshot);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            row.validate()?;
            if !row_ids.insert(row.row_id.clone()) {
                return Err(DriftTruthValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Returns the shell-ready display rows in snapshot order.
    pub fn display_rows(&self) -> Vec<DriftTruthDisplayState> {
        self.rows
            .iter()
            .map(DriftTruthSurfaceRow::display_state)
            .collect()
    }

    /// Returns a row by stable id.
    pub fn row_by_id(&self, row_id: &str) -> Option<&DriftTruthSurfaceRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the state classes covered by this snapshot.
    pub fn state_classes(&self) -> BTreeSet<DriftStateClass> {
        self.rows.iter().map(|row| row.state_class).collect()
    }

    /// Returns the surface classes covered by this snapshot.
    pub fn surface_classes(&self) -> BTreeSet<DriftTruthSurfaceClass> {
        self.rows.iter().map(|row| row.surface_class).collect()
    }

    /// True when this snapshot covers the four claimed alpha drift states.
    pub fn has_alpha_state_coverage(&self) -> bool {
        let states = self.state_classes();
        [
            DriftStateClass::UnsupportedSkew,
            DriftStateClass::RetryRequired,
            DriftStateClass::StaleSnapshot,
            DriftStateClass::MigrationReviewNeeded,
        ]
        .into_iter()
        .all(|state| states.contains(&state))
    }

    /// True when this snapshot covers helper, provider, and saved-artifact consumers.
    pub fn has_alpha_surface_coverage(&self) -> bool {
        let surfaces = self.surface_classes();
        [
            DriftTruthSurfaceClass::HelperAgent,
            DriftTruthSurfaceClass::ProviderSnapshot,
            DriftTruthSurfaceClass::SavedArtifact,
        ]
        .into_iter()
        .all(|surface| surfaces.contains(&surface))
    }

    /// Builds a redaction-safe support or review packet from the same rows.
    pub fn export_packet(&self, audience: DriftTruthExportAudience) -> DriftTruthExportPacket {
        DriftTruthExportPacket {
            record_kind: DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: DRIFT_TRUTH_SCHEMA_VERSION,
            packet_id: format!("{}:{}", self.snapshot_id, audience.as_str()),
            audience,
            snapshot_id: self.snapshot_id.clone(),
            emitted_at: self.emitted_at.clone(),
            raw_payloads_excluded: true,
            rows: self
                .rows
                .iter()
                .map(|row| DriftTruthExportRow::from_row(row, audience))
                .collect(),
            notes: "Metadata-only drift packet; raw provider payloads, target paths, logs, secrets, and artifact bodies are excluded."
                .to_owned(),
        }
    }
}

/// One export-safe row inside [`DriftTruthExportPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthExportRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface family.
    pub surface_class: DriftTruthSurfaceClass,
    /// Drift state.
    pub state_class: DriftStateClass,
    /// Visible state label.
    pub state_label: String,
    /// Severity class.
    pub severity: DriftSeverityClass,
    /// Mutation posture.
    pub mutation_posture: DriftMutationPostureClass,
    /// Boundary family.
    pub boundary_family: String,
    /// Compatibility row ref.
    pub compatibility_row_ref: String,
    /// Version-skew register ref.
    pub version_skew_register_ref: String,
    /// Concrete skew case ref.
    pub skew_case_ref: String,
    /// Skew-window declaration ref.
    pub skew_window_declaration_ref: String,
    /// Redaction-safe summary.
    pub visible_summary: String,
    /// Safe continuation summary.
    pub safe_continuation: String,
    /// Export refs selected for the packet audience.
    pub packet_refs: Vec<String>,
    /// Blocked action refs.
    pub blocked_action_refs: Vec<String>,
    /// Repair action refs.
    pub repair_action_refs: Vec<String>,
    /// Preserved artifact refs.
    pub preserved_artifact_refs: Vec<String>,
    /// Source refs used to reconstruct the row without unsafe payloads.
    pub source_refs: Vec<String>,
}

impl DriftTruthExportRow {
    /// Projects one row for a support or review packet.
    pub fn from_row(row: &DriftTruthSurfaceRow, audience: DriftTruthExportAudience) -> Self {
        let packet_refs = match audience {
            DriftTruthExportAudience::Support => row.support_packet_refs.clone(),
            DriftTruthExportAudience::Review => row.review_packet_refs.clone(),
        };

        Self {
            row_id: row.row_id.clone(),
            surface_class: row.surface_class,
            state_class: row.state_class,
            state_label: row.state_class.label().to_owned(),
            severity: row.state_class.severity(),
            mutation_posture: row.mutation_posture,
            boundary_family: row.skew_truth.boundary_family.clone(),
            compatibility_row_ref: row.skew_truth.compatibility_row_ref.clone(),
            version_skew_register_ref: row.skew_truth.version_skew_register_ref.clone(),
            skew_case_ref: row.skew_truth.skew_case_ref.clone(),
            skew_window_declaration_ref: row.skew_truth.skew_window_declaration_ref.clone(),
            visible_summary: row.visible_summary.clone(),
            safe_continuation: row.safe_continuation.clone(),
            packet_refs,
            blocked_action_refs: row.blocked_action_refs.clone(),
            repair_action_refs: row.repair_action_refs.clone(),
            preserved_artifact_refs: row.preserved_artifact_refs.clone(),
            source_refs: row.combined_source_refs(),
        }
    }
}

/// Export-safe support or review packet for drift truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthExportPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Packet audience.
    pub audience: DriftTruthExportAudience,
    /// Source snapshot id.
    pub snapshot_id: String,
    /// Emission timestamp or monotonic test timestamp.
    pub emitted_at: String,
    /// Always true. The packet carries metadata and refs only.
    pub raw_payloads_excluded: bool,
    /// Export-safe rows.
    pub rows: Vec<DriftTruthExportRow>,
    /// Redaction-safe packet note.
    pub notes: String,
}

impl DriftTruthExportPacket {
    /// True when the packet is structurally safe to include in support or review exports.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payloads_excluded
            && self.record_kind == DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND
            && self.schema_version == DRIFT_TRUTH_SCHEMA_VERSION
            && self
                .rows
                .iter()
                .all(|row| !row.packet_refs.is_empty() && !row.source_refs.is_empty())
    }
}

/// Validation error for drift truth records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftTruthValidationError {
    /// Snapshot record kind was not recognized.
    WrongSnapshotRecordKind { actual: String },
    /// Snapshot schema version is unsupported.
    WrongSnapshotSchemaVersion { actual: u32 },
    /// Row record kind was not recognized.
    WrongRowRecordKind { row_id: String, actual: String },
    /// Row schema version is unsupported.
    WrongRowSchemaVersion { row_id: String, actual: u32 },
    /// Snapshot id is empty.
    MissingSnapshotId,
    /// Snapshot contains no rows.
    EmptySnapshot,
    /// A required row field is missing.
    MissingRequiredField { row_id: String, field: &'static str },
    /// A row did not quote the canonical compatibility refs.
    MissingSkewTruthRefs { row_id: String },
    /// A row lacks support or review export refs.
    MissingExportRefs {
        row_id: String,
        audience: DriftTruthExportAudience,
    },
    /// A blocked or review state attempted to permit mutation.
    UnsafeMutationPosture {
        row_id: String,
        state_class: DriftStateClass,
        mutation_posture: DriftMutationPostureClass,
    },
    /// Unsupported skew did not name a repair path.
    MissingRepairActions { row_id: String },
    /// Migration review did not name preserved artifacts.
    MissingPreservedArtifacts { row_id: String },
    /// Row ids must be unique within a snapshot.
    DuplicateRowId { row_id: String },
}

impl fmt::Display for DriftTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSnapshotRecordKind { actual } => {
                write!(f, "unsupported drift snapshot record kind {actual}")
            }
            Self::WrongSnapshotSchemaVersion { actual } => {
                write!(f, "unsupported drift snapshot schema version {actual}")
            }
            Self::WrongRowRecordKind { row_id, actual } => {
                write!(f, "row {row_id} has unsupported record kind {actual}")
            }
            Self::WrongRowSchemaVersion { row_id, actual } => {
                write!(f, "row {row_id} has unsupported schema version {actual}")
            }
            Self::MissingSnapshotId => write!(f, "drift snapshot id is required"),
            Self::EmptySnapshot => write!(f, "drift snapshot must contain at least one row"),
            Self::MissingRequiredField { row_id, field } => {
                write!(f, "row {row_id} is missing required field {field}")
            }
            Self::MissingSkewTruthRefs { row_id } => {
                write!(f, "row {row_id} is missing compatibility skew refs")
            }
            Self::MissingExportRefs { row_id, audience } => write!(
                f,
                "row {row_id} is missing {} export refs",
                audience.as_str()
            ),
            Self::UnsafeMutationPosture {
                row_id,
                state_class,
                mutation_posture,
            } => write!(
                f,
                "row {row_id} state {} cannot use mutation posture {}",
                state_class.as_str(),
                mutation_posture.as_str()
            ),
            Self::MissingRepairActions { row_id } => {
                write!(f, "row {row_id} must name repair actions")
            }
            Self::MissingPreservedArtifacts { row_id } => {
                write!(f, "row {row_id} must name preserved artifact refs")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate drift row id {row_id}"),
        }
    }
}

impl std::error::Error for DriftTruthValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn skew_truth() -> VersionSkewTruth {
        VersionSkewTruth {
            boundary_family: "desktop_cli_and_remote_agent".to_owned(),
            compatibility_row_ref: "compat_row:remote.attach_envelope_and_drift".to_owned(),
            version_skew_register_ref: "skew_register:remote.attach_envelope_and_drift".to_owned(),
            skew_case_ref: "skew_case:remote.attach_unknown_required_feature".to_owned(),
            skew_window_declaration_ref:
                "skew_window:desktop_cli_and_remote_agent.declared_adjacent_window".to_owned(),
            source_refs: vec!["fixtures/remote/mixed_version_drift_alpha/manifest.yaml".to_owned()],
        }
    }

    fn unsupported_row() -> DriftTruthSurfaceRow {
        DriftTruthSurfaceRow {
            record_kind: DRIFT_TRUTH_ROW_RECORD_KIND.to_owned(),
            schema_version: DRIFT_TRUTH_SCHEMA_VERSION,
            row_id: "drift.row.helper.unsupported".to_owned(),
            surface_class: DriftTruthSurfaceClass::HelperAgent,
            surface_ref: "helper_capability_envelope:remote_agent.unsupported_required_feature"
                .to_owned(),
            title: "Unsupported helper skew".to_owned(),
            state_class: DriftStateClass::UnsupportedSkew,
            mutation_posture: DriftMutationPostureClass::BlockedUnsupportedSkew,
            skew_truth: skew_truth(),
            visible_summary: "Helper skew is outside the declared window.".to_owned(),
            safe_continuation: "Continue local-only.".to_owned(),
            next_action: None,
            retry_ref: None,
            stale_since: None,
            migration_review_ref: None,
            repair_action_refs: vec!["upgrade_or_repin_helper".to_owned()],
            blocked_action_refs: vec!["mutation:remote.file.write.pending".to_owned()],
            preserved_artifact_refs: Vec::new(),
            support_packet_refs: vec!["support_packet:helper.unsupported".to_owned()],
            review_packet_refs: vec!["review_packet:helper.unsupported".to_owned()],
            source_refs: vec![
                "fixtures/remote/mixed_version_drift_alpha/remote_agent_unsupported_required_feature.yaml"
                    .to_owned(),
            ],
        }
    }

    #[test]
    fn unsupported_skew_row_projects_display_and_export() {
        let row = unsupported_row();
        row.validate().expect("row validates");

        let display = row.display_state();
        assert_eq!(display.state_label, "Unsupported skew");
        assert_eq!(display.severity, DriftSeverityClass::Blocking);
        assert!(!display.mutation_posture.permits_mutation());

        let export = DriftTruthExportRow::from_row(&row, DriftTruthExportAudience::Support);
        assert_eq!(
            export.packet_refs,
            vec!["support_packet:helper.unsupported"]
        );
        assert!(export
            .source_refs
            .iter()
            .any(|item| item.ends_with("remote_agent_unsupported_required_feature.yaml")));
    }

    #[test]
    fn unsafe_mutation_posture_is_rejected() {
        let mut row = unsupported_row();
        row.mutation_posture = DriftMutationPostureClass::VerifiedMutable;
        assert!(matches!(
            row.validate(),
            Err(DriftTruthValidationError::UnsafeMutationPosture { .. })
        ));
    }
}
