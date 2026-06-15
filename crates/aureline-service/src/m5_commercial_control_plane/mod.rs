//! Frozen entitlement, meter-family, chargeback-scope, org-switch, and
//! grace-period matrix for the managed lanes.
//!
//! This module is the canonical commercial-control-plane object. It enumerates
//! one [`ManagedLaneRow`] per claimed managed lane — the AI gateway, settings
//! sync, the companion relay, the registry/mirror surface, support ingest, and
//! the managed workspace — and, alongside them, one [`ManagedStateRow`] per
//! locked user-visible managed-state token and one [`ConsumerBinding`] per
//! consumer surface that must project the matrix.
//!
//! Each lane row freezes the columns the contract requires: the service family
//! and meter family, the meter unit and aggregation window, the as-of-time
//! requirement, the scope owner and the distinct chargeback scopes (personal,
//! workspace, and organization never collapse into one total), the
//! fail-open/fail-closed posture, the forecast confidence, the grace-period
//! rights, the export guarantee, the local-safe baseline that always continues,
//! and the managed-only actions that pause. The lane's **effective marketed
//! claim** is recomputed from its declared claim and the cap of whichever
//! managed state is active, so a stale meter, an exhausted forecast, a grace
//! window, a removed seat, an org switch, or a plan downgrade narrows the
//! marketed claim automatically; the stored value must equal that recomputation
//! or [`CommercialControlPlaneMatrix::validate`] reports a violation.
//!
//! Two invariants keep the matrix honest. First, **local core is never
//! blocked**: every lane carries a non-empty [`ManagedLaneRow::local_safe_baseline`]
//! and every managed-state row asserts `local_safe_guaranteed`, so a metering or
//! rating failure narrows a managed action but never opening, editing, saving,
//! searching, local Git, or already-authorized local automation. Second,
//! **states stay distinct**: the ten managed-state rows are exactly the locked
//! vocabulary, and the four states the contract refuses to collapse — a removed
//! seat, an org switch, a grace window, and a sign-in/reauth failure — each list
//! the others in [`ManagedStateRow::must_not_collapse_with`], so a surface can
//! never draw one generic account error over four different conditions.
//!
//! [`canonical_commercial_control_plane_matrix`] builds the frozen matrix and
//! [`current_stable_commercial_control_plane_matrix`] reads and validates the
//! checked-in packet at
//! [`artifacts/service/m5-commercial-control-plane.json`](../../../../artifacts/service/m5-commercial-control-plane.json),
//! so account surfaces, diagnostics, Help/About, support/admin packets, and
//! claim/public-truth automation all ingest one packet rather than cloning
//! status text. [`CommercialControlPlaneMatrix::apply_managed_state`] narrows
//! every applicable lane for a single active managed state, so release and
//! diagnostics tooling can exercise the narrowing deterministically.
//!
//! The boundary schema is
//! [`schemas/service/m5-commercial-control-plane.schema.json`](../../../../schemas/service/m5-commercial-control-plane.schema.json).
//! The reviewer contract is
//! [`docs/m5/freeze-the-m5-entitlement-meter-family-chargeback-scope-org-switch-and-grace-period-matrix-for-managed-lanes.md`](../../../../docs/m5/freeze-the-m5-entitlement-meter-family-chargeback-scope-org-switch-and-grace-period-matrix-for-managed-lanes.md).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Supported schema version for the commercial-control-plane matrix.
pub const COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix packet.
pub const MATRIX_RECORD_KIND: &str = "m5_commercial_control_plane_matrix";

/// Stable record-kind tag for a managed-lane row.
pub const LANE_RECORD_KIND: &str = "m5_managed_lane_row";

/// Stable record-kind tag for a managed-state row.
pub const MANAGED_STATE_RECORD_KIND: &str = "m5_managed_state_row";

/// Stable record-kind tag for a consumer binding.
pub const CONSUMER_BINDING_RECORD_KIND: &str = "m5_commercial_consumer_binding";

/// Stable record-kind tag for the matrix inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_commercial_control_plane_inspection";

/// Repo-relative path to the boundary schema.
pub const COMMERCIAL_CONTROL_PLANE_SCHEMA_REF: &str =
    "schemas/service/m5-commercial-control-plane.schema.json";

/// Repo-relative path to the reviewer contract.
pub const COMMERCIAL_CONTROL_PLANE_DOC_REF: &str =
    "docs/m5/freeze-the-m5-entitlement-meter-family-chargeback-scope-org-switch-and-grace-period-matrix-for-managed-lanes.md";

/// Repo-relative path to the checked-in matrix packet.
pub const COMMERCIAL_CONTROL_PLANE_ARTIFACT_PATH: &str =
    "artifacts/service/m5-commercial-control-plane.json";

/// Closed service-family vocabulary, re-exported from the operating-mode and
/// metering contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFamily {
    /// Profile and settings sync.
    SyncFamily,
    /// Registry or mirror metadata.
    RegistryOrMirrorMetadataFamily,
    /// Collaboration relay and companion follow.
    CollaborationRelayFamily,
    /// Remote workspace control plane.
    RemoteWorkspaceControlPlaneFamily,
    /// Managed AI gateway.
    AiGatewayFamily,
    /// Telemetry or support ingest.
    TelemetryOrSupportIngestFamily,
}

impl ServiceFamily {
    /// Every service family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SyncFamily,
        Self::RegistryOrMirrorMetadataFamily,
        Self::CollaborationRelayFamily,
        Self::RemoteWorkspaceControlPlaneFamily,
        Self::AiGatewayFamily,
        Self::TelemetryOrSupportIngestFamily,
    ];
}

/// Closed meter-family vocabulary, re-exported from the metering contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeterFamily {
    /// Profile or settings sync meter family.
    ProfileOrSettingsSyncMeterFamily,
    /// Collaboration relay meter family.
    CollaborationRelayMeterFamily,
    /// Remote workspace control-plane meter family.
    RemoteWorkspaceControlPlaneMeterFamily,
    /// AI gateway meter family.
    AiGatewayMeterFamily,
    /// Registry or mirror meter family.
    RegistryOrMirrorMeterFamily,
    /// Support ingest meter family.
    SupportIngestMeterFamily,
}

impl MeterFamily {
    /// Every meter family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProfileOrSettingsSyncMeterFamily,
        Self::CollaborationRelayMeterFamily,
        Self::RemoteWorkspaceControlPlaneMeterFamily,
        Self::AiGatewayMeterFamily,
        Self::RegistryOrMirrorMeterFamily,
        Self::SupportIngestMeterFamily,
    ];
}

/// Closed service-id vocabulary, re-exported from the SLO rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceId {
    /// Managed settings sync.
    ManagedSettingsSync,
    /// Managed marketplace.
    ManagedMarketplace,
    /// Managed AI broker.
    ManagedAiBroker,
    /// Managed relay (collaboration and remote-agent transport).
    ManagedRelay,
    /// Managed catalog.
    ManagedCatalog,
    /// Managed telemetry sink.
    ManagedTelemetrySink,
    /// Managed support export.
    ManagedSupportExport,
    /// Managed entitlement and usage export surface.
    ManagedEntitlementUsage,
    /// Managed offboarding export.
    ManagedOffboardingExport,
}

/// Closed meter-unit vocabulary, re-exported from the quota-state contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeterUnit {
    /// AI tokens.
    Tokens,
    /// Bytes stored.
    BytesStored,
    /// Participant minutes.
    ParticipantMinutes,
    /// Download count.
    DownloadCount,
    /// Workspace hours.
    WorkspaceHours,
    /// Support-bundle count.
    SupportBundleCount,
}

/// Closed aggregation-window vocabulary, re-exported from the quota-state contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationWindow {
    /// Calendar month, UTC.
    CalendarMonthUtc,
    /// Rolling 24 hours.
    #[serde(rename = "rolling_24h")]
    Rolling24h,
    /// Rolling 30 days.
    #[serde(rename = "rolling_30d")]
    Rolling30d,
    /// Contract term.
    ContractTerm,
}

/// Whether a lane must carry an as-of measurement time with any exposed usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AsOfTimeRequirement {
    /// An as-of time is always required on this lane.
    Required,
    /// An as-of time is required whenever a usage or spend number is shown.
    RequiredWhenUsageShown,
    /// The lane never exposes a metered number, so an as-of time does not apply.
    NotApplicable,
}

/// Closed scope-owner vocabulary. Personal, workspace, and organization scopes
/// never collapse into one chargeback total.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeOwner {
    /// The individual user.
    Personal,
    /// A workspace.
    Workspace,
    /// An organization.
    Organization,
    /// A tenant.
    Tenant,
    /// A bring-your-own-key external account.
    ByokExternal,
}

/// Closed fail-posture vocabulary, re-exported from the operating-mode contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailPosture {
    /// The managed action fails closed; only that managed action stops.
    FailClosedManagedOnly,
    /// The lane fails open to the local-safe path.
    FailOpenLocalSafe,
    /// The lane fails open to the local-safe path with a visible label.
    FailOpenLocalSafeWithLabel,
    /// The lane pauses pending a boundary recheck.
    BoundaryRecheckRequired,
}

/// Closed forecast-confidence vocabulary, re-exported from the metering contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastConfidence {
    /// Authoritative forecast.
    ForecastAuthoritative,
    /// Best-effort local forecast.
    ForecastBestEffortLocal,
    /// No forecast available.
    ForecastUnavailable,
    /// Forecast suppressed by policy.
    ForecastPolicySuppressed,
    /// Forecast not applicable.
    ForecastNotApplicable,
}

/// Closed grace-period right vocabulary, re-exported from the account/seat/exit
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GracePeriodRight {
    /// Managed actions stay admissible per the grace window.
    ManagedAdmissiblePerGrace,
    /// Bounded artifacts may be exported before suspension.
    ExportBeforeSuspend,
    /// The managed surface becomes read-only during grace.
    ReadOnlyDuringGrace,
    /// The offboarding export stays admissible during grace.
    OffboardingExportAdmissible,
}

/// Closed export-guarantee vocabulary, re-exported from the metering contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportGuarantee {
    /// Bounded export parity in CSV and JSON.
    ParityWithCsvAndJson,
    /// Bounded export parity in JSON only.
    ParityWithJsonOnly,
    /// Bounded export parity in CSV only.
    ParityWithCsvOnly,
    /// A manifest only, with no per-row export.
    ManifestOnlyNoRowExport,
    /// No documented row export; the local path is the export of record.
    NoExportDocumentedLocalOnly,
}

/// Closed entitlement-state vocabulary, re-exported from the metering contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementState {
    /// Entitlement active.
    EntitlementActive,
    /// Entitlement within a grace window.
    EntitlementInGrace,
    /// Entitlement expired.
    EntitlementExpired,
    /// Entitlement suspended by an admin or policy.
    EntitlementSuspendedAdmin,
    /// Entitlement pending a recheck.
    EntitlementPendingRecheck,
    /// No managed entitlement applies.
    EntitlementNotApplicable,
}

/// The locked, user-visible managed-state vocabulary.
///
/// These ten tokens are the closed set a surface may render for the managed
/// lanes. They are exactly the vocabulary the contract freezes — `signed in`,
/// `local only`, `reauth required`, `managed blocked`, `grace period`, `seat
/// removed`, `plan downgrade`, `org switched`, `forecast threshold`, and `meter
/// stale` — and the four that name distinct loss conditions never collapse into
/// one generic account error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedStateClass {
    /// Signed in with an active managed account.
    SignedIn,
    /// No managed account; local-only operation.
    LocalOnly,
    /// A managed reauthentication is required before managed actions resume.
    ReauthRequired,
    /// Managed actions are blocked by admin, policy, or provider posture.
    ManagedBlocked,
    /// A typed grace window is open.
    GracePeriod,
    /// The user's seat was removed, reclaimed, or deprovisioned.
    SeatRemoved,
    /// The plan was downgraded; managed actions narrow to the plan floor.
    PlanDowngrade,
    /// The account or org was switched or transferred; managed scope is rebinding.
    OrgSwitched,
    /// A metered family is approaching or crossed its forecast threshold.
    ForecastThreshold,
    /// The meter or rating data is stale; the managed number cannot be confirmed now.
    MeterStale,
}

impl ManagedStateClass {
    /// Every managed-state token, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::SignedIn,
        Self::LocalOnly,
        Self::ReauthRequired,
        Self::ManagedBlocked,
        Self::GracePeriod,
        Self::SeatRemoved,
        Self::PlanDowngrade,
        Self::OrgSwitched,
        Self::ForecastThreshold,
        Self::MeterStale,
    ];

    /// The marketed-claim cap this state imposes when it is the active state.
    ///
    /// `signed in` imposes no cap; `local only`, `managed blocked`, and `seat
    /// removed` cap to the local-safe-only claim; the remaining states cap to
    /// the narrowed claim.
    pub const fn claim_cap(self) -> MarketedClaim {
        match self {
            Self::SignedIn => MarketedClaim::ManagedFull,
            Self::LocalOnly | Self::ManagedBlocked | Self::SeatRemoved => {
                MarketedClaim::LocalSafeOnly
            }
            Self::ReauthRequired
            | Self::GracePeriod
            | Self::PlanDowngrade
            | Self::OrgSwitched
            | Self::ForecastThreshold
            | Self::MeterStale => MarketedClaim::ManagedNarrowed,
        }
    }
}

/// Closed posture-origin vocabulary, re-exported from the account/seat/exit
/// contract, so a narrowing can always be cited back to one shared origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostureOrigin {
    /// The seat.
    Seat,
    /// The plan.
    Plan,
    /// A policy.
    Policy,
    /// The org.
    Org,
    /// The account.
    Account,
    /// The managed provider.
    ManagedProvider,
    /// The workspace lifecycle.
    WorkspaceLifecycle,
    /// The metering quota.
    MeteringQuota,
    /// No managed account; local-only operation.
    LocalOnlyNoManagedAccount,
}

/// The marketed claim a managed lane may publish after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketedClaim {
    /// The full managed claim stands.
    ManagedFull,
    /// The managed claim is narrowed but managed work continues in a reduced form.
    ManagedNarrowed,
    /// Only the local-safe baseline is claimed; managed work has paused.
    LocalSafeOnly,
}

impl MarketedClaim {
    /// Strength rank used to take the weaker of two claims.
    const fn rank(self) -> u8 {
        match self {
            Self::LocalSafeOnly => 0,
            Self::ManagedNarrowed => 1,
            Self::ManagedFull => 2,
        }
    }

    /// Returns the weaker (more-narrowed) of two claims.
    fn weaker(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }
}

/// Closed consumer-surface vocabulary that must project the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Account, seat, and usage surfaces.
    AccountSurface,
    /// Diagnostics and service-health surfaces.
    Diagnostics,
    /// Help/About truth surface.
    HelpAbout,
    /// Support and admin export packets.
    SupportAdminPacket,
    /// Claim and public-truth narrowing automation.
    ClaimPublicTruthAutomation,
}

impl ConsumerSurface {
    /// Every consumer surface the matrix must reach.
    pub const ALL: [Self; 5] = [
        Self::AccountSurface,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::SupportAdminPacket,
        Self::ClaimPublicTruthAutomation,
    ];
}

/// One frozen managed-lane row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedLaneRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable lane identifier.
    pub lane_id: String,
    /// Reviewable lane title.
    pub title: String,
    /// Reviewable lane summary.
    pub summary: String,
    /// Service family the lane resolves through.
    pub service_family: ServiceFamily,
    /// Meter family the lane is measured by.
    pub meter_family: MeterFamily,
    /// Service ids the lane resolves through.
    pub linked_service_ids: Vec<ServiceId>,
    /// Meter unit.
    pub meter_unit: MeterUnit,
    /// Aggregation window.
    pub aggregation_window: AggregationWindow,
    /// As-of-time requirement for any exposed usage.
    pub as_of_time_requirement: AsOfTimeRequirement,
    /// The scope that owns the lane's quota and chargeback.
    pub scope_owner: ScopeOwner,
    /// The distinct chargeback scopes the lane offers; never collapsed to one total.
    pub chargeback_scope_offers: Vec<ScopeOwner>,
    /// Fail posture when the managed lane cannot be bounded safely.
    pub fail_posture: FailPosture,
    /// Forecast confidence for the lane's usage projection.
    pub forecast_confidence: ForecastConfidence,
    /// Grace-period rights the lane preserves.
    pub grace_period_rights: Vec<GracePeriodRight>,
    /// Bounded export guarantee for the lane's usage summary.
    pub export_guarantee: ExportGuarantee,
    /// Non-empty local-safe baseline that always continues when the lane narrows.
    pub local_safe_baseline: Vec<String>,
    /// Managed-only actions that pause when the lane narrows.
    pub blocked_managed_only_actions: Vec<String>,
    /// The managed states that can affect this lane.
    pub applicable_managed_states: Vec<ManagedStateClass>,
    /// The marketed claim the lane declares before narrowing.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the active managed state's cap is applied.
    pub effective_marketed_claim: MarketedClaim,
    /// The managed states that narrowed the lane below its declared claim.
    pub narrowing_reasons: Vec<ManagedStateClass>,
    /// Short recovery cue. Present (non-null) when the lane is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_cue: Option<String>,
}

impl ManagedLaneRow {
    /// True when the lane still publishes its full managed claim.
    pub fn backs_full_managed_claim(&self) -> bool {
        self.effective_marketed_claim == MarketedClaim::ManagedFull
    }
}

/// One frozen managed-state row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedStateRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The managed-state token this row freezes.
    pub managed_state: ManagedStateClass,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary.
    pub summary: String,
    /// The frozen entitlement state this managed state binds to.
    pub linked_entitlement_state: EntitlementState,
    /// The posture origin the state is cited back to.
    pub posture_origin: PostureOrigin,
    /// The marketed-claim cap this state imposes when active.
    pub claim_cap: MarketedClaim,
    /// The other managed states this state must never collapse with.
    pub must_not_collapse_with: Vec<ManagedStateClass>,
    /// Always true: the state never blocks the local core.
    pub local_safe_guaranteed: bool,
    /// The disclosure the surface must render in this state.
    pub required_disclosure: String,
    /// The next user-visible step in this state.
    pub recovery_cue: String,
}

/// One consumer surface bound to the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The consumer surface that projects the matrix.
    pub consumer_surface: ConsumerSurface,
    /// The lane ids this surface resolves through.
    pub bound_lane_ids: Vec<String>,
    /// Always true: the surface projects the lane's effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommercialControlPlaneInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of managed-lane rows.
    pub lane_count: usize,
    /// Number of managed-state rows.
    pub managed_state_count: usize,
    /// Number of consumer bindings.
    pub consumer_binding_count: usize,
    /// Number of distinct service families covered.
    pub service_families_covered: usize,
    /// Number of distinct meter families covered.
    pub meter_families_covered: usize,
    /// True when all ten managed-state tokens appear exactly once.
    pub managed_state_vocab_complete: bool,
    /// True when all five consumer surfaces are bound.
    pub consumer_surface_coverage_complete: bool,
    /// True when every lane carries a non-empty local-safe baseline.
    pub all_lanes_local_safe_backed: bool,
    /// Number of lanes still backing the full managed claim.
    pub effective_full_lane_count: usize,
    /// Number of lanes narrowed to a reduced managed claim.
    pub narrowed_lane_count: usize,
    /// Number of lanes narrowed to the local-safe-only claim.
    pub local_safe_only_lane_count: usize,
    /// The active managed state, when one has been applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_managed_state: Option<ManagedStateClass>,
}

/// The frozen commercial-control-plane matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommercialControlPlaneMatrix {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the matrix content.
    pub matrix_revision: u32,
    /// Reviewable matrix title.
    pub title: String,
    /// Reviewable matrix summary.
    pub summary: String,
    /// Source schema and contract refs the matrix cites.
    pub source_refs: Vec<String>,
    /// The active managed state, when one has been applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_managed_state: Option<ManagedStateClass>,
    /// The managed-lane rows.
    pub lanes: Vec<ManagedLaneRow>,
    /// The managed-state rows.
    pub managed_states: Vec<ManagedStateRow>,
    /// The consumer bindings.
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// The recomputed inspection block.
    pub inspection: CommercialControlPlaneInspection,
}

impl CommercialControlPlaneMatrix {
    /// Applies a single active managed state, narrowing every applicable lane.
    ///
    /// Every lane whose [`ManagedLaneRow::applicable_managed_states`] contains
    /// `state` has its effective marketed claim recomputed from the state's
    /// [`ManagedStateClass::claim_cap`], its narrowing reasons updated, and its
    /// recovery cue set; the inspection block is recomputed. The local-safe
    /// baseline is never removed, so the local core stays available.
    pub fn apply_managed_state(&mut self, state: ManagedStateClass) {
        self.active_managed_state = Some(state);
        for lane in &mut self.lanes {
            let derived = derive_lane_effective(
                lane.declared_marketed_claim,
                &lane.applicable_managed_states,
                Some(state),
            );
            lane.effective_marketed_claim = derived.effective;
            lane.narrowing_reasons = derived.reasons;
            lane.recovery_cue = if derived.effective == lane.declared_marketed_claim {
                None
            } else {
                Some(narrowing_recovery_cue(state))
            };
        }
        self.inspection = CommercialControlPlaneInspection::derive(
            &self.lanes,
            &self.managed_states,
            &self.consumer_bindings,
            self.active_managed_state,
        );
    }

    /// Serializes the matrix as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the matrix cannot be serialized, which a validated matrix never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("matrix serializes to JSON")
    }

    /// Validates the matrix and recomputes every derived value.
    ///
    /// Returns an empty vector when the matrix is internally consistent.
    /// Otherwise returns one [`CommercialControlPlaneViolation`] per failed
    /// invariant: a wrong record kind or schema version, a missing identifier, a
    /// duplicate lane, an incomplete service- or meter-family set, a missing
    /// managed-state token, a collapsed chargeback scope, an empty local-safe
    /// baseline, a stored effective claim that does not match the recomputation,
    /// a missing recovery cue on a narrowed lane, an unbound consumer surface, or
    /// a stale inspection block.
    pub fn validate(&self) -> Vec<CommercialControlPlaneViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(CommercialControlPlaneViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != MATRIX_RECORD_KIND {
            push("record_kind", "matrix record_kind is wrong");
        }
        if self.schema_version != COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION {
            push("schema_version", "matrix schema_version is wrong");
        }
        if self.matrix_id.trim().is_empty() {
            push("matrix_id", "matrix_id must be non-empty");
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
        if self.matrix_revision == 0 {
            push("matrix_revision", "matrix_revision must be at least 1");
        }
        if !self
            .source_refs
            .iter()
            .any(|entry| entry == COMMERCIAL_CONTROL_PLANE_SCHEMA_REF)
        {
            push("source_refs", "matrix must cite its boundary schema");
        }
        if self.lanes.is_empty() {
            push("lanes", "matrix must contain at least one managed lane");
        }

        let mut lane_ids = BTreeSet::new();
        for lane in &self.lanes {
            self.validate_lane(lane, &mut push);
            if !lane_ids.insert(lane.lane_id.as_str()) {
                push("lanes", "lane_id values must be unique");
            }
        }

        // Every claimed service family and meter family must appear.
        for family in ServiceFamily::ALL {
            if !self.lanes.iter().any(|lane| lane.service_family == family) {
                push("lanes", "every service family must carry a managed lane");
                break;
            }
        }
        for family in MeterFamily::ALL {
            if !self.lanes.iter().any(|lane| lane.meter_family == family) {
                push("lanes", "every meter family must carry a managed lane");
                break;
            }
        }

        self.validate_managed_states(&mut push);
        self.validate_consumer_bindings(&mut push);

        let derived = CommercialControlPlaneInspection::derive(
            &self.lanes,
            &self.managed_states,
            &self.consumer_bindings,
            self.active_managed_state,
        );
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed matrix",
            );
        }

        violations
    }

    fn validate_lane(&self, lane: &ManagedLaneRow, push: &mut impl FnMut(&str, &str)) {
        if lane.record_kind != LANE_RECORD_KIND {
            push("lane.record_kind", "lane record_kind is wrong");
        }
        if lane.schema_version != COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION {
            push("lane.schema_version", "lane schema_version is wrong");
        }
        if lane.lane_id.trim().is_empty() {
            push("lane.lane_id", "lane_id must be non-empty");
        }
        if lane.title.trim().is_empty() {
            push("lane.title", "lane title must be non-empty");
        }
        if lane.summary.trim().is_empty() {
            push("lane.summary", "lane summary must be non-empty");
        }
        if lane.linked_service_ids.is_empty() {
            push(
                "lane.linked_service_ids",
                "lane must resolve through at least one service id",
            );
        }
        // The local core is never blocked: every lane keeps a non-empty
        // local-safe baseline.
        if lane.local_safe_baseline.is_empty()
            || lane.local_safe_baseline.iter().any(|s| s.trim().is_empty())
        {
            push(
                "lane.local_safe_baseline",
                "every lane must keep a non-empty local-safe baseline",
            );
        }
        if lane.grace_period_rights.is_empty() {
            push(
                "lane.grace_period_rights",
                "lane must name at least one grace-period right",
            );
        }
        // A metered lane must promise an as-of time so a number is never shown
        // without its measurement time.
        if lane.as_of_time_requirement == AsOfTimeRequirement::NotApplicable {
            push(
                "lane.as_of_time_requirement",
                "a metered lane must require an as-of time",
            );
        }
        // Personal, workspace, and organization scopes never collapse into one total.
        if lane.chargeback_scope_offers.is_empty() {
            push(
                "lane.chargeback_scope_offers",
                "lane must offer at least one chargeback scope",
            );
        }
        let mut seen_scopes = BTreeSet::new();
        for scope in &lane.chargeback_scope_offers {
            if !seen_scopes.insert(*scope) {
                push(
                    "lane.chargeback_scope_offers",
                    "chargeback scopes must be distinct, not collapsed",
                );
            }
        }
        // The applicable managed states must be a distinct subset of the vocabulary.
        let mut seen_states = BTreeSet::new();
        for state in &lane.applicable_managed_states {
            if !seen_states.insert(*state) {
                push(
                    "lane.applicable_managed_states",
                    "applicable managed states must be distinct",
                );
            }
        }

        // Recompute the lane's effective claim and reasons.
        let derived = derive_lane_effective(
            lane.declared_marketed_claim,
            &lane.applicable_managed_states,
            self.active_managed_state,
        );
        if derived.effective != lane.effective_marketed_claim {
            push(
                "lane.effective_marketed_claim",
                "stored effective claim does not match the recomputed lane state",
            );
        }
        let stored_reasons: BTreeSet<ManagedStateClass> =
            lane.narrowing_reasons.iter().copied().collect();
        let derived_reasons: BTreeSet<ManagedStateClass> =
            derived.reasons.iter().copied().collect();
        if stored_reasons != derived_reasons {
            push(
                "lane.narrowing_reasons",
                "stored narrowing reasons do not match the recomputed lane state",
            );
        }
        // A narrowed lane must carry a recovery cue; a full lane must not.
        let narrowed = derived.effective != lane.declared_marketed_claim;
        match (&lane.recovery_cue, narrowed) {
            (None, true) => push(
                "lane.recovery_cue",
                "a narrowed lane must carry a recovery cue",
            ),
            (Some(cue), _) if cue.trim().is_empty() => {
                push("lane.recovery_cue", "recovery cue must be non-empty")
            }
            _ => {}
        }
    }

    fn validate_managed_states(&self, push: &mut impl FnMut(&str, &str)) {
        let mut seen = BTreeSet::new();
        for row in &self.managed_states {
            if row.record_kind != MANAGED_STATE_RECORD_KIND {
                push(
                    "managed_state.record_kind",
                    "managed-state record_kind is wrong",
                );
            }
            if row.schema_version != COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION {
                push(
                    "managed_state.schema_version",
                    "managed-state schema_version is wrong",
                );
            }
            if !seen.insert(row.managed_state) {
                push(
                    "managed_states",
                    "each managed-state token must appear exactly once",
                );
            }
            if row.title.trim().is_empty() || row.summary.trim().is_empty() {
                push(
                    "managed_state",
                    "managed-state title and summary must be non-empty",
                );
            }
            if row.required_disclosure.trim().is_empty() {
                push(
                    "managed_state.required_disclosure",
                    "managed-state required_disclosure must be non-empty",
                );
            }
            if row.recovery_cue.trim().is_empty() {
                push(
                    "managed_state.recovery_cue",
                    "managed-state recovery_cue must be non-empty",
                );
            }
            // The local core is never blocked.
            if !row.local_safe_guaranteed {
                push(
                    "managed_state.local_safe_guaranteed",
                    "every managed state must guarantee the local-safe baseline",
                );
            }
            // The stored cap must equal the locked cap for the token.
            if row.claim_cap != row.managed_state.claim_cap() {
                push(
                    "managed_state.claim_cap",
                    "stored claim cap does not match the locked managed-state cap",
                );
            }
            // A state never lists itself as something it must not collapse with.
            if row.must_not_collapse_with.contains(&row.managed_state) {
                push(
                    "managed_state.must_not_collapse_with",
                    "a managed state cannot be distinct from itself",
                );
            }
        }
        // The full vocabulary must be present.
        if seen.len() != ManagedStateClass::ALL.len() {
            push(
                "managed_states",
                "the matrix must freeze all ten managed-state tokens",
            );
        }
        // The four loss conditions must each stay distinct from the other three.
        let distinct_loss = [
            ManagedStateClass::SeatRemoved,
            ManagedStateClass::OrgSwitched,
            ManagedStateClass::GracePeriod,
            ManagedStateClass::ReauthRequired,
        ];
        for row in &self.managed_states {
            if !distinct_loss.contains(&row.managed_state) {
                continue;
            }
            for other in distinct_loss {
                if other != row.managed_state && !row.must_not_collapse_with.contains(&other) {
                    push(
                        "managed_state.must_not_collapse_with",
                        "seat loss, org switch, grace, and sign-in failure must stay distinct",
                    );
                }
            }
        }
    }

    fn validate_consumer_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let lane_ids: BTreeSet<&str> = self.lanes.iter().map(|l| l.lane_id.as_str()).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.consumer_bindings {
            if binding.record_kind != CONSUMER_BINDING_RECORD_KIND {
                push(
                    "consumer_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION {
                push(
                    "consumer_binding.schema_version",
                    "binding schema_version is wrong",
                );
            }
            if binding.binding_id.trim().is_empty() {
                push(
                    "consumer_binding.binding_id",
                    "binding_id must be non-empty",
                );
            }
            if !binding_ids.insert(binding.binding_id.as_str()) {
                push("consumer_bindings", "binding_id values must be unique");
            }
            if binding.summary.trim().is_empty() {
                push(
                    "consumer_binding.summary",
                    "binding summary must be non-empty",
                );
            }
            if !binding.projects_effective_claim {
                push(
                    "consumer_binding.projects_effective_claim",
                    "a consumer surface must project the effective claim, never a stronger one",
                );
            }
            if binding.bound_lane_ids.is_empty() {
                push(
                    "consumer_binding.bound_lane_ids",
                    "a binding must resolve through at least one lane",
                );
            }
            for lane_ref in &binding.bound_lane_ids {
                if !lane_ids.contains(lane_ref.as_str()) {
                    push(
                        "consumer_binding.bound_lane_ids",
                        "binding lane ref must resolve to a managed lane",
                    );
                }
            }
        }
        // Every consumer surface must be bound.
        for surface in ConsumerSurface::ALL {
            if !self
                .consumer_bindings
                .iter()
                .any(|b| b.consumer_surface == surface)
            {
                push(
                    "consumer_bindings",
                    "account, diagnostics, Help/About, support/admin, and claim automation must all bind",
                );
                break;
            }
        }
    }
}

impl CommercialControlPlaneInspection {
    fn derive(
        lanes: &[ManagedLaneRow],
        managed_states: &[ManagedStateRow],
        consumer_bindings: &[ConsumerBinding],
        active_managed_state: Option<ManagedStateClass>,
    ) -> Self {
        let service_families: BTreeSet<ServiceFamily> =
            lanes.iter().map(|l| l.service_family).collect();
        let meter_families: BTreeSet<MeterFamily> = lanes.iter().map(|l| l.meter_family).collect();
        let state_tokens: BTreeSet<ManagedStateClass> =
            managed_states.iter().map(|r| r.managed_state).collect();
        let surfaces: BTreeSet<ConsumerSurface> = consumer_bindings
            .iter()
            .map(|b| b.consumer_surface)
            .collect();

        let effective_full_lane_count = lanes
            .iter()
            .filter(|l| l.effective_marketed_claim == MarketedClaim::ManagedFull)
            .count();
        let local_safe_only_lane_count = lanes
            .iter()
            .filter(|l| l.effective_marketed_claim == MarketedClaim::LocalSafeOnly)
            .count();
        let narrowed_lane_count = lanes
            .iter()
            .filter(|l| l.effective_marketed_claim != l.declared_marketed_claim)
            .count();

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION,
            lane_count: lanes.len(),
            managed_state_count: managed_states.len(),
            consumer_binding_count: consumer_bindings.len(),
            service_families_covered: service_families.len(),
            meter_families_covered: meter_families.len(),
            managed_state_vocab_complete: state_tokens.len() == ManagedStateClass::ALL.len(),
            consumer_surface_coverage_complete: surfaces.len() == ConsumerSurface::ALL.len(),
            all_lanes_local_safe_backed: lanes.iter().all(|l| !l.local_safe_baseline.is_empty()),
            effective_full_lane_count,
            narrowed_lane_count,
            local_safe_only_lane_count,
            active_managed_state,
        }
    }
}

/// One failed matrix invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialControlPlaneViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for CommercialControlPlaneViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in matrix cannot be read or validated.
#[derive(Debug)]
pub enum CommercialControlPlaneError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in matrix failed validation.
    Validation(Vec<CommercialControlPlaneViolation>),
}

impl fmt::Display for CommercialControlPlaneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "commercial-control-plane matrix parse error: {err}"),
            Self::Validation(violations) => {
                write!(
                    f,
                    "commercial-control-plane matrix failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl std::error::Error for CommercialControlPlaneError {}

struct DerivedLaneState {
    effective: MarketedClaim,
    reasons: Vec<ManagedStateClass>,
}

fn derive_lane_effective(
    declared: MarketedClaim,
    applicable_states: &[ManagedStateClass],
    active_state: Option<ManagedStateClass>,
) -> DerivedLaneState {
    let mut effective = declared;
    let mut reasons = Vec::new();
    if let Some(state) = active_state {
        if applicable_states.contains(&state) {
            let cap = state.claim_cap();
            effective = effective.weaker(cap);
            if cap.rank() < declared.rank() {
                reasons.push(state);
            }
        }
    }
    DerivedLaneState { effective, reasons }
}

fn narrowing_recovery_cue(state: ManagedStateClass) -> String {
    match state {
        ManagedStateClass::SignedIn => "Managed actions continue; no recovery needed.",
        ManagedStateClass::LocalOnly => {
            "Sign in to a managed account to enable managed actions; local work continues now."
        }
        ManagedStateClass::ReauthRequired => {
            "Reauthenticate to resume managed actions; local work continues now."
        }
        ManagedStateClass::ManagedBlocked => {
            "Review the account or policy hold to restore managed actions; local work continues now."
        }
        ManagedStateClass::GracePeriod => {
            "Export bounded artifacts before the grace window closes; local work continues now."
        }
        ManagedStateClass::SeatRemoved => {
            "Ask an admin to restore the seat to resume managed actions; local work continues now."
        }
        ManagedStateClass::PlanDowngrade => {
            "Managed actions are on the plan floor; upgrade the plan to widen them. Local work continues now."
        }
        ManagedStateClass::OrgSwitched => {
            "Managed scope is rebinding to the new org; local work continues now."
        }
        ManagedStateClass::ForecastThreshold => {
            "Usage is approaching the budget; raise the budget or wait for the window reset. Local work continues now."
        }
        ManagedStateClass::MeterStale => {
            "The metered number is stale and labeled; managed actions resume when the meter refreshes. Local work continues now."
        }
    }
    .to_owned()
}

/// Reads and validates the checked-in stable matrix.
///
/// This is the canonical reader: account surfaces, diagnostics, Help/About,
/// support/admin packets, and claim/public-truth automation call it to ingest
/// the matrix rather than cloning status text.
///
/// # Errors
///
/// Returns [`CommercialControlPlaneError`] when the checked-in packet fails to
/// parse or fails validation.
pub fn current_stable_commercial_control_plane_matrix(
) -> Result<CommercialControlPlaneMatrix, CommercialControlPlaneError> {
    let matrix: CommercialControlPlaneMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-commercial-control-plane.json"
    )))
    .map_err(CommercialControlPlaneError::Parse)?;
    let violations = matrix.validate();
    if violations.is_empty() {
        Ok(matrix)
    } else {
        Err(CommercialControlPlaneError::Validation(violations))
    }
}

/// Canonical source refs every matrix export carries.
pub fn canonical_source_refs() -> Vec<String> {
    vec![
        COMMERCIAL_CONTROL_PLANE_SCHEMA_REF.to_owned(),
        COMMERCIAL_CONTROL_PLANE_DOC_REF.to_owned(),
        "docs/service/metering_and_chargeback_contract.md".to_owned(),
        "docs/service/operating_mode_and_capacity_contract.md".to_owned(),
        "docs/managed/account_seat_plan_and_exit_contract.md".to_owned(),
        "docs/managed/metering_and_usage_export_contract.md".to_owned(),
    ]
}

// The lane row freezes many fixed columns; a wide builder keeps the canonical
// matrix readable as one table.
#[allow(clippy::too_many_arguments)]
fn lane(
    lane_id: &str,
    title: &str,
    summary: &str,
    service_family: ServiceFamily,
    meter_family: MeterFamily,
    linked_service_ids: &[ServiceId],
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    scope_owner: ScopeOwner,
    chargeback_scope_offers: &[ScopeOwner],
    fail_posture: FailPosture,
    forecast_confidence: ForecastConfidence,
    grace_period_rights: &[GracePeriodRight],
    export_guarantee: ExportGuarantee,
    local_safe_baseline: &[&str],
    blocked_managed_only_actions: &[&str],
) -> ManagedLaneRow {
    ManagedLaneRow {
        record_kind: LANE_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION,
        lane_id: lane_id.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        service_family,
        meter_family,
        linked_service_ids: linked_service_ids.to_vec(),
        meter_unit,
        aggregation_window,
        as_of_time_requirement: AsOfTimeRequirement::Required,
        scope_owner,
        chargeback_scope_offers: chargeback_scope_offers.to_vec(),
        fail_posture,
        forecast_confidence,
        grace_period_rights: grace_period_rights.to_vec(),
        export_guarantee,
        local_safe_baseline: local_safe_baseline
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        blocked_managed_only_actions: blocked_managed_only_actions
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        applicable_managed_states: ManagedStateClass::ALL.to_vec(),
        declared_marketed_claim: MarketedClaim::ManagedFull,
        effective_marketed_claim: MarketedClaim::ManagedFull,
        narrowing_reasons: Vec::new(),
        recovery_cue: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn managed_state_row(
    managed_state: ManagedStateClass,
    title: &str,
    summary: &str,
    linked_entitlement_state: EntitlementState,
    posture_origin: PostureOrigin,
    must_not_collapse_with: &[ManagedStateClass],
    required_disclosure: &str,
    recovery_cue: &str,
) -> ManagedStateRow {
    ManagedStateRow {
        record_kind: MANAGED_STATE_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION,
        managed_state,
        title: title.to_owned(),
        summary: summary.to_owned(),
        linked_entitlement_state,
        posture_origin,
        claim_cap: managed_state.claim_cap(),
        must_not_collapse_with: must_not_collapse_with.to_vec(),
        local_safe_guaranteed: true,
        required_disclosure: required_disclosure.to_owned(),
        recovery_cue: recovery_cue.to_owned(),
    }
}

fn binding(
    binding_id: &str,
    consumer_surface: ConsumerSurface,
    bound_lane_ids: &[&str],
    summary: &str,
) -> ConsumerBinding {
    ConsumerBinding {
        record_kind: CONSUMER_BINDING_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        consumer_surface,
        bound_lane_ids: bound_lane_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        summary: summary.to_owned(),
    }
}

/// Stable identifier for the checked-in matrix.
pub const STABLE_MATRIX_ID: &str = "commercial-control-plane:stable:0001";

/// Stable title for the checked-in matrix.
pub const STABLE_MATRIX_TITLE: &str =
    "Managed-lane entitlement, meter-family, chargeback-scope, org-switch, and grace-period matrix";

/// Deterministic timestamp for the checked-in matrix.
pub const STABLE_MATRIX_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in matrix.
pub const STABLE_MATRIX_REVISION: u32 = 1;

/// Builds the checked-in matrix with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_commercial_control_plane_matrix() -> CommercialControlPlaneMatrix {
    canonical_commercial_control_plane_matrix(
        STABLE_MATRIX_ID.to_owned(),
        STABLE_MATRIX_TITLE.to_owned(),
        STABLE_MATRIX_GENERATED_AT.to_owned(),
        STABLE_MATRIX_REVISION,
    )
}

/// Builds the canonical, frozen commercial-control-plane matrix.
///
/// The matrix freezes one lane per claimed managed lane, the full ten-token
/// managed-state vocabulary, and one binding per consumer surface. All lanes
/// start at their full managed claim with no active managed state; callers use
/// [`CommercialControlPlaneMatrix::apply_managed_state`] to exercise narrowing.
pub fn canonical_commercial_control_plane_matrix(
    matrix_id: String,
    title: String,
    generated_at: String,
    matrix_revision: u32,
) -> CommercialControlPlaneMatrix {
    let lanes = vec![
        lane(
            "managed_lane.ai_gateway",
            "Managed AI gateway",
            "Token spend on the managed AI broker is metered per organization; direct and bring-your-own-key AI routes continue when the managed lane narrows.",
            ServiceFamily::AiGatewayFamily,
            MeterFamily::AiGatewayMeterFamily,
            &[ServiceId::ManagedAiBroker],
            MeterUnit::Tokens,
            AggregationWindow::CalendarMonthUtc,
            ScopeOwner::Organization,
            &[ScopeOwner::Personal, ScopeOwner::Workspace, ScopeOwner::Organization],
            FailPosture::FailOpenLocalSafeWithLabel,
            ForecastConfidence::ForecastAuthoritative,
            &[GracePeriodRight::ManagedAdmissiblePerGrace, GracePeriodRight::ExportBeforeSuspend],
            ExportGuarantee::ParityWithCsvAndJson,
            &[
                "Direct and bring-your-own-key AI routes keep running.",
                "Local editing, search, and Git are unaffected.",
            ],
            &["New managed-broker inference once the monthly token budget is exhausted."],
        ),
        lane(
            "managed_lane.settings_sync",
            "Managed settings sync",
            "Stored bytes on the managed settings-sync store are metered per workspace; local settings and files stay authoritative when the lane narrows.",
            ServiceFamily::SyncFamily,
            MeterFamily::ProfileOrSettingsSyncMeterFamily,
            &[ServiceId::ManagedSettingsSync],
            MeterUnit::BytesStored,
            AggregationWindow::Rolling30d,
            ScopeOwner::Workspace,
            &[ScopeOwner::Personal, ScopeOwner::Workspace, ScopeOwner::Organization],
            FailPosture::FailOpenLocalSafe,
            ForecastConfidence::ForecastBestEffortLocal,
            &[GracePeriodRight::ManagedAdmissiblePerGrace, GracePeriodRight::ExportBeforeSuspend],
            ExportGuarantee::ParityWithJsonOnly,
            &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the lane clears.",
            ],
            &["Pushing new settings snapshots to the managed store once storage is exhausted."],
        ),
        lane(
            "managed_lane.companion_relay",
            "Companion relay",
            "Participant minutes on the managed relay are metered per workspace; local incident notes and offline packets continue when the relay narrows.",
            ServiceFamily::CollaborationRelayFamily,
            MeterFamily::CollaborationRelayMeterFamily,
            &[ServiceId::ManagedRelay],
            MeterUnit::ParticipantMinutes,
            AggregationWindow::Rolling24h,
            ScopeOwner::Workspace,
            &[ScopeOwner::Workspace, ScopeOwner::Organization],
            FailPosture::FailClosedManagedOnly,
            ForecastConfidence::ForecastBestEffortLocal,
            &[GracePeriodRight::ManagedAdmissiblePerGrace, GracePeriodRight::OffboardingExportAdmissible],
            ExportGuarantee::ParityWithCsvAndJson,
            &[
                "Local incident notes and offline packets continue.",
                "Desktop handoff resumes the exact local context.",
            ],
            &["Joining a live companion-follow or relay session once relay minutes are exhausted."],
        ),
        lane(
            "managed_lane.registry_mirror",
            "Registry and mirror",
            "Download count on the managed registry and mirror is metered per organization; installed extensions and local or sideloaded packages keep running when the lane narrows.",
            ServiceFamily::RegistryOrMirrorMetadataFamily,
            MeterFamily::RegistryOrMirrorMeterFamily,
            &[ServiceId::ManagedMarketplace, ServiceId::ManagedCatalog],
            MeterUnit::DownloadCount,
            AggregationWindow::CalendarMonthUtc,
            ScopeOwner::Organization,
            &[ScopeOwner::Organization, ScopeOwner::Tenant],
            FailPosture::FailOpenLocalSafe,
            ForecastConfidence::ForecastBestEffortLocal,
            &[GracePeriodRight::ManagedAdmissiblePerGrace],
            ExportGuarantee::ParityWithCsvAndJson,
            &[
                "Installed extensions keep running.",
                "Local and sideloaded packages are unaffected.",
            ],
            &["New managed-registry installs or publishes once the monthly download budget is exhausted."],
        ),
        lane(
            "managed_lane.support_ingest",
            "Support ingest",
            "Support-bundle uploads to the managed ingest sink are metered per tenant; local support bundles still generate when the lane narrows.",
            ServiceFamily::TelemetryOrSupportIngestFamily,
            MeterFamily::SupportIngestMeterFamily,
            &[ServiceId::ManagedSupportExport, ServiceId::ManagedTelemetrySink],
            MeterUnit::SupportBundleCount,
            AggregationWindow::Rolling30d,
            ScopeOwner::Tenant,
            &[ScopeOwner::Organization, ScopeOwner::Tenant],
            FailPosture::FailOpenLocalSafeWithLabel,
            ForecastConfidence::ForecastBestEffortLocal,
            &[GracePeriodRight::ExportBeforeSuspend, GracePeriodRight::OffboardingExportAdmissible],
            ExportGuarantee::ParityWithCsvAndJson,
            &[
                "Local support bundles still generate on device.",
                "Offline evidence capture continues.",
            ],
            &["Uploading new support bundles to the managed sink once the ingest budget is exhausted."],
        ),
        lane(
            "managed_lane.managed_workspace",
            "Managed workspace",
            "Remote workspace hours on the managed control plane are metered per organization; local checkout, editing, tasks, and Git continue when the remote workspace narrows.",
            ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            MeterFamily::RemoteWorkspaceControlPlaneMeterFamily,
            &[ServiceId::ManagedRelay],
            MeterUnit::WorkspaceHours,
            AggregationWindow::CalendarMonthUtc,
            ScopeOwner::Organization,
            &[ScopeOwner::Workspace, ScopeOwner::Organization],
            FailPosture::FailClosedManagedOnly,
            ForecastConfidence::ForecastAuthoritative,
            &[GracePeriodRight::ManagedAdmissiblePerGrace, GracePeriodRight::OffboardingExportAdmissible],
            ExportGuarantee::ParityWithJsonOnly,
            &[
                "Local checkout and editing continue.",
                "Local tasks and Git are unaffected when the remote workspace narrows.",
            ],
            &["Attaching or running a new remote workspace once the workspace-hour budget is exhausted."],
        ),
    ];

    let managed_states = vec![
        managed_state_row(
            ManagedStateClass::SignedIn,
            "Signed in",
            "A managed account is signed in and entitled; managed actions are admissible within seat, plan, policy, org, and provider posture.",
            EntitlementState::EntitlementActive,
            PostureOrigin::Account,
            &[],
            "Render the signed-in account scope; never imply the whole product depends on it.",
            "No action needed; managed actions are admissible.",
        ),
        managed_state_row(
            ManagedStateClass::LocalOnly,
            "Local only",
            "The install has no managed account; the local core is fully usable and managed actions do not apply.",
            EntitlementState::EntitlementNotApplicable,
            PostureOrigin::LocalOnlyNoManagedAccount,
            &[],
            "Render the local-only posture; never show a managed quota or spend number.",
            "Sign in to a managed account to enable managed actions; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::ReauthRequired,
            "Reauth required",
            "A managed reauthentication is required (signer rotation or attestation pending); managed actions pause until reauth completes.",
            EntitlementState::EntitlementPendingRecheck,
            PostureOrigin::Account,
            &[
                ManagedStateClass::SeatRemoved,
                ManagedStateClass::OrgSwitched,
                ManagedStateClass::GracePeriod,
                ManagedStateClass::ManagedBlocked,
            ],
            "Name reauth as the reason; never collapse it into a generic account error or a managed block.",
            "Reauthenticate to resume managed actions; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::ManagedBlocked,
            "Managed blocked",
            "Managed actions are blocked by admin, policy, or provider posture; the local core remains usable.",
            EntitlementState::EntitlementSuspendedAdmin,
            PostureOrigin::Policy,
            &[
                ManagedStateClass::SeatRemoved,
                ManagedStateClass::OrgSwitched,
                ManagedStateClass::GracePeriod,
                ManagedStateClass::ReauthRequired,
            ],
            "Cite the policy or provider origin; never imply local work is blocked.",
            "Review the account or policy hold to restore managed actions; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::GracePeriod,
            "Grace period",
            "A typed grace window is open; managed actions are admissible per the window and bounded artifacts can be exported before suspension.",
            EntitlementState::EntitlementInGrace,
            PostureOrigin::Account,
            &[
                ManagedStateClass::SeatRemoved,
                ManagedStateClass::OrgSwitched,
                ManagedStateClass::ReauthRequired,
                ManagedStateClass::ManagedBlocked,
            ],
            "Render the grace-window close time and the export-before-suspend path; never show grace as a hard block.",
            "Export bounded artifacts before the grace window closes; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::SeatRemoved,
            "Seat removed",
            "The user's seat was removed, reclaimed, or deprovisioned; managed actions for this seat block while local work continues.",
            EntitlementState::EntitlementSuspendedAdmin,
            PostureOrigin::Seat,
            &[
                ManagedStateClass::OrgSwitched,
                ManagedStateClass::GracePeriod,
                ManagedStateClass::ReauthRequired,
                ManagedStateClass::ManagedBlocked,
            ],
            "Cite the seat as the origin; never collapse a seat loss into an org or account error.",
            "Ask an admin to restore the seat to resume managed actions; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::PlanDowngrade,
            "Plan downgrade",
            "The plan was downgraded; managed actions narrow to the plan floor for the remainder of the term.",
            EntitlementState::EntitlementActive,
            PostureOrigin::Plan,
            &[ManagedStateClass::SeatRemoved, ManagedStateClass::OrgSwitched],
            "Cite the plan as the origin and name the plan-floor narrowing; never show it as a full block.",
            "Managed actions are on the plan floor; upgrade the plan to widen them. Local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::OrgSwitched,
            "Org switched",
            "The account or org was switched or transferred; managed scope is rebinding and the prior org's continuation is local-only for the transferor.",
            EntitlementState::EntitlementPendingRecheck,
            PostureOrigin::Org,
            &[
                ManagedStateClass::SeatRemoved,
                ManagedStateClass::GracePeriod,
                ManagedStateClass::ReauthRequired,
                ManagedStateClass::ManagedBlocked,
            ],
            "Cite the org switch and what migrates versus what stays local; never collapse it into a seat or generic account error.",
            "Managed scope is rebinding to the new org; local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::ForecastThreshold,
            "Forecast threshold",
            "A metered family is approaching or crossed its forecast threshold; managed actions stay admissible with a budget warning.",
            EntitlementState::EntitlementActive,
            PostureOrigin::MeteringQuota,
            &[ManagedStateClass::MeterStale],
            "Render the forecast with its unit, as-of time, and scope owner; never show a forecast under an unauthoritative state.",
            "Usage is approaching the budget; raise the budget or wait for the window reset. Local work continues now.",
        ),
        managed_state_row(
            ManagedStateClass::MeterStale,
            "Meter stale",
            "The meter or rating data is stale; the managed number cannot be confirmed now and is labeled stale rather than shown as live.",
            EntitlementState::EntitlementPendingRecheck,
            PostureOrigin::MeteringQuota,
            &[ManagedStateClass::ForecastThreshold],
            "Label the number stale with its last as-of time; never block local editing, search, or Git on a stale meter.",
            "The metered number is stale and labeled; managed actions resume when the meter refreshes. Local work continues now.",
        ),
    ];

    let consumer_bindings = vec![
        binding(
            "consumer.account_surface",
            ConsumerSurface::AccountSurface,
            &[
                "managed_lane.ai_gateway",
                "managed_lane.settings_sync",
                "managed_lane.companion_relay",
                "managed_lane.registry_mirror",
                "managed_lane.support_ingest",
                "managed_lane.managed_workspace",
            ],
            "The account and seat usage surface renders each lane's effective claim, unit, as-of time, and scope owner without re-deriving them.",
        ),
        binding(
            "consumer.diagnostics",
            ConsumerSurface::Diagnostics,
            &[
                "managed_lane.ai_gateway",
                "managed_lane.settings_sync",
                "managed_lane.companion_relay",
                "managed_lane.registry_mirror",
                "managed_lane.support_ingest",
                "managed_lane.managed_workspace",
            ],
            "Diagnostics and service-health surfaces project the lane fail posture and the active managed state without inventing a stronger claim.",
        ),
        binding(
            "consumer.help_about",
            ConsumerSurface::HelpAbout,
            &[
                "managed_lane.ai_gateway",
                "managed_lane.settings_sync",
                "managed_lane.registry_mirror",
            ],
            "The Help/About truth surface names which managed lanes are claimed and their local-safe baseline.",
        ),
        binding(
            "consumer.support_admin_packet",
            ConsumerSurface::SupportAdminPacket,
            &[
                "managed_lane.support_ingest",
                "managed_lane.companion_relay",
                "managed_lane.managed_workspace",
            ],
            "Support and admin export packets carry the lane's export guarantee, grace-period rights, and the posture origin of any narrowing.",
        ),
        binding(
            "consumer.claim_public_truth",
            ConsumerSurface::ClaimPublicTruthAutomation,
            &[
                "managed_lane.ai_gateway",
                "managed_lane.settings_sync",
                "managed_lane.companion_relay",
                "managed_lane.registry_mirror",
                "managed_lane.support_ingest",
                "managed_lane.managed_workspace",
            ],
            "Claim and public-truth automation narrows a marketed managed claim to the lane's effective claim when the active managed state caps it.",
        ),
    ];

    let inspection =
        CommercialControlPlaneInspection::derive(&lanes, &managed_states, &consumer_bindings, None);

    let summary =
        "Frozen entitlement, meter-family, chargeback-scope, org-switch, and grace-period \
        matrix for the managed lanes. Every lane keeps a local-safe baseline, and a managed lane's \
        marketed claim narrows automatically under the active managed state."
            .to_owned();

    CommercialControlPlaneMatrix {
        record_kind: MATRIX_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_CONTROL_PLANE_SCHEMA_VERSION,
        matrix_id,
        generated_at,
        matrix_revision,
        title,
        summary,
        source_refs: canonical_source_refs(),
        active_managed_state: None,
        lanes,
        managed_states,
        consumer_bindings,
        inspection,
    }
}
