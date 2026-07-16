//! M05-1255 closing B149 surface certification over the frozen M5 historical-reference matrix — the
//! retirement / last-supported snapshots, captured support / export evidence bundles, archived runbook
//! execution packets, imported / offline route evidence, and review / incident snapshots that no longer
//! point at live mutable state.
//!
//! Where the freeze matrix ([`crate::m5_historical_reference_matrix`]) defines the five governed
//! non-live-evidence object classes, the M05-1248..1254 implement lanes resolve each historical-snapshot
//! descriptor, descriptor-change-diff, archived-snapshot viewer, historical-versus-live compare flow,
//! live-target handoff, expiry / removal state, imported / offline lineage-propagation, and drill-corpus
//! registry; this closing capstone *certifies* that the shared non-live-evidence truth holds on every
//! claimed M5 support, retirement, incident, review, and export surface — snapshot labels, capture time,
//! provenance, mutation-blocked posture, imported / offline warnings, expired / removed metadata fallback,
//! and validated open-live-target handoffs — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a support engineer, release operator, program-governance owner, or
//! review / incident owner reads a snapshot descriptor, archived packet, imported / offline evidence, or
//! live-target-handoff surface through (a current, fully-attributed non-live-evidence lane; a reviewable
//! snapshot-record structure; a disclosed imported / offline-partial profile; an unverified live-target
//! profile; and an unverified expiry / removal-ledger profile), not on the underlying object class or
//! implement lane. Each [`HistoricalEvidenceProfileCertificationRow`] certifies one profile across nine truth
//! axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export,
//! degraded-state, and non-live-evidence-truth behavior — and either passes (green), auto-narrows its
//! non-live-evidence claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded
//! axis is hidden behind a fresh certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedNonLiveEvidence` / `ReviewableSnapshotRecord` claim while one of its truth axes is not current
//! is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound
//! reason and a frozen downgrade trigger) is honestly yellow. Only a current, fully-attributed non-live
//! evidence lane — one whose snapshot label, capture time, provenance, mutation-blocked posture, and validated
//! live-target handoff (or metadata-only exit) all converge on one current, export-safe, internally consistent
//! non-live-evidence record — may certify a `CertifiedNonLiveEvidence` claim; a reviewable, imported /
//! offline-partial, unverified-live-target, or unverified-expiry / removal-ledger profile that keeps a
//! certified claim is over-reaching and blocks. The always-on CLI/export axis must always stay certified so
//! support and automation can reconstruct the snapshot label, capture time, provenance, live-target
//! availability, imported / offline status, mutation-blocked posture, and expiry / removal state from the same
//! non-live-evidence proof the operator saw.
//!
//! The B149 hard invariants are enforced per row: no profile may let archived or imported / offline evidence
//! look live, writable, or current by omission; reopen a live target from a snapshot without validating target
//! identity, trust, route, and authority; dead-link an expired / removed artifact when it can still show
//! metadata, provenance, or safe cleanup state; leave non-live evidence unjoined to capture time, provenance,
//! retention / removal state, or any current live-target mismatch; or present a snapshot or imported / offline
//! packet as a current live object or reopen through an ambiguous route. A profile that breaches any invariant
//! blocks (red).
//!
//! Every row cites exactly one canonical historical-reference matrix proof bundle
//! ([`HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen historical-reference matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets,
//! bearer tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-historical-evidence-surface-certification.schema.json`](../../../../schemas/release/m5-historical-evidence-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_historical_evidence_surface_certification.md`](../../../../docs/release/m5_historical_evidence_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_historical_reference_matrix as matrix;
use matrix::{M5HistoricalReferenceDowngradeTrigger, M5HistoricalReferenceObject};

/// Schema version stamped on the M05-1255 certification packet.
pub const HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`HistoricalEvidenceProfileCertificationPacket`].
pub const HISTORICAL_EVIDENCE_CERT_RECORD_KIND: &str =
    "m5_historical_evidence_surface_certification_packet";

/// Stable record-kind tag carried by each [`HistoricalEvidenceProfileCertificationRow`].
pub const HISTORICAL_EVIDENCE_CERT_ROW_RECORD_KIND: &str =
    "m5_historical_evidence_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const HISTORICAL_EVIDENCE_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-historical-evidence-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const HISTORICAL_EVIDENCE_CERT_DOC_REF: &str =
    "docs/release/m5_historical_evidence_surface_certification.md";

/// Repo-relative path of the frozen historical-reference matrix schema the certified profiles render.
pub const HISTORICAL_EVIDENCE_CERT_MATRIX_REF: &str =
    matrix::M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF;

/// The one canonical historical-reference matrix proof bundle every certified profile cites as its
/// first-resolved non-live-evidence truth. All five profiles point back to it rather than cloning per-profile
/// evidence.
pub const HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_HISTORICAL_REFERENCE_ARTIFACT_REF;

/// The historical-evidence-health dashboard the release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's non-live-evidence truth ties back to the same dashboard consumers
/// read.
pub const HISTORICAL_EVIDENCE_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_HISTORICAL_REFERENCE_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const HISTORICAL_EVIDENCE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-historical-evidence-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const HISTORICAL_EVIDENCE_CERT_CSV_REF: &str =
    "artifacts/release/m5-historical-evidence-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const HISTORICAL_EVIDENCE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-historical-evidence-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const HISTORICAL_EVIDENCE_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-historical-evidence-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const HISTORICAL_EVIDENCE_CERT_PACKET_ID: &str =
    "m5-historical-evidence-surface-certification:stable:0001";

/// The five claimed M5 historical-evidence profiles this capstone certifies. Keyed on the profile
/// a support engineer, release operator, program-governance owner, or review / incident owner reads a
/// snapshot descriptor, archived packet, imported / offline evidence, or live-target-handoff surface through,
/// not on the reusable object class it renders. Only a current, fully-attributed non-live-evidence lane
/// profile may certify a certified non-live-evidence claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalEvidenceCertifiedProfile {
    /// A current, fully-attributed non-live-evidence lane — an archived-snapshot / captured-evidence class
    /// whose snapshot label, capture time, provenance lineage, mutation-blocked posture, and validated
    /// live-target handoff (or metadata-only exit) converge on one current, joined, export-safe
    /// non-live-evidence record, certifying the non-live-evidence claim exactly right now.
    CurrentNonLiveEvidenceLane,
    /// A reviewable snapshot-record structure: a self-sufficient, inspectable non-live-evidence projection (a
    /// snapshot descriptor / captured-evidence / archived-packet record an operator can review), never
    /// itself a current, fully-attributed non-live-evidence lane.
    ReviewableSnapshotRecordStructure,
    /// An imported / offline-partial lane whose imported / offline evidence coverage can only be partially
    /// disclosed; the claim narrows to an imported / offline-disclosed projection that discloses the imported /
    /// offline evidence alongside its source-snapshot descriptor and live-target mismatch, never imported /
    /// offline evidence shown as current route, service, or workspace truth while its coverage or live-route
    /// join is incomplete.
    DisclosedImportedOfflinePartialProfile,
    /// A live-target lane whose live-target availability (target existence, scope, route, trust, or authority)
    /// has aged out or become unresolvable; the claim narrows to a live-target-unverified projection that keeps
    /// the last-known live-target posture explicit, never a snapshot shown as reopenable when its target can no
    /// longer be validated or reopened through an ambiguous route.
    UnverifiedLiveTargetProfile,
    /// An expiry / removal-ledger lane whose retention / removal metadata (retention receipt, deletion receipt,
    /// or closure ledger) has aged out or become unreconstructable; the claim narrows to an expiry /
    /// removal-unverified projection that keeps the last-known unretained metadata posture explicit, never an
    /// expired / removed artifact dead-linked or presented as live behind a green line when its metadata is
    /// incomplete.
    UnverifiedExpiryRemovalLedgerProfile,
}

impl M5HistoricalEvidenceCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5HistoricalEvidenceCertifiedProfile; 5] = [
        M5HistoricalEvidenceCertifiedProfile::CurrentNonLiveEvidenceLane,
        M5HistoricalEvidenceCertifiedProfile::ReviewableSnapshotRecordStructure,
        M5HistoricalEvidenceCertifiedProfile::DisclosedImportedOfflinePartialProfile,
        M5HistoricalEvidenceCertifiedProfile::UnverifiedLiveTargetProfile,
        M5HistoricalEvidenceCertifiedProfile::UnverifiedExpiryRemovalLedgerProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentNonLiveEvidenceLane => "current_non_live_evidence_lane",
            Self::ReviewableSnapshotRecordStructure => "reviewable_snapshot_record_structure",
            Self::DisclosedImportedOfflinePartialProfile => {
                "disclosed_imported_offline_partial_profile"
            }
            Self::UnverifiedLiveTargetProfile => "unverified_live_target_profile",
            Self::UnverifiedExpiryRemovalLedgerProfile => {
                "unverified_expiry_removal_ledger_profile"
            }
        }
    }

    /// True only for the current, fully-attributed non-live-evidence lane profile. A certified
    /// non-live-evidence claim may be certified on this profile alone; every other profile is at most a
    /// reviewable snapshot-record structure or a narrowed projection.
    pub const fn is_current_non_live_evidence_lane(self) -> bool {
        matches!(self, Self::CurrentNonLiveEvidenceLane)
    }
}

/// The claim ladder a certified historical-evidence profile asserts and is certified down to. Minted locally
/// for this capstone (B149 folds accessibility into the cert): the strongest claim is a fully certified
/// non-live-evidence record; each weaker tier is a disclosed projection that keeps the last-known imported /
/// offline, live-target, or expiry / removal posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalEvidenceClaim {
    /// Certified non-live evidence: a fully-attributed archived-snapshot / captured-evidence class with a
    /// snapshot label, capture time, provenance lineage, mutation-blocked posture, and a validated live-target
    /// handoff (or metadata-only exit) all joined to its capture context — the strongest claim, non-live
    /// evidence Aureline can present as cleanly-attributed and honestly non-live right now.
    CertifiedNonLiveEvidence,
    /// Reviewable snapshot record: a self-sufficient, inspectable read-only non-live-evidence projection
    /// (a static snapshot descriptor / captured-evidence / archived-packet record an operator can inspect)
    /// that is not itself a current, fully-attributed non-live-evidence lane.
    ReviewableSnapshotRecord,
    /// Imported / offline-disclosed projection: an imported / offline-partial lane's evidence coverage can
    /// only be partially disclosed; the lane stays an imported / offline-disclosed projection that discloses
    /// the imported / offline evidence alongside its source-snapshot descriptor and live-route mismatch, never
    /// imported / offline evidence shown as current route, service, or workspace truth while its coverage is
    /// incomplete.
    ImportedOfflineDisclosedProjection,
    /// Live-target-unverified projection: a live-target lane's target existence, scope, route, trust, or
    /// authority has aged out or become unresolvable; the lane stays a live-target-unverified projection that
    /// keeps the last-known live-target posture explicit, never a snapshot shown as reopenable when its target
    /// can no longer be validated.
    LiveTargetUnverifiedProjection,
    /// Expiry / removal-unverified projection: an expiry / removal-ledger lane's retention / removal metadata
    /// has aged out or become unreconstructable; the lane stays an expiry / removal-unverified projection that
    /// keeps the last-known unretained metadata posture explicit, never an expired / removed artifact
    /// dead-linked or presented as live behind a green line.
    ExpiryRemovalUnverifiedProjection,
}

impl M5HistoricalEvidenceClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::CertifiedNonLiveEvidence,
        Self::ReviewableSnapshotRecord,
        Self::ImportedOfflineDisclosedProjection,
        Self::LiveTargetUnverifiedProjection,
        Self::ExpiryRemovalUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedNonLiveEvidence => 4,
            Self::ReviewableSnapshotRecord => 3,
            Self::ImportedOfflineDisclosedProjection => 2,
            Self::LiveTargetUnverifiedProjection => 1,
            Self::ExpiryRemovalUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-attributed, current non-live-evidence record.
    pub const fn asserts_certified_non_live_evidence(self) -> bool {
        matches!(self, Self::CertifiedNonLiveEvidence)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedNonLiveEvidence | Self::ReviewableSnapshotRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedNonLiveEvidence => "certified_non_live_evidence",
            Self::ReviewableSnapshotRecord => "reviewable_snapshot_record",
            Self::ImportedOfflineDisclosedProjection => "imported_offline_disclosed_projection",
            Self::LiveTargetUnverifiedProjection => "live_target_unverified_projection",
            Self::ExpiryRemovalUnverifiedProjection => "expiry_removal_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and non-live-evidence-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalEvidenceCertificationAxis {
    /// Visual parity: the snapshot label, capture time, provenance, live-target availability, imported /
    /// offline status, mutation-blocked posture, and expiry / removal state are shown on the primary surface
    /// without relying on a shell-chrome-only affordance or a mislabeled live-looking row alone, and no
    /// archived or imported / offline object still reads as a current live object.
    Visual,
    /// Keyboard-reach parity: the same non-live-evidence truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled live-looking row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// snapshot label, capture time, provenance, live-target availability, or expiry / removal state.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the snapshot label, capture time, or live-target availability.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// snapshot label, object class, imported / offline status, or capture time when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale snapshot descriptor, an unresolved live target, an unknown imported /
    /// offline status, or an incomplete retention / removal state honestly downgrades a
    /// `CertifiedNonLiveEvidence` / `ReviewableSnapshotRecord` claim rather than reading as a fresh, fully
    /// attributed non-live-evidence record.
    DegradedState,
    /// Non-live-evidence-truth parity: the snapshot label, capture time, provenance, live-target availability,
    /// imported / offline status, mutation-blocked posture, and expiry / removal state stay explicit and never
    /// let archived or imported / offline evidence look live, writable, or current by omission; reopen a live
    /// target from a snapshot without validating identity, trust, route, and authority; dead-link an expired /
    /// removed artifact when metadata, provenance, or cleanup state can be shown; leave non-live evidence
    /// unjoined to capture time, provenance, retention / removal state, or any current live-target mismatch;
    /// or present a snapshot or imported / offline packet as a current live object or reopen through an
    /// ambiguous route.
    NonLiveEvidenceTruth,
}

impl HistoricalEvidenceCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [HistoricalEvidenceCertificationAxis; 9] = [
        HistoricalEvidenceCertificationAxis::Visual,
        HistoricalEvidenceCertificationAxis::Keyboard,
        HistoricalEvidenceCertificationAxis::ScreenReader,
        HistoricalEvidenceCertificationAxis::HighZoomReflow,
        HistoricalEvidenceCertificationAxis::HighContrast,
        HistoricalEvidenceCertificationAxis::Localization,
        HistoricalEvidenceCertificationAxis::CliExport,
        HistoricalEvidenceCertificationAxis::DegradedState,
        HistoricalEvidenceCertificationAxis::NonLiveEvidenceTruth,
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
            Self::NonLiveEvidenceTruth => "non_live_evidence_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalEvidenceAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl HistoricalEvidenceAxisCertificationState {
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
pub enum HistoricalEvidenceProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live-evidence profile claims a certified non-live-evidence record, or the narrowing is inconsistent.
    Red,
}

impl HistoricalEvidenceProfileClaimStatus {
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

/// The five B149 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceCertGuardrails {
    /// True if the profile lets archived or imported / offline evidence look live, writable, or current by
    /// omission. Must be false.
    pub lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: bool,
    /// True if the profile reopens a live target from a snapshot without validating target identity, trust,
    /// route, and authority first. Must be false.
    pub reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority:
        bool,
    /// True if the profile dead-links an expired / removed historical artifact when it can still show
    /// metadata, provenance, or safe cleanup state. Must be false.
    pub dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state:
        bool,
    /// True if the profile leaves non-live evidence unjoined to capture time, provenance, retention / removal
    /// state, or any current live-target mismatch. Must be false.
    pub leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch:
        bool,
    /// True if the profile presents a snapshot or imported / offline packet as a current live object or
    /// reopens through an ambiguous route. Must be false.
    pub presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route:
        bool,
}

impl HistoricalEvidenceCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: false,
        reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority:
            false,
        dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state:
            false,
        leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch:
            false,
        presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route:
            false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission
            && !self.reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority
            && !self.dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state
            && !self.leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch
            && !self.presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The snapshot-label / capture-time / provenance / live-target-availability / imported-offline-status /
    /// mutation-blocked-posture / expiry-removal-state fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl HistoricalEvidenceCertExportParity {
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
pub struct HistoricalEvidenceAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: HistoricalEvidenceCertificationAxis,
    /// The certification state of the axis.
    pub state: HistoricalEvidenceAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5HistoricalReferenceDowngradeTrigger>,
}

impl HistoricalEvidenceAxisOutcome {
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
            HistoricalEvidenceAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            HistoricalEvidenceAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            HistoricalEvidenceAxisCertificationState::UndisclosedDrift => {
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
pub struct HistoricalEvidenceClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: HistoricalEvidenceCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5HistoricalEvidenceClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5HistoricalEvidenceClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 historical-evidence object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceProfileCertificationRow {
    /// Record kind; must equal [`HISTORICAL_EVIDENCE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5HistoricalEvidenceCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5HistoricalEvidenceClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5HistoricalEvidenceClaim,
    /// The frozen non-live-evidence object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5HistoricalReferenceObject>,
    /// One outcome per [`HistoricalEvidenceCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<HistoricalEvidenceAxisOutcome>,
    /// The B149 hard invariants; all must hold.
    pub guardrails: HistoricalEvidenceCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<HistoricalEvidenceClaimAutoNarrow>,
    /// The one canonical historical-reference matrix proof bundle this profile cites. Must equal
    /// [`HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: HistoricalEvidenceProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: HistoricalEvidenceCertExportParity,
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

impl HistoricalEvidenceProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: HistoricalEvidenceCertificationAxis,
    ) -> Option<&HistoricalEvidenceAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<HistoricalEvidenceCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && HistoricalEvidenceCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(HistoricalEvidenceAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<HistoricalEvidenceCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == HistoricalEvidenceAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a current non-live-evidence lane
    /// profile may certify a certified non-live-evidence record, every hard invariant must hold, CLI/export
    /// parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> HistoricalEvidenceProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        // Every B149 hard invariant must hold.
        if !self.guardrails.all_held() {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        // Only a current non-live-evidence lane profile may certify a certified non-live-evidence record.
        if self.certified_claim.asserts_certified_non_live_evidence()
            && !self.profile.is_current_non_live_evidence_lane()
        {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(HistoricalEvidenceCertificationAxis::CliExport) {
            Some(o) if o.state == HistoricalEvidenceAxisCertificationState::Certified => {}
            _ => return HistoricalEvidenceProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == HistoricalEvidenceAxisCertificationState::UndisclosedDrift)
        {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return HistoricalEvidenceProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return HistoricalEvidenceProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return HistoricalEvidenceProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return HistoricalEvidenceProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return HistoricalEvidenceProfileClaimStatus::Red;
        }

        HistoricalEvidenceProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == HISTORICAL_EVIDENCE_CERT_ROW_RECORD_KIND
            && self.schema_version == HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1255 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceProfileCertificationSummary {
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

/// Constructor input for [`HistoricalEvidenceProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalEvidenceProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<HistoricalEvidenceProfileCertificationRow>,
}

/// Checked-in M05-1255 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<HistoricalEvidenceProfileCertificationRow>,
    pub summary: HistoricalEvidenceProfileCertificationSummary,
}

impl HistoricalEvidenceProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: HistoricalEvidenceProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION,
            record_kind: HISTORICAL_EVIDENCE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: HistoricalEvidenceProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5HistoricalEvidenceCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Non-live-evidence object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5HistoricalReferenceObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5HistoricalEvidenceCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen non-live-evidence object class is certified on at least one profile — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5HistoricalReferenceObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(HistoricalEvidenceCertificationAxis::CliExport)
                .is_some_and(|o| o.state == HistoricalEvidenceAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> HistoricalEvidenceProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistoricalEvidenceProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistoricalEvidenceProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == HistoricalEvidenceProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(HistoricalEvidenceProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        HistoricalEvidenceProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(HistoricalEvidenceProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<HistoricalEvidenceCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION {
            violations.push(HistoricalEvidenceCertificationViolation::SchemaVersion {
                expected: HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != HISTORICAL_EVIDENCE_CERT_RECORD_KIND {
            violations.push(HistoricalEvidenceCertificationViolation::RecordKind {
                expected: HISTORICAL_EVIDENCE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(HistoricalEvidenceCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(HistoricalEvidenceCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(HistoricalEvidenceCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(HistoricalEvidenceCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    HistoricalEvidenceCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    HistoricalEvidenceCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    HistoricalEvidenceCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B149 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    HistoricalEvidenceCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a current non-live-evidence lane profile may certify a certified non-live-evidence record.
            if row.certified_claim.asserts_certified_non_live_evidence()
                && !row.profile.is_current_non_live_evidence_lane()
            {
                violations.push(
                    HistoricalEvidenceCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(HistoricalEvidenceCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    HistoricalEvidenceCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    HistoricalEvidenceCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    HistoricalEvidenceCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == HistoricalEvidenceProfileClaimStatus::Red {
                violations.push(HistoricalEvidenceCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(HistoricalEvidenceCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen non-live-evidence object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(HistoricalEvidenceCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(HistoricalEvidenceCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                HistoricalEvidenceCertificationViolation::RawHistoricalEvidenceMaterialInExport,
            );
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
        out.push_str("# M5 Historical-Evidence Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5HistoricalEvidenceCertifiedProfile::ALL.len(),
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
pub fn current_m5_historical_evidence_surface_certification_export() -> Result<
    HistoricalEvidenceProfileCertificationPacket,
    HistoricalEvidenceCertificationArtifactError,
> {
    let packet: HistoricalEvidenceProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-historical-evidence-surface-certification/support_export.json"
        )))
        .map_err(HistoricalEvidenceCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HistoricalEvidenceCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum HistoricalEvidenceCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HistoricalEvidenceCertificationViolation>),
}

impl fmt::Display for HistoricalEvidenceCertificationArtifactError {
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

impl Error for HistoricalEvidenceCertificationArtifactError {}

/// Validation failure for M05-1255 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoricalEvidenceCertificationViolation {
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
    RawHistoricalEvidenceMaterialInExport,
}

impl fmt::Display for HistoricalEvidenceCertificationViolation {
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
                    "packet does not cite the canonical historical-reference matrix proof bundle"
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
                    "row {id} does not cite the one canonical historical-reference matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B149 hard invariant: letting archived or imported / offline evidence \
look live, writable, or current by omission; reopening a live target from a snapshot without validating \
identity, trust, route, and authority; dead-linking an expired / removed artifact when metadata, provenance, \
or cleanup state can be shown; leaving non-live evidence unjoined to capture time, provenance, retention / \
removal state, or any current live-target mismatch; or presenting a snapshot or imported / offline packet as \
a current live object or reopening through an ambiguous route"
                )
            }
            Self::NonLiveProfileClaimsTrustedLane { id } => {
                write!(
                    f,
                    "row {id} certifies a certified non-live-evidence record on a non-current-lane profile"
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
invariant broke, CLI/export parity dropped, a non-current-lane profile claimed a certified non-live-evidence \
record, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 historical-evidence profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen non-live-evidence object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawHistoricalEvidenceMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for HistoricalEvidenceCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&HistoricalEvidenceAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != HistoricalEvidenceAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the non-live-evidence
/// generics the spec forbids collapsing distinct snapshot-descriptor, live-target-handoff, imported / offline
/// evidence, and expiry / removal truth into (whole-label matches so a full sentence naming a concrete
/// snapshot, capture time, provenance, or live target is not flagged).
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
            | "non-live"
            | "non live"
            | "non-live evidence"
            | "evidence"
            | "snapshot"
            | "snapshot label"
            | "captured evidence"
            | "archived snapshot"
            | "archive"
            | "archived"
            | "capture time"
            | "provenance"
            | "capture context"
            | "live target"
            | "live target availability"
            | "live target handoff"
            | "handoff"
            | "open live object"
            | "open current live object"
            | "metadata only"
            | "metadata-only exit"
            | "imported"
            | "offline evidence"
            | "imported offline"
            | "imported or offline evidence"
            | "imported/offline"
            | "expired"
            | "removed"
            | "expiry"
            | "removal"
            | "retention"
            | "retention state"
            | "retention window"
            | "cleanup"
            | "cleanup state"
            | "dead link"
            | "mutation blocked"
            | "read only"
            | "read-only"
            | "descriptor"
            | "source snapshot"
            | "lineage"
            | "mismatch"
            | "drift"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the historical-reference
/// matrix heuristic so the reused [`M5HistoricalReferenceDowngradeTrigger`] narrowings
/// serialize cleanly — the non-live-evidence proof grammar carries only typed class tokens and opaque refs,
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

/// Builds the canonical, checked-in M05-1255 certification packet. Certifies all five claimed M5
/// historical-evidence profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_historical_evidence_surface_certification_packet(
) -> HistoricalEvidenceProfileCertificationPacket {
    HistoricalEvidenceProfileCertificationPacket::new(
        HistoricalEvidenceProfileCertificationPacketInput {
            packet_id: HISTORICAL_EVIDENCE_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-16T00:00:00Z".to_owned(),
            matrix_ref: HISTORICAL_EVIDENCE_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:historical-evidence-surface-certification:{id}"),
        HISTORICAL_EVIDENCE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> HistoricalEvidenceCertExportParity {
    HistoricalEvidenceCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: HistoricalEvidenceCertificationAxis) -> &'static str {
    match axis {
        HistoricalEvidenceCertificationAxis::Visual => {
            "snapshot label, capture time, provenance, live-target availability, imported / offline status, mutation-blocked posture, and expiry / removal state shown on-surface without a shell-chrome-only affordance or a mislabeled live-looking row alone, and no archived or imported / offline object still reads as a current live object"
        }
        HistoricalEvidenceCertificationAxis::Keyboard => {
            "the same non-live-evidence role, capture context, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        HistoricalEvidenceCertificationAxis::ScreenReader => {
            "the same non-live-evidence truth is announced non-visually, never a shell-chrome-only / mislabeled-live-row / unlabeled-control-only cue"
        }
        HistoricalEvidenceCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the snapshot label, capture time, provenance, live-target availability, or expiry / removal state"
        }
        HistoricalEvidenceCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the snapshot label, capture time, or live-target availability"
        }
        HistoricalEvidenceCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a snapshot label, object class, imported / offline status, or capture time"
        }
        HistoricalEvidenceCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        HistoricalEvidenceCertificationAxis::DegradedState => {
            "a stale snapshot descriptor, an unresolved live target, an unknown imported / offline status, or an incomplete retention / removal state honestly downgrades the CertifiedNonLiveEvidence/ReviewableSnapshotRecord claim rather than reading as a fresh, fully attributed non-live-evidence record"
        }
        HistoricalEvidenceCertificationAxis::NonLiveEvidenceTruth => {
            "snapshot label, capture time, provenance, live-target availability, imported / offline status, mutation-blocked posture, and expiry / removal state stay explicit and never let archived or imported / offline evidence look live, writable, or current by omission, reopen a live target from a snapshot without validating identity, trust, route, and authority, dead-link an expired / removed artifact when metadata, provenance, or cleanup state can be shown, leave non-live evidence unjoined to capture time, provenance, retention / removal state, or any current live-target mismatch, or present a snapshot or imported / offline packet as a current live object or reopen through an ambiguous route"
        }
    }
}

fn seed_certified(axis: HistoricalEvidenceCertificationAxis) -> HistoricalEvidenceAxisOutcome {
    HistoricalEvidenceAxisOutcome {
        axis,
        state: HistoricalEvidenceAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: HistoricalEvidenceCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5HistoricalReferenceDowngradeTrigger,
) -> HistoricalEvidenceAxisOutcome {
    HistoricalEvidenceAxisOutcome {
        axis,
        state: HistoricalEvidenceAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<HistoricalEvidenceAxisOutcome> {
    HistoricalEvidenceCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: HistoricalEvidenceCertificationAxis,
    outcome: HistoricalEvidenceAxisOutcome,
) -> Vec<HistoricalEvidenceAxisOutcome> {
    HistoricalEvidenceCertificationAxis::ALL
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
    profile: M5HistoricalEvidenceCertifiedProfile,
    claimed_claim: M5HistoricalEvidenceClaim,
    certified_claim: M5HistoricalEvidenceClaim,
    consumed_families: &[M5HistoricalReferenceObject],
    axis_outcomes: Vec<HistoricalEvidenceAxisOutcome>,
    claim_auto_narrow: Option<HistoricalEvidenceClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> HistoricalEvidenceProfileCertificationRow {
    let mut row = HistoricalEvidenceProfileCertificationRow {
        record_kind: HISTORICAL_EVIDENCE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: HISTORICAL_EVIDENCE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: HistoricalEvidenceCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: HISTORICAL_EVIDENCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: HistoricalEvidenceProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            HISTORICAL_EVIDENCE_CERT_MATRIX_REF.to_owned(),
            HISTORICAL_EVIDENCE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: HistoricalEvidenceCertificationAxis,
    from_claim: M5HistoricalEvidenceClaim,
    to_claim: M5HistoricalEvidenceClaim,
    label: &str,
) -> HistoricalEvidenceClaimAutoNarrow {
    HistoricalEvidenceClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<HistoricalEvidenceProfileCertificationRow> {
    use HistoricalEvidenceCertificationAxis as Ax;
    use M5HistoricalEvidenceCertifiedProfile as P;
    use M5HistoricalEvidenceClaim::*;
    use M5HistoricalReferenceDowngradeTrigger as Trig;
    use M5HistoricalReferenceObject::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:current-non-live-evidence-lane",
            P::CurrentNonLiveEvidenceLane,
            CertifiedNonLiveEvidence,
            CertifiedNonLiveEvidence,
            &[RetirementSnapshot],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "live_target_availability",
            ],
            &[
                "current non-live-evidence lane: a snapshot label, capture time, provenance lineage, mutation-blocked posture, and a validated live-target handoff (or metadata-only exit) all join to one capture context, never a snapshot widened past its capture evidence or reopened without validation",
                "the certified non-live-evidence record keeps stable operation IDs while the snapshot label, capture time, provenance, live-target availability, and expiry / removal state bind to the one historical-reference matrix across shell / help / docs / support / review / runbook-archive / companion / release / governance / CLI surfaces, and no archived object still reads as a current live object",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered non-live-evidence record",
                "non-live-evidence-truth: a current, fully-attributed non-live-evidence lane with current, export-safe, and internally consistent capture evidence is the only profile that certifies a certified non-live-evidence record",
            ],
        ),
        seed_row(
            "cert:reviewable-snapshot-record-structure",
            P::ReviewableSnapshotRecordStructure,
            ReviewableSnapshotRecord,
            ReviewableSnapshotRecord,
            &[SupportExportEvidence],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "snapshot_label",
            ],
            &[
                "record-structure class: an export-safe snapshot descriptor / captured support / export evidence packet bound to one capture context and inspectable rather than a per-surface description copied by hand, with public-safe capture metadata separated from internal-only incident detail",
                "the reviewable snapshot-record structure keeps its snapshot label, capture time, provenance, and live-target availability inspectable rather than a shell-chrome-only or mislabeled-live-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable snapshot-record structure",
                "non-live-evidence-truth: a reviewable snapshot-record structure never certifies a current-lane claim and never stays green on a stale descriptor or an unresolved live target",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-imported-offline-partial-profile",
            P::DisclosedImportedOfflinePartialProfile,
            ReviewableSnapshotRecord,
            ImportedOfflineDisclosedProjection,
            &[ImportedOfflineRouteEvidence],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the imported / offline-partial lane carries route evidence whose imported / offline coverage can only be partially disclosed for this profile so a fully current, live-route-joined evidence packet cannot be certified",
                    "The imported / offline-partial lane carries imported / offline route evidence whose source-snapshot descriptor and live-route join can only be partially disclosed, so the ReviewableSnapshotRecord claim narrows to an imported / offline-disclosed projection and the lane discloses the imported / offline evidence alongside its source-snapshot descriptor rather than presenting it as current route, service, or workspace truth or letting a public-safe imported / offline label read as current",
                    Trig::ImportedOfflineEvidenceShownAsCurrent,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableSnapshotRecord,
                ImportedOfflineDisclosedProjection,
                "Imported / offline coverage is only partially disclosed for this route packet, so it is shown alongside its source-snapshot descriptor and live-route mismatch and never reads as current route truth",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "imported / offline-partial class: the evidence names its source-snapshot descriptor, imported / offline status, and live-route mismatch and marks coverage as disclosed-partial rather than letting imported / offline route evidence read as current route truth when its coverage is incomplete",
                "the imported / offline-partial surface keeps its source-snapshot descriptor and live-route mismatch legible while imported / offline coverage is disclosed as partial",
                "localization: ReviewableSnapshotRecord narrows to an imported / offline-disclosed projection (auto-narrowed)",
                "non-live-evidence-truth: partially-disclosed imported / offline evidence never reads as current route, service, or workspace truth — the source-snapshot descriptor and live-route mismatch are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-live-target-profile",
            P::UnverifiedLiveTargetProfile,
            ReviewableSnapshotRecord,
            LiveTargetUnverifiedProjection,
            &[ArchivedRunbookPacket],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the live target's existence, scope, route, trust, or authority can no longer be validated so a reopenable snapshot cannot be certified and the archive stays analysis-only",
                    "The live target's existence, scope, route, trust, or authority can no longer be validated, so the ReviewableSnapshotRecord claim narrows to a live-target-unverified projection and the lane keeps the last-known live-target posture explicit rather than staying green on a reopenable snapshot or reopening a live target through an ambiguous route without validating identity, trust, route, and authority",
                    Trig::LiveTargetAvailabilityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableSnapshotRecord,
                LiveTargetUnverifiedProjection,
                "The live target can no longer be validated, so the last-known live-target posture stays explicit and no snapshot reads as reopenable or reopens through an ambiguous route",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "live-target class: the handoff keeps its target existence, scope, route, trust, and authority checks explicit and marks the live target as unverified rather than staying green on a reopenable snapshot when the target can no longer be validated, and never reopens through an ambiguous route",
                "the live-target surface keeps its target identity and validation checks legible while the live-target availability is disclosed as unverified",
                "degraded-state: ReviewableSnapshotRecord narrows to a live-target-unverified projection (auto-narrowed)",
                "non-live-evidence-truth: a live target never reads as reopenable when its identity, trust, route, or authority can no longer be validated and never lets a snapshot imply a safe reopen",
            ],
        ),
        seed_row(
            "cert:unverified-expiry-removal-ledger-profile",
            P::UnverifiedExpiryRemovalLedgerProfile,
            ReviewableSnapshotRecord,
            ExpiryRemovalUnverifiedProjection,
            &[ReviewIncidentSnapshot],
            seed_certified_except(
                Ax::NonLiveEvidenceTruth,
                seed_narrowed(
                    Ax::NonLiveEvidenceTruth,
                    "a retention receipt, deletion receipt, or closure ledger is missing or the expiry / removal metadata has become unreconstructable so a current, retained expiry / removal ledger cannot be certified",
                    "A retention receipt, deletion receipt, or closure ledger is missing or the expiry / removal metadata has become unreconstructable, so the ReviewableSnapshotRecord claim narrows to an expiry / removal-unverified projection and the lane keeps the last-known unretained metadata posture explicit and still renders capture time, provenance, and the removal / expiry reason rather than dead-linking the artifact or presenting it as live behind a green line",
                    Trig::ExpiredArtifactDeadLinked,
                ),
            ),
            Some(seed_narrow(
                Ax::NonLiveEvidenceTruth,
                ReviewableSnapshotRecord,
                ExpiryRemovalUnverifiedProjection,
                "A retention or deletion receipt is missing, so the last-known unretained metadata posture stays explicit and no expired / removed artifact is dead-linked or presented as live behind a green line",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "expiry / removal-ledger class: the ledger keeps its retention / deletion receipt and closure-ledger lineage explicit and marks the retention as unverified rather than dead-linking an expired / removed artifact or leaving removal metadata unretained behind a green line",
                "the expiry / removal-ledger surface keeps its capture time, provenance, and removal / expiry reason legible while the retention metadata is disclosed as unverified",
                "non-live-evidence-truth: ReviewableSnapshotRecord narrows to an expiry / removal-unverified projection (auto-narrowed)",
                "non-live-evidence-truth: an expired / removed artifact cites its retention / removal metadata and never dead-links or reads as live, and no claim outpaces the retained metadata",
            ],
        ),
    ]
}
