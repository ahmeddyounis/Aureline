//! Canonical seed builders for the M5 runbook governance matrix and operator scenarios.
//!
//! These builders are the single producer of the checked-in governance matrix, the published
//! inventory, the Markdown proof, and the stale / missing-proof / waived drill fixtures, plus the
//! operator-scenario execution records. The headless emitter and the inline tests both call them so
//! the in-code packet, the artifacts, and the fixtures never drift. Every builder derives each
//! surface's verdict from the same checked-in object contracts, so the matrix is always generated
//! from the contract inventory Aureline ships: the canonical packet is all-governed; the drills
//! perturb one object contract's proof freshness (or add a waiver) and let the derivation recompute
//! the status, gate, effective claim, and named gaps.

use super::*;

/// Stable packet id for the canonical (all-governed) governance matrix packet.
pub const M5_RUNBOOK_GOVERNANCE_PACKET_ID: &str = "m5-runbook-governance:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every object's proof is
/// current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The object class the stale and missing-proof drills perturb. It is bound only by the operator
/// console and the release gate (stale drill) / the support export and the release gate
/// (missing-proof drill), so each drill gates exactly the surfaces that depend on it.
const STALE_DRILL_OBJECT: RunbookObjectClass = RunbookObjectClass::ControlPlaneHandoff;

/// The object class whose proof the missing-proof and waived drills mark absent.
const MISSING_DRILL_OBJECT: RunbookObjectClass = RunbookObjectClass::ArchivalExport;

const REDACTION_CLASS: &str = "metadata_safe_default";

/// Timestamp at which the operator-scenario deviations were recorded.
const SEED_DEVIATION_AT: &str = "2026-07-05T22:14:00Z";

/// Timestamp at which the operator-scenario executions were archived after closure.
const SEED_ARCHIVED_AT: &str = "2026-07-06T00:00:00Z";

/// Builds the canonical object contracts with every proof current.
fn canonical_object_contracts() -> Vec<RunbookObjectContract> {
    fn contract(
        object_class: RunbookObjectClass,
        label: &str,
        owner: &str,
        consumer: RunbookConsumer,
        governed_vocab: &[&str],
    ) -> RunbookObjectContract {
        RunbookObjectContract {
            object_class,
            object_label: label.to_owned(),
            owner_role: owner.to_owned(),
            first_consumer: consumer,
            schema_ref: object_class.schema_ref().to_owned(),
            proof_ref: M5_RUNBOOK_GOVERNANCE_PROOF_REF.to_owned(),
            proof_freshness: ProofFreshnessState::Current,
            governed_vocab: governed_vocab.iter().map(|t| (*t).to_owned()).collect(),
            detail_message_id: format!(
                "{}object.{}",
                M5_RUNBOOK_MESSAGE_ID_PREFIX,
                object_class.as_str()
            ),
        }
    }

    let source_classes: Vec<&str> = RunbookSourceClass::ALL.iter().map(|c| c.as_str()).collect();
    let step_classes: Vec<&str> = RunbookStepClass::ALL.iter().map(|c| c.as_str()).collect();
    let outcomes: Vec<&str> = StepOutcomeClass::ALL.iter().map(|c| c.as_str()).collect();
    let deviations: Vec<&str> = DeviationClass::ALL.iter().map(|c| c.as_str()).collect();
    let boundaries: Vec<&str> = ControlPlaneBoundaryClass::ALL
        .iter()
        .map(|c| c.as_str())
        .collect();

    vec![
        contract(
            RunbookObjectClass::SourceDescriptor,
            "Runbook source descriptor",
            "runbook_authoring_owner",
            RunbookConsumer::DocsHelp,
            &source_classes,
        ),
        contract(
            RunbookObjectClass::StepDescriptor,
            "Executable step descriptor",
            "runbook_authoring_owner",
            RunbookConsumer::OperatorDashboard,
            &step_classes,
        ),
        contract(
            RunbookObjectClass::ExecutionRecord,
            "Runbook execution record",
            "incident_operations_owner",
            RunbookConsumer::IncidentWorkspace,
            &outcomes,
        ),
        contract(
            RunbookObjectClass::DeviationNote,
            "Deviation note",
            "incident_operations_owner",
            RunbookConsumer::IncidentWorkspace,
            &deviations,
        ),
        contract(
            RunbookObjectClass::ControlPlaneHandoff,
            "Console/browser handoff packet",
            "control_plane_boundary_owner",
            RunbookConsumer::OperatorDashboard,
            &boundaries,
        ),
        contract(
            RunbookObjectClass::ArchivalExport,
            "Archival/export object",
            "support_export_owner",
            RunbookConsumer::SupportBundle,
            &["archived", "export_safe", "retention_class"],
        ),
    ]
}

/// The claimed runbook-backed surfaces and the governed objects each binds. Together the bindings
/// cover every governed object class.
const SURFACE_DEFS: [(&str, &str, RunbookConsumer, &str, &[RunbookObjectClass]); 6] = [
    (
        "incident-runbook-pane",
        "Incident workspace runbook pane",
        RunbookConsumer::IncidentWorkspace,
        "incident_operations_owner",
        &[
            RunbookObjectClass::SourceDescriptor,
            RunbookObjectClass::StepDescriptor,
            RunbookObjectClass::ExecutionRecord,
            RunbookObjectClass::DeviationNote,
        ],
    ),
    (
        "operator-runbook-console",
        "Operator dashboard runbook console",
        RunbookConsumer::OperatorDashboard,
        "operator_console_owner",
        &[
            RunbookObjectClass::SourceDescriptor,
            RunbookObjectClass::StepDescriptor,
            RunbookObjectClass::ExecutionRecord,
            RunbookObjectClass::ControlPlaneHandoff,
        ],
    ),
    (
        "docs-runbook-reference",
        "Docs/Help runbook reference",
        RunbookConsumer::DocsHelp,
        "docs_help_owner",
        &[
            RunbookObjectClass::SourceDescriptor,
            RunbookObjectClass::StepDescriptor,
        ],
    ),
    (
        "companion-runbook-assist",
        "Companion runbook assist",
        RunbookConsumer::Companion,
        "companion_owner",
        &[
            RunbookObjectClass::SourceDescriptor,
            RunbookObjectClass::StepDescriptor,
            RunbookObjectClass::DeviationNote,
        ],
    ),
    (
        "support-runbook-export",
        "Support bundle runbook export",
        RunbookConsumer::SupportBundle,
        "support_export_owner",
        &[
            RunbookObjectClass::ExecutionRecord,
            RunbookObjectClass::DeviationNote,
            RunbookObjectClass::ArchivalExport,
        ],
    ),
    (
        "release-runbook-gate",
        "Release center runbook gate",
        RunbookConsumer::ReleaseCenter,
        "release_center_owner",
        &[
            RunbookObjectClass::SourceDescriptor,
            RunbookObjectClass::StepDescriptor,
            RunbookObjectClass::ExecutionRecord,
            RunbookObjectClass::DeviationNote,
            RunbookObjectClass::ControlPlaneHandoff,
            RunbookObjectClass::ArchivalExport,
        ],
    ),
];

/// Builds a surface claim from a definition and recomputes it against the contracts.
fn build_surface(
    def: &(&str, &str, RunbookConsumer, &str, &[RunbookObjectClass]),
    contracts: &[RunbookObjectContract],
    waivers: Vec<RunbookWaiver>,
) -> RunbookSurfaceClaim {
    let (surface_id, label, consumer, owner, bound) = def;
    let mut surface = RunbookSurfaceClaim {
        surface_id: (*surface_id).to_owned(),
        surface_label: (*label).to_owned(),
        consumer: *consumer,
        owner_role: (*owner).to_owned(),
        claimed_class: RunbookClaimClass::Stable,
        bound_object_classes: bound.to_vec(),
        effective_class: RunbookClaimClass::Stable,
        status: RunbookSurfaceStatus::Mapped,
        signal: RunbookSignal::Green,
        gate_decision: RunbookGate::Governed,
        waivers,
        gaps: Vec::new(),
        status_message_id: format!("{}{}.status", M5_RUNBOOK_MESSAGE_ID_PREFIX, surface_id),
        gate_message_id: format!("{}{}.gate", M5_RUNBOOK_MESSAGE_ID_PREFIX, surface_id),
    };
    surface.recompute(contracts);
    surface
}

/// Builds the aggregate release gate from the per-surface gates.
fn build_release_gate(surfaces: &[RunbookSurfaceClaim]) -> RunbookReleaseGate {
    let sorted = |mut ids: Vec<String>| {
        ids.sort();
        ids
    };
    let pick = |f: &dyn Fn(&RunbookSurfaceClaim) -> bool| -> Vec<String> {
        sorted(
            surfaces
                .iter()
                .filter(|s| f(s))
                .map(|s| s.surface_id.clone())
                .collect(),
        )
    };
    let blocked = pick(&|s| s.is_blocked());
    RunbookReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_surface_ids: blocked,
        narrowed_surface_ids: pick(&|s| s.is_narrowed()),
        governed_surface_ids: pick(&|s| s.is_governed()),
        waived_surface_ids: pick(&|s| !s.waivers.is_empty()),
        gate_message_id: format!("{}release_gate", M5_RUNBOOK_MESSAGE_ID_PREFIX),
    }
}

fn canonical_conformance_review() -> RunbookConformanceReview {
    RunbookConformanceReview {
        every_object_class_governed: true,
        every_object_names_owner_consumer_and_proof: true,
        every_surface_binds_governed_objects: true,
        missing_object_blocks_stable_promotion: true,
        stale_or_missing_proof_gates_before_stable: true,
        waivers_disclosed_with_scope_owner_and_expiry: true,
        exact_gaps_named: true,
        runbooks_declare_authority_step_scope_and_evidence: true,
        console_pivots_and_archives_stay_attributable: true,
        companions_bounded_no_hidden_mutate_channels: true,
        generated_from_checked_in_contracts: true,
        support_export_carries_no_raw_boundary_material: true,
    }
}

fn canonical_consumer_projection() -> RunbookConsumerProjection {
    RunbookConsumerProjection {
        incident_workspace_references_inventory: true,
        operator_dashboard_references_inventory: true,
        docs_help_references_inventory: true,
        companion_follows_within_declared_scope: true,
        support_export_ships_runbook_objects: true,
        release_center_gates_on_matrix: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_RUNBOOK_GOVERNANCE_SCHEMA_REF.to_owned(),
        M5_RUNBOOK_SOURCE_SCHEMA_REF.to_owned(),
        M5_RUNBOOK_STEP_SCHEMA_REF.to_owned(),
        M5_RUNBOOK_EXECUTION_SCHEMA_REF.to_owned(),
        M5_RUNBOOK_GOVERNANCE_DOC_REF.to_owned(),
        M5_RUNBOOK_GOVERNANCE_MATRIX_REF.to_owned(),
    ]
}

/// Assembles a packet from the given contracts and per-surface waivers.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    contracts: Vec<RunbookObjectContract>,
    surface_waivers: &dyn Fn(&str) -> Vec<RunbookWaiver>,
) -> M5RunbookGovernancePacket {
    let surfaces: Vec<RunbookSurfaceClaim> = SURFACE_DEFS
        .iter()
        .map(|def| build_surface(def, &contracts, surface_waivers(def.0)))
        .collect();
    let release_gate = build_release_gate(&surfaces);
    M5RunbookGovernancePacket::new(M5RunbookGovernancePacketInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        object_contracts: contracts,
        surface_claims: surfaces,
        vocabulary_set: RunbookVocabularySet::canonical(),
        conformance_review: canonical_conformance_review(),
        consumer_projection: canonical_consumer_projection(),
        release_gate,
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// Returns no waivers for any surface.
fn no_waivers(_surface_id: &str) -> Vec<RunbookWaiver> {
    Vec::new()
}

/// Marks one contract's proof at the given freshness state.
fn with_proof_state(
    mut contracts: Vec<RunbookObjectContract>,
    object_class: RunbookObjectClass,
    state: ProofFreshnessState,
) -> Vec<RunbookObjectContract> {
    for contract in &mut contracts {
        if contract.object_class == object_class {
            contract.proof_freshness = state;
        }
    }
    contracts
}

/// The canonical, all-governed (green) runbook governance matrix packet.
pub fn seeded_m5_runbook_governance_packet() -> M5RunbookGovernancePacket {
    assemble_packet(
        M5_RUNBOOK_GOVERNANCE_PACKET_ID,
        "M5 runbook governance matrix",
        canonical_object_contracts(),
        &no_waivers,
    )
}

/// Drill: one object's proof is stale, so the surfaces that bind it auto-narrow below Stable.
pub fn seeded_m5_runbook_governance_packet_stale_proof_narrowed() -> M5RunbookGovernancePacket {
    let contracts = with_proof_state(
        canonical_object_contracts(),
        STALE_DRILL_OBJECT,
        ProofFreshnessState::Stale,
    );
    assemble_packet(
        "m5-runbook-governance:drill-stale:0001",
        "M5 runbook governance matrix — stale-proof drill",
        contracts,
        &no_waivers,
    )
}

/// Drill: one object's proof is missing, so the surfaces that bind it are blocked from Stable.
pub fn seeded_m5_runbook_governance_packet_missing_proof_blocked() -> M5RunbookGovernancePacket {
    let contracts = with_proof_state(
        canonical_object_contracts(),
        MISSING_DRILL_OBJECT,
        ProofFreshnessState::Missing,
    );
    assemble_packet(
        "m5-runbook-governance:drill-missing:0001",
        "M5 runbook governance matrix — missing-proof drill",
        contracts,
        &no_waivers,
    )
}

/// Drill: the same missing proof, but every surface that binds the object carries a disclosed,
/// time-bounded waiver, so the surfaces auto-narrow instead of blocking promotion. Their true status
/// stays red — the waiver never hides the gap.
pub fn seeded_m5_runbook_governance_packet_waived_narrowed() -> M5RunbookGovernancePacket {
    let contracts = with_proof_state(
        canonical_object_contracts(),
        MISSING_DRILL_OBJECT,
        ProofFreshnessState::Missing,
    );
    let waiver = |surface_id: &str| -> Vec<RunbookWaiver> {
        vec![RunbookWaiver {
            waiver_id: format!("waiver:{surface_id}:archival-export-proof"),
            gap_kind: RunbookGapKind::ProofMissing,
            object_class: MISSING_DRILL_OBJECT,
            reason_message_id: format!(
                "{}{}.archival_export.waiver_reason",
                M5_RUNBOOK_MESSAGE_ID_PREFIX, surface_id
            ),
            owner_role: "support_export_owner".to_owned(),
            expires_at: "2026-08-31T00:00:00Z".to_owned(),
            narrowed_to: RunbookClaimClass::Beta,
        }]
    };
    let surface_waivers = move |surface_id: &str| -> Vec<RunbookWaiver> {
        let binds_missing = SURFACE_DEFS
            .iter()
            .find(|d| d.0 == surface_id)
            .map(|d| d.4.contains(&MISSING_DRILL_OBJECT))
            .unwrap_or(false);
        if binds_missing {
            waiver(surface_id)
        } else {
            Vec::new()
        }
    };
    assemble_packet(
        "m5-runbook-governance:drill-waived:0001",
        "M5 runbook governance matrix — waived-narrowed drill",
        contracts,
        &surface_waivers,
    )
}

// --- Operator-scenario execution records ---------------------------------------------------------

fn source(
    source_id: &str,
    label: &str,
    source_class: RunbookSourceClass,
    authority_ref: &str,
    owner: &str,
    default_scope: RunbookApprovalScope,
    companion_may_request: bool,
) -> RunbookSourceDescriptor {
    RunbookSourceDescriptor {
        record_kind: M5_RUNBOOK_SOURCE_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_OBJECT_SCHEMA_VERSION,
        source_id: source_id.to_owned(),
        source_label: label.to_owned(),
        source_class,
        authority_ref: authority_ref.to_owned(),
        owner_role: owner.to_owned(),
        default_approval_scope: default_scope,
        companion_may_request,
        exportable: true,
        redaction_class: REDACTION_CLASS.to_owned(),
        detail_message_id: format!("{}source.{}", M5_RUNBOOK_MESSAGE_ID_PREFIX, source_id),
    }
}

fn step(
    step_id: &str,
    label: &str,
    step_class: RunbookStepClass,
    approval_scope: RunbookApprovalScope,
    boundary: ControlPlaneBoundaryClass,
    expected_evidence: &[&str],
    companion_permitted: bool,
) -> RunbookStepDescriptor {
    RunbookStepDescriptor {
        record_kind: M5_RUNBOOK_STEP_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_OBJECT_SCHEMA_VERSION,
        step_id: step_id.to_owned(),
        step_label: label.to_owned(),
        step_class,
        approval_scope,
        control_plane_boundary: boundary,
        mutating: step_class.is_mutating(),
        expected_evidence_outputs: expected_evidence.iter().map(|e| (*e).to_owned()).collect(),
        companion_permitted,
        detail_message_id: format!("{}step.{}", M5_RUNBOOK_MESSAGE_ID_PREFIX, step_id),
    }
}

fn clean_deviation(step_id: &str) -> DeviationNote {
    DeviationNote {
        deviation_id: format!("deviation:{step_id}:clean"),
        deviation_class: DeviationClass::NoDeviation,
        from_step_id: step_id.to_owned(),
        affected_step_ids: vec![step_id.to_owned()],
        actor_ref: String::new(),
        approver_role: String::new(),
        recorded_at: SEED_DEVIATION_AT.to_owned(),
        rationale_message_id: format!(
            "{}deviation.{}.clean",
            M5_RUNBOOK_MESSAGE_ID_PREFIX, step_id
        ),
        summary_message_id: format!(
            "{}deviation.{}.clean.summary",
            M5_RUNBOOK_MESSAGE_ID_PREFIX, step_id
        ),
        attributable: true,
    }
}

fn deviation(
    deviation_id: &str,
    class: DeviationClass,
    from_step: &str,
    affected: &[&str],
    actor: &str,
    approver: &str,
) -> DeviationNote {
    let mut affected_step_ids: Vec<String> = affected.iter().map(|s| (*s).to_owned()).collect();
    if !affected_step_ids.iter().any(|s| s == from_step) {
        affected_step_ids.insert(0, from_step.to_owned());
    }
    DeviationNote {
        deviation_id: deviation_id.to_owned(),
        deviation_class: class,
        from_step_id: from_step.to_owned(),
        affected_step_ids,
        actor_ref: actor.to_owned(),
        approver_role: approver.to_owned(),
        recorded_at: SEED_DEVIATION_AT.to_owned(),
        rationale_message_id: format!(
            "{}deviation.{}.rationale",
            M5_RUNBOOK_MESSAGE_ID_PREFIX, deviation_id
        ),
        summary_message_id: format!(
            "{}deviation.{}.summary",
            M5_RUNBOOK_MESSAGE_ID_PREFIX, deviation_id
        ),
        attributable: true,
    }
}

/// Builds an archival lineage join block from optional cross-family ids.
fn joins(
    incident: Option<&str>,
    rollout: Option<&str>,
    review: Option<&str>,
    support_bundle: Option<&str>,
) -> ArchivalLineageJoins {
    ArchivalLineageJoins {
        incident_ref: incident.map(str::to_owned),
        rollout_ref: rollout.map(str::to_owned),
        review_ref: review.map(str::to_owned),
        support_bundle_ref: support_bundle.map(str::to_owned),
    }
}

fn handoff(handoff_id: &str, boundary: ControlPlaneBoundaryClass) -> ControlPlaneHandoffPacket {
    ControlPlaneHandoffPacket {
        handoff_id: handoff_id.to_owned(),
        boundary_class: boundary,
        target_ref: format!("console-ref:{handoff_id}"),
        attribution_ref: format!("session-ref:{handoff_id}"),
        returns_to_governed_plane: true,
        creates_hidden_mutate_channel: false,
        detail_message_id: format!("{}handoff.{}", M5_RUNBOOK_MESSAGE_ID_PREFIX, handoff_id),
    }
}

fn archival(execution_id: &str, lineage_joins: ArchivalLineageJoins) -> ArchivalExportObject {
    ArchivalExportObject {
        archival_id: format!("archive:{execution_id}"),
        archived: true,
        archived_at: SEED_ARCHIVED_AT.to_owned(),
        export_safe: true,
        retention_class: "operator_history_default".to_owned(),
        support_pack_item_id: format!("support.item.runbook.execution.{execution_id}"),
        lineage_joins,
        lineage_recoverable_from_metadata_only: true,
        raw_content_exported: false,
    }
}

fn execution_record(
    execution_id: &str,
    label: &str,
    src: RunbookSourceDescriptor,
    operator: &str,
    companion_driven: bool,
    steps: Vec<ExecutedStepResult>,
    lineage_joins: ArchivalLineageJoins,
) -> RunbookExecutionRecord {
    let mut record = RunbookExecutionRecord {
        record_kind: M5_RUNBOOK_EXECUTION_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_OBJECT_SCHEMA_VERSION,
        execution_id: execution_id.to_owned(),
        execution_label: label.to_owned(),
        source: src,
        operator_role: operator.to_owned(),
        companion_driven,
        executed_steps: steps,
        deviation_lineage: Vec::new(),
        archival_export: archival(execution_id, lineage_joins),
        attributable: true,
        no_hidden_mutate_channel: true,
        redaction_class: REDACTION_CLASS.to_owned(),
        detail_message_id: format!("{}execution.{}", M5_RUNBOOK_MESSAGE_ID_PREFIX, execution_id),
    };
    record.recompute();
    record
}

/// The four checked-in operator scenarios demonstrating the object model end to end.
pub fn seeded_operator_scenario_records() -> Vec<RunbookExecutionRecord> {
    vec![
        restart_pipeline_governed(),
        failover_deviation_lineage(),
        vendor_console_handoff(),
        companion_within_scope(),
    ]
}

/// A clean, governed execution: inspect, diagnose, then a human-approved mitigation, all in-plane.
pub fn restart_pipeline_governed() -> RunbookExecutionRecord {
    let actor = "incident_operations_owner";
    let target = "target:pipeline/worker-3";
    let steps = vec![
        ExecutedStepResult::new(
            step(
                "restart.inspect",
                "Inspect pipeline state",
                RunbookStepClass::Inspect,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["pipeline_state_snapshot"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("restart.inspect"),
            None,
            vec!["evidence:restart:state".to_owned()],
            actor,
            target,
        ),
        ExecutedStepResult::new(
            step(
                "restart.diagnose",
                "Diagnose stalled worker",
                RunbookStepClass::Diagnose,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["diagnosis_summary"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("restart.diagnose"),
            None,
            vec!["evidence:restart:diagnosis".to_owned()],
            actor,
            target,
        ),
        ExecutedStepResult::new(
            step(
                "restart.mitigate",
                "Restart stalled worker",
                RunbookStepClass::Mitigate,
                RunbookApprovalScope::RequiresHumanApproval,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["mitigation_receipt"],
                false,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("restart.mitigate"),
            None,
            vec!["evidence:restart:mitigation".to_owned()],
            actor,
            target,
        ),
    ];
    execution_record(
        "restart-pipeline-governed",
        "Restart stalled pipeline worker (governed)",
        source(
            "src:restart-pipeline",
            "First-party pipeline restart runbook",
            RunbookSourceClass::VendoredFirstParty,
            "docs/runbooks/pipeline-restart",
            "runbook_authoring_owner",
            RunbookApprovalScope::ScopedSelfApprove,
            false,
        ),
        "incident_operations_owner",
        false,
        steps,
        joins(
            Some("incident:pipeline-stall:0007"),
            None,
            None,
            Some("support-bundle:restart-pipeline-governed"),
        ),
    )
}

/// A failover execution with deviation lineage: a declared step is skipped and an ad-hoc rollback is
/// added under privileged approval.
pub fn failover_deviation_lineage() -> RunbookExecutionRecord {
    let target = "target:db/primary";
    let steps = vec![
        ExecutedStepResult::new(
            step(
                "failover.inspect",
                "Inspect primary health",
                RunbookStepClass::Inspect,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["primary_health_snapshot"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("failover.inspect"),
            None,
            vec!["evidence:failover:health".to_owned()],
            "incident_operations_owner",
            target,
        ),
        ExecutedStepResult::new(
            step(
                "failover.drain",
                "Drain primary connections",
                RunbookStepClass::Mitigate,
                RunbookApprovalScope::RequiresHumanApproval,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["drain_receipt"],
                false,
            ),
            StepOutcomeClass::Skipped,
            deviation(
                "deviation:failover:drain-skipped",
                DeviationClass::StepSkipped,
                "failover.drain",
                &["failover.drain"],
                "incident_operations_owner",
                "incident_operations_owner",
            ),
            None,
            Vec::new(),
            "incident_operations_owner",
            target,
        ),
        ExecutedStepResult::new(
            step(
                "failover.rollback",
                "Ad-hoc rollback to last good snapshot",
                RunbookStepClass::Rollback,
                RunbookApprovalScope::RequiresPrivilegedApproval,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["rollback_receipt"],
                false,
            ),
            StepOutcomeClass::Completed,
            deviation(
                "deviation:failover:rollback-adhoc",
                DeviationClass::StepAddedAdHoc,
                "failover.rollback",
                &["failover.rollback", "failover.drain"],
                "privileged_operations_owner",
                "privileged_operations_owner",
            ),
            None,
            vec!["evidence:failover:rollback".to_owned()],
            "privileged_operations_owner",
            target,
        ),
    ];
    execution_record(
        "failover-deviation-lineage",
        "Failover with skipped step and ad-hoc rollback (deviation lineage)",
        source(
            "src:failover",
            "Organization failover runbook",
            RunbookSourceClass::OrganizationAuthored,
            "org-runbook-library:failover:v4",
            "incident_operations_owner",
            RunbookApprovalScope::ScopedSelfApprove,
            false,
        ),
        "incident_operations_owner",
        false,
        steps,
        joins(
            Some("incident:db-failover:0011"),
            Some("rollout:db-primary-failover:0003"),
            Some("review:postmortem:db-failover:0011"),
            Some("support-bundle:failover-deviation-lineage"),
        ),
    )
}

/// An execution that pivots to an external vendor console under an attributable handoff that returns
/// to the governed plane.
pub fn vendor_console_handoff() -> RunbookExecutionRecord {
    let actor = "operator_console_owner";
    let steps = vec![
        ExecutedStepResult::new(
            step(
                "vendor.inspect",
                "Inspect vendor-side status",
                RunbookStepClass::Inspect,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["vendor_status_summary"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("vendor.inspect"),
            None,
            vec!["evidence:vendor:status".to_owned()],
            actor,
            "target:vendor/status",
        ),
        ExecutedStepResult::new(
            step(
                "vendor.console",
                "Hand off to vendor console for scaling action",
                RunbookStepClass::ConsoleHandoff,
                RunbookApprovalScope::RequiresHumanApproval,
                ControlPlaneBoundaryClass::VendorConsoleHandoff,
                &["handoff_record"],
                false,
            ),
            StepOutcomeClass::HandedOff,
            clean_deviation("vendor.console"),
            Some(handoff(
                "vendor-scale",
                ControlPlaneBoundaryClass::VendorConsoleHandoff,
            )),
            vec!["evidence:vendor:handoff".to_owned()],
            actor,
            "target:vendor-console/scaling-group",
        ),
    ];
    execution_record(
        "vendor-console-handoff",
        "Vendor console handoff (attributable, returns to governed plane)",
        source(
            "src:vendor-console",
            "Imported vendor console reference",
            RunbookSourceClass::ImportedVendorConsole,
            "vendor-console:ref:scaling",
            "control_plane_boundary_owner",
            RunbookApprovalScope::RequiresHumanApproval,
            true,
        ),
        "operator_console_owner",
        false,
        steps,
        joins(
            Some("incident:vendor-scale:0014"),
            None,
            None,
            Some("support-bundle:vendor-console-handoff"),
        ),
    )
}

/// A companion-driven execution that stays within declared read-only/annotate scope and never mints
/// a privileged mutate channel.
pub fn companion_within_scope() -> RunbookExecutionRecord {
    let actor = "companion_assist_session";
    let steps = vec![
        ExecutedStepResult::new(
            step(
                "companion.inspect",
                "Companion inspects recent errors",
                RunbookStepClass::Inspect,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["error_window_summary"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("companion.inspect"),
            None,
            vec!["evidence:companion:errors".to_owned()],
            actor,
            "target:pipeline/error-window",
        ),
        ExecutedStepResult::new(
            step(
                "companion.diagnose",
                "Companion proposes a diagnosis",
                RunbookStepClass::Diagnose,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["proposed_diagnosis"],
                true,
            ),
            StepOutcomeClass::Completed,
            clean_deviation("companion.diagnose"),
            None,
            vec!["evidence:companion:diagnosis".to_owned()],
            actor,
            "target:pipeline/error-window",
        ),
        ExecutedStepResult::new(
            step(
                "companion.request",
                "Companion records a request for human-approved mitigation",
                RunbookStepClass::Annotate,
                RunbookApprovalScope::NoApprovalReadOnly,
                ControlPlaneBoundaryClass::InAppGoverned,
                &["mitigation_request_note"],
                true,
            ),
            StepOutcomeClass::AwaitingApproval,
            clean_deviation("companion.request"),
            None,
            vec!["evidence:companion:request".to_owned()],
            actor,
            "",
        ),
    ];
    execution_record(
        "companion-within-scope",
        "Companion follows and requests within declared scope (no hidden mutate)",
        source(
            "src:companion-draft",
            "Companion-drafted assist runbook",
            RunbookSourceClass::CompanionDrafted,
            "companion-draft:assist:001",
            "companion_owner",
            RunbookApprovalScope::RequiresHumanApproval,
            true,
        ),
        "companion_owner",
        true,
        steps,
        joins(
            None,
            None,
            Some("review:companion-assist:0021"),
            Some("support-bundle:companion-within-scope"),
        ),
    )
}
