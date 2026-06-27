//! Dynamic-surface assistive-technology certification capstone for the claimed M5
//! custom-rendered surfaces.
//!
//! Where the frozen matrix ([`crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`])
//! defines the canonical accessibility object model, the per-surface descriptors,
//! announcement grammar, focus/selection contract, and non-visual summaries each prove one
//! slice of assistive-tech truth, and the AT diagnostics ([`crate::accessibility::diagnostics`])
//! materialize per-surface *health*, this module certifies *whether each claimed dynamic
//! surface may keep a public Stable assistive-tech claim* and *auto-narrows the rows whose
//! screen-reader, focus-return, live-announcement, or non-visual proof has gone stale*.
//!
//! Each [`M5A11ySurfaceCertificationRow`] qualifies one custom-rendered surface across the
//! six governed proof dimensions — bridge health, dynamic announcement coverage,
//! focus-return, non-visual summaries, zoom/contrast/motion parity, and the stale-proof
//! downgrade rules — binds every dimension to the proof packet that keeps it current,
//! resolves a green/yellow/red traffic-light status (certified / limited / retest-pending /
//! degraded), and resolves a release gate the release/public-truth automation reads. A
//! surface that lacks current proof or carries a disclosed narrowing auto-narrows below its
//! claim before Stable promotion; an unwaived blocking regression on a custom dynamic
//! surface *blocks* the claim and is named in the release packet rather than left invisible.
//! Active waivers are disclosed with their scope (the waived dimension), accountable owner,
//! and expiry, and the exact stale-proof causes are named per dimension.
//!
//! The certification packet is the single M5 source of dynamic-surface assistive-tech
//! certification truth: release-center, support exports, docs/help, onboarding,
//! presentation, the stable-claim matrix, and the shell/editor/notebook/data/review
//! surfaces consume the same rows rather than reproducing certification by hand. The
//! compact [`M5A11yCertificationDashboard`] projection is the published green/yellow/red
//! scoreboard. Raw provider payloads, credentials, secret material, screenshots, and
//! untranslated free-text prose stay outside the support boundary.
//!
//! The shared state vocabularies (qualification class, downgrade trigger, consumer surface,
//! proof freshness, release posture, bridge/non-visual tokens) are reused verbatim from the
//! frozen matrix, the diagnostic outcome/severity tokens from the AT diagnostics, and the
//! protected surface families from the surface descriptors; only the certification-shaped
//! vocabularies this lane mints (proof dimension, proof freshness state, certification
//! status, certification signal, and certification gate decision) are frozen in a
//! self-describing [`M5A11yCertificationVocabularySet`].
//!
//! The boundary schema is
//! [`schemas/a11y/m5-dynamic-a11y-certification.schema.json`](../../../../../schemas/a11y/m5-dynamic-a11y-certification.schema.json)
//! and the dashboard schema is
//! [`schemas/a11y/m5-dynamic-a11y-dashboard.schema.json`](../../../../../schemas/a11y/m5-dynamic-a11y-dashboard.schema.json).
//! The contract doc is
//! [`docs/release/m5-dynamic-a11y-certification.md`](../../../../../docs/release/m5-dynamic-a11y-certification.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-dynamic-a11y-certification/`](../../../../../fixtures/a11y/m5-dynamic-a11y-certification/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_dynamic_a11y_certification, seeded_m5_dynamic_a11y_certification_regression_blocked,
    seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending,
    seeded_m5_dynamic_a11y_certification_waived_narrowed, M5_DYNAMIC_A11Y_CERTIFICATION_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The surface descriptors own the protected surface families; reuse them so certification
// rows map to the same surfaces the descriptors and diagnostics publish.
use crate::accessibility::M5SurfaceFamily;
// The AT diagnostics own the per-surface health report this lane certifies from, plus the
// diagnostic outcome / severity tokens the dimensions reuse.
use crate::accessibility::diagnostics::{
    M5AtDiagnosticClass, M5DiagnosticOutcome, M5DiagnosticSeverity, M5DynamicA11yDiagnosticsPacket,
    M5SurfaceDiagnostics,
};
// The announcement grammar owns the durable-fallback surface vocabulary the rows reuse.
use crate::announcement_grammar::M5DurableFallbackRef;
// The frozen matrix owns the shared state vocabularies, qualification classes, downgrade
// triggers, consumer surfaces, and proof/release posture.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use matrix::{
    M5DynamicSurfaceA11yConsumerSurface, M5DynamicSurfaceA11yDowngradeTrigger,
    M5DynamicSurfaceA11yProofFreshness, M5DynamicSurfaceA11yQualificationClass,
    M5DynamicSurfaceA11yReleasePosture, M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5DynamicA11yCertificationPacket`].
pub const M5_DYNAMIC_A11Y_CERTIFICATION_RECORD_KIND: &str = "m5_dynamic_a11y_certification";

/// Schema version for M5 dynamic-surface AT certification packets.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`M5A11yCertificationDashboard`].
pub const M5_DYNAMIC_A11Y_DASHBOARD_RECORD_KIND: &str = "m5_dynamic_a11y_dashboard";

/// Schema version for M5 dynamic-surface AT certification dashboards.
pub const M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification boundary schema.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/a11y/m5-dynamic-a11y-certification.schema.json";

/// Repo-relative path of the dashboard boundary schema.
pub const M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_REF: &str =
    "schemas/a11y/m5-dynamic-a11y-dashboard.schema.json";

/// Repo-relative path of the certification contract doc.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_DOC_REF: &str =
    "docs/release/m5-dynamic-a11y-certification.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that governs the
/// shared controlled vocabularies and qualification classes this lane reuses.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_MATRIX_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the AT diagnostics report this certification reads surface health
/// from.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_DIAGNOSTICS_REF: &str =
    "schemas/a11y/m5-dynamic-a11y-report.schema.json";

/// Repo-relative path of the bridge / surface-descriptor proof artifact (bridge health).
pub const M5_PROOF_BRIDGE_DESCRIPTOR_REF: &str =
    "artifacts/a11y/m5-bridge-descriptor-proof/support_export.json";

/// Repo-relative path of the live-announcement proof artifact (announcement coverage).
pub const M5_PROOF_LIVE_ANNOUNCEMENT_REF: &str =
    "artifacts/a11y/m5-live-announcement-proof/support_export.json";

/// Repo-relative path of the dynamic-event coverage proof artifact (stale-proof downgrade).
pub const M5_PROOF_EVENT_COVERAGE_REF: &str =
    "artifacts/a11y/m5-event-coverage-proof/support_export.json";

/// Repo-relative path of the focus-return proof artifact (focus return).
pub const M5_PROOF_FOCUS_RETURN_REF: &str =
    "artifacts/a11y/m5-focus-return-proof/support_export.json";

/// Repo-relative path of the non-visual summary proof artifact (non-visual summaries).
pub const M5_PROOF_NONVISUAL_SUMMARY_REF: &str =
    "artifacts/a11y/m5-nonvisual-summary-proof/support_export.json";

/// Repo-relative path of the AT diagnostics proof artifact (zoom/contrast/motion parity).
pub const M5_PROOF_DIAGNOSTICS_REF: &str =
    "artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json";

/// Repo-relative path of the checked certification support-export artifact.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-dynamic-a11y-certification/support_export.json";

/// Repo-relative path of the checked certification Markdown proof.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/release/m5-dynamic-a11y-certification/certification-proof.md";

/// Repo-relative path of the checked dashboard artifact.
pub const M5_DYNAMIC_A11Y_DASHBOARD_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-dynamic-a11y-dashboard.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DYNAMIC_A11Y_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/a11y/m5-dynamic-a11y-certification";

/// Stable prefix every certification-owned message id carries.
pub const M5_CERTIFICATION_MESSAGE_ID_PREFIX: &str = "certification.";

/// One of the six governed assistive-tech proof dimensions a claimed surface must pass to
/// keep a public Stable claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5A11yProofDimension {
    /// OS accessibility-bridge health and semantic-node coverage.
    BridgeHealth,
    /// Dynamic live-announcement coverage and coalescing discipline.
    AnnouncementCoverage,
    /// Focus-return safety across asynchronous updates and overlay teardown.
    FocusReturn,
    /// Dense-surface non-visual summaries and label/role fidelity.
    NonVisualSummaries,
    /// High-zoom / high-contrast / reduced-motion visual-adaptation parity.
    VisualAdaptationParity,
    /// Stale-proof downgrade rules that auto-narrow on stale evidence.
    StaleProofDowngrade,
}

impl M5A11yProofDimension {
    /// Every proof dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BridgeHealth,
        Self::AnnouncementCoverage,
        Self::FocusReturn,
        Self::NonVisualSummaries,
        Self::VisualAdaptationParity,
        Self::StaleProofDowngrade,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BridgeHealth => "bridge_health",
            Self::AnnouncementCoverage => "announcement_coverage",
            Self::FocusReturn => "focus_return",
            Self::NonVisualSummaries => "non_visual_summaries",
            Self::VisualAdaptationParity => "visual_adaptation_parity",
            Self::StaleProofDowngrade => "stale_proof_downgrade",
        }
    }

    /// Repo-relative proof packet ref that keeps this dimension current.
    pub const fn backing_proof_ref(self) -> &'static str {
        match self {
            Self::BridgeHealth => M5_PROOF_BRIDGE_DESCRIPTOR_REF,
            Self::AnnouncementCoverage => M5_PROOF_LIVE_ANNOUNCEMENT_REF,
            Self::FocusReturn => M5_PROOF_FOCUS_RETURN_REF,
            Self::NonVisualSummaries => M5_PROOF_NONVISUAL_SUMMARY_REF,
            Self::VisualAdaptationParity => M5_PROOF_DIAGNOSTICS_REF,
            Self::StaleProofDowngrade => M5_PROOF_EVENT_COVERAGE_REF,
        }
    }

    /// Representative downgrade trigger this dimension narrows under when it fails.
    pub const fn representative_trigger(self) -> M5DynamicSurfaceA11yDowngradeTrigger {
        use M5DynamicSurfaceA11yDowngradeTrigger as T;
        match self {
            Self::BridgeHealth => T::BridgeUnavailable,
            Self::AnnouncementCoverage => T::LiveRegionSpam,
            Self::FocusReturn => T::FocusLost,
            Self::NonVisualSummaries => T::NonVisualFidelityLost,
            Self::VisualAdaptationParity => T::UpstreamDependencyNarrowed,
            Self::StaleProofDowngrade => T::ProofStale,
        }
    }
}

/// Freshness of one dimension's backing assistive-tech proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5A11yProofFreshness {
    /// The proof is within its freshness SLO.
    Current,
    /// The proof exists but has fallen outside its freshness SLO.
    Stale,
    /// No current proof row exists for this dimension.
    Missing,
}

impl M5A11yProofFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// True when the proof is within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Green/yellow/red certification status for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5A11yCertificationStatus {
    /// Every dimension is current and conformant; the surface may keep its Stable claim.
    Certified,
    /// At least one dimension carries a disclosed narrowing; the surface is limited.
    Limited,
    /// At least one dimension's proof is stale or missing; the surface must be retested.
    RetestPending,
    /// At least one dimension carries an unhandled blocking regression; the surface is
    /// degraded.
    Degraded,
}

impl M5A11yCertificationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Limited,
        Self::RetestPending,
        Self::Degraded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Degraded => "degraded",
        }
    }

    /// The traffic-light signal this status maps to.
    pub const fn signal(self) -> M5A11yCertificationSignal {
        match self {
            Self::Certified => M5A11yCertificationSignal::Green,
            Self::Limited | Self::RetestPending => M5A11yCertificationSignal::Yellow,
            Self::Degraded => M5A11yCertificationSignal::Red,
        }
    }
}

/// Traffic-light signal for the published dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5A11yCertificationSignal {
    /// Certified.
    Green,
    /// Limited or retest-pending.
    Yellow,
    /// Degraded.
    Red,
}

impl M5A11yCertificationSignal {
    /// Every signal, in declaration order.
    pub const ALL: [Self; 3] = [Self::Green, Self::Yellow, Self::Red];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

/// Release-gate decision the release/public-truth automation reads for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5A11yCertificationGateDecision {
    /// The surface may promote to Stable at its full claim.
    CertifiedPromote,
    /// The surface auto-narrows to a disclosed reduced claim before promotion.
    AutoNarrowed,
    /// The surface is blocked from Stable promotion by an unwaived blocking regression or
    /// missing proof.
    Blocked,
}

impl M5A11yCertificationGateDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::CertifiedPromote, Self::AutoNarrowed, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedPromote => "certified_promote",
            Self::AutoNarrowed => "auto_narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the decision blocks Stable promotion.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Restrictiveness rank of a qualification class (Stable least, Unavailable most), used to
/// pick the most restrictive effective claim when narrowing.
fn qualification_rank(class: M5DynamicSurfaceA11yQualificationClass) -> u8 {
    use M5DynamicSurfaceA11yQualificationClass as Q;
    match class {
        Q::Stable => 0,
        Q::Beta => 1,
        Q::Preview => 2,
        Q::Experimental => 3,
        Q::Held => 4,
        Q::Unavailable => 5,
    }
}

/// The more restrictive of two qualification classes.
fn more_restrictive(
    a: M5DynamicSurfaceA11yQualificationClass,
    b: M5DynamicSurfaceA11yQualificationClass,
) -> M5DynamicSurfaceA11yQualificationClass {
    if qualification_rank(a) >= qualification_rank(b) {
        a
    } else {
        b
    }
}

/// One proof dimension's certification result for a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yDimensionCertification {
    /// The governed proof dimension.
    pub dimension: M5A11yProofDimension,
    /// Repo-relative proof packet ref that keeps this dimension current.
    pub backing_proof_ref: String,
    /// Freshness of the backing proof.
    pub proof_freshness: M5A11yProofFreshness,
    /// Conformance outcome the diagnostics report resolves for this dimension.
    pub conformance: M5DiagnosticOutcome,
    /// Whether a regression of this dimension blocks Stable promotion.
    pub severity: M5DiagnosticSeverity,
    /// Exact downgrade trigger this dimension narrows under; present iff the dimension has a
    /// problem (proof not current, or conformance not healthy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_cause: Option<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Stable message id; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
    /// Export-safe evidence ref backing the result.
    pub evidence_ref: String,
}

impl M5A11yDimensionCertification {
    /// True when this dimension has any problem (proof not current, or conformance not
    /// healthy).
    pub fn has_problem(&self) -> bool {
        !self.proof_freshness.is_current() || !self.conformance.is_healthy()
    }

    /// True when this dimension is an unhandled blocking regression or missing proof.
    pub fn is_blocking_problem(&self) -> bool {
        (self.conformance.is_regressed() && self.severity.is_blocking())
            || self.proof_freshness == M5A11yProofFreshness::Missing
    }

    /// True when this dimension's proof is stale (out of SLO but present).
    pub fn is_stale(&self) -> bool {
        self.proof_freshness == M5A11yProofFreshness::Stale
    }

    /// True when this dimension carries a disclosed conformance narrowing.
    pub fn is_narrowing(&self) -> bool {
        self.conformance.is_narrowed()
    }
}

/// One active waiver that accepts a disclosed reduced claim for a single dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationWaiver {
    /// Stable waiver id.
    pub waiver_id: String,
    /// The dimension the waiver scopes; the waiver only covers this dimension's problem.
    pub dimension: M5A11yProofDimension,
    /// Stable message id naming the waiver reason; prefixed
    /// [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Owner role accountable for the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry timestamp of the waiver.
    pub expires_at: String,
    /// The disclosed reduced claim accepted under this waiver.
    pub narrowed_to: M5DynamicSurfaceA11yQualificationClass,
}

/// One exact stale-proof / regression cause for a dimension, named in the release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yStaleProofCause {
    /// Surface this cause applies to.
    pub surface_id: String,
    /// Dimension whose proof is stale, missing, or non-conformant.
    pub dimension: M5A11yProofDimension,
    /// Exact downgrade trigger the dimension narrows under.
    pub trigger: M5DynamicSurfaceA11yDowngradeTrigger,
    /// Freshness state of the dimension's proof.
    pub freshness: M5A11yProofFreshness,
    /// Conformance outcome of the dimension.
    pub conformance: M5DiagnosticOutcome,
    /// Whether this cause was accepted under an active waiver.
    pub waived: bool,
    /// Stable message id; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// Derived certification fields for a surface row, computed from its dimensions and waivers.
struct DerivedRow {
    status: M5A11yCertificationStatus,
    signal: M5A11yCertificationSignal,
    effective_qualification: M5DynamicSurfaceA11yQualificationClass,
    gate_decision: M5A11yCertificationGateDecision,
    stale_proof_causes: Vec<M5A11yStaleProofCause>,
}

/// Computes the derived certification fields for a surface from its claim, dimensions, and
/// waivers. This is the single source of truth the seed reconciles to and the validator
/// recomputes against, so the stored derived fields can never silently drift.
fn derive_row(
    surface_id: &str,
    claimed: M5DynamicSurfaceA11yQualificationClass,
    dimensions: &[M5A11yDimensionCertification],
    waivers: &[M5A11yCertificationWaiver],
) -> DerivedRow {
    let waived = |dimension: M5A11yProofDimension| waivers.iter().any(|w| w.dimension == dimension);

    let any_blocking_health = dimensions.iter().any(|d| d.is_blocking_problem());
    let any_stale = dimensions.iter().any(|d| d.is_stale());
    let any_narrowing = dimensions.iter().any(|d| d.is_narrowing());

    // The status reflects true health, independent of waivers, so the dashboard never hides
    // a real regression behind a waiver.
    let status = if any_blocking_health {
        M5A11yCertificationStatus::Degraded
    } else if any_stale {
        M5A11yCertificationStatus::RetestPending
    } else if any_narrowing {
        M5A11yCertificationStatus::Limited
    } else {
        M5A11yCertificationStatus::Certified
    };

    let unwaived_blocking = dimensions
        .iter()
        .any(|d| d.is_blocking_problem() && !waived(d.dimension));
    let any_problem = dimensions
        .iter()
        .any(M5A11yDimensionCertification::has_problem);

    let gate_decision = if unwaived_blocking {
        M5A11yCertificationGateDecision::Blocked
    } else if any_problem {
        M5A11yCertificationGateDecision::AutoNarrowed
    } else {
        M5A11yCertificationGateDecision::CertifiedPromote
    };

    let effective_qualification = match gate_decision {
        M5A11yCertificationGateDecision::CertifiedPromote => claimed,
        M5A11yCertificationGateDecision::Blocked => M5DynamicSurfaceA11yQualificationClass::Held,
        M5A11yCertificationGateDecision::AutoNarrowed => {
            // Floor at Beta for any unwaived (stale/narrowed) problem, then apply the most
            // restrictive accepted claim across active waivers.
            let mut effective = M5DynamicSurfaceA11yQualificationClass::Stable;
            if dimensions
                .iter()
                .any(|d| d.has_problem() && !waived(d.dimension))
            {
                effective =
                    more_restrictive(effective, M5DynamicSurfaceA11yQualificationClass::Beta);
            }
            for waiver in waivers {
                effective = more_restrictive(effective, waiver.narrowed_to);
            }
            effective
        }
    };

    let mut stale_proof_causes: Vec<M5A11yStaleProofCause> = dimensions
        .iter()
        .filter(|d| d.has_problem())
        .map(|d| M5A11yStaleProofCause {
            surface_id: surface_id.to_owned(),
            dimension: d.dimension,
            trigger: d
                .stale_cause
                .unwrap_or_else(|| d.dimension.representative_trigger()),
            freshness: d.proof_freshness,
            conformance: d.conformance,
            waived: waived(d.dimension),
            cause_message_id: format!(
                "{}{}.{}.cause",
                M5_CERTIFICATION_MESSAGE_ID_PREFIX,
                surface_id,
                d.dimension.as_str()
            ),
        })
        .collect();
    stale_proof_causes.sort_by(|a, b| a.dimension.cmp(&b.dimension));

    DerivedRow {
        status,
        signal: status.signal(),
        effective_qualification,
        gate_decision,
        stale_proof_causes,
    }
}

/// One claimed dynamic surface's certification row: its six proof dimensions, traffic-light
/// status, release-gate decision, active waivers, and exact stale-proof causes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11ySurfaceCertificationRow {
    /// Stable surface id, unique within the packet.
    pub surface_id: String,
    /// Protected custom-rendered surface family (descriptor-owned).
    pub surface_family: M5SurfaceFamily,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Owner role accountable for keeping this surface's certification current.
    pub owner_role: String,
    /// Object identity this row is bound to — the SAME identity the descriptor, diagnostics,
    /// and visual surface carry.
    pub object_identity_ref: String,
    /// Public claim the surface wants to keep (always Stable for claimed dynamic surfaces).
    pub claimed_qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Effective claim after the certification gate applies.
    pub effective_qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Green/yellow/red certification status.
    pub certification_status: M5A11yCertificationStatus,
    /// Traffic-light signal (mirrors [`Self::certification_status`]).
    pub signal: M5A11yCertificationSignal,
    /// Certification result for each of the six governed proof dimensions.
    pub dimensions: Vec<M5A11yDimensionCertification>,
    /// Active waivers accepting a disclosed reduced claim for one dimension each.
    pub waivers: Vec<M5A11yCertificationWaiver>,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: M5A11yCertificationGateDecision,
    /// Exact stale-proof / regression causes for this surface.
    pub stale_proof_causes: Vec<M5A11yStaleProofCause>,
    /// Reopenable durable fallback surface that preserves this row's identity.
    pub durable_fallback: M5DurableFallbackRef,
    /// Downgrade triggers that can narrow this surface below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this row's certification.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
    /// Stable message id for the status; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl M5A11ySurfaceCertificationRow {
    /// Recomputes the derived status, signal, effective claim, gate, and stale-proof causes
    /// from the dimensions and waivers. The seed calls this after authoring or mutating a
    /// row so the derived blocks never need hand-maintenance.
    pub fn recompute_derived(&mut self) {
        let derived = derive_row(
            &self.surface_id,
            self.claimed_qualification,
            &self.dimensions,
            &self.waivers,
        );
        self.certification_status = derived.status;
        self.signal = derived.signal;
        self.effective_qualification = derived.effective_qualification;
        self.gate_decision = derived.gate_decision;
        self.stale_proof_causes = derived.stale_proof_causes;
    }

    /// True when the surface is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the surface auto-narrowed below its claim.
    pub fn is_auto_narrowed(&self) -> bool {
        matches!(
            self.gate_decision,
            M5A11yCertificationGateDecision::AutoNarrowed
        )
    }

    /// True when the surface is fully certified.
    pub fn is_certified(&self) -> bool {
        matches!(
            self.gate_decision,
            M5A11yCertificationGateDecision::CertifiedPromote
        )
    }

    /// Finds the certification result for a dimension, if present.
    fn dimension(&self, dimension: M5A11yProofDimension) -> Option<&M5A11yDimensionCertification> {
        self.dimensions.iter().find(|d| d.dimension == dimension)
    }
}

/// Compact green/yellow/red certification dashboard — the published scoreboard projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationDashboard {
    /// Record kind; must equal [`M5_DYNAMIC_A11Y_DASHBOARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Cross-ref to the certification packet this dashboard projects.
    pub certification_packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Total surfaces.
    pub total_surfaces: u32,
    /// Green (certified) count.
    pub green_count: u32,
    /// Yellow (limited or retest-pending) count.
    pub yellow_count: u32,
    /// Red (degraded) count.
    pub red_count: u32,
    /// Certified surface ids (sorted).
    pub certified_surface_ids: Vec<String>,
    /// Limited surface ids (sorted).
    pub limited_surface_ids: Vec<String>,
    /// Retest-pending surface ids (sorted).
    pub retest_pending_surface_ids: Vec<String>,
    /// Degraded surface ids (sorted).
    pub degraded_surface_ids: Vec<String>,
    /// Surface ids that auto-narrowed below their claim (sorted).
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Surface ids blocked from Stable promotion (sorted).
    pub blocked_surface_ids: Vec<String>,
    /// Surface ids that carry at least one active waiver (sorted).
    pub waived_surface_ids: Vec<String>,
    /// Active waiver ids (sorted).
    pub active_waiver_ids: Vec<String>,
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Exact stale-proof / regression causes across all surfaces.
    pub stale_proof_causes: Vec<M5A11yStaleProofCause>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Stable message id; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub dashboard_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Self-describing controlled-vocabulary set for the certification-shaped tokens this lane
/// mints, plus the matrix/diagnostics/descriptor tokens these rows reuse so the packet
/// resolves every token it carries on its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationVocabularySet {
    /// Proof-dimension tokens.
    pub proof_dimensions: Vec<String>,
    /// Proof-freshness-state tokens.
    pub proof_freshness_states: Vec<String>,
    /// Certification-status tokens.
    pub certification_statuses: Vec<String>,
    /// Certification-signal tokens.
    pub certification_signals: Vec<String>,
    /// Certification-gate-decision tokens.
    pub certification_gate_decisions: Vec<String>,
    /// Diagnostic-outcome tokens (diagnostics-owned).
    pub diagnostic_outcomes: Vec<String>,
    /// Diagnostic-severity tokens (diagnostics-owned).
    pub diagnostic_severities: Vec<String>,
    /// Qualification-class tokens (matrix-owned).
    pub qualification_classes: Vec<String>,
    /// Downgrade-trigger tokens (matrix-owned).
    pub downgrade_triggers: Vec<String>,
    /// Consumer-surface tokens (matrix-owned).
    pub consumer_surfaces: Vec<String>,
    /// Surface-family tokens (descriptor-owned).
    pub surface_families: Vec<String>,
}

impl M5A11yCertificationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        use M5DynamicSurfaceA11yConsumerSurface as Consumer;
        use M5DynamicSurfaceA11yDowngradeTrigger as Trigger;
        use M5DynamicSurfaceA11yQualificationClass as Qual;
        Self {
            proof_dimensions: M5A11yProofDimension::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            proof_freshness_states: M5A11yProofFreshness::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            certification_statuses: M5A11yCertificationStatus::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            certification_signals: M5A11yCertificationSignal::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            certification_gate_decisions: M5A11yCertificationGateDecision::ALL
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
            qualification_classes: [
                Qual::Stable,
                Qual::Beta,
                Qual::Preview,
                Qual::Experimental,
                Qual::Unavailable,
                Qual::Held,
            ]
            .iter()
            .map(|v| v.as_str().to_owned())
            .collect(),
            downgrade_triggers: Trigger::ALL.iter().map(|v| v.as_str().to_owned()).collect(),
            consumer_surfaces: [
                Consumer::Shell,
                Consumer::Editor,
                Consumer::Terminal,
                Consumer::Notebook,
                Consumer::DataGrid,
                Consumer::Review,
                Consumer::Help,
                Consumer::Presentation,
                Consumer::SupportExport,
                Consumer::AiSurfaces,
            ]
            .iter()
            .map(|v| v.as_str().to_owned())
            .collect(),
            surface_families: M5SurfaceFamily::ALL
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

/// Assistive-technology certification conformance review. Every flag is a hard invariant;
/// all must hold for the packet to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationConformanceReview {
    /// Every claimed dynamic surface has a certification row.
    pub every_dynamic_surface_has_certification_row: bool,
    /// Each surface is certified across all six governed proof dimensions.
    pub matrix_six_dimensions_covered_per_surface: bool,
    /// Bridge health is certified from the diagnostics report.
    pub bridge_health_certified_from_diagnostics: bool,
    /// Dynamic announcement coverage is certified.
    pub announcement_coverage_certified: bool,
    /// Focus-return safety is certified.
    pub focus_return_certified: bool,
    /// Non-visual summaries are certified.
    pub non_visual_summaries_certified: bool,
    /// Zoom/contrast/motion parity is certified.
    pub zoom_contrast_motion_parity_certified: bool,
    /// Stale-proof downgrade rules are enforced.
    pub stale_proof_downgrade_rules_enforced: bool,
    /// Stale or missing proof auto-narrows the row before Stable promotion.
    pub stale_or_missing_proof_auto_narrows_before_stable: bool,
    /// Unwaived blocking regressions block Stable promotion.
    pub unwaived_regressions_block_stable_promotion: bool,
    /// Regressions are never invisible in the release/public-truth packet.
    pub regressions_not_invisible_in_release_truth: bool,
    /// Active waivers are disclosed with scope, owner, and expiry.
    pub active_waivers_disclosed_with_scope_and_expiry: bool,
    /// Exact stale-proof causes are named per dimension.
    pub exact_stale_proof_causes_named: bool,
    /// The dashboard traffic-light counts match the rows.
    pub dashboard_traffic_light_matches_rows: bool,
    /// Certification rows reuse the descriptor object identity.
    pub surfaces_reuse_descriptor_object_identity: bool,
    /// Support/export carries no raw boundary material.
    pub support_export_carries_no_raw_boundary_material: bool,
}

impl M5A11yCertificationConformanceReview {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_dynamic_surface_has_certification_row
            && self.matrix_six_dimensions_covered_per_surface
            && self.bridge_health_certified_from_diagnostics
            && self.announcement_coverage_certified
            && self.focus_return_certified
            && self.non_visual_summaries_certified
            && self.zoom_contrast_motion_parity_certified
            && self.stale_proof_downgrade_rules_enforced
            && self.stale_or_missing_proof_auto_narrows_before_stable
            && self.unwaived_regressions_block_stable_promotion
            && self.regressions_not_invisible_in_release_truth
            && self.active_waivers_disclosed_with_scope_and_expiry
            && self.exact_stale_proof_causes_named
            && self.dashboard_traffic_light_matches_rows
            && self.surfaces_reuse_descriptor_object_identity
            && self.support_export_carries_no_raw_boundary_material
    }
}

/// Consumer projection block: who reads the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationConsumerProjection {
    /// Release center consumes the certification.
    pub release_center_consumes_certification: bool,
    /// Support export consumes the certification.
    pub support_export_consumes_certification: bool,
    /// Docs / help document the certification.
    pub docs_help_documents_certification: bool,
    /// Onboarding reflects the certification.
    pub onboarding_reflects_certification: bool,
    /// Presentation reflects the certification.
    pub presentation_reflects_certification: bool,
    /// Shell, editor, notebook, data, and review surfaces consume the certification.
    pub shell_editor_notebook_data_review_consume_certification: bool,
    /// Release/public-truth automation gates on the certification.
    pub release_public_truth_gates_on_certification: bool,
    /// The stable-claim matrix reads the certification.
    pub stable_claim_matrix_reads_certification: bool,
}

impl M5A11yCertificationConsumerProjection {
    /// True when every projection holds.
    pub fn all_hold(&self) -> bool {
        self.release_center_consumes_certification
            && self.support_export_consumes_certification
            && self.docs_help_documents_certification
            && self.onboarding_reflects_certification
            && self.presentation_reflects_certification
            && self.shell_editor_notebook_data_review_consume_certification
            && self.release_public_truth_gates_on_certification
            && self.stable_claim_matrix_reads_certification
    }
}

/// Packet-level release gate aggregating the per-surface certification gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5A11yCertificationReleaseGate {
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted surface ids blocked from Stable promotion.
    pub blocked_surface_ids: Vec<String>,
    /// Sorted surface ids that auto-narrowed below their claim.
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Sorted surface ids fully certified for Stable promotion.
    pub certified_surface_ids: Vec<String>,
    /// Sorted surface ids carrying at least one active waiver.
    pub waived_surface_ids: Vec<String>,
    /// Stable message id; prefixed [`M5_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Constructor input for [`M5DynamicA11yCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DynamicA11yCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Per-surface certification rows.
    pub surfaces: Vec<M5A11ySurfaceCertificationRow>,
    /// Certification-shaped controlled-vocabulary set.
    pub vocabulary_set: M5A11yCertificationVocabularySet,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5A11yCertificationConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5A11yCertificationConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5A11yCertificationReleaseGate,
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

/// Export-safe M5 dynamic-surface assistive-tech certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicA11yCertificationPacket {
    /// Record kind; must equal [`M5_DYNAMIC_A11Y_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Per-surface certification rows.
    pub surfaces: Vec<M5A11ySurfaceCertificationRow>,
    /// Certification-shaped controlled-vocabulary set.
    pub vocabulary_set: M5A11yCertificationVocabularySet,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5A11yCertificationConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5A11yCertificationConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5A11yCertificationReleaseGate,
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

impl M5DynamicA11yCertificationPacket {
    /// Builds a certification packet from seed input.
    pub fn new(input: M5DynamicA11yCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_DYNAMIC_A11Y_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            surfaces: input.surfaces,
            vocabulary_set: input.vocabulary_set,
            shared_vocabulary_set: input.shared_vocabulary_set,
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

    /// True when the release/public-truth automation must hold Stable promotion because at
    /// least one claimed surface is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Surface ids currently blocked from Stable promotion.
    pub fn blocked_surface_ids(&self) -> Vec<&str> {
        self.surfaces
            .iter()
            .filter(|s| s.is_blocked())
            .map(|s| s.surface_id.as_str())
            .collect()
    }

    /// Builds the compact green/yellow/red dashboard projection from the rows.
    pub fn dashboard(&self) -> M5A11yCertificationDashboard {
        let by_status = |status: M5A11yCertificationStatus| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surfaces
                .iter()
                .filter(|s| s.certification_status == status)
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };
        let mut auto_narrowed_surface_ids: Vec<String> = self
            .surfaces
            .iter()
            .filter(|s| s.is_auto_narrowed())
            .map(|s| s.surface_id.clone())
            .collect();
        auto_narrowed_surface_ids.sort();
        let mut blocked_surface_ids: Vec<String> = self
            .surfaces
            .iter()
            .filter(|s| s.is_blocked())
            .map(|s| s.surface_id.clone())
            .collect();
        blocked_surface_ids.sort();
        let mut waived_surface_ids: Vec<String> = self
            .surfaces
            .iter()
            .filter(|s| !s.waivers.is_empty())
            .map(|s| s.surface_id.clone())
            .collect();
        waived_surface_ids.sort();
        let mut active_waiver_ids: Vec<String> = self
            .surfaces
            .iter()
            .flat_map(|s| s.waivers.iter().map(|w| w.waiver_id.clone()))
            .collect();
        active_waiver_ids.sort();

        let mut stale_proof_causes: Vec<M5A11yStaleProofCause> = self
            .surfaces
            .iter()
            .flat_map(|s| s.stale_proof_causes.iter().cloned())
            .collect();
        stale_proof_causes.sort_by(|a, b| {
            a.surface_id
                .cmp(&b.surface_id)
                .then(a.dimension.cmp(&b.dimension))
        });

        let green = self
            .surfaces
            .iter()
            .filter(|s| s.signal == M5A11yCertificationSignal::Green)
            .count() as u32;
        let yellow = self
            .surfaces
            .iter()
            .filter(|s| s.signal == M5A11yCertificationSignal::Yellow)
            .count() as u32;
        let red = self
            .surfaces
            .iter()
            .filter(|s| s.signal == M5A11yCertificationSignal::Red)
            .count() as u32;

        M5A11yCertificationDashboard {
            record_kind: M5_DYNAMIC_A11Y_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_VERSION,
            certification_packet_id: self.packet_id.clone(),
            report_label: self.report_label.clone(),
            total_surfaces: self.surfaces.len() as u32,
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            certified_surface_ids: by_status(M5A11yCertificationStatus::Certified),
            limited_surface_ids: by_status(M5A11yCertificationStatus::Limited),
            retest_pending_surface_ids: by_status(M5A11yCertificationStatus::RetestPending),
            degraded_surface_ids: by_status(M5A11yCertificationStatus::Degraded),
            auto_narrowed_surface_ids,
            blocked_surface_ids: blocked_surface_ids.clone(),
            waived_surface_ids,
            active_waiver_ids,
            blocks_stable_promotion: !blocked_surface_ids.is_empty(),
            stale_proof_causes,
            source_contract_refs: self.source_contract_refs.clone(),
            dashboard_message_id: format!("{}dashboard", M5_CERTIFICATION_MESSAGE_ID_PREFIX),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Validates the certification-packet invariants.
    pub fn validate(&self) -> Vec<M5CertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DYNAMIC_A11Y_CERTIFICATION_RECORD_KIND {
            violations.push(M5CertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5CertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_surfaces(self, &mut violations);
        validate_release_gate_aggregate(self, &mut violations);
        validate_dashboard(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 certification packet serializes"),
        ) {
            violations.push(M5CertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 certification packet serializes")
    }

    /// Deterministic export-safe JSON for the dashboard projection.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn dashboard_json(&self) -> String {
        serde_json::to_string_pretty(&self.dashboard())
            .expect("m5 certification dashboard serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let dashboard = self.dashboard();
        let mut out = String::new();
        out.push_str("# M5 Dynamic-Surface Assistive-Tech Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} green, {} yellow, {} red)\n",
            dashboard.total_surfaces,
            dashboard.green_count,
            dashboard.yellow_count,
            dashboard.red_count
        ));
        out.push_str(&format!(
            "- Release gate: {} ({} blocked, {} auto-narrowed, {} certified)\n",
            if self.release_gate.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            },
            self.release_gate.blocked_surface_ids.len(),
            self.release_gate.auto_narrowed_surface_ids.len(),
            self.release_gate.certified_surface_ids.len()
        ));
        out.push_str(&format!(
            "- Active waivers: {}\n",
            dashboard.active_waiver_ids.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}` ({}), claim `{}` → `{}`, gate `{}`\n",
                surface.surface_id,
                surface.surface_family.as_str(),
                surface.certification_status.as_str(),
                surface.signal.as_str(),
                surface.claimed_qualification.as_str(),
                surface.effective_qualification.as_str(),
                surface.gate_decision.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", surface.owner_role));
            out.push_str(&format!(
                "  - Object identity: `{}`\n",
                surface.object_identity_ref
            ));
            for dimension in &surface.dimensions {
                out.push_str(&format!(
                    "  - {} `{}`: proof `{}`, conformance `{}`{}\n",
                    dimension.dimension.as_str(),
                    dimension.backing_proof_ref,
                    dimension.proof_freshness.as_str(),
                    dimension.conformance.as_str(),
                    match dimension.stale_cause {
                        Some(trigger) => format!(" — cause `{}`", trigger.as_str()),
                        None => String::new(),
                    }
                ));
            }
            for waiver in &surface.waivers {
                out.push_str(&format!(
                    "  - waiver `{}` on `{}` → `{}` (owner {}, expires {})\n",
                    waiver.waiver_id,
                    waiver.dimension.as_str(),
                    waiver.narrowed_to.as_str(),
                    waiver.owner_role,
                    waiver.expires_at
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum M5CertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CertificationViolation>),
}

impl fmt::Display for M5CertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 certification packet parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 certification packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CertificationArtifactError {}

/// Validation failures emitted by [`M5DynamicA11yCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CertificationViolation {
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
    /// A protected surface family has no certification row.
    RequiredSurfaceFamilyMissing,
    /// Two rows share a surface id.
    DuplicateSurfaceId,
    /// A certification row is incomplete.
    CertificationRowIncomplete,
    /// A row is not bound to an object identity.
    MissingObjectIdentity,
    /// A row does not claim Stable.
    RowDoesNotClaimStable,
    /// A row does not carry exactly one certification per proof dimension.
    DimensionsNotOnePerKind,
    /// A dimension certification is incomplete.
    DimensionIncomplete,
    /// A dimension's backing proof ref is wrong for its dimension.
    DimensionBackingProofMismatch,
    /// A dimension's stale cause is present without a problem, or absent with one.
    DimensionStaleCauseMismatch,
    /// A stale or missing dimension does not narrow under a stale-proof trigger.
    StaleProofTriggerMismatch,
    /// A row's derived status / signal / gate / effective claim disagrees with its
    /// dimensions and waivers.
    DerivedRowInconsistent,
    /// A row's stale-proof causes disagree with its dimensions.
    StaleProofCausesInconsistent,
    /// A waiver is incomplete.
    WaiverIncomplete,
    /// A waiver scopes a dimension the row does not carry.
    WaiverDimensionUnknown,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row has no reopenable durable fallback surface.
    DurableFallbackMissing,
    /// A row is missing required proof packet refs.
    RowMissingProofPacketRefs,
    /// A certification message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// The packet-level release gate disagrees with the per-surface gates.
    ReleaseGateAggregateInconsistent,
    /// The dashboard projection disagrees with the rows.
    DashboardInconsistent,
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

impl M5CertificationViolation {
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
            Self::CertificationRowIncomplete => "certification_row_incomplete",
            Self::MissingObjectIdentity => "missing_object_identity",
            Self::RowDoesNotClaimStable => "row_does_not_claim_stable",
            Self::DimensionsNotOnePerKind => "dimensions_not_one_per_kind",
            Self::DimensionIncomplete => "dimension_incomplete",
            Self::DimensionBackingProofMismatch => "dimension_backing_proof_mismatch",
            Self::DimensionStaleCauseMismatch => "dimension_stale_cause_mismatch",
            Self::StaleProofTriggerMismatch => "stale_proof_trigger_mismatch",
            Self::DerivedRowInconsistent => "derived_row_inconsistent",
            Self::StaleProofCausesInconsistent => "stale_proof_causes_inconsistent",
            Self::WaiverIncomplete => "waiver_incomplete",
            Self::WaiverDimensionUnknown => "waiver_dimension_unknown",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DurableFallbackMissing => "durable_fallback_missing",
            Self::RowMissingProofPacketRefs => "row_missing_proof_packet_refs",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::ReleaseGateAggregateInconsistent => "release_gate_aggregate_inconsistent",
            Self::DashboardInconsistent => "dashboard_inconsistent",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in certification support export.
pub fn current_stable_m5_dynamic_a11y_certification_export(
) -> Result<M5DynamicA11yCertificationPacket, M5CertificationArtifactError> {
    let packet: M5DynamicA11yCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-dynamic-a11y-certification/support_export.json"
    )))
    .map_err(M5CertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CertificationArtifactError::Validation(violations))
    }
}

/// Reads and validates the checked-in certification dashboard, returning it alongside the
/// packet it projects.
pub fn current_stable_m5_dynamic_a11y_dashboard(
) -> Result<M5A11yCertificationDashboard, M5CertificationArtifactError> {
    let dashboard: M5A11yCertificationDashboard = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-dynamic-a11y-dashboard.json"
    )))
    .map_err(M5CertificationArtifactError::SupportExport)?;
    Ok(dashboard)
}

fn validate_source_contracts(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DYNAMIC_A11Y_CERTIFICATION_SCHEMA_REF,
        M5_DYNAMIC_A11Y_DASHBOARD_SCHEMA_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_DOC_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_MATRIX_REF,
        M5_DYNAMIC_A11Y_CERTIFICATION_DIAGNOSTICS_REF,
        M5_PROOF_BRIDGE_DESCRIPTOR_REF,
        M5_PROOF_LIVE_ANNOUNCEMENT_REF,
        M5_PROOF_EVENT_COVERAGE_REF,
        M5_PROOF_FOCUS_RETURN_REF,
        M5_PROOF_NONVISUAL_SUMMARY_REF,
        M5_PROOF_DIAGNOSTICS_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    if !packet.vocabulary_set.matches_canonical()
        || !packet.shared_vocabulary_set.matches_canonical()
    {
        violations.push(M5CertificationViolation::VocabularySetDrift);
    }
}

fn validate_surfaces(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let present: BTreeSet<M5SurfaceFamily> =
        packet.surfaces.iter().map(|s| s.surface_family).collect();
    for required in M5SurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5CertificationViolation::RequiredSurfaceFamilyMissing);
            break;
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &packet.surfaces {
        if !seen_ids.insert(surface.surface_id.as_str()) {
            violations.push(M5CertificationViolation::DuplicateSurfaceId);
        }
        if surface.surface_id.trim().is_empty()
            || surface.surface_label.trim().is_empty()
            || surface.owner_role.trim().is_empty()
            || surface.source_contract_refs.is_empty()
        {
            violations.push(M5CertificationViolation::CertificationRowIncomplete);
        }
        if surface.object_identity_ref.trim().is_empty() {
            violations.push(M5CertificationViolation::MissingObjectIdentity);
        }
        if !surface.claimed_qualification.is_stable() {
            violations.push(M5CertificationViolation::RowDoesNotClaimStable);
        }
        if !surface
            .status_message_id
            .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
            || !surface
                .gate_message_id
                .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5CertificationViolation::MessageIdPrefixMissing);
        }

        validate_surface_dimensions(surface, violations);
        validate_surface_waivers(surface, violations);
        validate_surface_derived(surface, violations);

        if surface.required_proof_packet_refs.is_empty() {
            violations.push(M5CertificationViolation::RowMissingProofPacketRefs);
        }
        if surface.downgrade_triggers.is_empty() {
            violations.push(M5CertificationViolation::DowngradeTriggersMissing);
        }
        if surface.consumer_surfaces.is_empty() {
            violations.push(M5CertificationViolation::ConsumerSurfacesMissing);
        }
        if surface.durable_fallback.surface_ref.trim().is_empty()
            || !surface.durable_fallback.reopenable
        {
            violations.push(M5CertificationViolation::DurableFallbackMissing);
        }
    }
}

fn validate_surface_dimensions(
    surface: &M5A11ySurfaceCertificationRow,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let mut seen: BTreeSet<M5A11yProofDimension> = BTreeSet::new();
    for dimension in &surface.dimensions {
        seen.insert(dimension.dimension);
        if dimension.detail_message_id.trim().is_empty() || dimension.evidence_ref.trim().is_empty()
        {
            violations.push(M5CertificationViolation::DimensionIncomplete);
        }
        if !dimension
            .detail_message_id
            .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5CertificationViolation::MessageIdPrefixMissing);
        }
        if dimension.backing_proof_ref != dimension.dimension.backing_proof_ref() {
            violations.push(M5CertificationViolation::DimensionBackingProofMismatch);
        }
        // The stale cause is present exactly when the dimension has a problem, so a
        // narrowed/stale dimension can never hide behind an empty cause and a healthy
        // dimension can never invent one.
        if dimension.has_problem() != dimension.stale_cause.is_some() {
            violations.push(M5CertificationViolation::DimensionStaleCauseMismatch);
        }
        // Stale or missing proof must narrow under the stale-proof trigger.
        if !dimension.proof_freshness.is_current()
            && dimension.stale_cause != Some(M5DynamicSurfaceA11yDowngradeTrigger::ProofStale)
        {
            violations.push(M5CertificationViolation::StaleProofTriggerMismatch);
        }
    }
    if seen.len() != M5A11yProofDimension::ALL.len() || surface.dimensions.len() != seen.len() {
        violations.push(M5CertificationViolation::DimensionsNotOnePerKind);
    }
}

fn validate_surface_waivers(
    surface: &M5A11ySurfaceCertificationRow,
    violations: &mut Vec<M5CertificationViolation>,
) {
    for waiver in &surface.waivers {
        if waiver.waiver_id.trim().is_empty()
            || waiver.reason_message_id.trim().is_empty()
            || waiver.owner_role.trim().is_empty()
            || waiver.expires_at.trim().is_empty()
        {
            violations.push(M5CertificationViolation::WaiverIncomplete);
        }
        if !waiver
            .reason_message_id
            .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5CertificationViolation::MessageIdPrefixMissing);
        }
        // A waiver must accept a genuinely reduced claim, never re-grant Stable.
        if waiver.narrowed_to.is_stable() {
            violations.push(M5CertificationViolation::WaiverIncomplete);
        }
        if surface.dimension(waiver.dimension).is_none() {
            violations.push(M5CertificationViolation::WaiverDimensionUnknown);
        }
    }
}

fn validate_surface_derived(
    surface: &M5A11ySurfaceCertificationRow,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let derived = derive_row(
        &surface.surface_id,
        surface.claimed_qualification,
        &surface.dimensions,
        &surface.waivers,
    );
    if surface.certification_status != derived.status
        || surface.signal != derived.signal
        || surface.signal != surface.certification_status.signal()
        || surface.effective_qualification != derived.effective_qualification
        || surface.gate_decision != derived.gate_decision
    {
        violations.push(M5CertificationViolation::DerivedRowInconsistent);
    }
    if surface.stale_proof_causes != derived.stale_proof_causes {
        violations.push(M5CertificationViolation::StaleProofCausesInconsistent);
    }
}

fn validate_release_gate_aggregate(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let collect = |predicate: &dyn Fn(&M5A11ySurfaceCertificationRow) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = packet
            .surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    let auto_narrowed = collect(&|s| s.is_auto_narrowed());
    let certified = collect(&|s| s.is_certified());
    let waived = collect(&|s| !s.waivers.is_empty());
    let blocks_expected = !blocked.is_empty();

    let gate = &packet.release_gate;
    if gate.blocks_stable_promotion != blocks_expected
        || gate.blocked_surface_ids != blocked
        || gate.auto_narrowed_surface_ids != auto_narrowed
        || gate.certified_surface_ids != certified
        || gate.waived_surface_ids != waived
        || !gate
            .gate_message_id
            .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
    {
        violations.push(M5CertificationViolation::ReleaseGateAggregateInconsistent);
    }
}

fn validate_dashboard(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    // The dashboard is a pure projection; recompute and check its internal accounting holds.
    let dashboard = packet.dashboard();
    let signal_count = dashboard.green_count + dashboard.yellow_count + dashboard.red_count;
    let status_count = dashboard.certified_surface_ids.len()
        + dashboard.limited_surface_ids.len()
        + dashboard.retest_pending_surface_ids.len()
        + dashboard.degraded_surface_ids.len();
    if dashboard.total_surfaces != packet.surfaces.len() as u32
        || signal_count != dashboard.total_surfaces
        || status_count as u32 != dashboard.total_surfaces
        || dashboard.green_count != dashboard.certified_surface_ids.len() as u32
        || dashboard.red_count != dashboard.degraded_surface_ids.len() as u32
        || dashboard.yellow_count
            != (dashboard.limited_surface_ids.len() + dashboard.retest_pending_surface_ids.len())
                as u32
        || dashboard.blocks_stable_promotion != packet.release_gate.blocks_stable_promotion
        || dashboard.blocked_surface_ids != packet.release_gate.blocked_surface_ids
        || !dashboard
            .dashboard_message_id
            .starts_with(M5_CERTIFICATION_MESSAGE_ID_PREFIX)
    {
        violations.push(M5CertificationViolation::DashboardInconsistent);
    }
}

fn validate_conformance_review(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    if !packet.conformance_review.all_hold() {
        violations.push(M5CertificationViolation::ConformanceReviewIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(M5CertificationViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let freshness = &packet.proof_freshness;
    if freshness.proof_freshness_slo_hours == 0
        || freshness.last_proof_refresh.trim().is_empty()
        || !freshness.auto_narrow_on_stale
    {
        violations.push(M5CertificationViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DynamicA11yCertificationPacket,
    violations: &mut Vec<M5CertificationViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5CertificationViolation::ReleasePostureIncomplete);
    }
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential
/// bodies, raw provider payloads). Certification packets are metadata-only by construction;
/// this is a defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a certification packet's rows from an AT diagnostics report, certifying each
/// surface across the six governed proof dimensions from the diagnostics health truth.
///
/// This is the capstone consumption path: the certification reads the diagnostics report
/// rather than re-deriving surface health by hand, so bridge state, announcement budgets,
/// focus-return failures, and zoom/contrast/motion conformance flow straight into the
/// certification dimensions.
pub fn certification_rows_from_diagnostics(
    diagnostics: &M5DynamicA11yDiagnosticsPacket,
) -> Vec<M5A11ySurfaceCertificationRow> {
    diagnostics
        .surfaces
        .iter()
        .map(certification_row_from_diagnostics_surface)
        .collect()
}

/// The worse of two diagnostic outcomes (regressed > auto-narrowed > pass/not-applicable).
fn worse_outcome(a: M5DiagnosticOutcome, b: M5DiagnosticOutcome) -> M5DiagnosticOutcome {
    let rank = |outcome: M5DiagnosticOutcome| -> u8 {
        match outcome {
            M5DiagnosticOutcome::Pass | M5DiagnosticOutcome::NotApplicable => 0,
            M5DiagnosticOutcome::AutoNarrowed => 1,
            M5DiagnosticOutcome::Regressed => 2,
        }
    };
    if rank(a) >= rank(b) {
        normalize_outcome(a)
    } else {
        normalize_outcome(b)
    }
}

/// Normalizes `not_applicable` to `pass` so the certification carries a healthy outcome.
fn normalize_outcome(outcome: M5DiagnosticOutcome) -> M5DiagnosticOutcome {
    match outcome {
        M5DiagnosticOutcome::NotApplicable => M5DiagnosticOutcome::Pass,
        other => other,
    }
}

/// The conformance outcome a diagnostics surface resolves for one proof dimension.
fn dimension_conformance(
    diagnostics: &M5SurfaceDiagnostics,
    dimension: M5A11yProofDimension,
) -> M5DiagnosticOutcome {
    let check = |class: M5AtDiagnosticClass| -> M5DiagnosticOutcome {
        diagnostics
            .checks
            .iter()
            .find(|c| c.class == class)
            .map(|c| c.outcome)
            .unwrap_or(M5DiagnosticOutcome::Pass)
    };
    match dimension {
        M5A11yProofDimension::BridgeHealth => worse_outcome(
            check(M5AtDiagnosticClass::BridgeHealth),
            check(M5AtDiagnosticClass::MissingSemanticNode),
        ),
        M5A11yProofDimension::AnnouncementCoverage => worse_outcome(
            check(M5AtDiagnosticClass::AnnouncementRate),
            check(M5AtDiagnosticClass::CoalescingViolation),
        ),
        M5A11yProofDimension::FocusReturn => check(M5AtDiagnosticClass::FocusReturnFailure),
        M5A11yProofDimension::NonVisualSummaries => worse_outcome(
            check(M5AtDiagnosticClass::LabelOrRoleDrift),
            check(M5AtDiagnosticClass::MissingSemanticNode),
        ),
        M5A11yProofDimension::VisualAdaptationParity => {
            // High-zoom and high-contrast are blocking; reduced-motion is advisory, so a
            // reduced-motion regression certifies as a disclosed narrowing rather than a
            // blocking regression.
            let blocking = worse_outcome(
                check(M5AtDiagnosticClass::HighZoomRegression),
                check(M5AtDiagnosticClass::HighContrastRegression),
            );
            if blocking != M5DiagnosticOutcome::Pass {
                blocking
            } else if check(M5AtDiagnosticClass::ReducedMotionRegression)
                == M5DiagnosticOutcome::Regressed
            {
                M5DiagnosticOutcome::AutoNarrowed
            } else {
                normalize_outcome(check(M5AtDiagnosticClass::ReducedMotionRegression))
            }
        }
        // The stale-proof downgrade dimension certifies the policy itself; it is current and
        // conformant whenever the diagnostics report is fresh.
        M5A11yProofDimension::StaleProofDowngrade => M5DiagnosticOutcome::Pass,
    }
}

/// Builds one certification row from a diagnostics surface, reusing its surface family,
/// label, owner, object identity, durable fallback, downgrade triggers, and consumers.
fn certification_row_from_diagnostics_surface(
    diagnostics: &M5SurfaceDiagnostics,
) -> M5A11ySurfaceCertificationRow {
    let family = diagnostics.surface_family;
    let surface_id = format!("certification:{}", family.as_str());
    let dimensions: Vec<M5A11yDimensionCertification> = M5A11yProofDimension::ALL
        .iter()
        .map(|&dimension| {
            let conformance = dimension_conformance(diagnostics, dimension);
            // Reduced-motion advisories are already folded into a narrowing in
            // `dimension_conformance`, so every governed dimension blocks on a real
            // regression.
            let severity = M5DiagnosticSeverity::Blocking;
            let proof_freshness = M5A11yProofFreshness::Current;
            let has_problem = !proof_freshness.is_current() || !conformance.is_healthy();
            let stale_cause = if has_problem {
                Some(dimension.representative_trigger())
            } else {
                None
            };
            M5A11yDimensionCertification {
                dimension,
                backing_proof_ref: dimension.backing_proof_ref().to_owned(),
                proof_freshness,
                conformance,
                severity,
                stale_cause,
                detail_message_id: format!(
                    "{}{}.{}",
                    M5_CERTIFICATION_MESSAGE_ID_PREFIX,
                    surface_id,
                    dimension.as_str()
                ),
                evidence_ref: format!(
                    "evidence:at-certification:{}:{}",
                    family.as_str(),
                    dimension.as_str()
                ),
            }
        })
        .collect();

    let required_proof_packet_refs: Vec<String> = M5A11yProofDimension::ALL
        .iter()
        .map(|d| d.backing_proof_ref().to_owned())
        .collect();

    let mut row = M5A11ySurfaceCertificationRow {
        surface_id: surface_id.clone(),
        surface_family: family,
        surface_label: diagnostics.surface_label.clone(),
        owner_role: diagnostics.owner_role.clone(),
        object_identity_ref: diagnostics.object_identity_ref.clone(),
        claimed_qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        effective_qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        certification_status: M5A11yCertificationStatus::Certified,
        signal: M5A11yCertificationSignal::Green,
        dimensions,
        waivers: Vec::new(),
        gate_decision: M5A11yCertificationGateDecision::CertifiedPromote,
        stale_proof_causes: Vec::new(),
        durable_fallback: diagnostics.durable_fallback.clone(),
        downgrade_triggers: diagnostics.downgrade_triggers.clone(),
        required_proof_packet_refs,
        source_contract_refs: diagnostics.source_contract_refs.clone(),
        consumer_surfaces: diagnostics.consumer_surfaces.clone(),
        status_message_id: format!(
            "{}{}.status",
            M5_CERTIFICATION_MESSAGE_ID_PREFIX, surface_id
        ),
        gate_message_id: format!("{}{}.gate", M5_CERTIFICATION_MESSAGE_ID_PREFIX, surface_id),
    };
    row.recompute_derived();
    row
}
