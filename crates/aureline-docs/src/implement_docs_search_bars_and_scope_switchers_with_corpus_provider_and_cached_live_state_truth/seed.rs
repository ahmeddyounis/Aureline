//! Canonical seed builders for the M5 docs-search-bar / scope-switcher primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, the worked resolutions, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical docs-search-primitive packet.
pub const M5_DOCS_SEARCH_PRIMITIVE_PACKET_ID: &str =
    "m5-docs-search-bar-and-scope-switcher-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full search-bar state.
#[allow(clippy::too_many_arguments)]
fn case(
    search_bar_label: &str,
    scope_target_repr: &str,
    corpus_classes: &[M5DocsCorpusClass],
    source_provider: M5DocsSourceProvider,
    provider_availability: M5DocsProviderAvailability,
    retrieval_mode: M5DocsRetrievalMode,
    version_scope: M5DocsVersionScope,
    keyboard_hint_repr: &str,
    freshness_state: M5DocsFreshnessState,
) -> M5DocsSearchResolutionCase {
    M5DocsSearchResolutionCase::resolved(M5DocsSearchResolutionInput {
        search_bar_label: search_bar_label.to_owned(),
        scope_target_repr: scope_target_repr.to_owned(),
        corpus_classes: corpus_classes.to_vec(),
        source_provider,
        provider_availability,
        retrieval_mode,
        version_scope,
        keyboard_hint_repr: keyboard_hint_repr.to_owned(),
        freshness_state,
    })
}

/// A base row with the shared fields filled in and the full anatomy, corpus,
/// provider, retrieval, availability, limit-reason, next-action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5DocsSearchConsumerSurface,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5DocsSearchResolutionCase>,
) -> M5DocsSearchRow {
    M5DocsSearchRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DocsSearchBarAnatomyPart::ALL.to_vec(),
        corpus_classes: M5DocsCorpusClass::ALL.to_vec(),
        source_providers: M5DocsSourceProvider::ALL.to_vec(),
        provider_availabilities: M5DocsProviderAvailability::ALL.to_vec(),
        retrieval_modes: M5DocsRetrievalMode::ALL.to_vec(),
        version_scopes: M5DocsVersionScope::ALL.to_vec(),
        freshness_states: M5DocsFreshnessState::ALL.to_vec(),
        search_availabilities: M5DocsSearchAvailability::ALL.to_vec(),
        limit_reasons: M5DocsSearchLimitReason::ALL.to_vec(),
        next_actions: M5DocsSearchNextAction::ALL.to_vec(),
        export_fields: M5DocsSearchBarExportField::ALL.to_vec(),
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
            M5DocsDowngradeTrigger::CorpusClassUnstated,
            M5DocsDowngradeTrigger::SourceProviderMasked,
            M5DocsDowngradeTrigger::VersionScopeUnstated,
            M5DocsDowngradeTrigger::FreshnessHidden,
            M5DocsDowngradeTrigger::MirroredOrCachedShownAsLive,
            M5DocsDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_SEARCH_SCHEMA_REF,
            M5_DOCS_SEARCH_SOURCE_RESULT_REF,
            M5_DOCS_SEARCH_SOURCE_PRECEDENCE_REF,
        ]),
        example_resolutions,
        masks_corpus_or_provider: false,
        shows_cached_or_mirrored_as_live: false,
        invents_private_search_grammar: false,
        hides_degraded_state_reason: false,
    }
}

// Sequential pushes preserve the numbered consumer-matrix narrative below.
#[allow(clippy::vec_init_then_push)]
fn search_rows() -> Vec<M5DocsSearchRow> {
    use M5DocsCorpusClass as Corpus;
    use M5DocsFreshnessState as Fresh;
    use M5DocsProviderAvailability as Provider;
    use M5DocsRetrievalMode as Retrieval;
    use M5DocsSourceProvider as Source;
    use M5DocsVersionScope as Scope;

    let mut rows = Vec::new();

    // 1. Docs-browser search — a live-ready first-party/API search, and a
    //    cached-ready guide search served from a disclosed local cache (the
    //    live-versus-cached disclosure proof).
    rows.push(base_row(
        M5DocsSearchConsumerSurface::DocsBrowserSearch,
        M5DocsQualificationClass::Stable,
        "Docs-browser search owner",
        "The docs-browser search bar renders the shared primitive so a first-party/API search with an available provider reads as live-ready, while a guide search served from a local cache reads as cached-ready rather than being shown as live",
        "evidence:m5-docs-search-browser:001",
        vec![
            case(
                "Docs search",
                "aureline-docs@1.4.0",
                &[Corpus::FirstPartyDocs, Corpus::ApiReference],
                Source::FirstPartyHosted,
                Provider::ProviderAvailable,
                Retrieval::LiveRetrieval,
                Scope::ExactVersionMatch,
                "cmd+k",
                Fresh::LiveCurrent,
            ),
            case(
                "Docs search",
                "guides@latest-stable",
                &[Corpus::GuideTutorial],
                Source::BundledLocal,
                Provider::ProviderAvailable,
                Retrieval::CachedRetrieval,
                Scope::LatestStable,
                "cmd+k",
                Fresh::CachedOffline,
            ),
        ],
    ));

    // 2. Onboarding / tutorial lookup — a mirror-only lookup served from a disclosed
    //    mirror, and a degraded-provider lookup narrowed to the offline bundle (the
    //    mirror-versus-degraded proof).
    rows.push(base_row(
        M5DocsSearchConsumerSurface::OnboardingTutorialLookup,
        M5DocsQualificationClass::Stable,
        "Onboarding / tutorial lookup owner",
        "The onboarding / tutorial lookup renders the shared primitive so a mirror-only lookup reads as mirrored-ready, and a lookup whose provider is degraded reads as narrowed-provider-degraded with a use-cached-corpus next action rather than an unexplained empty result",
        "evidence:m5-docs-search-onboarding:001",
        vec![
            case(
                "Tutorial lookup",
                "onboarding-pack@pinned-2.1",
                &[Corpus::GuideTutorial, Corpus::FirstPartyDocs],
                Source::MirroredRegistry,
                Provider::ProviderMirrorOnly,
                Retrieval::MirroredRetrieval,
                Scope::PinnedRange,
                "ctrl+/",
                Fresh::RecentlySynced,
            ),
            case(
                "Tutorial lookup",
                "project-guides@this-project",
                &[Corpus::FirstPartyDocs],
                Source::OfflineImport,
                Provider::ProviderDegraded,
                Retrieval::OfflineBundledRetrieval,
                Scope::ProjectSpecific,
                "ctrl+/",
                Fresh::CachedOffline,
            ),
        ],
    ));

    // 3. AI citation-follow — a policy-limited follow narrowed to the permitted
    //    corpus, and an unavailable-provider follow degraded to a cached copy (the
    //    policy-versus-offline proof).
    rows.push(base_row(
        M5DocsSearchConsumerSurface::AiCitationFollow,
        M5DocsQualificationClass::Stable,
        "AI citation-follow owner",
        "The AI citation-follow flow renders the shared primitive so a follow whose provider is policy-limited reads as narrowed-policy-limited with a request-policy-access next action, while a follow whose provider is unavailable reads as degraded-provider-unavailable with a retry-when-online next action — never an empty citation with no explanation",
        "evidence:m5-docs-search-ai:001",
        vec![
            case(
                "Cited docs",
                "vendor-docs@unversioned",
                &[Corpus::VendorDependency],
                Source::ThirdPartyHosted,
                Provider::ProviderPolicyLimited,
                Retrieval::LiveRetrieval,
                Scope::Unversioned,
                "enter",
                Fresh::LiveCurrent,
            ),
            case(
                "Cited docs",
                "community-docs@nearby-3.0",
                &[Corpus::CommunityContributed],
                Source::MirroredRegistry,
                Provider::ProviderUnavailable,
                Retrieval::CachedRetrieval,
                Scope::NearbyVersion,
                "enter",
                Fresh::StaleExpired,
            ),
        ],
    ));

    // 4. Support / help search — an offline search with no local corpus, and a search
    //    whose provider availability is unknown (the offline-no-corpus versus
    //    unknown-state proof).
    rows.push(base_row(
        M5DocsSearchConsumerSurface::SupportHelpSearch,
        M5DocsQualificationClass::Stable,
        "Support / help search owner",
        "The support / help search renders the shared primitive so a search with no local corpus while offline reads as degraded-offline-no-corpus with an import-or-hand-off next action, while a search whose provider availability has not been evaluated reads as blocked-unknown-state with a run-availability-check next action — both degrade to calm explicit messaging",
        "evidence:m5-docs-search-support:001",
        vec![
            case(
                "Help search",
                "release-notes@unversioned",
                &[Corpus::ReleaseNotesChangelog],
                Source::BundledLocal,
                Provider::ProviderAvailable,
                Retrieval::NoCorpusAvailable,
                Scope::Unversioned,
                "f1",
                Fresh::UnknownFreshness,
            ),
            case(
                "Help search",
                "help-center@unversioned",
                &[Corpus::FirstPartyDocs],
                Source::FirstPartyHosted,
                Provider::ProviderAvailabilityUnknown,
                Retrieval::LiveRetrieval,
                Scope::Unversioned,
                "f1",
                Fresh::UnknownFreshness,
            ),
        ],
    ));

    // 5. CLI docs search — a headless search whose retrieval mode has not been
    //    evaluated (blocked-unknown), and a live-ready codebase-symbol search (the
    //    headless parity proof).
    rows.push(base_row(
        M5DocsSearchConsumerSurface::CliDocsSearch,
        M5DocsQualificationClass::Stable,
        "CLI docs-search owner",
        "The CLI docs search renders the shared primitive so a headless search whose retrieval mode has not been evaluated reads as blocked-unknown-state, while a codebase-symbol search with an available provider reads as live-ready — the same corpus/provider/scope vocabulary a docs-browser reader sees, reachable without a pointer",
        "evidence:m5-docs-search-cli:001",
        vec![
            case(
                "aureline docs search",
                "codebase-symbols@this-project",
                &[Corpus::CodebaseSymbol],
                Source::AiDerived,
                Provider::ProviderAvailable,
                Retrieval::RetrievalModeUnknown,
                Scope::ProjectSpecific,
                "--query",
                Fresh::UnknownFreshness,
            ),
            case(
                "aureline docs search",
                "api-symbols@2.0.0",
                &[Corpus::CodebaseSymbol, Corpus::ApiReference],
                Source::FirstPartyHosted,
                Provider::ProviderAvailable,
                Retrieval::LiveRetrieval,
                Scope::ExactVersionMatch,
                "--query",
                Fresh::LiveCurrent,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5DocsSearchGovernanceReview {
    M5DocsSearchGovernanceReview {
        one_primitive_carries_search_truth: true,
        corpus_and_provider_always_shown: true,
        scope_always_explicit: true,
        cached_or_mirrored_never_shown_as_live: true,
        keyboard_hint_keeps_bar_complete: true,
        degraded_state_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_search_truth: true,
        no_surface_invents_second_search_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsSearchConsumerProjection {
    M5DocsSearchConsumerProjection {
        search_surfaces_consume_shared_primitive: true,
        availability_resolver_reads_single_source: true,
        provider_availability_reads_single_source: true,
        retrieval_mode_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DocsSearchProofFreshness {
    M5DocsSearchProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsSearchReleasePosture {
    M5DocsSearchReleasePosture {
        proof_packet_ref: M5_DOCS_SEARCH_ARTIFACT_REF.to_owned(),
        search_audit_ref: M5_DOCS_SEARCH_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_SEARCH_SCHEMA_REF,
        M5_DOCS_SEARCH_DOC_REF,
        M5_DOCS_SEARCH_COMPONENT_MATRIX_REF,
        M5_DOCS_SEARCH_SOURCE_RESULT_REF,
        M5_DOCS_SEARCH_SOURCE_PRECEDENCE_REF,
    ])
}

/// Builds the canonical M5 docs-search-primitive packet.
pub fn seeded_m5_docs_search_primitive_packet() -> M5DocsSearchPrimitivePacket {
    M5DocsSearchPrimitivePacket::new(M5DocsSearchPrimitivePacketInput {
        packet_id: M5_DOCS_SEARCH_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs-search-bar and scope-switcher primitive: corpus class, source provider, provider availability, cached/live/mirrored retrieval, version scope, and keyboard hint"
                .to_owned(),
        search_rows: search_rows(),
        vocabulary_set: M5DocsSearchVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the onboarding / tutorial lookup is held at Beta because a
/// slice of onboarding surfaces do not yet render the retrieval-mode cue on every
/// profile; every consumer stays visible.
pub fn seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed(
) -> M5DocsSearchPrimitivePacket {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.packet_id =
        "m5-docs-search-bar-and-scope-switcher-primitive:onboarding-beta:0001".to_owned();
    let row = packet
        .search_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsSearchConsumerSurface::OnboardingTutorialLookup)
        .expect("onboarding lookup row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI citation-follow flow is narrowed to Preview pending
/// self-contained-banner parity proof across every AI-context export path; every
/// consumer stays visible.
pub fn seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed(
) -> M5DocsSearchPrimitivePacket {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.packet_id =
        "m5-docs-search-bar-and-scope-switcher-primitive:ai-citation-follow-preview:0001"
            .to_owned();
    let row = packet
        .search_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsSearchConsumerSurface::AiCitationFollow)
        .expect("ai citation-follow row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
