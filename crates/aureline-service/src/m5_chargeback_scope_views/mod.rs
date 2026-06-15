//! Chargeback scope switchers and personal/workspace/team/org cost truth.
//!
//! This module is the canonical, inspectable chargeback surface for Aureline's
//! optional managed lanes. Where the commercial-control-plane matrix freezes the
//! per-lane entitlement and metering contract and the usage-and-forecast views
//! render the month-to-date number a customer sees, this set answers a different
//! question: **who owns the cost, and is it charged directly or inherited from a
//! broader scope.** It freezes one [`ChargebackScopeView`] per claimed service
//! family — the AI gateway, settings sync, the companion relay, the
//! registry/mirror surface, support ingest, and the managed workspace — and a
//! single [`ChargebackScopeSwitcher`] that holds the active scope across them.
//!
//! Each view carries one [`ScopeCostTruth`] per offered chargeback scope, and
//! the personal, workspace, team, and organization scopes never collapse into one
//! ambiguous owner bucket. Every scope truth separates a **direct** cost line
//! from an **inherited** one: the direct line is the cost charged to the scope
//! itself, and the inherited line names the broader parent scope it rolls up from
//! ([`ScopeCostTruth::inherited`] → [`CostAttributionMeasurement::inherited_from`]).
//! The broadest scope in a view is the inheritance-chain root, so its inherited
//! line is explicitly not applicable rather than a hidden zero. No raw spend or
//! quota number ever crosses the boundary; every measurement is a descriptor
//! bound to its unit, aggregation window, as-of time, and scope owner.
//!
//! The switcher [preserves the active scope](ChargebackScopeSwitcher::preserves_active_scope),
//! the [inherited-versus-direct separation](ChargebackScopeSwitcher::preserves_inherited_direct_separation),
//! and the [owner identity](ChargebackScopeSwitcher::preserves_owner_identity)
//! through a [`ChargebackScopeViewSet::switch_scope`] call, and never collapses
//! the scopes into one total. The whole set exports at CSV/JSON parity —
//! [`ChargebackScopeViewSet::export_safe_csv`] and
//! [`ChargebackScopeViewSet::export_safe_json`] carry the same per-scope direct
//! and inherited lines — so the chargeback explanation lives in the product, not
//! only in a billing portal.
//!
//! Two invariants keep the surface honest. First, **local core is never
//! blocked**: every view keeps a non-empty
//! [`ChargebackScopeView::local_safe_baseline`], so a stale or unavailable
//! metering path narrows the managed chargeback view but never local editing,
//! search, Git, or already-authorized local automation. Second, **loss
//! conditions stay distinct**: a view's effective marketed claim is recomputed
//! from the active managed state's cap, so a removed seat, an org switch, a grace
//! window, and a sign-in failure each narrow with their own typed state and
//! recovery cue rather than collapsing into one generic account error.
//!
//! [`canonical_chargeback_scope_view_set`] builds the frozen set and
//! [`current_stable_chargeback_scope_view_set`] reads and validates the checked-in
//! packet at
//! [`artifacts/service/m5-chargeback-scope-views.json`](../../../../artifacts/service/m5-chargeback-scope-views.json),
//! so the account chargeback surface, service-health diagnostics, Help/About, the
//! support/admin export, and the release center all ingest one packet rather than
//! cloning status text. [`ChargebackScopeViewSet::cross_check_against_control_plane`]
//! confirms each view projects its control-plane lane rather than a parallel
//! spreadsheet. The boundary schema is
//! [`schemas/service/m5-chargeback-scope-views.schema.json`](../../../../schemas/service/m5-chargeback-scope-views.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, canonical_stable_commercial_control_plane_matrix, AggregationWindow,
    ManagedStateClass, MarketedClaim, MeterFamily, MeterUnit, ServiceFamily, ServiceId,
};
use crate::m5_entitlement_summary::SnapshotFreshness;

// Reuse the export-parity and value-presentation packets the usage surface
// already froze rather than minting a parallel synonym set.
pub use crate::m5_commercial_control_plane::ScopeOwner;
pub use crate::m5_usage_forecast_views::{ExportParity, ValuePresentation};

#[cfg(test)]
mod tests;

/// Supported schema version for the chargeback-scope view set.
pub const CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the view-set packet.
pub const VIEW_SET_RECORD_KIND: &str = "m5_chargeback_scope_view_set";

/// Stable record-kind tag for a chargeback-scope view.
pub const VIEW_RECORD_KIND: &str = "m5_chargeback_scope_view";

/// Stable record-kind tag for a per-scope cost-truth row.
pub const SCOPE_COST_TRUTH_RECORD_KIND: &str = "m5_chargeback_scope_cost_truth";

/// Stable record-kind tag for a direct or inherited cost-attribution descriptor.
pub const COST_ATTRIBUTION_RECORD_KIND: &str = "m5_chargeback_cost_attribution";

/// Stable record-kind tag for the scope switcher.
pub const SWITCHER_RECORD_KIND: &str = "m5_chargeback_scope_switcher";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_chargeback_scope_surface_binding";

/// Stable record-kind tag for the inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_chargeback_scope_inspection";

/// Repo-relative path to the boundary schema.
pub const CHARGEBACK_SCOPE_VIEWS_SCHEMA_REF: &str =
    "schemas/service/m5-chargeback-scope-views.schema.json";

/// Repo-relative path to the reviewer contract.
pub const CHARGEBACK_SCOPE_VIEWS_DOC_REF: &str = "docs/m5/implement-chargeback-scope-switchers-and-personal-versus-workspace-versus-team-versus-org-cost-truth-with-inherited-direct-separation-and-csv-json-exports.md";

/// Repo-relative path to the checked-in view-set packet.
pub const CHARGEBACK_SCOPE_VIEWS_ARTIFACT_PATH: &str =
    "artifacts/service/m5-chargeback-scope-views.json";

/// The fixed CSV header for the per-scope chargeback export.
pub const CHARGEBACK_CSV_HEADER: &str = "view_id,service_family,meter_family,scope_owner,owner_identity,attribution_basis,inherited_from,meter_unit,aggregation_window,as_of,freshness,value_presentation,carries_raw_number,effective_marketed_claim";

/// Whether a cost/usage line is charged directly to a scope or inherited from a
/// broader parent scope.
///
/// The two bases never collapse into one number: a scope's chargeback truth
/// always carries both lines so the attribution is inspectable in the product
/// rather than deferred to a billing portal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionBasis {
    /// The cost is charged directly to this scope's own activity.
    Direct,
    /// The cost is inherited from a broader parent scope's pooled allocation.
    Inherited,
}

/// A direct or inherited cost-attribution descriptor.
///
/// Like the usage measurement, this is a descriptor — meter unit, aggregation
/// window, scope owner, as-of time, freshness, and value presentation — and
/// never a raw spend or quota number. An inherited line names the parent scope it
/// rolls up from in [`Self::inherited_from`]; a direct line, and the inherited
/// line of the chain-root scope, leave it absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostAttributionMeasurement {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Whether this line is charged directly to the scope or inherited.
    pub attribution_basis: AttributionBasis,
    /// The broader parent scope this inherited line rolls up from.
    ///
    /// Present only on an inherited line that has a parent in the view; a direct
    /// line and the chain-root inherited line leave it absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<ScopeOwner>,
    /// The meter unit the cost is expressed in.
    pub meter_unit: MeterUnit,
    /// The aggregation window the cost covers.
    pub aggregation_window: AggregationWindow,
    /// The scope this cost line lands on.
    pub scope_owner: ScopeOwner,
    /// The as-of measurement time for the cost.
    pub as_of: String,
    /// The freshness class of the measurement.
    pub freshness: SnapshotFreshness,
    /// How the value is presented; a shown value is bound to unit, as-of time, and scope.
    pub value_presentation: ValuePresentation,
    /// Always false: a raw spend or quota number never crosses this boundary.
    pub carries_raw_number: bool,
}

/// One scope's chargeback truth within a view.
///
/// Names the scope, its opaque owner identity, and the direct and inherited cost
/// lines that never collapse into one total.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCostTruth {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The chargeback scope this truth describes.
    pub scope_owner: ScopeOwner,
    /// An opaque owner-identity ref; never a raw tenant or account name.
    pub owner_identity: String,
    /// The cost charged directly to this scope.
    pub direct: CostAttributionMeasurement,
    /// The cost inherited from a broader parent scope, or explicitly not applicable at the root.
    pub inherited: CostAttributionMeasurement,
    /// Reviewable summary of how this scope's direct and inherited cost are shown.
    pub summary: String,
}

/// One frozen chargeback-scope view in the set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChargebackScopeView {
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
    /// Service family the view resolves through.
    pub service_family: ServiceFamily,
    /// Meter family the view is measured by.
    pub meter_family: MeterFamily,
    /// Meter unit shared by every cost line in the view.
    pub meter_unit: MeterUnit,
    /// Aggregation window shared by every cost line in the view.
    pub aggregation_window: AggregationWindow,
    /// Service ids the view resolves through.
    pub linked_service_ids: Vec<ServiceId>,
    /// One cost truth per offered chargeback scope; scopes never collapse to one total.
    pub scope_cost_truths: Vec<ScopeCostTruth>,
    /// Bounded CSV/JSON export-parity guarantee for the view's per-scope breakdown.
    pub export_parity: ExportParity,
    /// The managed states that can affect this view.
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

impl ChargebackScopeView {
    /// True when the view still publishes its full managed claim.
    pub fn backs_full_managed_claim(&self) -> bool {
        self.effective_marketed_claim == MarketedClaim::ManagedFull
    }

    /// Returns the cost truth for `scope`, when the view offers it.
    pub fn scope_truth(&self, scope: ScopeOwner) -> Option<&ScopeCostTruth> {
        self.scope_cost_truths
            .iter()
            .find(|t| t.scope_owner == scope)
    }
}

/// The scope switcher that holds the active chargeback scope across the views.
///
/// Switching scope preserves the active scope, the inherited-versus-direct
/// separation, and the owner identity, and never collapses the scopes into one
/// total — the four preservation flags are always true.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChargebackScopeSwitcher {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable switcher identifier.
    pub switcher_id: String,
    /// The distinct scopes the switcher offers, broadest-last.
    pub available_scopes: Vec<ScopeOwner>,
    /// The currently active scope; always one of [`Self::available_scopes`].
    pub active_scope: ScopeOwner,
    /// Always true: a scope switch preserves the active scope.
    pub preserves_active_scope: bool,
    /// Always true: a scope switch keeps the direct and inherited lines separate.
    pub preserves_inherited_direct_separation: bool,
    /// Always true: a scope switch preserves each scope's owner identity.
    pub preserves_owner_identity: bool,
    /// Always true: the scopes never collapse into one ambiguous owner total.
    pub never_collapses_scopes: bool,
    /// Reviewable summary of what the switcher preserves.
    pub summary: String,
}

/// Closed surface vocabulary that must project the chargeback scope views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargebackScopeSurface {
    /// The account chargeback and cost-ownership surface.
    AccountChargebackSurface,
    /// Service-health and diagnostics surfaces.
    ServiceHealthDiagnostics,
    /// The Help/About truth surface.
    HelpAbout,
    /// Support and admin export packets.
    SupportAdminExport,
    /// The release center.
    ReleaseCenter,
}

impl ChargebackScopeSurface {
    /// Every surface the set must reach.
    pub const ALL: [Self; 5] = [
        Self::AccountChargebackSurface,
        Self::ServiceHealthDiagnostics,
        Self::HelpAbout,
        Self::SupportAdminExport,
        Self::ReleaseCenter,
    ];
}

/// One consumer surface bound to the chargeback scope views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChargebackScopeSurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The surface that projects the views.
    pub surface: ChargebackScopeSurface,
    /// The view ids this surface resolves through.
    pub bound_view_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the inherited-versus-direct separation.
    pub renders_inherited_direct_separation: bool,
    /// Always true: the surface renders the local-safe baseline.
    pub renders_local_safe_baseline: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChargebackScopeInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of chargeback-scope views.
    pub view_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// Number of distinct service families covered.
    pub service_families_covered: usize,
    /// Number of distinct scope owners covered across the views.
    pub scope_owners_covered: usize,
    /// True when every scope truth carries a distinct direct and inherited line.
    pub all_views_separate_direct_and_inherited: bool,
    /// True when every view exports at confirmed CSV/JSON parity.
    pub all_views_export_csv_json_parity: bool,
    /// True when no cost line carries a raw spend or quota number.
    pub value_never_bare: bool,
    /// True when no view collapses its scopes into one total.
    pub no_collapsed_scope_total: bool,
    /// True when every view carries a non-empty local-safe baseline.
    pub all_views_local_safe_backed: bool,
    /// True when the switcher preserves the active scope.
    pub switcher_preserves_active_scope: bool,
    /// True when the active scope is one the switcher offers.
    pub switcher_active_scope_available: bool,
    /// The active chargeback scope held by the switcher.
    pub active_scope: ScopeOwner,
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

/// The frozen chargeback-scope view set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChargebackScopeViewSet {
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
    /// The scope switcher holding the active scope.
    pub switcher: ChargebackScopeSwitcher,
    /// The chargeback-scope views.
    pub views: Vec<ChargebackScopeView>,
    /// The surface bindings.
    pub surface_bindings: Vec<ChargebackScopeSurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: ChargebackScopeInspection,
}

impl ChargebackScopeViewSet {
    /// Returns the view that covers `family`, when one is frozen.
    pub fn view_for_family(&self, family: ServiceFamily) -> Option<&ChargebackScopeView> {
        self.views.iter().find(|v| v.service_family == family)
    }

    /// Switches the active chargeback scope, preserving every view and scope.
    ///
    /// The switch only moves the active scope held by the switcher; it never
    /// removes a scope, collapses the direct/inherited separation, or drops an
    /// owner identity. The inspection block is recomputed so the active scope is
    /// reflected.
    pub fn switch_scope(&mut self, scope: ScopeOwner) {
        self.switcher.active_scope = scope;
        self.inspection = ChargebackScopeInspection::derive(
            &self.switcher,
            &self.views,
            &self.surface_bindings,
            self.active_managed_state,
        );
    }

    /// Applies a single active managed state, narrowing every applicable view.
    ///
    /// Every view whose [`ChargebackScopeView::applicable_managed_states`]
    /// contains `state` has its effective marketed claim recomputed from the
    /// state's [`ManagedStateClass::claim_cap`], its narrowing reasons updated, and
    /// its recovery cue set; the inspection block is recomputed. The local-safe
    /// baseline and the per-scope cost truth are never removed, so the local core
    /// stays available and the scopes stay inspectable.
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
        self.inspection = ChargebackScopeInspection::derive(
            &self.switcher,
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
        serde_json::to_string_pretty(self).expect("chargeback-scope view set serializes to JSON")
    }

    /// Serializes the per-scope chargeback breakdown as deterministic CSV.
    ///
    /// One row per view, scope, and attribution basis (direct then inherited), so
    /// the CSV carries the same fields, unit, as-of time, scope owner, and
    /// inherited/direct separation as the JSON export — the CSV/JSON parity the
    /// set guarantees. No cell carries a raw spend or quota number; an inherited
    /// line with no parent shows `suppressed_no_managed_number`.
    pub fn export_safe_csv(&self) -> String {
        let mut out = String::with_capacity(1024);
        out.push_str(CHARGEBACK_CSV_HEADER);
        out.push('\n');
        for view in &self.views {
            let effective = token(&view.effective_marketed_claim);
            for truth in &view.scope_cost_truths {
                for component in [&truth.direct, &truth.inherited] {
                    let inherited_from = component
                        .inherited_from
                        .map(|s| token(&s))
                        .unwrap_or_default();
                    let row = [
                        view.view_id.clone(),
                        token(&view.service_family),
                        token(&view.meter_family),
                        token(&truth.scope_owner),
                        truth.owner_identity.clone(),
                        token(&component.attribution_basis),
                        inherited_from,
                        token(&component.meter_unit),
                        token(&component.aggregation_window),
                        component.as_of.clone(),
                        token(&component.freshness),
                        token(&component.value_presentation),
                        component.carries_raw_number.to_string(),
                        effective.clone(),
                    ];
                    out.push_str(&row.join(","));
                    out.push('\n');
                }
            }
        }
        out
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`ChargebackScopeViewViolation`] per failed invariant: a wrong
    /// record kind or schema version, a missing identifier, a duplicate view, an
    /// incomplete service-family set, a collapsed scope, a missing or mislabeled
    /// direct/inherited line, an inherited line whose parent drifts from the
    /// recomputed chain, a bare value, a missing CSV/JSON export parity, an empty
    /// local-safe baseline, a switcher that drops a preservation guarantee or
    /// names an unavailable active scope, a stored effective claim that does not
    /// match the recomputation, a missing recovery cue on a narrowed view, an
    /// unbound surface, or a stale inspection block.
    pub fn validate(&self) -> Vec<ChargebackScopeViewViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(ChargebackScopeViewViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != VIEW_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
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
            .any(|entry| entry == CHARGEBACK_SCOPE_VIEWS_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.views.is_empty() {
            push("views", "set must contain at least one view");
        }

        self.validate_switcher(&mut push);

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
                push("views", "every service family must carry a chargeback view");
                break;
            }
        }
        for family in MeterFamily::ALL {
            if !self.views.iter().any(|v| v.meter_family == family) {
                push("views", "every meter family must carry a chargeback view");
                break;
            }
        }

        self.validate_surface_bindings(&mut push);

        let derived = ChargebackScopeInspection::derive(
            &self.switcher,
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

    fn validate_switcher(&self, push: &mut impl FnMut(&str, &str)) {
        let s = &self.switcher;
        if s.record_kind != SWITCHER_RECORD_KIND {
            push("switcher.record_kind", "switcher record_kind is wrong");
        }
        if s.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
            push(
                "switcher.schema_version",
                "switcher schema_version is wrong",
            );
        }
        if s.switcher_id.trim().is_empty() {
            push("switcher.switcher_id", "switcher_id must be non-empty");
        }
        if s.summary.trim().is_empty() {
            push("switcher.summary", "switcher summary must be non-empty");
        }
        if s.available_scopes.is_empty() {
            push(
                "switcher.available_scopes",
                "switcher must offer at least one scope",
            );
        }
        let mut seen = BTreeSet::new();
        for scope in &s.available_scopes {
            if !seen.insert(*scope) {
                push(
                    "switcher.available_scopes",
                    "available scopes must be distinct, not collapsed",
                );
            }
        }
        if !s.available_scopes.contains(&s.active_scope) {
            push(
                "switcher.active_scope",
                "the active scope must be one the switcher offers",
            );
        }
        for (field, flag) in [
            ("switcher.preserves_active_scope", s.preserves_active_scope),
            (
                "switcher.preserves_inherited_direct_separation",
                s.preserves_inherited_direct_separation,
            ),
            (
                "switcher.preserves_owner_identity",
                s.preserves_owner_identity,
            ),
            ("switcher.never_collapses_scopes", s.never_collapses_scopes),
        ] {
            if !flag {
                push(field, "a scope switch must preserve this guarantee");
            }
        }
    }

    fn validate_view(&self, view: &ChargebackScopeView, push: &mut impl FnMut(&str, &str)) {
        if view.record_kind != VIEW_RECORD_KIND {
            push("view.record_kind", "view record_kind is wrong");
        }
        if view.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
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

        // Personal, workspace, team, and org scopes never collapse into one total.
        if view.scope_cost_truths.len() < 2 {
            push(
                "view.scope_cost_truths",
                "a chargeback view must keep at least two distinct scopes, never one total",
            );
        }
        let offered: Vec<ScopeOwner> = view
            .scope_cost_truths
            .iter()
            .map(|t| t.scope_owner)
            .collect();
        let mut seen_scopes = BTreeSet::new();
        for scope in &offered {
            if !seen_scopes.insert(*scope) {
                push(
                    "view.scope_cost_truths",
                    "chargeback scopes must be distinct, not collapsed",
                );
            }
        }
        for truth in &view.scope_cost_truths {
            self.validate_scope_truth(view, truth, &offered, push);
        }

        // Every view exports at CSV/JSON parity.
        let e = &view.export_parity;
        if !(e.csv && e.json && e.parity_confirmed) {
            push(
                "view.export_parity",
                "a chargeback view must export at confirmed CSV/JSON parity",
            );
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

    fn validate_scope_truth(
        &self,
        view: &ChargebackScopeView,
        truth: &ScopeCostTruth,
        offered: &[ScopeOwner],
        push: &mut impl FnMut(&str, &str),
    ) {
        if truth.record_kind != SCOPE_COST_TRUTH_RECORD_KIND {
            push(
                "scope_cost_truth.record_kind",
                "scope cost-truth record_kind is wrong",
            );
        }
        if truth.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
            push(
                "scope_cost_truth.schema_version",
                "scope cost-truth schema_version is wrong",
            );
        }
        if truth.summary.trim().is_empty() {
            push(
                "scope_cost_truth.summary",
                "scope cost-truth summary must be non-empty",
            );
        }
        // Owner identity is present and opaque; never a raw tenant or account name.
        if truth.owner_identity.trim().is_empty() {
            push(
                "scope_cost_truth.owner_identity",
                "each scope must carry a non-empty owner identity",
            );
        }
        if truth.owner_identity.contains(',') {
            push(
                "scope_cost_truth.owner_identity",
                "owner identity must not contain a comma so the CSV export stays parseable",
            );
        }

        // The direct line is charged directly; the inherited line is inherited.
        self.validate_component(
            view,
            truth,
            &truth.direct,
            AttributionBasis::Direct,
            offered,
            "scope_cost_truth.direct",
            push,
        );
        self.validate_component(
            view,
            truth,
            &truth.inherited,
            AttributionBasis::Inherited,
            offered,
            "scope_cost_truth.inherited",
            push,
        );
    }

    // The cost-line check pins many fixed columns; a wide signature keeps the
    // direct and inherited checks reading as one routine.
    #[allow(clippy::too_many_arguments)]
    fn validate_component(
        &self,
        view: &ChargebackScopeView,
        truth: &ScopeCostTruth,
        component: &CostAttributionMeasurement,
        expected_basis: AttributionBasis,
        offered: &[ScopeOwner],
        field: &str,
        push: &mut impl FnMut(&str, &str),
    ) {
        if component.record_kind != COST_ATTRIBUTION_RECORD_KIND {
            push(field, "cost-attribution record_kind is wrong");
        }
        if component.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
            push(field, "cost-attribution schema_version is wrong");
        }
        if component.attribution_basis != expected_basis {
            push(field, "cost line carries the wrong attribution basis");
        }
        if component.carries_raw_number {
            push(
                field,
                "a cost line must never carry a raw spend or quota number",
            );
        }
        if component.as_of.trim().is_empty() {
            push(field, "a cost line must carry an as-of time");
        }
        if component.scope_owner != truth.scope_owner {
            push(field, "cost line scope owner must match the scope truth");
        }
        if component.meter_unit != view.meter_unit {
            push(field, "cost line meter unit must match the view");
        }
        if component.aggregation_window != view.aggregation_window {
            push(field, "cost line aggregation window must match the view");
        }

        match expected_basis {
            AttributionBasis::Direct => {
                // A direct line is always shown bound and never names a parent.
                if component.inherited_from.is_some() {
                    push(field, "a direct cost line must not name a parent scope");
                }
                if component.value_presentation
                    != ValuePresentation::MonthToDateBoundToUnitAsOfScope
                {
                    push(
                        field,
                        "a direct cost line must bind its value to the unit, as-of time, and scope",
                    );
                }
            }
            AttributionBasis::Inherited => {
                // The inherited line names the recomputed parent, or explicitly
                // marks the chain root not applicable rather than a hidden zero.
                let expected_parent = parent_scope_in(truth.scope_owner, offered);
                if component.inherited_from != expected_parent {
                    push(
                        field,
                        "inherited parent scope drifted from the recomputed inheritance chain",
                    );
                }
                match expected_parent {
                    Some(_) => {
                        if component.value_presentation
                            != ValuePresentation::MonthToDateBoundToUnitAsOfScope
                        {
                            push(
                                field,
                                "an inherited cost line with a parent must bind its value",
                            );
                        }
                    }
                    None => {
                        if component.value_presentation
                            != ValuePresentation::SuppressedNoManagedNumber
                        {
                            push(
                                field,
                                "the chain-root inherited line must explicitly suppress its value, not imply a zero",
                            );
                        }
                    }
                }
            }
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
            if binding.schema_version != CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION {
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
            if !binding.renders_inherited_direct_separation {
                push(
                    "surface_binding.renders_inherited_direct_separation",
                    "a surface must render the inherited-versus-direct separation",
                );
            }
            if !binding.renders_local_safe_baseline {
                push(
                    "surface_binding.renders_local_safe_baseline",
                    "a surface must render the local-safe baseline",
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
        for surface in ChargebackScopeSurface::ALL {
            if !self.surface_bindings.iter().any(|b| b.surface == surface) {
                push(
                    "surface_bindings",
                    "account-chargeback, service-health, Help/About, support/admin, and release-center must all bind",
                );
                break;
            }
        }
    }

    /// Cross-checks every view against its control-plane lane.
    ///
    /// Confirms each [`ChargebackScopeView`] projects the canonical
    /// commercial-control-plane matrix lane named by its
    /// [`ChargebackScopeView::lane_ref`] — the service family, meter family, meter
    /// unit, aggregation window, service ids, and applicable managed states must
    /// match, the lane's primary scope owner must appear among the view's scopes,
    /// and every chargeback scope the lane offers must be present — so the
    /// chargeback surface is a real consumer of the matrix rather than a parallel
    /// spreadsheet.
    ///
    /// Returns an empty vector when every view matches its lane.
    pub fn cross_check_against_control_plane(&self) -> Vec<ChargebackScopeViewViolation> {
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let mut violations = Vec::new();
        for view in &self.views {
            let Some(lane) = matrix.lanes.iter().find(|l| l.lane_id == view.lane_ref) else {
                violations.push(ChargebackScopeViewViolation {
                    field: "view.lane_ref".to_owned(),
                    message: format!(
                        "lane_ref {} does not resolve to a control-plane lane",
                        view.lane_ref
                    ),
                });
                continue;
            };
            let mut mismatch = |field: &str| {
                violations.push(ChargebackScopeViewViolation {
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
            if view.meter_unit != lane.meter_unit {
                mismatch("view.meter_unit");
            }
            if view.aggregation_window != lane.aggregation_window {
                mismatch("view.aggregation_window");
            }
            if view.linked_service_ids != lane.linked_service_ids {
                mismatch("view.linked_service_ids");
            }
            if view.applicable_managed_states != lane.applicable_managed_states {
                mismatch("view.applicable_managed_states");
            }
            let scopes: BTreeSet<ScopeOwner> = view
                .scope_cost_truths
                .iter()
                .map(|t| t.scope_owner)
                .collect();
            // The lane's primary owner and every chargeback scope it offers must
            // be represented; the view may add finer scopes (for example, team).
            if !scopes.contains(&lane.scope_owner) {
                mismatch("view.scope_cost_truths.scope_owner");
            }
            for offered in &lane.chargeback_scope_offers {
                if !scopes.contains(offered) {
                    mismatch("view.scope_cost_truths.chargeback_scope_offers");
                    break;
                }
            }
        }
        violations
    }
}

impl ChargebackScopeInspection {
    fn derive(
        switcher: &ChargebackScopeSwitcher,
        views: &[ChargebackScopeView],
        surface_bindings: &[ChargebackScopeSurfaceBinding],
        active_managed_state: Option<ManagedStateClass>,
    ) -> Self {
        let service_families: BTreeSet<ServiceFamily> =
            views.iter().map(|v| v.service_family).collect();
        let scope_owners: BTreeSet<ScopeOwner> = views
            .iter()
            .flat_map(|v| v.scope_cost_truths.iter().map(|t| t.scope_owner))
            .collect();

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
            schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
            view_count: views.len(),
            surface_binding_count: surface_bindings.len(),
            service_families_covered: service_families.len(),
            scope_owners_covered: scope_owners.len(),
            all_views_separate_direct_and_inherited: views.iter().all(|v| {
                v.scope_cost_truths.iter().all(|t| {
                    t.direct.attribution_basis == AttributionBasis::Direct
                        && t.inherited.attribution_basis == AttributionBasis::Inherited
                })
            }),
            all_views_export_csv_json_parity: views.iter().all(|v| {
                v.export_parity.csv && v.export_parity.json && v.export_parity.parity_confirmed
            }),
            value_never_bare: views.iter().all(|v| {
                v.scope_cost_truths
                    .iter()
                    .all(|t| !t.direct.carries_raw_number && !t.inherited.carries_raw_number)
            }),
            no_collapsed_scope_total: views.iter().all(|v| {
                let set: BTreeSet<ScopeOwner> =
                    v.scope_cost_truths.iter().map(|t| t.scope_owner).collect();
                set.len() == v.scope_cost_truths.len() && set.len() >= 2
            }),
            all_views_local_safe_backed: views.iter().all(|v| !v.local_safe_baseline.is_empty()),
            switcher_preserves_active_scope: switcher.preserves_active_scope,
            switcher_active_scope_available: switcher
                .available_scopes
                .contains(&switcher.active_scope),
            active_scope: switcher.active_scope,
            effective_full_view_count,
            narrowed_view_count,
            local_safe_only_view_count,
            active_managed_state,
        }
    }
}

/// One failed view-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChargebackScopeViewViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for ChargebackScopeViewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum ChargebackScopeViewError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<ChargebackScopeViewViolation>),
}

impl fmt::Display for ChargebackScopeViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "chargeback-scope view set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "chargeback-scope view set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for ChargebackScopeViewError {}

/// Scope breadth from narrowest to broadest, used to resolve the parent scope a
/// child scope inherits cost from. A bring-your-own-key external account is
/// orthogonal to the hierarchy and never appears in a chargeback view.
const SCOPE_BREADTH_ORDER: [ScopeOwner; 6] = [
    ScopeOwner::Personal,
    ScopeOwner::Workspace,
    ScopeOwner::Team,
    ScopeOwner::Organization,
    ScopeOwner::Tenant,
    ScopeOwner::ByokExternal,
];

fn scope_breadth_rank(scope: ScopeOwner) -> usize {
    SCOPE_BREADTH_ORDER
        .iter()
        .position(|s| *s == scope)
        .unwrap_or(SCOPE_BREADTH_ORDER.len())
}

/// The scope a `scope` inherits from within `offered`: the narrowest scope in
/// `offered` strictly broader than `scope`. Returns `None` when `scope` is the
/// broadest offered scope — the inheritance-chain root.
fn parent_scope_in(scope: ScopeOwner, offered: &[ScopeOwner]) -> Option<ScopeOwner> {
    let rank = scope_breadth_rank(scope);
    offered
        .iter()
        .copied()
        .filter(|s| scope_breadth_rank(*s) > rank)
        .min_by_key(|s| scope_breadth_rank(*s))
}

/// Snake-case token for a closed-vocabulary value, used for the CSV export.
fn token<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("closed-vocabulary token serializes")
        .trim_matches('"')
        .to_owned()
}

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
        ManagedStateClass::SignedIn => "Managed chargeback views continue; no recovery needed.",
        ManagedStateClass::LocalOnly => {
            "Sign in to a managed account to see managed chargeback scopes; local work continues now."
        }
        ManagedStateClass::ReauthRequired => {
            "Reauthenticate to refresh the chargeback scope views; local work continues now."
        }
        ManagedStateClass::ManagedBlocked => {
            "Review the account or policy hold to restore the chargeback scope views; local work continues now."
        }
        ManagedStateClass::GracePeriod => {
            "Export the per-scope CSV/JSON chargeback breakdown before the grace window closes; local work continues now."
        }
        ManagedStateClass::SeatRemoved => {
            "Ask an admin to restore the seat to resume the chargeback scope views; local work continues now."
        }
        ManagedStateClass::PlanDowngrade => {
            "The chargeback view is on the plan floor; upgrade the plan to widen it. Local work continues now."
        }
        ManagedStateClass::OrgSwitched => {
            "The chargeback scopes are rebinding to the new org; local work continues now."
        }
        ManagedStateClass::ForecastThreshold => {
            "Usage is approaching the budget; the chargeback view stays inspectable while new managed work narrows. Local work continues now."
        }
        ManagedStateClass::MeterStale => {
            "The metered cost is stale and labeled; the chargeback view refreshes when the meter reconnects. Local work continues now."
        }
    }
    .to_owned()
}

/// Reads and validates the checked-in stable chargeback-scope view set.
///
/// This is the canonical reader: the account chargeback surface, service-health
/// diagnostics, Help/About, the support/admin export, and the release center call
/// it to ingest the views rather than cloning status text.
///
/// # Errors
///
/// Returns [`ChargebackScopeViewError`] when the checked-in packet fails to parse
/// or fails validation.
pub fn current_stable_chargeback_scope_view_set(
) -> Result<ChargebackScopeViewSet, ChargebackScopeViewError> {
    let set: ChargebackScopeViewSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-chargeback-scope-views.json"
    )))
    .map_err(ChargebackScopeViewError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(ChargebackScopeViewError::Validation(violations))
    }
}

/// Source refs every chargeback-scope export carries.
fn chargeback_scope_source_refs() -> Vec<String> {
    let mut refs = vec![
        CHARGEBACK_SCOPE_VIEWS_SCHEMA_REF.to_owned(),
        CHARGEBACK_SCOPE_VIEWS_DOC_REF.to_owned(),
    ];
    // Reuse the control-plane refs so the views cite the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

/// Reviewable, lowercase display name for a scope owner.
fn scope_display(scope: ScopeOwner) -> &'static str {
    match scope {
        ScopeOwner::Personal => "personal",
        ScopeOwner::Workspace => "workspace",
        ScopeOwner::Team => "team",
        ScopeOwner::Organization => "organization",
        ScopeOwner::Tenant => "tenant",
        ScopeOwner::ByokExternal => "bring-your-own-key external account",
    }
}

fn cost_measurement(
    basis: AttributionBasis,
    inherited_from: Option<ScopeOwner>,
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    scope_owner: ScopeOwner,
    freshness: SnapshotFreshness,
) -> CostAttributionMeasurement {
    // A direct line, or an inherited line with a parent, presents a bound
    // value; the chain-root inherited line explicitly suppresses the number
    // rather than implying a hidden zero.
    let value_presentation = match basis {
        AttributionBasis::Direct => ValuePresentation::MonthToDateBoundToUnitAsOfScope,
        AttributionBasis::Inherited if inherited_from.is_some() => {
            ValuePresentation::MonthToDateBoundToUnitAsOfScope
        }
        AttributionBasis::Inherited => ValuePresentation::SuppressedNoManagedNumber,
    };
    CostAttributionMeasurement {
        record_kind: COST_ATTRIBUTION_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        attribution_basis: basis,
        inherited_from,
        meter_unit,
        aggregation_window,
        scope_owner,
        as_of: STABLE_MEASUREMENT_AS_OF.to_owned(),
        freshness,
        value_presentation,
        carries_raw_number: false,
    }
}

fn scope_cost_truth(
    scope: ScopeOwner,
    owner_identity: String,
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    freshness: SnapshotFreshness,
    offered: &[ScopeOwner],
) -> ScopeCostTruth {
    let parent = parent_scope_in(scope, offered);
    let summary = match parent {
        Some(p) => format!(
            "The {} scope's cost is shown as a direct line plus an inherited share rolled up from the {} pool, each bound to its unit, as-of time, and scope owner.",
            scope_display(scope),
            scope_display(p)
        ),
        None => format!(
            "The {} scope's cost is shown as a direct line only; it is the chain root, so the inherited line is explicitly not applicable rather than a hidden zero.",
            scope_display(scope)
        ),
    };
    ScopeCostTruth {
        record_kind: SCOPE_COST_TRUTH_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        scope_owner: scope,
        owner_identity,
        direct: cost_measurement(
            AttributionBasis::Direct,
            None,
            meter_unit,
            aggregation_window,
            scope,
            freshness,
        ),
        inherited: cost_measurement(
            AttributionBasis::Inherited,
            parent,
            meter_unit,
            aggregation_window,
            scope,
            freshness,
        ),
        summary,
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
    family_token: &str,
    service_family: ServiceFamily,
    meter_family: MeterFamily,
    linked_service_ids: &[ServiceId],
    meter_unit: MeterUnit,
    aggregation_window: AggregationWindow,
    freshness: SnapshotFreshness,
    scopes: &[ScopeOwner],
    local_safe_baseline: &[&str],
    blocked_managed_only_actions: &[&str],
) -> ChargebackScopeView {
    let scope_cost_truths = scopes
        .iter()
        .map(|scope| {
            let owner_identity = format!(
                "scope-owner.{}.{}.opaque",
                family_token,
                scope_token(*scope)
            );
            scope_cost_truth(
                *scope,
                owner_identity,
                meter_unit,
                aggregation_window,
                freshness,
                scopes,
            )
        })
        .collect();
    ChargebackScopeView {
        record_kind: VIEW_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        view_id: view_id.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        lane_ref: lane_ref.to_owned(),
        service_family,
        meter_family,
        meter_unit,
        aggregation_window,
        linked_service_ids: linked_service_ids.to_vec(),
        scope_cost_truths,
        export_parity: ExportParity::csv_json_parity(),
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

/// Stable, snake-case token for a scope owner.
fn scope_token(scope: ScopeOwner) -> &'static str {
    match scope {
        ScopeOwner::Personal => "personal",
        ScopeOwner::Workspace => "workspace",
        ScopeOwner::Team => "team",
        ScopeOwner::Organization => "organization",
        ScopeOwner::Tenant => "tenant",
        ScopeOwner::ByokExternal => "byok_external",
    }
}

fn binding(
    binding_id: &str,
    surface: ChargebackScopeSurface,
    bound_view_ids: &[&str],
    summary: &str,
) -> ChargebackScopeSurfaceBinding {
    ChargebackScopeSurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        surface,
        bound_view_ids: bound_view_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_inherited_direct_separation: true,
        renders_local_safe_baseline: true,
        summary: summary.to_owned(),
    }
}

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "chargeback-scope-views:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Chargeback scope switchers and personal/workspace/team/org cost truth with inherited/direct separation and CSV/JSON exports";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Deterministic as-of time for the checked-in cost measurements.
pub const STABLE_MEASUREMENT_AS_OF: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// The scope the switcher starts active in the checked-in set.
pub const STABLE_ACTIVE_SCOPE: ScopeOwner = ScopeOwner::Organization;

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_chargeback_scope_view_set() -> ChargebackScopeViewSet {
    canonical_chargeback_scope_view_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen chargeback-scope view set.
///
/// The set freezes one view per claimed service family — the AI gateway, settings
/// sync, the companion relay, the registry/mirror surface, support ingest, and the
/// managed workspace — and a single scope switcher across them. Each view projects
/// its control-plane lane, keeps one cost truth per offered chargeback scope with
/// a separated direct and inherited line, exports at CSV/JSON parity, and keeps a
/// non-empty local-safe baseline. All views start at their full managed claim with
/// no active managed state, and the switcher starts on the organization scope;
/// callers use [`ChargebackScopeViewSet::apply_managed_state`] to exercise
/// narrowing and [`ChargebackScopeViewSet::switch_scope`] to move the active scope.
pub fn canonical_chargeback_scope_view_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> ChargebackScopeViewSet {
    let views = vec![
        view(
            "chargeback_scope.ai_gateway",
            "Managed AI gateway cost ownership",
            "Per-scope managed AI token cost across personal, workspace, team, and organization scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; direct and bring-your-own-key AI routes continue when the managed lane narrows.",
            "managed_lane.ai_gateway",
            "ai_gateway",
            ServiceFamily::AiGatewayFamily,
            MeterFamily::AiGatewayMeterFamily,
            &[ServiceId::ManagedAiBroker],
            MeterUnit::Tokens,
            AggregationWindow::CalendarMonthUtc,
            SnapshotFreshness::FreshnessLive,
            &[
                ScopeOwner::Personal,
                ScopeOwner::Workspace,
                ScopeOwner::Team,
                ScopeOwner::Organization,
            ],
            &[
                "Direct and bring-your-own-key AI routes keep running.",
                "Local editing, search, and Git are unaffected.",
            ],
            &["New managed-broker inference once the monthly token budget is exhausted."],
        ),
        view(
            "chargeback_scope.settings_sync",
            "Managed settings sync cost ownership",
            "Per-scope managed settings-sync storage cost across personal, workspace, team, and organization scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; local settings and files stay authoritative when the lane narrows.",
            "managed_lane.settings_sync",
            "settings_sync",
            ServiceFamily::SyncFamily,
            MeterFamily::ProfileOrSettingsSyncMeterFamily,
            &[ServiceId::ManagedSettingsSync],
            MeterUnit::BytesStored,
            AggregationWindow::Rolling30d,
            SnapshotFreshness::FreshnessRecent,
            &[
                ScopeOwner::Personal,
                ScopeOwner::Workspace,
                ScopeOwner::Team,
                ScopeOwner::Organization,
            ],
            &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the lane clears.",
            ],
            &["Pushing new settings snapshots to the managed store once storage is exhausted."],
        ),
        view(
            "chargeback_scope.companion_relay",
            "Companion relay cost ownership",
            "Per-scope managed relay cost across workspace, team, and organization scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; local incident notes and offline packets continue when the relay narrows.",
            "managed_lane.companion_relay",
            "companion_relay",
            ServiceFamily::CollaborationRelayFamily,
            MeterFamily::CollaborationRelayMeterFamily,
            &[ServiceId::ManagedRelay],
            MeterUnit::ParticipantMinutes,
            AggregationWindow::Rolling24h,
            SnapshotFreshness::FreshnessRecent,
            &[
                ScopeOwner::Workspace,
                ScopeOwner::Team,
                ScopeOwner::Organization,
            ],
            &[
                "Local incident notes and offline packets continue.",
                "Desktop handoff resumes the exact local context.",
            ],
            &["Joining a live companion-follow or relay session once relay minutes are exhausted."],
        ),
        view(
            "chargeback_scope.registry_mirror",
            "Registry and mirror cost ownership",
            "Per-scope managed registry and mirror cost across team, organization, and tenant scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; installed extensions and local or sideloaded packages keep running when the lane narrows.",
            "managed_lane.registry_mirror",
            "registry_mirror",
            ServiceFamily::RegistryOrMirrorMetadataFamily,
            MeterFamily::RegistryOrMirrorMeterFamily,
            &[ServiceId::ManagedMarketplace, ServiceId::ManagedCatalog],
            MeterUnit::DownloadCount,
            AggregationWindow::CalendarMonthUtc,
            SnapshotFreshness::FreshnessAging,
            &[
                ScopeOwner::Team,
                ScopeOwner::Organization,
                ScopeOwner::Tenant,
            ],
            &[
                "Installed extensions keep running.",
                "Local and sideloaded packages are unaffected.",
            ],
            &["New managed-registry installs or publishes once the monthly download budget is exhausted."],
        ),
        view(
            "chargeback_scope.support_ingest",
            "Support ingest cost ownership",
            "Per-scope managed support-ingest cost across team, organization, and tenant scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; local support bundles still generate when the lane narrows.",
            "managed_lane.support_ingest",
            "support_ingest",
            ServiceFamily::TelemetryOrSupportIngestFamily,
            MeterFamily::SupportIngestMeterFamily,
            &[ServiceId::ManagedSupportExport, ServiceId::ManagedTelemetrySink],
            MeterUnit::SupportBundleCount,
            AggregationWindow::Rolling30d,
            SnapshotFreshness::FreshnessStale,
            &[
                ScopeOwner::Team,
                ScopeOwner::Organization,
                ScopeOwner::Tenant,
            ],
            &[
                "Local support bundles still generate on device.",
                "Offline evidence capture continues.",
            ],
            &["Uploading new support bundles to the managed sink once the ingest budget is exhausted."],
        ),
        view(
            "chargeback_scope.managed_workspace",
            "Managed workspace cost ownership",
            "Per-scope managed workspace-hour cost across workspace, team, and organization scopes, each separating a direct line from an inherited share, bound to its unit, as-of time, and scope owner, with CSV/JSON export parity; local checkout, editing, tasks, and Git continue when the remote workspace narrows.",
            "managed_lane.managed_workspace",
            "managed_workspace",
            ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            MeterFamily::RemoteWorkspaceControlPlaneMeterFamily,
            &[ServiceId::ManagedRelay],
            MeterUnit::WorkspaceHours,
            AggregationWindow::CalendarMonthUtc,
            SnapshotFreshness::FreshnessLive,
            &[
                ScopeOwner::Workspace,
                ScopeOwner::Team,
                ScopeOwner::Organization,
            ],
            &[
                "Local checkout and editing continue.",
                "Local tasks and Git are unaffected when the remote workspace narrows.",
            ],
            &["Attaching or running a new remote workspace once the workspace-hour budget is exhausted."],
        ),
    ];

    let all_view_ids: Vec<&str> = views.iter().map(|v| v.view_id.as_str()).collect();

    let switcher = ChargebackScopeSwitcher {
        record_kind: SWITCHER_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        switcher_id: "chargeback_scope.switcher".to_owned(),
        available_scopes: vec![
            ScopeOwner::Personal,
            ScopeOwner::Workspace,
            ScopeOwner::Team,
            ScopeOwner::Organization,
            ScopeOwner::Tenant,
        ],
        active_scope: STABLE_ACTIVE_SCOPE,
        preserves_active_scope: true,
        preserves_inherited_direct_separation: true,
        preserves_owner_identity: true,
        never_collapses_scopes: true,
        summary: "Switching the active chargeback scope preserves the active scope, the inherited-versus-direct separation, and each scope's owner identity, and never collapses personal, workspace, team, and organization into one ambiguous total.".to_owned(),
    };

    let surface_bindings = vec![
        binding(
            "chargeback_surface.account",
            ChargebackScopeSurface::AccountChargebackSurface,
            &all_view_ids,
            "The account chargeback surface renders each view's per-scope direct and inherited cost, bound to its unit, as-of time, and scope owner, with the CSV/JSON export action and the scope switcher.",
        ),
        binding(
            "chargeback_surface.service_health",
            ChargebackScopeSurface::ServiceHealthDiagnostics,
            &all_view_ids,
            "Service-health and diagnostics surfaces project the per-scope freshness and the inherited-versus-direct separation without inventing a stronger claim.",
        ),
        binding(
            "chargeback_surface.help_about",
            ChargebackScopeSurface::HelpAbout,
            &[
                "chargeback_scope.ai_gateway",
                "chargeback_scope.settings_sync",
                "chargeback_scope.registry_mirror",
            ],
            "The Help/About truth surface names which managed lanes carry a per-scope chargeback view and their local-safe baseline.",
        ),
        binding(
            "chargeback_surface.support_admin",
            ChargebackScopeSurface::SupportAdminExport,
            &[
                "chargeback_scope.support_ingest",
                "chargeback_scope.companion_relay",
                "chargeback_scope.managed_workspace",
            ],
            "Support and admin export packets carry the per-scope CSV/JSON chargeback breakdown, its as-of time and scope owner, the inherited-versus-direct separation, and the posture origin of any narrowing.",
        ),
        binding(
            "chargeback_surface.release_center",
            ChargebackScopeSurface::ReleaseCenter,
            &all_view_ids,
            "The release center narrows the marketed chargeback claim to each view's effective claim when the active managed state caps it.",
        ),
    ];

    let inspection = ChargebackScopeInspection::derive(&switcher, &views, &surface_bindings, None);

    let summary = "Frozen per-scope chargeback views for the managed lanes. Each view keeps one \
        cost truth per offered scope — personal, workspace, team, organization, and tenant never \
        collapse into one ambiguous owner total — separates a direct line from an inherited share \
        that names its parent scope, binds every value to its unit, as-of time, and scope owner, \
        and exports at CSV/JSON parity. A scope switcher holds the active scope across the views \
        while preserving the inherited-versus-direct separation and owner identity, and every view \
        keeps a local-safe baseline that continues when the managed lane narrows."
        .to_owned();

    ChargebackScopeViewSet {
        record_kind: VIEW_SET_RECORD_KIND.to_owned(),
        schema_version: CHARGEBACK_SCOPE_VIEWS_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary,
        source_refs: chargeback_scope_source_refs(),
        active_managed_state: None,
        switcher,
        views,
        surface_bindings,
        inspection,
    }
}
