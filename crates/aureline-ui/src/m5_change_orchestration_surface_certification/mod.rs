//! M05-1303 closing B154 surface certification over the frozen M5 change-object, patch-stack, and landing
//! matrix — the explicit change object, patch-stack / queue, stack-edit / review sheet, landing-candidate sheet,
//! portable shelf / bundle, and worktree cleanup preview that a Git, review, AI, work-item, provider-handoff,
//! help / docs, or support / export consumer must treat as first-class, durable, export-safe change-orchestration
//! objects rather than ambient branch state.
//!
//! Where the freeze matrix ([`crate::m5_change_object_patch_stack_and_landing_matrix`]) defines the six
//! governed change-orchestration object classes, the M05-1295..1300 implement lanes resolve each change-object
//! record / selected-change binding, patch-stack / member landing, stack-edit review / disposition,
//! landing-candidate sheet / authorization, portable shelf / reopen parity, and worktree-manager / cleanup-preview
//! registry; this closing capstone *certifies* that the shared change-orchestration truth holds on every claimed
//! M5 Git, review, AI, work-item, provider, help, and support / export surface — the selected change object, its
//! worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup
//! evidence — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a change-object owner, a stack / landing flow, a provider merge-queue
//! consumer, or a support / export consumer reads a change orchestration through (a fully-certified
//! change-orchestration lane; a reviewable change-object record structure; an unbound-worktree-binding profile;
//! an inferred-stack-membership profile; a silently-reordered-stack profile; an ambient-branch-landing profile; a
//! stale-validation-shelf profile; and a partial-cleanup-evidence profile), not on the underlying object class
//! or implement lane.
//! Each [`ChangeOrchestrationProfileCertificationRow`] certifies one profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! change-orchestration-truth behavior — and either passes (green), auto-narrows its change-orchestration claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh certified
//! claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedChangeOrchestrationTruth` / `ReviewableChangeOrchestrationRecord` claim while one of its truth axes is not current is
//! over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound reason
//! and a frozen downgrade trigger) is honestly yellow. Only a fully-certified change-orchestration lane — one whose
//! selected change object, worktree / base identity, stack membership and order, landing state, validation
//! freshness, and cleanup evidence all converge on one export-safe, provider-authoritative,
//! internally consistent change-orchestration record — may certify a `CertifiedChangeOrchestrationTruth` claim; a reviewable,
//! unbound-worktree, inferred-membership, silently-reordered, ambient-branch-landing, stale-validation, or
//! partial-cleanup profile that keeps a certified claim is over-reaching and blocks. The always-on CLI/export
//! axis must always stay certified so support and automation can reconstruct the selected change object,
//! worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup
//! evidence from the same change-orchestration proof the operator saw.
//!
//! The B154 hard invariants are enforced per row: no profile may infer stack membership from branch names alone;
//! mutate another worktree without an explicit selected change object and worktree binding; silently reorder,
//! collapse, or retarget stack members; land from ambient branch state; or delete an orphaned worktree or stale
//! member without previewing running tasks, open editors, uncommitted changes, recovery checkpoints, and
//! export-safe evidence. A profile that breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical change-orchestration lifecycle matrix proof bundle
//! ([`CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF`]) — the frozen change-orchestration lifecycle matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/teamwork/m5-change-orchestration-surface-certification.schema.json`](../../../../schemas/teamwork/m5-change-orchestration-surface-certification.schema.json).
//! The contract doc is
//! [`docs/team-workflows/m5-change-orchestration-surface-certification.md`](../../../../docs/team-workflows/m5-change-orchestration-surface-certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_change_object_patch_stack_and_landing_matrix as matrix;
use matrix::{M5ChangeOrchestrationDowngradeTrigger, M5ChangeOrchestrationObject};

/// Schema version stamped on the M05-1303 certification packet.
pub const CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ChangeOrchestrationProfileCertificationPacket`].
pub const CHANGE_ORCHESTRATION_CERT_RECORD_KIND: &str =
    "m5_change_orchestration_surface_certification_packet";

/// Stable record-kind tag carried by each [`ChangeOrchestrationProfileCertificationRow`].
pub const CHANGE_ORCHESTRATION_CERT_ROW_RECORD_KIND: &str =
    "m5_change_orchestration_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const CHANGE_ORCHESTRATION_CERT_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-orchestration-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const CHANGE_ORCHESTRATION_CERT_DOC_REF: &str =
    "docs/team-workflows/m5-change-orchestration-surface-certification.md";

/// Repo-relative path of the frozen change-orchestration lifecycle matrix schema the certified profiles render.
pub const CHANGE_ORCHESTRATION_CERT_MATRIX_REF: &str =
    matrix::M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF;

/// The one canonical change-orchestration lifecycle matrix proof bundle every certified profile cites as its
/// first-resolved change-orchestration truth. All eight profiles point back to it rather than cloning per-profile
/// evidence.
pub const CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_CHANGE_ORCHESTRATION_ARTIFACT_REF;

/// The change-orchestration-health dashboard the release surfaces consume. Recorded as a supporting evidence ref on
/// every row so the certification's change-orchestration truth ties back to the same dashboard consumers read.
pub const CHANGE_ORCHESTRATION_CERT_CONSUMERS_BUNDLE_REF: &str =
    matrix::M5_CHANGE_ORCHESTRATION_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CHANGE_ORCHESTRATION_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-orchestration-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CHANGE_ORCHESTRATION_CERT_CSV_REF: &str =
    "artifacts/release/m5-change-orchestration-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CHANGE_ORCHESTRATION_CERT_REPORT_REF: &str =
    "artifacts/release/m5-change-orchestration-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const CHANGE_ORCHESTRATION_CERT_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-orchestration-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const CHANGE_ORCHESTRATION_CERT_PACKET_ID: &str =
    "m5-change-orchestration-surface-certification:stable:0001";

/// The eight claimed M5 change-orchestration consumer profiles this capstone certifies. Keyed on the profile a
/// work-item owner, a start-work / handoff flow, a provider handoff consumer, or a support / export consumer
/// reads a change orchestration through — a fully-certified change-orchestration lane, a reviewable change-object
/// record structure, an unbound-worktree-binding profile, an inferred-stack-membership profile, a
/// silently-reordered-stack profile, an ambient-branch-landing profile, a stale-validation-shelf profile, and a
/// partial-cleanup-evidence profile — not on the reusable object class it renders. Only a fully-certified
/// change-orchestration lane profile may certify a certified change-orchestration claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationCertifiedProfile {
    /// A fully-certified change-orchestration lane — a tracked change object whose selected worktree / base
    /// identity, stack membership and order, landing state, validation freshness, and cleanup evidence all
    /// converge on one export-safe, provider-authoritative, internally consistent change-orchestration record that
    /// stays identical across every Git, review, AI, work-item, provider, help, and support / export consumer,
    /// certifying the change-orchestration claim exactly right now.
    CertifiedChangeOrchestrationLane,
    /// A reviewable change-object record structure: a self-sufficient, inspectable landing-candidate / review
    /// record (a change-object-bound record an operator can review), never itself a fully-certified
    /// change-orchestration lane.
    ReviewableChangeOrchestrationRecordStructure,
    /// A change-object lane whose selected worktree and base-or-dirty-tree identity can no longer be confirmed
    /// bound; the claim narrows to a worktree-binding-unverified projection that discloses the last-known worktree
    /// binding and never lets ambient branch state stand in for a selected change object.
    UnboundWorktreeBindingProfile,
    /// A patch-stack lane whose member stack membership cannot be confirmed explicitly declared; the claim narrows
    /// to a stack-membership-unverified projection that keeps each member's declared membership and dependency
    /// edges explicit and never infers membership from a branch name alone.
    InferredStackMembershipProfile,
    /// A stack-edit lane whose proposed order cannot be confirmed reviewed; the claim narrows to a
    /// stack-order-unverified projection that keeps the original order, proposed order, and affected parent-child
    /// links explicit and never silently reorders, collapses, or retargets stack members.
    SilentlyReorderedStackProfile,
    /// A landing-candidate lane whose queue authority or protected-branch gate cannot be proven (merge queue
    /// unavailable, ambiguous queue position, stale base, or unverifiable protected-branch rule); the claim narrows
    /// to a landing-authority-unverified projection that keeps the reviewed candidate labelled and never lands from
    /// ambient branch state.
    AmbientBranchLandingProfile,
    /// A portable-shelf lane whose packaged validation or approval evidence is stale; the claim narrows to a
    /// validation-freshness-unverified projection that keeps the last-known validation freshness and review-pack
    /// version explicit, never reopening an imported shelf as current provider-authoritative truth.
    StaleValidationShelfProfile,
    /// A worktree-cleanup lane whose affected-work and recovery evidence is partial; the claim narrows to a
    /// cleanup-evidence-unverified projection that keeps the affected running work, uncommitted-change scope, and
    /// reflog / checkpoint recovery explicit, never deleting an orphaned worktree or stale member without preview.
    PartialCleanupEvidenceProfile,
}

impl M5ChangeOrchestrationCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5ChangeOrchestrationCertifiedProfile; 8] = [
        M5ChangeOrchestrationCertifiedProfile::CertifiedChangeOrchestrationLane,
        M5ChangeOrchestrationCertifiedProfile::ReviewableChangeOrchestrationRecordStructure,
        M5ChangeOrchestrationCertifiedProfile::UnboundWorktreeBindingProfile,
        M5ChangeOrchestrationCertifiedProfile::InferredStackMembershipProfile,
        M5ChangeOrchestrationCertifiedProfile::SilentlyReorderedStackProfile,
        M5ChangeOrchestrationCertifiedProfile::AmbientBranchLandingProfile,
        M5ChangeOrchestrationCertifiedProfile::StaleValidationShelfProfile,
        M5ChangeOrchestrationCertifiedProfile::PartialCleanupEvidenceProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedChangeOrchestrationLane => "certified_change_orchestration_lane",
            Self::ReviewableChangeOrchestrationRecordStructure => {
                "reviewable_change_orchestration_record_structure"
            }
            Self::UnboundWorktreeBindingProfile => "unbound_worktree_binding_profile",
            Self::InferredStackMembershipProfile => "inferred_stack_membership_profile",
            Self::SilentlyReorderedStackProfile => "silently_reordered_stack_profile",
            Self::AmbientBranchLandingProfile => "ambient_branch_landing_profile",
            Self::StaleValidationShelfProfile => "stale_validation_shelf_profile",
            Self::PartialCleanupEvidenceProfile => "partial_cleanup_evidence_profile",
        }
    }

    /// True only for the fully-certified change-orchestration lane profile. A certified change-orchestration claim may be
    /// certified on this profile alone; every other profile is at most a reviewable change-orchestration record structure
    /// or a narrowed projection.
    pub const fn is_certified_change_orchestration_lane(self) -> bool {
        matches!(self, Self::CertifiedChangeOrchestrationLane)
    }
}

/// The claim ladder a certified change-orchestration profile asserts and is certified down to. Minted locally for this
/// capstone: the strongest claim is a fully certified change-orchestration record; each weaker tier is a disclosed
/// projection that keeps the last-known commit-state, side-effect-disclosure, linked-relation-source,
/// handoff-publishability, resolution-authority, or blocker-continuity posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationCertClaim {
    /// Certified change-orchestration truth: a fully-certified change orchestration whose selected change object,
    /// worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup
    /// evidence all join to one export-safe, provider-authoritative, internally consistent record — the strongest
    /// claim, the change-orchestration handling Aureline can present as explicitly-bound and land-safe across every
    /// consumer.
    CertifiedChangeOrchestrationTruth,
    /// Reviewable change-object record: a self-sufficient, inspectable change-object-bound record (a landing
    /// candidate / review record an operator can inspect) that is not itself a fully-certified change-orchestration
    /// lane.
    ReviewableChangeOrchestrationRecord,
    /// Worktree-binding-unverified projection: a change object's selected worktree and base-or-dirty-tree identity
    /// cannot be confirmed bound; the lane stays a worktree-binding-unverified projection that discloses the
    /// last-known worktree binding, never letting ambient branch state stand in for a selected change object.
    WorktreeBindingUnverifiedProjection,
    /// Stack-membership-unverified projection: a patch-stack member's stack membership cannot be confirmed
    /// explicitly declared; the lane stays a stack-membership-unverified projection that keeps each member's
    /// declared membership and dependency edges explicit, never inferring membership from a branch name alone.
    StackMembershipUnverifiedProjection,
    /// Stack-order-unverified projection: a stack-edit's proposed order cannot be confirmed reviewed; the lane
    /// stays a stack-order-unverified projection that keeps the original order, proposed order, and affected
    /// parent-child links explicit, never silently reordering, collapsing, or retargeting stack members.
    StackOrderUnverifiedProjection,
    /// Landing-authority-unverified projection: a landing candidate's queue authority or protected-branch gate
    /// cannot be proven; the lane stays a landing-authority-unverified projection that keeps the reviewed candidate
    /// labelled with its target branch and queue posture, never landing from ambient branch state.
    LandingAuthorityUnverifiedProjection,
    /// Validation-freshness-unverified projection: a portable shelf's packaged validation or approval evidence is
    /// stale; the lane stays a validation-freshness-unverified projection that keeps the last-known validation
    /// freshness and review-pack version explicit, never reopening an imported shelf as current hosted truth.
    ValidationFreshnessUnverifiedProjection,
    /// Cleanup-evidence-unverified projection: a worktree cleanup preview's affected-work and recovery evidence is
    /// partial; the lane stays a cleanup-evidence-unverified projection that keeps the affected running work,
    /// uncommitted-change scope, and reflog / checkpoint recovery explicit, never deleting without preview.
    CleanupEvidenceUnverifiedProjection,
}

impl M5ChangeOrchestrationCertClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::CertifiedChangeOrchestrationTruth,
        Self::ReviewableChangeOrchestrationRecord,
        Self::WorktreeBindingUnverifiedProjection,
        Self::StackMembershipUnverifiedProjection,
        Self::StackOrderUnverifiedProjection,
        Self::LandingAuthorityUnverifiedProjection,
        Self::ValidationFreshnessUnverifiedProjection,
        Self::CleanupEvidenceUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedChangeOrchestrationTruth => 7,
            Self::ReviewableChangeOrchestrationRecord => 6,
            Self::WorktreeBindingUnverifiedProjection => 5,
            Self::StackMembershipUnverifiedProjection => 4,
            Self::StackOrderUnverifiedProjection => 3,
            Self::LandingAuthorityUnverifiedProjection => 2,
            Self::ValidationFreshnessUnverifiedProjection => 1,
            Self::CleanupEvidenceUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-certified, certified change-orchestration record.
    pub const fn asserts_certified_change_orchestration_truth(self) -> bool {
        matches!(self, Self::CertifiedChangeOrchestrationTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedChangeOrchestrationTruth | Self::ReviewableChangeOrchestrationRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedChangeOrchestrationTruth => "certified_change_orchestration_truth",
            Self::ReviewableChangeOrchestrationRecord => "reviewable_change_orchestration_record",
            Self::WorktreeBindingUnverifiedProjection => "worktree_binding_unverified_projection",
            Self::StackMembershipUnverifiedProjection => "stack_membership_unverified_projection",
            Self::StackOrderUnverifiedProjection => "stack_order_unverified_projection",
            Self::LandingAuthorityUnverifiedProjection => "landing_authority_unverified_projection",
            Self::ValidationFreshnessUnverifiedProjection => {
                "validation_freshness_unverified_projection"
            }
            Self::CleanupEvidenceUnverifiedProjection => "cleanup_evidence_unverified_projection",
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and change-orchestration-truth behavior. The CLI/export axis is always-on and must stay
/// certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationCertificationAxis {
    /// Visual parity: the selected change object, its worktree / base identity, stack membership and order, landing
    /// state, validation freshness, and cleanup evidence are shown on the primary surface without relying on a
    /// shell-chrome-only affordance or an ambient-branch row alone, and no ambient branch state still reads as a
    /// reviewed landing candidate.
    Visual,
    /// Keyboard-reach parity: the same change-orchestration truth and its bound stack-edit / land / shelf / cleanup
    /// operations are reachable and operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, an ambient-branch row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the selected
    /// change object, worktree / base identity, stack membership and order, landing state, or validation freshness.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// worktree-binding badge, stack-membership-source class, or landing / cleanup state.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// worktree binding, stack-membership source, landing state, or validation freshness when a locale is
    /// incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text / JSON / Markdown
    /// for support and automation.
    CliExport,
    /// Degraded-state parity: an unverifiable worktree binding, an inferred stack membership, a drifted or
    /// silently-reordered stack, an unprovable queue or protected-branch landing authority, stale validation
    /// evidence, or partial cleanup evidence honestly downgrades a `CertifiedChangeOrchestrationTruth` /
    /// `ReviewableChangeOrchestrationRecord` claim rather than reading as a fresh, provider-authoritative change-orchestration record.
    DegradedState,
    /// Change-orchestration-truth parity: the selected change object, worktree / base identity, stack membership
    /// and order, landing state, validation freshness, and cleanup evidence stay explicit and never let ambient
    /// branch state read as a reviewed landing candidate; infer stack membership from branch names alone; mutate
    /// another worktree without a selected change object and worktree binding; silently reorder, collapse, or
    /// retarget stack members; land from ambient branch state; or delete an orphaned worktree or stale member
    /// without previewing running work and recovery.
    ChangeOrchestrationTruth,
}

impl ChangeOrchestrationCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ChangeOrchestrationCertificationAxis; 9] = [
        ChangeOrchestrationCertificationAxis::Visual,
        ChangeOrchestrationCertificationAxis::Keyboard,
        ChangeOrchestrationCertificationAxis::ScreenReader,
        ChangeOrchestrationCertificationAxis::HighZoomReflow,
        ChangeOrchestrationCertificationAxis::HighContrast,
        ChangeOrchestrationCertificationAxis::Localization,
        ChangeOrchestrationCertificationAxis::CliExport,
        ChangeOrchestrationCertificationAxis::DegradedState,
        ChangeOrchestrationCertificationAxis::ChangeOrchestrationTruth,
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
            Self::ChangeOrchestrationTruth => "change_orchestration_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl ChangeOrchestrationAxisCertificationState {
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
pub enum ChangeOrchestrationProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-lane change-orchestration profile claims a certified change-orchestration record, or the narrowing is inconsistent.
    Red,
}

impl ChangeOrchestrationProfileClaimStatus {
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

/// The five B154 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationCertGuardrails {
    /// True if the profile infers stack membership from branch names alone rather than explicitly declared
    /// membership. Must be false.
    pub infers_stack_membership_from_branch_names_alone: bool,
    /// True if the profile mutates another worktree without an explicit selected change object and worktree
    /// binding. Must be false.
    pub mutates_another_worktree_without_a_selected_change_object_and_worktree_binding: bool,
    /// True if the profile silently reorders, collapses, or retargets stack members. Must be false.
    pub silently_reorders_collapses_or_retargets_stack_members: bool,
    /// True if the profile lands from ambient branch state rather than an explicit reviewed landing candidate.
    /// Must be false.
    pub lands_from_ambient_branch_state: bool,
    /// True if the profile deletes orphaned worktrees or stale stack members without previewing running tasks,
    /// open editors, uncommitted changes, recovery checkpoints, and export-safe evidence. Must be false.
    pub deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery:
        bool,
}

impl ChangeOrchestrationCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        infers_stack_membership_from_branch_names_alone: false,
        mutates_another_worktree_without_a_selected_change_object_and_worktree_binding: false,
        silently_reorders_collapses_or_retargets_stack_members: false,
        lands_from_ambient_branch_state: false,
        deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery:
            false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.infers_stack_membership_from_branch_names_alone
            && !self.mutates_another_worktree_without_a_selected_change_object_and_worktree_binding
            && !self.silently_reorders_collapses_or_retargets_stack_members
            && !self.lands_from_ambient_branch_state
            && !self.deletes_orphaned_worktrees_or_stale_members_without_previewing_running_work_and_recovery
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationCertExportParity {
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

impl ChangeOrchestrationCertExportParity {
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
pub struct ChangeOrchestrationAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ChangeOrchestrationCertificationAxis,
    /// The certification state of the axis.
    pub state: ChangeOrchestrationAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ChangeOrchestrationDowngradeTrigger>,
}

impl ChangeOrchestrationAxisOutcome {
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
            ChangeOrchestrationAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ChangeOrchestrationAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ChangeOrchestrationAxisCertificationState::UndisclosedDrift => {
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
pub struct ChangeOrchestrationClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ChangeOrchestrationCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5ChangeOrchestrationCertClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5ChangeOrchestrationCertClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 change-orchestration object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationProfileCertificationRow {
    /// Record kind; must equal [`CHANGE_ORCHESTRATION_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5ChangeOrchestrationCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5ChangeOrchestrationCertClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5ChangeOrchestrationCertClaim,
    /// The frozen change-orchestration object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ChangeOrchestrationObject>,
    /// One outcome per [`ChangeOrchestrationCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ChangeOrchestrationAxisOutcome>,
    /// The B154 hard invariants; all must hold.
    pub guardrails: ChangeOrchestrationCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ChangeOrchestrationClaimAutoNarrow>,
    /// The one canonical change-orchestration lifecycle matrix proof bundle this profile cites. Must equal
    /// [`CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ChangeOrchestrationProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: ChangeOrchestrationCertExportParity,
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

impl ChangeOrchestrationProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: ChangeOrchestrationCertificationAxis,
    ) -> Option<&ChangeOrchestrationAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ChangeOrchestrationCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ChangeOrchestrationCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ChangeOrchestrationAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ChangeOrchestrationCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ChangeOrchestrationAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-certified change-orchestration lane
    /// profile may certify a certified change-orchestration record, every hard invariant must hold, CLI/export parity must
    /// always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ChangeOrchestrationProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        // Every B154 hard invariant must hold.
        if !self.guardrails.all_held() {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        // Only a fully-certified change-orchestration lane profile may certify a certified change-orchestration record.
        if self
            .certified_claim
            .asserts_certified_change_orchestration_truth()
            && !self.profile.is_certified_change_orchestration_lane()
        {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ChangeOrchestrationCertificationAxis::CliExport) {
            Some(o) if o.state == ChangeOrchestrationAxisCertificationState::Certified => {}
            _ => return ChangeOrchestrationProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ChangeOrchestrationAxisCertificationState::UndisclosedDrift)
        {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ChangeOrchestrationProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ChangeOrchestrationProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ChangeOrchestrationProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ChangeOrchestrationProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return ChangeOrchestrationProfileClaimStatus::Red;
        }

        ChangeOrchestrationProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CHANGE_ORCHESTRATION_CERT_ROW_RECORD_KIND
            && self.schema_version == CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1303 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationProfileCertificationSummary {
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

/// Constructor input for [`ChangeOrchestrationProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeOrchestrationProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ChangeOrchestrationProfileCertificationRow>,
}

/// Checked-in M05-1303 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ChangeOrchestrationProfileCertificationRow>,
    pub summary: ChangeOrchestrationProfileCertificationSummary,
}

impl ChangeOrchestrationProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ChangeOrchestrationProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION,
            record_kind: CHANGE_ORCHESTRATION_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ChangeOrchestrationProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5ChangeOrchestrationCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Review-pack object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ChangeOrchestrationObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5ChangeOrchestrationCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen change-orchestration object class is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ChangeOrchestrationObject::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ChangeOrchestrationCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ChangeOrchestrationAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ChangeOrchestrationProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeOrchestrationProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeOrchestrationProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ChangeOrchestrationProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ChangeOrchestrationProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        ChangeOrchestrationProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ChangeOrchestrationProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ChangeOrchestrationCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION {
            violations.push(ChangeOrchestrationCertificationViolation::SchemaVersion {
                expected: CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CHANGE_ORCHESTRATION_CERT_RECORD_KIND {
            violations.push(ChangeOrchestrationCertificationViolation::RecordKind {
                expected: CHANGE_ORCHESTRATION_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ChangeOrchestrationCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ChangeOrchestrationCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ChangeOrchestrationCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ChangeOrchestrationCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    ChangeOrchestrationCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    ChangeOrchestrationCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ChangeOrchestrationCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B154 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    ChangeOrchestrationCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a fully-certified change-orchestration lane profile may certify a certified change-orchestration record.
            if row
                .certified_claim
                .asserts_certified_change_orchestration_truth()
                && !row.profile.is_certified_change_orchestration_lane()
            {
                violations.push(
                    ChangeOrchestrationCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ChangeOrchestrationCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    ChangeOrchestrationCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ChangeOrchestrationCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    ChangeOrchestrationCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == ChangeOrchestrationProfileClaimStatus::Red {
                violations.push(ChangeOrchestrationCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(ChangeOrchestrationCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen change-orchestration object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(ChangeOrchestrationCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ChangeOrchestrationCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                ChangeOrchestrationCertificationViolation::RawChangeOrchestrationMaterialInExport,
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
        out.push_str("# M5 Change-Orchestration Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5ChangeOrchestrationCertifiedProfile::ALL.len(),
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
pub fn current_m5_change_orchestration_surface_certification_export() -> Result<
    ChangeOrchestrationProfileCertificationPacket,
    ChangeOrchestrationCertificationArtifactError,
> {
    let packet: ChangeOrchestrationProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-change-orchestration-surface-certification/support_export.json"
        )))
        .map_err(ChangeOrchestrationCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ChangeOrchestrationCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ChangeOrchestrationCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ChangeOrchestrationCertificationViolation>),
}

impl fmt::Display for ChangeOrchestrationCertificationArtifactError {
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

impl Error for ChangeOrchestrationCertificationArtifactError {}

/// Validation failure for M05-1303 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeOrchestrationCertificationViolation {
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
    RawChangeOrchestrationMaterialInExport,
}

impl fmt::Display for ChangeOrchestrationCertificationViolation {
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
                    "packet does not cite the canonical change-orchestration lifecycle matrix proof bundle"
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
                    "row {id} does not cite the one canonical change-orchestration lifecycle matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B154 hard invariant: inferring stack membership from branch names alone; \
mutating another worktree without an explicit selected change object and worktree binding; silently reordering, \
collapsing, or retargeting stack members; landing from ambient branch state; or deleting an orphaned worktree \
or stale member without previewing running tasks, open editors, uncommitted changes, recovery checkpoints, and \
export-safe evidence"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified change-orchestration record on a non-lane profile"
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
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified change-orchestration record, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 change-orchestration profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen change-orchestration object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawChangeOrchestrationMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for ChangeOrchestrationCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ChangeOrchestrationAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ChangeOrchestrationAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the change-orchestration
/// generics the spec forbids collapsing distinct worktree-binding, stack-membership, stack-order,
/// landing-authority, validation-freshness, and cleanup-evidence truth into (whole-label matches so a full
/// sentence naming a concrete worktree binding, stack membership, or landing state is not flagged).
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
            | "change orchestration"
            | "change-orchestration"
            | "change object"
            | "record"
            | "work item"
            | "selected change"
            | "worktree"
            | "worktree binding"
            | "base identity"
            | "patch stack"
            | "stack"
            | "stack member"
            | "stack membership"
            | "membership"
            | "stack order"
            | "reorder"
            | "restack"
            | "landing"
            | "landing candidate"
            | "landing state"
            | "target branch"
            | "ambient branch"
            | "merge queue"
            | "queue"
            | "protected branch"
            | "shelf"
            | "portable shelf"
            | "bundle"
            | "import"
            | "reopen"
            | "validation"
            | "validation freshness"
            | "approval"
            | "cleanup"
            | "orphan"
            | "orphaned"
            | "recovery"
            | "checkpoint"
            | "provider"
            | "local"
            | "local only"
            | "estimate"
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the change-orchestration lifecycle
/// matrix heuristic so the reused [`M5ChangeOrchestrationDowngradeTrigger`] narrowings serialize cleanly — the
/// change-orchestration proof grammar carries only typed class tokens and opaque refs, never raw secret values or
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

/// Builds the canonical, checked-in M05-1303 certification packet. Certifies all eight claimed M5 change-orchestration
/// profiles: two deliver their claim (green) and six auto-narrow a not-current truth axis to a weaker
/// configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_change_orchestration_surface_certification_packet(
) -> ChangeOrchestrationProfileCertificationPacket {
    ChangeOrchestrationProfileCertificationPacket::new(
        ChangeOrchestrationProfileCertificationPacketInput {
            packet_id: CHANGE_ORCHESTRATION_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-16T00:00:00Z".to_owned(),
            matrix_ref: CHANGE_ORCHESTRATION_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:change-orchestration-surface-certification:{id}"),
        CHANGE_ORCHESTRATION_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ChangeOrchestrationCertExportParity {
    ChangeOrchestrationCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ChangeOrchestrationCertificationAxis) -> &'static str {
    match axis {
        ChangeOrchestrationCertificationAxis::Visual => {
            "the selected change object, its worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup evidence are shown on-surface without a shell-chrome-only affordance or an ambient-branch row alone, and no ambient branch state still reads as a reviewed landing candidate"
        }
        ChangeOrchestrationCertificationAxis::Keyboard => {
            "the same worktree binding, stack membership, landing authority, and bound stack-edit / land / shelf / cleanup operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        ChangeOrchestrationCertificationAxis::ScreenReader => {
            "the same change-orchestration truth is announced non-visually, never a shell-chrome-only / ambient-branch-row / unlabeled-control-only cue"
        }
        ChangeOrchestrationCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the selected change object, worktree / base identity, stack membership and order, landing state, or validation freshness"
        }
        ChangeOrchestrationCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the worktree-binding badge, stack-membership-source class, or landing / cleanup state"
        }
        ChangeOrchestrationCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a worktree binding, stack-membership source, landing state, or validation freshness"
        }
        ChangeOrchestrationCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        ChangeOrchestrationCertificationAxis::DegradedState => {
            "an unverifiable worktree binding, an inferred stack membership, a drifted or silently-reordered stack, an unprovable queue or protected-branch landing authority, stale validation or approval evidence, or partial cleanup evidence honestly downgrades the CertifiedChangeOrchestrationTruth/ReviewableChangeOrchestrationRecord claim rather than reading as a fresh, provider-authoritative change-orchestration record"
        }
        ChangeOrchestrationCertificationAxis::ChangeOrchestrationTruth => {
            "the selected change object, worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup evidence stay explicit and never let ambient branch state read as a reviewed landing candidate, infer stack membership from branch names alone, mutate another worktree without a selected change object and worktree binding, silently reorder / collapse / retarget stack members, land from ambient branch state, or delete an orphaned worktree or stale member without previewing running work and recovery"
        }
    }
}

fn seed_certified(axis: ChangeOrchestrationCertificationAxis) -> ChangeOrchestrationAxisOutcome {
    ChangeOrchestrationAxisOutcome {
        axis,
        state: ChangeOrchestrationAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ChangeOrchestrationCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ChangeOrchestrationDowngradeTrigger,
) -> ChangeOrchestrationAxisOutcome {
    ChangeOrchestrationAxisOutcome {
        axis,
        state: ChangeOrchestrationAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ChangeOrchestrationAxisOutcome> {
    ChangeOrchestrationCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ChangeOrchestrationCertificationAxis,
    outcome: ChangeOrchestrationAxisOutcome,
) -> Vec<ChangeOrchestrationAxisOutcome> {
    ChangeOrchestrationCertificationAxis::ALL
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
    profile: M5ChangeOrchestrationCertifiedProfile,
    claimed_claim: M5ChangeOrchestrationCertClaim,
    certified_claim: M5ChangeOrchestrationCertClaim,
    consumed_families: &[M5ChangeOrchestrationObject],
    axis_outcomes: Vec<ChangeOrchestrationAxisOutcome>,
    claim_auto_narrow: Option<ChangeOrchestrationClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ChangeOrchestrationProfileCertificationRow {
    let mut row = ChangeOrchestrationProfileCertificationRow {
        record_kind: CHANGE_ORCHESTRATION_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: CHANGE_ORCHESTRATION_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: ChangeOrchestrationCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: CHANGE_ORCHESTRATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ChangeOrchestrationProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            CHANGE_ORCHESTRATION_CERT_MATRIX_REF.to_owned(),
            CHANGE_ORCHESTRATION_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ChangeOrchestrationCertificationAxis,
    from_claim: M5ChangeOrchestrationCertClaim,
    to_claim: M5ChangeOrchestrationCertClaim,
    label: &str,
) -> ChangeOrchestrationClaimAutoNarrow {
    ChangeOrchestrationClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ChangeOrchestrationProfileCertificationRow> {
    use ChangeOrchestrationCertificationAxis as Ax;
    use M5ChangeOrchestrationCertClaim::*;
    use M5ChangeOrchestrationCertifiedProfile as P;
    use M5ChangeOrchestrationDowngradeTrigger as Trig;
    use M5ChangeOrchestrationObject::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-change-orchestration-lane",
            P::CertifiedChangeOrchestrationLane,
            CertifiedChangeOrchestrationTruth,
            CertifiedChangeOrchestrationTruth,
            &[ChangeObject],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "selected_change_binding",
            ],
            &[
                "certified change-orchestration lane: the selected change object, its worktree / base identity, stack membership and order, landing state, validation freshness, and cleanup evidence all join to one export-safe, provider-authoritative change-orchestration record, never ambient branch state that reads as a reviewed landing candidate",
                "the certified change object keeps stable operation IDs while its worktree binding, declared stack membership, landing authority, and validation freshness bind to the one change-orchestration matrix across change-object-detail / patch-stack-queue / stack-edit-review-sheet / landing-candidate-sheet / portable-shelf / worktree-cleanup-preview / support-export / help-docs surfaces, and no change reads as queue-eligible in one surface and stale-base in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered change object",
                "change-orchestration-truth: a fully-certified change-orchestration lane with an explicit selected change object and worktree binding is the only profile that certifies a certified change-orchestration record",
            ],
        ),
        seed_row(
            "cert:reviewable-change-orchestration-record-structure",
            P::ReviewableChangeOrchestrationRecordStructure,
            ReviewableChangeOrchestrationRecord,
            ReviewableChangeOrchestrationRecord,
            &[LandingCandidateSheet],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "landing_state",
            ],
            &[
                "record-structure class: an export-safe landing-candidate / review sheet bound to one selected change object and inspectable rather than a per-surface description copied by hand, with the target branch, merge strategy, required checks, and rollback / export fallback kept bound to the change object it came from",
                "the reviewable landing candidate keeps its target branch, validation freshness, and queue eligibility inspectable rather than an ambient-branch or shell-chrome-only cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable change-orchestration record structure",
                "change-orchestration-truth: a reviewable landing candidate never certifies a fully-certified-lane claim and never stays green on ambient branch state or a missing worktree binding",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:unbound-worktree-binding-profile",
            P::UnboundWorktreeBindingProfile,
            ReviewableChangeOrchestrationRecord,
            WorktreeBindingUnverifiedProjection,
            &[ChangeObject],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the change object's worktree / base identity cannot be confirmed for this profile so a provider-authoritative change-orchestration record cannot be certified and the change stays inspect-only",
                    "The change object's selected worktree and base-or-dirty-tree fingerprint can no longer be confirmed bound, so the ReviewableChangeOrchestrationRecord claim narrows to a worktree-binding-unverified projection and the lane discloses the last-known worktree binding rather than mutating another worktree or reading ambient branch state as a selected change object",
                    Trig::WorktreeBindingUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableChangeOrchestrationRecord,
                WorktreeBindingUnverifiedProjection,
                "The worktree binding is unverified for this change object, so its last-known selected worktree and base identity are disclosed and no cross-worktree write happens from ambient branch state",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "unbound-worktree class: the change object names its selected worktree, base-or-dirty-tree fingerprint, and intent class and marks the binding unverified rather than letting ambient branch state stand in for a selected change object when the worktree binding is unconfirmed",
                "the unbound-worktree surface keeps its selected change object and last-known worktree binding legible while the binding is disclosed as unverified",
                "degraded-state: ReviewableChangeOrchestrationRecord narrows to a worktree-binding-unverified projection (auto-narrowed)",
                "change-orchestration-truth: a change object never mutates another worktree without an explicit selected change object and worktree binding — its binding is preserved and ambient branch state never reads as a selected change object",
            ],
        ),
        seed_row(
            "cert:inferred-stack-membership-profile",
            P::InferredStackMembershipProfile,
            ReviewableChangeOrchestrationRecord,
            StackMembershipUnverifiedProjection,
            &[PatchStackQueue],
            seed_certified_except(
                Ax::ChangeOrchestrationTruth,
                seed_narrowed(
                    Ax::ChangeOrchestrationTruth,
                    "a patch-stack member's stack membership cannot be confirmed declared for this profile so a provider-authoritative change-orchestration record cannot be certified and the membership stays inspect-only",
                    "A patch-stack member's stack membership cannot be confirmed as explicitly declared — it risks being inferred from a branch name alone — so the ReviewableChangeOrchestrationRecord claim narrows to a stack-membership-unverified projection and the lane keeps each member's declared membership and dependency edges explicit rather than inferring stack membership from branch names alone",
                    Trig::StackMembershipInferredFromBranchNameAlone,
                ),
            ),
            Some(seed_narrow(
                Ax::ChangeOrchestrationTruth,
                ReviewableChangeOrchestrationRecord,
                StackMembershipUnverifiedProjection,
                "The stack membership is not confirmed declared, so each member's declared-in-change-object membership and dependency edges stay explicit and none is inferred from a branch name alone",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "patch-stack class: the patch-stack queue keeps each member's declared membership, ordered position, queue eligibility, and dependency edges explicit and marks the membership unverified rather than inferring a member the operator did not declare",
                "the patch-stack surface keeps its per-member declared membership legible while the membership is disclosed as unverified",
                "change-orchestration-truth: ReviewableChangeOrchestrationRecord narrows to a stack-membership-unverified projection (auto-narrowed)",
                "change-orchestration-truth: stack membership is never inferred from branch names alone — each member's declared membership and dependency edges stay explicit",
            ],
        ),
        seed_row(
            "cert:silently-reordered-stack-profile",
            P::SilentlyReorderedStackProfile,
            ReviewableChangeOrchestrationRecord,
            StackOrderUnverifiedProjection,
            &[StackEditReviewSheet],
            seed_certified_except(
                Ax::Visual,
                seed_narrowed(
                    Ax::Visual,
                    "a stack-edit's proposed order cannot be confirmed reviewed for this profile so a provider-authoritative change-orchestration record cannot be certified and the re-stack stays inspect-only",
                    "A stack-edit review sheet's proposed order cannot be confirmed reviewed — a reorder, split, or squash risks landing silently — so the ReviewableChangeOrchestrationRecord claim narrows to a stack-order-unverified projection and the lane keeps the original order, proposed order, and affected parent-child links explicit rather than silently reordering, collapsing, or retargeting stack members",
                    Trig::StackMembersSilentlyReordered,
                ),
            ),
            Some(seed_narrow(
                Ax::Visual,
                ReviewableChangeOrchestrationRecord,
                StackOrderUnverifiedProjection,
                "The stack order is unverified, so the original order, proposed order, and affected parent-child links stay explicit and no member is silently reordered, collapsed, or retargeted",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "stack-edit class: the stack-edit review sheet keeps its original order, proposed order, and affected parent-child links explicit and marks the re-stack unverified rather than applying a reorder / split / squash the operator did not review",
                "the stack-edit surface keeps its order-change and dependency impact legible while the re-stack is disclosed as unverified",
                "visual: ReviewableChangeOrchestrationRecord narrows to a stack-order-unverified projection (auto-narrowed)",
                "change-orchestration-truth: stack members are never silently reordered, collapsed, or retargeted — the original and proposed order and affected links stay visible and reviewable",
            ],
        ),
        seed_row(
            "cert:ambient-branch-landing-profile",
            P::AmbientBranchLandingProfile,
            ReviewableChangeOrchestrationRecord,
            LandingAuthorityUnverifiedProjection,
            &[LandingCandidateSheet],
            seed_certified_except(
                Ax::HighZoomReflow,
                seed_narrowed(
                    Ax::HighZoomReflow,
                    "a landing candidate's queue authority or protected-branch gate cannot be proven for this profile so a provider-authoritative change-orchestration record cannot be certified and the landing stays a reviewed-but-blocked candidate",
                    "A landing candidate's queue authority or protected-branch gate cannot be proven — the merge queue is unavailable, the queue position is ambiguous, the base is stale, or the protected-branch rule is unverifiable — so the ReviewableChangeOrchestrationRecord claim narrows to a landing-authority-unverified projection and the lane keeps the reviewed landing candidate labelled with its target branch and queue posture rather than landing from ambient branch state",
                    Trig::LandedFromAmbientBranchState,
                ),
            ),
            Some(seed_narrow(
                Ax::HighZoomReflow,
                ReviewableChangeOrchestrationRecord,
                LandingAuthorityUnverifiedProjection,
                "The landing authority is unproven, so the landing candidate stays labelled with its target branch and queue / protected-branch posture and never lands from ambient branch state",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "landing-candidate class: the landing-candidate sheet keeps its target branch, merge strategy, required checks, and provider-authoritative-versus-local-estimate posture explicit and marks the candidate blocked rather than landing when queue authority or the protected-branch gate is unproven",
                "the landing-candidate surface keeps its target-branch and queue posture legible while the landing is disclosed as a reviewed-but-blocked candidate",
                "high-zoom-reflow: ReviewableChangeOrchestrationRecord narrows to a landing-authority-unverified projection (auto-narrowed)",
                "change-orchestration-truth: Aureline never lands from ambient branch state — the reviewed landing candidate, its target branch, and its queue / protected-branch posture stay explicit and a local estimate never reads as a provider-authoritative landing",
            ],
        ),
        seed_row(
            "cert:stale-validation-shelf-profile",
            P::StaleValidationShelfProfile,
            ReviewableChangeOrchestrationRecord,
            ValidationFreshnessUnverifiedProjection,
            &[PortableShelf],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "a portable shelf's validation or approval evidence is stale for this profile so a provider-authoritative change-orchestration record cannot be certified and the shelf stays inspect-only",
                    "A portable shelf's packaged validation or approval evidence is stale — its checks ran against a superseded base or its review-pack version has drifted — so the ReviewableChangeOrchestrationRecord claim narrows to a validation-freshness-unverified projection and the lane keeps the last-known validation freshness and review-pack version explicit rather than reopening the imported shelf as current provider-authoritative truth",
                    Trig::ValidationFreshnessUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableChangeOrchestrationRecord,
                ValidationFreshnessUnverifiedProjection,
                "The shelf's validation freshness is unverified, so its last-known validation and review-pack version stay explicit and the imported shelf never overclaims current hosted truth",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "portable-shelf class: the portable shelf keeps its bundle ID, diff refs, evidence refs, review-pack version, and last-known validation freshness explicit and marks the evidence stale rather than reopening an imported shelf as current provider-authoritative truth",
                "the portable-shelf surface keeps its bundle contents and last-known validation freshness legible while the evidence is disclosed as stale",
                "localization: ReviewableChangeOrchestrationRecord narrows to a validation-freshness-unverified projection (auto-narrowed)",
                "change-orchestration-truth: stale validation is never presented as fresh — the shelf's last-known validation freshness and review-pack version stay explicit and an imported shelf reopens as honest local-only / stale state, never current hosted truth",
            ],
        ),
        seed_row(
            "cert:partial-cleanup-evidence-profile",
            P::PartialCleanupEvidenceProfile,
            ReviewableChangeOrchestrationRecord,
            CleanupEvidenceUnverifiedProjection,
            &[WorktreeCleanupPreview],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "a worktree cleanup preview's affected-work and recovery evidence is partial for this profile so a provider-authoritative change-orchestration record cannot be certified and the cleanup stays blocked",
                    "A worktree cleanup preview's affected-work and recovery evidence is partial — the running tasks, open editors, uncommitted changes, or reflog / checkpoint recovery for an orphaned or stale worktree cannot be fully enumerated — so the ReviewableChangeOrchestrationRecord claim narrows to a cleanup-evidence-unverified projection and the lane keeps the affected running work, uncommitted-change scope, and recovery lineage explicit rather than deleting an orphaned worktree or stale member without previewing running work and recovery",
                    Trig::OrphanDeletedWithoutSafetyPreview,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableChangeOrchestrationRecord,
                CleanupEvidenceUnverifiedProjection,
                "The cleanup evidence is partial, so the affected running work, uncommitted-change scope, and reflog / checkpoint recovery stay explicit and no orphaned worktree or stale member is deleted without preview and confirmation",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "worktree-cleanup class: the worktree cleanup preview keeps its cleanup target, affected running tasks, open editors, uncommitted changes, and reflog / checkpoint recovery explicit and marks the evidence partial rather than removing a worktree whose running work and recovery are not fully previewed",
                "the worktree-cleanup surface keeps its affected-work and recovery lineage legible non-visually while the evidence is disclosed as partial",
                "screen-reader: ReviewableChangeOrchestrationRecord narrows to a cleanup-evidence-unverified projection (auto-narrowed)",
                "change-orchestration-truth: an orphaned worktree or stale member is never deleted without previewing running work and recovery — the affected running tasks, uncommitted-change scope, and reflog / checkpoint recovery stay explicit and survive export and reopen",
            ],
        ),
    ]
}
