//! M05-1099 surface certification over the frozen M5 workspace-trust /
//! guided-repair component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix`])
//! defines the eight reusable workspace-trust-banner, trust-fact-grid,
//! trust-elevation-sheet, restricted-capability-row, root-trust-strip,
//! repair-transaction-preview-card, rollback-class-strip, and
//! repair-result-receipt-row components, the M05-1093..1097 implement lanes narrow
//! each one, and the M05-1098 accessibility lane
//! ([`crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_trust_lineage_policy_epoch_checkpoint_state_or_reversal_evidence_weakens_across_claimed_m5_trust_and_repair_components`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity
//! and per-family auto-narrowing, this closing capstone *certifies* that the shared
//! component truth holds on every claimed M5 trust / repair operating profile — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, operator, or support engineer reads
//! workspace-trust and guided-repair truth through (local trusted workspace, remote
//! reviewed workspace, managed policy workspace, an exact-reversal repair, a restricted
//! workspace, a mixed-root workspace, a checkpoint-missing repair, and a
//! manual-follow-up repair), not on component family or implement lane. Each
//! [`TrustRepairProfileCertificationRow`] certifies one profile across six truth axes —
//! visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! trust-repair-truth behavior — and either passes (green), auto-narrows its
//! trust/repair claim to the weakest supported ceiling (yellow), or is blocked (red)
//! when a degraded axis is hidden behind a fresh full-trust claim inherited from a
//! healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A
//! profile that keeps a `FullTrustReviewedResult` / `ReviewableResult` claim while one
//! of its truth axes is not current is over-claiming and blocks; a profile that
//! discloses the reduction by narrowing its trust/repair claim (with a bound reason and
//! a frozen downgrade trigger) is honestly yellow. Only a local first-party profile may
//! certify a `FullTrustReviewedResult` claim — a remote, managed, restricted,
//! mixed-root, or repair profile that keeps a full-trust claim is over-reaching and
//! blocks. The always-on CLI/export axis must always stay certified so support and
//! automation can reconstruct the certified grant source, policy epoch, trust scope,
//! per-root trust, narrowed capability, repair-target ids, checkpoint availability,
//! reversal class, partial success, and manual follow-up from the same object identity
//! the user saw.
//!
//! The B130 guardrails are enforced per row: no profile may imply blanket approval
//! across roots or profiles, hide a repair's checkpoint absence or reversal limits, or
//! collapse exact / compensate / regenerate / manual / audit-only outcomes into a
//! generic success. A profile that breaches any guardrail blocks (red).
//!
//! Every row cites exactly one canonical workspace-trust-repair proof bundle
//! ([`WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF`]) — the frozen
//! workspace-trust-repair component matrix proof — rather than cloning per-profile
//! evidence. The packet is metadata-only: raw credentials, session tokens, and grant
//! secrets never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-workspace-trust-repair-component-certification.schema.json`](../../../../schemas/ui/m5-workspace-trust-repair-component-certification.schema.json).
//! The contract doc is
//! [`docs/trust/m5_workspace_trust_repair_component_certification_contract.md`](../../../../docs/trust/m5_workspace_trust_repair_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_trust_lineage_policy_epoch_checkpoint_state_or_reversal_evidence_weakens_across_claimed_m5_trust_and_repair_components as a11y;
use crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix as matrix;
use a11y::M5TrustRepairComponentClaim;
use matrix::{M5WorkspaceTrustRepairComponentFamily, M5WorkspaceTrustRepairDowngradeTrigger};

/// Schema version stamped on the M05-1099 certification packet.
pub const WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TrustRepairProfileCertificationPacket`].
pub const WORKSPACE_TRUST_REPAIR_CERT_RECORD_KIND: &str =
    "m5_workspace_trust_repair_component_certification_packet";

/// Stable record-kind tag carried by each [`TrustRepairProfileCertificationRow`].
pub const WORKSPACE_TRUST_REPAIR_CERT_ROW_RECORD_KIND: &str =
    "m5_workspace_trust_repair_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-trust-repair-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const WORKSPACE_TRUST_REPAIR_CERT_DOC_REF: &str =
    "docs/trust/m5_workspace_trust_repair_component_certification_contract.md";

/// Repo-relative path of the frozen workspace-trust-repair component matrix schema the
/// certified profiles render.
pub const WORKSPACE_TRUST_REPAIR_CERT_MATRIX_REF: &str =
    matrix::M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF;

/// The one canonical workspace-trust-repair proof bundle every certified profile cites
/// as its first-resolved component truth. All eight profiles point back to it rather
/// than cloning per-profile evidence.
pub const WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_WORKSPACE_TRUST_REPAIR_COMPONENT_ARTIFACT_REF;

/// The M05-1098 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const WORKSPACE_TRUST_REPAIR_CERT_A11Y_BUNDLE_REF: &str =
    a11y::WORKSPACE_TRUST_REPAIR_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const WORKSPACE_TRUST_REPAIR_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const WORKSPACE_TRUST_REPAIR_CERT_CSV_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const WORKSPACE_TRUST_REPAIR_CERT_REPORT_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-certification-proof/report.md";

/// The eight claimed M5 trust / repair operating profiles this capstone certifies.
/// Keyed on the profile a user, operator, or support engineer reads workspace-trust and
/// guided-repair truth through, not on the reusable component family it renders. Only a
/// local first-party profile may certify a full-trust reviewed result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRepairCertifiedProfile {
    /// A local, first-party trusted workspace where the trust banner and fact grid read
    /// as fully identified, current, first-party truth.
    LocalTrustedWorkspace,
    /// A remote / isolated workspace reviewed under a named grant — reviewable, never a
    /// blanket first-party full-trust reading.
    RemoteReviewedWorkspace,
    /// A managed / policy-governed workspace where trust and elevation are set by policy
    /// — reviewable under the named policy source.
    ManagedPolicyWorkspace,
    /// A guided repair with a present checkpoint and an exact reversal — reviewable
    /// before apply.
    ExactReversalRepair,
    /// A restricted workspace where a capability is narrowed; the claim narrows to a
    /// narrowed-capability projection.
    RestrictedWorkspace,
    /// A workspace whose roots carry mixed trust; the claim narrows to a mixed-root
    /// projection that names the per-root trust.
    MixedRootWorkspace,
    /// A guided repair whose checkpoint is absent; the claim narrows to a
    /// missing-checkpoint projection that discloses the reversal limits.
    CheckpointMissingRepair,
    /// A guided repair that completed with partial success and manual follow-up; the
    /// claim narrows to an unproven-reversal projection.
    ManualFollowUpRepair,
}

impl M5TrustRepairCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5TrustRepairCertifiedProfile; 8] = [
        M5TrustRepairCertifiedProfile::LocalTrustedWorkspace,
        M5TrustRepairCertifiedProfile::RemoteReviewedWorkspace,
        M5TrustRepairCertifiedProfile::ManagedPolicyWorkspace,
        M5TrustRepairCertifiedProfile::ExactReversalRepair,
        M5TrustRepairCertifiedProfile::RestrictedWorkspace,
        M5TrustRepairCertifiedProfile::MixedRootWorkspace,
        M5TrustRepairCertifiedProfile::CheckpointMissingRepair,
        M5TrustRepairCertifiedProfile::ManualFollowUpRepair,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTrustedWorkspace => "local_trusted_workspace",
            Self::RemoteReviewedWorkspace => "remote_reviewed_workspace",
            Self::ManagedPolicyWorkspace => "managed_policy_workspace",
            Self::ExactReversalRepair => "exact_reversal_repair",
            Self::RestrictedWorkspace => "restricted_workspace",
            Self::MixedRootWorkspace => "mixed_root_workspace",
            Self::CheckpointMissingRepair => "checkpoint_missing_repair",
            Self::ManualFollowUpRepair => "manual_follow_up_repair",
        }
    }

    /// True only for the local first-party profile. A full-trust reviewed result may be
    /// certified on this profile alone; every other profile is at most a reviewable
    /// result or a narrowed projection.
    pub const fn is_local_first_party(self) -> bool {
        matches!(self, Self::LocalTrustedWorkspace)
    }
}

/// The six truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and trust-repair-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRepairCertificationAxis {
    /// Visual parity: grant source, policy epoch, trust scope, per-root trust, narrowed
    /// capability, repair-target ids, checkpoint availability, reversal class, partial
    /// success, and manual follow-up are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same trust / repair truth and its actions (inspect
    /// trust, review transaction, reopen restricted, request approval) are reachable
    /// without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a chrome glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale lineage, expired epoch, mixed-root trust, narrowed
    /// capability, missing checkpoint, or unproven reversal honestly downgrades a
    /// `FullTrustReviewedResult` / `ReviewableResult` claim rather than reading as fresh,
    /// blanket first-party trust.
    DegradedState,
    /// Trust-repair-truth parity: grant source, policy epoch, trust scope, per-root
    /// trust, narrowed capability, checkpoint availability, reversal class, and repair
    /// outcome stay explicit and never collapse into generic chrome wording, imply
    /// blanket trust across roots or profiles, hide checkpoint absence or reversal
    /// limits, or collapse distinct reversal outcomes into a generic success.
    TrustRepairTruth,
}

impl TrustRepairCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [TrustRepairCertificationAxis; 6] = [
        TrustRepairCertificationAxis::Visual,
        TrustRepairCertificationAxis::Keyboard,
        TrustRepairCertificationAxis::ScreenReader,
        TrustRepairCertificationAxis::CliExport,
        TrustRepairCertificationAxis::DegradedState,
        TrustRepairCertificationAxis::TrustRepairTruth,
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
            Self::TrustRepairTruth => "trust_repair_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRepairAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a full-trust claim
    /// inherited from a healthier profile.
    UndisclosedDrift,
}

impl TrustRepairAxisCertificationState {
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
pub enum TrustRepairProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed trust/repair
    /// tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export
    /// parity drops, a non-local profile claims full trust, or the narrowing is
    /// inconsistent.
    Red,
}

impl TrustRepairProfileClaimStatus {
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

/// The three B130 guardrails carried on every certified profile. All three must hold —
/// a breach blocks the profile (red). Each field is `true` only when the profile
/// *breaks* the guardrail, so a clean profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairCertGuardrails {
    /// True if the profile implies blanket approval across roots or profiles rather than
    /// naming the trusted object, root scope, and per-root trust. Must be false.
    pub implies_blanket_trust_across_roots_or_profiles: bool,
    /// True if the profile hides a repair's checkpoint absence or reversal limits. Must
    /// be false.
    pub hides_checkpoint_absence_or_reversal_limits: bool,
    /// True if the profile collapses exact / compensate / regenerate / manual /
    /// audit-only outcomes into a generic success. Must be false.
    pub collapses_reversal_outcomes_into_generic_success: bool,
}

impl TrustRepairCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        implies_blanket_trust_across_roots_or_profiles: false,
        hides_checkpoint_absence_or_reversal_limits: false,
        collapses_reversal_outcomes_into_generic_success: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.implies_blanket_trust_across_roots_or_profiles
            && !self.hides_checkpoint_absence_or_reversal_limits
            && !self.collapses_reversal_outcomes_into_generic_success
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies
/// only when this offers text / JSON / Markdown reconstruction and prohibits a
/// screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The grant-source / policy-epoch / trust-scope / per-root-trust / narrowed-capability
    /// / repair-target / checkpoint / reversal / outcome fields the profile preserves in
    /// export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl TrustRepairCertExportParity {
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
pub struct TrustRepairAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: TrustRepairCertificationAxis,
    /// The certification state of the axis.
    pub state: TrustRepairAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5WorkspaceTrustRepairDowngradeTrigger>,
}

impl TrustRepairAxisOutcome {
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
            TrustRepairAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            TrustRepairAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            TrustRepairAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current.
/// Present iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: TrustRepairCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5TrustRepairComponentClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5TrustRepairComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 trust / repair profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairProfileCertificationRow {
    /// Record kind; must equal [`WORKSPACE_TRUST_REPAIR_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5TrustRepairCertifiedProfile,
    /// The trust / repair claim ceiling the profile asserts.
    pub claimed_claim: M5TrustRepairComponentClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5TrustRepairComponentClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5WorkspaceTrustRepairComponentFamily>,
    /// One outcome per [`TrustRepairCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<TrustRepairAxisOutcome>,
    /// The B130 guardrails; all must hold.
    pub guardrails: TrustRepairCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<TrustRepairClaimAutoNarrow>,
    /// The one canonical workspace-trust-repair proof bundle this profile cites. Must
    /// equal [`WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: TrustRepairProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: TrustRepairCertExportParity,
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

impl TrustRepairProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: TrustRepairCertificationAxis) -> Option<&TrustRepairAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<TrustRepairCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && TrustRepairCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(TrustRepairAxisOutcome::well_formed)
    }

    /// True when the profile narrows its trust / repair claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<TrustRepairCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == TrustRepairAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This
    /// is the heart of the capstone: a degraded axis must produce a visible claim
    /// narrowing, only a local first-party profile may certify full trust, every
    /// guardrail must hold, CLI/export parity must always certify, and the narrowing must
    /// be consistent.
    pub fn derive_status(&self) -> TrustRepairProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return TrustRepairProfileClaimStatus::Red;
        }

        // Every B130 guardrail must hold.
        if !self.guardrails.all_held() {
            return TrustRepairProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return TrustRepairProfileClaimStatus::Red;
        }

        // Only a local first-party profile may certify a full-trust reviewed result.
        if self.certified_claim.asserts_full_trust_reviewed_result()
            && !self.profile.is_local_first_party()
        {
            return TrustRepairProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(TrustRepairCertificationAxis::CliExport) {
            Some(o) if o.state == TrustRepairAxisCertificationState::Certified => {}
            _ => return TrustRepairProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == TrustRepairAxisCertificationState::UndisclosedDrift)
        {
            return TrustRepairProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return TrustRepairProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return TrustRepairProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return TrustRepairProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return TrustRepairProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return TrustRepairProfileClaimStatus::Red;
        }

        TrustRepairProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == WORKSPACE_TRUST_REPAIR_CERT_ROW_RECORD_KIND
            && self.schema_version == WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1099 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairProfileCertificationSummary {
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

/// Constructor input for [`TrustRepairProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRepairProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<TrustRepairProfileCertificationRow>,
}

/// Checked-in M05-1099 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<TrustRepairProfileCertificationRow>,
    pub summary: TrustRepairProfileCertificationSummary,
}

impl TrustRepairProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TrustRepairProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION,
            record_kind: WORKSPACE_TRUST_REPAIR_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: TrustRepairProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5TrustRepairCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5WorkspaceTrustRepairComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5TrustRepairCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof
    /// the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5WorkspaceTrustRepairComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(TrustRepairCertificationAxis::CliExport)
                .is_some_and(|o| o.state == TrustRepairAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TrustRepairProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TrustRepairProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TrustRepairProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TrustRepairProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(TrustRepairProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        TrustRepairProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self.rows.iter().all(|r| {
                r.canonical_bundle_ref == WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF
            }),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(TrustRepairProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TrustRepairCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION {
            violations.push(TrustRepairCertificationViolation::SchemaVersion {
                expected: WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != WORKSPACE_TRUST_REPAIR_CERT_RECORD_KIND {
            violations.push(TrustRepairCertificationViolation::RecordKind {
                expected: WORKSPACE_TRUST_REPAIR_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TrustRepairCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF {
            violations.push(TrustRepairCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TrustRepairCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(TrustRepairCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(TrustRepairCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(TrustRepairCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    TrustRepairCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B130 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(TrustRepairCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a local first-party profile may certify a full-trust reviewed result.
            if row.certified_claim.asserts_full_trust_reviewed_result()
                && !row.profile.is_local_first_party()
            {
                violations.push(
                    TrustRepairCertificationViolation::NonLocalProfileClaimsFullTrust {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(TrustRepairCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    TrustRepairCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    TrustRepairCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(TrustRepairCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == TrustRepairProfileClaimStatus::Red {
                violations.push(TrustRepairCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(TrustRepairCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(TrustRepairCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(TrustRepairCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(TrustRepairCertificationViolation::RawTrustMaterialInExport);
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
        out.push_str("# M5 Workspace-Trust / Guided-Repair Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5TrustRepairCertifiedProfile::ALL.len(),
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
pub fn current_m5_workspace_trust_repair_component_certification_export(
) -> Result<TrustRepairProfileCertificationPacket, TrustRepairCertificationArtifactError> {
    let packet: TrustRepairProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-trust-repair-component-certification-proof/support_export.json"
    )))
    .map_err(TrustRepairCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TrustRepairCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum TrustRepairCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TrustRepairCertificationViolation>),
}

impl fmt::Display for TrustRepairCertificationArtifactError {
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

impl Error for TrustRepairCertificationArtifactError {}

/// Validation failure for M05-1099 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustRepairCertificationViolation {
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
    NonLocalProfileClaimsFullTrust { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawTrustMaterialInExport,
}

impl fmt::Display for TrustRepairCertificationViolation {
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
                    "packet does not cite the canonical workspace-trust-repair proof bundle"
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
                    "row {id} does not cite the one canonical workspace-trust-repair proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B130 guardrail: blanket trust across roots/profiles, hidden \
checkpoint absence or reversal limits, or collapsed reversal outcomes"
                )
            }
            Self::NonLocalProfileClaimsFullTrust { id } => {
                write!(
                    f,
                    "row {id} certifies a full-trust reviewed result on a non-local first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh full-trust \
claim, a guardrail broke, CLI/export parity dropped, a non-local profile claimed full trust, or \
the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 trust / repair profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen workspace-trust-repair component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawTrustMaterialInExport => {
                write!(f, "export contains raw trust / credential material")
            }
        }
    }
}

impl Error for TrustRepairCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&TrustRepairAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != TrustRepairAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes
/// the trust / repair generics the spec forbids collapsing distinct grant-source,
/// trust-scope, checkpoint, and reversal truth into (whole-label matches so a full
/// sentence naming a concrete grant, root, or reversal class is not flagged).
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
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "loading"
            | "restricted"
            | "trusted"
            | "untrusted"
            | "mixed"
            | "mixed root"
            | "mixed-root"
            | "managed"
            | "remote"
            | "policy blocked"
            | "policy-blocked"
            | "checkpoint missing"
            | "reversal limited"
            | "compensate"
            | "regenerate"
            | "manual follow-up"
            | "audit only"
            | "success"
            | "repaired"
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

/// Builds the canonical, checked-in M05-1099 certification packet. Certifies all eight
/// claimed M5 trust / repair profiles: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker trust/repair ceiling (yellow). No
/// profile hides drift or breaks a guardrail (red).
pub fn seeded_m5_workspace_trust_repair_component_certification_packet(
) -> TrustRepairProfileCertificationPacket {
    TrustRepairProfileCertificationPacket::new(TrustRepairProfileCertificationPacketInput {
        packet_id: "m5-workspace-trust-repair-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: WORKSPACE_TRUST_REPAIR_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:workspace-trust-repair-component-certification:{id}"),
        WORKSPACE_TRUST_REPAIR_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> TrustRepairCertExportParity {
    TrustRepairCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: TrustRepairCertificationAxis) -> &'static str {
    match axis {
        TrustRepairCertificationAxis::Visual => {
            "grant source, policy epoch, trust scope, per-root trust, narrowed capability, repair-target ids, checkpoint availability, reversal class, partial success, and manual follow-up shown on-surface"
        }
        TrustRepairCertificationAxis::Keyboard => {
            "the same inspect-trust / review-transaction / reopen-restricted / request-approval actions are keyboard-reachable"
        }
        TrustRepairCertificationAxis::ScreenReader => {
            "the same trust / repair truth is announced non-visually, never color/glyph-only"
        }
        TrustRepairCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        TrustRepairCertificationAxis::DegradedState => {
            "a stale lineage, expired epoch, mixed-root trust, narrowed capability, missing checkpoint, or unproven reversal honestly downgrades the FullTrustReviewedResult/ReviewableResult claim rather than reading as fresh blanket first-party trust"
        }
        TrustRepairCertificationAxis::TrustRepairTruth => {
            "grant source, policy epoch, trust scope, per-root trust, narrowed capability, checkpoint availability, reversal class, and repair outcome stay explicit and never collapse into generic chrome, imply blanket trust across roots, hide checkpoint absence or reversal limits, or collapse distinct outcomes into a generic success"
        }
    }
}

fn seed_certified(axis: TrustRepairCertificationAxis) -> TrustRepairAxisOutcome {
    TrustRepairAxisOutcome {
        axis,
        state: TrustRepairAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: TrustRepairCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5WorkspaceTrustRepairDowngradeTrigger,
) -> TrustRepairAxisOutcome {
    TrustRepairAxisOutcome {
        axis,
        state: TrustRepairAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<TrustRepairAxisOutcome> {
    TrustRepairCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: TrustRepairCertificationAxis,
    outcome: TrustRepairAxisOutcome,
) -> Vec<TrustRepairAxisOutcome> {
    TrustRepairCertificationAxis::ALL
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
    profile: M5TrustRepairCertifiedProfile,
    claimed_claim: M5TrustRepairComponentClaim,
    certified_claim: M5TrustRepairComponentClaim,
    consumed_families: &[M5WorkspaceTrustRepairComponentFamily],
    axis_outcomes: Vec<TrustRepairAxisOutcome>,
    claim_auto_narrow: Option<TrustRepairClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> TrustRepairProfileCertificationRow {
    let mut row = TrustRepairProfileCertificationRow {
        record_kind: WORKSPACE_TRUST_REPAIR_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: TrustRepairCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: WORKSPACE_TRUST_REPAIR_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: TrustRepairProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            WORKSPACE_TRUST_REPAIR_CERT_MATRIX_REF.to_owned(),
            WORKSPACE_TRUST_REPAIR_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: TrustRepairCertificationAxis,
    from_claim: M5TrustRepairComponentClaim,
    to_claim: M5TrustRepairComponentClaim,
    label: &str,
) -> TrustRepairClaimAutoNarrow {
    TrustRepairClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<TrustRepairProfileCertificationRow> {
    use M5TrustRepairCertifiedProfile as P;
    use M5TrustRepairComponentClaim::*;
    use M5WorkspaceTrustRepairComponentFamily::*;
    use M5WorkspaceTrustRepairDowngradeTrigger as Trig;
    use TrustRepairCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:local-trusted-workspace",
            P::LocalTrustedWorkspace,
            FullTrustReviewedResult,
            FullTrustReviewedResult,
            &[WorkspaceTrustBanner, TrustFactGrid],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "grant_source"],
            &[
                "workspace-trust banner names the trusted object, root scope, and who granted the trust with its policy epoch",
                "trust-fact grid names grant source, trust scope, narrowed capability, and per-root trust in one place",
                "keyboard/screen-reader reach preserved for the banner and the fact grid",
                "trust-repair-truth: a local first-party workspace is the only profile that certifies a full-trust reviewed result",
            ],
        ),
        seed_row(
            "cert:remote-reviewed-workspace",
            P::RemoteReviewedWorkspace,
            ReviewableResult,
            ReviewableResult,
            &[WorkspaceTrustBanner, RootTrustStrip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "trust_scope"],
            &[
                "workspace-trust banner names the remote object and the named grant it was reviewed under, never blanket first-party trust",
                "root-trust strip names per-root trust so an isolated remote root never reads as uniform trust",
                "text / JSON / Markdown reconstruction certified for support replay",
                "trust-repair-truth: a remote workspace stays reviewable and never certifies a full first-party trust claim",
            ],
        ),
        seed_row(
            "cert:managed-policy-workspace",
            P::ManagedPolicyWorkspace,
            ReviewableResult,
            ReviewableResult,
            &[TrustFactGrid, TrustElevationSheet],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "policy_source"],
            &[
                "trust-fact grid names the managing policy source and the trust scope it sets",
                "trust-elevation sheet names what the elevation grants, its policy source and scope, and its lasting-versus-one-time effect with no ambient grant",
                "export preserves the policy-source and trust-scope truth",
                "trust-repair-truth: a managed workspace never implies an ambient grant beyond the reviewed policy scope",
            ],
        ),
        seed_row(
            "cert:exact-reversal-repair",
            P::ExactReversalRepair,
            ReviewableResult,
            ReviewableResult,
            &[RepairTransactionPreviewCard, RollbackClassStrip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "reversal_class"],
            &[
                "repair-transaction preview card names the repair-target ids, the present checkpoint, and the exact reversal class before anything is applied",
                "rollback-class strip names the exact reversal class and the checkpoint availability so reversibility is never implied without an exact path",
                "text / JSON / Markdown reconstruction certified so support can replay the transaction",
                "trust-repair-truth: an exact-reversal repair with a present checkpoint is reviewable, and the exact class never collapses into a generic success",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:restricted-workspace",
            P::RestrictedWorkspace,
            ReviewableResult,
            NarrowedCapabilityProjection,
            &[RestrictedCapabilityRow, TrustFactGrid],
            seed_certified_except(
                Ax::TrustRepairTruth,
                seed_narrowed(
                    Ax::TrustRepairTruth,
                    "a capability is narrowed by restricted mode so the reviewable claim narrows to a narrowed-capability projection",
                    "The workspace is in restricted mode and a capability is narrowed, so the ReviewableResult claim narrows to a narrowed-capability projection and the restricted-capability row names the blocked action families, the still-safe actions, and the restriction reason rather than implying blanket trust",
                    Trig::NarrowedCapabilityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::TrustRepairTruth,
                ReviewableResult,
                NarrowedCapabilityProjection,
                "Narrowed capability: restricted mode blocks a capability family; the blocked actions, the still-safe actions, and the restriction reason are named and the workspace never reads as blanket-trusted",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "restricted-capability row names exactly which capability family is narrowed and why, and the still-safe actions that remain reachable",
                "trust-fact grid keeps the trust scope and narrowed capability visible through the restriction",
                "trust-repair-truth: ReviewableResult narrows to a narrowed-capability projection (auto-narrowed)",
                "trust-repair-truth: restricted mode never collapses into a generic unavailable and never implies blanket trust",
            ],
        ),
        seed_row(
            "cert:mixed-root-workspace",
            P::MixedRootWorkspace,
            ReviewableResult,
            MixedRootProjection,
            &[RootTrustStrip, WorkspaceTrustBanner],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "some roots are trusted and some are not, so uniform trust cannot be certified",
                    "Some workspace roots are trusted and some are not, so the ReviewableResult claim narrows to a mixed-root projection and the root-trust strip names the per-root trust rather than presenting the workspace as uniformly trusted",
                    Trig::MixedRootShownAsUniformTrust,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableResult,
                MixedRootProjection,
                "Mixed-root trust: per-root trust is not uniform; each root's trust is named individually and the workspace is never presented as blanket-trusted across roots",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "root-trust strip names the trust of each root so mixed-root trust never reads as uniform",
                "workspace-trust banner names the mixed-root disposition instead of a single blanket-trust badge",
                "degraded-state: ReviewableResult narrows to a mixed-root projection (auto-narrowed)",
                "trust-repair-truth: mixed-root trust never collapses into blanket approval across roots",
            ],
        ),
        seed_row(
            "cert:checkpoint-missing-repair",
            P::CheckpointMissingRepair,
            ReviewableResult,
            MissingCheckpointProjection,
            &[RepairTransactionPreviewCard, RollbackClassStrip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "no checkpoint is available for this repair, so full reversibility cannot be certified",
                    "No checkpoint is available for this repair, so the ReviewableResult claim narrows to a missing-checkpoint projection and the preview card discloses the checkpoint absence and the reversal limits before apply rather than implying the repair is fully reversible",
                    Trig::CheckpointAbsenceHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableResult,
                MissingCheckpointProjection,
                "Missing checkpoint: no checkpoint is available so exact reversal cannot be promised; the checkpoint absence and the reversal limits are disclosed before the repair is applied",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "repair-transaction preview card discloses the checkpoint absence before apply and names the repair-target ids",
                "rollback-class strip names the reversal limits so reversibility is never implied without a checkpoint",
                "degraded-state: ReviewableResult narrows to a missing-checkpoint projection (auto-narrowed)",
                "trust-repair-truth: the missing checkpoint and reversal limits are never hidden",
            ],
        ),
        seed_row(
            "cert:manual-follow-up-repair",
            P::ManualFollowUpRepair,
            ReviewableResult,
            UnprovenReversalProjection,
            &[RepairResultReceiptRow, RollbackClassStrip],
            seed_certified_except(
                Ax::TrustRepairTruth,
                seed_narrowed(
                    Ax::TrustRepairTruth,
                    "the repair completed with partial success and requires manual follow-up, so a generic success cannot be certified",
                    "The repair completed with partial success and requires manual follow-up, so the ReviewableResult claim narrows to an unproven-reversal projection and the receipt row names the applied-versus-skipped scope, the compensate/manual reversal class, and the manual follow-up rather than collapsing the outcome into a generic success",
                    Trig::PartialSuccessShownAsComplete,
                ),
            ),
            Some(seed_narrow(
                Ax::TrustRepairTruth,
                ReviewableResult,
                UnprovenReversalProjection,
                "Partial success with manual follow-up: the applied-versus-skipped scope and the compensate/manual reversal class are named and the outcome is never collapsed into a generic success",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "repair-result receipt row names the applied-versus-skipped scope, the reversal class, and the required manual follow-up",
                "rollback-class strip names the compensate / manual reversal class rather than a generic success",
                "trust-repair-truth: ReviewableResult narrows to an unproven-reversal projection (auto-narrowed)",
                "trust-repair-truth: exact / compensate / regenerate / manual / audit-only outcomes never collapse into one generic success",
            ],
        ),
    ]
}
