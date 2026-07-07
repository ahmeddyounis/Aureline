//! Canonical seed builders for the M5 release-candidate-card primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, the worked resolutions, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical release-candidate-primitive packet.
pub const M5_RELEASE_CANDIDATE_PRIMITIVE_PACKET_ID: &str =
    "m5-release-candidate-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full candidate state.
#[allow(clippy::too_many_arguments)]
fn case(
    candidate_label: &str,
    version_repr: &str,
    channel_family: M5CandidateChannelFamily,
    scope_class: M5CandidateScopeClass,
    artifact_set: &[&str],
    blocker_state: M5CandidateBlockerState,
    evidence_freshness: M5EvidenceFreshnessState,
    known_issue_classes: &[M5KnownIssueClass],
    rollback_target_repr: Option<&str>,
    rollback_blast_radius: M5RollbackBlastRadius,
) -> M5ReleaseCandidateResolutionCase {
    M5ReleaseCandidateResolutionCase::resolved(M5ReleaseCandidateResolutionInput {
        candidate_label: candidate_label.to_owned(),
        version_repr: version_repr.to_owned(),
        channel_family,
        scope_class,
        artifact_set: artifact_set.iter().map(|s| (*s).to_owned()).collect(),
        blocker_state,
        evidence_freshness,
        known_issue_classes: known_issue_classes.to_vec(),
        rollback_target_repr: rollback_target_repr.map(str::to_owned),
        rollback_blast_radius,
    })
}

/// A base row with the shared fields filled in and the full anatomy, channel,
/// scope, blocker, evidence, known-issue, promotability, readiness, block-reason,
/// next-action, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5ReleaseCandidateConsumerSurface,
    qualification: M5ReleaseCenterQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5ReleaseCandidateResolutionCase>,
) -> M5ReleaseCandidateRow {
    M5ReleaseCandidateRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5CandidateCardAnatomyPart::ALL.to_vec(),
        channel_families: M5CandidateChannelFamily::ALL.to_vec(),
        scope_classes: M5CandidateScopeClass::ALL.to_vec(),
        blocker_states: M5CandidateBlockerState::ALL.to_vec(),
        evidence_freshness_states: M5EvidenceFreshnessState::ALL.to_vec(),
        known_issue_classes: M5KnownIssueClass::ALL.to_vec(),
        promotability_postures: M5CandidatePromotability::ALL.to_vec(),
        rollback_path_readinesses: M5RollbackPathReadiness::ALL.to_vec(),
        block_reasons: M5PromotionBlockReason::ALL.to_vec(),
        next_actions: M5PromotionNextAction::ALL.to_vec(),
        rollback_blast_radii: M5RollbackBlastRadius::ALL.to_vec(),
        export_fields: M5CandidateCardExportField::ALL.to_vec(),
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
            M5ReleaseCenterConsumerSurface::HelpAbout,
            M5ReleaseCenterConsumerSurface::AdminConsole,
            M5ReleaseCenterConsumerSurface::EvaluationPack,
            M5ReleaseCenterConsumerSurface::SupportExport,
            M5ReleaseCenterConsumerSurface::CliInspect,
            M5ReleaseCenterConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ReleaseCenterDowngradeTrigger::CandidateScopeUnstated,
            M5ReleaseCenterDowngradeTrigger::BlockerFreshnessHidden,
            M5ReleaseCenterDowngradeTrigger::RollbackBlastRadiusUnderstated,
            M5ReleaseCenterDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RELEASE_CANDIDATE_SCHEMA_REF,
            M5_RELEASE_CANDIDATE_OBJECT_MODEL_REF,
            M5_RELEASE_CANDIDATE_ROLLBACK_CONTRACT_REF,
        ]),
        example_resolutions,
        infers_scope_from_semver_alone: false,
        shows_stale_or_missing_evidence_as_clear: false,
        emits_generic_cannot_promote_banner: false,
        overstates_rollback_reversibility: false,
    }
}

fn candidate_rows() -> Vec<M5ReleaseCandidateRow> {
    use M5CandidateBlockerState as Block;
    use M5CandidateChannelFamily as Chan;
    use M5CandidateScopeClass as Scope;
    use M5EvidenceFreshnessState as Fresh;
    use M5KnownIssueClass as Issue;
    use M5RollbackBlastRadius as Blast;

    let mut rows = Vec::new();

    // 1. Release-center card — a cleanly promotable multi-family candidate, and a
    //    full-train candidate hard-blocked by an open blocker (the promotable /
    //    blocked coverage proof).
    rows.push(base_row(
        M5ReleaseCandidateConsumerSurface::ReleaseCenterCard,
        M5ReleaseCenterQualificationClass::Stable,
        "Release-center card owner",
        "The release-center card renders the shared candidate primitive so a multi-family candidate with fresh evidence and a pinned rollback target reads as promotable, while a full-train candidate with an open hard blocker reads as blocked with a self-contained banner naming the reason and next action",
        "evidence:m5-release-candidate-center:001",
        vec![
            case(
                "aureline release 5.2.0-rc.1",
                "5.2.0-rc.1",
                Chan::StableChannel,
                Scope::MultiFamilyCandidate,
                &["artifact:core-runtime", "artifact:cli"],
                Block::NoBlockers,
                Fresh::EvidenceFresh,
                &[],
                Some("5.1.4"),
                Blast::TrainScoped,
            ),
            case(
                "aureline release 5.2.0-rc.2",
                "5.2.0-rc.2",
                Chan::StableChannel,
                Scope::FullTrainCandidate,
                &["artifact:core-runtime", "artifact:graph", "artifact:shell"],
                Block::HardBlockerOpen,
                Fresh::EvidenceFresh,
                &[Issue::FunctionalKnownIssue],
                Some("5.1.4"),
                Blast::FleetWide,
            ),
        ],
    ));

    // 2. Update-center card — a candidate blocked on stale evidence, and one blocked
    //    on missing evidence (the stale-versus-missing evidence proof).
    rows.push(base_row(
        M5ReleaseCandidateConsumerSurface::UpdateCenterCard,
        M5ReleaseCenterQualificationClass::Stable,
        "Update-center card owner",
        "The update-center card renders the shared candidate primitive so a single-family candidate whose qualification evidence has gone stale reads as blocked-stale-evidence with a refresh-evidence next action, while a candidate missing required evidence reads as blocked-missing-evidence with a provide-evidence next action",
        "evidence:m5-release-candidate-update:001",
        vec![
            case(
                "aureline update 5.1.5-rc.1",
                "5.1.5-rc.1",
                Chan::BetaChannel,
                Scope::SingleFamilyCandidate,
                &["artifact:update-agent"],
                Block::NoBlockers,
                Fresh::EvidenceStale,
                &[Issue::PerformanceKnownIssue],
                Some("5.1.4"),
                Blast::FamilyScoped,
            ),
            case(
                "aureline update 5.1.5-rc.2",
                "5.1.5-rc.2",
                Chan::BetaChannel,
                Scope::MultiFamilyCandidate,
                &["artifact:update-agent", "artifact:mirror"],
                Block::SoftBlockersOnly,
                Fresh::EvidenceMissing,
                &[Issue::SecurityKnownIssue],
                Some("5.1.4"),
                Blast::TrainScoped,
            ),
        ],
    ));

    // 3. CLI release inspect — a preview candidate whose state is unknown with no
    //    prior to roll back to, and a backport candidate narrowed pending reverify.
    rows.push(base_row(
        M5ReleaseCandidateConsumerSurface::CliReleaseInspect,
        M5ReleaseCenterQualificationClass::Stable,
        "CLI release-inspect owner",
        "The CLI release-inspect surface renders the shared candidate primitive so a preview candidate that has not yet been evaluated reads as blocked-unknown-state with a run-evaluation next action and no-prior-to-roll-back-to readiness, while a backport candidate whose blocker was resolved reads as narrowed-pending-reverify with a reverify next action",
        "evidence:m5-release-candidate-cli:001",
        vec![
            case(
                "aureline preview 5.3.0-0.nightly",
                "5.3.0-0.nightly",
                Chan::NightlyChannel,
                Scope::PreviewChannelCandidate,
                &["artifact:experimental-core"],
                Block::BlockerStateUnknown,
                Fresh::EvidenceFreshnessUnknown,
                &[],
                None,
                Blast::SingleArtifact,
            ),
            case(
                "aureline backport 5.0.9-rc.1",
                "5.0.9-rc.1",
                Chan::StableChannel,
                Scope::BackportLineCandidate,
                &["artifact:lts-core"],
                Block::BlockerResolvedPendingReverify,
                Fresh::EvidenceFresh,
                &[Issue::CosmeticKnownIssue],
                Some("5.0.8"),
                Blast::FamilyScoped,
            ),
        ],
    ));

    // 4. Admin release report — an LTS candidate narrowed because its rollback target
    //    is undefined, and a hotfix candidate promotable under a disclosed waiver.
    rows.push(base_row(
        M5ReleaseCandidateConsumerSurface::AdminReleaseReport,
        M5ReleaseCenterQualificationClass::Stable,
        "Admin release-report owner",
        "The admin release report renders the shared candidate primitive so an LTS full-train candidate with no pinned rollback target reads as narrowed-rollback-undefined with a define-rollback-target next action rather than inferring a target from the version, while a hotfix candidate held under a disclosed waiver reads as promotable-under-waiver",
        "evidence:m5-release-candidate-admin:001",
        vec![
            case(
                "aureline lts 4.9.7-rc.1",
                "4.9.7-rc.1",
                Chan::LtsMaintenanceChannel,
                Scope::FullTrainCandidate,
                &["artifact:lts-core", "artifact:lts-cli"],
                Block::NoBlockers,
                Fresh::EvidenceFresh,
                &[Issue::DataAffectingKnownIssue],
                None,
                Blast::TrainScoped,
            ),
            case(
                "aureline hotfix 5.1.4-hotfix.1",
                "5.1.4-hotfix.1",
                Chan::StableChannel,
                Scope::HotfixCandidate,
                &["artifact:core-runtime"],
                Block::BlockerWaived,
                Fresh::EvidenceFresh,
                &[],
                Some("5.1.4"),
                Blast::SingleArtifact,
            ),
        ],
    ));

    // 5. Support / evaluation export — a candidate promotable with reservations on
    //    aging evidence, and one promotable with reservations on soft blockers (both
    //    stay promotable while the reservation is disclosed).
    rows.push(base_row(
        M5ReleaseCandidateConsumerSurface::SupportEvaluationExport,
        M5ReleaseCenterQualificationClass::Stable,
        "Support / evaluation export owner",
        "The support / evaluation export renders the shared candidate primitive so a candidate whose evidence is aging reads as promotable-with-reservations rather than clean or blocked, and a preview candidate with only soft blockers reads as promotable-with-reservations — the same candidate/blocker vocabulary a support or evaluation reviewer reads elsewhere",
        "evidence:m5-release-candidate-support:001",
        vec![
            case(
                "aureline release 5.2.0-rc.3",
                "5.2.0-rc.3",
                Chan::BetaChannel,
                Scope::MultiFamilyCandidate,
                &["artifact:core-runtime", "artifact:graph"],
                Block::NoBlockers,
                Fresh::EvidenceAging,
                &[Issue::CosmeticKnownIssue],
                Some("5.1.4"),
                Blast::FamilyScoped,
            ),
            case(
                "aureline preview 5.3.0-rc.1",
                "5.3.0-rc.1",
                Chan::PreviewChannel,
                Scope::PreviewChannelCandidate,
                &["artifact:experimental-core"],
                Block::SoftBlockersOnly,
                Fresh::EvidenceFresh,
                &[],
                Some("5.2.0"),
                Blast::SingleArtifact,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ReleaseCandidateGovernanceReview {
    M5ReleaseCandidateGovernanceReview {
        one_primitive_carries_candidate_truth: true,
        identity_and_channel_always_shown: true,
        scope_and_rollback_never_inferred_from_version: true,
        stale_or_missing_evidence_never_shown_clear: true,
        known_issues_always_disclosed: true,
        blocked_state_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_candidate_truth: true,
        no_surface_invents_second_candidate_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReleaseCandidateConsumerProjection {
    M5ReleaseCandidateConsumerProjection {
        candidate_surfaces_consume_shared_primitive: true,
        promotability_resolver_reads_single_source: true,
        evidence_freshness_reads_single_source: true,
        rollback_path_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ReleaseCandidateProofFreshness {
    M5ReleaseCandidateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReleaseCandidateReleasePosture {
    M5ReleaseCandidateReleasePosture {
        release_packet_ref: M5_RELEASE_CANDIDATE_ARTIFACT_REF.to_owned(),
        candidate_audit_ref: M5_RELEASE_CANDIDATE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RELEASE_CANDIDATE_SCHEMA_REF,
        M5_RELEASE_CANDIDATE_DOC_REF,
        M5_RELEASE_CANDIDATE_COMPONENT_MATRIX_REF,
        M5_RELEASE_CANDIDATE_OBJECT_MODEL_REF,
        M5_RELEASE_CANDIDATE_ROLLBACK_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 release-candidate-primitive packet.
pub fn seeded_m5_release_candidate_primitive_packet() -> M5ReleaseCandidatePrimitivePacket {
    M5ReleaseCandidatePrimitivePacket::new(M5ReleaseCandidatePrimitivePacketInput {
        packet_id: M5_RELEASE_CANDIDATE_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 release-candidate card and promotion-blocked-banner primitive: identity, channel, scoped artifact set, blocker summary, evidence freshness, known issues, and rollback path"
                .to_owned(),
        candidate_rows: candidate_rows(),
        vocabulary_set: M5ReleaseCandidateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the update-center card is held at Beta because a slice of
/// update-center candidates do not yet render the known-issues list on every
/// profile; every consumer stays visible.
pub fn seeded_m5_release_candidate_primitive_update_center_card_beta_narrowed(
) -> M5ReleaseCandidatePrimitivePacket {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.packet_id = "m5-release-candidate-card-primitive:update-center-beta:0001".to_owned();
    let row = packet
        .candidate_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReleaseCandidateConsumerSurface::UpdateCenterCard)
        .expect("update-center card row present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI release-inspect surface is narrowed to Preview pending
/// self-contained-banner parity proof across every headless export path; every
/// consumer stays visible.
pub fn seeded_m5_release_candidate_primitive_cli_release_inspect_preview_narrowed(
) -> M5ReleaseCandidatePrimitivePacket {
    let mut packet = seeded_m5_release_candidate_primitive_packet();
    packet.packet_id =
        "m5-release-candidate-card-primitive:cli-release-inspect-preview:0001".to_owned();
    let row = packet
        .candidate_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReleaseCandidateConsumerSurface::CliReleaseInspect)
        .expect("cli release-inspect row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
