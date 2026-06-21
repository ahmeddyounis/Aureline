//! Frozen M5 presentation-session / walkthrough-waypoint / speaker-note /
//! audience-follow qualification matrix for every claimed presentation surface.
//!
//! Presentation and walkthrough stay a thin, reversible layer over existing
//! Aureline panes: file, symbol, branch, workspace, and local/remote/shared
//! boundary labels stay visible; speaker notes default local/private; follow,
//! break away, request follow, and take over stay distinct states; teaching and
//! classroom roles stay separate from edit/debug/approval authority; entering
//! presentation checkpoints the prior layout; and exiting restores it without
//! hidden reruns or widened rights. The canonical object model already lives in
//! [`crate::presentation_mode`] (the [`PresentationSession`], [`FollowWaypoint`],
//! and [`SpeakerNote`] boundary objects) and [`crate::teaching_session`]
//! (classroom roles). This module turns the remaining implicit promise — that
//! every *claimed* presentation surface rests on verified speaker-note privacy,
//! audience-follow/breakaway truth, layout restore, authority separation, and
//! accessibility evidence — into one machine-readable, verification-bound matrix.
//!
//! * a [`PresentationClaimedSurfaceRow`] ties a durable claimed surface (keyed by
//!   a [`PresentationSurfaceKind`], a [`PresentationOriginClass`], and a
//!   non-display fingerprint) to the canonical [`PresentationSession`] it drives,
//!   plus the five qualification axes the exit gate names —
//!   [`SpeakerNotePrivacyPosture`], [`AudienceFollowTruth`],
//!   [`AuthoritySeparation`], [`LayoutRestoreEvidence`], and
//!   [`PresentationAccessibilityPosture`] — a [`PresentationVerification`] proof,
//!   and a claimed and effective [`PresentationQualificationGrade`];
//! * each row is **verification-bound, not asserted**: its
//!   [`PresentationVerification`] names a [`PresentationProofCurrency`] and,
//!   unless the proof is missing, a reopenable proof ref keyed by a non-display
//!   fingerprint, so restore, diagnostics, help, support-export, and release
//!   surfaces can reopen the same evidence object that backs the claim instead of
//!   cloning presentation-state text by hand;
//! * the row **auto-downgrades**:
//!   [`PresentationClaimedSurfaceRow::needs_downgrade`] is true whenever a
//!   *claimed* surface leaves speaker-note privacy, follow/breakaway truth,
//!   layout restore, authority separation, or accessibility evidence unverified,
//!   erases source provenance, goes unavailable, or carries stale, missing, or
//!   imported-on-local proof. A downgraded row must carry an effective grade
//!   strictly below its claim, a recorded
//!   [`PresentationDowngradeTrigger`], and a precise label — never a generic
//!   non-answer. Unclaimed (Labs/unadvertised) surfaces make no claim to
//!   downgrade from and stay clearly separate from claimed scope.
//!
//! [`M5PresentationQualificationMatrixPacket::validate`] additionally refuses any
//! packet that lets a presentation surface leak a raw speaker-note body, widen
//! mutation or control authority, drop the keyboard path, or strand the user in
//! an improvised shell instead of restoring the checkpointed layout.
//!
//! Raw speaker-note bodies, private file contents, and credentials never cross
//! this boundary; the packet carries only typed class tokens, booleans, stable
//! refs, opaque ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schemas are
//! [`schemas/presentation/presentation-session.schema.json`](../../../../schemas/presentation/presentation-session.schema.json),
//! [`schemas/presentation/follow-waypoint.schema.json`](../../../../schemas/presentation/follow-waypoint.schema.json),
//! and
//! [`schemas/presentation/speaker-note.schema.json`](../../../../schemas/presentation/speaker-note.schema.json).
//! The qualification matrix is published at
//! [`artifacts/presentation/m5-presentation-qualification-matrix.md`](../../../../artifacts/presentation/m5-presentation-qualification-matrix.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::presentation_mode::{
    restore_from_checkpoint, AudienceParticipant, AudienceScope, BoundaryLabel, FollowWaypoint,
    LayoutPreset, LeaderFollowState, ParticipantFollowState, ParticipantRole, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, RestoreOutcome, RestoreTrigger,
    SessionLifecycleState, SpeakerNote, SpeakerNoteScope, WalkthroughSurfaceKind,
    WaypointCompletionState,
};
pub use crate::teaching_session::TeachingRole;

/// Stable record-kind tag carried by [`M5PresentationQualificationMatrixPacket`].
pub const PRESENTATION_QUALIFICATION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_presentation_session_walkthrough_waypoint_speaker_note_audience_follow_matrix_packet";

/// Schema version shared by the matrix packet and its component objects.
pub const PRESENTATION_QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the canonical presentation-session boundary schema.
pub const PRESENTATION_SESSION_SCHEMA_REF: &str =
    "schemas/presentation/presentation-session.schema.json";

/// Repo-relative path of the canonical follow-waypoint boundary schema.
pub const FOLLOW_WAYPOINT_SCHEMA_REF: &str = "schemas/presentation/follow-waypoint.schema.json";

/// Repo-relative path of the canonical speaker-note boundary schema.
pub const SPEAKER_NOTE_SCHEMA_REF: &str = "schemas/presentation/speaker-note.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const PRESENTATION_QUALIFICATION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/presentation/m5-presentation-qualification-matrix/support_export.json";

/// Repo-relative path of the published Markdown qualification matrix.
pub const PRESENTATION_QUALIFICATION_MATRIX_SUMMARY_REF: &str =
    "artifacts/presentation/m5-presentation-qualification-matrix.md";

/// Repo-relative path of the human-readable truth doc.
pub const PRESENTATION_AND_WALKTHROUGH_TRUTH_DOC_REF: &str =
    "docs/ux/presentation-and-walkthrough-truth.md";

/// Repo-relative path of the cross-surface learning/presentation contract this
/// lane extends.
pub const LEARNING_AND_PRESENTATION_CONTRACT_REF: &str =
    "docs/ux/learning_and_presentation_contract.md";

/// Repo-relative path of the restore/presentation recovery contract this lane
/// builds on.
pub const COLLAB_RESTORE_AND_PRESENTATION_CONTRACT_REF: &str =
    "docs/recovery/collab_restore_and_presentation_contract.md";

/// Kind of claimed presentation surface a row covers. Mirrors the design's
/// presentation taxonomy so restore, help, accessibility, and release surfaces
/// read one set of surface ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationSurfaceKind {
    /// Presenter walkthrough overlay over an existing editor/diff/docs/graph/
    /// notebook surface, driven by an agenda/waypoint rail.
    PresenterWalkthrough,
    /// Audience follow / breakaway / request-follow / take-over surface.
    AudienceFollow,
    /// Speaker-note tray, local/private by default.
    SpeakerNotes,
    /// Teaching / classroom-role surface separated from edit/debug authority.
    ClassroomTeaching,
    /// Layout checkpoint-on-enter / restore-on-exit surface.
    LayoutRestore,
    /// Unavailable / degraded surface that always keeps a keyboard-first path.
    UnavailableFallback,
}

impl PresentationSurfaceKind {
    /// Every claimed presentation surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PresenterWalkthrough,
        Self::AudienceFollow,
        Self::SpeakerNotes,
        Self::ClassroomTeaching,
        Self::LayoutRestore,
        Self::UnavailableFallback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterWalkthrough => "presenter_walkthrough",
            Self::AudienceFollow => "audience_follow",
            Self::SpeakerNotes => "speaker_notes",
            Self::ClassroomTeaching => "classroom_teaching",
            Self::LayoutRestore => "layout_restore",
            Self::UnavailableFallback => "unavailable_fallback",
        }
    }
}

/// Origin of a claimed presentation surface. A shared-session-linked or imported
/// surface must never read as a first-party, locally verified one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationOriginClass {
    /// A first-party, locally verified presentation surface.
    FirstPartyLocalSurface,
    /// An enterprise-managed surface governed by org policy.
    EnterpriseManagedSurface,
    /// A shared-session-linked surface whose qualification is session-backed.
    SharedSessionLinkedSurface,
    /// An imported, read-only surface record.
    ImportedReadOnlySurface,
}

impl PresentationOriginClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyLocalSurface => "first_party_local_surface",
            Self::EnterpriseManagedSurface => "enterprise_managed_surface",
            Self::SharedSessionLinkedSurface => "shared_session_linked_surface",
            Self::ImportedReadOnlySurface => "imported_read_only_surface",
        }
    }

    /// Whether qualification for this origin is session-backed / imported rather
    /// than locally verified, so a current claim rests on imported proof.
    pub const fn is_shared_or_imported(self) -> bool {
        matches!(
            self,
            Self::SharedSessionLinkedSurface | Self::ImportedReadOnlySurface
        )
    }
}

/// Whether Aureline publicly claims a row's surface, or keeps it Labs/unadvertised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationClaimPosture {
    /// Aureline claims this row as a beta presentation surface.
    ClaimedBeta,
    /// Aureline claims this row as a preview presentation surface.
    ClaimedPreview,
    /// Unclaimed: the surface stays Labs/unadvertised, suppressed by default.
    LabsUnadvertised,
}

impl PresentationClaimPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedBeta => "claimed_beta",
            Self::ClaimedPreview => "claimed_preview",
            Self::LabsUnadvertised => "labs_unadvertised",
        }
    }

    /// Whether this posture carries a public beta/preview claim.
    pub const fn is_claimed(self) -> bool {
        matches!(self, Self::ClaimedBeta | Self::ClaimedPreview)
    }
}

/// Qualification grade a claimed presentation surface holds. Higher [`Self::rank`]
/// is a stronger claim, so a downgraded row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationQualificationGrade {
    /// A fully qualified claimed surface (every axis verified and current).
    QualifiedClaimedSurface,
    /// A qualified but deliberately narrowed surface (e.g. shared/preview-class).
    QualifiedNarrowedSurface,
    /// An unclaimed Labs/unadvertised surface, kept out of public scope.
    LabsUnadvertisedSurface,
    /// A surface whose qualification was withdrawn.
    QualificationWithdrawn,
    /// Qualification does not apply to this row.
    NotApplicable,
}

impl PresentationQualificationGrade {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedClaimedSurface => "qualified_claimed_surface",
            Self::QualifiedNarrowedSurface => "qualified_narrowed_surface",
            Self::LabsUnadvertisedSurface => "labs_unadvertised_surface",
            Self::QualificationWithdrawn => "qualification_withdrawn",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this grade carries a public, qualified claim.
    pub const fn is_qualified_claim(self) -> bool {
        matches!(
            self,
            Self::QualifiedClaimedSurface | Self::QualifiedNarrowedSurface
        )
    }

    /// Ordinal rank; higher is a stronger claim, so a downgrade must move strictly
    /// lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::QualificationWithdrawn => 1,
            Self::LabsUnadvertisedSurface => 2,
            Self::QualifiedNarrowedSurface => 3,
            Self::QualifiedClaimedSurface => 4,
        }
    }
}

/// Currency of the proof backing a row's verification. Only a current, reopenable
/// proof backs a claim; a stale, missing, review-pending, or imported-on-local
/// proof auto-downgrades the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationProofCurrency {
    /// A fresh local proof verified inside its freshness window.
    VerifiedCurrent,
    /// A cached local proof still inside its freshness window.
    CachedWithinWindow,
    /// A current proof imported / session-backed and read-only locally.
    ImportedCurrent,
    /// A proof that exists but has aged outside its freshness window.
    StaleExpired,
    /// A proof that still requires review and fails closed.
    RequiresReview,
    /// No proof object exists for this row.
    MissingProof,
}

impl PresentationProofCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::ImportedCurrent => "imported_current",
            Self::StaleExpired => "stale_expired",
            Self::RequiresReview => "requires_review",
            Self::MissingProof => "missing_proof",
        }
    }

    /// Whether this is a current, locally verified or cached proof.
    pub const fn is_current_local(self) -> bool {
        matches!(self, Self::VerifiedCurrent | Self::CachedWithinWindow)
    }

    /// Whether this is a current imported / session-backed proof.
    pub const fn is_imported_current(self) -> bool {
        matches!(self, Self::ImportedCurrent)
    }

    /// Whether this currency carries no proof object.
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::MissingProof)
    }
}

/// Reason a claimed surface auto-downgraded below its claim. The chrome quotes the
/// trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationDowngradeTrigger {
    /// Speaker-note privacy (local-default, explicit promotion, body redaction)
    /// could not be kept verified.
    SpeakerNotePrivacyUnverified,
    /// Follow / breakaway / request-follow / take-over truth was not verified.
    FollowBreakawayTruthUnverified,
    /// Layout checkpoint-on-enter / restore-on-exit could not be proven.
    LayoutRestoreUnverified,
    /// Teaching/classroom-role separation from edit/debug/approval authority
    /// could not be kept verified.
    AuthoritySeparationUnverified,
    /// Accessibility (keyboard-first, announcement, reduced-motion, provenance)
    /// evidence was stale or failing.
    AccessibilityEvidenceStale,
    /// Source provenance (file/symbol/branch/workspace + boundary) was erased.
    SourceProvenanceErased,
    /// The presentation surface went unavailable / degraded.
    SurfaceUnavailableDowngraded,
    /// The verification proof aged outside its freshness window.
    StaleVerificationProof,
    /// Imported / session proof stood in for a local-surface claim.
    ImportedProofOnLocalSurface,
}

impl PresentationDowngradeTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpeakerNotePrivacyUnverified => "speaker_note_privacy_unverified",
            Self::FollowBreakawayTruthUnverified => "follow_breakaway_truth_unverified",
            Self::LayoutRestoreUnverified => "layout_restore_unverified",
            Self::AuthoritySeparationUnverified => "authority_separation_unverified",
            Self::AccessibilityEvidenceStale => "accessibility_evidence_stale",
            Self::SourceProvenanceErased => "source_provenance_erased",
            Self::SurfaceUnavailableDowngraded => "surface_unavailable_downgraded",
            Self::StaleVerificationProof => "stale_verification_proof",
            Self::ImportedProofOnLocalSurface => "imported_proof_on_local_surface",
        }
    }
}

/// Speaker-note privacy posture. Notes default local/private; sharing is an
/// explicit, separately recorded promotion; raw note bodies never enter exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNotePrivacyPosture {
    /// Speaker notes default to a local-only scope (must be true).
    pub notes_default_local_only: bool,
    /// A note becomes shared only through an explicit promotion (must be true).
    pub shared_notes_require_explicit_promotion: bool,
    /// Raw note bodies are excluded from support/diagnostics/telemetry export
    /// (absolute; must be true).
    pub note_bodies_excluded_from_export: bool,
    /// A shared note is redacted before it leaves the local machine.
    pub redaction_before_share: bool,
}

impl SpeakerNotePrivacyPosture {
    /// Whether raw note bodies stay out of every export (the absolute invariant).
    pub const fn export_safe(&self) -> bool {
        self.note_bodies_excluded_from_export
    }

    /// Whether the full privacy posture holds for a claimed surface.
    pub const fn privacy_holds(&self) -> bool {
        self.notes_default_local_only
            && self.shared_notes_require_explicit_promotion
            && self.note_bodies_excluded_from_export
            && self.redaction_before_share
    }
}

/// Audience-follow truth. Follow, break away, request follow, and take over are
/// distinct, attributable states; following grants no control authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudienceFollowTruth {
    /// Follow / break away / request follow / take over are distinct states, never
    /// inferred from cursor motion (must be true).
    pub follow_break_request_takeover_states_distinct: bool,
    /// A breakaway shows a durable banner so the audience knows it is off-anchor.
    pub breakaway_banner_shown: bool,
    /// The presenter's anchor stays visible while a viewer is broken away.
    pub presenter_anchor_visible_on_breakaway: bool,
    /// Following the presenter grants no shared edit/debug control (must be true).
    pub following_grants_no_control: bool,
}

impl AudienceFollowTruth {
    /// Whether the full follow/breakaway truth holds for a claimed surface.
    pub const fn truth_holds(&self) -> bool {
        self.follow_break_request_takeover_states_distinct
            && self.breakaway_banner_shown
            && self.presenter_anchor_visible_on_breakaway
            && self.following_grants_no_control
    }
}

/// Authority separation. Teaching/classroom roles describe participation, never
/// control; they stay separate from edit, debug, and approval authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySeparation {
    /// A teaching role never grants edit authority (must be true).
    pub teaching_role_separate_from_edit_authority: bool,
    /// A teaching role never grants debug/terminal authority (must be true).
    pub teaching_role_separate_from_debug_authority: bool,
    /// A teaching role never grants approval authority (must be true).
    pub teaching_role_separate_from_approval_authority: bool,
    /// Presentation opens no mutation shortcut; mutation rides the ordinary fence
    /// (absolute; must be true).
    pub no_mutation_shortcut: bool,
}

impl AuthoritySeparation {
    /// Whether the full authority separation holds for any row.
    pub const fn separation_holds(&self) -> bool {
        self.teaching_role_separate_from_edit_authority
            && self.teaching_role_separate_from_debug_authority
            && self.teaching_role_separate_from_approval_authority
            && self.no_mutation_shortcut
    }
}

/// Layout-restore evidence. Entering checkpoints the prior layout; exit, cancel,
/// and crash recovery restore it exactly, with no hidden reruns or widened rights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutRestoreEvidence {
    /// Entering presentation checkpoints the prior layout first (must be true).
    pub enter_checkpoints_prior_layout: bool,
    /// Exit restores the checkpointed layout exactly (must be true).
    pub exit_restores_prior_layout: bool,
    /// The restored environment matches the checkpoint refs exactly (must be true).
    pub restore_matches_checkpoint: bool,
    /// Restore replays no hidden side effects or reruns (must be true).
    pub no_hidden_reruns_on_restore: bool,
    /// Exit, cancel, and crash recovery all restore the same checkpoint (must be
    /// true).
    pub restored_under_all_triggers: bool,
}

impl LayoutRestoreEvidence {
    /// Whether the full restore evidence holds for any row.
    pub const fn restore_holds(&self) -> bool {
        self.enter_checkpoints_prior_layout
            && self.exit_restores_prior_layout
            && self.restore_matches_checkpoint
            && self.no_hidden_reruns_on_restore
            && self.restored_under_all_triggers
    }
}

/// Accessibility posture. Presentation stays keyboard-first and announced, honors
/// reduced motion, and keeps source provenance labels visible under chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationAccessibilityPosture {
    /// Every presentation affordance has a complete keyboard path (absolute; must
    /// be true).
    pub keyboard_complete: bool,
    /// Mode, follow state, and waypoint changes are announced to assistive tech.
    pub announced_to_assistive_tech: bool,
    /// Reveal and spotlight motion honor the reduced-motion setting.
    pub reduced_motion_honored: bool,
    /// File/symbol/branch/workspace and local/remote/shared labels stay visible
    /// under the overlay (must be true).
    pub provenance_labels_visible: bool,
}

impl PresentationAccessibilityPosture {
    /// Whether the keyboard path is complete (the absolute invariant).
    pub const fn keyboard_path_complete(&self) -> bool {
        self.keyboard_complete
    }

    /// Whether the full accessibility posture holds for a claimed surface.
    pub const fn accessibility_holds(&self) -> bool {
        self.keyboard_complete
            && self.announced_to_assistive_tech
            && self.reduced_motion_honored
            && self.provenance_labels_visible
    }
}

/// A row's verification proof: the proof currency plus a reopenable evidence
/// object, so a qualification grade is backed by an object a reviewer can reopen
/// rather than an asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationVerification {
    /// Currency of the proof backing this row.
    pub proof_currency: PresentationProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref`
    /// is present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl PresentationVerification {
    /// Whether the proof object is reopenable: a present ref carries a distinct
    /// non-display fingerprint and a non-empty summary.
    pub fn proof_reopenable(&self) -> bool {
        match (&self.proof_ref, &self.proof_fingerprint_token) {
            (Some(reference), Some(fingerprint)) => {
                let reference = reference.trim();
                let fingerprint = fingerprint.trim();
                !reference.is_empty() && !fingerprint.is_empty() && fingerprint != reference
            }
            _ => false,
        }
    }

    /// Whether this verification is well-formed: a missing proof carries no ref,
    /// any other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if self.proof_currency.is_absent() {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether this verification backs a current claim for the given origin
    /// posture. A local surface needs locally verified or cached proof; a
    /// shared/imported surface needs current imported proof. Either way the proof
    /// must be reopenable.
    pub fn backs_claim(&self, shared_or_imported: bool) -> bool {
        if !self.proof_reopenable() {
            return false;
        }
        if shared_or_imported {
            self.proof_currency.is_imported_current()
        } else {
            self.proof_currency.is_current_local()
        }
    }
}

/// One claimed (or Labs/unadvertised) presentation surface row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationClaimedSurfaceRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Kind of claimed presentation surface.
    pub surface_kind: PresentationSurfaceKind,
    /// Origin class of the surface.
    pub origin_class: PresentationOriginClass,
    /// Non-display fingerprint token. Must differ from
    /// [`surface_id`](PresentationClaimedSurfaceRow::surface_id).
    pub surface_fingerprint_token: String,
    /// Human-readable row label.
    pub label_summary: String,
    /// Claim posture (claimed beta/preview vs Labs/unadvertised).
    pub claim_posture: PresentationClaimPosture,
    /// The canonical presentation-session object this surface drives.
    pub session: PresentationSession,
    /// The classroom role the local user holds, when the surface is a
    /// teaching/classroom surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classroom_role: Option<TeachingRole>,
    /// Speaker-note privacy posture.
    pub speaker_note_privacy: SpeakerNotePrivacyPosture,
    /// Audience-follow truth.
    pub follow_truth: AudienceFollowTruth,
    /// Authority separation.
    pub authority_separation: AuthoritySeparation,
    /// Layout-restore evidence.
    pub restore_evidence: LayoutRestoreEvidence,
    /// Accessibility posture.
    pub accessibility: PresentationAccessibilityPosture,
    /// Reopenable verification proof backing the qualification claim.
    pub verification: PresentationVerification,
    /// Headline qualification grade publicly claimed for this row.
    pub claimed_grade: PresentationQualificationGrade,
    /// Effective grade after auto-downgrading; equals the claim when every axis is
    /// honest and the proof is current, and ranks strictly below it otherwise.
    pub effective_grade: PresentationQualificationGrade,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<PresentationDowngradeTrigger>,
    /// Precise downgraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl PresentationClaimedSurfaceRow {
    /// Whether qualification for this row is session-backed / imported.
    pub fn shared_or_imported(&self) -> bool {
        self.origin_class.is_shared_or_imported()
    }

    /// Whether the row carries a public claim (claimed beta/preview).
    pub fn is_claimed(&self) -> bool {
        self.claim_posture.is_claimed()
    }

    /// Whether the embedded canonical session preserves source provenance: every
    /// waypoint reuses an existing surface and keeps its file/symbol/branch/
    /// workspace and boundary identity.
    pub fn provenance_preserved(&self) -> bool {
        self.session.preserves_source_provenance
            && self.session.reuses_existing_surfaces_only
            && self.session.waypoints_preserve_provenance()
            && self.session.waypoints_reuse_existing_surfaces()
    }

    /// Whether speaker-note privacy holds and the embedded session agrees: notes
    /// default local-only and every shared note carries an explicit promotion.
    pub fn speaker_note_privacy_ok(&self) -> bool {
        self.speaker_note_privacy.privacy_holds()
            && self.session.speaker_notes_default_local_only
            && self.session.shared_notes_are_explicit()
    }

    /// Whether the embedded session never widens authority and the separation
    /// block holds.
    pub fn authority_separation_ok(&self) -> bool {
        self.authority_separation.separation_holds()
            && !self.session.grants_mutation_authority
            && !self.session.grants_control_authority
            && !self.session.establishes_private_data_ownership
    }

    /// Replays the canonical restore path under every trigger and confirms each
    /// run restores the checkpointed environment with no improvised shell. Proves
    /// the restore evidence by reusing [`restore_from_checkpoint`] rather than
    /// asserting it.
    pub fn restore_round_trips(&self) -> bool {
        [
            RestoreTrigger::Exit,
            RestoreTrigger::Cancel,
            RestoreTrigger::CrashRecovery,
        ]
        .into_iter()
        .all(|trigger| {
            let outcome = restore_from_checkpoint(&self.session, trigger);
            let cp = &self.session.restore_checkpoint;
            outcome.matches_checkpoint
                && !outcome.left_in_improvised_shell
                && outcome.restored_layout_ref == cp.prior_layout_ref
                && outcome.restored_focus_ref == cp.prior_focus_ref
                && outcome.restored_panel_visibility_ref == cp.prior_panel_visibility_ref
                && outcome.restored_accessibility_posture_ref == cp.accessibility_posture_ref
        })
    }

    /// Whether layout restore holds: the evidence block and an actual restore
    /// round-trip across all triggers agree.
    pub fn layout_restore_ok(&self) -> bool {
        self.restore_evidence.restore_holds() && self.restore_round_trips()
    }

    /// Whether the surface is unavailable / degraded.
    pub fn surface_unavailable(&self) -> bool {
        self.surface_kind == PresentationSurfaceKind::UnavailableFallback
    }

    /// Whether the keyboard path stays complete, so presentation never becomes a
    /// dead end.
    pub fn keyboard_fallback_ok(&self) -> bool {
        self.accessibility.keyboard_path_complete()
    }

    /// Whether the verification proof backs a current claim for this row's origin
    /// posture.
    pub fn verification_current(&self) -> bool {
        self.verification.backs_claim(self.shared_or_imported())
    }

    /// Whether a classroom role, when present, is consistent with the surface: a
    /// teaching role never drives broader authority, and only the classroom
    /// surface carries one.
    pub fn classroom_role_consistent(&self) -> bool {
        match self.classroom_role {
            Some(role) => {
                !role.grants_terminal_or_debug_control() && !role.implies_broader_authority()
            }
            None => true,
        }
    }

    /// Whether a claimed row must downgrade below its claim because an axis is
    /// denied or the verification proof is not current. Unclaimed surfaces make no
    /// claim to downgrade from.
    pub fn needs_downgrade(&self) -> bool {
        if !self.is_claimed() {
            return false;
        }
        !self.verification_current()
            || !self.speaker_note_privacy_ok()
            || !self.follow_truth.truth_holds()
            || !self.authority_separation_ok()
            || !self.layout_restore_ok()
            || !self.accessibility.accessibility_holds()
            || !self.provenance_preserved()
            || self.surface_unavailable()
    }

    /// Whether the effective grade ranks strictly below the claim.
    pub fn properly_downgraded(&self) -> bool {
        self.effective_grade.rank() < self.claimed_grade.rank()
    }

    /// Whether the effective grade and downgrade evidence are consistent.
    ///
    /// When the row does not need downgrade the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise downgraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.properly_downgraded()
                && self.downgrade_trigger.is_some()
                && self
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the imported posture is consistent: a shared/imported surface never
    /// reads as a locally verified one, and a local surface never leans on imported
    /// proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.shared_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether the surface fingerprint is a real non-display basis distinct from
    /// the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.surface_fingerprint_token.trim();
        !token.is_empty() && token != self.surface_id.trim()
    }

    /// Whether the embedded session is structurally consistent: it carries the
    /// canonical record kind, at least one waypoint, and the safe guardrail flags.
    pub fn session_well_formed(&self) -> bool {
        self.session.record_kind == crate::presentation_mode::PRESENTATION_SESSION_RECORD_KIND
            && self.session.schema_version
                == crate::presentation_mode::PRESENTATION_MODE_BETA_SCHEMA_VERSION
            && !self.session.session_id.trim().is_empty()
            && !self.session.waypoints.is_empty()
            && !self.session.grants_mutation_authority
            && !self.session.grants_control_authority
            && !self.session.establishes_private_data_ownership
            && self.session.speaker_notes_default_local_only
            && self.session.preserves_source_provenance
            && self.session.reuses_existing_surfaces_only
    }

    /// Whether every field required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.surface_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.session_well_formed()
            && self.classroom_role_consistent()
            && self.verification.is_well_formed()
            && self.downgrade_consistent()
            && self.imported_posture_consistent()
            && self.keyboard_fallback_ok()
            && self.speaker_note_privacy.export_safe()
            && self.authority_separation.no_mutation_shortcut
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationQualificationGuardrails {
    /// Presentation stays a thin layer over existing panes; source identity and
    /// boundary labels stay visible.
    pub thin_layer_over_existing_panes: bool,
    /// Speaker notes default local/private and only share through explicit promotion.
    pub speaker_notes_default_local_and_explicitly_shared: bool,
    /// Follow, break away, request follow, and take over stay distinct states.
    pub follow_breakaway_request_takeover_states_distinct: bool,
    /// Teaching/classroom roles stay separate from edit/debug/approval authority.
    pub teaching_roles_separate_from_authority: bool,
    /// Entering checkpoints the prior layout and exiting restores it exactly.
    pub enter_checkpoints_and_exit_restores_layout: bool,
    /// A keyboard-first path is always available; presentation is never a dead end.
    pub keyboard_first_path_always_available: bool,
    /// Raw speaker-note bodies never enter support/diagnostics/telemetry exports.
    pub raw_speaker_note_bodies_never_exported: bool,
    /// Claimed presentation surfaces stay separate from broader collaboration
    /// ambitions.
    pub claimed_surfaces_separated_from_collaboration_ambitions: bool,
    /// Any claimed surface lacking current proof auto-downgrades below its claim.
    pub rows_auto_downgrade_without_current_proof: bool,
}

impl PresentationQualificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.thin_layer_over_existing_panes
            && self.speaker_notes_default_local_and_explicitly_shared
            && self.follow_breakaway_request_takeover_states_distinct
            && self.teaching_roles_separate_from_authority
            && self.enter_checkpoints_and_exit_restores_layout
            && self.keyboard_first_path_always_available
            && self.raw_speaker_note_bodies_never_exported
            && self.claimed_surfaces_separated_from_collaboration_ambitions
            && self.rows_auto_downgrade_without_current_proof
    }
}

/// Consumer projection block: the surfaces that read this matrix without cloning
/// presentation-state text by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationQualificationConsumerProjection {
    /// Product surfaces ingest this matrix.
    pub product_ingests_matrix: bool,
    /// Restore/recovery surfaces ingest the same session objects.
    pub restore_ingests_matrix: bool,
    /// Help / docs surfaces ingest the same matrix.
    pub help_ingests_matrix: bool,
    /// Accessibility surfaces ingest the same matrix.
    pub accessibility_ingests_matrix: bool,
    /// Diagnostics surfaces ingest the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// Support-export surfaces ingest the same matrix.
    pub support_export_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Downgraded surfaces are visibly labeled below their claim in every surface.
    pub downgraded_surfaces_labeled_below_claim: bool,
}

impl PresentationQualificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.restore_ingests_matrix
            && self.help_ingests_matrix
            && self.accessibility_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.support_export_ingests_matrix
            && self.release_control_ingests_matrix
            && self.downgraded_surfaces_labeled_below_claim
    }
}

/// Verification freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationQualificationFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically downgrades claimed rows.
    pub auto_downgrade_on_stale: bool,
}

impl PresentationQualificationFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`M5PresentationQualificationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PresentationQualificationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-surface rows.
    pub rows: Vec<PresentationClaimedSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: PresentationQualificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: PresentationQualificationConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: PresentationQualificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 presentation-session / walkthrough-waypoint / speaker-note /
/// audience-follow qualification matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresentationQualificationMatrixPacket {
    /// Record kind; must equal [`PRESENTATION_QUALIFICATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PRESENTATION_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-surface rows.
    pub rows: Vec<PresentationClaimedSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: PresentationQualificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: PresentationQualificationConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: PresentationQualificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PresentationQualificationMatrixPacket {
    /// Builds a presentation-qualification matrix packet.
    pub fn new(input: M5PresentationQualificationMatrixPacketInput) -> Self {
        Self {
            record_kind: PRESENTATION_QUALIFICATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_QUALIFICATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some row in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<PresentationSurfaceKind> {
        self.rows.iter().map(|row| row.surface_kind).collect()
    }

    /// Count of rows that auto-downgraded below their claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of Labs/unadvertised rows.
    pub fn labs_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_posture == PresentationClaimPosture::LabsUnadvertised)
            .count()
    }

    /// Count of shared-session-linked / imported rows.
    pub fn shared_or_imported_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.shared_or_imported())
            .count()
    }

    /// Resolves a row by its id.
    pub fn row(&self, surface_id: &str) -> Option<&PresentationClaimedSurfaceRow> {
        self.rows.iter().find(|row| row.surface_id == surface_id)
    }

    /// Validates the presentation-qualification matrix invariants.
    pub fn validate(&self) -> Vec<PresentationMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PRESENTATION_QUALIFICATION_MATRIX_RECORD_KIND {
            violations.push(PresentationMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != PRESENTATION_QUALIFICATION_MATRIX_SCHEMA_VERSION {
            violations.push(PresentationMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PresentationMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(PresentationMatrixViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PresentationMatrixViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(PresentationMatrixViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("presentation qualification matrix packet serializes"),
        ) {
            violations.push(PresentationMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("presentation qualification matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, accessibility, or release
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Presentation-Session / Walkthrough-Waypoint / Speaker-Note / Audience-Follow Qualification Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} Labs/unadvertised, {} shared/imported, {} downgraded)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.labs_row_count(),
            self.shared_or_imported_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            PresentationSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.surface_id,
                row.surface_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - posture `{}`, origin `{}`\n",
                row.claim_posture.as_str(),
                row.origin_class.as_str()
            ));
            out.push_str(&format!(
                "  - session `{}`: layout = `{}`, lifecycle = `{}`, leader/follow = `{}`, audience = `{}`\n",
                row.session.session_id,
                row.session.layout_preset.as_str(),
                row.session.lifecycle_state.as_str(),
                row.session.leader_follow_state.as_str(),
                row.session.audience_scope.as_str()
            ));
            if let Some(role) = row.classroom_role {
                out.push_str(&format!("  - classroom role = `{}`\n", role.as_str()));
            }
            out.push_str(&format!(
                "  - speaker-note privacy = {}, follow truth = {}, authority separation = {}\n",
                row.speaker_note_privacy_ok(),
                row.follow_truth.truth_holds(),
                row.authority_separation_ok()
            ));
            out.push_str(&format!(
                "  - layout restore = {}, accessibility = {}, provenance preserved = {}\n",
                row.layout_restore_ok(),
                row.accessibility.accessibility_holds(),
                row.provenance_preserved()
            ));
            out.push_str(&format!(
                "  - verification = `{}`\n",
                row.verification.proof_currency.as_str()
            ));
            if let Some(label) = &row.downgraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum PresentationQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PresentationMatrixViolation>),
}

impl fmt::Display for PresentationQualificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "presentation qualification matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "presentation qualification matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PresentationQualificationArtifactError {}

/// Validation failures emitted by [`M5PresentationQualificationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no row.
    RequiredSurfaceKindMissing,
    /// No Labs/unadvertised row separates claimed scope from broader ambitions.
    LabsSurfaceCaseMissing,
    /// No row demonstrates honest auto-downgrade on a denied axis.
    DowngradedRowCaseMissing,
    /// No clean, current, claimed row anchors a fully qualified claim.
    CleanClaimedCaseMissing,
    /// No shared-session-linked / imported row is present.
    SharedOrImportedCaseMissing,
    /// No unavailable / fallback row proves a complete keyboard-first path.
    UnavailableFallbackCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not downgraded below its claim despite a denied axis or
    /// uncurrent proof.
    RowNotDowngradedOnDeniedAxis,
    /// A downgraded row lacks a precise downgraded label or trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// A row's surface fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A row's keyboard-first path is missing.
    KeyboardFallbackMissing,
    /// A row exports a raw speaker-note body.
    RawSpeakerNoteBodyExported,
    /// A row widens mutation, control, or private-data authority.
    AuthorityWidened,
    /// A row's restore does not round-trip the checkpointed layout.
    RestoreDoesNotRoundTrip,
    /// A row's embedded session erases source provenance.
    SourceProvenanceErased,
    /// A claimed row leaves speaker-note privacy unverified without downgrading.
    SpeakerNotePrivacyDeniedNotDowngraded,
    /// A claimed row leaves follow/breakaway truth unverified without downgrading.
    FollowTruthDeniedNotDowngraded,
    /// A claimed row leaves layout restore unverified without downgrading.
    LayoutRestoreDeniedNotDowngraded,
    /// A claimed row leaves authority separation unverified without downgrading.
    AuthoritySeparationDeniedNotDowngraded,
    /// A shared-session-linked / imported row reads as a locally verified surface.
    ImportedReadsAsLocal,
    /// A row's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A row's embedded session is structurally inconsistent.
    SessionInconsistent,
    /// A row's classroom role is inconsistent with the surface.
    ClassroomRoleInconsistent,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PresentationMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::LabsSurfaceCaseMissing => "labs_surface_case_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::CleanClaimedCaseMissing => "clean_claimed_case_missing",
            Self::SharedOrImportedCaseMissing => "shared_or_imported_case_missing",
            Self::UnavailableFallbackCaseMissing => "unavailable_fallback_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnDeniedAxis => "row_not_downgraded_on_denied_axis",
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::KeyboardFallbackMissing => "keyboard_fallback_missing",
            Self::RawSpeakerNoteBodyExported => "raw_speaker_note_body_exported",
            Self::AuthorityWidened => "authority_widened",
            Self::RestoreDoesNotRoundTrip => "restore_does_not_round_trip",
            Self::SourceProvenanceErased => "source_provenance_erased",
            Self::SpeakerNotePrivacyDeniedNotDowngraded => {
                "speaker_note_privacy_denied_not_downgraded"
            }
            Self::FollowTruthDeniedNotDowngraded => "follow_truth_denied_not_downgraded",
            Self::LayoutRestoreDeniedNotDowngraded => "layout_restore_denied_not_downgraded",
            Self::AuthoritySeparationDeniedNotDowngraded => {
                "authority_separation_denied_not_downgraded"
            }
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::SessionInconsistent => "session_inconsistent",
            Self::ClassroomRoleInconsistent => "classroom_role_inconsistent",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_presentation_qualification_matrix_export(
) -> Result<M5PresentationQualificationMatrixPacket, PresentationQualificationArtifactError> {
    let packet: M5PresentationQualificationMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/presentation/m5-presentation-qualification-matrix/support_export.json"
        )))
        .map_err(PresentationQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PresentationQualificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PresentationQualificationMatrixPacket,
    violations: &mut Vec<PresentationMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PRESENTATION_SESSION_SCHEMA_REF,
        FOLLOW_WAYPOINT_SCHEMA_REF,
        SPEAKER_NOTE_SCHEMA_REF,
        PRESENTATION_QUALIFICATION_MATRIX_ARTIFACT_REF,
        PRESENTATION_QUALIFICATION_MATRIX_SUMMARY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PresentationMatrixViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &M5PresentationQualificationMatrixPacket,
    violations: &mut Vec<PresentationMatrixViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in PresentationSurfaceKind::ALL {
        if !surface_kinds.contains(&required) {
            violations.push(PresentationMatrixViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    if packet.labs_row_count() == 0 {
        violations.push(PresentationMatrixViolation::LabsSurfaceCaseMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(PresentationMatrixViolation::DowngradedRowCaseMissing);
    }

    if !packet.rows.iter().any(|row| {
        !row.needs_downgrade()
            && row.is_claimed()
            && row.claimed_grade == PresentationQualificationGrade::QualifiedClaimedSurface
            && row.verification_current()
    }) {
        violations.push(PresentationMatrixViolation::CleanClaimedCaseMissing);
    }

    if packet.shared_or_imported_row_count() == 0 {
        violations.push(PresentationMatrixViolation::SharedOrImportedCaseMissing);
    }

    if !packet.rows.iter().any(|row| {
        row.surface_kind == PresentationSurfaceKind::UnavailableFallback
            && row.keyboard_fallback_ok()
    }) {
        violations.push(PresentationMatrixViolation::UnavailableFallbackCaseMissing);
    }
}

fn validate_rows(
    packet: &M5PresentationQualificationMatrixPacket,
    violations: &mut Vec<PresentationMatrixViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(PresentationMatrixViolation::RowIncomplete);
        }
        if row.needs_downgrade() && !row.properly_downgraded() {
            violations.push(PresentationMatrixViolation::RowNotDowngradedOnDeniedAxis);
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(PresentationMatrixViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.fingerprint_independent_of_id() {
            violations.push(PresentationMatrixViolation::FingerprintSubstitutesIdentity);
        }

        // Absolute invariants — never allowed even on a downgraded or Labs row.
        if !row.keyboard_fallback_ok() {
            violations.push(PresentationMatrixViolation::KeyboardFallbackMissing);
        }
        if !row.speaker_note_privacy.export_safe() {
            violations.push(PresentationMatrixViolation::RawSpeakerNoteBodyExported);
        }
        if !row.authority_separation_ok() {
            violations.push(PresentationMatrixViolation::AuthorityWidened);
        }
        if !row.restore_round_trips() {
            violations.push(PresentationMatrixViolation::RestoreDoesNotRoundTrip);
        }
        if !row.provenance_preserved() {
            violations.push(PresentationMatrixViolation::SourceProvenanceErased);
        }
        if !row.session_well_formed() {
            violations.push(PresentationMatrixViolation::SessionInconsistent);
        }
        if !row.classroom_role_consistent() {
            violations.push(PresentationMatrixViolation::ClassroomRoleInconsistent);
        }

        // Denied-axis conditions — for a claimed row they must be reflected by a
        // strict downgrade rather than left standing at the claim.
        if row.is_claimed() && !row.properly_downgraded() {
            if !row.speaker_note_privacy_ok() {
                violations.push(PresentationMatrixViolation::SpeakerNotePrivacyDeniedNotDowngraded);
            }
            if !row.follow_truth.truth_holds() {
                violations.push(PresentationMatrixViolation::FollowTruthDeniedNotDowngraded);
            }
            if !row.layout_restore_ok() {
                violations.push(PresentationMatrixViolation::LayoutRestoreDeniedNotDowngraded);
            }
            if !row.authority_separation_ok() {
                violations
                    .push(PresentationMatrixViolation::AuthoritySeparationDeniedNotDowngraded);
            }
        }

        if !row.imported_posture_consistent() {
            violations.push(PresentationMatrixViolation::ImportedReadsAsLocal);
        }
        if !row.verification.is_well_formed() {
            violations.push(PresentationMatrixViolation::VerificationProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(PresentationMatrixViolation::RowEvidenceMissing);
        }
    }
}

/// Whether a downgraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "request failed"
            | "downgraded"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_presentation_qualification_matrix_packet`].
pub const SEED_PRESENTATION_QUALIFICATION_PACKET_ID: &str =
    "m5-presentation-qualification-matrix:stable:0001";

/// Mint timestamp used by [`seeded_presentation_qualification_matrix_packet`].
pub const SEED_PRESENTATION_QUALIFICATION_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating presentation-qualification matrix packet that
/// the checked-in support export, the Markdown summary, and the conformance tests
/// all share, so the in-crate builder stays byte-aligned with the artifact.
///
/// The seed covers every claimed presentation surface kind, anchors a clean local
/// presenter walkthrough, an audience-follow surface with invited guests, a
/// local-only speaker-notes rehearsal, a shared-session classroom teaching surface
/// that never reads as a local rerun, a layout-restore surface proven across exit/
/// cancel/crash, an unavailable-fallback surface that downgrades honestly to a
/// keyboard-first path, a Labs free-roam surface kept out of public scope, and one
/// claimed walkthrough that auto-downgrades because its verification proof went
/// stale.
pub fn seeded_presentation_qualification_matrix_packet() -> M5PresentationQualificationMatrixPacket
{
    M5PresentationQualificationMatrixPacket::new(M5PresentationQualificationMatrixPacketInput {
        packet_id: SEED_PRESENTATION_QUALIFICATION_PACKET_ID.to_owned(),
        label: "M5 Presentation-Session / Walkthrough-Waypoint / Speaker-Note / Audience-Follow Qualification Matrix"
            .to_owned(),
        rows: seeded_rows(),
        guardrails: PresentationQualificationGuardrails {
            thin_layer_over_existing_panes: true,
            speaker_notes_default_local_and_explicitly_shared: true,
            follow_breakaway_request_takeover_states_distinct: true,
            teaching_roles_separate_from_authority: true,
            enter_checkpoints_and_exit_restores_layout: true,
            keyboard_first_path_always_available: true,
            raw_speaker_note_bodies_never_exported: true,
            claimed_surfaces_separated_from_collaboration_ambitions: true,
            rows_auto_downgrade_without_current_proof: true,
        },
        consumer_projection: PresentationQualificationConsumerProjection {
            product_ingests_matrix: true,
            restore_ingests_matrix: true,
            help_ingests_matrix: true,
            accessibility_ingests_matrix: true,
            diagnostics_ingests_matrix: true,
            support_export_ingests_matrix: true,
            release_control_ingests_matrix: true,
            downgraded_surfaces_labeled_below_claim: true,
        },
        verification_freshness: PresentationQualificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_PRESENTATION_QUALIFICATION_MINTED_AT.to_owned(),
            auto_downgrade_on_stale: true,
        },
        source_contract_refs: vec![
            PRESENTATION_SESSION_SCHEMA_REF.to_owned(),
            FOLLOW_WAYPOINT_SCHEMA_REF.to_owned(),
            SPEAKER_NOTE_SCHEMA_REF.to_owned(),
            PRESENTATION_QUALIFICATION_MATRIX_ARTIFACT_REF.to_owned(),
            PRESENTATION_QUALIFICATION_MATRIX_SUMMARY_REF.to_owned(),
            PRESENTATION_AND_WALKTHROUGH_TRUTH_DOC_REF.to_owned(),
            LEARNING_AND_PRESENTATION_CONTRACT_REF.to_owned(),
            COLLAB_RESTORE_AND_PRESENTATION_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_PRESENTATION_QUALIFICATION_MINTED_AT.to_owned(),
    })
}

fn seeded_rows() -> Vec<PresentationClaimedSurfaceRow> {
    vec![
        presenter_walkthrough_row(),
        audience_follow_row(),
        speaker_notes_row(),
        classroom_teaching_row(),
        layout_restore_row(),
        unavailable_fallback_row(),
        labs_free_roam_row(),
        stale_walkthrough_downgraded_row(),
    ]
}

/// On-anchor presenter walkthrough: the clean, fully-claimed local surface.
fn presenter_walkthrough_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.presenter_walkthrough",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint("presenter_walkthrough"),
    )
    .layout(LayoutPreset::FocusedSingle)
    .focus("presentation.waypoint.presenter_walkthrough.0001")
    .waypoint(editor_waypoint(
        "presentation.waypoint.presenter_walkthrough.0001",
        0,
        WaypointCompletionState::Current,
        Some(SpeakerNote::local(
            "presentation.note.presenter_walkthrough.0001",
            "presentation.waypoint.presenter_walkthrough.0001",
            "Open with the request entry point",
        )),
    ))
    .waypoint(diff_waypoint(
        "presentation.waypoint.presenter_walkthrough.0002",
        1,
        WaypointCompletionState::Pending,
    ))
    .participant(viewer(
        "presentation.participant.presenter_walkthrough.viewer",
    ))
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:presenter-walkthrough:local:0001",
        surface_kind: PresentationSurfaceKind::PresenterWalkthrough,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "On-anchor presenter walkthrough over the editor and diff surfaces with a local-only speaker note",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::VerifiedCurrent,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    })
}

/// Audience-follow surface with invited guests: distinct follow/breakaway states.
fn audience_follow_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.audience_follow",
        LeaderFollowState::FollowingPresenter,
        AudienceScope::InvitedGuests,
        checkpoint("audience_follow"),
    )
    .layout(LayoutPreset::SplitCompare)
    .focus("presentation.waypoint.audience_follow.0001")
    .waypoint(docs_waypoint(
        "presentation.waypoint.audience_follow.0001",
        0,
        WaypointCompletionState::Current,
    ))
    .participant(AudienceParticipant {
        participant_id: "presentation.participant.audience_follow.following".to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::Following,
        is_external_guest: true,
    })
    .participant(AudienceParticipant {
        participant_id: "presentation.participant.audience_follow.broken_away".to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::BrokenAway,
        is_external_guest: true,
    })
    .participant(AudienceParticipant {
        participant_id: "presentation.participant.audience_follow.requesting".to_owned(),
        role_badge: ParticipantRole::CoPresenter,
        follow_state: ParticipantFollowState::RequestingFollow,
        is_external_guest: false,
    })
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:audience-follow:local:0001",
        surface_kind: PresentationSurfaceKind::AudienceFollow,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Audience-follow surface with invited guests where follow, break away, and request follow stay distinct, attributable states",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::VerifiedCurrent,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    })
}

/// Local-only speaker-notes rehearsal: notes never leave the machine.
fn speaker_notes_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.speaker_notes",
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint("speaker_notes"),
    )
    .layout(LayoutPreset::NarrativeWide)
    .focus("presentation.waypoint.speaker_notes.0001")
    .waypoint(docs_waypoint_with_note(
        "presentation.waypoint.speaker_notes.0001",
        0,
        WaypointCompletionState::Current,
        SpeakerNote::local(
            "presentation.note.speaker_notes.0001",
            "presentation.waypoint.speaker_notes.0001",
            "Remind the room the notes stay local",
        )
        .with_next_step("Advance to the topology map"),
    ))
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:speaker-notes:local:0001",
        surface_kind: PresentationSurfaceKind::SpeakerNotes,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Solo-rehearsal speaker-note tray whose presenter notes default local/private and never enter an export",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::CachedWithinWindow,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    })
}

/// Shared-session classroom teaching surface: a deliberately narrowed,
/// session-backed surface whose moderator role never widens authority.
fn classroom_teaching_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.classroom_teaching",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint("classroom_teaching"),
    )
    .layout(LayoutPreset::FocusedSingle)
    .focus("presentation.waypoint.classroom_teaching.0001")
    .waypoint(graph_waypoint(
        "presentation.waypoint.classroom_teaching.0001",
        0,
        WaypointCompletionState::Current,
    ))
    .participant(viewer("presentation.participant.classroom_teaching.viewer"))
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:classroom-teaching:shared:0001",
        surface_kind: PresentationSurfaceKind::ClassroomTeaching,
        origin_class: PresentationOriginClass::SharedSessionLinkedSurface,
        label: "Shared-session classroom teaching surface where the moderator role drives attention but never edit, debug, or approval authority",
        claim_posture: PresentationClaimPosture::ClaimedPreview,
        session,
        classroom_role: Some(TeachingRole::Moderator),
        currency: PresentationProofCurrency::ImportedCurrent,
        claimed: PresentationQualificationGrade::QualifiedNarrowedSurface,
    })
}

/// Layout-restore surface: proves checkpoint-on-enter / restore-on-exit across
/// exit, cancel, and crash recovery.
fn layout_restore_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.layout_restore",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint("layout_restore"),
    )
    .layout(LayoutPreset::SplitCompare)
    .focus("presentation.waypoint.layout_restore.0001")
    .waypoint(notebook_waypoint(
        "presentation.waypoint.layout_restore.0001",
        0,
        WaypointCompletionState::Current,
    ))
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:layout-restore:local:0001",
        surface_kind: PresentationSurfaceKind::LayoutRestore,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Layout-restore surface where entering checkpoints the prior layout and exit, cancel, and crash recovery all restore it exactly",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::VerifiedCurrent,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    })
}

/// Unavailable / fallback surface that always keeps a complete keyboard-first
/// path; the claimed surface downgrades honestly to Labs scope.
fn unavailable_fallback_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.unavailable_fallback",
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint("unavailable_fallback"),
    )
    .lifecycle(SessionLifecycleState::Active)
    .layout(LayoutPreset::InheritCurrent)
    .focus("presentation.waypoint.unavailable_fallback.0001")
    .waypoint(editor_waypoint(
        "presentation.waypoint.unavailable_fallback.0001",
        0,
        WaypointCompletionState::Current,
        None,
    ))
    .build();
    let mut row = base_row(BaseRow {
        surface_id: "presentation-qual:unavailable-fallback:local:0001",
        surface_kind: PresentationSurfaceKind::UnavailableFallback,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Unavailable presentation surface that always falls back to a keyboard-first walkthrough path",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::VerifiedCurrent,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    });
    row.effective_grade = PresentationQualificationGrade::LabsUnadvertisedSurface;
    row.downgrade_trigger = Some(PresentationDowngradeTrigger::SurfaceUnavailableDowngraded);
    row.downgraded_label = Some(
        "Presentation overlay unavailable; held at fallback scope with a complete keyboard-first walkthrough path rather than claiming a live overlay"
            .to_owned(),
    );
    row
}

/// Labs / unadvertised free-roam co-browsing surface, kept out of public scope.
fn labs_free_roam_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.labs_free_roam",
        LeaderFollowState::BrokenAway,
        AudienceScope::SharedWorkspace,
        checkpoint("labs_free_roam"),
    )
    .layout(LayoutPreset::NarrativeWide)
    .focus("presentation.waypoint.labs_free_roam.0001")
    .waypoint(graph_waypoint(
        "presentation.waypoint.labs_free_roam.0001",
        0,
        WaypointCompletionState::Current,
    ))
    .participant(AudienceParticipant {
        participant_id: "presentation.participant.labs_free_roam.broken_away".to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::BrokenAway,
        is_external_guest: false,
    })
    .build();
    base_row(BaseRow {
        surface_id: "presentation-qual:audience-follow:labs-free-roam:0001",
        surface_kind: PresentationSurfaceKind::AudienceFollow,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Labs/unadvertised free-roam co-browsing where every viewer breaks away independently, explicitly out of stable scope",
        claim_posture: PresentationClaimPosture::LabsUnadvertised,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::VerifiedCurrent,
        claimed: PresentationQualificationGrade::LabsUnadvertisedSurface,
    })
}

/// Claimed walkthrough whose verification proof went stale: auto-downgrades below
/// its claim with a precise label.
fn stale_walkthrough_downgraded_row() -> PresentationClaimedSurfaceRow {
    let session = PresentationSessionBuilder::new(
        "presentation.session.stale_walkthrough",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint("stale_walkthrough"),
    )
    .layout(LayoutPreset::FocusedSingle)
    .focus("presentation.waypoint.stale_walkthrough.0001")
    .waypoint(diff_waypoint(
        "presentation.waypoint.stale_walkthrough.0001",
        0,
        WaypointCompletionState::Current,
    ))
    .build();
    let mut row = base_row(BaseRow {
        surface_id: "presentation-qual:presenter-walkthrough:stale:0001",
        surface_kind: PresentationSurfaceKind::PresenterWalkthrough,
        origin_class: PresentationOriginClass::FirstPartyLocalSurface,
        label: "Presenter walkthrough that claimed full scope but whose audience-follow parity proof aged outside its freshness window",
        claim_posture: PresentationClaimPosture::ClaimedBeta,
        session,
        classroom_role: None,
        currency: PresentationProofCurrency::StaleExpired,
        claimed: PresentationQualificationGrade::QualifiedClaimedSurface,
    });
    row.effective_grade = PresentationQualificationGrade::QualifiedNarrowedSurface;
    row.downgrade_trigger = Some(PresentationDowngradeTrigger::StaleVerificationProof);
    row.downgraded_label = Some(
        "Audience-follow parity proof aged outside its freshness window; held narrowed until re-verified rather than claiming full walkthrough scope"
            .to_owned(),
    );
    row
}

// ---- shared seed helpers ----

fn checkpoint(slug: &str) -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: format!("presentation.checkpoint.{slug}"),
        prior_layout_ref: format!("layout.snapshot.{slug}"),
        prior_focus_ref: format!("focus.chain.{slug}"),
        prior_panel_visibility_ref: format!("panels.visibility.{slug}"),
        accessibility_posture_ref: format!("a11y.posture.{slug}"),
        captured_at: "2026-06-14T00:00:00Z".to_owned(),
    }
}

fn editor_waypoint(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
    note: Option<SpeakerNote>,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: "Editor anchor".to_owned(),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: format!("editor.buffer.{id}"),
        file_path_ref: Some(format!("file.path.{id}")),
        symbol_anchor_ref: Some("symbol.entrypoint".to_owned()),
        branch_workspace_ref: "branch.main.workspace.primary".to_owned(),
        boundary_label: BoundaryLabel::Local,
        zoom_layout_hint_ref: Some(format!("zoom.hint.{id}")),
        reveal_action_ref: Some(format!("reveal.action.{id}")),
        completion_state,
        speaker_note: note,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn diff_waypoint(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: "Diff anchor".to_owned(),
        surface_kind: WalkthroughSurfaceKind::Diff,
        target_object_ref: format!("diff.hunk.{id}"),
        file_path_ref: Some(format!("file.path.{id}")),
        symbol_anchor_ref: None,
        branch_workspace_ref: "branch.main.workspace.primary".to_owned(),
        boundary_label: BoundaryLabel::Local,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn docs_waypoint(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: "Docs anchor".to_owned(),
        surface_kind: WalkthroughSurfaceKind::Docs,
        target_object_ref: format!("docs.node.{id}"),
        file_path_ref: None,
        symbol_anchor_ref: None,
        branch_workspace_ref: "branch.main.workspace.primary".to_owned(),
        boundary_label: BoundaryLabel::Shared,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn docs_waypoint_with_note(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
    note: SpeakerNote,
) -> FollowWaypoint {
    let mut waypoint = docs_waypoint(id, ordinal, completion_state);
    waypoint.speaker_note = Some(note);
    waypoint
}

fn graph_waypoint(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: "Topology anchor".to_owned(),
        surface_kind: WalkthroughSurfaceKind::Graph,
        target_object_ref: format!("graph.node.{id}"),
        file_path_ref: None,
        symbol_anchor_ref: None,
        branch_workspace_ref: "branch.main.workspace.primary".to_owned(),
        boundary_label: BoundaryLabel::Remote,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn notebook_waypoint(
    id: &str,
    ordinal: u32,
    completion_state: WaypointCompletionState,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: "Notebook anchor".to_owned(),
        surface_kind: WalkthroughSurfaceKind::Notebook,
        target_object_ref: format!("notebook.cell.{id}"),
        file_path_ref: Some(format!("file.path.{id}")),
        symbol_anchor_ref: None,
        branch_workspace_ref: "branch.main.workspace.primary".to_owned(),
        boundary_label: BoundaryLabel::Local,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn viewer(participant_id: &str) -> AudienceParticipant {
    AudienceParticipant {
        participant_id: participant_id.to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::Following,
        is_external_guest: false,
    }
}

fn safe_speaker_note_privacy() -> SpeakerNotePrivacyPosture {
    SpeakerNotePrivacyPosture {
        notes_default_local_only: true,
        shared_notes_require_explicit_promotion: true,
        note_bodies_excluded_from_export: true,
        redaction_before_share: true,
    }
}

fn full_follow_truth() -> AudienceFollowTruth {
    AudienceFollowTruth {
        follow_break_request_takeover_states_distinct: true,
        breakaway_banner_shown: true,
        presenter_anchor_visible_on_breakaway: true,
        following_grants_no_control: true,
    }
}

fn full_authority_separation() -> AuthoritySeparation {
    AuthoritySeparation {
        teaching_role_separate_from_edit_authority: true,
        teaching_role_separate_from_debug_authority: true,
        teaching_role_separate_from_approval_authority: true,
        no_mutation_shortcut: true,
    }
}

fn full_restore_evidence() -> LayoutRestoreEvidence {
    LayoutRestoreEvidence {
        enter_checkpoints_prior_layout: true,
        exit_restores_prior_layout: true,
        restore_matches_checkpoint: true,
        no_hidden_reruns_on_restore: true,
        restored_under_all_triggers: true,
    }
}

fn full_accessibility() -> PresentationAccessibilityPosture {
    PresentationAccessibilityPosture {
        keyboard_complete: true,
        announced_to_assistive_tech: true,
        reduced_motion_honored: true,
        provenance_labels_visible: true,
    }
}

/// Inline constructor input for one seeded surface row.
struct BaseRow {
    surface_id: &'static str,
    surface_kind: PresentationSurfaceKind,
    origin_class: PresentationOriginClass,
    label: &'static str,
    claim_posture: PresentationClaimPosture,
    session: PresentationSession,
    classroom_role: Option<TeachingRole>,
    currency: PresentationProofCurrency,
    claimed: PresentationQualificationGrade,
}

/// Redacts presenter-facing speaker-note text from a session before it is
/// embedded in the export-safe matrix.
///
/// The canonical [`SpeakerNote`] carries a presenter-only `body_label` and
/// `next_step_cue_label`; those never cross this boundary. The note's *posture* —
/// its id, the waypoint it links, its local/shared scope, the explicit-promotion
/// marker, and any citation ids — is kept, so speaker-note privacy stays provable
/// from the export without ever exposing the note body.
fn redact_session_for_export(mut session: PresentationSession) -> PresentationSession {
    for waypoint in &mut session.waypoints {
        if let Some(note) = waypoint.speaker_note.as_mut() {
            note.body_label = None;
            note.next_step_cue_label = None;
        }
    }
    session
}

fn base_row(base: BaseRow) -> PresentationClaimedSurfaceRow {
    let (proof_ref, proof_fingerprint_token) = if base.currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", base.surface_id)),
            Some(format!("fp:proof:{}", base.surface_id)),
        )
    };
    PresentationClaimedSurfaceRow {
        surface_id: base.surface_id.to_owned(),
        surface_kind: base.surface_kind,
        origin_class: base.origin_class,
        surface_fingerprint_token: format!("fp:surface:{}", base.surface_id),
        label_summary: base.label.to_owned(),
        claim_posture: base.claim_posture,
        session: redact_session_for_export(base.session),
        classroom_role: base.classroom_role,
        speaker_note_privacy: safe_speaker_note_privacy(),
        follow_truth: full_follow_truth(),
        authority_separation: full_authority_separation(),
        restore_evidence: full_restore_evidence(),
        accessibility: full_accessibility(),
        verification: PresentationVerification {
            proof_currency: base.currency,
            proof_ref,
            proof_fingerprint_token,
            summary: format!(
                "{} qualification verified with {} proof",
                base.surface_kind.as_str(),
                base.currency.as_str()
            ),
        },
        claimed_grade: base.claimed,
        effective_grade: base.claimed,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:row:{}", base.surface_id)],
        source_contract_refs: vec![LEARNING_AND_PRESENTATION_CONTRACT_REF.to_owned()],
    }
}
