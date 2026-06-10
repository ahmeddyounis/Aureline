//! Connection browsers, schema trees, and target-context envelope qualification
//! records for database tooling.
//!
//! This module owns the typed records that keep connection browsers, schema
//! trees, and target-context envelopes inspectable and attributable without
//! depending on hidden shell shortcuts or ad hoc scripts. The boundary schema is
//! [`/schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json`](../../../schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`](../../../artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, and raw connection
//! strings do not belong in these records. They carry stable IDs, closed
//! posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for database-browser qualification packets.
pub const DATABASE_BROWSER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`DatabaseBrowserQualificationPacket`].
pub const DATABASE_BROWSER_QUALIFICATION_RECORD_KIND: &str =
    "connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling";

/// Repo-relative path to the checked-in database-browser qualification packet.
pub const DATABASE_BROWSER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json";

/// Embedded checked-in packet JSON.
pub const DATABASE_BROWSER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json"
));

/// Qualification label shown on promoted database-browser surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseBrowserQualificationLabel {
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

impl DatabaseBrowserQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Database-browser surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseBrowserSurfaceKind {
    /// Connection browser, picker, or profile list.
    ConnectionBrowser,
    /// Schema tree inspector or navigator.
    SchemaTree,
    /// Target-context envelope showing endpoint, auth, write posture, and safety.
    TargetContextEnvelope,
}

/// Connection target class that must remain visible before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseBrowserConnectionClass {
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

/// Credential source mode shown without exposing raw secret material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseBrowserAuthSourceMode {
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

/// Write authority state shown across browser, tree, and envelope surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseBrowserWritePosture {
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
pub enum DatabaseBrowserStatementSafetyClass {
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

impl DatabaseBrowserStatementSafetyClass {
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
pub enum DatabaseBrowserTransactionPosture {
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
pub enum DatabaseBrowserResultScope {
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
pub enum DatabaseBrowserRedactionMode {
    /// Metadata-only default export.
    MetadataOnly,
    /// Typed result values are redacted or scoped.
    RedactedTyped,
    /// Visible values require explicit review.
    ReviewedVisibleValues,
    /// Export or handoff is blocked by policy.
    PolicyBlocked,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseBrowserQualificationProof {
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
pub struct DatabaseBrowserSurfaceGuardSet {
    /// Connection class is visible before action.
    pub connection_class_visible: bool,
    /// Auth source mode is visible without raw secrets.
    pub auth_source_visible: bool,
    /// Read-only, write-capable, or blocked posture is visible.
    pub write_posture_visible: bool,
    /// Target identity is visible through opaque, non-secret refs.
    pub target_identity_visible: bool,
    /// Schema tree is visible and navigable.
    pub schema_tree_visible: bool,
    /// Target-context envelope is visible before send.
    pub target_context_envelope_visible: bool,
    /// Statement-safety class is visible before execution.
    pub statement_safety_visible: bool,
    /// Export/copy/handoff redaction mode is explicit.
    pub export_redaction_visible: bool,
}

impl DatabaseBrowserSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.connection_class_visible
            && self.auth_source_visible
            && self.write_posture_visible
            && self.target_identity_visible
            && self.schema_tree_visible
            && self.target_context_envelope_visible
            && self.statement_safety_visible
            && self.export_redaction_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseBrowserSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: DatabaseBrowserSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: DatabaseBrowserQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: DatabaseBrowserQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<DatabaseBrowserQualificationProof>,
    /// Visible guard set.
    pub guards: DatabaseBrowserSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One connection browser row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionBrowserRow {
    /// Stable browser id.
    pub browser_id: String,
    /// Connection class.
    pub connection_class: DatabaseBrowserConnectionClass,
    /// Auth source mode.
    pub auth_source_mode: DatabaseBrowserAuthSourceMode,
    /// Write posture.
    pub write_posture: DatabaseBrowserWritePosture,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Engine family label.
    pub engine: String,
    /// Current database or schema ref.
    pub current_database_or_schema_ref: String,
    /// Whether the browser supports schema tree navigation.
    pub schema_tree_supported: bool,
    /// Whether the browser supports target-context envelopes.
    pub target_context_envelope_supported: bool,
    /// Whether the browser is visible in UI.
    pub visible_in_ui: bool,
}

/// One schema tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaTreeRow {
    /// Stable tree id.
    pub tree_id: String,
    /// Source kind (engine or protocol family).
    pub source_kind: String,
    /// Root node ref.
    pub root_node_ref: String,
    /// Depth limit.
    pub depth_limit: u32,
    /// Node count limit.
    pub node_count_limit: u32,
    /// Freshness state.
    pub freshness_state: String,
    /// Whether stale schema is visibly labeled.
    pub stale_labeled: bool,
    /// Whether stale schema may masquerade as live truth.
    pub may_masquerade_as_live: bool,
    /// Whether the tree is visible in UI.
    pub visible_in_ui: bool,
}

/// One target-context envelope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetContextEnvelopeRow {
    /// Stable envelope id.
    pub envelope_id: String,
    /// Target endpoint identity ref.
    pub target_endpoint_ref: String,
    /// Connection class.
    pub connection_class: DatabaseBrowserConnectionClass,
    /// Auth source mode.
    pub auth_source_mode: DatabaseBrowserAuthSourceMode,
    /// Write posture.
    pub write_posture: DatabaseBrowserWritePosture,
    /// Statement-safety class.
    pub statement_safety_class: DatabaseBrowserStatementSafetyClass,
    /// Transaction posture.
    pub transaction_posture: DatabaseBrowserTransactionPosture,
    /// Result-set scope.
    pub result_scope: DatabaseBrowserResultScope,
    /// Redaction mode.
    pub redaction_mode: DatabaseBrowserRedactionMode,
    /// Whether the envelope is visible before send.
    pub visible_before_send: bool,
    /// Whether the envelope is visible in UI.
    pub visible_in_ui: bool,
}

/// Summary counts for a database-browser qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseBrowserQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of connection browser rows.
    pub connection_browser_count: usize,
    /// Number of schema tree rows.
    pub schema_tree_count: usize,
    /// Number of target-context envelope rows.
    pub target_context_envelope_count: usize,
}

/// Canonical database-browser qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseBrowserQualificationPacket {
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
    pub surfaces: Vec<DatabaseBrowserSurfaceQualificationRow>,
    /// Connection browser rows.
    pub connection_browsers: Vec<ConnectionBrowserRow>,
    /// Schema tree rows.
    pub schema_trees: Vec<SchemaTreeRow>,
    /// Target-context envelope rows.
    pub target_context_envelopes: Vec<TargetContextEnvelopeRow>,
    /// Summary counts.
    pub summary: DatabaseBrowserQualificationSummary,
}

impl DatabaseBrowserQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> DatabaseBrowserQualificationSummary {
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
        DatabaseBrowserQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            connection_browser_count: self.connection_browsers.len(),
            schema_tree_count: self.schema_trees.len(),
            target_context_envelope_count: self.target_context_envelopes.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<DatabaseBrowserQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != DATABASE_BROWSER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(DatabaseBrowserQualificationViolation::SchemaVersion {
                expected: DATABASE_BROWSER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DATABASE_BROWSER_QUALIFICATION_RECORD_KIND {
            violations.push(DatabaseBrowserQualificationViolation::RecordKind {
                expected: DATABASE_BROWSER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            DatabaseBrowserQualificationViolationKind::Surface,
        );
        collect_ids(
            self.connection_browsers
                .iter()
                .map(|row| row.browser_id.as_str()),
            &mut violations,
            DatabaseBrowserQualificationViolationKind::ConnectionBrowser,
        );
        collect_ids(
            self.schema_trees.iter().map(|row| row.tree_id.as_str()),
            &mut violations,
            DatabaseBrowserQualificationViolationKind::SchemaTree,
        );
        collect_ids(
            self.target_context_envelopes
                .iter()
                .map(|row| row.envelope_id.as_str()),
            &mut violations,
            DatabaseBrowserQualificationViolationKind::TargetContextEnvelope,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        DatabaseBrowserQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        DatabaseBrowserQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    DatabaseBrowserQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let connection_classes: BTreeSet<_> = self
            .connection_browsers
            .iter()
            .map(|row| row.connection_class)
            .chain(
                self.target_context_envelopes
                    .iter()
                    .map(|row| row.connection_class),
            )
            .collect();
        for required_class in [
            DatabaseBrowserConnectionClass::EmbeddedLocal,
            DatabaseBrowserConnectionClass::LocalNetworkContainerDev,
            DatabaseBrowserConnectionClass::RemoteControlledEnv,
            DatabaseBrowserConnectionClass::CloudWarehouse,
        ] {
            if !connection_classes.contains(&required_class) {
                violations.push(
                    DatabaseBrowserQualificationViolation::MissingConnectionClass {
                        connection_class: required_class,
                    },
                );
            }
        }
        if !connection_classes.contains(&DatabaseBrowserConnectionClass::ImportedSnapshot) {
            violations.push(DatabaseBrowserQualificationViolation::MissingImportedSnapshotOrigin);
        }

        let auth_modes: BTreeSet<_> = self
            .connection_browsers
            .iter()
            .map(|row| row.auth_source_mode)
            .chain(
                self.target_context_envelopes
                    .iter()
                    .map(|row| row.auth_source_mode),
            )
            .collect();
        for required_mode in [
            DatabaseBrowserAuthSourceMode::NoAuthLocalFile,
            DatabaseBrowserAuthSourceMode::SecretBrokerHandle,
            DatabaseBrowserAuthSourceMode::DelegatedIdentity,
            DatabaseBrowserAuthSourceMode::PolicyBlocked,
        ] {
            if !auth_modes.contains(&required_mode) {
                violations.push(
                    DatabaseBrowserQualificationViolation::MissingAuthSourceMode {
                        auth_source_mode: required_mode,
                    },
                );
            }
        }

        let write_postures: BTreeSet<_> = self
            .connection_browsers
            .iter()
            .map(|row| row.write_posture)
            .chain(
                self.target_context_envelopes
                    .iter()
                    .map(|row| row.write_posture),
            )
            .collect();
        for required_posture in [
            DatabaseBrowserWritePosture::ReadOnly,
            DatabaseBrowserWritePosture::WriteCapable,
            DatabaseBrowserWritePosture::PolicyBlocked,
        ] {
            if !write_postures.contains(&required_posture) {
                violations.push(DatabaseBrowserQualificationViolation::MissingWritePosture {
                    write_posture: required_posture,
                });
            }
        }

        for row in &self.connection_browsers {
            if row.target_identity_ref.is_empty()
                || row.engine.is_empty()
                || row.current_database_or_schema_ref.is_empty()
                || !row.visible_in_ui
            {
                violations.push(
                    DatabaseBrowserQualificationViolation::IncompleteConnectionBrowserProjection {
                        browser_id: row.browser_id.clone(),
                    },
                );
            }
        }

        for row in &self.schema_trees {
            if row.root_node_ref.is_empty() || row.freshness_state.is_empty() || !row.visible_in_ui
            {
                violations.push(
                    DatabaseBrowserQualificationViolation::IncompleteSchemaTreeProjection {
                        tree_id: row.tree_id.clone(),
                    },
                );
            }
            if row.may_masquerade_as_live {
                violations.push(
                    DatabaseBrowserQualificationViolation::SchemaTreeMayMasqueradeAsLive {
                        tree_id: row.tree_id.clone(),
                    },
                );
            }
        }

        let statement_safety_classes: BTreeSet<_> = self
            .target_context_envelopes
            .iter()
            .map(|row| row.statement_safety_class)
            .collect();
        for required_class in [
            DatabaseBrowserStatementSafetyClass::ReadOnlyQuery,
            DatabaseBrowserStatementSafetyClass::Dml,
            DatabaseBrowserStatementSafetyClass::Ddl,
            DatabaseBrowserStatementSafetyClass::SessionAffecting,
            DatabaseBrowserStatementSafetyClass::MultiStatement,
            DatabaseBrowserStatementSafetyClass::Ambiguous,
            DatabaseBrowserStatementSafetyClass::Blocked,
        ] {
            if !statement_safety_classes.contains(&required_class) {
                violations.push(
                    DatabaseBrowserQualificationViolation::MissingStatementSafetyClass {
                        statement_safety_class: required_class,
                    },
                );
            }
        }

        let transaction_postures: BTreeSet<_> = self
            .target_context_envelopes
            .iter()
            .map(|row| row.transaction_posture)
            .collect();
        for required_posture in [
            DatabaseBrowserTransactionPosture::Autocommit,
            DatabaseBrowserTransactionPosture::ExplicitTransaction,
            DatabaseBrowserTransactionPosture::ExplainOnly,
            DatabaseBrowserTransactionPosture::NotExecutable,
            DatabaseBrowserTransactionPosture::UnknownRequiresReview,
        ] {
            if !transaction_postures.contains(&required_posture) {
                violations.push(
                    DatabaseBrowserQualificationViolation::MissingTransactionPosture {
                        transaction_posture: required_posture,
                    },
                );
            }
        }

        let result_scopes: BTreeSet<_> = self
            .target_context_envelopes
            .iter()
            .map(|row| row.result_scope)
            .collect();
        for required_scope in [
            DatabaseBrowserResultScope::FullResult,
            DatabaseBrowserResultScope::VisibleRowsOnly,
            DatabaseBrowserResultScope::Truncated,
            DatabaseBrowserResultScope::StreamingUnknownTotal,
            DatabaseBrowserResultScope::ImportedSnapshot,
        ] {
            if !result_scopes.contains(&required_scope) {
                violations.push(DatabaseBrowserQualificationViolation::MissingResultScope {
                    result_scope: required_scope,
                });
            }
        }

        let redaction_modes: BTreeSet<_> = self
            .target_context_envelopes
            .iter()
            .map(|row| row.redaction_mode)
            .collect();
        for required_mode in [
            DatabaseBrowserRedactionMode::MetadataOnly,
            DatabaseBrowserRedactionMode::RedactedTyped,
            DatabaseBrowserRedactionMode::ReviewedVisibleValues,
            DatabaseBrowserRedactionMode::PolicyBlocked,
        ] {
            if !redaction_modes.contains(&required_mode) {
                violations.push(
                    DatabaseBrowserQualificationViolation::MissingRedactionMode {
                        redaction_mode: required_mode,
                    },
                );
            }
        }

        for row in &self.target_context_envelopes {
            if row.target_endpoint_ref.is_empty() || !row.visible_before_send || !row.visible_in_ui
            {
                violations.push(
                    DatabaseBrowserQualificationViolation::IncompleteTargetContextEnvelopeProjection {
                        envelope_id: row.envelope_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DatabaseBrowserQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in database-browser qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_database_browser_qualification(
) -> Result<DatabaseBrowserQualificationPacket, serde_json::Error> {
    serde_json::from_str(DATABASE_BROWSER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBrowserQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Connection browser rows.
    ConnectionBrowser,
    /// Schema tree rows.
    SchemaTree,
    /// Target-context envelope rows.
    TargetContextEnvelope,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<DatabaseBrowserQualificationViolation>,
    kind: DatabaseBrowserQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(DatabaseBrowserQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for database-browser qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseBrowserQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: DatabaseBrowserQualificationViolationKind,
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
        connection_class: DatabaseBrowserConnectionClass,
    },
    /// Imported snapshot origin coverage is missing.
    MissingImportedSnapshotOrigin,
    /// Required auth source mode is missing.
    MissingAuthSourceMode {
        auth_source_mode: DatabaseBrowserAuthSourceMode,
    },
    /// Required write posture is missing.
    MissingWritePosture {
        write_posture: DatabaseBrowserWritePosture,
    },
    /// Required statement-safety class is missing.
    MissingStatementSafetyClass {
        statement_safety_class: DatabaseBrowserStatementSafetyClass,
    },
    /// Required transaction posture is missing.
    MissingTransactionPosture {
        transaction_posture: DatabaseBrowserTransactionPosture,
    },
    /// Required result scope is missing.
    MissingResultScope {
        result_scope: DatabaseBrowserResultScope,
    },
    /// Required redaction mode is missing.
    MissingRedactionMode {
        redaction_mode: DatabaseBrowserRedactionMode,
    },
    /// Connection browser row does not project target truth everywhere.
    IncompleteConnectionBrowserProjection { browser_id: String },
    /// Schema tree row does not project tree truth everywhere.
    IncompleteSchemaTreeProjection { tree_id: String },
    /// Schema tree may masquerade stale schema as live truth.
    SchemaTreeMayMasqueradeAsLive { tree_id: String },
    /// Target-context envelope row does not project envelope truth everywhere.
    IncompleteTargetContextEnvelopeProjection { envelope_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for DatabaseBrowserQualificationViolation {
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
            Self::MissingImportedSnapshotOrigin => {
                write!(f, "imported snapshot origin is not covered")
            }
            Self::MissingAuthSourceMode { auth_source_mode } => {
                write!(f, "auth source mode {auth_source_mode:?} is not covered")
            }
            Self::MissingWritePosture { write_posture } => {
                write!(f, "write posture {write_posture:?} is not covered")
            }
            Self::MissingStatementSafetyClass {
                statement_safety_class,
            } => {
                write!(
                    f,
                    "statement safety class {statement_safety_class:?} is not covered"
                )
            }
            Self::MissingTransactionPosture {
                transaction_posture,
            } => {
                write!(
                    f,
                    "transaction posture {transaction_posture:?} is not covered"
                )
            }
            Self::MissingResultScope { result_scope } => {
                write!(f, "result scope {result_scope:?} is not covered")
            }
            Self::MissingRedactionMode { redaction_mode } => {
                write!(f, "redaction mode {redaction_mode:?} is not covered")
            }
            Self::IncompleteConnectionBrowserProjection { browser_id } => {
                write!(
                    f,
                    "{browser_id} does not project connection browser truth everywhere"
                )
            }
            Self::IncompleteSchemaTreeProjection { tree_id } => {
                write!(f, "{tree_id} does not project schema tree truth everywhere")
            }
            Self::SchemaTreeMayMasqueradeAsLive { tree_id } => {
                write!(f, "{tree_id} may masquerade stale schema as live truth")
            }
            Self::IncompleteTargetContextEnvelopeProjection { envelope_id } => {
                write!(
                    f,
                    "{envelope_id} does not project target-context envelope truth everywhere"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for DatabaseBrowserQualificationViolation {}
