//! Canonical seed builders for the M5 support-scenario-picker-row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical picker-row primitive packet.
pub const M5_SUPPORT_SCENARIO_PICKER_ROW_PACKET_ID: &str =
    "m5-support-scenario-picker-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked support-scenario-picker-row resolution case from a full scenario state.
#[allow(clippy::too_many_arguments)]
fn picker_case(
    scenario_family: M5SupportScenarioFamily,
    incident_scope: M5SupportIncidentScope,
    doctor_finding_family: M5DoctorFindingFamily,
    symptom_cue: &str,
    scope_label: &str,
    row_identity: &str,
    scenario_diagnosis_blocked: bool,
) -> M5ScenarioPickerRowResolutionCase {
    M5ScenarioPickerRowResolutionCase::resolved(M5ScenarioPickerRowResolutionInput {
        scenario_family,
        incident_scope,
        doctor_finding_family,
        symptom_cue: symptom_cue.to_owned(),
        scope_label: scope_label.to_owned(),
        row_identity: row_identity.to_owned(),
        scenario_diagnosis_blocked,
    })
}

/// A base row with the shared fields filled in and the full picker anatomy, scenario
/// family, incident scope, Doctor finding family, posture, action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5ScenarioPickerConsumerSurface,
    qualification: M5SupportQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    picker_examples: Vec<M5ScenarioPickerRowResolutionCase>,
) -> M5ScenarioPickerConsumerRow {
    M5ScenarioPickerConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ScenarioPickerRowAnatomyPart::ALL.to_vec(),
        scenario_families: M5SupportScenarioFamily::ALL.to_vec(),
        incident_scopes: M5SupportIncidentScope::ALL.to_vec(),
        doctor_finding_families: M5DoctorFindingFamily::ALL.to_vec(),
        row_postures: M5ScenarioPickerRowPosture::ALL.to_vec(),
        row_actions: M5ScenarioPickerRowAction::ALL.to_vec(),
        export_fields: M5ScenarioPickerRowExportField::ALL.to_vec(),
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5SupportConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5SupportDowngradeTrigger::ScenarioOrScopeUnstated,
            M5SupportDowngradeTrigger::DoctorFindingLineageUnstated,
            M5SupportDowngradeTrigger::AlternateStateLabelInvented,
            M5SupportDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
            M5_SUPPORT_SCENARIO_PICKER_ROW_SCENARIO_PICKER_REF,
            M5_SUPPORT_SCENARIO_PICKER_ROW_DOCTOR_FINDING_REF,
        ]),
        picker_examples,
        masks_scenario_or_scope: false,
        hides_doctor_finding_lineage: false,
        drops_local_only_route: false,
        invents_alternate_scenario_grammar: false,
    }
}

fn rows() -> Vec<M5ScenarioPickerConsumerRow> {
    use M5DoctorFindingFamily as Finding;
    use M5SupportIncidentScope as Scope;
    use M5SupportScenarioFamily as Scenario;

    vec![
        // 1. Doctor intake — an execution-context / startup crash scoped to one file, and a
        //    performance-health scenario scoped to the workspace; both scenario-coded and
        //    ready, each keeping the same-weight local-only route.
        base_row(
            M5ScenarioPickerConsumerSurface::DoctorIntake,
            M5SupportQualificationClass::Stable,
            "Doctor intake owner",
            "The Project Doctor intake surface renders the shared support-scenario picker row so a startup crash-recovery scenario scoped to a single file names its stable scenario family, user-facing symptom, claimed scope, and bound startup-health finding with a scenario-coded start-diagnosis action, and a performance-health scenario scoped to the workspace binds its index-integrity finding — both keeping a same-weight local-only diagnosis route",
            "evidence:m5-scenario-picker-doctor:001",
            vec![
                picker_case(
                    Scenario::CrashRecovery,
                    Scope::SingleFile,
                    Finding::StartupHealth,
                    "The app closes moments after opening one project file",
                    "launch profile: default, single project file",
                    "scenario:execution-context-startup-crash",
                    false,
                ),
                picker_case(
                    Scenario::PerformanceHealth,
                    Scope::Workspace,
                    Finding::IndexIntegrity,
                    "Editing is sluggish across the whole workspace",
                    "launch profile: default, whole workspace",
                    "scenario:performance-workspace-slowdown",
                    false,
                ),
            ],
        ),
        // 2. Support-center intake — an extension/host regression scoped to the device/host
        //    and a state-corruption/low-disk recovery scenario scoped to the account; both
        //    reach beyond the workspace, so scope is confirmed before diagnosis.
        base_row(
            M5ScenarioPickerConsumerSurface::SupportCenterIntake,
            M5SupportQualificationClass::Stable,
            "Support center intake owner",
            "The support-center intake surface renders the shared support-scenario picker row so an extension/host regression scenario whose scope reaches the device/host binds its extension-fault finding and confirms scope before diagnosis, and a state-corruption / schema-drift / low-disk recovery scenario scoped to the account binds its storage-pressure finding — neither understating its incident scope",
            "evidence:m5-scenario-picker-support-center:001",
            vec![
                picker_case(
                    Scenario::ExtensionConflict,
                    Scope::DeviceHost,
                    Finding::ExtensionFault,
                    "A recently updated extension broke editing on this machine",
                    "profile: host-wide, all workspaces on this device",
                    "scenario:extension-host-regression",
                    false,
                ),
                picker_case(
                    Scenario::DataIntegrity,
                    Scope::Account,
                    Finding::StoragePressure,
                    "Files fail to save and the disk is nearly full",
                    "profile: account settings and low-disk recovery",
                    "scenario:state-corruption-low-disk",
                    false,
                ),
            ],
        ),
        // 3. Recovery-center intake — a network/CA/proxy/mirror + remote/route/collaboration
        //    mismatch scenario scoped to a remote service, and a crash-recovery scenario
        //    whose scope is not yet determined; both resolve to remote-service scope.
        base_row(
            M5ScenarioPickerConsumerSurface::RecoveryCenterIntake,
            M5SupportQualificationClass::Stable,
            "Recovery center intake owner",
            "The recovery-center intake surface renders the shared support-scenario picker row so a connectivity/sync scenario covering network/CA/proxy/mirror failure and remote/route/collaboration mismatch scoped to a remote service binds its sync-connectivity finding and confirms scope, and a crash-recovery scenario whose scope is still unknown is treated as remote-reaching until confirmed",
            "evidence:m5-scenario-picker-recovery-center:001",
            vec![
                picker_case(
                    Scenario::ConnectivitySync,
                    Scope::RemoteService,
                    Finding::SyncConnectivity,
                    "Sync fails against the mirror and shared session cannot connect",
                    "profile: remote mirror and collaboration route",
                    "scenario:network-mirror-collab-mismatch",
                    false,
                ),
                picker_case(
                    Scenario::CrashRecovery,
                    Scope::UnknownScope,
                    Finding::StartupHealth,
                    "The app restarts in a loop and the cause is not yet known",
                    "profile: not yet determined",
                    "scenario:crash-loop-undetermined-scope",
                    false,
                ),
            ],
        ),
        // 4. Headless / CLI intake — a still-uncategorized scenario that routes to
        //    evidence-gathering (unmapped), and a state-corruption scenario scoped to one
        //    file; proves the scenario-coded path works headless.
        base_row(
            M5ScenarioPickerConsumerSurface::HeadlessCliIntake,
            M5SupportQualificationClass::Stable,
            "Headless CLI intake owner",
            "The headless / CLI intake surface renders the shared support-scenario picker row so an uncategorized scenario not yet mapped to a committed Doctor finding family is named once as unmapped and starts diagnosis by gathering evidence, and a state-corruption scenario scoped to a single file resolves to a focused scenario-coded start — proving the same scenario grammar works without a desktop UI",
            "evidence:m5-scenario-picker-headless-cli:001",
            vec![
                picker_case(
                    Scenario::UncategorizedScenario,
                    Scope::Workspace,
                    Finding::UncategorizedFinding,
                    "Something is wrong but it does not match a known scenario yet",
                    "profile: default workspace, scenario not yet mapped",
                    "scenario:uncategorized-intake",
                    false,
                ),
                picker_case(
                    Scenario::DataIntegrity,
                    Scope::SingleFile,
                    Finding::StoragePressure,
                    "One file will not open after a schema change",
                    "profile: single project file",
                    "scenario:schema-drift-single-file",
                    false,
                ),
            ],
        ),
        // 5. Support-packet export — a trust/policy/identity-blocked extension scenario
        //    whose scenario-coded live diagnosis is blocked (only the local-only route
        //    remains), and a performance scenario scoped to the account.
        base_row(
            M5ScenarioPickerConsumerSurface::SupportPacketExport,
            M5SupportQualificationClass::Stable,
            "Support packet export owner",
            "The support-packet export surface renders the shared support-scenario picker row so a scenario whose scenario-coded live diagnosis is blocked by policy still reads its scenario family, symptom, scope, and finding and keeps the same-weight local-only diagnosis route without ever faking a blocked scenario-coded start, and a performance scenario scoped to the account confirms scope — the same row a support reviewer reads elsewhere",
            "evidence:m5-scenario-picker-support-export:001",
            vec![
                picker_case(
                    Scenario::ExtensionConflict,
                    Scope::Workspace,
                    Finding::ExtensionFault,
                    "A blocked extension repair needs approval before running",
                    "profile: policy-restricted workspace",
                    "scenario:trust-policy-identity-block",
                    true,
                ),
                picker_case(
                    Scenario::PerformanceHealth,
                    Scope::Account,
                    Finding::IndexIntegrity,
                    "Search is slow across every workspace on the account",
                    "profile: account-wide search index",
                    "scenario:performance-account-index",
                    false,
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5ScenarioPickerRowGovernanceReview {
    M5ScenarioPickerRowGovernanceReview {
        picker_row_shows_scenario_family_and_symptom: true,
        picker_row_shows_claimed_scope: true,
        picker_row_binds_doctor_finding_family: true,
        start_diagnosis_always_offered_unless_blocked: true,
        same_weight_local_only_route_never_dropped: true,
        scenario_rows_stable_across_deployment_lines: true,
        scenario_rows_stable_across_consumer_surfaces: true,
        uncategorized_scenario_named_once: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_scenario_truth: true,
        later_rows_cannot_invent_parallel_scenario_vocabulary: true,
        no_surface_masks_scenario_or_scope: true,
    }
}

fn consumer_projection() -> M5ScenarioPickerRowConsumerProjection {
    M5ScenarioPickerRowConsumerProjection {
        doctor_and_support_surfaces_consume_scenario_vocabulary: true,
        scenario_posture_reads_single_source: true,
        start_diagnosis_actions_read_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5ScenarioPickerRowProofFreshness {
    M5ScenarioPickerRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ScenarioPickerRowReleasePosture {
    M5ScenarioPickerRowReleasePosture {
        release_packet_ref: M5_SUPPORT_SCENARIO_PICKER_ROW_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_SUPPORT_SCENARIO_PICKER_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_DOC_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_COMPONENT_MATRIX_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCENARIO_PICKER_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_DOCTOR_FINDING_REF,
    ])
}

/// Builds the canonical M5 support-scenario-picker-row packet.
pub fn seeded_m5_support_scenario_picker_row_packet() -> M5ScenarioPickerRowPacket {
    M5ScenarioPickerRowPacket::new(M5ScenarioPickerRowPacketInput {
        packet_id: M5_SUPPORT_SCENARIO_PICKER_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 support-scenario-picker-row primitive: stable scenario family, user-facing symptom cue, claimed launch/deployment/profile scope, bound Doctor finding family, derived picker posture, and bounded reveal/start-diagnosis/start-local-only/confirm-scope/export actions with a same-weight local-only route"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5ScenarioPickerRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the recovery-center intake consumer is narrowed to Preview pending
/// scope-confirmation parity proof across every remote-reaching deployment; every consumer
/// stays visible.
pub fn seeded_m5_support_scenario_picker_row_recovery_center_intake_preview_narrowed(
) -> M5ScenarioPickerRowPacket {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.packet_id =
        "m5-support-scenario-picker-row-primitive:recovery-center-intake-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5ScenarioPickerConsumerSurface::RecoveryCenterIntake
        })
        .expect("recovery-center-intake row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI intake consumer is held at Beta because a slice of
/// headless rows do not yet render the keyboard route cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_support_scenario_picker_row_headless_cli_intake_beta_narrowed(
) -> M5ScenarioPickerRowPacket {
    let mut packet = seeded_m5_support_scenario_picker_row_packet();
    packet.packet_id =
        "m5-support-scenario-picker-row-primitive:headless-cli-intake-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ScenarioPickerConsumerSurface::HeadlessCliIntake)
        .expect("headless-cli-intake row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}
