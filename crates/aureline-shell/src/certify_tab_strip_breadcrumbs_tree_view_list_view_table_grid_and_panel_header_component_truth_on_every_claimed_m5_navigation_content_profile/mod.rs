//! M05-1115 surface certification over the frozen M5 tab-strip / breadcrumbs /
//! tree-view / list-view / table-grid / panel-header navigation-content component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix`])
//! defines the six reusable tab-strip, breadcrumbs, tree-view, list-view, table/grid, and
//! panel-header components, the M05-1109..1112 implement lanes narrow each one, the
//! M05-shared consumer lane aligns their vocabulary, and the M05-1114 accessibility lane
//! ([`crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_hierarchy_selection_count_sort_filter_or_freshness_truth_is_missing_or_stale_across_claimed_m5_navigation_content_components`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared component
//! truth holds on every claimed M5 navigation-content operating profile — and auto-narrows
//! any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, operator, or support engineer reads
//! navigation and content truth through (a live active-context shell, a reviewable
//! explorer tree, a reviewable result grid, a traced breadcrumb trail, a stale-hierarchy
//! breadcrumb, an unresolved-count list, a stale-provenance grid, and a partial-freshness
//! panel), not on component family or implement lane. Each
//! [`NavContentProfileCertificationRow`] certifies one profile across eight truth axes —
//! visual, keyboard, screen-reader, high-zoom-reflow, reduced-motion, CLI/export,
//! degraded-state, and navigation-content-truth behavior — and either passes (green),
//! auto-narrows its navigation/content claim to the weakest supported ceiling (yellow), or
//! is blocked (red) when a degraded axis is hidden behind a fresh current-navigation claim
//! inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile
//! that keeps a `CurrentNavigationResult` / `ReviewableStructureResult` claim while one of
//! its truth axes is not current is over-claiming and blocks; a profile that discloses the
//! reduction by narrowing its claim (with a bound reason and a frozen downgrade trigger) is
//! honestly yellow. Only a live, first-party current-navigation profile may certify a
//! `CurrentNavigationResult` claim — a reviewable, stale, unresolved, or partial profile
//! that keeps a current-navigation claim is over-reaching and blocks. The always-on
//! CLI/export axis must always stay certified so support and automation can reconstruct the
//! active context, hierarchy / path, disclosure state, selection-versus-current, exact /
//! loaded / all-matching counts, sort/filter provenance, and source-freshness from the same
//! component identity the user saw.
//!
//! The B132 guardrails are enforced per row: no profile may let tabs masquerade as
//! top-level workflow navigation, hide counts or blocked rows behind ambiguous ellipses,
//! make tree/list/table actions hover-only, let a panel header become a cluttered secondary
//! toolbar, or collapse exact / loaded / all-matching count scopes into one vague total. A
//! profile that breaches any guardrail blocks (red).
//!
//! Every row cites exactly one canonical navigation-content proof bundle
//! ([`NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen navigation-content component
//! matrix proof — rather than cloning per-profile evidence. The packet is metadata-only:
//! raw tree bodies, row payloads, query internals, credentials, and endpoint refs never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-navigation-content-component-certification.schema.json`](../../../../schemas/ui/m5-navigation-content-component-certification.schema.json).
//! The contract doc is
//! [`docs/navigation/m5_navigation_content_component_certification_contract.md`](../../../../docs/navigation/m5_navigation_content_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_hierarchy_selection_count_sort_filter_or_freshness_truth_is_missing_or_stale_across_claimed_m5_navigation_content_components as a11y;
use crate::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix as matrix;
use a11y::M5NavContentComponentClaim;
use matrix::{M5NavigationContentComponentFamily, M5NavigationContentDowngradeTrigger};

/// Schema version stamped on the M05-1115 certification packet.
pub const NAV_CONTENT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NavContentProfileCertificationPacket`].
pub const NAV_CONTENT_CERT_RECORD_KIND: &str =
    "m5_navigation_content_component_certification_packet";

/// Stable record-kind tag carried by each [`NavContentProfileCertificationRow`].
pub const NAV_CONTENT_CERT_ROW_RECORD_KIND: &str =
    "m5_navigation_content_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const NAV_CONTENT_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-navigation-content-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const NAV_CONTENT_CERT_DOC_REF: &str =
    "docs/navigation/m5_navigation_content_component_certification_contract.md";

/// Repo-relative path of the frozen navigation-content component matrix schema the
/// certified profiles render.
pub const NAV_CONTENT_CERT_MATRIX_REF: &str = matrix::M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF;

/// The one canonical navigation-content proof bundle every certified profile cites as its
/// first-resolved component truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_NAVIGATION_CONTENT_COMPONENT_ARTIFACT_REF;

/// The M05-1114 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const NAV_CONTENT_CERT_A11Y_BUNDLE_REF: &str = a11y::NAV_CONTENT_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const NAV_CONTENT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-navigation-content-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NAV_CONTENT_CERT_CSV_REF: &str =
    "artifacts/release/m5-navigation-content-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NAV_CONTENT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-navigation-content-component-certification-proof/report.md";

/// The eight claimed M5 navigation-content operating profiles this capstone certifies.
/// Keyed on the profile a user, operator, or support engineer reads navigation and content
/// truth through, not on the reusable component family it renders. Only a live, first-party
/// current-navigation profile may certify a current-navigation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NavContentCertifiedProfile {
    /// The shell's live active-context surface — tab strip and panel header naming the
    /// current open context authoritatively as exactly current right now.
    LiveActiveContextShell,
    /// A reviewable explorer tree / list: a self-sufficient read-only dense structure a
    /// user can review, never itself an authoritative live-current navigation surface.
    ReviewableExplorerTree,
    /// A reviewable result grid: a search / data table/grid a user can review with honest
    /// count scopes, never a live-current navigation surface.
    ReviewableResultGrid,
    /// A breadcrumb trail whose ancestry to the current object is fully current and traced.
    TracedBreadcrumbTrail,
    /// A breadcrumb / tree whose hierarchy signal is stale; the claim narrows to a
    /// hierarchy-unverified projection with last-known ancestry preserved.
    StaleHierarchyBreadcrumb,
    /// A list whose exact / loaded / all-matching count scope is unresolved; the claim
    /// narrows to a count-unverified projection with last-known loaded scope preserved.
    UnresolvedCountList,
    /// A table/grid whose sort / filter provenance is stale; the claim narrows to a
    /// sort-filter-unverified projection naming the last-known ordering.
    StaleProvenanceGrid,
    /// A panel whose source-freshness cue is only partial / cached; the claim narrows to a
    /// source-freshness projection disclosing the cached / partial cue.
    PartialFreshnessPanel,
}

impl M5NavContentCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5NavContentCertifiedProfile; 8] = [
        M5NavContentCertifiedProfile::LiveActiveContextShell,
        M5NavContentCertifiedProfile::ReviewableExplorerTree,
        M5NavContentCertifiedProfile::ReviewableResultGrid,
        M5NavContentCertifiedProfile::TracedBreadcrumbTrail,
        M5NavContentCertifiedProfile::StaleHierarchyBreadcrumb,
        M5NavContentCertifiedProfile::UnresolvedCountList,
        M5NavContentCertifiedProfile::StaleProvenanceGrid,
        M5NavContentCertifiedProfile::PartialFreshnessPanel,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveActiveContextShell => "live_active_context_shell",
            Self::ReviewableExplorerTree => "reviewable_explorer_tree",
            Self::ReviewableResultGrid => "reviewable_result_grid",
            Self::TracedBreadcrumbTrail => "traced_breadcrumb_trail",
            Self::StaleHierarchyBreadcrumb => "stale_hierarchy_breadcrumb",
            Self::UnresolvedCountList => "unresolved_count_list",
            Self::StaleProvenanceGrid => "stale_provenance_grid",
            Self::PartialFreshnessPanel => "partial_freshness_panel",
        }
    }

    /// True only for the live, first-party current-navigation shell profile. A
    /// current-navigation result may be certified on this profile alone; every other
    /// profile is at most a reviewable structure result or a narrowed projection.
    pub const fn is_live_current_navigation(self) -> bool {
        matches!(self, Self::LiveActiveContextShell)
    }
}

/// The eight truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, high-zoom
/// reflow, reduced-motion, CLI/export, degraded-state, and navigation-content-truth
/// behavior. The CLI/export axis is always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentCertificationAxis {
    /// Visual parity: active context, hierarchy / path, disclosure state,
    /// selection-versus-current, item state, count scope, sort/filter provenance, and
    /// source-freshness are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same navigation / content truth and its bounded local
    /// actions are reachable and operable without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color, motion, or a chrome glyph alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than
    /// clipping counts, hierarchy, or the active context.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion,
    /// never motion-only.
    ReducedMotion,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale hierarchy, unresolved count scope, stale sort/filter
    /// provenance, or partial source-freshness cue honestly downgrades a
    /// `CurrentNavigationResult` / `ReviewableStructureResult` claim rather than reading as
    /// fresh, authoritative navigation.
    DegradedState,
    /// Navigation-content-truth parity: active context, hierarchy / path, disclosure,
    /// selection-versus-current, count scope, sort/filter provenance, and source-freshness
    /// stay explicit and never collapse into generic chrome wording, let tabs masquerade as
    /// top-level navigation, hide counts or blocked rows behind ellipses, make actions
    /// hover-only, overload the panel header, or collapse distinct count scopes into one
    /// total.
    NavigationContentTruth,
}

impl NavContentCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [NavContentCertificationAxis; 8] = [
        NavContentCertificationAxis::Visual,
        NavContentCertificationAxis::Keyboard,
        NavContentCertificationAxis::ScreenReader,
        NavContentCertificationAxis::HighZoomReflow,
        NavContentCertificationAxis::ReducedMotion,
        NavContentCertificationAxis::CliExport,
        NavContentCertificationAxis::DegradedState,
        NavContentCertificationAxis::NavigationContentTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotion => "reduced_motion",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::NavigationContentTruth => "navigation_content_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a current-navigation claim
    /// inherited from a healthier profile.
    UndisclosedDrift,
}

impl NavContentAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author —
/// always recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavContentProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed navigation/content
    /// tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export
    /// parity drops, a non-live profile claims current navigation, or the narrowing is
    /// inconsistent.
    Red,
}

impl NavContentProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red
    /// profiles block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B132 guardrails carried on every certified profile. All five must hold — a
/// breach blocks the profile (red). Each field is `true` only when the profile *breaks* the
/// guardrail, so a clean profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentCertGuardrails {
    /// True if the profile lets a tab strip masquerade as top-level workflow navigation
    /// rather than a set of open contexts. Must be false.
    pub tabs_masquerade_as_top_level_navigation: bool,
    /// True if the profile hides counts or blocked rows behind ambiguous ellipses. Must be
    /// false.
    pub hides_counts_or_blocked_rows_behind_ellipses: bool,
    /// True if the profile makes tree / list / table local actions hover-only. Must be
    /// false.
    pub makes_tree_list_table_actions_hover_only: bool,
    /// True if the profile lets a panel header become a cluttered secondary toolbar. Must
    /// be false.
    pub panel_header_becomes_secondary_toolbar: bool,
    /// True if the profile collapses exact / loaded / all-matching count scopes into one
    /// vague total. Must be false.
    pub collapses_exact_loaded_all_matching_scopes: bool,
}

impl NavContentCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        tabs_masquerade_as_top_level_navigation: false,
        hides_counts_or_blocked_rows_behind_ellipses: false,
        makes_tree_list_table_actions_hover_only: false,
        panel_header_becomes_secondary_toolbar: false,
        collapses_exact_loaded_all_matching_scopes: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.tabs_masquerade_as_top_level_navigation
            && !self.hides_counts_or_blocked_rows_behind_ellipses
            && !self.makes_tree_list_table_actions_hover_only
            && !self.panel_header_becomes_secondary_toolbar
            && !self.collapses_exact_loaded_all_matching_scopes
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies
/// only when this offers text / JSON / Markdown reconstruction and prohibits a
/// screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The active-context / hierarchy / disclosure / selection / count-scope / provenance /
    /// source-freshness fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl NavContentCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a screenshot-only
    /// export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: NavContentCertificationAxis,
    /// The certification state of the axis.
    pub state: NavContentAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5NavigationContentDowngradeTrigger>,
}

impl NavContentAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            NavContentAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            NavContentAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            NavContentAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present
/// iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: NavContentCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5NavContentComponentClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5NavContentComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 navigation-content profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentProfileCertificationRow {
    /// Record kind; must equal [`NAV_CONTENT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NAV_CONTENT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5NavContentCertifiedProfile,
    /// The navigation / content claim ceiling the profile asserts.
    pub claimed_claim: M5NavContentComponentClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5NavContentComponentClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5NavigationContentComponentFamily>,
    /// One outcome per [`NavContentCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<NavContentAxisOutcome>,
    /// The B132 guardrails; all must hold.
    pub guardrails: NavContentCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<NavContentClaimAutoNarrow>,
    /// The one canonical navigation-content proof bundle this profile cites. Must equal
    /// [`NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: NavContentProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: NavContentCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NavContentProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: NavContentCertificationAxis) -> Option<&NavContentAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<NavContentCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && NavContentCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(NavContentAxisOutcome::well_formed)
    }

    /// True when the profile narrows its navigation / content claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<NavContentCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == NavContentAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is
    /// the heart of the capstone: a degraded axis must produce a visible claim narrowing,
    /// only a live first-party profile may certify current navigation, every guardrail must
    /// hold, CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> NavContentProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return NavContentProfileClaimStatus::Red;
        }

        // Every B132 guardrail must hold.
        if !self.guardrails.all_held() {
            return NavContentProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return NavContentProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a current-navigation result.
        if self.certified_claim.asserts_current_navigation_result()
            && !self.profile.is_live_current_navigation()
        {
            return NavContentProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(NavContentCertificationAxis::CliExport) {
            Some(o) if o.state == NavContentAxisCertificationState::Certified => {}
            _ => return NavContentProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == NavContentAxisCertificationState::UndisclosedDrift)
        {
            return NavContentProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return NavContentProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return NavContentProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return NavContentProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return NavContentProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return NavContentProfileClaimStatus::Red;
        }

        NavContentProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == NAV_CONTENT_CERT_ROW_RECORD_KIND
            && self.schema_version == NAV_CONTENT_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1115 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`NavContentProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavContentProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<NavContentProfileCertificationRow>,
}

/// Checked-in M05-1115 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavContentProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<NavContentProfileCertificationRow>,
    pub summary: NavContentProfileCertificationSummary,
}

impl NavContentProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: NavContentProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NAV_CONTENT_CERT_SCHEMA_VERSION,
            record_kind: NAV_CONTENT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: NavContentProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5NavContentCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5NavigationContentComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5NavContentCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof
    /// the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5NavigationContentComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(NavContentCertificationAxis::CliExport)
                .is_some_and(|o| o.state == NavContentAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> NavContentProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NavContentProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NavContentProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NavContentProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(NavContentProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        NavContentProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(NavContentProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NavContentCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NAV_CONTENT_CERT_SCHEMA_VERSION {
            violations.push(NavContentCertificationViolation::SchemaVersion {
                expected: NAV_CONTENT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NAV_CONTENT_CERT_RECORD_KIND {
            violations.push(NavContentCertificationViolation::RecordKind {
                expected: NAV_CONTENT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NavContentCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(NavContentCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NavContentCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(NavContentCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(NavContentCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(NavContentCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    NavContentCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B132 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(NavContentCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a current-navigation result.
            if row.certified_claim.asserts_current_navigation_result()
                && !row.profile.is_live_current_navigation()
            {
                violations.push(
                    NavContentCertificationViolation::NonLiveProfileClaimsCurrentNavigation {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(NavContentCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(NavContentCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    NavContentCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(NavContentCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == NavContentProfileClaimStatus::Red {
                violations.push(NavContentCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(NavContentCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(NavContentCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(NavContentCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(NavContentCertificationViolation::RawNavContentMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Navigation-Content Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5NavContentCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Guardrails held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_navigation_content_component_certification_export(
) -> Result<NavContentProfileCertificationPacket, NavContentCertificationArtifactError> {
    let packet: NavContentProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-navigation-content-component-certification-proof/support_export.json"
    )))
    .map_err(NavContentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NavContentCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum NavContentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NavContentCertificationViolation>),
}

impl fmt::Display for NavContentCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for NavContentCertificationArtifactError {}

/// Validation failure for M05-1115 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavContentCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLiveProfileClaimsCurrentNavigation { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawNavContentMaterialInExport,
}

impl fmt::Display for NavContentCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical navigation-content proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical navigation-content proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B132 guardrail: tabs masquerading as top-level navigation, \
counts or blocked rows hidden behind ellipses, hover-only tree/list/table actions, an overloaded \
panel-header toolbar, or collapsed exact/loaded/all-matching count scopes"
                )
            }
            Self::NonLiveProfileClaimsCurrentNavigation { id } => {
                write!(
                    f,
                    "row {id} certifies a current-navigation result on a non-live first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh \
current-navigation claim, a guardrail broke, CLI/export parity dropped, a non-live profile claimed \
current navigation, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 navigation-content profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen navigation-content component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawNavContentMaterialInExport => {
                write!(
                    f,
                    "export contains a raw tree body, row payload, query internals, or credential material"
                )
            }
        }
    }
}

impl Error for NavContentCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&NavContentAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != NavContentAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// navigation / content generics the spec forbids collapsing distinct active-context,
/// hierarchy, count-scope, provenance, and freshness truth into (whole-label matches so a
/// full sentence naming a concrete context, ancestry, or count scope is not flagged).
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "loading"
            | "partial"
            | "cached"
            | "current"
            | "tabs"
            | "breadcrumbs"
            | "tree"
            | "list"
            | "grid"
            | "table"
            | "panel header"
            | "panel-header"
            | "hierarchy"
            | "count"
            | "counts"
            | "provenance"
            | "freshness"
            | "selection"
            | "disclosure"
            | "active context"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1115 certification packet. Certifies all eight
/// claimed M5 navigation-content profiles: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker navigation/content ceiling (yellow). No
/// profile hides drift or breaks a guardrail (red).
pub fn seeded_m5_navigation_content_component_certification_packet(
) -> NavContentProfileCertificationPacket {
    NavContentProfileCertificationPacket::new(NavContentProfileCertificationPacketInput {
        packet_id: "m5-navigation-content-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: NAV_CONTENT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:navigation-content-component-certification:{id}"),
        NAV_CONTENT_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> NavContentCertExportParity {
    NavContentCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: NavContentCertificationAxis) -> &'static str {
    match axis {
        NavContentCertificationAxis::Visual => {
            "active context, hierarchy / path, disclosure state, selection-versus-current, item state, exact / loaded / all-matching counts, sort/filter provenance, and source-freshness shown on-surface"
        }
        NavContentCertificationAxis::Keyboard => {
            "the same active context, hierarchy, selection, counts, and bounded local actions are keyboard-reachable, never hover-only"
        }
        NavContentCertificationAxis::ScreenReader => {
            "the same navigation / content truth is announced non-visually, never color/motion/glyph-only"
        }
        NavContentCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping counts, hierarchy, or the active context"
        }
        NavContentCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        NavContentCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        NavContentCertificationAxis::DegradedState => {
            "a stale hierarchy, unresolved count scope, stale sort/filter provenance, or partial source-freshness cue honestly downgrades the CurrentNavigationResult/ReviewableStructureResult claim rather than reading as fresh authoritative navigation"
        }
        NavContentCertificationAxis::NavigationContentTruth => {
            "active context, hierarchy / path, disclosure, selection-versus-current, count scope, sort/filter provenance, and source-freshness stay explicit and never collapse into generic chrome, let tabs masquerade as top-level navigation, hide counts or blocked rows behind ellipses, make actions hover-only, overload the panel header, or collapse distinct count scopes into one total"
        }
    }
}

fn seed_certified(axis: NavContentCertificationAxis) -> NavContentAxisOutcome {
    NavContentAxisOutcome {
        axis,
        state: NavContentAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: NavContentCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5NavigationContentDowngradeTrigger,
) -> NavContentAxisOutcome {
    NavContentAxisOutcome {
        axis,
        state: NavContentAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<NavContentAxisOutcome> {
    NavContentCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: NavContentCertificationAxis,
    outcome: NavContentAxisOutcome,
) -> Vec<NavContentAxisOutcome> {
    NavContentCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5NavContentCertifiedProfile,
    claimed_claim: M5NavContentComponentClaim,
    certified_claim: M5NavContentComponentClaim,
    consumed_families: &[M5NavigationContentComponentFamily],
    axis_outcomes: Vec<NavContentAxisOutcome>,
    claim_auto_narrow: Option<NavContentClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> NavContentProfileCertificationRow {
    let mut row = NavContentProfileCertificationRow {
        record_kind: NAV_CONTENT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: NAV_CONTENT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: NavContentCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: NAV_CONTENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: NavContentProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            NAV_CONTENT_CERT_MATRIX_REF.to_owned(),
            NAV_CONTENT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-12T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: NavContentCertificationAxis,
    from_claim: M5NavContentComponentClaim,
    to_claim: M5NavContentComponentClaim,
    label: &str,
) -> NavContentClaimAutoNarrow {
    NavContentClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<NavContentProfileCertificationRow> {
    use M5NavContentCertifiedProfile as P;
    use M5NavContentComponentClaim::*;
    use M5NavigationContentComponentFamily::*;
    use M5NavigationContentDowngradeTrigger as Trig;
    use NavContentCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-active-context-shell",
            P::LiveActiveContextShell,
            CurrentNavigationResult,
            CurrentNavigationResult,
            &[TabStrip, PanelHeader],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "active_context"],
            &[
                "tab strip names the active context, per-tab item state, and overflow for the open contexts without masquerading as top-level workflow navigation",
                "panel header names the active context and a bounded local-action budget without becoming a secondary toolbar",
                "keyboard / screen-reader / high-zoom / reduced-motion reach preserved for the tab strip and the panel header",
                "navigation-content-truth: a live first-party shell is the only profile that certifies a current-navigation result",
            ],
        ),
        seed_row(
            "cert:reviewable-explorer-tree",
            P::ReviewableExplorerTree,
            ReviewableStructureResult,
            ReviewableStructureResult,
            &[TreeView, ListView],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "disclosure_state"],
            &[
                "tree view names hierarchy, disclosure state, selection-versus-current, item state, counts, and capped keyboard-discoverable local actions with virtualization-honest disclosure",
                "list view names selection-versus-current, item state, and exact / loaded / all-matching / hidden counts with local actions never hidden behind hover",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable structure",
                "navigation-content-truth: a reviewable read-only tree never certifies a live current-navigation claim, and no count scope is collapsed",
            ],
        ),
        seed_row(
            "cert:reviewable-result-grid",
            P::ReviewableResultGrid,
            ReviewableStructureResult,
            ReviewableStructureResult,
            &[TableGrid, ListView],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "count_scope"],
            &[
                "table / grid names selection, exact / loaded / all-matching counts, density, item state, and sort/filter provenance without overloading a panel header",
                "list view keeps exact / loaded / all-matching / hidden count scopes distinct rather than collapsing them into one vague total",
                "export preserves the sort/filter provenance and the exact / loaded / all-matching count scopes",
                "navigation-content-truth: a reviewable result grid keeps counts honest and never hides blocked rows behind an ellipsis",
            ],
        ),
        seed_row(
            "cert:traced-breadcrumb-trail",
            P::TracedBreadcrumbTrail,
            ReviewableStructureResult,
            ReviewableStructureResult,
            &[Breadcrumbs, TabStrip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "hierarchy_path"],
            &[
                "breadcrumb trail names the full, current hierarchy / path to the current object, including any truncated ancestry, explicitly",
                "tab strip names the active context so the traced path and the open context stay aligned",
                "text / JSON / Markdown reconstruction certified so support can replay the traced ancestry",
                "navigation-content-truth: the breadcrumb ancestry stays explicit and never masquerades as more than a reviewable, current path",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-hierarchy-breadcrumb",
            P::StaleHierarchyBreadcrumb,
            ReviewableStructureResult,
            HierarchyUnverifiedProjection,
            &[Breadcrumbs, TreeView],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the hierarchy / path signal is stale so a live current ancestry cannot be certified",
                    "The breadcrumb / tree hierarchy signal is stale, so the ReviewableStructureResult claim narrows to a hierarchy-unverified projection and the trail preserves its last-known ancestry rather than presenting a stale path as the live current hierarchy",
                    Trig::HierarchyPathUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableStructureResult,
                HierarchyUnverifiedProjection,
                "Hierarchy unverified: the path signal is stale so the last-known ancestry is preserved and the trail never reads as the live current hierarchy",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "breadcrumb trail preserves the last-known ancestry and marks the hierarchy as unverified rather than presenting a stale path as current",
                "tree view keeps its disclosure and selection truth while the hierarchy is disclosed as unverified",
                "degraded-state: ReviewableStructureResult narrows to a hierarchy-unverified projection (auto-narrowed)",
                "navigation-content-truth: a stale hierarchy never masquerades as the live current path",
            ],
        ),
        seed_row(
            "cert:unresolved-count-list",
            P::UnresolvedCountList,
            ReviewableStructureResult,
            CountUnverifiedProjection,
            &[ListView, TableGrid],
            seed_certified_except(
                Ax::NavigationContentTruth,
                seed_narrowed(
                    Ax::NavigationContentTruth,
                    "the exact / loaded / all-matching count scope is unresolved so a single exact total cannot be certified",
                    "The exact / loaded / all-matching count scope is unresolved, so the ReviewableStructureResult claim narrows to a count-unverified projection and the list preserves its last-known loaded scope rather than collapsing exact, loaded, and all-matching into one vague total",
                    Trig::CountScopeCollapsed,
                ),
            ),
            Some(seed_narrow(
                Ax::NavigationContentTruth,
                ReviewableStructureResult,
                CountUnverifiedProjection,
                "Count unverified: the exact / loaded / all-matching scope is unresolved so the last-known loaded scope is preserved and no single exact total is implied",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "list view keeps the last-known loaded scope visible and marks the count as unverified rather than showing one exact total",
                "table / grid keeps its selection and provenance truth while the count scope is disclosed as unresolved",
                "navigation-content-truth: ReviewableStructureResult narrows to a count-unverified projection (auto-narrowed)",
                "navigation-content-truth: exact, loaded, and all-matching count scopes never collapse into one vague total",
            ],
        ),
        seed_row(
            "cert:stale-provenance-grid",
            P::StaleProvenanceGrid,
            ReviewableStructureResult,
            SortFilterUnverifiedProjection,
            &[TableGrid, ListView],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the sort / filter provenance is stale so a canonically ordered grid cannot be certified",
                    "The table / grid sort / filter provenance is stale, so the ReviewableStructureResult claim narrows to a sort-filter-unverified projection and the grid names its last-known ordering rather than presenting a stale order as the canonical current ordering",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableStructureResult,
                SortFilterUnverifiedProjection,
                "Sort/filter unverified: the provenance is stale so the last-known ordering is named and the grid never reads as canonically ordered right now",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "table / grid names its last-known ordering and marks the sort/filter provenance as unverified rather than implying a canonical current order",
                "list view keeps its count-scope truth while the provenance is disclosed as stale",
                "degraded-state: ReviewableStructureResult narrows to a sort-filter-unverified projection (auto-narrowed)",
                "navigation-content-truth: a stale sort/filter provenance never reads as the canonical current ordering",
            ],
        ),
        seed_row(
            "cert:partial-freshness-panel",
            P::PartialFreshnessPanel,
            ReviewableStructureResult,
            SourceFreshnessProjection,
            &[PanelHeader, TabStrip],
            seed_certified_except(
                Ax::NavigationContentTruth,
                seed_narrowed(
                    Ax::NavigationContentTruth,
                    "the source-freshness cue at the pane boundary is only partial / cached so a freshly-current header cannot be certified",
                    "The panel header's source-freshness cue is only partial / cached, so the ReviewableStructureResult claim narrows to a source-freshness projection and the header discloses the cached / partial cue rather than presenting the pane as freshly current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::NavigationContentTruth,
                ReviewableStructureResult,
                SourceFreshnessProjection,
                "Source freshness partial: the pane-boundary cue is cached / partial so the header discloses the cached cue and never reads as freshly current",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "panel header discloses the cached / partial source-freshness cue at the pane boundary rather than presenting the pane as freshly current",
                "tab strip keeps the active context explicit while the source-freshness cue is disclosed as partial",
                "navigation-content-truth: ReviewableStructureResult narrows to a source-freshness projection (auto-narrowed)",
                "navigation-content-truth: a partial / cached source-freshness cue never reads as a freshly-current header",
            ],
        ),
    ]
}
