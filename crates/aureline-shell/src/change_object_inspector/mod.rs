//! Shell change-object inspector — alpha consumer of the change-object family.
//!
//! This module turns the checked-in alpha change-object fixtures into the
//! deterministic landing-state rows the change-object inspector renders ahead
//! of any publish, merge, or apply action. It re-uses the
//! [`aureline_git::change_objects`] projection so the inspector, support
//! exports, review packets, and CLI / headless plaintext output read one
//! truth about where a branch, worktree, or patch stack will land.

use std::fmt;

use aureline_git::{project_change_object, ChangeObjectError, ChangeObjectProjection};

const ALPHA_CHANGE_OBJECTS: &[(&str, &str)] = &[
    (
        "fixtures/workspace/m3/change_objects/branch_local_pending_publish.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/change_objects/branch_local_pending_publish.json"
        )),
    ),
    (
        "fixtures/workspace/m3/change_objects/branch_pending_merge.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/change_objects/branch_pending_merge.json"
        )),
    ),
    (
        "fixtures/workspace/m3/change_objects/branch_landed_publicly.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/change_objects/branch_landed_publicly.json"
        )),
    ),
    (
        "fixtures/workspace/m3/change_objects/worktree_linked_local_only.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/change_objects/worktree_linked_local_only.json"
        )),
    ),
    (
        "fixtures/workspace/m3/change_objects/patch_stack_provider_pull_request.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/change_objects/patch_stack_provider_pull_request.json"
        )),
    ),
];

/// Presentation label rendered for the change-object inspector surface.
pub const CHANGE_OBJECT_INSPECTOR_PRESENTATION_LABEL: &str = "Change-object inspector";

/// Presentation subtitle rendered for the change-object inspector surface.
pub const CHANGE_OBJECT_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Inspect branches, worktrees, and patch stacks before publish, merge, or apply.";

/// One landing-state row rendered in the change-object inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeObjectInspectorRow {
    pub source_ref: &'static str,
    pub change_object_id: String,
    pub change_object_kind: String,
    pub display_label: String,
    pub summary: String,
    pub base_ref: String,
    pub divergence_class: String,
    pub commits_ahead: Option<u32>,
    pub commits_behind: Option<u32>,
    pub landing_state_class: String,
    pub landing_action_class: String,
    pub target_ref: String,
    pub target_kind: String,
    pub mutation_authority_class: String,
    pub remote_visibility_class: String,
    pub required_network_egress_class: String,
    pub pending_writes_summary: String,
    pub landing_notes: Vec<String>,
    pub variant_class_summary: String,
    pub variant_ref_label: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
}

impl ChangeObjectInspectorRow {
    fn from_projection(source_ref: &'static str, projection: ChangeObjectProjection) -> Self {
        Self {
            source_ref,
            change_object_id: projection.change_object_id,
            change_object_kind: projection.change_object_kind,
            display_label: projection.display_label,
            summary: projection.summary,
            base_ref: projection.base_ref,
            divergence_class: projection.divergence_class,
            commits_ahead: projection.commits_ahead,
            commits_behind: projection.commits_behind,
            landing_state_class: projection.landing_state_class,
            landing_action_class: projection.landing_action_class,
            target_ref: projection.target_ref,
            target_kind: projection.target_kind,
            mutation_authority_class: projection.mutation_authority_class,
            remote_visibility_class: projection.remote_visibility_class,
            required_network_egress_class: projection.required_network_egress_class,
            pending_writes_summary: projection.pending_writes_summary,
            landing_notes: projection.landing_notes,
            variant_class_summary: projection.variant_class_summary,
            variant_ref_label: projection.variant_ref_label,
            consumer_surfaces: projection.consumer_surfaces,
            support_export_refs: projection.support_export_refs,
            redaction_class: projection.redaction_class,
        }
    }
}

/// Error returned when the inspector cannot project an alpha change object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeObjectInspectorError {
    source_ref: &'static str,
    message: String,
}

impl ChangeObjectInspectorError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ChangeObjectInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for ChangeObjectInspectorError {}

/// Builds change-object inspector rows from the checked-in alpha fixtures.
///
/// # Errors
///
/// Returns [`ChangeObjectInspectorError`] when any fixture fails to parse or
/// validate against the alpha contract.
pub fn build_alpha_change_object_rows(
) -> Result<Vec<ChangeObjectInspectorRow>, ChangeObjectInspectorError> {
    let mut rows = Vec::with_capacity(ALPHA_CHANGE_OBJECTS.len());
    for (source_ref, payload) in ALPHA_CHANGE_OBJECTS {
        let projection =
            project_change_object(payload).map_err(|err| projection_error(source_ref, err))?;
        rows.push(ChangeObjectInspectorRow::from_projection(
            source_ref, projection,
        ));
    }
    Ok(rows)
}

/// Renders the alpha change-object inspector projection as deterministic
/// plaintext for CLI / headless / docs / support consumers.
///
/// # Errors
///
/// Returns [`ChangeObjectInspectorError`] when a fixture cannot be projected.
pub fn render_alpha_change_object_plaintext() -> Result<String, ChangeObjectInspectorError> {
    let rows = build_alpha_change_object_rows()?;
    let mut lines = vec![
        "Change-object inspector alpha".to_string(),
        "change_object_id | kind | base_ref/divergence | landing_state/action | target_ref | mutation_authority | remote_visibility | egress"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {}/{} | {}/{} | {} | {} | {} | {}",
            row.change_object_id,
            row.change_object_kind,
            row.base_ref,
            row.divergence_class,
            row.landing_state_class,
            row.landing_action_class,
            row.target_ref,
            row.mutation_authority_class,
            row.remote_visibility_class,
            row.required_network_egress_class,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(
    source_ref: &'static str,
    err: ChangeObjectError,
) -> ChangeObjectInspectorError {
    ChangeObjectInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_change_object_rows_project() {
        let rows = build_alpha_change_object_rows().expect("alpha change objects must project");
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
        for row in &rows {
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "change_object_inspector"),
                "every row must keep the change_object_inspector consumer wired",
            );
        }
    }

    #[test]
    fn alpha_change_object_plaintext_is_deterministic() {
        let first = render_alpha_change_object_plaintext().expect("plaintext renders");
        let second = render_alpha_change_object_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Change-object inspector alpha"));
        assert!(first.contains("pending_publish_to_remote"));
        assert!(first.contains("local_only_no_remote_yet"));
        assert!(first.contains("pending_patch_apply"));
    }

    #[test]
    fn presentation_labels_are_quotable() {
        assert!(CHANGE_OBJECT_INSPECTOR_PRESENTATION_LABEL
            .to_lowercase()
            .contains("change-object"));
        assert!(CHANGE_OBJECT_INSPECTOR_PRESENTATION_SUBTITLE
            .to_lowercase()
            .contains("publish"));
    }
}
