//! M05-1237 closing B147 surface certification over the frozen M5 public-proof-ledger /
//! transparency-report / migration-scoreboard / orr-history-event / correction-train-archive
//! supported-line-transparency matrix.
//!
//! Where the freeze matrix ([`crate::m5_supported_line_transparency_matrix`]) defines the five governed
//! supported-line proof objects, the M05-1230..1236 implement lanes resolve each public-proof-ledger,
//! claim-history-diff, transparency-report, snapshot-diff, migration-scoreboard, scoreboard-delta,
//! ORR-history, follow-up-closure, correction-train-archive, closure-gate, truth-feed, audience-packet,
//! retention-policy, and stale-escalation registry; this closing capstone *certifies* that the shared
//! durable-proof truth holds on every claimed M5 supported line — current public-proof ledgers, export-safe
//! transparency reports, versioned migration scoreboards, retained ORR history, and archived correction
//! trains — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a release engineer, release operator, program-governance owner, or
//! support engineer reads a public-proof, transparency-report, migration-scoreboard, ORR-history,
//! correction-archive, freshness-window, or export-class surface through (a live, first-party supported-line
//! operating lane; a reviewable transparency structure; a disclosed correction-archive profile; an
//! unverified migration-scoreboard profile; and an unverified ORR-history profile), not on the underlying
//! proof object or implement lane. Each [`SupportedLineTransparencyProfileCertificationRow`] certifies one
//! profile across nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and supported-line-proof-truth behavior — and either passes
//! (green), auto-narrows its operating claim to the weakest supported ceiling (yellow), or is blocked (red)
//! when a degraded axis is hidden behind a fresh certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedOperatingLine` / `ReviewableTransparencySurface` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party supported-line
//! operating lane with current, export-safe, and internally consistent public-proof, transparency,
//! migration-scoreboard, ORR-history, and correction-archive evidence may certify a `CertifiedOperatingLine`
//! claim — a reviewable, disclosed-correction-archive, unverified-migration-scoreboard, or
//! unverified-ORR-history profile that keeps a certified claim is over-reaching and blocks. The always-on
//! CLI/export axis must always stay certified so support and automation can reconstruct the public-proof-ledger
//! truth, transparency snapshot, migration-scoreboard currency, ORR-history retention, correction-archive
//! retention, freshness window, export class, supported-line association, and registry reference from the same
//! supported-line proof the operator saw.
//!
//! The B147 hard invariants are enforced per row: no profile may widen a claim because a report once existed
//! without current freshness, stay green on stale external proof or opaque upstream health, leak internal-only
//! incident or security detail into a public-safe feed, leave public-proof / migration / history unjoined to
//! build and release-line identity, or leave migration pain / ORR / correction history unretained. A profile
//! that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical supported-line-transparency proof bundle
//! ([`SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen supported-line-transparency matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets,
//! bearer tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-supported-line-surface-certification.schema.json`](../../../../schemas/release/m5-supported-line-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_supported_line_surface_certification.md`](../../../../docs/release/m5_supported_line_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_supported_line_transparency_matrix as matrix;
use matrix::{M5SupportedLineTransparencyDowngradeTrigger, M5SupportedLineTransparencyObject};

/// Schema version stamped on the M05-1228 certification packet.
pub const SUPPORTED_LINE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`SupportedLineTransparencyProfileCertificationPacket`].
pub const SUPPORTED_LINE_CERT_RECORD_KIND: &str = "m5_supported_line_surface_certification_packet";

/// Stable record-kind tag carried by each [`SupportedLineTransparencyProfileCertificationRow`].
pub const SUPPORTED_LINE_CERT_ROW_RECORD_KIND: &str = "m5_supported_line_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const SUPPORTED_LINE_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-supported-line-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const SUPPORTED_LINE_CERT_DOC_REF: &str =
    "docs/release/m5_supported_line_surface_certification.md";

/// Repo-relative path of the frozen supported-line-transparency matrix schema the certified profiles render.
pub const SUPPORTED_LINE_CERT_MATRIX_REF: &str =
    matrix::M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF;

/// The one canonical supported-line-transparency proof bundle every certified profile cites as its
/// first-resolved supported-line truth. All five profiles point back to it rather than cloning per-profile
/// evidence.
pub const SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_SUPPORTED_LINE_TRANSPARENCY_ARTIFACT_REF;

/// The supported-line public-proof dashboard the release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's supported-line truth ties back to the same dashboard consumers read.
pub const SUPPORTED_LINE_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_SUPPORTED_LINE_TRANSPARENCY_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SUPPORTED_LINE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-supported-line-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SUPPORTED_LINE_CERT_CSV_REF: &str =
    "artifacts/release/m5-supported-line-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SUPPORTED_LINE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-supported-line-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const SUPPORTED_LINE_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-supported-line-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const SUPPORTED_LINE_CERT_PACKET_ID: &str =
    "m5-supported-line-surface-certification:stable:0001";

/// The five claimed M5 supported-line profiles this capstone certifies. Keyed on the profile
/// a release engineer, release operator, program-governance owner, or support engineer reads a
/// public-proof, transparency-report, migration-scoreboard, ORR-history, correction-archive, or
/// freshness-window surface through, not on the reusable proof object it renders. Only a live,
/// first-party supported-line operating lane profile may certify a certified operating line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyCertifiedProfile {
    /// A live, first-party supported-line operating lane — a registry-bound line whose current public-proof
    /// ledger, export-safe transparency report, versioned migration scoreboard, retained ORR history, and
    /// archived correction train converge on one current, joined, export-safe proof record, rendering the
    /// certified operating claim exactly right now.
    LiveSupportedLineOperatingLane,
    /// A reviewable transparency structure: a self-sufficient, inspectable supported-line proof projection (a
    /// public-proof ledger / transparency snapshot / archived history record an operator can review), never
    /// itself an authoritative, live-operating line.
    ReviewableTransparencyStructure,
    /// A correction-archive lane whose correction-train history can only be partially disclosed; the claim
    /// narrows to a correction-archive-disclosed projection that discloses the archived correction packet
    /// alongside its advisory / public-communication history and exact-build join, never a correction archive
    /// shown as fully retained while its coverage or build join is incomplete.
    DisclosedCorrectionArchiveProfile,
    /// A migration-scoreboard lane whose importer / bridge outcome scoring and migration-pain deltas have aged
    /// out; the claim narrows to a migration-scoreboard-unverified projection that keeps the last-known
    /// scoreboard posture explicit, never a stale scoreboard shown as current or migration pain left unscored.
    UnverifiedMigrationScoreboardProfile,
    /// An ORR-history lane whose retained ORR / go-no-go / cohort-transition decisions or archived history have
    /// aged out or become unreconstructable; the claim narrows to an ORR-history-unverified projection that keeps
    /// the last-known unretained-history posture explicit, never an ORR / go-no-go claim shown as backed by a
    /// current, retained decision history behind a green line.
    UnverifiedOrrHistoryProfile,
}

impl M5SupportedLineTransparencyCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5SupportedLineTransparencyCertifiedProfile; 5] = [
        M5SupportedLineTransparencyCertifiedProfile::LiveSupportedLineOperatingLane,
        M5SupportedLineTransparencyCertifiedProfile::ReviewableTransparencyStructure,
        M5SupportedLineTransparencyCertifiedProfile::DisclosedCorrectionArchiveProfile,
        M5SupportedLineTransparencyCertifiedProfile::UnverifiedMigrationScoreboardProfile,
        M5SupportedLineTransparencyCertifiedProfile::UnverifiedOrrHistoryProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSupportedLineOperatingLane => "live_supported_line_operating_lane",
            Self::ReviewableTransparencyStructure => "reviewable_transparency_structure",
            Self::DisclosedCorrectionArchiveProfile => "disclosed_correction_archive_profile",
            Self::UnverifiedMigrationScoreboardProfile => "unverified_migration_scoreboard_profile",
            Self::UnverifiedOrrHistoryProfile => "unverified_orr_history_profile",
        }
    }

    /// True only for the live, first-party supported-line operating lane profile. A certified operating line may
    /// be certified on this profile alone; every other profile is at most a reviewable transparency structure or
    /// a narrowed projection.
    pub const fn is_live_supported_line_operating_lane(self) -> bool {
        matches!(self, Self::LiveSupportedLineOperatingLane)
    }
}

/// The claim ladder a certified supported-line profile asserts and is certified down to. Minted locally
/// for this capstone (B147 folds accessibility into the cert): the strongest claim is a fully certified
/// operating line; each weaker tier is a disclosed projection that keeps the last-known correction-archive,
/// migration-scoreboard, or ORR-history posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportedLineTransparencyClaim {
    /// Certified operating line: a fully current, registry-bound supported line with a current public-proof
    /// ledger, an export-safe transparency report, a versioned migration scoreboard, retained ORR history, and
    /// an archived correction train all joined to exact build / release-line identity — the strongest claim, a
    /// supported-line surface Aureline can present as durable-proof-current right now.
    CertifiedOperatingLine,
    /// Reviewable transparency surface: a self-sufficient, inspectable read-only supported-line proof projection
    /// (a static public-proof ledger / transparency snapshot / archived history record an operator can inspect)
    /// that is not itself an authoritative, live-operating line.
    ReviewableTransparencySurface,
    /// Correction-archive-disclosed projection: a correction-archive lane's correction-train history can only be
    /// partially disclosed; the lane stays a correction-archive-disclosed projection that discloses the archived
    /// correction packet alongside its advisory / public-communication history and exact-build join, never a
    /// correction archive shown as fully retained while its coverage or build join is incomplete.
    CorrectionArchiveDisclosedProjection,
    /// Migration-scoreboard-unverified projection: a migration-scoreboard lane's importer / bridge scoring and
    /// migration-pain deltas have aged out; the lane stays a migration-scoreboard-unverified projection that keeps
    /// the last-known scoreboard posture explicit, never a stale scoreboard shown as current.
    MigrationScoreboardUnverifiedProjection,
    /// ORR-history-unverified projection: an ORR-history lane's retained ORR / go-no-go / cohort-transition
    /// decisions have aged out or become unreconstructable; the lane stays an ORR-history-unverified projection
    /// that keeps the last-known unretained-history posture explicit, never an ORR / go-no-go claim shown as
    /// backed by a current, retained decision history behind a green line.
    OrrHistoryUnverifiedProjection,
}

impl M5SupportedLineTransparencyClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::CertifiedOperatingLine,
        Self::ReviewableTransparencySurface,
        Self::CorrectionArchiveDisclosedProjection,
        Self::MigrationScoreboardUnverifiedProjection,
        Self::OrrHistoryUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedOperatingLine => 4,
            Self::ReviewableTransparencySurface => 3,
            Self::CorrectionArchiveDisclosedProjection => 2,
            Self::MigrationScoreboardUnverifiedProjection => 1,
            Self::OrrHistoryUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully certified, durable-proof-current supported line.
    pub const fn asserts_certified_operating_line(self) -> bool {
        matches!(self, Self::CertifiedOperatingLine)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedOperatingLine | Self::ReviewableTransparencySurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedOperatingLine => "certified_operating_line",
            Self::ReviewableTransparencySurface => "reviewable_transparency_surface",
            Self::CorrectionArchiveDisclosedProjection => "correction_archive_disclosed_projection",
            Self::MigrationScoreboardUnverifiedProjection => {
                "migration_scoreboard_unverified_projection"
            }
            Self::OrrHistoryUnverifiedProjection => "orr_history_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and supported-line-proof-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedLineTransparencyCertificationAxis {
    /// Visual parity: supported-line association, public-proof freshness, transparency snapshot,
    /// migration-scoreboard currency, ORR-history retention, correction-archive retention, freshness window,
    /// export class, and registry reference are shown on the primary surface without relying on a
    /// shell-chrome-only affordance or a mislabeled green release row alone.
    Visual,
    /// Keyboard-reach parity: the same supported-line proof truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled release row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// supported-line association, freshness window, migration-scoreboard state, ORR-history record, or registry
    /// reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the supported-line association, freshness window, or transparency snapshot.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// supported-line name, proof-object class, export class, or freshness window when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale public proof, an opaque upstream-health report, an unscored migration
    /// scoreboard, or a stale ORR / correction record honestly downgrades a `CertifiedOperatingLine` /
    /// `ReviewableTransparencySurface` claim rather than reading as a fresh, fully certified operating line.
    DegradedState,
    /// Supported-line-proof-truth parity: public-proof freshness, transparency snapshot, migration-scoreboard
    /// currency, ORR-history retention, correction-archive retention, freshness window, export class, and
    /// supported-line association stay explicit and never let a line widen a claim because a report once existed
    /// without current freshness, stay green on stale external proof or opaque upstream health, leak internal-only
    /// incident or security detail into a public-safe feed, leave public-proof / migration / history unjoined to
    /// build and release-line identity, or leave migration pain / ORR / correction history unretained.
    SupportedLineProofTruth,
}

impl SupportedLineTransparencyCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [SupportedLineTransparencyCertificationAxis; 9] = [
        SupportedLineTransparencyCertificationAxis::Visual,
        SupportedLineTransparencyCertificationAxis::Keyboard,
        SupportedLineTransparencyCertificationAxis::ScreenReader,
        SupportedLineTransparencyCertificationAxis::HighZoomReflow,
        SupportedLineTransparencyCertificationAxis::HighContrast,
        SupportedLineTransparencyCertificationAxis::Localization,
        SupportedLineTransparencyCertificationAxis::CliExport,
        SupportedLineTransparencyCertificationAxis::DegradedState,
        SupportedLineTransparencyCertificationAxis::SupportedLineProofTruth,
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
            Self::SupportedLineProofTruth => "supported_line_proof_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedLineTransparencyAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl SupportedLineTransparencyAxisCertificationState {
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
pub enum SupportedLineTransparencyProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a certified operating line, or the narrowing is inconsistent.
    Red,
}

impl SupportedLineTransparencyProfileClaimStatus {
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

/// The five B147 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedLineTransparencyCertGuardrails {
    /// True if the profile widens a claim because a report once existed without current freshness. Must be false.
    pub widens_a_claim_because_a_report_once_existed_without_current_freshness: bool,
    /// True if the profile stays green on stale external proof or opaque upstream health. Must be false.
    pub stays_green_on_stale_external_proof_or_opaque_upstream_health: bool,
    /// True if the profile leaks internal-only incident or security detail into a public-safe feed. Must be false.
    pub leaks_internal_only_incident_or_security_detail_into_public_safe_feeds: bool,
    /// True if the profile leaves public-proof / migration / history unjoined to build and release-line identity.
    /// Must be false.
    pub leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity: bool,
    /// True if the profile leaves migration pain / ORR / correction history unretained. Must be false.
    pub leaves_migration_pain_or_orr_and_correction_history_unretained: bool,
}

impl SupportedLineTransparencyCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        widens_a_claim_because_a_report_once_existed_without_current_freshness: false,
        stays_green_on_stale_external_proof_or_opaque_upstream_health: false,
        leaks_internal_only_incident_or_security_detail_into_public_safe_feeds: false,
        leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity: false,
        leaves_migration_pain_or_orr_and_correction_history_unretained: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.widens_a_claim_because_a_report_once_existed_without_current_freshness
            && !self.stays_green_on_stale_external_proof_or_opaque_upstream_health
            && !self.leaks_internal_only_incident_or_security_detail_into_public_safe_feeds
            && !self.leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity
            && !self.leaves_migration_pain_or_orr_and_correction_history_unretained
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedLineTransparencyCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The supported-line-association / public-proof-freshness / transparency-snapshot /
    /// migration-scoreboard / ORR-history / correction-archive / freshness-window / registry-reference
    /// fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl SupportedLineTransparencyCertExportParity {
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
pub struct SupportedLineTransparencyAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: SupportedLineTransparencyCertificationAxis,
    /// The certification state of the axis.
    pub state: SupportedLineTransparencyAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5SupportedLineTransparencyDowngradeTrigger>,
}

impl SupportedLineTransparencyAxisOutcome {
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
            SupportedLineTransparencyAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            SupportedLineTransparencyAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            SupportedLineTransparencyAxisCertificationState::UndisclosedDrift => {
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
pub struct SupportedLineTransparencyClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: SupportedLineTransparencyCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5SupportedLineTransparencyClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5SupportedLineTransparencyClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 supported-line proof-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedLineTransparencyProfileCertificationRow {
    /// Record kind; must equal [`SUPPORTED_LINE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SUPPORTED_LINE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5SupportedLineTransparencyCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5SupportedLineTransparencyClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5SupportedLineTransparencyClaim,
    /// The frozen proof objects this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5SupportedLineTransparencyObject>,
    /// One outcome per [`SupportedLineTransparencyCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<SupportedLineTransparencyAxisOutcome>,
    /// The B147 hard invariants; all must hold.
    pub guardrails: SupportedLineTransparencyCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<SupportedLineTransparencyClaimAutoNarrow>,
    /// The one canonical supported-line-transparency proof bundle this profile cites. Must equal
    /// [`SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: SupportedLineTransparencyProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: SupportedLineTransparencyCertExportParity,
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

impl SupportedLineTransparencyProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: SupportedLineTransparencyCertificationAxis,
    ) -> Option<&SupportedLineTransparencyAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<SupportedLineTransparencyCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && SupportedLineTransparencyCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(SupportedLineTransparencyAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<SupportedLineTransparencyCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| {
                o.state == SupportedLineTransparencyAxisCertificationState::DisclosedNarrowed
            })
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a certified operating line, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> SupportedLineTransparencyProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        // Every B147 hard invariant must hold.
        if !self.guardrails.all_held() {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a certified operating line.
        if self.certified_claim.asserts_certified_operating_line()
            && !self.profile.is_live_supported_line_operating_lane()
        {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(SupportedLineTransparencyCertificationAxis::CliExport) {
            Some(o) if o.state == SupportedLineTransparencyAxisCertificationState::Certified => {}
            _ => return SupportedLineTransparencyProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == SupportedLineTransparencyAxisCertificationState::UndisclosedDrift)
        {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return SupportedLineTransparencyProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return SupportedLineTransparencyProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return SupportedLineTransparencyProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return SupportedLineTransparencyProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return SupportedLineTransparencyProfileClaimStatus::Red;
        }

        SupportedLineTransparencyProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SUPPORTED_LINE_CERT_ROW_RECORD_KIND
            && self.schema_version == SUPPORTED_LINE_CERT_SCHEMA_VERSION
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
pub struct SupportedLineTransparencyProfileCertificationSummary {
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

/// Constructor input for [`SupportedLineTransparencyProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedLineTransparencyProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<SupportedLineTransparencyProfileCertificationRow>,
}

/// Checked-in M05-1228 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedLineTransparencyProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<SupportedLineTransparencyProfileCertificationRow>,
    pub summary: SupportedLineTransparencyProfileCertificationSummary,
}

impl SupportedLineTransparencyProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: SupportedLineTransparencyProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SUPPORTED_LINE_CERT_SCHEMA_VERSION,
            record_kind: SUPPORTED_LINE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: SupportedLineTransparencyProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5SupportedLineTransparencyCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Proof objects rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5SupportedLineTransparencyObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5SupportedLineTransparencyCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen line is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5SupportedLineTransparencyObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(SupportedLineTransparencyCertificationAxis::CliExport)
                .is_some_and(|o| {
                    o.state == SupportedLineTransparencyAxisCertificationState::Certified
                })
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> SupportedLineTransparencyProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportedLineTransparencyProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportedLineTransparencyProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportedLineTransparencyProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(SupportedLineTransparencyProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        SupportedLineTransparencyProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(SupportedLineTransparencyProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<SupportedLineTransparencyCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SUPPORTED_LINE_CERT_SCHEMA_VERSION {
            violations.push(
                SupportedLineTransparencyCertificationViolation::SchemaVersion {
                    expected: SUPPORTED_LINE_CERT_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != SUPPORTED_LINE_CERT_RECORD_KIND {
            violations.push(
                SupportedLineTransparencyCertificationViolation::RecordKind {
                    expected: SUPPORTED_LINE_CERT_RECORD_KIND.to_owned(),
                    actual: self.record_kind.clone(),
                },
            );
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(SupportedLineTransparencyCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(SupportedLineTransparencyCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::DuplicateId {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.is_complete() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::IncompleteRow {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.covers_all_axes() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B147 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a live first-party profile may certify a certified operating line.
            if row.certified_claim.asserts_certified_operating_line()
                && !row.profile.is_live_supported_line_operating_lane()
            {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::NonLiveProfileClaimsTrustedLane {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(SupportedLineTransparencyCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == SupportedLineTransparencyProfileClaimStatus::Red {
                violations.push(
                    SupportedLineTransparencyCertificationViolation::ProfileBlocked {
                        id: row.row_id.clone(),
                    },
                );
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations
                .push(SupportedLineTransparencyCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen line must be certified on some profile.
        if !self.all_families_covered() {
            violations
                .push(SupportedLineTransparencyCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(SupportedLineTransparencyCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                SupportedLineTransparencyCertificationViolation::RawSupportedLineMaterialInExport,
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
        out.push_str("# M5 Supported-Line Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5SupportedLineTransparencyCertifiedProfile::ALL.len(),
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
pub fn current_m5_supported_line_surface_certification_export() -> Result<
    SupportedLineTransparencyProfileCertificationPacket,
    SupportedLineTransparencyCertificationArtifactError,
> {
    let packet: SupportedLineTransparencyProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-supported-line-surface-certification/support_export.json"
        )))
        .map_err(SupportedLineTransparencyCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SupportedLineTransparencyCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum SupportedLineTransparencyCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SupportedLineTransparencyCertificationViolation>),
}

impl fmt::Display for SupportedLineTransparencyCertificationArtifactError {
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

impl Error for SupportedLineTransparencyCertificationArtifactError {}

/// Validation failure for M05-1228 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportedLineTransparencyCertificationViolation {
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
    RawSupportedLineMaterialInExport,
}

impl fmt::Display for SupportedLineTransparencyCertificationViolation {
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
                    "packet does not cite the canonical supported-line-transparency proof bundle"
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
                    "row {id} does not cite the one canonical supported-line-transparency proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B147 hard invariant: widening a claim because a report once existed without \
current freshness; staying green on stale external proof or opaque upstream health; leaking internal-only \
incident or security detail into a public-safe feed; leaving public-proof / migration / history unjoined to \
build and release-line identity; or leaving migration pain / ORR / correction history unretained"
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
            Self::RawSupportedLineMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for SupportedLineTransparencyCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&SupportedLineTransparencyAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != SupportedLineTransparencyAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the supported-line
/// generics the spec forbids collapsing distinct public-proof, transparency-report, migration-scoreboard,
/// ORR-history, correction-archive, freshness-window, and export-class truth into (whole-label matches so a
/// full sentence naming a concrete supported line, proof object, or registry reference is not flagged).
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
            | "public proof"
            | "public-proof"
            | "public proof ledger"
            | "proof"
            | "proof ledger"
            | "freshness"
            | "freshness window"
            | "transparency"
            | "transparency report"
            | "transparency snapshot"
            | "upstream health"
            | "migration"
            | "migration scoreboard"
            | "scoreboard"
            | "migration pain"
            | "scoreboard delta"
            | "orr"
            | "orr history"
            | "go/no-go"
            | "go no go"
            | "cohort transition"
            | "correction"
            | "correction archive"
            | "correction train"
            | "advisory"
            | "revocation"
            | "export class"
            | "export-class"
            | "export safe"
            | "internal only"
            | "snapshot"
            | "evidence"
            | "release evidence"
            | "claim history"
            | "known limits"
            | "registry reference"
            | "line association"
            | "build identity"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the supported-line
/// transparency matrix heuristic so the reused [`M5SupportedLineTransparencyDowngradeTrigger`] narrowings
/// serialize cleanly — the supported-line proof grammar carries only typed class tokens and opaque refs,
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

/// Builds the canonical, checked-in M05-1228 certification packet. Certifies all five claimed M5
/// supported-line profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_supported_line_surface_certification_packet(
) -> SupportedLineTransparencyProfileCertificationPacket {
    SupportedLineTransparencyProfileCertificationPacket::new(
        SupportedLineTransparencyProfileCertificationPacketInput {
            packet_id: SUPPORTED_LINE_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-15T00:00:00Z".to_owned(),
            matrix_ref: SUPPORTED_LINE_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:supported-line-surface-certification:{id}"),
        SUPPORTED_LINE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> SupportedLineTransparencyCertExportParity {
    SupportedLineTransparencyCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: SupportedLineTransparencyCertificationAxis) -> &'static str {
    match axis {
        SupportedLineTransparencyCertificationAxis::Visual => {
            "supported-line association, public-proof freshness, transparency snapshot, migration-scoreboard currency, ORR-history retention, correction-archive retention, freshness window, export class, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled green release row alone"
        }
        SupportedLineTransparencyCertificationAxis::Keyboard => {
            "the same supported-line proof role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        SupportedLineTransparencyCertificationAxis::ScreenReader => {
            "the same supported-line proof truth is announced non-visually, never a shell-chrome-only / mislabeled-release-row / unlabeled-control-only cue"
        }
        SupportedLineTransparencyCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the supported-line association, freshness window, migration-scoreboard state, ORR-history record, or registry reference"
        }
        SupportedLineTransparencyCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the supported-line association, freshness window, or transparency snapshot"
        }
        SupportedLineTransparencyCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a supported-line name, proof-object class, export class, or freshness window"
        }
        SupportedLineTransparencyCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        SupportedLineTransparencyCertificationAxis::DegradedState => {
            "a stale public proof, an opaque upstream-health report, an unscored migration scoreboard, or a stale ORR or correction record honestly downgrades the CertifiedOperatingLine/ReviewableTransparencySurface claim rather than reading as a fresh, fully certified operating line"
        }
        SupportedLineTransparencyCertificationAxis::SupportedLineProofTruth => {
            "public-proof freshness, transparency snapshot, migration-scoreboard currency, ORR-history retention, correction-archive retention, freshness window, export class, and supported-line association stay explicit and never let a line widen a claim because a report once existed without current freshness, stay green on stale external proof or opaque upstream health, leak internal-only incident or security detail into a public-safe feed, leave public-proof / migration / history unjoined to build and release-line identity, or leave migration pain / ORR / correction history unretained"
        }
    }
}

fn seed_certified(
    axis: SupportedLineTransparencyCertificationAxis,
) -> SupportedLineTransparencyAxisOutcome {
    SupportedLineTransparencyAxisOutcome {
        axis,
        state: SupportedLineTransparencyAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: SupportedLineTransparencyCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5SupportedLineTransparencyDowngradeTrigger,
) -> SupportedLineTransparencyAxisOutcome {
    SupportedLineTransparencyAxisOutcome {
        axis,
        state: SupportedLineTransparencyAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<SupportedLineTransparencyAxisOutcome> {
    SupportedLineTransparencyCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: SupportedLineTransparencyCertificationAxis,
    outcome: SupportedLineTransparencyAxisOutcome,
) -> Vec<SupportedLineTransparencyAxisOutcome> {
    SupportedLineTransparencyCertificationAxis::ALL
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
    profile: M5SupportedLineTransparencyCertifiedProfile,
    claimed_claim: M5SupportedLineTransparencyClaim,
    certified_claim: M5SupportedLineTransparencyClaim,
    consumed_families: &[M5SupportedLineTransparencyObject],
    axis_outcomes: Vec<SupportedLineTransparencyAxisOutcome>,
    claim_auto_narrow: Option<SupportedLineTransparencyClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> SupportedLineTransparencyProfileCertificationRow {
    let mut row = SupportedLineTransparencyProfileCertificationRow {
        record_kind: SUPPORTED_LINE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: SUPPORTED_LINE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: SupportedLineTransparencyCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: SUPPORTED_LINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: SupportedLineTransparencyProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            SUPPORTED_LINE_CERT_MATRIX_REF.to_owned(),
            SUPPORTED_LINE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: SupportedLineTransparencyCertificationAxis,
    from_claim: M5SupportedLineTransparencyClaim,
    to_claim: M5SupportedLineTransparencyClaim,
    label: &str,
) -> SupportedLineTransparencyClaimAutoNarrow {
    SupportedLineTransparencyClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<SupportedLineTransparencyProfileCertificationRow> {
    use M5SupportedLineTransparencyCertifiedProfile as P;
    use M5SupportedLineTransparencyClaim::*;
    use M5SupportedLineTransparencyDowngradeTrigger as Trig;
    use M5SupportedLineTransparencyObject::*;
    use SupportedLineTransparencyCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-supported-line-operating-lane",
            P::LiveSupportedLineOperatingLane,
            CertifiedOperatingLine,
            CertifiedOperatingLine,
            &[PublicProofLedger],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "public_proof_freshness",
            ],
            &[
                "public-proof-ledger line: a current public-proof ledger, an export-safe transparency report, a versioned migration scoreboard, retained ORR history, and an archived correction train all join to exact build / release-line identity within their freshness window, never a claim widened because a report once existed without current freshness",
                "the certified operating line keeps stable operation IDs while the supported-line association, freshness window, transparency snapshot, and registry reference bind to the one supported-line-transparency registry across release / help / docs / support / public-proof / partner surfaces",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered operating line",
                "supported-line-proof-truth: a live, first-party supported-line operating lane with current, export-safe, and internally consistent durable proof is the only profile that certifies a certified operating line",
            ],
        ),
        seed_row(
            "cert:reviewable-transparency-structure",
            P::ReviewableTransparencyStructure,
            ReviewableTransparencySurface,
            ReviewableTransparencySurface,
            &[TransparencyReport],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "line_association",
            ],
            &[
                "transparency-report line: an export-safe upstream-health / compatibility-health / maintainer-durability report bound to one supported-line association and inspectable before widening rather than a per-surface description copied by hand, with public-safe health separated from internal-only incident detail",
                "the reviewable transparency structure keeps its supported-line association, freshness window, transparency snapshot, and registry labels inspectable rather than a shell-chrome-only or mislabeled-release-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable transparency structure",
                "supported-line-proof-truth: a reviewable transparency structure never certifies a live operating claim and never stays green on stale external proof or opaque upstream health",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-correction-archive-profile",
            P::DisclosedCorrectionArchiveProfile,
            ReviewableTransparencySurface,
            CorrectionArchiveDisclosedProjection,
            &[CorrectionTrainArchive],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the correction-archive lane carries a correction train whose archive coverage can only be partially disclosed for this profile so a fully retained, build-joined archive cannot be certified",
                    "The correction-archive lane carries an archived correction train whose advisory / public-communication history and exact-build join can only be partially disclosed, so the ReviewableTransparencySurface claim narrows to a correction-archive-disclosed projection and the lane discloses the archived correction packet alongside its build join rather than presenting it as fully retained or letting a public-safe advisory read as complete",
                    Trig::ImpliedGreenWhileProofOrArchiveWasStale,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableTransparencySurface,
                CorrectionArchiveDisclosedProjection,
                "Correction archive disclosed partial: the correction-train archive coverage is only partially retained so it is disclosed alongside its advisory history and exact-build join and never reads as fully retained",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "correction-train-archive line: the archive names its hotfix / backport / advisory / public-communication history and exact-build join and marks coverage as disclosed-partial rather than letting an archived correction train read as fully retained when its coverage is incomplete",
                "the correction-archive surface keeps its advisory / public-communication history and exact-build join legible while archive coverage is disclosed as partial",
                "localization: ReviewableTransparencySurface narrows to a correction-archive-disclosed projection (auto-narrowed)",
                "supported-line-proof-truth: a partially-retained correction archive never reads as fully retained — the advisory history and exact-build join are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-migration-scoreboard-profile",
            P::UnverifiedMigrationScoreboardProfile,
            ReviewableTransparencySurface,
            MigrationScoreboardUnverifiedProjection,
            &[MigrationScoreboard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the importer / bridge outcome scoring and migration-pain deltas have aged out so a fully current migration scoreboard cannot be certified",
                    "The importer / bridge outcome scoring and migration-pain deltas have aged out, so the ReviewableTransparencySurface claim narrows to a migration-scoreboard-unverified projection and the lane keeps the last-known scoreboard posture explicit rather than staying green on a stale scoreboard or leaving migration pain unscored",
                    Trig::LeftMigrationPainUnscored,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableTransparencySurface,
                MigrationScoreboardUnverifiedProjection,
                "Migration scoreboard unverified: the importer / bridge outcome scoring and migration-pain deltas have aged out so the last-known scoreboard posture stays explicit and no stale scoreboard reads as current",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "migration-scoreboard line: the scoreboard keeps its per-outcome-class scoring and migration-pain deltas explicit and marks the scoreboard as unverified rather than staying green on a stale scoreboard when the scoring has aged out, and never leaves migration pain unscored",
                "the migration-scoreboard surface keeps its per-outcome-class scoring and last-published lineage legible while the scoreboard currency is disclosed as unverified",
                "degraded-state: ReviewableTransparencySurface narrows to a migration-scoreboard-unverified projection (auto-narrowed)",
                "supported-line-proof-truth: a migration scoreboard never reads as current when its scoring has aged out and never lets a stale scoreboard imply a fresh claim",
            ],
        ),
        seed_row(
            "cert:unverified-orr-history-profile",
            P::UnverifiedOrrHistoryProfile,
            ReviewableTransparencySurface,
            OrrHistoryUnverifiedProjection,
            &[OrrHistoryEvent],
            seed_certified_except(
                Ax::SupportedLineProofTruth,
                seed_narrowed(
                    Ax::SupportedLineProofTruth,
                    "a retained ORR / go-no-go / cohort-transition decision is missing or the archived line history has become unreconstructable so a current, retained ORR history cannot be certified",
                    "A retained ORR / go-no-go / cohort-transition decision is missing or the archived line history has become unreconstructable, so the ReviewableTransparencySurface claim narrows to an ORR-history-unverified projection and the lane keeps the last-known unretained-history posture explicit rather than presenting an ORR / go-no-go claim as backed by a current, retained decision history behind a green line",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::SupportedLineProofTruth,
                ReviewableTransparencySurface,
                OrrHistoryUnverifiedProjection,
                "ORR history unverified: a retained ORR / go-no-go decision is missing so the last-known unretained-history posture stays explicit and no ORR / go-no-go claim reads as backed by a current, retained decision history",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "ORR-history line: the history keeps its ORR / go-no-go / cohort-transition lineage explicit and marks the retention as unverified rather than leaving ORR / correction history unretained behind a green line",
                "the ORR-history surface keeps its ORR / go-no-go / cohort-transition lineage legible while the history retention is disclosed as unverified",
                "supported-line-proof-truth: ReviewableTransparencySurface narrows to an ORR-history-unverified projection (auto-narrowed)",
                "supported-line-proof-truth: an ORR / go-no-go claim cites its retained decision history and never leaves ORR or correction history unretained, and no claim outpaces the retained history",
            ],
        ),
    ]
}
