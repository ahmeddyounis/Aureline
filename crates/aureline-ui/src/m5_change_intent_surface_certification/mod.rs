//! M05-1293 closing B153 surface certification over the frozen M5 change-intent and engineering-lifecycle
//! matrix — the durable change-intent record, start-work sheet, linked-change panel, ready-for-review handoff
//! sheet, resolve-or-close sheet, and blocked-or-escalate card that a work-item, start-work, review,
//! provider-handoff, help / docs, or support / export consumer must treat as first-class, durable,
//! publish-safe change-intent objects rather than ad hoc work-item chrome.
//!
//! Where the freeze matrix ([`crate::m5_change_intent_and_engineering_lifecycle_matrix`]) defines the six
//! governed change-intent object classes, the M05-1285..1290 implement lanes resolve each change-intent
//! record / start-work sheet, linked-change panel / relation, ready-for-review handoff / publish-action,
//! resolve-or-close sheet / resolution-outcome, blocked-or-escalate card / escalation-outcome, and
//! lifecycle-state / reconcile-flow registry; this closing capstone *certifies* that the shared change-intent
//! truth holds on every claimed M5 work-item, start-work, review, provider, help, and support / export
//! surface — provider ownership, local-versus-provider commit state, linked branch / worktree / review
//! identity, relation source, blocker / resolution state, and validation evidence — and auto-narrows any
//! profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a work-item owner, a start-work / handoff flow, a provider handoff
//! consumer, or a support / export consumer reads a change intent through (a fully-certified change-intent lane; a
//! reviewable change-intent record structure; a local-only-or-reconcile-required commit-state profile; an
//! undisclosed-start-work-side-effect profile; a flattened-linked-relation-source profile; a
//! blocked-handoff-publishability profile; a local-only-resolution-authority profile; and an
//! unresolved-blocker-continuity profile), not on the underlying object class or implement lane.
//! Each [`ChangeIntentProfileCertificationRow`] certifies one profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! change-intent-truth behavior — and either passes (green), auto-narrows its change-intent claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh certified
//! claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedChangeIntentTruth` / `ReviewableChangeIntentRecord` claim while one of its truth axes is not current is
//! over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound reason
//! and a frozen downgrade trigger) is honestly yellow. Only a fully-certified change-intent lane — one whose
//! provider ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation
//! source, blocker / resolution state, and validation evidence all converge on one export-safe, provider-committed,
//! internally consistent change-intent record — may certify a `CertifiedChangeIntentTruth` claim; a reviewable,
//! local-only-commit, undisclosed-side-effect, flattened-relation, blocked-handoff, local-only-resolution, or
//! unresolved-blocker profile that keeps a certified claim is over-reaching and blocks. The always-on CLI/export
//! axis must always stay certified so support and automation can reconstruct the provider ownership,
//! local-versus-provider commit state, linked engineering identity, relation source, blocker / resolution state,
//! and validation evidence from the same change-intent proof the operator saw.
//!
//! The B153 hard invariants are enforced per row: no profile may let start work silently create a branch,
//! worktree, review draft, or provider link without separately disclosing each side effect; let a local handoff
//! packet or queued publish masquerade as a provider-committed update; flatten linked-by-provider, linked-locally,
//! suggested-by-Aureline, and stale-or-broken relation into one generic relation badge; auto-resolve tracked work
//! while engineering blockers remain unresolved; or drop local notes, handoff packets, or linked evidence when a
//! provider write fails. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical change-intent lifecycle matrix proof bundle
//! ([`CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen change-intent lifecycle matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/teamwork/m5-change-intent-surface-certification.schema.json`](../../../../schemas/teamwork/m5-change-intent-surface-certification.schema.json).
//! The contract doc is
//! [`docs/team-workflows/m5-change-intent-surface-certification.md`](../../../../docs/team-workflows/m5-change-intent-surface-certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_change_intent_and_engineering_lifecycle_matrix as matrix;
use matrix::{M5ChangeIntentDowngradeTrigger, M5ChangeIntentObject};

/// Schema version stamped on the M05-1293 certification packet.
pub const CHANGE_INTENT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ChangeIntentProfileCertificationPacket`].
pub const CHANGE_INTENT_CERT_RECORD_KIND: &str = "m5_change_intent_surface_certification_packet";

/// Stable record-kind tag carried by each [`ChangeIntentProfileCertificationRow`].
pub const CHANGE_INTENT_CERT_ROW_RECORD_KIND: &str = "m5_change_intent_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const CHANGE_INTENT_CERT_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-intent-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const CHANGE_INTENT_CERT_DOC_REF: &str =
    "docs/team-workflows/m5-change-intent-surface-certification.md";

/// Repo-relative path of the frozen change-intent lifecycle matrix schema the certified profiles render.
pub const CHANGE_INTENT_CERT_MATRIX_REF: &str = matrix::M5_CHANGE_INTENT_MATRIX_SCHEMA_REF;

/// The one canonical change-intent lifecycle matrix proof bundle every certified profile cites as its
/// first-resolved change-intent truth. All eight profiles point back to it rather than cloning per-profile
/// evidence.
pub const CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_CHANGE_INTENT_ARTIFACT_REF;

/// The change-intent-health dashboard the release surfaces consume. Recorded as a supporting evidence ref on
/// every row so the certification's change-intent truth ties back to the same dashboard consumers read.
pub const CHANGE_INTENT_CERT_CONSUMERS_BUNDLE_REF: &str = matrix::M5_CHANGE_INTENT_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CHANGE_INTENT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-intent-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CHANGE_INTENT_CERT_CSV_REF: &str =
    "artifacts/release/m5-change-intent-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CHANGE_INTENT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-change-intent-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const CHANGE_INTENT_CERT_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-intent-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const CHANGE_INTENT_CERT_PACKET_ID: &str = "m5-change-intent-surface-certification:stable:0001";

/// The eight claimed M5 change-intent consumer profiles this capstone certifies. Keyed on the profile a
/// work-item owner, a start-work / handoff flow, a provider handoff consumer, or a support / export consumer
/// reads a change intent through — a fully-certified change-intent lane, a reviewable change-intent record
/// structure, a local-only-or-reconcile-required commit-state profile, an undisclosed-start-work-side-effect
/// profile, a flattened-linked-relation-source profile, a blocked-handoff-publishability profile, a
/// local-only-resolution-authority profile, and an unresolved-blocker-continuity profile — not on the reusable
/// object class it renders. Only a fully-certified change-intent lane profile may certify a certified
/// change-intent claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentCertifiedProfile {
    /// A fully-certified change-intent lane — a tracked change intent whose provider ownership,
    /// local-versus-provider commit state, linked branch / worktree / review identity, relation source,
    /// blocker / resolution state, and validation evidence all converge on one export-safe, provider-committed,
    /// internally consistent change-intent record that stays identical across every work-item, start-work,
    /// review, provider, help, and support / export consumer, certifying the change-intent claim exactly right now.
    CertifiedChangeIntentLane,
    /// A reviewable change-intent record structure: a self-sufficient, inspectable ready-for-review handoff /
    /// evidence record (a tracked-item-bound record an operator can review), never itself a fully-certified
    /// change-intent lane.
    ReviewableChangeIntentRecordStructure,
    /// A change-intent lane whose local-versus-provider commit state can no longer be confirmed
    /// provider-committed — the intent is a local-only draft, a queued publish, or a reconcile-required view;
    /// the claim narrows to a commit-state-unverified projection that discloses the last-known commit state and
    /// marks the intent local-only, never a local draft silently looking provider-committed.
    LocalOnlyOrReconcileCommitStateProfile,
    /// A start-work lane whose branch / worktree / review-draft / provider-link side effects cannot be confirmed
    /// separately disclosed; the claim narrows to a side-effect-disclosure-unverified projection that keeps each
    /// pending side effect explicit and never lets start work silently create one.
    UndisclosedStartWorkSideEffectProfile,
    /// A linked-change lane whose relation source (linked-by-provider, linked-locally, suggested-by-Aureline, or
    /// stale-or-broken) cannot be confirmed distinct; the claim narrows to a linked-relation-source-unverified
    /// projection that keeps each relation source explicit and never flattens them into one relation badge.
    FlattenedLinkedRelationSourceProfile,
    /// A ready-for-review handoff lane whose publishability is blocked (offline, missing write scope,
    /// policy-blocked, or partially writable); the claim narrows to a handoff-publishability-unverified projection
    /// that keeps the handoff labelled as a local packet and never lets it masquerade as a provider-committed update.
    BlockedHandoffPublishabilityProfile,
    /// A resolve-or-close lane whose final-resolution authority is local-only or has an unresolved engineering
    /// blocker; the claim narrows to a resolution-authority-unverified projection that keeps the authority and any
    /// open blocker explicit, never auto-resolving tracked work while a blocker remains.
    LocalOnlyResolutionAuthorityProfile,
    /// A blocked-or-escalate lane whose blocker / resolution state is unstated or whose retained local evidence is
    /// at risk after a failed provider write; the claim narrows to a blocker-continuity-unverified projection that
    /// keeps the blocker state, escalation path, and retained local notes / handoff packet / linked evidence
    /// explicit, never dropping evidence when a provider write fails.
    UnresolvedBlockerContinuityProfile,
}

impl M5ChangeIntentCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5ChangeIntentCertifiedProfile; 8] = [
        M5ChangeIntentCertifiedProfile::CertifiedChangeIntentLane,
        M5ChangeIntentCertifiedProfile::ReviewableChangeIntentRecordStructure,
        M5ChangeIntentCertifiedProfile::LocalOnlyOrReconcileCommitStateProfile,
        M5ChangeIntentCertifiedProfile::UndisclosedStartWorkSideEffectProfile,
        M5ChangeIntentCertifiedProfile::FlattenedLinkedRelationSourceProfile,
        M5ChangeIntentCertifiedProfile::BlockedHandoffPublishabilityProfile,
        M5ChangeIntentCertifiedProfile::LocalOnlyResolutionAuthorityProfile,
        M5ChangeIntentCertifiedProfile::UnresolvedBlockerContinuityProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedChangeIntentLane => "certified_change_intent_lane",
            Self::ReviewableChangeIntentRecordStructure => {
                "reviewable_change_intent_record_structure"
            }
            Self::LocalOnlyOrReconcileCommitStateProfile => {
                "local_only_or_reconcile_commit_state_profile"
            }
            Self::UndisclosedStartWorkSideEffectProfile => {
                "undisclosed_start_work_side_effect_profile"
            }
            Self::FlattenedLinkedRelationSourceProfile => {
                "flattened_linked_relation_source_profile"
            }
            Self::BlockedHandoffPublishabilityProfile => "blocked_handoff_publishability_profile",
            Self::LocalOnlyResolutionAuthorityProfile => "local_only_resolution_authority_profile",
            Self::UnresolvedBlockerContinuityProfile => "unresolved_blocker_continuity_profile",
        }
    }

    /// True only for the fully-certified change-intent lane profile. A certified change-intent claim may be
    /// certified on this profile alone; every other profile is at most a reviewable change-intent record structure
    /// or a narrowed projection.
    pub const fn is_certified_change_intent_lane(self) -> bool {
        matches!(self, Self::CertifiedChangeIntentLane)
    }
}

/// The claim ladder a certified change-intent profile asserts and is certified down to. Minted locally for this
/// capstone: the strongest claim is a fully certified change-intent record; each weaker tier is a disclosed
/// projection that keeps the last-known commit-state, side-effect-disclosure, linked-relation-source,
/// handoff-publishability, resolution-authority, or blocker-continuity posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentCertClaim {
    /// Certified change-intent truth: a fully-certified change intent whose provider ownership,
    /// local-versus-provider commit state, linked branch / worktree / review identity, relation source,
    /// blocker / resolution state, and validation evidence all join to one export-safe, provider-committed,
    /// internally consistent record — the strongest claim, the change-intent handling Aureline can present as
    /// cleanly-tracked and publish-safe across every consumer.
    CertifiedChangeIntentTruth,
    /// Reviewable change-intent record: a self-sufficient, inspectable tracked-item-bound record (a handoff /
    /// evidence record an operator can inspect) that is not itself a fully-certified change-intent lane.
    ReviewableChangeIntentRecord,
    /// Commit-state-unverified projection: a record's local-versus-provider commit state cannot be confirmed
    /// provider-committed; the lane stays a commit-state-unverified projection that discloses the last-known
    /// commit state and marks the intent local-only or reconcile-required, never a local draft looking committed.
    CommitStateUnverifiedProjection,
    /// Side-effect-disclosure-unverified projection: a start-work side effect (branch, worktree, review draft, or
    /// provider link) cannot be confirmed separately disclosed; the lane stays a side-effect-disclosure-unverified
    /// projection that keeps each pending side effect explicit and never lets start work silently create one.
    SideEffectDisclosureUnverifiedProjection,
    /// Linked-relation-source-unverified projection: a linked-change relation source cannot be confirmed distinct;
    /// the lane stays a linked-relation-source-unverified projection that keeps the linked-by-provider /
    /// linked-locally / suggested-by-Aureline / stale-or-broken class explicit, never flattened into one badge.
    LinkedRelationSourceUnverifiedProjection,
    /// Handoff-publishability-unverified projection: a ready-for-review handoff's publishability is blocked;
    /// the lane stays a handoff-publishability-unverified projection that keeps the handoff labelled as a local
    /// packet with its publish-later fallback, never masquerading as a provider-committed update.
    HandoffPublishabilityUnverifiedProjection,
    /// Resolution-authority-unverified projection: a resolve-or-close sheet's final-resolution authority is
    /// local-only or an engineering blocker remains; the lane stays a resolution-authority-unverified projection
    /// that keeps the authority and any open blocker explicit, never auto-resolving over an open blocker.
    ResolutionAuthorityUnverifiedProjection,
    /// Blocker-continuity-unverified projection: a blocked-or-escalate card's blocker state or retained local
    /// evidence cannot be confirmed; the lane stays a blocker-continuity-unverified projection that keeps the
    /// blocker state, escalation path, and retained local notes / handoff packet / linked evidence explicit.
    BlockerContinuityUnverifiedProjection,
}

impl M5ChangeIntentCertClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::CertifiedChangeIntentTruth,
        Self::ReviewableChangeIntentRecord,
        Self::CommitStateUnverifiedProjection,
        Self::SideEffectDisclosureUnverifiedProjection,
        Self::LinkedRelationSourceUnverifiedProjection,
        Self::HandoffPublishabilityUnverifiedProjection,
        Self::ResolutionAuthorityUnverifiedProjection,
        Self::BlockerContinuityUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedChangeIntentTruth => 7,
            Self::ReviewableChangeIntentRecord => 6,
            Self::CommitStateUnverifiedProjection => 5,
            Self::SideEffectDisclosureUnverifiedProjection => 4,
            Self::LinkedRelationSourceUnverifiedProjection => 3,
            Self::HandoffPublishabilityUnverifiedProjection => 2,
            Self::ResolutionAuthorityUnverifiedProjection => 1,
            Self::BlockerContinuityUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-certified, certified change-intent record.
    pub const fn asserts_certified_change_intent_truth(self) -> bool {
        matches!(self, Self::CertifiedChangeIntentTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedChangeIntentTruth | Self::ReviewableChangeIntentRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedChangeIntentTruth => "certified_change_intent_truth",
            Self::ReviewableChangeIntentRecord => "reviewable_change_intent_record",
            Self::CommitStateUnverifiedProjection => "commit_state_unverified_projection",
            Self::SideEffectDisclosureUnverifiedProjection => {
                "side_effect_disclosure_unverified_projection"
            }
            Self::LinkedRelationSourceUnverifiedProjection => {
                "linked_relation_source_unverified_projection"
            }
            Self::HandoffPublishabilityUnverifiedProjection => {
                "handoff_publishability_unverified_projection"
            }
            Self::ResolutionAuthorityUnverifiedProjection => {
                "resolution_authority_unverified_projection"
            }
            Self::BlockerContinuityUnverifiedProjection => {
                "blocker_continuity_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and change-intent-truth behavior. The CLI/export axis is always-on and must stay
/// certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentCertificationAxis {
    /// Visual parity: the provider ownership, local-versus-provider commit state, linked branch / worktree /
    /// review identity, relation source, blocker / resolution state, and validation evidence are shown on the
    /// primary surface without relying on a shell-chrome-only affordance or a mislabeled provider-committed-looking
    /// row alone, and no local-only draft or queued publish still reads as a provider-committed update.
    Visual,
    /// Keyboard-reach parity: the same change-intent truth and its bound start-work / handoff / resolve operations
    /// are reachable and operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled provider-committed-looking row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the provider
    /// ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source,
    /// or blocker state.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// commit-state badge, relation-source class, or blocker / resolution state.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// commit state, relation source, blocker state, or resolution authority when a locale is
    /// incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text / JSON / Markdown
    /// for support and automation.
    CliExport,
    /// Degraded-state parity: a local-only-or-reconcile-required commit state, an undisclosed start-work side
    /// effect, a flattened or stale relation source, a blocked handoff publishability, a local-only resolution
    /// authority, or an unresolved blocker honestly downgrades a `CertifiedChangeIntentTruth` /
    /// `ReviewableChangeIntentRecord` claim rather than reading as a fresh, provider-committed change-intent record.
    DegradedState,
    /// Change-intent-truth parity: the provider ownership, local-versus-provider commit state, linked branch /
    /// worktree / review identity, relation source, blocker / resolution state, and validation evidence stay
    /// explicit and never let a local handoff packet or queued publish masquerade as a provider-committed update;
    /// let start work silently create a branch, worktree, review draft, or provider link without disclosure;
    /// flatten linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one relation
    /// badge; auto-resolve tracked work while engineering blockers remain unresolved; or drop local notes, handoff
    /// packets, or linked evidence when a provider write fails.
    ChangeIntentTruth,
}

impl ChangeIntentCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ChangeIntentCertificationAxis; 9] = [
        ChangeIntentCertificationAxis::Visual,
        ChangeIntentCertificationAxis::Keyboard,
        ChangeIntentCertificationAxis::ScreenReader,
        ChangeIntentCertificationAxis::HighZoomReflow,
        ChangeIntentCertificationAxis::HighContrast,
        ChangeIntentCertificationAxis::Localization,
        ChangeIntentCertificationAxis::CliExport,
        ChangeIntentCertificationAxis::DegradedState,
        ChangeIntentCertificationAxis::ChangeIntentTruth,
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
            Self::ChangeIntentTruth => "change_intent_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl ChangeIntentAxisCertificationState {
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
pub enum ChangeIntentProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-lane change-intent profile claims a certified change-intent record, or the narrowing is inconsistent.
    Red,
}

impl ChangeIntentProfileClaimStatus {
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

/// The five B153 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentCertGuardrails {
    /// True if the profile lets start work silently create a branch, worktree, review draft, or provider link
    /// without separately disclosing each side effect. Must be false.
    pub lets_start_work_silently_create_a_side_effect_without_disclosure: bool,
    /// True if the profile lets a local handoff packet or queued publish masquerade as a provider-committed
    /// update. Must be false.
    pub lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update:
        bool,
    /// True if the profile flattens linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken
    /// into one generic relation badge. Must be false.
    pub flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge:
        bool,
    /// True if the profile auto-resolves tracked work while engineering blockers remain unresolved. Must be false.
    pub auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: bool,
    /// True if the profile drops local notes, handoff packets, or linked evidence when a provider write fails.
    /// Must be false.
    pub drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails: bool,
}

impl ChangeIntentCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_start_work_silently_create_a_side_effect_without_disclosure: false,
        lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update:
            false,
        flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge:
            false,
        auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved: false,
        drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_start_work_silently_create_a_side_effect_without_disclosure
            && !self.lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update
            && !self.flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge
            && !self.auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved
            && !self.drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The provider-ownership / commit-state / linked-engineering-identity / relation-source / blocker-state /
    /// validation-evidence fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl ChangeIntentCertExportParity {
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
pub struct ChangeIntentAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ChangeIntentCertificationAxis,
    /// The certification state of the axis.
    pub state: ChangeIntentAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ChangeIntentDowngradeTrigger>,
}

impl ChangeIntentAxisOutcome {
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
            ChangeIntentAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ChangeIntentAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ChangeIntentAxisCertificationState::UndisclosedDrift => {
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
pub struct ChangeIntentClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ChangeIntentCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5ChangeIntentCertClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5ChangeIntentCertClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 change-intent object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentProfileCertificationRow {
    /// Record kind; must equal [`CHANGE_INTENT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CHANGE_INTENT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5ChangeIntentCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5ChangeIntentCertClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5ChangeIntentCertClaim,
    /// The frozen change-intent object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ChangeIntentObject>,
    /// One outcome per [`ChangeIntentCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ChangeIntentAxisOutcome>,
    /// The B153 hard invariants; all must hold.
    pub guardrails: ChangeIntentCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ChangeIntentClaimAutoNarrow>,
    /// The one canonical change-intent lifecycle matrix proof bundle this profile cites. Must equal
    /// [`CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ChangeIntentProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: ChangeIntentCertExportParity,
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

impl ChangeIntentProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ChangeIntentCertificationAxis) -> Option<&ChangeIntentAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ChangeIntentCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ChangeIntentCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ChangeIntentAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ChangeIntentCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ChangeIntentAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-certified change-intent lane
    /// profile may certify a certified change-intent record, every hard invariant must hold, CLI/export parity must
    /// always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ChangeIntentProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ChangeIntentProfileClaimStatus::Red;
        }

        // Every B153 hard invariant must hold.
        if !self.guardrails.all_held() {
            return ChangeIntentProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ChangeIntentProfileClaimStatus::Red;
        }

        // Only a fully-certified change-intent lane profile may certify a certified change-intent record.
        if self.certified_claim.asserts_certified_change_intent_truth()
            && !self.profile.is_certified_change_intent_lane()
        {
            return ChangeIntentProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ChangeIntentCertificationAxis::CliExport) {
            Some(o) if o.state == ChangeIntentAxisCertificationState::Certified => {}
            _ => return ChangeIntentProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ChangeIntentAxisCertificationState::UndisclosedDrift)
        {
            return ChangeIntentProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ChangeIntentProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ChangeIntentProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ChangeIntentProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ChangeIntentProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return ChangeIntentProfileClaimStatus::Red;
        }

        ChangeIntentProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CHANGE_INTENT_CERT_ROW_RECORD_KIND
            && self.schema_version == CHANGE_INTENT_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1293 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentProfileCertificationSummary {
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

/// Constructor input for [`ChangeIntentProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeIntentProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ChangeIntentProfileCertificationRow>,
}

/// Checked-in M05-1293 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ChangeIntentProfileCertificationRow>,
    pub summary: ChangeIntentProfileCertificationSummary,
}

impl ChangeIntentProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ChangeIntentProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CHANGE_INTENT_CERT_SCHEMA_VERSION,
            record_kind: CHANGE_INTENT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ChangeIntentProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5ChangeIntentCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Review-pack object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ChangeIntentObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5ChangeIntentCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen change-intent object class is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ChangeIntentObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ChangeIntentCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ChangeIntentAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ChangeIntentProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeIntentProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeIntentProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeIntentProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ChangeIntentProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        ChangeIntentProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ChangeIntentProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ChangeIntentCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CHANGE_INTENT_CERT_SCHEMA_VERSION {
            violations.push(ChangeIntentCertificationViolation::SchemaVersion {
                expected: CHANGE_INTENT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CHANGE_INTENT_CERT_RECORD_KIND {
            violations.push(ChangeIntentCertificationViolation::RecordKind {
                expected: CHANGE_INTENT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ChangeIntentCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ChangeIntentCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ChangeIntentCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ChangeIntentCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ChangeIntentCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ChangeIntentCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ChangeIntentCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B153 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(ChangeIntentCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a fully-certified change-intent lane profile may certify a certified change-intent record.
            if row.certified_claim.asserts_certified_change_intent_truth()
                && !row.profile.is_certified_change_intent_lane()
            {
                violations.push(
                    ChangeIntentCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ChangeIntentCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    ChangeIntentCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ChangeIntentCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ChangeIntentCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == ChangeIntentProfileClaimStatus::Red {
                violations.push(ChangeIntentCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(ChangeIntentCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen change-intent object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(ChangeIntentCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ChangeIntentCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ChangeIntentCertificationViolation::RawChangeIntentMaterialInExport);
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
        out.push_str("# M5 Change-Intent Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5ChangeIntentCertifiedProfile::ALL.len(),
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
pub fn current_m5_change_intent_surface_certification_export(
) -> Result<ChangeIntentProfileCertificationPacket, ChangeIntentCertificationArtifactError> {
    let packet: ChangeIntentProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-change-intent-surface-certification/support_export.json"
        )))
        .map_err(ChangeIntentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ChangeIntentCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ChangeIntentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ChangeIntentCertificationViolation>),
}

impl fmt::Display for ChangeIntentCertificationArtifactError {
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

impl Error for ChangeIntentCertificationArtifactError {}

/// Validation failure for M05-1293 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeIntentCertificationViolation {
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
    RawChangeIntentMaterialInExport,
}

impl fmt::Display for ChangeIntentCertificationViolation {
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
                    "packet does not cite the canonical change-intent lifecycle matrix proof bundle"
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
                    "row {id} does not cite the one canonical change-intent lifecycle matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B153 hard invariant: letting start work silently create a branch, \
worktree, review draft, or provider link without separately disclosing each side effect; letting a local \
handoff packet or queued publish masquerade as a provider-committed update; flattening linked-by-provider, \
linked-locally, suggested-by-Aureline, and stale-or-broken into one generic relation badge; auto-resolving \
tracked work while engineering blockers remain unresolved; or dropping local notes, handoff packets, or linked \
evidence when a provider write fails"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified change-intent record on a non-lane profile"
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
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified change-intent record, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 change-intent profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen change-intent object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawChangeIntentMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for ChangeIntentCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ChangeIntentAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ChangeIntentAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the change-intent generics
/// the spec forbids collapsing distinct commit-state, side-effect-disclosure, relation-source,
/// handoff-publishability, resolution-authority, and blocker-continuity truth into (whole-label matches so a full
/// sentence naming a concrete commit state, relation source, or blocker state is not flagged).
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
            | "change intent"
            | "change-intent"
            | "intent"
            | "record"
            | "work item"
            | "tracked item"
            | "owner"
            | "ownership"
            | "provider ownership"
            | "provider"
            | "provider committed"
            | "local"
            | "local draft"
            | "local only"
            | "commit state"
            | "queued publish"
            | "relation"
            | "relation source"
            | "linked"
            | "linked change"
            | "linked locally"
            | "suggested"
            | "branch"
            | "worktree"
            | "review draft"
            | "side effect"
            | "handoff"
            | "handoff packet"
            | "publish"
            | "publish later"
            | "resolve"
            | "resolution"
            | "resolution authority"
            | "blocker"
            | "blocked state"
            | "escalate"
            | "escalation"
            | "evidence"
            | "validation evidence"
            | "reconcile"
            | "export"
            | "export fallback"
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the change-intent lifecycle
/// matrix heuristic so the reused [`M5ChangeIntentDowngradeTrigger`] narrowings serialize cleanly — the
/// change-intent proof grammar carries only typed class tokens and opaque refs, never raw secret values or
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

/// Builds the canonical, checked-in M05-1293 certification packet. Certifies all eight claimed M5 change-intent
/// profiles: two deliver their claim (green) and six auto-narrow a not-current truth axis to a weaker
/// configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_change_intent_surface_certification_packet(
) -> ChangeIntentProfileCertificationPacket {
    ChangeIntentProfileCertificationPacket::new(ChangeIntentProfileCertificationPacketInput {
        packet_id: CHANGE_INTENT_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: CHANGE_INTENT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:change-intent-surface-certification:{id}"),
        CHANGE_INTENT_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ChangeIntentCertExportParity {
    ChangeIntentCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ChangeIntentCertificationAxis) -> &'static str {
    match axis {
        ChangeIntentCertificationAxis::Visual => {
            "provider ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source, blocker / resolution state, and validation evidence shown on-surface without a shell-chrome-only affordance or a mislabeled provider-committed-looking row alone, and no local-only draft or queued publish still reads as a provider-committed update"
        }
        ChangeIntentCertificationAxis::Keyboard => {
            "the same commit state, linked engineering identity, relation source, and bound start-work / handoff / resolve operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        ChangeIntentCertificationAxis::ScreenReader => {
            "the same change-intent truth is announced non-visually, never a shell-chrome-only / mislabeled-provider-committed-row / unlabeled-control-only cue"
        }
        ChangeIntentCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the provider ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source, or blocker state"
        }
        ChangeIntentCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the commit-state badge, relation-source class, or blocker / resolution state"
        }
        ChangeIntentCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a commit state, relation source, blocker state, or resolution authority"
        }
        ChangeIntentCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        ChangeIntentCertificationAxis::DegradedState => {
            "a local-only-or-reconcile-required commit state, an undisclosed start-work side effect, a flattened or stale relation source, a blocked handoff publishability, a local-only resolution authority, or an unresolved blocker honestly downgrades the CertifiedChangeIntentTruth/ReviewableChangeIntentRecord claim rather than reading as a fresh, provider-committed change-intent record"
        }
        ChangeIntentCertificationAxis::ChangeIntentTruth => {
            "provider ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source, blocker / resolution state, and validation evidence stay explicit and never let a local handoff packet or queued publish masquerade as a provider-committed update, let start work silently create a branch / worktree / review draft / provider link without disclosure, flatten linked-by-provider / linked-locally / suggested-by-Aureline / stale-or-broken into one relation badge, auto-resolve tracked work while engineering blockers remain unresolved, or drop local notes / handoff packets / linked evidence when a provider write fails"
        }
    }
}

fn seed_certified(axis: ChangeIntentCertificationAxis) -> ChangeIntentAxisOutcome {
    ChangeIntentAxisOutcome {
        axis,
        state: ChangeIntentAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ChangeIntentCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ChangeIntentDowngradeTrigger,
) -> ChangeIntentAxisOutcome {
    ChangeIntentAxisOutcome {
        axis,
        state: ChangeIntentAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ChangeIntentAxisOutcome> {
    ChangeIntentCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ChangeIntentCertificationAxis,
    outcome: ChangeIntentAxisOutcome,
) -> Vec<ChangeIntentAxisOutcome> {
    ChangeIntentCertificationAxis::ALL
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
    profile: M5ChangeIntentCertifiedProfile,
    claimed_claim: M5ChangeIntentCertClaim,
    certified_claim: M5ChangeIntentCertClaim,
    consumed_families: &[M5ChangeIntentObject],
    axis_outcomes: Vec<ChangeIntentAxisOutcome>,
    claim_auto_narrow: Option<ChangeIntentClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ChangeIntentProfileCertificationRow {
    let mut row = ChangeIntentProfileCertificationRow {
        record_kind: CHANGE_INTENT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: CHANGE_INTENT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: ChangeIntentCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: CHANGE_INTENT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ChangeIntentProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            CHANGE_INTENT_CERT_MATRIX_REF.to_owned(),
            CHANGE_INTENT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ChangeIntentCertificationAxis,
    from_claim: M5ChangeIntentCertClaim,
    to_claim: M5ChangeIntentCertClaim,
    label: &str,
) -> ChangeIntentClaimAutoNarrow {
    ChangeIntentClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ChangeIntentProfileCertificationRow> {
    use ChangeIntentCertificationAxis as Ax;
    use M5ChangeIntentCertClaim::*;
    use M5ChangeIntentCertifiedProfile as P;
    use M5ChangeIntentDowngradeTrigger as Trig;
    use M5ChangeIntentObject::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-change-intent-lane",
            P::CertifiedChangeIntentLane,
            CertifiedChangeIntentTruth,
            CertifiedChangeIntentTruth,
            &[ChangeIntentRecord],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "provider_commit_state",
            ],
            &[
                "certified change-intent lane: the provider ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source, blocker / resolution state, and validation evidence all join to one export-safe, provider-committed change-intent record, never a local-only draft or queued publish that reads as a provider-committed update",
                "the certified change-intent record keeps stable operation IDs while the provider ownership, commit state, linked engineering identity, relation source, and resolution authority bind to the one change-intent lifecycle matrix across work-item-detail / start-work-sheet / linked-change-panel / ready-for-review-handoff / resolve-close-sheet / blocked-escalate-card / support-export / help-docs surfaces, and no intent reads as provider-committed in one surface and local-only in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered change-intent record",
                "change-intent-truth: a fully-certified change-intent lane with export-safe, provider-committed, internally consistent state is the only profile that certifies a certified change-intent record",
            ],
        ),
        seed_row(
            "cert:reviewable-change-intent-record-structure",
            P::ReviewableChangeIntentRecordStructure,
            ReviewableChangeIntentRecord,
            ReviewableChangeIntentRecord,
            &[ReadyForReviewHandoffSheet],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "linked_engineering_identity",
            ],
            &[
                "record-structure class: an export-safe ready-for-review handoff / evidence record bound to one tracked item and inspectable rather than a per-surface description copied by hand, with the linked branch / worktree / review identity and validation evidence kept bound to the change-intent record it came from",
                "the reviewable change-intent record keeps its commit state, linked engineering identity, relation source, and validation evidence inspectable rather than a shell-chrome-only or mislabeled-provider-committed-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable change-intent record structure",
                "change-intent-truth: a reviewable change-intent record never certifies a fully-certified-lane claim and never stays green on a local-only commit state or a missing linked engineering identity",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:local-only-or-reconcile-commit-state-profile",
            P::LocalOnlyOrReconcileCommitStateProfile,
            ReviewableChangeIntentRecord,
            CommitStateUnverifiedProjection,
            &[ChangeIntentRecord],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the record's local-versus-provider commit state is local-only or reconcile-required for this profile so a provider-committed change-intent record cannot be certified and the intent stays inspect-only",
                    "The record's commit state can no longer be confirmed provider-committed — the intent is a local-only draft, a queued publish, or a reconcile-required view that has diverged from the connected provider — so the ReviewableChangeIntentRecord claim narrows to a commit-state-unverified projection and the lane discloses the last-known local-versus-provider commit state rather than letting a local-only draft or queued publish read as a provider-committed update",
                    Trig::LocalVersusProviderStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableChangeIntentRecord,
                CommitStateUnverifiedProjection,
                "The commit state is local-only or reconcile-required for this record, so its last-known local-versus-provider state is disclosed and it never reads as a provider-committed, freshly-published update",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "local-only-commit class: the record names its tracked item, actor lineage, and last-known local-versus-provider commit state and marks the intent local-only or reconcile-required rather than letting a local draft read as provider-committed when its commit state is unconfirmed",
                "the local-only-commit surface keeps its tracked item and last-known commit state legible while the intent is disclosed as local-only",
                "degraded-state: ReviewableChangeIntentRecord narrows to a commit-state-unverified projection (auto-narrowed)",
                "change-intent-truth: a local-only draft or queued publish never looks provider-committed — its commit state is preserved and it never reads as an authoritative tracked-item update",
            ],
        ),
        seed_row(
            "cert:undisclosed-start-work-side-effect-profile",
            P::UndisclosedStartWorkSideEffectProfile,
            ReviewableChangeIntentRecord,
            SideEffectDisclosureUnverifiedProjection,
            &[StartWorkSheet],
            seed_certified_except(
                Ax::ChangeIntentTruth,
                seed_narrowed(
                    Ax::ChangeIntentTruth,
                    "a start-work side effect (branch, worktree, review draft, or provider link) cannot be confirmed separately disclosed for this profile so a provider-committed change-intent record cannot be certified and the side effect stays inspect-only",
                    "A start-work side effect — a created branch, worktree, review draft, or provider link — cannot be confirmed as separately disclosed before commit, so the ReviewableChangeIntentRecord claim narrows to a side-effect-disclosure-unverified projection and the lane keeps each pending side effect explicit rather than letting start work silently create a branch, worktree, review draft, or provider link without disclosure",
                    Trig::SilentSideEffectCreated,
                ),
            ),
            Some(seed_narrow(
                Ax::ChangeIntentTruth,
                ReviewableChangeIntentRecord,
                SideEffectDisclosureUnverifiedProjection,
                "The start-work side effects are not confirmed disclosed, so each pending branch / worktree / review-draft / provider-link side effect stays explicit and none is silently created",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "start-work class: the start-work sheet keeps each branch / worktree / review-draft / provider-link side effect explicit and marks the disclosure unverified rather than committing a side effect the operator did not separately approve",
                "the start-work surface keeps its per-side-effect disclosure legible while the side effect is disclosed as unverified",
                "change-intent-truth: ReviewableChangeIntentRecord narrows to a side-effect-disclosure-unverified projection (auto-narrowed)",
                "change-intent-truth: start work never silently creates a branch, worktree, review draft, or provider link — each side effect stays separately disclosed",
            ],
        ),
        seed_row(
            "cert:flattened-linked-relation-source-profile",
            P::FlattenedLinkedRelationSourceProfile,
            ReviewableChangeIntentRecord,
            LinkedRelationSourceUnverifiedProjection,
            &[LinkedChangePanel],
            seed_certified_except(
                Ax::Visual,
                seed_narrowed(
                    Ax::Visual,
                    "the linked-change relation source (linked-by-provider, linked-locally, suggested-by-Aureline, or stale-or-broken) cannot be confirmed distinct for this profile so a provider-committed change-intent record cannot be certified and the relation stays inspect-only",
                    "The linked-change relation source cannot be confirmed distinct — the linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken relations risk collapsing into one badge — so the ReviewableChangeIntentRecord claim narrows to a linked-relation-source-unverified projection and the lane keeps each relation source explicit rather than flattening linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken into one generic relation badge",
                    Trig::RelationSourceUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Visual,
                ReviewableChangeIntentRecord,
                LinkedRelationSourceUnverifiedProjection,
                "The relation source is unverified, so the linked-by-provider / linked-locally / suggested-by-Aureline / stale-or-broken class stays explicit and never flattens into one generic relation badge",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "linked-change class: the linked-change panel keeps its linked-by-provider / linked-locally / suggested-by-Aureline / stale-or-broken relation class explicit and marks the relation source unverified rather than presenting one generic relation badge when the source is unresolved",
                "the linked-change surface keeps its relation-source class legible while the relation is disclosed as unverified",
                "visual: ReviewableChangeIntentRecord narrows to a linked-relation-source-unverified projection (auto-narrowed)",
                "change-intent-truth: linked-by-provider, linked-locally, suggested-by-Aureline, and stale-or-broken stay distinct — no relation badge flattens the source and a stale-or-broken relation stays visible and actionable",
            ],
        ),
        seed_row(
            "cert:blocked-handoff-publishability-profile",
            P::BlockedHandoffPublishabilityProfile,
            ReviewableChangeIntentRecord,
            HandoffPublishabilityUnverifiedProjection,
            &[ReadyForReviewHandoffSheet],
            seed_certified_except(
                Ax::HighZoomReflow,
                seed_narrowed(
                    Ax::HighZoomReflow,
                    "the ready-for-review handoff's publishability is blocked or offline for this profile so a provider-committed change-intent record cannot be certified and the handoff stays a labelled local packet",
                    "The ready-for-review handoff's publishability is blocked — the provider is offline, the write scope is missing, or the publish is policy-blocked or only partially writable — so the ReviewableChangeIntentRecord claim narrows to a handoff-publishability-unverified projection and the lane keeps the handoff labelled as a local packet with its publish-later fallback rather than letting a local handoff packet or queued publish masquerade as a provider-committed update",
                    Trig::LocalHandoffShownAsProviderCommitted,
                ),
            ),
            Some(seed_narrow(
                Ax::HighZoomReflow,
                ReviewableChangeIntentRecord,
                HandoffPublishabilityUnverifiedProjection,
                "The handoff publishability is blocked, so it stays labelled as a local handoff packet with its publish-later fallback explicit and never reads as a provider-committed update",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "ready-for-review class: the ready-for-review handoff sheet keeps its local-packet-versus-provider-committed state and publish-later fallback explicit and marks the handoff a local packet rather than provider-committed when publishability is blocked",
                "the ready-for-review surface keeps its local-packet label and publish-later fallback legible while the handoff is disclosed as a local packet",
                "high-zoom-reflow: ReviewableChangeIntentRecord narrows to a handoff-publishability-unverified projection (auto-narrowed)",
                "change-intent-truth: a local handoff packet or queued publish never masquerades as provider-committed — the packet stays labelled and its validation evidence and publish-later fallback are preserved",
            ],
        ),
        seed_row(
            "cert:local-only-resolution-authority-profile",
            P::LocalOnlyResolutionAuthorityProfile,
            ReviewableChangeIntentRecord,
            ResolutionAuthorityUnverifiedProjection,
            &[ResolveCloseSheet],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the resolve-or-close sheet's final-resolution authority is local-only or an engineering blocker remains unresolved for this profile so a provider-committed change-intent record cannot be certified and the resolution stays inspect-only",
                    "The resolve-or-close sheet's final-resolution authority is local-only, or an engineering blocker remains unresolved, so the ReviewableChangeIntentRecord claim narrows to a resolution-authority-unverified projection and the lane keeps the final-resolution authority and any unresolved blocker explicit rather than auto-resolving tracked work while engineering blockers remain unresolved",
                    Trig::AutoResolvedWithOpenBlocker,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableChangeIntentRecord,
                ResolutionAuthorityUnverifiedProjection,
                "The resolution authority is local-only or a blocker is open, so the final-resolution authority and any unresolved blocker stay explicit and tracked work is never auto-resolved while an engineering blocker remains",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "resolve-close class: the resolve-or-close sheet keeps its final-resolution authority and any unresolved engineering blocker explicit and marks the resolution local-only rather than closing a tracked item the provider has not accepted",
                "the resolve-close surface keeps its resolution-authority and open-blocker state legible while the resolution is disclosed as local-only",
                "localization: ReviewableChangeIntentRecord narrows to a resolution-authority-unverified projection (auto-narrowed)",
                "change-intent-truth: tracked work is never auto-resolved while an engineering blocker remains — the final-resolution authority stays explicit and a local-only resolution never reads as provider-accepted",
            ],
        ),
        seed_row(
            "cert:unresolved-blocker-continuity-profile",
            P::UnresolvedBlockerContinuityProfile,
            ReviewableChangeIntentRecord,
            BlockerContinuityUnverifiedProjection,
            &[BlockedEscalateCard],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "the blocked-or-escalate card's blocker / resolution state is unstated or at risk of dropping local notes for this profile so a provider-committed change-intent record cannot be certified and the blocker stays inspect-only",
                    "The blocked-or-escalate card's blocker / resolution state is unstated, or a provider write failed and its local notes, handoff packet, and linked evidence are at risk, so the ReviewableChangeIntentRecord claim narrows to a blocker-continuity-unverified projection and the lane keeps the blocker state, escalation path, and retained local evidence explicit rather than dropping local notes, handoff packets, or linked evidence when a provider write fails",
                    Trig::BlockerStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableChangeIntentRecord,
                BlockerContinuityUnverifiedProjection,
                "The blocker continuity is unverified, so the blocker state, escalation path, and retained local notes / handoff packet / linked evidence stay explicit and none is dropped when a provider write fails",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "blocked-escalate class: the blocked-or-escalate card keeps its blocker state, escalation path, and retained local notes / handoff packet / linked evidence explicit and marks the continuity unverified rather than presenting a blocker without its retained evidence",
                "the blocked-escalate surface keeps its blocker state, escalation path, and retained local evidence legible non-visually while continuity is disclosed as unverified",
                "screen-reader: ReviewableChangeIntentRecord narrows to a blocker-continuity-unverified projection (auto-narrowed)",
                "change-intent-truth: a blocker never drops its evidence — the blocker state, escalation path, and retained local notes / handoff packet / linked evidence stay explicit and survive a failed provider write, export, and reopen",
            ],
        ),
    ]
}
