//! Review-pack parity-harness alpha — the post-execution comparison
//! between a local-lane run and a CI-derived run of one upstream
//! review-pack DSL record. The record is the durable, exportable row the
//! parity-harness inspector renders **after** both lanes have run (or
//! after one lane has documented-declined the pack). It binds the
//! upstream `review_pack_alpha` id, names the engaged or declined lanes,
//! projects per-check outcomes for each lane, declares the resulting
//! `parity_finding_class` per check, names any `drift_downgrades` that
//! lowered the row, and quotes the overall verdict the docs / support /
//! partner packets read. The record is pre-publication review truth for
//! the parity claim itself: every `drift_detected` finding **must** pair
//! with a `drift_downgrades` entry so a green claim can never be
//! silently preserved.
//!
//! A reviewer can answer four questions from one row before adopting a
//! pack's parity claim:
//!
//! 1. **Which upstream pack was harnessed?** `review_pack_ref` and
//!    `repo_anchor_ref` pin the source pack and repo anchor without
//!    exporting raw paths.
//! 2. **Which lanes engaged or declined?** `harness_lane_observations`
//!    names every lane with one of `lane_engaged`,
//!    `lane_declined_uncertified`, `lane_declined_unsupported`, or
//!    `lane_degraded_unknown_requires_review`.
//! 3. **What did each check do in each lane?** Each
//!    `check_parity_findings` row carries the expected parity class, the
//!    `local_outcome_class`, the `ci_outcome_class`, the observed parity
//!    class, the `parity_finding_class`, and a short reviewable sentence.
//! 4. **Did drift downgrade the row?** `drift_downgrades`,
//!    `row_downgrade_class`, and `overall_verdict_class` make the answer
//!    explicit instead of leaving it to ad hoc harness behavior.
//!
//! The companion schema lives at
//! `schemas/review/review_pack_parity_harness.schema.json`. The reviewer
//! doc lives at `docs/review/m3/review_pack_parity_alpha.md`. Canonical
//! fixtures live under `fixtures/review/m3/review_pack_harness/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every alpha parity-harness record.
pub const REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Harness version for every alpha parity-harness record.
pub const REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION: u32 = 1;

/// Record-kind discriminator for [`ReviewPackParityHarnessRecord`].
pub const REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND: &str =
    "review_pack_parity_harness_alpha_record";

/// Closed set of pack-authority classes (mirrors the upstream DSL).
pub const REVIEW_PACK_PARITY_HARNESS_AUTHORITY_CLASSES: &[&str] = &[
    "repo_first_party",
    "repo_team_shared",
    "repo_partner_signed",
    "repo_uncertified_community",
    "pack_authority_unknown_requires_review",
];

/// Closed set of harness-lane classes.
pub const REVIEW_PACK_PARITY_HARNESS_LANE_CLASSES: &[&str] = &["local_lane", "ci_lane"];

/// Closed set of harness-lane status classes.
pub const REVIEW_PACK_PARITY_HARNESS_LANE_STATUS_CLASSES: &[&str] = &[
    "lane_engaged",
    "lane_declined_uncertified",
    "lane_declined_unsupported",
    "lane_degraded_unknown_requires_review",
];

/// Closed set of per-lane check outcome classes.
pub const REVIEW_PACK_PARITY_HARNESS_LANE_OUTCOME_CLASSES: &[&str] = &[
    "passed_parity",
    "passed_with_drift_note",
    "failed_blocking",
    "failed_advisory",
    "declined_by_lane_documented",
    "skipped_by_lane_documented",
    "lane_outcome_unknown_requires_review",
];

/// Closed set of parity-finding classes per check.
pub const REVIEW_PACK_PARITY_HARNESS_PARITY_FINDING_CLASSES: &[&str] = &[
    "full_parity",
    "local_only_documented_match",
    "ci_only_documented_match",
    "declined_documented_match",
    "drift_detected",
    "parity_unknown_requires_review",
];

/// Closed set of row-downgrade classes.
pub const REVIEW_PACK_PARITY_HARNESS_ROW_DOWNGRADE_CLASSES: &[&str] = &[
    "no_downgrade",
    "downgraded_to_advisory",
    "downgraded_to_review_required",
    "suspended_pack",
];

/// Closed set of overall-verdict classes for the run.
pub const REVIEW_PACK_PARITY_HARNESS_OVERALL_VERDICT_CLASSES: &[&str] = &[
    "full_parity",
    "drift_downgraded",
    "lane_declined_documented",
    "parity_unknown_requires_review",
];

/// Closed set of consumer surfaces wired off the parity-harness record.
pub const REVIEW_PACK_PARITY_HARNESS_CONSUMER_SURFACES: &[&str] = &[
    "parity_harness_inspector",
    "review_pack_inspector",
    "review_preview",
    "cli_headless_entry",
    "support_export",
    "docs_review",
    "activity_center",
];

/// Closed set of expected/observed parity classes (mirrors the upstream DSL).
pub const REVIEW_PACK_PARITY_HARNESS_EXPECTED_PARITY_CLASSES: &[&str] = &[
    "local_and_ci_parity",
    "ci_only_documented",
    "local_only_documented",
    "parity_unknown_requires_review",
];

/// One lane observation pinning a lane class to a status class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessLaneObservation {
    pub lane_class: String,
    pub lane_status_class: String,
    pub summary: String,
}

/// One per-check parity finding describing local and CI lane outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessCheckFinding {
    pub check_ref: String,
    pub expected_parity_class: String,
    pub local_outcome_class: String,
    pub ci_outcome_class: String,
    pub observed_parity_class: String,
    pub parity_finding_class: String,
    pub summary: String,
}

/// One drift-downgrade pairing a check with a downgrade class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessDriftDowngrade {
    pub check_ref: String,
    pub downgrade_class: String,
    pub summary: String,
}

/// Closed support-export disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessSupportExport {
    pub export_packet_refs: Vec<String>,
    pub raw_path_export_allowed: bool,
    pub raw_glob_body_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_check_output_export_allowed: bool,
    pub redaction_class: String,
}

/// Pre-publication review invariants the parity-harness record must claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessReviewInvariants {
    pub review_pack_ref_pinned: bool,
    pub harness_lanes_pinned: bool,
    pub check_findings_pinned: bool,
    pub drift_downgrades_pinned: bool,
    pub overall_verdict_pinned: bool,
    pub no_hidden_writes: bool,
}

/// One alpha parity-harness record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackParityHarnessRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub harness_version: u32,
    pub parity_harness_id: String,
    pub review_pack_ref: String,
    pub repo_anchor_ref: String,
    pub pack_authority_class: String,
    pub display_label: String,
    pub summary: String,
    pub operator_caveat: String,
    pub harness_lane_observations: Vec<ReviewPackParityHarnessLaneObservation>,
    pub check_parity_findings: Vec<ReviewPackParityHarnessCheckFinding>,
    pub drift_downgrades: Vec<ReviewPackParityHarnessDriftDowngrade>,
    pub overall_verdict_class: String,
    pub row_downgrade_class: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export: ReviewPackParityHarnessSupportExport,
    pub review_invariants: ReviewPackParityHarnessReviewInvariants,
    pub minted_at: String,
}

/// Compact lane-observation projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackParityHarnessLaneProjection {
    pub lane_class: String,
    pub lane_status_class: String,
    pub summary: String,
}

/// Compact check-finding projection consumed by shell, CLI, and docs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackParityHarnessFindingProjection {
    pub check_ref: String,
    pub expected_parity_class: String,
    pub local_outcome_class: String,
    pub ci_outcome_class: String,
    pub observed_parity_class: String,
    pub parity_finding_class: String,
    pub summary: String,
}

/// Compact drift-downgrade projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackParityHarnessDowngradeProjection {
    pub check_ref: String,
    pub downgrade_class: String,
    pub summary: String,
}

/// Compact projection consumed by the parity-harness inspector surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackParityHarnessProjection {
    pub parity_harness_id: String,
    pub review_pack_ref: String,
    pub repo_anchor_ref: String,
    pub pack_authority_class: String,
    pub display_label: String,
    pub summary: String,
    pub operator_caveat: String,
    pub harness_version: u32,
    pub schema_version: u32,
    pub lane_observations: Vec<ReviewPackParityHarnessLaneProjection>,
    pub check_findings: Vec<ReviewPackParityHarnessFindingProjection>,
    pub drift_downgrades: Vec<ReviewPackParityHarnessDowngradeProjection>,
    pub overall_verdict_class: String,
    pub row_downgrade_class: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_path_export_allowed: bool,
    pub raw_glob_body_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_check_output_export_allowed: bool,
    pub finding_count: usize,
    pub full_parity_count: usize,
    pub local_only_match_count: usize,
    pub ci_only_match_count: usize,
    pub drift_detected_count: usize,
    pub downgrade_count: usize,
}

impl ReviewPackParityHarnessRecord {
    /// Validates the record against the alpha parity-harness contract.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewPackParityHarnessValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), ReviewPackParityHarnessValidationError> {
        validate_record(self)
    }

    /// Projects the record into the compact parity-harness inspector row.
    pub fn project(&self) -> ReviewPackParityHarnessProjection {
        let lane_observations: Vec<ReviewPackParityHarnessLaneProjection> = self
            .harness_lane_observations
            .iter()
            .map(|lane| ReviewPackParityHarnessLaneProjection {
                lane_class: lane.lane_class.clone(),
                lane_status_class: lane.lane_status_class.clone(),
                summary: lane.summary.clone(),
            })
            .collect();
        let check_findings: Vec<ReviewPackParityHarnessFindingProjection> = self
            .check_parity_findings
            .iter()
            .map(|finding| ReviewPackParityHarnessFindingProjection {
                check_ref: finding.check_ref.clone(),
                expected_parity_class: finding.expected_parity_class.clone(),
                local_outcome_class: finding.local_outcome_class.clone(),
                ci_outcome_class: finding.ci_outcome_class.clone(),
                observed_parity_class: finding.observed_parity_class.clone(),
                parity_finding_class: finding.parity_finding_class.clone(),
                summary: finding.summary.clone(),
            })
            .collect();
        let drift_downgrades: Vec<ReviewPackParityHarnessDowngradeProjection> = self
            .drift_downgrades
            .iter()
            .map(|down| ReviewPackParityHarnessDowngradeProjection {
                check_ref: down.check_ref.clone(),
                downgrade_class: down.downgrade_class.clone(),
                summary: down.summary.clone(),
            })
            .collect();
        let finding_count = check_findings.len();
        let full_parity_count = check_findings
            .iter()
            .filter(|f| f.parity_finding_class == "full_parity")
            .count();
        let local_only_match_count = check_findings
            .iter()
            .filter(|f| f.parity_finding_class == "local_only_documented_match")
            .count();
        let ci_only_match_count = check_findings
            .iter()
            .filter(|f| f.parity_finding_class == "ci_only_documented_match")
            .count();
        let drift_detected_count = check_findings
            .iter()
            .filter(|f| f.parity_finding_class == "drift_detected")
            .count();
        let downgrade_count = drift_downgrades.len();
        ReviewPackParityHarnessProjection {
            parity_harness_id: self.parity_harness_id.clone(),
            review_pack_ref: self.review_pack_ref.clone(),
            repo_anchor_ref: self.repo_anchor_ref.clone(),
            pack_authority_class: self.pack_authority_class.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            operator_caveat: self.operator_caveat.clone(),
            harness_version: self.harness_version,
            schema_version: self.schema_version,
            lane_observations,
            check_findings,
            drift_downgrades,
            overall_verdict_class: self.overall_verdict_class.clone(),
            row_downgrade_class: self.row_downgrade_class.clone(),
            consumer_surfaces: self.consumer_surfaces.clone(),
            support_export_refs: self.support_export.export_packet_refs.clone(),
            redaction_class: self.support_export.redaction_class.clone(),
            raw_path_export_allowed: self.support_export.raw_path_export_allowed,
            raw_glob_body_export_allowed: self.support_export.raw_glob_body_export_allowed,
            raw_command_export_allowed: self.support_export.raw_command_export_allowed,
            raw_check_output_export_allowed: self.support_export.raw_check_output_export_allowed,
            finding_count,
            full_parity_count,
            local_only_match_count,
            ci_only_match_count,
            drift_detected_count,
            downgrade_count,
        }
    }
}

/// Validation failure for a parity-harness record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackParityHarnessValidationError {
    message: String,
}

impl ReviewPackParityHarnessValidationError {
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

impl fmt::Display for ReviewPackParityHarnessValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "review-pack parity-harness validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for ReviewPackParityHarnessValidationError {}

/// Error returned when a parity-harness JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackParityHarnessError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha parity-harness contract.
    Validation(ReviewPackParityHarnessValidationError),
}

impl ReviewPackParityHarnessError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for ReviewPackParityHarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => {
                write!(
                    formatter,
                    "review-pack parity-harness JSON error: {message}"
                )
            }
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ReviewPackParityHarnessError {}

/// Parses and validates an alpha parity-harness JSON payload, returning
/// the compact projection on success.
///
/// # Errors
///
/// Returns [`ReviewPackParityHarnessError::Json`] when the payload is
/// not valid JSON for the record, and
/// [`ReviewPackParityHarnessError::Validation`] when any frozen alpha
/// guarantee is violated.
pub fn project_review_pack_parity_harness(
    payload: &str,
) -> Result<ReviewPackParityHarnessProjection, ReviewPackParityHarnessError> {
    let record: ReviewPackParityHarnessRecord = serde_json::from_str(payload)
        .map_err(|err| ReviewPackParityHarnessError::Json(err.to_string()))?;
    record
        .validate()
        .map_err(ReviewPackParityHarnessError::Validation)?;
    Ok(record.project())
}

fn validate_record(
    record: &ReviewPackParityHarnessRecord,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    require_equal(
        "record_kind",
        REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION {
        return Err(ReviewPackParityHarnessValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION
        )));
    }
    if record.harness_version != REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION {
        return Err(ReviewPackParityHarnessValidationError::new(format!(
            "harness_version is {}, expected {}",
            record.harness_version, REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION
        )));
    }
    require_non_empty("parity_harness_id", &record.parity_harness_id)?;
    require_non_empty("review_pack_ref", &record.review_pack_ref)?;
    require_non_empty("repo_anchor_ref", &record.repo_anchor_ref)?;
    require_one_of(
        "pack_authority_class",
        REVIEW_PACK_PARITY_HARNESS_AUTHORITY_CLASSES,
        &record.pack_authority_class,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("operator_caveat", &record.operator_caveat)?;
    require_non_empty("minted_at", &record.minted_at)?;
    require_one_of(
        "overall_verdict_class",
        REVIEW_PACK_PARITY_HARNESS_OVERALL_VERDICT_CLASSES,
        &record.overall_verdict_class,
    )?;
    require_one_of(
        "row_downgrade_class",
        REVIEW_PACK_PARITY_HARNESS_ROW_DOWNGRADE_CLASSES,
        &record.row_downgrade_class,
    )?;
    validate_lane_observations(&record.harness_lane_observations)?;
    validate_findings(&record.check_parity_findings)?;
    validate_drift_downgrades(&record.drift_downgrades)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_support_export(&record.support_export)?;
    validate_review_invariants(&record.review_invariants)?;
    cross_check_drift_downgrades(record)?;
    cross_check_overall_verdict(record)?;
    cross_check_row_downgrade(record)?;
    Ok(())
}

fn validate_lane_observations(
    observations: &[ReviewPackParityHarnessLaneObservation],
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if observations.len() < 2 {
        return Err(ReviewPackParityHarnessValidationError::new(
            "harness_lane_observations must declare both local and CI lanes",
        ));
    }
    let mut seen_lanes: BTreeSet<&str> = BTreeSet::new();
    for observation in observations {
        require_one_of(
            "harness_lane_observations[].lane_class",
            REVIEW_PACK_PARITY_HARNESS_LANE_CLASSES,
            &observation.lane_class,
        )?;
        require_one_of(
            "harness_lane_observations[].lane_status_class",
            REVIEW_PACK_PARITY_HARNESS_LANE_STATUS_CLASSES,
            &observation.lane_status_class,
        )?;
        require_non_empty("harness_lane_observations[].summary", &observation.summary)?;
        if !seen_lanes.insert(observation.lane_class.as_str()) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "harness_lane_observations contains a duplicate lane_class: {}",
                observation.lane_class
            )));
        }
    }
    for required in ["local_lane", "ci_lane"] {
        if !seen_lanes.contains(required) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "harness_lane_observations must include a {required} entry"
            )));
        }
    }
    Ok(())
}

fn validate_findings(
    findings: &[ReviewPackParityHarnessCheckFinding],
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if findings.is_empty() {
        return Err(ReviewPackParityHarnessValidationError::new(
            "check_parity_findings must list at least one finding",
        ));
    }
    let mut seen_refs: BTreeSet<&str> = BTreeSet::new();
    for finding in findings {
        require_non_empty("check_parity_findings[].check_ref", &finding.check_ref)?;
        if !seen_refs.insert(finding.check_ref.as_str()) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "check_parity_findings contains a duplicate check_ref: {}",
                finding.check_ref
            )));
        }
        require_one_of(
            "check_parity_findings[].expected_parity_class",
            REVIEW_PACK_PARITY_HARNESS_EXPECTED_PARITY_CLASSES,
            &finding.expected_parity_class,
        )?;
        require_one_of(
            "check_parity_findings[].local_outcome_class",
            REVIEW_PACK_PARITY_HARNESS_LANE_OUTCOME_CLASSES,
            &finding.local_outcome_class,
        )?;
        require_one_of(
            "check_parity_findings[].ci_outcome_class",
            REVIEW_PACK_PARITY_HARNESS_LANE_OUTCOME_CLASSES,
            &finding.ci_outcome_class,
        )?;
        require_one_of(
            "check_parity_findings[].observed_parity_class",
            REVIEW_PACK_PARITY_HARNESS_EXPECTED_PARITY_CLASSES,
            &finding.observed_parity_class,
        )?;
        require_one_of(
            "check_parity_findings[].parity_finding_class",
            REVIEW_PACK_PARITY_HARNESS_PARITY_FINDING_CLASSES,
            &finding.parity_finding_class,
        )?;
        require_non_empty("check_parity_findings[].summary", &finding.summary)?;
    }
    Ok(())
}

fn validate_drift_downgrades(
    downgrades: &[ReviewPackParityHarnessDriftDowngrade],
) -> Result<(), ReviewPackParityHarnessValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for downgrade in downgrades {
        require_non_empty("drift_downgrades[].check_ref", &downgrade.check_ref)?;
        if !seen.insert(downgrade.check_ref.as_str()) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "drift_downgrades contains a duplicate check_ref: {}",
                downgrade.check_ref
            )));
        }
        require_one_of(
            "drift_downgrades[].downgrade_class",
            REVIEW_PACK_PARITY_HARNESS_ROW_DOWNGRADE_CLASSES,
            &downgrade.downgrade_class,
        )?;
        if downgrade.downgrade_class == "no_downgrade" {
            return Err(ReviewPackParityHarnessValidationError::new(
                "drift_downgrades[].downgrade_class must not be no_downgrade; only real downgrades belong on the list",
            ));
        }
        require_non_empty("drift_downgrades[].summary", &downgrade.summary)?;
    }
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if surfaces.is_empty() {
        return Err(ReviewPackParityHarnessValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for surface in surfaces {
        require_one_of(
            "consumer_surfaces[]",
            REVIEW_PACK_PARITY_HARNESS_CONSUMER_SURFACES,
            surface,
        )?;
    }
    if !surfaces.iter().any(|s| s == "parity_harness_inspector") {
        return Err(ReviewPackParityHarnessValidationError::new(
            "consumer_surfaces must include parity_harness_inspector so the first product surface stays wired",
        ));
    }
    Ok(())
}

fn validate_support_export(
    export: &ReviewPackParityHarnessSupportExport,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if export.raw_path_export_allowed
        || export.raw_glob_body_export_allowed
        || export.raw_command_export_allowed
        || export.raw_check_output_export_allowed
    {
        return Err(ReviewPackParityHarnessValidationError::new(
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
    invariants: &ReviewPackParityHarnessReviewInvariants,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if !invariants.review_pack_ref_pinned
        || !invariants.harness_lanes_pinned
        || !invariants.check_findings_pinned
        || !invariants.drift_downgrades_pinned
        || !invariants.overall_verdict_pinned
        || !invariants.no_hidden_writes
    {
        return Err(ReviewPackParityHarnessValidationError::new(
            "review_invariants must all be true; the parity-harness record is a pre-publication review record for the parity claim itself",
        ));
    }
    Ok(())
}

fn cross_check_drift_downgrades(
    record: &ReviewPackParityHarnessRecord,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    let known_check_refs: BTreeSet<&str> = record
        .check_parity_findings
        .iter()
        .map(|f| f.check_ref.as_str())
        .collect();
    for downgrade in &record.drift_downgrades {
        if !known_check_refs.contains(downgrade.check_ref.as_str()) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "drift_downgrades entry {} references a check_ref not present in check_parity_findings",
                downgrade.check_ref
            )));
        }
    }
    let downgraded: BTreeSet<&str> = record
        .drift_downgrades
        .iter()
        .map(|d| d.check_ref.as_str())
        .collect();
    for finding in &record.check_parity_findings {
        if finding.parity_finding_class == "drift_detected"
            && !downgraded.contains(finding.check_ref.as_str())
        {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
                "check_parity_findings.check_ref {} reports drift_detected but no drift_downgrades entry was recorded; drift must never silently preserve a green claim",
                finding.check_ref
            )));
        }
    }
    Ok(())
}

fn cross_check_overall_verdict(
    record: &ReviewPackParityHarnessRecord,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    let any_drift = record
        .check_parity_findings
        .iter()
        .any(|f| f.parity_finding_class == "drift_detected");
    let any_unknown = record
        .check_parity_findings
        .iter()
        .any(|f| f.parity_finding_class == "parity_unknown_requires_review");
    match record.overall_verdict_class.as_str() {
        "full_parity" => {
            if any_drift || any_unknown {
                return Err(ReviewPackParityHarnessValidationError::new(
                    "overall_verdict_class=full_parity but a finding reports drift_detected or parity_unknown_requires_review",
                ));
            }
        }
        "drift_downgraded" => {
            if !any_drift {
                return Err(ReviewPackParityHarnessValidationError::new(
                    "overall_verdict_class=drift_downgraded requires at least one drift_detected finding",
                ));
            }
        }
        "parity_unknown_requires_review" => {
            if !any_unknown {
                return Err(ReviewPackParityHarnessValidationError::new(
                    "overall_verdict_class=parity_unknown_requires_review requires at least one parity_unknown_requires_review finding",
                ));
            }
        }
        "lane_declined_documented" => {
            let any_declined = record
                .harness_lane_observations
                .iter()
                .any(|l| l.lane_status_class != "lane_engaged");
            if !any_declined {
                return Err(ReviewPackParityHarnessValidationError::new(
                    "overall_verdict_class=lane_declined_documented requires at least one lane with a non-engaged status",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn cross_check_row_downgrade(
    record: &ReviewPackParityHarnessRecord,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    let any_drift = record
        .check_parity_findings
        .iter()
        .any(|f| f.parity_finding_class == "drift_detected");
    if any_drift && record.row_downgrade_class == "no_downgrade" {
        return Err(ReviewPackParityHarnessValidationError::new(
            "row_downgrade_class must not be no_downgrade when a finding reports drift_detected; drift must always downgrade the row",
        ));
    }
    if record.overall_verdict_class == "drift_downgraded"
        && record.row_downgrade_class == "no_downgrade"
    {
        return Err(ReviewPackParityHarnessValidationError::new(
            "row_downgrade_class must not be no_downgrade when overall_verdict_class=drift_downgraded",
        ));
    }
    if record.row_downgrade_class != "no_downgrade" && record.drift_downgrades.is_empty() {
        return Err(ReviewPackParityHarnessValidationError::new(
            "row_downgrade_class names a downgrade but drift_downgrades is empty; record the per-check downgrade reason",
        ));
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ReviewPackParityHarnessValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(
    label: &str,
    value: &str,
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if value.trim().is_empty() {
        Err(ReviewPackParityHarnessValidationError::new(format!(
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
) -> Result<(), ReviewPackParityHarnessValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(ReviewPackParityHarnessValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), ReviewPackParityHarnessValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(ReviewPackParityHarnessValidationError::new(format!(
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
        "/../../fixtures/review/m3/review_pack_harness/first_party_full_parity_run.json"
    ));
    const FIXTURE_TEAM_SHARED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_harness/team_shared_mixed_parity_documented.json"
    ));
    const FIXTURE_PARTNER: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_harness/partner_signed_ci_only_documented.json"
    ));
    const FIXTURE_COMMUNITY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m3/review_pack_harness/uncertified_community_drift_downgrade.json"
    ));

    #[test]
    fn first_party_full_parity_projects() {
        let projection = project_review_pack_parity_harness(FIXTURE_FIRST_PARTY)
            .expect("first-party full-parity fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_first_party");
        assert_eq!(projection.overall_verdict_class, "full_parity");
        assert_eq!(projection.row_downgrade_class, "no_downgrade");
        assert_eq!(projection.drift_detected_count, 0);
        assert!(projection.full_parity_count >= 1);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|s| s == "parity_harness_inspector"));
    }

    #[test]
    fn team_shared_mixed_parity_projects() {
        let projection = project_review_pack_parity_harness(FIXTURE_TEAM_SHARED)
            .expect("team-shared mixed-parity fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_team_shared");
        assert_eq!(projection.overall_verdict_class, "full_parity");
        assert!(projection.local_only_match_count >= 1);
        assert!(projection.ci_only_match_count >= 1);
    }

    #[test]
    fn partner_signed_ci_only_projects() {
        let projection = project_review_pack_parity_harness(FIXTURE_PARTNER)
            .expect("partner-signed CI-only fixture must project");
        assert_eq!(projection.pack_authority_class, "repo_partner_signed");
        assert!(projection.ci_only_match_count >= 1);
    }

    #[test]
    fn community_drift_downgrades_the_row() {
        let projection = project_review_pack_parity_harness(FIXTURE_COMMUNITY)
            .expect("community drift-downgrade fixture must project");
        assert_eq!(
            projection.pack_authority_class,
            "repo_uncertified_community"
        );
        assert_eq!(projection.overall_verdict_class, "drift_downgraded");
        assert_eq!(
            projection.row_downgrade_class,
            "downgraded_to_review_required"
        );
        assert!(projection.drift_detected_count >= 1);
        assert!(projection.downgrade_count >= 1);
    }

    #[test]
    fn rejects_drift_without_downgrade() {
        let mut record: ReviewPackParityHarnessRecord =
            serde_json::from_str(FIXTURE_COMMUNITY).expect("fixture must parse");
        record.drift_downgrades.clear();
        let err = record
            .validate()
            .expect_err("drift without downgrade must fail");
        assert!(err.message().contains("drift"));
    }

    #[test]
    fn rejects_full_parity_verdict_with_drift_finding() {
        let mut record: ReviewPackParityHarnessRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.check_parity_findings[0].parity_finding_class = "drift_detected".to_string();
        record.check_parity_findings[0].observed_parity_class =
            "parity_unknown_requires_review".to_string();
        record
            .drift_downgrades
            .push(ReviewPackParityHarnessDriftDowngrade {
                check_ref: record.check_parity_findings[0].check_ref.clone(),
                downgrade_class: "downgraded_to_review_required".to_string(),
                summary: "Drift downgrades the row.".to_string(),
            });
        record.row_downgrade_class = "downgraded_to_review_required".to_string();
        let err = record
            .validate()
            .expect_err("full_parity verdict with drift must fail");
        assert!(err.message().contains("full_parity"));
    }

    #[test]
    fn rejects_missing_local_lane() {
        let mut record: ReviewPackParityHarnessRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record
            .harness_lane_observations
            .retain(|lane| lane.lane_class != "local_lane");
        record
            .harness_lane_observations
            .push(ReviewPackParityHarnessLaneObservation {
                lane_class: "ci_lane".to_string(),
                lane_status_class: "lane_engaged".to_string(),
                summary: "Duplicate placeholder.".to_string(),
            });
        let err = record.validate().expect_err("missing local_lane must fail");
        assert!(err.message().contains("local_lane") || err.message().contains("duplicate"));
    }

    #[test]
    fn rejects_raw_path_export() {
        let mut record: ReviewPackParityHarnessRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.support_export.raw_path_export_allowed = true;
        let err = record.validate().expect_err("raw path export must fail");
        assert!(err.message().contains("raw_"));
    }

    #[test]
    fn rejects_missing_parity_harness_inspector_consumer() {
        let mut record: ReviewPackParityHarnessRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record
            .consumer_surfaces
            .retain(|surface| surface != "parity_harness_inspector");
        let err = record
            .validate()
            .expect_err("missing parity_harness_inspector must fail");
        assert!(err.message().contains("parity_harness_inspector"));
    }

    #[test]
    fn rejects_wrong_record_kind_via_project() {
        let tampered = FIXTURE_FIRST_PARTY.replace(
            "review_pack_parity_harness_alpha_record",
            "other_record_kind",
        );
        match project_review_pack_parity_harness(&tampered) {
            Err(ReviewPackParityHarnessError::Validation(err)) => {
                assert!(err.message().contains("record_kind"));
            }
            other => panic!("expected validation failure, got {other:?}"),
        }
    }
}
