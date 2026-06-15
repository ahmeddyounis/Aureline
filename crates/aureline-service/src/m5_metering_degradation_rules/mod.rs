//! Stale-or-unreachable metering degradation rules for the managed lanes.
//!
//! This module is the canonical metering-degradation object. Where the sibling
//! [`crate::m5_commercial_control_plane`] freezes the per-lane fail posture and
//! [`crate::m5_usage_forecast_views`] renders the customer-visible usage number,
//! this module freezes the *runtime degradation behavior* when a metering or
//! rating path goes stale or unreachable: which local-safe promise keeps running,
//! whether the one spend-bearing optional managed action gates and why, and the
//! retry and details actions the surface offers. It reuses the closed
//! vocabularies already frozen by the control-plane matrix —
//! [`ServiceFamily`], [`MeterFamily`], [`FailPosture`], [`MarketedClaim`],
//! [`ManagedStateClass`], and [`ConsumerSurface`] — plus the
//! [`SnapshotFreshness`] freshness vocabulary, rather than minting a parallel
//! synonym set.
//!
//! The object freezes one [`MeteringDegradationRule`] per
//! (service family × [`DegradationTrigger`]) pair, so the AI gateway, settings
//! sync, the companion relay, the registry/mirror surface, support ingest, and
//! the managed workspace each carry a rule for a stale meter, an unreachable
//! metering service, and an unavailable rating path. The matrix is exhaustive:
//! every family covers every trigger exactly once.
//!
//! Three invariants keep the rules honest. First, **the local core is never
//! blocked**: every rule carries a non-empty
//! [`MeteringDegradationRule::local_safe_promise`] and sets
//! [`MeteringDegradationRule::narrows_to_local_safe_only`] to false, so a stale
//! or unreachable metering path narrows only the relevant managed action and
//! never local editing, search, Git, or already-authorized local automation.
//! Second, **fail posture matches the frozen matrix**: each rule's
//! [`MeteringDegradationRule::disposition`] is recomputed from its lane's
//! [`FailPosture`] via [`DegradationDisposition::for_posture`], and
//! [`MeteringDegradationRuleSet::cross_check_against_control_plane`] confirms the
//! posture against the canonical control-plane lane, so a fail-open lane keeps
//! its local-safe path and a fail-closed lane gates exactly one optional action.
//! Third, **a degradation is not an account error**: every rule lists the four
//! distinct account-loss states it must never collapse into — a removed seat, an
//! org switch, a grace window, and a sign-in/reauth failure — in
//! [`MeteringDegradationRule::distinct_from_account_states`], and a gated rule
//! names exactly one optional action rather than failing closed generically.
//!
//! Any number a rule references is bound to its unit, as-of time, and scope owner
//! or suppressed entirely — never shown bare — via
//! [`DegradationValueDisclosure`]. A degradation narrows the marketed claim to
//! [`MarketedClaim::ManagedNarrowed`] (never to [`MarketedClaim::LocalSafeOnly`]
//! and never staying [`MarketedClaim::ManagedFull`]), so a marketed managed claim
//! narrows automatically when its metering evidence goes stale or unreachable.
//!
//! [`canonical_metering_degradation_rule_set`] builds the frozen set and
//! [`current_stable_metering_degradation_rule_set`] reads and validates the
//! checked-in packet at
//! [`artifacts/service/m5-metering-degradation-rules.json`](../../../../artifacts/service/m5-metering-degradation-rules.json),
//! so service-health diagnostics, the account/usage surface, Help/About, the
//! support/admin export, and claim/public-truth automation all ingest one packet
//! rather than cloning status text. The boundary schema is
//! [`schemas/service/m5-metering-degradation-rules.schema.json`](../../../../schemas/service/m5-metering-degradation-rules.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, canonical_stable_commercial_control_plane_matrix, ConsumerSurface,
    FailPosture, ManagedStateClass, MarketedClaim, MeterFamily, ServiceFamily,
};
use crate::m5_entitlement_summary::SnapshotFreshness;

#[cfg(test)]
mod tests;

/// Supported schema version for the metering-degradation rule set.
pub const METERING_DEGRADATION_RULES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the rule-set packet.
pub const RULE_SET_RECORD_KIND: &str = "m5_metering_degradation_rule_set";

/// Stable record-kind tag for a single degradation rule.
pub const RULE_RECORD_KIND: &str = "m5_metering_degradation_rule";

/// Stable record-kind tag for a degradation action.
pub const ACTION_RECORD_KIND: &str = "m5_metering_degradation_action";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_metering_degradation_surface_binding";

/// Stable record-kind tag for the rule-set inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_metering_degradation_inspection";

/// Repo-relative path to the boundary schema.
pub const METERING_DEGRADATION_RULES_SCHEMA_REF: &str =
    "schemas/service/m5-metering-degradation-rules.schema.json";

/// Repo-relative path to the reviewer contract.
pub const METERING_DEGRADATION_RULES_DOC_REF: &str = "docs/m5/add-stale-or-unreachable-metering-degradation-rules-fail-open-local-core-behavior-and-fail-closed-optional-managed-action-gates-across-m5-commercial-surfaces.md";

/// Repo-relative path to the checked-in rule-set packet.
pub const METERING_DEGRADATION_RULES_ARTIFACT_PATH: &str =
    "artifacts/service/m5-metering-degradation-rules.json";

/// The metering or rating condition that degrades a managed lane.
///
/// A degradation trigger is a metering posture, not an account error: a stale
/// meter or an unreachable metering service is distinct from a seat loss, an org
/// switch, a grace window, or a sign-in failure, which stay in the
/// [`ManagedStateClass`] vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationTrigger {
    /// The meter or rating data is stale; the managed number cannot be confirmed now.
    MeteringStale,
    /// The metering service is unreachable; the managed number cannot be fetched now.
    ServiceUnreachable,
    /// The rating path is unavailable; the cost of the next managed action cannot be computed now.
    RatingPathUnavailable,
}

impl DegradationTrigger {
    /// Every degradation trigger, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::MeteringStale,
        Self::ServiceUnreachable,
        Self::RatingPathUnavailable,
    ];

    /// The stable token used to build a rule id.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::MeteringStale => "metering_stale",
            Self::ServiceUnreachable => "service_unreachable",
            Self::RatingPathUnavailable => "rating_path_unavailable",
        }
    }

    /// The freshness class a measurement carries under this trigger.
    pub const fn freshness(self) -> SnapshotFreshness {
        match self {
            Self::MeteringStale => SnapshotFreshness::FreshnessStale,
            Self::ServiceUnreachable | Self::RatingPathUnavailable => {
                SnapshotFreshness::FreshnessUnknown
            }
        }
    }

    /// How a number is disclosed under this trigger; it is never shown bare.
    pub const fn value_disclosure(self) -> DegradationValueDisclosure {
        match self {
            // A stale meter has a last-known number, shown labeled and bound.
            Self::MeteringStale => DegradationValueDisclosure::LabeledStaleBoundToUnitAsOfScope,
            // An unreachable meter or rating path has no current number to show.
            Self::ServiceUnreachable | Self::RatingPathUnavailable => {
                DegradationValueDisclosure::SuppressedNoManagedNumber
            }
        }
    }

    /// The control-plane managed state this trigger maps to, when one exists.
    ///
    /// Only the stale trigger maps to the frozen [`ManagedStateClass::MeterStale`]
    /// token; an unreachable service or rating path is a distinct metering posture
    /// with no account-state token, so it never borrows one.
    pub const fn related_managed_state(self) -> Option<ManagedStateClass> {
        match self {
            Self::MeteringStale => Some(ManagedStateClass::MeterStale),
            Self::ServiceUnreachable | Self::RatingPathUnavailable => None,
        }
    }
}

/// How a managed lane degrades when its metering or rating path fails.
///
/// The disposition is recomputed from the lane's [`FailPosture`] via
/// [`DegradationDisposition::for_posture`], so a fail-open lane keeps its
/// local-safe path and a fail-closed lane gates exactly one optional action. In
/// every disposition the local core continues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationDisposition {
    /// Fail open: the lane falls back to its local-safe path; no managed action is gated.
    FailOpenLocalSafePath,
    /// Fail open: the managed number is labeled and shown bounded; the optional action continues.
    FailOpenManagedLabeled,
    /// Fail closed: only the one spend-bearing optional managed action gates; the local core continues.
    FailClosedOptionalActionGated,
    /// Fail closed: the one spend-bearing optional action waits for a boundary recheck.
    FailClosedPendingBoundaryRecheck,
}

impl DegradationDisposition {
    /// The disposition a lane fail posture always resolves to.
    pub const fn for_posture(posture: FailPosture) -> Self {
        match posture {
            FailPosture::FailOpenLocalSafe => Self::FailOpenLocalSafePath,
            FailPosture::FailOpenLocalSafeWithLabel => Self::FailOpenManagedLabeled,
            FailPosture::FailClosedManagedOnly => Self::FailClosedOptionalActionGated,
            FailPosture::BoundaryRecheckRequired => Self::FailClosedPendingBoundaryRecheck,
        }
    }

    /// True when the lane fails open and the local-safe path absorbs the failure.
    pub const fn is_fail_open(self) -> bool {
        matches!(
            self,
            Self::FailOpenLocalSafePath | Self::FailOpenManagedLabeled
        )
    }

    /// True when exactly one optional managed action gates under this disposition.
    pub const fn gates_optional_action(self) -> bool {
        matches!(
            self,
            Self::FailClosedOptionalActionGated | Self::FailClosedPendingBoundaryRecheck
        )
    }
}

/// How a number a rule references is disclosed across the boundary.
///
/// A managed value is always bound to its unit, as-of time, and scope owner, or
/// suppressed entirely; it is never shown bare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationValueDisclosure {
    /// The last-known value is shown labeled stale and bound to its unit, as-of time, and scope owner.
    LabeledStaleBoundToUnitAsOfScope,
    /// No managed number is shown; the metering or rating path could not be reached.
    SuppressedNoManagedNumber,
}

impl DegradationValueDisclosure {
    /// True when a number is shown (labeled and bound), false when it is suppressed.
    pub const fn shows_number(self) -> bool {
        matches!(self, Self::LabeledStaleBoundToUnitAsOfScope)
    }
}

/// The kind of action a degradation rule offers the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationActionKind {
    /// Retry the metering or rating path.
    Retry,
    /// Open the details surface for the affected service.
    Details,
}

/// One user-visible action a degradation rule offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradationAction {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The action kind.
    pub kind: DegradationActionKind,
    /// The reviewable label the surface renders.
    pub label: String,
}

impl DegradationAction {
    /// Builds a retry action with the given label.
    pub fn retry(label: &str) -> Self {
        Self {
            record_kind: ACTION_RECORD_KIND.to_owned(),
            schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
            kind: DegradationActionKind::Retry,
            label: label.to_owned(),
        }
    }

    /// Builds a details action with the given label.
    pub fn details(label: &str) -> Self {
        Self {
            record_kind: ACTION_RECORD_KIND.to_owned(),
            schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
            kind: DegradationActionKind::Details,
            label: label.to_owned(),
        }
    }
}

/// One frozen metering-degradation rule for a (service family, trigger) pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeteringDegradationRule {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable rule identifier.
    pub rule_id: String,
    /// Reviewable rule title.
    pub title: String,
    /// Reviewable rule summary.
    pub summary: String,
    /// The control-plane lane this rule projects.
    pub lane_ref: String,
    /// The affected service family.
    pub service_family: ServiceFamily,
    /// The meter family the lane is measured by.
    pub meter_family: MeterFamily,
    /// The metering or rating condition that triggers this rule.
    pub degradation_trigger: DegradationTrigger,
    /// The lane's fail posture, projected from the control-plane matrix.
    pub fail_posture: FailPosture,
    /// The disposition, recomputed from the fail posture.
    pub disposition: DegradationDisposition,
    /// Non-empty local-safe promise that always continues when the lane degrades.
    pub local_safe_promise: Vec<String>,
    /// Always false: a metering degradation never collapses to the local-safe-only claim.
    pub narrows_to_local_safe_only: bool,
    /// The one optional managed action that gates. Present (non-null) only when the disposition fails closed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gated_optional_action: Option<String>,
    /// Why the optional action is gated under this trigger. Present iff [`Self::gated_optional_action`] is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_reason: Option<String>,
    /// How any number is disclosed; bound to unit, as-of time, and scope owner, or suppressed.
    pub value_disclosure: DegradationValueDisclosure,
    /// Measurement freshness under this trigger.
    pub freshness: SnapshotFreshness,
    /// Last-contact as-of time for the meter; present even when the number is suppressed.
    pub as_of: String,
    /// The retry action that re-checks the metering or rating path.
    pub retry_action: DegradationAction,
    /// The details action that opens the affected service's surface.
    pub details_action: DegradationAction,
    /// Always true: a metering degradation is a metering posture, not an account error.
    pub not_an_account_error: bool,
    /// The distinct account-loss states this degradation must never collapse into.
    pub distinct_from_account_states: Vec<ManagedStateClass>,
    /// The related managed state, when the trigger maps to one (only the stale trigger does).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_managed_state: Option<ManagedStateClass>,
    /// The marketed claim the lane declares absent degradation.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the degradation narrows it.
    pub effective_marketed_claim: MarketedClaim,
    /// The next user-visible step; the local core always continues.
    pub recovery_cue: String,
}

impl MeteringDegradationRule {
    /// True when the rule fails open and the local-safe path absorbs the failure.
    pub fn is_fail_open(&self) -> bool {
        self.disposition.is_fail_open()
    }

    /// True when the rule gates exactly one optional managed action.
    pub fn gates_optional_action(&self) -> bool {
        self.disposition.gates_optional_action()
    }
}

/// One surface bound to the degradation rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradationSurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The consumer surface that projects the rules.
    pub consumer_surface: ConsumerSurface,
    /// The rule ids this surface resolves through.
    pub bound_rule_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the local-safe promise.
    pub renders_local_safe_promise: bool,
    /// Always true: the surface names the blocking reason when an action gates.
    pub names_blocking_reason: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the rule set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeteringDegradationInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of degradation rules.
    pub rule_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// Number of distinct service families covered.
    pub service_families_covered: usize,
    /// Number of distinct degradation triggers covered.
    pub degradation_triggers_covered: usize,
    /// True when there is exactly one rule per (family, trigger) pair across the full matrix.
    pub matrix_complete: bool,
    /// True when every rule keeps a non-empty local-safe promise.
    pub all_rules_local_safe_backed: bool,
    /// True when no rule collapses the local core to the local-safe-only claim.
    pub never_blocks_local_core: bool,
    /// True when no rule shows a bare number: every value is labeled-and-bound or suppressed, with an as-of time.
    pub value_never_bare: bool,
    /// True when every rule lists all four distinct account-loss states it must not collapse into.
    pub account_state_distinctions_complete: bool,
    /// Number of rules that fail open to the local-safe path.
    pub fail_open_rule_count: usize,
    /// Number of rules that fail closed and gate exactly one optional action.
    pub fail_closed_rule_count: usize,
}

/// The frozen metering-degradation rule-set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeteringDegradationRuleSet {
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
    /// The degradation rules.
    pub rules: Vec<MeteringDegradationRule>,
    /// The surface bindings.
    pub surface_bindings: Vec<DegradationSurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: MeteringDegradationInspection,
}

impl MeteringDegradationRuleSet {
    /// Returns the rule covering `family` under `trigger`, when one is frozen.
    pub fn rule_for(
        &self,
        family: ServiceFamily,
        trigger: DegradationTrigger,
    ) -> Option<&MeteringDegradationRule> {
        self.rules
            .iter()
            .find(|r| r.service_family == family && r.degradation_trigger == trigger)
    }

    /// Returns every rule covering `family`, one per trigger.
    pub fn rules_for_family(&self, family: ServiceFamily) -> Vec<&MeteringDegradationRule> {
        self.rules
            .iter()
            .filter(|r| r.service_family == family)
            .collect()
    }

    /// Serializes the set as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the set cannot be serialized, which a validated set never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("metering-degradation rule set serializes to JSON")
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`MeteringDegradationRuleViolation`] per failed invariant: a
    /// wrong record kind or schema version, a missing identifier, a duplicate
    /// rule, an incomplete service-family or trigger set, a missing (family,
    /// trigger) pair, a disposition that drifts from the fail posture, an empty
    /// local-safe promise, a rule that collapses the local core, a gated action
    /// without a blocking reason (or a fail-open rule that gates), a bare number,
    /// a missing as-of time, a missing account-state distinction, a wrong action
    /// kind, an effective claim that does not narrow correctly, an unbound
    /// surface, or a stale inspection block.
    pub fn validate(&self) -> Vec<MeteringDegradationRuleViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(MeteringDegradationRuleViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != RULE_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != METERING_DEGRADATION_RULES_SCHEMA_VERSION {
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
            .any(|entry| entry == METERING_DEGRADATION_RULES_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.rules.is_empty() {
            push("rules", "set must contain at least one rule");
        }

        let mut rule_ids = BTreeSet::new();
        let mut seen_pairs = BTreeSet::new();
        for rule in &self.rules {
            self.validate_rule(rule, &mut push);
            if !rule_ids.insert(rule.rule_id.as_str()) {
                push("rules", "rule_id values must be unique");
            }
            // Exactly one rule per (family, trigger) pair: the matrix never doubles a cell.
            if !seen_pairs.insert((rule.service_family, rule.degradation_trigger)) {
                push(
                    "rules",
                    "each (service family, trigger) pair must carry at most one rule",
                );
            }
        }

        // The matrix is exhaustive: every family covers every trigger.
        for family in ServiceFamily::ALL {
            for trigger in DegradationTrigger::ALL {
                if !self
                    .rules
                    .iter()
                    .any(|r| r.service_family == family && r.degradation_trigger == trigger)
                {
                    push(
                        "rules",
                        "every service family must carry a rule for every degradation trigger",
                    );
                }
            }
        }

        self.validate_surface_bindings(&mut push);

        let derived = MeteringDegradationInspection::derive(&self.rules, &self.surface_bindings);
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed set",
            );
        }

        violations
    }

    fn validate_rule(&self, rule: &MeteringDegradationRule, push: &mut impl FnMut(&str, &str)) {
        if rule.record_kind != RULE_RECORD_KIND {
            push("rule.record_kind", "rule record_kind is wrong");
        }
        if rule.schema_version != METERING_DEGRADATION_RULES_SCHEMA_VERSION {
            push("rule.schema_version", "rule schema_version is wrong");
        }
        for (field, value) in [
            ("rule.rule_id", &rule.rule_id),
            ("rule.title", &rule.title),
            ("rule.summary", &rule.summary),
            ("rule.lane_ref", &rule.lane_ref),
            ("rule.recovery_cue", &rule.recovery_cue),
            ("rule.as_of", &rule.as_of),
        ] {
            if value.trim().is_empty() {
                push(field, "value must be non-empty");
            }
        }

        // The local core is never blocked: every rule keeps a non-empty promise
        // and never collapses to the local-safe-only claim.
        if rule.local_safe_promise.is_empty()
            || rule.local_safe_promise.iter().any(|s| s.trim().is_empty())
        {
            push(
                "rule.local_safe_promise",
                "every rule must keep a non-empty local-safe promise",
            );
        }
        if rule.narrows_to_local_safe_only {
            push(
                "rule.narrows_to_local_safe_only",
                "a metering degradation must never collapse the local core to the local-safe-only claim",
            );
        }

        // Fail posture matches the frozen matrix: the disposition is recomputed
        // from the lane's fail posture.
        let expected_disposition = DegradationDisposition::for_posture(rule.fail_posture);
        if rule.disposition != expected_disposition {
            push(
                "rule.disposition",
                "stored disposition does not match the lane fail posture",
            );
        }

        // A gated rule names exactly one optional action and its blocking reason;
        // a fail-open rule never gates.
        match (
            rule.disposition.gates_optional_action(),
            &rule.gated_optional_action,
        ) {
            (true, None) => push(
                "rule.gated_optional_action",
                "a fail-closed rule must name the one optional action it gates",
            ),
            (true, Some(action)) if action.trim().is_empty() => push(
                "rule.gated_optional_action",
                "the gated optional action must be non-empty",
            ),
            (false, Some(_)) => push(
                "rule.gated_optional_action",
                "a fail-open rule must not gate an optional action",
            ),
            _ => {}
        }
        match (rule.gated_optional_action.is_some(), &rule.blocking_reason) {
            (true, None) => push(
                "rule.blocking_reason",
                "a gated rule must name the blocking reason for the optional action",
            ),
            (true, Some(reason)) if reason.trim().is_empty() => push(
                "rule.blocking_reason",
                "the blocking reason must be non-empty",
            ),
            (false, Some(_)) => push(
                "rule.blocking_reason",
                "a rule with no gated action must not carry a blocking reason",
            ),
            _ => {}
        }

        // No number crosses the boundary bare: the disclosure and freshness are
        // recomputed from the trigger.
        if rule.value_disclosure != rule.degradation_trigger.value_disclosure() {
            push(
                "rule.value_disclosure",
                "stored value disclosure does not match the trigger",
            );
        }
        if rule.freshness != rule.degradation_trigger.freshness() {
            push(
                "rule.freshness",
                "stored freshness does not match the trigger",
            );
        }
        // A labeled-stale number must carry a stale freshness; a suppressed number shows nothing.
        if rule.value_disclosure == DegradationValueDisclosure::LabeledStaleBoundToUnitAsOfScope
            && rule.freshness != SnapshotFreshness::FreshnessStale
        {
            push(
                "rule.freshness",
                "a labeled-stale number must carry a stale freshness",
            );
        }

        // A degradation is a metering posture, not an account error.
        if !rule.not_an_account_error {
            push(
                "rule.not_an_account_error",
                "a metering degradation must declare it is not an account error",
            );
        }
        for state in ACCOUNT_ERROR_STATES {
            if !rule.distinct_from_account_states.contains(&state) {
                push(
                    "rule.distinct_from_account_states",
                    "a degradation must stay distinct from seat loss, org switch, grace, and sign-in failure",
                );
            }
        }
        if rule.related_managed_state != rule.degradation_trigger.related_managed_state() {
            push(
                "rule.related_managed_state",
                "stored related managed state does not match the trigger",
            );
        }

        // The retry and details actions carry their kind and a non-empty label.
        if rule.retry_action.kind != DegradationActionKind::Retry
            || rule.retry_action.label.trim().is_empty()
            || rule.retry_action.record_kind != ACTION_RECORD_KIND
        {
            push(
                "rule.retry_action",
                "rule must carry a labeled retry action",
            );
        }
        if rule.details_action.kind != DegradationActionKind::Details
            || rule.details_action.label.trim().is_empty()
            || rule.details_action.record_kind != ACTION_RECORD_KIND
        {
            push(
                "rule.details_action",
                "rule must carry a labeled details action",
            );
        }

        // A degradation narrows the marketed claim to managed-narrowed: never
        // staying full and never collapsing to local-safe-only.
        if rule.declared_marketed_claim != MarketedClaim::ManagedFull {
            push(
                "rule.declared_marketed_claim",
                "a lane declares the full managed claim absent degradation",
            );
        }
        if rule.effective_marketed_claim != MarketedClaim::ManagedNarrowed {
            push(
                "rule.effective_marketed_claim",
                "a degradation narrows the marketed claim to managed-narrowed, never full or local-safe-only",
            );
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let rule_ids: BTreeSet<&str> = self.rules.iter().map(|r| r.rule_id.as_str()).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != METERING_DEGRADATION_RULES_SCHEMA_VERSION {
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
            if !binding.renders_local_safe_promise {
                push(
                    "surface_binding.renders_local_safe_promise",
                    "a surface must render the local-safe promise",
                );
            }
            if !binding.names_blocking_reason {
                push(
                    "surface_binding.names_blocking_reason",
                    "a surface must name the blocking reason when an action gates",
                );
            }
            if binding.bound_rule_ids.is_empty() {
                push(
                    "surface_binding.bound_rule_ids",
                    "a binding must resolve through at least one rule",
                );
            }
            for rule_ref in &binding.bound_rule_ids {
                if !rule_ids.contains(rule_ref.as_str()) {
                    push(
                        "surface_binding.bound_rule_ids",
                        "binding rule ref must resolve to a rule",
                    );
                }
            }
        }
        // Every consumer surface must be bound.
        for surface in ConsumerSurface::ALL {
            if !self
                .surface_bindings
                .iter()
                .any(|b| b.consumer_surface == surface)
            {
                push(
                    "surface_bindings",
                    "account, diagnostics, Help/About, support/admin, and claim automation must all bind",
                );
                break;
            }
        }
    }

    /// Cross-checks every rule against its control-plane lane.
    ///
    /// Confirms each [`MeteringDegradationRule`] projects the canonical
    /// commercial-control-plane matrix lane named by its
    /// [`MeteringDegradationRule::lane_ref`] — the service family, meter family,
    /// and fail posture must match — so a fail-open or fail-closed disposition is
    /// the matrix's posture rather than a parallel spreadsheet. Returns an empty
    /// vector when every rule matches its lane.
    pub fn cross_check_against_control_plane(&self) -> Vec<MeteringDegradationRuleViolation> {
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let mut violations = Vec::new();
        for rule in &self.rules {
            let Some(lane) = matrix.lanes.iter().find(|l| l.lane_id == rule.lane_ref) else {
                violations.push(MeteringDegradationRuleViolation {
                    field: "rule.lane_ref".to_owned(),
                    message: format!(
                        "lane_ref {} does not resolve to a control-plane lane",
                        rule.lane_ref
                    ),
                });
                continue;
            };
            let mut mismatch = |field: &str| {
                violations.push(MeteringDegradationRuleViolation {
                    field: field.to_owned(),
                    message: format!(
                        "rule {} drifted from control-plane lane {}",
                        rule.rule_id, lane.lane_id
                    ),
                });
            };
            if rule.service_family != lane.service_family {
                mismatch("rule.service_family");
            }
            if rule.meter_family != lane.meter_family {
                mismatch("rule.meter_family");
            }
            if rule.fail_posture != lane.fail_posture {
                mismatch("rule.fail_posture");
            }
            // The disposition is the matrix's posture, recomputed.
            if rule.disposition != DegradationDisposition::for_posture(lane.fail_posture) {
                mismatch("rule.disposition");
            }
        }
        violations
    }
}

impl MeteringDegradationInspection {
    fn derive(
        rules: &[MeteringDegradationRule],
        surface_bindings: &[DegradationSurfaceBinding],
    ) -> Self {
        let service_families: BTreeSet<ServiceFamily> =
            rules.iter().map(|r| r.service_family).collect();
        let triggers: BTreeSet<DegradationTrigger> =
            rules.iter().map(|r| r.degradation_trigger).collect();
        let pairs: BTreeSet<(ServiceFamily, DegradationTrigger)> = rules
            .iter()
            .map(|r| (r.service_family, r.degradation_trigger))
            .collect();

        let fail_open_rule_count = rules.iter().filter(|r| r.is_fail_open()).count();
        let fail_closed_rule_count = rules.iter().filter(|r| r.gates_optional_action()).count();

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
            rule_count: rules.len(),
            surface_binding_count: surface_bindings.len(),
            service_families_covered: service_families.len(),
            degradation_triggers_covered: triggers.len(),
            // Exhaustive matrix: one rule per family-trigger pair, all pairs present.
            matrix_complete: pairs.len() == rules.len()
                && pairs.len() == ServiceFamily::ALL.len() * DegradationTrigger::ALL.len(),
            all_rules_local_safe_backed: rules.iter().all(|r| !r.local_safe_promise.is_empty()),
            never_blocks_local_core: rules
                .iter()
                .all(|r| !r.narrows_to_local_safe_only && !r.local_safe_promise.is_empty()),
            value_never_bare: rules
                .iter()
                .all(|r| !r.as_of.trim().is_empty() && r.value_disclosure_is_safe()),
            account_state_distinctions_complete: rules.iter().all(|r| {
                ACCOUNT_ERROR_STATES
                    .iter()
                    .all(|s| r.distinct_from_account_states.contains(s))
            }),
            fail_open_rule_count,
            fail_closed_rule_count,
        }
    }
}

impl MeteringDegradationRule {
    /// True when the value disclosure never shows a bare number.
    fn value_disclosure_is_safe(&self) -> bool {
        match self.value_disclosure {
            // A labeled-stale number is bound to unit/as-of/scope and never bare.
            DegradationValueDisclosure::LabeledStaleBoundToUnitAsOfScope => {
                !self.as_of.trim().is_empty()
            }
            // A suppressed number is not shown at all.
            DegradationValueDisclosure::SuppressedNoManagedNumber => true,
        }
    }
}

/// One failed rule-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeteringDegradationRuleViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for MeteringDegradationRuleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum MeteringDegradationRuleError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<MeteringDegradationRuleViolation>),
}

impl fmt::Display for MeteringDegradationRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "metering-degradation rule set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "metering-degradation rule set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for MeteringDegradationRuleError {}

/// The four distinct account-loss states a degradation must never collapse into.
const ACCOUNT_ERROR_STATES: [ManagedStateClass; 4] = [
    ManagedStateClass::SeatRemoved,
    ManagedStateClass::OrgSwitched,
    ManagedStateClass::GracePeriod,
    ManagedStateClass::ReauthRequired,
];

/// Reads and validates the checked-in stable metering-degradation rule set.
///
/// This is the canonical reader: service-health diagnostics, the account/usage
/// surface, Help/About, the support/admin export, and claim/public-truth
/// automation call it to ingest the rules rather than cloning status text.
///
/// # Errors
///
/// Returns [`MeteringDegradationRuleError`] when the checked-in packet fails to
/// parse or fails validation.
pub fn current_stable_metering_degradation_rule_set(
) -> Result<MeteringDegradationRuleSet, MeteringDegradationRuleError> {
    let set: MeteringDegradationRuleSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-metering-degradation-rules.json"
    )))
    .map_err(MeteringDegradationRuleError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(MeteringDegradationRuleError::Validation(violations))
    }
}

/// Source refs every metering-degradation export carries.
fn degradation_source_refs() -> Vec<String> {
    let mut refs = vec![
        METERING_DEGRADATION_RULES_SCHEMA_REF.to_owned(),
        METERING_DEGRADATION_RULES_DOC_REF.to_owned(),
    ];
    // Reuse the control-plane refs so the rules cite the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

/// Deterministic last-contact as-of time for the checked-in rules.
pub const STABLE_LAST_CONTACT_AS_OF: &str = "2026-06-15T00:00:00Z";

/// The fixed, lane-level data a degradation rule projects from the control plane.
struct LaneDef {
    service_family: ServiceFamily,
    meter_family: MeterFamily,
    lane_ref: &'static str,
    family_slug: &'static str,
    lane_name: &'static str,
    fail_posture: FailPosture,
    /// The one spend-bearing optional managed action this lane offers.
    optional_action: &'static str,
    local_safe_promise: &'static [&'static str],
}

/// The clause a trigger contributes to a rule summary.
fn trigger_clause(trigger: DegradationTrigger) -> &'static str {
    match trigger {
        DegradationTrigger::MeteringStale => {
            "the meter or rating data is stale and the managed number cannot be confirmed now"
        }
        DegradationTrigger::ServiceUnreachable => {
            "the metering service is unreachable and the managed number cannot be fetched now"
        }
        DegradationTrigger::RatingPathUnavailable => {
            "the rating path is unavailable and the cost of the next managed action cannot be computed now"
        }
    }
}

/// The short phrase a trigger contributes to a rule title.
fn trigger_phrase(trigger: DegradationTrigger) -> &'static str {
    match trigger {
        DegradationTrigger::MeteringStale => "meter stale",
        DegradationTrigger::ServiceUnreachable => "metering service unreachable",
        DegradationTrigger::RatingPathUnavailable => "rating path unavailable",
    }
}

/// The clause a disposition contributes to a rule summary.
fn disposition_clause(disposition: DegradationDisposition) -> &'static str {
    match disposition {
        DegradationDisposition::FailOpenLocalSafePath => {
            "the lane falls back to its local-safe path and no managed action is gated"
        }
        DegradationDisposition::FailOpenManagedLabeled => {
            "the managed number is labeled and shown bounded while the optional action continues"
        }
        DegradationDisposition::FailClosedOptionalActionGated => {
            "only the one spend-bearing optional action is gated until the meter is bounded again"
        }
        DegradationDisposition::FailClosedPendingBoundaryRecheck => {
            "the one spend-bearing optional action waits for a boundary recheck"
        }
    }
}

/// The retry-action label for a trigger.
fn retry_label(trigger: DegradationTrigger) -> &'static str {
    match trigger {
        DegradationTrigger::MeteringStale => "Re-check the meter now.",
        DegradationTrigger::ServiceUnreachable => "Reconnect to the metering service.",
        DegradationTrigger::RatingPathUnavailable => "Retry the rating path.",
    }
}

/// The blocking-reason text for a gated rule under a trigger.
fn blocking_reason_for(trigger: DegradationTrigger) -> String {
    match trigger {
        DegradationTrigger::MeteringStale => {
            "Spend cannot be bounded because the meter is stale, so this one action waits."
        }
        DegradationTrigger::ServiceUnreachable => {
            "Spend cannot be bounded because the metering service is unreachable, so this one action waits."
        }
        DegradationTrigger::RatingPathUnavailable => {
            "Cost cannot be computed because the rating path is unavailable, so this one action waits."
        }
    }
    .to_owned()
}

/// The recovery cue for a trigger; the local core always continues.
fn recovery_cue_for(trigger: DegradationTrigger) -> String {
    match trigger {
        DegradationTrigger::MeteringStale => {
            "The number refreshes when the meter reconnects; local editing, search, Git, and existing local automation continue now."
        }
        DegradationTrigger::ServiceUnreachable => {
            "The managed action resumes when the metering service reconnects; local editing, search, Git, and existing local automation continue now."
        }
        DegradationTrigger::RatingPathUnavailable => {
            "The managed action resumes when the rating path returns; local editing, search, Git, and existing local automation continue now."
        }
    }
    .to_owned()
}

fn build_rule(lane: &LaneDef, trigger: DegradationTrigger) -> MeteringDegradationRule {
    let disposition = DegradationDisposition::for_posture(lane.fail_posture);
    let gated_optional_action = if disposition.gates_optional_action() {
        Some(lane.optional_action.to_owned())
    } else {
        None
    };
    let blocking_reason = gated_optional_action
        .as_ref()
        .map(|_| blocking_reason_for(trigger));

    let summary = format!(
        "When {}, {} keeps its local-safe baseline running and {}.",
        trigger_clause(trigger),
        lane.lane_name,
        disposition_clause(disposition),
    );

    MeteringDegradationRule {
        record_kind: RULE_RECORD_KIND.to_owned(),
        schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
        rule_id: format!("degradation.{}.{}", lane.family_slug, trigger.slug()),
        title: format!("{} — {}", lane.lane_name, trigger_phrase(trigger)),
        summary,
        lane_ref: lane.lane_ref.to_owned(),
        service_family: lane.service_family,
        meter_family: lane.meter_family,
        degradation_trigger: trigger,
        fail_posture: lane.fail_posture,
        disposition,
        local_safe_promise: lane
            .local_safe_promise
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrows_to_local_safe_only: false,
        gated_optional_action,
        blocking_reason,
        value_disclosure: trigger.value_disclosure(),
        freshness: trigger.freshness(),
        as_of: STABLE_LAST_CONTACT_AS_OF.to_owned(),
        retry_action: DegradationAction::retry(retry_label(trigger)),
        details_action: DegradationAction::details(
            "Open service health to see the affected service and its last contact time.",
        ),
        not_an_account_error: true,
        distinct_from_account_states: ACCOUNT_ERROR_STATES.to_vec(),
        related_managed_state: trigger.related_managed_state(),
        declared_marketed_claim: MarketedClaim::ManagedFull,
        effective_marketed_claim: MarketedClaim::ManagedNarrowed,
        recovery_cue: recovery_cue_for(trigger),
    }
}

fn binding(
    binding_id: &str,
    consumer_surface: ConsumerSurface,
    bound_rule_ids: &[&str],
    summary: &str,
) -> DegradationSurfaceBinding {
    DegradationSurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        consumer_surface,
        bound_rule_ids: bound_rule_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_local_safe_promise: true,
        names_blocking_reason: true,
        summary: summary.to_owned(),
    }
}

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "metering-degradation-rules:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Stale-or-unreachable metering degradation rules with fail-open local-core behavior and fail-closed optional managed-action gates";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_metering_degradation_rule_set() -> MeteringDegradationRuleSet {
    canonical_metering_degradation_rule_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen metering-degradation rule set.
///
/// The set freezes one rule per (service family × [`DegradationTrigger`]) pair
/// across the AI gateway, settings sync, the companion relay, the registry/mirror
/// surface, support ingest, and the managed workspace, plus one binding per
/// consumer surface. Each rule projects its control-plane lane's fail posture,
/// keeps a non-empty local-safe promise, gates exactly one optional action when
/// the lane fails closed, discloses any number bound or suppressed, and narrows
/// the marketed claim to managed-narrowed.
pub fn canonical_metering_degradation_rule_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> MeteringDegradationRuleSet {
    let lanes = [
        LaneDef {
            service_family: ServiceFamily::AiGatewayFamily,
            meter_family: MeterFamily::AiGatewayMeterFamily,
            lane_ref: "managed_lane.ai_gateway",
            family_slug: "ai_gateway",
            lane_name: "Managed AI gateway",
            fail_posture: FailPosture::FailOpenLocalSafeWithLabel,
            optional_action: "New managed-broker inference while token spend cannot be bounded.",
            local_safe_promise: &[
                "Direct and bring-your-own-key AI routes keep running.",
                "Local editing, search, and Git are unaffected.",
            ],
        },
        LaneDef {
            service_family: ServiceFamily::SyncFamily,
            meter_family: MeterFamily::ProfileOrSettingsSyncMeterFamily,
            lane_ref: "managed_lane.settings_sync",
            family_slug: "settings_sync",
            lane_name: "Managed settings sync",
            fail_posture: FailPosture::FailOpenLocalSafe,
            optional_action: "Pushing new settings snapshots to the managed store while storage cannot be bounded.",
            local_safe_promise: &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the meter clears.",
            ],
        },
        LaneDef {
            service_family: ServiceFamily::CollaborationRelayFamily,
            meter_family: MeterFamily::CollaborationRelayMeterFamily,
            lane_ref: "managed_lane.companion_relay",
            family_slug: "companion_relay",
            lane_name: "Companion relay",
            fail_posture: FailPosture::FailClosedManagedOnly,
            optional_action: "Joining a new live companion-follow or relay session while relay minutes cannot be bounded.",
            local_safe_promise: &[
                "Local incident notes and offline packets continue.",
                "Desktop handoff resumes the exact local context.",
            ],
        },
        LaneDef {
            service_family: ServiceFamily::RegistryOrMirrorMetadataFamily,
            meter_family: MeterFamily::RegistryOrMirrorMeterFamily,
            lane_ref: "managed_lane.registry_mirror",
            family_slug: "registry_mirror",
            lane_name: "Registry and mirror",
            fail_posture: FailPosture::FailOpenLocalSafe,
            optional_action: "New managed-registry installs or publishes while the download meter cannot be bounded.",
            local_safe_promise: &[
                "Installed extensions keep running.",
                "Local and sideloaded packages are unaffected.",
            ],
        },
        LaneDef {
            service_family: ServiceFamily::TelemetryOrSupportIngestFamily,
            meter_family: MeterFamily::SupportIngestMeterFamily,
            lane_ref: "managed_lane.support_ingest",
            family_slug: "support_ingest",
            lane_name: "Support ingest",
            fail_posture: FailPosture::FailOpenLocalSafeWithLabel,
            optional_action: "Uploading new support bundles to the managed sink while the ingest meter cannot be bounded.",
            local_safe_promise: &[
                "Local support bundles still generate on device.",
                "Offline evidence capture continues.",
            ],
        },
        LaneDef {
            service_family: ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            meter_family: MeterFamily::RemoteWorkspaceControlPlaneMeterFamily,
            lane_ref: "managed_lane.managed_workspace",
            family_slug: "managed_workspace",
            lane_name: "Managed workspace",
            fail_posture: FailPosture::FailClosedManagedOnly,
            optional_action: "Attaching or running a new remote workspace while workspace-hour spend cannot be bounded.",
            local_safe_promise: &[
                "Local checkout and editing continue.",
                "Local tasks and Git are unaffected when the remote workspace narrows.",
            ],
        },
    ];

    let mut rules = Vec::with_capacity(lanes.len() * DegradationTrigger::ALL.len());
    for lane in &lanes {
        for trigger in DegradationTrigger::ALL {
            rules.push(build_rule(lane, trigger));
        }
    }

    let all_rule_ids: Vec<&str> = rules.iter().map(|r| r.rule_id.as_str()).collect();
    let gated_rule_ids: Vec<&str> = rules
        .iter()
        .filter(|r| r.gates_optional_action())
        .map(|r| r.rule_id.as_str())
        .collect();

    let surface_bindings = vec![
        binding(
            "degradation_surface.diagnostics",
            ConsumerSurface::Diagnostics,
            &all_rule_ids,
            "Service-health and diagnostics surfaces render each rule's degradation trigger, fail posture, local-safe promise, gated action with its blocking reason, and the retry and details actions.",
        ),
        binding(
            "degradation_surface.account",
            ConsumerSurface::AccountSurface,
            &all_rule_ids,
            "The account and usage surface labels a stale or unreachable meter, names the local-safe promise, and never shows a spend or quota number without its unit, as-of time, and scope owner.",
        ),
        binding(
            "degradation_surface.help_about",
            ConsumerSurface::HelpAbout,
            &[
                "degradation.ai_gateway.metering_stale",
                "degradation.settings_sync.metering_stale",
                "degradation.registry_mirror.metering_stale",
            ],
            "The Help/About truth surface states that a stale or unreachable metering path narrows only the relevant managed action and never local editing, search, Git, or existing local automation.",
        ),
        binding(
            "degradation_surface.support_admin",
            ConsumerSurface::SupportAdminPacket,
            &gated_rule_ids,
            "Support and admin export packets carry each fail-closed rule's gated action, its blocking reason, the fail posture, and the last-contact as-of time.",
        ),
        binding(
            "degradation_surface.claim_public_truth",
            ConsumerSurface::ClaimPublicTruthAutomation,
            &all_rule_ids,
            "Claim and public-truth automation narrows a marketed managed claim to managed-narrowed when its metering evidence goes stale or unreachable, never collapsing it to local-safe-only.",
        ),
    ];

    let inspection = MeteringDegradationInspection::derive(&rules, &surface_bindings);

    let summary = "Frozen stale-or-unreachable metering degradation rules for the managed lanes. \
        Each rule projects its control-plane lane's fail posture so the local core fails open and \
        keeps running, gates exactly one optional managed action when the lane fails closed and \
        spend cannot be bounded, names the affected service family, the local-safe promise, the \
        blocking reason, and the retry and details actions, and narrows the marketed claim to \
        managed-narrowed without ever collapsing the local core to local-safe-only."
        .to_owned();

    MeteringDegradationRuleSet {
        record_kind: RULE_SET_RECORD_KIND.to_owned(),
        schema_version: METERING_DEGRADATION_RULES_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary,
        source_refs: degradation_source_refs(),
        rules,
        surface_bindings,
        inspection,
    }
}
