//! M5 attention-routing qualification: the frozen certification that binds every
//! claimed attention family to the shell, companion, and operator profiles that
//! advertise it, and narrows a profile's claim automatically the moment its
//! attention evidence goes stale or failing.
//!
//! The sibling lanes in this crate each freeze one governed attention family —
//! the [notification envelope](crate::m5_envelope_routing), the
//! [durable activity object](crate::m5_activity_objects), the
//! [action/retention engine](crate::m5_attention_actions), the
//! [quiet-hours / suppression policy](crate::m5_quiet_hours_suppression), the
//! [badge aggregate](crate::m5_badge_aggregates), and the
//! [fanout receipt](crate::m5_fanout_receipts) — each backed by a checked-in
//! fixture and a freeze gate, all binding back to the
//! [attention-routing matrix](crate::m5_attention_routing). What was still
//! implicit was the *promotion* contract: which shell, companion, and operator
//! claims actually depend on which families, and what happens to a claim when one
//! family's proof goes stale. This lane is that contract.
//!
//! The bundle does three things:
//!
//! 1. **Publishes one qualification row per claimed attention family**
//!    ([`FamilyQualificationRow`]): the family, the proof packet that keeps it
//!    current (fixture, schema, and freeze gate), the
//!    [release-evidence rows](ProofCheckTag) it covers, and its current
//!    [evidence state](EvidenceState).
//! 2. **Derives each claimed profile's promotion state**
//!    ([`ProfileClaimRow`]): every claimed shell, companion, and operator profile
//!    declares the families it depends on, and its [claim state](ClaimState) is
//!    *computed* from those families' evidence by [`evaluate_profile_claim`] —
//!    never asserted. A stale dependency narrows the claim; a failing or missing
//!    one withdraws it. Release automation calls [`recompute_profiles`] with live
//!    evidence to get the narrowed claims directly.
//! 3. **Routes the result to one set of consumers**
//!    ([`QualificationConsumerBinding`]): release-evidence packets, About/Help,
//!    the activity center, support export, the compatibility report, and the
//!    public-truth surface all read [`AttentionQualificationBundle::projection`]
//!    instead of restating attention quality claims by hand.
//!
//! [`attention_qualification_bundle`] is the canonical binding: it builds the
//! bundle deterministically and computes every [`QualificationInvariant`]'s
//! `holds` flag — including the narrowing behavior itself — from the built data,
//! so the checked-in fixture and the freeze gate freeze the contract byte-for-byte
//! and an inconsistent edit flips an invariant and fails CI. The record carries no
//! message bodies, credentials, raw provider payloads, hostnames, or absolute
//! paths — only opaque object refs, stable tokens, and short reviewable sentences
//! — so it is safe to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{all_unique, is_export_safe_ref};

#[cfg(test)]
mod tests;

/// Schema version for the attention-qualification bundle.
pub const M5_ATTENTION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the attention-qualification bundle.
pub const M5_ATTENTION_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/activity/m5-attention-qualification.schema.json";

/// Stable record-kind tag for the attention-qualification bundle.
pub const M5_ATTENTION_QUALIFICATION_RECORD_KIND: &str = "m5_attention_qualification_bundle";

/// Stable id for the canonical attention-qualification bundle.
pub const M5_ATTENTION_QUALIFICATION_BUNDLE_ID: &str = "m5-attention-qualification:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ATTENTION_QUALIFICATION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the bundle binding current. Stable promotion runs
/// this gate; it fails when the in-code bundle drifts from the checked-in fixture
/// or any invariant flips.
pub const M5_ATTENTION_QUALIFICATION_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_attention_qualification.rs";

/// The attention-routing matrix fixture this bundle binds back to as its spine.
pub const M5_ATTENTION_QUALIFICATION_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

// ---------------------------------------------------------------------------
// Claimed attention families.
// ---------------------------------------------------------------------------

/// The closed set of governed attention families this bundle certifies.
///
/// Each family corresponds to one frozen lane in this crate, backed by a
/// checked-in fixture and a freeze gate. Adding a family is a breaking change;
/// the tokens are frozen here so a profile can name a dependency by token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionFamily {
    /// The notification envelope: typed, privacy-classed, deduped, reopen-safe.
    NotificationEnvelope,
    /// The durable activity object: long-running or reviewable work, never
    /// toast-only.
    ActivityObject,
    /// The action / retention engine: dismiss, snooze, acknowledge, resolve, mute.
    AttentionAction,
    /// The quiet-hours / suppression policy across in-app, OS, and companion.
    QuietHoursSuppression,
    /// The badge aggregate: deduped pending counts derived from durable items.
    BadgeAggregate,
    /// The fanout receipt: per-destination cross-client delivery truth.
    FanoutReceipt,
    /// The attention-routing matrix: the shared object-model spine every family
    /// binds back to.
    AttentionRoutingMatrix,
}

impl AttentionFamily {
    /// All families, in bundle order.
    pub const ALL: [Self; 7] = [
        Self::NotificationEnvelope,
        Self::ActivityObject,
        Self::AttentionAction,
        Self::QuietHoursSuppression,
        Self::BadgeAggregate,
        Self::FanoutReceipt,
        Self::AttentionRoutingMatrix,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationEnvelope => "notification_envelope",
            Self::ActivityObject => "activity_object",
            Self::AttentionAction => "attention_action",
            Self::QuietHoursSuppression => "quiet_hours_suppression",
            Self::BadgeAggregate => "badge_aggregate",
            Self::FanoutReceipt => "fanout_receipt",
            Self::AttentionRoutingMatrix => "attention_routing_matrix",
        }
    }

    /// Stable, namespaced family id.
    pub fn family_id(self) -> String {
        format!("attention_family.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotificationEnvelope => "Notification envelope",
            Self::ActivityObject => "Durable activity object",
            Self::AttentionAction => "Attention action / retention",
            Self::QuietHoursSuppression => "Quiet-hours / suppression",
            Self::BadgeAggregate => "Badge aggregate",
            Self::FanoutReceipt => "Fanout receipt",
            Self::AttentionRoutingMatrix => "Attention-routing matrix",
        }
    }
}

// ---------------------------------------------------------------------------
// Claimed profiles.
// ---------------------------------------------------------------------------

/// The claimed M5 attention profiles whose promotion this bundle governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedProfile {
    /// The desktop shell: in-app activity center, dock/taskbar badge, and OS
    /// native notifications.
    ShellAttention,
    /// The cross-client companions: the browser and mobile companion surfaces.
    CompanionAttention,
    /// The operator / admin dashboard: read-only managed alert and suppression
    /// visibility.
    OperatorAttention,
}

impl ClaimedProfile {
    /// All profiles, in bundle order.
    pub const ALL: [Self; 3] = [
        Self::ShellAttention,
        Self::CompanionAttention,
        Self::OperatorAttention,
    ];

    /// Stable snake_case token for this profile.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellAttention => "shell_attention",
            Self::CompanionAttention => "companion_attention",
            Self::OperatorAttention => "operator_attention",
        }
    }

    /// Stable, namespaced profile id.
    pub fn profile_id(self) -> String {
        format!("attention_profile.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShellAttention => "Shell attention",
            Self::CompanionAttention => "Companion attention",
            Self::OperatorAttention => "Operator attention",
        }
    }

    /// The families this profile's attention claim depends on. A claim cannot be
    /// promoted wider than the weakest of these families' evidence.
    pub fn dependencies(self) -> Vec<AttentionFamily> {
        use AttentionFamily::*;
        match self {
            // The shell owns the durable record and every in-product affordance, and
            // mirrors out through OS notifications, so it depends on every family.
            Self::ShellAttention => vec![
                NotificationEnvelope,
                ActivityObject,
                AttentionAction,
                QuietHoursSuppression,
                BadgeAggregate,
                FanoutReceipt,
                AttentionRoutingMatrix,
            ],
            // The companions mirror envelopes and fanout copies under quiet-hours and
            // badge truth; they neither author durable jobs nor own the action engine.
            Self::CompanionAttention => vec![
                NotificationEnvelope,
                QuietHoursSuppression,
                BadgeAggregate,
                FanoutReceipt,
                AttentionRoutingMatrix,
            ],
            // The operator view reads durable jobs, badges, suppression, and fanout
            // delivery truth across the fleet; it never authors envelopes or actions.
            Self::OperatorAttention => vec![
                ActivityObject,
                QuietHoursSuppression,
                BadgeAggregate,
                FanoutReceipt,
                AttentionRoutingMatrix,
            ],
        }
    }

    /// The independent surfaces this profile presents. The guardrail invariant
    /// requires that presenting any of these never lets the profile stay green
    /// while the underlying routing/privacy/dedupe families are stale — so each
    /// claimed surface is backed by a real family dependency, not asserted alone.
    pub fn claimed_surfaces(self) -> Vec<ClaimedSurface> {
        use ClaimedSurface::*;
        match self {
            Self::ShellAttention => {
                vec![InAppActivityCenter, OsNativeNotification, DockTaskbarBadge]
            }
            Self::CompanionAttention => vec![BrowserCompanion, MobileCompanion],
            Self::OperatorAttention => vec![OperatorDashboard, ChronologyReuse],
        }
    }
}

/// An independent attention surface a profile presents.
///
/// The guardrail is that none of these may stay green on its own: each is bound to
/// the underlying family that makes its routing, privacy, and dedupe truth real.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedSurface {
    /// The in-app activity center.
    InAppActivityCenter,
    /// OS native notifications.
    OsNativeNotification,
    /// The dock / taskbar badge.
    DockTaskbarBadge,
    /// The browser companion.
    BrowserCompanion,
    /// The mobile companion.
    MobileCompanion,
    /// The operator / admin dashboard.
    OperatorDashboard,
    /// Reuse of the chronology / activity history surface.
    ChronologyReuse,
}

impl ClaimedSurface {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InAppActivityCenter => "in_app_activity_center",
            Self::OsNativeNotification => "os_native_notification",
            Self::DockTaskbarBadge => "dock_taskbar_badge",
            Self::BrowserCompanion => "browser_companion",
            Self::MobileCompanion => "mobile_companion",
            Self::OperatorDashboard => "operator_dashboard",
            Self::ChronologyReuse => "chronology_reuse",
        }
    }
}

// ---------------------------------------------------------------------------
// Evidence and claim states.
// ---------------------------------------------------------------------------

/// The freshness of one family's proof packet, the input release automation feeds
/// the bundle to compute claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// The freeze gate is current and passing.
    Fresh,
    /// The proof exists but is older than its freshness window; the family still
    /// holds but its claim must narrow.
    Stale,
    /// The freeze gate is failing; the claim cannot stand.
    Failing,
    /// The proof packet is absent; the claim cannot stand.
    Missing,
}

impl EvidenceState {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Failing => "failing",
            Self::Missing => "missing",
        }
    }

    /// Severity rank: higher means the claim must narrow further. `fresh` is 0,
    /// `stale` is 1, and `failing` / `missing` are 2.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Fresh => 0,
            Self::Stale => 1,
            Self::Failing | Self::Missing => 2,
        }
    }

    /// Whether this state keeps a claim at full strength.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::Fresh)
    }

    /// Whether this state blocks the claim entirely (the proof is broken or gone)
    /// rather than merely narrowing it.
    pub const fn blocks_claim(self) -> bool {
        matches!(self, Self::Failing | Self::Missing)
    }
}

/// The promotion state of a claimed profile, derived from its dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    /// Every dependency is fresh; the profile may advertise its full claim.
    Full,
    /// At least one dependency is stale; the profile narrows to a degraded claim
    /// while the underlying objects still exist.
    Narrowed,
    /// At least one dependency is failing or missing; the claim is withdrawn until
    /// the proof is restored.
    Withdrawn,
}

impl ClaimState {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Narrowed => "narrowed",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Full => "Full",
            Self::Narrowed => "Narrowed",
            Self::Withdrawn => "Withdrawn",
        }
    }

    /// The claim state implied by the worst dependency severity seen: `0` is full,
    /// `1` narrows, and `2` withdraws.
    pub const fn from_worst_severity(severity: u8) -> Self {
        match severity {
            0 => Self::Full,
            1 => Self::Narrowed,
            _ => Self::Withdrawn,
        }
    }

    /// Whether the profile is published at full strength.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::Full)
    }
}

// ---------------------------------------------------------------------------
// Release-evidence rows and consumers.
// ---------------------------------------------------------------------------

/// The explicit release-evidence rows a claimed attention family must cover, so a
/// release packet names notification-envelope, activity-object, quiet-hours,
/// badge/dedupe, and fanout privacy/reopen evidence rather than a vague summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCheckTag {
    /// Typed, privacy-classed, deduped notification envelopes.
    NotificationEnvelopes,
    /// Durable, reopen-safe activity objects (never toast-only).
    DurableActivityObjects,
    /// Distinct dismiss / snooze / acknowledge / resolve / mute action semantics.
    ActionSemantics,
    /// Quiet-hours / suppression behavior kept separate from audit history.
    QuietHoursSuppression,
    /// Badge counts deduped from durable items.
    BadgeDedupeFidelity,
    /// OS / companion fanout privacy and reopen parity, with no silent failure.
    FanoutPrivacyReopenParity,
    /// A surface reopens the authoritative object instead of reissuing a side
    /// effect.
    ReopenAuthoritative,
    /// The shared routing object model the families bind back to.
    RoutingObjectModel,
}

impl ProofCheckTag {
    /// All release-evidence rows, in catalog order.
    pub const ALL: [Self; 8] = [
        Self::NotificationEnvelopes,
        Self::DurableActivityObjects,
        Self::ActionSemantics,
        Self::QuietHoursSuppression,
        Self::BadgeDedupeFidelity,
        Self::FanoutPrivacyReopenParity,
        Self::ReopenAuthoritative,
        Self::RoutingObjectModel,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationEnvelopes => "notification_envelopes",
            Self::DurableActivityObjects => "durable_activity_objects",
            Self::ActionSemantics => "action_semantics",
            Self::QuietHoursSuppression => "quiet_hours_suppression",
            Self::BadgeDedupeFidelity => "badge_dedupe_fidelity",
            Self::FanoutPrivacyReopenParity => "fanout_privacy_reopen_parity",
            Self::ReopenAuthoritative => "reopen_authoritative",
            Self::RoutingObjectModel => "routing_object_model",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotificationEnvelopes => "Notification envelopes",
            Self::DurableActivityObjects => "Durable activity objects",
            Self::ActionSemantics => "Action semantics",
            Self::QuietHoursSuppression => "Quiet-hours / suppression",
            Self::BadgeDedupeFidelity => "Badge / dedupe fidelity",
            Self::FanoutPrivacyReopenParity => "Fanout privacy / reopen parity",
            Self::ReopenAuthoritative => "Reopen authoritative object",
            Self::RoutingObjectModel => "Routing object model",
        }
    }
}

/// The consumers that read this bundle's published qualification state instead of
/// restating attention quality claims by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationConsumer {
    /// Release evidence packets and release-center cards.
    ReleaseEvidence,
    /// The About / Help truth surface.
    AboutHelp,
    /// The shell activity center.
    ActivityCenter,
    /// Support export / bundle.
    SupportExport,
    /// The compatibility report.
    CompatibilityReport,
    /// The public-truth surface.
    PublicTruth,
}

impl QualificationConsumer {
    /// All consumers, in bundle order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseEvidence,
        Self::AboutHelp,
        Self::ActivityCenter,
        Self::SupportExport,
        Self::CompatibilityReport,
        Self::PublicTruth,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEvidence => "release_evidence",
            Self::AboutHelp => "about_help",
            Self::ActivityCenter => "activity_center",
            Self::SupportExport => "support_export",
            Self::CompatibilityReport => "compatibility_report",
            Self::PublicTruth => "public_truth",
        }
    }

    /// Stable, namespaced consumer id.
    pub fn consumer_id(self) -> String {
        format!("attention_qualification_consumer.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseEvidence => "Release evidence",
            Self::AboutHelp => "About / Help",
            Self::ActivityCenter => "Activity center",
            Self::SupportExport => "Support export",
            Self::CompatibilityReport => "Compatibility report",
            Self::PublicTruth => "Public truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One claimed attention family's qualification row: the proof packet that keeps
/// it current and the release-evidence rows it covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyQualificationRow {
    /// The attention family.
    pub family: AttentionFamily,
    /// Stable, namespaced family id.
    pub family_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the family's certified guarantee.
    pub summary: String,
    /// The record kind of the family's published proof packet.
    pub record_kind: String,
    /// The stable id of the family's published proof packet.
    pub source_object_id: String,
    /// The boundary schema the proof packet validates against.
    pub schema_ref: String,
    /// The checked-in fixture that is the published proof packet.
    pub fixture_ref: String,
    /// The freeze gate that keeps the proof packet current.
    pub freeze_gate_ref: String,
    /// The evaluation stamp of the proof packet.
    pub as_of: String,
    /// The current freshness of the proof packet. The canonical bundle freezes
    /// this `fresh`; release automation overrides it from live evidence.
    pub evidence_state: EvidenceState,
    /// The release-evidence rows this family's proof covers.
    pub proof_checks: Vec<ProofCheckTag>,
    /// Whether this family preserves the rule that a security advisory is never
    /// silenced — always true; certification never narrows that escape.
    pub preserves_security_escalation: bool,
    /// One reviewable sentence stating how a stale or failing proof narrows the
    /// claims that depend on this family.
    pub narrowing_note: String,
}

impl FamilyQualificationRow {
    /// Whether this family's proof covers a given release-evidence row.
    pub fn covers(&self, tag: ProofCheckTag) -> bool {
        self.proof_checks.contains(&tag)
    }
}

/// A reference to a family and the evidence state that narrowed a profile's claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyEvidenceRef {
    /// The family whose evidence is not fresh.
    pub family: AttentionFamily,
    /// Stable family token, surfaced for consumers.
    pub family_token: String,
    /// The non-fresh evidence state that narrowed the claim.
    pub evidence_state: EvidenceState,
}

/// The computed outcome of evaluating one profile's claim against its
/// dependencies' evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileClaimOutcome {
    /// The derived claim state.
    pub claim_state: ClaimState,
    /// The dependencies, in family order, whose evidence was not fresh, with the
    /// state that narrowed the claim. Empty when the claim is full.
    pub narrowed_by: Vec<FamilyEvidenceRef>,
}

/// One claimed profile's promotion row, with a claim state derived from its
/// dependencies' evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileClaimRow {
    /// The claimed profile.
    pub profile: ClaimedProfile,
    /// Stable, namespaced profile id.
    pub profile_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the profile's full claim.
    pub summary: String,
    /// The families this profile's claim depends on.
    pub depends_on: Vec<AttentionFamily>,
    /// The independent surfaces this profile presents.
    pub claimed_surfaces: Vec<ClaimedSurface>,
    /// The claim state derived from the dependencies' evidence.
    pub claim_state: ClaimState,
    /// The dependencies that narrowed the claim, empty when full.
    pub narrowed_by: Vec<FamilyEvidenceRef>,
    /// One reviewable sentence stating the currently published claim.
    pub published_claim: String,
}

impl ProfileClaimRow {
    /// Whether this profile depends on a given family.
    pub fn depends_on_family(&self, family: AttentionFamily) -> bool {
        self.depends_on.contains(&family)
    }
}

/// One consumer binding: a surface that reads this bundle's projection instead of
/// minting per-surface attention quality vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationConsumerBinding {
    /// The consumer.
    pub consumer: QualificationConsumer,
    /// Stable, namespaced consumer id.
    pub consumer_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the consumer reuses [`AttentionQualificationBundle::projection`] —
    /// always true.
    pub reuses_projection: bool,
    /// One reviewable sentence of consumer-specific notes.
    pub note: String,
}

/// One release-evidence row definition in the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofCheckDef {
    /// Stable token.
    pub tag: ProofCheckTag,
    /// Stable token string, surfaced for consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen attention-qualification bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionQualificationBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_attention_qualification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix fixture this bundle binds back to.
    pub matrix_ref: String,
    /// The attention-routing matrix id this bundle binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps the bundle binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the bundle.
    pub summary: String,
    /// The catalog of release-evidence rows every family is measured against.
    pub proof_check_catalog: Vec<ProofCheckDef>,
    /// The per-family qualification rows.
    pub families: Vec<FamilyQualificationRow>,
    /// The per-profile claim rows, with derived claim states.
    pub profiles: Vec<ProfileClaimRow>,
    /// The consumer bindings that reuse the projection.
    pub consumers: Vec<QualificationConsumerBinding>,
    /// The computed invariants.
    pub invariants: Vec<QualificationInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for QualificationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attention-qualification bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for QualificationValidationError {}

impl AttentionQualificationBundle {
    /// Returns the qualification row for a family, if present.
    pub fn family(&self, family: AttentionFamily) -> Option<&FamilyQualificationRow> {
        self.families.iter().find(|f| f.family == family)
    }

    /// Returns the claim row for a profile, if present.
    pub fn profile(&self, profile: ClaimedProfile) -> Option<&ProfileClaimRow> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// The release-evidence rows covered by at least one family.
    pub fn covered_proof_checks(&self) -> Vec<ProofCheckTag> {
        let mut out: Vec<ProofCheckTag> = ProofCheckTag::ALL
            .into_iter()
            .filter(|tag| self.families.iter().any(|f| f.covers(*tag)))
            .collect();
        out.sort();
        out
    }

    /// A support-export-safe projection of the published qualification state for
    /// About/Help, the activity center, support export, the compatibility report,
    /// and release/public-truth surfaces. It restates no attention quality claim;
    /// it lists each profile's derived claim state and each family's evidence.
    pub fn projection(&self) -> QualificationProjection {
        QualificationProjection {
            bundle_id: self.bundle_id.clone(),
            as_of: self.as_of.clone(),
            matrix_id: self.matrix_id.clone(),
            profiles: self
                .profiles
                .iter()
                .map(|p| ProfileProjectionRow {
                    profile: p.profile,
                    profile_token: p.profile.as_str().to_owned(),
                    label: p.label.clone(),
                    claim_state: p.claim_state,
                    published_claim: p.published_claim.clone(),
                    narrowed_by: p.narrowed_by.clone(),
                })
                .collect(),
            families: self
                .families
                .iter()
                .map(|f| FamilyProjectionRow {
                    family: f.family,
                    family_token: f.family.as_str().to_owned(),
                    label: f.label.clone(),
                    evidence_state: f.evidence_state,
                    fixture_ref: f.fixture_ref.clone(),
                    freeze_gate_ref: f.freeze_gate_ref.clone(),
                })
                .collect(),
        }
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the bundle, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_self = [self.matrix_ref.as_str(), self.freeze_gate_ref.as_str()].into_iter();
        let from_families = self.families.iter().flat_map(|f| {
            [
                f.schema_ref.as_str(),
                f.fixture_ref.as_str(),
                f.freeze_gate_ref.as_str(),
            ]
            .into_iter()
        });
        from_self.chain(from_families)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`QualificationInvariant`]s with the uniqueness and
    /// completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), QualificationValidationError> {
        let fail = |reason: String| Err(QualificationValidationError { reason });

        if self.record_kind != M5_ATTENTION_QUALIFICATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ATTENTION_QUALIFICATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every family and profile is present exactly once.
        for family in AttentionFamily::ALL {
            if self.families.iter().filter(|f| f.family == family).count() != 1 {
                return fail(format!(
                    "family {} not present exactly once",
                    family.as_str()
                ));
            }
        }
        for profile in ClaimedProfile::ALL {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }

        // Stable ids are unique.
        if !all_unique(self.families.iter().map(|f| f.family_id.as_str())) {
            return fail("family ids are not unique".to_owned());
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        // Per-family floor: every family cites a complete proof packet and at least
        // one release-evidence row.
        for row in &self.families {
            if row.family_id != row.family.family_id() {
                return fail(format!("family id mismatch for {}", row.family.as_str()));
            }
            if row.schema_ref.is_empty()
                || row.fixture_ref.is_empty()
                || row.freeze_gate_ref.is_empty()
            {
                return fail(format!(
                    "family {} has an incomplete proof packet",
                    row.family.as_str()
                ));
            }
            if row.proof_checks.is_empty() {
                return fail(format!(
                    "family {} covers no release-evidence row",
                    row.family.as_str()
                ));
            }
            if !row.preserves_security_escalation {
                return fail(format!(
                    "family {} must preserve security escalation",
                    row.family.as_str()
                ));
            }
        }

        // Per-profile floor: the published claim row equals a fresh re-evaluation,
        // so a claim is always derived from evidence rather than asserted.
        for row in &self.profiles {
            if row.profile_id != row.profile.profile_id() {
                return fail(format!("profile id mismatch for {}", row.profile.as_str()));
            }
            if row.depends_on.is_empty() {
                return fail(format!(
                    "profile {} declares no dependencies",
                    row.profile.as_str()
                ));
            }
            let recomputed = evaluate_profile_claim(&row.depends_on, &self.families);
            if recomputed.claim_state != row.claim_state
                || recomputed.narrowed_by != row.narrowed_by
            {
                return fail(format!(
                    "profile {} claim is not derived from its dependencies",
                    row.profile.as_str()
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
// Projection.
// ---------------------------------------------------------------------------

/// One profile row in the published projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileProjectionRow {
    /// The claimed profile.
    pub profile: ClaimedProfile,
    /// Stable profile token.
    pub profile_token: String,
    /// Human-readable label.
    pub label: String,
    /// The derived claim state.
    pub claim_state: ClaimState,
    /// The currently published claim sentence.
    pub published_claim: String,
    /// The dependencies that narrowed the claim, empty when full.
    pub narrowed_by: Vec<FamilyEvidenceRef>,
}

/// One family row in the published projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyProjectionRow {
    /// The attention family.
    pub family: AttentionFamily,
    /// Stable family token.
    pub family_token: String,
    /// Human-readable label.
    pub label: String,
    /// The current evidence state.
    pub evidence_state: EvidenceState,
    /// The checked-in fixture that is the proof packet.
    pub fixture_ref: String,
    /// The freeze gate that keeps the proof packet current.
    pub freeze_gate_ref: String,
}

/// The support-export-safe projection consumed by About/Help, the activity
/// center, support export, the compatibility report, and release/public-truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationProjection {
    /// The bundle id this projection derives from.
    pub bundle_id: String,
    /// The evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix id the bundle binds back to.
    pub matrix_id: String,
    /// The per-profile published claim rows.
    pub profiles: Vec<ProfileProjectionRow>,
    /// The per-family evidence rows.
    pub families: Vec<FamilyProjectionRow>,
}

// ---------------------------------------------------------------------------
// Claim evaluation.
// ---------------------------------------------------------------------------

/// Evaluates one profile's claim from its dependencies' evidence.
///
/// The claim state is the worst of the dependencies' evidence severities: every
/// dependency fresh yields [`ClaimState::Full`], any stale dependency yields
/// [`ClaimState::Narrowed`], and any failing or missing dependency yields
/// [`ClaimState::Withdrawn`]. `narrowed_by` lists every dependency whose evidence
/// is not fresh, in family order, so the narrowing names its cause. This is the
/// hook release automation calls with live evidence to derive the narrowed claim.
pub fn evaluate_profile_claim(
    dependencies: &[AttentionFamily],
    families: &[FamilyQualificationRow],
) -> ProfileClaimOutcome {
    let mut worst = 0u8;
    let mut narrowed_by = Vec::new();
    for family in AttentionFamily::ALL {
        if !dependencies.contains(&family) {
            continue;
        }
        let state = families
            .iter()
            .find(|f| f.family == family)
            .map(|f| f.evidence_state)
            // A dependency with no qualification row at all is missing evidence.
            .unwrap_or(EvidenceState::Missing);
        if !state.is_fresh() {
            worst = worst.max(state.severity());
            narrowed_by.push(FamilyEvidenceRef {
                family,
                family_token: family.as_str().to_owned(),
                evidence_state: state,
            });
        }
    }
    ProfileClaimOutcome {
        claim_state: ClaimState::from_worst_severity(worst),
        narrowed_by,
    }
}

/// Recomputes every claimed profile's row against a set of live family evidence
/// states, returning the narrowed claim rows release automation publishes.
///
/// `evidence` overrides the canonical `fresh` state for the named families; any
/// family not named keeps the state it carries in `families`. This is the
/// release-automation entry point: feed it the live freeze-gate results and it
/// returns each profile's derived claim — narrowed or withdrawn — without
/// restating any claim by hand.
pub fn recompute_profiles(
    families: &[FamilyQualificationRow],
    evidence: &[(AttentionFamily, EvidenceState)],
) -> Vec<ProfileClaimRow> {
    let resolved: Vec<FamilyQualificationRow> = families
        .iter()
        .map(|f| {
            let mut row = f.clone();
            if let Some((_, state)) = evidence.iter().find(|(fam, _)| *fam == f.family) {
                row.evidence_state = *state;
            }
            row
        })
        .collect();
    ClaimedProfile::ALL
        .into_iter()
        .map(|profile| build_profile_row(profile, &resolved))
        .collect()
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical attention-qualification bundle.
///
/// Deterministic: the same bytes every call. The canonical bundle freezes every
/// family `fresh`, so every profile's derived claim is [`ClaimState::Full`]. The
/// invariant `holds` flags — including the narrowing and withdrawal behavior — are
/// computed from the built data, so an inconsistent edit flips an invariant rather
/// than silently passing.
pub fn attention_qualification_bundle() -> AttentionQualificationBundle {
    let families = build_families();
    let profiles: Vec<ProfileClaimRow> = ClaimedProfile::ALL
        .into_iter()
        .map(|profile| build_profile_row(profile, &families))
        .collect();
    let consumers = build_consumers();
    let proof_check_catalog = ProofCheckTag::ALL
        .into_iter()
        .map(|tag| ProofCheckDef {
            tag,
            token: tag.as_str().to_owned(),
            label: tag.label().to_owned(),
        })
        .collect();
    let invariants = compute_invariants(&families, &profiles, &consumers);

    AttentionQualificationBundle {
        record_kind: M5_ATTENTION_QUALIFICATION_RECORD_KIND.to_owned(),
        m5_attention_qualification_schema_version: M5_ATTENTION_QUALIFICATION_SCHEMA_VERSION,
        schema_ref: M5_ATTENTION_QUALIFICATION_SCHEMA_REF.to_owned(),
        bundle_id: M5_ATTENTION_QUALIFICATION_BUNDLE_ID.to_owned(),
        as_of: M5_ATTENTION_QUALIFICATION_AS_OF.to_owned(),
        matrix_ref: M5_ATTENTION_QUALIFICATION_MATRIX_REF.to_owned(),
        matrix_id: crate::m5_attention_routing::M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ATTENTION_QUALIFICATION_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen certification binding every claimed attention family — notification \
                  envelopes, durable activity objects, action/retention semantics, quiet-hours and \
                  suppression, badge aggregates, and fanout receipts, all on the attention-routing \
                  matrix spine — to the shell, companion, and operator profiles that advertise it. \
                  Each family carries the proof packet that keeps it current; each profile's claim is \
                  derived from its dependencies' evidence, so a stale proof narrows the claim and a \
                  failing or missing proof withdraws it, automatically and per profile. About/Help, \
                  the activity center, support export, the compatibility report, and release and \
                  public-truth surfaces read one published projection instead of restating attention \
                  quality claims, and certification never silences a security advisory."
            .to_owned(),
        proof_check_catalog,
        families,
        profiles,
        consumers,
        invariants,
        raw_payload_excluded: true,
    }
}

fn checks(tags: &[ProofCheckTag]) -> Vec<ProofCheckTag> {
    tags.to_vec()
}

fn build_families() -> Vec<FamilyQualificationRow> {
    use ProofCheckTag::*;

    vec![
        FamilyQualificationRow {
            family: AttentionFamily::NotificationEnvelope,
            family_id: AttentionFamily::NotificationEnvelope.family_id(),
            label: AttentionFamily::NotificationEnvelope.label().to_owned(),
            summary: "Every attention is a typed, privacy-classed, deduped envelope with a stable \
                      action target and a route back to its authoritative object, never an ad hoc \
                      toast."
                .to_owned(),
            record_kind: crate::m5_envelope_routing::M5_ENVELOPE_ROUTING_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_envelope_routing::M5_ENVELOPE_ROUTING_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_envelope_routing::M5_ENVELOPE_ROUTING_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-envelope-routing/canonical_bundle.json".to_owned(),
            freeze_gate_ref: crate::m5_envelope_routing::M5_ENVELOPE_ROUTING_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_envelope_routing::M5_ENVELOPE_ROUTING_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[NotificationEnvelopes, ReopenAuthoritative, RoutingObjectModel]),
            preserves_security_escalation: true,
            narrowing_note: "If envelope-routing proof goes stale, the shell and companion claims \
                             narrow; if it fails or is missing, they are withdrawn until the gate is \
                             restored."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::ActivityObject,
            family_id: AttentionFamily::ActivityObject.family_id(),
            label: AttentionFamily::ActivityObject.label().to_owned(),
            summary: "Long-running or reviewable work is a durable activity object with phase, \
                      progress, cancel/retry affordances, and a reopen anchor — retained until \
                      archived, never toast-only."
                .to_owned(),
            record_kind: crate::m5_activity_objects::M5_ACTIVITY_OBJECTS_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_activity_objects::M5_ACTIVITY_OBJECTS_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_activity_objects::M5_ACTIVITY_OBJECTS_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-activity-objects/canonical_bundle.json".to_owned(),
            freeze_gate_ref: crate::m5_activity_objects::M5_ACTIVITY_OBJECTS_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_activity_objects::M5_ACTIVITY_OBJECTS_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[
                DurableActivityObjects,
                ReopenAuthoritative,
                RoutingObjectModel,
            ]),
            preserves_security_escalation: true,
            narrowing_note: "If durable-activity proof goes stale, the shell and operator claims \
                             narrow; if it fails or is missing, they are withdrawn."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::AttentionAction,
            family_id: AttentionFamily::AttentionAction.family_id(),
            label: AttentionFamily::AttentionAction.label().to_owned(),
            summary: "Dismiss, snooze, acknowledge, resolve, and mute are distinct actions with \
                      distinct retention and resume meaning that keep the underlying durable record \
                      and reopen the same authoritative target."
                .to_owned(),
            record_kind: crate::m5_attention_actions::M5_ATTENTION_ACTIONS_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_attention_actions::M5_ATTENTION_ACTIONS_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_attention_actions::M5_ATTENTION_ACTIONS_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-attention-actions/canonical_bundle.json".to_owned(),
            freeze_gate_ref: crate::m5_attention_actions::M5_ATTENTION_ACTIONS_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_attention_actions::M5_ATTENTION_ACTIONS_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[ActionSemantics, ReopenAuthoritative, RoutingObjectModel]),
            preserves_security_escalation: true,
            narrowing_note: "If action-semantics proof goes stale, the shell claim narrows; if it \
                             fails or is missing, it is withdrawn."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::QuietHoursSuppression,
            family_id: AttentionFamily::QuietHoursSuppression.family_id(),
            label: AttentionFamily::QuietHoursSuppression.label().to_owned(),
            summary: "One coherent quiet-hours, do-not-disturb, presentation, lock-screen, and \
                      managed-endpoint suppression policy explains every shown, downgraded, or \
                      withheld outcome, keeps the durable record, records suppression separate from \
                      audit history, and never silences a security advisory."
                .to_owned(),
            record_kind: crate::m5_quiet_hours_suppression::M5_QUIET_HOURS_SUPPRESSION_RECORD_KIND
                .to_owned(),
            source_object_id:
                crate::m5_quiet_hours_suppression::M5_QUIET_HOURS_SUPPRESSION_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_quiet_hours_suppression::M5_QUIET_HOURS_SUPPRESSION_SCHEMA_REF
                .to_owned(),
            fixture_ref: "fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json"
                .to_owned(),
            freeze_gate_ref:
                crate::m5_quiet_hours_suppression::M5_QUIET_HOURS_SUPPRESSION_FREEZE_GATE_REF
                    .to_owned(),
            as_of: crate::m5_quiet_hours_suppression::M5_QUIET_HOURS_SUPPRESSION_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[QuietHoursSuppression, ReopenAuthoritative, RoutingObjectModel]),
            preserves_security_escalation: true,
            narrowing_note: "If quiet-hours / suppression proof goes stale, every claimed profile \
                             narrows, because privacy routing is shared across them all; if it fails \
                             or is missing, every claim is withdrawn."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::BadgeAggregate,
            family_id: AttentionFamily::BadgeAggregate.family_id(),
            label: AttentionFamily::BadgeAggregate.label().to_owned(),
            summary: "Badge counts derive from deduped durable items keyed by scope, coalesce \
                      repeated failures from one root cause into one object counted once, and project \
                      one shared count across every badge-bearing surface."
                .to_owned(),
            record_kind: crate::m5_badge_aggregates::M5_BADGE_AGGREGATES_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_badge_aggregates::M5_BADGE_AGGREGATES_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_badge_aggregates::M5_BADGE_AGGREGATES_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-badge-aggregates/canonical_bundle.json".to_owned(),
            freeze_gate_ref: crate::m5_badge_aggregates::M5_BADGE_AGGREGATES_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_badge_aggregates::M5_BADGE_AGGREGATES_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[BadgeDedupeFidelity, ReopenAuthoritative, RoutingObjectModel]),
            preserves_security_escalation: true,
            narrowing_note: "If badge / dedupe proof goes stale, every claimed profile narrows, \
                             because each renders the shared count; if it fails or is missing, every \
                             claim is withdrawn."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::FanoutReceipt,
            family_id: AttentionFamily::FanoutReceipt.family_id(),
            label: AttentionFamily::FanoutReceipt.label().to_owned(),
            summary: "Cross-client delivery is a durable per-destination receipt that names the \
                      source, destination, delivery state, and explicit stale/undelivered reason; a \
                      failed or stale copy is labeled rather than counted as delivered, and every \
                      copy reopens the source's authoritative object without acting inline."
                .to_owned(),
            record_kind: crate::m5_fanout_receipts::M5_FANOUT_RECEIPTS_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_fanout_receipts::M5_FANOUT_RECEIPTS_BUNDLE_ID.to_owned(),
            schema_ref: crate::m5_fanout_receipts::M5_FANOUT_RECEIPTS_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-fanout-receipts/canonical_bundle.json".to_owned(),
            freeze_gate_ref: crate::m5_fanout_receipts::M5_FANOUT_RECEIPTS_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_fanout_receipts::M5_FANOUT_RECEIPTS_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[
                FanoutPrivacyReopenParity,
                ReopenAuthoritative,
                RoutingObjectModel,
            ]),
            preserves_security_escalation: true,
            narrowing_note: "If fanout proof goes stale, every claimed profile narrows, because each \
                             depends on cross-client delivery truth; if it fails or is missing, every \
                             claim is withdrawn."
                .to_owned(),
        },
        FamilyQualificationRow {
            family: AttentionFamily::AttentionRoutingMatrix,
            family_id: AttentionFamily::AttentionRoutingMatrix.family_id(),
            label: AttentionFamily::AttentionRoutingMatrix.label().to_owned(),
            summary: "The shared object-model spine names every attention family, freezes its stable \
                      identifiers and vocabulary, and states the invariants every surface holds, so \
                      every other family binds back to one routing truth."
                .to_owned(),
            record_kind: crate::m5_attention_routing::M5_ATTENTION_ROUTING_RECORD_KIND.to_owned(),
            source_object_id: crate::m5_attention_routing::M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
            schema_ref: crate::m5_attention_routing::M5_ATTENTION_ROUTING_SCHEMA_REF.to_owned(),
            fixture_ref: "fixtures/activity/m5-attention-routing/canonical_matrix.json".to_owned(),
            freeze_gate_ref: crate::m5_attention_routing::M5_ATTENTION_ROUTING_FREEZE_GATE_REF
                .to_owned(),
            as_of: crate::m5_attention_routing::M5_ATTENTION_ROUTING_AS_OF.to_owned(),
            evidence_state: EvidenceState::Fresh,
            proof_checks: checks(&[RoutingObjectModel, ReopenAuthoritative]),
            preserves_security_escalation: true,
            narrowing_note: "The routing matrix is the spine every profile depends on, so if its \
                             proof goes stale every claim narrows, and if it fails or is missing every \
                             claim is withdrawn."
                .to_owned(),
        },
    ]
}

fn profile_summary(profile: ClaimedProfile) -> &'static str {
    match profile {
        ClaimedProfile::ShellAttention => {
            "The desktop shell routes typed, durable, reopen-safe attention through the activity \
             center, dock/taskbar badge, and OS native notifications, with deduped badges, distinct \
             actions, and quiet-hours-aware privacy."
        }
        ClaimedProfile::CompanionAttention => {
            "The browser and mobile companions mirror attention out of window as privacy-safe, \
             reopen-safe copies, with labeled fanout delivery truth, shared badge counts, and \
             quiet-hours-aware privacy that never widens on fanout."
        }
        ClaimedProfile::OperatorAttention => {
            "The operator dashboard presents read-only managed alert, suppression, badge, and \
             fanout-delivery truth across the fleet, reopening the authoritative object rather than \
             reissuing a side effect."
        }
    }
}

fn build_profile_row(
    profile: ClaimedProfile,
    families: &[FamilyQualificationRow],
) -> ProfileClaimRow {
    let depends_on = profile.dependencies();
    let outcome = evaluate_profile_claim(&depends_on, families);
    let published_claim = match outcome.claim_state {
        ClaimState::Full => format!("{} advertised at full strength.", profile.label()),
        ClaimState::Narrowed => format!(
            "{} narrowed to a degraded claim: {}.",
            profile.label(),
            narrowing_causes(&outcome.narrowed_by)
        ),
        ClaimState::Withdrawn => format!(
            "{} withdrawn until proof is restored: {}.",
            profile.label(),
            narrowing_causes(&outcome.narrowed_by)
        ),
    };
    ProfileClaimRow {
        profile,
        profile_id: profile.profile_id(),
        label: profile.label().to_owned(),
        summary: profile_summary(profile).to_owned(),
        depends_on,
        claimed_surfaces: profile.claimed_surfaces(),
        claim_state: outcome.claim_state,
        narrowed_by: outcome.narrowed_by,
        published_claim,
    }
}

fn narrowing_causes(narrowed_by: &[FamilyEvidenceRef]) -> String {
    narrowed_by
        .iter()
        .map(|r| format!("{} is {}", r.family.as_str(), r.evidence_state.as_str()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn build_consumers() -> Vec<QualificationConsumerBinding> {
    let binding = |consumer: QualificationConsumer, note: &str| QualificationConsumerBinding {
        consumer,
        consumer_id: consumer.consumer_id(),
        label: consumer.label().to_owned(),
        reuses_projection: true,
        note: note.to_owned(),
    };
    vec![
        binding(
            QualificationConsumer::ReleaseEvidence,
            "Release evidence packets include one row per family — envelopes, activity objects, \
             quiet-hours/suppression, badge/dedupe, and fanout privacy/reopen parity — and the \
             derived per-profile claim, narrowing the affected claim when proof is stale or failing.",
        ),
        binding(
            QualificationConsumer::AboutHelp,
            "About / Help reads the published projection so its attention quality statements always \
             match the current claim state rather than restating it.",
        ),
        binding(
            QualificationConsumer::ActivityCenter,
            "The activity center surfaces the same qualification state next to the attention objects \
             it governs.",
        ),
        binding(
            QualificationConsumer::SupportExport,
            "Support export embeds the projection verbatim; it carries opaque object refs and stable \
             tokens only, never message bodies or payloads.",
        ),
        binding(
            QualificationConsumer::CompatibilityReport,
            "The compatibility report inherits each profile's claim state, so a narrowed attention \
             claim narrows the marketable wording it can publish.",
        ),
        binding(
            QualificationConsumer::PublicTruth,
            "Release and public-truth surfaces publish the derived claim, never wider than the \
             current evidence allows.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> QualificationInvariant {
    QualificationInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    families: &[FamilyQualificationRow],
    profiles: &[ProfileClaimRow],
    consumers: &[QualificationConsumerBinding],
) -> Vec<QualificationInvariant> {
    let mut out = Vec::new();

    // Every family present and proven.
    out.push(invariant(
        "attention_qualification.every_family_proven",
        "Every claimed attention family is present exactly once and cites a complete proof packet — \
         a schema, a checked-in fixture, and a freeze gate.",
        AttentionFamily::ALL.iter().all(|family| {
            families
                .iter()
                .filter(|f| f.family == *family)
                .count()
                == 1
        }) && families.iter().all(|f| {
            !f.schema_ref.is_empty() && !f.fixture_ref.is_empty() && !f.freeze_gate_ref.is_empty()
        }),
    ));

    // Every release-evidence row is covered by some family.
    out.push(invariant(
        "attention_qualification.release_rows_covered",
        "Each release-evidence row — notification envelopes, durable activity objects, action \
         semantics, quiet-hours/suppression, badge/dedupe fidelity, fanout privacy/reopen parity, \
         reopen-authoritative, and routing object model — is covered by at least one family.",
        ProofCheckTag::ALL
            .iter()
            .all(|tag| families.iter().any(|f| f.covers(*tag))),
    ));

    // Every profile present and claim derived, not asserted.
    out.push(invariant(
        "attention_qualification.profile_claim_derived",
        "Every claimed shell, companion, and operator profile is present exactly once and its \
         published claim equals a fresh re-evaluation of its dependencies' evidence, so a claim is \
         derived rather than asserted.",
        ClaimedProfile::ALL
            .iter()
            .all(|profile| profiles.iter().filter(|p| p.profile == *profile).count() == 1)
            && profiles.iter().all(|p| {
                let recomputed = evaluate_profile_claim(&p.depends_on, families);
                recomputed.claim_state == p.claim_state && recomputed.narrowed_by == p.narrowed_by
            }),
    ));

    // The canonical, all-fresh bundle promotes every profile at full strength.
    out.push(invariant(
        "attention_qualification.fresh_promotes_full",
        "When every family's evidence is fresh, every claimed profile is promoted at full strength \
         with no narrowing cause.",
        families.iter().all(|f| f.evidence_state.is_fresh())
            && profiles
                .iter()
                .all(|p| p.claim_state.is_full() && p.narrowed_by.is_empty()),
    ));

    // A stale dependency narrows exactly the dependent profiles — the core
    // automatic-narrowing behavior, proven by exercising the evaluator over every
    // (profile, family) pair.
    out.push(invariant(
        "attention_qualification.stale_dependency_narrows",
        "Marking any one family stale narrows every profile that depends on it to a narrowed claim \
         that names the stale family, and leaves every profile that does not depend on it at full \
         strength.",
        narrowing_behaves(families, profiles, EvidenceState::Stale, ClaimState::Narrowed),
    ));

    // A failing or missing dependency withdraws exactly the dependent profiles.
    out.push(invariant(
        "attention_qualification.failing_dependency_withdraws",
        "Marking any one family failing withdraws every profile that depends on it and leaves every \
         profile that does not depend on it at full strength, so a broken proof cannot keep a claim \
         green.",
        narrowing_behaves(families, profiles, EvidenceState::Failing, ClaimState::Withdrawn),
    ));

    // The routing matrix and the quiet-hours / suppression families are shared
    // spines every profile depends on, so attention-routing and privacy staleness
    // narrows all claims (the acceptance-criteria trio: routing, privacy, fanout).
    out.push(invariant(
        "attention_qualification.shared_spines_depended_everywhere",
        "Every claimed profile depends on the attention-routing matrix, the quiet-hours/suppression \
         policy, and the fanout receipt, so routing, privacy, or fanout proof going stale narrows \
         every claim.",
        profiles.iter().all(|p| {
            p.depends_on_family(AttentionFamily::AttentionRoutingMatrix)
                && p.depends_on_family(AttentionFamily::QuietHoursSuppression)
                && p.depends_on_family(AttentionFamily::FanoutReceipt)
        }),
    ));

    // Guardrail: no profile stays green on a standalone surface while the
    // underlying routing/privacy/dedupe families are stale. Every profile that
    // presents an independent surface depends on the routing matrix, quiet-hours,
    // and badge families.
    out.push(invariant(
        "attention_qualification.no_standalone_green_surface",
        "No profile may stay green because an independent surface exists: every profile that \
         presents a surface depends on the routing matrix, the quiet-hours/suppression policy, and \
         the badge aggregate that make that surface's routing, privacy, and dedupe truth real.",
        profiles.iter().all(|p| {
            p.claimed_surfaces.is_empty()
                || (p.depends_on_family(AttentionFamily::AttentionRoutingMatrix)
                    && p.depends_on_family(AttentionFamily::QuietHoursSuppression)
                    && p.depends_on_family(AttentionFamily::BadgeAggregate))
        }),
    ));

    // Certification never silences a security advisory.
    out.push(invariant(
        "attention_qualification.security_never_silenced",
        "Every family preserves the rule that a security advisory is never silenced; certification \
         narrows a marketable claim but never narrows that escape.",
        families.iter().all(|f| f.preserves_security_escalation),
    ));

    // Every consumer reuses the one projection.
    out.push(invariant(
        "attention_qualification.consumers_reuse_projection",
        "About/Help, the activity center, support export, the compatibility report, and release and \
         public-truth surfaces all reuse the one published projection instead of restating attention \
         quality claims.",
        QualificationConsumer::ALL
            .iter()
            .all(|c| consumers.iter().any(|b| b.consumer == *c && b.reuses_projection)),
    ));

    // The bundle binds back to the attention-routing matrix spine.
    out.push(invariant(
        "attention_qualification.binds_routing_matrix",
        "The bundle binds back to the attention-routing matrix as its spine, and the matrix family \
         row cites the same matrix id.",
        families
            .iter()
            .find(|f| f.family == AttentionFamily::AttentionRoutingMatrix)
            .is_some_and(|f| {
                f.source_object_id == crate::m5_attention_routing::M5_ATTENTION_ROUTING_MATRIX_ID
            }),
    ));

    out
}

/// Exercises the evaluator: marking each family in turn to `state` must move every
/// dependent profile to `expected` (naming that family) and leave every
/// non-dependent profile at [`ClaimState::Full`].
fn narrowing_behaves(
    families: &[FamilyQualificationRow],
    profiles: &[ProfileClaimRow],
    state: EvidenceState,
    expected: ClaimState,
) -> bool {
    for family in AttentionFamily::ALL {
        let perturbed: Vec<FamilyQualificationRow> = families
            .iter()
            .map(|f| {
                let mut row = f.clone();
                if row.family == family {
                    row.evidence_state = state;
                }
                row
            })
            .collect();
        for profile_row in profiles {
            let outcome = evaluate_profile_claim(&profile_row.depends_on, &perturbed);
            if profile_row.depends_on_family(family) {
                if outcome.claim_state != expected
                    || !outcome.narrowed_by.iter().any(|r| r.family == family)
                {
                    return false;
                }
            } else if outcome.claim_state != ClaimState::Full || !outcome.narrowed_by.is_empty() {
                return false;
            }
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn attention_qualification_lines(bundle: &AttentionQualificationBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Attention-qualification bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Families: {}  Profiles: {}  Consumers: {}  Invariants: {}",
        bundle.families.len(),
        bundle.profiles.len(),
        bundle.consumers.len(),
        bundle.invariants.len(),
    ));

    lines.push("Families:".to_owned());
    for f in &bundle.families {
        let tags: Vec<&str> = f.proof_checks.iter().map(|t| t.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] evidence={}",
            f.family.as_str(),
            f.family_id,
            f.evidence_state.as_str(),
        ));
        lines.push(format!("      proof: {}", f.fixture_ref));
        lines.push(format!("      gate: {}", f.freeze_gate_ref));
        lines.push(format!("      covers: {}", tags.join(", ")));
    }

    lines.push("Profiles:".to_owned());
    for p in &bundle.profiles {
        let deps: Vec<&str> = p.depends_on.iter().map(|d| d.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] claim={}",
            p.profile.as_str(),
            p.profile_id,
            p.claim_state.as_str(),
        ));
        lines.push(format!("      {}", p.published_claim));
        lines.push(format!("      depends_on: {}", deps.join(", ")));
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
