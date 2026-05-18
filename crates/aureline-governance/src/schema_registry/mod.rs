//! Schema-family and record-class governance registries.
//!
//! The module embeds the JSON registries under `schemas/registry/` and exposes
//! a typed validation and projection surface for product inspectors. Consumers
//! can resolve a schema row by id, classify packet-version compatibility, and
//! render the same consent, endpoint, lifecycle, and record-class metadata for
//! settings, Help/About, Support Center, admin/export, release packets, and
//! CLI/headless output.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Repo-relative path to the governed schema-family registry.
pub const GOVERNED_SCHEMA_REGISTRY_PATH: &str = "schemas/registry/schema_registry.json";

/// Embedded governed schema-family registry JSON.
pub const GOVERNED_SCHEMA_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/registry/schema_registry.json"
));

/// Repo-relative path to the governed record-class registry.
pub const GOVERNED_RECORD_CLASS_REGISTRY_PATH: &str = "schemas/registry/record_class_registry.json";

/// Embedded governed record-class registry JSON.
pub const GOVERNED_RECORD_CLASS_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/registry/record_class_registry.json"
));

/// Supported JSON registry schema version.
pub const GOVERNED_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Product surfaces that must expose governed schema metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceSurfaceClass {
    /// Settings and privacy-related configuration inspectors.
    Settings,
    /// Help/About and product self-description surfaces.
    HelpAbout,
    /// Support Center and support-bundle preview surfaces.
    SupportCenter,
    /// Admin export, audit export, and access/export views.
    AdminExport,
    /// Release packets and release-evidence bundles.
    ReleasePacket,
    /// CLI/headless machine-readable output.
    CliHeadless,
}

impl GovernanceSurfaceClass {
    /// Returns the stable registry token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::HelpAbout => "help_about",
            Self::SupportCenter => "support_center",
            Self::AdminExport => "admin_export",
            Self::ReleasePacket => "release_packet",
            Self::CliHeadless => "cli_headless",
        }
    }

    /// Returns every required surface class.
    pub const fn all() -> [Self; 6] {
        [
            Self::Settings,
            Self::HelpAbout,
            Self::SupportCenter,
            Self::AdminExport,
            Self::ReleasePacket,
            Self::CliHeadless,
        ]
    }
}

/// Visible support state for a packet version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketVersionSupport {
    /// The packet version exactly matches the registered schema version.
    Supported,
    /// The packet is a near-future or otherwise unknown version requiring review.
    Limited,
    /// The packet is older than the registered version but still readable.
    Deprecated,
    /// The packet is outside the declared readable window.
    Unsupported,
}

impl PacketVersionSupport {
    /// Stable token shown to downstream inspectors.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Deprecated => "deprecated",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Governed schema-family registry envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedSchemaRegistry {
    /// Registry schema version.
    pub schema_version: u32,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Lifecycle status of this registry artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Upstream seed registries this JSON artifact quotes.
    pub source_registries: Vec<String>,
    /// Family classes that must have at least one row.
    pub required_family_class_coverage: Vec<String>,
    /// Product surfaces that must render every row's schema metadata.
    pub required_surface_visibility: Vec<String>,
    /// Required default posture for telemetry in open builds.
    pub open_source_telemetry_default: String,
    /// Governed schema-family rows.
    pub rows: Vec<GovernedSchemaRow>,
}

impl GovernedSchemaRegistry {
    /// Returns the row registered for `schema_id`.
    pub fn row(&self, schema_id: &str) -> Option<&GovernedSchemaRow> {
        self.rows.iter().find(|row| row.schema_id == schema_id)
    }

    /// Returns the rows visible on `surface`.
    pub fn rows_for_surface(&self, surface: GovernanceSurfaceClass) -> Vec<&GovernedSchemaRow> {
        let surface = surface.as_str();
        self.rows
            .iter()
            .filter(|row| row.surface_visibility.iter().any(|token| token == surface))
            .collect()
    }

    /// Builds a surface projection with the metadata every product inspector shows.
    pub fn surface_projection(&self, surface: GovernanceSurfaceClass) -> SurfaceProjection {
        let rows = self
            .rows_for_surface(surface)
            .into_iter()
            .map(SurfaceSchemaRow::from)
            .collect();
        SurfaceProjection {
            surface_class: surface,
            schema_registry_ref: GOVERNED_SCHEMA_REGISTRY_PATH.to_owned(),
            rows,
        }
    }
}

/// One governed schema-family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedSchemaRow {
    /// Stable schema-family id.
    pub schema_id: String,
    /// Human-readable title.
    pub title: String,
    /// Payload family class.
    pub family_class: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Repo-relative schema file.
    pub schema_ref: String,
    /// Current schema version.
    pub schema_version: u32,
    /// Stable schema version URI.
    pub schema_version_uri: String,
    /// Reviewable reason this version is registered or changed.
    pub version_change_rationale: String,
    /// Consent posture.
    pub consent_class: String,
    /// Endpoint or destination posture.
    pub endpoint_class: String,
    /// Record classes governing retention, export, delete, and hold semantics.
    pub record_class_id_refs: Vec<String>,
    /// Reviewable retention posture.
    pub retention_posture: String,
    /// Lifecycle state rendered by product and packet surfaces.
    pub lifecycle_state: String,
    /// Default posture for open or local builds.
    pub open_source_default_posture: String,
    /// Downgrade behavior for older or unknown versions.
    pub downgrade_rule: DowngradeRule,
    /// Visible support labels for packet-version handling.
    pub version_support_labels: Vec<String>,
    /// Product surfaces that render this row.
    pub surface_visibility: Vec<String>,
    /// Documentation references for reviewers.
    pub docs_refs: Vec<String>,
    /// Rule keeping sibling payload families separate.
    pub separation: SeparationRule,
}

impl GovernedSchemaRow {
    /// Classifies an observed packet version against this row.
    pub fn classify_packet_version(&self, packet_version: u32) -> PacketVersionSupport {
        if packet_version == self.schema_version {
            PacketVersionSupport::Supported
        } else if packet_version < self.downgrade_rule.min_readable_version {
            PacketVersionSupport::Unsupported
        } else if packet_version < self.schema_version {
            PacketVersionSupport::Deprecated
        } else if packet_version == self.schema_version.saturating_add(1) {
            PacketVersionSupport::Limited
        } else {
            PacketVersionSupport::Unsupported
        }
    }
}

/// Downgrade and compatibility behavior for one schema row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeRule {
    /// Oldest packet version readers may inspect.
    pub min_readable_version: u32,
    /// Oldest packet version writers may emit for compatibility output.
    pub min_writable_version: u32,
    /// Visible policy for unknown packet versions.
    pub unknown_version_policy: String,
    /// Visible policy for deprecated packet versions.
    pub deprecated_version_policy: String,
}

/// Family-separation rule for a schema row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeparationRule {
    /// Family this row is canonical for.
    pub canonical_family_class: String,
    /// Sibling families that must not be collapsed into this row.
    pub must_not_conflate_with: Vec<String>,
    /// Reviewable explanation of the separation.
    pub note: String,
}

/// Governed record-class registry envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRecordClassRegistry {
    /// Registry schema version.
    pub schema_version: u32,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Lifecycle status of this registry artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Upstream registry and document references this artifact quotes.
    pub source_registries: Vec<String>,
    /// Governed record-class rows.
    pub rows: Vec<GovernedRecordClassRow>,
}

impl GovernedRecordClassRegistry {
    /// Returns the row registered for `record_class_id`.
    pub fn row(&self, record_class_id: &str) -> Option<&GovernedRecordClassRow> {
        self.rows
            .iter()
            .find(|row| row.record_class_id == record_class_id)
    }

    /// Returns true when `record_class_id` is registered.
    pub fn contains_class(&self, record_class_id: &str) -> bool {
        self.row(record_class_id).is_some()
    }
}

/// One governed record-class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRecordClassRow {
    /// Stable record-class id.
    pub record_class_id: String,
    /// Human-readable title.
    pub title: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Local versus managed copy truth.
    pub local_vs_managed_truth: String,
    /// Export behavior.
    pub export_semantics: String,
    /// Delete behavior.
    pub delete_semantics: String,
    /// Hold behavior.
    pub hold_semantics: String,
    /// Redaction posture.
    pub redaction_posture: String,
    /// Retention posture.
    pub retention_posture: String,
    /// Role in access-end or offboarding packages.
    pub offboarding_posture: String,
    /// Upstream governance references.
    pub governance_refs: Vec<String>,
}

/// Surface projection row exposing shared schema metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceSchemaRow {
    /// Stable schema-family id.
    pub schema_id: String,
    /// Current schema version.
    pub schema_version: u32,
    /// Lifecycle state.
    pub lifecycle_state: String,
    /// Consent posture.
    pub consent_class: String,
    /// Endpoint posture.
    pub endpoint_class: String,
    /// Record classes governing this schema.
    pub record_class_id_refs: Vec<String>,
}

impl From<&GovernedSchemaRow> for SurfaceSchemaRow {
    fn from(row: &GovernedSchemaRow) -> Self {
        Self {
            schema_id: row.schema_id.clone(),
            schema_version: row.schema_version,
            lifecycle_state: row.lifecycle_state.clone(),
            consent_class: row.consent_class.clone(),
            endpoint_class: row.endpoint_class.clone(),
            record_class_id_refs: row.record_class_id_refs.clone(),
        }
    }
}

/// Projection consumed by one product or packet surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjection {
    /// Surface this projection targets.
    pub surface_class: GovernanceSurfaceClass,
    /// Registry artifact that owns the projected truth.
    pub schema_registry_ref: String,
    /// Rows visible on this surface.
    pub rows: Vec<SurfaceSchemaRow>,
}

/// Successful validation summary for the default registries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaRegistryValidationReport {
    /// Number of schema rows validated.
    pub schema_row_count: usize,
    /// Number of record-class rows validated.
    pub record_class_row_count: usize,
    /// Required family classes covered by the schema registry.
    pub covered_family_classes: Vec<String>,
    /// Required surfaces covered by every schema row.
    pub required_surface_visibility: Vec<String>,
}

/// Errors returned while loading or validating governed registries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaRegistryError {
    /// JSON parsing failed.
    Json {
        /// Source artifact that failed to parse.
        source_ref: String,
        /// Parser error message.
        message: String,
    },
    /// A registry artifact has an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Source artifact containing the version.
        source_ref: String,
        /// Supported version.
        expected: u32,
        /// Version found in the artifact.
        actual: u32,
    },
    /// A schema id or record-class id appears more than once.
    DuplicateId {
        /// Duplicate id.
        id: String,
    },
    /// A schema row references a record class missing from the registry.
    UnknownRecordClass {
        /// Schema row id.
        schema_id: String,
        /// Missing record-class id.
        record_class_id: String,
    },
    /// A required payload family has no schema row.
    MissingFamilyCoverage {
        /// Missing family class.
        family_class: String,
    },
    /// A schema row omits a required surface.
    MissingSurfaceVisibility {
        /// Schema row id.
        schema_id: String,
        /// Missing surface token.
        surface: String,
    },
    /// A telemetry row does not keep open builds opt-in.
    InvalidTelemetryDefault {
        /// Schema row id.
        schema_id: String,
        /// Observed default posture.
        posture: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row id.
        row_id: String,
        /// Field name.
        field_name: String,
    },
    /// A row has no downgrade policy.
    MissingDowngradeRule {
        /// Schema row id.
        schema_id: String,
    },
}

impl fmt::Display for SchemaRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json {
                source_ref,
                message,
            } => write!(f, "failed to parse {source_ref}: {message}"),
            Self::UnsupportedSchemaVersion {
                source_ref,
                expected,
                actual,
            } => write!(
                f,
                "{source_ref} has unsupported schema_version {actual}; expected {expected}"
            ),
            Self::DuplicateId { id } => write!(f, "duplicate registry id {id}"),
            Self::UnknownRecordClass {
                schema_id,
                record_class_id,
            } => write!(
                f,
                "schema row {schema_id} references unknown record class {record_class_id}"
            ),
            Self::MissingFamilyCoverage { family_class } => {
                write!(f, "missing required family coverage for {family_class}")
            }
            Self::MissingSurfaceVisibility { schema_id, surface } => {
                write!(f, "schema row {schema_id} is not visible on {surface}")
            }
            Self::InvalidTelemetryDefault { schema_id, posture } => write!(
                f,
                "telemetry row {schema_id} has invalid open-build default posture {posture}"
            ),
            Self::EmptyField { row_id, field_name } => {
                write!(f, "registry row {row_id} has empty field {field_name}")
            }
            Self::MissingDowngradeRule { schema_id } => {
                write!(f, "schema row {schema_id} has no downgrade rule")
            }
        }
    }
}

impl Error for SchemaRegistryError {}

/// Loads the embedded governed schema registry.
pub fn load_default_schema_registry() -> Result<GovernedSchemaRegistry, SchemaRegistryError> {
    serde_json::from_str(GOVERNED_SCHEMA_REGISTRY_JSON).map_err(|err| SchemaRegistryError::Json {
        source_ref: GOVERNED_SCHEMA_REGISTRY_PATH.to_owned(),
        message: err.to_string(),
    })
}

/// Loads the embedded governed record-class registry.
pub fn load_default_record_class_registry(
) -> Result<GovernedRecordClassRegistry, SchemaRegistryError> {
    serde_json::from_str(GOVERNED_RECORD_CLASS_REGISTRY_JSON).map_err(|err| {
        SchemaRegistryError::Json {
            source_ref: GOVERNED_RECORD_CLASS_REGISTRY_PATH.to_owned(),
            message: err.to_string(),
        }
    })
}

/// Loads and validates the default governed registries.
pub fn validate_default_registries() -> Result<SchemaRegistryValidationReport, SchemaRegistryError>
{
    let schema_registry = load_default_schema_registry()?;
    let record_class_registry = load_default_record_class_registry()?;
    validate_registries(&schema_registry, &record_class_registry)
}

/// Validates a schema registry against a record-class registry.
pub fn validate_registries(
    schema_registry: &GovernedSchemaRegistry,
    record_class_registry: &GovernedRecordClassRegistry,
) -> Result<SchemaRegistryValidationReport, SchemaRegistryError> {
    validate_version(
        GOVERNED_SCHEMA_REGISTRY_PATH,
        schema_registry.schema_version,
    )?;
    validate_version(
        GOVERNED_RECORD_CLASS_REGISTRY_PATH,
        record_class_registry.schema_version,
    )?;

    let mut record_class_ids = BTreeSet::new();
    for row in &record_class_registry.rows {
        require_non_empty(
            &row.record_class_id,
            &row.record_class_id,
            "record_class_id",
        )?;
        require_non_empty(&row.owner_ref, &row.record_class_id, "owner_ref")?;
        require_non_empty(
            &row.local_vs_managed_truth,
            &row.record_class_id,
            "local_vs_managed_truth",
        )?;
        require_non_empty(
            &row.export_semantics,
            &row.record_class_id,
            "export_semantics",
        )?;
        require_non_empty(
            &row.delete_semantics,
            &row.record_class_id,
            "delete_semantics",
        )?;
        require_non_empty(&row.hold_semantics, &row.record_class_id, "hold_semantics")?;
        require_non_empty(
            &row.redaction_posture,
            &row.record_class_id,
            "redaction_posture",
        )?;
        require_non_empty(
            &row.retention_posture,
            &row.record_class_id,
            "retention_posture",
        )?;
        require_non_empty(
            &row.offboarding_posture,
            &row.record_class_id,
            "offboarding_posture",
        )?;
        if !record_class_ids.insert(row.record_class_id.clone()) {
            return Err(SchemaRegistryError::DuplicateId {
                id: row.record_class_id.clone(),
            });
        }
    }

    let mut schema_ids = BTreeSet::new();
    let mut family_classes = BTreeSet::new();
    for row in &schema_registry.rows {
        require_non_empty(&row.schema_id, &row.schema_id, "schema_id")?;
        require_non_empty(&row.owner_ref, &row.schema_id, "owner_ref")?;
        require_non_empty(&row.schema_ref, &row.schema_id, "schema_ref")?;
        require_non_empty(
            &row.version_change_rationale,
            &row.schema_id,
            "version_change_rationale",
        )?;
        require_non_empty(&row.consent_class, &row.schema_id, "consent_class")?;
        require_non_empty(&row.endpoint_class, &row.schema_id, "endpoint_class")?;
        require_non_empty(&row.retention_posture, &row.schema_id, "retention_posture")?;
        require_non_empty(&row.lifecycle_state, &row.schema_id, "lifecycle_state")?;
        if !schema_ids.insert(row.schema_id.clone()) {
            return Err(SchemaRegistryError::DuplicateId {
                id: row.schema_id.clone(),
            });
        }
        family_classes.insert(row.family_class.clone());

        if row.downgrade_rule.min_readable_version == 0
            || row.downgrade_rule.min_writable_version == 0
            || row.downgrade_rule.unknown_version_policy.is_empty()
            || row.downgrade_rule.deprecated_version_policy.is_empty()
        {
            return Err(SchemaRegistryError::MissingDowngradeRule {
                schema_id: row.schema_id.clone(),
            });
        }

        if row.family_class == "telemetry_payload"
            && row.open_source_default_posture != schema_registry.open_source_telemetry_default
        {
            return Err(SchemaRegistryError::InvalidTelemetryDefault {
                schema_id: row.schema_id.clone(),
                posture: row.open_source_default_posture.clone(),
            });
        }

        for record_class_id in &row.record_class_id_refs {
            if !record_class_registry.contains_class(record_class_id) {
                return Err(SchemaRegistryError::UnknownRecordClass {
                    schema_id: row.schema_id.clone(),
                    record_class_id: record_class_id.clone(),
                });
            }
        }

        for surface in &schema_registry.required_surface_visibility {
            if !row.surface_visibility.iter().any(|token| token == surface) {
                return Err(SchemaRegistryError::MissingSurfaceVisibility {
                    schema_id: row.schema_id.clone(),
                    surface: surface.clone(),
                });
            }
        }
    }

    for family_class in &schema_registry.required_family_class_coverage {
        if !family_classes.contains(family_class) {
            return Err(SchemaRegistryError::MissingFamilyCoverage {
                family_class: family_class.clone(),
            });
        }
    }

    Ok(SchemaRegistryValidationReport {
        schema_row_count: schema_registry.rows.len(),
        record_class_row_count: record_class_registry.rows.len(),
        covered_family_classes: family_classes.into_iter().collect(),
        required_surface_visibility: schema_registry.required_surface_visibility.clone(),
    })
}

fn validate_version(source_ref: &str, actual: u32) -> Result<(), SchemaRegistryError> {
    if actual == GOVERNED_REGISTRY_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(SchemaRegistryError::UnsupportedSchemaVersion {
            source_ref: source_ref.to_owned(),
            expected: GOVERNED_REGISTRY_SCHEMA_VERSION,
            actual,
        })
    }
}

fn require_non_empty(
    value: &str,
    row_id: &str,
    field_name: &str,
) -> Result<(), SchemaRegistryError> {
    if value.is_empty() {
        Err(SchemaRegistryError::EmptyField {
            row_id: row_id.to_owned(),
            field_name: field_name.to_owned(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registries_validate() {
        let report = validate_default_registries().expect("default registries validate");
        assert_eq!(report.schema_row_count, 7);
        assert_eq!(report.record_class_row_count, 8);
        assert!(report
            .covered_family_classes
            .contains(&"telemetry_payload".to_owned()));
        assert!(report
            .covered_family_classes
            .contains(&"sdk_result_schema".to_owned()));
    }

    #[test]
    fn telemetry_defaults_to_opt_in_for_open_builds() {
        let registry = load_default_schema_registry().expect("schema registry loads");
        let row = registry
            .row("telemetry.ux_product_event")
            .expect("telemetry row exists");
        assert_eq!(
            row.open_source_default_posture,
            registry.open_source_telemetry_default
        );
        assert_eq!(row.consent_class, "explicit_opt_in_required");
    }

    #[test]
    fn support_manifest_is_separate_from_telemetry_and_usage() {
        let registry = load_default_schema_registry().expect("schema registry loads");
        let row = registry
            .row("support.bundle_manifest")
            .expect("support row exists");
        assert_eq!(row.family_class, "support_export_payload");
        assert!(row
            .separation
            .must_not_conflate_with
            .contains(&"telemetry_payload".to_owned()));
        assert!(row
            .separation
            .must_not_conflate_with
            .contains(&"usage_export_payload".to_owned()));
    }

    #[test]
    fn packet_versions_degrade_visibly() {
        let registry = load_default_schema_registry().expect("schema registry loads");
        let row = registry
            .row("usage.metering_export_packet")
            .expect("usage row exists");
        assert_eq!(
            row.classify_packet_version(1),
            PacketVersionSupport::Supported
        );
        assert_eq!(
            row.classify_packet_version(0),
            PacketVersionSupport::Unsupported
        );
        assert_eq!(
            row.classify_packet_version(2),
            PacketVersionSupport::Limited
        );
        assert_eq!(
            row.classify_packet_version(9),
            PacketVersionSupport::Unsupported
        );

        let mut newer_row = row.clone();
        newer_row.schema_version = 2;
        newer_row.downgrade_rule.min_readable_version = 1;
        assert_eq!(
            newer_row.classify_packet_version(1),
            PacketVersionSupport::Deprecated
        );
    }

    #[test]
    fn every_required_surface_projects_shared_schema_metadata() {
        let registry = load_default_schema_registry().expect("schema registry loads");
        for surface in GovernanceSurfaceClass::all() {
            let projection = registry.surface_projection(surface);
            assert_eq!(projection.rows.len(), registry.rows.len());
            assert!(projection.rows.iter().all(|row| row.schema_version > 0
                && !row.consent_class.is_empty()
                && !row.endpoint_class.is_empty()
                && !row.lifecycle_state.is_empty()));
        }
    }
}
