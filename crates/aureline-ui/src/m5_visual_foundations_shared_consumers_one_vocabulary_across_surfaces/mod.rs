//! Shared editor / review / search / data / notebook / docs-help / settings / export-support
//! consumers that keep the B136 visual-foundation families — the color system, semantic theme
//! tokens, syntax / diff / chart tokens, typography, spacing / sizing / radii / elevation
//! geometry, and minimum hit-target baselines — at **one vocabulary** across every claimed M5
//! surface.
//!
//! This module is the closing consumer-adoption lane for the eight reusable visual-foundation
//! families frozen in [`crate::m5_visual_foundation_matrix`] and implemented by the color / theme
//! lane ([`crate::m5_color_system_and_semantic_theme_token_registries`]), the syntax / diff /
//! chart lane ([`crate::m5_syntax_diff_and_chart_token_registries`]), the typography lane
//! ([`crate::m5_typography_scale_font_stack_and_overflow_registries`]), and the geometry /
//! hit-target lane
//! ([`crate::m5_spacing_sizing_radii_elevation_and_hit_target_registries`]).
//!
//! It binds each shared foundation family to the concrete shell, editor, review, data, docs,
//! settings, CLI/export, and support-export consumers that render it, and proves — by fixtures,
//! not screenshots — that the same foundation object presents the same semantic-role, family,
//! token-reference, theme-variant, density-context, and non-color-cue vocabulary wherever it
//! appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the eight shared foundation families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared visual infrastructure rather than a
//!    one-surface, feature-local fork of color or geometry meaning.
//! 2. **One vocabulary / no drift.** For a given foundation object every consumer surface must
//!    present identical [`VisualFoundationStateFacetValues`] — the same semantic-role word, the
//!    same family word, the same token-reference word, the same theme-variant word, the same
//!    density-context word, and the same non-color-cue word. The semantic-role word must be a
//!    token from the frozen [`M5VisualSemanticRole`] vocabulary, so no feature rewrites `brand`,
//!    `interactive`, `neutral`, `status`, `syntax`, `diff`, or `chart` in its own words. A
//!    surface may narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying vocabulary per surface, and a role
//!    that carries status or data meaning may never fall back to hue alone.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
//!    per-domain schema and the frozen matrix by id, so an exported packet can always map a
//!    shell / editor / review / data / docs visual surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`VisualFoundationNarrowNote`] naming the reason, the preserved vocabulary, and the
//! next action, and an exported representation additionally names its export-safe detail boundary
//! rather than collapsing the object out of view.
//!
//! The packet references upstream foundation contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/design-system/m5-visual-foundations-shared-consumers.schema.json`](../../../../schemas/design-system/m5-visual-foundations-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_visual_foundations_shared_consumers_one_vocabulary.md`](../../../../docs/design-system/m5_visual_foundations_shared_consumers_one_vocabulary.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-visual-foundations-shared-consumers/`](../../../../fixtures/ui/m5-visual-foundations-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_visual_foundations_shared_consumers,
    seeded_m5_visual_foundations_shared_consumers_compact_remote_narrowed,
    seeded_m5_visual_foundations_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_visual_foundation_matrix::{
    M5VisualFoundationConsumerSurface, M5VisualFoundationFamily, M5VisualSemanticRole,
    M5_VISUAL_FOUNDATION_MATRIX_DOC_REF, M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5VisualFoundationSharedConsumersPacket`].
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_visual_foundations_shared_consumer_vocabulary_parity";

/// Schema version for visual-foundation shared-consumer parity records.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-visual-foundations-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/design-system/m5-visual-foundations-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/design-system/m5_visual_foundations_shared_consumers_one_vocabulary.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-foundations-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-visual-foundations-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-visual-foundations-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-foundations-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Non-color-cue sentinel words a status / syntax / diff / chart role may never fall back to; a
/// role that carries status or data meaning must always pair its color with one of these cues,
/// never rely on hue alone.
const HUE_ALONE_SENTINELS: [&str; 4] = ["none", "hue_alone", "color_only", "color_alone"];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5VisualFoundationConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5VisualFoundationConsumerSurface::SupportExport
            | M5VisualFoundationConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5VisualSemanticRole`] vocabulary.
///
/// This is the "one vocabulary" gate: a foundation object's semantic-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_semantic_role_token(token: &str) -> bool {
    semantic_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5VisualSemanticRole`], if it is one.
pub fn semantic_role_from_token(token: &str) -> Option<M5VisualSemanticRole> {
    M5VisualSemanticRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared foundation family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying vocabulary: a narrowed
/// representation still carries the same semantic-role, family, token-reference, theme-variant,
/// density-context, and non-color-cue words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl VisualFoundationRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A vocabulary axis whose word must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationParityFacet {
    /// The frozen semantic-role word.
    SemanticRoleWord,
    /// The foundation-family word.
    FamilyWord,
    /// The canonical token-reference word the family points at.
    TokenReferenceWord,
    /// The theme-variant word (dark / light / high-contrast coverage).
    ThemeVariantWord,
    /// The density-context word.
    DensityContextWord,
    /// The non-color-cue word paired with a color / data role.
    NonColorCueWord,
}

impl VisualFoundationParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SemanticRoleWord,
        Self::FamilyWord,
        Self::TokenReferenceWord,
        Self::ThemeVariantWord,
        Self::DensityContextWord,
        Self::NonColorCueWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticRoleWord => "semantic_role_word",
            Self::FamilyWord => "family_word",
            Self::TokenReferenceWord => "token_reference_word",
            Self::ThemeVariantWord => "theme_variant_word",
            Self::DensityContextWord => "density_context_word",
            Self::NonColorCueWord => "non_color_cue_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared foundation family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl VisualFoundationNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl VisualFoundationNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationParityState {
    /// All vocabulary is preserved and shown in full.
    FacetsPreserved,
    /// All vocabulary is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl VisualFoundationParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualFoundationSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Foundation vocabulary drifted between surfaces for the same object.
    VocabularyDriftDetected,
    /// A status or trust role relied on hue alone to carry meaning.
    HueAloneUsedForMeaning,
    /// A syntax or diff palette collided with the diagnostics palette.
    SyntaxOrDiffCollidedWithDiagnostics,
    /// A hit target shrank below its supported minimum.
    HitTargetShrunkBelowMinimum,
    /// Chart meaning depended on color alone.
    ChartMeaningDependedOnColorAlone,
    /// A feature-local spacing / elevation fork drifted from the shared geometry.
    LocalGeometryForkedFromFoundation,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalReferenceMissing,
    /// An upstream shared foundation family narrowed.
    UpstreamFoundationNarrowed,
}

impl VisualFoundationSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::VocabularyDriftDetected,
        Self::HueAloneUsedForMeaning,
        Self::SyntaxOrDiffCollidedWithDiagnostics,
        Self::HitTargetShrunkBelowMinimum,
        Self::ChartMeaningDependedOnColorAlone,
        Self::LocalGeometryForkedFromFoundation,
        Self::CanonicalReferenceMissing,
        Self::UpstreamFoundationNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::VocabularyDriftDetected => "vocabulary_drift_detected",
            Self::HueAloneUsedForMeaning => "hue_alone_used_for_meaning",
            Self::SyntaxOrDiffCollidedWithDiagnostics => "syntax_or_diff_collided_with_diagnostics",
            Self::HitTargetShrunkBelowMinimum => "hit_target_shrunk_below_minimum",
            Self::ChartMeaningDependedOnColorAlone => "chart_meaning_depended_on_color_alone",
            Self::LocalGeometryForkedFromFoundation => "local_geometry_forked_from_foundation",
            Self::CanonicalReferenceMissing => "canonical_reference_missing",
            Self::UpstreamFoundationNarrowed => "upstream_foundation_narrowed",
        }
    }
}

/// The controlled vocabulary a foundation object presents.
///
/// These six words must be identical across every consumer surface that shows the same foundation
/// object. The semantic-role word must be a frozen role token; the rest are controlled words the
/// object's family carries. A surface may narrow how much it renders, but it may never reword any
/// of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationStateFacetValues {
    /// Semantic-role word (must be a frozen [`M5VisualSemanticRole`] token).
    pub semantic_role_word: String,
    /// Foundation-family word.
    pub family_word: String,
    /// Canonical token-reference word the family points at.
    pub token_reference_word: String,
    /// Theme-variant word (dark / light / high-contrast coverage).
    pub theme_variant_word: String,
    /// Density-context word.
    pub density_context_word: String,
    /// Non-color-cue word paired with a color / data role.
    pub non_color_cue_word: String,
}

impl VisualFoundationStateFacetValues {
    /// Whether every vocabulary word is present.
    pub fn all_present(&self) -> bool {
        !self.semantic_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.token_reference_word.trim().is_empty()
            && !self.theme_variant_word.trim().is_empty()
            && !self.density_context_word.trim().is_empty()
            && !self.non_color_cue_word.trim().is_empty()
    }

    /// Whether the semantic-role word is a member of the frozen role vocabulary.
    pub fn semantic_role_word_in_vocabulary(&self) -> bool {
        is_known_semantic_role_token(self.semantic_role_word.trim())
    }

    /// Whether the object honours the never-hue-alone rule: a role that carries status or data
    /// meaning (`status`, `syntax`, `diff`, `chart`) must pair color with a real non-color cue and
    /// never fall back to a hue-alone sentinel.
    pub fn non_color_cue_satisfied(&self) -> bool {
        match semantic_role_from_token(self.semantic_role_word.trim()) {
            Some(role) if role.demands_non_color_cue() => {
                let cue = self.non_color_cue_word.trim().to_lowercase();
                !cue.is_empty() && !HUE_ALONE_SENTINELS.contains(&cue.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationNarrowNote {
    /// Why the representation narrowed.
    pub reason: VisualFoundationNarrowReason,
    /// Note naming the preserved vocabulary (never omitted).
    pub preserved_vocabulary_note: String,
    /// The next action offered.
    pub next_action: VisualFoundationNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualFoundationRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: VisualFoundationParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<VisualFoundationNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<VisualFoundationNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows
/// disclosure depth, a remote-projected representation names its remote source, and an exported
/// representation names its export-safe-detail boundary — but all three keep every vocabulary word
/// and disclose the narrowing through an explicit note.
pub const fn resolve_visual_foundation_render_disclosure(
    representation: VisualFoundationRepresentation,
) -> VisualFoundationRenderDisclosure {
    match representation {
        VisualFoundationRepresentation::DesktopFull => VisualFoundationRenderDisclosure {
            parity_state: VisualFoundationParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        VisualFoundationRepresentation::CompactNarrowed => VisualFoundationRenderDisclosure {
            parity_state: VisualFoundationParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualFoundationNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(VisualFoundationNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        VisualFoundationRepresentation::RemoteProjected => VisualFoundationRenderDisclosure {
            parity_state: VisualFoundationParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualFoundationNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(VisualFoundationNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        VisualFoundationRepresentation::ExportedRedacted => VisualFoundationRenderDisclosure {
            parity_state: VisualFoundationParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualFoundationNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(VisualFoundationNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared foundation family rendered on one consumer surface in one
/// representation for one foundation object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable foundation-object id (shared across surfaces that show the same object).
    pub foundation_object_id: String,
    /// Human-readable foundation-object identity.
    pub foundation_object_label: String,
    /// Which shared foundation family this binding renders.
    pub family: M5VisualFoundationFamily,
    /// Which consumer surface renders it.
    pub consumer: M5VisualFoundationConsumerSurface,
    /// Which representation this surface renders.
    pub representation: VisualFoundationRepresentation,
    /// The controlled vocabulary presented (identical across surfaces for one object).
    pub state_facets: VisualFoundationStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: VisualFoundationParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<VisualFoundationNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface relies on hue alone to carry status / trust meaning. MUST be
    /// `false`.
    pub relies_on_hue_alone_for_meaning: bool,
    /// Guardrail: this surface lets a syntax or diff palette collide with diagnostics. MUST be
    /// `false`.
    pub lets_syntax_or_diff_palette_collide_with_diagnostics: bool,
    /// Guardrail: this surface shrinks a hit target below its supported minimum. MUST be `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Guardrail: this surface lets chart meaning depend on color alone. MUST be `false`.
    pub lets_chart_meaning_depend_on_color_alone: bool,
    /// Guardrail: this surface forks local spacing or elevation from the shared geometry. MUST be
    /// `false`.
    pub forks_local_spacing_or_elevation_from_shared_geometry: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl VisualFoundationConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> VisualFoundationRenderDisclosure {
        resolve_visual_foundation_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.relies_on_hue_alone_for_meaning
            && !self.lets_syntax_or_diff_palette_collide_with_diagnostics
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.lets_chart_meaning_depend_on_color_alone
            && !self.forks_local_spacing_or_elevation_from_shared_geometry
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.family.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same foundation object presents the same vocabulary across surfaces.
    pub same_object_same_vocabulary_across_surfaces: bool,
    /// Every semantic-role word is a frozen role token.
    pub semantic_role_words_stay_in_frozen_vocabulary: bool,
    /// Status and trust meaning never relies on hue alone.
    pub meaning_never_relies_on_hue_alone: bool,
    /// Syntax and diff palettes never collide with diagnostics.
    pub syntax_diff_never_collide_with_diagnostics: bool,
    /// Chart meaning never depends on color alone.
    pub chart_meaning_never_depends_on_color_alone: bool,
    /// Hit targets never shrink below their supported minimum.
    pub hit_targets_never_shrink_below_minimum: bool,
    /// No surface forks local geometry from the shared foundation.
    pub geometry_never_forks_from_shared_foundation: bool,
    /// Typography and geometry stay density-aware.
    pub typography_and_geometry_stay_density_aware: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl VisualFoundationSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_object_same_vocabulary_across_surfaces
            && self.semantic_role_words_stay_in_frozen_vocabulary
            && self.meaning_never_relies_on_hue_alone
            && self.syntax_diff_never_collide_with_diagnostics
            && self.chart_meaning_never_depends_on_color_alone
            && self.hit_targets_never_shrink_below_minimum
            && self.geometry_never_forks_from_shared_foundation
            && self.typography_and_geometry_stay_density_aware
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationSharedConsumersProjection {
    /// The shell UI consumes the shared visual foundation.
    pub shell_ui_consumes_shared_foundation: bool,
    /// The editor UI consumes the shared visual foundation.
    pub editor_ui_consumes_shared_foundation: bool,
    /// The review UI consumes the shared visual foundation.
    pub review_ui_consumes_shared_foundation: bool,
    /// The data UI consumes the shared visual foundation.
    pub data_ui_consumes_shared_foundation: bool,
    /// The docs UI consumes the shared visual foundation.
    pub docs_ui_consumes_shared_foundation: bool,
    /// The settings UI consumes the shared visual foundation.
    pub settings_ui_consumes_shared_foundation: bool,
    /// The support / export path consumes the shared visual foundation.
    pub support_export_consumes_shared_foundation: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Vocabulary is identical for the same foundation object.
    pub vocabulary_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_foundation_family: bool,
}

impl VisualFoundationSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_ui_consumes_shared_foundation
            && self.editor_ui_consumes_shared_foundation
            && self.review_ui_consumes_shared_foundation
            && self.data_ui_consumes_shared_foundation
            && self.docs_ui_consumes_shared_foundation
            && self.settings_ui_consumes_shared_foundation
            && self.support_export_consumes_shared_foundation
            && self.every_family_adopted_by_two_or_more_consumers
            && self.vocabulary_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_foundation_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFoundationSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5VisualFoundationSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5VisualFoundationSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<VisualFoundationConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<VisualFoundationSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5VisualFoundationConsumerSurface>,
    /// Trust review block.
    pub trust_review: VisualFoundationSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: VisualFoundationSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: VisualFoundationSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe visual-foundation shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationSharedConsumersPacket {
    /// Record kind; must equal [`M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<VisualFoundationConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<VisualFoundationSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5VisualFoundationConsumerSurface>,
    /// Trust review block.
    pub trust_review: VisualFoundationSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: VisualFoundationSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: VisualFoundationSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5VisualFoundationSharedConsumersPacket {
    /// Builds a visual-foundation shared-consumer packet from stable-lane input.
    pub fn new(input: M5VisualFoundationSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the visual-foundation shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5VisualFoundationSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5VisualFoundationSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5VisualFoundationSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5VisualFoundationSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5VisualFoundationSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5VisualFoundationSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5VisualFoundationSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5VisualFoundationSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5VisualFoundationSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("visual-foundation shared-consumer packet serializes"),
        ) {
            violations
                .push(M5VisualFoundationSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("visual-foundation shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("family,consumer,representation,semantic_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.semantic_role_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Shared Visual-Foundation Consumers: One Vocabulary Across Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: family `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.foundation_object_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.semantic_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in visual-foundation shared-consumer export.
#[derive(Debug)]
pub enum M5VisualFoundationSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5VisualFoundationSharedConsumersViolation>),
}

impl fmt::Display for M5VisualFoundationSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "visual-foundation shared-consumer export parse failed: {error}"
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
                    "visual-foundation shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5VisualFoundationSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5VisualFoundationSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5VisualFoundationSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's vocabulary values are incomplete.
    VocabularyFacetIncomplete,
    /// A binding's semantic-role word is not a frozen role token.
    SemanticRoleWordOutsideVocabulary,
    /// A binding's status / data role fell back to hue alone.
    NonColorCueMissingForColorMeaningRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same foundation object with different vocabulary.
    VocabularyDriftAcrossSurfaces,
    /// A shared family is not adopted by at least two distinct consumers.
    FamilyReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-vocabulary note.
    NarrowNotePreservedVocabularyMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding relies on hue alone for meaning.
    HueAloneForMeaning,
    /// A binding lets a syntax or diff palette collide with diagnostics.
    SyntaxOrDiffCollidesWithDiagnostics,
    /// A binding shrinks a hit target below its supported minimum.
    HitTargetShrunkBelowMinimum,
    /// A binding lets chart meaning depend on color alone.
    ChartMeaningDependsOnColorAlone,
    /// A binding forks local spacing or elevation from the shared geometry.
    LocalGeometryForkedFromFoundation,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared family appears among the bindings.
    FamilyCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5VisualFoundationSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::VocabularyFacetIncomplete => "vocabulary_facet_incomplete",
            Self::SemanticRoleWordOutsideVocabulary => "semantic_role_word_outside_vocabulary",
            Self::NonColorCueMissingForColorMeaningRole => {
                "non_color_cue_missing_for_color_meaning_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::VocabularyDriftAcrossSurfaces => "vocabulary_drift_across_surfaces",
            Self::FamilyReuseUnproven => "family_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedVocabularyMissing => {
                "narrow_note_preserved_vocabulary_missing"
            }
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::HueAloneForMeaning => "hue_alone_for_meaning",
            Self::SyntaxOrDiffCollidesWithDiagnostics => "syntax_or_diff_collides_with_diagnostics",
            Self::HitTargetShrunkBelowMinimum => "hit_target_shrunk_below_minimum",
            Self::ChartMeaningDependsOnColorAlone => "chart_meaning_depends_on_color_alone",
            Self::LocalGeometryForkedFromFoundation => "local_geometry_forked_from_foundation",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::FamilyCoverageMissing => "family_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable visual-foundation shared-consumer export.
pub fn current_stable_m5_visual_foundations_shared_consumers_export(
) -> Result<M5VisualFoundationSharedConsumersPacket, M5VisualFoundationSharedConsumersArtifactError>
{
    let packet: M5VisualFoundationSharedConsumersPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-visual-foundations-shared-consumers-proof/support_export.json"
        )
    ))
    .map_err(M5VisualFoundationSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5VisualFoundationSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5VisualFoundationSharedConsumersPacket,
    violations: &mut Vec<M5VisualFoundationSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_SCHEMA_REF,
        M5_VISUAL_FOUNDATIONS_SHARED_CONSUMERS_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
    ];
    // The eight families map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5VisualFoundationFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5VisualFoundationSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5VisualFoundationSharedConsumersPacket,
    violations: &mut Vec<M5VisualFoundationSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5VisualFoundationSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One vocabulary: the facet values must be identical for every binding that renders the same
    // foundation object.
    let mut object_facets: BTreeMap<&str, &VisualFoundationStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5VisualFoundationFamily,
        BTreeSet<M5VisualFoundationConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5VisualFoundationConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5VisualFoundationFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.foundation_object_id.trim().is_empty()
            || binding.foundation_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5VisualFoundationSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5VisualFoundationSharedConsumersViolation::VocabularyFacetIncomplete);
        }
        if !binding.state_facets.semantic_role_word_in_vocabulary() {
            violations.push(
                M5VisualFoundationSharedConsumersViolation::SemanticRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.non_color_cue_satisfied() {
            violations.push(
                M5VisualFoundationSharedConsumersViolation::NonColorCueMissingForColorMeaningRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5VisualFoundationSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5VisualFoundationSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5VisualFoundationSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5VisualFoundationSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_vocabulary_note.trim().is_empty() {
                        violations.push(
                            M5VisualFoundationSharedConsumersViolation::NarrowNotePreservedVocabularyMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5VisualFoundationSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5VisualFoundationSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5VisualFoundationSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5VisualFoundationSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.relies_on_hue_alone_for_meaning {
            violations.push(M5VisualFoundationSharedConsumersViolation::HueAloneForMeaning);
        }
        if binding.lets_syntax_or_diff_palette_collide_with_diagnostics {
            violations.push(
                M5VisualFoundationSharedConsumersViolation::SyntaxOrDiffCollidesWithDiagnostics,
            );
        }
        if binding.shrinks_hit_target_below_supported_minimum {
            violations
                .push(M5VisualFoundationSharedConsumersViolation::HitTargetShrunkBelowMinimum);
        }
        if binding.lets_chart_meaning_depend_on_color_alone {
            violations
                .push(M5VisualFoundationSharedConsumersViolation::ChartMeaningDependsOnColorAlone);
        }
        if binding.forks_local_spacing_or_elevation_from_shared_geometry {
            violations.push(
                M5VisualFoundationSharedConsumersViolation::LocalGeometryForkedFromFoundation,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5VisualFoundationSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Vocabulary-drift accumulation.
        match object_facets.get(binding.foundation_object_id.as_str()) {
            None => {
                object_facets.insert(binding.foundation_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5VisualFoundationSharedConsumersViolation::VocabularyDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        family_consumers
            .entry(binding.family)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_families.insert(binding.family);
    }

    // Coverage: every consumer surface and every family must appear.
    for consumer in M5VisualFoundationConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5VisualFoundationSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5VisualFoundationFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5VisualFoundationSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5VisualFoundationSharedConsumersViolation::FamilyReuseUnproven);
            break;
        }
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
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
