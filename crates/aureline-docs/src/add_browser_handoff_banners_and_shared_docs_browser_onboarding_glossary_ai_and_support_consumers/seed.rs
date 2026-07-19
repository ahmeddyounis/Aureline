//! Canonical seed builders for the M5 docs handoff-banner / shared-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical handoff-consumer packet.
pub const M5_DOCS_HANDOFF_CONSUMER_PACKET_ID: &str =
    "m5-docs-handoff-banner-and-shared-consumers:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked handoff resolution case from a full handoff state.
#[allow(clippy::too_many_arguments)]
fn handoff(
    banner_title_repr: &str,
    handoff_reason: M5DocsHandoffReason,
    destination_repr: &str,
    corpus_class: M5DocsCorpusClass,
    source_provider: M5DocsSourceProvider,
    version_scope: M5DocsVersionScope,
    freshness_state: M5DocsFreshnessState,
    pack_state: M5DocsPackState,
    privacy_exposure: M5DocsHandoffPrivacyExposure,
    return_anchor_repr: &str,
    return_context_source_repr: &str,
    return_context_version_repr: &str,
) -> M5DocsHandoffBannerResolutionCase {
    M5DocsHandoffBannerResolutionCase::resolved(M5DocsHandoffBannerResolutionInput {
        banner_title_repr: banner_title_repr.to_owned(),
        handoff_reason,
        destination_repr: destination_repr.to_owned(),
        corpus_class,
        source_provider,
        version_scope,
        freshness_state,
        pack_state,
        privacy_exposure,
        return_anchor_repr: return_anchor_repr.to_owned(),
        return_context_source_repr: return_context_source_repr.to_owned(),
        return_context_version_repr: return_context_version_repr.to_owned(),
    })
}

/// A base consumer row with the shared fields filled in and the full reason, necessity,
/// exposure, consequence, return-posture, action, export-field, corpus, provider, version,
/// freshness, pack, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5DocsHandoffConsumerSurface,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    reused_components: Vec<M5DocsSharedComponent>,
    handoff_examples: Vec<M5DocsHandoffBannerResolutionCase>,
) -> M5DocsHandoffConsumerRow {
    M5DocsHandoffConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        reused_components,
        banner_anatomy_parts: M5DocsHandoffBannerAnatomyPart::ALL.to_vec(),
        handoff_reasons: M5DocsHandoffReason::ALL.to_vec(),
        necessities: M5DocsHandoffNecessity::ALL.to_vec(),
        privacy_exposures: M5DocsHandoffPrivacyExposure::ALL.to_vec(),
        privacy_consequences: M5DocsHandoffPrivacyConsequence::ALL.to_vec(),
        return_path_postures: M5DocsHandoffReturnPathPosture::ALL.to_vec(),
        handoff_actions: M5DocsHandoffAction::ALL.to_vec(),
        corpus_classes: M5DocsCorpusClass::ALL.to_vec(),
        source_providers: M5DocsSourceProvider::ALL.to_vec(),
        version_scopes: M5DocsVersionScope::ALL.to_vec(),
        freshness_states: M5DocsFreshnessState::ALL.to_vec(),
        pack_states: M5DocsPackState::ALL.to_vec(),
        export_fields: M5DocsHandoffExportField::ALL.to_vec(),
        accessibility_routes: M5DocsAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5DocsConsumerSurface::DocsBrowserUi,
            M5DocsConsumerSurface::HelpAbout,
            M5DocsConsumerSurface::OnboardingTour,
            M5DocsConsumerSurface::AiContextPanel,
            M5DocsConsumerSurface::SupportExport,
            M5DocsConsumerSurface::CliInspect,
            M5DocsConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5DocsDowngradeTrigger::SourceProviderMasked,
            M5DocsDowngradeTrigger::VersionScopeUnstated,
            M5DocsDowngradeTrigger::FreshnessHidden,
            M5DocsDowngradeTrigger::HandoffReasonUnstated,
            M5DocsDowngradeTrigger::MirroredOrCachedShownAsLive,
            M5DocsDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_HANDOFF_CONSUMER_SCHEMA_REF,
            M5_DOCS_HANDOFF_CONSUMER_HANDOFF_PACKET_REF,
            M5_DOCS_HANDOFF_CONSUMER_SOURCE_RESULT_REF,
        ]),
        handoff_examples,
        strips_source_version_context: false,
        understates_privacy_consequence: false,
        flattens_to_raw_url_jump: false,
        invents_private_handoff_grammar: false,
    }
}

// Sequential pushes preserve the numbered consumer-matrix narrative below.
#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5DocsHandoffConsumerRow> {
    use M5DocsCorpusClass as Corpus;
    use M5DocsFreshnessState as Fresh;
    use M5DocsHandoffPrivacyExposure as Exposure;
    use M5DocsHandoffReason as Reason;
    use M5DocsPackState as Pack;
    use M5DocsSharedComponent as Component;
    use M5DocsSourceProvider as Source;
    use M5DocsVersionScope as Scope;

    let mut rows = Vec::new();

    // 1. Docs browser — a no-local-corpus handoff to upstream std docs (cannot serve
    //    in-product, context preserved on return) and a user-requested handoff to a bundled
    //    mirror that leaves nothing behind (stays fully in-product).
    rows.push(base_row(
        M5DocsHandoffConsumerSurface::DocsBrowser,
        M5DocsQualificationClass::Stable,
        "Docs browser owner",
        "The docs browser renders the shared handoff banner so a no-local-corpus lookup reads as cannot-serve-in-product with the query context shared and the source/version preserved on return, while a user-requested open of a bundled mirror stays fully in-product — reusing the same search bar, result row, source/version badge, and pack row it renders everywhere else",
        "evidence:m5-handoff-docs-browser:001",
        vec![
            Component::SearchBar,
            Component::ResultRow,
            Component::SourceVersionBadge,
            Component::PackRow,
            Component::HandoffBanner,
        ],
        vec![
            handoff(
                "Open Rust std docs in browser",
                Reason::NoLocalCorpus,
                "external:rust-std-docs",
                Corpus::ApiReference,
                Source::ThirdPartyHosted,
                Scope::ExactVersionMatch,
                Fresh::RecentlySynced,
                Pack::UnpinnedTracking,
                Exposure::DocumentContextLeaves,
                "return:docs-browser/std-vec",
                "ctx-src:rust-std",
                "ctx-ver:1.75",
            ),
            handoff(
                "Open bundled guide mirror",
                Reason::UserRequestedBrowser,
                "external:bundled-guide-mirror",
                Corpus::GuideTutorial,
                Source::BundledLocal,
                Scope::LatestStable,
                Fresh::LiveCurrent,
                Pack::PinnedPack,
                Exposure::NoDataLeaves,
                "return:docs-browser/guide-intro",
                "",
                "",
            ),
        ],
    ));

    // 2. Onboarding tour — a dynamic-rendering handoff to an interactive playground (cannot
    //    serve in-product, only an anonymous lookup leaves, context preserved).
    rows.push(base_row(
        M5DocsHandoffConsumerSurface::OnboardingTour,
        M5DocsQualificationClass::Stable,
        "Onboarding tour owner",
        "The onboarding tour renders the shared handoff banner so a dynamic-rendering-only playground reads as cannot-serve-in-product with only an anonymous lookup leaving Aureline, and the tour step is preserved as the return anchor so the learner comes back where they left off — reusing the same search bar, result row, stale-example row, and handoff banner",
        "evidence:m5-handoff-onboarding:001",
        vec![
            Component::SearchBar,
            Component::ResultRow,
            Component::StaleExampleRow,
            Component::HandoffBanner,
        ],
        vec![handoff(
            "Open interactive playground",
            Reason::DynamicRenderingRequired,
            "external:interactive-playground",
            Corpus::GuideTutorial,
            Source::FirstPartyHosted,
            Scope::LatestStable,
            Fresh::LiveCurrent,
            Pack::UnpinnedTracking,
            Exposure::AnonymousQueryLeaves,
            "return:onboarding/tour-step-3",
            "ctx-src:onboarding",
            "ctx-ver:latest",
        )],
    ));

    // 3. Glossary card — an external-canonical handoff to a canonical glossary (should defer
    //    to canonical, query context shared, context preserved).
    rows.push(base_row(
        M5DocsHandoffConsumerSurface::GlossaryCard,
        M5DocsQualificationClass::Stable,
        "Glossary card owner",
        "The glossary card renders the shared handoff banner so an external-canonical definition reads as should-defer-to-canonical with the query context shared and the glossary term preserved on return — reusing the same symbol-reference card, source/version badge, and handoff banner it shows in hover and peek",
        "evidence:m5-handoff-glossary:001",
        vec![
            Component::ReferenceCard,
            Component::SourceVersionBadge,
            Component::HandoffBanner,
        ],
        vec![handoff(
            "Open canonical glossary entry",
            Reason::ExternalCanonicalSource,
            "external:canonical-glossary",
            Corpus::FirstPartyDocs,
            Source::FirstPartyHosted,
            Scope::Unversioned,
            Fresh::LiveCurrent,
            Pack::UnpinnedTracking,
            Exposure::DocumentContextLeaves,
            "return:glossary/term-idempotency",
            "ctx-src:glossary",
            "ctx-ver:live",
        )],
    ));

    // 4. AI-evidence follow — an auth-gated handoff (escalated to identified request even
    //    though the caller declared no data leaving) and an external-account handoff
    //    (external account + identity shared).
    rows.push(base_row(
        M5DocsHandoffConsumerSurface::AiEvidenceFollow,
        M5DocsQualificationClass::Stable,
        "AI-evidence follow owner",
        "The AI-evidence follow link renders the shared handoff banner so an auth-gated vendor portal is escalated to an identified request — never understated as no-data-leaves just because the citation was local — and an external API console reads as sharing an external account and identity; both preserve the citation's source/version on return, reusing the same result row, symbol-reference card, pack row, and handoff banner",
        "evidence:m5-handoff-ai-evidence:001",
        vec![
            Component::ResultRow,
            Component::ReferenceCard,
            Component::PackRow,
            Component::HandoffBanner,
        ],
        vec![
            handoff(
                "Open auth-gated vendor portal",
                Reason::AuthGatedSource,
                "external:vendor-portal",
                Corpus::VendorDependency,
                Source::ThirdPartyHosted,
                Scope::PinnedRange,
                Fresh::CachedOffline,
                Pack::MirroredPack,
                Exposure::NoDataLeaves,
                "return:ai/evidence-cite-42",
                "ctx-src:vendor",
                "ctx-ver:2.0",
            ),
            handoff(
                "Open external API console",
                Reason::ExternalCanonicalSource,
                "external:api-console",
                Corpus::ApiReference,
                Source::ThirdPartyHosted,
                Scope::LatestStable,
                Fresh::RecentlySynced,
                Pack::UnpinnedTracking,
                Exposure::ExternalAccountRequired,
                "return:ai/api-explorer",
                "ctx-src:api",
                "ctx-ver:3",
            ),
        ],
    ));

    // 5. Support / help — a no-local-corpus handoff to a support KB that carries an
    //    identified request (cannot serve in-product, identified request shared, context
    //    preserved for the ticket).
    rows.push(base_row(
        M5DocsHandoffConsumerSurface::SupportHelp,
        M5DocsQualificationClass::Stable,
        "Support / help owner",
        "The support / help view renders the shared handoff banner so a no-local-corpus lookup into the support knowledge base reads as cannot-serve-in-product with an identified request shared, and the ticket context is preserved on return so the handoff survives the support/export path with the same words — reusing the same pack row, stale-example row, source/version badge, and handoff banner",
        "evidence:m5-handoff-support:001",
        vec![
            Component::PackRow,
            Component::StaleExampleRow,
            Component::SourceVersionBadge,
            Component::HandoffBanner,
        ],
        vec![handoff(
            "Open support knowledge base",
            Reason::NoLocalCorpus,
            "external:support-kb",
            Corpus::FirstPartyDocs,
            Source::FirstPartyHosted,
            Scope::LatestStable,
            Fresh::RecentlySynced,
            Pack::UnpinnedTracking,
            Exposure::IdentifiedRequestLeaves,
            "return:support/ticket-ctx-88",
            "ctx-src:support",
            "ctx-ver:kb",
        )],
    ));

    rows
}

fn governance_review() -> M5DocsHandoffGovernanceReview {
    M5DocsHandoffGovernanceReview {
        shared_banner_carries_truth: true,
        destination_reason_always_stated: true,
        privacy_consequence_always_stated: true,
        return_path_always_present: true,
        in_product_blocker_always_explained: true,
        source_version_context_preserved_through_handoff: true,
        components_consistent_across_consumers: true,
        no_consumer_invents_second_handoff_grammar: true,
        every_row_declares_accessibility_route: true,
        privacy_consequence_never_understated: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsHandoffConsumerProjection {
    M5DocsHandoffConsumerProjection {
        consumers_consume_shared_banner: true,
        privacy_consequence_reads_single_source: true,
        return_path_reads_single_source: true,
        component_reuse_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DocsHandoffProofFreshness {
    M5DocsHandoffProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsHandoffReleasePosture {
    M5DocsHandoffReleasePosture {
        proof_packet_ref: M5_DOCS_HANDOFF_CONSUMER_ARTIFACT_REF.to_owned(),
        handoff_audit_ref: M5_DOCS_HANDOFF_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_HANDOFF_CONSUMER_SCHEMA_REF,
        M5_DOCS_HANDOFF_CONSUMER_DOC_REF,
        M5_DOCS_HANDOFF_CONSUMER_COMPONENT_MATRIX_REF,
        M5_DOCS_HANDOFF_CONSUMER_HANDOFF_PACKET_REF,
        M5_DOCS_HANDOFF_CONSUMER_SOURCE_RESULT_REF,
        M5_DOCS_SEARCH_SCHEMA_REF,
        M5_DOCS_RESULT_ROW_SCHEMA_REF,
        M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
        M5_DOCS_PACK_FINDING_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 docs handoff-banner / shared-consumer packet.
pub fn seeded_m5_docs_handoff_consumer_packet() -> M5DocsHandoffConsumerPacket {
    M5DocsHandoffConsumerPacket::new(M5DocsHandoffConsumerPacketInput {
        packet_id: M5_DOCS_HANDOFF_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs handoff banner and shared docs-browser consumers: destination reason, in-product necessity, privacy consequence, return path, and source/version/pack context preserved across docs-browser, onboarding, glossary, AI-evidence, and support/help"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5DocsHandoffVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the onboarding tour is held at Beta because a slice of onboarding
/// surfaces do not yet render the source/version context badge on every profile; every
/// consumer stays visible.
pub fn seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed() -> M5DocsHandoffConsumerPacket
{
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.packet_id =
        "m5-docs-handoff-banner-and-shared-consumers:onboarding-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsHandoffConsumerSurface::OnboardingTour)
        .expect("onboarding tour row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-evidence follow link is narrowed to Preview pending
/// privacy-consequence parity proof across every AI-evidence export path; every consumer
/// stays visible.
pub fn seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed() -> M5DocsHandoffConsumerPacket
{
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.packet_id =
        "m5-docs-handoff-banner-and-shared-consumers:ai-evidence-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsHandoffConsumerSurface::AiEvidenceFollow)
        .expect("ai-evidence follow row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
