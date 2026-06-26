//! Canonical seed builders for the content-ops metadata catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! localized / offline-mirror fixtures. The headless emitter and the inline tests
//! both call them so the in-code catalog, the artifact, and the fixtures never
//! drift.

use super::*;

/// Stable catalog id for the canonical content-ops metadata catalog.
pub const CONTENT_OPS_METADATA_CATALOG_ID: &str = "m5-content-ops-metadata-catalog:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Product version ref the seeded wording reflects.
const SEED_VERSION_REF: &str = "version.channel.stable.2026.06";

/// Build ref the seeded wording was authored / captured against.
const SEED_BUILD_REF: &str = "build.m5.content_ops.0001";

use CaptionSyncState as Cs;
use CapturePosture as Cp;
use ContentArtifactKind as Ak;
use ContentOpsConsumer as Co;
use LocaleFallbackStrategy as Fs;
use PlaceholderKind as Pk;
use TokenFidelityClass as Tf;
use TranslatorNoteClass as Tn;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a [`VersionContext`]. Non-media content uses [`CapturePosture::NotApplicable`]
/// / [`CaptionSyncState::NotApplicable`]; the mocked-versus-live disclosure tracks
/// whether the posture describes real captured media.
fn version_context(
    capture_posture: CapturePosture,
    caption_sync_state: CaptionSyncState,
) -> VersionContext {
    VersionContext {
        product_version_ref: SEED_VERSION_REF.to_owned(),
        build_ref: SEED_BUILD_REF.to_owned(),
        capture_posture,
        caption_sync_state,
        mocked_versus_live_disclosed: capture_posture.is_media(),
    }
}

#[allow(clippy::too_many_arguments)]
fn placeholder(
    placeholder: &str,
    token_id: &str,
    kind: PlaceholderKind,
    fidelity: TokenFidelityClass,
    semantic: &str,
    fallback: &str,
    bidi_isolation_required: bool,
    plural_rule_ref: Option<&str>,
    glossary_term_ref: Option<&str>,
) -> PlaceholderNote {
    PlaceholderNote {
        placeholder: placeholder.to_owned(),
        token_id: token_id.to_owned(),
        kind,
        fidelity,
        semantic: semantic.to_owned(),
        fallback: fallback.to_owned(),
        bidi_isolation_required,
        plural_rule_ref: plural_rule_ref.map(str::to_owned),
        glossary_term_ref: glossary_term_ref.map(str::to_owned),
    }
}

fn count_note(placeholder_token: &str, token_id: &str, semantic: &str) -> PlaceholderNote {
    placeholder(
        placeholder_token,
        token_id,
        Pk::Count,
        Tf::LocaleFormattedValue,
        semantic,
        "0",
        false,
        Some("plural.rule.cardinal"),
        None,
    )
}

fn scope_note() -> PlaceholderNote {
    placeholder(
        "{scope}",
        "scope",
        Pk::GlossaryTermToken,
        Tf::ControlledVocabularyTranslation,
        "The controlled scope term the count applies to; resolve through the controlled glossary, not a translator-local synonym.",
        "the selected scope",
        false,
        None,
        Some("glossary.term.scope"),
    )
}

fn build_note() -> PlaceholderNote {
    placeholder(
        "{build}",
        "build",
        Pk::VersionOrBuildToken,
        Tf::LiteralUnchanged,
        "The capture build identity; preserve it verbatim so the caption ties back to a build.",
        "the capture build",
        false,
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn fallback(
    default_locale: &str,
    strategy: LocaleFallbackStrategy,
    chain: &[&str],
    policy_block_ref: Option<&str>,
) -> LocaleFallback {
    LocaleFallback {
        default_locale: default_locale.to_owned(),
        strategy,
        fallback_chain: strings(chain),
        non_authoritative_disclosed: true,
        policy_block_ref: policy_block_ref.map(str::to_owned),
    }
}

#[allow(clippy::too_many_arguments)]
fn entry(
    entry_id: &str,
    kind: ContentArtifactKind,
    canonical_text: &str,
    machine_field_name: Option<&str>,
    source_ref: &str,
    command_ref: Option<&str>,
    version_context: VersionContext,
    placeholder_notes: Vec<PlaceholderNote>,
    translator_note_class: Option<TranslatorNoteClass>,
    target_string_ref: Option<&str>,
    locale_fallback: LocaleFallback,
    consumers: &[ContentOpsConsumer],
    release_support_path: bool,
) -> ContentOpsEntry {
    ContentOpsEntry {
        entry_id: entry_id.to_owned(),
        kind,
        canonical_text: canonical_text.to_owned(),
        machine_field_name: machine_field_name.map(str::to_owned),
        source_ref: source_ref.to_owned(),
        command_ref: command_ref.map(str::to_owned),
        version_context,
        placeholder_notes,
        translator_note_class,
        target_string_ref: target_string_ref.map(str::to_owned),
        locale_fallback,
        consumers: consumers.to_vec(),
        release_support_path,
    }
}

fn entries() -> Vec<ContentOpsEntry> {
    vec![
        // Docs/help snippet — variable-rich, cited, version-anchored, with placeholder
        // notes for both the count and the controlled scope term.
        entry(
            "entry.docs.project_doctor_findings",
            Ak::DocsHelpSnippet,
            "Project Doctor checked your workspace and found {count} findings in {scope}.",
            None,
            "glossary.term.project_doctor",
            Some("command.project.doctor"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![
                count_note(
                    "{count}",
                    "count",
                    "The number of findings; pluralize the localized noun, not the token.",
                ),
                scope_note(),
            ],
            None,
            None,
            fallback("en", Fs::SourceLanguageRoute, &["fr", "en"], None),
            &[Co::DocsHelp, Co::ReleaseNotes, Co::SupportExport, Co::CliHelp],
            true,
        ),
        // Docs/help snippet — no placeholders, nearest-locale fallback.
        entry(
            "entry.docs.open_source_before_apply",
            Ak::DocsHelpSnippet,
            "Open the source to review the proposed change before you apply it.",
            None,
            "glossary.term.open_source",
            Some("command.review.open_source"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![],
            None,
            None,
            fallback("en", Fs::NearestLocale, &["fr_ca", "fr", "en"], None),
            &[Co::DocsHelp, Co::CliHelp, Co::SupportExport],
            false,
        ),
        // Export/report heading — human label paired with a locale-neutral machine
        // field id; machine-token fallback when no localized prose is available.
        entry(
            "entry.heading.findings_by_severity",
            Ak::ExportReportHeading,
            "Findings by severity",
            Some("report.column.findings_by_severity"),
            "glossary.term.severity",
            Some("command.report.generate"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![],
            None,
            None,
            fallback("en", Fs::MachineToken, &["de", "en"], None),
            &[Co::SupportExport, Co::ReleaseNotes, Co::DocsHelp],
            true,
        ),
        // Export/report heading — variable-rich; the count heading carries a
        // placeholder note so the heading stays translation-safe.
        entry(
            "entry.heading.findings_exported_count",
            Ak::ExportReportHeading,
            "{count} findings exported",
            Some("report.heading.findings_exported_count"),
            "glossary.term.export",
            Some("command.report.export"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![count_note(
                "{count}",
                "count",
                "The number of exported findings; pluralize the localized noun, not the token.",
            )],
            None,
            None,
            fallback("en", Fs::SourceLanguageRoute, &["ja", "en"], None),
            &[Co::SupportExport, Co::ReleaseNotes],
            true,
        ),
        // Screenshot/demo caption — captured from a live run; in-sync, mocked-versus-live
        // disclosed, version + build declared.
        entry(
            "entry.caption.activity_center_live",
            Ak::ScreenshotDemoCaption,
            "Aureline shell showing the activity center.",
            None,
            "string.shell.activity_center_title",
            Some("command.window.activity_center"),
            version_context(Cp::Live, Cs::InSync),
            vec![],
            None,
            None,
            fallback("en", Fs::SourceLanguageRoute, &["es", "en"], None),
            &[Co::ReleaseNotes, Co::DocsHelp, Co::ScreenshotDemoPipeline],
            true,
        ),
        // Screenshot/demo caption — mocked sample data; posture disclosed so it never
        // implies live truth.
        entry(
            "entry.caption.demo_workspace_mocked",
            Ak::ScreenshotDemoCaption,
            "Demo workspace populated with sample data.",
            None,
            "string.demo.workspace_label",
            Some("command.window.workspace"),
            version_context(Cp::Mocked, Cs::InSync),
            vec![],
            None,
            None,
            fallback("en", Fs::NearestLocale, &["pt_br", "pt", "en"], None),
            &[Co::ReleaseNotes, Co::ScreenshotDemoPipeline],
            true,
        ),
        // Screenshot/demo caption — synthetic preview; pending caption-sync review, and
        // source-language fallback is policy-blocked with a named policy ref.
        entry(
            "entry.caption.patch_review_synthetic",
            Ak::ScreenshotDemoCaption,
            "Synthetic preview of the patch review surface.",
            None,
            "string.review.patch_review_title",
            Some("command.review.patch"),
            version_context(Cp::Synthetic, Cs::PendingReview),
            vec![],
            None,
            None,
            fallback(
                "en",
                Fs::PolicyBlocked,
                &["ar"],
                Some("policy.locale.source_language_block"),
            ),
            &[Co::ReleaseNotes, Co::ScreenshotDemoPipeline, Co::SupportExport],
            true,
        ),
        // Translator note — placeholder semantics for the docs/help snippet, attaching
        // the count and scope placeholder notes to the target string.
        entry(
            "entry.note.project_doctor_findings",
            Ak::TranslatorNote,
            "Keep {count} adjacent to the localized noun and pluralize the noun, not the token; {scope} is a controlled glossary term and must use its glossary ref.",
            None,
            "docs.copy.translation_safe_content_ops_contract",
            Some("command.project.doctor"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![
                count_note(
                    "{count}",
                    "count",
                    "The number of findings; pluralize the localized noun, not the token.",
                ),
                scope_note(),
            ],
            Some(Tn::PlaceholderSemantics),
            Some("entry.docs.project_doctor_findings"),
            fallback("en", Fs::SourceLanguageRoute, &["fr", "en"], None),
            &[Co::DocsHelp, Co::SupportExport],
            false,
        ),
        // Translator note — screenshot/demo caption governance, attaching the build
        // token note to the live caption.
        entry(
            "entry.note.caption_build_governance",
            Ak::TranslatorNote,
            "Caption must name the capture build {build} and stay in sync with the source surface; it must not imply live truth without it.",
            None,
            "docs.copy.translation_safe_content_ops_contract",
            Some("command.window.activity_center"),
            version_context(Cp::NotApplicable, Cs::NotApplicable),
            vec![build_note()],
            Some(Tn::ScreenshotOrDemoCaptionGovernance),
            Some("entry.caption.activity_center_live"),
            fallback("en", Fs::SourceLanguageRoute, &["es", "en"], None),
            &[Co::ScreenshotDemoPipeline, Co::ReleaseNotes, Co::SupportExport],
            false,
        ),
    ]
}

fn shared_reuse_entry_ids() -> Vec<String> {
    strings(&[
        "entry.docs.project_doctor_findings",
        "entry.heading.findings_by_severity",
        "entry.caption.activity_center_live",
    ])
}

fn trust_review() -> ContentOpsTrustReview {
    ContentOpsTrustReview {
        every_artifact_declares_source: true,
        rendered_artifacts_declare_version_and_build: true,
        screenshots_declare_capture_posture_and_sync: true,
        captions_never_imply_live_without_metadata: true,
        headings_pair_human_label_with_machine_code: true,
        variable_rich_strings_carry_placeholder_notes: true,
        count_placeholders_declare_plural_rules: true,
        glossary_tokens_use_stable_refs: true,
        placeholder_tokens_resolve_by_id_not_position: true,
        localized_prose_never_controls_machine_identity: true,
        locale_fallback_declared_and_disclosed: true,
        release_help_support_never_versionless: true,
        support_export_reconstructs_provenance: true,
        one_catalog_not_parallel_metadata_islands: true,
        machine_identity_stays_locale_neutral: true,
        human_prose_localizes_around_tokens: true,
    }
}

fn consumer_projection() -> ContentOpsConsumerProjection {
    ContentOpsConsumerProjection {
        docs_help_resolves_through_catalog: true,
        release_notes_resolve_through_catalog: true,
        support_export_uses_catalog_metadata: true,
        screenshot_demo_pipeline_declares_metadata: true,
        cli_help_reuses_metadata: true,
        report_headings_pair_human_and_machine_codes: true,
    }
}

fn proof_freshness() -> ContentOpsProofFreshness {
    ContentOpsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> ContentOpsReleasePosture {
    ContentOpsReleasePosture {
        release_packet_ref: "evidence:content-ops-metadata-catalog-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:content-ops-metadata-catalog-mirror-offline-packet:m5"
            .to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        CONTENT_OPS_METADATA_CATALOG_SCHEMA_REF,
        CONTENT_OPS_METADATA_CATALOG_DOC_REF,
        CONTENT_OPS_CONTRACT_REF,
        MESSAGE_PLACEHOLDER_SCHEMA_REF,
        LATE_COPY_CHANGE_SCHEMA_REF,
        NAMING_LABEL_CONTRACT_REF,
        COUNT_SCOPE_GRAMMAR_REF,
        LOCALE_FALLBACK_CONTRACT_REF,
        CONTROLLED_GLOSSARY_REF,
    ])
}

fn base_input() -> ContentOpsMetadataCatalogInput {
    ContentOpsMetadataCatalogInput {
        catalog_id: CONTENT_OPS_METADATA_CATALOG_ID.to_owned(),
        catalog_label:
            "Content-Ops Metadata for Docs/Help Snippets, Export/Report Headings, Captions, and Translator Notes"
                .to_owned(),
        reference_locale: "en".to_owned(),
        entries: entries(),
        shared_reuse_entry_ids: shared_reuse_entry_ids(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical content-ops metadata catalog.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_content_ops_metadata_catalog() -> ContentOpsMetadataCatalog {
    ContentOpsMetadataCatalog::new(base_input())
}

/// Builds a localized overlay of the canonical catalog.
///
/// Only the human prose changes: entry canonical text and placeholder semantics /
/// fallbacks are pseudo-localized, while every entry id, machine field name, command
/// ref, source ref, placeholder token (and token id), glossary / plural refs, locale
/// tags, and posture stay byte-for-byte identical. A localized overlay can never fork
/// a command id, an export field id, or a placeholder token into machine identity.
pub fn seeded_content_ops_metadata_catalog_localized() -> ContentOpsMetadataCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-content-ops-metadata-catalog:localized:0001".to_owned();
    input.catalog_label = format!("{} (localized overlay)", input.catalog_label);
    input.reference_locale = "qps-ploc".to_owned();
    for entry in &mut input.entries {
        entry.canonical_text = pseudo_localize_prose(&entry.canonical_text);
        for note in &mut entry.placeholder_notes {
            // The placeholder literal and token id are machine identity and never
            // localize; only the human-facing semantic and fallback prose do.
            note.semantic = pseudo_localize_prose(&note.semantic);
            note.fallback = pseudo_localize_prose(&note.fallback);
        }
    }
    ContentOpsMetadataCatalog::new(input)
}

/// Builds an offline-mirror variant of the canonical catalog.
///
/// The catalog identity and entries are unchanged; only the catalog id and the
/// mirror/offline release ref differ. This proves the catalog survives an offline
/// mirror without forking the meaning of any entry.
pub fn seeded_content_ops_metadata_catalog_offline_mirror() -> ContentOpsMetadataCatalog {
    let mut input = base_input();
    input.catalog_id = "m5-content-ops-metadata-catalog:offline-mirror:0001".to_owned();
    input.catalog_label = format!("{} (offline mirror)", input.catalog_label);
    input.release_posture.release_packet_ref =
        "evidence:content-ops-metadata-catalog-release-packet:m5:mirror".to_owned();
    ContentOpsMetadataCatalog::new(input)
}
