//! Customer-visible usage and forecast views for the managed lanes.
//!
//! This module is the canonical usage-and-forecast object. Where the sibling
//! [`crate::m5_commercial_control_plane`] freezes the per-lane entitlement and
//! metering matrix, this module renders the *usage surface* a customer sees for
//! each claimed managed lane — the AI gateway, settings sync, the companion
//! relay, the registry/mirror surface, support ingest, and the managed
//! workspace. Every [`UsageForecastView`] pins the meter unit, the
//! month-to-date measurement descriptor (unit, aggregation window, scope owner,
//! as-of time, and freshness — never a raw spend or quota number), the owner
//! scope and the distinct chargeback scopes, the forecast threshold status and
//! the banner that explains *what changes next*, and the CSV/JSON export-parity
//! guarantee. It reuses the closed vocabularies already frozen by the
//! control-plane matrix — [`ServiceFamily`], [`MeterFamily`], [`ServiceId`],
//! [`MeterUnit`], [`AggregationWindow`], [`ScopeOwner`], [`ForecastConfidence`],
//! [`ManagedStateClass`], and [`MarketedClaim`] — plus the
//! [`SnapshotFreshness`](crate::m5_entitlement_summary::SnapshotFreshness)
//! freshness vocabulary, instead of minting a parallel synonym set.
//!
//! Five invariants keep the views honest. First, **no number crosses the
//! boundary bare**: every [`UsageMeasurement`] sets `carries_raw_number` to
//! false and binds its value to the unit, as-of time, and scope owner via
//! [`ValuePresentation`], so a month-to-date value is never shown without its
//! unit, time, and owner. Second, **a forecast banner explains what changes
//! next**: each [`ForecastBanner`] is recomputed from its [`ThresholdStatus`]
//! and carries a non-empty [`ForecastBanner::what_changes_next`] sentence rather
//! than only a warning color. Third, **every view exports at CSV/JSON parity**:
//! each [`ExportParity`] asserts both formats and confirmed parity, so usage and
//! forecast export the same fields. Fourth, **unlike service families never
//! merge**: there is one view per family with its own meter unit, and the set
//! never carries an opaque cross-family total. Fifth, **local core is never
//! blocked**: every view keeps a non-empty
//! [`UsageForecastView::local_safe_baseline`], so a stale or unavailable
//! metering path narrows the managed usage view but never local editing, search,
//! Git, or already-authorized local automation.
//!
//! A view's effective marketed claim is recomputed from its declared claim and
//! the cap of whichever managed state is active, so a stale meter, an exhausted
//! forecast, a grace window, a removed seat, an org switch, or a plan downgrade
//! narrows the marketed usage claim automatically;
//! [`UsageForecastViewSet::apply_managed_state`] exercises the narrowing and the
//! stored value must equal the recomputation or
//! [`UsageForecastViewSet::validate`] reports a violation. Because the seat,
//! org, grace, and sign-in states each carry their own typed managed state and
//! recovery cue, a usage surface can never draw one generic account error over
//! the distinct loss conditions.
//!
//! [`canonical_usage_forecast_view_set`] builds the frozen set and
//! [`current_stable_usage_forecast_view_set`] reads and validates the checked-in
//! packet at
//! [`artifacts/service/m5-usage-forecast-views.json`](../../../../artifacts/service/m5-usage-forecast-views.json),
//! so the account/usage surface, service-health diagnostics, Help/About, the
//! support/admin export, and the release center all ingest one packet rather
//! than cloning status text. [`UsageForecastViewSet::cross_check_against_control_plane`]
//! confirms each view projects its control-plane lane rather than a parallel
//! spreadsheet. The boundary schema is
//! [`schemas/service/m5-usage-forecast-views.schema.json`](../../../../schemas/service/m5-usage-forecast-views.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, canonical_stable_commercial_control_plane_matrix, AggregationWindow,
    ForecastConfidence, ManagedStateClass, MarketedClaim, MeterFamily, MeterUnit, ServiceFamily,
    ServiceId,
};
use crate::m5_entitlement_summary::SnapshotFreshness;

// Re-exported so a usage consumer can name the owner scope without reaching back
// into the control-plane module.
pub use crate::m5_commercial_control_plane::ScopeOwner;

#[cfg(test)]
mod tests;

/// Supported schema version for the usage-and-forecast view set.
pub const USAGE_FORECAST_VIEWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the view-set packet.
pub const VIEW_SET_RECORD_KIND: &str = "m5_usage_forecast_view_set";

/// Stable record-kind tag for a single usage-and-forecast view.
pub const VIEW_RECORD_KIND: &str = "m5_usage_forecast_view";

/// Stable record-kind tag for a usage-measurement descriptor.
pub const MEASUREMENT_RECORD_KIND: &str = "m5_usage_measurement_descriptor";

/// Stable record-kind tag for a forecast banner.
pub const FORECAST_BANNER_RECORD_KIND: &str = "m5_forecast_banner";

/// Stable record-kind tag for an export-parity descriptor.
pub const EXPORT_PARITY_RECORD_KIND: &str = "m5_usage_export_parity";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_usage_forecast_surface_binding";

/// Stable record-kind tag for the view-set inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_usage_forecast_inspection";

/// Repo-relative path to the boundary schema.
pub const USAGE_FORECAST_VIEWS_SCHEMA_REF: &str =
    "schemas/service/m5-usage-forecast-views.schema.json";

/// Repo-relative path to the reviewer contract.
pub const USAGE_FORECAST_VIEWS_DOC_REF: &str = "docs/m5/ship-usage-and-forecast-views-with-meter-units-as-of-time-owner-scope-threshold-banners-and-export-parity-for-ai-sync-relay-registry-and-workspace-services.md";

/// Repo-relative path to the checked-in view-set packet.
pub const USAGE_FORECAST_VIEWS_ARTIFACT_PATH: &str =
    "artifacts/service/m5-usage-forecast-views.json";

/// The forecast/threshold status a usage view reports for its window.
///
/// The status is metering posture, not an account error: a budget threshold or
/// a stale meter is distinct from a seat loss, an org switch, a grace window, or
/// a sign-in failure, which stay in the [`ManagedStateClass`] vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdStatus {
    /// Usage is within the budget for the window.
    WithinBudget,
    /// Usage is forecast to approach the budget threshold before the window resets.
    ApproachingThreshold,
    /// The forecast has crossed the budget threshold.
    ThresholdCrossed,
    /// The budget for the window is exhausted; new managed-only actions pause.
    BudgetExhausted,
    /// No forecast is available for the window; the month-to-date value implies no projection.
    ForecastUnavailable,
    /// The meter is stale; the number cannot be confirmed now and is labeled stale.
    MeterStaleUnconfirmed,
}

impl ThresholdStatus {
    /// Every threshold status, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WithinBudget,
        Self::ApproachingThreshold,
        Self::ThresholdCrossed,
        Self::BudgetExhausted,
        Self::ForecastUnavailable,
        Self::MeterStaleUnconfirmed,
    ];

    /// The banner severity this status maps to.
    pub const fn severity(self) -> BannerSeverity {
        match self {
            Self::WithinBudget => BannerSeverity::Informational,
            Self::ApproachingThreshold
            | Self::ForecastUnavailable
            | Self::MeterStaleUnconfirmed => BannerSeverity::Advisory,
            Self::ThresholdCrossed => BannerSeverity::Warning,
            Self::BudgetExhausted => BannerSeverity::Critical,
        }
    }
}

/// Closed banner-severity vocabulary.
///
/// Severity colors the banner, but a [`ForecastBanner`] never relies on color
/// alone: it always carries a [`ForecastBanner::what_changes_next`] sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BannerSeverity {
    /// Nothing changes; the window is within budget.
    Informational,
    /// A change is forecast but managed actions stay admissible.
    Advisory,
    /// The threshold is crossed; managed actions narrow next.
    Warning,
    /// The budget is exhausted; new managed-only actions pause.
    Critical,
}

/// How a month-to-date value is presented across the boundary.
///
/// A managed value is always bound to its unit, as-of time, and scope owner; it
/// is never shown bare. A local-only or not-applicable view suppresses the
/// managed number entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValuePresentation {
    /// The month-to-date value is shown only together with its unit, as-of time, and scope owner.
    MonthToDateBoundToUnitAsOfScope,
    /// No managed number is shown (local-only or not-applicable view).
    SuppressedNoManagedNumber,
}

/// A usage-measurement descriptor: unit, window, scope owner, as-of time,
/// freshness, and presentation — never a raw spend or quota number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageMeasurement {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The meter unit the month-to-date value is expressed in.
    pub meter_unit: MeterUnit,
    /// The aggregation window the value covers.
    pub aggregation_window: AggregationWindow,
    /// The scope that owns the metered usage.
    pub scope_owner: ScopeOwner,
    /// The as-of measurement time for the value.
    pub as_of: String,
    /// The freshness class of the measurement.
    pub freshness: SnapshotFreshness,
    /// How the value is presented; a managed value is always bound to unit, as-of time, and scope.
    pub value_presentation: ValuePresentation,
    /// Always false: a raw spend or quota number never crosses this boundary.
    pub carries_raw_number: bool,
}

/// A forecast banner that explains what changes next.
///
/// The banner is recomputed from its [`ThresholdStatus`] via
/// [`ForecastBanner::for_status`], so a stored banner that drifts from the
/// status fails validation. It never relies on a warning color alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastBanner {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The threshold status this banner renders.
    pub threshold_status: ThresholdStatus,
    /// The banner severity, derived from the threshold status.
    pub severity: BannerSeverity,
    /// Short reviewable headline.
    pub headline: String,
    /// Non-empty sentence explaining what changes next, not only a warning color.
    pub what_changes_next: String,
    /// The next user-visible step.
    pub recovery_cue: String,
}

impl ForecastBanner {
    /// Builds the canonical banner for a threshold status.
    pub fn for_status(status: ThresholdStatus) -> Self {
        let (headline, what_changes_next, recovery_cue) = match status {
            ThresholdStatus::WithinBudget => (
                "Within budget for this window.",
                "Usage is within the budget for the window; nothing changes until the forecast crosses the threshold. The month-to-date value is shown with its unit, as-of time, and scope owner.",
                "No action needed; managed actions are admissible and local work continues now.",
            ),
            ThresholdStatus::ApproachingThreshold => (
                "Forecast approaching the budget threshold.",
                "Usage is forecast to reach the budget threshold before the window resets; at the threshold, new managed-broker work pauses while local editing, search, and Git continue.",
                "Raise the budget or wait for the window reset to keep managed actions open; local work continues now.",
            ),
            ThresholdStatus::ThresholdCrossed => (
                "Forecast has crossed the budget threshold.",
                "The forecast has crossed the budget threshold; new managed actions narrow to the plan floor next while the local core continues unchanged.",
                "Raise the budget to widen managed actions; local work continues now.",
            ),
            ThresholdStatus::BudgetExhausted => (
                "Budget exhausted for this window.",
                "The budget for this window is exhausted; new managed-only actions pause until the window resets, while local editing, search, Git, and existing local automation continue.",
                "Wait for the window reset or raise the budget to resume managed actions; local work continues now.",
            ),
            ThresholdStatus::ForecastUnavailable => (
                "No forecast available for this window.",
                "No forecast is available for this window; the month-to-date value is shown with its as-of time and no projection is implied. Managed actions stay admissible and local work continues.",
                "A forecast returns when the rating data is available again; local work continues now.",
            ),
            ThresholdStatus::MeterStaleUnconfirmed => (
                "Metered number is stale and cannot be confirmed now.",
                "The metered number is stale and labeled with its last as-of time; it cannot be confirmed now, so no fresh forecast is implied and the local core is never blocked.",
                "The number refreshes when the meter reconnects; local editing, search, and Git are never blocked by a stale meter.",
            ),
        };
        Self {
            record_kind: FORECAST_BANNER_RECORD_KIND.to_owned(),
            schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
            threshold_status: status,
            severity: status.severity(),
            headline: headline.to_owned(),
            what_changes_next: what_changes_next.to_owned(),
            recovery_cue: recovery_cue.to_owned(),
        }
    }
}

/// A CSV/JSON export-parity descriptor for a usage and forecast view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportParity {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Always true: the view exports as CSV.
    pub csv: bool,
    /// Always true: the view exports as JSON.
    pub json: bool,
    /// Always true: CSV and JSON carry the same fields, unit, as-of time, and scope owner.
    pub parity_confirmed: bool,
}

impl ExportParity {
    /// Builds the canonical CSV/JSON parity descriptor.
    pub fn csv_json_parity() -> Self {
        Self {
            record_kind: EXPORT_PARITY_RECORD_KIND.to_owned(),
            schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
            csv: true,
            json: true,
            parity_confirmed: true,
        }
    }
}

/// One frozen usage-and-forecast view for a single service family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageForecastView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view identifier.
    pub view_id: String,
    /// Reviewable view title.
    pub title: String,
    /// Reviewable view summary.
    pub summary: String,
    /// The control-plane lane this view projects.
    pub lane_ref: String,
    /// Service family the view covers.
    pub service_family: ServiceFamily,
    /// Meter family the view is measured by.
    pub meter_family: MeterFamily,
    /// Service ids the view resolves through.
    pub linked_service_ids: Vec<ServiceId>,
    /// The month-to-date measurement descriptor.
    pub measurement: UsageMeasurement,
    /// Forecast confidence for the view's projection.
    pub forecast_confidence: ForecastConfidence,
    /// The forecast/threshold status for the window.
    pub threshold_status: ThresholdStatus,
    /// The banner that explains what changes next.
    pub forecast_banner: ForecastBanner,
    /// The CSV/JSON export-parity guarantee.
    pub export_parity: ExportParity,
    /// The distinct chargeback scopes the view offers; never collapsed to one total.
    pub chargeback_scope_offers: Vec<ScopeOwner>,
    /// The managed states that can narrow this view.
    pub applicable_managed_states: Vec<ManagedStateClass>,
    /// The marketed claim the view declares before narrowing.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the active managed state's cap is applied.
    pub effective_marketed_claim: MarketedClaim,
    /// The managed states that narrowed the view below its declared claim.
    pub narrowing_reasons: Vec<ManagedStateClass>,
    /// Short recovery cue. Present (non-null) when the view is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_cue: Option<String>,
    /// Non-empty local-safe baseline that always continues when the view narrows.
    pub local_safe_baseline: Vec<String>,
    /// Managed-only actions that pause when the view narrows.
    pub blocked_managed_only_actions: Vec<String>,
}

impl UsageForecastView {
    /// True when the view still publishes its full managed usage claim.
    pub fn backs_full_managed_claim(&self) -> bool {
        self.effective_marketed_claim == MarketedClaim::ManagedFull
    }
}

/// Closed surface vocabulary that must project the usage and forecast views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageForecastSurface {
    /// The account, seat, and usage surface.
    AccountUsageSurface,
    /// Service-health and diagnostics surfaces.
    ServiceHealthDiagnostics,
    /// The Help/About truth surface.
    HelpAbout,
    /// Support and admin export packets.
    SupportAdminExport,
    /// The release center.
    ReleaseCenter,
}

impl UsageForecastSurface {
    /// Every surface the views must reach.
    pub const ALL: [Self; 5] = [
        Self::AccountUsageSurface,
        Self::ServiceHealthDiagnostics,
        Self::HelpAbout,
        Self::SupportAdminExport,
        Self::ReleaseCenter,
    ];
}

/// One surface bound to the usage and forecast views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageForecastSurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The surface that projects the views.
    pub surface: UsageForecastSurface,
    /// The view ids this surface resolves through.
    pub bound_view_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the local-safe baseline.
    pub renders_local_safe_baseline: bool,
    /// Always true: the surface renders the banner's what-changes-next sentence.
    pub explains_what_changes_next: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the view set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageForecastInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of usage-and-forecast views.
    pub view_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// Number of distinct service families covered.
    pub service_families_covered: usize,
    /// Number of distinct meter families covered.
    pub meter_families_covered: usize,
    /// Number of distinct threshold statuses exercised.
    pub threshold_status_coverage: usize,
    /// True when every banner carries a non-empty what-changes-next sentence.
    pub all_banners_explain_what_changes_next: bool,
    /// True when every view carries a non-empty local-safe baseline.
    pub all_views_local_safe_backed: bool,
    /// True when every view exports at CSV/JSON parity.
    pub all_views_export_csv_json_parity: bool,
    /// True when no view shows a bare value: every measurement is bound and carries no raw number.
    pub value_never_bare: bool,
    /// True when there is one view per family and no opaque cross-family total.
    pub no_collapsed_family_total: bool,
    /// Number of views still backing the full managed claim.
    pub effective_full_view_count: usize,
    /// Number of views narrowed to a reduced managed claim.
    pub narrowed_view_count: usize,
    /// Number of views narrowed to the local-safe-only claim.
    pub local_safe_only_view_count: usize,
    /// The active managed state, when one has been applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_managed_state: Option<ManagedStateClass>,
}

/// The frozen usage-and-forecast view-set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageForecastViewSet {
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
    /// The active managed state, when one has been applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_managed_state: Option<ManagedStateClass>,
    /// The usage-and-forecast views.
    pub views: Vec<UsageForecastView>,
    /// The surface bindings.
    pub surface_bindings: Vec<UsageForecastSurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: UsageForecastInspection,
}

impl UsageForecastViewSet {
    /// Returns the view that covers `family`, when one is frozen.
    pub fn view_for_family(&self, family: ServiceFamily) -> Option<&UsageForecastView> {
        self.views.iter().find(|v| v.service_family == family)
    }

    /// Applies a single active managed state, narrowing every applicable view.
    ///
    /// Every view whose [`UsageForecastView::applicable_managed_states`] contains
    /// `state` has its effective marketed claim recomputed from the state's
    /// [`ManagedStateClass::claim_cap`], its narrowing reasons updated, and its
    /// recovery cue set; the inspection block is recomputed. The local-safe
    /// baseline is never removed, so the local core stays available.
    pub fn apply_managed_state(&mut self, state: ManagedStateClass) {
        self.active_managed_state = Some(state);
        for view in &mut self.views {
            let derived = derive_view_effective(
                view.declared_marketed_claim,
                &view.applicable_managed_states,
                Some(state),
            );
            view.effective_marketed_claim = derived.effective;
            view.narrowing_reasons = derived.reasons;
            view.recovery_cue = if derived.effective == view.declared_marketed_claim {
                None
            } else {
                Some(narrowing_recovery_cue(state))
            };
        }
        self.inspection = UsageForecastInspection::derive(
            &self.views,
            &self.surface_bindings,
            self.active_managed_state,
        );
    }

    /// Serializes the set as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the set cannot be serialized, which a validated set never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("usage-forecast view set serializes to JSON")
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`UsageForecastViewViolation`] per failed invariant: a wrong
    /// record kind or schema version, a missing identifier, a duplicate view, an
    /// incomplete service- or meter-family set, a collapsed chargeback scope, a
    /// bare value, a missing as-of time, a banner that drifts from its threshold
    /// status, a missing CSV/JSON export parity, an empty local-safe baseline, a
    /// stored effective claim that does not match the recomputation, a missing
    /// recovery cue on a narrowed view, an unbound surface, or a stale inspection
    /// block.
    pub fn validate(&self) -> Vec<UsageForecastViewViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(UsageForecastViewViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != VIEW_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != USAGE_FORECAST_VIEWS_SCHEMA_VERSION {
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
            .any(|entry| entry == USAGE_FORECAST_VIEWS_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.views.is_empty() {
            push("views", "set must contain at least one view");
        }

        let mut view_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for view in &self.views {
            self.validate_view(view, &mut push);
            if !view_ids.insert(view.view_id.as_str()) {
                push("views", "view_id values must be unique");
            }
            // One view per family: unlike service families never merge into one total.
            if !seen_families.insert(view.service_family) {
                push("views", "each service family must carry at most one view");
            }
        }

        // Every claimed service family and meter family must appear.
        for family in ServiceFamily::ALL {
            if !self.views.iter().any(|v| v.service_family == family) {
                push("views", "every service family must carry a usage view");
                break;
            }
        }
        for family in MeterFamily::ALL {
            if !self.views.iter().any(|v| v.meter_family == family) {
                push("views", "every meter family must carry a usage view");
                break;
            }
        }

        self.validate_surface_bindings(&mut push);

        let derived = UsageForecastInspection::derive(
            &self.views,
            &self.surface_bindings,
            self.active_managed_state,
        );
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed set",
            );
        }

        violations
    }

    fn validate_view(&self, view: &UsageForecastView, push: &mut impl FnMut(&str, &str)) {
        if view.record_kind != VIEW_RECORD_KIND {
            push("view.record_kind", "view record_kind is wrong");
        }
        if view.schema_version != USAGE_FORECAST_VIEWS_SCHEMA_VERSION {
            push("view.schema_version", "view schema_version is wrong");
        }
        for (field, value) in [
            ("view.view_id", &view.view_id),
            ("view.title", &view.title),
            ("view.summary", &view.summary),
            ("view.lane_ref", &view.lane_ref),
        ] {
            if value.trim().is_empty() {
                push(field, "value must be non-empty");
            }
        }
        if view.linked_service_ids.is_empty() {
            push(
                "view.linked_service_ids",
                "view must resolve through at least one service id",
            );
        }

        // No number crosses the boundary bare: the measurement is bound to its
        // unit, as-of time, and scope owner, and never carries a raw number.
        let m = &view.measurement;
        if m.record_kind != MEASUREMENT_RECORD_KIND {
            push(
                "view.measurement.record_kind",
                "measurement record_kind is wrong",
            );
        }
        if m.schema_version != USAGE_FORECAST_VIEWS_SCHEMA_VERSION {
            push(
                "view.measurement.schema_version",
                "measurement schema_version is wrong",
            );
        }
        if m.as_of.trim().is_empty() {
            push(
                "view.measurement.as_of",
                "a usage measurement must carry an as-of time",
            );
        }
        if m.carries_raw_number {
            push(
                "view.measurement.carries_raw_number",
                "a usage measurement must never carry a raw spend or quota number",
            );
        }
        if m.value_presentation != ValuePresentation::MonthToDateBoundToUnitAsOfScope {
            push(
                "view.measurement.value_presentation",
                "a managed usage view must bind its month-to-date value to the unit, as-of time, and scope owner",
            );
        }
        // A stale meter is labeled stale rather than shown live.
        if view.threshold_status == ThresholdStatus::MeterStaleUnconfirmed
            && m.freshness != SnapshotFreshness::FreshnessStale
        {
            push(
                "view.measurement.freshness",
                "a meter-stale view must label its measurement stale",
            );
        }

        // A forecast banner explains what changes next, recomputed from the status.
        let expected_banner = ForecastBanner::for_status(view.threshold_status);
        if view.forecast_banner != expected_banner {
            push(
                "view.forecast_banner",
                "stored forecast banner does not match the recomputed threshold status",
            );
        }

        // Every view exports at CSV/JSON parity.
        let e = &view.export_parity;
        if e.record_kind != EXPORT_PARITY_RECORD_KIND {
            push(
                "view.export_parity.record_kind",
                "export-parity record_kind is wrong",
            );
        }
        if e.schema_version != USAGE_FORECAST_VIEWS_SCHEMA_VERSION {
            push(
                "view.export_parity.schema_version",
                "export-parity schema_version is wrong",
            );
        }
        if !(e.csv && e.json && e.parity_confirmed) {
            push(
                "view.export_parity",
                "a usage and forecast view must export at confirmed CSV/JSON parity",
            );
        }

        // Personal, workspace, and organization scopes never collapse into one total.
        if view.chargeback_scope_offers.is_empty() {
            push(
                "view.chargeback_scope_offers",
                "view must offer at least one chargeback scope",
            );
        }
        let mut seen_scopes = BTreeSet::new();
        for scope in &view.chargeback_scope_offers {
            if !seen_scopes.insert(*scope) {
                push(
                    "view.chargeback_scope_offers",
                    "chargeback scopes must be distinct, not collapsed",
                );
            }
        }

        // The local core is never blocked: every view keeps a non-empty baseline.
        if view.local_safe_baseline.is_empty()
            || view.local_safe_baseline.iter().any(|s| s.trim().is_empty())
        {
            push(
                "view.local_safe_baseline",
                "every view must keep a non-empty local-safe baseline",
            );
        }

        // The applicable managed states must be distinct.
        let mut seen_states = BTreeSet::new();
        for state in &view.applicable_managed_states {
            if !seen_states.insert(*state) {
                push(
                    "view.applicable_managed_states",
                    "applicable managed states must be distinct",
                );
            }
        }

        // Recompute the view's effective claim and reasons.
        let derived = derive_view_effective(
            view.declared_marketed_claim,
            &view.applicable_managed_states,
            self.active_managed_state,
        );
        if derived.effective != view.effective_marketed_claim {
            push(
                "view.effective_marketed_claim",
                "stored effective claim does not match the recomputed view state",
            );
        }
        let stored_reasons: BTreeSet<ManagedStateClass> =
            view.narrowing_reasons.iter().copied().collect();
        let derived_reasons: BTreeSet<ManagedStateClass> =
            derived.reasons.iter().copied().collect();
        if stored_reasons != derived_reasons {
            push(
                "view.narrowing_reasons",
                "stored narrowing reasons do not match the recomputed view state",
            );
        }
        let narrowed = derived.effective != view.declared_marketed_claim;
        match (&view.recovery_cue, narrowed) {
            (None, true) => push(
                "view.recovery_cue",
                "a narrowed view must carry a recovery cue",
            ),
            (Some(cue), _) if cue.trim().is_empty() => {
                push("view.recovery_cue", "recovery cue must be non-empty")
            }
            _ => {}
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let view_ids: BTreeSet<&str> = self.views.iter().map(|v| v.view_id.as_str()).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != USAGE_FORECAST_VIEWS_SCHEMA_VERSION {
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
            if !binding.renders_local_safe_baseline {
                push(
                    "surface_binding.renders_local_safe_baseline",
                    "a surface must render the local-safe baseline",
                );
            }
            if !binding.explains_what_changes_next {
                push(
                    "surface_binding.explains_what_changes_next",
                    "a surface must render the banner's what-changes-next sentence",
                );
            }
            if binding.bound_view_ids.is_empty() {
                push(
                    "surface_binding.bound_view_ids",
                    "a binding must resolve through at least one view",
                );
            }
            for view_ref in &binding.bound_view_ids {
                if !view_ids.contains(view_ref.as_str()) {
                    push(
                        "surface_binding.bound_view_ids",
                        "binding view ref must resolve to a view",
                    );
                }
            }
        }
        // Every surface must be bound.
        for surface in UsageForecastSurface::ALL {
            if !self.surface_bindings.iter().any(|b| b.surface == surface) {
                push(
                    "surface_bindings",
                    "account/usage, service-health, Help/About, support/admin, and release-center must all bind",
                );
                break;
            }
        }
    }

    /// Cross-checks every view against its control-plane lane.
    ///
    /// Confirms each [`UsageForecastView`] projects the canonical
    /// commercial-control-plane matrix lane named by its
    /// [`UsageForecastView::lane_ref`] — the service family, meter family, meter
    /// unit, aggregation window, scope owner, service ids, and applicable managed
    /// states must match — so the usage surface is a real consumer of the matrix
    /// rather than a parallel spreadsheet.
    ///
    /// Returns an empty vector when every view matches its lane.
    pub fn cross_check_against_control_plane(&self) -> Vec<UsageForecastViewViolation> {
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let mut violations = Vec::new();
        for view in &self.views {
            let Some(lane) = matrix.lanes.iter().find(|l| l.lane_id == view.lane_ref) else {
                violations.push(UsageForecastViewViolation {
                    field: "view.lane_ref".to_owned(),
                    message: format!(
                        "lane_ref {} does not resolve to a control-plane lane",
                        view.lane_ref
                    ),
                });
                continue;
            };
            let mut mismatch = |field: &str| {
                violations.push(UsageForecastViewViolation {
                    field: field.to_owned(),
                    message: format!(
                        "view {} drifted from control-plane lane {}",
                        view.view_id, lane.lane_id
                    ),
                });
            };
            if view.service_family != lane.service_family {
                mismatch("view.service_family");
            }
            if view.meter_family != lane.meter_family {
                mismatch("view.meter_family");
            }
            if view.measurement.meter_unit != lane.meter_unit {
                mismatch("view.measurement.meter_unit");
            }
            if view.measurement.aggregation_window != lane.aggregation_window {
                mismatch("view.measurement.aggregation_window");
            }
            if view.measurement.scope_owner != lane.scope_owner {
                mismatch("view.measurement.scope_owner");
            }
            if view.linked_service_ids != lane.linked_service_ids {
                mismatch("view.linked_service_ids");
            }
            if view.applicable_managed_states != lane.applicable_managed_states {
                mismatch("view.applicable_managed_states");
            }
        }
        violations
    }
}

impl UsageForecastInspection {
    fn derive(
        views: &[UsageForecastView],
        surface_bindings: &[UsageForecastSurfaceBinding],
        active_managed_state: Option<ManagedStateClass>,
    ) -> Self {
        let service_families: BTreeSet<ServiceFamily> =
            views.iter().map(|v| v.service_family).collect();
        let meter_families: BTreeSet<MeterFamily> = views.iter().map(|v| v.meter_family).collect();
        let threshold_statuses: BTreeSet<ThresholdStatus> =
            views.iter().map(|v| v.threshold_status).collect();

        let effective_full_view_count = views
            .iter()
            .filter(|v| v.effective_marketed_claim == MarketedClaim::ManagedFull)
            .count();
        let local_safe_only_view_count = views
            .iter()
            .filter(|v| v.effective_marketed_claim == MarketedClaim::LocalSafeOnly)
            .count();
        let narrowed_view_count = views
            .iter()
            .filter(|v| v.effective_marketed_claim != v.declared_marketed_claim)
            .count();

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
            view_count: views.len(),
            surface_binding_count: surface_bindings.len(),
            service_families_covered: service_families.len(),
            meter_families_covered: meter_families.len(),
            threshold_status_coverage: threshold_statuses.len(),
            all_banners_explain_what_changes_next: views
                .iter()
                .all(|v| !v.forecast_banner.what_changes_next.trim().is_empty()),
            all_views_local_safe_backed: views.iter().all(|v| !v.local_safe_baseline.is_empty()),
            all_views_export_csv_json_parity: views.iter().all(|v| {
                v.export_parity.csv && v.export_parity.json && v.export_parity.parity_confirmed
            }),
            value_never_bare: views.iter().all(|v| {
                !v.measurement.carries_raw_number
                    && v.measurement.value_presentation
                        == ValuePresentation::MonthToDateBoundToUnitAsOfScope
                    && !v.measurement.as_of.trim().is_empty()
            }),
            // One view per family and no opaque cross-family total.
            no_collapsed_family_total: views.len() == service_families.len()
                && service_families.len() == ServiceFamily::ALL.len(),
            effective_full_view_count,
            narrowed_view_count,
            local_safe_only_view_count,
            active_managed_state,
        }
    }
}

/// One failed view-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageForecastViewViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for UsageForecastViewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum UsageForecastViewError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<UsageForecastViewViolation>),
}

impl fmt::Display for UsageForecastViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "usage-forecast view set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "usage-forecast view set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for UsageForecastViewError {}

struct DerivedViewState {
    effective: MarketedClaim,
    reasons: Vec<ManagedStateClass>,
}

fn claim_rank(claim: MarketedClaim) -> u8 {
    match claim {
        MarketedClaim::LocalSafeOnly => 0,
        MarketedClaim::ManagedNarrowed => 1,
        MarketedClaim::ManagedFull => 2,
    }
}

fn derive_view_effective(
    declared: MarketedClaim,
    applicable_states: &[ManagedStateClass],
    active_state: Option<ManagedStateClass>,
) -> DerivedViewState {
    let mut effective = declared;
    let mut reasons = Vec::new();
    if let Some(state) = active_state {
        if applicable_states.contains(&state) {
            let cap = state.claim_cap();
            if claim_rank(cap) < claim_rank(effective) {
                effective = cap;
            }
            if claim_rank(cap) < claim_rank(declared) {
                reasons.push(state);
            }
        }
    }
    DerivedViewState { effective, reasons }
}

fn narrowing_recovery_cue(state: ManagedStateClass) -> String {
    match state {
        ManagedStateClass::SignedIn => "Managed actions continue; no recovery needed.",
        ManagedStateClass::LocalOnly => {
            "Sign in to a managed account to enable the managed usage view; local work continues now."
        }
        ManagedStateClass::ReauthRequired => {
            "Reauthenticate to refresh the managed usage view; local work continues now."
        }
        ManagedStateClass::ManagedBlocked => {
            "Review the account or policy hold to restore the managed usage view; local work continues now."
        }
        ManagedStateClass::GracePeriod => {
            "Export the bounded usage summary before the grace window closes; local work continues now."
        }
        ManagedStateClass::SeatRemoved => {
            "Ask an admin to restore the seat to resume the managed usage view; local work continues now."
        }
        ManagedStateClass::PlanDowngrade => {
            "The usage view is on the plan floor; upgrade the plan to widen it. Local work continues now."
        }
        ManagedStateClass::OrgSwitched => {
            "The managed usage view is rebinding to the new org; local work continues now."
        }
        ManagedStateClass::ForecastThreshold => {
            "Usage is approaching the budget; raise the budget or wait for the window reset. Local work continues now."
        }
        ManagedStateClass::MeterStale => {
            "The metered number is stale and labeled; the usage view refreshes when the meter reconnects. Local work continues now."
        }
    }
    .to_owned()
}

/// Reads and validates the checked-in stable usage-and-forecast view set.
///
/// This is the canonical reader: the account/usage surface, service-health
/// diagnostics, Help/About, the support/admin export, and the release center
/// call it to ingest the views rather than cloning status text.
///
/// # Errors
///
/// Returns [`UsageForecastViewError`] when the checked-in packet fails to parse
/// or fails validation.
pub fn current_stable_usage_forecast_view_set(
) -> Result<UsageForecastViewSet, UsageForecastViewError> {
    let set: UsageForecastViewSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-usage-forecast-views.json"
    )))
    .map_err(UsageForecastViewError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(UsageForecastViewError::Validation(violations))
    }
}

/// Source refs every usage-and-forecast export carries.
fn usage_forecast_source_refs() -> Vec<String> {
    let mut refs = vec![
        USAGE_FORECAST_VIEWS_SCHEMA_REF.to_owned(),
        USAGE_FORECAST_VIEWS_DOC_REF.to_owned(),
    ];
    // Reuse the control-plane refs so the views cite the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

fn measurement(
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    scope_owner: ScopeOwner,
    as_of: &str,
    freshness: SnapshotFreshness,
) -> UsageMeasurement {
    UsageMeasurement {
        record_kind: MEASUREMENT_RECORD_KIND.to_owned(),
        schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
        meter_unit,
        aggregation_window,
        scope_owner,
        as_of: as_of.to_owned(),
        freshness,
        value_presentation: ValuePresentation::MonthToDateBoundToUnitAsOfScope,
        carries_raw_number: false,
    }
}

// One view freezes many fixed columns; a wide builder keeps the canonical set
// readable as one table.
#[allow(clippy::too_many_arguments)]
fn view(
    view_id: &str,
    title: &str,
    summary: &str,
    lane_ref: &str,
    service_family: ServiceFamily,
    meter_family: MeterFamily,
    linked_service_ids: &[ServiceId],
    measurement: UsageMeasurement,
    forecast_confidence: ForecastConfidence,
    threshold_status: ThresholdStatus,
    chargeback_scope_offers: &[ScopeOwner],
    local_safe_baseline: &[&str],
    blocked_managed_only_actions: &[&str],
) -> UsageForecastView {
    UsageForecastView {
        record_kind: VIEW_RECORD_KIND.to_owned(),
        schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
        view_id: view_id.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        lane_ref: lane_ref.to_owned(),
        service_family,
        meter_family,
        linked_service_ids: linked_service_ids.to_vec(),
        measurement,
        forecast_confidence,
        threshold_status,
        forecast_banner: ForecastBanner::for_status(threshold_status),
        export_parity: ExportParity::csv_json_parity(),
        chargeback_scope_offers: chargeback_scope_offers.to_vec(),
        applicable_managed_states: ManagedStateClass::ALL.to_vec(),
        declared_marketed_claim: MarketedClaim::ManagedFull,
        effective_marketed_claim: MarketedClaim::ManagedFull,
        narrowing_reasons: Vec::new(),
        recovery_cue: None,
        local_safe_baseline: local_safe_baseline
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        blocked_managed_only_actions: blocked_managed_only_actions
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    }
}

fn binding(
    binding_id: &str,
    surface: UsageForecastSurface,
    bound_view_ids: &[&str],
    summary: &str,
) -> UsageForecastSurfaceBinding {
    UsageForecastSurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        surface,
        bound_view_ids: bound_view_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_local_safe_baseline: true,
        explains_what_changes_next: true,
        summary: summary.to_owned(),
    }
}

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "usage-forecast-views:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Usage and forecast views with meter units, as-of time, owner scope, threshold banners, and export parity";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Deterministic as-of time for the checked-in usage measurements.
pub const STABLE_MEASUREMENT_AS_OF: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_usage_forecast_view_set() -> UsageForecastViewSet {
    canonical_usage_forecast_view_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen usage-and-forecast view set.
///
/// The set freezes one view per claimed service family — the AI gateway,
/// settings sync, the companion relay, the registry/mirror surface, support
/// ingest, and the managed workspace — and one binding per surface. Each view
/// projects its control-plane lane, binds its month-to-date value to the unit,
/// as-of time, and scope owner, carries a forecast banner that explains what
/// changes next, exports at CSV/JSON parity, and keeps a non-empty local-safe
/// baseline. All views start at their full managed claim with no active managed
/// state; callers use [`UsageForecastViewSet::apply_managed_state`] to exercise
/// narrowing.
pub fn canonical_usage_forecast_view_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> UsageForecastViewSet {
    let views = vec![
        view(
            "usage_forecast.ai_gateway",
            "Managed AI gateway usage and forecast",
            "Month-to-date managed AI token spend, metered per organization, with its as-of time, scope owner, forecast threshold status, and CSV/JSON export parity; direct and bring-your-own-key AI routes continue when the managed lane narrows.",
            "managed_lane.ai_gateway",
            ServiceFamily::AiGatewayFamily,
            MeterFamily::AiGatewayMeterFamily,
            &[ServiceId::ManagedAiBroker],
            measurement(
                MeterUnit::Tokens,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessLive,
            ),
            ForecastConfidence::ForecastAuthoritative,
            ThresholdStatus::ApproachingThreshold,
            &[ScopeOwner::Personal, ScopeOwner::Workspace, ScopeOwner::Organization],
            &[
                "Direct and bring-your-own-key AI routes keep running.",
                "Local editing, search, and Git are unaffected.",
            ],
            &["New managed-broker inference once the monthly token budget is exhausted."],
        ),
        view(
            "usage_forecast.settings_sync",
            "Managed settings sync usage and forecast",
            "Month-to-date stored bytes on the managed settings-sync store, metered per workspace, with its as-of time, scope owner, threshold status, and CSV/JSON export parity; local settings and files stay authoritative when the lane narrows.",
            "managed_lane.settings_sync",
            ServiceFamily::SyncFamily,
            MeterFamily::ProfileOrSettingsSyncMeterFamily,
            &[ServiceId::ManagedSettingsSync],
            measurement(
                MeterUnit::BytesStored,
                AggregationWindow::Rolling30d,
                ScopeOwner::Workspace,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            ),
            ForecastConfidence::ForecastBestEffortLocal,
            ThresholdStatus::WithinBudget,
            &[ScopeOwner::Personal, ScopeOwner::Workspace, ScopeOwner::Organization],
            &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the lane clears.",
            ],
            &["Pushing new settings snapshots to the managed store once storage is exhausted."],
        ),
        view(
            "usage_forecast.companion_relay",
            "Companion relay usage and forecast",
            "Month-to-date participant minutes on the managed relay, metered per workspace, with its as-of time, scope owner, threshold status, and CSV/JSON export parity; local incident notes and offline packets continue when the relay narrows.",
            "managed_lane.companion_relay",
            ServiceFamily::CollaborationRelayFamily,
            MeterFamily::CollaborationRelayMeterFamily,
            &[ServiceId::ManagedRelay],
            measurement(
                MeterUnit::ParticipantMinutes,
                AggregationWindow::Rolling24h,
                ScopeOwner::Workspace,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessRecent,
            ),
            ForecastConfidence::ForecastBestEffortLocal,
            ThresholdStatus::BudgetExhausted,
            &[ScopeOwner::Workspace, ScopeOwner::Organization],
            &[
                "Local incident notes and offline packets continue.",
                "Desktop handoff resumes the exact local context.",
            ],
            &["Joining a live companion-follow or relay session once relay minutes are exhausted."],
        ),
        view(
            "usage_forecast.registry_mirror",
            "Registry and mirror usage and forecast",
            "Month-to-date download count on the managed registry and mirror, metered per organization, with its as-of time, scope owner, threshold status, and CSV/JSON export parity; installed extensions and local or sideloaded packages keep running when the lane narrows.",
            "managed_lane.registry_mirror",
            ServiceFamily::RegistryOrMirrorMetadataFamily,
            MeterFamily::RegistryOrMirrorMeterFamily,
            &[ServiceId::ManagedMarketplace, ServiceId::ManagedCatalog],
            measurement(
                MeterUnit::DownloadCount,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessAging,
            ),
            ForecastConfidence::ForecastBestEffortLocal,
            ThresholdStatus::ForecastUnavailable,
            &[ScopeOwner::Organization, ScopeOwner::Tenant],
            &[
                "Installed extensions keep running.",
                "Local and sideloaded packages are unaffected.",
            ],
            &["New managed-registry installs or publishes once the monthly download budget is exhausted."],
        ),
        view(
            "usage_forecast.support_ingest",
            "Support ingest usage and forecast",
            "Month-to-date support-bundle uploads to the managed ingest sink, metered per tenant, with its as-of time, scope owner, threshold status, and CSV/JSON export parity; local support bundles still generate when the lane narrows.",
            "managed_lane.support_ingest",
            ServiceFamily::TelemetryOrSupportIngestFamily,
            MeterFamily::SupportIngestMeterFamily,
            &[ServiceId::ManagedSupportExport, ServiceId::ManagedTelemetrySink],
            measurement(
                MeterUnit::SupportBundleCount,
                AggregationWindow::Rolling30d,
                ScopeOwner::Tenant,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessStale,
            ),
            ForecastConfidence::ForecastBestEffortLocal,
            ThresholdStatus::MeterStaleUnconfirmed,
            &[ScopeOwner::Organization, ScopeOwner::Tenant],
            &[
                "Local support bundles still generate on device.",
                "Offline evidence capture continues.",
            ],
            &["Uploading new support bundles to the managed sink once the ingest budget is exhausted."],
        ),
        view(
            "usage_forecast.managed_workspace",
            "Managed workspace usage and forecast",
            "Month-to-date remote workspace hours on the managed control plane, metered per organization, with its as-of time, scope owner, threshold status, and CSV/JSON export parity; local checkout, editing, tasks, and Git continue when the remote workspace narrows.",
            "managed_lane.managed_workspace",
            ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            MeterFamily::RemoteWorkspaceControlPlaneMeterFamily,
            &[ServiceId::ManagedRelay],
            measurement(
                MeterUnit::WorkspaceHours,
                AggregationWindow::CalendarMonthUtc,
                ScopeOwner::Organization,
                STABLE_MEASUREMENT_AS_OF,
                SnapshotFreshness::FreshnessLive,
            ),
            ForecastConfidence::ForecastAuthoritative,
            ThresholdStatus::ThresholdCrossed,
            &[ScopeOwner::Workspace, ScopeOwner::Organization],
            &[
                "Local checkout and editing continue.",
                "Local tasks and Git are unaffected when the remote workspace narrows.",
            ],
            &["Attaching or running a new remote workspace once the workspace-hour budget is exhausted."],
        ),
    ];

    let all_view_ids: Vec<&str> = views.iter().map(|v| v.view_id.as_str()).collect();

    let surface_bindings = vec![
        binding(
            "usage_surface.account",
            UsageForecastSurface::AccountUsageSurface,
            &all_view_ids,
            "The account and usage surface renders each view's meter unit, month-to-date value, as-of time, owner scope, forecast threshold status, banner, and CSV/JSON export action.",
        ),
        binding(
            "usage_surface.service_health",
            UsageForecastSurface::ServiceHealthDiagnostics,
            &all_view_ids,
            "Service-health and diagnostics surfaces project the forecast threshold status, measurement freshness, and forecast confidence without inventing a stronger claim.",
        ),
        binding(
            "usage_surface.help_about",
            UsageForecastSurface::HelpAbout,
            &[
                "usage_forecast.ai_gateway",
                "usage_forecast.settings_sync",
                "usage_forecast.registry_mirror",
            ],
            "The Help/About truth surface names which managed lanes carry a usage and forecast view and their local-safe baseline.",
        ),
        binding(
            "usage_surface.support_admin",
            UsageForecastSurface::SupportAdminExport,
            &[
                "usage_forecast.support_ingest",
                "usage_forecast.companion_relay",
                "usage_forecast.managed_workspace",
            ],
            "Support and admin export packets carry the CSV/JSON usage export, its as-of time and scope owner, and the posture origin of any narrowing.",
        ),
        binding(
            "usage_surface.release_center",
            UsageForecastSurface::ReleaseCenter,
            &all_view_ids,
            "The release center narrows the marketed usage claim to each view's effective claim when the active managed state caps it.",
        ),
    ];

    let inspection = UsageForecastInspection::derive(&views, &surface_bindings, None);

    let summary = "Frozen customer-visible usage and forecast views for the managed lanes. Each \
        view names its meter unit, month-to-date value (bound to the unit, as-of time, and scope \
        owner), forecast threshold status, and CSV/JSON export parity; every banner explains what \
        changes next, unlike service families never merge into one total, and every view keeps a \
        local-safe baseline that continues when the managed lane narrows."
        .to_owned();

    UsageForecastViewSet {
        record_kind: VIEW_SET_RECORD_KIND.to_owned(),
        schema_version: USAGE_FORECAST_VIEWS_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary,
        source_refs: usage_forecast_source_refs(),
        active_managed_state: None,
        views,
        surface_bindings,
        inspection,
    }
}
