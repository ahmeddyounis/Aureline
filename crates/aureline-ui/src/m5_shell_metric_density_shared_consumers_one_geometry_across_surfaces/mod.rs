//! Shared shell / editor / review / notebook / data / debug / docs / incident / settings /
//! extension consumers that keep the B138 shell-geometry families — shell metrics, minimum sizes,
//! density modes, responsive window classes, and collapse priority — at **one geometry** across
//! every claimed M5 surface.
//!
//! This module is the consumer-adoption lane for the five reusable shell-geometry families frozen
//! in [`crate::m5_shell_metric_density_matrix`] and implemented by the shell-metric / minimum-size
//! lane ([`crate::m5_shell_metric_and_minimum_size_registries`]), the density-mode lane
//! ([`crate::m5_density_mode_registries`]), and the responsive-geometry / collapse-priority lane
//! ([`crate::m5_responsive_geometry_and_collapse_priority_registries`]).
//!
//! It binds each shared shell-geometry family to the concrete shell, editor, review, notebook,
//! data, settings, CLI/export, support-export, and general product consumers that render it, and
//! proves — by fixtures, not screenshots — that the same geometry object presents the same
//! geometry-role, family, registry-reference, width/density-class, surface-context, and
//! minimum-guarantee grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the five shared shell-geometry families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared shell-geometry infrastructure rather
//!    than a one-surface, feature-local fork of metrics, density, or adaptive sizing.
//! 2. **One geometry / no drift.** For a given geometry object every consumer surface must present
//!    identical [`ShellGeometryStateFacetValues`] — the same geometry-role word, the same family
//!    word, the same registry-reference word, the same width/density-class word, the same
//!    surface-context word, and the same minimum-guarantee word. The geometry-role word must be a
//!    token from the frozen [`M5ShellGeometryRole`] vocabulary, so no feature rewrites `zone`,
//!    `metric`, `hit_target`, `density`, `responsive`, `collapse`, or `workspace_dominance` in its
//!    own words. A surface may narrow *how much* it shows across desktop, compact, remote, and
//!    exported representations, but it may never reword the underlying grammar per surface, and a
//!    role that carries density, responsive, collapse, or workspace-dominance meaning may never
//!    drop task identity, shrink a hit target below the supported minimum, invent a private
//!    fracturing width, or hide a primary workflow behind an overlay-only fallback.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
//!    per-domain schema and the frozen matrix by id, so an exported packet can always map a shell /
//!    editor / review / notebook / data geometry surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`ShellGeometryNarrowNote`] naming the reason, the preserved grammar, and the next
//! action, and an exported representation additionally names its export-safe detail boundary rather
//! than collapsing the object out of view.
//!
//! The packet references upstream shell-geometry contracts by id rather than embedding their
//! content. Raw secret values, credentials, and private endpoints stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/shell/m5-shell-metric-density-shared-consumers.schema.json`](../../../../schemas/shell/m5-shell-metric-density-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_shell_metric_density_shared_consumers_one_geometry.md`](../../../../docs/design-system/m5_shell_metric_density_shared_consumers_one_geometry.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-shell-metric-density-shared-consumers/`](../../../../fixtures/ui/m5-shell-metric-density-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_shell_metric_density_shared_consumers,
    seeded_m5_shell_metric_density_shared_consumers_compact_remote_narrowed,
    seeded_m5_shell_metric_density_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_shell_metric_density_matrix::{
    M5ShellGeometryConsumerSurface, M5ShellGeometryFamily, M5ShellGeometryRole,
    M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF, M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ShellGeometrySharedConsumersPacket`].
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_shell_metric_density_shared_consumer_geometry_parity";

/// Schema version for shell-geometry shared-consumer parity records.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-shell-metric-density-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-metric-density-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/design-system/m5_shell_metric_density_shared_consumers_one_geometry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-metric-density-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-shell-metric-density-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-shell-metric-density-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-shell-metric-density-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Minimum-guarantee sentinel words a density / responsive / collapse / workspace-dominance role may
/// never fall back to; an adaptive role that changes presentation must always keep a real declared
/// minimum size, preserved task identity, and dominant main workspace, never collapse to a private
/// width, an overlay-only fallback, a shrunken hit target, or a hidden workflow.
const MINIMUM_GUARANTEE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "overlay_only",
    "private_width",
    "shrunk_below_minimum",
    "hidden_workflow",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5ShellGeometryConsumerSurface) -> bool {
    matches!(
        consumer,
        M5ShellGeometryConsumerSurface::SupportExport | M5ShellGeometryConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5ShellGeometryRole`] vocabulary.
///
/// This is the "one geometry" gate: a geometry object's geometry-role word must be a controlled role
/// token rather than a per-surface synonym.
pub fn is_known_geometry_role_token(token: &str) -> bool {
    geometry_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ShellGeometryRole`], if it is one.
pub fn geometry_role_from_token(token: &str) -> Option<M5ShellGeometryRole> {
    M5ShellGeometryRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared shell-geometry family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation
/// still carries the same geometry-role, family, registry-reference, width/density-class,
/// surface-context, and minimum-guarantee words, and discloses the narrowing through an explicit
/// note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl ShellGeometryRepresentation {
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
pub enum ShellGeometryParityFacet {
    /// The frozen geometry-role word.
    GeometryRoleWord,
    /// The geometry-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The width/density-class word (density mode / window class / zoom / snapped-width coverage).
    WidthOrDensityClassWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The minimum-guarantee word paired with a density / responsive / collapse role.
    MinimumGuaranteeWord,
}

impl ShellGeometryParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GeometryRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::WidthOrDensityClassWord,
        Self::SurfaceContextWord,
        Self::MinimumGuaranteeWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeometryRoleWord => "geometry_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::WidthOrDensityClassWord => "width_or_density_class_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::MinimumGuaranteeWord => "minimum_guarantee_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared shell-geometry family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl ShellGeometryNarrowReason {
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
pub enum ShellGeometryNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl ShellGeometryNarrowNextAction {
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
pub enum ShellGeometryParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ShellGeometryParityState {
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
pub enum ShellGeometrySharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Shell-geometry grammar drifted between surfaces for the same object.
    GeometryGrammarDriftDetected,
    /// A density / responsive / collapse role dropped task identity or recovery-critical state.
    TaskIdentityOrRecoveryStateDropped,
    /// Density or collapse changed command meaning, focus order, or trust visibility.
    DensityOrCollapseChangedCommandFocusOrTrust,
    /// An extension or embedded surface set a private fracturing width.
    ExtensionSetPrivateFracturingWidth,
    /// A hit target shrank below the supported minimum.
    HitTargetShrankBelowSupportedMinimum,
    /// A primary workflow was hidden behind an overlay-only fallback.
    PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
    /// A zone starved the main workspace below its minimum.
    ZoneStarvedMainWorkspaceBelowMinimum,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared shell-geometry family narrowed.
    UpstreamGeometryNarrowed,
}

impl ShellGeometrySharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GeometryGrammarDriftDetected,
        Self::TaskIdentityOrRecoveryStateDropped,
        Self::DensityOrCollapseChangedCommandFocusOrTrust,
        Self::ExtensionSetPrivateFracturingWidth,
        Self::HitTargetShrankBelowSupportedMinimum,
        Self::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
        Self::ZoneStarvedMainWorkspaceBelowMinimum,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamGeometryNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GeometryGrammarDriftDetected => "geometry_grammar_drift_detected",
            Self::TaskIdentityOrRecoveryStateDropped => "task_identity_or_recovery_state_dropped",
            Self::DensityOrCollapseChangedCommandFocusOrTrust => {
                "density_or_collapse_changed_command_focus_or_trust"
            }
            Self::ExtensionSetPrivateFracturingWidth => "extension_set_private_fracturing_width",
            Self::HitTargetShrankBelowSupportedMinimum => {
                "hit_target_shrank_below_supported_minimum"
            }
            Self::PrimaryWorkflowHiddenBehindOverlayOnlyFallback => {
                "primary_workflow_hidden_behind_overlay_only_fallback"
            }
            Self::ZoneStarvedMainWorkspaceBelowMinimum => {
                "zone_starved_main_workspace_below_minimum"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamGeometryNarrowed => "upstream_geometry_narrowed",
        }
    }
}

/// The controlled grammar a geometry object presents.
///
/// These six words must be identical across every consumer surface that shows the same geometry
/// object. The geometry-role word must be a frozen role token; the rest are controlled words the
/// object's family carries. A surface may narrow how much it renders, but it may never reword any of
/// these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryStateFacetValues {
    /// Geometry-role word (must be a frozen [`M5ShellGeometryRole`] token).
    pub geometry_role_word: String,
    /// Geometry-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Width/density-class word (density mode / window class / zoom / snapped-width coverage).
    pub width_or_density_class_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Minimum-guarantee word paired with a density / responsive / collapse role.
    pub minimum_guarantee_word: String,
}

impl ShellGeometryStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.geometry_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.width_or_density_class_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.minimum_guarantee_word.trim().is_empty()
    }

    /// Whether the geometry-role word is a member of the frozen role vocabulary.
    pub fn geometry_role_word_in_vocabulary(&self) -> bool {
        is_known_geometry_role_token(self.geometry_role_word.trim())
    }

    /// Whether the object honours the never-drop-task-identity rule: a role that carries density,
    /// responsive, collapse, or workspace-dominance meaning must pair its presentation change with a
    /// real minimum guarantee and never collapse to a private-width, overlay-only, shrunken, or
    /// hidden-workflow sentinel.
    pub fn minimum_guarantee_satisfied(&self) -> bool {
        match geometry_role_from_token(self.geometry_role_word.trim()) {
            Some(role) if role.must_preserve_task_identity_under_collapse() => {
                let guarantee = self.minimum_guarantee_word.trim().to_lowercase();
                !guarantee.is_empty()
                    && !MINIMUM_GUARANTEE_ABSENT_SENTINELS.contains(&guarantee.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryNarrowNote {
    /// Why the representation narrowed.
    pub reason: ShellGeometryNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: ShellGeometryNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellGeometryRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: ShellGeometryParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<ShellGeometryNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<ShellGeometryNarrowNextAction>,
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
pub const fn resolve_shell_geometry_render_disclosure(
    representation: ShellGeometryRepresentation,
) -> ShellGeometryRenderDisclosure {
    match representation {
        ShellGeometryRepresentation::DesktopFull => ShellGeometryRenderDisclosure {
            parity_state: ShellGeometryParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ShellGeometryRepresentation::CompactNarrowed => ShellGeometryRenderDisclosure {
            parity_state: ShellGeometryParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ShellGeometryNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(ShellGeometryNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        ShellGeometryRepresentation::RemoteProjected => ShellGeometryRenderDisclosure {
            parity_state: ShellGeometryParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ShellGeometryNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(ShellGeometryNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        ShellGeometryRepresentation::ExportedRedacted => ShellGeometryRenderDisclosure {
            parity_state: ShellGeometryParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(ShellGeometryNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(ShellGeometryNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared shell-geometry family rendered on one consumer surface in one
/// representation for one geometry object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable geometry-object id (shared across surfaces that show the same object).
    pub geometry_object_id: String,
    /// Human-readable geometry-object identity.
    pub geometry_object_label: String,
    /// Which shared shell-geometry family this binding renders.
    pub family: M5ShellGeometryFamily,
    /// Which consumer surface renders it.
    pub consumer: M5ShellGeometryConsumerSurface,
    /// Which representation this surface renders.
    pub representation: ShellGeometryRepresentation,
    /// The controlled grammar presented (identical across surfaces for one object).
    pub state_facets: ShellGeometryStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: ShellGeometryParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<ShellGeometryNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets density or collapse change command meaning, focus order, or
    /// trust visibility. MUST be `false`.
    pub density_or_collapse_changes_command_focus_or_trust: bool,
    /// Guardrail: this surface lets an extension or embedded surface set a private fracturing width.
    /// MUST be `false`.
    pub extension_or_embedded_sets_private_fracturing_width: bool,
    /// Guardrail: this surface shrinks a hit target below the supported minimum. MUST be `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Guardrail: this surface hides a primary workflow behind an overlay-only fallback. MUST be
    /// `false`.
    pub hides_primary_workflow_behind_overlay_only_fallback: bool,
    /// Guardrail: this surface lets a zone starve the main workspace below its minimum. MUST be
    /// `false`.
    pub lets_zone_starve_main_workspace_below_minimum: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ShellGeometryConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> ShellGeometryRenderDisclosure {
        resolve_shell_geometry_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.density_or_collapse_changes_command_focus_or_trust
            && !self.extension_or_embedded_sets_private_fracturing_width
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.hides_primary_workflow_behind_overlay_only_fallback
            && !self.lets_zone_starve_main_workspace_below_minimum
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
                .any(|reference| reference == M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometrySharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same geometry object presents the same grammar across surfaces.
    pub same_object_same_geometry_across_surfaces: bool,
    /// Every geometry-role word is a frozen role token.
    pub geometry_role_words_stay_in_frozen_vocabulary: bool,
    /// Density, responsive, collapse, and workspace-dominance meaning never drops task identity or
    /// recovery-critical state.
    pub adaptive_roles_never_drop_task_identity_or_recovery_state: bool,
    /// Density and collapse never change command meaning, focus order, or trust visibility.
    pub density_or_collapse_never_changes_command_focus_or_trust: bool,
    /// Extensions and embedded surfaces never set private fracturing widths.
    pub extension_or_embedded_never_sets_private_fracturing_width: bool,
    /// Hit targets never shrink below the supported minimum.
    pub hit_targets_never_shrink_below_supported_minimum: bool,
    /// Primary workflows are never hidden behind overlay-only fallbacks.
    pub primary_workflows_never_hidden_behind_overlay_only_fallback: bool,
    /// Zones never starve the main workspace below its minimum.
    pub zones_never_starve_main_workspace_below_minimum: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ShellGeometrySharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_object_same_geometry_across_surfaces
            && self.geometry_role_words_stay_in_frozen_vocabulary
            && self.adaptive_roles_never_drop_task_identity_or_recovery_state
            && self.density_or_collapse_never_changes_command_focus_or_trust
            && self.extension_or_embedded_never_sets_private_fracturing_width
            && self.hit_targets_never_shrink_below_supported_minimum
            && self.primary_workflows_never_hidden_behind_overlay_only_fallback
            && self.zones_never_starve_main_workspace_below_minimum
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometrySharedConsumersProjection {
    /// The shell UI consumes the shared shell-geometry grammar.
    pub shell_ui_consumes_shared_geometry: bool,
    /// The editor UI consumes the shared shell-geometry grammar.
    pub editor_ui_consumes_shared_geometry: bool,
    /// The review UI consumes the shared shell-geometry grammar.
    pub review_ui_consumes_shared_geometry: bool,
    /// The notebook UI consumes the shared shell-geometry grammar.
    pub notebook_ui_consumes_shared_geometry: bool,
    /// The data UI consumes the shared shell-geometry grammar.
    pub data_ui_consumes_shared_geometry: bool,
    /// The settings UI consumes the shared shell-geometry grammar.
    pub settings_ui_consumes_shared_geometry: bool,
    /// The support / export path consumes the shared shell-geometry grammar.
    pub support_export_consumes_shared_geometry: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same geometry object.
    pub geometry_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_geometry_family: bool,
}

impl ShellGeometrySharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_ui_consumes_shared_geometry
            && self.editor_ui_consumes_shared_geometry
            && self.review_ui_consumes_shared_geometry
            && self.notebook_ui_consumes_shared_geometry
            && self.data_ui_consumes_shared_geometry
            && self.settings_ui_consumes_shared_geometry
            && self.support_export_consumes_shared_geometry
            && self.every_family_adopted_by_two_or_more_consumers
            && self.geometry_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_geometry_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometrySharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ShellGeometrySharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellGeometrySharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ShellGeometryConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ShellGeometrySharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ShellGeometryConsumerSurface>,
    /// Trust review block.
    pub trust_review: ShellGeometrySharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ShellGeometrySharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: ShellGeometrySharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe shell-geometry shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometrySharedConsumersPacket {
    /// Record kind; must equal [`M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ShellGeometryConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ShellGeometrySharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ShellGeometryConsumerSurface>,
    /// Trust review block.
    pub trust_review: ShellGeometrySharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ShellGeometrySharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: ShellGeometrySharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShellGeometrySharedConsumersPacket {
    /// Builds a shell-geometry shared-consumer packet from stable-lane input.
    pub fn new(input: M5ShellGeometrySharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the shell-geometry shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5ShellGeometrySharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5ShellGeometrySharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5ShellGeometrySharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShellGeometrySharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ShellGeometrySharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ShellGeometrySharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ShellGeometrySharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ShellGeometrySharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ShellGeometrySharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("shell-geometry shared-consumer packet serializes"),
        ) {
            violations.push(M5ShellGeometrySharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("shell-geometry shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("family,consumer,representation,geometry_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.geometry_role_word,
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
        out.push_str("# Shared Shell-Geometry Consumers: One Geometry Across Surfaces\n\n");
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
                binding.geometry_object_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.geometry_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in shell-geometry shared-consumer export.
#[derive(Debug)]
pub enum M5ShellGeometrySharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShellGeometrySharedConsumersViolation>),
}

impl fmt::Display for M5ShellGeometrySharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "shell-geometry shared-consumer export parse failed: {error}"
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
                    "shell-geometry shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShellGeometrySharedConsumersArtifactError {}

/// Validation failures emitted by [`M5ShellGeometrySharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShellGeometrySharedConsumersViolation {
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
    /// A binding's geometry-role word is not a frozen role token.
    GeometryRoleWordOutsideVocabulary,
    /// A binding's density / responsive / collapse role dropped its minimum guarantee.
    MinimumGuaranteeMissingForAdaptiveRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same geometry object with different grammar.
    GeometryGrammarDriftAcrossSurfaces,
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
    /// A binding lets density or collapse change command meaning, focus order, or trust visibility.
    DensityOrCollapseChangesCommandFocusOrTrust,
    /// A binding lets an extension or embedded surface set a private fracturing width.
    ExtensionSetsPrivateFracturingWidth,
    /// A binding shrinks a hit target below the supported minimum.
    ShrinksHitTargetBelowSupportedMinimum,
    /// A binding hides a primary workflow behind an overlay-only fallback.
    HidesPrimaryWorkflowBehindOverlayOnly,
    /// A binding lets a zone starve the main workspace below its minimum.
    LetsZoneStarveMainWorkspaceBelowMinimum,
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

impl M5ShellGeometrySharedConsumersViolation {
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
            Self::GeometryRoleWordOutsideVocabulary => "geometry_role_word_outside_vocabulary",
            Self::MinimumGuaranteeMissingForAdaptiveRole => {
                "minimum_guarantee_missing_for_adaptive_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::GeometryGrammarDriftAcrossSurfaces => "geometry_grammar_drift_across_surfaces",
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
            Self::DensityOrCollapseChangesCommandFocusOrTrust => {
                "density_or_collapse_changes_command_focus_or_trust"
            }
            Self::ExtensionSetsPrivateFracturingWidth => "extension_sets_private_fracturing_width",
            Self::ShrinksHitTargetBelowSupportedMinimum => {
                "shrinks_hit_target_below_supported_minimum"
            }
            Self::HidesPrimaryWorkflowBehindOverlayOnly => {
                "hides_primary_workflow_behind_overlay_only_fallback"
            }
            Self::LetsZoneStarveMainWorkspaceBelowMinimum => {
                "lets_zone_starve_main_workspace_below_minimum"
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

/// Reads and validates the checked-in stable shell-geometry shared-consumer export.
pub fn current_stable_m5_shell_metric_density_shared_consumers_export(
) -> Result<M5ShellGeometrySharedConsumersPacket, M5ShellGeometrySharedConsumersArtifactError> {
    let packet: M5ShellGeometrySharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-metric-density-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5ShellGeometrySharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShellGeometrySharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ShellGeometrySharedConsumersPacket,
    violations: &mut Vec<M5ShellGeometrySharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
    ];
    // The five families map to two canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5ShellGeometryFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ShellGeometrySharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ShellGeometrySharedConsumersPacket,
    violations: &mut Vec<M5ShellGeometrySharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5ShellGeometrySharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One geometry: the facet values must be identical for every binding that renders the same
    // geometry object.
    let mut object_facets: BTreeMap<&str, &ShellGeometryStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5ShellGeometryFamily,
        BTreeSet<M5ShellGeometryConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ShellGeometryConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5ShellGeometryFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.geometry_object_id.trim().is_empty()
            || binding.geometry_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ShellGeometrySharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5ShellGeometrySharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding.state_facets.geometry_role_word_in_vocabulary() {
            violations
                .push(M5ShellGeometrySharedConsumersViolation::GeometryRoleWordOutsideVocabulary);
        }
        if !binding.state_facets.minimum_guarantee_satisfied() {
            violations.push(
                M5ShellGeometrySharedConsumersViolation::MinimumGuaranteeMissingForAdaptiveRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5ShellGeometrySharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5ShellGeometrySharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5ShellGeometrySharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5ShellGeometrySharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5ShellGeometrySharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5ShellGeometrySharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5ShellGeometrySharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5ShellGeometrySharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5ShellGeometrySharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.density_or_collapse_changes_command_focus_or_trust {
            violations.push(
                M5ShellGeometrySharedConsumersViolation::DensityOrCollapseChangesCommandFocusOrTrust,
            );
        }
        if binding.extension_or_embedded_sets_private_fracturing_width {
            violations
                .push(M5ShellGeometrySharedConsumersViolation::ExtensionSetsPrivateFracturingWidth);
        }
        if binding.shrinks_hit_target_below_supported_minimum {
            violations.push(
                M5ShellGeometrySharedConsumersViolation::ShrinksHitTargetBelowSupportedMinimum,
            );
        }
        if binding.hides_primary_workflow_behind_overlay_only_fallback {
            violations.push(
                M5ShellGeometrySharedConsumersViolation::HidesPrimaryWorkflowBehindOverlayOnly,
            );
        }
        if binding.lets_zone_starve_main_workspace_below_minimum {
            violations.push(
                M5ShellGeometrySharedConsumersViolation::LetsZoneStarveMainWorkspaceBelowMinimum,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ShellGeometrySharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match object_facets.get(binding.geometry_object_id.as_str()) {
            None => {
                object_facets.insert(binding.geometry_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5ShellGeometrySharedConsumersViolation::GeometryGrammarDriftAcrossSurfaces,
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
    for consumer in M5ShellGeometryConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ShellGeometrySharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5ShellGeometryFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5ShellGeometrySharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ShellGeometrySharedConsumersViolation::FamilyReuseUnproven);
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
