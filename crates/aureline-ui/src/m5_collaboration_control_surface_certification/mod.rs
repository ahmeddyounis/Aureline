//! M05-1313 closing B155 surface certification over the frozen M5 shared-terminal/debug-view, control-grant,
//! presenter-token, consent-envelope, retention-review, and session-restore-view matrix — the explicit shared
//! terminal / debug view, control grant, presenter / moderator token, consent envelope, retention review, and
//! session-restore view that a desktop, browser-companion, incident / support, or audit / export consumer must
//! treat as first-class, durable, export-safe collaboration-control objects rather than incidental presence.
//!
//! Where the freeze matrix ([`crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`]) defines the six
//! governed collaboration-control object classes, the M05-1305..1310 implement lanes resolve each shared
//! terminal / debug view, control-grant / presenter-handoff sheet, consent envelope / join-review sheet,
//! retention review / sealed-archive manifest, and session-restore view / restore-grant posture registry; this
//! closing capstone *certifies* that the shared collaboration-control truth holds on every claimed M5 desktop,
//! companion, support, incident, and audit / export surface — the control authority, single active driver,
//! presenter handoff, join-time consent scope, recording / retention state, and replay-free restore safety — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a shared-session owner, a control-grant flow, a companion-follow
//! consumer, or a support / export consumer reads a collaboration control through (a fully-certified
//! collaboration-control lane; a reviewable collaboration-control record structure; an unproven-control-authority
//! profile; an inferred-active-driver profile; a silently-transferred-presenter profile; an
//! undisclosed-consent-scope profile; a stale-retention-state profile; and an unproven-replay-free-restore
//! profile), not on the underlying object class or implement lane.
//! Each [`CollaborationControlProfileCertificationRow`] certifies one profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! collaboration-control-truth behavior — and either passes (green), auto-narrows its collaboration-control claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh certified
//! claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedCollaborationControlTruth` / `ReviewableCollaborationControlRecord` claim while one of its truth axes is not current is
//! over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound reason
//! and a frozen downgrade trigger) is honestly yellow. Only a fully-certified collaboration-control lane — one whose
//! control authority, single active driver, presenter handoff, join-time consent scope, recording / retention
//! state, and replay-free restore safety all converge on one export-safe, provider-authoritative,
//! internally consistent collaboration-control record — may certify a `CertifiedCollaborationControlTruth` claim; a reviewable,
//! unproven-control-authority, inferred-active-driver, silently-transferred-presenter, undisclosed-consent-scope,
//! stale-retention-state, or unproven-replay-free-restore profile that keeps a certified claim is over-reaching
//! and blocks. The always-on CLI/export axis must always stay certified so support and automation can reconstruct
//! the control authority, single active driver, presenter handoff, consent scope, retention state, and restore
//! safety from the same collaboration-control proof the operator saw.
//!
//! The B155 hard invariants are enforced per row: no profile may acquire terminal / debug control from presence,
//! reconnect, or follow without an explicit grant; allow more than one active driver on a sensitive surface;
//! start recording, transcript retention, or guest-scope widening silently; replay prior terminal / debug input
//! on join or restore; or reveal raw secrets, command text, variable bodies, or clipboard contents without an
//! explicit consent posture and visible guardrail. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical collaboration-control lifecycle matrix proof bundle
//! ([`COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF`]) — the frozen collaboration-control lifecycle matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/collaboration/m5-collaboration-control-surface-certification.schema.json`](../../../../schemas/collaboration/m5-collaboration-control-surface-certification.schema.json).
//! The contract doc is
//! [`docs/collaboration/m5-collaboration-control-surface-certification.md`](../../../../docs/collaboration/m5-collaboration-control-surface-certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix as matrix;
use matrix::{M5CollaborationControlDowngradeTrigger, M5CollaborationControlObject};

/// Schema version stamped on the M05-1313 certification packet.
pub const COLLABORATION_CONTROL_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CollaborationControlProfileCertificationPacket`].
pub const COLLABORATION_CONTROL_CERT_RECORD_KIND: &str =
    "m5_collaboration_control_surface_certification_packet";

/// Stable record-kind tag carried by each [`CollaborationControlProfileCertificationRow`].
pub const COLLABORATION_CONTROL_CERT_ROW_RECORD_KIND: &str =
    "m5_collaboration_control_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const COLLABORATION_CONTROL_CERT_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-control-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const COLLABORATION_CONTROL_CERT_DOC_REF: &str =
    "docs/collaboration/m5-collaboration-control-surface-certification.md";

/// Repo-relative path of the frozen collaboration-control lifecycle matrix schema the certified profiles render.
pub const COLLABORATION_CONTROL_CERT_MATRIX_REF: &str =
    matrix::M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF;

/// The one canonical collaboration-control lifecycle matrix proof bundle every certified profile cites as its
/// first-resolved collaboration-control truth. All eight profiles point back to it rather than cloning per-profile
/// evidence.
pub const COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_COLLABORATION_CONTROL_ARTIFACT_REF;

/// The collaboration-control-health dashboard the release surfaces consume. Recorded as a supporting evidence ref on
/// every row so the certification's collaboration-control truth ties back to the same dashboard consumers read.
pub const COLLABORATION_CONTROL_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_COLLABORATION_CONTROL_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const COLLABORATION_CONTROL_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-control-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COLLABORATION_CONTROL_CERT_CSV_REF: &str =
    "artifacts/release/m5-collaboration-control-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COLLABORATION_CONTROL_CERT_REPORT_REF: &str =
    "artifacts/release/m5-collaboration-control-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const COLLABORATION_CONTROL_CERT_FIXTURE_DIR: &str =
    "fixtures/collaboration/m5-collaboration-control-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const COLLABORATION_CONTROL_CERT_PACKET_ID: &str =
    "m5-collaboration-control-surface-certification:stable:0001";

/// The eight claimed M5 collaboration-control consumer profiles this capstone certifies. Keyed on the profile a
/// shared-session owner, a control-grant / presenter-handoff flow, a companion-follow consumer, or a support /
/// export consumer reads a collaboration control through — a fully-certified collaboration-control lane, a
/// reviewable collaboration-control record structure, an unproven-control-authority profile, an
/// inferred-active-driver profile, a silently-transferred-presenter profile, an undisclosed-consent-scope
/// profile, a stale-retention-state profile, and an unproven-replay-free-restore profile — not on the reusable
/// object class it renders. Only a fully-certified collaboration-control lane profile may certify a certified
/// collaboration-control claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlCertifiedProfile {
    /// A fully-certified collaboration-control lane — a shared terminal / debug session whose control authority,
    /// single active driver, presenter handoff, join-time consent scope, recording / retention state, and
    /// replay-free restore safety all converge on one export-safe, provider-authoritative, internally consistent
    /// collaboration-control record that stays identical across every desktop, companion, support, incident, and
    /// audit / export consumer, certifying the collaboration-control claim exactly right now.
    CertifiedCollaborationControlLane,
    /// A reviewable collaboration-control record structure: a self-sufficient, inspectable consent-envelope /
    /// join-review record (a session-bound record an operator can review), never itself a fully-certified
    /// collaboration-control lane.
    ReviewableCollaborationControlRecordStructure,
    /// A shared terminal / debug view whose control authority can no longer be confirmed explicitly granted; the
    /// claim narrows to a control-authority-unverified projection that discloses the last-known control authority
    /// and never lets presence, reconnect, or follow stand in for an explicit grant.
    UnprovenControlAuthorityProfile,
    /// A control-grant lane whose single active driver cannot be confirmed; the claim narrows to an
    /// active-driver-unverified projection that keeps the sole active driver and its grant scope explicit and never
    /// lets two participants drive one sensitive surface.
    InferredActiveDriverProfile,
    /// A presenter-token lane whose handoff cannot be confirmed reviewed; the claim narrows to a
    /// presenter-handoff-unverified projection that keeps the token holder, handoff target, and moderation scope
    /// explicit and never silently transfers shell / debug control between presenters.
    SilentlyTransferredPresenterProfile,
    /// A consent-envelope lane whose join-time consent scope cannot be proven disclosed (recording, retention,
    /// guest scope, or route visibility undisclosed or silently widened); the claim narrows to a
    /// consent-scope-unverified projection that keeps the disclosed scope explicit and never widens scope silently.
    UndisclosedConsentScopeProfile,
    /// A retention-review lane whose recording / retention state is stale or was broadened silently; the claim
    /// narrows to a retention-state-unverified projection that keeps the last-known recording state, retention
    /// mode, and sealed-archive scope explicit, never broadening retention silently.
    StaleRetentionStateProfile,
    /// A session-restore lane whose replay-free restore safety and recovery evidence is unproven; the claim narrows
    /// to a restore-replay-safety-unverified projection that keeps the read-only reattach, replayed-nothing
    /// evidence, and fresh-grant requirement explicit, never replaying prior terminal / debug input on restore.
    UnprovenReplayFreeRestoreProfile,
}

impl M5CollaborationControlCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5CollaborationControlCertifiedProfile; 8] = [
        M5CollaborationControlCertifiedProfile::CertifiedCollaborationControlLane,
        M5CollaborationControlCertifiedProfile::ReviewableCollaborationControlRecordStructure,
        M5CollaborationControlCertifiedProfile::UnprovenControlAuthorityProfile,
        M5CollaborationControlCertifiedProfile::InferredActiveDriverProfile,
        M5CollaborationControlCertifiedProfile::SilentlyTransferredPresenterProfile,
        M5CollaborationControlCertifiedProfile::UndisclosedConsentScopeProfile,
        M5CollaborationControlCertifiedProfile::StaleRetentionStateProfile,
        M5CollaborationControlCertifiedProfile::UnprovenReplayFreeRestoreProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedCollaborationControlLane => "certified_collaboration_control_lane",
            Self::ReviewableCollaborationControlRecordStructure => {
                "reviewable_collaboration_control_record_structure"
            }
            Self::UnprovenControlAuthorityProfile => "unproven_control_authority_profile",
            Self::InferredActiveDriverProfile => "inferred_active_driver_profile",
            Self::SilentlyTransferredPresenterProfile => "silently_transferred_presenter_profile",
            Self::UndisclosedConsentScopeProfile => "undisclosed_consent_scope_profile",
            Self::StaleRetentionStateProfile => "stale_retention_state_profile",
            Self::UnprovenReplayFreeRestoreProfile => "unproven_replay_free_restore_profile",
        }
    }

    /// True only for the fully-certified collaboration-control lane profile. A certified collaboration-control claim may be
    /// certified on this profile alone; every other profile is at most a reviewable collaboration-control record structure
    /// or a narrowed projection.
    pub const fn is_certified_collaboration_control_lane(self) -> bool {
        matches!(self, Self::CertifiedCollaborationControlLane)
    }
}

/// The claim ladder a certified collaboration-control profile asserts and is certified down to. Minted locally for this
/// capstone: the strongest claim is a fully certified collaboration-control record; each weaker tier is a disclosed
/// projection that keeps the last-known control-authority, active-driver, presenter-handoff, consent-scope,
/// retention-state, or restore-replay-safety posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlCertClaim {
    /// Certified collaboration-control truth: a fully-certified collaboration control whose control authority,
    /// single active driver, presenter handoff, join-time consent scope, recording / retention state, and
    /// replay-free restore safety all join to one export-safe, provider-authoritative, internally consistent record
    /// — the strongest claim, the collaboration-control handling Aureline can present as explicitly-granted and
    /// replay-safe across every consumer.
    CertifiedCollaborationControlTruth,
    /// Reviewable collaboration-control record: a self-sufficient, inspectable session-bound record (a
    /// consent-envelope / join-review record an operator can inspect) that is not itself a fully-certified
    /// collaboration-control lane.
    ReviewableCollaborationControlRecord,
    /// Control-authority-unverified projection: a shared terminal / debug view's control authority cannot be
    /// confirmed explicitly granted; the lane stays a control-authority-unverified projection that discloses the
    /// last-known control authority, never letting presence, reconnect, or follow stand in for an explicit grant.
    ControlAuthorityUnverifiedProjection,
    /// Active-driver-unverified projection: a control grant's single active driver cannot be confirmed; the lane
    /// stays an active-driver-unverified projection that keeps the sole active driver and grant scope explicit,
    /// never letting two participants drive one sensitive surface.
    ActiveDriverUnverifiedProjection,
    /// Presenter-handoff-unverified projection: a presenter token's handoff cannot be confirmed reviewed; the lane
    /// stays a presenter-handoff-unverified projection that keeps the token holder, handoff target, and moderation
    /// scope explicit, never silently transferring shell / debug control between presenters.
    PresenterHandoffUnverifiedProjection,
    /// Consent-scope-unverified projection: a consent envelope's join-time scope cannot be proven disclosed; the
    /// lane stays a consent-scope-unverified projection that keeps the disclosed recording, retention, guest, and
    /// route-visibility scope explicit, never widening scope silently.
    ConsentScopeUnverifiedProjection,
    /// Retention-state-unverified projection: a retention review's recording / retention state is stale or was
    /// broadened silently; the lane stays a retention-state-unverified projection that keeps the last-known
    /// recording state, retention mode, and sealed-archive scope explicit, never broadening retention silently.
    RetentionStateUnverifiedProjection,
    /// Restore-replay-safety-unverified projection: a session-restore view's replay-free restore safety and
    /// recovery evidence is unproven; the lane stays a restore-replay-safety-unverified projection that keeps the
    /// read-only reattach, replayed-nothing evidence, and fresh-grant requirement explicit, never replaying input.
    RestoreReplaySafetyUnverifiedProjection,
}

impl M5CollaborationControlCertClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::CertifiedCollaborationControlTruth,
        Self::ReviewableCollaborationControlRecord,
        Self::ControlAuthorityUnverifiedProjection,
        Self::ActiveDriverUnverifiedProjection,
        Self::PresenterHandoffUnverifiedProjection,
        Self::ConsentScopeUnverifiedProjection,
        Self::RetentionStateUnverifiedProjection,
        Self::RestoreReplaySafetyUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedCollaborationControlTruth => 7,
            Self::ReviewableCollaborationControlRecord => 6,
            Self::ControlAuthorityUnverifiedProjection => 5,
            Self::ActiveDriverUnverifiedProjection => 4,
            Self::PresenterHandoffUnverifiedProjection => 3,
            Self::ConsentScopeUnverifiedProjection => 2,
            Self::RetentionStateUnverifiedProjection => 1,
            Self::RestoreReplaySafetyUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-certified, certified collaboration-control record.
    pub const fn asserts_certified_collaboration_control_truth(self) -> bool {
        matches!(self, Self::CertifiedCollaborationControlTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedCollaborationControlTruth | Self::ReviewableCollaborationControlRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedCollaborationControlTruth => "certified_collaboration_control_truth",
            Self::ReviewableCollaborationControlRecord => "reviewable_collaboration_control_record",
            Self::ControlAuthorityUnverifiedProjection => "control_authority_unverified_projection",
            Self::ActiveDriverUnverifiedProjection => "active_driver_unverified_projection",
            Self::PresenterHandoffUnverifiedProjection => "presenter_handoff_unverified_projection",
            Self::ConsentScopeUnverifiedProjection => "consent_scope_unverified_projection",
            Self::RetentionStateUnverifiedProjection => "retention_state_unverified_projection",
            Self::RestoreReplaySafetyUnverifiedProjection => {
                "restore_replay_safety_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and collaboration-control-truth behavior. The CLI/export axis is always-on and must stay
/// certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlCertificationAxis {
    /// Visual parity: the control authority, single active driver, presenter handoff, join-time consent scope,
    /// recording / retention state, and replay-free restore safety are shown on the primary surface without relying
    /// on a chrome-only affordance or a presence badge alone, and no presence still reads as terminal / debug
    /// control.
    Visual,
    /// Keyboard-reach parity: the same collaboration-control truth and its bound request-control / grant / revoke /
    /// handoff / consent-review operations are reachable and operable without a pointer, never hover-only, with
    /// stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a chrome-only affordance, a
    /// presence badge, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the control
    /// authority, single active driver, presenter handoff, consent scope, or retention state.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// control-authority badge, active-driver class, or presenter / consent / retention state.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// control authority, active driver, presenter handoff, consent scope, or retention state when a locale is
    /// incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text / JSON / Markdown
    /// for support and automation.
    CliExport,
    /// Degraded-state parity: an unproven control authority, an unconfirmed active driver, a contested or silently
    /// transferred presenter handoff, an undisclosed or silently widened consent scope, a stale or silently
    /// broadened retention state, or an unproven replay-free restore honestly downgrades a
    /// `CertifiedCollaborationControlTruth` / `ReviewableCollaborationControlRecord` claim rather than reading as a
    /// fresh, provider-authoritative collaboration-control record.
    DegradedState,
    /// Collaboration-control-truth parity: the control authority, single active driver, presenter handoff, consent
    /// scope, recording / retention state, and replay-free restore safety stay explicit and never let presence,
    /// reconnect, or follow imply terminal / debug control; allow more than one active driver on a sensitive
    /// surface; silently transfer a presenter token; start recording, retention, or guest-scope widening silently;
    /// replay prior terminal / debug input on join or restore; or reveal raw secrets, command text, variable
    /// bodies, or clipboard contents without an explicit consent posture and visible guardrail.
    CollaborationControlTruth,
}

impl CollaborationControlCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [CollaborationControlCertificationAxis; 9] = [
        CollaborationControlCertificationAxis::Visual,
        CollaborationControlCertificationAxis::Keyboard,
        CollaborationControlCertificationAxis::ScreenReader,
        CollaborationControlCertificationAxis::HighZoomReflow,
        CollaborationControlCertificationAxis::HighContrast,
        CollaborationControlCertificationAxis::Localization,
        CollaborationControlCertificationAxis::CliExport,
        CollaborationControlCertificationAxis::DegradedState,
        CollaborationControlCertificationAxis::CollaborationControlTruth,
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
            Self::CollaborationControlTruth => "collaboration_control_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl CollaborationControlAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed from
/// the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationControlProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-lane collaboration-control profile claims a certified collaboration-control record, or the narrowing is inconsistent.
    Red,
}

impl CollaborationControlProfileClaimStatus {
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

/// The five B155 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlCertGuardrails {
    /// True if the profile acquires terminal / debug control from presence, reconnect, or follow without an
    /// explicit grant. Must be false.
    pub acquires_terminal_or_debug_control_from_presence_or_follow_without_grant: bool,
    /// True if the profile allows more than one active driver on a sensitive surface. Must be false.
    pub allows_more_than_one_active_driver_on_a_sensitive_surface: bool,
    /// True if the profile starts recording, transcript retention, or guest-scope widening silently. Must be false.
    pub starts_recording_retention_or_guest_scope_widening_silently: bool,
    /// True if the profile replays prior terminal / debug input on join or restore. Must be false.
    pub replays_prior_terminal_or_debug_input_on_join_or_restore: bool,
    /// True if the profile reveals raw secrets, command text, variable bodies, or clipboard contents without an
    /// explicit consent posture and visible guardrail. Must be false.
    pub reveals_raw_secrets_or_clipboard_without_an_explicit_consent_and_visible_guard: bool,
}

impl CollaborationControlCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        acquires_terminal_or_debug_control_from_presence_or_follow_without_grant: false,
        allows_more_than_one_active_driver_on_a_sensitive_surface: false,
        starts_recording_retention_or_guest_scope_widening_silently: false,
        replays_prior_terminal_or_debug_input_on_join_or_restore: false,
        reveals_raw_secrets_or_clipboard_without_an_explicit_consent_and_visible_guard: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.acquires_terminal_or_debug_control_from_presence_or_follow_without_grant
            && !self.allows_more_than_one_active_driver_on_a_sensitive_surface
            && !self.starts_recording_retention_or_guest_scope_widening_silently
            && !self.replays_prior_terminal_or_debug_input_on_join_or_restore
            && !self.reveals_raw_secrets_or_clipboard_without_an_explicit_consent_and_visible_guard
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The control-authority / active-driver / presenter-handoff / consent-scope / retention-state /
    /// restore-replay-safety fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl CollaborationControlCertExportParity {
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
pub struct CollaborationControlAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: CollaborationControlCertificationAxis,
    /// The certification state of the axis.
    pub state: CollaborationControlAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5CollaborationControlDowngradeTrigger>,
}

impl CollaborationControlAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is exactly
    ///   what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            CollaborationControlAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            CollaborationControlAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            CollaborationControlAxisCertificationState::UndisclosedDrift => {
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
pub struct CollaborationControlClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: CollaborationControlCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5CollaborationControlCertClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5CollaborationControlCertClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 collaboration-control object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlProfileCertificationRow {
    /// Record kind; must equal [`COLLABORATION_CONTROL_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COLLABORATION_CONTROL_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5CollaborationControlCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5CollaborationControlCertClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5CollaborationControlCertClaim,
    /// The frozen collaboration-control object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5CollaborationControlObject>,
    /// One outcome per [`CollaborationControlCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<CollaborationControlAxisOutcome>,
    /// The B155 hard invariants; all must hold.
    pub guardrails: CollaborationControlCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<CollaborationControlClaimAutoNarrow>,
    /// The one canonical collaboration-control lifecycle matrix proof bundle this profile cites. Must equal
    /// [`COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: CollaborationControlProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: CollaborationControlCertExportParity,
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

impl CollaborationControlProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: CollaborationControlCertificationAxis,
    ) -> Option<&CollaborationControlAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<CollaborationControlCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && CollaborationControlCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(CollaborationControlAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<CollaborationControlCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == CollaborationControlAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-certified collaboration-control lane
    /// profile may certify a certified collaboration-control record, every hard invariant must hold, CLI/export parity must
    /// always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> CollaborationControlProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return CollaborationControlProfileClaimStatus::Red;
        }

        // Every B155 hard invariant must hold.
        if !self.guardrails.all_held() {
            return CollaborationControlProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return CollaborationControlProfileClaimStatus::Red;
        }

        // Only a fully-certified collaboration-control lane profile may certify a certified collaboration-control record.
        if self
            .certified_claim
            .asserts_certified_collaboration_control_truth()
            && !self.profile.is_certified_collaboration_control_lane()
        {
            return CollaborationControlProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(CollaborationControlCertificationAxis::CliExport) {
            Some(o) if o.state == CollaborationControlAxisCertificationState::Certified => {}
            _ => return CollaborationControlProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == CollaborationControlAxisCertificationState::UndisclosedDrift)
        {
            return CollaborationControlProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return CollaborationControlProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return CollaborationControlProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return CollaborationControlProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return CollaborationControlProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return CollaborationControlProfileClaimStatus::Red;
        }

        CollaborationControlProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COLLABORATION_CONTROL_CERT_ROW_RECORD_KIND
            && self.schema_version == COLLABORATION_CONTROL_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1313 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlProfileCertificationSummary {
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

/// Constructor input for [`CollaborationControlProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollaborationControlProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<CollaborationControlProfileCertificationRow>,
}

/// Checked-in M05-1313 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationControlProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<CollaborationControlProfileCertificationRow>,
    pub summary: CollaborationControlProfileCertificationSummary,
}

impl CollaborationControlProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CollaborationControlProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COLLABORATION_CONTROL_CERT_SCHEMA_VERSION,
            record_kind: COLLABORATION_CONTROL_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: CollaborationControlProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5CollaborationControlCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Collaboration-control object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5CollaborationControlObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5CollaborationControlCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen collaboration-control object class is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5CollaborationControlObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(CollaborationControlCertificationAxis::CliExport)
                .is_some_and(|o| o.state == CollaborationControlAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CollaborationControlProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationControlProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationControlProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationControlProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(CollaborationControlProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        CollaborationControlProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(CollaborationControlProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CollaborationControlCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COLLABORATION_CONTROL_CERT_SCHEMA_VERSION {
            violations.push(CollaborationControlCertificationViolation::SchemaVersion {
                expected: COLLABORATION_CONTROL_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COLLABORATION_CONTROL_CERT_RECORD_KIND {
            violations.push(CollaborationControlCertificationViolation::RecordKind {
                expected: COLLABORATION_CONTROL_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CollaborationControlCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF {
            violations.push(CollaborationControlCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CollaborationControlCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(CollaborationControlCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    CollaborationControlCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    CollaborationControlCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    CollaborationControlCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B155 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    CollaborationControlCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a fully-certified collaboration-control lane profile may certify a certified collaboration-control record.
            if row
                .certified_claim
                .asserts_certified_collaboration_control_truth()
                && !row.profile.is_certified_collaboration_control_lane()
            {
                violations.push(
                    CollaborationControlCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(CollaborationControlCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    CollaborationControlCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    CollaborationControlCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    CollaborationControlCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == CollaborationControlProfileClaimStatus::Red {
                violations.push(CollaborationControlCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(CollaborationControlCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen collaboration-control object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(CollaborationControlCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(CollaborationControlCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                CollaborationControlCertificationViolation::RawCollaborationControlMaterialInExport,
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
        out.push_str("# M5 Collaboration-Control Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5CollaborationControlCertifiedProfile::ALL.len(),
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
pub fn current_m5_collaboration_control_surface_certification_export() -> Result<
    CollaborationControlProfileCertificationPacket,
    CollaborationControlCertificationArtifactError,
> {
    let packet: CollaborationControlProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-collaboration-control-surface-certification/support_export.json"
        )))
        .map_err(CollaborationControlCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CollaborationControlCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum CollaborationControlCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CollaborationControlCertificationViolation>),
}

impl fmt::Display for CollaborationControlCertificationArtifactError {
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

impl Error for CollaborationControlCertificationArtifactError {}

/// Validation failure for M05-1313 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollaborationControlCertificationViolation {
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
    NonLaneProfileClaimsCertifiedTruth { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawCollaborationControlMaterialInExport,
}

impl fmt::Display for CollaborationControlCertificationViolation {
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
                    "packet does not cite the canonical collaboration-control lifecycle matrix proof bundle"
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
                    "row {id} does not cite the one canonical collaboration-control lifecycle matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B155 hard invariant: acquiring terminal / debug control from presence, \
reconnect, or follow without an explicit grant; allowing more than one active driver on a sensitive surface; \
starting recording, transcript retention, or guest-scope widening silently; replaying prior terminal / debug \
input on join or restore; or revealing raw secrets, command text, variable bodies, or clipboard contents \
without an explicit consent posture and visible guardrail"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified collaboration-control record on a non-lane profile"
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
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified collaboration-control record, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 collaboration-control profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen collaboration-control object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawCollaborationControlMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for CollaborationControlCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&CollaborationControlAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != CollaborationControlAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the collaboration-control
/// generics the spec forbids collapsing distinct control-authority, active-driver, presenter-handoff,
/// consent-scope, retention-state, and restore-replay-safety truth into (whole-label matches so a full sentence
/// naming a concrete control authority, active driver, or consent scope is not flagged).
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
            | "collaboration control"
            | "collaboration-control"
            | "session"
            | "record"
            | "participant"
            | "presence"
            | "control"
            | "control authority"
            | "control grant"
            | "grant"
            | "active driver"
            | "driver"
            | "single driver"
            | "presenter"
            | "presenter token"
            | "handoff"
            | "moderation"
            | "consent"
            | "consent scope"
            | "consent envelope"
            | "join"
            | "recording"
            | "retention"
            | "retention state"
            | "sealed archive"
            | "archive"
            | "guest scope"
            | "route visibility"
            | "restore"
            | "session restore"
            | "replay"
            | "reattach"
            | "read only"
            | "read-only"
            | "recovery"
            | "checkpoint"
            | "provider"
            | "local"
            | "local only"
            | "evidence"
            | "export"
            | "export fallback"
            | "rollback"
            | "copy"
            | "fallback"
            | "drift"
            | "mismatch"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the collaboration-control lifecycle
/// matrix heuristic so the reused [`M5CollaborationControlDowngradeTrigger`] narrowings serialize cleanly — the
/// collaboration-control proof grammar carries only typed class tokens and opaque refs, never raw secret values or
/// endpoints.
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

/// Builds the canonical, checked-in M05-1313 certification packet. Certifies all eight claimed M5 collaboration-control
/// profiles: two deliver their claim (green) and six auto-narrow a not-current truth axis to a weaker
/// configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_collaboration_control_surface_certification_packet(
) -> CollaborationControlProfileCertificationPacket {
    CollaborationControlProfileCertificationPacket::new(
        CollaborationControlProfileCertificationPacketInput {
            packet_id: COLLABORATION_CONTROL_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-16T00:00:00Z".to_owned(),
            matrix_ref: COLLABORATION_CONTROL_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:collaboration-control-surface-certification:{id}"),
        COLLABORATION_CONTROL_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> CollaborationControlCertExportParity {
    CollaborationControlCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: CollaborationControlCertificationAxis) -> &'static str {
    match axis {
        CollaborationControlCertificationAxis::Visual => {
            "the control authority, single active driver, presenter handoff, join-time consent scope, recording / retention state, and replay-free restore safety are shown on-surface without a chrome-only affordance or a presence badge alone, and no presence still reads as terminal / debug control"
        }
        CollaborationControlCertificationAxis::Keyboard => {
            "the same control authority, active driver, consent scope, and bound request-control / grant / revoke / handoff / consent-review operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        CollaborationControlCertificationAxis::ScreenReader => {
            "the same collaboration-control truth is announced non-visually, never a chrome-only / presence-badge / unlabeled-control-only cue"
        }
        CollaborationControlCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the control authority, single active driver, presenter handoff, consent scope, or retention state"
        }
        CollaborationControlCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the control-authority badge, active-driver class, or presenter / consent / retention state"
        }
        CollaborationControlCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a control authority, active driver, presenter handoff, consent scope, or retention state"
        }
        CollaborationControlCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        CollaborationControlCertificationAxis::DegradedState => {
            "an unproven control authority, an unconfirmed active driver, a contested or silently transferred presenter handoff, an undisclosed or silently widened consent scope, a stale or silently broadened retention state, or an unproven replay-free restore honestly downgrades the CertifiedCollaborationControlTruth/ReviewableCollaborationControlRecord claim rather than reading as a fresh, provider-authoritative collaboration-control record"
        }
        CollaborationControlCertificationAxis::CollaborationControlTruth => {
            "the control authority, single active driver, presenter handoff, consent scope, recording / retention state, and replay-free restore safety stay explicit and never let presence, reconnect, or follow imply terminal / debug control, allow more than one active driver on a sensitive surface, silently transfer a presenter token, start recording / retention / guest-scope widening silently, replay prior terminal / debug input on join or restore, or reveal raw secrets, command text, variable bodies, or clipboard contents without an explicit consent posture and visible guardrail"
        }
    }
}

fn seed_certified(axis: CollaborationControlCertificationAxis) -> CollaborationControlAxisOutcome {
    CollaborationControlAxisOutcome {
        axis,
        state: CollaborationControlAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: CollaborationControlCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5CollaborationControlDowngradeTrigger,
) -> CollaborationControlAxisOutcome {
    CollaborationControlAxisOutcome {
        axis,
        state: CollaborationControlAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<CollaborationControlAxisOutcome> {
    CollaborationControlCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: CollaborationControlCertificationAxis,
    outcome: CollaborationControlAxisOutcome,
) -> Vec<CollaborationControlAxisOutcome> {
    CollaborationControlCertificationAxis::ALL
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
    profile: M5CollaborationControlCertifiedProfile,
    claimed_claim: M5CollaborationControlCertClaim,
    certified_claim: M5CollaborationControlCertClaim,
    consumed_families: &[M5CollaborationControlObject],
    axis_outcomes: Vec<CollaborationControlAxisOutcome>,
    claim_auto_narrow: Option<CollaborationControlClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> CollaborationControlProfileCertificationRow {
    let mut row = CollaborationControlProfileCertificationRow {
        record_kind: COLLABORATION_CONTROL_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: COLLABORATION_CONTROL_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: CollaborationControlCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: COLLABORATION_CONTROL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: CollaborationControlProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            COLLABORATION_CONTROL_CERT_MATRIX_REF.to_owned(),
            COLLABORATION_CONTROL_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: CollaborationControlCertificationAxis,
    from_claim: M5CollaborationControlCertClaim,
    to_claim: M5CollaborationControlCertClaim,
    label: &str,
) -> CollaborationControlClaimAutoNarrow {
    CollaborationControlClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<CollaborationControlProfileCertificationRow> {
    use CollaborationControlCertificationAxis as Ax;
    use M5CollaborationControlCertClaim::*;
    use M5CollaborationControlCertifiedProfile as P;
    use M5CollaborationControlDowngradeTrigger as Trig;
    use M5CollaborationControlObject::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-collaboration-control-lane",
            P::CertifiedCollaborationControlLane,
            CertifiedCollaborationControlTruth,
            CertifiedCollaborationControlTruth,
            &[SharedTerminalDebugView],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "control_authority_binding",
            ],
            &[
                "certified collaboration-control lane: the control authority, single active driver, presenter handoff, join-time consent scope, recording / retention state, and replay-free restore safety all join to one export-safe, provider-authoritative collaboration-control record, never presence that reads as terminal / debug control",
                "the certified shared terminal / debug view keeps stable operation IDs while its control authority, single active driver, presenter handoff, and consent / retention state bind to the one collaboration-control matrix across shared-terminal-debug-view / collaboration-join-review-sheet / control-grant-prompt / presenter-handoff-sheet / paste-secret-guard / collaboration-retention-sheet / support-export / help-docs surfaces, and no session reads as controlled in one surface and view-only in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered shared session",
                "collaboration-control-truth: a fully-certified collaboration-control lane with an explicit control grant and single active driver is the only profile that certifies a certified collaboration-control record",
            ],
        ),
        seed_row(
            "cert:reviewable-collaboration-control-record-structure",
            P::ReviewableCollaborationControlRecordStructure,
            ReviewableCollaborationControlRecord,
            ReviewableCollaborationControlRecord,
            &[ConsentEnvelope],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "consent_scope",
            ],
            &[
                "record-structure class: an export-safe consent-envelope / join-review sheet bound to one session and inspectable rather than a per-surface description copied by hand, with the recording, retention, guest scope, and route visibility kept bound to the session it came from",
                "the reviewable consent envelope keeps its recording state, retention mode, and guest scope inspectable rather than a presence-badge or chrome-only cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable collaboration-control record structure",
                "collaboration-control-truth: a reviewable consent envelope never certifies a fully-certified-lane claim and never stays green on presence-implied control or a missing control grant",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:unproven-control-authority-profile",
            P::UnprovenControlAuthorityProfile,
            ReviewableCollaborationControlRecord,
            ControlAuthorityUnverifiedProjection,
            &[SharedTerminalDebugView],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the shared terminal / debug view's control authority cannot be confirmed explicitly granted for this profile so a provider-authoritative collaboration-control record cannot be certified and the session stays view-only",
                    "The shared terminal / debug view's control authority can no longer be confirmed explicitly granted, so the ReviewableCollaborationControlRecord claim narrows to a control-authority-unverified projection and the lane discloses the last-known control authority rather than letting presence, reconnect, or follow acquire terminal / debug control",
                    Trig::ControlAuthorityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableCollaborationControlRecord,
                ControlAuthorityUnverifiedProjection,
                "The control authority is unverified for this shared session, so its last-known control authority is disclosed and no terminal / debug write is acquired from presence or follow",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "unproven-control-authority class: the shared terminal / debug view names its control authority, single active driver, and view-first default and marks the authority unverified rather than letting presence stand in for an explicit grant when the control authority is unconfirmed",
                "the unproven-control-authority surface keeps its shared session and last-known control authority legible while the authority is disclosed as unverified",
                "degraded-state: ReviewableCollaborationControlRecord narrows to a control-authority-unverified projection (auto-narrowed)",
                "collaboration-control-truth: a shared session never acquires terminal / debug control from presence, reconnect, or follow — its control authority is preserved and presence never reads as an explicit grant",
            ],
        ),
        seed_row(
            "cert:inferred-active-driver-profile",
            P::InferredActiveDriverProfile,
            ReviewableCollaborationControlRecord,
            ActiveDriverUnverifiedProjection,
            &[ControlGrant],
            seed_certified_except(
                Ax::CollaborationControlTruth,
                seed_narrowed(
                    Ax::CollaborationControlTruth,
                    "a control grant's single active driver cannot be confirmed for this profile so a provider-authoritative collaboration-control record cannot be certified and the driver stays inspect-only",
                    "A control grant's single active driver cannot be confirmed — a second participant risks driving the same sensitive surface — so the ReviewableCollaborationControlRecord claim narrows to an active-driver-unverified projection and the lane keeps the sole active driver and grant scope explicit rather than allowing more than one active driver on a sensitive surface",
                    Trig::ActiveDriverUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::CollaborationControlTruth,
                ReviewableCollaborationControlRecord,
                ActiveDriverUnverifiedProjection,
                "The active driver is not confirmed, so the sole active driver and grant scope stay explicit and no second participant drives the same sensitive surface",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "control-grant class: the control grant keeps its single active driver, granted scope, expiry, and revoke / reclaim path explicit and marks the driver unverified rather than admitting a driver the grant did not name",
                "the control-grant surface keeps its sole active driver legible while the driver is disclosed as unverified",
                "collaboration-control-truth: ReviewableCollaborationControlRecord narrows to an active-driver-unverified projection (auto-narrowed)",
                "collaboration-control-truth: more than one active driver is never allowed on a sensitive surface — the sole active driver and grant scope stay explicit",
            ],
        ),
        seed_row(
            "cert:silently-transferred-presenter-profile",
            P::SilentlyTransferredPresenterProfile,
            ReviewableCollaborationControlRecord,
            PresenterHandoffUnverifiedProjection,
            &[PresenterToken],
            seed_certified_except(
                Ax::Visual,
                seed_narrowed(
                    Ax::Visual,
                    "a presenter token's handoff cannot be confirmed reviewed for this profile so a provider-authoritative collaboration-control record cannot be certified and the handoff stays inspect-only",
                    "A presenter token's handoff cannot be confirmed reviewed — a moderation change risks transferring shell / debug control silently — so the ReviewableCollaborationControlRecord claim narrows to a presenter-handoff-unverified projection and the lane keeps the token holder, handoff target, and moderation scope explicit rather than silently transferring control between presenters",
                    Trig::ViewFirstDefaultUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Visual,
                ReviewableCollaborationControlRecord,
                PresenterHandoffUnverifiedProjection,
                "The presenter handoff is unverified, so the token holder, handoff target, and moderation scope stay explicit and no shell / debug control is transferred silently between presenters",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "presenter-token class: the presenter token keeps its holder, handoff target, and moderation scope explicit and marks the handoff unverified rather than applying a moderation change the participants did not review",
                "the presenter-token surface keeps its holder and handoff target legible while the handoff is disclosed as unverified",
                "visual: ReviewableCollaborationControlRecord narrows to a presenter-handoff-unverified projection (auto-narrowed)",
                "collaboration-control-truth: a presenter token is never silently transferred — the token holder, handoff target, and moderation scope stay visible and reviewable, and moderation never transfers shell / debug control",
            ],
        ),
        seed_row(
            "cert:undisclosed-consent-scope-profile",
            P::UndisclosedConsentScopeProfile,
            ReviewableCollaborationControlRecord,
            ConsentScopeUnverifiedProjection,
            &[ConsentEnvelope],
            seed_certified_except(
                Ax::HighZoomReflow,
                seed_narrowed(
                    Ax::HighZoomReflow,
                    "a consent envelope's join-time scope cannot be proven disclosed for this profile so a provider-authoritative collaboration-control record cannot be certified and the join stays a disclosed-but-blocked envelope",
                    "A consent envelope's join-time scope cannot be proven disclosed — the recording, retention, guest scope, or route visibility is undisclosed or was silently widened — so the ReviewableCollaborationControlRecord claim narrows to a consent-scope-unverified projection and the lane keeps the disclosed consent scope explicit rather than widening scope silently",
                    Trig::ConsentScopeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::HighZoomReflow,
                ReviewableCollaborationControlRecord,
                ConsentScopeUnverifiedProjection,
                "The consent scope is unproven, so the consent envelope stays labelled with its disclosed recording, retention, guest, and route-visibility scope and never widens scope silently",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "consent-envelope class: the consent envelope keeps its recording, retention, guest scope, and route visibility explicit and marks the scope blocked rather than joining when the join-time consent scope is undisclosed or was silently widened",
                "the consent-envelope surface keeps its disclosed scope legible while the join is disclosed as a blocked-until-consent envelope",
                "high-zoom-reflow: ReviewableCollaborationControlRecord narrows to a consent-scope-unverified projection (auto-narrowed)",
                "collaboration-control-truth: consent scope is never widened silently — the disclosed recording, retention, guest scope, and route visibility stay explicit and a widened scope never reads as the originally disclosed consent",
            ],
        ),
        seed_row(
            "cert:stale-retention-state-profile",
            P::StaleRetentionStateProfile,
            ReviewableCollaborationControlRecord,
            RetentionStateUnverifiedProjection,
            &[RetentionReview],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "a retention review's recording / retention state is stale for this profile so a provider-authoritative collaboration-control record cannot be certified and the retention stays inspect-only",
                    "A retention review's recording / retention state is stale — its retention mode drifted or a sealed archive was created against a superseded scope — so the ReviewableCollaborationControlRecord claim narrows to a retention-state-unverified projection and the lane keeps the last-known recording state, retention mode, and sealed-archive scope explicit rather than broadening retention silently",
                    Trig::RetentionStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableCollaborationControlRecord,
                RetentionStateUnverifiedProjection,
                "The retention state is unverified, so its last-known recording state, retention mode, and sealed-archive scope stay explicit and retention is never broadened silently",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "retention-review class: the retention review keeps its recording state, retention mode and duration, sealed-archive scope, and export / delete rights explicit and marks the state stale rather than broadening retention as if it were current disclosed truth",
                "the retention-review surface keeps its recording state and last-known retention mode legible while the state is disclosed as stale",
                "localization: ReviewableCollaborationControlRecord narrows to a retention-state-unverified projection (auto-narrowed)",
                "collaboration-control-truth: retention is never broadened silently — the last-known recording state, retention mode, and sealed-archive scope stay explicit and a widened retention reopens as honest disclosed state, never silent broadening",
            ],
        ),
        seed_row(
            "cert:unproven-replay-free-restore-profile",
            P::UnprovenReplayFreeRestoreProfile,
            ReviewableCollaborationControlRecord,
            RestoreReplaySafetyUnverifiedProjection,
            &[SessionRestoreView],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "a session-restore view's replay-free restore safety and recovery evidence is unproven for this profile so a provider-authoritative collaboration-control record cannot be certified and the restore stays blocked",
                    "A session-restore view's replay-free restore safety and recovery evidence is unproven — the read-only reattach, replayed-nothing evidence, or fresh-grant requirement for a reconnected sensitive session cannot be fully confirmed — so the ReviewableCollaborationControlRecord claim narrows to a restore-replay-safety-unverified projection and the lane keeps the read-only reattach, replayed-nothing evidence, and fresh-grant requirement explicit rather than replaying prior terminal / debug input on restore",
                    Trig::RestoreReplaySafetyUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableCollaborationControlRecord,
                RestoreReplaySafetyUnverifiedProjection,
                "The restore replay-safety is unproven, so the read-only reattach, replayed-nothing evidence, and fresh-grant requirement stay explicit and no prior terminal / debug input is replayed on restore",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "session-restore class: the session-restore view keeps its read-only reattach, replayed-nothing evidence, retained scope, and fresh-grant requirement explicit and marks the evidence unproven rather than reattaching a session whose replay-free safety and recovery are not fully proven",
                "the session-restore surface keeps its read-only reattach and replayed-nothing evidence legible non-visually while the evidence is disclosed as unproven",
                "screen-reader: ReviewableCollaborationControlRecord narrows to a restore-replay-safety-unverified projection (auto-narrowed)",
                "collaboration-control-truth: prior terminal / debug input is never replayed on join or restore — the read-only reattach, replayed-nothing evidence, and fresh-grant requirement stay explicit and survive export and reconnect",
            ],
        ),
    ]
}
