//! M05-1091 profile certification over the frozen M5 notebook document / kernel /
//! output / trust / recovery component matrix — the closing capstone of the B129 batch.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`])
//! defines the eight reusable notebook-document header, kernel-state strip, kernel-picker
//! row, kernel-origin pill, output-trust banner, output-provenance chip group,
//! restart-consequence card, and kernel-recovery card components, the four M05-1085..1088
//! implement lanes narrow each one, the M05-1090 consumer lane
//! ([`crate::wire_editor_diff_review_debug_ai_support_and_export_consumers_so_notebook_document_kernel_and_output_components_keep_one_vocabulary_across_claimed_m5_notebook_and_data_surfaces`])
//! adopts them, and the M05-1089 accessibility lane
//! ([`crate::add_large_output_virtualization_collapsed_output_summaries_keyboard_screen_reader_high_zoom_reduced_motion_export_parity_and_automatic_claim_narrowing_when_kernel_debug_parity_or_output_trust_evidence_weakens`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared component
//! truth holds on every claimed M5 local / remote / managed notebook profile — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **notebook runtime / output profile** a user, operator, or
//! support engineer reads notebook document / kernel / output / trust / recovery truth
//! through (a local trusted kernel, an isolated remote kernel, a managed kernel, a
//! trusted local output, a stale output, a degraded-origin kernel, a restarted kernel,
//! and a disconnected-then-reconnecting kernel), not on component family or implement
//! lane. Each [`NotebookProfileCertificationRow`] certifies one profile across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! notebook-truth behavior — and either passes (green), auto-narrows its
//! result claim to the weakest supported ceiling (yellow), or is blocked (red) when a
//! degraded axis is hidden behind a fresh live-trusted claim inherited from a healthier
//! profile, or a spec guardrail is violated.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A
//! profile that keeps a `LiveTrustedResult` / `ReviewableResult` claim while one of its
//! truth axes is not current is over-claiming and blocks; a profile that discloses the
//! reduction by narrowing its result claim (with a bound reason and a frozen downgrade
//! trigger) is honestly yellow. The always-on CLI/export axis must always stay certified,
//! so support and automation can reconstruct the certified document identity, kernel
//! origin / class / liveness, output trust class, output provenance, stale-vs-live
//! honesty, restart / reconnect consequence, and preserved-vs-lost recovery truth from
//! the same object identity the user saw. A stale, disconnected, or kernel-free profile
//! can never keep a fresh live-trusted claim, only the local trusted-kernel profile may
//! certify a live-trusted claim, and no notebook profile may let a recovery card imply a
//! rerun, present stale output as live, hide the raw / sanitized / active trust class
//! behind hover-only affordances, or collapse local, remote, and managed kernels into one
//! unlabeled badge.
//!
//! Every row cites exactly one canonical notebook-kernel-output proof bundle
//! ([`NOTEBOOK_CERT_CANONICAL_BUNDLE_REF`]) — the frozen notebook-kernel-output component
//! matrix proof — rather than cloning per-profile evidence. The packet is metadata-only:
//! raw notebook cell material, credential material, and bearer secrets never cross this
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-notebook-kernel-output-component-certification.schema.json`](../../../../schemas/ui/m5-notebook-kernel-output-component-certification.schema.json).
//! The contract doc is
//! [`docs/notebooks/m5_notebook_kernel_output_component_certification_contract.md`](../../../../docs/notebooks/m5_notebook_kernel_output_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_large_output_virtualization_collapsed_output_summaries_keyboard_screen_reader_high_zoom_reduced_motion_export_parity_and_automatic_claim_narrowing_when_kernel_debug_parity_or_output_trust_evidence_weakens as a11y;
use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix as matrix;
use a11y::M5NotebookComponentClaim;
use matrix::{M5NotebookKernelOutputComponentFamily, M5NotebookKernelOutputDowngradeTrigger};

/// Schema version stamped on the M05-1091 certification packet.
pub const NOTEBOOK_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NotebookProfileCertificationPacket`].
pub const NOTEBOOK_CERT_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_certification_packet";

/// Stable record-kind tag carried by each [`NotebookProfileCertificationRow`].
pub const NOTEBOOK_CERT_ROW_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const NOTEBOOK_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-kernel-output-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const NOTEBOOK_CERT_DOC_REF: &str =
    "docs/notebooks/m5_notebook_kernel_output_component_certification_contract.md";

/// Repo-relative path of the frozen notebook-kernel-output component matrix schema the
/// certified profiles render.
pub const NOTEBOOK_CERT_MATRIX_REF: &str = matrix::M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF;

/// The one canonical notebook-kernel-output proof bundle every certified profile cites as
/// its first-resolved component truth. All eight profiles point back to it rather than
/// cloning per-profile evidence.
pub const NOTEBOOK_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_ARTIFACT_REF;

/// The M05-1089 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const NOTEBOOK_CERT_A11Y_BUNDLE_REF: &str = a11y::NOTEBOOK_KERNEL_OUTPUT_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const NOTEBOOK_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NOTEBOOK_CERT_CSV_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NOTEBOOK_CERT_REPORT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-certification-proof/report.md";

/// The eight claimed M5 local / remote / managed notebook profiles this capstone
/// certifies. Keyed on the notebook runtime / output profile a user, operator, or support
/// engineer reads notebook document / kernel / output / trust / recovery truth through,
/// not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookCertifiedProfile {
    /// A local, first-party trusted kernel with a live, trusted output — the only profile
    /// that may certify a live-trusted claim.
    LocalTrustedKernel,
    /// An isolated remote kernel (SSH / container) with an explicit remote origin.
    RemoteIsolatedKernel,
    /// A managed-workspace kernel with an explicit managed origin.
    ManagedKernel,
    /// A trusted local output rendered through the sanitized / active trust classes.
    TrustedLocalOutput,
    /// An output whose trust evidence has gone stale and must not read as live.
    StaleOutput,
    /// A kernel whose origin is degraded (unstated or approximate).
    DegradedOriginKernel,
    /// A kernel that was restarted clean, clearing live results without a hidden rerun.
    RestartedKernel,
    /// A kernel that disconnected and is reconnecting with only partial parity.
    DisconnectedReconnectingKernel,
}

impl M5NotebookCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5NotebookCertifiedProfile; 8] = [
        M5NotebookCertifiedProfile::LocalTrustedKernel,
        M5NotebookCertifiedProfile::RemoteIsolatedKernel,
        M5NotebookCertifiedProfile::ManagedKernel,
        M5NotebookCertifiedProfile::TrustedLocalOutput,
        M5NotebookCertifiedProfile::StaleOutput,
        M5NotebookCertifiedProfile::DegradedOriginKernel,
        M5NotebookCertifiedProfile::RestartedKernel,
        M5NotebookCertifiedProfile::DisconnectedReconnectingKernel,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTrustedKernel => "local_trusted_kernel",
            Self::RemoteIsolatedKernel => "remote_isolated_kernel",
            Self::ManagedKernel => "managed_kernel",
            Self::TrustedLocalOutput => "trusted_local_output",
            Self::StaleOutput => "stale_output",
            Self::DegradedOriginKernel => "degraded_origin_kernel",
            Self::RestartedKernel => "restarted_kernel",
            Self::DisconnectedReconnectingKernel => "disconnected_reconnecting_kernel",
        }
    }

    /// True for the single local, first-party trusted-kernel profile — the only one that
    /// may certify a live `LiveTrustedResult` claim.
    pub const fn is_local_first_party(self) -> bool {
        matches!(self, Self::LocalTrustedKernel)
    }
}

/// The six truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and notebook-truth behavior. The CLI/export axis is always-on and must
/// stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookCertificationAxis {
    /// Visual parity: document identity, kernel origin / class / liveness, output trust
    /// class, output provenance, restart / reconnect consequence, and preserved-vs-lost
    /// recovery state are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same notebook truth and its actions (select kernel,
    /// inspect origin, open raw output, reconnect, restart-clean, choose-another-kernel,
    /// export) are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color, a status glyph, or a hover-only affordance alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale, disconnected, or kernel-free notebook reading
    /// honestly downgrades a `LiveTrustedResult` / `ReviewableResult` claim rather than
    /// reading as a fresh live-trusted result.
    DegradedState,
    /// Notebook-truth parity: document identity, kernel origin / class / liveness, output
    /// trust class, output provenance, stale-vs-live honesty, restart / reconnect
    /// consequence, and preserved-vs-lost recovery stay explicit and never let a recovery
    /// card imply a rerun, present stale output as live, hide the trust class behind
    /// hover-only affordances, or collapse local / remote / managed kernels into one
    /// unlabeled badge.
    NotebookTruth,
}

impl NotebookCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [NotebookCertificationAxis; 6] = [
        NotebookCertificationAxis::Visual,
        NotebookCertificationAxis::Keyboard,
        NotebookCertificationAxis::ScreenReader,
        NotebookCertificationAxis::CliExport,
        NotebookCertificationAxis::DegradedState,
        NotebookCertificationAxis::NotebookTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::NotebookTruth => "notebook_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a live-trusted claim
    /// inherited from a healthier profile.
    UndisclosedDrift,
}

impl NotebookAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author —
/// always recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookProfileClaimStatus {
    /// Full standing: every axis certified, claimed result tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, a
    /// guardrail is violated, or the narrowing is inconsistent.
    Red,
}

impl NotebookProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red
    /// profiles block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The four B129 spec guardrails, evaluated per certified profile. All must stay false;
/// any true blocks the profile (red) regardless of axis parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCertGuardrails {
    /// A kernel-recovery card implies a rerun happened on restore / recovery.
    pub recovery_card_implies_rerun: bool,
    /// A stale output is presented as live truth.
    pub presents_stale_output_as_live: bool,
    /// The raw / sanitized / active trust class is hidden behind a hover-only affordance.
    pub hides_trust_class_behind_hover_only: bool,
    /// Local, remote, and managed kernels are collapsed into one unlabeled badge.
    pub collapses_kernel_origins_into_one_badge: bool,
}

impl NotebookCertGuardrails {
    /// A clean, all-false guardrail set.
    pub const CLEAN: NotebookCertGuardrails = NotebookCertGuardrails {
        recovery_card_implies_rerun: false,
        presents_stale_output_as_live: false,
        hides_trust_class_behind_hover_only: false,
        collapses_kernel_origins_into_one_badge: false,
    };

    /// True when every guardrail is held (all false).
    pub const fn all_held(self) -> bool {
        !self.recovery_card_implies_rerun
            && !self.presents_stale_output_as_live
            && !self.hides_trust_class_behind_hover_only
            && !self.collapses_kernel_origins_into_one_badge
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies
/// only when this offers text / JSON / Markdown reconstruction and prohibits a
/// screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The document-identity / kernel-origin / kernel-liveness / output-trust /
    /// output-provenance / restart-consequence / recovery-continuity fields the profile
    /// preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl NotebookCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: NotebookCertificationAxis,
    /// The certification state of the axis.
    pub state: NotebookAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5NotebookKernelOutputDowngradeTrigger>,
}

impl NotebookAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            NotebookAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            NotebookAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            NotebookAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present
/// iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: NotebookCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5NotebookComponentClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5NotebookComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 local / remote / managed notebook profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookProfileCertificationRow {
    /// Record kind; must equal [`NOTEBOOK_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NOTEBOOK_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5NotebookCertifiedProfile,
    /// The result claim ceiling the profile asserts.
    pub claimed_claim: M5NotebookComponentClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5NotebookComponentClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5NotebookKernelOutputComponentFamily>,
    /// One outcome per [`NotebookCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<NotebookAxisOutcome>,
    /// The four spec guardrails for this profile; all must be held (false).
    pub guardrails: NotebookCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<NotebookClaimAutoNarrow>,
    /// The one canonical notebook-kernel-output proof bundle this profile cites. Must equal
    /// [`NOTEBOOK_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: NotebookProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: NotebookCertExportParity,
    /// The compatibility notes captured for this profile.
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

impl NotebookProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: NotebookCertificationAxis) -> Option<&NotebookAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<NotebookCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && NotebookCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(NotebookAxisOutcome::well_formed)
    }

    /// True when the profile narrows its result claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<NotebookCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == NotebookAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is
    /// the heart of the capstone: a guardrail breach blocks, a degraded axis must produce a
    /// visible claim narrowing, CLI/export parity must always certify, and the narrowing
    /// must be consistent.
    pub fn derive_status(&self) -> NotebookProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != NOTEBOOK_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return NotebookProfileClaimStatus::Red;
        }

        // Any spec guardrail breach blocks outright.
        if !self.guardrails.all_held() {
            return NotebookProfileClaimStatus::Red;
        }

        // A live-trusted claim may only stand on the local, first-party trusted-kernel
        // profile.
        if self.certified_claim.asserts_live_trusted_result()
            && !self.profile.is_local_first_party()
        {
            return NotebookProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return NotebookProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(NotebookCertificationAxis::CliExport) {
            Some(o) if o.state == NotebookAxisCertificationState::Certified => {}
            _ => return NotebookProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == NotebookAxisCertificationState::UndisclosedDrift)
        {
            return NotebookProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return NotebookProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return NotebookProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return NotebookProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return NotebookProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return NotebookProfileClaimStatus::Red;
        }

        NotebookProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == NOTEBOOK_CERT_ROW_RECORD_KIND
            && self.schema_version == NOTEBOOK_CERT_SCHEMA_VERSION
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
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1091 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`NotebookProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotebookProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<NotebookProfileCertificationRow>,
}

/// Checked-in M05-1091 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<NotebookProfileCertificationRow>,
    pub summary: NotebookProfileCertificationSummary,
}

impl NotebookProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: NotebookProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NOTEBOOK_CERT_SCHEMA_VERSION,
            record_kind: NOTEBOOK_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: NotebookProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5NotebookCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5NotebookKernelOutputComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5NotebookCertifiedProfile::ALL
                .iter()
                .all(|p| profiles.contains(p))
    }

    /// Whether every frozen component family is certified on at least one profile — proof
    /// the full matrix runs across the claimed profiles.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5NotebookKernelOutputComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(NotebookCertificationAxis::CliExport)
                .is_some_and(|o| o.state == NotebookAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Whether every row holds all four spec guardrails.
    pub fn all_guardrails_held(&self) -> bool {
        self.rows.iter().all(|r| r.guardrails.all_held())
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> NotebookProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NotebookProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NotebookProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == NotebookProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(NotebookProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();
        let all_guardrails = self.all_guardrails_held();

        NotebookProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == NOTEBOOK_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: all_guardrails,
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(NotebookProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_profiles
                && all_families
                && all_guardrails,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NotebookCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NOTEBOOK_CERT_SCHEMA_VERSION {
            violations.push(NotebookCertificationViolation::SchemaVersion {
                expected: NOTEBOOK_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NOTEBOOK_CERT_RECORD_KIND {
            violations.push(NotebookCertificationViolation::RecordKind {
                expected: NOTEBOOK_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NotebookCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != NOTEBOOK_CERT_CANONICAL_BUNDLE_REF {
            violations.push(NotebookCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NotebookCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(NotebookCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(NotebookCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(NotebookCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != NOTEBOOK_CERT_CANONICAL_BUNDLE_REF {
                violations.push(NotebookCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // The four spec guardrails must be held.
            if !row.guardrails.all_held() {
                violations.push(NotebookCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // A live-trusted claim may only stand on the local first-party profile.
            if row.certified_claim.asserts_live_trusted_result()
                && !row.profile.is_local_first_party()
            {
                violations.push(
                    NotebookCertificationViolation::NonLocalProfileClaimsLiveTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(NotebookCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(NotebookCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(NotebookCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(NotebookCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == NotebookProfileClaimStatus::Red {
                violations.push(NotebookCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(NotebookCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(NotebookCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(NotebookCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(NotebookCertificationViolation::RawNotebookMaterialInExport);
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
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Notebook Document/Kernel/Output Component Profile Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5NotebookCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Guardrails held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_notebook_kernel_output_component_certification_export(
) -> Result<NotebookProfileCertificationPacket, NotebookCertificationArtifactError> {
    let packet: NotebookProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notebook-kernel-output-component-certification-proof/support_export.json"
    )))
    .map_err(NotebookCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NotebookCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum NotebookCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NotebookCertificationViolation>),
}

impl fmt::Display for NotebookCertificationArtifactError {
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

impl Error for NotebookCertificationArtifactError {}

/// Validation failure for M05-1091 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotebookCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLocalProfileClaimsLiveTruth { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawNotebookMaterialInExport,
}

impl fmt::Display for NotebookCertificationViolation {
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
                    "packet does not cite the canonical notebook-kernel-output proof bundle"
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
                    "row {id} does not cite the one canonical notebook-kernel-output proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaches a B129 spec guardrail: a recovery card implies a rerun, stale \
output is presented as live, the trust class is hidden behind hover only, or local / remote / \
managed kernels collapse into one unlabeled badge"
                )
            }
            Self::NonLocalProfileClaimsLiveTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a live-trusted claim on a non-local-first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
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
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh live-trusted \
claim, a guardrail is breached, CLI/export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 local / remote / managed notebook profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen notebook-kernel-output component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawNotebookMaterialInExport => {
                write!(f, "export contains raw notebook cell / credential material")
            }
        }
    }
}

impl Error for NotebookCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&NotebookAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != NotebookAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// notebook generics the spec forbids collapsing distinct document identity, kernel origin
/// / class / liveness, output trust class, output provenance, restart / reconnect
/// consequence, and recovery-continuity truth into (whole-label matches so a full sentence
/// naming a concrete kernel, output, or recovery state is not flagged).
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
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "loading"
            | "content"
            | "busy"
            | "queued"
            | "disconnected"
            | "remote"
            | "managed"
            | "sanitized"
            | "no kernel"
            | "reconnect"
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

/// Builds the canonical, checked-in M05-1091 certification packet. Certifies all eight
/// claimed M5 local / remote / managed notebook profiles: four deliver their claim (green)
/// and four auto-narrow a not-current truth axis to a weaker result ceiling (yellow). No
/// profile hides drift or breaches a guardrail (red).
pub fn seeded_m5_notebook_kernel_output_component_certification_packet(
) -> NotebookProfileCertificationPacket {
    NotebookProfileCertificationPacket::new(NotebookProfileCertificationPacketInput {
        packet_id: "m5-notebook-kernel-output-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: NOTEBOOK_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: NOTEBOOK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:notebook-kernel-output-component-certification:{id}"),
        NOTEBOOK_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> NotebookCertExportParity {
    NotebookCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: NotebookCertificationAxis) -> &'static str {
    match axis {
        NotebookCertificationAxis::Visual => {
            "document identity, kernel origin / class / liveness, output trust class, output provenance, restart / reconnect consequence, and preserved-vs-lost recovery state shown on-surface"
        }
        NotebookCertificationAxis::Keyboard => {
            "the same select-kernel / inspect-origin / open-raw / reconnect / restart-clean / choose-another-kernel / export actions are keyboard-reachable"
        }
        NotebookCertificationAxis::ScreenReader => {
            "the same notebook truth is announced non-visually, never color / glyph / hover-only"
        }
        NotebookCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        NotebookCertificationAxis::DegradedState => {
            "a stale, disconnected, or kernel-free reading honestly downgrades the LiveTrustedResult/ReviewableResult claim rather than reading as a fresh live-trusted result"
        }
        NotebookCertificationAxis::NotebookTruth => {
            "document identity, kernel origin, output trust class, output provenance, restart / reconnect consequence, and recovery continuity stay explicit and never let a recovery card imply a rerun, present stale output as live, hide the trust class behind hover only, or collapse local / remote / managed kernels into one unlabeled badge"
        }
    }
}

fn seed_certified(axis: NotebookCertificationAxis) -> NotebookAxisOutcome {
    NotebookAxisOutcome {
        axis,
        state: NotebookAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: NotebookCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5NotebookKernelOutputDowngradeTrigger,
) -> NotebookAxisOutcome {
    NotebookAxisOutcome {
        axis,
        state: NotebookAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<NotebookAxisOutcome> {
    NotebookCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: NotebookCertificationAxis,
    outcome: NotebookAxisOutcome,
) -> Vec<NotebookAxisOutcome> {
    NotebookCertificationAxis::ALL
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
    profile: M5NotebookCertifiedProfile,
    claimed_claim: M5NotebookComponentClaim,
    certified_claim: M5NotebookComponentClaim,
    consumed_families: &[M5NotebookKernelOutputComponentFamily],
    axis_outcomes: Vec<NotebookAxisOutcome>,
    claim_auto_narrow: Option<NotebookClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> NotebookProfileCertificationRow {
    let mut row = NotebookProfileCertificationRow {
        record_kind: NOTEBOOK_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: NOTEBOOK_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: NotebookCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: NOTEBOOK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: NotebookProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            NOTEBOOK_CERT_MATRIX_REF.to_owned(),
            NOTEBOOK_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: NotebookCertificationAxis,
    from_claim: M5NotebookComponentClaim,
    to_claim: M5NotebookComponentClaim,
    label: &str,
) -> NotebookClaimAutoNarrow {
    NotebookClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<NotebookProfileCertificationRow> {
    use M5NotebookCertifiedProfile as P;
    use M5NotebookComponentClaim::*;
    use M5NotebookKernelOutputComponentFamily::*;
    use M5NotebookKernelOutputDowngradeTrigger as Trig;
    use NotebookCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:local-trusted-kernel",
            P::LocalTrustedKernel,
            LiveTrustedResult,
            LiveTrustedResult,
            &[NotebookDocumentHeader, KernelStateStrip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "document_identity"],
            &[
                "notebook document header names the canonical .ipynb identity and its local first-party source",
                "kernel state strip names the ready, live, local first-party kernel and its execution state",
                "keyboard / screen-reader reach preserved for the document header and the kernel strip",
                "notebook-truth: the local trusted kernel is the only profile that certifies a live-trusted result",
            ],
        ),
        seed_row(
            "cert:remote-isolated-kernel",
            P::RemoteIsolatedKernel,
            ReviewableResult,
            ReviewableResult,
            &[KernelOriginPill, KernelPickerRow],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "kernel_origin"],
            &[
                "kernel origin pill names the isolated remote origin and never reads as local first-party truth",
                "kernel picker row names the remote kernel class, environment identity, and locality",
                "text / JSON / Markdown reconstruction certified for support replay",
                "notebook-truth: local / remote / managed kernels stay distinctly labeled, never collapsed into one badge",
            ],
        ),
        seed_row(
            "cert:managed-kernel",
            P::ManagedKernel,
            ReviewableResult,
            ReviewableResult,
            &[KernelStateStrip, NotebookDocumentHeader],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "kernel_state"],
            &[
                "kernel state strip names the managed-workspace kernel origin and connection state",
                "notebook document header keeps the canonical .ipynb identity explicit for the managed session",
                "export preserves the managed kernel origin, class, and connection truth",
                "notebook-truth: the managed origin stays explicit and distinct from local and remote kernels",
            ],
        ),
        seed_row(
            "cert:trusted-local-output",
            P::TrustedLocalOutput,
            ReviewableResult,
            ReviewableResult,
            &[OutputTrustBanner, OutputProvenanceChipGroup],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "output_trust"],
            &[
                "output trust banner names the sanitized / active trust class and never hides it behind hover only",
                "output provenance chip group names the producing kernel, run identity, and output lineage",
                "text / JSON / Markdown reconstruction certified so support can replay the output trust story",
                "notebook-truth: the raw / sanitized / active trust class stays explicit, never hover-only",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-output",
            P::StaleOutput,
            ReviewableResult,
            StaleOutputProjection,
            &[OutputTrustBanner, OutputProvenanceChipGroup],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the output's trust evidence aged out relative to the current kernel state",
                    "The output's trust evidence has gone stale relative to the current kernel state, so the ReviewableResult claim narrows to a stale-output projection and the trust banner shows a last-known freshness rather than presenting the stale output as live truth",
                    Trig::StaleOutputShownAsLive,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableResult,
                StaleOutputProjection,
                "Stale output: the output's trust evidence has aged out and its last-known freshness is shown; the stale output is never presented as live truth",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "output trust banner keeps the trust class visible and marks the freshness as stale, not live",
                "output provenance chip group keeps the producing run identity and lineage visible through the stale window",
                "degraded-state: ReviewableResult narrows to a stale-output projection (auto-narrowed)",
                "notebook-truth: the stale output is never presented as live and the trust class is never hover-only",
            ],
        ),
        seed_row(
            "cert:degraded-origin-kernel",
            P::DegradedOriginKernel,
            ReviewableResult,
            DegradedOriginProjection,
            &[KernelOriginPill, KernelPickerRow],
            seed_certified_except(
                Ax::NotebookTruth,
                seed_narrowed(
                    Ax::NotebookTruth,
                    "the kernel origin is only approximately known and cannot be stated exactly",
                    "The kernel's origin is degraded — unstated or only approximately known — so the ReviewableResult claim narrows to a degraded-origin projection and the origin pill shows the last-known origin rather than implying an exact, fully-provenanced kernel origin",
                    Trig::KernelOriginUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::NotebookTruth,
                ReviewableResult,
                DegradedOriginProjection,
                "Degraded kernel origin: the origin is only approximately known and its last-known value is shown; an exact kernel origin is not implied and origins are never collapsed into one badge",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "kernel origin pill names the last-known origin and marks it as approximate, never collapsing local / remote / managed into one badge",
                "kernel picker row keeps the kernel class and locality visible while the origin is degraded",
                "notebook-truth: ReviewableResult narrows to a degraded-origin projection (auto-narrowed)",
                "notebook-truth: a degraded origin never implies exact, fully-provenanced kernel origin truth",
            ],
        ),
        seed_row(
            "cert:restarted-kernel",
            P::RestartedKernel,
            ReviewableResult,
            NoKernelProjection,
            &[RestartConsequenceCard, KernelRecoveryCard],
            seed_certified_except(
                Ax::NotebookTruth,
                seed_narrowed(
                    Ax::NotebookTruth,
                    "a clean restart cleared the live kernel state and outputs without a rerun",
                    "A clean restart cleared the kernel's live state and outputs and no rerun happened, so the ReviewableResult claim narrows to a no-kernel projection; the restart consequence card names the cleared state and the recovery card offers reconnect / restart-clean / choose-another-kernel without implying a silent rerun",
                    Trig::RecoveryOverclaimed,
                ),
            ),
            Some(seed_narrow(
                Ax::NotebookTruth,
                ReviewableResult,
                NoKernelProjection,
                "Restarted kernel: a clean restart cleared the live state and outputs with no rerun; the cleared state is named and recovery actions never imply a silent rerun",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "restart consequence card names the cleared live state and preserved-vs-lost outputs after a clean restart",
                "kernel recovery card offers reconnect / restart-clean / choose-another-kernel and never implies a rerun happened",
                "notebook-truth: ReviewableResult narrows to a no-kernel projection (auto-narrowed)",
                "notebook-truth: the recovery card never implies a silent rerun on restore",
            ],
        ),
        seed_row(
            "cert:disconnected-reconnecting-kernel",
            P::DisconnectedReconnectingKernel,
            ReviewableResult,
            PartialKernelParityProjection,
            &[KernelRecoveryCard, KernelStateStrip],
            seed_certified_except(
                Ax::NotebookTruth,
                seed_narrowed(
                    Ax::NotebookTruth,
                    "the kernel disconnected and its parity is only partially re-resolved on reconnect",
                    "The kernel disconnected and is reconnecting with only partial parity re-resolved, so the ReviewableResult claim narrows to a partial-kernel-parity projection; the recovery card names the reconnect offer and the state strip names the resolved axes rather than showing the reconnect as a fresh, fully-live kernel",
                    Trig::ReconnectShownAsFresh,
                ),
            ),
            Some(seed_narrow(
                Ax::NotebookTruth,
                ReviewableResult,
                PartialKernelParityProjection,
                "Disconnected, reconnecting kernel: parity is only partially re-resolved and the resolved axes are named; the reconnect is never shown as a fresh, fully-live kernel",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "kernel recovery card names the reconnect offer and the disconnected state without implying a fresh live kernel",
                "kernel state strip names the disconnected-then-reconnecting connection state and the resolved parity axes",
                "notebook-truth: ReviewableResult narrows to a partial-kernel-parity projection (auto-narrowed)",
                "notebook-truth: a reconnect is never presented as a fresh, fully-live kernel result",
            ],
        ),
    ]
}
