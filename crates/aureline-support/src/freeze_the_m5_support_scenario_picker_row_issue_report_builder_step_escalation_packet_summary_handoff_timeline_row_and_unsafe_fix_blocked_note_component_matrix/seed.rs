//! Canonical seed builders for the frozen M5 support-intake / escalation component
//! matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical support-intake / escalation component matrix.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-support-intake-escalation-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5SupportRequiredLabel> {
    M5SupportRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5SupportRequiredLabel]) -> Vec<M5SupportRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5SupportIntakeEscalationComponentFamily,
    qualification: M5SupportQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5SupportIntakeEscalationComponentRow {
    M5SupportIntakeEscalationComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        scenario_families: vec![],
        incident_scopes: vec![],
        doctor_finding_families: vec![],
        builder_step_kinds: vec![],
        evidence_classes: vec![],
        packet_destinations: vec![],
        redaction_states: vec![],
        handoff_stages: vec![],
        next_human_steps: vec![],
        unsafe_fix_block_reasons: vec![],
        approved_repair_classes: vec![],
        case_dispositions: vec![],
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5SupportConsumerSurface::SupportCenterUi,
            M5SupportConsumerSurface::SupportExport,
            M5SupportConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5SupportDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_scenario_or_scope: false,
        hides_unsafe_fix_block_reason: false,
        invents_alternate_state_label: false,
        bypasses_escalation_packet_minimums: false,
    }
}

fn component_rows() -> Vec<M5SupportIntakeEscalationComponentRow> {
    use M5ApprovedRepairClass as AR;
    use M5DoctorFindingFamily as DF;
    use M5EscalationPacketDestination as PD;
    use M5HandoffStage as HS;
    use M5NextHumanStep as NS;
    use M5ReportBuilderStepKind as SK;
    use M5SupportCaseDisposition as CD;
    use M5SupportConsumerSurface as C;
    use M5SupportDowngradeTrigger as D;
    use M5SupportEvidenceClass as EV;
    use M5SupportIncidentScope as SC;
    use M5SupportIntakeEscalationComponentFamily as F;
    use M5SupportQualificationClass as Q;
    use M5SupportRedactionState as RS;
    use M5SupportRequiredLabel as L;
    use M5SupportScenarioFamily as SF;
    use M5UnsafeFixBlockReason as UnsafeBlockReason;

    let mut rows = Vec::new();

    // 1. Support-scenario picker row.
    let mut row = base_row(
        F::SupportScenarioPickerRow,
        Q::Stable,
        "Support-scenario picker row owner",
        "One support-scenario-picker-row model naming which class of problem a user is starting from — crash recovery, performance health, extension conflict, data integrity, connectivity sync, or an uncategorized scenario — how wide the incident reaches, and which Doctor finding family the scenario binds to, so a user never has to assemble a case from generic logs or guess which diagnosis path applies",
        "evidence:m5-support-scenario-picker-row-parity:001",
        &[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCENARIO_PICKER_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOCTOR_FINDING_REF,
        ],
    );
    row.scenario_families = SF::ALL.to_vec();
    row.incident_scopes = SC::ALL.to_vec();
    row.doctor_finding_families = DF::ALL.to_vec();
    row.required_labels = labels_with(&[L::ScenarioAndScope]);
    row.consumer_surfaces = vec![
        C::DoctorUi,
        C::SupportCenterUi,
        C::ReportBuilderUi,
        C::HelpCenterUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ScenarioOrScopeUnstated,
        D::DoctorFindingLineageUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Issue-report builder step.
    let mut row = base_row(
        F::IssueReportBuilderStep,
        Q::Stable,
        "Issue-report builder step owner",
        "One issue-report-builder-step model naming which step of the report the user is on — choose scenario, describe symptom, attach evidence, review redaction, confirm scope, or submit / export — and which evidence classes it selects and omits, so selected and omitted evidence is explicit and a user never ships a case without knowing what it carries",
        "evidence:m5-issue-report-builder-step-parity:001",
        &[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCENARIO_PICKER_REF,
        ],
    );
    row.builder_step_kinds = SK::ALL.to_vec();
    row.evidence_classes = EV::ALL.to_vec();
    row.required_labels = labels_with(&[L::ScenarioAndScope, L::EvidenceAndRedaction]);
    row.consumer_surfaces = vec![
        C::ReportBuilderUi,
        C::SupportCenterUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EvidenceClassMasked,
        D::RedactionStateUndisclosed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Escalation-packet summary.
    let mut row = base_row(
        F::EscalationPacketSummary,
        Q::Stable,
        "Escalation-packet summary owner",
        "One escalation-packet-summary model naming where a case is bound — a local-only bundle, a self-serve export, a vendor support case, an enterprise admin queue, a community forum, or a blocked destination — and how it redacts on export, so a local-only bundle is never mislabelled as a shared case and a redacted packet is never shown as a full export",
        "evidence:m5-escalation-packet-summary-parity:001",
        &[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ESCALATION_PACKET_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REDACTION_PROFILE_REF,
        ],
    );
    row.packet_destinations = PD::ALL.to_vec();
    row.redaction_states = RS::ALL.to_vec();
    row.case_dispositions = CD::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceAndRedaction, L::DestinationAndNextStep]);
    row.consumer_surfaces = vec![
        C::EscalationDeskUi,
        C::SupportCenterUi,
        C::RecoveryCenterUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PacketDestinationUnstated,
        D::RedactionStateUndisclosed,
        D::CaseDispositionUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Handoff-timeline row.
    let mut row = base_row(
        F::HandoffTimelineRow,
        Q::Stable,
        "Handoff-timeline row owner",
        "One handoff-timeline-row model naming where in the diagnosis-to-handoff timeline a case sits — diagnosis started, repair suggested, repair attempted, case built, handed off, or awaiting a human — and the next human step, so scenario, finding, and packet lineage is never lost between local diagnosis and human handoff and the next step is always explicit",
        "evidence:m5-handoff-timeline-row-parity:001",
        &[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ESCALATION_PACKET_REF,
        ],
    );
    row.handoff_stages = HS::ALL.to_vec();
    row.next_human_steps = NS::ALL.to_vec();
    row.required_labels = labels_with(&[L::DestinationAndNextStep]);
    row.consumer_surfaces = vec![
        C::EscalationDeskUi,
        C::SupportCenterUi,
        C::RecoveryCenterUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HandoffStageCollapsed,
        D::NextHumanStepUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Unsafe-fix blocked note.
    let mut row = base_row(
        F::UnsafeFixBlockedNote,
        Q::Stable,
        "Unsafe-fix blocked note owner",
        "One unsafe-fix-blocked-note model naming why a suggested fix is blocked — approval required, irreversible change, out-of-scope repair, insufficient evidence, policy blocked, or unsupported scenario — and which repair class is approved instead, so a user never guesses which repair is safe and an unsafe fix is never applied without saying why it is blocked",
        "evidence:m5-unsafe-fix-blocked-note-parity:001",
        &[
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
            M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_RECOVERY_ACTION_REF,
        ],
    );
    row.unsafe_fix_block_reasons = UnsafeBlockReason::ALL.to_vec();
    row.approved_repair_classes = AR::ALL.to_vec();
    row.case_dispositions = CD::ALL.to_vec();
    row.required_labels = labels_with(&[L::DestinationAndNextStep]);
    row.consumer_surfaces = vec![
        C::DoctorUi,
        C::RecoveryCenterUi,
        C::SupportCenterUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::UnsafeFixBlockReasonHidden,
        D::ApprovedRepairClassMasked,
        D::CaseDispositionUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5SupportIntakeEscalationComponentGovernanceReview {
    M5SupportIntakeEscalationComponentGovernanceReview {
        scenario_picker_row_shows_scenario_and_scope: true,
        scenario_picker_row_binds_doctor_finding_family: true,
        report_builder_step_shows_selected_and_omitted_evidence: true,
        escalation_packet_summary_shows_destination_and_redaction: true,
        handoff_timeline_row_shows_stage_and_next_step: true,
        unsafe_fix_blocked_note_shows_block_reason_and_approved_repair: true,
        no_surface_invents_alternate_state_label: true,
        local_only_vendor_case_uncategorized_and_unsafe_blocked_named_once: true,
        doctor_finding_lineage_always_explicit: true,
        approved_repair_class_always_explicit: true,
        escalation_packet_minimums_always_enforced: true,
        next_human_step_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SupportIntakeEscalationComponentConsumerProjection {
    M5SupportIntakeEscalationComponentConsumerProjection {
        doctor_and_support_surfaces_consume_scenario_vocabulary: true,
        report_builder_surfaces_consume_evidence_vocabulary: true,
        escalation_surfaces_consume_destination_and_redaction_vocabulary: true,
        unsafe_fix_surfaces_consume_block_reason_vocabulary: true,
        support_export_reads_single_source: true,
        help_and_admin_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5SupportIntakeEscalationComponentProofFreshness {
    M5SupportIntakeEscalationComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportIntakeEscalationComponentReleasePosture {
    M5SupportIntakeEscalationComponentReleasePosture {
        proof_packet_ref: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOC_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCENARIO_PICKER_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOCTOR_FINDING_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ESCALATION_PACKET_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_RECOVERY_ACTION_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REDACTION_PROFILE_REF,
    ])
}

/// Builds the canonical frozen M5 support-intake / escalation component matrix packet.
pub fn seeded_m5_support_intake_escalation_component_matrix(
) -> M5SupportIntakeEscalationComponentMatrixPacket {
    M5SupportIntakeEscalationComponentMatrixPacket::new(
        M5SupportIntakeEscalationComponentMatrixPacketInput {
            packet_id: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_PACKET_ID.to_owned(),
            matrix_label:
                "M5 support-scenario-picker-row, issue-report-builder-step, escalation-packet-summary, handoff-timeline-row, and unsafe-fix-blocked-note component matrix"
                    .to_owned(),
            component_rows: component_rows(),
            vocabulary_set: M5SupportIntakeEscalationComponentVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the escalation-packet summary is held at Beta because a slice of
/// vendor-case redaction states does not yet round-trip across every support surface;
/// every component stays visible.
pub fn seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed(
) -> M5SupportIntakeEscalationComponentMatrixPacket {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.packet_id =
        "m5-support-intake-escalation-components:escalation-packet-summary-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary
        })
        .expect("escalation-packet-summary row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}

/// Narrowed variant: the unsafe-fix blocked note is narrowed to Preview pending
/// approved-repair-class parity proof across every surface; every component stays
/// visible.
pub fn seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed(
) -> M5SupportIntakeEscalationComponentMatrixPacket {
    let mut packet = seeded_m5_support_intake_escalation_component_matrix();
    packet.packet_id =
        "m5-support-intake-escalation-components:unsafe-fix-blocked-note-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote
        })
        .expect("unsafe-fix-blocked-note row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}
