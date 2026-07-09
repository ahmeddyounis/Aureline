//! Rotation/revoke-event rows and export-safety banners carrying the credential class,
//! the prior / new lifecycle state, the derived credential-continuity class, the impacted
//! running sessions / queued jobs / remembered decisions, the recovery next step, and the
//! audit / export actions — and, for the export-safety banner, the export surface, the
//! export-safety class, the reveal posture, the derived redaction posture, whether
//! handle-class / source labels are preserved, and the always-stated raw-secret-excluded
//! default.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_credential_component_matrix`] — the `rotation_revoke_event_row`
//! and the `export_safety_banner` — into one implemented, export-safe packet with two
//! co-equal control vectors. Together they keep credential lifecycle and export behavior
//! explicit after rotation, revoke, expiry, or a support / export handoff event.
//!
//! A [`RotationRevokeEventRow`] always names the credential class, the prior and new
//! lifecycle state, the impacted running sessions / queued jobs / remembered decisions,
//! the recovery next step, and the audit event, and offers keyboard-complete
//! follow-recovery / export-audit-evidence actions. Its continuity class is *derived* from
//! the new lifecycle state rather than asserted: a revoked or expired credential can never
//! read as still usable, so what rotation or revoke will impact never has to be inferred.
//!
//! An [`ExportSafetyBanner`] always states that raw credentials are excluded by default
//! from profiles, support bundles, handoff packets, recipes, and portable workspace
//! exports, and preserves handle-class / source labels where allowed. Its redaction posture
//! is *derived* from the export-safety class rather than asserted: an export never implies
//! a raw secret is exportable, and credential exclusion is never left to implication.
//!
//! The credential classes ([`M5CredentialClass`]), lifecycle states
//! ([`M5CredentialLifecycleState`]), export-safety classes
//! ([`M5CredentialExportSafetyClass`]), reveal postures
//! ([`M5CredentialRevealPosture`]), degraded states ([`M5CredentialDegradedState`]),
//! required labels ([`M5CredentialRequiredLabel`]), surface families
//! ([`M5CredentialSurfaceFamily`]), deployment lines ([`M5CredentialDeploymentLine`]),
//! consumer surfaces ([`M5CredentialConsumerSurface`]), accessibility routes
//! ([`M5CredentialAccessibilityRoute`]), and downgrade triggers
//! ([`M5CredentialDowngradeTrigger`]) are reused directly from the frozen matrix, so this
//! lane never invents a parallel credential vocabulary. It mints new vocabulary only for
//! what that matrix left implicit about these two controls: the derived credential
//! continuity class, the impacted-workflow class, the event-row actions, the derived
//! export-safety posture, the export-surface class, and the banner actions.
//!
//! Raw secret values, pasted tokens, passphrases, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json`](../../../../schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json).
//! The contract doc is
//! [`docs/security/implement_rotation_revoke_event_rows_and_export_safety_banners.md`](../../../../docs/security/implement_rotation_revoke_event_rows_and_export_safety_banners.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_rotation_revoke_export_safety_controls,
    seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded,
    seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows,
    ROTATION_REVOKE_EXPORT_SAFETY_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The credential class, lifecycle state, export-safety class, reveal posture, degraded
// state, required labels, surface family, deployment line, consumer surface, accessibility
// route, and downgrade triggers are frozen once, in the credential component matrix. This
// lane reuses them verbatim so it never invents a parallel credential vocabulary.
use crate::freeze_the_m5_credential_component_matrix::{
    M5CredentialAccessibilityRoute, M5CredentialClass, M5CredentialComponentFamily,
    M5CredentialConsumerSurface, M5CredentialDegradedState, M5CredentialDeploymentLine,
    M5CredentialDowngradeTrigger, M5CredentialExportSafetyClass, M5CredentialLifecycleState,
    M5CredentialRequiredLabel, M5CredentialRevealPosture, M5CredentialSurfaceFamily,
    M5_CREDENTIAL_COMPONENT_DOC_REF, M5_CREDENTIAL_COMPONENT_FOUNDATION_EXPORT_REDACTION_REF,
    M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF, M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
    M5_EXPORT_SAFETY_BANNER_SCHEMA_REF, M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`RotationRevokeExportSafetyControlsPacket`].
pub const ROTATION_REVOKE_EXPORT_SAFETY_RECORD_KIND: &str =
    "rotation_revoke_export_safety_controls";

/// Schema version for rotation/revoke / export-safety control records.
pub const ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF: &str =
    "schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const ROTATION_REVOKE_EXPORT_SAFETY_DOC_REF: &str =
    "docs/security/implement_rotation_revoke_event_rows_and_export_safety_banners.md";

/// Repo-relative path of the protected fixture directory.
pub const ROTATION_REVOKE_EXPORT_SAFETY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-rotation-revoke-export-safety-controls";

/// Repo-relative path of the checked support-export artifact.
pub const ROTATION_REVOKE_EXPORT_SAFETY_ARTIFACT_REF: &str =
    "artifacts/release/m5-rotation-revoke-export-safety-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ROTATION_REVOKE_EXPORT_SAFETY_SUMMARY_REF: &str =
    "artifacts/release/m5-rotation-revoke-export-safety-proof/summary.md";

// ---- rotation-revoke-event-row vocabulary -------------------------------

/// Derived credential-continuity class a rotation/revoke-event row may present.
///
/// This is the lifecycle honesty axis: the class is derived from the new lifecycle state,
/// never asserted, so a revoked or expired credential can never read as still usable, and
/// what rotation or revoke will impact never has to be inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialContinuityClass {
    /// The credential is active and current after the event.
    StillActive,
    /// The credential still works but needs a refresh or rotation soon.
    ActionRequired,
    /// The credential is revoked or expired and can no longer be used.
    NoLongerUsable,
    /// The credential has been superseded by a newer one.
    Superseded,
}

impl CredentialContinuityClass {
    /// Every continuity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StillActive,
        Self::ActionRequired,
        Self::NoLongerUsable,
        Self::Superseded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StillActive => "still_active",
            Self::ActionRequired => "action_required",
            Self::NoLongerUsable => "no_longer_usable",
            Self::Superseded => "superseded",
        }
    }
}

/// The kind of workflow a rotation or revoke event impacts, so a row never leaves what is
/// affected implicit and a user can always see the running sessions, queued jobs, or
/// remembered decisions in play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactedWorkflowClass {
    /// A running interactive session using this credential.
    RunningSession,
    /// A queued or in-flight background job.
    QueuedJob,
    /// A remembered decision / saved choice that referenced this credential.
    RememberedDecision,
    /// A scheduled automation or recurring task.
    ScheduledAutomation,
    /// A delegated forward that relayed this credential onward.
    DelegatedForward,
    /// No active workflow is affected.
    NoActiveImpact,
}

impl ImpactedWorkflowClass {
    /// Every impacted-workflow class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunningSession,
        Self::QueuedJob,
        Self::RememberedDecision,
        Self::ScheduledAutomation,
        Self::DelegatedForward,
        Self::NoActiveImpact,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunningSession => "running_session",
            Self::QueuedJob => "queued_job",
            Self::RememberedDecision => "remembered_decision",
            Self::ScheduledAutomation => "scheduled_automation",
            Self::DelegatedForward => "delegated_forward",
            Self::NoActiveImpact => "no_active_impact",
        }
    }
}

/// One keyboard-complete default action a rotation/revoke-event row offers, so a row never
/// hides its recovery or audit / export affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RotationRevokeEventRowAction {
    /// Follow the recovery next step for this event.
    FollowRecoveryStep,
    /// View the running sessions, queued jobs, and remembered decisions this event impacts.
    ViewImpactedWorkflows,
    /// Rotate the credential now.
    RotateNow,
    /// Revoke the credential now.
    RevokeNow,
    /// Open the underlying audit event.
    OpenAuditEvent,
    /// Export the row as export-safe audit evidence.
    ExportAuditEvidence,
}

impl RotationRevokeEventRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FollowRecoveryStep,
        Self::ViewImpactedWorkflows,
        Self::RotateNow,
        Self::RevokeNow,
        Self::OpenAuditEvent,
        Self::ExportAuditEvidence,
    ];

    /// The recovery / audit-export semantics every keyboard-complete event row must offer.
    pub const MANDATORY: [Self; 2] = [Self::FollowRecoveryStep, Self::ExportAuditEvidence];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowRecoveryStep => "follow_recovery_step",
            Self::ViewImpactedWorkflows => "view_impacted_workflows",
            Self::RotateNow => "rotate_now",
            Self::RevokeNow => "revoke_now",
            Self::OpenAuditEvent => "open_audit_event",
            Self::ExportAuditEvidence => "export_audit_evidence",
        }
    }
}

/// Disclosures a rotation/revoke-event row must carry, derived from the new lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialContinuityDisclosure {
    /// The derived continuity class this row may present.
    pub continuity_class: CredentialContinuityClass,
    /// Whether the credential is still usable after the event.
    pub is_still_usable: bool,
    /// Whether the row must carry an explicit still-active note.
    pub needs_still_active_note: bool,
    /// Whether the row must carry an explicit action-required note.
    pub needs_action_required_note: bool,
    /// Whether the row must carry an explicit no-longer-usable note.
    pub needs_no_longer_usable_note: bool,
    /// Whether the row must carry an explicit superseded note.
    pub needs_superseded_note: bool,
}

/// Resolves the continuity truth a rotation/revoke-event row may present.
///
/// An active-current credential is still active. A refresh-needed or rotation-due
/// credential still works but needs action. A revoked or expired credential can no longer
/// be used. A superseded credential has been replaced — none of which can be asserted; a
/// revoked or expired credential can never read as still usable.
pub fn resolve_credential_continuity(
    new_state: M5CredentialLifecycleState,
) -> CredentialContinuityDisclosure {
    use CredentialContinuityClass as Continuity;
    use M5CredentialLifecycleState as Lifecycle;

    let continuity_class = match new_state {
        Lifecycle::ActiveCurrent => Continuity::StillActive,
        Lifecycle::RefreshNeeded | Lifecycle::RotationDue => Continuity::ActionRequired,
        Lifecycle::Revoked | Lifecycle::Expired => Continuity::NoLongerUsable,
        Lifecycle::Superseded => Continuity::Superseded,
    };

    CredentialContinuityDisclosure {
        continuity_class,
        is_still_usable: matches!(
            continuity_class,
            Continuity::StillActive | Continuity::ActionRequired
        ),
        needs_still_active_note: matches!(continuity_class, Continuity::StillActive),
        needs_action_required_note: matches!(continuity_class, Continuity::ActionRequired),
        needs_no_longer_usable_note: matches!(continuity_class, Continuity::NoLongerUsable),
        needs_superseded_note: matches!(continuity_class, Continuity::Superseded),
    }
}

/// A rotation/revoke-event row naming credential class, prior / new lifecycle state,
/// impacted workflows, recovery next step, audit event, and derived continuity class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRevokeEventRow {
    /// Frozen component this control implements; must be `rotation_revoke_event_row`.
    pub component: M5CredentialComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Credential class this event concerns, reused from the frozen matrix.
    pub credential_class: M5CredentialClass,
    /// Credential-identity label / which credential object this event concerns; required.
    pub credential_id_label: String,
    /// Prior lifecycle state before the event, reused from the frozen matrix.
    pub prior_state: M5CredentialLifecycleState,
    /// New lifecycle state after the event, reused from the frozen matrix.
    pub new_state: M5CredentialLifecycleState,
    /// Prior-state note; always required so the prior state stays explicit.
    pub prior_state_note: String,
    /// New-state note; always required so the new state stays explicit.
    pub new_state_note: String,
    /// Derived continuity class (must equal the resolved class).
    pub continuity_class: CredentialContinuityClass,
    /// Whether the row claims the credential is still usable (must equal the derived truth).
    pub claims_still_usable: bool,
    /// Still-active note; required when the credential remains active after the event.
    pub still_active_note: String,
    /// Action-required note; required when the credential needs refresh / rotation.
    pub action_required_note: String,
    /// No-longer-usable note; required when the credential is revoked / expired.
    pub no_longer_usable_note: String,
    /// Superseded note; required when the credential has been superseded.
    pub superseded_note: String,
    /// Impacted workflow classes; required and non-empty so the impact is never implicit.
    pub impacted_workflows: Vec<ImpactedWorkflowClass>,
    /// Impacted-workflows note naming affected sessions / jobs / decisions; always required.
    pub impacted_workflows_note: String,
    /// Recovery next-step note; always required so recovery is never left to inference.
    pub recovery_next_step_note: String,
    /// Audit-event note; always required so the audit trail stays explicit.
    pub audit_note: String,
    /// Keyboard-complete default actions (must include the mandatory recovery / export).
    pub default_actions: Vec<RotationRevokeEventRowAction>,
    /// Degraded states this row can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks the workflows a rotation / revoke impacts. MUST be `false`.
    pub masks_impacted_workflows: bool,
    /// Hard invariant: never implies a raw secret is export-safe. MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: friendly "connected" wording never conceals the lifecycle state or
    /// the impact. MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl RotationRevokeEventRow {
    /// Continuity disclosures this row must carry, derived from the new lifecycle state.
    pub fn continuity_disclosure(&self) -> CredentialContinuityDisclosure {
        resolve_credential_continuity(self.new_state)
    }

    /// Whether the row offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RotationRevokeEventRowAction> =
            self.default_actions.iter().copied().collect();
        RotationRevokeEventRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }
}

// ---- export-safety-banner vocabulary ------------------------------------

/// Derived export-safety / redaction posture an export-safety banner may present.
///
/// This is the export honesty axis: the posture is derived from the export-safety class,
/// never asserted, so an export never implies a raw secret is exportable and credential
/// exclusion is never left to implication. Raw secret values are excluded in every posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSafetyPosture {
    /// Raw secrets excluded; handle-class / source labels preserved.
    RawExcludedLabelsPreserved,
    /// Only a handle reference is exported.
    HandleReferenceOnly,
    /// A redacted or endpoint-masked share is exported.
    RedactedOrMasked,
    /// Export is blocked entirely; not even a handle is exported.
    FullyBlocked,
}

impl ExportSafetyPosture {
    /// Every export-safety posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RawExcludedLabelsPreserved,
        Self::HandleReferenceOnly,
        Self::RedactedOrMasked,
        Self::FullyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawExcludedLabelsPreserved => "raw_excluded_labels_preserved",
            Self::HandleReferenceOnly => "handle_reference_only",
            Self::RedactedOrMasked => "redacted_or_masked",
            Self::FullyBlocked => "fully_blocked",
        }
    }
}

/// The export surface an export-safety banner governs, so a banner names which of profiles,
/// support bundles, handoff packets, recipes, and portable workspace exports it applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSurfaceClass {
    /// A shareable profile export.
    Profile,
    /// A support / diagnostics bundle.
    SupportBundle,
    /// An offline handoff packet.
    HandoffPacket,
    /// A shared recipe / workflow template.
    Recipe,
    /// A portable workspace export.
    PortableWorkspace,
    /// An audit-log export.
    AuditLog,
}

impl ExportSurfaceClass {
    /// Every export surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Profile,
        Self::SupportBundle,
        Self::HandoffPacket,
        Self::Recipe,
        Self::PortableWorkspace,
        Self::AuditLog,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Profile => "profile",
            Self::SupportBundle => "support_bundle",
            Self::HandoffPacket => "handoff_packet",
            Self::Recipe => "recipe",
            Self::PortableWorkspace => "portable_workspace",
            Self::AuditLog => "audit_log",
        }
    }
}

/// One keyboard-complete default action an export-safety banner offers, so a banner never
/// hides its redaction-policy or excluded-fields affordance behind a pointer-only gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSafetyBannerAction {
    /// View the full redaction policy behind this export.
    ViewRedactionPolicy,
    /// View exactly which fields are excluded from export.
    ViewExcludedFields,
    /// View the handle-class / source labels preserved in export.
    ViewPreservedLabels,
    /// Export a redacted, export-safe copy.
    ExportRedactedCopy,
    /// Report an unexpected exposure in an export.
    ReportUnexpectedExposure,
}

impl ExportSafetyBannerAction {
    /// Every banner action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ViewRedactionPolicy,
        Self::ViewExcludedFields,
        Self::ViewPreservedLabels,
        Self::ExportRedactedCopy,
        Self::ReportUnexpectedExposure,
    ];

    /// The redaction-policy / excluded-fields semantics every keyboard-complete banner must
    /// offer.
    pub const MANDATORY: [Self; 2] = [Self::ViewRedactionPolicy, Self::ViewExcludedFields];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewRedactionPolicy => "view_redaction_policy",
            Self::ViewExcludedFields => "view_excluded_fields",
            Self::ViewPreservedLabels => "view_preserved_labels",
            Self::ExportRedactedCopy => "export_redacted_copy",
            Self::ReportUnexpectedExposure => "report_unexpected_exposure",
        }
    }
}

/// Disclosures an export-safety banner must carry, derived from the export-safety class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportSafetyDisclosure {
    /// The derived export-safety posture this banner may present.
    pub export_safety_posture: ExportSafetyPosture,
    /// Whether handle-class / source labels are preserved in this export.
    pub preserves_handle_class_labels: bool,
    /// Whether the banner must carry an explicit preserved-handle-label note.
    pub needs_handle_label_note: bool,
    /// Whether the banner must carry an explicit redaction note.
    pub needs_redaction_note: bool,
    /// Whether the banner must carry an explicit fully-blocked note.
    pub needs_blocked_note: bool,
}

/// Resolves the export-safety truth an export-safety banner may present.
///
/// A raw-secret-excluded or metadata-only export preserves handle-class / source labels. A
/// handle-reference-only export exports only a handle. A redacted-share or endpoints-masked
/// export is a redacted share. A blocked export exports nothing — none of which can be
/// asserted; raw secret values are excluded in every posture, so a banner never implies a
/// raw secret is exportable.
pub fn resolve_export_safety_posture(
    export_safety_class: M5CredentialExportSafetyClass,
) -> ExportSafetyDisclosure {
    use ExportSafetyPosture as Posture;
    use M5CredentialExportSafetyClass as Safety;

    let export_safety_posture = match export_safety_class {
        Safety::RawSecretExcluded | Safety::MetadataOnly => Posture::RawExcludedLabelsPreserved,
        Safety::HandleReferenceOnly => Posture::HandleReferenceOnly,
        Safety::RedactedShare | Safety::EndpointsMasked => Posture::RedactedOrMasked,
        Safety::ExportBlocked => Posture::FullyBlocked,
    };

    ExportSafetyDisclosure {
        export_safety_posture,
        preserves_handle_class_labels: !matches!(export_safety_posture, Posture::FullyBlocked),
        needs_handle_label_note: matches!(
            export_safety_posture,
            Posture::RawExcludedLabelsPreserved | Posture::HandleReferenceOnly
        ),
        needs_redaction_note: matches!(export_safety_posture, Posture::RedactedOrMasked),
        needs_blocked_note: matches!(export_safety_posture, Posture::FullyBlocked),
    }
}

/// An export-safety banner naming its export surface, export-safety class, reveal posture,
/// derived redaction posture, and always-stated raw-secret-excluded default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSafetyBanner {
    /// Frozen component this control implements; must be `export_safety_banner`.
    pub component: M5CredentialComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// The export surface this banner governs.
    pub export_surface_class: ExportSurfaceClass,
    /// Export-surface note; always required so the export surface stays explicit.
    pub export_surface_note: String,
    /// Export-safety class, reused from the frozen matrix.
    pub export_safety_class: M5CredentialExportSafetyClass,
    /// Reveal posture behind this export, reused from the frozen matrix.
    pub reveal_posture: M5CredentialRevealPosture,
    /// Derived export-safety posture (must equal the resolved posture).
    pub export_safety_posture: ExportSafetyPosture,
    /// Whether the banner claims handle-class / source labels are preserved (must equal the
    /// derived truth).
    pub claims_preserves_handle_labels: bool,
    /// Raw-secret-excluded note; always required so the default exclusion stays explicit.
    pub raw_secret_excluded_note: String,
    /// Preserved-handle-label note; required when handle-class / source labels are preserved.
    pub handle_label_note: String,
    /// Redaction note; required when a redacted / masked share is exported.
    pub redaction_note: String,
    /// Fully-blocked note; required when export is blocked entirely.
    pub blocked_note: String,
    /// Preserved-labels note; always required so what is preserved stays explicit.
    pub preserved_labels_note: String,
    /// Reveal-posture note; always required so the reveal posture stays explicit.
    pub reveal_posture_note: String,
    /// Keyboard-complete default actions (must include the mandatory policy / excluded-fields).
    pub default_actions: Vec<ExportSafetyBannerAction>,
    /// Degraded states this banner can name (required, matching the frozen matrix).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Credential subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never implies a raw secret is export-safe. MUST be `false`.
    pub implies_raw_secret_exportable: bool,
    /// Hard invariant: never leaves credential exclusion to implication. MUST be `false`.
    pub leaves_exclusion_to_implication: bool,
    /// Hard invariant: friendly "connected" wording never conceals the export boundary.
    /// MUST be `false`.
    pub uses_friendly_connected_wording: bool,
}

impl ExportSafetyBanner {
    /// Export-safety disclosures this banner must carry, derived from the export-safety class.
    pub fn export_safety_disclosure(&self) -> ExportSafetyDisclosure {
        resolve_export_safety_posture(self.export_safety_class)
    }

    /// Whether the banner offers every mandatory keyboard-complete default action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ExportSafetyBannerAction> =
            self.default_actions.iter().copied().collect();
        ExportSafetyBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }
}

// ---- review blocks ------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRevokeExportSafetyTrustReview {
    /// The event row names its credential class and its prior / new state.
    pub row_shows_credential_class_and_prior_new_state: bool,
    /// The impacted workflows a rotation / revoke affects are always shown.
    pub rotation_revoke_impacted_workflows_always_shown: bool,
    /// Running sessions, queued jobs, and remembered decisions stay distinct.
    pub running_sessions_queued_jobs_remembered_decisions_stay_distinct: bool,
    /// The recovery next step is always shown.
    pub recovery_next_step_always_shown: bool,
    /// A revoked or expired credential never reads as still usable.
    pub revoked_expired_never_reads_as_still_usable: bool,
    /// Audit and export actions are always present.
    pub audit_and_export_actions_present: bool,
    /// The banner states raw secrets are excluded by default from every export.
    pub banner_states_raw_secret_excluded_by_default: bool,
    /// Export exclusion is never left to implication.
    pub export_exclusion_never_left_to_implication: bool,
    /// Handle-class and source labels are preserved where allowed.
    pub handle_class_and_source_labels_preserved_where_allowed: bool,
    /// The export surface is named across every export surface.
    pub export_surface_named_across_all_surfaces: bool,
    /// The reveal posture is always shown.
    pub reveal_posture_always_shown: bool,
    /// Raw-secret handling is never normalized on any surface.
    pub raw_secret_handling_never_normalized: bool,
    /// No friendly "connected" wording conceals the lifecycle state or export boundary.
    pub no_friendly_connected_wording: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl RotationRevokeExportSafetyTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.row_shows_credential_class_and_prior_new_state
            && self.rotation_revoke_impacted_workflows_always_shown
            && self.running_sessions_queued_jobs_remembered_decisions_stay_distinct
            && self.recovery_next_step_always_shown
            && self.revoked_expired_never_reads_as_still_usable
            && self.audit_and_export_actions_present
            && self.banner_states_raw_secret_excluded_by_default
            && self.export_exclusion_never_left_to_implication
            && self.handle_class_and_source_labels_preserved_where_allowed
            && self.export_surface_named_across_all_surfaces
            && self.reveal_posture_always_shown
            && self.raw_secret_handling_never_normalized
            && self.no_friendly_connected_wording
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRevokeExportSafetyConsumerProjection {
    /// The event row shows its impacted workflows and recovery next step without docs.
    pub row_shows_impacted_workflows_and_recovery_without_docs: bool,
    /// A revoked / expired state is visible before any reuse.
    pub revoked_expired_state_visible_before_reuse: bool,
    /// The banner shows its exclusion posture inline.
    pub banner_shows_exclusion_posture_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl RotationRevokeExportSafetyConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.row_shows_impacted_workflows_and_recovery_without_docs
            && self.revoked_expired_state_visible_before_reuse
            && self.banner_shows_exclusion_posture_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRevokeExportSafetyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RotationRevokeExportSafetyControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationRevokeExportSafetyControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rotation/revoke-event rows.
    pub event_rows: Vec<RotationRevokeEventRow>,
    /// Export-safety banners.
    pub export_banners: Vec<ExportSafetyBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: RotationRevokeExportSafetyTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RotationRevokeExportSafetyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RotationRevokeExportSafetyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe rotation/revoke / export-safety controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRevokeExportSafetyControlsPacket {
    /// Record kind; must equal [`ROTATION_REVOKE_EXPORT_SAFETY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rotation/revoke-event rows.
    pub event_rows: Vec<RotationRevokeEventRow>,
    /// Export-safety banners.
    pub export_banners: Vec<ExportSafetyBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Trust review block.
    pub trust_review: RotationRevokeExportSafetyTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RotationRevokeExportSafetyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RotationRevokeExportSafetyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RotationRevokeExportSafetyControlsPacket {
    /// Builds a rotation/revoke / export-safety controls packet from stable-lane input.
    pub fn new(input: RotationRevokeExportSafetyControlsPacketInput) -> Self {
        Self {
            record_kind: ROTATION_REVOKE_EXPORT_SAFETY_RECORD_KIND.to_owned(),
            schema_version: ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            event_rows: input.event_rows,
            export_banners: input.export_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the rotation/revoke / export-safety control invariants.
    pub fn validate(&self) -> Vec<RotationRevokeExportSafetyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ROTATION_REVOKE_EXPORT_SAFETY_RECORD_KIND {
            violations.push(RotationRevokeExportSafetyViolation::WrongRecordKind);
        }
        if self.schema_version != ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_VERSION {
            violations.push(RotationRevokeExportSafetyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RotationRevokeExportSafetyViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_event_rows(self, &mut violations);
        validate_export_banners(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(RotationRevokeExportSafetyViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RotationRevokeExportSafetyViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RotationRevokeExportSafetyViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("rotation revoke export safety packet serializes"),
        ) {
            violations.push(RotationRevokeExportSafetyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("rotation revoke export safety packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("control,id,kind,scope_or_state,derived,secondary,flag\n");
        for row in &self.event_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "rotation_revoke_event_row",
                csv_field(&row.row_id),
                row.credential_class.as_str(),
                row.new_state.as_str(),
                row.continuity_disclosure().continuity_class.as_str(),
                row.prior_state.as_str(),
                row.continuity_disclosure().is_still_usable,
            ));
        }
        for banner in &self.export_banners {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "export_safety_banner",
                csv_field(&banner.banner_id),
                banner.export_safety_class.as_str(),
                banner.export_surface_class.as_str(),
                banner
                    .export_safety_disclosure()
                    .export_safety_posture
                    .as_str(),
                banner.reveal_posture.as_str(),
                banner
                    .export_safety_disclosure()
                    .preserves_handle_class_labels,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_usable = self
            .event_rows
            .iter()
            .filter(|row| !row.continuity_disclosure().is_still_usable)
            .count();
        let raw_excluded = self.export_banners.len();

        let mut out = String::new();
        out.push_str("# Rotation/revoke-event rows and export-safety banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Rotation/revoke-event rows: {} ({} are no longer usable)\n",
            self.event_rows.len(),
            not_usable
        ));
        out.push_str(&format!(
            "- Export-safety banners: {} (all {} exclude raw secrets by default)\n",
            self.export_banners.len(),
            raw_excluded
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Rotation/revoke-event rows\n\n");
        for row in &self.event_rows {
            out.push_str(&format!(
                "- **{}** ({}) — `{}` → `{}` → continuity `{}`\n",
                row.credential_id_label,
                row.credential_class.as_str(),
                row.prior_state.as_str(),
                row.new_state.as_str(),
                row.continuity_disclosure().continuity_class.as_str(),
            ));
        }

        out.push_str("\n## Export-safety banners\n\n");
        for banner in &self.export_banners {
            out.push_str(&format!(
                "- **{}** — class `{}` → posture `{}` (raw secrets excluded)\n",
                banner.export_surface_class.as_str(),
                banner.export_safety_class.as_str(),
                banner
                    .export_safety_disclosure()
                    .export_safety_posture
                    .as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in rotation/revoke / export-safety export.
#[derive(Debug)]
pub enum RotationRevokeExportSafetyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RotationRevokeExportSafetyViolation>),
}

impl fmt::Display for RotationRevokeExportSafetyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "rotation revoke export safety export parse failed: {error}"
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
                    "rotation revoke export safety export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RotationRevokeExportSafetyArtifactError {}

/// Validation failures emitted by [`RotationRevokeExportSafetyControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationRevokeExportSafetyViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No rotation/revoke-event rows are present.
    EventRowsMissing,
    /// A rotation/revoke-event row is incomplete.
    EventRowIncomplete,
    /// A rotation/revoke-event row carries the wrong frozen component class.
    EventRowWrongComponentClass,
    /// A rotation/revoke-event row does not name its credential identity.
    CredentialIdentityMissing,
    /// A rotation/revoke-event row misrepresents its derived continuity class.
    ContinuityMisrepresented,
    /// A rotation/revoke-event row does not name its prior or new state.
    PriorOrNewStateNoteMissing,
    /// A still-active row does not name its still-active state.
    StillActiveNoteMissing,
    /// An action-required row does not name its required action.
    ActionRequiredNoteMissing,
    /// A no-longer-usable row does not name its no-longer-usable state.
    NoLongerUsableNoteMissing,
    /// A superseded row does not name its superseded state.
    SupersededNoteMissing,
    /// A rotation/revoke-event row does not name any impacted workflow.
    ImpactedWorkflowsMissing,
    /// A rotation/revoke-event row does not carry its impacted-workflows note.
    ImpactedWorkflowsNoteMissing,
    /// A rotation/revoke-event row does not name its recovery next step.
    RecoveryNextStepNoteMissing,
    /// A rotation/revoke-event row does not name its audit event.
    AuditNoteMissing,
    /// A rotation/revoke-event row omits a mandatory recovery / export action.
    EventActionsIncomplete,
    /// The event rows do not cover every lifecycle state.
    LifecycleStateCoverageMissing,
    /// The event rows do not cover every continuity class.
    ContinuityClassCoverageMissing,
    /// The event rows do not cover every impacted-workflow class.
    ImpactedWorkflowCoverageMissing,
    /// An event row masks the workflows a rotation / revoke impacts.
    ImpactedWorkflowsMasked,
    /// No export-safety banners are present.
    ExportBannersMissing,
    /// An export-safety banner is incomplete.
    ExportBannerIncomplete,
    /// An export-safety banner carries the wrong frozen component class.
    ExportBannerWrongComponentClass,
    /// An export-safety banner does not name its export surface.
    ExportSurfaceNoteMissing,
    /// An export-safety banner misrepresents its derived export-safety posture.
    ExportSafetyMisrepresented,
    /// An export-safety banner does not state raw secrets are excluded by default.
    RawSecretExcludedNoteMissing,
    /// An export-safety banner does not name its preserved handle-class / source labels.
    HandleLabelNoteMissing,
    /// A redacted / masked banner does not name its redaction.
    RedactionNoteMissing,
    /// A fully-blocked banner does not name its blocked export.
    BlockedNoteMissing,
    /// An export-safety banner does not name its preserved labels.
    PreservedLabelsNoteMissing,
    /// An export-safety banner does not name its reveal posture.
    RevealPostureNoteMissing,
    /// An export-safety banner omits a mandatory policy / excluded-fields action.
    BannerActionsIncomplete,
    /// The banners do not cover every export-safety class.
    ExportSafetyClassCoverageMissing,
    /// The banners do not cover every export-safety posture.
    ExportSafetyPostureCoverageMissing,
    /// The banners do not cover every export surface.
    ExportSurfaceCoverageMissing,
    /// An export-safety banner leaves credential exclusion to implication.
    ExclusionLeftToImplication,
    /// A control does not declare its degraded states.
    DegradedStatesMissing,
    /// A control does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control implies a raw secret is export-safe (normalizes raw-secret handling).
    RawSecretHandlingNormalized,
    /// A control uses friendly "connected" wording that conceals a boundary.
    FriendlyConnectedWordingUsed,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RotationRevokeExportSafetyViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::EventRowsMissing => "event_rows_missing",
            Self::EventRowIncomplete => "event_row_incomplete",
            Self::EventRowWrongComponentClass => "event_row_wrong_component_class",
            Self::CredentialIdentityMissing => "credential_identity_missing",
            Self::ContinuityMisrepresented => "continuity_misrepresented",
            Self::PriorOrNewStateNoteMissing => "prior_or_new_state_note_missing",
            Self::StillActiveNoteMissing => "still_active_note_missing",
            Self::ActionRequiredNoteMissing => "action_required_note_missing",
            Self::NoLongerUsableNoteMissing => "no_longer_usable_note_missing",
            Self::SupersededNoteMissing => "superseded_note_missing",
            Self::ImpactedWorkflowsMissing => "impacted_workflows_missing",
            Self::ImpactedWorkflowsNoteMissing => "impacted_workflows_note_missing",
            Self::RecoveryNextStepNoteMissing => "recovery_next_step_note_missing",
            Self::AuditNoteMissing => "audit_note_missing",
            Self::EventActionsIncomplete => "event_actions_incomplete",
            Self::LifecycleStateCoverageMissing => "lifecycle_state_coverage_missing",
            Self::ContinuityClassCoverageMissing => "continuity_class_coverage_missing",
            Self::ImpactedWorkflowCoverageMissing => "impacted_workflow_coverage_missing",
            Self::ImpactedWorkflowsMasked => "impacted_workflows_masked",
            Self::ExportBannersMissing => "export_banners_missing",
            Self::ExportBannerIncomplete => "export_banner_incomplete",
            Self::ExportBannerWrongComponentClass => "export_banner_wrong_component_class",
            Self::ExportSurfaceNoteMissing => "export_surface_note_missing",
            Self::ExportSafetyMisrepresented => "export_safety_misrepresented",
            Self::RawSecretExcludedNoteMissing => "raw_secret_excluded_note_missing",
            Self::HandleLabelNoteMissing => "handle_label_note_missing",
            Self::RedactionNoteMissing => "redaction_note_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::PreservedLabelsNoteMissing => "preserved_labels_note_missing",
            Self::RevealPostureNoteMissing => "reveal_posture_note_missing",
            Self::BannerActionsIncomplete => "banner_actions_incomplete",
            Self::ExportSafetyClassCoverageMissing => "export_safety_class_coverage_missing",
            Self::ExportSafetyPostureCoverageMissing => "export_safety_posture_coverage_missing",
            Self::ExportSurfaceCoverageMissing => "export_surface_coverage_missing",
            Self::ExclusionLeftToImplication => "exclusion_left_to_implication",
            Self::DegradedStatesMissing => "degraded_states_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::RawSecretHandlingNormalized => "raw_secret_handling_normalized",
            Self::FriendlyConnectedWordingUsed => "friendly_connected_wording_used",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable rotation/revoke / export-safety export.
pub fn current_rotation_revoke_export_safety_export(
) -> Result<RotationRevokeExportSafetyControlsPacket, RotationRevokeExportSafetyArtifactError> {
    let packet: RotationRevokeExportSafetyControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-rotation-revoke-export-safety-proof/support_export.json"
        )))
        .map_err(RotationRevokeExportSafetyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RotationRevokeExportSafetyArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &RotationRevokeExportSafetyControlsPacket,
    violations: &mut Vec<RotationRevokeExportSafetyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF,
        ROTATION_REVOKE_EXPORT_SAFETY_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
        M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RotationRevokeExportSafetyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_event_rows(
    packet: &RotationRevokeExportSafetyControlsPacket,
    violations: &mut Vec<RotationRevokeExportSafetyViolation>,
) {
    if packet.event_rows.is_empty() {
        violations.push(RotationRevokeExportSafetyViolation::EventRowsMissing);
        return;
    }

    let mut lifecycle_states: BTreeSet<M5CredentialLifecycleState> = BTreeSet::new();
    let mut continuity_classes: BTreeSet<CredentialContinuityClass> = BTreeSet::new();
    let mut impacted_classes: BTreeSet<ImpactedWorkflowClass> = BTreeSet::new();

    for row in &packet.event_rows {
        let disclosure = row.continuity_disclosure();
        lifecycle_states.insert(row.new_state);
        continuity_classes.insert(disclosure.continuity_class);
        for impacted in &row.impacted_workflows {
            impacted_classes.insert(*impacted);
        }

        if row.row_id.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(RotationRevokeExportSafetyViolation::EventRowIncomplete);
        }
        if row.component != M5CredentialComponentFamily::RotationRevokeEventRow {
            violations.push(RotationRevokeExportSafetyViolation::EventRowWrongComponentClass);
        }
        if row.credential_id_label.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::CredentialIdentityMissing);
        }
        if row.continuity_class != disclosure.continuity_class
            || row.claims_still_usable != disclosure.is_still_usable
        {
            violations.push(RotationRevokeExportSafetyViolation::ContinuityMisrepresented);
        }
        if row.prior_state_note.trim().is_empty() || row.new_state_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::PriorOrNewStateNoteMissing);
        }
        if disclosure.needs_still_active_note && row.still_active_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::StillActiveNoteMissing);
        }
        if disclosure.needs_action_required_note && row.action_required_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::ActionRequiredNoteMissing);
        }
        if disclosure.needs_no_longer_usable_note && row.no_longer_usable_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::NoLongerUsableNoteMissing);
        }
        if disclosure.needs_superseded_note && row.superseded_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::SupersededNoteMissing);
        }
        if row.impacted_workflows.is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::ImpactedWorkflowsMissing);
        }
        if row.impacted_workflows_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::ImpactedWorkflowsNoteMissing);
        }
        if row.recovery_next_step_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::RecoveryNextStepNoteMissing);
        }
        if row.audit_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::AuditNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(RotationRevokeExportSafetyViolation::EventActionsIncomplete);
        }
        if row.degraded_states.is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::DegradedStatesMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(RotationRevokeExportSafetyViolation::RequiredLabelsIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(RotationRevokeExportSafetyViolation::AccessibilityRouteMissing);
        }
        if row.masks_impacted_workflows {
            violations.push(RotationRevokeExportSafetyViolation::ImpactedWorkflowsMasked);
        }
        if row.implies_raw_secret_exportable {
            violations.push(RotationRevokeExportSafetyViolation::RawSecretHandlingNormalized);
        }
        if row.uses_friendly_connected_wording {
            violations.push(RotationRevokeExportSafetyViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in M5CredentialLifecycleState::ALL {
        if !lifecycle_states.contains(&required) {
            violations.push(RotationRevokeExportSafetyViolation::LifecycleStateCoverageMissing);
            break;
        }
    }
    for required in CredentialContinuityClass::ALL {
        if !continuity_classes.contains(&required) {
            violations.push(RotationRevokeExportSafetyViolation::ContinuityClassCoverageMissing);
            break;
        }
    }
    for required in ImpactedWorkflowClass::ALL {
        if !impacted_classes.contains(&required) {
            violations.push(RotationRevokeExportSafetyViolation::ImpactedWorkflowCoverageMissing);
            break;
        }
    }
}

fn validate_export_banners(
    packet: &RotationRevokeExportSafetyControlsPacket,
    violations: &mut Vec<RotationRevokeExportSafetyViolation>,
) {
    if packet.export_banners.is_empty() {
        violations.push(RotationRevokeExportSafetyViolation::ExportBannersMissing);
        return;
    }

    let mut safety_classes: BTreeSet<M5CredentialExportSafetyClass> = BTreeSet::new();
    let mut postures: BTreeSet<ExportSafetyPosture> = BTreeSet::new();
    let mut surfaces: BTreeSet<ExportSurfaceClass> = BTreeSet::new();

    for banner in &packet.export_banners {
        let disclosure = banner.export_safety_disclosure();
        safety_classes.insert(banner.export_safety_class);
        postures.insert(disclosure.export_safety_posture);
        surfaces.insert(banner.export_surface_class);

        if banner.banner_id.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(RotationRevokeExportSafetyViolation::ExportBannerIncomplete);
        }
        if banner.component != M5CredentialComponentFamily::ExportSafetyBanner {
            violations.push(RotationRevokeExportSafetyViolation::ExportBannerWrongComponentClass);
        }
        if banner.export_surface_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::ExportSurfaceNoteMissing);
        }
        if banner.export_safety_posture != disclosure.export_safety_posture
            || banner.claims_preserves_handle_labels != disclosure.preserves_handle_class_labels
        {
            violations.push(RotationRevokeExportSafetyViolation::ExportSafetyMisrepresented);
        }
        if banner.raw_secret_excluded_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::RawSecretExcludedNoteMissing);
        }
        if disclosure.needs_handle_label_note && banner.handle_label_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::HandleLabelNoteMissing);
        }
        if disclosure.needs_redaction_note && banner.redaction_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::RedactionNoteMissing);
        }
        if disclosure.needs_blocked_note && banner.blocked_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::BlockedNoteMissing);
        }
        if banner.preserved_labels_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::PreservedLabelsNoteMissing);
        }
        if banner.reveal_posture_note.trim().is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::RevealPostureNoteMissing);
        }
        if !banner.declares_mandatory_actions() {
            violations.push(RotationRevokeExportSafetyViolation::BannerActionsIncomplete);
        }
        if banner.degraded_states.is_empty() {
            violations.push(RotationRevokeExportSafetyViolation::DegradedStatesMissing);
        }
        if !banner.declares_mandatory_labels() {
            violations.push(RotationRevokeExportSafetyViolation::RequiredLabelsIncomplete);
        }
        if banner.accessibility_routes.is_empty()
            || !banner
                .accessibility_routes
                .contains(&M5CredentialAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(RotationRevokeExportSafetyViolation::AccessibilityRouteMissing);
        }
        if banner.implies_raw_secret_exportable {
            violations.push(RotationRevokeExportSafetyViolation::RawSecretHandlingNormalized);
        }
        if banner.leaves_exclusion_to_implication {
            violations.push(RotationRevokeExportSafetyViolation::ExclusionLeftToImplication);
        }
        if banner.uses_friendly_connected_wording {
            violations.push(RotationRevokeExportSafetyViolation::FriendlyConnectedWordingUsed);
        }
    }

    for required in M5CredentialExportSafetyClass::ALL {
        if !safety_classes.contains(&required) {
            violations.push(RotationRevokeExportSafetyViolation::ExportSafetyClassCoverageMissing);
            break;
        }
    }
    for required in ExportSafetyPosture::ALL {
        if !postures.contains(&required) {
            violations
                .push(RotationRevokeExportSafetyViolation::ExportSafetyPostureCoverageMissing);
            break;
        }
    }
    for required in ExportSurfaceClass::ALL {
        if !surfaces.contains(&required) {
            violations.push(RotationRevokeExportSafetyViolation::ExportSurfaceCoverageMissing);
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
///
/// The credential vocabulary uses the words "secret", "credential", and "api_key"
/// pervasively as governed tokens, so this check flags only raw-*value* shapes: a
/// password / passphrase literal, a bearer literal, a URL scheme, or a PEM header.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
