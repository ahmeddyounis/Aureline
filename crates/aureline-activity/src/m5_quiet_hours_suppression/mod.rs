//! M5 *quiet-hours and suppression routing*: the working engine that applies one
//! coherent suppression policy — quiet-hours, do-not-disturb, presentation/follow,
//! admin suppression, lock-screen privacy, and managed-endpoint posture — across the
//! in-app activity center, OS notification, and companion attention surfaces, and
//! explains for every surface whether the event was shown, downgraded, or withheld.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes the
//! contract* — the attention object families, the shared state vocabulary, and the
//! fanout channels — and [`m5_envelope_routing`](crate::m5_envelope_routing) *routes a
//! fresh envelope to its surfaces against a routing context*, this lane makes
//! quiet-hours and suppression a *first-class routing policy* rather than a
//! surface-local preference. The same [`SuppressionPolicy`] governs every governed
//! surface, so a person never sees one surface honor quiet-hours while another ignores
//! it, and no surface invents its own mute logic.
//!
//! [`evaluate_suppression`] is a pure function of an [`AttentionSignal`] (the
//! suppression-relevant projection of a notification envelope or durable activity
//! object) and a [`SuppressionPolicy`]. It returns a [`SuppressionDecision`] with one
//! [`SurfaceSuppressionOutcome`] per governed surface and a [`SuppressionLedgerEntry`]
//! for every out-of-window surface that withheld or downgraded the event — so the same
//! `(signal, policy)` yields the same decision byte-for-byte in support export and
//! CLI/headless diagnostics. The honesty rules the track invariant requires are
//! enforced, not just described:
//!
//! - **One policy across surfaces.** The in-app activity center, OS notification, and
//!   browser/mobile companions are all evaluated by the same engine against the same
//!   policy (`suppression.parity_one_policy_all_surfaces`).
//! - **The durable record is never dropped.** The in-app activity center always shows
//!   the durable authoritative record, independent of any suppression
//!   (`suppression.in_app_durable_record_always`).
//! - **Every surface explains itself.** Each outcome carries a stable suppression
//!   source token and a short reviewable reason — shown, downgraded, or withheld
//!   (`suppression.explains_every_surface`).
//! - **High-importance escapes only when named.** A high-importance security, trust,
//!   approval, or route warning escapes quiet-hours, do-not-disturb, and
//!   presentation/follow only when it explicitly names its scope and consequence;
//!   otherwise it is withheld out-of-window and kept durably in-product
//!   (`suppression.high_importance_escapes_only_when_named`).
//! - **A security advisory is never silenced.** It always shows in-app and escapes
//!   out-of-window with a redacted summary, never withheld on every surface
//!   (`suppression.security_never_silenced`).
//! - **Suppression stays separate from audit history.** Every withheld or downgraded
//!   out-of-window outcome records a suppression-ledger entry that is marked separate
//!   from audit history and never implies the underlying job or incident disappeared
//!   (`suppression.separate_from_audit_history`).
//! - **Blocked high-value events stay accountable.** Every withheld or downgraded
//!   high-importance event sets an audit-trail requirement and a ledger entry, so a
//!   muted event is still inspectable and exportable (`suppression.audit_trail_for_blocked_high_importance`).
//! - **Suppression never widens privacy.** A downgrade only ever raises redaction
//!   above the surface's normal treatment (`suppression.downgrade_never_widens_privacy`).
//!
//! The canonical [`quiet_hours_suppression_bundle`] freezes the governed surfaces, a
//! representative policy corpus, a representative signal corpus, and every decision so
//! the freeze gate and checked-in fixture pin the contract byte-for-byte. Every
//! privacy class, scope, redaction class, reopen target, severity, and suppression
//! state the bundle uses is one the attention-routing matrix defines, so the
//! suppression path can never drift from the frozen object model
//! (`suppression.matrix_bound`).
//!
//! The record carries no message bodies, credentials, raw provider payloads,
//! hostnames, or absolute paths — only opaque object refs, stable tokens, and short
//! reviewable sentences — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionObjectClass,
    AttentionRedactionClass, AttentionRoutingMatrix, AttentionScopeClass, AttentionStateClass,
    FanoutChannelClass, NotificationPrivacyClass, ReopenTargetClass,
    M5_ATTENTION_ROUTING_MATRIX_ID,
};
use crate::m5_envelope_routing::{
    AdminNotificationPolicyClass, DoNotDisturbClass, NotificationSeverityClass,
    PresentationModeClass, SourceSubsystemClass,
};

#[cfg(test)]
mod tests;

/// Schema version for the quiet-hours-suppression bundle.
pub const M5_QUIET_HOURS_SUPPRESSION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the quiet-hours-suppression bundle.
pub const M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF: &str =
    "schemas/activity/m5-quiet-hours-suppression.schema.json";

/// Stable record-kind tag for the quiet-hours-suppression bundle.
pub const M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND: &str = "m5_quiet_hours_suppression_bundle";

/// Stable id for the canonical quiet-hours-suppression bundle.
pub const M5_QUIET_HOURS_SUPPRESSION_BUNDLE_ID: &str = "m5-quiet-hours-suppression:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding stays
/// deterministic and the fixture freezes byte-for-byte.
pub const M5_QUIET_HOURS_SUPPRESSION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_QUIET_HOURS_SUPPRESSION_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate; it
/// fails when the in-code bundle drifts from the checked-in fixture or any invariant
/// flips.
pub const M5_QUIET_HOURS_SUPPRESSION_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_quiet_hours_suppression.rs";

/// The attention surfaces this suppression lane governs with one shared policy: the
/// durable in-app activity center plus the out-of-window OS and companion surfaces.
///
/// The dock/taskbar badge and the operator dashboard are governed by their own lanes
/// (badge aggregate and operator surface), so they are out of scope here; this lane
/// proves one policy across the in-app, OS, and companion surfaces the spec names.
pub const GOVERNED_SURFACES: [FanoutChannelClass; 4] = [
    FanoutChannelClass::InAppActivityCenter,
    FanoutChannelClass::OsNativeNotification,
    FanoutChannelClass::BrowserCompanion,
    FanoutChannelClass::MobileCompanion,
];

// ---------------------------------------------------------------------------
// Suppression-policy vocabulary.
// ---------------------------------------------------------------------------

/// Whether quiet-hours is currently active.
///
/// Quiet-hours is the scheduled, user-or-policy-defined window during which
/// out-of-window interruptions are deferred while the durable in-product record keeps
/// accruing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuietHoursModeClass {
    /// Quiet-hours is not active.
    Off,
    /// Quiet-hours is active; out-of-window fanout is deferred unless explicitly named.
    Active,
}

impl QuietHoursModeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Active => "active",
        }
    }

    /// Whether quiet-hours is active.
    const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// The lock-screen privacy posture.
///
/// On a locked screen, content above [`NotificationPrivacyClass::SummarySafe`] is hidden
/// behind a count-only affordance rather than rendered in the clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockScreenStateClass {
    /// The screen is unlocked; lock-screen privacy does not apply.
    Unlocked,
    /// The screen is locked; sensitive content is hidden behind a count-only affordance.
    Locked,
}

impl LockScreenStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Locked => "locked",
        }
    }

    /// Whether the screen is locked.
    const fn is_locked(self) -> bool {
        matches!(self, Self::Locked)
    }
}

/// The managed-endpoint compliance posture.
///
/// A non-compliant managed endpoint is not allowed to receive out-of-window payloads at
/// all; the attention stays in-product on its durable record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedEndpointPostureClass {
    /// The endpoint is unmanaged.
    Unmanaged,
    /// The endpoint is managed and compliant; out-of-window fanout is allowed.
    Compliant,
    /// The endpoint is managed and non-compliant; out-of-window fanout is withheld.
    NonCompliant,
}

impl ManagedEndpointPostureClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unmanaged => "unmanaged",
            Self::Compliant => "compliant",
            Self::NonCompliant => "non_compliant",
        }
    }
}

/// The reason a surface decision shows, downgrades, or withholds an event.
///
/// Each token names exactly one suppression input, so a surface can always explain *why*
/// the disposition it reached was reached. [`SuppressionSourceClass::None`] means no
/// suppression input applied — the surface showed the event at its normal treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionSourceClass {
    /// No suppression input applied.
    None,
    /// Deferred by an active quiet-hours window.
    QuietHours,
    /// Deferred by do-not-disturb.
    DoNotDisturb,
    /// Deferred by presentation / follow mode.
    PresentationFollow,
    /// Downgraded by lock-screen privacy.
    LockScreenPrivacy,
    /// Restricted or withheld by admin / managed suppression policy.
    AdminSuppression,
    /// Withheld because the managed endpoint is non-compliant.
    ManagedEndpointPolicy,
}

impl SuppressionSourceClass {
    /// All sources, in token order.
    pub const ALL: [Self; 7] = [
        Self::None,
        Self::QuietHours,
        Self::DoNotDisturb,
        Self::PresentationFollow,
        Self::LockScreenPrivacy,
        Self::AdminSuppression,
        Self::ManagedEndpointPolicy,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::QuietHours => "quiet_hours",
            Self::DoNotDisturb => "do_not_disturb",
            Self::PresentationFollow => "presentation_follow",
            Self::LockScreenPrivacy => "lock_screen_privacy",
            Self::AdminSuppression => "admin_suppression",
            Self::ManagedEndpointPolicy => "managed_endpoint_policy",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No suppression",
            Self::QuietHours => "Quiet-hours",
            Self::DoNotDisturb => "Do-not-disturb",
            Self::PresentationFollow => "Presentation / follow",
            Self::LockScreenPrivacy => "Lock-screen privacy",
            Self::AdminSuppression => "Admin suppression",
            Self::ManagedEndpointPolicy => "Managed-endpoint policy",
        }
    }

    /// Whether this source is one of the interruption-deferring postures (quiet-hours,
    /// do-not-disturb, or presentation/follow) that a named high-importance event may
    /// escape with a redacted summary.
    const fn is_interruption(self) -> bool {
        matches!(
            self,
            Self::QuietHours | Self::DoNotDisturb | Self::PresentationFollow
        )
    }
}

/// How a surface treated an event under the active policy.
///
/// The three dispositions are the explainability triad the spec requires: every surface
/// can say whether it **showed**, **downgraded**, or **withheld** the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionDispositionClass {
    /// Shown at the surface's normal treatment; no suppression input applied.
    Shown,
    /// Delivered with a raised redaction because a suppression input applied.
    Downgraded,
    /// Not delivered to this surface now; the durable in-product record is unaffected.
    Withheld,
}

impl SuppressionDispositionClass {
    /// All dispositions, in order.
    pub const ALL: [Self; 3] = [Self::Shown, Self::Downgraded, Self::Withheld];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shown => "shown",
            Self::Downgraded => "downgraded",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the event reached this surface in some form (shown or downgraded).
    pub const fn is_delivered(self) -> bool {
        matches!(self, Self::Shown | Self::Downgraded)
    }

    /// Whether the event was held back from this surface.
    pub const fn is_withheld(self) -> bool {
        matches!(self, Self::Withheld)
    }
}

/// The class of consequence a high-importance event explicitly names.
///
/// A high-importance event escapes quiet-hours, do-not-disturb, and presentation/follow
/// only when it carries a named consequence (anything other than
/// [`ConsequenceClass::None`]) together with a non-empty consequence note, so an
/// interruption is always justified by a named scope and consequence rather than raw
/// severity alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceClass {
    /// No explicitly named consequence; the event is routine and does not escape.
    None,
    /// A security advisory or revocation.
    SecurityAdvisory,
    /// A trust posture change (provider, credential, or attestation).
    TrustChange,
    /// An approval is required before work proceeds.
    ApprovalRequired,
    /// A routing or delivery warning that risks dropped or misrouted work.
    RouteWarning,
}

impl ConsequenceClass {
    /// All consequence classes, in order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::SecurityAdvisory,
        Self::TrustChange,
        Self::ApprovalRequired,
        Self::RouteWarning,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SecurityAdvisory => "security_advisory",
            Self::TrustChange => "trust_change",
            Self::ApprovalRequired => "approval_required",
            Self::RouteWarning => "route_warning",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SecurityAdvisory => "Security advisory",
            Self::TrustChange => "Trust change",
            Self::ApprovalRequired => "Approval required",
            Self::RouteWarning => "Route warning",
        }
    }

    /// Whether this is an explicitly named consequence (anything but
    /// [`ConsequenceClass::None`]).
    pub const fn is_named(self) -> bool {
        !matches!(self, Self::None)
    }
}

// ---------------------------------------------------------------------------
// Suppression policy.
// ---------------------------------------------------------------------------

/// The one coherent suppression policy applied across every governed surface.
///
/// This is the routing policy the spec asks for: a single object carrying quiet-hours,
/// do-not-disturb, presentation/follow, lock-screen privacy, admin suppression, and
/// managed-endpoint posture, evaluated identically on the in-app, OS, and companion
/// surfaces rather than re-expressed as a per-surface preference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionPolicy {
    /// Stable, namespaced policy id.
    pub policy_id: String,
    /// One reviewable sentence describing the policy.
    pub summary: String,
    /// Whether quiet-hours is active.
    pub quiet_hours: QuietHoursModeClass,
    /// The do-not-disturb posture.
    pub do_not_disturb: DoNotDisturbClass,
    /// The presentation / follow posture.
    pub presentation_mode: PresentationModeClass,
    /// The lock-screen privacy posture.
    pub lock_screen: LockScreenStateClass,
    /// The admin / managed suppression policy.
    pub admin_suppression: AdminNotificationPolicyClass,
    /// The managed-endpoint compliance posture.
    pub managed_endpoint: ManagedEndpointPostureClass,
}

impl SuppressionPolicy {
    /// Whether do-not-disturb is on.
    fn dnd_on(&self) -> bool {
        self.do_not_disturb == DoNotDisturbClass::On
    }

    /// Whether presentation or follow mode is active.
    fn presentation_active(&self) -> bool {
        matches!(
            self.presentation_mode,
            PresentationModeClass::Presenting | PresentationModeClass::FollowMode
        )
    }

    /// The highest-priority active interruption-deferring source, if any. Quiet-hours
    /// ranks above do-not-disturb, which ranks above presentation/follow, so the source
    /// named on an outcome is deterministic.
    fn active_interruption_source(&self) -> Option<SuppressionSourceClass> {
        if self.quiet_hours.is_active() {
            Some(SuppressionSourceClass::QuietHours)
        } else if self.dnd_on() {
            Some(SuppressionSourceClass::DoNotDisturb)
        } else if self.presentation_active() {
            Some(SuppressionSourceClass::PresentationFollow)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Attention signal (the event being evaluated).
// ---------------------------------------------------------------------------

/// The suppression-relevant projection of a notification envelope or durable activity
/// object, against which a policy is evaluated.
///
/// It carries the severity, privacy class, scope, and explicitly named consequence the
/// engine needs, plus the durable reopen route, but never raw message text — the
/// `consequence_note` is a short metadata-safe sentence naming the scope and
/// consequence, not the event body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionSignal {
    /// Stable, namespaced signal id.
    pub signal_id: String,
    /// Human-readable label.
    pub label: String,
    /// The source subsystem that produced the event.
    pub source_subsystem: SourceSubsystemClass,
    /// The severity class.
    pub severity: NotificationSeverityClass,
    /// The scope namespace the attention applies to.
    pub scope: AttentionScopeClass,
    /// The privacy class governing what may be shown, mirrored, or exported.
    pub privacy_class: NotificationPrivacyClass,
    /// The explicitly named consequence class, if any.
    pub consequence: ConsequenceClass,
    /// A short, metadata-safe sentence naming the scope and consequence. Non-empty iff
    /// [`consequence`](Self::consequence) is named; it carries no message body.
    pub consequence_note: String,
    /// The authoritative object the durable record reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref (never a URL, host, or path).
    pub reopen_anchor_ref: String,
    /// The default redaction posture on the in-product record (metadata-safe).
    pub default_redaction: AttentionRedactionClass,
    /// Whether the event is backed by a durable authoritative record (always true —
    /// suppression never operates on toast-only attention).
    pub carries_durable_record: bool,
    /// Evaluation stamp.
    pub created_at: String,
}

impl AttentionSignal {
    /// Whether this event is important enough to be a candidate for escaping quiet-hours
    /// (a review/incident/collaboration handoff or a security advisory).
    pub fn is_high_importance(&self) -> bool {
        self.severity.is_important()
    }

    /// Whether this event is a security advisory, which can never be silenced.
    pub fn is_security(&self) -> bool {
        self.severity.is_security()
    }

    /// Whether this event explicitly names its scope and consequence, the precondition
    /// for a high-importance event to escape quiet-hours, do-not-disturb, and
    /// presentation/follow.
    pub fn names_scope_and_consequence(&self) -> bool {
        self.consequence.is_named() && !self.consequence_note.is_empty()
    }

    /// Whether this event may escape the interruption-deferring postures: a security
    /// advisory always may; any other high-importance event may only when it names its
    /// scope and consequence.
    pub fn can_escape_interruption(&self) -> bool {
        self.is_security() || (self.is_high_importance() && self.names_scope_and_consequence())
    }
}

// ---------------------------------------------------------------------------
// Surface profile (mirrors the matrix; bound by an invariant).
// ---------------------------------------------------------------------------

/// The suppression-relevant facts about a governed surface, mirrored from the matrix.
struct SurfaceProfile {
    privacy_ceiling: NotificationPrivacyClass,
    default_redaction: AttentionRedactionClass,
    is_durable_authoritative: bool,
    mirrors_authoritative: bool,
}

fn surface_profile(surface: FanoutChannelClass) -> SurfaceProfile {
    use AttentionRedactionClass::*;
    use FanoutChannelClass::*;
    use NotificationPrivacyClass::*;
    match surface {
        InAppActivityCenter => SurfaceProfile {
            privacy_ceiling: ManagedSensitive,
            default_redaction: MetadataSafeDefault,
            is_durable_authoritative: true,
            mirrors_authoritative: false,
        },
        OsNativeNotification => SurfaceProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: SummaryOnly,
            is_durable_authoritative: false,
            mirrors_authoritative: true,
        },
        BrowserCompanion => SurfaceProfile {
            privacy_ceiling: WorkspaceSensitive,
            default_redaction: RedactedPayload,
            is_durable_authoritative: false,
            mirrors_authoritative: true,
        },
        MobileCompanion => SurfaceProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: SummaryOnly,
            is_durable_authoritative: false,
            mirrors_authoritative: true,
        },
        // The dock/taskbar badge and operator dashboard are out of this lane's scope;
        // they keep a coarse profile so the match stays total.
        DockTaskbarBadge => SurfaceProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: CountOnly,
            is_durable_authoritative: false,
            mirrors_authoritative: true,
        },
        OperatorDashboard => SurfaceProfile {
            privacy_ceiling: ManagedSensitive,
            default_redaction: InternalSupportRestricted,
            is_durable_authoritative: false,
            mirrors_authoritative: true,
        },
    }
}

fn is_companion(surface: FanoutChannelClass) -> bool {
    matches!(
        surface,
        FanoutChannelClass::BrowserCompanion | FanoutChannelClass::MobileCompanion
    )
}

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

/// The token for a redaction class (the matrix enum carries no `as_str`).
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

fn stronger_redaction(
    a: AttentionRedactionClass,
    b: AttentionRedactionClass,
) -> AttentionRedactionClass {
    if redaction_rank(a) >= redaction_rank(b) {
        a
    } else {
        b
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

/// The surface's *normal* redaction for a signal with no suppression input active: the
/// stronger of the signal's in-product default and the surface default, raised to a
/// redacted payload when the signal's privacy class exceeds the surface's ceiling so
/// privacy is never widened on fanout.
fn surface_normal_redaction(
    signal: &AttentionSignal,
    surface: FanoutChannelClass,
) -> AttentionRedactionClass {
    let profile = surface_profile(surface);
    let mut normal = stronger_redaction(signal.default_redaction, profile.default_redaction);
    if privacy_rank(signal.privacy_class) > privacy_rank(profile.privacy_ceiling) {
        normal = stronger_redaction(normal, AttentionRedactionClass::RedactedPayload);
    }
    normal
}

// ---------------------------------------------------------------------------
// Per-surface outcome and decision.
// ---------------------------------------------------------------------------

/// The outcome of evaluating one signal against one policy on one governed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceSuppressionOutcome {
    /// The governed surface.
    pub surface: FanoutChannelClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// How the surface treated the event.
    pub disposition: SuppressionDispositionClass,
    /// The suppression input that produced the disposition.
    pub suppression_source: SuppressionSourceClass,
    /// Stable suppression-source token.
    pub source_token: String,
    /// The redaction actually applied on this surface.
    pub applied_redaction: AttentionRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// Whether this surface holds the durable authoritative record (the in-app activity
    /// center only).
    pub delivers_durable_record: bool,
    /// Whether a high-importance or security event escaped the interruption-deferring
    /// postures with a redacted summary on this surface.
    pub escaped_suppression: bool,
    /// Whether this outcome names the scope and consequence that justified an escape
    /// (true only on escape outcomes).
    pub names_scope_and_consequence: bool,
    /// Whether this outcome requires a visible audit trail (a withheld or downgraded
    /// high-importance event out-of-window).
    pub audit_trail_required: bool,
    /// The matrix suppression state this outcome maps to, or `None` when the event was
    /// shown at normal treatment.
    pub resulting_state: Option<AttentionStateClass>,
    /// One reviewable sentence explaining the disposition.
    pub reason: String,
}

/// One suppression-ledger entry, recorded separately from audit history for an
/// out-of-window surface that withheld or downgraded an event.
///
/// The ledger is the durable, inspectable record of *why a surface suppressed an event*.
/// It is explicitly separate from the underlying object's audit history and never
/// implies the underlying job or incident disappeared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionLedgerEntry {
    /// Stable, namespaced ledger-entry id.
    pub ledger_entry_id: String,
    /// The signal that was suppressed.
    pub signal_id: String,
    /// The surface that withheld or downgraded the event.
    pub surface: FanoutChannelClass,
    /// The disposition recorded (downgraded or withheld).
    pub disposition: SuppressionDispositionClass,
    /// The suppression input that produced the disposition.
    pub suppression_source: SuppressionSourceClass,
    /// Stable suppression-source token.
    pub source_token: String,
    /// The scope namespace the suppressed event applied to.
    pub scope: AttentionScopeClass,
    /// The explicitly named consequence class, if any.
    pub consequence: ConsequenceClass,
    /// A short, metadata-safe sentence naming the scope and consequence (or a routine
    /// note when no consequence is named); never the message body.
    pub consequence_note: String,
    /// The matrix suppression state this entry records.
    pub resulting_state: AttentionStateClass,
    /// The authoritative object the durable record still reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque reopen anchor ref of the durable record.
    pub reopen_anchor_ref: String,
    /// Whether this entry is stored separately from audit history (always true).
    pub separate_from_audit_history: bool,
    /// Whether suppression implies the underlying job/incident disappeared (always
    /// false — a muted event is still durable).
    pub implies_underlying_disappeared: bool,
    /// Whether the suppressed event was high-importance.
    pub high_importance: bool,
    /// One reviewable sentence describing the ledger entry.
    pub note: String,
}

/// The full suppression decision for one signal against one policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionDecision {
    /// Stable, namespaced decision id.
    pub decision_id: String,
    /// The signal id evaluated.
    pub signal_id: String,
    /// The policy id evaluated against.
    pub policy_id: String,
    /// The source subsystem of the evaluated signal.
    pub source_subsystem: SourceSubsystemClass,
    /// The severity of the evaluated signal.
    pub severity: NotificationSeverityClass,
    /// Whether a durable in-product record is present regardless of suppression (always
    /// true — the in-app activity center always shows it).
    pub durable_record_present: bool,
    /// Whether a security advisory was silenced on every surface (always false).
    pub security_silenced: bool,
    /// Whether a high-importance or security event escaped the interruption-deferring
    /// postures on any out-of-window surface.
    pub high_importance_escaped: bool,
    /// The per-surface outcomes, one per governed surface, in canonical order.
    pub outcomes: Vec<SurfaceSuppressionOutcome>,
    /// The suppression-ledger entries, one per out-of-window surface that withheld or
    /// downgraded the event.
    pub ledger_entries: Vec<SuppressionLedgerEntry>,
}

impl SuppressionDecision {
    /// The outcome for a surface, if handled.
    pub fn outcome(&self, surface: FanoutChannelClass) -> Option<&SurfaceSuppressionOutcome> {
        self.outcomes.iter().find(|o| o.surface == surface)
    }

    /// The surfaces this decision delivered the event to (shown or downgraded).
    pub fn delivered_surfaces(&self) -> Vec<FanoutChannelClass> {
        self.outcomes
            .iter()
            .filter(|o| o.disposition.is_delivered())
            .map(|o| o.surface)
            .collect()
    }

    /// The surfaces this decision withheld the event from.
    pub fn withheld_surfaces(&self) -> Vec<FanoutChannelClass> {
        self.outcomes
            .iter()
            .filter(|o| o.disposition.is_withheld())
            .map(|o| o.surface)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// The suppression engine.
// ---------------------------------------------------------------------------

/// Evaluates one signal against one policy, deterministically.
///
/// Pure: the same `(signal, policy)` yields the same [`SuppressionDecision`] every call,
/// so a suppression decision is reproducible in support export and CLI/headless
/// diagnostics. The in-app activity center always shows the durable record; out-of-window
/// surfaces are governed by the same policy, with the disposition explained per surface,
/// a separate suppression-ledger entry for every withheld or downgraded out-of-window
/// surface, and a named-consequence escape for high-importance and security events.
pub fn evaluate_suppression(
    signal: &AttentionSignal,
    policy: &SuppressionPolicy,
) -> SuppressionDecision {
    let outcomes: Vec<SurfaceSuppressionOutcome> = GOVERNED_SURFACES
        .iter()
        .map(|surface| evaluate_surface(signal, policy, *surface))
        .collect();

    let ledger_entries: Vec<SuppressionLedgerEntry> = outcomes
        .iter()
        .filter(|o| o.surface != FanoutChannelClass::InAppActivityCenter)
        .filter(|o| o.disposition != SuppressionDispositionClass::Shown)
        .map(|o| ledger_entry(signal, o))
        .collect();

    let durable_record_present = outcomes.iter().any(|o| {
        o.surface == FanoutChannelClass::InAppActivityCenter
            && o.disposition == SuppressionDispositionClass::Shown
            && o.delivers_durable_record
    });

    // A security advisory is silenced only if no surface delivered it at all; the in-app
    // activity center always shows it, so this is always false.
    let security_silenced =
        signal.is_security() && !outcomes.iter().any(|o| o.disposition.is_delivered());

    let high_importance_escaped =
        signal.is_high_importance() && outcomes.iter().any(|o| o.escaped_suppression);

    SuppressionDecision {
        decision_id: format!(
            "m5-quiet-hours-suppression:decision:{}:{}",
            signal.signal_id, policy.policy_id
        ),
        signal_id: signal.signal_id.clone(),
        policy_id: policy.policy_id.clone(),
        source_subsystem: signal.source_subsystem,
        severity: signal.severity,
        durable_record_present,
        security_silenced,
        high_importance_escaped,
        outcomes,
        ledger_entries,
    }
}

fn evaluate_surface(
    signal: &AttentionSignal,
    policy: &SuppressionPolicy,
    surface: FanoutChannelClass,
) -> SurfaceSuppressionOutcome {
    if surface == FanoutChannelClass::InAppActivityCenter {
        return in_app_outcome(signal);
    }
    out_of_window_outcome(signal, policy, surface)
}

fn in_app_outcome(signal: &AttentionSignal) -> SurfaceSuppressionOutcome {
    let surface = FanoutChannelClass::InAppActivityCenter;
    SurfaceSuppressionOutcome {
        surface,
        surface_id: surface.channel_id(),
        disposition: SuppressionDispositionClass::Shown,
        suppression_source: SuppressionSourceClass::None,
        source_token: SuppressionSourceClass::None.as_str().to_owned(),
        applied_redaction: signal.default_redaction,
        redaction_token: redaction_token(signal.default_redaction).to_owned(),
        delivers_durable_record: true,
        escaped_suppression: false,
        names_scope_and_consequence: false,
        audit_trail_required: false,
        resulting_state: None,
        reason: "The in-app activity center holds the durable authoritative record and \
                 always shows it, independent of quiet-hours, do-not-disturb, \
                 presentation/follow, lock-screen, or admin suppression."
            .to_owned(),
    }
}

fn out_of_window_outcome(
    signal: &AttentionSignal,
    policy: &SuppressionPolicy,
    surface: FanoutChannelClass,
) -> SurfaceSuppressionOutcome {
    let normal = surface_normal_redaction(signal, surface);
    let high = signal.is_high_importance();
    let security = signal.is_security();
    let escape = signal.can_escape_interruption();

    // 1. A non-compliant managed endpoint may not receive out-of-window payloads at all.
    if policy.managed_endpoint == ManagedEndpointPostureClass::NonCompliant {
        return withheld(
            signal,
            surface,
            SuppressionSourceClass::ManagedEndpointPolicy,
            normal,
            high,
            format!(
                "The managed endpoint is non-compliant, so the {} is withheld; the durable \
                 in-product record still holds the attention.",
                surface.label()
            ),
        );
    }

    // 2. Admin suppression can lock cross-client companion fanout entirely.
    if policy.admin_suppression == AdminNotificationPolicyClass::ManagedLocked
        && is_companion(surface)
    {
        return withheld(
            signal,
            surface,
            SuppressionSourceClass::AdminSuppression,
            normal,
            high,
            format!(
                "Admin policy locks cross-client companion fanout, so the {} is withheld; the \
                 attention stays in-product.",
                surface.label()
            ),
        );
    }

    // 3. The interruption-deferring postures: quiet-hours, do-not-disturb, or
    //    presentation/follow. A security advisory or a named high-importance event
    //    escapes with a redacted summary; everything else is withheld.
    if let Some(source) = policy.active_interruption_source() {
        if escape {
            // Escapes with a redacted summary, and lock-screen privacy still applies.
            let mut applied = stronger_redaction(normal, AttentionRedactionClass::SummaryOnly);
            applied = apply_lock_screen(applied, policy, signal);
            return downgraded_escape(signal, surface, source, applied, security);
        }
        return withheld(
            signal,
            surface,
            source,
            normal,
            high,
            format!(
                "{} defers this out-of-window fanout to the {}; it did not name a scope and \
                 consequence to escape, so it stays in-product and returns when the policy ends.",
                source.label(),
                surface.label()
            ),
        );
    }

    // 4. Lock-screen privacy downgrades sensitive content to a count-only affordance.
    if policy.lock_screen.is_locked()
        && privacy_rank(signal.privacy_class) > privacy_rank(NotificationPrivacyClass::SummarySafe)
    {
        let applied = stronger_redaction(normal, AttentionRedactionClass::CountOnly);
        return downgraded(
            signal,
            surface,
            SuppressionSourceClass::LockScreenPrivacy,
            applied,
            high,
            format!(
                "Lock-screen privacy downgrades the {} to a count-only affordance ({}) because the \
                 event is above summary-safe; the full payload stays in-product.",
                surface.label(),
                redaction_token(applied),
            ),
        );
    }

    // 5. Admin restricted policy raises redaction without withholding.
    if policy.admin_suppression == AdminNotificationPolicyClass::ManagedRestricted {
        let applied = stronger_redaction(normal, AttentionRedactionClass::RedactedPayload);
        return downgraded(
            signal,
            surface,
            SuppressionSourceClass::AdminSuppression,
            applied,
            high,
            format!(
                "Admin policy restricts cross-client fanout, so the {} is delivered with a raised \
                 redaction ({}); privacy is never widened on fanout.",
                surface.label(),
                redaction_token(applied),
            ),
        );
    }

    // 6. No suppression input applied — shown at the surface's normal treatment.
    shown(signal, surface, normal)
}

/// Raises the applied redaction for lock-screen privacy when the screen is locked and
/// the event is above summary-safe, so an escape still respects the lock screen.
fn apply_lock_screen(
    applied: AttentionRedactionClass,
    policy: &SuppressionPolicy,
    signal: &AttentionSignal,
) -> AttentionRedactionClass {
    if policy.lock_screen.is_locked()
        && privacy_rank(signal.privacy_class) > privacy_rank(NotificationPrivacyClass::SummarySafe)
    {
        stronger_redaction(applied, AttentionRedactionClass::CountOnly)
    } else {
        applied
    }
}

fn shown(
    _signal: &AttentionSignal,
    surface: FanoutChannelClass,
    applied: AttentionRedactionClass,
) -> SurfaceSuppressionOutcome {
    SurfaceSuppressionOutcome {
        surface,
        surface_id: surface.channel_id(),
        disposition: SuppressionDispositionClass::Shown,
        suppression_source: SuppressionSourceClass::None,
        source_token: SuppressionSourceClass::None.as_str().to_owned(),
        applied_redaction: applied,
        redaction_token: redaction_token(applied).to_owned(),
        delivers_durable_record: false,
        escaped_suppression: false,
        names_scope_and_consequence: false,
        audit_trail_required: false,
        resulting_state: None,
        reason: format!(
            "No suppression policy applies, so the {} shows the event at its normal treatment ({}).",
            surface.label(),
            redaction_token(applied),
        ),
    }
}

fn downgraded(
    _signal: &AttentionSignal,
    surface: FanoutChannelClass,
    source: SuppressionSourceClass,
    applied: AttentionRedactionClass,
    high: bool,
    reason: String,
) -> SurfaceSuppressionOutcome {
    SurfaceSuppressionOutcome {
        surface,
        surface_id: surface.channel_id(),
        disposition: SuppressionDispositionClass::Downgraded,
        suppression_source: source,
        source_token: source.as_str().to_owned(),
        applied_redaction: applied,
        redaction_token: redaction_token(applied).to_owned(),
        delivers_durable_record: false,
        escaped_suppression: false,
        names_scope_and_consequence: false,
        audit_trail_required: high,
        resulting_state: Some(AttentionStateClass::Suppressed),
        reason,
    }
}

fn downgraded_escape(
    signal: &AttentionSignal,
    surface: FanoutChannelClass,
    source: SuppressionSourceClass,
    applied: AttentionRedactionClass,
    security: bool,
) -> SurfaceSuppressionOutcome {
    let lead = if security {
        "A security advisory"
    } else {
        "A high-importance event that names its scope and consequence"
    };
    let reason = format!(
        "{} escapes {} to the {} with a redacted summary ({}); the named consequence is \"{}\" and \
         the full payload stays in-product.",
        lead,
        source.label(),
        surface.label(),
        redaction_token(applied),
        signal.consequence_note,
    );
    SurfaceSuppressionOutcome {
        surface,
        surface_id: surface.channel_id(),
        disposition: SuppressionDispositionClass::Downgraded,
        suppression_source: source,
        source_token: source.as_str().to_owned(),
        applied_redaction: applied,
        redaction_token: redaction_token(applied).to_owned(),
        delivers_durable_record: false,
        escaped_suppression: true,
        names_scope_and_consequence: true,
        audit_trail_required: true,
        resulting_state: Some(AttentionStateClass::Suppressed),
        reason,
    }
}

fn withheld(
    _signal: &AttentionSignal,
    surface: FanoutChannelClass,
    source: SuppressionSourceClass,
    applied: AttentionRedactionClass,
    high: bool,
    reason: String,
) -> SurfaceSuppressionOutcome {
    let resulting_state = if source == SuppressionSourceClass::QuietHours {
        AttentionStateClass::QuietHoursDeferred
    } else {
        AttentionStateClass::Suppressed
    };
    SurfaceSuppressionOutcome {
        surface,
        surface_id: surface.channel_id(),
        disposition: SuppressionDispositionClass::Withheld,
        suppression_source: source,
        source_token: source.as_str().to_owned(),
        applied_redaction: applied,
        redaction_token: redaction_token(applied).to_owned(),
        delivers_durable_record: false,
        escaped_suppression: false,
        names_scope_and_consequence: false,
        audit_trail_required: high,
        resulting_state: Some(resulting_state),
        reason,
    }
}

fn ledger_entry(
    signal: &AttentionSignal,
    outcome: &SurfaceSuppressionOutcome,
) -> SuppressionLedgerEntry {
    let resulting_state = outcome
        .resulting_state
        .unwrap_or(AttentionStateClass::Suppressed);
    let consequence_note = if signal.consequence.is_named() {
        signal.consequence_note.clone()
    } else {
        format!(
            "Routine {} event in scope {}; no named consequence.",
            signal.source_subsystem.as_str(),
            signal.scope.as_str(),
        )
    };
    let note = format!(
        "{} {} the {} via {}; the underlying object is unchanged and stays reopenable on its {}.",
        if outcome.disposition.is_withheld() {
            "Withheld from"
        } else {
            "Downgraded on"
        },
        signal.label,
        outcome.surface.label(),
        outcome.suppression_source.label(),
        signal.reopen_target.as_str(),
    );
    SuppressionLedgerEntry {
        ledger_entry_id: format!(
            "m5-quiet-hours-suppression:ledger:{}:{}",
            signal.signal_id,
            outcome.surface.as_str(),
        ),
        signal_id: signal.signal_id.clone(),
        surface: outcome.surface,
        disposition: outcome.disposition,
        suppression_source: outcome.suppression_source,
        source_token: outcome.suppression_source.as_str().to_owned(),
        scope: signal.scope,
        consequence: signal.consequence,
        consequence_note,
        resulting_state,
        reopen_target: signal.reopen_target,
        reopen_anchor_ref: signal.reopen_anchor_ref.clone(),
        separate_from_audit_history: true,
        implies_underlying_disappeared: false,
        high_importance: signal.is_high_importance(),
        note,
    }
}

// ---------------------------------------------------------------------------
// Governed-surface registry and bundle record.
// ---------------------------------------------------------------------------

/// One governed-surface entry: a surface this lane evaluates with the shared policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedSurfaceEntry {
    /// The surface.
    pub surface: FanoutChannelClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// Human-readable label.
    pub label: String,
    /// The privacy ceiling above which content is redacted on this surface.
    pub privacy_ceiling: NotificationPrivacyClass,
    /// The surface's default redaction posture.
    pub default_redaction: AttentionRedactionClass,
    /// Stable default-redaction token.
    pub default_redaction_token: String,
    /// Whether this surface holds the durable authoritative record.
    pub is_durable_authoritative: bool,
    /// Whether this surface mirrors the authoritative object rather than being it.
    pub mirrors_authoritative: bool,
    /// One reviewable sentence describing how the policy governs this surface.
    pub governed_note: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuietHoursSuppressionInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen quiet-hours-suppression bundle: the governed surfaces, the policy corpus,
/// the signal corpus, and every decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuietHoursSuppressionBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_quiet_hours_suppression_schema_version: u32,
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
    /// The governed surfaces.
    pub governed_surfaces: Vec<GovernedSurfaceEntry>,
    /// The representative suppression-policy corpus.
    pub policies: Vec<SuppressionPolicy>,
    /// The representative signal corpus.
    pub signals: Vec<AttentionSignal>,
    /// Every decision (each signal evaluated against each policy).
    pub decisions: Vec<SuppressionDecision>,
    /// The computed invariants.
    pub invariants: Vec<QuietHoursSuppressionInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuietHoursSuppressionValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for QuietHoursSuppressionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "quiet-hours-suppression bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for QuietHoursSuppressionValidationError {}

impl QuietHoursSuppressionBundle {
    /// The signal with a given id, if present.
    pub fn signal(&self, signal_id: &str) -> Option<&AttentionSignal> {
        self.signals.iter().find(|s| s.signal_id == signal_id)
    }

    /// The policy with a given id, if present.
    pub fn policy(&self, policy_id: &str) -> Option<&SuppressionPolicy> {
        self.policies.iter().find(|p| p.policy_id == policy_id)
    }

    /// The decision for a `(signal, policy)` pair, if present.
    pub fn decision(&self, signal_id: &str, policy_id: &str) -> Option<&SuppressionDecision> {
        self.decisions
            .iter()
            .find(|d| d.signal_id == signal_id && d.policy_id == policy_id)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
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
        let from_signals = self.signals.iter().map(|s| s.reopen_anchor_ref.as_str());
        let from_ledger = self.decisions.iter().flat_map(|d| {
            d.ledger_entries
                .iter()
                .map(|l| l.reopen_anchor_ref.as_str())
        });
        fixed.chain(from_signals).chain(from_ledger)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), QuietHoursSuppressionValidationError> {
        let fail = |reason: String| Err(QuietHoursSuppressionValidationError { reason });

        if self.record_kind != M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.policies.is_empty() || self.signals.is_empty() || self.decisions.is_empty() {
            return fail("policies, signals, and decisions must be non-empty".to_owned());
        }

        // The governed surfaces are exactly the four this lane evaluates.
        if self.governed_surfaces.len() != GOVERNED_SURFACES.len()
            || !GOVERNED_SURFACES
                .iter()
                .all(|s| self.governed_surfaces.iter().any(|e| e.surface == *s))
        {
            return fail("governed surfaces must be exactly the four governed surfaces".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.policies.iter().map(|p| p.policy_id.as_str())) {
            return fail("policy ids are not unique".to_owned());
        }
        if !all_unique(self.signals.iter().map(|s| s.signal_id.as_str())) {
            return fail("signal ids are not unique".to_owned());
        }
        if !all_unique(self.decisions.iter().map(|d| d.decision_id.as_str())) {
            return fail("decision ids are not unique".to_owned());
        }

        // Every signal carries a durable record and, when it names a consequence, a
        // non-empty note (and the reverse).
        for signal in &self.signals {
            if !signal.carries_durable_record {
                return fail(format!(
                    "signal {} must carry a durable record",
                    signal.signal_id
                ));
            }
            if signal.consequence.is_named() == signal.consequence_note.is_empty() {
                return fail(format!(
                    "signal {} consequence and note must agree",
                    signal.signal_id
                ));
            }
            if signal.reopen_anchor_ref.is_empty() {
                return fail(format!(
                    "signal {} is missing its reopen anchor",
                    signal.signal_id
                ));
            }
        }

        // Every decision references a known signal and policy and recomputes identically
        // (reproducible suppression).
        for decision in &self.decisions {
            let Some(signal) = self.signal(&decision.signal_id) else {
                return fail(format!(
                    "decision {} references unknown signal {}",
                    decision.decision_id, decision.signal_id
                ));
            };
            let Some(policy) = self.policy(&decision.policy_id) else {
                return fail(format!(
                    "decision {} references unknown policy {}",
                    decision.decision_id, decision.policy_id
                ));
            };
            if &evaluate_suppression(signal, policy) != decision {
                return fail(format!(
                    "decision {} is not reproducible from its signal and policy",
                    decision.decision_id
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

/// Builds the canonical quiet-hours-suppression bundle.
///
/// Deterministic: the same bytes every call. The governed surfaces, policy corpus, and
/// signal corpus are fixed, every decision is computed by [`evaluate_suppression`], and
/// each invariant's `holds` flag is computed from the built data, so an inconsistent edit
/// flips an invariant rather than silently passing.
pub fn quiet_hours_suppression_bundle() -> QuietHoursSuppressionBundle {
    let governed_surfaces = build_governed_surfaces();
    let policies = build_policies();
    let signals = build_signals();
    let decisions = build_decisions(&signals, &policies);
    let invariants = compute_invariants(&governed_surfaces, &policies, &signals, &decisions);

    QuietHoursSuppressionBundle {
        record_kind: M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND.to_owned(),
        m5_quiet_hours_suppression_schema_version: M5_QUIET_HOURS_SUPPRESSION_SCHEMA_VERSION,
        schema_ref: M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF.to_owned(),
        bundle_id: M5_QUIET_HOURS_SUPPRESSION_BUNDLE_ID.to_owned(),
        as_of: M5_QUIET_HOURS_SUPPRESSION_AS_OF.to_owned(),
        matrix_ref: M5_QUIET_HOURS_SUPPRESSION_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_QUIET_HOURS_SUPPRESSION_FREEZE_GATE_REF.to_owned(),
        summary: "One coherent suppression policy — quiet-hours, do-not-disturb, \
                  presentation/follow, lock-screen privacy, admin suppression, and managed-endpoint \
                  posture — applied across the in-app activity center, OS notification, and \
                  browser/mobile companion surfaces. Every surface explains whether it showed, \
                  downgraded, or withheld the event; the in-app activity center always keeps the \
                  durable authoritative record; high-importance security, trust, approval, and route \
                  warnings escape quiet-hours only when they name their scope and consequence; a \
                  security advisory is never silenced; suppression is recorded in a ledger separate \
                  from audit history that never implies the underlying job or incident disappeared; \
                  and a downgrade never widens privacy on fanout."
            .to_owned(),
        governed_surfaces,
        policies,
        signals,
        decisions,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_governed_surfaces() -> Vec<GovernedSurfaceEntry> {
    GOVERNED_SURFACES
        .iter()
        .map(|surface| {
            let profile = surface_profile(*surface);
            let governed_note = if profile.is_durable_authoritative {
                "Holds the durable authoritative record and always shows the event; suppression \
                 never drops it."
                    .to_owned()
            } else {
                format!(
                    "An out-of-window mirror governed by the same policy; it shows, downgrades, or \
                     withholds the event and never invents its own mute logic (privacy ceiling {}).",
                    profile.privacy_ceiling.as_str(),
                )
            };
            GovernedSurfaceEntry {
                surface: *surface,
                surface_id: surface.channel_id(),
                label: surface.label().to_owned(),
                privacy_ceiling: profile.privacy_ceiling,
                default_redaction: profile.default_redaction,
                default_redaction_token: redaction_token(profile.default_redaction).to_owned(),
                is_durable_authoritative: profile.is_durable_authoritative,
                mirrors_authoritative: profile.mirrors_authoritative,
                governed_note,
            }
        })
        .collect()
}

fn build_decisions(
    signals: &[AttentionSignal],
    policies: &[SuppressionPolicy],
) -> Vec<SuppressionDecision> {
    let mut out = Vec::new();
    for signal in signals {
        for policy in policies {
            out.push(evaluate_suppression(signal, policy));
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn policy(
    slug: &str,
    summary: &str,
    quiet_hours: QuietHoursModeClass,
    do_not_disturb: DoNotDisturbClass,
    presentation_mode: PresentationModeClass,
    lock_screen: LockScreenStateClass,
    admin_suppression: AdminNotificationPolicyClass,
    managed_endpoint: ManagedEndpointPostureClass,
) -> SuppressionPolicy {
    SuppressionPolicy {
        policy_id: format!("suppression_policy:{slug}:0001"),
        summary: summary.to_owned(),
        quiet_hours,
        do_not_disturb,
        presentation_mode,
        lock_screen,
        admin_suppression,
        managed_endpoint,
    }
}

fn build_policies() -> Vec<SuppressionPolicy> {
    use AdminNotificationPolicyClass as A;
    use DoNotDisturbClass as D;
    use LockScreenStateClass as L;
    use ManagedEndpointPostureClass as M;
    use PresentationModeClass as Pr;
    use QuietHoursModeClass as Q;

    vec![
        policy(
            "clear",
            "Nothing suppressing: quiet-hours off, no do-not-disturb, not presenting, unlocked, \
             unmanaged, compliant endpoint.",
            Q::Off,
            D::Off,
            Pr::Off,
            L::Unlocked,
            A::Unmanaged,
            M::Compliant,
        ),
        policy(
            "quiet_hours",
            "Quiet-hours is active; out-of-window fanout defers unless an event names its scope \
             and consequence.",
            Q::Active,
            D::Off,
            Pr::Off,
            L::Unlocked,
            A::Unmanaged,
            M::Compliant,
        ),
        policy(
            "do_not_disturb",
            "Do-not-disturb is on; out-of-window interruptions defer unless explicitly named.",
            Q::Off,
            D::On,
            Pr::Off,
            L::Unlocked,
            A::Unmanaged,
            M::Compliant,
        ),
        policy(
            "presentation",
            "Presenting / screen-sharing; out-of-window interruptions defer unless explicitly \
             named.",
            Q::Off,
            D::Off,
            Pr::Presenting,
            L::Unlocked,
            A::Unmanaged,
            M::Compliant,
        ),
        policy(
            "lock_screen",
            "The screen is locked; sensitive content is hidden behind a count-only affordance \
             out-of-window.",
            Q::Off,
            D::Off,
            Pr::Off,
            L::Locked,
            A::Unmanaged,
            M::Compliant,
        ),
        policy(
            "managed_restricted",
            "Admin policy restricts cross-client fanout to a raised redaction.",
            Q::Off,
            D::Off,
            Pr::Off,
            L::Unlocked,
            A::ManagedRestricted,
            M::Compliant,
        ),
        policy(
            "managed_locked",
            "Admin policy locks cross-client companion fanout entirely.",
            Q::Off,
            D::Off,
            Pr::Off,
            L::Unlocked,
            A::ManagedLocked,
            M::Compliant,
        ),
        policy(
            "managed_endpoint_noncompliant",
            "The managed endpoint is non-compliant; out-of-window fanout is withheld to the device.",
            Q::Off,
            D::Off,
            Pr::Off,
            L::Unlocked,
            A::ManagedDefault,
            M::NonCompliant,
        ),
        policy(
            "quiet_hours_locked_managed",
            "Quiet-hours active on a locked screen under a restricted managed policy: layered \
             suppression a named event still escapes with a lock-screen-redacted summary.",
            Q::Active,
            D::Off,
            Pr::Off,
            L::Locked,
            A::ManagedRestricted,
            M::Compliant,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn signal(
    slug: &str,
    label: &str,
    source_subsystem: SourceSubsystemClass,
    severity: NotificationSeverityClass,
    scope: AttentionScopeClass,
    privacy_class: NotificationPrivacyClass,
    consequence: ConsequenceClass,
    consequence_note: &str,
    reopen_target: ReopenTargetClass,
) -> AttentionSignal {
    AttentionSignal {
        signal_id: format!("attention_signal:{slug}:0001"),
        label: label.to_owned(),
        source_subsystem,
        severity,
        scope,
        privacy_class,
        consequence,
        consequence_note: consequence_note.to_owned(),
        reopen_target,
        reopen_anchor_ref: format!("aureline://object/{slug}/0001"),
        default_redaction: AttentionRedactionClass::MetadataSafeDefault,
        carries_durable_record: true,
        created_at: M5_QUIET_HOURS_SUPPRESSION_AS_OF.to_owned(),
    }
}

fn build_signals() -> Vec<AttentionSignal> {
    use AttentionScopeClass as Sc;
    use ConsequenceClass as C;
    use NotificationPrivacyClass as P;
    use NotificationSeverityClass as Sev;
    use ReopenTargetClass as R;
    use SourceSubsystemClass as S;

    vec![
        signal(
            "task.completed",
            "Task run completed",
            S::TaskRunner,
            Sev::MinorSuccess,
            Sc::Session,
            P::SummarySafe,
            C::None,
            "",
            R::ActivityJobRow,
        ),
        signal(
            "support.export_ready",
            "Support export ready",
            S::Support,
            Sev::Informational,
            Sc::AppGlobal,
            P::SummarySafe,
            C::None,
            "",
            R::EvidencePacket,
        ),
        signal(
            "collab.review_requested",
            "Collaboration review requested",
            S::Collaboration,
            Sev::HandoffActionable,
            Sc::Collaboration,
            P::WorkspaceSensitive,
            // A routine handoff that does NOT name a consequence: high-importance but it
            // cannot escape quiet-hours.
            C::None,
            "",
            R::ReviewRequest,
        ),
        signal(
            "ai.awaiting_approval",
            "AI change awaiting approval",
            S::Ai,
            Sev::HandoffActionable,
            Sc::Session,
            P::WorkspaceSensitive,
            C::ApprovalRequired,
            "Approval required before the AI change applies to the session workspace.",
            R::ReviewRequest,
        ),
        signal(
            "route.policy_warning",
            "Managed route warning",
            S::ManagedPolicy,
            Sev::HandoffActionable,
            Sc::TenantOrg,
            P::ManagedSensitive,
            C::RouteWarning,
            "A managed routing change risks misrouting tenant-org traffic until reviewed.",
            R::PolicyDiff,
        ),
        signal(
            "trust.provider_changed",
            "Provider trust changed",
            S::Sync,
            Sev::HandoffActionable,
            Sc::Workspace,
            P::SecurityCritical,
            C::TrustChange,
            "A workspace provider's trust posture changed and needs review before reuse.",
            R::AuditEvent,
        ),
        signal(
            "security.credential_revoked",
            "Security credential revoked",
            S::Security,
            Sev::SecurityAdvisory,
            Sc::AppGlobal,
            P::SecurityCritical,
            C::SecurityAdvisory,
            "An app-global credential was revoked and must be rotated.",
            R::AuditEvent,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> QuietHoursSuppressionInvariant {
    QuietHoursSuppressionInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn signal_by_id<'a>(
    signals: &'a [AttentionSignal],
    signal_id: &str,
) -> Option<&'a AttentionSignal> {
    signals.iter().find(|s| s.signal_id == signal_id)
}

fn policy_by_id<'a>(
    policies: &'a [SuppressionPolicy],
    policy_id: &str,
) -> Option<&'a SuppressionPolicy> {
    policies.iter().find(|p| p.policy_id == policy_id)
}

fn compute_invariants(
    governed_surfaces: &[GovernedSurfaceEntry],
    policies: &[SuppressionPolicy],
    signals: &[AttentionSignal],
    decisions: &[SuppressionDecision],
) -> Vec<QuietHoursSuppressionInvariant> {
    let matrix = attention_routing_matrix();
    let mut out = Vec::new();

    // One policy across the in-app, OS, and companion surfaces.
    out.push(invariant(
        "suppression.parity_one_policy_all_surfaces",
        "Every decision evaluates exactly the four governed surfaces — the in-app activity center, \
         the OS notification, and the browser and mobile companions — against one policy, so \
         quiet-hours and suppression are one routing policy rather than a per-surface preference.",
        governed_surfaces.len() == GOVERNED_SURFACES.len()
            && decisions.iter().all(|d| {
                d.outcomes.len() == GOVERNED_SURFACES.len()
                    && GOVERNED_SURFACES
                        .iter()
                        .all(|s| d.outcomes.iter().any(|o| o.surface == *s))
            }),
    ));

    // The in-app activity center always shows the durable record.
    out.push(invariant(
        "suppression.in_app_durable_record_always",
        "Every decision shows the in-app activity center as the durable authoritative record, \
         independent of any suppression, so a muted or quiet event never loses its in-product home.",
        decisions.iter().all(|d| {
            d.durable_record_present
                && d.outcome(FanoutChannelClass::InAppActivityCenter)
                    .is_some_and(|o| {
                        o.disposition == SuppressionDispositionClass::Shown
                            && o.delivers_durable_record
                            && o.suppression_source == SuppressionSourceClass::None
                    })
        }),
    ));

    // Every surface explains itself with a source token and a reason.
    out.push(invariant(
        "suppression.explains_every_surface",
        "Every surface outcome carries a stable suppression-source token and a non-empty reason, so \
         each surface can explain whether it showed, downgraded, or withheld the event.",
        decisions.iter().all(|d| {
            d.outcomes.iter().all(|o| {
                !o.reason.is_empty()
                    && o.source_token == o.suppression_source.as_str()
                    && o.redaction_token == redaction_token(o.applied_redaction)
            })
        }),
    ));

    // The corpus exercises all three dispositions.
    out.push(invariant(
        "suppression.three_dispositions_exercised",
        "The corpus produces all three dispositions — shown, downgraded, and withheld — so the \
         shown/downgraded/withheld explainability triad is real, not nominal.",
        SuppressionDispositionClass::ALL.iter().all(|disp| {
            decisions
                .iter()
                .any(|d| d.outcomes.iter().any(|o| o.disposition == *disp))
        }),
    ));

    // Every suppression source (except none) is exercised.
    out.push(invariant(
        "suppression.every_source_exercised",
        "Every suppression source — quiet-hours, do-not-disturb, presentation/follow, lock-screen \
         privacy, admin suppression, and managed-endpoint policy — appears in at least one outcome.",
        SuppressionSourceClass::ALL
            .iter()
            .filter(|s| **s != SuppressionSourceClass::None)
            .all(|src| {
                decisions
                    .iter()
                    .any(|d| d.outcomes.iter().any(|o| o.suppression_source == *src))
            }),
    ));

    // A security advisory is never silenced.
    out.push(invariant(
        "suppression.security_never_silenced",
        "For every security-advisory signal, the in-app activity center shows it and at least one \
         out-of-window surface escapes with a redacted summary when an interruption policy is \
         active; it is never silenced on every surface.",
        decisions
            .iter()
            .all(|d| match signal_by_id(signals, &d.signal_id) {
                Some(s) if s.is_security() => {
                    !d.security_silenced
                        && d.outcome(FanoutChannelClass::InAppActivityCenter)
                            .is_some_and(|o| o.disposition == SuppressionDispositionClass::Shown)
                        && d.outcomes.iter().any(|o| o.disposition.is_delivered())
                }
                _ => true,
            })
            && decisions.iter().all(|d| !d.security_silenced),
    ));

    // High-importance escapes only when it names scope and consequence.
    out.push(invariant(
        "suppression.high_importance_escapes_only_when_named",
        "On an out-of-window surface under an active interruption policy, a high-importance event \
         escapes (is downgraded rather than withheld) exactly when it names its scope and \
         consequence; an unnamed high-importance event is withheld and kept in-product.",
        decisions.iter().all(|d| {
            let Some(signal) = signal_by_id(signals, &d.signal_id) else {
                return false;
            };
            let Some(policy) = policy_by_id(policies, &d.policy_id) else {
                return false;
            };
            // Only meaningful when an interruption policy is active and nothing harder
            // (managed-endpoint or admin-lock) overrides it.
            let interruption_active = policy.active_interruption_source().is_some();
            d.outcomes.iter().all(|o| {
                if o.surface == FanoutChannelClass::InAppActivityCenter {
                    return true;
                }
                if !interruption_active {
                    return true;
                }
                if !o.suppression_source.is_interruption() {
                    // A harder source (managed-endpoint or admin lock) took precedence.
                    return true;
                }
                if signal.is_high_importance() {
                    o.escaped_suppression == signal.can_escape_interruption()
                } else {
                    // A routine event under interruption suppression is never escaped.
                    !o.escaped_suppression
                }
            })
        }),
    ));

    // Every escape names its scope and consequence.
    out.push(invariant(
        "suppression.escape_names_scope_and_consequence",
        "Every escaped outcome marks that it names a scope and consequence, and the signal behind it \
         carries a named consequence and a non-empty note, so an interruption is always justified by \
         an explicitly named scope and consequence.",
        decisions.iter().all(|d| {
            d.outcomes.iter().all(|o| {
                if !o.escaped_suppression {
                    return true;
                }
                let Some(signal) = signal_by_id(signals, &d.signal_id) else {
                    return false;
                };
                o.names_scope_and_consequence
                    && o.disposition == SuppressionDispositionClass::Downgraded
                    && (signal.is_security()
                        || (signal.consequence.is_named() && !signal.consequence_note.is_empty()))
            })
        }),
    ));

    // A withheld or downgraded out-of-window event keeps its durable record and reopen
    // route.
    out.push(invariant(
        "suppression.withheld_keeps_durable_record_and_reopen",
        "Every out-of-window outcome that withheld or downgraded an event keeps the decision's \
         durable in-product record and a ledger entry that reopens the same authoritative object, \
         so a suppressed event never loses its reopen route.",
        decisions.iter().all(|d| {
            d.durable_record_present
                && d.outcomes
                    .iter()
                    .filter(|o| o.surface != FanoutChannelClass::InAppActivityCenter)
                    .filter(|o| o.disposition != SuppressionDispositionClass::Shown)
                    .all(|o| {
                        d.ledger_entries.iter().any(|l| {
                            l.surface == o.surface
                                && !l.reopen_anchor_ref.is_empty()
                                && is_export_safe_ref(&l.reopen_anchor_ref)
                        })
                    })
        }),
    ));

    // Suppression stays separate from audit history.
    out.push(invariant(
        "suppression.separate_from_audit_history",
        "Every suppression-ledger entry is marked separate from audit history and never implies the \
         underlying job or incident disappeared, so suppression state and audit history stay \
         distinct.",
        decisions.iter().all(|d| {
            d.ledger_entries
                .iter()
                .all(|l| l.separate_from_audit_history && !l.implies_underlying_disappeared)
        }),
    ));

    // Blocked or downgraded high-importance events stay accountable.
    out.push(invariant(
        "suppression.audit_trail_for_blocked_high_importance",
        "Every out-of-window outcome that withheld or downgraded a high-importance event requires a \
         visible audit trail and produces a ledger entry; a shown event never requires one, so a \
         blocked high-value event is always inspectable.",
        decisions.iter().all(|d| {
            let high = signal_by_id(signals, &d.signal_id).is_some_and(|s| s.is_high_importance());
            d.outcomes.iter().all(|o| {
                if o.surface == FanoutChannelClass::InAppActivityCenter {
                    return !o.audit_trail_required;
                }
                match o.disposition {
                    SuppressionDispositionClass::Shown => !o.audit_trail_required,
                    SuppressionDispositionClass::Downgraded
                    | SuppressionDispositionClass::Withheld => {
                        let entry_present = d.ledger_entries.iter().any(|l| l.surface == o.surface);
                        entry_present && (o.audit_trail_required == high)
                    }
                }
            })
        }),
    ));

    // A downgrade never widens privacy.
    out.push(invariant(
        "suppression.downgrade_never_widens_privacy",
        "Every delivered out-of-window outcome applies a redaction at least as strong as the \
         surface's normal treatment, so suppression only ever raises redaction and never widens \
         privacy on fanout.",
        decisions.iter().all(|d| {
            let Some(signal) = signal_by_id(signals, &d.signal_id) else {
                return false;
            };
            d.outcomes
                .iter()
                .filter(|o| o.surface != FanoutChannelClass::InAppActivityCenter)
                .filter(|o| o.disposition.is_delivered())
                .all(|o| {
                    redaction_rank(o.applied_redaction)
                        >= redaction_rank(surface_normal_redaction(signal, o.surface))
                })
        }),
    ));

    // Every non-shown outcome maps to a matrix suppression state.
    out.push(invariant(
        "suppression.state_is_matrix_suppression_state",
        "Every outcome that withheld or downgraded an event maps to a matrix suppression state \
         (suppressed or quiet-hours-deferred), and every shown outcome maps to no suppression state, \
         so suppression state is named in the frozen vocabulary.",
        decisions.iter().all(|d| {
            d.outcomes.iter().all(|o| match o.disposition {
                SuppressionDispositionClass::Shown => o.resulting_state.is_none(),
                _ => o.resulting_state.is_some_and(|st| st.is_suppression()),
            })
        }),
    ));

    // Every decision is reproducible.
    out.push(invariant(
        "suppression.decisions_reproducible",
        "Re-evaluating every decision from its signal and policy yields an identical decision, so a \
         suppression decision is reproducible in support export and diagnostics.",
        decisions.iter().all(|d| {
            match (
                signal_by_id(signals, &d.signal_id),
                policy_by_id(policies, &d.policy_id),
            ) {
                (Some(s), Some(p)) => &evaluate_suppression(s, p) == d,
                _ => false,
            }
        }),
    ));

    // Every token binds back to the attention-routing matrix.
    out.push(invariant(
        "suppression.matrix_bound",
        "Every privacy class, scope, redaction class, reopen target, severity, and suppression \
         state the bundle uses is one the attention-routing matrix defines, and the routing-context \
         object can show the suppression states, so the suppression path never drifts from the \
         frozen object model.",
        matrix_bound_holds(signals, decisions, &matrix),
    ));

    // Every reference is support-export safe with no raw text.
    out.push(invariant(
        "suppression.support_export_safe",
        "Every signal reopen anchor and every ledger reopen anchor is a repo-relative object ref or \
         opaque aureline:// handle, never a URL, host, credential, message body, or absolute path.",
        signals.iter().all(|s| is_export_safe_ref(&s.reopen_anchor_ref))
            && decisions.iter().all(|d| {
                d.ledger_entries
                    .iter()
                    .all(|l| is_export_safe_ref(&l.reopen_anchor_ref))
            }),
    ));

    out
}

fn matrix_bound_holds(
    signals: &[AttentionSignal],
    decisions: &[SuppressionDecision],
    matrix: &AttentionRoutingMatrix,
) -> bool {
    let privacy_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .privacy_classes
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let scope_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .scopes
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let redaction_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .redaction_classes
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let reopen_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .reopen_targets
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let severity_tokens: Vec<&str> = matrix
        .shared_vocabulary
        .severities
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let routing_context = matrix.object(AttentionObjectClass::RoutingContext);

    let signals_bound = signals.iter().all(|s| {
        privacy_tokens.contains(&s.privacy_class.as_str())
            && scope_tokens.contains(&s.scope.as_str())
            && redaction_tokens.contains(&redaction_token(s.default_redaction))
            && reopen_tokens.contains(&s.reopen_target.as_str())
            && severity_tokens.contains(&s.severity.as_str())
    });

    let states_bound = routing_context.is_some_and(|o| {
        o.can_show(AttentionStateClass::Suppressed)
            && o.can_show(AttentionStateClass::QuietHoursDeferred)
            && o.can_show(AttentionStateClass::Shown)
    });

    let outcomes_bound = decisions.iter().all(|d| {
        d.outcomes
            .iter()
            .all(|o| redaction_tokens.contains(&redaction_token(o.applied_redaction)))
    });

    signals_bound && states_bound && outcomes_bound
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn quiet_hours_suppression_lines(bundle: &QuietHoursSuppressionBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Quiet-hours-suppression bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Surfaces: {}  Policies: {}  Signals: {}  Decisions: {}  Invariants: {}",
        bundle.governed_surfaces.len(),
        bundle.policies.len(),
        bundle.signals.len(),
        bundle.decisions.len(),
        bundle.invariants.len(),
    ));

    lines.push("Governed surfaces:".to_owned());
    for s in &bundle.governed_surfaces {
        lines.push(format!(
            "  - {} [{}] ceiling={} default_redaction={} durable={}",
            s.label,
            s.surface_id,
            s.privacy_ceiling.as_str(),
            s.default_redaction_token,
            s.is_durable_authoritative,
        ));
    }

    lines.push("Policies:".to_owned());
    for p in &bundle.policies {
        lines.push(format!(
            "  - {} quiet_hours={} dnd={} presentation={} lock={} admin={} endpoint={}",
            p.policy_id,
            p.quiet_hours.as_str(),
            p.do_not_disturb.as_str(),
            p.presentation_mode.as_str(),
            p.lock_screen.as_str(),
            p.admin_suppression.as_str(),
            p.managed_endpoint.as_str(),
        ));
    }

    lines.push("Signals:".to_owned());
    for s in &bundle.signals {
        lines.push(format!(
            "  - {} [{}] severity={} privacy={} scope={} consequence={} high={}",
            s.label,
            s.signal_id,
            s.severity.as_str(),
            s.privacy_class.as_str(),
            s.scope.as_str(),
            s.consequence.as_str(),
            s.is_high_importance(),
        ));
    }

    lines.push("Decisions:".to_owned());
    for d in &bundle.decisions {
        let surfaces: Vec<String> = d
            .outcomes
            .iter()
            .map(|o| {
                format!(
                    "{}={}/{}",
                    o.surface.as_str(),
                    o.disposition.as_str(),
                    o.source_token,
                )
            })
            .collect();
        lines.push(format!(
            "  - {} + {} -> [{}] ledger={} escaped={}",
            d.signal_id,
            d.policy_id,
            surfaces.join(", "),
            d.ledger_entries.len(),
            d.high_importance_escaped,
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
