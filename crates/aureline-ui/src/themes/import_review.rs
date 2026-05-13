//! Imported-theme mapping report projection.
//!
//! The projection keeps imported themes honest by exposing translated,
//! unsupported, unresolved, and fallback slots before an appearance session can
//! claim parity.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

const IMPORT_REPORT_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/theme_support_cases/imported_translated_theme_mapping_report_with_warnings.yaml"
));

static IMPORT_REPORT: OnceLock<Result<ThemeImportMappingReport, ThemeImportMappingError>> =
    OnceLock::new();

/// Error emitted while loading an imported-theme mapping report.
#[derive(Debug, Clone)]
pub enum ThemeImportMappingError {
    /// The embedded mapping report YAML did not parse.
    ParseFailed(String),
    /// The mapping report is structurally invalid.
    InvalidReport(String),
}

impl fmt::Display for ThemeImportMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed(detail) => {
                write!(f, "failed to parse theme import mapping report: {detail}")
            }
            Self::InvalidReport(detail) => {
                write!(f, "invalid theme import mapping report: {detail}")
            }
        }
    }
}

impl std::error::Error for ThemeImportMappingError {}

/// Review state for an imported-theme parity claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportedThemeParityReadiness {
    /// Full parity is claimable because no unresolved, unsupported, or blocked rows remain.
    FullParityClaimable,
    /// The report is usable but must stay partial because visible gaps remain.
    PartialWithVisibleGaps,
    /// The report blocks commit because required mapping or rollback evidence is missing.
    Blocked,
}

/// Aggregate counts surfaced by an imported-theme mapping report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeImportMappingSummary {
    /// Number of translated source slots.
    pub translated_slot_count: u32,
    /// Number of slots substituted through fallback tokens.
    pub substituted_with_fallback_count: u32,
    /// Number of unsupported source slots.
    pub unsupported_slot_count: u32,
    /// Number of unresolved source slots.
    pub unresolved_mapping_count: u32,
    /// Number of blocked honesty rows.
    pub blocked_honesty_count: u32,
    /// Number of deprecated replacement rows.
    pub deprecated_replacement_count: u32,
}

/// Loaded imported-theme mapping report used by the appearance review lane.
#[derive(Debug, Clone)]
pub struct ThemeImportMappingReport {
    doc: ThemeImportMappingReportDoc,
}

impl ThemeImportMappingReport {
    /// Loads and validates the protected imported-theme mapping report fixture.
    pub fn load_warnings_fixture() -> Result<Self, ThemeImportMappingError> {
        let doc: ThemeImportMappingReportDoc = serde_yaml::from_str(IMPORT_REPORT_YAML)
            .map_err(|err| ThemeImportMappingError::ParseFailed(err.to_string()))?;
        let report = Self { doc };
        report.validate()?;
        Ok(report)
    }

    /// Returns the stable report id.
    pub fn report_id(&self) -> &str {
        &self.doc.report_id
    }

    /// Returns the appearance checkpoint ref minted for this import.
    pub fn appearance_checkpoint_ref(&self) -> &str {
        &self.doc.appearance_checkpoint_ref
    }

    /// Returns the parity claim state from the report.
    pub fn parity_claim_state(&self) -> &str {
        &self.doc.parity_claim_state
    }

    /// Returns the import outcome from the report.
    pub fn import_outcome(&self) -> &str {
        &self.doc.import_outcome
    }

    /// Returns the aggregate mapping summary.
    pub fn summary(&self) -> ThemeImportMappingSummary {
        ThemeImportMappingSummary {
            translated_slot_count: self.doc.mapping_summary.translated_slot_count,
            substituted_with_fallback_count: self
                .doc
                .mapping_summary
                .substituted_with_fallback_count,
            unsupported_slot_count: self.doc.mapping_summary.unsupported_slot_count,
            unresolved_mapping_count: self.doc.mapping_summary.unresolved_mapping_count,
            blocked_honesty_count: self.doc.mapping_summary.blocked_honesty_count,
            deprecated_replacement_count: self.doc.mapping_summary.deprecated_replacement_count,
        }
    }

    /// Returns true when every per-slot row is visible in the mapping report.
    pub fn all_mapping_rows_visible(&self) -> bool {
        self.all_slots().all(|slot| slot.visible_in_report)
    }

    /// Returns the review readiness for claiming imported-theme parity.
    pub fn parity_readiness(&self) -> ImportedThemeParityReadiness {
        let summary = self.summary();
        if !self.all_mapping_rows_visible()
            || self.doc.rollback_path.checkpoint_ref.as_deref().is_none()
            || self.doc.rollback_path.rollback_ref.as_deref().is_none()
        {
            return ImportedThemeParityReadiness::Blocked;
        }
        if summary.unsupported_slot_count == 0
            && summary.unresolved_mapping_count == 0
            && summary.blocked_honesty_count == 0
            && self.doc.parity_claim_state == "claimed_with_report"
        {
            ImportedThemeParityReadiness::FullParityClaimable
        } else {
            ImportedThemeParityReadiness::PartialWithVisibleGaps
        }
    }

    fn all_slots(&self) -> impl Iterator<Item = &MappingSlotRowDoc> {
        self.doc
            .translated_slots
            .iter()
            .chain(self.doc.unsupported_slots.iter())
            .chain(self.doc.unresolved_slots.iter())
            .chain(self.doc.blocked_slots.iter())
    }

    fn validate(&self) -> Result<(), ThemeImportMappingError> {
        if self.doc.record_kind != "theme_import_mapping_report_record" {
            return Err(ThemeImportMappingError::InvalidReport(format!(
                "unexpected record_kind {}",
                self.doc.record_kind
            )));
        }
        if self.doc.theme_import_mapping_schema_version != 1 {
            return Err(ThemeImportMappingError::InvalidReport(format!(
                "unsupported schema version {}",
                self.doc.theme_import_mapping_schema_version
            )));
        }
        if self.doc.appearance_checkpoint_ref.trim().is_empty() {
            return Err(ThemeImportMappingError::InvalidReport(
                "missing appearance checkpoint ref".to_string(),
            ));
        }
        if self.doc.rollback_path.checkpoint_ref.as_deref().is_none()
            || self.doc.rollback_path.rollback_ref.as_deref().is_none()
        {
            return Err(ThemeImportMappingError::InvalidReport(
                "rollback path must cite checkpoint and rollback refs".to_string(),
            ));
        }
        if !self.all_mapping_rows_visible() {
            return Err(ThemeImportMappingError::InvalidReport(
                "all mapping rows must be visible".to_string(),
            ));
        }

        let counts = count_slots_by_state(self.all_slots());
        let summary = &self.doc.mapping_summary;
        let expected_total = summary.translated_slot_count
            + summary.unsupported_slot_count
            + summary.unresolved_mapping_count
            + summary.substituted_with_fallback_count
            + summary.blocked_honesty_count
            + summary.deprecated_replacement_count;
        if summary.total_source_slot_count != expected_total {
            return Err(ThemeImportMappingError::InvalidReport(
                "mapping summary counts do not sum to total_source_slot_count".to_string(),
            ));
        }
        for (state, expected) in [
            ("translated", summary.translated_slot_count),
            (
                "substituted_fallback",
                summary.substituted_with_fallback_count,
            ),
            ("unsupported", summary.unsupported_slot_count),
            ("unresolved", summary.unresolved_mapping_count),
            ("blocked_honesty", summary.blocked_honesty_count),
            (
                "deprecated_replacement",
                summary.deprecated_replacement_count,
            ),
        ] {
            let actual = counts.get(state).copied().unwrap_or_default();
            if actual != expected {
                return Err(ThemeImportMappingError::InvalidReport(format!(
                    "mapping state {state} count mismatch: expected {expected}, actual {actual}"
                )));
            }
        }

        let protected_cues: HashSet<&str> = self
            .doc
            .protected_cue_honesty_checks
            .iter()
            .map(|row| row.protected_cue_class.as_str())
            .collect();
        for cue in ["trust", "policy_lock", "severity", "source_integrity"] {
            if !protected_cues.contains(cue) {
                return Err(ThemeImportMappingError::InvalidReport(format!(
                    "missing protected cue honesty check {cue}"
                )));
            }
        }
        Ok(())
    }
}

/// Returns the protected imported-theme mapping report fixture.
pub fn imported_theme_mapping_report_with_warnings(
) -> Result<&'static ThemeImportMappingReport, ThemeImportMappingError> {
    let report = IMPORT_REPORT.get_or_init(ThemeImportMappingReport::load_warnings_fixture);
    match report {
        Ok(report) => Ok(report),
        Err(err) => Err(err.clone()),
    }
}

fn count_slots_by_state<'a>(
    slots: impl Iterator<Item = &'a MappingSlotRowDoc>,
) -> std::collections::HashMap<&'a str, u32> {
    let mut counts = std::collections::HashMap::new();
    for slot in slots {
        *counts.entry(slot.mapping_state.as_str()).or_insert(0) += 1;
    }
    counts
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ThemeImportMappingReportDoc {
    record_kind: String,
    theme_import_mapping_schema_version: u32,
    report_id: String,
    translated_slots: Vec<MappingSlotRowDoc>,
    unsupported_slots: Vec<MappingSlotRowDoc>,
    unresolved_slots: Vec<MappingSlotRowDoc>,
    #[serde(default)]
    blocked_slots: Vec<MappingSlotRowDoc>,
    mapping_summary: MappingSummaryDoc,
    protected_cue_honesty_checks: Vec<ProtectedCueHonestyCheckDoc>,
    appearance_checkpoint_ref: String,
    rollback_path: RollbackPathDoc,
    parity_claim_state: String,
    import_outcome: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct MappingSlotRowDoc {
    source_slot_ref: String,
    target_token_ref: Option<String>,
    mapping_state: String,
    fallback_token_class: String,
    visible_in_report: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct MappingSummaryDoc {
    total_source_slot_count: u32,
    translated_slot_count: u32,
    unsupported_slot_count: u32,
    unresolved_mapping_count: u32,
    substituted_with_fallback_count: u32,
    blocked_honesty_count: u32,
    deprecated_replacement_count: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ProtectedCueHonestyCheckDoc {
    protected_cue_class: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct RollbackPathDoc {
    rollback_path_class: String,
    checkpoint_ref: Option<String>,
    rollback_ref: Option<String>,
    user_visible_action_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_theme_report_keeps_gaps_visible_before_commit() {
        let report = imported_theme_mapping_report_with_warnings().expect("import report");
        let summary = report.summary();

        assert_eq!(summary.translated_slot_count, 2);
        assert_eq!(summary.substituted_with_fallback_count, 1);
        assert_eq!(summary.unsupported_slot_count, 1);
        assert_eq!(summary.unresolved_mapping_count, 2);
        assert!(report.all_mapping_rows_visible());
        assert_eq!(
            report.parity_readiness(),
            ImportedThemeParityReadiness::PartialWithVisibleGaps
        );
        assert_eq!(
            report.appearance_checkpoint_ref(),
            "appearance_checkpoint:user.alex:vscode_import:01"
        );
    }
}
