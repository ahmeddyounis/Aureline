//! Stabilize effective policy, remembered-decision, waiver-expiry, and
//! exception-preview UX.
//!
//! This module promotes the beta policy simulation page in
//! [`crate::simulation`] to an evidence-backed stable proof packet whose
//! qualification is derived from the audit rather than asserted from a
//! spreadsheet row.
//!
//! The stable claim holds when **all** of the following conditions are
//! verified:
//!
//! 1. The upstream policy simulation beta page audits with zero defects.
//! 2. Both required change classes (`policy_bundle_change` and
//!    `settings_lock_change`) have at least one simulation with complete
//!    affected-surface truth (personas, actions, degraded modes, and
//!    protected-path changes).
//! 3. Every exception or waiver has an explicit expiry horizon, a named
//!    renewal path, a revocation path, and an owner; dashboard buckets
//!    reflect current lifecycle status.
//! 4. Every remembered decision is narrowly bound (actor, object, action
//!    family, environment, time horizon) and any drift is explained with at
//!    least one typed [`RememberedDecisionDriftReason`][crate::simulation::RememberedDecisionDriftReason].
//! 5. Simulation records link to overlapping exceptions and remembered
//!    decisions via `exception_preview_refs` and
//!    `remembered_decision_preview_refs` so the exception-preview UX can be
//!    populated from typed records rather than cloned status text.
//! 6. Action-time policy snapshots preserve historical truth for support and
//!    admin exports rather than overwriting it with current-only truth.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - A [`PolicySimulationBetaDefectKind::RawPrivateMaterialExposed`][crate::simulation::PolicySimulationBetaDefectKind::RawPrivateMaterialExposed]
//!   defect in the upstream beta page (narrow reason:
//!   [`EffectivePolicyStabilizeNarrowReasonClass::RawPrivateMaterialExposed`]).
//!
//! A missing required change class narrows to `Preview` (not `Beta`) because
//! the coverage gap prevents any verifiable claim for that class.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only. Raw policy bundle bodies, raw rule text, raw identities,
//! raw credentials, and secret material stay outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md`
//! - Artifact: `artifacts/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md`
//! - Contract ref: [`EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::simulation::{
    audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page,
    PolicySimulationBetaDefectKind, PolicySimulationBetaPage, PolicySimulationSupportExport,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF: &str =
    "policy:effective_policy_stabilize:v1";

/// Record-kind tag for [`EffectivePolicyStabilizePage`] payloads.
pub const EFFECTIVE_POLICY_STABILIZE_PAGE_RECORD_KIND: &str =
    "policy_effective_policy_stabilize_page_record";

/// Record-kind tag for [`EffectivePolicyStabilizeRow`] payloads.
pub const EFFECTIVE_POLICY_STABILIZE_ROW_RECORD_KIND: &str =
    "policy_effective_policy_stabilize_row_record";

/// Record-kind tag for [`EffectivePolicyStabilizeDefect`] payloads.
pub const EFFECTIVE_POLICY_STABILIZE_DEFECT_RECORD_KIND: &str =
    "policy_effective_policy_stabilize_defect_record";

/// Record-kind tag for [`EffectivePolicyStabilizeSupportExport`] payloads.
pub const EFFECTIVE_POLICY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_effective_policy_stabilize_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const EFFECTIVE_POLICY_STABILIZE_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md";

/// Repo-relative path of the artifact summary for this lane.
pub const EFFECTIVE_POLICY_STABILIZE_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md";

/// Repo-relative path of the upstream beta simulation contract.
pub const POLICY_SIMULATION_BETA_CONTRACT_REF: &str =
    "docs/verification/policy_simulation_packet.md";

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// lists from the beta page against the six stability conditions. A caller may
/// never bump a row to `stable` without a clean audit and complete coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectivePolicyStabilizeQualificationClass {
    /// All six stability conditions hold and the upstream audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required change class has no simulation; coverage gap prevents a
    /// beta claim for the missing class.
    Preview,
    /// Raw private material was exposed in the upstream beta page; the row is
    /// withdrawn entirely and cannot be overridden.
    Withdrawn,
}

impl EffectivePolicyStabilizeQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`EffectivePolicyStabilizeQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectivePolicyStabilizeNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The upstream policy simulation beta page has one or more defects.
    PolicySimulationBetaPageHasDefects,
    /// A required change class (`policy_bundle_change` or
    /// `settings_lock_change`) has no simulation; narrows to preview.
    RequiredChangeClassMissing,
    /// Exceptions exist in the beta page but no simulation links them via
    /// `exception_preview_refs`.
    ExceptionPreviewLinksMissing,
    /// An exception or waiver in the beta page is missing a bounded expiry
    /// horizon.
    ExceptionMissingBoundedExpiry,
    /// A drifted or expired remembered decision does not name invalidation
    /// reasons.
    RememberedDecisionDriftUnexplained,
    /// Action-time policy snapshots do not preserve historical truth.
    SupportExportDropsHistoricalTruth,
    /// Raw private material was exposed in the upstream beta page; withdraws
    /// the packet immediately.
    RawPrivateMaterialExposed,
}

impl EffectivePolicyStabilizeNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::PolicySimulationBetaPageHasDefects => {
                "policy_simulation_beta_page_has_defects"
            }
            Self::RequiredChangeClassMissing => "required_change_class_missing",
            Self::ExceptionPreviewLinksMissing => "exception_preview_links_missing",
            Self::ExceptionMissingBoundedExpiry => "exception_missing_bounded_expiry",
            Self::RememberedDecisionDriftUnexplained => "remembered_decision_drift_unexplained",
            Self::SupportExportDropsHistoricalTruth => "support_export_drops_historical_truth",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawPrivateMaterialExposed)
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect types
// ---------------------------------------------------------------------------

/// Stability qualification for one simulation row in the stabilize packet.
///
/// Each row is bound to a single [`crate::simulation::PolicySimulationRecord`]
/// from the upstream beta page. The qualification is derived from the
/// combined audit of that simulation and the shared exception, memory, and
/// action-time snapshot state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyStabilizeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Simulation id from the upstream beta page.
    pub simulation_id: String,
    /// Change class token for this simulation.
    pub change_class_token: String,
    /// Scope id string from the simulation request.
    pub scope_ref_id: String,
    /// Number of exception preview refs linked in the simulation.
    pub exception_preview_count: usize,
    /// Number of exceptions or waivers in the page that are expiring soon.
    pub expiring_exception_count: usize,
    /// Number of remembered decision refs linked in the simulation.
    pub remembered_decision_count: usize,
    /// Number of linked remembered decisions with non-empty invalidation
    /// reasons (drift detected).
    pub drifted_remembered_decision_count: usize,
    /// True when the page has at least one action-time policy snapshot for
    /// this simulation's action family.
    pub action_time_policy_state_present: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the stabilize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EffectivePolicyStabilizeSummary {
    /// Total row count (one row per simulation in the beta page).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of defects in the upstream beta page.
    pub simulation_beta_page_defect_count: usize,
    /// Change class tokens covered by the simulations.
    pub change_classes_covered: Vec<String>,
    /// Number of exceptions or waivers expiring soon across the page.
    pub expiring_exception_count: usize,
    /// Number of remembered decisions with drift detected.
    pub drifted_remembered_decision_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl EffectivePolicyStabilizeSummary {
    fn from_rows(rows: &[EffectivePolicyStabilizeRow], beta_page: &PolicySimulationBetaPage) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            EffectivePolicyStabilizeQualificationClass::Withdrawn
        } else if preview > 0 {
            EffectivePolicyStabilizeQualificationClass::Preview
        } else if beta > 0 {
            EffectivePolicyStabilizeQualificationClass::Beta
        } else {
            EffectivePolicyStabilizeQualificationClass::Stable
        };
        let change_classes_covered: BTreeSet<String> = beta_page
            .simulations
            .iter()
            .map(|s| s.request.change_class_token.clone())
            .collect();
        let expiring_exception_count = beta_page.summary.expiring_exception_count;
        let drifted_remembered_decision_count = beta_page.summary.drift_detected_count;
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            simulation_beta_page_defect_count: beta_page.defects.len(),
            change_classes_covered: change_classes_covered.into_iter().collect(),
            expiring_exception_count,
            drifted_remembered_decision_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the stabilize page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyStabilizeDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: EffectivePolicyStabilizeNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (simulation id, exception id, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl EffectivePolicyStabilizeDefect {
    fn new(
        narrow_reason: EffectivePolicyStabilizeNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: EFFECTIVE_POLICY_STABILIZE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:effective-policy-stabilize:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for effective policy, remembered-decision,
/// waiver-expiry, and exception-preview UX.
///
/// The packet is the single inspectable record that proves the stable claim
/// for this lane. Dashboards, docs, Help/About surfaces, and support exports
/// should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyStabilizePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows.
    pub summary: EffectivePolicyStabilizeSummary,
    /// Per-simulation stability rows.
    pub rows: Vec<EffectivePolicyStabilizeRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<EffectivePolicyStabilizeDefect>,
    /// The upstream policy simulation beta page embedded as evidence.
    pub simulation_beta_page: PolicySimulationBetaPage,
}

impl EffectivePolicyStabilizePage {
    /// Build the stabilize page from an upstream beta page.
    ///
    /// Rows are derived per simulation, and the qualification for each is
    /// computed from the combined audit of the whole page.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        simulation_beta_page: PolicySimulationBetaPage,
    ) -> Self {
        let defects = audit_stabilize_page(&simulation_beta_page);
        let rows = derive_stabilize_rows(&simulation_beta_page, &defects);
        let summary = EffectivePolicyStabilizeSummary::from_rows(&rows, &simulation_beta_page);
        Self {
            record_kind: EFFECTIVE_POLICY_STABILIZE_PAGE_RECORD_KIND.to_owned(),
            schema_version: EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            simulation_beta_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == EffectivePolicyStabilizeQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all required change classes have simulations.
    pub fn covers_required_change_classes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .simulation_beta_page
            .simulations
            .iter()
            .map(|s| s.request.change_class_token.as_str())
            .collect();
        covered.contains("policy_bundle_change") && covered.contains("settings_lock_change")
    }

    /// True when all exceptions and waivers have bounded expiry horizons.
    pub fn exceptions_are_bounded_and_attributable(&self) -> bool {
        self.simulation_beta_page.exceptions.iter().all(|e| {
            !e.owner.stable_id.is_empty()
                && !e.scope.scope_id.is_empty()
                && !e.time_horizon.expires_at.is_empty()
                && !e.evidence_trail_refs.is_empty()
        })
    }

    /// True when all drifted or expired remembered decisions name invalidation
    /// reasons.
    pub fn remembered_decisions_have_explained_drift(&self) -> bool {
        use crate::simulation::MemoryStateClass;
        self.simulation_beta_page
            .remembered_decisions
            .iter()
            .all(|mem| {
                let needs_reason = matches!(
                    mem.memory_state,
                    MemoryStateClass::Expired
                        | MemoryStateClass::RequiresReapproval
                        | MemoryStateClass::ForceRetiredByPolicy
                );
                !needs_reason || !mem.invalidation_reasons.is_empty()
            })
    }

    /// True when action-time policy snapshots preserve historical truth.
    pub fn action_time_policy_truth_is_preserved(&self) -> bool {
        !self.simulation_beta_page.action_time_policy_states.is_empty()
            && self
                .simulation_beta_page
                .action_time_policy_states
                .iter()
                .all(|s| s.preserves_historical_truth)
    }

    /// True when exceptions in the page are linked by at least one simulation
    /// via `exception_preview_refs`.
    pub fn exception_preview_links_are_present(&self) -> bool {
        if self.simulation_beta_page.exceptions.is_empty() {
            return true;
        }
        self.simulation_beta_page
            .simulations
            .iter()
            .any(|s| !s.exception_preview_refs.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the stabilize page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyStabilizeSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The stabilize page embedded as evidence.
    pub page: EffectivePolicyStabilizePage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<EffectivePolicyStabilizeNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// The upstream policy simulation support export.
    pub simulation_support_export: PolicySimulationSupportExport,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl EffectivePolicyStabilizeSupportExport {
    /// Wrap a stabilize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: EffectivePolicyStabilizePage,
    ) -> Self {
        let mut reasons: Vec<EffectivePolicyStabilizeNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        let generated = generated_at.into();
        let export_id_str = export_id.into();
        let simulation_support_export = PolicySimulationSupportExport::from_page(
            format!("{export_id_str}-simulation-beta"),
            generated.clone(),
            page.simulation_beta_page.clone(),
        );
        Self {
            record_kind: EFFECTIVE_POLICY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id_str,
            generated_at: generated,
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            simulation_support_export,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the stabilize audit over the upstream beta page.
pub fn audit_effective_policy_stabilize_page(
    page: &EffectivePolicyStabilizePage,
) -> Vec<EffectivePolicyStabilizeDefect> {
    audit_stabilize_page(&page.simulation_beta_page)
}

/// Validate a stabilize page; returns `Ok` when the audit is clean.
pub fn validate_effective_policy_stabilize_page(
    page: &EffectivePolicyStabilizePage,
) -> Result<(), Vec<EffectivePolicyStabilizeDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn audit_stabilize_page(
    beta_page: &PolicySimulationBetaPage,
) -> Vec<EffectivePolicyStabilizeDefect> {
    let mut defects: Vec<EffectivePolicyStabilizeDefect> = Vec::new();

    // Hard guardrail: raw private material exposed — withdraw immediately.
    let upstream_defects = audit_policy_simulation_beta_page(beta_page);
    let has_raw_material = upstream_defects
        .iter()
        .any(|d| d.defect_kind == PolicySimulationBetaDefectKind::RawPrivateMaterialExposed);
    if has_raw_material {
        defects.push(EffectivePolicyStabilizeDefect::new(
            EffectivePolicyStabilizeNarrowReasonClass::RawPrivateMaterialExposed,
            "simulation_beta_page",
            "upstream beta page has a raw_private_material_exposed defect; packet is withdrawn",
        ));
        return defects;
    }

    // Non-critical: upstream beta page has other defects.
    if !upstream_defects.is_empty() {
        defects.push(EffectivePolicyStabilizeDefect::new(
            EffectivePolicyStabilizeNarrowReasonClass::PolicySimulationBetaPageHasDefects,
            "simulation_beta_page",
            "upstream policy simulation beta page has one or more defects; packet is narrowed to beta",
        ));
    }

    // Required change class coverage (narrows to preview when missing).
    let covered_classes: BTreeSet<&str> = beta_page
        .simulations
        .iter()
        .map(|s| s.request.change_class_token.as_str())
        .collect();
    for required in ["policy_bundle_change", "settings_lock_change"] {
        if !covered_classes.contains(required) {
            defects.push(EffectivePolicyStabilizeDefect::new(
                EffectivePolicyStabilizeNarrowReasonClass::RequiredChangeClassMissing,
                "simulation_beta_page",
                format!("no simulation covers the required change class '{required}'; packet is narrowed to preview"),
            ));
        }
    }

    // Exception-preview UX: exceptions in the page must be linked by at least
    // one simulation.
    if !beta_page.exceptions.is_empty() {
        let any_linked = beta_page
            .simulations
            .iter()
            .any(|s| !s.exception_preview_refs.is_empty());
        if !any_linked {
            defects.push(EffectivePolicyStabilizeDefect::new(
                EffectivePolicyStabilizeNarrowReasonClass::ExceptionPreviewLinksMissing,
                "simulation_beta_page",
                "exceptions exist in the page but no simulation links them via exception_preview_refs",
            ));
        }
    }

    // Exception expiry: every exception must have an explicit expiry.
    for exception in &beta_page.exceptions {
        if exception.time_horizon.expires_at.is_empty() {
            defects.push(EffectivePolicyStabilizeDefect::new(
                EffectivePolicyStabilizeNarrowReasonClass::ExceptionMissingBoundedExpiry,
                exception.exception_id.clone(),
                "exception or waiver is missing a bounded expiry horizon",
            ));
        }
    }

    // Remembered-decision drift explanation.
    use crate::simulation::MemoryStateClass;
    for mem in &beta_page.remembered_decisions {
        let needs_reason = matches!(
            mem.memory_state,
            MemoryStateClass::Expired
                | MemoryStateClass::RequiresReapproval
                | MemoryStateClass::ForceRetiredByPolicy
        );
        if needs_reason && mem.invalidation_reasons.is_empty() {
            defects.push(EffectivePolicyStabilizeDefect::new(
                EffectivePolicyStabilizeNarrowReasonClass::RememberedDecisionDriftUnexplained,
                mem.remembered_decision_id.clone(),
                "drifted or expired remembered decision does not name invalidation reasons",
            ));
        }
    }

    // Support-export historical truth.
    let any_drops_historical = beta_page
        .action_time_policy_states
        .iter()
        .any(|s| !s.preserves_historical_truth);
    if beta_page.action_time_policy_states.is_empty() || any_drops_historical {
        defects.push(EffectivePolicyStabilizeDefect::new(
            EffectivePolicyStabilizeNarrowReasonClass::SupportExportDropsHistoricalTruth,
            "simulation_beta_page",
            "action-time policy snapshots must preserve historical truth for support exports",
        ));
    }

    defects
}

fn derive_stabilize_rows(
    beta_page: &PolicySimulationBetaPage,
    page_defects: &[EffectivePolicyStabilizeDefect],
) -> Vec<EffectivePolicyStabilizeRow> {
    let has_withdrawal = page_defects.iter().any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == EffectivePolicyStabilizeNarrowReasonClass::RequiredChangeClassMissing
    });

    let overall_qual = if has_withdrawal {
        EffectivePolicyStabilizeQualificationClass::Withdrawn
    } else if has_preview {
        EffectivePolicyStabilizeQualificationClass::Preview
    } else if !page_defects.is_empty() {
        EffectivePolicyStabilizeQualificationClass::Beta
    } else {
        EffectivePolicyStabilizeQualificationClass::Stable
    };

    let narrow_reason = if has_withdrawal {
        EffectivePolicyStabilizeNarrowReasonClass::RawPrivateMaterialExposed
    } else if has_preview {
        EffectivePolicyStabilizeNarrowReasonClass::RequiredChangeClassMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        EffectivePolicyStabilizeNarrowReasonClass::NotNarrowed
    };

    // Build a lookup from exception_id → expiring_soon bool.
    let expiring_exception_ids: BTreeSet<&str> = beta_page
        .exceptions
        .iter()
        .filter(|e| {
            e.status == crate::simulation::ExceptionalAuthorityStatusClass::ExpiringSoon
                || e.dashboard_bucket == crate::simulation::DashboardBucketClass::ExpiringSoon
        })
        .map(|e| e.exception_id.as_str())
        .collect();

    // Build a lookup from remembered_decision_id → has_drift bool.
    let drifted_decision_ids: BTreeSet<&str> = beta_page
        .remembered_decisions
        .iter()
        .filter(|m| !m.invalidation_reasons.is_empty())
        .map(|m| m.remembered_decision_id.as_str())
        .collect();

    // Build a set of action families covered by action-time snapshots.
    let action_time_families: BTreeSet<&str> = beta_page
        .action_time_policy_states
        .iter()
        .map(|s| s.action_family_token.as_str())
        .collect();

    beta_page
        .simulations
        .iter()
        .map(|sim| {
            let expiring = sim
                .exception_preview_refs
                .iter()
                .filter(|r| expiring_exception_ids.contains(r.as_str()))
                .count();
            let drifted = sim
                .remembered_decision_preview_refs
                .iter()
                .filter(|r| drifted_decision_ids.contains(r.as_str()))
                .count();
            let action_time_present = sim
                .affected_surfaces
                .iter()
                .any(|s| action_time_families.contains(s.action_family_token.as_str()));
            let summary = build_row_summary(
                &sim.request.simulation_id,
                &sim.request.change_class_token,
                &overall_qual,
                narrow_reason,
            );
            EffectivePolicyStabilizeRow {
                record_kind: EFFECTIVE_POLICY_STABILIZE_ROW_RECORD_KIND.to_owned(),
                schema_version: EFFECTIVE_POLICY_STABILIZE_SCHEMA_VERSION,
                shared_contract_ref: EFFECTIVE_POLICY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
                simulation_id: sim.request.simulation_id.clone(),
                change_class_token: sim.request.change_class_token.clone(),
                scope_ref_id: sim.request.scope.scope_id.clone(),
                exception_preview_count: sim.exception_preview_refs.len(),
                expiring_exception_count: expiring,
                remembered_decision_count: sim.remembered_decision_preview_refs.len(),
                drifted_remembered_decision_count: drifted,
                action_time_policy_state_present: action_time_present,
                qualification_token: overall_qual.as_str().to_owned(),
                narrow_reason_token: narrow_reason.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    simulation_id: &str,
    change_class_token: &str,
    qual: &EffectivePolicyStabilizeQualificationClass,
    narrow_reason: EffectivePolicyStabilizeNarrowReasonClass,
) -> String {
    match qual {
        EffectivePolicyStabilizeQualificationClass::Stable => format!(
            "Simulation '{}' ({}) qualifies stable: upstream beta audit is clean, \
             required change classes are covered, exception expiry and preview links \
             are present, remembered-decision drift is explained, and historical \
             policy truth is preserved.",
            simulation_id, change_class_token
        ),
        _ => format!(
            "Simulation '{}' ({}) narrowed to {} ({}): see defect list for details.",
            simulation_id,
            change_class_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page seeds zero defects: the upstream beta page audits clean,
/// both required change classes are covered, exception expiry and preview
/// links are present, remembered-decision drift is explained, and historical
/// policy truth is preserved.
pub fn seeded_effective_policy_stabilize_page() -> EffectivePolicyStabilizePage {
    EffectivePolicyStabilizePage::new(
        "policy:effective_policy_stabilize:default",
        "Effective policy, remembered-decision, waiver-expiry, and exception-preview UX (stable)",
        "2026-06-01T00:00:00Z",
        seeded_policy_simulation_beta_page(),
    )
}
