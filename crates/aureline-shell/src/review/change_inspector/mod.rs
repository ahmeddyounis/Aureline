//! Shell landing-state inspector — alpha consumer of the change-lineage
//! family.
//!
//! This module reads the checked-in alpha change-lineage fixtures and
//! projects them into the deterministic landing-state rows the inspector
//! renders ahead of any publish, merge, or apply action. Every row exposes
//! the four review questions the inspector exists to answer:
//!
//! 1. Which scope am I operating on (main worktree, side worktree, or
//!    stacked patch set)?
//! 2. Where will the change land (landing-state class, landing-action
//!    class, target ref, mutation authority, remote visibility, egress)?
//! 3. What is the ancestry (base ref, divergence class, commits ahead /
//!    behind, ancestor chain length)?
//! 4. Is the change ready, or what is in the way (conflict-state class,
//!    publish-readiness class, blockers)?
//!
//! The shared data types are defined in
//! [`aureline_review::change_inspector`] so review and support packets
//! re-use the same lineage fields without forking vocabulary.

use std::fmt;

use aureline_review::{project_change_lineage, ChangeLineageError, ChangeLineageProjection};

const ALPHA_CHANGE_LINEAGE_ROWS: &[(&str, &str)] = &[
    (
        "fixtures/review/m3/change_lineage/branch_main_worktree_ready_to_publish.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/change_lineage/branch_main_worktree_ready_to_publish.json"
        )),
    ),
    (
        "fixtures/review/m3/change_lineage/branch_blocked_by_review_required.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/change_lineage/branch_blocked_by_review_required.json"
        )),
    ),
    (
        "fixtures/review/m3/change_lineage/branch_landed_publicly_inspect_only.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/change_lineage/branch_landed_publicly_inspect_only.json"
        )),
    ),
    (
        "fixtures/review/m3/change_lineage/worktree_side_worktree_inspect_only.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/change_lineage/worktree_side_worktree_inspect_only.json"
        )),
    ),
    (
        "fixtures/review/m3/change_lineage/patch_stack_blocked_by_conflicts.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/change_lineage/patch_stack_blocked_by_conflicts.json"
        )),
    ),
];

/// Presentation label rendered for the landing-state inspector surface.
pub const CHANGE_LINEAGE_INSPECTOR_PRESENTATION_LABEL: &str = "Landing-state inspector";

/// Presentation subtitle rendered for the landing-state inspector surface.
pub const CHANGE_LINEAGE_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Explain target, ancestry, conflicts, and publish readiness before publish, merge, or apply.";

/// One landing-state row rendered in the change inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLineageInspectorRow {
    pub source_ref: &'static str,
    pub change_lineage_id: String,
    pub change_object_ref: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub active_scope_class: String,
    pub operator_caveat: String,
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
    pub base_ref: String,
    pub divergence_class: String,
    pub commits_ahead: Option<u32>,
    pub commits_behind: Option<u32>,
    pub ancestor_chain_len: usize,
    pub conflict_state_class: String,
    pub conflict_path_count: u32,
    pub conflict_notes: Vec<String>,
    pub publish_readiness_class: String,
    pub readiness_blockers: Vec<String>,
    pub readiness_notes: Vec<String>,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
}

impl ChangeLineageInspectorRow {
    fn from_projection(source_ref: &'static str, projection: ChangeLineageProjection) -> Self {
        Self {
            source_ref,
            change_lineage_id: projection.change_lineage_id,
            change_object_ref: projection.change_object_ref,
            change_object_kind: projection.change_object_kind,
            display_label: projection.display_label,
            summary: projection.summary,
            active_scope_class: projection.active_scope_class,
            operator_caveat: projection.operator_caveat,
            landing_state_class: projection.landing_state_class,
            landing_action_class: projection.landing_action_class,
            target_ref: projection.target_ref,
            target_kind: projection.target_kind,
            mutation_authority_class: projection.mutation_authority_class,
            remote_visibility_class: projection.remote_visibility_class,
            required_network_egress_class: projection.required_network_egress_class,
            pending_writes_summary: projection.pending_writes_summary,
            base_ref: projection.base_ref,
            divergence_class: projection.divergence_class,
            commits_ahead: projection.commits_ahead,
            commits_behind: projection.commits_behind,
            ancestor_chain_len: projection.ancestor_chain_len,
            conflict_state_class: projection.conflict_state_class,
            conflict_path_count: projection.conflict_path_count,
            conflict_notes: projection.conflict_notes,
            publish_readiness_class: projection.publish_readiness_class,
            readiness_blockers: projection.readiness_blockers,
            readiness_notes: projection.readiness_notes,
            consumer_surfaces: projection.consumer_surfaces,
            support_export_refs: projection.support_export_refs,
            redaction_class: projection.redaction_class,
        }
    }
}

/// Error returned when the inspector cannot project a change-lineage row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeLineageInspectorError {
    source_ref: &'static str,
    message: String,
}

impl ChangeLineageInspectorError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ChangeLineageInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for ChangeLineageInspectorError {}

/// Builds landing-state inspector rows from the checked-in alpha
/// change-lineage fixtures.
///
/// # Errors
///
/// Returns [`ChangeLineageInspectorError`] when any fixture fails to parse
/// or validate against the alpha contract.
pub fn build_alpha_change_lineage_rows(
) -> Result<Vec<ChangeLineageInspectorRow>, ChangeLineageInspectorError> {
    let mut rows = Vec::with_capacity(ALPHA_CHANGE_LINEAGE_ROWS.len());
    for (source_ref, payload) in ALPHA_CHANGE_LINEAGE_ROWS {
        let projection =
            project_change_lineage(payload).map_err(|err| projection_error(source_ref, err))?;
        rows.push(ChangeLineageInspectorRow::from_projection(
            source_ref, projection,
        ));
    }
    Ok(rows)
}

/// Renders the alpha change-lineage inspector projection as deterministic
/// plaintext for CLI / headless / docs / support consumers.
///
/// # Errors
///
/// Returns [`ChangeLineageInspectorError`] when a fixture cannot be
/// projected.
pub fn render_alpha_change_lineage_plaintext() -> Result<String, ChangeLineageInspectorError> {
    let rows = build_alpha_change_lineage_rows()?;
    let mut lines = vec![
        "Landing-state inspector alpha".to_string(),
        "change_lineage_id | kind | scope | target_ref | landing_state/action | conflict_state | publish_readiness | blockers"
            .to_string(),
    ];
    for row in rows {
        let blockers = if row.readiness_blockers.is_empty() {
            "none".to_string()
        } else {
            row.readiness_blockers.join(",")
        };
        lines.push(format!(
            "{} | {} | {} | {} | {}/{} | {} | {} | {}",
            row.change_lineage_id,
            row.change_object_kind,
            row.active_scope_class,
            row.target_ref,
            row.landing_state_class,
            row.landing_action_class,
            row.conflict_state_class,
            row.publish_readiness_class,
            blockers,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(
    source_ref: &'static str,
    err: ChangeLineageError,
) -> ChangeLineageInspectorError {
    ChangeLineageInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_change_lineage_rows_project() {
        let rows =
            build_alpha_change_lineage_rows().expect("alpha change-lineage rows must project");
        assert_eq!(rows.len(), 5);
        let mut kinds: Vec<&str> = rows
            .iter()
            .map(|row| row.change_object_kind.as_str())
            .collect();
        kinds.sort();
        kinds.dedup();
        assert!(kinds.contains(&"branch"));
        assert!(kinds.contains(&"worktree"));
        assert!(kinds.contains(&"patch_stack"));

        let mut scopes: Vec<&str> = rows
            .iter()
            .map(|row| row.active_scope_class.as_str())
            .collect();
        scopes.sort();
        scopes.dedup();
        assert!(scopes.contains(&"main_worktree"));
        assert!(scopes.contains(&"side_worktree"));
        assert!(scopes.contains(&"stacked_patch_set"));

        for row in &rows {
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "change_inspector"),
                "every row must keep the change_inspector consumer wired",
            );
            assert!(
                row.change_object_ref.starts_with("change_object_alpha:"),
                "every row must quote the underlying change-object id",
            );
        }
    }

    #[test]
    fn alpha_change_lineage_plaintext_is_deterministic() {
        let first = render_alpha_change_lineage_plaintext().expect("plaintext renders");
        let second = render_alpha_change_lineage_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Landing-state inspector alpha"));
        assert!(first.contains("ready_to_publish"));
        assert!(first.contains("blocked_by_conflicts"));
        assert!(first.contains("not_applicable_inspect_only"));
        assert!(first.contains("stacked_patch_set"));
        assert!(first.contains("side_worktree"));
    }

    #[test]
    fn presentation_labels_are_quotable() {
        assert!(CHANGE_LINEAGE_INSPECTOR_PRESENTATION_LABEL
            .to_lowercase()
            .contains("landing-state"));
        assert!(CHANGE_LINEAGE_INSPECTOR_PRESENTATION_SUBTITLE
            .to_lowercase()
            .contains("publish"));
    }
}
