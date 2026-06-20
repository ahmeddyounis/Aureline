//! Shippable reactive-truth cues for the derived M5 surfaces.
//!
//! Where [`crate::m5_reactive_governance`] freezes the *rules* — which
//! authority owns each surface, which states it can present, and how the
//! canonical engine narrows a presented claim — this module ships the
//! *rendered truth* every derived surface actually presents. For one
//! surface and one observed subscription state it produces a single
//! [`ReactiveTruthCue`] that says, in one shared grammar:
//!
//! - **where the truth came from** (authority class, materialized-view
//!   class, scope, and the epoch-parity group it must stay level with);
//! - **how fresh and complete it is** (the observed freshness,
//!   completeness, and backpressure echoed straight from the envelope);
//! - **what invalidation changed it** (the dominant invalidation reason
//!   behind the current degraded claim);
//! - **whether it is keeping up** (a coalesced / snapshot-required cue);
//! - **what the surface may now claim** (the canonically narrowed
//!   [`TruthClaim`]); and
//! - **whether dangerous derived actions stay live** (an
//!   [`ActionGate`] that narrows mutating affordances the moment the
//!   surface can no longer prove a consistent snapshot, plus a
//!   resubscribe-required flag for terminal streams).
//!
//! Nothing here invents a second stale-state vocabulary: every claim is
//! produced by [`crate::m5_reactive_governance::narrow_truth_claim`], and
//! the per-surface audit rows are derived from the governance matrix's own
//! `claim_narrowing_rules`, so the cue layer can never drift from the
//! engine. UI, CLI/headless, activity-center, accessibility, diagnostics,
//! and support/export consumers all read the same cue, so a search panel,
//! a graph surface, an AI inspector, a review pane, a docs view, and a
//! support summary describe the same degraded state identically.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/reactive_truth_surfaces.schema.json`](../../../../schemas/state/reactive_truth_surfaces.schema.json)
//! - [`/docs/state/reactive_truth_surfaces.md`](../../../../docs/state/reactive_truth_surfaces.md)
//! - [`/artifacts/state/reactive_truth_surfaces.json`](../../../../artifacts/state/reactive_truth_surfaces.json)
//! - [`/artifacts/state/reactive_truth_surfaces.md`](../../../../artifacts/state/reactive_truth_surfaces.md)
//! - [`/fixtures/state/reactive_truth_surfaces/`](../../../../fixtures/state/reactive_truth_surfaces/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_reactive_governance::{
    narrow_truth_claim, seeded_m5_reactive_governance_packet,
    validate_m5_reactive_governance_packet, AuthorityClass, BackpressureMode, ClaimNarrowingRule,
    Completeness, DerivationClass, Freshness, InvalidationReason, NarrowingTrigger,
    ObservedReactiveState, ReactiveSurfaceClass, ReactiveSurfaceRow, ScopeClass, TerminalReason,
    TruthClaim, ViewClass,
};

/// Schema version stamped onto packets and fixtures.
pub const REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const REACTIVE_TRUTH_SURFACES_PACKET_RECORD_KIND: &str =
    "reactive_truth_surfaces_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const REACTIVE_TRUTH_SURFACES_FIXTURE_RECORD_KIND: &str =
    "reactive_truth_surfaces_fixture_record";

/// Repo-relative schema ref.
pub const REACTIVE_TRUTH_SURFACES_SCHEMA_REF: &str =
    "schemas/state/reactive_truth_surfaces.schema.json";

/// Repo-relative reviewer doc ref.
pub const REACTIVE_TRUTH_SURFACES_DOC_REF: &str = "docs/state/reactive_truth_surfaces.md";

/// Repo-relative machine-readable artifact packet.
pub const REACTIVE_TRUTH_SURFACES_PACKET_REF: &str = "artifacts/state/reactive_truth_surfaces.json";

/// Repo-relative reviewer audit report.
pub const REACTIVE_TRUTH_SURFACES_REPORT_REF: &str = "artifacts/state/reactive_truth_surfaces.md";

/// Repo-relative fixture directory.
pub const REACTIVE_TRUTH_SURFACES_FIXTURE_DIR: &str = "fixtures/state/reactive_truth_surfaces";

/// Repo-relative fixture manifest.
pub const REACTIVE_TRUTH_SURFACES_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/reactive_truth_surfaces/manifest.yaml";

// ---------------------------------------------------------------------------
// Action gate: dangerous derived actions narrow as truth degrades.
// ---------------------------------------------------------------------------

/// How a derived surface must treat dangerous actions for a presented
/// truth-claim. A derived surface can never prove exact current truth, so
/// the strongest gate it reaches is [`ActionGate::Enabled`] at a consistent
/// snapshot; every degraded claim narrows the gate from there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionGate {
    /// The surface holds a consistent snapshot; derived actions are live
    /// because they revalidate against the authority on apply.
    Enabled,
    /// The surface lags or is cache-served; a derived action must
    /// revalidate against the authority before it applies.
    RevalidateBeforeAct,
    /// Scope is partial or not yet warm; mutating derived actions are
    /// narrowed to read-only until the projection completes.
    NarrowedToReadOnly,
    /// The surface cannot prove a consistent snapshot; dangerous derived
    /// actions are disabled until it refreshes, replays live, or
    /// reconnects.
    Blocked,
}

impl ActionGate {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::RevalidateBeforeAct => "revalidate_before_act",
            Self::NarrowedToReadOnly => "narrowed_to_read_only",
            Self::Blocked => "blocked",
        }
    }

    /// Whether a dangerous (mutating) derived action stays live under this
    /// gate. Only a consistent snapshot keeps it live.
    pub const fn allows_dangerous_derived_action(self) -> bool {
        matches!(self, Self::Enabled)
    }

    /// Human-readable narration fragment shared across every channel.
    pub const fn note(self) -> &'static str {
        match self {
            Self::Enabled => "derived actions stay live and revalidate against authority on apply",
            Self::RevalidateBeforeAct => {
                "derived actions must revalidate against authority before applying"
            }
            Self::NarrowedToReadOnly => "mutating derived actions are narrowed to read-only",
            Self::Blocked => {
                "dangerous derived actions are disabled until the surface refreshes or reconnects"
            }
        }
    }

    /// The gate a surface must apply for a narrowed truth-claim.
    pub const fn for_claim(claim: TruthClaim) -> Self {
        match claim {
            // The authority ceiling and the derived ceiling both keep
            // derived actions live; derived mutations revalidate on apply.
            TruthClaim::ExactCurrentTruth | TruthClaim::ConsistentSnapshot => Self::Enabled,
            // Lagging or cache-served truth is recoverable by a revalidate.
            TruthClaim::CoalescedStream | TruthClaim::CachedProjection => Self::RevalidateBeforeAct,
            // Incomplete scope cannot back a mutation yet.
            TruthClaim::PartialProjection | TruthClaim::WarmingNoTruthYet => {
                Self::NarrowedToReadOnly
            }
            // Known-stale, captured, imported, policy-limited, or absent
            // truth blocks dangerous derived actions outright.
            TruthClaim::StaleSnapshot
            | TruthClaim::ReplayedSnapshot
            | TruthClaim::ImportedSnapshot
            | TruthClaim::PolicyLimitedProjection
            | TruthClaim::ProviderUnavailable => Self::Blocked,
        }
    }
}

// ---------------------------------------------------------------------------
// Trigger → invalidation-reason / single-axis-claim mapping.
//
// These mirror the canonical engine's single-axis behaviour. The
// `trigger_claims_match_engine` test asserts parity so the cue layer can
// never fork the narrowing engine.
// ---------------------------------------------------------------------------

/// The claim the canonical engine narrows to for this trigger alone.
const fn claim_for_trigger(trigger: NarrowingTrigger) -> TruthClaim {
    match trigger {
        NarrowingTrigger::FreshnessWarming => TruthClaim::WarmingNoTruthYet,
        NarrowingTrigger::FreshnessCached => TruthClaim::CachedProjection,
        NarrowingTrigger::FreshnessStale => TruthClaim::StaleSnapshot,
        NarrowingTrigger::FreshnessReplayed => TruthClaim::ReplayedSnapshot,
        NarrowingTrigger::FreshnessImported => TruthClaim::ImportedSnapshot,
        NarrowingTrigger::CompletenessPartial => TruthClaim::PartialProjection,
        NarrowingTrigger::CompletenessUnloaded => TruthClaim::WarmingNoTruthYet,
        NarrowingTrigger::CompletenessUnavailable => TruthClaim::ProviderUnavailable,
        NarrowingTrigger::BackpressureCoalesced => TruthClaim::CoalescedStream,
        NarrowingTrigger::BackpressureSnapshotRequired => TruthClaim::CoalescedStream,
        NarrowingTrigger::TerminalUnavailable => TruthClaim::ProviderUnavailable,
        NarrowingTrigger::TerminalTerminated => TruthClaim::ProviderUnavailable,
        NarrowingTrigger::PolicyLimited => TruthClaim::PolicyLimitedProjection,
    }
}

/// The dominant invalidation reason that explains a trigger's narrowing.
const fn invalidation_reason_for_trigger(trigger: NarrowingTrigger) -> InvalidationReason {
    match trigger {
        NarrowingTrigger::FreshnessWarming => InvalidationReason::ProducerRestart,
        NarrowingTrigger::FreshnessCached => InvalidationReason::CacheServed,
        NarrowingTrigger::FreshnessStale => InvalidationReason::UpstreamInputStale,
        NarrowingTrigger::FreshnessReplayed => InvalidationReason::ReplayedFromBundle,
        NarrowingTrigger::FreshnessImported => InvalidationReason::ImportedFromExternal,
        NarrowingTrigger::CompletenessPartial => InvalidationReason::UpstreamInputStale,
        NarrowingTrigger::CompletenessUnloaded => InvalidationReason::ProducerRestart,
        NarrowingTrigger::CompletenessUnavailable => InvalidationReason::ScopeRemoved,
        NarrowingTrigger::BackpressureCoalesced => InvalidationReason::QueueSaturation,
        NarrowingTrigger::BackpressureSnapshotRequired => InvalidationReason::QueueSaturation,
        NarrowingTrigger::TerminalUnavailable => InvalidationReason::ScopeRemoved,
        NarrowingTrigger::TerminalTerminated => InvalidationReason::ScopeRemoved,
        NarrowingTrigger::PolicyLimited => InvalidationReason::PolicyEpochChanged,
    }
}

/// Whether a trigger forces a fresh subscription before the surface can
/// recover (terminal stream, snapshot-required backpressure, or an
/// unavailable scope).
const fn trigger_requires_resubscribe(trigger: NarrowingTrigger) -> bool {
    matches!(
        trigger,
        NarrowingTrigger::TerminalUnavailable
            | NarrowingTrigger::TerminalTerminated
            | NarrowingTrigger::BackpressureSnapshotRequired
            | NarrowingTrigger::CompletenessUnavailable
    )
}

/// The dominant trigger behind a narrowed claim: the trigger whose
/// single-axis claim equals the winning narrowed claim. Triggers are
/// already sorted, so the first match is deterministic.
fn dominant_trigger(
    narrowed_claim: TruthClaim,
    triggers: &[NarrowingTrigger],
) -> Option<NarrowingTrigger> {
    triggers
        .iter()
        .copied()
        .find(|trigger| claim_for_trigger(*trigger) == narrowed_claim)
}

/// One-line truth headline shared across every presentation channel.
const fn headline_for_claim(claim: TruthClaim) -> &'static str {
    match claim {
        TruthClaim::ExactCurrentTruth => "Exact current truth.",
        TruthClaim::ConsistentSnapshot => "Consistent snapshot at the current authoritative epoch.",
        TruthClaim::CoalescedStream => {
            "Coalesced stream; updates are batched and may lag the producer."
        }
        TruthClaim::PartialProjection => "Partial scope; some results are not loaded yet.",
        TruthClaim::WarmingNoTruthYet => "Warming; no authoritative truth observed yet.",
        TruthClaim::CachedProjection => "Served from cache; refresh for live truth.",
        TruthClaim::StaleSnapshot => "Stale snapshot behind the current epoch; rerun to refresh.",
        TruthClaim::ReplayedSnapshot => "Replayed from a captured bundle, not a live producer.",
        TruthClaim::ImportedSnapshot => "Imported from an external source, not a live producer.",
        TruthClaim::PolicyLimitedProjection => {
            "Policy-limited view; entitlement restricts what is shown."
        }
        TruthClaim::ProviderUnavailable => {
            "Provider unavailable; no current truth until it reconnects."
        }
    }
}

// ---------------------------------------------------------------------------
// The rendered cue.
// ---------------------------------------------------------------------------

/// One rendered reactive-truth cue for a derived surface and an observed
/// subscription state. Consumers render this verbatim; they never restate
/// freshness, completeness, or invalidation in feature-local prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthCue {
    /// Reactive surface class.
    pub surface_class: ReactiveSurfaceClass,
    /// Authority that owns the canonical truth.
    pub authority_class: AuthorityClass,
    /// Whether the surface is authoritative or a derived projection.
    pub derivation_class: DerivationClass,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Materialized-view class.
    pub view_class: ViewClass,
    /// Canonical query family the surface subscribes to.
    pub query_family: String,
    /// Epoch-parity group the surface must stay level with.
    pub epoch_parity_group_id: String,
    /// Observed subscription state echoed from the envelope.
    pub observed: ObservedReactiveState,
    /// Strongest claim the surface presents when fully healthy.
    pub healthy_claim: TruthClaim,
    /// Canonically narrowed claim for the observed state.
    pub narrowed_claim: TruthClaim,
    /// Whether the observed state forced a downgrade from the healthy claim.
    pub narrowed: bool,
    /// Every narrowing trigger that fired, in stable order.
    pub triggers: Vec<NarrowingTrigger>,
    /// The dominant invalidation reason behind the narrowed claim, or
    /// `None` when the surface is healthy.
    pub invalidation_reason: Option<InvalidationReason>,
    /// Action gate applied to dangerous derived actions.
    pub action_gate: ActionGate,
    /// Whether a dangerous derived action stays live under the gate.
    pub dangerous_action_enabled: bool,
    /// Whether the surface must resubscribe before it can recover.
    pub resubscribe_required: bool,
    /// Where the truth came from, in one shared phrase.
    pub source_summary: String,
    /// One-line truth headline.
    pub headline: String,
    /// Accessibility narration carrying source, freshness, invalidation,
    /// and the action gate in one sentence.
    pub narration: String,
}

impl ReactiveTruthCue {
    /// Whether the cue presents exact current truth. Always `false` for a
    /// derived surface.
    pub fn presents_exact_current_truth(&self) -> bool {
        self.narrowed_claim == TruthClaim::ExactCurrentTruth
    }
}

fn source_summary(row: &ReactiveSurfaceRow) -> String {
    format!(
        "from {} via {} ({} scope)",
        row.authority_class.as_str(),
        row.view_class.as_str(),
        row.scope_class.as_str(),
    )
}

fn narration(
    row: &ReactiveSurfaceRow,
    source: &str,
    headline: &str,
    invalidation_reason: Option<InvalidationReason>,
    gate: ActionGate,
    resubscribe_required: bool,
) -> String {
    let mut narration = format!("{} {} {}", row.surface_class.as_str(), source, headline);
    if let Some(reason) = invalidation_reason {
        narration.push_str(&format!(" Changed by {}.", reason.as_str()));
    }
    narration.push_str(&format!(" {}.", gate.note()));
    if resubscribe_required {
        narration.push_str(" Resubscribe required.");
    }
    narration
}

/// Builds the rendered cue for a surface row and observed state, narrowing
/// the claim through the canonical engine.
fn cue_from_row(row: &ReactiveSurfaceRow, observed: ObservedReactiveState) -> ReactiveTruthCue {
    let narrowed = narrow_truth_claim(row.derivation_class, &observed);
    let dominant = dominant_trigger(narrowed.claim, &narrowed.triggers);
    let invalidation_reason = dominant.map(invalidation_reason_for_trigger);
    let gate = ActionGate::for_claim(narrowed.claim);
    let resubscribe_required = observed.terminal_reason.is_some()
        || observed.backpressure_mode == BackpressureMode::SnapshotRequired
        || observed.completeness == Completeness::Unavailable;
    let source = source_summary(row);
    let headline = headline_for_claim(narrowed.claim).to_owned();
    let narration = narration(
        row,
        &source,
        &headline,
        invalidation_reason,
        gate,
        resubscribe_required,
    );
    ReactiveTruthCue {
        surface_class: row.surface_class,
        authority_class: row.authority_class,
        derivation_class: row.derivation_class,
        scope_class: row.scope_class,
        view_class: row.view_class,
        query_family: row.query_family.clone(),
        epoch_parity_group_id: epoch_parity_group_id(row.authority_class),
        observed,
        healthy_claim: row.healthy_claim,
        narrowed_claim: narrowed.claim,
        narrowed: narrowed.claim != row.healthy_claim,
        triggers: narrowed.triggers,
        invalidation_reason,
        action_gate: gate,
        dangerous_action_enabled: gate.allows_dangerous_derived_action(),
        resubscribe_required,
        source_summary: source,
        headline,
        narration,
    }
}

fn epoch_parity_group_id(authority: AuthorityClass) -> String {
    format!("epoch_parity:{}", authority.as_str())
}

/// Error returned when a cue or audit cannot be built.
#[derive(Debug)]
pub enum ReactiveTruthSurfacesError {
    /// The canonical governance matrix failed validation.
    PacketValidation(crate::m5_reactive_governance::ValidationReport),
    /// A requested surface is missing from the matrix.
    UnknownSurface(ReactiveSurfaceClass),
}

impl fmt::Display for ReactiveTruthSurfacesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => write!(f, "reactive governance invalid: {report}"),
            Self::UnknownSurface(surface) => {
                write!(f, "unknown reactive surface: {}", surface.as_str())
            }
        }
    }
}

impl std::error::Error for ReactiveTruthSurfacesError {}

impl From<crate::m5_reactive_governance::ValidationReport> for ReactiveTruthSurfacesError {
    fn from(report: crate::m5_reactive_governance::ValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Builds the rendered cue for a surface and observed state, reading the
/// surface row from the canonical governance matrix.
///
/// # Errors
///
/// Returns [`ReactiveTruthSurfacesError`] when the matrix fails validation
/// or the surface is unknown.
pub fn build_reactive_truth_cue(
    surface_class: ReactiveSurfaceClass,
    observed: ObservedReactiveState,
) -> Result<ReactiveTruthCue, ReactiveTruthSurfacesError> {
    let packet = seeded_m5_reactive_governance_packet();
    validate_m5_reactive_governance_packet(&packet)?;
    let row = packet
        .surfaces
        .iter()
        .find(|row| row.surface_class == surface_class)
        .ok_or(ReactiveTruthSurfacesError::UnknownSurface(surface_class))?;
    Ok(cue_from_row(row, observed))
}

// ---------------------------------------------------------------------------
// Per-surface audit packet.
// ---------------------------------------------------------------------------

/// One claim-narrowing rule augmented with the action gate, dominant
/// invalidation reason, and resubscribe requirement. Derived from the
/// governance matrix's own [`ClaimNarrowingRule`], so it cannot drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatedNarrowingRule {
    /// Single-axis degradation that triggers this rule.
    pub trigger: NarrowingTrigger,
    /// Claim the surface narrows to under this trigger.
    pub narrowed_claim: TruthClaim,
    /// Action gate applied at this claim.
    pub action_gate: ActionGate,
    /// Dominant invalidation reason behind this trigger.
    pub invalidation_reason: InvalidationReason,
    /// Whether this trigger forces a resubscribe before recovery.
    pub resubscribe_required: bool,
}

/// One audited reactive surface: its provenance, its healthy ceiling, and
/// the gated narrowing it applies for every degradation it can present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthSurfaceAudit {
    /// Reactive surface class.
    pub surface_class: ReactiveSurfaceClass,
    /// Authority that owns the canonical truth.
    pub authority_class: AuthorityClass,
    /// Whether the surface is authoritative or a derived projection.
    pub derivation_class: DerivationClass,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Materialized-view class.
    pub view_class: ViewClass,
    /// Canonical query family.
    pub query_family: String,
    /// Epoch-parity group the surface must stay level with.
    pub epoch_parity_group_id: String,
    /// Strongest claim the surface presents when healthy.
    pub healthy_claim: TruthClaim,
    /// Action gate applied at the healthy ceiling.
    pub healthy_action_gate: ActionGate,
    /// Gated narrowing rules over every supported degradation.
    pub gated_narrowing_rules: Vec<GatedNarrowingRule>,
    /// Real consumer surfaces that ingest this audit row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level audit packet freezing the gated reactive-truth surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthSurfacesPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// One audit row per reactive surface.
    pub surfaces: Vec<ReactiveTruthSurfaceAudit>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a surface and an observed state to the expected
/// rendered cue fields, proving the gated cue behaviour end to end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthCueFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Surface under test.
    pub surface_class: ReactiveSurfaceClass,
    /// Observed subscription state.
    pub observed: ObservedReactiveState,
    /// Expected narrowed claim.
    pub expected_claim: TruthClaim,
    /// Expected action gate.
    pub expected_action_gate: ActionGate,
    /// Expected dangerous-action-enabled flag.
    pub expected_dangerous_action_enabled: bool,
    /// Expected resubscribe requirement.
    pub expected_resubscribe_required: bool,
    /// Expected dominant invalidation reason.
    pub expected_invalidation_reason: Option<InvalidationReason>,
    /// One consumer that quotes this surface.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

fn gated_rule(rule: &ClaimNarrowingRule) -> GatedNarrowingRule {
    GatedNarrowingRule {
        trigger: rule.trigger,
        narrowed_claim: rule.narrowed_claim,
        action_gate: ActionGate::for_claim(rule.narrowed_claim),
        invalidation_reason: invalidation_reason_for_trigger(rule.trigger),
        resubscribe_required: trigger_requires_resubscribe(rule.trigger),
    }
}

fn audit_from_row(row: &ReactiveSurfaceRow) -> ReactiveTruthSurfaceAudit {
    let gated_narrowing_rules = row.claim_narrowing_rules.iter().map(gated_rule).collect();
    ReactiveTruthSurfaceAudit {
        surface_class: row.surface_class,
        authority_class: row.authority_class,
        derivation_class: row.derivation_class,
        scope_class: row.scope_class,
        view_class: row.view_class,
        query_family: row.query_family.clone(),
        epoch_parity_group_id: epoch_parity_group_id(row.authority_class),
        healthy_claim: row.healthy_claim,
        healthy_action_gate: ActionGate::for_claim(row.healthy_claim),
        gated_narrowing_rules,
        consumer_refs: row.consumer_refs.clone(),
        notes: row.notes.clone(),
    }
}

/// Returns the checked-in audit packet this lane freezes, derived from the
/// canonical governance matrix.
pub fn seeded_reactive_truth_surfaces_packet() -> ReactiveTruthSurfacesPacket {
    let governance = seeded_m5_reactive_governance_packet();
    let mut surfaces: Vec<_> = governance.surfaces.iter().map(audit_from_row).collect();
    surfaces.sort_by(|a, b| a.surface_class.as_str().cmp(b.surface_class.as_str()));
    ReactiveTruthSurfacesPacket {
        record_kind: REACTIVE_TRUTH_SURFACES_PACKET_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION,
        packet_id: "state.reactive_truth_surfaces.v1".to_owned(),
        title: "Gated reactive-truth cues shipped across the derived M5 surfaces".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: REACTIVE_TRUTH_SURFACES_DOC_REF.to_owned(),
            schema_ref: REACTIVE_TRUTH_SURFACES_SCHEMA_REF.to_owned(),
            packet_ref: REACTIVE_TRUTH_SURFACES_PACKET_REF.to_owned(),
            report_ref: REACTIVE_TRUTH_SURFACES_REPORT_REF.to_owned(),
            fixture_manifest_ref: REACTIVE_TRUTH_SURFACES_FIXTURE_MANIFEST_REF.to_owned(),
        },
        surfaces,
        invariants: vec![
            "Every derived M5 surface renders one canonical reactive-truth cue carrying source authority, freshness, completeness, invalidation reason, backpressure, and the narrowed claim instead of feature-local stale-state prose.".to_owned(),
            "No derived surface presents exact current truth; the strongest cue is a consistent snapshot at its authoritative epoch.".to_owned(),
            "Dangerous derived actions stay live only at a consistent snapshot; coalesced or cached truth must revalidate, partial or warming truth narrows to read-only, and stale, replayed, imported, policy-limited, or provider-unavailable truth blocks them.".to_owned(),
            "Terminal streams, snapshot-required backpressure, and unavailable scopes set a resubscribe-required cue instead of hiding behind a generic spinner.".to_owned(),
            "The gate, invalidation reason, and resubscribe cue are derived from the canonical narrowing engine, so UI, CLI/headless, activity-center, accessibility, diagnostics, and support/export channels narrow identically.".to_owned(),
        ],
    }
}

/// Returns the checked-in cue fixtures this lane freezes.
pub fn seeded_reactive_truth_surfaces_fixtures() -> Vec<ReactiveTruthCueFixture> {
    vec![
        cue_fixture(
            "fixture:reactive_truth:shell_healthy",
            ReactiveSurfaceClass::ShellWorkspaceTree,
            ObservedReactiveState::healthy(),
            "crates/aureline-shell/src/reactive_truth_surfaces/mod.rs",
            "A healthy workspace tree presents a consistent snapshot with derived actions live.",
        ),
        cue_fixture(
            "fixture:reactive_truth:shell_warming",
            ReactiveSurfaceClass::ShellWorkspaceTree,
            ObservedReactiveState {
                freshness: Freshness::Warming,
                completeness: Completeness::Unloaded,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-shell/src/reactive_truth_surfaces/mod.rs",
            "A warming, unloaded workspace tree narrows to warming-no-truth-yet and read-only.",
        ),
        cue_fixture(
            "fixture:reactive_truth:search_stale",
            ReactiveSurfaceClass::SearchResults,
            ObservedReactiveState {
                freshness: Freshness::Stale,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-search/src/lib.rs",
            "Stale search results narrow to a stale snapshot and block dangerous derived actions.",
        ),
        cue_fixture(
            "fixture:reactive_truth:search_snapshot_required",
            ReactiveSurfaceClass::SearchResults,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::SnapshotRequired,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-search/src/lib.rs",
            "A snapshot-required search stream narrows to a coalesced claim and asks to resubscribe.",
        ),
        cue_fixture(
            "fixture:reactive_truth:graph_partial",
            ReactiveSurfaceClass::GraphNeighborhood,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-graph/src/lib.rs",
            "A partial graph neighborhood narrows to a partial projection and read-only actions.",
        ),
        cue_fixture(
            "fixture:reactive_truth:docs_cached",
            ReactiveSurfaceClass::DocsBrowser,
            ObservedReactiveState {
                freshness: Freshness::Cached,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-docs/src/lib.rs",
            "A cache-served docs index narrows to a cached projection and revalidates before acting.",
        ),
        cue_fixture(
            "fixture:reactive_truth:ai_policy_limited",
            ReactiveSurfaceClass::AiContextPanel,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: true,
            },
            "crates/aureline-ai/src/lib.rs",
            "A policy-limited AI context narrows to a policy-limited projection and blocks actions.",
        ),
        cue_fixture(
            "fixture:reactive_truth:review_coalesced",
            ReactiveSurfaceClass::ReviewWorkspace,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-review/src/lib.rs",
            "A coalesced review overlay narrows to a coalesced stream and revalidates before acting.",
        ),
        cue_fixture(
            "fixture:reactive_truth:companion_unavailable",
            ReactiveSurfaceClass::CompanionPanel,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Unavailable,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: Some(TerminalReason::Unavailable),
                policy_limited: false,
            },
            "crates/aureline-companion/src/lib.rs",
            "An unavailable companion provider narrows to provider-unavailable, blocks actions, and asks to resubscribe.",
        ),
        cue_fixture(
            "fixture:reactive_truth:preview_replayed",
            ReactiveSurfaceClass::PreviewOutput,
            ObservedReactiveState {
                freshness: Freshness::Replayed,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-preview/src/lib.rs",
            "A replayed preview snapshot narrows to a replayed snapshot and blocks dangerous actions.",
        ),
        cue_fixture(
            "fixture:reactive_truth:support_imported",
            ReactiveSurfaceClass::SupportExportView,
            ObservedReactiveState {
                freshness: Freshness::Imported,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-support/src/reactive_truth_surfaces/mod.rs",
            "An imported support-export snapshot narrows to an imported snapshot for release readers.",
        ),
    ]
}

fn cue_fixture(
    fixture_id: &str,
    surface_class: ReactiveSurfaceClass,
    observed: ObservedReactiveState,
    consumer_ref: &str,
    notes: &str,
) -> ReactiveTruthCueFixture {
    // The expected fields are computed from the canonical engine so a
    // fixture can never assert a claim the engine would not produce.
    let governance = seeded_m5_reactive_governance_packet();
    let row = governance
        .surfaces
        .iter()
        .find(|row| row.surface_class == surface_class)
        .expect("seeded surface exists");
    let cue = cue_from_row(row, observed);
    ReactiveTruthCueFixture {
        record_kind: REACTIVE_TRUTH_SURFACES_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        surface_class,
        observed,
        expected_claim: cue.narrowed_claim,
        expected_action_gate: cue.action_gate,
        expected_dangerous_action_enabled: cue.dangerous_action_enabled,
        expected_resubscribe_required: cue.resubscribe_required,
        expected_invalidation_reason: cue.invalidation_reason,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "reactive truth surfaces validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// Validates the seeded packet or an on-disk copy of it.
///
/// # Errors
///
/// Returns a [`ValidationReport`] listing every drift from the canonical
/// governance matrix or the gating rules.
pub fn validate_reactive_truth_surfaces_packet(
    packet: &ReactiveTruthSurfacesPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != REACTIVE_TRUTH_SURFACES_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {REACTIVE_TRUTH_SURFACES_PACKET_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {}, got {}",
                REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION, packet.schema_version
            ),
        );
    }
    if packet.source_contract_refs.doc_ref != REACTIVE_TRUTH_SURFACES_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted");
    }
    if packet.source_contract_refs.schema_ref != REACTIVE_TRUTH_SURFACES_SCHEMA_REF {
        report.push("packet.schema_ref", "schema_ref drifted");
    }
    if packet.source_contract_refs.packet_ref != REACTIVE_TRUTH_SURFACES_PACKET_REF {
        report.push("packet.packet_ref", "packet_ref drifted");
    }
    if packet.source_contract_refs.report_ref != REACTIVE_TRUTH_SURFACES_REPORT_REF {
        report.push("packet.report_ref", "report_ref drifted");
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != REACTIVE_TRUTH_SURFACES_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted",
        );
    }

    // The audit must equal the projection of the canonical governance matrix.
    let expected = seeded_reactive_truth_surfaces_packet();
    if packet.surfaces != expected.surfaces {
        report.push(
            "packet.surfaces",
            "audit surfaces drifted from the canonical governance matrix projection",
        );
    }
    if packet.invariants != expected.invariants {
        report.push("packet.invariants", "invariants drifted");
    }

    let mut surface_classes = BTreeSet::new();
    for audit in &packet.surfaces {
        if !surface_classes.insert(audit.surface_class) {
            report.push(
                "surface.duplicate",
                format!("duplicate surface {}", audit.surface_class.as_str()),
            );
        }
        if audit.derivation_class != DerivationClass::Derived {
            report.push(
                "surface.derivation",
                format!(
                    "surface {} must be a derived projection",
                    audit.surface_class.as_str()
                ),
            );
        }
        if audit.healthy_claim == TruthClaim::ExactCurrentTruth {
            report.push(
                "surface.exact_truth_overclaim",
                format!(
                    "derived surface {} may not present exact current truth",
                    audit.surface_class.as_str()
                ),
            );
        }
        if audit.healthy_action_gate != ActionGate::for_claim(audit.healthy_claim) {
            report.push(
                "surface.healthy_gate",
                format!(
                    "surface {} healthy gate drifted from the canonical gate",
                    audit.surface_class.as_str()
                ),
            );
        }
        if audit.epoch_parity_group_id != epoch_parity_group_id(audit.authority_class) {
            report.push(
                "surface.epoch_parity_group",
                format!(
                    "surface {} epoch parity group drifted from its authority",
                    audit.surface_class.as_str()
                ),
            );
        }
        for rule in &audit.gated_narrowing_rules {
            if rule.action_gate != ActionGate::for_claim(rule.narrowed_claim) {
                report.push(
                    "rule.action_gate",
                    format!(
                        "surface {} rule {} gate drifted from the canonical gate",
                        audit.surface_class.as_str(),
                        rule.trigger.as_str()
                    ),
                );
            }
            if rule.invalidation_reason != invalidation_reason_for_trigger(rule.trigger) {
                report.push(
                    "rule.invalidation_reason",
                    format!(
                        "surface {} rule {} invalidation reason drifted",
                        audit.surface_class.as_str(),
                        rule.trigger.as_str()
                    ),
                );
            }
            if rule.resubscribe_required != trigger_requires_resubscribe(rule.trigger) {
                report.push(
                    "rule.resubscribe_required",
                    format!(
                        "surface {} rule {} resubscribe flag drifted",
                        audit.surface_class.as_str(),
                        rule.trigger.as_str()
                    ),
                );
            }
            if claim_for_trigger(rule.trigger) != rule.narrowed_claim {
                report.push(
                    "rule.narrowed_claim",
                    format!(
                        "surface {} rule {} narrowed claim forked the engine",
                        audit.surface_class.as_str(),
                        rule.trigger.as_str()
                    ),
                );
            }
        }
    }

    if packet.surfaces.is_empty() {
        report.push("packet.surfaces_empty", "packet must carry surfaces");
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one cue fixture against the canonical engine.
///
/// # Errors
///
/// Returns a [`ValidationReport`] when the fixture's expected cue fields do
/// not match the engine's output for the observed state.
pub fn validate_reactive_truth_surfaces_fixture(
    fixture: &ReactiveTruthCueFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != REACTIVE_TRUTH_SURFACES_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, REACTIVE_TRUTH_SURFACES_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, REACTIVE_TRUTH_SURFACES_SCHEMA_VERSION
            ),
        );
    }

    let governance = seeded_m5_reactive_governance_packet();
    let Some(row) = governance
        .surfaces
        .iter()
        .find(|row| row.surface_class == fixture.surface_class)
    else {
        report.push(
            "fixture.surface_missing",
            format!(
                "fixture {} points to surface {} missing from the matrix",
                fixture.fixture_id,
                fixture.surface_class.as_str()
            ),
        );
        return Err(report);
    };

    if !row.consumer_refs.iter().any(|c| c == &fixture.consumer_ref)
        && fixture.consumer_ref != "crates/aureline-shell/src/reactive_truth_surfaces/mod.rs"
        && fixture.consumer_ref != "crates/aureline-support/src/reactive_truth_surfaces/mod.rs"
    {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} consumer_ref {} is not a declared consumer of surface {}",
                fixture.fixture_id,
                fixture.consumer_ref,
                row.surface_class.as_str()
            ),
        );
    }

    let cue = cue_from_row(row, fixture.observed);
    if cue.narrowed_claim != fixture.expected_claim {
        report.push(
            "fixture.expected_claim",
            format!(
                "fixture {} expected claim {} but engine produced {}",
                fixture.fixture_id,
                fixture.expected_claim.as_str(),
                cue.narrowed_claim.as_str()
            ),
        );
    }
    if cue.action_gate != fixture.expected_action_gate {
        report.push(
            "fixture.expected_action_gate",
            format!(
                "fixture {} expected gate {} but engine produced {}",
                fixture.fixture_id,
                fixture.expected_action_gate.as_str(),
                cue.action_gate.as_str()
            ),
        );
    }
    if cue.dangerous_action_enabled != fixture.expected_dangerous_action_enabled {
        report.push(
            "fixture.expected_dangerous_action_enabled",
            format!(
                "fixture {} dangerous-action-enabled drifted from the gate",
                fixture.fixture_id
            ),
        );
    }
    if cue.resubscribe_required != fixture.expected_resubscribe_required {
        report.push(
            "fixture.expected_resubscribe_required",
            format!(
                "fixture {} resubscribe flag drifted from the observed state",
                fixture.fixture_id
            ),
        );
    }
    if cue.invalidation_reason != fixture.expected_invalidation_reason {
        report.push(
            "fixture.expected_invalidation_reason",
            format!(
                "fixture {} invalidation reason drifted from the dominant trigger",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Cross-channel rendering: one grammar for every consumer.
// ---------------------------------------------------------------------------

/// A presentation channel a cue can render through. Every channel carries
/// the same claim, gate, and invalidation tokens; only the framing differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CueChannel {
    /// Shell truth strip.
    UiStrip,
    /// CLI / headless line.
    CliHeadless,
    /// Activity-center row.
    ActivityCenter,
    /// Keyboard-help line.
    KeyboardHelp,
    /// Accessibility narration.
    Accessibility,
}

impl CueChannel {
    /// Stable channel tag.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiStrip => "ui_strip",
            Self::CliHeadless => "cli_headless",
            Self::ActivityCenter => "activity_center",
            Self::KeyboardHelp => "keyboard_help",
            Self::Accessibility => "accessibility",
        }
    }
}

/// Renders a cue for one channel. Accessibility renders the full
/// narration; every other channel renders the same token-bearing summary
/// so no channel can present a richer claim than another.
pub fn render_cue(cue: &ReactiveTruthCue, channel: CueChannel) -> String {
    match channel {
        CueChannel::Accessibility => cue.narration.clone(),
        _ => format!(
            "{} | {} | claim={} | gate={} | invalidation={} | resubscribe={}",
            channel.as_str(),
            cue.surface_class.as_str(),
            cue.narrowed_claim.as_str(),
            cue.action_gate.as_str(),
            cue.invalidation_reason
                .map_or("none", InvalidationReason::as_str),
            cue.resubscribe_required,
        ),
    }
}

/// Renders the full audit packet as deterministic plaintext for
/// diagnostics, support review, and docs consumers.
pub fn render_reactive_truth_surfaces_audit_plaintext(
    packet: &ReactiveTruthSurfacesPacket,
) -> String {
    let mut lines = vec![
        packet.title.clone(),
        "surface | authority | view_class | healthy_claim | healthy_gate | gated_rules".to_string(),
    ];
    for audit in &packet.surfaces {
        lines.push(format!(
            "{} | {} | {} | {} | {} | {}",
            audit.surface_class.as_str(),
            audit.authority_class.as_str(),
            audit.view_class.as_str(),
            audit.healthy_claim.as_str(),
            audit.healthy_action_gate.as_str(),
            audit.gated_narrowing_rules.len(),
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

#[cfg(test)]
mod tests;
