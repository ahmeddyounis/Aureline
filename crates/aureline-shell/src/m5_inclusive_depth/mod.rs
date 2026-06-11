//! Accessibility, IME / bidi, pseudolocalization, locale-fallback, and
//! translated-help qualification audit for the M5 depth surfaces.
//!
//! The M5 depth lanes ship new panes — notebook cells, result-grid rows,
//! pipeline and log views, profiler timelines, guided tours, docs/help panes,
//! companion surfaces, query consoles, preview-route panes, glossary panels,
//! and support packets — that are easy to certify for a sighted mouse user on
//! an English, left-to-right, single-script happy path, then quietly become
//! expert-only depth islands the moment a keyboard user, an assistive-tech
//! user, an IME user, a right-to-left locale, or a localized profile reaches
//! them. This module carries the stable v1 accessibility and localization
//! contract forward into those lanes: every marketed M5 surface MUST stay a
//! first-class citizen for keyboard users, screen-reader users, high-zoom
//! users, IME users, and localized profiles, and it MUST never let a rich,
//! structured, or translated content path silently corrupt text, drop
//! narration, or narrow a localized row to English-only guidance.
//!
//! The audit projects, for each registered M5 surface, the canonical inclusive
//! descriptor against the qualification result the surface actually certifies
//! for each of the nine inclusive scenario rows the M5 lanes must pass:
//!
//! - `keyboard_reachability`
//! - `screen_reader_narration`
//! - `high_zoom`
//! - `ime_composition`
//! - `grapheme_correctness`
//! - `bidi_direction`
//! - `pseudolocalization`
//! - `locale_fallback`
//! - `translated_help_parity`
//!
//! The resulting [`M5InclusiveDepthReport`] is the canonical truth object for
//! the M5 accessibility-and-locale qualification lane. It is consumed by:
//!
//! - the live shell accessibility/locale inspector (so the in-product audit
//!   quotes the same per-row findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_inclusive_depth`), which is the
//!   only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/a11y/m5_ime_bidi_pseudoloc/`;
//! - the support-export wrapper that lets a reviewer pivot from a support case
//!   to the row that flagged a stale or red inclusive result;
//! - the markdown audit under
//!   `artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md` (rendered
//!   from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which
//!   ingest the audit directly when qualifying or narrowing a marketed M5 row
//!   whose accessibility or localized evidence is stale or red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 surface must declare a qualification binding for each
//!    of the nine inclusive scenario rows.
//! 2. Every surface must carry a canonical locale/narration anchor, a non-empty
//!    inclusive note, at least one claimed locale, and a flag asserting it rides
//!    the shared inclusive-conformance harness; a missing anchor, missing note,
//!    no claimed locale, or a surface that drives its own accessibility/locale
//!    path outside the harness is a blocker.
//! 3. A qualified row must carry the captured evidence the row requires — an
//!    evidence pack, keyboard reachability, screen-reader narration, focus
//!    visibility, and text correctness for every row; an IME-composition result
//!    on the IME row; a bidi-isolation result on the bidi row; a zoom-reflow
//!    result on the high-zoom row; and a locale-parity result on the
//!    pseudolocalization, locale-fallback, and translated-help rows. A red
//!    result (a keyboard trap, silent or misannounced narration, a hidden focus
//!    indicator, corrupted text, broken IME composition, leaking bidi, clipped
//!    zoom content, a lost locale parity, or a hidden suspicious-content cue) is
//!    a blocker.
//! 4. A surface that drives a row through an ad-hoc local accessibility/locale
//!    path outside the shared inclusive-conformance harness
//!    (`unqualified_local_a11y_path`), and a marketed row claimed with no
//!    captured evidence (`missing_evidence`), are blockers.
//! 5. Stale inclusive evidence on a marketed row is a blocker, so release
//!    tooling can narrow a marketed M5 row on the affected locales instead of
//!    shipping it as implicitly accessible or implicitly localized.
//! 6. At least one surface must qualify each of the nine scenario rows so the
//!    audit cannot regress into a sighted-mouse, English-only, single-script
//!    happy-path view.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/a11y/m5_ime_bidi_pseudoloc/` are bit-for-bit equal
//! to the seeded report returned by [`seeded_m5_inclusive_depth_audit`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every M5 inclusive-depth record.
pub const M5_INCLUSIVE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every M5 inclusive-depth row.
pub const M5_INCLUSIVE_SHARED_CONTRACT_REF: &str = "shell:m5_inclusive_depth:v1";

/// Stable record kind for [`M5InclusiveDepthReport`] payloads.
pub const M5_INCLUSIVE_REPORT_RECORD_KIND: &str = "shell_m5_inclusive_depth_report_record";

/// Stable record kind for [`M5InclusiveQualificationRow`] payloads.
pub const M5_INCLUSIVE_ROW_RECORD_KIND: &str = "shell_m5_inclusive_depth_row_record";

/// Stable record kind for [`M5InclusiveSupportExport`] payloads.
pub const M5_INCLUSIVE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_inclusive_depth_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_INCLUSIVE_REPORT_ID: &str = "shell:m5_inclusive_depth:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const M5_INCLUSIVE_SUPPORT_EXPORT_ID: &str = "support-export:m5-inclusive-depth:001";

/// Source schema ref for the canonical inclusive-qualification contract.
pub const M5_INCLUSIVE_SOURCE_SCHEMA_REF: &str = "schemas/a11y/m5-depth-qualification.schema.json";

/// Path of the published markdown audit artifact.
pub const M5_INCLUSIVE_PUBLISHED_REPORT_REF: &str =
    "artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md";

/// Path of the published companion doc.
pub const M5_INCLUSIVE_PUBLISHED_DOC_REF: &str = "docs/m5/accessibility-and-locale-depth.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 depth surface whose inclusive scenario rows the audit qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InclusiveSurfaceFamily {
    /// Notebook cell editor and output.
    NotebookCell,
    /// Data / API result-grid rows.
    ResultGridRow,
    /// Pipeline / log viewer rows.
    PipelineLogView,
    /// Profiler capture timeline.
    ProfilerTimeline,
    /// Guided tour / coachmark surfaces.
    GuidedTour,
    /// Embedded docs / help panes.
    DocsHelpPane,
    /// Companion / cross-device surfaces.
    CompanionSurface,
    /// Query / console session surfaces.
    QueryConsole,
    /// Live preview-route panes.
    PreviewRoutePane,
    /// Translated glossary panels.
    GlossaryPanel,
    /// Incident / support packet surfaces.
    SupportPacket,
}

impl M5InclusiveSurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCell => "notebook_cell",
            Self::ResultGridRow => "result_grid_row",
            Self::PipelineLogView => "pipeline_log_view",
            Self::ProfilerTimeline => "profiler_timeline",
            Self::GuidedTour => "guided_tour",
            Self::DocsHelpPane => "docs_help_pane",
            Self::CompanionSurface => "companion_surface",
            Self::QueryConsole => "query_console",
            Self::PreviewRoutePane => "preview_route_pane",
            Self::GlossaryPanel => "glossary_panel",
            Self::SupportPacket => "support_packet",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookCell => "Notebook cell",
            Self::ResultGridRow => "Result-grid row",
            Self::PipelineLogView => "Pipeline / log view",
            Self::ProfilerTimeline => "Profiler timeline",
            Self::GuidedTour => "Guided tour",
            Self::DocsHelpPane => "Docs / help pane",
            Self::CompanionSurface => "Companion surface",
            Self::QueryConsole => "Query console",
            Self::PreviewRoutePane => "Preview-route pane",
            Self::GlossaryPanel => "Glossary panel",
            Self::SupportPacket => "Support packet",
        }
    }
}

/// A claimed locale the audit certifies a surface on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InclusiveLocale {
    /// English (base locale).
    En,
    /// Arabic (right-to-left, bidi-bearing).
    Ar,
    /// Japanese (IME / CJK grapheme-bearing).
    Ja,
    /// German (long-string / reflow-bearing).
    De,
}

impl M5InclusiveLocale {
    /// Stable schema token (BCP-47 base tag).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ar => "ar",
            Self::Ja => "ja",
            Self::De => "de",
        }
    }

    /// Returns the four claimed locales in canonical order.
    pub const fn all() -> [Self; 4] {
        [Self::En, Self::Ar, Self::Ja, Self::De]
    }
}

/// The inclusive dimension a scenario row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InclusiveDimension {
    /// Interaction (keyboard reachability, narration, high zoom).
    Interaction,
    /// Text correctness (IME composition, grapheme, bidi).
    Text,
    /// Localization (pseudolocalization, locale fallback, translated help).
    Localization,
}

impl M5InclusiveDimension {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interaction => "interaction",
            Self::Text => "text",
            Self::Localization => "localization",
        }
    }
}

/// One of the nine inclusive scenario rows the audit certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InclusiveRow {
    /// The surface is fully reachable and operable by keyboard.
    KeyboardReachability,
    /// The surface is narrated truthfully by assistive technology.
    ScreenReaderNarration,
    /// The surface reflows at high zoom without clipping content.
    HighZoom,
    /// The surface preserves IME composition (pre-edit) without corruption.
    ImeComposition,
    /// The surface renders graphemes (combined clusters, emoji) correctly.
    GraphemeCorrectness,
    /// The surface isolates bidi / mixed-direction content without leakage.
    BidiDirection,
    /// The surface survives pseudolocalization without truncation or breakage.
    Pseudolocalization,
    /// The surface falls back honestly when a locale string is missing.
    LocaleFallback,
    /// The surface keeps translated help/tour/glossary parity with the feature.
    TranslatedHelpParity,
}

impl M5InclusiveRow {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardReachability => "keyboard_reachability",
            Self::ScreenReaderNarration => "screen_reader_narration",
            Self::HighZoom => "high_zoom",
            Self::ImeComposition => "ime_composition",
            Self::GraphemeCorrectness => "grapheme_correctness",
            Self::BidiDirection => "bidi_direction",
            Self::Pseudolocalization => "pseudolocalization",
            Self::LocaleFallback => "locale_fallback",
            Self::TranslatedHelpParity => "translated_help_parity",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::KeyboardReachability => "Keyboard reachability",
            Self::ScreenReaderNarration => "Screen-reader narration",
            Self::HighZoom => "High zoom",
            Self::ImeComposition => "IME composition",
            Self::GraphemeCorrectness => "Grapheme correctness",
            Self::BidiDirection => "Bidi direction",
            Self::Pseudolocalization => "Pseudolocalization",
            Self::LocaleFallback => "Locale fallback",
            Self::TranslatedHelpParity => "Translated-help parity",
        }
    }

    /// Returns the nine required scenario rows in canonical order.
    pub const fn required_rows() -> [Self; 9] {
        [
            Self::KeyboardReachability,
            Self::ScreenReaderNarration,
            Self::HighZoom,
            Self::ImeComposition,
            Self::GraphemeCorrectness,
            Self::BidiDirection,
            Self::Pseudolocalization,
            Self::LocaleFallback,
            Self::TranslatedHelpParity,
        ]
    }

    /// Canonical inclusive dimension this row certifies.
    pub const fn canonical_dimension(self) -> M5InclusiveDimension {
        match self {
            Self::KeyboardReachability | Self::ScreenReaderNarration | Self::HighZoom => {
                M5InclusiveDimension::Interaction
            }
            Self::ImeComposition | Self::GraphemeCorrectness | Self::BidiDirection => {
                M5InclusiveDimension::Text
            }
            Self::Pseudolocalization | Self::LocaleFallback | Self::TranslatedHelpParity => {
                M5InclusiveDimension::Localization
            }
        }
    }

    /// `true` for the row that must prove IME composition survives intact.
    pub const fn requires_ime_composition(self) -> bool {
        matches!(self, Self::ImeComposition)
    }

    /// `true` for the row that must prove bidi content stays isolated.
    pub const fn requires_bidi_isolation(self) -> bool {
        matches!(self, Self::BidiDirection)
    }

    /// `true` for the row that must prove content reflows at high zoom.
    pub const fn requires_zoom_reflow(self) -> bool {
        matches!(self, Self::HighZoom)
    }

    /// `true` for the localization rows that must prove locale parity.
    pub const fn requires_locale_parity(self) -> bool {
        matches!(
            self,
            Self::Pseudolocalization | Self::LocaleFallback | Self::TranslatedHelpParity
        )
    }
}

/// Qualification status a surface reports for one inclusive scenario row.
///
/// Only `Qualified` rows project captured evidence and are drift/red checked.
/// `ExplicitlyNarrowed`, `NotApplicable`, `LocaleOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalA11yPath` (an ad-hoc accessibility or
/// locale path outside the shared harness) and `MissingEvidence` are blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InclusiveQualificationStatus {
    /// The row is qualified with captured inclusive evidence.
    Qualified,
    /// The surface narrows this row; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The row does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// The row is not surfaced for the claimed locales; a reason MUST be set.
    LocaleOmitted,
    /// An extension/provider surface declares a known capture gap honestly;
    /// a reason MUST be set.
    DeclaredCaptureGap,
    /// The surface drives this row through an ad-hoc local accessibility/locale
    /// path outside the shared inclusive-conformance harness. Always a blocker.
    UnqualifiedLocalA11yPath,
    /// A marketed row is claimed with no captured evidence. Always a blocker.
    MissingEvidence,
}

impl M5InclusiveQualificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::LocaleOmitted => "locale_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalA11yPath => "unqualified_local_a11y_path",
            Self::MissingEvidence => "missing_evidence",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::NotApplicable
                | Self::LocaleOmitted
                | Self::DeclaredCaptureGap
        )
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Whether the surface is reachable and operable by keyboard alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KeyboardReachability {
    /// The surface is fully reachable and operable by keyboard.
    Reachable,
    /// The surface traps or strands keyboard focus. Always a blocker.
    Trapped,
    /// The surface exposes no focusable interaction to reach.
    NotApplicable,
}

impl M5KeyboardReachability {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Trapped => "trapped",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether assistive technology narrates the surface truthfully.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5Narration {
    /// The surface is narrated truthfully (role, name, state).
    Narrated,
    /// The surface is silent to assistive technology. Always a blocker.
    Silent,
    /// The surface is narrated with the wrong role, name, or state. Always a
    /// blocker.
    Misannounced,
}

impl M5Narration {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Narrated => "narrated",
            Self::Silent => "silent",
            Self::Misannounced => "misannounced",
        }
    }
}

/// Whether the focus indicator stays visible on the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FocusVisibility {
    /// The focus indicator is visible.
    Visible,
    /// The focus indicator is hidden. Always a blocker.
    Hidden,
}

impl M5FocusVisibility {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
        }
    }
}

/// Whether rendered text stays correct across raw / rendered / escaped paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextCorrectness {
    /// Graphemes, escapes, and decode-recovery paths render correctly.
    Correct,
    /// Text is corrupted (split graphemes, mojibake, mis-escaped). Always a
    /// blocker.
    Corrupted,
}

impl M5TextCorrectness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Correct => "correct",
            Self::Corrupted => "corrupted",
        }
    }
}

/// Whether IME composition (pre-edit) survives the surface intact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ImeComposition {
    /// Composition pre-edit is preserved and committed correctly.
    Preserved,
    /// Composition is broken (pre-edit dropped or committed early). Always a
    /// blocker on the IME row.
    Broken,
    /// The surface accepts no text input to compose into.
    NotApplicable,
}

impl M5ImeComposition {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Broken => "broken",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether bidi / mixed-direction content stays isolated on the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BidiIsolation {
    /// Mixed-direction runs are isolated and ordered correctly.
    Isolated,
    /// Direction leaks across runs and reorders adjacent content. Always a
    /// blocker on the bidi row.
    Leaking,
    /// The surface renders no mixed-direction content.
    NotApplicable,
}

impl M5BidiIsolation {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Isolated => "isolated",
            Self::Leaking => "leaking",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the surface reflows at high zoom without clipping content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ZoomReflow {
    /// Content reflows at high zoom and stays fully reachable.
    Reflowed,
    /// Content is clipped or stranded off-screen at high zoom. Always a blocker
    /// on the high-zoom row.
    Clipped,
    /// The surface carries no content that must reflow.
    NotApplicable,
}

impl M5ZoomReflow {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reflowed => "reflowed",
            Self::Clipped => "clipped",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether a localized row keeps parity with the feature it documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocaleParity {
    /// The localized row stays consistent with the feature packet.
    Parity,
    /// The localized row silently narrows to English-only guidance. Always a
    /// blocker on a localization row.
    SilentEnglishFallback,
    /// The localized row drifted out of date with the feature. Always a blocker
    /// on a localization row.
    Mismatched,
    /// The surface carries no localized content for this row.
    NotApplicable,
}

impl M5LocaleParity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::SilentEnglishFallback => "silent_english_fallback",
            Self::Mismatched => "mismatched",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether a suspicious-content indicator stays visible on a high-salience row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspiciousContentCue {
    /// The suspicious-content indicator is present.
    Present,
    /// The suspicious-content indicator is hidden. Always a blocker.
    Hidden,
    /// The surface carries no suspicious-content indicator.
    NotApplicable,
}

impl M5SuspiciousContentCue {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Hidden => "hidden",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Freshness of the captured inclusive evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed row.
    Stale,
}

impl M5EvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// How much trust, lifecycle, or severity meaning the surface conveys.
///
/// A surface that conveys lifecycle, trust, or severity meaning is
/// "high-salience": no inclusive scenario may hide that meaning, so the audit
/// requires a present suspicious-content cue on every qualified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SemanticSalience {
    /// Purely decorative; carries no semantic meaning.
    DecorativeOnly,
    /// Informational only; no trust, lifecycle, or severity meaning.
    Informational,
    /// Conveys lifecycle state (preview, stale, pending).
    LifecycleBearing,
    /// Conveys trust or identity (companion presence, boundary).
    TrustBearing,
    /// Conveys severity or risk (blocked, destructive, failed).
    SeverityBearing,
}

impl M5SemanticSalience {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DecorativeOnly => "decorative_only",
            Self::Informational => "informational",
            Self::LifecycleBearing => "lifecycle_bearing",
            Self::TrustBearing => "trust_bearing",
            Self::SeverityBearing => "severity_bearing",
        }
    }

    /// `true` for salience classes that must never hide their meaning.
    pub const fn is_high_salience(self) -> bool {
        matches!(
            self,
            Self::LifecycleBearing | Self::TrustBearing | Self::SeverityBearing
        )
    }
}

/// Lifecycle label retained on the canonical surface descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must point at the replacement.
    Deprecated,
}

impl M5SurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Canonical descriptor for one M5 surface's inclusive contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveDescriptor {
    /// Stable surface id (e.g. `surface:notebook.cell`).
    pub surface_id: String,
    /// Surface family the descriptor belongs to.
    pub surface_family: M5InclusiveSurfaceFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical locale/narration anchor ref the audit resolves narration and
    /// localized help from. MUST be non-empty.
    pub locale_anchor_ref: String,
    /// Inclusive note retained on the descriptor. MUST be non-empty.
    pub inclusive_note: String,
    /// Pinned semantic salience.
    pub semantic_salience: M5SemanticSalience,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5SurfaceLifecycle,
    /// Claimed locales. MUST be non-empty.
    pub claimed_locales: Vec<M5InclusiveLocale>,
    /// `true` when the surface is marketed on inclusive scenario rows and
    /// therefore must pass the claimed matrix or narrow accordingly.
    pub marketed_on_inclusive_rows: bool,
    /// `true` once the surface rides the shared inclusive-conformance harness
    /// and does not drive its own accessibility/locale path. MUST be `true`.
    pub registered_on_inclusive_harness: bool,
}

impl M5InclusiveDescriptor {
    /// `true` when this surface's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// Per-row qualification binding a surface reports for one scenario row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveBinding {
    /// Scenario row this binding covers.
    pub row: M5InclusiveRow,
    /// Inclusive dimension projected for the row. MUST equal the row's
    /// canonical dimension.
    pub dimension: M5InclusiveDimension,
    /// Qualification status the surface reports.
    pub qualification_status: M5InclusiveQualificationStatus,
    /// `true` when the surface is marketed on this row.
    pub marketed_on_row: bool,
    /// Captured evidence-pack ref (`None` for non-qualified rows).
    pub projected_evidence_pack_ref: Option<String>,
    /// Captured keyboard reachability (`None` for non-qualified rows).
    pub projected_keyboard_reachability: Option<M5KeyboardReachability>,
    /// Captured screen-reader narration (`None` for non-qualified rows).
    pub projected_narration: Option<M5Narration>,
    /// Captured focus visibility (`None` for non-qualified rows).
    pub projected_focus_visibility: Option<M5FocusVisibility>,
    /// Captured text correctness (`None` for non-qualified rows).
    pub projected_text_correctness: Option<M5TextCorrectness>,
    /// Captured IME composition (`None` unless the row requires it).
    pub projected_ime_composition: Option<M5ImeComposition>,
    /// Captured bidi isolation (`None` unless the row requires it).
    pub projected_bidi_isolation: Option<M5BidiIsolation>,
    /// Captured zoom reflow (`None` unless the row requires it).
    pub projected_zoom_reflow: Option<M5ZoomReflow>,
    /// Captured locale parity (`None` unless the row requires it).
    pub projected_locale_parity: Option<M5LocaleParity>,
    /// Captured suspicious-content cue (`None` for non-qualified rows).
    pub projected_suspicious_content_cue: Option<M5SuspiciousContentCue>,
    /// Freshness of the captured evidence (`None` for non-qualified rows).
    pub evidence_freshness: Option<M5EvidenceFreshness>,
    /// Timestamp the evidence was captured (`None` for non-qualified rows).
    pub evidence_captured_at: Option<String>,
    /// Narrowing reason set when `qualification_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5InclusiveBlockingFinding {
    /// A surface drives a row through an ad-hoc local accessibility/locale path
    /// outside the shared inclusive-conformance harness.
    UnqualifiedLocalA11yPath {
        /// Surface that exposes the gap.
        surface_id: String,
        /// Row that exposes the gap.
        row: M5InclusiveRow,
    },
    /// A marketed row is claimed with no captured evidence.
    MissingEvidence {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A qualified row is missing its captured evidence pack.
    MissingEvidencePack {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A row traps or strands keyboard focus.
    KeyboardUnreachable {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A row is silent to assistive technology.
    NarrationSilent {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A row is narrated with the wrong role, name, or state.
    NarrationMisannounced {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A row hides the focus indicator.
    FocusIndicatorHidden {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A row corrupts text across the raw/rendered/escaped paths.
    TextCorrupted {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// The IME row breaks composition.
    ImeCompositionBroken {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// The bidi row leaks direction across runs.
    BidiLeaking {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// The high-zoom row clips content.
    ZoomContentClipped {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A localization row loses parity with the feature.
    LocaleParityLost {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A high-salience row hides the suspicious-content cue.
    SuspiciousContentHidden {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A marketed row carries stale inclusive evidence.
    StaleEvidenceOnMarketedRow {
        surface_id: String,
        row: M5InclusiveRow,
    },
    /// A binding projects an inclusive dimension that disagrees with the row's
    /// canonical dimension.
    DimensionDrift {
        surface_id: String,
        row: M5InclusiveRow,
        /// Projected dimension.
        projected_dimension: M5InclusiveDimension,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        row: M5InclusiveRow,
        qualification_status: M5InclusiveQualificationStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        surface_id: String,
        row: M5InclusiveRow,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical locale/narration anchor.
    DescriptorMissingLocaleAnchor { surface_id: String },
    /// The descriptor carries no inclusive note.
    MissingInclusiveNote { surface_id: String },
    /// The descriptor claims no locale.
    MissingClaimedLocales { surface_id: String },
    /// The surface drives its own accessibility/locale path outside the shared
    /// inclusive-conformance harness.
    SurfaceNotOnInclusiveHarness { surface_id: String },
}

impl M5InclusiveBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalA11yPath { .. } => "unqualified_local_a11y_path",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingEvidencePack { .. } => "missing_evidence_pack",
            Self::KeyboardUnreachable { .. } => "keyboard_unreachable",
            Self::NarrationSilent { .. } => "narration_silent",
            Self::NarrationMisannounced { .. } => "narration_misannounced",
            Self::FocusIndicatorHidden { .. } => "focus_indicator_hidden",
            Self::TextCorrupted { .. } => "text_corrupted",
            Self::ImeCompositionBroken { .. } => "ime_composition_broken",
            Self::BidiLeaking { .. } => "bidi_leaking",
            Self::ZoomContentClipped { .. } => "zoom_content_clipped",
            Self::LocaleParityLost { .. } => "locale_parity_lost",
            Self::SuspiciousContentHidden { .. } => "suspicious_content_hidden",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::DimensionDrift { .. } => "dimension_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingLocaleAnchor { .. } => "descriptor_missing_locale_anchor",
            Self::MissingInclusiveNote { .. } => "missing_inclusive_note",
            Self::MissingClaimedLocales { .. } => "missing_claimed_locales",
            Self::SurfaceNotOnInclusiveHarness { .. } => "surface_not_on_inclusive_harness",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalA11yPath { surface_id, .. }
            | Self::MissingEvidence { surface_id, .. }
            | Self::MissingEvidencePack { surface_id, .. }
            | Self::KeyboardUnreachable { surface_id, .. }
            | Self::NarrationSilent { surface_id, .. }
            | Self::NarrationMisannounced { surface_id, .. }
            | Self::FocusIndicatorHidden { surface_id, .. }
            | Self::TextCorrupted { surface_id, .. }
            | Self::ImeCompositionBroken { surface_id, .. }
            | Self::BidiLeaking { surface_id, .. }
            | Self::ZoomContentClipped { surface_id, .. }
            | Self::LocaleParityLost { surface_id, .. }
            | Self::SuspiciousContentHidden { surface_id, .. }
            | Self::StaleEvidenceOnMarketedRow { surface_id, .. }
            | Self::DimensionDrift { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingLocaleAnchor { surface_id }
            | Self::MissingInclusiveNote { surface_id }
            | Self::MissingClaimedLocales { surface_id }
            | Self::SurfaceNotOnInclusiveHarness { surface_id } => surface_id,
        }
    }

    /// Returns the row this finding is attached to, when row-scoped.
    pub fn row(&self) -> Option<M5InclusiveRow> {
        match self {
            Self::UnqualifiedLocalA11yPath { row, .. }
            | Self::MissingEvidence { row, .. }
            | Self::MissingEvidencePack { row, .. }
            | Self::KeyboardUnreachable { row, .. }
            | Self::NarrationSilent { row, .. }
            | Self::NarrationMisannounced { row, .. }
            | Self::FocusIndicatorHidden { row, .. }
            | Self::TextCorrupted { row, .. }
            | Self::ImeCompositionBroken { row, .. }
            | Self::BidiLeaking { row, .. }
            | Self::ZoomContentClipped { row, .. }
            | Self::LocaleParityLost { row, .. }
            | Self::SuspiciousContentHidden { row, .. }
            | Self::StaleEvidenceOnMarketedRow { row, .. }
            | Self::DimensionDrift { row, .. }
            | Self::MissingNarrowingReason { row, .. }
            | Self::MissingProjection { row, .. } => Some(*row),
            Self::DescriptorMissingLocaleAnchor { .. }
            | Self::MissingInclusiveNote { .. }
            | Self::MissingClaimedLocales { .. }
            | Self::SurfaceNotOnInclusiveHarness { .. } => None,
        }
    }
}

/// One per-surface inclusive-qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveQualificationRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5InclusiveDescriptor,
    /// Row-by-row qualification bindings, in canonical row order.
    pub bindings: Vec<M5InclusiveBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5InclusiveBlockingFinding>,
    /// `true` when the surface's descriptor classifies it as high-salience.
    pub high_salience: bool,
    /// `true` when the surface is marketed on inclusive scenario rows.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_a11y_path` findings.
    pub unqualified_local_a11y_path: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_evidence_pack` findings.
    pub missing_evidence_pack: usize,
    /// Number of `keyboard_unreachable` findings.
    pub keyboard_unreachable: usize,
    /// Number of `narration_silent` findings.
    pub narration_silent: usize,
    /// Number of `narration_misannounced` findings.
    pub narration_misannounced: usize,
    /// Number of `focus_indicator_hidden` findings.
    pub focus_indicator_hidden: usize,
    /// Number of `text_corrupted` findings.
    pub text_corrupted: usize,
    /// Number of `ime_composition_broken` findings.
    pub ime_composition_broken: usize,
    /// Number of `bidi_leaking` findings.
    pub bidi_leaking: usize,
    /// Number of `zoom_content_clipped` findings.
    pub zoom_content_clipped: usize,
    /// Number of `locale_parity_lost` findings.
    pub locale_parity_lost: usize,
    /// Number of `suspicious_content_hidden` findings.
    pub suspicious_content_hidden: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `dimension_drift` findings.
    pub dimension_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_locale_anchor` findings.
    pub descriptor_missing_locale_anchor: usize,
    /// Number of `missing_inclusive_note` findings.
    pub missing_inclusive_note: usize,
    /// Number of `missing_claimed_locales` findings.
    pub missing_claimed_locales: usize,
    /// Number of `surface_not_on_inclusive_harness` findings.
    pub surface_not_on_inclusive_harness: usize,
}

impl M5InclusiveFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_a11y_path: 0,
            missing_evidence: 0,
            missing_evidence_pack: 0,
            keyboard_unreachable: 0,
            narration_silent: 0,
            narration_misannounced: 0,
            focus_indicator_hidden: 0,
            text_corrupted: 0,
            ime_composition_broken: 0,
            bidi_leaking: 0,
            zoom_content_clipped: 0,
            locale_parity_lost: 0,
            suspicious_content_hidden: 0,
            stale_evidence_on_marketed_row: 0,
            dimension_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_locale_anchor: 0,
            missing_inclusive_note: 0,
            missing_claimed_locales: 0,
            surface_not_on_inclusive_harness: 0,
        }
    }

    fn record(&mut self, finding: &M5InclusiveBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5InclusiveBlockingFinding::UnqualifiedLocalA11yPath { .. } => {
                self.unqualified_local_a11y_path += 1
            }
            M5InclusiveBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5InclusiveBlockingFinding::MissingEvidencePack { .. } => {
                self.missing_evidence_pack += 1
            }
            M5InclusiveBlockingFinding::KeyboardUnreachable { .. } => {
                self.keyboard_unreachable += 1
            }
            M5InclusiveBlockingFinding::NarrationSilent { .. } => self.narration_silent += 1,
            M5InclusiveBlockingFinding::NarrationMisannounced { .. } => {
                self.narration_misannounced += 1
            }
            M5InclusiveBlockingFinding::FocusIndicatorHidden { .. } => {
                self.focus_indicator_hidden += 1
            }
            M5InclusiveBlockingFinding::TextCorrupted { .. } => self.text_corrupted += 1,
            M5InclusiveBlockingFinding::ImeCompositionBroken { .. } => {
                self.ime_composition_broken += 1
            }
            M5InclusiveBlockingFinding::BidiLeaking { .. } => self.bidi_leaking += 1,
            M5InclusiveBlockingFinding::ZoomContentClipped { .. } => self.zoom_content_clipped += 1,
            M5InclusiveBlockingFinding::LocaleParityLost { .. } => self.locale_parity_lost += 1,
            M5InclusiveBlockingFinding::SuspiciousContentHidden { .. } => {
                self.suspicious_content_hidden += 1
            }
            M5InclusiveBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5InclusiveBlockingFinding::DimensionDrift { .. } => self.dimension_drift += 1,
            M5InclusiveBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5InclusiveBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5InclusiveBlockingFinding::DescriptorMissingLocaleAnchor { .. } => {
                self.descriptor_missing_locale_anchor += 1
            }
            M5InclusiveBlockingFinding::MissingInclusiveNote { .. } => {
                self.missing_inclusive_note += 1
            }
            M5InclusiveBlockingFinding::MissingClaimedLocales { .. } => {
                self.missing_claimed_locales += 1
            }
            M5InclusiveBlockingFinding::SurfaceNotOnInclusiveHarness { .. } => {
                self.surface_not_on_inclusive_harness += 1
            }
        }
    }
}

/// Per-row coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveCoverageSummary {
    /// Row this summary covers.
    pub row: M5InclusiveRow,
    /// Number of `qualified` rows on this scenario row.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this scenario row.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this scenario row.
    pub not_applicable_rows: usize,
    /// Number of `locale_omitted` rows on this scenario row.
    pub locale_omitted_rows: usize,
    /// Number of `declared_capture_gap` rows on this scenario row.
    pub declared_capture_gap_rows: usize,
    /// Number of `unqualified_local_a11y_path` rows on this scenario row.
    pub unqualified_local_a11y_path_rows: usize,
    /// Number of `missing_evidence` rows on this scenario row.
    pub missing_evidence_rows: usize,
}

impl M5InclusiveCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.locale_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single locale-anchor index entry the audit publishes so accessibility QA,
/// docs, and release surfaces can resolve narration and localized help for each
/// M5 surface by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveLocaleAnchorEntry {
    /// Surface family the anchor belongs to.
    pub surface_family: M5InclusiveSurfaceFamily,
    /// Surface id the anchor resolves.
    pub surface_id: String,
    /// Canonical locale/narration anchor ref.
    pub locale_anchor_ref: String,
}

/// One marketed row release tooling should narrow because its inclusive
/// evidence is stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveNarrowableRow {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Scenario row that must narrow.
    pub row: M5InclusiveRow,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 accessibility-and-locale qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveDepthReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required scenario rows, in canonical order.
    pub required_rows: Vec<M5InclusiveRow>,
    /// Union of claimed locales across all surfaces, sorted.
    pub claimed_locales: Vec<M5InclusiveLocale>,
    /// Per-surface qualification rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5InclusiveQualificationRow>,
    /// Per-row coverage summary, in canonical row order.
    pub row_coverage: Vec<M5InclusiveCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5InclusiveFindingSummary,
    /// Canonical locale-anchor index, sorted by surface id.
    pub locale_anchor_index: Vec<M5InclusiveLocaleAnchorEntry>,
    /// Number of registered M5 surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-salience surfaces present.
    pub high_salience_surface_count: usize,
    /// Number of surfaces marketed on inclusive scenario rows.
    pub marketed_surface_count: usize,
    /// Total scenario rows checked.
    pub inclusive_rows_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5InclusiveNarrowableRow>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5InclusiveDepthReport {
    /// Returns `true` when every required row is qualified by at least one
    /// surface.
    pub fn every_required_row_qualified(&self) -> bool {
        for row in M5InclusiveRow::required_rows() {
            let any_qualified = self.rows.iter().any(|surface| {
                surface.bindings.iter().any(|binding| {
                    binding.row == row
                        && binding.qualification_status == M5InclusiveQualificationStatus::Qualified
                })
            });
            if !any_qualified {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: surfaces={}, high_salience={}, marketed={}, rows={}, blocking={}, clean={}",
            self.registered_surface_count,
            self.high_salience_surface_count,
            self.marketed_surface_count,
            self.inclusive_rows_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.row_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_a11y_path_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for surface in &self.rows {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding
                        .row()
                        .map(M5InclusiveRow::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.surface_id,
                narrowable.row.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 accessibility and locale qualification audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_inclusive_depth`](../../../../crates/aureline-shell/src/m5_inclusive_depth/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- report-md > \\\n  artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed locales: {}\n",
            self.claimed_locales
                .iter()
                .map(|locale| format!("`{}`", locale.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered M5 surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-salience surfaces: `{}`\n",
            self.high_salience_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Inclusive rows checked: `{}`\n",
            self.inclusive_rows_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed rows: `{}`\n",
            self.narrowable_marketed_rows.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-row coverage\n\n");
        out.push_str(
            "| Scenario row | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | ------------ | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.row_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_a11y_path_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_a11y_path` | {} |\n",
            self.findings_summary.unqualified_local_a11y_path
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_evidence_pack` | {} |\n",
            self.findings_summary.missing_evidence_pack
        ));
        out.push_str(&format!(
            "| `keyboard_unreachable` | {} |\n",
            self.findings_summary.keyboard_unreachable
        ));
        out.push_str(&format!(
            "| `narration_silent` | {} |\n",
            self.findings_summary.narration_silent
        ));
        out.push_str(&format!(
            "| `narration_misannounced` | {} |\n",
            self.findings_summary.narration_misannounced
        ));
        out.push_str(&format!(
            "| `focus_indicator_hidden` | {} |\n",
            self.findings_summary.focus_indicator_hidden
        ));
        out.push_str(&format!(
            "| `text_corrupted` | {} |\n",
            self.findings_summary.text_corrupted
        ));
        out.push_str(&format!(
            "| `ime_composition_broken` | {} |\n",
            self.findings_summary.ime_composition_broken
        ));
        out.push_str(&format!(
            "| `bidi_leaking` | {} |\n",
            self.findings_summary.bidi_leaking
        ));
        out.push_str(&format!(
            "| `zoom_content_clipped` | {} |\n",
            self.findings_summary.zoom_content_clipped
        ));
        out.push_str(&format!(
            "| `locale_parity_lost` | {} |\n",
            self.findings_summary.locale_parity_lost
        ));
        out.push_str(&format!(
            "| `suspicious_content_hidden` | {} |\n",
            self.findings_summary.suspicious_content_hidden
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_row` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_row
        ));
        out.push_str(&format!(
            "| `dimension_drift` | {} |\n",
            self.findings_summary.dimension_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_locale_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_locale_anchor
        ));
        out.push_str(&format!(
            "| `missing_inclusive_note` | {} |\n",
            self.findings_summary.missing_inclusive_note
        ));
        out.push_str(&format!(
            "| `missing_claimed_locales` | {} |\n",
            self.findings_summary.missing_claimed_locales
        ));
        out.push_str(&format!(
            "| `surface_not_on_inclusive_harness` | {} |\n\n",
            self.findings_summary.surface_not_on_inclusive_harness
        ));

        out.push_str("## Locale anchor index\n\n");
        out.push_str(
            "| Surface family | Surface | Locale anchor |\n| -------------- | ------- | ------------- |\n",
        );
        for entry in &self.locale_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.surface_family.display_label(),
                entry.surface_id,
                entry.locale_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for surface in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {})\n\n",
                surface.descriptor.surface_id,
                surface.descriptor.surface_family.as_str(),
                surface.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                surface.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Semantic salience: `{}`\n",
                surface.descriptor.semantic_salience.as_str()
            ));
            out.push_str(&format!(
                "- Locale anchor: `{}`\n",
                surface.descriptor.locale_anchor_ref
            ));
            out.push_str(&format!(
                "- Claimed locales: {}\n",
                surface
                    .descriptor
                    .claimed_locales
                    .iter()
                    .map(|locale| format!("`{}`", locale.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Marketed on inclusive rows: `{}`\n",
                if surface.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-salience: `{}`\n\n",
                if surface.high_salience { "yes" } else { "no" }
            ));

            out.push_str(
                "| Scenario row | Status | Keyboard | Narration | Focus | Text | IME | Bidi | Zoom | Locale | Cue | Freshness | Narrowing reason |\n\
                 | ------------ | ------ | -------- | --------- | ----- | ---- | --- | ---- | ---- | ------ | --- | --------- | ---------------- |\n",
            );
            for binding in &surface.bindings {
                let keyboard = binding
                    .projected_keyboard_reachability
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narration = binding
                    .projected_narration
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let focus = binding
                    .projected_focus_visibility
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let text = binding
                    .projected_text_correctness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let ime = binding
                    .projected_ime_composition
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let bidi = binding
                    .projected_bidi_isolation
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let zoom = binding
                    .projected_zoom_reflow
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let locale = binding
                    .projected_locale_parity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let cue = binding
                    .projected_suspicious_content_cue
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness = binding
                    .evidence_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.row.display_label(),
                    binding.qualification_status.as_str(),
                    keyboard,
                    narration,
                    focus,
                    text,
                    ime,
                    bidi,
                    zoom,
                    locale,
                    cue,
                    freshness,
                    narrowing,
                ));
            }
            out.push('\n');

            if surface.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &surface.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .row()
                            .map(M5InclusiveRow::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_inclusive_depth -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_inclusive_depth_fixtures\n");
        out.push_str("python3 tools/ci/m5/inclusive_depth_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 inclusive-depth audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InclusiveSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5InclusiveDepthReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5InclusiveSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5InclusiveDepthReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for surface in &report.rows {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_INCLUSIVE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_INCLUSIVE_SCHEMA_VERSION,
            shared_contract_ref: M5_INCLUSIVE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor and its row
/// bindings.
fn compute_surface_findings(
    descriptor: &M5InclusiveDescriptor,
    bindings: &[M5InclusiveBinding],
    high_salience: bool,
) -> Vec<M5InclusiveBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.locale_anchor_ref.trim().is_empty() {
        findings.push(M5InclusiveBlockingFinding::DescriptorMissingLocaleAnchor {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.inclusive_note.trim().is_empty() {
        findings.push(M5InclusiveBlockingFinding::MissingInclusiveNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.claimed_locales.is_empty() {
        findings.push(M5InclusiveBlockingFinding::MissingClaimedLocales {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.registered_on_inclusive_harness {
        findings.push(M5InclusiveBlockingFinding::SurfaceNotOnInclusiveHarness {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    for binding in bindings {
        let row = binding.row;
        let surface_id = descriptor.surface_id.clone();

        // A binding's dimension must match its row's canonical dimension.
        if binding.dimension != row.canonical_dimension() {
            findings.push(M5InclusiveBlockingFinding::DimensionDrift {
                surface_id: surface_id.clone(),
                row,
                projected_dimension: binding.dimension,
            });
        }

        match binding.qualification_status {
            M5InclusiveQualificationStatus::UnqualifiedLocalA11yPath => {
                findings.push(M5InclusiveBlockingFinding::UnqualifiedLocalA11yPath {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5InclusiveQualificationStatus::MissingEvidence => {
                findings.push(M5InclusiveBlockingFinding::MissingEvidence {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5InclusiveQualificationStatus::Qualified => {
                compute_qualified_findings(binding, high_salience, &surface_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5InclusiveBlockingFinding::MissingNarrowingReason {
                        surface_id: surface_id.clone(),
                        row,
                        qualification_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the blocking findings for one qualified inclusive binding.
fn compute_qualified_findings(
    binding: &M5InclusiveBinding,
    high_salience: bool,
    surface_id: &str,
    findings: &mut Vec<M5InclusiveBlockingFinding>,
) {
    let row = binding.row;

    // Required captured-evidence projections (every qualified row).
    if binding.projected_evidence_pack_ref.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_evidence_pack_ref".to_owned(),
        });
    }
    if binding.projected_keyboard_reachability.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_keyboard_reachability".to_owned(),
        });
    }
    if binding.projected_narration.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_narration".to_owned(),
        });
    }
    if binding.projected_focus_visibility.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_focus_visibility".to_owned(),
        });
    }
    if binding.projected_text_correctness.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_text_correctness".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "evidence_freshness".to_owned(),
        });
    }
    if row.requires_ime_composition() && binding.projected_ime_composition.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_ime_composition".to_owned(),
        });
    }
    if row.requires_bidi_isolation() && binding.projected_bidi_isolation.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_bidi_isolation".to_owned(),
        });
    }
    if row.requires_zoom_reflow() && binding.projected_zoom_reflow.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_zoom_reflow".to_owned(),
        });
    }
    if row.requires_locale_parity() && binding.projected_locale_parity.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_locale_parity".to_owned(),
        });
    }
    if high_salience && binding.projected_suspicious_content_cue.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_suspicious_content_cue".to_owned(),
        });
    }

    // Missing evidence pack is also a dedicated class.
    if binding.projected_evidence_pack_ref.is_none() {
        findings.push(M5InclusiveBlockingFinding::MissingEvidencePack {
            surface_id: surface_id.to_owned(),
            row,
        });
    }

    // Red captured results.
    if binding.projected_keyboard_reachability == Some(M5KeyboardReachability::Trapped) {
        findings.push(M5InclusiveBlockingFinding::KeyboardUnreachable {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_narration == Some(M5Narration::Silent) {
        findings.push(M5InclusiveBlockingFinding::NarrationSilent {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_narration == Some(M5Narration::Misannounced) {
        findings.push(M5InclusiveBlockingFinding::NarrationMisannounced {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_focus_visibility == Some(M5FocusVisibility::Hidden) {
        findings.push(M5InclusiveBlockingFinding::FocusIndicatorHidden {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_text_correctness == Some(M5TextCorrectness::Corrupted) {
        findings.push(M5InclusiveBlockingFinding::TextCorrupted {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_ime_composition()
        && binding.projected_ime_composition == Some(M5ImeComposition::Broken)
    {
        findings.push(M5InclusiveBlockingFinding::ImeCompositionBroken {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_bidi_isolation()
        && binding.projected_bidi_isolation == Some(M5BidiIsolation::Leaking)
    {
        findings.push(M5InclusiveBlockingFinding::BidiLeaking {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_zoom_reflow() && binding.projected_zoom_reflow == Some(M5ZoomReflow::Clipped) {
        findings.push(M5InclusiveBlockingFinding::ZoomContentClipped {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_locale_parity()
        && matches!(
            binding.projected_locale_parity,
            Some(M5LocaleParity::SilentEnglishFallback) | Some(M5LocaleParity::Mismatched)
        )
    {
        findings.push(M5InclusiveBlockingFinding::LocaleParityLost {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_suspicious_content_cue == Some(M5SuspiciousContentCue::Hidden) {
        findings.push(M5InclusiveBlockingFinding::SuspiciousContentHidden {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.marketed_on_row && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale) {
        findings.push(M5InclusiveBlockingFinding::StaleEvidenceOnMarketedRow {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
}

/// Computes the per-row and per-class summaries from finished surfaces.
fn summarize_report(
    surfaces: &[M5InclusiveQualificationRow],
) -> (Vec<M5InclusiveCoverageSummary>, M5InclusiveFindingSummary) {
    let mut summary = M5InclusiveFindingSummary::empty();
    let mut coverage: Vec<M5InclusiveCoverageSummary> = M5InclusiveRow::required_rows()
        .into_iter()
        .map(|row| M5InclusiveCoverageSummary {
            row,
            qualified_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            locale_omitted_rows: 0,
            declared_capture_gap_rows: 0,
            unqualified_local_a11y_path_rows: 0,
            missing_evidence_rows: 0,
        })
        .collect();

    for surface in surfaces {
        for binding in &surface.bindings {
            if let Some(coverage_row) = coverage.iter_mut().find(|entry| entry.row == binding.row) {
                match binding.qualification_status {
                    M5InclusiveQualificationStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5InclusiveQualificationStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5InclusiveQualificationStatus::NotApplicable => {
                        coverage_row.not_applicable_rows += 1
                    }
                    M5InclusiveQualificationStatus::LocaleOmitted => {
                        coverage_row.locale_omitted_rows += 1
                    }
                    M5InclusiveQualificationStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5InclusiveQualificationStatus::UnqualifiedLocalA11yPath => {
                        coverage_row.unqualified_local_a11y_path_rows += 1
                    }
                    M5InclusiveQualificationStatus::MissingEvidence => {
                        coverage_row.missing_evidence_rows += 1
                    }
                }
            }
        }
        for finding in &surface.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// inclusive evidence is stale or red.
fn compute_narrowable_rows(
    surfaces: &[M5InclusiveQualificationRow],
) -> Vec<M5InclusiveNarrowableRow> {
    let mut narrowable = Vec::new();
    for surface in surfaces {
        if !surface.marketed {
            continue;
        }
        for finding in &surface.blocking_findings {
            if let Some(row) = finding.row() {
                narrowable.push(M5InclusiveNarrowableRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5InclusiveQualificationRow`] from a descriptor and its row
/// bindings, computing the per-surface blocking findings.
pub fn build_m5_inclusive_row(
    descriptor: M5InclusiveDescriptor,
    bindings: Vec<M5InclusiveBinding>,
) -> M5InclusiveQualificationRow {
    let high_salience = descriptor.is_high_salience();
    let marketed = descriptor.marketed_on_inclusive_rows;
    let blocking_findings = compute_surface_findings(&descriptor, &bindings, high_salience);

    M5InclusiveQualificationRow {
        record_kind: M5_INCLUSIVE_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_INCLUSIVE_SCHEMA_VERSION,
        shared_contract_ref: M5_INCLUSIVE_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_salience,
        marketed,
    }
}

/// Builds a full [`M5InclusiveDepthReport`] from per-surface rows.
pub fn build_m5_inclusive_depth_audit(
    surfaces: Vec<M5InclusiveQualificationRow>,
) -> M5InclusiveDepthReport {
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = surfaces.len();
    let high_salience_surface_count = surfaces.iter().filter(|row| row.high_salience).count();
    let marketed_surface_count = surfaces.iter().filter(|row| row.marketed).count();
    let inclusive_rows_checked = surfaces.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (row_coverage, findings_summary) = summarize_report(&surfaces);
    let narrowable_marketed_rows = compute_narrowable_rows(&surfaces);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut locale_set: Vec<M5InclusiveLocale> = Vec::new();
    for surface in &surfaces {
        for locale in &surface.descriptor.claimed_locales {
            if !locale_set.contains(locale) {
                locale_set.push(*locale);
            }
        }
    }
    locale_set.sort();

    let mut locale_anchor_index: Vec<M5InclusiveLocaleAnchorEntry> = surfaces
        .iter()
        .map(|surface| M5InclusiveLocaleAnchorEntry {
            surface_family: surface.descriptor.surface_family,
            surface_id: surface.descriptor.surface_id.clone(),
            locale_anchor_ref: surface.descriptor.locale_anchor_ref.clone(),
        })
        .collect();
    locale_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5InclusiveDepthReport {
        record_kind: M5_INCLUSIVE_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_INCLUSIVE_SCHEMA_VERSION,
        shared_contract_ref: M5_INCLUSIVE_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_INCLUSIVE_REPORT_ID.to_owned(),
        source_schema_ref: M5_INCLUSIVE_SOURCE_SCHEMA_REF.to_owned(),
        required_rows: M5InclusiveRow::required_rows().to_vec(),
        claimed_locales: locale_set,
        rows: surfaces,
        row_coverage,
        findings_summary,
        locale_anchor_index,
        registered_surface_count,
        high_salience_surface_count,
        marketed_surface_count,
        inclusive_rows_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_INCLUSIVE_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_INCLUSIVE_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_INCLUSIVE_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/component-state-parity.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-inclusive-depth".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_inclusive_depth`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5InclusiveValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required scenario row has no qualified surface.
    RequiredRowNotQualified { row: String },
    /// A surface is missing a required scenario row from its binding set.
    MissingRequiredRow { surface_id: String, row: String },
    /// A blocking finding remains on the surface.
    BlockingFindingPresent {
        surface_id: String,
        row: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A surface's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { surface_id: String },
}

/// Validates an audit report against the M5 inclusive acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_inclusive_depth(
    report: &M5InclusiveDepthReport,
) -> Result<(), Vec<M5InclusiveValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5InclusiveValidationError::NoRegisteredSurfaces);
    }

    for required in M5InclusiveRow::required_rows() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.row == required
                    && binding.qualification_status == M5InclusiveQualificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(M5InclusiveValidationError::RequiredRowNotQualified {
                row: required.as_str().to_owned(),
            });
        }
    }

    for surface in &report.rows {
        for required in M5InclusiveRow::required_rows() {
            if !surface
                .bindings
                .iter()
                .any(|binding| binding.row == required)
            {
                errors.push(M5InclusiveValidationError::MissingRequiredRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row: required.as_str().to_owned(),
                });
            }
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5InclusiveValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(M5InclusiveValidationError::BlockingFindingPresent {
                surface_id: finding.surface_id().to_owned(),
                row: finding
                    .row()
                    .map(|row| row.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5InclusiveValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5InclusiveValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_inclusive_depth_audit`].
struct SurfaceSeed {
    surface_id: &'static str,
    surface_family: M5InclusiveSurfaceFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    locale_anchor_ref: &'static str,
    inclusive_note: &'static str,
    semantic_salience: M5SemanticSalience,
    lifecycle_label: M5SurfaceLifecycle,
    suspicious_cue: M5SuspiciousContentCue,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    row: M5InclusiveRow,
    qualification_status: M5InclusiveQualificationStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified row with captured evidence.
const fn qualified(row: M5InclusiveRow) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5InclusiveQualificationStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(row: M5InclusiveRow, reason: &'static str) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5InclusiveQualificationStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

const ALL_QUALIFIED: &[BindingSeed] = &[
    qualified(M5InclusiveRow::KeyboardReachability),
    qualified(M5InclusiveRow::ScreenReaderNarration),
    qualified(M5InclusiveRow::HighZoom),
    qualified(M5InclusiveRow::ImeComposition),
    qualified(M5InclusiveRow::GraphemeCorrectness),
    qualified(M5InclusiveRow::BidiDirection),
    qualified(M5InclusiveRow::Pseudolocalization),
    qualified(M5InclusiveRow::LocaleFallback),
    qualified(M5InclusiveRow::TranslatedHelpParity),
];

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Notebook cell. Lifecycle-bearing; editable code/markdown plus output.
    SurfaceSeed {
        surface_id: "surface:notebook.cell",
        surface_family: M5InclusiveSurfaceFamily::NotebookCell,
        descriptor_revision_ref: "surface-rev:notebook.cell:2026.06.01-01",
        primary_label_ref: "label:notebook.cell:primary",
        locale_anchor_ref: "a11y:anchor:notebook:cell",
        inclusive_note: "A cell editor narrates run state and stays operable by keyboard; CJK composition and right-to-left output keep their graphemes and direction across the cell frame.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Result-grid rows. Lifecycle-bearing; dense data rows.
    SurfaceSeed {
        surface_id: "surface:data_api.result_grid_row",
        surface_family: M5InclusiveSurfaceFamily::ResultGridRow,
        descriptor_revision_ref: "surface-rev:data_api.result_grid_row:2026.06.01-01",
        primary_label_ref: "label:data_api.result_grid_row:primary",
        locale_anchor_ref: "a11y:anchor:data_api:result_grid_row",
        inclusive_note: "Dense result cells stay keyboard-navigable and narrated per cell; mixed-direction values are isolated and escaped values render exactly, never split or mojibaked.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Pipeline / log view. Severity-bearing; status and log rows.
    SurfaceSeed {
        surface_id: "surface:review.pipeline_log_view",
        surface_family: M5InclusiveSurfaceFamily::PipelineLogView,
        descriptor_revision_ref: "surface-rev:review.pipeline_log_view:2026.06.01-01",
        primary_label_ref: "label:review.pipeline_log_view:primary",
        locale_anchor_ref: "a11y:anchor:review:pipeline_log_view",
        inclusive_note: "A blocked pipeline narrates its failure as severity, not colour alone; log lines with control characters and bidi runs decode-recover without corrupting adjacent rows.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Profiler timeline. Informational; diagnostic timeline.
    SurfaceSeed {
        surface_id: "surface:profiler.timeline",
        surface_family: M5InclusiveSurfaceFamily::ProfilerTimeline,
        descriptor_revision_ref: "surface-rev:profiler.timeline:2026.06.01-01",
        primary_label_ref: "label:profiler.timeline:primary",
        locale_anchor_ref: "a11y:anchor:profiler:timeline",
        inclusive_note: "Timeline frames are reachable and narrated by keyboard with a text alternative; localized frame labels keep parity and reflow at high zoom rather than clipping.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Guided tour. Informational; coachmark walkthrough.
    SurfaceSeed {
        surface_id: "surface:learning.guided_tour",
        surface_family: M5InclusiveSurfaceFamily::GuidedTour,
        descriptor_revision_ref: "surface-rev:learning.guided_tour:2026.06.01-01",
        primary_label_ref: "label:learning.guided_tour:primary",
        locale_anchor_ref: "a11y:anchor:learning:guided_tour",
        inclusive_note: "Tour steps trap no focus, advance by keyboard, and narrate progress; translated tour copy keeps parity with the feature rather than silently falling back to English.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Docs / help pane. Informational; embedded help content.
    SurfaceSeed {
        surface_id: "surface:docs_help.pane",
        surface_family: M5InclusiveSurfaceFamily::DocsHelpPane,
        descriptor_revision_ref: "surface-rev:docs_help.pane:2026.06.01-01",
        primary_label_ref: "label:docs_help.pane:primary",
        locale_anchor_ref: "a11y:anchor:docs_help:pane",
        inclusive_note: "Help articles are keyboard-readable and narrated as a document; translated help stays in parity, and rich/escaped snippets render exactly across raw, rendered, and escaped paths.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Companion surface. Trust-bearing; provider-backed; declares an IME gap.
    SurfaceSeed {
        surface_id: "surface:companion.surface",
        surface_family: M5InclusiveSurfaceFamily::CompanionSurface,
        descriptor_revision_ref: "surface-rev:companion.surface:2026.06.01-01",
        primary_label_ref: "label:companion.surface:primary",
        locale_anchor_ref: "a11y:anchor:companion:surface",
        inclusive_note: "Presence and handoff cues narrate trust state and stay keyboard-reachable; the companion provider's on-device IME composition is declared honestly, not claimed.",
        semantic_salience: M5SemanticSalience::TrustBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: &[
            qualified(M5InclusiveRow::KeyboardReachability),
            qualified(M5InclusiveRow::ScreenReaderNarration),
            qualified(M5InclusiveRow::HighZoom),
            declared_capture_gap(
                M5InclusiveRow::ImeComposition,
                "companion_provider_owns_its_on_device_ime_so_the_composition_capture_is_provider_attributed",
            ),
            qualified(M5InclusiveRow::GraphemeCorrectness),
            qualified(M5InclusiveRow::BidiDirection),
            qualified(M5InclusiveRow::Pseudolocalization),
            qualified(M5InclusiveRow::LocaleFallback),
            qualified(M5InclusiveRow::TranslatedHelpParity),
        ],
    },
    // Query console. Lifecycle-bearing; editable query input plus results.
    SurfaceSeed {
        surface_id: "surface:data_api.query_console",
        surface_family: M5InclusiveSurfaceFamily::QueryConsole,
        descriptor_revision_ref: "surface-rev:data_api.query_console:2026.06.01-01",
        primary_label_ref: "label:data_api.query_console:primary",
        locale_anchor_ref: "a11y:anchor:data_api:query_console",
        inclusive_note: "The query input preserves IME pre-edit and narrates validation state; localized hints keep parity and bidi query fragments stay isolated from surrounding chrome.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Preview-route pane. Lifecycle-bearing; embedded preview; declares a help gap.
    SurfaceSeed {
        surface_id: "surface:preview.route_pane",
        surface_family: M5InclusiveSurfaceFamily::PreviewRoutePane,
        descriptor_revision_ref: "surface-rev:preview.route_pane:2026.06.01-01",
        primary_label_ref: "label:preview.route_pane:primary",
        locale_anchor_ref: "a11y:anchor:preview:route_pane",
        inclusive_note: "Shell chrome around the embedded preview narrates route lifecycle and stays keyboard-reachable; the embedded provider's own translated help is declared, not claimed.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: &[
            qualified(M5InclusiveRow::KeyboardReachability),
            qualified(M5InclusiveRow::ScreenReaderNarration),
            qualified(M5InclusiveRow::HighZoom),
            qualified(M5InclusiveRow::ImeComposition),
            qualified(M5InclusiveRow::GraphemeCorrectness),
            qualified(M5InclusiveRow::BidiDirection),
            qualified(M5InclusiveRow::Pseudolocalization),
            qualified(M5InclusiveRow::LocaleFallback),
            declared_capture_gap(
                M5InclusiveRow::TranslatedHelpParity,
                "embedded_provider_owns_its_localized_help_so_the_translated_help_parity_capture_is_provider_attributed",
            ),
        ],
    },
    // Glossary panel. Informational; translated glossary content.
    SurfaceSeed {
        surface_id: "surface:learning.glossary_panel",
        surface_family: M5InclusiveSurfaceFamily::GlossaryPanel,
        descriptor_revision_ref: "surface-rev:learning.glossary_panel:2026.06.01-01",
        primary_label_ref: "label:learning.glossary_panel:primary",
        locale_anchor_ref: "a11y:anchor:learning:glossary_panel",
        inclusive_note: "Glossary entries are keyboard-navigable and narrated as a definition list; translated terms keep parity, and pseudolocalized strings expand without truncation.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Support packet. Severity-bearing; support handoff.
    SurfaceSeed {
        surface_id: "surface:support.packet",
        surface_family: M5InclusiveSurfaceFamily::SupportPacket,
        descriptor_revision_ref: "surface-rev:support.packet:2026.06.01-01",
        primary_label_ref: "label:support.packet:primary",
        locale_anchor_ref: "a11y:anchor:support:packet",
        inclusive_note: "A support packet narrates its severity and suspicious-content cue, stays keyboard-reachable, and renders captured user text exactly across raw, rendered, and escaped paths.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        suspicious_cue: M5SuspiciousContentCue::Present,
        bindings: ALL_QUALIFIED,
    },
];

fn build_binding_from_seed(seed: &SurfaceSeed, binding_seed: &BindingSeed) -> M5InclusiveBinding {
    let row = binding_seed.row;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_salience = seed.semantic_salience.is_high_salience();
    let marketed_on_row = !matches!(
        binding_seed.qualification_status,
        M5InclusiveQualificationStatus::NotApplicable
            | M5InclusiveQualificationStatus::LocaleOmitted
    );

    M5InclusiveBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_row,
        projected_evidence_pack_ref: qualified
            .then(|| format!("drill:{}:{}", seed.surface_id, row.as_str())),
        projected_keyboard_reachability: qualified.then_some(M5KeyboardReachability::Reachable),
        projected_narration: qualified.then_some(M5Narration::Narrated),
        projected_focus_visibility: qualified.then_some(M5FocusVisibility::Visible),
        projected_text_correctness: qualified.then_some(M5TextCorrectness::Correct),
        projected_ime_composition: (qualified && row.requires_ime_composition())
            .then_some(M5ImeComposition::Preserved),
        projected_bidi_isolation: (qualified && row.requires_bidi_isolation())
            .then_some(M5BidiIsolation::Isolated),
        projected_zoom_reflow: (qualified && row.requires_zoom_reflow())
            .then_some(M5ZoomReflow::Reflowed),
        projected_locale_parity: (qualified && row.requires_locale_parity())
            .then_some(M5LocaleParity::Parity),
        projected_suspicious_content_cue: (qualified && high_salience)
            .then_some(seed.suspicious_cue),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> M5InclusiveQualificationRow {
    let descriptor = M5InclusiveDescriptor {
        surface_id: seed.surface_id.to_owned(),
        surface_family: seed.surface_family,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        locale_anchor_ref: seed.locale_anchor_ref.to_owned(),
        inclusive_note: seed.inclusive_note.to_owned(),
        semantic_salience: seed.semantic_salience,
        lifecycle_label: seed.lifecycle_label,
        claimed_locales: M5InclusiveLocale::all().to_vec(),
        marketed_on_inclusive_rows: true,
        registered_on_inclusive_harness: true,
    };
    let bindings: Vec<M5InclusiveBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_inclusive_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/a11y/m5_ime_bidi_pseudoloc/`.
pub fn seeded_m5_inclusive_depth_audit() -> M5InclusiveDepthReport {
    let surfaces = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_m5_inclusive_depth_audit(surfaces)
}
