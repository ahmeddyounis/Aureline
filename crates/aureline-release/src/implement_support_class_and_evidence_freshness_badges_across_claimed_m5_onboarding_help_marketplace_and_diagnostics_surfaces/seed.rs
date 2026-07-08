//! Canonical seed builders for the M5 support-class / evidence-freshness badge
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical badge-claim primitive packet.
pub const M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID: &str =
    "m5-support-class-and-evidence-freshness-badge-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full badge-claim state.
fn case(
    subject_label: &str,
    support_class: M5SupportClassBadgeValue,
    freshness: M5EvidenceFreshnessValue,
    evidence_source_repr: &str,
    last_evaluated_repr: &str,
) -> M5BadgeClaimResolutionCase {
    M5BadgeClaimResolutionCase::resolved(M5BadgeClaimInput {
        subject_label: subject_label.to_owned(),
        support_class,
        freshness,
        evidence_source_repr: evidence_source_repr.to_owned(),
        last_evaluated_repr: last_evaluated_repr.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, support-class,
/// freshness, effective-claim, narrowing-reason, next-action, explanation-field,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5BadgeClaimConsumerSurface,
    qualification: M5BadgeQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5BadgeClaimResolutionCase>,
) -> M5BadgeClaimRow {
    M5BadgeClaimRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BadgeSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5BadgeClaimAnatomyPart::ALL.to_vec(),
        support_class_values: M5SupportClassBadgeValue::ALL.to_vec(),
        freshness_values: M5EvidenceFreshnessValue::ALL.to_vec(),
        effective_claim_postures: M5EffectiveClaimPosture::ALL.to_vec(),
        narrowing_reasons: M5FreshnessReducesClaimReason::ALL.to_vec(),
        next_actions: M5BadgeNextAction::ALL.to_vec(),
        explanation_fields: M5BadgeExplanationField::ALL.to_vec(),
        export_fields: M5BadgeClaimExportField::ALL.to_vec(),
        accessibility_routes: M5BadgeAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5BadgeConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5BadgeDowngradeTrigger::SupportClassValueUnstated,
            M5BadgeDowngradeTrigger::EvidenceFreshnessHidden,
            M5BadgeDowngradeTrigger::ExplanationDrawerMissing,
            M5BadgeDowngradeTrigger::FreshnessImpliedFromSupportClass,
            M5BadgeDowngradeTrigger::ExportLostBadgeMeaning,
            M5BadgeDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BADGE_CLAIM_SCHEMA_REF,
            M5_BADGE_CLAIM_FAMILY_MATRIX_REF,
            M5_BADGE_CLAIM_SUPPORT_CLASS_REF,
            M5_BADGE_CLAIM_FRESHNESS_REF,
        ]),
        example_resolutions,
        collapses_support_and_freshness_into_one_badge: false,
        implies_freshness_from_support_class: false,
        drops_support_class_context_on_narrowing: false,
        drops_badge_meaning_in_export: false,
    }
}

fn badge_rows() -> Vec<M5BadgeClaimRow> {
    use M5EvidenceFreshnessValue as Fresh;
    use M5SupportClassBadgeValue as Sup;

    vec![
    // 1. Onboarding checklist — a certified capability with fresh evidence reads as a
    //    current claim, while a supported capability whose evidence was imported and
    //    not re-verified narrows the claim while keeping the Supported context (the
    //    distinct-cues and context-preservation proof).
    base_row(
        M5BadgeClaimConsumerSurface::OnboardingChecklist,
        M5BadgeQualificationClass::Stable,
        "Onboarding badge owner",
        "The onboarding checklist renders the shared support-class and evidence-freshness badges as two distinct cues so a certified capability with fresh evidence reads as a current claim, while a supported capability whose evidence was imported and not re-verified narrows the claim with a note that preserves the Supported context and offers a reverify next action",
        "evidence:m5-support-class-badge-parity:001",
        vec![
            case(
                "aureline onboarding capability: workspace sync",
                Sup::Certified,
                Fresh::Fresh,
                "evidence-source:cert-suite:workspace-sync",
                "2026-07-01T00:00:00Z",
            ),
            case(
                "aureline onboarding capability: provider bridge",
                Sup::Supported,
                Fresh::ImportedEvidence,
                "evidence-source:imported:provider-bridge",
                "2026-05-14T00:00:00Z",
            ),
        ],
    ),

    // 2. Help capability card — a supported capability with fresh evidence, and a
    //    limited-scope capability whose retest is pending (the retest-pending proof).
    base_row(
        M5BadgeClaimConsumerSurface::HelpCapabilityCard,
        M5BadgeQualificationClass::Stable,
        "Help badge owner",
        "The Help capability card renders the shared badges so a supported capability with fresh evidence reads as a current claim, while a limited-scope capability whose retest is pending reads as retest-pending with an await-retest note rather than as stale or as an implied lower support class",
        "evidence:m5-support-class-badge-parity:002",
        vec![
            case(
                "aureline help capability: export center",
                Sup::Supported,
                Fresh::Fresh,
                "evidence-source:cert-suite:export-center",
                "2026-07-02T00:00:00Z",
            ),
            case(
                "aureline help capability: offline mirror",
                Sup::Limited,
                Fresh::RetestPending,
                "evidence-source:retest-queue:offline-mirror",
                "2026-06-20T00:00:00Z",
            ),
        ],
    ),

    // 3. Marketplace listing — a certified capability whose evidence has gone stale
    //    narrows the claim while keeping the Certified context (proving Certified does
    //    not imply Fresh), and a community capability with fresh evidence reads as a
    //    current claim.
    base_row(
        M5BadgeClaimConsumerSurface::MarketplaceListing,
        M5BadgeQualificationClass::Stable,
        "Marketplace badge owner",
        "The marketplace listing renders the shared badges so a certified capability whose evidence has gone stale narrows the claim while still showing the Certified support class as context — proving Certified never implies Fresh — and a community-supported capability with fresh evidence reads as a current claim",
        "evidence:m5-evidence-freshness-badge-parity:001",
        vec![
            case(
                "aureline marketplace listing: graph runtime",
                Sup::Certified,
                Fresh::EvidenceStale,
                "evidence-source:cert-suite:graph-runtime",
                "2026-01-05T00:00:00Z",
            ),
            case(
                "aureline marketplace listing: community theme pack",
                Sup::Community,
                Fresh::Fresh,
                "evidence-source:community-audit:theme-pack",
                "2026-07-03T00:00:00Z",
            ),
        ],
    ),

    // 4. Diagnostics report — a limited capability with stale evidence and an
    //    experimental capability with imported evidence, both narrowed with distinct
    //    reasons.
    base_row(
        M5BadgeClaimConsumerSurface::DiagnosticsReport,
        M5BadgeQualificationClass::Stable,
        "Diagnostics badge owner",
        "The diagnostics report renders the shared badges so a limited-scope capability with stale evidence reads as narrowed-evidence-stale with a refresh next action, while an experimental capability with imported evidence reads as narrowed-imported-evidence with a reverify next action — the same two-cue vocabulary a diagnostics reviewer reads elsewhere",
        "evidence:m5-evidence-freshness-badge-parity:002",
        vec![
            case(
                "aureline diagnostics subject: skew inspector",
                Sup::Limited,
                Fresh::EvidenceStale,
                "evidence-source:diagnostics:skew-inspector",
                "2026-02-11T00:00:00Z",
            ),
            case(
                "aureline diagnostics subject: preview sandbox",
                Sup::Experimental,
                Fresh::ImportedEvidence,
                "evidence-source:imported:preview-sandbox",
                "2026-04-01T00:00:00Z",
            ),
        ],
    ),

    // 5. Certification record — a certified capability whose retest is pending, and a
    //    supported capability whose evidence is stale.
    base_row(
        M5BadgeClaimConsumerSurface::CertificationRecord,
        M5BadgeQualificationClass::Stable,
        "Certification badge owner",
        "The certification record renders the shared badges so a certified capability whose retest is pending reads as retest-pending while keeping the Certified support class visible, and a supported capability whose evidence is stale narrows the claim while preserving the Supported context — support class and evidence age stay separate facts a certifier reads together",
        "evidence:m5-support-class-badge-parity:003",
        vec![
            case(
                "aureline certification subject: attestation engine",
                Sup::Certified,
                Fresh::RetestPending,
                "evidence-source:cert-suite:attestation-engine",
                "2026-06-25T00:00:00Z",
            ),
            case(
                "aureline certification subject: rollback service",
                Sup::Supported,
                Fresh::EvidenceStale,
                "evidence-source:cert-suite:rollback-service",
                "2026-01-30T00:00:00Z",
            ),
        ],
    ),

    // 6. Evaluation pack — a community capability with imported evidence narrowed, and
    //    an experimental capability with fresh evidence reading as a current claim.
    base_row(
        M5BadgeClaimConsumerSurface::EvaluationPack,
        M5BadgeQualificationClass::Stable,
        "Evaluation badge owner",
        "The evaluation pack renders the shared badges so a community-supported capability with imported evidence narrows the claim while keeping the Community context, and an experimental capability with fresh evidence reads as a current claim rather than being penalised on freshness for its lower support class — the same support-class / freshness vocabulary an evaluation reviewer reads elsewhere",
        "evidence:m5-evidence-freshness-badge-parity:003",
        vec![
            case(
                "aureline evaluation subject: partner connector",
                Sup::Community,
                Fresh::ImportedEvidence,
                "evidence-source:imported:partner-connector",
                "2026-03-18T00:00:00Z",
            ),
            case(
                "aureline evaluation subject: experimental planner",
                Sup::Experimental,
                Fresh::Fresh,
                "evidence-source:eval-run:experimental-planner",
                "2026-07-04T00:00:00Z",
            ),
        ],
    ),
    ]
}

fn governance_review() -> M5BadgeClaimGovernanceReview {
    M5BadgeClaimGovernanceReview {
        support_and_freshness_shown_as_distinct_cues: true,
        neither_badge_collapsed_into_the_other: true,
        support_class_never_implies_freshness: true,
        freshness_never_implies_support_class: true,
        stale_or_imported_evidence_auto_narrows_claim: true,
        narrowing_preserves_support_class_context: true,
        every_badge_opens_explanation_drawer: true,
        every_badge_is_separately_filterable: true,
        exported_evidence_keeps_badge_meaning: true,
        no_surface_invents_second_badge_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5BadgeClaimConsumerProjection {
    M5BadgeClaimConsumerProjection {
        onboarding_help_marketplace_surfaces_consume_shared_badges: true,
        diagnostics_certification_evaluation_surfaces_consume_shared_badges: true,
        support_class_filter_reads_single_source: true,
        freshness_filter_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5BadgeClaimProofFreshness {
    M5BadgeClaimProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BadgeClaimReleasePosture {
    M5BadgeClaimReleasePosture {
        release_packet_ref: M5_BADGE_CLAIM_ARTIFACT_REF.to_owned(),
        badge_audit_ref: M5_BADGE_CLAIM_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BADGE_CLAIM_SCHEMA_REF,
        M5_BADGE_CLAIM_DOC_REF,
        M5_BADGE_CLAIM_FAMILY_MATRIX_REF,
        M5_BADGE_CLAIM_SUPPORT_CLASS_REF,
        M5_BADGE_CLAIM_FRESHNESS_REF,
    ])
}

/// Builds the canonical M5 badge-claim primitive packet.
pub fn seeded_m5_badge_claim_primitive_packet() -> M5BadgeClaimPrimitivePacket {
    M5BadgeClaimPrimitivePacket::new(M5BadgeClaimPrimitivePacketInput {
        packet_id: M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 support-class and evidence-freshness badge primitive: certified/supported/limited/community/experimental support class and fresh/retest-pending/evidence-stale/imported-evidence freshness as two distinct, composable cues"
                .to_owned(),
        badge_rows: badge_rows(),
        vocabulary_set: M5BadgeClaimVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace listing is held at Beta because a slice of
/// marketplace badges do not yet render the freshness explanation drawer on every
/// profile; every badge consumer stays visible.
pub fn seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed(
) -> M5BadgeClaimPrimitivePacket {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.packet_id =
        "m5-support-class-and-evidence-freshness-badge-primitive:marketplace-beta:0001".to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BadgeClaimConsumerSurface::MarketplaceListing)
        .expect("marketplace listing row present");
    row.qualification = M5BadgeQualificationClass::Beta;
    packet
}

/// Narrowed variant: the certification record is narrowed to Preview pending
/// context-preservation parity proof across every export path; every badge consumer
/// stays visible.
pub fn seeded_m5_badge_claim_primitive_certification_record_preview_narrowed(
) -> M5BadgeClaimPrimitivePacket {
    let mut packet = seeded_m5_badge_claim_primitive_packet();
    packet.packet_id =
        "m5-support-class-and-evidence-freshness-badge-primitive:certification-preview:0001"
            .to_owned();
    let row = packet
        .badge_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BadgeClaimConsumerSurface::CertificationRecord)
        .expect("certification record row present");
    row.qualification = M5BadgeQualificationClass::Preview;
    packet
}
