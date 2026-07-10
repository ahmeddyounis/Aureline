//! M05-1027 closing surface certification over the frozen M5 scaffold-template-card /
//! starter-parameter-row / scaffold-preflight-card / template-health-row /
//! generated-project-diff-card / scaffold-handoff-banner component matrix.
//!
//! Where the freeze matrix
//! (`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`)
//! defines the six reusable scaffold-template-card, starter-parameter-row,
//! scaffold-preflight-card, template-health-row, generated-project-diff-card, and
//! scaffold-handoff-banner components, the M05-1021..1024 primitive lanes narrow each one, the
//! M05-1025 consumer lane
//! (`crate::add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles`)
//! proves they are reusable across the claimed start-center / workspace-admission /
//! template-registry / framework-pack / workflow-bundle / help-support / safe-handoff-export
//! consumers, and the M05-1026 accessibility / auto-narrowing capstone
//! (`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_template_freshness_drifted_prerequisite_health_is_blocked_starter_parameters_are_secret_bound_or_generation_diff_truth_is_partial_across_claimed_m5_scaffold_components`)
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared scaffold-component truth holds on every claimed M5 stack-entry and
//! project-generation surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user actually reviews a starter, a preflight, a health
//! signal, a generated diff, or a workspace handoff on (the start center, the template gallery, the
//! scaffold preflight, the generation diff-review, the workspace handoff, the template-health
//! dashboard, the support / export bundle, and the CLI / headless surface), not on component family
//! or primitive lane. Each [`ScaffoldSurfaceCertificationRow`] certifies one surface across six
//! truth axes — visual, keyboard, screen-reader, export, degraded-state, and
//! source-side-effect-and-recovery — and either passes (green), auto-narrows its readiness claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden
//! behind a full-truth claim inherited from a healthier scaffold lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `QualifiedStarter` claim while one of its truth axes is not current — the
//! template freshness has drifted, the generation diff truth is partial, a prerequisite health
//! check is blocked, or a starter parameter is secret-bound and cannot travel — is over-claiming
//! and blocks; a surface that discloses the reduction by narrowing its readiness claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Scaffold truth never loses its
//! source or recovery: a narrowed surface always preserves its starter-source / support /
//! side-effect / generated-versus-user-owned / delete-generated-or-continue-without-starter recovery
//! continuity rather than dropping it between a template card, a preflight card, a generated diff
//! card, and a workspace handoff banner. The always-on export axis must always stay certified, so
//! support and automation can reconstruct the same source / support / side-effect / health / recovery
//! truth from the same component identity the user saw. No certified surface may let a generic
//! `Create` hide a network, dependency-install, remote-provisioning, trust, or managed-workspace side
//! effect, and no certified surface may expose a secret-bound raw value by default: creation stays
//! side-effect-disclosed and secret references stay redacted.
//!
//! Every row cites exactly one canonical scaffold-component proof bundle
//! ([`SCAFFOLD_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw parameter values, secret
//! material, generated file bytes, and credential-bearing material never cross this boundary.
//!
//! The boundary schema is `schemas/ui/m5-scaffold-component-certification.schema.json`.
//! The contract doc is `docs/templates/m5_scaffold_component_certification_contract.md`.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_template_freshness_drifted_prerequisite_health_is_blocked_starter_parameters_are_secret_bound_or_generation_diff_truth_is_partial_across_claimed_m5_scaffold_components as a11y;
use a11y::M5ScaffoldComponentClaim;
use matrix::{M5ScaffoldComponentFamily, M5ScaffoldDowngradeTrigger};

/// Schema version stamped on the M05-1027 certification packet.
pub const SCAFFOLD_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ScaffoldSurfaceCertificationPacket`].
pub const SCAFFOLD_CERT_RECORD_KIND: &str = "m5_scaffold_component_certification_packet";

/// Stable record-kind tag carried by each [`ScaffoldSurfaceCertificationRow`].
pub const SCAFFOLD_CERT_ROW_RECORD_KIND: &str = "m5_scaffold_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const SCAFFOLD_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const SCAFFOLD_CERT_DOC_REF: &str =
    "docs/templates/m5_scaffold_component_certification_contract.md";

/// Repo-relative path of the frozen scaffold-component matrix schema the certified surfaces render.
pub const SCAFFOLD_CERT_MATRIX_REF: &str = matrix::M5_SCAFFOLD_COMPONENT_SCHEMA_REF;

/// The one canonical scaffold-component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const SCAFFOLD_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_SCAFFOLD_COMPONENT_ARTIFACT_REF;

/// The M05-1025 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const SCAFFOLD_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_SCAFFOLD_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-1026 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every row.
pub const SCAFFOLD_CERT_A11Y_BUNDLE_REF: &str = a11y::SCAFFOLD_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SCAFFOLD_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SCAFFOLD_CERT_CSV_REF: &str =
    "artifacts/release/m5-scaffold-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SCAFFOLD_CERT_REPORT_REF: &str =
    "artifacts/release/m5-scaffold-component-certification/report.md";

/// The eight claimed M5 stack-entry and project-generation surfaces this capstone certifies.
/// Keyed on the surface a user actually reviews a starter, preflight, health signal, generated
/// diff, or workspace handoff on, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldCertifiedSurface {
    /// The start center (recent work / project entry / new-project surface).
    StartCenter,
    /// The template gallery surface.
    TemplateGallery,
    /// The scaffold preflight surface.
    ScaffoldPreflight,
    /// The generation diff-review surface.
    GenerationDiffReview,
    /// The workspace handoff surface (post-bootstrap).
    WorkspaceHandoff,
    /// The template-health dashboard surface.
    TemplateHealth,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5ScaffoldCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ScaffoldCertifiedSurface; 8] = [
        M5ScaffoldCertifiedSurface::StartCenter,
        M5ScaffoldCertifiedSurface::TemplateGallery,
        M5ScaffoldCertifiedSurface::ScaffoldPreflight,
        M5ScaffoldCertifiedSurface::GenerationDiffReview,
        M5ScaffoldCertifiedSurface::WorkspaceHandoff,
        M5ScaffoldCertifiedSurface::TemplateHealth,
        M5ScaffoldCertifiedSurface::SupportExport,
        M5ScaffoldCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::TemplateGallery => "template_gallery",
            Self::ScaffoldPreflight => "scaffold_preflight",
            Self::GenerationDiffReview => "generation_diff_review",
            Self::WorkspaceHandoff => "workspace_handoff",
            Self::TemplateHealth => "template_health",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, export, degraded-state, and
/// source-side-effect-and-recovery. The export axis is always-on and must stay certified for every
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldCertificationAxis {
    /// Visual parity: the starter source / support class, host boundary, parameter source,
    /// immediate-versus-deferred action, file / dependency / task / extension impact, health
    /// freshness, and generated-versus-user-owned boundary are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same source / side-effect / health / recovery truth and its
    /// actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or a
    /// status glyph alone, and the hierarchy-heavy generated diff tree binds a flat list / textual
    /// path.
    ScreenReader,
    /// Export parity (always-on): the certified surface state is reconstructable as text / JSON /
    /// Markdown for support and automation, from the same component identity, without exposing a
    /// raw parameter value or secret material.
    Export,
    /// Degraded-state parity: a drifted template freshness, a partial generation diff, a blocked
    /// prerequisite, a secret-bound parameter, or a cached / unchecked validation honestly
    /// downgrades a `QualifiedStarter` claim to a weaker projection tier.
    DegradedState,
    /// Source-side-effect-and-recovery parity: the starter source and support class, the host and
    /// managed-workspace boundary, the concrete side effect behind any `Create`, the
    /// generated-versus-user-owned boundary, and the delete-generated / continue-without-starter
    /// recovery path stay explicit before any create, generate, or handoff — never inheriting a
    /// healthier lane's readiness truth, never letting a generic `Create` hide a side effect, never
    /// exposing a secret-bound raw value by default, and never dropping the starter-source /
    /// support / side-effect / recovery continuity between a template card, a preflight card, a
    /// generated diff card, and a workspace handoff banner.
    SourceSideEffectAndRecovery,
}

impl ScaffoldCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ScaffoldCertificationAxis; 6] = [
        ScaffoldCertificationAxis::Visual,
        ScaffoldCertificationAxis::Keyboard,
        ScaffoldCertificationAxis::ScreenReader,
        ScaffoldCertificationAxis::Export,
        ScaffoldCertificationAxis::DegradedState,
        ScaffoldCertificationAxis::SourceSideEffectAndRecovery,
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
            Self::SourceSideEffectAndRecovery => "source_side_effect_and_recovery",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited from
    /// a healthier surface.
    UndisclosedDrift,
}

impl ScaffoldAxisCertificationState {
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
pub enum ScaffoldSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed readiness tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, export parity drops, source / recovery
    /// is dropped, a generic `Create` hides a side effect, a secret-bound raw value is exposed by
    /// default, or the narrowing is inconsistent.
    Red,
}

impl ScaffoldSurfaceClaimStatus {
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
pub struct ScaffoldCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The source / support / side-effect / health / recovery fields the surface preserves in
    /// export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-value-only export is prohibited (the export reconstructs metadata rather than
    /// dumping raw parameter values, secret material, or generated file bytes).
    pub raw_value_only_prohibited: bool,
}

impl ScaffoldCertExportParity {
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
pub struct ScaffoldAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ScaffoldCertificationAxis,
    /// The certification state of the axis.
    pub state: ScaffoldAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ScaffoldDowngradeTrigger>,
}

impl ScaffoldAxisOutcome {
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
            ScaffoldAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ScaffoldAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ScaffoldAxisCertificationState::UndisclosedDrift => {
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
pub struct ScaffoldClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ScaffoldCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5ScaffoldComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5ScaffoldComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its starter-source / support / side-effect /
    /// generated-versus-user-owned / recovery continuity rather than dropping it between a template
    /// card, a preflight card, a generated diff card, and a workspace handoff banner.
    pub preserves_source_and_recovery_continuity: bool,
}

/// One certified M5 stack-entry / project-generation surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldSurfaceCertificationRow {
    /// Record kind; must equal [`SCAFFOLD_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ScaffoldCertifiedSurface,
    /// The readiness-claim ceiling the surface asserts.
    pub claimed_claim: M5ScaffoldComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5ScaffoldComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ScaffoldComponentFamily>,
    /// One outcome per [`ScaffoldCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ScaffoldAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ScaffoldClaimAutoNarrow>,
    /// True when this surface never drops its starter-source / support / side-effect /
    /// generated-versus-user-owned / recovery continuity between a template card, a preflight card,
    /// a generated diff card, and a workspace handoff banner.
    pub source_and_recovery_preserved: bool,
    /// True iff this surface lets a generic `Create` hide a network, dependency-install,
    /// remote-provisioning, trust, or managed-workspace side effect. A certified surface MUST keep
    /// this false: creation stays side-effect-disclosed.
    pub hides_side_effect_behind_generic_create: bool,
    /// True iff this surface exposes a secret-bound raw value by default in a review / share /
    /// export flow. A certified surface MUST keep this false: secret references stay redacted and a
    /// raw value is never the default.
    pub exposes_raw_value_by_default: bool,
    /// The one canonical scaffold proof bundle this surface cites. Must equal
    /// [`SCAFFOLD_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ScaffoldSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: ScaffoldCertExportParity,
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

impl ScaffoldSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ScaffoldCertificationAxis) -> Option<&ScaffoldAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ScaffoldCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ScaffoldCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ScaffoldAxisOutcome::well_formed)
    }

    /// True when the surface narrows its readiness claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ScaffoldCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ScaffoldAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its starter-source / support / side-effect / recovery
    /// continuity rather than dropping it. A non-narrowed surface trivially preserves it; a narrowed
    /// one must say so.
    pub fn preserves_source_and_recovery_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => {
                self.source_and_recovery_preserved
                    && narrow.preserves_source_and_recovery_continuity
            }
            None => self.source_and_recovery_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, export parity must always
    /// certify, scaffold truth must never drop source / recovery, let a generic `Create` hide a side
    /// effect, or expose a secret-bound raw value by default, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ScaffoldSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != SCAFFOLD_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_source_and_recovery_continuity()
            || self.hides_side_effect_behind_generic_create
            || self.exposes_raw_value_by_default
        {
            return ScaffoldSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ScaffoldSurfaceClaimStatus::Red;
        }

        // The always-on export axis must stay certified.
        match self.axis(ScaffoldCertificationAxis::Export) {
            Some(o) if o.state == ScaffoldAxisCertificationState::Certified => {}
            _ => return ScaffoldSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ScaffoldAxisCertificationState::UndisclosedDrift)
        {
            return ScaffoldSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ScaffoldSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ScaffoldSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_source_and_recovery_continuity
                {
                    return ScaffoldSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ScaffoldSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return ScaffoldSurfaceClaimStatus::Red;
        }

        ScaffoldSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SCAFFOLD_CERT_ROW_RECORD_KIND
            && self.schema_version == SCAFFOLD_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} source_recovery={preserved} hides_side_effect={hides} raw_value={raw}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.source_and_recovery_preserved,
            hides = self.hides_side_effect_behind_generic_create,
            raw = self.exposes_raw_value_by_default,
        )
    }
}

/// Rolled-up summary of an M05-1027 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldSurfaceCertificationSummary {
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
    pub all_source_and_recovery_preserved: bool,
    pub no_surface_hides_side_effect: bool,
    pub no_surface_exposes_raw_value: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`ScaffoldSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ScaffoldSurfaceCertificationRow>,
}

/// Checked-in M05-1027 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ScaffoldSurfaceCertificationRow>,
    pub summary: ScaffoldSurfaceCertificationSummary,
}

impl ScaffoldSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ScaffoldSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SCAFFOLD_CERT_SCHEMA_VERSION,
            record_kind: SCAFFOLD_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ScaffoldSurfaceCertificationSummary {
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
                all_source_and_recovery_preserved: false,
                no_surface_hides_side_effect: false,
                no_surface_exposes_raw_value: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ScaffoldCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ScaffoldComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ScaffoldCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ScaffoldComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether an export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ScaffoldCertificationAxis::Export)
                .is_some_and(|o| o.state == ScaffoldAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ScaffoldSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ScaffoldSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ScaffoldSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ScaffoldSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ScaffoldSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(ScaffoldSurfaceCertificationRow::preserves_source_and_recovery_continuity);
        let no_hidden_side_effect = self
            .rows
            .iter()
            .all(|r| !r.hides_side_effect_behind_generic_create);
        let no_raw_value = self.rows.iter().all(|r| !r.exposes_raw_value_by_default);

        ScaffoldSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == SCAFFOLD_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ScaffoldSurfaceCertificationRow::covers_all_axes),
            all_source_and_recovery_preserved: all_preserved,
            no_surface_hides_side_effect: no_hidden_side_effect,
            no_surface_exposes_raw_value: no_raw_value,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved
                && no_hidden_side_effect
                && no_raw_value,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ScaffoldCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SCAFFOLD_CERT_SCHEMA_VERSION {
            violations.push(ScaffoldCertificationViolation::SchemaVersion {
                expected: SCAFFOLD_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SCAFFOLD_CERT_RECORD_KIND {
            violations.push(ScaffoldCertificationViolation::RecordKind {
                expected: SCAFFOLD_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ScaffoldCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != SCAFFOLD_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ScaffoldCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ScaffoldCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ScaffoldCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ScaffoldCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ScaffoldCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != SCAFFOLD_CERT_CANONICAL_BUNDLE_REF {
                violations.push(ScaffoldCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // Export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ScaffoldCertificationAxis::Export)
                    .is_none_or_state_not_certified()
            {
                violations.push(ScaffoldCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Scaffold truth must never drop source / recovery.
            if !row.preserves_source_and_recovery_continuity() {
                violations.push(ScaffoldCertificationViolation::SourceOrRecoveryDropped {
                    id: row.row_id.clone(),
                });
            }

            // No certified surface may let a generic Create hide a side effect.
            if row.hides_side_effect_behind_generic_create {
                violations.push(
                    ScaffoldCertificationViolation::SideEffectHiddenBehindGenericCreate {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No certified surface may expose a secret-bound raw value by default.
            if row.exposes_raw_value_by_default {
                violations.push(ScaffoldCertificationViolation::RawValueExposedByDefault {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(ScaffoldCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ScaffoldCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == ScaffoldSurfaceClaimStatus::Red {
                violations.push(ScaffoldCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(ScaffoldCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(ScaffoldCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ScaffoldCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ScaffoldCertificationViolation::RawStarterMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,source_and_recovery_preserved,hides_side_effect,exposes_raw_value\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved},{hides},{raw}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.source_and_recovery_preserved,
                hides = row.hides_side_effect_behind_generic_create,
                raw = row.exposes_raw_value_by_default,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Scaffold Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ScaffoldCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Source and recovery preserved on every surface: {}\n",
            self.summary.all_source_and_recovery_preserved
        ));
        out.push_str(&format!(
            "- No surface hides a side effect behind a generic Create: {}\n",
            self.summary.no_surface_hides_side_effect
        ));
        out.push_str(&format!(
            "- No surface exposes a secret-bound raw value by default: {}\n",
            self.summary.no_surface_exposes_raw_value
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
pub fn current_m5_scaffold_component_certification_export(
) -> Result<ScaffoldSurfaceCertificationPacket, ScaffoldCertificationArtifactError> {
    let packet: ScaffoldSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-certification/support_export.json"
    )))
    .map_err(ScaffoldCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ScaffoldCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldCertificationViolation>),
}

impl fmt::Display for ScaffoldCertificationArtifactError {
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

impl Error for ScaffoldCertificationArtifactError {}

/// Validation failure for M05-1027 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldCertificationViolation {
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
    SideEffectHiddenBehindGenericCreate { id: String },
    RawValueExposedByDefault { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawStarterMaterialInExport,
}

impl fmt::Display for ScaffoldCertificationViolation {
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
                    "packet does not cite the canonical scaffold-component proof bundle"
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
                    "row {id} does not cite the one canonical scaffold-component proof bundle"
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
                    "row {id} drops starter-source / support / side-effect / recovery continuity (a narrowed surface must preserve it between a template card, a preflight card, a generated diff card, and a workspace handoff banner)"
                )
            }
            Self::SideEffectHiddenBehindGenericCreate { id } => {
                write!(
                    f,
                    "row {id} lets a generic Create hide a network, dependency-install, remote-provisioning, trust, or managed-workspace side effect"
                )
            }
            Self::RawValueExposedByDefault { id } => {
                write!(
                    f,
                    "row {id} exposes a secret-bound raw value by default (secret references must stay redacted and a raw value must be opt-in)"
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
export parity dropped, source / recovery was dropped, a generic Create hid a side effect, a \
secret-bound raw value was exposed by default, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 stack-entry / project-generation surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen scaffold-component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawStarterMaterialInExport => {
                write!(f, "export contains raw starter material")
            }
        }
    }
}

impl Error for ScaffoldCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ScaffoldAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ScaffoldAxisCertificationState::Certified,
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
            | "secret_bound"
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
/// The governed vocabulary legitimately names `secret_reference`, `secret_bound_parameter`, and
/// `SecretBoundParameterProjection`, so the substring `secret` is intentionally NOT flagged — the
/// scaffold matrix stores a redacted secret *reference*, never a raw value.
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

/// Builds the canonical, checked-in M05-1027 certification packet. Certifies all eight claimed M5
/// stack-entry and project-generation surfaces: four deliver their `QualifiedStarter` claim (green)
/// and four auto-narrow a not-current truth axis to a weaker projection ceiling (yellow). No surface
/// hides drift (red), no surface lets a generic `Create` hide a side effect, no surface exposes a
/// secret-bound raw value by default, and no surface drops its starter-source / support /
/// side-effect / recovery continuity.
pub fn seeded_m5_scaffold_component_certification_packet() -> ScaffoldSurfaceCertificationPacket {
    ScaffoldSurfaceCertificationPacket::new(ScaffoldSurfaceCertificationPacketInput {
        packet_id: "m5-scaffold-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: SCAFFOLD_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: SCAFFOLD_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:scaffold-component-certification:{id}"),
        SCAFFOLD_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        SCAFFOLD_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ScaffoldCertExportParity {
    ScaffoldCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_value_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ScaffoldCertificationAxis) -> &'static str {
    match axis {
        ScaffoldCertificationAxis::Visual => {
            "starter source/support class, host boundary, parameter source, immediate-versus-deferred action, file/dependency/task/extension impact, health freshness, and generated-versus-user-owned boundary shown on-surface"
        }
        ScaffoldCertificationAxis::Keyboard => {
            "the same source/side-effect/health/recovery truth and its actions are keyboard-reachable"
        }
        ScaffoldCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only, and the generated diff tree binds a flat list/textual path"
        }
        ScaffoldCertificationAxis::Export => {
            "surface state exports as text / JSON / Markdown for support and automation from the same component identity, without exposing a raw parameter value or secret material"
        }
        ScaffoldCertificationAxis::DegradedState => {
            "a drifted template freshness, a partial generation diff, a blocked prerequisite, a secret-bound parameter, or a cached/unchecked validation honestly downgrades the QualifiedStarter claim"
        }
        ScaffoldCertificationAxis::SourceSideEffectAndRecovery => {
            "starter source/support class, host and managed-workspace boundary, the concrete side effect behind any Create, the generated-versus-user-owned boundary, and the delete-generated/continue-without-starter recovery path stay explicit before any create, generate, or handoff; the surface never lets a generic Create hide a side effect, never exposes a secret-bound raw value by default, and never drops starter-source/support/side-effect/recovery continuity"
        }
    }
}

fn seed_certified(axis: ScaffoldCertificationAxis) -> ScaffoldAxisOutcome {
    ScaffoldAxisOutcome {
        axis,
        state: ScaffoldAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ScaffoldCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ScaffoldDowngradeTrigger,
) -> ScaffoldAxisOutcome {
    ScaffoldAxisOutcome {
        axis,
        state: ScaffoldAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ScaffoldAxisOutcome> {
    ScaffoldCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ScaffoldCertificationAxis,
    outcome: ScaffoldAxisOutcome,
) -> Vec<ScaffoldAxisOutcome> {
    ScaffoldCertificationAxis::ALL
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
    surface: M5ScaffoldCertifiedSurface,
    claimed_claim: M5ScaffoldComponentClaim,
    certified_claim: M5ScaffoldComponentClaim,
    consumed_families: &[M5ScaffoldComponentFamily],
    axis_outcomes: Vec<ScaffoldAxisOutcome>,
    claim_auto_narrow: Option<ScaffoldClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ScaffoldSurfaceCertificationRow {
    let mut row = ScaffoldSurfaceCertificationRow {
        record_kind: SCAFFOLD_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: SCAFFOLD_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        source_and_recovery_preserved: true,
        hides_side_effect_behind_generic_create: false,
        exposes_raw_value_by_default: false,
        canonical_bundle_ref: SCAFFOLD_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ScaffoldSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            SCAFFOLD_CERT_MATRIX_REF.to_owned(),
            SCAFFOLD_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-09T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ScaffoldCertificationAxis,
    from_claim: M5ScaffoldComponentClaim,
    to_claim: M5ScaffoldComponentClaim,
    label: &str,
) -> ScaffoldClaimAutoNarrow {
    ScaffoldClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_source_and_recovery_continuity: true,
    }
}

fn seeded_rows() -> Vec<ScaffoldSurfaceCertificationRow> {
    use M5ScaffoldCertifiedSurface as S;
    use M5ScaffoldComponentClaim::*;
    use M5ScaffoldComponentFamily::*;
    use M5ScaffoldDowngradeTrigger as Trig;
    use ScaffoldCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:start-center",
            S::StartCenter,
            QualifiedStarter,
            QualifiedStarter,
            &[ScaffoldTemplateCard, ScaffoldHandoffBanner],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "starter_source_and_support"],
            &[
                "the scaffold template card keeps its starter source class, support class, and host boundary explicit before it reads as a ready-to-run starter",
                "the scaffold handoff banner keeps its Run now / Run later / Review files choices and its delete-generated / reopen-preflight recovery route explicit rather than assuming the safest next step",
                "keyboard/screen-reader reach preserved for the scaffold template card and the scaffold handoff banner",
                "source-side-effect-and-recovery: the start center never routes creation through a generic Create that hides a side effect, and always keeps a Continue without starter path",
            ],
        ),
        seed_row(
            "cert:template-gallery",
            S::TemplateGallery,
            QualifiedStarter,
            QualifiedStarter,
            &[ScaffoldTemplateCard, StarterParameterRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "parameter_source"],
            &[
                "the scaffold template card keeps its source and support class explicit in the gallery rather than presenting a community or mirrored starter as governed first-party",
                "the starter parameter row keeps its source-precedence origin (template default / user input / workspace value / policy value / secret reference) explicit and never reveals a raw value",
                "keyboard/screen-reader reach preserved for the scaffold template card and the starter parameter row",
                "source-side-effect-and-recovery: the gallery never presents a secret-bound value as portable user input and never exposes a raw value by default",
            ],
        ),
        seed_row(
            "cert:workspace-handoff",
            S::WorkspaceHandoff,
            QualifiedStarter,
            QualifiedStarter,
            &[ScaffoldHandoffBanner, GeneratedProjectDiffCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "generated_versus_user_owned_boundary"],
            &[
                "the scaffold handoff banner keeps its created-workspace identity, trust state, and health summary explicit after Aureline writes files",
                "the generated-project diff card keeps its created / modified / renamed / deleted counts and its generated-versus-user-owned boundary explicit rather than presenting generated output as free-to-overwrite user-owned work",
                "keyboard/screen-reader reach preserved for the scaffold handoff banner and the generated-project diff card",
                "source-side-effect-and-recovery: the handoff keeps a rollback / delete-generated recovery route reachable and never assumes the safest next step for the user",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            QualifiedStarter,
            QualifiedStarter,
            &[ScaffoldTemplateCard, TemplateHealthRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "health_freshness"],
            &[
                "support export reconstructs source / support / side-effect / health / recovery truth from the same component identity",
                "the scaffold template card keeps its source and support class explicit in the exported record rather than leaking a raw parameter value",
                "the template health row keeps its check status, freshness, and Blocker / Warning / Info severity explicit in the exported record",
                "source-side-effect-and-recovery: a scaffold export never carries raw parameter values, secret material, or generated file bytes",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:scaffold-preflight",
            S::ScaffoldPreflight,
            QualifiedStarter,
            BlockedPrerequisiteProjection,
            &[ScaffoldPreflightCard],
            seed_certified_except(
                Ax::SourceSideEffectAndRecovery,
                seed_narrowed(
                    Ax::SourceSideEffectAndRecovery,
                    "a prerequisite health check is blocked and its host / managed-workspace boundary must be named rather than presenting the preflight as passed",
                    "The scaffold-preflight surface resolves a blocked prerequisite, so the QualifiedStarter claim narrows to blocked-prerequisite-projection with its blocked check, host boundary, and Create empty / continue-without-starter recovery preserved instead of reading as a passed preflight",
                    Trig::HostBoundaryUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceSideEffectAndRecovery,
                QualifiedStarter,
                BlockedPrerequisiteProjection,
                "Prerequisite blocked: the scaffold preflight card names the blocked check and its host boundary and keeps a Create empty / Continue without starter path rather than presenting a passed preflight or routing creation through a generic Create",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the scaffold preflight card keeps its generated file / folder counts, its side-effect disclosure, and its blocked-check recovery path explicit rather than hiding a side effect behind a generic Create",
                "the scaffold preflight card keeps a same-weight Create empty / Continue without starter path reachable while the prerequisite stays blocked",
                "source-side-effect-and-recovery: QualifiedStarter narrows to blocked-prerequisite-projection (auto-narrowed)",
                "known compatibility note: blocked-prerequisite behavior — a blocked preflight never reads as a passed, ready-to-run starter",
            ],
        ),
        seed_row(
            "cert:template-health",
            S::TemplateHealth,
            QualifiedStarter,
            DriftedTemplateProjection,
            &[TemplateHealthRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the template freshness has drifted / is stale and cannot claim a currently-fresh starter",
                    "The template-health surface resolves a drifted freshness, so the QualifiedStarter claim narrows to drifted-template-projection with its last-known freshness and source preserved instead of implying a currently-fresh template",
                    Trig::HealthFreshnessStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                QualifiedStarter,
                DriftedTemplateProjection,
                "Template freshness drifted: the template health row preserves its last-known freshness and source and shows the signal is stale rather than presenting a currently-fresh starter",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the template health row keeps its check status, last-checked freshness, and source explicit rather than presenting a drifted signal as fresh",
                "the template health row keeps its rerun-check / open-detail actions reachable while the drift is disclosed",
                "degraded-state: QualifiedStarter narrows to drifted-template-projection (auto-narrowed)",
                "known compatibility note: drifted-template behavior — a stale template-health signal never reads as a currently-fresh starter",
            ],
        ),
        seed_row(
            "cert:generation-diff-review",
            S::GenerationDiffReview,
            QualifiedStarter,
            PartialGenerationProjection,
            &[GeneratedProjectDiffCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the generation diff truth is only partial and cannot claim a clean applied change",
                    "The generation-diff-review surface resolves a partial generation diff, so the QualifiedStarter claim narrows to partial-generation-projection with its generated-versus-user-owned boundary and rollback / delete-generated recovery preserved instead of implying a clean applied change",
                    Trig::GeneratedBoundaryBlurred,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                QualifiedStarter,
                PartialGenerationProjection,
                "Generation diff partial: the generated-project diff card keeps its generated-versus-user-owned boundary and a rollback / delete-generated recovery path explicit rather than presenting a partial diff as a clean applied change",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the generated-project diff card keeps its created / modified / renamed / deleted counts and its generated-versus-user-owned boundary explicit rather than blurring the boundary",
                "the generated-project diff card keeps its rollback / delete-generated recovery path reachable while the partial state is disclosed",
                "degraded-state: QualifiedStarter narrows to partial-generation-projection (auto-narrowed)",
                "known compatibility note: partial-generation behavior — a partial generation diff never reads as a clean applied change",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            QualifiedStarter,
            SecretBoundParameterProjection,
            &[StarterParameterRow],
            seed_certified_except(
                Ax::SourceSideEffectAndRecovery,
                seed_narrowed(
                    Ax::SourceSideEffectAndRecovery,
                    "a starter parameter is bound to a secret reference that cannot travel to the headless context and its raw value must never be exported",
                    "The CLI-headless surface resolves a secret-bound starter parameter, so the QualifiedStarter claim narrows to secret-bound-parameter-projection that names the parameter and its source layer instead of exporting or committing a raw value",
                    Trig::ParameterSourceUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceSideEffectAndRecovery,
                QualifiedStarter,
                SecretBoundParameterProjection,
                "Parameter secret-bound: the starter parameter row names the parameter and its source layer and keeps the raw value redacted rather than exporting or committing it in the headless run",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the starter parameter row keeps its source-precedence origin and secret-reference class explicit rather than presenting a secret-bound value as portable user input",
                "the starter parameter row keeps the raw value redacted in the headless export while naming its source layer",
                "source-side-effect-and-recovery: QualifiedStarter narrows to secret-bound-parameter-projection (auto-narrowed)",
                "known compatibility note: secret-bound-parameter behavior — a secret-bound parameter never travels or reads as a portable, exportable value",
            ],
        ),
    ]
}
