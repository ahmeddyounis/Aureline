//! M05-1228 closing B146 surface certification over the frozen M5 fresh-stable-line /
//! evidence-refresh-line / correction-backport-line / bundle-currentness-line / lts-candidate-line
//! stable-line-protection matrix.
//!
//! Where the freeze matrix ([`crate::m5_stable_line_protection_matrix`]) defines the five governed active
//! stable / stable-candidate lines, the M05-1221..1227 implement lanes resolve each protection-plan,
//! correction-queue, refresh-policy, claim-downgrade, deferral-backlog, correction-conversion,
//! bundle-refresh-audit, shipping-line-drift, supported-line defect-ledger, backport-decision-timer,
//! post-launch correction-report, train-comparison, LTS-readiness-decision, and line-creation-gate
//! registry; this closing capstone *certifies* that the shared stable-line operating truth holds on every
//! claimed M5 supported line — stable-line protection, evidence refresh, backlog conversion, bundle
//! currentness, correction/backport servicing, and LTS readiness — and auto-narrows any profile that
//! cannot sustain it.
//!
//! It is keyed on the claimed **profile** a release engineer, release operator, program-governance owner, or
//! support engineer reads a support-window, refresh-state, correction-posture, bundle-currentness,
//! LTS-readiness, evidence-snapshot, or rollback-stop surface through (a live, first-party supported-line
//! operating lane; a reviewable stable-line structure; a disclosed correction-ownership profile; an
//! unverified bundle-currentness profile; and an unverified LTS-readiness profile), not on the underlying
//! line or implement lane. Each [`StableLineProtectionProfileCertificationRow`] certifies one profile across
//! nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast, localization,
//! CLI/export, degraded-state, and stable-line-component-truth behavior — and either passes (green),
//! auto-narrows its operating claim to the weakest supported ceiling (yellow), or is blocked (red) when a
//! degraded axis is hidden behind a fresh certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedOperatingLine` / `ReviewableStableLineSurface` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party supported-line
//! operating lane with current refresh, correction, bundle-currentness, and LTS-readiness evidence may certify
//! a `CertifiedOperatingLine` claim — a reviewable, disclosed-correction-ownership, unverified-bundle-currentness,
//! or unverified-LTS-readiness profile that keeps a certified claim is over-reaching and blocks. The always-on
//! CLI/export axis must always stay certified so support and automation can reconstruct the support-window truth,
//! refresh state, correction ownership, backport decision, LTS-readiness posture, preserved evidence snapshot,
//! named correction-owner roster, bundle-currentness audit, and registry reference from the same stable-line
//! truth the operator saw.
//!
//! The B146 hard invariants are enforced per row: no profile may widen support language without current refresh
//! and correction evidence, drift a shipping line on stale evidence or frozen launch bundles, rely on tribal
//! backport memory instead of a documented correction packet, claim LTS eligibility without current rollback and
//! support evidence, or leave a supported-line defect unowned or unresolved past its SLA. A profile that breaches
//! any invariant blocks (red).
//!
//! Every row cites exactly one canonical stable-line proof bundle
//! ([`STABLE_LINE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen stable-line matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer tokens,
//! endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-stable-line-surface-certification.schema.json`](../../../../schemas/release/m5-stable-line-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_stable_line_surface_certification.md`](../../../../docs/release/m5_stable_line_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_stable_line_protection_matrix as matrix;
use matrix::{M5StableLineProtectionDowngradeTrigger, M5StableLineProtectionLine};

/// Schema version stamped on the M05-1228 certification packet.
pub const STABLE_LINE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`StableLineProtectionProfileCertificationPacket`].
pub const STABLE_LINE_CERT_RECORD_KIND: &str = "m5_stable_line_surface_certification_packet";

/// Stable record-kind tag carried by each [`StableLineProtectionProfileCertificationRow`].
pub const STABLE_LINE_CERT_ROW_RECORD_KIND: &str = "m5_stable_line_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const STABLE_LINE_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-stable-line-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const STABLE_LINE_CERT_DOC_REF: &str = "docs/release/m5_stable_line_surface_certification.md";

/// Repo-relative path of the frozen stable-line matrix schema the certified profiles render.
pub const STABLE_LINE_CERT_MATRIX_REF: &str = matrix::M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF;

/// The one canonical stable-line proof bundle every certified profile cites as its first-resolved
/// stable-line truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const STABLE_LINE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_STABLE_LINE_PROTECTION_ARTIFACT_REF;

/// The stable-line dashboard the release / release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's stable-line truth ties back to the same dashboard consumers read.
pub const STABLE_LINE_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_STABLE_LINE_PROTECTION_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const STABLE_LINE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-stable-line-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const STABLE_LINE_CERT_CSV_REF: &str =
    "artifacts/release/m5-stable-line-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const STABLE_LINE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-stable-line-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const STABLE_LINE_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-stable-line-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const STABLE_LINE_CERT_PACKET_ID: &str = "m5-stable-line-surface-certification:stable:0001";

/// The five claimed M5 supported lines this capstone certifies. Keyed on the profile
/// a release engineer, release operator, program-governance owner, or support engineer reads a
/// line-membership, readiness-event, refresh-currency, correction-packet, LTS-readiness, or evidence-snapshot
/// surface through, not on the reusable line it renders. Only a live, first-party certified-archetype lane
/// profile may certify a certified operating line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionCertifiedProfile {
    /// A live, first-party certified-archetype lane — a registry-bound lane whose current line and refresh
    /// evidence, current correction packet, and explicit stable/LTS LTS-readiness decision converge on one preserved evidence
    /// snapshot and named correction-owner roster, rendering the certified widening claim exactly right now.
    LiveSupportedLineOperatingLane,
    /// A reviewable stable-line structure: a self-sufficient, inspectable stable-line projection (a line
    /// descriptor / readiness state / ring-history snapshot an operator can review), never itself an
    /// authoritative, live-operating line.
    ReviewableStableLineStructure,
    /// An extension-author lane whose correction-packet scope can only be partially disclosed; the claim narrows
    /// to a correction-packet-disclosed projection that discloses the correction packet alongside its rollback /
    /// narrowing path, owner, and expiry, never a correction packet shown as fully documented while it becomes
    /// undocumented scope widening.
    DisclosedCorrectionOwnershipProfile,
    /// A public-preview lane whose publish/rollback, mixed-version, advisory/revocation, and support-handoff
    /// refresh drills have aged out; the claim narrows to a refresh-currency-unverified projection that keeps
    /// the last-known refresh posture explicit, never a stale refresh cadence shown as current or a stable
    /// claim widened without current refresh evidence.
    UnverifiedBundleCurrentnessProfile,
    /// A design-partner-preview lane whose closed supported-line defect is missing its linked backport decision or
    /// whose LTS-readiness evidence snapshot has aged out; the claim narrows to a lts-readiness-evidence-unverified
    /// projection that keeps the last-known missing-backport-decision posture explicit, never a LTS-readiness decision
    /// shown as backed by a fresh evidence snapshot or a Sev incident closed without a backport decision behind a
    /// green release row.
    UnverifiedLtsReadinessProfile,
}

impl M5StableLineProtectionCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5StableLineProtectionCertifiedProfile; 5] = [
        M5StableLineProtectionCertifiedProfile::LiveSupportedLineOperatingLane,
        M5StableLineProtectionCertifiedProfile::ReviewableStableLineStructure,
        M5StableLineProtectionCertifiedProfile::DisclosedCorrectionOwnershipProfile,
        M5StableLineProtectionCertifiedProfile::UnverifiedBundleCurrentnessProfile,
        M5StableLineProtectionCertifiedProfile::UnverifiedLtsReadinessProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSupportedLineOperatingLane => "live_supported_line_operating_lane",
            Self::ReviewableStableLineStructure => "reviewable_stable_line_structure",
            Self::DisclosedCorrectionOwnershipProfile => "disclosed_correction_ownership_profile",
            Self::UnverifiedBundleCurrentnessProfile => "unverified_bundle_currentness_profile",
            Self::UnverifiedLtsReadinessProfile => "unverified_lts_readiness_profile",
        }
    }

    /// True only for the live, first-party certified-archetype lane profile. A certified operating line may be
    /// certified on this profile alone; every other profile is at most a reviewable stable-line structure or
    /// a narrowed projection.
    pub const fn is_live_supported_line_operating_lane(self) -> bool {
        matches!(self, Self::LiveSupportedLineOperatingLane)
    }
}

/// The claim ladder a certified stable-line profile asserts and is certified down to. Minted locally
/// for this capstone (B146 has no separate accessibility lane): the strongest claim is a fully certified
/// operating line; each weaker tier is a disclosed projection that keeps the last-known correction-packet,
/// refresh-currency, or LTS-readiness-evidence posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StableLineProtectionClaim {
    /// Certified operating line: a fully current, registry-bound, certified-archetype lane with current line and
    /// refresh evidence, a current correction packet, and an explicit stable/LTS LTS-readiness decision backed by one preserved
    /// evidence snapshot and named correction-owner roster — the strongest claim, a stable-line surface
    /// Aureline can present as cleared to widen right now.
    CertifiedOperatingLine,
    /// Reviewable stable-line surface: a self-sufficient, inspectable read-only stable-line projection
    /// (a static line descriptor / readiness state / ring-history snapshot an operator can inspect) that is
    /// not itself an authoritative, live-operating line.
    ReviewableStableLineSurface,
    /// Freeze-exception-disclosed projection: an extension-author lane's correction-packet scope can only be
    /// partially disclosed; the lane stays a correction-packet-disclosed projection that discloses the freeze
    /// exception alongside its rollback / narrowing path, owner, and expiry, never a correction packet shown as
    /// fully documented while it becomes undocumented scope widening.
    CorrectionOwnershipDisclosedProjection,
    /// Rehearsal-currency-unverified projection: a public-preview lane's publish/rollback, mixed-version,
    /// advisory/revocation, and support-handoff refresh drills have aged out; the lane stays a
    /// refresh-currency-unverified projection that keeps the last-known refresh posture explicit, never a
    /// stale refresh cadence shown as current.
    BundleCurrentnessUnverifiedProjection,
    /// Go/no-go-evidence-unverified projection: a design-partner-preview lane's closed supported-line defect is
    /// missing its linked backport decision or its LTS-readiness evidence snapshot has aged out; the lane stays a
    /// lts-readiness-evidence-unverified projection that keeps the last-known missing-backport-decision posture
    /// explicit, never a LTS-readiness decision shown as backed by a fresh evidence snapshot or a Sev incident closed
    /// without a backport decision behind a green release row.
    LtsReadinessUnverifiedProjection,
}

impl M5StableLineProtectionClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::CertifiedOperatingLine,
        Self::ReviewableStableLineSurface,
        Self::CorrectionOwnershipDisclosedProjection,
        Self::BundleCurrentnessUnverifiedProjection,
        Self::LtsReadinessUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedOperatingLine => 4,
            Self::ReviewableStableLineSurface => 3,
            Self::CorrectionOwnershipDisclosedProjection => 2,
            Self::BundleCurrentnessUnverifiedProjection => 1,
            Self::LtsReadinessUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully certified, cleared-to-widen stable-line lane.
    pub const fn asserts_certified_operating_line(self) -> bool {
        matches!(self, Self::CertifiedOperatingLine)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedOperatingLine | Self::ReviewableStableLineSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedOperatingLine => "certified_operating_line",
            Self::ReviewableStableLineSurface => "reviewable_stable_line_surface",
            Self::CorrectionOwnershipDisclosedProjection => {
                "correction_ownership_disclosed_projection"
            }
            Self::BundleCurrentnessUnverifiedProjection => {
                "bundle_currentness_unverified_projection"
            }
            Self::LtsReadinessUnverifiedProjection => "lts_readiness_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and stable-line-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableLineProtectionCertificationAxis {
    /// Visual parity: line membership, readiness event, bundle currentness, correction-packet authority,
    /// LTS-readiness decision, preserved evidence snapshot, named correction-owner roster, rollback-stop rule, and
    /// registry reference are shown on the primary surface without relying on a shell-chrome-only affordance or a
    /// mislabeled green release row alone.
    Visual,
    /// Keyboard-reach parity: the same stable-line truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled release row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// line membership, readiness state, LTS-readiness decision, evidence snapshot, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the line membership, readiness state, or LTS-readiness decision.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// line name, readiness class, correction-packet class, or LTS-readiness class when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale refresh or correction evidence, an undocumented correction packet, or a stale
    /// LTS-readiness or correction record honestly downgrades a `CertifiedOperatingLine` / `ReviewableStableLineSurface`
    /// claim rather than reading as a fresh, fully certified operating line.
    DegradedState,
    /// Stable-line-component-truth parity: support-window truth, refresh state, correction ownership, backport
    /// decision, bundle-currentness audit, LTS-readiness decision, preserved evidence snapshot, named
    /// correction-owner roster, rollback-stop rule, and defect-ledger state stay explicit and never let a line widen
    /// support language without current refresh and correction evidence, drift a shipping line on stale evidence or
    /// frozen launch bundles, rely on tribal backport memory instead of a documented correction packet, claim LTS
    /// eligibility without current rollback and support evidence, or leave a supported-line defect unowned or
    /// unresolved past its SLA.
    StableLineComponentTruth,
}

impl StableLineProtectionCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [StableLineProtectionCertificationAxis; 9] = [
        StableLineProtectionCertificationAxis::Visual,
        StableLineProtectionCertificationAxis::Keyboard,
        StableLineProtectionCertificationAxis::ScreenReader,
        StableLineProtectionCertificationAxis::HighZoomReflow,
        StableLineProtectionCertificationAxis::HighContrast,
        StableLineProtectionCertificationAxis::Localization,
        StableLineProtectionCertificationAxis::CliExport,
        StableLineProtectionCertificationAxis::DegradedState,
        StableLineProtectionCertificationAxis::StableLineComponentTruth,
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
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrast => "high_contrast",
            Self::Localization => "localization",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::StableLineComponentTruth => "stable_line_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableLineProtectionAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl StableLineProtectionAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed
/// from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableLineProtectionProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a certified operating line, or the narrowing is inconsistent.
    Red,
}

impl StableLineProtectionProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block the
    /// release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B146 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionCertGuardrails {
    /// True if the profile widens a stable claim without current refresh and correction evidence. Must be false.
    pub widens_support_language_without_current_refresh_and_correction_evidence: bool,
    /// True if the profile lets a correction packet become undocumented scope widening. Must be false.
    pub drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles: bool,
    /// True if the profile closes a supported-line defect without a backport decision. Must be false.
    pub relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet: bool,
    /// True if the profile implies green when LTS-readiness records or correction packets are stale. Must be false.
    pub claims_lts_eligibility_without_current_rollback_and_support_evidence: bool,
    /// True if the profile maintains partner or public support language that outruns current line proof. Must
    /// be false.
    pub leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla: bool,
}

impl StableLineProtectionCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        widens_support_language_without_current_refresh_and_correction_evidence: false,
        drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles: false,
        relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet: false,
        claims_lts_eligibility_without_current_rollback_and_support_evidence: false,
        leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.widens_support_language_without_current_refresh_and_correction_evidence
            && !self.drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles
            && !self.relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet
            && !self.claims_lts_eligibility_without_current_rollback_and_support_evidence
            && !self.leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The line-membership / readiness-event / refresh-currency / correction-packet / lts-readiness /
    /// evidence-snapshot / rollback-stop / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl StableLineProtectionCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: StableLineProtectionCertificationAxis,
    /// The certification state of the axis.
    pub state: StableLineProtectionAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5StableLineProtectionDowngradeTrigger>,
}

impl StableLineProtectionAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is
    ///   exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            StableLineProtectionAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            StableLineProtectionAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            StableLineProtectionAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the certified
/// claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: StableLineProtectionCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5StableLineProtectionClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5StableLineProtectionClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 configuration-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionProfileCertificationRow {
    /// Record kind; must equal [`STABLE_LINE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STABLE_LINE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5StableLineProtectionCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5StableLineProtectionClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5StableLineProtectionClaim,
    /// The frozen lines this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5StableLineProtectionLine>,
    /// One outcome per [`StableLineProtectionCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<StableLineProtectionAxisOutcome>,
    /// The B146 hard invariants; all must hold.
    pub guardrails: StableLineProtectionCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<StableLineProtectionClaimAutoNarrow>,
    /// The one canonical stable-line proof bundle this profile cites. Must equal
    /// [`STABLE_LINE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: StableLineProtectionProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: StableLineProtectionCertExportParity,
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

impl StableLineProtectionProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: StableLineProtectionCertificationAxis,
    ) -> Option<&StableLineProtectionAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<StableLineProtectionCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && StableLineProtectionCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(StableLineProtectionAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<StableLineProtectionCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == StableLineProtectionAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a certified operating line, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> StableLineProtectionProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != STABLE_LINE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        // Every B146 hard invariant must hold.
        if !self.guardrails.all_held() {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a certified operating line.
        if self.certified_claim.asserts_certified_operating_line()
            && !self.profile.is_live_supported_line_operating_lane()
        {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(StableLineProtectionCertificationAxis::CliExport) {
            Some(o) if o.state == StableLineProtectionAxisCertificationState::Certified => {}
            _ => return StableLineProtectionProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == StableLineProtectionAxisCertificationState::UndisclosedDrift)
        {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return StableLineProtectionProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return StableLineProtectionProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return StableLineProtectionProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return StableLineProtectionProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return StableLineProtectionProfileClaimStatus::Red;
        }

        StableLineProtectionProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == STABLE_LINE_CERT_ROW_RECORD_KIND
            && self.schema_version == STABLE_LINE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1228 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionProfileCertificationSummary {
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

/// Constructor input for [`StableLineProtectionProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableLineProtectionProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<StableLineProtectionProfileCertificationRow>,
}

/// Checked-in M05-1228 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLineProtectionProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<StableLineProtectionProfileCertificationRow>,
    pub summary: StableLineProtectionProfileCertificationSummary,
}

impl StableLineProtectionProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: StableLineProtectionProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: STABLE_LINE_CERT_SCHEMA_VERSION,
            record_kind: STABLE_LINE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: StableLineProtectionProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5StableLineProtectionCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Cohorts rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5StableLineProtectionLine> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5StableLineProtectionCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen line is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5StableLineProtectionLine::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(StableLineProtectionCertificationAxis::CliExport)
                .is_some_and(|o| o.state == StableLineProtectionAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> StableLineProtectionProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StableLineProtectionProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StableLineProtectionProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StableLineProtectionProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(StableLineProtectionProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        StableLineProtectionProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == STABLE_LINE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(StableLineProtectionProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<StableLineProtectionCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != STABLE_LINE_CERT_SCHEMA_VERSION {
            violations.push(StableLineProtectionCertificationViolation::SchemaVersion {
                expected: STABLE_LINE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_LINE_CERT_RECORD_KIND {
            violations.push(StableLineProtectionCertificationViolation::RecordKind {
                expected: STABLE_LINE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(StableLineProtectionCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != STABLE_LINE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(StableLineProtectionCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(StableLineProtectionCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(StableLineProtectionCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    StableLineProtectionCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    StableLineProtectionCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != STABLE_LINE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    StableLineProtectionCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B146 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    StableLineProtectionCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a live first-party profile may certify a certified operating line.
            if row.certified_claim.asserts_certified_operating_line()
                && !row.profile.is_live_supported_line_operating_lane()
            {
                violations.push(
                    StableLineProtectionCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(StableLineProtectionCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    StableLineProtectionCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    StableLineProtectionCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    StableLineProtectionCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == StableLineProtectionProfileClaimStatus::Red {
                violations.push(StableLineProtectionCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(StableLineProtectionCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen line must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(StableLineProtectionCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(StableLineProtectionCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations
                .push(StableLineProtectionCertificationViolation::RawStableLineMaterialInExport);
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
        out.push_str("# M5 Stable-Line Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5StableLineProtectionCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
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
pub fn current_m5_stable_line_surface_certification_export() -> Result<
    StableLineProtectionProfileCertificationPacket,
    StableLineProtectionCertificationArtifactError,
> {
    let packet: StableLineProtectionProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-stable-line-surface-certification/support_export.json"
        )))
        .map_err(StableLineProtectionCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StableLineProtectionCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum StableLineProtectionCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StableLineProtectionCertificationViolation>),
}

impl fmt::Display for StableLineProtectionCertificationArtifactError {
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

impl Error for StableLineProtectionCertificationArtifactError {}

/// Validation failure for M05-1228 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableLineProtectionCertificationViolation {
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
    NonLiveProfileClaimsTrustedLane { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawStableLineMaterialInExport,
}

impl fmt::Display for StableLineProtectionCertificationViolation {
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
                    "packet does not cite the canonical stable-line proof bundle"
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
                    "row {id} does not cite the one canonical stable-line proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B146 hard invariant: widening a stable claim without current line and \
refresh evidence; letting a correction packet become undocumented scope widening; closing a Sev-1/Sev-2 \
incident without a backport decision; implying green when LTS-readiness records or correction packets are stale; or \
maintaining partner or public support language that outruns current line proof"
                )
            }
            Self::NonLiveProfileClaimsTrustedLane { id } => {
                write!(
                    f,
                    "row {id} certifies a certified operating line on a non-live first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh certified claim, a hard \
invariant broke, CLI/export parity dropped, a non-live profile claimed a certified operating line, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 supported line is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(f, "not every frozen line is certified on some profile")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawStableLineMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for StableLineProtectionCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&StableLineProtectionAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != StableLineProtectionAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the stable-line
/// generics the spec forbids collapsing distinct support-window, refresh-state, correction-ownership,
/// backport-decision, bundle-currentness, LTS-readiness, evidence-snapshot, and defect-ledger truth into
/// (whole-label matches so a full sentence naming a concrete line, correction packet, or registry reference
/// is not flagged).
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
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "certified"
            | "reviewable"
            | "line"
            | "supported line"
            | "operating line"
            | "lane"
            | "support window"
            | "support-window"
            | "readiness"
            | "refresh"
            | "refresh state"
            | "refresh cadence"
            | "bundle currentness"
            | "bundle-currentness"
            | "correction"
            | "correction packet"
            | "correction ownership"
            | "scope widening"
            | "lts"
            | "lts readiness"
            | "lts-readiness"
            | "lts posture"
            | "decision"
            | "backport"
            | "backport decision"
            | "evidence"
            | "evidence snapshot"
            | "signoff"
            | "on-call"
            | "roster"
            | "rollback"
            | "rollback stop"
            | "rollback-stop"
            | "defect"
            | "defect ledger"
            | "regression"
            | "incident"
            | "ring"
            | "ring history"
            | "known limits"
            | "registry reference"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the stable-line
/// matrix and M05-1210 heuristic so the reused [`M5StableLineProtectionDowngradeTrigger`] narrowings serialize
/// cleanly — the stable-line grammar carries only typed class tokens and opaque refs, never raw
/// secret values or endpoints.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
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

/// Builds the canonical, checked-in M05-1228 certification packet. Certifies all five claimed M5
/// configuration-bearing profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_stable_line_surface_certification_packet(
) -> StableLineProtectionProfileCertificationPacket {
    StableLineProtectionProfileCertificationPacket::new(
        StableLineProtectionProfileCertificationPacketInput {
            packet_id: STABLE_LINE_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-15T00:00:00Z".to_owned(),
            matrix_ref: STABLE_LINE_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: STABLE_LINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:stable-line-surface-certification:{id}"),
        STABLE_LINE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> StableLineProtectionCertExportParity {
    StableLineProtectionCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: StableLineProtectionCertificationAxis) -> &'static str {
    match axis {
        StableLineProtectionCertificationAxis::Visual => {
            "line membership, readiness event, bundle currentness, correction-packet authority, LTS-readiness decision, preserved evidence snapshot, named correction-owner roster, rollback-stop rule, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled green release row alone"
        }
        StableLineProtectionCertificationAxis::Keyboard => {
            "the same stable-line role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        StableLineProtectionCertificationAxis::ScreenReader => {
            "the same stable-line truth is announced non-visually, never a shell-chrome-only / mislabeled-release-row / unlabeled-control-only cue"
        }
        StableLineProtectionCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the line membership, readiness state, LTS-readiness decision, evidence snapshot, or registry reference"
        }
        StableLineProtectionCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the line membership, readiness state, or LTS-readiness decision"
        }
        StableLineProtectionCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a line name, readiness class, correction-packet class, or LTS-readiness class"
        }
        StableLineProtectionCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        StableLineProtectionCertificationAxis::DegradedState => {
            "a stale refresh or correction evidence, an undocumented correction packet, or a stale LTS-readiness or correction record honestly downgrades the CertifiedOperatingLine/ReviewableStableLineSurface claim rather than reading as a fresh, fully certified operating line"
        }
        StableLineProtectionCertificationAxis::StableLineComponentTruth => {
            "line membership, readiness event, bundle currentness, correction-packet authority, LTS-readiness decision, preserved evidence snapshot, named correction-owner roster, rollback-stop rule, and backport decision stay explicit and never let a lane widen a stable claim without current refresh and correction evidence, leave a correction packet as undocumented scope widening, close a supported-line defect without a backport decision, imply green while LTS-readiness or correction records are stale, or maintain partner or public support language that outruns current line proof"
        }
    }
}

fn seed_certified(axis: StableLineProtectionCertificationAxis) -> StableLineProtectionAxisOutcome {
    StableLineProtectionAxisOutcome {
        axis,
        state: StableLineProtectionAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: StableLineProtectionCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5StableLineProtectionDowngradeTrigger,
) -> StableLineProtectionAxisOutcome {
    StableLineProtectionAxisOutcome {
        axis,
        state: StableLineProtectionAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<StableLineProtectionAxisOutcome> {
    StableLineProtectionCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: StableLineProtectionCertificationAxis,
    outcome: StableLineProtectionAxisOutcome,
) -> Vec<StableLineProtectionAxisOutcome> {
    StableLineProtectionCertificationAxis::ALL
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
    profile: M5StableLineProtectionCertifiedProfile,
    claimed_claim: M5StableLineProtectionClaim,
    certified_claim: M5StableLineProtectionClaim,
    consumed_families: &[M5StableLineProtectionLine],
    axis_outcomes: Vec<StableLineProtectionAxisOutcome>,
    claim_auto_narrow: Option<StableLineProtectionClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> StableLineProtectionProfileCertificationRow {
    let mut row = StableLineProtectionProfileCertificationRow {
        record_kind: STABLE_LINE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: STABLE_LINE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: StableLineProtectionCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: STABLE_LINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: StableLineProtectionProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            STABLE_LINE_CERT_MATRIX_REF.to_owned(),
            STABLE_LINE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: StableLineProtectionCertificationAxis,
    from_claim: M5StableLineProtectionClaim,
    to_claim: M5StableLineProtectionClaim,
    label: &str,
) -> StableLineProtectionClaimAutoNarrow {
    StableLineProtectionClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<StableLineProtectionProfileCertificationRow> {
    use M5StableLineProtectionCertifiedProfile as P;
    use M5StableLineProtectionClaim::*;
    use M5StableLineProtectionDowngradeTrigger as Trig;
    use M5StableLineProtectionLine::*;
    use StableLineProtectionCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-certified-widening-lane",
            P::LiveSupportedLineOperatingLane,
            CertifiedOperatingLine,
            CertifiedOperatingLine,
            &[FreshStableLine],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "lts_readiness_decision",
            ],
            &[
                "certified-archetype line: current refresh and correction evidence, a current correction packet, and an explicit stable/LTS LTS-readiness decision converge on one preserved evidence snapshot and named correction-owner roster, never a stale LTS-readiness or correction record dressed up as a fresh widening decision",
                "the certified operating line keeps stable operation IDs while the line membership, readiness event, LTS-readiness decision, and evidence snapshot bind to the one stable-line registry across release-center / release / diagnostics / support",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered operating line",
                "stable-line-component-truth: a live, first-party certified-archetype lane with current line, refresh, correction, and LTS-readiness evidence is the only profile that certifies a certified operating line",
            ],
        ),
        seed_row(
            "cert:reviewable-stable-line-structure",
            P::ReviewableStableLineStructure,
            ReviewableStableLineSurface,
            ReviewableStableLineSurface,
            &[EvidenceRefreshLine],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "line_membership",
            ],
            &[
                "core-team canary line: an internal dogfood ring with an armed rollback-stop rule, its line descriptor, known-limits packet, and rollback target bound to the single stable-line registry and inspectable before widening rather than a per-surface description copied by hand, and ring history preserved across the ring",
                "the reviewable stable-line structure keeps its line-membership, readiness-state, rollback-stop, and registry labels inspectable rather than a shell-chrome-only or mislabeled-release-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable stable-line structure",
                "stable-line-component-truth: a reviewable stable-line structure never certifies a live stable/LTS widening claim and never widens a stable claim without current refresh and correction evidence",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-correction-packet-profile",
            P::DisclosedCorrectionOwnershipProfile,
            ReviewableStableLineSurface,
            CorrectionOwnershipDisclosedProjection,
            &[CorrectionBackportLine],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the extension-author lane carries a correction packet whose scope can only be partially disclosed for this profile so a fully documented, non-widening exception cannot be certified",
                    "The extension-author lane carries a correction packet whose scope, rollback/narrowing path, and expiry can only be partially disclosed, so the ReviewableStableLineSurface claim narrows to a correction-packet-disclosed projection and the lane discloses the correction packet alongside its owner and risk capture rather than presenting it as fully documented or letting it become undocumented scope widening",
                    Trig::ReliedOnTribalBackportMemory,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableStableLineSurface,
                CorrectionOwnershipDisclosedProjection,
                "Freeze exception disclosed partial: the extension-author correction-packet scope is only partially documented so it is disclosed alongside its rollback/narrowing path and owner and never becomes undocumented scope widening",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "extension-author line: the correction-packet packet names its scope, rollback/narrowing path, owner, risk, and expiry and marks the exception as disclosed-partial rather than letting a correction packet become undocumented scope widening when the scope is incomplete",
                "the extension-author surface keeps its correction-packet scope, rollback/narrowing path, and expiry legible while the exception is disclosed as partial",
                "localization: ReviewableStableLineSurface narrows to a correction-packet-disclosed projection (auto-narrowed)",
                "stable-line-component-truth: a partially-documented correction packet never becomes undocumented scope widening — the rollback/narrowing path and owner are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-refresh-currency-profile",
            P::UnverifiedBundleCurrentnessProfile,
            ReviewableStableLineSurface,
            BundleCurrentnessUnverifiedProjection,
            &[BundleCurrentnessLine],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the publish/rollback, mixed-version, advisory/revocation, and support-handoff refresh drills have aged out so a fully current refresh cadence cannot be certified",
                    "The publish/rollback, mixed-version, advisory/revocation, and support-handoff refresh drills have aged out, so the ReviewableStableLineSurface claim narrows to a refresh-currency-unverified projection and the lane keeps the last-known refresh posture explicit rather than widening a stable claim without current refresh evidence or implying green while the refresh cadence is stale",
                    Trig::WidenedSupportWithoutCurrentRefreshEvidence,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableStableLineSurface,
                BundleCurrentnessUnverifiedProjection,
                "Rehearsal currency unverified: the publish/rollback, mixed-version, advisory/revocation, and support-handoff drills have aged out so the last-known refresh posture stays explicit and no stale refresh cadence reads as a fresh widening",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "public-preview line: the refresh ledger keeps its per-drill currency explicit and marks the cadence as unverified rather than widening a stable claim without current refresh evidence when the drills have aged out, and never implies green while the cadence is stale",
                "the public-preview surface keeps its per-drill refresh ledger and last-run lineage legible while the bundle currentness is disclosed as unverified",
                "degraded-state: ReviewableStableLineSurface narrows to a refresh-currency-unverified projection (auto-narrowed)",
                "stable-line-component-truth: a refresh cadence never reads as current when its drills have aged out and never lets a stale refresh ledger imply a fresh widening",
            ],
        ),
        seed_row(
            "cert:unverified-backport-decision-profile",
            P::UnverifiedLtsReadinessProfile,
            ReviewableStableLineSurface,
            LtsReadinessUnverifiedProjection,
            &[LtsCandidateLine],
            seed_certified_except(
                Ax::StableLineComponentTruth,
                seed_narrowed(
                    Ax::StableLineComponentTruth,
                    "a closed supported-line defect is missing its linked backport decision or the LTS-readiness evidence snapshot has aged out so LTS-readiness evidence and incident-regression convergence cannot be certified",
                    "A closed supported-line defect is missing its linked backport decision or the LTS-readiness evidence snapshot has aged out, so the ReviewableStableLineSurface claim narrows to a lts-readiness-evidence-unverified projection and the lane keeps the last-known missing-backport-decision posture explicit rather than presenting the LTS-readiness decision as backed by a fresh evidence snapshot or closing a Sev incident without a backport decision behind a green release row",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::StableLineComponentTruth,
                ReviewableStableLineSurface,
                LtsReadinessUnverifiedProjection,
                "Go/no-go evidence unverified: a closed supported-line defect is missing its linked backport decision so the last-known missing-backport-decision posture stays explicit and no LTS-readiness decision reads as backed by a fresh evidence snapshot",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "design-partner preview line: the LTS-readiness record keeps its backport-decision lineage and evidence snapshot explicit and marks the evidence as unverified rather than closing a supported-line defect without a backport decision behind a green release row",
                "the design-partner surface keeps its LTS-readiness record and backport-decision lineage legible while the LTS-readiness evidence is disclosed as unverified",
                "stable-line-component-truth: ReviewableStableLineSurface narrows to a lts-readiness-evidence-unverified projection (auto-narrowed)",
                "stable-line-component-truth: a LTS-readiness decision cites its backport decision and evidence snapshot and never lets a Sev incident close without a linked backport decision, and no widening claim outpaces the preserved evidence snapshot",
            ],
        ),
    ]
}
