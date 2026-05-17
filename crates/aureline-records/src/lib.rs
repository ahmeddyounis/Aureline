//! Typed access to the alpha record-class registry.
//!
//! This crate loads the checked-in record-class registry and validates
//! producer `record_kind` constants against registered lifecycle classes.
//! Runtime producers call [`validate`] before emitting a record so support,
//! export, delete, and hold semantics stay tied to the governed registry.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// Repo-relative path to the alpha record-class registry consumed by this crate.
pub const RECORD_CLASS_REGISTRY_ALPHA_PATH: &str =
    "artifacts/governance/record_class_registry_alpha.yaml";

/// Embedded alpha record-class registry contents.
pub const RECORD_CLASS_REGISTRY_ALPHA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/record_class_registry_alpha.yaml"
));

/// Supported registry schema version.
pub const RECORD_CLASS_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Stable record-class identifiers registered in the alpha registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordClassId {
    /// Durable local workspace and restore state.
    DurableWorkspaceState,
    /// User-controlled portable state packages.
    PortableStatePackage,
    /// Local or exported support-bundle archive records.
    SupportBundleArchive,
    /// Managed-copy reference index entries.
    ManagedCopyIndexEntry,
    /// Managed workspace lifecycle copy records.
    ManagedWorkspaceLifecycleCopy,
    /// Entitlement and usage export packets.
    EntitlementUsageExportPacket,
    /// Offboarding and access-end export packets.
    OffboardingExitPacket,
    /// Durable destruction receipt records.
    DestructionReceiptRecord,
    /// AI evidence packets retained under managed policy.
    AiRetainedEvidencePacket,
}

impl RecordClassId {
    /// Returns the registry token for this record class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableWorkspaceState => "durable_workspace_state",
            Self::PortableStatePackage => "portable_state_package",
            Self::SupportBundleArchive => "support_bundle_archive",
            Self::ManagedCopyIndexEntry => "managed_copy_index_entry",
            Self::ManagedWorkspaceLifecycleCopy => "managed_workspace_lifecycle_copy",
            Self::EntitlementUsageExportPacket => "entitlement_usage_export_packet",
            Self::OffboardingExitPacket => "offboarding_exit_packet",
            Self::DestructionReceiptRecord => "destruction_receipt_record",
            Self::AiRetainedEvidencePacket => "ai_retained_evidence_packet",
        }
    }

    /// Parses a record-class id token.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "durable_workspace_state" => Some(Self::DurableWorkspaceState),
            "portable_state_package" => Some(Self::PortableStatePackage),
            "support_bundle_archive" => Some(Self::SupportBundleArchive),
            "managed_copy_index_entry" => Some(Self::ManagedCopyIndexEntry),
            "managed_workspace_lifecycle_copy" => Some(Self::ManagedWorkspaceLifecycleCopy),
            "entitlement_usage_export_packet" => Some(Self::EntitlementUsageExportPacket),
            "offboarding_exit_packet" => Some(Self::OffboardingExitPacket),
            "destruction_receipt_record" => Some(Self::DestructionReceiptRecord),
            "ai_retained_evidence_packet" => Some(Self::AiRetainedEvidencePacket),
            _ => None,
        }
    }
}

impl std::fmt::Display for RecordClassId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Local authority and managed-copy relationship for a record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalVsManagedCopy {
    /// The local copy is authoritative.
    LocalAuthoritative,
    /// The generated packet is authoritative output.
    GeneratedPacketAuthoritative,
    /// The managed service copy is authoritative.
    ManagedAuthoritative,
    /// Local and managed copies both carry distinct authority.
    MixedLocalManaged,
    /// The local copy is only a cache or preview.
    LocalCacheOnly,
}

/// Managed-copy posture declared for a record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedCopyPosture {
    /// Managed copies are forbidden for this class.
    Forbidden,
    /// Managed copies require explicit opt-in.
    OptionalOptIn,
    /// Managed copies exist when the feature is managed.
    RequiredWhenManaged,
    /// The managed copy is authoritative.
    ManagedAuthoritative,
}

/// Whether a record class may participate in hold semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HoldEligibility {
    /// The class may be held.
    Eligible,
    /// The class cannot be held.
    Ineligible,
}

impl HoldEligibility {
    /// Returns `true` when holds may apply to this record class.
    pub const fn as_bool(self) -> bool {
        matches!(self, Self::Eligible)
    }
}

impl From<bool> for HoldEligibility {
    fn from(value: bool) -> Self {
        if value {
            Self::Eligible
        } else {
            Self::Ineligible
        }
    }
}

impl Serialize for HoldEligibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.as_bool())
    }
}

impl<'de> Deserialize<'de> for HoldEligibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(bool::deserialize(deserializer)?.into())
    }
}

/// Export and delete semantic classes used by the alpha registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportDeleteSemantics {
    /// The class cannot be exported.
    NotExportable,
    /// The class is exportable when requested.
    ExportableOnRequest,
    /// A manifest is required for the class.
    ManifestRequired,
    /// The packet itself is the export.
    PacketIsExport,
    /// A receipt is emitted by an action.
    ReceiptEmittedOnAction,
    /// Only inventory metadata is exported.
    InventoryOnly,
    /// Only the local materialization is deleted.
    LocalDeleteOnly,
    /// Local and managed delete actions are distinct.
    DistinctLocalManagedDelete,
    /// Managed delete emits a receipt.
    ManagedDeleteWithReceipt,
    /// Holds can block terminal completion.
    HoldBlocksCompletion,
    /// The receipt itself is retained metadata.
    NotDeletableReceipt,
    /// Local state is invalidated rather than physically deleted.
    LocalInvalidateOnly,
}

/// Scope family for a record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordClassScope {
    /// Durable state record class.
    DurableState,
    /// Support-bundle record class.
    SupportBundle,
    /// Portable-package record class.
    PortablePackage,
    /// Managed-copy record class.
    ManagedCopy,
    /// Export-packet record class.
    ExportPacket,
    /// Receipt record class.
    Receipt,
}

/// Retention label declared for a record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionLabel {
    /// Local user owns the class until it is cleared.
    LocalUserOwnedUntilCleared,
    /// Local user owns the class until export or delete.
    LocalUserOwnedUntilExportOrDelete,
    /// Support-case retention governs the class.
    SupportCaseRetention,
    /// Managed policy governs retention.
    ManagedPolicyRetained,
    /// Generated packet is retained only for delivery.
    GeneratedPacketDeliveryWindow,
    /// Receipt metadata is retained.
    ReceiptMetadataRetained,
}

/// Parsed alpha record-class registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordClassRegistry {
    /// Registry schema version.
    pub schema_version: u32,
    /// Effective date for the registry.
    pub as_of: String,
    /// Registry owner.
    pub owner: String,
    /// Stable registry id.
    pub registry_id: String,
    /// Human entrypoint for reviewer context.
    pub human_entrypoint_ref: String,
    /// Schema registry paired with this record registry.
    pub schema_registry_ref: String,
    /// Base record-class registry this alpha narrows.
    pub base_record_class_registry_ref: String,
    /// Alpha wedge matrix reference.
    pub alpha_wedge_matrix_ref: String,
    /// Alpha exit scoreboard reference.
    pub alpha_exit_scoreboard_ref: String,
    /// Validator command metadata.
    pub validator: ValidatorRef,
    /// Source registry references.
    pub source_registry_refs: Vec<String>,
    /// Declared class-scope vocabulary.
    pub class_scope_vocabulary: Vec<RecordClassScope>,
    /// Declared authority-class vocabulary.
    pub authority_class_vocabulary: Vec<LocalVsManagedCopy>,
    /// Declared managed-copy posture vocabulary.
    pub managed_copy_posture_vocabulary: Vec<ManagedCopyPosture>,
    /// Declared retention-label vocabulary.
    pub retention_label_vocabulary: Vec<RetentionLabel>,
    /// Declared delete semantic vocabulary.
    pub delete_semantic_vocabulary: Vec<ExportDeleteSemantics>,
    /// Declared export semantic vocabulary.
    pub export_semantic_vocabulary: Vec<ExportDeleteSemantics>,
    /// First consumers of this registry.
    pub first_consumers: Vec<FirstConsumer>,
    /// Registered record classes.
    pub record_classes: Vec<RecordClassRow>,
}

impl RecordClassRegistry {
    /// Returns the row for `record_class_id`.
    pub fn row(&self, record_class_id: RecordClassId) -> Option<&RecordClassRow> {
        self.record_classes
            .iter()
            .find(|row| row.record_class_id == record_class_id)
    }

    /// Returns true when `record_class_id` exists in the registry.
    pub fn contains_class(&self, record_class_id: RecordClassId) -> bool {
        self.row(record_class_id).is_some()
    }

    fn validate_shape(&self) -> Result<(), RecordRegistryError> {
        if self.schema_version != RECORD_CLASS_REGISTRY_SCHEMA_VERSION {
            return Err(RecordRegistryError::UnsupportedSchemaVersion {
                expected: RECORD_CLASS_REGISTRY_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        let mut class_ids = BTreeSet::new();
        for row in &self.record_classes {
            if !class_ids.insert(row.record_class_id) {
                return Err(RecordRegistryError::DuplicateRecordClass {
                    record_class_id: row.record_class_id.as_str().to_owned(),
                });
            }
        }
        Ok(())
    }
}

/// Validator command metadata from the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatorRef {
    /// Repo-relative validator script.
    pub script_ref: String,
    /// Default validator command.
    pub command: String,
    /// Default support/export projection command.
    pub consumer_projection_command: String,
}

/// First-consumer row declared by the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FirstConsumer {
    /// Stable consumer id.
    pub consumer_id: String,
    /// Surface class consuming the projection.
    pub surface_class: String,
    /// Repo reference to the consumer.
    pub consumer_ref: String,
    /// Command that runs the consumer projection.
    pub command: String,
    /// Fields the consumer requires.
    pub required_rendered_fields: Vec<String>,
    /// Text contract for the output.
    pub output_contract: String,
}

/// One typed row from the alpha record-class registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordClassRow {
    /// Row discriminator.
    pub row_kind: String,
    /// Row-shape version.
    pub row_version: u32,
    /// Stable alpha row id.
    pub row_id: String,
    /// Stable record-class id.
    pub record_class_id: RecordClassId,
    /// Human-readable title.
    pub title: String,
    /// Scope family for this row.
    pub class_scope: RecordClassScope,
    /// Owner for this row.
    pub owner_dri: String,
    /// Base registry class references.
    pub base_record_class_refs: Vec<String>,
    /// True when this alpha class is a placeholder.
    pub placeholder_record_class: bool,
    /// Criteria for replacing a placeholder class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_exit_criteria: Option<String>,
    /// Schema rows governed by this record class.
    pub schema_row_refs: Vec<String>,
    /// Local authority and managed-copy posture.
    pub local_truth: LocalTruth,
    /// Retention defaults.
    pub retention: RetentionPosture,
    /// Hold semantics.
    pub hold_semantics: HoldSemantics,
    /// Delete semantics.
    pub delete_semantics: DeleteSemantics,
    /// Export semantics.
    pub export_semantics: ExportSemantics,
    /// Support-surface references.
    pub support_surface_refs: Vec<String>,
    /// Product-surface references.
    pub product_surface_refs: Vec<String>,
    /// Documentation references.
    pub docs_refs: Vec<String>,
}

/// Local authority and managed-copy fields on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalTruth {
    /// Authority relation between local and managed copies.
    pub authority_class: LocalVsManagedCopy,
    /// Local materialization posture token.
    pub local_materialization: String,
    /// Managed-copy posture.
    pub managed_copy_posture: ManagedCopyPosture,
    /// Reviewer-facing managed-copy label.
    pub managed_copy_label: String,
}

/// Retention posture for a record class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPosture {
    /// Retention label for the row.
    pub retention_label: RetentionLabel,
    /// Local owner reference.
    pub local_owner_ref: String,
    /// Managed owner reference.
    pub managed_owner_ref: String,
    /// Trigger class for retention changes.
    pub trigger_class: String,
    /// Retention artifact references.
    pub retention_artifact_refs: Vec<String>,
}

/// Hold posture for a record class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HoldSemantics {
    /// Whether this class may be held.
    pub eligible: HoldEligibility,
    /// Hold classes that may apply.
    pub hold_classes: Vec<String>,
}

/// Delete posture for a record class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteSemantics {
    /// Whether delete requests are supported.
    pub request_supported: bool,
    /// Whether local and managed delete actions are distinct.
    pub local_and_managed_actions_are_distinct: bool,
    /// Whether holds block terminal completion.
    pub hold_blocks_completion: bool,
    /// Evidence proving completion.
    pub completion_evidence: String,
    /// Delete semantic classes.
    pub semantic_classes: Vec<ExportDeleteSemantics>,
    /// Blockers that may prevent completion.
    pub blocker_classes: Vec<String>,
}

/// Export posture for a record class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportSemantics {
    /// Export availability token.
    pub availability: String,
    /// Default export formats.
    pub default_formats: Vec<String>,
    /// Whether an export manifest is required.
    pub manifest_required: bool,
    /// Export semantic classes.
    pub semantic_classes: Vec<ExportDeleteSemantics>,
    /// Whether local export copies are disclosed.
    pub local_export_copy_disclosed: bool,
}

/// A known producer record-kind binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordKindBinding {
    /// Stable record-kind token emitted by a producer.
    pub record_kind: &'static str,
    /// Registry class that governs the record kind.
    pub record_class_id: RecordClassId,
}

/// A record-kind binding proven against the current registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedRecordKind {
    /// Stable record-kind token.
    pub record_kind: String,
    /// Registered record class governing the token.
    pub record_class_id: RecordClassId,
    /// Alpha registry row id for the class.
    pub row_id: String,
}

/// Errors returned by registry loading and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordRegistryError {
    /// The embedded YAML could not be parsed.
    Yaml {
        /// Parser error message.
        message: String,
    },
    /// The registry schema version is not supported by this crate.
    UnsupportedSchemaVersion {
        /// Supported schema version.
        expected: u32,
        /// Version found in the registry.
        actual: u32,
    },
    /// The registry declares the same record class more than once.
    DuplicateRecordClass {
        /// Duplicate record-class id.
        record_class_id: String,
    },
    /// The producer binding table declares the same record kind more than once.
    DuplicateRecordKindBinding {
        /// Duplicate record-kind token.
        record_kind: String,
    },
    /// The supplied record kind was empty.
    EmptyRecordKind,
    /// The supplied record class was empty.
    EmptyRecordClass,
    /// The supplied record class is not registered.
    UnknownRecordClass {
        /// Unknown record-class id.
        record_class_id: String,
    },
    /// The supplied record kind is not in the producer binding table.
    UnknownRecordKind {
        /// Unknown record-kind token.
        record_kind: String,
    },
    /// The supplied record kind is bound to a different class.
    RecordKindClassMismatch {
        /// Record-kind token being validated.
        record_kind: String,
        /// Class registered for the record kind.
        expected_record_class_id: RecordClassId,
        /// Class supplied by the caller.
        actual_record_class_id: RecordClassId,
    },
}

impl std::fmt::Display for RecordRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml { message } => write!(f, "record registry YAML parse failed: {message}"),
            Self::UnsupportedSchemaVersion { expected, actual } => write!(
                f,
                "record registry schema_version must be {expected}, got {actual}"
            ),
            Self::DuplicateRecordClass { record_class_id } => {
                write!(f, "duplicate record class in registry: {record_class_id}")
            }
            Self::DuplicateRecordKindBinding { record_kind } => {
                write!(f, "duplicate record-kind binding: {record_kind}")
            }
            Self::EmptyRecordKind => write!(f, "record_kind must not be empty"),
            Self::EmptyRecordClass => write!(f, "record_class_id must not be empty"),
            Self::UnknownRecordClass { record_class_id } => {
                write!(f, "unknown record class: {record_class_id}")
            }
            Self::UnknownRecordKind { record_kind } => {
                write!(f, "unknown record kind: {record_kind}")
            }
            Self::RecordKindClassMismatch {
                record_kind,
                expected_record_class_id,
                actual_record_class_id,
            } => write!(
                f,
                "record kind {record_kind} is registered as {expected_record_class_id}, not {actual_record_class_id}"
            ),
        }
    }
}

impl std::error::Error for RecordRegistryError {}

/// Producer record kinds currently validated against the alpha registry.
pub const PRODUCER_RECORD_KIND_BINDINGS: &[RecordKindBinding] = &[
    RecordKindBinding {
        record_kind: "mutation_journal_entry",
        record_class_id: RecordClassId::DurableWorkspaceState,
    },
    RecordKindBinding {
        record_kind: "mutation_group_record",
        record_class_id: RecordClassId::DurableWorkspaceState,
    },
    RecordKindBinding {
        record_kind: "support_bundle_manifest_record",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
    RecordKindBinding {
        record_kind: "support_bundle_preview_item_record",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
    RecordKindBinding {
        record_kind: "support_bundle_preview_record",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
    RecordKindBinding {
        record_kind: "support_bundle_diagnosis_latency_scorecard_projection",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
    RecordKindBinding {
        record_kind: "records_governance_packet_record",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
    RecordKindBinding {
        record_kind: "support_destruction_receipt_record",
        record_class_id: RecordClassId::DestructionReceiptRecord,
    },
    RecordKindBinding {
        record_kind: "evidence_timeline_packet_record",
        record_class_id: RecordClassId::SupportBundleArchive,
    },
];

static CURRENT_REGISTRY: OnceLock<Result<RecordClassRegistry, RecordRegistryError>> =
    OnceLock::new();

/// Parses an alpha record-class registry from YAML.
pub fn parse_registry(yaml: &str) -> Result<RecordClassRegistry, RecordRegistryError> {
    let registry: RecordClassRegistry =
        serde_yaml::from_str(yaml).map_err(|err| RecordRegistryError::Yaml {
            message: err.to_string(),
        })?;
    registry.validate_shape()?;
    Ok(registry)
}

/// Loads the embedded alpha record-class registry.
pub fn load_alpha_registry() -> Result<RecordClassRegistry, RecordRegistryError> {
    parse_registry(RECORD_CLASS_REGISTRY_ALPHA_YAML)
}

/// Returns the embedded alpha record-class registry, parsed once per process.
pub fn current_registry() -> Result<&'static RecordClassRegistry, RecordRegistryError> {
    match CURRENT_REGISTRY.get_or_init(load_alpha_registry) {
        Ok(registry) => Ok(registry),
        Err(err) => Err(err.clone()),
    }
}

/// Validates that `record_kind` belongs to a registered record class.
pub fn validate(
    record_kind: &str,
    record_class_id: impl AsRef<str>,
) -> Result<ValidatedRecordKind, RecordRegistryError> {
    let record_kind = record_kind.trim();
    if record_kind.is_empty() {
        return Err(RecordRegistryError::EmptyRecordKind);
    }
    let record_class_id = record_class_id.as_ref().trim();
    if record_class_id.is_empty() {
        return Err(RecordRegistryError::EmptyRecordClass);
    }
    let Some(record_class_id) = RecordClassId::parse(record_class_id) else {
        return Err(RecordRegistryError::UnknownRecordClass {
            record_class_id: record_class_id.to_owned(),
        });
    };
    validate_typed(record_kind, record_class_id)
}

/// Validates that `record_kind` belongs to `record_class_id`.
pub fn validate_typed(
    record_kind: &str,
    record_class_id: RecordClassId,
) -> Result<ValidatedRecordKind, RecordRegistryError> {
    let record_kind = record_kind.trim();
    if record_kind.is_empty() {
        return Err(RecordRegistryError::EmptyRecordKind);
    }

    let registry = current_registry()?;
    let Some(row) = registry.row(record_class_id) else {
        return Err(RecordRegistryError::UnknownRecordClass {
            record_class_id: record_class_id.as_str().to_owned(),
        });
    };

    let bindings = producer_bindings_by_kind()?;
    let Some(binding) = bindings.get(record_kind) else {
        return Err(RecordRegistryError::UnknownRecordKind {
            record_kind: record_kind.to_owned(),
        });
    };
    if binding.record_class_id != record_class_id {
        return Err(RecordRegistryError::RecordKindClassMismatch {
            record_kind: record_kind.to_owned(),
            expected_record_class_id: binding.record_class_id,
            actual_record_class_id: record_class_id,
        });
    }

    Ok(ValidatedRecordKind {
        record_kind: record_kind.to_owned(),
        record_class_id,
        row_id: row.row_id.clone(),
    })
}

fn producer_bindings_by_kind(
) -> Result<BTreeMap<&'static str, RecordKindBinding>, RecordRegistryError> {
    let mut by_kind = BTreeMap::new();
    for binding in PRODUCER_RECORD_KIND_BINDINGS {
        if by_kind.insert(binding.record_kind, *binding).is_some() {
            return Err(RecordRegistryError::DuplicateRecordKindBinding {
                record_kind: binding.record_kind.to_owned(),
            });
        }
    }
    Ok(by_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_round_trips_every_registry_row() {
        let registry = load_alpha_registry().expect("registry parses");
        assert_eq!(
            registry.schema_version,
            RECORD_CLASS_REGISTRY_SCHEMA_VERSION
        );
        assert_eq!(registry.record_classes.len(), 9);

        for row in &registry.record_classes {
            let yaml = serde_yaml::to_string(row).expect("row serializes");
            let reparsed: RecordClassRow = serde_yaml::from_str(&yaml).expect("row reparses");
            assert_eq!(reparsed, *row);
            assert!(registry.contains_class(row.record_class_id));
        }
    }

    #[test]
    fn unknown_record_class_is_rejected() {
        let error = validate("support_bundle_manifest_record", "unregistered_class")
            .expect_err("unknown class rejected");
        assert!(matches!(
            error,
            RecordRegistryError::UnknownRecordClass { .. }
        ));
    }

    #[test]
    fn known_record_kind_must_match_registered_class() {
        let validated = validate_typed(
            "support_bundle_manifest_record",
            RecordClassId::SupportBundleArchive,
        )
        .expect("support bundle manifest class is registered");
        assert_eq!(
            validated.record_class_id,
            RecordClassId::SupportBundleArchive
        );

        let error = validate_typed(
            "support_bundle_manifest_record",
            RecordClassId::DurableWorkspaceState,
        )
        .expect_err("mismatched class rejected");
        assert!(matches!(
            error,
            RecordRegistryError::RecordKindClassMismatch { .. }
        ));
    }
}
