//! Support-export consumer for the canonical state-class recovery packet.
//!
//! This module folds the checked-in recovery packet into a metadata-safe
//! support-export envelope so support and repair flows can quote the same
//! failed class, chosen recovery route, placeholder continuity plan, intact
//! state summary, and safest-route rationale that the state packet freezes.

use std::fmt;

use aureline_reactive_state::{
    seeded_state_class_recovery_packet, validate_state_class_recovery_packet,
    StateClassRecoveryAuthorityClass, StateClassRecoveryBlockedCapabilityClass,
    StateClassRecoveryPacket, StateClassRecoveryPlaceholderActionClass,
    StateClassRecoveryPlaceholderKind, StateClassRecoveryPreservedContextClass,
    StateClassRecoveryRoute, StateClassRecoveryStateClass, StateClassRecoveryValidationReport,
    STATE_CLASS_RECOVERY_DOC_REF, STATE_CLASS_RECOVERY_REPORT_REF, STATE_CLASS_RECOVERY_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "state_class_recovery_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "state_class_recovery_support_export_envelope";

/// One support-export row copied from the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateClassRecoverySupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Stable family id.
    pub family_id: String,
    /// State class that failed or degraded.
    pub state_class: StateClassRecoveryStateClass,
    /// Authority owner class.
    pub authority_class: StateClassRecoveryAuthorityClass,
    /// Primary recovery route selected for the family.
    pub primary_recovery_route: StateClassRecoveryRoute,
    /// Fallback routes that remain available.
    pub fallback_recovery_routes: Vec<StateClassRecoveryRoute>,
    /// Placeholder continuity posture that preserves surrounding context.
    pub placeholder_kind: StateClassRecoveryPlaceholderKind,
    /// Context that remained intact.
    pub preserved_context: Vec<StateClassRecoveryPreservedContextClass>,
    /// Capabilities still blocked while the placeholder is active.
    pub blocked_capabilities: Vec<StateClassRecoveryBlockedCapabilityClass>,
    /// Safe placeholder actions.
    pub actions: Vec<StateClassRecoveryPlaceholderActionClass>,
    /// Support-safe summary of what remained intact.
    pub intact_state_summary: String,
    /// Support-safe summary of why the chosen route is safest.
    pub safest_route_rationale: String,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
}

impl StateClassRecoverySupportExportRow {
    fn from_family(row: &aureline_reactive_state::StateClassRecoveryFamilyRow) -> Self {
        Self {
            record_kind: STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            family_id: row.family_id.clone(),
            state_class: row.state_class,
            authority_class: row.authority_class,
            primary_recovery_route: row.primary_recovery_route,
            fallback_recovery_routes: row.fallback_recovery_routes.clone(),
            placeholder_kind: row.placeholder_plan.placeholder_kind,
            preserved_context: row.placeholder_plan.preserved_context.clone(),
            blocked_capabilities: row.placeholder_plan.blocked_capabilities.clone(),
            actions: row.placeholder_plan.actions.clone(),
            intact_state_summary: row.intact_state_summary.clone(),
            safest_route_rationale: row.safest_route_rationale.clone(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    /// Returns true when the row remains metadata-safe and support-usable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && !self.fallback_recovery_routes.is_empty()
            && !self.preserved_context.is_empty()
            && !self.actions.is_empty()
            && !self.intact_state_summary.trim().is_empty()
            && !self.safest_route_rationale.trim().is_empty()
    }
}

/// Metadata-safe support-export envelope for state-class recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateClassRecoverySupportExportEnvelope {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Stable envelope id.
    pub envelope_id: String,
    /// Capture time supplied by the caller.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Reviewer report ref.
    pub report_ref: String,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
    /// Export rows.
    pub rows: Vec<StateClassRecoverySupportExportRow>,
}

impl StateClassRecoverySupportExportEnvelope {
    /// Builds an envelope from a validated packet.
    pub fn from_packet(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        packet: &StateClassRecoveryPacket,
    ) -> Self {
        let mut rows: Vec<_> = packet
            .families
            .iter()
            .map(StateClassRecoverySupportExportRow::from_family)
            .collect();
        rows.sort_by(|a, b| a.family_id.cmp(&b.family_id));
        Self {
            record_kind: STATE_CLASS_RECOVERY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: STATE_CLASS_RECOVERY_DOC_REF.to_owned(),
            schema_ref: STATE_CLASS_RECOVERY_SCHEMA_REF.to_owned(),
            report_ref: STATE_CLASS_RECOVERY_REPORT_REF.to_owned(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
            rows,
        }
    }

    /// Returns true when the envelope remains metadata-safe and in sync with
    /// the canonical packet refs.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == STATE_CLASS_RECOVERY_DOC_REF
            && self.schema_ref == STATE_CLASS_RECOVERY_SCHEMA_REF
            && self.report_ref == STATE_CLASS_RECOVERY_REPORT_REF
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(StateClassRecoverySupportExportRow::is_export_safe)
    }
}

/// Error returned when the support envelope cannot be compiled.
#[derive(Debug)]
pub enum StateClassRecoverySupportExportError {
    /// The canonical packet failed validation.
    PacketValidation(StateClassRecoveryValidationReport),
}

impl fmt::Display for StateClassRecoverySupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "state-class recovery packet invalid: {report}")
            }
        }
    }
}

impl std::error::Error for StateClassRecoverySupportExportError {}

impl From<StateClassRecoveryValidationReport> for StateClassRecoverySupportExportError {
    fn from(report: StateClassRecoveryValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// state-class recovery packet.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<StateClassRecoverySupportExportEnvelope, StateClassRecoverySupportExportError> {
    let packet = seeded_state_class_recovery_packet();
    validate_state_class_recovery_packet(&packet)?;
    Ok(StateClassRecoverySupportExportEnvelope::from_packet(
        envelope_id,
        captured_at,
        &packet,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_envelope_round_trip() {
        let envelope = compile_support_export_envelope(
            "envelope:state_class_recovery:test",
            "2026-06-13T08:30:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 7);

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: StateClassRecoverySupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn trust_policy_row_stays_fail_closed_without_widening_authority() {
        let envelope = compile_support_export_envelope(
            "envelope:state_class_recovery:trust_policy",
            "2026-06-13T08:35:00Z",
        )
        .expect("envelope compiles");
        let row = envelope
            .rows
            .iter()
            .find(|row| row.family_id == "trust_policy")
            .expect("trust policy row exists");
        assert_eq!(
            row.primary_recovery_route,
            StateClassRecoveryRoute::FailClosedPrivilegedOperations
        );
        assert!(row
            .blocked_capabilities
            .contains(&StateClassRecoveryBlockedCapabilityClass::PrivilegedApply));
        assert!(row
            .actions
            .contains(&StateClassRecoveryPlaceholderActionClass::ReauthenticateManagedSurface));
    }
}
