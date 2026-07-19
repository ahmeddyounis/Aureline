//! One reusable M5 support-intake primitive — the support-scenario picker row — so
//! issue classification starts from an explicit scenario family, a user-facing symptom
//! cue, and a claimed launch/deployment/profile scope instead of a generic "other" form
//! or free-form guesswork.
//!
//! Aureline's frozen support-intake / escalation component matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`])
//! names the support-scenario picker row as one governed component family and freezes
//! its controlled vocabulary — the scenario families, the incident scopes, and the
//! Doctor finding families the row binds, plus the surface families, the deployment
//! lines, the consumer surfaces, the accessibility routes, the qualification classes,
//! and the downgrade triggers. This module *implements* that contract as one reusable
//! resolver so a user can tell — from the picker row alone — which stable scenario
//! family a problem belongs to, the user-facing symptom cue behind it, the claimed
//! launch/deployment/profile scope, the Doctor finding family it is bound to, and how to
//! begin diagnosis without ever losing a same-weight local-only route.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_support_scenario_picker_row`] — takes one scenario's family, incident
//!    scope, bound Doctor finding family, user-facing symptom cue, claimed scope label,
//!    stable row identity, and a scenario-diagnosis-blocked signal, and produces one
//!    [`M5ResolvedScenarioPickerRow`] carrying the derived picker posture (a focused,
//!    workspace, account/device, remote-service, unmapped, or diagnosis-blocked
//!    scenario), whether the scenario-coded diagnosis can start, whether the scenario is
//!    mapped to a committed Doctor finding family, and the bounded reveal-lineage /
//!    start-diagnosis / start-local-only-diagnosis / confirm-scope / export actions. It
//!    never masks the scenario family or scope, always binds the Doctor finding lineage,
//!    and always offers the same-weight local-only diagnosis route so a user is never
//!    forced onto a remote-only path.
//!
//! A single parity matrix — [`M5ScenarioPickerRowPacket`] — binds one row per claimed M5
//! support-intake consumer (Doctor intake, support-center intake, recovery-center
//! intake, headless/CLI intake, and support-packet export) to the shared picker-row
//! anatomy, the same scenario families, incident scopes, Doctor finding families, picker
//! postures, bounded actions, export fields, and non-visual accessibility routes, so the
//! scenario / scope / symptom / finding vocabulary stays identical across desktop,
//! headless/export, and support-packet consumers.
//!
//! The scenario family ([`M5SupportScenarioFamily`]), incident scope
//! ([`M5SupportIncidentScope`]), Doctor finding family ([`M5DoctorFindingFamily`]),
//! surface family ([`M5SupportSurfaceFamily`]), deployment line
//! ([`M5SupportDeploymentLine`]), consumer surface ([`M5SupportConsumerSurface`]),
//! accessibility route ([`M5SupportAccessibilityRoute`]), qualification class
//! ([`M5SupportQualificationClass`]), and downgrade trigger
//! ([`M5SupportDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the picker
//! row itself: its intake consumers, its anatomy parts, its derived picker posture, its
//! bounded actions, and its export fields. No M5 support surface invents a second
//! scenario grammar.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! support boundary; every symptom cue, scope label, and row identity is carried only as
//! an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed,
    seeded_m5_support_scenario_picker_row_packet,
    seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed,
    M5_SUPPORT_SCENARIO_PICKER_ROW_PACKET_ID,
};

// The scenario family, incident scope, Doctor finding family, surface family, deployment
// line, consumer surface, accessibility route, qualification class, and downgrade triggers
// are frozen once, in the support-intake / escalation component matrix. This primitive
// reuses them verbatim so it never invents a parallel scenario vocabulary.
pub use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5DoctorFindingFamily, M5SupportAccessibilityRoute, M5SupportConsumerSurface,
    M5SupportDeploymentLine, M5SupportDowngradeTrigger, M5SupportIncidentScope,
    M5SupportQualificationClass, M5SupportScenarioFamily, M5SupportSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ScenarioPickerRowPacket`].
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_RECORD_KIND: &str =
    "implement_m5_support_scenario_picker_rows_and_seeded_symptom_scope_cues_with_start_diagnosis_parity_across_claimed_m5_support_intake_surfaces";

/// Schema version for M5 support-scenario-picker-row records.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the picker-row boundary schema.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-support-scenario-picker-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_DOC_REF: &str =
    "docs/support/m5_support_scenario_picker_row_primitive.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this
/// primitive narrows from.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the scenario-picker contract this primitive binds its scenario /
/// scope truth against.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_SCENARIO_PICKER_REF: &str =
    "schemas/support/scenario_picker.schema.json";

/// Repo-relative path of the Doctor-finding contract this primitive binds its scenario
/// lineage against.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_DOCTOR_FINDING_REF: &str =
    "schemas/support/doctor_finding.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-scenario-picker-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-scenario-picker-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_CSV_REF: &str =
    "artifacts/release/m5-support-scenario-picker-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_REPORT_REF: &str =
    "artifacts/design/m5-support-scenario-picker-row-primitive.md";

/// One claimed M5 support-intake consumer that renders the shared support-scenario picker
/// row. These are the consumers the acceptance criteria name — a scenario-coded start on
/// desktop Doctor / support-center / recovery-center intake, on the headless / CLI intake,
/// and in the support packet export — so the same scenario grammar works across every
/// claimed support-intake surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScenarioPickerConsumerSurface {
    /// The Project Doctor start-diagnosis intake surface.
    DoctorIntake,
    /// The support-center scenario intake surface.
    SupportCenterIntake,
    /// The recovery-center scenario intake surface.
    RecoveryCenterIntake,
    /// The headless / CLI scenario intake surface.
    HeadlessCliIntake,
    /// The support-packet export surface.
    SupportPacketExport,
}

impl M5ScenarioPickerConsumerSurface {
    /// Every claimed support-intake consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DoctorIntake,
        Self::SupportCenterIntake,
        Self::RecoveryCenterIntake,
        Self::HeadlessCliIntake,
        Self::SupportPacketExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorIntake => "doctor_intake",
            Self::SupportCenterIntake => "support_center_intake",
            Self::RecoveryCenterIntake => "recovery_center_intake",
            Self::HeadlessCliIntake => "headless_cli_intake",
            Self::SupportPacketExport => "support_packet_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DoctorIntake => "Doctor Intake",
            Self::SupportCenterIntake => "Support Center Intake",
            Self::RecoveryCenterIntake => "Recovery Center Intake",
            Self::HeadlessCliIntake => "Headless / CLI Intake",
            Self::SupportPacketExport => "Support Packet Export",
        }
    }
}

/// The derived posture of a support-scenario picker row — the resolver's verdict about
/// how a scenario-coded diagnosis begins. Computed in a fixed blocking-first order, so a
/// diagnosis-blocked or unmapped scenario never reads as a ready, mapped scenario, and a
/// wide incident scope is never understated as a focused one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScenarioPickerRowPosture {
    /// A mapped scenario scoped to a single file; scenario-coded diagnosis is ready.
    FocusedFileScenario,
    /// A mapped scenario scoped to the whole workspace; scenario-coded diagnosis is ready.
    WorkspaceScenario,
    /// A mapped scenario whose scope reaches the account or device/host; scope is
    /// confirmed before diagnosis.
    AccountOrDeviceScenario,
    /// A mapped scenario whose scope reaches a remote service or is not yet determined;
    /// scope is confirmed before diagnosis.
    RemoteServiceScenario,
    /// A scenario not yet mapped to a committed Doctor finding family; diagnosis starts by
    /// gathering evidence.
    UnmappedScenario,
    /// The scenario-coded live diagnosis path is blocked; only the local-only route
    /// remains.
    ScenarioDiagnosisBlocked,
}

impl M5ScenarioPickerRowPosture {
    /// Every picker posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FocusedFileScenario,
        Self::WorkspaceScenario,
        Self::AccountOrDeviceScenario,
        Self::RemoteServiceScenario,
        Self::UnmappedScenario,
        Self::ScenarioDiagnosisBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusedFileScenario => "focused_file_scenario",
            Self::WorkspaceScenario => "workspace_scenario",
            Self::AccountOrDeviceScenario => "account_or_device_scenario",
            Self::RemoteServiceScenario => "remote_service_scenario",
            Self::UnmappedScenario => "unmapped_scenario",
            Self::ScenarioDiagnosisBlocked => "scenario_diagnosis_blocked",
        }
    }

    /// True when a scenario at this posture can still begin its scenario-coded diagnosis.
    pub const fn can_start_scenario_diagnosis(self) -> bool {
        !matches!(self, Self::ScenarioDiagnosisBlocked)
    }

    /// True when a scenario at this posture must confirm its incident scope before
    /// diagnosis, because the scope reaches beyond the local workspace.
    pub const fn needs_scope_confirmation(self) -> bool {
        matches!(
            self,
            Self::AccountOrDeviceScenario | Self::RemoteServiceScenario
        )
    }

    /// True when the row needs operator attention before diagnosis or escalation.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::AccountOrDeviceScenario
                | Self::RemoteServiceScenario
                | Self::UnmappedScenario
                | Self::ScenarioDiagnosisBlocked
        )
    }
}

/// One bounded action a support-scenario picker row offers, so a row never hides its
/// reveal-lineage / start-diagnosis / start-local-only / confirm-scope / export
/// affordances, and never drops the same-weight local-only diagnosis route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScenarioPickerRowAction {
    /// Reveal the bound Doctor finding family and the claimed incident scope.
    RevealScenarioLineage,
    /// Begin diagnosis from the scenario-coded path.
    StartDiagnosis,
    /// Begin diagnosis from the same-weight local-only path.
    StartLocalOnlyDiagnosis,
    /// Confirm the claimed incident scope before diagnosis.
    ConfirmScope,
    /// Export the scenario row as support evidence.
    ExportScenario,
}

impl M5ScenarioPickerRowAction {
    /// Every picker action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealScenarioLineage,
        Self::StartDiagnosis,
        Self::StartLocalOnlyDiagnosis,
        Self::ConfirmScope,
        Self::ExportScenario,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealScenarioLineage => "reveal_scenario_lineage",
            Self::StartDiagnosis => "start_diagnosis",
            Self::StartLocalOnlyDiagnosis => "start_local_only_diagnosis",
            Self::ConfirmScope => "confirm_scope",
            Self::ExportScenario => "export_scenario",
        }
    }
}

/// Controlled support-scenario-picker-row anatomy part the shared row surfaces. The parts
/// in [`M5ScenarioPickerRowAnatomyPart::MANDATORY`] are required on every row so the
/// scenario family, symptom cue, scope, start-diagnosis action, and same-weight local-only
/// route are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScenarioPickerRowAnatomyPart {
    /// The stable scenario-family cue.
    ScenarioFamilyCue,
    /// The user-facing symptom cue.
    SymptomCue,
    /// The claimed launch/deployment/profile scope cue.
    ScopeCue,
    /// The bound Doctor finding lineage cue.
    DoctorFindingLineageCue,
    /// The scenario-coded start-diagnosis cue.
    StartDiagnosisCue,
    /// The same-weight local-only diagnosis route cue.
    LocalOnlyRouteCue,
    /// The bounded action row (reveal / start / confirm / export).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5ScenarioPickerRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ScenarioFamilyCue,
        Self::SymptomCue,
        Self::ScopeCue,
        Self::DoctorFindingLineageCue,
        Self::StartDiagnosisCue,
        Self::LocalOnlyRouteCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ScenarioFamilyCue,
        Self::SymptomCue,
        Self::ScopeCue,
        Self::StartDiagnosisCue,
        Self::LocalOnlyRouteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioFamilyCue => "scenario_family_cue",
            Self::SymptomCue => "symptom_cue",
            Self::ScopeCue => "scope_cue",
            Self::DoctorFindingLineageCue => "doctor_finding_lineage_cue",
            Self::StartDiagnosisCue => "start_diagnosis_cue",
            Self::LocalOnlyRouteCue => "local_only_route_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the row export carries so support-scenario-picker-row truth is reconstructable.
/// The fields in [`M5ScenarioPickerRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScenarioPickerRowExportField {
    /// The scenario family.
    ScenarioFamily,
    /// The incident scope.
    IncidentScope,
    /// The bound Doctor finding family.
    DoctorFindingFamily,
    /// The user-facing symptom cue.
    SymptomCue,
    /// The derived picker posture.
    RowPosture,
    /// Whether the scenario-coded diagnosis can start.
    CanStartScenarioDiagnosis,
    /// Whether the same-weight local-only route is available.
    LocalOnlyRouteAvailable,
    /// The bounded available actions.
    AvailableActions,
}

impl M5ScenarioPickerRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ScenarioFamily,
        Self::IncidentScope,
        Self::DoctorFindingFamily,
        Self::SymptomCue,
        Self::RowPosture,
        Self::CanStartScenarioDiagnosis,
        Self::LocalOnlyRouteAvailable,
        Self::AvailableActions,
    ];

    /// The export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ScenarioFamily,
        Self::IncidentScope,
        Self::DoctorFindingFamily,
        Self::RowPosture,
        Self::LocalOnlyRouteAvailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioFamily => "scenario_family",
            Self::IncidentScope => "incident_scope",
            Self::DoctorFindingFamily => "doctor_finding_family",
            Self::SymptomCue => "symptom_cue",
            Self::RowPosture => "row_posture",
            Self::CanStartScenarioDiagnosis => "can_start_scenario_diagnosis",
            Self::LocalOnlyRouteAvailable => "local_only_route_available",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a scenario family is not yet mapped to a committed Doctor finding family.
pub const fn scenario_is_unmapped(
    scenario_family: M5SupportScenarioFamily,
    doctor_finding_family: M5DoctorFindingFamily,
) -> bool {
    matches!(
        scenario_family,
        M5SupportScenarioFamily::UncategorizedScenario
    ) || matches!(
        doctor_finding_family,
        M5DoctorFindingFamily::UncategorizedFinding
    )
}

// ---- support-scenario-picker-row resolver -------------------------------

/// The full input to the support-scenario-picker-row resolver for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowResolutionInput {
    /// The stable scenario family.
    pub scenario_family: M5SupportScenarioFamily,
    /// The claimed incident scope.
    pub incident_scope: M5SupportIncidentScope,
    /// The bound Doctor finding family.
    pub doctor_finding_family: M5DoctorFindingFamily,
    /// The opaque user-facing symptom cue (must be non-empty).
    pub symptom_cue: String,
    /// The opaque claimed launch/deployment/profile scope label (must be non-empty).
    pub scope_label: String,
    /// The opaque stable row identity (must be non-empty).
    pub row_identity: String,
    /// True when the scenario-coded live diagnosis path is blocked by policy or
    /// unavailability (the local-only route always remains).
    pub scenario_diagnosis_blocked: bool,
}

/// The resolved support-scenario-picker-row truth for one scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedScenarioPickerRow {
    /// The stable scenario family.
    pub scenario_family: M5SupportScenarioFamily,
    /// The claimed incident scope.
    pub incident_scope: M5SupportIncidentScope,
    /// The bound Doctor finding family.
    pub doctor_finding_family: M5DoctorFindingFamily,
    /// The opaque user-facing symptom cue, preserved exactly from the input.
    pub symptom_cue: String,
    /// The opaque claimed scope label, preserved exactly from the input.
    pub scope_label: String,
    /// The opaque stable row identity, preserved exactly from the input.
    pub row_identity: String,
    /// The derived picker posture.
    pub row_posture: M5ScenarioPickerRowPosture,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5ScenarioPickerRowAction>,
    /// True when the scenario-coded diagnosis can start.
    pub can_start_scenario_diagnosis: bool,
    /// True when the same-weight local-only diagnosis route is available. Always `true`:
    /// the local-only route is never dropped.
    pub local_only_route_available: bool,
    /// True when the local-only route carries the same weight as the scenario-coded path
    /// (never a hidden fallback). Always `true`.
    pub local_only_route_same_weight: bool,
    /// True when the scenario is mapped to a committed Doctor finding family.
    pub is_scenario_mapped: bool,
    /// True when the incident scope must be confirmed before diagnosis.
    pub needs_scope_confirmation: bool,
    /// True when the row needs operator attention before diagnosis or escalation.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_support_scenario_picker_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ScenarioPickerRowResolutionError {
    /// The symptom cue was empty.
    EmptySymptomCue,
    /// The scope label was empty.
    EmptyScopeLabel,
    /// The row identity was empty.
    EmptyRowIdentity,
    /// A row descriptor carried forbidden material.
    ForbiddenScenarioMaterial,
}

impl M5ScenarioPickerRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySymptomCue => "empty_symptom_cue",
            Self::EmptyScopeLabel => "empty_scope_label",
            Self::EmptyRowIdentity => "empty_row_identity",
            Self::ForbiddenScenarioMaterial => "forbidden_scenario_material",
        }
    }
}

impl fmt::Display for M5ScenarioPickerRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "support scenario picker row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ScenarioPickerRowResolutionError {}

/// Resolves one support-scenario picker row from its declared scenario state.
///
/// The derived picker posture is computed in a fixed blocking-first order: a
/// scenario-coded diagnosis blocked by policy or unavailability wins first (only the
/// local-only route remains), then a scenario not yet mapped to a committed Doctor
/// finding family, then a scenario whose incident scope reaches a remote service or is
/// undetermined, then one whose scope reaches the account or device/host, then a
/// workspace-scoped scenario, and otherwise a focused single-file scenario. The scenario
/// family, incident scope, Doctor finding family, symptom cue, and scope label are
/// carried explicitly, never inferred away; the row always offers reveal-lineage and the
/// same-weight local-only diagnosis route, offers the scenario-coded start only when the
/// diagnosis path is not blocked, and offers confirm-scope only when the incident scope
/// reaches beyond the local workspace — so a user can begin diagnosis from a
/// scenario-coded path without ever losing a same-weight local-only route.
pub fn resolve_support_scenario_picker_row(
    input: &M5ScenarioPickerRowResolutionInput,
) -> Result<M5ResolvedScenarioPickerRow, M5ScenarioPickerRowResolutionError> {
    if input.symptom_cue.trim().is_empty() {
        return Err(M5ScenarioPickerRowResolutionError::EmptySymptomCue);
    }
    if input.scope_label.trim().is_empty() {
        return Err(M5ScenarioPickerRowResolutionError::EmptyScopeLabel);
    }
    if input.row_identity.trim().is_empty() {
        return Err(M5ScenarioPickerRowResolutionError::EmptyRowIdentity);
    }
    if value_repr_is_forbidden(&input.symptom_cue)
        || value_repr_is_forbidden(&input.scope_label)
        || value_repr_is_forbidden(&input.row_identity)
    {
        return Err(M5ScenarioPickerRowResolutionError::ForbiddenScenarioMaterial);
    }

    let is_scenario_mapped =
        !scenario_is_unmapped(input.scenario_family, input.doctor_finding_family);
    let row_posture = derive_picker_posture(
        input.scenario_family,
        input.incident_scope,
        input.doctor_finding_family,
        input.scenario_diagnosis_blocked,
    );
    let can_start_scenario_diagnosis = row_posture.can_start_scenario_diagnosis();
    let needs_scope_confirmation = row_posture.needs_scope_confirmation();
    let available_actions =
        derive_picker_actions(can_start_scenario_diagnosis, needs_scope_confirmation);

    Ok(M5ResolvedScenarioPickerRow {
        scenario_family: input.scenario_family,
        incident_scope: input.incident_scope,
        doctor_finding_family: input.doctor_finding_family,
        symptom_cue: input.symptom_cue.clone(),
        scope_label: input.scope_label.clone(),
        row_identity: input.row_identity.clone(),
        row_posture,
        available_actions,
        can_start_scenario_diagnosis,
        local_only_route_available: true,
        local_only_route_same_weight: true,
        is_scenario_mapped,
        needs_scope_confirmation,
        needs_attention: row_posture.needs_attention(),
    })
}

/// The fixed blocking-first picker-posture ladder.
fn derive_picker_posture(
    scenario_family: M5SupportScenarioFamily,
    incident_scope: M5SupportIncidentScope,
    doctor_finding_family: M5DoctorFindingFamily,
    scenario_diagnosis_blocked: bool,
) -> M5ScenarioPickerRowPosture {
    use M5ScenarioPickerRowPosture as Posture;
    use M5SupportIncidentScope as Scope;
    if scenario_diagnosis_blocked {
        Posture::ScenarioDiagnosisBlocked
    } else if scenario_is_unmapped(scenario_family, doctor_finding_family) {
        Posture::UnmappedScenario
    } else {
        match incident_scope {
            Scope::RemoteService | Scope::UnknownScope => Posture::RemoteServiceScenario,
            Scope::Account | Scope::DeviceHost => Posture::AccountOrDeviceScenario,
            Scope::Workspace => Posture::WorkspaceScenario,
            Scope::SingleFile => Posture::FocusedFileScenario,
        }
    }
}

/// Derives the bounded action set from the startable and scope-confirmation signals.
///
/// Reveal-lineage is always offered so the bound Doctor finding family and scope are
/// always inspectable; the scenario-coded start is offered only when the diagnosis path
/// is not blocked; the same-weight local-only diagnosis route is always offered so it is
/// never lost; confirm-scope is offered when the scope reaches beyond the local
/// workspace; export-scenario is always offered.
fn derive_picker_actions(
    can_start_scenario_diagnosis: bool,
    needs_scope_confirmation: bool,
) -> Vec<M5ScenarioPickerRowAction> {
    use M5ScenarioPickerRowAction as Action;
    let mut actions = vec![Action::RevealScenarioLineage];
    if can_start_scenario_diagnosis {
        actions.push(Action::StartDiagnosis);
    }
    actions.push(Action::StartLocalOnlyDiagnosis);
    if needs_scope_confirmation {
        actions.push(Action::ConfirmScope);
    }
    actions.push(Action::ExportScenario);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked support-scenario-picker-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowResolutionCase {
    /// The resolver input.
    pub input: M5ScenarioPickerRowResolutionInput,
    /// The resolved truth. Must equal `resolve_support_scenario_picker_row(&input)`.
    pub resolved: M5ResolvedScenarioPickerRow,
}

impl M5ScenarioPickerRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ScenarioPickerRowResolutionInput) -> Self {
        let resolved =
            resolve_support_scenario_picker_row(&input).expect("seed picker row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_support_scenario_picker_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved row identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.row_identity == self.input.row_identity
            && self.resolved.symptom_cue == self.input.symptom_cue
    }
}

/// One row in the primitive matrix: one support-intake consumer bound to the shared
/// picker-row anatomy, scenario families, incident scopes, Doctor finding families, picker
/// postures, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerConsumerRow {
    /// Support-intake consumer family.
    pub consumer_surface: M5ScenarioPickerConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 support / escalation surface families that render / consume this row.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ScenarioPickerRowAnatomyPart>,
    /// Scenario families this consumer distinguishes.
    pub scenario_families: Vec<M5SupportScenarioFamily>,
    /// Incident scopes this consumer distinguishes.
    pub incident_scopes: Vec<M5SupportIncidentScope>,
    /// Doctor finding families this consumer binds.
    pub doctor_finding_families: Vec<M5DoctorFindingFamily>,
    /// Picker postures this consumer distinguishes.
    pub row_postures: Vec<M5ScenarioPickerRowPosture>,
    /// Bounded picker actions this consumer offers.
    pub row_actions: Vec<M5ScenarioPickerRowAction>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5ScenarioPickerRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5SupportAccessibilityRoute>,
    /// Support / escalation subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5SupportDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked picker-row resolutions proving the resolver on this consumer.
    pub picker_examples: Vec<M5ScenarioPickerRowResolutionCase>,
    /// Hard invariant: this consumer never masks its scenario family or incident scope.
    /// MUST be `false`.
    pub masks_scenario_or_scope: bool,
    /// Hard invariant: this consumer never hides the bound Doctor finding lineage. MUST be
    /// `false`.
    pub hides_doctor_finding_lineage: bool,
    /// Hard invariant: this consumer never drops the same-weight local-only route. MUST be
    /// `false`.
    pub drops_local_only_route: bool,
    /// Hard invariant: this consumer never invents an alternate scenario grammar. MUST be
    /// `false`.
    pub invents_alternate_scenario_grammar: bool,
}

impl M5ScenarioPickerConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ScenarioPickerRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ScenarioPickerRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5ScenarioPickerRowExportField> =
            self.export_fields.iter().copied().collect();
        M5ScenarioPickerRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_scenario_or_scope
            && !self.hides_doctor_finding_lineage
            && !self.drops_local_only_route
            && !self.invents_alternate_scenario_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowVocabularySet {
    /// Support-intake-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Picker-posture tokens.
    pub row_postures: Vec<String>,
    /// Picker-action tokens.
    pub row_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Scenario-family tokens (reused from the frozen matrix).
    pub scenario_families: Vec<String>,
    /// Incident-scope tokens (reused from the frozen matrix).
    pub incident_scopes: Vec<String>,
    /// Doctor-finding-family tokens (reused from the frozen matrix).
    pub doctor_finding_families: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ScenarioPickerRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ScenarioPickerConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ScenarioPickerRowAnatomyPart::ALL, |v| v.as_str()),
            row_postures: tokens(&M5ScenarioPickerRowPosture::ALL, |v| v.as_str()),
            row_actions: tokens(&M5ScenarioPickerRowAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ScenarioPickerRowExportField::ALL, |v| v.as_str()),
            scenario_families: tokens(&M5SupportScenarioFamily::ALL, |v| v.as_str()),
            incident_scopes: tokens(&M5SupportIncidentScope::ALL, |v| v.as_str()),
            doctor_finding_families: tokens(&M5DoctorFindingFamily::ALL, |v| v.as_str()),
            surface_families: tokens(&M5SupportSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5SupportDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5SupportAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowGovernanceReview {
    /// The picker row shows its stable scenario family and user-facing symptom cue.
    pub picker_row_shows_scenario_family_and_symptom: bool,
    /// The picker row shows its claimed launch/deployment/profile scope.
    pub picker_row_shows_claimed_scope: bool,
    /// The picker row binds its scenario to a Doctor finding family.
    pub picker_row_binds_doctor_finding_family: bool,
    /// The scenario-coded start-diagnosis action is offered unless the path is blocked.
    pub start_diagnosis_always_offered_unless_blocked: bool,
    /// The same-weight local-only diagnosis route is never dropped.
    pub same_weight_local_only_route_never_dropped: bool,
    /// Scenario rows keep the same truth across every deployment line.
    pub scenario_rows_stable_across_deployment_lines: bool,
    /// Scenario rows keep the same truth across desktop, headless/export, and support
    /// packet consumers.
    pub scenario_rows_stable_across_consumer_surfaces: bool,
    /// The uncategorized scenario state is named once, never relabeled.
    pub uncategorized_scenario_named_once: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs scenario, scope, and finding truth.
    pub support_export_reconstructs_scenario_truth: bool,
    /// Later M5 rows cannot invent parallel scenario vocabulary.
    pub later_rows_cannot_invent_parallel_scenario_vocabulary: bool,
    /// No consumer masks the scenario family or the incident scope.
    pub no_surface_masks_scenario_or_scope: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowConsumerProjection {
    /// Doctor and support surfaces consume the shared scenario vocabulary.
    pub doctor_and_support_surfaces_consume_scenario_vocabulary: bool,
    /// The picker-posture resolver reads a single canonical source.
    pub scenario_posture_reads_single_source: bool,
    /// The start-diagnosis action derivation reads a single canonical source.
    pub start_diagnosis_actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop intake read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the picker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting support-case audit.
    pub support_case_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ScenarioPickerRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ScenarioPickerRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Support-intake rows.
    pub rows: Vec<M5ScenarioPickerConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScenarioPickerRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScenarioPickerRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScenarioPickerRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScenarioPickerRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScenarioPickerRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 support-scenario-picker-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScenarioPickerRowPacket {
    /// Record kind; must equal [`M5_SUPPORT_SCENARIO_PICKER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Support-intake rows.
    pub rows: Vec<M5ScenarioPickerConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScenarioPickerRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScenarioPickerRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScenarioPickerRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScenarioPickerRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScenarioPickerRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ScenarioPickerRowPacket {
    /// Builds an M5 picker-row-primitive packet from stable-lane input.
    pub fn new(input: M5ScenarioPickerRowPacketInput) -> Self {
        Self {
            record_kind: M5_SUPPORT_SCENARIO_PICKER_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 picker-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5ScenarioPickerRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORT_SCENARIO_PICKER_ROW_RECORD_KIND {
            violations.push(M5ScenarioPickerRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_VERSION {
            violations.push(M5ScenarioPickerRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ScenarioPickerRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_scenario_family_coverage(self, &mut violations);
        validate_local_route_coverage(self, &mut violations);
        validate_scenario_coded_start_coverage(self, &mut violations);
        validate_scenario_mapping_coverage(self, &mut violations);
        validate_scope_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 picker row primitive packet serializes"),
        ) {
            violations.push(M5ScenarioPickerRowViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 picker row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per support-intake consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,scenario_families,incident_scopes,doctor_finding_families,row_postures,row_actions,picker_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.scenario_families, |v| v.as_str()),
                join_tokens(&row.incident_scopes, |v| v.as_str()),
                join_tokens(&row.doctor_finding_families, |v| v.as_str()),
                join_tokens(&row.row_postures, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                row.picker_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Support-Scenario-Picker-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Support-intake consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Picker postures: {}\n",
            self.vocabulary_set.row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Picker actions: {}\n",
            self.vocabulary_set.row_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Scenario families: {}\n",
            self.vocabulary_set.scenario_families.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Support-intake consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked rows: {}\n", row.picker_examples.len()));
            for case in &row.picker_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (start `{}`, local-only `{}`, mapped `{}`)\n",
                    case.resolved.row_identity,
                    case.resolved.scenario_family.as_str(),
                    case.resolved.incident_scope.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.can_start_scenario_diagnosis,
                    case.resolved.local_only_route_available,
                    case.resolved.is_scenario_mapped,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 picker-row-primitive export.
#[derive(Debug)]
pub enum M5ScenarioPickerRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ScenarioPickerRowViolation>),
}

impl fmt::Display for M5ScenarioPickerRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 picker row primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 picker row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ScenarioPickerRowArtifactError {}

/// Validation failures emitted by [`M5ScenarioPickerRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ScenarioPickerRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required support-intake consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A support-intake row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked picker resolutions.
    PickerExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every scenario family.
    ScenarioFamilyCoverageUnproven,
    /// A worked resolution does not offer the same-weight local-only diagnosis route.
    LocalRouteCoverageUnproven,
    /// The worked resolutions do not prove both a startable and a blocked scenario-coded
    /// path.
    ScenarioCodedStartCoverageUnproven,
    /// The worked resolutions do not prove both a mapped and an unmapped scenario.
    ScenarioMappingCoverageUnproven,
    /// The worked resolutions do not prove both a focused and a scope-confirmation
    /// scenario.
    ScopeCoverageUnproven,
    /// A worked resolution does not preserve its exact row identity and symptom cue.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ScenarioPickerRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::PickerExampleMissing => "picker_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ScenarioFamilyCoverageUnproven => "scenario_family_coverage_unproven",
            Self::LocalRouteCoverageUnproven => "local_route_coverage_unproven",
            Self::ScenarioCodedStartCoverageUnproven => "scenario_coded_start_coverage_unproven",
            Self::ScenarioMappingCoverageUnproven => "scenario_mapping_coverage_unproven",
            Self::ScopeCoverageUnproven => "scope_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 picker-row-primitive export.
pub fn current_stable_m5_support_scenario_picker_row_export(
) -> Result<M5ScenarioPickerRowPacket, M5ScenarioPickerRowArtifactError> {
    let packet: M5ScenarioPickerRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-scenario-picker-row-primitive-proof/support_export.json"
    )))
    .map_err(M5ScenarioPickerRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ScenarioPickerRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_DOC_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_COMPONENT_MATRIX_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCENARIO_PICKER_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_DOCTOR_FINDING_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ScenarioPickerRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ScenarioPickerRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let present: BTreeSet<M5ScenarioPickerConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5ScenarioPickerConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ScenarioPickerRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.scenario_families.is_empty()
            || row.incident_scopes.is_empty()
            || row.doctor_finding_families.is_empty()
            || row.row_postures.is_empty()
            || row.row_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5ScenarioPickerRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ScenarioPickerRowViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5ScenarioPickerRowViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5SupportAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ScenarioPickerRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ScenarioPickerRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ScenarioPickerRowViolation::DowngradeTriggersMissing);
        }
        if row.picker_examples.is_empty() {
            violations.push(M5ScenarioPickerRowViolation::PickerExampleMissing);
        }
        if row
            .picker_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ScenarioPickerRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ScenarioPickerRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ScenarioPickerRowViolation::RowInvariantViolated);
        }
    }
}

/// Every scenario family must be exercised by some worked resolution — the implementation
/// requirement that scenario families cover, at minimum, execution-context mismatch,
/// trust/policy/identity block, network/CA/proxy/mirror failure, extension/host
/// regression, state corruption/schema drift/low-disk recovery, and remote/route/
/// collaboration mismatch, mapped onto the frozen scenario-family vocabulary.
fn validate_scenario_family_coverage(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let exercised: BTreeSet<M5SupportScenarioFamily> = packet
        .rows
        .iter()
        .flat_map(|row| row.picker_examples.iter())
        .map(|case| case.resolved.scenario_family)
        .collect();
    let covered = M5SupportScenarioFamily::ALL
        .iter()
        .all(|family| exercised.contains(family));
    if !covered {
        violations.push(M5ScenarioPickerRowViolation::ScenarioFamilyCoverageUnproven);
    }
}

/// Every worked resolution must offer the same-weight local-only diagnosis route — the
/// acceptance-criterion example that a user can begin diagnosis from a scenario-coded path
/// without losing a same-weight local-only route.
fn validate_local_route_coverage(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.picker_examples.iter())
        .all(|case| {
            case.resolved.local_only_route_available
                && case
                    .resolved
                    .available_actions
                    .contains(&M5ScenarioPickerRowAction::StartLocalOnlyDiagnosis)
        });
    if !preserved {
        violations.push(M5ScenarioPickerRowViolation::LocalRouteCoverageUnproven);
    }
}

/// At least one worked resolution must prove a startable scenario-coded path and at least
/// one must prove a blocked one — the acceptance-criterion example that a scenario-coded
/// start is offered when the path is live and withheld (never faked) when it is blocked,
/// while the local-only route still remains.
fn validate_scenario_coded_start_coverage(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let has_startable = packet.rows.iter().any(|row| {
        row.picker_examples.iter().any(|case| {
            case.resolved.can_start_scenario_diagnosis
                && case
                    .resolved
                    .available_actions
                    .contains(&M5ScenarioPickerRowAction::StartDiagnosis)
        })
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.picker_examples.iter().any(|case| {
            !case.resolved.can_start_scenario_diagnosis
                && !case
                    .resolved
                    .available_actions
                    .contains(&M5ScenarioPickerRowAction::StartDiagnosis)
                && case
                    .resolved
                    .available_actions
                    .contains(&M5ScenarioPickerRowAction::StartLocalOnlyDiagnosis)
        })
    });
    if !(has_startable && has_blocked) {
        violations.push(M5ScenarioPickerRowViolation::ScenarioCodedStartCoverageUnproven);
    }
}

/// At least one worked resolution must prove a mapped scenario and at least one must prove
/// an unmapped one — the implementation requirement that scenario vocabulary is bound to
/// the Doctor finding families rather than reinvented.
fn validate_scenario_mapping_coverage(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let has_mapped = packet.rows.iter().any(|row| {
        row.picker_examples
            .iter()
            .any(|case| case.resolved.is_scenario_mapped)
    });
    let has_unmapped = packet.rows.iter().any(|row| {
        row.picker_examples
            .iter()
            .any(|case| !case.resolved.is_scenario_mapped)
    });
    if !(has_mapped && has_unmapped) {
        violations.push(M5ScenarioPickerRowViolation::ScenarioMappingCoverageUnproven);
    }
}

/// At least one worked resolution must prove a focused scope that needs no confirmation
/// and at least one must prove a wider scope that needs confirmation — the implementation
/// requirement that claimed launch/deployment/profile scope is never left implicit or
/// understated.
fn validate_scope_coverage(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let has_focused = packet.rows.iter().any(|row| {
        row.picker_examples
            .iter()
            .any(|case| !case.resolved.needs_scope_confirmation)
    });
    let has_wide = packet.rows.iter().any(|row| {
        row.picker_examples
            .iter()
            .any(|case| case.resolved.needs_scope_confirmation)
    });
    if !(has_focused && has_wide) {
        violations.push(M5ScenarioPickerRowViolation::ScopeCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact row identity and symptom cue — the
/// invariant that the picker row never rewrites the user's scenario identity.
fn validate_identity_preservation(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.picker_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5ScenarioPickerRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.picker_row_shows_scenario_family_and_symptom,
        review.picker_row_shows_claimed_scope,
        review.picker_row_binds_doctor_finding_family,
        review.start_diagnosis_always_offered_unless_blocked,
        review.same_weight_local_only_route_never_dropped,
        review.scenario_rows_stable_across_deployment_lines,
        review.scenario_rows_stable_across_consumer_surfaces,
        review.uncategorized_scenario_named_once,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_scenario_truth,
        review.later_rows_cannot_invent_parallel_scenario_vocabulary,
        review.no_surface_masks_scenario_or_scope,
    ] {
        if !ok {
            violations.push(M5ScenarioPickerRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.doctor_and_support_surfaces_consume_scenario_vocabulary,
        projection.scenario_posture_reads_single_source,
        projection.start_diagnosis_actions_read_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ScenarioPickerRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ScenarioPickerRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ScenarioPickerRowPacket,
    violations: &mut Vec<M5ScenarioPickerRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ScenarioPickerRowViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
