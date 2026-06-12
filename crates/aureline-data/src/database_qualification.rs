//! Database connection, statement-safety, and result-grid qualification records.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_auth::{
    seeded_secret_boundary_profile_parity_rows,
    SecretBoundaryCredentialMode, SecretBoundaryCredentialStateRow,
    SecretBoundaryDeclinePath, SecretBoundaryDelegatedCredentialRow,
    SecretBoundaryDelegatedUseClass, SecretBoundaryExportSafetyBanner,
    SecretBoundaryHealthStateClass, SecretBoundaryProjectionMode,
    SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryVaultPickerOption,
    SecretBoundaryVaultPickerState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

/// Supported schema version for database qualification packets.
pub const DATABASE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`DatabaseQualificationPacket`].
pub const DATABASE_QUALIFICATION_RECORD_KIND: &str =
    "database_statement_safety_and_result_grid_qualification";

/// Repo-relative path to the checked-in database qualification packet.
pub const DATABASE_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json";

/// Embedded checked-in packet JSON.
pub const DATABASE_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json"
));

const DATABASE_CONNECTION_MATRIX_ROW_ID: &str = "m5.secret.database.connection_picker";
const DATABASE_HISTORY_MATRIX_ROW_ID: &str = "m5.secret.database.query_history_portability";

/// Qualification label shown on promoted database tooling surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl DatabaseQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Data-tooling surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataToolingSurfaceKind {
    /// Connection picker, connection strip, or connection profile row.
    ConnectionPicker,
    /// SQL editor run bar or query session header.
    SqlRunBar,
    /// Statement-safety preview or review sheet.
    StatementSafetyReview,
    /// Result grid and copy/export review.
    ResultGrid,
    /// Query history and replay row.
    QueryHistory,
    /// Explain-plan pane or imported plan inspector.
    ExplainPlan,
    /// Notebook, chart, AI, clipboard, or support handoff card.
    HandoffCard,
    /// Direct row editing or mutation staging.
    RowMutation,
}

/// Connection target class that must remain visible before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseConnectionClass {
    /// SQLite, DuckDB, or another embedded local-file engine.
    EmbeddedLocal,
    /// Localhost, Docker, devcontainer, or local network development service.
    LocalNetworkContainerDev,
    /// Staging, shared service, SSH tunnel, or other controlled remote target.
    RemoteControlledEnv,
    /// Managed warehouse or cloud data service.
    CloudWarehouse,
    /// Captured import, support packet, or exported result with no live connection.
    ImportedSnapshot,
}

/// Surface that initiated or is inspecting database work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseExecutionOrigin {
    /// Desktop SQL editor.
    DesktopSqlEditor,
    /// Headless or CLI runner.
    CliHeadlessInspect,
    /// Notebook or chart handoff.
    NotebookOrChartHandoff,
    /// AI review surface that proposes or inspects database work.
    AiToolReview,
    /// Support/export reader.
    SupportExport,
    /// Admin or policy review.
    AdminPolicyReview,
}

/// Credential source mode shown without exposing raw secret material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseAuthSourceMode {
    /// Local file has no authentication secret.
    NoAuthLocalFile,
    /// Credential is referenced through the secret broker.
    SecretBrokerHandle,
    /// Identity is delegated from a signed-in session.
    DelegatedIdentity,
    /// Enterprise policy injected the credential reference.
    PolicyInjectedCredential,
    /// Managed workspace or cloud service identity.
    ManagedServiceIdentity,
    /// Imported packet has no live credential.
    ImportedNoLiveAuth,
    /// Auth source is blocked by policy.
    PolicyBlocked,
}

/// Write authority state shown across picker, run bar, history, export, and plan surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseWritePosture {
    /// Engine/session is constrained to read-only work.
    ReadOnly,
    /// Writes are possible after guardrails and review.
    WriteCapable,
    /// Policy blocks execution or export.
    PolicyBlocked,
}

/// Statement-safety class used before any query execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatementSafetyClass {
    /// Query is read-only.
    ReadOnlyQuery,
    /// Statement mutates data rows.
    Dml,
    /// Statement mutates schema or database objects.
    Ddl,
    /// Statement changes session, privileges, or transaction state.
    SessionAffecting,
    /// Script contains more than one statement.
    MultiStatement,
    /// Classifier cannot prove the class.
    Ambiguous,
    /// Statement is blocked on the current target.
    Blocked,
}

impl DatabaseStatementSafetyClass {
    /// Returns true when this class requires review or step-up before execution.
    pub const fn requires_review(self) -> bool {
        matches!(
            self,
            Self::Dml
                | Self::Ddl
                | Self::SessionAffecting
                | Self::MultiStatement
                | Self::Ambiguous
                | Self::Blocked
        )
    }
}

/// Transaction posture shown beside mutation-capable statements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseTransactionPosture {
    /// Statement is running with autocommit.
    Autocommit,
    /// Explicit transaction is open.
    ExplicitTransaction,
    /// Statement is explain-only and does not execute.
    ExplainOnly,
    /// Target is imported or inspect-only.
    NotExecutable,
    /// Transaction state is unknown and requires review.
    UnknownRequiresReview,
}

/// Result-set scope disclosed on grid, export, and handoff actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseResultScope {
    /// Full exact result set is known.
    FullResult,
    /// Visible rows only.
    VisibleRowsOnly,
    /// Result is truncated by row, byte, time, or provider cap.
    Truncated,
    /// Streaming result has no known total.
    StreamingUnknownTotal,
    /// Imported result scope is captured and non-live.
    ImportedSnapshot,
}

/// Redaction mode used when copying, exporting, or handing off results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseRedactionMode {
    /// Metadata-only default export.
    MetadataOnly,
    /// Typed result values are redacted or scoped.
    RedactedTyped,
    /// Visible values require explicit review.
    ReviewedVisibleValues,
    /// Export or handoff is blocked by policy.
    PolicyBlocked,
}

/// Explain-plan capture mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseExplainPlanMode {
    /// Estimated plan only; statement did not execute.
    Estimated,
    /// Actual plan captured from execution.
    Actual,
    /// Imported plan with captured freshness.
    ImportedStale,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseSurfaceGuardSet {
    /// Connection class is visible before action.
    pub connection_class_visible: bool,
    /// Execution origin is visible before action.
    pub execution_origin_visible: bool,
    /// Auth source mode is visible without raw secrets.
    pub auth_source_visible: bool,
    /// Target identity is visible through opaque, non-secret refs.
    pub target_identity_visible: bool,
    /// Read-only, write-capable, or blocked posture is visible.
    pub write_posture_visible: bool,
    /// Statement-safety class is visible before execution.
    pub statement_safety_visible: bool,
    /// Transaction or non-executable posture is visible.
    pub transaction_posture_visible: bool,
    /// Result-set row scope is visible on grid/export/handoff actions.
    pub result_scope_visible: bool,
    /// Export/copy/handoff redaction mode is explicit.
    pub export_redaction_visible: bool,
}

impl DatabaseSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.connection_class_visible
            && self.execution_origin_visible
            && self.auth_source_visible
            && self.target_identity_visible
            && self.write_posture_visible
            && self.statement_safety_visible
            && self.transaction_posture_visible
            && self.result_scope_visible
            && self.export_redaction_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: DataToolingSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: DatabaseQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: DatabaseQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<DatabaseQualificationProof>,
    /// Visible guard set.
    pub guards: DatabaseSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// Connection corpus row used by picker, run bar, history, export, and plan tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConnectionCorpusRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Connection class under test.
    pub connection_class: DatabaseConnectionClass,
    /// Execution origin under test.
    pub execution_origin: DatabaseExecutionOrigin,
    /// Auth source mode under test.
    pub auth_source_mode: DatabaseAuthSourceMode,
    /// Write posture under test.
    pub write_posture: DatabaseWritePosture,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Engine family label.
    pub engine: String,
    /// Current database or schema ref.
    pub current_database_or_schema_ref: String,
    /// Whether the picker row preserves the state.
    pub picker_row: bool,
    /// Whether the run bar preserves the state.
    pub run_bar: bool,
    /// Whether query history preserves the state.
    pub history_row: bool,
    /// Whether export review preserves the state.
    pub export_review: bool,
    /// Whether explain-plan panes preserve the state.
    pub explain_plan: bool,
}

impl DatabaseConnectionCorpusRow {
    /// Returns true when every downstream projection preserves the connection truth.
    pub const fn projected_everywhere(&self) -> bool {
        self.picker_row
            && self.run_bar
            && self.history_row
            && self.export_review
            && self.explain_plan
    }
}

/// Statement-safety lab row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseStatementSafetyLabRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Classified statement class.
    pub statement_class: DatabaseStatementSafetyClass,
    /// Transaction posture shown before execution.
    pub transaction_posture: DatabaseTransactionPosture,
    /// Write posture of the target.
    pub write_posture: DatabaseWritePosture,
    /// Whether object-impact hints are visible when available.
    pub object_impact_visible: bool,
    /// Whether explicit review or step-up is required.
    pub review_or_step_up_required: bool,
    /// Whether execution is blocked.
    pub blocked: bool,
    /// Whether the row represents a protected target.
    pub protected_target: bool,
}

/// Result-grid scale and export lab row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseResultGridLabRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Result-set scope.
    pub result_scope: DatabaseResultScope,
    /// Redaction mode.
    pub redaction_mode: DatabaseRedactionMode,
    /// Whether row or row/column virtualization is active.
    pub virtualization: bool,
    /// Whether large cells require explicit expansion.
    pub large_cell_expansion: bool,
    /// Whether typed column headers are visible.
    pub typed_column_headers: bool,
    /// Whether truncation is disclosed.
    pub truncation_disclosed: bool,
    /// Whether safe preview rules cover binary or rich payloads.
    pub safe_preview_for_binary_or_rich_payloads: bool,
    /// Whether copy/export actions preserve scope, format, and redaction.
    pub copy_export_preserves_scope_format_redaction: bool,
}

/// Query-history privacy and replay lab row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseQueryHistoryLabRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Whether history is local-first.
    pub local_first: bool,
    /// Whether history is bounded by count and age.
    pub bounded: bool,
    /// Whether history is redactable.
    pub redactable: bool,
    /// Whether metadata is stored before raw statement or payload values.
    pub metadata_first: bool,
    /// Whether statement fingerprint is retained.
    pub statement_fingerprint_retained: bool,
    /// Whether no raw secrets, full statements, or row payloads are persisted.
    pub no_raw_secret_statement_or_payload: bool,
}

/// Explain-plan freshness and execution-truth lab row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseExplainPlanLabRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Plan mode.
    pub plan_mode: DatabaseExplainPlanMode,
    /// Engine/version ref.
    pub engine_version_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Whether this plan implies statement execution.
    pub implies_execution: bool,
    /// Whether comparability is claimed.
    pub comparability_claimed: bool,
    /// Whether stale imports are visibly labeled.
    pub stale_import_labeled: bool,
}

/// Handoff lab row for notebook, chart, AI, clipboard, and support/export lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseHandoffLabRow {
    /// Stable scenario id.
    pub case_id: String,
    /// Destination class.
    pub destination_class: String,
    /// Whether source connection/query refs are preserved.
    pub source_refs_preserved: bool,
    /// Whether row/column scope is preserved.
    pub row_column_scope_preserved: bool,
    /// Whether type-fidelity notes are preserved.
    pub type_fidelity_notes_preserved: bool,
    /// Whether freshness is preserved.
    pub freshness_preserved: bool,
    /// Whether share/local restrictions are preserved.
    pub share_local_restrictions_preserved: bool,
}

/// Summary counts for a database qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of connection corpus rows.
    pub connection_corpus_count: usize,
    /// Number of statement lab rows.
    pub statement_lab_count: usize,
    /// Number of result-grid lab rows.
    pub result_grid_lab_count: usize,
    /// Number of query-history lab rows.
    pub query_history_lab_count: usize,
    /// Number of explain-plan lab rows.
    pub explain_plan_lab_count: usize,
    /// Number of handoff lab rows.
    pub handoff_lab_count: usize,
}

/// Canonical database qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<DatabaseSurfaceQualificationRow>,
    /// Connection corpus rows.
    pub connection_corpus: Vec<DatabaseConnectionCorpusRow>,
    /// Statement-safety lab rows.
    pub statement_labs: Vec<DatabaseStatementSafetyLabRow>,
    /// Result-grid lab rows.
    pub result_grid_labs: Vec<DatabaseResultGridLabRow>,
    /// Query-history lab rows.
    pub query_history_labs: Vec<DatabaseQueryHistoryLabRow>,
    /// Explain-plan lab rows.
    pub explain_plan_labs: Vec<DatabaseExplainPlanLabRow>,
    /// Handoff lab rows.
    pub handoff_labs: Vec<DatabaseHandoffLabRow>,
    /// Summary counts.
    pub summary: DatabaseQualificationSummary,
}

impl DatabaseQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> DatabaseQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        DatabaseQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            connection_corpus_count: self.connection_corpus.len(),
            statement_lab_count: self.statement_labs.len(),
            result_grid_lab_count: self.result_grid_labs.len(),
            query_history_lab_count: self.query_history_labs.len(),
            explain_plan_lab_count: self.explain_plan_labs.len(),
            handoff_lab_count: self.handoff_labs.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<DatabaseQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != DATABASE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(DatabaseQualificationViolation::SchemaVersion {
                expected: DATABASE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DATABASE_QUALIFICATION_RECORD_KIND {
            violations.push(DatabaseQualificationViolation::RecordKind {
                expected: DATABASE_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::Surface,
        );
        collect_ids(
            self.connection_corpus
                .iter()
                .map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::Connection,
        );
        collect_ids(
            self.statement_labs.iter().map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::Statement,
        );
        collect_ids(
            self.result_grid_labs.iter().map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::ResultGrid,
        );
        collect_ids(
            self.query_history_labs
                .iter()
                .map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::QueryHistory,
        );
        collect_ids(
            self.explain_plan_labs
                .iter()
                .map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::ExplainPlan,
        );
        collect_ids(
            self.handoff_labs.iter().map(|row| row.case_id.as_str()),
            &mut violations,
            DatabaseQualificationViolationKind::Handoff,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(DatabaseQualificationViolation::StableSurfaceMissingProof {
                        surface_id: surface.surface_id.clone(),
                    });
                }
                if !surface.guards.all_visible() {
                    violations.push(DatabaseQualificationViolation::StableSurfaceMissingGuard {
                        surface_id: surface.surface_id.clone(),
                    });
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    DatabaseQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let connection_classes: BTreeSet<_> = self
            .connection_corpus
            .iter()
            .map(|row| row.connection_class)
            .collect();
        for required_class in [
            DatabaseConnectionClass::EmbeddedLocal,
            DatabaseConnectionClass::LocalNetworkContainerDev,
            DatabaseConnectionClass::RemoteControlledEnv,
            DatabaseConnectionClass::CloudWarehouse,
        ] {
            if !connection_classes.contains(&required_class) {
                violations.push(DatabaseQualificationViolation::MissingConnectionClass {
                    connection_class: required_class,
                });
            }
        }
        if !connection_classes.contains(&DatabaseConnectionClass::ImportedSnapshot) {
            violations.push(DatabaseQualificationViolation::MissingImportedOrManagedOrigin);
        }

        let write_postures: BTreeSet<_> = self
            .connection_corpus
            .iter()
            .map(|row| row.write_posture)
            .collect();
        for required_posture in [
            DatabaseWritePosture::ReadOnly,
            DatabaseWritePosture::WriteCapable,
            DatabaseWritePosture::PolicyBlocked,
        ] {
            if !write_postures.contains(&required_posture) {
                violations.push(DatabaseQualificationViolation::MissingWritePosture {
                    write_posture: required_posture,
                });
            }
        }
        for row in &self.connection_corpus {
            if row.target_identity_ref.is_empty()
                || row.current_database_or_schema_ref.is_empty()
                || !row.projected_everywhere()
            {
                violations.push(
                    DatabaseQualificationViolation::IncompleteConnectionProjection {
                        case_id: row.case_id.clone(),
                    },
                );
            }
        }

        if !self
            .statement_labs
            .iter()
            .any(|row| row.statement_class == DatabaseStatementSafetyClass::Ddl && row.blocked)
        {
            violations.push(DatabaseQualificationViolation::MissingDestructiveStatementBlock);
        }
        if !self.statement_labs.iter().any(|row| {
            row.statement_class == DatabaseStatementSafetyClass::Ambiguous
                && row.review_or_step_up_required
        }) {
            violations.push(DatabaseQualificationViolation::MissingAmbiguousStatementReview);
        }
        for row in &self.statement_labs {
            if row.statement_class.requires_review()
                && !row.review_or_step_up_required
                && !row.blocked
            {
                violations.push(DatabaseQualificationViolation::UnsafeStatementAdmitted {
                    case_id: row.case_id.clone(),
                });
            }
            if row.protected_target
                && row.write_posture == DatabaseWritePosture::WriteCapable
                && !row.review_or_step_up_required
            {
                violations.push(
                    DatabaseQualificationViolation::ProtectedWriteWithoutStepUp {
                        case_id: row.case_id.clone(),
                    },
                );
            }
        }

        for row in &self.result_grid_labs {
            if !(row.virtualization
                && row.large_cell_expansion
                && row.typed_column_headers
                && row.truncation_disclosed
                && row.safe_preview_for_binary_or_rich_payloads
                && row.copy_export_preserves_scope_format_redaction)
            {
                violations.push(DatabaseQualificationViolation::IncompleteResultGridTruth {
                    case_id: row.case_id.clone(),
                });
            }
        }

        for row in &self.query_history_labs {
            if !(row.local_first
                && row.bounded
                && row.redactable
                && row.metadata_first
                && row.statement_fingerprint_retained
                && row.no_raw_secret_statement_or_payload)
            {
                violations.push(DatabaseQualificationViolation::UnsafeQueryHistory {
                    case_id: row.case_id.clone(),
                });
            }
        }

        let plan_modes: BTreeSet<_> = self
            .explain_plan_labs
            .iter()
            .map(|row| row.plan_mode)
            .collect();
        for required_mode in [
            DatabaseExplainPlanMode::Estimated,
            DatabaseExplainPlanMode::Actual,
            DatabaseExplainPlanMode::ImportedStale,
        ] {
            if !plan_modes.contains(&required_mode) {
                violations.push(DatabaseQualificationViolation::MissingExplainPlanMode {
                    plan_mode: required_mode,
                });
            }
        }
        for row in &self.explain_plan_labs {
            if row.engine_version_ref.is_empty() || row.captured_at.is_empty() {
                violations.push(
                    DatabaseQualificationViolation::IncompleteExplainPlanFreshness {
                        case_id: row.case_id.clone(),
                    },
                );
            }
            if row.plan_mode == DatabaseExplainPlanMode::Estimated && row.implies_execution {
                violations.push(
                    DatabaseQualificationViolation::EstimatedPlanImpliesExecution {
                        case_id: row.case_id.clone(),
                    },
                );
            }
            if row.plan_mode == DatabaseExplainPlanMode::ImportedStale && !row.stale_import_labeled
            {
                violations.push(DatabaseQualificationViolation::StalePlanUnlabeled {
                    case_id: row.case_id.clone(),
                });
            }
        }

        for row in &self.handoff_labs {
            if !(row.source_refs_preserved
                && row.row_column_scope_preserved
                && row.type_fidelity_notes_preserved
                && row.freshness_preserved
                && row.share_local_restrictions_preserved)
            {
                violations.push(DatabaseQualificationViolation::IncompleteHandoffTruth {
                    case_id: row.case_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DatabaseQualificationViolation::SummaryMismatch);
        }

        violations
    }

    /// Projects the shared M5 secret-boundary state for the connection-picker
    /// and query-history database surfaces.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let Some(connection_row) = self
            .connection_corpus
            .iter()
            .find(|row| row.picker_row && row.auth_source_mode != DatabaseAuthSourceMode::ImportedNoLiveAuth)
            .or_else(|| self.connection_corpus.first())
        else {
            return Vec::new();
        };
        let history_row = self
            .connection_corpus
            .iter()
            .find(|row| row.history_row)
            .unwrap_or(connection_row);

        vec![
            database_surface_state(
                DATABASE_CONNECTION_MATRIX_ROW_ID,
                "Database connection picker",
                connection_row,
                vec![
                    db_workflow("workflow:database.connect", "Open live database session"),
                    db_workflow("workflow:database.schema", "Browse schema and target context"),
                ],
                "Declining keeps schema inspection, statement review, and imported-result browsing available.",
            ),
            database_surface_state(
                DATABASE_HISTORY_MATRIX_ROW_ID,
                "Database query history",
                history_row,
                vec![
                    db_workflow("workflow:database.history", "Inspect query history"),
                    db_workflow("workflow:database.replay", "Replay stored query metadata"),
                ],
                "Declining keeps history review, redacted exports, and replay preparation available.",
            ),
        ]
    }
}

fn database_surface_state(
    matrix_row_id: &str,
    requester_label: &str,
    row: &DatabaseConnectionCorpusRow,
    dependent_workflows: Vec<SecretBoundaryWorkflowDependency>,
    decline_summary: &str,
) -> SecretBoundarySurfaceState {
    let credential_mode = database_credential_mode(row.auth_source_mode);
    let storage_class = database_storage_class(row.auth_source_mode);
    let projection_mode = database_projection_mode(row.auth_source_mode);
    let secret_class = database_secret_class(row.auth_source_mode);
    let health_state = database_health_state(row.auth_source_mode, row.write_posture);
    let decline_path = SecretBoundaryDeclinePath {
        decline_label: "Stay local-only".to_owned(),
        still_works_summary: decline_summary.to_owned(),
    };
    let target_label = format!("{} on {}", row.engine, row.current_database_or_schema_ref);
    let delegated_credential_row = database_delegated_row(matrix_row_id, row);

    SecretBoundarySurfaceState {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        secret_access_prompt: SecretBoundarySecretAccessPrompt {
            matrix_row_id: matrix_row_id.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            requester_label: requester_label.to_owned(),
            secret_class,
            target_workflow_label: target_label.clone(),
            storage_class,
            credential_mode,
            projection_mode,
            lifetime_label: database_lifetime_label(row.auth_source_mode).to_owned(),
            expires_at: database_expires_at(row.auth_source_mode),
            dependent_workflows: dependent_workflows.clone(),
            decline_path: decline_path.clone(),
        },
        credential_state_row: SecretBoundaryCredentialStateRow {
            matrix_row_id: matrix_row_id.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            display_label: format!("{requester_label} credential state"),
            secret_class,
            source_class: credential_mode,
            target_boundary_label: target_label,
            storage_class,
            projection_mode,
            health_state,
            expires_at: database_expires_at(row.auth_source_mode),
            rotate_action_label: "Rotate connection credential".to_owned(),
            revoke_action_label: "Revoke live DB auth".to_owned(),
            test_action_label: "Test DB auth".to_owned(),
            dependent_workflows,
            decline_path,
        },
        vault_picker: Some(database_picker_state(matrix_row_id, row)),
        delegated_credential_row,
        profile_parity_rows: seeded_secret_boundary_profile_parity_rows(matrix_row_id),
        export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
            matrix_row_id,
            "Raw database credentials remain excluded from profiles, support bundles, query history, and result handoff exports.",
        ),
    }
}

fn db_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn database_picker_state(
    matrix_row_id: &str,
    row: &DatabaseConnectionCorpusRow,
) -> SecretBoundaryVaultPickerState {
    let uses_delegated = matches!(
        row.auth_source_mode,
        DatabaseAuthSourceMode::DelegatedIdentity | DatabaseAuthSourceMode::ManagedServiceIdentity
    );
    SecretBoundaryVaultPickerState {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        picker_label: "Database credential source picker".to_owned(),
        options: vec![
            SecretBoundaryVaultPickerOption {
                option_id: format!("{matrix_row_id}:os-store"),
                option_label: "OS credential store".to_owned(),
                source_class: SecretBoundaryCredentialMode::OsStore,
                storage_class: SecretBoundaryStorageClass::OsStore,
                access_scope_label: "Connection-scoped database auth".to_owned(),
                reveal_policy_label: "Handle only".to_owned(),
                portability_note: "Exports aliases and posture only.".to_owned(),
                open_source_of_truth_action_label: "Open keychain detail".to_owned(),
                selectable: !uses_delegated,
            },
            SecretBoundaryVaultPickerOption {
                option_id: format!("{matrix_row_id}:vault"),
                option_label: "Enterprise vault".to_owned(),
                source_class: SecretBoundaryCredentialMode::EnterpriseVault,
                storage_class: SecretBoundaryStorageClass::EnterpriseVault,
                access_scope_label: "Connection-scoped database auth".to_owned(),
                reveal_policy_label: "Vault or cert binding only".to_owned(),
                portability_note: "Portable exports omit raw values.".to_owned(),
                open_source_of_truth_action_label: "Open vault source".to_owned(),
                selectable: true,
            },
            SecretBoundaryVaultPickerOption {
                option_id: format!("{matrix_row_id}:delegated"),
                option_label: "Delegated or managed identity".to_owned(),
                source_class: SecretBoundaryCredentialMode::Delegated,
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                access_scope_label: "Session-bounded remote database auth".to_owned(),
                reveal_policy_label: "No raw value reveal".to_owned(),
                portability_note: "Reauth or session refresh may be required.".to_owned(),
                open_source_of_truth_action_label: "Open identity detail".to_owned(),
                selectable: true,
            },
        ],
    }
}

fn database_delegated_row(
    matrix_row_id: &str,
    row: &DatabaseConnectionCorpusRow,
) -> Option<SecretBoundaryDelegatedCredentialRow> {
    let delegated_use_class = match row.auth_source_mode {
        DatabaseAuthSourceMode::DelegatedIdentity => {
            SecretBoundaryDelegatedUseClass::ServiceIssuedDelegatedIdentity
        }
        DatabaseAuthSourceMode::ManagedServiceIdentity => {
            SecretBoundaryDelegatedUseClass::RemoteVaultFetch
        }
        _ => return None,
    };

    Some(SecretBoundaryDelegatedCredentialRow {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        delegated_use_class,
        target_host_or_workspace_label: row.target_identity_ref.clone(),
        expires_at: database_expires_at(row.auth_source_mode),
        policy_owner_label: "Data or platform operator".to_owned(),
        stop_forwarding_action_label: "Stop delegated DB auth".to_owned(),
    })
}

fn database_secret_class(auth_mode: DatabaseAuthSourceMode) -> SecretBoundarySecretClass {
    match auth_mode {
        DatabaseAuthSourceMode::DelegatedIdentity | DatabaseAuthSourceMode::ManagedServiceIdentity => {
            SecretBoundarySecretClass::CloudDelegatedIdentity
        }
        DatabaseAuthSourceMode::NoAuthLocalFile | DatabaseAuthSourceMode::ImportedNoLiveAuth => {
            SecretBoundarySecretClass::SessionScopedSecretInput
        }
        _ => SecretBoundarySecretClass::DatabaseCredential,
    }
}

fn database_credential_mode(auth_mode: DatabaseAuthSourceMode) -> SecretBoundaryCredentialMode {
    match auth_mode {
        DatabaseAuthSourceMode::NoAuthLocalFile => SecretBoundaryCredentialMode::SessionOnly,
        DatabaseAuthSourceMode::SecretBrokerHandle => SecretBoundaryCredentialMode::HandleOnly,
        DatabaseAuthSourceMode::DelegatedIdentity => SecretBoundaryCredentialMode::Delegated,
        DatabaseAuthSourceMode::PolicyInjectedCredential => {
            SecretBoundaryCredentialMode::EnterpriseVault
        }
        DatabaseAuthSourceMode::ManagedServiceIdentity => {
            SecretBoundaryCredentialMode::RemoteVaultFetch
        }
        DatabaseAuthSourceMode::ImportedNoLiveAuth | DatabaseAuthSourceMode::PolicyBlocked => {
            SecretBoundaryCredentialMode::NotConfigured
        }
    }
}

fn database_storage_class(auth_mode: DatabaseAuthSourceMode) -> SecretBoundaryStorageClass {
    match auth_mode {
        DatabaseAuthSourceMode::PolicyInjectedCredential => {
            SecretBoundaryStorageClass::EnterpriseVault
        }
        DatabaseAuthSourceMode::ManagedServiceIdentity => SecretBoundaryStorageClass::RemoteVault,
        DatabaseAuthSourceMode::DelegatedIdentity | DatabaseAuthSourceMode::NoAuthLocalFile => {
            SecretBoundaryStorageClass::SessionOnly
        }
        DatabaseAuthSourceMode::ImportedNoLiveAuth | DatabaseAuthSourceMode::PolicyBlocked => {
            SecretBoundaryStorageClass::NotConfigured
        }
        _ => SecretBoundaryStorageClass::OsStore,
    }
}

fn database_projection_mode(auth_mode: DatabaseAuthSourceMode) -> SecretBoundaryProjectionMode {
    match auth_mode {
        DatabaseAuthSourceMode::DelegatedIdentity => SecretBoundaryProjectionMode::Delegated,
        DatabaseAuthSourceMode::ManagedServiceIdentity => {
            SecretBoundaryProjectionMode::RemoteVaultFetch
        }
        DatabaseAuthSourceMode::NoAuthLocalFile
        | DatabaseAuthSourceMode::SecretBrokerHandle
        | DatabaseAuthSourceMode::ImportedNoLiveAuth
        | DatabaseAuthSourceMode::PolicyBlocked => SecretBoundaryProjectionMode::FileDescriptor,
        DatabaseAuthSourceMode::PolicyInjectedCredential => SecretBoundaryProjectionMode::SignOnly,
    }
}

fn database_health_state(
    auth_mode: DatabaseAuthSourceMode,
    write_posture: DatabaseWritePosture,
) -> SecretBoundaryHealthStateClass {
    match auth_mode {
        DatabaseAuthSourceMode::PolicyBlocked => SecretBoundaryHealthStateClass::PolicyBlocked,
        DatabaseAuthSourceMode::ImportedNoLiveAuth => SecretBoundaryHealthStateClass::Missing,
        _ if write_posture == DatabaseWritePosture::PolicyBlocked => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
        _ => SecretBoundaryHealthStateClass::Healthy,
    }
}

fn database_lifetime_label(auth_mode: DatabaseAuthSourceMode) -> &'static str {
    match auth_mode {
        DatabaseAuthSourceMode::DelegatedIdentity | DatabaseAuthSourceMode::ManagedServiceIdentity => {
            "Session-bounded delegated database auth"
        }
        DatabaseAuthSourceMode::ImportedNoLiveAuth | DatabaseAuthSourceMode::PolicyBlocked => {
            "No live credential"
        }
        _ => "Connection-scoped database auth",
    }
}

fn database_expires_at(auth_mode: DatabaseAuthSourceMode) -> Option<String> {
    match auth_mode {
        DatabaseAuthSourceMode::DelegatedIdentity => Some("2026-06-12T18:30:00Z".to_owned()),
        DatabaseAuthSourceMode::ManagedServiceIdentity => {
            Some("2026-06-12T19:15:00Z".to_owned())
        }
        _ => None,
    }
}

/// Loads the checked-in database qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_database_qualification() -> Result<DatabaseQualificationPacket, serde_json::Error> {
    serde_json::from_str(DATABASE_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Connection corpus rows.
    Connection,
    /// Statement lab rows.
    Statement,
    /// Result-grid lab rows.
    ResultGrid,
    /// Query-history lab rows.
    QueryHistory,
    /// Explain-plan lab rows.
    ExplainPlan,
    /// Handoff lab rows.
    Handoff,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<DatabaseQualificationViolation>,
    kind: DatabaseQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(DatabaseQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for database qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: DatabaseQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required connection class is missing.
    MissingConnectionClass {
        connection_class: DatabaseConnectionClass,
    },
    /// Managed or imported origin coverage is missing.
    MissingImportedOrManagedOrigin,
    /// Required write posture is missing.
    MissingWritePosture { write_posture: DatabaseWritePosture },
    /// Connection row does not project target truth everywhere.
    IncompleteConnectionProjection { case_id: String },
    /// Destructive DDL block coverage is missing.
    MissingDestructiveStatementBlock,
    /// Ambiguous-statement review coverage is missing.
    MissingAmbiguousStatementReview,
    /// Review-required statement is admitted without review or block.
    UnsafeStatementAdmitted { case_id: String },
    /// Protected target writes lack step-up.
    ProtectedWriteWithoutStepUp { case_id: String },
    /// Result grid lacks virtualization, expansion, truncation, typing, or export truth.
    IncompleteResultGridTruth { case_id: String },
    /// Query history is not local-first, bounded, redactable, and metadata-first.
    UnsafeQueryHistory { case_id: String },
    /// Required explain-plan mode is missing.
    MissingExplainPlanMode { plan_mode: DatabaseExplainPlanMode },
    /// Explain-plan row lacks engine/version or capture freshness.
    IncompleteExplainPlanFreshness { case_id: String },
    /// Estimated plan incorrectly implies execution.
    EstimatedPlanImpliesExecution { case_id: String },
    /// Stale imported plan is not labeled.
    StalePlanUnlabeled { case_id: String },
    /// Handoff row does not preserve source, scope, type, freshness, and sharing truth.
    IncompleteHandoffTruth { case_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for DatabaseQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingConnectionClass { connection_class } => {
                write!(f, "connection class {connection_class:?} is not covered")
            }
            Self::MissingImportedOrManagedOrigin => {
                write!(f, "managed or imported connection origin is not covered")
            }
            Self::MissingWritePosture { write_posture } => {
                write!(f, "write posture {write_posture:?} is not covered")
            }
            Self::IncompleteConnectionProjection { case_id } => {
                write!(f, "{case_id} does not project connection truth everywhere")
            }
            Self::MissingDestructiveStatementBlock => {
                write!(f, "destructive statement blocking coverage is missing")
            }
            Self::MissingAmbiguousStatementReview => {
                write!(f, "ambiguous statement review coverage is missing")
            }
            Self::UnsafeStatementAdmitted { case_id } => {
                write!(f, "{case_id} admits a review-required statement unsafely")
            }
            Self::ProtectedWriteWithoutStepUp { case_id } => {
                write!(f, "{case_id} writes to a protected target without step-up")
            }
            Self::IncompleteResultGridTruth { case_id } => {
                write!(f, "{case_id} lacks complete result-grid/export truth")
            }
            Self::UnsafeQueryHistory { case_id } => {
                write!(f, "{case_id} lacks safe query-history posture")
            }
            Self::MissingExplainPlanMode { plan_mode } => {
                write!(f, "explain-plan mode {plan_mode:?} is not covered")
            }
            Self::IncompleteExplainPlanFreshness { case_id } => {
                write!(f, "{case_id} lacks plan engine/version or freshness truth")
            }
            Self::EstimatedPlanImpliesExecution { case_id } => {
                write!(f, "{case_id} makes an estimated plan imply execution")
            }
            Self::StalePlanUnlabeled { case_id } => {
                write!(f, "{case_id} does not label a stale imported plan")
            }
            Self::IncompleteHandoffTruth { case_id } => {
                write!(f, "{case_id} does not preserve handoff truth")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for DatabaseQualificationViolation {}
