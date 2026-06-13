//! Safe-preview limited mode, oversized/generated-artifact banners,
//! open-raw/open-source actions, and no-silent-expensive-render guards for the
//! new M5 log, lockfile, snapshot, bundle, evidence-packet, and generated
//! artifact families.
//!
//! The frozen content-integrity matrix in
//! [`crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`]
//! locks the *static* qualification each M5 artifact/viewer family may claim,
//! [`crate::m5_trust_class_ladder`] resolves the *runtime* trust class of active
//! content, and [`crate::m5_raw_rendered_handoff`] keeps raw-versus-rendered
//! copy/export honest. This lane covers the orthogonal *cost* gap they leave
//! open: a large or generated artifact must not jump straight into an expensive
//! or unsafe render path just because the surface can technically try.
//!
//! Every large or generated artifact family opens in
//! [`M5OpenMode::SafePreviewLimited`] first: a bounded preview plus explicit
//! banners. Oversized artifacts carry an [`M5LimitedModeBannerKind::Oversized`]
//! banner; generated/derived artifacts carry an
//! [`M5LimitedModeBannerKind::GeneratedArtifact`] banner that names the
//! canonical source or generator they came from, so the derived artifact never
//! pretends to be the only truth. The fuller, more expensive render path is an
//! [`M5LimitedModeActionKind::ExpandFullRender`] action gated behind an explicit
//! opt-in ([`M5ActionPosture::RequiresExplicitOptIn`]) — it never fires
//! silently. Open-raw and open-canonical-source actions stay reachable on every
//! artifact, suspicious bytes are never normalized away, and the default view is
//! never an expensive or unsafe render.
//!
//! The boundary schema is
//! [`schemas/security/m5-safe-preview-limited-mode.schema.json`](../../../../schemas/security/m5-safe-preview-limited-mode.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_safe_preview_limited_mode.md`](../../../../docs/security/m5/m5_safe_preview_limited_mode.md).
//! The protected fixture directory is
//! [`fixtures/security/m5/m5_safe_preview_limited_mode/`](../../../../fixtures/security/m5/m5_safe_preview_limited_mode/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SafePreviewLimitedModePacket`].
pub const M5_SAFE_PREVIEW_LIMITED_RECORD_KIND: &str = "m5_safe_preview_limited_mode_packet";

/// Integer schema version for the M5 safe-preview limited-mode packet.
pub const M5_SAFE_PREVIEW_LIMITED_SCHEMA_VERSION: u32 = 1;

/// Stable packet id minted by [`frozen_m5_safe_preview_limited_mode_packet`].
pub const M5_SAFE_PREVIEW_LIMITED_PACKET_ID: &str = "m5-safe-preview-limited-mode:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_SAFE_PREVIEW_LIMITED_SCHEMA_REF: &str =
    "schemas/security/m5-safe-preview-limited-mode.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SAFE_PREVIEW_LIMITED_DOC_REF: &str =
    "docs/security/m5/m5_safe_preview_limited_mode.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SAFE_PREVIEW_LIMITED_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_safe_preview_limited_mode";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SAFE_PREVIEW_LIMITED_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_safe_preview_limited_mode/support_export.json";

/// Repo-relative path of the sibling trust-class ladder contract.
pub const M5_SAFE_PREVIEW_LIMITED_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/m5-trust-class-ladder.schema.json";

/// Repo-relative path of the sibling raw-versus-rendered handoff contract.
pub const M5_SAFE_PREVIEW_LIMITED_RAW_RENDERED_CONTRACT_REF: &str =
    "schemas/security/m5-raw-rendered-handoff.schema.json";

/// Byte budget above which an artifact is treated as oversized and opens in
/// limited mode first.
pub const M5_SAFE_PREVIEW_BYTE_BUDGET: u64 = 256 * 1024;

/// Line budget above which an artifact is treated as oversized and opens in
/// limited mode first.
pub const M5_SAFE_PREVIEW_LINE_BUDGET: u64 = 5_000;

/// A large or generated M5 artifact family whose viewer must default into a
/// bounded safe preview before any expensive or unsafe render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LimitedModeArtifactFamily {
    /// Build, run, or pipeline logs.
    BuildLog,
    /// Dependency lockfiles generated from a manifest.
    DependencyLockfile,
    /// Test or UI snapshots generated from a test.
    TestSnapshot,
    /// Distribution bundles built from sources.
    DistributionBundle,
    /// Incident or review evidence packets assembled from underlying records.
    EvidencePacket,
    /// Generated source or documentation produced by a generator.
    GeneratedArtifact,
}

impl M5LimitedModeArtifactFamily {
    /// Every family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BuildLog,
        Self::DependencyLockfile,
        Self::TestSnapshot,
        Self::DistributionBundle,
        Self::EvidencePacket,
        Self::GeneratedArtifact,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildLog => "build_log",
            Self::DependencyLockfile => "dependency_lockfile",
            Self::TestSnapshot => "test_snapshot",
            Self::DistributionBundle => "distribution_bundle",
            Self::EvidencePacket => "evidence_packet",
            Self::GeneratedArtifact => "generated_artifact",
        }
    }

    /// The kind of canonical source or generator this family derives from, used
    /// for the generated-artifact banner and open-source action label.
    pub const fn canonical_source_kind(self) -> &'static str {
        match self {
            Self::BuildLog => "originating run",
            Self::DependencyLockfile => "source manifest",
            Self::TestSnapshot => "originating test",
            Self::DistributionBundle => "build inputs",
            Self::EvidencePacket => "underlying records",
            Self::GeneratedArtifact => "generator source",
        }
    }
}

/// The initial mode a viewer opens an artifact in, before any explicit opt-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OpenMode {
    /// A bounded preview shown first: limited bytes/lines, banners, and explicit
    /// open-raw / open-source / expand actions.
    SafePreviewLimited,
    /// The full render shown immediately; only chosen for small, non-generated
    /// artifacts whose full render is cheap and inert.
    FullRenderInline,
}

impl M5OpenMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafePreviewLimited => "safe_preview_limited",
            Self::FullRenderInline => "full_render_inline",
        }
    }

    /// Whether this is the bounded, limited-preview mode.
    pub const fn is_limited(self) -> bool {
        matches!(self, Self::SafePreviewLimited)
    }
}

/// The cost class of a render path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RenderCost {
    /// Cheap: bounded preview, raw text head, or metadata; safe to do by default.
    Cheap,
    /// Expensive: full syntax highlight, full structured tree, or full diff.
    Expensive,
    /// Unsafe: would render active content; must never fire without opt-in.
    Unsafe,
}

impl M5RenderCost {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cheap => "cheap",
            Self::Expensive => "expensive",
            Self::Unsafe => "unsafe",
        }
    }

    /// Whether this cost requires an explicit user opt-in before it may run.
    pub const fn requires_opt_in(self) -> bool {
        matches!(self, Self::Expensive | Self::Unsafe)
    }
}

/// The kind of banner a limited-mode viewer shows above the bounded preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LimitedModeBannerKind {
    /// The artifact exceeds the preview byte/line budget.
    Oversized,
    /// The artifact is generated/derived and names its canonical source.
    GeneratedArtifact,
    /// The preview is bounded; the full artifact is not all shown.
    LimitedPreview,
    /// A fuller, expensive render is available but guarded behind opt-in.
    ExpensiveRenderGuarded,
    /// Active content is present and would render only behind explicit opt-in.
    ActiveContentGuarded,
}

impl M5LimitedModeBannerKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Oversized => "oversized",
            Self::GeneratedArtifact => "generated_artifact",
            Self::LimitedPreview => "limited_preview",
            Self::ExpensiveRenderGuarded => "expensive_render_guarded",
            Self::ActiveContentGuarded => "active_content_guarded",
        }
    }
}

/// A typed banner surfaced above a bounded preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LimitedModeBanner {
    /// Banner kind.
    pub kind: M5LimitedModeBannerKind,
    /// Banner-kind token, redundant with [`Self::kind`] for export readers.
    pub kind_token: String,
    /// Human-readable banner message.
    pub message: String,
}

/// The kind of action affordance a limited-mode viewer offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LimitedModeActionKind {
    /// Open the raw bytes/text of the artifact itself.
    OpenRaw,
    /// Open the canonical source or generator the artifact derived from.
    OpenCanonicalSource,
    /// Expand into the fuller, more expensive (or unsafe) render path.
    ExpandFullRender,
}

impl M5LimitedModeActionKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRaw => "open_raw",
            Self::OpenCanonicalSource => "open_canonical_source",
            Self::ExpandFullRender => "expand_full_render",
        }
    }
}

/// Whether an action is available immediately or needs an explicit opt-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActionPosture {
    /// The action is cheap and inert; it may run on a single direct request.
    AvailableImmediately,
    /// The action is expensive or unsafe; it requires an explicit opt-in.
    RequiresExplicitOptIn,
}

impl M5ActionPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableImmediately => "available_immediately",
            Self::RequiresExplicitOptIn => "requires_explicit_opt_in",
        }
    }

    /// Whether this posture gates the action behind an explicit opt-in.
    pub const fn is_gated(self) -> bool {
        matches!(self, Self::RequiresExplicitOptIn)
    }
}

/// An action affordance with its posture, render cost, and target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LimitedModeAction {
    /// Action kind.
    pub kind: M5LimitedModeActionKind,
    /// Action-kind token, redundant with [`Self::kind`] for export readers.
    pub kind_token: String,
    /// Whether the action is available immediately or gated behind opt-in.
    pub posture: M5ActionPosture,
    /// Posture token.
    pub posture_token: String,
    /// Render cost incurred when the action runs.
    pub render_cost: M5RenderCost,
    /// Render-cost token.
    pub render_cost_token: String,
    /// Opaque ref the action targets (raw subject or canonical source).
    pub target_ref: String,
    /// Human-readable action label.
    pub label: String,
}

/// Inputs describing one artifact's size, derivation, and render cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5LimitedModeArtifactInput<'a> {
    /// Family this input describes.
    pub family: M5LimitedModeArtifactFamily,
    /// Opaque artifact subject ref.
    pub subject_ref: &'a str,
    /// Opaque canonical-source or generator ref the artifact derived from.
    pub canonical_source_ref: &'a str,
    /// Size of the artifact in bytes.
    pub byte_size: u64,
    /// Line count of the artifact.
    pub line_count: u64,
    /// Whether the artifact is generated/derived from a canonical source.
    pub generated: bool,
    /// Whether the full render path is expensive.
    pub full_render_is_expensive: bool,
    /// Whether the artifact carries active rich content.
    pub active_content_present: bool,
}

/// Inputs needed to project the M5 safe-preview limited-mode packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5LimitedModeSeed<'a> {
    /// Stable case id shared by all artifact projections.
    pub case_id: &'a str,
    /// Per-artifact inputs, one per [`M5LimitedModeArtifactFamily::ALL`].
    pub artifact_inputs: [M5LimitedModeArtifactInput<'a>; 6],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// The resolved limited-mode posture for one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LimitedModeArtifactProjection {
    /// Family this projection describes.
    pub family: M5LimitedModeArtifactFamily,
    /// Family token, redundant with [`Self::family`] for export readers.
    pub family_token: String,
    /// Opaque artifact subject ref.
    pub subject_ref: String,
    /// Opaque canonical-source or generator ref the artifact derived from.
    pub canonical_source_ref: String,
    /// Size of the artifact in bytes.
    pub byte_size: u64,
    /// Line count of the artifact.
    pub line_count: u64,
    /// Whether the artifact is generated/derived.
    pub generated: bool,
    /// Whether the artifact exceeds the preview byte/line budget.
    pub oversized: bool,
    /// Initial open mode the viewer resolves to.
    pub open_mode: M5OpenMode,
    /// Open-mode token.
    pub open_mode_token: String,
    /// Cost of what is shown by default (always [`M5RenderCost::Cheap`]).
    pub default_render_cost: M5RenderCost,
    /// Default-render-cost token.
    pub default_render_cost_token: String,
    /// Banners shown above the preview, in declaration order.
    pub banners: Vec<M5LimitedModeBanner>,
    /// Action affordances offered.
    pub actions: Vec<M5LimitedModeAction>,
    /// Whether an expensive or unsafe render path exists and is guarded.
    pub expensive_render_guarded: bool,
    /// Whether raw inspection stays reachable (always true).
    pub raw_inspection_reachable: bool,
    /// Whether a raw copy path stays reachable (always true).
    pub raw_copy_reachable: bool,
    /// Whether the canonical-source/generator relationship is preserved.
    pub canonical_relationship_preserved: bool,
    /// Whether a visible trust/representation cue is present.
    pub visible_representation_cue: bool,
    /// Human-readable rationale for the resolved posture.
    pub rationale: String,
}

impl M5LimitedModeArtifactProjection {
    /// Whether this artifact opened in the bounded, limited-preview mode.
    pub fn is_limited(&self) -> bool {
        self.open_mode.is_limited()
    }

    /// The expand-full-render action, if one is offered.
    pub fn expand_action(&self) -> Option<&M5LimitedModeAction> {
        self.actions
            .iter()
            .find(|a| a.kind == M5LimitedModeActionKind::ExpandFullRender)
    }

    /// Whether an open-raw action is offered immediately and cheaply.
    pub fn has_immediate_open_raw(&self) -> bool {
        self.actions.iter().any(|a| {
            a.kind == M5LimitedModeActionKind::OpenRaw
                && a.posture == M5ActionPosture::AvailableImmediately
                && a.render_cost == M5RenderCost::Cheap
        })
    }

    /// Whether an open-canonical-source action is offered.
    pub fn has_open_canonical_source(&self) -> bool {
        self.actions
            .iter()
            .any(|a| a.kind == M5LimitedModeActionKind::OpenCanonicalSource)
    }
}

/// Limited-mode review block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LimitedModeReview {
    /// One shared content-integrity policy library governs every surface.
    pub one_shared_policy_library_governs_all_surfaces: bool,
    /// Oversized or generated artifacts open in safe-preview/limited mode first.
    pub oversized_or_generated_open_in_limited_mode_first: bool,
    /// No expensive or unsafe render path fires without explicit user action.
    pub no_silent_expensive_or_unsafe_render: bool,
    /// Expensive and unsafe render paths require an explicit opt-in.
    pub expensive_and_unsafe_render_require_explicit_opt_in: bool,
    /// Open-raw stays reachable immediately and cheaply on every artifact.
    pub open_raw_always_reachable: bool,
    /// The canonical-source or generator relationship is preserved.
    pub canonical_source_or_generator_relationship_preserved: bool,
    /// A visible trust/representation cue is shown in limited mode.
    pub visible_trust_or_representation_cue_in_limited_mode: bool,
    /// The default view is never an expensive or unsafe render.
    pub default_view_is_never_expensive_or_unsafe: bool,
    /// Suspicious bytes are surfaced, never normalized away.
    pub suspicious_bytes_never_normalized_away: bool,
    /// A generated artifact is never presented as the only truth.
    pub generated_artifact_not_presented_as_only_truth: bool,
}

impl M5LimitedModeReview {
    /// The frozen, all-invariants-hold review block.
    pub const fn frozen() -> Self {
        Self {
            one_shared_policy_library_governs_all_surfaces: true,
            oversized_or_generated_open_in_limited_mode_first: true,
            no_silent_expensive_or_unsafe_render: true,
            expensive_and_unsafe_render_require_explicit_opt_in: true,
            open_raw_always_reachable: true,
            canonical_source_or_generator_relationship_preserved: true,
            visible_trust_or_representation_cue_in_limited_mode: true,
            default_view_is_never_expensive_or_unsafe: true,
            suspicious_bytes_never_normalized_away: true,
            generated_artifact_not_presented_as_only_truth: true,
        }
    }

    fn all_hold(&self) -> bool {
        self.one_shared_policy_library_governs_all_surfaces
            && self.oversized_or_generated_open_in_limited_mode_first
            && self.no_silent_expensive_or_unsafe_render
            && self.expensive_and_unsafe_render_require_explicit_opt_in
            && self.open_raw_always_reachable
            && self.canonical_source_or_generator_relationship_preserved
            && self.visible_trust_or_representation_cue_in_limited_mode
            && self.default_view_is_never_expensive_or_unsafe
            && self.suspicious_bytes_never_normalized_away
            && self.generated_artifact_not_presented_as_only_truth
    }
}

/// Cross-family safe-preview limited-mode and expensive-render-guard packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SafePreviewLimitedModePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Number of artifacts that opened in limited mode.
    pub limited_mode_artifact_count: usize,
    /// Number of artifacts with a guarded expensive/unsafe render path.
    pub guarded_render_count: usize,
    /// Number of artifacts treated as oversized.
    pub oversized_count: usize,
    /// Number of generated/derived artifacts.
    pub generated_count: usize,
    /// Distinct open-mode tokens across artifacts.
    pub open_modes: Vec<String>,
    /// Byte budget above which an artifact opens in limited mode.
    pub byte_budget: u64,
    /// Line budget above which an artifact opens in limited mode.
    pub line_budget: u64,
    /// Whether projection normalized or stripped any source (always false).
    pub normalization_applied: bool,
    /// Per-artifact resolved projections.
    pub artifacts: Vec<M5LimitedModeArtifactProjection>,
    /// Limited-mode review block.
    pub review: M5LimitedModeReview,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SafePreviewLimitedModePacket {
    /// Returns true when every required family is present exactly once.
    pub fn covers_all_families(&self) -> bool {
        let present: BTreeSet<_> = self.artifacts.iter().map(|a| a.family).collect();
        M5LimitedModeArtifactFamily::ALL
            .iter()
            .all(|f| present.contains(f))
            && present.len() == M5LimitedModeArtifactFamily::ALL.len()
    }

    /// Returns true when every oversized or generated artifact opens in limited
    /// mode first.
    pub fn oversized_or_generated_open_limited(&self) -> bool {
        self.artifacts
            .iter()
            .all(|a| !(a.oversized || a.generated) || a.open_mode.is_limited())
    }

    /// Returns true when no expensive or unsafe action is available without an
    /// explicit opt-in.
    pub fn no_silent_expensive_render(&self) -> bool {
        self.artifacts.iter().all(|a| {
            a.actions
                .iter()
                .all(|action| !action.render_cost.requires_opt_in() || action.posture.is_gated())
        })
    }

    /// Returns true when the default view of every artifact is cheap.
    pub fn default_view_never_expensive(&self) -> bool {
        self.artifacts
            .iter()
            .all(|a| a.default_render_cost == M5RenderCost::Cheap)
    }

    /// Returns true when open-raw and raw copy stay reachable everywhere.
    pub fn raw_always_reachable(&self) -> bool {
        self.artifacts.iter().all(|a| {
            a.raw_inspection_reachable && a.raw_copy_reachable && a.has_immediate_open_raw()
        })
    }

    /// Returns true when every artifact preserves its canonical-source or
    /// generator relationship with a non-empty ref and an open-source action.
    pub fn canonical_relationship_preserved(&self) -> bool {
        self.artifacts.iter().all(|a| {
            a.canonical_relationship_preserved
                && !a.canonical_source_ref.trim().is_empty()
                && a.has_open_canonical_source()
        })
    }

    /// Returns true when every limited-mode artifact carries at least one banner
    /// as a visible cue.
    pub fn limited_mode_has_visible_cue(&self) -> bool {
        self.artifacts.iter().all(|a| {
            !a.open_mode.is_limited() || (a.visible_representation_cue && !a.banners.is_empty())
        })
    }

    /// Returns true when every guarded artifact offers a gated expand action.
    pub fn guarded_render_offers_gated_expand(&self) -> bool {
        self.artifacts.iter().all(|a| {
            if !a.expensive_render_guarded {
                return true;
            }
            a.expand_action()
                .is_some_and(|action| action.posture.is_gated())
        })
    }

    /// Validates the safe-preview limited-mode invariants.
    pub fn validate(&self) -> Vec<M5SafePreviewLimitedModeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SAFE_PREVIEW_LIMITED_RECORD_KIND {
            violations.push(M5SafePreviewLimitedModeViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SAFE_PREVIEW_LIMITED_SCHEMA_VERSION {
            violations.push(M5SafePreviewLimitedModeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(M5SafePreviewLimitedModeViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5SafePreviewLimitedModeViolation::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(M5SafePreviewLimitedModeViolation::NormalizationApplied);
        }
        if !self.covers_all_families() {
            violations.push(M5SafePreviewLimitedModeViolation::FamilyMissing);
        }
        if !self.oversized_or_generated_open_limited() {
            violations.push(M5SafePreviewLimitedModeViolation::OversizedOrGeneratedNotLimited);
        }
        if !self.no_silent_expensive_render() {
            violations.push(M5SafePreviewLimitedModeViolation::SilentExpensiveRender);
        }
        if !self.default_view_never_expensive() {
            violations.push(M5SafePreviewLimitedModeViolation::DefaultViewExpensive);
        }
        if !self.raw_always_reachable() {
            violations.push(M5SafePreviewLimitedModeViolation::RawInspectionUnreachable);
        }
        if !self.canonical_relationship_preserved() {
            violations.push(M5SafePreviewLimitedModeViolation::CanonicalRelationshipLost);
        }
        if !self.limited_mode_has_visible_cue() {
            violations.push(M5SafePreviewLimitedModeViolation::LimitedModeMissingCue);
        }
        if !self.guarded_render_offers_gated_expand() {
            violations.push(M5SafePreviewLimitedModeViolation::GuardedRenderMissingGate);
        }
        if self.limited_mode_artifact_count != self.declared_limited_count() {
            violations.push(M5SafePreviewLimitedModeViolation::LimitedCountMismatch);
        }
        if self.guarded_render_count != self.declared_guarded_count() {
            violations.push(M5SafePreviewLimitedModeViolation::GuardedCountMismatch);
        }
        for artifact in &self.artifacts {
            if artifact.actions.is_empty() {
                violations.push(M5SafePreviewLimitedModeViolation::ActionsMissing);
                break;
            }
            for banner in &artifact.banners {
                if banner.kind_token != banner.kind.as_str() || banner.message.trim().is_empty() {
                    violations.push(M5SafePreviewLimitedModeViolation::BannerMalformed);
                    break;
                }
            }
        }
        if !self.review.all_hold() {
            violations.push(M5SafePreviewLimitedModeViolation::ReviewIncomplete);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 safe-preview limited-mode packet serializes"),
        ) {
            violations.push(M5SafePreviewLimitedModeViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    fn declared_limited_count(&self) -> usize {
        self.artifacts.iter().filter(|a| a.is_limited()).count()
    }

    fn declared_guarded_count(&self) -> usize {
        self.artifacts
            .iter()
            .filter(|a| a.expensive_render_guarded)
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 safe-preview limited-mode packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Safe-Preview Limited Mode & Expensive-Render Guards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Limited-mode artifacts: {} of {}\n",
            self.limited_mode_artifact_count,
            self.artifacts.len()
        ));
        out.push_str(&format!(
            "- Oversized: {} · generated: {} · guarded render: {}\n",
            self.oversized_count, self.generated_count, self.guarded_render_count
        ));
        out.push_str(&format!(
            "- Budgets: {} bytes / {} lines\n",
            self.byte_budget, self.line_budget
        ));
        out.push_str("\n## Artifacts\n\n");
        for artifact in &self.artifacts {
            out.push_str(&format!(
                "- **{}** (`{}`): {} bytes / {} lines → open `{}`\n",
                artifact.family.as_str(),
                artifact.subject_ref,
                artifact.byte_size,
                artifact.line_count,
                artifact.open_mode.as_str(),
            ));
            out.push_str(&format!(
                "  - Canonical source: `{}` · generated: {} · guarded: {}\n",
                artifact.canonical_source_ref,
                artifact.generated,
                artifact.expensive_render_guarded,
            ));
            if !artifact.banners.is_empty() {
                let banners = artifact
                    .banners
                    .iter()
                    .map(|b| b.kind.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("  - Banners: {banners}\n"));
            }
            let actions = artifact
                .actions
                .iter()
                .map(|a| format!("{} ({})", a.kind.as_str(), a.posture.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Actions: {actions}\n"));
        }
        out
    }
}

/// Validation failures emitted by [`M5SafePreviewLimitedModePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SafePreviewLimitedModeViolation {
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
    /// A required family is missing.
    FamilyMissing,
    /// An oversized or generated artifact did not open in limited mode.
    OversizedOrGeneratedNotLimited,
    /// An expensive or unsafe action is available without an explicit opt-in.
    SilentExpensiveRender,
    /// The default view of some artifact is an expensive or unsafe render.
    DefaultViewExpensive,
    /// Open-raw or raw copy is unreachable on some artifact.
    RawInspectionUnreachable,
    /// An artifact lost its canonical-source/generator relationship.
    CanonicalRelationshipLost,
    /// A limited-mode artifact carries no visible banner cue.
    LimitedModeMissingCue,
    /// A guarded artifact offers no gated expand action.
    GuardedRenderMissingGate,
    /// The declared limited-mode count does not match the projections.
    LimitedCountMismatch,
    /// The declared guarded-render count does not match the projections.
    GuardedCountMismatch,
    /// An artifact offers no actions.
    ActionsMissing,
    /// A banner is malformed (token mismatch or empty message).
    BannerMalformed,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5SafePreviewLimitedModeViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::FamilyMissing => "family_missing",
            Self::OversizedOrGeneratedNotLimited => "oversized_or_generated_not_limited",
            Self::SilentExpensiveRender => "silent_expensive_render",
            Self::DefaultViewExpensive => "default_view_expensive",
            Self::RawInspectionUnreachable => "raw_inspection_unreachable",
            Self::CanonicalRelationshipLost => "canonical_relationship_lost",
            Self::LimitedModeMissingCue => "limited_mode_missing_cue",
            Self::GuardedRenderMissingGate => "guarded_render_missing_gate",
            Self::LimitedCountMismatch => "limited_count_mismatch",
            Self::GuardedCountMismatch => "guarded_count_mismatch",
            Self::ActionsMissing => "actions_missing",
            Self::BannerMalformed => "banner_malformed",
            Self::ReviewIncomplete => "review_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in safe-preview limited-mode export.
#[derive(Debug)]
pub enum M5SafePreviewLimitedModeExportError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SafePreviewLimitedModeViolation>),
}

impl std::fmt::Display for M5SafePreviewLimitedModeExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 safe-preview limited-mode export parse failed: {error}"
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
                    "m5 safe-preview limited-mode export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5SafePreviewLimitedModeExportError {}

/// Resolves a single artifact's limited-mode posture from its size, derivation,
/// and render cost.
fn resolve_artifact(input: &M5LimitedModeArtifactInput<'_>) -> M5LimitedModeArtifactProjection {
    let family = input.family;
    let oversized = input.byte_size > M5_SAFE_PREVIEW_BYTE_BUDGET
        || input.line_count > M5_SAFE_PREVIEW_LINE_BUDGET;

    let full_render_cost = if input.active_content_present {
        M5RenderCost::Unsafe
    } else if input.full_render_is_expensive {
        M5RenderCost::Expensive
    } else {
        M5RenderCost::Cheap
    };

    let needs_limited = oversized || input.generated || full_render_cost.requires_opt_in();
    let open_mode = if needs_limited {
        M5OpenMode::SafePreviewLimited
    } else {
        M5OpenMode::FullRenderInline
    };
    let expensive_render_guarded = full_render_cost.requires_opt_in();

    let mut banners = Vec::new();
    if oversized {
        banners.push(banner(
            M5LimitedModeBannerKind::Oversized,
            format!(
                "This {} is {} bytes / {} lines, above the {}-byte preview budget; only a bounded preview is shown.",
                family.as_str(),
                input.byte_size,
                input.line_count,
                M5_SAFE_PREVIEW_BYTE_BUDGET
            ),
        ));
    }
    if input.generated {
        banners.push(banner(
            M5LimitedModeBannerKind::GeneratedArtifact,
            format!(
                "This {} is a generated artifact derived from its {} (`{}`); it is not the canonical source of truth.",
                family.as_str(),
                family.canonical_source_kind(),
                input.canonical_source_ref
            ),
        ));
    }
    if open_mode.is_limited() {
        banners.push(banner(
            M5LimitedModeBannerKind::LimitedPreview,
            "Showing a limited safe preview; open raw or expand to see the full artifact."
                .to_owned(),
        ));
    }
    if full_render_cost == M5RenderCost::Expensive {
        banners.push(banner(
            M5LimitedModeBannerKind::ExpensiveRenderGuarded,
            "The full render is expensive and runs only after you expand it.".to_owned(),
        ));
    }
    if full_render_cost == M5RenderCost::Unsafe {
        banners.push(banner(
            M5LimitedModeBannerKind::ActiveContentGuarded,
            "This artifact carries active content; it renders only after you explicitly opt in."
                .to_owned(),
        ));
    }

    let mut actions = vec![
        M5LimitedModeAction {
            kind: M5LimitedModeActionKind::OpenRaw,
            kind_token: M5LimitedModeActionKind::OpenRaw.as_str().to_owned(),
            posture: M5ActionPosture::AvailableImmediately,
            posture_token: M5ActionPosture::AvailableImmediately.as_str().to_owned(),
            render_cost: M5RenderCost::Cheap,
            render_cost_token: M5RenderCost::Cheap.as_str().to_owned(),
            target_ref: input.subject_ref.to_owned(),
            label: "Open raw".to_owned(),
        },
        M5LimitedModeAction {
            kind: M5LimitedModeActionKind::OpenCanonicalSource,
            kind_token: M5LimitedModeActionKind::OpenCanonicalSource
                .as_str()
                .to_owned(),
            posture: M5ActionPosture::AvailableImmediately,
            posture_token: M5ActionPosture::AvailableImmediately.as_str().to_owned(),
            render_cost: M5RenderCost::Cheap,
            render_cost_token: M5RenderCost::Cheap.as_str().to_owned(),
            target_ref: input.canonical_source_ref.to_owned(),
            label: format!("Open {}", family.canonical_source_kind()),
        },
    ];

    if open_mode.is_limited() {
        let posture = if full_render_cost.requires_opt_in() {
            M5ActionPosture::RequiresExplicitOptIn
        } else {
            M5ActionPosture::AvailableImmediately
        };
        actions.push(M5LimitedModeAction {
            kind: M5LimitedModeActionKind::ExpandFullRender,
            kind_token: M5LimitedModeActionKind::ExpandFullRender
                .as_str()
                .to_owned(),
            posture,
            posture_token: posture.as_str().to_owned(),
            render_cost: full_render_cost,
            render_cost_token: full_render_cost.as_str().to_owned(),
            target_ref: input.subject_ref.to_owned(),
            label: "Expand full render".to_owned(),
        });
    }

    let rationale = build_rationale(
        family,
        oversized,
        input.generated,
        full_render_cost,
        open_mode,
    );

    M5LimitedModeArtifactProjection {
        family,
        family_token: family.as_str().to_owned(),
        subject_ref: input.subject_ref.to_owned(),
        canonical_source_ref: input.canonical_source_ref.to_owned(),
        byte_size: input.byte_size,
        line_count: input.line_count,
        generated: input.generated,
        oversized,
        open_mode,
        open_mode_token: open_mode.as_str().to_owned(),
        default_render_cost: M5RenderCost::Cheap,
        default_render_cost_token: M5RenderCost::Cheap.as_str().to_owned(),
        banners,
        actions,
        expensive_render_guarded,
        raw_inspection_reachable: true,
        raw_copy_reachable: true,
        canonical_relationship_preserved: true,
        visible_representation_cue: true,
        rationale,
    }
}

fn banner(kind: M5LimitedModeBannerKind, message: String) -> M5LimitedModeBanner {
    M5LimitedModeBanner {
        kind,
        kind_token: kind.as_str().to_owned(),
        message,
    }
}

fn build_rationale(
    family: M5LimitedModeArtifactFamily,
    oversized: bool,
    generated: bool,
    full_render_cost: M5RenderCost,
    open_mode: M5OpenMode,
) -> String {
    if !open_mode.is_limited() {
        return format!(
            "{} is small, inert, and cheap to render, so it opens fully inline; open-raw and open-source stay reachable.",
            family.as_str()
        );
    }
    let mut reasons = Vec::new();
    if oversized {
        reasons.push("it exceeds the preview budget");
    }
    if generated {
        reasons.push("it is a generated artifact derived from a canonical source");
    }
    match full_render_cost {
        M5RenderCost::Expensive => reasons.push("its full render is expensive"),
        M5RenderCost::Unsafe => reasons.push("it carries active content"),
        M5RenderCost::Cheap => {}
    }
    format!(
        "{} opens in safe-preview limited mode because {}; the full render is an explicit opt-in.",
        family.as_str(),
        reasons.join(", ")
    )
}

fn distinct_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<String> {
    let set: BTreeSet<&str> = tokens.collect();
    set.into_iter().map(str::to_owned).collect()
}

/// Projects the safe-preview limited-mode resolution, banners, open-raw/
/// open-source actions, and expensive-render guards across every new M5 large
/// or generated artifact family.
pub fn project_m5_safe_preview_limited_mode(
    seed: &M5LimitedModeSeed<'_>,
) -> M5SafePreviewLimitedModePacket {
    let artifacts: Vec<_> = seed.artifact_inputs.iter().map(resolve_artifact).collect();

    let limited_mode_artifact_count = artifacts.iter().filter(|a| a.is_limited()).count();
    let guarded_render_count = artifacts
        .iter()
        .filter(|a| a.expensive_render_guarded)
        .count();
    let oversized_count = artifacts.iter().filter(|a| a.oversized).count();
    let generated_count = artifacts.iter().filter(|a| a.generated).count();
    let open_modes = distinct_tokens(artifacts.iter().map(|a| a.open_mode.as_str()));

    M5SafePreviewLimitedModePacket {
        record_kind: M5_SAFE_PREVIEW_LIMITED_RECORD_KIND.to_owned(),
        schema_version: M5_SAFE_PREVIEW_LIMITED_SCHEMA_VERSION,
        packet_id: M5_SAFE_PREVIEW_LIMITED_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        limited_mode_artifact_count,
        guarded_render_count,
        oversized_count,
        generated_count,
        open_modes,
        byte_budget: M5_SAFE_PREVIEW_BYTE_BUDGET,
        line_budget: M5_SAFE_PREVIEW_LINE_BUDGET,
        normalization_applied: false,
        artifacts,
        review: M5LimitedModeReview::frozen(),
        source_contract_refs: vec![
            M5_SAFE_PREVIEW_LIMITED_SCHEMA_REF.to_owned(),
            M5_SAFE_PREVIEW_LIMITED_DOC_REF.to_owned(),
            M5_SAFE_PREVIEW_LIMITED_TRUST_CLASS_CONTRACT_REF.to_owned(),
            M5_SAFE_PREVIEW_LIMITED_RAW_RENDERED_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen safe-preview limited-mode packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_SAFE_PREVIEW_LIMITED_ARTIFACT_REF`]; the bin emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged. The
/// scenario exercises every banner kind and every expand posture — oversized,
/// generated, expensive, unsafe, and cheap-expand.
pub fn frozen_m5_safe_preview_limited_mode_packet() -> M5SafePreviewLimitedModePacket {
    use M5LimitedModeArtifactFamily as Family;

    let artifact_inputs = [
        // Oversized raw log capture with an expensive full highlight.
        M5LimitedModeArtifactInput {
            family: Family::BuildLog,
            subject_ref: "pipeline:run:128:log:build",
            canonical_source_ref: "pipeline:run:128",
            byte_size: 2_400_000,
            line_count: 84_000,
            generated: false,
            full_render_is_expensive: true,
            active_content_present: false,
        },
        // Generated lockfile with an expensive resolve-graph render.
        M5LimitedModeArtifactInput {
            family: Family::DependencyLockfile,
            subject_ref: "workspace:lockfile:Cargo.lock",
            canonical_source_ref: "workspace:manifest:Cargo.toml",
            byte_size: 61_000,
            line_count: 1_200,
            generated: true,
            full_render_is_expensive: true,
            active_content_present: false,
        },
        // Small generated snapshot whose full render is cheap (cheap-expand case).
        M5LimitedModeArtifactInput {
            family: Family::TestSnapshot,
            subject_ref: "test:snapshot:ui:home:1",
            canonical_source_ref: "test:case:ui_home_renders",
            byte_size: 12_000,
            line_count: 300,
            generated: true,
            full_render_is_expensive: false,
            active_content_present: false,
        },
        // Oversized generated bundle carrying active content (unsafe expand).
        M5LimitedModeArtifactInput {
            family: Family::DistributionBundle,
            subject_ref: "release:bundle:app@2.0.0",
            canonical_source_ref: "build:inputs:app@2.0.0",
            byte_size: 52_000_000,
            line_count: 1,
            generated: true,
            full_render_is_expensive: true,
            active_content_present: true,
        },
        // Generated evidence packet with an expensive structured render.
        M5LimitedModeArtifactInput {
            family: Family::EvidencePacket,
            subject_ref: "incident:evidence:packet:42",
            canonical_source_ref: "incident:42:underlying-records",
            byte_size: 92_000,
            line_count: 2_100,
            generated: true,
            full_render_is_expensive: true,
            active_content_present: false,
        },
        // Small generated artifact whose full render is cheap (cheap-expand case).
        M5LimitedModeArtifactInput {
            family: Family::GeneratedArtifact,
            subject_ref: "codegen:output:api_client.rs",
            canonical_source_ref: "codegen:spec:openapi.yaml",
            byte_size: 8_400,
            line_count: 210,
            generated: true,
            full_render_is_expensive: false,
            active_content_present: false,
        },
    ];

    project_m5_safe_preview_limited_mode(&M5LimitedModeSeed {
        case_id: "case:m5-safe-preview-limited-mode:stable",
        artifact_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in safe-preview limited-mode support export.
pub fn current_m5_safe_preview_limited_mode_export(
) -> Result<M5SafePreviewLimitedModePacket, M5SafePreviewLimitedModeExportError> {
    let packet: M5SafePreviewLimitedModePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_safe_preview_limited_mode/support_export.json"
    )))
    .map_err(M5SafePreviewLimitedModeExportError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SafePreviewLimitedModeExportError::Validation(violations))
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
