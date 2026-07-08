//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the M5
//! shared-component-state-taxonomy / interactive-state / selection-or-lock-state /
//! degraded-state-application contract families.
//!
//! This module is the M05-938 accessibility-and-auto-narrowing capstone over the frozen M5
//! shared-component-state matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`]).
//! Where the freeze matrix defines the reusable component-state taxonomy, interactive-state,
//! selection-or-lock-state, and degraded-state-application contracts, and the 933-937
//! implementation / consumer lanes resolve their per-surface truth, this lane certifies — per
//! contract family — that state claims stay **keyboard-complete, assistive-tech-reachable,
//! CLI/export-safe, and self-narrowing** rather than presenting a state whose cause, lock owner,
//! block reason, or recovery truth is missing or stale as still fully exact, live truth:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same typed state, state
//!   cause, lock owner / block reason, and recovery action the rich component shows — never a
//!   pointer-only, hover-only chip that strands assistive-tech or headless users. Hierarchy-heavy
//!   families (the selection-or-lock dense collection's nested tab / tree / list / table lineage)
//!   additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each component's
//!   state meaning from typed tokens and opaque refs without a screenshot, preserving the same
//!   stable state enums, cause / owner / block-reason / recovery fields, and narrowing reasons
//!   shown in-product so state, blocked-action, and recovery truth can be reconstructed without
//!   screenshots or tribal knowledge — never semantically weaker than it is on desktop.
//! - **Honest auto-narrowing.** When a state's cause cannot be resolved, a lock owner cannot be
//!   named, a degraded state's recovery cannot be preserved, or the accessibility / export proof
//!   has gone stale, the component's state claim auto-narrows from `exact_state_truth` /
//!   `reviewable_state_guidance` to a cause- / owner- / recovery-narrowed or stale-proof
//!   projection, discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical state-cause / owner / block-reason / recovery lineage — the
//!   underlying state lineage is never dropped opaquely. A component with every dimension intact
//!   must NOT carry a spurious narrowing, and a missing-cause / missing-owner / missing-recovery
//!   state can never keep an exact state claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the design-system, shell,
//!   command, help, settings, product, CLI, and support / release exports so product, docs, and
//!   release publication stay aligned on state downgrade behavior rather than drifting in copy —
//!   an exact-looking surface can never outrun the cause / owner / recovery proof it is being
//!   viewed away from.
//!
//! Each [`StateComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::M5SharedComponentStateFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5ComponentStateRequiredLabel`] and
//! [`M5ComponentStateDowngradeTrigger`] and the shared [`M5ComponentStateConsumerSurface`]
//! consumer surfaces rather than minting parallel synonyms, so the certified labels stay
//! byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, request bodies, and endpoint secrets
//! never cross this boundary; the packet carries only typed class tokens, opaque state refs,
//! booleans, and controlled labels so support, release, and diagnostics exports can reconstruct
//! exactly what an accessible fallback would have shown without leaking sensitive material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateConsumerSurface, M5ComponentStateDowngradeTrigger, M5ComponentStateRequiredLabel,
    M5SharedComponentStateFamily, M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
};

/// Schema version stamped on the M05-938 shared-state-taxonomy accessibility fallback packet.
pub const STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`StateComponentAccessibilityPacket`].
pub const STATE_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_shared_state_taxonomy_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`StateComponentAccessibilityRow`].
pub const STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_shared_state_taxonomy_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-shared-state-taxonomy-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const STATE_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/design-system/m5_shared_state_taxonomy_accessibility_fallback.md";

/// Repo-relative path of the frozen shared-component-state matrix this lane certifies.
pub const STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_SHARED_COMPONENT_STATE_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const STATE_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-shared-state-taxonomy-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const STATE_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const STATE_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const STATE_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the selection-or-lock
/// dense collection's nested tab / tree / list / table lineage) and therefore MUST bind their
/// tree to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5SharedComponentStateFamily) -> bool {
    matches!(family, M5SharedComponentStateFamily::SelectionOrLockState)
}

/// The state dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5SharedComponentStateFamily,
) -> M5StateComponentClaimDimension {
    match family {
        M5SharedComponentStateFamily::SharedComponentStateTaxonomy => {
            M5StateComponentClaimDimension::StateSemantics
        }
        M5SharedComponentStateFamily::InteractiveState => {
            M5StateComponentClaimDimension::InteractionState
        }
        M5SharedComponentStateFamily::SelectionOrLockState => {
            M5StateComponentClaimDimension::SelectionOrLockState
        }
        M5SharedComponentStateFamily::DegradedStateApplication => {
            M5StateComponentClaimDimension::RecoveryReadiness
        }
    }
}

/// A rendered fallback modality for a shared-state component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentFallbackModality {
    /// A rich, structured (nested selection / lock tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5StateComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentRenderingSurface {
    /// The full-capability desktop state surface.
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

impl M5StateComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
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

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl StateComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
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

/// Whether an export-safe summary preserves the component state meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComponentExportSummaryState {
    /// The component state meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl StateComponentExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl StateComponentNarrowingDisclosureState {
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

/// The state claim ceiling a component asserts: how strong a state-truth posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a state dimension weakens so a
/// missing-cause, missing-owner, missing-recovery, or stale-proof state can never keep an old
/// `ExactStateTruth` or `ReviewableStateGuidance` label — a missing-truth state never
/// masquerades as exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentClaim {
    /// Exact state truth: the typed state, its cause, its owner / block reason, and its recovery
    /// are all named, live, and complete — the strongest claim, a surface Aureline can present as
    /// exactly true right now.
    ExactStateTruth,
    /// Reviewable state guidance: a self-sufficient, reviewable read-only state projection
    /// (guidance a user can read) that is not itself a certified exact-truth path.
    ReviewableStateGuidance,
    /// Cause-narrowed projection: the state's cause could not be resolved; the surface stays a
    /// cause-narrowed explanation, never an exact live state.
    CauseNarrowedProjection,
    /// Owner-narrowed projection: the lock / disabled / read-only owner could not be resolved;
    /// the surface stays an owner-narrowed explanation, never a plain silent disabled control.
    OwnerNarrowedProjection,
    /// Recovery-narrowed projection: the degraded / warning / error state's recovery could not be
    /// preserved; the surface stays a recovery-narrowed explanation, never a healthy live state.
    RecoveryNarrowedProjection,
    /// Stale-proof projection: the accessibility / export proof has gone stale; the surface stays
    /// a stale-proof projection with its identity, state, and keyboard route preserved.
    StaleProofProjection,
}

impl M5StateComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ExactStateTruth,
        Self::ReviewableStateGuidance,
        Self::CauseNarrowedProjection,
        Self::OwnerNarrowedProjection,
        Self::RecoveryNarrowedProjection,
        Self::StaleProofProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger state posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExactStateTruth => 5,
            Self::ReviewableStateGuidance => 4,
            Self::CauseNarrowedProjection => 3,
            Self::OwnerNarrowedProjection => 2,
            Self::RecoveryNarrowedProjection => 1,
            Self::StaleProofProjection => 0,
        }
    }

    /// Returns true when this claim asserts fully exact, live state parity.
    pub const fn asserts_exact_state(self) -> bool {
        matches!(self, Self::ExactStateTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (exact or reviewable) state
    /// projection.
    pub const fn asserts_trustworthy_state(self) -> bool {
        matches!(self, Self::ExactStateTruth | Self::ReviewableStateGuidance)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactStateTruth => "exact_state_truth",
            Self::ReviewableStateGuidance => "reviewable_state_guidance",
            Self::CauseNarrowedProjection => "cause_narrowed_projection",
            Self::OwnerNarrowedProjection => "owner_narrowed_projection",
            Self::RecoveryNarrowedProjection => "recovery_narrowed_projection",
            Self::StaleProofProjection => "stale_proof_projection",
        }
    }
}

/// The state dimension whose state governs how far a component may claim to be an exact, live
/// state surface. The dimensions map 1:1 to the four frozen contract families so every family
/// carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentClaimDimension {
    /// State semantics: does the shared taxonomy resolve the state's identity, class, and cause,
    /// or is the state cause unresolved?
    StateSemantics,
    /// Interaction state: does the interactive-state contract render default / hover /
    /// focus-visible / pressed truth with a keyboard route, or is the proof stale?
    InteractionState,
    /// Selection or lock state: does the selection-or-lock contract name its selected / current /
    /// disabled / read-only / locked owner, or is the lock owner unresolved?
    SelectionOrLockState,
    /// Recovery readiness: does the degraded-state-application contract name what still works and
    /// the recovery action, or is the recovery unavailable?
    RecoveryReadiness,
}

impl M5StateComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StateSemantics,
        Self::InteractionState,
        Self::SelectionOrLockState,
        Self::RecoveryReadiness,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateSemantics => "state_semantics",
            Self::InteractionState => "interaction_state",
            Self::SelectionOrLockState => "selection_or_lock_state",
            Self::RecoveryReadiness => "recovery_readiness",
        }
    }
}

/// The observed condition of one state dimension. Anything weaker than
/// [`Self::LiveExactState`] imposes a narrowing ceiling on the component's state claim. The four
/// spec axes the lane must auto-narrow on — a state whose cause, lock owner, or recovery truth is
/// missing, or whose proof is stale — are [`Self::StateCauseUnresolved`],
/// [`Self::LockOwnerUnresolved`], [`Self::RecoveryUnavailable`], and [`Self::ProofStale`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentConditionState {
    /// Live, typed, cause-resolved, owner-named, recovery-ready — imposes no ceiling.
    LiveExactState,
    /// The state's cause could not be resolved — state claim drops to a cause-narrowed
    /// projection.
    StateCauseUnresolved,
    /// The lock / disabled / read-only owner could not be resolved — state claim drops to an
    /// owner-narrowed projection.
    LockOwnerUnresolved,
    /// The degraded / warning / error state's recovery could not be preserved — state claim drops
    /// to a recovery-narrowed projection.
    RecoveryUnavailable,
    /// The accessibility / export proof has gone stale — state claim drops to a stale-proof
    /// projection.
    ProofStale,
}

impl M5StateComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveExactState,
        Self::StateCauseUnresolved,
        Self::LockOwnerUnresolved,
        Self::RecoveryUnavailable,
        Self::ProofStale,
    ];

    /// Returns true when the dimension is weaker than exact and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveExactState)
    }

    /// Returns true when the condition reflects a missing state cause / owner / recovery truth
    /// that must never be shown as exact state. A stale proof is a freshness reduction, not a
    /// missing-truth overstatement, so it is deliberately excluded here.
    pub const fn is_missing_state_truth(self) -> bool {
        matches!(
            self,
            Self::StateCauseUnresolved | Self::LockOwnerUnresolved | Self::RecoveryUnavailable
        )
    }

    /// The strongest state claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5StateComponentClaim {
        match self {
            Self::LiveExactState => M5StateComponentClaim::ExactStateTruth,
            Self::StateCauseUnresolved => M5StateComponentClaim::CauseNarrowedProjection,
            Self::LockOwnerUnresolved => M5StateComponentClaim::OwnerNarrowedProjection,
            Self::RecoveryUnavailable => M5StateComponentClaim::RecoveryNarrowedProjection,
            Self::ProofStale => M5StateComponentClaim::StaleProofProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ComponentStateDowngradeTrigger {
        match self {
            // The exact baseline never narrows; kept for exhaustiveness.
            Self::LiveExactState => M5ComponentStateDowngradeTrigger::ProofStale,
            Self::StateCauseUnresolved => M5ComponentStateDowngradeTrigger::StateCauseUnstated,
            Self::LockOwnerUnresolved => M5ComponentStateDowngradeTrigger::LockOwnerMasked,
            Self::RecoveryUnavailable => {
                M5ComponentStateDowngradeTrigger::ConsequenceOrRecoveryOmitted
            }
            Self::ProofStale => M5ComponentStateDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExactState => "live_exact_state",
            Self::StateCauseUnresolved => "state_cause_unresolved",
            Self::LockOwnerUnresolved => "lock_owner_unresolved",
            Self::RecoveryUnavailable => "recovery_unavailable",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One state dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5StateComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5StateComponentConditionState,
}

/// An honest state-claim auto-narrow block. When a state dimension weakens, the component's state
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and
/// preserves the canonical state-cause / owner / block-reason / recovery lineage rather than
/// silently dropping it — the underlying state lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentClaimAutoNarrow {
    /// The state claim the component is narrowed to.
    pub narrowed_to: M5StateComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5StateComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ComponentStateDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical typed state, its cause, its owner / block reason, and its recovery action
    /// are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying state-cause / owner / block-reason / recovery lineage is preserved (never
    /// dropped) across the narrowing; must hold so cause-narrowed, owner-narrowed,
    /// recovery-narrowed, and stale-proof states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl StateComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and state lineage
    /// and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl StateComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5StateComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: StateComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a shared-state accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims exactness, or drops state silently
    /// (red).
    Stranded,
}

impl StateComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one shared-component-state contract family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentAccessibilityRow {
    /// Record kind; must equal [`STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen contract family this row certifies.
    pub component_family: M5SharedComponentStateFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the typed state / cause / owner / recovery object this component projects;
    /// stays visible on every surface, so this is never empty.
    pub state_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5StateComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical typed state, cause, owner / block
    /// reason, and recovery truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: StateComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: StateComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: StateComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component state meaning.
    pub export_summary: StateComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: StateComponentCopyExportParity,
    /// The full state claim this family asserts when every dimension is intact.
    pub full_state_claim: M5StateComponentClaim,
    /// The observed condition of each modeled state dimension.
    #[serde(default)]
    pub claim_conditions: Vec<StateComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<StateComponentClaimAutoNarrow>,
    /// Whether the underlying state lineage is preserved on this component regardless of
    /// narrowing; must hold so cause-narrowed, owner-narrowed, recovery-narrowed, and stale-proof
    /// states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5StateComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<StateComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl StateComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `LiveExactState` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5StateComponentClaimDimension,
    ) -> M5StateComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5StateComponentConditionState::LiveExactState)
    }

    /// Whether any modeled dimension is weaker than exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest state claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5StateComponentClaim {
        let mut permitted = self.full_state_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&StateComponentClaimConditionEntry> {
        let mut binding: Option<(&StateComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_state_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5StateComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The state claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5StateComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_state_claim,
        }
    }

    /// AC / auto-narrowing honesty: a missing-cause, missing-owner, missing-recovery, or
    /// stale-proof state can no longer keep an old `ExactStateTruth` / `ReviewableStateGuidance`
    /// label. The effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the permitted
    /// ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and state lineage. When nothing narrows, no spurious narrow block is
    /// present.
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

    /// AC / exact-state honesty: a missing-cause / missing-owner / missing-recovery state never
    /// keeps an exact state claim. When such a state is modeled, the effective claim must not
    /// assert `ExactStateTruth`.
    pub fn exact_state_honesty_holds(&self) -> bool {
        let has_missing_state_truth = self
            .claim_conditions
            .iter()
            .any(|c| c.state.is_missing_state_truth());
        !(has_missing_state_truth && self.effective_claim().asserts_exact_state())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.state_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component state meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: cause-narrowed, owner-narrowed, recovery-narrowed, and stale-proof states
    /// preserve the underlying state lineage. The row must assert `lineage_preserved`, and any
    /// narrow block must preserve lineage continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned
    /// on the same narrowed state.
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
        M5ComponentStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> StateComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.exact_state_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return StateComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            StateComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            StateComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.state_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_state_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-938 shared-state-taxonomy accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_exact_state_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`StateComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<StateComponentAccessibilityRow>,
}

/// Checked-in M05-938 shared-state-taxonomy accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<StateComponentAccessibilityRow>,
    pub summary: StateComponentAccessibilitySummary,
}

impl StateComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: StateComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: StateComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_exact_state_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5SharedComponentStateFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5StateComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5StateComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// State claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5StateComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ComponentStateConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> StateComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ComponentStateConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&StateComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                StateComponentAccessibilityStatus::Parity => green += 1,
                StateComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                StateComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        StateComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::claim_is_honest),
            all_exact_state_honesty_holds: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::exact_state_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(StateComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<StateComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(StateComponentAccessibilityViolation::SchemaVersion {
                expected: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != STATE_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(StateComponentAccessibilityViolation::RecordKind {
                expected: STATE_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(StateComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_missing_state_truth_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(StateComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.is_missing_state_truth())
            {
                has_missing_state_truth_row = true;
            }

            if !row.is_complete() {
                violations.push(StateComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    StateComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory state label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    StateComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5StateComponentFallbackModality::Structured)
            {
                violations.push(
                    StateComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts an exact / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(StateComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a missing-cause / missing-owner / missing-recovery state never keeps an exact
            // state claim.
            if !row.exact_state_honesty_holds() {
                violations.push(
                    StateComponentAccessibilityViolation::MissingStateTruthShownAsExact {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    StateComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    StateComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: cause-narrowed, owner-narrowed, recovery-narrowed, and stale-proof
            // states preserve state lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(StateComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    StateComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    StateComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == StateComponentAccessibilityStatus::Stranded {
                violations.push(StateComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5SharedComponentStateFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(StateComponentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5StateComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    StateComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the exact baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5StateComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    StateComponentAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every state claim tier appears as an effective claim, so the full narrowing
        // spectrum (exact-state → … → stale-proof) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5StateComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(StateComponentAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Exact-state honesty must be proven with at least one missing-cause / missing-owner /
        // missing-recovery row in the packet, so the "missing-truth never shown as exact"
        // guarantee is exercised end-to-end.
        if !has_missing_state_truth_row {
            violations.push(StateComponentAccessibilityViolation::ExactStateHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the design-system, shell, command,
        // help, settings, support / release, CLI, and product surfaces — so every consumer
        // surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ComponentStateConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    StateComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(StateComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("shared-state accessibility fallback packet serializes"),
        ) {
            violations.push(StateComponentAccessibilityViolation::RawStateMaterialInExport);
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
            .expect("shared-state accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_state_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Shared-State Taxonomy Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5SharedComponentStateFamily::ALL.len(),
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
                    row.full_state_claim.as_str(),
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

/// Reads and validates the checked-in shared-state-taxonomy accessibility fallback export.
pub fn current_m5_state_component_a11y_fallback_export(
) -> Result<StateComponentAccessibilityPacket, StateComponentAccessibilityArtifactError> {
    let packet: StateComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/support_export.json"
    )))
    .map_err(StateComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StateComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in shared-state-taxonomy accessibility fallback export.
#[derive(Debug)]
pub enum StateComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StateComponentAccessibilityViolation>),
}

impl fmt::Display for StateComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "shared-state accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "shared-state accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for StateComponentAccessibilityArtifactError {}

/// Validation failure for M05-938 shared-state-taxonomy accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateComponentAccessibilityViolation {
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
        dimension: M5StateComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    MissingStateTruthShownAsExact {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    LineageDropped {
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
        family: M5SharedComponentStateFamily,
    },
    MissingDimensionCoverage {
        dimension: M5StateComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5StateComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5StateComponentClaim,
    },
    ExactStateHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ComponentStateConsumerSurface,
    },
    SummaryMismatch,
    RawStateMaterialInExport,
}

impl fmt::Display for StateComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory state label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts an exact / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::MissingStateTruthShownAsExact { id } => {
                write!(
                    f,
                    "row {id} shows a missing-cause / missing-owner / missing-recovery state as exact state truth"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve state lineage across narrowing"
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
                    "component family {family:?} is not certified in the packet"
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
                    "state claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::ExactStateHonestyUnproven => {
                write!(
                    f,
                    "no missing-cause / missing-owner / missing-recovery row is present to prove the exact-state-honesty guarantee"
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
            Self::RawStateMaterialInExport => {
                write!(f, "export contains raw state material")
            }
        }
    }
}

impl Error for StateComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unresolved"
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
            | "disabled"
            | "locked"
            | "stale"
            | "stale proof"
            | "no owner"
            | "no cause"
            | "no recovery"
            | "unknown"
            | "missing"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in shared-state-taxonomy accessibility fallback packet. This is
/// the one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_state_component_a11y_fallback_packet() -> StateComponentAccessibilityPacket {
    StateComponentAccessibilityPacket::new(StateComponentAccessibilityPacketInput {
        packet_id: "m5-shared-state-taxonomy-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:shared-state-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ComponentStateRequiredLabel> {
    M5ComponentStateRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> StateComponentCopyExportParity {
    StateComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5StateComponentClaimDimension,
    state: M5StateComponentConditionState,
) -> StateComponentClaimConditionEntry {
    StateComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// headless — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5ComponentStateConsumerSurface],
) -> Vec<M5ComponentStateConsumerSurface> {
    let mut out = vec![
        M5ComponentStateConsumerSurface::SupportExport,
        M5ComponentStateConsumerSurface::CliHeadless,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: StateComponentNarrowingDisclosureState,
) -> Vec<StateComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        StateComponentRenderingNarrowingDisclosure {
            rendering_surface: M5StateComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        StateComponentRenderingNarrowingDisclosure {
            rendering_surface: M5StateComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_state_treatment".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<StateComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        StateComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<StateComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        StateComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5StateComponentRenderingSurface> {
    vec![
        M5StateComponentRenderingSurface::DesktopFull,
        M5StateComponentRenderingSurface::CliHeadless,
        M5StateComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<StateComponentAccessibilityRow> {
    vec![
        // Interactive state (live) — default / hover / focus-visible / pressed truth is live,
        // keyboard-routed, and reachable on every surface, so it is fully exact state truth
        // (green).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:interactive-state-live".to_owned(),
            component_family: M5SharedComponentStateFamily::InteractiveState,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:interactive-control:0001".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:interactive-state-live:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "interactive_state",
                "state_cause",
                "keyboard_route",
            ]),
            full_state_claim: M5StateComponentClaim::ExactStateTruth,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::InteractionState,
                M5StateComponentConditionState::LiveExactState,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "control_identity",
                "interactive_state",
                "keyboard_route",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::DesignSystemUi,
                M5ComponentStateConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §10.1 shared component state taxonomy".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("interactive-state-live"),
        },
        // Shared component state taxonomy (reviewable) — the state identity, class, and cause are
        // all stated and the row is a self-sufficient, reviewable read-only state explanation (not
        // itself an exact-truth path), reachable on every surface (green).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:shared-state-taxonomy-reviewable".to_owned(),
            component_family: M5SharedComponentStateFamily::SharedComponentStateTaxonomy,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:shared-taxonomy:0002".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:shared-state-taxonomy-reviewable:a11y".to_owned(),
            copy_export: copy_export(&[
                "state_identity",
                "state_class",
                "state_cause",
                "keyboard_route",
            ]),
            full_state_claim: M5StateComponentClaim::ReviewableStateGuidance,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::StateSemantics,
                M5StateComponentConditionState::LiveExactState,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "state_identity",
                "state_class",
                "state_cause",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::CommandUi,
                M5ComponentStateConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UX Design System §15.4 shared component state taxonomy".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("shared-state-taxonomy-reviewable"),
        },
        // Shared component state taxonomy (cause unresolved) — the state's cause could not be
        // resolved from a live signal, so the row auto-narrows to a cause-narrowed projection
        // rather than presenting a fully-explained live state, while keeping its identity, class,
        // and keyboard route visible (yellow).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:shared-state-taxonomy-cause-unresolved".to_owned(),
            component_family: M5SharedComponentStateFamily::SharedComponentStateTaxonomy,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:shared-taxonomy:0003".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:shared-state-taxonomy-cause-unresolved:a11y".to_owned(),
            copy_export: copy_export(&[
                "state_identity",
                "state_class",
                "cause_resolution_state",
                "keyboard_route",
            ]),
            full_state_claim: M5StateComponentClaim::ExactStateTruth,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::StateSemantics,
                M5StateComponentConditionState::StateCauseUnresolved,
            )],
            claim_narrow: Some(StateComponentClaimAutoNarrow {
                narrowed_to: M5StateComponentClaim::CauseNarrowedProjection,
                binding_dimension: M5StateComponentClaimDimension::StateSemantics,
                trigger: M5ComponentStateDowngradeTrigger::StateCauseUnstated,
                narrowed_label:
                    "The reason this state applies could not be resolved from a live signal — shown as a cause-narrowed projection that still names the component identity, its typed state, and the keyboard route, never as a fully-explained live state"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "state_identity",
                "state_class",
                "cause_resolution_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::SettingsUi,
                M5ComponentStateConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §10.1 global state / degraded-mode rules".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("shared-state-taxonomy-cause-unresolved"),
        },
        // Selection or lock state (owner unresolved) — hierarchy-heavy (nested tab / tree / list /
        // table lineage); the lock / disabled owner could not be resolved, so the row auto-narrows
        // to an owner-narrowed projection and binds its nested collection to a flat list / textual
        // path, keeping the item identity, selection/lock state, and inspect route visible
        // (yellow).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:selection-or-lock-state-owner-unresolved".to_owned(),
            component_family: M5SharedComponentStateFamily::SelectionOrLockState,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:selection-or-lock:0004".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::Structured,
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:selection-or-lock-state-owner-unresolved:a11y".to_owned(),
            copy_export: copy_export(&[
                "item_identity",
                "selection_or_lock_state",
                "lock_owner_state",
                "inspect_route",
            ]),
            full_state_claim: M5StateComponentClaim::ExactStateTruth,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::SelectionOrLockState,
                M5StateComponentConditionState::LockOwnerUnresolved,
            )],
            claim_narrow: Some(StateComponentClaimAutoNarrow {
                narrowed_to: M5StateComponentClaim::OwnerNarrowedProjection,
                binding_dimension: M5StateComponentClaimDimension::SelectionOrLockState,
                trigger: M5ComponentStateDowngradeTrigger::LockOwnerMasked,
                narrowed_label:
                    "The policy or ownership behind this lock could not be resolved — shown as an owner-narrowed projection that keeps the item identity, its selection and lock state, and the inspect route visible, never as a plain silent disabled control"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "item_identity",
                "selection_or_lock_state",
                "lock_owner_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::DesignSystemUi,
                M5ComponentStateConsumerSurface::CommandUi,
            ]),
            source_refs: vec![
                "UX Design System §15.4 component-state rules (locked over disabled)".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("selection-or-lock-state-owner-unresolved"),
        },
        // Degraded state application (recovery unavailable) — the degraded / warning / error
        // state's recovery could not be preserved, so the row auto-narrows to a recovery-narrowed
        // projection that still names what still works and the state consequence, never as a
        // healthy live state (yellow).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:degraded-state-application-recovery-unavailable".to_owned(),
            component_family: M5SharedComponentStateFamily::DegradedStateApplication,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:degraded-application:0005".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:degraded-state-application-recovery-unavailable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "surface_identity",
                "degraded_state",
                "what_still_works",
                "recovery_action_state",
            ]),
            full_state_claim: M5StateComponentClaim::ExactStateTruth,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::RecoveryReadiness,
                M5StateComponentConditionState::RecoveryUnavailable,
            )],
            claim_narrow: Some(StateComponentClaimAutoNarrow {
                narrowed_to: M5StateComponentClaim::RecoveryNarrowedProjection,
                binding_dimension: M5StateComponentClaimDimension::RecoveryReadiness,
                trigger: M5ComponentStateDowngradeTrigger::ConsequenceOrRecoveryOmitted,
                narrowed_label:
                    "The recovery path out of this degraded state could not be preserved — shown as a recovery-narrowed projection that still names what still works and the state consequence, never as a healthy live state"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "surface_identity",
                "degraded_state",
                "what_still_works",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::ProductUi,
                M5ComponentStateConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §10.1 degraded-mode rules (name consequence and recovery)".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("degraded-state-application-recovery-unavailable"),
        },
        // Interactive state (proof stale) — the accessibility / export proof for this interactive
        // state has gone stale, so the row auto-narrows to a stale-proof projection with its
        // identity, state, and keyboard route preserved, never as freshly-verified parity
        // (yellow).
        StateComponentAccessibilityRow {
            record_kind: STATE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:interactive-state-proof-stale".to_owned(),
            component_family: M5SharedComponentStateFamily::InteractiveState,
            source_family_schema_ref: STATE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            state_context_ref: "state:interactive-control:0006".to_owned(),
            fallback_modalities: vec![
                M5StateComponentFallbackModality::List,
                M5StateComponentFallbackModality::Textual,
                M5StateComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: StateComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: StateComponentExportSummaryState::DisclosedPartialCapture,
            export_summary_ref: "summary:interactive-state-proof-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "interactive_state",
                "proof_freshness_state",
                "keyboard_route",
            ]),
            full_state_claim: M5StateComponentClaim::ExactStateTruth,
            claim_conditions: vec![condition(
                M5StateComponentClaimDimension::InteractionState,
                M5StateComponentConditionState::ProofStale,
            )],
            claim_narrow: Some(StateComponentClaimAutoNarrow {
                narrowed_to: M5StateComponentClaim::StaleProofProjection,
                binding_dimension: M5StateComponentClaimDimension::InteractionState,
                trigger: M5ComponentStateDowngradeTrigger::ProofStale,
                narrowed_label:
                    "The accessibility and export proof for this interactive state has gone out of date — shown as a stale-proof projection with its identity, typed state, and keyboard route preserved, never as freshly-verified parity"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "interactive_state",
                "proof_freshness_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComponentStateConsumerSurface::ShellUi,
                M5ComponentStateConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "TAD/TDD state-semantic conformance and proof freshness".to_owned(),
                STATE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("interactive-state-proof-stale"),
        },
    ]
}
