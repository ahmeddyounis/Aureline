//! Canonical seed builders for the frozen M5 review-pack matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical review-pack matrix.
pub const M5_REVIEW_PACK_MATRIX_PACKET_ID: &str = "m5-review-pack-evaluator:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5ReviewPackRequiredLabel> {
    M5ReviewPackRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(extra: &[M5ReviewPackRequiredLabel]) -> Vec<M5ReviewPackRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5ReviewPackObject,
    qualification: M5ReviewPackQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5ReviewPackVisibleState,
) -> M5ReviewPackRow {
    M5ReviewPackRow {
        object_class,
        qualification,
        parity_state: M5ReviewPackParityState::LocalParityEstimate,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5ReviewPackSurfaceFamily::ALL.to_vec(),
        classification_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        review_pack_record_roles: vec![],
        ownership_signal_roles: vec![],
        required_evidence_roles: vec![],
        local_ci_parity_roles: vec![],
        ai_policy_hook_roles: vec![],
        template_packet_roles: vec![],
        degraded_reasons: M5ReviewPackDegradedReason::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReviewPackConsumerSurface::ReviewDetail,
            M5ReviewPackConsumerSurface::SupportExportPacket,
        ],
        downgrade_triggers: vec![M5ReviewPackDowngradeTrigger::ReviewPackMatrixStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_a_local_parity_estimate_masquerade_as_provider_authoritative: false,
        hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: false,
        flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: false,
        lets_ai_review_run_under_a_different_pack_version_without_disclosure: false,
        loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening: false,
    }
}

fn txn(f: [&str; 7]) -> M5ReviewPackVisibleState {
    M5ReviewPackVisibleState {
        pack_label: f[0].to_owned(),
        pack_version_and_digest: f[1].to_owned(),
        owner_provenance: f[2].to_owned(),
        evaluator_result_class: f[3].to_owned(),
        local_versus_provider_parity: f[4].to_owned(),
        pack_freshness_state: f[5].to_owned(),
        template_attribution: f[6].to_owned(),
    }
}

fn review_pack_rows() -> Vec<M5ReviewPackRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;
    use M5ReviewPackObject as O;
    use M5ReviewPackQualificationClass as Q;
    use M5ReviewPackRequiredLabel as L;
    use M5ReviewPackRole as R;

    let mut rows = Vec::new();

    // 1. ReviewPackRecord.
    let mut row = base_row(
        O::ReviewPackRecord,
        Q::Stable,
        "Review-pack record owner",
        "Review-governance backup owner",
        "One declarative review-pack record shows its version and content digest, names its scope selectors and evaluator identity, shows pack freshness and invalidation (stale-pack, partial-scope, slice-omitted), and never lets its pack version or digest be swapped without disclosure",
        "evidence:m5-review-pack-record-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "review pack record",
            "review pack version 4 with content digest sha256:pack-4-abc",
            "pack version and content digest with the scope selectors and evaluator identity it binds",
            "local parity estimate at pack load; provider-authoritative confirmation happens per check",
            "local pack load, not a provider-authoritative merge decision",
            "pack fresh: fully evaluated at the current base and head",
            "advisory of the repo review-pack summary template used for this pack",
        ]),
    );
    row.review_pack_record_roles = M5ReviewPackRecordRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::PackVersionAndDigestDisclosure,
        R::PackFreshnessAndInvalidationDisclosure,
    ];
    row.required_labels = labels_with(&[L::PackVersionDigest]);
    row.consumer_surfaces = vec![
        C::ReviewDetail,
        C::ReviewPackSummary,
        C::MergeReadiness,
        C::AiReviewPanel,
        C::SupportExportPacket,
    ];
    row.parity_state = M5ReviewPackParityState::LocalParityEstimate;
    row.downgrade_triggers = vec![
        D::PackVersionDigestUnstated,
        D::PackVersionOrDigestDropped,
        D::PackFreshnessUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    // 2. OwnershipSignal.
    let mut row = base_row(
        O::OwnershipSignal,
        Q::Stable,
        "Ownership-signal owner",
        "Review-governance backup owner",
        "One ownership signal shows advisory-owner versus enforced-owner provenance for a named scope slice, shows the owner approval state, flags partial-scope or slice-omitted coverage, and never flattens advisory-owner and enforced-owner into one owner pill",
        "evidence:m5-ownership-signal-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "ownership signal",
            "enforced-owner approval required for the owned scope slice",
            "advisory-versus-enforced owner provenance for the named owned scope slice",
            "provider-authoritative: enforced-owner approval is a real merge gate on the connected provider",
            "enforced owner, distinct from an advisory reviewer suggestion",
            "pack fresh: ownership resolved for the full evaluated scope",
            "advisory of the review-pack ownership summary template",
        ]),
    );
    row.ownership_signal_roles = M5ReviewPackOwnershipSignalRole::ALL.to_vec();
    row.semantic_roles = vec![R::OwnerProvenanceDisclosure];
    row.required_labels = labels_with(&[L::EvaluatorResultClass]);
    row.consumer_surfaces = vec![
        C::ReviewDetail,
        C::OwnershipOverlay,
        C::MergeReadiness,
        C::SupportExportPacket,
    ];
    row.parity_state = M5ReviewPackParityState::ProviderAuthoritative;
    row.downgrade_triggers = vec![
        D::AdvisoryAndEnforcedOwnerFlattened,
        D::OwnerProvenanceUnstated,
        D::EvaluatorResultClassUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    // 3. RequiredEvidenceCheckRow.
    let mut row = base_row(
        O::RequiredEvidenceCheckRow,
        Q::Stable,
        "Required-evidence-row owner",
        "CI-governance backup owner",
        "One required-evidence / required-check row names the demanded evidence or check, shows its evaluator result class, shows ci-only and not-evaluated-here state, names an evidence anchor, and never hides a ci-only, not-evaluated-here, or provider-unavailable check behind a green summary",
        "evidence:m5-required-evidence-check-row-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "required evidence check row",
            "required integration check with a provider-authoritative and a ci-only lane",
            "the required check and the evidence anchor that satisfies it under the pack scope",
            "ci-only: this check runs in CI and is not evaluated locally, never a local green",
            "local parity estimate here; the provider-authoritative result comes from CI",
            "partial scope: the local lane evaluated only the subset it can run",
            "advisory of the review-pack checks summary template",
        ]),
    );
    row.required_evidence_roles = M5ReviewPackRequiredEvidenceRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::EvaluatorResultClassDisclosure,
        R::RequiredEvidenceAndCheckDisclosure,
    ];
    row.required_labels = labels_with(&[L::EvaluatorResultClass]);
    row.consumer_surfaces = vec![
        C::MergeReadiness,
        C::ReviewDetail,
        C::ReviewPackSummary,
        C::SupportExportPacket,
    ];
    row.parity_state = M5ReviewPackParityState::CiOnly;
    row.downgrade_triggers = vec![
        D::UnevaluatedCheckHiddenBehindGreenSummary,
        D::EvaluatorResultClassUnstated,
        D::PackVersionDigestUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    // 4. LocalCiParityStrip.
    let mut row = base_row(
        O::LocalCiParityStrip,
        Q::Stable,
        "Local-CI-parity-strip owner",
        "Review-governance backup owner",
        "One local-CI parity strip labels a local parity estimate as an estimate, names the provider-authoritative state, shows ci-only and provider-unavailable state, flags stale-relative-to-base/head, and never presents a local parity estimate as provider-authoritative mergeability",
        "evidence:m5-local-ci-parity-strip-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "local ci parity strip",
            "each check tagged provider-authoritative, local-parity-estimate, ci-only, or not-evaluated-here",
            "the analyzed base and head the local estimate was computed on",
            "local-parity-estimate; a provider-authoritative confirmation is fetched separately",
            "local parity estimate, explicitly not a provider-authoritative merge decision",
            "stale relative to base/head once the diff moves; a re-evaluation is offered",
            "advisory of the review-pack parity summary template",
        ]),
    );
    row.local_ci_parity_roles = M5ReviewPackLocalCiParityRole::ALL.to_vec();
    row.semantic_roles = vec![R::LocalVersusProviderParityDisclosure];
    row.required_labels = labels_with(&[L::EvaluatorResultClass]);
    row.consumer_surfaces = vec![
        C::LocalCiParityStrip,
        C::MergeReadiness,
        C::ReviewDetail,
        C::ProviderHandoff,
        C::SupportExportPacket,
    ];
    row.parity_state = M5ReviewPackParityState::LocalParityEstimate;
    row.downgrade_triggers = vec![
        D::LocalEstimateShownAsProviderAuthoritative,
        D::ParityStateUnstated,
        D::UnevaluatedCheckHiddenBehindGreenSummary,
        D::EvaluatorResultClassUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    // 5. AiPolicyHook.
    let mut row = base_row(
        O::AiPolicyHook,
        Q::Stable,
        "AI-policy-hook owner",
        "AI-review-governance backup owner",
        "One AI review policy hook shows the pack version and digest the AI run used, names the pack-driven policy, discloses any divergent pack version, binds the AI result to the pack's evaluator result class, and never lets AI review run under a different pack version without disclosure",
        "evidence:m5-ai-policy-hook-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "ai policy hook",
            "AI review run bound to review pack version 4 digest sha256:pack-4-abc",
            "the pack version, digest, and pack-driven policy the AI review ran under",
            "not evaluated here: the AI finding is advisory until the pack's authoritative checks confirm it",
            "AI advisory result bound to the pack, not a provider-authoritative merge decision",
            "pack fresh: the AI review ran under the current disclosed pack version",
            "advisory of the review-pack AI review summary template",
        ]),
    );
    row.ai_policy_hook_roles = M5ReviewPackAiPolicyHookRole::ALL.to_vec();
    row.semantic_roles = vec![R::EvaluatorResultClassDisclosure];
    row.required_labels = labels_with(&[L::EvaluatorResultClass]);
    row.consumer_surfaces = vec![
        C::AiReviewPanel,
        C::ReviewDetail,
        C::ProviderHandoff,
        C::SupportExportPacket,
    ];
    row.parity_state = M5ReviewPackParityState::NotEvaluatedHere;
    row.downgrade_triggers = vec![
        D::AiReviewRanUnderUndisclosedPackVersion,
        D::EvaluatorResultClassUnstated,
        D::PackVersionDigestUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    // 6. ReviewTemplatePacket.
    let mut row = base_row(
        O::ReviewTemplatePacket,
        Q::Stable,
        "Review-template-packet owner",
        "Support-governance backup owner",
        "One review-template packet shows the comment / summary template attribution, names the template, shows the template version and pack binding, and preserves the attribution on export, publish, and reopen so review-pack version / digest and template attribution are never lost",
        "evidence:m5-review-template-packet-closure:001",
        &[
            M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
            M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "review template packet",
            "comment / summary template version 2 bound to review pack version 4",
            "the template, its version, and the review pack version and digest it is bound to",
            "draft-only review state until the rendered comment or summary is published",
            "draft-only, not a provider-authoritative merge decision",
            "pack fresh: template bound to the current disclosed pack version",
            "attributed to the review-pack comment / summary template version 2",
        ]),
    );
    row.template_packet_roles = M5ReviewPackTemplatePacketRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::TemplateAttributionDisclosure,
        R::PackVersionAndDigestDisclosure,
    ];
    row.required_labels = labels_with(&[L::TemplateAttribution]);
    row.consumer_surfaces = vec![
        C::ReviewDetail,
        C::ReviewPackSummary,
        C::SupportExportPacket,
        C::HelpDocs,
    ];
    row.parity_state = M5ReviewPackParityState::DraftOnlyReviewState;
    row.downgrade_triggers = vec![
        D::TemplateAttributionDropped,
        D::PackVersionOrDigestDropped,
        D::PackVersionDigestUnstated,
        D::ReviewPackMatrixStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ReviewPackGovernanceReview {
    M5ReviewPackGovernanceReview {
        no_local_parity_estimate_is_shown_as_provider_authoritative: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        provider_authoritative_state_is_mechanically_distinct_from_local_parity_estimate: true,
        every_review_pack_names_its_version_and_digest: true,
        every_ownership_signal_names_advisory_versus_enforced_owner: true,
        every_required_check_names_its_evaluator_result_class: true,
        no_ci_only_or_not_evaluated_here_check_is_hidden_behind_a_green_summary: true,
        every_ai_review_run_discloses_its_pack_version_and_digest: true,
        no_review_pack_version_digest_or_template_attribution_is_lost_on_export_publish_or_reopen:
            true,
        every_object_declares_classification_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_review_pack_source: true,
        review_merge_ai_provider_and_support_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_review_pack_vocabulary: true,
        review_pack_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5ReviewPackConsumerProjection {
    M5ReviewPackConsumerProjection {
        review_detail_and_merge_readiness_consume_shared_review_pack_result_truth: true,
        ai_review_and_provider_handoff_consume_shared_pack_version_truth: true,
        help_and_support_export_consume_shared_ownership_and_parity_truth: true,
        docs_help_and_screenshots_read_single_review_pack_source: true,
        review_packs_bind_to_shared_local_ci_parity_relation: true,
        support_export_reads_single_review_pack_source: true,
    }
}

fn proof_freshness() -> M5ReviewPackProofFreshness {
    M5ReviewPackProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReviewPackReleasePosture {
    M5ReviewPackReleasePosture {
        proof_packet_ref: M5_REVIEW_PACK_ARTIFACT_REF.to_owned(),
        review_pack_audit_ref: M5_REVIEW_PACK_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
        M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
        M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
        M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 review-pack matrix packet.
pub fn seeded_m5_review_pack_matrix() -> M5ReviewPackMatrixPacket {
    M5ReviewPackMatrixPacket::new(M5ReviewPackMatrixPacketInput {
        packet_id: M5_REVIEW_PACK_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 review-pack, ownership-signal, required-evidence-check, local-CI-parity-strip, AI-policy-hook, and review-template-packet matrix"
            .to_owned(),
        review_pack_rows: review_pack_rows(),
        vocabulary_set: M5ReviewPackVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the local-CI parity strip is held at Beta because its provider-authoritative
/// reconciliation is not yet fully proven; every object class stays visible.
pub fn seeded_m5_review_pack_matrix_local_ci_parity_beta_narrowed() -> M5ReviewPackMatrixPacket {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.packet_id = "m5-review-pack-evaluator:local-ci-parity-beta:0001".to_owned();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::LocalCiParityStrip)
        .expect("local-ci-parity-strip row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI review policy hook is narrowed to Preview pending durable pack-version
/// disclosure and divergence proof; every object class stays visible.
pub fn seeded_m5_review_pack_matrix_ai_policy_hook_preview_narrowed() -> M5ReviewPackMatrixPacket {
    let mut packet = seeded_m5_review_pack_matrix();
    packet.packet_id = "m5-review-pack-evaluator:ai-policy-hook-preview:0001".to_owned();
    let row = packet
        .review_pack_rows
        .iter_mut()
        .find(|row| row.object_class == M5ReviewPackObject::AiPolicyHook)
        .expect("ai-policy-hook row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
