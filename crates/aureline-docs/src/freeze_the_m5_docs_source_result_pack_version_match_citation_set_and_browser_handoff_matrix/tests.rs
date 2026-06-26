use super::*;

const PACKET_ID: &str = "m5-docs-contracts-matrix:stable:0001";

fn all_source_classes() -> Vec<DocsContractSourceClass> {
    DocsContractSourceClass::ALL.to_vec()
}

fn all_version_match_states() -> Vec<DocsContractVersionMatchState> {
    DocsContractVersionMatchState::ALL.to_vec()
}

fn all_freshness_states() -> Vec<DocsContractFreshnessState> {
    DocsContractFreshnessState::ALL.to_vec()
}

fn all_trust_classes() -> Vec<DocsContractTrustClass> {
    DocsContractTrustClass::ALL.to_vec()
}

fn all_locale_matches() -> Vec<DocsContractLocaleMatch> {
    DocsContractLocaleMatch::ALL.to_vec()
}

fn all_mirror_postures() -> Vec<DocsContractMirrorOfflinePosture> {
    DocsContractMirrorOfflinePosture::ALL.to_vec()
}

fn all_handoff_reasons() -> Vec<DocsContractBrowserHandoffReason> {
    DocsContractBrowserHandoffReason::ALL.to_vec()
}

fn all_handoff_privacy() -> Vec<DocsContractBrowserHandoffPrivacyConsequence> {
    DocsContractBrowserHandoffPrivacyConsequence::ALL.to_vec()
}

fn fields(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn refs(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn object_rows() -> Vec<M5DocsObjectRow> {
    use M5DocsContractStateVocabulary as V;
    vec![
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::DocsSourceDescriptor,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Docs source descriptor naming a documentation source's class, trust, locale, and mirror/offline posture; project docs never masquerade as vendor docs and live external docs always require an explicit handoff".to_owned(),
            required_fields: fields(&[
                "source_id",
                "source_class",
                "trust_class",
                "locale_match",
                "mirror_offline_posture",
                "pack_revision",
            ]),
            state_vocabularies: vec![V::SourceClass, V::TrustClass, V::LocaleMatch, V::MirrorOfflinePosture],
            source_classes: all_source_classes(),
            version_match_states: vec![],
            freshness_states: vec![],
            trust_classes: all_trust_classes(),
            locale_matches: all_locale_matches(),
            mirror_offline_postures: all_mirror_postures(),
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&["evidence:docs-source-descriptor-conformance:m5"]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::SourceClassUnverified,
                M5DocsContractsDowngradeTrigger::MirrorOffline,
                M5DocsContractsDowngradeTrigger::TrustNarrowing,
                M5DocsContractsDowngradeTrigger::LocaleSkewDetected,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::SourceLabeledNeverImpersonated,
            source_contract_refs: refs(&[M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::DocsSearch,
                M5DocsContractsConsumerSurface::AiExplain,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::ExtensionApi,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::DocsResultObject,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Docs result object returned by docs search / recall; carries source class, trust class, version-match state, and freshness so a captured or stale result never claims live authority".to_owned(),
            required_fields: fields(&[
                "result_id",
                "source_ref",
                "source_class",
                "trust_class",
                "version_match_state",
                "freshness_state",
                "anchor_ref",
            ]),
            state_vocabularies: vec![V::SourceClass, V::TrustClass, V::VersionMatchState, V::FreshnessState],
            source_classes: all_source_classes(),
            version_match_states: all_version_match_states(),
            freshness_states: all_freshness_states(),
            trust_classes: all_trust_classes(),
            locale_matches: vec![],
            mirror_offline_postures: vec![],
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&["evidence:docs-result-object-conformance:m5"]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::SourceVersionMismatch,
                M5DocsContractsDowngradeTrigger::FreshnessExpired,
                M5DocsContractsDowngradeTrigger::TrustNarrowing,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::VersionFreshnessVisible,
            source_contract_refs: refs(&[
                M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF,
                M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF,
            ]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::DocsSearch,
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::AiExplain,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::ExtensionApi,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::DocsPackManifest,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Docs-pack manifest for an installed or mirrored documentation pack; carries source class, version-match state, mirror/offline posture, and locale match with signature state".to_owned(),
            required_fields: fields(&[
                "pack_id",
                "pack_revision",
                "source_class",
                "version_match_state",
                "mirror_offline_posture",
                "locale_match",
                "signature_state",
            ]),
            state_vocabularies: vec![V::SourceClass, V::VersionMatchState, V::MirrorOfflinePosture, V::LocaleMatch],
            source_classes: all_source_classes(),
            version_match_states: all_version_match_states(),
            freshness_states: vec![],
            trust_classes: vec![],
            locale_matches: all_locale_matches(),
            mirror_offline_postures: all_mirror_postures(),
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&["evidence:docs-pack-manifest-conformance:m5"]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::MirrorOffline,
                M5DocsContractsDowngradeTrigger::SourceVersionMismatch,
                M5DocsContractsDowngradeTrigger::FreshnessExpired,
                M5DocsContractsDowngradeTrigger::LocaleSkewDetected,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::VersionFreshnessVisible,
            source_contract_refs: refs(&[M5_DOCS_CONTRACTS_PACK_MANIFEST_CONTRACT_REF]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::DocsSearch,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::ExtensionApi,
                M5DocsContractsConsumerSurface::ReleaseCenter,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::DerivedExplanationCitationSet,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Derived-explanation citation set binding a generated explanation to its citations; the explanation is never primary authority and expires with its citation set".to_owned(),
            required_fields: fields(&[
                "explanation_id",
                "citation_refs",
                "source_class",
                "trust_class",
                "freshness_state",
                "derivation_basis",
            ]),
            state_vocabularies: vec![V::SourceClass, V::TrustClass, V::FreshnessState],
            source_classes: all_source_classes(),
            version_match_states: vec![],
            freshness_states: all_freshness_states(),
            trust_classes: all_trust_classes(),
            locale_matches: vec![],
            mirror_offline_postures: vec![],
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&[
                "evidence:derived-explanation-citation-set-conformance:m5",
            ]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::CitationSetExpired,
                M5DocsContractsDowngradeTrigger::FreshnessExpired,
                M5DocsContractsDowngradeTrigger::TrustNarrowing,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::CitationBoundExpiresWithCitations,
            source_contract_refs: refs(&[
                M5_DOCS_CONTRACTS_DERIVED_EXPLANATION_CONTRACT_REF,
                M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF,
            ]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::AiExplain,
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::ExtensionApi,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::VersionMatchState,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Version-match / freshness state between a documentation source and the active build; never silently upgrades to an exact build match".to_owned(),
            required_fields: fields(&[
                "subject_ref",
                "version_match_state",
                "freshness_state",
                "build_identity",
            ]),
            state_vocabularies: vec![V::VersionMatchState, V::FreshnessState],
            source_classes: vec![],
            version_match_states: all_version_match_states(),
            freshness_states: all_freshness_states(),
            trust_classes: vec![],
            locale_matches: vec![],
            mirror_offline_postures: vec![],
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&["evidence:docs-version-match-conformance:m5"]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::SourceVersionMismatch,
                M5DocsContractsDowngradeTrigger::FreshnessExpired,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::VersionFreshnessVisible,
            source_contract_refs: refs(&[M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::DocsSearch,
                M5DocsContractsConsumerSurface::AiExplain,
                M5DocsContractsConsumerSurface::Onboarding,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::ExtensionApi,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::StaleExampleFinding,
            qualification: M5DocsContractsQualificationClass::Stable,
            scope_summary: "Stale-example finding flagging documented examples that drifted from current behavior; the finding is surfaced rather than hidden and names the drift reason".to_owned(),
            required_fields: fields(&[
                "finding_id",
                "example_ref",
                "version_match_state",
                "freshness_state",
                "drift_reason",
            ]),
            state_vocabularies: vec![V::VersionMatchState, V::FreshnessState],
            source_classes: vec![],
            version_match_states: all_version_match_states(),
            freshness_states: all_freshness_states(),
            trust_classes: vec![],
            locale_matches: vec![],
            mirror_offline_postures: vec![],
            handoff_reasons: vec![],
            handoff_privacy_consequences: vec![],
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&["evidence:stale-example-finding-conformance:m5"]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::SourceVersionMismatch,
                M5DocsContractsDowngradeTrigger::FreshnessExpired,
                M5DocsContractsDowngradeTrigger::UpstreamDependencyNarrowed,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::VersionFreshnessVisible,
            source_contract_refs: refs(&[M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::DocsSearch,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::Diagnostics,
            ],
        },
        M5DocsObjectRow {
            object_kind: M5DocsObjectKind::BrowserHandoffObject,
            qualification: M5DocsContractsQualificationClass::Beta,
            scope_summary: "Browser-handoff object describing why and how the product opens an external surface; names the reason and the privacy consequence and never silently shares context or impersonates a governed docs surface".to_owned(),
            required_fields: fields(&[
                "handoff_id",
                "destination_ref",
                "handoff_reason",
                "privacy_consequence",
                "return_path",
                "trust_class_disclosure",
            ]),
            state_vocabularies: vec![V::BrowserHandoffReason, V::BrowserHandoffPrivacyConsequence],
            source_classes: vec![],
            version_match_states: vec![],
            freshness_states: vec![],
            trust_classes: vec![],
            locale_matches: vec![],
            mirror_offline_postures: vec![],
            handoff_reasons: all_handoff_reasons(),
            handoff_privacy_consequences: all_handoff_privacy(),
            evidence_requirement: M5DocsContractsEvidenceRequirement::Required,
            required_proof_packet_refs: refs(&[
                "evidence:browser-handoff-object-conformance:m5",
                "evidence:docs-browser-truth-packet:m5",
            ]),
            downgrade_triggers: vec![
                M5DocsContractsDowngradeTrigger::HandoffContextLeakRisk,
                M5DocsContractsDowngradeTrigger::PolicyBlocked,
                M5DocsContractsDowngradeTrigger::TrustNarrowing,
                M5DocsContractsDowngradeTrigger::ProofStale,
            ],
            rollback_posture: M5DocsContractsRollbackPosture::HandoffIsolatedReturnPathPreserved,
            source_contract_refs: refs(&[
                M5_DOCS_CONTRACTS_BROWSER_HANDOFF_CONTRACT_REF,
                M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF,
            ]),
            consumer_surfaces: vec![
                M5DocsContractsConsumerSurface::BrowserCompanion,
                M5DocsContractsConsumerSurface::DocsBrowser,
                M5DocsContractsConsumerSurface::SupportExport,
                M5DocsContractsConsumerSurface::HelpAbout,
                M5DocsContractsConsumerSurface::ReleaseCenter,
            ],
        },
    ]
}

fn trust_review() -> M5DocsContractsTrustReview {
    M5DocsContractsTrustReview {
        source_class_locale_version_freshness_visible: true,
        project_docs_never_masquerade_as_vendor: true,
        derived_explanations_never_outlive_citation_sets: true,
        citations_bound_to_source_and_version: true,
        version_match_and_freshness_never_silently_upgraded: true,
        mirror_offline_state_disclosed: true,
        handoff_never_silently_shares_context: true,
        handoff_never_impersonates_governed_docs: true,
        stale_examples_surfaced_not_hidden: true,
        no_speculative_knowledge_platform_or_hosted_search: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5DocsContractsConsumerProjection {
    M5DocsContractsConsumerProjection {
        docs_browser_consumes_object_model: true,
        docs_search_shows_result_object_truth: true,
        ai_explain_shows_citation_set: true,
        onboarding_shows_source_and_freshness: true,
        support_export_shows_object_model: true,
        extension_api_consumes_same_object_model: true,
        release_center_shows_qualification: true,
        help_about_shows_qualification: true,
        browser_companion_shows_handoff_reason_and_privacy: true,
        preview_labs_label_for_unqualified_objects: true,
    }
}

fn proof_freshness() -> M5DocsContractsProofFreshness {
    M5DocsContractsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-24T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DocsContractsReleasePosture {
    M5DocsContractsReleasePosture {
        release_packet_ref: "evidence:docs-contracts-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:docs-contracts-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF,
        M5_DOCS_CONTRACTS_MATRIX_DOC_REF,
        M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF,
        M5_DOCS_CONTRACTS_PACK_MANIFEST_CONTRACT_REF,
        M5_DOCS_CONTRACTS_DERIVED_EXPLANATION_CONTRACT_REF,
        M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF,
        M5_DOCS_CONTRACTS_BROWSER_HANDOFF_CONTRACT_REF,
    ])
}

fn packet() -> M5DocsContractsMatrixPacket {
    M5DocsContractsMatrixPacket::new(M5DocsContractsMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label:
            "M5 Docs Source, Result, Pack, Version-Match, Citation-Set, and Browser-Handoff Matrix"
                .to_owned(),
        object_rows: object_rows(),
        vocabulary_set: M5DocsContractsVocabularySet::canonical(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-24T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_docs_contracts_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_object_fails_validation() {
    let mut packet = packet();
    packet
        .object_rows
        .retain(|row| row.object_kind != M5DocsObjectKind::VersionMatchState);
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = packet();
    packet.vocabulary_set.source_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::VocabularySetDrift));
}

#[test]
fn required_vocabulary_missing_fails() {
    let mut packet = packet();
    // The source descriptor must declare TrustClass; drop it (and its tokens).
    let row = &mut packet.object_rows[0];
    row.state_vocabularies
        .retain(|v| *v != M5DocsContractStateVocabulary::TrustClass);
    row.trust_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::RequiredVocabularyMissing));
}

#[test]
fn declared_vocabulary_without_tokens_fails() {
    let mut packet = packet();
    // Keep SourceClass declared but strip its tokens.
    packet.object_rows[0].source_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::DeclaredVocabularyHasNoTokens));
}

#[test]
fn undeclared_vocabulary_with_tokens_fails() {
    let mut packet = packet();
    // The browser-handoff object does not declare SourceClass; add a token.
    let row = packet
        .object_rows
        .iter_mut()
        .find(|row| row.object_kind == M5DocsObjectKind::BrowserHandoffObject)
        .expect("browser handoff row present");
    row.source_classes
        .push(DocsContractSourceClass::ProjectDocs);
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::UndeclaredVocabularyHasTokens));
}

#[test]
fn stable_object_missing_proof_fails() {
    let mut packet = packet();
    packet.object_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::StableObjectMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.object_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.object_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.project_docs_never_masquerade_as_vendor = false;
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_objects = false;
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = packet();
    packet.release_posture.mirror_offline_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsContractsMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_object() {
    let summary = packet().render_markdown_summary();
    for object in M5DocsObjectKind::ALL {
        assert!(
            summary.contains(object.as_str()),
            "summary missing object {}",
            object.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_docs_contracts_matrix_export()
        .expect("checked M5 docs-contracts matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/browser_handoff_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/mirror_offline_pack_narrowed.json"
        )),
    ] {
        let packet: M5DocsContractsMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
