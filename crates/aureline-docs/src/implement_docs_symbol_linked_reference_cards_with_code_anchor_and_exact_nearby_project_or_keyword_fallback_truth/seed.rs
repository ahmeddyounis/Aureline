//! Canonical seed builders for the M5 symbol-linked reference-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical reference-card-primitive packet.
pub const M5_DOCS_REFERENCE_CARD_PRIMITIVE_PACKET_ID: &str =
    "m5-symbol-linked-reference-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full reference-card state.
#[allow(clippy::too_many_arguments)]
fn case(
    card_title_repr: &str,
    initiating_file_repr: &str,
    initiating_symbol_repr: &str,
    symbol_anchor: M5DocsSymbolAnchor,
    corpus_class: M5DocsCorpusClass,
    source_provider: M5DocsSourceProvider,
    match_state: M5DocsMatchState,
    override_reason: M5DocsOverrideReason,
    version_scope: M5DocsVersionScope,
    freshness_state: M5DocsFreshnessState,
    cited_source_revision_repr: &str,
    open_action_target_repr: &str,
) -> M5DocsReferenceCardResolutionCase {
    M5DocsReferenceCardResolutionCase::resolved(M5DocsReferenceCardResolutionInput {
        card_title_repr: card_title_repr.to_owned(),
        initiating_file_repr: initiating_file_repr.to_owned(),
        initiating_symbol_repr: initiating_symbol_repr.to_owned(),
        symbol_anchor,
        corpus_class,
        source_provider,
        match_state,
        override_reason,
        version_scope,
        freshness_state,
        cited_source_revision_repr: cited_source_revision_repr.to_owned(),
        open_action_target_repr: open_action_target_repr.to_owned(),
    })
}

/// A base card row with the shared fields filled in and the full anatomy, symbol
/// anchor, linkage-strength, corpus, provider, match-state, override-reason, version,
/// freshness, posture, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5DocsReferenceCardConsumerSurface,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5DocsReferenceCardResolutionCase>,
) -> M5DocsReferenceCardRow {
    M5DocsReferenceCardRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DocsReferenceCardAnatomyPart::ALL.to_vec(),
        symbol_anchors: M5DocsSymbolAnchor::ALL.to_vec(),
        linkage_strengths: M5DocsSymbolLinkageStrength::ALL.to_vec(),
        corpus_classes: M5DocsCorpusClass::ALL.to_vec(),
        source_providers: M5DocsSourceProvider::ALL.to_vec(),
        match_states: M5DocsMatchState::ALL.to_vec(),
        override_reasons: M5DocsOverrideReason::ALL.to_vec(),
        version_scopes: M5DocsVersionScope::ALL.to_vec(),
        freshness_states: M5DocsFreshnessState::ALL.to_vec(),
        freshness_postures: M5DocsCardFreshnessPosture::ALL.to_vec(),
        export_fields: M5DocsReferenceCardExportField::ALL.to_vec(),
        accessibility_routes: M5DocsAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5DocsConsumerSurface::DocsBrowserUi,
            M5DocsConsumerSurface::HelpAbout,
            M5DocsConsumerSurface::HoverPeek,
            M5DocsConsumerSurface::OnboardingTour,
            M5DocsConsumerSurface::AiContextPanel,
            M5DocsConsumerSurface::SupportExport,
            M5DocsConsumerSurface::CliInspect,
            M5DocsConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5DocsDowngradeTrigger::SourceProviderMasked,
            M5DocsDowngradeTrigger::VersionScopeUnstated,
            M5DocsDowngradeTrigger::SymbolAnchorUnresolvedHidden,
            M5DocsDowngradeTrigger::ProjectOverrideReasonHidden,
            M5DocsDowngradeTrigger::FreshnessHidden,
            M5DocsDowngradeTrigger::MirroredOrCachedShownAsLive,
            M5DocsDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
            M5_DOCS_REFERENCE_CARD_SOURCE_RESULT_REF,
            M5_DOCS_REFERENCE_CARD_SOURCE_PRECEDENCE_REF,
        ]),
        example_resolutions,
        masks_source_or_version: false,
        shows_cached_or_stale_as_live: false,
        invents_private_card_grammar: false,
        hides_symbol_linkage: false,
    }
}

fn reference_card_rows() -> Vec<M5DocsReferenceCardRow> {
    use M5DocsCorpusClass as Corpus;
    use M5DocsFreshnessState as Fresh;
    use M5DocsMatchState as Match;
    use M5DocsOverrideReason as Override;
    use M5DocsSourceProvider as Source;
    use M5DocsSymbolAnchor as Anchor;
    use M5DocsVersionScope as Scope;

    let mut rows = Vec::new();

    // 1. Editor hover / peek — an exact live function match, a nearby-version vendor
    //    type, and an unresolved anchor that fell back to a keyword match (never shown
    //    as an exact symbol match).
    rows.push(base_row(
        M5DocsReferenceCardConsumerSurface::EditorHoverPeek,
        M5DocsQualificationClass::Stable,
        "Editor hover/peek reference-card owner",
        "The editor hover/peek reference card keeps the initiating file/symbol anchor visible so an exact live function match reads as exact-symbol linkage, a nearby vendor type reads as nearby-version linkage, and an unresolved anchor served from a mirror reads as a keyword fallback — never an exact symbol match",
        "evidence:m5-reference-card-hover:001",
        vec![
            case(
                "Client::send",
                "src/client.rs",
                "Client::send",
                Anchor::FunctionSymbol,
                Corpus::ApiReference,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Scope::ExactVersionMatch,
                Fresh::LiveCurrent,
                "rev:api-1.4.0",
                "open:doc/api/client-send",
            ),
            case(
                "Widget",
                "src/ui/widget.rs",
                "Widget",
                Anchor::TypeSymbol,
                Corpus::VendorDependency,
                Source::ThirdPartyHosted,
                Match::NearbyMatch,
                Override::NoOverride,
                Scope::NearbyVersion,
                Fresh::RecentlySynced,
                "rev:widgetkit-2.0",
                "open:doc/vendor/widget",
            ),
            case(
                "retry helpers",
                "src/net/retry.rs",
                "retry_backoff",
                Anchor::UnresolvedAnchor,
                Corpus::GuideTutorial,
                Source::MirroredRegistry,
                Match::NearbyMatch,
                Override::NoOverride,
                Scope::LatestStable,
                Fresh::CachedOffline,
                "",
                "open:doc/guide/retry",
            ),
        ],
    ));

    // 2. Docs-browser card — a project-specific codebase symbol that took precedence, a
    //    mirror-served heuristic type (mirrored-explicit-not-live), and an exact live
    //    module match.
    rows.push(base_row(
        M5DocsReferenceCardConsumerSurface::DocsBrowserCard,
        M5DocsQualificationClass::Stable,
        "Docs-browser reference-card owner",
        "The docs-browser reference card renders the shared primitive so a project-specific codebase symbol reads as project-specific linkage, a mirror-served type reads as heuristic linkage that is mirrored-explicit-not-live, and an exact live module match reads as exact-symbol linkage — the same anchor/linkage vocabulary the editor shows",
        "evidence:m5-reference-card-browser:001",
        vec![
            case(
                "resolve_run_context",
                "crates/aureline-shell/src/run.rs",
                "resolve_run_context",
                Anchor::FunctionSymbol,
                Corpus::CodebaseSymbol,
                Source::FirstPartyHosted,
                Match::ProjectSpecificMatch,
                Override::ProjectPinnedOverride,
                Scope::ProjectSpecific,
                Fresh::RecentlySynced,
                "rev:workspace-head",
                "open:doc/symbol/resolve-run-context",
            ),
            case(
                "Config",
                "src/config.rs",
                "Config",
                Anchor::TypeSymbol,
                Corpus::ApiReference,
                Source::MirroredRegistry,
                Match::MirroredMatch,
                Override::NoOverride,
                Scope::PinnedRange,
                Fresh::CachedOffline,
                "rev:config-1.2",
                "open:doc/api/config",
            ),
            case(
                "logging module",
                "src/logging/mod.rs",
                "logging",
                Anchor::ModuleSymbol,
                Corpus::FirstPartyDocs,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Scope::LatestStable,
                Fresh::LiveCurrent,
                "rev:docs-main",
                "open:doc/module/logging",
            ),
        ],
    ));

    // 3. AI-explanation card — an AI-derived field/method nearby match of unknown
    //    freshness, an unresolved+stale anchor with no linkage at all, and an exact
    //    local macro match.
    rows.push(base_row(
        M5DocsReferenceCardConsumerSurface::AiExplanationCard,
        M5DocsQualificationClass::Stable,
        "AI-explanation reference-card owner",
        "The AI-explanation reference card renders the shared primitive so an AI-derived field/method reads as nearby-version linkage with an unknown-freshness posture, an unresolved stale anchor reads as unresolved-no-linkage flagged stale, and an exact local macro reads as exact-symbol linkage — never an AI paraphrase that hides how weak the linkage is",
        "evidence:m5-reference-card-ai:001",
        vec![
            case(
                "Backoff::max_delay",
                "src/net/retry.rs",
                "Backoff::max_delay",
                Anchor::FieldOrMethod,
                Corpus::ApiReference,
                Source::AiDerived,
                Match::NearbyMatch,
                Override::ExplicitUserPreference,
                Scope::Unversioned,
                Fresh::UnknownFreshness,
                "",
                "open:doc/ai/backoff-max-delay",
            ),
            case(
                "legacy config keys",
                "src/config/legacy.rs",
                "LEGACY_KEYS",
                Anchor::UnresolvedAnchor,
                Corpus::CommunityContributed,
                Source::OfflineImport,
                Match::StaleMatch,
                Override::VendorSourceUnavailable,
                Scope::PinnedRange,
                Fresh::StaleExpired,
                "rev:legacy-0.1",
                "open:doc/ai/legacy-keys",
            ),
            case(
                "declare_component macro",
                "src/macros.rs",
                "declare_component",
                Anchor::MacroSymbol,
                Corpus::ApiReference,
                Source::BundledLocal,
                Match::ExactMatch,
                Override::LocalFreshnessOverride,
                Scope::ExactVersionMatch,
                Fresh::LiveCurrent,
                "rev:macros-1.0",
                "open:doc/macro/declare-component",
            ),
        ],
    ));

    // 4. Onboarding reference card — an exact live first-party function, and a
    //    community plugin type served from cache (heuristic, cached-explicit-not-live).
    rows.push(base_row(
        M5DocsReferenceCardConsumerSurface::OnboardingReferenceCard,
        M5DocsQualificationClass::Stable,
        "Onboarding reference-card owner",
        "The onboarding reference card renders the shared primitive so an exact live first-party function reads as exact-symbol linkage, while a cached community plugin type reads as heuristic linkage that is cached-explicit-not-live — the same anchor/linkage/freshness vocabulary a docs-browser reader sees",
        "evidence:m5-reference-card-onboarding:001",
        vec![
            case(
                "quickstart main",
                "examples/quickstart.rs",
                "main",
                Anchor::FunctionSymbol,
                Corpus::FirstPartyDocs,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::NoOverride,
                Scope::Unversioned,
                Fresh::LiveCurrent,
                "rev:guide-main",
                "open:doc/onboarding/quickstart",
            ),
            case(
                "PluginRegistry",
                "src/plugins/mod.rs",
                "PluginRegistry",
                Anchor::TypeSymbol,
                Corpus::CommunityContributed,
                Source::OfflineImport,
                Match::CachedMatch,
                Override::ExplicitUserPreference,
                Scope::PinnedRange,
                Fresh::CachedOffline,
                "rev:plugin-0.9",
                "open:doc/onboarding/plugin-registry",
            ),
        ],
    ));

    // 5. Support evidence card — an exact live release-notes field decided by policy,
    //    and a stale vendor type whose upstream was unavailable (heuristic, stale).
    rows.push(base_row(
        M5DocsReferenceCardConsumerSurface::SupportEvidenceCard,
        M5DocsQualificationClass::Stable,
        "Support evidence reference-card owner",
        "The support evidence reference card renders the shared primitive so an exact release-notes field decided by policy reads as exact-symbol linkage with a recently-synced posture, while a stale vendor type whose upstream was unavailable reads as heuristic linkage flagged stale — both keep the initiating anchor and source descriptors so identity survives the support/AI evidence path",
        "evidence:m5-reference-card-support:001",
        vec![
            case(
                "VERSION",
                "src/version.rs",
                "VERSION",
                Anchor::FieldOrMethod,
                Corpus::ReleaseNotesChangelog,
                Source::FirstPartyHosted,
                Match::ExactMatch,
                Override::PolicyScopedOverride,
                Scope::LatestStable,
                Fresh::RecentlySynced,
                "rev:release-1.4",
                "open:doc/support/release-1-4",
            ),
            case(
                "VendorClient",
                "src/vendor/auth.rs",
                "VendorClient",
                Anchor::TypeSymbol,
                Corpus::VendorDependency,
                Source::ThirdPartyHosted,
                Match::StaleMatch,
                Override::VendorSourceUnavailable,
                Scope::PinnedRange,
                Fresh::StaleExpired,
                "rev:vendor-3.1",
                "open:doc/support/vendor-auth",
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5DocsReferenceCardGovernanceReview {
    M5DocsReferenceCardGovernanceReview {
        one_primitive_carries_card_truth: true,
        initiating_anchor_always_preserved: true,
        linkage_strength_always_explicit: true,
        exact_nearby_project_keyword_never_blended: true,
        cited_source_revision_visible: true,
        cached_or_stale_never_shown_as_live: true,
        reference_card_identity_survives_export: true,
        badge_state_vocabulary_stable_across_surfaces: true,
        no_surface_invents_second_card_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsReferenceCardConsumerProjection {
    M5DocsReferenceCardConsumerProjection {
        card_surfaces_consume_shared_primitive: true,
        linkage_strength_reads_single_source: true,
        anchor_reads_single_source: true,
        freshness_posture_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DocsReferenceCardProofFreshness {
    M5DocsReferenceCardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsReferenceCardReleasePosture {
    M5DocsReferenceCardReleasePosture {
        proof_packet_ref: M5_DOCS_REFERENCE_CARD_ARTIFACT_REF.to_owned(),
        reference_card_audit_ref: M5_DOCS_REFERENCE_CARD_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
        M5_DOCS_REFERENCE_CARD_DOC_REF,
        M5_DOCS_REFERENCE_CARD_COMPONENT_MATRIX_REF,
        M5_DOCS_REFERENCE_CARD_SOURCE_RESULT_REF,
        M5_DOCS_REFERENCE_CARD_SOURCE_PRECEDENCE_REF,
    ])
}

/// Builds the canonical M5 symbol-linked-reference-card-primitive packet.
pub fn seeded_m5_reference_card_primitive_packet() -> M5DocsReferenceCardPrimitivePacket {
    M5DocsReferenceCardPrimitivePacket::new(M5DocsReferenceCardPrimitivePacketInput {
        packet_id: M5_DOCS_REFERENCE_CARD_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 symbol-linked reference-card primitive: initiating code anchor, symbol anchor, linkage strength, source provider, version scope, cited revision, and freshness posture"
                .to_owned(),
        reference_card_rows: reference_card_rows(),
        vocabulary_set: M5DocsReferenceCardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the onboarding reference card is held at Beta because a slice of
/// onboarding surfaces do not yet render the linkage-strength cue on every profile;
/// every consumer stays visible.
pub fn seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed(
) -> M5DocsReferenceCardPrimitivePacket {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.packet_id = "m5-symbol-linked-reference-card-primitive:onboarding-beta:0001".to_owned();
    let row = packet
        .reference_card_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5DocsReferenceCardConsumerSurface::OnboardingReferenceCard
        })
        .expect("onboarding reference card row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-explanation card is narrowed to Preview pending
/// linkage-disclosure parity proof across every AI-context export path; every consumer
/// stays visible.
pub fn seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed(
) -> M5DocsReferenceCardPrimitivePacket {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.packet_id =
        "m5-symbol-linked-reference-card-primitive:ai-explanation-preview:0001".to_owned();
    let row = packet
        .reference_card_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsReferenceCardConsumerSurface::AiExplanationCard)
        .expect("ai-explanation card row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
