//! Start Center prebuild fingerprint alpha projection.
//!
//! This module turns the checked-in alpha prebuild fingerprint, reuse
//! decision, and disclosure records into the compact Start Center rows
//! that disclose fingerprint, freshness, origin, and invalidation reason
//! **before** any reuse path is committed. The same projection backs the
//! deterministic CLI / headless plaintext export so docs, support
//! packets, and scripted entry surfaces read one prebuild truth.

use std::fmt;

use aureline_workspace::{
    project_prebuild_fingerprint_alpha, PrebuildFingerprintError, PrebuildFingerprintProjection,
};

const ALPHA_PREBUILD_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/workspace/m3/prebuild_fingerprint/valid_cached_prebuild_fingerprint.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/prebuild_fingerprint/valid_cached_prebuild_fingerprint.json"
        )),
    ),
    (
        "fixtures/workspace/m3/prebuild_fingerprint/reuse_allowed_decision.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/prebuild_fingerprint/reuse_allowed_decision.json"
        )),
    ),
    (
        "fixtures/workspace/m3/prebuild_fingerprint/stale_snapshot_resume_denied_decision.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/prebuild_fingerprint/stale_snapshot_resume_denied_decision.json"
        )),
    ),
    (
        "fixtures/workspace/m3/prebuild_fingerprint/local_override_rebuild_disclosure.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/prebuild_fingerprint/local_override_rebuild_disclosure.json"
        )),
    ),
    (
        "fixtures/workspace/m3/prebuild_fingerprint/fresh_clone_disclosure.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/prebuild_fingerprint/fresh_clone_disclosure.json"
        )),
    ),
];

/// Start Center row for one alpha prebuild fingerprint, decision, or
/// disclosure record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterPrebuildFingerprintRow {
    /// Stable record id (fingerprint, decision, or disclosure).
    pub record_id: String,
    /// `prebuild_fingerprint_record`, `prebuild_reuse_decision_record`, or
    /// `prebuild_disclosure_record`.
    pub record_kind: String,
    /// Display label rendered on the card.
    pub display_label: String,
    /// Short reviewable summary.
    pub summary: String,
    /// Requested entry path: `resume_live_workspace`, `start_from_snapshot`,
    /// `clone_fresh`, or `reuse_cached_prebuild`.
    pub requested_path: String,
    /// Source materialization class.
    pub source_materialization_class: String,
    /// Reuse outcome (empty for fingerprint / disclosure rows).
    pub reuse_outcome: String,
    /// Disclosure state (empty for fingerprint / decision rows).
    pub disclosure_state: String,
    /// Freshness age class shown on the card.
    pub freshness_age_class: String,
    /// Producer class for the fingerprint.
    pub producer_class: String,
    /// Signer posture for the fingerprint.
    pub signer_posture: String,
    /// Host class shown next to the freshness age class.
    pub host_class: String,
    /// Platform / arch shown next to the host class.
    pub platform_arch: String,
    /// Required revalidation classes the user must clear before reuse.
    pub required_revalidations: Vec<String>,
    /// Cache class labels carried on the fingerprint or decision.
    pub cache_class_labels: Vec<String>,
    /// Whether rebuild is required.
    pub rebuild_required: bool,
    /// Whether fresh clone is required.
    pub fresh_clone_required: bool,
    /// Whether a local override changed the fingerprint.
    pub local_override_disclosed: bool,
    /// Local override ref when present.
    pub local_override_ref: Option<String>,
    /// Alternative entry-path lanes the user may still take.
    pub alternative_lane_refs: Vec<String>,
    /// Stale snapshots are never labelled live resume.
    pub stale_snapshot_must_not_be_labeled_live_resume: bool,
}

impl From<PrebuildFingerprintProjection> for StartCenterPrebuildFingerprintRow {
    fn from(projection: PrebuildFingerprintProjection) -> Self {
        Self {
            record_id: projection.record_id,
            record_kind: projection.record_kind,
            display_label: projection.display_label,
            summary: projection.summary,
            requested_path: projection.requested_path,
            source_materialization_class: projection.source_materialization_class,
            reuse_outcome: projection.reuse_outcome,
            disclosure_state: projection.disclosure_state,
            freshness_age_class: projection.freshness_age_class,
            producer_class: projection.producer_class,
            signer_posture: projection.signer_posture,
            host_class: projection.host_class,
            platform_arch: projection.platform_arch,
            required_revalidations: projection.required_revalidations,
            cache_class_labels: projection.cache_class_labels,
            rebuild_required: projection.rebuild_required,
            fresh_clone_required: projection.fresh_clone_required,
            local_override_disclosed: projection.local_override_disclosed,
            local_override_ref: projection.local_override_ref,
            alternative_lane_refs: projection.alternative_lane_refs,
            stale_snapshot_must_not_be_labeled_live_resume: projection
                .stale_snapshot_must_not_be_labeled_live_resume,
        }
    }
}

/// Error returned when the Start Center cannot project an alpha prebuild
/// record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterPrebuildFingerprintError {
    source_ref: &'static str,
    message: String,
}

impl StartCenterPrebuildFingerprintError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StartCenterPrebuildFingerprintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for StartCenterPrebuildFingerprintError {}

/// Builds Start Center prebuild fingerprint rows from the checked-in alpha
/// fixtures.
///
/// # Errors
///
/// Returns [`StartCenterPrebuildFingerprintError`] when any fixture fails to
/// parse or validate against the alpha contract.
pub fn build_alpha_prebuild_fingerprint_rows(
) -> Result<Vec<StartCenterPrebuildFingerprintRow>, StartCenterPrebuildFingerprintError> {
    let mut rows = Vec::with_capacity(ALPHA_PREBUILD_FIXTURES.len());
    for (source_ref, payload) in ALPHA_PREBUILD_FIXTURES {
        let projection = project_prebuild_fingerprint_alpha(payload)
            .map_err(|err| projection_error(source_ref, err))?;
        rows.push(StartCenterPrebuildFingerprintRow::from(projection));
    }
    Ok(rows)
}

/// Renders the alpha projection as deterministic plaintext for CLI /
/// headless / docs consumers.
///
/// # Errors
///
/// Returns [`StartCenterPrebuildFingerprintError`] when a fixture cannot be
/// projected.
pub fn render_alpha_prebuild_fingerprint_plaintext(
) -> Result<String, StartCenterPrebuildFingerprintError> {
    let rows = build_alpha_prebuild_fingerprint_rows()?;
    let mut lines = vec![
        "Prebuild fingerprint alpha".to_string(),
        "record_id | record_kind | requested_path | outcome/state | freshness | host/platform | revalidations"
            .to_string(),
    ];
    for row in rows {
        let outcome_or_state = if !row.reuse_outcome.is_empty() {
            row.reuse_outcome.clone()
        } else if !row.disclosure_state.is_empty() {
            row.disclosure_state.clone()
        } else {
            "fingerprint".to_string()
        };
        let revalidations = if row.required_revalidations.is_empty() {
            "none".to_string()
        } else {
            row.required_revalidations.join(",")
        };
        lines.push(format!(
            "{} | {} | {} | {} | {} | {}/{} | {}",
            row.record_id,
            row.record_kind,
            row.requested_path,
            outcome_or_state,
            row.freshness_age_class,
            row.host_class,
            row.platform_arch,
            revalidations,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(
    source_ref: &'static str,
    err: PrebuildFingerprintError,
) -> StartCenterPrebuildFingerprintError {
    StartCenterPrebuildFingerprintError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_prebuild_rows_project() {
        let rows = build_alpha_prebuild_fingerprint_rows().expect("alpha fixtures must project");
        assert_eq!(rows.len(), ALPHA_PREBUILD_FIXTURES.len());
        let record_kinds: Vec<&str> = rows.iter().map(|row| row.record_kind.as_str()).collect();
        assert!(record_kinds.contains(&"prebuild_fingerprint_record"));
        assert!(record_kinds.contains(&"prebuild_reuse_decision_record"));
        assert!(record_kinds.contains(&"prebuild_disclosure_record"));
        for row in &rows {
            assert!(
                row.stale_snapshot_must_not_be_labeled_live_resume,
                "every row must keep the resume-live invariant true",
            );
        }
    }

    #[test]
    fn resume_live_denied_row_is_visible() {
        let rows = build_alpha_prebuild_fingerprint_rows().expect("alpha fixtures must project");
        let denied = rows
            .iter()
            .find(|row| row.reuse_outcome == "resume_live_denied")
            .expect("a resume_live_denied row must be wired into Start Center");
        assert_eq!(denied.requested_path, "resume_live_workspace");
        assert_eq!(
            denied.source_materialization_class,
            "stale_prebuild_snapshot"
        );
    }

    #[test]
    fn fresh_clone_row_keeps_path_distinct() {
        let rows = build_alpha_prebuild_fingerprint_rows().expect("alpha fixtures must project");
        let clone_row = rows
            .iter()
            .find(|row| row.disclosure_state == "fresh_clone")
            .expect("a fresh_clone row must be wired into Start Center");
        assert!(clone_row.fresh_clone_required);
        assert!(!clone_row.rebuild_required);
    }

    #[test]
    fn alpha_prebuild_plaintext_is_deterministic() {
        let first = render_alpha_prebuild_fingerprint_plaintext().expect("plaintext renders");
        let second = render_alpha_prebuild_fingerprint_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Prebuild fingerprint alpha"));
        assert!(first.contains("resume_live_denied"));
        assert!(first.contains("fresh_clone"));
    }
}
