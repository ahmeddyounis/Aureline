//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 contextual-tip-card / migration-bridge-card / sequence-help-strip /
//! why-unavailable-explanation-row / source-language-fallback teaching components.
//!
//! This module is the M05-930 accessibility-and-auto-narrowing capstone over the frozen
//! M5 contextual-teaching / migration-bridge component matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`]).
//! Where the freeze matrix defines the reusable contextual-tip card, migration-bridge card,
//! sequence-help strip, why-unavailable explanation row, and source-language fallback primitives,
//! and the 925-929 implementation / consumer lanes resolve their per-surface truth, this lane
//! certifies — per component family — that teaching claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than presenting a
//! snoozed tip, a partial migration bridge, an unsupported command sequence, or stale localized
//! fallback content as still fully exact, live, authoritative teaching:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same command binding,
//!   migration mapping state, blocked-action owner / reason / next safe action, sequence-help
//!   state, and source-language citation the rich component shows — never a hover-only chip that
//!   strands assistive-tech or headless users. Hierarchy-heavy families (the sequence-help
//!   strip's nested leader / chord / motion / operator / terminal-action lineage) additionally
//!   bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same stable
//!   command IDs, mapping states, blocked-action owner / reason / next actions, source-language
//!   fallback links, and narrowing reasons shown in-product so teaching, migration, and
//!   blocked-action truth can be reconstructed without screenshots or tribal knowledge.
//! - **Honest auto-narrowing.** When a tip is snoozed, a migration bridge is partial, a command
//!   sequence is unsupported, or localized fallback content is stale, the component's teaching
//!   claim auto-narrows from `ExactTeaching` / `ReviewableGuidance` to a snoozed-tip /
//!   partial-bridge / unsupported-sequence / stale-fallback projection, discloses the narrowing
//!   with a precise trigger and binding dimension, and preserves the canonical command-binding /
//!   migration-mapping / blocked-action / source-language lineage — the underlying teaching
//!   lineage is never dropped opaquely. A component with every dimension intact must NOT carry a
//!   spurious narrowing, and a partial / unsupported / stale state can never keep an exact
//!   teaching claim.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the onboarding UI,
//!   tour overlay, command palette, migration report, inline tip, help panel, CLI help, product
//!   UI, and support / release exports so product, docs, and release publication stay aligned on
//!   teaching downgrade behavior rather than drifting in copy — an exact-looking surface can
//!   never outrun the mapping / sequence / fallback proof it is being viewed away from.
//!
//! Each [`TeachingComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::M5ContextualTeachingComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5TeachingRequiredLabel`] and
//! [`M5TeachingDowngradeTrigger`] and the shared [`M5TeachingConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw credentials, tokens, request bodies, and endpoint secrets
//! never cross this boundary; the packet carries only typed class tokens, opaque teaching refs,
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
use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5ContextualTeachingComponentFamily, M5TeachingConsumerSurface, M5TeachingDowngradeTrigger,
    M5TeachingRequiredLabel, M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-930 contextual-teaching component accessibility fallback
/// packet.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TeachingComponentAccessibilityPacket`].
pub const TEACHING_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_contextual_teaching_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`TeachingComponentAccessibilityRow`].
pub const TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_contextual_teaching_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-contextual-teaching-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/help/m5_contextual_teaching_component_accessibility_fallback.md";

/// Repo-relative path of the frozen contextual-teaching / migration-bridge component matrix this
/// lane certifies.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-contextual-teaching-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEACHING_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEACHING_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the sequence-help strip's
/// nested leader / chord / motion / operator / terminal-action lineage) and therefore MUST bind
/// their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5ContextualTeachingComponentFamily) -> bool {
    matches!(
        family,
        M5ContextualTeachingComponentFamily::SequenceHelpStrip
    )
}

/// The teaching dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5ContextualTeachingComponentFamily,
) -> M5TeachingComponentClaimDimension {
    match family {
        M5ContextualTeachingComponentFamily::ContextualTipCard => {
            M5TeachingComponentClaimDimension::TipDelivery
        }
        M5ContextualTeachingComponentFamily::MigrationBridgeCard => {
            M5TeachingComponentClaimDimension::MigrationMapping
        }
        M5ContextualTeachingComponentFamily::SequenceHelpStrip => {
            M5TeachingComponentClaimDimension::SequenceState
        }
        M5ContextualTeachingComponentFamily::WhyUnavailableExplanationRow => {
            M5TeachingComponentClaimDimension::BlockedExplanation
        }
        M5ContextualTeachingComponentFamily::SourceLanguageFallback => {
            M5TeachingComponentClaimDimension::SourceLanguage
        }
    }
}

/// A rendered fallback modality for a contextual-teaching component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentFallbackModality {
    /// A rich, structured (nested sequence / mapping tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5TeachingComponentFallbackModality {
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
pub enum M5TeachingComponentRenderingSurface {
    /// The full-capability desktop teaching surface.
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

impl M5TeachingComponentRenderingSurface {
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
pub enum TeachingComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl TeachingComponentNonVisualReachState {
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

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl TeachingComponentExportSummaryState {
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
pub enum TeachingComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl TeachingComponentNarrowingDisclosureState {
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

/// The teaching claim ceiling a component asserts: how strong a teaching posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a teaching dimension weakens so a
/// snoozed tip, a partial migration bridge, an unsupported command sequence, or stale localized
/// fallback content can never keep an old `ExactTeaching` or `ReviewableGuidance` label — a
/// partial / unsupported / stale state never masquerades as exact teaching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentClaim {
    /// Exact teaching: a live, command-backed, exactly-mapped, localized-current teaching
    /// component — the strongest claim, a surface Aureline can teach against as exactly true
    /// right now.
    ExactTeaching,
    /// Reviewable guidance: a self-sufficient, reviewable read-only explanation (guidance a
    /// user can read) that is not itself a certified exact-teaching path.
    ReviewableGuidance,
    /// Snoozed-tip projection: the contextual tip is snoozed / suppressed and only its
    /// stable command binding remains available; the surface must not present it as an active
    /// live tip.
    SnoozedTipProjection,
    /// Partial-bridge projection: the migration bridge maps the imported behavior only
    /// partially; the surface stays a partial-bridge explanation, never an exact mapping.
    PartialBridgeProjection,
    /// Unsupported-sequence projection: the command sequence has no committed binding in the
    /// current context; the surface stays an unsupported-sequence explanation.
    UnsupportedSequenceProjection,
    /// Stale-fallback projection: the localized help content is stale / source-language only;
    /// the surface stays a stale-fallback explanation with its canonical citation preserved.
    StaleFallbackProjection,
}

impl M5TeachingComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ExactTeaching,
        Self::ReviewableGuidance,
        Self::SnoozedTipProjection,
        Self::PartialBridgeProjection,
        Self::UnsupportedSequenceProjection,
        Self::StaleFallbackProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger teaching posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExactTeaching => 5,
            Self::ReviewableGuidance => 4,
            Self::SnoozedTipProjection => 3,
            Self::PartialBridgeProjection => 2,
            Self::UnsupportedSequenceProjection => 1,
            Self::StaleFallbackProjection => 0,
        }
    }

    /// Returns true when this claim asserts fully exact, live teaching parity.
    pub const fn asserts_exact_teaching(self) -> bool {
        matches!(self, Self::ExactTeaching)
    }

    /// Returns true when this claim asserts a fully self-sufficient (exact or reviewable)
    /// teaching projection.
    pub const fn asserts_trustworthy_teaching(self) -> bool {
        matches!(self, Self::ExactTeaching | Self::ReviewableGuidance)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTeaching => "exact_teaching",
            Self::ReviewableGuidance => "reviewable_guidance",
            Self::SnoozedTipProjection => "snoozed_tip_projection",
            Self::PartialBridgeProjection => "partial_bridge_projection",
            Self::UnsupportedSequenceProjection => "unsupported_sequence_projection",
            Self::StaleFallbackProjection => "stale_fallback_projection",
        }
    }
}

/// The teaching dimension whose state governs how far a component may claim to be an exact, live
/// teaching surface. The dimensions map 1:1 to the five frozen component families so every
/// family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentClaimDimension {
    /// Tip delivery: is the contextual tip card active and command-backed, or is it snoozed /
    /// suppressed?
    TipDelivery,
    /// Migration mapping: does the migration bridge card map the imported behavior exactly, or
    /// is the mapping only partial / unsupported?
    MigrationMapping,
    /// Sequence state: does the sequence-help strip resolve a committed command sequence, or is
    /// the sequence unsupported in the current context?
    SequenceState,
    /// Blocked explanation: does the why-unavailable explanation row name its blocked-action
    /// owner, reason, and next safe action, or is the explanation incomplete?
    BlockedExplanation,
    /// Source language: is the localized help content current, or is it falling back to stale /
    /// source-language content?
    SourceLanguage,
}

impl M5TeachingComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TipDelivery,
        Self::MigrationMapping,
        Self::SequenceState,
        Self::BlockedExplanation,
        Self::SourceLanguage,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TipDelivery => "tip_delivery",
            Self::MigrationMapping => "migration_mapping",
            Self::SequenceState => "sequence_state",
            Self::BlockedExplanation => "blocked_explanation",
            Self::SourceLanguage => "source_language",
        }
    }
}

/// The observed condition of one teaching dimension. Anything weaker than
/// [`Self::LiveExactTeaching`] imposes a narrowing ceiling on the component's teaching claim. The
/// four spec axes the lane must auto-narrow on — a snoozed tip, a partial bridge, an unsupported
/// sequence, and stale fallback content — are [`Self::TipSnoozed`], [`Self::BridgePartial`],
/// [`Self::SequenceUnsupported`], and [`Self::FallbackStale`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentConditionState {
    /// Live, command-backed, exactly-mapped, localized-current — imposes no ceiling.
    LiveExactTeaching,
    /// The contextual tip is snoozed / suppressed — teaching claim drops to a snoozed-tip
    /// projection.
    TipSnoozed,
    /// The migration bridge maps the imported behavior only partially — teaching claim drops to
    /// a partial-bridge projection.
    BridgePartial,
    /// The command sequence has no committed binding in the current context — teaching claim
    /// drops to an unsupported-sequence projection.
    SequenceUnsupported,
    /// The localized help content is stale / source-language only — teaching claim drops to a
    /// stale-fallback projection.
    FallbackStale,
}

impl M5TeachingComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveExactTeaching,
        Self::TipSnoozed,
        Self::BridgePartial,
        Self::SequenceUnsupported,
        Self::FallbackStale,
    ];

    /// Returns true when the dimension is weaker than exact and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveExactTeaching)
    }

    /// Returns true when the condition reflects a partial / unsupported / stale state that must
    /// never be shown as exact teaching. A snoozed tip is a delivery state, not an exactness
    /// overstatement, so it is deliberately excluded here.
    pub const fn is_partial_unsupported_or_stale(self) -> bool {
        matches!(
            self,
            Self::BridgePartial | Self::SequenceUnsupported | Self::FallbackStale
        )
    }

    /// The strongest teaching claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5TeachingComponentClaim {
        match self {
            Self::LiveExactTeaching => M5TeachingComponentClaim::ExactTeaching,
            Self::TipSnoozed => M5TeachingComponentClaim::SnoozedTipProjection,
            Self::BridgePartial => M5TeachingComponentClaim::PartialBridgeProjection,
            Self::SequenceUnsupported => M5TeachingComponentClaim::UnsupportedSequenceProjection,
            Self::FallbackStale => M5TeachingComponentClaim::StaleFallbackProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5TeachingDowngradeTrigger {
        match self {
            // The exact baseline never narrows; kept for exhaustiveness.
            Self::LiveExactTeaching => M5TeachingDowngradeTrigger::ProofStale,
            Self::TipSnoozed => M5TeachingDowngradeTrigger::TipCommandBindingUnstated,
            Self::BridgePartial => M5TeachingDowngradeTrigger::MigrationMappingUnstated,
            Self::SequenceUnsupported => M5TeachingDowngradeTrigger::SequenceHelpStateUnstated,
            Self::FallbackStale => M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExactTeaching => "live_exact_teaching",
            Self::TipSnoozed => "tip_snoozed",
            Self::BridgePartial => "bridge_partial",
            Self::SequenceUnsupported => "sequence_unsupported",
            Self::FallbackStale => "fallback_stale",
        }
    }
}

/// One teaching dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5TeachingComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5TeachingComponentConditionState,
}

/// An honest teaching-claim auto-narrow block. When a teaching dimension weakens, the
/// component's teaching claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical command-binding / migration-mapping /
/// blocked-action / source-language lineage rather than silently dropping it — the underlying
/// teaching lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentClaimAutoNarrow {
    /// The teaching claim the component is narrowed to.
    pub narrowed_to: M5TeachingComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5TeachingComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5TeachingDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical command binding, migration mapping, blocked-action owner / reason / next
    /// action, and source-language citation are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying command-binding / migration-mapping / blocked-action / source-language
    /// lineage is preserved (never dropped) across the narrowing; must hold so snoozed-tip,
    /// partial-bridge, unsupported-sequence, and stale-fallback states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl TeachingComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and teaching
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl TeachingComponentCopyExportParity {
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
pub struct TeachingComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5TeachingComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: TeachingComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a contextual-teaching accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims exactness, or drops state
    /// silently (red).
    Stranded,
}

impl TeachingComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one contextual-teaching component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentAccessibilityRow {
    /// Record kind; must equal [`TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the command / mapping / blocked-action / fallback object this component
    /// teaches; stays visible on every surface, so this is never empty.
    pub teaching_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5TeachingComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical command binding, mapping,
    /// blocked-action, sequence, and source-language truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: TeachingComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: TeachingComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: TeachingComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: TeachingComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: TeachingComponentCopyExportParity,
    /// The full teaching claim this family asserts when every dimension is intact.
    pub full_teaching_claim: M5TeachingComponentClaim,
    /// The observed condition of each modeled teaching dimension.
    #[serde(default)]
    pub claim_conditions: Vec<TeachingComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<TeachingComponentClaimAutoNarrow>,
    /// Whether the underlying teaching lineage is preserved on this component regardless of
    /// narrowing; must hold so snoozed-tip, partial-bridge, unsupported-sequence, and
    /// stale-fallback states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5TeachingComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<TeachingComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5TeachingRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl TeachingComponentAccessibilityRow {
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

    /// The condition state observed for one dimension, or `LiveExactTeaching` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5TeachingComponentClaimDimension,
    ) -> M5TeachingComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5TeachingComponentConditionState::LiveExactTeaching)
    }

    /// Whether any modeled dimension is weaker than exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest teaching claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5TeachingComponentClaim {
        let mut permitted = self.full_teaching_claim;
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
    pub fn binding_condition(&self) -> Option<&TeachingComponentClaimConditionEntry> {
        let mut binding: Option<(&TeachingComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_teaching_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5TeachingComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The teaching claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5TeachingComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_teaching_claim,
        }
    }

    /// AC / auto-narrowing honesty: a snoozed tip, a partial bridge, an unsupported sequence, or
    /// stale fallback content can no longer keep an old `ExactTeaching` / `ReviewableGuidance`
    /// label. The effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the permitted
    /// ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and teaching lineage. When nothing narrows, no spurious narrow block
    /// is present.
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

    /// AC / exact-teaching honesty: a partial / unsupported / stale state never keeps an exact
    /// teaching claim. When such a state is modeled, the effective claim must not assert
    /// `ExactTeaching`.
    pub fn exact_teaching_honesty_holds(&self) -> bool {
        let has_partial_unsupported_or_stale = self
            .claim_conditions
            .iter()
            .any(|c| c.state.is_partial_unsupported_or_stale());
        !(has_partial_unsupported_or_stale && self.effective_claim().asserts_exact_teaching())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.teaching_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: snoozed-tip, partial-bridge, unsupported-sequence, and stale-fallback states
    /// preserve the underlying teaching lineage. The row must assert `lineage_preserved`, and
    /// any narrow block must preserve lineage continuity too.
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
        M5TeachingRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> TeachingComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.exact_teaching_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return TeachingComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            TeachingComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            TeachingComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.teaching_context_ref.trim().is_empty()
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
            full = self.full_teaching_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-930 contextual-teaching component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_exact_teaching_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`TeachingComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeachingComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<TeachingComponentAccessibilityRow>,
}

/// Checked-in M05-930 contextual-teaching component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<TeachingComponentAccessibilityRow>,
    pub summary: TeachingComponentAccessibilitySummary,
}

impl TeachingComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TeachingComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: TeachingComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_exact_teaching_honesty_holds: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5ContextualTeachingComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5TeachingComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5TeachingComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Teaching claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5TeachingComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5TeachingConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TeachingComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5TeachingConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&TeachingComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                TeachingComponentAccessibilityStatus::Parity => green += 1,
                TeachingComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                TeachingComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        TeachingComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::claim_is_honest),
            all_exact_teaching_honesty_holds: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::exact_teaching_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(TeachingComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TeachingComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(TeachingComponentAccessibilityViolation::SchemaVersion {
                expected: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEACHING_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(TeachingComponentAccessibilityViolation::RecordKind {
                expected: TEACHING_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TeachingComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_partial_unsupported_or_stale_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TeachingComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.is_partial_unsupported_or_stale())
            {
                has_partial_unsupported_or_stale_row = true;
            }

            if !row.is_complete() {
                violations.push(TeachingComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory teaching label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5TeachingComponentFallbackModality::Structured)
            {
                violations.push(
                    TeachingComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts an exact / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(TeachingComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a partial / unsupported / stale state never keeps an exact teaching claim.
            if !row.exact_teaching_honesty_holds() {
                violations.push(
                    TeachingComponentAccessibilityViolation::PartialUnsupportedOrStaleShownAsExact {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    TeachingComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    TeachingComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: snoozed-tip, partial-bridge, unsupported-sequence, and stale-fallback
            // states preserve teaching lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(TeachingComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    TeachingComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == TeachingComponentAccessibilityStatus::Stranded {
                violations.push(TeachingComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ContextualTeachingComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5TeachingComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the exact baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5TeachingComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every teaching claim tier appears as an effective claim, so the full
        // narrowing spectrum (exact-teaching → … → stale-fallback) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5TeachingComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Exact-teaching honesty must be proven with at least one partial / unsupported / stale
        // row in the packet, so the "partial / unsupported / stale never shown as exact"
        // guarantee is exercised end-to-end.
        if !has_partial_unsupported_or_stale_row {
            violations.push(TeachingComponentAccessibilityViolation::ExactTeachingHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the onboarding, tour-overlay,
        // command-palette, migration-report, inline-tip, help-panel, CLI help, product UI, and
        // support / release exports — so every consumer surface is exercised at least once
        // across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5TeachingConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    TeachingComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(TeachingComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("contextual-teaching accessibility fallback packet serializes"),
        ) {
            violations.push(TeachingComponentAccessibilityViolation::RawTeachingMaterialInExport);
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
            .expect("contextual-teaching accessibility fallback packet serializes")
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
                full = row.full_teaching_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Contextual-Teaching Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ContextualTeachingComponentFamily::ALL.len(),
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
                    row.full_teaching_claim.as_str(),
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

/// Reads and validates the checked-in contextual-teaching component accessibility fallback
/// export.
pub fn current_m5_teaching_component_a11y_fallback_export(
) -> Result<TeachingComponentAccessibilityPacket, TeachingComponentAccessibilityArtifactError> {
    let packet: TeachingComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-teaching-component-accessibility-fallback/support_export.json"
    )))
    .map_err(TeachingComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TeachingComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in contextual-teaching component accessibility
/// fallback export.
#[derive(Debug)]
pub enum TeachingComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TeachingComponentAccessibilityViolation>),
}

impl fmt::Display for TeachingComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "contextual-teaching accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "contextual-teaching accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for TeachingComponentAccessibilityArtifactError {}

/// Validation failure for M05-930 contextual-teaching component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeachingComponentAccessibilityViolation {
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
        dimension: M5TeachingComponentClaimDimension,
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
    PartialUnsupportedOrStaleShownAsExact {
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
        family: M5ContextualTeachingComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5TeachingComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5TeachingComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5TeachingComponentClaim,
    },
    ExactTeachingHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5TeachingConsumerSurface,
    },
    SummaryMismatch,
    RawTeachingMaterialInExport,
}

impl fmt::Display for TeachingComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory teaching label")
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
            Self::PartialUnsupportedOrStaleShownAsExact { id } => {
                write!(
                    f,
                    "row {id} shows a partial / unsupported / stale state as exact teaching"
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
                    "row {id} does not preserve teaching lineage across narrowing"
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
                    "teaching claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::ExactTeachingHonestyUnproven => {
                write!(
                    f,
                    "no partial / unsupported / stale row is present to prove the exact-teaching-honesty guarantee"
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
            Self::RawTeachingMaterialInExport => {
                write!(f, "export contains raw teaching material")
            }
        }
    }
}

impl Error for TeachingComponentAccessibilityViolation {}

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
            | "snoozed"
            | "partial"
            | "partial bridge"
            | "stale"
            | "stale fallback"
            | "no binding"
            | "source language"
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

/// Builds the canonical, checked-in contextual-teaching component accessibility fallback packet.
/// This is the one source of truth shared by the tests and the on-disk support export so both
/// stay byte-aligned.
pub fn seeded_m5_teaching_component_a11y_fallback_packet() -> TeachingComponentAccessibilityPacket {
    TeachingComponentAccessibilityPacket::new(TeachingComponentAccessibilityPacketInput {
        packet_id: "m5-contextual-teaching-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:contextual-teaching-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5TeachingRequiredLabel> {
    M5TeachingRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> TeachingComponentCopyExportParity {
    TeachingComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5TeachingComponentClaimDimension,
    state: M5TeachingComponentConditionState,
) -> TeachingComponentClaimConditionEntry {
    TeachingComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// help — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5TeachingConsumerSurface]) -> Vec<M5TeachingConsumerSurface> {
    let mut out = vec![
        M5TeachingConsumerSurface::SupportExport,
        M5TeachingConsumerSurface::CliHelp,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: TeachingComponentNarrowingDisclosureState,
) -> Vec<TeachingComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        TeachingComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TeachingComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        TeachingComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TeachingComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<TeachingComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TeachingComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<TeachingComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TeachingComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5TeachingComponentRenderingSurface> {
    vec![
        M5TeachingComponentRenderingSurface::DesktopFull,
        M5TeachingComponentRenderingSurface::CliHeadless,
        M5TeachingComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<TeachingComponentAccessibilityRow> {
    vec![
        // Contextual tip card (live) — the tip is active, command-backed, and dismissible, so it
        // is fully exact teaching and reachable on every surface (green).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:contextual-tip-card-live".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::ContextualTipCard,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:contextual-tip-card:0001".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:contextual-tip-card-live:a11y".to_owned(),
            copy_export: copy_export(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
                "keyboard_route",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ExactTeaching,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::TipDelivery,
                M5TeachingComponentConditionState::LiveExactTeaching,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::OnboardingUi,
                M5TeachingConsumerSurface::TourOverlayUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §7.9 contextual tip cards".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("contextual-tip-card-live"),
        },
        // Contextual tip card (snoozed) — the tip has been snoozed, so it auto-narrows to a
        // snoozed-tip projection rather than presenting itself as an active live tip, while
        // keeping its identity, trigger, and command binding visible (yellow).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:contextual-tip-card-snoozed".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::ContextualTipCard,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:contextual-tip-card:0002".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:contextual-tip-card-snoozed:a11y".to_owned(),
            copy_export: copy_export(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
                "snooze_state",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ExactTeaching,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::TipDelivery,
                M5TeachingComponentConditionState::TipSnoozed,
            )],
            claim_narrow: Some(TeachingComponentClaimAutoNarrow {
                narrowed_to: M5TeachingComponentClaim::SnoozedTipProjection,
                binding_dimension: M5TeachingComponentClaimDimension::TipDelivery,
                trigger: M5TeachingDowngradeTrigger::TipCommandBindingUnstated,
                narrowed_label:
                    "This tip is snoozed for now — shown as a snoozed-tip projection with its trigger and stable command binding still reachable, never as an active live tip"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "tip_identity",
                "tip_trigger",
                "command_binding",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::InlineTipUi,
                M5TeachingConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §7.9 contextual tip snooze / dismissal".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("contextual-tip-card-snoozed"),
        },
        // Migration bridge card (partial) — the imported behavior maps only partially, so the
        // card auto-narrows to a partial-bridge projection rather than presenting an exact
        // mapping, while keeping the old path, new command, and mapping class visible (yellow).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:migration-bridge-card-partial".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::MigrationBridgeCard,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:migration-bridge-card:0003".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:migration-bridge-card-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "old_path",
                "new_command",
                "mapping_class",
                "keyboard_route",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ExactTeaching,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::MigrationMapping,
                M5TeachingComponentConditionState::BridgePartial,
            )],
            claim_narrow: Some(TeachingComponentClaimAutoNarrow {
                narrowed_to: M5TeachingComponentClaim::PartialBridgeProjection,
                binding_dimension: M5TeachingComponentClaimDimension::MigrationMapping,
                trigger: M5TeachingDowngradeTrigger::MigrationMappingUnstated,
                narrowed_label:
                    "The imported behavior maps only part-way onto Aureline — shown as a partial-bridge projection that names the old path, the new command, and the unmapped edge cases, never as an exact one-to-one mapping"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "old_path",
                "new_command",
                "mapping_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::MigrationReportUi,
                M5TeachingConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.44 migration bridge cards".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("migration-bridge-card-partial"),
        },
        // Sequence-help strip (unsupported) — hierarchy-heavy (nested leader / chord / motion /
        // operator / terminal-action lineage); the entered sequence has no committed binding in
        // this context, so the strip auto-narrows to an unsupported-sequence projection and binds
        // its nested step lineage to a flat list / textual path (yellow).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:sequence-help-strip-unsupported".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::SequenceHelpStrip,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:sequence-help-strip:0004".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::Structured,
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach:
                TeachingComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:sequence-help-strip-unsupported:a11y".to_owned(),
            copy_export: copy_export(&[
                "current_mode",
                "entered_keys",
                "sequence_state",
                "cancel_key",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ExactTeaching,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::SequenceState,
                M5TeachingComponentConditionState::SequenceUnsupported,
            )],
            claim_narrow: Some(TeachingComponentClaimAutoNarrow {
                narrowed_to: M5TeachingComponentClaim::UnsupportedSequenceProjection,
                binding_dimension: M5TeachingComponentClaimDimension::SequenceState,
                trigger: M5TeachingDowngradeTrigger::SequenceHelpStateUnstated,
                narrowed_label:
                    "The entered keys have no bound command in this mode — shown as an unsupported-sequence projection that names the current mode, the entered keys, and the cancel key, never as a ready-to-run command"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "current_mode",
                "entered_keys",
                "sequence_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::CommandPaletteUi,
                M5TeachingConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix AB sequence-help strips".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("sequence-help-strip-unsupported"),
        },
        // Why-unavailable explanation row — the blocked-action owner, reason, and next safe
        // action are all stated and the row is a self-sufficient, reviewable read-only
        // explanation (not itself an exact-teaching path), reachable on every surface (green).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:why-unavailable-explanation-row".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::WhyUnavailableExplanationRow,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:why-unavailable-explanation-row:0005".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:why-unavailable-explanation-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "blocked_action_owner",
                "unavailable_reason",
                "next_safe_action",
                "keyboard_route",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ReviewableGuidance,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::BlockedExplanation,
                M5TeachingComponentConditionState::LiveExactTeaching,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "blocked_action_owner",
                "unavailable_reason",
                "next_safe_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::ProductUi,
                M5TeachingConsumerSurface::HelpPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §23.44 why-unavailable explanation rows".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("why-unavailable-explanation-row"),
        },
        // Source-language fallback (stale) — the localized help content is stale and falling back
        // to source-language text, so the surface auto-narrows to a stale-fallback projection
        // that keeps its canonical citation preserved, never masquerading as authoritative
        // localized-current help (yellow).
        TeachingComponentAccessibilityRow {
            record_kind: TEACHING_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:source-language-fallback-stale".to_owned(),
            component_family: M5ContextualTeachingComponentFamily::SourceLanguageFallback,
            source_family_schema_ref: TEACHING_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            teaching_context_ref: "teaching:source-language-fallback:0006".to_owned(),
            fallback_modalities: vec![
                M5TeachingComponentFallbackModality::List,
                M5TeachingComponentFallbackModality::Textual,
                M5TeachingComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TeachingComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TeachingComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:source-language-fallback-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "display_locale",
                "fallback_state",
                "canonical_citation",
                "keyboard_route",
            ]),
            full_teaching_claim: M5TeachingComponentClaim::ExactTeaching,
            claim_conditions: vec![condition(
                M5TeachingComponentClaimDimension::SourceLanguage,
                M5TeachingComponentConditionState::FallbackStale,
            )],
            claim_narrow: Some(TeachingComponentClaimAutoNarrow {
                narrowed_to: M5TeachingComponentClaim::StaleFallbackProjection,
                binding_dimension: M5TeachingComponentClaimDimension::SourceLanguage,
                trigger: M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated,
                narrowed_label:
                    "The localized help is out of date and falling back to the source language — shown as a stale-fallback projection with its canonical citation preserved, never as authoritative localized-current help"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "display_locale",
                "fallback_state",
                "canonical_citation",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TeachingConsumerSurface::HelpPanelUi,
                M5TeachingConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix AB source-language fallback".to_owned(),
                TEACHING_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("source-language-fallback-stale"),
        },
    ]
}
