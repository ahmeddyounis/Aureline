//! Typed consumer for the protected fitness review packet.
//!
//! This module consumes the checked-in protected fitness packet together
//! with the protected fitness-function catalog and the dashboard state-row
//! vocabulary. It validates the owner, waiver, expiry, catalog linkage, and
//! regression-history invariants before projecting the packet into support
//! export and release-evidence metadata.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

/// Stable record-kind tag for the protected fitness review packet.
pub const PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND: &str = "protected_fitness_review_packet";

/// Current schema version for the protected fitness review packet.
pub const PROTECTED_FITNESS_PACKET_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current schema version for metadata-only protected fitness projections.
pub const PROTECTED_FITNESS_PACKET_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked-in protected fitness packet.
pub const CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_PATH: &str =
    "artifacts/release/protected_fitness_packet_alpha.yaml";

/// Repository-relative path to the checked-in protected fitness catalog.
pub const CURRENT_FITNESS_FUNCTION_CATALOG_PATH: &str =
    "artifacts/bench/fitness_function_catalog.yaml";

/// Repository-relative path to the checked-in fitness state-row vocabulary.
pub const CURRENT_FITNESS_STATE_ROWS_PATH: &str = "artifacts/governance/fitness_state_rows.yaml";

/// Stable record-kind tag for the protected fitness release-candidate packet.
pub const PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND: &str =
    "protected_fitness_release_candidate_packet";

/// Current schema version for the frozen protected fitness release-candidate packet.
pub const PROTECTED_FITNESS_PACKET_BETA_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked-in frozen beta packet.
pub const CURRENT_PROTECTED_FITNESS_PACKET_BETA_PATH: &str =
    "artifacts/release/protected_fitness_packet_beta.yaml";

/// Closed comparator vocabulary for release-candidate thresholds.
const RELEASE_CANDIDATE_COMPARATORS: [&str; 4] = [
    "measured_at_or_below_bar",
    "ratio_at_or_above_floor",
    "boolean_must_hold",
    "provisional_bar_pending_council",
];

const CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/protected_fitness_packet_alpha.yaml"
));

const CURRENT_PROTECTED_FITNESS_PACKET_BETA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/protected_fitness_packet_beta.yaml"
));

const CURRENT_FITNESS_FUNCTION_CATALOG_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/bench/fitness_function_catalog.yaml"
));

const CURRENT_FITNESS_STATE_ROWS_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/fitness_state_rows.yaml"
));

const EXPECTED_WAIVER_STATES: [&str; 4] = [
    "no_active_waiver",
    "active_waiver",
    "expired_waiver",
    "threshold_provisional_pending_council",
];

const REQUIRED_FIXTURE_STATES: [&str; 4] = [
    "current_pass",
    "stale_evidence_degrades",
    "active_waiver_visible",
    "expired_waiver_degrades",
];

/// Loads and validates the checked-in protected fitness packet.
///
/// # Errors
///
/// Returns a typed parse, I/O, or validation error when the packet, catalog,
/// or state-row vocabulary cannot be trusted.
pub fn current_fitness_packet_alpha() -> Result<FitnessPacketAlpha, FitnessPacketAlphaError> {
    FitnessPacketAlpha::from_yaml_documents(
        CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_YAML,
        CURRENT_FITNESS_FUNCTION_CATALOG_YAML,
        CURRENT_FITNESS_STATE_ROWS_YAML,
    )
}

/// Loads and validates the checked-in frozen beta release-candidate packet.
///
/// The beta record reuses the checked-in alpha review packet verbatim as its
/// base evidence and layers release-candidate thresholds, active-waiver
/// visibility, and expired-waiver degradation on top.
///
/// # Errors
///
/// Returns a typed parse, I/O, or validation error when the beta packet, its
/// base alpha packet, the catalog, or the state-row vocabulary cannot be
/// trusted.
pub fn current_fitness_packet_beta() -> Result<FitnessPacketBeta, FitnessPacketBetaError> {
    FitnessPacketBeta::from_yaml_documents(
        CURRENT_PROTECTED_FITNESS_PACKET_BETA_YAML,
        CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_YAML,
        CURRENT_FITNESS_FUNCTION_CATALOG_YAML,
        CURRENT_FITNESS_STATE_ROWS_YAML,
    )
}

/// Loads the checked-in protected fitness catalog.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in catalog shape drifts from
/// [`FitnessFunctionCatalog`].
pub fn current_fitness_function_catalog() -> Result<FitnessFunctionCatalog, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_FITNESS_FUNCTION_CATALOG_YAML)
}

/// Loads the checked-in fitness state-row vocabulary.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in state-row shape drifts from
/// [`FitnessStateRows`].
pub fn current_fitness_state_rows() -> Result<FitnessStateRows, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_FITNESS_STATE_ROWS_YAML)
}

/// Error returned when the protected fitness packet cannot be consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FitnessPacketAlphaError {
    /// The packet YAML could not be parsed.
    PacketYaml(String),
    /// The catalog YAML could not be parsed.
    CatalogYaml(String),
    /// The state-row YAML could not be parsed.
    StateRowsYaml(String),
    /// A filesystem read failed while loading explicit paths.
    Io {
        /// Path that failed to read.
        path: String,
        /// Redaction-safe I/O error detail.
        detail: String,
    },
    /// The packet parsed but failed one or more trust checks.
    Invalid(Vec<FitnessPacketAlphaViolation>),
}

impl fmt::Display for FitnessPacketAlphaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketYaml(detail) => {
                write!(f, "failed to parse protected fitness packet YAML: {detail}")
            }
            Self::CatalogYaml(detail) => {
                write!(
                    f,
                    "failed to parse protected fitness catalog YAML: {detail}"
                )
            }
            Self::StateRowsYaml(detail) => {
                write!(f, "failed to parse fitness state-row YAML: {detail}")
            }
            Self::Io { path, detail } => write!(f, "failed to read {path}: {detail}"),
            Self::Invalid(violations) => {
                if let Some(first) = violations.first() {
                    write!(
                        f,
                        "invalid protected fitness packet: {} at {}",
                        first.check_id, first.reference
                    )
                } else {
                    write!(f, "invalid protected fitness packet")
                }
            }
        }
    }
}

impl std::error::Error for FitnessPacketAlphaError {}

/// One validation violation emitted by the protected fitness consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FitnessPacketAlphaViolation {
    /// Stable check id for the violated rule.
    pub check_id: String,
    /// Packet, row, or artifact ref associated with the violation.
    pub reference: String,
    /// Redaction-safe validation message.
    pub message: String,
}

/// Protected fitness-function catalog consumed by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessFunctionCatalog {
    /// Catalog schema version.
    #[serde(default)]
    pub schema_version: u32,
    /// Stable catalog id.
    #[serde(default)]
    pub catalog_id: String,
    /// Stable catalog revision.
    #[serde(default)]
    pub catalog_revision: u32,
    /// Closed threshold-mode vocabulary.
    #[serde(default)]
    pub threshold_modes: Vec<String>,
    /// Closed waiver-authority vocabulary.
    #[serde(default)]
    pub waiver_authorities: Vec<String>,
    /// Catalog rows keyed by protected fitness-function id.
    #[serde(default)]
    pub rows: Vec<FitnessCatalogRow>,
}

/// One protected fitness-function row from the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessCatalogRow {
    /// Stable protected fitness-function id.
    #[serde(default)]
    pub id: String,
    /// Primary DRI handle.
    #[serde(default)]
    pub owner: String,
    /// Owning scorecard lane.
    #[serde(default)]
    pub owning_lane: String,
    /// Optional co-owning scorecard lane.
    #[serde(default)]
    pub co_owning_lane: Option<String>,
    /// Catalog row lifecycle state.
    #[serde(default)]
    pub row_status: String,
    /// Protected SLO family used for grouping.
    #[serde(default)]
    pub protected_slo_family: String,
    /// Forum that may waive this row.
    #[serde(default)]
    pub waiver_authority: String,
    /// Threshold interpretation mode.
    #[serde(default)]
    pub threshold_mode: String,
}

/// Fitness dashboard state-row vocabulary consumed by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessStateRows {
    /// State-row schema version.
    #[serde(default)]
    pub schema_version: u32,
    /// Stable register id.
    #[serde(default)]
    pub register_id: String,
    /// Closed tile-state rows.
    #[serde(default)]
    pub tile_states: Vec<FitnessVocabularyRow>,
    /// Closed evidence-freshness rows.
    #[serde(default)]
    pub evidence_freshness_classes: Vec<FitnessVocabularyRow>,
    /// Closed waiver-authority rows.
    #[serde(default)]
    pub waiver_authority_classes: Vec<FitnessVocabularyRow>,
}

/// One row in a closed vocabulary register.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessVocabularyRow {
    /// Stable token id.
    #[serde(default)]
    pub id: String,
}

/// The protected fitness review packet consumed by support and release lanes.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessPacketAlpha {
    /// Packet schema version.
    #[serde(default)]
    pub schema_version: u32,
    /// Record-kind discriminator.
    #[serde(default)]
    pub record_kind: String,
    /// Stable packet id.
    #[serde(default)]
    pub packet_id: String,
    /// Packet revision number.
    #[serde(default)]
    pub packet_revision: u32,
    /// Current review state.
    #[serde(default)]
    pub packet_state: String,
    /// RFC 3339 UTC timestamp for the packet evaluation.
    #[serde(default)]
    pub as_of: String,
    /// Primary owner for the packet.
    #[serde(default)]
    pub owner_dri: String,
    /// Evidence owner for release/support follow-up.
    #[serde(default)]
    pub evidence_owner: String,
    /// Overall protected fitness result.
    #[serde(default)]
    pub overall_result: String,
    /// Redaction-safe packet summary.
    #[serde(default)]
    pub overall_summary: String,
    /// Source contracts consumed by this packet.
    #[serde(default)]
    pub source_contract_refs: BTreeMap<String, String>,
    /// Generation metadata for validators and review rendering.
    #[serde(default)]
    pub generation: FitnessPacketGeneration,
    /// Release context for the packet.
    #[serde(default)]
    pub review_context: FitnessReviewContext,
    /// Allowed result-state vocabulary emitted by the packet.
    #[serde(default)]
    pub result_state_vocabulary: Vec<String>,
    /// Allowed waiver-state vocabulary emitted by the packet.
    #[serde(default)]
    pub waiver_state_vocabulary: Vec<String>,
    /// Degrade rules the packet applies to upstream evidence.
    #[serde(default)]
    pub degrade_rules: Vec<FitnessDegradeRule>,
    /// Protected function rows rendered by the packet.
    #[serde(default)]
    pub protected_function_rows: Vec<FitnessProtectedFunctionRow>,
    /// Fixture coverage advertised by the packet.
    #[serde(default)]
    pub protected_fixture_coverage: Vec<FitnessFixtureCoverageRow>,
    /// Downstream projection refs consumed by support and release lanes.
    #[serde(default)]
    pub release_consumer_projection: FitnessReleaseConsumerProjection,
}

impl FitnessPacketAlpha {
    /// Parses and validates a packet from YAML strings.
    ///
    /// # Errors
    ///
    /// Returns a parse error when any YAML document is malformed, or
    /// [`FitnessPacketAlphaError::Invalid`] when structural validation fails.
    pub fn from_yaml_documents(
        packet_yaml: &str,
        catalog_yaml: &str,
        state_rows_yaml: &str,
    ) -> Result<Self, FitnessPacketAlphaError> {
        let packet = serde_yaml::from_str::<Self>(packet_yaml)
            .map_err(|err| FitnessPacketAlphaError::PacketYaml(err.to_string()))?;
        let catalog = serde_yaml::from_str::<FitnessFunctionCatalog>(catalog_yaml)
            .map_err(|err| FitnessPacketAlphaError::CatalogYaml(err.to_string()))?;
        let state_rows = serde_yaml::from_str::<FitnessStateRows>(state_rows_yaml)
            .map_err(|err| FitnessPacketAlphaError::StateRowsYaml(err.to_string()))?;

        let violations = packet.validate_with_catalogs(&catalog, &state_rows);
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(FitnessPacketAlphaError::Invalid(violations))
        }
    }

    /// Loads and validates a packet from explicit filesystem paths.
    ///
    /// # Errors
    ///
    /// Returns a typed I/O, parse, or validation error.
    pub fn from_paths(
        packet_path: impl AsRef<Path>,
        catalog_path: impl AsRef<Path>,
        state_rows_path: impl AsRef<Path>,
    ) -> Result<Self, FitnessPacketAlphaError> {
        let packet_path = packet_path.as_ref();
        let catalog_path = catalog_path.as_ref();
        let state_rows_path = state_rows_path.as_ref();

        let packet_yaml = read_to_string(packet_path)?;
        let catalog_yaml = read_to_string(catalog_path)?;
        let state_rows_yaml = read_to_string(state_rows_path)?;

        Self::from_yaml_documents(&packet_yaml, &catalog_yaml, &state_rows_yaml)
    }

    /// Validates this packet against the catalog and state-row vocabulary.
    pub fn validate_with_catalogs(
        &self,
        catalog: &FitnessFunctionCatalog,
        state_rows: &FitnessStateRows,
    ) -> Vec<FitnessPacketAlphaViolation> {
        let mut violations = Vec::new();
        let as_of = parse_datetime(&self.as_of);

        if self.schema_version != PROTECTED_FITNESS_PACKET_ALPHA_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "fitness_packet.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND {
            push_violation(
                &mut violations,
                "fitness_packet.record_kind",
                &self.packet_id,
                "record_kind must be protected_fitness_review_packet",
            );
        }
        require_non_empty(
            &mut violations,
            "fitness_packet.packet_id",
            &self.packet_id,
            &self.packet_id,
        );
        require_non_empty(
            &mut violations,
            "fitness_packet.owner_dri",
            &self.owner_dri,
            &self.packet_id,
        );
        require_non_empty(
            &mut violations,
            "fitness_packet.evidence_owner",
            &self.evidence_owner,
            &self.packet_id,
        );
        if as_of.is_none() {
            push_violation(
                &mut violations,
                "fitness_packet.as_of",
                &self.packet_id,
                "as_of must be an RFC 3339 UTC timestamp",
            );
        }

        self.validate_vocabularies(catalog, state_rows, &mut violations);

        let catalog_by_id = catalog
            .rows
            .iter()
            .filter(|row| !row.id.trim().is_empty())
            .map(|row| (row.id.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        let catalog_authorities = catalog
            .waiver_authorities
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let evidence_freshness = state_rows
            .evidence_freshness_classes
            .iter()
            .map(|row| row.id.as_str())
            .collect::<BTreeSet<_>>();
        let waiver_authorities = state_rows
            .waiver_authority_classes
            .iter()
            .map(|row| row.id.as_str())
            .collect::<BTreeSet<_>>();

        if self.protected_function_rows.is_empty() {
            push_violation(
                &mut violations,
                "fitness_packet.protected_function_rows",
                &self.packet_id,
                "protected_function_rows must be non-empty",
            );
        }

        let mut seen_rows = BTreeSet::new();
        for row in &self.protected_function_rows {
            self.validate_row(
                row,
                as_of,
                &catalog_by_id,
                &catalog_authorities,
                &evidence_freshness,
                &waiver_authorities,
                &mut seen_rows,
                &mut violations,
            );
        }

        self.validate_fixture_coverage(&mut violations);

        violations
    }

    /// Projects packet metadata into a support-bundle export row.
    pub fn support_bundle_projection(&self) -> FitnessPacketProjection {
        self.projection()
    }

    /// Projects packet metadata into a release-evidence row.
    pub fn release_evidence_projection(&self) -> FitnessPacketProjection {
        self.projection()
    }

    fn projection(&self) -> FitnessPacketProjection {
        let result_counts = count_by(self.protected_function_rows.iter().map(|row| {
            if row.current_result.trim().is_empty() {
                "unknown"
            } else {
                row.current_result.as_str()
            }
        }));
        let waiver_counts = count_by(self.protected_function_rows.iter().map(|row| {
            if row.waiver.waiver_state.trim().is_empty() {
                "unknown"
            } else {
                row.waiver.waiver_state.as_str()
            }
        }));
        let stale_or_blocking_row_count = self
            .protected_function_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.current_result.as_str(),
                    "blocked" | "evidence_stale" | "provisional" | "waiver_expired"
                )
            })
            .count() as u32;
        let source_refs = self
            .protected_function_rows
            .iter()
            .filter_map(|row| row.result_source.source_ref.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        FitnessPacketProjection {
            schema_version: PROTECTED_FITNESS_PACKET_PROJECTION_SCHEMA_VERSION,
            record_kind: PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND.to_owned(),
            packet_ref: CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_PATH.to_owned(),
            packet_id: self.packet_id.clone(),
            packet_state: self.packet_state.clone(),
            as_of: self.as_of.clone(),
            owner_dri: self.owner_dri.clone(),
            evidence_owner: self.evidence_owner.clone(),
            overall_result: self.overall_result.clone(),
            protected_function_count: self.protected_function_rows.len() as u32,
            result_counts,
            waiver_state_counts: waiver_counts,
            stale_or_blocking_row_count,
            expired_waiver_count: self
                .protected_function_rows
                .iter()
                .filter(|row| row.waiver.waiver_state == "expired_waiver")
                .count() as u32,
            source_refs,
            support_export_projection_ref: self
                .release_consumer_projection
                .support_export_projection_ref
                .clone(),
            release_packet_projection_ref: self
                .release_consumer_projection
                .release_packet_projection_ref
                .clone(),
            raw_private_material_excluded: true,
        }
    }

    fn validate_vocabularies(
        &self,
        catalog: &FitnessFunctionCatalog,
        state_rows: &FitnessStateRows,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let packet_results = self
            .result_state_vocabulary
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let tile_states = state_rows
            .tile_states
            .iter()
            .map(|row| row.id.as_str())
            .collect::<BTreeSet<_>>();

        for state in tile_states {
            if !packet_results.contains(state) {
                push_violation(
                    violations,
                    "fitness_packet.result_state_vocabulary",
                    state,
                    "packet result-state vocabulary must include every fitness tile state",
                );
            }
        }
        if !packet_results.contains("provisional") {
            push_violation(
                violations,
                "fitness_packet.result_state_vocabulary",
                "provisional",
                "packet result-state vocabulary must include provisional for threshold seeds",
            );
        }

        let packet_waiver_states = self
            .waiver_state_vocabulary
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for state in EXPECTED_WAIVER_STATES {
            if !packet_waiver_states.contains(state) {
                push_violation(
                    violations,
                    "fitness_packet.waiver_state_vocabulary",
                    state,
                    "packet waiver-state vocabulary is missing a required state",
                );
            }
        }

        let catalog_authorities = catalog
            .waiver_authorities
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let state_authorities = state_rows
            .waiver_authority_classes
            .iter()
            .map(|row| row.id.as_str())
            .collect::<BTreeSet<_>>();
        for authority in &catalog_authorities {
            if !state_authorities.contains(authority) {
                push_violation(
                    violations,
                    "fitness_state_rows.waiver_authority_classes",
                    *authority,
                    "state-row waiver-authority vocabulary must include catalog authorities",
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_row(
        &self,
        row: &FitnessProtectedFunctionRow,
        as_of: Option<OffsetDateTime>,
        catalog_by_id: &BTreeMap<&str, &FitnessCatalogRow>,
        catalog_authorities: &BTreeSet<&str>,
        evidence_freshness: &BTreeSet<&str>,
        waiver_authorities: &BTreeSet<&str>,
        seen_rows: &mut BTreeSet<String>,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let reference = row.reference();
        if !seen_rows.insert(reference.clone()) {
            push_violation(
                violations,
                "fitness_packet.protected_function_rows.duplicate",
                &reference,
                "protected_function_ref values must be unique",
            );
        }

        for (check_id, value) in [
            ("protected_function_rows.protected_function_ref", &reference),
            ("protected_function_rows.owner_dri", &row.owner_dri),
            ("protected_function_rows.owning_lane", &row.owning_lane),
            (
                "protected_function_rows.waiver_authority_ref",
                &row.waiver_authority_ref,
            ),
            (
                "protected_function_rows.current_result_reason",
                &row.current_result_reason,
            ),
            (
                "protected_function_rows.last_pass_summary",
                &row.last_pass_summary,
            ),
        ] {
            require_non_empty(violations, check_id, value, &reference);
        }

        if !self
            .result_state_vocabulary
            .iter()
            .any(|state| state == &row.current_result)
        {
            push_violation(
                violations,
                "protected_function_rows.current_result",
                &reference,
                "current_result must resolve through result_state_vocabulary",
            );
        }
        if row.current_result == "passing" && row.last_passed_at.is_none() {
            push_violation(
                violations,
                "protected_function_rows.last_passed_at",
                &reference,
                "passing rows must include last_passed_at",
            );
        }
        if let Some(last_passed_at) = &row.last_passed_at {
            if parse_datetime(last_passed_at).is_none() {
                push_violation(
                    violations,
                    "protected_function_rows.last_passed_at",
                    &reference,
                    "last_passed_at must be an RFC 3339 UTC timestamp",
                );
            }
        }

        self.validate_catalog_link(row, catalog_by_id, violations);

        if !catalog_authorities.contains(row.waiver_authority_ref.as_str()) {
            push_violation(
                violations,
                "protected_function_rows.waiver_authority_ref",
                &reference,
                "waiver_authority_ref must resolve through the catalog waiver authorities",
            );
        }
        if !waiver_authorities.contains(row.waiver_authority_ref.as_str()) {
            push_violation(
                violations,
                "protected_function_rows.waiver_authority_ref",
                &reference,
                "waiver_authority_ref must resolve through the state-row waiver authorities",
            );
        }

        self.validate_evidence(row, as_of, evidence_freshness, violations);
        self.validate_waiver(row, as_of, violations);
        self.validate_regression_history(row, violations);
    }

    fn validate_catalog_link(
        &self,
        row: &FitnessProtectedFunctionRow,
        catalog_by_id: &BTreeMap<&str, &FitnessCatalogRow>,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let reference = row.reference();
        if row.protected_function_kind == "fitness_catalog_row" && row.catalog_row_ref.is_none() {
            push_violation(
                violations,
                "protected_function_rows.catalog_row_ref",
                &reference,
                "fitness catalog rows must carry catalog_row_ref",
            );
            return;
        }

        let Some(catalog_ref) = row.catalog_row_ref.as_deref() else {
            return;
        };
        let Some(catalog_row) = catalog_by_id.get(catalog_ref) else {
            push_violation(
                violations,
                "protected_function_rows.catalog_row_ref",
                &reference,
                "catalog_row_ref must resolve through the protected fitness catalog",
            );
            return;
        };

        if row.protected_function_ref != catalog_ref {
            push_violation(
                violations,
                "protected_function_rows.catalog_row_ref",
                &reference,
                "catalog_row_ref must match protected_function_ref for catalog rows",
            );
        }
        if row.owner_dri != catalog_row.owner {
            push_violation(
                violations,
                "protected_function_rows.owner_dri",
                &reference,
                "row owner_dri must match the catalog owner",
            );
        }
        if row.owning_lane != catalog_row.owning_lane {
            push_violation(
                violations,
                "protected_function_rows.owning_lane",
                &reference,
                "row owning_lane must match the catalog owning_lane",
            );
        }
        if row.co_owning_lane != catalog_row.co_owning_lane {
            push_violation(
                violations,
                "protected_function_rows.co_owning_lane",
                &reference,
                "row co_owning_lane must match the catalog co_owning_lane",
            );
        }
        if row.waiver_authority_ref != catalog_row.waiver_authority {
            push_violation(
                violations,
                "protected_function_rows.waiver_authority_ref",
                &reference,
                "row waiver_authority_ref must match the catalog waiver_authority",
            );
        }
    }

    fn validate_evidence(
        &self,
        row: &FitnessProtectedFunctionRow,
        as_of: Option<OffsetDateTime>,
        evidence_freshness: &BTreeSet<&str>,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let reference = row.reference();
        let evidence = &row.evidence;

        if !evidence_freshness.contains(evidence.evidence_freshness_class.as_str()) {
            push_violation(
                violations,
                "protected_function_rows.evidence.evidence_freshness_class",
                &reference,
                "evidence_freshness_class must resolve through fitness_state_rows",
            );
        }

        let captured_at = parse_datetime(&evidence.captured_at);
        if captured_at.is_none() {
            push_violation(
                violations,
                "protected_function_rows.evidence.captured_at",
                &reference,
                "captured_at must be an RFC 3339 UTC timestamp",
            );
        }
        let expires_at = parse_datetime(&evidence.expires_at);
        if expires_at.is_none() {
            push_violation(
                violations,
                "protected_function_rows.evidence.expires_at",
                &reference,
                "expires_at must be an RFC 3339 UTC timestamp",
            );
        }
        let stale_after = parse_day_duration(&evidence.stale_after);
        if stale_after.is_none() {
            push_violation(
                violations,
                "protected_function_rows.evidence.stale_after",
                &reference,
                "stale_after must be an ISO day duration such as P14D",
            );
        }

        if let (Some(captured_at), Some(stale_after), Some(expires_at)) =
            (captured_at, stale_after, expires_at)
        {
            if captured_at + stale_after != expires_at {
                push_violation(
                    violations,
                    "protected_function_rows.evidence.expires_at",
                    &reference,
                    "expires_at must equal captured_at plus stale_after",
                );
            }
            if matches!(as_of, Some(as_of) if as_of > expires_at) && row.current_result == "passing"
            {
                push_violation(
                    violations,
                    "protected_function_rows.evidence.stale_overclaim",
                    &reference,
                    "passing rows must not carry expired evidence",
                );
            }
        }
    }

    fn validate_waiver(
        &self,
        row: &FitnessProtectedFunctionRow,
        as_of: Option<OffsetDateTime>,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let reference = row.reference();
        let waiver = &row.waiver;
        if !self
            .waiver_state_vocabulary
            .iter()
            .any(|state| state == &waiver.waiver_state)
            || !EXPECTED_WAIVER_STATES.contains(&waiver.waiver_state.as_str())
        {
            push_violation(
                violations,
                "protected_function_rows.waiver.waiver_state",
                &reference,
                "waiver_state must resolve through the protected waiver-state vocabulary",
            );
            return;
        }

        if waiver.waiver_authority_ref != row.waiver_authority_ref {
            push_violation(
                violations,
                "protected_function_rows.waiver.waiver_authority_ref",
                &reference,
                "waiver waiver_authority_ref must match the row waiver_authority_ref",
            );
        }

        let expiry_at = waiver.expiry_at.as_deref().and_then(parse_datetime);
        match waiver.waiver_state.as_str() {
            "no_active_waiver" => {
                if non_empty_option(&waiver.waiver_record_ref) || waiver.expiry_at.is_some() {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.no_active_has_ref_or_expiry",
                        &reference,
                        "no_active_waiver rows must keep waiver_record_ref and expiry_at null",
                    );
                }
            }
            "active_waiver" => {
                if !non_empty_option(&waiver.waiver_record_ref) || expiry_at.is_none() {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.active_missing_fields",
                        &reference,
                        "active waivers must carry waiver_record_ref and expiry_at",
                    );
                }
                if matches!((as_of, expiry_at), (Some(as_of), Some(expiry_at)) if expiry_at <= as_of)
                {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.active_expired",
                        &reference,
                        "active waiver expiry_at must be in the future",
                    );
                }
                if row.current_result == "passing" {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.active_overclaims",
                        &reference,
                        "rows with active waivers must not render as passing",
                    );
                }
            }
            "expired_waiver" => {
                if !non_empty_option(&waiver.waiver_record_ref) || expiry_at.is_none() {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.expired_missing_fields",
                        &reference,
                        "expired waivers must carry waiver_record_ref and expiry_at",
                    );
                }
                if matches!((as_of, expiry_at), (Some(as_of), Some(expiry_at)) if expiry_at > as_of)
                {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.expired_in_future",
                        &reference,
                        "expired_waiver expiry_at must not be in the future",
                    );
                }
                if matches!(
                    row.current_result.as_str(),
                    "passing" | "warning" | "waived"
                ) {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.expired_overclaims",
                        &reference,
                        "expired waivers must degrade instead of rendering passing, warning, or waived",
                    );
                }
            }
            "threshold_provisional_pending_council" => {
                if row.current_result != "provisional" {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.threshold_provisional_mismatch",
                        &reference,
                        "threshold_provisional_pending_council is only admissible on provisional rows",
                    );
                }
                if non_empty_option(&waiver.waiver_record_ref) || waiver.expiry_at.is_some() {
                    push_violation(
                        violations,
                        "protected_function_rows.waiver.threshold_provisional_has_ref_or_expiry",
                        &reference,
                        "threshold-provisional rows must not carry waiver_record_ref or expiry_at",
                    );
                }
            }
            _ => {}
        }
    }

    fn validate_regression_history(
        &self,
        row: &FitnessProtectedFunctionRow,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let reference = row.reference();
        if row.regression_history.status_counts.is_empty() {
            push_violation(
                violations,
                "protected_function_rows.regression_history.status_counts",
                &reference,
                "regression history must carry at least one status count",
            );
        }
        require_non_empty(
            violations,
            "protected_function_rows.regression_history.prior_baseline_summary",
            &row.regression_history.prior_baseline_summary,
            &reference,
        );
        require_non_empty(
            violations,
            "protected_function_rows.regression_history.history_source_ref",
            &row.regression_history.history_source_ref,
            &reference,
        );
        if let Some(source_ref) = row.result_source.source_ref.as_deref() {
            if row.regression_history.history_source_ref != source_ref {
                push_violation(
                    violations,
                    "protected_function_rows.regression_history.history_source_ref",
                    &reference,
                    "regression history source must match the result source",
                );
            }
        }

        if row.current_result == "passing"
            && !row.regression_history.status_counts.contains_key("pass")
        {
            push_violation(
                violations,
                "protected_function_rows.regression_history.status_counts",
                &reference,
                "passing rows must carry pass history",
            );
        }
        if row.current_result == "provisional"
            && !row
                .regression_history
                .status_counts
                .contains_key("pending_scenario_seed")
        {
            push_violation(
                violations,
                "protected_function_rows.regression_history.status_counts",
                &reference,
                "provisional rows must carry pending scenario history",
            );
        }
    }

    fn validate_fixture_coverage(&self, violations: &mut Vec<FitnessPacketAlphaViolation>) {
        let seen = self
            .protected_fixture_coverage
            .iter()
            .map(|fixture| fixture.exercises_state.as_str())
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_FIXTURE_STATES {
            if !seen.contains(required) {
                push_violation(
                    violations,
                    "protected_fixture_coverage.exercises_state",
                    required,
                    "fixture coverage must include current pass, stale evidence, active waiver, and expired waiver states",
                );
            }
        }
        for fixture in &self.protected_fixture_coverage {
            require_non_empty(
                violations,
                "protected_fixture_coverage.fixture_id",
                &fixture.fixture_id,
                &fixture.fixture_id,
            );
            require_non_empty(
                violations,
                "protected_fixture_coverage.fixture_ref",
                &fixture.fixture_ref,
                &fixture.fixture_id,
            );
            if !self
                .result_state_vocabulary
                .iter()
                .any(|state| state == &fixture.expected_current_result)
            {
                push_violation(
                    violations,
                    "protected_fixture_coverage.expected_current_result",
                    &fixture.fixture_id,
                    "expected_current_result must resolve through result_state_vocabulary",
                );
            }
        }
    }
}

/// Packet generation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessPacketGeneration {
    /// Validator command for the packet.
    #[serde(default)]
    pub validator_command: String,
    /// Review-rendering command for the packet.
    #[serde(default)]
    pub review_render_command: String,
    /// Checked-in rendered review packet ref.
    #[serde(default)]
    pub review_packet_ref: String,
    /// Inputs used to generate the packet.
    #[serde(default)]
    pub generated_from_checked_in_outputs: Vec<String>,
}

/// Release context attached to the protected fitness packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessReviewContext {
    /// Release channel context.
    #[serde(default)]
    pub channel_context: String,
    /// Deployment contexts covered by the packet.
    #[serde(default)]
    pub deployment_context: Vec<String>,
    /// Candidate stage under review.
    #[serde(default)]
    pub candidate_stage: String,
    /// Exact-build identity source ref.
    #[serde(default)]
    pub exact_build_identity_ref: String,
    /// Exact-build identity token.
    #[serde(default)]
    pub exact_build_identity_token: String,
    /// Review forum refs in scope.
    #[serde(default)]
    pub review_forum_refs: Vec<String>,
    /// Release output refs in scope.
    #[serde(default)]
    pub release_output_refs: Vec<String>,
    /// Promotion gate refs in scope.
    #[serde(default)]
    pub promotion_gate_refs: Vec<String>,
}

/// One packet degrade rule.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessDegradeRule {
    /// Stable degrade-rule id.
    #[serde(default)]
    pub rule_id: String,
    /// Redaction-safe rule summary.
    #[serde(default)]
    pub summary: String,
}

/// One protected function row rendered by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessProtectedFunctionRow {
    /// Stable protected function ref.
    #[serde(default)]
    pub protected_function_ref: String,
    /// Human-readable title.
    #[serde(default)]
    pub title: String,
    /// Packet row kind.
    #[serde(default)]
    pub protected_function_kind: String,
    /// Packet family grouping.
    #[serde(default)]
    pub protected_function_family: String,
    /// Optional catalog row ref.
    #[serde(default)]
    pub catalog_row_ref: Option<String>,
    /// Primary DRI handle.
    #[serde(default)]
    pub owner_dri: String,
    /// Owning scorecard lane.
    #[serde(default)]
    pub owning_lane: String,
    /// Optional co-owning scorecard lane.
    #[serde(default)]
    pub co_owning_lane: Option<String>,
    /// Forum that may waive this row.
    #[serde(default)]
    pub waiver_authority_ref: String,
    /// Current result token.
    #[serde(default)]
    pub current_result: String,
    /// Redaction-safe result reason.
    #[serde(default)]
    pub current_result_reason: String,
    /// Last passing timestamp, when a current pass exists.
    #[serde(default)]
    pub last_passed_at: Option<String>,
    /// Redaction-safe summary of the last pass posture.
    #[serde(default)]
    pub last_pass_summary: String,
    /// Source that produced the row result.
    #[serde(default)]
    pub result_source: FitnessResultSource,
    /// Evidence freshness envelope.
    #[serde(default)]
    pub evidence: FitnessEvidence,
    /// Waiver lifecycle envelope.
    #[serde(default)]
    pub waiver: FitnessWaiver,
    /// Regression-history summary.
    #[serde(default)]
    pub regression_history: FitnessRegressionHistory,
}

impl FitnessProtectedFunctionRow {
    fn reference(&self) -> String {
        if self.protected_function_ref.trim().is_empty() {
            "<missing protected_function_ref>".to_owned()
        } else {
            self.protected_function_ref.clone()
        }
    }
}

/// Result source for one protected function row.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessResultSource {
    /// Source kind.
    #[serde(default)]
    pub kind: String,
    /// Optional gate definition ref.
    #[serde(default)]
    pub gate_definition_ref: Option<String>,
    /// Optional evidence source ref.
    #[serde(default)]
    pub source_ref: Option<String>,
    /// Gate row refs used by dashboard-backed rows.
    #[serde(default)]
    pub gate_row_refs: Vec<String>,
    /// Expected CI result for dashboard-backed rows.
    #[serde(default)]
    pub expected_ci_result: Option<String>,
    /// Scope-degrade rule applied by partial projections.
    #[serde(default)]
    pub scope_degrade: Option<String>,
    /// Expected support scorecard status for support rows.
    #[serde(default)]
    pub expected_scorecard_status: Option<String>,
    /// Scenario family count for support rows.
    #[serde(default)]
    pub scenario_family_count: Option<u32>,
}

/// Evidence freshness envelope for one protected function row.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessEvidence {
    /// Capture timestamp.
    #[serde(default)]
    pub captured_at: String,
    /// ISO duration after which the evidence is stale.
    #[serde(default)]
    pub stale_after: String,
    /// Expiry timestamp derived from capture time and stale-after duration.
    #[serde(default)]
    pub expires_at: String,
    /// Evidence freshness token.
    #[serde(default)]
    pub evidence_freshness_class: String,
    /// Exact-build identity source ref.
    #[serde(default)]
    pub exact_build_identity_ref: String,
    /// Exact-build identity token.
    #[serde(default)]
    pub exact_build_identity_token: String,
}

/// Waiver lifecycle envelope for one protected function row.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessWaiver {
    /// Current waiver state token.
    #[serde(default)]
    pub waiver_state: String,
    /// Optional waiver record ref.
    #[serde(default)]
    pub waiver_record_ref: Option<String>,
    /// Forum that may waive this row.
    #[serde(default)]
    pub waiver_authority_ref: String,
    /// Optional waiver expiry timestamp.
    #[serde(default)]
    pub expiry_at: Option<String>,
    /// Redaction-safe waiver summary.
    #[serde(default)]
    pub summary: String,
}

/// Regression-history summary for one protected function row.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessRegressionHistory {
    /// Status counts observed in the upstream history source.
    #[serde(default)]
    pub status_counts: BTreeMap<String, u32>,
    /// Redaction-safe prior baseline summary.
    #[serde(default)]
    pub prior_baseline_summary: String,
    /// Optional last regression event ref.
    #[serde(default)]
    pub last_regression_event_ref: Option<String>,
    /// Upstream history source ref.
    #[serde(default)]
    pub history_source_ref: String,
}

/// One fixture coverage row advertised by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessFixtureCoverageRow {
    /// Stable fixture id.
    #[serde(default)]
    pub fixture_id: String,
    /// Repository-relative fixture ref.
    #[serde(default)]
    pub fixture_ref: String,
    /// Expected current result in the fixture.
    #[serde(default)]
    pub expected_current_result: String,
    /// State transition covered by the fixture.
    #[serde(default)]
    pub exercises_state: String,
}

/// Downstream consumer projection refs from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessReleaseConsumerProjection {
    /// First consumer that validates the packet.
    #[serde(default)]
    pub first_consumer: String,
    /// Consumer kind token.
    #[serde(default)]
    pub consumer_kind: String,
    /// Rendered review packet ref.
    #[serde(default)]
    pub emitted_review_packet_ref: String,
    /// Support-export projection ref.
    #[serde(default)]
    pub support_export_projection_ref: String,
    /// Release-packet projection ref.
    #[serde(default)]
    pub release_packet_projection_ref: String,
    /// Whether free-form status notes are allowed.
    #[serde(default)]
    pub free_form_status_notes_allowed: bool,
}

/// Metadata-only projection consumed by support export and release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FitnessPacketProjection {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag copied from the protected fitness packet.
    pub record_kind: String,
    /// Repository-relative packet ref.
    pub packet_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Current packet state.
    pub packet_state: String,
    /// Packet evaluation timestamp.
    pub as_of: String,
    /// Primary packet owner.
    pub owner_dri: String,
    /// Evidence owner for release/support follow-up.
    pub evidence_owner: String,
    /// Overall packet result.
    pub overall_result: String,
    /// Number of protected function rows.
    pub protected_function_count: u32,
    /// Count of rows by result token.
    pub result_counts: BTreeMap<String, u32>,
    /// Count of rows by waiver-state token.
    pub waiver_state_counts: BTreeMap<String, u32>,
    /// Number of rows that block or narrow release claims.
    pub stale_or_blocking_row_count: u32,
    /// Number of rows whose waiver is expired.
    pub expired_waiver_count: u32,
    /// Evidence source refs represented in the packet.
    pub source_refs: Vec<String>,
    /// Support-export projection ref from the packet.
    pub support_export_projection_ref: String,
    /// Release-packet projection ref from the packet.
    pub release_packet_projection_ref: String,
    /// Whether raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
}

/// Error returned when the frozen beta release-candidate packet cannot be
/// consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FitnessPacketBetaError {
    /// The beta packet YAML could not be parsed.
    PacketYaml(String),
    /// The base alpha packet YAML could not be parsed.
    BasePacketYaml(String),
    /// The catalog YAML could not be parsed.
    CatalogYaml(String),
    /// The state-row YAML could not be parsed.
    StateRowsYaml(String),
    /// A filesystem read failed while loading explicit paths.
    Io {
        /// Path that failed to read.
        path: String,
        /// Redaction-safe I/O error detail.
        detail: String,
    },
    /// The packet parsed but failed one or more trust checks. Reuses the
    /// alpha violation row so support/release consumers read one shape.
    Invalid(Vec<FitnessPacketAlphaViolation>),
}

impl fmt::Display for FitnessPacketBetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketYaml(detail) => {
                write!(
                    f,
                    "failed to parse beta release-candidate packet YAML: {detail}"
                )
            }
            Self::BasePacketYaml(detail) => {
                write!(
                    f,
                    "failed to parse base protected fitness packet YAML: {detail}"
                )
            }
            Self::CatalogYaml(detail) => {
                write!(
                    f,
                    "failed to parse protected fitness catalog YAML: {detail}"
                )
            }
            Self::StateRowsYaml(detail) => {
                write!(f, "failed to parse fitness state-row YAML: {detail}")
            }
            Self::Io { path, detail } => write!(f, "failed to read {path}: {detail}"),
            Self::Invalid(violations) => {
                if let Some(first) = violations.first() {
                    write!(
                        f,
                        "invalid beta release-candidate packet: {} at {}",
                        first.check_id, first.reference
                    )
                } else {
                    write!(f, "invalid beta release-candidate packet")
                }
            }
        }
    }
}

impl std::error::Error for FitnessPacketBetaError {}

/// One release-candidate threshold row layered over a base protected
/// function. The bar is owned by the catalog; this row records the frozen
/// release-candidate posture (within bar or over bar) and the authority that
/// may waive it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessReleaseCandidateThreshold {
    /// Base protected-function row this threshold governs.
    #[serde(default)]
    pub protected_function_ref: String,
    /// Catalog row the threshold mirrors.
    #[serde(default)]
    pub catalog_row_ref: Option<String>,
    /// Threshold interpretation mode copied from the catalog row.
    #[serde(default)]
    pub threshold_mode: String,
    /// Comparator from the closed release-candidate comparator vocabulary.
    #[serde(default)]
    pub comparator: String,
    /// Redaction-safe release-candidate bar label.
    #[serde(default)]
    pub release_candidate_bar: String,
    /// Redaction-safe measured-value label.
    #[serde(default)]
    pub measured_value: String,
    /// Whether the measured value sits within the release-candidate bar.
    #[serde(default)]
    pub within_release_candidate_bar: bool,
    /// Forum that may waive a breach of this bar.
    #[serde(default)]
    pub waiver_authority_ref: String,
}

/// The frozen beta release-candidate packet. Reuses the alpha review packet
/// verbatim as its base and adds release-candidate thresholds.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FitnessPacketBeta {
    /// Packet schema version.
    #[serde(default)]
    pub schema_version: u32,
    /// Record-kind discriminator.
    #[serde(default)]
    pub record_kind: String,
    /// Stable packet id.
    #[serde(default)]
    pub packet_id: String,
    /// Packet revision number.
    #[serde(default)]
    pub packet_revision: u32,
    /// Current review state.
    #[serde(default)]
    pub packet_state: String,
    /// Candidate stage under review.
    #[serde(default)]
    pub candidate_stage: String,
    /// RFC 3339 UTC timestamp for the packet evaluation.
    #[serde(default)]
    pub as_of: String,
    /// Primary owner for the packet.
    #[serde(default)]
    pub owner_dri: String,
    /// Evidence owner for release/support follow-up.
    #[serde(default)]
    pub evidence_owner: String,
    /// Redaction-safe packet summary.
    #[serde(default)]
    pub overall_summary: String,
    /// Repository-relative ref to the base review packet this record reuses.
    #[serde(default)]
    pub base_packet_ref: String,
    /// Release-candidate thresholds, one per catalog-linked protected function.
    #[serde(default)]
    pub release_candidate_thresholds: Vec<FitnessReleaseCandidateThreshold>,
    /// Base review packet, loaded from `base_packet_ref` and validated by the
    /// alpha consumer unchanged. Not deserialized from the beta document.
    #[serde(skip)]
    pub base: FitnessPacketAlpha,
}

impl FitnessPacketBeta {
    /// Parses and validates a beta packet from YAML strings.
    ///
    /// # Errors
    ///
    /// Returns a parse error when any YAML document is malformed, or
    /// [`FitnessPacketBetaError::Invalid`] when validation fails.
    pub fn from_yaml_documents(
        beta_yaml: &str,
        base_packet_yaml: &str,
        catalog_yaml: &str,
        state_rows_yaml: &str,
    ) -> Result<Self, FitnessPacketBetaError> {
        let mut packet = serde_yaml::from_str::<Self>(beta_yaml)
            .map_err(|err| FitnessPacketBetaError::PacketYaml(err.to_string()))?;
        packet.base = serde_yaml::from_str::<FitnessPacketAlpha>(base_packet_yaml)
            .map_err(|err| FitnessPacketBetaError::BasePacketYaml(err.to_string()))?;
        let catalog = serde_yaml::from_str::<FitnessFunctionCatalog>(catalog_yaml)
            .map_err(|err| FitnessPacketBetaError::CatalogYaml(err.to_string()))?;
        let state_rows = serde_yaml::from_str::<FitnessStateRows>(state_rows_yaml)
            .map_err(|err| FitnessPacketBetaError::StateRowsYaml(err.to_string()))?;

        let violations = packet.validate_with_catalogs(&catalog, &state_rows);
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(FitnessPacketBetaError::Invalid(violations))
        }
    }

    /// Loads and validates a beta packet from explicit filesystem paths.
    ///
    /// # Errors
    ///
    /// Returns a typed I/O, parse, or validation error.
    pub fn from_paths(
        beta_packet_path: impl AsRef<Path>,
        base_packet_path: impl AsRef<Path>,
        catalog_path: impl AsRef<Path>,
        state_rows_path: impl AsRef<Path>,
    ) -> Result<Self, FitnessPacketBetaError> {
        let beta_yaml = read_to_string(beta_packet_path.as_ref()).map_err(beta_io_from_alpha_io)?;
        let base_yaml = read_to_string(base_packet_path.as_ref()).map_err(beta_io_from_alpha_io)?;
        let catalog_yaml = read_to_string(catalog_path.as_ref()).map_err(beta_io_from_alpha_io)?;
        let state_rows_yaml =
            read_to_string(state_rows_path.as_ref()).map_err(beta_io_from_alpha_io)?;

        Self::from_yaml_documents(&beta_yaml, &base_yaml, &catalog_yaml, &state_rows_yaml)
    }

    /// Validates this beta packet against the catalog and state-row vocabulary.
    ///
    /// Runs the full alpha validation over the reused base packet, then adds
    /// the beta-specific release-candidate threshold checks.
    pub fn validate_with_catalogs(
        &self,
        catalog: &FitnessFunctionCatalog,
        state_rows: &FitnessStateRows,
    ) -> Vec<FitnessPacketAlphaViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROTECTED_FITNESS_PACKET_BETA_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "fitness_packet_beta.schema_version",
                &self.packet_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND {
            push_violation(
                &mut violations,
                "fitness_packet_beta.record_kind",
                &self.packet_id,
                "record_kind must be protected_fitness_release_candidate_packet",
            );
        }
        require_non_empty(
            &mut violations,
            "fitness_packet_beta.packet_id",
            &self.packet_id,
            &self.packet_id,
        );
        require_non_empty(
            &mut violations,
            "fitness_packet_beta.owner_dri",
            &self.owner_dri,
            &self.packet_id,
        );
        require_non_empty(
            &mut violations,
            "fitness_packet_beta.candidate_stage",
            &self.candidate_stage,
            &self.packet_id,
        );
        if parse_datetime(&self.as_of).is_none() {
            push_violation(
                &mut violations,
                "fitness_packet_beta.as_of",
                &self.packet_id,
                "as_of must be an RFC 3339 UTC timestamp",
            );
        }
        if self.base_packet_ref != CURRENT_PROTECTED_FITNESS_PACKET_ALPHA_PATH {
            push_violation(
                &mut violations,
                "fitness_packet_beta.base_packet_ref",
                &self.packet_id,
                "base_packet_ref must name the checked-in protected fitness packet",
            );
        }

        // Reuse the full alpha validation over the base review packet.
        violations.extend(self.base.validate_with_catalogs(catalog, state_rows));

        self.validate_release_candidate_thresholds(catalog, &mut violations);

        violations
    }

    /// Projects the release-candidate posture into a metadata-only summary.
    pub fn release_candidate_summary(&self) -> FitnessReleaseCandidateSummary {
        let rows_by_ref = self
            .base
            .protected_function_rows
            .iter()
            .map(|row| (row.protected_function_ref.as_str(), row))
            .collect::<BTreeMap<_, _>>();

        let mut within_bar_count = 0;
        let mut over_bar_count = 0;
        let mut over_bar_held_by_active_waiver_count = 0;
        let mut expired_waiver_count = 0;
        for threshold in &self.release_candidate_thresholds {
            if threshold.within_release_candidate_bar {
                within_bar_count += 1;
            } else {
                over_bar_count += 1;
            }
            if let Some(row) = rows_by_ref.get(threshold.protected_function_ref.as_str()) {
                if !threshold.within_release_candidate_bar
                    && row.waiver.waiver_state == "active_waiver"
                {
                    over_bar_held_by_active_waiver_count += 1;
                }
                if row.waiver.waiver_state == "expired_waiver" {
                    expired_waiver_count += 1;
                }
            }
        }

        FitnessReleaseCandidateSummary {
            schema_version: PROTECTED_FITNESS_PACKET_BETA_SCHEMA_VERSION,
            record_kind: PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND.to_owned(),
            packet_ref: CURRENT_PROTECTED_FITNESS_PACKET_BETA_PATH.to_owned(),
            base_packet_ref: self.base_packet_ref.clone(),
            threshold_count: self.release_candidate_thresholds.len() as u32,
            within_bar_count,
            over_bar_count,
            over_bar_held_by_active_waiver_count,
            expired_waiver_count,
        }
    }

    fn validate_release_candidate_thresholds(
        &self,
        catalog: &FitnessFunctionCatalog,
        violations: &mut Vec<FitnessPacketAlphaViolation>,
    ) {
        let catalog_by_id = catalog
            .rows
            .iter()
            .filter(|row| !row.id.trim().is_empty())
            .map(|row| (row.id.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        let rows_by_ref = self
            .base
            .protected_function_rows
            .iter()
            .map(|row| (row.protected_function_ref.as_str(), row))
            .collect::<BTreeMap<_, _>>();
        let threshold_refs = self
            .release_candidate_thresholds
            .iter()
            .map(|threshold| threshold.protected_function_ref.as_str())
            .collect::<BTreeSet<_>>();

        // Every catalog-linked base row must carry a release-candidate bar.
        for row in &self.base.protected_function_rows {
            if row.catalog_row_ref.is_some()
                && !threshold_refs.contains(row.protected_function_ref.as_str())
            {
                push_violation(
                    violations,
                    "release_candidate_thresholds.coverage",
                    row.reference(),
                    "every catalog-linked protected function must carry a release-candidate threshold",
                );
            }
        }

        let mut seen = BTreeSet::new();
        for threshold in &self.release_candidate_thresholds {
            let reference = if threshold.protected_function_ref.trim().is_empty() {
                "<missing protected_function_ref>".to_owned()
            } else {
                threshold.protected_function_ref.clone()
            };
            if !seen.insert(reference.clone()) {
                push_violation(
                    violations,
                    "release_candidate_thresholds.duplicate",
                    &reference,
                    "release-candidate threshold protected_function_ref values must be unique",
                );
            }
            if !RELEASE_CANDIDATE_COMPARATORS.contains(&threshold.comparator.as_str()) {
                push_violation(
                    violations,
                    "release_candidate_thresholds.comparator",
                    &reference,
                    "comparator must resolve through the release-candidate comparator vocabulary",
                );
            }
            require_non_empty(
                violations,
                "release_candidate_thresholds.release_candidate_bar",
                &threshold.release_candidate_bar,
                &reference,
            );
            require_non_empty(
                violations,
                "release_candidate_thresholds.measured_value",
                &threshold.measured_value,
                &reference,
            );

            let Some(base_row) = rows_by_ref.get(threshold.protected_function_ref.as_str()) else {
                push_violation(
                    violations,
                    "release_candidate_thresholds.unknown_function",
                    &reference,
                    "threshold protected_function_ref must resolve to a base protected_function_row",
                );
                continue;
            };

            match threshold.catalog_row_ref.as_deref() {
                None => push_violation(
                    violations,
                    "release_candidate_thresholds.catalog_row_ref",
                    &reference,
                    "release-candidate thresholds must name a catalog_row_ref",
                ),
                Some(catalog_ref) => {
                    match catalog_by_id.get(catalog_ref) {
                        None => push_violation(
                            violations,
                            "release_candidate_thresholds.catalog_row_ref",
                            &reference,
                            "catalog_row_ref must resolve through the protected fitness catalog",
                        ),
                        Some(catalog_row) => {
                            if catalog_row.threshold_mode != threshold.threshold_mode {
                                push_violation(
                                    violations,
                                    "release_candidate_thresholds.threshold_mode",
                                    &reference,
                                    "threshold_mode must match the catalog threshold_mode",
                                );
                            }
                            if catalog_row.waiver_authority != threshold.waiver_authority_ref {
                                push_violation(
                                    violations,
                                    "release_candidate_thresholds.waiver_authority_ref",
                                    &reference,
                                    "waiver_authority_ref must match the catalog waiver_authority",
                                );
                            }
                        }
                    }
                    if base_row.catalog_row_ref.as_deref() != Some(catalog_ref) {
                        push_violation(
                            violations,
                            "release_candidate_thresholds.catalog_row_ref",
                            &reference,
                            "catalog_row_ref must match the base row catalog_row_ref",
                        );
                    }
                }
            }

            if threshold.waiver_authority_ref != base_row.waiver_authority_ref {
                push_violation(
                    violations,
                    "release_candidate_thresholds.waiver_authority_ref",
                    &reference,
                    "waiver_authority_ref must match the base row waiver_authority_ref",
                );
            }

            let waiver_state = base_row.waiver.waiver_state.as_str();
            let current_result = base_row.current_result.as_str();

            if !threshold.within_release_candidate_bar {
                if waiver_state == "active_waiver" {
                    // An over-bar metric held by an active waiver must stay
                    // visible: the waiver record must be present and the row
                    // may not render as a clean pass.
                    if !non_empty_option(&base_row.waiver.waiver_record_ref) {
                        push_violation(
                            violations,
                            "release_candidate_thresholds.active_waiver_visibility",
                            &reference,
                            "an over-bar metric held by an active waiver must carry a visible waiver_record_ref",
                        );
                    }
                    if current_result == "passing" {
                        push_violation(
                            violations,
                            "release_candidate_thresholds.active_waiver_visibility",
                            &reference,
                            "an over-bar metric held by an active waiver must render its waived state, not passing",
                        );
                    }
                } else if matches!(current_result, "passing" | "waived") {
                    push_violation(
                        violations,
                        "release_candidate_thresholds.over_threshold_without_active_waiver",
                        &reference,
                        "a metric over its release-candidate bar without an active waiver must not render passing or waived",
                    );
                }
            }

            if waiver_state == "expired_waiver"
                && matches!(current_result, "passing" | "warning" | "waived")
            {
                push_violation(
                    violations,
                    "release_candidate_thresholds.expired_waiver_degrades",
                    &reference,
                    "an expired waiver must degrade the protected function instead of holding it as passing, warning, or waived",
                );
            }
        }
    }
}

/// Metadata-only release-candidate posture summary consumed by support
/// export and release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FitnessReleaseCandidateSummary {
    /// Summary schema version.
    pub schema_version: u32,
    /// Stable record-kind tag copied from the beta packet.
    pub record_kind: String,
    /// Repository-relative beta packet ref.
    pub packet_ref: String,
    /// Repository-relative base review packet ref.
    pub base_packet_ref: String,
    /// Number of release-candidate threshold rows.
    pub threshold_count: u32,
    /// Number of metrics within their release-candidate bar.
    pub within_bar_count: u32,
    /// Number of metrics over their release-candidate bar.
    pub over_bar_count: u32,
    /// Number of over-bar metrics held open by an active waiver.
    pub over_bar_held_by_active_waiver_count: u32,
    /// Number of threshold rows whose waiver is expired.
    pub expired_waiver_count: u32,
}

fn beta_io_from_alpha_io(err: FitnessPacketAlphaError) -> FitnessPacketBetaError {
    match err {
        FitnessPacketAlphaError::Io { path, detail } => FitnessPacketBetaError::Io { path, detail },
        other => FitnessPacketBetaError::PacketYaml(other.to_string()),
    }
}

fn read_to_string(path: &Path) -> Result<String, FitnessPacketAlphaError> {
    fs::read_to_string(path).map_err(|err| FitnessPacketAlphaError::Io {
        path: path.display().to_string(),
        detail: err.to_string(),
    })
}

fn push_violation(
    violations: &mut Vec<FitnessPacketAlphaViolation>,
    check_id: impl Into<String>,
    reference: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(FitnessPacketAlphaViolation {
        check_id: check_id.into(),
        reference: reference.into(),
        message: message.into(),
    });
}

fn require_non_empty(
    violations: &mut Vec<FitnessPacketAlphaViolation>,
    check_id: &str,
    value: &str,
    reference: &str,
) {
    if value.trim().is_empty() {
        push_violation(
            violations,
            check_id,
            reference,
            "required string field must be non-empty",
        );
    }
}

fn parse_datetime(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &Rfc3339).ok()
}

fn parse_day_duration(value: &str) -> Option<Duration> {
    let days = value.strip_prefix("P")?.strip_suffix("D")?.parse().ok()?;
    Some(Duration::days(days))
}

fn non_empty_option(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|value| !value.trim().is_empty())
}

fn count_by<'a>(values: impl Iterator<Item = &'a str>) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for value in values {
        *counts.entry(value.to_owned()).or_insert(0) += 1;
    }
    counts
}
