//! Canonical seed builders for the frozen M5 docs-browser component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical docs-browser-component matrix.
pub const M5_DOCS_BROWSER_MATRIX_PACKET_ID: &str = "m5-docs-browser-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5DocsRequiredLabel> {
    M5DocsRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5DocsRequiredLabel]) -> Vec<M5DocsRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5DocsBrowserComponentFamily,
    qualification: M5DocsQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
) -> M5DocsBrowserComponentRow {
    M5DocsBrowserComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5DocsSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DocsDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        corpus_classes: vec![],
        version_scopes: vec![],
        match_states: vec![],
        override_reasons: vec![],
        symbol_anchors: vec![],
        source_providers: vec![],
        freshness_states: vec![],
        pack_states: vec![],
        stale_example_statuses: vec![],
        handoff_reasons: vec![],
        accessibility_routes: M5DocsAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5DocsConsumerSurface::DocsBrowserUi,
            M5DocsConsumerSurface::SupportExport,
            M5DocsConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5DocsDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DOCS_BROWSER_SCHEMA_REF,
            M5_DOCS_BROWSER_SOURCE_RESULT_REF,
        ]),
        masks_corpus_or_source_provenance: false,
        shows_stale_or_cached_as_live_current: false,
        invents_private_docs_status_grammar: false,
        hides_handoff_reason_or_override_reason: false,
    }
}

fn component_rows() -> Vec<M5DocsBrowserComponentRow> {
    use M5DocsBrowserComponentFamily as F;
    use M5DocsConsumerSurface as C;
    use M5DocsCorpusClass as CC;
    use M5DocsDowngradeTrigger as D;
    use M5DocsFreshnessState as FR;
    use M5DocsHandoffReason as HR;
    use M5DocsMatchState as MS;
    use M5DocsOverrideReason as OR;
    use M5DocsPackState as PK;
    use M5DocsQualificationClass as Q;
    use M5DocsRequiredLabel as L;
    use M5DocsSourceProvider as SP;
    use M5DocsStaleExampleStatus as SE;
    use M5DocsSymbolAnchor as SA;
    use M5DocsVersionScope as VS;

    let mut rows = Vec::new();

    // 1. Docs search bar.
    let mut row = base_row(
        F::DocsSearchBar,
        Q::Stable,
        "Docs-search component owner",
        "One docs-search-bar model naming every corpus it can search — first-party docs, API reference, guides, codebase symbols, community, vendor, or changelog — and the source provider behind each, so a search never leaves the corpus or the origin implicit",
        "evidence:m5-docs-search-bar-parity:001",
    );
    row.corpus_classes = CC::ALL.to_vec();
    row.required_labels = labels_with(&[L::CorpusClass, L::SourceProvider]);
    row.consumer_surfaces = vec![
        C::SearchPalette,
        C::DocsBrowserUi,
        C::AiContextPanel,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CorpusClassUnstated,
        D::SourceProviderMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Docs scope switcher.
    let mut row = base_row(
        F::DocsScopeSwitcher,
        Q::Stable,
        "Docs-scope component owner",
        "One docs-scope-switcher model naming the version / package scope in effect — exact, nearby, project-specific, latest-stable, pinned-range, or unversioned — so a user always knows which version of the docs they are reading",
        "evidence:m5-docs-scope-switcher-parity:001",
    );
    row.version_scopes = VS::ALL.to_vec();
    row.required_labels = labels_with(&[L::Freshness]);
    row.consumer_surfaces = vec![
        C::SearchPalette,
        C::DocsBrowserUi,
        C::HelpAbout,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::VersionScopeUnstated, D::ProofStale];
    rows.push(row);

    // 3. Docs result row.
    let mut row = base_row(
        F::DocsResultRow,
        Q::Stable,
        "Docs-result component owner",
        "One docs-result-row model naming its match state — exact, nearby, project-specific, mirrored, cached, or stale — and why a project doc outranked vendor docs, so a nearby or cached hit is never presented as an exact live one and reordering is never silent",
        "evidence:m5-docs-result-row-parity:001",
    );
    row.match_states = MS::ALL.to_vec();
    row.override_reasons = OR::ALL.to_vec();
    row.required_labels = labels_with(&[L::CorpusClass, L::SourceProvider, L::Freshness]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::SearchPalette,
        C::AiContextPanel,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ProjectOverrideReasonHidden,
        D::MirroredOrCachedShownAsLive,
        D::FreshnessHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Symbol-linked reference card.
    let mut row = base_row(
        F::SymbolLinkedReferenceCard,
        Q::Stable,
        "Symbol-reference component owner",
        "One symbol-linked-reference-card model naming the code entity it anchors — function, type, module, field/method, macro, or an unresolved anchor — so a reference card never shows an unresolved or drifted anchor as a resolved deep link",
        "evidence:m5-symbol-linked-reference-card-parity:001",
    );
    row.symbol_anchors = SA::ALL.to_vec();
    row.required_labels = labels_with(&[L::CorpusClass]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::HoverPeek,
        C::AiContextPanel,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::SymbolAnchorUnresolvedHidden, D::ProofStale];
    rows.push(row);

    // 5. Docs source / version badge.
    let mut row = base_row(
        F::DocsSourceVersionBadge,
        Q::Stable,
        "Docs-source-badge component owner",
        "One docs-source-version-badge model naming the source provider — bundled-local, mirrored-registry, first-party-hosted, third-party-hosted, offline-import, or AI-derived — and the freshness of the content, so mirrored or cached documentation is never shown as live first-party truth",
        "evidence:m5-docs-source-version-badge-parity:001",
    );
    row.source_providers = SP::ALL.to_vec();
    row.freshness_states = FR::ALL.to_vec();
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::HoverPeek,
        C::HelpAbout,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SourceProviderMasked,
        D::FreshnessHidden,
        D::MirroredOrCachedShownAsLive,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Docs-pack row.
    let mut row = base_row(
        F::DocsPackRow,
        Q::Stable,
        "Docs-pack component owner",
        "One docs-pack-row model naming whether the pack is pinned, mirrored, offline, quarantined, tracking upstream, or has an update available, so a quarantined or offline pack is never presented as a freely trusted, up-to-date source",
        "evidence:m5-docs-pack-row-parity:001",
    );
    row.pack_states = PK::ALL.to_vec();
    row.required_labels = labels_with(&[L::SourceProvider, L::Freshness]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::AdminConsole,
        C::HelpAbout,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PackStateMisrepresented,
        D::QuarantinedPackShownAsTrusted,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Stale-example finding row.
    let mut row = base_row(
        F::StaleExampleFindingRow,
        Q::Stable,
        "Stale-example component owner",
        "One stale-example-finding-row model naming the integrity of a documented example — current, API-signature-drifted, deprecated-symbol, broken-link, version-mismatch, or unverified — so a drifted or broken example is never shown as current, runnable guidance",
        "evidence:m5-stale-example-finding-row-parity:001",
    );
    row.stale_example_statuses = SE::ALL.to_vec();
    row.required_labels = labels_with(&[L::Freshness]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::AiContextPanel,
        C::AdminConsole,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::StaleExampleShownAsCurrent, D::ProofStale];
    rows.push(row);

    // 8. Docs handoff banner.
    let mut row = base_row(
        F::DocsHandoffBanner,
        Q::Stable,
        "Docs-handoff component owner",
        "One docs-handoff-banner model naming exactly why Aureline had to hand a docs task off to a browser — no local corpus, interactive content, auth-gated source, dynamic rendering, external canonical source, or an explicit user request — so a handoff is never a silent dead-end",
        "evidence:m5-docs-handoff-banner-parity:001",
    );
    row.handoff_reasons = HR::ALL.to_vec();
    row.required_labels = labels_with(&[L::SourceProvider]);
    row.consumer_surfaces = vec![
        C::DocsBrowserUi,
        C::HelpAbout,
        C::AiContextPanel,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::HandoffReasonUnstated, D::ProofStale];
    rows.push(row);

    rows
}

fn governance_review() -> M5DocsBrowserGovernanceReview {
    M5DocsBrowserGovernanceReview {
        search_bar_shows_corpus_and_source: true,
        scope_switcher_shows_version_scope: true,
        result_row_shows_match_state_and_override_reason: true,
        symbol_card_shows_anchor_and_resolution: true,
        source_badge_shows_provider_and_freshness: true,
        pack_row_shows_pin_mirror_offline_quarantine: true,
        stale_example_row_shows_stale_status: true,
        handoff_banner_shows_reason: true,
        live_versus_cached_never_conflated: true,
        no_component_invents_second_status_grammar: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5DocsBrowserConsumerProjection {
    M5DocsBrowserConsumerProjection {
        search_and_result_surfaces_consume_corpus_vocabulary: true,
        badge_surfaces_consume_source_and_freshness_vocabulary: true,
        pack_surfaces_consume_pack_state_vocabulary: true,
        handoff_surfaces_consume_handoff_reason_vocabulary: true,
        support_export_reads_single_source: true,
        onboarding_and_ai_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5DocsBrowserProofFreshness {
    M5DocsBrowserProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsBrowserReleasePosture {
    M5DocsBrowserReleasePosture {
        proof_packet_ref: M5_DOCS_BROWSER_ARTIFACT_REF.to_owned(),
        docs_audit_ref: M5_DOCS_BROWSER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DOCS_BROWSER_SCHEMA_REF,
        M5_DOCS_BROWSER_DOC_REF,
        M5_DOCS_BROWSER_SOURCE_RESULT_REF,
        M5_DOCS_BROWSER_SYMBOL_REF,
        M5_DOCS_BROWSER_PACK_REF,
        M5_DOCS_BROWSER_HANDOFF_REF,
    ])
}

/// Builds the canonical frozen M5 docs-browser-component matrix packet.
pub fn seeded_m5_docs_browser_component_matrix() -> M5DocsBrowserMatrixPacket {
    M5DocsBrowserMatrixPacket::new(M5DocsBrowserMatrixPacketInput {
        packet_id: M5_DOCS_BROWSER_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 docs-search-bar, docs-scope-switcher, docs-result-row, symbol-linked-reference-card, docs-source-version-badge, docs-pack-row, stale-example-finding-row, and docs-handoff-banner component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5DocsBrowserVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the stale-example finding row is held at Beta because a slice
/// of stale-example statuses do not yet round-trip across every export path; every
/// component stays visible.
pub fn seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed(
) -> M5DocsBrowserMatrixPacket {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.packet_id = "m5-docs-browser-components:stale-example-finding-row-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::StaleExampleFindingRow)
        .expect("stale-example-finding-row row present");
    row.qualification = M5DocsQualificationClass::Beta;
    packet
}

/// Narrowed variant: the docs handoff banner is narrowed to Preview pending
/// handoff-reason parity proof across every browser surface; every component stays
/// visible.
pub fn seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed(
) -> M5DocsBrowserMatrixPacket {
    let mut packet = seeded_m5_docs_browser_component_matrix();
    packet.packet_id = "m5-docs-browser-components:handoff-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5DocsBrowserComponentFamily::DocsHandoffBanner)
        .expect("docs-handoff-banner row present");
    row.qualification = M5DocsQualificationClass::Preview;
    packet
}
