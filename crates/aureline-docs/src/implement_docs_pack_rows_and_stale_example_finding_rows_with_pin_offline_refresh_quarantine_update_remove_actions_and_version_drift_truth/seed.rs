//! Canonical seed builders for the M5 docs-pack-row / stale-example-finding-row
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical pack/finding-primitive packet.
pub const M5_DOCS_PACK_FINDING_PRIMITIVE_PACKET_ID: &str =
    "m5-docs-pack-row-and-stale-example-finding-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked pack resolution case from a full pack state.
#[allow(clippy::too_many_arguments)]
fn pack(
    pack_name_repr: &str,
    corpus_class: M5DocsCorpusClass,
    source_provider: M5DocsSourceProvider,
    version_scope: M5DocsVersionScope,
    pack_state: M5DocsPackState,
    freshness_state: M5DocsFreshnessState,
    verification_state: M5DocsPackVerificationState,
    item_count: u32,
    size_bytes: u64,
    signer_repr: &str,
    refresh_time_repr: &str,
    manage_action_target_repr: &str,
) -> M5DocsPackRowResolutionCase {
    M5DocsPackRowResolutionCase::resolved(M5DocsPackRowResolutionInput {
        pack_name_repr: pack_name_repr.to_owned(),
        corpus_class,
        source_provider,
        version_scope,
        pack_state,
        freshness_state,
        verification_state,
        item_count,
        size_bytes,
        signer_repr: signer_repr.to_owned(),
        refresh_time_repr: refresh_time_repr.to_owned(),
        manage_action_target_repr: manage_action_target_repr.to_owned(),
    })
}

/// Builds a worked stale-example finding case from a full finding state.
#[allow(clippy::too_many_arguments)]
fn finding(
    finding_title_repr: &str,
    affected_anchor_repr: &str,
    anchor_kind: M5DocsExampleAnchorKind,
    corpus_class: M5DocsCorpusClass,
    source_provider: M5DocsSourceProvider,
    version_scope: M5DocsVersionScope,
    stale_example_status: M5DocsStaleExampleStatus,
    freshness_state: M5DocsFreshnessState,
    documented_version_repr: &str,
    current_version_repr: &str,
    open_current_source_target_repr: &str,
) -> M5DocsStaleExampleRowResolutionCase {
    M5DocsStaleExampleRowResolutionCase::resolved(M5DocsStaleExampleRowResolutionInput {
        finding_title_repr: finding_title_repr.to_owned(),
        affected_anchor_repr: affected_anchor_repr.to_owned(),
        anchor_kind,
        corpus_class,
        source_provider,
        version_scope,
        stale_example_status,
        freshness_state,
        documented_version_repr: documented_version_repr.to_owned(),
        current_version_repr: current_version_repr.to_owned(),
        open_current_source_target_repr: open_current_source_target_repr.to_owned(),
    })
}

/// A base pack/finding row with the shared fields filled in and the full pack/example
/// anatomy, pack-state, trust-posture, verification, action, stale-example-status,
/// drift-posture, anchor-kind, corpus, provider, version, freshness, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5DocsPackConsumerSurface,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    pack_examples: Vec<M5DocsPackRowResolutionCase>,
    stale_example_findings: Vec<M5DocsStaleExampleRowResolutionCase>,
) -> M5DocsPackFindingRow {
    M5DocsPackFindingRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        pack_anatomy_parts: M5DocsPackRowAnatomyPart::ALL.to_vec(),
        example_anatomy_parts: M5DocsStaleExampleRowAnatomyPart::ALL.to_vec(),
        pack_states: M5DocsPackState::ALL.to_vec(),
        trust_postures: M5DocsPackTrustPosture::ALL.to_vec(),
        verification_states: M5DocsPackVerificationState::ALL.to_vec(),
        pack_actions: M5DocsPackAction::ALL.to_vec(),
        stale_example_statuses: M5DocsStaleExampleStatus::ALL.to_vec(),
        drift_postures: M5DocsExampleDriftPosture::ALL.to_vec(),
        anchor_kinds: M5DocsExampleAnchorKind::ALL.to_vec(),
        example_actions: M5DocsExampleAction::ALL.to_vec(),
        corpus_classes: M5DocsCorpusClass::ALL.to_vec(),
        source_providers: M5DocsSourceProvider::ALL.to_vec(),
        version_scopes: M5DocsVersionScope::ALL.to_vec(),
        freshness_states: M5DocsFreshnessState::ALL.to_vec(),
        export_fields: M5DocsPackFindingExportField::ALL.to_vec(),
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
            M5DocsDowngradeTrigger::FreshnessHidden,
            M5DocsDowngradeTrigger::PackStateMisrepresented,
            M5DocsDowngradeTrigger::StaleExampleShownAsCurrent,
            M5DocsDowngradeTrigger::MirroredOrCachedShownAsLive,
            M5DocsDowngradeTrigger::QuarantinedPackShownAsTrusted,
            M5DocsDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_PACK_FINDING_SCHEMA_REF,
            M5_DOCS_PACK_FINDING_SOURCE_RESULT_REF,
            M5_DOCS_PACK_FINDING_SOURCE_PRECEDENCE_REF,
        ]),
        pack_examples,
        stale_example_findings,
        masks_pack_state_or_source: false,
        shows_quarantined_or_stale_as_trusted: false,
        invents_private_pack_grammar: false,
        hides_version_drift: false,
    }
}

fn pack_finding_rows() -> Vec<M5DocsPackFindingRow> {
    use M5DocsCorpusClass as Corpus;
    use M5DocsExampleAnchorKind as Anchor;
    use M5DocsFreshnessState as Fresh;
    use M5DocsPackState as Pack;
    use M5DocsPackVerificationState as Verify;
    use M5DocsSourceProvider as Source;
    use M5DocsStaleExampleStatus as Status;
    use M5DocsVersionScope as Scope;

    let mut rows = Vec::new();

    // 1. Docs-pack manager — a pinned-current first-party pack and a mirror-served vendor
    //    pack (mirror, never live); a verified-current guide example and an AI-derived
    //    signature-drift finding with concrete version drift.
    rows.push(base_row(
        M5DocsPackConsumerSurface::DocsPackManager,
        M5DocsQualificationClass::Stable,
        "Docs-pack manager owner",
        "The docs-pack manager renders the shared pack row so a pinned first-party pack reads as pinned-current while a mirror-served vendor pack reads as mirror-served-not-live, and the shared finding row so an API-signature drift becomes an actionable, version-anchored row rather than a vague hint",
        "evidence:m5-pack-finding-manager:001",
        vec![
            pack(
                "aureline-core-docs",
                Corpus::FirstPartyDocs,
                Source::FirstPartyHosted,
                Scope::PinnedRange,
                Pack::PinnedPack,
                Fresh::LiveCurrent,
                Verify::SignatureVerified,
                412,
                8_540_160,
                "signer:aureline-release",
                "refresh:2026-07-05T22:00Z",
                "manage:pack/aureline-core-docs",
            ),
            pack(
                "widgetkit-vendor-docs",
                Corpus::VendorDependency,
                Source::MirroredRegistry,
                Scope::NearbyVersion,
                Pack::MirroredPack,
                Fresh::RecentlySynced,
                Verify::SignatureVerified,
                97,
                2_100_400,
                "signer:widgetkit",
                "refresh:2026-07-04T09:00Z",
                "manage:pack/widgetkit-vendor-docs",
            ),
        ],
        vec![
            finding(
                "Quickstart snippet still current",
                "examples/quickstart.rs#L10-L24",
                Anchor::CodeSnippet,
                Corpus::GuideTutorial,
                Source::FirstPartyHosted,
                Scope::LatestStable,
                Status::ExampleCurrent,
                Fresh::LiveCurrent,
                "",
                "",
                "open:source/quickstart",
            ),
            finding(
                "Client::send signature drifted",
                "src/client.rs#Client::send",
                Anchor::ApiSignature,
                Corpus::ApiReference,
                Source::AiDerived,
                Scope::NearbyVersion,
                Status::ApiSignatureDrifted,
                Fresh::RecentlySynced,
                "api-1.2",
                "api-1.4",
                "open:source/client-send",
            ),
        ],
    ));

    // 2. Help pack panel — a tracking-current API pack and an offline-only community pack;
    //    a config example claiming current but stale (pending reverify) and a deprecated
    //    vendor symbol finding.
    rows.push(base_row(
        M5DocsPackConsumerSurface::HelpPackPanel,
        M5DocsQualificationClass::Stable,
        "Help pack panel owner",
        "The help pack panel renders the shared primitives so an unpinned API pack reads as tracking-current while an offline community pack reads as offline-only, and a config example claiming current with stale freshness is held for reverification rather than shown as verified",
        "evidence:m5-pack-finding-help:001",
        vec![
            pack(
                "api-reference-pack",
                Corpus::ApiReference,
                Source::FirstPartyHosted,
                Scope::LatestStable,
                Pack::UnpinnedTracking,
                Fresh::LiveCurrent,
                Verify::ChecksumOnly,
                256,
                5_242_880,
                "signer:aureline-release",
                "refresh:2026-07-05T20:00Z",
                "manage:pack/api-reference-pack",
            ),
            pack(
                "community-plugins-pack",
                Corpus::CommunityContributed,
                Source::OfflineImport,
                Scope::PinnedRange,
                Pack::OfflinePack,
                Fresh::CachedOffline,
                Verify::ChecksumOnly,
                64,
                1_310_720,
                "",
                "refresh:2026-06-20T12:00Z",
                "manage:pack/community-plugins-pack",
            ),
        ],
        vec![
            finding(
                "Config example needs reverify",
                "docs/config/logging.toml",
                Anchor::ConfigShape,
                Corpus::FirstPartyDocs,
                Source::BundledLocal,
                Scope::ProjectSpecific,
                Status::ExampleCurrent,
                Fresh::StaleExpired,
                "cfg-1",
                "cfg-1",
                "open:source/config-logging",
            ),
            finding(
                "Vendor deprecated symbol used",
                "src/vendor/auth.rs#legacy_login",
                Anchor::CodeSnippet,
                Corpus::VendorDependency,
                Source::ThirdPartyHosted,
                Scope::PinnedRange,
                Status::DeprecatedSymbolUsed,
                Fresh::CachedOffline,
                "v2",
                "v3",
                "open:source/vendor-auth",
            ),
        ],
    ));

    // 3. Onboarding pack step — an update-overdue API pack and a stale-needs-refresh
    //    bundled guide pack; a broken-link finding into release notes.
    rows.push(base_row(
        M5DocsPackConsumerSurface::OnboardingPackStep,
        M5DocsQualificationClass::Stable,
        "Onboarding pack step owner",
        "The onboarding pack step renders the shared primitives so an update-available pack reads as update-overdue and a stale bundled guide pack reads as stale-needs-refresh — distinct states, never one generic warning — and a broken example link becomes an actionable finding",
        "evidence:m5-pack-finding-onboarding:001",
        vec![
            pack(
                "getting-started-pack",
                Corpus::ApiReference,
                Source::FirstPartyHosted,
                Scope::LatestStable,
                Pack::UpdateAvailable,
                Fresh::RecentlySynced,
                Verify::SignatureVerified,
                120,
                3_145_728,
                "signer:aureline-release",
                "refresh:2026-07-01T08:00Z",
                "manage:pack/getting-started-pack",
            ),
            pack(
                "tutorials-pack",
                Corpus::GuideTutorial,
                Source::BundledLocal,
                Scope::PinnedRange,
                Pack::PinnedPack,
                Fresh::StaleExpired,
                Verify::SignatureVerified,
                80,
                1_572_864,
                "signer:aureline-release",
                "refresh:2026-05-15T08:00Z",
                "manage:pack/tutorials-pack",
            ),
        ],
        vec![finding(
            "Release-notes link broken",
            "docs/release/1.4.md#upgrade",
            Anchor::LinkTarget,
            Corpus::ReleaseNotesChangelog,
            Source::MirroredRegistry,
            Scope::LatestStable,
            Status::BrokenLinkTarget,
            Fresh::UnknownFreshness,
            "",
            "",
            "open:source/release-1-4",
        )],
    ));

    // 4. AI pack context — a quarantined community pack (untrusted); a version-mismatch
    //    shell-command finding with concrete version drift.
    rows.push(base_row(
        M5DocsPackConsumerSurface::AiPackContext,
        M5DocsQualificationClass::Stable,
        "AI pack-context owner",
        "The AI pack-context panel renders the shared primitives so a quarantined community pack reads as quarantined-untrusted and is never cited as trusted, and a version-mismatched CLI example becomes an actionable finding with the documented and current versions on the row",
        "evidence:m5-pack-finding-ai:001",
        vec![pack(
            "unreviewed-community-pack",
            Corpus::CommunityContributed,
            Source::ThirdPartyHosted,
            Scope::Unversioned,
            Pack::QuarantinedPack,
            Fresh::UnknownFreshness,
            Verify::Unverified,
            33,
            720_896,
            "",
            "refresh:2026-06-30T18:00Z",
            "manage:pack/unreviewed-community-pack",
        )],
        vec![
            finding(
                "CLI example version mismatch",
                "docs/cli/deploy.md#example",
                Anchor::ShellCommand,
                Corpus::GuideTutorial,
                Source::OfflineImport,
                Scope::Unversioned,
                Status::VersionMismatchExample,
                Fresh::RecentlySynced,
                "cli-1.0",
                "cli-2.0",
                "open:source/cli-deploy",
            ),
            finding(
                "Unverified config example",
                "docs/config/plugins.toml",
                Anchor::ConfigShape,
                Corpus::CommunityContributed,
                Source::FirstPartyHosted,
                Scope::PinnedRange,
                Status::UnverifiedExample,
                Fresh::UnknownFreshness,
                "",
                "",
                "open:source/config-plugins",
            ),
        ],
    ));

    // 5. Support pack evidence — a verification-failed pinned pack (untrusted); a
    //    deprecated-symbol finding kept anchored for support review.
    rows.push(base_row(
        M5DocsPackConsumerSurface::SupportPackEvidence,
        M5DocsQualificationClass::Stable,
        "Support pack evidence owner",
        "The support pack evidence view renders the shared primitives so a pinned pack that failed verification reads as verification-unverified — never trusted — and every finding keeps its concrete anchor and source descriptors so identity survives the support/AI evidence path",
        "evidence:m5-pack-finding-support:001",
        vec![pack(
            "codebase-symbols-pack",
            Corpus::CodebaseSymbol,
            Source::FirstPartyHosted,
            Scope::ExactVersionMatch,
            Pack::PinnedPack,
            Fresh::LiveCurrent,
            Verify::VerificationFailed,
            510,
            9_437_184,
            "signer:unknown",
            "refresh:2026-07-05T23:00Z",
            "manage:pack/codebase-symbols-pack",
        )],
        vec![finding(
            "Deprecated helper in support snippet",
            "support/snippets/retry.rs#retry_backoff",
            Anchor::CodeSnippet,
            Corpus::ApiReference,
            Source::MirroredRegistry,
            Scope::PinnedRange,
            Status::DeprecatedSymbolUsed,
            Fresh::CachedOffline,
            "retry-0.9",
            "retry-1.0",
            "open:source/retry-backoff",
        )],
    ));

    rows
}

fn governance_review() -> M5DocsPackFindingGovernanceReview {
    M5DocsPackFindingGovernanceReview {
        shared_primitives_carry_truth: true,
        pack_states_stay_distinct: true,
        quarantined_or_stale_never_shown_trusted: true,
        verification_state_visible: true,
        example_drift_actionable_with_anchor: true,
        drifted_example_never_shown_current: true,
        version_drift_context_visible: true,
        actions_keep_mirror_offline_export_parity: true,
        no_surface_invents_second_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsPackFindingConsumerProjection {
    M5DocsPackFindingConsumerProjection {
        consumers_consume_shared_primitives: true,
        trust_posture_reads_single_source: true,
        drift_posture_reads_single_source: true,
        actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DocsPackFindingProofFreshness {
    M5DocsPackFindingProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsPackFindingReleasePosture {
    M5DocsPackFindingReleasePosture {
        proof_packet_ref: M5_DOCS_PACK_FINDING_ARTIFACT_REF.to_owned(),
        pack_finding_audit_ref: M5_DOCS_PACK_FINDING_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_PACK_FINDING_SCHEMA_REF,
        M5_DOCS_PACK_FINDING_DOC_REF,
        M5_DOCS_PACK_FINDING_COMPONENT_MATRIX_REF,
        M5_DOCS_PACK_FINDING_SOURCE_RESULT_REF,
        M5_DOCS_PACK_FINDING_SOURCE_PRECEDENCE_REF,
    ])
}

/// Builds the canonical M5 docs-pack-row / stale-example-finding-row primitive packet.
pub fn seeded_m5_pack_finding_primitive_packet() -> M5DocsPackFindingPrimitivePacket {
    M5DocsPackFindingPrimitivePacket::new(M5DocsPackFindingPrimitivePacketInput {
        packet_id: M5_DOCS_PACK_FINDING_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs-pack row and stale-example finding row primitive: pack lifecycle, verification, trust posture, example drift, version-drift context, and pin/offline/refresh/quarantine/update/remove actions"
                .to_owned(),
        pack_finding_rows: pack_finding_rows(),
        vocabulary_set: M5DocsPackFindingVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the onboarding pack step is held at Beta because a slice of
/// onboarding surfaces do not yet render the pack-state badge on every profile; every
/// consumer stays visible.
pub fn seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed(
) -> M5DocsPackFindingPrimitivePacket {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.packet_id =
        "m5-docs-pack-row-and-stale-example-finding-row-primitive:onboarding-beta:0001".to_owned();
    let row = packet
        .pack_finding_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsPackConsumerSurface::OnboardingPackStep)
        .expect("onboarding pack step row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI pack context is narrowed to Preview pending drift-disclosure
/// parity proof across every AI-context export path; every consumer stays visible.
pub fn seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed(
) -> M5DocsPackFindingPrimitivePacket {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.packet_id =
        "m5-docs-pack-row-and-stale-example-finding-row-primitive:ai-pack-context-preview:0001"
            .to_owned();
    let row = packet
        .pack_finding_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DocsPackConsumerSurface::AiPackContext)
        .expect("ai pack context row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
