//! Entitlement summaries that make the current managed state legible across the
//! M5 managed surfaces.
//!
//! This module is the canonical entitlement-summary object. Where the sibling
//! [`crate::m5_commercial_control_plane`] freezes the per-lane matrix, this
//! module renders the *account-context* view a surface shows a user: the plan,
//! the role, the seat owner, the org/tenant scope, the entitlement label, the
//! quota-snapshot age, and the non-empty local-only continuation notes that say
//! what keeps working on device when managed features degrade or expire.
//!
//! An [`EntitlementSummarySet`] freezes one [`EntitlementSummary`] per managed
//! state in the locked ten-token vocabulary and one [`SummarySurfaceBinding`]
//! per surface that must project them — the account/seat surface, diagnostics,
//! the support/admin packet, Help/About, and the feature entry points. Each
//! summary reuses the closed vocabularies already frozen by the control-plane
//! matrix — [`ManagedStateClass`], [`EntitlementState`], [`ScopeOwner`],
//! [`MarketedClaim`], [`MeterUnit`], [`AggregationWindow`], and
//! [`PostureOrigin`] — instead of minting a parallel synonym set.
//!
//! Three invariants keep the summaries honest. First, **local-use continuation
//! is never implicit**: every summary carries a non-empty
//! [`EntitlementSummary::local_only_continuation`], so a surface always says what
//! stays usable locally. Second, **seat loss and expiry degrade to explicit
//! managed-blocked states, never a generic sign-in error**: a removed seat is
//! cited to the seat and an expired entitlement to the policy or seat origin —
//! distinct from the reauthentication (sign-in) family — and the
//! [`DegradationKind`] and [`EntitlementSummary::posture_origin`] are recomputed
//! from the state so a forged generic-error summary fails
//! [`EntitlementSummarySet::validate`]. Third, **no metered number crosses the
//! boundary bare**: a [`QuotaSnapshotDescriptor`] carries the unit, aggregation
//! window, scope owner, as-of time, and freshness class but never a raw spend or
//! quota number, so the snapshot age is legible without exposing billing values,
//! and a stale meter narrows the managed claim without ever blocking the local
//! core.
//!
//! [`canonical_entitlement_summary_set`] builds the frozen set and
//! [`current_stable_entitlement_summary_set`] reads and validates the checked-in
//! packet at
//! [`artifacts/service/m5-entitlement-summary.json`](../../../../artifacts/service/m5-entitlement-summary.json),
//! so account, diagnostics, support/admin, Help/About, and feature-entry
//! surfaces all ingest one packet rather than cloning status text. The boundary
//! schema is
//! [`schemas/service/m5-entitlement-summary.schema.json`](../../../../schemas/service/m5-entitlement-summary.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, AggregationWindow, EntitlementState, ManagedStateClass, MarketedClaim,
    MeterUnit, PostureOrigin, ScopeOwner,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the entitlement-summary set.
pub const ENTITLEMENT_SUMMARY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the summary-set packet.
pub const SUMMARY_SET_RECORD_KIND: &str = "m5_entitlement_summary_set";

/// Stable record-kind tag for a single entitlement summary.
pub const SUMMARY_RECORD_KIND: &str = "m5_entitlement_summary";

/// Stable record-kind tag for a quota-snapshot descriptor.
pub const QUOTA_SNAPSHOT_RECORD_KIND: &str = "m5_quota_snapshot_descriptor";

/// Stable record-kind tag for a surface binding.
pub const SUMMARY_SURFACE_BINDING_RECORD_KIND: &str = "m5_entitlement_summary_surface_binding";

/// Stable record-kind tag for the summary-set inspection block.
pub const SUMMARY_INSPECTION_RECORD_KIND: &str = "m5_entitlement_summary_inspection";

/// Repo-relative path to the boundary schema.
pub const ENTITLEMENT_SUMMARY_SCHEMA_REF: &str =
    "schemas/service/m5-entitlement-summary.schema.json";

/// Repo-relative path to the reviewer contract.
pub const ENTITLEMENT_SUMMARY_DOC_REF: &str = "docs/m5/implement-entitlement-summaries-with-plan-seat-owner-role-quota-snapshot-age-and-local-only-continuation-notes-across-m5-managed-surfaces.md";

/// Repo-relative path to the checked-in summary-set packet.
pub const ENTITLEMENT_SUMMARY_ARTIFACT_PATH: &str = "artifacts/service/m5-entitlement-summary.json";

/// Closed plan-tier vocabulary.
///
/// The tier is always present alongside the friendly plan label so the label
/// never hides the entitlement freshness or owner scope behind a marketing name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTier {
    /// No managed plan; the install is local-only.
    LocalOnlyNoPlan,
    /// An individual free tier.
    IndividualFree,
    /// An individual paid tier.
    IndividualPro,
    /// A team tier.
    TeamPlan,
    /// An enterprise tier.
    EnterprisePlan,
}

impl PlanTier {
    /// Every plan tier, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnlyNoPlan,
        Self::IndividualFree,
        Self::IndividualPro,
        Self::TeamPlan,
        Self::EnterprisePlan,
    ];
}

/// Closed account-role vocabulary for the seat the summary describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountRole {
    /// The organization owner.
    OrgOwner,
    /// An organization administrator.
    OrgAdmin,
    /// A billing administrator.
    BillingAdmin,
    /// An ordinary member.
    Member,
    /// A read-only viewer.
    Viewer,
    /// No managed role; the install is local-only.
    NoManagedRole,
}

/// Closed snapshot-freshness vocabulary for the entitlement and quota snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotFreshness {
    /// The snapshot is live (confirmed against the control plane now).
    FreshnessLive,
    /// The snapshot is recent and within its freshness budget.
    FreshnessRecent,
    /// The snapshot is aging and approaching its freshness budget.
    FreshnessAging,
    /// The snapshot is stale; the number cannot be confirmed now and is labeled stale.
    FreshnessStale,
    /// The snapshot freshness is unknown (never refreshed or offline since first cache).
    FreshnessUnknown,
    /// No managed snapshot applies (local-only operation).
    FreshnessNotApplicable,
}

/// How the managed claim degrades in this summary's state.
///
/// The kind is recomputed from the [`ManagedStateClass`], so a seat loss or an
/// expiry always resolves to [`DegradationKind::ManagedBlockedExplicit`] — never
/// a generic sign-in error and never a silent local-only collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationKind {
    /// No degradation; managed actions are admissible.
    NoDegradation,
    /// No managed account; the local core is fully usable and managed actions do not apply.
    LocalOnlyNoAccount,
    /// Managed actions are explicitly blocked (seat removed, expired, or suspended).
    ManagedBlockedExplicit,
    /// Managed actions are narrowed but continue in a reduced form.
    ManagedNarrowed,
}

impl DegradationKind {
    /// The degradation kind a managed state always resolves to.
    ///
    /// `signed in` does not degrade; `local only` is the no-account path;
    /// `managed blocked` and `seat removed` are explicit blocks; every other
    /// state narrows.
    pub const fn for_state(state: ManagedStateClass) -> Self {
        match state {
            ManagedStateClass::SignedIn => Self::NoDegradation,
            ManagedStateClass::LocalOnly => Self::LocalOnlyNoAccount,
            ManagedStateClass::ManagedBlocked | ManagedStateClass::SeatRemoved => {
                Self::ManagedBlockedExplicit
            }
            ManagedStateClass::ReauthRequired
            | ManagedStateClass::GracePeriod
            | ManagedStateClass::PlanDowngrade
            | ManagedStateClass::OrgSwitched
            | ManagedStateClass::ForecastThreshold
            | ManagedStateClass::MeterStale => Self::ManagedNarrowed,
        }
    }
}

/// The posture origin a managed state is always cited back to.
///
/// A removed seat resolves to [`PostureOrigin::Seat`] and an org switch to
/// [`PostureOrigin::Org`], distinct from the reauthentication (sign-in) family
/// at [`PostureOrigin::Account`], so a seat loss can never collapse into a
/// generic sign-in or account error.
pub const fn expected_posture_origin(state: ManagedStateClass) -> PostureOrigin {
    match state {
        ManagedStateClass::SignedIn => PostureOrigin::Account,
        ManagedStateClass::LocalOnly => PostureOrigin::LocalOnlyNoManagedAccount,
        ManagedStateClass::ReauthRequired => PostureOrigin::Account,
        ManagedStateClass::ManagedBlocked => PostureOrigin::Policy,
        ManagedStateClass::GracePeriod => PostureOrigin::Account,
        ManagedStateClass::SeatRemoved => PostureOrigin::Seat,
        ManagedStateClass::PlanDowngrade => PostureOrigin::Plan,
        ManagedStateClass::OrgSwitched => PostureOrigin::Org,
        ManagedStateClass::ForecastThreshold => PostureOrigin::MeteringQuota,
        ManagedStateClass::MeterStale => PostureOrigin::MeteringQuota,
    }
}

/// True when `entitlement` is a coherent entitlement state for `state`.
fn entitlement_state_is_coherent(state: ManagedStateClass, entitlement: EntitlementState) -> bool {
    match state {
        ManagedStateClass::SignedIn => entitlement == EntitlementState::EntitlementActive,
        ManagedStateClass::LocalOnly => entitlement == EntitlementState::EntitlementNotApplicable,
        ManagedStateClass::ReauthRequired => {
            entitlement == EntitlementState::EntitlementPendingRecheck
        }
        // A block or a seat loss may be an admin suspension or a hard expiry.
        ManagedStateClass::ManagedBlocked | ManagedStateClass::SeatRemoved => matches!(
            entitlement,
            EntitlementState::EntitlementSuspendedAdmin | EntitlementState::EntitlementExpired
        ),
        ManagedStateClass::GracePeriod => entitlement == EntitlementState::EntitlementInGrace,
        ManagedStateClass::PlanDowngrade => entitlement == EntitlementState::EntitlementActive,
        ManagedStateClass::OrgSwitched => {
            entitlement == EntitlementState::EntitlementPendingRecheck
        }
        ManagedStateClass::ForecastThreshold => entitlement == EntitlementState::EntitlementActive,
        ManagedStateClass::MeterStale => entitlement == EntitlementState::EntitlementPendingRecheck,
    }
}

/// A quota-snapshot descriptor: unit, window, scope owner, as-of time, and
/// freshness — never a raw spend or quota number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaSnapshotDescriptor {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The meter unit the snapshot would be expressed in.
    pub meter_unit: MeterUnit,
    /// The aggregation window the snapshot covers.
    pub aggregation_window: AggregationWindow,
    /// The scope that owns the metered quota.
    pub scope_owner: ScopeOwner,
    /// The as-of measurement time for the snapshot.
    pub as_of: String,
    /// The freshness class of the snapshot.
    pub freshness: SnapshotFreshness,
    /// Always false: a raw spend or quota number never crosses this boundary.
    pub carries_raw_number: bool,
}

/// One frozen entitlement summary for a single managed state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementSummary {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable summary identifier.
    pub summary_id: String,
    /// Reviewable summary title.
    pub title: String,
    /// Reviewable one-line summary.
    pub summary: String,
    /// The managed state this summary renders.
    pub managed_state: ManagedStateClass,
    /// The frozen entitlement state behind the managed state.
    pub entitlement_state: EntitlementState,
    /// Reviewable entitlement label (for example, "Team plan — active").
    pub entitlement_label: String,
    /// The plan tier; always present so the friendly label never hides it.
    pub plan_tier: PlanTier,
    /// The friendly plan label shown to the user.
    pub plan_label: String,
    /// The account role for this seat.
    pub role: AccountRole,
    /// The scope that owns the seat.
    pub seat_owner_scope: ScopeOwner,
    /// Opaque reference to the seat owner; never a raw account id or name.
    pub seat_owner_ref: String,
    /// The org or tenant scope the account resolves through.
    pub account_scope: ScopeOwner,
    /// Opaque reference to the account scope; never a raw tenant id or name.
    pub account_scope_ref: String,
    /// The posture origin the state is cited back to.
    pub posture_origin: PostureOrigin,
    /// How the managed claim degrades in this state.
    pub degradation: DegradationKind,
    /// The marketed claim after the state's cap is applied.
    pub effective_marketed_claim: MarketedClaim,
    /// The quota snapshot descriptor, when a managed quota applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota_snapshot: Option<QuotaSnapshotDescriptor>,
    /// Non-empty notes on what stays usable locally in this state.
    pub local_only_continuation: Vec<String>,
    /// Managed-only actions that pause in this state.
    pub blocked_managed_only_actions: Vec<String>,
    /// The disclosure the surface must render in this state.
    pub required_disclosure: String,
    /// The next user-visible step in this state.
    pub recovery_cue: String,
}

impl EntitlementSummary {
    /// True when this summary still backs the full managed claim.
    pub fn backs_full_managed_claim(&self) -> bool {
        self.effective_marketed_claim == MarketedClaim::ManagedFull
    }

    /// True when this summary degrades to an explicit managed-blocked state.
    ///
    /// Seat loss and expiry always satisfy this, so a surface can render them as
    /// an explicit block rather than a generic sign-in error.
    pub fn is_explicitly_blocked(&self) -> bool {
        self.degradation == DegradationKind::ManagedBlockedExplicit
    }
}

/// Closed surface vocabulary that must project the entitlement summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummarySurface {
    /// The account, seat, and usage surface.
    AccountSurface,
    /// Diagnostics and service-health surfaces.
    Diagnostics,
    /// Support and admin export packets.
    SupportAdminPacket,
    /// The Help/About truth surface.
    HelpAbout,
    /// Managed-feature entry points.
    FeatureEntryPoint,
}

impl SummarySurface {
    /// Every surface the summaries must reach.
    pub const ALL: [Self; 5] = [
        Self::AccountSurface,
        Self::Diagnostics,
        Self::SupportAdminPacket,
        Self::HelpAbout,
        Self::FeatureEntryPoint,
    ];
}

/// One surface bound to the entitlement summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummarySurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The surface that projects the summaries.
    pub surface: SummarySurface,
    /// The summary ids this surface resolves through.
    pub bound_summary_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the local-only continuation notes.
    pub renders_local_only_continuation: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the summary set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementSummaryInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of entitlement summaries.
    pub summary_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// True when all ten managed-state tokens appear exactly once.
    pub managed_state_vocab_complete: bool,
    /// True when all five surfaces are bound.
    pub surface_coverage_complete: bool,
    /// True when every summary carries a non-empty local-only continuation note.
    pub all_summaries_carry_local_only_continuation: bool,
    /// True when no summary renders a generic account error in place of a typed state.
    pub no_generic_account_error: bool,
    /// Number of summaries still backing the full managed claim.
    pub full_claim_count: usize,
    /// Number of summaries narrowed to a reduced managed claim.
    pub managed_narrowed_count: usize,
    /// Number of summaries narrowed to the local-safe-only claim.
    pub local_safe_only_count: usize,
    /// Number of summaries that degrade to an explicit managed-blocked state.
    pub explicit_blocked_count: usize,
}

/// The frozen entitlement-summary set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementSummarySet {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable set identifier.
    pub set_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the set content.
    pub set_revision: u32,
    /// Reviewable set title.
    pub title: String,
    /// Reviewable set summary.
    pub summary: String,
    /// Source schema and contract refs the set cites.
    pub source_refs: Vec<String>,
    /// The entitlement summaries.
    pub summaries: Vec<EntitlementSummary>,
    /// The surface bindings.
    pub surface_bindings: Vec<SummarySurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: EntitlementSummaryInspection,
}

impl EntitlementSummarySet {
    /// Returns the summary that renders `state`, when one is frozen.
    pub fn summary_for_state(&self, state: ManagedStateClass) -> Option<&EntitlementSummary> {
        self.summaries.iter().find(|s| s.managed_state == state)
    }

    /// Serializes the set as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the set cannot be serialized, which a validated set never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("entitlement-summary set serializes to JSON")
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`EntitlementSummaryViolation`] per failed invariant: a wrong
    /// record kind or schema version, a missing identifier, a duplicate summary,
    /// an incomplete managed-state vocabulary, an empty local-only continuation,
    /// a recomputed degradation/claim/posture-origin mismatch, an incoherent
    /// entitlement state, an expiry that does not degrade to an explicit block, a
    /// quota descriptor that carries a raw number, an unbound surface, or a stale
    /// inspection block.
    pub fn validate(&self) -> Vec<EntitlementSummaryViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(EntitlementSummaryViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != SUMMARY_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != ENTITLEMENT_SUMMARY_SCHEMA_VERSION {
            push("schema_version", "set schema_version is wrong");
        }
        if self.set_id.trim().is_empty() {
            push("set_id", "set_id must be non-empty");
        }
        if self.generated_at.trim().is_empty() {
            push("generated_at", "generated_at must be non-empty");
        }
        if self.title.trim().is_empty() {
            push("title", "title must be non-empty");
        }
        if self.summary.trim().is_empty() {
            push("summary", "summary must be non-empty");
        }
        if self.set_revision == 0 {
            push("set_revision", "set_revision must be at least 1");
        }
        if !self
            .source_refs
            .iter()
            .any(|entry| entry == ENTITLEMENT_SUMMARY_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.summaries.is_empty() {
            push("summaries", "set must contain at least one summary");
        }

        let mut summary_ids = BTreeSet::new();
        let mut seen_states = BTreeSet::new();
        for summary in &self.summaries {
            self.validate_summary(summary, &mut push);
            if !summary_ids.insert(summary.summary_id.as_str()) {
                push("summaries", "summary_id values must be unique");
            }
            if !seen_states.insert(summary.managed_state) {
                push(
                    "summaries",
                    "each managed-state token must be summarized at most once",
                );
            }
        }
        // The full managed-state vocabulary must be summarized.
        if seen_states.len() != ManagedStateClass::ALL.len() {
            push(
                "summaries",
                "the set must summarize all ten managed-state tokens",
            );
        }

        self.validate_surface_bindings(&mut push);

        let derived = EntitlementSummaryInspection::derive(&self.summaries, &self.surface_bindings);
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed set",
            );
        }

        violations
    }

    fn validate_summary(&self, s: &EntitlementSummary, push: &mut impl FnMut(&str, &str)) {
        if s.record_kind != SUMMARY_RECORD_KIND {
            push("summary.record_kind", "summary record_kind is wrong");
        }
        if s.schema_version != ENTITLEMENT_SUMMARY_SCHEMA_VERSION {
            push("summary.schema_version", "summary schema_version is wrong");
        }
        if s.summary_id.trim().is_empty() {
            push("summary.summary_id", "summary_id must be non-empty");
        }
        for (field, value) in [
            ("summary.title", &s.title),
            ("summary.summary", &s.summary),
            ("summary.entitlement_label", &s.entitlement_label),
            ("summary.plan_label", &s.plan_label),
            ("summary.seat_owner_ref", &s.seat_owner_ref),
            ("summary.account_scope_ref", &s.account_scope_ref),
            ("summary.required_disclosure", &s.required_disclosure),
            ("summary.recovery_cue", &s.recovery_cue),
        ] {
            if value.trim().is_empty() {
                push(field, "value must be non-empty");
            }
        }

        // Local-use continuation is never implicit and never blocked.
        if s.local_only_continuation.is_empty()
            || s.local_only_continuation
                .iter()
                .any(|n| n.trim().is_empty())
        {
            push(
                "summary.local_only_continuation",
                "every summary must carry a non-empty local-only continuation note",
            );
        }

        // The effective claim is recomputed from the state's cap.
        let expected_claim = s.managed_state.claim_cap();
        if s.effective_marketed_claim != expected_claim {
            push(
                "summary.effective_marketed_claim",
                "stored effective claim does not match the managed state's cap",
            );
        }
        // The degradation kind is recomputed from the state.
        if s.degradation != DegradationKind::for_state(s.managed_state) {
            push(
                "summary.degradation",
                "stored degradation does not match the managed state",
            );
        }
        // The posture origin is recomputed from the state; a seat loss is cited
        // to the seat and an org switch to the org, never a generic sign-in.
        if s.posture_origin != expected_posture_origin(s.managed_state) {
            push(
                "summary.posture_origin",
                "stored posture origin does not match the managed state",
            );
        }
        // The entitlement state must be coherent with the managed state.
        if !entitlement_state_is_coherent(s.managed_state, s.entitlement_state) {
            push(
                "summary.entitlement_state",
                "entitlement state is not coherent with the managed state",
            );
        }
        // Seat loss and expiry degrade to an explicit managed-blocked state.
        if s.entitlement_state == EntitlementState::EntitlementExpired
            && s.degradation != DegradationKind::ManagedBlockedExplicit
        {
            push(
                "summary.degradation",
                "an expired entitlement must degrade to an explicit managed-blocked state, not a generic error",
            );
        }

        // The plan tier and role track the local-only state and never hide it
        // behind a friendly plan name.
        if s.managed_state == ManagedStateClass::LocalOnly {
            if s.plan_tier != PlanTier::LocalOnlyNoPlan {
                push(
                    "summary.plan_tier",
                    "a local-only summary must carry the local-only-no-plan tier",
                );
            }
            if s.role != AccountRole::NoManagedRole {
                push(
                    "summary.role",
                    "a local-only summary must carry no managed role",
                );
            }
            if s.quota_snapshot.is_some() {
                push(
                    "summary.quota_snapshot",
                    "a local-only summary must not carry a managed quota snapshot",
                );
            }
        } else {
            if s.plan_tier == PlanTier::LocalOnlyNoPlan {
                push(
                    "summary.plan_tier",
                    "a managed summary must name a real plan tier",
                );
            }
            if s.role == AccountRole::NoManagedRole {
                push("summary.role", "a managed summary must name a role");
            }
        }

        // A quota snapshot carries the unit, as-of time, and scope owner, and
        // never a raw number.
        if let Some(snapshot) = &s.quota_snapshot {
            if snapshot.record_kind != QUOTA_SNAPSHOT_RECORD_KIND {
                push(
                    "summary.quota_snapshot.record_kind",
                    "quota-snapshot record_kind is wrong",
                );
            }
            if snapshot.schema_version != ENTITLEMENT_SUMMARY_SCHEMA_VERSION {
                push(
                    "summary.quota_snapshot.schema_version",
                    "quota-snapshot schema_version is wrong",
                );
            }
            if snapshot.as_of.trim().is_empty() {
                push(
                    "summary.quota_snapshot.as_of",
                    "a quota snapshot must carry an as-of time",
                );
            }
            if snapshot.carries_raw_number {
                push(
                    "summary.quota_snapshot.carries_raw_number",
                    "a quota snapshot must never carry a raw spend or quota number",
                );
            }
            // A stale meter is labeled stale rather than shown live.
            if s.managed_state == ManagedStateClass::MeterStale
                && snapshot.freshness != SnapshotFreshness::FreshnessStale
            {
                push(
                    "summary.quota_snapshot.freshness",
                    "a meter-stale summary must label its snapshot stale",
                );
            }
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let summary_ids: BTreeSet<&str> = self
            .summaries
            .iter()
            .map(|s| s.summary_id.as_str())
            .collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SUMMARY_SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != ENTITLEMENT_SUMMARY_SCHEMA_VERSION {
                push(
                    "surface_binding.schema_version",
                    "binding schema_version is wrong",
                );
            }
            if binding.binding_id.trim().is_empty() {
                push("surface_binding.binding_id", "binding_id must be non-empty");
            }
            if !binding_ids.insert(binding.binding_id.as_str()) {
                push("surface_bindings", "binding_id values must be unique");
            }
            if binding.summary.trim().is_empty() {
                push(
                    "surface_binding.summary",
                    "binding summary must be non-empty",
                );
            }
            if !binding.projects_effective_claim {
                push(
                    "surface_binding.projects_effective_claim",
                    "a surface must project the effective claim, never a stronger one",
                );
            }
            if !binding.renders_local_only_continuation {
                push(
                    "surface_binding.renders_local_only_continuation",
                    "a surface must render the local-only continuation notes",
                );
            }
            if binding.bound_summary_ids.is_empty() {
                push(
                    "surface_binding.bound_summary_ids",
                    "a binding must resolve through at least one summary",
                );
            }
            for summary_ref in &binding.bound_summary_ids {
                if !summary_ids.contains(summary_ref.as_str()) {
                    push(
                        "surface_binding.bound_summary_ids",
                        "binding summary ref must resolve to a summary",
                    );
                }
            }
        }
        // Every surface must be bound.
        for surface in SummarySurface::ALL {
            if !self.surface_bindings.iter().any(|b| b.surface == surface) {
                push(
                    "surface_bindings",
                    "account, diagnostics, support/admin, Help/About, and feature entry points must all bind",
                );
                break;
            }
        }
    }
}

impl EntitlementSummaryInspection {
    fn derive(
        summaries: &[EntitlementSummary],
        surface_bindings: &[SummarySurfaceBinding],
    ) -> Self {
        let state_tokens: BTreeSet<ManagedStateClass> =
            summaries.iter().map(|s| s.managed_state).collect();
        let surfaces: BTreeSet<SummarySurface> =
            surface_bindings.iter().map(|b| b.surface).collect();

        let full_claim_count = summaries
            .iter()
            .filter(|s| s.effective_marketed_claim == MarketedClaim::ManagedFull)
            .count();
        let local_safe_only_count = summaries
            .iter()
            .filter(|s| s.effective_marketed_claim == MarketedClaim::LocalSafeOnly)
            .count();
        let managed_narrowed_count = summaries
            .iter()
            .filter(|s| s.effective_marketed_claim == MarketedClaim::ManagedNarrowed)
            .count();
        let explicit_blocked_count = summaries
            .iter()
            .filter(|s| s.degradation == DegradationKind::ManagedBlockedExplicit)
            .count();

        Self {
            record_kind: SUMMARY_INSPECTION_RECORD_KIND.to_owned(),
            schema_version: ENTITLEMENT_SUMMARY_SCHEMA_VERSION,
            summary_count: summaries.len(),
            surface_binding_count: surface_bindings.len(),
            managed_state_vocab_complete: state_tokens.len() == ManagedStateClass::ALL.len(),
            surface_coverage_complete: surfaces.len() == SummarySurface::ALL.len(),
            all_summaries_carry_local_only_continuation: summaries
                .iter()
                .all(|s| !s.local_only_continuation.is_empty()),
            // Every summary names a typed managed state and a typed posture
            // origin; the set never renders a generic account error.
            no_generic_account_error: summaries
                .iter()
                .all(|s| s.posture_origin == expected_posture_origin(s.managed_state)),
            full_claim_count,
            managed_narrowed_count,
            local_safe_only_count,
            explicit_blocked_count,
        }
    }
}

/// One failed summary-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitlementSummaryViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for EntitlementSummaryViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum EntitlementSummaryError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<EntitlementSummaryViolation>),
}

impl fmt::Display for EntitlementSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "entitlement-summary set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "entitlement-summary set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for EntitlementSummaryError {}

/// Reads and validates the checked-in stable entitlement-summary set.
///
/// This is the canonical reader: account, diagnostics, support/admin,
/// Help/About, and feature-entry surfaces call it to ingest the summaries rather
/// than cloning status text.
///
/// # Errors
///
/// Returns [`EntitlementSummaryError`] when the checked-in packet fails to parse
/// or fails validation.
pub fn current_stable_entitlement_summary_set(
) -> Result<EntitlementSummarySet, EntitlementSummaryError> {
    let set: EntitlementSummarySet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-entitlement-summary.json"
    )))
    .map_err(EntitlementSummaryError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(EntitlementSummaryError::Validation(violations))
    }
}

/// Source refs every entitlement-summary export carries.
fn entitlement_summary_source_refs() -> Vec<String> {
    let mut refs = vec![
        ENTITLEMENT_SUMMARY_SCHEMA_REF.to_owned(),
        ENTITLEMENT_SUMMARY_DOC_REF.to_owned(),
    ];
    // Reuse the control-plane refs so the summary cites the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

// One summary freezes many fixed columns; a wide builder keeps the canonical set
// readable as one table.
#[allow(clippy::too_many_arguments)]
fn summary(
    summary_id: &str,
    title: &str,
    summary_line: &str,
    managed_state: ManagedStateClass,
    entitlement_state: EntitlementState,
    entitlement_label: &str,
    plan_tier: PlanTier,
    plan_label: &str,
    role: AccountRole,
    seat_owner_scope: ScopeOwner,
    seat_owner_ref: &str,
    account_scope: ScopeOwner,
    account_scope_ref: &str,
    quota_snapshot: Option<QuotaSnapshotDescriptor>,
    local_only_continuation: &[&str],
    blocked_managed_only_actions: &[&str],
    required_disclosure: &str,
    recovery_cue: &str,
) -> EntitlementSummary {
    EntitlementSummary {
        record_kind: SUMMARY_RECORD_KIND.to_owned(),
        schema_version: ENTITLEMENT_SUMMARY_SCHEMA_VERSION,
        summary_id: summary_id.to_owned(),
        title: title.to_owned(),
        summary: summary_line.to_owned(),
        managed_state,
        entitlement_state,
        entitlement_label: entitlement_label.to_owned(),
        plan_tier,
        plan_label: plan_label.to_owned(),
        role,
        seat_owner_scope,
        seat_owner_ref: seat_owner_ref.to_owned(),
        account_scope,
        account_scope_ref: account_scope_ref.to_owned(),
        posture_origin: expected_posture_origin(managed_state),
        degradation: DegradationKind::for_state(managed_state),
        effective_marketed_claim: managed_state.claim_cap(),
        quota_snapshot,
        local_only_continuation: local_only_continuation
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        blocked_managed_only_actions: blocked_managed_only_actions
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        required_disclosure: required_disclosure.to_owned(),
        recovery_cue: recovery_cue.to_owned(),
    }
}

fn snapshot(
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    scope_owner: ScopeOwner,
    as_of: &str,
    freshness: SnapshotFreshness,
) -> QuotaSnapshotDescriptor {
    QuotaSnapshotDescriptor {
        record_kind: QUOTA_SNAPSHOT_RECORD_KIND.to_owned(),
        schema_version: ENTITLEMENT_SUMMARY_SCHEMA_VERSION,
        meter_unit,
        aggregation_window,
        scope_owner,
        as_of: as_of.to_owned(),
        freshness,
        carries_raw_number: false,
    }
}

fn binding(
    binding_id: &str,
    surface: SummarySurface,
    bound_summary_ids: &[&str],
    summary_line: &str,
) -> SummarySurfaceBinding {
    SummarySurfaceBinding {
        record_kind: SUMMARY_SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: ENTITLEMENT_SUMMARY_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        surface,
        bound_summary_ids: bound_summary_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_local_only_continuation: true,
        summary: summary_line.to_owned(),
    }
}

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "entitlement-summary:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Entitlement summaries with plan, seat owner, role, quota snapshot age, and local-only continuation";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Deterministic as-of time for the checked-in quota snapshots.
pub const STABLE_SNAPSHOT_AS_OF: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_entitlement_summary_set() -> EntitlementSummarySet {
    canonical_entitlement_summary_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen entitlement-summary set.
///
/// The set freezes one summary per managed state in the locked ten-token
/// vocabulary and one binding per surface. Every summary reuses the control-plane
/// vocabulary, recomputes its degradation, claim, and posture origin from the
/// state, and carries a non-empty local-only continuation note.
pub fn canonical_entitlement_summary_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> EntitlementSummarySet {
    let summaries = vec![
        summary(
            "entitlement_summary.signed_in",
            "Signed in",
            "A team plan is signed in and active; managed actions are admissible within seat, plan, policy, org, and provider posture.",
            ManagedStateClass::SignedIn,
            EntitlementState::EntitlementActive,
            "Team plan — active",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::OrgAdmin,
            ScopeOwner::Organization,
            "seat-owner.opaque-0001",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessLive,
            )),
            &[
                "Local editing, search, and Git continue regardless of managed state.",
                "Direct and bring-your-own-key AI routes stay available.",
            ],
            &[],
            "Render the signed-in account scope, plan tier, role, and seat owner; never imply the whole product depends on it.",
            "No action needed; managed actions are admissible.",
        ),
        summary(
            "entitlement_summary.local_only",
            "Local only",
            "No managed account is signed in; the local core is fully usable and managed actions do not apply.",
            ManagedStateClass::LocalOnly,
            EntitlementState::EntitlementNotApplicable,
            "No managed plan — local only",
            PlanTier::LocalOnlyNoPlan,
            "Local",
            AccountRole::NoManagedRole,
            ScopeOwner::Personal,
            "seat-owner.local",
            ScopeOwner::Personal,
            "org-scope.local",
            None,
            &[
                "Opening, editing, saving, searching, local Git, and local automation all continue.",
                "Sign in only to add managed lanes; nothing local depends on it.",
            ],
            &[],
            "Render the local-only posture and never show a managed quota or spend number.",
            "Sign in to a managed account to enable managed actions; local work continues now.",
        ),
        summary(
            "entitlement_summary.reauth_required",
            "Reauthentication required",
            "A managed reauthentication is required (signer rotation or attestation pending); managed actions pause until reauth completes.",
            ManagedStateClass::ReauthRequired,
            EntitlementState::EntitlementPendingRecheck,
            "Team plan — reauthentication required",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::Member,
            ScopeOwner::Organization,
            "seat-owner.opaque-0001",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessAging,
            )),
            &[
                "Local editing, search, and Git continue while reauth is pending.",
                "Cached entitlement keeps managed reads available where policy allows.",
            ],
            &["Starting new managed-broker work until reauthentication completes."],
            "Name reauthentication as the reason; never collapse it into a generic account error, a seat loss, or a managed block.",
            "Reauthenticate to resume managed actions; local work continues now.",
        ),
        summary(
            "entitlement_summary.managed_blocked",
            "Managed blocked",
            "Managed actions are blocked by an admin or policy hold; the entitlement has expired and the local core remains usable.",
            ManagedStateClass::ManagedBlocked,
            EntitlementState::EntitlementExpired,
            "Team plan — expired and blocked by policy",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::Member,
            ScopeOwner::Organization,
            "seat-owner.opaque-0001",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            )),
            &[
                "Local editing, search, Git, and existing local automation continue.",
                "Bounded usage and offboarding exports stay available.",
            ],
            &["New managed-broker, relay, and remote-workspace actions while the hold is in place."],
            "Cite the policy origin and the expired entitlement; render an explicit managed-blocked state, never a generic sign-in error.",
            "Review the account or policy hold to restore managed actions; local work continues now.",
        ),
        summary(
            "entitlement_summary.grace_period",
            "Grace period",
            "A typed grace window is open; managed actions are admissible per the window and bounded artifacts can be exported before suspension.",
            ManagedStateClass::GracePeriod,
            EntitlementState::EntitlementInGrace,
            "Team plan — in grace window",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::OrgAdmin,
            ScopeOwner::Organization,
            "seat-owner.opaque-0001",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::BytesStored,
                AggregationWindow::Rolling30d,
                ScopeOwner::Workspace,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            )),
            &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the lane clears.",
            ],
            &["Pushing new managed work once the grace window closes."],
            "Render the grace-window close time and the export-before-suspend path; never show grace as a hard block.",
            "Export bounded artifacts before the grace window closes; local work continues now.",
        ),
        summary(
            "entitlement_summary.seat_removed",
            "Seat removed",
            "This seat was removed, reclaimed, or deprovisioned; managed actions for the seat are explicitly blocked while local work continues.",
            ManagedStateClass::SeatRemoved,
            EntitlementState::EntitlementSuspendedAdmin,
            "Team plan — seat removed by admin",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::Member,
            ScopeOwner::Organization,
            "seat-owner.opaque-0002",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            )),
            &[
                "Local editing, search, Git, and existing local automation continue.",
                "Local and offboarding exports stay available for the removed seat.",
            ],
            &["All managed-broker, relay, and remote-workspace actions for this seat."],
            "Cite the seat as the origin and render an explicit managed-blocked state; never collapse a seat loss into an org switch, a sign-in error, or a generic account error.",
            "Ask an admin to restore the seat to resume managed actions; local work continues now.",
        ),
        summary(
            "entitlement_summary.plan_downgrade",
            "Plan downgrade",
            "The plan was downgraded; managed actions narrow to the plan floor for the remainder of the term.",
            ManagedStateClass::PlanDowngrade,
            EntitlementState::EntitlementActive,
            "Individual plan — narrowed to the plan floor",
            PlanTier::IndividualPro,
            "Pro",
            AccountRole::OrgOwner,
            ScopeOwner::Personal,
            "seat-owner.opaque-0003",
            ScopeOwner::Organization,
            "org-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::DownloadCount,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            )),
            &[
                "Installed extensions and local or sideloaded packages keep running.",
                "Local editing, search, and Git are unaffected by the plan floor.",
            ],
            &["Managed actions above the new plan floor for the rest of the term."],
            "Cite the plan as the origin and name the plan-floor narrowing; never show it as a full block.",
            "Managed actions are on the plan floor; upgrade the plan to widen them. Local work continues now.",
        ),
        summary(
            "entitlement_summary.org_switched",
            "Org switched",
            "The account or org was switched or transferred; managed scope is rebinding and the prior org's continuation is local-only for the transferor.",
            ManagedStateClass::OrgSwitched,
            EntitlementState::EntitlementPendingRecheck,
            "Team plan — rebinding to a new org",
            PlanTier::TeamPlan,
            "Team",
            AccountRole::OrgAdmin,
            ScopeOwner::Organization,
            "seat-owner.opaque-0001",
            ScopeOwner::Organization,
            "org-scope.opaque-0004",
            Some(snapshot(
                MeterUnit::WorkspaceHours,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessAging,
            )),
            &[
                "Local checkout, editing, tasks, and Git continue while scope rebinds.",
                "The prior org's local work stays available to the transferor.",
            ],
            &["Managed actions against the prior org until the new org scope is confirmed."],
            "Cite the org switch and what migrates versus what stays local; never collapse it into a seat loss or a generic account error.",
            "Managed scope is rebinding to the new org; local work continues now.",
        ),
        summary(
            "entitlement_summary.forecast_threshold",
            "Forecast threshold",
            "A metered family is approaching or crossed its forecast threshold; managed actions stay admissible with a budget warning.",
            ManagedStateClass::ForecastThreshold,
            EntitlementState::EntitlementActive,
            "Enterprise plan — approaching the budget threshold",
            PlanTier::EnterprisePlan,
            "Enterprise",
            AccountRole::BillingAdmin,
            ScopeOwner::Organization,
            "seat-owner.opaque-0005",
            ScopeOwner::Tenant,
            "tenant-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessLive,
            )),
            &[
                "Local editing, search, and Git continue regardless of the budget warning.",
                "Direct and bring-your-own-key AI routes stay available.",
            ],
            &["New managed-broker inference once the budget is exhausted."],
            "Render the forecast with its unit, as-of time, and scope owner; never show a forecast under an unauthoritative state.",
            "Usage is approaching the budget; raise the budget or wait for the window reset. Local work continues now.",
        ),
        summary(
            "entitlement_summary.meter_stale",
            "Meter stale",
            "The meter or rating data is stale; the managed number cannot be confirmed now and is labeled stale rather than shown live.",
            ManagedStateClass::MeterStale,
            EntitlementState::EntitlementPendingRecheck,
            "Enterprise plan — metered number stale",
            PlanTier::EnterprisePlan,
            "Enterprise",
            AccountRole::BillingAdmin,
            ScopeOwner::Organization,
            "seat-owner.opaque-0005",
            ScopeOwner::Tenant,
            "tenant-scope.opaque-0001",
            Some(snapshot(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_SNAPSHOT_AS_OF,
                SnapshotFreshness::FreshnessStale,
            )),
            &[
                "Local editing, search, and Git are never blocked by a stale metering path.",
                "Managed reads continue against the last confirmed snapshot, labeled stale.",
            ],
            &["Confirming a fresh managed number until the meter refreshes."],
            "Label the number stale with its last as-of time; never block local editing, search, or Git on a stale meter.",
            "The metered number is stale and labeled; managed actions resume when the meter refreshes. Local work continues now.",
        ),
    ];

    let all_summary_ids: Vec<&str> = summaries.iter().map(|s| s.summary_id.as_str()).collect();

    let surface_bindings = vec![
        binding(
            "summary_surface.account",
            SummarySurface::AccountSurface,
            &all_summary_ids,
            "The account and seat usage surface renders each summary's plan tier, role, seat owner, org/tenant scope, entitlement label, quota snapshot age, and local-only continuation.",
        ),
        binding(
            "summary_surface.diagnostics",
            SummarySurface::Diagnostics,
            &all_summary_ids,
            "Diagnostics and service-health surfaces project the managed state, posture origin, and snapshot freshness without inventing a stronger claim.",
        ),
        binding(
            "summary_surface.support_admin",
            SummarySurface::SupportAdminPacket,
            &all_summary_ids,
            "Support and admin export packets carry the entitlement label, posture origin, degradation kind, and the local-only continuation notes for each state.",
        ),
        binding(
            "summary_surface.help_about",
            SummarySurface::HelpAbout,
            &[
                "entitlement_summary.signed_in",
                "entitlement_summary.local_only",
                "entitlement_summary.managed_blocked",
                "entitlement_summary.seat_removed",
            ],
            "The Help/About truth surface names the current managed state, the local-only continuation, and that the local core never depends on a managed lane.",
        ),
        binding(
            "summary_surface.feature_entry",
            SummarySurface::FeatureEntryPoint,
            &[
                "entitlement_summary.reauth_required",
                "entitlement_summary.managed_blocked",
                "entitlement_summary.seat_removed",
                "entitlement_summary.grace_period",
                "entitlement_summary.plan_downgrade",
                "entitlement_summary.org_switched",
                "entitlement_summary.forecast_threshold",
                "entitlement_summary.meter_stale",
            ],
            "Managed-feature entry points show the explicit managed-blocked or narrowed state, the recovery cue, and what continues locally — never a generic sign-in error.",
        ),
    ];

    let inspection = EntitlementSummaryInspection::derive(&summaries, &surface_bindings);

    let summary_line =
        "Frozen entitlement summaries for the managed lanes. Each summary names the plan, role, \
        seat owner, org/tenant scope, entitlement label, and quota-snapshot age, carries a \
        non-empty local-only continuation note, and degrades seat loss and expiry to an explicit \
        managed-blocked state rather than a generic sign-in error."
            .to_owned();

    EntitlementSummarySet {
        record_kind: SUMMARY_SET_RECORD_KIND.to_owned(),
        schema_version: ENTITLEMENT_SUMMARY_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary: summary_line,
        source_refs: entitlement_summary_source_refs(),
        summaries,
        surface_bindings,
        inspection,
    }
}
