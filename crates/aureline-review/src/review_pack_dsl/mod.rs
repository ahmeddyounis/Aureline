//! Review-pack DSL alpha — the versioned, repo-defined declaration of a
//! review pack the local and CI review harnesses both consume. The record
//! is pre-execution review truth: it names what would run, who owns the
//! relevant scopes, which fields are unsupported on the current DSL
//! version, and how local/CI parity is claimed, without itself mutating
//! any branch, worktree, or remote. A reviewer can answer five questions
//! from a single row before either the local or CI harness executes:
//!
//! 1. **Which pack am I about to run, and where does it live?**
//!    `review_pack_id` and `repo_anchor_ref` pin the pack identity and
//!    repo anchor without exporting raw paths.
//! 2. **Who authored or signed it?** `pack_authority_class` distinguishes
//!    first-party, team-shared, partner-signed, and uncertified-community
//!    packs so authority is never silently widened.
//! 3. **What checks would run?** `checks` enumerates each check with its
//!    kind, severity class, parity class, and execution class.
//! 4. **Who owns the affected scopes?** `ownership_hints` quote
//!    closed-vocabulary scope kinds and opaque owner refs.
//! 5. **What is local-only, CI-only, or unsupported?**
//!    `parity_observations` and `unsupported_fields` make the answers
//!    explicit instead of leaving them to ad hoc plugin behavior.
//!
//! The companion schema lives at
//! `schemas/review/review_pack.schema.json`. The reviewer doc lives at
//! `docs/review/m3/review_pack_dsl_alpha.md`. Canonical fixtures live
//! under `fixtures/review/m3/review_pack_dsl/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every alpha review-pack record.
pub const REVIEW_PACK_ALPHA_SCHEMA_VERSION: u32 = 1;

/// DSL version for every alpha review-pack record.
pub const REVIEW_PACK_ALPHA_DSL_VERSION: u32 = 1;

/// Record-kind discriminator for [`ReviewPackRecord`].
pub const REVIEW_PACK_ALPHA_RECORD_KIND: &str = "review_pack_alpha_record";

/// Closed set of pack-authority classes.
pub const REVIEW_PACK_AUTHORITY_CLASSES: &[&str] = &[
    "repo_first_party",
    "repo_team_shared",
    "repo_partner_signed",
    "repo_uncertified_community",
    "pack_authority_unknown_requires_review",
];

/// Closed set of check-kind classes.
pub const REVIEW_PACK_CHECK_KINDS: &[&str] = &[
    "policy_lint",
    "schema_validation",
    "ownership_review",
    "test_replay",
    "doc_freshness",
    "format_check",
    "custom_local",
    "check_kind_unknown_requires_review",
];

/// Closed set of check-severity classes.
pub const REVIEW_PACK_SEVERITY_CLASSES: &[&str] = &[
    "advisory",
    "blocking",
    "informational",
    "severity_unknown_requires_review",
];

/// Closed set of local/CI parity classes.
pub const REVIEW_PACK_PARITY_CLASSES: &[&str] = &[
    "local_and_ci_parity",
    "ci_only_documented",
    "local_only_documented",
    "parity_unknown_requires_review",
];

/// Closed set of execution classes.
pub const REVIEW_PACK_EXECUTION_CLASSES: &[&str] = &[
    "deterministic_replay",
    "stateful_local_only",
    "stateful_ci_managed",
    "execution_class_unknown_requires_review",
];

/// Closed set of ownership-scope kinds.
pub const REVIEW_PACK_OWNERSHIP_SCOPE_KINDS: &[&str] = &[
    "path_glob_first_party",
    "path_glob_team",
    "path_glob_external_partner",
    "path_glob_uncertified_community",
    "ownership_scope_unknown_requires_review",
];

/// Closed set of unsupported-field classes.
pub const REVIEW_PACK_UNSUPPORTED_FIELD_CLASSES: &[&str] = &[
    "future_dsl_version",
    "vendor_specific_extension",
    "experimental_local_only",
    "deprecated_pending_removal",
    "unsupported_class_unknown_requires_review",
];

/// Closed set of consumer surfaces.
pub const REVIEW_PACK_CONSUMER_SURFACES: &[&str] = &[
    "review_pack_inspector",
    "review_preview",
    "cli_headless_entry",
    "support_export",
    "docs_review",
    "activity_center",
];

/// One declarative check entry inside a review pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackCheck {
    pub check_id: String,
    pub check_kind: String,
    pub display_label: String,
    pub summary: String,
    pub severity_class: String,
    pub parity_class: String,
    pub execution_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity_note: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ownership_scope_refs: Vec<String>,
}

/// One ownership hint declaring who owns a closed-scope vocabulary class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackOwnershipHint {
    pub ownership_scope_id: String,
    pub ownership_scope_kind: String,
    pub display_label: String,
    pub owner_ref: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity_note: Option<String>,
}

/// One parity observation explaining how local and CI execution relate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityObservation {
    pub parity_class: String,
    pub summary: String,
}

/// One unsupported-field declaration carried alongside the pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackUnsupportedField {
    pub field_path: String,
    pub unsupported_class: String,
    pub summary: String,
}

/// Closed support-export disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSupportExport {
    pub export_packet_refs: Vec<String>,
    pub raw_path_export_allowed: bool,
    pub raw_glob_body_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_check_output_export_allowed: bool,
    pub redaction_class: String,
}

/// Pre-execution review invariants the record must claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackReviewInvariants {
    pub repo_anchor_pinned: bool,
    pub checks_pinned: bool,
    pub ownership_hints_pinned: bool,
    pub local_ci_parity_declared: bool,
    pub unsupported_fields_declared: bool,
    pub no_hidden_writes: bool,
}

/// One alpha review-pack record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub dsl_version: u32,
    pub review_pack_id: String,
    pub repo_anchor_ref: String,
    pub display_label: String,
    pub summary: String,
    pub pack_authority_class: String,
    pub operator_caveat: String,
    pub checks: Vec<ReviewPackCheck>,
    pub ownership_hints: Vec<ReviewPackOwnershipHint>,
    pub parity_observations: Vec<ReviewPackParityObservation>,
    pub unsupported_fields: Vec<ReviewPackUnsupportedField>,
    pub consumer_surfaces: Vec<String>,
    pub support_export: ReviewPackSupportExport,
    pub review_invariants: ReviewPackReviewInvariants,
    pub minted_at: String,
}

/// Compact check projection consumed by shell, CLI / headless, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackCheckProjection {
    pub check_id: String,
    pub check_kind: String,
    pub display_label: String,
    pub summary: String,
    pub severity_class: String,
    pub parity_class: String,
    pub execution_class: String,
    pub ownership_scope_refs: Vec<String>,
}

/// Compact ownership-hint projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackOwnershipProjection {
    pub ownership_scope_id: String,
    pub ownership_scope_kind: String,
    pub display_label: String,
    pub owner_ref: String,
    pub summary: String,
}

/// Compact projection consumed by the first review-pack inspector surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackProjection {
    pub review_pack_id: String,
    pub repo_anchor_ref: String,
    pub display_label: String,
    pub summary: String,
    pub pack_authority_class: String,
    pub operator_caveat: String,
    pub dsl_version: u32,
    pub schema_version: u32,
    pub checks: Vec<ReviewPackCheckProjection>,
    pub ownership_hints: Vec<ReviewPackOwnershipProjection>,
    pub parity_observations: Vec<ReviewPackParityObservation>,
    pub unsupported_fields: Vec<ReviewPackUnsupportedField>,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_path_export_allowed: bool,
    pub raw_glob_body_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_check_output_export_allowed: bool,
    pub local_ci_parity_classes: Vec<String>,
    pub blocking_check_count: usize,
    pub local_only_check_count: usize,
    pub ci_only_check_count: usize,
}

impl ReviewPackRecord {
    /// Validates the record against the alpha review-pack contract.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewPackValidationError`] when any frozen guarantee is
    /// violated.
    pub fn validate(&self) -> Result<(), ReviewPackValidationError> {
        validate_record(self)
    }

    /// Projects the record into the compact review-pack inspector row.
    pub fn project(&self) -> ReviewPackProjection {
        let checks: Vec<ReviewPackCheckProjection> = self
            .checks
            .iter()
            .map(|check| ReviewPackCheckProjection {
                check_id: check.check_id.clone(),
                check_kind: check.check_kind.clone(),
                display_label: check.display_label.clone(),
                summary: check.summary.clone(),
                severity_class: check.severity_class.clone(),
                parity_class: check.parity_class.clone(),
                execution_class: check.execution_class.clone(),
                ownership_scope_refs: check.ownership_scope_refs.clone(),
            })
            .collect();
        let ownership_hints: Vec<ReviewPackOwnershipProjection> = self
            .ownership_hints
            .iter()
            .map(|hint| ReviewPackOwnershipProjection {
                ownership_scope_id: hint.ownership_scope_id.clone(),
                ownership_scope_kind: hint.ownership_scope_kind.clone(),
                display_label: hint.display_label.clone(),
                owner_ref: hint.owner_ref.clone(),
                summary: hint.summary.clone(),
            })
            .collect();
        let blocking_check_count = self
            .checks
            .iter()
            .filter(|c| c.severity_class == "blocking")
            .count();
        let local_only_check_count = self
            .checks
            .iter()
            .filter(|c| c.parity_class == "local_only_documented")
            .count();
        let ci_only_check_count = self
            .checks
            .iter()
            .filter(|c| c.parity_class == "ci_only_documented")
            .count();
        let mut local_ci_parity_classes: Vec<String> = self
            .parity_observations
            .iter()
            .map(|o| o.parity_class.clone())
            .collect();
        local_ci_parity_classes.sort();
        local_ci_parity_classes.dedup();
        ReviewPackProjection {
            review_pack_id: self.review_pack_id.clone(),
            repo_anchor_ref: self.repo_anchor_ref.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            pack_authority_class: self.pack_authority_class.clone(),
            operator_caveat: self.operator_caveat.clone(),
            dsl_version: self.dsl_version,
            schema_version: self.schema_version,
            checks,
            ownership_hints,
            parity_observations: self.parity_observations.clone(),
            unsupported_fields: self.unsupported_fields.clone(),
            consumer_surfaces: self.consumer_surfaces.clone(),
            support_export_refs: self.support_export.export_packet_refs.clone(),
            redaction_class: self.support_export.redaction_class.clone(),
            raw_path_export_allowed: self.support_export.raw_path_export_allowed,
            raw_glob_body_export_allowed: self.support_export.raw_glob_body_export_allowed,
            raw_command_export_allowed: self.support_export.raw_command_export_allowed,
            raw_check_output_export_allowed: self
                .support_export
                .raw_check_output_export_allowed,
            local_ci_parity_classes,
            blocking_check_count,
            local_only_check_count,
            ci_only_check_count,
        }
    }
}

/// Validation failure for a review-pack record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackValidationError {
    message: String,
}

impl ReviewPackValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ReviewPackValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "review-pack validation error: {}", self.message)
    }
}

impl std::error::Error for ReviewPackValidationError {}

/// Error returned when a review-pack JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha review-pack contract.
    Validation(ReviewPackValidationError),
}

impl ReviewPackError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for ReviewPackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "review-pack JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ReviewPackError {}

/// Parses and validates an alpha review-pack JSON payload, returning the
/// compact projection on success.
///
/// # Errors
///
/// Returns [`ReviewPackError::Json`] when the payload is not valid JSON
/// for the record, and [`ReviewPackError::Validation`] when any frozen
/// alpha guarantee is violated.
pub fn project_review_pack(payload: &str) -> Result<ReviewPackProjection, ReviewPackError> {
    let record: ReviewPackRecord =
        serde_json::from_str(payload).map_err(|err| ReviewPackError::Json(err.to_string()))?;
    record.validate().map_err(ReviewPackError::Validation)?;
    Ok(record.project())
}

fn validate_record(record: &ReviewPackRecord) -> Result<(), ReviewPackValidationError> {
    require_equal("record_kind", REVIEW_PACK_ALPHA_RECORD_KIND, &record.record_kind)?;
    if record.schema_version != REVIEW_PACK_ALPHA_SCHEMA_VERSION {
        return Err(ReviewPackValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, REVIEW_PACK_ALPHA_SCHEMA_VERSION
        )));
    }
    if record.dsl_version != REVIEW_PACK_ALPHA_DSL_VERSION {
        return Err(ReviewPackValidationError::new(format!(
            "dsl_version is {}, expected {}",
            record.dsl_version, REVIEW_PACK_ALPHA_DSL_VERSION
        )));
    }
    require_non_empty("review_pack_id", &record.review_pack_id)?;
    require_non_empty("repo_anchor_ref", &record.repo_anchor_ref)?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_one_of(
        "pack_authority_class",
        REVIEW_PACK_AUTHORITY_CLASSES,
        &record.pack_authority_class,
    )?;
    require_non_empty("operator_caveat", &record.operator_caveat)?;
    require_non_empty("minted_at", &record.minted_at)?;
    validate_checks(&record.checks)?;
    validate_ownership_hints(&record.ownership_hints)?;
    validate_parity_observations(&record.parity_observations)?;
    validate_unsupported_fields(&record.unsupported_fields)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    validate_review_invariants(&record.review_invariants)?;
    cross_check_ownership_refs(&record.checks, &record.ownership_hints)?;
    cross_check_parity_coverage(record)?;
    Ok(())
}

fn validate_checks(checks: &[ReviewPackCheck]) -> Result<(), ReviewPackValidationError> {
    if checks.is_empty() {
        return Err(ReviewPackValidationError::new(
            "checks must list at least one check definition",
        ));
    }
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for check in checks {
        require_non_empty("checks[].check_id", &check.check_id)?;
        if !seen_ids.insert(check.check_id.as_str()) {
            return Err(ReviewPackValidationError::new(format!(
                "checks contains a duplicate check_id: {}",
                check.check_id
            )));
        }
        require_one_of("checks[].check_kind", REVIEW_PACK_CHECK_KINDS, &check.check_kind)?;
        require_non_empty("checks[].display_label", &check.display_label)?;
        require_non_empty("checks[].summary", &check.summary)?;
        require_one_of(
            "checks[].severity_class",
            REVIEW_PACK_SEVERITY_CLASSES,
            &check.severity_class,
        )?;
        require_one_of(
            "checks[].parity_class",
            REVIEW_PACK_PARITY_CLASSES,
            &check.parity_class,
        )?;
        require_one_of(
            "checks[].execution_class",
            REVIEW_PACK_EXECUTION_CLASSES,
            &check.execution_class,
        )?;
        require_unique("checks[].ownership_scope_refs", &check.ownership_scope_refs)?;
    }
    Ok(())
}

fn validate_ownership_hints(
    hints: &[ReviewPackOwnershipHint],
) -> Result<(), ReviewPackValidationError> {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for hint in hints {
        require_non_empty("ownership_hints[].ownership_scope_id", &hint.ownership_scope_id)?;
        if !seen_ids.insert(hint.ownership_scope_id.as_str()) {
            return Err(ReviewPackValidationError::new(format!(
                "ownership_hints contains a duplicate ownership_scope_id: {}",
                hint.ownership_scope_id
            )));
        }
        require_one_of(
            "ownership_hints[].ownership_scope_kind",
            REVIEW_PACK_OWNERSHIP_SCOPE_KINDS,
            &hint.ownership_scope_kind,
        )?;
        require_non_empty("ownership_hints[].display_label", &hint.display_label)?;
        require_non_empty("ownership_hints[].owner_ref", &hint.owner_ref)?;
        require_non_empty("ownership_hints[].summary", &hint.summary)?;
    }
    Ok(())
}

fn validate_parity_observations(
    observations: &[ReviewPackParityObservation],
) -> Result<(), ReviewPackValidationError> {
    if observations.is_empty() {
        return Err(ReviewPackValidationError::new(
            "parity_observations must declare at least one local/CI parity observation",
        ));
    }
    let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
    for observation in observations {
        require_one_of(
            "parity_observations[].parity_class",
            REVIEW_PACK_PARITY_CLASSES,
            &observation.parity_class,
        )?;
        require_non_empty("parity_observations[].summary", &observation.summary)?;
        if !seen.insert((observation.parity_class.as_str(), observation.summary.as_str())) {
            return Err(ReviewPackValidationError::new(
                "parity_observations contains duplicate parity_class/summary pair",
            ));
        }
    }
    Ok(())
}

fn validate_unsupported_fields(
    fields: &[ReviewPackUnsupportedField],
) -> Result<(), ReviewPackValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for field in fields {
        require_non_empty("unsupported_fields[].field_path", &field.field_path)?;
        if !seen.insert(field.field_path.as_str()) {
            return Err(ReviewPackValidationError::new(format!(
                "unsupported_fields contains a duplicate field_path: {}",
                field.field_path
            )));
        }
        require_one_of(
            "unsupported_fields[].unsupported_class",
            REVIEW_PACK_UNSUPPORTED_FIELD_CLASSES,
            &field.unsupported_class,
        )?;
        require_non_empty("unsupported_fields[].summary", &field.summary)?;
    }
    Ok(())
}

fn validate_consumer_surfaces(surfaces: &[String]) -> Result<(), ReviewPackValidationError> {
    if surfaces.is_empty() {
        return Err(ReviewPackValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of("consumer_surfaces[]", REVIEW_PACK_CONSUMER_SURFACES, surface)?;
    }
    if !surfaces.iter().any(|s| s == "review_pack_inspector") {
        return Err(ReviewPackValidationError::new(
            "consumer_surfaces must include review_pack_inspector so the first product surface stays wired",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &ReviewPackSupportExport,
) -> Result<(), ReviewPackValidationError> {
    if export.raw_path_export_allowed
        || export.raw_glob_body_export_allowed
        || export.raw_command_export_allowed
        || export.raw_check_output_export_allowed
    {
        return Err(ReviewPackValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    require_unique(
        "support_export.export_packet_refs",
        &export.export_packet_refs,
    )?;
    Ok(())
}

fn validate_review_invariants(
    invariants: &ReviewPackReviewInvariants,
) -> Result<(), ReviewPackValidationError> {
    if !invariants.repo_anchor_pinned
        || !invariants.checks_pinned
        || !invariants.ownership_hints_pinned
        || !invariants.local_ci_parity_declared
        || !invariants.unsupported_fields_declared
        || !invariants.no_hidden_writes
    {
        return Err(ReviewPackValidationError::new(
            "review_invariants must all be true; the review-pack record is a pre-execution review record",
        ));
    }
    Ok(())
}

fn cross_check_ownership_refs(
    checks: &[ReviewPackCheck],
    hints: &[ReviewPackOwnershipHint],
) -> Result<(), ReviewPackValidationError> {
    let known: BTreeSet<&str> = hints.iter().map(|h| h.ownership_scope_id.as_str()).collect();
    for check in checks {
        for scope_ref in &check.ownership_scope_refs {
            if !known.contains(scope_ref.as_str()) {
                return Err(ReviewPackValidationError::new(format!(
                    "check {} references ownership_scope {} that is not declared in ownership_hints",
                    check.check_id, scope_ref
                )));
            }
        }
    }
    Ok(())
}

fn cross_check_parity_coverage(
    record: &ReviewPackRecord,
) -> Result<(), ReviewPackValidationError> {
    let declared_classes: BTreeSet<&str> = record
        .parity_observations
        .iter()
        .map(|o| o.parity_class.as_str())
        .collect();
    for check in &record.checks {
        if !declared_classes.contains(check.parity_class.as_str()) {
            return Err(ReviewPackValidationError::new(format!(
                "check {} declares parity_class {} but no matching parity_observation is recorded",
                check.check_id, check.parity_class
            )));
        }
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), ReviewPackValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ReviewPackValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), ReviewPackValidationError> {
    if value.trim().is_empty() {
        Err(ReviewPackValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), ReviewPackValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(ReviewPackValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(label: &str, values: &[String]) -> Result<(), ReviewPackValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(ReviewPackValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_FIRST_PARTY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_dsl/first_party_local_and_ci_parity.json"
    ));
    const FIXTURE_PARTNER: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_dsl/partner_signed_ci_only_lane.json"
    ));
    const FIXTURE_COMMUNITY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_dsl/uncertified_community_local_only_lane.json"
    ));

    #[test]
    fn first_party_pack_projects() {
        let projection = project_review_pack(FIXTURE_FIRST_PARTY)
            .expect("first-party pack fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_first_party");
        assert!(projection
            .local_ci_parity_classes
            .iter()
            .any(|c| c == "local_and_ci_parity"));
        assert!(projection.blocking_check_count >= 1);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|s| s == "review_pack_inspector"));
    }

    #[test]
    fn partner_signed_ci_only_projects() {
        let projection = project_review_pack(FIXTURE_PARTNER)
            .expect("partner-signed CI-only fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_partner_signed");
        assert!(projection.ci_only_check_count >= 1);
    }

    #[test]
    fn uncertified_community_local_only_projects() {
        let projection = project_review_pack(FIXTURE_COMMUNITY)
            .expect("uncertified-community local-only fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_uncertified_community");
        assert!(projection.local_only_check_count >= 1);
        assert!(!projection.unsupported_fields.is_empty());
    }

    #[test]
    fn rejects_check_with_unknown_ownership_scope_ref() {
        let mut record: ReviewPackRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.checks[0]
            .ownership_scope_refs
            .push("ownership_scope:not_declared".to_string());
        let err = record
            .validate()
            .expect_err("must reject undeclared ownership scope refs");
        assert!(err.message().contains("not declared"));
    }

    #[test]
    fn rejects_check_with_unobserved_parity_class() {
        let mut record: ReviewPackRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.checks[0].parity_class = "ci_only_documented".to_string();
        record
            .parity_observations
            .retain(|o| o.parity_class != "ci_only_documented");
        let err = record
            .validate()
            .expect_err("must reject check parity_class without an observation");
        assert!(err.message().contains("parity_observation"));
    }

    #[test]
    fn rejects_raw_path_export() {
        let mut record: ReviewPackRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.support_export.raw_path_export_allowed = true;
        let err = record.validate().expect_err("must reject raw path export");
        assert!(err.message().contains("raw_"));
    }

    #[test]
    fn rejects_missing_review_pack_inspector_consumer() {
        let mut record: ReviewPackRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record
            .consumer_surfaces
            .retain(|surface| surface != "review_pack_inspector");
        let err = record
            .validate()
            .expect_err("must reject missing review_pack_inspector");
        assert!(err.message().contains("review_pack_inspector"));
    }

    #[test]
    fn rejects_wrong_record_kind_via_project() {
        let tampered =
            FIXTURE_FIRST_PARTY.replace("review_pack_alpha_record", "other_record_kind");
        match project_review_pack(&tampered) {
            Err(ReviewPackError::Validation(err)) => {
                assert!(err.message().contains("record_kind"));
            }
            other => panic!("expected validation failure, got {other:?}"),
        }
    }
}
