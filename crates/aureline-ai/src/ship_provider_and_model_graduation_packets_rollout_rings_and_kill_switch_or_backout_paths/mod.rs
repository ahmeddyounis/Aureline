//! Provider and model graduation packets, rollout rings, and kill-switch or
//! backout paths.
//!
//! This module materializes the provider/model graduation surface into one
//! export-safe truth packet whose unit of truth is a
//! [`ProviderModelGraduationRow`]: a single provider/model route binding its
//! claimed qualification, the rollout ring it is currently exposed through, the
//! progress state of that ring, the provider-neutral kill switch that can halt
//! it, and the backout path that reverses it. The packet is the canonical
//! provider/model graduation source for shell, docs, support export, and release
//! tooling; consumers project it instead of re-deriving rollout or kill-switch
//! state by hand.
//!
//! The packet refuses to present a route as graduating further than its safety
//! posture can back. Every route — claimed or not — must carry a kill switch
//! that fails closed, so a route can never be exposed behind a switch that fails
//! open. A claimed route must keep its kill switch armed (or already fired), so a
//! shipped route is never left without a halt path. A claimed route exposed
//! through a broad-exposure ring (broad or general availability) must carry a
//! verified, non-trivial backout path, so a widely-exposed route always has a
//! proven way back. A route may only reach the general-availability ring while it
//! claims the Stable qualification, so general availability never outruns the
//! claim. A route whose ring was kill-switched or backed out may not keep
//! claiming Stable; the halt narrows the claim instead of hiding it. Every
//! claimed route carries a closed set of downgrade rules that narrow the claim
//! instead of hiding the route, reusing the qualification and downgrade-trigger
//! vocabularies frozen by the M5 AI workflow matrix lane so no route row may stay
//! greener than its evidence.
//!
//! Raw provider endpoints, credential bodies, raw provider payloads, exact spend
//! values, and internal kill-switch tokens stay outside the support boundary; the
//! packet carries scopes, classes, postures, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json`](../../../../schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths.md`](../../../../docs/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/`](../../../../fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_MODEL_REGISTRY_SCHEMA_REF;

/// Stable record-kind tag carried by [`ProviderModelGraduationPacket`].
pub const PROVIDER_MODEL_GRADUATION_RECORD_KIND: &str =
    "ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths";

/// Schema version for provider/model graduation records.
pub const PROVIDER_MODEL_GRADUATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROVIDER_MODEL_GRADUATION_SCHEMA_REF: &str =
    "schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json";

/// Repo-relative path of the provider/model graduation contract doc.
pub const PROVIDER_MODEL_GRADUATION_DOC_REF: &str =
    "docs/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths.md";

/// Repo-relative path of the protected fixture directory.
pub const PROVIDER_MODEL_GRADUATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths";

/// Repo-relative path of the checked support-export artifact.
pub const PROVIDER_MODEL_GRADUATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PROVIDER_MODEL_GRADUATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths.md";

/// Rollout ring a route is currently exposed through.
///
/// Rings are ordered from least to most exposure; [`Self::rank`] makes that
/// ordering explicit for consumers and release tooling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRingClass {
    /// Internal-only exposure (the team itself).
    Internal,
    /// Canary exposure to a small opt-in cohort.
    Canary,
    /// Early-access exposure to a broader opt-in cohort.
    EarlyAccess,
    /// Broad exposure to most users on the channel.
    Broad,
    /// General availability — the default for everyone on the channel.
    GeneralAvailability,
}

impl RolloutRingClass {
    /// Every rollout ring, in declaration (least-to-most-exposure) order.
    pub const ALL: [Self; 5] = [
        Self::Internal,
        Self::Canary,
        Self::EarlyAccess,
        Self::Broad,
        Self::GeneralAvailability,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Canary => "canary",
            Self::EarlyAccess => "early_access",
            Self::Broad => "broad",
            Self::GeneralAvailability => "general_availability",
        }
    }

    /// Exposure rank; higher means more users see the route.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Internal => 0,
            Self::Canary => 1,
            Self::EarlyAccess => 2,
            Self::Broad => 3,
            Self::GeneralAvailability => 4,
        }
    }

    /// Whether this ring exposes the route to a broad audience.
    ///
    /// Broad-exposure rings demand a verified backout path on any claimed route.
    pub const fn is_broad_exposure(self) -> bool {
        matches!(self, Self::Broad | Self::GeneralAvailability)
    }
}

/// Progress state of a route within its current rollout ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RingProgressState {
    /// The ring is scheduled but has not begun.
    Pending,
    /// The route is rolling out through the ring.
    Rolling,
    /// The ring is paused pending a decision.
    Held,
    /// The ring completed and the route can advance.
    Complete,
    /// The route's rollout was halted by its kill switch.
    KillSwitched,
    /// The route was rolled back along its backout path.
    BackedOut,
}

impl RingProgressState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Rolling => "rolling",
            Self::Held => "held",
            Self::Complete => "complete",
            Self::KillSwitched => "kill_switched",
            Self::BackedOut => "backed_out",
        }
    }

    /// Whether the rollout was halted (kill-switched or backed out).
    pub const fn is_halted(self) -> bool {
        matches!(self, Self::KillSwitched | Self::BackedOut)
    }

    /// Whether the route is actively advancing or has completed a ring.
    pub const fn is_advancing(self) -> bool {
        matches!(self, Self::Rolling | Self::Complete)
    }
}

/// Scope a kill switch reaches when it fires.
///
/// The kill switch is provider-neutral: the same scopes apply regardless of
/// provider, model, or external-tool route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchScopeClass {
    /// Halts this single route only.
    RouteScoped,
    /// Halts every route bound to this model.
    ModelScoped,
    /// Halts every route bound to this provider.
    ProviderScoped,
    /// Halts every provider/model route at once.
    GlobalAllRoutes,
}

impl KillSwitchScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteScoped => "route_scoped",
            Self::ModelScoped => "model_scoped",
            Self::ProviderScoped => "provider_scoped",
            Self::GlobalAllRoutes => "global_all_routes",
        }
    }
}

/// Armed state of a route's kill switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchState {
    /// No kill switch is armed for this route.
    NotArmed,
    /// The kill switch is armed and ready to fire.
    Armed,
    /// The kill switch has fired and the route is halted.
    Fired,
}

impl KillSwitchState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotArmed => "not_armed",
            Self::Armed => "armed",
            Self::Fired => "fired",
        }
    }

    /// Whether the switch is armed and ready or already fired.
    pub const fn is_armed_or_fired(self) -> bool {
        matches!(self, Self::Armed | Self::Fired)
    }
}

/// Provider-neutral kill switch governing one route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraduationKillSwitch {
    /// Scope the switch reaches when it fires.
    pub scope: KillSwitchScopeClass,
    /// Armed state of the switch.
    pub state: KillSwitchState,
    /// True when the switch fails closed (denies dispatch) on any ambiguity.
    pub fails_closed: bool,
    /// Review-safe label shown alongside the switch (no raw token).
    pub label: String,
}

/// Backout path that reverses a route's rollout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraduationBackoutPath {
    /// Rollback posture the backout path provides.
    pub posture: M5AiWorkflowRollbackPosture,
    /// True when the backout path has been drilled and verified.
    pub verified: bool,
    /// Review-safe label describing what the backout restores.
    pub label: String,
}

impl GraduationBackoutPath {
    /// Whether the backout path actually reverses the rollout.
    ///
    /// A `not_applicable` posture provides no reversal, so a broad-exposure route
    /// may not rely on it.
    pub const fn is_reversing(&self) -> bool {
        !matches!(self.posture, M5AiWorkflowRollbackPosture::NotApplicable)
    }
}

/// One downgrade rule that narrows a route's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelGraduationDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the route narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One provider/model graduation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelGraduationRow {
    /// Stable route id.
    pub route_id: String,
    /// Provider identity token (no raw endpoint URL).
    pub provider_id: String,
    /// Model identity token.
    pub model_id: String,
    /// Human-readable route label.
    pub route_label: String,
    /// Qualification class claimed for this route.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Rollout ring the route is currently exposed through.
    pub current_ring: RolloutRingClass,
    /// Progress state of the current ring.
    pub ring_state: RingProgressState,
    /// Provider-neutral kill switch governing the route.
    pub kill_switch: GraduationKillSwitch,
    /// Backout path that reverses the rollout.
    pub backout: GraduationBackoutPath,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<ProviderModelGraduationDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl ProviderModelGraduationRow {
    /// Whether this route carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Qualification this route narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this route.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Route `{}`\n", self.route_id));
        out.push_str(&format!("- Provider: `{}`\n", self.provider_id));
        out.push_str(&format!("- Model: `{}`\n", self.model_id));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Ring: `{}`\n", self.current_ring.as_str()));
        out.push_str(&format!("- Ring state: `{}`\n", self.ring_state.as_str()));
        out.push_str(&format!(
            "- Kill switch: `{}` / `{}` (fails closed: {}) ({})\n",
            self.kill_switch.scope.as_str(),
            self.kill_switch.state.as_str(),
            self.kill_switch.fails_closed,
            self.kill_switch.label
        ));
        out.push_str(&format!(
            "- Backout: `{}` (verified: {}) ({})\n",
            self.backout.posture.as_str(),
            self.backout.verified,
            self.backout.label
        ));
        out
    }
}

/// Proof freshness block for the graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelGraduationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed routes.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ProviderModelGraduationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderModelGraduationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Provider/model graduation rows.
    pub routes: Vec<ProviderModelGraduationRow>,
    /// Proof freshness block.
    pub proof_freshness: ProviderModelGraduationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe provider/model graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelGraduationPacket {
    /// Record kind; must equal [`PROVIDER_MODEL_GRADUATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROVIDER_MODEL_GRADUATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Provider/model graduation rows.
    pub routes: Vec<ProviderModelGraduationRow>,
    /// Proof freshness block.
    pub proof_freshness: ProviderModelGraduationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ProviderModelGraduationPacket {
    /// Builds a provider/model graduation packet from stable-lane input.
    pub fn new(input: ProviderModelGraduationPacketInput) -> Self {
        Self {
            record_kind: PROVIDER_MODEL_GRADUATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_MODEL_GRADUATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            routes: input.routes,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the provider/model graduation invariants.
    pub fn validate(&self) -> Vec<ProviderModelGraduationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PROVIDER_MODEL_GRADUATION_RECORD_KIND {
            violations.push(ProviderModelGraduationViolation::WrongRecordKind);
        }
        if self.schema_version != PROVIDER_MODEL_GRADUATION_SCHEMA_VERSION {
            violations.push(ProviderModelGraduationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ProviderModelGraduationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_routes_present(self, &mut violations);
        for route in &self.routes {
            validate_route(route, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("provider/model graduation packet serializes"),
        ) {
            violations.push(ProviderModelGraduationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of routes carrying a publicly claimed qualification.
    pub fn claimed_route_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.is_claimed())
            .count()
    }

    /// Count of routes currently exposed through a broad-exposure ring.
    pub fn broad_exposure_route_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.current_ring.is_broad_exposure())
            .count()
    }

    /// Count of routes that have reached the general-availability ring.
    pub fn ga_route_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.current_ring == RolloutRingClass::GeneralAvailability)
            .count()
    }

    /// Count of routes whose rollout was halted by a kill switch.
    pub fn kill_switched_route_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.ring_state == RingProgressState::KillSwitched)
            .count()
    }

    /// Count of routes that were rolled back along their backout path.
    pub fn backed_out_route_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.ring_state == RingProgressState::BackedOut)
            .count()
    }

    /// Count of routes whose kill switch is armed or already fired.
    pub fn armed_kill_switch_count(&self) -> usize {
        self.routes
            .iter()
            .filter(|route| route.kill_switch.state.is_armed_or_fired())
            .count()
    }

    /// Returns the route row for `route_id`, if present.
    pub fn route(&self, route_id: &str) -> Option<&ProviderModelGraduationRow> {
        self.routes.iter().find(|route| route.route_id == route_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("provider/model graduation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Provider And Model Graduation, Rollout Rings, And Kill Switch\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Routes: {} ({} claimed, {} broad-exposure, {} GA, {} kill-switched, {} backed-out)\n",
            self.routes.len(),
            self.claimed_route_count(),
            self.broad_exposure_route_count(),
            self.ga_route_count(),
            self.kill_switched_route_count(),
            self.backed_out_route_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Route inspectors\n\n");
        for route in &self.routes {
            out.push_str(&route.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in provider/model graduation export.
#[derive(Debug)]
pub enum ProviderModelGraduationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProviderModelGraduationViolation>),
}

impl fmt::Display for ProviderModelGraduationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "provider/model graduation export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "provider/model graduation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProviderModelGraduationArtifactError {}

/// Validation failures emitted by [`ProviderModelGraduationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderModelGraduationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no routes.
    NoRoutes,
    /// A route id appears more than once.
    DuplicateRoute,
    /// A route row is missing a required identity or label field.
    RouteRowIncomplete,
    /// A route's kill switch does not fail closed.
    KillSwitchNotFailClosed,
    /// A claimed route's kill switch is not armed (or already fired).
    ClaimedRouteKillSwitchNotArmed,
    /// A claimed broad-exposure route has no reversing backout path.
    BroadExposureWithoutBackoutPath,
    /// A claimed broad-exposure route's backout path is not verified.
    BroadExposureBackoutUnverified,
    /// A route is at general availability without claiming Stable.
    GaRingWithoutStableClaim,
    /// A kill-switched or backed-out route still claims Stable.
    HaltedRouteClaimsStable,
    /// A claimed route is missing required evidence packet refs.
    ClaimedRouteMissingEvidence,
    /// A route has no downgrade rules.
    DowngradeRulesMissing,
    /// A route's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ProviderModelGraduationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoRoutes => "no_routes",
            Self::DuplicateRoute => "duplicate_route",
            Self::RouteRowIncomplete => "route_row_incomplete",
            Self::KillSwitchNotFailClosed => "kill_switch_not_fail_closed",
            Self::ClaimedRouteKillSwitchNotArmed => "claimed_route_kill_switch_not_armed",
            Self::BroadExposureWithoutBackoutPath => "broad_exposure_without_backout_path",
            Self::BroadExposureBackoutUnverified => "broad_exposure_backout_unverified",
            Self::GaRingWithoutStableClaim => "ga_ring_without_stable_claim",
            Self::HaltedRouteClaimsStable => "halted_route_claims_stable",
            Self::ClaimedRouteMissingEvidence => "claimed_route_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in provider/model graduation export.
pub fn current_provider_model_graduation_export(
) -> Result<ProviderModelGraduationPacket, ProviderModelGraduationArtifactError> {
    let packet: ProviderModelGraduationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/support_export.json"
    )))
    .map_err(ProviderModelGraduationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderModelGraduationArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn validate_source_contracts(
    packet: &ProviderModelGraduationPacket,
    violations: &mut Vec<ProviderModelGraduationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PROVIDER_MODEL_GRADUATION_SCHEMA_REF,
        PROVIDER_MODEL_GRADUATION_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ProviderModelGraduationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_routes_present(
    packet: &ProviderModelGraduationPacket,
    violations: &mut Vec<ProviderModelGraduationViolation>,
) {
    if packet.routes.is_empty() {
        violations.push(ProviderModelGraduationViolation::NoRoutes);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for route in &packet.routes {
        if !seen.insert(route.route_id.as_str()) {
            violations.push(ProviderModelGraduationViolation::DuplicateRoute);
        }
    }
}

fn validate_route(
    route: &ProviderModelGraduationRow,
    violations: &mut Vec<ProviderModelGraduationViolation>,
) {
    if route.route_id.trim().is_empty()
        || route.provider_id.trim().is_empty()
        || route.model_id.trim().is_empty()
        || route.route_label.trim().is_empty()
        || route.kill_switch.label.trim().is_empty()
        || route.backout.label.trim().is_empty()
    {
        violations.push(ProviderModelGraduationViolation::RouteRowIncomplete);
    }

    // The kill switch must fail closed on every route, claimed or not.
    if !route.kill_switch.fails_closed {
        violations.push(ProviderModelGraduationViolation::KillSwitchNotFailClosed);
    }

    if route.is_claimed() && !route.kill_switch.state.is_armed_or_fired() {
        violations.push(ProviderModelGraduationViolation::ClaimedRouteKillSwitchNotArmed);
    }

    if route.is_claimed() && route.current_ring.is_broad_exposure() {
        if !route.backout.is_reversing() {
            violations.push(ProviderModelGraduationViolation::BroadExposureWithoutBackoutPath);
        } else if !route.backout.verified {
            violations.push(ProviderModelGraduationViolation::BroadExposureBackoutUnverified);
        }
    }

    if route.current_ring == RolloutRingClass::GeneralAvailability
        && route.claimed_qualification != M5AiWorkflowQualificationClass::Stable
    {
        violations.push(ProviderModelGraduationViolation::GaRingWithoutStableClaim);
    }

    if route.ring_state.is_halted()
        && route.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(ProviderModelGraduationViolation::HaltedRouteClaimsStable);
    }

    if route.is_claimed() && route.evidence_packet_refs.is_empty() {
        violations.push(ProviderModelGraduationViolation::ClaimedRouteMissingEvidence);
    }

    validate_downgrade_rules(route, violations);
}

fn validate_downgrade_rules(
    route: &ProviderModelGraduationRow,
    violations: &mut Vec<ProviderModelGraduationViolation>,
) {
    if route.downgrade_rules.is_empty() {
        violations.push(ProviderModelGraduationViolation::DowngradeRulesMissing);
        return;
    }

    if !route
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(ProviderModelGraduationViolation::DowngradeRuleMissingProofStale);
    }

    let claimed_rank = qualification_rank(route.claimed_qualification);
    for rule in &route.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(ProviderModelGraduationViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &ProviderModelGraduationPacket,
    violations: &mut Vec<ProviderModelGraduationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ProviderModelGraduationViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("https://")
                || lower.contains("http://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
