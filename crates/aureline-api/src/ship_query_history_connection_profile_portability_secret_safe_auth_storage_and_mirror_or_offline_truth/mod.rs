//! Query history, connection-profile portability, secret-safe auth storage, and
//! mirror or offline truth qualification records.
//!
//! This module owns the typed records that keep query history entries,
//! connection-profile portability, secret-safe auth storage, and mirror or
//! offline truth inspectable and attributable without depending on hidden shell
//! shortcuts or ad hoc scripts. The boundary schema is
//! [`/schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json`](../../../schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`](../../../artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json).
//!
//! Raw statement bodies, raw connection strings, raw secrets, raw credential
//! bodies, raw endpoint URLs, and raw hostnames do not belong in these records.
//! They carry stable IDs, closed posture vocabularies, and reviewable summaries
//! that UI, CLI, export, support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for ship-query-history qualification packets.
pub const SHIP_QUERY_HISTORY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ShipQueryHistoryQualificationPacket`].
pub const SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND: &str =
    "ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth";

/// Repo-relative path to the checked-in ship-query-history qualification packet.
pub const SHIP_QUERY_HISTORY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json";

/// Embedded checked-in packet JSON.
pub const SHIP_QUERY_HISTORY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json"
));

/// Qualification label shown on promoted ship-query-history surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipQueryHistoryQualificationLabel {
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

impl ShipQueryHistoryQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Ship-query-history surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipQueryHistorySurfaceKind {
    /// Query history viewer, list, or lane.
    QueryHistoryViewer,
    /// Connection-profile portability panel for export, import, or migration.
    ConnectionProfilePortability,
    /// Secret-safe auth storage inspector or settings.
    SecretSafeAuthStorage,
    /// Mirror or offline truth indicator, chip, or panel.
    MirrorOrOfflineTruth,
}

/// Retention posture for query-history entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryHistoryRetentionPosture {
    /// Local-first storage with no remote retention.
    LocalFirst,
    /// Bounded by count and age.
    Bounded,
    /// User-pinned until explicit clear.
    Pinned,
    /// Ephemeral, discarded after session.
    Ephemeral,
    /// Audit-only, no user-facing replay.
    AuditOnly,
}

/// Replay drift risk class shown before rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryHistoryReplayDriftRisk {
    /// No drift risk for pure read-only metadata.
    NoDrift,
    /// Low drift risk for idempotent selects.
    LowDrift,
    /// Moderate drift risk due to changing data or non-deterministic functions.
    ModerateDrift,
    /// High drift risk for DML, DDL, or changed context.
    HighDrift,
    /// Replay is blocked by drift or policy.
    Blocked,
}

/// Portability posture for connection-profile export, import, or migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionProfilePortabilityPosture {
    /// Local-only, no portable export.
    LocalOnly,
    /// Redacted export with no raw secrets.
    RedactedExport,
    /// Full migration with broker-handle resolution.
    FullMigration,
    /// Portability blocked by policy.
    Blocked,
}

/// Secret-safe auth storage mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretSafeAuthStorageMode {
    /// Local encrypted store.
    LocalEncrypted,
    /// Secret broker only, no local raw storage.
    SecretBrokerOnly,
    /// Managed rotation with policy-enforced expiry.
    ManagedRotation,
    /// Policy-locked, no user override.
    PolicyLocked,
}

/// Mirror or offline state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOrOfflineStateClass {
    /// Online default, origin admissible.
    OnlineDefault,
    /// Online replica routed, no direct origin.
    OnlineReplica,
    /// Offline grace window using warm cache.
    OfflineGraceWindow,
    /// Offline local file only.
    OfflineLocalOnly,
    /// Network disabled by user setting or policy.
    NetworkDisabled,
}

/// Redaction class for export and support-bundle safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipQueryHistoryRedactionClass {
    /// Metadata is safe by default.
    MetadataSafe,
    /// Operator-only restricted view.
    OperatorOnly,
    /// Internal support restricted view.
    InternalSupport,
    /// Signing evidence only.
    SigningEvidence,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipQueryHistoryQualificationProof {
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
pub struct ShipQueryHistorySurfaceGuardSet {
    /// Query history identity, connection class, and safety class are visible.
    pub query_history_identity_visible: bool,
    /// Connection-profile portability posture is visible.
    pub connection_profile_portability_visible: bool,
    /// Secret-safe auth storage mode and broker handles are visible.
    pub secret_safe_auth_storage_visible: bool,
    /// Mirror or offline truth state is visible.
    pub mirror_or_offline_truth_visible: bool,
    /// Auth scope is visible without raw secrets.
    pub auth_scope_visible: bool,
    /// Write posture and mutation risk are visible.
    pub write_posture_visible: bool,
    /// Retention and redaction posture are visible.
    pub retention_redaction_visible: bool,
    /// Replay drift risk is visible before rerun.
    pub replay_drift_risk_visible: bool,
}

impl ShipQueryHistorySurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.query_history_identity_visible
            && self.connection_profile_portability_visible
            && self.secret_safe_auth_storage_visible
            && self.mirror_or_offline_truth_visible
            && self.auth_scope_visible
            && self.write_posture_visible
            && self.retention_redaction_visible
            && self.replay_drift_risk_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipQueryHistorySurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ShipQueryHistorySurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ShipQueryHistoryQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ShipQueryHistoryQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ShipQueryHistoryQualificationProof>,
    /// Visible guard set.
    pub guards: ShipQueryHistorySurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One query-history entry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryHistoryEntryRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Connection profile ref.
    pub connection_profile_ref: String,
    /// Statement body ref (opaque, non-secret).
    pub statement_body_ref: String,
    /// Statement safety result ref.
    pub statement_safety_result_ref: String,
    /// Result size class label.
    pub result_size_class: String,
    /// Row count truth class label.
    pub row_count_truth_class: String,
    /// Retention posture.
    pub retention_posture: QueryHistoryRetentionPosture,
    /// Replay drift risk.
    pub replay_drift_risk: QueryHistoryReplayDriftRisk,
    /// Whether the entry is safe for export.
    pub export_safe: bool,
    /// Redaction class.
    pub redaction_class: ShipQueryHistoryRedactionClass,
    /// Last executed timestamp.
    pub last_executed_at: String,
    /// Whether the entry is pinned against automatic eviction.
    pub pinned: bool,
}

/// One connection-profile portability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionProfilePortabilityRow {
    /// Stable portability id.
    pub portability_id: String,
    /// Source connection profile ref.
    pub source_profile_ref: String,
    /// Target format label.
    pub target_format: String,
    /// Whether raw secrets are included.
    pub includes_raw_secrets: bool,
    /// Whether raw endpoint details are included.
    pub includes_raw_endpoint: bool,
    /// Whether auth handle migration is visible.
    pub auth_handle_migration_visible: bool,
    /// Export posture.
    pub export_posture: ConnectionProfilePortabilityPosture,
    /// Import posture.
    pub import_posture: ConnectionProfilePortabilityPosture,
}

/// One secret-safe auth storage row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretSafeAuthStorageRow {
    /// Stable storage id.
    pub storage_id: String,
    /// Auth handle class label.
    pub auth_handle_class: String,
    /// Secret broker ref.
    pub secret_broker_ref: String,
    /// Whether encryption at rest is enabled.
    pub encryption_at_rest: bool,
    /// Whether rotation policy is visible.
    pub rotation_policy_visible: bool,
    /// Whether a raw secret was observed in workspace state.
    pub raw_secret_observed: bool,
    /// Storage mode.
    pub storage_mode: SecretSafeAuthStorageMode,
    /// Whether the storage record is safe for export.
    pub export_safe: bool,
}

/// One mirror or offline truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorOrOfflineTruthRow {
    /// Stable truth id.
    pub truth_id: String,
    /// Mirror or offline state class.
    pub mirror_or_offline_state_class: MirrorOrOfflineStateClass,
    /// Replica endpoint ref, if any.
    pub replica_endpoint_ref: Option<String>,
    /// Offline since timestamp, if applicable.
    pub offline_since: Option<String>,
    /// Cache warmth label.
    pub cache_warmth: String,
    /// Fallback posture label.
    pub fallback_posture: String,
    /// Connectivity disclosure sentence.
    pub connectivity_disclosure: String,
}

/// Summary counts for a ship-query-history qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipQueryHistoryQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of query-history entry rows.
    pub query_history_entry_count: usize,
    /// Number of connection-profile portability rows.
    pub connection_profile_portability_count: usize,
    /// Number of secret-safe auth storage rows.
    pub secret_safe_auth_storage_count: usize,
    /// Number of mirror or offline truth rows.
    pub mirror_or_offline_truth_count: usize,
}

/// Canonical ship-query-history qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipQueryHistoryQualificationPacket {
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
    pub surfaces: Vec<ShipQueryHistorySurfaceQualificationRow>,
    /// Query-history entry rows.
    pub query_history_entries: Vec<QueryHistoryEntryRow>,
    /// Connection-profile portability rows.
    pub connection_profile_portabilities: Vec<ConnectionProfilePortabilityRow>,
    /// Secret-safe auth storage rows.
    pub secret_safe_auth_storages: Vec<SecretSafeAuthStorageRow>,
    /// Mirror or offline truth rows.
    pub mirror_or_offline_truths: Vec<MirrorOrOfflineTruthRow>,
    /// Summary counts.
    pub summary: ShipQueryHistoryQualificationSummary,
}

impl ShipQueryHistoryQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ShipQueryHistoryQualificationSummary {
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
        ShipQueryHistoryQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            query_history_entry_count: self.query_history_entries.len(),
            connection_profile_portability_count: self.connection_profile_portabilities.len(),
            secret_safe_auth_storage_count: self.secret_safe_auth_storages.len(),
            mirror_or_offline_truth_count: self.mirror_or_offline_truths.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ShipQueryHistoryQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != SHIP_QUERY_HISTORY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ShipQueryHistoryQualificationViolation::SchemaVersion {
                expected: SHIP_QUERY_HISTORY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND {
            violations.push(ShipQueryHistoryQualificationViolation::RecordKind {
                expected: SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            ShipQueryHistoryQualificationViolationKind::Surface,
        );
        collect_ids(
            self.query_history_entries
                .iter()
                .map(|row| row.entry_id.as_str()),
            &mut violations,
            ShipQueryHistoryQualificationViolationKind::QueryHistoryEntry,
        );
        collect_ids(
            self.connection_profile_portabilities
                .iter()
                .map(|row| row.portability_id.as_str()),
            &mut violations,
            ShipQueryHistoryQualificationViolationKind::ConnectionProfilePortability,
        );
        collect_ids(
            self.secret_safe_auth_storages
                .iter()
                .map(|row| row.storage_id.as_str()),
            &mut violations,
            ShipQueryHistoryQualificationViolationKind::SecretSafeAuthStorage,
        );
        collect_ids(
            self.mirror_or_offline_truths
                .iter()
                .map(|row| row.truth_id.as_str()),
            &mut violations,
            ShipQueryHistoryQualificationViolationKind::MirrorOrOfflineTruth,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        ShipQueryHistoryQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        ShipQueryHistoryQualificationViolation::StableSurfaceMissingGuard {
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
                    ShipQueryHistoryQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let retention_postures: BTreeSet<_> = self
            .query_history_entries
            .iter()
            .map(|row| row.retention_posture)
            .collect();
        for required_posture in [
            QueryHistoryRetentionPosture::LocalFirst,
            QueryHistoryRetentionPosture::Bounded,
            QueryHistoryRetentionPosture::Pinned,
        ] {
            if !retention_postures.contains(&required_posture) {
                violations.push(
                    ShipQueryHistoryQualificationViolation::MissingQueryHistoryRetentionPosture {
                        retention_posture: required_posture,
                    },
                );
            }
        }

        let drift_risks: BTreeSet<_> = self
            .query_history_entries
            .iter()
            .map(|row| row.replay_drift_risk)
            .collect();
        for required_risk in [
            QueryHistoryReplayDriftRisk::NoDrift,
            QueryHistoryReplayDriftRisk::LowDrift,
            QueryHistoryReplayDriftRisk::HighDrift,
            QueryHistoryReplayDriftRisk::Blocked,
        ] {
            if !drift_risks.contains(&required_risk) {
                violations.push(
                    ShipQueryHistoryQualificationViolation::MissingReplayDriftRisk {
                        replay_drift_risk: required_risk,
                    },
                );
            }
        }

        for row in &self.query_history_entries {
            if row.connection_profile_ref.is_empty()
                || row.statement_body_ref.is_empty()
                || row.statement_safety_result_ref.is_empty()
                || row.last_executed_at.is_empty()
            {
                violations.push(
                    ShipQueryHistoryQualificationViolation::IncompleteQueryHistoryEntry {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        let portability_postures: BTreeSet<_> = self
            .connection_profile_portabilities
            .iter()
            .map(|row| row.export_posture)
            .collect();
        for required_posture in [
            ConnectionProfilePortabilityPosture::LocalOnly,
            ConnectionProfilePortabilityPosture::RedactedExport,
            ConnectionProfilePortabilityPosture::Blocked,
        ] {
            if !portability_postures.contains(&required_posture) {
                violations.push(
                    ShipQueryHistoryQualificationViolation::MissingPortabilityPosture {
                        portability_posture: required_posture,
                    },
                );
            }
        }

        for row in &self.connection_profile_portabilities {
            if row.includes_raw_secrets || row.includes_raw_endpoint {
                violations.push(
                    ShipQueryHistoryQualificationViolation::ExportIncludesRawSensitiveData {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }
            if row.source_profile_ref.is_empty() || row.target_format.is_empty() {
                violations.push(
                    ShipQueryHistoryQualificationViolation::IncompleteConnectionProfilePortability {
                        portability_id: row.portability_id.clone(),
                    },
                );
            }
        }

        let storage_modes: BTreeSet<_> = self
            .secret_safe_auth_storages
            .iter()
            .map(|row| row.storage_mode)
            .collect();
        for required_mode in [
            SecretSafeAuthStorageMode::LocalEncrypted,
            SecretSafeAuthStorageMode::SecretBrokerOnly,
            SecretSafeAuthStorageMode::ManagedRotation,
        ] {
            if !storage_modes.contains(&required_mode) {
                violations.push(
                    ShipQueryHistoryQualificationViolation::MissingAuthStorageMode {
                        auth_storage_mode: required_mode,
                    },
                );
            }
        }

        for row in &self.secret_safe_auth_storages {
            if row.raw_secret_observed {
                violations.push(
                    ShipQueryHistoryQualificationViolation::SecretSafeAuthRawSecretObserved {
                        storage_id: row.storage_id.clone(),
                    },
                );
            }
            if row.secret_broker_ref.is_empty() || row.auth_handle_class.is_empty() {
                violations.push(
                    ShipQueryHistoryQualificationViolation::IncompleteSecretSafeAuthStorage {
                        storage_id: row.storage_id.clone(),
                    },
                );
            }
        }

        let offline_states: BTreeSet<_> = self
            .mirror_or_offline_truths
            .iter()
            .map(|row| row.mirror_or_offline_state_class)
            .collect();
        for required_state in [
            MirrorOrOfflineStateClass::OnlineDefault,
            MirrorOrOfflineStateClass::OnlineReplica,
            MirrorOrOfflineStateClass::OfflineGraceWindow,
            MirrorOrOfflineStateClass::OfflineLocalOnly,
        ] {
            if !offline_states.contains(&required_state) {
                violations.push(
                    ShipQueryHistoryQualificationViolation::MissingMirrorOrOfflineState {
                        mirror_or_offline_state: required_state,
                    },
                );
            }
        }

        for row in &self.mirror_or_offline_truths {
            if row.cache_warmth.is_empty()
                || row.fallback_posture.is_empty()
                || row.connectivity_disclosure.is_empty()
            {
                violations.push(
                    ShipQueryHistoryQualificationViolation::IncompleteMirrorOrOfflineTruth {
                        truth_id: row.truth_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ShipQueryHistoryQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in ship-query-history qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_ship_query_history_qualification(
) -> Result<ShipQueryHistoryQualificationPacket, serde_json::Error> {
    serde_json::from_str(SHIP_QUERY_HISTORY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipQueryHistoryQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Query-history entry rows.
    QueryHistoryEntry,
    /// Connection-profile portability rows.
    ConnectionProfilePortability,
    /// Secret-safe auth storage rows.
    SecretSafeAuthStorage,
    /// Mirror or offline truth rows.
    MirrorOrOfflineTruth,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ShipQueryHistoryQualificationViolation>,
    kind: ShipQueryHistoryQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ShipQueryHistoryQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for ship-query-history qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipQueryHistoryQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ShipQueryHistoryQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required query-history retention posture is missing.
    MissingQueryHistoryRetentionPosture {
        retention_posture: QueryHistoryRetentionPosture,
    },
    /// Required replay drift risk is missing.
    MissingReplayDriftRisk {
        replay_drift_risk: QueryHistoryReplayDriftRisk,
    },
    /// Query-history entry lacks required identity refs.
    IncompleteQueryHistoryEntry { entry_id: String },
    /// Required connection-profile portability posture is missing.
    MissingPortabilityPosture {
        portability_posture: ConnectionProfilePortabilityPosture,
    },
    /// Connection-profile portability includes raw secrets or raw endpoint details.
    ExportIncludesRawSensitiveData { portability_id: String },
    /// Connection-profile portability row is incomplete.
    IncompleteConnectionProfilePortability { portability_id: String },
    /// Required secret-safe auth storage mode is missing.
    MissingAuthStorageMode {
        auth_storage_mode: SecretSafeAuthStorageMode,
    },
    /// Secret-safe auth storage row observed a raw secret in workspace state.
    SecretSafeAuthRawSecretObserved { storage_id: String },
    /// Secret-safe auth storage row is incomplete.
    IncompleteSecretSafeAuthStorage { storage_id: String },
    /// Required mirror or offline state is missing.
    MissingMirrorOrOfflineState {
        mirror_or_offline_state: MirrorOrOfflineStateClass,
    },
    /// Mirror or offline truth row is incomplete.
    IncompleteMirrorOrOfflineTruth { truth_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ShipQueryHistoryQualificationViolation {
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
            Self::MissingQueryHistoryRetentionPosture { retention_posture } => {
                write!(
                    f,
                    "query-history retention posture {retention_posture:?} is not covered"
                )
            }
            Self::MissingReplayDriftRisk { replay_drift_risk } => {
                write!(f, "replay drift risk {replay_drift_risk:?} is not covered")
            }
            Self::IncompleteQueryHistoryEntry { entry_id } => {
                write!(
                    f,
                    "{entry_id} does not project query-history truth everywhere"
                )
            }
            Self::MissingPortabilityPosture {
                portability_posture,
            } => {
                write!(
                    f,
                    "connection-profile portability posture {portability_posture:?} is not covered"
                )
            }
            Self::ExportIncludesRawSensitiveData { portability_id } => {
                write!(
                    f,
                    "{portability_id} includes raw secrets or raw endpoint details"
                )
            }
            Self::IncompleteConnectionProfilePortability { portability_id } => {
                write!(
                    f,
                    "{portability_id} does not project portability truth everywhere"
                )
            }
            Self::MissingAuthStorageMode { auth_storage_mode } => {
                write!(
                    f,
                    "secret-safe auth storage mode {auth_storage_mode:?} is not covered"
                )
            }
            Self::SecretSafeAuthRawSecretObserved { storage_id } => {
                write!(f, "{storage_id} observed a raw secret in workspace state")
            }
            Self::IncompleteSecretSafeAuthStorage { storage_id } => {
                write!(
                    f,
                    "{storage_id} does not project auth-storage truth everywhere"
                )
            }
            Self::MissingMirrorOrOfflineState {
                mirror_or_offline_state,
            } => {
                write!(
                    f,
                    "mirror or offline state {mirror_or_offline_state:?} is not covered"
                )
            }
            Self::IncompleteMirrorOrOfflineTruth { truth_id } => {
                write!(
                    f,
                    "{truth_id} does not project mirror/offline truth everywhere"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ShipQueryHistoryQualificationViolation {}
