//! Support-export consumer for the cross-surface subscription contract.
//!
//! This module folds the live subscription inspector — built by replaying
//! the seeded contract in [`aureline_reactive_state::subscriptions`]
//! through its bus — into a metadata-safe support-export envelope. Support,
//! release, and procurement readers can round-trip the same stable
//! subscription fields the shell inspector renders: which authority
//! published each view, which scope and epoch it belongs to, the narrowed
//! truth claim, and which surfaces subscribe — without embedding raw
//! payloads, private material, or ambient authority.

use std::fmt;

use aureline_reactive_state::{
    seeded_cross_surface_subscription_fixtures, seeded_cross_surface_subscription_packet,
    validate_cross_surface_subscription_packet, ConsumerSurface, CrossSurfaceSubscriptionBus,
    CrossSurfaceSubscriptionValidationReport, StableSubscriptionFields, SubscriptionError,
    CROSS_SURFACE_SUBSCRIPTION_DOC_REF, CROSS_SURFACE_SUBSCRIPTION_PROOF_REF,
    CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const M5_SUBSCRIPTION_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "cross_surface_subscription_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const M5_SUBSCRIPTION_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "cross_surface_subscription_support_export_envelope";

/// One support-export row carrying the stable subscription fields of the
/// latest published frame for a binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SubscriptionSupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// The stable subscription fields, round-tripped verbatim.
    pub subscription: StableSubscriptionFields,
    /// Consumer surfaces subscribed to the binding, in stable order.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Metadata-safe export invariant: no raw producer payload bodies.
    pub raw_payload_excluded: bool,
    /// Metadata-safe export invariant: no ambient (unscoped) authority.
    pub ambient_authority_excluded: bool,
}

impl M5SubscriptionSupportExportRow {
    /// Whether the row preserves the metadata-safe export invariants. A
    /// scoped subscription with a concrete scope id is the proof that no
    /// ambient authority leaked into the export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && !self.subscription.scope_id.trim().is_empty()
    }
}

/// Metadata-safe support-export envelope for the subscription contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SubscriptionSupportExport {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Source contract packet id.
    pub packet_id: String,
    /// Source contract schema version.
    pub schema_version: u32,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof report ref.
    pub proof_ref: String,
    /// One row per active `(binding, scope)` pair, in stable order.
    pub rows: Vec<M5SubscriptionSupportExportRow>,
    /// Invariant summary copied from the contract.
    pub invariants: Vec<String>,
}

impl M5SubscriptionSupportExport {
    /// Whether every row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.rows
            .iter()
            .all(M5SubscriptionSupportExportRow::is_export_safe)
    }
}

/// Error returned when the support export cannot be compiled.
#[derive(Debug)]
pub enum M5SubscriptionSupportExportError {
    /// The canonical contract failed validation.
    PacketValidation(CrossSurfaceSubscriptionValidationReport),
    /// A frame could not be published through the bus.
    Publish(SubscriptionError),
}

impl fmt::Display for M5SubscriptionSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "cross-surface subscription invalid: {report}")
            }
            Self::Publish(err) => write!(f, "subscription publish failed: {err}"),
        }
    }
}

impl std::error::Error for M5SubscriptionSupportExportError {}

impl From<CrossSurfaceSubscriptionValidationReport> for M5SubscriptionSupportExportError {
    fn from(report: CrossSurfaceSubscriptionValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

impl From<SubscriptionError> for M5SubscriptionSupportExportError {
    fn from(err: SubscriptionError) -> Self {
        Self::Publish(err)
    }
}

/// Compiles the metadata-safe support-export envelope by replaying the
/// seeded contract through its bus and folding the inspector report.
///
/// # Errors
///
/// Returns [`M5SubscriptionSupportExportError`] when the contract fails
/// validation or a frame cannot be published.
pub fn compile_subscription_support_export(
) -> Result<M5SubscriptionSupportExport, M5SubscriptionSupportExportError> {
    let packet = seeded_cross_surface_subscription_packet();
    validate_cross_surface_subscription_packet(&packet)?;

    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    for fixture in seeded_cross_surface_subscription_fixtures() {
        bus.publish(&fixture.binding_id, &fixture.frame)?;
    }
    let report = bus.inspector_report();

    let rows = report
        .rows
        .into_iter()
        .map(|row| M5SubscriptionSupportExportRow {
            record_kind: M5_SUBSCRIPTION_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            subscription: row.subscription,
            consumer_surfaces: row.consumer_surfaces,
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect();

    Ok(M5SubscriptionSupportExport {
        record_kind: M5_SUBSCRIPTION_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
        packet_id: packet.packet_id,
        schema_version: packet.schema_version,
        doc_ref: CROSS_SURFACE_SUBSCRIPTION_DOC_REF.to_owned(),
        schema_ref: CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF.to_owned(),
        proof_ref: CROSS_SURFACE_SUBSCRIPTION_PROOF_REF.to_owned(),
        rows,
        invariants: packet.invariants,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_is_export_safe_and_complete() {
        let envelope = compile_subscription_support_export().expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 8);
        assert_eq!(
            envelope.record_kind,
            M5_SUBSCRIPTION_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
        );
    }

    #[test]
    fn stable_subscription_fields_round_trip_through_serde() {
        let envelope = compile_subscription_support_export().expect("envelope compiles");
        let json = serde_json::to_string(&envelope).expect("serializes");
        let decoded: M5SubscriptionSupportExport =
            serde_json::from_str(&json).expect("deserializes");
        assert_eq!(envelope, decoded);
        // The export names authorities and claims, not raw payloads.
        assert!(json.contains("\"authority_class\":\"provider_overlay\""));
        assert!(json.contains("\"truth_claim\":\"provider_unavailable\""));
        assert!(json.contains("\"raw_payload_excluded\":true"));
    }

    #[test]
    fn every_exported_row_is_scoped() {
        let envelope = compile_subscription_support_export().expect("envelope compiles");
        for row in &envelope.rows {
            assert!(
                !row.subscription.scope_id.trim().is_empty(),
                "binding {} leaked an ambient scope into the export",
                row.subscription.binding_id
            );
        }
    }
}
