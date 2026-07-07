//! Canonical seed builders for the M5 unsafe-fix-blocked-note / approved-repair-guidance
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical unsafe-fix / approved-repair primitive packet.
pub const M5_UNSAFE_REPAIR_PACKET_ID: &str =
    "m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked unsafe-fix-blocked-note resolution case from a full note state.
#[allow(clippy::too_many_arguments)]
fn note_case(
    note_id: &str,
    blocked_action_label: &str,
    scenario_family: M5SupportScenarioFamily,
    finding_families: &[M5DoctorFindingFamily],
    related_evidence_ids: &[&str],
    block_reason: M5UnsafeFixBlockReason,
    recommended_repair: M5ApprovedRepairClass,
    redaction_state: M5SupportRedactionState,
    build_profile_identity: &str,
    case_disposition: M5SupportCaseDisposition,
) -> M5BlockedNoteResolutionCase {
    M5BlockedNoteResolutionCase::resolved(M5BlockedNoteResolutionInput {
        note_id: note_id.to_owned(),
        blocked_action_label: blocked_action_label.to_owned(),
        scenario_family,
        finding_families: finding_families.to_vec(),
        related_evidence_ids: related_evidence_ids
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        block_reason,
        recommended_repair,
        redaction_state,
        build_profile_identity: build_profile_identity.to_owned(),
        case_disposition,
    })
}

/// Builds a worked approved-repair-guidance resolution case from a full repair state.
fn guidance_case(
    guidance_id: &str,
    repair_class: M5ApprovedRepairClass,
    blast_radius: M5RepairBlastRadius,
    changed_classes: &[M5RepairChangeClass],
    unchanged_classes: &[M5RepairChangeClass],
    reversibility: M5RepairReversibility,
) -> M5ApprovedRepairGuidanceResolutionCase {
    M5ApprovedRepairGuidanceResolutionCase::resolved(M5ApprovedRepairGuidanceResolutionInput {
        guidance_id: guidance_id.to_owned(),
        repair_class,
        blast_radius,
        changed_classes: changed_classes.to_vec(),
        unchanged_classes: unchanged_classes.to_vec(),
        reversibility,
    })
}

/// A base row with the shared fields filled in and the full blocked-note and repair-guidance
/// anatomy, lineage, reason, repair, redaction, disposition, blast, change-class,
/// reversibility, posture, action, export-field, and accessibility parity every consumer
/// carries.
fn base_row(
    consumer_surface: M5UnsafeRepairConsumerSurface,
    qualification: M5SupportQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    note_examples: Vec<M5BlockedNoteResolutionCase>,
    guidance_examples: Vec<M5ApprovedRepairGuidanceResolutionCase>,
) -> M5UnsafeRepairConsumerRow {
    M5UnsafeRepairConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        note_anatomy_parts: M5BlockedNoteAnatomyPart::ALL.to_vec(),
        guidance_anatomy_parts: M5ApprovedRepairGuidanceAnatomyPart::ALL.to_vec(),
        scenario_families: M5SupportScenarioFamily::ALL.to_vec(),
        finding_families: M5DoctorFindingFamily::ALL.to_vec(),
        block_reasons: M5UnsafeFixBlockReason::ALL.to_vec(),
        approved_repair_classes: M5ApprovedRepairClass::ALL.to_vec(),
        redaction_states: M5SupportRedactionState::ALL.to_vec(),
        case_dispositions: M5SupportCaseDisposition::ALL.to_vec(),
        blast_radii: M5RepairBlastRadius::ALL.to_vec(),
        change_classes: M5RepairChangeClass::ALL.to_vec(),
        reversibilities: M5RepairReversibility::ALL.to_vec(),
        note_postures: M5BlockedNotePosture::ALL.to_vec(),
        note_actions: M5BlockedNoteAction::ALL.to_vec(),
        guidance_postures: M5ApprovedRepairGuidancePosture::ALL.to_vec(),
        guidance_actions: M5ApprovedRepairGuidanceAction::ALL.to_vec(),
        note_export_fields: M5BlockedNoteExportField::ALL.to_vec(),
        guidance_export_fields: M5ApprovedRepairGuidanceExportField::ALL.to_vec(),
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5SupportConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5SupportDowngradeTrigger::UnsafeFixBlockReasonHidden,
            M5SupportDowngradeTrigger::ApprovedRepairClassMasked,
            M5SupportDowngradeTrigger::RedactionStateUndisclosed,
            M5SupportDowngradeTrigger::CaseDispositionUnstated,
            M5SupportDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_UNSAFE_REPAIR_SCHEMA_REF,
            M5_UNSAFE_REPAIR_REPAIR_TRANSACTION_REF,
            M5_UNSAFE_REPAIR_RECOVERY_ACTION_REF,
            M5_UNSAFE_REPAIR_RECOVERY_LADDER_REF,
        ]),
        note_examples,
        guidance_examples,
        masks_block_reason_or_repair: false,
        presents_reset_as_reviewed_transaction: false,
        drops_rollback_or_evidence_posture: false,
        collapses_guidance_into_blob: false,
    }
}

fn rows() -> Vec<M5UnsafeRepairConsumerRow> {
    use M5ApprovedRepairClass as Repair;
    use M5DoctorFindingFamily as Finding;
    use M5RepairBlastRadius as Blast;
    use M5RepairChangeClass as Change;
    use M5RepairReversibility as Reverse;
    use M5SupportCaseDisposition as Disposition;
    use M5SupportRedactionState as Redaction;
    use M5SupportScenarioFamily as Scenario;
    use M5UnsafeFixBlockReason as UnsafeReason;

    vec![
        // 1. Doctor suggested-repair review — an irreversible workspace-delete blocked in
        //    favour of a reviewed cache rebuild, and an approval-required index reset blocked
        //    in favour of an index repair. Its repair guidance shows a scoped reviewed cache
        //    rebuild and a device-wide irreversible reset.
        base_row(
            M5UnsafeRepairConsumerSurface::DoctorRepairReview,
            M5SupportQualificationClass::Stable,
            "Doctor repair review owner",
            "The Project Doctor suggested-repair review renders the shared unsafe-fix blocked note so an irreversible \"delete the workspace and reset to a clean checkout\" action is blocked for a crash-recovery scenario with a startup-health finding lineage — recommending a reviewed cache rebuild instead and preserving the rollback and evidence posture — while an approval-required index reset is blocked in favour of an index repair; its approved-repair guidance keeps a scoped, reviewed, reversible cache rebuild distinct from a device-wide irreversible reset",
            "evidence:m5-unsafe-repair-doctor-review:001",
            vec![
                note_case(
                    "note:doctor-review:workspace-delete",
                    "Delete the workspace and reset to a clean checkout",
                    Scenario::CrashRecovery,
                    &[Finding::StartupHealth],
                    &["finding:startup-loop", "crash:sig-eleven"],
                    UnsafeReason::IrreversibleChange,
                    Repair::CacheRebuild,
                    Redaction::CredentialsScrubbed,
                    "build:stable:linux:profile-a",
                    Disposition::LocalOnly,
                ),
                note_case(
                    "note:doctor-review:index-reset",
                    "Force-reset the search index without a checkpoint",
                    Scenario::PerformanceHealth,
                    &[Finding::StoragePressure],
                    &["finding:storage-pressure"],
                    UnsafeReason::ApprovalRequired,
                    Repair::IndexRepair,
                    Redaction::FullMetadata,
                    "build:stable:mac:profile-b",
                    Disposition::VendorCase,
                ),
            ],
            vec![
                guidance_case(
                    "guidance:doctor-review:cache-rebuild",
                    Repair::CacheRebuild,
                    Blast::SingleArtifact,
                    &[Change::CacheArtifacts],
                    &[Change::Settings, Change::UserContent],
                    Reverse::ReversibleTransaction,
                ),
                guidance_case(
                    "guidance:doctor-review:device-reset",
                    Repair::TargetedReset,
                    Blast::DeviceWide,
                    &[Change::WorkspaceState, Change::GeneratedFiles],
                    &[Change::UserContent],
                    Reverse::Irreversible,
                ),
            ],
        ),
        // 2. Support-center unsafe-fix desk — a policy-blocked extension purge in favour of a
        //    settings repair, and an insufficient-evidence data wipe in favour of a state
        //    migration. Its repair guidance shows a workspace-scoped reviewed settings repair
        //    and a partially reversible profile-scoped state migration.
        base_row(
            M5UnsafeRepairConsumerSurface::SupportCenterUnsafeFixDesk,
            M5SupportQualificationClass::Stable,
            "Support center unsafe-fix desk owner",
            "The support-center unsafe-fix desk renders the shared unsafe-fix blocked note so a policy-blocked \"purge every extension and its stored state\" action is blocked for an extension-conflict scenario — recommending a settings repair instead — while an \"erase the workspace database\" action blocked for insufficient evidence recommends a reviewed state migration and keeps the evidence a user retains if they decline; its approved-repair guidance keeps a workspace-scoped reviewed settings repair distinct from a profile-scoped partially reversible state migration",
            "evidence:m5-unsafe-repair-support-center:001",
            vec![
                note_case(
                    "note:support-center:extension-purge",
                    "Purge every extension and its stored state",
                    Scenario::ExtensionConflict,
                    &[Finding::ExtensionFault],
                    &["finding:extension-fault"],
                    UnsafeReason::PolicyBlocked,
                    Repair::SettingsRepair,
                    Redaction::PathsRedacted,
                    "build:beta:linux:profile-c",
                    Disposition::UnsafeFixBlocked,
                ),
                note_case(
                    "note:support-center:data-wipe",
                    "Erase the workspace database and rebuild from empty",
                    Scenario::DataIntegrity,
                    &[Finding::IndexIntegrity],
                    &["finding:index-integrity"],
                    UnsafeReason::InsufficientEvidence,
                    Repair::StateMigration,
                    Redaction::BodiesOmitted,
                    "build:stable:win:profile-d",
                    Disposition::ResolvedLocally,
                ),
            ],
            vec![
                guidance_case(
                    "guidance:support-center:settings-repair",
                    Repair::SettingsRepair,
                    Blast::WorkspaceScoped,
                    &[Change::Settings],
                    &[Change::CacheArtifacts, Change::UserContent],
                    Reverse::ReversibleTransaction,
                ),
                guidance_case(
                    "guidance:support-center:state-migration",
                    Repair::StateMigration,
                    Blast::ProfileScoped,
                    &[Change::WorkspaceState],
                    &[Change::UserContent, Change::GeneratedFiles],
                    Reverse::PartiallyReversible,
                ),
            ],
        ),
        // 3. Recovery-center repair guidance — an out-of-scope remote reset in favour of a
        //    targeted reset, and an unsupported-scenario auto-fix with no safe repair. Its
        //    repair guidance shows a broad reversible index repair and a no-change,
        //    no-safe-repair card that still preserves the evidence.
        base_row(
            M5UnsafeRepairConsumerSurface::RecoveryCenterRepairGuidance,
            M5SupportQualificationClass::Stable,
            "Recovery center repair guidance owner",
            "The recovery-center repair guidance renders the shared unsafe-fix blocked note so an out-of-scope \"reset the remote service credentials\" action is blocked for a connectivity-sync scenario — recommending a targeted reset instead — while an unsupported-scenario \"apply the community auto-fix macro\" action names that no safe repair is available and still preserves the evidence a user keeps; its approved-repair guidance keeps a profile-scoped broad-but-reversible index repair distinct from a no-change card that offers no safe repair",
            "evidence:m5-unsafe-repair-recovery-center:001",
            vec![
                note_case(
                    "note:recovery-center:remote-credential-reset",
                    "Reset the remote service credentials from the client",
                    Scenario::ConnectivitySync,
                    &[Finding::SyncConnectivity],
                    &["finding:sync-connectivity"],
                    UnsafeReason::OutOfScopeRepair,
                    Repair::TargetedReset,
                    Redaction::PolicyRestricted,
                    "build:stable:linux:profile-e",
                    Disposition::VendorCase,
                ),
                note_case(
                    "note:recovery-center:auto-fix-macro",
                    "Apply the community auto-fix macro",
                    Scenario::UncategorizedScenario,
                    &[Finding::UncategorizedFinding],
                    &["finding:uncategorized"],
                    UnsafeReason::UnsupportedScenario,
                    Repair::NoSafeRepair,
                    Redaction::ExportBlocked,
                    "build:beta:mac:profile-f",
                    Disposition::Uncategorized,
                ),
            ],
            vec![
                guidance_case(
                    "guidance:recovery-center:index-repair",
                    Repair::IndexRepair,
                    Blast::ProfileScoped,
                    &[Change::SearchIndex],
                    &[Change::UserContent],
                    Reverse::ReversibleTransaction,
                ),
                guidance_case(
                    "guidance:recovery-center:no-repair",
                    Repair::NoSafeRepair,
                    Blast::NoChange,
                    &[],
                    &[Change::UserContent, Change::WorkspaceState, Change::GeneratedFiles],
                    Reverse::Irreversible,
                ),
            ],
        ),
        // 4. Headless / CLI repair review — a no-safe-repair irreversible wipe, and an
        //    approval-required settings clear in favour of a settings repair. Its repair
        //    guidance shows a scoped reviewed cache rebuild and a workspace-scoped partially
        //    reversible state migration.
        base_row(
            M5UnsafeRepairConsumerSurface::HeadlessCliRepairReview,
            M5SupportQualificationClass::Stable,
            "Headless CLI repair review owner",
            "The headless / CLI repair review renders the shared unsafe-fix blocked note so an irreversible \"wipe every generated artifact and workspace checkpoint\" action names that no safe repair is available without a desktop UI, while an approval-required \"clear all settings to defaults\" action is blocked in favour of a settings repair; its approved-repair guidance keeps a single-artifact reviewed cache rebuild distinct from a workspace-scoped partially reversible state migration",
            "evidence:m5-unsafe-repair-headless-cli:001",
            vec![
                note_case(
                    "note:headless-cli:artifact-wipe",
                    "Wipe every generated artifact and workspace checkpoint",
                    Scenario::CrashRecovery,
                    &[Finding::StartupHealth, Finding::IndexIntegrity],
                    &["finding:cli-startup"],
                    UnsafeReason::IrreversibleChange,
                    Repair::NoSafeRepair,
                    Redaction::FullMetadata,
                    "build:stable:cli:profile-g",
                    Disposition::UnsafeFixBlocked,
                ),
                note_case(
                    "note:headless-cli:settings-clear",
                    "Clear all settings to defaults without a backup",
                    Scenario::PerformanceHealth,
                    &[Finding::StoragePressure],
                    &["finding:cli-perf"],
                    UnsafeReason::ApprovalRequired,
                    Repair::SettingsRepair,
                    Redaction::CredentialsScrubbed,
                    "build:stable:cli:profile-h",
                    Disposition::VendorCase,
                ),
            ],
            vec![
                guidance_case(
                    "guidance:headless-cli:cache-rebuild",
                    Repair::CacheRebuild,
                    Blast::SingleArtifact,
                    &[Change::CacheArtifacts],
                    &[Change::Settings],
                    Reverse::ReversibleTransaction,
                ),
                guidance_case(
                    "guidance:headless-cli:state-migration",
                    Repair::StateMigration,
                    Blast::WorkspaceScoped,
                    &[Change::WorkspaceState, Change::SearchIndex],
                    &[Change::UserContent],
                    Reverse::PartiallyReversible,
                ),
            ],
        ),
        // 5. Support repair export — a policy-blocked factory reset in favour of an index
        //    repair, and an insufficient-evidence migration blocked in favour of a state
        //    migration. Its repair guidance shows a workspace-scoped reviewed settings repair
        //    and a device-wide irreversible reset.
        base_row(
            M5UnsafeRepairConsumerSurface::SupportRepairExport,
            M5SupportQualificationClass::Stable,
            "Support repair export owner",
            "The support repair export renders the shared unsafe-fix blocked note so a policy-blocked \"factory-reset the device profile\" action is blocked for a data-integrity scenario with an index-integrity and sync-connectivity finding lineage — recommending an index repair — while an insufficient-evidence \"migrate every workspace in place\" action recommends a reviewed state migration and preserves the evidence another surface reads; its approved-repair guidance keeps a workspace-scoped reviewed settings repair distinct from a device-wide irreversible reset",
            "evidence:m5-unsafe-repair-support-export:001",
            vec![
                note_case(
                    "note:support-export:factory-reset",
                    "Factory-reset the device profile",
                    Scenario::DataIntegrity,
                    &[Finding::IndexIntegrity, Finding::SyncConnectivity],
                    &["finding:data-integrity"],
                    UnsafeReason::PolicyBlocked,
                    Repair::IndexRepair,
                    Redaction::PathsRedacted,
                    "build:stable:export:profile-i",
                    Disposition::LocalOnly,
                ),
                note_case(
                    "note:support-export:in-place-migration",
                    "Migrate every workspace in place without a checkpoint",
                    Scenario::ExtensionConflict,
                    &[Finding::ExtensionFault],
                    &["finding:extension-fault-two"],
                    UnsafeReason::InsufficientEvidence,
                    Repair::StateMigration,
                    Redaction::PolicyRestricted,
                    "build:stable:export:profile-j",
                    Disposition::ResolvedLocally,
                ),
            ],
            vec![
                guidance_case(
                    "guidance:support-export:settings-repair",
                    Repair::SettingsRepair,
                    Blast::WorkspaceScoped,
                    &[Change::Settings],
                    &[Change::UserContent, Change::GeneratedFiles],
                    Reverse::ReversibleTransaction,
                ),
                guidance_case(
                    "guidance:support-export:device-reset",
                    Repair::TargetedReset,
                    Blast::DeviceWide,
                    &[Change::GeneratedFiles, Change::WorkspaceState],
                    &[Change::UserContent],
                    Reverse::Irreversible,
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5UnsafeRepairGovernanceReview {
    M5UnsafeRepairGovernanceReview {
        note_shows_blocked_action_and_reason: true,
        note_shows_recommended_safer_repair: true,
        note_shows_rollback_and_evidence_posture: true,
        note_always_offers_dismiss_and_preserve_evidence: true,
        destructive_reset_never_equals_reviewed_transaction: true,
        guidance_shows_blast_radius_and_change_classes: true,
        guidance_shows_decline_continuity: true,
        user_can_see_why_safer_and_evidence_remains: true,
        approval_required_before_irreversible_repair: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_block_and_guidance_truth: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
        no_surface_masks_reason_repair_or_evidence: true,
    }
}

fn consumer_projection() -> M5UnsafeRepairConsumerProjection {
    M5UnsafeRepairConsumerProjection {
        doctor_and_support_surfaces_consume_reason_vocabulary: true,
        note_posture_reads_single_source: true,
        guidance_posture_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5UnsafeRepairProofFreshness {
    M5UnsafeRepairProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5UnsafeRepairReleasePosture {
    M5UnsafeRepairReleasePosture {
        release_packet_ref: M5_UNSAFE_REPAIR_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_UNSAFE_REPAIR_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_UNSAFE_REPAIR_SCHEMA_REF,
        M5_UNSAFE_REPAIR_DOC_REF,
        M5_UNSAFE_REPAIR_COMPONENT_MATRIX_REF,
        M5_UNSAFE_REPAIR_REPAIR_TRANSACTION_REF,
        M5_UNSAFE_REPAIR_RECOVERY_ACTION_REF,
        M5_UNSAFE_REPAIR_RECOVERY_LADDER_REF,
        M5_UNSAFE_REPAIR_EXPORT_REDACTION_PROFILE_REF,
        M5_UNSAFE_REPAIR_DOCTOR_FINDING_REF,
    ])
}

/// Builds the canonical M5 unsafe-fix / approved-repair primitive packet.
pub fn seeded_m5_unsafe_repair_packet() -> M5UnsafeRepairPacket {
    M5UnsafeRepairPacket::new(M5UnsafeRepairPacketInput {
        packet_id: M5_UNSAFE_REPAIR_PACKET_ID.to_owned(),
        matrix_label:
            "M5 unsafe-fix-blocked-note / approved-repair-guidance primitive: blocked action, block reason, scenario code, finding lineage, recommended safer repair, preserved rollback / evidence posture, and case disposition for the blocked note; repair class, blast radius, changed / unchanged classes, reversibility, and decline continuity for the approved-repair guidance"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5UnsafeRepairVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the recovery-center repair guidance consumer is narrowed to Preview
/// pending decline-continuity parity proof across every deployment; every consumer stays
/// visible.
pub fn seeded_m5_unsafe_repair_recovery_center_repair_guidance_preview_narrowed(
) -> M5UnsafeRepairPacket {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.packet_id =
        "m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive:recovery-center-repair-guidance-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5UnsafeRepairConsumerSurface::RecoveryCenterRepairGuidance
        })
        .expect("recovery-center-repair-guidance row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI repair review consumer is held at Beta because a
/// slice of headless notes do not yet render the keyboard route cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_unsafe_repair_headless_cli_repair_review_beta_narrowed() -> M5UnsafeRepairPacket {
    let mut packet = seeded_m5_unsafe_repair_packet();
    packet.packet_id =
        "m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive:headless-cli-repair-review-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5UnsafeRepairConsumerSurface::HeadlessCliRepairReview)
        .expect("headless-cli-repair-review row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}
