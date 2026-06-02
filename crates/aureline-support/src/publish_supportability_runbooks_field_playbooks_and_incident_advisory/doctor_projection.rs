//! Doctor-facing projection for the published supportability runbook catalog.
//!
//! This submodule consumes the stable-line supportability runbook catalog and
//! projects it into doctor-facing support rows. The projection is read-only
//! and metadata-safe by construction.

use serde::{Deserialize, Serialize};

use crate::publish_supportability_runbooks_field_playbooks_and_incident_advisory::{
    current_supportability_runbook_catalog, ApprovalRequirementClass, AuthoritativePosture,
    FieldPlaybookPacket, RunbookSourceClass, RunbookStepEnvelope, StepClass, TargetSelectorScope,
    PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION, SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND,
};

/// Stable record-kind tag for the doctor-facing runbook projection.
pub const DOCTOR_RUNBOOK_PROJECTION_RECORD_KIND: &str = "doctor_runbook_projection_record";

/// Stable record-kind tag for one row in the doctor-facing projection.
pub const DOCTOR_RUNBOOK_PROJECTION_ROW_RECORD_KIND: &str = "doctor_runbook_projection_row_record";

/// One doctor-facing row projected from a runbook step envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorRunbookProjectionRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Source catalog entry id.
    pub catalog_entry_id: String,
    /// Source runbook packet id.
    pub runbook_packet_id: String,
    /// Closed runbook source class.
    pub source_class: RunbookSourceClass,
    /// Authoritative posture of the source.
    pub authoritative_posture: AuthoritativePosture,
    /// Step id.
    pub step_id: String,
    /// Closed step class.
    pub step_class: StepClass,
    /// Step title.
    pub step_title: String,
    /// Target selector scope.
    pub target_selector_scope: TargetSelectorScope,
    /// Whether the step requires external handoff.
    pub requires_external_handoff: bool,
    /// Approval requirement class.
    pub approval_requirement: ApprovalRequirementClass,
    /// Whether the step is mutating.
    pub is_mutating: bool,
    /// Whether a deviation note is required.
    pub deviation_note_required: bool,
    /// Repair hook class derived from step class and target scope.
    pub repair_hook_class: String,
    /// Whether the row is safe to surface in a doctor diagnosis context.
    pub safe_for_diagnosis_context: bool,
}

/// Doctor-facing projection of the supportability runbook catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorRunbookProjection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Catalog record kind consumed.
    pub catalog_record_kind: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Projected rows.
    pub rows: Vec<DoctorRunbookProjectionRow>,
}

impl DoctorRunbookProjection {
    /// Returns true when every row is safe for the doctor diagnosis context.
    pub fn is_safe_for_diagnosis_context(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.rows.iter().all(|r| r.safe_for_diagnosis_context)
    }
}

/// Projects the checked-in supportability runbook catalog into doctor-facing
/// rows.
///
/// # Errors
///
/// Returns a YAML parse error when the embedded fixture corpus fails to parse.
pub fn current_doctor_runbook_projection() -> Result<DoctorRunbookProjection, serde_yaml::Error> {
    let catalog = current_supportability_runbook_catalog()?;
    let generated_at = catalog.generated_at.clone();
    let mut rows = Vec::new();

    for entry in &catalog.entries {
        for step in &entry.playbook_packet.steps {
            rows.push(project_step(
                &entry.entry_id,
                &entry.source_class,
                &entry.playbook_packet,
                step,
            ));
        }
    }

    Ok(DoctorRunbookProjection {
        record_kind: DOCTOR_RUNBOOK_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: PUBLISHED_SUPPORTABILITY_SCHEMA_VERSION,
        projection_id: "support.m4.doctor_runbook_projection.baseline.v1".to_owned(),
        generated_at,
        catalog_record_kind: SUPPORTABILITY_RUNBOOK_CATALOG_RECORD_KIND.to_owned(),
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        rows,
    })
}

fn project_step(
    entry_id: &str,
    source_class: &RunbookSourceClass,
    packet: &FieldPlaybookPacket,
    step: &RunbookStepEnvelope,
) -> DoctorRunbookProjectionRow {
    let repair_hook_class = derive_repair_hook_class(step.step_class, step.target_selector_scope);
    let safe_for_diagnosis_context =
        packet.raw_private_material_excluded && packet.ambient_authority_excluded;

    DoctorRunbookProjectionRow {
        record_kind: DOCTOR_RUNBOOK_PROJECTION_ROW_RECORD_KIND.to_owned(),
        row_id: format!("doctor.row.{}.{}", packet.packet_id, step.step_id),
        catalog_entry_id: entry_id.to_owned(),
        runbook_packet_id: packet.packet_id.clone(),
        source_class: *source_class,
        authoritative_posture: packet.source_document.authoritative_posture,
        step_id: step.step_id.clone(),
        step_class: step.step_class,
        step_title: step.title.clone(),
        target_selector_scope: step.target_selector_scope,
        requires_external_handoff: step.target_selector_scope.requires_external_handoff(),
        approval_requirement: step.approval_requirement,
        is_mutating: step.step_class.is_mutating(),
        deviation_note_required: step.deviation_note_required,
        repair_hook_class,
        safe_for_diagnosis_context,
    }
}

fn derive_repair_hook_class(step_class: StepClass, scope: TargetSelectorScope) -> String {
    match (step_class, scope) {
        (StepClass::Observe, _) => "observe_only_no_repair".to_owned(),
        (StepClass::Verify, _) => "verify_target_or_route".to_owned(),
        (StepClass::Mitigate, TargetSelectorScope::LocalWorkspace) => {
            "reset_ephemeral_cache".to_owned()
        }
        (StepClass::Mitigate, TargetSelectorScope::RuntimeTarget) => {
            "reacquire_trust_approval".to_owned()
        }
        (StepClass::Mitigate, TargetSelectorScope::EnvironmentScope) => {
            "refresh_route_manifest".to_owned()
        }
        (StepClass::Mitigate, TargetSelectorScope::ServiceResource) => {
            "reset_targeted_durable_state".to_owned()
        }
        (StepClass::Mitigate, _) => "defer_to_escalation_packet".to_owned(),
        (StepClass::Rollback, _) => "rollback_or_compensate".to_owned(),
        (StepClass::Communicate, _) => "export_support_packet".to_owned(),
    }
}
