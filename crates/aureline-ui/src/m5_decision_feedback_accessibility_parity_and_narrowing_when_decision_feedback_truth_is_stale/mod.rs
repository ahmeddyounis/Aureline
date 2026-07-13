//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 badge-chip-pill / popover / dialog-sheet / banner-inline-notice / toast /
//! empty-state / loading-state / consequence-block decision and feedback primitives.
//!
//! This module is the M05-1138 accessibility-and-auto-narrowing capstone over the frozen M5
//! decision-feedback component matrix ([`crate::m5_decision_feedback_component_matrix`]). Where the freeze
//! matrix defines the reusable badge, popover, dialog, banner, toast, empty-state, loading-state, and
//! consequence-block primitives, and the 1133-1136 implementation lanes resolve their per-surface truth,
//! this lane certifies — per primitive family — that decision / feedback claims stay
//! **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe, CLI/export-safe, and
//! self-narrowing** rather than presenting a stale-severity badge, an unscoped notice, an unanchored
//! popover focus return, a toast-only durable truth, a full-screen-spinning loading state, or a partial
//! consequence recovery posture as still a trusted, ready-to-read decision surface:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same primitive identity, disposition / state, severity, scope,
//!   rationale, recovery path, focus-return anchor, and durable-object linkage the rich primitive shows —
//!   never a color-only badge, a hover-only popover, a toast-only cue, or a motion-only spinner that
//!   strands assistive-tech or headless-CLI users. Structure-heavy families (the dialog's action set, the
//!   popover's anchored content, the consequence block's named blast radius) additionally bind their
//!   structured layout to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each primitive's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same identity, disposition, severity,
//!   scope, rationale, recovery path, and durable-object linkage shown in-product so support, help, and
//!   release proof can reconstruct exactly what the user was actually shown without leaking a raw field
//!   value, secret, endpoint, or message payload.
//! - **Honest auto-narrowing.** When a badge's severity evidence is stale, a banner's scope cannot be
//!   confirmed, a popover's focus-return anchor is stale, a toast's durable-object linkage is missing, a
//!   loading state can only prove a partial capability, or a consequence block can only disclose a partial
//!   recovery / rollback posture, the primitive's claim auto-narrows from `trusted_decision_surface` /
//!   `reviewable_decision_surface` to a severity-unverified / scope-unverified / focus-return-unverified /
//!   durable-object-unverified / partial-capability-unverified / recovery-path-disclosed projection,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the canonical
//!   primitive identity / last-known state. The underlying severity / scope / recovery / durability truth
//!   is never dropped opaquely. A primitive with every dimension intact must NOT carry a spurious
//!   narrowing, and a stale-severity / unscoped / unanchored-focus-return / toast-only-durable /
//!   unconfirmed-partial-capability state can never keep a trusted, ready-to-read claim — a durable outcome
//!   is never represented as toast-only truth, and a partial capability never hides behind a full-screen
//!   spinner.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the help UI, the
//!   support UI, the review UI, the settings UI, the updates UI, the CLI export, the support export, and
//!   the product UI so product, help, and release publication stay aligned on downgrade behavior rather
//!   than drifting in copy — a trusted-looking primitive can never outrun the severity / scope / recovery /
//!   durability evidence it is being viewed away from.
//!
//! Each [`DecisionFeedbackAccessibilityRow`] keys on one
//! [`crate::m5_decision_feedback_component_matrix::M5DecisionFeedbackFamily`] and reuses that frozen family
//! vocabulary plus the frozen [`M5DecisionFeedbackRequiredLabel`], [`M5DecisionFeedbackDowngradeTrigger`],
//! and shared [`M5DecisionFeedbackConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw field values, message payloads, credentials, secrets, and endpoint
//! refs never cross this boundary; the packet carries only typed class tokens, opaque primitive refs,
//! booleans, and controlled labels so support, release, and diagnostics exports can reconstruct exactly
//! what an accessible fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_decision_feedback_component_matrix::{
    M5DecisionFeedbackConsumerSurface, M5DecisionFeedbackDowngradeTrigger,
    M5DecisionFeedbackFamily, M5DecisionFeedbackRequiredLabel,
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1138 decision-feedback component accessibility parity packet.
pub const DECISION_FEEDBACK_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DecisionFeedbackAccessibilityPacket`].
pub const DECISION_FEEDBACK_A11Y_RECORD_KIND: &str =
    "m5_decision_feedback_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`DecisionFeedbackAccessibilityRow`].
pub const DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND: &str =
    "m5_decision_feedback_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const DECISION_FEEDBACK_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-decision-feedback-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const DECISION_FEEDBACK_A11Y_DOC_REF: &str =
    "docs/components/m5_decision_feedback_component_accessibility_parity.md";

/// Repo-relative path of the frozen decision-feedback component matrix this lane certifies.
pub const DECISION_FEEDBACK_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const DECISION_FEEDBACK_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-decision-feedback-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const DECISION_FEEDBACK_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-decision-feedback-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DECISION_FEEDBACK_A11Y_CSV_REF: &str =
    "artifacts/release/m5-decision-feedback-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DECISION_FEEDBACK_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-decision-feedback-component-accessibility-parity.md";

/// The reusable primitive families that render a dense, structured surface (the dialog's action set, the
/// popover's anchored content, the consequence block's named blast radius) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5DecisionFeedbackFamily) -> bool {
    matches!(
        family,
        M5DecisionFeedbackFamily::DialogSheet
            | M5DecisionFeedbackFamily::Popover
            | M5DecisionFeedbackFamily::ConsequenceBlock
    )
}

/// The decision-feedback-truth dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5DecisionFeedbackFamily,
) -> M5DecisionFeedbackClaimDimension {
    match family {
        M5DecisionFeedbackFamily::BadgeChipPill => {
            M5DecisionFeedbackClaimDimension::SeverityMeaningClarity
        }
        M5DecisionFeedbackFamily::Popover => {
            M5DecisionFeedbackClaimDimension::FocusReturnAnchorClarity
        }
        M5DecisionFeedbackFamily::DialogSheet => {
            M5DecisionFeedbackClaimDimension::RationaleScopeActionClarity
        }
        M5DecisionFeedbackFamily::BannerInlineNotice => {
            M5DecisionFeedbackClaimDimension::NoticeScopeClarity
        }
        M5DecisionFeedbackFamily::Toast => {
            M5DecisionFeedbackClaimDimension::DurableObjectLinkageClarity
        }
        M5DecisionFeedbackFamily::EmptyState => {
            M5DecisionFeedbackClaimDimension::PurposeNextActionClarity
        }
        M5DecisionFeedbackFamily::LoadingState => {
            M5DecisionFeedbackClaimDimension::PartialCapabilityFidelityClarity
        }
        M5DecisionFeedbackFamily::ConsequenceBlock => {
            M5DecisionFeedbackClaimDimension::BlastRadiusRecoveryClarity
        }
    }
}

/// A rendered fallback modality for a decision-feedback primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackFallbackModality {
    /// A rich, structured (action set / anchored content / named blast radius) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5DecisionFeedbackFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same primitive may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs
/// export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackRenderingSurface {
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

impl M5DecisionFeedbackRenderingSurface {
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

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a primitive's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl DecisionFeedbackNonVisualReachState {
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

/// Whether an export-safe summary preserves the primitive meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackExportSummaryState {
    /// The primitive meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl DecisionFeedbackExportSummaryState {
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
pub enum DecisionFeedbackNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl DecisionFeedbackNarrowingDisclosureState {
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

/// The decision-feedback claim ceiling a family asserts: how strong a trusted / ready-to-read posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a severity / scope / focus-return /
/// durability / capability / recovery dimension weakens so a stale-severity badge, an unscoped notice, an
/// unanchored popover, a toast-only durable truth, a full-screen-spinning loading state, or a partial
/// recovery disclosure can never keep an old `TrustedDecisionSurface` or `ReviewableDecisionSurface` label
/// — a durable outcome is never represented as toast-only truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackA11yClaim {
    /// Trusted decision surface: a fully current, severity-clear, scoped, focus-anchored, durable-linked
    /// primitive — the strongest claim, a decision / feedback surface Aureline can present as exactly
    /// trusted and ready to read right now.
    TrustedDecisionSurface,
    /// Reviewable decision surface: a self-sufficient, reviewable read-only primitive (a badge / empty
    /// state a user can inspect) that is not itself an authoritative, action-driving surface.
    ReviewableDecisionSurface,
    /// Severity-unverified projection: the badge / notice severity evidence is stale; the primitive stays a
    /// severity-unverified projection with its last-known meaning preserved, never a fresh, color-only
    /// severity shown as authoritative.
    SeverityUnverifiedProjection,
    /// Scope-unverified projection: a banner / notice scope cannot be confirmed; the primitive stays a
    /// scope-unverified projection that keeps the last-known scope explicit, never an unscoped notice shown
    /// as global truth.
    ScopeUnverifiedProjection,
    /// Focus-return-unverified projection: a popover's safe focus-return anchor cannot be confirmed; the
    /// primitive stays a focus-return-unverified projection that keeps the anchor and content inspectable,
    /// never a popover that strands focus or carries the only critical instruction.
    FocusReturnUnverifiedProjection,
    /// Durable-object-unverified projection: a toast's durable-object linkage is missing; the primitive
    /// stays a durable-object-unverified projection that discloses the missing durable back-link, never a
    /// durable outcome shown as toast-only truth.
    DurableObjectUnverifiedProjection,
    /// Partial-capability-unverified projection: a loading state can only prove a partial capability; the
    /// primitive stays a partial-capability-unverified projection that preserves the useful partial data,
    /// never a full-screen spinner that blanks a useful pane.
    PartialCapabilityUnverifiedProjection,
    /// Recovery-path-disclosed projection: a consequence block can only disclose a partial / redacted
    /// recovery / rollback posture; the primitive stays a recovery-path-disclosed projection that discloses
    /// the partial recovery posture, never a fully-reversible, no-consequence block.
    RecoveryPathDisclosedProjection,
}

impl M5DecisionFeedbackA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedDecisionSurface,
        Self::ReviewableDecisionSurface,
        Self::SeverityUnverifiedProjection,
        Self::ScopeUnverifiedProjection,
        Self::FocusReturnUnverifiedProjection,
        Self::DurableObjectUnverifiedProjection,
        Self::PartialCapabilityUnverifiedProjection,
        Self::RecoveryPathDisclosedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedDecisionSurface => 7,
            Self::ReviewableDecisionSurface => 6,
            Self::SeverityUnverifiedProjection => 5,
            Self::ScopeUnverifiedProjection => 4,
            Self::FocusReturnUnverifiedProjection => 3,
            Self::DurableObjectUnverifiedProjection => 2,
            Self::PartialCapabilityUnverifiedProjection => 1,
            Self::RecoveryPathDisclosedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, ready-to-read decision surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedDecisionSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedDecisionSurface | Self::ReviewableDecisionSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedDecisionSurface => "trusted_decision_surface",
            Self::ReviewableDecisionSurface => "reviewable_decision_surface",
            Self::SeverityUnverifiedProjection => "severity_unverified_projection",
            Self::ScopeUnverifiedProjection => "scope_unverified_projection",
            Self::FocusReturnUnverifiedProjection => "focus_return_unverified_projection",
            Self::DurableObjectUnverifiedProjection => "durable_object_unverified_projection",
            Self::PartialCapabilityUnverifiedProjection => {
                "partial_capability_unverified_projection"
            }
            Self::RecoveryPathDisclosedProjection => "recovery_path_disclosed_projection",
        }
    }
}

/// The severity / scope / focus-return / durability / capability / recovery dimension whose state governs
/// how far a primitive may claim to be a fully trusted, ready-to-read decision surface. The dimensions map
/// 1:1 to the eight frozen primitive families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackClaimDimension {
    /// Severity-meaning clarity: does the badge / chip / pill expand into a plain-language severity meaning
    /// instead of a color-only shorthand (badge-chip-pill)?
    SeverityMeaningClarity,
    /// Focus-return-anchor clarity: does the popover keep a safe focus-return anchor and stay a lightweight
    /// secondary control (popover)?
    FocusReturnAnchorClarity,
    /// Rationale / scope / action clarity: does the dialog / sheet name rationale, scope, and explicit
    /// actions rather than generic Yes/No (dialog-sheet)?
    RationaleScopeActionClarity,
    /// Notice-scope clarity: does the banner / inline notice stay scoped and actionable rather than an
    /// unscoped, color-only alert (banner-inline-notice)?
    NoticeScopeClarity,
    /// Durable-object-linkage clarity: does the toast keep a durable back-link so it acknowledges without
    /// becoming the only durable truth (toast)?
    DurableObjectLinkageClarity,
    /// Purpose / next-action clarity: does the empty state explain purpose, current emptiness, and next
    /// action (empty-state)?
    PurposeNextActionClarity,
    /// Partial-capability fidelity clarity: does the loading state preserve useful partial data rather than
    /// blanking a pane or spinning full-screen (loading-state)?
    PartialCapabilityFidelityClarity,
    /// Blast-radius / recovery clarity: does the consequence block name blast radius and rollback / help
    /// posture (consequence-block)?
    BlastRadiusRecoveryClarity,
}

impl M5DecisionFeedbackClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SeverityMeaningClarity,
        Self::FocusReturnAnchorClarity,
        Self::RationaleScopeActionClarity,
        Self::NoticeScopeClarity,
        Self::DurableObjectLinkageClarity,
        Self::PurposeNextActionClarity,
        Self::PartialCapabilityFidelityClarity,
        Self::BlastRadiusRecoveryClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeverityMeaningClarity => "severity_meaning_clarity",
            Self::FocusReturnAnchorClarity => "focus_return_anchor_clarity",
            Self::RationaleScopeActionClarity => "rationale_scope_action_clarity",
            Self::NoticeScopeClarity => "notice_scope_clarity",
            Self::DurableObjectLinkageClarity => "durable_object_linkage_clarity",
            Self::PurposeNextActionClarity => "purpose_next_action_clarity",
            Self::PartialCapabilityFidelityClarity => "partial_capability_fidelity_clarity",
            Self::BlastRadiusRecoveryClarity => "blast_radius_recovery_clarity",
        }
    }
}

/// The observed condition of one decision-feedback-truth dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the primitive's claim. The stale / missing /
/// unconfirmed states the lane must auto-narrow on as *weakened evidence* — a stale severity, an
/// unconfirmed scope, an unanchored focus return, a missing durable-object linkage, and an unconfirmed
/// partial capability — are the states that [`Self::cannot_be_shown_trusted`] flags. A partial recovery
/// disclosure is an honest disclosed-absence operation (a partial / redacted rollback posture shown
/// honestly with an inspectable recovery note), not a truth overstatement, so it is deliberately excluded
/// there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackConditionState {
    /// Fully current, severity-clear, scoped, focus-anchored, durable-linked, capability-honest — imposes
    /// no ceiling.
    FullyQualified,
    /// The badge / notice severity evidence is stale — claim drops to a severity-unverified projection.
    SeverityEvidenceStale,
    /// The banner / notice scope cannot be confirmed — claim drops to a scope-unverified projection.
    ScopeEvidenceStale,
    /// The popover's safe focus-return anchor cannot be confirmed — claim drops to a
    /// focus-return-unverified projection.
    FocusReturnAnchorStale,
    /// The toast's durable-object linkage is missing — claim drops to a durable-object-unverified
    /// projection.
    DurableObjectLinkageStale,
    /// The loading state can only prove a partial capability — claim drops to a
    /// partial-capability-unverified projection.
    PartialCapabilityUnconfirmed,
    /// The consequence block can only disclose a partial / redacted recovery / rollback posture — claim
    /// drops to a recovery-path-disclosed projection.
    RecoveryPathDisclosedPartial,
}

impl M5DecisionFeedbackConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::SeverityEvidenceStale,
        Self::ScopeEvidenceStale,
        Self::FocusReturnAnchorStale,
        Self::DurableObjectLinkageStale,
        Self::PartialCapabilityUnconfirmed,
        Self::RecoveryPathDisclosedPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// ready-to-read decision surface and must never be shown as such. A partial recovery disclosure is an
    /// honest disclosed-absence operation (a partial / redacted rollback posture shown honestly with an
    /// inspectable recovery note), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::SeverityEvidenceStale
                | Self::ScopeEvidenceStale
                | Self::FocusReturnAnchorStale
                | Self::DurableObjectLinkageStale
                | Self::PartialCapabilityUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5DecisionFeedbackA11yClaim {
        match self {
            Self::FullyQualified => M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            Self::SeverityEvidenceStale => {
                M5DecisionFeedbackA11yClaim::SeverityUnverifiedProjection
            }
            Self::ScopeEvidenceStale => M5DecisionFeedbackA11yClaim::ScopeUnverifiedProjection,
            Self::FocusReturnAnchorStale => {
                M5DecisionFeedbackA11yClaim::FocusReturnUnverifiedProjection
            }
            Self::DurableObjectLinkageStale => {
                M5DecisionFeedbackA11yClaim::DurableObjectUnverifiedProjection
            }
            Self::PartialCapabilityUnconfirmed => {
                M5DecisionFeedbackA11yClaim::PartialCapabilityUnverifiedProjection
            }
            Self::RecoveryPathDisclosedPartial => {
                M5DecisionFeedbackA11yClaim::RecoveryPathDisclosedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5DecisionFeedbackDowngradeTrigger::ProofStale,
            Self::SeverityEvidenceStale => M5DecisionFeedbackDowngradeTrigger::StateTaxonomyDrifted,
            Self::ScopeEvidenceStale => M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
            Self::FocusReturnAnchorStale => {
                M5DecisionFeedbackDowngradeTrigger::PopoverCarriedOnlyCriticalInstruction
            }
            Self::DurableObjectLinkageStale => {
                M5DecisionFeedbackDowngradeTrigger::DurableWorkShownAsToastOnly
            }
            Self::PartialCapabilityUnconfirmed => {
                M5DecisionFeedbackDowngradeTrigger::FullScreenSpinnerWhenPartialCapable
            }
            Self::RecoveryPathDisclosedPartial => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::SeverityEvidenceStale => "severity_evidence_stale",
            Self::ScopeEvidenceStale => "scope_evidence_stale",
            Self::FocusReturnAnchorStale => "focus_return_anchor_stale",
            Self::DurableObjectLinkageStale => "durable_object_linkage_stale",
            Self::PartialCapabilityUnconfirmed => "partial_capability_unconfirmed",
            Self::RecoveryPathDisclosedPartial => "recovery_path_disclosed_partial",
        }
    }
}

/// One decision-feedback-truth dimension's observed condition on a primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5DecisionFeedbackClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5DecisionFeedbackConditionState,
}

/// An honest claim auto-narrow block. When a decision-feedback-truth dimension weakens, the primitive's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical primitive identity / last-known state rather than silently dropping it — the underlying
/// severity / scope / recovery / durability truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackClaimAutoNarrow {
    /// The claim the primitive is narrowed to.
    pub narrowed_to: M5DecisionFeedbackA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5DecisionFeedbackClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5DecisionFeedbackDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical primitive identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying severity / scope / recovery / durability truth is preserved (never dropped) across
    /// the narrowing; must hold so severity-unverified, scope-unverified, focus-return-unverified,
    /// durable-object-unverified, partial-capability-unverified, and recovery-path-disclosed states never
    /// fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl DecisionFeedbackClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and severity / scope /
    /// recovery / durability truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a primitive's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl DecisionFeedbackCopyExportParity {
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
pub struct DecisionFeedbackRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5DecisionFeedbackRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: DecisionFeedbackNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a decision-feedback accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl DecisionFeedbackAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one decision-feedback primitive family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackAccessibilityRow {
    /// Record kind; must equal [`DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DECISION_FEEDBACK_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen primitive family this row certifies.
    pub component_family: M5DecisionFeedbackFamily,
    /// Ref to the frozen per-component schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the primitive this row represents; stays visible on every surface, so this is never
    /// empty.
    pub component_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5DecisionFeedbackFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, disposition, severity, scope,
    /// rationale, recovery path, focus-return anchor, and durable-object linkage as the rich primitive;
    /// must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: DecisionFeedbackNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: DecisionFeedbackNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: DecisionFeedbackNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: DecisionFeedbackNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: DecisionFeedbackNonVisualReachState,
    /// Whether the export-safe summary preserves primitive meaning.
    pub export_summary: DecisionFeedbackExportSummaryState,
    /// Ref to the export-safe summary object for this primitive.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: DecisionFeedbackCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5DecisionFeedbackA11yClaim,
    /// The observed condition of each modeled decision-feedback-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<DecisionFeedbackClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<DecisionFeedbackClaimAutoNarrow>,
    /// Whether the underlying severity / scope / recovery / durability truth is preserved on this primitive
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this primitive is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5DecisionFeedbackRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<DecisionFeedbackRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5DecisionFeedbackRequiredLabel>,
    /// Semantic consumer surfaces this primitive is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5DecisionFeedbackConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl DecisionFeedbackAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5DecisionFeedbackClaimDimension,
    ) -> M5DecisionFeedbackConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5DecisionFeedbackConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5DecisionFeedbackA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_condition(&self) -> Option<&DecisionFeedbackClaimConditionEntry> {
        let mut binding: Option<(&DecisionFeedbackClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5DecisionFeedbackClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this primitive effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5DecisionFeedbackA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-severity badge, an unscoped notice, an unanchored popover, a
    /// toast-only durable truth, a full-screen-spinning loading state, or a partial recovery disclosure can
    /// no longer keep an old `TrustedDecisionSurface` / `ReviewableDecisionSurface` label. The effective
    /// claim never exceeds the permitted ceiling; when a dimension narrows below the full claim, an honest
    /// narrow block is present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity and truth. When nothing narrows,
    /// no spurious narrow block is present.
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

    /// AC / trusted honesty: a stale-severity / unscoped / unanchored-focus-return / toast-only-durable /
    /// unconfirmed-partial-capability state never keeps a trusted claim — a durable outcome is never
    /// represented as toast-only truth. When such a state is modeled, the effective claim must not assert
    /// `TrustedDecisionSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a structure-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.component_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the primitive meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying severity / scope / recovery /
    /// durability truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the primitive carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
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

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5DecisionFeedbackRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> DecisionFeedbackAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return DecisionFeedbackAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            DecisionFeedbackAccessibilityStatus::NarrowedDisclosed
        } else {
            DecisionFeedbackAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND
            && self.schema_version == DECISION_FEEDBACK_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.component_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1138 decision-feedback accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
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

/// Constructor input for [`DecisionFeedbackAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFeedbackAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<DecisionFeedbackAccessibilityRow>,
}

/// Checked-in M05-1138 decision-feedback accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<DecisionFeedbackAccessibilityRow>,
    pub summary: DecisionFeedbackAccessibilitySummary,
}

impl DecisionFeedbackAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: DecisionFeedbackAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            record_kind: DECISION_FEEDBACK_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: DecisionFeedbackAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
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

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5DecisionFeedbackFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5DecisionFeedbackClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5DecisionFeedbackConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5DecisionFeedbackA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5DecisionFeedbackConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DecisionFeedbackAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5DecisionFeedbackConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&DecisionFeedbackAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                DecisionFeedbackAccessibilityStatus::Parity => green += 1,
                DecisionFeedbackAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                DecisionFeedbackAccessibilityStatus::Stranded => red += 1,
            }
        }

        DecisionFeedbackAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(DecisionFeedbackAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DecisionFeedbackAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DECISION_FEEDBACK_A11Y_SCHEMA_VERSION {
            violations.push(DecisionFeedbackAccessibilityViolation::SchemaVersion {
                expected: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DECISION_FEEDBACK_A11Y_RECORD_KIND {
            violations.push(DecisionFeedbackAccessibilityViolation::RecordKind {
                expected: DECISION_FEEDBACK_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DecisionFeedbackAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DecisionFeedbackAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(DecisionFeedbackAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory primitive label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5DecisionFeedbackFallbackModality::Structured)
            {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(DecisionFeedbackAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-severity / unscoped / unanchored-focus-return /
            // toast-only-durable / unconfirmed-partial-capability state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve severity / scope / recovery / durability truth.
            if !row.preserves_truth_continuity() {
                violations.push(DecisionFeedbackAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == DecisionFeedbackAccessibilityStatus::Stranded {
                violations.push(DecisionFeedbackAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5DecisionFeedbackFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(DecisionFeedbackAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5DecisionFeedbackClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5DecisionFeedbackConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → recovery-path-disclosed) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5DecisionFeedbackA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one stale-severity / unscoped /
        // unanchored-focus-return / toast-only-durable / unconfirmed-partial-capability row in the packet,
        // so the "cannot-prove never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(DecisionFeedbackAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, help, support, review, settings,
        // updates, CLI-export, support-export, and product surfaces — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5DecisionFeedbackConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    DecisionFeedbackAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DecisionFeedbackAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("decision-feedback accessibility parity packet serializes"),
        ) {
            violations.push(DecisionFeedbackAccessibilityViolation::RawComponentMaterialInExport);
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
            .expect("decision-feedback accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
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
        out.push_str("# M5 Decision/Feedback-Primitive Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5DecisionFeedbackFamily::ALL.len(),
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
                row.component_family.as_str(),
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

/// Reads and validates the checked-in decision-feedback accessibility parity export.
pub fn current_m5_decision_feedback_component_a11y_export(
) -> Result<DecisionFeedbackAccessibilityPacket, DecisionFeedbackAccessibilityArtifactError> {
    let packet: DecisionFeedbackAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-decision-feedback-component-accessibility-parity/support_export.json"
    )))
    .map_err(DecisionFeedbackAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DecisionFeedbackAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in decision-feedback accessibility parity export.
#[derive(Debug)]
pub enum DecisionFeedbackAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DecisionFeedbackAccessibilityViolation>),
}

impl fmt::Display for DecisionFeedbackAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "decision-feedback accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "decision-feedback accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for DecisionFeedbackAccessibilityArtifactError {}

/// Validation failure for M05-1138 decision-feedback accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionFeedbackAccessibilityViolation {
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
        dimension: M5DecisionFeedbackClaimDimension,
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
    MissingFamilyCoverage {
        family: M5DecisionFeedbackFamily,
    },
    MissingDimensionCoverage {
        dimension: M5DecisionFeedbackClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5DecisionFeedbackConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5DecisionFeedbackA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5DecisionFeedbackConsumerSurface,
    },
    SummaryMismatch,
    RawComponentMaterialInExport,
}

impl DecisionFeedbackAccessibilityViolation {
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
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawComponentMaterialInExport => "raw_component_material_in_export",
        }
    }
}

impl fmt::Display for DecisionFeedbackAccessibilityViolation {
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
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory primitive label")
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
                    "row {id} shows a stale-severity / unscoped / unanchored-focus-return / toast-only-durable / unconfirmed-partial-capability state as a trusted decision surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / CLI users from the canonical truth"
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
                    "row {id} does not preserve severity / scope / recovery / durability truth across narrowing"
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
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "primitive family {family:?} is not certified in the packet"
                )
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
                    "no stale-severity / unscoped / unanchored-focus-return / toast-only-durable / unconfirmed-partial-capability row is present to prove the trusted-honesty guarantee"
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
            Self::RawComponentMaterialInExport => {
                write!(f, "export contains raw component material")
            }
        }
    }
}

impl Error for DecisionFeedbackAccessibilityViolation {}

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
pub const DECISION_FEEDBACK_A11Y_PACKET_ID: &str =
    "m5-decision-feedback-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in decision-feedback accessibility parity packet. This is the one source
/// of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_decision_feedback_component_a11y_packet() -> DecisionFeedbackAccessibilityPacket {
    DecisionFeedbackAccessibilityPacket::new(DecisionFeedbackAccessibilityPacketInput {
        packet_id: DECISION_FEEDBACK_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: DECISION_FEEDBACK_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:decision-feedback-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5DecisionFeedbackRequiredLabel> {
    M5DecisionFeedbackRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> DecisionFeedbackCopyExportParity {
    DecisionFeedbackCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5DecisionFeedbackClaimDimension,
    state: M5DecisionFeedbackConditionState,
) -> DecisionFeedbackClaimConditionEntry {
    DecisionFeedbackClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5DecisionFeedbackConsumerSurface],
) -> Vec<M5DecisionFeedbackConsumerSurface> {
    let mut out = vec![
        M5DecisionFeedbackConsumerSurface::SupportExport,
        M5DecisionFeedbackConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: DecisionFeedbackNarrowingDisclosureState,
) -> Vec<DecisionFeedbackRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        DecisionFeedbackRenderingNarrowingDisclosure {
            rendering_surface: M5DecisionFeedbackRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        DecisionFeedbackRenderingNarrowingDisclosure {
            rendering_surface: M5DecisionFeedbackRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_hover_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<DecisionFeedbackRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        DecisionFeedbackNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<DecisionFeedbackRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        DecisionFeedbackNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5DecisionFeedbackRenderingSurface> {
    vec![
        M5DecisionFeedbackRenderingSurface::DesktopFull,
        M5DecisionFeedbackRenderingSurface::CliHeadless,
        M5DecisionFeedbackRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5DecisionFeedbackFallbackModality> {
    vec![
        M5DecisionFeedbackFallbackModality::List,
        M5DecisionFeedbackFallbackModality::Textual,
        M5DecisionFeedbackFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5DecisionFeedbackFallbackModality> {
    vec![
        M5DecisionFeedbackFallbackModality::Structured,
        M5DecisionFeedbackFallbackModality::List,
        M5DecisionFeedbackFallbackModality::Textual,
        M5DecisionFeedbackFallbackModality::Cli,
    ]
}

const REACHABLE: DecisionFeedbackNonVisualReachState =
    DecisionFeedbackNonVisualReachState::ReachableAndLabeled;
const REDUCED: DecisionFeedbackNonVisualReachState =
    DecisionFeedbackNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<DecisionFeedbackAccessibilityRow> {
    vec![
        // Dialog / sheet (rationale, scope, and actions fully stated) — the dialog names its rationale,
        // scope, and explicit actions rather than a generic Yes/No, so it is a trusted decision surface
        // reachable on every surface with no narrowing (green). Structure-heavy: its action set binds to a
        // flat list / textual path.
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:dialog-sheet-rationale-scope-actions-stated".to_owned(),
            component_family: M5DecisionFeedbackFamily::DialogSheet,
            source_family_schema_ref: M5DecisionFeedbackFamily::DialogSheet
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "repair:dialog-sheet:0001".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:dialog-sheet-rationale-scope-actions-stated:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "rationale",
                "named_scope",
                "explicit_actions",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::RationaleScopeActionClarity,
                M5DecisionFeedbackConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "primitive_identity",
                "rationale",
                "named_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::ReviewUi,
                M5DecisionFeedbackConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UX Style Guide §16.9 — Dialog / sheet".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("dialog-sheet-rationale-scope-actions-stated"),
        },
        // Empty state (purpose and next action fully stated) — the empty state explains purpose, current
        // emptiness, and next action, so it is a self-sufficient reviewable decision surface a user can
        // inspect, but its narrower non-visual traversal discloses a reduced linear screen-reader walk
        // (yellow).
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:empty-state-purpose-next-action-stated".to_owned(),
            component_family: M5DecisionFeedbackFamily::EmptyState,
            source_family_schema_ref: M5DecisionFeedbackFamily::EmptyState
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "review:empty-state:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:empty-state-purpose-next-action-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "purpose",
                "current_emptiness",
                "best_next_action",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::ReviewableDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::PurposeNextActionClarity,
                M5DecisionFeedbackConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "purpose",
                "best_next_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::ReviewUi,
                M5DecisionFeedbackConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.10 — Empty state".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("empty-state-purpose-next-action-stated"),
        },
        // Badge / chip / pill (severity evidence stale) — the badge's severity evidence is stale, so it
        // auto-narrows to a severity-unverified projection that keeps the last-known plain-language severity
        // meaning visible without relying on color alone, never a fresh, authoritative severity (yellow).
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:badge-chip-pill-severity-evidence-stale".to_owned(),
            component_family: M5DecisionFeedbackFamily::BadgeChipPill,
            source_family_schema_ref: M5DecisionFeedbackFamily::BadgeChipPill
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "settings:badge-chip-pill:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:badge-chip-pill-severity-evidence-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "severity_meaning",
                "disposition",
                "last_known_severity",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::SeverityMeaningClarity,
                M5DecisionFeedbackConditionState::SeverityEvidenceStale,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::SeverityUnverifiedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::SeverityMeaningClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::StateTaxonomyDrifted,
                narrowed_label:
                    "This badge's severity evidence is stale or unresolved — shown as a severity-unverified projection that keeps the last-known plain-language severity meaning visible without relying on color alone, never presenting a stale badge as a fresh, authoritative severity"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "severity_meaning",
                "disposition",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::SettingsUi,
                M5DecisionFeedbackConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.7 — Badge / chip / pill".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("badge-chip-pill-severity-evidence-stale"),
        },
        // Banner / inline notice (scope evidence stale) — the notice's scope cannot be confirmed, so it
        // auto-narrows to a scope-unverified projection that keeps the last-known scope and what-still-works
        // explicit, never an unscoped notice shown as global truth (yellow). Its dense reflow narrows the
        // high-zoom legibility to a disclosed reduction.
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:banner-inline-notice-scope-evidence-stale".to_owned(),
            component_family: M5DecisionFeedbackFamily::BannerInlineNotice,
            source_family_schema_ref: M5DecisionFeedbackFamily::BannerInlineNotice
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "updates:banner-inline-notice:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:banner-inline-notice-scope-evidence-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "named_scope",
                "what_still_works",
                "last_known_scope",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::NoticeScopeClarity,
                M5DecisionFeedbackConditionState::ScopeEvidenceStale,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::ScopeUnverifiedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::NoticeScopeClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
                narrowed_label:
                    "This banner / inline notice cannot confirm its scope — shown as a scope-unverified projection that keeps the last-known scope and what-still-works explicit and actionable, never presenting an unscoped notice as global truth"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "named_scope",
                "what_still_works",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::UpdatesUi,
                M5DecisionFeedbackConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.8 — Banner / inline notice".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("banner-inline-notice-scope-evidence-stale"),
        },
        // Popover (focus-return anchor stale) — structure-heavy (an anchored secondary surface); its safe
        // focus-return anchor cannot be confirmed, so it auto-narrows to a focus-return-unverified
        // projection that keeps the anchor and content inspectable and never carries the only critical
        // instruction, never stranding focus (yellow).
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:popover-focus-return-anchor-stale".to_owned(),
            component_family: M5DecisionFeedbackFamily::Popover,
            source_family_schema_ref: M5DecisionFeedbackFamily::Popover
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "help:popover:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:popover-focus-return-anchor-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "anchor_target",
                "focus_return_target",
                "secondary_content",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::FocusReturnAnchorClarity,
                M5DecisionFeedbackConditionState::FocusReturnAnchorStale,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::FocusReturnUnverifiedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::FocusReturnAnchorClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::PopoverCarriedOnlyCriticalInstruction,
                narrowed_label:
                    "This popover cannot confirm its safe focus-return anchor — shown as a focus-return-unverified projection that keeps the anchor target and secondary content inspectable and never carries the only critical instruction, never stranding keyboard focus after dismissal"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "anchor_target",
                "focus_return_target",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::HelpUi,
                M5DecisionFeedbackConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.7 — Popover".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("popover-focus-return-anchor-stale"),
        },
        // Toast (durable-object linkage missing) — the toast's durable-object back-link is missing, so it
        // auto-narrows to a durable-object-unverified projection that discloses the missing durable
        // back-link alongside the acknowledgement, never representing a durable outcome as toast-only truth
        // (yellow).
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:toast-durable-object-linkage-stale".to_owned(),
            component_family: M5DecisionFeedbackFamily::Toast,
            source_family_schema_ref: M5DecisionFeedbackFamily::Toast
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "settings:toast:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:toast-durable-object-linkage-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "acknowledgement",
                "durable_object_backlink",
                "last_known_durable_object",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::DurableObjectLinkageClarity,
                M5DecisionFeedbackConditionState::DurableObjectLinkageStale,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::DurableObjectUnverifiedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::DurableObjectLinkageClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::DurableWorkShownAsToastOnly,
                narrowed_label:
                    "This toast cannot confirm its durable-object back-link — shown as a durable-object-unverified projection that discloses the missing durable back-link alongside the acknowledgement, never representing a durable or reviewable outcome as toast-only truth"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "acknowledgement",
                "durable_object_backlink",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::SettingsUi,
                M5DecisionFeedbackConsumerSurface::SupportUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.8 — Toast".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("toast-durable-object-linkage-stale"),
        },
        // Loading state (partial capability unconfirmed) — the loading state can only prove a partial
        // capability, so it auto-narrows to a partial-capability-unverified projection that preserves the
        // useful partial data and skeleton structure rather than a full-screen spinner or a blanked pane
        // (yellow). Its animated affordance narrows the reduced-motion path to a disclosed reduction.
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:loading-state-partial-capability-unconfirmed".to_owned(),
            component_family: M5DecisionFeedbackFamily::LoadingState,
            source_family_schema_ref: M5DecisionFeedbackFamily::LoadingState
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "shell:loading-state:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:loading-state-partial-capability-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "loading_treatment",
                "preserved_partial_data",
                "last_known_capability",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::PartialCapabilityFidelityClarity,
                M5DecisionFeedbackConditionState::PartialCapabilityUnconfirmed,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::PartialCapabilityUnverifiedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::PartialCapabilityFidelityClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::FullScreenSpinnerWhenPartialCapable,
                narrowed_label:
                    "This loading state can only prove a partial capability — shown as a partial-capability-unverified projection that preserves the useful partial data and skeleton structure, never blanking a useful pane or falling back to a full-screen spinner where partial capability exists"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "loading_treatment",
                "preserved_partial_data",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::ShellUi,
                M5DecisionFeedbackConsumerSurface::SupportUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.9 — Loading state".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("loading-state-partial-capability-unconfirmed"),
        },
        // Consequence block (recovery posture partial) — structure-heavy (a named blast radius); the block
        // can only disclose a partial / redacted recovery / rollback posture, so it auto-narrows to a
        // recovery-path-disclosed projection that discloses the partial recovery posture alongside the named
        // blast radius, never hiding the consequence behind a fully-reversible claim (yellow). A partial
        // recovery disclosure is an honest disclosed-absence operation, not a trusted overstatement.
        DecisionFeedbackAccessibilityRow {
            record_kind: DECISION_FEEDBACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: DECISION_FEEDBACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:consequence-block-recovery-path-disclosed-partial".to_owned(),
            component_family: M5DecisionFeedbackFamily::ConsequenceBlock,
            source_family_schema_ref: M5DecisionFeedbackFamily::ConsequenceBlock
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "settings:consequence-block:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: DecisionFeedbackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:consequence-block-recovery-path-disclosed-partial:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "primitive_identity",
                "named_blast_radius",
                "recovery_posture",
                "partial_or_redacted_note",
            ]),
            full_ready_claim: M5DecisionFeedbackA11yClaim::TrustedDecisionSurface,
            claim_conditions: vec![condition(
                M5DecisionFeedbackClaimDimension::BlastRadiusRecoveryClarity,
                M5DecisionFeedbackConditionState::RecoveryPathDisclosedPartial,
            )],
            claim_narrow: Some(DecisionFeedbackClaimAutoNarrow {
                narrowed_to: M5DecisionFeedbackA11yClaim::RecoveryPathDisclosedProjection,
                binding_dimension: M5DecisionFeedbackClaimDimension::BlastRadiusRecoveryClarity,
                trigger: M5DecisionFeedbackDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This consequence block can only disclose a partial or redacted recovery and rollback posture — shown as a recovery-path-disclosed projection that discloses the partial recovery posture alongside the named blast radius, never hiding the consequence behind a fully-reversible claim"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "primitive_identity",
                "named_blast_radius",
                "recovery_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DecisionFeedbackConsumerSurface::SettingsUi,
                M5DecisionFeedbackConsumerSurface::SupportUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.10 — Consequence block".to_owned(),
                DECISION_FEEDBACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("consequence-block-recovery-path-disclosed-partial"),
        },
    ]
}
