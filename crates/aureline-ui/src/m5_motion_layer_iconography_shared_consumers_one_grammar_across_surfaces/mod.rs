//! Shared shell / dialog / notification / command-palette / onboarding / browser-handoff /
//! extension / docs-help / export-support consumers that keep the B137 visual-interaction families
//! — motion tokens, reduced-motion behavior, opacity / scrim, layer order, portal ownership,
//! iconography, and illustration boundaries — at **one grammar** across every claimed M5 surface.
//!
//! This module is the consumer-adoption lane for the seven reusable visual-interaction families
//! frozen in [`crate::m5_motion_layer_iconography_matrix`] and implemented by the motion /
//! reduced-motion lane
//! ([`crate::m5_motion_token_and_reduced_motion_registries`]), the opacity / scrim / overlay-depth
//! lane ([`crate::m5_opacity_scrim_and_overlay_depth_registries`]), the layer-order / portal lane
//! ([`crate::m5_layer_order_and_portal_registries`]), and the iconography / illustration lane
//! ([`crate::m5_iconography_and_illustration_registries`]).
//!
//! It binds each shared interaction family to the concrete shell, editor, help, marketplace /
//! extension, onboarding, settings, CLI/export, and support-export consumers that render it, and
//! proves — by fixtures, not screenshots — that the same interaction object presents the same
//! interaction-role, family, token-reference, state-variant, surface-context, and accessible-fallback
//! grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the seven shared interaction families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared visual-interaction infrastructure
//!    rather than a one-surface, feature-local fork of motion, layering, scrim, or symbol meaning.
//! 2. **One grammar / no drift.** For a given interaction object every consumer surface must present
//!    identical [`VisualInteractionStateFacetValues`] — the same interaction-role word, the same
//!    family word, the same token-reference word, the same state-variant word, the same
//!    surface-context word, and the same accessible-fallback word. The interaction-role word must be
//!    a token from the frozen [`M5VisualInteractionRole`] vocabulary, so no feature rewrites
//!    `motion`, `overlay`, `layer`, `portal`, `icon`, `illustration`, or `attention` in its own
//!    words. A surface may narrow *how much* it shows across desktop, compact, remote, and exported
//!    representations, but it may never reword the underlying grammar per surface, and a role that
//!    carries motion, overlay, icon, illustration, or attention meaning may never fall back to
//!    motion, decoration, or an unlabeled symbol alone.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
//!    per-domain schema and the frozen matrix by id, so an exported packet can always map a shell /
//!    dialog / notification / palette / onboarding / embedded visual surface back to one shared
//!    contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`VisualInteractionNarrowNote`] naming the reason, the preserved grammar, and the next
//! action, and an exported representation additionally names its export-safe detail boundary rather
//! than collapsing the object out of view.
//!
//! The packet references upstream interaction contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/design-system/m5-motion-layer-iconography-shared-consumers.schema.json`](../../../../schemas/design-system/m5-motion-layer-iconography-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_motion_layer_iconography_shared_consumers_one_grammar.md`](../../../../docs/design-system/m5_motion_layer_iconography_shared_consumers_one_grammar.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-motion-layer-iconography-shared-consumers/`](../../../../fixtures/ui/m5-motion-layer-iconography-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_motion_layer_iconography_shared_consumers,
    seeded_m5_motion_layer_iconography_shared_consumers_compact_remote_narrowed,
    seeded_m5_motion_layer_iconography_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_motion_layer_iconography_matrix::{
    M5VisualInteractionConsumerSurface, M5VisualInteractionFamily, M5VisualInteractionRole,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF, M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5VisualInteractionSharedConsumersPacket`].
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_motion_layer_iconography_shared_consumer_grammar_parity";

/// Schema version for visual-interaction shared-consumer parity records.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-motion-layer-iconography-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-layer-iconography-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/design-system/m5_motion_layer_iconography_shared_consumers_one_grammar.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-motion-layer-iconography-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Accessible-fallback sentinel words a motion / overlay / icon / illustration / attention role may
/// never fall back to; a role that carries meaning must always pair its visual cue with a real
/// reduced-motion-safe, labeled, or announced fallback, never rely on motion, decoration, or an
/// unlabeled symbol alone.
const FALLBACK_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "motion_alone",
    "decoration_alone",
    "unlabeled_symbol",
    "hover_only",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5VisualInteractionConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5VisualInteractionConsumerSurface::SupportExport
            | M5VisualInteractionConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5VisualInteractionRole`] vocabulary.
///
/// This is the "one grammar" gate: an interaction object's interaction-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_interaction_role_token(token: &str) -> bool {
    interaction_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5VisualInteractionRole`], if it is one.
pub fn interaction_role_from_token(token: &str) -> Option<M5VisualInteractionRole> {
    M5VisualInteractionRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared interaction family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation
/// still carries the same interaction-role, family, token-reference, state-variant, surface-context,
/// and accessible-fallback words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl VisualInteractionRepresentation {
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

/// A grammar axis whose word must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionParityFacet {
    /// The frozen interaction-role word.
    InteractionRoleWord,
    /// The interaction-family word.
    FamilyWord,
    /// The canonical token-reference word the family points at.
    TokenReferenceWord,
    /// The state-variant word (reduced-motion / power-saver / thermal / high-contrast coverage).
    StateVariantWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The accessible-fallback word paired with a motion / overlay / icon / illustration role.
    AccessibleFallbackWord,
}

impl VisualInteractionParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InteractionRoleWord,
        Self::FamilyWord,
        Self::TokenReferenceWord,
        Self::StateVariantWord,
        Self::SurfaceContextWord,
        Self::AccessibleFallbackWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InteractionRoleWord => "interaction_role_word",
            Self::FamilyWord => "family_word",
            Self::TokenReferenceWord => "token_reference_word",
            Self::StateVariantWord => "state_variant_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::AccessibleFallbackWord => "accessible_fallback_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared interaction family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteractionNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl VisualInteractionNarrowReason {
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
pub enum VisualInteractionNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl VisualInteractionNarrowNextAction {
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
pub enum VisualInteractionParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl VisualInteractionParityState {
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
pub enum VisualInteractionSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Interaction grammar drifted between surfaces for the same object.
    GrammarDriftDetected,
    /// A motion / overlay / icon / illustration role relied on motion or decoration alone.
    MeaningReliedOnMotionOrDecorationAlone,
    /// Motion delayed input on a protected path.
    MotionDelayedProtectedInput,
    /// A scrim erased workspace orientation or contrast.
    ScrimErasedOrientationOrContrast,
    /// An overlay or portal bypassed the shared z-order model.
    OverlayBypassedSharedZOrder,
    /// An unlabeled icon was used for an uncommon or destructive action.
    UnlabeledIconForDestructiveAction,
    /// An illustration impersonated operational, safety, or security truth.
    IllustrationImpersonatedOperationalTruth,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalReferenceMissing,
    /// An upstream shared interaction family narrowed.
    UpstreamInteractionNarrowed,
}

impl VisualInteractionSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GrammarDriftDetected,
        Self::MeaningReliedOnMotionOrDecorationAlone,
        Self::MotionDelayedProtectedInput,
        Self::ScrimErasedOrientationOrContrast,
        Self::OverlayBypassedSharedZOrder,
        Self::UnlabeledIconForDestructiveAction,
        Self::IllustrationImpersonatedOperationalTruth,
        Self::CanonicalReferenceMissing,
        Self::UpstreamInteractionNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GrammarDriftDetected => "grammar_drift_detected",
            Self::MeaningReliedOnMotionOrDecorationAlone => {
                "meaning_relied_on_motion_or_decoration_alone"
            }
            Self::MotionDelayedProtectedInput => "motion_delayed_protected_input",
            Self::ScrimErasedOrientationOrContrast => "scrim_erased_orientation_or_contrast",
            Self::OverlayBypassedSharedZOrder => "overlay_bypassed_shared_z_order",
            Self::UnlabeledIconForDestructiveAction => "unlabeled_icon_for_destructive_action",
            Self::IllustrationImpersonatedOperationalTruth => {
                "illustration_impersonated_operational_truth"
            }
            Self::CanonicalReferenceMissing => "canonical_reference_missing",
            Self::UpstreamInteractionNarrowed => "upstream_interaction_narrowed",
        }
    }
}

/// The controlled grammar an interaction object presents.
///
/// These six words must be identical across every consumer surface that shows the same interaction
/// object. The interaction-role word must be a frozen role token; the rest are controlled words the
/// object's family carries. A surface may narrow how much it renders, but it may never reword any of
/// these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionStateFacetValues {
    /// Interaction-role word (must be a frozen [`M5VisualInteractionRole`] token).
    pub interaction_role_word: String,
    /// Interaction-family word.
    pub family_word: String,
    /// Canonical token-reference word the family points at.
    pub token_reference_word: String,
    /// State-variant word (reduced-motion / power-saver / thermal / high-contrast coverage).
    pub state_variant_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Accessible-fallback word paired with a motion / overlay / icon / illustration role.
    pub accessible_fallback_word: String,
}

impl VisualInteractionStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.interaction_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.token_reference_word.trim().is_empty()
            && !self.state_variant_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.accessible_fallback_word.trim().is_empty()
    }

    /// Whether the interaction-role word is a member of the frozen role vocabulary.
    pub fn interaction_role_word_in_vocabulary(&self) -> bool {
        is_known_interaction_role_token(self.interaction_role_word.trim())
    }

    /// Whether the object honours the never-motion-or-decoration-alone rule: a role that carries
    /// motion, overlay, icon, illustration, or attention meaning must pair its visual cue with a real
    /// accessible fallback and never fall back to a motion-, decoration-, or unlabeled-symbol-alone
    /// sentinel.
    pub fn accessible_fallback_satisfied(&self) -> bool {
        match interaction_role_from_token(self.interaction_role_word.trim()) {
            Some(role) if role.demands_accessible_fallback() => {
                let cue = self.accessible_fallback_word.trim().to_lowercase();
                !cue.is_empty() && !FALLBACK_ABSENT_SENTINELS.contains(&cue.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionNarrowNote {
    /// Why the representation narrowed.
    pub reason: VisualInteractionNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: VisualInteractionNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualInteractionRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: VisualInteractionParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<VisualInteractionNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<VisualInteractionNarrowNextAction>,
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
/// representation names its export-safe-detail boundary — but all three keep every grammar word and
/// disclose the narrowing through an explicit note.
pub const fn resolve_visual_interaction_render_disclosure(
    representation: VisualInteractionRepresentation,
) -> VisualInteractionRenderDisclosure {
    match representation {
        VisualInteractionRepresentation::DesktopFull => VisualInteractionRenderDisclosure {
            parity_state: VisualInteractionParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        VisualInteractionRepresentation::CompactNarrowed => VisualInteractionRenderDisclosure {
            parity_state: VisualInteractionParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualInteractionNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(VisualInteractionNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        VisualInteractionRepresentation::RemoteProjected => VisualInteractionRenderDisclosure {
            parity_state: VisualInteractionParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualInteractionNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(VisualInteractionNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        VisualInteractionRepresentation::ExportedRedacted => VisualInteractionRenderDisclosure {
            parity_state: VisualInteractionParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(VisualInteractionNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(VisualInteractionNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared interaction family rendered on one consumer surface in one
/// representation for one interaction object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable interaction-object id (shared across surfaces that show the same object).
    pub interaction_object_id: String,
    /// Human-readable interaction-object identity.
    pub interaction_object_label: String,
    /// Which shared interaction family this binding renders.
    pub family: M5VisualInteractionFamily,
    /// Which consumer surface renders it.
    pub consumer: M5VisualInteractionConsumerSurface,
    /// Which representation this surface renders.
    pub representation: VisualInteractionRepresentation,
    /// The controlled grammar presented (identical across surfaces for one object).
    pub state_facets: VisualInteractionStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: VisualInteractionParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<VisualInteractionNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface delays input on a protected path with motion. MUST be `false`.
    pub delays_protected_input_with_motion: bool,
    /// Guardrail: this surface lets a scrim erase orientation or contrast. MUST be `false`.
    pub lets_scrim_erase_orientation_or_contrast: bool,
    /// Guardrail: this surface lets an overlay or portal bypass the shared z-order. MUST be `false`.
    pub lets_overlay_bypass_shared_z_order: bool,
    /// Guardrail: this surface uses an unlabeled icon for an uncommon or destructive action. MUST be
    /// `false`.
    pub uses_unlabeled_icon_for_uncommon_or_destructive_action: bool,
    /// Guardrail: this surface lets an illustration impersonate operational or security truth. MUST
    /// be `false`.
    pub lets_illustration_impersonate_operational_or_security_truth: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl VisualInteractionConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> VisualInteractionRenderDisclosure {
        resolve_visual_interaction_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.delays_protected_input_with_motion
            && !self.lets_scrim_erase_orientation_or_contrast
            && !self.lets_overlay_bypass_shared_z_order
            && !self.uses_unlabeled_icon_for_uncommon_or_destructive_action
            && !self.lets_illustration_impersonate_operational_or_security_truth
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
                .any(|reference| reference == M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same interaction object presents the same grammar across surfaces.
    pub same_object_same_grammar_across_surfaces: bool,
    /// Every interaction-role word is a frozen role token.
    pub interaction_role_words_stay_in_frozen_vocabulary: bool,
    /// Motion, overlay, icon, illustration, and attention meaning never relies on motion or
    /// decoration alone.
    pub meaning_never_relies_on_motion_or_decoration_alone: bool,
    /// Motion never delays input on a protected path.
    pub motion_never_delays_protected_input: bool,
    /// Scrims never erase workspace orientation or contrast.
    pub scrims_never_erase_orientation_or_contrast: bool,
    /// Overlays and portals never bypass the shared z-order model.
    pub overlays_never_bypass_shared_z_order: bool,
    /// Icons are never unlabeled for uncommon or destructive actions.
    pub icons_never_unlabeled_for_uncommon_or_destructive_actions: bool,
    /// Illustrations never impersonate operational or security truth.
    pub illustrations_never_impersonate_operational_truth: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl VisualInteractionSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_object_same_grammar_across_surfaces
            && self.interaction_role_words_stay_in_frozen_vocabulary
            && self.meaning_never_relies_on_motion_or_decoration_alone
            && self.motion_never_delays_protected_input
            && self.scrims_never_erase_orientation_or_contrast
            && self.overlays_never_bypass_shared_z_order
            && self.icons_never_unlabeled_for_uncommon_or_destructive_actions
            && self.illustrations_never_impersonate_operational_truth
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionSharedConsumersProjection {
    /// The shell UI consumes the shared visual-interaction grammar.
    pub shell_ui_consumes_shared_grammar: bool,
    /// The editor UI consumes the shared visual-interaction grammar.
    pub editor_ui_consumes_shared_grammar: bool,
    /// The help UI consumes the shared visual-interaction grammar.
    pub help_ui_consumes_shared_grammar: bool,
    /// The marketplace / extensions UI consumes the shared visual-interaction grammar.
    pub marketplace_ui_consumes_shared_grammar: bool,
    /// The onboarding UI consumes the shared visual-interaction grammar.
    pub onboarding_ui_consumes_shared_grammar: bool,
    /// The settings UI consumes the shared visual-interaction grammar.
    pub settings_ui_consumes_shared_grammar: bool,
    /// The support / export path consumes the shared visual-interaction grammar.
    pub support_export_consumes_shared_grammar: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same interaction object.
    pub grammar_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_interaction_family: bool,
}

impl VisualInteractionSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_ui_consumes_shared_grammar
            && self.editor_ui_consumes_shared_grammar
            && self.help_ui_consumes_shared_grammar
            && self.marketplace_ui_consumes_shared_grammar
            && self.onboarding_ui_consumes_shared_grammar
            && self.settings_ui_consumes_shared_grammar
            && self.support_export_consumes_shared_grammar
            && self.every_family_adopted_by_two_or_more_consumers
            && self.grammar_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_interaction_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualInteractionSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5VisualInteractionSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5VisualInteractionSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<VisualInteractionConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<VisualInteractionSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5VisualInteractionConsumerSurface>,
    /// Trust review block.
    pub trust_review: VisualInteractionSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: VisualInteractionSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: VisualInteractionSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe visual-interaction shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionSharedConsumersPacket {
    /// Record kind; must equal [`M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<VisualInteractionConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<VisualInteractionSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5VisualInteractionConsumerSurface>,
    /// Trust review block.
    pub trust_review: VisualInteractionSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: VisualInteractionSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: VisualInteractionSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5VisualInteractionSharedConsumersPacket {
    /// Builds a visual-interaction shared-consumer packet from stable-lane input.
    pub fn new(input: M5VisualInteractionSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the visual-interaction shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5VisualInteractionSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5VisualInteractionSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5VisualInteractionSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5VisualInteractionSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5VisualInteractionSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5VisualInteractionSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5VisualInteractionSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5VisualInteractionSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5VisualInteractionSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("visual-interaction shared-consumer packet serializes"),
        ) {
            violations
                .push(M5VisualInteractionSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("visual-interaction shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("family,consumer,representation,interaction_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.interaction_role_word,
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
        out.push_str("# Shared Visual-Interaction Consumers: One Grammar Across Surfaces\n\n");
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
                binding.interaction_object_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.interaction_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in visual-interaction shared-consumer export.
#[derive(Debug)]
pub enum M5VisualInteractionSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5VisualInteractionSharedConsumersViolation>),
}

impl fmt::Display for M5VisualInteractionSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "visual-interaction shared-consumer export parse failed: {error}"
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
                    "visual-interaction shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5VisualInteractionSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5VisualInteractionSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5VisualInteractionSharedConsumersViolation {
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
    /// A binding's grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's interaction-role word is not a frozen role token.
    InteractionRoleWordOutsideVocabulary,
    /// A binding's motion / overlay / icon / illustration role fell back to motion or decoration
    /// alone.
    AccessibleFallbackMissingForMeaningRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same interaction object with different grammar.
    GrammarDriftAcrossSurfaces,
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
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding delays protected input with motion.
    DelaysProtectedInputWithMotion,
    /// A binding lets a scrim erase orientation or contrast.
    ScrimErasesOrientationOrContrast,
    /// A binding lets an overlay or portal bypass the shared z-order.
    OverlayBypassesSharedZOrder,
    /// A binding uses an unlabeled icon for an uncommon or destructive action.
    UnlabeledIconForUncommonOrDestructiveAction,
    /// A binding lets an illustration impersonate operational or security truth.
    IllustrationImpersonatesOperationalTruth,
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

impl M5VisualInteractionSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::InteractionRoleWordOutsideVocabulary => {
                "interaction_role_word_outside_vocabulary"
            }
            Self::AccessibleFallbackMissingForMeaningRole => {
                "accessible_fallback_missing_for_meaning_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::GrammarDriftAcrossSurfaces => "grammar_drift_across_surfaces",
            Self::FamilyReuseUnproven => "family_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::DelaysProtectedInputWithMotion => "delays_protected_input_with_motion",
            Self::ScrimErasesOrientationOrContrast => "scrim_erases_orientation_or_contrast",
            Self::OverlayBypassesSharedZOrder => "overlay_bypasses_shared_z_order",
            Self::UnlabeledIconForUncommonOrDestructiveAction => {
                "unlabeled_icon_for_uncommon_or_destructive_action"
            }
            Self::IllustrationImpersonatesOperationalTruth => {
                "illustration_impersonates_operational_truth"
            }
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

/// Reads and validates the checked-in stable visual-interaction shared-consumer export.
pub fn current_stable_m5_motion_layer_iconography_shared_consumers_export(
) -> Result<M5VisualInteractionSharedConsumersPacket, M5VisualInteractionSharedConsumersArtifactError>
{
    let packet: M5VisualInteractionSharedConsumersPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-motion-layer-iconography-shared-consumers-proof/support_export.json"
        )
    ))
    .map_err(M5VisualInteractionSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5VisualInteractionSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5VisualInteractionSharedConsumersPacket,
    violations: &mut Vec<M5VisualInteractionSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_SHARED_CONSUMERS_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
    ];
    // The seven families map to four canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5VisualInteractionFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5VisualInteractionSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5VisualInteractionSharedConsumersPacket,
    violations: &mut Vec<M5VisualInteractionSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5VisualInteractionSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One grammar: the facet values must be identical for every binding that renders the same
    // interaction object.
    let mut object_facets: BTreeMap<&str, &VisualInteractionStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5VisualInteractionFamily,
        BTreeSet<M5VisualInteractionConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5VisualInteractionConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5VisualInteractionFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.interaction_object_id.trim().is_empty()
            || binding.interaction_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5VisualInteractionSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5VisualInteractionSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding.state_facets.interaction_role_word_in_vocabulary() {
            violations.push(
                M5VisualInteractionSharedConsumersViolation::InteractionRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.accessible_fallback_satisfied() {
            violations.push(
                M5VisualInteractionSharedConsumersViolation::AccessibleFallbackMissingForMeaningRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5VisualInteractionSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5VisualInteractionSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5VisualInteractionSharedConsumersViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5VisualInteractionSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5VisualInteractionSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5VisualInteractionSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5VisualInteractionSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5VisualInteractionSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5VisualInteractionSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.delays_protected_input_with_motion {
            violations
                .push(M5VisualInteractionSharedConsumersViolation::DelaysProtectedInputWithMotion);
        }
        if binding.lets_scrim_erase_orientation_or_contrast {
            violations.push(
                M5VisualInteractionSharedConsumersViolation::ScrimErasesOrientationOrContrast,
            );
        }
        if binding.lets_overlay_bypass_shared_z_order {
            violations
                .push(M5VisualInteractionSharedConsumersViolation::OverlayBypassesSharedZOrder);
        }
        if binding.uses_unlabeled_icon_for_uncommon_or_destructive_action {
            violations.push(
                M5VisualInteractionSharedConsumersViolation::UnlabeledIconForUncommonOrDestructiveAction,
            );
        }
        if binding.lets_illustration_impersonate_operational_or_security_truth {
            violations.push(
                M5VisualInteractionSharedConsumersViolation::IllustrationImpersonatesOperationalTruth,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5VisualInteractionSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match object_facets.get(binding.interaction_object_id.as_str()) {
            None => {
                object_facets.insert(
                    binding.interaction_object_id.as_str(),
                    &binding.state_facets,
                );
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5VisualInteractionSharedConsumersViolation::GrammarDriftAcrossSurfaces,
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
    for consumer in M5VisualInteractionConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5VisualInteractionSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5VisualInteractionFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5VisualInteractionSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5VisualInteractionSharedConsumersViolation::FamilyReuseUnproven);
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
