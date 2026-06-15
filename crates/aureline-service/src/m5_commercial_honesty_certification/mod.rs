//! Cross-lane certification of entitlement, metering, forecast, chargeback, and
//! downgrade honesty across the claimed managed deployment profiles.
//!
//! This module is the canonical certification object for the managed-service
//! economics boundary. Where the sibling lanes each render one slice of the
//! truth — [`crate::m5_entitlement_summary`] the account context,
//! [`crate::m5_usage_forecast_views`] the usage and forecast, the
//! [`crate::m5_metering_degradation_rules`] the fail-open/fail-closed behavior,
//! [`crate::m5_chargeback_scope_views`] the cost ownership, the
//! [`crate::m5_offboarding_cards`] the wind-down continuity, and the
//! [`crate::m5_commercial_boundary_cards`] the open-versus-paid boundary — this
//! packet **certifies** that each honesty dimension actually holds before any M5
//! surface promotes the managed claim, and **narrows** the marketed claim the
//! moment a certification drill fails or its evidence goes stale.
//!
//! The packet freezes one [`HonestyCertificationRow`] per
//! [`HonestyDimension`] — entitlement, metering, forecast, chargeback,
//! downgrade/offboarding, and commercial-boundary honesty. Each row names the
//! sibling consumer that backs it, the [`ServiceFamily`] managed lanes it covers,
//! the [`DeploymentProfile`]s it is certified in and the profiles it is honestly
//! not offered in, and the [`CertificationDrillResult`] set that exercises it:
//! the stale-meter drill, the fail-open-local-core and fail-closed-managed-action
//! drills, the seat-loss, org-switch, and grace-period drills, the export-rights
//! validation, the chargeback-scope export check, and the residual-dependency
//! disclosure review.
//!
//! Two invariants keep the certification honest. First, **a failed drill narrows
//! the claim**: each drill carries a [`DrillGrade`] and a
//! [`BoundaryEvidenceStatus`], the weaker of whose caps recomputes the drill's
//! marketed-claim cap, and a row's [`HonestyCertificationRow::effective_certified_claim`]
//! is the weakest of its declared claim and every drill cap, so a narrowed drill
//! or stale evidence drops the row's claim from `managed_full` to
//! `managed_narrowed` (or `local_safe_only`) automatically rather than inheriting
//! broader managed marketing language; the stored value must equal that
//! recomputation or [`HonestyCertificationPacket::validate`] reports a violation.
//! Second, **the local core is never blocked and the offline profiles are never
//! skipped**: every row keeps a non-empty [`HonestyCertificationRow::local_safe_baseline`],
//! every row partitions all five deployment profiles between certified and
//! not-offered so the self-host, air-gapped, and mirror profiles are always
//! addressed, and the certification is never drawn from one vendor-managed online
//! profile alone.
//!
//! [`canonical_stable_honesty_certification_packet`] builds the frozen packet and
//! [`current_stable_honesty_certification_packet`] reads and validates the
//! checked-in artifact at
//! [`artifacts/service/m5-commercial-honesty-certification.json`](../../../../artifacts/service/m5-commercial-honesty-certification.json),
//! so the release center, Help/About, diagnostics, service health, support/admin
//! packets, and claim/public-truth automation all ingest one certification packet
//! rather than cloning verdict text.
//! [`HonestyCertificationPacket::cross_check_backing_consumers`] loads each
//! sibling packet and confirms the certification rides real, validating consumers
//! instead of a parallel scorecard, and
//! [`HonestyCertificationPacket::narrow_for_drill_failure`] narrows a row for a
//! single failed drill so release tooling can exercise the narrowing
//! deterministically.
//!
//! The boundary schema is
//! [`schemas/service/m5-commercial-honesty-certification.schema.json`](../../../../schemas/service/m5-commercial-honesty-certification.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_boundary_cards::{
    canonical_stable_commercial_boundary_card_set, BoundaryEvidenceStatus, DeploymentProfile,
};
use crate::m5_commercial_control_plane::{
    canonical_stable_commercial_control_plane_matrix, MarketedClaim, ServiceFamily,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the certification packet.
pub const HONESTY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the certification packet.
pub const PACKET_RECORD_KIND: &str = "m5_commercial_honesty_certification";

/// Stable record-kind tag for a certification row.
pub const ROW_RECORD_KIND: &str = "m5_commercial_honesty_certification_row";

/// Stable record-kind tag for a drill result.
pub const DRILL_RESULT_RECORD_KIND: &str = "m5_commercial_certification_drill_result";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_commercial_certification_surface_binding";

/// Stable record-kind tag for the inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_commercial_honesty_certification_inspection";

/// Repo-relative path to the boundary schema.
pub const HONESTY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/service/m5-commercial-honesty-certification.schema.json";

/// Repo-relative path to the reviewer contract.
pub const HONESTY_CERTIFICATION_DOC_REF: &str =
    "docs/m5/certify-entitlement-metering-forecast-chargeback-and-downgrade-honesty-across-claimed-m5-managed-deployment-profiles.md";

/// Repo-relative path to the checked-in certification artifact.
pub const HONESTY_CERTIFICATION_ARTIFACT_PATH: &str =
    "artifacts/service/m5-commercial-honesty-certification.json";

/// The honesty dimension a certification row covers.
///
/// These six tokens are the closed set of honesty claims the M5 commercial
/// control plane makes: that the entitlement context, the metering behavior, the
/// forecast messaging, the chargeback ownership, the downgrade/offboarding
/// continuity, and the open-versus-paid boundary each tell the truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HonestyDimension {
    /// Plan, seat, role, scope, and quota-snapshot-age honesty.
    EntitlementHonesty,
    /// Meter unit, as-of time, and fail-open/fail-closed honesty.
    MeteringHonesty,
    /// Forecast threshold and what-changes-next honesty.
    ForecastHonesty,
    /// Scope-ownership and direct-versus-inherited chargeback honesty.
    ChargebackHonesty,
    /// Seat-loss, org-switch, grace-period, and offboarding-continuity honesty.
    DowngradeOffboardingHonesty,
    /// Open-versus-paid boundary and residual-dependency-disclosure honesty.
    CommercialBoundaryHonesty,
}

impl HonestyDimension {
    /// Every honesty dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EntitlementHonesty,
        Self::MeteringHonesty,
        Self::ForecastHonesty,
        Self::ChargebackHonesty,
        Self::DowngradeOffboardingHonesty,
        Self::CommercialBoundaryHonesty,
    ];

    /// The sibling consumer that primarily backs this dimension.
    pub const fn backing_consumer(self) -> BackingConsumer {
        match self {
            Self::EntitlementHonesty => BackingConsumer::EntitlementSummary,
            Self::MeteringHonesty => BackingConsumer::MeteringDegradationRules,
            Self::ForecastHonesty => BackingConsumer::UsageForecastViews,
            Self::ChargebackHonesty => BackingConsumer::ChargebackScopeViews,
            Self::DowngradeOffboardingHonesty => BackingConsumer::OffboardingCards,
            Self::CommercialBoundaryHonesty => BackingConsumer::CommercialBoundaryCards,
        }
    }
}

/// The closed certification-drill vocabulary.
///
/// Each drill exercises one honesty behavior the acceptance contract requires:
/// the metered numbers stay labeled when stale, the local core fails open, only a
/// named managed action fails closed, seat loss and an org switch stay distinct,
/// a grace window preserves export, exports keep their parity, the chargeback
/// scope export holds, and the residual-dependency disclosure is current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDrill {
    /// A stale meter keeps the number labeled and never silently green.
    StaleMeterDrill,
    /// The local core fails open and keeps editing, search, and Git running.
    FailOpenLocalCore,
    /// Exactly one named managed action fails closed with its blocking reason.
    FailClosedManagedAction,
    /// Seat loss is shown distinctly, never collapsed into a generic account error.
    SeatLossDrill,
    /// An org switch is shown distinctly while managed scope rebinds.
    OrgSwitchDrill,
    /// A grace window is shown distinctly and preserves export-before-suspend.
    GracePeriodDrill,
    /// The bounded export keeps its documented CSV/JSON parity.
    ExportRightsValidation,
    /// The chargeback-scope export keeps scopes distinct at export parity.
    ChargebackScopeExportCheck,
    /// The residual vendor-hosted dependency disclosure is current and complete.
    ResidualDependencyDisclosureReview,
}

impl CertificationDrill {
    /// Every certification drill, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StaleMeterDrill,
        Self::FailOpenLocalCore,
        Self::FailClosedManagedAction,
        Self::SeatLossDrill,
        Self::OrgSwitchDrill,
        Self::GracePeriodDrill,
        Self::ExportRightsValidation,
        Self::ChargebackScopeExportCheck,
        Self::ResidualDependencyDisclosureReview,
    ];
}

/// The grade a certification drill earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillGrade {
    /// The drilled honesty behavior was proven; the managed claim stands.
    Certified,
    /// The behavior was unproven or failed; the managed claim narrows.
    Narrowed,
    /// The drill does not apply to this row's dimension.
    NotApplicable,
}

impl DrillGrade {
    /// The marketed-claim cap this grade imposes on its drill.
    ///
    /// A certified or not-applicable grade imposes no cap; a narrowed grade caps
    /// the drill to the reduced managed claim, which then narrows the row.
    pub const fn claim_cap(self) -> MarketedClaim {
        match self {
            Self::Certified | Self::NotApplicable => MarketedClaim::ManagedFull,
            Self::Narrowed => MarketedClaim::ManagedNarrowed,
        }
    }
}

/// The sibling consumer packet that backs a row or a drill.
///
/// Every certification row rides a real, validating sibling packet rather than a
/// parallel scorecard; [`HonestyCertificationPacket::cross_check_backing_consumers`]
/// loads each one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackingConsumer {
    /// The commercial-control-plane matrix anchor.
    CommercialControlPlane,
    /// The entitlement-summary set.
    EntitlementSummary,
    /// The usage-and-forecast view set.
    UsageForecastViews,
    /// The chargeback-scope view set.
    ChargebackScopeViews,
    /// The metering-degradation rule set.
    MeteringDegradationRules,
    /// The offboarding-card set.
    OffboardingCards,
    /// The commercial-boundary card set.
    CommercialBoundaryCards,
}

impl BackingConsumer {
    /// Repo-relative path to the consumer's checked-in artifact.
    pub const fn artifact_path(self) -> &'static str {
        match self {
            Self::CommercialControlPlane => "artifacts/service/m5-commercial-control-plane.json",
            Self::EntitlementSummary => "artifacts/service/m5-entitlement-summary.json",
            Self::UsageForecastViews => "artifacts/service/m5-usage-forecast-views.json",
            Self::ChargebackScopeViews => "artifacts/service/m5-chargeback-scope-views.json",
            Self::MeteringDegradationRules => {
                "artifacts/service/m5-metering-degradation-rules.json"
            }
            Self::OffboardingCards => "artifacts/service/m5-offboarding-cards.json",
            Self::CommercialBoundaryCards => "artifacts/service/m5-commercial-boundary-cards.json",
        }
    }
}

/// The surface that consumes the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurface {
    /// The release center, which narrows a train row from the certification verdict.
    ReleaseCenter,
    /// The Help/About open-versus-paid truth surface.
    HelpAbout,
    /// Diagnostics.
    Diagnostics,
    /// Service health.
    ServiceHealth,
    /// Support and admin export packets.
    SupportAdminPacket,
    /// Claim and public-truth narrowing automation.
    ClaimPublicTruthAutomation,
}

impl CertificationSurface {
    /// Every consumer surface the certification packet must reach.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::Diagnostics,
        Self::ServiceHealth,
        Self::SupportAdminPacket,
        Self::ClaimPublicTruthAutomation,
    ];

    /// True when this surface narrows a marketed claim from the verdict.
    pub const fn narrows_on_failure(self) -> bool {
        matches!(self, Self::ReleaseCenter | Self::ClaimPublicTruthAutomation)
    }
}

/// One drill result inside a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrillResult {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The drill this result records.
    pub drill: CertificationDrill,
    /// The grade the drill earned.
    pub grade: DrillGrade,
    /// The freshness of the evidence backing the drill.
    pub evidence_status: BoundaryEvidenceStatus,
    /// The marketed-claim cap, recomputed from the weaker of the grade and evidence caps.
    pub claim_cap: MarketedClaim,
    /// The sibling consumer the drill rode.
    pub backing_consumer: BackingConsumer,
    /// Reviewable detail of what the drill proved.
    pub detail: String,
}

impl CertificationDrillResult {
    /// The marketed-claim cap a grade/evidence pair imposes.
    fn derive_cap(grade: DrillGrade, evidence: BoundaryEvidenceStatus) -> MarketedClaim {
        weaker_claim(grade.claim_cap(), evidence.claim_cap())
    }
}

/// One frozen certification row, one per honesty dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HonestyCertificationRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable row identifier.
    pub row_id: String,
    /// The honesty dimension this row certifies.
    pub dimension: HonestyDimension,
    /// Reviewable row title.
    pub title: String,
    /// Reviewable row summary.
    pub summary: String,
    /// The sibling consumer that backs the row.
    pub backing_consumer: BackingConsumer,
    /// The managed service families the row's honesty applies across.
    pub service_families: Vec<ServiceFamily>,
    /// The deployment profiles the row is certified in.
    pub certified_profiles: Vec<DeploymentProfile>,
    /// The deployment profiles the row's managed lane is honestly not offered in.
    pub not_offered_profiles: Vec<DeploymentProfile>,
    /// The drill results that exercise the row.
    pub drills: Vec<CertificationDrillResult>,
    /// Non-empty local-safe baseline that always continues when the row narrows.
    pub local_safe_baseline: Vec<String>,
    /// The marketed claim the row declares before any drill narrows it.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the weakest drill cap is applied.
    pub effective_certified_claim: MarketedClaim,
    /// The drills that narrowed the row below its declared claim.
    pub narrowing_reasons: Vec<CertificationDrill>,
    /// Short recovery cue. Present (non-null) when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_cue: Option<String>,
}

impl HonestyCertificationRow {
    /// True when the row still certifies its full managed claim.
    pub fn is_fully_certified(&self) -> bool {
        self.effective_certified_claim == self.declared_marketed_claim
    }

    /// True when the row is certified in `profile`.
    pub fn certifies_in(&self, profile: DeploymentProfile) -> bool {
        self.certified_profiles.contains(&profile)
    }

    /// Recomputes the row's drill caps, effective claim, reasons, and cue.
    fn recompute(&mut self) {
        let mut effective = self.declared_marketed_claim;
        let mut reasons = Vec::new();
        for drill in &mut self.drills {
            drill.claim_cap =
                CertificationDrillResult::derive_cap(drill.grade, drill.evidence_status);
            effective = weaker_claim(effective, drill.claim_cap);
            if claim_rank(drill.claim_cap) < claim_rank(self.declared_marketed_claim) {
                reasons.push(drill.drill);
            }
        }
        self.effective_certified_claim = effective;
        self.narrowing_reasons = reasons;
        self.recovery_cue = if effective == self.declared_marketed_claim {
            None
        } else {
            Some(row_recovery_cue(self.dimension))
        };
    }
}

/// One surface bound to the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The surface that consumes the certification.
    pub surface: CertificationSurface,
    /// The dimensions this surface projects.
    pub bound_dimensions: Vec<HonestyDimension>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// True when the surface narrows a marketed claim from a failed certification.
    pub narrows_on_failure: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HonestyCertificationInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of certification rows.
    pub row_count: usize,
    /// Number of distinct honesty dimensions covered.
    pub dimensions_covered: usize,
    /// True when all six honesty dimensions appear exactly once.
    pub dimension_vocab_complete: bool,
    /// Number of distinct drills exercised across the packet.
    pub drills_exercised: usize,
    /// True when all nine drills appear at least once.
    pub drill_vocab_complete: bool,
    /// Total number of drill results across all rows.
    pub total_drill_results: usize,
    /// Number of distinct deployment profiles addressed (certified or not offered).
    pub deployment_profiles_addressed: usize,
    /// Number of distinct deployment profiles certified in at least one row.
    pub deployment_profiles_certified: usize,
    /// True when every deployment profile is addressed by every row.
    pub addresses_all_deployment_profiles: bool,
    /// True when at least one self-host or air-gapped profile is certified.
    pub certifies_self_host_or_offline: bool,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// True when all six consumer surfaces are bound.
    pub surface_coverage_complete: bool,
    /// True when every row carries a non-empty local-safe baseline.
    pub all_rows_local_safe_backed: bool,
    /// Number of rows still certifying the full managed claim.
    pub certified_row_count: usize,
    /// Number of rows narrowed below their declared claim.
    pub narrowed_row_count: usize,
    /// Number of rows narrowed to the local-safe-only claim.
    pub local_safe_only_row_count: usize,
    /// True when no row is narrowed.
    pub fully_certified: bool,
}

/// The frozen certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HonestyCertificationPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the packet content.
    pub packet_revision: u32,
    /// Reviewable packet title.
    pub title: String,
    /// Reviewable packet summary.
    pub summary: String,
    /// Source schema and contract refs the packet cites.
    pub source_refs: Vec<String>,
    /// The certification rows.
    pub rows: Vec<HonestyCertificationRow>,
    /// The surface bindings.
    pub surface_bindings: Vec<CertificationSurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: HonestyCertificationInspection,
}

impl HonestyCertificationPacket {
    /// Serializes the packet as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the packet cannot be serialized, which a validated packet never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("packet serializes to JSON")
    }

    /// Recomputes every derived value: drill caps, row claims, and the inspection block.
    pub fn recompute(&mut self) {
        for row in &mut self.rows {
            row.recompute();
        }
        self.inspection =
            HonestyCertificationInspection::derive(&self.rows, &self.surface_bindings);
    }

    /// Narrows a single row by failing one of its drills, then recomputes.
    ///
    /// The named drill on the row for `dimension` is regraded
    /// [`DrillGrade::Narrowed`]; the row's effective claim, reasons, and recovery
    /// cue and the inspection block are recomputed. The local-safe baseline is
    /// never removed, so the local core stays available. Returns `true` when a
    /// matching drill was found and narrowed.
    pub fn narrow_for_drill_failure(
        &mut self,
        dimension: HonestyDimension,
        drill: CertificationDrill,
    ) -> bool {
        let mut narrowed = false;
        for row in &mut self.rows {
            if row.dimension != dimension {
                continue;
            }
            for result in &mut row.drills {
                if result.drill == drill {
                    result.grade = DrillGrade::Narrowed;
                    narrowed = true;
                }
            }
            if narrowed {
                row.recompute();
            }
        }
        if narrowed {
            self.inspection =
                HonestyCertificationInspection::derive(&self.rows, &self.surface_bindings);
        }
        narrowed
    }

    /// The certification row for a dimension, if present.
    pub fn row(&self, dimension: HonestyDimension) -> Option<&HonestyCertificationRow> {
        self.rows.iter().find(|r| r.dimension == dimension)
    }

    /// Cross-checks the certification against every backing sibling consumer.
    ///
    /// Loads the control-plane matrix and each sibling packet, confirms each
    /// validates cleanly, and confirms each row's backing consumer matches its
    /// dimension, its declared claim matches a managed control-plane lane, and its
    /// local-safe baseline is non-empty, so the certification rides real consumers
    /// rather than a parallel scorecard. Returns an empty vector when every backing
    /// consumer matches.
    pub fn cross_check_backing_consumers(&self) -> Vec<HonestyCertificationViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: String| {
            violations.push(HonestyCertificationViolation {
                field: field.to_owned(),
                message,
            });
        };

        // The control-plane matrix is the anchor every managed claim cites.
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let matrix_violations = matrix.validate();
        if !matrix_violations.is_empty() {
            push(
                "backing.commercial_control_plane",
                format!(
                    "control-plane matrix failed validation: {} violation(s)",
                    matrix_violations.len()
                ),
            );
        }
        let managed_full_lane = matrix
            .lanes
            .iter()
            .any(|lane| lane.declared_marketed_claim == MarketedClaim::ManagedFull);
        if !managed_full_lane {
            push(
                "backing.commercial_control_plane",
                "control-plane matrix exposes no managed_full lane to certify against".to_owned(),
            );
        }

        // The boundary card set anchors the open-versus-paid and air-gapped truth.
        let boundary = canonical_stable_commercial_boundary_card_set();
        let boundary_violations = boundary.validate();
        if !boundary_violations.is_empty() {
            push(
                "backing.commercial_boundary_cards",
                format!(
                    "boundary card set failed validation: {} violation(s)",
                    boundary_violations.len()
                ),
            );
        }
        let boundary_drift = boundary.cross_check_against_control_plane();
        if !boundary_drift.is_empty() {
            push(
                "backing.commercial_boundary_cards",
                format!(
                    "boundary card set drifted from the control plane: {} violation(s)",
                    boundary_drift.len()
                ),
            );
        }

        // Each remaining backing consumer must validate cleanly.
        let entitlement = crate::m5_entitlement_summary::canonical_stable_entitlement_summary_set();
        if !entitlement.validate().is_empty() {
            push(
                "backing.entitlement_summary",
                "entitlement summary set failed validation".to_owned(),
            );
        }
        let forecast = crate::m5_usage_forecast_views::canonical_stable_usage_forecast_view_set();
        if !forecast.validate().is_empty() {
            push(
                "backing.usage_forecast_views",
                "usage forecast view set failed validation".to_owned(),
            );
        }
        let chargeback =
            crate::m5_chargeback_scope_views::canonical_stable_chargeback_scope_view_set();
        if !chargeback.validate().is_empty() {
            push(
                "backing.chargeback_scope_views",
                "chargeback scope view set failed validation".to_owned(),
            );
        }
        let metering =
            crate::m5_metering_degradation_rules::canonical_stable_metering_degradation_rule_set();
        if !metering.validate().is_empty() {
            push(
                "backing.metering_degradation_rules",
                "metering degradation rule set failed validation".to_owned(),
            );
        }
        let offboarding = crate::m5_offboarding_cards::canonical_stable_offboarding_card_set();
        if !offboarding.validate().is_empty() {
            push(
                "backing.offboarding_cards",
                "offboarding card set failed validation".to_owned(),
            );
        }

        // Every row must ride its declared backing consumer and a managed lane.
        for row in &self.rows {
            if row.backing_consumer != row.dimension.backing_consumer() {
                push(
                    "row.backing_consumer",
                    format!(
                        "row {} cites the wrong backing consumer for its dimension",
                        row.row_id
                    ),
                );
            }
            if row.declared_marketed_claim != MarketedClaim::ManagedFull {
                push(
                    "row.declared_marketed_claim",
                    format!("row {} must declare the full managed claim", row.row_id),
                );
            }
            if row.local_safe_baseline.is_empty() {
                push(
                    "row.local_safe_baseline",
                    format!(
                        "row {} must keep a non-empty local-safe baseline",
                        row.row_id
                    ),
                );
            }
        }

        violations
    }

    /// Validates the packet and recomputes every derived value.
    ///
    /// Returns an empty vector when the packet is internally consistent. Otherwise
    /// returns one [`HonestyCertificationViolation`] per failed invariant.
    pub fn validate(&self) -> Vec<HonestyCertificationViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(HonestyCertificationViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != PACKET_RECORD_KIND {
            push("record_kind", "packet record_kind is wrong");
        }
        if self.schema_version != HONESTY_CERTIFICATION_SCHEMA_VERSION {
            push("schema_version", "packet schema_version is wrong");
        }
        if self.packet_id.trim().is_empty() {
            push("packet_id", "packet_id must be non-empty");
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
        if self.packet_revision == 0 {
            push("packet_revision", "packet_revision must be at least 1");
        }
        if !self
            .source_refs
            .iter()
            .any(|entry| entry == HONESTY_CERTIFICATION_SCHEMA_REF)
        {
            push("source_refs", "packet must cite its boundary schema");
        }
        if self.rows.is_empty() {
            push("rows", "packet must contain at least one certification row");
        }

        let mut seen_dimensions = BTreeSet::new();
        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            self.validate_row(row, &mut push);
            if !seen_dimensions.insert(row.dimension) {
                push("rows", "each honesty dimension must appear exactly once");
            }
            if !row_ids.insert(row.row_id.as_str()) {
                push("rows", "row_id values must be unique");
            }
        }
        if seen_dimensions.len() != HonestyDimension::ALL.len() {
            push("rows", "the packet must certify all six honesty dimensions");
        }

        // Every drill in the vocabulary must be exercised at least once.
        let exercised: BTreeSet<CertificationDrill> = self
            .rows
            .iter()
            .flat_map(|r| r.drills.iter().map(|d| d.drill))
            .collect();
        if exercised.len() != CertificationDrill::ALL.len() {
            push(
                "rows",
                "every certification drill must be exercised by at least one row",
            );
        }

        // The certification must not ride one vendor-managed online profile alone.
        let certified_union: BTreeSet<DeploymentProfile> = self
            .rows
            .iter()
            .flat_map(|r| r.certified_profiles.iter().copied())
            .collect();
        if !certified_union.contains(&DeploymentProfile::SelfHosted)
            && !certified_union.contains(&DeploymentProfile::AirGapped)
        {
            push(
                "rows.certified_profiles",
                "certification must cover at least one self-host or air-gapped profile, not vendor-managed online only",
            );
        }

        self.validate_surface_bindings(&mut push);

        let derived = HonestyCertificationInspection::derive(&self.rows, &self.surface_bindings);
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed packet",
            );
        }

        violations
    }

    fn validate_row(&self, row: &HonestyCertificationRow, push: &mut impl FnMut(&str, &str)) {
        if row.record_kind != ROW_RECORD_KIND {
            push("row.record_kind", "row record_kind is wrong");
        }
        if row.schema_version != HONESTY_CERTIFICATION_SCHEMA_VERSION {
            push("row.schema_version", "row schema_version is wrong");
        }
        if row.row_id.trim().is_empty() {
            push("row.row_id", "row_id must be non-empty");
        }
        if row.title.trim().is_empty() || row.summary.trim().is_empty() {
            push("row", "row title and summary must be non-empty");
        }
        if row.backing_consumer != row.dimension.backing_consumer() {
            push(
                "row.backing_consumer",
                "row backing_consumer must match its dimension",
            );
        }
        if row.service_families.is_empty() {
            push(
                "row.service_families",
                "row must name at least one managed service family",
            );
        }
        let mut seen_families = BTreeSet::new();
        for family in &row.service_families {
            if !seen_families.insert(*family) {
                push("row.service_families", "service families must be distinct");
            }
        }
        // The local core is never blocked.
        if row.local_safe_baseline.is_empty()
            || row.local_safe_baseline.iter().any(|s| s.trim().is_empty())
        {
            push(
                "row.local_safe_baseline",
                "every row must keep a non-empty local-safe baseline",
            );
        }

        // The certified and not-offered profiles must partition all five profiles.
        let mut profile_seen = BTreeSet::new();
        let mut duplicate_profile = false;
        for profile in row
            .certified_profiles
            .iter()
            .chain(row.not_offered_profiles.iter())
        {
            if !profile_seen.insert(*profile) {
                duplicate_profile = true;
            }
        }
        if duplicate_profile {
            push(
                "row.certified_profiles",
                "a profile cannot be both certified and not offered",
            );
        }
        if profile_seen.len() != DeploymentProfile::ALL.len() {
            push(
                "row.certified_profiles",
                "every deployment profile must be addressed as certified or not offered",
            );
        }
        if row.certified_profiles.is_empty() {
            push(
                "row.certified_profiles",
                "a row must be certified in at least one deployment profile",
            );
        }

        // The drills must be the exact required set for the dimension, once each.
        let required: BTreeSet<CertificationDrill> =
            required_drills(row.dimension).iter().copied().collect();
        let present: BTreeSet<CertificationDrill> = row.drills.iter().map(|d| d.drill).collect();
        if present != required {
            push(
                "row.drills",
                "row drills must be exactly the required set for the dimension",
            );
        }
        if present.len() != row.drills.len() {
            push("row.drills", "row drills must not repeat a drill");
        }
        for drill in &row.drills {
            self.validate_drill(drill, push);
        }

        // Recompute the row's effective claim, reasons, and cue.
        let mut recomputed = row.clone();
        recomputed.recompute();
        if recomputed.effective_certified_claim != row.effective_certified_claim {
            push(
                "row.effective_certified_claim",
                "stored effective claim does not match the recomputed row",
            );
        }
        let stored_reasons: BTreeSet<CertificationDrill> =
            row.narrowing_reasons.iter().copied().collect();
        let derived_reasons: BTreeSet<CertificationDrill> =
            recomputed.narrowing_reasons.iter().copied().collect();
        if stored_reasons != derived_reasons {
            push(
                "row.narrowing_reasons",
                "stored narrowing reasons do not match the recomputed row",
            );
        }
        match (&row.recovery_cue, recomputed.recovery_cue.is_some()) {
            (None, true) => push(
                "row.recovery_cue",
                "a narrowed row must carry a recovery cue",
            ),
            (Some(_), false) => push(
                "row.recovery_cue",
                "a fully certified row must not carry a recovery cue",
            ),
            (Some(cue), _) if cue.trim().is_empty() => {
                push("row.recovery_cue", "recovery cue must be non-empty")
            }
            _ => {}
        }
    }

    fn validate_drill(&self, drill: &CertificationDrillResult, push: &mut impl FnMut(&str, &str)) {
        if drill.record_kind != DRILL_RESULT_RECORD_KIND {
            push("drill.record_kind", "drill record_kind is wrong");
        }
        if drill.schema_version != HONESTY_CERTIFICATION_SCHEMA_VERSION {
            push("drill.schema_version", "drill schema_version is wrong");
        }
        if drill.detail.trim().is_empty() {
            push("drill.detail", "drill detail must be non-empty");
        }
        // A not-applicable drill must carry current evidence so it never narrows.
        if drill.grade == DrillGrade::NotApplicable
            && drill.evidence_status != BoundaryEvidenceStatus::Current
        {
            push(
                "drill.evidence_status",
                "a not-applicable drill must carry current evidence",
            );
        }
        let expected = CertificationDrillResult::derive_cap(drill.grade, drill.evidence_status);
        if drill.claim_cap != expected {
            push(
                "drill.claim_cap",
                "stored drill claim_cap does not match the recomputed grade/evidence cap",
            );
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let dimensions: BTreeSet<HonestyDimension> =
            self.rows.iter().map(|r| r.dimension).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != HONESTY_CERTIFICATION_SCHEMA_VERSION {
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
            if binding.narrows_on_failure != binding.surface.narrows_on_failure() {
                push(
                    "surface_binding.narrows_on_failure",
                    "binding narrows_on_failure must match the surface's narrowing role",
                );
            }
            if binding.bound_dimensions.is_empty() {
                push(
                    "surface_binding.bound_dimensions",
                    "a binding must project at least one dimension",
                );
            }
            for dim in &binding.bound_dimensions {
                if !dimensions.contains(dim) {
                    push(
                        "surface_binding.bound_dimensions",
                        "binding dimension must resolve to a certification row",
                    );
                }
            }
        }
        for surface in CertificationSurface::ALL {
            if !self.surface_bindings.iter().any(|b| b.surface == surface) {
                push(
                    "surface_bindings",
                    "release center, Help/About, diagnostics, service health, support/admin, and claim automation must all bind",
                );
                break;
            }
        }
        // At least one surface must narrow a marketed claim from the verdict.
        if !self.surface_bindings.iter().any(|b| b.narrows_on_failure) {
            push(
                "surface_bindings",
                "at least one surface must narrow the marketed claim on a failed certification",
            );
        }
    }
}

impl HonestyCertificationInspection {
    fn derive(
        rows: &[HonestyCertificationRow],
        surface_bindings: &[CertificationSurfaceBinding],
    ) -> Self {
        let dimensions: BTreeSet<HonestyDimension> = rows.iter().map(|r| r.dimension).collect();
        let exercised: BTreeSet<CertificationDrill> = rows
            .iter()
            .flat_map(|r| r.drills.iter().map(|d| d.drill))
            .collect();
        let addressed: BTreeSet<DeploymentProfile> = rows
            .iter()
            .flat_map(|r| {
                r.certified_profiles
                    .iter()
                    .chain(r.not_offered_profiles.iter())
                    .copied()
            })
            .collect();
        let certified: BTreeSet<DeploymentProfile> = rows
            .iter()
            .flat_map(|r| r.certified_profiles.iter().copied())
            .collect();
        let surfaces: BTreeSet<CertificationSurface> =
            surface_bindings.iter().map(|b| b.surface).collect();

        let certified_row_count = rows.iter().filter(|r| r.is_fully_certified()).count();
        let narrowed_row_count = rows.iter().filter(|r| !r.is_fully_certified()).count();
        let local_safe_only_row_count = rows
            .iter()
            .filter(|r| r.effective_certified_claim == MarketedClaim::LocalSafeOnly)
            .count();
        let addresses_all = rows.iter().all(|r| {
            r.certified_profiles
                .iter()
                .chain(r.not_offered_profiles.iter())
                .copied()
                .collect::<BTreeSet<_>>()
                .len()
                == DeploymentProfile::ALL.len()
        });

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: HONESTY_CERTIFICATION_SCHEMA_VERSION,
            row_count: rows.len(),
            dimensions_covered: dimensions.len(),
            dimension_vocab_complete: dimensions.len() == HonestyDimension::ALL.len(),
            drills_exercised: exercised.len(),
            drill_vocab_complete: exercised.len() == CertificationDrill::ALL.len(),
            total_drill_results: rows.iter().map(|r| r.drills.len()).sum(),
            deployment_profiles_addressed: addressed.len(),
            deployment_profiles_certified: certified.len(),
            addresses_all_deployment_profiles: addressed.len() == DeploymentProfile::ALL.len()
                && addresses_all,
            certifies_self_host_or_offline: certified.contains(&DeploymentProfile::SelfHosted)
                || certified.contains(&DeploymentProfile::AirGapped),
            surface_binding_count: surface_bindings.len(),
            surface_coverage_complete: surfaces.len() == CertificationSurface::ALL.len(),
            all_rows_local_safe_backed: rows.iter().all(|r| !r.local_safe_baseline.is_empty()),
            certified_row_count,
            narrowed_row_count,
            local_safe_only_row_count,
            fully_certified: narrowed_row_count == 0,
        }
    }
}

/// One failed packet invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HonestyCertificationViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for HonestyCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in packet cannot be read or validated.
#[derive(Debug)]
pub enum HonestyCertificationError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in packet failed validation.
    Validation(Vec<HonestyCertificationViolation>),
}

impl fmt::Display for HonestyCertificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "honesty-certification packet parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "honesty-certification packet failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for HonestyCertificationError {}

/// Strength rank used to take the weaker of two marketed claims.
fn claim_rank(claim: MarketedClaim) -> u8 {
    match claim {
        MarketedClaim::LocalSafeOnly => 0,
        MarketedClaim::ManagedNarrowed => 1,
        MarketedClaim::ManagedFull => 2,
    }
}

/// Returns the weaker (more-narrowed) of two marketed claims.
fn weaker_claim(a: MarketedClaim, b: MarketedClaim) -> MarketedClaim {
    if claim_rank(a) <= claim_rank(b) {
        a
    } else {
        b
    }
}

/// The exact drill set required for each honesty dimension.
fn required_drills(dimension: HonestyDimension) -> &'static [CertificationDrill] {
    use CertificationDrill::*;
    match dimension {
        HonestyDimension::EntitlementHonesty => &[
            SeatLossDrill,
            OrgSwitchDrill,
            GracePeriodDrill,
            ExportRightsValidation,
        ],
        HonestyDimension::MeteringHonesty => {
            &[StaleMeterDrill, FailOpenLocalCore, FailClosedManagedAction]
        }
        HonestyDimension::ForecastHonesty => {
            &[StaleMeterDrill, FailOpenLocalCore, ExportRightsValidation]
        }
        HonestyDimension::ChargebackHonesty => {
            &[ChargebackScopeExportCheck, ExportRightsValidation]
        }
        HonestyDimension::DowngradeOffboardingHonesty => &[
            SeatLossDrill,
            OrgSwitchDrill,
            GracePeriodDrill,
            ExportRightsValidation,
        ],
        HonestyDimension::CommercialBoundaryHonesty => {
            &[ResidualDependencyDisclosureReview, ExportRightsValidation]
        }
    }
}

/// The recovery cue shown when a certification row narrows its claim.
fn row_recovery_cue(dimension: HonestyDimension) -> String {
    match dimension {
        HonestyDimension::EntitlementHonesty => {
            "Entitlement certification narrowed; the account context is shown without the broader managed claim. Local editing, search, and Git continue now."
        }
        HonestyDimension::MeteringHonesty => {
            "Metering certification narrowed; the local core keeps running and the gated managed action names its blocking reason. Local work continues now."
        }
        HonestyDimension::ForecastHonesty => {
            "Forecast certification narrowed; usage is shown without the broader managed forecast claim. Local work continues now."
        }
        HonestyDimension::ChargebackHonesty => {
            "Chargeback certification narrowed; scope ownership is shown without the broader managed claim. Local work continues now."
        }
        HonestyDimension::DowngradeOffboardingHonesty => {
            "Downgrade certification narrowed; export and local continuation stay above any upgrade prompt. Local work continues now."
        }
        HonestyDimension::CommercialBoundaryHonesty => {
            "Boundary certification narrowed; the open-versus-paid card claims only what its evidence supports. Local work continues now."
        }
    }
    .to_owned()
}

// ---- canonical builder -------------------------------------------------------

/// Stable identifier for the checked-in packet.
pub const STABLE_PACKET_ID: &str = "commercial-honesty-certification:stable:0001";

/// Stable title for the checked-in packet.
pub const STABLE_PACKET_TITLE: &str =
    "Entitlement, metering, forecast, chargeback, and downgrade honesty certification across claimed managed deployment profiles";

/// Deterministic timestamp for the checked-in packet.
pub const STABLE_PACKET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in packet.
pub const STABLE_PACKET_REVISION: u32 = 1;

/// Source refs every certification export carries.
fn packet_source_refs() -> Vec<String> {
    vec![
        HONESTY_CERTIFICATION_SCHEMA_REF.to_owned(),
        HONESTY_CERTIFICATION_DOC_REF.to_owned(),
        BackingConsumer::CommercialControlPlane
            .artifact_path()
            .to_owned(),
        BackingConsumer::EntitlementSummary
            .artifact_path()
            .to_owned(),
        BackingConsumer::UsageForecastViews
            .artifact_path()
            .to_owned(),
        BackingConsumer::ChargebackScopeViews
            .artifact_path()
            .to_owned(),
        BackingConsumer::MeteringDegradationRules
            .artifact_path()
            .to_owned(),
        BackingConsumer::OffboardingCards.artifact_path().to_owned(),
        BackingConsumer::CommercialBoundaryCards
            .artifact_path()
            .to_owned(),
    ]
}

/// Builds one fully certified drill result (`Certified` grade, `Current` evidence).
fn drill(
    drill: CertificationDrill,
    backing_consumer: BackingConsumer,
    detail: &str,
) -> CertificationDrillResult {
    CertificationDrillResult {
        record_kind: DRILL_RESULT_RECORD_KIND.to_owned(),
        schema_version: HONESTY_CERTIFICATION_SCHEMA_VERSION,
        drill,
        grade: DrillGrade::Certified,
        evidence_status: BoundaryEvidenceStatus::Current,
        claim_cap: MarketedClaim::ManagedFull,
        backing_consumer,
        detail: detail.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    dimension: HonestyDimension,
    title: &str,
    summary: &str,
    certified_profiles: &[DeploymentProfile],
    not_offered_profiles: &[DeploymentProfile],
    local_safe_baseline: &[&str],
    drills: Vec<CertificationDrillResult>,
) -> HonestyCertificationRow {
    let mut built = HonestyCertificationRow {
        record_kind: ROW_RECORD_KIND.to_owned(),
        schema_version: HONESTY_CERTIFICATION_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        dimension,
        title: title.to_owned(),
        summary: summary.to_owned(),
        backing_consumer: dimension.backing_consumer(),
        service_families: ServiceFamily::ALL.to_vec(),
        certified_profiles: certified_profiles.to_vec(),
        not_offered_profiles: not_offered_profiles.to_vec(),
        drills,
        local_safe_baseline: local_safe_baseline
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        declared_marketed_claim: MarketedClaim::ManagedFull,
        effective_certified_claim: MarketedClaim::ManagedFull,
        narrowing_reasons: Vec::new(),
        recovery_cue: None,
    };
    built.recompute();
    built
}

fn binding(
    binding_id: &str,
    surface: CertificationSurface,
    summary: &str,
) -> CertificationSurfaceBinding {
    CertificationSurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: HONESTY_CERTIFICATION_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        surface,
        bound_dimensions: HonestyDimension::ALL.to_vec(),
        projects_effective_claim: true,
        narrows_on_failure: surface.narrows_on_failure(),
        summary: summary.to_owned(),
    }
}

/// Builds the canonical, fully certified packet from the frozen vocabulary.
///
/// Every row certifies `managed_full` with all drills graded `certified` and
/// their evidence `current`, so the packet is the green baseline the checked-in
/// artifact pins.
pub fn canonical_honesty_certification_packet() -> HonestyCertificationPacket {
    use BackingConsumer as B;
    use CertificationDrill as D;
    use DeploymentProfile as P;

    // The managed lanes are offered self-hosted, enterprise-online, and managed
    // cloud; they are honestly not offered to a single fully-local user or in an
    // air-gapped deployment.
    let managed_certified = [P::SelfHosted, P::EnterpriseOnline, P::ManagedCloud];
    let managed_not_offered = [P::IndividualLocal, P::AirGapped];

    let rows = vec![
        row(
            "honesty.entitlement",
            HonestyDimension::EntitlementHonesty,
            "Entitlement context is honest across the claimed managed profiles",
            "Certifies that plan, seat owner, role, org/tenant scope, and quota-snapshot age render distinctly, that a seat loss or an org switch degrades to an explicit managed-blocked state rather than a generic sign-in error, and that local-only continuation notes always hold.",
            &managed_certified,
            &managed_not_offered,
            &[
                "Local editing, search, and Git continue with no managed account.",
                "The account context degrades to a local-only continuation note, never a generic account error.",
            ],
            vec![
                drill(D::SeatLossDrill, B::OffboardingCards, "A removed seat renders as a distinct seat-removed state with its export rights, never collapsed into a sign-in failure."),
                drill(D::OrgSwitchDrill, B::OffboardingCards, "An org switch renders as a distinct org-switched state while managed scope rebinds, never collapsed into a seat loss."),
                drill(D::GracePeriodDrill, B::OffboardingCards, "A grace window renders as a distinct grace-period state and preserves export-before-suspend."),
                drill(D::ExportRightsValidation, B::EntitlementSummary, "The entitlement summary exports at its documented parity with the quota-snapshot age bound to its as-of time and scope owner."),
            ],
        ),
        row(
            "honesty.metering",
            HonestyDimension::MeteringHonesty,
            "Metering degradation is honest across the claimed managed profiles",
            "Certifies that a stale meter keeps its number labeled, that the local core fails open and keeps editing, search, and Git running, and that only one named spend-bearing managed action fails closed with its blocking reason.",
            &managed_certified,
            &managed_not_offered,
            &[
                "Local editing, search, Git, and existing local automation keep running when a metering or rating path is stale or unreachable.",
            ],
            vec![
                drill(D::StaleMeterDrill, B::MeteringDegradationRules, "A stale meter keeps its number bound to its unit, as-of time, and scope owner or suppresses it; it never goes silently green."),
                drill(D::FailOpenLocalCore, B::MeteringDegradationRules, "A stale or unreachable metering path fails open to the local-safe baseline without blocking the local core."),
                drill(D::FailClosedManagedAction, B::MeteringDegradationRules, "Exactly one named spend-bearing managed action fails closed with its blocking reason, retry, and details actions."),
            ],
        ),
        row(
            "honesty.forecast",
            HonestyDimension::ForecastHonesty,
            "Usage and forecast messaging is honest across the claimed managed profiles",
            "Certifies that the month-to-date measurement stays bound to its unit, as-of time, and scope owner, that the forecast threshold banner explains what changes next, and that the usage view exports at CSV/JSON parity.",
            &managed_certified,
            &managed_not_offered,
            &[
                "Local editing, search, and Git continue regardless of the managed usage forecast.",
                "Usage numbers are never shown bare; each carries its unit, as-of time, and scope owner.",
            ],
            vec![
                drill(D::StaleMeterDrill, B::UsageForecastViews, "A stale meter narrows the forecast claim and keeps the month-to-date number labeled rather than optimistic."),
                drill(D::FailOpenLocalCore, B::UsageForecastViews, "An unavailable forecast falls back to a best-effort or suppressed forecast without blocking the local core."),
                drill(D::ExportRightsValidation, B::UsageForecastViews, "The usage and forecast view exports at CSV/JSON parity with every number bound to its unit, as-of time, and scope owner."),
            ],
        ),
        row(
            "honesty.chargeback",
            HonestyDimension::ChargebackHonesty,
            "Chargeback scope ownership is honest across the claimed managed profiles",
            "Certifies that personal, workspace, team, and organization cost stay distinct, that direct cost is separated from an inherited share that names its parent scope, and that the scope set exports at CSV/JSON parity.",
            &managed_certified,
            &managed_not_offered,
            &[
                "Local editing, search, and Git continue regardless of the managed chargeback view.",
                "Cost is attributed to a named scope owner; personal, workspace, team, and organization never collapse into one bucket.",
            ],
            vec![
                drill(D::ChargebackScopeExportCheck, B::ChargebackScopeViews, "The chargeback export keeps every offered scope distinct and separates direct cost from an inherited share that names its parent scope."),
                drill(D::ExportRightsValidation, B::ChargebackScopeViews, "The chargeback scope set exports at CSV/JSON parity with each value bound to its unit, as-of time, and owner."),
            ],
        ),
        row(
            "honesty.downgrade_offboarding",
            HonestyDimension::DowngradeOffboardingHonesty,
            "Downgrade and offboarding continuity is honest across the claimed managed profiles",
            "Certifies that a grace period, a seat loss, a cancellation, and an org switch stay distinct, that each states its effective date, impacted features, export rights, deletion timeline, and owner handoff, and that export and local continuation stay above any upgrade prompt.",
            &managed_certified,
            &managed_not_offered,
            &[
                "Local artifacts keep working after a managed entitlement winds down; the local core is never deleted or blocked.",
                "Export and local continuation always rank above any upgrade or renewal prompt.",
            ],
            vec![
                drill(D::SeatLossDrill, B::OffboardingCards, "The seat-loss card states its effective date, impacted features, export rights, deletion timeline, and owner handoff, distinct from the other three events."),
                drill(D::OrgSwitchDrill, B::OffboardingCards, "The org-switch card separates local artifacts from tenant-scoped managed state and stays distinct from a seat loss."),
                drill(D::GracePeriodDrill, B::OffboardingCards, "The grace-period card keeps the offboarding export admissible before suspension."),
                drill(D::ExportRightsValidation, B::OffboardingCards, "The offboarding export honors its documented export rights at parity, above any upgrade prompt."),
            ],
        ),
        row(
            "honesty.commercial_boundary",
            HonestyDimension::CommercialBoundaryHonesty,
            "Commercial-boundary truth is honest across every deployment profile",
            "Certifies that the open-versus-paid boundary, the residual vendor-hosted dependency disclosure, the deployment-profile qualifiers, and the procurement/support packet parity all hold, including the air-gapped profile where the local open core stands alone.",
            &P::ALL,
            &[],
            &[
                "The local open core runs in every deployment profile, including air-gapped, with no managed dependency.",
                "Procurement and support evidence stays above any upsell prompt and defers spend numbers to the metering surfaces.",
            ],
            vec![
                drill(D::ResidualDependencyDisclosureReview, B::CommercialBoundaryCards, "Every managed card discloses its residual vendor-hosted dependencies and whether self-hosting eliminates them; the local open core declares none."),
                drill(D::ExportRightsValidation, B::CommercialBoundaryCards, "The procurement and support packets reuse one evidence object at export parity, keeping evidence above any upsell prompt."),
            ],
        ),
    ];

    let surface_bindings = vec![
        binding(
            "certification.release_center",
            CertificationSurface::ReleaseCenter,
            "The release center reads the certification verdict and narrows a train row when a dimension fails or its evidence goes stale.",
        ),
        binding(
            "certification.help_about",
            CertificationSurface::HelpAbout,
            "Help/About projects the certified open-versus-paid boundary truth rather than optimistic managed marketing.",
        ),
        binding(
            "certification.diagnostics",
            CertificationSurface::Diagnostics,
            "Diagnostics projects the per-dimension certification and its fail-open/fail-closed drills.",
        ),
        binding(
            "certification.service_health",
            CertificationSurface::ServiceHealth,
            "Service health projects the metering degradation certification alongside the managed-lane posture.",
        ),
        binding(
            "certification.support_admin_packet",
            CertificationSurface::SupportAdminPacket,
            "The support and admin export packet carries the certification rows, drills, and export-rights validation at parity.",
        ),
        binding(
            "certification.claim_public_truth_automation",
            CertificationSurface::ClaimPublicTruthAutomation,
            "Claim and public-truth automation narrows the marketed claim to each row's effective certified claim.",
        ),
    ];

    let inspection = HonestyCertificationInspection::derive(&rows, &surface_bindings);

    HonestyCertificationPacket {
        record_kind: PACKET_RECORD_KIND.to_owned(),
        schema_version: HONESTY_CERTIFICATION_SCHEMA_VERSION,
        packet_id: STABLE_PACKET_ID.to_owned(),
        generated_at: STABLE_PACKET_GENERATED_AT.to_owned(),
        packet_revision: STABLE_PACKET_REVISION,
        title: STABLE_PACKET_TITLE.to_owned(),
        summary: "Certifies entitlement, metering, forecast, chargeback, and downgrade/offboarding honesty across the claimed managed deployment profiles, narrowing any row whose drill fails or whose evidence goes stale instead of inheriting broader managed marketing language, and keeping a non-empty local-safe baseline on every row.".to_owned(),
        source_refs: packet_source_refs(),
        rows,
        surface_bindings,
        inspection,
    }
}

/// Builds the canonical packet (alias kept stable for dump and consumer call sites).
pub fn canonical_stable_honesty_certification_packet() -> HonestyCertificationPacket {
    canonical_honesty_certification_packet()
}

/// Reads and validates the checked-in stable certification packet.
///
/// This is the canonical reader: the release center, Help/About, diagnostics,
/// service health, support/admin packets, and claim/public-truth automation call
/// it to ingest the certification rather than cloning verdict text.
///
/// # Errors
///
/// Returns [`HonestyCertificationError`] when the checked-in packet fails to parse
/// or fails validation.
pub fn current_stable_honesty_certification_packet(
) -> Result<HonestyCertificationPacket, HonestyCertificationError> {
    let packet: HonestyCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-commercial-honesty-certification.json"
    )))
    .map_err(HonestyCertificationError::Parse)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HonestyCertificationError::Validation(violations))
    }
}
