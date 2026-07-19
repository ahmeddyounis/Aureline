//! Assistive-technology diagnostics, announcement-spam budgets, OS accessibility-bridge
//! probes, and high-zoom/high-contrast/reduced-motion conformance for the claimed M5
//! custom-rendered dynamic surfaces.
//!
//! Where the per-surface descriptors ([`crate::accessibility`]) bind a custom surface to
//! its semantic roles, label model, and OS bridge mapping, and the announcement grammar
//! ([`crate::announcement_grammar`]) bounds how often a live region may speak, this
//! module materializes *whether each claimed surface is currently healthy* and *whether
//! it may ship*. One [`M5SurfaceDiagnostics`] row per protected surface family probes the
//! OS accessibility bridge (bridge state plus missing-semantic-node coverage), runs the
//! full battery of AT diagnostic checks (bridge health, missing semantic nodes,
//! announcement-rate and coalescing violations, focus-return failures, and
//! high-zoom/high-contrast/reduced-motion regressions plus screen-reader label/role
//! drift), measures the surface's live-region traffic against a published
//! announcement-spam budget, and resolves a per-surface release gate that the
//! release/public-truth automation reads to fail rows for bridge regressions,
//! announcement spam, or zoom/contrast/motion breakage.
//!
//! The report is the single M5 source for assistive-tech *health* truth: the shell,
//! support exports, help/docs, and release/public-truth automation consume the same rows
//! rather than reproducing AT failures by hand. Every diagnostic finding is carried by a
//! stable `diagnostic.`-prefixed message id and an export-safe evidence ref, so a support
//! bundle and an AT conformance packet can disclose bridge health, the focus-contract
//! disposition a surface fell back to, the current degraded state, and which surfaces are
//! release-blocked — without leaking raw provider payloads, credentials, screenshots, or
//! untranslated free-text prose. When a surface's bridge or proof goes stale the row
//! auto-narrows (a disclosed claim change) rather than implying silent screen-reader
//! completeness, and the per-surface bridge and announcement diagnostics are never
//! collapsed into a single aggregate pass/fail dashboard.
//!
//! The controlled state vocabularies — semantic role class, non-visual fidelity, bridge
//! state, coalescing strategy, focus-return disposition, qualification class, downgrade
//! trigger, consumer surface, and proof/release posture — are reused verbatim from the
//! frozen dynamic-surface matrix; the protected surface families, OS bridge kinds, and
//! bridge-degradation reasons are reused from the surface descriptors; and the coalescing
//! budget and durable-fallback surface tokens are reused from the announcement grammar.
//! Only the diagnostics-shaped vocabularies this lane adds (AT diagnostic class,
//! diagnostic outcome, diagnostic severity, visual-adaptation mode, and release-gate
//! decision) are minted here and frozen in a self-describing
//! [`M5DiagnosticsVocabularySet`].
//!
//! The boundary schema is
//! [`schemas/a11y/m5-dynamic-a11y-report.schema.json`](../../../../../schemas/a11y/m5-dynamic-a11y-report.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-dynamic-a11y-diagnostics.md`](../../../../../docs/a11y/m5-dynamic-a11y-diagnostics.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-bridge-and-announcement-drills/`](../../../../../fixtures/a11y/m5-bridge-and-announcement-drills/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_dynamic_a11y_diagnostics_report,
    seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked,
    seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked,
    seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed,
    seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked,
    M5_DYNAMIC_A11Y_DIAGNOSTICS_REPORT_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The surface descriptors own the protected surface families, OS bridge kinds, and
// bridge-degradation reasons; reuse them so diagnostics map to the same surfaces and
// bridge taxonomy the descriptors publish rather than minting parallel surface ids.
use crate::accessibility::{M5BridgeDegradationReason, M5SurfaceBridgeKind, M5SurfaceFamily};
// The announcement grammar owns the coalescing-budget shape and the durable-fallback
// surface vocabulary; reuse them so the spam budget and the reopenable fallback resolve
// against the same tokens the live-announcement lane already governs.
use crate::announcement_grammar::{M5CoalescingBudget, M5DurableFallbackRef};
// The frozen matrix owns the shared state vocabularies, qualification classes, downgrade
// triggers, consumer surfaces, and proof/release posture.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use matrix::{
    A11yBridgeState, A11yCoalescingStrategy, A11yFocusReturnDisposition, A11yNonVisualFidelity,
    A11ySemanticRoleClass, M5DynamicSurfaceA11yConsumerSurface,
    M5DynamicSurfaceA11yDowngradeTrigger, M5DynamicSurfaceA11yProofFreshness,
    M5DynamicSurfaceA11yQualificationClass, M5DynamicSurfaceA11yReleasePosture,
    M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5DynamicA11yDiagnosticsPacket`].
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_RECORD_KIND: &str = "m5_dynamic_a11y_diagnostics_report";

/// Schema version for M5 dynamic-surface AT diagnostics reports.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_REF: &str =
    "schemas/a11y/m5-dynamic-a11y-report.schema.json";

/// Repo-relative path of the M5 AT-diagnostics contract doc.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_DOC_REF: &str = "docs/a11y/m5-dynamic-a11y-diagnostics.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that governs
/// this lane's shared controlled vocabularies and qualification classes.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_MATRIX_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the per-surface accessibility descriptors these diagnostics
/// probe and share their surface families and bridge taxonomy with.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_SURFACE_DESCRIPTOR_REF: &str =
    "schemas/a11y/m5-surface-descriptors.schema.json";

/// Repo-relative path of the live-announcement grammar that owns the coalescing-budget
/// shape and durable-fallback surface vocabulary the spam budgets reuse.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_ANNOUNCEMENT_GRAMMAR_REF: &str =
    "schemas/a11y/m5-announcement-grammar.schema.json";

/// Repo-relative path of the frozen screen-reader announcement / live-region contract.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the frozen focus / zoom / pointer-independence contract.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_FOCUS_CONTRACT_REF: &str =
    "docs/accessibility/focus_zoom_and_pointer_independence_contract.md";

/// Repo-relative path of the frozen visual-adaptation (zoom / contrast / motion) contract.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_VISUAL_ADAPTATION_CONTRACT_REF: &str =
    "docs/accessibility/visual_adaptation_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_FIXTURE_DIR: &str =
    "fixtures/a11y/m5-bridge-and-announcement-drills";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_DYNAMIC_A11Y_DIAGNOSTICS_SUMMARY_REF: &str =
    "artifacts/a11y/m5-dynamic-a11y-diagnostics/dynamic-a11y-diagnostics-proof.md";

/// Stable prefix every diagnostics-owned message id carries (probes, checks, budgets,
/// conformance, degraded-state disclosures, and the release-gate verdict).
pub const M5_DIAGNOSTIC_MESSAGE_ID_PREFIX: &str = "diagnostic.";

/// One class of assistive-technology diagnostic run against a claimed surface.
///
/// These are exactly the failure modes the lane must diagnose from the support/export
/// system instead of one-off manual reproduction: bridge health and missing semantic
/// nodes, announcement-rate and coalescing spam, focus-return failures, the three
/// visual-adaptation regressions, and screen-reader label/role drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AtDiagnosticClass {
    /// OS accessibility-bridge connection health.
    BridgeHealth,
    /// Missing semantic nodes in the accessibility tree.
    MissingSemanticNode,
    /// Announcement-rate / spam-budget pressure on the live region.
    AnnouncementRate,
    /// Live-region coalescing-rule violation.
    CoalescingViolation,
    /// Focus-return failure (teleport or vanish) on an async update.
    FocusReturnFailure,
    /// High-zoom / large-text layout regression.
    HighZoomRegression,
    /// High-contrast / forced-colors regression.
    HighContrastRegression,
    /// Reduced-motion regression.
    ReducedMotionRegression,
    /// Screen-reader label or role drift from its semantic source.
    LabelOrRoleDrift,
}

impl M5AtDiagnosticClass {
    /// Every diagnostic class, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::BridgeHealth,
        Self::MissingSemanticNode,
        Self::AnnouncementRate,
        Self::CoalescingViolation,
        Self::FocusReturnFailure,
        Self::HighZoomRegression,
        Self::HighContrastRegression,
        Self::ReducedMotionRegression,
        Self::LabelOrRoleDrift,
    ];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BridgeHealth => "bridge_health",
            Self::MissingSemanticNode => "missing_semantic_node",
            Self::AnnouncementRate => "announcement_rate",
            Self::CoalescingViolation => "coalescing_violation",
            Self::FocusReturnFailure => "focus_return_failure",
            Self::HighZoomRegression => "high_zoom_regression",
            Self::HighContrastRegression => "high_contrast_regression",
            Self::ReducedMotionRegression => "reduced_motion_regression",
            Self::LabelOrRoleDrift => "label_or_role_drift",
        }
    }

    /// Downgrade triggers a surface must carry when this class auto-narrows it, so the
    /// narrowing is provable against the matrix-owned trigger vocabulary.
    pub const fn related_triggers(self) -> &'static [M5DynamicSurfaceA11yDowngradeTrigger] {
        use M5DynamicSurfaceA11yDowngradeTrigger as D;
        match self {
            Self::BridgeHealth => &[D::BridgeUnavailable, D::BridgePartialOrStale],
            Self::MissingSemanticNode => &[D::BridgePartialOrStale, D::NonVisualFidelityLost],
            Self::AnnouncementRate => &[D::LiveRegionSpam],
            Self::CoalescingViolation => &[D::LiveRegionSpam],
            Self::FocusReturnFailure => &[D::FocusTeleported, D::FocusLost],
            Self::HighZoomRegression
            | Self::HighContrastRegression
            | Self::ReducedMotionRegression => &[D::NonVisualFidelityLost],
            Self::LabelOrRoleDrift => &[D::LabelOrRoleDrift],
        }
    }
}

/// Outcome of one diagnostic check.
///
/// `pass` and `not_applicable` are healthy. `auto_narrowed` is a disclosed claim change
/// the surface made in response to the condition (it keeps shipping at a lower
/// qualification). `regressed` is an unhandled regression that, when blocking, fails the
/// release gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticOutcome {
    /// The check passed.
    Pass,
    /// An unhandled regression was detected.
    Regressed,
    /// The surface auto-narrowed (disclosed) in response to the condition.
    AutoNarrowed,
    /// The check does not apply to this surface.
    NotApplicable,
}

impl M5DiagnosticOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Pass,
        Self::Regressed,
        Self::AutoNarrowed,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Regressed => "regressed",
            Self::AutoNarrowed => "auto_narrowed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the outcome is an unhandled regression.
    pub const fn is_regressed(self) -> bool {
        matches!(self, Self::Regressed)
    }

    /// True when the outcome is a disclosed auto-narrowing.
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::AutoNarrowed)
    }

    /// True when the outcome is healthy (pass or not-applicable).
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Pass | Self::NotApplicable)
    }
}

/// Severity of a diagnostic check: whether a regression of this class blocks release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticSeverity {
    /// A regression of this class blocks the release gate.
    Blocking,
    /// A regression of this class is recorded but does not block release.
    Advisory,
}

impl M5DiagnosticSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 2] = [Self::Blocking, Self::Advisory];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Advisory => "advisory",
        }
    }

    /// True when a regression of this class blocks release.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Blocking)
    }
}

/// Visual-adaptation mode a surface is probed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualAdaptationMode {
    /// High OS zoom / large text.
    HighZoom,
    /// High contrast / forced colors.
    HighContrast,
    /// Reduced motion.
    ReducedMotion,
}

impl M5VisualAdaptationMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::HighZoom, Self::HighContrast, Self::ReducedMotion];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighZoom => "high_zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }

    /// The diagnostic class a regression of this mode maps to.
    pub const fn diagnostic_class(self) -> M5AtDiagnosticClass {
        match self {
            Self::HighZoom => M5AtDiagnosticClass::HighZoomRegression,
            Self::HighContrast => M5AtDiagnosticClass::HighContrastRegression,
            Self::ReducedMotion => M5AtDiagnosticClass::ReducedMotionRegression,
        }
    }
}

/// Release-gate decision for a surface or the report as a whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleaseGateDecision {
    /// The surface may ship at its (possibly narrowed) claim.
    Pass,
    /// The surface is blocked from release by a blocking regression.
    Blocked,
}

impl M5ReleaseGateDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 2] = [Self::Pass, Self::Blocked];

    /// Stable token recorded in the report.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Blocked => "blocked",
        }
    }

    /// True when the decision blocks release.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Semantic-node coverage measured by a bridge probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SemanticNodeCoverage {
    /// Number of semantic nodes the surface descriptor expects on the bridge.
    pub expected_nodes: u32,
    /// Number of semantic nodes currently mapped onto the bridge.
    pub present_nodes: u32,
    /// Number of semantic nodes missing from the bridge.
    pub missing_nodes: u32,
}

impl M5SemanticNodeCoverage {
    /// True when `present + missing == expected`, the coverage accounting is closed.
    pub const fn is_consistent(&self) -> bool {
        self.present_nodes + self.missing_nodes == self.expected_nodes
    }
}

/// OS accessibility-bridge probe for one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceBridgeProbe {
    /// Bridge this surface maps into.
    pub bridge_kind: M5SurfaceBridgeKind,
    /// Current bridge connection state (health).
    pub bridge_state: A11yBridgeState,
    /// Non-visual fidelity the bridge currently delivers for this surface.
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// Semantic-node coverage on the bridge.
    pub semantic_node_coverage: M5SemanticNodeCoverage,
    /// Disclosed reason the mapping is degraded, if any.
    pub degradation_reason: M5BridgeDegradationReason,
    /// Stable message id for the probe; prefixed [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub probe_message_id: String,
}

/// One assistive-technology diagnostic check against a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticCheck {
    /// Diagnostic class.
    pub class: M5AtDiagnosticClass,
    /// Outcome of the check.
    pub outcome: M5DiagnosticOutcome,
    /// Whether a regression of this class blocks release.
    pub severity: M5DiagnosticSeverity,
    /// Stable message id describing the finding; prefixed
    /// [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
    /// Export-safe evidence ref (an id, never a raw payload) backing the finding.
    pub evidence_ref: String,
    /// Focus-return disposition the surface fell back to; present only for the
    /// [`M5AtDiagnosticClass::FocusReturnFailure`] class so the focus-contract failure is
    /// exportable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_return_disposition: Option<A11yFocusReturnDisposition>,
}

/// Announcement-spam budget and the surface's measured live-region traffic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnnouncementBudgetCheck {
    /// Published announcement-spam budget (grammar-owned coalescing-budget shape).
    pub budget: M5CoalescingBudget,
    /// Announcements observed within the budget window.
    pub observed_announcements_in_window: u32,
    /// Minimum spacing observed between announcements, in milliseconds.
    pub observed_min_interval_ms: u32,
    /// True when the observed traffic stays within the published budget.
    pub within_budget: bool,
    /// Stable message id for the budget check; prefixed
    /// [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub budget_message_id: String,
}

impl M5AnnouncementBudgetCheck {
    /// Whether the observed traffic actually fits the published budget.
    fn observed_within_budget(&self) -> bool {
        self.observed_announcements_in_window <= self.budget.max_announcements_per_window
            && self.observed_min_interval_ms >= self.budget.min_interval_ms
    }
}

/// High-zoom, high-contrast, and reduced-motion conformance outcomes for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualConformanceCheck {
    /// High-zoom / large-text outcome.
    pub high_zoom: M5DiagnosticOutcome,
    /// High-contrast / forced-colors outcome.
    pub high_contrast: M5DiagnosticOutcome,
    /// Reduced-motion outcome.
    pub reduced_motion: M5DiagnosticOutcome,
    /// Stable message id for the conformance check; prefixed
    /// [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub conformance_message_id: String,
}

impl M5VisualConformanceCheck {
    /// The outcome recorded for the given mode.
    fn outcome_for(&self, mode: M5VisualAdaptationMode) -> M5DiagnosticOutcome {
        match mode {
            M5VisualAdaptationMode::HighZoom => self.high_zoom,
            M5VisualAdaptationMode::HighContrast => self.high_contrast,
            M5VisualAdaptationMode::ReducedMotion => self.reduced_motion,
        }
    }
}

/// Current degraded-state disclosure for a surface, mirroring its bridge probe.
///
/// This is the block a support bundle exports to explain *why* a surface is degraded in
/// the same object/state vocabulary the user saw in-product.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateDisclosure {
    /// True when the surface's bridge or non-visual fidelity is currently degraded.
    pub is_degraded: bool,
    /// Bridge state, mirrored from the probe.
    pub bridge_state: A11yBridgeState,
    /// Non-visual fidelity, mirrored from the probe.
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// Disclosed degradation reason, mirrored from the probe.
    pub degradation_reason: M5BridgeDegradationReason,
    /// Stable message id for the disclosure; prefixed
    /// [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub disclosure_message_id: String,
}

/// Per-surface release gate the release/public-truth automation reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceReleaseGate {
    /// Gate decision for this surface.
    pub decision: M5ReleaseGateDecision,
    /// Diagnostic classes that block this surface; empty iff the decision is `pass`.
    pub blocking_finding_classes: Vec<M5AtDiagnosticClass>,
    /// Stable message id for the gate; prefixed [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// One surface's diagnostics row: probe, checks, budget, conformance, degraded state,
/// and release gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceDiagnostics {
    /// Stable surface id, unique within the report.
    pub surface_id: String,
    /// Protected custom-rendered surface family (descriptor-owned).
    pub surface_family: M5SurfaceFamily,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Owner role accountable for keeping this surface's diagnostics current.
    pub owner_role: String,
    /// Object identity this row is bound to — the SAME identity the descriptor and the
    /// visual surface carry, so diagnostics never drift from the object.
    pub object_identity_ref: String,
    /// Qualification class the surface currently claims.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// OS accessibility-bridge probe.
    pub bridge_probe: M5SurfaceBridgeProbe,
    /// Diagnostic checks; one per [`M5AtDiagnosticClass`].
    pub checks: Vec<M5DiagnosticCheck>,
    /// Announcement-spam budget and observed traffic.
    pub announcement_budget: M5AnnouncementBudgetCheck,
    /// High-zoom / high-contrast / reduced-motion conformance.
    pub visual_conformance: M5VisualConformanceCheck,
    /// Current degraded-state disclosure.
    pub current_degraded_state: M5DegradedStateDisclosure,
    /// Per-surface release gate.
    pub gate: M5SurfaceReleaseGate,
    /// Reopenable durable fallback surface that preserves this row's identity.
    pub durable_fallback: M5DurableFallbackRef,
    /// Downgrade triggers that can narrow this surface below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this row's diagnostics.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

impl M5SurfaceDiagnostics {
    /// Finds the check for the given class, if present.
    fn check(&self, class: M5AtDiagnosticClass) -> Option<&M5DiagnosticCheck> {
        self.checks.iter().find(|c| c.class == class)
    }

    /// The sorted, unique set of blocking diagnostic classes (a `regressed` check whose
    /// severity is `blocking`). This is the single source of truth the release gate must
    /// agree with.
    fn computed_blocking_classes(&self) -> Vec<M5AtDiagnosticClass> {
        let mut classes: BTreeSet<M5AtDiagnosticClass> = BTreeSet::new();
        for check in &self.checks {
            if check.outcome.is_regressed() && check.severity.is_blocking() {
                classes.insert(check.class);
            }
        }
        classes.into_iter().collect()
    }

    /// True when any check is a disclosed auto-narrowing.
    fn is_narrowed(&self) -> bool {
        self.checks.iter().any(|c| c.outcome.is_narrowed())
    }

    /// True when every check is healthy (no regression, no narrowing).
    fn is_green(&self) -> bool {
        self.checks.iter().all(|c| c.outcome.is_healthy())
    }
}

/// Self-describing controlled-vocabulary set for the diagnostics-shaped tokens this lane
/// mints, plus the descriptor-owned surface/bridge tokens these diagnostics reuse so the
/// report resolves every token it carries on its own. The shared state tokens live in the
/// matrix; the coalescing-budget and durable-fallback tokens live in the grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticsVocabularySet {
    /// Diagnostic-class tokens.
    pub diagnostic_classes: Vec<String>,
    /// Diagnostic-outcome tokens.
    pub diagnostic_outcomes: Vec<String>,
    /// Diagnostic-severity tokens.
    pub diagnostic_severities: Vec<String>,
    /// Visual-adaptation-mode tokens.
    pub visual_adaptation_modes: Vec<String>,
    /// Release-gate-decision tokens.
    pub release_gate_decisions: Vec<String>,
    /// Surface-family tokens (descriptor-owned).
    pub surface_families: Vec<String>,
    /// Bridge-kind tokens (descriptor-owned).
    pub bridge_kinds: Vec<String>,
    /// Bridge-degradation-reason tokens (descriptor-owned).
    pub bridge_degradation_reasons: Vec<String>,
}

impl M5DiagnosticsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            diagnostic_classes: M5AtDiagnosticClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            diagnostic_outcomes: M5DiagnosticOutcome::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            diagnostic_severities: M5DiagnosticSeverity::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            visual_adaptation_modes: M5VisualAdaptationMode::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            release_gate_decisions: M5ReleaseGateDecision::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            surface_families: M5SurfaceFamily::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            bridge_kinds: M5SurfaceBridgeKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            bridge_degradation_reasons: M5BridgeDegradationReason::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the diagnostics lane.
///
/// Every flag is a hard invariant; all must hold for the report to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticsConformanceReview {
    /// Every protected surface family has an exportable diagnostics row.
    pub every_protected_surface_has_diagnostics: bool,
    /// Bridge state and missing semantic nodes are diagnosable per surface.
    pub bridge_state_and_missing_nodes_diagnosable: bool,
    /// Announcement-rate and coalescing violations are diagnosable per surface.
    pub announcement_rate_and_coalescing_diagnosable: bool,
    /// Focus-return failures are diagnosable, with the fallback disposition exported.
    pub focus_return_failures_diagnosable: bool,
    /// High-zoom, high-contrast, and reduced-motion regressions are diagnosable.
    pub zoom_contrast_motion_regressions_diagnosable: bool,
    /// Announcement-spam budgets are published and enforced per surface.
    pub announcement_spam_budgets_enforced: bool,
    /// The release gate fails surfaces with blocking regressions.
    pub release_gate_fails_on_blocking_regressions: bool,
    /// Current degraded state is disclosed, never hidden.
    pub degraded_state_disclosed_not_hidden: bool,
    /// Diagnostics reuse the descriptor object identity rather than re-deriving it.
    pub diagnostics_reuse_descriptor_object_identity: bool,
    /// Support/export carries bridge health, message ids, focus failures, and degraded
    /// state without leaking unrelated content.
    pub support_export_carries_bridge_health_message_ids_focus_failures_degraded_state: bool,
    /// Claimed surfaces auto-narrow when bridge or proof state goes stale.
    pub claimed_surfaces_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Per-surface bridge and announcement diagnostics are not replaced by an aggregate
    /// pass/fail dashboard.
    pub per_surface_diagnostics_not_replaced_by_aggregate_dashboard: bool,
}

/// Consumer projection block: who reads the diagnostics report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticsConsumerProjection {
    /// Shell surfaces the diagnostics health to the user.
    pub shell_consumes_diagnostics: bool,
    /// Editor surface is diagnosed.
    pub editor_surface_diagnosed: bool,
    /// Terminal surface is diagnosed.
    pub terminal_surface_diagnosed: bool,
    /// Dense data grid / collection surface is diagnosed.
    pub data_grid_surface_diagnosed: bool,
    /// Notebook surface is diagnosed.
    pub notebook_surface_diagnosed: bool,
    /// Review / diff surface is diagnosed.
    pub review_surface_diagnosed: bool,
    /// Support export reuses the diagnostics report.
    pub support_export_consumes_diagnostics: bool,
    /// Help / docs document the diagnostics packet.
    pub help_documents_diagnostics: bool,
    /// Release/public-truth automation gates on the diagnostics report.
    pub release_public_truth_gates_on_diagnostics: bool,
    /// Assistive-tech conformance packets reuse the diagnostics report.
    pub at_conformance_packets_reuse_diagnostics: bool,
}

/// Report-level release gate aggregating the per-surface gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticsReleaseGate {
    /// True when at least one surface is blocked from release.
    pub blocks_release: bool,
    /// Sorted surface ids that are blocked from release.
    pub blocked_surface_ids: Vec<String>,
    /// Stable message id for the aggregate gate; prefixed
    /// [`M5_DIAGNOSTIC_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Constructor input for [`M5DynamicA11yDiagnosticsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DynamicA11yDiagnosticsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Per-surface diagnostics rows.
    pub surfaces: Vec<M5SurfaceDiagnostics>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Diagnostics-shaped controlled-vocabulary set.
    pub diagnostics_vocabulary_set: M5DiagnosticsVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DiagnosticsConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiagnosticsConsumerProjection,
    /// Report-level release gate.
    pub release_gate: M5DiagnosticsReleaseGate,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 dynamic-surface AT diagnostics report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicA11yDiagnosticsPacket {
    /// Record kind; must equal [`M5_DYNAMIC_A11Y_DIAGNOSTICS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Per-surface diagnostics rows.
    pub surfaces: Vec<M5SurfaceDiagnostics>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Diagnostics-shaped controlled-vocabulary set.
    pub diagnostics_vocabulary_set: M5DiagnosticsVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DiagnosticsConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiagnosticsConsumerProjection,
    /// Report-level release gate.
    pub release_gate: M5DiagnosticsReleaseGate,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DynamicA11yDiagnosticsPacket {
    /// Builds a diagnostics report packet from seed input.
    pub fn new(input: M5DynamicA11yDiagnosticsPacketInput) -> Self {
        Self {
            record_kind: M5_DYNAMIC_A11Y_DIAGNOSTICS_RECORD_KIND.to_owned(),
            schema_version: M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            surfaces: input.surfaces,
            shared_vocabulary_set: input.shared_vocabulary_set,
            diagnostics_vocabulary_set: input.diagnostics_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            release_gate: input.release_gate,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must fail this report's rows because
    /// at least one protected surface is blocked.
    pub fn blocks_release(&self) -> bool {
        self.release_gate.blocks_release
    }

    /// Surface ids currently blocked from release.
    pub fn blocked_surface_ids(&self) -> Vec<&str> {
        self.surfaces
            .iter()
            .filter(|s| s.gate.decision.blocks())
            .map(|s| s.surface_id.as_str())
            .collect()
    }

    /// Validates the diagnostics-report invariants.
    pub fn validate(&self) -> Vec<M5DiagnosticsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DYNAMIC_A11Y_DIAGNOSTICS_RECORD_KIND {
            violations.push(M5DiagnosticsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_VERSION {
            violations.push(M5DiagnosticsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DiagnosticsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_surfaces(self, &mut violations);
        validate_diagnostic_class_coverage(self, &mut violations);
        validate_release_gate_aggregate(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 diagnostics report serializes"),
        ) {
            violations.push(M5DiagnosticsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 diagnostics report serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked = self
            .surfaces
            .iter()
            .filter(|s| s.gate.decision.blocks())
            .count();
        let narrowed = self.surfaces.iter().filter(|s| s.is_narrowed()).count();
        let mut out = String::new();
        out.push_str("# M5 Dynamic-Surface Assistive-Tech Diagnostics\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} green, {} narrowed, {} blocked)\n",
            self.surfaces.len(),
            self.surfaces.iter().filter(|s| s.is_green()).count(),
            narrowed,
            blocked
        ));
        out.push_str(&format!(
            "- Release gate: {} ({} blocked surfaces)\n",
            if self.release_gate.blocks_release {
                "blocked"
            } else {
                "pass"
            },
            self.release_gate.blocked_surface_ids.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}`, gate `{}`\n",
                surface.surface_id,
                surface.surface_family.as_str(),
                surface.qualification.as_str(),
                surface.gate.decision.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", surface.owner_role));
            out.push_str(&format!(
                "  - Object identity: `{}`\n",
                surface.object_identity_ref
            ));
            out.push_str(&format!(
                "  - Bridge: {} / {} ({}, {} missing nodes)\n",
                surface.bridge_probe.bridge_kind.as_str(),
                surface.bridge_probe.bridge_state.as_str(),
                surface.bridge_probe.degradation_reason.as_str(),
                surface.bridge_probe.semantic_node_coverage.missing_nodes
            ));
            out.push_str(&format!(
                "  - Announcement budget: observed {} / max {} per {}s ({})\n",
                surface.announcement_budget.observed_announcements_in_window,
                surface
                    .announcement_budget
                    .budget
                    .max_announcements_per_window,
                surface.announcement_budget.budget.window_seconds,
                if surface.announcement_budget.within_budget {
                    "within budget"
                } else {
                    "over budget"
                }
            ));
            out.push_str(&format!(
                "  - Visual conformance: zoom `{}` / contrast `{}` / motion `{}`\n",
                surface.visual_conformance.high_zoom.as_str(),
                surface.visual_conformance.high_contrast.as_str(),
                surface.visual_conformance.reduced_motion.as_str()
            ));
            out.push_str(&format!(
                "  - Degraded state: {}\n",
                if surface.current_degraded_state.is_degraded {
                    surface.current_degraded_state.degradation_reason.as_str()
                } else {
                    "none"
                }
            ));
            for check in &surface.checks {
                if !check.outcome.is_healthy() {
                    out.push_str(&format!(
                        "  - check `{}` -> {} ({}, `{}`)\n",
                        check.class.as_str(),
                        check.outcome.as_str(),
                        check.severity.as_str(),
                        check.detail_message_id
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in diagnostics-report export.
#[derive(Debug)]
pub enum M5DiagnosticsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DiagnosticsViolation>),
}

impl fmt::Display for M5DiagnosticsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 diagnostics report parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 diagnostics report failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DiagnosticsArtifactError {}

/// Validation failures emitted by [`M5DynamicA11yDiagnosticsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DiagnosticsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A protected surface family has no diagnostics row.
    RequiredSurfaceFamilyMissing,
    /// Two rows share a surface id.
    DuplicateSurfaceId,
    /// A diagnostics row is incomplete.
    DiagnosticsRowIncomplete,
    /// A row is not bound to an object identity.
    MissingObjectIdentity,
    /// A diagnostic class is never exercised across the report.
    DiagnosticClassNotExercised,
    /// A row does not run exactly one check per diagnostic class.
    DiagnosticChecksNotOnePerClass,
    /// A diagnostic check is incomplete.
    DiagnosticCheckIncomplete,
    /// A check's focus-return disposition is present without the focus class, or absent
    /// with it.
    FocusDispositionMismatch,
    /// A bridge probe is internally inconsistent.
    BridgeProbeInconsistent,
    /// An announcement-budget check is internally inconsistent.
    AnnouncementBudgetInconsistent,
    /// The announcement budget verdict disagrees with the announcement/coalescing checks.
    BudgetOutcomeMismatch,
    /// A visual-conformance mode disagrees with its diagnostic-class check.
    VisualConformanceMismatch,
    /// The degraded-state disclosure disagrees with the bridge probe.
    DegradedStateInconsistent,
    /// A surface's release gate disagrees with its blocking regressions.
    GateDecisionInconsistent,
    /// A narrowed or green surface is internally inconsistent with its claim.
    NarrowingInconsistent,
    /// A diagnostics message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row has no reopenable durable fallback surface.
    DurableFallbackMissing,
    /// The report-level release gate disagrees with the per-surface gates.
    ReleaseGateAggregateInconsistent,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5DiagnosticsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceFamilyMissing => "required_surface_family_missing",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::DiagnosticsRowIncomplete => "diagnostics_row_incomplete",
            Self::MissingObjectIdentity => "missing_object_identity",
            Self::DiagnosticClassNotExercised => "diagnostic_class_not_exercised",
            Self::DiagnosticChecksNotOnePerClass => "diagnostic_checks_not_one_per_class",
            Self::DiagnosticCheckIncomplete => "diagnostic_check_incomplete",
            Self::FocusDispositionMismatch => "focus_disposition_mismatch",
            Self::BridgeProbeInconsistent => "bridge_probe_inconsistent",
            Self::AnnouncementBudgetInconsistent => "announcement_budget_inconsistent",
            Self::BudgetOutcomeMismatch => "budget_outcome_mismatch",
            Self::VisualConformanceMismatch => "visual_conformance_mismatch",
            Self::DegradedStateInconsistent => "degraded_state_inconsistent",
            Self::GateDecisionInconsistent => "gate_decision_inconsistent",
            Self::NarrowingInconsistent => "narrowing_inconsistent",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DurableFallbackMissing => "durable_fallback_missing",
            Self::ReleaseGateAggregateInconsistent => "release_gate_aggregate_inconsistent",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable diagnostics-report export.
pub fn current_stable_m5_dynamic_a11y_diagnostics_export(
) -> Result<M5DynamicA11yDiagnosticsPacket, M5DiagnosticsArtifactError> {
    let packet: M5DynamicA11yDiagnosticsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json"
    )))
    .map_err(M5DiagnosticsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DiagnosticsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SCHEMA_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_DOC_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_MATRIX_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SURFACE_DESCRIPTOR_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_ANNOUNCEMENT_GRAMMAR_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_SCREEN_READER_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_FOCUS_CONTRACT_REF,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_VISUAL_ADAPTATION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DiagnosticsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.diagnostics_vocabulary_set.matches_canonical()
    {
        violations.push(M5DiagnosticsViolation::VocabularySetDrift);
    }
}

fn validate_surfaces(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let present: BTreeSet<M5SurfaceFamily> =
        packet.surfaces.iter().map(|s| s.surface_family).collect();
    for required in M5SurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DiagnosticsViolation::RequiredSurfaceFamilyMissing);
            break;
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &packet.surfaces {
        if !seen_ids.insert(surface.surface_id.as_str()) {
            violations.push(M5DiagnosticsViolation::DuplicateSurfaceId);
        }
        if surface.surface_id.trim().is_empty()
            || surface.surface_label.trim().is_empty()
            || surface.owner_role.trim().is_empty()
            || surface.source_contract_refs.is_empty()
        {
            violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
        }
        if surface.object_identity_ref.trim().is_empty() {
            violations.push(M5DiagnosticsViolation::MissingObjectIdentity);
        }

        validate_surface_checks(surface, violations);
        validate_surface_bridge_probe(surface, violations);
        validate_surface_announcement_budget(surface, violations);
        validate_surface_visual_conformance(surface, violations);
        validate_surface_degraded_state(surface, violations);
        validate_surface_gate(surface, violations);
        validate_surface_narrowing(surface, violations);

        if surface.qualification.is_stable() && surface.required_proof_packet_refs.is_empty() {
            violations.push(M5DiagnosticsViolation::StableSurfaceMissingProof);
        }
        if surface.downgrade_triggers.is_empty() {
            violations.push(M5DiagnosticsViolation::DowngradeTriggersMissing);
        }
        if surface.consumer_surfaces.is_empty() {
            violations.push(M5DiagnosticsViolation::ConsumerSurfacesMissing);
        }
        if surface.durable_fallback.surface_ref.trim().is_empty()
            || !surface.durable_fallback.reopenable
        {
            violations.push(M5DiagnosticsViolation::DurableFallbackMissing);
        }
    }
}

fn validate_surface_checks(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    // Each surface must run exactly the full battery: one check per diagnostic class.
    let mut seen: BTreeSet<M5AtDiagnosticClass> = BTreeSet::new();
    for check in &surface.checks {
        seen.insert(check.class);
        if check.detail_message_id.trim().is_empty() || check.evidence_ref.trim().is_empty() {
            violations.push(M5DiagnosticsViolation::DiagnosticCheckIncomplete);
        }
        if !check
            .detail_message_id
            .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
        {
            violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
        }
        // The focus-return disposition is exported only for the focus class, so a support
        // bundle can read the focus-contract failure exactly where it applies.
        let is_focus = check.class == M5AtDiagnosticClass::FocusReturnFailure;
        if is_focus != check.focus_return_disposition.is_some() {
            violations.push(M5DiagnosticsViolation::FocusDispositionMismatch);
        }
    }
    if seen.len() != M5AtDiagnosticClass::ALL.len() || surface.checks.len() != seen.len() {
        violations.push(M5DiagnosticsViolation::DiagnosticChecksNotOnePerClass);
    }
}

fn validate_surface_bridge_probe(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let probe = &surface.bridge_probe;
    if probe.probe_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !probe
        .probe_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    // The coverage accounting must close, a healthy bridge must not disclose a
    // degradation, and a degraded bridge must disclose one — so missing nodes and a
    // dropped bridge can never hide behind a green probe.
    let healthy = probe.bridge_state == A11yBridgeState::BridgedActive;
    let consistent = probe.semantic_node_coverage.is_consistent()
        && (healthy != probe.degradation_reason.is_degraded())
        && (healthy == (probe.semantic_node_coverage.missing_nodes == 0));
    if !consistent {
        violations.push(M5DiagnosticsViolation::BridgeProbeInconsistent);
    }
}

fn validate_surface_announcement_budget(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let budget = &surface.announcement_budget;
    if budget.budget_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !budget
        .budget_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    // A published spam budget must actually bound the live region: a real coalescing
    // strategy and positive caps. The verdict must match the observed traffic.
    if budget.budget.strategy == A11yCoalescingStrategy::None
        || budget.budget.max_announcements_per_window == 0
        || budget.budget.window_seconds == 0
        || budget.within_budget != budget.observed_within_budget()
    {
        violations.push(M5DiagnosticsViolation::AnnouncementBudgetInconsistent);
    }

    // The announcement/coalescing diagnostic checks must agree with the budget verdict:
    // an over-budget surface cannot show passing announcement checks.
    let rate_ok = surface
        .check(M5AtDiagnosticClass::AnnouncementRate)
        .map(|c| c.outcome.is_healthy())
        .unwrap_or(false);
    let coalescing_ok = surface
        .check(M5AtDiagnosticClass::CoalescingViolation)
        .map(|c| c.outcome.is_healthy())
        .unwrap_or(false);
    if budget.within_budget {
        if !(rate_ok && coalescing_ok) {
            violations.push(M5DiagnosticsViolation::BudgetOutcomeMismatch);
        }
    } else if rate_ok && coalescing_ok {
        violations.push(M5DiagnosticsViolation::BudgetOutcomeMismatch);
    }
}

fn validate_surface_visual_conformance(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let conformance = &surface.visual_conformance;
    if conformance.conformance_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !conformance
        .conformance_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    // Each per-mode outcome must mirror the matching diagnostic-class check, so a
    // zoom/contrast/motion regression always lands in the gate-bearing check.
    for mode in M5VisualAdaptationMode::ALL {
        let mode_outcome = conformance.outcome_for(mode);
        let class_outcome = surface
            .check(mode.diagnostic_class())
            .map(|c| c.outcome)
            .unwrap_or(M5DiagnosticOutcome::NotApplicable);
        if mode_outcome != class_outcome {
            violations.push(M5DiagnosticsViolation::VisualConformanceMismatch);
            return;
        }
    }
}

fn validate_surface_degraded_state(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let state = &surface.current_degraded_state;
    let probe = &surface.bridge_probe;
    if state.disclosure_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !state
        .disclosure_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    // The disclosure must faithfully mirror the probe, and `is_degraded` must follow from
    // the probe state — a degraded bridge can never be disclosed as healthy.
    let expected_degraded = probe.bridge_state != A11yBridgeState::BridgedActive
        || probe.non_visual_fidelity != A11yNonVisualFidelity::FullAccessible
        || probe.degradation_reason.is_degraded();
    if state.bridge_state != probe.bridge_state
        || state.non_visual_fidelity != probe.non_visual_fidelity
        || state.degradation_reason != probe.degradation_reason
        || state.is_degraded != expected_degraded
    {
        violations.push(M5DiagnosticsViolation::DegradedStateInconsistent);
    }
}

fn validate_surface_gate(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    if surface.gate.gate_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !surface
        .gate
        .gate_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    // The gate is a deterministic function of the blocking regressions: it blocks iff a
    // blocking regression is present, and it must name exactly those classes.
    let computed = surface.computed_blocking_classes();
    let mut declared = surface.gate.blocking_finding_classes.clone();
    declared.sort_unstable();
    declared.dedup();
    let expected_decision = if computed.is_empty() {
        M5ReleaseGateDecision::Pass
    } else {
        M5ReleaseGateDecision::Blocked
    };
    if surface.gate.decision != expected_decision || declared != computed {
        violations.push(M5DiagnosticsViolation::GateDecisionInconsistent);
    }
}

fn validate_surface_narrowing(
    surface: &M5SurfaceDiagnostics,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let blocked = !surface.computed_blocking_classes().is_empty();
    if surface.is_green() {
        // A fully green surface must claim Stable and pass its gate cleanly.
        if !surface.qualification.is_stable() || blocked {
            violations.push(M5DiagnosticsViolation::NarrowingInconsistent);
        }
        return;
    }
    if surface.is_narrowed() {
        // An auto-narrowed surface must drop below Stable, must not also be blocked, and
        // must carry a downgrade trigger matching every narrowed class so the narrowing
        // is provable.
        if surface.qualification.is_stable() || blocked {
            violations.push(M5DiagnosticsViolation::NarrowingInconsistent);
            return;
        }
        for check in &surface.checks {
            if check.outcome.is_narrowed() {
                let has_trigger = check
                    .class
                    .related_triggers()
                    .iter()
                    .any(|t| surface.downgrade_triggers.contains(t));
                if !has_trigger {
                    violations.push(M5DiagnosticsViolation::NarrowingInconsistent);
                    return;
                }
            }
        }
    }
}

fn validate_diagnostic_class_coverage(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let present: BTreeSet<M5AtDiagnosticClass> = packet
        .surfaces
        .iter()
        .flat_map(|s| s.checks.iter().map(|c| c.class))
        .collect();
    for class in M5AtDiagnosticClass::ALL {
        if !present.contains(&class) {
            violations.push(M5DiagnosticsViolation::DiagnosticClassNotExercised);
            return;
        }
    }
}

fn validate_release_gate_aggregate(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let gate = &packet.release_gate;
    if gate.gate_message_id.trim().is_empty() {
        violations.push(M5DiagnosticsViolation::DiagnosticsRowIncomplete);
    }
    if !gate
        .gate_message_id
        .starts_with(M5_DIAGNOSTIC_MESSAGE_ID_PREFIX)
    {
        violations.push(M5DiagnosticsViolation::MessageIdPrefixMissing);
    }
    let mut expected_blocked: Vec<String> = packet
        .surfaces
        .iter()
        .filter(|s| s.gate.decision.blocks())
        .map(|s| s.surface_id.clone())
        .collect();
    expected_blocked.sort();
    let mut declared = gate.blocked_surface_ids.clone();
    declared.sort();
    if gate.blocks_release == expected_blocked.is_empty() || declared != expected_blocked {
        violations.push(M5DiagnosticsViolation::ReleaseGateAggregateInconsistent);
    }
}

fn validate_conformance_review(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.every_protected_surface_has_diagnostics,
        review.bridge_state_and_missing_nodes_diagnosable,
        review.announcement_rate_and_coalescing_diagnosable,
        review.focus_return_failures_diagnosable,
        review.zoom_contrast_motion_regressions_diagnosable,
        review.announcement_spam_budgets_enforced,
        review.release_gate_fails_on_blocking_regressions,
        review.degraded_state_disclosed_not_hidden,
        review.diagnostics_reuse_descriptor_object_identity,
        review.support_export_carries_bridge_health_message_ids_focus_failures_degraded_state,
        review.claimed_surfaces_auto_narrow_when_bridge_or_proof_stale,
        review.per_surface_diagnostics_not_replaced_by_aggregate_dashboard,
    ] {
        if !ok {
            violations.push(M5DiagnosticsViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_diagnostics,
        projection.editor_surface_diagnosed,
        projection.terminal_surface_diagnosed,
        projection.data_grid_surface_diagnosed,
        projection.notebook_surface_diagnosed,
        projection.review_surface_diagnosed,
        projection.support_export_consumes_diagnostics,
        projection.help_documents_diagnostics,
        projection.release_public_truth_gates_on_diagnostics,
        projection.at_conformance_packets_reuse_diagnostics,
    ] {
        if !ok {
            violations.push(M5DiagnosticsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DiagnosticsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DynamicA11yDiagnosticsPacket,
    violations: &mut Vec<M5DiagnosticsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5DiagnosticsViolation::ReleasePostureIncomplete);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
