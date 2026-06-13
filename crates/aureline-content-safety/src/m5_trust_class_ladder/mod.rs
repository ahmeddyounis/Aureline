//! Trust-class ladders, active-content downgrade rules, and compare-only
//! fallbacks for the new M5 preview and embedded surfaces.
//!
//! The frozen content-integrity matrix in
//! [`crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`]
//! locks the *static* qualification each M5 artifact/viewer family may claim,
//! and [`crate::m5_raw_rendered_handoff`] keeps raw-versus-rendered copy/export
//! honest. This lane covers the orthogonal *runtime* gap they leave open: given
//! a surface's requested trust class and the live trust signals around it, what
//! posture does the surface actually resolve to before content renders or
//! executes?
//!
//! Every preview/embedded surface climbs an explicit ladder —
//! [`M5TrustClass::RawText`] → [`M5TrustClass::SanitizedRich`] →
//! [`M5TrustClass::TrustedLocalActive`] / [`M5TrustClass::IsolatedRemoteActive`].
//! When a requested rung cannot be trusted (isolation runtime missing, local
//! trust unestablished, suspicious bytes found, raw/rendered divergence
//! unresolved, safe preview unavailable, proof stale, or policy block), the
//! resolution applies a named downgrade rule and degrades the surface to
//! *sanitized* visibility, a *compare-only* fallback, or a *blocked* metadata
//! view — never silent execution and never opaque failure. Raw inspection and
//! raw copy stay reachable on every surface, suspicious bytes are never
//! normalized away, rendered copy never masquerades as raw, and active content
//! never executes outside its declared trust class.
//!
//! The boundary schema is
//! [`schemas/security/m5-trust-class-ladder.schema.json`](../../../../schemas/security/m5-trust-class-ladder.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_trust_class_ladder.md`](../../../../docs/security/m5/m5_trust_class_ladder.md).
//! The protected fixture directory is
//! [`fixtures/security/m5/m5_trust_class_ladder/`](../../../../fixtures/security/m5/m5_trust_class_ladder/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TrustClassLadderPacket`].
pub const M5_TRUST_CLASS_LADDER_RECORD_KIND: &str = "m5_trust_class_ladder_packet";

/// Integer schema version for the M5 trust-class ladder packet.
pub const M5_TRUST_CLASS_LADDER_SCHEMA_VERSION: u32 = 1;

/// Stable packet id minted by [`frozen_m5_trust_class_ladder_packet`].
pub const M5_TRUST_CLASS_LADDER_PACKET_ID: &str = "m5-trust-class-ladder:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_TRUST_CLASS_LADDER_SCHEMA_REF: &str =
    "schemas/security/m5-trust-class-ladder.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TRUST_CLASS_LADDER_DOC_REF: &str = "docs/security/m5/m5_trust_class_ladder.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TRUST_CLASS_LADDER_FIXTURE_DIR: &str = "fixtures/security/m5/m5_trust_class_ladder";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TRUST_CLASS_LADDER_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_trust_class_ladder/support_export.json";

/// Repo-relative path of the frozen trust-class vocabulary contract.
pub const M5_TRUST_CLASS_LADDER_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the frozen safe-preview trust-class contract.
pub const M5_TRUST_CLASS_LADDER_SAFE_PREVIEW_CONTRACT_REF: &str =
    "schemas/trust/safe-preview-trust-class.schema.json";

/// A preview or embedded surface whose trust class is resolved at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustLadderSurface {
    /// Notebook rich-output blocks.
    NotebookRichOutput,
    /// Docs and in-product browser panels.
    DocsBrowserPanel,
    /// AI evidence and finding-card viewers.
    AiEvidenceViewer,
    /// Pipeline run and artifact browsers.
    PipelineArtifactBrowser,
    /// Provider account/policy overlays.
    ProviderOverlay,
    /// Marketplace install and update review surfaces.
    MarketplaceInstallReview,
    /// Remote preview targets.
    RemotePreviewTarget,
    /// Structured compare and diff views.
    StructuredCompareView,
}

impl M5TrustLadderSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookRichOutput,
        Self::DocsBrowserPanel,
        Self::AiEvidenceViewer,
        Self::PipelineArtifactBrowser,
        Self::ProviderOverlay,
        Self::MarketplaceInstallReview,
        Self::RemotePreviewTarget,
        Self::StructuredCompareView,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRichOutput => "notebook_rich_output",
            Self::DocsBrowserPanel => "docs_browser_panel",
            Self::AiEvidenceViewer => "ai_evidence_viewer",
            Self::PipelineArtifactBrowser => "pipeline_artifact_browser",
            Self::ProviderOverlay => "provider_overlay",
            Self::MarketplaceInstallReview => "marketplace_install_review",
            Self::RemotePreviewTarget => "remote_preview_target",
            Self::StructuredCompareView => "structured_compare_view",
        }
    }

    /// Whether this is a strong-decision surface (install/update, attach/share,
    /// collaboration, or policy review) that must render owner and origin
    /// identity more strictly than ordinary browsing panes.
    pub const fn is_strong_decision_surface(self) -> bool {
        matches!(
            self,
            Self::ProviderOverlay | Self::MarketplaceInstallReview | Self::RemotePreviewTarget
        )
    }

    /// Whether this is an embedded or review surface that must never
    /// auto-execute active rich content.
    pub const fn is_embedded_review_surface(self) -> bool {
        matches!(
            self,
            Self::AiEvidenceViewer | Self::PipelineArtifactBrowser | Self::StructuredCompareView
        )
    }
}

/// A rung on the safe-preview trust-class ladder, used both for the requested
/// class and the resolved effective class.
///
/// Tokens mirror the closed trust-class vocabulary in
/// [`schemas/security/trust_class.schema.json`](../../../../schemas/security/trust_class.schema.json).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustClass {
    /// Plain, raw text — no rendering, no active behavior.
    RawText,
    /// Sanitized rich rendering with active content neutralized.
    SanitizedRich,
    /// Active content that may run only inside the declared trusted-local class.
    TrustedLocalActive,
    /// Active remote content confined to an isolated runtime class.
    IsolatedRemoteActive,
    /// Compare-only fallback: raw and rendered shown side by side, both inert.
    CompareOnly,
    /// Content blocked from rendering; only redaction-safe metadata is shown.
    Blocked,
}

impl M5TrustClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawText => "raw_text",
            Self::SanitizedRich => "sanitized_rich",
            Self::TrustedLocalActive => "trusted_local_active",
            Self::IsolatedRemoteActive => "isolated_remote_active",
            Self::CompareOnly => "compare_only",
            Self::Blocked => "blocked",
        }
    }

    /// Whether this class lets active content execute (within its declared
    /// trust boundary).
    pub const fn is_active(self) -> bool {
        matches!(self, Self::TrustedLocalActive | Self::IsolatedRemoteActive)
    }

    /// Relative capability rank, used to assert resolution never escalates a
    /// surface above the class it requested. Compare-only and blocked share the
    /// inert (non-executing) capability of sanitized rich or below.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::Blocked => 0,
            Self::CompareOnly => 1,
            Self::RawText => 1,
            Self::SanitizedRich => 2,
            Self::TrustedLocalActive | Self::IsolatedRemoteActive => 3,
        }
    }

    /// Builds the ladder a surface climbs to reach `self`, from raw text upward.
    pub fn ladder_to(self) -> Vec<Self> {
        match self {
            Self::RawText => vec![Self::RawText],
            Self::SanitizedRich => vec![Self::RawText, Self::SanitizedRich],
            Self::TrustedLocalActive => {
                vec![Self::RawText, Self::SanitizedRich, Self::TrustedLocalActive]
            }
            Self::IsolatedRemoteActive => {
                vec![
                    Self::RawText,
                    Self::SanitizedRich,
                    Self::IsolatedRemoteActive,
                ]
            }
            // A surface never *requests* compare-only or blocked; they are
            // fallbacks. Their ladder is still raw → sanitized so raw stays
            // reachable underneath the fallback.
            Self::CompareOnly | Self::Blocked => vec![Self::RawText, Self::SanitizedRich],
        }
    }
}

/// The effective active-content behavior a surface resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActiveContentPosture {
    /// Active content is inert and never executes in this surface.
    InertNeverExecutes,
    /// Active content executes only inside the declared trusted-local class.
    TrustedLocalExecution,
    /// Active content executes only inside an isolated remote runtime class.
    IsolatedRemoteExecution,
    /// Active content is blocked pending review.
    BlockedPendingReview,
}

impl M5ActiveContentPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertNeverExecutes => "inert_never_executes",
            Self::TrustedLocalExecution => "trusted_local_execution",
            Self::IsolatedRemoteExecution => "isolated_remote_execution",
            Self::BlockedPendingReview => "blocked_pending_review",
        }
    }

    /// Whether this posture lets active content execute.
    pub const fn executes(self) -> bool {
        matches!(
            self,
            Self::TrustedLocalExecution | Self::IsolatedRemoteExecution
        )
    }
}

/// Decision-strictness display mode for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayMode {
    /// Ordinary browsing-pane identity rendering.
    OrdinaryBrowsing,
    /// Stricter owner/origin identity rendering for strong-decision surfaces.
    StrongDecisionStrictIdentity,
}

impl M5DisplayMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryBrowsing => "ordinary_browsing",
            Self::StrongDecisionStrictIdentity => "strong_decision_strict_identity",
        }
    }

    /// Whether this is the stricter strong-decision display mode.
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::StrongDecisionStrictIdentity)
    }
}

/// The safe-preview fallback a surface degrades into when its requested trust
/// class cannot be honored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackMode {
    /// No fallback applied; the requested trust class was honored.
    NoFallback,
    /// Degraded to sanitized rich visibility with active content neutralized.
    SanitizedVisibility,
    /// Degraded to a compare-only view: raw and rendered side by side, inert.
    CompareOnly,
    /// Blocked; only redaction-safe metadata is shown.
    BlockedMetadataOnly,
}

impl M5FallbackMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFallback => "no_fallback",
            Self::SanitizedVisibility => "sanitized_visibility",
            Self::CompareOnly => "compare_only",
            Self::BlockedMetadataOnly => "blocked_metadata_only",
        }
    }

    /// Severity rank used to take the most restrictive of several fired rules.
    const fn severity(self) -> u8 {
        match self {
            Self::NoFallback => 0,
            Self::SanitizedVisibility => 1,
            Self::CompareOnly => 2,
            Self::BlockedMetadataOnly => 3,
        }
    }

    /// The effective trust class this fallback resolves a surface to, given the
    /// originally requested class.
    fn effective_class(self, requested: M5TrustClass) -> M5TrustClass {
        match self {
            Self::NoFallback => requested,
            Self::SanitizedVisibility => M5TrustClass::SanitizedRich,
            Self::CompareOnly => M5TrustClass::CompareOnly,
            Self::BlockedMetadataOnly => M5TrustClass::Blocked,
        }
    }
}

/// A runtime trust signal that can fire a downgrade rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DowngradeTrigger {
    /// A policy or legal block applies to this surface.
    PolicyBlocked,
    /// The isolated remote runtime needed for active content is unavailable.
    IsolationRuntimeUnavailable,
    /// The declared local trust needed for trusted-local active content is not
    /// established.
    LocalTrustNotEstablished,
    /// Raw and rendered forms diverge and the divergence is unresolved.
    RawRenderedDivergenceUnresolved,
    /// Safe-preview rendering is unavailable for this surface.
    SafePreviewUnavailable,
    /// Suspicious bytes (bidi/invisible/confusable) were detected.
    SuspiciousContentDetected,
    /// The freshness proof for this surface's trust evidence is stale.
    ProofStale,
    /// The surface is an embedded or review surface that must never execute
    /// active content.
    EmbeddedReviewSurface,
}

impl M5DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PolicyBlocked,
        Self::IsolationRuntimeUnavailable,
        Self::LocalTrustNotEstablished,
        Self::RawRenderedDivergenceUnresolved,
        Self::SafePreviewUnavailable,
        Self::SuspiciousContentDetected,
        Self::ProofStale,
        Self::EmbeddedReviewSurface,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocked => "policy_blocked",
            Self::IsolationRuntimeUnavailable => "isolation_runtime_unavailable",
            Self::LocalTrustNotEstablished => "local_trust_not_established",
            Self::RawRenderedDivergenceUnresolved => "raw_rendered_divergence_unresolved",
            Self::SafePreviewUnavailable => "safe_preview_unavailable",
            Self::SuspiciousContentDetected => "suspicious_content_detected",
            Self::ProofStale => "proof_stale",
            Self::EmbeddedReviewSurface => "embedded_review_surface",
        }
    }
}

/// One named downgrade rule in the canonical catalog: a trigger plus the
/// fallback it forces and the rationale recorded for support and audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Trigger that fires this rule.
    pub trigger: M5DowngradeTrigger,
    /// Trigger token, redundant with [`Self::trigger`] for export readers.
    pub trigger_token: String,
    /// Fallback this rule forces when it fires.
    pub target_fallback: M5FallbackMode,
    /// Target fallback token, redundant with [`Self::target_fallback`].
    pub target_fallback_token: String,
    /// Whether this rule only applies when the requested class is active.
    pub applies_to_active_only: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Per-surface runtime trust signals fed into the resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5TrustSignals {
    /// A policy or legal block applies.
    pub policy_blocked: bool,
    /// The isolated remote runtime is available.
    pub isolation_runtime_available: bool,
    /// The declared local trust is established.
    pub local_trust_established: bool,
    /// Raw and rendered forms diverge and the divergence is unresolved.
    pub raw_rendered_divergence_unresolved: bool,
    /// Safe-preview rendering is available.
    pub safe_preview_available: bool,
    /// Suspicious bytes were detected.
    pub suspicious_content_detected: bool,
    /// The freshness proof is stale.
    pub proof_stale: bool,
}

impl M5TrustSignals {
    /// All-clear signals: nothing fires, every requested class is honorable.
    pub const fn all_clear() -> Self {
        Self {
            policy_blocked: false,
            isolation_runtime_available: true,
            local_trust_established: true,
            raw_rendered_divergence_unresolved: false,
            safe_preview_available: true,
            suspicious_content_detected: false,
            proof_stale: false,
        }
    }
}

/// Inputs describing a single surface's requested posture and live signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5TrustLadderSurfaceInput<'a> {
    /// Surface this input describes.
    pub surface: M5TrustLadderSurface,
    /// Opaque surface subject ref.
    pub subject_ref: &'a str,
    /// Trust class the content wants to render or execute at.
    pub requested_trust_class: M5TrustClass,
    /// Whether the content carries active behavior at all.
    pub active_content_present: bool,
    /// Live trust signals around the surface.
    pub signals: M5TrustSignals,
}

/// Inputs needed to project the M5 trust-class ladder packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5TrustLadderSeed<'a> {
    /// Stable case id shared by all surface projections.
    pub case_id: &'a str,
    /// Per-surface inputs, one per [`M5TrustLadderSurface::ALL`].
    pub surface_inputs: [M5TrustLadderSurfaceInput<'a>; 8],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// The resolved trust posture for one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustLadderSurfaceProjection {
    /// Surface this projection describes.
    pub surface: M5TrustLadderSurface,
    /// Surface token, redundant with [`Self::surface`] for export readers.
    pub surface_token: String,
    /// Opaque surface subject ref.
    pub subject_ref: String,
    /// Decision-strictness display mode for the surface.
    pub display_mode: M5DisplayMode,
    /// Display-mode token.
    pub display_mode_token: String,
    /// Trust class the content requested.
    pub requested_trust_class: M5TrustClass,
    /// Requested-class token.
    pub requested_trust_class_token: String,
    /// Effective trust class after downgrade resolution.
    pub effective_trust_class: M5TrustClass,
    /// Effective-class token.
    pub effective_trust_class_token: String,
    /// The ladder rungs this surface climbs to reach its requested class.
    pub trust_class_ladder: Vec<M5TrustClass>,
    /// Whether the content carries active behavior at all.
    pub active_content_present: bool,
    /// Effective active-content behavior.
    pub active_content_posture: M5ActiveContentPosture,
    /// Active-content posture token.
    pub active_content_posture_token: String,
    /// Safe-preview fallback applied (or [`M5FallbackMode::NoFallback`]).
    pub fallback_mode: M5FallbackMode,
    /// Fallback-mode token.
    pub fallback_mode_token: String,
    /// Whether the surface was downgraded below its requested class.
    pub downgraded: bool,
    /// Triggers that fired, in declaration order.
    pub fired_triggers: Vec<M5DowngradeTrigger>,
    /// Ids of the downgrade rules that applied.
    pub applied_downgrade_rules: Vec<String>,
    /// Whether suspicious content was detected and is annotated rather than
    /// silently normalized.
    pub suspicious_annotated: bool,
    /// Whether raw inspection stays reachable (always true).
    pub raw_inspection_reachable: bool,
    /// Whether a raw copy path stays reachable (always true).
    pub raw_copy_reachable: bool,
    /// Whether a compare-only fallback is reachable for this surface.
    pub compare_only_fallback_available: bool,
    /// Human-readable rationale for the resolved posture.
    pub rationale: String,
}

impl M5TrustLadderSurfaceProjection {
    /// Whether the surface honored its requested class with no fallback.
    pub fn is_honored(&self) -> bool {
        self.fallback_mode == M5FallbackMode::NoFallback
    }
}

/// Trust review block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustLadderReview {
    /// One shared suspicious-text and content-integrity policy library governs all surfaces.
    pub one_shared_policy_library_governs_all_surfaces: bool,
    /// Active content never executes outside its declared trust class.
    pub active_content_never_executes_outside_declared_class: bool,
    /// Unsafe or unsupported states degrade to sanitized or compare-only, never silent execution.
    pub unsafe_states_degrade_to_sanitized_or_compare_only: bool,
    /// Downgrade narrows the posture rather than hiding the surface or failing opaquely.
    pub downgrade_narrows_instead_of_hiding: bool,
    /// Raw inspection and raw copy stay reachable on every surface.
    pub raw_inspection_and_copy_always_reachable: bool,
    /// Suspicious bytes are surfaced, never normalized away.
    pub suspicious_bytes_never_normalized_away: bool,
    /// Rendered copy never masquerades as raw bytes.
    pub rendered_never_masquerades_as_raw: bool,
    /// Embedded or review surfaces never auto-execute active content.
    pub no_auto_execute_in_embedded_or_review_surfaces: bool,
    /// Strong-decision surfaces use stricter identity rendering.
    pub strong_decision_surfaces_use_strict_identity: bool,
    /// A compare-only fallback is reachable whenever the rendered form cannot be trusted.
    pub compare_only_fallback_available_when_render_untrusted: bool,
}

impl M5TrustLadderReview {
    /// The frozen, all-invariants-hold review block.
    pub const fn frozen() -> Self {
        Self {
            one_shared_policy_library_governs_all_surfaces: true,
            active_content_never_executes_outside_declared_class: true,
            unsafe_states_degrade_to_sanitized_or_compare_only: true,
            downgrade_narrows_instead_of_hiding: true,
            raw_inspection_and_copy_always_reachable: true,
            suspicious_bytes_never_normalized_away: true,
            rendered_never_masquerades_as_raw: true,
            no_auto_execute_in_embedded_or_review_surfaces: true,
            strong_decision_surfaces_use_strict_identity: true,
            compare_only_fallback_available_when_render_untrusted: true,
        }
    }

    fn all_hold(&self) -> bool {
        self.one_shared_policy_library_governs_all_surfaces
            && self.active_content_never_executes_outside_declared_class
            && self.unsafe_states_degrade_to_sanitized_or_compare_only
            && self.downgrade_narrows_instead_of_hiding
            && self.raw_inspection_and_copy_always_reachable
            && self.suspicious_bytes_never_normalized_away
            && self.rendered_never_masquerades_as_raw
            && self.no_auto_execute_in_embedded_or_review_surfaces
            && self.strong_decision_surfaces_use_strict_identity
            && self.compare_only_fallback_available_when_render_untrusted
    }
}

/// Cross-surface trust-class ladder and downgrade resolution packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustClassLadderPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Number of surfaces downgraded below their requested class.
    pub downgraded_surface_count: usize,
    /// Distinct fallback-mode tokens across surfaces.
    pub fallback_kinds: Vec<String>,
    /// Whether projection normalized or stripped any source (always false).
    pub normalization_applied: bool,
    /// Per-surface resolved projections.
    pub surfaces: Vec<M5TrustLadderSurfaceProjection>,
    /// Canonical downgrade-rule catalog.
    pub downgrade_rules: Vec<M5DowngradeRule>,
    /// Trust review block.
    pub trust_review: M5TrustLadderReview,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TrustClassLadderPacket {
    /// Returns true when every required surface is present exactly once.
    pub fn covers_all_surfaces(&self) -> bool {
        let present: BTreeSet<_> = self.surfaces.iter().map(|s| s.surface).collect();
        M5TrustLadderSurface::ALL
            .iter()
            .all(|s| present.contains(s))
            && present.len() == M5TrustLadderSurface::ALL.len()
    }

    /// Returns true when the downgrade-rule catalog covers every trigger.
    pub fn downgrade_rules_cover_every_trigger(&self) -> bool {
        let present: BTreeSet<_> = self.downgrade_rules.iter().map(|r| r.trigger).collect();
        M5DowngradeTrigger::ALL.iter().all(|t| present.contains(t))
    }

    /// Returns true when no surface executes active content outside its
    /// declared trust class.
    pub fn active_content_confined_to_declared_class(&self) -> bool {
        self.surfaces
            .iter()
            .all(|s| match s.active_content_posture {
                M5ActiveContentPosture::TrustedLocalExecution => {
                    s.effective_trust_class == M5TrustClass::TrustedLocalActive
                }
                M5ActiveContentPosture::IsolatedRemoteExecution => {
                    s.effective_trust_class == M5TrustClass::IsolatedRemoteActive
                }
                M5ActiveContentPosture::InertNeverExecutes => !s.effective_trust_class.is_active(),
                M5ActiveContentPosture::BlockedPendingReview => {
                    s.effective_trust_class == M5TrustClass::Blocked
                }
            })
    }

    /// Returns true when no embedded or review surface executes active content.
    pub fn embedded_review_surfaces_never_execute(&self) -> bool {
        self.surfaces.iter().all(|s| {
            !s.surface.is_embedded_review_surface() || !s.active_content_posture.executes()
        })
    }

    /// Returns true when every strong-decision surface uses strict display.
    pub fn strong_decision_surfaces_use_strict_display(&self) -> bool {
        self.surfaces
            .iter()
            .all(|s| !s.surface.is_strong_decision_surface() || s.display_mode.is_strict())
    }

    /// Returns true when resolution never escalated a surface above the class it
    /// requested.
    pub fn never_escalates_above_request(&self) -> bool {
        self.surfaces.iter().all(|s| {
            s.effective_trust_class.capability_rank() <= s.requested_trust_class.capability_rank()
        })
    }

    /// Returns true when every downgraded surface degrades into a defined safe
    /// fallback with a recorded rationale, never an opaque failure.
    pub fn downgrade_narrows_without_opaque_failure(&self) -> bool {
        self.surfaces.iter().all(|s| {
            if !s.downgraded {
                return s.fallback_mode == M5FallbackMode::NoFallback;
            }
            matches!(
                s.fallback_mode,
                M5FallbackMode::SanitizedVisibility
                    | M5FallbackMode::CompareOnly
                    | M5FallbackMode::BlockedMetadataOnly
            ) && !s.rationale.trim().is_empty()
                && !s.applied_downgrade_rules.is_empty()
        })
    }

    /// Returns true when raw inspection and raw copy stay reachable everywhere.
    pub fn raw_always_reachable(&self) -> bool {
        self.surfaces
            .iter()
            .all(|s| s.raw_inspection_reachable && s.raw_copy_reachable)
    }

    /// Validates the trust-class ladder invariants.
    pub fn validate(&self) -> Vec<M5TrustClassLadderViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TRUST_CLASS_LADDER_RECORD_KIND {
            violations.push(M5TrustClassLadderViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TRUST_CLASS_LADDER_SCHEMA_VERSION {
            violations.push(M5TrustClassLadderViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(M5TrustClassLadderViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5TrustClassLadderViolation::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(M5TrustClassLadderViolation::NormalizationApplied);
        }
        if !self.covers_all_surfaces() {
            violations.push(M5TrustClassLadderViolation::SurfaceMissing);
        }
        if !self.downgrade_rules_cover_every_trigger() {
            violations.push(M5TrustClassLadderViolation::DowngradeRuleCoverageIncomplete);
        }
        if !self.active_content_confined_to_declared_class() {
            violations.push(M5TrustClassLadderViolation::ActiveContentOutsideDeclaredClass);
        }
        if !self.embedded_review_surfaces_never_execute() {
            violations.push(M5TrustClassLadderViolation::ActiveContentInReviewSurface);
        }
        if !self.strong_decision_surfaces_use_strict_display() {
            violations.push(M5TrustClassLadderViolation::StrongDecisionDisplayTooWeak);
        }
        if !self.never_escalates_above_request() {
            violations.push(M5TrustClassLadderViolation::EscalatedAboveRequest);
        }
        if !self.downgrade_narrows_without_opaque_failure() {
            violations.push(M5TrustClassLadderViolation::OpaqueDowngrade);
        }
        if !self.raw_always_reachable() {
            violations.push(M5TrustClassLadderViolation::RawInspectionUnreachable);
        }
        if self.downgraded_surface_count != self.declared_downgraded_count() {
            violations.push(M5TrustClassLadderViolation::DowngradedCountMismatch);
        }
        for surface in &self.surfaces {
            if surface.trust_class_ladder.is_empty() {
                violations.push(M5TrustClassLadderViolation::TrustClassLadderMissing);
                break;
            }
        }
        // The compare-only floor: when a surface can render at all, a
        // compare-only fallback must stay reachable so the rendered form is
        // never the only authority.
        for surface in &self.surfaces {
            if surface.effective_trust_class != M5TrustClass::Blocked
                && !surface.compare_only_fallback_available
            {
                violations.push(M5TrustClassLadderViolation::CompareOnlyFallbackMissing);
                break;
            }
        }
        if !self.trust_review.all_hold() {
            violations.push(M5TrustClassLadderViolation::TrustReviewIncomplete);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 trust-class ladder packet serializes"),
        ) {
            violations.push(M5TrustClassLadderViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    fn declared_downgraded_count(&self) -> usize {
        self.surfaces.iter().filter(|s| s.downgraded).count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 trust-class ladder packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Trust-Class Ladders, Downgrade Rules & Compare-Only Fallbacks\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Downgraded surfaces: {} of {}\n",
            self.downgraded_surface_count,
            self.surfaces.len()
        ));
        out.push_str(&format!(
            "- Fallback kinds: {}\n",
            self.fallback_kinds.join(", ")
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** ({}): requested `{}` → effective `{}` (fallback `{}`)\n",
                surface.surface.as_str(),
                surface.display_mode.as_str(),
                surface.requested_trust_class.as_str(),
                surface.effective_trust_class.as_str(),
                surface.fallback_mode.as_str(),
            ));
            out.push_str(&format!(
                "  - Active content: {} · raw reachable: {}\n",
                surface.active_content_posture.as_str(),
                surface.raw_copy_reachable,
            ));
            if !surface.fired_triggers.is_empty() {
                let triggers = surface
                    .fired_triggers
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("  - Triggers: {triggers}\n"));
            }
        }
        out.push_str("\n## Downgrade rules\n\n");
        for rule in &self.downgrade_rules {
            out.push_str(&format!(
                "- `{}`: on `{}` → `{}`\n",
                rule.rule_id,
                rule.trigger.as_str(),
                rule.target_fallback.as_str(),
            ));
        }
        out
    }
}

/// Validation failures emitted by [`M5TrustClassLadderPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TrustClassLadderViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Projection normalized or stripped the source bytes.
    NormalizationApplied,
    /// A required surface is missing.
    SurfaceMissing,
    /// The downgrade-rule catalog does not cover every trigger.
    DowngradeRuleCoverageIncomplete,
    /// A surface executes active content outside its declared trust class.
    ActiveContentOutsideDeclaredClass,
    /// An embedded or review surface executes active content.
    ActiveContentInReviewSurface,
    /// A strong-decision surface does not use strict display mode.
    StrongDecisionDisplayTooWeak,
    /// Resolution escalated a surface above the class it requested.
    EscalatedAboveRequest,
    /// A downgraded surface failed opaquely instead of degrading to a safe fallback.
    OpaqueDowngrade,
    /// Raw inspection or raw copy is unreachable on some surface.
    RawInspectionUnreachable,
    /// The declared downgraded-surface count does not match the projections.
    DowngradedCountMismatch,
    /// A surface has an empty trust-class ladder.
    TrustClassLadderMissing,
    /// A renderable surface has no reachable compare-only fallback.
    CompareOnlyFallbackMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5TrustClassLadderViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::SurfaceMissing => "surface_missing",
            Self::DowngradeRuleCoverageIncomplete => "downgrade_rule_coverage_incomplete",
            Self::ActiveContentOutsideDeclaredClass => "active_content_outside_declared_class",
            Self::ActiveContentInReviewSurface => "active_content_in_review_surface",
            Self::StrongDecisionDisplayTooWeak => "strong_decision_display_too_weak",
            Self::EscalatedAboveRequest => "escalated_above_request",
            Self::OpaqueDowngrade => "opaque_downgrade",
            Self::RawInspectionUnreachable => "raw_inspection_unreachable",
            Self::DowngradedCountMismatch => "downgraded_count_mismatch",
            Self::TrustClassLadderMissing => "trust_class_ladder_missing",
            Self::CompareOnlyFallbackMissing => "compare_only_fallback_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in trust-class ladder export.
#[derive(Debug)]
pub enum M5TrustClassLadderExportError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TrustClassLadderViolation>),
}

impl std::fmt::Display for M5TrustClassLadderExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 trust-class ladder export parse failed: {error}"
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
                    "m5 trust-class ladder export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5TrustClassLadderExportError {}

/// The canonical downgrade-rule catalog: one rule per trigger, in the order
/// resolution evaluates them (most restrictive first).
pub fn m5_downgrade_rule_catalog() -> Vec<M5DowngradeRule> {
    use M5DowngradeTrigger as Trigger;
    use M5FallbackMode as Fallback;

    let rule =
        |rule_id: &str, trigger: Trigger, target: Fallback, active_only: bool, rationale: &str| {
            M5DowngradeRule {
                rule_id: rule_id.to_owned(),
                trigger,
                trigger_token: trigger.as_str().to_owned(),
                target_fallback: target,
                target_fallback_token: target.as_str().to_owned(),
                applies_to_active_only: active_only,
                rationale: rationale.to_owned(),
            }
        };

    vec![
        rule(
            "policy_block_forces_blocked",
            Trigger::PolicyBlocked,
            Fallback::BlockedMetadataOnly,
            false,
            "A policy or legal block blocks rendering; only redaction-safe metadata is shown.",
        ),
        rule(
            "unresolved_divergence_forces_compare_only",
            Trigger::RawRenderedDivergenceUnresolved,
            Fallback::CompareOnly,
            false,
            "Raw and rendered forms diverge unresolved, so the surface shows both side by side rather than letting the rendered form claim authority.",
        ),
        rule(
            "safe_preview_unavailable_forces_compare_only",
            Trigger::SafePreviewUnavailable,
            Fallback::CompareOnly,
            false,
            "Safe-preview rendering is unavailable, so the surface falls back to a compare-only view instead of failing opaquely.",
        ),
        rule(
            "isolation_unavailable_downgrades_active_to_sanitized",
            Trigger::IsolationRuntimeUnavailable,
            Fallback::SanitizedVisibility,
            true,
            "The isolated remote runtime is unavailable, so isolated-remote-active content degrades to sanitized rich visibility and never executes.",
        ),
        rule(
            "local_trust_absent_downgrades_active_to_sanitized",
            Trigger::LocalTrustNotEstablished,
            Fallback::SanitizedVisibility,
            true,
            "The declared local trust is not established, so trusted-local-active content degrades to sanitized rich visibility and never executes.",
        ),
        rule(
            "suspicious_content_downgrades_active_to_sanitized",
            Trigger::SuspiciousContentDetected,
            Fallback::SanitizedVisibility,
            true,
            "Suspicious bytes were detected, so active content degrades to annotated sanitized visibility with raw inspection reachable; bytes are never normalized away.",
        ),
        rule(
            "proof_stale_narrows_active_to_sanitized",
            Trigger::ProofStale,
            Fallback::SanitizedVisibility,
            true,
            "The trust-evidence freshness proof is stale, so active content narrows to sanitized rich visibility until the proof refreshes.",
        ),
        rule(
            "embedded_review_surface_never_executes",
            Trigger::EmbeddedReviewSurface,
            Fallback::SanitizedVisibility,
            true,
            "Embedded and review surfaces never auto-execute active content; any active request degrades to sanitized rich visibility.",
        ),
    ]
}

/// Resolves a single surface's effective posture from its requested class and
/// signals by walking the downgrade-rule catalog.
fn resolve_surface(input: &M5TrustLadderSurfaceInput<'_>) -> M5TrustLadderSurfaceProjection {
    use M5DowngradeTrigger as Trigger;
    use M5FallbackMode as Fallback;

    let surface = input.surface;
    let requested = input.requested_trust_class;
    let signals = input.signals;
    let requested_is_active = requested.is_active();

    let display_mode = if surface.is_strong_decision_surface() {
        M5DisplayMode::StrongDecisionStrictIdentity
    } else {
        M5DisplayMode::OrdinaryBrowsing
    };

    let mut fired: Vec<Trigger> = Vec::new();
    let mut applied: Vec<String> = Vec::new();
    let mut fallback = Fallback::NoFallback;

    let fire = |trigger: Trigger,
                rule_id: &str,
                target: Fallback,
                fired: &mut Vec<Trigger>,
                applied: &mut Vec<String>,
                fallback: &mut Fallback| {
        fired.push(trigger);
        applied.push(rule_id.to_owned());
        if target.severity() > fallback.severity() {
            *fallback = target;
        }
    };

    if signals.policy_blocked {
        fire(
            Trigger::PolicyBlocked,
            "policy_block_forces_blocked",
            Fallback::BlockedMetadataOnly,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if signals.raw_rendered_divergence_unresolved {
        fire(
            Trigger::RawRenderedDivergenceUnresolved,
            "unresolved_divergence_forces_compare_only",
            Fallback::CompareOnly,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if !signals.safe_preview_available {
        fire(
            Trigger::SafePreviewUnavailable,
            "safe_preview_unavailable_forces_compare_only",
            Fallback::CompareOnly,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if requested == M5TrustClass::IsolatedRemoteActive && !signals.isolation_runtime_available {
        fire(
            Trigger::IsolationRuntimeUnavailable,
            "isolation_unavailable_downgrades_active_to_sanitized",
            Fallback::SanitizedVisibility,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if requested == M5TrustClass::TrustedLocalActive && !signals.local_trust_established {
        fire(
            Trigger::LocalTrustNotEstablished,
            "local_trust_absent_downgrades_active_to_sanitized",
            Fallback::SanitizedVisibility,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if signals.suspicious_content_detected && requested_is_active {
        fire(
            Trigger::SuspiciousContentDetected,
            "suspicious_content_downgrades_active_to_sanitized",
            Fallback::SanitizedVisibility,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if signals.proof_stale && requested_is_active {
        fire(
            Trigger::ProofStale,
            "proof_stale_narrows_active_to_sanitized",
            Fallback::SanitizedVisibility,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }
    if surface.is_embedded_review_surface() && requested_is_active {
        fire(
            Trigger::EmbeddedReviewSurface,
            "embedded_review_surface_never_executes",
            Fallback::SanitizedVisibility,
            &mut fired,
            &mut applied,
            &mut fallback,
        );
    }

    let effective = fallback.effective_class(requested);
    let active_content_posture = match effective {
        M5TrustClass::TrustedLocalActive => M5ActiveContentPosture::TrustedLocalExecution,
        M5TrustClass::IsolatedRemoteActive => M5ActiveContentPosture::IsolatedRemoteExecution,
        M5TrustClass::Blocked => M5ActiveContentPosture::BlockedPendingReview,
        _ => M5ActiveContentPosture::InertNeverExecutes,
    };
    let downgraded = fallback != Fallback::NoFallback;
    let compare_only_fallback_available = effective != M5TrustClass::Blocked;

    let rationale = build_rationale(surface, requested, effective, fallback, &fired);

    M5TrustLadderSurfaceProjection {
        surface,
        surface_token: surface.as_str().to_owned(),
        subject_ref: input.subject_ref.to_owned(),
        display_mode,
        display_mode_token: display_mode.as_str().to_owned(),
        requested_trust_class: requested,
        requested_trust_class_token: requested.as_str().to_owned(),
        effective_trust_class: effective,
        effective_trust_class_token: effective.as_str().to_owned(),
        trust_class_ladder: requested.ladder_to(),
        active_content_present: input.active_content_present,
        active_content_posture,
        active_content_posture_token: active_content_posture.as_str().to_owned(),
        fallback_mode: fallback,
        fallback_mode_token: fallback.as_str().to_owned(),
        downgraded,
        fired_triggers: fired,
        applied_downgrade_rules: applied,
        suspicious_annotated: signals.suspicious_content_detected,
        raw_inspection_reachable: true,
        raw_copy_reachable: true,
        compare_only_fallback_available,
        rationale,
    }
}

fn build_rationale(
    surface: M5TrustLadderSurface,
    requested: M5TrustClass,
    effective: M5TrustClass,
    fallback: M5FallbackMode,
    fired: &[M5DowngradeTrigger],
) -> String {
    if fallback == M5FallbackMode::NoFallback {
        return format!(
            "{} renders at its requested {} trust class; no downgrade trigger fired.",
            surface.as_str(),
            requested.as_str()
        );
    }
    let triggers = fired
        .iter()
        .map(|t| t.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{} requested {} but degraded to {} ({}) because: {}.",
        surface.as_str(),
        requested.as_str(),
        effective.as_str(),
        fallback.as_str(),
        triggers
    )
}

fn distinct_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<String> {
    let set: BTreeSet<&str> = tokens.collect();
    set.into_iter().map(str::to_owned).collect()
}

/// Projects the trust-class ladder, downgrade resolution, and compare-only
/// fallbacks across every new M5 preview and embedded surface.
pub fn project_m5_trust_class_ladder(seed: &M5TrustLadderSeed<'_>) -> M5TrustClassLadderPacket {
    let surfaces: Vec<_> = seed.surface_inputs.iter().map(resolve_surface).collect();

    let downgraded_surface_count = surfaces.iter().filter(|s| s.downgraded).count();
    let fallback_kinds = distinct_tokens(surfaces.iter().map(|s| s.fallback_mode.as_str()));

    M5TrustClassLadderPacket {
        record_kind: M5_TRUST_CLASS_LADDER_RECORD_KIND.to_owned(),
        schema_version: M5_TRUST_CLASS_LADDER_SCHEMA_VERSION,
        packet_id: M5_TRUST_CLASS_LADDER_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        downgraded_surface_count,
        fallback_kinds,
        normalization_applied: false,
        surfaces,
        downgrade_rules: m5_downgrade_rule_catalog(),
        trust_review: M5TrustLadderReview::frozen(),
        source_contract_refs: vec![
            M5_TRUST_CLASS_LADDER_SCHEMA_REF.to_owned(),
            M5_TRUST_CLASS_LADDER_DOC_REF.to_owned(),
            M5_TRUST_CLASS_LADDER_TRUST_CLASS_CONTRACT_REF.to_owned(),
            M5_TRUST_CLASS_LADDER_SAFE_PREVIEW_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen trust-class ladder packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_TRUST_CLASS_LADDER_ARTIFACT_REF`]; the bin emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged. The
/// scenario exercises every fallback mode — honored, sanitized, compare-only,
/// and blocked — and every trust class.
pub fn frozen_m5_trust_class_ladder_packet() -> M5TrustClassLadderPacket {
    use M5TrustClass as Class;
    use M5TrustLadderSurface as Surface;

    let clear = M5TrustSignals::all_clear;

    let surface_inputs = [
        // Honored trusted-local-active: a notebook running locally trusted code.
        M5TrustLadderSurfaceInput {
            surface: Surface::NotebookRichOutput,
            subject_ref: "notebook:cell:demo:out:2",
            requested_trust_class: Class::TrustedLocalActive,
            active_content_present: true,
            signals: clear(),
        },
        // Honored isolated-remote-active: a docs panel embedding sandboxed content.
        M5TrustLadderSurfaceInput {
            surface: Surface::DocsBrowserPanel,
            subject_ref: "docs:page:guide#embed",
            requested_trust_class: Class::IsolatedRemoteActive,
            active_content_present: true,
            signals: clear(),
        },
        // Compare-only: AI evidence whose rendered summary diverges from raw.
        M5TrustLadderSurfaceInput {
            surface: Surface::AiEvidenceViewer,
            subject_ref: "ai:evidence:review:finding:7",
            requested_trust_class: Class::SanitizedRich,
            active_content_present: false,
            signals: M5TrustSignals {
                raw_rendered_divergence_unresolved: true,
                ..clear()
            },
        },
        // Compare-only: pipeline artifact whose safe preview is unavailable.
        M5TrustLadderSurfaceInput {
            surface: Surface::PipelineArtifactBrowser,
            subject_ref: "pipeline:run:128:artifact:logs",
            requested_trust_class: Class::SanitizedRich,
            active_content_present: false,
            signals: M5TrustSignals {
                safe_preview_available: false,
                ..clear()
            },
        },
        // Honored sanitized with suspicious annotation in strict identity mode.
        M5TrustLadderSurfaceInput {
            surface: Surface::ProviderOverlay,
            subject_ref: "provider:policy:overlay:org",
            requested_trust_class: Class::SanitizedRich,
            active_content_present: false,
            signals: M5TrustSignals {
                suspicious_content_detected: true,
                ..clear()
            },
        },
        // Sanitized fallback: marketplace active demo with no isolation runtime.
        M5TrustLadderSurfaceInput {
            surface: Surface::MarketplaceInstallReview,
            subject_ref: "marketplace:listing:demo@2.0.0",
            requested_trust_class: Class::IsolatedRemoteActive,
            active_content_present: true,
            signals: M5TrustSignals {
                isolation_runtime_available: false,
                ..clear()
            },
        },
        // Blocked: remote preview target under a policy block.
        M5TrustLadderSurfaceInput {
            surface: Surface::RemotePreviewTarget,
            subject_ref: "remote:preview:target:pr:128",
            requested_trust_class: Class::IsolatedRemoteActive,
            active_content_present: true,
            signals: M5TrustSignals {
                policy_blocked: true,
                ..clear()
            },
        },
        // Honored sanitized: a structured compare view, inert by construction.
        M5TrustLadderSurfaceInput {
            surface: Surface::StructuredCompareView,
            subject_ref: "review:diff:pr:128:file:3",
            requested_trust_class: Class::SanitizedRich,
            active_content_present: false,
            signals: clear(),
        },
    ];

    project_m5_trust_class_ladder(&M5TrustLadderSeed {
        case_id: "case:m5-trust-class-ladder:stable",
        surface_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in trust-class ladder support export.
pub fn current_m5_trust_class_ladder_export(
) -> Result<M5TrustClassLadderPacket, M5TrustClassLadderExportError> {
    let packet: M5TrustClassLadderPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_trust_class_ladder/support_export.json"
    )))
    .map_err(M5TrustClassLadderExportError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TrustClassLadderExportError::Validation(violations))
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
