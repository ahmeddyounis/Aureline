//! M05-1323 closing B156 surface certification over the frozen M5 collaboration-state shared-object authority,
//! anchor-drift, convergence, and session-archive matrix — the CRDT-backed shared text, sampled presence /
//! cursors / selections, server-ordered comments / annotations / review pins, presenter / follow state,
//! higher-risk control plane, and sealed session archive that a desktop, browser-companion, review, incident /
//! support, or audit / export consumer must treat as first-class collaboration-state objects with a declared
//! authority model rather than an implicit convergence guarantee.
//!
//! Where the freeze matrix ([`crate::m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`]) defines the six
//! governed collaboration-state shared-object classes, the M05-1315..1322 implement lanes resolve each replica
//! descriptor, shared-object record, unsent-local-edit preservation, anchor-history / rebind review, compaction
//! manifest / archive finalization, degradation ladder, provenance / freshness label, share-eligibility, and
//! headless-inspect / support-bundle parity registry; this closing capstone *certifies* that the shared-object
//! authority, convergence, anchor-drift, compaction, and degradation truth holds on every claimed M5 desktop,
//! companion, review, support, incident, and audit / export surface — the authority model, local-truth
//! preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and session
//! provenance / freshness — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a shared-session owner, a review / follow consumer, or a support /
//! export consumer reads a collaboration-state object through (a fully-certified collaboration-state lane; a
//! reviewable collaboration-state record structure; an unproven-authority-model profile; an
//! unconfirmed-convergence-state profile; an unpreserved-local-truth profile; an unresolved-anchor-drift profile;
//! an undisclosed-export-posture profile; and an unproven-provenance-freshness profile), not on the underlying
//! object class or implement lane.
//! Each [`CollaborationStateProfileCertificationRow`] certifies one profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! collaboration-state-truth behavior — and either passes (green), auto-narrows its collaboration-state claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh
//! certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedCollaborationStateTruth` / `ReviewableCollaborationStateRecord` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Only a fully-certified collaboration-state
//! lane — one whose authority model, local-truth preservation, merge / drift semantics, downgrade behavior,
//! anchor-drift history, export posture, and session provenance / freshness all converge on one export-safe,
//! provider-authoritative, internally consistent collaboration-state record — may certify a
//! `CertifiedCollaborationStateTruth` claim; a reviewable, unproven-authority-model, unconfirmed-convergence-state,
//! unpreserved-local-truth, unresolved-anchor-drift, undisclosed-export-posture, or unproven-provenance-freshness
//! profile that keeps a certified claim is over-reaching and blocks. The always-on CLI/export axis must always
//! stay certified so support and automation can reconstruct the authority model, convergence state, local-truth
//! disposition, anchor-drift history, export posture, and provenance / freshness from the same collaboration-state
//! proof the operator saw.
//!
//! The B156 hard invariants are enforced per row: no profile may let a collaboration replica overwrite the
//! canonical local buffer, VFS, or Git truth; discard unsent local edits on a permission downgrade, relay
//! failure, or leave-session flow; silently rebind a comment, annotation, or review pin without append-only
//! drift history; collapse a convergence-degraded, awareness-degraded, or anchor-unresolved state into a generic
//! stale badge; or export an op-log, snapshot, or archive without policy-labeled redaction and actor lineage. A
//! profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical collaboration-state convergence matrix proof bundle
//! ([`COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen collaboration-state convergence matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets,
//! bearer tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/collaboration/m5-collaboration-state-surface-certification.schema.json`](../../../../schemas/collaboration/m5-collaboration-state-surface-certification.schema.json).
//! The contract doc is
//! [`docs/collaboration/m5-collaboration-state-surface-certification.md`](../../../../docs/collaboration/m5-collaboration-state-surface-certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix as matrix;
use matrix::{M5CollaborationStateDowngradeTrigger, M5CollaborationStateObject};

/// Schema version stamped on the M05-1323 certification packet.
pub const COLLABORATION_STATE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CollaborationStateProfileCertificationPacket`].
pub const COLLABORATION_STATE_CERT_RECORD_KIND: &str =
    "m5_collaboration_state_surface_certification_packet";

/// Stable record-kind tag carried by each [`CollaborationStateProfileCertificationRow`].
pub const COLLABORATION_STATE_CERT_ROW_RECORD_KIND: &str =
    "m5_collaboration_state_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const COLLABORATION_STATE_CERT_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-state-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const COLLABORATION_STATE_CERT_DOC_REF: &str =
    "docs/collaboration/m5-collaboration-state-surface-certification.md";

/// Repo-relative path of the frozen collaboration-state lifecycle matrix schema the certified profiles render.
pub const COLLABORATION_STATE_CERT_MATRIX_REF: &str =
    matrix::M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF;

/// The one canonical collaboration-state lifecycle matrix proof bundle every certified profile cites as its
/// first-resolved collaboration-state truth. All eight profiles point back to it rather than cloning per-profile
/// evidence.
pub const COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_COLLABORATION_STATE_ARTIFACT_REF;

/// The collaboration-state-health dashboard the release surfaces consume. Recorded as a supporting evidence ref on
/// every row so the certification's collaboration-state truth ties back to the same dashboard consumers read.
pub const COLLABORATION_STATE_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_COLLABORATION_STATE_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const COLLABORATION_STATE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-state-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COLLABORATION_STATE_CERT_CSV_REF: &str =
    "artifacts/release/m5-collaboration-state-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COLLABORATION_STATE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-collaboration-state-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const COLLABORATION_STATE_CERT_FIXTURE_DIR: &str =
    "fixtures/collaboration/m5-collaboration-state-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const COLLABORATION_STATE_CERT_PACKET_ID: &str =
    "m5-collaboration-state-surface-certification:stable:0001";

/// The eight claimed M5 collaboration-state consumer profiles this capstone certifies. Keyed on the profile a
/// shared-session owner, a review / follow consumer, or a support / export consumer reads a collaboration-state
/// object through — a fully-certified collaboration-state lane, a reviewable collaboration-state record
/// structure, an unproven-authority-model profile, an unconfirmed-convergence-state profile, an
/// unpreserved-local-truth profile, an unresolved-anchor-drift profile, an undisclosed-export-posture profile,
/// and an unproven-provenance-freshness profile — not on the reusable object class it renders. Only a
/// fully-certified collaboration-state lane profile may certify a certified collaboration-state claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateCertifiedProfile {
    /// A fully-certified collaboration-state lane — a shared session whose authority model, local-truth
    /// preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and
    /// session provenance / freshness all converge on one export-safe, provider-authoritative, internally
    /// consistent collaboration-state record that stays identical across every desktop, companion, review,
    /// support, incident, and audit / export consumer, certifying the collaboration-state claim exactly right now.
    CertifiedCollaborationStateLane,
    /// A reviewable collaboration-state record structure: a self-sufficient, inspectable sealed-session-archive /
    /// compaction-manifest record (a session-bound record an operator can review), never itself a fully-certified
    /// collaboration-state lane.
    ReviewableCollaborationStateRecordStructure,
    /// A CRDT-backed shared-text replica whose authority model can no longer be confirmed disclosed; the claim
    /// narrows to an authority-model-unverified projection that discloses the last-known authority model and never
    /// lets a replica imply convergence or replace the canonical local buffer, VFS, or Git truth.
    UnprovenAuthorityModelProfile,
    /// A presenter / follow convergence lane whose convergence state cannot be confirmed; the claim narrows to a
    /// convergence-state-unverified projection that keeps convergence-degraded distinct from awareness-degraded
    /// and never collapses either into a generic stale badge.
    UnconfirmedConvergenceStateProfile,
    /// A higher-risk control-plane lane whose local-truth preservation on downgrade cannot be confirmed; the claim
    /// narrows to a local-truth-preservation-unverified projection that keeps unsent local edits preserved first
    /// and never discards them on a permission downgrade, relay failure, or leave-session flow.
    UnpreservedLocalTruthProfile,
    /// A server-ordered comment / annotation / review-pin lane whose anchor-drift history cannot be confirmed
    /// append-only; the claim narrows to an anchor-drift-unverified projection that keeps the drift history
    /// reviewable and never silently rebinds a comment, annotation, or review pin.
    UnresolvedAnchorDriftProfile,
    /// A sealed session-archive lane whose export posture cannot be proven policy-labeled; the claim narrows to an
    /// export-posture-unverified projection that keeps the redaction label and actor lineage explicit and never
    /// exports an op-log, snapshot, or archive without both.
    UndisclosedExportPostureProfile,
    /// A sampled presence / cursors / selections lane whose session provenance / freshness is unproven; the claim
    /// narrows to a provenance-freshness-unverified projection that keeps the provenance and freshness explicit and
    /// never lets stale collaboration state read as current canonical truth to search, AI, review, docs, or
    /// support.
    UnprovenProvenanceFreshnessProfile,
}

impl M5CollaborationStateCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5CollaborationStateCertifiedProfile; 8] = [
        M5CollaborationStateCertifiedProfile::CertifiedCollaborationStateLane,
        M5CollaborationStateCertifiedProfile::ReviewableCollaborationStateRecordStructure,
        M5CollaborationStateCertifiedProfile::UnprovenAuthorityModelProfile,
        M5CollaborationStateCertifiedProfile::UnconfirmedConvergenceStateProfile,
        M5CollaborationStateCertifiedProfile::UnpreservedLocalTruthProfile,
        M5CollaborationStateCertifiedProfile::UnresolvedAnchorDriftProfile,
        M5CollaborationStateCertifiedProfile::UndisclosedExportPostureProfile,
        M5CollaborationStateCertifiedProfile::UnprovenProvenanceFreshnessProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedCollaborationStateLane => "certified_collaboration_state_lane",
            Self::ReviewableCollaborationStateRecordStructure => {
                "reviewable_collaboration_state_record_structure"
            }
            Self::UnprovenAuthorityModelProfile => "unproven_authority_model_profile",
            Self::UnconfirmedConvergenceStateProfile => "unconfirmed_convergence_state_profile",
            Self::UnpreservedLocalTruthProfile => "unpreserved_local_truth_profile",
            Self::UnresolvedAnchorDriftProfile => "unresolved_anchor_drift_profile",
            Self::UndisclosedExportPostureProfile => "undisclosed_export_posture_profile",
            Self::UnprovenProvenanceFreshnessProfile => "unproven_provenance_freshness_profile",
        }
    }

    /// True only for the fully-certified collaboration-state lane profile. A certified collaboration-state claim
    /// may be certified on this profile alone; every other profile is at most a reviewable collaboration-state
    /// record structure or a narrowed projection.
    pub const fn is_certified_collaboration_state_lane(self) -> bool {
        matches!(self, Self::CertifiedCollaborationStateLane)
    }
}

/// The claim ladder a certified collaboration-state profile asserts and is certified down to. Minted locally for
/// this capstone: the strongest claim is a fully certified collaboration-state record; each weaker tier is a
/// disclosed projection that keeps the last-known authority-model, convergence-state, local-truth-preservation,
/// anchor-drift, export-posture, or provenance-freshness posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateCertClaim {
    /// Certified collaboration-state truth: a fully-certified collaboration-state object whose authority model,
    /// local-truth preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export
    /// posture, and session provenance / freshness all join to one export-safe, provider-authoritative,
    /// internally consistent record — the strongest claim, the collaboration state Aureline can present as
    /// authority-declared and convergence-honest across every consumer.
    CertifiedCollaborationStateTruth,
    /// Reviewable collaboration-state record: a self-sufficient, inspectable session-bound record (a
    /// sealed-session-archive / compaction-manifest record an operator can inspect) that is not itself a
    /// fully-certified collaboration-state lane.
    ReviewableCollaborationStateRecord,
    /// Authority-model-unverified projection: a shared object's authority model cannot be confirmed disclosed; the
    /// lane stays an authority-model-unverified projection that discloses the last-known authority model, never
    /// letting a replica imply convergence or replace the canonical local buffer, VFS, or Git truth.
    AuthorityModelUnverifiedProjection,
    /// Convergence-state-unverified projection: a presenter / follow convergence state cannot be confirmed; the
    /// lane stays a convergence-state-unverified projection that keeps convergence-degraded distinct from
    /// awareness-degraded, never collapsing either into a generic stale badge.
    ConvergenceStateUnverifiedProjection,
    /// Local-truth-preservation-unverified projection: a downgrade's local-unsent preservation cannot be
    /// confirmed; the lane stays a local-truth-preservation-unverified projection that keeps unsent local edits
    /// preserved first, never discarding them on a permission downgrade, relay failure, or leave-session flow.
    LocalTruthPreservationUnverifiedProjection,
    /// Anchor-drift-unverified projection: a comment / annotation / review-pin anchor-drift history cannot be
    /// confirmed append-only; the lane stays an anchor-drift-unverified projection that keeps the drift history
    /// reviewable, never silently rebinding a comment, annotation, or review pin.
    AnchorDriftUnverifiedProjection,
    /// Export-posture-unverified projection: a sealed archive's export posture cannot be proven policy-labeled; the
    /// lane stays an export-posture-unverified projection that keeps the redaction label and actor lineage
    /// explicit, never exporting an op-log, snapshot, or archive without both.
    ExportPostureUnverifiedProjection,
    /// Provenance-freshness-unverified projection: a session's provenance / freshness is unproven; the lane stays a
    /// provenance-freshness-unverified projection that keeps the provenance and freshness explicit, never letting
    /// stale collaboration state read as current canonical truth.
    ProvenanceFreshnessUnverifiedProjection,
}

impl M5CollaborationStateCertClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::CertifiedCollaborationStateTruth,
        Self::ReviewableCollaborationStateRecord,
        Self::AuthorityModelUnverifiedProjection,
        Self::ConvergenceStateUnverifiedProjection,
        Self::LocalTruthPreservationUnverifiedProjection,
        Self::AnchorDriftUnverifiedProjection,
        Self::ExportPostureUnverifiedProjection,
        Self::ProvenanceFreshnessUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedCollaborationStateTruth => 7,
            Self::ReviewableCollaborationStateRecord => 6,
            Self::AuthorityModelUnverifiedProjection => 5,
            Self::ConvergenceStateUnverifiedProjection => 4,
            Self::LocalTruthPreservationUnverifiedProjection => 3,
            Self::AnchorDriftUnverifiedProjection => 2,
            Self::ExportPostureUnverifiedProjection => 1,
            Self::ProvenanceFreshnessUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-certified, certified collaboration-state record.
    pub const fn asserts_certified_collaboration_state_truth(self) -> bool {
        matches!(self, Self::CertifiedCollaborationStateTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedCollaborationStateTruth | Self::ReviewableCollaborationStateRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedCollaborationStateTruth => "certified_collaboration_state_truth",
            Self::ReviewableCollaborationStateRecord => "reviewable_collaboration_state_record",
            Self::AuthorityModelUnverifiedProjection => "authority_model_unverified_projection",
            Self::ConvergenceStateUnverifiedProjection => "convergence_state_unverified_projection",
            Self::LocalTruthPreservationUnverifiedProjection => {
                "local_truth_preservation_unverified_projection"
            }
            Self::AnchorDriftUnverifiedProjection => "anchor_drift_unverified_projection",
            Self::ExportPostureUnverifiedProjection => "export_posture_unverified_projection",
            Self::ProvenanceFreshnessUnverifiedProjection => {
                "provenance_freshness_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and collaboration-state-truth behavior. The CLI/export axis is always-on and must
/// stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationStateCertificationAxis {
    /// Visual parity: the authority model, convergence state, local-truth disposition, merge / drift summary,
    /// export posture, and provenance / freshness are shown on the primary surface without relying on a
    /// chrome-only affordance or a presence badge alone, and no presence still reads as convergence.
    Visual,
    /// Keyboard-reach parity: the same collaboration-state truth and its bound review / rebind / reconcile /
    /// export operations are reachable and operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a chrome-only affordance, a
    /// presence badge, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the authority
    /// model, convergence state, local-truth disposition, anchor-drift history, or export posture.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// authority-model badge, convergence-state class, or anchor-drift / export / provenance state.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling an
    /// authority model, convergence state, anchor-drift history, export posture, or provenance / freshness when a
    /// locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text / JSON / Markdown
    /// for support and automation.
    CliExport,
    /// Degraded-state parity: an unproven authority model, an unconfirmed convergence state, an unpreserved local
    /// truth on downgrade, an unresolved anchor drift, an undisclosed export posture, or an unproven provenance /
    /// freshness honestly downgrades a `CertifiedCollaborationStateTruth` / `ReviewableCollaborationStateRecord`
    /// claim rather than reading as a fresh, provider-authoritative collaboration-state record.
    DegradedState,
    /// Collaboration-state-truth parity: the authority model, local-truth preservation, merge / drift semantics,
    /// downgrade behavior, anchor-drift history, export posture, and session provenance / freshness stay explicit
    /// and never let a replica overwrite the canonical local buffer, VFS, or Git truth; discard unsent local edits
    /// on a permission downgrade, relay failure, or leave-session flow; silently rebind a comment, annotation, or
    /// review pin without drift history; collapse a convergence-degraded, awareness-degraded, or anchor-unresolved
    /// state into a generic stale badge; or export an op-log, snapshot, or archive without policy-labeled
    /// redaction and actor lineage.
    CollaborationStateTruth,
}

impl CollaborationStateCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [CollaborationStateCertificationAxis; 9] = [
        CollaborationStateCertificationAxis::Visual,
        CollaborationStateCertificationAxis::Keyboard,
        CollaborationStateCertificationAxis::ScreenReader,
        CollaborationStateCertificationAxis::HighZoomReflow,
        CollaborationStateCertificationAxis::HighContrast,
        CollaborationStateCertificationAxis::Localization,
        CollaborationStateCertificationAxis::CliExport,
        CollaborationStateCertificationAxis::DegradedState,
        CollaborationStateCertificationAxis::CollaborationStateTruth,
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
            Self::CollaborationStateTruth => "collaboration_state_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationStateAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl CollaborationStateAxisCertificationState {
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
pub enum CollaborationStateProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-lane collaboration-state profile claims a certified collaboration-state record, or the narrowing is inconsistent.
    Red,
}

impl CollaborationStateProfileClaimStatus {
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

/// The five B156 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationStateCertGuardrails {
    /// True if a collaboration replica overwrote the canonical local buffer, VFS, or Git truth. Must be false.
    pub replica_overwrote_local_canonical_buffer_vfs_or_git_truth: bool,
    /// True if unsent local edits were discarded on a permission downgrade, relay failure, or leave-session flow.
    /// Must be false.
    pub discarded_unsent_local_edits_on_downgrade_relay_or_leave: bool,
    /// True if a comment, annotation, or review pin was silently rebound without append-only drift history. Must
    /// be false.
    pub silently_rebound_comments_annotations_or_review_pins_without_drift_history: bool,
    /// True if a convergence-degraded, awareness-degraded, or anchor-unresolved state was collapsed into a generic
    /// stale badge. Must be false.
    pub collapsed_convergence_awareness_or_anchor_unresolved_into_generic_stale: bool,
    /// True if an op-log, snapshot, or archive was exported without policy-labeled redaction and actor lineage.
    /// Must be false.
    pub exported_op_logs_snapshots_or_archives_without_redaction_or_actor_lineage: bool,
}

impl CollaborationStateCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        replica_overwrote_local_canonical_buffer_vfs_or_git_truth: false,
        discarded_unsent_local_edits_on_downgrade_relay_or_leave: false,
        silently_rebound_comments_annotations_or_review_pins_without_drift_history: false,
        collapsed_convergence_awareness_or_anchor_unresolved_into_generic_stale: false,
        exported_op_logs_snapshots_or_archives_without_redaction_or_actor_lineage: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.replica_overwrote_local_canonical_buffer_vfs_or_git_truth
            && !self.discarded_unsent_local_edits_on_downgrade_relay_or_leave
            && !self.silently_rebound_comments_annotations_or_review_pins_without_drift_history
            && !self.collapsed_convergence_awareness_or_anchor_unresolved_into_generic_stale
            && !self.exported_op_logs_snapshots_or_archives_without_redaction_or_actor_lineage
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationStateCertExportParity {
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

impl CollaborationStateCertExportParity {
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
pub struct CollaborationStateAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: CollaborationStateCertificationAxis,
    /// The certification state of the axis.
    pub state: CollaborationStateAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5CollaborationStateDowngradeTrigger>,
}

impl CollaborationStateAxisOutcome {
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
            CollaborationStateAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            CollaborationStateAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            CollaborationStateAxisCertificationState::UndisclosedDrift => {
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
pub struct CollaborationStateClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: CollaborationStateCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5CollaborationStateCertClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5CollaborationStateCertClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 collaboration-state object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationStateProfileCertificationRow {
    /// Record kind; must equal [`COLLABORATION_STATE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COLLABORATION_STATE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5CollaborationStateCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5CollaborationStateCertClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5CollaborationStateCertClaim,
    /// The frozen collaboration-state object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5CollaborationStateObject>,
    /// One outcome per [`CollaborationStateCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<CollaborationStateAxisOutcome>,
    /// The B156 hard invariants; all must hold.
    pub guardrails: CollaborationStateCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<CollaborationStateClaimAutoNarrow>,
    /// The one canonical collaboration-state lifecycle matrix proof bundle this profile cites. Must equal
    /// [`COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: CollaborationStateProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: CollaborationStateCertExportParity,
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

impl CollaborationStateProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: CollaborationStateCertificationAxis,
    ) -> Option<&CollaborationStateAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<CollaborationStateCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && CollaborationStateCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(CollaborationStateAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<CollaborationStateCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == CollaborationStateAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-certified collaboration-state lane
    /// profile may certify a certified collaboration-state record, every hard invariant must hold, CLI/export parity must
    /// always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> CollaborationStateProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return CollaborationStateProfileClaimStatus::Red;
        }

        // Every B156 hard invariant must hold.
        if !self.guardrails.all_held() {
            return CollaborationStateProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return CollaborationStateProfileClaimStatus::Red;
        }

        // Only a fully-certified collaboration-state lane profile may certify a certified collaboration-state record.
        if self
            .certified_claim
            .asserts_certified_collaboration_state_truth()
            && !self.profile.is_certified_collaboration_state_lane()
        {
            return CollaborationStateProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(CollaborationStateCertificationAxis::CliExport) {
            Some(o) if o.state == CollaborationStateAxisCertificationState::Certified => {}
            _ => return CollaborationStateProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == CollaborationStateAxisCertificationState::UndisclosedDrift)
        {
            return CollaborationStateProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return CollaborationStateProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return CollaborationStateProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return CollaborationStateProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return CollaborationStateProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return CollaborationStateProfileClaimStatus::Red;
        }

        CollaborationStateProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COLLABORATION_STATE_CERT_ROW_RECORD_KIND
            && self.schema_version == COLLABORATION_STATE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1323 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationStateProfileCertificationSummary {
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

/// Constructor input for [`CollaborationStateProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollaborationStateProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<CollaborationStateProfileCertificationRow>,
}

/// Checked-in M05-1323 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationStateProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<CollaborationStateProfileCertificationRow>,
    pub summary: CollaborationStateProfileCertificationSummary,
}

impl CollaborationStateProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CollaborationStateProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COLLABORATION_STATE_CERT_SCHEMA_VERSION,
            record_kind: COLLABORATION_STATE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: CollaborationStateProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5CollaborationStateCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Collaboration-control object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5CollaborationStateObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5CollaborationStateCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen collaboration-state object class is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5CollaborationStateObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(CollaborationStateCertificationAxis::CliExport)
                .is_some_and(|o| o.state == CollaborationStateAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CollaborationStateProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationStateProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationStateProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CollaborationStateProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(CollaborationStateProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        CollaborationStateProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(CollaborationStateProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CollaborationStateCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COLLABORATION_STATE_CERT_SCHEMA_VERSION {
            violations.push(CollaborationStateCertificationViolation::SchemaVersion {
                expected: COLLABORATION_STATE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COLLABORATION_STATE_CERT_RECORD_KIND {
            violations.push(CollaborationStateCertificationViolation::RecordKind {
                expected: COLLABORATION_STATE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CollaborationStateCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(CollaborationStateCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CollaborationStateCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(CollaborationStateCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    CollaborationStateCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    CollaborationStateCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    CollaborationStateCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B156 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    CollaborationStateCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a fully-certified collaboration-state lane profile may certify a certified collaboration-state record.
            if row
                .certified_claim
                .asserts_certified_collaboration_state_truth()
                && !row.profile.is_certified_collaboration_state_lane()
            {
                violations.push(
                    CollaborationStateCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(CollaborationStateCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    CollaborationStateCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    CollaborationStateCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    CollaborationStateCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == CollaborationStateProfileClaimStatus::Red {
                violations.push(CollaborationStateCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(CollaborationStateCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen collaboration-state object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(CollaborationStateCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(CollaborationStateCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                CollaborationStateCertificationViolation::RawCollaborationStateMaterialInExport,
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
        out.push_str("# M5 Collaboration-State Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5CollaborationStateCertifiedProfile::ALL.len(),
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
pub fn current_m5_collaboration_state_surface_certification_export() -> Result<
    CollaborationStateProfileCertificationPacket,
    CollaborationStateCertificationArtifactError,
> {
    let packet: CollaborationStateProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-collaboration-state-surface-certification/support_export.json"
        )))
        .map_err(CollaborationStateCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CollaborationStateCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum CollaborationStateCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CollaborationStateCertificationViolation>),
}

impl fmt::Display for CollaborationStateCertificationArtifactError {
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

impl Error for CollaborationStateCertificationArtifactError {}

/// Validation failure for M05-1323 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollaborationStateCertificationViolation {
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
    RawCollaborationStateMaterialInExport,
}

impl fmt::Display for CollaborationStateCertificationViolation {
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
                    "packet does not cite the canonical collaboration-state lifecycle matrix proof bundle"
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
                    "row {id} does not cite the one canonical collaboration-state lifecycle matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B156 hard invariant: a collaboration replica overwriting the canonical \
local buffer, VFS, or Git truth; discarding unsent local edits on a permission downgrade, relay failure, or \
leave-session flow; silently rebinding a comment, annotation, or review pin without append-only drift history; \
collapsing a convergence-degraded, awareness-degraded, or anchor-unresolved state into a generic stale badge; or \
exporting an op-log, snapshot, or archive without policy-labeled redaction and actor lineage"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified collaboration-state record on a non-lane profile"
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
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified collaboration-state record, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 collaboration-state profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen collaboration-state object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawCollaborationStateMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for CollaborationStateCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&CollaborationStateAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != CollaborationStateAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the collaboration-state
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
            | "collaboration-state"
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the collaboration-state lifecycle
/// matrix heuristic so the reused [`M5CollaborationStateDowngradeTrigger`] narrowings serialize cleanly — the
/// collaboration-state proof grammar carries only typed class tokens and opaque refs, never raw secret values or
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

/// Builds the canonical, checked-in M05-1323 certification packet. Certifies all eight claimed M5 collaboration-state
/// profiles: two deliver their claim (green) and six auto-narrow a not-current truth axis to a weaker
/// configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_collaboration_state_surface_certification_packet(
) -> CollaborationStateProfileCertificationPacket {
    CollaborationStateProfileCertificationPacket::new(
        CollaborationStateProfileCertificationPacketInput {
            packet_id: COLLABORATION_STATE_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-16T00:00:00Z".to_owned(),
            matrix_ref: COLLABORATION_STATE_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:collaboration-state-surface-certification:{id}"),
        COLLABORATION_STATE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> CollaborationStateCertExportParity {
    CollaborationStateCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: CollaborationStateCertificationAxis) -> &'static str {
    match axis {
        CollaborationStateCertificationAxis::Visual => {
            "the authority model, convergence state, local-truth disposition, merge / drift summary, export posture, and provenance / freshness are shown on-surface without a chrome-only affordance or a presence badge alone, and no presence still reads as convergence"
        }
        CollaborationStateCertificationAxis::Keyboard => {
            "the same authority model, convergence state, anchor-drift history, and bound review / rebind / reconcile / export operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        CollaborationStateCertificationAxis::ScreenReader => {
            "the same collaboration-state truth is announced non-visually, never a chrome-only / presence-badge / unlabeled-control-only cue"
        }
        CollaborationStateCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the authority model, convergence state, local-truth disposition, anchor-drift history, or export posture"
        }
        CollaborationStateCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the authority-model badge, convergence-state class, or anchor-drift / export / provenance state"
        }
        CollaborationStateCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling an authority model, convergence state, anchor-drift history, export posture, or provenance / freshness"
        }
        CollaborationStateCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        CollaborationStateCertificationAxis::DegradedState => {
            "an unproven authority model, an unconfirmed convergence state, an unpreserved local truth on downgrade, an unresolved anchor drift, an undisclosed export posture, or an unproven provenance / freshness honestly downgrades the CertifiedCollaborationStateTruth/ReviewableCollaborationStateRecord claim rather than reading as a fresh, provider-authoritative collaboration-state record"
        }
        CollaborationStateCertificationAxis::CollaborationStateTruth => {
            "the authority model, local-truth preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and session provenance / freshness stay explicit and never let a replica overwrite the canonical local buffer, VFS, or Git truth, discard unsent local edits on a permission downgrade, relay failure, or leave-session flow, silently rebind a comment, annotation, or review pin without drift history, collapse a convergence-degraded, awareness-degraded, or anchor-unresolved state into a generic stale badge, or export an op-log, snapshot, or archive without policy-labeled redaction and actor lineage"
        }
    }
}

fn seed_certified(axis: CollaborationStateCertificationAxis) -> CollaborationStateAxisOutcome {
    CollaborationStateAxisOutcome {
        axis,
        state: CollaborationStateAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: CollaborationStateCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5CollaborationStateDowngradeTrigger,
) -> CollaborationStateAxisOutcome {
    CollaborationStateAxisOutcome {
        axis,
        state: CollaborationStateAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<CollaborationStateAxisOutcome> {
    CollaborationStateCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: CollaborationStateCertificationAxis,
    outcome: CollaborationStateAxisOutcome,
) -> Vec<CollaborationStateAxisOutcome> {
    CollaborationStateCertificationAxis::ALL
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
    profile: M5CollaborationStateCertifiedProfile,
    claimed_claim: M5CollaborationStateCertClaim,
    certified_claim: M5CollaborationStateCertClaim,
    consumed_families: &[M5CollaborationStateObject],
    axis_outcomes: Vec<CollaborationStateAxisOutcome>,
    claim_auto_narrow: Option<CollaborationStateClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> CollaborationStateProfileCertificationRow {
    let mut row = CollaborationStateProfileCertificationRow {
        record_kind: COLLABORATION_STATE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: COLLABORATION_STATE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: CollaborationStateCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: COLLABORATION_STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: CollaborationStateProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            COLLABORATION_STATE_CERT_MATRIX_REF.to_owned(),
            COLLABORATION_STATE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: CollaborationStateCertificationAxis,
    from_claim: M5CollaborationStateCertClaim,
    to_claim: M5CollaborationStateCertClaim,
    label: &str,
) -> CollaborationStateClaimAutoNarrow {
    CollaborationStateClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<CollaborationStateProfileCertificationRow> {
    use CollaborationStateCertificationAxis as Ax;
    use M5CollaborationStateCertClaim::*;
    use M5CollaborationStateCertifiedProfile as P;
    use M5CollaborationStateDowngradeTrigger as Trig;
    use M5CollaborationStateObject::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-collaboration-state-lane",
            P::CertifiedCollaborationStateLane,
            CertifiedCollaborationStateTruth,
            CertifiedCollaborationStateTruth,
            &[CrdtBackedSharedText],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "authority_model_binding",
            ],
            &[
                "certified collaboration-state lane: the authority model, local-truth preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and session provenance / freshness all join to one export-safe, provider-authoritative collaboration-state record, never a replica that reads as canonical local buffer, VFS, or Git truth",
                "the certified CRDT-backed shared-text replica keeps stable operation IDs while its authority model, convergence state, local-truth disposition, anchor-drift history, and export posture bind to the one collaboration-state matrix across shared-editor-replica-view / presence-cursor-layer / comment-annotation-review-pin-layer / collaboration-degradation-banner / support-export / help-docs surfaces, and no session reads as converged in one surface and degraded in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered shared session",
                "collaboration-state-truth: a fully-certified collaboration-state lane with a disclosed authority model and preserved local truth is the only profile that certifies a certified collaboration-state record",
            ],
        ),
        seed_row(
            "cert:reviewable-collaboration-state-record-structure",
            P::ReviewableCollaborationStateRecordStructure,
            ReviewableCollaborationStateRecord,
            ReviewableCollaborationStateRecord,
            &[SealedSessionArchive],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "export_posture",
            ],
            &[
                "record-structure class: an export-safe sealed-session-archive / compaction-manifest record bound to one session and inspectable rather than a per-surface description copied by hand, with the compaction lineage, actor lineage, and policy-labeled redaction kept bound to the session it came from",
                "the reviewable session archive keeps its compaction lineage, retained-object refs, and redaction label inspectable rather than a presence-badge or chrome-only cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable collaboration-state record structure",
                "collaboration-state-truth: a reviewable session archive never certifies a fully-certified-lane claim and never stays green on a replica-implied convergence or a missing authority model",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:unproven-authority-model-profile",
            P::UnprovenAuthorityModelProfile,
            ReviewableCollaborationStateRecord,
            AuthorityModelUnverifiedProjection,
            &[CrdtBackedSharedText],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the CRDT-backed shared-text replica's authority model cannot be confirmed disclosed for this profile so a provider-authoritative collaboration-state record cannot be certified and the replica stays local-truth-first",
                    "The CRDT-backed shared-text replica's authority model can no longer be confirmed disclosed, so the ReviewableCollaborationStateRecord claim narrows to an authority-model-unverified projection and the lane discloses the last-known authority model rather than letting the replica imply convergence or replace the canonical local buffer, VFS, or Git truth",
                    Trig::AuthorityModelUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableCollaborationStateRecord,
                AuthorityModelUnverifiedProjection,
                "The authority model is unverified for this shared object, so its last-known authority model is disclosed and no replica overwrites the canonical local buffer, VFS, or Git truth",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "unproven-authority-model class: the CRDT-backed shared-text replica names its authority model, local-truth-first default, and merge / drift semantics and marks the authority unverified rather than letting a replica stand in for canonical truth when the authority model is unconfirmed",
                "the unproven-authority-model surface keeps its shared session and last-known authority model legible while the authority is disclosed as unverified",
                "degraded-state: ReviewableCollaborationStateRecord narrows to an authority-model-unverified projection (auto-narrowed)",
                "collaboration-state-truth: a replica never overwrites the canonical local buffer, VFS, or Git truth — its authority model is preserved and a replica never reads as canonical truth",
            ],
        ),
        seed_row(
            "cert:unconfirmed-convergence-state-profile",
            P::UnconfirmedConvergenceStateProfile,
            ReviewableCollaborationStateRecord,
            ConvergenceStateUnverifiedProjection,
            &[PresenterFollowState],
            seed_certified_except(
                Ax::CollaborationStateTruth,
                seed_narrowed(
                    Ax::CollaborationStateTruth,
                    "a presenter / follow convergence state cannot be confirmed for this profile so a provider-authoritative collaboration-state record cannot be certified and the convergence stays inspect-only",
                    "A presenter / follow convergence state cannot be confirmed — a convergence-degraded state risks reading as awareness-degraded — so the ReviewableCollaborationStateRecord claim narrows to a convergence-state-unverified projection and the lane keeps convergence-degraded distinct from awareness-degraded rather than collapsing either into a generic stale badge",
                    Trig::ConvergenceStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::CollaborationStateTruth,
                ReviewableCollaborationStateRecord,
                ConvergenceStateUnverifiedProjection,
                "The convergence state is not confirmed, so convergence-degraded stays distinct from awareness-degraded and neither collapses into a generic stale badge",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "presenter-follow class: the presenter / follow state keeps its convergence state, view-only follow, and provenance-tracked handoff explicit and marks the convergence unverified rather than implying convergence the state did not reach",
                "the presenter-follow surface keeps its convergence-degraded and awareness-degraded states distinct while the convergence is disclosed as unverified",
                "collaboration-state-truth: ReviewableCollaborationStateRecord narrows to a convergence-state-unverified projection (auto-narrowed)",
                "collaboration-state-truth: convergence-degraded, awareness-degraded, and anchor-unresolved state is never collapsed into a generic stale badge — each stays distinct and legible",
            ],
        ),
        seed_row(
            "cert:unpreserved-local-truth-profile",
            P::UnpreservedLocalTruthProfile,
            ReviewableCollaborationStateRecord,
            LocalTruthPreservationUnverifiedProjection,
            &[HigherRiskControlPlane],
            seed_certified_except(
                Ax::Visual,
                seed_narrowed(
                    Ax::Visual,
                    "a higher-risk control-plane downgrade's local-unsent preservation cannot be confirmed for this profile so a provider-authoritative collaboration-state record cannot be certified and the downgrade stays local-editing-first",
                    "A higher-risk control-plane downgrade's local-unsent preservation cannot be confirmed — a permission downgrade, relay failure, or leave-session flow risks discarding unsent local edits — so the ReviewableCollaborationStateRecord claim narrows to a local-truth-preservation-unverified projection and the lane keeps unsent local edits preserved first rather than discarding them on downgrade",
                    Trig::LocalTruthPreservationUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Visual,
                ReviewableCollaborationStateRecord,
                LocalTruthPreservationUnverifiedProjection,
                "The local-truth preservation is unverified, so unsent local edits stay preserved first and none are discarded on a permission downgrade, relay failure, or leave-session flow",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "higher-risk-control-plane class: the control plane keeps its downgrade behavior, local-unsent preservation, and convergence-vs-awareness distinction explicit and marks the preservation unverified rather than discarding unsent local edits the downgrade did not preserve",
                "the higher-risk-control-plane surface keeps its preserved unsent local edits legible while the preservation is disclosed as unverified",
                "visual: ReviewableCollaborationStateRecord narrows to a local-truth-preservation-unverified projection (auto-narrowed)",
                "collaboration-state-truth: unsent local edits are never discarded on a permission downgrade, relay failure, or leave-session flow — they stay preserved first and reviewable",
            ],
        ),
        seed_row(
            "cert:unresolved-anchor-drift-profile",
            P::UnresolvedAnchorDriftProfile,
            ReviewableCollaborationStateRecord,
            AnchorDriftUnverifiedProjection,
            &[ServerOrderedCommentsAnnotationsReviewPins],
            seed_certified_except(
                Ax::HighZoomReflow,
                seed_narrowed(
                    Ax::HighZoomReflow,
                    "a server-ordered comment / annotation / review-pin anchor-drift history cannot be confirmed append-only for this profile so a provider-authoritative collaboration-state record cannot be certified and the anchor stays drift-reviewable",
                    "A server-ordered comment / annotation / review-pin anchor-drift history cannot be confirmed append-only — an anchor risks rebinding silently — so the ReviewableCollaborationStateRecord claim narrows to an anchor-drift-unverified projection and the lane keeps the drift history reviewable rather than silently rebinding a comment, annotation, or review pin",
                    Trig::AnchorDriftHistoryUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::HighZoomReflow,
                ReviewableCollaborationStateRecord,
                AnchorDriftUnverifiedProjection,
                "The anchor-drift history is unverified, so the comment, annotation, and review-pin anchors keep their append-only drift history reviewable and never rebind silently",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "server-ordered-pin class: the comment / annotation / review-pin objects keep their server-ordered anchor, revision-pair lineage, and append-only drift history explicit and mark the drift unverified rather than rebinding an anchor the history did not record",
                "the server-ordered-pin surface keeps its anchor-drift history legible while the drift is disclosed as unverified",
                "high-zoom-reflow: ReviewableCollaborationStateRecord narrows to an anchor-drift-unverified projection (auto-narrowed)",
                "collaboration-state-truth: a comment, annotation, or review pin is never silently rebound — its append-only drift history stays reviewable and a rebind is always recorded, never silent",
            ],
        ),
        seed_row(
            "cert:undisclosed-export-posture-profile",
            P::UndisclosedExportPostureProfile,
            ReviewableCollaborationStateRecord,
            ExportPostureUnverifiedProjection,
            &[SealedSessionArchive],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "a sealed session-archive export posture cannot be proven policy-labeled for this profile so a provider-authoritative collaboration-state record cannot be certified and the export stays blocked-until-labeled",
                    "A sealed session-archive export posture cannot be proven policy-labeled — an op-log, snapshot, or archive risks exporting without redaction or actor lineage — so the ReviewableCollaborationStateRecord claim narrows to an export-posture-unverified projection and the lane keeps the policy-labeled redaction and actor lineage explicit rather than exporting without both",
                    Trig::ExportPostureUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableCollaborationStateRecord,
                ExportPostureUnverifiedProjection,
                "The export posture is unproven, so the sealed archive stays labelled with its policy-labeled redaction and actor lineage and never exports an op-log, snapshot, or archive without both",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "sealed-archive class: the sealed session archive keeps its compaction lineage, retained-object refs, policy-labeled redaction, and actor lineage explicit and marks the export blocked rather than exporting an op-log, snapshot, or archive whose redaction label or actor lineage is undisclosed",
                "the sealed-archive surface keeps its policy-labeled redaction legible while the export is disclosed as a blocked-until-labeled posture",
                "localization: ReviewableCollaborationStateRecord narrows to an export-posture-unverified projection (auto-narrowed)",
                "collaboration-state-truth: an op-log, snapshot, or archive is never exported without policy-labeled redaction and actor lineage — both stay explicit and an unlabeled export never reads as a policy-labeled one",
            ],
        ),
        seed_row(
            "cert:unproven-provenance-freshness-profile",
            P::UnprovenProvenanceFreshnessProfile,
            ReviewableCollaborationStateRecord,
            ProvenanceFreshnessUnverifiedProjection,
            &[SampledPresenceCursorsSelections],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "a sampled presence / cursors / selections session provenance / freshness is unproven for this profile so a provider-authoritative collaboration-state record cannot be certified and the state stays provenance-labeled",
                    "A sampled presence / cursors / selections session provenance / freshness is unproven — the sampled state risks reading as current canonical truth to search, AI, review, docs, or support — so the ReviewableCollaborationStateRecord claim narrows to a provenance-freshness-unverified projection and the lane keeps the provenance and freshness explicit rather than letting stale collaboration state read as canonical",
                    Trig::ProvenanceOrFreshnessUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableCollaborationStateRecord,
                ProvenanceFreshnessUnverifiedProjection,
                "The provenance / freshness is unproven, so the sampled presence keeps its provenance and freshness explicit and never reads as current canonical truth to search, AI, review, docs, or support",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "sampled-presence class: the sampled presence / cursors / selections stream keeps its provenance, freshness, and expire-when-stale behavior explicit and marks the freshness unproven rather than reading as current canonical truth when the provenance or freshness is unconfirmed",
                "the sampled-presence surface keeps its provenance and freshness legible non-visually while the freshness is disclosed as unproven",
                "screen-reader: ReviewableCollaborationStateRecord narrows to a provenance-freshness-unverified projection (auto-narrowed)",
                "collaboration-state-truth: stale collaboration state is never read as current canonical truth — its provenance and freshness stay explicit and survive export and reconnect",
            ],
        ),
    ]
}
