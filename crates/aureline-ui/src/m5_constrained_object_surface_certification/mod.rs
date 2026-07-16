//! M05-1264 closing B150 surface certification over the frozen M5 constrained-file-state matrix — the
//! read-only, generated, policy-locked, managed, projection, and captured-snapshot current objects a
//! write-capable consumer must never treat as an ordinary directly-writable file.
//!
//! Where the freeze matrix ([`crate::m5_constrained_file_state_matrix`]) defines the six governed
//! constrained-current-object classes, the M05-1257..1263 implement lanes resolve each constrained-state
//! descriptor, change-diff, badge-group / reason-strip consumer, canonical-source relation, write-target
//! review, write-review-sheet fallback path, cross-actor mutation gate, drill corpus, and support / export
//! evidence packet; this closing capstone *certifies* that the shared constrained-object truth holds on every
//! claimed M5 editor, review, save, AI, repair, and export surface — state badges, blocked-write reason
//! strips, canonical-source rows, exact write targets, reviewed safe-next-steps, and actor-parity blocking —
//! and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** an editor / save operator, a review / diff owner, an AI / automation
//! flow, or a support / export consumer reads a constrained current object through (a fully-classified
//! constrained-object lane; a reviewable constrained-state record structure; a disclosed
//! generated-divergence-partial profile; an unverified canonical-source profile; an unverified
//! write-target-review profile; and an unverified actor-parity profile), not on the underlying object class or
//! implement lane. Each [`ConstrainedObjectProfileCertificationRow`] certifies one profile across nine truth
//! axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export,
//! degraded-state, and constrained-object-truth behavior — and either passes (green), auto-narrows its
//! constrained-object claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded
//! axis is hidden behind a fresh certified claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedConstrainedObjectTruth` / `ReviewableConstrainedStateRecord` claim while one of its truth axes is
//! not current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a fully-classified constrained-object
//! lane — one whose state badge, blocked-write reason, canonical-source relation, exact write target, and
//! reviewed safe-next-step all converge on one export-safe, internally consistent constrained-object record —
//! may certify a `CertifiedConstrainedObjectTruth` claim; a reviewable, generated-divergence-partial,
//! unverified-canonical-source, unverified-write-target-review, or unverified-actor-parity profile that keeps a
//! certified claim is over-reaching and blocks. The always-on CLI/export axis must always stay certified so
//! support and automation can reconstruct the state badge, blocked-write reason, canonical source, exact write
//! target, write disposition, and safe-next-step from the same constrained-object proof the operator saw.
//!
//! The B150 hard invariants are enforced per row: no profile may let one constrained-state class hide another
//! when both materially affect behavior; let a generated, managed, projection, or archived object silently fall
//! back to a lossy direct write; give an AI, automation, import, or repair flow a hidden bypass around the
//! constrained-state rules; leave the canonical source, exact write target, preserved-versus-lost sync, or
//! recovery / regenerate path unstated; or present a constrained object as directly writable or hide the
//! recovery / regenerate path. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical constrained-file-state matrix proof bundle
//! ([`CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen constrained-file-state matrix proof —
//! rather than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets,
//! bearer tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/release/m5-constrained-object-surface-certification.schema.json`](../../../../schemas/release/m5-constrained-object-surface-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_constrained_object_surface_certification.md`](../../../../docs/release/m5_constrained_object_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_constrained_file_state_matrix as matrix;
use matrix::{M5ConstrainedFileStateDowngradeTrigger, M5ConstrainedFileStateObject};

/// Schema version stamped on the M05-1264 certification packet.
pub const CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ConstrainedObjectProfileCertificationPacket`].
pub const CONSTRAINED_OBJECT_CERT_RECORD_KIND: &str =
    "m5_constrained_object_surface_certification_packet";

/// Stable record-kind tag carried by each [`ConstrainedObjectProfileCertificationRow`].
pub const CONSTRAINED_OBJECT_CERT_ROW_RECORD_KIND: &str =
    "m5_constrained_object_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const CONSTRAINED_OBJECT_CERT_SCHEMA_REF: &str =
    "schemas/release/m5-constrained-object-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const CONSTRAINED_OBJECT_CERT_DOC_REF: &str =
    "docs/release/m5_constrained_object_surface_certification.md";

/// Repo-relative path of the frozen constrained-file-state matrix schema the certified profiles render.
pub const CONSTRAINED_OBJECT_CERT_MATRIX_REF: &str =
    matrix::M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF;

/// The one canonical constrained-file-state matrix proof bundle every certified profile cites as its
/// first-resolved constrained-object truth. All five profiles point back to it rather than cloning per-profile
/// evidence.
pub const CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_CONSTRAINED_FILE_STATE_ARTIFACT_REF;

/// The constrained-object-health dashboard the release surfaces consume. Recorded as a supporting evidence
/// ref on every row so the certification's constrained-object truth ties back to the same dashboard consumers
/// read.
pub const CONSTRAINED_OBJECT_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_CONSTRAINED_FILE_STATE_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CONSTRAINED_OBJECT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-constrained-object-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CONSTRAINED_OBJECT_CERT_CSV_REF: &str =
    "artifacts/release/m5-constrained-object-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CONSTRAINED_OBJECT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-constrained-object-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const CONSTRAINED_OBJECT_CERT_FIXTURE_DIR: &str =
    "fixtures/release/m5-constrained-object-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const CONSTRAINED_OBJECT_CERT_PACKET_ID: &str =
    "m5-constrained-object-surface-certification:stable:0001";

/// The six claimed M5 constrained-object consumer profiles this capstone certifies. Keyed on the profile
/// an editor / save operator, a review / diff owner, an AI / automation flow, or a support / export consumer
/// reads a constrained current object through — a fully-classified constrained-object lane, a reviewable
/// constrained-state record structure, a disclosed generated-divergence-partial profile, an unverified
/// canonical-source profile, an unverified write-target-review profile, and an unverified actor-parity profile
/// — not on the reusable object class it renders. Only a fully-classified constrained-object lane profile may
/// certify a certified constrained-object claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedObjectCertifiedProfile {
    /// A fully-classified constrained-object lane — a constrained current object whose state badge,
    /// blocked-write reason, canonical-source relation, exact write target, and reviewed safe-next-step all
    /// converge on one export-safe, internally consistent constrained-object record that stays identical across
    /// every editor, review, save, AI, repair, and export consumer, certifying the constrained-object claim
    /// exactly right now.
    CertifiedConstrainedObjectLane,
    /// A reviewable constrained-state record structure: a self-sufficient, inspectable constrained-state
    /// descriptor (a state-class / canonical-source / write-target record an operator can review), never itself
    /// a fully-classified constrained-object lane.
    ReviewableConstrainedRecordStructure,
    /// A generated / derived-artifact lane whose divergence from its generator can only be partially disclosed;
    /// the claim narrows to a generated-divergence-disclosed projection that discloses the diverged-from-source
    /// state alongside its canonical generator input and regenerate path, never a generated artifact silently
    /// falling back to a lossy direct write while its divergence is incomplete.
    DisclosedGeneratedDivergencePartialProfile,
    /// A canonical-source lane whose canonical source or live target (the authoritative object an edit belongs
    /// to) can no longer be validated; the claim narrows to a canonical-source-unverified projection that keeps
    /// the last-known canonical-source relation explicit, never a constrained object shown as directly writable
    /// when its canonical source or exact write target can no longer be resolved.
    UnverifiedCanonicalSourceProfile,
    /// A write-target-review lane whose reviewed write target (the exact bytes a mutation would touch and the
    /// approval / restore review before it) can no longer be reconstructed; the claim narrows to a
    /// write-target-review-unverified projection that keeps the last-known preserved-versus-lost sync posture
    /// explicit, never a managed / captured-snapshot object mutated through a silent lossy fallback.
    UnverifiedWriteTargetReviewProfile,
    /// An actor-parity lane whose shared constrained-write blocking across the direct-edit, AI, automation,
    /// import, and repair actors can no longer be verified; the claim narrows to an actor-parity-unverified
    /// projection that keeps the last-known blocked-write reason explicit, never an AI / automation / import /
    /// repair flow given a hidden bypass around the constrained-state rules.
    UnverifiedActorParityProfile,
}

impl M5ConstrainedObjectCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5ConstrainedObjectCertifiedProfile; 6] = [
        M5ConstrainedObjectCertifiedProfile::CertifiedConstrainedObjectLane,
        M5ConstrainedObjectCertifiedProfile::ReviewableConstrainedRecordStructure,
        M5ConstrainedObjectCertifiedProfile::DisclosedGeneratedDivergencePartialProfile,
        M5ConstrainedObjectCertifiedProfile::UnverifiedCanonicalSourceProfile,
        M5ConstrainedObjectCertifiedProfile::UnverifiedWriteTargetReviewProfile,
        M5ConstrainedObjectCertifiedProfile::UnverifiedActorParityProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedConstrainedObjectLane => "certified_constrained_object_lane",
            Self::ReviewableConstrainedRecordStructure => "reviewable_constrained_record_structure",
            Self::DisclosedGeneratedDivergencePartialProfile => {
                "disclosed_generated_divergence_partial_profile"
            }
            Self::UnverifiedCanonicalSourceProfile => "unverified_canonical_source_profile",
            Self::UnverifiedWriteTargetReviewProfile => "unverified_write_target_review_profile",
            Self::UnverifiedActorParityProfile => "unverified_actor_parity_profile",
        }
    }

    /// True only for the fully-classified constrained-object lane profile. A certified constrained-object claim
    /// may be certified on this profile alone; every other profile is at most a reviewable constrained-state
    /// record structure or a narrowed projection.
    pub const fn is_certified_constrained_object_lane(self) -> bool {
        matches!(self, Self::CertifiedConstrainedObjectLane)
    }
}

/// The claim ladder a certified constrained-object profile asserts and is certified down to. Minted locally
/// for this capstone (B150 folds accessibility into the cert): the strongest claim is a fully certified
/// constrained-object record; each weaker tier is a disclosed projection that keeps the last-known
/// generated-divergence, canonical-source, write-target-review, or actor-parity posture explicit rather than
/// overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConstrainedObjectClaim {
    /// Certified constrained-object truth: a fully-classified constrained current object whose state badge,
    /// blocked-write reason, canonical-source relation, exact write target, and reviewed safe-next-step all join
    /// to one export-safe, internally consistent record — the strongest claim, the constrained-object handling
    /// Aureline can present as cleanly-classified and honestly non-writable-in-place across every consumer.
    CertifiedConstrainedObjectTruth,
    /// Reviewable constrained-state record: a self-sufficient, inspectable constrained-state descriptor
    /// (a state-class / canonical-source / write-target record an operator can inspect) that is not itself a
    /// fully-classified constrained-object lane.
    ReviewableConstrainedStateRecord,
    /// Generated-divergence-disclosed projection: a generated / derived-artifact lane's divergence from its
    /// generator can only be partially disclosed; the lane stays a generated-divergence-disclosed projection
    /// that discloses the diverged-from-source state alongside its canonical generator input and regenerate
    /// path, never a generated artifact falling back to a lossy direct write while its divergence is incomplete.
    GeneratedDivergenceDisclosedProjection,
    /// Canonical-source-unverified projection: a canonical-source lane's authoritative source or live target
    /// can no longer be validated; the lane stays a canonical-source-unverified projection that keeps the
    /// last-known canonical-source relation explicit, never a constrained object shown as directly writable when
    /// its canonical source or exact write target can no longer be resolved.
    CanonicalSourceUnverifiedProjection,
    /// Write-target-review-unverified projection: a write-target-review lane's reviewed write target and
    /// approval / restore review can no longer be reconstructed; the lane stays a write-target-review-unverified
    /// projection that keeps the last-known preserved-versus-lost sync posture explicit, never a managed /
    /// captured-snapshot object mutated through a silent lossy fallback.
    WriteTargetReviewUnverifiedProjection,
    /// Actor-parity-unverified projection: an actor-parity lane's shared constrained-write blocking across the
    /// direct-edit, AI, automation, import, and repair actors can no longer be verified; the lane stays an
    /// actor-parity-unverified projection that keeps the last-known blocked-write reason explicit, never an AI /
    /// automation / import / repair flow given a hidden bypass around the constrained-state rules.
    ActorParityUnverifiedProjection,
}

impl M5ConstrainedObjectClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::CertifiedConstrainedObjectTruth,
        Self::ReviewableConstrainedStateRecord,
        Self::GeneratedDivergenceDisclosedProjection,
        Self::CanonicalSourceUnverifiedProjection,
        Self::WriteTargetReviewUnverifiedProjection,
        Self::ActorParityUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedConstrainedObjectTruth => 5,
            Self::ReviewableConstrainedStateRecord => 4,
            Self::GeneratedDivergenceDisclosedProjection => 3,
            Self::CanonicalSourceUnverifiedProjection => 2,
            Self::WriteTargetReviewUnverifiedProjection => 1,
            Self::ActorParityUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-classified, certified constrained-object record.
    pub const fn asserts_certified_constrained_object_truth(self) -> bool {
        matches!(self, Self::CertifiedConstrainedObjectTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedConstrainedObjectTruth | Self::ReviewableConstrainedStateRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedConstrainedObjectTruth => "certified_constrained_object_truth",
            Self::ReviewableConstrainedStateRecord => "reviewable_constrained_state_record",
            Self::GeneratedDivergenceDisclosedProjection => {
                "generated_divergence_disclosed_projection"
            }
            Self::CanonicalSourceUnverifiedProjection => "canonical_source_unverified_projection",
            Self::WriteTargetReviewUnverifiedProjection => {
                "write_target_review_unverified_projection"
            }
            Self::ActorParityUnverifiedProjection => "actor_parity_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and constrained-object-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstrainedObjectCertificationAxis {
    /// Visual parity: the state badge, blocked-write reason, canonical source, exact write target, and reviewed
    /// safe-next-step are shown on the primary surface without relying on a shell-chrome-only affordance or a
    /// mislabeled writable-looking row alone, and no constrained object still reads as directly writable.
    Visual,
    /// Keyboard-reach parity: the same constrained-object truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled writable-looking row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// state badge, blocked-write reason, canonical source, exact write target, or safe-next-step.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the state badge, blocked-write reason, or canonical source.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// state class, canonical source, write disposition, or safe-next-step when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale constrained-state descriptor, an unresolved canonical source, an
    /// unverified write target, or an unproven actor-parity block honestly downgrades a
    /// `CertifiedConstrainedObjectTruth` / `ReviewableConstrainedStateRecord` claim rather than reading as a
    /// fresh, fully-classified constrained-object record.
    DegradedState,
    /// Constrained-object-truth parity: the state badge, blocked-write reason, canonical-source relation, exact
    /// write target, preserved-versus-lost sync, and recovery / regenerate path stay explicit and never let one
    /// constrained-state class hide another; let a generated / managed / projection / archived object silently
    /// fall back to a lossy direct write; give an AI / automation / import / repair flow a hidden bypass around
    /// the constrained-state rules; leave the canonical source, exact write target, preserved-versus-lost sync,
    /// or recovery / regenerate path unstated; or present a constrained object as directly writable or hide the
    /// recovery / regenerate path.
    ConstrainedObjectTruth,
}

impl ConstrainedObjectCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ConstrainedObjectCertificationAxis; 9] = [
        ConstrainedObjectCertificationAxis::Visual,
        ConstrainedObjectCertificationAxis::Keyboard,
        ConstrainedObjectCertificationAxis::ScreenReader,
        ConstrainedObjectCertificationAxis::HighZoomReflow,
        ConstrainedObjectCertificationAxis::HighContrast,
        ConstrainedObjectCertificationAxis::Localization,
        ConstrainedObjectCertificationAxis::CliExport,
        ConstrainedObjectCertificationAxis::DegradedState,
        ConstrainedObjectCertificationAxis::ConstrainedObjectTruth,
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
            Self::ConstrainedObjectTruth => "constrained_object_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstrainedObjectAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl ConstrainedObjectAxisCertificationState {
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
pub enum ConstrainedObjectProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// constrained-object profile claims a certified constrained-object record, or the narrowing is inconsistent.
    Red,
}

impl ConstrainedObjectProfileClaimStatus {
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

/// The five B150 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedObjectCertGuardrails {
    /// True if the profile lets one constrained-state class hide another when both materially affect behavior.
    /// Must be false.
    pub lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior: bool,
    /// True if the profile lets a generated, managed, projection, or archived object silently fall back to a
    /// lossy direct write. Must be false.
    pub lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write:
        bool,
    /// True if the profile gives an AI, automation, import, or repair flow a hidden bypass around the
    /// constrained-state rules. Must be false.
    pub gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules:
        bool,
    /// True if the profile leaves the canonical source, exact write target, preserved-versus-lost sync, or
    /// recovery / regenerate path unstated. Must be false.
    pub leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated:
        bool,
    /// True if the profile presents a constrained object as directly writable or hides the recovery / regenerate
    /// path. Must be false.
    pub presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path:
        bool,
}

impl ConstrainedObjectCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior: false,
        lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write:
            false,
        gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules:
            false,
        leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated:
            false,
        presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path:
            false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior
            && !self.lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write
            && !self.gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules
            && !self.leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated
            && !self.presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedObjectCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The state-badge / blocked-write-reason / canonical-source / exact-write-target / write-disposition /
    /// safe-next-step fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl ConstrainedObjectCertExportParity {
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
pub struct ConstrainedObjectAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ConstrainedObjectCertificationAxis,
    /// The certification state of the axis.
    pub state: ConstrainedObjectAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ConstrainedFileStateDowngradeTrigger>,
}

impl ConstrainedObjectAxisOutcome {
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
            ConstrainedObjectAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ConstrainedObjectAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ConstrainedObjectAxisCertificationState::UndisclosedDrift => {
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
pub struct ConstrainedObjectClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ConstrainedObjectCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5ConstrainedObjectClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5ConstrainedObjectClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 constrained-object object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedObjectProfileCertificationRow {
    /// Record kind; must equal [`CONSTRAINED_OBJECT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5ConstrainedObjectCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5ConstrainedObjectClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5ConstrainedObjectClaim,
    /// The frozen constrained-object object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ConstrainedFileStateObject>,
    /// One outcome per [`ConstrainedObjectCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ConstrainedObjectAxisOutcome>,
    /// The B150 hard invariants; all must hold.
    pub guardrails: ConstrainedObjectCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ConstrainedObjectClaimAutoNarrow>,
    /// The one canonical constrained-file-state matrix proof bundle this profile cites. Must equal
    /// [`CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ConstrainedObjectProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: ConstrainedObjectCertExportParity,
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

impl ConstrainedObjectProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: ConstrainedObjectCertificationAxis,
    ) -> Option<&ConstrainedObjectAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ConstrainedObjectCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ConstrainedObjectCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ConstrainedObjectAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ConstrainedObjectCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ConstrainedObjectAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-classified constrained-object lane
    /// profile may certify a certified constrained-object record, every hard invariant must hold, CLI/export
    /// parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ConstrainedObjectProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        // Every B150 hard invariant must hold.
        if !self.guardrails.all_held() {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        // Only a fully-classified constrained-object lane profile may certify a certified constrained-object record.
        if self
            .certified_claim
            .asserts_certified_constrained_object_truth()
            && !self.profile.is_certified_constrained_object_lane()
        {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ConstrainedObjectCertificationAxis::CliExport) {
            Some(o) if o.state == ConstrainedObjectAxisCertificationState::Certified => {}
            _ => return ConstrainedObjectProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ConstrainedObjectAxisCertificationState::UndisclosedDrift)
        {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ConstrainedObjectProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ConstrainedObjectProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ConstrainedObjectProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ConstrainedObjectProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return ConstrainedObjectProfileClaimStatus::Red;
        }

        ConstrainedObjectProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CONSTRAINED_OBJECT_CERT_ROW_RECORD_KIND
            && self.schema_version == CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1264 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedObjectProfileCertificationSummary {
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

/// Constructor input for [`ConstrainedObjectProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedObjectProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ConstrainedObjectProfileCertificationRow>,
}

/// Checked-in M05-1264 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedObjectProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ConstrainedObjectProfileCertificationRow>,
    pub summary: ConstrainedObjectProfileCertificationSummary,
}

impl ConstrainedObjectProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ConstrainedObjectProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION,
            record_kind: CONSTRAINED_OBJECT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ConstrainedObjectProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5ConstrainedObjectCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Constrained-object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ConstrainedFileStateObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5ConstrainedObjectCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen constrained-object object class is certified on at least one profile — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ConstrainedFileStateObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ConstrainedObjectCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ConstrainedObjectAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ConstrainedObjectProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ConstrainedObjectProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        ConstrainedObjectProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ConstrainedObjectProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ConstrainedObjectCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION {
            violations.push(ConstrainedObjectCertificationViolation::SchemaVersion {
                expected: CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CONSTRAINED_OBJECT_CERT_RECORD_KIND {
            violations.push(ConstrainedObjectCertificationViolation::RecordKind {
                expected: CONSTRAINED_OBJECT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ConstrainedObjectCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ConstrainedObjectCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ConstrainedObjectCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ConstrainedObjectCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    ConstrainedObjectCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    ConstrainedObjectCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ConstrainedObjectCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B150 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(ConstrainedObjectCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a fully-classified constrained-object lane profile may certify a certified constrained-object record.
            if row
                .certified_claim
                .asserts_certified_constrained_object_truth()
                && !row.profile.is_certified_constrained_object_lane()
            {
                violations.push(
                    ConstrainedObjectCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ConstrainedObjectCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    ConstrainedObjectCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ConstrainedObjectCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    ConstrainedObjectCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == ConstrainedObjectProfileClaimStatus::Red {
                violations.push(ConstrainedObjectCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(ConstrainedObjectCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen constrained-object object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(ConstrainedObjectCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ConstrainedObjectCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                ConstrainedObjectCertificationViolation::RawConstrainedObjectMaterialInExport,
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
        out.push_str("# M5 Constrained-Object Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5ConstrainedObjectCertifiedProfile::ALL.len(),
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
pub fn current_m5_constrained_object_surface_certification_export(
) -> Result<ConstrainedObjectProfileCertificationPacket, ConstrainedObjectCertificationArtifactError>
{
    let packet: ConstrainedObjectProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-constrained-object-surface-certification/support_export.json"
        )))
        .map_err(ConstrainedObjectCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ConstrainedObjectCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ConstrainedObjectCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ConstrainedObjectCertificationViolation>),
}

impl fmt::Display for ConstrainedObjectCertificationArtifactError {
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

impl Error for ConstrainedObjectCertificationArtifactError {}

/// Validation failure for M05-1264 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstrainedObjectCertificationViolation {
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
    RawConstrainedObjectMaterialInExport,
}

impl fmt::Display for ConstrainedObjectCertificationViolation {
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
                    "packet does not cite the canonical constrained-file-state matrix proof bundle"
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
                    "row {id} does not cite the one canonical constrained-file-state matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B150 hard invariant: letting one constrained-state class hide another \
when both materially affect behavior; letting a generated / managed / projection / archived object silently \
fall back to a lossy direct write; giving an AI / automation / import / repair flow a hidden bypass around the \
constrained-state rules; leaving the canonical source, exact write target, preserved-versus-lost sync, or \
recovery / regenerate path unstated; or presenting a constrained object as directly writable or hiding the \
recovery / regenerate path"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified constrained-object record on a non-lane profile"
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
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified constrained-object \
record, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 constrained-object profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen constrained-object object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawConstrainedObjectMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for ConstrainedObjectCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ConstrainedObjectAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ConstrainedObjectAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the constrained-object
/// generics the spec forbids collapsing distinct state-class, canonical-source, write-target, and actor-parity
/// truth into (whole-label matches so a full sentence naming a concrete state class, canonical source, or exact
/// write target is not flagged).
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
            | "constrained"
            | "constrained object"
            | "read only"
            | "read-only"
            | "generated"
            | "generated artifact"
            | "policy locked"
            | "policy-locked"
            | "managed"
            | "projection"
            | "captured snapshot"
            | "snapshot"
            | "state badge"
            | "state class"
            | "blocked write"
            | "blocked write reason"
            | "canonical source"
            | "canonical source relation"
            | "write target"
            | "exact write target"
            | "write disposition"
            | "safe next step"
            | "safe action"
            | "next step"
            | "duplicate"
            | "detach"
            | "overlay"
            | "regenerate"
            | "request approval"
            | "approval"
            | "preserved versus lost"
            | "sync"
            | "recovery"
            | "recovery path"
            | "regenerate path"
            | "actor parity"
            | "bypass"
            | "lossy"
            | "lossy fallback"
            | "fallback"
            | "descriptor"
            | "mismatch"
            | "drift"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the constrained-file-state
/// matrix heuristic so the reused [`M5ConstrainedFileStateDowngradeTrigger`] narrowings
/// serialize cleanly — the constrained-object proof grammar carries only typed class tokens and opaque refs,
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

/// Builds the canonical, checked-in M05-1264 certification packet. Certifies all six claimed M5
/// constrained-object profiles: two deliver their claim (green) and four auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_constrained_object_surface_certification_packet(
) -> ConstrainedObjectProfileCertificationPacket {
    ConstrainedObjectProfileCertificationPacket::new(
        ConstrainedObjectProfileCertificationPacketInput {
            packet_id: CONSTRAINED_OBJECT_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-16T00:00:00Z".to_owned(),
            matrix_ref: CONSTRAINED_OBJECT_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:constrained-object-surface-certification:{id}"),
        CONSTRAINED_OBJECT_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ConstrainedObjectCertExportParity {
    ConstrainedObjectCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ConstrainedObjectCertificationAxis) -> &'static str {
    match axis {
        ConstrainedObjectCertificationAxis::Visual => {
            "state badge, blocked-write reason, canonical source, exact write target, and reviewed safe-next-step shown on-surface without a shell-chrome-only affordance or a mislabeled writable-looking row alone, and no constrained object still reads as directly writable"
        }
        ConstrainedObjectCertificationAxis::Keyboard => {
            "the same constrained-object state class, canonical source, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        ConstrainedObjectCertificationAxis::ScreenReader => {
            "the same constrained-object truth is announced non-visually, never a shell-chrome-only / mislabeled-writable-row / unlabeled-control-only cue"
        }
        ConstrainedObjectCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the state badge, blocked-write reason, canonical source, exact write target, or safe-next-step"
        }
        ConstrainedObjectCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the state badge, blocked-write reason, or canonical source"
        }
        ConstrainedObjectCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a state class, canonical source, write disposition, or safe-next-step"
        }
        ConstrainedObjectCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        ConstrainedObjectCertificationAxis::DegradedState => {
            "a stale constrained-state descriptor, an unresolved canonical source, an unverified write target, or an unproven actor-parity block honestly downgrades the CertifiedConstrainedObjectTruth/ReviewableConstrainedStateRecord claim rather than reading as a fresh, fully-classified constrained-object record"
        }
        ConstrainedObjectCertificationAxis::ConstrainedObjectTruth => {
            "state badge, blocked-write reason, canonical-source relation, exact write target, preserved-versus-lost sync, and recovery / regenerate path stay explicit and never let one constrained-state class hide another, let a generated / managed / projection / archived object silently fall back to a lossy direct write, give an AI / automation / import / repair flow a hidden bypass around the constrained-state rules, leave the canonical source, exact write target, preserved-versus-lost sync, or recovery / regenerate path unstated, or present a constrained object as directly writable or hide the recovery / regenerate path"
        }
    }
}

fn seed_certified(axis: ConstrainedObjectCertificationAxis) -> ConstrainedObjectAxisOutcome {
    ConstrainedObjectAxisOutcome {
        axis,
        state: ConstrainedObjectAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ConstrainedObjectCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ConstrainedFileStateDowngradeTrigger,
) -> ConstrainedObjectAxisOutcome {
    ConstrainedObjectAxisOutcome {
        axis,
        state: ConstrainedObjectAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ConstrainedObjectAxisOutcome> {
    ConstrainedObjectCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ConstrainedObjectCertificationAxis,
    outcome: ConstrainedObjectAxisOutcome,
) -> Vec<ConstrainedObjectAxisOutcome> {
    ConstrainedObjectCertificationAxis::ALL
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
    profile: M5ConstrainedObjectCertifiedProfile,
    claimed_claim: M5ConstrainedObjectClaim,
    certified_claim: M5ConstrainedObjectClaim,
    consumed_families: &[M5ConstrainedFileStateObject],
    axis_outcomes: Vec<ConstrainedObjectAxisOutcome>,
    claim_auto_narrow: Option<ConstrainedObjectClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ConstrainedObjectProfileCertificationRow {
    let mut row = ConstrainedObjectProfileCertificationRow {
        record_kind: CONSTRAINED_OBJECT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: ConstrainedObjectCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ConstrainedObjectProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            CONSTRAINED_OBJECT_CERT_MATRIX_REF.to_owned(),
            CONSTRAINED_OBJECT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ConstrainedObjectCertificationAxis,
    from_claim: M5ConstrainedObjectClaim,
    to_claim: M5ConstrainedObjectClaim,
    label: &str,
) -> ConstrainedObjectClaimAutoNarrow {
    ConstrainedObjectClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ConstrainedObjectProfileCertificationRow> {
    use ConstrainedObjectCertificationAxis as Ax;
    use M5ConstrainedFileStateDowngradeTrigger as Trig;
    use M5ConstrainedFileStateObject::*;
    use M5ConstrainedObjectCertifiedProfile as P;
    use M5ConstrainedObjectClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-constrained-object-lane",
            P::CertifiedConstrainedObjectLane,
            CertifiedConstrainedObjectTruth,
            CertifiedConstrainedObjectTruth,
            &[ReadOnly],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "exact_write_target",
            ],
            &[
                "certified constrained-object lane: the state badge, blocked-write reason, canonical-source relation, exact write target, and reviewed safe-next-step all join to one export-safe record, never a constrained object shown as directly writable or mutated through a silent lossy fallback",
                "the certified constrained-object record keeps stable operation IDs while the state badge, blocked-write reason, canonical source, exact write target, and safe-next-step bind to the one constrained-file-state matrix across tab-chrome / breadcrumb / status-bar / command-palette / editor-banner / diff-review-header / write-review-sheet / AI-automation-path / support-export surfaces, and no constrained object reads as directly writable in one surface and blocked in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered constrained-object record",
                "constrained-object-truth: a fully-classified constrained-object lane with export-safe, internally consistent state is the only profile that certifies a certified constrained-object record",
            ],
        ),
        seed_row(
            "cert:reviewable-constrained-record-structure",
            P::ReviewableConstrainedRecordStructure,
            ReviewableConstrainedStateRecord,
            ReviewableConstrainedStateRecord,
            &[PolicyLocked],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "state_badge",
            ],
            &[
                "record-structure class: an export-safe constrained-state descriptor bound to one object and inspectable rather than a per-surface description copied by hand, with the policy-locked approval gate and owning rule separated from the editable body",
                "the reviewable constrained-state record keeps its state badge, blocked-write reason, canonical source, and exact write target inspectable rather than a shell-chrome-only or mislabeled-writable-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable constrained-state record structure",
                "constrained-object-truth: a reviewable constrained-state record never certifies a fully-classified-lane claim and never stays green on a stale descriptor or an unresolved canonical source",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-generated-divergence-partial-profile",
            P::DisclosedGeneratedDivergencePartialProfile,
            ReviewableConstrainedStateRecord,
            GeneratedDivergenceDisclosedProjection,
            &[Generated],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the generated / derived-artifact lane carries an artifact whose divergence from its generator can only be partially disclosed for this profile so a fully-classified constrained-object record cannot be certified",
                    "The generated / derived-artifact lane carries an artifact whose diverged-from-source state and regenerate path can only be partially disclosed, so the ReviewableConstrainedStateRecord claim narrows to a generated-divergence-disclosed projection and the lane discloses the diverged-from-source state alongside its canonical generator input and regenerate path rather than letting the artifact silently fall back to a lossy direct write or read as directly writable",
                    Trig::SilentLossyDirectWriteFallback,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableConstrainedStateRecord,
                GeneratedDivergenceDisclosedProjection,
                "Generated-artifact divergence from its generator is only partially disclosed for this artifact, so it is shown alongside its canonical generator input and regenerate path and never reads as a directly writable file",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "generated-divergence-partial class: the artifact names its canonical generator input, diverged-from-source state, and regenerate path and marks divergence as disclosed-partial rather than letting a generated artifact read as directly writable when its divergence is incomplete",
                "the generated-divergence-partial surface keeps its canonical generator input and diverged-from-source state legible while divergence is disclosed as partial",
                "localization: ReviewableConstrainedStateRecord narrows to a generated-divergence-disclosed projection (auto-narrowed)",
                "constrained-object-truth: a partially-disclosed generated artifact never falls back to a lossy direct write — the canonical generator input and regenerate path are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-canonical-source-profile",
            P::UnverifiedCanonicalSourceProfile,
            ReviewableConstrainedStateRecord,
            CanonicalSourceUnverifiedProjection,
            &[Projection],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the projection's canonical source or backing object can no longer be resolved so a fully-classified constrained-object record cannot be certified and the projection stays inspect-only",
                    "The projection's canonical source or backing object can no longer be resolved, so the ReviewableConstrainedStateRecord claim narrows to a canonical-source-unverified projection and the lane keeps the last-known canonical-source relation explicit rather than staying green on a resolvable write target or presenting the projection as directly writable when its exact write target can no longer be resolved",
                    Trig::CanonicalSourceUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableConstrainedStateRecord,
                CanonicalSourceUnverifiedProjection,
                "The projection's canonical source can no longer be resolved, so the last-known canonical-source relation stays explicit and no projection reads as directly writable or resolves to an ambiguous write target",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "canonical-source class: the projection keeps its canonical-source relation and exact-write-target resolution explicit and marks the canonical source as unverified rather than staying green on a resolvable write target when the backing object can no longer be resolved",
                "the canonical-source surface keeps its backing-object relation and write-target resolution legible while the canonical source is disclosed as unverified",
                "degraded-state: ReviewableConstrainedStateRecord narrows to a canonical-source-unverified projection (auto-narrowed)",
                "constrained-object-truth: a projection never reads as directly writable when its canonical source or exact write target can no longer be resolved",
            ],
        ),
        seed_row(
            "cert:unverified-write-target-review-profile",
            P::UnverifiedWriteTargetReviewProfile,
            ReviewableConstrainedStateRecord,
            WriteTargetReviewUnverifiedProjection,
            &[Managed],
            seed_certified_except(
                Ax::ConstrainedObjectTruth,
                seed_narrowed(
                    Ax::ConstrainedObjectTruth,
                    "the managed object's reviewed write target and preserved-versus-lost sync note can no longer be reconstructed so a fully-classified constrained-object record cannot be certified",
                    "The managed object's reviewed write target, required approval, and preserved-versus-lost sync note can no longer be reconstructed, so the ReviewableConstrainedStateRecord claim narrows to a write-target-review-unverified projection and the lane keeps the last-known preserved-versus-lost sync posture explicit and still names the managing owner and detach path rather than mutating the managed object through a silent lossy fallback",
                    Trig::PreservedVersusLostSyncUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ConstrainedObjectTruth,
                ReviewableConstrainedStateRecord,
                WriteTargetReviewUnverifiedProjection,
                "The managed object's reviewed write target is unreconstructable, so the last-known preserved-versus-lost sync posture stays explicit and no managed object is mutated through a silent lossy fallback",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "write-target-review class: the managed object keeps its exact write target, required approval, and preserved-versus-lost sync note explicit and marks the write-target review as unverified rather than mutating a managed / mirrored object through a silent lossy fallback",
                "the write-target-review surface keeps its managing owner, detach path, and preserved-versus-lost sync note legible while the reviewed write target is disclosed as unverified",
                "constrained-object-truth: ReviewableConstrainedStateRecord narrows to a write-target-review-unverified projection (auto-narrowed)",
                "constrained-object-truth: a managed object cites its exact write target and preserved-versus-lost sync and never mutates through a lossy fallback, and no claim outpaces the reconstructable review",
            ],
        ),
        seed_row(
            "cert:unverified-actor-parity-profile",
            P::UnverifiedActorParityProfile,
            ReviewableConstrainedStateRecord,
            ActorParityUnverifiedProjection,
            &[CapturedSnapshot],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "the shared constrained-write block across the direct-edit, AI, automation, import, and repair actors can no longer be verified for this captured snapshot so a fully-classified constrained-object record cannot be certified",
                    "The shared constrained-write block across the direct-edit, AI, automation, import, and repair actors can no longer be verified, so the ReviewableConstrainedStateRecord claim narrows to an actor-parity-unverified projection and the lane keeps the last-known blocked-write reason and safe-next-step explicit rather than giving an AI / automation / import / repair flow a hidden bypass around the constrained-state rules",
                    Trig::AiAutomationBypassedConstraint,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableConstrainedStateRecord,
                ActorParityUnverifiedProjection,
                "Shared actor-parity blocking is unverified for this captured snapshot, so the last-known blocked-write reason stays explicit and no AI / automation / import / repair flow gets a hidden bypass",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "actor-parity class: the captured snapshot keeps its blocked-write reason and safe-next-step identical across the direct-edit, AI, automation, import, and repair actors and marks actor parity as unverified rather than giving one actor a hidden bypass",
                "the actor-parity surface keeps its blocked-write reason and safe-next-step legible non-visually while actor parity is disclosed as unverified",
                "screen-reader: ReviewableConstrainedStateRecord narrows to an actor-parity-unverified projection (auto-narrowed)",
                "constrained-object-truth: every mutation actor hits the same blocked-write reason and safe-next-step, and no AI / automation / import / repair flow bypasses the constrained-state rules",
            ],
        ),
    ]
}
