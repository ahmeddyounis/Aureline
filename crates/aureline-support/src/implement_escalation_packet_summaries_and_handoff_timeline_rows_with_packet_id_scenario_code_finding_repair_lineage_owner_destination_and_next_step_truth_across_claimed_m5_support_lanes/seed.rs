//! Canonical seed builders for the M5 escalation-packet-summary / handoff-timeline-row
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical escalation / handoff primitive packet.
pub const M5_ESCALATION_HANDOFF_PACKET_ID: &str =
    "m5-support-escalation-packet-summary-handoff-timeline-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked escalation-packet-summary resolution case from a full packet state.
#[allow(clippy::too_many_arguments)]
fn escalation_case(
    packet_id: &str,
    scenario_family: M5SupportScenarioFamily,
    finding_families: &[M5DoctorFindingFamily],
    related_evidence_ids: &[&str],
    repair_attempts: &[M5ApprovedRepairClass],
    redaction_state: M5SupportRedactionState,
    build_profile_identity: &str,
    destination: M5EscalationPacketDestination,
    case_disposition: M5SupportCaseDisposition,
    share_requested: bool,
) -> M5EscalationPacketSummaryResolutionCase {
    M5EscalationPacketSummaryResolutionCase::resolved(M5EscalationPacketSummaryResolutionInput {
        packet_id: packet_id.to_owned(),
        scenario_family,
        finding_families: finding_families.to_vec(),
        related_evidence_ids: related_evidence_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        repair_attempts: repair_attempts.to_vec(),
        redaction_state,
        build_profile_identity: build_profile_identity.to_owned(),
        destination,
        case_disposition,
        share_requested,
    })
}

/// Builds a worked handoff-timeline-row resolution case from a full event state.
fn handoff_case(
    event_identity: &str,
    stage: M5HandoffStage,
    owner_role: &str,
    current_owner_role: &str,
    related_evidence_ids: &[&str],
    next_step: M5NextHumanStep,
) -> M5HandoffTimelineRowResolutionCase {
    M5HandoffTimelineRowResolutionCase::resolved(M5HandoffTimelineRowResolutionInput {
        event_identity: event_identity.to_owned(),
        stage,
        owner_role: owner_role.to_owned(),
        current_owner_role: current_owner_role.to_owned(),
        related_evidence_ids: related_evidence_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        next_step,
    })
}

/// A base row with the shared fields filled in and the full escalation-summary and
/// handoff-row anatomy, lineage, destination, redaction, stage, next-step, repair-class,
/// disposition, posture, action, export-field, and accessibility parity every consumer
/// carries.
fn base_row(
    consumer_surface: M5EscalationHandoffConsumerSurface,
    qualification: M5SupportQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    escalation_examples: Vec<M5EscalationPacketSummaryResolutionCase>,
    handoff_examples: Vec<M5HandoffTimelineRowResolutionCase>,
) -> M5EscalationHandoffConsumerRow {
    M5EscalationHandoffConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        escalation_anatomy_parts: M5EscalationPacketSummaryAnatomyPart::ALL.to_vec(),
        handoff_anatomy_parts: M5HandoffTimelineRowAnatomyPart::ALL.to_vec(),
        scenario_families: M5SupportScenarioFamily::ALL.to_vec(),
        finding_families: M5DoctorFindingFamily::ALL.to_vec(),
        destinations: M5EscalationPacketDestination::ALL.to_vec(),
        redaction_states: M5SupportRedactionState::ALL.to_vec(),
        handoff_stages: M5HandoffStage::ALL.to_vec(),
        next_human_steps: M5NextHumanStep::ALL.to_vec(),
        approved_repair_classes: M5ApprovedRepairClass::ALL.to_vec(),
        case_dispositions: M5SupportCaseDisposition::ALL.to_vec(),
        summary_postures: M5EscalationPacketSummaryPosture::ALL.to_vec(),
        summary_actions: M5EscalationPacketSummaryAction::ALL.to_vec(),
        row_postures: M5HandoffTimelineRowPosture::ALL.to_vec(),
        row_actions: M5HandoffTimelineRowAction::ALL.to_vec(),
        summary_export_fields: M5EscalationPacketSummaryExportField::ALL.to_vec(),
        row_export_fields: M5HandoffTimelineRowExportField::ALL.to_vec(),
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5SupportConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5SupportDowngradeTrigger::DoctorFindingLineageUnstated,
            M5SupportDowngradeTrigger::PacketDestinationUnstated,
            M5SupportDowngradeTrigger::NextHumanStepUnstated,
            M5SupportDowngradeTrigger::HandoffStageCollapsed,
            M5SupportDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ESCALATION_HANDOFF_SCHEMA_REF,
            M5_ESCALATION_HANDOFF_ESCALATION_PACKET_REF,
            M5_ESCALATION_HANDOFF_HANDOFF_PACKET_REF,
            M5_ESCALATION_HANDOFF_RECOVERY_ACTION_REF,
        ]),
        escalation_examples,
        handoff_examples,
        masks_scenario_or_finding_lineage: false,
        hides_packet_destination: false,
        drops_next_human_step: false,
        collapses_case_into_blob: false,
    }
}

fn rows() -> Vec<M5EscalationHandoffConsumerRow> {
    use M5ApprovedRepairClass as Repair;
    use M5DoctorFindingFamily as Finding;
    use M5EscalationPacketDestination as Dest;
    use M5HandoffStage as Stage;
    use M5NextHumanStep as Next;
    use M5SupportCaseDisposition as Disposition;
    use M5SupportRedactionState as Redaction;
    use M5SupportScenarioFamily as Scenario;

    vec![
        // 1. Support-center escalation desk — a crash-recovery packet ready to escalate to a
        //    vendor case once credentials are scrubbed, and a performance-health packet bound
        //    for an enterprise admin queue under a full-metadata posture that forces a
        //    redaction review. Its handoff rows open a local diagnosis and note a suggested
        //    repair.
        base_row(
            M5EscalationHandoffConsumerSurface::SupportCenterEscalationDesk,
            M5SupportQualificationClass::Stable,
            "Support center escalation desk owner",
            "The support-center escalation desk renders the shared escalation-packet summary so a crash-recovery packet with a startup-health and index-integrity finding lineage, a cache-rebuild attempt, and credentials scrubbed is ready to escalate to a vendor case, while a performance-health packet bound for the enterprise admin queue under a full-metadata posture forces a redaction review before anything leaves the device; its handoff-timeline rows keep a locally owned diagnosis-started event and a suggested-repair event legible with their owner, evidence, and next step",
            "evidence:m5-escalation-handoff-support-center:001",
            vec![
                escalation_case(
                    "packet:support-center:crash-recovery",
                    Scenario::CrashRecovery,
                    &[Finding::StartupHealth, Finding::IndexIntegrity],
                    &["finding:startup-loop", "crash:sig-eleven"],
                    &[Repair::CacheRebuild],
                    Redaction::CredentialsScrubbed,
                    "build:stable:linux:profile-a",
                    Dest::VendorSupportCase,
                    Disposition::VendorCase,
                    true,
                ),
                escalation_case(
                    "packet:support-center:performance-health",
                    Scenario::PerformanceHealth,
                    &[Finding::StoragePressure],
                    &["finding:storage-pressure"],
                    &[Repair::IndexRepair, Repair::TargetedReset],
                    Redaction::FullMetadata,
                    "build:stable:mac:profile-b",
                    Dest::EnterpriseAdmin,
                    Disposition::VendorCase,
                    true,
                ),
            ],
            vec![
                handoff_case(
                    "event:support-center:diagnosis-open",
                    Stage::DiagnosisStarted,
                    "Local diagnosing user",
                    "Local diagnosing user",
                    &["finding:startup-loop"],
                    Next::RunDoctor,
                ),
                handoff_case(
                    "event:support-center:repair-suggested",
                    Stage::RepairSuggested,
                    "Local diagnosing user",
                    "Local diagnosing user",
                    &["repair:cache-rebuild"],
                    Next::ApplyApprovedRepair,
                ),
            ],
        ),
        // 2. Recovery-center handoff — an extension-conflict packet held as a local-only
        //    bundle (share not requested), and a data-integrity packet ready for a self-serve
        //    export. Its handoff rows note a repair attempt and a fully assembled case.
        base_row(
            M5EscalationHandoffConsumerSurface::RecoveryCenterHandoff,
            M5SupportQualificationClass::Stable,
            "Recovery center handoff owner",
            "The recovery-center handoff renders the shared escalation-packet summary so an extension-conflict packet kept as a local-only bundle stays on the device with its extension-fault finding lineage and settings-repair attempt legible, while a data-integrity packet with an index-integrity finding lineage and a state-migration attempt is ready for a self-serve export; its handoff-timeline rows keep a repair-attempted event and a case-built event legible with their owner, evidence, and next step so a reviewer can reconstruct what was tried",
            "evidence:m5-escalation-handoff-recovery-center:001",
            vec![
                escalation_case(
                    "packet:recovery-center:extension-conflict",
                    Scenario::ExtensionConflict,
                    &[Finding::ExtensionFault],
                    &["finding:extension-fault"],
                    &[Repair::SettingsRepair],
                    Redaction::PathsRedacted,
                    "build:beta:linux:profile-c",
                    Dest::LocalOnlyBundle,
                    Disposition::LocalOnly,
                    false,
                ),
                escalation_case(
                    "packet:recovery-center:data-integrity",
                    Scenario::DataIntegrity,
                    &[Finding::IndexIntegrity],
                    &["finding:index-integrity"],
                    &[Repair::StateMigration],
                    Redaction::BodiesOmitted,
                    "build:stable:win:profile-d",
                    Dest::SelfServeExport,
                    Disposition::ResolvedLocally,
                    true,
                ),
            ],
            vec![
                handoff_case(
                    "event:recovery-center:repair-attempted",
                    Stage::RepairAttempted,
                    "Recovery operator",
                    "Recovery operator",
                    &["repair:index-repair"],
                    Next::GatherMoreEvidence,
                ),
                handoff_case(
                    "event:recovery-center:case-built",
                    Stage::CaseBuilt,
                    "Recovery operator",
                    "Recovery operator",
                    &["packet:case-recovery-01"],
                    Next::ExportBundle,
                ),
            ],
        ),
        // 3. Doctor handoff timeline — a connectivity-sync packet ready to escalate to a
        //    community forum with no safe repair, and an uncategorized packet whose lineage
        //    is incomplete so it cannot escalate yet. Its handoff rows record a handed-off
        //    event and an awaiting-human event.
        base_row(
            M5EscalationHandoffConsumerSurface::DoctorHandoffTimeline,
            M5SupportQualificationClass::Stable,
            "Doctor handoff timeline owner",
            "The Project Doctor handoff timeline renders the shared escalation-packet summary so a connectivity-sync packet with a sync-connectivity finding lineage and no safe repair is ready to escalate to a community forum, while an uncategorized packet with only an uncategorized finding is held lineage-incomplete until its scenario is committed; its handoff-timeline rows keep a handed-off event whose ownership moved to a vendor owner and an awaiting-human event legible with their current owner and next expected step so a human handoff consumer never restarts the case",
            "evidence:m5-escalation-handoff-doctor-handoff:001",
            vec![
                escalation_case(
                    "packet:doctor-handoff:connectivity-sync",
                    Scenario::ConnectivitySync,
                    &[Finding::SyncConnectivity],
                    &["finding:sync-connectivity"],
                    &[Repair::NoSafeRepair],
                    Redaction::PolicyRestricted,
                    "build:stable:linux:profile-e",
                    Dest::CommunityForum,
                    Disposition::VendorCase,
                    true,
                ),
                escalation_case(
                    "packet:doctor-handoff:uncategorized",
                    Scenario::UncategorizedScenario,
                    &[Finding::UncategorizedFinding],
                    &["finding:uncategorized"],
                    &[Repair::CacheRebuild],
                    Redaction::FullMetadata,
                    "build:beta:mac:profile-f",
                    Dest::SelfServeExport,
                    Disposition::Uncategorized,
                    true,
                ),
            ],
            vec![
                handoff_case(
                    "event:doctor-handoff:handed-off",
                    Stage::HandedOff,
                    "Recovery operator",
                    "Vendor support owner",
                    &["packet:case-recovery-01"],
                    Next::ContactVendor,
                ),
                handoff_case(
                    "event:doctor-handoff:awaiting-human",
                    Stage::AwaitingHuman,
                    "Vendor support owner",
                    "Vendor support owner",
                    &["packet:case-recovery-01"],
                    Next::WaitForResponse,
                ),
            ],
        ),
        // 4. Headless / CLI escalation — a crash-recovery packet whose export is blocked by
        //    redaction, and a performance-health packet bound for a blocked destination.
        //    Both remain escalation-blocked. Its handoff rows note a headless local diagnosis
        //    and a CLI handoff whose ownership moved to an admin queue.
        base_row(
            M5EscalationHandoffConsumerSurface::HeadlessCliEscalation,
            M5SupportQualificationClass::Stable,
            "Headless CLI escalation owner",
            "The headless / CLI escalation surface renders the shared escalation-packet summary so a crash-recovery packet whose export is blocked still names its escalation-blocked posture and keeps its startup-health lineage and targeted-reset attempt legible without a desktop UI, while a performance-health packet bound for a blocked destination stays escalation-blocked; its handoff-timeline rows keep a headless diagnosis-started event and a CLI handoff whose ownership moved to an admin queue owner legible with their next expected step",
            "evidence:m5-escalation-handoff-headless-cli:001",
            vec![
                escalation_case(
                    "packet:headless-cli:crash-recovery",
                    Scenario::CrashRecovery,
                    &[Finding::StartupHealth],
                    &["finding:startup"],
                    &[Repair::TargetedReset],
                    Redaction::ExportBlocked,
                    "build:stable:linux:profile-g",
                    Dest::VendorSupportCase,
                    Disposition::VendorCase,
                    true,
                ),
                escalation_case(
                    "packet:headless-cli:performance-health",
                    Scenario::PerformanceHealth,
                    &[Finding::StoragePressure, Finding::StartupHealth],
                    &["finding:perf"],
                    &[Repair::IndexRepair],
                    Redaction::CredentialsScrubbed,
                    "build:stable:cli:profile-h",
                    Dest::BlockedDestination,
                    Disposition::VendorCase,
                    true,
                ),
            ],
            vec![
                handoff_case(
                    "event:headless-cli:diagnosis-open",
                    Stage::DiagnosisStarted,
                    "Headless operator",
                    "Headless operator",
                    &["finding:cli-startup"],
                    Next::RunDoctor,
                ),
                handoff_case(
                    "event:headless-cli:handed-off",
                    Stage::HandedOff,
                    "Headless operator",
                    "Admin queue owner",
                    &["packet:cli-case-01"],
                    Next::ContactVendor,
                ),
            ],
        ),
        // 5. Support-packet export — a data-integrity packet held local-only-ready because a
        //    share was not requested even though the destination leaves the device, and an
        //    extension-conflict packet bound for a vendor case under a full-metadata posture
        //    that forces a redaction review. Its handoff rows record a fully assembled export
        //    case and an ownership transfer to the escalation desk.
        base_row(
            M5EscalationHandoffConsumerSurface::SupportPacketExport,
            M5SupportQualificationClass::Stable,
            "Support packet export owner",
            "The support-packet export surface renders the shared escalation-packet summary so a data-integrity packet whose share was not requested is held local-only-ready even though the enterprise admin destination leaves the device — with its index-integrity and sync-connectivity lineage and its state-migration and settings-repair attempts legible — while an extension-conflict packet bound for a vendor case under a full-metadata posture forces a redaction review; its handoff-timeline rows keep an export case-built event and an ownership transfer to the escalation desk legible so a support reviewer reconstructs the same lineage another surface reads",
            "evidence:m5-escalation-handoff-support-export:001",
            vec![
                escalation_case(
                    "packet:support-export:data-integrity",
                    Scenario::DataIntegrity,
                    &[Finding::IndexIntegrity, Finding::SyncConnectivity],
                    &["finding:data-integrity"],
                    &[Repair::StateMigration, Repair::SettingsRepair],
                    Redaction::PathsRedacted,
                    "build:stable:export:profile-i",
                    Dest::EnterpriseAdmin,
                    Disposition::UnsafeFixBlocked,
                    false,
                ),
                escalation_case(
                    "packet:support-export:extension-conflict",
                    Scenario::ExtensionConflict,
                    &[Finding::ExtensionFault],
                    &["finding:extension-fault-two"],
                    &[Repair::SettingsRepair],
                    Redaction::FullMetadata,
                    "build:stable:export:profile-j",
                    Dest::VendorSupportCase,
                    Disposition::VendorCase,
                    true,
                ),
            ],
            vec![
                handoff_case(
                    "event:support-export:case-built",
                    Stage::CaseBuilt,
                    "Support export owner",
                    "Support export owner",
                    &["packet:export-case-01"],
                    Next::ExportBundle,
                ),
                handoff_case(
                    "event:support-export:ownership-moved",
                    Stage::RepairAttempted,
                    "Support export owner",
                    "Escalation desk owner",
                    &["repair:targeted-reset"],
                    Next::GatherMoreEvidence,
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5EscalationHandoffGovernanceReview {
    M5EscalationHandoffGovernanceReview {
        summary_shows_packet_scenario_and_finding_lineage: true,
        summary_shows_repair_attempts_and_redaction: true,
        summary_shows_build_profile_and_destination: true,
        summary_always_offers_confirm_and_cancel: true,
        lineage_continuous_from_diagnosis_through_export: true,
        handoff_row_preserves_identity_and_owners: true,
        handoff_row_preserves_evidence_and_next_step: true,
        handoff_consumer_can_reconstruct_case: true,
        redaction_review_required_before_leaving_device: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_escalation_and_handoff_truth: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
        no_surface_masks_lineage_destination_or_next_step: true,
    }
}

fn consumer_projection() -> M5EscalationHandoffConsumerProjection {
    M5EscalationHandoffConsumerProjection {
        doctor_and_support_surfaces_consume_lineage_vocabulary: true,
        summary_posture_reads_single_source: true,
        row_posture_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5EscalationHandoffProofFreshness {
    M5EscalationHandoffProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EscalationHandoffReleasePosture {
    M5EscalationHandoffReleasePosture {
        release_packet_ref: M5_ESCALATION_HANDOFF_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_ESCALATION_HANDOFF_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ESCALATION_HANDOFF_SCHEMA_REF,
        M5_ESCALATION_HANDOFF_DOC_REF,
        M5_ESCALATION_HANDOFF_COMPONENT_MATRIX_REF,
        M5_ESCALATION_HANDOFF_ESCALATION_PACKET_REF,
        M5_ESCALATION_HANDOFF_HANDOFF_PACKET_REF,
        M5_ESCALATION_HANDOFF_RECOVERY_ACTION_REF,
        M5_ESCALATION_HANDOFF_EXPORT_REDACTION_PROFILE_REF,
        M5_ESCALATION_HANDOFF_DOCTOR_FINDING_REF,
    ])
}

/// Builds the canonical M5 escalation / handoff primitive packet.
pub fn seeded_m5_escalation_handoff_packet() -> M5EscalationHandoffPacket {
    M5EscalationHandoffPacket::new(M5EscalationHandoffPacketInput {
        packet_id: M5_ESCALATION_HANDOFF_PACKET_ID.to_owned(),
        matrix_label:
            "M5 escalation-packet-summary / handoff-timeline-row primitive: packet id, scenario code, related finding / crash lineage, repair attempts, redaction posture, build / profile identity, destination class, and confirm / cancel actions for the escalation summary; event identity, owner, related evidence, current owner, and next expected step for the handoff-timeline row"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5EscalationHandoffVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the recovery-center handoff consumer is narrowed to Preview pending
/// handoff-owner parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed(
) -> M5EscalationHandoffPacket {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.packet_id =
        "m5-support-escalation-packet-summary-handoff-timeline-row-primitive:recovery-center-handoff-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5EscalationHandoffConsumerSurface::RecoveryCenterHandoff
        })
        .expect("recovery-center-handoff row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI escalation consumer is held at Beta because a slice
/// of headless summaries do not yet render the keyboard route cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed(
) -> M5EscalationHandoffPacket {
    let mut packet = seeded_m5_escalation_handoff_packet();
    packet.packet_id =
        "m5-support-escalation-packet-summary-handoff-timeline-row-primitive:headless-cli-escalation-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5EscalationHandoffConsumerSurface::HeadlessCliEscalation
        })
        .expect("headless-cli-escalation row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}
