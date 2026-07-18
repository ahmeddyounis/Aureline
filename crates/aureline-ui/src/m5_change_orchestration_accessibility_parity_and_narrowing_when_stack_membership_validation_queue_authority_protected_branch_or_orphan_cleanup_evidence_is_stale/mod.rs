//! Keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity, and honest automatic claim
//! narrowing for the M5 change object / patch-stack queue / stack-edit review sheet / landing-candidate sheet
//! / portable shelf / worktree-cleanup-preview objects.
//!
//! This module is the M05-1302 accessibility-and-auto-narrowing capstone over the frozen M5 change-object,
//! patch-stack, and landing matrix ([`crate::m5_change_object_patch_stack_and_landing_matrix`]). Where the
//! freeze matrix defines the reusable change object, patch-stack queue, stack-edit review sheet,
//! landing-candidate sheet, portable shelf, and worktree-cleanup-preview objects, and the 1295-1301
//! implementation lanes resolve their per-surface truth, this lane certifies — per object class — that stack,
//! landing, shelf, and cleanup claims stay **keyboard-complete, assistive-tech-reachable, high-zoom /
//! high-contrast-safe, CLI/export-safe, and self-narrowing** rather than presenting an unbound worktree
//! binding, a stack membership inferred from branch names, a drifted / restack-required stack order, an
//! unprovable queue authority or protected-branch rule, stale validation / approval evidence, or partial
//! orphan-cleanup evidence as still a fully provider-authoritative, landed surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / CLI reach.** Every object exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, high-contrast-safe, and
//!   CLI/headless-reachable path into the same object identity, selected worktree / base binding, stack
//!   membership source, stack order, landing state, validation freshness, and cleanup safety the rich object
//!   shows — never a color-only stack-dependency chip, a hover-only landing-authority pill, or a pointer-only
//!   cleanup affordance that strands assistive-tech or headless-CLI users. Structure-heavy objects (the
//!   stack-edit review sheet's ordered / parent-child rows, the worktree-cleanup preview's affected running
//!   work / recovery set) additionally bind their structured layout to a flat list / textual path.
//! - **Export parity.** The support / CLI / release export reconstructs each object's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same worktree binding, stack membership,
//!   stack order, landing state, validation freshness, and cleanup-safety labels visible in-product so support,
//!   help, and release proof can reconstruct exactly what the user was shown without leaking a raw diff hunk,
//!   message payload, secret, endpoint, or provider token.
//! - **Honest auto-narrowing.** When a change object's selected worktree binding is unbound, a patch-stack
//!   queue's membership is inferred / unverifiable, a stack-edit review sheet's order is drifted or
//!   restack-required, a landing candidate's queue authority or protected-branch rule cannot be proven,
//!   a portable shelf's validation or approval evidence is stale, or a worktree-cleanup preview's evidence is
//!   partial, the object's claim auto-narrows from `trusted_provider_landed_surface` /
//!   `local_reviewable_surface` to a selected-change-binding-unverified / stack-membership-unverified /
//!   stack-order-unverified / landing-authority-unverified / validation-freshness-unverified /
//!   cleanup-evidence-unverified projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical object identity / last-known state. The underlying worktree,
//!   stack, landing, shelf, and cleanup truth is never dropped opaquely. An object with every dimension intact
//!   must NOT carry a spurious narrowing, and an unbound-binding / inferred-membership / drifted-order /
//!   unprovable-authority / stale-validation / partial-cleanup state can never keep a fully
//!   provider-authoritative, landed claim — a local landing estimate never masquerades as a
//!   provider-authoritative land, ambient branch state is never shown as a reviewed landing candidate, and
//!   stack membership is never inferred from branch names alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the change-object detail, patch-stack
//!   queue, stack-edit review sheet, review detail, provider merge queue, portable shelf, worktree-cleanup
//!   preview, support / export packet, and help / docs so product, help, and release publication stay aligned
//!   on downgrade behavior rather than drifting in copy — a trusted-looking object can never outrun the
//!   worktree binding, stack membership, stack order, landing state, validation freshness, or cleanup evidence
//!   it is being viewed away from.
//!
//! Each [`ChangeOrchestrationAccessibilityRow`] keys on one
//! [`crate::m5_change_object_patch_stack_and_landing_matrix::M5ChangeOrchestrationObject`] and reuses that frozen
//! object vocabulary plus the frozen [`M5ChangeOrchestrationRequiredLabel`], [`M5ChangeOrchestrationDowngradeTrigger`], and
//! shared [`M5ChangeOrchestrationConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling change-orchestration packets.
//!
//! The packet is metadata-only: raw diff hunks, message payloads, credentials, secrets, and endpoint refs
//! never cross this boundary; the packet carries only typed class tokens, opaque object refs, booleans, and
//! controlled labels so support, release, and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen change-orchestration vocabulary — the capstone certifies the freeze matrix's objects, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_change_object_patch_stack_and_landing_matrix::{
    M5ChangeOrchestrationConsumerSurface, M5ChangeOrchestrationDowngradeTrigger,
    M5ChangeOrchestrationObject, M5ChangeOrchestrationRequiredLabel,
    M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1282 change-orchestration accessibility parity packet.
pub const CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ChangeOrchestrationAccessibilityPacket`].
pub const CHANGE_ORCHESTRATION_A11Y_RECORD_KIND: &str =
    "m5_change_orchestration_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`ChangeOrchestrationAccessibilityRow`].
pub const CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND: &str =
    "m5_change_orchestration_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const CHANGE_ORCHESTRATION_A11Y_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-orchestration-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const CHANGE_ORCHESTRATION_A11Y_DOC_REF: &str =
    "docs/team-workflows/m5_change_orchestration_accessibility_parity.md";

/// Repo-relative path of the frozen change-orchestration and engineering-lifecycle matrix this lane certifies.
pub const CHANGE_ORCHESTRATION_A11Y_MATRIX_REF: &str = M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const CHANGE_ORCHESTRATION_A11Y_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-orchestration-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CHANGE_ORCHESTRATION_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-orchestration-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CHANGE_ORCHESTRATION_A11Y_CSV_REF: &str =
    "artifacts/release/m5-change-orchestration-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CHANGE_ORCHESTRATION_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-change-orchestration-accessibility-parity.md";

/// The reusable objects that render a dense, structured surface (the linked branch / worktree / review relation set, the
/// blocked-escalate card's blocker class / dependency / escalation set) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable non-visually.
const fn object_is_structure_heavy(object: M5ChangeOrchestrationObject) -> bool {
    matches!(
        object,
        M5ChangeOrchestrationObject::StackEditReviewSheet
            | M5ChangeOrchestrationObject::WorktreeCleanupPreview
    )
}

/// The change-orchestration-truth dimension whose weakening an object primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn object_primary_dimension(
    object: M5ChangeOrchestrationObject,
) -> M5ChangeOrchestrationClaimDimension {
    match object {
        M5ChangeOrchestrationObject::ChangeObject => {
            M5ChangeOrchestrationClaimDimension::SelectedChangeBindingClarity
        }
        M5ChangeOrchestrationObject::PatchStackQueue => {
            M5ChangeOrchestrationClaimDimension::StackMembershipClarity
        }
        M5ChangeOrchestrationObject::StackEditReviewSheet => {
            M5ChangeOrchestrationClaimDimension::StackOrderIntegrityClarity
        }
        M5ChangeOrchestrationObject::LandingCandidateSheet => {
            M5ChangeOrchestrationClaimDimension::LandingAuthorityClarity
        }
        M5ChangeOrchestrationObject::PortableShelf => {
            M5ChangeOrchestrationClaimDimension::ValidationFreshnessClarity
        }
        M5ChangeOrchestrationObject::WorktreeCleanupPreview => {
            M5ChangeOrchestrationClaimDimension::CleanupEvidenceClarity
        }
    }
}

/// A rendered fallback modality for an change-orchestration object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationFallbackModality {
    /// A rich, structured (outbound action set / lifecycle history) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ChangeOrchestrationFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same object may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ChangeOrchestrationRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / CLI reach for an object's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl ChangeOrchestrationNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the object meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationExportSummaryState {
    /// The object meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl ChangeOrchestrationExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ChangeOrchestrationNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The change-orchestration claim ceiling an object asserts: how strong a provider-authoritative, landed
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a selected-change-binding /
/// stack-membership / stack-order / landing-authority / validation-freshness / cleanup-evidence dimension
/// weakens so an unbound worktree binding, a stack membership inferred from branch names, a drifted /
/// restack-required stack order, an unprovable queue authority or protected-branch rule, stale validation /
/// approval evidence, or partial orphan-cleanup evidence can never keep an old `TrustedProviderLandedSurface`
/// or `LocalReviewableSurface` label — a local landing estimate never masquerades as a provider-authoritative
/// land from a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationA11yClaim {
    /// Trusted landed surface: a fully bound, stack-verified, order-fresh, landing-authoritative,
    /// validation-fresh, cleanup-safe object — the strongest claim, a change-orchestration surface Aureline can
    /// present as exactly provider-authoritative and landed to inspect, stack, land, export, or reopen right now.
    TrustedProviderLandedSurface,
    /// Local reviewable surface: a self-sufficient, reviewable read-only object (a worktree-cleanup preview a
    /// user can inspect) that is not itself an authoritative, landing-driving surface.
    LocalReviewableSurface,
    /// Selected-change-binding-unverified projection: the change object's selected worktree / base binding is
    /// unbound; the object stays a selected-change-binding-unverified projection with its last-known change
    /// identity preserved, never a cross-worktree write shown as a selected change.
    SelectedChangeBindingUnverifiedProjection,
    /// Stack-membership-unverified projection: the patch-stack membership is inferred or unverifiable; the
    /// object stays a stack-membership-unverified projection that keeps disclosed and inferred membership
    /// distinct, never inferring stack membership from branch names alone.
    StackMembershipUnverifiedProjection,
    /// Stack-order-unverified projection: the stack order is drifted or a restack is required here; the object
    /// stays a stack-order-unverified projection that keeps the original / proposed order and parent-child links
    /// explicit, never silently reordering, collapsing, or retargeting stack members.
    StackOrderUnverifiedProjection,
    /// Landing-authority-unverified projection: the queue authority or protected-branch rule cannot be proven;
    /// the object stays a landing-authority-unverified projection that names the local-estimate-versus-provider
    /// difference, never widening a local landing estimate into a provider-authoritative land or landing from
    /// ambient branch state.
    LandingAuthorityUnverifiedProjection,
    /// Validation-freshness-unverified projection: the validation or approval evidence is stale; the object
    /// stays a validation-freshness-unverified projection that discloses the stale validation / approval
    /// binding, never presenting stale validation or approval evidence as currently green.
    ValidationFreshnessUnverifiedProjection,
    /// Cleanup-evidence-unverified projection: the orphan-cleanup evidence is partial; the object stays a
    /// cleanup-evidence-unverified projection that keeps affected running work, uncommitted-change scope, and
    /// recovery / checkpoint lineage visible, never deleting a worktree or stale member without a safety preview.
    CleanupEvidenceUnverifiedProjection,
}

impl M5ChangeOrchestrationA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedProviderLandedSurface,
        Self::LocalReviewableSurface,
        Self::SelectedChangeBindingUnverifiedProjection,
        Self::StackMembershipUnverifiedProjection,
        Self::StackOrderUnverifiedProjection,
        Self::LandingAuthorityUnverifiedProjection,
        Self::ValidationFreshnessUnverifiedProjection,
        Self::CleanupEvidenceUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedProviderLandedSurface => 7,
            Self::LocalReviewableSurface => 6,
            Self::SelectedChangeBindingUnverifiedProjection => 5,
            Self::StackMembershipUnverifiedProjection => 4,
            Self::StackOrderUnverifiedProjection => 3,
            Self::LandingAuthorityUnverifiedProjection => 2,
            Self::ValidationFreshnessUnverifiedProjection => 1,
            Self::CleanupEvidenceUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully provider-committed, publish-safe review surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedProviderLandedSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedProviderLandedSurface | Self::LocalReviewableSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedProviderLandedSurface => "trusted_provider_landed_surface",
            Self::LocalReviewableSurface => "local_reviewable_surface",
            Self::SelectedChangeBindingUnverifiedProjection => {
                "selected_change_binding_unverified_projection"
            }
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

/// The selected-change-binding / stack-membership / stack-order / landing-authority / validation-freshness /
/// cleanup-evidence dimension whose state governs how far an object may claim to be a fully
/// provider-authoritative, landed surface. The dimensions map to the six frozen change-orchestration objects
/// so every object carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationClaimDimension {
    /// Selected-change-binding clarity: does the change object keep its selected worktree / base binding
    /// explicit so no command, tool, or provider action mutates another worktree from ambient branch state
    /// (change-object)?
    SelectedChangeBindingClarity,
    /// Stack-membership clarity: does the patch-stack queue keep its membership disclosed as explicit metadata
    /// rather than inferred from branch names alone (patch-stack-queue)?
    StackMembershipClarity,
    /// Stack-order-integrity clarity: does the stack-edit review sheet keep its original / proposed order and
    /// parent-child links explicit rather than silently reordering, collapsing, or retargeting members
    /// (stack-edit-review-sheet)?
    StackOrderIntegrityClarity,
    /// Landing-authority clarity: does the landing candidate keep a local landing estimate distinct from a
    /// provider-authoritative land and name the queue-authority / protected-branch blocker rather than landing
    /// from ambient branch state (landing-candidate-sheet)?
    LandingAuthorityClarity,
    /// Validation-freshness clarity: does the portable shelf keep its validation / approval freshness explicit
    /// rather than presenting stale validation or approval evidence as currently green (portable-shelf)?
    ValidationFreshnessClarity,
    /// Cleanup-evidence clarity: does the worktree-cleanup preview keep its affected running work, uncommitted
    /// changes, and recovery / checkpoint evidence explicit rather than deleting a worktree or stale member
    /// without a safety preview (worktree-cleanup-preview)?
    CleanupEvidenceClarity,
}

impl M5ChangeOrchestrationClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectedChangeBindingClarity,
        Self::StackMembershipClarity,
        Self::StackOrderIntegrityClarity,
        Self::LandingAuthorityClarity,
        Self::ValidationFreshnessClarity,
        Self::CleanupEvidenceClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedChangeBindingClarity => "selected_change_binding_clarity",
            Self::StackMembershipClarity => "stack_membership_clarity",
            Self::StackOrderIntegrityClarity => "stack_order_integrity_clarity",
            Self::LandingAuthorityClarity => "landing_authority_clarity",
            Self::ValidationFreshnessClarity => "validation_freshness_clarity",
            Self::CleanupEvidenceClarity => "cleanup_evidence_clarity",
        }
    }
}

/// The observed condition of one change-orchestration-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the object's claim. The unbound / inferred / drifted / unprovable / stale /
/// partial states the lane must auto-narrow on — an unbound worktree binding, a stack membership inferred from
/// branch names, a drifted / restack-required stack order, an unprovable queue authority or protected-branch
/// rule, stale validation / approval evidence, and partial orphan-cleanup evidence — are the states that
/// [`Self::cannot_be_shown_trusted`] flags: each is a genuine truth degradation that can never be shown as a
/// fully provider-authoritative, landed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeOrchestrationConditionState {
    /// Fully bound, stack-verified, order-fresh, landing-authoritative, validation-fresh, cleanup-safe —
    /// imposes no ceiling.
    FullyQualified,
    /// The selected worktree / base binding is unbound — claim drops to a selected-change-binding-unverified
    /// projection.
    SelectedChangeBindingUnbound,
    /// The patch-stack membership is inferred or unverifiable — claim drops to a stack-membership-unverified
    /// projection.
    StackMembershipInferredOrUnverifiable,
    /// The stack order is drifted or a restack is required here — claim drops to a stack-order-unverified
    /// projection.
    StackOrderDriftedOrRestackRequired,
    /// The queue authority or protected-branch rule cannot be proven — claim drops to a
    /// landing-authority-unverified projection.
    QueueOrProtectedBranchUnprovable,
    /// The validation or approval evidence is stale — claim drops to a validation-freshness-unverified
    /// projection.
    ValidationOrApprovalStale,
    /// The orphan-cleanup evidence is partial — claim drops to a cleanup-evidence-unverified projection.
    CleanupEvidencePartial,
}

impl M5ChangeOrchestrationConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::SelectedChangeBindingUnbound,
        Self::StackMembershipInferredOrUnverifiable,
        Self::StackOrderDriftedOrRestackRequired,
        Self::QueueOrProtectedBranchUnprovable,
        Self::ValidationOrApprovalStale,
        Self::CleanupEvidencePartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects a weakened state that cannot be shown as a fully
    /// provider-committed, publish-safe review surface and must never be shown as such. Every weak change-orchestration
    /// condition is a genuine truth degradation, so all six flag here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::SelectedChangeBindingUnbound
                | Self::StackMembershipInferredOrUnverifiable
                | Self::StackOrderDriftedOrRestackRequired
                | Self::QueueOrProtectedBranchUnprovable
                | Self::ValidationOrApprovalStale
                | Self::CleanupEvidencePartial
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ChangeOrchestrationA11yClaim {
        match self {
            Self::FullyQualified => M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            Self::SelectedChangeBindingUnbound => {
                M5ChangeOrchestrationA11yClaim::SelectedChangeBindingUnverifiedProjection
            }
            Self::StackMembershipInferredOrUnverifiable => {
                M5ChangeOrchestrationA11yClaim::StackMembershipUnverifiedProjection
            }
            Self::StackOrderDriftedOrRestackRequired => {
                M5ChangeOrchestrationA11yClaim::StackOrderUnverifiedProjection
            }
            Self::QueueOrProtectedBranchUnprovable => {
                M5ChangeOrchestrationA11yClaim::LandingAuthorityUnverifiedProjection
            }
            Self::ValidationOrApprovalStale => {
                M5ChangeOrchestrationA11yClaim::ValidationFreshnessUnverifiedProjection
            }
            Self::CleanupEvidencePartial => {
                M5ChangeOrchestrationA11yClaim::CleanupEvidenceUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ChangeOrchestrationDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => {
                M5ChangeOrchestrationDowngradeTrigger::ChangeOrchestrationMatrixStale
            }
            Self::SelectedChangeBindingUnbound => {
                M5ChangeOrchestrationDowngradeTrigger::WorktreeBindingUnstated
            }
            Self::StackMembershipInferredOrUnverifiable => {
                M5ChangeOrchestrationDowngradeTrigger::StackMembershipInferredFromBranchNameAlone
            }
            Self::StackOrderDriftedOrRestackRequired => {
                M5ChangeOrchestrationDowngradeTrigger::StackMembersSilentlyReordered
            }
            Self::QueueOrProtectedBranchUnprovable => {
                M5ChangeOrchestrationDowngradeTrigger::LandedFromAmbientBranchState
            }
            Self::ValidationOrApprovalStale => {
                M5ChangeOrchestrationDowngradeTrigger::ValidationFreshnessUnstated
            }
            Self::CleanupEvidencePartial => {
                M5ChangeOrchestrationDowngradeTrigger::OrphanDeletedWithoutSafetyPreview
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::SelectedChangeBindingUnbound => "selected_change_binding_unbound",
            Self::StackMembershipInferredOrUnverifiable => {
                "stack_membership_inferred_or_unverifiable"
            }
            Self::StackOrderDriftedOrRestackRequired => "stack_order_drifted_or_restack_required",
            Self::QueueOrProtectedBranchUnprovable => "queue_or_protected_branch_unprovable",
            Self::ValidationOrApprovalStale => "validation_or_approval_stale",
            Self::CleanupEvidencePartial => "cleanup_evidence_partial",
        }
    }
}

/// One change-orchestration-truth dimension's observed condition on an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ChangeOrchestrationClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ChangeOrchestrationConditionState,
}

/// An honest claim auto-narrow block. When an AI-review-truth dimension weakens, the object's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// object identity / last-known state rather than silently dropping it — the underlying finding, scope,
/// publish, and lifecycle truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationClaimAutoNarrow {
    /// The claim the object is narrowed to.
    pub narrowed_to: M5ChangeOrchestrationA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5ChangeOrchestrationClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ChangeOrchestrationDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying provider / linked-change / handoff / resolution / blocker truth is preserved (never dropped) across the
    /// narrowing; must hold so provider-freshness-unverified, diff-scope-unverified,
    /// publish-target-unverified, and finding-lifecycle-unverified states never fail opaquely, and no local
    /// draft or evidence is lost.
    pub preserves_truth_continuity: bool,
}

impl ChangeOrchestrationClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and finding / scope /
    /// publish / lifecycle truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an object's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl ChangeOrchestrationCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ChangeOrchestrationRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ChangeOrchestrationNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an change-orchestration accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeOrchestrationAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl ChangeOrchestrationAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one change-orchestration object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationAccessibilityRow {
    /// Record kind; must equal [`CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen object this row certifies.
    pub object: M5ChangeOrchestrationObject,
    /// Ref to the frozen per-object domain schema this row certifies.
    pub source_object_schema_ref: String,
    /// Opaque ref to the object this row represents; stays visible on every surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a structure-heavy object must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ChangeOrchestrationFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, provider ownership / commit state,
    /// analyzed scope, publish mode / provider destination, local-versus-provider state, and finding
    /// lifecycle state as the rich object; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ChangeOrchestrationNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ChangeOrchestrationNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: ChangeOrchestrationNonVisualReachState,
    /// High-contrast / forced-colors behavior of the non-visual path.
    pub high_contrast_reach: ChangeOrchestrationNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ChangeOrchestrationNonVisualReachState,
    /// Whether the export-safe summary preserves object meaning.
    pub export_summary: ChangeOrchestrationExportSummaryState,
    /// Ref to the export-safe summary object for this object.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ChangeOrchestrationCopyExportParity,
    /// The full claim this object asserts when every dimension is intact.
    pub full_ready_claim: M5ChangeOrchestrationA11yClaim,
    /// The observed condition of each modeled AI-review-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ChangeOrchestrationClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the object's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ChangeOrchestrationClaimAutoNarrow>,
    /// Whether the underlying provider / linked-change / handoff / resolution / blocker truth is preserved on this object
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this object is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ChangeOrchestrationRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ChangeOrchestrationRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ChangeOrchestrationRequiredLabel>,
    /// Semantic consumer surfaces this object is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ChangeOrchestrationConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ChangeOrchestrationAccessibilityRow {
    /// Returns true when this object renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        object_is_structure_heavy(self.object)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5ChangeOrchestrationClaimDimension,
    ) -> M5ChangeOrchestrationConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ChangeOrchestrationConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// object's full claim.
    pub fn permitted_claim(&self) -> M5ChangeOrchestrationA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the object's full claim.
    pub fn binding_condition(&self) -> Option<&ChangeOrchestrationClaimConditionEntry> {
        let mut binding: Option<(&ChangeOrchestrationClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5ChangeOrchestrationClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this object effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ChangeOrchestrationA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-provider finding, a diff-drifted scope, an unavailable publish
    /// target, or an outdated / suppressed lifecycle state can no longer keep an old `TrustedProviderLandedSurface` /
    /// `LocalReviewableSurface` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and truth. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: a stale-pack / missing-owner / unevaluated-check / parity-diverged /
    /// undisclosed-AI-pack / stale-template state never keeps a trusted claim — a local handoff packet never
    /// masquerades as provider-committed from a narrowed object. When such a state is modeled, the
    /// effective claim must not assert `TrustedProviderLandedSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / CLI trap, a structure-heavy object offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.object_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the object meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying finding / scope / publish /
    /// lifecycle truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the object carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its object's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = object_primary_dimension(self.object);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ChangeOrchestrationRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ChangeOrchestrationAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ChangeOrchestrationAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ChangeOrchestrationAccessibilityStatus::NarrowedDisclosed
        } else {
            ChangeOrchestrationAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND
            && self.schema_version == CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_object_schema_ref.trim().is_empty()
            && !self.object_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "object={object} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            object = self.object.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1272 change-orchestration accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationAccessibilitySummary {
    pub row_count: usize,
    pub object_count: usize,
    pub structure_heavy_object_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ChangeOrchestrationAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeOrchestrationAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ChangeOrchestrationAccessibilityRow>,
}

/// Checked-in M05-1272 change-orchestration accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeOrchestrationAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ChangeOrchestrationAccessibilityRow>,
    pub summary: ChangeOrchestrationAccessibilitySummary,
}

impl ChangeOrchestrationAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ChangeOrchestrationAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            record_kind: CHANGE_ORCHESTRATION_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ChangeOrchestrationAccessibilitySummary {
                row_count: 0,
                object_count: 0,
                structure_heavy_object_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Objects represented by some row in this packet.
    pub fn represented_objects(&self) -> BTreeSet<M5ChangeOrchestrationObject> {
        self.rows.iter().map(|r| r.object).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ChangeOrchestrationClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ChangeOrchestrationConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ChangeOrchestrationA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ChangeOrchestrationConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ChangeOrchestrationAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ChangeOrchestrationConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&ChangeOrchestrationAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ChangeOrchestrationAccessibilityStatus::Parity => green += 1,
                ChangeOrchestrationAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ChangeOrchestrationAccessibilityStatus::Stranded => red += 1,
            }
        }

        ChangeOrchestrationAccessibilitySummary {
            row_count: self.rows.len(),
            object_count: self.represented_objects().len(),
            structure_heavy_object_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ChangeOrchestrationAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ChangeOrchestrationAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION {
            violations.push(ChangeOrchestrationAccessibilityViolation::SchemaVersion {
                expected: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CHANGE_ORCHESTRATION_A11Y_RECORD_KIND {
            violations.push(ChangeOrchestrationAccessibilityViolation::RecordKind {
                expected: CHANGE_ORCHESTRATION_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ChangeOrchestrationAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_objects = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ChangeOrchestrationAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_objects.insert(row.object);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(ChangeOrchestrationAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its object's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: object_primary_dimension(row.object),
                    },
                );
            }

            // Each row must preserve every mandatory object label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy object must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ChangeOrchestrationFallbackModality::Structured)
            {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / trusted honesty: a stale-provider / diff-drifted / publish-target-unavailable /
            // lifecycle-degraded state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve provider / linked-change / handoff / resolution / blocker truth.
            if !row.preserves_truth_continuity() {
                violations.push(ChangeOrchestrationAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == ChangeOrchestrationAccessibilityStatus::Stranded {
                violations.push(ChangeOrchestrationAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen object is certified at least once.
        for object in M5ChangeOrchestrationObject::ALL {
            if !seen_objects.contains(&object) {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingObjectCoverage { object },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ChangeOrchestrationClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ChangeOrchestrationConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → finding-lifecycle-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ChangeOrchestrationA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one stale-provider / diff-drifted /
        // publish-target-unavailable / lifecycle-degraded row in the packet, so the "cannot-prove never
        // shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(ChangeOrchestrationAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the review detail, AI panel, finding row, scope
        // selector, publish sheet, pending-review tray, provider publish review, resolution memory ledger,
        // and support / export packet — so every consumer surface is exercised at least once.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ChangeOrchestrationConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ChangeOrchestrationAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ChangeOrchestrationAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("change-orchestration accessibility parity packet serializes"),
        ) {
            violations.push(ChangeOrchestrationAccessibilityViolation::RawObjectMaterialInExport);
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
            .expect("change-orchestration accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,object,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{object},{keyboard},{screen_reader},{high_zoom},{high_contrast},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                object = row.object.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Change-Orchestration Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Objects: {} certified across {} / {} frozen objects\n",
            self.summary.object_count,
            self.represented_objects().len(),
            M5ChangeOrchestrationObject::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.object.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in change-orchestration accessibility parity export.
pub fn current_m5_change_orchestration_accessibility_parity_export(
) -> Result<ChangeOrchestrationAccessibilityPacket, ChangeOrchestrationAccessibilityArtifactError> {
    let packet: ChangeOrchestrationAccessibilityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-orchestration-accessibility-parity/support_export.json"
    )))
        .map_err(ChangeOrchestrationAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ChangeOrchestrationAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in change-orchestration accessibility parity export.
#[derive(Debug)]
pub enum ChangeOrchestrationAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ChangeOrchestrationAccessibilityViolation>),
}

impl fmt::Display for ChangeOrchestrationAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "change-orchestration accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "change-orchestration accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ChangeOrchestrationAccessibilityArtifactError {}

/// Validation failure for M05-1272 change-orchestration accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeOrchestrationAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5ChangeOrchestrationClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingObjectCoverage {
        object: M5ChangeOrchestrationObject,
    },
    MissingDimensionCoverage {
        dimension: M5ChangeOrchestrationClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ChangeOrchestrationConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ChangeOrchestrationA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ChangeOrchestrationConsumerSurface,
    },
    SummaryMismatch,
    RawObjectMaterialInExport,
}

impl ChangeOrchestrationAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingObjectCoverage { .. } => "missing_object_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawObjectMaterialInExport => "raw_object_material_in_export",
        }
    }
}

impl fmt::Display for ChangeOrchestrationAccessibilityViolation {
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
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its object's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory object label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows a stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded state as a trusted review surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve finding / scope / publish / lifecycle truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingObjectCoverage { object } => {
                write!(f, "object {object:?} is not certified in the packet")
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawObjectMaterialInExport => {
                write!(f, "export contains raw object material")
            }
        }
    }
}

impl Error for ChangeOrchestrationAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
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
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const CHANGE_ORCHESTRATION_A11Y_PACKET_ID: &str =
    "m5-change-orchestration-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in change-orchestration accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_change_orchestration_accessibility_parity_packet(
) -> ChangeOrchestrationAccessibilityPacket {
    ChangeOrchestrationAccessibilityPacket::new(ChangeOrchestrationAccessibilityPacketInput {
        packet_id: CHANGE_ORCHESTRATION_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: CHANGE_ORCHESTRATION_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!(
        "evidence:change-orchestration-accessibility-parity:{id}"
    )]
}

fn all_required_labels() -> Vec<M5ChangeOrchestrationRequiredLabel> {
    M5ChangeOrchestrationRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ChangeOrchestrationCopyExportParity {
    ChangeOrchestrationCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ChangeOrchestrationClaimDimension,
    state: M5ChangeOrchestrationConditionState,
) -> ChangeOrchestrationClaimConditionEntry {
    ChangeOrchestrationClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the support / export packet and the
/// change-object detail surface — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5ChangeOrchestrationConsumerSurface],
) -> Vec<M5ChangeOrchestrationConsumerSurface> {
    let mut out = vec![
        M5ChangeOrchestrationConsumerSurface::SupportExportPacket,
        M5ChangeOrchestrationConsumerSurface::ChangeObjectDetail,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: ChangeOrchestrationNarrowingDisclosureState,
) -> Vec<ChangeOrchestrationRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ChangeOrchestrationRenderingNarrowingDisclosure {
            rendering_surface: M5ChangeOrchestrationRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ChangeOrchestrationRenderingNarrowingDisclosure {
            rendering_surface: M5ChangeOrchestrationRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_publish_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ChangeOrchestrationRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ChangeOrchestrationNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ChangeOrchestrationRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ChangeOrchestrationNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ChangeOrchestrationRenderingSurface> {
    vec![
        M5ChangeOrchestrationRenderingSurface::DesktopFull,
        M5ChangeOrchestrationRenderingSurface::CliHeadless,
        M5ChangeOrchestrationRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ChangeOrchestrationFallbackModality> {
    vec![
        M5ChangeOrchestrationFallbackModality::List,
        M5ChangeOrchestrationFallbackModality::Textual,
        M5ChangeOrchestrationFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5ChangeOrchestrationFallbackModality> {
    vec![
        M5ChangeOrchestrationFallbackModality::Structured,
        M5ChangeOrchestrationFallbackModality::List,
        M5ChangeOrchestrationFallbackModality::Textual,
        M5ChangeOrchestrationFallbackModality::Cli,
    ]
}

const REACHABLE: ChangeOrchestrationNonVisualReachState =
    ChangeOrchestrationNonVisualReachState::ReachableAndLabeled;
const REDUCED: ChangeOrchestrationNonVisualReachState =
    ChangeOrchestrationNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<ChangeOrchestrationAccessibilityRow> {
    vec![
        // Change object (selected-change bound) — the change object keeps its selected worktree / base
        // binding, disclosed stack membership, and landing state explicit, so it is a fully
        // provider-authoritative, landed surface reachable on every surface with no narrowing (green).
        // Keyboard-only and screen-reader users can inspect, stack, land, reopen, and export it without losing
        // the worktree binding or membership truth, and no command or provider action can mutate another
        // worktree from ambient branch state.
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:change-object-selected-change-bound".to_owned(),
            object: M5ChangeOrchestrationObject::ChangeObject,
            source_object_schema_ref: M5ChangeOrchestrationObject::ChangeObject
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:change-object:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:change-object-selected-change-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "selected_worktree_base_binding",
                "stack_membership_source",
                "landing_state",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::SelectedChangeBindingClarity,
                M5ChangeOrchestrationConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "selected_worktree_base_binding",
                "stack_membership_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::PatchStackQueue,
                M5ChangeOrchestrationConsumerSurface::ProviderMergeQueue,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.6.2 — explicit change objects & selected worktree binding".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("change-object-selected-change-bound"),
        },
        // Worktree-cleanup preview (evidence bound) — structure-heavy (affected running tasks / open editors /
        // uncommitted-change scope / reflog / checkpoint recovery); it keeps its cleanup evidence bound, so it
        // is a self-sufficient, locally reviewable surface a user can inspect, with full parity on every
        // surface (green). Its structured affected-work / recovery set binds to a flat list / textual path.
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:worktree-cleanup-preview-evidence-bound".to_owned(),
            object: M5ChangeOrchestrationObject::WorktreeCleanupPreview,
            source_object_schema_ref: M5ChangeOrchestrationObject::WorktreeCleanupPreview
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:worktree-cleanup-preview:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:worktree-cleanup-preview-evidence-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "affected_running_work",
                "recovery_checkpoint_lineage",
                "cleanup_safety_state",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::LocalReviewableSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::CleanupEvidenceClarity,
                M5ChangeOrchestrationConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "affected_running_work",
                "recovery_checkpoint_lineage",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::PortableShelf,
                M5ChangeOrchestrationConsumerSurface::HelpDocs,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22 — worktree-manager & cleanup preview".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("worktree-cleanup-preview-evidence-bound"),
        },
        // Change object (selected worktree binding unbound) — the change object's selected worktree / base
        // binding is unbound, so it auto-narrows to a selected-change-binding-unverified projection that keeps
        // the last-known change identity visible without letting any command, tool, or provider action mutate
        // another worktree from ambient branch state (yellow). Its screen-reader traversal discloses a reduced
        // linear walk.
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:change-object-worktree-binding-unbound".to_owned(),
            object: M5ChangeOrchestrationObject::ChangeObject,
            source_object_schema_ref: M5ChangeOrchestrationObject::ChangeObject
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:change-object:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:change-object-worktree-binding-unbound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "last_known_change_identity",
                "unbound_worktree_binding_reason",
                "selected_worktree_base_binding",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::SelectedChangeBindingClarity,
                M5ChangeOrchestrationConditionState::SelectedChangeBindingUnbound,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::SelectedChangeBindingUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::SelectedChangeBindingClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::WorktreeBindingUnstated,
                narrowed_label:
                    "This change object's selected worktree / base binding is unbound — shown as a selected-change-binding-unverified projection that keeps the change identity, last-known worktree / base fingerprint, and stack membership explicit, never letting a command, tool, refactor, formatter, or provider action mutate another worktree from ambient branch state without an explicit selected change object and worktree binding"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "last_known_change_identity",
                "unbound_worktree_binding_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::PatchStackQueue,
                M5ChangeOrchestrationConsumerSurface::ReviewDetail,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.6.2 — no hidden cross-worktree writes".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("change-object-worktree-binding-unbound"),
        },
        // Patch-stack queue (membership inferred / unverifiable) — the stack membership would rely on branch
        // names alone, so it auto-narrows to a stack-membership-unverified projection that keeps the stack ID,
        // disclosed ordered member set, and parent-child links explicit, never inferring stack membership from
        // branch names alone (yellow).
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:patch-stack-queue-membership-inferred".to_owned(),
            object: M5ChangeOrchestrationObject::PatchStackQueue,
            source_object_schema_ref: M5ChangeOrchestrationObject::PatchStackQueue
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:patch-stack-queue:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:patch-stack-queue-membership-inferred:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "stack_membership_source",
                "ordered_member_set",
                "landing_state",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::StackMembershipClarity,
                M5ChangeOrchestrationConditionState::StackMembershipInferredOrUnverifiable,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::StackMembershipUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::StackMembershipClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::StackMembershipInferredFromBranchNameAlone,
                narrowed_label:
                    "This patch-stack queue's membership is inferred or unverifiable (would rely on branch names alone) — shown as a stack-membership-unverified projection that keeps the stack ID, disclosed ordered member set, and parent-child links explicit, never inferring stack membership from branch names alone"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "stack_membership_source",
                "ordered_member_set",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::StackEditReviewSheet,
                M5ChangeOrchestrationConsumerSurface::ProviderMergeQueue,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §13 — stack / dependency strip".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("patch-stack-queue-membership-inferred"),
        },
        // Stack-edit review sheet (order drifted / restack required) — structure-heavy (original / proposed
        // order and parent-child links); the stack order is drifted or a restack is required here, so it
        // auto-narrows to a stack-order-unverified projection that keeps the original and proposed order and
        // affected parent-child links explicit, never silently reordering, collapsing, or retargeting stack
        // members (yellow). Its dense reflow narrows the high-zoom legibility to a disclosed reduction.
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:stack-edit-review-sheet-order-drifted".to_owned(),
            object: M5ChangeOrchestrationObject::StackEditReviewSheet,
            source_object_schema_ref: M5ChangeOrchestrationObject::StackEditReviewSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:stack-edit-review-sheet:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:stack-edit-review-sheet-order-drifted:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "original_and_proposed_order",
                "affected_parent_child_links",
                "restack_required_reason",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::StackOrderIntegrityClarity,
                M5ChangeOrchestrationConditionState::StackOrderDriftedOrRestackRequired,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::StackOrderUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::StackOrderIntegrityClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::StackMembersSilentlyReordered,
                narrowed_label:
                    "This stack-edit review sheet's order is drifted or a restack is required (base moved / a member landed / validation staled) — shown as a stack-order-unverified projection that keeps the original and proposed order and affected parent-child links explicit, never silently reordering, collapsing, or retargeting stack members"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "original_and_proposed_order",
                "affected_parent_child_links",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::PatchStackQueue,
                M5ChangeOrchestrationConsumerSurface::WorktreeCleanupPreview,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22 — reusable stack-dependency commands".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("stack-edit-review-sheet-order-drifted"),
        },
        // Landing candidate sheet (queue authority unavailable / protected-branch unprovable) — the queue
        // authority is unavailable or the protected-branch rule cannot be proven, so it auto-narrows to a
        // landing-authority-unverified projection that keeps the exact target branch, merge strategy, required
        // checks, and queue eligibility as a local estimate, never widening a local landing estimate into a
        // provider-authoritative land or landing from ambient branch state (yellow).
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:landing-candidate-sheet-queue-authority-unavailable".to_owned(),
            object: M5ChangeOrchestrationObject::LandingCandidateSheet,
            source_object_schema_ref: M5ChangeOrchestrationObject::LandingCandidateSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:landing-candidate-sheet:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:landing-candidate-sheet-queue-authority-unavailable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "target_branch_and_merge_strategy",
                "queue_authority_source",
                "protected_branch_posture",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::LandingAuthorityClarity,
                M5ChangeOrchestrationConditionState::QueueOrProtectedBranchUnprovable,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::LandingAuthorityUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::LandingAuthorityClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::LandedFromAmbientBranchState,
                narrowed_label:
                    "This landing candidate's queue authority is unavailable or its protected-branch rule cannot be proven — shown as a landing-authority-unverified projection that keeps the exact target branch, merge strategy, required checks, and queue eligibility as a local estimate, never widening a local landing estimate into a provider-authoritative land or landing from ambient branch state"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "target_branch_and_merge_strategy",
                "queue_authority_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::ProviderMergeQueue,
                M5ChangeOrchestrationConsumerSurface::ReviewDetail,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §13 — merge-queue & provider-authoritative-versus-local-estimate".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("landing-candidate-sheet-queue-authority-unavailable"),
        },
        // Portable shelf (validation / approval stale) — the validation or approval evidence is stale, so it
        // auto-narrows to a validation-freshness-unverified projection that keeps the bundle ID, diff /
        // evidence refs, review-pack version, and redaction profile explicit, never presenting stale
        // validation or approval evidence as currently green (yellow).
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:portable-shelf-validation-stale".to_owned(),
            object: M5ChangeOrchestrationObject::PortableShelf,
            source_object_schema_ref: M5ChangeOrchestrationObject::PortableShelf
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:portable-shelf:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REDUCED,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:portable-shelf-validation-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "bundle_and_diff_refs",
                "stale_validation_or_approval_reason",
                "review_pack_version",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::ValidationFreshnessClarity,
                M5ChangeOrchestrationConditionState::ValidationOrApprovalStale,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::ValidationFreshnessUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::ValidationFreshnessClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::ValidationFreshnessUnstated,
                narrowed_label:
                    "This portable shelf's validation or approval evidence is stale — shown as a validation-freshness-unverified projection that keeps the bundle ID, diff / evidence refs, review-pack version, and redaction profile explicit, never presenting stale validation or approval evidence as currently green or as hosted-authoritative truth"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "bundle_and_diff_refs",
                "stale_validation_or_approval_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::PortableShelf,
                M5ChangeOrchestrationConsumerSurface::WorktreeCleanupPreview,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.6.2 — portable bundles / shelves & stale validation".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("portable-shelf-validation-stale"),
        },
        // Worktree-cleanup preview (orphan-cleanup evidence partial) — structure-heavy (affected running
        // tasks / open editors / uncommitted-change scope / reflog / checkpoint recovery); the orphan-cleanup
        // evidence is partial, so it auto-narrows to a cleanup-evidence-unverified projection that keeps
        // affected running work, uncommitted-change scope, and reflog / checkpoint recovery explicit, never
        // deleting an orphaned worktree or stale stack member without a safety preview (yellow).
        ChangeOrchestrationAccessibilityRow {
            record_kind: CHANGE_ORCHESTRATION_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_ORCHESTRATION_A11Y_SCHEMA_VERSION,
            row_id: "a11y:worktree-cleanup-preview-evidence-partial".to_owned(),
            object: M5ChangeOrchestrationObject::WorktreeCleanupPreview,
            source_object_schema_ref: M5ChangeOrchestrationObject::WorktreeCleanupPreview
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "change:worktree-cleanup-preview:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeOrchestrationExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:worktree-cleanup-preview-evidence-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "affected_running_work",
                "uncommitted_change_scope",
                "recovery_checkpoint_lineage",
            ]),
            full_ready_claim: M5ChangeOrchestrationA11yClaim::TrustedProviderLandedSurface,
            claim_conditions: vec![condition(
                M5ChangeOrchestrationClaimDimension::CleanupEvidenceClarity,
                M5ChangeOrchestrationConditionState::CleanupEvidencePartial,
            )],
            claim_narrow: Some(ChangeOrchestrationClaimAutoNarrow {
                narrowed_to: M5ChangeOrchestrationA11yClaim::CleanupEvidenceUnverifiedProjection,
                binding_dimension: M5ChangeOrchestrationClaimDimension::CleanupEvidenceClarity,
                trigger: M5ChangeOrchestrationDowngradeTrigger::OrphanDeletedWithoutSafetyPreview,
                narrowed_label:
                    "This worktree-cleanup preview's orphan-cleanup evidence is partial — shown as a cleanup-evidence-unverified projection that keeps affected running tasks, open editors, uncommitted-change scope, and reflog / checkpoint recovery explicit, never deleting an orphaned worktree or stale stack member without previewing running work and export-safe recovery evidence"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "affected_running_work",
                "uncommitted_change_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeOrchestrationConsumerSurface::HelpDocs,
                M5ChangeOrchestrationConsumerSurface::StackEditReviewSheet,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22 — cleanup preview & recovery evidence".to_owned(),
                CHANGE_ORCHESTRATION_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-18T00:00:00Z".to_owned(),
            evidence_refs: ev("worktree-cleanup-preview-evidence-partial"),
        },
    ]
}
