//! M5 *fanout receipts*: the working engine that mints one durable, privacy-safe
//! delivery-truth record per cross-client destination when an attention object is
//! fanned out to native OS notifications and browser/mobile companions.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes the
//! contract* — including the
//! [`FanoutReceipt`](crate::m5_attention_routing::AttentionObjectClass::FanoutReceipt)
//! object family and the `fanout_stale` / `fanout_undelivered` states — and
//! [`m5_envelope_routing`](crate::m5_envelope_routing) *routes a fresh envelope to its
//! surfaces*, this lane turns out-of-window delivery into a *governed receipt model*
//! rather than a best-effort side effect. Every push to an OS notification or a companion
//! produces a [`FanoutReceipt`] that names the source notification, the destination client
//! class, the delivery state, an explicit stale or undelivered reason, and a privacy-safe
//! summary posture — so a failed fanout is *visible truth*, never silently counted as
//! delivered.
//!
//! [`mint_receipt`] is a pure function of a [`FanoutSource`] (the delivery-relevant
//! projection of a notification envelope or durable activity object) and a
//! [`FanoutAttempt`] (what the transport layer reported for one destination). It returns a
//! [`FanoutReceipt`]; [`mint_dispatch`] mints one receipt per governed destination for a
//! named [`FanoutConditionClass`], so the same `(source, condition)` yields the same
//! [`FanoutDispatch`] byte-for-byte in support export and CLI/headless diagnostics. The
//! honesty rules the track invariant requires are enforced, not just described:
//!
//! - **No silent success.** A destination is reported delivered only when the transport
//!   acknowledged it; a stale or failed copy is labeled `fanout_stale` /
//!   `fanout_undelivered` with an explicit reason, never folded into the delivered count
//!   (`fanout.failures_labeled_never_counted_delivered`).
//! - **Stale and undelivered states explain themselves.** Every gap carries a reason class
//!   and a short reviewable sentence (`fanout.stale_undelivered_have_reason`).
//! - **Privacy-safe summaries by default.** Every delivered or stale copy uses a
//!   privacy-safe posture that never renders the full payload and never drops below the
//!   source's privacy floor (`fanout.privacy_safe_summary_default`).
//! - **Lock-screen-safe.** On a locked screen, an above-summary-safe copy is reduced to a
//!   count-only affordance (`fanout.lock_screen_safe`).
//! - **Managed endpoints never receive the payload.** A non-compliant managed endpoint is
//!   recorded as undelivered with the managed-endpoint reason and renders no summary
//!   (`fanout.managed_endpoint_blocks_payload`).
//! - **Reopen parity.** Every receipt reopens the same authoritative object the source
//!   names, by an exact anchor, never an ambiguous generic shell
//!   (`fanout.reopen_parity`).
//! - **No preview/approval bypass.** A receipt whose source routes through preview/approval
//!   may not execute the action inline; it hands off to the in-product flow
//!   (`fanout.no_preview_approval_bypass`).
//! - **The durable record survives.** Even an all-undelivered or all-suppressed fanout
//!   keeps the authoritative in-product record (`fanout.durable_record_present`).
//! - **Suppression is not failure.** A policy-withheld copy is recorded as suppressed with
//!   a suppression reason, kept distinct from a transport failure
//!   (`fanout.suppressed_separate_from_failure`).
//!
//! The canonical [`fanout_receipts_bundle`] freezes the governed destinations, a
//! representative source corpus, the transport-condition corpus, and every dispatch so the
//! freeze gate and checked-in fixture pin the contract byte-for-byte. Every privacy class,
//! scope, redaction class, reopen target, severity, dedupe scheme, and resulting state the
//! bundle uses is one the attention-routing matrix defines, so the fanout path can never
//! drift from the frozen object model (`fanout.matrix_bound`).
//!
//! The record carries no message bodies, credentials, raw provider payloads, hostnames,
//! device identifiers, or absolute paths — only opaque object refs, stable tokens, and
//! short reviewable sentences — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionObjectClass,
    AttentionRedactionClass, AttentionRoutingMatrix, AttentionScopeClass, AttentionStateClass,
    FanoutChannelClass, NotificationPrivacyClass, ReopenTargetClass,
    M5_ATTENTION_ROUTING_MATRIX_ID,
};
use crate::m5_envelope_routing::{
    DedupeStrategyClass, NotificationSeverityClass, SourceSubsystemClass,
};

#[cfg(test)]
mod tests;

/// Schema version for the fanout-receipts bundle.
pub const M5_FANOUT_RECEIPTS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the fanout-receipts bundle.
pub const M5_FANOUT_RECEIPTS_SCHEMA_REF: &str = "schemas/activity/m5-fanout-receipts.schema.json";

/// Stable record-kind tag for the fanout-receipts bundle.
pub const M5_FANOUT_RECEIPTS_RECORD_KIND: &str = "m5_fanout_receipts_bundle";

/// Stable id for the canonical fanout-receipts bundle.
pub const M5_FANOUT_RECEIPTS_BUNDLE_ID: &str = "m5-fanout-receipts:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding stays
/// deterministic and the fixture freezes byte-for-byte.
pub const M5_FANOUT_RECEIPTS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_FANOUT_RECEIPTS_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate; it
/// fails when the in-code bundle drifts from the checked-in fixture or any invariant flips.
pub const M5_FANOUT_RECEIPTS_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_fanout_receipts.rs";

/// The cross-client destinations this lane mints receipts for: the out-of-window native OS
/// notification plus the browser and mobile companions.
///
/// The in-app activity center is the *authoritative durable record*, not a fanout copy, so
/// it is not a destination here; the dock/taskbar badge and operator dashboard are governed
/// by their own lanes. This lane proves per-destination delivery truth for exactly the
/// native OS and companion surfaces the spec names.
pub const GOVERNED_DESTINATIONS: [FanoutChannelClass; 3] = [
    FanoutChannelClass::OsNativeNotification,
    FanoutChannelClass::BrowserCompanion,
    FanoutChannelClass::MobileCompanion,
];

// ---------------------------------------------------------------------------
// Receipt-state vocabulary.
// ---------------------------------------------------------------------------

/// The delivery truth for one cross-client copy.
///
/// Each state maps to exactly one matrix [`AttentionStateClass`], so a receipt's delivery
/// truth is named in the frozen vocabulary. Only [`Delivered`](Self::Delivered) counts as
/// a successful fanout; every other state is a labeled gap or a policy suppression and is
/// never folded into the delivered count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutDeliveryStateClass {
    /// The transport acknowledged the copy and it mirrors the authoritative object.
    Delivered,
    /// The copy was delivered earlier but now lags the authoritative object.
    Stale,
    /// The copy was never delivered or delivery failed, labeled as such.
    Undelivered,
    /// The copy was withheld by policy (not a transport failure); the durable record holds
    /// the attention.
    Suppressed,
    /// The transport state could not be determined and requires review.
    Unknown,
}

impl FanoutDeliveryStateClass {
    /// All states, in order.
    pub const ALL: [Self; 5] = [
        Self::Delivered,
        Self::Stale,
        Self::Undelivered,
        Self::Suppressed,
        Self::Unknown,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delivered => "delivered",
            Self::Stale => "stale",
            Self::Undelivered => "undelivered",
            Self::Suppressed => "suppressed",
            Self::Unknown => "unknown",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Delivered => "Delivered",
            Self::Stale => "Stale",
            Self::Undelivered => "Undelivered",
            Self::Suppressed => "Suppressed",
            Self::Unknown => "Unknown — requires review",
        }
    }

    /// The matrix state this delivery state maps to.
    pub const fn matrix_state(self) -> AttentionStateClass {
        match self {
            Self::Delivered => AttentionStateClass::Shown,
            Self::Stale => AttentionStateClass::FanoutStale,
            Self::Undelivered => AttentionStateClass::FanoutUndelivered,
            Self::Suppressed => AttentionStateClass::Suppressed,
            Self::Unknown => AttentionStateClass::UnknownRequiresReview,
        }
    }

    /// Whether the copy reached the destination as a current mirror.
    pub const fn is_delivered(self) -> bool {
        matches!(self, Self::Delivered)
    }

    /// Whether the copy was rendered in some form (delivered or stale) and so carries a
    /// privacy-safe summary posture.
    pub const fn is_rendered(self) -> bool {
        matches!(self, Self::Delivered | Self::Stale | Self::Unknown)
    }

    /// Whether this state is a cross-client delivery gap (stale or undelivered) that must
    /// carry an explicit reason rather than be counted as delivered.
    pub const fn is_delivery_gap(self) -> bool {
        matches!(self, Self::Stale | Self::Undelivered | Self::Unknown)
    }

    /// Whether the copy was withheld by policy rather than failing to deliver.
    pub const fn is_suppressed(self) -> bool {
        matches!(self, Self::Suppressed)
    }
}

/// Why a copy is stale, undelivered, or in an unknown state.
///
/// [`StaleUndeliveredReasonClass::None`] means the copy was delivered or suppressed (a
/// suppression carries a [`FanoutSuppressionReasonClass`] instead); every other reason
/// names exactly one delivery-gap cause, so a non-delivered receipt can always explain why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleUndeliveredReasonClass {
    /// No delivery-gap reason (the copy was delivered or suppressed).
    None,
    /// Stale: the authoritative object advanced past the delivered copy.
    SupersededByNewerState,
    /// Undelivered: the client was offline or the push service was unreachable.
    ClientUnreachable,
    /// Undelivered: no delivery acknowledgement arrived within the window.
    DeliveryTimedOut,
    /// Undelivered: the managed endpoint is non-compliant and may not receive the payload.
    ManagedEndpointBlocked,
    /// Unknown: the transport state could not be determined.
    TransportIndeterminate,
}

impl StaleUndeliveredReasonClass {
    /// All reasons, in order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::SupersededByNewerState,
        Self::ClientUnreachable,
        Self::DeliveryTimedOut,
        Self::ManagedEndpointBlocked,
        Self::TransportIndeterminate,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SupersededByNewerState => "superseded_by_newer_state",
            Self::ClientUnreachable => "client_unreachable",
            Self::DeliveryTimedOut => "delivery_timed_out",
            Self::ManagedEndpointBlocked => "managed_endpoint_blocked",
            Self::TransportIndeterminate => "transport_indeterminate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SupersededByNewerState => "Superseded by newer state",
            Self::ClientUnreachable => "Client unreachable",
            Self::DeliveryTimedOut => "Delivery timed out",
            Self::ManagedEndpointBlocked => "Managed endpoint blocked",
            Self::TransportIndeterminate => "Transport indeterminate",
        }
    }

    /// Whether this is a named delivery-gap reason (anything but
    /// [`StaleUndeliveredReasonClass::None`]).
    pub const fn is_named(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Why a copy was suppressed by policy (distinct from a transport failure).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutSuppressionReasonClass {
    /// Not suppressed.
    None,
    /// Withheld by an active quiet-hours / do-not-disturb / presentation policy.
    QuietHoursPolicy,
    /// Withheld by an admin / managed suppression policy.
    AdminPolicy,
}

impl FanoutSuppressionReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::QuietHoursPolicy => "quiet_hours_policy",
            Self::AdminPolicy => "admin_policy",
        }
    }

    /// Whether this is a named suppression reason.
    pub const fn is_named(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The privacy-safe summary posture a destination renders for a copy.
///
/// The posture is the human-facing privacy treatment; the matrix-bound
/// [`AttentionRedactionClass`] derived from it ties the posture back to the frozen
/// vocabulary. No posture ever renders the full payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutSummaryPostureClass {
    /// A short summary is allowed (a summary-safe source).
    ClearSummary,
    /// A redacted, generic summary only (a workspace- or security-sensitive source).
    RedactedSummary,
    /// A count-only affordance with no content in the clear (a locked screen).
    LockScreenSafe,
    /// An open-app affordance with no payload (a managed-sensitive source).
    OpenAppOnly,
    /// Nothing rendered (the copy was undelivered or suppressed).
    NoSummary,
}

impl FanoutSummaryPostureClass {
    /// All postures, in order.
    pub const ALL: [Self; 5] = [
        Self::ClearSummary,
        Self::RedactedSummary,
        Self::LockScreenSafe,
        Self::OpenAppOnly,
        Self::NoSummary,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearSummary => "clear_summary",
            Self::RedactedSummary => "redacted_summary",
            Self::LockScreenSafe => "lock_screen_safe",
            Self::OpenAppOnly => "open_app_only",
            Self::NoSummary => "no_summary",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ClearSummary => "Clear summary",
            Self::RedactedSummary => "Redacted summary",
            Self::LockScreenSafe => "Lock-screen safe",
            Self::OpenAppOnly => "Open-app only",
            Self::NoSummary => "No summary",
        }
    }

    /// Whether this posture renders a count-only / lock-screen-safe affordance.
    pub const fn is_lock_screen_safe(self) -> bool {
        matches!(self, Self::LockScreenSafe)
    }

    /// The matrix redaction class this posture applies.
    pub const fn redaction(self) -> AttentionRedactionClass {
        match self {
            Self::ClearSummary => AttentionRedactionClass::SummaryOnly,
            Self::RedactedSummary => AttentionRedactionClass::RedactedPayload,
            // A count-only affordance, an open-app-only affordance, and a not-rendered copy
            // all render no titles or summaries.
            Self::LockScreenSafe | Self::OpenAppOnly | Self::NoSummary => {
                AttentionRedactionClass::CountOnly
            }
        }
    }
}

/// What the transport layer reported for one destination attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportSignalClass {
    /// The destination acknowledged the copy.
    Acknowledged,
    /// The destination held an earlier copy and the authoritative object advanced.
    Superseded,
    /// The destination was offline or the push service was unreachable.
    Unreachable,
    /// No acknowledgement arrived within the window.
    TimedOut,
    /// The copy was withheld by policy before any attempt.
    PolicyWithheld,
    /// The transport state could not be determined.
    Indeterminate,
}

impl TransportSignalClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Acknowledged => "acknowledged",
            Self::Superseded => "superseded",
            Self::Unreachable => "unreachable",
            Self::TimedOut => "timed_out",
            Self::PolicyWithheld => "policy_withheld",
            Self::Indeterminate => "indeterminate",
        }
    }
}

/// A named transport-and-environment condition applied across every governed destination of
/// a dispatch, so the corpus exercises every delivery state, reason, and posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanoutConditionClass {
    /// Every destination acknowledged the copy.
    AllDelivered,
    /// The mobile companion holds a superseded copy; the others delivered.
    MobileStale,
    /// The browser companion was unreachable; the others delivered.
    CompanionUndelivered,
    /// The OS notification timed out; the others delivered.
    OsTimedOut,
    /// The screen is locked; every destination delivered with a lock-screen-safe posture.
    LockedScreen,
    /// The managed endpoint is non-compliant; every destination is undelivered.
    ManagedEndpointBlocked,
    /// Policy withheld the fanout; every destination is suppressed.
    PolicyWithheld,
    /// The OS notification transport state is unknown; the others delivered.
    TransportUnknown,
}

impl FanoutConditionClass {
    /// All conditions, in order.
    pub const ALL: [Self; 8] = [
        Self::AllDelivered,
        Self::MobileStale,
        Self::CompanionUndelivered,
        Self::OsTimedOut,
        Self::LockedScreen,
        Self::ManagedEndpointBlocked,
        Self::PolicyWithheld,
        Self::TransportUnknown,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllDelivered => "all_delivered",
            Self::MobileStale => "mobile_stale",
            Self::CompanionUndelivered => "companion_undelivered",
            Self::OsTimedOut => "os_timed_out",
            Self::LockedScreen => "locked_screen",
            Self::ManagedEndpointBlocked => "managed_endpoint_blocked",
            Self::PolicyWithheld => "policy_withheld",
            Self::TransportUnknown => "transport_unknown",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AllDelivered => "All delivered",
            Self::MobileStale => "Mobile companion stale",
            Self::CompanionUndelivered => "Browser companion undelivered",
            Self::OsTimedOut => "OS notification timed out",
            Self::LockedScreen => "Locked screen",
            Self::ManagedEndpointBlocked => "Managed endpoint blocked",
            Self::PolicyWithheld => "Policy withheld",
            Self::TransportUnknown => "OS transport unknown",
        }
    }
}

// ---------------------------------------------------------------------------
// Source (the attention object being fanned out).
// ---------------------------------------------------------------------------

/// The delivery-relevant projection of a notification envelope or durable activity object,
/// from which fanout receipts are minted.
///
/// It carries the source notification id, the canonical dedupe identity, the privacy class,
/// the authoritative reopen route, and whether the action routes through preview/approval —
/// but never raw message text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutSource {
    /// Stable, namespaced source notification-envelope id.
    pub source_envelope_id: String,
    /// The canonical event id used for cross-client dedupe.
    pub canonical_event_id: String,
    /// Human-readable label.
    pub label: String,
    /// The source subsystem that produced the event.
    pub source_subsystem: SourceSubsystemClass,
    /// The severity class.
    pub severity: NotificationSeverityClass,
    /// The scope namespace the attention applies to.
    pub scope: AttentionScopeClass,
    /// The privacy class governing what may be mirrored on a destination.
    pub privacy_class: NotificationPrivacyClass,
    /// How repeated events coalesce into one canonical copy.
    pub dedupe_key_scheme: DedupeStrategyClass,
    /// The authoritative object every fanout copy reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (never a URL, host, or path).
    pub reopen_anchor_ref: String,
    /// Whether acting on this attention must route through the in-product preview/approval
    /// flow. When true, no out-of-window copy may execute the action inline.
    pub routes_through_preview_approval: bool,
    /// Whether the event is backed by a durable authoritative record (always true — fanout
    /// receipts are never minted for toast-only attention).
    pub carries_durable_record: bool,
    /// Evaluation stamp.
    pub created_at: String,
}

impl FanoutSource {
    /// The privacy floor: the weakest redaction a destination copy may use for this source,
    /// so a fanout copy never widens privacy below what the source class requires.
    fn privacy_floor(&self) -> AttentionRedactionClass {
        use AttentionRedactionClass::*;
        use NotificationPrivacyClass::*;
        match self.privacy_class {
            SummarySafe => SummaryOnly,
            WorkspaceSensitive | SecurityCritical => RedactedPayload,
            ManagedSensitive => CountOnly,
        }
    }

    /// The base summary posture for this source on an unlocked, compliant endpoint.
    fn base_posture(&self) -> FanoutSummaryPostureClass {
        use FanoutSummaryPostureClass::*;
        use NotificationPrivacyClass::*;
        match self.privacy_class {
            SummarySafe => ClearSummary,
            WorkspaceSensitive | SecurityCritical => RedactedSummary,
            ManagedSensitive => OpenAppOnly,
        }
    }
}

// ---------------------------------------------------------------------------
// Attempt (what the transport reported for one destination).
// ---------------------------------------------------------------------------

/// What the transport layer reported for one destination, the per-destination input to
/// [`mint_receipt`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutAttempt {
    /// Stable, namespaced attempt id; becomes the receipt's identity.
    pub attempt_id: String,
    /// The destination client class.
    pub destination: FanoutChannelClass,
    /// A redaction-safe token naming the destination client (never a host, device id, or
    /// path).
    pub client_scope: String,
    /// What the transport reported.
    pub transport: TransportSignalClass,
    /// Whether the destination screen is locked.
    pub lock_screen_locked: bool,
    /// Whether the destination is a non-compliant managed endpoint.
    pub managed_endpoint_noncompliant: bool,
}

// ---------------------------------------------------------------------------
// Receipt and dispatch.
// ---------------------------------------------------------------------------

/// One durable, privacy-safe delivery-truth record for one cross-client copy.
///
/// The receipt names the source notification, the destination client class, the delivery
/// state, an explicit stale/undelivered reason, the privacy-safe summary posture, and the
/// reopen route back to the same authoritative object — so a failed fanout is visible truth
/// and an external alert can never bypass preview/approval or land on a generic shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutReceipt {
    /// Stable, namespaced receipt id.
    pub receipt_id: String,
    /// The source notification-envelope id this copy was fanned out from.
    pub source_envelope_id: String,
    /// The canonical event id used for cross-client dedupe.
    pub canonical_event_id: String,
    /// The destination client class.
    pub destination: FanoutChannelClass,
    /// Stable destination channel id.
    pub destination_id: String,
    /// A redaction-safe token naming the destination client.
    pub client_scope: String,
    /// The delivery truth for this copy.
    pub delivery_state: FanoutDeliveryStateClass,
    /// Stable delivery-state token.
    pub delivery_state_token: String,
    /// The matrix state this delivery state maps to.
    pub resulting_state: AttentionStateClass,
    /// The explicit stale/undelivered reason (`none` for a delivered or suppressed copy).
    pub stale_or_undelivered_reason: StaleUndeliveredReasonClass,
    /// Stable stale/undelivered-reason token.
    pub stale_or_undelivered_reason_token: String,
    /// The suppression reason when the copy was withheld by policy (`none` otherwise).
    pub suppression_reason: FanoutSuppressionReasonClass,
    /// The privacy-safe summary posture rendered for this copy.
    pub summary_posture: FanoutSummaryPostureClass,
    /// Stable summary-posture token.
    pub summary_posture_token: String,
    /// The matrix redaction class applied.
    pub applied_redaction: AttentionRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// How repeated events coalesce into one canonical copy.
    pub dedupe_key_scheme: DedupeStrategyClass,
    /// The authoritative object this copy reopens (always the source's).
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (always the source's; never a generic shell).
    pub reopen_anchor_ref: String,
    /// Whether this copy reopens an exact authoritative target rather than an ambiguous
    /// generic shell (always true — the anchor is the source's exact anchor).
    pub reopen_is_exact: bool,
    /// Whether acting on this copy must route through the in-product preview/approval flow.
    pub routes_through_preview_approval: bool,
    /// Whether this copy may execute its action inline. Always false when the source routes
    /// through preview/approval, so an external alert hands off instead of bypassing the
    /// guardrail.
    pub inline_action_allowed: bool,
    /// Whether the underlying authoritative record is still present regardless of this
    /// copy's delivery state (always true).
    pub durable_record_present: bool,
    /// When the receipt was minted.
    pub minted_at: String,
    /// One reviewable sentence explaining the delivery truth.
    pub note: String,
}

impl FanoutReceipt {
    /// Whether this copy was delivered as a current mirror.
    pub fn is_delivered(&self) -> bool {
        self.delivery_state.is_delivered()
    }

    /// Whether this copy is a labeled delivery gap (stale, undelivered, or unknown).
    pub fn is_delivery_gap(&self) -> bool {
        self.delivery_state.is_delivery_gap()
    }
}

/// The full set of receipts for one source fanned out across every governed destination
/// under one named condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutDispatch {
    /// Stable, namespaced dispatch id.
    pub dispatch_id: String,
    /// The source notification-envelope id.
    pub source_envelope_id: String,
    /// The canonical event id.
    pub canonical_event_id: String,
    /// The condition that produced the per-destination attempts.
    pub condition: FanoutConditionClass,
    /// The source subsystem.
    pub source_subsystem: SourceSubsystemClass,
    /// The severity of the source.
    pub severity: NotificationSeverityClass,
    /// One receipt per governed destination, in canonical order.
    pub receipts: Vec<FanoutReceipt>,
    /// The number of destinations the copy was delivered to.
    pub delivered_count: usize,
    /// The number of destinations whose copy is stale.
    pub stale_count: usize,
    /// The number of destinations whose copy was undelivered.
    pub undelivered_count: usize,
    /// The number of destinations whose copy was suppressed by policy.
    pub suppressed_count: usize,
    /// The number of destinations whose copy is in an unknown state.
    pub unknown_count: usize,
    /// Whether every non-delivered destination is labeled with an explicit reason rather
    /// than counted as delivered (always true).
    pub all_failures_labeled: bool,
    /// Whether the underlying authoritative record is still present regardless of the fanout
    /// outcome (always true).
    pub durable_record_present: bool,
}

impl FanoutDispatch {
    /// The receipt for a destination, if present.
    pub fn receipt(&self, destination: FanoutChannelClass) -> Option<&FanoutReceipt> {
        self.receipts.iter().find(|r| r.destination == destination)
    }

    /// The destinations this dispatch delivered the copy to.
    pub fn delivered_destinations(&self) -> Vec<FanoutChannelClass> {
        self.receipts
            .iter()
            .filter(|r| r.is_delivered())
            .map(|r| r.destination)
            .collect()
    }

    /// The destinations this dispatch labeled as a delivery gap (stale, undelivered, or
    /// unknown).
    pub fn gap_destinations(&self) -> Vec<FanoutChannelClass> {
        self.receipts
            .iter()
            .filter(|r| r.is_delivery_gap())
            .map(|r| r.destination)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// The minting engine.
// ---------------------------------------------------------------------------

/// Mints one [`FanoutReceipt`] for one destination, deterministically.
///
/// Pure: the same `(source, attempt)` yields the same receipt every call, so a receipt is
/// reproducible in support export and CLI/headless diagnostics. A non-compliant managed
/// endpoint is recorded as undelivered (the payload never leaves the device boundary); a
/// stale or failed copy is labeled with an explicit reason rather than counted as delivered;
/// every rendered copy uses a privacy-safe posture that never widens privacy below the
/// source floor and is reduced to a lock-screen-safe affordance on a locked screen; and the
/// reopen route and preview/approval posture are copied from the source so an external alert
/// can never bypass a guardrail or land on a generic shell.
pub fn mint_receipt(source: &FanoutSource, attempt: &FanoutAttempt) -> FanoutReceipt {
    let delivery_state = delivery_state(attempt);
    let reason = stale_undelivered_reason(delivery_state, attempt);
    let suppression_reason = suppression_reason(delivery_state);
    let posture = summary_posture(source, attempt, delivery_state);
    let applied_redaction = posture.redaction();
    let inline_action_allowed = !source.routes_through_preview_approval;
    let note = receipt_note(source, attempt, delivery_state, reason, posture);

    FanoutReceipt {
        receipt_id: format!("m5-fanout-receipts:receipt:{}", attempt.attempt_id),
        source_envelope_id: source.source_envelope_id.clone(),
        canonical_event_id: source.canonical_event_id.clone(),
        destination: attempt.destination,
        destination_id: attempt.destination.channel_id(),
        client_scope: attempt.client_scope.clone(),
        delivery_state,
        delivery_state_token: delivery_state.as_str().to_owned(),
        resulting_state: delivery_state.matrix_state(),
        stale_or_undelivered_reason: reason,
        stale_or_undelivered_reason_token: reason.as_str().to_owned(),
        suppression_reason,
        summary_posture: posture,
        summary_posture_token: posture.as_str().to_owned(),
        applied_redaction,
        redaction_token: redaction_token(applied_redaction).to_owned(),
        dedupe_key_scheme: source.dedupe_key_scheme,
        reopen_target: source.reopen_target,
        reopen_anchor_ref: source.reopen_anchor_ref.clone(),
        reopen_is_exact: !source.reopen_anchor_ref.is_empty(),
        routes_through_preview_approval: source.routes_through_preview_approval,
        inline_action_allowed,
        durable_record_present: source.carries_durable_record,
        minted_at: source.created_at.clone(),
        note,
    }
}

/// Mints one receipt per governed destination for a source under a named condition.
///
/// Deterministic: the same `(source, condition)` yields the same [`FanoutDispatch`] every
/// call. The per-destination transport context is fixed by the condition, so an inconsistent
/// edit changes the receipts rather than silently passing.
pub fn mint_dispatch(source: &FanoutSource, condition: FanoutConditionClass) -> FanoutDispatch {
    let receipts: Vec<FanoutReceipt> = GOVERNED_DESTINATIONS
        .iter()
        .map(|destination| {
            let attempt = condition_attempt(source, condition, *destination);
            mint_receipt(source, &attempt)
        })
        .collect();

    let count = |state: FanoutDeliveryStateClass| {
        receipts
            .iter()
            .filter(|r| r.delivery_state == state)
            .count()
    };
    let delivered_count = count(FanoutDeliveryStateClass::Delivered);
    let stale_count = count(FanoutDeliveryStateClass::Stale);
    let undelivered_count = count(FanoutDeliveryStateClass::Undelivered);
    let suppressed_count = count(FanoutDeliveryStateClass::Suppressed);
    let unknown_count = count(FanoutDeliveryStateClass::Unknown);

    // Every non-delivered receipt is labeled: a gap carries a named reason, a suppression
    // carries a named suppression reason. No failure is counted as delivered.
    let all_failures_labeled = receipts.iter().all(|r| match r.delivery_state {
        FanoutDeliveryStateClass::Delivered => {
            !r.stale_or_undelivered_reason.is_named() && !r.suppression_reason.is_named()
        }
        FanoutDeliveryStateClass::Suppressed => r.suppression_reason.is_named(),
        _ => r.stale_or_undelivered_reason.is_named(),
    });

    FanoutDispatch {
        dispatch_id: format!(
            "m5-fanout-receipts:dispatch:{}:{}",
            source.source_envelope_id,
            condition.as_str()
        ),
        source_envelope_id: source.source_envelope_id.clone(),
        canonical_event_id: source.canonical_event_id.clone(),
        condition,
        source_subsystem: source.source_subsystem,
        severity: source.severity,
        receipts,
        delivered_count,
        stale_count,
        undelivered_count,
        suppressed_count,
        unknown_count,
        all_failures_labeled,
        durable_record_present: source.carries_durable_record,
    }
}

fn delivery_state(attempt: &FanoutAttempt) -> FanoutDeliveryStateClass {
    // A non-compliant managed endpoint never receives the payload, regardless of what the
    // transport reported.
    if attempt.managed_endpoint_noncompliant {
        return FanoutDeliveryStateClass::Undelivered;
    }
    match attempt.transport {
        TransportSignalClass::Acknowledged => FanoutDeliveryStateClass::Delivered,
        TransportSignalClass::Superseded => FanoutDeliveryStateClass::Stale,
        TransportSignalClass::Unreachable | TransportSignalClass::TimedOut => {
            FanoutDeliveryStateClass::Undelivered
        }
        TransportSignalClass::PolicyWithheld => FanoutDeliveryStateClass::Suppressed,
        TransportSignalClass::Indeterminate => FanoutDeliveryStateClass::Unknown,
    }
}

fn stale_undelivered_reason(
    state: FanoutDeliveryStateClass,
    attempt: &FanoutAttempt,
) -> StaleUndeliveredReasonClass {
    match state {
        FanoutDeliveryStateClass::Delivered | FanoutDeliveryStateClass::Suppressed => {
            StaleUndeliveredReasonClass::None
        }
        FanoutDeliveryStateClass::Stale => StaleUndeliveredReasonClass::SupersededByNewerState,
        FanoutDeliveryStateClass::Unknown => StaleUndeliveredReasonClass::TransportIndeterminate,
        FanoutDeliveryStateClass::Undelivered => {
            if attempt.managed_endpoint_noncompliant {
                StaleUndeliveredReasonClass::ManagedEndpointBlocked
            } else {
                match attempt.transport {
                    TransportSignalClass::TimedOut => StaleUndeliveredReasonClass::DeliveryTimedOut,
                    _ => StaleUndeliveredReasonClass::ClientUnreachable,
                }
            }
        }
    }
}

fn suppression_reason(state: FanoutDeliveryStateClass) -> FanoutSuppressionReasonClass {
    match state {
        FanoutDeliveryStateClass::Suppressed => FanoutSuppressionReasonClass::QuietHoursPolicy,
        _ => FanoutSuppressionReasonClass::None,
    }
}

fn summary_posture(
    source: &FanoutSource,
    attempt: &FanoutAttempt,
    state: FanoutDeliveryStateClass,
) -> FanoutSummaryPostureClass {
    // A copy that was not rendered shows nothing.
    if !state.is_rendered() {
        return FanoutSummaryPostureClass::NoSummary;
    }
    // On a locked screen, an above-summary-safe copy is reduced to a count-only affordance.
    if attempt.lock_screen_locked
        && privacy_rank(source.privacy_class) > privacy_rank(NotificationPrivacyClass::SummarySafe)
    {
        return FanoutSummaryPostureClass::LockScreenSafe;
    }
    source.base_posture()
}

fn receipt_note(
    source: &FanoutSource,
    attempt: &FanoutAttempt,
    state: FanoutDeliveryStateClass,
    reason: StaleUndeliveredReasonClass,
    posture: FanoutSummaryPostureClass,
) -> String {
    let dest = attempt.destination.label();
    match state {
        FanoutDeliveryStateClass::Delivered => format!(
            "The {} mirrored {} with a {} posture; it reopens the source's {} and never widens \
             privacy.",
            dest,
            source.label,
            posture.as_str(),
            source.reopen_target.as_str(),
        ),
        FanoutDeliveryStateClass::Stale => format!(
            "The {} copy of {} is stale ({}); it is labeled rather than counted as delivered and \
             still reopens the source's {}.",
            dest,
            source.label,
            reason.as_str(),
            source.reopen_target.as_str(),
        ),
        FanoutDeliveryStateClass::Undelivered => format!(
            "The {} copy of {} was not delivered ({}); the failure is labeled, not counted as \
             delivered, and the durable in-product record still holds the attention.",
            dest,
            source.label,
            reason.as_str(),
        ),
        FanoutDeliveryStateClass::Suppressed => format!(
            "The {} copy of {} was withheld by policy; suppression is recorded separately from a \
             transport failure and the durable in-product record still holds the attention.",
            dest, source.label,
        ),
        FanoutDeliveryStateClass::Unknown => format!(
            "The {} copy of {} is in an unknown transport state ({}); it requires review and is \
             never counted as delivered.",
            dest,
            source.label,
            reason.as_str(),
        ),
    }
}

/// The per-destination attempt a condition produces for a destination.
fn condition_attempt(
    source: &FanoutSource,
    condition: FanoutConditionClass,
    destination: FanoutChannelClass,
) -> FanoutAttempt {
    use FanoutChannelClass::*;
    use FanoutConditionClass as C;
    use TransportSignalClass as T;

    let (transport, lock, managed) = match condition {
        C::AllDelivered => (T::Acknowledged, false, false),
        C::MobileStale => match destination {
            MobileCompanion => (T::Superseded, false, false),
            _ => (T::Acknowledged, false, false),
        },
        C::CompanionUndelivered => match destination {
            BrowserCompanion => (T::Unreachable, false, false),
            _ => (T::Acknowledged, false, false),
        },
        C::OsTimedOut => match destination {
            OsNativeNotification => (T::TimedOut, false, false),
            _ => (T::Acknowledged, false, false),
        },
        C::LockedScreen => (T::Acknowledged, true, false),
        C::ManagedEndpointBlocked => (T::Unreachable, false, true),
        C::PolicyWithheld => (T::PolicyWithheld, false, false),
        C::TransportUnknown => match destination {
            OsNativeNotification => (T::Indeterminate, false, false),
            _ => (T::Acknowledged, false, false),
        },
    };

    FanoutAttempt {
        attempt_id: format!(
            "{}:{}:{}",
            source.source_envelope_id,
            condition.as_str(),
            destination.as_str()
        ),
        destination,
        client_scope: client_scope(destination),
        transport,
        lock_screen_locked: lock,
        managed_endpoint_noncompliant: managed,
    }
}

/// A redaction-safe token naming the destination client class.
fn client_scope(destination: FanoutChannelClass) -> String {
    use FanoutChannelClass::*;
    match destination {
        OsNativeNotification => "os_primary_endpoint",
        BrowserCompanion => "browser_companion_session",
        MobileCompanion => "mobile_companion_device",
        InAppActivityCenter => "in_app_activity_center",
        DockTaskbarBadge => "dock_taskbar_badge",
        OperatorDashboard => "operator_dashboard",
    }
    .to_owned()
}

// ---------------------------------------------------------------------------
// Redaction / privacy helpers (the matrix enums carry no `as_str`).
// ---------------------------------------------------------------------------

fn redaction_rank(r: AttentionRedactionClass) -> u8 {
    use AttentionRedactionClass::*;
    match r {
        MetadataSafeDefault => 1,
        SummaryOnly => 2,
        RedactedPayload => 3,
        CountOnly => 4,
        InternalSupportRestricted => 5,
    }
}

/// The token for a redaction class.
fn redaction_token(r: AttentionRedactionClass) -> &'static str {
    use AttentionRedactionClass::*;
    match r {
        MetadataSafeDefault => "metadata_safe_default",
        SummaryOnly => "summary_only",
        RedactedPayload => "redacted_payload",
        CountOnly => "count_only",
        InternalSupportRestricted => "internal_support_restricted",
    }
}

fn privacy_rank(p: NotificationPrivacyClass) -> u8 {
    use NotificationPrivacyClass::*;
    match p {
        SummarySafe => 1,
        WorkspaceSensitive => 2,
        SecurityCritical => 3,
        ManagedSensitive => 4,
    }
}

// ---------------------------------------------------------------------------
// Governed-destination registry and bundle record.
// ---------------------------------------------------------------------------

/// One governed-destination entry: a cross-client surface this lane mints receipts for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedDestinationEntry {
    /// The destination channel.
    pub destination: FanoutChannelClass,
    /// Stable destination channel id.
    pub destination_id: String,
    /// Human-readable label.
    pub label: String,
    /// The redaction-safe client-scope token.
    pub client_scope: String,
    /// Whether this destination holds the authoritative durable record (always false — the
    /// destinations are out-of-window mirrors).
    pub is_durable_authoritative: bool,
    /// One reviewable sentence describing how this lane treats the destination.
    pub note: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutReceiptsInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen fanout-receipts bundle: the governed destinations, the source corpus, the
/// condition corpus, and every dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanoutReceiptsBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_fanout_receipts_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix this bundle binds its vocabulary back to.
    pub matrix_ref: String,
    /// The matrix id the bundle binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps the bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the bundle.
    pub summary: String,
    /// The governed destinations.
    pub governed_destinations: Vec<GovernedDestinationEntry>,
    /// The representative source corpus.
    pub sources: Vec<FanoutSource>,
    /// The representative transport-condition corpus.
    pub conditions: Vec<FanoutConditionClass>,
    /// Every dispatch (each source under each condition).
    pub dispatches: Vec<FanoutDispatch>,
    /// The computed invariants.
    pub invariants: Vec<FanoutReceiptsInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanoutReceiptsValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for FanoutReceiptsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fanout-receipts bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for FanoutReceiptsValidationError {}

impl FanoutReceiptsBundle {
    /// The source with a given id, if present.
    pub fn source(&self, source_envelope_id: &str) -> Option<&FanoutSource> {
        self.sources
            .iter()
            .find(|s| s.source_envelope_id == source_envelope_id)
    }

    /// The dispatch for a `(source, condition)` pair, if present.
    pub fn dispatch(
        &self,
        source_envelope_id: &str,
        condition: FanoutConditionClass,
    ) -> Option<&FanoutDispatch> {
        self.dispatches
            .iter()
            .find(|d| d.source_envelope_id == source_envelope_id && d.condition == condition)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are excluded
    /// and every ref is a repo-relative object ref or opaque `aureline://` handle, never a
    /// URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let fixed = [
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
            self.schema_ref.as_str(),
        ]
        .into_iter();
        let from_sources = self.sources.iter().map(|s| s.reopen_anchor_ref.as_str());
        let from_receipts = self
            .dispatches
            .iter()
            .flat_map(|d| d.receipts.iter().map(|r| r.reopen_anchor_ref.as_str()));
        fixed.chain(from_sources).chain(from_receipts)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), FanoutReceiptsValidationError> {
        let fail = |reason: String| Err(FanoutReceiptsValidationError { reason });

        if self.record_kind != M5_FANOUT_RECEIPTS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_FANOUT_RECEIPTS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.sources.is_empty() || self.conditions.is_empty() || self.dispatches.is_empty() {
            return fail("sources, conditions, and dispatches must be non-empty".to_owned());
        }

        // The governed destinations are exactly the three this lane mints receipts for.
        if self.governed_destinations.len() != GOVERNED_DESTINATIONS.len()
            || !GOVERNED_DESTINATIONS.iter().all(|d| {
                self.governed_destinations
                    .iter()
                    .any(|e| e.destination == *d)
            })
        {
            return fail(
                "governed destinations must be exactly the three governed destinations".to_owned(),
            );
        }

        // Stable ids are unique.
        if !all_unique(self.sources.iter().map(|s| s.source_envelope_id.as_str())) {
            return fail("source ids are not unique".to_owned());
        }
        if !all_unique(self.dispatches.iter().map(|d| d.dispatch_id.as_str())) {
            return fail("dispatch ids are not unique".to_owned());
        }

        // Every source carries a durable record and an exact reopen anchor.
        for source in &self.sources {
            if !source.carries_durable_record {
                return fail(format!(
                    "source {} must carry a durable record",
                    source.source_envelope_id
                ));
            }
            if source.reopen_anchor_ref.is_empty() {
                return fail(format!(
                    "source {} is missing its reopen anchor",
                    source.source_envelope_id
                ));
            }
            if source.canonical_event_id.is_empty() {
                return fail(format!(
                    "source {} is missing its canonical event id",
                    source.source_envelope_id
                ));
            }
        }

        // Every dispatch references a known source and reproduces identically.
        for dispatch in &self.dispatches {
            let Some(source) = self.source(&dispatch.source_envelope_id) else {
                return fail(format!(
                    "dispatch {} references unknown source {}",
                    dispatch.dispatch_id, dispatch.source_envelope_id
                ));
            };
            if &mint_dispatch(source, dispatch.condition) != dispatch {
                return fail(format!(
                    "dispatch {} is not reproducible from its source and condition",
                    dispatch.dispatch_id
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical fanout-receipts bundle.
///
/// Deterministic: the same bytes every call. The governed destinations, source corpus, and
/// condition corpus are fixed, every dispatch is minted by [`mint_dispatch`], and each
/// invariant's `holds` flag is computed from the built data, so an inconsistent edit flips
/// an invariant rather than silently passing.
pub fn fanout_receipts_bundle() -> FanoutReceiptsBundle {
    let governed_destinations = build_governed_destinations();
    let sources = build_sources();
    let conditions = FanoutConditionClass::ALL.to_vec();
    let dispatches = build_dispatches(&sources, &conditions);
    let invariants = compute_invariants(&governed_destinations, &sources, &dispatches);

    FanoutReceiptsBundle {
        record_kind: M5_FANOUT_RECEIPTS_RECORD_KIND.to_owned(),
        m5_fanout_receipts_schema_version: M5_FANOUT_RECEIPTS_SCHEMA_VERSION,
        schema_ref: M5_FANOUT_RECEIPTS_SCHEMA_REF.to_owned(),
        bundle_id: M5_FANOUT_RECEIPTS_BUNDLE_ID.to_owned(),
        as_of: M5_FANOUT_RECEIPTS_AS_OF.to_owned(),
        matrix_ref: M5_FANOUT_RECEIPTS_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_FANOUT_RECEIPTS_FREEZE_GATE_REF.to_owned(),
        summary: "One durable, privacy-safe fanout receipt per cross-client destination — the \
                  native OS notification and the browser and mobile companions — minted when an \
                  attention object is fanned out. Each receipt names its source notification, the \
                  destination client class, the delivery state, an explicit stale or undelivered \
                  reason, and a privacy-safe summary posture; a failed or stale copy is labeled \
                  rather than counted as delivered; every rendered copy uses a privacy-safe posture \
                  that never widens privacy and is lock-screen-safe on a locked screen; a \
                  non-compliant managed endpoint never receives the payload; suppression-by-policy is \
                  kept distinct from a transport failure; the authoritative in-product record \
                  survives any fanout outcome; and every copy reopens the same authoritative object \
                  by an exact anchor and never executes a preview/approval action inline."
            .to_owned(),
        governed_destinations,
        sources,
        conditions,
        dispatches,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_governed_destinations() -> Vec<GovernedDestinationEntry> {
    GOVERNED_DESTINATIONS
        .iter()
        .map(|destination| GovernedDestinationEntry {
            destination: *destination,
            destination_id: destination.channel_id(),
            label: destination.label().to_owned(),
            client_scope: client_scope(*destination),
            is_durable_authoritative: false,
            note: format!(
                "An out-of-window mirror; each fanout copy to the {} is recorded with its delivery \
                 state and a privacy-safe summary posture, and reopens the source's authoritative \
                 object.",
                destination.label(),
            ),
        })
        .collect()
}

fn build_dispatches(
    sources: &[FanoutSource],
    conditions: &[FanoutConditionClass],
) -> Vec<FanoutDispatch> {
    let mut out = Vec::new();
    for source in sources {
        for condition in conditions {
            out.push(mint_dispatch(source, *condition));
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn source(
    slug: &str,
    label: &str,
    source_subsystem: SourceSubsystemClass,
    severity: NotificationSeverityClass,
    scope: AttentionScopeClass,
    privacy_class: NotificationPrivacyClass,
    dedupe_key_scheme: DedupeStrategyClass,
    reopen_target: ReopenTargetClass,
    routes_through_preview_approval: bool,
) -> FanoutSource {
    FanoutSource {
        source_envelope_id: format!("notification_envelope:{slug}:0001"),
        canonical_event_id: format!("canonical_event:{slug}:0001"),
        label: label.to_owned(),
        source_subsystem,
        severity,
        scope,
        privacy_class,
        dedupe_key_scheme,
        reopen_target,
        reopen_anchor_ref: format!("aureline://object/{slug}/0001"),
        routes_through_preview_approval,
        carries_durable_record: true,
        created_at: M5_FANOUT_RECEIPTS_AS_OF.to_owned(),
    }
}

fn build_sources() -> Vec<FanoutSource> {
    use AttentionScopeClass as Sc;
    use DedupeStrategyClass as D;
    use NotificationPrivacyClass as P;
    use NotificationSeverityClass as Sev;
    use ReopenTargetClass as R;
    use SourceSubsystemClass as S;

    vec![
        source(
            "task.completed",
            "Task run completed",
            S::TaskRunner,
            Sev::MinorSuccess,
            Sc::Session,
            P::SummarySafe,
            D::CanonicalKeyCoalesce,
            R::ActivityJobRow,
            false,
        ),
        source(
            "ai.awaiting_approval",
            "AI change awaiting approval",
            S::Ai,
            Sev::HandoffActionable,
            Sc::Session,
            P::WorkspaceSensitive,
            D::LatestSupersedes,
            R::ReviewRequest,
            // Acting on this attention must route through the in-product preview/approval
            // flow; no out-of-window copy may execute it inline.
            true,
        ),
        source(
            "incident.flagged",
            "Incident flagged for review",
            S::Incident,
            Sev::HandoffActionable,
            Sc::Collaboration,
            P::WorkspaceSensitive,
            D::RootCauseCollapse,
            R::IncidentThread,
            false,
        ),
        source(
            "route.policy_warning",
            "Managed route warning",
            S::ManagedPolicy,
            Sev::HandoffActionable,
            Sc::TenantOrg,
            P::ManagedSensitive,
            D::LatestSupersedes,
            R::PolicyDiff,
            // A managed policy change routes through preview/approval.
            true,
        ),
        source(
            "security.credential_revoked",
            "Security credential revoked",
            S::Security,
            Sev::SecurityAdvisory,
            Sc::AppGlobal,
            P::SecurityCritical,
            D::CountRollup,
            R::AuditEvent,
            false,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> FanoutReceiptsInvariant {
    FanoutReceiptsInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn source_by_id<'a>(sources: &'a [FanoutSource], id: &str) -> Option<&'a FanoutSource> {
    sources.iter().find(|s| s.source_envelope_id == id)
}

fn compute_invariants(
    governed_destinations: &[GovernedDestinationEntry],
    sources: &[FanoutSource],
    dispatches: &[FanoutDispatch],
) -> Vec<FanoutReceiptsInvariant> {
    let matrix = attention_routing_matrix();
    let mut out = Vec::new();

    // One receipt per governed destination.
    out.push(invariant(
        "fanout.receipt_per_destination",
        "Every dispatch mints exactly one receipt per governed destination — the native OS \
         notification and the browser and mobile companions — so cross-client delivery truth is \
         per-destination, not a single best-effort flag.",
        governed_destinations.len() == GOVERNED_DESTINATIONS.len()
            && dispatches.iter().all(|d| {
                d.receipts.len() == GOVERNED_DESTINATIONS.len()
                    && GOVERNED_DESTINATIONS
                        .iter()
                        .all(|dest| d.receipts.iter().any(|r| r.destination == *dest))
            }),
    ));

    // Every receipt binds to its source notification and canonical event.
    out.push(invariant(
        "fanout.binds_source_and_canonical_event",
        "Every receipt carries the source notification-envelope id and the canonical event id of \
         its dispatch, so a fanout copy always ties back to the authoritative notification/activity \
         object.",
        dispatches.iter().all(|d| {
            d.receipts.iter().all(|r| {
                r.source_envelope_id == d.source_envelope_id
                    && r.canonical_event_id == d.canonical_event_id
                    && !r.source_envelope_id.is_empty()
                    && !r.canonical_event_id.is_empty()
            })
        }),
    ));

    // No silent success: failures are labeled, never counted as delivered.
    out.push(invariant(
        "fanout.failures_labeled_never_counted_delivered",
        "In every dispatch the delivered count equals the number of receipts in the delivered \
         state, every non-delivered receipt is labeled with an explicit reason, and \
         all_failures_labeled holds — so a stale or failed fanout is visible truth, never counted \
         as delivered.",
        dispatches.iter().all(|d| {
            let delivered = d.receipts.iter().filter(|r| r.is_delivered()).count();
            d.all_failures_labeled
                && d.delivered_count == delivered
                && d.delivered_count == d.delivered_destinations().len()
        }),
    ));

    // Stale/undelivered/unknown receipts carry an explicit reason and a reviewable note.
    out.push(invariant(
        "fanout.stale_undelivered_have_reason",
        "Every stale, undelivered, or unknown receipt carries a named stale/undelivered reason and \
         a non-empty note, and every delivered receipt carries no such reason, so a delivery gap \
         can always explain itself.",
        dispatches.iter().all(|d| {
            d.receipts.iter().all(|r| {
                if !r.note.is_empty()
                    && r.delivery_state_token == r.delivery_state.as_str()
                    && r.stale_or_undelivered_reason_token == r.stale_or_undelivered_reason.as_str()
                {
                    match r.delivery_state {
                        FanoutDeliveryStateClass::Delivered
                        | FanoutDeliveryStateClass::Suppressed => {
                            !r.stale_or_undelivered_reason.is_named()
                        }
                        _ => r.stale_or_undelivered_reason.is_named(),
                    }
                } else {
                    false
                }
            })
        }),
    ));

    // Privacy-safe summaries by default; never widen privacy below the source floor.
    out.push(invariant(
        "fanout.privacy_safe_summary_default",
        "Every delivered or stale receipt uses a privacy-safe summary posture that never renders \
         the full payload and applies a redaction at least as strong as the source's privacy floor, \
         so a fanout copy never widens privacy.",
        dispatches.iter().all(|d| {
            let Some(source) = source_by_id(sources, &d.source_envelope_id) else {
                return false;
            };
            d.receipts
                .iter()
                .filter(|r| matches!(
                    r.delivery_state,
                    FanoutDeliveryStateClass::Delivered | FanoutDeliveryStateClass::Stale
                ))
                .all(|r| {
                    r.summary_posture != FanoutSummaryPostureClass::NoSummary
                        && r.applied_redaction == r.summary_posture.redaction()
                        && redaction_rank(r.applied_redaction)
                            >= redaction_rank(source.privacy_floor())
                })
        }),
    ));

    // Lock-screen-safe: above-summary-safe copies are count-only on a locked screen.
    out.push(invariant(
        "fanout.lock_screen_safe",
        "Under the locked-screen condition, every delivered or stale receipt for an \
         above-summary-safe source uses the lock-screen-safe posture (a count-only affordance), so \
         sensitive content is never rendered in the clear on a locked screen.",
        dispatches
            .iter()
            .filter(|d| d.condition == FanoutConditionClass::LockedScreen)
            .all(|d| {
                let Some(source) = source_by_id(sources, &d.source_envelope_id) else {
                    return false;
                };
                if privacy_rank(source.privacy_class)
                    <= privacy_rank(NotificationPrivacyClass::SummarySafe)
                {
                    return true;
                }
                d.receipts
                    .iter()
                    .filter(|r| {
                        matches!(
                            r.delivery_state,
                            FanoutDeliveryStateClass::Delivered | FanoutDeliveryStateClass::Stale
                        )
                    })
                    .all(|r| {
                        r.summary_posture.is_lock_screen_safe()
                            && r.applied_redaction == AttentionRedactionClass::CountOnly
                    })
            }),
    ));

    // Managed endpoints never receive the payload.
    out.push(invariant(
        "fanout.managed_endpoint_blocks_payload",
        "Under the managed-endpoint-blocked condition, every receipt is undelivered with the \
         managed-endpoint reason and renders no summary, so a non-compliant managed endpoint never \
         receives the payload.",
        dispatches
            .iter()
            .filter(|d| d.condition == FanoutConditionClass::ManagedEndpointBlocked)
            .all(|d| {
                d.receipts.iter().all(|r| {
                    r.delivery_state == FanoutDeliveryStateClass::Undelivered
                        && r.stale_or_undelivered_reason
                            == StaleUndeliveredReasonClass::ManagedEndpointBlocked
                        && r.summary_posture == FanoutSummaryPostureClass::NoSummary
                })
            }),
    ));

    // Reopen parity: same authoritative object, exact anchor, never a generic shell.
    out.push(invariant(
        "fanout.reopen_parity",
        "Every receipt reopens the same authoritative object the source names, by the source's \
         exact export-safe anchor, with reopen_is_exact set — so an external alert never lands on an \
         ambiguous generic shell when an exact reopen path exists.",
        dispatches.iter().all(|d| {
            let Some(source) = source_by_id(sources, &d.source_envelope_id) else {
                return false;
            };
            d.receipts.iter().all(|r| {
                r.reopen_target == source.reopen_target
                    && r.reopen_anchor_ref == source.reopen_anchor_ref
                    && r.reopen_is_exact
                    && !r.reopen_anchor_ref.is_empty()
                    && is_export_safe_ref(&r.reopen_anchor_ref)
            })
        }),
    ));

    // No preview/approval bypass.
    out.push(invariant(
        "fanout.no_preview_approval_bypass",
        "Every receipt copies the source's preview/approval posture, and a receipt whose source \
         routes through preview/approval may not execute the action inline — so an external alert \
         hands off to the in-product flow instead of bypassing the guardrail.",
        dispatches.iter().all(|d| {
            let Some(source) = source_by_id(sources, &d.source_envelope_id) else {
                return false;
            };
            d.receipts.iter().all(|r| {
                r.routes_through_preview_approval == source.routes_through_preview_approval
                    && r.inline_action_allowed != source.routes_through_preview_approval
                    && (!r.routes_through_preview_approval || !r.inline_action_allowed)
            })
        }),
    ));

    // The durable record survives any fanout outcome.
    out.push(invariant(
        "fanout.durable_record_present",
        "Every dispatch and every receipt keeps the authoritative in-product record present, so \
         even an all-undelivered or all-suppressed fanout never drops the durable object.",
        dispatches.iter().all(|d| {
            d.durable_record_present && d.receipts.iter().all(|r| r.durable_record_present)
        }),
    ));

    // Suppression is not a transport failure.
    out.push(invariant(
        "fanout.suppressed_separate_from_failure",
        "A suppressed receipt carries a named suppression reason, no stale/undelivered reason, and \
         is never counted in the undelivered total, so suppression-by-policy stays distinct from a \
         transport failure.",
        dispatches.iter().all(|d| {
            d.receipts
                .iter()
                .filter(|r| r.delivery_state == FanoutDeliveryStateClass::Suppressed)
                .all(|r| {
                    r.suppression_reason.is_named()
                        && !r.stale_or_undelivered_reason.is_named()
                })
                && d.undelivered_count
                    == d.receipts
                        .iter()
                        .filter(|r| r.delivery_state == FanoutDeliveryStateClass::Undelivered)
                        .count()
        }),
    ));

    // Every delivery state is exercised.
    out.push(invariant(
        "fanout.every_state_exercised",
        "The corpus produces every delivery state — delivered, stale, undelivered, suppressed, and \
         unknown — so the no-silent-success delivery model is real, not nominal.",
        FanoutDeliveryStateClass::ALL.iter().all(|state| {
            dispatches
                .iter()
                .any(|d| d.receipts.iter().any(|r| r.delivery_state == *state))
        }),
    ));

    // Every summary posture is exercised.
    out.push(invariant(
        "fanout.every_posture_exercised",
        "The corpus produces every summary posture — clear summary, redacted summary, \
         lock-screen-safe, open-app-only, and no summary — so the privacy-safe rendering model is \
         exercised end to end.",
        FanoutSummaryPostureClass::ALL.iter().all(|posture| {
            dispatches
                .iter()
                .any(|d| d.receipts.iter().any(|r| r.summary_posture == *posture))
        }),
    ));

    // Every dispatch is reproducible.
    out.push(invariant(
        "fanout.dispatches_reproducible",
        "Re-minting every dispatch from its source and condition yields an identical dispatch, so a \
         fanout receipt is reproducible in support export and diagnostics.",
        dispatches.iter().all(|d| match source_by_id(sources, &d.source_envelope_id) {
            Some(s) => &mint_dispatch(s, d.condition) == d,
            None => false,
        }),
    ));

    // Every token binds back to the attention-routing matrix.
    out.push(invariant(
        "fanout.matrix_bound",
        "Every privacy class, scope, redaction class, reopen target, severity, dedupe scheme, and \
         resulting state the bundle uses is one the attention-routing matrix defines, and the \
         fanout-receipt object can show the stale and undelivered states, so the fanout path never \
         drifts from the frozen object model.",
        matrix_bound_holds(sources, dispatches, &matrix),
    ));

    // Every reference is support-export safe with no raw text.
    out.push(invariant(
        "fanout.support_export_safe",
        "Every source reopen anchor and every receipt reopen anchor is a repo-relative object ref \
         or opaque aureline:// handle, never a URL, host, credential, message body, device id, or \
         absolute path.",
        sources
            .iter()
            .all(|s| is_export_safe_ref(&s.reopen_anchor_ref))
            && dispatches.iter().all(|d| {
                d.receipts
                    .iter()
                    .all(|r| is_export_safe_ref(&r.reopen_anchor_ref))
            }),
    ));

    out
}

fn matrix_bound_holds(
    sources: &[FanoutSource],
    dispatches: &[FanoutDispatch],
    matrix: &AttentionRoutingMatrix,
) -> bool {
    let tokens = |defs: &[crate::m5_attention_routing::AttentionTokenDef]| -> Vec<String> {
        defs.iter().map(|t| t.token.clone()).collect()
    };
    let privacy_tokens = tokens(&matrix.shared_vocabulary.privacy_classes);
    let scope_tokens = tokens(&matrix.shared_vocabulary.scopes);
    let redaction_tokens = tokens(&matrix.shared_vocabulary.redaction_classes);
    let reopen_tokens = tokens(&matrix.shared_vocabulary.reopen_targets);
    let severity_tokens = tokens(&matrix.shared_vocabulary.severities);
    let dedupe_tokens = tokens(&matrix.shared_vocabulary.dedupe_rules);
    let has = |list: &[String], token: &str| list.iter().any(|t| t == token);

    let sources_bound = sources.iter().all(|s| {
        has(&privacy_tokens, s.privacy_class.as_str())
            && has(&scope_tokens, s.scope.as_str())
            && has(&reopen_tokens, s.reopen_target.as_str())
            && has(&severity_tokens, s.severity.as_str())
            && has(&dedupe_tokens, s.dedupe_key_scheme.as_str())
    });

    let receipts_bound = dispatches.iter().all(|d| {
        d.receipts.iter().all(|r| {
            has(&redaction_tokens, redaction_token(r.applied_redaction))
                && has(&reopen_tokens, r.reopen_target.as_str())
                && has(&dedupe_tokens, r.dedupe_key_scheme.as_str())
        })
    });

    let fanout_object = matrix.object(AttentionObjectClass::FanoutReceipt);
    let states_bound = fanout_object.is_some_and(|o| {
        o.can_show(AttentionStateClass::FanoutStale)
            && o.can_show(AttentionStateClass::FanoutUndelivered)
            && o.can_show(AttentionStateClass::Shown)
            && o.can_show(AttentionStateClass::Suppressed)
            && o.can_show(AttentionStateClass::UnknownRequiresReview)
    });

    sources_bound && receipts_bound && states_bound
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn fanout_receipts_lines(bundle: &FanoutReceiptsBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Fanout-receipts bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Destinations: {}  Sources: {}  Conditions: {}  Dispatches: {}  Invariants: {}",
        bundle.governed_destinations.len(),
        bundle.sources.len(),
        bundle.conditions.len(),
        bundle.dispatches.len(),
        bundle.invariants.len(),
    ));

    lines.push("Governed destinations:".to_owned());
    for d in &bundle.governed_destinations {
        lines.push(format!(
            "  - {} [{}] client_scope={} durable={}",
            d.label, d.destination_id, d.client_scope, d.is_durable_authoritative,
        ));
    }

    lines.push("Sources:".to_owned());
    for s in &bundle.sources {
        lines.push(format!(
            "  - {} [{}] severity={} privacy={} scope={} reopen={} preview_approval={}",
            s.label,
            s.source_envelope_id,
            s.severity.as_str(),
            s.privacy_class.as_str(),
            s.scope.as_str(),
            s.reopen_target.as_str(),
            s.routes_through_preview_approval,
        ));
    }

    lines.push("Dispatches:".to_owned());
    for d in &bundle.dispatches {
        let receipts: Vec<String> = d
            .receipts
            .iter()
            .map(|r| {
                format!(
                    "{}={}/{}/{}",
                    r.destination.as_str(),
                    r.delivery_state.as_str(),
                    r.stale_or_undelivered_reason.as_str(),
                    r.summary_posture.as_str(),
                )
            })
            .collect();
        lines.push(format!(
            "  - {} [{}] -> [{}] delivered={} stale={} undelivered={} suppressed={} unknown={}",
            d.source_envelope_id,
            d.condition.as_str(),
            receipts.join(", "),
            d.delivered_count,
            d.stale_count,
            d.undelivered_count,
            d.suppressed_count,
            d.unknown_count,
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
