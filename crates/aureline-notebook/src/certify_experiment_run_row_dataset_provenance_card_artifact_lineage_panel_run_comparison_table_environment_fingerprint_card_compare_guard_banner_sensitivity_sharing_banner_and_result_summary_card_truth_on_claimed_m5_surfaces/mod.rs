//! M05-1019 closing surface certification over the frozen M5 experiment-run-row /
//! dataset-provenance-card / artifact-lineage-panel / run-comparison-table /
//! environment-fingerprint-card / compare-guard-banner / sensitivity-sharing-banner /
//! result-summary-card component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`])
//! defines the eight reusable experiment-run-row, dataset-provenance-card,
//! artifact-lineage-panel, run-comparison-table, environment-fingerprint-card,
//! compare-guard-banner, sensitivity-sharing-banner, and result-summary-card components, the
//! M05-1013..1016 primitive lanes narrow each one, the M05-1017 consumer lane
//! ([`crate::add_shared_notebook_task_test_eval_review_support_and_export_consumers_so_experiment_components_keep_provenance_sensitivity_and_comparison_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed notebook-run-history / task-test-eval /
//! review-evidence / compare-view / companion-summary / CLI-headless-export / support-export
//! consumers, and the M05-1018 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_dataset_sensitivity_blocks_preview_lineage_is_stale_or_reproducibility_evidence_is_incomplete_across_claimed_m5_experiment_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared experiment-component truth holds on every claimed M5
//! notebook-adjacent and data-workflow surface — and auto-narrows any surface that cannot
//! sustain it.
//!
//! It is keyed on the claimed **surface** a user actually reviews, compares, shares, or
//! escalates a result on (the notebook experiment-run surface, the experiment dashboard, the
//! run-comparison surface, the data catalog, the artifact-lineage surface, the review-evidence
//! surface, the support / export bundle, and the CLI / headless surface), not on component
//! family or primitive lane. Each [`ExperimentSurfaceCertificationRow`] certifies one surface
//! across six truth axes — visual, keyboard, screen-reader, export, degraded-state, and
//! provenance-and-comparability — and either passes (green), auto-narrows its result claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden
//! behind a full-truth claim inherited from a healthier experiment lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps an `ExactComparableResult` / `ReviewableResult` claim while one of its
//! truth axes is not current — the artifact lineage is stale, the comparison evidence is
//! incomplete, the environment fingerprint is only partially captured, or a dataset sensitivity
//! class blocks the raw preview — is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its result claim (with a bound reason and a frozen downgrade trigger)
//! is honestly yellow. Experiment truth never loses lineage: a narrowed surface always
//! preserves its run-origin / code-revision / dataset-provenance / sensitivity /
//! comparability-and-confounder / export-scope lineage continuity rather than dropping it
//! between an experiment run row, a lineage panel, and an exported result summary. The always-on
//! export axis must always stay certified, so support and automation can reconstruct the same
//! run / dataset / lineage / comparison / fingerprint / sensitivity / summary truth from the same
//! component identity the user saw. No certified surface may imply an apples-to-apples
//! comparison without parity evidence, and no certified surface may expose raw production-like
//! data by default: comparisons stay parity-backed and previews stay metadata-only.
//!
//! Every row cites exactly one canonical experiment-component proof bundle
//! ([`EXPERIMENT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof —
//! rather than cloning per-surface evidence. The packet is metadata-only: raw dataset payloads,
//! captured output bytes, raw model weights, credentials, and raw production-like data never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-experiment-component-certification.schema.json`](../../../../schemas/ui/m5-experiment-component-certification.schema.json).
//! The contract doc is
//! [`docs/notebooks/m5_experiment_component_certification_contract.md`](../../../../docs/notebooks/m5_experiment_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_notebook_task_test_eval_review_support_and_export_consumers_so_experiment_components_keep_provenance_sensitivity_and_comparison_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_dataset_sensitivity_blocks_preview_lineage_is_stale_or_reproducibility_evidence_is_incomplete_across_claimed_m5_experiment_components as a11y;
use a11y::M5ExperimentComponentClaim;
use matrix::{M5ExperimentComponentFamily, M5ExperimentDowngradeTrigger};

/// Schema version stamped on the M05-1019 certification packet.
pub const EXPERIMENT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ExperimentSurfaceCertificationPacket`].
pub const EXPERIMENT_CERT_RECORD_KIND: &str = "m5_experiment_component_certification_packet";

/// Stable record-kind tag carried by each [`ExperimentSurfaceCertificationRow`].
pub const EXPERIMENT_CERT_ROW_RECORD_KIND: &str = "m5_experiment_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const EXPERIMENT_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-experiment-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const EXPERIMENT_CERT_DOC_REF: &str =
    "docs/notebooks/m5_experiment_component_certification_contract.md";

/// Repo-relative path of the frozen experiment-component matrix schema the certified surfaces
/// render.
pub const EXPERIMENT_CERT_MATRIX_REF: &str = matrix::M5_EXPERIMENT_COMPONENT_SCHEMA_REF;

/// The one canonical experiment-component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const EXPERIMENT_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_EXPERIMENT_COMPONENT_ARTIFACT_REF;

/// The M05-1017 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const EXPERIMENT_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_EXPERIMENT_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-1018 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every
/// row.
pub const EXPERIMENT_CERT_A11Y_BUNDLE_REF: &str =
    a11y::EXPERIMENT_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const EXPERIMENT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-experiment-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXPERIMENT_CERT_CSV_REF: &str =
    "artifacts/release/m5-experiment-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EXPERIMENT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-experiment-component-certification/report.md";

/// The eight claimed M5 notebook-adjacent and data-workflow surfaces this capstone certifies.
/// Keyed on the surface a user actually reviews, compares, shares, or escalates a result on, not
/// on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentCertifiedSurface {
    /// The notebook experiment-run surface (notebook run history / run rows).
    NotebookExperimentRun,
    /// The experiment dashboard surface.
    ExperimentDashboard,
    /// The run-comparison surface (compare view).
    RunComparison,
    /// The data-catalog surface (dataset provenance / data lanes).
    DataCatalog,
    /// The artifact-lineage surface.
    ArtifactLineage,
    /// The review-evidence surface (review workspace).
    ReviewEvidence,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5ExperimentCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ExperimentCertifiedSurface; 8] = [
        M5ExperimentCertifiedSurface::NotebookExperimentRun,
        M5ExperimentCertifiedSurface::ExperimentDashboard,
        M5ExperimentCertifiedSurface::RunComparison,
        M5ExperimentCertifiedSurface::DataCatalog,
        M5ExperimentCertifiedSurface::ArtifactLineage,
        M5ExperimentCertifiedSurface::ReviewEvidence,
        M5ExperimentCertifiedSurface::SupportExport,
        M5ExperimentCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookExperimentRun => "notebook_experiment_run",
            Self::ExperimentDashboard => "experiment_dashboard",
            Self::RunComparison => "run_comparison",
            Self::DataCatalog => "data_catalog",
            Self::ArtifactLineage => "artifact_lineage",
            Self::ReviewEvidence => "review_evidence",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, export, degraded-state, and
/// provenance-and-comparability. The export axis is always-on and must stay certified for every
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentCertificationAxis {
    /// Visual parity: the run origin / revision, dataset provenance, artifact lineage,
    /// comparability, environment fingerprint, sensitivity class, and export scope are shown on
    /// the primary surface.
    Visual,
    /// Keyboard-reach parity: the same run / dataset / lineage / comparison / sensitivity truth
    /// and its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or
    /// a status glyph alone.
    ScreenReader,
    /// Export parity (always-on): the certified surface state is reconstructable as text / JSON /
    /// Markdown for support and automation, from the same component identity, without exposing a
    /// raw payload.
    Export,
    /// Degraded-state parity: a stale artifact lineage, incomplete comparison evidence, a
    /// partially captured environment fingerprint, a severed dataset provenance, a blocking
    /// compare guard, or a sensitivity-blocked preview honestly downgrades an
    /// `ExactComparableResult` / `ReviewableResult` claim to a weaker result tier.
    DegradedState,
    /// Provenance-and-comparability parity: the run origin, code revision, environment
    /// fingerprint, dataset provenance, sensitivity state, comparability / confounder
    /// disclosure, and summary-versus-evidence-versus-raw export scope stay explicit before any
    /// compare, review, share, or export — never inheriting a healthier lane's result truth,
    /// never masking a stale lineage, incomparable runs, partial fingerprint, or blocked preview
    /// as an exact-comparable surface, never implying an apples-to-apples comparison without
    /// parity evidence, never exposing raw production-like data by default, and never dropping
    /// run-origin / dataset-provenance / lineage / export-scope lineage between an experiment run
    /// row, a lineage panel, and an exported result summary.
    ProvenanceAndComparability,
}

impl ExperimentCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ExperimentCertificationAxis; 6] = [
        ExperimentCertificationAxis::Visual,
        ExperimentCertificationAxis::Keyboard,
        ExperimentCertificationAxis::ScreenReader,
        ExperimentCertificationAxis::Export,
        ExperimentCertificationAxis::DegradedState,
        ExperimentCertificationAxis::ProvenanceAndComparability,
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
            Self::ProvenanceAndComparability => "provenance_and_comparability",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl ExperimentAxisCertificationState {
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
pub enum ExperimentSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed result tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, export parity drops, lineage is
    /// dropped, an apples-to-apples comparison is implied without parity, a raw payload is
    /// exposed by default, or the narrowing is inconsistent.
    Red,
}

impl ExperimentSurfaceClaimStatus {
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

/// The copy / export parity a certified surface preserves. The export axis certifies only when
/// this offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The run / dataset / lineage / comparison / fingerprint / sensitivity / summary fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited (the export reconstructs metadata rather
    /// than dumping raw dataset / output / weight bytes).
    pub raw_payload_only_prohibited: bool,
}

impl ExperimentCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only
    /// export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ExperimentCertificationAxis,
    /// The certification state of the axis.
    pub state: ExperimentAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ExperimentDowngradeTrigger>,
}

impl ExperimentAxisOutcome {
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
            ExperimentAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ExperimentAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ExperimentAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present iff
/// the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ExperimentCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5ExperimentComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5ExperimentComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its run-origin / dataset-provenance /
    /// lineage / export-scope lineage continuity rather than dropping it between an experiment run
    /// row, a lineage panel, and an exported result summary.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 notebook-adjacent / data-workflow surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentSurfaceCertificationRow {
    /// Record kind; must equal [`EXPERIMENT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXPERIMENT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ExperimentCertifiedSurface,
    /// The result-claim ceiling the surface asserts.
    pub claimed_claim: M5ExperimentComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5ExperimentComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ExperimentComponentFamily>,
    /// One outcome per [`ExperimentCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ExperimentAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ExperimentClaimAutoNarrow>,
    /// True when this surface never drops its run-origin / dataset-provenance / lineage /
    /// export-scope lineage continuity between an experiment run row, a lineage panel, and an
    /// exported result summary.
    pub lineage_preserved: bool,
    /// True iff this surface implies an apples-to-apples comparison without parity evidence. A
    /// certified surface MUST keep this false: metric deltas never read as a fair baseline unless
    /// parity is proven.
    pub implies_apples_to_apples_without_parity: bool,
    /// True iff this surface exposes raw production-like data by default in a review / share /
    /// export flow. A certified surface MUST keep this false: previews stay metadata-only and raw
    /// payloads are opt-in.
    pub exposes_raw_payload_by_default: bool,
    /// The one canonical experiment proof bundle this surface cites. Must equal
    /// [`EXPERIMENT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ExperimentSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: ExperimentCertExportParity,
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

impl ExperimentSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ExperimentCertificationAxis) -> Option<&ExperimentAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ExperimentCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ExperimentCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ExperimentAxisOutcome::well_formed)
    }

    /// True when the surface narrows its result claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ExperimentCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ExperimentAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its run-origin / dataset-provenance / lineage /
    /// export-scope lineage continuity rather than dropping it. A non-narrowed surface trivially
    /// preserves lineage; a narrowed one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, export parity must always
    /// certify, experiment truth must never drop lineage, imply an apples-to-apples comparison
    /// without parity, or expose raw production-like data by default, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> ExperimentSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != EXPERIMENT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
            || self.implies_apples_to_apples_without_parity
            || self.exposes_raw_payload_by_default
        {
            return ExperimentSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ExperimentSurfaceClaimStatus::Red;
        }

        // The always-on export axis must stay certified.
        match self.axis(ExperimentCertificationAxis::Export) {
            Some(o) if o.state == ExperimentAxisCertificationState::Certified => {}
            _ => return ExperimentSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ExperimentAxisCertificationState::UndisclosedDrift)
        {
            return ExperimentSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ExperimentSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ExperimentSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return ExperimentSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ExperimentSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return ExperimentSurfaceClaimStatus::Red;
        }

        ExperimentSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EXPERIMENT_CERT_ROW_RECORD_KIND
            && self.schema_version == EXPERIMENT_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} lineage_preserved={preserved} implies_parity={implies} raw_payload={raw}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.lineage_preserved,
            implies = self.implies_apples_to_apples_without_parity,
            raw = self.exposes_raw_payload_by_default,
        )
    }
}

/// Rolled-up summary of an M05-1019 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentSurfaceCertificationSummary {
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
    pub all_lineage_preserved: bool,
    pub no_surface_implies_unproven_parity: bool,
    pub no_surface_exposes_raw_payload: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`ExperimentSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ExperimentSurfaceCertificationRow>,
}

/// Checked-in M05-1019 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ExperimentSurfaceCertificationRow>,
    pub summary: ExperimentSurfaceCertificationSummary,
}

impl ExperimentSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ExperimentSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EXPERIMENT_CERT_SCHEMA_VERSION,
            record_kind: EXPERIMENT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ExperimentSurfaceCertificationSummary {
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
                all_lineage_preserved: false,
                no_surface_implies_unproven_parity: false,
                no_surface_exposes_raw_payload: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ExperimentCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ExperimentComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ExperimentCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ExperimentComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether an export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ExperimentCertificationAxis::Export)
                .is_some_and(|o| o.state == ExperimentAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ExperimentSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ExperimentSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ExperimentSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ExperimentSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ExperimentSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(ExperimentSurfaceCertificationRow::preserves_lineage_continuity);
        let no_unproven_parity = self
            .rows
            .iter()
            .all(|r| !r.implies_apples_to_apples_without_parity);
        let no_raw_payload = self.rows.iter().all(|r| !r.exposes_raw_payload_by_default);

        ExperimentSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == EXPERIMENT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ExperimentSurfaceCertificationRow::covers_all_axes),
            all_lineage_preserved: all_preserved,
            no_surface_implies_unproven_parity: no_unproven_parity,
            no_surface_exposes_raw_payload: no_raw_payload,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved
                && no_unproven_parity
                && no_raw_payload,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ExperimentCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EXPERIMENT_CERT_SCHEMA_VERSION {
            violations.push(ExperimentCertificationViolation::SchemaVersion {
                expected: EXPERIMENT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXPERIMENT_CERT_RECORD_KIND {
            violations.push(ExperimentCertificationViolation::RecordKind {
                expected: EXPERIMENT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ExperimentCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != EXPERIMENT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ExperimentCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ExperimentCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ExperimentCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ExperimentCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ExperimentCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != EXPERIMENT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ExperimentCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ExperimentCertificationAxis::Export)
                    .is_none_or_state_not_certified()
            {
                violations.push(ExperimentCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Experiment truth must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(ExperimentCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // No certified surface may imply an apples-to-apples comparison without parity.
            if row.implies_apples_to_apples_without_parity {
                violations.push(
                    ExperimentCertificationViolation::ApplesToApplesImpliedWithoutParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No certified surface may expose raw production-like data by default.
            if row.exposes_raw_payload_by_default {
                violations.push(
                    ExperimentCertificationViolation::RawPayloadExposedByDefault {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ExperimentCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ExperimentCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == ExperimentSurfaceClaimStatus::Red {
                violations.push(ExperimentCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(ExperimentCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(ExperimentCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ExperimentCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ExperimentCertificationViolation::RawExperimentPayloadInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,lineage_preserved,implies_parity,exposes_raw_payload\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved},{implies},{raw}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.lineage_preserved,
                implies = row.implies_apples_to_apples_without_parity,
                raw = row.exposes_raw_payload_by_default,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Experiment Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ExperimentCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Lineage preserved on every surface: {}\n",
            self.summary.all_lineage_preserved
        ));
        out.push_str(&format!(
            "- No surface implies apples-to-apples without parity: {}\n",
            self.summary.no_surface_implies_unproven_parity
        ));
        out.push_str(&format!(
            "- No surface exposes raw payload by default: {}\n",
            self.summary.no_surface_exposes_raw_payload
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
pub fn current_m5_experiment_component_certification_export(
) -> Result<ExperimentSurfaceCertificationPacket, ExperimentCertificationArtifactError> {
    let packet: ExperimentSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-certification/support_export.json"
    )))
    .map_err(ExperimentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExperimentCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ExperimentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExperimentCertificationViolation>),
}

impl fmt::Display for ExperimentCertificationArtifactError {
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

impl Error for ExperimentCertificationArtifactError {}

/// Validation failure for M05-1019 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentCertificationViolation {
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
    LineageDropped { id: String },
    ApplesToApplesImpliedWithoutParity { id: String },
    RawPayloadExposedByDefault { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawExperimentPayloadInExport,
}

impl fmt::Display for ExperimentCertificationViolation {
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
                    "packet does not cite the canonical experiment-component proof bundle"
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
                    "row {id} does not cite the one canonical experiment-component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} drops run-origin / dataset-provenance / lineage / export-scope lineage continuity (a narrowed surface must preserve its lineage between an experiment run row, a lineage panel, and an exported result summary)"
                )
            }
            Self::ApplesToApplesImpliedWithoutParity { id } => {
                write!(
                    f,
                    "row {id} implies an apples-to-apples comparison without parity evidence (a metric delta must never read as a fair baseline unless parity is proven)"
                )
            }
            Self::RawPayloadExposedByDefault { id } => {
                write!(
                    f,
                    "row {id} exposes raw production-like data by default (previews must stay metadata-only and raw payloads must be opt-in)"
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
export parity dropped, lineage was dropped, an apples-to-apples comparison was implied without \
parity, a raw payload was exposed by default, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 notebook-adjacent / data-workflow surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen experiment-component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawExperimentPayloadInExport => {
                write!(f, "export contains raw experiment payload material")
            }
        }
    }
}

impl Error for ExperimentCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ExperimentAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ExperimentAxisCertificationState::Certified,
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
            | "incomparable"
            | "unprovenanced"
            | "not installed"
            | "not_installed"
            | "local only"
            | "local_only"
            | "no provenance"
            | "no_provenance"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
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

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1019 certification packet. Certifies all eight claimed M5
/// notebook-adjacent and data-workflow surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker result ceiling (yellow). No surface hides
/// drift (red), no surface implies an apples-to-apples comparison without parity, no surface
/// exposes raw production-like data by default, and no surface drops run-origin /
/// dataset-provenance / lineage / export-scope lineage.
pub fn seeded_m5_experiment_component_certification_packet() -> ExperimentSurfaceCertificationPacket
{
    ExperimentSurfaceCertificationPacket::new(ExperimentSurfaceCertificationPacketInput {
        packet_id: "m5-experiment-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: EXPERIMENT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: EXPERIMENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:experiment-component-certification:{id}"),
        EXPERIMENT_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        EXPERIMENT_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ExperimentCertExportParity {
    ExperimentCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ExperimentCertificationAxis) -> &'static str {
    match axis {
        ExperimentCertificationAxis::Visual => {
            "run origin/revision, dataset provenance, artifact lineage, comparability, environment fingerprint, sensitivity class, and export scope shown on-surface"
        }
        ExperimentCertificationAxis::Keyboard => {
            "the same run/dataset/lineage/comparison/sensitivity truth and its actions are keyboard-reachable"
        }
        ExperimentCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        ExperimentCertificationAxis::Export => {
            "surface state exports as text / JSON / Markdown for support and automation from the same component identity, without exposing a raw payload"
        }
        ExperimentCertificationAxis::DegradedState => {
            "a stale artifact lineage, incomplete comparison evidence, a partially captured environment fingerprint, a severed dataset provenance, or a sensitivity-blocked preview honestly downgrades the ExactComparableResult/ReviewableResult claim"
        }
        ExperimentCertificationAxis::ProvenanceAndComparability => {
            "run origin, code revision, environment fingerprint, dataset provenance, sensitivity state, comparability/confounder disclosure, and export scope stay explicit before any compare, review, share, or export; the surface never implies apples-to-apples without parity, never exposes raw payloads by default, and never drops run-origin/dataset-provenance/lineage/export-scope lineage"
        }
    }
}

fn seed_certified(axis: ExperimentCertificationAxis) -> ExperimentAxisOutcome {
    ExperimentAxisOutcome {
        axis,
        state: ExperimentAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ExperimentCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ExperimentDowngradeTrigger,
) -> ExperimentAxisOutcome {
    ExperimentAxisOutcome {
        axis,
        state: ExperimentAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ExperimentAxisOutcome> {
    ExperimentCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ExperimentCertificationAxis,
    outcome: ExperimentAxisOutcome,
) -> Vec<ExperimentAxisOutcome> {
    ExperimentCertificationAxis::ALL
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
    surface: M5ExperimentCertifiedSurface,
    claimed_claim: M5ExperimentComponentClaim,
    certified_claim: M5ExperimentComponentClaim,
    consumed_families: &[M5ExperimentComponentFamily],
    axis_outcomes: Vec<ExperimentAxisOutcome>,
    claim_auto_narrow: Option<ExperimentClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ExperimentSurfaceCertificationRow {
    let mut row = ExperimentSurfaceCertificationRow {
        record_kind: EXPERIMENT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: EXPERIMENT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        implies_apples_to_apples_without_parity: false,
        exposes_raw_payload_by_default: false,
        canonical_bundle_ref: EXPERIMENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ExperimentSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            EXPERIMENT_CERT_MATRIX_REF.to_owned(),
            EXPERIMENT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-09T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ExperimentCertificationAxis,
    from_claim: M5ExperimentComponentClaim,
    to_claim: M5ExperimentComponentClaim,
    label: &str,
) -> ExperimentClaimAutoNarrow {
    ExperimentClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<ExperimentSurfaceCertificationRow> {
    use ExperimentCertificationAxis as Ax;
    use M5ExperimentCertifiedSurface as S;
    use M5ExperimentComponentClaim::*;
    use M5ExperimentComponentFamily::*;
    use M5ExperimentDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:notebook-experiment-run",
            S::NotebookExperimentRun,
            ExactComparableResult,
            ExactComparableResult,
            &[ExperimentRunRow, EnvironmentFingerprintCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "run_origin_and_revision"],
            &[
                "the experiment run row keeps its run origin, notebook/script/task lineage, and code revision explicit before it reads as a comparable run",
                "the environment fingerprint card keeps its captured interpreter/package/hardware fingerprint explicit rather than implying an untracked environment",
                "keyboard/screen-reader reach preserved for the experiment run row and the environment fingerprint card",
                "provenance: a notebook experiment-run surface never presents a run it cannot trace to an origin and revision, and never implies apples-to-apples without parity",
            ],
        ),
        seed_row(
            "cert:experiment-dashboard",
            S::ExperimentDashboard,
            ReviewableResult,
            ReviewableResult,
            &[ResultSummaryCard, ExperimentRunRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "export_scope"],
            &[
                "the result summary card keeps its summary-versus-evidence-versus-raw export scope explicit while the dashboard renders it",
                "the experiment run row keeps its run origin and status explicit on the dashboard rather than implying a managed tracker",
                "keyboard/screen-reader reach preserved for the result summary card and the experiment run row",
                "provenance: the dashboard never shows a summary whose export scope it cannot name and never exposes a raw payload by default",
            ],
        ),
        seed_row(
            "cert:review-evidence",
            S::ReviewEvidence,
            ReviewableResult,
            ReviewableResult,
            &[ResultSummaryCard, ArtifactLineagePanel],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "artifact_lineage"],
            &[
                "the result summary card keeps its reviewable, read-only summary and export scope explicit for the reviewer",
                "the artifact lineage panel keeps its producing-run identity and current lineage explicit rather than presenting an anonymous artifact",
                "keyboard/screen-reader reach preserved for the result summary card and the artifact lineage panel",
                "provenance: review evidence never presents an artifact without its producing run and never exposes a raw payload by default",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableResult,
            ReviewableResult,
            &[ResultSummaryCard, DatasetProvenanceCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "provenance_and_sensitivity"],
            &[
                "support export reconstructs run/dataset/lineage/comparison/fingerprint/sensitivity/summary truth from the same component identity",
                "the result summary card keeps its metadata-only export scope explicit in the exported record rather than leaking a raw payload",
                "the dataset provenance card keeps its source class and sensitivity state explicit in the exported record",
                "provenance: an experiment export never carries raw dataset payloads, captured output bytes, model weights, or credentials",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:run-comparison",
            S::RunComparison,
            ExactComparableResult,
            IncomparableRunsProjection,
            &[RunComparisonTable, CompareGuardBanner],
            seed_certified_except(
                Ax::ProvenanceAndComparability,
                seed_narrowed(
                    Ax::ProvenanceAndComparability,
                    "the comparison evidence is incomplete and cannot claim an apples-to-apples fair baseline across the two runs",
                    "The run-comparison surface resolves two runs whose parity evidence is incomplete, so the ExactComparableResult claim narrows to incomparable-runs-projection instead of implying the metric delta is a fair baseline",
                    Trig::ComparabilityOverstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ProvenanceAndComparability,
                ExactComparableResult,
                IncomparableRunsProjection,
                "Comparison evidence is incomplete: the run comparison table discloses its confounders and unmatched factors rather than implying the metric delta is an apples-to-apples fair baseline",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the run comparison table keeps its baseline/candidate identity and confounder disclosure explicit rather than presenting an unproven fair delta",
                "the compare guard banner keeps its guard reason explicit while the comparison stays incomparable",
                "provenance-and-comparability: ExactComparableResult narrows to incomparable-runs-projection (auto-narrowed)",
                "known compatibility note: incomparable-runs behavior — an incomplete-parity comparison never reads as an apples-to-apples fair baseline",
            ],
        ),
        seed_row(
            "cert:data-catalog",
            S::DataCatalog,
            ExactComparableResult,
            BlockedPreviewProjection,
            &[DatasetProvenanceCard, SensitivitySharingBanner],
            seed_certified_except(
                Ax::ProvenanceAndComparability,
                seed_narrowed(
                    Ax::ProvenanceAndComparability,
                    "the dataset sensitivity class blocks the raw preview and cannot claim a raw-data-backed comparable result",
                    "The data-catalog surface resolves a dataset whose sensitivity class blocks preview, so the ExactComparableResult claim narrows to blocked-preview-projection instead of implying a raw-data preview is available",
                    Trig::SensitivityClassUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ProvenanceAndComparability,
                ExactComparableResult,
                BlockedPreviewProjection,
                "Dataset sensitivity blocks preview: the sensitivity/sharing banner stays metadata-only and shows the raw preview is withheld rather than exposing raw production-like data",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the dataset provenance card keeps its source class and provenance state explicit while the raw preview stays blocked",
                "the sensitivity/sharing banner keeps its sensitivity class and metadata-only share scope explicit rather than exposing a raw payload by default",
                "provenance-and-comparability: ExactComparableResult narrows to blocked-preview-projection (auto-narrowed)",
                "known compatibility note: blocked-preview behavior — a sensitivity-blocked dataset never reads as a raw-data-backed result",
            ],
        ),
        seed_row(
            "cert:artifact-lineage",
            S::ArtifactLineage,
            ExactComparableResult,
            StaleLineageProjection,
            &[ArtifactLineagePanel],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the artifact lineage is stale / cached and cannot claim a current producing-run lineage",
                    "The artifact-lineage surface resolves a stale lineage, so the ExactComparableResult claim narrows to stale-lineage-projection instead of implying the artifact matches a current producing run — its last-known producing run stays preserved",
                    Trig::CachedStateHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactComparableResult,
                StaleLineageProjection,
                "Artifact lineage is stale: the lineage panel preserves its last-known producing run and shows the lineage is cached rather than implying a current-lineage artifact",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the artifact lineage panel keeps its producing-run identity and stale/diverged notes explicit rather than presenting a current lineage",
                "the artifact lineage panel keeps its last-known lineage reachable while the cached state is disclosed",
                "degraded-state: ExactComparableResult narrows to stale-lineage-projection (auto-narrowed)",
                "known compatibility note: stale-lineage behavior — a stale artifact lineage never reads as a current producing-run lineage",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExactComparableResult,
            PartialFingerprintProjection,
            &[EnvironmentFingerprintCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the environment fingerprint is only partially captured in the headless context and cannot claim a fully captured fingerprint",
                    "The CLI-headless surface resolves an environment fingerprint that is only partially captured, so the ExactComparableResult claim narrows to partial-fingerprint-projection instead of implying a fully fingerprinted comparable run",
                    Trig::EnvironmentFingerprintUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactComparableResult,
                PartialFingerprintProjection,
                "Environment fingerprint is partial: the fingerprint card discloses which scopes are uncaptured rather than implying a fully fingerprinted, apples-to-apples comparable run",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the environment fingerprint card keeps its captured-versus-uncaptured scopes explicit rather than implying a complete fingerprint",
                "the environment fingerprint card keeps its partial-capture state reachable in the headless export",
                "degraded-state: ExactComparableResult narrows to partial-fingerprint-projection (auto-narrowed)",
                "known compatibility note: partial-fingerprint behavior — a partially captured fingerprint never reads as a fully fingerprinted comparable run",
            ],
        ),
    ]
}
