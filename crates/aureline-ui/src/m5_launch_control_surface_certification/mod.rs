//! M05-1219 closing B145 surface certification over the frozen M5 core-team-canary /
//! design-partner-preview / extension-author / public-preview / certified-archetype launch-control matrix.
//!
//! Where the freeze matrix ([`crate::m5_launch_control_matrix`]) defines the five governed launch-bearing
//! cohorts, the M05-1213..1218 implement lanes resolve each cohort-descriptor, cohort-evidence-packet,
//! ring-progression, rollback-stop, regression-asset, incident-close, freeze-exception, go/no-go,
//! ORR-review, rehearsal-drill, widening-decision, and ring-history registry, this closing capstone
//! *certifies* that the shared launch-control truth holds on every claimed M5 launch-bearing widening
//! profile — cohort graduation, ring soak, incident-regression assets, intake/freeze-exception gating,
//! rehearsal freshness, and explicit go/no-go records — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a release engineer, shiproom operator, program-governance owner, or
//! support engineer reads a cohort-membership, readiness-event, rehearsal-currency, freeze-exception,
//! go/no-go, evidence-snapshot, or rollback-stop surface through (a live, first-party certified widening lane;
//! a reviewable launch-control structure; a disclosed freeze-exception profile; an unverified rehearsal-currency
//! profile; and an unverified regression-asset profile), not on the underlying cohort or implement lane.
//! Each [`LaunchControlProfileCertificationRow`] certifies one profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! launch-control-component-truth behavior — and either passes (green), auto-narrows its widening claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh
//! certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedWideningLane` / `ReviewableLaunchControlSurface` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party certified-archetype
//! lane with current cohort, rehearsal, ORR, and go/no-go evidence may certify a `CertifiedWideningLane` claim —
//! a reviewable, disclosed-freeze-exception, unverified-rehearsal-currency, or unverified-regression-asset profile
//! that keeps a certified claim is over-reaching and blocks. The always-on CLI/export axis must always stay
//! certified so support and automation can reconstruct the cohort membership, readiness event, rehearsal currency,
//! freeze-exception authority, go/no-go decision, preserved evidence snapshot, named on-call/signoff roster,
//! rollback-stop rule, and registry reference from the same launch-control truth the operator saw.
//!
//! The B145 hard invariants are enforced per row: no profile may widen a stable claim without current cohort and
//! rehearsal evidence, let a freeze exception become undocumented scope widening, close a Sev-1/Sev-2 incident
//! without a regression asset, imply green when go/no-go records or ORR packets are stale, or maintain partner or
//! public support language that outruns current cohort proof. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical launch-control proof bundle
//! ([`LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF`]) — the frozen launch-control matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer tokens,
//! endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-launch-control-surface-certification.schema.json`](../../../../schemas/release/m5-launch-control-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_launch_control_surface_certification.md`](../../../../docs/release/m5_launch_control_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_launch_control_matrix as matrix;
use matrix::{M5LaunchControlCohort, M5LaunchControlDowngradeTrigger};

/// Schema version stamped on the M05-1219 certification packet.
pub const LAUNCH_CONTROL_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`LaunchControlProfileCertificationPacket`].
pub const LAUNCH_CONTROL_CERT_RECORD_KIND: &str = "m5_launch_control_surface_certification_packet";

/// Stable record-kind tag carried by each [`LaunchControlProfileCertificationRow`].
pub const LAUNCH_CONTROL_CERT_ROW_RECORD_KIND: &str = "m5_launch_control_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const LAUNCH_CONTROL_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-launch-control-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const LAUNCH_CONTROL_CERT_DOC_REF: &str =
    "docs/release/m5_launch_control_surface_certification.md";

/// Repo-relative path of the frozen launch-control matrix schema the certified profiles render.
pub const LAUNCH_CONTROL_CERT_MATRIX_REF: &str = matrix::M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF;

/// The one canonical launch-control proof bundle every certified profile cites as its first-resolved
/// launch-control truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_LAUNCH_CONTROL_ARTIFACT_REF;

/// The launch-control dashboard the shiproom / release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's launch-control truth ties back to the same dashboard consumers read.
pub const LAUNCH_CONTROL_CERT_CONSUMERS_BUNDLE_REF: &str = matrix::M5_LAUNCH_CONTROL_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const LAUNCH_CONTROL_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-launch-control-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const LAUNCH_CONTROL_CERT_CSV_REF: &str =
    "artifacts/release/m5-launch-control-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const LAUNCH_CONTROL_CERT_REPORT_REF: &str =
    "artifacts/release/m5-launch-control-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const LAUNCH_CONTROL_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-launch-control-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const LAUNCH_CONTROL_CERT_PACKET_ID: &str =
    "m5-launch-control-surface-certification:stable:0001";

/// The five claimed M5 launch-bearing widening profiles this capstone certifies. Keyed on the profile
/// a release engineer, shiproom operator, program-governance owner, or support engineer reads a
/// cohort-membership, readiness-event, rehearsal-currency, freeze-exception, go/no-go, or evidence-snapshot
/// surface through, not on the reusable cohort it renders. Only a live, first-party certified-archetype lane
/// profile may certify a certified widening lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlCertifiedProfile {
    /// A live, first-party certified-archetype lane — a registry-bound lane whose current cohort and rehearsal
    /// evidence, signed ORR, and explicit stable/LTS go/no-go decision converge on one preserved evidence
    /// snapshot and named on-call/signoff roster, rendering the certified widening claim exactly right now.
    LiveCertifiedWideningLane,
    /// A reviewable launch-control structure: a self-sufficient, inspectable launch-control projection (a cohort
    /// descriptor / readiness state / ring-history snapshot an operator can review), never itself an
    /// authoritative, live-widening lane.
    ReviewableLaunchControlStructure,
    /// An extension-author lane whose freeze-exception scope can only be partially disclosed; the claim narrows
    /// to a freeze-exception-disclosed projection that discloses the freeze exception alongside its rollback /
    /// narrowing path, owner, and expiry, never a freeze exception shown as fully documented while it becomes
    /// undocumented scope widening.
    DisclosedFreezeExceptionProfile,
    /// A public-preview lane whose publish/rollback, mixed-version, advisory/revocation, and support-handoff
    /// rehearsal drills have aged out; the claim narrows to a rehearsal-currency-unverified projection that keeps
    /// the last-known rehearsal posture explicit, never a stale rehearsal cadence shown as current or a stable
    /// claim widened without current rehearsal evidence.
    UnverifiedRehearsalCurrencyProfile,
    /// A design-partner-preview lane whose closed Sev-1/Sev-2 incident is missing its linked regression asset or
    /// whose go/no-go evidence snapshot has aged out; the claim narrows to a go-no-go-evidence-unverified
    /// projection that keeps the last-known missing-regression-asset posture explicit, never a go/no-go decision
    /// shown as backed by a fresh evidence snapshot or a Sev incident closed without a regression asset behind a
    /// green shiproom row.
    UnverifiedRegressionAssetProfile,
}

impl M5LaunchControlCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5LaunchControlCertifiedProfile; 5] = [
        M5LaunchControlCertifiedProfile::LiveCertifiedWideningLane,
        M5LaunchControlCertifiedProfile::ReviewableLaunchControlStructure,
        M5LaunchControlCertifiedProfile::DisclosedFreezeExceptionProfile,
        M5LaunchControlCertifiedProfile::UnverifiedRehearsalCurrencyProfile,
        M5LaunchControlCertifiedProfile::UnverifiedRegressionAssetProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCertifiedWideningLane => "live_certified_widening_lane",
            Self::ReviewableLaunchControlStructure => "reviewable_launch_control_structure",
            Self::DisclosedFreezeExceptionProfile => "disclosed_freeze_exception_profile",
            Self::UnverifiedRehearsalCurrencyProfile => "unverified_rehearsal_currency_profile",
            Self::UnverifiedRegressionAssetProfile => "unverified_regression_asset_profile",
        }
    }

    /// True only for the live, first-party certified-archetype lane profile. A certified widening lane may be
    /// certified on this profile alone; every other profile is at most a reviewable launch-control structure or
    /// a narrowed projection.
    pub const fn is_live_certified_widening_lane(self) -> bool {
        matches!(self, Self::LiveCertifiedWideningLane)
    }
}

/// The claim ladder a certified launch-control profile asserts and is certified down to. Minted locally
/// for this capstone (B145 has no separate accessibility lane): the strongest claim is a fully certified
/// widening lane; each weaker tier is a disclosed projection that keeps the last-known freeze-exception,
/// rehearsal-currency, or go/no-go-evidence posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaunchControlClaim {
    /// Certified widening lane: a fully current, registry-bound, certified-archetype lane with current cohort and
    /// rehearsal evidence, a signed ORR, and an explicit stable/LTS go/no-go decision backed by one preserved
    /// evidence snapshot and named on-call/signoff roster — the strongest claim, a launch-control surface
    /// Aureline can present as cleared to widen right now.
    CertifiedWideningLane,
    /// Reviewable launch-control surface: a self-sufficient, inspectable read-only launch-control projection
    /// (a static cohort descriptor / readiness state / ring-history snapshot an operator can inspect) that is
    /// not itself an authoritative, live-widening lane.
    ReviewableLaunchControlSurface,
    /// Freeze-exception-disclosed projection: an extension-author lane's freeze-exception scope can only be
    /// partially disclosed; the lane stays a freeze-exception-disclosed projection that discloses the freeze
    /// exception alongside its rollback / narrowing path, owner, and expiry, never a freeze exception shown as
    /// fully documented while it becomes undocumented scope widening.
    FreezeExceptionDisclosedProjection,
    /// Rehearsal-currency-unverified projection: a public-preview lane's publish/rollback, mixed-version,
    /// advisory/revocation, and support-handoff rehearsal drills have aged out; the lane stays a
    /// rehearsal-currency-unverified projection that keeps the last-known rehearsal posture explicit, never a
    /// stale rehearsal cadence shown as current.
    RehearsalCurrencyUnverifiedProjection,
    /// Go/no-go-evidence-unverified projection: a design-partner-preview lane's closed Sev-1/Sev-2 incident is
    /// missing its linked regression asset or its go/no-go evidence snapshot has aged out; the lane stays a
    /// go-no-go-evidence-unverified projection that keeps the last-known missing-regression-asset posture
    /// explicit, never a go/no-go decision shown as backed by a fresh evidence snapshot or a Sev incident closed
    /// without a regression asset behind a green shiproom row.
    GoNoGoEvidenceUnverifiedProjection,
}

impl M5LaunchControlClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::CertifiedWideningLane,
        Self::ReviewableLaunchControlSurface,
        Self::FreezeExceptionDisclosedProjection,
        Self::RehearsalCurrencyUnverifiedProjection,
        Self::GoNoGoEvidenceUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedWideningLane => 4,
            Self::ReviewableLaunchControlSurface => 3,
            Self::FreezeExceptionDisclosedProjection => 2,
            Self::RehearsalCurrencyUnverifiedProjection => 1,
            Self::GoNoGoEvidenceUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully certified, cleared-to-widen launch-control lane.
    pub const fn asserts_certified_widening_lane(self) -> bool {
        matches!(self, Self::CertifiedWideningLane)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedWideningLane | Self::ReviewableLaunchControlSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedWideningLane => "certified_widening_lane",
            Self::ReviewableLaunchControlSurface => "reviewable_launch_control_surface",
            Self::FreezeExceptionDisclosedProjection => "freeze_exception_disclosed_projection",
            Self::RehearsalCurrencyUnverifiedProjection => {
                "rehearsal_currency_unverified_projection"
            }
            Self::GoNoGoEvidenceUnverifiedProjection => "go_no_go_evidence_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and launch-control-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchControlCertificationAxis {
    /// Visual parity: cohort membership, readiness event, rehearsal currency, freeze-exception authority,
    /// go/no-go decision, preserved evidence snapshot, named on-call/signoff roster, rollback-stop rule, and
    /// registry reference are shown on the primary surface without relying on a shell-chrome-only affordance or a
    /// mislabeled green shiproom row alone.
    Visual,
    /// Keyboard-reach parity: the same launch-control truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled shiproom row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// cohort membership, readiness state, go/no-go decision, evidence snapshot, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the cohort membership, readiness state, or go/no-go decision.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// cohort name, readiness class, freeze-exception class, or go/no-go class when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale cohort or rehearsal evidence, an undocumented freeze exception, or a stale
    /// go/no-go or ORR record honestly downgrades a `CertifiedWideningLane` / `ReviewableLaunchControlSurface`
    /// claim rather than reading as a fresh, fully certified widening lane.
    DegradedState,
    /// Launch-control-component-truth parity: cohort membership, readiness event, rehearsal currency,
    /// freeze-exception authority, go/no-go decision, preserved evidence snapshot, named on-call/signoff roster,
    /// rollback-stop rule, and regression asset stay explicit and never let a lane widen a stable claim without
    /// current cohort and rehearsal evidence, leave a freeze exception as undocumented scope widening, close a
    /// Sev-1/Sev-2 incident without a regression asset, imply green while go/no-go or ORR records are stale, or
    /// maintain partner or public support language that outruns current cohort proof.
    LaunchControlComponentTruth,
}

impl LaunchControlCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [LaunchControlCertificationAxis; 9] = [
        LaunchControlCertificationAxis::Visual,
        LaunchControlCertificationAxis::Keyboard,
        LaunchControlCertificationAxis::ScreenReader,
        LaunchControlCertificationAxis::HighZoomReflow,
        LaunchControlCertificationAxis::HighContrast,
        LaunchControlCertificationAxis::Localization,
        LaunchControlCertificationAxis::CliExport,
        LaunchControlCertificationAxis::DegradedState,
        LaunchControlCertificationAxis::LaunchControlComponentTruth,
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
            Self::LaunchControlComponentTruth => "launch_control_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchControlAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl LaunchControlAxisCertificationState {
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
pub enum LaunchControlProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a certified widening lane, or the narrowing is inconsistent.
    Red,
}

impl LaunchControlProfileClaimStatus {
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

/// The five B145 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchControlCertGuardrails {
    /// True if the profile widens a stable claim without current cohort and rehearsal evidence. Must be false.
    pub widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence: bool,
    /// True if the profile lets a freeze exception become undocumented scope widening. Must be false.
    pub lets_a_freeze_exception_become_undocumented_scope_widening: bool,
    /// True if the profile closes a Sev-1/Sev-2 incident without a regression asset. Must be false.
    pub closes_a_sev_one_or_sev_two_incident_without_a_regression_asset: bool,
    /// True if the profile implies green when go/no-go records or ORR packets are stale. Must be false.
    pub implies_green_when_go_no_go_records_or_orr_packets_are_stale: bool,
    /// True if the profile maintains partner or public support language that outruns current cohort proof. Must
    /// be false.
    pub maintains_partner_or_public_support_language_that_outruns_current_cohort_proof: bool,
}

impl LaunchControlCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence: false,
        lets_a_freeze_exception_become_undocumented_scope_widening: false,
        closes_a_sev_one_or_sev_two_incident_without_a_regression_asset: false,
        implies_green_when_go_no_go_records_or_orr_packets_are_stale: false,
        maintains_partner_or_public_support_language_that_outruns_current_cohort_proof: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence
            && !self.lets_a_freeze_exception_become_undocumented_scope_widening
            && !self.closes_a_sev_one_or_sev_two_incident_without_a_regression_asset
            && !self.implies_green_when_go_no_go_records_or_orr_packets_are_stale
            && !self.maintains_partner_or_public_support_language_that_outruns_current_cohort_proof
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchControlCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The cohort-membership / readiness-event / rehearsal-currency / freeze-exception / go-no-go /
    /// evidence-snapshot / rollback-stop / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl LaunchControlCertExportParity {
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
pub struct LaunchControlAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: LaunchControlCertificationAxis,
    /// The certification state of the axis.
    pub state: LaunchControlAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5LaunchControlDowngradeTrigger>,
}

impl LaunchControlAxisOutcome {
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
            LaunchControlAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            LaunchControlAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            LaunchControlAxisCertificationState::UndisclosedDrift => {
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
pub struct LaunchControlClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: LaunchControlCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5LaunchControlClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5LaunchControlClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 configuration-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchControlProfileCertificationRow {
    /// Record kind; must equal [`LAUNCH_CONTROL_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LAUNCH_CONTROL_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5LaunchControlCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5LaunchControlClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5LaunchControlClaim,
    /// The frozen cohorts this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5LaunchControlCohort>,
    /// One outcome per [`LaunchControlCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<LaunchControlAxisOutcome>,
    /// The B145 hard invariants; all must hold.
    pub guardrails: LaunchControlCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<LaunchControlClaimAutoNarrow>,
    /// The one canonical launch-control proof bundle this profile cites. Must equal
    /// [`LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: LaunchControlProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: LaunchControlCertExportParity,
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

impl LaunchControlProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: LaunchControlCertificationAxis) -> Option<&LaunchControlAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<LaunchControlCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && LaunchControlCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(LaunchControlAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<LaunchControlCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == LaunchControlAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a certified widening lane, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> LaunchControlProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return LaunchControlProfileClaimStatus::Red;
        }

        // Every B145 hard invariant must hold.
        if !self.guardrails.all_held() {
            return LaunchControlProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return LaunchControlProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a certified widening lane.
        if self.certified_claim.asserts_certified_widening_lane()
            && !self.profile.is_live_certified_widening_lane()
        {
            return LaunchControlProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(LaunchControlCertificationAxis::CliExport) {
            Some(o) if o.state == LaunchControlAxisCertificationState::Certified => {}
            _ => return LaunchControlProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == LaunchControlAxisCertificationState::UndisclosedDrift)
        {
            return LaunchControlProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return LaunchControlProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return LaunchControlProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return LaunchControlProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return LaunchControlProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return LaunchControlProfileClaimStatus::Red;
        }

        LaunchControlProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == LAUNCH_CONTROL_CERT_ROW_RECORD_KIND
            && self.schema_version == LAUNCH_CONTROL_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1219 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchControlProfileCertificationSummary {
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

/// Constructor input for [`LaunchControlProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchControlProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<LaunchControlProfileCertificationRow>,
}

/// Checked-in M05-1219 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchControlProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<LaunchControlProfileCertificationRow>,
    pub summary: LaunchControlProfileCertificationSummary,
}

impl LaunchControlProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: LaunchControlProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: LAUNCH_CONTROL_CERT_SCHEMA_VERSION,
            record_kind: LAUNCH_CONTROL_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: LaunchControlProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5LaunchControlCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Cohorts rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LaunchControlCohort> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5LaunchControlCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen cohort is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5LaunchControlCohort::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(LaunchControlCertificationAxis::CliExport)
                .is_some_and(|o| o.state == LaunchControlAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> LaunchControlProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LaunchControlProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LaunchControlProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LaunchControlProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(LaunchControlProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        LaunchControlProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(LaunchControlProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<LaunchControlCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != LAUNCH_CONTROL_CERT_SCHEMA_VERSION {
            violations.push(LaunchControlCertificationViolation::SchemaVersion {
                expected: LAUNCH_CONTROL_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != LAUNCH_CONTROL_CERT_RECORD_KIND {
            violations.push(LaunchControlCertificationViolation::RecordKind {
                expected: LAUNCH_CONTROL_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(LaunchControlCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF {
            violations.push(LaunchControlCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(LaunchControlCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(LaunchControlCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    LaunchControlCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(LaunchControlCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    LaunchControlCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B145 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(LaunchControlCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a certified widening lane.
            if row.certified_claim.asserts_certified_widening_lane()
                && !row.profile.is_live_certified_widening_lane()
            {
                violations.push(
                    LaunchControlCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(LaunchControlCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    LaunchControlCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    LaunchControlCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(LaunchControlCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == LaunchControlProfileClaimStatus::Red {
                violations.push(LaunchControlCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(LaunchControlCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen cohort must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(LaunchControlCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(LaunchControlCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(LaunchControlCertificationViolation::RawLaunchControlMaterialInExport);
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
        out.push_str("# M5 Launch-Control Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5LaunchControlCertifiedProfile::ALL.len(),
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
pub fn current_m5_launch_control_surface_certification_export(
) -> Result<LaunchControlProfileCertificationPacket, LaunchControlCertificationArtifactError> {
    let packet: LaunchControlProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-launch-control-surface-certification/support_export.json"
        )))
        .map_err(LaunchControlCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LaunchControlCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum LaunchControlCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LaunchControlCertificationViolation>),
}

impl fmt::Display for LaunchControlCertificationArtifactError {
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

impl Error for LaunchControlCertificationArtifactError {}

/// Validation failure for M05-1219 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchControlCertificationViolation {
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
    RawLaunchControlMaterialInExport,
}

impl fmt::Display for LaunchControlCertificationViolation {
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
                    "packet does not cite the canonical launch-control proof bundle"
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
                    "row {id} does not cite the one canonical launch-control proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B145 hard invariant: widening a stable claim without current cohort and \
rehearsal evidence; letting a freeze exception become undocumented scope widening; closing a Sev-1/Sev-2 \
incident without a regression asset; implying green when go/no-go records or ORR packets are stale; or \
maintaining partner or public support language that outruns current cohort proof"
                )
            }
            Self::NonLiveProfileClaimsTrustedLane { id } => {
                write!(
                    f,
                    "row {id} certifies a certified widening lane on a non-live first-party profile"
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
invariant broke, CLI/export parity dropped, a non-live profile claimed a certified widening lane, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 launch-bearing widening profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(f, "not every frozen cohort is certified on some profile")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawLaunchControlMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for LaunchControlCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&LaunchControlAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != LaunchControlAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the launch-control
/// generics the spec forbids collapsing distinct cohort-membership, readiness-event, rehearsal-currency,
/// freeze-exception, go/no-go, evidence-snapshot, rollback-stop, and regression-asset truth into (whole-label
/// matches so a full sentence naming a concrete cohort, freeze exception, or registry reference is not flagged).
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
            | "cohort"
            | "cohort membership"
            | "widening"
            | "widening lane"
            | "lane"
            | "readiness"
            | "readiness event"
            | "rehearsal"
            | "rehearsal currency"
            | "rehearsal cadence"
            | "freeze exception"
            | "freeze-exception"
            | "exception"
            | "scope widening"
            | "go/no-go"
            | "go no go"
            | "go-no-go"
            | "decision"
            | "evidence"
            | "evidence snapshot"
            | "signoff"
            | "on-call"
            | "roster"
            | "rollback"
            | "rollback stop"
            | "rollback-stop"
            | "regression"
            | "regression asset"
            | "incident"
            | "orr"
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the launch-control
/// matrix and M05-1210 heuristic so the reused [`M5LaunchControlDowngradeTrigger`] narrowings serialize
/// cleanly — the launch-control grammar carries only typed class tokens and opaque refs, never raw
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

/// Builds the canonical, checked-in M05-1219 certification packet. Certifies all five claimed M5
/// configuration-bearing profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_launch_control_surface_certification_packet(
) -> LaunchControlProfileCertificationPacket {
    LaunchControlProfileCertificationPacket::new(LaunchControlProfileCertificationPacketInput {
        packet_id: LAUNCH_CONTROL_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-15T00:00:00Z".to_owned(),
        matrix_ref: LAUNCH_CONTROL_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:launch-control-surface-certification:{id}"),
        LAUNCH_CONTROL_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> LaunchControlCertExportParity {
    LaunchControlCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: LaunchControlCertificationAxis) -> &'static str {
    match axis {
        LaunchControlCertificationAxis::Visual => {
            "cohort membership, readiness event, rehearsal currency, freeze-exception authority, go/no-go decision, preserved evidence snapshot, named on-call/signoff roster, rollback-stop rule, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled green shiproom row alone"
        }
        LaunchControlCertificationAxis::Keyboard => {
            "the same launch-control role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        LaunchControlCertificationAxis::ScreenReader => {
            "the same launch-control truth is announced non-visually, never a shell-chrome-only / mislabeled-shiproom-row / unlabeled-control-only cue"
        }
        LaunchControlCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the cohort membership, readiness state, go/no-go decision, evidence snapshot, or registry reference"
        }
        LaunchControlCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the cohort membership, readiness state, or go/no-go decision"
        }
        LaunchControlCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a cohort name, readiness class, freeze-exception class, or go/no-go class"
        }
        LaunchControlCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        LaunchControlCertificationAxis::DegradedState => {
            "a stale cohort or rehearsal evidence, an undocumented freeze exception, or a stale go/no-go or ORR record honestly downgrades the CertifiedWideningLane/ReviewableLaunchControlSurface claim rather than reading as a fresh, fully certified widening lane"
        }
        LaunchControlCertificationAxis::LaunchControlComponentTruth => {
            "cohort membership, readiness event, rehearsal currency, freeze-exception authority, go/no-go decision, preserved evidence snapshot, named on-call/signoff roster, rollback-stop rule, and regression asset stay explicit and never let a lane widen a stable claim without current cohort and rehearsal evidence, leave a freeze exception as undocumented scope widening, close a Sev-1/Sev-2 incident without a regression asset, imply green while go/no-go or ORR records are stale, or maintain partner or public support language that outruns current cohort proof"
        }
    }
}

fn seed_certified(axis: LaunchControlCertificationAxis) -> LaunchControlAxisOutcome {
    LaunchControlAxisOutcome {
        axis,
        state: LaunchControlAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: LaunchControlCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5LaunchControlDowngradeTrigger,
) -> LaunchControlAxisOutcome {
    LaunchControlAxisOutcome {
        axis,
        state: LaunchControlAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<LaunchControlAxisOutcome> {
    LaunchControlCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: LaunchControlCertificationAxis,
    outcome: LaunchControlAxisOutcome,
) -> Vec<LaunchControlAxisOutcome> {
    LaunchControlCertificationAxis::ALL
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
    profile: M5LaunchControlCertifiedProfile,
    claimed_claim: M5LaunchControlClaim,
    certified_claim: M5LaunchControlClaim,
    consumed_families: &[M5LaunchControlCohort],
    axis_outcomes: Vec<LaunchControlAxisOutcome>,
    claim_auto_narrow: Option<LaunchControlClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> LaunchControlProfileCertificationRow {
    let mut row = LaunchControlProfileCertificationRow {
        record_kind: LAUNCH_CONTROL_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: LAUNCH_CONTROL_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: LaunchControlCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: LAUNCH_CONTROL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: LaunchControlProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            LAUNCH_CONTROL_CERT_MATRIX_REF.to_owned(),
            LAUNCH_CONTROL_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: LaunchControlCertificationAxis,
    from_claim: M5LaunchControlClaim,
    to_claim: M5LaunchControlClaim,
    label: &str,
) -> LaunchControlClaimAutoNarrow {
    LaunchControlClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<LaunchControlProfileCertificationRow> {
    use LaunchControlCertificationAxis as Ax;
    use M5LaunchControlCertifiedProfile as P;
    use M5LaunchControlClaim::*;
    use M5LaunchControlCohort::*;
    use M5LaunchControlDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-certified-widening-lane",
            P::LiveCertifiedWideningLane,
            CertifiedWideningLane,
            CertifiedWideningLane,
            &[CertifiedArchetype],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "go_no_go_decision",
            ],
            &[
                "certified-archetype cohort: current cohort and rehearsal evidence, a signed ORR, and an explicit stable/LTS go/no-go decision converge on one preserved evidence snapshot and named on-call/signoff roster, never a stale go/no-go or ORR record dressed up as a fresh widening decision",
                "the certified widening lane keeps stable operation IDs while the cohort membership, readiness event, go/no-go decision, and evidence snapshot bind to the one launch-control registry across release-center / shiproom / diagnostics / support",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered widening lane",
                "launch-control-component-truth: a live, first-party certified-archetype lane with current cohort, rehearsal, ORR, and go/no-go evidence is the only profile that certifies a certified widening lane",
            ],
        ),
        seed_row(
            "cert:reviewable-launch-control-structure",
            P::ReviewableLaunchControlStructure,
            ReviewableLaunchControlSurface,
            ReviewableLaunchControlSurface,
            &[CoreTeamCanary],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "cohort_membership",
            ],
            &[
                "core-team canary cohort: an internal dogfood ring with an armed rollback-stop rule, its cohort descriptor, known-limits packet, and rollback target bound to the single launch-control registry and inspectable before widening rather than a per-surface description copied by hand, and ring history preserved across the ring",
                "the reviewable launch-control structure keeps its cohort-membership, readiness-state, rollback-stop, and registry labels inspectable rather than a shell-chrome-only or mislabeled-shiproom-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable launch-control structure",
                "launch-control-component-truth: a reviewable launch-control structure never certifies a live stable/LTS widening claim and never widens a stable claim without current cohort and rehearsal evidence",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-freeze-exception-profile",
            P::DisclosedFreezeExceptionProfile,
            ReviewableLaunchControlSurface,
            FreezeExceptionDisclosedProjection,
            &[ExtensionAuthor],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the extension-author lane carries a freeze exception whose scope can only be partially disclosed for this profile so a fully documented, non-widening exception cannot be certified",
                    "The extension-author lane carries a freeze exception whose scope, rollback/narrowing path, and expiry can only be partially disclosed, so the ReviewableLaunchControlSurface claim narrows to a freeze-exception-disclosed projection and the lane discloses the freeze exception alongside its owner and risk capture rather than presenting it as fully documented or letting it become undocumented scope widening",
                    Trig::LeftAFreezeExceptionUndocumented,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableLaunchControlSurface,
                FreezeExceptionDisclosedProjection,
                "Freeze exception disclosed partial: the extension-author freeze-exception scope is only partially documented so it is disclosed alongside its rollback/narrowing path and owner and never becomes undocumented scope widening",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "extension-author cohort: the freeze-exception packet names its scope, rollback/narrowing path, owner, risk, and expiry and marks the exception as disclosed-partial rather than letting a freeze exception become undocumented scope widening when the scope is incomplete",
                "the extension-author surface keeps its freeze-exception scope, rollback/narrowing path, and expiry legible while the exception is disclosed as partial",
                "localization: ReviewableLaunchControlSurface narrows to a freeze-exception-disclosed projection (auto-narrowed)",
                "launch-control-component-truth: a partially-documented freeze exception never becomes undocumented scope widening — the rollback/narrowing path and owner are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-rehearsal-currency-profile",
            P::UnverifiedRehearsalCurrencyProfile,
            ReviewableLaunchControlSurface,
            RehearsalCurrencyUnverifiedProjection,
            &[PublicPreview],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the publish/rollback, mixed-version, advisory/revocation, and support-handoff rehearsal drills have aged out so a fully current rehearsal cadence cannot be certified",
                    "The publish/rollback, mixed-version, advisory/revocation, and support-handoff rehearsal drills have aged out, so the ReviewableLaunchControlSurface claim narrows to a rehearsal-currency-unverified projection and the lane keeps the last-known rehearsal posture explicit rather than widening a stable claim without current rehearsal evidence or implying green while the rehearsal cadence is stale",
                    Trig::WidenedWithoutCurrentRehearsalEvidence,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableLaunchControlSurface,
                RehearsalCurrencyUnverifiedProjection,
                "Rehearsal currency unverified: the publish/rollback, mixed-version, advisory/revocation, and support-handoff drills have aged out so the last-known rehearsal posture stays explicit and no stale rehearsal cadence reads as a fresh widening",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "public-preview cohort: the rehearsal ledger keeps its per-drill currency explicit and marks the cadence as unverified rather than widening a stable claim without current rehearsal evidence when the drills have aged out, and never implies green while the cadence is stale",
                "the public-preview surface keeps its per-drill rehearsal ledger and last-run lineage legible while the rehearsal currency is disclosed as unverified",
                "degraded-state: ReviewableLaunchControlSurface narrows to a rehearsal-currency-unverified projection (auto-narrowed)",
                "launch-control-component-truth: a rehearsal cadence never reads as current when its drills have aged out and never lets a stale rehearsal ledger imply a fresh widening",
            ],
        ),
        seed_row(
            "cert:unverified-regression-asset-profile",
            P::UnverifiedRegressionAssetProfile,
            ReviewableLaunchControlSurface,
            GoNoGoEvidenceUnverifiedProjection,
            &[DesignPartnerPreview],
            seed_certified_except(
                Ax::LaunchControlComponentTruth,
                seed_narrowed(
                    Ax::LaunchControlComponentTruth,
                    "a closed Sev-1/Sev-2 incident is missing its linked regression asset or the go/no-go evidence snapshot has aged out so go/no-go evidence and incident-regression convergence cannot be certified",
                    "A closed Sev-1/Sev-2 incident is missing its linked regression asset or the go/no-go evidence snapshot has aged out, so the ReviewableLaunchControlSurface claim narrows to a go-no-go-evidence-unverified projection and the lane keeps the last-known missing-regression-asset posture explicit rather than presenting the go/no-go decision as backed by a fresh evidence snapshot or closing a Sev incident without a regression asset behind a green shiproom row",
                    Trig::ClosedASevIncidentWithoutARegressionAsset,
                ),
            ),
            Some(seed_narrow(
                Ax::LaunchControlComponentTruth,
                ReviewableLaunchControlSurface,
                GoNoGoEvidenceUnverifiedProjection,
                "Go/no-go evidence unverified: a closed Sev-1/Sev-2 incident is missing its linked regression asset so the last-known missing-regression-asset posture stays explicit and no go/no-go decision reads as backed by a fresh evidence snapshot",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "design-partner preview cohort: the go/no-go record keeps its regression-asset lineage and evidence snapshot explicit and marks the evidence as unverified rather than closing a Sev-1/Sev-2 incident without a regression asset behind a green shiproom row",
                "the design-partner surface keeps its go/no-go record and regression-asset lineage legible while the go/no-go evidence is disclosed as unverified",
                "launch-control-component-truth: ReviewableLaunchControlSurface narrows to a go-no-go-evidence-unverified projection (auto-narrowed)",
                "launch-control-component-truth: a go/no-go decision cites its regression asset and evidence snapshot and never lets a Sev incident close without a linked regression asset, and no widening claim outpaces the preserved evidence snapshot",
            ],
        ),
    ]
}
