//! Canonical seed builders for the frozen M5 launch-control matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical launch-control matrix.
pub const M5_LAUNCH_CONTROL_MATRIX_PACKET_ID: &str = "m5-launch-control:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every cohort must be able to show.
fn mandatory_labels() -> Vec<M5LaunchControlRequiredLabel> {
    M5LaunchControlRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a cohort carries.
fn labels_with(extra: &[M5LaunchControlRequiredLabel]) -> Vec<M5LaunchControlRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every cohort filled in and every cohort-specific vocabulary left empty
/// for the caller to populate.
fn base_row(
    cohort_class: M5LaunchControlCohort,
    qualification: M5LaunchControlQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5LaunchControlRow {
    M5LaunchControlRow {
        cohort_class,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5LaunchControlSurfaceFamily::ALL.to_vec(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        core_team_canary_roles: vec![],
        design_partner_preview_roles: vec![],
        extension_author_roles: vec![],
        public_preview_roles: vec![],
        certified_archetype_roles: vec![],
        degraded_reasons: M5LaunchControlDegradedReason::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5LaunchControlConsumerSurface::SupportExport,
            M5LaunchControlConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5LaunchControlDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence: false,
        lets_a_freeze_exception_become_undocumented_scope_widening: false,
        closes_a_sev_one_or_sev_two_incident_without_a_regression_asset: false,
        implies_green_when_go_no_go_records_or_orr_packets_are_stale: false,
        maintains_partner_or_public_support_language_that_outruns_current_cohort_proof: false,
    }
}

fn launch_control_rows() -> Vec<M5LaunchControlRow> {
    use M5LaunchControlCohort as F;
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;
    use M5LaunchControlQualificationClass as Q;
    use M5LaunchControlRequiredLabel as L;
    use M5LaunchControlRole as R;

    let mut rows = Vec::new();

    // 1. Core-team canary cohort.
    let mut row = base_row(
        F::CoreTeamCanary,
        Q::Stable,
        "Core-team canary owner",
        "One core-team canary cohort naming the internal dogfood ring entered, the known limits published before widening, the armed rollback-stop rule, and the reviewed dogfood telemetry so no stable claim skips the canary cohort and no ring widens on tribal memory",
        "evidence:m5-core-team-canary-cohort-parity:001",
        &[
            M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF,
        ],
    );
    row.core_team_canary_roles = M5CoreTeamCanaryRole::ALL.to_vec();
    row.semantic_roles = vec![R::CohortMembership, R::RollbackStop];
    row.required_labels = labels_with(&[L::CohortMembership]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::WidenedWithoutCurrentCohortEvidence,
        D::CohortMembershipUnstated,
        D::RollbackStopRuleUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Design-partner preview cohort.
    let mut row = base_row(
        F::DesignPartnerPreview,
        Q::Stable,
        "Design-partner preview owner",
        "One design-partner preview cohort naming the partners enrolled under NDA, the preview feedback triaged to requirements, the partner support language matched to cohort proof, and the ring widening gated on known limits so partner support language never outruns current cohort proof",
        "evidence:m5-design-partner-preview-cohort-parity:001",
        &[
            M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF,
        ],
    );
    row.design_partner_preview_roles = M5DesignPartnerPreviewRole::ALL.to_vec();
    row.semantic_roles = vec![R::CohortMembership, R::RegressionAsset];
    row.required_labels = labels_with(&[L::CohortMembership]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::PublicProof,
        C::Diagnostics,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::RanPartnerOrPublicLanguageAheadOfCohortProof,
        D::ClosedASevIncidentWithoutARegressionAsset,
        D::CohortMembershipUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Extension-author cohort.
    let mut row = base_row(
        F::ExtensionAuthor,
        Q::Stable,
        "Extension-author cohort owner",
        "One extension-author cohort naming the cohort admitted, the compatibility rehearsal kept current, the freeze exception documented not implicit, and the mixed-version drill passed so a freeze exception never becomes undocumented scope widening",
        "evidence:m5-extension-author-cohort-parity:001",
        &[
            M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF,
            M5_FREEZE_EXCEPTION_LANDED_SCHEMA_REF,
        ],
    );
    row.extension_author_roles = M5ExtensionAuthorRole::ALL.to_vec();
    row.semantic_roles = vec![R::FreezeExceptionAuthority, R::RehearsalCurrency];
    row.required_labels = labels_with(&[L::ReadinessState]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ProgramGovernance,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::LeftAFreezeExceptionUndocumented,
        D::WidenedWithoutCurrentRehearsalEvidence,
        D::ReadinessStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Public preview cohort.
    let mut row = base_row(
        F::PublicPreview,
        Q::Stable,
        "Public preview owner",
        "One public preview cohort naming the public preview ring opened, the publish/rollback drill kept current, the advisory/revocation rehearsal kept current, and the public support-handoff drill kept current so public proof never outruns cohort evidence and rehearsals stay current",
        "evidence:m5-public-preview-cohort-parity:001",
        &[
            M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF,
            M5_FREEZE_EXCEPTION_LANDED_SCHEMA_REF,
        ],
    );
    row.public_preview_roles = M5PublicPreviewRole::ALL.to_vec();
    row.semantic_roles = vec![R::ReadinessEvent, R::RehearsalCurrency];
    row.required_labels = labels_with(&[L::ReadinessState]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::PublicProof,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::RanPartnerOrPublicLanguageAheadOfCohortProof,
        D::WidenedWithoutCurrentRehearsalEvidence,
        D::ReadinessStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Certified-archetype cohort.
    let mut row = base_row(
        F::CertifiedArchetype,
        Q::Stable,
        "Certified-archetype owner",
        "One certified-archetype cohort naming the cohort validated, the operational-readiness review signed, the go/no-go decision recorded, and the evidence snapshot and on-call roster preserved so a stable claim never widens without a go/no-go decision and shiproom never implies green while go/no-go or ORR state is stale",
        "evidence:m5-certified-archetype-cohort-parity:001",
        &[
            M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
            M5_GO_NO_GO_DECISION_DOMAIN_SCHEMA_REF,
            M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF,
        ],
    );
    row.certified_archetype_roles = M5CertifiedArchetypeRole::ALL.to_vec();
    row.semantic_roles = vec![R::GoNoGoAuthority, R::ReadinessEvent];
    row.required_labels = labels_with(&[L::GoNoGoState]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::ProgramGovernance,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
        D::WidenedWithoutCurrentCohortEvidence,
        D::GoNoGoStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5LaunchControlGovernanceReview {
    M5LaunchControlGovernanceReview {
        no_stable_claim_skips_cohorts: true,
        every_committed_item_enters_with_requirement_row_evidence_class_and_rollback_path: true,
        ring_widening_depends_on_current_known_limits_and_rollback_stop_rules: true,
        sev_one_and_sev_two_incidents_generate_linked_regression_assets_before_close_out: true,
        orr_publish_rollback_mixed_version_and_advisory_revocation_drills_stay_current: true,
        support_handoff_drills_stay_current: true,
        stable_go_no_go_decisions_preserve_evidence_snapshot_and_signoff_roster: true,
        freeze_exceptions_are_documented_not_implicit_scope_widening: true,
        every_cohort_declares_widening_stages: true,
        every_cohort_declares_accessibility_route: true,
        support_export_reads_single_launch_control_source: true,
        shiproom_release_center_and_executive_steering_bind_to_single_launch_control_source: true,
        later_rows_cannot_invent_parallel_launch_control_vocabulary: true,
        launch_control_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
        partner_and_public_support_language_never_outruns_cohort_proof: true,
    }
}

fn consumer_projection() -> M5LaunchControlConsumerProjection {
    M5LaunchControlConsumerProjection {
        shiproom_and_release_center_consume_shared_launch_control_truth: true,
        executive_steering_and_program_governance_consume_shared_cohort_and_readiness_truth: true,
        diagnostics_and_cli_export_consume_shared_rehearsal_and_rollback_truth: true,
        docs_help_and_screenshots_read_single_launch_control_source: true,
        go_no_go_and_orr_proofs_bind_to_shared_evidence_snapshot: true,
        support_export_reads_single_launch_control_source: true,
    }
}

fn proof_freshness() -> M5LaunchControlProofFreshness {
    M5LaunchControlProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LaunchControlReleasePosture {
    M5LaunchControlReleasePosture {
        proof_packet_ref: M5_LAUNCH_CONTROL_ARTIFACT_REF.to_owned(),
        launch_control_audit_ref: M5_LAUNCH_CONTROL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_PACKET_DOMAIN_SCHEMA_REF,
        M5_GO_NO_GO_DECISION_DOMAIN_SCHEMA_REF,
        M5_COHORT_SCOREBOARD_LANDED_SCHEMA_REF,
        M5_FREEZE_EXCEPTION_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 launch-control matrix packet.
pub fn seeded_m5_launch_control_matrix() -> M5LaunchControlMatrixPacket {
    M5LaunchControlMatrixPacket::new(M5LaunchControlMatrixPacketInput {
        packet_id: M5_LAUNCH_CONTROL_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 dogfood-ring, certification-cohort, ORR, rehearsal, freeze-exception, and go/no-go control matrix"
                .to_owned(),
        launch_control_rows: launch_control_rows(),
        vocabulary_set: M5LaunchControlVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the public preview cohort is held at Beta because rehearsal evidence is not yet current
/// across every drill; every cohort stays visible.
pub fn seeded_m5_launch_control_matrix_public_preview_beta_narrowed() -> M5LaunchControlMatrixPacket
{
    let mut packet = seeded_m5_launch_control_matrix();
    packet.packet_id = "m5-launch-control:public-preview-beta:0001".to_owned();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::PublicPreview)
        .expect("public-preview row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the certified-archetype cohort is narrowed to Preview pending a signed go/no-go decision
/// with a preserved evidence snapshot; every cohort stays visible.
pub fn seeded_m5_launch_control_matrix_certified_archetype_preview_narrowed(
) -> M5LaunchControlMatrixPacket {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.packet_id = "m5-launch-control:certified-archetype-preview:0001".to_owned();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::CertifiedArchetype)
        .expect("certified-archetype row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
