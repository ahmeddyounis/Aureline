//! Suppression, dedupe, fanout, and escalation audit records.
//!
//! The router emits surface routes; this module turns those route outcomes
//! into durable audit entries that explain why a notification was shown,
//! held, suppressed, deduped, muted, snoozed, or escalated through quiet
//! hours. The audit is intentionally metadata-only and keeps raw payload
//! material out of support and proof artifacts.

use serde::{Deserialize, Serialize};

use super::envelope::{
    ClientScope, DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope,
    QuietHoursMode, SeverityClass, SourceSubsystem, SuppressionReason,
};
use super::router::{RoutedNotification, SurfaceRoute};

/// Schema version for notification suppression audit reports.
pub const NOTIFICATION_SUPPRESSION_AUDIT_SCHEMA_VERSION: u32 = 1;
/// Stable record kind for audit reports.
pub const NOTIFICATION_SUPPRESSION_AUDIT_REPORT_RECORD_KIND: &str =
    "notification_suppression_audit_report";
/// Stable record kind for one audit entry.
pub const NOTIFICATION_SUPPRESSION_AUDIT_ENTRY_RECORD_KIND: &str =
    "notification_suppression_audit_entry";

/// Explanation class for one routed notification surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationSuppressionExplanationClass {
    /// Surface delivered normally.
    Shown,
    /// Surface delivered because a critical-safety event bypassed hold modes.
    EscalatedCriticalSafety,
    /// Surface delivery was held by quiet hours or do-not-disturb.
    HeldQuietHours,
    /// Surface delivery was suppressed by policy.
    SuppressedPolicy,
    /// Surface delivery coalesced into an existing canonical event.
    DedupedCanonicalEvent,
    /// Surface delivery coalesced into an existing grouped burst.
    DedupedGroupedBurst,
    /// Delivery was muted by user class/source choice.
    MutedByUser,
    /// Delivery was snoozed by user choice.
    SnoozedByUser,
    /// Surface had no route.
    NoRoute,
    /// Surface route records a stale or failed external fanout.
    StaleOrFailedFanout,
}

impl NotificationSuppressionExplanationClass {
    /// Stable token recorded in audit fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shown => "shown",
            Self::EscalatedCriticalSafety => "escalated_critical_safety",
            Self::HeldQuietHours => "held_quiet_hours",
            Self::SuppressedPolicy => "suppressed_policy",
            Self::DedupedCanonicalEvent => "deduped_canonical_event",
            Self::DedupedGroupedBurst => "deduped_grouped_burst",
            Self::MutedByUser => "muted_by_user",
            Self::SnoozedByUser => "snoozed_by_user",
            Self::NoRoute => "no_route",
            Self::StaleOrFailedFanout => "stale_or_failed_fanout",
        }
    }
}

/// One audit row explaining a surface-route outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSuppressionAuditEntry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable audit entry id.
    pub audit_entry_id: String,
    /// Source notification envelope id.
    pub notification_envelope_id: String,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Event lineage id.
    pub event_lineage_id_ref: String,
    /// Canonical object target ref.
    pub canonical_object_target_ref: String,
    /// Source subsystem.
    pub source_subsystem: SourceSubsystem,
    /// Surface being explained.
    pub intended_delivery_surface_class: FanoutSurfaceClass,
    /// Destination client scope.
    pub client_scope: ClientScope,
    /// Fanout receipt outcome.
    pub receipt_state: FanoutReceiptState,
    /// Explanation class.
    pub explanation_class: NotificationSuppressionExplanationClass,
    /// Suppression or dedupe reasons.
    pub suppression_reasons: Vec<SuppressionReason>,
    /// Quiet-hours modes active at the decision.
    pub active_quiet_hours_modes_at_decision: Vec<QuietHoursMode>,
    /// Dedupe scheme.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Reopen target ref.
    pub reopen_target_ref: String,
    /// Exact target identity when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_target_identity_ref: Option<String>,
    /// True when the route resolves to a durable in-product truth surface.
    pub durable_truth_preserved: bool,
    /// Privacy-safe summary of the decision.
    pub decision_summary_label: String,
    /// Decision time.
    pub decided_at: String,
}

impl NotificationSuppressionAuditEntry {
    /// Builds one audit entry from an envelope, routed notification, and
    /// surface route.
    pub fn from_route(
        envelope: &NotificationEnvelope,
        routed: &RoutedNotification,
        route: &SurfaceRoute,
    ) -> Self {
        let explanation_class = explanation_class_for(envelope, route);
        Self {
            record_kind: NOTIFICATION_SUPPRESSION_AUDIT_ENTRY_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_SUPPRESSION_AUDIT_SCHEMA_VERSION,
            audit_entry_id: format!(
                "audit:notification:{}:{}",
                routed.notification_envelope_id,
                route.fanout_surface_class.as_str()
            ),
            notification_envelope_id: routed.notification_envelope_id.clone(),
            canonical_event_id: routed.canonical_event_id.clone(),
            event_lineage_id_ref: routed.event_lineage_id_ref.clone(),
            canonical_object_target_ref: envelope.canonical_object_target_ref.clone(),
            source_subsystem: routed.source_subsystem,
            intended_delivery_surface_class: route.fanout_surface_class,
            client_scope: route.client_scope,
            receipt_state: route.receipt_state,
            explanation_class,
            suppression_reasons: route.suppression_reasons.clone(),
            active_quiet_hours_modes_at_decision: envelope
                .suppression_state
                .active_modes_at_mint
                .clone(),
            dedupe_key_scheme: route.dedupe_key_scheme,
            reopen_target_ref: route.reopen_target_ref.clone(),
            exact_target_identity_ref: routed.reopen_target.exact_target_identity_ref.clone(),
            durable_truth_preserved: routed_has_durable_truth(routed),
            decision_summary_label: decision_summary_label(explanation_class),
            decided_at: route.minted_at.clone(),
        }
    }
}

/// Durable report of notification delivery decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSuppressionAuditReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Generation time.
    pub generated_at: String,
    /// Audit entries.
    pub entries: Vec<NotificationSuppressionAuditEntry>,
    /// True when the report includes delivered, muted/snoozed/suppressed,
    /// deduped, and escalation explanation classes where present.
    pub reconstructs_delivery_decisions: bool,
    /// True when raw message bodies, paths, URLs, prompts, and secrets are
    /// excluded from the report.
    pub raw_private_material_excluded: bool,
}

impl NotificationSuppressionAuditReport {
    /// Builds a report from routed notification pairs.
    pub fn from_routed_pairs<'a>(
        report_id: impl Into<String>,
        generated_at: impl Into<String>,
        pairs: impl IntoIterator<Item = (&'a NotificationEnvelope, &'a RoutedNotification)>,
    ) -> Self {
        let mut entries = Vec::new();
        for (envelope, routed) in pairs {
            for route in &routed.surface_routes {
                entries.push(NotificationSuppressionAuditEntry::from_route(
                    envelope, routed, route,
                ));
            }
        }
        Self {
            record_kind: NOTIFICATION_SUPPRESSION_AUDIT_REPORT_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_SUPPRESSION_AUDIT_SCHEMA_VERSION,
            report_id: report_id.into(),
            generated_at: generated_at.into(),
            entries,
            reconstructs_delivery_decisions: true,
            raw_private_material_excluded: true,
        }
    }
}

fn explanation_class_for(
    envelope: &NotificationEnvelope,
    route: &SurfaceRoute,
) -> NotificationSuppressionExplanationClass {
    if route
        .suppression_reasons
        .contains(&SuppressionReason::ClassMutedByUser)
    {
        return NotificationSuppressionExplanationClass::MutedByUser;
    }
    if route
        .suppression_reasons
        .contains(&SuppressionReason::ClassSnoozedByUser)
    {
        return NotificationSuppressionExplanationClass::SnoozedByUser;
    }
    match route.receipt_state {
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold => {
            if matches!(envelope.severity_class, SeverityClass::Critical)
                && envelope.suppression_state.has_active_quiet_mode()
            {
                NotificationSuppressionExplanationClass::EscalatedCriticalSafety
            } else {
                NotificationSuppressionExplanationClass::Shown
            }
        }
        FanoutReceiptState::HeldQuietHours => {
            NotificationSuppressionExplanationClass::HeldQuietHours
        }
        FanoutReceiptState::SuppressedPolicy => {
            NotificationSuppressionExplanationClass::SuppressedPolicy
        }
        FanoutReceiptState::DedupedCanonicalEvent => {
            NotificationSuppressionExplanationClass::DedupedCanonicalEvent
        }
        FanoutReceiptState::DedupedGroupedBurst => {
            NotificationSuppressionExplanationClass::DedupedGroupedBurst
        }
        FanoutReceiptState::NotAttemptedNoRoute => NotificationSuppressionExplanationClass::NoRoute,
    }
}

fn routed_has_durable_truth(routed: &RoutedNotification) -> bool {
    routed.surface_routes.iter().any(|route| {
        matches!(
            route.fanout_surface_class,
            FanoutSurfaceClass::DurableJobRow
                | FanoutSurfaceClass::ActivityCenterDigestCard
                | FanoutSurfaceClass::DigestGroupRow
        )
    })
}

fn decision_summary_label(class: NotificationSuppressionExplanationClass) -> String {
    match class {
        NotificationSuppressionExplanationClass::Shown => "Shown on the requested surface.",
        NotificationSuppressionExplanationClass::EscalatedCriticalSafety => {
            "Critical notification bypassed active hold modes."
        }
        NotificationSuppressionExplanationClass::HeldQuietHours => {
            "Held by quiet-hours policy; durable truth preserved."
        }
        NotificationSuppressionExplanationClass::SuppressedPolicy => {
            "Suppressed by policy; durable truth preserved."
        }
        NotificationSuppressionExplanationClass::DedupedCanonicalEvent => {
            "Coalesced into the existing canonical event."
        }
        NotificationSuppressionExplanationClass::DedupedGroupedBurst => {
            "Coalesced into the existing grouped burst."
        }
        NotificationSuppressionExplanationClass::MutedByUser => {
            "Muted by user class or source choice."
        }
        NotificationSuppressionExplanationClass::SnoozedByUser => {
            "Snoozed until the recorded resume condition."
        }
        NotificationSuppressionExplanationClass::NoRoute => {
            "No route was available for the requested surface."
        }
        NotificationSuppressionExplanationClass::StaleOrFailedFanout => {
            "External fanout was stale or failed and remains durable truth."
        }
    }
    .to_owned()
}
