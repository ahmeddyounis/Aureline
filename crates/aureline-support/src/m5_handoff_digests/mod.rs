//! M5 handoff bundles and shift digests: reopen-safe operator continuity packets
//! over the same canonical incident/support/admin/service objects the detail
//! surfaces own.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the *family*
//! of a handoff bundle and a shift digest — what each is, the one shared state
//! vocabulary, and the ownership/freshness/redaction/scope/live-vs-snapshot fields
//! every operator surface holds. The [action plans](crate::m5_action_plans) build
//! the first real ordered checklists. This lane builds the first real **continuity
//! packets**: the end-of-shift / client / role handoff bundles and the daily / shift
//! digests an operator hands the next operator so operational meaning survives the
//! handoff instead of being reconstructed from screenshots and tribal knowledge.
//!
//! A continuity packet is not a generic "attached info" blob. The hard part is
//! preserving, outside the live session, exactly what the operator saw — and the
//! *storage / freshness / boundary* distinction behind each piece of evidence:
//!
//! 1. **Live, cached, mirrored, and snapshot evidence stay distinct.** Every
//!    [`EvidenceItem`] carries a [`StorageClass`] — live link, cached copy,
//!    mirrored last-synced view, or frozen snapshot — and a [`FreshnessClass`].
//!    The packet never flattens these into one bucket; the roll-up counts each
//!    class separately so a snapshot can never read as a live link.
//! 2. **Reopen-safe continuity.** Every [`ObjectGroup`] and packet carries a
//!    [`ReopenAnchor`]; [`compute_resolves_object`] makes the rule executable: an
//!    anchor lands on the live canonical object, a labeled cached/mirrored view, or
//!    a [`ReopenAnchorClass::TruthfulPlaceholder`] that names what the object was —
//!    **never** a generic, unscoped home screen.
//! 3. **Digests group by object and severity before chronology.** A digest's
//!    groups are ordered by [`SeverityClass`] (most severe first); each group keeps
//!    its latest update time and major blocker, and only *within* a group are
//!    [`DigestEvent`]s ordered chronologically, so the next operator resumes from
//!    the worst, freshest thing first.
//! 4. **Unresolved questions travel with the work.** Each [`UnresolvedQuestion`]
//!    names what is still open, who owns it, the canonical object it is about, and
//!    the next safe action — so a handoff answers what changed, what remains
//!    unresolved, and what to do next.
//! 5. **Explicit scope and boundary truth before share/export.** Every packet names
//!    a [`ScopeClass`] and a [`SharePosture`] and a [`ContinuityExportGate`] stating
//!    exactly what crosses the boundary on share/export at that scope.
//! 6. **Frozen, lossless export.** [`ContinuityHandoffExport`] freezes the packet as
//!    a `snapshot_only` export that preserves every truth field — including each
//!    evidence item's storage class and freshness — so the truth survives outside
//!    the UI and a lossy export fails CI.
//!
//! [`handoff_digest_set`] is the canonical binding: it builds the packets
//! deterministically and computes each [`ContinuityInvariant`]'s `holds` flag from
//! the built data, so the checked-in fixture and the replay gate freeze the contract
//! byte-for-byte. The record carries no endpoint URLs, hostnames, credentials, raw
//! provider payloads, or absolute paths — only opaque object refs, stable tokens,
//! and short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::m5_action_plans::SharePosture;
use crate::m5_operator_boards::{BlockerWaiverClass, FreshnessClass, ObjectKind};
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorSurfaceClass, RedactionClass, ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the handoff/digest set.
pub const M5_HANDOFF_DIGESTS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the handoff/digest set.
pub const M5_HANDOFF_DIGESTS_SCHEMA_REF: &str = "schemas/ops/m5-handoff-digests.schema.json";

/// Stable record-kind tag for the handoff/digest set.
pub const M5_HANDOFF_DIGESTS_RECORD_KIND: &str = "m5_handoff_digest_set";

/// Stable id for the canonical handoff/digest set.
pub const M5_HANDOFF_DIGESTS_SET_ID: &str = "m5-handoff-digests:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_HANDOFF_DIGESTS_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for object identity.
pub const M5_HANDOFF_DIGESTS_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_HANDOFF_DIGESTS_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Packet kind and families.
// ---------------------------------------------------------------------------

/// Which operator-surface family a continuity packet is an instance of.
///
/// A continuity packet is either a [`HandoffBundle`](OperatorSurfaceClass::HandoffBundle)
/// (a frozen handoff to the next operator / client / role) or a
/// [`ShiftDigest`](OperatorSurfaceClass::ShiftDigest) (a windowed roll-up). Both
/// share the same evidence, reopen-anchor, unresolved-question, and scope/boundary
/// vocabulary; the kind names which matrix surface the packet binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityPacketKind {
    /// A handoff bundle: a frozen, scope-preserving export handed to the next
    /// operator, a client, or another role.
    HandoffBundle,
    /// A shift digest: a windowed roll-up grouped by object and severity.
    ShiftDigest,
}

impl ContinuityPacketKind {
    /// All packet kinds, in vocabulary order.
    pub const ALL: [Self; 2] = [Self::HandoffBundle, Self::ShiftDigest];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandoffBundle => "handoff_bundle",
            Self::ShiftDigest => "shift_digest",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HandoffBundle => "Handoff bundle",
            Self::ShiftDigest => "Shift digest",
        }
    }

    /// The operator-surface matrix family this kind binds.
    pub const fn surface(self) -> OperatorSurfaceClass {
        match self {
            Self::HandoffBundle => OperatorSurfaceClass::HandoffBundle,
            Self::ShiftDigest => OperatorSurfaceClass::ShiftDigest,
        }
    }
}

/// The first real continuity packets this lane proves the shared contract with.
///
/// Two handoff bundles (an outgoing-shift handoff and a client-facing handoff) and
/// two digests (a daily operations digest and a night-shift digest) prove both
/// matrix surfaces, the four storage classes, the four reopen-anchor classes, and a
/// private, a workspace-shared, and an org-shared scope. Adding a packet is a
/// breaking change to the set; the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketClass {
    /// End-of-shift handoff to the incoming on-call operator.
    OutgoingShiftHandoff,
    /// Client-facing status handoff with a metadata-safe view.
    ClientStatusHandoff,
    /// Daily operations digest shared org-wide.
    DailyOperationsDigest,
    /// Private night-shift digest for the next on-call.
    NightShiftDigest,
}

impl PacketClass {
    /// All packets, in set order.
    pub const ALL: [Self; 4] = [
        Self::OutgoingShiftHandoff,
        Self::ClientStatusHandoff,
        Self::DailyOperationsDigest,
        Self::NightShiftDigest,
    ];

    /// Stable snake_case token for this packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutgoingShiftHandoff => "outgoing_shift_handoff",
            Self::ClientStatusHandoff => "client_status_handoff",
            Self::DailyOperationsDigest => "daily_operations_digest",
            Self::NightShiftDigest => "night_shift_digest",
        }
    }

    /// Stable, namespaced packet id.
    pub fn packet_id(self) -> String {
        format!("continuity_packet.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OutgoingShiftHandoff => "Outgoing shift handoff",
            Self::ClientStatusHandoff => "Client status handoff",
            Self::DailyOperationsDigest => "Daily operations digest",
            Self::NightShiftDigest => "Night-shift digest",
        }
    }

    /// The operator-surface family this packet binds.
    pub const fn kind(self) -> ContinuityPacketKind {
        match self {
            Self::OutgoingShiftHandoff | Self::ClientStatusHandoff => {
                ContinuityPacketKind::HandoffBundle
            }
            Self::DailyOperationsDigest | Self::NightShiftDigest => {
                ContinuityPacketKind::ShiftDigest
            }
        }
    }

    /// The matrix surface this packet is an instance of.
    pub const fn surface(self) -> OperatorSurfaceClass {
        self.kind().surface()
    }
}

// ---------------------------------------------------------------------------
// Severity.
// ---------------------------------------------------------------------------

/// The severity of an object group or a digest event.
///
/// Digests group by object and severity *before* chronology, so groups are ordered
/// by [`SeverityClass::rank`] (most severe first) and a group's severity is the
/// most severe of its events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityClass {
    /// Sev1 — critical / customer-impacting.
    Sev1Critical,
    /// Sev2 — major.
    Sev2Major,
    /// Sev3 — minor.
    Sev3Minor,
    /// Sev4 — informational.
    Sev4Info,
}

impl SeverityClass {
    /// All severities, most severe first.
    pub const ALL: [Self; 4] = [
        Self::Sev1Critical,
        Self::Sev2Major,
        Self::Sev3Minor,
        Self::Sev4Info,
    ];

    /// Stable snake_case token for this severity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sev1Critical => "sev1_critical",
            Self::Sev2Major => "sev2_major",
            Self::Sev3Minor => "sev3_minor",
            Self::Sev4Info => "sev4_info",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sev1Critical => "Sev1 — critical",
            Self::Sev2Major => "Sev2 — major",
            Self::Sev3Minor => "Sev3 — minor",
            Self::Sev4Info => "Sev4 — informational",
        }
    }

    /// Severity rank, higher being more severe, for severity-ordered grouping.
    pub const fn rank(self) -> i64 {
        match self {
            Self::Sev1Critical => 3,
            Self::Sev2Major => 2,
            Self::Sev3Minor => 1,
            Self::Sev4Info => 0,
        }
    }
}

/// Returns the most severe of a slice of severities, or `None` when empty.
fn max_severity(severities: &[SeverityClass]) -> Option<SeverityClass> {
    severities.iter().copied().max_by_key(|s| s.rank())
}

// ---------------------------------------------------------------------------
// Storage class — the live / cached / mirrored / snapshot distinction.
// ---------------------------------------------------------------------------

/// How a piece of evidence is stored relative to the live canonical object.
///
/// This is the central guardrail of the lane: a continuity packet never flattens
/// live, cached, mirrored, and snapshot evidence into one generic "attached info"
/// blob. The storage / freshness / boundary distinction is part of the contract, so
/// every [`EvidenceItem`] carries its storage class and the roll-up counts each
/// class separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageClass {
    /// A live link into the canonical object; reopening resolves current truth.
    LiveLink,
    /// A cached copy captured at handoff time; refreshable when reconnected.
    Cached,
    /// A last-synced mirror from an offline / mirrored path; not live until resync.
    Mirrored,
    /// A frozen point-in-time snapshot; immutable and never refreshed.
    Snapshot,
}

impl StorageClass {
    /// All storage classes, in vocabulary order.
    pub const ALL: [Self; 4] = [Self::LiveLink, Self::Cached, Self::Mirrored, Self::Snapshot];

    /// Stable snake_case token for this storage class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLink => "live_link",
            Self::Cached => "cached",
            Self::Mirrored => "mirrored",
            Self::Snapshot => "snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveLink => "Live link",
            Self::Cached => "Cached",
            Self::Mirrored => "Mirrored (last synced)",
            Self::Snapshot => "Snapshot (frozen)",
        }
    }

    /// Whether reopening this evidence resolves current live truth. True only for a
    /// live link; a cached, mirrored, or snapshot copy is explicitly not live.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveLink)
    }

    /// Whether this evidence can be refreshed against the source when reconnected.
    /// True for everything but a frozen snapshot.
    pub const fn can_refresh(self) -> bool {
        !matches!(self, Self::Snapshot)
    }
}

// ---------------------------------------------------------------------------
// Reopen anchors — reopen-safe continuity.
// ---------------------------------------------------------------------------

/// Where reopening a packet, group, or evidence item lands the next operator.
///
/// The closed set has no "generic dashboard" variant on purpose: reopening always
/// resolves to the canonical object (live, cached, or mirrored) or a
/// [`ReopenAnchorClass::TruthfulPlaceholder`] that names what the object was — never
/// an unscoped home screen. [`compute_resolves_object`] makes the rule executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenAnchorClass {
    /// Lands on the live canonical object.
    LiveObject,
    /// Lands on a labeled cached snapshot of the canonical object.
    CachedObjectSnapshot,
    /// Lands on the last-synced, offline-labeled mirror of the canonical object.
    MirroredOfflineView,
    /// The object no longer resolves; lands on a labeled placeholder naming what it
    /// was and why it cannot resolve — never a generic dashboard.
    TruthfulPlaceholder,
}

impl ReopenAnchorClass {
    /// All anchor classes, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::LiveObject,
        Self::CachedObjectSnapshot,
        Self::MirroredOfflineView,
        Self::TruthfulPlaceholder,
    ];

    /// Stable snake_case token for this anchor class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveObject => "live_object",
            Self::CachedObjectSnapshot => "cached_object_snapshot",
            Self::MirroredOfflineView => "mirrored_offline_view",
            Self::TruthfulPlaceholder => "truthful_placeholder",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveObject => "Live object",
            Self::CachedObjectSnapshot => "Cached object snapshot",
            Self::MirroredOfflineView => "Mirrored offline view",
            Self::TruthfulPlaceholder => "Truthful placeholder",
        }
    }

    /// Whether the anchor resolves to a canonical object (live, cached, or
    /// mirrored). False only for a truthful placeholder.
    pub const fn resolves_object(self) -> bool {
        !matches!(self, Self::TruthfulPlaceholder)
    }

    /// Whether the anchor requires a non-empty canonical target ref.
    pub const fn requires_target(self) -> bool {
        self.resolves_object()
    }

    /// Whether the anchor requires a written placeholder label (true only for a
    /// truthful placeholder, which must name what the object was).
    pub const fn requires_placeholder_label(self) -> bool {
        matches!(self, Self::TruthfulPlaceholder)
    }
}

/// Computes whether a reopen anchor resolves to a canonical object.
///
/// This is the reopen-safe-continuity rule made executable: an anchor lands on the
/// canonical object only when its class is not [`ReopenAnchorClass::TruthfulPlaceholder`].
/// No anchor ever resolves to a generic, unscoped home screen.
pub fn compute_resolves_object(anchor_class: ReopenAnchorClass) -> bool {
    anchor_class.resolves_object()
}

// ---------------------------------------------------------------------------
// Unresolved-question status.
// ---------------------------------------------------------------------------

/// The status of an unresolved question carried by a continuity packet.
///
/// Every variant is, by definition, still unresolved; the status names *how* it is
/// unresolved so the next operator knows whether to act, wait, or decide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionStatus {
    /// Open and unworked.
    Open,
    /// Being actively investigated.
    Investigating,
    /// Blocked, with a stated reason.
    Blocked,
    /// Awaiting a decision before any action.
    NeedsDecision,
}

impl QuestionStatus {
    /// All statuses, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::Open,
        Self::Investigating,
        Self::Blocked,
        Self::NeedsDecision,
    ];

    /// Stable snake_case token for this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Investigating => "investigating",
            Self::Blocked => "blocked",
            Self::NeedsDecision => "needs_decision",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Investigating => "Investigating",
            Self::Blocked => "Blocked",
            Self::NeedsDecision => "Needs decision",
        }
    }

    /// Whether a written reason is required for this status.
    pub const fn requires_reason(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

// ---------------------------------------------------------------------------
// Target audience.
// ---------------------------------------------------------------------------

/// Who a continuity packet is handed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAudienceClass {
    /// The operator picking up the next shift.
    NextOperatorShift,
    /// A client / customer-facing role; default to the most redacted view.
    ClientFacing,
    /// The wider operations team / org.
    TeamWide,
}

impl TargetAudienceClass {
    /// All audiences, in vocabulary order.
    pub const ALL: [Self; 3] = [Self::NextOperatorShift, Self::ClientFacing, Self::TeamWide];

    /// Stable snake_case token for this audience.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NextOperatorShift => "next_operator_shift",
            Self::ClientFacing => "client_facing",
            Self::TeamWide => "team_wide",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NextOperatorShift => "Next operator shift",
            Self::ClientFacing => "Client-facing",
            Self::TeamWide => "Team-wide",
        }
    }
}

// ---------------------------------------------------------------------------
// Packet actions.
// ---------------------------------------------------------------------------

/// The actions a continuity packet exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityActionClass {
    /// Open the canonical object behind a group (routes to the detail object).
    OpenObject,
    /// Open a linked evidence item (local-safe).
    OpenEvidence,
    /// Reopen the packet at its anchor: the canonical object or a truthful
    /// placeholder, never a generic dashboard (local-safe).
    ReopenAtAnchor,
    /// Capture an answer or note against an unresolved question (local-safe).
    CaptureAnswer,
    /// Export the packet as a frozen, machine-readable snapshot (local-safe).
    ExportSnapshot,
    /// Share the packet at its scope, with explicit boundary truth.
    SharePacket,
}

impl ContinuityActionClass {
    /// All actions, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::OpenObject,
        Self::OpenEvidence,
        Self::ReopenAtAnchor,
        Self::CaptureAnswer,
        Self::ExportSnapshot,
        Self::SharePacket,
    ];

    /// Stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenObject => "open_object",
            Self::OpenEvidence => "open_evidence",
            Self::ReopenAtAnchor => "reopen_at_anchor",
            Self::CaptureAnswer => "capture_answer",
            Self::ExportSnapshot => "export_snapshot",
            Self::SharePacket => "share_packet",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenObject => "Open object",
            Self::OpenEvidence => "Open evidence",
            Self::ReopenAtAnchor => "Reopen at anchor",
            Self::CaptureAnswer => "Capture answer",
            Self::ExportSnapshot => "Export snapshot",
            Self::SharePacket => "Share packet",
        }
    }

    /// Whether the action is local-safe: it reads or captures locally and never
    /// crosses a share boundary on its own.
    pub const fn local_safe(self) -> bool {
        !matches!(self, Self::SharePacket)
    }

    /// Whether the action routes to a canonical detail object rather than only
    /// reading the packet's own state.
    pub const fn routes_to_canonical_object(self) -> bool {
        matches!(self, Self::OpenObject | Self::ReopenAtAnchor)
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One piece of evidence pinned by a continuity packet.
///
/// The storage class and freshness are kept distinct on every item so the packet
/// never flattens live, cached, mirrored, and snapshot evidence into one blob. The
/// `is_live` and `can_refresh` flags are computed from the storage class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceItem {
    /// Stable, packet-namespaced evidence id.
    pub evidence_id: String,
    /// Short label for the evidence.
    pub label: String,
    /// The canonical handle of the evidence object.
    pub evidence_ref: String,
    /// How the evidence is stored relative to the live object.
    pub storage_class: StorageClass,
    /// The freshness age of the evidence.
    pub freshness: FreshnessClass,
    /// A short origin lane label (never a URL or host).
    pub origin: String,
    /// When the evidence was captured / last synced.
    pub captured_at: String,
    /// Whether reopening resolves current live truth ([`StorageClass::is_live`]).
    pub is_live: bool,
    /// Whether the evidence can be refreshed when reconnected
    /// ([`StorageClass::can_refresh`]).
    pub can_refresh: bool,
}

/// One event in a group's chronology — what changed about the object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestEvent {
    /// Stable, group-namespaced event id.
    pub event_id: String,
    /// When the event occurred.
    pub at: String,
    /// One reviewable sentence describing what changed.
    pub summary: String,
    /// The severity of the event.
    pub severity: SeverityClass,
}

/// Where reopening lands the next operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenAnchor {
    /// The anchor class.
    pub anchor_class: ReopenAnchorClass,
    /// The canonical object the anchor resolves to, or empty for a placeholder.
    pub target_ref: String,
    /// A written placeholder label naming what the object was; required (and only
    /// present) for a truthful placeholder.
    pub placeholder_label: String,
    /// Whether the anchor resolves to a canonical object
    /// ([`compute_resolves_object`]); false only for a truthful placeholder.
    pub resolves_object: bool,
    /// One reviewable sentence describing what reopening does.
    pub note: String,
}

/// One canonical object in a packet, grouped before chronology.
///
/// Groups are ordered by [`SeverityClass`] (most severe first); within a group the
/// `events` are ordered chronologically. The group preserves the object's latest
/// update time, its major blocker, and the reopen anchor that lands the next
/// operator on the right object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectGroup {
    /// Stable, packet-namespaced group id.
    pub group_id: String,
    /// The canonical object this group is about.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// Short label for the object.
    pub object_label: String,
    /// The group's severity — the most severe of its events.
    pub severity: SeverityClass,
    /// The object's blocker / waiver state.
    pub blocker: BlockerWaiverClass,
    /// A written blocker reason; required when the object is blocked or waived.
    pub blocker_reason: String,
    /// The freshness age of the object's headline evidence.
    pub freshness: FreshnessClass,
    /// The latest update time across this group's events.
    pub latest_update_at: String,
    /// One reviewable sentence describing what changed about the object.
    pub what_changed: String,
    /// The reopen anchor for this object.
    pub reopen_anchor: ReopenAnchor,
    /// The selected evidence pinned for this object, storage classes preserved.
    pub evidence: Vec<EvidenceItem>,
    /// The object's events, ordered chronologically.
    pub events: Vec<DigestEvent>,
}

/// One unresolved question carried by a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnresolvedQuestion {
    /// Stable, packet-namespaced question id.
    pub question_id: String,
    /// The question text.
    pub question: String,
    /// The status of the question.
    pub status: QuestionStatus,
    /// Who owns the question.
    pub owner: String,
    /// The canonical object the question is about.
    pub linked_object_ref: String,
    /// The next safe action for the next operator.
    pub next_safe_action: String,
    /// A written blocker reason; required when the question is blocked.
    pub blocker_reason: String,
}

/// The explicit boundary truth a packet states before it is shared or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityExportGate {
    /// The governance scope of the packet.
    pub scope: ScopeClass,
    /// The operator-facing share posture.
    pub share_posture: SharePosture,
    /// Whether sharing/exporting requires an explicit operator acknowledgement.
    pub requires_boundary_ack: bool,
    /// One reviewable sentence naming exactly what crosses the boundary on
    /// share/export at this scope.
    pub crosses_on_share: String,
    /// The redaction posture on share/export.
    pub redaction_class: RedactionClass,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// One action a continuity packet exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityAction {
    /// The action.
    pub action: ContinuityActionClass,
    /// Human-readable label.
    pub label: String,
    /// Whether the action is local-safe (never crosses a share boundary).
    pub local_safe: bool,
    /// Whether the action routes to a canonical detail object.
    pub routes_to_canonical_object: bool,
    /// One reviewable sentence describing the action.
    pub summary: String,
}

/// The coverage window a packet rolls up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageWindow {
    /// When the window starts.
    pub start: String,
    /// When the window ends.
    pub end: String,
    /// Short label for the window.
    pub label: String,
}

/// The computed roll-up of a packet.
///
/// The storage-class counts are reported separately on purpose: a snapshot is never
/// merged into a live-link count. The roll-up answers the three handoff questions —
/// what changed, what remains unresolved, and the next safe action — in separate
/// sentences plus a combined headline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityRollUp {
    /// Total objects in scope.
    pub object_count: u32,
    /// Objects at Sev1.
    pub sev1_count: u32,
    /// Objects at Sev2.
    pub sev2_count: u32,
    /// Objects at Sev3.
    pub sev3_count: u32,
    /// Objects at Sev4.
    pub sev4_count: u32,
    /// Objects with an active blocker or expired waiver.
    pub blocked_object_count: u32,
    /// Live-link evidence items.
    pub live_link_count: u32,
    /// Cached evidence items.
    pub cached_count: u32,
    /// Mirrored evidence items.
    pub mirrored_count: u32,
    /// Snapshot evidence items.
    pub snapshot_count: u32,
    /// Unresolved questions that are open.
    pub unresolved_open: u32,
    /// Unresolved questions being investigated.
    pub unresolved_investigating: u32,
    /// Unresolved questions that are blocked.
    pub unresolved_blocked: u32,
    /// Unresolved questions awaiting a decision.
    pub unresolved_needs_decision: u32,
    /// Total unresolved questions.
    pub unresolved_total: u32,
    /// The latest update time across all groups.
    pub latest_update_at: String,
    /// One reviewable sentence answering "what changed".
    pub what_changed: String,
    /// One reviewable sentence answering "what remains unresolved".
    pub what_unresolved: String,
    /// One reviewable sentence answering "what is the next safe action".
    pub next_safe_action: String,
    /// A combined headline that keeps the storage classes distinct.
    pub headline: String,
}

/// A continuity packet frozen as a machine-readable handoff export.
///
/// The export preserves the exact object groups (with each evidence item's storage
/// class and freshness), unresolved questions, reopen anchors, coverage window,
/// scope, ownership, redaction, and boundary truth of the live packet, so the truth
/// survives outside the UI instead of flattening into prose. It is always
/// `snapshot_only`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityHandoffExport {
    /// Stable, namespaced export id.
    pub export_id: String,
    /// The packet this export belongs to.
    pub packet: PacketClass,
    /// The packet's stable id.
    pub packet_id: String,
    /// The packet kind.
    pub kind: ContinuityPacketKind,
    /// The packet's governance scope.
    pub scope: ScopeClass,
    /// The packet's share posture.
    pub share_posture: SharePosture,
    /// The role / audience the export is for.
    pub target_role: String,
    /// The redaction posture of the export.
    pub redaction_class: RedactionClass,
    /// Live-versus-snapshot posture; always snapshot for a frozen export.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// What crosses the boundary on share/export, preserved verbatim.
    pub crosses_on_share: String,
    /// One reviewable sentence summarizing the export and what handing it off does.
    pub summary: String,
    /// The coverage window, preserved.
    pub coverage_window: CoverageWindow,
    /// The number of objects in the export.
    pub object_count: u32,
    /// The resolved object groups, storage classes preserved.
    pub object_groups: Vec<ObjectGroup>,
    /// The unresolved questions, preserved.
    pub unresolved_questions: Vec<UnresolvedQuestion>,
    /// The packet's reopen anchor, preserved.
    pub reopen_anchor: ReopenAnchor,
    /// The computed roll-up, preserved in the snapshot.
    pub roll_up: ContinuityRollUp,
}

/// One continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityPacket {
    /// The packet family.
    pub packet: PacketClass,
    /// Stable, namespaced packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the packet.
    pub summary: String,
    /// The packet kind (handoff bundle or shift digest).
    pub kind: ContinuityPacketKind,
    /// The operator-surface matrix family this packet is an instance of.
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// The role / audience the packet is handed to.
    pub target_role: String,
    /// The class of audience the packet is for.
    pub target_audience: TargetAudienceClass,
    /// The owning role for the packet.
    pub owning_role: String,
    /// Who holds the decision right for the packet.
    pub decision_right: String,
    /// The packet's governance scope.
    pub scope: ScopeClass,
    /// The packet's operator-facing share posture.
    pub share_posture: SharePosture,
    /// The consumers that render this packet.
    pub consumed_by: Vec<ConsumerClass>,
    /// The default redaction posture on export / handoff.
    pub default_redaction: RedactionClass,
    /// Live-versus-snapshot posture of the live packet.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The coverage window the packet rolls up.
    pub coverage_window: CoverageWindow,
    /// One reviewable sentence stating the packet's boundary honesty.
    pub boundary_note: String,
    /// The packet-level reopen anchor that lands the next operator on the right
    /// object or a truthful placeholder.
    pub reopen_anchor: ReopenAnchor,
    /// The explicit boundary truth stated before save/share/export.
    pub export_gate: ContinuityExportGate,
    /// The actions this packet exposes.
    pub actions: Vec<ContinuityAction>,
    /// The object groups, ordered by severity before chronology.
    pub object_groups: Vec<ObjectGroup>,
    /// The unresolved questions carried with the packet.
    pub unresolved_questions: Vec<UnresolvedQuestion>,
    /// The computed roll-up.
    pub roll_up: ContinuityRollUp,
    /// The frozen handoff export of the packet, proving export parity.
    pub export: ContinuityHandoffExport,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen handoff/digest set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffDigestSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_handoff_digests_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The operator-surface matrix fixture this set binds for object identity.
    pub matrix_ref: String,
    /// The matrix record kind this set binds.
    pub matrix_record_kind: String,
    /// The packet kinds packets can carry.
    pub packet_kinds: Vec<TokenDef>,
    /// The storage classes evidence can carry.
    pub storage_classes: Vec<TokenDef>,
    /// The reopen-anchor classes anchors can carry.
    pub reopen_anchor_classes: Vec<TokenDef>,
    /// The severities groups and events can carry.
    pub severities: Vec<TokenDef>,
    /// The question statuses questions can carry.
    pub question_statuses: Vec<TokenDef>,
    /// The target audiences packets can carry.
    pub target_audiences: Vec<TokenDef>,
    /// The share postures packets can carry.
    pub share_postures: Vec<TokenDef>,
    /// The canonical object kinds packets address.
    pub object_kinds: Vec<TokenDef>,
    /// The packets.
    pub packets: Vec<ContinuityPacket>,
    /// The computed invariants.
    pub invariants: Vec<ContinuityInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the handoff/digest set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffDigestValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for HandoffDigestValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handoff/digest set invalid: {}", self.reason)
    }
}

impl std::error::Error for HandoffDigestValidationError {}

impl HandoffDigestSet {
    /// Returns the packet, if present.
    pub fn packet(&self, packet: PacketClass) -> Option<&ContinuityPacket> {
        self.packets.iter().find(|p| p.packet == packet)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().iter().all(|r| is_export_safe_ref(r))
    }

    /// Every ref string carried by the set, for export-safety auditing.
    fn all_refs(&self) -> Vec<&str> {
        let mut refs = vec![self.matrix_ref.as_str(), self.schema_ref.as_str()];
        for packet in &self.packets {
            push_packet_refs(packet, &mut refs);
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), HandoffDigestValidationError> {
        let fail = |reason: String| Err(HandoffDigestValidationError { reason });

        if self.record_kind != M5_HANDOFF_DIGESTS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_HANDOFF_DIGESTS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_HANDOFF_DIGESTS_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }

        // Every packet is present exactly once.
        for packet in PacketClass::ALL {
            if self.packets.iter().filter(|p| p.packet == packet).count() != 1 {
                return fail(format!(
                    "packet {} not present exactly once",
                    packet.as_str()
                ));
            }
        }

        // Ids are unique across the whole set.
        if !all_unique(self.packets.iter().map(|p| p.packet_id.as_str())) {
            return fail("packet ids are not unique".to_owned());
        }
        if !all_unique(
            self.packets
                .iter()
                .flat_map(|p| p.object_groups.iter().map(|g| g.group_id.as_str())),
        ) {
            return fail("group ids are not unique".to_owned());
        }
        if !all_unique(self.packets.iter().flat_map(|p| {
            p.object_groups
                .iter()
                .flat_map(|g| g.evidence.iter().map(|e| e.evidence_id.as_str()))
        })) {
            return fail("evidence ids are not unique".to_owned());
        }
        if !all_unique(self.packets.iter().flat_map(|p| {
            p.unresolved_questions
                .iter()
                .map(|q| q.question_id.as_str())
        })) {
            return fail("question ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

        for packet in &self.packets {
            if packet.packet_id != packet.packet.packet_id() {
                return fail(format!("packet id mismatch for {}", packet.packet.as_str()));
            }
            if packet.kind != packet.packet.kind()
                || packet.surface != packet.packet.surface()
                || packet.surface_id != packet.surface.surface_id()
                || matrix.surface(packet.surface).is_none()
            {
                return fail(format!(
                    "packet {} does not bind a canonical matrix surface",
                    packet.packet.as_str()
                ));
            }
            if packet.owning_role.is_empty()
                || packet.decision_right.is_empty()
                || packet.target_role.is_empty()
            {
                return fail(format!(
                    "packet {} hides owner/decision-right/target",
                    packet.packet.as_str()
                ));
            }
            if packet.object_groups.is_empty() {
                return fail(format!(
                    "packet {} has no object groups",
                    packet.packet.as_str()
                ));
            }
            if packet.unresolved_questions.is_empty() {
                return fail(format!(
                    "packet {} carries no unresolved questions",
                    packet.packet.as_str()
                ));
            }
            // Scope / share-posture / export-gate boundary truth.
            if packet.share_posture.scope() != packet.scope {
                return fail(format!(
                    "packet {} share posture disagrees with its scope",
                    packet.packet.as_str()
                ));
            }
            validate_export_gate(packet)
                .map_err(|reason| HandoffDigestValidationError { reason })?;
            validate_reopen_anchor(
                &packet.reopen_anchor,
                &format!("packet {}", packet.packet.as_str()),
            )
            .map_err(|reason| HandoffDigestValidationError { reason })?;

            // Required actions are offered.
            for required in [
                ContinuityActionClass::OpenObject,
                ContinuityActionClass::ReopenAtAnchor,
                ContinuityActionClass::CaptureAnswer,
                ContinuityActionClass::ExportSnapshot,
            ] {
                if !packet.actions.iter().any(|a| a.action == required) {
                    return fail(format!(
                        "packet {} must offer the {} action",
                        packet.packet.as_str(),
                        required.as_str()
                    ));
                }
            }

            // Groups are ordered by severity before chronology.
            let mut prev_rank: Option<i64> = None;
            for group in &packet.object_groups {
                let rank = group.severity.rank();
                if let Some(prev) = prev_rank {
                    if rank > prev {
                        return fail(format!(
                            "packet {} group {} breaks severity ordering",
                            packet.packet.as_str(),
                            group.group_id
                        ));
                    }
                }
                prev_rank = Some(rank);
                validate_group(packet.packet, group)
                    .map_err(|reason| HandoffDigestValidationError { reason })?;
            }

            for question in &packet.unresolved_questions {
                validate_question(packet.packet, question)
                    .map_err(|reason| HandoffDigestValidationError { reason })?;
            }

            // Roll-up parity.
            let recomputed_roll_up = compute_roll_up(packet);
            if packet.roll_up != recomputed_roll_up {
                return fail(format!(
                    "packet {} roll-up does not match its groups",
                    packet.packet.as_str()
                ));
            }
            // Export parity.
            let recomputed_export = compute_export(packet);
            if packet.export != recomputed_export {
                return fail(format!(
                    "packet {} export does not match its content",
                    packet.packet.as_str()
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("handoff/digest set is not support-export safe".to_owned());
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

fn push_packet_refs<'a>(packet: &'a ContinuityPacket, refs: &mut Vec<&'a str>) {
    push_anchor_ref(&packet.reopen_anchor, refs);
    push_groups_refs(&packet.object_groups, refs);
    for q in &packet.unresolved_questions {
        refs.push(q.linked_object_ref.as_str());
    }
    push_anchor_ref(&packet.export.reopen_anchor, refs);
    push_groups_refs(&packet.export.object_groups, refs);
    for q in &packet.export.unresolved_questions {
        refs.push(q.linked_object_ref.as_str());
    }
}

fn push_groups_refs<'a>(groups: &'a [ObjectGroup], refs: &mut Vec<&'a str>) {
    for group in groups {
        refs.push(group.object_ref.as_str());
        push_anchor_ref(&group.reopen_anchor, refs);
        for ev in &group.evidence {
            refs.push(ev.evidence_ref.as_str());
        }
    }
}

fn push_anchor_ref<'a>(anchor: &'a ReopenAnchor, refs: &mut Vec<&'a str>) {
    if !anchor.target_ref.is_empty() {
        refs.push(anchor.target_ref.as_str());
    }
}

fn validate_export_gate(packet: &ContinuityPacket) -> Result<(), String> {
    let gate = &packet.export_gate;
    if gate.scope != packet.scope {
        return Err(format!(
            "packet {} export gate scope mismatch",
            packet.packet.as_str()
        ));
    }
    if gate.share_posture != packet.share_posture {
        return Err(format!(
            "packet {} export gate share posture mismatch",
            packet.packet.as_str()
        ));
    }
    if gate.requires_boundary_ack != packet.share_posture.requires_boundary_ack() {
        return Err(format!(
            "packet {} export gate boundary-ack flag is inconsistent with its posture",
            packet.packet.as_str()
        ));
    }
    if gate.redaction_class != packet.default_redaction {
        return Err(format!(
            "packet {} export gate redaction mismatch",
            packet.packet.as_str()
        ));
    }
    if gate.crosses_on_share.is_empty() {
        return Err(format!(
            "packet {} export gate hides what crosses the boundary",
            packet.packet.as_str()
        ));
    }
    if !gate.raw_payload_excluded {
        return Err(format!(
            "packet {} export gate must exclude raw payloads",
            packet.packet.as_str()
        ));
    }
    Ok(())
}

fn validate_reopen_anchor(anchor: &ReopenAnchor, where_: &str) -> Result<(), String> {
    if anchor.resolves_object != compute_resolves_object(anchor.anchor_class) {
        return Err(format!(
            "{where_} reopen resolves flag is not the computed value"
        ));
    }
    if anchor.anchor_class.requires_target() {
        if !anchor.target_ref.starts_with("aureline://") {
            return Err(format!("{where_} reopen anchor names no canonical object"));
        }
        if !anchor.placeholder_label.is_empty() {
            return Err(format!(
                "{where_} resolvable anchor carries a placeholder label"
            ));
        }
    } else {
        if !anchor.target_ref.is_empty() {
            return Err(format!("{where_} placeholder anchor names a live object"));
        }
        if anchor.placeholder_label.is_empty() {
            return Err(format!(
                "{where_} placeholder anchor hides what the object was"
            ));
        }
    }
    if anchor.note.is_empty() {
        return Err(format!("{where_} reopen anchor hides its note"));
    }
    Ok(())
}

fn validate_group(packet: PacketClass, group: &ObjectGroup) -> Result<(), String> {
    let where_ = || format!("packet {} group {}", packet.as_str(), group.group_id);
    if !group.object_ref.starts_with("aureline://") {
        return Err(format!("{} names no canonical object", where_()));
    }
    if group.object_label.is_empty() || group.what_changed.is_empty() {
        return Err(format!("{} hides its label/what-changed", where_()));
    }
    if group.blocker.requires_reason() && group.blocker_reason.is_empty() {
        return Err(format!("{} is blocked/waived without a reason", where_()));
    }
    if !group.blocker.requires_reason() && !group.blocker_reason.is_empty() {
        return Err(format!(
            "{} carries a blocker reason but is not blocked",
            where_()
        ));
    }
    if group.events.is_empty() {
        return Err(format!("{} has no events", where_()));
    }
    if group.evidence.is_empty() {
        return Err(format!("{} pins no evidence", where_()));
    }
    validate_reopen_anchor(&group.reopen_anchor, &where_())?;

    // The group's severity is the most severe of its events.
    let event_severities: Vec<SeverityClass> = group.events.iter().map(|e| e.severity).collect();
    if max_severity(&event_severities) != Some(group.severity) {
        return Err(format!(
            "{} severity is not the most severe of its events",
            where_()
        ));
    }

    // Events are ordered chronologically; the latest update is preserved.
    let mut prev_at: Option<&str> = None;
    for event in &group.events {
        if event.summary.is_empty() || event.at.is_empty() {
            return Err(format!(
                "{} event {} hides its summary/time",
                where_(),
                event.event_id
            ));
        }
        if let Some(prev) = prev_at {
            if event.at.as_str() < prev {
                return Err(format!("{} events are not chronological", where_()));
            }
        }
        prev_at = Some(event.at.as_str());
    }
    let latest = group
        .events
        .iter()
        .map(|e| e.at.as_str())
        .max()
        .unwrap_or("");
    if group.latest_update_at != latest {
        return Err(format!(
            "{} latest_update_at is not the latest event time",
            where_()
        ));
    }

    // Evidence keeps its storage class and freshness distinct.
    for ev in &group.evidence {
        if ev.label.is_empty() || ev.origin.is_empty() || ev.captured_at.is_empty() {
            return Err(format!(
                "{} evidence {} hides its label/origin/captured-at",
                where_(),
                ev.evidence_id
            ));
        }
        if !ev.evidence_ref.starts_with("aureline://") {
            return Err(format!(
                "{} evidence {} links a non-canonical ref",
                where_(),
                ev.evidence_id
            ));
        }
        if ev.is_live != ev.storage_class.is_live()
            || ev.can_refresh != ev.storage_class.can_refresh()
        {
            return Err(format!(
                "{} evidence {} live/refresh flags are not computed from its storage class",
                where_(),
                ev.evidence_id
            ));
        }
    }
    Ok(())
}

fn validate_question(packet: PacketClass, question: &UnresolvedQuestion) -> Result<(), String> {
    let where_ = || {
        format!(
            "packet {} question {}",
            packet.as_str(),
            question.question_id
        )
    };
    if question.question.is_empty() {
        return Err(format!("{} hides its text", where_()));
    }
    if question.owner.is_empty() {
        return Err(format!("{} hides its owner", where_()));
    }
    if !question.linked_object_ref.starts_with("aureline://") {
        return Err(format!("{} links no canonical object", where_()));
    }
    if question.next_safe_action.is_empty() {
        return Err(format!("{} hides the next safe action", where_()));
    }
    if question.status.requires_reason() && question.blocker_reason.is_empty() {
        return Err(format!("{} is blocked without a reason", where_()));
    }
    if !question.status.requires_reason() && !question.blocker_reason.is_empty() {
        return Err(format!(
            "{} carries a blocker reason but is not blocked",
            where_()
        ));
    }
    Ok(())
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}

// ---------------------------------------------------------------------------
// Roll-up and export computation.
// ---------------------------------------------------------------------------

/// Builds the computed roll-up of a packet.
///
/// Storage-class counts are kept separate so a snapshot is never merged into a
/// live-link count, and the headline answers what changed, what remains unresolved,
/// and the next safe action in distinct sentences.
pub fn compute_roll_up(packet: &ContinuityPacket) -> ContinuityRollUp {
    let groups = &packet.object_groups;
    let questions = &packet.unresolved_questions;

    let object_count = groups.len() as u32;
    let count_sev = |sev: SeverityClass| groups.iter().filter(|g| g.severity == sev).count() as u32;
    let sev1_count = count_sev(SeverityClass::Sev1Critical);
    let sev2_count = count_sev(SeverityClass::Sev2Major);
    let sev3_count = count_sev(SeverityClass::Sev3Minor);
    let sev4_count = count_sev(SeverityClass::Sev4Info);
    let blocked_object_count = groups
        .iter()
        .filter(|g| g.blocker.requires_reason())
        .count() as u32;

    let count_storage = |sc: StorageClass| {
        groups
            .iter()
            .flat_map(|g| g.evidence.iter())
            .filter(|e| e.storage_class == sc)
            .count() as u32
    };
    let live_link_count = count_storage(StorageClass::LiveLink);
    let cached_count = count_storage(StorageClass::Cached);
    let mirrored_count = count_storage(StorageClass::Mirrored);
    let snapshot_count = count_storage(StorageClass::Snapshot);

    let count_status =
        |st: QuestionStatus| questions.iter().filter(|q| q.status == st).count() as u32;
    let unresolved_open = count_status(QuestionStatus::Open);
    let unresolved_investigating = count_status(QuestionStatus::Investigating);
    let unresolved_blocked = count_status(QuestionStatus::Blocked);
    let unresolved_needs_decision = count_status(QuestionStatus::NeedsDecision);
    let unresolved_total = questions.len() as u32;

    let latest_update_at = groups
        .iter()
        .map(|g| g.latest_update_at.as_str())
        .max()
        .unwrap_or("")
        .to_owned();

    let highest_sev = groups
        .iter()
        .map(|g| g.severity)
        .max_by_key(|s| s.rank())
        .map(|s| s.as_str())
        .unwrap_or("none");

    let next_safe_action = questions
        .first()
        .map(|q| q.next_safe_action.clone())
        .unwrap_or_else(|| "No open questions; resume normal operation.".to_owned());

    let what_changed = format!(
        "Latest update {latest_update_at}; {blocked_object_count} of {object_count} objects \
         blocked; highest severity {highest_sev}."
    );
    let what_unresolved = format!(
        "{unresolved_total} unresolved — {unresolved_open} open, {unresolved_investigating} \
         investigating, {unresolved_blocked} blocked, {unresolved_needs_decision} awaiting a \
         decision."
    );
    let headline = format!(
        "{object_count} objects in scope ({sev1_count} sev1, {sev2_count} sev2, {sev3_count} \
         sev3, {sev4_count} sev4). What changed: {what_changed} What remains unresolved: \
         {what_unresolved} Next safe action: {next_safe_action} Evidence stays distinct — \
         {live_link_count} live links, {cached_count} cached, {mirrored_count} mirrored, \
         {snapshot_count} snapshot — never flattened into one blob."
    );

    ContinuityRollUp {
        object_count,
        sev1_count,
        sev2_count,
        sev3_count,
        sev4_count,
        blocked_object_count,
        live_link_count,
        cached_count,
        mirrored_count,
        snapshot_count,
        unresolved_open,
        unresolved_investigating,
        unresolved_blocked,
        unresolved_needs_decision,
        unresolved_total,
        latest_update_at,
        what_changed,
        what_unresolved,
        next_safe_action,
        headline,
    }
}

/// Builds the frozen handoff export of a packet.
fn compute_export(packet: &ContinuityPacket) -> ContinuityHandoffExport {
    let object_groups = packet.object_groups.clone();
    let object_count = object_groups.len() as u32;
    let roll_up = compute_roll_up(packet);
    ContinuityHandoffExport {
        export_id: format!("{}.export", packet.packet_id),
        packet: packet.packet,
        packet_id: packet.packet_id.clone(),
        kind: packet.kind,
        scope: packet.scope,
        share_posture: packet.share_posture,
        target_role: packet.target_role.clone(),
        redaction_class: packet.default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
        crosses_on_share: packet.export_gate.crosses_on_share.clone(),
        summary: format!(
            "Frozen {} export of {} — {object_count} canonical objects grouped by severity, each \
             with its latest update, major blocker, storage-distinct evidence, and a reopen anchor \
             that lands on the object or a truthful placeholder; unresolved questions and the next \
             safe action travel with it. Live, cached, mirrored, and snapshot evidence stay \
             distinct in the snapshot.",
            packet.kind.as_str(),
            packet.packet_id
        ),
        coverage_window: packet.coverage_window.clone(),
        object_count,
        object_groups,
        unresolved_questions: packet.unresolved_questions.clone(),
        reopen_anchor: packet.reopen_anchor.clone(),
        roll_up,
    }
}

/// Exports a continuity packet as a frozen handoff export.
pub fn export_packet(packet: &ContinuityPacket) -> ContinuityHandoffExport {
    compute_export(packet)
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical handoff/digest set.
///
/// Deterministic: the same bytes every call. Each evidence item's live/refresh
/// flags, each anchor's resolves flag, each group's latest-update time, each
/// packet's roll-up and export, and every invariant `holds` flag are computed from
/// the built data, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn handoff_digest_set() -> HandoffDigestSet {
    let packets = build_packets();
    let invariants = compute_invariants(&packets);

    HandoffDigestSet {
        record_kind: M5_HANDOFF_DIGESTS_RECORD_KIND.to_owned(),
        m5_handoff_digests_schema_version: M5_HANDOFF_DIGESTS_SCHEMA_VERSION,
        schema_ref: M5_HANDOFF_DIGESTS_SCHEMA_REF.to_owned(),
        set_id: M5_HANDOFF_DIGESTS_SET_ID.to_owned(),
        as_of: M5_HANDOFF_DIGESTS_AS_OF.to_owned(),
        summary:
            "The first real Aureline operator continuity packets — an outgoing-shift handoff, \
                  a client-facing handoff, a daily operations digest, and a private night-shift \
                  digest — that preserve, outside the live session, the same object identity, \
                  grouping, freshness, ownership, redaction, unresolved questions, and \
                  live-versus-cached-versus-mirrored-versus-snapshot truth the operator saw. \
                  Digests group by object and severity before chronology and keep each object's \
                  latest update and major blocker; every packet reopens onto the canonical object \
                  or a truthful placeholder rather than a generic dashboard, declares explicit \
                  scope and boundary truth before share/export, and freezes a snapshot export that \
                  never flattens the storage/freshness distinction, all bound to the \
                  operator-surface matrix."
                .to_owned(),
        matrix_ref: M5_HANDOFF_DIGESTS_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_HANDOFF_DIGESTS_MATRIX_RECORD_KIND.to_owned(),
        packet_kinds: token_defs(
            ContinuityPacketKind::ALL
                .iter()
                .map(|k| (k.as_str(), k.label())),
        ),
        storage_classes: token_defs(StorageClass::ALL.iter().map(|s| (s.as_str(), s.label()))),
        reopen_anchor_classes: token_defs(
            ReopenAnchorClass::ALL
                .iter()
                .map(|a| (a.as_str(), a.label())),
        ),
        severities: token_defs(SeverityClass::ALL.iter().map(|s| (s.as_str(), s.label()))),
        question_statuses: token_defs(QuestionStatus::ALL.iter().map(|s| (s.as_str(), s.label()))),
        target_audiences: token_defs(
            TargetAudienceClass::ALL
                .iter()
                .map(|a| (a.as_str(), a.label())),
        ),
        share_postures: token_defs(SharePosture::ALL.iter().map(|s| (s.as_str(), s.label()))),
        object_kinds: token_defs(ObjectKind::ALL.iter().map(|k| (k.as_str(), k.label()))),
        packets,
        invariants,
        raw_payload_excluded: true,
    }
}

fn token_defs<'a>(iter: impl Iterator<Item = (&'a str, &'a str)>) -> Vec<TokenDef> {
    iter.map(|(token, label)| TokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    })
    .collect()
}

// ---------------------------------------------------------------------------
// Builders.
// ---------------------------------------------------------------------------

/// All fields an evidence item carries, before id and flag computation.
struct EvidenceSpec<'a> {
    n: u32,
    label: &'a str,
    evidence_ref: &'a str,
    storage_class: StorageClass,
    freshness: FreshnessClass,
    origin: &'a str,
    captured_at: &'a str,
}

fn evidence(packet: PacketClass, group_n: u32, spec: EvidenceSpec<'_>) -> EvidenceItem {
    EvidenceItem {
        evidence_id: format!(
            "{}.group.{:02}.evidence.{:02}",
            packet.packet_id(),
            group_n,
            spec.n
        ),
        label: spec.label.to_owned(),
        evidence_ref: spec.evidence_ref.to_owned(),
        storage_class: spec.storage_class,
        freshness: spec.freshness,
        origin: spec.origin.to_owned(),
        captured_at: spec.captured_at.to_owned(),
        is_live: spec.storage_class.is_live(),
        can_refresh: spec.storage_class.can_refresh(),
    }
}

fn event(
    packet: PacketClass,
    group_n: u32,
    n: u32,
    at: &str,
    summary: &str,
    severity: SeverityClass,
) -> DigestEvent {
    DigestEvent {
        event_id: format!("{}.group.{:02}.event.{:02}", packet.packet_id(), group_n, n),
        at: at.to_owned(),
        summary: summary.to_owned(),
        severity,
    }
}

fn anchor(
    class: ReopenAnchorClass,
    target_ref: &str,
    placeholder_label: &str,
    note: &str,
) -> ReopenAnchor {
    ReopenAnchor {
        anchor_class: class,
        target_ref: target_ref.to_owned(),
        placeholder_label: placeholder_label.to_owned(),
        resolves_object: compute_resolves_object(class),
        note: note.to_owned(),
    }
}

/// All fields an object group carries, before id and latest-update computation.
struct GroupSpec<'a> {
    n: u32,
    object_ref: &'a str,
    object_kind: ObjectKind,
    object_label: &'a str,
    severity: SeverityClass,
    blocker: BlockerWaiverClass,
    blocker_reason: &'a str,
    freshness: FreshnessClass,
    what_changed: &'a str,
    reopen_anchor: ReopenAnchor,
    evidence: Vec<EvidenceItem>,
    events: Vec<DigestEvent>,
}

fn group(packet: PacketClass, spec: GroupSpec<'_>) -> ObjectGroup {
    let latest_update_at = spec
        .events
        .iter()
        .map(|e| e.at.clone())
        .max()
        .unwrap_or_default();
    ObjectGroup {
        group_id: format!("{}.group.{:02}", packet.packet_id(), spec.n),
        object_ref: spec.object_ref.to_owned(),
        object_kind: spec.object_kind,
        object_label: spec.object_label.to_owned(),
        severity: spec.severity,
        blocker: spec.blocker,
        blocker_reason: spec.blocker_reason.to_owned(),
        freshness: spec.freshness,
        latest_update_at,
        what_changed: spec.what_changed.to_owned(),
        reopen_anchor: spec.reopen_anchor,
        evidence: spec.evidence,
        events: spec.events,
    }
}

/// All fields an unresolved question carries, before id computation.
struct QuestionSpec<'a> {
    n: u32,
    question: &'a str,
    status: QuestionStatus,
    owner: &'a str,
    linked_object_ref: &'a str,
    next_safe_action: &'a str,
    blocker_reason: &'a str,
}

fn question(packet: PacketClass, spec: QuestionSpec<'_>) -> UnresolvedQuestion {
    UnresolvedQuestion {
        question_id: format!("{}.question.{:02}", packet.packet_id(), spec.n),
        question: spec.question.to_owned(),
        status: spec.status,
        owner: spec.owner.to_owned(),
        linked_object_ref: spec.linked_object_ref.to_owned(),
        next_safe_action: spec.next_safe_action.to_owned(),
        blocker_reason: spec.blocker_reason.to_owned(),
    }
}

fn export_gate(
    scope: ScopeClass,
    share_posture: SharePosture,
    redaction_class: RedactionClass,
    crosses_on_share: &str,
) -> ContinuityExportGate {
    ContinuityExportGate {
        scope,
        share_posture,
        requires_boundary_ack: share_posture.requires_boundary_ack(),
        crosses_on_share: crosses_on_share.to_owned(),
        redaction_class,
        raw_payload_excluded: true,
    }
}

fn default_actions() -> Vec<ContinuityAction> {
    [
        (
            ContinuityActionClass::OpenObject,
            "Open the canonical incident/support/admin/service object behind a group.",
        ),
        (
            ContinuityActionClass::OpenEvidence,
            "Open a linked evidence item; local-safe. The evidence shows its storage class — live \
             link, cached, mirrored, or snapshot — and its freshness.",
        ),
        (
            ContinuityActionClass::ReopenAtAnchor,
            "Reopen the packet at its anchor — the canonical object or a truthful placeholder, \
             never a generic dashboard; local-safe.",
        ),
        (
            ContinuityActionClass::CaptureAnswer,
            "Capture an answer or note against an unresolved question; local-safe.",
        ),
        (
            ContinuityActionClass::ExportSnapshot,
            "Export the packet as a frozen, machine-readable snapshot that preserves every truth \
             field including the storage distinction; local-safe.",
        ),
        (
            ContinuityActionClass::SharePacket,
            "Share the packet at its scope, after acknowledging exactly what crosses the boundary.",
        ),
    ]
    .into_iter()
    .map(|(action, summary)| ContinuityAction {
        action,
        label: action.label().to_owned(),
        local_safe: action.local_safe(),
        routes_to_canonical_object: action.routes_to_canonical_object(),
        summary: summary.to_owned(),
    })
    .collect()
}

/// Assembles a packet, computing its roll-up and frozen export.
#[allow(clippy::too_many_arguments)]
fn assemble_packet(
    packet: PacketClass,
    summary: &str,
    target_role: &str,
    target_audience: TargetAudienceClass,
    owning_role: &str,
    decision_right: &str,
    share_posture: SharePosture,
    consumed_by: Vec<ConsumerClass>,
    default_redaction: RedactionClass,
    coverage_window: CoverageWindow,
    boundary_note: &str,
    reopen_anchor: ReopenAnchor,
    export_gate: ContinuityExportGate,
    object_groups: Vec<ObjectGroup>,
    unresolved_questions: Vec<UnresolvedQuestion>,
) -> ContinuityPacket {
    let scope = share_posture.scope();
    let empty_window = CoverageWindow {
        start: String::new(),
        end: String::new(),
        label: String::new(),
    };
    let placeholder_anchor = anchor(
        ReopenAnchorClass::LiveObject,
        "aureline://placeholder",
        "",
        "x",
    );
    let mut built = ContinuityPacket {
        packet,
        packet_id: packet.packet_id(),
        label: packet.label().to_owned(),
        summary: summary.to_owned(),
        kind: packet.kind(),
        surface: packet.surface(),
        surface_id: packet.surface().surface_id(),
        target_role: target_role.to_owned(),
        target_audience,
        owning_role: owning_role.to_owned(),
        decision_right: decision_right.to_owned(),
        scope,
        share_posture,
        consumed_by,
        default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        coverage_window,
        boundary_note: boundary_note.to_owned(),
        reopen_anchor,
        export_gate,
        actions: default_actions(),
        object_groups,
        unresolved_questions,
        // Placeholders; replaced below once the packet is otherwise complete so the
        // roll-up and export see the final groups/questions.
        roll_up: ContinuityRollUp {
            object_count: 0,
            sev1_count: 0,
            sev2_count: 0,
            sev3_count: 0,
            sev4_count: 0,
            blocked_object_count: 0,
            live_link_count: 0,
            cached_count: 0,
            mirrored_count: 0,
            snapshot_count: 0,
            unresolved_open: 0,
            unresolved_investigating: 0,
            unresolved_blocked: 0,
            unresolved_needs_decision: 0,
            unresolved_total: 0,
            latest_update_at: String::new(),
            what_changed: String::new(),
            what_unresolved: String::new(),
            next_safe_action: String::new(),
            headline: String::new(),
        },
        export: ContinuityHandoffExport {
            export_id: String::new(),
            packet,
            packet_id: packet.packet_id(),
            kind: packet.kind(),
            scope,
            share_posture,
            target_role: target_role.to_owned(),
            redaction_class: default_redaction,
            live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
            crosses_on_share: String::new(),
            summary: String::new(),
            coverage_window: empty_window,
            object_count: 0,
            object_groups: Vec::new(),
            unresolved_questions: Vec::new(),
            reopen_anchor: placeholder_anchor,
            roll_up: ContinuityRollUp {
                object_count: 0,
                sev1_count: 0,
                sev2_count: 0,
                sev3_count: 0,
                sev4_count: 0,
                blocked_object_count: 0,
                live_link_count: 0,
                cached_count: 0,
                mirrored_count: 0,
                snapshot_count: 0,
                unresolved_open: 0,
                unresolved_investigating: 0,
                unresolved_blocked: 0,
                unresolved_needs_decision: 0,
                unresolved_total: 0,
                latest_update_at: String::new(),
                what_changed: String::new(),
                what_unresolved: String::new(),
                next_safe_action: String::new(),
                headline: String::new(),
            },
        },
    };
    built.roll_up = compute_roll_up(&built);
    built.export = compute_export(&built);
    built
}

fn build_packets() -> Vec<ContinuityPacket> {
    use ConsumerClass::*;
    use FreshnessClass as Fresh;
    use ReopenAnchorClass as Anchor;
    use SeverityClass as Sev;
    use StorageClass as Store;

    let evening = || CoverageWindow {
        start: "2026-06-21T20:00:00Z".to_owned(),
        end: "2026-06-22T04:00:00Z".to_owned(),
        label: "Evening on-call shift".to_owned(),
    };

    // ---- Packet 1: outgoing shift handoff. ----
    let outgoing =
        {
            let p = PacketClass::OutgoingShiftHandoff;
            let groups = vec![
            group(
                p,
                GroupSpec {
                    n: 1,
                    object_ref: "aureline://incident/inc-3001",
                    object_kind: ObjectKind::IncidentRecord,
                    object_label: "Auth latency incident",
                    severity: Sev::Sev1Critical,
                    blocker: BlockerWaiverClass::Blocked,
                    blocker_reason: "Connection-pool change is confirmed but the incident stays \
                                     open pending 4h stability.",
                    freshness: Fresh::Fresh,
                    what_changed: "Auth latency alerted and was mitigated via a confirmed managed \
                                   config change; still monitoring.",
                    reopen_anchor: anchor(
                        Anchor::LiveObject,
                        "aureline://incident/inc-3001",
                        "",
                        "Resolves to the live incident; the staged rollback stays ready.",
                    ),
                    evidence: vec![
                        evidence(
                            p,
                            1,
                            EvidenceSpec {
                                n: 1,
                                label: "Incident timeline (live)",
                                evidence_ref: "aureline://evidence/inc-3001-timeline",
                                storage_class: Store::LiveLink,
                                freshness: Fresh::Fresh,
                                origin: "incident_workspace",
                                captured_at: "2026-06-22T03:55:00Z",
                            },
                        ),
                        evidence(
                            p,
                            1,
                            EvidenceSpec {
                                n: 2,
                                label: "Auth latency metric slice (cached)",
                                evidence_ref: "aureline://evidence/inc-3001-latency-slice",
                                storage_class: Store::Cached,
                                freshness: Fresh::Recent,
                                origin: "metric_store",
                                captured_at: "2026-06-22T02:20:00Z",
                            },
                        ),
                    ],
                    events: vec![
                        event(
                            p,
                            1,
                            1,
                            "2026-06-21T20:30:00Z",
                            "Auth latency alert fired for the primary region",
                            Sev::Sev1Critical,
                        ),
                        event(
                            p,
                            1,
                            2,
                            "2026-06-21T22:15:00Z",
                            "Managed connection-pool config executed and confirmed",
                            Sev::Sev2Major,
                        ),
                    ],
                },
            ),
            group(
                p,
                GroupSpec {
                    n: 2,
                    object_ref: "aureline://support-case/case-8801",
                    object_kind: ObjectKind::SupportCase,
                    object_label: "Hotfix canary case",
                    severity: Sev::Sev2Major,
                    blocker: BlockerWaiverClass::Blocked,
                    blocker_reason: "Canary deploy is held by an announced read-only window until \
                                     05:00Z.",
                    freshness: Fresh::Recent,
                    what_changed: "Canary hotfix deploy failed closed; the case is held until the \
                                   read-only window lifts.",
                    reopen_anchor: anchor(
                        Anchor::MirroredOfflineView,
                        "aureline://support-case/case-8801",
                        "",
                        "Resolves to the last-synced offline mirror; refresh on reconnect before \
                         acting.",
                    ),
                    evidence: vec![
                        evidence(
                            p,
                            2,
                            EvidenceSpec {
                                n: 1,
                                label: "Last-synced case view (mirrored)",
                                evidence_ref: "aureline://evidence/case-8801-mirror",
                                storage_class: Store::Mirrored,
                                freshness: Fresh::Stale,
                                origin: "offline_mirror",
                                captured_at: "2026-06-21T23:40:00Z",
                            },
                        ),
                        evidence(
                            p,
                            2,
                            EvidenceSpec {
                                n: 2,
                                label: "Canary deploy failure (snapshot)",
                                evidence_ref: "aureline://evidence/case-8801-deploy-fail",
                                storage_class: Store::Snapshot,
                                freshness: Fresh::Recent,
                                origin: "release_pipeline",
                                captured_at: "2026-06-21T21:05:00Z",
                            },
                        ),
                    ],
                    events: vec![event(
                        p,
                        2,
                        1,
                        "2026-06-21T21:00:00Z",
                        "Canary hotfix deploy failed closed",
                        Sev::Sev2Major,
                    )],
                },
            ),
        ];
            let questions = vec![
                question(
                    p,
                    QuestionSpec {
                        n: 1,
                        question: "Will the connection-pool ceiling hold past the morning peak?",
                        status: QuestionStatus::Open,
                        owner: "incoming_on_call",
                        linked_object_ref: "aureline://incident/inc-3001",
                        next_safe_action:
                            "Watch the auth-latency tile; if it re-reddens, execute the \
                                       prepared rollback on the approve-and-confirm path.",
                        blocker_reason: "",
                    },
                ),
                question(
                    p,
                    QuestionSpec {
                        n: 2,
                        question: "Can the canary hotfix redeploy once the read-only window lifts?",
                        status: QuestionStatus::Blocked,
                        owner: "release_operator",
                        linked_object_ref: "aureline://support-case/case-8801",
                        next_safe_action:
                            "Hold; re-attempt only after the window lifts and a fresh \
                                       approval is captured.",
                        blocker_reason:
                            "The canary pipeline is in an announced read-only window until \
                                     05:00Z.",
                    },
                ),
            ];
            assemble_packet(
            p,
            "End-of-shift handoff to the incoming on-call: the open auth incident and the held \
             canary case, each with its latest update, major blocker, storage-distinct evidence, \
             and a reopen anchor, plus the two unresolved questions and their next safe actions.",
            "incoming_on_call",
            TargetAudienceClass::NextOperatorShift,
            "outgoing_on_call",
            "incident_commander",
            SharePosture::WorkspaceShared,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport, ManagedService],
            RedactionClass::OperatorOnlyRestricted,
            evening(),
            "Reopening lands on the live incident or the labeled offline mirror, never a generic \
             dashboard; cached, mirrored, and snapshot evidence are labeled, never flattened.",
            anchor(
                Anchor::LiveObject,
                "aureline://incident/inc-3001",
                "",
                "Reopens on the highest-severity object — the live auth incident — with the rest \
                 of the shift one step away.",
            ),
            export_gate(
                ScopeClass::SharedTeam,
                SharePosture::WorkspaceShared,
                RedactionClass::OperatorOnlyRestricted,
                "Object identity, grouping, severities, latest updates, blockers, evidence refs \
                 with their storage class and freshness, unresolved questions, and ownership \
                 become visible to the workspace; raw provider payloads, credentials, and \
                 endpoint URLs never cross.",
            ),
            groups,
            questions,
        )
        };

    // ---- Packet 2: client status handoff. ----
    let client = {
        let p = PacketClass::ClientStatusHandoff;
        let groups = vec![
            group(
                p,
                GroupSpec {
                    n: 1,
                    object_ref: "aureline://incident/inc-3001",
                    object_kind: ObjectKind::IncidentRecord,
                    object_label: "Auth latency incident (client view)",
                    severity: Sev::Sev2Major,
                    blocker: BlockerWaiverClass::None,
                    blocker_reason: "",
                    freshness: Fresh::Recent,
                    what_changed: "Mitigation applied; service monitoring. No customer action \
                                   required.",
                    reopen_anchor: anchor(
                        Anchor::CachedObjectSnapshot,
                        "aureline://incident/inc-3001",
                        "",
                        "Resolves to the cached client-safe summary of the incident; open the live \
                         incident internally for full detail.",
                    ),
                    evidence: vec![evidence(
                        p,
                        1,
                        EvidenceSpec {
                            n: 1,
                            label: "Client status summary (cached)",
                            evidence_ref: "aureline://evidence/inc-3001-client-summary",
                            storage_class: Store::Cached,
                            freshness: Fresh::Recent,
                            origin: "status_page_draft",
                            captured_at: "2026-06-22T00:10:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        1,
                        1,
                        "2026-06-21T22:15:00Z",
                        "Mitigation applied; service monitoring",
                        Sev::Sev2Major,
                    )],
                },
            ),
            group(
                p,
                GroupSpec {
                    n: 2,
                    object_ref: "aureline://release-gate/rel-204",
                    object_kind: ObjectKind::ReleaseGate,
                    object_label: "Evening release gate",
                    severity: Sev::Sev3Minor,
                    blocker: BlockerWaiverClass::None,
                    blocker_reason: "",
                    freshness: Fresh::VeryStale,
                    what_changed: "Release gate passed and was archived during the shift.",
                    reopen_anchor: anchor(
                        Anchor::TruthfulPlaceholder,
                        "",
                        "Release gate rel-204 — archived after this shift; no live object remains. \
                         Open the archived decision snapshot instead of a dashboard.",
                        "The gate object was archived; reopening lands on a labeled placeholder, \
                         never a generic home screen.",
                    ),
                    evidence: vec![evidence(
                        p,
                        2,
                        EvidenceSpec {
                            n: 1,
                            label: "Release gate decision (snapshot)",
                            evidence_ref: "aureline://evidence/rel-204-decision",
                            storage_class: Store::Snapshot,
                            freshness: Fresh::VeryStale,
                            origin: "release_evidence",
                            captured_at: "2026-06-21T20:12:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        2,
                        1,
                        "2026-06-21T20:10:00Z",
                        "Release gate passed and archived",
                        Sev::Sev3Minor,
                    )],
                },
            ),
        ];
        let questions = vec![question(
            p,
            QuestionSpec {
                n: 1,
                question: "Does the customer need a written incident summary?",
                status: QuestionStatus::NeedsDecision,
                owner: "customer_success_lead",
                linked_object_ref: "aureline://incident/inc-3001",
                next_safe_action: "Decide with the incident commander before sending anything \
                                   external; share only the metadata-safe summary, never raw \
                                   internal evidence.",
                blocker_reason: "",
            },
        )];
        assemble_packet(
            p,
            "Client-facing status handoff: a metadata-safe view of the auth incident and the \
             archived evening release gate, with the one decision the customer-success lead must \
             make. The archived gate reopens on a truthful placeholder, not a dashboard.",
            "customer_success_lead",
            TargetAudienceClass::ClientFacing,
            "outgoing_on_call",
            "incident_commander",
            SharePosture::WorkspaceShared,
            vec![ShellUi, CliHeadless, SupportExport, CompanionBrowser],
            RedactionClass::MetadataSafeDefault,
            evening(),
            "A client handoff stays metadata-safe and never impersonates internal detail; an \
             archived object reopens on a labeled placeholder rather than a generic home screen.",
            anchor(
                Anchor::CachedObjectSnapshot,
                "aureline://incident/inc-3001",
                "",
                "Reopens on the cached client-safe incident summary; the archived gate's \
                 placeholder is one step away.",
            ),
            export_gate(
                ScopeClass::SharedTeam,
                SharePosture::WorkspaceShared,
                RedactionClass::MetadataSafeDefault,
                "Metadata-safe object labels, severities, latest updates, and the open decision \
                 cross to the client-facing surface; internal evidence bodies, raw payloads, \
                 credentials, and endpoint URLs never cross.",
            ),
            groups,
            questions,
        )
    };

    // ---- Packet 3: daily operations digest. ----
    let daily = {
        let p = PacketClass::DailyOperationsDigest;
        let window = CoverageWindow {
            start: "2026-06-22T00:00:00Z".to_owned(),
            end: "2026-06-23T00:00:00Z".to_owned(),
            label: "Daily operations digest".to_owned(),
        };
        let groups = vec![
            group(
                p,
                GroupSpec {
                    n: 1,
                    object_ref: "aureline://incident/inc-3001",
                    object_kind: ObjectKind::IncidentRecord,
                    object_label: "Auth latency incident",
                    severity: Sev::Sev1Critical,
                    blocker: BlockerWaiverClass::Blocked,
                    blocker_reason: "Incident stays open pending 4h stability; rollback staged.",
                    freshness: Fresh::Fresh,
                    what_changed:
                        "Auth incident carried from the prior shift; mitigated and stable \
                                   for 2h.",
                    reopen_anchor: anchor(
                        Anchor::LiveObject,
                        "aureline://incident/inc-3001",
                        "",
                        "Resolves to the live incident; the staged rollback stays ready.",
                    ),
                    evidence: vec![evidence(
                        p,
                        1,
                        EvidenceSpec {
                            n: 1,
                            label: "Incident timeline (live)",
                            evidence_ref: "aureline://evidence/inc-3001-timeline",
                            storage_class: Store::LiveLink,
                            freshness: Fresh::Fresh,
                            origin: "incident_workspace",
                            captured_at: "2026-06-22T02:05:00Z",
                        },
                    )],
                    events: vec![
                        event(
                            p,
                            1,
                            1,
                            "2026-06-22T00:30:00Z",
                            "Auth incident still mitigating from the prior shift",
                            Sev::Sev1Critical,
                        ),
                        event(
                            p,
                            1,
                            2,
                            "2026-06-22T02:00:00Z",
                            "Auth latency stable for 2h; monitoring",
                            Sev::Sev3Minor,
                        ),
                    ],
                },
            ),
            group(
                p,
                GroupSpec {
                    n: 2,
                    object_ref: "aureline://admin-approval/req-501",
                    object_kind: ObjectKind::AdminApprovalRequest,
                    object_label: "Access review",
                    severity: Sev::Sev2Major,
                    blocker: BlockerWaiverClass::Blocked,
                    blocker_reason: "Grant held: policy forbids reviewer self-approval; residency \
                                     confirmation pending.",
                    freshness: Fresh::Recent,
                    what_changed: "Access review opened; the grant is held pending a separate \
                                   security-owner approval.",
                    reopen_anchor: anchor(
                        Anchor::CachedObjectSnapshot,
                        "aureline://admin-approval/req-501",
                        "",
                        "Resolves to the cached review context; refresh against the live approval \
                         before granting.",
                    ),
                    evidence: vec![evidence(
                        p,
                        2,
                        EvidenceSpec {
                            n: 1,
                            label: "Access review context (cached)",
                            evidence_ref: "aureline://evidence/req-501-context",
                            storage_class: Store::Cached,
                            freshness: Fresh::Recent,
                            origin: "admin_console",
                            captured_at: "2026-06-22T06:05:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        2,
                        1,
                        "2026-06-22T06:00:00Z",
                        "Access review opened; grant held (forbidden self-approval)",
                        Sev::Sev2Major,
                    )],
                },
            ),
            group(
                p,
                GroupSpec {
                    n: 3,
                    object_ref: "aureline://service-health/svc-auth",
                    object_kind: ObjectKind::ServiceHealthRecord,
                    object_label: "Auth service health",
                    severity: Sev::Sev3Minor,
                    blocker: BlockerWaiverClass::None,
                    blocker_reason: "",
                    freshness: Fresh::Recent,
                    what_changed: "Auth service recovered to nominal after the incident \
                                   mitigation.",
                    reopen_anchor: anchor(
                        Anchor::MirroredOfflineView,
                        "aureline://service-health/svc-auth",
                        "",
                        "Resolves to the last-synced service-health mirror; refresh on reconnect.",
                    ),
                    evidence: vec![evidence(
                        p,
                        3,
                        EvidenceSpec {
                            n: 1,
                            label: "Service-health card (mirrored)",
                            evidence_ref: "aureline://evidence/svc-auth-health",
                            storage_class: Store::Mirrored,
                            freshness: Fresh::Recent,
                            origin: "service_health_feed",
                            captured_at: "2026-06-22T01:05:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        3,
                        1,
                        "2026-06-22T01:00:00Z",
                        "Auth service recovered to nominal",
                        Sev::Sev3Minor,
                    )],
                },
            ),
        ];
        let questions = vec![
            question(
                p,
                QuestionSpec {
                    n: 1,
                    question: "Is the auth incident safe to close?",
                    status: QuestionStatus::Open,
                    owner: "operations_lead",
                    linked_object_ref: "aureline://incident/inc-3001",
                    next_safe_action: "Keep open until 4h stable; the staged rollback stays ready.",
                    blocker_reason: "",
                },
            ),
            question(
                p,
                QuestionSpec {
                    n: 2,
                    question: "Who approves the held access grant?",
                    status: QuestionStatus::Investigating,
                    owner: "security_owner",
                    linked_object_ref: "aureline://admin-approval/req-501",
                    next_safe_action: "Route to the security owner; the reviewer cannot \
                                       self-approve.",
                    blocker_reason: "",
                },
            ),
        ];
        assemble_packet(
            p,
            "The daily operations digest: every object touched in the day grouped by severity \
             before chronology — the open auth incident first, then the held access review, then \
             the recovered auth service — each with its latest update and major blocker, plus the \
             day's unresolved questions.",
            "operations_lead",
            TargetAudienceClass::TeamWide,
            "operations_lead",
            "operations_lead",
            SharePosture::OrgShared,
            vec![ShellUi, CliHeadless, IncidentWorkspace, AdminQueue, SupportExport, ManagedService],
            RedactionClass::InternalSupportRestricted,
            window,
            "The digest groups by object and severity before chronology and never claims more \
             coverage than its window; each object reopens on its canonical record or a labeled \
             offline mirror.",
            anchor(
                Anchor::LiveObject,
                "aureline://incident/inc-3001",
                "",
                "Reopens on the highest-severity object — the live auth incident — with the rest \
                 of the day's objects grouped behind it.",
            ),
            export_gate(
                ScopeClass::ManagedOrg,
                SharePosture::OrgShared,
                RedactionClass::InternalSupportRestricted,
                "Object identity, severities, grouping, latest updates, blockers, evidence refs \
                 with storage class and freshness, and unresolved questions become visible org-wide \
                 under managed governance; raw payloads, credentials, and endpoint URLs never \
                 cross.",
            ),
            groups,
            questions,
        )
    };

    // ---- Packet 4: night-shift digest (private). ----
    let night = {
        let p = PacketClass::NightShiftDigest;
        let window = CoverageWindow {
            start: "2026-06-22T00:00:00Z".to_owned(),
            end: "2026-06-22T08:00:00Z".to_owned(),
            label: "Night shift digest".to_owned(),
        };
        let groups = vec![
            group(
                p,
                GroupSpec {
                    n: 1,
                    object_ref: "aureline://support-case/case-8802",
                    object_kind: ObjectKind::SupportCase,
                    object_label: "New overnight case",
                    severity: Sev::Sev2Major,
                    blocker: BlockerWaiverClass::None,
                    blocker_reason: "",
                    freshness: Fresh::Fresh,
                    what_changed: "New case opened overnight; repro captured, no page raised.",
                    reopen_anchor: anchor(
                        Anchor::LiveObject,
                        "aureline://support-case/case-8802",
                        "",
                        "Resolves to the live case; the repro is attached as a snapshot.",
                    ),
                    evidence: vec![evidence(
                        p,
                        1,
                        EvidenceSpec {
                            n: 1,
                            label: "Overnight repro (snapshot)",
                            evidence_ref: "aureline://evidence/case-8802-repro",
                            storage_class: Store::Snapshot,
                            freshness: Fresh::Fresh,
                            origin: "support_intake",
                            captured_at: "2026-06-22T03:35:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        1,
                        1,
                        "2026-06-22T03:30:00Z",
                        "New case opened; repro captured",
                        Sev::Sev2Major,
                    )],
                },
            ),
            group(
                p,
                GroupSpec {
                    n: 2,
                    object_ref: "aureline://review-item/rev-77",
                    object_kind: ObjectKind::ReviewItem,
                    object_label: "Morning review note",
                    severity: Sev::Sev4Info,
                    blocker: BlockerWaiverClass::None,
                    blocker_reason: "",
                    freshness: Fresh::Stale,
                    what_changed: "A review note was left for the morning owner.",
                    reopen_anchor: anchor(
                        Anchor::CachedObjectSnapshot,
                        "aureline://review-item/rev-77",
                        "",
                        "Resolves to the cached review note; refresh against the live review item \
                         in the morning.",
                    ),
                    evidence: vec![evidence(
                        p,
                        2,
                        EvidenceSpec {
                            n: 1,
                            label: "Review note (cached)",
                            evidence_ref: "aureline://evidence/rev-77-note",
                            storage_class: Store::Cached,
                            freshness: Fresh::Stale,
                            origin: "review_queue",
                            captured_at: "2026-06-22T05:05:00Z",
                        },
                    )],
                    events: vec![event(
                        p,
                        2,
                        1,
                        "2026-06-22T05:00:00Z",
                        "Review note left for the morning",
                        Sev::Sev4Info,
                    )],
                },
            ),
        ];
        let questions = vec![question(
            p,
            QuestionSpec {
                n: 1,
                question: "Should case-8802 page the daytime owner?",
                status: QuestionStatus::Open,
                owner: "night_on_call",
                linked_object_ref: "aureline://support-case/case-8802",
                next_safe_action: "No page overnight unless it escalates to Sev1; carry it into \
                                   the morning digest.",
                blocker_reason: "",
            },
        )];
        assemble_packet(
            p,
            "A private night-shift digest the on-call keeps as a local draft: the new overnight \
             case and a review note left for the morning, grouped by severity, with the one open \
             question about paging. Stays on the host until the operator changes scope.",
            "night_on_call",
            TargetAudienceClass::NextOperatorShift,
            "night_on_call",
            "night_on_call",
            SharePosture::Private,
            vec![ShellUi, CliHeadless, SupportExport],
            RedactionClass::PrivateTriageOnly,
            window,
            "A private digest stays on the host; reopening still lands on the canonical case or a \
             labeled cached note, never a generic dashboard.",
            anchor(
                Anchor::LiveObject,
                "aureline://support-case/case-8802",
                "",
                "Reopens on the highest-severity object — the new overnight case — with the review \
                 note grouped behind it.",
            ),
            export_gate(
                ScopeClass::LocalPrivate,
                SharePosture::Private,
                RedactionClass::PrivateTriageOnly,
                "Stays on this host as a private draft; nothing crosses a share boundary until the \
                 operator changes scope. Export produces a local snapshot only.",
            ),
            groups,
            questions,
        )
    };

    vec![outgoing, client, daily, night]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn compute_invariants(packets: &[ContinuityPacket]) -> Vec<ContinuityInvariant> {
    let all_groups: Vec<&ObjectGroup> = packets
        .iter()
        .flat_map(|p| p.object_groups.iter())
        .collect();
    let all_evidence: Vec<&EvidenceItem> =
        all_groups.iter().flat_map(|g| g.evidence.iter()).collect();
    let all_questions: Vec<&UnresolvedQuestion> = packets
        .iter()
        .flat_map(|p| p.unresolved_questions.iter())
        .collect();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

    let surface_binding = packets.iter().all(|p| {
        p.kind == p.packet.kind()
            && p.surface == p.packet.surface()
            && p.surface_id == p.surface.surface_id()
            && matrix.surface(p.surface).is_some()
    });

    let both_surfaces_present = ContinuityPacketKind::ALL
        .iter()
        .all(|kind| packets.iter().any(|p| p.kind == *kind));

    let canonical_object_linkage = packets.iter().all(|p| {
        p.object_groups.iter().all(|g| {
            g.object_ref.starts_with("aureline://")
                && g.evidence
                    .iter()
                    .all(|e| e.evidence_ref.starts_with("aureline://"))
                && anchor_is_canonical(&g.reopen_anchor)
        }) && p
            .unresolved_questions
            .iter()
            .all(|q| q.linked_object_ref.starts_with("aureline://"))
            && anchor_is_canonical(&p.reopen_anchor)
    });

    // The central guardrail: live/cached/mirrored/snapshot evidence is never flattened.
    let storage_class_not_flattened = StorageClass::ALL
        .iter()
        .all(|sc| all_evidence.iter().any(|e| e.storage_class == *sc))
        && packets.iter().all(|p| {
            let r = &p.roll_up;
            r.live_link_count == count_storage_in(p, StorageClass::LiveLink)
                && r.cached_count == count_storage_in(p, StorageClass::Cached)
                && r.mirrored_count == count_storage_in(p, StorageClass::Mirrored)
                && r.snapshot_count == count_storage_in(p, StorageClass::Snapshot)
        });

    let evidence_freshness_preserved = all_evidence.iter().all(|e| {
        e.is_live == e.storage_class.is_live()
            && e.can_refresh == e.storage_class.can_refresh()
            && !e.origin.is_empty()
            && !e.captured_at.is_empty()
    });

    let digests_group_by_severity_before_chronology = packets
        .iter()
        .filter(|p| p.kind == ContinuityPacketKind::ShiftDigest)
        .all(|p| {
            groups_severity_ordered(&p.object_groups)
                && p.object_groups.iter().all(events_chronological)
        });

    // All packets (handoff bundles too) keep groups severity-ordered and events chronological.
    let all_packets_grouped_and_chronological = packets.iter().all(|p| {
        groups_severity_ordered(&p.object_groups)
            && p.object_groups.iter().all(events_chronological)
    });

    let latest_update_and_blockers_preserved = packets.iter().all(|p| {
        p.object_groups.iter().all(|g| {
            let latest = g.events.iter().map(|e| e.at.as_str()).max().unwrap_or("");
            g.latest_update_at == latest
                && max_severity(&g.events.iter().map(|e| e.severity).collect::<Vec<_>>())
                    == Some(g.severity)
                && (!g.blocker.requires_reason() || !g.blocker_reason.is_empty())
        }) && {
            let roll_latest = p
                .object_groups
                .iter()
                .map(|g| g.latest_update_at.as_str())
                .max()
                .unwrap_or("");
            p.roll_up.latest_update_at == roll_latest
        }
    });

    let reopen_lands_on_object_or_placeholder = packets.iter().all(|p| {
        anchor_well_formed(&p.reopen_anchor)
            && p.object_groups
                .iter()
                .all(|g| anchor_well_formed(&g.reopen_anchor))
    });

    let reopen_anchor_classes_distinct = ReopenAnchorClass::ALL.iter().all(|class| {
        packets.iter().any(|p| {
            p.reopen_anchor.anchor_class == *class
                || p.object_groups
                    .iter()
                    .any(|g| g.reopen_anchor.anchor_class == *class)
        })
    });

    let unresolved_questions_answerable =
        all_questions.iter().all(|q| {
            !q.question.is_empty()
                && !q.owner.is_empty()
                && q.linked_object_ref.starts_with("aureline://")
                && !q.next_safe_action.is_empty()
                && (!q.status.requires_reason() || !q.blocker_reason.is_empty())
        }) && packets.iter().all(|p| !p.unresolved_questions.is_empty());

    let scope_boundary_truth = packets.iter().all(|p| {
        p.share_posture.scope() == p.scope
            && p.export_gate.scope == p.scope
            && p.export_gate.share_posture == p.share_posture
            && p.export_gate.requires_boundary_ack == p.share_posture.requires_boundary_ack()
            && p.export_gate.redaction_class == p.default_redaction
            && !p.export_gate.crosses_on_share.is_empty()
            && p.export_gate.raw_payload_excluded
    });

    let share_postures_distinct = SharePosture::ALL
        .iter()
        .all(|posture| packets.iter().any(|p| p.share_posture == *posture));

    let ownership_present = packets.iter().all(|p| {
        !p.owning_role.is_empty() && !p.decision_right.is_empty() && !p.target_role.is_empty()
    });

    let export_parity = packets.iter().all(|p| {
        compute_export(p) == p.export
            && p.export.live_vs_snapshot == LiveSnapshotClass::SnapshotOnly
    });

    let export_preserves_storage_distinction = packets.iter().all(|p| {
        p.export.object_groups == p.object_groups
            && p.export.unresolved_questions == p.unresolved_questions
            && p.export.reopen_anchor == p.reopen_anchor
            && p.export.roll_up == compute_roll_up(p)
    });

    let roll_up_answers_three_questions = packets.iter().all(|p| {
        let r = &p.roll_up;
        !r.what_changed.is_empty()
            && !r.what_unresolved.is_empty()
            && !r.next_safe_action.is_empty()
            && r.headline.contains("What changed")
            && r.headline.contains("What remains unresolved")
            && r.headline.contains("Next safe action")
            && r.headline.contains("never flattened")
            && r == &compute_roll_up(p)
    });

    let first_real_packets_present = PacketClass::ALL
        .iter()
        .all(|c| packets.iter().any(|p| p.packet == *c));

    let object_kinds_distinct = ObjectKind::ALL
        .iter()
        .all(|kind| all_groups.iter().any(|g| g.object_kind == *kind));

    let severities_distinct = SeverityClass::ALL
        .iter()
        .all(|sev| all_groups.iter().any(|g| g.severity == *sev));

    let stable_ids_unique = all_unique(packets.iter().map(|p| p.packet_id.as_str()))
        && all_unique(
            packets
                .iter()
                .flat_map(|p| p.object_groups.iter().map(|g| g.group_id.as_str())),
        )
        && all_unique(packets.iter().flat_map(|p| {
            p.object_groups
                .iter()
                .flat_map(|g| g.evidence.iter().map(|e| e.evidence_id.as_str()))
        }))
        && all_unique(packets.iter().flat_map(|p| {
            p.unresolved_questions
                .iter()
                .map(|q| q.question_id.as_str())
        }));

    vec![
        invariant(
            "continuity.surface_binding",
            "Every packet binds its operator-surface matrix family (handoff bundle or shift \
             digest) by the matrix's own surface id.",
            surface_binding,
        ),
        invariant(
            "continuity.both_surfaces_present",
            "The set proves both the handoff-bundle and the shift-digest surfaces.",
            both_surfaces_present,
        ),
        invariant(
            "continuity.canonical_object_linkage",
            "Every group object, evidence ref, question link, and resolvable reopen target is a \
             canonical aureline:// handle.",
            canonical_object_linkage,
        ),
        invariant(
            "continuity.storage_class_not_flattened",
            "The set proves all four storage classes — live link, cached, mirrored, and snapshot — \
             and every roll-up counts them separately, never flattening them into one blob.",
            storage_class_not_flattened,
        ),
        invariant(
            "continuity.evidence_freshness_preserved",
            "Every evidence item carries an origin, a captured-at, and live/refresh flags computed \
             from its storage class.",
            evidence_freshness_preserved,
        ),
        invariant(
            "continuity.digests_group_by_severity_before_chronology",
            "Every digest orders its groups by severity (most severe first) and orders events \
             chronologically only within a group.",
            digests_group_by_severity_before_chronology,
        ),
        invariant(
            "continuity.all_packets_grouped_and_chronological",
            "Every packet — handoff bundle and digest alike — keeps its groups severity-ordered \
             and its within-group events chronological.",
            all_packets_grouped_and_chronological,
        ),
        invariant(
            "continuity.latest_update_and_blockers_preserved",
            "Every group preserves its latest update time and its blocker reason, its severity is \
             the most severe of its events, and the roll-up's latest update is the newest group's.",
            latest_update_and_blockers_preserved,
        ),
        invariant(
            "continuity.reopen_lands_on_object_or_placeholder",
            "Every reopen anchor resolves to a canonical object or a truthful placeholder that \
             names what the object was — never a generic dashboard.",
            reopen_lands_on_object_or_placeholder,
        ),
        invariant(
            "continuity.reopen_anchor_classes_distinct",
            "The set proves all four reopen-anchor classes — live object, cached snapshot, \
             mirrored offline view, and truthful placeholder.",
            reopen_anchor_classes_distinct,
        ),
        invariant(
            "continuity.unresolved_questions_answerable",
            "Every packet carries unresolved questions, each naming an owner, a canonical object, \
             and a next safe action, with a reason when blocked.",
            unresolved_questions_answerable,
        ),
        invariant(
            "continuity.scope_boundary_truth",
            "Every packet declares a scope and a matching export gate that names what crosses the \
             boundary on share/export and requires acknowledgement above private scope.",
            scope_boundary_truth,
        ),
        invariant(
            "continuity.share_postures_distinct",
            "The set proves a private, a workspace-shared, and an org-shared packet.",
            share_postures_distinct,
        ),
        invariant(
            "continuity.ownership_present",
            "Every packet names an owning role, a decision right, and the target role it is handed \
             to.",
            ownership_present,
        ),
        invariant(
            "continuity.export_parity",
            "Each packet's frozen export equals re-exporting it and is labeled snapshot_only.",
            export_parity,
        ),
        invariant(
            "continuity.export_preserves_storage_distinction",
            "Each export preserves the exact object groups (with every evidence item's storage \
             class and freshness), unresolved questions, reopen anchor, and roll-up.",
            export_preserves_storage_distinction,
        ),
        invariant(
            "continuity.roll_up_answers_three_questions",
            "Each roll-up answers what changed, what remains unresolved, and the next safe action, \
             and its headline keeps the storage classes distinct.",
            roll_up_answers_three_questions,
        ),
        invariant(
            "continuity.first_real_packets_present",
            "The outgoing-shift handoff, client handoff, daily operations digest, and night-shift \
             digest are all present.",
            first_real_packets_present,
        ),
        invariant(
            "continuity.object_kinds_distinct",
            "The set proves all six canonical object kinds across its groups.",
            object_kinds_distinct,
        ),
        invariant(
            "continuity.severities_distinct",
            "The set proves all four severities across its groups.",
            severities_distinct,
        ),
        invariant(
            "continuity.stable_ids_unique",
            "Packet, group, evidence, and question ids are unique.",
            stable_ids_unique,
        ),
    ]
}

fn invariant(id: &str, statement: &str, holds: bool) -> ContinuityInvariant {
    ContinuityInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn count_storage_in(packet: &ContinuityPacket, sc: StorageClass) -> u32 {
    packet
        .object_groups
        .iter()
        .flat_map(|g| g.evidence.iter())
        .filter(|e| e.storage_class == sc)
        .count() as u32
}

fn anchor_is_canonical(anchor: &ReopenAnchor) -> bool {
    if anchor.anchor_class.resolves_object() {
        anchor.target_ref.starts_with("aureline://")
    } else {
        anchor.target_ref.is_empty()
    }
}

fn anchor_well_formed(anchor: &ReopenAnchor) -> bool {
    anchor.resolves_object == compute_resolves_object(anchor.anchor_class)
        && !anchor.note.is_empty()
        && if anchor.anchor_class.requires_target() {
            anchor.target_ref.starts_with("aureline://") && anchor.placeholder_label.is_empty()
        } else {
            anchor.target_ref.is_empty() && !anchor.placeholder_label.is_empty()
        }
}

fn groups_severity_ordered(groups: &[ObjectGroup]) -> bool {
    groups
        .windows(2)
        .all(|w| w[0].severity.rank() >= w[1].severity.rank())
}

fn events_chronological(group: &ObjectGroup) -> bool {
    group.events.windows(2).all(|w| w[0].at <= w[1].at)
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the handoff/digest set as human-readable lines for headless / support
/// surfaces that cannot show the live UI.
pub fn handoff_digest_lines(set: &HandoffDigestSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator handoff bundles & shift digests — {} packets, {} storage classes, {} invariants \
         (as of {})",
        set.packets.len(),
        set.storage_classes.len(),
        set.invariants.len(),
        set.as_of
    ));
    lines.push(format!(
        "bound matrix: {} ({})",
        set.matrix_ref, set.matrix_record_kind
    ));
    for packet in &set.packets {
        lines.push(String::new());
        lines.push(format!(
            "[{}] {} — kind {} — surface {} — target {} ({}) — scope {} / {} — window {}",
            packet.packet.as_str(),
            packet.label,
            packet.kind.as_str(),
            packet.surface_id,
            packet.target_role,
            packet.target_audience.as_str(),
            scope_token(packet.scope),
            packet.share_posture.as_str(),
            packet.coverage_window.label,
        ));
        lines.push(format!(
            "  reopen: {} -> {}",
            packet.reopen_anchor.anchor_class.as_str(),
            if packet.reopen_anchor.target_ref.is_empty() {
                "<placeholder>"
            } else {
                &packet.reopen_anchor.target_ref
            },
        ));
        for g in &packet.object_groups {
            lines.push(format!(
                "  [{}] {} | {} | blocker={} | fresh={} | latest={} | reopen={}",
                g.severity.as_str(),
                g.object_ref,
                g.object_label,
                g.blocker.as_str(),
                g.freshness.as_str(),
                g.latest_update_at,
                g.reopen_anchor.anchor_class.as_str(),
            ));
            for ev in &g.evidence {
                lines.push(format!(
                    "      evidence {} | storage={} (live={}) | fresh={} | {}",
                    ev.evidence_ref,
                    ev.storage_class.as_str(),
                    ev.is_live,
                    ev.freshness.as_str(),
                    ev.label,
                ));
            }
            for e in &g.events {
                lines.push(format!(
                    "      event {} [{}] {}",
                    e.at,
                    e.severity.as_str(),
                    e.summary
                ));
            }
        }
        lines.push("  unresolved:".to_owned());
        for q in &packet.unresolved_questions {
            lines.push(format!(
                "      [{}] {} (owner {}, {}) -> next: {}",
                q.status.as_str(),
                q.question,
                q.owner,
                q.linked_object_ref,
                q.next_safe_action,
            ));
        }
        lines.push(format!("  roll-up: {}", packet.roll_up.headline));
        lines.push(format!(
            "  export gate: scope {} ({}), boundary-ack {} — {}",
            scope_token(packet.export_gate.scope),
            packet.export_gate.share_posture.as_str(),
            packet.export_gate.requires_boundary_ack,
            packet.export_gate.crosses_on_share,
        ));
        lines.push(format!(
            "  export: {} objects, {} (snapshot)",
            packet.export.object_count, packet.export.export_id
        ));
    }
    lines.push(String::new());
    lines.push("invariants:".to_owned());
    for inv in &set.invariants {
        lines.push(format!(
            "  [{}] {} — {}",
            if inv.holds { "OK" } else { "FAIL" },
            inv.invariant_id,
            inv.statement
        ));
    }
    lines
}
