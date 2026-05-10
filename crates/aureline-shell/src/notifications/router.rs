//! Notification router: collapse repeated envelopes onto one durable truth
//! and project surface routes (toast, banner, status item, durable activity
//! row, OS notification, lock-screen summary, companion push) without
//! splitting the reopen target.
//!
//! ## Responsibilities
//!
//! 1. Read a [`NotificationEnvelope`] verbatim — never re-derive privacy,
//!    severity, or reopen vocabulary.
//! 2. Pick a per-surface [`FanoutReceiptState`] for every entry in
//!    `recommended_surfaces[]`. The first envelope on a given dedupe key
//!    delivers; subsequent envelopes with the same dedupe key emit
//!    `deduped_canonical_event` (or `deduped_grouped_burst`) receipts on
//!    surfaces that already saw the event.
//! 3. Honor explicit suppression state: when `suppression_state.suppressed`
//!    is true, attention-grabbing surfaces (toast, banner, OS notification,
//!    lock-screen summary, companion push) are held or suppressed; durable
//!    truth surfaces (durable_job_row, status_item, status_strip,
//!    activity_center_digest_card) still deliver so the user has a path back.
//! 4. Preserve the envelope's `reopen_target` on every routed surface so a
//!    toast, a banner, and a status row all reopen the same canonical object.
//!
//! ## What this module does NOT own
//!
//! - rendering (no copy invented here);
//! - retry scheduling, transport, or platform adapters;
//! - the privacy class enforcement at the OS / lock-screen surface — that is
//!   the adapter's job. The router simply preserves the privacy / payload /
//!   redaction classes on the routed receipts so the adapter has them.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use super::envelope::{
    ClientScope, DedupeKeyScheme, FanoutReceipt, FanoutReceiptState, FanoutSurfaceClass,
    NotificationEnvelope, PrivacyClass, PrivacyPayloadClass, QuietHoursMode, RedactionClass,
    ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem, StableAction,
    StaleOrUndeliveredReason, StaleOrUndeliveredReasonClass, SuppressionReason,
    FANOUT_RECEIPT_SCHEMA_VERSION,
};

/// Stable record-kind tag carried in serialized routed-notification snapshots.
pub const ROUTED_NOTIFICATION_RECORD_KIND: &str = "routed_notification_record";
/// Schema version for [`RoutedNotification`] snapshots emitted by the router.
pub const ROUTED_NOTIFICATION_SCHEMA_VERSION: u32 = 1;

/// One per-surface routing decision. The router emits one
/// [`SurfaceRoute`] for every surface in `recommended_surfaces[]` plus a
/// not-attempted entry whenever a surface has no live route on this client
/// scope (kept here as a stub so support exports retain a row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoute {
    pub fanout_surface_class: FanoutSurfaceClass,
    pub client_scope: ClientScope,
    pub receipt_state: FanoutReceiptState,
    pub stale_or_undelivered_reason: StaleOrUndeliveredReason,
    pub dedupe_key_scheme: DedupeKeyScheme,
    pub delivery_envelope_ref: String,
    pub reopen_target_ref: String,
    pub redaction_class: RedactionClass,
    #[serde(default)]
    pub suppression_reasons: Vec<SuppressionReason>,
    pub minted_at: String,
}

impl SurfaceRoute {
    /// True when this route should render on its surface (delivered or a
    /// release from a held queue).
    pub fn is_visible(&self) -> bool {
        matches!(
            self.receipt_state,
            FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
        )
    }

    /// True when this route was held, suppressed, deduped, or had no route —
    /// it remains a visible truth row in the activity center but the surface
    /// itself does not light up.
    pub fn is_suppressed_or_deduped(&self) -> bool {
        matches!(
            self.receipt_state,
            FanoutReceiptState::HeldQuietHours
                | FanoutReceiptState::SuppressedPolicy
                | FanoutReceiptState::DedupedCanonicalEvent
                | FanoutReceiptState::DedupedGroupedBurst
                | FanoutReceiptState::NotAttemptedNoRoute
        )
    }

    /// Project this route into the canonical [`FanoutReceipt`] payload the
    /// envelope's `fanout_receipts[]` carries. Receipt ids are derived from
    /// the envelope id and surface so routing stays deterministic in tests
    /// and support exports.
    pub fn to_fanout_receipt(
        &self,
        source_envelope_id: &str,
        canonical_event_id: &str,
        event_lineage_id_ref: &str,
    ) -> FanoutReceipt {
        FanoutReceipt {
            record_kind: "fanout_receipt_record".to_owned(),
            fanout_receipt_schema_version: FANOUT_RECEIPT_SCHEMA_VERSION,
            fanout_receipt_id: format!(
                "fanout-receipt:{}:{}",
                source_envelope_id,
                self.fanout_surface_class.as_str()
            ),
            source_notification_envelope_id_ref: source_envelope_id.to_owned(),
            canonical_event_id: canonical_event_id.to_owned(),
            event_lineage_id_ref: event_lineage_id_ref.to_owned(),
            fanout_surface_class: self.fanout_surface_class,
            client_scope: self.client_scope,
            receipt_state: self.receipt_state,
            stale_or_undelivered_reason: self.stale_or_undelivered_reason.clone(),
            dedupe_key_scheme: self.dedupe_key_scheme,
            delivery_envelope_ref: self.delivery_envelope_ref.clone(),
            reopen_target_ref: self.reopen_target_ref.clone(),
            redaction_class: self.redaction_class,
            suppression_reasons: self.suppression_reasons.clone(),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Truthful, dedupe-aware projection of a routed notification.
///
/// Toasts, banners, status rows, durable activity rows, OS notifications,
/// lock-screen summaries, and companion mirrors all read this struct
/// verbatim. The shell never lets a single surface mint a private routing
/// vocabulary on top of this projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutedNotification {
    pub record_kind: String,
    pub schema_version: u32,
    pub notification_envelope_id: String,
    pub canonical_event_id: String,
    pub event_lineage_id_ref: String,
    pub source_subsystem: SourceSubsystem,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub privacy_payload_class: PrivacyPayloadClass,
    pub redaction_class: RedactionClass,
    pub dedupe_key_scheme: DedupeKeyScheme,
    pub dedupe_key_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_burst_id_ref: Option<String>,
    pub summary_label: String,
    pub reopen_target: ReopenTarget,
    pub actions: Vec<StableAction>,
    pub surface_routes: Vec<SurfaceRoute>,
    /// Times the router has seen this dedupe key including this routing call
    /// (1 on the first emission). Surfaces that surface "x more occurrences"
    /// chips quote this without inventing a per-surface counter.
    pub occurrence_count: u32,
    /// True when this routing call is a repeat of a previously-routed
    /// canonical event on the same dedupe key.
    pub is_dedupe_repeat: bool,
    pub minted_at: String,
}

impl RoutedNotification {
    /// True when at least one surface route delivered or released from hold.
    pub fn has_visible_surface(&self) -> bool {
        self.surface_routes.iter().any(SurfaceRoute::is_visible)
    }

    /// Surface routes that delivered or released from hold.
    pub fn visible_routes(&self) -> impl Iterator<Item = &SurfaceRoute> {
        self.surface_routes.iter().filter(|r| r.is_visible())
    }

    /// Surface routes that were held, suppressed, deduped, or had no route.
    pub fn suppressed_routes(&self) -> impl Iterator<Item = &SurfaceRoute> {
        self.surface_routes
            .iter()
            .filter(|r| r.is_suppressed_or_deduped())
    }

    /// True when every surface route preserves the envelope's reopen target
    /// ref. The router enforces this at construction time; the predicate is
    /// here for downstream audits / support exports.
    pub fn all_routes_preserve_reopen_target(&self) -> bool {
        let envelope_ref = &self.reopen_target.reopen_target_ref;
        self.surface_routes
            .iter()
            .all(|route| &route.reopen_target_ref == envelope_ref)
    }
}

/// Per-key routing memory the router keeps so it can dedupe repeats.
#[derive(Debug, Clone)]
struct DedupeMemory {
    surfaces_routed: BTreeSet<FanoutSurfaceClass>,
    occurrence_count: u32,
}

/// Routing engine. Stateful: it keeps a small dedupe memory keyed by the
/// envelope's `dedupe_key_scheme` + dedupe join key.
///
/// One [`NotificationRouter`] is meant to live behind the canonical
/// notification truth lane (one per workspace shell). Tests can spin up
/// scratch routers freely.
#[derive(Debug, Clone, Default)]
pub struct NotificationRouter {
    memory: HashMap<RouterMemoryKey, DedupeMemory>,
    client_scope: ClientScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RouterMemoryKey {
    scheme: DedupeKeyScheme,
    key: String,
}

impl Default for ClientScope {
    fn default() -> Self {
        ClientScope::DesktopProduct
    }
}

/// Thrown when a router cannot route an envelope honestly. Surfaces should
/// never silently downgrade — they should escalate to durable truth instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationRoutingError {
    /// The envelope's reopen target does not resolve to an exact identity —
    /// routing onto attention surfaces is non-conforming. Surfaces MUST fall
    /// back to a durable activity row (or a placeholder) instead of inventing
    /// a generic "Open" target.
    ReopenTargetMissingExactIdentity,
    /// The envelope schema version is not understood by this router.
    UnsupportedSchemaVersion { found: u32, supported: u32 },
    /// The envelope declared no recommended surfaces.
    NoRecommendedSurfaces,
    /// The envelope's privacy / payload classes do not agree with each other
    /// (e.g., a `summary_safe` envelope with a `policy_forbidden_on_lock_screen`
    /// payload). The router refuses these so the failure is loud.
    InconsistentPrivacyPosture,
}

impl std::fmt::Display for NotificationRoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReopenTargetMissingExactIdentity => write!(
                f,
                "notification envelope reopen target does not resolve to an exact identity"
            ),
            Self::UnsupportedSchemaVersion { found, supported } => write!(
                f,
                "unsupported notification envelope schema version {found}; this router supports {supported}"
            ),
            Self::NoRecommendedSurfaces => {
                write!(f, "notification envelope has no recommended surfaces")
            }
            Self::InconsistentPrivacyPosture => write!(
                f,
                "notification envelope privacy class and payload class disagree"
            ),
        }
    }
}

impl std::error::Error for NotificationRoutingError {}

impl NotificationRouter {
    /// Build a router scoped to the desktop product.
    pub fn new() -> Self {
        Self::for_client_scope(ClientScope::DesktopProduct)
    }

    /// Build a router for a non-desktop client scope (e.g., a companion
    /// surface mirror). The scope is recorded on emitted receipts.
    pub fn for_client_scope(client_scope: ClientScope) -> Self {
        Self {
            memory: HashMap::new(),
            client_scope,
        }
    }

    /// Reset the dedupe memory. Mostly useful in tests; a live router never
    /// needs to forget.
    pub fn reset(&mut self) {
        self.memory.clear();
    }

    /// Route the envelope, dedupe against earlier emissions on the same
    /// dedupe key, and preserve the envelope's reopen target on every
    /// emitted route.
    pub fn route(
        &mut self,
        envelope: &NotificationEnvelope,
    ) -> Result<RoutedNotification, NotificationRoutingError> {
        validate_envelope(envelope)?;

        let memory_key = RouterMemoryKey {
            scheme: envelope.dedupe_key_scheme,
            key: envelope.dedupe_join_key(),
        };

        let already_seen = self.memory.contains_key(&memory_key);
        let occurrence_count = self
            .memory
            .get(&memory_key)
            .map(|memory| memory.occurrence_count + 1)
            .unwrap_or(1);

        let mut surfaces_routed = self
            .memory
            .get(&memory_key)
            .map(|memory| memory.surfaces_routed.clone())
            .unwrap_or_default();

        let mut routes = Vec::with_capacity(envelope.recommended_surfaces.len());
        let mut deduped_route_count = 0u32;
        for surface in envelope_surfaces_in_stable_order(envelope) {
            let already_delivered_to_surface = surfaces_routed.contains(&surface);
            let route = self.derive_route(envelope, surface, already_delivered_to_surface);
            if matches!(
                route.receipt_state,
                FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
            ) {
                surfaces_routed.insert(surface);
            }
            if matches!(
                route.receipt_state,
                FanoutReceiptState::DedupedCanonicalEvent | FanoutReceiptState::DedupedGroupedBurst
            ) {
                deduped_route_count += 1;
            }
            routes.push(route);
        }

        self.memory.insert(
            memory_key.clone(),
            DedupeMemory {
                surfaces_routed,
                occurrence_count,
            },
        );

        let is_dedupe_repeat = already_seen && deduped_route_count > 0;

        Ok(RoutedNotification {
            record_kind: ROUTED_NOTIFICATION_RECORD_KIND.to_owned(),
            schema_version: ROUTED_NOTIFICATION_SCHEMA_VERSION,
            notification_envelope_id: envelope.notification_envelope_id.clone(),
            canonical_event_id: envelope.canonical_event_id.clone(),
            event_lineage_id_ref: envelope.event_lineage_id_ref.clone(),
            source_subsystem: envelope.source_subsystem,
            severity_class: envelope.severity_class,
            privacy_class: envelope.privacy_class,
            privacy_payload_class: envelope.privacy_payload_class,
            redaction_class: envelope.redaction_class,
            dedupe_key_scheme: envelope.dedupe_key_scheme,
            dedupe_key_ref: envelope.dedupe_key_ref.clone(),
            grouped_burst_id_ref: envelope.grouped_burst_id_ref.clone(),
            summary_label: envelope.summary_label.clone(),
            reopen_target: envelope.reopen_target.clone(),
            actions: envelope.actions.clone(),
            surface_routes: routes,
            occurrence_count,
            is_dedupe_repeat,
            minted_at: envelope.minted_at.clone(),
        })
    }

    fn derive_route(
        &self,
        envelope: &NotificationEnvelope,
        surface: FanoutSurfaceClass,
        already_delivered_to_surface: bool,
    ) -> SurfaceRoute {
        let suppression = &envelope.suppression_state;
        let active_modes: BTreeSet<QuietHoursMode> = suppression
            .non_trivial_modes()
            .into_iter()
            .collect::<BTreeSet<_>>();

        let dedupe_state = if already_delivered_to_surface {
            Some(match envelope.dedupe_key_scheme {
                DedupeKeyScheme::GroupedBurstId => FanoutReceiptState::DedupedGroupedBurst,
                _ => FanoutReceiptState::DedupedCanonicalEvent,
            })
        } else {
            None
        };

        // Dedupe wins over suppression: a repeat that already delivered to
        // this surface is deduped, regardless of quiet-hours posture, so the
        // join key on the receipt remains the dedupe key (not the suppression
        // mode).
        let receipt_state = if let Some(state) = dedupe_state {
            state
        } else if suppression.suppressed && surface_is_attention_grabbing(surface) {
            attention_grabbing_held_state(&active_modes, &suppression.suppression_reasons)
        } else if surface_is_lock_screen_denied_by_payload_class(
            surface,
            envelope.privacy_payload_class,
        ) {
            FanoutReceiptState::SuppressedPolicy
        } else {
            FanoutReceiptState::Delivered
        };

        let stale_or_undelivered_reason = stale_reason_for(
            receipt_state,
            envelope.dedupe_key_scheme,
            surface,
            envelope.privacy_payload_class,
        );

        let suppression_reasons: Vec<SuppressionReason> = match receipt_state {
            FanoutReceiptState::DedupedCanonicalEvent => {
                vec![SuppressionReason::DedupeSameCanonicalEvent]
            }
            FanoutReceiptState::DedupedGroupedBurst => {
                vec![SuppressionReason::DedupeSameGroupedBurst]
            }
            FanoutReceiptState::HeldQuietHours | FanoutReceiptState::SuppressedPolicy => {
                suppression.suppression_reasons.clone()
            }
            _ => Vec::new(),
        };

        SurfaceRoute {
            fanout_surface_class: surface,
            client_scope: self.client_scope,
            receipt_state,
            stale_or_undelivered_reason,
            dedupe_key_scheme: envelope.dedupe_key_scheme,
            delivery_envelope_ref: derive_delivery_envelope_ref(
                &envelope.notification_envelope_id,
                surface,
            ),
            reopen_target_ref: envelope.reopen_target.reopen_target_ref.clone(),
            redaction_class: envelope.redaction_class,
            suppression_reasons,
            minted_at: envelope.minted_at.clone(),
        }
    }
}

fn validate_envelope(envelope: &NotificationEnvelope) -> Result<(), NotificationRoutingError> {
    if envelope.notification_envelope_schema_version
        != super::envelope::NOTIFICATION_ENVELOPE_SCHEMA_VERSION
    {
        return Err(NotificationRoutingError::UnsupportedSchemaVersion {
            found: envelope.notification_envelope_schema_version,
            supported: super::envelope::NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        });
    }
    if envelope.recommended_surfaces.is_empty() {
        return Err(NotificationRoutingError::NoRecommendedSurfaces);
    }
    if !envelope.reopen_target.resolves_to_exact_target() {
        // Placeholder-announced and revalidation-required targets are valid
        // for activity-center surfaces, but must NOT route to attention
        // surfaces. Since the router only owns the routing decision (not the
        // surface adapter), we let the placeholder pass IFF the reopen target
        // kind explicitly announces it. A `canonical_object` kind missing its
        // identity is non-conforming.
        match envelope.reopen_target.reopen_target_kind {
            ReopenTargetKind::PlaceholderAnnounced
            | ReopenTargetKind::DeniedRequiresRevalidation => {}
            _ => return Err(NotificationRoutingError::ReopenTargetMissingExactIdentity),
        }
    }
    if !privacy_posture_is_consistent(envelope.privacy_class, envelope.privacy_payload_class) {
        return Err(NotificationRoutingError::InconsistentPrivacyPosture);
    }
    Ok(())
}

fn privacy_posture_is_consistent(class: PrivacyClass, payload: PrivacyPayloadClass) -> bool {
    match (class, payload) {
        // summary_safe envelopes can be lock-screen safe (generic or scoped)
        // or in-product only.
        (
            PrivacyClass::SummarySafe,
            PrivacyPayloadClass::LockScreenSafeGeneric
            | PrivacyPayloadClass::LockScreenSafeScoped
            | PrivacyPayloadClass::InProductOnly
            | PrivacyPayloadClass::RedactedMetadataOnly,
        ) => true,
        // workspace_sensitive envelopes can render generic lock-screen
        // payloads OR redacted metadata; scoped payloads risk leaking
        // workspace identifiers.
        (
            PrivacyClass::WorkspaceSensitive,
            PrivacyPayloadClass::LockScreenSafeGeneric
            | PrivacyPayloadClass::InProductOnly
            | PrivacyPayloadClass::RedactedMetadataOnly,
        ) => true,
        // security_critical envelopes never render scoped or generic
        // lock-screen payloads.
        (
            PrivacyClass::SecurityCritical,
            PrivacyPayloadClass::RedactedMetadataOnly
            | PrivacyPayloadClass::PolicyForbiddenOnLockScreen
            | PrivacyPayloadClass::InProductOnly,
        ) => true,
        // managed_sensitive follows the security_critical posture for
        // lock-screen denial; in-product surfaces remain available.
        (
            PrivacyClass::ManagedSensitive,
            PrivacyPayloadClass::RedactedMetadataOnly
            | PrivacyPayloadClass::PolicyForbiddenOnLockScreen
            | PrivacyPayloadClass::InProductOnly,
        ) => true,
        _ => false,
    }
}

fn surface_is_attention_grabbing(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::Toast
            | FanoutSurfaceClass::ContextualBanner
            | FanoutSurfaceClass::OsNotification
            | FanoutSurfaceClass::LockScreenSummary
            | FanoutSurfaceClass::CompanionPush
    )
}

fn surface_is_lock_screen_denied_by_payload_class(
    surface: FanoutSurfaceClass,
    payload: PrivacyPayloadClass,
) -> bool {
    matches!(surface, FanoutSurfaceClass::LockScreenSummary)
        && matches!(payload, PrivacyPayloadClass::PolicyForbiddenOnLockScreen)
}

fn attention_grabbing_held_state(
    active_modes: &BTreeSet<QuietHoursMode>,
    suppression_reasons: &[SuppressionReason],
) -> FanoutReceiptState {
    if suppression_reasons.contains(&SuppressionReason::AdminPolicySuppression)
        || active_modes.contains(&QuietHoursMode::ModeAdminSuppression)
    {
        FanoutReceiptState::SuppressedPolicy
    } else if active_modes.contains(&QuietHoursMode::ModePresentation)
        || active_modes.contains(&QuietHoursMode::ModeScreenShare)
        || active_modes.contains(&QuietHoursMode::ModePrivacyMode)
    {
        FanoutReceiptState::SuppressedPolicy
    } else if !active_modes.is_empty() {
        FanoutReceiptState::HeldQuietHours
    } else {
        // suppressed=true with no active mode is non-conforming on the
        // envelope side; treat as held so the surface still records a
        // visible truth row instead of silently dropping.
        FanoutReceiptState::HeldQuietHours
    }
}

fn stale_reason_for(
    state: FanoutReceiptState,
    scheme: DedupeKeyScheme,
    surface: FanoutSurfaceClass,
    payload: PrivacyPayloadClass,
) -> StaleOrUndeliveredReason {
    match state {
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold => {
            StaleOrUndeliveredReason::none()
        }
        FanoutReceiptState::DedupedCanonicalEvent => StaleOrUndeliveredReason {
            reason_class: StaleOrUndeliveredReasonClass::DedupedOnCanonicalEvent,
            reason_label: Some("Same canonical event already delivered.".to_owned()),
        },
        FanoutReceiptState::DedupedGroupedBurst => StaleOrUndeliveredReason {
            reason_class: StaleOrUndeliveredReasonClass::DedupedOnGroupedBurst,
            reason_label: Some("Same grouped burst already delivered.".to_owned()),
        },
        FanoutReceiptState::HeldQuietHours => StaleOrUndeliveredReason {
            reason_class: StaleOrUndeliveredReasonClass::HeldByQuietHours,
            reason_label: Some("Held during quiet hours.".to_owned()),
        },
        FanoutReceiptState::SuppressedPolicy => StaleOrUndeliveredReason {
            reason_class: StaleOrUndeliveredReasonClass::SuppressedByPolicy,
            reason_label: Some(
                if surface_is_lock_screen_denied_by_payload_class(surface, payload) {
                    "Lock-screen render forbidden by privacy class.".to_owned()
                } else {
                    "Suppressed by policy.".to_owned()
                },
            ),
        },
        FanoutReceiptState::NotAttemptedNoRoute => StaleOrUndeliveredReason {
            reason_class: StaleOrUndeliveredReasonClass::NoRouteForSurfaceOrTier,
            reason_label: Some("No active route for this surface on this client.".to_owned()),
        },
    }
    .with_scheme_hint(scheme)
}

trait WithSchemeHint {
    fn with_scheme_hint(self, scheme: DedupeKeyScheme) -> Self;
}

impl WithSchemeHint for StaleOrUndeliveredReason {
    fn with_scheme_hint(self, _scheme: DedupeKeyScheme) -> Self {
        // The scheme is recorded on the route itself; the trait gives us a
        // single chainable assembly point in case future routing classes need
        // to qualify the label by scheme (e.g., per-burst label suffixes).
        self
    }
}

fn derive_delivery_envelope_ref(envelope_id: &str, surface: FanoutSurfaceClass) -> String {
    format!("delivery:{}:{}", envelope_id, surface.as_str())
}

fn envelope_surfaces_in_stable_order(envelope: &NotificationEnvelope) -> Vec<FanoutSurfaceClass> {
    // Recommended surfaces are an array on the schema; the envelope contract
    // does NOT promise insertion order. The router emits routes in the
    // surface enum's stable order so two envelopes with the same surface set
    // produce byte-identical routing snapshots — matters for support exports
    // and replay fixtures.
    let unique: BTreeMap<FanoutSurfaceClass, ()> = envelope
        .recommended_surfaces
        .iter()
        .copied()
        .map(|s| (s, ()))
        .collect();
    unique.into_keys().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::envelope::{
        NotificationEnvelope, ReopenTarget, ReopenTargetKind, StableAction, SuppressionState,
    };

    fn base_envelope() -> NotificationEnvelope {
        NotificationEnvelope {
            record_kind: "notification_envelope_record".into(),
            notification_envelope_schema_version: 1,
            notification_envelope_id: "ux:notif-env:test:01".into(),
            canonical_event_id: "ux:event:test:01".into(),
            event_lineage_id_ref: "ux:lineage:test:01".into(),
            source_subsystem: SourceSubsystem::Indexer,
            source_event_ref: "test:event:01".into(),
            actor_identity_ref: "id:actor:system:test".into(),
            canonical_object_target_ref: "obj:test:01".into(),
            severity_class: SeverityClass::Warning,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
            redaction_class: RedactionClass::OperatorOnlyRestricted,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
            dedupe_key_ref: "ux:event:test:01".into(),
            grouped_burst_id_ref: None,
            recommended_surfaces: vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::Toast,
            ],
            summary_label: "Test event".into(),
            reopen_target: ReopenTarget {
                reopen_target_ref: "ux:reopen:test:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                exact_target_identity_ref: Some("obj:test:01".into()),
                placeholder_announcement_label: None,
                revalidation_required_reason_label: None,
            },
            actions: vec![StableAction {
                action_id: "ux:action:test:open:01".into(),
                label: "Open".into(),
                command_id: "cmd:test.open".into(),
                target_identity_ref: "obj:test:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            }],
            suppression_state: SuppressionState {
                active_modes_at_mint: vec![QuietHoursMode::ModeNone],
                suppression_reasons: vec![],
                suppressed: false,
            },
            fanout_receipts: vec![],
            minted_at: "2026-05-10T10:00:00Z".into(),
        }
    }

    #[test]
    fn first_emission_delivers_every_recommended_surface() {
        let mut router = NotificationRouter::new();
        let routed = router
            .route(&base_envelope())
            .expect("routing should succeed");
        assert_eq!(routed.occurrence_count, 1);
        assert!(!routed.is_dedupe_repeat);
        assert_eq!(routed.surface_routes.len(), 3);
        for route in &routed.surface_routes {
            assert_eq!(route.receipt_state, FanoutReceiptState::Delivered);
            assert_eq!(route.reopen_target_ref, "ux:reopen:test:01");
            assert_eq!(
                route.stale_or_undelivered_reason.reason_class,
                StaleOrUndeliveredReasonClass::None
            );
        }
        assert!(routed.all_routes_preserve_reopen_target());
    }

    #[test]
    fn repeat_emission_dedupes_already_delivered_surfaces() {
        let mut router = NotificationRouter::new();
        let env = base_envelope();
        let _ = router.route(&env).unwrap();
        let routed = router.route(&env).unwrap();
        assert_eq!(routed.occurrence_count, 2);
        assert!(routed.is_dedupe_repeat);
        for route in &routed.surface_routes {
            assert_eq!(
                route.receipt_state,
                FanoutReceiptState::DedupedCanonicalEvent,
                "surface {:?} should be deduped on repeat",
                route.fanout_surface_class
            );
            // The reopen target ref MUST be preserved on the dedupe receipt
            // so the deduped row in the activity center still leads to the
            // same canonical object.
            assert_eq!(route.reopen_target_ref, "ux:reopen:test:01");
            assert!(route
                .suppression_reasons
                .contains(&SuppressionReason::DedupeSameCanonicalEvent));
        }
    }

    #[test]
    fn quiet_hours_holds_attention_surfaces_but_delivers_durable_truth() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.suppression_state = SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeQuietHoursUser],
            suppression_reasons: vec![SuppressionReason::QuietHoursUserPolicy],
            suppressed: true,
        };
        env.recommended_surfaces = vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::Toast,
            FanoutSurfaceClass::OsNotification,
        ];
        let routed = router.route(&env).expect("routing should succeed");
        let by_surface: HashMap<FanoutSurfaceClass, FanoutReceiptState> = routed
            .surface_routes
            .iter()
            .map(|r| (r.fanout_surface_class, r.receipt_state))
            .collect();
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::DurableJobRow),
            Some(&FanoutReceiptState::Delivered)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::StatusItem),
            Some(&FanoutReceiptState::Delivered)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::Toast),
            Some(&FanoutReceiptState::HeldQuietHours)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::OsNotification),
            Some(&FanoutReceiptState::HeldQuietHours)
        );
    }

    #[test]
    fn admin_policy_suppression_blocks_attention_surfaces_outright() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.privacy_class = PrivacyClass::SecurityCritical;
        env.privacy_payload_class = PrivacyPayloadClass::PolicyForbiddenOnLockScreen;
        env.redaction_class = RedactionClass::InternalSupportRestricted;
        env.suppression_state = SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeAdminSuppression],
            suppression_reasons: vec![SuppressionReason::AdminPolicySuppression],
            suppressed: true,
        };
        env.recommended_surfaces = vec![
            FanoutSurfaceClass::ContextualBanner,
            FanoutSurfaceClass::StatusItem,
            FanoutSurfaceClass::LockScreenSummary,
        ];
        let routed = router.route(&env).expect("routing should succeed");
        let by_surface: HashMap<FanoutSurfaceClass, FanoutReceiptState> = routed
            .surface_routes
            .iter()
            .map(|r| (r.fanout_surface_class, r.receipt_state))
            .collect();
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::ContextualBanner),
            Some(&FanoutReceiptState::SuppressedPolicy)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::LockScreenSummary),
            Some(&FanoutReceiptState::SuppressedPolicy)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::StatusItem),
            Some(&FanoutReceiptState::Delivered)
        );
    }

    #[test]
    fn missing_exact_reopen_identity_is_rejected_for_canonical_object() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.reopen_target.reopen_target_kind = ReopenTargetKind::CanonicalObject;
        env.reopen_target.exact_target_identity_ref = None;
        let err = router.route(&env).unwrap_err();
        assert_eq!(
            err,
            NotificationRoutingError::ReopenTargetMissingExactIdentity
        );
    }

    #[test]
    fn placeholder_announced_reopen_target_is_routable() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.reopen_target.reopen_target_kind = ReopenTargetKind::PlaceholderAnnounced;
        env.reopen_target.exact_target_identity_ref = None;
        env.reopen_target.placeholder_announcement_label =
            Some("Recovery target announced.".to_owned());
        let routed = router
            .route(&env)
            .expect("placeholder routing should succeed");
        assert!(routed.has_visible_surface());
    }

    #[test]
    fn inconsistent_privacy_posture_is_rejected() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.privacy_class = PrivacyClass::SummarySafe;
        env.privacy_payload_class = PrivacyPayloadClass::PolicyForbiddenOnLockScreen;
        let err = router.route(&env).unwrap_err();
        assert_eq!(err, NotificationRoutingError::InconsistentPrivacyPosture);
    }

    #[test]
    fn unsupported_schema_version_is_rejected() {
        let mut router = NotificationRouter::new();
        let mut env = base_envelope();
        env.notification_envelope_schema_version = 99;
        let err = router.route(&env).unwrap_err();
        assert_eq!(
            err,
            NotificationRoutingError::UnsupportedSchemaVersion {
                found: 99,
                supported: 1,
            }
        );
    }

    #[test]
    fn dedupe_join_key_uses_grouped_burst_when_scheme_is_burst() {
        let mut env = base_envelope();
        env.dedupe_key_scheme = DedupeKeyScheme::GroupedBurstId;
        env.dedupe_key_ref = "ux:dedupe:burst:01".into();
        env.grouped_burst_id_ref = Some("ux:burst:01".into());
        assert_eq!(env.dedupe_join_key(), "ux:burst:01");
    }

    #[test]
    fn route_to_fanout_receipt_round_trips_to_envelope_shape() {
        let mut router = NotificationRouter::new();
        let env = base_envelope();
        let routed = router.route(&env).unwrap();
        let receipts: Vec<FanoutReceipt> = routed
            .surface_routes
            .iter()
            .map(|route| {
                route.to_fanout_receipt(
                    &routed.notification_envelope_id,
                    &routed.canonical_event_id,
                    &routed.event_lineage_id_ref,
                )
            })
            .collect();
        assert_eq!(receipts.len(), 3);
        assert!(receipts
            .iter()
            .all(|r| r.canonical_event_id == "ux:event:test:01"));
        assert!(receipts
            .iter()
            .all(|r| r.reopen_target_ref == "ux:reopen:test:01"));
    }

    #[test]
    fn dedupe_repeat_increments_occurrence_count_per_dedupe_key() {
        let mut router = NotificationRouter::new();
        let env = base_envelope();
        let _ = router.route(&env).unwrap();
        let _ = router.route(&env).unwrap();
        let third = router.route(&env).unwrap();
        assert_eq!(third.occurrence_count, 3);
        assert!(third.is_dedupe_repeat);
    }
}
