//! Cross-surface M5 mutation-history inspector.
//!
//! This module projects the canonical M5 mutation-lineage packet into the
//! deterministic inspector rows the shell, CLI, and support review surfaces
//! can render when users need to understand what changed, who or what changed
//! it, which checkpoint lineage exists, and whether the visible recovery path
//! is exact, grouped exact, compensate, regenerate, manual, or audit-only.

use std::fmt;

use aureline_reactive_state::{
    seeded_m5_mutation_lineage_packet, validate_m5_mutation_lineage_packet,
    M5MutationArtifactClass, M5MutationAutomationInfluence, M5MutationHistoryInspectorRow as PacketHistoryInspectorRow,
    M5MutationLineagePacket, M5MutationLineageValidationReport, M5MutationPolicyInfluence,
    M5MutationReversalClass, M5MutationSurfaceClass,
};

/// Presentation label rendered for the cross-surface history inspector.
pub const M5_MUTATION_HISTORY_INSPECTOR_PRESENTATION_LABEL: &str =
    "M5 mutation-history inspector";

/// Presentation subtitle rendered for the cross-surface history inspector.
pub const M5_MUTATION_HISTORY_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Explain lineage root, reversal class, checkpoint chain, and automation or policy influence across M5 mutation surfaces.";

/// One row rendered in the shell history inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MutationHistoryInspectorRow {
    pub row_id: String,
    pub lineage_root_id: String,
    pub title: String,
    pub primary_surface_class: M5MutationSurfaceClass,
    pub group_count: usize,
    pub mutation_count: usize,
    pub highest_risk_reversal_class: M5MutationReversalClass,
    pub reversal_classes: Vec<M5MutationReversalClass>,
    pub total_file_count: u32,
    pub artifact_classes: Vec<M5MutationArtifactClass>,
    pub automation_influences: Vec<M5MutationAutomationInfluence>,
    pub policy_influences: Vec<M5MutationPolicyInfluence>,
    pub checkpoint_count: usize,
    pub reopen_action_label: String,
    pub notes: String,
}

impl M5MutationHistoryInspectorRow {
    fn from_packet_row(row: &PacketHistoryInspectorRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            lineage_root_id: row.lineage_root_id.clone(),
            title: row.title.clone(),
            primary_surface_class: row.primary_surface_class,
            group_count: row.group_ids.len(),
            mutation_count: row.mutation_ids.len(),
            highest_risk_reversal_class: row.highest_risk_reversal_class,
            reversal_classes: row.reversal_classes.clone(),
            total_file_count: row.total_file_count,
            artifact_classes: row.artifact_classes.clone(),
            automation_influences: row.automation_influences.clone(),
            policy_influences: row.policy_influences.clone(),
            checkpoint_count: row.checkpoint_ids.len(),
            reopen_action_label: row.reopen_action_label.clone(),
            notes: row.notes.clone(),
        }
    }
}

/// Error returned when the inspector cannot project rows from the canonical
/// M5 packet.
#[derive(Debug)]
pub enum M5MutationHistoryInspectorError {
    /// The canonical packet failed validation.
    PacketValidation(M5MutationLineageValidationReport),
}

impl fmt::Display for M5MutationHistoryInspectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => write!(f, "m5 mutation lineage invalid: {report}"),
        }
    }
}

impl std::error::Error for M5MutationHistoryInspectorError {}

impl From<M5MutationLineageValidationReport> for M5MutationHistoryInspectorError {
    fn from(report: M5MutationLineageValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Builds cross-surface history inspector rows from the canonical M5 packet.
///
/// # Errors
///
/// Returns [`M5MutationHistoryInspectorError`] when the packet fails
/// validation.
pub fn build_m5_mutation_history_rows(
) -> Result<Vec<M5MutationHistoryInspectorRow>, M5MutationHistoryInspectorError> {
    let packet = seeded_m5_mutation_lineage_packet();
    validate_m5_mutation_lineage_packet(&packet)?;
    Ok(rows_from_packet(&packet))
}

/// Renders the history inspector projection as deterministic plaintext for
/// CLI, support review, and docs consumers.
///
/// # Errors
///
/// Returns [`M5MutationHistoryInspectorError`] when the packet fails
/// validation.
pub fn render_m5_mutation_history_plaintext(
) -> Result<String, M5MutationHistoryInspectorError> {
    let rows = build_m5_mutation_history_rows()?;
    let mut lines = vec![
        "M5 mutation-history inspector".to_string(),
        "lineage_root | primary_surface | groups/mutations | highest_risk | file_count | artifacts | automation | policy"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {}/{} | {} | {} | {} | {} | {}",
            row.lineage_root_id,
            row.primary_surface_class.as_str(),
            row.group_count,
            row.mutation_count,
            row.highest_risk_reversal_class.as_str(),
            row.total_file_count,
            join_artifacts(&row.artifact_classes),
            join_automation(&row.automation_influences),
            join_policy(&row.policy_influences),
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn rows_from_packet(packet: &M5MutationLineagePacket) -> Vec<M5MutationHistoryInspectorRow> {
    let mut rows: Vec<_> = packet
        .history_inspector_rows
        .iter()
        .map(M5MutationHistoryInspectorRow::from_packet_row)
        .collect();
    rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));
    rows
}

fn join_artifacts(classes: &[M5MutationArtifactClass]) -> String {
    classes
        .iter()
        .map(|class| class.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn join_automation(classes: &[M5MutationAutomationInfluence]) -> String {
    classes
        .iter()
        .map(|class| class.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn join_policy(classes: &[M5MutationPolicyInfluence]) -> String {
    classes
        .iter()
        .map(|class| class.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_build_from_canonical_packet() {
        let rows = build_m5_mutation_history_rows().expect("rows build");
        assert_eq!(rows.len(), 5);
        assert!(rows.iter().any(|row| {
            row.lineage_root_id == "lineage:m5:provider_sync:0001"
                && row.highest_risk_reversal_class == M5MutationReversalClass::Manual
                && row
                    .reversal_classes
                    .contains(&M5MutationReversalClass::Compensate)
        }));
        assert!(rows.iter().any(|row| {
            row.lineage_root_id == "lineage:m5:notebook_execution:0001"
                && row
                    .reversal_classes
                    .contains(&M5MutationReversalClass::AuditOnly)
                && row
                    .reversal_classes
                    .contains(&M5MutationReversalClass::Regenerate)
                && row
                    .reversal_classes
                    .contains(&M5MutationReversalClass::Exact)
        }));
    }

    #[test]
    fn plaintext_is_deterministic_and_mentions_explicit_reversal_classes() {
        let first = render_m5_mutation_history_plaintext().expect("plaintext renders");
        let second = render_m5_mutation_history_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("M5 mutation-history inspector"));
        assert!(first.contains("lineage:m5:provider_sync:0001"));
        assert!(first.contains("manual"));
        assert!(first.contains("audit_only"));
        assert!(first.contains("grouped_exact"));
    }
}
