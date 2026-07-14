//! Implemented M5 managed-deployment operations and policy-bootstrap-injection registries.
//!
//! The frozen [install-topology matrix][matrix] names Aureline's per-user-managed and per-machine-managed
//! delivery-topology families and the first implement lane [over the whole matrix][registries] resolves the
//! managed install-topology object. This module is the managed-deployment execution lane: it makes *managed
//! deployment* a contract instead of a set of installer flags. It turns the *silent install / uninstall /
//! repair-or-verify / channel-pinning / update-deferral operation* grammar and the *policy-bundle / bootstrap
//! injection* grammar into registry resolvers that produce export-safe, honest projections. A claimed managed
//! profile then resolves to one stable managed-operation object — the operation kind, the operation-target,
//! receipt, and failure-diagnostics roots, the full receipt inventory (copyable install ID, timestamp, failure
//! summary, and repair/verify receipt), and the explicit admin-versus-user ownership — that proves the managed
//! installer never looks user-controlled and that failures never strand the user in an ambiguous ownership
//! state, and to one policy-bootstrap-injection record — the policy-bundle source, bootstrap target, applied
//! settings, admin owner, and deferral window — that support and admin surfaces can inspect and that drills can
//! fail against when bootstrap-policy injection, channel pinning, or repair/verify semantics drift from the
//! published matrix.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Expose silent install, uninstall, repair/verify, channel pinning, and update deferral through one
//!   inspectable object.** [`resolve_managed_operation_entry`] refuses to read as a clean, registry-bound
//!   operation unless it names a canonical token, a classified [operation][M5ManagedOperation], a managed role,
//!   covers every [presentation form][M5ManagedPresentationForm], inventories every mandatory
//!   [receipt field][M5ManagedReceiptField], keeps a disclosed [ownership][M5ManagedOwnership], and proves the
//!   managed installer was never presented as user-controlled; a misrepresentation degrades to
//!   [`M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled`].
//! * **Produce copyable install IDs, timestamps, failure summaries, and repair/verify receipts.** A managed
//!   operation entry whose receipt inventory drops a mandatory field, or whose operation-target / receipt /
//!   failure-diagnostics roots are unstated, degrades to
//!   [`M5ManagedOperationEntryDegradeReason::ManagedReceiptInventoryIncomplete`] so a human or automated flow is
//!   never stranded without an actionable receipt.
//! * **Keep admin-owned versus user-owned responsibilities explicit and publish policy-bootstrap injection.**
//!   [`resolve_policy_bootstrap_injection_entry`] names a classified
//!   [injection surface][M5PolicyInjectionSurface], must disclose every mandatory
//!   [injection field][M5PolicyInjectionField] (policy-bundle source, bootstrap target, applied settings, admin
//!   owner, deferral window) and the admin ownership, and degrades to
//!   [`M5PolicyInjectionEntryDegradeReason::PolicyInjectionDisclosureIncomplete`] when a field or the admin
//!   ownership is left implicit, or to
//!   [`M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented`] when the channel-pin /
//!   update-deferral posture or its continuity note is absent.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5InstallTopologyRole`] role vocabulary,
//! the [`M5InstallTopologyConsumerSurface`] consumer-surface taxonomy, and the matrix downgrade triggers — so
//! installer, updater, diagnostics, admin, docs, CLI, and support surfaces can never fork their own managed
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_install_topology_matrix
//! [registries]: crate::m5_install_topology_and_state_root_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection,
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed,
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed,
    M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_topology_and_state_root_registries::M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF;
use crate::m5_install_topology_matrix::{
    M5InstallTopologyAccessibilityRoute, M5InstallTopologyConsumerSurface,
    M5InstallTopologyDeploymentLine, M5InstallTopologyDowngradeTrigger, M5InstallTopologyFamily,
    M5InstallTopologyQualificationClass, M5InstallTopologyRequiredLabel, M5InstallTopologyRole,
    M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF, M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
    M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket`].
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_RECORD_KIND: &str =
    "implement_m5_managed_deployment_operations_and_policy_bootstrap_injection";

/// Schema version for M5 managed-deployment operations / policy-bootstrap-injection registry records.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_REF: &str =
    "schemas/install/m5-managed-deployment-operations-and-policy-bootstrap-injection.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_DOC_REF: &str =
    "docs/install/m5_managed_deployment_operations_and_policy_bootstrap_injection.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_ARTIFACT_REF: &str =
    "artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_CSV_REF: &str =
    "artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_REPORT_REF: &str =
    "artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_FIXTURE_DIR: &str =
    "fixtures/install/m5-managed-deployment-operations-and-policy-bootstrap-injection";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5ManagedConsumerSurface = M5InstallTopologyConsumerSurface;

/// One of the three presentation forms every managed-operation or policy-injection entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedPresentationForm {
    /// The canonical resolved managed-operation / policy-injection object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved managed truth discoverable without
    /// visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved managed truth inspectable off-renderer.
    AuditRecord,
}

impl M5ManagedPresentationForm {
    /// Every presentation form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled managed-deployment operation a managed-operation entry resolves, so a claimed managed profile
/// exposes silent install, silent uninstall, repair-or-verify, channel pinning, and update deferral through one
/// inspectable contract rather than a per-installer flag. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedOperation {
    /// Silent (unattended) install.
    SilentInstall,
    /// Silent (unattended) uninstall.
    SilentUninstall,
    /// Repair or verify an existing managed install.
    RepairOrVerify,
    /// Pin the update channel (stable / beta / preview) so it never drifts silently.
    ChannelPin,
    /// Defer an update within an admin-controlled window.
    UpdateDefer,
    /// The operation is unclassified, which is disallowed.
    OperationUnclassified,
}

impl M5ManagedOperation {
    /// Every operation, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SilentInstall,
        Self::SilentUninstall,
        Self::RepairOrVerify,
        Self::ChannelPin,
        Self::UpdateDefer,
        Self::OperationUnclassified,
    ];

    /// The five canonical managed operations every claimed managed profile resolves against.
    pub const CANONICAL_OPERATIONS: [Self; 5] = [
        Self::SilentInstall,
        Self::SilentUninstall,
        Self::RepairOrVerify,
        Self::ChannelPin,
        Self::UpdateDefer,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentInstall => "silent_install",
            Self::SilentUninstall => "silent_uninstall",
            Self::RepairOrVerify => "repair_or_verify",
            Self::ChannelPin => "channel_pin",
            Self::UpdateDefer => "update_defer",
            Self::OperationUnclassified => "operation_unclassified",
        }
    }

    /// Whether the operation is one of the supported managed operations (never the unclassified sentinel).
    pub const fn is_supported(self) -> bool {
        !matches!(self, Self::OperationUnclassified)
    }
}

/// Controlled receipt field a managed operation must publish so human and automated deployment flows share one
/// copyable truth model instead of a screenshot. Minted by this lane, tracking the fields the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedReceiptField {
    /// The copyable install ID that ties the human and automated flows to one operation.
    InstallId,
    /// The operation timestamp.
    Timestamp,
    /// The actionable failure summary (empty on success, populated on failure).
    FailureSummary,
    /// The repair / verify receipt confirming the install is intact.
    RepairVerifyReceipt,
    /// The operation log reference for deeper inspection.
    OperationLog,
}

impl M5ManagedReceiptField {
    /// Every receipt field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InstallId,
        Self::Timestamp,
        Self::FailureSummary,
        Self::RepairVerifyReceipt,
        Self::OperationLog,
    ];

    /// The four receipt fields a managed operation must publish before it can read as complete — the exact
    /// copyable receipt the implementation requirement names.
    pub const MANDATORY: [Self; 4] = [
        Self::InstallId,
        Self::Timestamp,
        Self::FailureSummary,
        Self::RepairVerifyReceipt,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallId => "install_id",
            Self::Timestamp => "timestamp",
            Self::FailureSummary => "failure_summary",
            Self::RepairVerifyReceipt => "repair_verify_receipt",
            Self::OperationLog => "operation_log",
        }
    }
}

/// Controlled ownership a managed-operation entry resolves, so admin-owned versus user-owned responsibilities
/// stay explicit and a managed installer never looks user-controlled in product surfaces. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedOwnership {
    /// The operation is admin / system owned (per-machine managed, admin updater ownership).
    AdminOwned,
    /// The operation is user owned (per-user managed, user updater ownership).
    UserOwned,
    /// The operation is jointly owned with the admin-versus-user split explicitly disclosed.
    MixedDisclosed,
    /// The ownership cannot be distinguished, which is disallowed.
    OwnershipAmbiguous,
}

impl M5ManagedOwnership {
    /// Every ownership, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdminOwned,
        Self::UserOwned,
        Self::MixedDisclosed,
        Self::OwnershipAmbiguous,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminOwned => "admin_owned",
            Self::UserOwned => "user_owned",
            Self::MixedDisclosed => "mixed_disclosed",
            Self::OwnershipAmbiguous => "ownership_ambiguous",
        }
    }

    /// Whether the ownership is disclosed (never the ambiguous sentinel).
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::OwnershipAmbiguous)
    }
}

/// Controlled policy-injection surface a policy-injection entry resolves, so the policy-bundle / bootstrap
/// injection shares one registry rather than a per-surface reconstruction. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyInjectionSurface {
    /// The managed-policy channel that reads the bootstrap bundle.
    ManagedPolicyChannel,
    /// The support-export injection record.
    SupportExportInjection,
    /// The docs / help injection reference.
    DocsHelpInjection,
    /// The injection surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5PolicyInjectionSurface {
    /// Every injection surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ManagedPolicyChannel,
        Self::SupportExportInjection,
        Self::DocsHelpInjection,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical injection surfaces the published injection truth must stay complete across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::ManagedPolicyChannel,
        Self::SupportExportInjection,
        Self::DocsHelpInjection,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPolicyChannel => "managed_policy_channel",
            Self::SupportExportInjection => "support_export_injection",
            Self::DocsHelpInjection => "docs_help_injection",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the injection surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// One injection field the published policy-bootstrap injection must disclose so nothing about the injected
/// policy bundle, bootstrap target, admin owner, or deferral window is left implicit. Minted by this lane,
/// tracking the fields the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyInjectionField {
    /// The policy-bundle source the bootstrap reads from.
    PolicyBundleSource,
    /// The bootstrap target the injected policy applies to.
    BootstrapTarget,
    /// The applied settings the injection produced.
    AppliedSettings,
    /// The admin owner accountable for the injected policy.
    AdminOwner,
    /// The deferral window (channel-pin / update-deferral) the injected policy sets.
    DeferralWindow,
}

impl M5PolicyInjectionField {
    /// Every injection field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PolicyBundleSource,
        Self::BootstrapTarget,
        Self::AppliedSettings,
        Self::AdminOwner,
        Self::DeferralWindow,
    ];

    /// Every field is mandatory: a published injection record must disclose all five.
    pub const MANDATORY: [Self; 5] = Self::ALL;

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBundleSource => "policy_bundle_source",
            Self::BootstrapTarget => "bootstrap_target",
            Self::AppliedSettings => "applied_settings",
            Self::AdminOwner => "admin_owner",
            Self::DeferralWindow => "deferral_window",
        }
    }
}

/// Controlled channel-pin / update-deferral posture a policy-injection entry resolves, so a pinned channel or a
/// deferred update stays documented rather than drifting silently. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelDeferralPosture {
    /// The update channel is explicitly pinned (stable / beta / preview held constant).
    ChannelPinned,
    /// An update is deferred within a documented admin-controlled window.
    UpdateDeferred,
    /// The channel is explicitly unmanaged (user-controlled) and that is disclosed.
    UnmanagedChannel,
    /// The posture is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5ChannelDeferralPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ChannelPinned,
        Self::UpdateDeferred,
        Self::UnmanagedChannel,
        Self::PostureUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelPinned => "channel_pinned",
            Self::UpdateDeferred => "update_deferred",
            Self::UnmanagedChannel => "unmanaged_channel",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the posture is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a managed token's
/// meaning stays stable whether it appears in the installer flow, the update flow, diagnostics, admin, or a
/// support / export form. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedSurfaceContext {
    /// The installer flow surface.
    InstallerFlow,
    /// The update flow surface.
    UpdateFlow,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ManagedSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InstallerFlow,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::InstallerFlow,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallerFlow => "installer_flow",
            Self::UpdateFlow => "update_flow",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a managed-operation or policy-injection entry must be able to show, so no
/// operation, receipt field, ownership, injection field, or registry fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The operation the entry resolves (operation entry).
    Operation,
    /// The operation-target, receipt, and failure-diagnostics roots (operation entry).
    OperationAndReceiptRoots,
    /// The presentation-form coverage (canonical / accessible / audit).
    PresentationFormCoverage,
    /// The receipt inventory (operation entry).
    ReceiptInventory,
    /// The admin-versus-user ownership (operation entry).
    Ownership,
    /// The channel-pin / update-deferral posture and continuity note (injection entry).
    PinDeferralPostureAndContinuity,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved operation or injection record (both entries).
    PlainLanguageMeaning,
}

impl M5ManagedAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::Operation,
        Self::OperationAndReceiptRoots,
        Self::PresentationFormCoverage,
        Self::ReceiptInventory,
        Self::Ownership,
        Self::PinDeferralPostureAndContinuity,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::Operation => "operation",
            Self::OperationAndReceiptRoots => "operation_and_receipt_roots",
            Self::PresentationFormCoverage => "presentation_form_coverage",
            Self::ReceiptInventory => "receipt_inventory",
            Self::Ownership => "ownership",
            Self::PinDeferralPostureAndContinuity => "pin_deferral_posture_and_continuity",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// operation, an injection record, or a degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedNextAction {
    /// Expand the resolved operation's or injection record's plain-language meaning.
    ExpandManagedMeaning,
    /// Inspect the operation or injection surface the entry resolves.
    InspectOperationOrSurface,
    /// Complete the canonical / accessible / audit presentation-form coverage.
    CompletePresentationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ManagedNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandManagedMeaning,
        Self::InspectOperationOrSurface,
        Self::CompletePresentationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandManagedMeaning => "expand_managed_meaning",
            Self::InspectOperationOrSurface => "inspect_operation_or_surface",
            Self::CompletePresentationFormCoverage => "complete_presentation_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The managed operations covered.
    ManagedOperations,
    /// The ownerships carried.
    Ownerships,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The presentation forms covered.
    PresentationForms,
    /// The injection surfaces carried.
    InjectionSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The receipt fields carried.
    ReceiptFields,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ManagedExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ManagedOperations,
        Self::Ownerships,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationForms,
        Self::InjectionSurfaces,
        Self::SurfaceContext,
        Self::ReceiptFields,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ManagedOperations,
        Self::Ownerships,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ManagedOperations => "managed_operations",
            Self::Ownerships => "ownerships",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationForms => "presentation_forms",
            Self::InjectionSurfaces => "injection_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::ReceiptFields => "receipt_fields",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a managed-operation entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, misrepresented, receipt-incomplete, or
/// form-incomplete operation read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedOperationEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the operation means.
    OperationTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The operation is unclassified (not in the resolved taxonomy).
    OperationUnclassified,
    /// The behavior is a hand-copied per-profile assumption instead of tracing to the canonical registry.
    OperationNotBoundToRegistry,
    /// The receipt inventory is incomplete: a mandatory receipt field (install ID, timestamp, failure summary,
    /// or repair/verify receipt) is not published, or the operation-target / receipt / failure-diagnostics
    /// roots are unstated.
    ManagedReceiptInventoryIncomplete,
    /// The managed installer was presented as user-controlled (admin ownership hidden), and that presentation
    /// is not explicitly disclosed as honest.
    ManagedInstallerPresentedAsUserControlled,
    /// The ownership is ambiguous, so a failure would strand the user without knowing who owns the operation.
    OwnershipAmbiguous,
    /// The canonical / accessible / audit presentation-form coverage is incomplete.
    PresentationFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ManagedOperationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::OperationTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::OperationUnclassified,
        Self::OperationNotBoundToRegistry,
        Self::ManagedReceiptInventoryIncomplete,
        Self::ManagedInstallerPresentedAsUserControlled,
        Self::OwnershipAmbiguous,
        Self::PresentationFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperationTokenUnstated => "operation_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::OperationUnclassified => "operation_unclassified",
            Self::OperationNotBoundToRegistry => "operation_not_bound_to_registry",
            Self::ManagedReceiptInventoryIncomplete => "managed_receipt_inventory_incomplete",
            Self::ManagedInstallerPresentedAsUserControlled => {
                "managed_installer_presented_as_user_controlled"
            }
            Self::OwnershipAmbiguous => "ownership_ambiguous",
            Self::PresentationFormCoverageIncomplete => "presentation_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ManagedNextAction {
        match self {
            Self::OperationTokenUnstated | Self::OperationNotBoundToRegistry => {
                M5ManagedNextAction::TraceCanonicalRegistry
            }
            Self::OperationUnclassified
            | Self::ManagedReceiptInventoryIncomplete
            | Self::ManagedInstallerPresentedAsUserControlled
            | Self::OwnershipAmbiguous => M5ManagedNextAction::InspectOperationOrSurface,
            Self::PresentationFormCoverageIncomplete => {
                M5ManagedNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ManagedNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::OperationTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::PresentationFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::OperationUnclassified => M5InstallTopologyDowngradeTrigger::InstallModeUnstated,
            Self::ManagedReceiptInventoryIncomplete => {
                M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence
            }
            Self::ManagedInstallerPresentedAsUserControlled => {
                M5InstallTopologyDowngradeTrigger::UpdaterOwnershipOrAdminControlHiddenInManagedFlow
            }
            Self::OperationNotBoundToRegistry => {
                M5InstallTopologyDowngradeTrigger::StateRootBoundaryDriftedByTopology
            }
            Self::OwnershipAmbiguous => M5InstallTopologyDowngradeTrigger::UpdaterOwnerUnstated,
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a policy-bootstrap-injection entry degraded below a clean, published state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyInjectionEntryDegradeReason {
    /// The canonical registry token name is unstated.
    InjectionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The injection surface is unclassified (not in the resolved taxonomy).
    InjectionSurfaceUnclassified,
    /// The injection disclosure is incomplete — a mandatory field (policy-bundle source, bootstrap target,
    /// applied settings, admin owner, or deferral window) or the admin ownership is missing.
    PolicyInjectionDisclosureIncomplete,
    /// The channel-pin / update-deferral posture is unclassified or its continuity note is absent.
    PinAndDeferralContinuityUndocumented,
    /// The canonical / accessible / audit presentation-form coverage of the injection record is incomplete.
    InjectionFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PolicyInjectionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InjectionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::InjectionSurfaceUnclassified,
        Self::PolicyInjectionDisclosureIncomplete,
        Self::PinAndDeferralContinuityUndocumented,
        Self::InjectionFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InjectionTokenUnstated => "injection_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::InjectionSurfaceUnclassified => "injection_surface_unclassified",
            Self::PolicyInjectionDisclosureIncomplete => "policy_injection_disclosure_incomplete",
            Self::PinAndDeferralContinuityUndocumented => {
                "pin_and_deferral_continuity_undocumented"
            }
            Self::InjectionFormCoverageIncomplete => "injection_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ManagedNextAction {
        match self {
            Self::InjectionTokenUnstated => M5ManagedNextAction::TraceCanonicalRegistry,
            Self::InjectionSurfaceUnclassified
            | Self::PolicyInjectionDisclosureIncomplete
            | Self::PinAndDeferralContinuityUndocumented => {
                M5ManagedNextAction::InspectOperationOrSurface
            }
            Self::InjectionFormCoverageIncomplete => {
                M5ManagedNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ManagedNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::InjectionTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::InjectionSurfaceUnclassified
            | Self::InjectionFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::PolicyInjectionDisclosureIncomplete => {
                M5InstallTopologyDowngradeTrigger::UpdaterOwnershipOrAdminControlHiddenInManagedFlow
            }
            Self::PinAndDeferralContinuityUndocumented => {
                M5InstallTopologyDowngradeTrigger::DeploymentClaimOutpacedRingOrRepairVerifyEvidence
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_managed_operation_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ManagedOperationEntryResolutionInput {
    /// Stable identity of the operation entry.
    pub entry_id: String,
    /// The stable managed-profile ID this operation binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name (e.g. `managed.operation.silent_install`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The managed operation this entry resolves.
    pub operation: M5ManagedOperation,
    /// The render / surface context.
    pub surface_context: M5ManagedSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5ManagedPresentationForm>,
    /// The published operation-target root (a filesystem path, never a URL); empty means unstated.
    pub operation_target_root: String,
    /// The published receipt root the receipt is written to (a filesystem path); empty means unstated.
    pub receipt_root: String,
    /// The published failure-diagnostics root (a filesystem path); empty means unstated.
    pub failure_diagnostics_root: String,
    /// The receipt fields published by this operation (must cover every mandatory field).
    pub receipt_fields_covered: Vec<M5ManagedReceiptField>,
    /// The admin-versus-user ownership distinguishing who owns the operation.
    pub ownership: M5ManagedOwnership,
    /// True when the behavior traces to the shared managed registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the managed installer was actually presented as user-controlled (admin ownership hidden).
    pub ownership_misrepresented_used: bool,
    /// True when honest admin-versus-user ownership disclosure is enforced, proving absence of misrepresentation.
    pub ownership_disclosure_enforced: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe managed-operation projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedManagedOperationEntry {
    /// Stable identity of the operation entry.
    pub entry_id: String,
    /// The stable managed-profile ID this operation binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The operation token named by the entry.
    pub operation: String,
    /// Whether the operation is one of the supported managed operations.
    pub operation_is_supported: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published operation-target root.
    pub operation_target_root: String,
    /// The published receipt root.
    pub receipt_root: String,
    /// The published failure-diagnostics root.
    pub failure_diagnostics_root: String,
    /// The receipt-field tokens published by the entry.
    pub receipt_fields_covered: Vec<String>,
    /// The ownership token named by the entry.
    pub ownership: String,
    /// Whether the ownership is disclosed (admin-versus-user is explicit).
    pub ownership_is_disclosed: bool,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// Whether the receipt inventory publishes every required root and mandatory receipt field.
    pub managed_receipt_complete: bool,
    /// Whether the operation is accountable: receipt complete and the installer never presented as user
    /// controlled.
    pub operation_is_accountable: bool,
    /// Whether the behavior traces to the shared managed registry.
    pub bound_to_registry: bool,
    /// Whether the managed installer was actually presented as user-controlled.
    pub ownership_misrepresented_used: bool,
    /// Whether honest admin-versus-user ownership disclosure is enforced.
    pub ownership_disclosure_enforced: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5ManagedOperationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ManagedNextAction,
    /// Whether the operation resolves cleanly across every claimed profile (clean entry naming every fact).
    pub operation_resolves_across_profiles: bool,
}

impl M5ResolvedManagedOperationEntry {
    /// Whether this operation entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_policy_bootstrap_injection_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PolicyInjectionEntryResolutionInput {
    /// Stable identity of the injection entry.
    pub entry_id: String,
    /// The stable managed-profile ID this injection record binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The injection surface this entry resolves.
    pub injection_surface: M5PolicyInjectionSurface,
    /// The render / surface context.
    pub surface_context: M5ManagedSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5ManagedPresentationForm>,
    /// The published policy-bundle source (a filesystem path); empty means unstated.
    pub policy_bundle_source: String,
    /// The published bootstrap target (a filesystem path); empty means unstated.
    pub bootstrap_target: String,
    /// The injection fields disclosed by this record (must cover every mandatory field).
    pub disclosed_fields: Vec<M5PolicyInjectionField>,
    /// The channel-pin / update-deferral posture this record resolves.
    pub pin_deferral_posture: M5ChannelDeferralPosture,
    /// True when the channel-pin / update-deferral continuity note is documented.
    pub pin_and_deferral_continuity_documented: bool,
    /// True when the admin-versus-user ownership of the injected policy is explicitly disclosed.
    pub admin_control_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe policy-bootstrap-injection projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPolicyInjectionEntry {
    /// Stable identity of the injection entry.
    pub entry_id: String,
    /// The stable managed-profile ID this injection record binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The injection-surface token named by the entry.
    pub injection_surface: String,
    /// Whether the injection surface is classified into the resolved taxonomy.
    pub injection_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// The published policy-bundle source.
    pub policy_bundle_source: String,
    /// The published bootstrap target.
    pub bootstrap_target: String,
    /// The injection-field tokens disclosed by the entry.
    pub disclosed_fields: Vec<String>,
    /// The channel-pin / update-deferral posture token named by the entry.
    pub pin_deferral_posture: String,
    /// Whether the channel-pin / update-deferral posture is classified.
    pub pin_deferral_posture_is_classified: bool,
    /// Whether the channel-pin / update-deferral continuity note is documented.
    pub pin_and_deferral_continuity_documented: bool,
    /// Whether the admin-versus-user ownership of the injected policy is explicitly disclosed.
    pub admin_control_disclosed: bool,
    /// Whether the injection record discloses every mandatory field and the admin ownership.
    pub injection_is_disclosed: bool,
    /// Whether the channel-pin / update-deferral posture is classified and continuity is documented.
    pub pin_and_deferral_is_continuous: bool,
    /// Degrade reason, if the entry could not read as a clean, published state.
    pub degrade_reason: Option<M5PolicyInjectionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ManagedNextAction,
    /// Whether the injection record is discoverable on every claimed profile (clean entry naming every fact).
    pub injection_discoverable_on_every_profile: bool,
}

impl M5ResolvedPolicyInjectionEntry {
    /// Whether this injection entry reads as a clean, published state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ManagedResolutionError {
    /// The operation-entry id was empty.
    EmptyOperationEntryId,
    /// The injection-entry id was empty.
    EmptyInjectionEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ManagedResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyOperationEntryId => "empty_operation_entry_id",
            Self::EmptyInjectionEntryId => "empty_injection_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ManagedResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 managed-deployment operations / policy-bootstrap-injection registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ManagedResolutionError {}

fn presentation_form_tokens(forms: &[M5ManagedPresentationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_presentation_forms(forms: &[M5ManagedPresentationForm]) -> bool {
    let present: BTreeSet<M5ManagedPresentationForm> = forms.iter().copied().collect();
    M5ManagedPresentationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

fn receipt_fields_cover_mandatory(fields: &[M5ManagedReceiptField]) -> bool {
    let present: BTreeSet<M5ManagedReceiptField> = fields.iter().copied().collect();
    M5ManagedReceiptField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

fn injection_fields_cover_mandatory(fields: &[M5PolicyInjectionField]) -> bool {
    let present: BTreeSet<M5PolicyInjectionField> = fields.iter().copied().collect();
    M5PolicyInjectionField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// Whether the managed-operation receipt inventory publishes every required root and mandatory receipt field:
/// the operation must be supported, the operation-target / receipt / failure-diagnostics roots must all be
/// stated, and the copyable install ID, timestamp, failure summary, and repair/verify receipt must all be
/// published.
pub fn managed_operation_receipt_is_complete(
    operation: M5ManagedOperation,
    operation_target_root: &str,
    receipt_root: &str,
    failure_diagnostics_root: &str,
    receipt_fields_covered: &[M5ManagedReceiptField],
) -> bool {
    operation.is_supported()
        && !operation_target_root.trim().is_empty()
        && !receipt_root.trim().is_empty()
        && !failure_diagnostics_root.trim().is_empty()
        && receipt_fields_cover_mandatory(receipt_fields_covered)
}

/// Whether the managed operation is accountable: the operation must be supported, the receipt inventory must be
/// complete, the managed installer must not have been presented as user-controlled, and honest admin-versus-user
/// ownership disclosure must be enforced (proving absence of misrepresentation).
#[allow(clippy::too_many_arguments)]
pub fn managed_operation_is_accountable(
    operation: M5ManagedOperation,
    operation_target_root: &str,
    receipt_root: &str,
    failure_diagnostics_root: &str,
    receipt_fields_covered: &[M5ManagedReceiptField],
    ownership_misrepresented_used: bool,
    ownership_disclosure_enforced: bool,
) -> bool {
    managed_operation_receipt_is_complete(
        operation,
        operation_target_root,
        receipt_root,
        failure_diagnostics_root,
        receipt_fields_covered,
    ) && !ownership_misrepresented_used
        && ownership_disclosure_enforced
}

/// Whether the published policy-bootstrap injection discloses everything: the surface must be classified, every
/// mandatory injection field must be present, and the admin-versus-user ownership must be disclosed.
pub fn policy_injection_is_disclosed(
    surface: M5PolicyInjectionSurface,
    disclosed_fields: &[M5PolicyInjectionField],
    admin_control_disclosed: bool,
) -> bool {
    surface.is_classified()
        && injection_fields_cover_mandatory(disclosed_fields)
        && admin_control_disclosed
}

/// Whether channel-pin / update-deferral continuity stays documented: the posture must be classified and the
/// continuity note must be documented.
pub fn channel_pin_and_deferral_is_continuous(
    posture: M5ChannelDeferralPosture,
    pin_and_deferral_continuity_documented: bool,
) -> bool {
    posture.is_classified() && pin_and_deferral_continuity_documented
}

/// Resolves a managed-operation entry so it stays bound to the shared managed registry: the entry names its
/// canonical token, semantic role, and operation, covers all three presentation forms, inventories every
/// receipt field, keeps a disclosed ownership, and proves the managed installer was never presented as
/// user-controlled.
pub fn resolve_managed_operation_entry(
    input: M5ManagedOperationEntryResolutionInput,
) -> Result<M5ResolvedManagedOperationEntry, M5ManagedResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ManagedResolutionError::EmptyOperationEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.operation_target_root)
        || string_is_forbidden(&input.receipt_root)
        || string_is_forbidden(&input.failure_diagnostics_root)
    {
        return Err(M5ManagedResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let receipt_complete = managed_operation_receipt_is_complete(
        input.operation,
        &input.operation_target_root,
        &input.receipt_root,
        &input.failure_diagnostics_root,
        &input.receipt_fields_covered,
    );
    let is_accountable = managed_operation_is_accountable(
        input.operation,
        &input.operation_target_root,
        &input.receipt_root,
        &input.failure_diagnostics_root,
        &input.receipt_fields_covered,
        input.ownership_misrepresented_used,
        input.ownership_disclosure_enforced,
    );
    let misrepresentation_detected =
        input.ownership_misrepresented_used || !input.ownership_disclosure_enforced;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ManagedOperationEntryDegradeReason::OperationTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ManagedOperationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.operation.is_supported() {
        Some(M5ManagedOperationEntryDegradeReason::OperationUnclassified)
    } else if !input.bound_to_registry {
        Some(M5ManagedOperationEntryDegradeReason::OperationNotBoundToRegistry)
    } else if !receipt_complete {
        Some(M5ManagedOperationEntryDegradeReason::ManagedReceiptInventoryIncomplete)
    } else if misrepresentation_detected {
        Some(M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled)
    } else if !input.ownership.is_disclosed() {
        Some(M5ManagedOperationEntryDegradeReason::OwnershipAmbiguous)
    } else if !all_forms {
        Some(M5ManagedOperationEntryDegradeReason::PresentationFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5ManagedOperationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ManagedNextAction::ExpandManagedMeaning,
    };

    Ok(M5ResolvedManagedOperationEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        operation: input.operation.as_str().to_owned(),
        operation_is_supported: input.operation.is_supported(),
        surface_context: input.surface_context.as_str().to_owned(),
        operation_target_root: input.operation_target_root,
        receipt_root: input.receipt_root,
        failure_diagnostics_root: input.failure_diagnostics_root,
        receipt_fields_covered: input
            .receipt_fields_covered
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        ownership: input.ownership.as_str().to_owned(),
        ownership_is_disclosed: input.ownership.is_disclosed(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        managed_receipt_complete: receipt_complete,
        operation_is_accountable: is_accountable,
        bound_to_registry: input.bound_to_registry,
        ownership_misrepresented_used: input.ownership_misrepresented_used,
        ownership_disclosure_enforced: input.ownership_disclosure_enforced,
        degrade_reason,
        next_action,
        operation_resolves_across_profiles: degrade_reason.is_none(),
    })
}

/// Resolves a policy-bootstrap-injection entry so its injection stays discoverable and its channel-pin /
/// update-deferral continuity stays documented: the entry names its canonical token, semantic role, and
/// injection surface, covers all three presentation forms, discloses every mandatory injection field and the
/// admin ownership, and documents its channel-pin / update-deferral continuity.
pub fn resolve_policy_bootstrap_injection_entry(
    input: M5PolicyInjectionEntryResolutionInput,
) -> Result<M5ResolvedPolicyInjectionEntry, M5ManagedResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ManagedResolutionError::EmptyInjectionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.policy_bundle_source)
        || string_is_forbidden(&input.bootstrap_target)
    {
        return Err(M5ManagedResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let is_disclosed = policy_injection_is_disclosed(
        input.injection_surface,
        &input.disclosed_fields,
        input.admin_control_disclosed,
    ) && !input.policy_bundle_source.trim().is_empty()
        && !input.bootstrap_target.trim().is_empty();
    let is_continuous = channel_pin_and_deferral_is_continuous(
        input.pin_deferral_posture,
        input.pin_and_deferral_continuity_documented,
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PolicyInjectionEntryDegradeReason::InjectionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PolicyInjectionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.injection_surface.is_classified() {
        Some(M5PolicyInjectionEntryDegradeReason::InjectionSurfaceUnclassified)
    } else if !is_disclosed {
        Some(M5PolicyInjectionEntryDegradeReason::PolicyInjectionDisclosureIncomplete)
    } else if !is_continuous {
        Some(M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented)
    } else if !all_forms {
        Some(M5PolicyInjectionEntryDegradeReason::InjectionFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PolicyInjectionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ManagedNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPolicyInjectionEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        injection_surface: input.injection_surface.as_str().to_owned(),
        injection_surface_is_classified: input.injection_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        policy_bundle_source: input.policy_bundle_source,
        bootstrap_target: input.bootstrap_target,
        disclosed_fields: input
            .disclosed_fields
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        pin_deferral_posture: input.pin_deferral_posture.as_str().to_owned(),
        pin_deferral_posture_is_classified: input.pin_deferral_posture.is_classified(),
        pin_and_deferral_continuity_documented: input.pin_and_deferral_continuity_documented,
        admin_control_disclosed: input.admin_control_disclosed,
        injection_is_disclosed: is_disclosed,
        pin_and_deferral_is_continuous: is_continuous,
        degrade_reason,
        next_action,
        injection_discoverable_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved managed-operation and policy-injection entries
/// it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ManagedConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5InstallTopologyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5InstallTopologyDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5InstallTopologyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5InstallTopologyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ManagedAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ManagedExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    /// Resolved managed-operation examples.
    pub managed_operation_entries: Vec<M5ResolvedManagedOperationEntry>,
    /// Resolved policy-injection examples.
    pub policy_injection_entries: Vec<M5ResolvedPolicyInjectionEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the install-topology domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a managed installer is never presented as user-controlled. MUST be `false`.
    pub managed_installer_presented_as_user_controlled: bool,
    /// Hard invariant: a managed failure never strands the user without actionable diagnostics. MUST be `false`.
    pub managed_failure_stranded_user_without_diagnostics: bool,
    /// Hard invariant: channel pinning / repair-verify never drifts from the published matrix. MUST be `false`.
    pub channel_pinning_or_repair_verify_drifted_from_matrix: bool,
    /// Hard invariant: policy-bootstrap injection ownership is never left undisclosed. MUST be `false`.
    pub policy_bootstrap_injection_ownership_left_undisclosed: bool,
}

impl M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ManagedAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5ManagedAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ManagedExportField> = self.export_fields.iter().copied().collect();
        M5ManagedExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.managed_installer_presented_as_user_controlled
            && !self.managed_failure_stranded_user_without_diagnostics
            && !self.channel_pinning_or_repair_verify_drifted_from_matrix
            && !self.policy_bootstrap_injection_ownership_left_undisclosed
    }

    /// True when a clean operation entry preserves registry-bound truth: it traces to the registry, keeps a
    /// supported operation, inventories every receipt field, stays accountable (no misrepresentation), keeps a
    /// disclosed ownership, and covers all three presentation forms.
    fn operation_is_honest(ex: &M5ResolvedManagedOperationEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.operation_is_supported
                && ex.managed_receipt_complete
                && ex.operation_is_accountable
                && ex.ownership_is_disclosed
                && ex.covers_all_presentation_forms)
    }

    /// True when a clean injection entry preserves published truth: it keeps a classified surface, discloses
    /// everything, documents continuity, and covers all three presentation forms.
    fn injection_is_honest(ex: &M5ResolvedPolicyInjectionEntry) -> bool {
        !ex.is_clean()
            || (ex.injection_surface_is_classified
                && ex.injection_is_disclosed
                && ex.pin_and_deferral_is_continuous
                && ex.covers_all_presentation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.managed_operation_entries
            .iter()
            .all(Self::operation_is_honest)
            && self
                .policy_injection_entries
                .iter()
                .all(Self::injection_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Presentation-form tokens (minted by this lane).
    pub presentation_forms: Vec<String>,
    /// Managed-operation tokens (minted by this lane).
    pub managed_operations: Vec<String>,
    /// Receipt-field tokens (minted by this lane).
    pub receipt_fields: Vec<String>,
    /// Ownership tokens (minted by this lane).
    pub ownerships: Vec<String>,
    /// Injection-surface tokens (minted by this lane).
    pub injection_surfaces: Vec<String>,
    /// Injection-field tokens (minted by this lane).
    pub injection_fields: Vec<String>,
    /// Channel-pin / update-deferral posture tokens (minted by this lane).
    pub pin_deferral_postures: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Operation-entry degrade-reason tokens.
    pub operation_degrade_reasons: Vec<String>,
    /// Injection-entry degrade-reason tokens.
    pub injection_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5InstallTopologyRole::ALL, |v| v.as_str()),
            presentation_forms: tokens(&M5ManagedPresentationForm::ALL, |v| v.as_str()),
            managed_operations: tokens(&M5ManagedOperation::ALL, |v| v.as_str()),
            receipt_fields: tokens(&M5ManagedReceiptField::ALL, |v| v.as_str()),
            ownerships: tokens(&M5ManagedOwnership::ALL, |v| v.as_str()),
            injection_surfaces: tokens(&M5PolicyInjectionSurface::ALL, |v| v.as_str()),
            injection_fields: tokens(&M5PolicyInjectionField::ALL, |v| v.as_str()),
            pin_deferral_postures: tokens(&M5ChannelDeferralPosture::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ManagedSurfaceContext::ALL, |v| v.as_str()),
            operation_degrade_reasons: tokens(&M5ManagedOperationEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            injection_degrade_reasons: tokens(&M5PolicyInjectionEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ManagedAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ManagedNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ManagedExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5InstallTopologyConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionGovernanceReview {
    /// The managed registry names a canonical token, semantic role, and operation for every entry.
    pub managed_registry_names_token_role_and_operation: bool,
    /// Every claimed managed profile exposes silent install, uninstall, repair/verify, pinning, and deferral.
    pub profile_exposes_all_canonical_operations: bool,
    /// Every receipt field (install ID, timestamp, failure summary, repair/verify receipt) is copyable and
    /// published.
    pub all_receipt_fields_copyable_and_published: bool,
    /// A managed installer is never presented as user-controlled on any profile.
    pub managed_installer_never_presented_as_user_controlled: bool,
    /// Admin-owned versus user-owned responsibilities stay explicit and distinguishable.
    pub admin_versus_user_ownership_explicit: bool,
    /// Policy-bootstrap injection is published across the managed-channel, support, and docs surfaces.
    pub policy_injection_published_across_surfaces: bool,
    /// Every operation and injection entry covers the canonical / accessible / audit presentation forms.
    pub every_entry_covers_all_presentation_forms: bool,
    /// Channel-pin / update-deferral continuity is documented for every pinned or deferred posture.
    pub pin_and_deferral_continuity_documented: bool,
    /// Managed behavior stays bound to the shared registries rather than hand-copied per profile.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Installer, update, diagnostics, admin, docs, and support read a single managed-deployment source.
    pub installer_update_diagnostics_admin_read_single_source: bool,
    /// A misrepresented installer, an ambiguous ownership, or an injection drift is caught by fixtures before
    /// release evidence turns green.
    pub managed_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionConsumerProjection {
    /// The installer and update flows consume the shared managed registry.
    pub installer_and_update_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared managed registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// The updater service and policy-bootstrap channel consume the shared registries.
    pub updater_and_policy_channel_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical install-topology and state-root-boundary contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical managed-deployment registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting managed-operations audit for the lane.
    pub managed_operations_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 managed-deployment operations and policy-bootstrap-injection registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket {
    /// Record kind; must equal [`M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacketInput) -> Self {
        Self {
            record_kind:
                M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_RECORD_KIND
                    .to_owned(),
            schema_version:
                M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(
        &self,
    ) -> Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_RECORD_KIND
        {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_VERSION
        {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(&serde_json::to_value(self).expect(
            "m5 managed-deployment operations / policy-bootstrap-injection packet serializes",
        )) {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::RawMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect(
            "m5 managed-deployment operations / policy-bootstrap-injection packet serializes",
        )
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,managed_operation_entries,policy_injection_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .managed_operation_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.policy_injection_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.managed_operation_entries.len(),
                row.policy_injection_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Managed-Deployment Operations and Policy-Bootstrap-Injection Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Managed operations: {}\n",
            self.vocabulary_set.managed_operations.join(", ")
        ));
        out.push_str(&format!(
            "- Presentation forms: {}\n",
            self.vocabulary_set.presentation_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Operation entries: {} / injection entries: {}\n",
                row.managed_operation_entries.len(),
                row.policy_injection_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-profile managed-operation receipt reference table generated from the registry, so docs
    /// and support runbooks render the same operation / operation-target-root / receipt-root /
    /// failure-diagnostics-root / ownership truth the resolvers produced rather than a hand-copied path table.
    /// Only clean, registry-bound operation entries are listed.
    pub fn render_managed_operation_receipt_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| profile_id | operation | operation_target_root | receipt_root | failure_diagnostics_root | ownership |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.managed_operation_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | {} |\n",
                    ex.profile_id,
                    ex.operation,
                    ex.operation_target_root,
                    ex.receipt_root,
                    ex.failure_diagnostics_root,
                    ex.ownership
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>),
}

impl fmt::Display for M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 managed-deployment operations / policy-bootstrap-injection export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 managed-deployment operations / policy-bootstrap-injection export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError {}

/// Validation failures emitted by
/// [`M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at the install-topology domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, misrepresented, receipt-incomplete,
    /// ownership-ambiguous, or an injection entry missing a disclosure).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Managed-operation-contract is not proven: clean operation entries do not cover the canonical operations
    /// or the first installer / update / diagnostics / admin / support surfaces, no receipt-incomplete example
    /// degrades, or a clean operation entry published an incomplete receipt.
    ManagedOperationContractNotProven,
    /// Ownership-disclosure is not proven: no ownership-ambiguous example degrades, no clean disclosed operation
    /// entry is present, a clean operation entry is ambiguous, or clean injection entries do not cover the
    /// canonical injection surfaces with full presentation-form coverage while disclosed.
    OwnershipDisclosureNotProven,
    /// Drift-detection is not proven: no misrepresented-installer example degrades, a clean operation entry was
    /// misrepresented, no injection-disclosure-incomplete example degrades, or no pin-and-deferral-continuity-
    /// undocumented example degrades.
    DriftDetectionNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::ManagedOperationContractNotProven => "managed_operation_contract_not_proven",
            Self::OwnershipDisclosureNotProven => "ownership_disclosure_not_proven",
            Self::DriftDetectionNotProven => "drift_detection_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_managed_deployment_operations_and_policy_bootstrap_injection_export(
) -> Result<
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError,
> {
    let packet: M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/support_export.json"
        )))
        .map_err(M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_SCHEMA_REF,
        M5_MANAGED_DEPLOYMENT_OPERATIONS_AND_POLICY_BOOTSTRAP_INJECTION_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::NoRegistryRows,
        );
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF) {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DomainSchemaRefMissing,
            );
        }
        if row.managed_operation_entries.is_empty() || row.policy_injection_entries.is_empty() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.managed_registry_names_token_role_and_operation,
        review.profile_exposes_all_canonical_operations,
        review.all_receipt_fields_copyable_and_published,
        review.managed_installer_never_presented_as_user_controlled,
        review.admin_versus_user_ownership_explicit,
        review.policy_injection_published_across_surfaces,
        review.every_entry_covers_all_presentation_forms,
        review.pin_and_deferral_continuity_documented,
        review.behavior_bound_to_registry_not_hand_copied,
        review.installer_update_diagnostics_admin_read_single_source,
        review.managed_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.installer_and_update_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.updater_and_policy_channel_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.managed_operations_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
    violations: &mut Vec<M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation>,
) {
    let operations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.managed_operation_entries.iter())
    };
    let injections = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.policy_injection_entries.iter())
    };

    // AC1: claimed managed profiles expose install / uninstall / repair-verify / pinning / deferral through one
    // inspectable contract. Clean operation entries cover the canonical operations and the first installer /
    // update / diagnostics / admin / support surfaces, a receipt-incomplete example degrades, and no clean
    // operation entry published an incomplete receipt.
    let clean_operations: BTreeSet<String> = operations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.operation.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = operations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let operations_covered = M5ManagedOperation::CANONICAL_OPERATIONS
        .iter()
        .all(|o| clean_operations.contains(o.as_str()));
    let first_surfaces_covered = M5ManagedSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let receipt_incomplete_degrades = operations().any(|ex| {
        ex.degrade_reason
            == Some(M5ManagedOperationEntryDegradeReason::ManagedReceiptInventoryIncomplete)
    });
    let no_clean_incomplete = !operations().any(|ex| ex.is_clean() && !ex.managed_receipt_complete);
    if !(operations_covered
        && first_surfaces_covered
        && receipt_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::ManagedOperationContractNotProven,
        );
    }

    // AC2: managed-install failures do not strand the user in ambiguous ownership state. An ownership-ambiguous
    // example degrades, at least one clean disclosed operation entry is present, no clean operation entry is
    // ambiguous, and clean injection entries cover the canonical injection surfaces with full presentation-form
    // coverage while disclosed.
    let ownership_ambiguous_degrades = operations().any(|ex| {
        ex.degrade_reason == Some(M5ManagedOperationEntryDegradeReason::OwnershipAmbiguous)
    });
    let disclosed_clean_operation =
        operations().any(|ex| ex.is_clean() && ex.ownership_is_disclosed);
    let no_clean_ambiguous = !operations().any(|ex| ex.is_clean() && !ex.ownership_is_disclosed);
    let clean_injection_surfaces: BTreeSet<String> = injections()
        .filter(|ex| {
            ex.is_clean()
                && ex.injection_surface_is_classified
                && ex.injection_is_disclosed
                && ex.covers_all_presentation_forms
        })
        .map(|ex| ex.injection_surface.clone())
        .collect();
    let injection_surfaces_covered = M5PolicyInjectionSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_injection_surfaces.contains(s.as_str()));
    if !(ownership_ambiguous_degrades
        && disclosed_clean_operation
        && no_clean_ambiguous
        && injection_surfaces_covered)
    {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::OwnershipDisclosureNotProven,
        );
    }

    // AC3: enterprise rollout drills fail when bootstrap-policy injection, channel pinning, or repair/verify
    // semantics drift from the published matrix. A misrepresented-installer example degrades, no clean operation
    // entry was misrepresented, an injection-disclosure-incomplete example degrades, and a
    // pin-and-deferral-continuity-undocumented example degrades.
    let misrepresentation_degrades = operations().any(|ex| {
        ex.degrade_reason
            == Some(M5ManagedOperationEntryDegradeReason::ManagedInstallerPresentedAsUserControlled)
    });
    let no_clean_misrepresentation = !operations().any(|ex| {
        ex.is_clean() && (ex.ownership_misrepresented_used || !ex.ownership_disclosure_enforced)
    });
    let disclosure_incomplete_degrades = injections().any(|ex| {
        ex.degrade_reason
            == Some(M5PolicyInjectionEntryDegradeReason::PolicyInjectionDisclosureIncomplete)
    });
    let continuity_undocumented_degrades = injections().any(|ex| {
        ex.degrade_reason
            == Some(M5PolicyInjectionEntryDegradeReason::PinAndDeferralContinuityUndocumented)
    });
    if !(misrepresentation_degrades
        && no_clean_misrepresentation
        && disclosure_incomplete_degrades
        && continuity_undocumented_degrades)
    {
        violations.push(
            M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionViolation::DriftDetectionNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The install-topology families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5InstallTopologyFamily; 2] = [
    M5InstallTopologyFamily::PerUserManaged,
    M5InstallTopologyFamily::PerMachineManaged,
];
