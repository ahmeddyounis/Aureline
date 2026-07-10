//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the M5
//! framework-pack-header / route-endpoint-row / component-service-tree-node /
//! convention-diagnostic-row / generator-preview-sheet / run-config-scaffold-card /
//! derived-relationship-banner components.
//!
//! This module is the M05-1042 accessibility-and-auto-narrowing capstone over the frozen M5
//! framework-component matrix
//! ([`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`]).
//! Where the freeze matrix defines the reusable framework pack header, route / endpoint row,
//! component / service tree node, convention-diagnostic row, generator preview sheet, run-config
//! scaffold card, and derived-relationship banner primitives, and the 1037-1041 implementation /
//! consumer lanes resolve their per-surface truth, this lane certifies — per component family —
//! that framework-aware claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe,
//! and self-narrowing** rather than presenting an unverified pack, an unproven supported version
//! range, an unlinked proving source, a heuristic inference, or a partial generator-effect as
//! still exact, first-party, fully-proven framework truth:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same pack identity, support
//!   class, exact-versus-heuristic-versus-runtime-confirmed certainty, proving-source linkage,
//!   local-versus-remote execution boundary, file / dependency / config impact, and rollback or
//!   regenerate recovery boundary the rich component shows — never a hover-only chip that strands
//!   assistive-tech or headless-CLI users. Hierarchy-heavy families (the component / service tree
//!   node's nested topology) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning
//!   from typed tokens and opaque refs **without a raw value** — never a raw credential or raw
//!   generated file payload — preserving the same stable component identity, pack / certainty
//!   posture, execution boundary, impact disclosure, proving-source linkage, recovery boundary, and
//!   narrowing reasons shown in-product so support, docs, and release proof can reconstruct exactly
//!   what the user was actually shown.
//! - **Honest auto-narrowing.** When a framework pack's health cannot be proven, a supported
//!   version range cannot be proven for the active project, a proving-source linkage is missing, a
//!   relationship is only heuristically inferred, or a generator-effect truth is only partial, the
//!   component's exactness claim auto-narrows from `ExactFrameworkTruth` to an unverified-pack /
//!   unproven-version-range / unlinked-source / heuristic-inference / partial-generator-effect
//!   projection, discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical pack / certainty / execution-boundary / proving-source / recovery
//!   boundary. The underlying pack source and recovery path is never dropped opaquely. A component
//!   with every dimension intact must NOT carry a spurious narrowing, and an unproven-version-range
//!   / unlinked-source / heuristic-inference / partial-generator-effect state can never keep an
//!   exact framework claim — incomplete evidence never invents exact certainty.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the framework-pack UI, the
//!   route / topology explorers, the diagnostic center, the generator-review and run-config
//!   surfaces, the editor gutter, the CLI surface, and the support export so product, docs, and
//!   release publication stay aligned on downgrade behavior rather than drifting in copy — an
//!   exact-looking surface can never outrun the pack / certainty / proving-source / execution
//!   proof it is being viewed away from.
//!
//! Each [`FrameworkComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::M5FrameworkComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5FrameworkRequiredLabel`] and
//! [`M5FrameworkDowngradeTrigger`] and the shared [`M5FrameworkConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the matrix
//! and the sibling primitive packets.
//!
//! The packet is metadata-only: raw generated file bodies, credentials, tokens, and secret values
//! never cross this boundary; the packet carries only typed class tokens, opaque framework refs,
//! booleans, and controlled labels so support, release, and diagnostics exports can reconstruct
//! exactly what an accessible fallback would have shown without leaking sensitive material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5FrameworkComponentFamily, M5FrameworkConsumerSurface, M5FrameworkDowngradeTrigger,
    M5FrameworkRequiredLabel, M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1042 framework-component accessibility fallback packet.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`FrameworkComponentAccessibilityPacket`].
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_framework_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`FrameworkComponentAccessibilityRow`].
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_framework_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/frameworks/m5/m5_framework_component_accessibility_fallback.md";

/// Repo-relative path of the frozen framework-component matrix this lane certifies.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_FRAMEWORK_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-framework-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-framework-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-framework-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const FRAMEWORK_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-framework-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the component / service tree
/// node's nested topology) and therefore MUST bind their tree to an equivalent flat list / textual
/// path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5FrameworkComponentFamily) -> bool {
    matches!(family, M5FrameworkComponentFamily::ComponentServiceTreeNode)
}

/// The framework dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered. The seven families fold onto the five
/// spec axes (pack health, supported version range, proving-source linkage, heuristic inference,
/// generator-effect truth); the pack header additionally carries its supported-version-range
/// dimension as a secondary condition.
const fn family_primary_dimension(
    family: M5FrameworkComponentFamily,
) -> M5FrameworkComponentClaimDimension {
    match family {
        M5FrameworkComponentFamily::FrameworkPackHeader => {
            M5FrameworkComponentClaimDimension::PackHealthIntegrity
        }
        M5FrameworkComponentFamily::RouteEndpointRow => {
            M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary
        }
        M5FrameworkComponentFamily::ComponentServiceTreeNode => {
            M5FrameworkComponentClaimDimension::ProvingSourceLinkage
        }
        M5FrameworkComponentFamily::ConventionDiagnosticRow => {
            M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary
        }
        M5FrameworkComponentFamily::GeneratorPreviewSheet => {
            M5FrameworkComponentClaimDimension::GeneratorEffectEvidence
        }
        M5FrameworkComponentFamily::RunConfigScaffoldCard => {
            M5FrameworkComponentClaimDimension::GeneratorEffectEvidence
        }
        M5FrameworkComponentFamily::DerivedRelationshipBanner => {
            M5FrameworkComponentClaimDimension::ProvingSourceLinkage
        }
    }
}

/// A rendered fallback modality for a framework component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentFallbackModality {
    /// A rich, structured (nested component / service topology tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5FrameworkComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentRenderingSurface {
    /// The full-capability desktop framework surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5FrameworkComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl FrameworkComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without leaking a raw value (a
/// raw credential or raw generated file body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw value.
    ReconstructableWithoutRawValue,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw value (red).
    RequiresRawValue,
}

impl FrameworkComponentExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw value.
    pub const fn never_requires_raw_value(self) -> bool {
        !matches!(self, Self::RequiresRawValue)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawValue => "reconstructable_without_raw_value",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawValue => "requires_raw_value",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl FrameworkComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The certainty claim ceiling a component asserts: how strong an exact-framework-truth posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a framework dimension weakens so
/// an unverified pack, an unproven supported version range, an unlinked proving source, a heuristic
/// inference, or a partial generator-effect can never keep an old `ExactFrameworkTruth` label —
/// incomplete evidence never masquerades as exact, fully-proven, first-party framework truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentClaim {
    /// Exact framework truth: a fully-proven, supported, health-verified, source-linked, exact
    /// framework insight — the strongest claim, a surface Aureline can present as exactly true
    /// right now.
    ExactFrameworkTruth,
    /// Unproven-version-range projection: the pack's supported version range cannot be proven for
    /// the active project; the surface stays an unproven-version-range projection that keeps the
    /// pack identity and last-known range visible, never claiming supported version coverage.
    UnprovenVersionRangeProjection,
    /// Unverified-pack projection: the framework pack's health / support cannot be proven; the
    /// surface stays an unverified-pack projection with its pack identity and support source
    /// preserved, never exact first-party support.
    UnverifiedPackProjection,
    /// Unlinked-source projection: the component's proving-source linkage is missing; the surface
    /// stays an unlinked-source projection with its derived state and recovery path preserved,
    /// never a source-linked exact fact.
    UnlinkedSourceProjection,
    /// Heuristic-inference projection: the relationship / route is only heuristically inferred; the
    /// surface stays a heuristic-inference projection with its inference source preserved, never an
    /// exact-from-source fact.
    HeuristicInferenceProjection,
    /// Partial-generator-effect projection: the generator-effect truth is only partial; the surface
    /// stays a partial-generator-effect projection with its file / dependency / config impact and
    /// rollback or regenerate path preserved, never a safe or no-op write.
    PartialGeneratorEffectProjection,
}

impl M5FrameworkComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ExactFrameworkTruth,
        Self::UnprovenVersionRangeProjection,
        Self::UnverifiedPackProjection,
        Self::UnlinkedSourceProjection,
        Self::HeuristicInferenceProjection,
        Self::PartialGeneratorEffectProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExactFrameworkTruth => 5,
            Self::UnprovenVersionRangeProjection => 4,
            Self::UnverifiedPackProjection => 3,
            Self::UnlinkedSourceProjection => 2,
            Self::HeuristicInferenceProjection => 1,
            Self::PartialGeneratorEffectProjection => 0,
        }
    }

    /// Returns true when this claim asserts exact, fully-proven framework truth.
    pub const fn asserts_exact_framework_truth(self) -> bool {
        matches!(self, Self::ExactFrameworkTruth)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFrameworkTruth => "exact_framework_truth",
            Self::UnprovenVersionRangeProjection => "unproven_version_range_projection",
            Self::UnverifiedPackProjection => "unverified_pack_projection",
            Self::UnlinkedSourceProjection => "unlinked_source_projection",
            Self::HeuristicInferenceProjection => "heuristic_inference_projection",
            Self::PartialGeneratorEffectProjection => "partial_generator_effect_projection",
        }
    }
}

/// The framework dimension whose state governs how far a component may claim exact framework truth.
/// The five dimensions are the five spec axes; the seven frozen component families fold onto them
/// so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentClaimDimension {
    /// Pack health integrity: is the active framework pack's health / support currently proven?
    PackHealthIntegrity,
    /// Supported version range: can the pack's supported version range be proven for the active
    /// project / profile?
    SupportedVersionRange,
    /// Proving-source linkage: does the component link back to a canonical proving source, or is
    /// the linkage missing?
    ProvingSourceLinkage,
    /// Heuristic-inference boundary: is the route / relationship exact from source, or only
    /// heuristically inferred?
    HeuristicInferenceBoundary,
    /// Generator-effect evidence: is the generator-effect truth complete, or only partial?
    GeneratorEffectEvidence,
}

impl M5FrameworkComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PackHealthIntegrity,
        Self::SupportedVersionRange,
        Self::ProvingSourceLinkage,
        Self::HeuristicInferenceBoundary,
        Self::GeneratorEffectEvidence,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackHealthIntegrity => "pack_health_integrity",
            Self::SupportedVersionRange => "supported_version_range",
            Self::ProvingSourceLinkage => "proving_source_linkage",
            Self::HeuristicInferenceBoundary => "heuristic_inference_boundary",
            Self::GeneratorEffectEvidence => "generator_effect_evidence",
        }
    }
}

/// The observed condition of one framework dimension. Anything weaker than
/// [`Self::FrameworkVerifiedExact`] imposes a narrowing ceiling on the component's exactness claim.
/// The four spec axes the lane must auto-narrow on as *incomplete evidence that may not invent
/// exact certainty* — an unproven supported version range, an unlinked proving source, a heuristic
/// inference, and a partial generator-effect — are the states that
/// [`Self::cannot_be_proven_exact`] flags. An unproven pack health is an honest support /
/// operational disclosure, not an exactness overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentConditionState {
    /// Fully proven, supported, health-verified, source-linked, and exact — imposes no ceiling.
    FrameworkVerifiedExact,
    /// The framework pack's health / support cannot be proven — exactness claim drops to an
    /// unverified-pack projection.
    PackHealthUnproven,
    /// The pack's supported version range cannot be proven for the active project — exactness claim
    /// drops to an unproven-version-range projection.
    VersionRangeUnproven,
    /// The component's proving-source linkage is missing — exactness claim drops to an
    /// unlinked-source projection.
    SourceLinkageUnproven,
    /// The route / relationship is only heuristically inferred — exactness claim drops to a
    /// heuristic-inference projection.
    HeuristicInferenceOnly,
    /// The generator-effect truth is only partial — exactness claim drops to a
    /// partial-generator-effect projection.
    GeneratorEffectPartial,
}

impl M5FrameworkComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FrameworkVerifiedExact,
        Self::PackHealthUnproven,
        Self::VersionRangeUnproven,
        Self::SourceLinkageUnproven,
        Self::HeuristicInferenceOnly,
        Self::GeneratorEffectPartial,
    ];

    /// Returns true when the dimension is weaker than verified-exact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FrameworkVerifiedExact)
    }

    /// Returns true when the condition reflects incomplete evidence that cannot be proven exact and
    /// must never be shown as exact, fully-proven framework truth. An unproven pack health is an
    /// honest support / operational disclosure, not an exactness overstatement, so it is
    /// deliberately excluded here.
    pub const fn cannot_be_proven_exact(self) -> bool {
        matches!(
            self,
            Self::VersionRangeUnproven
                | Self::SourceLinkageUnproven
                | Self::HeuristicInferenceOnly
                | Self::GeneratorEffectPartial
        )
    }

    /// The strongest exactness claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5FrameworkComponentClaim {
        match self {
            Self::FrameworkVerifiedExact => M5FrameworkComponentClaim::ExactFrameworkTruth,
            Self::PackHealthUnproven => M5FrameworkComponentClaim::UnverifiedPackProjection,
            Self::VersionRangeUnproven => M5FrameworkComponentClaim::UnprovenVersionRangeProjection,
            Self::SourceLinkageUnproven => M5FrameworkComponentClaim::UnlinkedSourceProjection,
            Self::HeuristicInferenceOnly => M5FrameworkComponentClaim::HeuristicInferenceProjection,
            Self::GeneratorEffectPartial => {
                M5FrameworkComponentClaim::PartialGeneratorEffectProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5FrameworkDowngradeTrigger {
        match self {
            // The verified-exact baseline never narrows; kept for exhaustiveness.
            Self::FrameworkVerifiedExact => M5FrameworkDowngradeTrigger::ProofStale,
            Self::PackHealthUnproven => M5FrameworkDowngradeTrigger::SupportClassUnstated,
            Self::VersionRangeUnproven => M5FrameworkDowngradeTrigger::PackIdentityUnstated,
            Self::SourceLinkageUnproven => M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
            Self::HeuristicInferenceOnly => {
                M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated
            }
            Self::GeneratorEffectPartial => M5FrameworkDowngradeTrigger::ImpactUndisclosed,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkVerifiedExact => "framework_verified_exact",
            Self::PackHealthUnproven => "pack_health_unproven",
            Self::VersionRangeUnproven => "version_range_unproven",
            Self::SourceLinkageUnproven => "source_linkage_unproven",
            Self::HeuristicInferenceOnly => "heuristic_inference_only",
            Self::GeneratorEffectPartial => "generator_effect_partial",
        }
    }
}

/// One framework dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5FrameworkComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5FrameworkComponentConditionState,
}

/// An honest exactness-claim auto-narrow block. When a framework dimension weakens, the component's
/// exactness claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical pack / certainty / execution-boundary / proving-source / recovery
/// boundary rather than silently dropping it — the underlying pack source and recovery path is
/// never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentClaimAutoNarrow {
    /// The exactness claim the component is narrowed to.
    pub narrowed_to: M5FrameworkComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5FrameworkComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5FrameworkDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity, pack / certainty source, and recovery boundary are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying pack / certainty source, proving-source linkage, and rollback / regenerate
    /// recovery boundary are preserved (never dropped) across the narrowing; must hold so
    /// unverified-pack, unproven-version-range, unlinked-source, heuristic-inference, and
    /// partial-generator-effect states never fail opaquely.
    pub preserves_source_and_recovery: bool,
}

impl FrameworkComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and the pack source
    /// / recovery boundary and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_source_and_recovery
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw value is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw value is never the only export; must always hold.
    pub raw_value_only_prohibited: bool,
}

impl FrameworkComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and a raw-value-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_value_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5FrameworkComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: FrameworkComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a framework-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw value, over-claims exactness, or drops state silently
    /// (red).
    Stranded,
}

impl FrameworkComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one framework-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentAccessibilityRow {
    /// Record kind; must equal [`FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5FrameworkComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the pack / route / node / diagnostic / generator / run-config / banner object
    /// this component represents; stays visible on every surface, so this is never empty.
    pub framework_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5FrameworkComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical pack identity, support class, certainty
    /// class, proving-source linkage, execution boundary, impact, and recovery boundary as the rich
    /// surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: FrameworkComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: FrameworkComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: FrameworkComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: FrameworkComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: FrameworkComponentCopyExportParity,
    /// The full exactness claim this family asserts when every dimension is intact.
    pub full_framework_claim: M5FrameworkComponentClaim,
    /// The observed condition of each modeled framework dimension.
    #[serde(default)]
    pub claim_conditions: Vec<FrameworkComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<FrameworkComponentClaimAutoNarrow>,
    /// Whether the underlying pack / certainty source, proving-source linkage, and rollback /
    /// regenerate recovery boundary are preserved on this component regardless of narrowing; must
    /// hold so unverified-pack, unproven-version-range, unlinked-source, heuristic-inference, and
    /// partial-generator-effect states never fail opaquely.
    pub source_and_recovery_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5FrameworkComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<FrameworkComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl FrameworkComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FrameworkVerifiedExact` when the row
    /// does not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5FrameworkComponentClaimDimension,
    ) -> M5FrameworkComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5FrameworkComponentConditionState::FrameworkVerifiedExact)
    }

    /// Whether any modeled dimension is weaker than verified-exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest exactness claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5FrameworkComponentClaim {
        let mut permitted = self.full_framework_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&FrameworkComponentClaimConditionEntry> {
        let mut binding: Option<(&FrameworkComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_framework_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5FrameworkComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The exactness claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5FrameworkComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_framework_claim,
        }
    }

    /// AC / auto-narrowing honesty: an unverified pack, an unproven supported version range, an
    /// unlinked proving source, a heuristic inference, or a partial generator-effect can no longer
    /// keep an old `ExactFrameworkTruth` label. The effective claim never exceeds the permitted
    /// ceiling; when a dimension narrows below the full claim, an honest narrow block is present,
    /// narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its
    /// frozen trigger, and preserves canonical identity and the source / recovery boundary. When
    /// nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / exactness honesty: an unproven-version-range / unlinked-source / heuristic-inference /
    /// partial-generator-effect state never keeps an exact framework claim — incomplete evidence
    /// never invents exact certainty. When such a state is modeled, the effective claim must not
    /// assert `ExactFrameworkTruth`.
    pub fn exactness_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_proven_exact());
        !(has_unprovable_state && self.effective_claim().asserts_exact_framework_truth())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth —
    /// no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a non-visual
    /// fallback, and the export reconstructs meaning without a raw value.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.framework_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw value.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_value()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: unverified-pack, unproven-version-range, unlinked-source, heuristic-inference,
    /// and partial-generator-effect states preserve the underlying pack / certainty source,
    /// proving-source linkage, and rollback / regenerate recovery boundary. The row must assert
    /// `source_and_recovery_preserved`, and any narrow block must preserve the source / recovery
    /// boundary too.
    pub fn preserves_source_and_recovery_continuity(&self) -> bool {
        self.source_and_recovery_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_source_and_recovery)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5FrameworkRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> FrameworkComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.exactness_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_source_and_recovery_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return FrameworkComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            FrameworkComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            FrameworkComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.framework_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_framework_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1042 framework-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_exactness_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_source_and_recovery_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`FrameworkComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<FrameworkComponentAccessibilityRow>,
}

/// Checked-in M05-1042 framework-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<FrameworkComponentAccessibilityRow>,
    pub summary: FrameworkComponentAccessibilitySummary,
}

impl FrameworkComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: FrameworkComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: FrameworkComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_exactness_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_source_and_recovery_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5FrameworkComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5FrameworkComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5FrameworkComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Exactness claim tiers that appear as a permitted ceiling of some row's claim conditions, so
    /// the full narrowing spectrum is exercised even where a multi-condition row's effective claim
    /// binds to a single dimension.
    pub fn represented_ceiling_claims(&self) -> BTreeSet<M5FrameworkComponentClaim> {
        self.rows
            .iter()
            .flat_map(|r| {
                r.claim_conditions
                    .iter()
                    .map(|c| c.state.permitted_ceiling())
            })
            .collect()
    }

    /// Exactness claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5FrameworkComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5FrameworkConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> FrameworkComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5FrameworkConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&FrameworkComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                FrameworkComponentAccessibilityStatus::Parity => green += 1,
                FrameworkComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                FrameworkComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        FrameworkComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::claim_is_honest),
            all_exactness_honesty_holds: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::exactness_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::export_preserves_meaning),
            all_source_and_recovery_preserved: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::preserves_source_and_recovery_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(FrameworkComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<FrameworkComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(FrameworkComponentAccessibilityViolation::SchemaVersion {
                expected: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != FRAMEWORK_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(FrameworkComponentAccessibilityViolation::RecordKind {
                expected: FRAMEWORK_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(FrameworkComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(FrameworkComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_proven_exact())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(FrameworkComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory framework label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5FrameworkComponentFallbackModality::Structured)
            {
                violations.push(
                    FrameworkComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts exact framework truth for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: an unproven-version-range / unlinked-source / heuristic-inference /
            // partial-generator-effect state never keeps an exact framework claim.
            if !row.exactness_honesty_holds() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::UnprovableStateShownAsExact {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without leaking a raw value.
            if !row.export_preserves_meaning() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::ExportRequiresRawValue {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: unverified-pack, unproven-version-range, unlinked-source,
            // heuristic-inference, and partial-generator-effect states preserve the pack /
            // certainty source and recovery boundary.
            if !row.preserves_source_and_recovery_continuity() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::SourceOrRecoveryDropped {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    FrameworkComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == FrameworkComponentAccessibilityStatus::Stranded {
                violations.push(FrameworkComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5FrameworkComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5FrameworkComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the verified-exact baseline plus each spec narrowing
        // axis) is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5FrameworkComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every exactness claim tier appears as a permitted ceiling of some condition, so
        // the full narrowing spectrum (exact-framework-truth → … → partial-generator-effect) is
        // proven end-to-end.
        let ceilings = self.represented_ceiling_claims();
        for claim in M5FrameworkComponentClaim::ALL {
            if !ceilings.contains(&claim) {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Exactness honesty must be proven with at least one unproven-version-range / unlinked-
        // source / heuristic-inference / partial-generator-effect row in the packet, so the
        // "cannot-prove never shown as exact" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(FrameworkComponentAccessibilityViolation::ExactnessHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the framework-pack, route / topology,
        // diagnostic-center, generator-review, run-config, editor-gutter, CLI, and support-export
        // surfaces — so every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5FrameworkConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    FrameworkComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(FrameworkComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("framework-component accessibility fallback packet serializes"),
        ) {
            violations.push(FrameworkComponentAccessibilityViolation::RawFrameworkMaterialInExport);
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
            .expect("framework-component accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_framework_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Framework-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5FrameworkComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_framework_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in framework-component accessibility fallback export.
pub fn current_m5_framework_component_a11y_fallback_export(
) -> Result<FrameworkComponentAccessibilityPacket, FrameworkComponentAccessibilityArtifactError> {
    let packet: FrameworkComponentAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-accessibility-fallback/support_export.json"
    )))
        .map_err(FrameworkComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in framework-component accessibility fallback export.
#[derive(Debug)]
pub enum FrameworkComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FrameworkComponentAccessibilityViolation>),
}

impl fmt::Display for FrameworkComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "framework-component accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "framework-component accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for FrameworkComponentAccessibilityArtifactError {}

/// Validation failure for M05-1042 framework-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameworkComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5FrameworkComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnprovableStateShownAsExact {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawValue {
        id: String,
    },
    SourceOrRecoveryDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5FrameworkComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5FrameworkComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5FrameworkComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5FrameworkComponentClaim,
    },
    ExactnessHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5FrameworkConsumerSurface,
    },
    SummaryMismatch,
    RawFrameworkMaterialInExport,
}

impl FrameworkComponentAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::UnprovableStateShownAsExact { .. } => "unprovable_state_shown_as_exact",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawValue { .. } => "export_requires_raw_value",
            Self::SourceOrRecoveryDropped { .. } => "source_or_recovery_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::ExactnessHonestyUnproven => "exactness_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawFrameworkMaterialInExport => "raw_framework_material_in_export",
        }
    }
}

impl fmt::Display for FrameworkComponentAccessibilityViolation {
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
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory framework label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts exact framework truth for a weakened one, or narrows spuriously"
                )
            }
            Self::UnprovableStateShownAsExact { id } => {
                write!(
                    f,
                    "row {id} shows an unproven-version-range / unlinked-source / heuristic-inference / partial-generator-effect state as exact framework truth"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawValue { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw value"
                )
            }
            Self::SourceOrRecoveryDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve the pack source / proving-source / recovery boundary across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "exactness claim tier {} does not appear as a permitted ceiling",
                    claim.as_str()
                )
            }
            Self::ExactnessHonestyUnproven => {
                write!(
                    f,
                    "no unproven-version-range / unlinked-source / heuristic-inference / partial-generator-effect row is present to prove the exactness-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawFrameworkMaterialInExport => {
                write!(f, "export contains raw framework material")
            }
        }
    }
}

impl Error for FrameworkComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
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
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "heuristic"
            | "incomplete"
            | "not exact"
            | "not proven"
            | "unproven"
            | "unverified"
            | "unlinked"
            | "inferred"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. The governed tokens for
/// this lane never legitimately name a raw credential, so a raw secret, api key, password, PEM
/// block, or bearer token is treated as forbidden.
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

/// Builds the canonical, checked-in framework-component accessibility fallback packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_framework_component_a11y_fallback_packet() -> FrameworkComponentAccessibilityPacket
{
    FrameworkComponentAccessibilityPacket::new(FrameworkComponentAccessibilityPacketInput {
        packet_id: "m5-framework-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:framework-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5FrameworkRequiredLabel> {
    M5FrameworkRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> FrameworkComponentCopyExportParity {
    FrameworkComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_value_only_prohibited: true,
    }
}

fn condition(
    dimension: M5FrameworkComponentClaimDimension,
    state: M5FrameworkComponentConditionState,
) -> FrameworkComponentClaimConditionEntry {
    FrameworkComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the CLI surface and the support /
/// release export — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5FrameworkConsumerSurface]) -> Vec<M5FrameworkConsumerSurface> {
    let mut out = vec![
        M5FrameworkConsumerSurface::CliSurface,
        M5FrameworkConsumerSurface::SupportExport,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: FrameworkComponentNarrowingDisclosureState,
) -> Vec<FrameworkComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        FrameworkComponentRenderingNarrowingDisclosure {
            rendering_surface: M5FrameworkComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        FrameworkComponentRenderingNarrowingDisclosure {
            rendering_surface: M5FrameworkComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<FrameworkComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        FrameworkComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<FrameworkComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        FrameworkComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5FrameworkComponentRenderingSurface> {
    vec![
        M5FrameworkComponentRenderingSurface::DesktopFull,
        M5FrameworkComponentRenderingSurface::CliHeadless,
        M5FrameworkComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5FrameworkComponentFallbackModality> {
    vec![
        M5FrameworkComponentFallbackModality::List,
        M5FrameworkComponentFallbackModality::Textual,
        M5FrameworkComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<FrameworkComponentAccessibilityRow> {
    vec![
        // Framework pack header (pack health + version range unproven) — the active pack's health /
        // support cannot be proven and its supported version range cannot be proven for the active
        // project, so the header carries both weak conditions and auto-narrows to the binding
        // (pack-health) unverified-pack projection, keeping the pack identity, support source, and
        // last-known version range visible, never exact first-party support (yellow).
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:framework-pack-header-unverified".to_owned(),
            component_family: M5FrameworkComponentFamily::FrameworkPackHeader,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:framework-pack-header:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:framework-pack-header-unverified:a11y".to_owned(),
            copy_export: copy_export(&[
                "pack_identity_and_version_range",
                "pack_source_and_certainty",
                "support_class",
                "keyboard_route",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![
                condition(
                    M5FrameworkComponentClaimDimension::PackHealthIntegrity,
                    M5FrameworkComponentConditionState::PackHealthUnproven,
                ),
                condition(
                    M5FrameworkComponentClaimDimension::SupportedVersionRange,
                    M5FrameworkComponentConditionState::VersionRangeUnproven,
                ),
            ],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::UnverifiedPackProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::PackHealthIntegrity,
                trigger: M5FrameworkDowngradeTrigger::SupportClassUnstated,
                narrowed_label:
                    "This framework pack's health and support class cannot be proven for the active project — shown as an unverified-pack projection that keeps the pack identity, provider source, and last-known supported version range visible, never as exact first-party support"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "pack_identity_and_version_range",
                "pack_source_and_certainty",
                "support_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::FrameworkPackUi,
                M5FrameworkConsumerSurface::EditorGutterUi,
            ]),
            source_refs: vec![
                "UX Design System §16.50 framework-pack headers".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("framework-pack-header-unverified"),
        },
        // Route / endpoint row (heuristic inference) — the route is only heuristically inferred, so
        // the row auto-narrows to a heuristic-inference projection that keeps its inference source
        // visible, never an exact-from-source route (yellow). A heuristic route is incomplete
        // evidence, so it can never keep an exact framework claim.
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:route-endpoint-row-heuristic".to_owned(),
            component_family: M5FrameworkComponentFamily::RouteEndpointRow,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:route-endpoint-row:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::DisclosedReducedButReachable,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:route-endpoint-row-heuristic:a11y".to_owned(),
            copy_export: copy_export(&[
                "route_identity",
                "evidence_certainty_class",
                "inference_source",
                "authorship_state",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary,
                M5FrameworkComponentConditionState::HeuristicInferenceOnly,
            )],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::HeuristicInferenceProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary,
                trigger: M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
                narrowed_label:
                    "This route is inferred by a heuristic convention rather than proven from source — shown as a heuristic-inference projection that names the inference source, never as an exact-from-source route"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "route_identity",
                "evidence_certainty_class",
                "inference_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::RouteExplorerUi,
                M5FrameworkConsumerSurface::EditorGutterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.9 framework-aware tooling".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("route-endpoint-row-heuristic"),
        },
        // Component / service tree node (source linkage unproven) — hierarchy-heavy (nested
        // component / service topology); the node's proving-source linkage is missing, so it
        // auto-narrows to an unlinked-source projection and binds its nested tree to a flat list /
        // textual path (yellow). A runtime-only or unresolved node is incomplete evidence, so it can
        // never keep an exact framework claim, and never fakes a source it does not have.
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:component-service-tree-node-unlinked".to_owned(),
            component_family: M5FrameworkComponentFamily::ComponentServiceTreeNode,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:component-service-tree-node:0003".to_owned(),
            fallback_modalities: vec![
                M5FrameworkComponentFallbackModality::Structured,
                M5FrameworkComponentFallbackModality::List,
                M5FrameworkComponentFallbackModality::Textual,
                M5FrameworkComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:component-service-tree-node-unlinked:a11y".to_owned(),
            copy_export: copy_export(&[
                "node_identity",
                "entity_kind_and_relation",
                "proving_source_linkage",
                "derived_note",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::ProvingSourceLinkage,
                M5FrameworkComponentConditionState::SourceLinkageUnproven,
            )],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::UnlinkedSourceProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::ProvingSourceLinkage,
                trigger: M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
                narrowed_label:
                    "This topology node is observed at runtime with no canonical proving-source file or symbol — shown as an unlinked-source projection that keeps the derived state and open-references recovery path visible, never faking a source it does not have"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "node_identity",
                "entity_kind_and_relation",
                "proving_source_linkage",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::TopologyUi,
                M5FrameworkConsumerSurface::RouteExplorerUi,
            ]),
            source_refs: vec![
                "TAD §15 language and framework intelligence architecture".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("component-service-tree-node-unlinked"),
        },
        // Convention-diagnostic row (heuristic inference) — the diagnostic is a heuristic suspicion
        // rather than a proven contract fact, so the row auto-narrows to a heuristic-inference
        // projection that keeps its detected source and support-class caveat visible, never an exact
        // contract violation (yellow).
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:convention-diagnostic-row-heuristic".to_owned(),
            component_family: M5FrameworkComponentFamily::ConventionDiagnosticRow,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:convention-diagnostic-row:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:convention-diagnostic-row-heuristic:a11y".to_owned(),
            copy_export: copy_export(&[
                "diagnostic_identity",
                "diagnostic_class_and_certainty",
                "detected_source",
                "support_class_caveat",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary,
                M5FrameworkComponentConditionState::HeuristicInferenceOnly,
            )],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::HeuristicInferenceProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::HeuristicInferenceBoundary,
                trigger: M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
                narrowed_label:
                    "This diagnostic is a heuristic suspicion rather than a proven contract violation — shown as a heuristic-inference projection that keeps the detected source and support-class caveat visible, never as an exact first-party contract fact"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "diagnostic_identity",
                "diagnostic_class_and_certainty",
                "detected_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::DiagnosticCenterUi,
                M5FrameworkConsumerSurface::EditorGutterUi,
            ]),
            source_refs: vec![
                "UX Design System §16.50 convention-diagnostic rows".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("convention-diagnostic-row-heuristic"),
        },
        // Generator preview sheet (generator-effect partial) — the generator-effect truth is only
        // partial, so the sheet auto-narrows to a partial-generator-effect projection that keeps its
        // file / dependency / config impact and rollback or regenerate path visible, never a safe or
        // no-op write (yellow). Partial generator-effect truth is incomplete evidence, so it can
        // never keep an exact framework claim.
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:generator-preview-sheet-partial".to_owned(),
            component_family: M5FrameworkComponentFamily::GeneratorPreviewSheet,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:generator-preview-sheet:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:generator-preview-sheet-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "generator_identity_and_version",
                "file_dependency_config_impact",
                "write_effect_posture",
                "rollback_or_regenerate_path",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::GeneratorEffectEvidence,
                M5FrameworkComponentConditionState::GeneratorEffectPartial,
            )],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::PartialGeneratorEffectProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::GeneratorEffectEvidence,
                trigger: M5FrameworkDowngradeTrigger::ImpactUndisclosed,
                narrowed_label:
                    "This generator's write-effect truth is only partial — shown as a partial-generator-effect projection that keeps the file / dependency / config impact and rollback or regenerate path visible, never as a safe or no-op write"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "generator_identity_and_version",
                "file_dependency_config_impact",
                "write_effect_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::GeneratorReviewUi,
                M5FrameworkConsumerSurface::RunConfigUi,
            ]),
            source_refs: vec![
                "TAD §15.8 source-first preview / live-preview rules".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("generator-preview-sheet-partial"),
        },
        // Run-config scaffold card (verified / exact) — the execution boundary, required toolchain,
        // and launch command are all proven, so it is exact framework truth and reachable on every
        // surface (green).
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:run-config-scaffold-card-verified".to_owned(),
            component_family: M5FrameworkComponentFamily::RunConfigScaffoldCard,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:run-config-scaffold-card:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:run-config-scaffold-card-verified:a11y".to_owned(),
            copy_export: copy_export(&[
                "run_config_identity",
                "execution_boundary_and_impact",
                "required_toolchain",
                "keyboard_route",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::GeneratorEffectEvidence,
                M5FrameworkComponentConditionState::FrameworkVerifiedExact,
            )],
            claim_narrow: None,
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "run_config_identity",
                "execution_boundary_and_impact",
                "required_toolchain",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::RunConfigUi,
                M5FrameworkConsumerSurface::GeneratorReviewUi,
            ]),
            source_refs: vec![
                "UX Design System §16.50 generator review sheets".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("run-config-scaffold-card-verified"),
        },
        // Derived-relationship banner (source linkage unproven) — the relationship is inferred and
        // its proving-source linkage is not yet resolved, so the banner auto-narrows to an
        // unlinked-source projection that keeps its inference source and open-raw-source recovery
        // path visible, never an exact link (yellow).
        FrameworkComponentAccessibilityRow {
            record_kind: FRAMEWORK_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:derived-relationship-banner-unlinked".to_owned(),
            component_family: M5FrameworkComponentFamily::DerivedRelationshipBanner,
            source_family_schema_ref: FRAMEWORK_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            framework_context_ref: "framework:derived-relationship-banner:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: FrameworkComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: FrameworkComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:derived-relationship-banner-unlinked:a11y".to_owned(),
            copy_export: copy_export(&[
                "banner_identity",
                "inference_source",
                "proving_source_linkage",
                "consumed_context_label",
            ]),
            full_framework_claim: M5FrameworkComponentClaim::ExactFrameworkTruth,
            claim_conditions: vec![condition(
                M5FrameworkComponentClaimDimension::ProvingSourceLinkage,
                M5FrameworkComponentConditionState::SourceLinkageUnproven,
            )],
            claim_narrow: Some(FrameworkComponentClaimAutoNarrow {
                narrowed_to: M5FrameworkComponentClaim::UnlinkedSourceProjection,
                binding_dimension: M5FrameworkComponentClaimDimension::ProvingSourceLinkage,
                trigger: M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
                narrowed_label:
                    "This relationship is derived by inference and its proving source is not yet linked — shown as an unlinked-source projection that keeps the inference source and open-raw-source or open-wider-graph recovery path visible, never as an exact link"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "banner_identity",
                "inference_source",
                "proving_source_linkage",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5FrameworkConsumerSurface::TopologyUi,
                M5FrameworkConsumerSurface::DiagnosticCenterUi,
            ]),
            source_refs: vec![
                "TAD §15 language and framework intelligence architecture".to_owned(),
                FRAMEWORK_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("derived-relationship-banner-unlinked"),
        },
    ]
}
