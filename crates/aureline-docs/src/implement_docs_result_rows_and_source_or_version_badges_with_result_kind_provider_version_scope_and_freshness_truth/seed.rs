//! Canonical seed builders for the M5 docs-result-row / source-version-badge
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical docs-result-row-primitive packet.
pub const M5_DOCS_RESULT_ROW_PRIMITIVE_PACKET_ID: &str =
    "m5-docs-result-row-and-source-version-badge-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full result-row state.
#[allow(clippy::too_many_arguments)]
fn case(
    title_repr: &str,
    result_kind: M5DocsResultKind,
    corpus_class: M5DocsCorpusClass,
    source_provider: M5DocsSourceProvider,
    match_state: M5DocsMatchState,
    override_reason: M5DocsOverrideReason,
    symbol_match_confidence: M5DocsSymbolMatchConfidence,
    version_scope: M5DocsVersionScope,
    freshness_state: M5DocsFreshnessState,
    open_action_target_repr: &str,
) -> M5DocsResultRowResolutionCase {
    M5DocsResultRowResolutionCase::resolved(M5DocsResultRowResolutionInput {
        title_repr: title_repr.to_owned(),
        result_kind,
        corpus_class,
        source_provider,
        match_state,
        override_reason,
        symbol_match_confidence,
        version_scope,
        freshness_state,
        open_action_target_repr: open_action_target_repr.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, result-kind,
/// corpus, provider, source-badge, match-state, override-reason, rank-factor,
/// confidence, version, freshness, posture, export-field, and accessibility parity
/// every consumer carries.
fn base_row(
    consumer_surface: M5DocsResultConsumerSurface,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5DocsResultRowResolutionCase>,
) -> M5DocsResultRow {
    M5DocsResultRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DocsResultRowAnatomyPart::ALL.to_vec(),
        result_kinds: M5DocsResultKind::ALL.to_vec(),
        corpus_classes: M5DocsCorpusClass::ALL.to_vec(),
        source_providers: M5DocsSourceProvider::ALL.to_vec(),
        source_badge_classes: M5DocsSourceBadgeClass::ALL.to_vec(),
        match_states: M5DocsMatchState::ALL.to_vec(),
        override_reasons: M5DocsOverrideReason::ALL.to_vec(),
        rank_factors: M5DocsRankFactor::ALL.to_vec(),
        symbol_match_confidences: M5DocsSymbolMatchConfidence::ALL.to_vec(),
        version_scopes: M5DocsVersionScope::ALL.to_vec(),
        freshness_states: M5DocsFreshnessState::ALL.to_vec(),
        freshness_postures: M5DocsResultFreshnessPosture::ALL.to_vec(),
        export_fields: M5DocsResultRowExportField::ALL.to_vec(),
        accessibility_routes: M5DocsAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5DocsConsumerSurface::DocsBrowserUi,
            M5DocsConsumerSurface::HelpAbout,
            M5DocsConsumerSurface::SearchPalette,
            M5DocsConsumerSurface::OnboardingTour,
            M5DocsConsumerSurface::AiContextPanel,
            M5DocsConsumerSurface::SupportExport,
            M5DocsConsumerSurface::CliInspect,
            M5DocsConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5DocsDowngradeTrigger::SourceProviderMasked,
            M5DocsDowngradeTrigger::VersionScopeUnstated,
            M5DocsDowngradeTrigger::ProjectOverrideReasonHidden,
            M5DocsDowngradeTrigger::FreshnessHidden,
            M5DocsDowngradeTrigger::MirroredOrCachedShownAsLive,
            M5DocsDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_RESULT_ROW_SCHEMA_REF,
            M5_DOCS_RESULT_ROW_SOURCE_RESULT_REF,
            M5_DOCS_RESULT_ROW_SOURCE_PRECEDENCE_REF,
        ]),
        example_resolutions,
        masks_source_or_version: false,
        shows_cached_or_stale_as_live: false,
        invents_private_result_grammar: false,
        hides_rank_reason: false,
    }
}

// Sequential pushes preserve the numbered consumer-matrix narrative below.
#[allow(clippy::vec_init_then_push)]
fn result_rows() -> Vec<M5DocsResultRow> {
    use M5DocsCorpusClass as Corpus;
    use M5DocsFreshnessState as Fresh;
    use M5DocsMatchState as Match;
    use M5DocsOverrideReason as Override;
    use M5DocsResultKind as Kind;
    use M5DocsSourceProvider as Source;
    use M5DocsSymbolMatchConfidence as Conf;
    use M5DocsVersionScope as Scope;

    let mut rows = Vec::new();

    // 1. Docs-browser result — a live-ready first-party doc, a cached API reference
    //    whose local freshness outranked a staler mirror (rank-reason disclosure), and
    //    a project-specific codebase symbol (local-project badge) that took precedence.
    rows.push(base_row(
        M5DocsResultConsumerSurface::DocsBrowserResult,
        M5DocsQualificationClass::Stable,
        "Docs-browser result owner",
        "The docs-browser result list renders the shared primitive so a live first-party doc reads as a first-party reference, a cached API reference reads as cached-explicit-not-live with a mirror-freshness rank reason, and a project-specific codebase symbol reads as local project docs that took precedence over vendor docs",
        "evidence:m5-docs-result-browser:001",
        vec![
            case(
                "Getting started",
                Kind::DocPage,
                Corpus::FirstPartyDocs,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Conf::NotSymbolScoped,
                Scope::ExactVersionMatch,
                Fresh::LiveCurrent,
                "open:docs/getting-started",
            ),
            case(
                "Client::new",
                Kind::ApiSymbolEntry,
                Corpus::ApiReference,
                Source::BundledLocal,
                Match::CachedMatch,
                Override::LocalFreshnessOverride,
                Conf::StrongMatch,
                Scope::LatestStable,
                Fresh::CachedOffline,
                "open:api/client-new",
            ),
            case(
                "resolve_run_context",
                Kind::CodeSymbolAnchor,
                Corpus::CodebaseSymbol,
                Source::FirstPartyHosted,
                Match::ProjectSpecificMatch,
                Override::ProjectPinnedOverride,
                Conf::ExactSymbolMatch,
                Scope::ProjectSpecific,
                Fresh::RecentlySynced,
                "open:symbol/resolve-run-context",
            ),
        ],
    ));

    // 2. AI-answer citation — an AI-derived explanation at a nearby version
    //    (version-adjacency rank reason), a stale live-vendor citation whose vendor
    //    source was unavailable, and a mirrored API reference decided by policy.
    rows.push(base_row(
        M5DocsResultConsumerSurface::AiAnswerCitation,
        M5DocsQualificationClass::Stable,
        "AI-answer citation owner",
        "The AI-answer citation renders the shared primitive so an AI-derived explanation reads as ai-derived-explanation with a version-adjacency rank reason, a stale vendor doc reads as live-vendor-upstream flagged stale with a vendor-fallback rank reason, and a mirrored API reference reads as mirrored-explicit-not-live with a policy-scoped rank reason — never a citation shown as live when it is cached, mirrored, or stale",
        "evidence:m5-docs-result-ai:001",
        vec![
            case(
                "How retries work",
                Kind::GuideSection,
                Corpus::GuideTutorial,
                Source::AiDerived,
                Match::NearbyMatch,
                Override::NoOverride,
                Conf::HeuristicMatch,
                Scope::NearbyVersion,
                Fresh::RecentlySynced,
                "open:ai/how-retries-work",
            ),
            case(
                "Vendor SDK guide",
                Kind::DocPage,
                Corpus::VendorDependency,
                Source::ThirdPartyHosted,
                Match::StaleMatch,
                Override::VendorSourceUnavailable,
                Conf::NotSymbolScoped,
                Scope::PinnedRange,
                Fresh::StaleExpired,
                "open:vendor/sdk-guide",
            ),
            case(
                "Rate limits",
                Kind::ApiSymbolEntry,
                Corpus::ApiReference,
                Source::MirroredRegistry,
                Match::MirroredMatch,
                Override::PolicyScopedOverride,
                Conf::PartialMatch,
                Scope::Unversioned,
                Fresh::CachedOffline,
                "open:api/rate-limits",
            ),
        ],
    ));

    // 3. Onboarding step reference — an extension-contributed guide chosen by explicit
    //    user preference, and a cached example snippet under default ranking.
    rows.push(base_row(
        M5DocsResultConsumerSurface::OnboardingStepReference,
        M5DocsQualificationClass::Stable,
        "Onboarding step reference owner",
        "The onboarding step reference renders the shared primitive so an extension-contributed guide reads as extension-contributed with an explicit-preference rank reason, while a cached example snippet under default ranking reads as cached-explicit-not-live with no rank-reason disclosure — the same badge/state vocabulary a docs-browser reader sees",
        "evidence:m5-docs-result-onboarding:001",
        vec![
            case(
                "Community setup guide",
                Kind::GuideSection,
                Corpus::CommunityContributed,
                Source::OfflineImport,
                Match::NearbyMatch,
                Override::ExplicitUserPreference,
                Conf::NotSymbolScoped,
                Scope::NearbyVersion,
                Fresh::RecentlySynced,
                "open:onboarding/community-setup",
            ),
            case(
                "Config example",
                Kind::ExampleSnippet,
                Corpus::GuideTutorial,
                Source::BundledLocal,
                Match::CachedMatch,
                Override::NoOverride,
                Conf::NotSymbolScoped,
                Scope::LatestStable,
                Fresh::CachedOffline,
                "open:onboarding/config-example",
            ),
        ],
    ));

    // 4. Support answer result — a first-party changelog whose freshness is unknown,
    //    and a cached workspace-spec codebase symbol with an unresolved anchor.
    rows.push(base_row(
        M5DocsResultConsumerSurface::SupportAnswerResult,
        M5DocsQualificationClass::Stable,
        "Support answer result owner",
        "The support answer result renders the shared primitive so a first-party changelog entry with unknown freshness reads as first-party-reference with a freshness-unknown posture, while a cached codebase symbol whose anchor is unresolved reads as a workspace spec that is cached-explicit-not-live — both keep source and version visible without inferring certainty",
        "evidence:m5-docs-result-support:001",
        vec![
            case(
                "Release 1.4 notes",
                Kind::ChangelogEntry,
                Corpus::ReleaseNotesChangelog,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Conf::NotSymbolScoped,
                Scope::Unversioned,
                Fresh::UnknownFreshness,
                "open:support/release-1-4",
            ),
            case(
                "internal::cache_key",
                Kind::CodeSymbolAnchor,
                Corpus::CodebaseSymbol,
                Source::OfflineImport,
                Match::CachedMatch,
                Override::NoOverride,
                Conf::UnresolvedSymbol,
                Scope::PinnedRange,
                Fresh::CachedOffline,
                "open:symbol/cache-key",
            ),
        ],
    ));

    // 5. CLI result list — a first-party doc whose declared live freshness is
    //    overridden to stale because the match is stale (the honesty guard), and a
    //    clean live-ready API symbol (headless parity proof).
    rows.push(base_row(
        M5DocsResultConsumerSurface::CliResultList,
        M5DocsQualificationClass::Stable,
        "CLI result-list owner",
        "The CLI result list renders the shared primitive so a first-party doc whose match is stale reads as stale-flagged even when its declared freshness is live — never shown as live — while a clean API-symbol result reads as first-party-reference current-live, the same badge/state vocabulary a docs-browser reader sees, reachable without a pointer",
        "evidence:m5-docs-result-cli:001",
        vec![
            case(
                "Deprecated flag",
                Kind::DocPage,
                Corpus::FirstPartyDocs,
                Source::FirstPartyHosted,
                Match::StaleMatch,
                Override::NoOverride,
                Conf::NotSymbolScoped,
                Scope::LatestStable,
                Fresh::LiveCurrent,
                "open:cli/deprecated-flag",
            ),
            case(
                "Client::send",
                Kind::ApiSymbolEntry,
                Corpus::ApiReference,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Conf::ExactSymbolMatch,
                Scope::ExactVersionMatch,
                Fresh::LiveCurrent,
                "open:api/client-send",
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5DocsResultRowGovernanceReview {
    M5DocsResultRowGovernanceReview {
        one_primitive_carries_result_truth: true,
        source_and_version_always_shown: true,
        local_vs_upstream_distinguishable_at_row_level: true,
        cached_or_stale_never_shown_as_live: true,
        version_freshness_visible_on_every_reuse: true,
        rank_reason_stays_inspectable: true,
        badge_state_vocabulary_stable_across_surfaces: true,
        support_export_reconstructs_result_truth: true,
        no_surface_invents_second_result_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsResultRowConsumerProjection {
    M5DocsResultRowConsumerProjection {
        result_surfaces_consume_shared_primitive: true,
        source_badge_reads_single_source: true,
        freshness_posture_reads_single_source: true,
        rank_reason_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DocsResultRowProofFreshness {
    M5DocsResultRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsResultRowReleasePosture {
    M5DocsResultRowReleasePosture {
        proof_packet_ref: M5_DOCS_RESULT_ROW_ARTIFACT_REF.to_owned(),
        result_row_audit_ref: M5_DOCS_RESULT_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_RESULT_ROW_SCHEMA_REF,
        M5_DOCS_RESULT_ROW_DOC_REF,
        M5_DOCS_RESULT_ROW_COMPONENT_MATRIX_REF,
        M5_DOCS_RESULT_ROW_SOURCE_RESULT_REF,
        M5_DOCS_RESULT_ROW_SOURCE_PRECEDENCE_REF,
    ])
}

/// Builds the canonical M5 docs-result-row-primitive packet.
pub fn seeded_m5_docs_result_row_primitive_packet() -> M5DocsResultRowPrimitivePacket {
    M5DocsResultRowPrimitivePacket::new(M5DocsResultRowPrimitivePacketInput {
        packet_id: M5_DOCS_RESULT_ROW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs-result-row and source-version-badge primitive: result kind, source provider, source-badge class, version scope, symbol-match confidence, freshness posture, and rank reason"
                .to_owned(),
        result_rows: result_rows(),
        vocabulary_set: M5DocsResultRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the onboarding step reference is held at Beta because a slice of
/// onboarding surfaces do not yet render the source-badge glyph on every profile;
/// every consumer stays visible.
pub fn seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed(
) -> M5DocsResultRowPrimitivePacket {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.packet_id =
        "m5-docs-result-row-and-source-version-badge-primitive:onboarding-beta:0001".to_owned();
    let row = packet
        .result_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsResultConsumerSurface::OnboardingStepReference)
        .expect("onboarding step reference row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-answer citation is narrowed to Preview pending
/// rank-reason-disclosure parity proof across every AI-context export path; every
/// consumer stays visible.
pub fn seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed(
) -> M5DocsResultRowPrimitivePacket {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.packet_id =
        "m5-docs-result-row-and-source-version-badge-primitive:ai-citation-preview:0001".to_owned();
    let row = packet
        .result_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsResultConsumerSurface::AiAnswerCitation)
        .expect("ai-answer citation row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
