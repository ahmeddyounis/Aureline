//! Start Center template/scaffold alpha projection.
//!
//! This module turns the checked-in template/scaffold alpha packet into the
//! compact Start Center row that exposes source, signing, health, preflight,
//! rollback, and generated-project lineage truth before a starter is used.

use std::fmt;

use aureline_workspace::{
    project_template_scaffold_alpha_packet, TemplateScaffoldAlphaError,
    TemplateScaffoldAlphaProjection,
};

const TEMPLATE_SCAFFOLD_ALPHA_PACKET: (&str, &str) = (
    "artifacts/compat/template_scaffold_alpha_packet.json",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/template_scaffold_alpha_packet.json"
    )),
);

/// Start Center row for one inspectable alpha template/scaffold packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterTemplateScaffoldRow {
    /// Stable template id.
    pub template_id: String,
    /// Template card label.
    pub template_label: String,
    /// Signed manifest reference.
    pub manifest_ref: String,
    /// Source class shown next to the template name.
    pub source_class: String,
    /// Source distribution class, preserving mirror/offline vocabulary.
    pub source_distribution_class: String,
    /// Signature state shown before generation.
    pub signature_state: String,
    /// Signer label shown with source and signature state.
    pub signer_label: String,
    /// Support class shown on the starter row.
    pub support_class: String,
    /// Supported ecosystem classes.
    pub supported_ecosystems: Vec<String>,
    /// Supported platform classes.
    pub supported_platforms: Vec<String>,
    /// Required parameter count.
    pub required_parameter_count: usize,
    /// Declared hook count.
    pub declared_hook_count: usize,
    /// Declared setup-task count.
    pub declared_setup_task_count: usize,
    /// Stable preflight reference opened before writes.
    pub preflight_ref: String,
    /// Preflight target summary.
    pub target_summary: String,
    /// File-impact summary for compact review rows.
    pub file_impact_summary: String,
    /// Dependency and setup plan summaries.
    pub setup_summary: Vec<String>,
    /// Rollback checkpoint reference shown before apply.
    pub checkpoint_ref: String,
    /// Same-weight bypass path ids.
    pub bypass_path_ids: Vec<String>,
    /// Whether writes are blocked until review/export is visible.
    pub no_writes_before_review: bool,
    /// Template-health report reference.
    pub health_report_ref: String,
    /// Health roll-up state.
    pub health_state: String,
    /// Health freshness sources preserved as separate labels.
    pub health_freshness_sources: Vec<String>,
    /// Health count summary for blockers, warnings, and infos.
    pub health_counts_label: String,
    /// Scaffold-run reference.
    pub scaffold_run_ref: String,
    /// Generated-project lineage reference.
    pub lineage_ref: String,
    /// Current generated-project divergence state.
    pub divergence_state: String,
    /// Manual-edit detection state.
    pub manual_edit_detection_state: String,
    /// Update/rebase compatibility state.
    pub update_rebase_compatibility_state: String,
    /// Plain lineage metadata file reference.
    pub lineage_metadata_path_ref: String,
    /// Support-export packet refs.
    pub support_export_refs: Vec<String>,
    /// Whether raw user content may be exported.
    pub raw_content_export_allowed: bool,
}

/// Error returned when the Start Center template projection cannot load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterTemplateScaffoldError {
    source_ref: &'static str,
    message: String,
}

impl StartCenterTemplateScaffoldError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StartCenterTemplateScaffoldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for StartCenterTemplateScaffoldError {}

/// Builds Start Center template/scaffold rows from the checked-in alpha packet.
///
/// # Errors
///
/// Returns [`StartCenterTemplateScaffoldError`] if the packet cannot be parsed
/// or fails workspace-level scaffold lineage validation.
pub fn build_alpha_template_scaffold_rows(
) -> Result<Vec<StartCenterTemplateScaffoldRow>, StartCenterTemplateScaffoldError> {
    let projection = project_template_scaffold_alpha_packet(TEMPLATE_SCAFFOLD_ALPHA_PACKET.1)
        .map_err(|err| projection_error(TEMPLATE_SCAFFOLD_ALPHA_PACKET.0, err))?;
    Ok(vec![project_start_center_row(projection)])
}

/// Renders the alpha template/scaffold projection as deterministic plaintext.
///
/// # Errors
///
/// Returns [`StartCenterTemplateScaffoldError`] if the checked-in packet cannot
/// be projected.
pub fn render_alpha_template_scaffold_plaintext() -> Result<String, StartCenterTemplateScaffoldError>
{
    let rows = build_alpha_template_scaffold_rows()?;
    let mut lines = vec![
        "Template scaffold alpha".to_string(),
        "template_id | source/signature | support | health | preflight | impact | rollback | lineage".to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {}/{} | {} | {} ({}) | {} | {} | {} | {} {} {}",
            row.template_id,
            row.source_class,
            row.signature_state,
            row.support_class,
            row.health_state,
            row.health_freshness_sources.join(","),
            row.preflight_ref,
            row.file_impact_summary,
            row.checkpoint_ref,
            row.lineage_ref,
            row.divergence_state,
            row.update_rebase_compatibility_state
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn project_start_center_row(
    projection: TemplateScaffoldAlphaProjection,
) -> StartCenterTemplateScaffoldRow {
    let setup_summary = projection
        .preflight
        .dependency_plan_summary
        .iter()
        .chain(projection.preflight.setup_task_plan.iter())
        .cloned()
        .collect();
    StartCenterTemplateScaffoldRow {
        template_id: projection.template_id,
        template_label: projection.template_label,
        manifest_ref: projection.template_manifest_ref,
        source_class: projection.source_class,
        source_distribution_class: projection.source_distribution_class,
        signature_state: projection.signature_state,
        signer_label: projection.signer_label,
        support_class: projection.support_class,
        supported_ecosystems: projection.supported_ecosystems,
        supported_platforms: projection.supported_platforms,
        required_parameter_count: projection.required_parameter_count,
        declared_hook_count: projection.declared_hook_count,
        declared_setup_task_count: projection.declared_setup_task_count,
        preflight_ref: projection.preflight.preflight_ref,
        target_summary: format!(
            "{} at {}",
            projection.preflight.target_scope, projection.preflight.target_path_ref
        ),
        file_impact_summary: format!(
            "create={} modify={} delete={} dirs={}",
            projection.preflight.create_count,
            projection.preflight.modify_count,
            projection.preflight.delete_count,
            projection.preflight.directory_count
        ),
        setup_summary,
        checkpoint_ref: projection.preflight.checkpoint_ref,
        bypass_path_ids: projection.preflight.bypass_path_ids,
        no_writes_before_review: projection.preflight.no_writes_before_review,
        health_report_ref: projection.health.health_report_ref,
        health_state: projection.health.overall_state,
        health_freshness_sources: projection.health.freshness_sources,
        health_counts_label: format!(
            "blockers={} warnings={} infos={}",
            projection.health.blocker_count,
            projection.health.warning_count,
            projection.health.info_count
        ),
        scaffold_run_ref: projection.run.scaffold_run_ref,
        lineage_ref: projection.lineage.lineage_ref,
        divergence_state: projection.lineage.divergence_state,
        manual_edit_detection_state: projection.lineage.manual_edit_detection_state,
        update_rebase_compatibility_state: projection.lineage.update_rebase_compatibility_state,
        lineage_metadata_path_ref: projection.lineage.lineage_metadata_path_ref,
        support_export_refs: projection.support_export_refs,
        raw_content_export_allowed: projection.raw_content_export_allowed,
    }
}

fn projection_error(
    source_ref: &'static str,
    err: TemplateScaffoldAlphaError,
) -> StartCenterTemplateScaffoldError {
    StartCenterTemplateScaffoldError {
        source_ref,
        message: err.to_string(),
    }
}
