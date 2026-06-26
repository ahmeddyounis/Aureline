//! Canonical seed builders for the frozen M5 content-wording matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical content-wording matrix.
pub const M5_CONTENT_WORDING_MATRIX_PACKET_ID: &str = "m5-content-wording-matrix:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tokens_for(vocab: M5ContentStateVocabulary) -> ContentTokenVecs {
    use M5ContentStateVocabulary as V;
    let mut vecs = ContentTokenVecs::default();
    match vocab {
        V::LifecycleState => vecs.lifecycle_states = ContentLifecycleState::ALL.to_vec(),
        V::TrustClass => vecs.trust_classes = ContentTrustClass::ALL.to_vec(),
        V::PolicyState => vecs.policy_states = ContentPolicyState::ALL.to_vec(),
        V::CompatibilityState => {
            vecs.compatibility_states = ContentCompatibilityState::ALL.to_vec()
        }
        V::FreshnessState => vecs.freshness_states = ContentFreshnessState::ALL.to_vec(),
        V::ClientScope => vecs.client_scopes = ContentClientScope::ALL.to_vec(),
        V::HostingBoundary => vecs.hosting_boundaries = ContentHostingBoundary::ALL.to_vec(),
        V::EditionLabel => vecs.edition_labels = ContentEditionLabel::ALL.to_vec(),
    }
    vecs
}

#[derive(Default)]
struct ContentTokenVecs {
    lifecycle_states: Vec<ContentLifecycleState>,
    trust_classes: Vec<ContentTrustClass>,
    policy_states: Vec<ContentPolicyState>,
    compatibility_states: Vec<ContentCompatibilityState>,
    freshness_states: Vec<ContentFreshnessState>,
    client_scopes: Vec<ContentClientScope>,
    hosting_boundaries: Vec<ContentHostingBoundary>,
    edition_labels: Vec<ContentEditionLabel>,
}

impl ContentTokenVecs {
    fn merge(&mut self, other: ContentTokenVecs) {
        self.lifecycle_states.extend(other.lifecycle_states);
        self.trust_classes.extend(other.trust_classes);
        self.policy_states.extend(other.policy_states);
        self.compatibility_states.extend(other.compatibility_states);
        self.freshness_states.extend(other.freshness_states);
        self.client_scopes.extend(other.client_scopes);
        self.hosting_boundaries.extend(other.hosting_boundaries);
        self.edition_labels.extend(other.edition_labels);
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    object_kind: M5ContentObjectKind,
    qualification: M5ContentQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    required_fields: &[&str],
    evidence_requirement: M5ContentEvidenceRequirement,
    required_proof_packet_refs: &[&str],
    downgrade_triggers: Vec<M5ContentDowngradeTrigger>,
    rollback_posture: M5ContentRollbackPosture,
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5ContentConsumerSurface>,
) -> M5ContentObjectRow {
    // Declared vocabularies come straight from the object kind so the row's
    // token vecs and declared list cannot disagree.
    let state_vocabularies = object_kind.required_state_vocabularies().to_vec();
    let mut tokens = ContentTokenVecs::default();
    for vocab in &state_vocabularies {
        tokens.merge(tokens_for(*vocab));
    }

    M5ContentObjectRow {
        object_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_fields: strings(required_fields),
        state_vocabularies,
        lifecycle_states: tokens.lifecycle_states,
        trust_classes: tokens.trust_classes,
        policy_states: tokens.policy_states,
        compatibility_states: tokens.compatibility_states,
        freshness_states: tokens.freshness_states,
        client_scopes: tokens.client_scopes,
        hosting_boundaries: tokens.hosting_boundaries,
        edition_labels: tokens.edition_labels,
        evidence_requirement,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        downgrade_triggers,
        rollback_posture,
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

fn object_rows() -> Vec<M5ContentObjectRow> {
    use M5ContentConsumerSurface as S;
    use M5ContentDowngradeTrigger as D;
    vec![
        row(
            M5ContentObjectKind::SafetyCriticalUiString,
            M5ContentQualificationClass::Stable,
            "Product copy owner",
            "Safety-critical UI string for trust, policy, destructive, recovery, and degraded-state surfaces; carries a stable message id and controlled terms so the same reserved meaning survives UI, CLI, docs, exports, accessibility strings, and support packets",
            &[
                "message_id",
                "controlled_terms",
                "lifecycle_state",
                "trust_class",
                "policy_state",
                "freshness_state",
                "surface_family",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:safety-critical-string-conformance:m5"],
            vec![
                D::MessageIdUnstable,
                D::ControlledTermDrift,
                D::LocalizationParityLost,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::MessageIdStablePreserved,
            &[
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
                M5_CONTENT_CONTROLLED_GLOSSARY_REF,
                M5_CONTENT_PRODUCT_TRUTH_VOCABULARY_REF,
            ],
            vec![
                S::ProductUi,
                S::CliHelp,
                S::Docs,
                S::SupportExport,
                S::HelpAbout,
            ],
        ),
        row(
            M5ContentObjectKind::GlossaryTerm,
            M5ContentQualificationClass::Stable,
            "Design systems owner",
            "Controlled glossary / state-label term with one reserved meaning, alias posture, and allowed surfaces; the same visible word keeps the same meaning everywhere and is never repurposed without a controlled alias",
            &[
                "term_id",
                "display_label",
                "reserved_meaning",
                "lifecycle_state",
                "trust_class",
                "client_scope",
                "allowed_surfaces",
                "alias_posture",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:controlled-glossary-conformance:m5"],
            vec![
                D::ControlledTermDrift,
                D::LocalizationParityLost,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::TermLabeledNeverSoftened,
            &[
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
                M5_CONTENT_CONTROLLED_GLOSSARY_REF,
            ],
            vec![
                S::ProductUi,
                S::CliHelp,
                S::Docs,
                S::SupportExport,
                S::Onboarding,
            ],
        ),
        row(
            M5ContentObjectKind::ActionLabelPattern,
            M5ContentQualificationClass::Stable,
            "Product copy owner",
            "Verb-first, outcome-specific action-label pattern; consequential actions never ship a standalone vague label and a narrowed client scope is disclosed, never implied as full desktop parity",
            &[
                "pattern_id",
                "verb",
                "object_or_scope",
                "policy_state",
                "client_scope",
                "destructive_posture",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:action-label-pattern-conformance:m5"],
            vec![
                D::ControlledTermDrift,
                D::ScopeCountDishonest,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::ScopeCountStaysHonest,
            &[
                M5_CONTENT_UI_COPY_CONTRACT_REF,
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
            ],
            vec![S::ProductUi, S::CliHelp, S::SupportExport, S::Docs],
        ),
        row(
            M5ContentObjectKind::ErrorRecoveryBlock,
            M5ContentQualificationClass::Stable,
            "Supportability owner",
            "Four-part error / recovery block that names what failed, why it likely failed, what still works, and the next safe action; degraded states always disclose remaining capability instead of a generic failure",
            &[
                "error_id",
                "what_failed",
                "likely_cause",
                "what_still_works",
                "next_safe_action",
                "policy_state",
                "freshness_state",
                "severity_class",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:error-recovery-block-conformance:m5"],
            vec![
                D::ControlledTermDrift,
                D::FreshnessExpired,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::TermLabeledNeverSoftened,
            &[
                M5_CONTENT_UI_COPY_CONTRACT_REF,
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
            ],
            vec![
                S::ProductUi,
                S::CliHelp,
                S::SupportExport,
                S::Docs,
                S::HelpAbout,
            ],
        ),
        row(
            M5ContentObjectKind::AiCopyGuardrail,
            M5ContentQualificationClass::Beta,
            "AI product owner",
            "AI copy guardrail governing certainty, evidence, context, and autonomy language; AI wording never overstates confidence or autonomy, never claims false validation or freshness, and never obscures the route or spend truth",
            &[
                "guardrail_id",
                "certainty_language",
                "evidence_basis",
                "trust_class",
                "policy_state",
                "freshness_state",
                "autonomy_disclosure",
            ],
            M5ContentEvidenceRequirement::Required,
            &[
                "evidence:ai-copy-guardrail-conformance:m5",
                "evidence:ai-overclaim-rejection-corpus:m5",
            ],
            vec![
                D::OverclaimDetected,
                D::FreshnessExpired,
                D::ControlledTermDrift,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::OverclaimBlockedBeforeShip,
            &[
                M5_CONTENT_AI_COPY_GUARDRAILS_CONTRACT_REF,
                M5_CONTENT_UI_COPY_CONTRACT_REF,
            ],
            vec![
                S::AiSurfaces,
                S::ProductUi,
                S::SupportExport,
                S::Docs,
                S::ReleaseNotes,
            ],
        ),
        row(
            M5ContentObjectKind::CountScopePhraseSet,
            M5ContentQualificationClass::Stable,
            "Design systems owner",
            "Count / scope / freshness phrase set that keeps visible, loaded, selected, and all-matching counts scope-honest, names omission reasons, and never lets a cached or stale count imply proven-current authority",
            &[
                "phrase_set_id",
                "count_scope_terms",
                "omission_reasons",
                "count_status",
                "freshness_state",
                "compatibility_state",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:count-scope-phrase-set-conformance:m5"],
            vec![
                D::ScopeCountDishonest,
                D::FreshnessExpired,
                D::ControlledTermDrift,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::ScopeCountStaysHonest,
            &[
                M5_CONTENT_COUNT_SCOPE_GRAMMAR_REF,
                M5_CONTENT_COUNT_SCOPE_TERM_SET_REF,
            ],
            vec![S::ProductUi, S::CliHelp, S::SupportExport, S::Docs],
        ),
        row(
            M5ContentObjectKind::ContentOpsArtifact,
            M5ContentQualificationClass::Stable,
            "Docs owner",
            "Content-ops metadata artifact that pins version and source metadata on docs, help, exports, and screenshots/demos so a captured artifact discloses its build, source, and compatibility basis instead of implying current authority",
            &[
                "artifact_id",
                "source_ref",
                "version_metadata",
                "capture_basis",
                "compatibility_state",
                "freshness_state",
            ],
            M5ContentEvidenceRequirement::Required,
            &["evidence:content-ops-metadata-conformance:m5"],
            vec![
                D::ContentOpsMetadataMissing,
                D::FreshnessExpired,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::BoundaryWordingMatchesProduct,
            &[
                M5_CONTENT_CONTENT_OPS_CONTRACT_REF,
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
            ],
            vec![
                S::Docs,
                S::SupportExport,
                S::ReleaseNotes,
                S::ScreenshotsDemos,
                S::HelpAbout,
            ],
        ),
        row(
            M5ContentObjectKind::CommercialBoundaryWording,
            M5ContentQualificationClass::Beta,
            "Commercial boundary owner",
            "Commercial-boundary wording review for hosted / open / self-hosted / managed language; hosting boundary, edition label, and client scope cannot drift from the actual deployment profile, and open or local-independent language is never used when managed services participated",
            &[
                "review_id",
                "claimed_edition_label",
                "claimed_hosting_boundary",
                "claimed_client_scope",
                "actual_deployment_profile_ref",
                "boundary_match_finding",
            ],
            M5ContentEvidenceRequirement::Required,
            &[
                "evidence:commercial-boundary-wording-conformance:m5",
                "evidence:deployment-profile-parity:m5",
            ],
            vec![
                D::CommercialBoundaryDrift,
                D::PolicyBlocked,
                D::UpstreamDependencyNarrowed,
                D::ProofStale,
            ],
            M5ContentRollbackPosture::BoundaryWordingMatchesProduct,
            &[
                M5_CONTENT_DEPLOYMENT_PROFILES_REF,
                M5_CONTENT_PRODUCT_TRUTH_VOCABULARY_REF,
                M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
            ],
            vec![
                S::HelpAbout,
                S::Marketplace,
                S::ReleaseNotes,
                S::Docs,
                S::SupportExport,
                S::ProductUi,
            ],
        ),
    ]
}

fn trust_review() -> M5ContentTrustReview {
    M5ContentTrustReview {
        safety_critical_strings_use_stable_ids: true,
        safety_critical_strings_use_controlled_terms: true,
        action_labels_and_counts_scope_honest: true,
        error_copy_explains_failure_remaining_capability_and_next_action: true,
        ai_wording_never_overstates_confidence_or_autonomy: true,
        content_ops_artifacts_keep_version_and_source_metadata: true,
        commercial_boundary_wording_matches_product_boundary: true,
        controlled_terms_never_softened_for_tone: true,
        one_controlled_term_inventory_not_parallel_copy_lists: true,
        no_speculative_brand_or_marketing_campaign_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ContentConsumerProjection {
    M5ContentConsumerProjection {
        product_ui_consumes_object_model: true,
        cli_help_shows_controlled_terms: true,
        docs_shows_content_ops_metadata: true,
        support_export_shows_object_model: true,
        release_notes_use_controlled_vocabulary: true,
        screenshots_demos_carry_version_source_metadata: true,
        ai_surfaces_honor_copy_guardrails: true,
        onboarding_uses_controlled_terms: true,
        help_about_shows_commercial_boundary_truth: true,
        preview_labs_label_for_unqualified_objects: true,
    }
}

fn proof_freshness() -> M5ContentProofFreshness {
    M5ContentProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ContentReleasePosture {
    M5ContentReleasePosture {
        release_packet_ref: "evidence:content-wording-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:content-wording-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CONTENT_WORDING_MATRIX_SCHEMA_REF,
        M5_CONTENT_WORDING_MATRIX_DOC_REF,
        M5_CONTENT_UI_COPY_CONTRACT_REF,
        M5_CONTENT_NAMING_LABEL_CONTRACT_REF,
        M5_CONTENT_COUNT_SCOPE_GRAMMAR_REF,
        M5_CONTENT_CONTENT_OPS_CONTRACT_REF,
        M5_CONTENT_AI_COPY_GUARDRAILS_CONTRACT_REF,
        M5_CONTENT_CONTROLLED_GLOSSARY_REF,
        M5_CONTENT_COUNT_SCOPE_TERM_SET_REF,
        M5_CONTENT_PRODUCT_TRUTH_VOCABULARY_REF,
        M5_CONTENT_DEPLOYMENT_PROFILES_REF,
    ])
}

fn base_input() -> M5ContentWordingMatrixPacketInput {
    M5ContentWordingMatrixPacketInput {
        packet_id: M5_CONTENT_WORDING_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 Content-Design, Controlled-Vocabulary, Content-Ops, and Commercial-Boundary Wording Matrix"
                .to_owned(),
        object_rows: object_rows(),
        vocabulary_set: M5ContentVocabularySet::canonical(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable M5 content-wording matrix packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_content_wording_matrix() -> M5ContentWordingMatrixPacket {
    M5ContentWordingMatrixPacket::new(base_input())
}

/// Builds a narrowed variant where commercial-boundary wording is held after a
/// boundary-drift finding, proving downgrade narrows the claim rather than hiding
/// the object.
pub fn seeded_m5_content_wording_matrix_commercial_boundary_held() -> M5ContentWordingMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-content-wording-matrix:commercial-boundary-held:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5ContentObjectKind::CommercialBoundaryWording {
            row.qualification = M5ContentQualificationClass::Held;
            // A held object no longer carries a public claim, so proof becomes
            // recommended rather than required; the object stays visible.
            row.evidence_requirement = M5ContentEvidenceRequirement::Recommended;
        }
    }
    M5ContentWordingMatrixPacket::new(input)
}

/// Builds a narrowed variant where the AI copy guardrail is pulled to preview
/// after an overclaim finding, proving auto-narrowing keeps the object visible.
pub fn seeded_m5_content_wording_matrix_ai_guardrail_narrowed() -> M5ContentWordingMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-content-wording-matrix:ai-guardrail-narrowed:0001".to_owned();
    for row in &mut input.object_rows {
        if row.object_kind == M5ContentObjectKind::AiCopyGuardrail {
            row.qualification = M5ContentQualificationClass::Preview;
        }
    }
    M5ContentWordingMatrixPacket::new(input)
}
