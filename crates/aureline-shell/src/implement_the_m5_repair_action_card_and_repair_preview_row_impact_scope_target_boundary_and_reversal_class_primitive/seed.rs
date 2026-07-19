// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 repair-action-card primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical repair-action-card primitive packet.
pub const M5_REPAIR_ACTION_CARD_PRIMITIVE_PACKET_ID: &str =
    "m5-repair-action-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn changes(items: &[M5RepairChangeClass]) -> Vec<M5RepairChangeClass> {
    items.to_vec()
}

/// Builds a worked resolution case from a full repair state.
#[allow(clippy::too_many_arguments)]
fn case(
    repair_title: &str,
    repair_class: M5RepairClass,
    target_scope_repr: &str,
    blast_radius: M5RepairBlastRadius,
    host_boundary: M5HostBoundaryClass,
    reversibility: M5ReversibilityClass,
    trust_requirement: M5RepairTrustRequirement,
    changed_classes: Vec<M5RepairChangeClass>,
    unchanged_classes: Vec<M5RepairChangeClass>,
    preview_only: bool,
    approval_required: bool,
    rerunnable: bool,
    factory_reset_out_of_band: bool,
) -> M5RepairActionResolutionCase {
    M5RepairActionResolutionCase::resolved(M5RepairActionResolutionInput {
        repair_title: repair_title.to_owned(),
        repair_class,
        target_scope_repr: target_scope_repr.to_owned(),
        blast_radius,
        host_boundary,
        reversibility,
        trust_requirement,
        changed_classes,
        unchanged_classes,
        preview_only,
        approval_required,
        rerunnable,
        factory_reset_out_of_band,
    })
}

/// A base row with the shared fields filled in and the full card-part, preview-row-part,
/// repair-class, blast-radius, target-boundary, reversibility, trust-requirement,
/// change-class, action-label-class, action, export-field, and accessibility parity every
/// surface carries.
fn base_row(
    consumer_surface: M5RepairConsumerSurface,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5RepairActionResolutionCase>,
) -> M5RepairConsumerRow {
    M5RepairConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        card_parts: M5RepairActionCardPart::ALL.to_vec(),
        preview_row_parts: M5RepairPreviewRowPart::ALL.to_vec(),
        repair_classes: M5RepairClass::ALL.to_vec(),
        blast_radii: M5RepairBlastRadius::ALL.to_vec(),
        target_boundaries: M5RepairTargetBoundary::ALL.to_vec(),
        reversibility_classes: M5ReversibilityClass::ALL.to_vec(),
        trust_requirements: M5RepairTrustRequirement::ALL.to_vec(),
        change_classes: M5RepairChangeClass::ALL.to_vec(),
        action_label_classes: M5RepairActionLabelClass::ALL.to_vec(),
        repair_actions: M5RepairAction::ALL.to_vec(),
        export_fields: M5RepairExportField::ALL.to_vec(),
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5RuntimeBoundaryDowngradeTrigger::RepairBlastRadiusUnderstated,
            M5RuntimeBoundaryDowngradeTrigger::ReversibilityOverstated,
            M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked,
            M5RuntimeBoundaryDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5RuntimeBoundaryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REPAIR_ACTION_CARD_SCHEMA_REF,
            M5_REPAIR_PREVIEW_ROW_SCHEMA_REF,
            M5_REPAIR_ACTION_CARD_TRANSACTION_REF,
        ]),
        example_resolutions,
        understates_blast_radius: false,
        overstates_reversibility: false,
        masks_target_boundary: false,
        hides_changed_or_unchanged_classes: false,
    }
}

fn consumer_rows() -> Vec<M5RepairConsumerRow> {
    use M5HostBoundaryClass as Host;
    use M5RepairBlastRadius as Blast;
    use M5RepairChangeClass as Change;
    use M5RepairClass as Class;
    use M5RepairTrustRequirement as Trust;
    use M5ReversibilityClass as Rev;

    let mut rows = Vec::with_capacity(9);

    // 1. Project Doctor panel — a local, exact-reversible config repair reading as an
    //    ordinary apply, and a no-writes preview of an index rebuild.
    rows.push(base_row(
        M5RepairConsumerSurface::ProjectDoctorPanel,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Project Doctor panel owner",
        "The Project Doctor panel renders the shared repair action card and preview row so a local exact-reversible environment-config repair reads as an ordinary apply with its changed and untouched classes named, while a no-writes index-rebuild preview stays preview-only and changes nothing",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-repair-doctor-panel:001",
        vec![
            case(
                "doctor-env-config-apply",
                Class::RepairEnvironmentConfig,
                "scope:workspace-env",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::WorkspaceConfig]),
                changes(&[Change::UserSourceFiles, Change::CacheArtifacts]),
                false,
                false,
                true,
                false,
            ),
            case(
                "doctor-index-preview",
                Class::RebuildIndex,
                "scope:workspace-index",
                Blast::NoWritesPreview,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::LocalConfirmation,
                changes(&[]),
                changes(&[Change::IndexData, Change::UserSourceFiles]),
                true,
                false,
                true,
                false,
            ),
        ],
    ));

    // 2. Doctor repair card — a non-exact cache clear (reads as non-exact, not "Fix now"),
    //    and a policy-gated toolchain reinstall that must request approval first.
    rows.push(base_row(
        M5RepairConsumerSurface::DoctorRepairCard,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Doctor repair card owner",
        "The Doctor repair card renders the shared components so a backup-reversible cache clear reads as a non-exact apply rather than a generic fix, and so a policy-gated toolchain reinstall requests approval before any mutation and never reads like a plain button",
        M5ShellZoneSlot::RightInspector,
        "evidence:m5-repair-doctor-card:001",
        vec![
            case(
                "card-cache-clear-nonexact",
                Class::ClearCache,
                "scope:workspace-cache",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::ReversibleWithBackup,
                Trust::NoElevation,
                changes(&[Change::CacheArtifacts]),
                changes(&[Change::WorkspaceConfig, Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
            case(
                "card-toolchain-approval",
                Class::ReinstallToolchain,
                "scope:toolchain-primary",
                Blast::ToolchainScoped,
                Host::LocalHost,
                Rev::ReversibleWithBackup,
                Trust::PolicyApprovalRequired,
                changes(&[Change::ToolchainBinaries]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
        ],
    ));

    // 3. Guided repair wizard — a remote host-environment repair reviewed off-device
    //    (partially reversible), and a local exact-reversible lockfile regeneration.
    rows.push(base_row(
        M5RepairConsumerSurface::GuidedRepairWizard,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Guided repair wizard owner",
        "The guided repair wizard renders the shared components so a partially reversible host-environment repair on a remote target reads as an off-device review rather than a local fix, and so a local exact-reversible lockfile regeneration reads as an ordinary apply with both change lists shown",
        M5ShellZoneSlot::TransientOverlay,
        "evidence:m5-repair-wizard:001",
        vec![
            case(
                "wizard-remote-env",
                Class::RepairEnvironmentConfig,
                "scope:remote-env",
                Blast::HostEnvironmentScoped,
                Host::RemoteSshHost,
                Rev::PartiallyReversible,
                Trust::AdminElevation,
                changes(&[Change::WorkspaceConfig, Change::FilePermissions]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
            case(
                "wizard-lockfile-apply",
                Class::RegenerateLockfile,
                "scope:workspace-lockfile",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::LocalConfirmation,
                changes(&[Change::LockfileState]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
        ],
    ));

    // 4. Support-bundle repair row — a local exact-reversible permission repair, and a
    //    multi-target factory reset performed out of band with manual reversal.
    rows.push(base_row(
        M5RepairConsumerSurface::SupportBundleRepairRow,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Support-bundle repair row owner",
        "The support-bundle repair row renders the shared components so a local exact-reversible permission repair explains what it changes and leaves untouched outside the live UI, and so a multi-target factory reset reads as an out-of-band reset requiring manual reversal rather than a generic fix",
        M5ShellZoneSlot::BottomPanel,
        "evidence:m5-repair-support-bundle:001",
        vec![
            case(
                "bundle-permissions-apply",
                Class::RepairPermissions,
                "scope:workspace-permissions",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::FilePermissions]),
                changes(&[Change::UserSourceFiles, Change::WorkspaceConfig]),
                false,
                false,
                false,
                false,
            ),
            case(
                "bundle-factory-reset",
                Class::FactoryResetComponent,
                "scope:component-local-plus-remote",
                Blast::MultiTargetScoped,
                Host::LocalHost,
                Rev::ReversalRequiresManualSteps,
                Trust::AdminElevation,
                changes(&[Change::WorkspaceConfig, Change::CacheArtifacts, Change::IndexData]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                false,
                true,
            ),
        ],
    ));

    // 5. Environment repair prompt — a managed, irreversible, admin-managed env repair
    //    that requests approval, and a managed container index rebuild reviewed
    //    off-device.
    rows.push(base_row(
        M5RepairConsumerSurface::EnvironmentRepairPrompt,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Environment repair prompt owner",
        "The environment repair prompt renders the shared components so an administrator-managed irreversible environment repair on a managed workspace requests approval and confirms irreversibility, and so a container-scoped index rebuild reads as an off-device review because the target is managed rather than local",
        M5ShellZoneSlot::TitleContextBar,
        "evidence:m5-repair-env-prompt:001",
        vec![
            case(
                "env-managed-approval",
                Class::RepairEnvironmentConfig,
                "scope:managed-workspace-env",
                Blast::HostEnvironmentScoped,
                Host::ManagedWorkspaceHost,
                Rev::IrreversibleConfirmed,
                Trust::ManagedByAdministrator,
                changes(&[Change::WorkspaceConfig]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                false,
                false,
            ),
            case(
                "env-container-index",
                Class::RebuildIndex,
                "scope:container-index",
                Blast::WorkspaceScoped,
                Host::ContainerHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::IndexData]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
        ],
    ));

    // 6. Toolchain repair card — a non-exact toolchain reinstall, and a no-writes preview
    //    of a lockfile regeneration.
    rows.push(base_row(
        M5RepairConsumerSurface::ToolchainRepairCard,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Toolchain repair card owner",
        "The toolchain repair card renders the shared components so a backup-reversible toolchain reinstall reads as a non-exact apply that keeps user sources and config untouched, and so a no-writes lockfile regeneration preview stays preview-only and writes nothing",
        M5ShellZoneSlot::RightInspector,
        "evidence:m5-repair-toolchain:001",
        vec![
            case(
                "toolchain-reinstall-nonexact",
                Class::ReinstallToolchain,
                "scope:toolchain-pinned",
                Blast::ToolchainScoped,
                Host::LocalHost,
                Rev::ReversibleWithBackup,
                Trust::AdminElevation,
                changes(&[Change::ToolchainBinaries]),
                changes(&[Change::UserSourceFiles, Change::WorkspaceConfig]),
                false,
                false,
                true,
                false,
            ),
            case(
                "toolchain-lockfile-preview",
                Class::RegenerateLockfile,
                "scope:toolchain-lockfile",
                Blast::NoWritesPreview,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[]),
                changes(&[Change::LockfileState, Change::ToolchainBinaries]),
                true,
                false,
                true,
                false,
            ),
        ],
    ));

    // 7. Remote-host repair card — a remote reconnect reviewed off-device, and a remote
    //    permission repair whose fix explicitly requires approval.
    rows.push(base_row(
        M5RepairConsumerSurface::RemoteHostRepairCard,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Remote-host repair card owner",
        "The remote-host repair card renders the shared components so a multi-target remote reconnect reads as an off-device review that never masks the remote boundary as local, and so a remote permission repair whose fix requires explicit approval requests it before running",
        M5ShellZoneSlot::TitleContextBar,
        "evidence:m5-repair-remote-host:001",
        vec![
            case(
                "remote-reconnect-review",
                Class::ReconnectRemoteTarget,
                "scope:remote-vm-session",
                Blast::MultiTargetScoped,
                Host::VirtualMachineHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::RemoteSessionState]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
            case(
                "remote-permissions-approval",
                Class::RepairPermissions,
                "scope:remote-ssh-permissions",
                Blast::HostEnvironmentScoped,
                Host::RemoteSshHost,
                Rev::PartiallyReversible,
                Trust::NoElevation,
                changes(&[Change::FilePermissions]),
                changes(&[Change::RemoteSessionState]),
                false,
                true,
                true,
                false,
            ),
        ],
    ));

    // 8. Repair preview sheet — a no-writes cache-clear preview naming everything left
    //    untouched, and a local exact-reversible repair disclosing both change lists
    //    before it runs (the AC1 review-before-mutation example).
    rows.push(base_row(
        M5RepairConsumerSurface::RepairPreviewSheet,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Repair preview sheet owner",
        "The repair preview sheet renders the shared components so a no-writes cache-clear preview names every untouched class and writes nothing, and so a local exact-reversible environment repair discloses both its changed and its untouched classes before it runs so a user can review blast radius and reversibility first",
        M5ShellZoneSlot::TransientOverlay,
        "evidence:m5-repair-preview-sheet:001",
        vec![
            case(
                "preview-cache-clear",
                Class::ClearCache,
                "scope:workspace-cache-preview",
                Blast::NoWritesPreview,
                Host::LocalHost,
                Rev::ReversibleWithBackup,
                Trust::NoElevation,
                changes(&[]),
                changes(&[Change::CacheArtifacts, Change::WorkspaceConfig, Change::UserSourceFiles]),
                true,
                false,
                true,
                false,
            ),
            case(
                "preview-env-review-before-apply",
                Class::RepairEnvironmentConfig,
                "scope:workspace-env-review",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::LocalConfirmation,
                changes(&[Change::WorkspaceConfig, Change::IndexData]),
                changes(&[Change::UserSourceFiles, Change::CacheArtifacts]),
                false,
                false,
                true,
                false,
            ),
        ],
    ));

    // 9. Activity-center repair — a sandboxed index rebuild reviewed off-device, and a
    //    local exact-reversible cache clear.
    rows.push(base_row(
        M5RepairConsumerSurface::ActivityCenterRepair,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Activity-center repair owner",
        "The activity-center repair entry renders the shared components so a sandboxed index rebuild reads as an off-device review because the target is managed, and so a local exact-reversible cache clear reads as an ordinary apply with both change lists preserved into the activity feed",
        M5ShellZoneSlot::ActivityRail,
        "evidence:m5-repair-activity:001",
        vec![
            case(
                "activity-sandbox-index",
                Class::RebuildIndex,
                "scope:sandbox-index",
                Blast::WorkspaceScoped,
                Host::WasmSandboxHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::IndexData]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
            case(
                "activity-cache-apply",
                Class::ClearCache,
                "scope:workspace-cache-local",
                Blast::WorkspaceScoped,
                Host::LocalHost,
                Rev::FullyReversibleCheckpoint,
                Trust::NoElevation,
                changes(&[Change::CacheArtifacts]),
                changes(&[Change::UserSourceFiles]),
                false,
                false,
                true,
                false,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5RepairGovernanceReview {
    M5RepairGovernanceReview {
        one_primitive_carries_repair_truth: true,
        blast_radius_and_reversibility_reviewable_before_mutation: true,
        changed_and_unchanged_classes_both_identified: true,
        blast_radius_never_understated: true,
        reversibility_never_overstated: true,
        target_boundary_never_masked: true,
        non_generic_action_labels_for_gated_or_non_exact_repairs: true,
        preview_and_reversal_vocabulary_in_support_export: true,
        no_surface_invents_second_repair_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5RepairConsumerProjection {
    M5RepairConsumerProjection {
        recovery_surfaces_consume_shared_primitive: true,
        repair_resolver_reads_single_transaction_source: true,
        preview_rows_read_single_preview_source: true,
        target_boundary_reads_single_host_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5RepairProofFreshness {
    M5RepairProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RepairReleasePosture {
    M5RepairReleasePosture {
        release_packet_ref: M5_REPAIR_ACTION_CARD_ARTIFACT_REF.to_owned(),
        repair_audit_ref: M5_REPAIR_ACTION_CARD_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REPAIR_ACTION_CARD_SCHEMA_REF,
        M5_REPAIR_PREVIEW_ROW_SCHEMA_REF,
        M5_REPAIR_ACTION_CARD_DOC_REF,
        M5_REPAIR_ACTION_CARD_SHELL_ZONE_REF,
        M5_REPAIR_ACTION_CARD_COMPONENT_MATRIX_REF,
        M5_REPAIR_ACTION_CARD_TRANSACTION_REF,
        M5_REPAIR_ACTION_CARD_PREVIEW_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 repair-action-card primitive packet.
pub fn seeded_m5_repair_action_card_primitive_packet() -> M5RepairActionCardPrimitivePacket {
    M5RepairActionCardPrimitivePacket::new(M5RepairActionCardPrimitivePacketInput {
        packet_id: M5_REPAIR_ACTION_CARD_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 repair action card and repair preview row primitive: repair class, target / scope, changed-versus-unchanged classes, local-or-remote-or-managed boundary, trust / policy requirement, and reversal-class honesty"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5RepairVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the remote-host repair card is held at Beta because a slice of
/// remote reconnect flows do not yet render the reversal-class badge on every profile;
/// every surface stays visible.
pub fn seeded_m5_repair_action_card_primitive_remote_host_repair_card_beta_narrowed(
) -> M5RepairActionCardPrimitivePacket {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.packet_id =
        "m5-repair-action-card-primitive:remote-host-repair-card-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepairConsumerSurface::RemoteHostRepairCard)
        .expect("remote-host repair card row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the repair preview sheet is narrowed to Preview pending
/// changed-versus-unchanged parity proof across every export path; every surface stays
/// visible.
pub fn seeded_m5_repair_action_card_primitive_repair_preview_sheet_preview_narrowed(
) -> M5RepairActionCardPrimitivePacket {
    let mut packet = seeded_m5_repair_action_card_primitive_packet();
    packet.packet_id =
        "m5-repair-action-card-primitive:repair-preview-sheet-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepairConsumerSurface::RepairPreviewSheet)
        .expect("repair preview sheet row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
