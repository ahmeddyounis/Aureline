//! Canonical seed builders for the M5 managed-deployment operations and policy-bootstrap-injection registries
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean operation and injection entries prove the supported
//! managed operation, the complete copyable receipt inventory, the never-user-controlled managed installer, the
//! disclosed admin-versus-user ownership, the published policy-bootstrap injection, and the documented
//! channel-pin / update-deferral continuity across the installer, update, diagnostics, admin, docs, and support
//! surfaces without any hand-copied per-profile assumption, misrepresented installer, ambiguous ownership,
//! undisclosed field, or presentation-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_PACKET_ID: &str =
    "m5-managed-deployment-operations-and-policy-bootstrap-injection:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn operation(input: M5ManagedOperationEntryResolutionInput) -> M5ResolvedManagedOperationEntry {
    resolve_managed_operation_entry(input).expect("seed managed-operation entry resolves")
}

fn injection(input: M5PolicyInjectionEntryResolutionInput) -> M5ResolvedPolicyInjectionEntry {
    resolve_policy_bootstrap_injection_entry(input).expect("seed policy-injection entry resolves")
}

fn all_forms() -> Vec<M5ManagedPresentationForm> {
    M5ManagedPresentationForm::ALL.to_vec()
}

fn all_receipt_fields() -> Vec<M5ManagedReceiptField> {
    M5ManagedReceiptField::ALL.to_vec()
}

fn all_injection_fields() -> Vec<M5PolicyInjectionField> {
    M5PolicyInjectionField::ALL.to_vec()
}

// -- Clean managed-operation entries ------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_operation_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    op: M5ManagedOperation,
    surface_context: M5ManagedSurfaceContext,
    ownership: M5ManagedOwnership,
    operation_target_root: &str,
    receipt_root: &str,
    failure_diagnostics_root: &str,
) -> M5ManagedOperationEntryResolutionInput {
    M5ManagedOperationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        operation: op,
        surface_context,
        presentation_form_coverage: all_forms(),
        operation_target_root: operation_target_root.to_owned(),
        receipt_root: receipt_root.to_owned(),
        failure_diagnostics_root: failure_diagnostics_root.to_owned(),
        receipt_fields_covered: all_receipt_fields(),
        ownership,
        bound_to_registry: true,
        ownership_misrepresented_used: false,
        ownership_disclosure_enforced: true,
        proof_fresh: true,
    }
}

fn operation_silent_install_installer_clean() -> M5ResolvedManagedOperationEntry {
    operation(clean_operation_base(
        "operation:silent-install:installer",
        "profile.per_machine_managed",
        "managed.operation.silent_install",
        M5InstallTopologyRole::InstallMode,
        M5ManagedOperation::SilentInstall,
        M5ManagedSurfaceContext::InstallerFlow,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    ))
}

fn operation_update_defer_update_clean() -> M5ResolvedManagedOperationEntry {
    operation(clean_operation_base(
        "operation:update-defer:update",
        "profile.per_machine_managed",
        "managed.operation.update_defer",
        M5InstallTopologyRole::UpdaterOwner,
        M5ManagedOperation::UpdateDefer,
        M5ManagedSurfaceContext::UpdateFlow,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    ))
}

fn operation_repair_verify_diagnostics_clean() -> M5ResolvedManagedOperationEntry {
    operation(clean_operation_base(
        "operation:repair-verify:diagnostics",
        "profile.per_machine_managed",
        "managed.operation.repair_or_verify",
        M5InstallTopologyRole::RollbackTarget,
        M5ManagedOperation::RepairOrVerify,
        M5ManagedSurfaceContext::DiagnosticsSurface,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    ))
}

fn operation_channel_pin_admin_clean() -> M5ResolvedManagedOperationEntry {
    operation(clean_operation_base(
        "operation:channel-pin:admin",
        "profile.per_machine_managed",
        "managed.operation.channel_pin",
        M5InstallTopologyRole::PolicyRoots,
        M5ManagedOperation::ChannelPin,
        M5ManagedSurfaceContext::AdminSurface,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    ))
}

fn operation_silent_uninstall_support_clean() -> M5ResolvedManagedOperationEntry {
    operation(clean_operation_base(
        "operation:silent-uninstall:support",
        "profile.per_user_managed",
        "managed.operation.silent_uninstall",
        M5InstallTopologyRole::InstallMode,
        M5ManagedOperation::SilentUninstall,
        M5ManagedSurfaceContext::SupportOrExportForm,
        M5ManagedOwnership::UserOwned,
        r"%LOCALAPPDATA%\Aureline",
        r"%LOCALAPPDATA%\Aureline\receipts",
        r"%LOCALAPPDATA%\Aureline\logs",
    ))
}

// -- Degraded managed-operation entries ---------------------------------------------------------

/// Degraded operation entry: the receipt inventory is incomplete — the repair/verify receipt is not published.
fn operation_receipt_incomplete() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:silent-install:receipt-incomplete",
        "profile.per_machine_managed",
        "managed.operation.silent_install",
        M5InstallTopologyRole::InstallMode,
        M5ManagedOperation::SilentInstall,
        M5ManagedSurfaceContext::InstallerFlow,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    );
    base.receipt_fields_covered = vec![
        M5ManagedReceiptField::InstallId,
        M5ManagedReceiptField::Timestamp,
        M5ManagedReceiptField::FailureSummary,
        // RepairVerifyReceipt is dropped: an automated flow can no longer confirm the install is intact.
    ];
    operation(base)
}

/// Degraded operation entry: the managed installer was presented as user-controlled (admin ownership hidden and
/// disclosure not enforced) — the operation reads as accountable when it is not.
fn operation_misrepresented() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:update-defer:misrepresented",
        "profile.per_machine_managed",
        "managed.operation.update_defer",
        M5InstallTopologyRole::UpdaterOwner,
        M5ManagedOperation::UpdateDefer,
        M5ManagedSurfaceContext::UpdateFlow,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    );
    base.ownership_misrepresented_used = true;
    base.ownership_disclosure_enforced = false;
    operation(base)
}

/// Degraded operation entry: the ownership is ambiguous, so a failure would strand the user without knowing who
/// owns the operation.
fn operation_ownership_ambiguous() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:repair-verify:ownership-ambiguous",
        "profile.per_machine_managed",
        "managed.operation.repair_or_verify",
        M5InstallTopologyRole::RollbackTarget,
        M5ManagedOperation::RepairOrVerify,
        M5ManagedSurfaceContext::DiagnosticsSurface,
        M5ManagedOwnership::OwnershipAmbiguous,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    );
    base.ownership = M5ManagedOwnership::OwnershipAmbiguous;
    operation(base)
}

/// Degraded operation entry: the behavior is a hand-copied per-profile assumption instead of tracing to the
/// registry.
fn operation_unbound() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:channel-pin:unbound",
        "profile.per_machine_managed",
        "managed.operation.channel_pin",
        M5InstallTopologyRole::PolicyRoots,
        M5ManagedOperation::ChannelPin,
        M5ManagedSurfaceContext::AdminSurface,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    );
    base.bound_to_registry = false;
    operation(base)
}

/// Degraded operation entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn operation_form_incomplete() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:silent-install:form-incomplete",
        "profile.per_machine_managed",
        "managed.operation.silent_install",
        M5InstallTopologyRole::InstallMode,
        M5ManagedOperation::SilentInstall,
        M5ManagedSurfaceContext::InstallerFlow,
        M5ManagedOwnership::AdminOwned,
        r"%ProgramFiles%\Aureline",
        r"%ProgramData%\Aureline\receipts",
        r"%ProgramData%\Aureline\logs",
    );
    base.presentation_form_coverage = vec![M5ManagedPresentationForm::CanonicalObject];
    operation(base)
}

/// Degraded operation entry: the canonical registry token name is unstated.
fn operation_token_unstated() -> M5ResolvedManagedOperationEntry {
    let mut base = clean_operation_base(
        "operation:silent-uninstall:token-unstated",
        "profile.per_user_managed",
        "  ",
        M5InstallTopologyRole::InstallMode,
        M5ManagedOperation::SilentUninstall,
        M5ManagedSurfaceContext::SupportOrExportForm,
        M5ManagedOwnership::UserOwned,
        r"%LOCALAPPDATA%\Aureline",
        r"%LOCALAPPDATA%\Aureline\receipts",
        r"%LOCALAPPDATA%\Aureline\logs",
    );
    base.token_name = "  ".to_owned();
    operation(base)
}

// -- Clean policy-injection entries -------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_injection_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    injection_surface: M5PolicyInjectionSurface,
    surface_context: M5ManagedSurfaceContext,
    posture: M5ChannelDeferralPosture,
    policy_bundle_source: &str,
    bootstrap_target: &str,
) -> M5PolicyInjectionEntryResolutionInput {
    M5PolicyInjectionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        injection_surface,
        surface_context,
        presentation_form_coverage: all_forms(),
        policy_bundle_source: policy_bundle_source.to_owned(),
        bootstrap_target: bootstrap_target.to_owned(),
        disclosed_fields: all_injection_fields(),
        pin_deferral_posture: posture,
        pin_and_deferral_continuity_documented: true,
        admin_control_disclosed: true,
        proof_fresh: true,
    }
}

fn injection_managed_channel_installer_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:managed-channel:installer",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::InstallerFlow,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

fn injection_managed_channel_update_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:managed-channel:update",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::UpdateFlow,
        M5ChannelDeferralPosture::UpdateDeferred,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

fn injection_managed_channel_diagnostics_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:managed-channel:diagnostics",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::DiagnosticsSurface,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

fn injection_managed_channel_admin_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:managed-channel:admin",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::AdminSurface,
        M5ChannelDeferralPosture::UnmanagedChannel,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

fn injection_docs_help_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:docs:help",
        "profile.per_machine_managed",
        "managed.injection.docs",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::DocsHelpInjection,
        M5ManagedSurfaceContext::DiagnosticsSurface,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

fn injection_support_export_clean() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:support:export",
        "profile.per_user_managed",
        "managed.injection.support",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::SupportExportInjection,
        M5ManagedSurfaceContext::SupportOrExportForm,
        M5ChannelDeferralPosture::UpdateDeferred,
        r"%LOCALAPPDATA%\Aureline\policy\bootstrap.json",
        r"%LOCALAPPDATA%\Aureline\policy\applied",
    ))
}

// -- Degraded policy-injection entries ----------------------------------------------------------

/// Degraded injection entry: the disclosure is incomplete — the deferral window is not disclosed and the admin
/// ownership is left implicit.
fn injection_disclosure_incomplete() -> M5ResolvedPolicyInjectionEntry {
    let mut base = clean_injection_base(
        "injection:managed-channel:disclosure-incomplete",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::InstallerFlow,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    );
    base.disclosed_fields = vec![
        M5PolicyInjectionField::PolicyBundleSource,
        M5PolicyInjectionField::BootstrapTarget,
        M5PolicyInjectionField::AppliedSettings,
        M5PolicyInjectionField::AdminOwner,
        // DeferralWindow is dropped: the injected policy's deferral window stays implicit.
    ];
    base.admin_control_disclosed = false;
    injection(base)
}

/// Degraded injection entry: the channel-pin / update-deferral continuity note is absent.
fn injection_continuity_undocumented() -> M5ResolvedPolicyInjectionEntry {
    let mut base = clean_injection_base(
        "injection:managed-channel:continuity-undocumented",
        "profile.per_machine_managed",
        "managed.injection.channel",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::ManagedPolicyChannel,
        M5ManagedSurfaceContext::UpdateFlow,
        M5ChannelDeferralPosture::UpdateDeferred,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    );
    base.pin_and_deferral_continuity_documented = false;
    injection(base)
}

/// Degraded injection entry: the injection surface is unclassified.
fn injection_surface_unclassified() -> M5ResolvedPolicyInjectionEntry {
    injection(clean_injection_base(
        "injection:admin:surface-unclassified",
        "profile.per_machine_managed",
        "managed.injection.unknown",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::SurfaceUnclassified,
        M5ManagedSurfaceContext::AdminSurface,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    ))
}

/// Degraded injection entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn injection_form_incomplete() -> M5ResolvedPolicyInjectionEntry {
    let mut base = clean_injection_base(
        "injection:docs:form-incomplete",
        "profile.per_machine_managed",
        "managed.injection.docs",
        M5InstallTopologyRole::PolicyRoots,
        M5PolicyInjectionSurface::DocsHelpInjection,
        M5ManagedSurfaceContext::DiagnosticsSurface,
        M5ChannelDeferralPosture::ChannelPinned,
        r"%ProgramData%\Aureline\policy\bootstrap.json",
        r"%ProgramData%\Aureline\policy\applied",
    );
    base.presentation_form_coverage = vec![M5ManagedPresentationForm::CanonicalObject];
    injection(base)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ManagedConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    managed_operation_entries: Vec<M5ResolvedManagedOperationEntry>,
    policy_injection_entries: Vec<M5ResolvedPolicyInjectionEntry>,
) -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow {
        consumer_surface,
        qualification: M5InstallTopologyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5InstallTopologyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5InstallTopologyRequiredLabel::Identity,
            M5InstallTopologyRequiredLabel::SemanticRole,
            M5InstallTopologyRequiredLabel::RegistryReference,
            M5InstallTopologyRequiredLabel::InstallMode,
            M5InstallTopologyRequiredLabel::RollbackTarget,
        ],
        accessibility_routes: M5InstallTopologyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ManagedAnatomyPart::ALL.to_vec(),
        export_fields: M5ManagedExportField::ALL.to_vec(),
        downgrade_triggers,
        managed_operation_entries,
        policy_injection_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
        ]),
        managed_installer_presented_as_user_controlled: false,
        managed_failure_stranded_user_without_diagnostics: false,
        channel_pinning_or_repair_verify_drifted_from_matrix: false,
        policy_bootstrap_injection_ownership_left_undisclosed: false,
    }
}

fn registry_rows() -> Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow> {
    use M5InstallTopologyConsumerSurface as C;
    use M5InstallTopologyDowngradeTrigger as D;

    vec![
        base_row(
            C::Installer,
            "Installer/silent-flow owner",
            "The installer resolves the silent-install operation to one inspectable object — operation, operation-target / receipt / failure-diagnostics roots, and the copyable install-ID / timestamp / failure-summary / repair-verify receipt — from the shared registry and reads the managed-policy injection; a receipt that omits the repair/verify confirmation and an injection that hides the deferral window and admin ownership degrade honestly instead of reading as a clean pass",
            "evidence:m5-managed-installer:001",
            vec![
                D::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
                D::ProofStale,
            ],
            vec![
                operation_silent_install_installer_clean(),
                operation_receipt_incomplete(),
            ],
            vec![
                injection_managed_channel_installer_clean(),
                injection_disclosure_incomplete(),
            ],
        ),
        base_row(
            C::UpdaterService,
            "Updater/update-deferral owner",
            "The updater resolves the update-deferral operation and the managed-channel injection; a managed installer presented as user-controlled and an undocumented channel-pin / update-deferral continuity note are caught before a managed update can hide admin ownership or drift its pinning semantics",
            "evidence:m5-managed-updater:001",
            vec![
                D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
                D::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                D::ProofStale,
            ],
            vec![
                operation_update_defer_update_clean(),
                operation_misrepresented(),
            ],
            vec![
                injection_managed_channel_update_clean(),
                injection_continuity_undocumented(),
            ],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the repair-or-verify operation and its published injection without manual reconstruction; an operation whose ownership is ambiguous — so a failure would strand the user without knowing who owns it — is caught instead of reading as a clean pass",
            "evidence:m5-managed-diagnostics:001",
            vec![
                D::UpdaterOwnerUnstated,
                D::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                D::ProofStale,
            ],
            vec![
                operation_repair_verify_diagnostics_clean(),
                operation_ownership_ambiguous(),
            ],
            vec![injection_managed_channel_diagnostics_clean()],
        ),
        base_row(
            C::Admin,
            "Admin surface owner",
            "Admin resolves the channel-pin operation while preserving one registry-bound source; a hand-copied per-profile assumption and an injection record on an unclassified surface degrade honestly",
            "evidence:m5-managed-admin:001",
            vec![
                D::StateRootBoundaryDriftedByTopology,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                operation_channel_pin_admin_clean(),
                operation_unbound(),
            ],
            vec![
                injection_managed_channel_admin_clean(),
                injection_surface_unclassified(),
            ],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the same resolved managed-operation and published-injection truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied receipt table",
            "evidence:m5-managed-docs-help:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
                D::ProofStale,
            ],
            vec![
                operation_silent_install_installer_clean(),
                operation_form_incomplete(),
            ],
            vec![injection_docs_help_clean(), injection_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved managed-operation and injection truth, so a hand-copied constant, an unstated registry token, an ambiguous ownership, or a managed installer presented as user-controlled is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-managed-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
                D::ProofStale,
            ],
            vec![
                operation_silent_uninstall_support_clean(),
                operation_token_unstated(),
            ],
            vec![injection_support_export_clean()],
        ),
    ]
}

fn governance_review() -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionGovernanceReview {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionGovernanceReview {
        managed_registry_names_token_role_and_operation: true,
        profile_exposes_all_canonical_operations: true,
        all_receipt_fields_copyable_and_published: true,
        managed_installer_never_presented_as_user_controlled: true,
        admin_versus_user_ownership_explicit: true,
        policy_injection_published_across_surfaces: true,
        every_entry_covers_all_presentation_forms: true,
        pin_and_deferral_continuity_documented: true,
        behavior_bound_to_registry_not_hand_copied: true,
        installer_update_diagnostics_admin_read_single_source: true,
        managed_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionConsumerProjection {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionConsumerProjection {
        installer_and_update_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        updater_and_policy_channel_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionProofFreshness {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionReleasePosture {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionReleasePosture {
        proof_packet_ref:
            M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_ARTIFACT_REF.to_owned(),
        managed_operations_audit_ref:
            M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_REF,
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 managed-deployment operations and policy-bootstrap-injection registries packet.
pub fn seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection(
) -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket {
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket::new(
        M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacketInput {
            packet_id:
                M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_PACKET_ID.to_owned(),
            registries_label:
                "M5 managed-deployment operations and policy-bootstrap-injection registries enforcing silent install / uninstall / repair-or-verify / channel-pinning / update-deferral operations with a complete copyable receipt inventory of install ID / timestamp / failure summary / repair-verify receipt, a never-user-controlled managed installer, explicit admin-versus-user ownership, published policy-bootstrap injection, and documented channel-pin / update-deferral continuity across the installer, update, diagnostics, admin, docs, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionVocabularySet::canonical(),
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

/// Narrowed variant: the admin row is held at Beta pending per-machine-managed channel-pin parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed(
) -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.packet_id =
        "m5-managed-deployment-operations-and-policy-bootstrap-injection:per-machine-managed-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::Admin)
        .expect("admin row present");
    row.qualification = M5InstallTopologyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updater row is narrowed to Preview pending offline / air-gap managed-update parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed(
) -> M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket {
    let mut packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
    packet.packet_id =
        "m5-managed-deployment-operations-and-policy-bootstrap-injection:offline-airgap-bundle-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .expect("updater-service row present");
    row.qualification = M5InstallTopologyQualificationClass::Preview;
    packet
}
