//! M05-1043 closing surface certification over the frozen M5 framework-pack-header /
//! route-endpoint-row / component-service-tree-node / convention-diagnostic-row /
//! generator-preview-sheet / run-config-scaffold-card / derived-relationship-banner component
//! matrix.
//!
//! Where the freeze matrix
//! (`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`)
//! defines the seven reusable framework-pack-header, route-endpoint-row, component-service-tree-node,
//! convention-diagnostic-row, generator-preview-sheet, run-config-scaffold-card, and
//! derived-relationship-banner components, the M05-1037..1040 primitive lanes narrow each one, the
//! M05-1041 consumer lane
//! (`crate::add_shared_preview_runtime_docs_browser_onboarding_template_registry_workflow_bundle_visual_designer_and_support_consumers_so_framework_aware_components_keep_pack_version_evidence_and_boundary_language_aligned_across_claimed_m5_profiles`)
//! proves they are reusable across the claimed preview-runtime / docs-browser / onboarding /
//! template-registry / workflow-bundle / visual-designer / support-export consumers, and the
//! M05-1042 accessibility / auto-narrowing capstone
//! (`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_framework_pack_health_supported_version_range_proving_source_linkage_heuristic_inference_or_generator_effect_truth_is_partial_or_stale_across_claimed_m5_framework_components`)
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared framework-component truth holds on every claimed M5 framework-aware
//! surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user actually reads a pack header, a route graph, a
//! component tree, a convention warning, a generator preview, or a run-config card on (the
//! framework-pack center, the route explorer, the topology view, the convention-diagnostics center,
//! the generator-review sheet, the run-config center, the support / export bundle, and the CLI /
//! headless surface), not on component family or primitive lane. Each
//! [`FrameworkSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, export, degraded-state, and source-linkage-and-execution-boundary — and
//! either passes (green), auto-narrows its exactness claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim inherited
//! from a healthier framework lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps an `ExactFrameworkTruth` claim while one of its truth axes is not current —
//! the framework pack's health cannot be proven, the supported version range cannot be proven for
//! the active project, a proving-source linkage is missing, a relationship is only heuristically
//! inferred, or a generator-effect truth is only partial — is over-claiming and blocks; a surface
//! that discloses the reduction by narrowing its exactness claim (with a bound reason and a frozen
//! downgrade trigger) is honestly yellow. Framework truth never loses its proving source or recovery
//! boundary: a narrowed surface always preserves its pack-identity / support / evidence-certainty /
//! proving-source / execution-boundary / rollback-or-regenerate recovery continuity rather than
//! dropping it between a pack header, a route row, a component-tree node, a convention row, a
//! generator preview, and a derived-relationship banner. The always-on export axis must always stay
//! certified, so support and automation can reconstruct the same pack / support / certainty /
//! source-linkage / execution-boundary / recovery truth from the same component identity the user
//! saw. No certified surface may let a heuristic route or component tree masquerade as exact, and no
//! certified surface may imply a no-op write when a generator changes files / dependencies / config
//! or hide the local / container / SSH / managed execution boundary behind framework convenience
//! language.
//!
//! Every row cites exactly one canonical framework-component proof bundle
//! ([`FRAMEWORK_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw generated file bytes, secret
//! material, and credential-bearing material never cross this boundary.
//!
//! The boundary schema is `schemas/ui/m5-framework-component-certification.schema.json`.
//! The contract doc is `docs/frameworks/m5/m5_framework_component_certification_contract.md`.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_preview_runtime_docs_browser_onboarding_template_registry_workflow_bundle_visual_designer_and_support_consumers_so_framework_aware_components_keep_pack_version_evidence_and_boundary_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_framework_pack_health_supported_version_range_proving_source_linkage_heuristic_inference_or_generator_effect_truth_is_partial_or_stale_across_claimed_m5_framework_components as a11y;
use a11y::M5FrameworkComponentClaim;
use matrix::{M5FrameworkComponentFamily, M5FrameworkDowngradeTrigger};

/// Schema version stamped on the M05-1043 certification packet.
pub const FRAMEWORK_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`FrameworkSurfaceCertificationPacket`].
pub const FRAMEWORK_CERT_RECORD_KIND: &str = "m5_framework_component_certification_packet";

/// Stable record-kind tag carried by each [`FrameworkSurfaceCertificationRow`].
pub const FRAMEWORK_CERT_ROW_RECORD_KIND: &str = "m5_framework_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const FRAMEWORK_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const FRAMEWORK_CERT_DOC_REF: &str =
    "docs/frameworks/m5/m5_framework_component_certification_contract.md";

/// Repo-relative path of the frozen framework-component matrix schema the certified surfaces render.
pub const FRAMEWORK_CERT_MATRIX_REF: &str = matrix::M5_FRAMEWORK_COMPONENT_SCHEMA_REF;

/// The one canonical framework-component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const FRAMEWORK_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_FRAMEWORK_COMPONENT_ARTIFACT_REF;

/// The M05-1041 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const FRAMEWORK_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_FRAMEWORK_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-1042 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every row.
pub const FRAMEWORK_CERT_A11Y_BUNDLE_REF: &str =
    a11y::FRAMEWORK_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const FRAMEWORK_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-framework-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const FRAMEWORK_CERT_CSV_REF: &str =
    "artifacts/release/m5-framework-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const FRAMEWORK_CERT_REPORT_REF: &str =
    "artifacts/release/m5-framework-component-certification/report.md";

/// The eight claimed M5 framework-aware surfaces this capstone certifies. Keyed on the surface a
/// user actually reads a pack header, route graph, component tree, convention warning, generator
/// preview, or run-config card on, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkCertifiedSurface {
    /// The framework-pack center (pack header / status strip surface).
    FrameworkPackCenter,
    /// The route / endpoint explorer surface.
    RouteExplorer,
    /// The component / service topology view surface.
    TopologyView,
    /// The convention-diagnostics center surface.
    ConventionDiagnostics,
    /// The generator-review sheet surface.
    GeneratorReview,
    /// The run-config center surface.
    RunConfigCenter,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5FrameworkCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5FrameworkCertifiedSurface; 8] = [
        M5FrameworkCertifiedSurface::FrameworkPackCenter,
        M5FrameworkCertifiedSurface::RouteExplorer,
        M5FrameworkCertifiedSurface::TopologyView,
        M5FrameworkCertifiedSurface::ConventionDiagnostics,
        M5FrameworkCertifiedSurface::GeneratorReview,
        M5FrameworkCertifiedSurface::RunConfigCenter,
        M5FrameworkCertifiedSurface::SupportExport,
        M5FrameworkCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackCenter => "framework_pack_center",
            Self::RouteExplorer => "route_explorer",
            Self::TopologyView => "topology_view",
            Self::ConventionDiagnostics => "convention_diagnostics",
            Self::GeneratorReview => "generator_review",
            Self::RunConfigCenter => "run_config_center",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, export, degraded-state, and
/// source-linkage-and-execution-boundary. The export axis is always-on and must stay certified for
/// every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkCertificationAxis {
    /// Visual parity: the framework pack identity / version / support class, authored-versus-generated
    /// status, exact-versus-heuristic-versus-runtime-confirmed truth, proving-source linkage,
    /// local-versus-remote execution boundary, file / dependency / config impact, and rollback or
    /// regenerate posture are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same pack / certainty / source-linkage / execution-boundary /
    /// recovery truth and its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or a
    /// status glyph alone, and the hierarchy-heavy component / service tree binds a flat list /
    /// textual path.
    ScreenReader,
    /// Export parity (always-on): the certified surface state is reconstructable as text / JSON /
    /// Markdown for support and automation, from the same component identity, without exposing raw
    /// generated bytes or credential-bearing material.
    Export,
    /// Degraded-state parity: an unproven pack health, an unprovable supported version range, a
    /// missing proving-source linkage, a heuristically-inferred relationship, or a partial
    /// generator-effect truth honestly downgrades an `ExactFrameworkTruth` claim to a weaker
    /// projection tier.
    DegradedState,
    /// Source-linkage-and-execution-boundary parity: the pack identity / version / support class, the
    /// exact-versus-heuristic-versus-runtime-confirmed truth, the proving-source linkage, the
    /// local / container / SSH / managed execution boundary, the file / dependency / config impact,
    /// and the rollback or regenerate recovery path stay explicit before any framework lens is
    /// trusted or any generator / launch action dispatches — never inheriting a healthier lane's
    /// exactness truth, never letting a heuristic route or component tree masquerade as exact, never
    /// implying a no-op write when a generator changes files / dependencies / config, never hiding
    /// the execution boundary behind framework convenience language, and never dropping the
    /// pack-identity / support / evidence-certainty / proving-source / recovery continuity between a
    /// pack header, a route row, a component-tree node, a convention row, a generator preview, and a
    /// derived-relationship banner.
    SourceLinkageAndExecutionBoundary,
}

impl FrameworkCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [FrameworkCertificationAxis; 6] = [
        FrameworkCertificationAxis::Visual,
        FrameworkCertificationAxis::Keyboard,
        FrameworkCertificationAxis::ScreenReader,
        FrameworkCertificationAxis::Export,
        FrameworkCertificationAxis::DegradedState,
        FrameworkCertificationAxis::SourceLinkageAndExecutionBoundary,
    ];

    /// The always-on export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::Export)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::Export => "export",
            Self::DegradedState => "degraded_state",
            Self::SourceLinkageAndExecutionBoundary => "source_linkage_and_execution_boundary",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited from
    /// a healthier surface.
    UndisclosedDrift,
}

impl FrameworkAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author — always
/// recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed exactness tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, export parity drops, proving-source /
    /// recovery is dropped, a heuristic route masquerades as exact, a generator implies a no-op write
    /// or the execution boundary is hidden, or the narrowing is inconsistent.
    Red,
}

impl FrameworkSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow); red surfaces
    /// block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-value-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The pack / support / certainty / source-linkage / execution-boundary / recovery fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-value-only export is prohibited (the export reconstructs metadata rather than
    /// dumping raw generated file bytes, secret material, or credential-bearing material).
    pub raw_value_only_prohibited: bool,
}

impl FrameworkCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-value-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_value_only_prohibited
    }
}

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: FrameworkCertificationAxis,
    /// The certification state of the axis.
    pub state: FrameworkAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5FrameworkDowngradeTrigger>,
}

impl FrameworkAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger
    ///   (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            FrameworkAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            FrameworkAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            FrameworkAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present iff the
/// certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: FrameworkCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5FrameworkComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5FrameworkComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its pack-identity / support / evidence-certainty
    /// / proving-source / execution-boundary / rollback-or-regenerate recovery continuity rather than
    /// dropping it between a pack header, a route row, a component-tree node, a convention row, a
    /// generator preview, and a derived-relationship banner.
    pub preserves_source_and_recovery_continuity: bool,
}

/// One certified M5 framework-aware surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkSurfaceCertificationRow {
    /// Record kind; must equal [`FRAMEWORK_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`FRAMEWORK_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5FrameworkCertifiedSurface,
    /// The exactness-claim ceiling the surface asserts.
    pub claimed_claim: M5FrameworkComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5FrameworkComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5FrameworkComponentFamily>,
    /// One outcome per [`FrameworkCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<FrameworkAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<FrameworkClaimAutoNarrow>,
    /// True when this surface never drops its pack-identity / support / evidence-certainty /
    /// proving-source / execution-boundary / rollback-or-regenerate recovery continuity between a
    /// pack header, a route row, a component-tree node, a convention row, a generator preview, and a
    /// derived-relationship banner.
    pub proving_source_and_recovery_preserved: bool,
    /// True iff this surface lets a heuristic route or component tree masquerade as exact
    /// (an exact-from-source or runtime-confirmed reading it did not earn). A certified surface MUST
    /// keep this false: heuristic evidence stays labeled heuristic.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// True iff this surface implies a safe or no-op write when a generator changes files /
    /// dependencies / config, or hides the local / container / SSH / managed execution boundary
    /// behind framework convenience language. A certified surface MUST keep this false: write effect
    /// and execution boundary stay disclosed.
    pub implies_no_op_write_or_hides_execution_boundary: bool,
    /// The one canonical framework proof bundle this surface cites. Must equal
    /// [`FRAMEWORK_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: FrameworkSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: FrameworkCertExportParity,
    /// The compatibility notes captured for this surface.
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

impl FrameworkSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: FrameworkCertificationAxis) -> Option<&FrameworkAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<FrameworkCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && FrameworkCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(FrameworkAxisOutcome::well_formed)
    }

    /// True when the surface narrows its exactness claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<FrameworkCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == FrameworkAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its pack-identity / support / evidence-certainty /
    /// proving-source / execution-boundary / recovery continuity rather than dropping it. A
    /// non-narrowed surface trivially preserves it; a narrowed one must say so.
    pub fn preserves_source_and_recovery_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => {
                self.proving_source_and_recovery_preserved
                    && narrow.preserves_source_and_recovery_continuity
            }
            None => self.proving_source_and_recovery_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, export parity must always
    /// certify, framework truth must never drop proving-source / recovery, let a heuristic route
    /// masquerade as exact, or imply a no-op write / hide the execution boundary, and the narrowing
    /// must be consistent.
    pub fn derive_status(&self) -> FrameworkSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != FRAMEWORK_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_source_and_recovery_continuity()
            || self.lets_heuristic_masquerade_as_exact
            || self.implies_no_op_write_or_hides_execution_boundary
        {
            return FrameworkSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return FrameworkSurfaceClaimStatus::Red;
        }

        // The always-on export axis must stay certified.
        match self.axis(FrameworkCertificationAxis::Export) {
            Some(o) if o.state == FrameworkAxisCertificationState::Certified => {}
            _ => return FrameworkSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == FrameworkAxisCertificationState::UndisclosedDrift)
        {
            return FrameworkSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return FrameworkSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return FrameworkSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_source_and_recovery_continuity
                {
                    return FrameworkSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return FrameworkSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return FrameworkSurfaceClaimStatus::Red;
        }

        FrameworkSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == FRAMEWORK_CERT_ROW_RECORD_KIND
            && self.schema_version == FRAMEWORK_CERT_SCHEMA_VERSION
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
            "surface={surface} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed} proving_source_recovery={preserved} heuristic_as_exact={heuristic} \
no_op_or_hidden_boundary={write}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.proving_source_and_recovery_preserved,
            heuristic = self.lets_heuristic_masquerade_as_exact,
            write = self.implies_no_op_write_or_hides_execution_boundary,
        )
    }
}

/// Rolled-up summary of an M05-1043 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkSurfaceCertificationSummary {
    pub row_count: usize,
    pub surface_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_surfaces_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub every_axis_covered_on_every_row: bool,
    pub all_proving_source_and_recovery_preserved: bool,
    pub no_surface_lets_heuristic_masquerade_as_exact: bool,
    pub no_surface_implies_no_op_write_or_hides_execution_boundary: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`FrameworkSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<FrameworkSurfaceCertificationRow>,
}

/// Checked-in M05-1043 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<FrameworkSurfaceCertificationRow>,
    pub summary: FrameworkSurfaceCertificationSummary,
}

impl FrameworkSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: FrameworkSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: FRAMEWORK_CERT_SCHEMA_VERSION,
            record_kind: FRAMEWORK_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: FrameworkSurfaceCertificationSummary {
                row_count: 0,
                surface_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_surfaces_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                every_axis_covered_on_every_row: false,
                all_proving_source_and_recovery_preserved: false,
                no_surface_lets_heuristic_masquerade_as_exact: false,
                no_surface_implies_no_op_write_or_hides_execution_boundary: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5FrameworkCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5FrameworkComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5FrameworkCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5FrameworkComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether an export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(FrameworkCertificationAxis::Export)
                .is_some_and(|o| o.state == FrameworkAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> FrameworkSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == FrameworkSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == FrameworkSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == FrameworkSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(FrameworkSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(FrameworkSurfaceCertificationRow::preserves_source_and_recovery_continuity);
        let no_heuristic_as_exact = self
            .rows
            .iter()
            .all(|r| !r.lets_heuristic_masquerade_as_exact);
        let no_hidden_write = self
            .rows
            .iter()
            .all(|r| !r.implies_no_op_write_or_hides_execution_boundary);

        FrameworkSurfaceCertificationSummary {
            row_count: self.rows.len(),
            surface_count: surfaces.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_surfaces_present: all_surfaces,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == FRAMEWORK_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(FrameworkSurfaceCertificationRow::covers_all_axes),
            all_proving_source_and_recovery_preserved: all_preserved,
            no_surface_lets_heuristic_masquerade_as_exact: no_heuristic_as_exact,
            no_surface_implies_no_op_write_or_hides_execution_boundary: no_hidden_write,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved
                && no_heuristic_as_exact
                && no_hidden_write,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<FrameworkCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != FRAMEWORK_CERT_SCHEMA_VERSION {
            violations.push(FrameworkCertificationViolation::SchemaVersion {
                expected: FRAMEWORK_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != FRAMEWORK_CERT_RECORD_KIND {
            violations.push(FrameworkCertificationViolation::RecordKind {
                expected: FRAMEWORK_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(FrameworkCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != FRAMEWORK_CERT_CANONICAL_BUNDLE_REF {
            violations.push(FrameworkCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(FrameworkCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(FrameworkCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(FrameworkCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(FrameworkCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != FRAMEWORK_CERT_CANONICAL_BUNDLE_REF {
                violations.push(FrameworkCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // Export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(FrameworkCertificationAxis::Export)
                    .is_none_or_state_not_certified()
            {
                violations.push(FrameworkCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Framework truth must never drop proving-source / recovery.
            if !row.preserves_source_and_recovery_continuity() {
                violations.push(FrameworkCertificationViolation::SourceOrRecoveryDropped {
                    id: row.row_id.clone(),
                });
            }

            // No certified surface may let a heuristic route masquerade as exact.
            if row.lets_heuristic_masquerade_as_exact {
                violations.push(
                    FrameworkCertificationViolation::HeuristicMasqueradesAsExact {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No certified surface may imply a no-op write or hide the execution boundary.
            if row.implies_no_op_write_or_hides_execution_boundary {
                violations.push(
                    FrameworkCertificationViolation::NoOpWriteOrHiddenExecutionBoundary {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    FrameworkCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(FrameworkCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == FrameworkSurfaceClaimStatus::Red {
                violations.push(FrameworkCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(FrameworkCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(FrameworkCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(FrameworkCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(FrameworkCertificationViolation::RawFrameworkMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,proving_source_and_recovery_preserved,lets_heuristic_masquerade_as_exact,implies_no_op_write_or_hides_execution_boundary\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved},{heuristic},{write}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.proving_source_and_recovery_preserved,
                heuristic = row.lets_heuristic_masquerade_as_exact,
                write = row.implies_no_op_write_or_hides_execution_boundary,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Framework Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5FrameworkCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Proving source and recovery preserved on every surface: {}\n",
            self.summary.all_proving_source_and_recovery_preserved
        ));
        out.push_str(&format!(
            "- No surface lets a heuristic route masquerade as exact: {}\n",
            self.summary.no_surface_lets_heuristic_masquerade_as_exact
        ));
        out.push_str(&format!(
            "- No surface implies a no-op write or hides the execution boundary: {}\n",
            self.summary
                .no_surface_implies_no_op_write_or_hides_execution_boundary
        ));
        out.push_str(&format!(
            "- Auto-narrowed surfaces: {}\n",
            self.summary.narrowed_surface_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_framework_component_certification_export(
) -> Result<FrameworkSurfaceCertificationPacket, FrameworkCertificationArtifactError> {
    let packet: FrameworkSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-certification/support_export.json"
    )))
    .map_err(FrameworkCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum FrameworkCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FrameworkCertificationViolation>),
}

impl fmt::Display for FrameworkCertificationArtifactError {
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

impl Error for FrameworkCertificationArtifactError {}

/// Validation failure for M05-1043 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameworkCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    ExportParityNotCertified { id: String },
    SourceOrRecoveryDropped { id: String },
    HeuristicMasqueradesAsExact { id: String },
    NoOpWriteOrHiddenExecutionBoundary { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawFrameworkMaterialInExport,
}

impl fmt::Display for FrameworkCertificationViolation {
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
                    "packet does not cite the canonical framework-component proof bundle"
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
                    "row {id} does not cite the one canonical framework-component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::SourceOrRecoveryDropped { id } => {
                write!(
                    f,
                    "row {id} drops pack-identity / support / evidence-certainty / proving-source / execution-boundary / recovery continuity (a narrowed surface must preserve it between a pack header, a route row, a component-tree node, a convention row, a generator preview, and a derived-relationship banner)"
                )
            }
            Self::HeuristicMasqueradesAsExact { id } => {
                write!(
                    f,
                    "row {id} lets a heuristic route or component tree masquerade as exact (an exact-from-source or runtime-confirmed reading it did not earn)"
                )
            }
            Self::NoOpWriteOrHiddenExecutionBoundary { id } => {
                write!(
                    f,
                    "row {id} implies a safe or no-op write when a generator changes files / dependencies / config, or hides the local / container / SSH / managed execution boundary behind framework convenience language"
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
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
export parity dropped, proving-source / recovery was dropped, a heuristic route masqueraded as \
exact, a generator implied a no-op write or hid the execution boundary, or the narrowing is \
inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 framework-aware surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen framework-component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawFrameworkMaterialInExport => {
                write!(f, "export contains raw framework material")
            }
        }
    }
}

impl Error for FrameworkCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&FrameworkAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != FrameworkAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure.
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
            | "stale"
            | "cached"
            | "unverified"
            | "offline"
            | "blocked"
            | "paused"
            | "snoozed"
            | "interrupted"
            | "incomplete"
            | "uncertain"
            | "partial"
            | "drifted"
            | "heuristic"
            | "not installed"
            | "not_installed"
            | "local only"
            | "local_only"
            | "no source"
            | "no_source"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The governed framework vocabulary never carries raw parameter values or secret material — it
/// stores typed class tokens, opaque refs, booleans, and redacted labels — so any credential-bearing
/// substring is a leak, not legitimate vocabulary.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
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

/// Builds the canonical, checked-in M05-1043 certification packet. Certifies all eight claimed M5
/// framework-aware surfaces: three deliver their `ExactFrameworkTruth` claim (green) and five
/// auto-narrow a not-current truth axis to a weaker projection ceiling (yellow). No surface hides
/// drift (red), no surface lets a heuristic route masquerade as exact, no surface implies a no-op
/// write or hides its execution boundary, and no surface drops its pack-identity / support /
/// evidence-certainty / proving-source / execution-boundary / recovery continuity.
pub fn seeded_m5_framework_component_certification_packet() -> FrameworkSurfaceCertificationPacket {
    FrameworkSurfaceCertificationPacket::new(FrameworkSurfaceCertificationPacketInput {
        packet_id: "m5-framework-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: FRAMEWORK_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: FRAMEWORK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:framework-component-certification:{id}"),
        FRAMEWORK_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        FRAMEWORK_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> FrameworkCertExportParity {
    FrameworkCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_value_only_prohibited: true,
    }
}

fn seed_certified_note(axis: FrameworkCertificationAxis) -> &'static str {
    match axis {
        FrameworkCertificationAxis::Visual => {
            "pack identity/version/support class, authored-versus-generated status, exact-versus-heuristic-versus-runtime-confirmed truth, proving-source linkage, local-versus-remote execution boundary, file/dependency/config impact, and rollback or regenerate posture shown on-surface"
        }
        FrameworkCertificationAxis::Keyboard => {
            "the same pack/certainty/source-linkage/execution-boundary/recovery truth and its actions are keyboard-reachable"
        }
        FrameworkCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only, and the hierarchy-heavy component/service tree binds a flat list/textual path"
        }
        FrameworkCertificationAxis::Export => {
            "surface state exports as text / JSON / Markdown for support and automation from the same component identity, carrying pack/support/certainty/proving-source/execution-boundary metadata rather than raw generated bytes"
        }
        FrameworkCertificationAxis::DegradedState => {
            "an unproven pack health, an unprovable supported version range, a missing proving-source linkage, a heuristically-inferred relationship, or a partial generator-effect truth honestly downgrades the exact-framework-truth claim"
        }
        FrameworkCertificationAxis::SourceLinkageAndExecutionBoundary => {
            "pack identity/version/support class, exact-versus-heuristic-versus-runtime-confirmed truth, proving-source linkage, the local/container/SSH/managed execution boundary, file/dependency/config impact, and the rollback or regenerate recovery path stay explicit before any framework lens is trusted or any generator/launch action dispatches; the surface never lets a heuristic route masquerade as exact, never implies a no-op write when a generator changes files/dependencies/config, never hides the execution boundary, and never drops proving-source/recovery continuity"
        }
    }
}

fn seed_certified(axis: FrameworkCertificationAxis) -> FrameworkAxisOutcome {
    FrameworkAxisOutcome {
        axis,
        state: FrameworkAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: FrameworkCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5FrameworkDowngradeTrigger,
) -> FrameworkAxisOutcome {
    FrameworkAxisOutcome {
        axis,
        state: FrameworkAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<FrameworkAxisOutcome> {
    FrameworkCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: FrameworkCertificationAxis,
    outcome: FrameworkAxisOutcome,
) -> Vec<FrameworkAxisOutcome> {
    FrameworkCertificationAxis::ALL
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
    surface: M5FrameworkCertifiedSurface,
    claimed_claim: M5FrameworkComponentClaim,
    certified_claim: M5FrameworkComponentClaim,
    consumed_families: &[M5FrameworkComponentFamily],
    axis_outcomes: Vec<FrameworkAxisOutcome>,
    claim_auto_narrow: Option<FrameworkClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> FrameworkSurfaceCertificationRow {
    let mut row = FrameworkSurfaceCertificationRow {
        record_kind: FRAMEWORK_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: FRAMEWORK_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        proving_source_and_recovery_preserved: true,
        lets_heuristic_masquerade_as_exact: false,
        implies_no_op_write_or_hides_execution_boundary: false,
        canonical_bundle_ref: FRAMEWORK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: FrameworkSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            FRAMEWORK_CERT_MATRIX_REF.to_owned(),
            FRAMEWORK_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: FrameworkCertificationAxis,
    from_claim: M5FrameworkComponentClaim,
    to_claim: M5FrameworkComponentClaim,
    label: &str,
) -> FrameworkClaimAutoNarrow {
    FrameworkClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_source_and_recovery_continuity: true,
    }
}

fn seeded_rows() -> Vec<FrameworkSurfaceCertificationRow> {
    use FrameworkCertificationAxis as Ax;
    use M5FrameworkCertifiedSurface as S;
    use M5FrameworkComponentClaim::*;
    use M5FrameworkComponentFamily::*;
    use M5FrameworkDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:run-config-center",
            S::RunConfigCenter,
            ExactFrameworkTruth,
            ExactFrameworkTruth,
            &[RunConfigScaffoldCard, GeneratorPreviewSheet],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "execution_boundary"],
            &[
                "the run-config scaffold card keeps its target kind, environment / profile, launch command, required toolchain, and local / container / SSH / managed execution boundary explicit before a convenience action dispatches execution",
                "the generator-preview sheet keeps its created-versus-modified paths, managed-versus-user-owned files, dependency / config impact, and rollback or regenerate posture explicit rather than implying a safe no-op write",
                "keyboard/screen-reader reach preserved for the run-config scaffold card and the generator-preview sheet",
                "source-linkage-and-execution-boundary: the run-config center never dispatches execution without naming where code runs and which toolchain is required",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ExactFrameworkTruth,
            ExactFrameworkTruth,
            &[FrameworkPackHeader, DerivedRelationshipBanner],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "proving_source_linkage"],
            &[
                "support export reconstructs pack / support / certainty / proving-source / execution-boundary / recovery truth from the same component identity",
                "the framework pack header keeps its pack identity, version range, and support class explicit in the exported record rather than presenting bridged or heuristic behavior as exact first-party support",
                "the derived-relationship banner keeps its source of inference, last refresh, and exact / partial / heuristic / runtime-confirmed state explicit in the exported record",
                "source-linkage-and-execution-boundary: a framework export never carries raw generated file bytes, secret material, or credential-bearing material",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExactFrameworkTruth,
            ExactFrameworkTruth,
            &[RouteEndpointRow, RunConfigScaffoldCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "evidence_source"],
            &[
                "the route / endpoint row keeps its route / matcher, source file / symbol, kind, owning framework, and evidence source explicit in the headless output rather than presenting a heuristic route as exact",
                "the run-config scaffold card keeps its launch command, required toolchain, and execution boundary explicit in the headless output before it dispatches execution",
                "keyboard/screen-reader reach is not applicable to the CLI surface but its export and boundary truth stay intact",
                "source-linkage-and-execution-boundary: the CLI surface never lets a heuristic route read as exact and never dispatches a non-local run without naming the boundary",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:framework-pack-center",
            S::FrameworkPackCenter,
            ExactFrameworkTruth,
            UnverifiedPackProjection,
            &[FrameworkPackHeader],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the active framework pack's health / support cannot be proven and its support class must be named rather than presenting the pack as exact first-party support",
                    "The framework-pack-center surface resolves an unverified pack health, so the exact-framework-truth claim narrows to unverified-pack-projection with its pack identity and support source preserved instead of reading as exact first-party support",
                    Trig::SupportClassUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactFrameworkTruth,
                UnverifiedPackProjection,
                "Pack health unproven: the framework pack header names its pack identity and support source and shows the support class cannot be verified rather than presenting bridged or heuristic behavior as exact first-party support",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the framework pack header keeps its pack identity, version range, provider source, and support class explicit rather than presenting an unverified pack as exact first-party support",
                "the framework pack header keeps its refresh-pack / open-support actions reachable while the unverified state is disclosed",
                "degraded-state: exact-framework-truth narrows to unverified-pack-projection (auto-narrowed)",
                "known compatibility note: unverified-pack behavior — an unproven pack health never reads as exact first-party support",
            ],
        ),
        seed_row(
            "cert:route-explorer",
            S::RouteExplorer,
            ExactFrameworkTruth,
            HeuristicInferenceProjection,
            &[RouteEndpointRow],
            seed_certified_except(
                Ax::SourceLinkageAndExecutionBoundary,
                seed_narrowed(
                    Ax::SourceLinkageAndExecutionBoundary,
                    "the route is only heuristically inferred and its exact-versus-heuristic certainty must be named rather than presenting the route as exact from source",
                    "The route-explorer surface resolves a heuristically-inferred route, so the exact-framework-truth claim narrows to heuristic-inference-projection with its inference source and proving-source linkage preserved instead of reading as an exact-from-source fact",
                    Trig::ExactVersusHeuristicUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceLinkageAndExecutionBoundary,
                ExactFrameworkTruth,
                HeuristicInferenceProjection,
                "Route heuristic: the route / endpoint row names its inference source and keeps its proving-source linkage and exact-versus-heuristic label rather than presenting a heuristic route as exact from source",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the route / endpoint row keeps its route / matcher, source file / symbol, evidence source, and exact-versus-heuristic label explicit rather than letting a heuristic route read as exact",
                "the route / endpoint row keeps its open-source / open-references actions reachable while the heuristic state is disclosed",
                "source-linkage-and-execution-boundary: exact-framework-truth narrows to heuristic-inference-projection (auto-narrowed)",
                "known compatibility note: heuristic-inference behavior — a heuristically-inferred route never reads as an exact-from-source fact",
            ],
        ),
        seed_row(
            "cert:topology-view",
            S::TopologyView,
            ExactFrameworkTruth,
            UnlinkedSourceProjection,
            &[ComponentServiceTreeNode, DerivedRelationshipBanner],
            seed_certified_except(
                Ax::SourceLinkageAndExecutionBoundary,
                seed_narrowed(
                    Ax::SourceLinkageAndExecutionBoundary,
                    "the component / relationship has no proving-source linkage and its derived state must be labeled rather than presenting the node as a source-linked exact fact",
                    "The topology-view surface resolves a missing proving-source linkage, so the exact-framework-truth claim narrows to unlinked-source-projection with its derived state and recovery path preserved instead of reading as a source-linked exact fact",
                    Trig::ProvingSourceOmitted,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceLinkageAndExecutionBoundary,
                ExactFrameworkTruth,
                UnlinkedSourceProjection,
                "Proving source missing: the component / service tree node and derived-relationship banner keep their derived-state label and recovery path rather than presenting an unlinked node as a source-linked exact fact",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the component / service tree node keeps its entity kind, parent / child or provider / consumer relation, and derived-versus-source-linked note explicit rather than faking a proving source it does not have",
                "the derived-relationship banner appears exactly where the inferred relationship is consumed and keeps its open-raw-source / open-wider-graph actions reachable while the unlinked state is disclosed",
                "source-linkage-and-execution-boundary: exact-framework-truth narrows to unlinked-source-projection (auto-narrowed)",
                "known compatibility note: unlinked-source behavior — a component with no proving-source linkage never reads as a source-linked exact fact",
            ],
        ),
        seed_row(
            "cert:convention-diagnostics",
            S::ConventionDiagnostics,
            ExactFrameworkTruth,
            UnprovenVersionRangeProjection,
            &[ConventionDiagnosticRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the pack's supported version range cannot be proven for the active project and its pack identity / last-known range must be named rather than claiming supported version coverage",
                    "The convention-diagnostics surface resolves an unprovable supported version range, so the exact-framework-truth claim narrows to unproven-version-range-projection with its pack identity and last-known range preserved instead of claiming supported version coverage",
                    Trig::PackIdentityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactFrameworkTruth,
                UnprovenVersionRangeProjection,
                "Version range unproven: the convention-diagnostic row names the pack identity and last-known range and flags the version mismatch rather than claiming the supported version range covers the active project",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the convention-diagnostic row keeps its distinct diagnostic class (version mismatch), affected entity / file, confidence, detected source, and support-class caveat explicit rather than collapsing it into a generic warning",
                "the convention-diagnostic row keeps its suggested-fix / open-docs actions reachable while the version-range state is disclosed",
                "degraded-state: exact-framework-truth narrows to unproven-version-range-projection (auto-narrowed)",
                "known compatibility note: unproven-version-range behavior — an unprovable supported version range never reads as verified version coverage",
            ],
        ),
        seed_row(
            "cert:generator-review",
            S::GeneratorReview,
            ExactFrameworkTruth,
            PartialGeneratorEffectProjection,
            &[GeneratorPreviewSheet],
            seed_certified_except(
                Ax::SourceLinkageAndExecutionBoundary,
                seed_narrowed(
                    Ax::SourceLinkageAndExecutionBoundary,
                    "the generator-effect truth is only partial and its file / dependency / config impact and rollback or regenerate path must be named rather than presenting a safe or no-op write",
                    "The generator-review surface resolves a partial generator-effect truth, so the exact-framework-truth claim narrows to partial-generator-effect-projection with its file / dependency / config impact and rollback or regenerate path preserved instead of implying a safe or no-op write",
                    Trig::ImpactUndisclosed,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceLinkageAndExecutionBoundary,
                ExactFrameworkTruth,
                PartialGeneratorEffectProjection,
                "Generator effect partial: the generator-preview sheet keeps its file / dependency / config impact and a rollback or regenerate recovery path explicit rather than implying a safe or no-op write",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the generator-preview sheet keeps its created-versus-modified paths, managed-versus-user-owned files, and dependency / config impact explicit rather than implying a no-op write",
                "the generator-preview sheet keeps its rollback or regenerate recovery path reachable while the partial state is disclosed",
                "source-linkage-and-execution-boundary: exact-framework-truth narrows to partial-generator-effect-projection (auto-narrowed)",
                "known compatibility note: partial-generator-effect behavior — a partial generator-effect truth never reads as a safe or no-op write",
            ],
        ),
    ]
}
