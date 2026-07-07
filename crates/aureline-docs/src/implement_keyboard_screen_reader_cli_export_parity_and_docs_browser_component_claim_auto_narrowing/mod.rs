//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for the
//! M5 docs-browser components.
//!
//! This module is the M05-874 accessibility-and-auto-narrowing capstone over the
//! frozen M5 docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`]).
//! Where the freeze matrix defines the reusable docs search bar, scope switcher, docs
//! result row, symbol-linked reference card, docs source / version badge, docs-pack row,
//! stale-example finding row, and browser-handoff banner primitives, and the 869-873
//! implementation lanes resolve their per-surface truth, this lane certifies — per
//! component family — that documentation claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than
//! presenting cached, version-adjacent, mirrored, quarantined, or policy-blocked docs as
//! still a current authoritative reference:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same corpus class, provider / source, version / package scope, symbol anchor,
//!   project-doc override reason, freshness reading, pack pin / mirror / offline /
//!   quarantine state, stale-example status, and browser-handoff reason the rich surface
//!   shows — never a hover-only card that strands assistive-tech or headless users.
//!   Hierarchy-heavy families (the symbol-linked reference card's symbol-anchor tree with
//!   its nested member / signature sub-rows) additionally bind their tree to a flat
//!   list / textual path.
//! - **Export parity.** The support / docs / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot,
//!   preserving the same corpus, provider, version scope, freshness, pack state, and
//!   handoff reason shown in-product.
//! - **Honest auto-narrowing.** When docs freshness, version match, pack verification,
//!   or source / handoff reachability weakens — becoming version-adjacent, cached,
//!   mirrored, unverified, or quarantined — the component's docs-support claim
//!   auto-narrows from `CurrentAuthoritative` / `SupportedReference` to
//!   version-adjacent / cached / unverified / policy-blocked, discloses the narrowing
//!   with a precise trigger and binding dimension, and preserves the canonical corpus /
//!   source / version / symbol / pack / handoff identity rather than silently dropping
//!   it. A component with every dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the docs browser,
//!   help center, AI evidence, onboarding exports, headless CLI, and support / admin
//!   exports so docs / help / onboarding / AI publication stays aligned on
//!   docs-component downgrade behavior rather than drifting in copy — a current-looking
//!   reference can never outrun the freshness / version / pack proof it is being viewed
//!   away from.
//!
//! Each [`DocsBrowserAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::M5DocsBrowserComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5DocsRequiredLabel`] and
//! [`M5DocsDowngradeTrigger`] and the shared [`M5DocsConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to
//! the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw doc bodies, corpus contents, provider credentials,
//! and mirror cursors never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking docs material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, downgrade triggers, and consumer surfaces rather than mint
// parallel ones.
use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsBrowserComponentFamily, M5DocsConsumerSurface, M5DocsDowngradeTrigger,
    M5DocsRequiredLabel,
};

/// Schema version stamped on the M05-874 docs-browser-component accessibility fallback
/// packet.
pub const DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsBrowserAccessibilityPacket`].
pub const DOCS_BROWSER_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_docs_browser_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`DocsBrowserAccessibilityRow`].
pub const DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_docs_browser_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/docs/implement-keyboard-screen-reader-cli-export-parity-and-docs-browser-component-claim-auto-narrowing.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_BROWSER_A11Y_FALLBACK_DOC_REF: &str =
    "docs/docs/m5/implement_keyboard_screen_reader_cli_export_parity_and_docs_browser_component_claim_auto_narrowing.md";

/// Repo-relative path of the frozen docs-browser component matrix this lane certifies.
pub const DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_BROWSER_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-docs-browser-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const DOCS_BROWSER_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DOCS_BROWSER_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DOCS_BROWSER_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the
/// symbol-linked reference card's symbol-anchor tree with its nested member / signature
/// sub-rows) and therefore MUST bind their tree to an equivalent flat list / textual
/// path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5DocsBrowserComponentFamily) -> bool {
    matches!(
        family,
        M5DocsBrowserComponentFamily::SymbolLinkedReferenceCard
    )
}

/// The docs dimension whose weakening a family primarily discloses. Every row must model
/// at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5DocsBrowserComponentFamily) -> M5DocsClaimDimension {
    match family {
        M5DocsBrowserComponentFamily::DocsSearchBar => M5DocsClaimDimension::CorpusReachability,
        M5DocsBrowserComponentFamily::DocsScopeSwitcher => M5DocsClaimDimension::VersionMatch,
        M5DocsBrowserComponentFamily::DocsResultRow => M5DocsClaimDimension::ResultFreshness,
        M5DocsBrowserComponentFamily::SymbolLinkedReferenceCard => {
            M5DocsClaimDimension::SymbolLinkage
        }
        M5DocsBrowserComponentFamily::DocsSourceVersionBadge => {
            M5DocsClaimDimension::SourceProvenance
        }
        M5DocsBrowserComponentFamily::DocsPackRow => M5DocsClaimDimension::PackVerification,
        M5DocsBrowserComponentFamily::StaleExampleFindingRow => M5DocsClaimDimension::ExampleDrift,
        M5DocsBrowserComponentFamily::DocsHandoffBanner => M5DocsClaimDimension::HandoffState,
    }
}

/// A rendered fallback modality for a docs-browser component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsFallbackModality {
    /// A rich, structured (symbol-anchor tree / grouped card) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5DocsFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich,
    /// structured surface (i.e. a keyboard / screen-reader / headless path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the
/// same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRenderingSurface {
    /// The full-capability desktop docs-browser surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin / evaluation export.
    SupportExport,
}

impl M5DocsRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
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
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless
    /// users (red).
    ViewOnlyTrap,
}

impl DocsNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless
    /// users.
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
pub enum DocsExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl DocsExportSummaryState {
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
pub enum DocsNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl DocsNarrowingDisclosureState {
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

/// The docs-support claim ceiling a component asserts: how strong a documentation
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a docs
/// dimension weakens so cached, version-adjacent, or quarantined docs can never keep an
/// old `CurrentAuthoritative` or `SupportedReference` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSupportClaim {
    /// Current authoritative: a fresh, version-matched, provider-verified documentation
    /// reference — the strongest claim.
    CurrentAuthoritative,
    /// Supported reference: a resolved, self-sufficient docs object (a scope switcher or
    /// resolved result) that is not itself a certified-current authoritative claim.
    SupportedReference,
    /// Version-adjacent reference: usable, but drawn from a nearby version / package
    /// scope rather than the exact one requested.
    VersionAdjacentReference,
    /// Cached reference: the content is a cached / mirrored last-known copy, not a live
    /// provider read.
    CachedReference,
    /// Unverified reference: the symbol linkage / source could not be verified; the
    /// content is reconstructed from keyword-fallback or unproven material.
    UnverifiedReference,
    /// Policy-blocked reference: the pack is quarantined or a required policy dependency
    /// is unmet.
    PolicyBlockedReference,
}

impl M5DocsSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::CurrentAuthoritative,
        Self::SupportedReference,
        Self::VersionAdjacentReference,
        Self::CachedReference,
        Self::UnverifiedReference,
        Self::PolicyBlockedReference,
    ];

    /// Capability rank; a higher rank asserts a stronger documentation posture.
    /// Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CurrentAuthoritative => 5,
            Self::SupportedReference => 4,
            Self::VersionAdjacentReference => 3,
            Self::CachedReference => 2,
            Self::UnverifiedReference => 1,
            Self::PolicyBlockedReference => 0,
        }
    }

    /// Returns true when this claim asserts a fresh, current authoritative reference.
    pub const fn asserts_current_authoritative(self) -> bool {
        matches!(self, Self::CurrentAuthoritative)
    }

    /// Returns true when this claim asserts a fully self-sufficient (current
    /// authoritative or resolved / supported) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::CurrentAuthoritative | Self::SupportedReference)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentAuthoritative => "current_authoritative",
            Self::SupportedReference => "supported_reference",
            Self::VersionAdjacentReference => "version_adjacent_reference",
            Self::CachedReference => "cached_reference",
            Self::UnverifiedReference => "unverified_reference",
            Self::PolicyBlockedReference => "policy_blocked_reference",
        }
    }
}

/// The docs dimension whose state governs how far a component may claim to be a current
/// authoritative reference. The four spec axes the lane must auto-narrow on — docs
/// freshness, version match, pack verification, and source / handoff reachability — are
/// [`Self::ResultFreshness`], [`Self::VersionMatch`], [`Self::PackVerification`], and
/// [`Self::HandoffState`]; the remaining dimensions cover the search-bar, symbol-card,
/// and source-badge families' primary weakening axes so every frozen family carries an
/// honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsClaimDimension {
    /// Corpus reachability: is the search bar reaching its live corpus / provider, or a
    /// mirrored / cached fallback?
    CorpusReachability,
    /// Version match: does the scope switcher's version / package scope match the one
    /// requested, or a nearby / adjacent one?
    VersionMatch,
    /// Result freshness: is the result row's content a live provider read, or a cached /
    /// mirrored / stale copy?
    ResultFreshness,
    /// Symbol linkage: is the reference card's symbol anchor exactly resolved, or a
    /// nearby / keyword-fallback / unresolved link?
    SymbolLinkage,
    /// Source provenance: is the source / version badge's provider verified and current,
    /// or masked / version-adjacent?
    SourceProvenance,
    /// Pack verification: is the docs-pack pinned and verified, or mirrored / offline /
    /// quarantined?
    PackVerification,
    /// Example drift: is the example current against its source, or drifted / stale?
    ExampleDrift,
    /// Handoff state: is the browser-handoff banner's return-path source reachable and
    /// current, or stale / unreachable?
    HandoffState,
}

impl M5DocsClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CorpusReachability,
        Self::VersionMatch,
        Self::ResultFreshness,
        Self::SymbolLinkage,
        Self::SourceProvenance,
        Self::PackVerification,
        Self::ExampleDrift,
        Self::HandoffState,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5DocsDowngradeTrigger {
        match self {
            Self::CorpusReachability => M5DocsDowngradeTrigger::CorpusClassUnstated,
            Self::VersionMatch => M5DocsDowngradeTrigger::VersionScopeUnstated,
            Self::ResultFreshness => M5DocsDowngradeTrigger::FreshnessHidden,
            Self::SymbolLinkage => M5DocsDowngradeTrigger::SymbolAnchorUnresolvedHidden,
            Self::SourceProvenance => M5DocsDowngradeTrigger::SourceProviderMasked,
            Self::PackVerification => M5DocsDowngradeTrigger::PackStateMisrepresented,
            Self::ExampleDrift => M5DocsDowngradeTrigger::StaleExampleShownAsCurrent,
            Self::HandoffState => M5DocsDowngradeTrigger::HandoffReasonUnstated,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusReachability => "corpus_reachability",
            Self::VersionMatch => "version_match",
            Self::ResultFreshness => "result_freshness",
            Self::SymbolLinkage => "symbol_linkage",
            Self::SourceProvenance => "source_provenance",
            Self::PackVerification => "pack_verification",
            Self::ExampleDrift => "example_drift",
            Self::HandoffState => "handoff_state",
        }
    }
}

/// The observed condition of one docs dimension. Anything weaker than [`Self::Current`]
/// imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsConditionState {
    /// Fully current / exact / verified — imposes no ceiling.
    Current,
    /// Version-adjacent — a nearby version / package scope; support drops to
    /// version-adjacent.
    Adjacent,
    /// Cached / mirrored — the content is a last-known copy, not a live read; support
    /// drops to cached.
    Cached,
    /// Unverified — the symbol linkage / source could not be proven; support drops to
    /// unverified.
    Unverified,
    /// Quarantined — the pack is quarantined or a required policy dependency is unmet;
    /// support drops to policy-blocked.
    Quarantined,
}

impl M5DocsConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Adjacent,
        Self::Cached,
        Self::Unverified,
        Self::Quarantined,
    ];

    /// Returns true when the dimension is weaker than current and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// The strongest docs-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5DocsSupportClaim {
        match self {
            Self::Current => M5DocsSupportClaim::CurrentAuthoritative,
            Self::Adjacent => M5DocsSupportClaim::VersionAdjacentReference,
            Self::Cached => M5DocsSupportClaim::CachedReference,
            Self::Unverified => M5DocsSupportClaim::UnverifiedReference,
            Self::Quarantined => M5DocsSupportClaim::PolicyBlockedReference,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Adjacent => "adjacent",
            Self::Cached => "cached",
            Self::Unverified => "unverified",
            Self::Quarantined => "quarantined",
        }
    }
}

/// One docs dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5DocsClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5DocsConditionState,
}

/// An honest docs-support-claim auto-narrow block. When a docs dimension weakens, the
/// component's support claim lowers to the permitted ceiling, names the binding
/// dimension and frozen trigger, and preserves the canonical corpus / source / version /
/// symbol / pack / handoff identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5DocsSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5DocsClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5DocsDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical corpus, provider, version scope, symbol anchor, pack state, and
    /// handoff reason are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl DocsClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl DocsCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as
    /// the sole export.
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
pub struct DocsRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5DocsRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: DocsNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a docs accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops state
    /// silently (red).
    Stranded,
}

impl DocsAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one docs-browser component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserAccessibilityRow {
    /// Record kind; must equal [`DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5DocsBrowserComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the docs / corpus / result / pack / handoff context this component
    /// acts on; stays visible on every surface, so this is never empty.
    pub docs_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5DocsFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical corpus, source, version,
    /// pack, and handoff truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: DocsNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: DocsNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: DocsNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: DocsExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: DocsCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5DocsSupportClaim,
    /// The observed condition of each modeled docs dimension.
    #[serde(default)]
    pub claim_conditions: Vec<DocsClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<DocsClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5DocsRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<DocsRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5DocsRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl DocsBrowserAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Current` when the row does
    /// not model that dimension.
    pub fn condition_for(&self, dimension: M5DocsClaimDimension) -> M5DocsConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5DocsConditionState::Current)
    }

    /// Whether any modeled dimension is weaker than current.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5DocsSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5DocsClaimDimension> {
        let mut binding: Option<(M5DocsClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5DocsSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: cached, version-adjacent, or quarantined docs can no
    /// longer keep an old `CurrentAuthoritative` / `SupportedReference` label. The
    /// effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity. When nothing narrows, no spurious
    /// narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a
    /// screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.docs_context_ref.trim().is_empty()
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

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component
    /// carries an honest claim narrow.
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

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its
    /// reduced interactivity and keeps its labels, so docs / help / onboarding / AI
    /// publication stay aligned on the same narrowed state.
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
        // Every disclosure never silently drops and preserves labels on a narrowed
        // surface.
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
        M5DocsRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> DocsAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return DocsAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            DocsAccessibilityStatus::NarrowedDisclosed
        } else {
            DocsAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.docs_context_ref.trim().is_empty()
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
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-874 docs-browser-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`DocsBrowserAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsBrowserAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<DocsBrowserAccessibilityRow>,
}

/// Checked-in M05-874 docs-browser-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<DocsBrowserAccessibilityRow>,
    pub summary: DocsBrowserAccessibilitySummary,
}

impl DocsBrowserAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: DocsBrowserAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: DocsBrowserAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5DocsBrowserComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5DocsClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5DocsSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5DocsConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DocsBrowserAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5DocsConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&DocsBrowserAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                DocsAccessibilityStatus::Parity => green += 1,
                DocsAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                DocsAccessibilityStatus::Stranded => red += 1,
            }
        }

        DocsBrowserAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(DocsBrowserAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(DocsBrowserAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(DocsBrowserAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(DocsBrowserAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DocsBrowserAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(DocsBrowserAccessibilityViolation::SchemaVersion {
                expected: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DOCS_BROWSER_A11Y_FALLBACK_RECORD_KIND {
            violations.push(DocsBrowserAccessibilityViolation::RecordKind {
                expected: DOCS_BROWSER_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DocsBrowserAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DocsBrowserAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(DocsBrowserAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(DocsBrowserAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory docs label.
            if !row.preserves_mandatory_labels() {
                violations.push(DocsBrowserAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5DocsFallbackModality::Structured)
            {
                violations.push(
                    DocsBrowserAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a current authoritative / supported reference
            // for a weakened one.
            if !row.claim_is_honest() {
                violations.push(DocsBrowserAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(DocsBrowserAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    DocsBrowserAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    DocsBrowserAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(DocsBrowserAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == DocsAccessibilityStatus::Stranded {
                violations.push(DocsBrowserAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5DocsBrowserComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(DocsBrowserAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5DocsClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    DocsBrowserAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (current authoritative → … → policy-blocked) is proven
        // end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5DocsSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(DocsBrowserAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach the docs browser, help
        // center, AI evidence, onboarding, CLI, and support / admin exports — so every
        // consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5DocsConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    DocsBrowserAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DocsBrowserAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("docs browser accessibility fallback packet serializes"),
        ) {
            violations.push(DocsBrowserAccessibilityViolation::RawDocsMaterialInExport);
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
            .expect("docs browser accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for docs / support handoff.
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
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Docs-Browser-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5DocsBrowserComponentFamily::ALL.len(),
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
                    row.full_support_claim.as_str(),
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

/// Reads and validates the checked-in docs-browser-component accessibility fallback
/// export.
pub fn current_m5_docs_browser_a11y_fallback_export(
) -> Result<DocsBrowserAccessibilityPacket, DocsBrowserAccessibilityArtifactError> {
    let packet: DocsBrowserAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/support_export.json"
    )))
    .map_err(DocsBrowserAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DocsBrowserAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in docs-browser-component accessibility
/// fallback export.
#[derive(Debug)]
pub enum DocsBrowserAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DocsBrowserAccessibilityViolation>),
}

impl fmt::Display for DocsBrowserAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "docs browser accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "docs browser accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for DocsBrowserAccessibilityArtifactError {}

/// Validation failure for M05-874 docs-browser-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocsBrowserAccessibilityViolation {
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
        dimension: M5DocsClaimDimension,
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
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
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
        family: M5DocsBrowserComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5DocsClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5DocsSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5DocsConsumerSurface,
    },
    SummaryMismatch,
    RawDocsMaterialInExport,
}

impl fmt::Display for DocsBrowserAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory docs label")
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
                    "row {id} over-asserts a current authoritative / supported reference for a weakened one, or narrows spuriously"
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
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
                    claim.as_str()
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
            Self::RawDocsMaterialInExport => write!(f, "export contains raw docs material"),
        }
    }
}

impl Error for DocsBrowserAccessibilityViolation {}

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
            | "stale"
            | "cached"
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

/// Builds the canonical, checked-in docs-browser-component accessibility fallback packet.
/// This is the one source of truth shared by the tests, the example dump, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_docs_browser_a11y_fallback_packet() -> DocsBrowserAccessibilityPacket {
    DocsBrowserAccessibilityPacket::new(DocsBrowserAccessibilityPacketInput {
        packet_id: "m5-docs-browser-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:docs-browser-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5DocsRequiredLabel> {
    M5DocsRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> DocsCopyExportParity {
    DocsCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5DocsClaimDimension,
    state: M5DocsConditionState,
) -> DocsClaimConditionEntry {
    DocsClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / admin export and
/// CLI inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5DocsConsumerSurface]) -> Vec<M5DocsConsumerSurface> {
    let mut out = vec![
        M5DocsConsumerSurface::SupportExport,
        M5DocsConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: DocsNarrowingDisclosureState,
) -> Vec<DocsRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        DocsRenderingNarrowingDisclosure {
            rendering_surface: M5DocsRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        DocsRenderingNarrowingDisclosure {
            rendering_surface: M5DocsRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<DocsRenderingNarrowingDisclosure> {
    surface_disclosures(labels, DocsNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<DocsRenderingNarrowingDisclosure> {
    surface_disclosures(labels, DocsNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5DocsRenderingSurface> {
    vec![
        M5DocsRenderingSurface::DesktopFull,
        M5DocsRenderingSurface::CliHeadless,
        M5DocsRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<DocsBrowserAccessibilityRow> {
    vec![
        // Docs search bar — the corpus / provider is reachable live, so the search bar
        // carries a fully current authoritative corpus scope and is reachable on every
        // surface (green).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-search-bar".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsSearchBar,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "search:corpus:0001".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-search-bar:a11y".to_owned(),
            copy_export: copy_export(&[
                "corpus_class",
                "provider",
                "retrieval_mode",
                "cached_live_state",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::CorpusReachability,
                M5DocsConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["corpus_class", "provider", "retrieval_mode"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::DocsBrowserUi,
                M5DocsConsumerSurface::HelpAbout,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.8 docs search-bar rules".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-search-bar"),
        },
        // Docs scope switcher — the requested version / package scope matches exactly, so
        // the switcher carries a supported, self-sufficient scope with no version drift
        // (green).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-scope-switcher".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsScopeSwitcher,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "scope:version:0002".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-scope-switcher:a11y".to_owned(),
            copy_export: copy_export(&[
                "version_scope",
                "package_scope",
                "active_scope",
                "scope_source",
            ]),
            full_support_claim: M5DocsSupportClaim::SupportedReference,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::VersionMatch,
                M5DocsConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["version_scope", "package_scope", "active_scope"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::SearchPalette,
                M5DocsConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD §7.3.6 docs browser source-version truth".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-scope-switcher"),
        },
        // Docs result row — the content is a cached / mirrored last-known copy rather
        // than a live provider read, so the result auto-narrows to a cached reference
        // until the live read lands (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-result-row".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsResultRow,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "result:row:0003".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-result-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "result_kind",
                "match_state",
                "freshness",
                "rank_reason",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::ResultFreshness,
                M5DocsConditionState::Cached,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::CachedReference,
                binding_dimension: M5DocsClaimDimension::ResultFreshness,
                trigger: M5DocsDowngradeTrigger::FreshnessHidden,
                narrowed_label:
                    "Result shown from a cached / mirrored copy — not a live provider read until the corpus refreshes"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["result_kind", "match_state", "freshness"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::HoverPeek,
                M5DocsConsumerSurface::AiContextPanel,
            ]),
            source_refs: vec![
                "TDD §7.3.7 docs-integrity result freshness".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-result-row"),
        },
        // Symbol-linked reference card — hierarchy-heavy (symbol-anchor tree with nested
        // member / signature sub-rows); the symbol anchor could only be resolved by
        // keyword fallback, so the card auto-narrows to an unverified reference and binds
        // its tree to a flat list / textual path (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:symbol-linked-reference-card".to_owned(),
            component_family: M5DocsBrowserComponentFamily::SymbolLinkedReferenceCard,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "symbol:card:0004".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::Structured,
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:symbol-linked-reference-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "symbol_anchor",
                "linkage_strength",
                "code_anchor",
                "cited_revision",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::SymbolLinkage,
                M5DocsConditionState::Unverified,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::UnverifiedReference,
                binding_dimension: M5DocsClaimDimension::SymbolLinkage,
                trigger: M5DocsDowngradeTrigger::SymbolAnchorUnresolvedHidden,
                narrowed_label:
                    "Symbol anchor resolved by keyword fallback only — reference shown unverified, not linked to the exact symbol"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["symbol_anchor", "linkage_strength", "code_anchor"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::OnboardingTour,
                M5DocsConsumerSurface::DocsBrowserUi,
            ]),
            source_refs: vec![
                "TAD Appendix BR docs-browser result / symbol linkage rules".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("symbol-linked-reference-card"),
        },
        // Docs source / version badge — the provider is only resolvable at a nearby /
        // version-adjacent scope, so the badge auto-narrows to a version-adjacent
        // reference rather than showing the exact-version provider (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-source-version-badge".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsSourceVersionBadge,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "badge:source-version:0005".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-source-version-badge:a11y".to_owned(),
            copy_export: copy_export(&[
                "source_provider",
                "version_scope",
                "freshness",
                "badge_class",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::SourceProvenance,
                M5DocsConditionState::Adjacent,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::VersionAdjacentReference,
                binding_dimension: M5DocsClaimDimension::SourceProvenance,
                trigger: M5DocsDowngradeTrigger::SourceProviderMasked,
                narrowed_label:
                    "Source resolvable only at a nearby version — badge shown version-adjacent, not the exact-version provider"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["source_provider", "version_scope", "freshness"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::AdminConsole,
                M5DocsConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD §8.21 docs source-version badge truth".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-source-version-badge"),
        },
        // Docs-pack row — the pack is quarantined pending re-verification, so the row
        // auto-narrows to a policy-blocked reference rather than presenting a trusted,
        // pinned pack (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-pack-row".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsPackRow,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "pack:row:0006".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-pack-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "pack_state",
                "trust_posture",
                "pin_state",
                "offline_state",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::PackVerification,
                M5DocsConditionState::Quarantined,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::PolicyBlockedReference,
                binding_dimension: M5DocsClaimDimension::PackVerification,
                trigger: M5DocsDowngradeTrigger::PackStateMisrepresented,
                narrowed_label:
                    "Pack quarantined pending re-verification — shown policy-blocked, not a trusted pinned pack"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["pack_state", "trust_posture", "pin_state"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::AdminConsole,
                M5DocsConsumerSurface::DocsBrowserUi,
            ]),
            source_refs: vec![
                "TDD Appendix BA docs-pack lifecycle".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-pack-row"),
        },
        // Stale-example finding row — the example drifted from its source and is a
        // cached / last-known snapshot, so the row auto-narrows to a cached reference
        // rather than presenting the example as current (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:stale-example-finding-row".to_owned(),
            component_family: M5DocsBrowserComponentFamily::StaleExampleFindingRow,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "stale-example:finding:0007".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:stale-example-finding-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "stale_example_status",
                "drift_posture",
                "anchored_version",
                "compare_action",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::ExampleDrift,
                M5DocsConditionState::Cached,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::CachedReference,
                binding_dimension: M5DocsClaimDimension::ExampleDrift,
                trigger: M5DocsDowngradeTrigger::StaleExampleShownAsCurrent,
                narrowed_label:
                    "Example drifted from its source — shown from a cached snapshot anchored to an older version, not current"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["stale_example_status", "drift_posture", "anchored_version"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::SearchPalette,
                M5DocsConsumerSurface::HelpAbout,
            ]),
            source_refs: vec![
                "TDD Appendix BA stale-example matrices".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("stale-example-finding-row"),
        },
        // Browser-handoff banner — the return-path source could not be verified as
        // reachable and current, so the banner auto-narrows to an unverified reference
        // rather than presenting a live, context-preserved return path (yellow).
        DocsBrowserAccessibilityRow {
            record_kind: DOCS_BROWSER_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:docs-handoff-banner".to_owned(),
            component_family: M5DocsBrowserComponentFamily::DocsHandoffBanner,
            source_family_schema_ref: DOCS_BROWSER_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            docs_context_ref: "handoff:banner:0008".to_owned(),
            fallback_modalities: vec![
                M5DocsFallbackModality::List,
                M5DocsFallbackModality::Textual,
                M5DocsFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: DocsNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: DocsNonVisualReachState::ReachableAndLabeled,
            cli_reach: DocsNonVisualReachState::ReachableAndLabeled,
            export_summary: DocsExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:docs-handoff-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "handoff_reason",
                "destination",
                "privacy_consequence",
                "return_path",
            ]),
            full_support_claim: M5DocsSupportClaim::CurrentAuthoritative,
            claim_conditions: vec![condition(
                M5DocsClaimDimension::HandoffState,
                M5DocsConditionState::Unverified,
            )],
            claim_narrow: Some(DocsClaimAutoNarrow {
                narrowed_to: M5DocsSupportClaim::UnverifiedReference,
                binding_dimension: M5DocsClaimDimension::HandoffState,
                trigger: M5DocsDowngradeTrigger::HandoffReasonUnstated,
                narrowed_label:
                    "Handoff return-path source unverified — shown unverified until the destination reachability is re-confirmed"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["handoff_reason", "destination", "return_path"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DocsConsumerSurface::OnboardingTour,
                M5DocsConsumerSurface::AiContextPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.8 browser-handoff banner rules".to_owned(),
                DOCS_BROWSER_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-06T00:00:00Z".to_owned(),
            evidence_refs: ev("docs-handoff-banner"),
        },
    ]
}
