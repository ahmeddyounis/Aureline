//! Content-ops metadata for docs/help snippets, export/report headings,
//! screenshot/demo captions, and translator notes.
//!
//! This module materializes the canonical, export-safe catalog that gives
//! Aureline's *non-runtime* wording the same source/command/version/build context
//! the product already expects from runtime surfaces. Where the safety-critical
//! string catalog locks the *identity* of a message, the action-label catalog locks
//! its *scope honesty*, and the AI copy guardrails lock its *trust posture*, this
//! catalog locks its *provenance*: every docs/help snippet, export/report heading,
//! screenshot/demo caption, and translator note declares where its wording came
//! from, which command/source it reflects, the product version and build it was
//! captured against, the placeholder semantics a translator needs, and the locale
//! fallback posture it falls back through.
//!
//! It is the focused content-ops projection of the product-wide
//! [translation-safe content-ops contract](../../../../../docs/copy/translation_safe_content_ops_contract.md):
//! the four [`ContentArtifactKind`]s named for this lane are exactly the easy-to-corrupt
//! surfaces that move through translation, screenshot capture, docs/help packaging,
//! and support workflows.
//!
//! Three honesty rules drive the validation:
//!
//! - **No versionless release/support truth.** Rendered artifacts
//!   ([`ContentArtifactKind::DocsHelpSnippet`], [`ContentArtifactKind::ExportReportHeading`],
//!   [`ContentArtifactKind::ScreenshotDemoCaption`]) declare a product version and
//!   build ref, and a screenshot/demo caption additionally declares its
//!   [`CapturePosture`] (live, mocked, synthetic) and [`CaptionSyncState`] and discloses
//!   mocked-versus-live posture — so a caption can never imply live/stable/current
//!   product truth while lacking the metadata that would prove it.
//! - **Headings pair a human label with a machine code.** An export/report heading
//!   carries a locale-neutral [`ContentOpsEntry::machine_field_name`] beside its
//!   localizable [`ContentOpsEntry::canonical_text`], so the heading localizes safely
//!   while the export field id / report column id stays stable.
//! - **Variable-rich wording is translation-safe.** Every placeholder in a rendered
//!   string resolves to a [`PlaceholderNote`] by id (not position) with a typed
//!   [`PlaceholderKind`], a [`TokenFidelityClass`], and a fallback, and a translator
//!   note can attach those notes to a variable-rich safety-critical string or heading.
//!
//! Machine-facing identity stays locale-neutral — entry ids, machine field names,
//! command refs, and placeholder token ids are lowercase ascii (`[a-z0-9_.]`) — while
//! human prose localizes safely around it, so a localized overlay can never fork a
//! command id, an export field id, or a placeholder token into business logic. The
//! localized overlay fixture proves it. The packet carries no credential bodies or
//! raw provider payloads, so the product UI, docs/help, release notes, support
//! exports, and screenshot/demo pipelines can all reconstruct the same provenance.
//!
//! The boundary schema is
//! [`schemas/content/m5-content-ops-metadata.schema.json`](../../../../../schemas/content/m5-content-ops-metadata.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_content_ops_metadata.md`](../../../../../docs/content/m5/m5_content_ops_metadata.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-content-ops-metadata/`](../../../../../fixtures/content/m5-content-ops-metadata/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_content_ops_metadata_catalog, seeded_content_ops_metadata_catalog_localized,
    seeded_content_ops_metadata_catalog_offline_mirror, CONTENT_OPS_METADATA_CATALOG_ID,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ContentOpsMetadataCatalog`].
pub const CONTENT_OPS_METADATA_CATALOG_RECORD_KIND: &str = "m5_content_ops_metadata_catalog";

/// Schema version for content-ops metadata catalog records.
pub const CONTENT_OPS_METADATA_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct reuse consumers a shared entry must span.
pub const SHARED_ENTRY_MIN_REUSE_CONSUMERS: usize = 3;

/// Repo-relative path of the boundary schema.
pub const CONTENT_OPS_METADATA_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-content-ops-metadata.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const CONTENT_OPS_METADATA_CATALOG_DOC_REF: &str = "docs/content/m5/m5_content_ops_metadata.md";

/// Repo-relative path of the product-wide translation-safe content-ops contract this
/// catalog materializes.
pub const CONTENT_OPS_CONTRACT_REF: &str = "docs/copy/translation_safe_content_ops_contract.md";

/// Repo-relative path of the message-placeholder boundary schema the placeholder
/// notes align with.
pub const MESSAGE_PLACEHOLDER_SCHEMA_REF: &str = "schemas/ux/message_placeholder.schema.json";

/// Repo-relative path of the controlled late-copy change schema.
pub const LATE_COPY_CHANGE_SCHEMA_REF: &str = "schemas/copy/late_copy_change.schema.json";

/// Repo-relative path of the naming / state-label contract that owns controlled
/// labels and glossary ownership.
pub const NAMING_LABEL_CONTRACT_REF: &str = "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the count/scope/freshness grammar contract.
pub const COUNT_SCOPE_GRAMMAR_REF: &str = "docs/copy/count_scope_freshness_grammar.md";

/// Repo-relative path of the locale-fallback / copy-representation contract.
pub const LOCALE_FALLBACK_CONTRACT_REF: &str =
    "docs/accessibility/locale_fallback_and_copy_representation_contract.md";

/// Repo-relative path of the controlled glossary; glossary token refs must align
/// with the controlled terms owned there.
pub const CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the protected fixture directory.
pub const CONTENT_OPS_METADATA_CATALOG_FIXTURE_DIR: &str =
    "fixtures/content/m5-content-ops-metadata";

/// Repo-relative path of the checked support-export artifact.
pub const CONTENT_OPS_METADATA_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-content-ops-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CONTENT_OPS_METADATA_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-content-ops-proof/m5_content_ops_metadata.md";

/// The kind of content-ops artifact a metadata entry governs.
///
/// These are exactly the four artifact kinds this lane is required to carry
/// metadata for: a docs/help snippet, an export/report heading, a screenshot/demo
/// caption, and a translator note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentArtifactKind {
    /// A docs browser, help, service-health, support, or learning prose excerpt
    /// carried as a snippet rather than a full docs body.
    DocsHelpSnippet,
    /// A report heading, support-bundle heading, evidence export label, release row,
    /// or CSV/JSON companion label.
    ExportReportHeading,
    /// A caption, subtitle, voice-over line, alt text, or presentation copy paired
    /// with captured product media.
    ScreenshotDemoCaption,
    /// A translator-facing review note carrying placeholder semantics, glossary refs,
    /// or caption-governance guidance for a target string.
    TranslatorNote,
}

impl ContentArtifactKind {
    /// Every artifact kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsHelpSnippet,
        Self::ExportReportHeading,
        Self::ScreenshotDemoCaption,
        Self::TranslatorNote,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpSnippet => "docs_help_snippet",
            Self::ExportReportHeading => "export_report_heading",
            Self::ScreenshotDemoCaption => "screenshot_demo_caption",
            Self::TranslatorNote => "translator_note",
        }
    }

    /// True when this kind is rendered product wording that must declare a product
    /// version and build ref. Translator notes are review guidance, not rendered
    /// product truth, so they are exempt.
    pub const fn requires_version_context(self) -> bool {
        matches!(
            self,
            Self::DocsHelpSnippet | Self::ExportReportHeading | Self::ScreenshotDemoCaption
        )
    }

    /// True when this kind's [`ContentOpsEntry::canonical_text`] is itself rendered
    /// product wording, so every placeholder it contains must resolve to a note. A
    /// translator note's prose references the *target* string's placeholders, so it
    /// is excluded.
    pub const fn canonical_text_is_rendered(self) -> bool {
        !matches!(self, Self::TranslatorNote)
    }
}

/// A consumer surface that must be able to reconstruct the same content-ops metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentOpsConsumer {
    /// Docs and help content.
    DocsHelp,
    /// Release notes and release rows.
    ReleaseNotes,
    /// A support / report export packet.
    SupportExport,
    /// The screenshot / demo capture pipeline.
    ScreenshotDemoPipeline,
    /// CLI / `--help` output.
    CliHelp,
}

impl ContentOpsConsumer {
    /// Every reuse consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsHelp,
        Self::ReleaseNotes,
        Self::SupportExport,
        Self::ScreenshotDemoPipeline,
        Self::CliHelp,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelp => "docs_help",
            Self::ReleaseNotes => "release_notes",
            Self::SupportExport => "support_export",
            Self::ScreenshotDemoPipeline => "screenshot_demo_pipeline",
            Self::CliHelp => "cli_help",
        }
    }
}

/// The mocked-versus-live posture of captured media.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturePosture {
    /// Captured from a live product run at the declared build.
    Live,
    /// Captured against mocked or sample data, not a live product run.
    Mocked,
    /// A synthetic or composed preview, not a real capture.
    Synthetic,
    /// Not captured media (docs snippet, heading, or translator note).
    NotApplicable,
}

impl CapturePosture {
    /// Every capture posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Live,
        Self::Mocked,
        Self::Synthetic,
        Self::NotApplicable,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mocked => "mocked",
            Self::Synthetic => "synthetic",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the posture describes real captured media (live, mocked, or
    /// synthetic) rather than non-media content.
    pub const fn is_media(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Whether a screenshot/demo caption is in sync with the source surface it describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptionSyncState {
    /// Caption text and the source surface were reviewed together and agree.
    InSync,
    /// The capture is older than the source surface; usable only in internal review.
    Stale,
    /// The caption is awaiting caption-sync review.
    PendingReview,
    /// Not captured media, so caption sync does not apply.
    NotApplicable,
}

impl CaptionSyncState {
    /// Every caption-sync state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InSync,
        Self::Stale,
        Self::PendingReview,
        Self::NotApplicable,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::Stale => "stale",
            Self::PendingReview => "pending_review",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the caption may be published as launch/release evidence. A stale or
    /// pending caption may live in an internal fixture but is not publishable.
    pub const fn publishable(self) -> bool {
        matches!(self, Self::InSync)
    }
}

/// The source-language / locale fallback posture a content-ops entry falls through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleFallbackStrategy {
    /// Falls back through the locale chain to the authoritative source language.
    SourceLanguageRoute,
    /// Falls back to the nearest available regional locale before source language.
    NearestLocale,
    /// Falls back to the locale-neutral machine token / code (e.g. an export field
    /// id) when no localized prose is available.
    MachineToken,
    /// Source-language fallback is blocked by policy; the entry names the policy ref.
    PolicyBlocked,
}

impl LocaleFallbackStrategy {
    /// Every fallback strategy, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceLanguageRoute,
        Self::NearestLocale,
        Self::MachineToken,
        Self::PolicyBlocked,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageRoute => "source_language_route",
            Self::NearestLocale => "nearest_locale",
            Self::MachineToken => "machine_token",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The semantic kind of a placeholder token, mirroring the product-wide content-ops
/// contract's closed `placeholder_kind` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderKind {
    /// A pluralizable count.
    Count,
    /// A stable command id token.
    CommandIdToken,
    /// An analytics key token.
    AnalyticsKeyToken,
    /// An automation route id token.
    AutomationRouteToken,
    /// A support-tool field id token.
    SupportFieldToken,
    /// A policy id token.
    PolicyIdToken,
    /// A file path token.
    FilePathToken,
    /// A host or URL token.
    HostOrUrlToken,
    /// A tenant or account token.
    TenantOrAccountToken,
    /// A CLI flag or argument token.
    FlagOrArgumentToken,
    /// A product version or build token.
    VersionOrBuildToken,
    /// A locale tag token.
    LocaleTagToken,
    /// A controlled glossary term token.
    GlossaryTermToken,
    /// An enumerated controlled-state token.
    EnumeratedStateToken,
    /// An evidence reference token.
    EvidenceRefToken,
    /// A free-form human-translatable string.
    FreeformString,
}

impl PlaceholderKind {
    /// Every placeholder kind, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::Count,
        Self::CommandIdToken,
        Self::AnalyticsKeyToken,
        Self::AutomationRouteToken,
        Self::SupportFieldToken,
        Self::PolicyIdToken,
        Self::FilePathToken,
        Self::HostOrUrlToken,
        Self::TenantOrAccountToken,
        Self::FlagOrArgumentToken,
        Self::VersionOrBuildToken,
        Self::LocaleTagToken,
        Self::GlossaryTermToken,
        Self::EnumeratedStateToken,
        Self::EvidenceRefToken,
        Self::FreeformString,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::CommandIdToken => "command_id_token",
            Self::AnalyticsKeyToken => "analytics_key_token",
            Self::AutomationRouteToken => "automation_route_token",
            Self::SupportFieldToken => "support_field_token",
            Self::PolicyIdToken => "policy_id_token",
            Self::FilePathToken => "file_path_token",
            Self::HostOrUrlToken => "host_or_url_token",
            Self::TenantOrAccountToken => "tenant_or_account_token",
            Self::FlagOrArgumentToken => "flag_or_argument_token",
            Self::VersionOrBuildToken => "version_or_build_token",
            Self::LocaleTagToken => "locale_tag_token",
            Self::GlossaryTermToken => "glossary_term_token",
            Self::EnumeratedStateToken => "enumerated_state_token",
            Self::EvidenceRefToken => "evidence_ref_token",
            Self::FreeformString => "freeform_string",
        }
    }

    /// True when this kind requires a controlled glossary ref rather than a
    /// translator-local synonym.
    pub const fn requires_glossary_ref(self) -> bool {
        matches!(self, Self::GlossaryTermToken | Self::EnumeratedStateToken)
    }

    /// True when this kind must declare a plural-rule ref.
    pub const fn requires_plural_rule(self) -> bool {
        matches!(self, Self::Count)
    }
}

/// How faithfully a placeholder token must survive translation, mirroring the
/// product-wide content-ops contract's closed `token_fidelity_class` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenFidelityClass {
    /// Must be preserved verbatim; translators must not paraphrase or normalize it.
    LiteralUnchanged,
    /// A controlled-vocabulary term resolved through a glossary ref.
    ControlledVocabularyTranslation,
    /// A locale-formatted value (number, date, duration).
    LocaleFormattedValue,
    /// Free human-translatable prose.
    HumanTranslatable,
}

impl TokenFidelityClass {
    /// Every token-fidelity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiteralUnchanged,
        Self::ControlledVocabularyTranslation,
        Self::LocaleFormattedValue,
        Self::HumanTranslatable,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralUnchanged => "literal_unchanged",
            Self::ControlledVocabularyTranslation => "controlled_vocabulary_translation",
            Self::LocaleFormattedValue => "locale_formatted_value",
            Self::HumanTranslatable => "human_translatable",
        }
    }
}

/// The review class of a translator note, mirroring the product-wide content-ops
/// contract's closed `translator_note_class` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslatorNoteClass {
    /// Placeholder semantics, reordering, and token fidelity.
    PlaceholderSemantics,
    /// Pluralization-rule guidance.
    PluralizationRule,
    /// Mixed-direction / bidi-isolated token guidance.
    MixedDirectionToken,
    /// A controlled glossary-term reference.
    GlossaryTermRef,
    /// Screenshot or demo caption governance.
    ScreenshotOrDemoCaptionGovernance,
    /// Source-language escape-hatch guidance.
    SourceLanguageEscapeHatch,
    /// Pseudoloc / truncation review guidance.
    PseudolocTruncationReview,
    /// A controlled late-copy delta after string freeze.
    LateCopyControlledDelta,
    /// Policy or legal review is required.
    PolicyOrLegalReviewRequired,
    /// Evidence-source review guidance.
    EvidenceSourceReview,
}

impl TranslatorNoteClass {
    /// Every translator-note class, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PlaceholderSemantics,
        Self::PluralizationRule,
        Self::MixedDirectionToken,
        Self::GlossaryTermRef,
        Self::ScreenshotOrDemoCaptionGovernance,
        Self::SourceLanguageEscapeHatch,
        Self::PseudolocTruncationReview,
        Self::LateCopyControlledDelta,
        Self::PolicyOrLegalReviewRequired,
        Self::EvidenceSourceReview,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceholderSemantics => "placeholder_semantics",
            Self::PluralizationRule => "pluralization_rule",
            Self::MixedDirectionToken => "mixed_direction_token",
            Self::GlossaryTermRef => "glossary_term_ref",
            Self::ScreenshotOrDemoCaptionGovernance => "screenshot_or_demo_caption_governance",
            Self::SourceLanguageEscapeHatch => "source_language_escape_hatch",
            Self::PseudolocTruncationReview => "pseudoloc_truncation_review",
            Self::LateCopyControlledDelta => "late_copy_controlled_delta",
            Self::PolicyOrLegalReviewRequired => "policy_or_legal_review_required",
            Self::EvidenceSourceReview => "evidence_source_review",
        }
    }
}

/// The product version and build context a content-ops entry was authored or
/// captured against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionContext {
    /// Product version ref the wording reflects (e.g. a channel/version ref).
    pub product_version_ref: String,
    /// Build ref the wording was authored or captured against.
    pub build_ref: String,
    /// Mocked-versus-live posture of any captured media.
    pub capture_posture: CapturePosture,
    /// Caption-sync state of any captured media.
    pub caption_sync_state: CaptionSyncState,
    /// True when the mocked-versus-live posture is explicitly disclosed to viewers.
    /// A screenshot/demo caption MUST disclose it so it can never imply live truth.
    pub mocked_versus_live_disclosed: bool,
}

/// A translation-safe note for one placeholder token in a rendered string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderNote {
    /// The literal placeholder as it appears in the rendered text (e.g. `{count}`).
    pub placeholder: String,
    /// The locale-neutral token id placeholders resolve by (e.g. `count`).
    pub token_id: String,
    /// The semantic kind of the token.
    pub kind: PlaceholderKind,
    /// How faithfully the token must survive translation.
    pub fidelity: TokenFidelityClass,
    /// Human-prose explanation of what the token means.
    pub semantic: String,
    /// What to render when the value is unavailable.
    pub fallback: String,
    /// True when the token must be bidi-isolated inside mixed-direction prose.
    pub bidi_isolation_required: bool,
    /// Plural-rule ref; required for [`PlaceholderKind::Count`].
    pub plural_rule_ref: Option<String>,
    /// Controlled glossary ref; required for glossary / enumerated-state tokens.
    pub glossary_term_ref: Option<String>,
}

/// The locale fallback posture for a content-ops entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleFallback {
    /// The authoritative source (default) locale.
    pub default_locale: String,
    /// The fallback strategy.
    pub strategy: LocaleFallbackStrategy,
    /// The fallback chain, starting at the requested locale; for any strategy other
    /// than [`LocaleFallbackStrategy::PolicyBlocked`] it terminates at the source
    /// language ([`LocaleFallback::default_locale`]).
    pub fallback_chain: Vec<String>,
    /// True when a non-authoritative fallback is disclosed to reviewers and to
    /// assistive technology where the text appears.
    pub non_authoritative_disclosed: bool,
    /// Policy ref; required for [`LocaleFallbackStrategy::PolicyBlocked`].
    pub policy_block_ref: Option<String>,
}

/// One content-ops metadata entry: a typed provenance packet for a docs/help
/// snippet, export/report heading, screenshot/demo caption, or translator note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsEntry {
    /// Stable, locale-neutral entry id (e.g. `entry.docs.project_doctor_findings`).
    pub entry_id: String,
    /// The artifact kind this entry governs.
    pub kind: ContentArtifactKind,
    /// Canonical (default-locale) human-readable wording.
    pub canonical_text: String,
    /// Locale-neutral machine field name / export field id / report column id paired
    /// with the human label. Required for [`ContentArtifactKind::ExportReportHeading`].
    pub machine_field_name: Option<String>,
    /// Where the wording came from: a glossary term, source message id, catalog ref,
    /// or docs anchor (the citation/provenance).
    pub source_ref: String,
    /// The command / source / route the wording reflects (locale-neutral).
    pub command_ref: Option<String>,
    /// Product version and build context.
    pub version_context: VersionContext,
    /// Translation-safe placeholder notes for the rendered string (or, for a
    /// translator note, for its target string).
    pub placeholder_notes: Vec<PlaceholderNote>,
    /// The translator-note class; required for [`ContentArtifactKind::TranslatorNote`].
    pub translator_note_class: Option<TranslatorNoteClass>,
    /// The target string id a translator note annotates; required for
    /// [`ContentArtifactKind::TranslatorNote`].
    pub target_string_ref: Option<String>,
    /// Locale fallback posture.
    pub locale_fallback: LocaleFallback,
    /// Reuse consumers that must reconstruct this entry.
    pub consumers: Vec<ContentOpsConsumer>,
    /// True when this entry is used on a release/support path and therefore must
    /// never be versionless.
    pub release_support_path: bool,
}

/// Catalog-level content-ops honesty review block.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsTrustReview {
    /// Every artifact declares where its wording came from (a source ref).
    pub every_artifact_declares_source: bool,
    /// Rendered artifacts declare product version and build context.
    pub rendered_artifacts_declare_version_and_build: bool,
    /// Screenshots/demos declare capture posture and caption-sync state.
    pub screenshots_declare_capture_posture_and_sync: bool,
    /// A caption never implies live/stable/current truth without metadata.
    pub captions_never_imply_live_without_metadata: bool,
    /// Export/report headings pair a human label with a locale-neutral machine code.
    pub headings_pair_human_label_with_machine_code: bool,
    /// Variable-rich strings carry a placeholder note per token.
    pub variable_rich_strings_carry_placeholder_notes: bool,
    /// Count placeholders declare plural rules.
    pub count_placeholders_declare_plural_rules: bool,
    /// Glossary / enumerated tokens use stable glossary refs.
    pub glossary_tokens_use_stable_refs: bool,
    /// Placeholder tokens resolve by id, not by position.
    pub placeholder_tokens_resolve_by_id_not_position: bool,
    /// Localized prose never controls machine identity.
    pub localized_prose_never_controls_machine_identity: bool,
    /// Locale fallback posture is declared and disclosed.
    pub locale_fallback_declared_and_disclosed: bool,
    /// Release/help/support materials are never versionless or uncited.
    pub release_help_support_never_versionless: bool,
    /// A support export reconstructs the wording's provenance.
    pub support_export_reconstructs_provenance: bool,
    /// One catalog is the source of truth, not parallel metadata islands.
    pub one_catalog_not_parallel_metadata_islands: bool,
    /// Machine identity (ids, codes, token ids) stays locale-neutral.
    pub machine_identity_stays_locale_neutral: bool,
    /// Human prose localizes around the locale-neutral tokens.
    pub human_prose_localizes_around_tokens: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsConsumerProjection {
    /// Docs/help resolves content-ops metadata through the catalog.
    pub docs_help_resolves_through_catalog: bool,
    /// Release notes resolve content-ops metadata through the catalog.
    pub release_notes_resolve_through_catalog: bool,
    /// Support export uses the catalog metadata.
    pub support_export_uses_catalog_metadata: bool,
    /// The screenshot/demo pipeline declares metadata through the catalog.
    pub screenshot_demo_pipeline_declares_metadata: bool,
    /// CLI/help reuses the catalog metadata.
    pub cli_help_reuses_metadata: bool,
    /// Report headings reuse the locale-neutral machine codes.
    pub report_headings_pair_human_and_machine_codes: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every entry.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every entry.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`ContentOpsMetadataCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentOpsMetadataCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy (e.g. `en`).
    pub reference_locale: String,
    /// Content-ops metadata entries.
    pub entries: Vec<ContentOpsEntry>,
    /// Shared reuse entry ids that must span multiple consumers.
    pub shared_reuse_entry_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: ContentOpsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ContentOpsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ContentOpsProofFreshness,
    /// Release posture.
    pub release_posture: ContentOpsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe content-ops metadata catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentOpsMetadataCatalog {
    /// Record kind; must equal [`CONTENT_OPS_METADATA_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CONTENT_OPS_METADATA_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy.
    pub reference_locale: String,
    /// Closed artifact-kind inventory (locale-neutral tokens).
    pub kind_inventory: Vec<String>,
    /// Closed reuse-consumer inventory (locale-neutral tokens).
    pub consumer_inventory: Vec<String>,
    /// Closed capture-posture inventory (locale-neutral tokens).
    pub capture_posture_inventory: Vec<String>,
    /// Closed caption-sync-state inventory (locale-neutral tokens).
    pub caption_sync_state_inventory: Vec<String>,
    /// Closed locale-fallback-strategy inventory (locale-neutral tokens).
    pub fallback_strategy_inventory: Vec<String>,
    /// Closed placeholder-kind inventory (locale-neutral tokens).
    pub placeholder_kind_inventory: Vec<String>,
    /// Closed token-fidelity-class inventory (locale-neutral tokens).
    pub token_fidelity_class_inventory: Vec<String>,
    /// Closed translator-note-class inventory (locale-neutral tokens).
    pub translator_note_class_inventory: Vec<String>,
    /// Content-ops metadata entries.
    pub entries: Vec<ContentOpsEntry>,
    /// Shared reuse entry ids that must span multiple consumers.
    pub shared_reuse_entry_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: ContentOpsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ContentOpsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ContentOpsProofFreshness,
    /// Release posture.
    pub release_posture: ContentOpsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ContentOpsMetadataCatalog {
    /// Builds a catalog packet from lane input, filling the closed inventories from
    /// the canonical enum token lists.
    pub fn new(input: ContentOpsMetadataCatalogInput) -> Self {
        Self {
            record_kind: CONTENT_OPS_METADATA_CATALOG_RECORD_KIND.to_owned(),
            schema_version: CONTENT_OPS_METADATA_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            kind_inventory: token_list(&ContentArtifactKind::ALL, ContentArtifactKind::as_str),
            consumer_inventory: token_list(&ContentOpsConsumer::ALL, ContentOpsConsumer::as_str),
            capture_posture_inventory: token_list(&CapturePosture::ALL, CapturePosture::as_str),
            caption_sync_state_inventory: token_list(
                &CaptionSyncState::ALL,
                CaptionSyncState::as_str,
            ),
            fallback_strategy_inventory: token_list(
                &LocaleFallbackStrategy::ALL,
                LocaleFallbackStrategy::as_str,
            ),
            placeholder_kind_inventory: token_list(&PlaceholderKind::ALL, PlaceholderKind::as_str),
            token_fidelity_class_inventory: token_list(
                &TokenFidelityClass::ALL,
                TokenFidelityClass::as_str,
            ),
            translator_note_class_inventory: token_list(
                &TranslatorNoteClass::ALL,
                TranslatorNoteClass::as_str,
            ),
            entries: input.entries,
            shared_reuse_entry_ids: input.shared_reuse_entry_ids,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves an entry by id.
    pub fn entry(&self, entry_id: &str) -> Option<&ContentOpsEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// All entries of an artifact kind, in catalog order.
    pub fn entries_for_kind(&self, kind: ContentArtifactKind) -> Vec<&ContentOpsEntry> {
        self.entries.iter().filter(|e| e.kind == kind).collect()
    }

    /// Renders the deterministic provenance line for an entry so docs/help, support,
    /// release, and screenshot/demo surfaces can explain where the wording came from
    /// and which command / source / version / build it reflects. Returns `None` if
    /// the entry id is unknown.
    pub fn render_provenance(&self, entry_id: &str) -> Option<String> {
        let entry = self.entry(entry_id)?;
        let vc = &entry.version_context;
        let mut out = format!(
            "{} [{}] — source: {}; command: {}; version: {}; build: {}",
            entry.canonical_text,
            entry.kind.as_str(),
            entry.source_ref,
            entry.command_ref.as_deref().unwrap_or("none"),
            vc.product_version_ref,
            vc.build_ref,
        );
        if vc.capture_posture.is_media() {
            out.push_str(&format!(
                "; posture: {}; sync: {}",
                vc.capture_posture.as_str(),
                vc.caption_sync_state.as_str()
            ));
        }
        if let Some(field) = &entry.machine_field_name {
            out.push_str(&format!("; field: {field}"));
        }
        if let Some(target) = &entry.target_string_ref {
            out.push_str(&format!("; target: {target}"));
        }
        Some(out)
    }

    /// Maps each entry id to the distinct reuse consumers that reconstruct it.
    pub fn cross_consumer_reuse(&self) -> BTreeMap<String, BTreeSet<&'static str>> {
        let mut reuse: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
        for entry in &self.entries {
            let bucket = reuse.entry(entry.entry_id.clone()).or_default();
            for consumer in &entry.consumers {
                bucket.insert(consumer.as_str());
            }
        }
        reuse
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<ContentOpsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CONTENT_OPS_METADATA_CATALOG_RECORD_KIND {
            violations.push(ContentOpsViolation::WrongRecordKind);
        }
        if self.schema_version != CONTENT_OPS_METADATA_CATALOG_SCHEMA_VERSION {
            violations.push(ContentOpsViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ContentOpsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_entries(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_reuse(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("content-ops metadata catalog serializes"),
        ) {
            violations.push(ContentOpsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("content-ops metadata catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Content-Ops Metadata: Snippets, Headings, Captions, and Translator Notes\n\n",
        );
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!("- Entries: {}\n", self.entries.len()));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        for kind in ContentArtifactKind::ALL {
            let entries = self.entries_for_kind(kind);
            if entries.is_empty() {
                continue;
            }
            out.push_str(&format!("\n## {}\n\n", kind.as_str()));
            for entry in entries {
                if let Some(provenance) = self.render_provenance(&entry.entry_id) {
                    out.push_str(&format!("- `{}` — {}\n", entry.entry_id, provenance));
                }
                out.push_str(&format!(
                    "  - Consumers: {}\n",
                    entry
                        .consumers
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
                for note in &entry.placeholder_notes {
                    out.push_str(&format!(
                        "  - placeholder `{}` ({} / {}): {}\n",
                        note.placeholder,
                        note.kind.as_str(),
                        note.fidelity.as_str(),
                        note.semantic
                    ));
                }
            }
        }

        out.push_str("\n## Cross-consumer entry reuse\n\n");
        for (entry_id, consumers) in self.cross_consumer_reuse() {
            out.push_str(&format!(
                "- `{}`: {}\n",
                entry_id,
                consumers.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum ContentOpsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ContentOpsViolation>),
}

impl fmt::Display for ContentOpsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "content-ops metadata catalog export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "content-ops metadata catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ContentOpsArtifactError {}

/// Validation failures emitted by [`ContentOpsMetadataCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentOpsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A closed inventory drifted from the canonical token list.
    InventoryDrift,
    /// An entry is incomplete (text, source, consumers).
    EntryIncomplete,
    /// An entry id, machine field name, command ref, or placeholder token id is not
    /// locale-neutral.
    EntryTokenNotLocaleNeutral,
    /// An entry id or machine field name is duplicated.
    DuplicateEntry,
    /// An export/report heading is missing its machine field name.
    HeadingMissingMachineField,
    /// A translator note is missing its class, target ref, or placeholder notes.
    TranslatorNoteIncomplete,
    /// A rendered artifact is missing required version/build context.
    MissingVersionContext,
    /// A screenshot/demo caption is missing capture posture, sync state, or the
    /// mocked-versus-live disclosure.
    CaptionPostureUndeclared,
    /// A variable-rich rendered string has a placeholder without a matching note.
    PlaceholderNoteMissing,
    /// A placeholder note is incomplete.
    PlaceholderNoteIncomplete,
    /// A count placeholder is missing its plural-rule ref.
    PluralRuleMissing,
    /// A glossary / enumerated token is missing its glossary ref.
    GlossaryRefMissing,
    /// A locale fallback posture is incomplete or inconsistent.
    LocaleFallbackIncomplete,
    /// An artifact kind, consumer, capture posture, or fallback strategy is never
    /// represented.
    CoverageGap,
    /// A shared reuse entry does not span enough consumers.
    SharedEntryReuseInsufficient,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ContentOpsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::EntryIncomplete => "entry_incomplete",
            Self::EntryTokenNotLocaleNeutral => "entry_token_not_locale_neutral",
            Self::DuplicateEntry => "duplicate_entry",
            Self::HeadingMissingMachineField => "heading_missing_machine_field",
            Self::TranslatorNoteIncomplete => "translator_note_incomplete",
            Self::MissingVersionContext => "missing_version_context",
            Self::CaptionPostureUndeclared => "caption_posture_undeclared",
            Self::PlaceholderNoteMissing => "placeholder_note_missing",
            Self::PlaceholderNoteIncomplete => "placeholder_note_incomplete",
            Self::PluralRuleMissing => "plural_rule_missing",
            Self::GlossaryRefMissing => "glossary_ref_missing",
            Self::LocaleFallbackIncomplete => "locale_fallback_incomplete",
            Self::CoverageGap => "coverage_gap",
            Self::SharedEntryReuseInsufficient => "shared_entry_reuse_insufficient",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in catalog export.
pub fn current_content_ops_metadata_catalog_export(
) -> Result<ContentOpsMetadataCatalog, ContentOpsArtifactError> {
    let packet: ContentOpsMetadataCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-content-ops-proof/support_export.json"
    )))
    .map_err(ContentOpsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ContentOpsArtifactError::Validation(violations))
    }
}

/// Extracts the distinct `{token}` placeholders in a rendered string, in order of
/// first appearance.
pub fn extract_placeholders(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            if let Some(rel_end) = text[i..].find('}') {
                let token = text[i..i + rel_end + 1].to_owned();
                if token.len() > 2 && !out.contains(&token) {
                    out.push(token);
                }
                i += rel_end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// True when `token` is a locale-neutral machine identifier: non-empty and only
/// lowercase ascii letters, digits, `_`, and `.`.
fn is_locale_neutral(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
}

fn token_list<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| as_str(*t).to_owned()).collect()
}

fn validate_source_contracts(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CONTENT_OPS_METADATA_CATALOG_SCHEMA_REF,
        CONTENT_OPS_METADATA_CATALOG_DOC_REF,
        CONTENT_OPS_CONTRACT_REF,
        MESSAGE_PLACEHOLDER_SCHEMA_REF,
        LATE_COPY_CHANGE_SCHEMA_REF,
        NAMING_LABEL_CONTRACT_REF,
        COUNT_SCOPE_GRAMMAR_REF,
        LOCALE_FALLBACK_CONTRACT_REF,
        CONTROLLED_GLOSSARY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ContentOpsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    if packet.kind_inventory != token_list(&ContentArtifactKind::ALL, ContentArtifactKind::as_str)
        || packet.consumer_inventory
            != token_list(&ContentOpsConsumer::ALL, ContentOpsConsumer::as_str)
        || packet.capture_posture_inventory
            != token_list(&CapturePosture::ALL, CapturePosture::as_str)
        || packet.caption_sync_state_inventory
            != token_list(&CaptionSyncState::ALL, CaptionSyncState::as_str)
        || packet.fallback_strategy_inventory
            != token_list(&LocaleFallbackStrategy::ALL, LocaleFallbackStrategy::as_str)
        || packet.placeholder_kind_inventory
            != token_list(&PlaceholderKind::ALL, PlaceholderKind::as_str)
        || packet.token_fidelity_class_inventory
            != token_list(&TokenFidelityClass::ALL, TokenFidelityClass::as_str)
        || packet.translator_note_class_inventory
            != token_list(&TranslatorNoteClass::ALL, TranslatorNoteClass::as_str)
    {
        violations.push(ContentOpsViolation::InventoryDrift);
    }
}

fn validate_entries(packet: &ContentOpsMetadataCatalog, violations: &mut Vec<ContentOpsViolation>) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_fields: BTreeSet<&str> = BTreeSet::new();

    for entry in &packet.entries {
        if entry.canonical_text.trim().is_empty()
            || entry.source_ref.trim().is_empty()
            || entry.consumers.is_empty()
        {
            violations.push(ContentOpsViolation::EntryIncomplete);
        }
        if !is_locale_neutral(&entry.entry_id) {
            violations.push(ContentOpsViolation::EntryTokenNotLocaleNeutral);
        }
        if let Some(field) = &entry.machine_field_name {
            if !is_locale_neutral(field) {
                violations.push(ContentOpsViolation::EntryTokenNotLocaleNeutral);
            }
            if !seen_fields.insert(field.as_str()) {
                violations.push(ContentOpsViolation::DuplicateEntry);
            }
        }
        if let Some(command) = &entry.command_ref {
            if !is_locale_neutral(command) {
                violations.push(ContentOpsViolation::EntryTokenNotLocaleNeutral);
            }
        }
        if !seen_ids.insert(entry.entry_id.as_str()) {
            violations.push(ContentOpsViolation::DuplicateEntry);
        }

        validate_entry_kind_rules(entry, violations);
        validate_entry_version_context(entry, violations);
        validate_entry_placeholders(entry, violations);
        validate_entry_locale_fallback(entry, violations);
    }
}

fn validate_entry_kind_rules(entry: &ContentOpsEntry, violations: &mut Vec<ContentOpsViolation>) {
    // An export/report heading pairs a human label with a locale-neutral machine code.
    if entry.kind == ContentArtifactKind::ExportReportHeading {
        match &entry.machine_field_name {
            Some(field) if !field.trim().is_empty() => {}
            _ => violations.push(ContentOpsViolation::HeadingMissingMachineField),
        }
    }

    // A translator note carries a class, a target string ref, and placeholder notes.
    if entry.kind == ContentArtifactKind::TranslatorNote {
        let class_ok = entry.translator_note_class.is_some();
        let target_ok = entry
            .target_string_ref
            .as_deref()
            .map(|t| !t.trim().is_empty())
            .unwrap_or(false);
        if !class_ok || !target_ok || entry.placeholder_notes.is_empty() {
            violations.push(ContentOpsViolation::TranslatorNoteIncomplete);
        }
    }
}

fn validate_entry_version_context(
    entry: &ContentOpsEntry,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let vc = &entry.version_context;
    // No versionless release/support truth: rendered artifacts and any release/support
    // path entry declare a product version and build ref.
    if (entry.kind.requires_version_context() || entry.release_support_path)
        && (vc.product_version_ref.trim().is_empty() || vc.build_ref.trim().is_empty())
    {
        violations.push(ContentOpsViolation::MissingVersionContext);
    }

    // A screenshot/demo caption can never imply live/stable/current truth without
    // metadata: it declares a real capture posture, a real caption-sync state, and the
    // mocked-versus-live disclosure.
    if entry.kind == ContentArtifactKind::ScreenshotDemoCaption
        && (!vc.capture_posture.is_media()
            || vc.caption_sync_state == CaptionSyncState::NotApplicable
            || !vc.mocked_versus_live_disclosed)
    {
        violations.push(ContentOpsViolation::CaptionPostureUndeclared);
    }
}

fn validate_entry_placeholders(entry: &ContentOpsEntry, violations: &mut Vec<ContentOpsViolation>) {
    // Variable-rich rendered strings: every placeholder in the text resolves to a note.
    if entry.kind.canonical_text_is_rendered() {
        let declared: BTreeSet<&str> = entry
            .placeholder_notes
            .iter()
            .map(|n| n.placeholder.as_str())
            .collect();
        for placeholder in extract_placeholders(&entry.canonical_text) {
            if !declared.contains(placeholder.as_str()) {
                violations.push(ContentOpsViolation::PlaceholderNoteMissing);
                break;
            }
        }
    }

    for note in &entry.placeholder_notes {
        if note.placeholder.trim().is_empty()
            || note.semantic.trim().is_empty()
            || note.fallback.trim().is_empty()
            || !note.placeholder.starts_with('{')
            || !note.placeholder.ends_with('}')
        {
            violations.push(ContentOpsViolation::PlaceholderNoteIncomplete);
        }
        if !is_locale_neutral(&note.token_id) {
            violations.push(ContentOpsViolation::EntryTokenNotLocaleNeutral);
        }
        if note.kind.requires_plural_rule()
            && note
                .plural_rule_ref
                .as_deref()
                .map(|r| r.trim().is_empty())
                .unwrap_or(true)
        {
            violations.push(ContentOpsViolation::PluralRuleMissing);
        }
        if note.kind.requires_glossary_ref()
            && note
                .glossary_term_ref
                .as_deref()
                .map(|r| r.trim().is_empty())
                .unwrap_or(true)
        {
            violations.push(ContentOpsViolation::GlossaryRefMissing);
        }
    }
}

fn validate_entry_locale_fallback(
    entry: &ContentOpsEntry,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let fallback = &entry.locale_fallback;
    if fallback.default_locale.trim().is_empty() || fallback.fallback_chain.is_empty() {
        violations.push(ContentOpsViolation::LocaleFallbackIncomplete);
        return;
    }
    match fallback.strategy {
        LocaleFallbackStrategy::PolicyBlocked => {
            // A policy-blocked fallback names the policy ref and discloses the block.
            if fallback
                .policy_block_ref
                .as_deref()
                .map(|r| r.trim().is_empty())
                .unwrap_or(true)
                || !fallback.non_authoritative_disclosed
            {
                violations.push(ContentOpsViolation::LocaleFallbackIncomplete);
            }
        }
        _ => {
            // Any non-blocked chain terminates at the source language and discloses a
            // non-authoritative fallback.
            if fallback.fallback_chain.last() != Some(&fallback.default_locale)
                || !fallback.non_authoritative_disclosed
            {
                violations.push(ContentOpsViolation::LocaleFallbackIncomplete);
            }
        }
    }
}

fn validate_coverage(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let kinds: BTreeSet<ContentArtifactKind> = packet.entries.iter().map(|e| e.kind).collect();
    let consumers: BTreeSet<ContentOpsConsumer> = packet
        .entries
        .iter()
        .flat_map(|e| e.consumers.iter().copied())
        .collect();
    let postures: BTreeSet<CapturePosture> = packet
        .entries
        .iter()
        .map(|e| e.version_context.capture_posture)
        .collect();
    let strategies: BTreeSet<LocaleFallbackStrategy> = packet
        .entries
        .iter()
        .map(|e| e.locale_fallback.strategy)
        .collect();

    let kinds_covered = ContentArtifactKind::ALL.iter().all(|k| kinds.contains(k));
    let consumers_covered = ContentOpsConsumer::ALL
        .iter()
        .all(|c| consumers.contains(c));
    let postures_covered = CapturePosture::ALL.iter().all(|p| postures.contains(p));
    let strategies_covered = LocaleFallbackStrategy::ALL
        .iter()
        .all(|s| strategies.contains(s));

    if !kinds_covered || !consumers_covered || !postures_covered || !strategies_covered {
        violations.push(ContentOpsViolation::CoverageGap);
    }
}

fn validate_shared_reuse(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    if packet.shared_reuse_entry_ids.is_empty() {
        violations.push(ContentOpsViolation::SharedEntryReuseInsufficient);
        return;
    }
    let reuse = packet.cross_consumer_reuse();
    for entry_id in &packet.shared_reuse_entry_ids {
        let spans = reuse.get(entry_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_ENTRY_MIN_REUSE_CONSUMERS {
            violations.push(ContentOpsViolation::SharedEntryReuseInsufficient);
        }
    }
}

fn validate_trust_review(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.every_artifact_declares_source,
        review.rendered_artifacts_declare_version_and_build,
        review.screenshots_declare_capture_posture_and_sync,
        review.captions_never_imply_live_without_metadata,
        review.headings_pair_human_label_with_machine_code,
        review.variable_rich_strings_carry_placeholder_notes,
        review.count_placeholders_declare_plural_rules,
        review.glossary_tokens_use_stable_refs,
        review.placeholder_tokens_resolve_by_id_not_position,
        review.localized_prose_never_controls_machine_identity,
        review.locale_fallback_declared_and_disclosed,
        review.release_help_support_never_versionless,
        review.support_export_reconstructs_provenance,
        review.one_catalog_not_parallel_metadata_islands,
        review.machine_identity_stays_locale_neutral,
        review.human_prose_localizes_around_tokens,
    ] {
        if !ok {
            violations.push(ContentOpsViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.docs_help_resolves_through_catalog,
        projection.release_notes_resolve_through_catalog,
        projection.support_export_uses_catalog_metadata,
        projection.screenshot_demo_pipeline_declares_metadata,
        projection.cli_help_reuses_metadata,
        projection.report_headings_pair_human_and_machine_codes,
    ] {
        if !ok {
            violations.push(ContentOpsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ContentOpsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &ContentOpsMetadataCatalog,
    violations: &mut Vec<ContentOpsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(ContentOpsViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Rewrites a human prose run into a pseudo-localized form by wrapping it in locale
/// markers. Machine-facing identity (ids, codes, token ids, patterns) never passes
/// through this function, so a localized overlay can never fork the meaning of an
/// entry.
pub fn pseudo_localize_prose(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.is_empty() {
        return prose.to_owned();
    }
    let leading = &prose[..prose.len() - prose.trim_start().len()];
    let trailing = &prose[prose.trim_end().len()..];
    format!("{leading}\u{27e6}{trimmed}\u{27e7}{trailing}")
}
