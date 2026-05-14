//! Generated-project lineage and scaffold-run truth.
//!
//! This module owns the export-safe alpha projection that connects a signed
//! template manifest, scaffold preflight, template-health report, scaffold run,
//! and generated-project lineage record. Shell surfaces consume the projection
//! so starter rows do not invent local hook, health, or divergence vocabulary.

use std::collections::BTreeSet;
use std::fmt;

use serde::Deserialize;

const PACKET_RECORD_KIND: &str = "template_scaffold_alpha_packet";
const MANIFEST_RECORD_KIND: &str = "template_manifest_alpha_record";
const PREFLIGHT_RECORD_KIND: &str = "scaffold_preflight_alpha_record";
const HEALTH_RECORD_KIND: &str = "template_health_report_alpha_record";
const RUN_RECORD_KIND: &str = "scaffold_run_alpha_record";
const LINEAGE_RECORD_KIND: &str = "generated_project_lineage_alpha_record";

/// Health freshness classes the alpha template-health packet must preserve.
pub const TEMPLATE_HEALTH_ALPHA_FRESHNESS_SOURCES: [&str; 4] =
    ["live", "cached", "policy-evaluated", "unchecked"];

/// Export-safe projection of one alpha template/scaffold packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateScaffoldAlphaProjection {
    /// Stable packet identifier shared by support export and Start Center.
    pub packet_id: String,
    /// Opaque manifest reference that all dependent records cite.
    pub template_manifest_ref: String,
    /// Stable template identifier from the signed manifest.
    pub template_id: String,
    /// Template version shown in preflight and lineage surfaces.
    pub template_version: String,
    /// Display label for the template card.
    pub template_label: String,
    /// Source class from the manifest signer block.
    pub source_class: String,
    /// Public, mirror, offline, repo-local, or ad-hoc distribution class.
    pub source_distribution_class: String,
    /// Signature state shown before generation.
    pub signature_state: String,
    /// Signer label shown with the source class.
    pub signer_label: String,
    /// Support posture for the selected template.
    pub support_class: String,
    /// Supported ecosystem classes.
    pub supported_ecosystems: Vec<String>,
    /// Supported platform classes.
    pub supported_platforms: Vec<String>,
    /// Count of required parameters.
    pub required_parameter_count: usize,
    /// Count of declared hooks.
    pub declared_hook_count: usize,
    /// Count of declared setup tasks.
    pub declared_setup_task_count: usize,
    /// Trust notes surfaced before apply.
    pub trust_notes: Vec<String>,
    /// Egress notes surfaced before apply.
    pub egress_notes: Vec<String>,
    /// Scaffold preflight summary.
    pub preflight: TemplateScaffoldPreflightProjection,
    /// Template-health summary.
    pub health: TemplateHealthAlphaProjection,
    /// Scaffold-run summary.
    pub run: ScaffoldRunAlphaProjection,
    /// Generated-project lineage summary.
    pub lineage: GeneratedProjectLineageAlphaProjection,
    /// Support-export packet references.
    pub support_export_refs: Vec<String>,
    /// Whether raw user content may be exported from this packet.
    pub raw_content_export_allowed: bool,
}

/// Reviewable preflight summary for a template scaffold run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateScaffoldPreflightProjection {
    /// Stable preflight record reference.
    pub preflight_ref: String,
    /// Scaffold scope, such as `new_project`.
    pub target_scope: String,
    /// Opaque workspace-relative target path reference.
    pub target_path_ref: String,
    /// Number of files created by the scaffold plan.
    pub create_count: u32,
    /// Number of files modified by the scaffold plan.
    pub modify_count: u32,
    /// Number of files deleted by the scaffold plan.
    pub delete_count: u32,
    /// Number of directories created or affected by the scaffold plan.
    pub directory_count: u32,
    /// Dependency-plan summaries, without raw commands or URLs.
    pub dependency_plan_summary: Vec<String>,
    /// Setup-task summaries, without raw commands or secrets.
    pub setup_task_plan: Vec<String>,
    /// Rollback checkpoint or delete-generated boundary.
    pub rollback_boundary_ref: String,
    /// Checkpoint planted before writes.
    pub checkpoint_ref: String,
    /// Same-weight bypass paths.
    pub bypass_path_ids: Vec<String>,
    /// True when no write may happen before review/export.
    pub no_writes_before_review: bool,
    /// Export-safe preflight review reference.
    pub review_export_ref: String,
}

/// Template-health summary shown before scaffold apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateHealthAlphaProjection {
    /// Stable template-health report reference.
    pub health_report_ref: String,
    /// Roll-up state for the report.
    pub overall_state: String,
    /// Distinct freshness sources preserved by the report.
    pub freshness_sources: Vec<String>,
    /// Runtime named by the report.
    pub runtime_scope: String,
    /// Toolchain named by the report.
    pub toolchain_scope: String,
    /// Platform classes covered by the report.
    pub platform_scope: Vec<String>,
    /// Blocking check count.
    pub blocker_count: u32,
    /// Warning check count.
    pub warning_count: u32,
    /// Informational check count.
    pub info_count: u32,
}

/// Scaffold-run summary bound to generated-project lineage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldRunAlphaProjection {
    /// Stable scaffold-run record reference.
    pub scaffold_run_ref: String,
    /// Run outcome class.
    pub outcome_class: String,
    /// Actor class that dispatched the run.
    pub actor: String,
    /// Created generated artifact references.
    pub created_artifact_refs: Vec<String>,
    /// Modified generated artifact references.
    pub modified_artifact_refs: Vec<String>,
    /// Checkpoint reference used for rollback.
    pub checkpoint_ref: String,
    /// Hook ids invoked by the run.
    pub invoked_declared_hook_ids: Vec<String>,
    /// Setup task ids invoked by the run.
    pub invoked_declared_setup_task_ids: Vec<String>,
    /// True when undeclared actions were blocked by contract.
    pub undeclared_actions_blocked: bool,
}

/// Generated-project lineage summary carried by the created project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedProjectLineageAlphaProjection {
    /// Stable generated-project lineage reference.
    pub lineage_ref: String,
    /// Generated root identity reference.
    pub generated_root_ref: String,
    /// Current divergence state.
    pub divergence_state: String,
    /// Manual-edit detection state.
    pub manual_edit_detection_state: String,
    /// Update or rebase compatibility state.
    pub update_rebase_compatibility_state: String,
    /// Last template-health report reference.
    pub last_health_report_ref: String,
    /// Opaque reference to the plain lineage metadata file.
    pub lineage_metadata_path_ref: String,
    /// True when a plain file remains authoritative.
    pub plain_file_authority: bool,
    /// True when no hidden project database is authoritative.
    pub no_hidden_project_database: bool,
    /// Public, mirror, offline, repo-local, or ad-hoc source class.
    pub mirror_or_offline_source_class: String,
}

/// Error returned when an alpha scaffold packet cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateScaffoldAlphaError {
    /// JSON parsing failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha template/scaffold contract.
    Validation(TemplateScaffoldAlphaValidationError),
}

impl TemplateScaffoldAlphaError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for TemplateScaffoldAlphaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "template scaffold JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for TemplateScaffoldAlphaError {}

/// Validation error for an alpha template/scaffold packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateScaffoldAlphaValidationError {
    message: String,
}

impl TemplateScaffoldAlphaValidationError {
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

impl fmt::Display for TemplateScaffoldAlphaValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "template scaffold validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for TemplateScaffoldAlphaValidationError {}

/// Parses and validates an alpha template/scaffold packet.
pub fn project_template_scaffold_alpha_packet(
    payload: &str,
) -> Result<TemplateScaffoldAlphaProjection, TemplateScaffoldAlphaError> {
    let packet: PacketDoc = serde_json::from_str(payload)
        .map_err(|err| TemplateScaffoldAlphaError::Json(err.to_string()))?;
    validate_packet(&packet).map_err(TemplateScaffoldAlphaError::Validation)?;
    Ok(project_packet(packet))
}

fn project_packet(packet: PacketDoc) -> TemplateScaffoldAlphaProjection {
    let preflight = TemplateScaffoldPreflightProjection {
        preflight_ref: packet.scaffold_preflight.preflight_id,
        target_scope: packet.scaffold_preflight.target.scope,
        target_path_ref: packet.scaffold_preflight.target.target_path_ref,
        create_count: packet.scaffold_preflight.file_impact.create_count,
        modify_count: packet.scaffold_preflight.file_impact.modify_count,
        delete_count: packet.scaffold_preflight.file_impact.delete_count,
        directory_count: packet.scaffold_preflight.file_impact.directory_count,
        dependency_plan_summary: packet
            .scaffold_preflight
            .dependency_plan
            .iter()
            .map(|row| row.summary.clone())
            .collect(),
        setup_task_plan: packet
            .scaffold_preflight
            .setup_task_plan
            .iter()
            .map(|row| row.summary.clone())
            .collect(),
        rollback_boundary_ref: packet
            .scaffold_preflight
            .rollback_boundary
            .checkpoint_ref
            .clone(),
        checkpoint_ref: packet.scaffold_preflight.rollback_boundary.checkpoint_ref,
        bypass_path_ids: vec![
            packet.scaffold_preflight.review_state.create_empty_path_id,
            packet
                .scaffold_preflight
                .review_state
                .continue_without_starter_path_id,
        ],
        no_writes_before_review: packet
            .scaffold_preflight
            .review_state
            .no_writes_before_review,
        review_export_ref: packet.scaffold_preflight.review_state.review_export_ref,
    };

    let health = TemplateHealthAlphaProjection {
        health_report_ref: packet.template_health_report.report_id,
        overall_state: packet.template_health_report.overall_state,
        freshness_sources: sorted_unique(packet.template_health_report.freshness_summary),
        runtime_scope: packet.template_health_report.runtime_scope.runtime,
        toolchain_scope: packet.template_health_report.runtime_scope.toolchain,
        platform_scope: packet.template_health_report.runtime_scope.platforms,
        blocker_count: packet.template_health_report.counts.blockers,
        warning_count: packet.template_health_report.counts.warnings,
        info_count: packet.template_health_report.counts.infos,
    };

    let run = ScaffoldRunAlphaProjection {
        scaffold_run_ref: packet.scaffold_run.scaffold_run_id,
        outcome_class: packet.scaffold_run.outcome_class,
        actor: packet.scaffold_run.actor,
        created_artifact_refs: packet.scaffold_run.created_artifacts,
        modified_artifact_refs: packet.scaffold_run.modified_artifacts,
        checkpoint_ref: packet.scaffold_run.checkpoint_ref,
        invoked_declared_hook_ids: packet.scaffold_run.invoked_declared_hook_ids,
        invoked_declared_setup_task_ids: packet.scaffold_run.invoked_declared_setup_task_ids,
        undeclared_actions_blocked: packet.scaffold_run.undeclared_actions_blocked,
    };

    let lineage = GeneratedProjectLineageAlphaProjection {
        lineage_ref: packet.generated_project_lineage.lineage_id,
        generated_root_ref: packet.generated_project_lineage.generated_root_ref,
        divergence_state: packet.generated_project_lineage.divergence_state,
        manual_edit_detection_state: packet.generated_project_lineage.manual_edit_detection.state,
        update_rebase_compatibility_state: packet
            .generated_project_lineage
            .update_rebase_compatibility
            .state,
        last_health_report_ref: packet.generated_project_lineage.last_health_report_ref,
        lineage_metadata_path_ref: packet.generated_project_lineage.lineage_metadata_path_ref,
        plain_file_authority: packet.generated_project_lineage.plain_file_authority,
        no_hidden_project_database: packet.generated_project_lineage.no_hidden_project_database,
        mirror_or_offline_source_class: packet
            .generated_project_lineage
            .mirror_or_offline_source_class,
    };

    TemplateScaffoldAlphaProjection {
        packet_id: packet.packet_id,
        template_manifest_ref: packet.template_manifest.manifest_ref,
        template_id: packet.template_manifest.template_id,
        template_version: packet.template_manifest.template_version,
        template_label: packet.template_manifest.display_name,
        source_class: packet.template_manifest.signer.source_class,
        source_distribution_class: packet.template_manifest.signer.source_distribution_class,
        signature_state: packet.template_manifest.signer.signature_state,
        signer_label: packet.template_manifest.signer.publisher_label,
        support_class: packet.template_manifest.support_class,
        supported_ecosystems: packet.template_manifest.supported_ecosystems,
        supported_platforms: packet.template_manifest.supported_platforms,
        required_parameter_count: packet.template_manifest.required_parameters.len(),
        declared_hook_count: packet.template_manifest.declared_hooks.len(),
        declared_setup_task_count: packet.template_manifest.declared_setup_tasks.len(),
        trust_notes: packet.template_manifest.trust_notes,
        egress_notes: packet.template_manifest.egress_notes,
        preflight,
        health,
        run,
        lineage,
        support_export_refs: packet.support_export.export_packet_refs,
        raw_content_export_allowed: packet.support_export.raw_content_export_allowed,
    }
}

fn validate_packet(packet: &PacketDoc) -> Result<(), TemplateScaffoldAlphaValidationError> {
    require_record_kind("packet", &packet.record_kind, PACKET_RECORD_KIND)?;
    require_record_kind(
        "template_manifest",
        &packet.template_manifest.record_kind,
        MANIFEST_RECORD_KIND,
    )?;
    require_record_kind(
        "scaffold_preflight",
        &packet.scaffold_preflight.record_kind,
        PREFLIGHT_RECORD_KIND,
    )?;
    require_record_kind(
        "template_health_report",
        &packet.template_health_report.record_kind,
        HEALTH_RECORD_KIND,
    )?;
    require_record_kind(
        "scaffold_run",
        &packet.scaffold_run.record_kind,
        RUN_RECORD_KIND,
    )?;
    require_record_kind(
        "generated_project_lineage",
        &packet.generated_project_lineage.record_kind,
        LINEAGE_RECORD_KIND,
    )?;

    let manifest_ref = packet.template_manifest.manifest_ref.as_str();
    require_equal(
        "preflight manifest ref",
        manifest_ref,
        &packet.scaffold_preflight.manifest_ref,
    )?;
    require_equal(
        "health manifest ref",
        manifest_ref,
        &packet.template_health_report.manifest_ref,
    )?;
    require_equal(
        "run manifest ref",
        manifest_ref,
        &packet.scaffold_run.manifest_ref,
    )?;
    require_equal(
        "lineage manifest ref",
        manifest_ref,
        &packet.generated_project_lineage.manifest_ref,
    )?;

    require_equal(
        "run preflight ref",
        &packet.scaffold_preflight.preflight_id,
        &packet.scaffold_run.preflight_ref,
    )?;
    require_equal(
        "lineage scaffold run ref",
        &packet.scaffold_run.scaffold_run_id,
        &packet.generated_project_lineage.scaffold_run_ref,
    )?;
    require_equal(
        "lineage checkpoint ref",
        &packet.scaffold_run.checkpoint_ref,
        &packet.generated_project_lineage.checkpoint_ref,
    )?;
    require_equal(
        "run checkpoint ref",
        &packet.scaffold_preflight.rollback_boundary.checkpoint_ref,
        &packet.scaffold_run.checkpoint_ref,
    )?;
    require_equal(
        "lineage health report ref",
        &packet.template_health_report.report_id,
        &packet.generated_project_lineage.last_health_report_ref,
    )?;
    require_equal(
        "lineage template id",
        &packet.template_manifest.template_id,
        &packet.generated_project_lineage.template_id,
    )?;
    require_equal(
        "lineage template version",
        &packet.template_manifest.template_version,
        &packet.generated_project_lineage.template_version,
    )?;

    if !packet
        .scaffold_preflight
        .review_state
        .no_writes_before_review
        || !packet.scaffold_run.no_writes_before_review
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "scaffold packet allows writes before review",
        ));
    }
    if !packet
        .scaffold_preflight
        .review_state
        .review_export_available
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "scaffold preflight is not exportable before apply",
        ));
    }
    if packet.support_export.raw_content_export_allowed {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "support export allows raw content",
        ));
    }
    if !packet.scaffold_run.undeclared_actions_blocked {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "scaffold run does not block undeclared actions",
        ));
    }
    if !packet.generated_project_lineage.plain_file_authority
        || !packet.generated_project_lineage.no_hidden_project_database
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "generated-project lineage is not plain-file authoritative",
        ));
    }

    validate_bypass(&packet.scaffold_preflight.review_state)?;
    validate_declared_actions(packet)?;
    validate_health(packet)?;
    validate_artifact_lineage(packet)?;
    Ok(())
}

fn validate_bypass(review: &ReviewStateDoc) -> Result<(), TemplateScaffoldAlphaValidationError> {
    if review.create_empty_path_id != "bypass.create_empty_workspace" {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "preflight is missing create-empty bypass",
        ));
    }
    if review.continue_without_starter_path_id != "bypass.continue_without_starter" {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "preflight is missing continue-without-starter bypass",
        ));
    }
    Ok(())
}

fn validate_declared_actions(
    packet: &PacketDoc,
) -> Result<(), TemplateScaffoldAlphaValidationError> {
    let declared_hooks = packet
        .template_manifest
        .declared_hooks
        .iter()
        .map(|hook| hook.hook_id.as_str())
        .collect::<BTreeSet<_>>();
    for hook_id in &packet.scaffold_run.invoked_declared_hook_ids {
        if !declared_hooks.contains(hook_id.as_str()) {
            return Err(TemplateScaffoldAlphaValidationError::new(format!(
                "scaffold run invoked undeclared hook {hook_id}"
            )));
        }
    }

    let declared_tasks = packet
        .template_manifest
        .declared_setup_tasks
        .iter()
        .map(|task| task.task_id.as_str())
        .collect::<BTreeSet<_>>();
    for task_id in &packet.scaffold_run.invoked_declared_setup_task_ids {
        if !declared_tasks.contains(task_id.as_str()) {
            return Err(TemplateScaffoldAlphaValidationError::new(format!(
                "scaffold run invoked undeclared setup task {task_id}"
            )));
        }
    }
    for planned in &packet.scaffold_preflight.setup_task_plan {
        if !planned.declared_in_manifest || !declared_tasks.contains(planned.task_id.as_str()) {
            return Err(TemplateScaffoldAlphaValidationError::new(format!(
                "preflight planned undeclared setup task {}",
                planned.task_id
            )));
        }
    }
    Ok(())
}

fn validate_health(packet: &PacketDoc) -> Result<(), TemplateScaffoldAlphaValidationError> {
    let sources = packet
        .template_health_report
        .checks
        .iter()
        .map(|check| check.freshness_source.as_str())
        .collect::<BTreeSet<_>>();
    for required in TEMPLATE_HEALTH_ALPHA_FRESHNESS_SOURCES {
        if !sources.contains(required) {
            return Err(TemplateScaffoldAlphaValidationError::new(format!(
                "template health report is missing freshness source {required}"
            )));
        }
    }

    let blockers = count_severity(&packet.template_health_report.checks, "blocker");
    let warnings = count_severity(&packet.template_health_report.checks, "warning");
    let infos = count_severity(&packet.template_health_report.checks, "info");
    if packet.template_health_report.counts.blockers != blockers
        || packet.template_health_report.counts.warnings != warnings
        || packet.template_health_report.counts.infos != infos
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "template health counts do not match check severities",
        ));
    }
    Ok(())
}

fn validate_artifact_lineage(
    packet: &PacketDoc,
) -> Result<(), TemplateScaffoldAlphaValidationError> {
    if packet.scaffold_preflight.file_impact.create_count == 0
        && packet.scaffold_preflight.file_impact.modify_count == 0
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "preflight has no created or modified file impact",
        ));
    }
    if packet
        .generated_project_lineage
        .lineage_metadata_path_ref
        .trim()
        .is_empty()
    {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "generated-project lineage metadata path ref is empty",
        ));
    }

    let run_created = packet
        .scaffold_run
        .created_artifacts
        .iter()
        .collect::<BTreeSet<_>>();
    let lineage_created = packet
        .generated_project_lineage
        .created_artifact_refs
        .iter()
        .collect::<BTreeSet<_>>();
    if run_created != lineage_created {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "created artifacts differ between scaffold run and lineage record",
        ));
    }

    let run_modified = packet
        .scaffold_run
        .modified_artifacts
        .iter()
        .collect::<BTreeSet<_>>();
    let lineage_modified = packet
        .generated_project_lineage
        .modified_artifact_refs
        .iter()
        .collect::<BTreeSet<_>>();
    if run_modified != lineage_modified {
        return Err(TemplateScaffoldAlphaValidationError::new(
            "modified artifacts differ between scaffold run and lineage record",
        ));
    }
    Ok(())
}

fn require_record_kind(
    label: &str,
    actual: &str,
    expected: &str,
) -> Result<(), TemplateScaffoldAlphaValidationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(TemplateScaffoldAlphaValidationError::new(format!(
            "{label} record_kind is {actual}, expected {expected}"
        )))
    }
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), TemplateScaffoldAlphaValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(TemplateScaffoldAlphaValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn count_severity(checks: &[HealthCheckDoc], severity: &str) -> u32 {
    checks
        .iter()
        .filter(|check| check.severity == severity)
        .count() as u32
}

fn sorted_unique(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Debug, Deserialize)]
struct PacketDoc {
    record_kind: String,
    packet_id: String,
    template_manifest: ManifestDoc,
    scaffold_preflight: PreflightDoc,
    template_health_report: HealthReportDoc,
    scaffold_run: RunDoc,
    generated_project_lineage: LineageDoc,
    support_export: SupportExportDoc,
}

#[derive(Debug, Deserialize)]
struct ManifestDoc {
    record_kind: String,
    manifest_ref: String,
    template_id: String,
    template_version: String,
    display_name: String,
    signer: SignerDoc,
    support_class: String,
    supported_ecosystems: Vec<String>,
    supported_platforms: Vec<String>,
    required_parameters: Vec<serde_json::Value>,
    declared_hooks: Vec<HookDoc>,
    declared_setup_tasks: Vec<SetupTaskDoc>,
    trust_notes: Vec<String>,
    egress_notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SignerDoc {
    source_class: String,
    source_distribution_class: String,
    signature_state: String,
    publisher_label: String,
}

#[derive(Debug, Deserialize)]
struct HookDoc {
    hook_id: String,
}

#[derive(Debug, Deserialize)]
struct SetupTaskDoc {
    task_id: String,
}

#[derive(Debug, Deserialize)]
struct PreflightDoc {
    record_kind: String,
    preflight_id: String,
    manifest_ref: String,
    target: TargetDoc,
    file_impact: FileImpactDoc,
    dependency_plan: Vec<DependencyPlanDoc>,
    setup_task_plan: Vec<SetupTaskPlanDoc>,
    rollback_boundary: RollbackBoundaryDoc,
    review_state: ReviewStateDoc,
}

#[derive(Debug, Deserialize)]
struct TargetDoc {
    target_path_ref: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct FileImpactDoc {
    create_count: u32,
    modify_count: u32,
    delete_count: u32,
    directory_count: u32,
}

#[derive(Debug, Deserialize)]
struct DependencyPlanDoc {
    summary: String,
}

#[derive(Debug, Deserialize)]
struct SetupTaskPlanDoc {
    task_id: String,
    declared_in_manifest: bool,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct RollbackBoundaryDoc {
    checkpoint_ref: String,
}

#[derive(Debug, Deserialize)]
struct ReviewStateDoc {
    no_writes_before_review: bool,
    review_export_available: bool,
    review_export_ref: String,
    create_empty_path_id: String,
    continue_without_starter_path_id: String,
}

#[derive(Debug, Deserialize)]
struct HealthReportDoc {
    record_kind: String,
    report_id: String,
    manifest_ref: String,
    overall_state: String,
    freshness_summary: Vec<String>,
    runtime_scope: RuntimeScopeDoc,
    counts: HealthCountsDoc,
    checks: Vec<HealthCheckDoc>,
}

#[derive(Debug, Deserialize)]
struct RuntimeScopeDoc {
    runtime: String,
    toolchain: String,
    platforms: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HealthCountsDoc {
    blockers: u32,
    warnings: u32,
    infos: u32,
}

#[derive(Debug, Deserialize)]
struct HealthCheckDoc {
    severity: String,
    freshness_source: String,
}

#[derive(Debug, Deserialize)]
struct RunDoc {
    record_kind: String,
    scaffold_run_id: String,
    manifest_ref: String,
    preflight_ref: String,
    created_artifacts: Vec<String>,
    modified_artifacts: Vec<String>,
    invoked_declared_hook_ids: Vec<String>,
    invoked_declared_setup_task_ids: Vec<String>,
    checkpoint_ref: String,
    outcome_class: String,
    actor: String,
    no_writes_before_review: bool,
    undeclared_actions_blocked: bool,
}

#[derive(Debug, Deserialize)]
struct LineageDoc {
    record_kind: String,
    lineage_id: String,
    manifest_ref: String,
    template_id: String,
    template_version: String,
    generated_root_ref: String,
    scaffold_run_ref: String,
    created_artifact_refs: Vec<String>,
    modified_artifact_refs: Vec<String>,
    checkpoint_ref: String,
    divergence_state: String,
    manual_edit_detection: ManualEditDetectionDoc,
    update_rebase_compatibility: UpdateRebaseCompatibilityDoc,
    last_health_report_ref: String,
    lineage_metadata_path_ref: String,
    mirror_or_offline_source_class: String,
    plain_file_authority: bool,
    no_hidden_project_database: bool,
}

#[derive(Debug, Deserialize)]
struct ManualEditDetectionDoc {
    state: String,
}

#[derive(Debug, Deserialize)]
struct UpdateRebaseCompatibilityDoc {
    state: String,
}

#[derive(Debug, Deserialize)]
struct SupportExportDoc {
    export_packet_refs: Vec<String>,
    raw_content_export_allowed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PACKET: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/template_scaffold_alpha_packet.json"
    ));

    #[test]
    fn alpha_packet_validates_and_projects_lineage() {
        let projection = project_template_scaffold_alpha_packet(PACKET).expect("valid packet");

        assert_eq!(
            projection.template_id,
            "template.alpha.ts_web.vite_react_seed"
        );
        assert_eq!(projection.source_class, "first_party");
        assert_eq!(projection.signature_state, "verified");
        assert_eq!(projection.preflight.create_count, 9);
        assert!(projection.preflight.no_writes_before_review);
        assert_eq!(
            projection.health.freshness_sources,
            vec!["cached", "live", "policy-evaluated", "unchecked"]
        );
        assert_eq!(
            projection.run.checkpoint_ref,
            "checkpoint:scaffold.typescript_web_vite_local.001"
        );
        assert_eq!(projection.lineage.divergence_state, "in_sync");
        assert!(projection.lineage.plain_file_authority);
        assert!(!projection.raw_content_export_allowed);
    }
}
