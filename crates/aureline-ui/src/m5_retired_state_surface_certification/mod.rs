//! M05-1246 closing B148 surface certification over the frozen M5 retired-state matrix — the
//! supported lines, stable-facing capabilities, bundles, commands / deep links, schema-bearing
//! surfaces, registry-visible packages, and managed / new-tenant-gated features that reach terminal
//! `Retired` state.
//!
//! Where the freeze matrix ([`crate::m5_retired_state_matrix`]) defines the seven governed
//! retirement object classes, the M05-1239..1245 implement lanes resolve each retirement-manifest,
//! manifest-change-diff, impact-report, blocker-gate, countdown, safety-gate, review-packet,
//! closure-gate, tombstone, claim-block-gate, last-supported-snapshot, archive-export-gate,
//! closure-ledger, and propagation-blocker-gate registry; this closing capstone *certifies* that the
//! shared retired-state truth holds on every claimed M5 supported line and stable-facing surface —
//! complete retirement manifests, exact-build last-supported snapshots, tombstones and archival
//! routes, closed support notes, recorded migration outcomes, and multi-profile propagation — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a release engineer, release operator, program-governance owner, or
//! support engineer reads a retirement-manifest, last-supported-snapshot, tombstone, closure-ledger,
//! successor-route, or propagation surface through (a live, fully closed retired-state closure lane; a
//! reviewable retirement-record structure; a disclosed archive-partial profile; an unverified
//! propagation profile; and an unverified closure-ledger profile), not on the underlying object class
//! or implement lane. Each [`RetiredStateProfileCertificationRow`] certifies one
//! profile across nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and retired-state-truth behavior — and either passes
//! (green), auto-narrows its closure claim to the weakest supported ceiling (yellow), or is blocked (red)
//! when a degraded axis is hidden behind a fresh certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedRetiredClosure` / `ReviewableRetirementRecord` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, fully closed retired-state
//! closure lane — one whose retirement manifest, last-supported snapshot, tombstone, closure ledger, and
//! deployment-profile propagation all converge on one current, export-safe, internally consistent retired-state
//! record — may certify a `CertifiedRetiredClosure` claim; a reviewable, archive-partial, unverified-propagation,
//! or unverified-closure-ledger profile that keeps a certified claim is over-reaching and blocks. The always-on
//! CLI/export axis must always stay certified so support and automation can reconstruct the last-supported
//! version / channel, cutoff date, successor path, disable path, export / rollback route, archival note,
//! migration outcome, support-note closure state, and registry reference from the same retired-state proof the
//! operator saw.
//!
//! The B148 hard invariants are enforced per row: no profile may let a retired surface disappear without a
//! tombstone, archival route, or successor pointer; keep a retired class selectable in a new-install /
//! new-tenant / marketplace / upgrade flow; destroy last-supported docs / schemas / evidence before support-note
//! closure and export-safe archive handoff; leave retirement state unjoined to exact build, line identity,
//! deployment profile, and migration outcome; or retire a surface through silent disappearance, stale selection
//! UI, or orphaned support / docs truth. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical retired-state matrix proof bundle
//! ([`RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen retired-state matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets,
//! bearer tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-retired-state-surface-certification.schema.json`](../../../../schemas/release/m5-retired-state-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_retired_state_surface_certification.md`](../../../../docs/release/m5_retired_state_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_retired_state_matrix as matrix;
use matrix::{M5RetiredStateDowngradeTrigger, M5RetiredStateObject};

/// Schema version stamped on the M05-1246 certification packet.
pub const RETIRED_STATE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RetiredStateProfileCertificationPacket`].
pub const RETIRED_STATE_CERT_RECORD_KIND: &str = "m5_retired_state_surface_certification_packet";

/// Stable record-kind tag carried by each [`RetiredStateProfileCertificationRow`].
pub const RETIRED_STATE_CERT_ROW_RECORD_KIND: &str = "m5_retired_state_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const RETIRED_STATE_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-retired-state-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const RETIRED_STATE_CERT_DOC_REF: &str =
    "docs/release/m5_retired_state_surface_certification.md";

/// Repo-relative path of the frozen retired-state matrix schema the certified profiles render.
pub const RETIRED_STATE_CERT_MATRIX_REF: &str = matrix::M5_RETIRED_STATE_MATRIX_SCHEMA_REF;

/// The one canonical retired-state matrix proof bundle every certified profile cites as its
/// first-resolved retired-state truth. All five profiles point back to it rather than cloning per-profile
/// evidence.
pub const RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_RETIRED_STATE_ARTIFACT_REF;

/// The retired-surface-health dashboard the release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's retired-state truth ties back to the same dashboard consumers read.
pub const RETIRED_STATE_CERT_CONSUMERS_BUNDLE_REF: &str = matrix::M5_RETIRED_STATE_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const RETIRED_STATE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-retired-state-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const RETIRED_STATE_CERT_CSV_REF: &str =
    "artifacts/release/m5-retired-state-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const RETIRED_STATE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-retired-state-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const RETIRED_STATE_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-retired-state-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const RETIRED_STATE_CERT_PACKET_ID: &str = "m5-retired-state-surface-certification:stable:0001";

/// The five claimed M5 retired-state profiles this capstone certifies. Keyed on the profile
/// a release engineer, release operator, program-governance owner, or support engineer reads a
/// retirement-manifest, last-supported-snapshot, tombstone, closure-ledger, successor-route, or
/// propagation surface through, not on the reusable object class it renders. Only a live, fully
/// closed retired-state closure lane profile may certify a certified retired closure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateCertifiedProfile {
    /// A live, fully closed retired-state closure lane — a retiring class whose retirement manifest,
    /// exact-build last-supported snapshot, tombstone / archival route, closed support note, recorded
    /// migration outcome, and deployment-profile propagation converge on one current, joined, export-safe
    /// retired-state record, certifying the closed retirement claim exactly right now.
    LiveRetiredStateClosureLane,
    /// A reviewable retirement-record structure: a self-sufficient, inspectable retired-state projection (a
    /// retirement manifest / last-supported snapshot / tombstone record an operator can review), never
    /// itself a live, fully closed closure lane.
    ReviewableRetirementRecordStructure,
    /// An archive-partial lane whose last-supported snapshot / archive coverage can only be partially disclosed;
    /// the claim narrows to an archive-disclosed projection that discloses the archived last-supported bundle
    /// alongside its successor path and exact-build join, never an archive shown as fully retained while its
    /// coverage or build join is incomplete.
    DisclosedArchivePartialProfile,
    /// A propagation lane whose deployment-profile propagation (mirror / offline / self-hosted / managed) has
    /// aged out or a profile still lags; the claim narrows to a propagation-unverified projection that keeps the
    /// last-known propagation posture explicit, never a lagging profile shown as fully converged or a retired
    /// line still advertised after another profile closed it.
    UnverifiedPropagationProfile,
    /// A closure-ledger lane whose support-note closure / migration-outcome retention has aged out or become
    /// unreconstructable; the claim narrows to a closure-ledger-unverified projection that keeps the last-known
    /// unretained-closure posture explicit, never a support-note closure shown as complete behind a green line
    /// when its ledger is incomplete.
    UnverifiedClosureLedgerProfile,
}

impl M5RetiredStateCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5RetiredStateCertifiedProfile; 5] = [
        M5RetiredStateCertifiedProfile::LiveRetiredStateClosureLane,
        M5RetiredStateCertifiedProfile::ReviewableRetirementRecordStructure,
        M5RetiredStateCertifiedProfile::DisclosedArchivePartialProfile,
        M5RetiredStateCertifiedProfile::UnverifiedPropagationProfile,
        M5RetiredStateCertifiedProfile::UnverifiedClosureLedgerProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveRetiredStateClosureLane => "live_retired_state_closure_lane",
            Self::ReviewableRetirementRecordStructure => "reviewable_retirement_record_structure",
            Self::DisclosedArchivePartialProfile => "disclosed_archive_partial_profile",
            Self::UnverifiedPropagationProfile => "unverified_propagation_profile",
            Self::UnverifiedClosureLedgerProfile => "unverified_closure_ledger_profile",
        }
    }

    /// True only for the live, fully closed retired-state closure lane profile. A certified retired closure may
    /// be certified on this profile alone; every other profile is at most a reviewable retirement-record
    /// structure or a narrowed projection.
    pub const fn is_live_retired_state_closure_lane(self) -> bool {
        matches!(self, Self::LiveRetiredStateClosureLane)
    }
}

/// The claim ladder a certified retired-state profile asserts and is certified down to. Minted locally
/// for this capstone (B148 folds accessibility into the cert): the strongest claim is a fully certified
/// retired closure; each weaker tier is a disclosed projection that keeps the last-known archive,
/// propagation, or closure-ledger posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateClaim {
    /// Certified retired closure: a fully closed retiring class with a complete retirement manifest, an
    /// exact-build last-supported snapshot, a tombstone / archival route, a closed support note, a recorded
    /// migration outcome, and converged deployment-profile propagation all joined to exact build / line
    /// identity — the strongest claim, a retired surface Aureline can present as cleanly-closed right now.
    CertifiedRetiredClosure,
    /// Reviewable retirement record: a self-sufficient, inspectable read-only retired-state projection
    /// (a static retirement manifest / last-supported snapshot / tombstone record an operator can inspect)
    /// that is not itself a live, fully closed closure lane.
    ReviewableRetirementRecord,
    /// Archive-disclosed projection: an archive-partial lane's last-supported snapshot / archive coverage can
    /// only be partially disclosed; the lane stays an archive-disclosed projection that discloses the archived
    /// last-supported bundle alongside its successor path and exact-build join, never an archive shown as fully
    /// retained while its coverage or build join is incomplete.
    ArchiveDisclosedProjection,
    /// Propagation-unverified projection: a propagation lane's deployment-profile propagation (mirror / offline /
    /// self-hosted / managed) has aged out or a profile still lags; the lane stays a propagation-unverified
    /// projection that keeps the last-known propagation posture explicit, never a lagging profile shown as fully
    /// converged.
    PropagationUnverifiedProjection,
    /// Closure-ledger-unverified projection: a closure-ledger lane's support-note closure / migration-outcome
    /// retention has aged out or become unreconstructable; the lane stays a closure-ledger-unverified projection
    /// that keeps the last-known unretained-closure posture explicit, never a support-note closure shown as
    /// complete behind a green line.
    ClosureLedgerUnverifiedProjection,
}

impl M5RetiredStateClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::CertifiedRetiredClosure,
        Self::ReviewableRetirementRecord,
        Self::ArchiveDisclosedProjection,
        Self::PropagationUnverifiedProjection,
        Self::ClosureLedgerUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedRetiredClosure => 4,
            Self::ReviewableRetirementRecord => 3,
            Self::ArchiveDisclosedProjection => 2,
            Self::PropagationUnverifiedProjection => 1,
            Self::ClosureLedgerUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully closed, cleanly-retired surface.
    pub const fn asserts_certified_retired_closure(self) -> bool {
        matches!(self, Self::CertifiedRetiredClosure)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedRetiredClosure | Self::ReviewableRetirementRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedRetiredClosure => "certified_retired_closure",
            Self::ReviewableRetirementRecord => "reviewable_retirement_record",
            Self::ArchiveDisclosedProjection => "archive_disclosed_projection",
            Self::PropagationUnverifiedProjection => "propagation_unverified_projection",
            Self::ClosureLedgerUnverifiedProjection => "closure_ledger_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and retired-state-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetiredStateCertificationAxis {
    /// Visual parity: the retired-state identity, last-supported version / channel, cutoff date, successor path,
    /// disable path, archival note, migration outcome, support-note closure state, and registry reference are
    /// shown on the primary surface without relying on a shell-chrome-only affordance or a mislabeled green
    /// release row alone, and no retired object still reads as selectable.
    Visual,
    /// Keyboard-reach parity: the same retired-state truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled release row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// retired-state identity, last-supported version, cutoff date, successor path, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the retired-state identity, last-supported version, or successor path.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// retired object name, object class, export class, or last-supported version when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale retirement manifest, a missing last-supported snapshot, a lagging
    /// deployment-profile propagation, or an incomplete support-note closure honestly downgrades a
    /// `CertifiedRetiredClosure` / `ReviewableRetirementRecord` claim rather than reading as a fresh, fully
    /// closed retirement.
    DegradedState,
    /// Retired-state-truth parity: the last-supported version / channel, cutoff date, successor path, disable
    /// path, export / rollback route, archival note, migration outcome, support-note closure state, and
    /// deployment-profile propagation stay explicit and never let a retirement let a surface disappear without a
    /// tombstone, archival route, or successor pointer; keep a retired class selectable in a new-install /
    /// new-tenant / marketplace / upgrade flow; destroy last-supported docs / schemas / evidence before
    /// support-note closure; leave retirement state unjoined to exact build, line identity, deployment profile,
    /// and migration outcome; or retire a surface through silent disappearance, stale selection UI, or orphaned
    /// support / docs truth.
    RetiredStateTruth,
}

impl RetiredStateCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [RetiredStateCertificationAxis; 9] = [
        RetiredStateCertificationAxis::Visual,
        RetiredStateCertificationAxis::Keyboard,
        RetiredStateCertificationAxis::ScreenReader,
        RetiredStateCertificationAxis::HighZoomReflow,
        RetiredStateCertificationAxis::HighContrast,
        RetiredStateCertificationAxis::Localization,
        RetiredStateCertificationAxis::CliExport,
        RetiredStateCertificationAxis::DegradedState,
        RetiredStateCertificationAxis::RetiredStateTruth,
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
            Self::RetiredStateTruth => "retired_state_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetiredStateAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl RetiredStateAxisCertificationState {
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
pub enum RetiredStateProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a certified retired closure, or the narrowing is inconsistent.
    Red,
}

impl RetiredStateProfileClaimStatus {
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

/// The five B148 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredStateCertGuardrails {
    /// True if the profile lets a retired surface disappear without a tombstone, archival route, or successor
    /// pointer. Must be false.
    pub lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer:
        bool,
    /// True if the profile keeps a retired class selectable in a new-install, new-tenant, marketplace, or
    /// upgrade flow. Must be false.
    pub keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow:
        bool,
    /// True if the profile destroys last-supported docs, schemas, or evidence before support-note closure and
    /// export-safe archive handoff. Must be false.
    pub destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure: bool,
    /// True if the profile leaves retirement state unjoined to exact build, line identity, deployment profile,
    /// and migration outcome. Must be false.
    pub leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome:
        bool,
    /// True if the profile retires a surface through silent disappearance, stale selection UI, or orphaned
    /// support / docs truth. Must be false.
    pub retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth:
        bool,
}

impl RetiredStateCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer: false,
        keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow: false,
        destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure: false,
        leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome:
            false,
        retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth:
            false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer
            && !self.keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow
            && !self.destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure
            && !self.leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome
            && !self.retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredStateCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The retired-state-identity / last-supported-version / cutoff-date / successor-path /
    /// disable-path / archival-note / migration-outcome / support-note-closure-state / registry-reference
    /// fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl RetiredStateCertExportParity {
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
pub struct RetiredStateAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: RetiredStateCertificationAxis,
    /// The certification state of the axis.
    pub state: RetiredStateAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5RetiredStateDowngradeTrigger>,
}

impl RetiredStateAxisOutcome {
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
            RetiredStateAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            RetiredStateAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            RetiredStateAxisCertificationState::UndisclosedDrift => {
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
pub struct RetiredStateClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: RetiredStateCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5RetiredStateClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5RetiredStateClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 retired-state object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredStateProfileCertificationRow {
    /// Record kind; must equal [`RETIRED_STATE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RETIRED_STATE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5RetiredStateCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5RetiredStateClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5RetiredStateClaim,
    /// The frozen retirement object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5RetiredStateObject>,
    /// One outcome per [`RetiredStateCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<RetiredStateAxisOutcome>,
    /// The B148 hard invariants; all must hold.
    pub guardrails: RetiredStateCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<RetiredStateClaimAutoNarrow>,
    /// The one canonical retired-state matrix proof bundle this profile cites. Must equal
    /// [`RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: RetiredStateProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: RetiredStateCertExportParity,
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

impl RetiredStateProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: RetiredStateCertificationAxis) -> Option<&RetiredStateAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<RetiredStateCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && RetiredStateCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(RetiredStateAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<RetiredStateCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == RetiredStateAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a certified retired closure, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> RetiredStateProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return RetiredStateProfileClaimStatus::Red;
        }

        // Every B148 hard invariant must hold.
        if !self.guardrails.all_held() {
            return RetiredStateProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return RetiredStateProfileClaimStatus::Red;
        }

        // Only a live closure-lane profile may certify a certified retired closure.
        if self.certified_claim.asserts_certified_retired_closure()
            && !self.profile.is_live_retired_state_closure_lane()
        {
            return RetiredStateProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(RetiredStateCertificationAxis::CliExport) {
            Some(o) if o.state == RetiredStateAxisCertificationState::Certified => {}
            _ => return RetiredStateProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == RetiredStateAxisCertificationState::UndisclosedDrift)
        {
            return RetiredStateProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return RetiredStateProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return RetiredStateProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return RetiredStateProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return RetiredStateProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return RetiredStateProfileClaimStatus::Red;
        }

        RetiredStateProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == RETIRED_STATE_CERT_ROW_RECORD_KIND
            && self.schema_version == RETIRED_STATE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1246 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredStateProfileCertificationSummary {
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

/// Constructor input for [`RetiredStateProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetiredStateProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<RetiredStateProfileCertificationRow>,
}

/// Checked-in M05-1246 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredStateProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<RetiredStateProfileCertificationRow>,
    pub summary: RetiredStateProfileCertificationSummary,
}

impl RetiredStateProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: RetiredStateProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: RETIRED_STATE_CERT_SCHEMA_VERSION,
            record_kind: RETIRED_STATE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: RetiredStateProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5RetiredStateCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Retirement object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5RetiredStateObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5RetiredStateCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen line is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5RetiredStateObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(RetiredStateCertificationAxis::CliExport)
                .is_some_and(|o| o.state == RetiredStateAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> RetiredStateProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RetiredStateProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RetiredStateProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RetiredStateProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(RetiredStateProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        RetiredStateProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(RetiredStateProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<RetiredStateCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != RETIRED_STATE_CERT_SCHEMA_VERSION {
            violations.push(RetiredStateCertificationViolation::SchemaVersion {
                expected: RETIRED_STATE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != RETIRED_STATE_CERT_RECORD_KIND {
            violations.push(RetiredStateCertificationViolation::RecordKind {
                expected: RETIRED_STATE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(RetiredStateCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(RetiredStateCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(RetiredStateCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(RetiredStateCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(RetiredStateCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(RetiredStateCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    RetiredStateCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B148 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(RetiredStateCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live closure-lane profile may certify a certified retired closure.
            if row.certified_claim.asserts_certified_retired_closure()
                && !row.profile.is_live_retired_state_closure_lane()
            {
                violations.push(
                    RetiredStateCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(RetiredStateCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    RetiredStateCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    RetiredStateCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(RetiredStateCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == RetiredStateProfileClaimStatus::Red {
                violations.push(RetiredStateCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(RetiredStateCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen line must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(RetiredStateCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(RetiredStateCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(RetiredStateCertificationViolation::RawRetiredStateMaterialInExport);
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
        out.push_str("# M5 Retired-State Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5RetiredStateCertifiedProfile::ALL.len(),
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
pub fn current_m5_retired_state_surface_certification_export(
) -> Result<RetiredStateProfileCertificationPacket, RetiredStateCertificationArtifactError> {
    let packet: RetiredStateProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-retired-state-surface-certification/support_export.json"
        )))
        .map_err(RetiredStateCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RetiredStateCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum RetiredStateCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RetiredStateCertificationViolation>),
}

impl fmt::Display for RetiredStateCertificationArtifactError {
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

impl Error for RetiredStateCertificationArtifactError {}

/// Validation failure for M05-1246 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetiredStateCertificationViolation {
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
    RawRetiredStateMaterialInExport,
}

impl fmt::Display for RetiredStateCertificationViolation {
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
                    "packet does not cite the canonical retired-state matrix proof bundle"
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
                    "row {id} does not cite the one canonical retired-state matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B148 hard invariant: letting a retired surface disappear without a \
tombstone, archival route, or successor pointer; keeping a retired class selectable in a new-install / \
new-tenant / marketplace / upgrade flow; destroying last-supported docs / schemas / evidence before \
support-note closure; leaving retirement state unjoined to exact build, line identity, deployment profile, \
and migration outcome; or retiring a surface through silent disappearance, stale selection UI, or orphaned \
support / docs truth"
                )
            }
            Self::NonLiveProfileClaimsTrustedLane { id } => {
                write!(
                    f,
                    "row {id} certifies a certified retired closure on a non-live closure-lane profile"
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
invariant broke, CLI/export parity dropped, a non-live profile claimed a certified retired closure, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 retired-state profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen retirement object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawRetiredStateMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for RetiredStateCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&RetiredStateAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != RetiredStateAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the retired-state
/// generics the spec forbids collapsing distinct retirement-manifest, last-supported-snapshot, tombstone,
/// closure-ledger, successor-route, and propagation truth into (whole-label matches so a full sentence
/// naming a concrete retired object, successor path, or registry reference is not flagged).
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
            | "retired"
            | "line"
            | "supported line"
            | "closure lane"
            | "closure"
            | "lane"
            | "retirement"
            | "retirement manifest"
            | "manifest"
            | "retired state"
            | "tombstone"
            | "archive"
            | "archival note"
            | "archival route"
            | "last supported"
            | "last supported snapshot"
            | "snapshot"
            | "successor"
            | "successor path"
            | "successor route"
            | "disable path"
            | "rollback"
            | "export rollback route"
            | "cutoff"
            | "cutoff date"
            | "propagation"
            | "deployment profile"
            | "closure ledger"
            | "support note"
            | "support note closure"
            | "migration outcome"
            | "migration"
            | "export class"
            | "export-class"
            | "export safe"
            | "internal only"
            | "evidence"
            | "release evidence"
            | "known limits"
            | "registry reference"
            | "line identity"
            | "build identity"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the retired-state
/// matrix heuristic so the reused [`M5RetiredStateDowngradeTrigger`] narrowings
/// serialize cleanly — the retired-state proof grammar carries only typed class tokens and opaque refs,
/// never raw secret values or endpoints.
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

/// Builds the canonical, checked-in M05-1246 certification packet. Certifies all five claimed M5
/// retired-state profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_retired_state_surface_certification_packet(
) -> RetiredStateProfileCertificationPacket {
    RetiredStateProfileCertificationPacket::new(RetiredStateProfileCertificationPacketInput {
        packet_id: RETIRED_STATE_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-15T00:00:00Z".to_owned(),
        matrix_ref: RETIRED_STATE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:retired-state-surface-certification:{id}"),
        RETIRED_STATE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> RetiredStateCertExportParity {
    RetiredStateCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: RetiredStateCertificationAxis) -> &'static str {
    match axis {
        RetiredStateCertificationAxis::Visual => {
            "retired-state identity, last-supported version / channel, cutoff date, successor path, disable path, archival note, migration outcome, support-note closure state, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled green release row alone, and no retired object still reads as selectable"
        }
        RetiredStateCertificationAxis::Keyboard => {
            "the same retired-state role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        RetiredStateCertificationAxis::ScreenReader => {
            "the same retired-state truth is announced non-visually, never a shell-chrome-only / mislabeled-release-row / unlabeled-control-only cue"
        }
        RetiredStateCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the retired-state identity, last-supported version, cutoff date, successor path, or registry reference"
        }
        RetiredStateCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the retired-state identity, last-supported version, or successor path"
        }
        RetiredStateCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a retired object name, object class, export class, or last-supported version"
        }
        RetiredStateCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        RetiredStateCertificationAxis::DegradedState => {
            "a stale retirement manifest, a missing last-supported snapshot, a lagging deployment-profile propagation, or an incomplete support-note closure honestly downgrades the CertifiedRetiredClosure/ReviewableRetirementRecord claim rather than reading as a fresh, fully closed retirement"
        }
        RetiredStateCertificationAxis::RetiredStateTruth => {
            "last-supported version / channel, cutoff date, successor path, disable path, export / rollback route, archival note, migration outcome, support-note closure state, and deployment-profile propagation stay explicit and never let a retirement let a surface disappear without a tombstone, archival route, or successor pointer, keep a retired class selectable in a new-install / new-tenant / marketplace / upgrade flow, destroy last-supported docs / schemas / evidence before support-note closure, leave retirement state unjoined to exact build, line identity, deployment profile, and migration outcome, or retire a surface through silent disappearance, stale selection UI, or orphaned support / docs truth"
        }
    }
}

fn seed_certified(axis: RetiredStateCertificationAxis) -> RetiredStateAxisOutcome {
    RetiredStateAxisOutcome {
        axis,
        state: RetiredStateAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: RetiredStateCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5RetiredStateDowngradeTrigger,
) -> RetiredStateAxisOutcome {
    RetiredStateAxisOutcome {
        axis,
        state: RetiredStateAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<RetiredStateAxisOutcome> {
    RetiredStateCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: RetiredStateCertificationAxis,
    outcome: RetiredStateAxisOutcome,
) -> Vec<RetiredStateAxisOutcome> {
    RetiredStateCertificationAxis::ALL
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
    profile: M5RetiredStateCertifiedProfile,
    claimed_claim: M5RetiredStateClaim,
    certified_claim: M5RetiredStateClaim,
    consumed_families: &[M5RetiredStateObject],
    axis_outcomes: Vec<RetiredStateAxisOutcome>,
    claim_auto_narrow: Option<RetiredStateClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> RetiredStateProfileCertificationRow {
    let mut row = RetiredStateProfileCertificationRow {
        record_kind: RETIRED_STATE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: RETIRED_STATE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: RetiredStateCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: RETIRED_STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: RetiredStateProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            RETIRED_STATE_CERT_MATRIX_REF.to_owned(),
            RETIRED_STATE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: RetiredStateCertificationAxis,
    from_claim: M5RetiredStateClaim,
    to_claim: M5RetiredStateClaim,
    label: &str,
) -> RetiredStateClaimAutoNarrow {
    RetiredStateClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<RetiredStateProfileCertificationRow> {
    use M5RetiredStateCertifiedProfile as P;
    use M5RetiredStateClaim::*;
    use M5RetiredStateDowngradeTrigger as Trig;
    use M5RetiredStateObject::*;
    use RetiredStateCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-retired-state-closure-lane",
            P::LiveRetiredStateClosureLane,
            CertifiedRetiredClosure,
            CertifiedRetiredClosure,
            &[SupportedLine, StableCapability],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "support_note_closure_state",
            ],
            &[
                "closure-lane class: a complete retirement manifest, an exact-build last-supported snapshot, a tombstone / archival route, a closed support note, a recorded migration outcome, and converged deployment-profile propagation all join to exact build / line identity, never a retirement widened past its closure evidence",
                "the certified retired closure keeps stable operation IDs while the retired-state identity, last-supported version, cutoff date, successor path, and registry reference bind to the one retired-state matrix across release / help / docs / support / marketplace / partner surfaces, and no retired object still reads as selectable",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered retired-state record",
                "retired-state-truth: a live, fully closed retired-state closure lane with current, export-safe, and internally consistent retirement evidence is the only profile that certifies a certified retired closure",
            ],
        ),
        seed_row(
            "cert:reviewable-retirement-record-structure",
            P::ReviewableRetirementRecordStructure,
            ReviewableRetirementRecord,
            ReviewableRetirementRecord,
            &[SchemaBearingSurface, CommandDeepLink],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "retired_state_identity",
            ],
            &[
                "record-structure class: an export-safe retirement manifest / last-supported snapshot / tombstone bound to one retired-state identity and inspectable before closure rather than a per-surface description copied by hand, with public-safe closure state separated from internal-only incident detail",
                "the reviewable retirement-record structure keeps its retired-state identity, last-supported version, successor path, and registry labels inspectable rather than a shell-chrome-only or mislabeled-release-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable retirement-record structure",
                "retired-state-truth: a reviewable retirement-record structure never certifies a live closed-closure claim and never stays green on a stale manifest or a lagging propagation",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-archive-partial-profile",
            P::DisclosedArchivePartialProfile,
            ReviewableRetirementRecord,
            ArchiveDisclosedProjection,
            &[Bundle],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the archive-partial lane carries a last-supported snapshot whose archive coverage can only be partially disclosed for this profile so a fully retained, build-joined archive cannot be certified",
                    "The archive-partial lane carries an archived last-supported bundle whose successor path and exact-build join can only be partially disclosed, so the ReviewableRetirementRecord claim narrows to an archive-disclosed projection and the lane discloses the archived bundle alongside its build join rather than presenting it as fully retained or letting a public-safe archival note read as complete",
                    Trig::ArchivalNoteMissing,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableRetirementRecord,
                ArchiveDisclosedProjection,
                "Archive coverage is only partially retained for this bundle, so it is disclosed alongside its successor path and exact-build join and never reads as a fully retained archive",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "archive-partial class: the archive names its last-supported version, successor path, and exact-build join and marks coverage as disclosed-partial rather than letting an archived last-supported bundle read as fully retained when its coverage is incomplete",
                "the archive-partial surface keeps its successor path and exact-build join legible while archive coverage is disclosed as partial",
                "localization: ReviewableRetirementRecord narrows to an archive-disclosed projection (auto-narrowed)",
                "retired-state-truth: a partially-retained archive never reads as fully retained — the successor path and exact-build join are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-propagation-profile",
            P::UnverifiedPropagationProfile,
            ReviewableRetirementRecord,
            PropagationUnverifiedProjection,
            &[RegistryVisiblePackage],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the mirror / offline / self-hosted / managed deployment-profile propagation has aged out or a profile still lags so a fully converged retirement cannot be certified",
                    "The mirror / offline / self-hosted / managed deployment-profile propagation has aged out or a profile still lags, so the ReviewableRetirementRecord claim narrows to a propagation-unverified projection and the lane keeps the last-known propagation posture explicit rather than staying green on a lagging profile or leaving a retired line still advertised after another profile closed it",
                    Trig::RetirementManifestStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableRetirementRecord,
                PropagationUnverifiedProjection,
                "Deployment-profile propagation has aged out or a profile still lags, so the last-known propagation posture stays explicit and no lagging profile reads as fully converged",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "propagation class: the closure ledger keeps its per-deployment-profile propagation status explicit and marks the propagation as unverified rather than staying green on a lagging profile when the propagation has aged out, and never keeps advertising a retired line after another profile closed it",
                "the propagation surface keeps its per-deployment-profile propagation status and last-published lineage legible while the propagation currency is disclosed as unverified",
                "degraded-state: ReviewableRetirementRecord narrows to a propagation-unverified projection (auto-narrowed)",
                "retired-state-truth: a deployment-profile propagation never reads as converged when a profile still lags and never lets a lagging profile imply a fully closed retirement",
            ],
        ),
        seed_row(
            "cert:unverified-closure-ledger-profile",
            P::UnverifiedClosureLedgerProfile,
            ReviewableRetirementRecord,
            ClosureLedgerUnverifiedProjection,
            &[ManagedTenantFeature],
            seed_certified_except(
                Ax::RetiredStateTruth,
                seed_narrowed(
                    Ax::RetiredStateTruth,
                    "a support-note closure or recorded migration outcome is missing or the closure ledger has become unreconstructable so a current, retained closure ledger cannot be certified",
                    "A support-note closure or recorded migration outcome is missing or the closure ledger has become unreconstructable, so the ReviewableRetirementRecord claim narrows to a closure-ledger-unverified projection and the lane keeps the last-known unretained-closure posture explicit rather than presenting a support-note closure as complete behind a green line",
                    Trig::SupportNoteClosureIncomplete,
                ),
            ),
            Some(seed_narrow(
                Ax::RetiredStateTruth,
                ReviewableRetirementRecord,
                ClosureLedgerUnverifiedProjection,
                "A support-note closure or migration outcome is missing, so the last-known unretained-closure posture stays explicit and no support-note closure reads as complete behind a green line",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "closure-ledger class: the ledger keeps its support-note closure and migration-outcome lineage explicit and marks the retention as unverified rather than leaving support-note or migration-outcome history unretained behind a green line",
                "the closure-ledger surface keeps its support-note closure and migration-outcome lineage legible while the closure retention is disclosed as unverified",
                "retired-state-truth: ReviewableRetirementRecord narrows to a closure-ledger-unverified projection (auto-narrowed)",
                "retired-state-truth: a support-note closure cites its retained closure ledger and never leaves support-note or migration-outcome history unretained, and no claim outpaces the retained closure",
            ],
        ),
    ]
}
