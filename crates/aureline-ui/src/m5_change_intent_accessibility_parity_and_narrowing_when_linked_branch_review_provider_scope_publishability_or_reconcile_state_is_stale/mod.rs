//! Keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity, and honest automatic claim
//! narrowing for the M5 change-intent record / start-work sheet / linked-change panel / ready-for-review
//! handoff sheet / resolve-close sheet / blocked-escalate card objects.
//!
//! This module is the M05-1292 accessibility-and-auto-narrowing capstone over the frozen M5 change-intent and
//! engineering-lifecycle matrix ([`crate::m5_change_intent_and_engineering_lifecycle_matrix`]). Where the
//! freeze matrix defines the reusable change-intent record, start-work sheet, linked-change panel,
//! ready-for-review handoff sheet, resolve-close sheet, and blocked-escalate card objects, and the 1285-1291
//! implementation lanes resolve their per-surface truth, this lane certifies — per object class — that
//! change-intent, start-work, handoff, resolve, and blocker claims stay **keyboard-complete,
//! assistive-tech-reachable, high-zoom / high-contrast-safe, CLI/export-safe, and self-narrowing** rather than
//! presenting a local-only / reconcile-required commit state, an undisclosed start-work side effect, a stale
//! or broken linked branch / worktree / review relation, a blocked handoff packet publishability, a
//! local-only resolution, or an unresolved blocker as still a fully provider-committed, publish-safe surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / CLI reach.** Every object exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, high-contrast-safe, and
//!   CLI/headless-reachable path into the same object identity, provider ownership, local-versus-provider
//!   commit state, linked branch / worktree / review identity, relation source, publishability, resolution
//!   authority, and blocker state the rich object shows — never a color-only relation badge, a hover-only
//!   authority pill, or a pointer-only publish affordance that strands assistive-tech or headless-CLI users.
//!   Structure-heavy objects (the linked-change panel's relation rows, the blocked-escalate card's blocker
//!   class / dependency / escalation set) additionally bind their structured layout to a flat list / textual
//!   path.
//! - **Export parity.** The support / CLI / release export reconstructs each object's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same provider ownership, commit state,
//!   relation source, publishability, resolution authority, and blocker labels visible in-product so support,
//!   help, and release proof can reconstruct exactly what the user was shown without leaking a raw diff hunk,
//!   message payload, secret, endpoint, or provider token.
//! - **Honest auto-narrowing.** When a change-intent commit state is local-only or reconcile-required, a
//!   start-work side effect is undisclosed, a linked branch / worktree / review relation is stale or broken,
//!   provider write scope is missing so a handoff packet is not publishable, a resolution is local-only, or a
//!   blocker is unresolved, the object's claim auto-narrows from `trusted_provider_committed_surface` /
//!   `local_reviewable_surface` to a provider-commit-state-unverified / side-effect-disclosure-unverified /
//!   linked-relation-unverified / handoff-publishability-unverified / resolution-authority-unverified /
//!   blocker-continuity-unverified projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical object identity / last-known state. The underlying provider,
//!   linked-change, handoff, resolution, and blocker truth is never dropped opaquely. An object with every
//!   dimension intact must NOT carry a spurious narrowing, and a local-only-commit / undisclosed-side-effect /
//!   stale-relation / publishability-blocked / local-only-resolution / unresolved-blocker state can never keep
//!   a fully provider-committed, publish-safe claim — a local handoff packet or queued publish never
//!   masquerades as a provider-committed update, and linked-by-provider, linked-locally, suggested, and
//!   stale-or-broken relations are never flattened into one badge.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the work-item detail, start-work
//!   sheet, linked-change panel, review detail, ready-for-review handoff, resolve-close sheet,
//!   blocked-escalate card, support / export packet, and help / docs so product, help, and release
//!   publication stay aligned on downgrade behavior rather than drifting in copy — a trusted-looking object
//!   can never outrun the provider ownership, commit state, relation source, publishability, resolution
//!   authority, or blocker evidence it is being viewed away from.
//!
//! Each [`ChangeIntentAccessibilityRow`] keys on one
//! [`crate::m5_change_intent_and_engineering_lifecycle_matrix::M5ChangeIntentObject`] and reuses that frozen
//! object vocabulary plus the frozen [`M5ChangeIntentRequiredLabel`], [`M5ChangeIntentDowngradeTrigger`], and
//! shared [`M5ChangeIntentConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling change-intent packets.
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

// Reused frozen change-intent vocabulary — the capstone certifies the freeze matrix's objects, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_change_intent_and_engineering_lifecycle_matrix::{
    M5ChangeIntentConsumerSurface, M5ChangeIntentDowngradeTrigger, M5ChangeIntentObject,
    M5ChangeIntentRequiredLabel, M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1282 change-intent accessibility parity packet.
pub const CHANGE_INTENT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ChangeIntentAccessibilityPacket`].
pub const CHANGE_INTENT_A11Y_RECORD_KIND: &str = "m5_change_intent_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`ChangeIntentAccessibilityRow`].
pub const CHANGE_INTENT_A11Y_ROW_RECORD_KIND: &str = "m5_change_intent_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const CHANGE_INTENT_A11Y_SCHEMA_REF: &str =
    "schemas/teamwork/m5-change-intent-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const CHANGE_INTENT_A11Y_DOC_REF: &str =
    "docs/team-workflows/m5_change_intent_accessibility_parity.md";

/// Repo-relative path of the frozen change-intent and engineering-lifecycle matrix this lane certifies.
pub const CHANGE_INTENT_A11Y_MATRIX_REF: &str = M5_CHANGE_INTENT_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const CHANGE_INTENT_A11Y_FIXTURE_DIR: &str =
    "fixtures/teamwork/m5-change-intent-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CHANGE_INTENT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-change-intent-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CHANGE_INTENT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-change-intent-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CHANGE_INTENT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-change-intent-accessibility-parity.md";

/// The reusable objects that render a dense, structured surface (the linked branch / worktree / review relation set, the
/// blocked-escalate card's blocker class / dependency / escalation set) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable non-visually.
const fn object_is_structure_heavy(object: M5ChangeIntentObject) -> bool {
    matches!(
        object,
        M5ChangeIntentObject::LinkedChangePanel | M5ChangeIntentObject::BlockedEscalateCard
    )
}

/// The change-intent-truth dimension whose weakening an object primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn object_primary_dimension(object: M5ChangeIntentObject) -> M5ChangeIntentClaimDimension {
    match object {
        M5ChangeIntentObject::ChangeIntentRecord => {
            M5ChangeIntentClaimDimension::ProviderCommitStateClarity
        }
        M5ChangeIntentObject::StartWorkSheet => {
            M5ChangeIntentClaimDimension::SideEffectDisclosureClarity
        }
        M5ChangeIntentObject::LinkedChangePanel => {
            M5ChangeIntentClaimDimension::LinkedRelationSourceClarity
        }
        M5ChangeIntentObject::ReadyForReviewHandoffSheet => {
            M5ChangeIntentClaimDimension::HandoffPublishabilityClarity
        }
        M5ChangeIntentObject::ResolveCloseSheet => {
            M5ChangeIntentClaimDimension::ResolutionAuthorityClarity
        }
        M5ChangeIntentObject::BlockedEscalateCard => {
            M5ChangeIntentClaimDimension::BlockerContinuityClarity
        }
    }
}

/// A rendered fallback modality for an change-intent object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentFallbackModality {
    /// A rich, structured (outbound action set / lifecycle history) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ChangeIntentFallbackModality {
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
pub enum M5ChangeIntentRenderingSurface {
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

impl M5ChangeIntentRenderingSurface {
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
pub enum ChangeIntentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl ChangeIntentNonVisualReachState {
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
pub enum ChangeIntentExportSummaryState {
    /// The object meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl ChangeIntentExportSummaryState {
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
pub enum ChangeIntentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ChangeIntentNarrowingDisclosureState {
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

/// The change-intent claim ceiling an object asserts: how strong a provider-committed, publish-safe posture it lets
/// a surface present. Auto-narrowing lowers this ceiling when a pack-version / owner-provenance /
/// evidence-check / local-parity / resolution-authority / template-attribution dimension weakens so a stale pack
/// version / digest, a missing relation source, an stale or broken relation, a divergent parity estimate, an
/// undisclosed AI provider commit state, or a stale blocker continuity can never keep an old `TrustedProviderCommittedSurface`
/// or `LocalReviewableSurface` label — a local handoff packet never masquerades as provider-committed
/// from a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentA11yClaim {
    /// Trusted review surface: a fully pack-versioned, owner-provenanced, evidence-evaluated,
    /// parity-disclosed, pack-bound, template-attributed object — the strongest claim, a change-intent surface
    /// Aureline can present as exactly provider-committed and publish-safe to inspect, rerun, compare, export, or
    /// reopen right now.
    TrustedProviderCommittedSurface,
    /// Reviewable review surface: a self-sufficient, reviewable read-only object (a blocked-escalate card a
    /// user can inspect) that is not itself an authoritative, publishability-driving surface.
    LocalReviewableSurface,
    /// Pack-version-unverified projection: the provider commit state is stale; the object stays a
    /// provider-commit-state-unverified projection with its last-known provider commit identity preserved, never a stale pack
    /// version / digest shown as current, provider-committed truth.
    ProviderCommitStateUnverifiedProjection,
    /// Owner-provenance-unverified projection: the advisory-versus-enforced relation source is missing; the
    /// object stays an side-effect-disclosure-unverified projection that keeps linked-by-provider and linked-locally relations
    /// distinct, never flattening them into one relation badge.
    SideEffectDisclosureUnverifiedProjection,
    /// Evidence-check-unverified projection: a linked relation is unevaluated here (ci-only /
    /// not-evaluated-here / provider-unavailable); the object stays an linked-relation-unverified projection
    /// that keeps the evaluation state explicit, never folding an stale or broken relation into a one flattened relation badge.
    LinkedRelationUnverifiedProjection,
    /// Local-parity-unverified projection: a local handoff packet diverges from provider-committed
    /// state; the object stays a handoff-publishability-unverified projection that names the capability difference,
    /// never widening a local estimate into a provider-committed update.
    HandoffPublishabilityUnverifiedProjection,
    /// AI-provider-commit-state-unverified projection: the change-intent lifecycle ran under an undisclosed or different pack
    /// version; the object stays an ai-provider-commit-state-unverified projection that discloses the resolution authority binding,
    /// never presenting an change-intent lifecycle under a different provider commit state as provider-accepted.
    ResolutionAuthorityUnverifiedProjection,
    /// Template-attribution-unverified projection: the blocker cause and local handoff continuity is stale; the
    /// object stays a blocker-continuity-unverified projection that keeps the blocker class / attached evidence
    /// visible, never dropping blocker continuity on export or reopen.
    BlockerContinuityUnverifiedProjection,
}

impl M5ChangeIntentA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedProviderCommittedSurface,
        Self::LocalReviewableSurface,
        Self::ProviderCommitStateUnverifiedProjection,
        Self::SideEffectDisclosureUnverifiedProjection,
        Self::LinkedRelationUnverifiedProjection,
        Self::HandoffPublishabilityUnverifiedProjection,
        Self::ResolutionAuthorityUnverifiedProjection,
        Self::BlockerContinuityUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedProviderCommittedSurface => 7,
            Self::LocalReviewableSurface => 6,
            Self::ProviderCommitStateUnverifiedProjection => 5,
            Self::SideEffectDisclosureUnverifiedProjection => 4,
            Self::LinkedRelationUnverifiedProjection => 3,
            Self::HandoffPublishabilityUnverifiedProjection => 2,
            Self::ResolutionAuthorityUnverifiedProjection => 1,
            Self::BlockerContinuityUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully provider-committed, publish-safe review surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedProviderCommittedSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedProviderCommittedSurface | Self::LocalReviewableSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedProviderCommittedSurface => "trusted_provider_committed_surface",
            Self::LocalReviewableSurface => "local_reviewable_surface",
            Self::ProviderCommitStateUnverifiedProjection => {
                "provider_commit_state_unverified_projection"
            }
            Self::SideEffectDisclosureUnverifiedProjection => {
                "side_effect_disclosure_unverified_projection"
            }
            Self::LinkedRelationUnverifiedProjection => "linked_relation_unverified_projection",
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

/// The provider-commit-state / side-effect-disclosure / linked-relation / handoff-publishability /
/// resolution-authority / blocker-continuity dimension whose state governs how far an object may claim to be
/// a fully provider-committed, publish-safe surface. The dimensions map to the six frozen change-intent
/// objects so every object carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentClaimDimension {
    /// Provider-commit-state clarity: does the change-intent record keep its provider ownership and
    /// local-versus-provider commit state current so a local draft or queued publish never reads as a
    /// provider-committed update (change-intent-record)?
    ProviderCommitStateClarity,
    /// Side-effect-disclosure clarity: does the start-work sheet keep each create-branch / worktree /
    /// review-draft / provider-link side effect separately disclosed rather than silently creating one
    /// (start-work-sheet)?
    SideEffectDisclosureClarity,
    /// Linked-relation-source clarity: does the linked-change panel keep its relation source (linked-by-provider
    /// / linked-locally / suggested / stale-or-broken) explicit rather than flattening it into one relation
    /// badge or keeping a stale relation green (linked-change-panel)?
    LinkedRelationSourceClarity,
    /// Handoff-publishability clarity: does the ready-for-review handoff keep its queued local packet distinct
    /// from a provider-committed update and name the publishability blocker (offline / missing write scope /
    /// policy-blocked / partial) rather than implying provider acceptance (ready-for-review-handoff-sheet)?
    HandoffPublishabilityClarity,
    /// Resolution-authority clarity: does the resolve-close sheet keep a local-only resolution distinct from a
    /// provider-accepted terminal state and name unresolved engineering blockers rather than auto-resolving
    /// (resolve-close-sheet)?
    ResolutionAuthorityClarity,
    /// Blocker-continuity clarity: does the blocked-escalate card keep its blocker class, missing dependency /
    /// approval, and attached local evidence explicit rather than dropping local notes or letting a local
    /// handoff packet masquerade as a provider escalation (blocked-escalate-card)?
    BlockerContinuityClarity,
}

impl M5ChangeIntentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderCommitStateClarity,
        Self::SideEffectDisclosureClarity,
        Self::LinkedRelationSourceClarity,
        Self::HandoffPublishabilityClarity,
        Self::ResolutionAuthorityClarity,
        Self::BlockerContinuityClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderCommitStateClarity => "provider_commit_state_clarity",
            Self::SideEffectDisclosureClarity => "side_effect_disclosure_clarity",
            Self::LinkedRelationSourceClarity => "linked_relation_source_clarity",
            Self::HandoffPublishabilityClarity => "handoff_publishability_clarity",
            Self::ResolutionAuthorityClarity => "resolution_authority_clarity",
            Self::BlockerContinuityClarity => "blocker_continuity_clarity",
        }
    }
}

/// The observed condition of one change-intent-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the object's claim. The stale / missing / unevaluated / diverged states
/// the lane must auto-narrow on — a stale provider commit state, a missing relation source, an unevaluated
/// linked relation, a local-versus-provider parity capability difference, an change-intent lifecycle under an undisclosed
/// provider commit state, and a stale blocker continuity — are the states that [`Self::cannot_be_shown_trusted`]
/// flags: each is a genuine truth degradation that can never be shown as a fully provider-committed, publish-safe
/// review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChangeIntentConditionState {
    /// Fully pack-versioned, owner-provenanced, evidence-evaluated, parity-disclosed, pack-bound,
    /// template-attributed — imposes no ceiling.
    FullyQualified,
    /// The provider commit state is stale — claim drops to a provider-commit-state-unverified projection.
    LocalOnlyOrReconcileRequired,
    /// The advisory-versus-enforced relation source is missing — claim drops to an
    /// side-effect-disclosure-unverified projection.
    SideEffectUndisclosed,
    /// A linked relation is unevaluated here (ci-only / not-evaluated-here / provider-unavailable) — claim
    /// drops to an linked-relation-unverified projection.
    LinkedRelationStaleOrBroken,
    /// A local handoff packet diverges from provider-committed state — claim drops to a
    /// handoff-publishability-unverified projection.
    HandoffPublishabilityBlocked,
    /// The resolution was applied locally without provider acceptance — claim drops to an
    /// ai-provider-commit-state-unverified projection.
    ResolutionAuthorityLocalOnly,
    /// The blocker cause and local handoff continuity is stale — claim drops to a
    /// blocker-continuity-unverified projection.
    BlockerUnresolvedOrMasquerade,
}

impl M5ChangeIntentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::LocalOnlyOrReconcileRequired,
        Self::SideEffectUndisclosed,
        Self::LinkedRelationStaleOrBroken,
        Self::HandoffPublishabilityBlocked,
        Self::ResolutionAuthorityLocalOnly,
        Self::BlockerUnresolvedOrMasquerade,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects a weakened state that cannot be shown as a fully
    /// provider-committed, publish-safe review surface and must never be shown as such. Every weak change-intent
    /// condition is a genuine truth degradation, so all six flag here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::LocalOnlyOrReconcileRequired
                | Self::SideEffectUndisclosed
                | Self::LinkedRelationStaleOrBroken
                | Self::HandoffPublishabilityBlocked
                | Self::ResolutionAuthorityLocalOnly
                | Self::BlockerUnresolvedOrMasquerade
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ChangeIntentA11yClaim {
        match self {
            Self::FullyQualified => M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            Self::LocalOnlyOrReconcileRequired => {
                M5ChangeIntentA11yClaim::ProviderCommitStateUnverifiedProjection
            }
            Self::SideEffectUndisclosed => {
                M5ChangeIntentA11yClaim::SideEffectDisclosureUnverifiedProjection
            }
            Self::LinkedRelationStaleOrBroken => {
                M5ChangeIntentA11yClaim::LinkedRelationUnverifiedProjection
            }
            Self::HandoffPublishabilityBlocked => {
                M5ChangeIntentA11yClaim::HandoffPublishabilityUnverifiedProjection
            }
            Self::ResolutionAuthorityLocalOnly => {
                M5ChangeIntentA11yClaim::ResolutionAuthorityUnverifiedProjection
            }
            Self::BlockerUnresolvedOrMasquerade => {
                M5ChangeIntentA11yClaim::BlockerContinuityUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ChangeIntentDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5ChangeIntentDowngradeTrigger::ChangeIntentMatrixStale,
            Self::LocalOnlyOrReconcileRequired => {
                M5ChangeIntentDowngradeTrigger::LocalVersusProviderStateUnstated
            }
            Self::SideEffectUndisclosed => M5ChangeIntentDowngradeTrigger::SilentSideEffectCreated,
            Self::LinkedRelationStaleOrBroken => {
                M5ChangeIntentDowngradeTrigger::RelationSourceUnstated
            }
            Self::HandoffPublishabilityBlocked => {
                M5ChangeIntentDowngradeTrigger::LocalHandoffShownAsProviderCommitted
            }
            Self::ResolutionAuthorityLocalOnly => {
                M5ChangeIntentDowngradeTrigger::AutoResolvedWithOpenBlocker
            }
            Self::BlockerUnresolvedOrMasquerade => {
                M5ChangeIntentDowngradeTrigger::BlockerStateUnstated
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::LocalOnlyOrReconcileRequired => "local_only_or_reconcile_required",
            Self::SideEffectUndisclosed => "side_effect_undisclosed",
            Self::LinkedRelationStaleOrBroken => "linked_relation_stale_or_broken",
            Self::HandoffPublishabilityBlocked => "handoff_publishability_blocked",
            Self::ResolutionAuthorityLocalOnly => "resolution_authority_local_only",
            Self::BlockerUnresolvedOrMasquerade => "blocker_unresolved_or_masquerade",
        }
    }
}

/// One change-intent-truth dimension's observed condition on an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ChangeIntentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ChangeIntentConditionState,
}

/// An honest claim auto-narrow block. When an AI-review-truth dimension weakens, the object's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// object identity / last-known state rather than silently dropping it — the underlying finding, scope,
/// publish, and lifecycle truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentClaimAutoNarrow {
    /// The claim the object is narrowed to.
    pub narrowed_to: M5ChangeIntentA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5ChangeIntentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ChangeIntentDowngradeTrigger,
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

impl ChangeIntentClaimAutoNarrow {
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
pub struct ChangeIntentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl ChangeIntentCopyExportParity {
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
pub struct ChangeIntentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ChangeIntentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ChangeIntentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an change-intent accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl ChangeIntentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one change-intent object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentAccessibilityRow {
    /// Record kind; must equal [`CHANGE_INTENT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CHANGE_INTENT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen object this row certifies.
    pub object: M5ChangeIntentObject,
    /// Ref to the frozen per-object domain schema this row certifies.
    pub source_object_schema_ref: String,
    /// Opaque ref to the object this row represents; stays visible on every surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a structure-heavy object must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ChangeIntentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, provider ownership / commit state,
    /// analyzed scope, publish mode / provider destination, local-versus-provider state, and finding
    /// lifecycle state as the rich object; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ChangeIntentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ChangeIntentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: ChangeIntentNonVisualReachState,
    /// High-contrast / forced-colors behavior of the non-visual path.
    pub high_contrast_reach: ChangeIntentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ChangeIntentNonVisualReachState,
    /// Whether the export-safe summary preserves object meaning.
    pub export_summary: ChangeIntentExportSummaryState,
    /// Ref to the export-safe summary object for this object.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ChangeIntentCopyExportParity,
    /// The full claim this object asserts when every dimension is intact.
    pub full_ready_claim: M5ChangeIntentA11yClaim,
    /// The observed condition of each modeled AI-review-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ChangeIntentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the object's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ChangeIntentClaimAutoNarrow>,
    /// Whether the underlying provider / linked-change / handoff / resolution / blocker truth is preserved on this object
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this object is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ChangeIntentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ChangeIntentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ChangeIntentRequiredLabel>,
    /// Semantic consumer surfaces this object is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ChangeIntentConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ChangeIntentAccessibilityRow {
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
        dimension: M5ChangeIntentClaimDimension,
    ) -> M5ChangeIntentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ChangeIntentConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// object's full claim.
    pub fn permitted_claim(&self) -> M5ChangeIntentA11yClaim {
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
    pub fn binding_condition(&self) -> Option<&ChangeIntentClaimConditionEntry> {
        let mut binding: Option<(&ChangeIntentClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5ChangeIntentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this object effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ChangeIntentA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-provider finding, a diff-drifted scope, an unavailable publish
    /// target, or an outdated / suppressed lifecycle state can no longer keep an old `TrustedProviderCommittedSurface` /
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
    /// effective claim must not assert `TrustedProviderCommittedSurface`.
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
        M5ChangeIntentRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ChangeIntentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ChangeIntentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ChangeIntentAccessibilityStatus::NarrowedDisclosed
        } else {
            ChangeIntentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CHANGE_INTENT_A11Y_ROW_RECORD_KIND
            && self.schema_version == CHANGE_INTENT_A11Y_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1272 change-intent accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentAccessibilitySummary {
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

/// Constructor input for [`ChangeIntentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeIntentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ChangeIntentAccessibilityRow>,
}

/// Checked-in M05-1272 change-intent accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeIntentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ChangeIntentAccessibilityRow>,
    pub summary: ChangeIntentAccessibilitySummary,
}

impl ChangeIntentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ChangeIntentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            record_kind: CHANGE_INTENT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ChangeIntentAccessibilitySummary {
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
    pub fn represented_objects(&self) -> BTreeSet<M5ChangeIntentObject> {
        self.rows.iter().map(|r| r.object).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ChangeIntentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ChangeIntentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ChangeIntentA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ChangeIntentConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ChangeIntentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ChangeIntentConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&ChangeIntentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ChangeIntentAccessibilityStatus::Parity => green += 1,
                ChangeIntentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ChangeIntentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ChangeIntentAccessibilitySummary {
            row_count: self.rows.len(),
            object_count: self.represented_objects().len(),
            structure_heavy_object_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ChangeIntentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ChangeIntentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CHANGE_INTENT_A11Y_SCHEMA_VERSION {
            violations.push(ChangeIntentAccessibilityViolation::SchemaVersion {
                expected: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CHANGE_INTENT_A11Y_RECORD_KIND {
            violations.push(ChangeIntentAccessibilityViolation::RecordKind {
                expected: CHANGE_INTENT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ChangeIntentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_objects = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ChangeIntentAccessibilityViolation::DuplicateId {
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
                violations.push(ChangeIntentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its object's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ChangeIntentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: object_primary_dimension(row.object),
                    },
                );
            }

            // Each row must preserve every mandatory object label.
            if !row.preserves_mandatory_labels() {
                violations.push(ChangeIntentAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy object must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ChangeIntentFallbackModality::Structured)
            {
                violations.push(
                    ChangeIntentAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(ChangeIntentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-provider / diff-drifted / publish-target-unavailable /
            // lifecycle-degraded state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    ChangeIntentAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(ChangeIntentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    ChangeIntentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve provider / linked-change / handoff / resolution / blocker truth.
            if !row.preserves_truth_continuity() {
                violations.push(ChangeIntentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ChangeIntentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ChangeIntentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ChangeIntentAccessibilityStatus::Stranded {
                violations.push(ChangeIntentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen object is certified at least once.
        for object in M5ChangeIntentObject::ALL {
            if !seen_objects.contains(&object) {
                violations
                    .push(ChangeIntentAccessibilityViolation::MissingObjectCoverage { object });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ChangeIntentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ChangeIntentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ChangeIntentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ChangeIntentAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → finding-lifecycle-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ChangeIntentA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(ChangeIntentAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one stale-provider / diff-drifted /
        // publish-target-unavailable / lifecycle-degraded row in the packet, so the "cannot-prove never
        // shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(ChangeIntentAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the review detail, AI panel, finding row, scope
        // selector, publish sheet, pending-review tray, provider publish review, resolution memory ledger,
        // and support / export packet — so every consumer surface is exercised at least once.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ChangeIntentConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ChangeIntentAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ChangeIntentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("change-intent accessibility parity packet serializes"),
        ) {
            violations.push(ChangeIntentAccessibilityViolation::RawObjectMaterialInExport);
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
            .expect("change-intent accessibility parity packet serializes")
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
        out.push_str("# M5 Change-Intent Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Objects: {} certified across {} / {} frozen objects\n",
            self.summary.object_count,
            self.represented_objects().len(),
            M5ChangeIntentObject::ALL.len(),
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

/// Reads and validates the checked-in change-intent accessibility parity export.
pub fn current_m5_change_intent_accessibility_parity_export(
) -> Result<ChangeIntentAccessibilityPacket, ChangeIntentAccessibilityArtifactError> {
    let packet: ChangeIntentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-change-intent-accessibility-parity/support_export.json"
    )))
    .map_err(ChangeIntentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ChangeIntentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in change-intent accessibility parity export.
#[derive(Debug)]
pub enum ChangeIntentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ChangeIntentAccessibilityViolation>),
}

impl fmt::Display for ChangeIntentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "change-intent accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "change-intent accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ChangeIntentAccessibilityArtifactError {}

/// Validation failure for M05-1272 change-intent accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeIntentAccessibilityViolation {
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
        dimension: M5ChangeIntentClaimDimension,
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
        object: M5ChangeIntentObject,
    },
    MissingDimensionCoverage {
        dimension: M5ChangeIntentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ChangeIntentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ChangeIntentA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ChangeIntentConsumerSurface,
    },
    SummaryMismatch,
    RawObjectMaterialInExport,
}

impl ChangeIntentAccessibilityViolation {
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

impl fmt::Display for ChangeIntentAccessibilityViolation {
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

impl Error for ChangeIntentAccessibilityViolation {}

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
pub const CHANGE_INTENT_A11Y_PACKET_ID: &str = "m5-change-intent-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in change-intent accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_change_intent_accessibility_parity_packet() -> ChangeIntentAccessibilityPacket {
    ChangeIntentAccessibilityPacket::new(ChangeIntentAccessibilityPacketInput {
        packet_id: CHANGE_INTENT_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: CHANGE_INTENT_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:change-intent-accessibility-parity:{id}")]
}

fn all_required_labels() -> Vec<M5ChangeIntentRequiredLabel> {
    M5ChangeIntentRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ChangeIntentCopyExportParity {
    ChangeIntentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ChangeIntentClaimDimension,
    state: M5ChangeIntentConditionState,
) -> ChangeIntentClaimConditionEntry {
    ChangeIntentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the support / export packet and the review
/// detail surface — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ChangeIntentConsumerSurface]) -> Vec<M5ChangeIntentConsumerSurface> {
    let mut out = vec![
        M5ChangeIntentConsumerSurface::SupportExportPacket,
        M5ChangeIntentConsumerSurface::WorkItemDetail,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: ChangeIntentNarrowingDisclosureState,
) -> Vec<ChangeIntentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ChangeIntentRenderingNarrowingDisclosure {
            rendering_surface: M5ChangeIntentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ChangeIntentRenderingNarrowingDisclosure {
            rendering_surface: M5ChangeIntentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_publish_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ChangeIntentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ChangeIntentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ChangeIntentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ChangeIntentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ChangeIntentRenderingSurface> {
    vec![
        M5ChangeIntentRenderingSurface::DesktopFull,
        M5ChangeIntentRenderingSurface::CliHeadless,
        M5ChangeIntentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ChangeIntentFallbackModality> {
    vec![
        M5ChangeIntentFallbackModality::List,
        M5ChangeIntentFallbackModality::Textual,
        M5ChangeIntentFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5ChangeIntentFallbackModality> {
    vec![
        M5ChangeIntentFallbackModality::Structured,
        M5ChangeIntentFallbackModality::List,
        M5ChangeIntentFallbackModality::Textual,
        M5ChangeIntentFallbackModality::Cli,
    ]
}

const REACHABLE: ChangeIntentNonVisualReachState =
    ChangeIntentNonVisualReachState::ReachableAndLabeled;
const REDUCED: ChangeIntentNonVisualReachState =
    ChangeIntentNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<ChangeIntentAccessibilityRow> {
    vec![
        // Change-intent record (provider-committed) — the record keeps its provider ownership, linked
        // branch / worktree / review identity, and local-versus-provider commit state current, so it is a
        // fully provider-committed, publish-safe surface reachable on every surface with no narrowing (green).
        // Keyboard-only and screen-reader users can inspect, start work, hand off, reopen, and export it
        // without losing provider ownership or commit-state truth.
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:change-intent-record-provider-committed".to_owned(),
            object: M5ChangeIntentObject::ChangeIntentRecord,
            source_object_schema_ref: M5ChangeIntentObject::ChangeIntentRecord
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:change-intent-record:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:change-intent-record-provider-committed:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "provider_ownership_and_commit_state",
                "relation_source_class",
                "local_versus_provider_parity",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::ProviderCommitStateClarity,
                M5ChangeIntentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "provider_ownership_and_commit_state",
                "relation_source_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::ReadyForReviewHandoff,
                M5ChangeIntentConsumerSurface::StartWorkSheet,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.7 — change intent & work items".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("change-intent-record-provider-committed"),
        },
        // Blocked / escalate card (continuity bound) — structure-heavy (blocker class / missing dependency
        // or approval / suggested escalation path / attached local evidence); it keeps its blocker cause and
        // local handoff-packet continuity bound, so it is a self-sufficient, locally reviewable surface a user
        // can inspect, with full parity on every surface (green). Its structured blocker set binds to a flat
        // list / textual path.
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:blocked-escalate-card-continuity-bound".to_owned(),
            object: M5ChangeIntentObject::BlockedEscalateCard,
            source_object_schema_ref: M5ChangeIntentObject::BlockedEscalateCard
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:blocked-escalate-card:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:blocked-escalate-card-continuity-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "blocker_class_and_evidence",
                "provider_ownership_and_commit_state",
                "attached_local_evidence_ref",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::LocalReviewableSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::BlockerContinuityClarity,
                M5ChangeIntentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "blocker_class_and_evidence",
                "provider_ownership_and_commit_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::ReadyForReviewHandoff,
                M5ChangeIntentConsumerSurface::HelpDocs,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22.37 — blocked / escalate cards".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("blocked-escalate-card-continuity-bound"),
        },
        // Change-intent record (provider commit state local-only / reconcile-required) — the record's
        // provider commit state is local-only or reconcile-required, so it auto-narrows to a
        // provider-commit-state-unverified projection that keeps the last-known linked-change identity visible
        // without presenting a local draft or queued publish as a provider-committed update (yellow). Its
        // screen-reader traversal discloses a reduced linear walk.
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:change-intent-record-local-only".to_owned(),
            object: M5ChangeIntentObject::ChangeIntentRecord,
            source_object_schema_ref: M5ChangeIntentObject::ChangeIntentRecord
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:change-intent-record:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:change-intent-record-local-only:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "last_known_linked_change_identity",
                "reconcile_required_reason",
                "local_versus_provider_parity",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::ProviderCommitStateClarity,
                M5ChangeIntentConditionState::LocalOnlyOrReconcileRequired,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::ProviderCommitStateUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::ProviderCommitStateClarity,
                trigger: M5ChangeIntentDowngradeTrigger::LocalVersusProviderStateUnstated,
                narrowed_label:
                    "This change-intent record's provider commit state is local-only / reconcile-required — shown as a provider-commit-state-unverified projection that keeps the provider ownership, last-known linked branch / worktree / review identity, and local-versus-provider state explicit, never presenting a local draft or queued publish as a provider-committed update"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "last_known_linked_change_identity",
                "reconcile_required_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::ReadyForReviewHandoff,
                M5ChangeIntentConsumerSurface::ReviewDetail,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.7.1 — local-versus-provider commit state".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("change-intent-record-local-only"),
        },
        // Start-work sheet (side effect undisclosed) — a create-branch / worktree / review-draft /
        // provider-link side effect is not yet separately disclosed, so it auto-narrows to a
        // side-effect-disclosure-unverified projection that keeps each pending side effect named and its
        // commit state explicit, never silently creating a branch, worktree, review draft, or link (yellow).
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:start-work-sheet-side-effect-undisclosed".to_owned(),
            object: M5ChangeIntentObject::StartWorkSheet,
            source_object_schema_ref: M5ChangeIntentObject::StartWorkSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:start-work-sheet:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:start-work-sheet-side-effect-undisclosed:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "pending_side_effect_set",
                "side_effect_commit_state",
                "provider_ownership_and_commit_state",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::SideEffectDisclosureClarity,
                M5ChangeIntentConditionState::SideEffectUndisclosed,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::SideEffectDisclosureUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::SideEffectDisclosureClarity,
                trigger: M5ChangeIntentDowngradeTrigger::SilentSideEffectCreated,
                narrowed_label:
                    "This start-work sheet has an undisclosed side effect (create-branch / worktree / review-draft / provider-link) — shown as a side-effect-disclosure-unverified projection that keeps each pending side effect separately named and its commit state explicit, never silently creating a branch, worktree, review draft, or provider link as if already committed"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "pending_side_effect_set",
                "side_effect_commit_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::ResolveCloseSheet,
                M5ChangeIntentConsumerSurface::StartWorkSheet,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22.37 — start-work side-effect disclosure".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("start-work-sheet-side-effect-undisclosed"),
        },
        // Linked-change panel (relation stale or broken) — structure-heavy (linked branch / worktree /
        // hosted-review relation rows); the linked relation is stale or broken, so it auto-narrows to a
        // linked-relation-unverified projection that keeps the relation source (linked-by-provider /
        // linked-locally / suggested / stale-or-broken) explicit, never flattening the relation sources into
        // one badge or keeping a stale relation green (yellow). Its dense reflow narrows the high-zoom
        // legibility to a disclosed reduction.
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:linked-change-panel-relation-stale".to_owned(),
            object: M5ChangeIntentObject::LinkedChangePanel,
            source_object_schema_ref: M5ChangeIntentObject::LinkedChangePanel
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:linked-change-panel:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:linked-change-panel-relation-stale:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "linked_branch_worktree_review_state",
                "relation_source_class",
                "stale_or_broken_relation_reason",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::LinkedRelationSourceClarity,
                M5ChangeIntentConditionState::LinkedRelationStaleOrBroken,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::LinkedRelationUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::LinkedRelationSourceClarity,
                trigger: M5ChangeIntentDowngradeTrigger::RelationSourceUnstated,
                narrowed_label:
                    "This linked-change relation is stale or broken (linked branch / worktree / hosted review drifted from provider-authoritative state) — shown as a linked-relation-unverified projection that keeps the relation source (linked-by-provider / linked-locally / suggested-by-Aureline / stale-or-broken) explicit, never flattening the relation sources into one badge or keeping a stale relation green"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "linked_branch_worktree_review_state",
                "relation_source_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::StartWorkSheet,
                M5ChangeIntentConsumerSurface::BlockedEscalateCard,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §13.7 — linked-change relation source".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("linked-change-panel-relation-stale"),
        },
        // Ready-for-review handoff (publishability blocked) — the handoff packet is not publishable to the
        // provider (offline / missing provider write scope / policy-blocked / partially writable), so it
        // auto-narrows to a handoff-publishability-unverified projection that keeps the queued packet, changed
        // scope, and linked review target explicit, never letting a local handoff packet or queued publish
        // read as a provider-committed update (yellow).
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ready-for-review-handoff-publishability-blocked".to_owned(),
            object: M5ChangeIntentObject::ReadyForReviewHandoffSheet,
            source_object_schema_ref: M5ChangeIntentObject::ReadyForReviewHandoffSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:ready-for-review-handoff:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ready-for-review-handoff-publishability-blocked:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "local_versus_provider_parity",
                "publishability_blocker_reason",
                "reconcile_action",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::HandoffPublishabilityClarity,
                M5ChangeIntentConditionState::HandoffPublishabilityBlocked,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::HandoffPublishabilityUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::HandoffPublishabilityClarity,
                trigger: M5ChangeIntentDowngradeTrigger::LocalHandoffShownAsProviderCommitted,
                narrowed_label:
                    "This ready-for-review handoff packet is not publishable to the provider (offline / missing provider write scope / policy-blocked / partially writable) — shown as a handoff-publishability-unverified projection that keeps the queued packet, changed scope, and linked review target explicit, never letting a local handoff packet or queued publish read as a provider-committed update"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "local_versus_provider_parity",
                "publishability_blocker_reason",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::BlockedEscalateCard,
                M5ChangeIntentConsumerSurface::ReviewDetail,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §13.7 — ready-for-review publish-later handoff".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("ready-for-review-handoff-publishability-blocked"),
        },
        // Resolve / close sheet (resolution authority local-only) — the resolution is local-only and not
        // provider-accepted, so it auto-narrows to a resolution-authority-unverified projection that keeps the
        // requested terminal state, unresolved engineering blockers, and reopen / export path explicit, never
        // treating a local-only resolution as a provider-accepted terminal state (yellow).
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:resolve-close-sheet-authority-local-only".to_owned(),
            object: M5ChangeIntentObject::ResolveCloseSheet,
            source_object_schema_ref: M5ChangeIntentObject::ResolveCloseSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:resolve-close-sheet:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REDUCED,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:resolve-close-sheet-authority-local-only:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "resolution_authority_binding",
                "relation_source_class",
                "provider_ownership_and_commit_state",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::ResolutionAuthorityClarity,
                M5ChangeIntentConditionState::ResolutionAuthorityLocalOnly,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::ResolutionAuthorityUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::ResolutionAuthorityClarity,
                trigger: M5ChangeIntentDowngradeTrigger::AutoResolvedWithOpenBlocker,
                narrowed_label:
                    "This resolve / close sheet's resolution is local-only (not provider-accepted) — shown as a resolution-authority-unverified projection that keeps the requested terminal state, unresolved engineering blockers, and reopen / export path explicit, never treating a local-only resolution as a provider-accepted terminal state or auto-resolving tracked work while a blocker remains"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "resolution_authority_binding",
                "relation_source_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::LinkedChangePanel,
                M5ChangeIntentConsumerSurface::ReviewDetail,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22.37 — resolve / close authority".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("resolve-close-sheet-authority-local-only"),
        },
        // Blocked / escalate card (blocker unresolved) — structure-heavy (blocker class / missing
        // dependency or approval / attached local evidence); the blocker is unresolved and its local handoff
        // packet must not read as a provider escalation, so it auto-narrows to a blocker-continuity-unverified
        // projection that keeps the blocker class, missing dependency / approval, and attached local evidence
        // explicit, never dropping local notes or letting a local packet masquerade as a provider escalation
        // (yellow).
        ChangeIntentAccessibilityRow {
            record_kind: CHANGE_INTENT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CHANGE_INTENT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:blocked-escalate-card-blocker-unresolved".to_owned(),
            object: M5ChangeIntentObject::BlockedEscalateCard,
            source_object_schema_ref: M5ChangeIntentObject::BlockedEscalateCard
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "work-item:blocked-escalate-card:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ChangeIntentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:blocked-escalate-card-blocker-unresolved:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "blocker_class_and_evidence",
                "attached_local_evidence_ref",
                "last_known_linked_change_identity",
            ]),
            full_ready_claim: M5ChangeIntentA11yClaim::TrustedProviderCommittedSurface,
            claim_conditions: vec![condition(
                M5ChangeIntentClaimDimension::BlockerContinuityClarity,
                M5ChangeIntentConditionState::BlockerUnresolvedOrMasquerade,
            )],
            claim_narrow: Some(ChangeIntentClaimAutoNarrow {
                narrowed_to: M5ChangeIntentA11yClaim::BlockerContinuityUnverifiedProjection,
                binding_dimension: M5ChangeIntentClaimDimension::BlockerContinuityClarity,
                trigger: M5ChangeIntentDowngradeTrigger::BlockerStateUnstated,
                narrowed_label:
                    "This blocked / escalate card's blocker is unresolved and its local handoff packet must not read as a provider escalation — shown as a blocker-continuity-unverified projection that keeps the blocker class, missing dependency / approval, and attached local evidence explicit, never dropping local notes or handoff packets and never letting a local handoff packet masquerade as a provider-committed escalation"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "blocker_class_and_evidence",
                "attached_local_evidence_ref",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ChangeIntentConsumerSurface::HelpDocs,
                M5ChangeIntentConsumerSurface::LinkedChangePanel,
            ]),
            source_refs: vec![
                "UX Design System v1.37 §22.37 — blocker escalation continuity".to_owned(),
                CHANGE_INTENT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("blocked-escalate-card-blocker-unresolved"),
        },
    ]
}
