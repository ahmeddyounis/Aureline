//! The M5 boundary inspector — the user / admin / evaluator-facing surface that explains, for a
//! consequential M5 action, *where execution and data went, which host / service hops carried it,
//! and which approval authority was in effect*, bound to the same gate-bound state grammar the
//! [governance matrix](crate::m5_assurance_route_governance) froze.
//!
//! High-risk M5 actions — sending a prompt to a remote provider, rotating a provider credential,
//! exporting workspace data, syncing to a managed control plane, pulling a model over a mirror,
//! pushing an admin policy, handing a diagnostic bundle to vendor support — should not require a
//! person or a support engineer to reconstruct *where work ran* or *who approved it* from scattered
//! task logs and badges. This lane is the one inspector that answers those questions for each action
//! with three reusable cards:
//!
//! - [`BoundarySummaryCard`]s. One per action. The card declares the [boundary class](BoundaryClass),
//!   the [actor / source](ActorClass), the [target class](TargetClass), the [sensitive data
//!   classes](SensitiveDataClass) that crossed, the [approving authority](ApprovalAuthority), and an
//!   export-safe one-line summary. Its active [capability-boundary state](CapabilityBoundaryState) is
//!   read from the matrix vocabulary, so a card can never read further within boundary than its proof.
//! - [`RouteHopTimeline`]s. One per action — an *ordered* list of [route hops](RouteHop), each naming
//!   its [locality](HopLocality), [role](HopRole), [certificate context](CertificateContext), and any
//!   [drift marker](HopDriftMarker). The timeline's [route state](RouteHopState) auto-narrows when a
//!   hop drifts (a mirror substitution, a certificate change) and blocks when a hop cannot be
//!   attributed, so the timeline never reads more attributable than its hops allow.
//! - [`ApprovalTicketInspector`]s. One per action — the [capability class](CapabilityClass), the
//!   approving authority, the scope, the [approval state](ApprovalState) drawn from the runtime
//!   authority vocabulary, the expiry and its [standing](ExpiryStanding), and the revoke / renew
//!   actions an operator can take. An expired ticket blocks; an expiring one narrows.
//!
//! Each action's three cards roll up into an [`ActionInspector`] whose effective gate is the *worst*
//! of the three, so the inspector never reads safer than its least-attested facet. The packet also
//! carries an [`InspectorEvaluationPacket`] export that reuses the exact boundary / route / approval
//! vocabulary the cards show, so an exported evaluation pack and the in-product inspector can never
//! drift. The [`M5BoundaryInspector`] packet is the one inspectable, serde-serializable truth record
//! this lane produces: it preserves route / evidence lineage as refs only and carries no credential
//! bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-boundary-inspector.schema.json`](../../../../../schemas/public-truth/m5-boundary-inspector.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-boundary-inspector-contract.md`](../../../../../docs/public-truth/m5-boundary-inspector-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_boundary_inspector, seeded_m5_boundary_inspector_approval_expired_blocked,
    seeded_m5_boundary_inspector_boundary_narrowed,
    seeded_m5_boundary_inspector_route_drift_narrowed,
    seeded_m5_boundary_inspector_route_unattributed_blocked, M5_BOUNDARY_INSPECTOR_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The boundary inspector reuses the governance matrix's frozen capability-boundary / route-hop /
// approval state vocabulary and the descriptor / badge gate runtime, so the in-product cards and the
// exported evaluation packet can never drift to a different state grammar.
use crate::m5_assurance_route_governance::{
    ApprovalState, CapabilityBoundaryState, EvidenceClass, RouteHopState, TrustBoundary,
};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5BoundaryInspector`].
pub const M5_BOUNDARY_INSPECTOR_RECORD_KIND: &str = "m5_boundary_inspector";

/// Record-kind tag carried by the embedded [`InspectorEvaluationPacket`].
pub const M5_BOUNDARY_INSPECTOR_EVALUATION_RECORD_KIND: &str =
    "m5_boundary_inspector_evaluation_packet";

/// Schema version for the boundary-inspector packet.
pub const M5_BOUNDARY_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary-inspector packet schema.
pub const M5_BOUNDARY_INSPECTOR_SCHEMA_REF: &str =
    "schemas/public-truth/m5-boundary-inspector.schema.json";

/// Repo-relative path of the published boundary-inspector inventory.
pub const M5_BOUNDARY_INSPECTOR_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-boundary-inspector.json";

/// Repo-relative path of the rendered boundary-inspector overview document.
pub const M5_BOUNDARY_INSPECTOR_OVERVIEW_REF: &str =
    "artifacts/public-truth/m5-boundary-inspector.md";

/// Repo-relative path of the machine-readable action / facet matrix export.
pub const M5_BOUNDARY_INSPECTOR_ACTIONS_CSV_REF: &str =
    "artifacts/public-truth/m5-boundary-inspector-actions.csv";

/// Repo-relative path of the release-grade boundary-inspector parity proof.
pub const M5_BOUNDARY_INSPECTOR_PROOF_REF: &str =
    "artifacts/public-truth/m5-boundary-inspector-proof/boundary-inspector.json";

/// Repo-relative path of the exported evaluation packet.
pub const M5_BOUNDARY_INSPECTOR_EVALUATION_PACKET_REF: &str =
    "artifacts/public-truth/m5-boundary-inspector-proof/evaluation-packet.json";

/// Repo-relative path of the boundary-inspector contract doc.
pub const M5_BOUNDARY_INSPECTOR_DOC_REF: &str =
    "docs/public-truth/m5-boundary-inspector-contract.md";

/// Repo-relative directory of the per-state boundary-inspector fixtures.
pub const M5_BOUNDARY_INSPECTOR_FIXTURE_DIR: &str = "fixtures/public-truth/m5-boundary-inspector/";

/// Prefix every boundary-inspector message id carries so consumers can route it.
pub const M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX: &str = "public_truth.boundary_inspector.";

/// Repo-relative proof ref backing the capability-boundary facet — drawn from the governance-matrix
/// proofs rather than a parallel evidence family.
const BOUNDARY_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json";

/// Repo-relative proof ref backing the route-hop facet.
const ROUTE_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/route-hop.json";

/// Repo-relative proof ref backing the approval-ticket facet.
const APPROVAL_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/approval-ticket.json";

/// Owner role accountable for keeping the capability-boundary facet current.
const BOUNDARY_OWNER_ROLE: &str = "capability_boundary_owner";

/// Owner role accountable for keeping the route-hop facet current.
const ROUTE_OWNER_ROLE: &str = "route_explainability_owner";

/// Owner role accountable for keeping the approval-ticket facet current.
const APPROVAL_OWNER_ROLE: &str = "runtime_authority_owner";

// ---------------------------------------------------------------------------------------------
// High-risk actions
// ---------------------------------------------------------------------------------------------

/// One consequential M5 action the inspector explains — the high-risk local / remote / provider /
/// admin operations whose execution boundary, route, and approval authority are worth inspecting.
/// The set invents no new runtime semantics; it names the existing local-first, remote-provider,
/// control-plane, mirror, and vendor operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskAction {
    /// A model runs locally over workspace content; work stays on the machine.
    LocalModelExecution,
    /// A prompt and context are sent to a remote provider for inference.
    RemoteModelInference,
    /// An admin rotates a provider credential through the control plane.
    ProviderCredentialRotation,
    /// Workspace data is exported to an external sink.
    WorkspaceDataExport,
    /// Workspace metadata is synced to the managed control plane.
    ControlPlaneSync,
    /// A model artifact is acquired over an offline mirror.
    OfflineModelAcquisition,
    /// An admin pushes a runtime policy to the control plane.
    AdminPolicyPush,
    /// A diagnostic bundle is handed off to vendor support.
    SupportBundleHandoff,
}

impl HighRiskAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LocalModelExecution,
        Self::RemoteModelInference,
        Self::ProviderCredentialRotation,
        Self::WorkspaceDataExport,
        Self::ControlPlaneSync,
        Self::OfflineModelAcquisition,
        Self::AdminPolicyPush,
        Self::SupportBundleHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModelExecution => "local_model_execution",
            Self::RemoteModelInference => "remote_model_inference",
            Self::ProviderCredentialRotation => "provider_credential_rotation",
            Self::WorkspaceDataExport => "workspace_data_export",
            Self::ControlPlaneSync => "control_plane_sync",
            Self::OfflineModelAcquisition => "offline_model_acquisition",
            Self::AdminPolicyPush => "admin_policy_push",
            Self::SupportBundleHandoff => "support_bundle_handoff",
        }
    }

    /// Reader-facing action label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalModelExecution => "Local model execution",
            Self::RemoteModelInference => "Remote model inference",
            Self::ProviderCredentialRotation => "Provider credential rotation",
            Self::WorkspaceDataExport => "Workspace data export",
            Self::ControlPlaneSync => "Control-plane sync",
            Self::OfflineModelAcquisition => "Offline model acquisition",
            Self::AdminPolicyPush => "Admin policy push",
            Self::SupportBundleHandoff => "Support bundle handoff",
        }
    }

    /// The execution / data boundary class this action crosses.
    pub const fn boundary_class(self) -> BoundaryClass {
        match self {
            Self::LocalModelExecution => BoundaryClass::LocalExecution,
            Self::RemoteModelInference
            | Self::WorkspaceDataExport
            | Self::OfflineModelAcquisition => BoundaryClass::LocalToRemoteProvider,
            Self::ProviderCredentialRotation | Self::ControlPlaneSync | Self::AdminPolicyPush => {
                BoundaryClass::LocalToControlPlane
            }
            Self::SupportBundleHandoff => BoundaryClass::VendorHandoff,
        }
    }

    /// Who initiated the action.
    pub const fn actor(self) -> ActorClass {
        match self {
            Self::LocalModelExecution | Self::RemoteModelInference | Self::WorkspaceDataExport => {
                ActorClass::LocalUser
            }
            Self::ProviderCredentialRotation | Self::AdminPolicyPush => ActorClass::WorkspaceAdmin,
            Self::ControlPlaneSync | Self::OfflineModelAcquisition => ActorClass::AutomationAgent,
            Self::SupportBundleHandoff => ActorClass::SupportEngineer,
        }
    }

    /// The class of target the action reached.
    pub const fn target_class(self) -> TargetClass {
        match self {
            Self::LocalModelExecution => TargetClass::LocalMachine,
            Self::RemoteModelInference | Self::WorkspaceDataExport => TargetClass::RemoteProvider,
            Self::ProviderCredentialRotation | Self::ControlPlaneSync | Self::AdminPolicyPush => {
                TargetClass::ManagedControlPlane
            }
            Self::OfflineModelAcquisition => TargetClass::MirrorRegistry,
            Self::SupportBundleHandoff => TargetClass::VendorSupport,
        }
    }

    /// The sensitive data classes that crossed the boundary, in canonical order. These are *category*
    /// labels, never the data itself.
    pub fn sensitive_data_classes(self) -> Vec<SensitiveDataClass> {
        let mut classes = match self {
            Self::LocalModelExecution => {
                vec![
                    SensitiveDataClass::SourceContent,
                    SensitiveDataClass::PromptContext,
                ]
            }
            Self::RemoteModelInference => {
                vec![
                    SensitiveDataClass::PromptContext,
                    SensitiveDataClass::SourceContent,
                ]
            }
            Self::ProviderCredentialRotation => vec![
                SensitiveDataClass::CredentialReference,
                SensitiveDataClass::WorkspaceMetadata,
            ],
            Self::WorkspaceDataExport => vec![
                SensitiveDataClass::SourceContent,
                SensitiveDataClass::WorkspaceMetadata,
            ],
            Self::ControlPlaneSync => vec![SensitiveDataClass::WorkspaceMetadata],
            Self::OfflineModelAcquisition => vec![SensitiveDataClass::ModelArtifact],
            Self::AdminPolicyPush => vec![SensitiveDataClass::WorkspaceMetadata],
            Self::SupportBundleHandoff => vec![SensitiveDataClass::DiagnosticBundle],
        };
        classes.sort_by_key(|c| data_class_rank(*c));
        classes.dedup();
        classes
    }

    /// The capability class the action's approval ticket grants.
    pub const fn capability_class(self) -> CapabilityClass {
        match self {
            Self::LocalModelExecution => CapabilityClass::LocalInference,
            Self::RemoteModelInference => CapabilityClass::RemoteInference,
            Self::ProviderCredentialRotation => CapabilityClass::CredentialManagement,
            Self::WorkspaceDataExport | Self::ControlPlaneSync => CapabilityClass::DataEgress,
            Self::OfflineModelAcquisition => CapabilityClass::ModelAcquisition,
            Self::AdminPolicyPush => CapabilityClass::PolicyAdministration,
            Self::SupportBundleHandoff => CapabilityClass::SupportDisclosure,
        }
    }

    /// The authority that grants the action — the same authority vocabulary the runtime objects use.
    pub const fn approving_authority(self) -> ApprovalAuthority {
        match self {
            Self::LocalModelExecution | Self::ControlPlaneSync => ApprovalAuthority::StandingPolicy,
            Self::RemoteModelInference | Self::WorkspaceDataExport | Self::SupportBundleHandoff => {
                ApprovalAuthority::UserConsent
            }
            Self::ProviderCredentialRotation => ApprovalAuthority::SecurityOfficer,
            Self::OfflineModelAcquisition => ApprovalAuthority::RuntimeBroker,
            Self::AdminPolicyPush => ApprovalAuthority::WorkspaceAdmin,
        }
    }

    /// Owner role accountable for keeping this action's inspector current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ProviderCredentialRotation | Self::AdminPolicyPush => BOUNDARY_OWNER_ROLE,
            _ => ROUTE_OWNER_ROLE,
        }
    }

    /// An export-safe one-line summary of the boundary the action crosses (no secrets).
    pub const fn export_safe_summary(self) -> &'static str {
        match self {
            Self::LocalModelExecution => {
                "Runs a model on the local machine; no data leaves the device."
            }
            Self::RemoteModelInference => {
                "Sends prompt and context to a named remote provider over a pinned route."
            }
            Self::ProviderCredentialRotation => {
                "Rotates a provider credential reference through the managed control plane."
            }
            Self::WorkspaceDataExport => {
                "Exports selected workspace content to an external sink under user consent."
            }
            Self::ControlPlaneSync => {
                "Syncs workspace metadata to the managed control plane under standing policy."
            }
            Self::OfflineModelAcquisition => {
                "Pulls a model artifact from an attributed mirror back to the local machine."
            }
            Self::AdminPolicyPush => {
                "Pushes a runtime policy bundle to the control plane under admin authority."
            }
            Self::SupportBundleHandoff => {
                "Hands a redacted diagnostic bundle to vendor support under user consent."
            }
        }
    }

    /// A short scope statement the approval ticket carries (no secrets).
    pub const fn scope_summary(self) -> &'static str {
        match self {
            Self::LocalModelExecution => "Local inference over the active workspace only.",
            Self::RemoteModelInference => "One inference request to the selected provider model.",
            Self::ProviderCredentialRotation => {
                "Rotate the named provider credential; no credential body exposed."
            }
            Self::WorkspaceDataExport => {
                "Export the selected files to the chosen destination once."
            }
            Self::ControlPlaneSync => "Sync workspace metadata for the active deployment only.",
            Self::OfflineModelAcquisition => {
                "Acquire the requested model from the configured mirror."
            }
            Self::AdminPolicyPush => "Publish the policy bundle to the active deployment.",
            Self::SupportBundleHandoff => {
                "Disclose the redacted diagnostic bundle to vendor support."
            }
        }
    }

    /// A repo-relative governance-ticket ref for the action's approval (refs only, no secrets).
    pub const fn ticket_ref(self) -> &'static str {
        match self {
            Self::LocalModelExecution => "governance-ticket://approval/local-model-execution",
            Self::RemoteModelInference => "governance-ticket://approval/remote-model-inference",
            Self::ProviderCredentialRotation => {
                "governance-ticket://approval/provider-credential-rotation"
            }
            Self::WorkspaceDataExport => "governance-ticket://approval/workspace-data-export",
            Self::ControlPlaneSync => "governance-ticket://approval/control-plane-sync",
            Self::OfflineModelAcquisition => {
                "governance-ticket://approval/offline-model-acquisition"
            }
            Self::AdminPolicyPush => "governance-ticket://approval/admin-policy-push",
            Self::SupportBundleHandoff => "governance-ticket://approval/support-bundle-handoff",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Boundary vocabulary
// ---------------------------------------------------------------------------------------------

/// The execution / data boundary class an action crosses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// Work stays on the local machine.
    LocalExecution,
    /// Work crosses from local to a remote provider.
    LocalToRemoteProvider,
    /// Work crosses from local to the managed control plane.
    LocalToControlPlane,
    /// Work is handed off to the vendor.
    VendorHandoff,
}

impl BoundaryClass {
    /// Every boundary class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalExecution,
        Self::LocalToRemoteProvider,
        Self::LocalToControlPlane,
        Self::VendorHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalExecution => "local_execution",
            Self::LocalToRemoteProvider => "local_to_remote_provider",
            Self::LocalToControlPlane => "local_to_control_plane",
            Self::VendorHandoff => "vendor_handoff",
        }
    }

    /// Reader-facing boundary-class label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalExecution => "Local execution",
            Self::LocalToRemoteProvider => "Local → remote provider",
            Self::LocalToControlPlane => "Local → control plane",
            Self::VendorHandoff => "Vendor handoff",
        }
    }

    /// The trust boundaries this class spans.
    pub fn trust_boundaries(self) -> Vec<TrustBoundary> {
        match self {
            Self::LocalExecution => vec![TrustBoundary::LocalFirst],
            Self::LocalToRemoteProvider | Self::LocalToControlPlane | Self::VendorHandoff => {
                vec![TrustBoundary::LocalFirst, TrustBoundary::ControlPlane]
            }
        }
    }

    /// True when the action leaves the local-first trust boundary.
    pub const fn crosses_trust_boundary(self) -> bool {
        !matches!(self, Self::LocalExecution)
    }
}

/// Who initiated an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    /// The local workspace user.
    LocalUser,
    /// The workspace / deployment admin.
    WorkspaceAdmin,
    /// An automated runtime agent.
    AutomationAgent,
    /// A support engineer.
    SupportEngineer,
}

impl ActorClass {
    /// Every actor class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalUser,
        Self::WorkspaceAdmin,
        Self::AutomationAgent,
        Self::SupportEngineer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::WorkspaceAdmin => "workspace_admin",
            Self::AutomationAgent => "automation_agent",
            Self::SupportEngineer => "support_engineer",
        }
    }

    /// Reader-facing actor label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalUser => "Local user",
            Self::WorkspaceAdmin => "Workspace admin",
            Self::AutomationAgent => "Automation agent",
            Self::SupportEngineer => "Support engineer",
        }
    }

    /// The locality the actor initiates from — always the local machine.
    pub const fn source_locality(self) -> HopLocality {
        HopLocality::LocalMachine
    }
}

/// The class of target an action reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
    /// The local machine.
    LocalMachine,
    /// A remote provider.
    RemoteProvider,
    /// The managed control plane.
    ManagedControlPlane,
    /// A mirror registry.
    MirrorRegistry,
    /// Vendor support.
    VendorSupport,
}

impl TargetClass {
    /// Every target class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalMachine,
        Self::RemoteProvider,
        Self::ManagedControlPlane,
        Self::MirrorRegistry,
        Self::VendorSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalMachine => "local_machine",
            Self::RemoteProvider => "remote_provider",
            Self::ManagedControlPlane => "managed_control_plane",
            Self::MirrorRegistry => "mirror_registry",
            Self::VendorSupport => "vendor_support",
        }
    }

    /// Reader-facing target label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalMachine => "Local machine",
            Self::RemoteProvider => "Remote provider",
            Self::ManagedControlPlane => "Managed control plane",
            Self::MirrorRegistry => "Mirror registry",
            Self::VendorSupport => "Vendor support",
        }
    }
}

/// A category of sensitive data that crossed a boundary. These are category labels, never the data
/// itself — the inspector never captures the bytes that crossed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitiveDataClass {
    /// Source / workspace content.
    SourceContent,
    /// Prompt / model context.
    PromptContext,
    /// Workspace metadata.
    WorkspaceMetadata,
    /// A model artifact.
    ModelArtifact,
    /// A redacted diagnostic bundle.
    DiagnosticBundle,
    /// A reference to a credential — the handle, never the credential body.
    CredentialReference,
}

impl SensitiveDataClass {
    /// Every data class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceContent,
        Self::PromptContext,
        Self::WorkspaceMetadata,
        Self::ModelArtifact,
        Self::DiagnosticBundle,
        Self::CredentialReference,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceContent => "source_content",
            Self::PromptContext => "prompt_context",
            Self::WorkspaceMetadata => "workspace_metadata",
            Self::ModelArtifact => "model_artifact",
            Self::DiagnosticBundle => "diagnostic_bundle",
            Self::CredentialReference => "credential_reference",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Route-hop vocabulary
// ---------------------------------------------------------------------------------------------

/// The locality of one route hop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HopLocality {
    /// The local machine.
    LocalMachine,
    /// The local network / LAN.
    LocalNetwork,
    /// A remote provider region.
    RemoteRegion,
    /// The managed control plane.
    ControlPlane,
    /// A mirror edge.
    MirrorEdge,
    /// A vendor edge.
    VendorEdge,
}

impl HopLocality {
    /// Every locality, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalMachine,
        Self::LocalNetwork,
        Self::RemoteRegion,
        Self::ControlPlane,
        Self::MirrorEdge,
        Self::VendorEdge,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalMachine => "local_machine",
            Self::LocalNetwork => "local_network",
            Self::RemoteRegion => "remote_region",
            Self::ControlPlane => "control_plane",
            Self::MirrorEdge => "mirror_edge",
            Self::VendorEdge => "vendor_edge",
        }
    }

    /// Reader-facing locality label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalMachine => "Local machine",
            Self::LocalNetwork => "Local network",
            Self::RemoteRegion => "Remote region",
            Self::ControlPlane => "Control plane",
            Self::MirrorEdge => "Mirror edge",
            Self::VendorEdge => "Vendor edge",
        }
    }

    /// True when the hop stays on the local-first side of the trust boundary.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::LocalMachine | Self::LocalNetwork)
    }
}

/// The role a hop plays on the route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HopRole {
    /// The origin of the route.
    Origin,
    /// A forwarding proxy.
    Proxy,
    /// A mirror that serves cached content.
    Mirror,
    /// The route target.
    Target,
}

impl HopRole {
    /// Every role, in declaration order.
    pub const ALL: [Self; 4] = [Self::Origin, Self::Proxy, Self::Mirror, Self::Target];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Proxy => "proxy",
            Self::Mirror => "mirror",
            Self::Target => "target",
        }
    }
}

/// The certificate / transport-trust context of one hop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificateContext {
    /// No transport security needed — the hop stays on-device.
    NoTlsLocal,
    /// Local trust anchor.
    LocalTrust,
    /// A pinned certificate.
    PinnedCertificate,
    /// A mirror certificate.
    MirrorCertificate,
    /// A control-plane certificate.
    ControlPlaneCertificate,
}

impl CertificateContext {
    /// Every certificate context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoTlsLocal,
        Self::LocalTrust,
        Self::PinnedCertificate,
        Self::MirrorCertificate,
        Self::ControlPlaneCertificate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTlsLocal => "no_tls_local",
            Self::LocalTrust => "local_trust",
            Self::PinnedCertificate => "pinned_certificate",
            Self::MirrorCertificate => "mirror_certificate",
            Self::ControlPlaneCertificate => "control_plane_certificate",
        }
    }
}

/// A drift marker on a route hop — a deviation from the expected route that narrows or blocks
/// attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HopDriftMarker {
    /// No drift; the hop matched the expected route.
    None,
    /// The hop reached an unexpected locality.
    LocalityDrift,
    /// The hop's certificate changed from the pinned one.
    CertificateDrift,
    /// A mirror silently replaced the named target.
    MirrorSubstitution,
    /// The hop cannot be attributed at all.
    UnattributedHop,
}

impl HopDriftMarker {
    /// Every drift marker, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::LocalityDrift,
        Self::CertificateDrift,
        Self::MirrorSubstitution,
        Self::UnattributedHop,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::LocalityDrift => "locality_drift",
            Self::CertificateDrift => "certificate_drift",
            Self::MirrorSubstitution => "mirror_substitution",
            Self::UnattributedHop => "unattributed_hop",
        }
    }

    /// The gate posture this drift marker imposes on its hop: none keeps it governed, a documented
    /// drift narrows, an unattributed hop blocks.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::None => DescriptorGate::Governed,
            Self::LocalityDrift | Self::CertificateDrift | Self::MirrorSubstitution => {
                DescriptorGate::Narrowed
            }
            Self::UnattributedHop => DescriptorGate::Blocked,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Approval vocabulary
// ---------------------------------------------------------------------------------------------

/// The capability class an approval ticket grants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClass {
    /// Run a model locally.
    LocalInference,
    /// Send work to a remote provider.
    RemoteInference,
    /// Manage a provider credential.
    CredentialManagement,
    /// Egress data beyond the local boundary.
    DataEgress,
    /// Administer runtime policy.
    PolicyAdministration,
    /// Acquire a model artifact.
    ModelAcquisition,
    /// Disclose a diagnostic bundle to support.
    SupportDisclosure,
}

impl CapabilityClass {
    /// Every capability class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalInference,
        Self::RemoteInference,
        Self::CredentialManagement,
        Self::DataEgress,
        Self::PolicyAdministration,
        Self::ModelAcquisition,
        Self::SupportDisclosure,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInference => "local_inference",
            Self::RemoteInference => "remote_inference",
            Self::CredentialManagement => "credential_management",
            Self::DataEgress => "data_egress",
            Self::PolicyAdministration => "policy_administration",
            Self::ModelAcquisition => "model_acquisition",
            Self::SupportDisclosure => "support_disclosure",
        }
    }

    /// Reader-facing capability label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalInference => "Local inference",
            Self::RemoteInference => "Remote inference",
            Self::CredentialManagement => "Credential management",
            Self::DataEgress => "Data egress",
            Self::PolicyAdministration => "Policy administration",
            Self::ModelAcquisition => "Model acquisition",
            Self::SupportDisclosure => "Support disclosure",
        }
    }
}

/// The authority that grants an approval — the same authority vocabulary the runtime objects use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAuthority {
    /// A standing runtime policy.
    StandingPolicy,
    /// Explicit user consent.
    UserConsent,
    /// The workspace / deployment admin.
    WorkspaceAdmin,
    /// A security officer.
    SecurityOfficer,
    /// An automated runtime broker.
    RuntimeBroker,
}

impl ApprovalAuthority {
    /// Every authority, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StandingPolicy,
        Self::UserConsent,
        Self::WorkspaceAdmin,
        Self::SecurityOfficer,
        Self::RuntimeBroker,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandingPolicy => "standing_policy",
            Self::UserConsent => "user_consent",
            Self::WorkspaceAdmin => "workspace_admin",
            Self::SecurityOfficer => "security_officer",
            Self::RuntimeBroker => "runtime_broker",
        }
    }

    /// Reader-facing authority label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StandingPolicy => "Standing policy",
            Self::UserConsent => "User consent",
            Self::WorkspaceAdmin => "Workspace admin",
            Self::SecurityOfficer => "Security officer",
            Self::RuntimeBroker => "Runtime broker",
        }
    }
}

/// The standing of an approval ticket's expiry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryStanding {
    /// The approval is in date.
    Active,
    /// The approval expires soon; it narrows.
    ExpiringSoon,
    /// The approval has expired; it blocks.
    Expired,
}

impl ExpiryStanding {
    /// Every standing, in declaration order.
    pub const ALL: [Self; 3] = [Self::Active, Self::ExpiringSoon, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
        }
    }

    /// The gate posture this standing imposes: active keeps the ticket governed, expiring narrows,
    /// expired blocks.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Active => DescriptorGate::Governed,
            Self::ExpiringSoon => DescriptorGate::Narrowed,
            Self::Expired => DescriptorGate::Blocked,
        }
    }
}

/// A revoke / renew action an operator can take on an approval ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketAction {
    /// Revoke the approval immediately.
    RevokeApproval,
    /// Renew the approval for another window.
    RenewApproval,
    /// Require a fresh approval before the action may proceed.
    RequireReapproval,
    /// Tighten the granted scope.
    TightenScope,
}

impl TicketAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevokeApproval,
        Self::RenewApproval,
        Self::RequireReapproval,
        Self::TightenScope,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevokeApproval => "revoke_approval",
            Self::RenewApproval => "renew_approval",
            Self::RequireReapproval => "require_reapproval",
            Self::TightenScope => "tighten_scope",
        }
    }
}

/// An evaluation / export action an inspector offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationAction {
    /// Inspect the boundary summary card.
    InspectBoundary,
    /// Trace the route-hop timeline.
    TraceRoute,
    /// Review the approval ticket.
    ReviewApproval,
    /// Export the inspector evaluation packet.
    ExportInspectorPacket,
}

impl EvaluationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectBoundary,
        Self::TraceRoute,
        Self::ReviewApproval,
        Self::ExportInspectorPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectBoundary => "inspect_boundary",
            Self::TraceRoute => "trace_route",
            Self::ReviewApproval => "review_approval",
            Self::ExportInspectorPacket => "export_inspector_packet",
        }
    }
}

/// The facet of an inspector a gap applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorFacet {
    /// The boundary summary card.
    Boundary,
    /// The route-hop timeline.
    Route,
    /// The approval-ticket inspector.
    Approval,
}

impl InspectorFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 3] = [Self::Boundary, Self::Route, Self::Approval];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Boundary => "boundary",
            Self::Route => "route",
            Self::Approval => "approval",
        }
    }
}

/// The kind of drift a facet inflicts on an inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorGapKind {
    /// The facet narrowed the inspector below fully governed.
    FacetNarrowed,
    /// The facet blocked the inspector.
    FacetBlocked,
}

impl InspectorGapKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetNarrowed => "facet_narrowed",
            Self::FacetBlocked => "facet_blocked",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate / posture helpers
// ---------------------------------------------------------------------------------------------

/// Maps a gate posture to the qualification floor it implies: governed stands at Stable, narrowed
/// floors at Beta, blocked at Unavailable.
const fn floor_for_gate(gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// Restrictiveness rank of a gate posture (least restrictive first).
const fn gate_rank(gate: DescriptorGate) -> usize {
    match gate {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

/// The more restrictive of two gate postures.
const fn worse_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

/// The gate posture an evidence freshness implies on its own: current keeps it governed, stale
/// narrows, expired / missing block.
const fn freshness_gate(freshness: FreshnessState) -> DescriptorGate {
    match freshness {
        FreshnessState::Current => DescriptorGate::Governed,
        FreshnessState::Stale => DescriptorGate::Narrowed,
        FreshnessState::Expired | FreshnessState::Missing => DescriptorGate::Blocked,
    }
}

/// Maps a gate posture to the coverage status it implies.
const fn gate_status(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// Maps a gate posture to the inspector-gap kind it implies, when not governed.
const fn gap_kind_for_gate(gate: DescriptorGate) -> Option<InspectorGapKind> {
    match gate {
        DescriptorGate::Governed => None,
        DescriptorGate::Narrowed => Some(InspectorGapKind::FacetNarrowed),
        DescriptorGate::Blocked => Some(InspectorGapKind::FacetBlocked),
    }
}

// ---------------------------------------------------------------------------------------------
// Boundary summary card
// ---------------------------------------------------------------------------------------------

/// One boundary summary card: for one action, the boundary class it crosses, who initiated it, the
/// target class, the sensitive data classes that crossed, the approving authority, and an export-safe
/// summary. Its active state is read from the matrix [capability-boundary
/// vocabulary](CapabilityBoundaryState), and the effective gate folds in evidence freshness so the
/// card can never read further within boundary than its proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundarySummaryCard {
    /// The action this card describes.
    pub action: HighRiskAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The boundary class the action crosses.
    pub boundary_class: BoundaryClass,
    /// Reader-facing boundary-class label.
    pub boundary_class_label: String,
    /// Who initiated the action.
    pub actor: ActorClass,
    /// Reader-facing actor label.
    pub actor_label: String,
    /// The locality the actor initiated from.
    pub source_locality: HopLocality,
    /// The class of target the action reached.
    pub target_class: TargetClass,
    /// Reader-facing target label.
    pub target_class_label: String,
    /// The sensitive data classes that crossed the boundary, in canonical order.
    pub sensitive_data_classes: Vec<SensitiveDataClass>,
    /// The authority that granted the action.
    pub approval_authority: ApprovalAuthority,
    /// The active capability-boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// Reader-facing boundary-state label.
    pub boundary_state_label: String,
    /// Freshness of the boundary evidence.
    pub evidence_freshness: FreshnessState,
    /// The gate the boundary state and freshness together imply (the more restrictive of the two).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The trust boundaries the action spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// True when the action leaves the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The evidence class backing the boundary card.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the boundary proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the boundary card.
    pub proof_ref: String,
    /// An export-safe one-line summary (no secrets).
    pub export_safe_summary: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl BoundarySummaryCard {
    /// Derives a boundary summary card from the action, its boundary state, and evidence freshness.
    fn derive(
        action: HighRiskAction,
        boundary_state: CapabilityBoundaryState,
        evidence_freshness: FreshnessState,
    ) -> Self {
        let boundary_class = action.boundary_class();
        let effective_gate = worse_gate(
            boundary_state.gate_posture(),
            freshness_gate(evidence_freshness),
        );
        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            boundary_class,
            boundary_class_label: boundary_class.label().to_owned(),
            actor: action.actor(),
            actor_label: action.actor().label().to_owned(),
            source_locality: action.actor().source_locality(),
            target_class: action.target_class(),
            target_class_label: action.target_class().label().to_owned(),
            sensitive_data_classes: action.sensitive_data_classes(),
            approval_authority: action.approving_authority(),
            boundary_state,
            boundary_state_label: boundary_state.label().to_owned(),
            evidence_freshness,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            trust_boundaries: boundary_class.trust_boundaries(),
            crosses_trust_boundary: boundary_class.crosses_trust_boundary(),
            evidence_class: EvidenceClass::BoundaryManifest,
            owner_role: BOUNDARY_OWNER_ROLE.to_owned(),
            proof_ref: BOUNDARY_PROOF_REF.to_owned(),
            export_safe_summary: action.export_safe_summary().to_owned(),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}boundary.{}",
                M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// Validates the card's invariants: every derived field matches the action, the effective gate
    /// matches the boundary state and freshness, the status mirrors the gate, and the message id
    /// carries the lane prefix.
    fn validate(&self) -> Vec<M5BoundaryInspectorViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(self.action, self.boundary_state, self.evidence_freshness);
        if probe != *self {
            out.push(M5BoundaryInspectorViolation::BoundaryCardDrift);
        }
        let expected_gate = worse_gate(
            self.boundary_state.gate_posture(),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5BoundaryInspectorViolation::BoundaryGateDrift);
        }
        if self.sensitive_data_classes.is_empty()
            || self.proof_ref.trim().is_empty()
            || self.export_safe_summary.trim().is_empty()
        {
            out.push(M5BoundaryInspectorViolation::BoundaryDisclosureIncomplete);
        }
        if !self
            .detail_message_id
            .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
        {
            out.push(M5BoundaryInspectorViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Route-hop timeline
// ---------------------------------------------------------------------------------------------

/// Seed input for one route hop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteHopSeed {
    /// The hop's locality.
    pub locality: HopLocality,
    /// The hop's role on the route.
    pub role: HopRole,
    /// The hop's certificate context.
    pub certificate_context: CertificateContext,
    /// Any drift marker on the hop.
    pub drift_marker: HopDriftMarker,
}

/// One ordered route hop: its index, locality, role, certificate context, and any drift marker, plus
/// the gate the drift marker imposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHop {
    /// Zero-based hop index in route order.
    pub index: u32,
    /// The hop's locality.
    pub locality: HopLocality,
    /// Reader-facing locality label.
    pub locality_label: String,
    /// The hop's role on the route.
    pub role: HopRole,
    /// The hop's certificate context.
    pub certificate_context: CertificateContext,
    /// Whether the hop stays on the local-first side of the trust boundary.
    pub is_local: bool,
    /// Any drift marker on the hop.
    pub drift_marker: HopDriftMarker,
    /// The gate posture this hop's drift marker imposes.
    pub hop_gate: DescriptorGate,
    /// Stable message id; prefixed [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RouteHop {
    /// Builds a route hop from its seed at the given index and action.
    fn from_seed(action: HighRiskAction, index: u32, seed: RouteHopSeed) -> Self {
        Self {
            index,
            locality: seed.locality,
            locality_label: seed.locality.label().to_owned(),
            role: seed.role,
            certificate_context: seed.certificate_context,
            is_local: seed.locality.is_local(),
            drift_marker: seed.drift_marker,
            hop_gate: seed.drift_marker.gate_posture(),
            detail_message_id: format!(
                "{}route.{}.hop.{}",
                M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                action.as_str(),
                index
            ),
        }
    }
}

/// One route-hop timeline: for one action, the canonical [route state](RouteHopState), the ordered
/// hops, and the effective gate that folds in any hop drift and evidence freshness, so the timeline
/// never reads more attributable than its hops allow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHopTimeline {
    /// The action this timeline describes.
    pub action: HighRiskAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The canonical route-hop state for the whole route.
    pub route_state: RouteHopState,
    /// Reader-facing route-state label.
    pub route_state_label: String,
    /// The ordered route hops.
    pub hops: Vec<RouteHop>,
    /// The locality of the first hop.
    pub origin_locality: HopLocality,
    /// The locality of the last hop.
    pub final_locality: HopLocality,
    /// True when the route leaves the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// Count of hops carrying a drift marker.
    pub drift_marker_count: u32,
    /// Freshness of the route evidence.
    pub evidence_freshness: FreshnessState,
    /// The gate the route state, hop drift, and freshness together imply (the most restrictive).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The evidence class backing the route timeline.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the route proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the route timeline.
    pub proof_ref: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub timeline_message_id: String,
}

impl RouteHopTimeline {
    /// Derives a route-hop timeline from the action, its route state, the ordered hop seeds, and the
    /// evidence freshness. The effective gate is the worst of the route state's gate, the worst hop
    /// drift gate, and the freshness gate.
    fn derive(
        action: HighRiskAction,
        route_state: RouteHopState,
        hop_seeds: &[RouteHopSeed],
        evidence_freshness: FreshnessState,
    ) -> Self {
        let hops: Vec<RouteHop> = hop_seeds
            .iter()
            .enumerate()
            .map(|(i, seed)| RouteHop::from_seed(action, i as u32, *seed))
            .collect();
        let worst_drift_gate = hops
            .iter()
            .map(|h| h.hop_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        let effective_gate = worse_gate(
            worse_gate(route_state.gate_posture(), worst_drift_gate),
            freshness_gate(evidence_freshness),
        );
        let status = gate_status(effective_gate);
        let origin_locality = hops
            .first()
            .map(|h| h.locality)
            .unwrap_or(HopLocality::LocalMachine);
        let final_locality = hops
            .last()
            .map(|h| h.locality)
            .unwrap_or(HopLocality::LocalMachine);
        let crosses_trust_boundary = hops.iter().any(|h| !h.is_local);
        let drift_marker_count = hops
            .iter()
            .filter(|h| !matches!(h.drift_marker, HopDriftMarker::None))
            .count() as u32;
        Self {
            action,
            action_label: action.label().to_owned(),
            route_state,
            route_state_label: route_state.label().to_owned(),
            hops,
            origin_locality,
            final_locality,
            crosses_trust_boundary,
            drift_marker_count,
            evidence_freshness,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            evidence_class: EvidenceClass::RouteTimeline,
            owner_role: ROUTE_OWNER_ROLE.to_owned(),
            proof_ref: ROUTE_PROOF_REF.to_owned(),
            status,
            signal: status.signal(),
            timeline_message_id: format!(
                "{}route.{}",
                M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// The most restrictive gate any hop drift imposes.
    fn worst_drift_gate(&self) -> DescriptorGate {
        self.hops
            .iter()
            .map(|h| h.hop_gate)
            .fold(DescriptorGate::Governed, worse_gate)
    }

    /// Validates the timeline's invariants: the hops are contiguously indexed, the route state is
    /// consistent with hop drift, the effective gate folds in drift and freshness, and the message ids
    /// carry the lane prefix.
    fn validate(&self) -> Vec<M5BoundaryInspectorViolation> {
        let mut out = Vec::new();
        if self.hops.is_empty() {
            out.push(M5BoundaryInspectorViolation::RouteTimelineEmpty);
        }
        for (i, hop) in self.hops.iter().enumerate() {
            if hop.index != i as u32
                || hop.locality_label != hop.locality.label()
                || hop.is_local != hop.locality.is_local()
                || hop.hop_gate != hop.drift_marker.gate_posture()
                || !hop
                    .detail_message_id
                    .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
            {
                out.push(M5BoundaryInspectorViolation::RouteHopDrift);
            }
        }
        // The route state must never read more attributable than its hops: a drifting hop forbids a
        // governed route state, and an unattributed hop forces a blocked route state.
        let worst_drift = self.worst_drift_gate();
        if gate_rank(self.route_state.gate_posture()) < gate_rank(worst_drift) {
            out.push(M5BoundaryInspectorViolation::RouteStateDriftMismatch);
        }
        let expected_gate = worse_gate(
            worse_gate(self.route_state.gate_posture(), worst_drift),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5BoundaryInspectorViolation::RouteGateDrift);
        }
        if self.proof_ref.trim().is_empty()
            || !self
                .timeline_message_id
                .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
        {
            out.push(M5BoundaryInspectorViolation::RouteDisclosureIncomplete);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Approval-ticket inspector
// ---------------------------------------------------------------------------------------------

/// One approval-ticket inspector: for one action, the capability class, the approving authority, the
/// scope, the [approval state](ApprovalState) drawn from the runtime authority vocabulary, the expiry
/// and its standing, and the revoke / renew actions an operator can take. An expired ticket blocks; an
/// expiring one narrows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketInspector {
    /// The action this ticket authorizes.
    pub action: HighRiskAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The capability class the ticket grants.
    pub capability_class: CapabilityClass,
    /// Reader-facing capability label.
    pub capability_class_label: String,
    /// The authority that granted the ticket.
    pub approving_authority: ApprovalAuthority,
    /// Reader-facing authority label.
    pub approving_authority_label: String,
    /// A short scope statement (no secrets).
    pub scope_summary: String,
    /// The approval state — drawn from the runtime authority vocabulary.
    pub approval_state: ApprovalState,
    /// Reader-facing approval-state label.
    pub approval_state_label: String,
    /// The ticket's expiry date.
    pub expiry: String,
    /// The standing of the ticket's expiry.
    pub expiry_standing: ExpiryStanding,
    /// The gate the approval state and expiry standing together imply (the more restrictive of the
    /// two).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The revoke / renew actions an operator can take, in canonical order.
    pub revoke_renew_actions: Vec<TicketAction>,
    /// A repo-relative governance-ticket ref (refs only, no secrets).
    pub ticket_ref: String,
    /// The evidence class backing the approval ticket.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the approval proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the approval ticket.
    pub proof_ref: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl ApprovalTicketInspector {
    /// Derives an approval-ticket inspector from the action, its approval state, the expiry standing,
    /// and the expiry date.
    fn derive(
        action: HighRiskAction,
        approval_state: ApprovalState,
        expiry_standing: ExpiryStanding,
        expiry: &str,
    ) -> Self {
        let effective_gate = worse_gate(
            approval_state.gate_posture(),
            expiry_standing.gate_posture(),
        );
        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            capability_class: action.capability_class(),
            capability_class_label: action.capability_class().label().to_owned(),
            approving_authority: action.approving_authority(),
            approving_authority_label: action.approving_authority().label().to_owned(),
            scope_summary: action.scope_summary().to_owned(),
            approval_state,
            approval_state_label: approval_state.label().to_owned(),
            expiry: expiry.to_owned(),
            expiry_standing,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            revoke_renew_actions: revoke_renew_actions_for(effective_gate),
            ticket_ref: action.ticket_ref().to_owned(),
            evidence_class: EvidenceClass::RuntimeApprovalRecord,
            owner_role: APPROVAL_OWNER_ROLE.to_owned(),
            proof_ref: APPROVAL_PROOF_REF.to_owned(),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}approval.{}",
                M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// Validates the ticket's invariants: every derived field matches the action, the effective gate
    /// matches the approval state and expiry standing, the offered actions match the gate, and the
    /// message id carries the lane prefix.
    fn validate(&self) -> Vec<M5BoundaryInspectorViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(
            self.action,
            self.approval_state,
            self.expiry_standing,
            &self.expiry,
        );
        if probe != *self {
            out.push(M5BoundaryInspectorViolation::ApprovalTicketDrift);
        }
        let expected_gate = worse_gate(
            self.approval_state.gate_posture(),
            self.expiry_standing.gate_posture(),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
            || self.revoke_renew_actions != revoke_renew_actions_for(expected_gate)
        {
            out.push(M5BoundaryInspectorViolation::ApprovalGateDrift);
        }
        if self.scope_summary.trim().is_empty()
            || self.expiry.trim().is_empty()
            || self.ticket_ref.trim().is_empty()
            || self.revoke_renew_actions.is_empty()
        {
            out.push(M5BoundaryInspectorViolation::ApprovalDisclosureIncomplete);
        }
        if !self
            .detail_message_id
            .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
        {
            out.push(M5BoundaryInspectorViolation::UnprefixedMessageId);
        }
        out
    }
}

/// The revoke / renew actions offered for an approval-ticket effective gate: a governed ticket can be
/// revoked or renewed; a narrowed one renews or requires reapproval; a blocked one requires
/// reapproval or a tighter scope.
fn revoke_renew_actions_for(gate: DescriptorGate) -> Vec<TicketAction> {
    match gate {
        DescriptorGate::Governed => vec![TicketAction::RevokeApproval, TicketAction::RenewApproval],
        DescriptorGate::Narrowed => {
            vec![TicketAction::RenewApproval, TicketAction::RequireReapproval]
        }
        DescriptorGate::Blocked => {
            vec![TicketAction::RequireReapproval, TicketAction::TightenScope]
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Action inspector
// ---------------------------------------------------------------------------------------------

/// One coverage gap on an action inspector: a facet that narrowed or blocked the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorGap {
    /// The action this gap applies to.
    pub action: HighRiskAction,
    /// The facet that drifted.
    pub facet: InspectorFacet,
    /// The kind of gap.
    pub gap_kind: InspectorGapKind,
    /// Stable message id; prefixed [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One action inspector: the boundary summary card, the route-hop timeline, and the approval-ticket
/// inspector for one action, plus the effective gate (the worst of the three facets), so the inspector
/// never reads safer than its least-attested facet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionInspector {
    /// The action this inspector explains.
    pub action: HighRiskAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The boundary summary card.
    pub boundary_card: BoundarySummaryCard,
    /// The route-hop timeline.
    pub route_timeline: RouteHopTimeline,
    /// The approval-ticket inspector.
    pub approval_ticket: ApprovalTicketInspector,
    /// The effective gate — the worst of the three facets' gates.
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The trust boundaries the action spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// True when the action leaves the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The exact per-facet gaps for this action.
    pub gaps: Vec<InspectorGap>,
    /// The evaluation / export actions offered for this inspector.
    pub evaluation_actions: Vec<EvaluationAction>,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id for the inspector verdict; prefixed
    /// [`M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX`].
    pub verdict_message_id: String,
}

impl ActionInspector {
    /// Derives an action inspector from its seed, building the three cards and folding their gates.
    fn derive(seed: &ActionInspectorSeed) -> Self {
        let action = seed.action;
        let boundary_card =
            BoundarySummaryCard::derive(action, seed.boundary_state, seed.boundary_freshness);
        let route_timeline =
            RouteHopTimeline::derive(action, seed.route_state, &seed.hops, seed.route_freshness);
        let approval_ticket = ApprovalTicketInspector::derive(
            action,
            seed.approval_state,
            seed.expiry_standing,
            &seed.expiry,
        );

        let effective_gate = worse_gate(
            worse_gate(boundary_card.effective_gate, route_timeline.effective_gate),
            approval_ticket.effective_gate,
        );

        // Gaps: one per facet whose effective gate is not governed.
        let mut gaps = Vec::new();
        for (facet, gate) in [
            (InspectorFacet::Boundary, boundary_card.effective_gate),
            (InspectorFacet::Route, route_timeline.effective_gate),
            (InspectorFacet::Approval, approval_ticket.effective_gate),
        ] {
            if let Some(kind) = gap_kind_for_gate(gate) {
                gaps.push(InspectorGap {
                    action,
                    facet,
                    gap_kind: kind,
                    cause_message_id: format!(
                        "{}inspector.{}.{}.{}.gap",
                        M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                        action.as_str(),
                        facet.as_str(),
                        kind.as_str()
                    ),
                });
            }
        }

        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            trust_boundaries: boundary_card.trust_boundaries.clone(),
            crosses_trust_boundary: boundary_card.crosses_trust_boundary,
            boundary_card,
            route_timeline,
            approval_ticket,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            gaps,
            evaluation_actions: EvaluationAction::ALL.to_vec(),
            status,
            signal: status.signal(),
            verdict_message_id: format!(
                "{}inspector.{}.verdict",
                M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// True when every facet stands fully governed.
    pub fn is_governed(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Governed)
    }

    /// True when a facet narrowed the inspector below fully governed.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Narrowed)
    }

    /// True when a facet blocked the inspector.
    pub fn is_blocked(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Blocked)
    }

    /// Validates the inspector's invariants: the cards self-validate, the effective gate is the worst
    /// of the three facets, the gaps match the facet gates, and the message ids carry the lane prefix.
    fn validate(&self) -> Vec<M5BoundaryInspectorViolation> {
        let mut out = Vec::new();
        if self.boundary_card.action != self.action
            || self.route_timeline.action != self.action
            || self.approval_ticket.action != self.action
            || self.action_label != self.action.label()
        {
            out.push(M5BoundaryInspectorViolation::InspectorFieldMismatch);
        }
        out.extend(self.boundary_card.validate());
        out.extend(self.route_timeline.validate());
        out.extend(self.approval_ticket.validate());

        let expected_gate = worse_gate(
            worse_gate(
                self.boundary_card.effective_gate,
                self.route_timeline.effective_gate,
            ),
            self.approval_ticket.effective_gate,
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5BoundaryInspectorViolation::InspectorGateDrift);
        }

        // Gaps must name exactly the not-governed facets.
        let expected_gaps: Vec<(InspectorFacet, InspectorGapKind)> = [
            (InspectorFacet::Boundary, self.boundary_card.effective_gate),
            (InspectorFacet::Route, self.route_timeline.effective_gate),
            (
                InspectorFacet::Approval,
                self.approval_ticket.effective_gate,
            ),
        ]
        .into_iter()
        .filter_map(|(facet, gate)| gap_kind_for_gate(gate).map(|kind| (facet, kind)))
        .collect();
        let actual_gaps: Vec<(InspectorFacet, InspectorGapKind)> =
            self.gaps.iter().map(|g| (g.facet, g.gap_kind)).collect();
        if actual_gaps != expected_gaps {
            out.push(M5BoundaryInspectorViolation::InspectorGapDrift);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
            {
                out.push(M5BoundaryInspectorViolation::UnprefixedMessageId);
            }
        }
        if self.evaluation_actions != EvaluationAction::ALL.to_vec()
            || !self
                .verdict_message_id
                .starts_with(M5_BOUNDARY_INSPECTOR_MESSAGE_ID_PREFIX)
        {
            out.push(M5BoundaryInspectorViolation::InspectorFieldMismatch);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Evaluation packet (export)
// ---------------------------------------------------------------------------------------------

/// One action entry in the exported evaluation packet — the same boundary / route / approval
/// vocabulary the cards show, reduced to refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationActionEntry {
    /// The action token.
    pub action: HighRiskAction,
    /// The boundary class.
    pub boundary_class: BoundaryClass,
    /// The boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// The route state.
    pub route_state: RouteHopState,
    /// The approval state.
    pub approval_state: ApprovalState,
    /// The capability class.
    pub capability_class: CapabilityClass,
    /// The approving authority.
    pub approving_authority: ApprovalAuthority,
    /// The inspector's effective gate.
    pub effective_gate: DescriptorGate,
    /// Effective qualification.
    pub effective_qualification: QualificationClass,
    /// True when the action crosses the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The proof refs backing the action's facets (refs only).
    pub proof_refs: Vec<String>,
}

/// The exported evaluation packet: each action inspector reduced to the exact boundary / route /
/// approval vocabulary the in-product inspector shows, so an exported pack and the live UI can never
/// read differently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorEvaluationPacket {
    /// Record kind; must equal [`M5_BOUNDARY_INSPECTOR_EVALUATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; mirrors the parent packet.
    pub schema_version: u32,
    /// Stable evaluation-packet id.
    pub packet_id: String,
    /// The boundary-inspector packet this export was generated from.
    pub generated_from: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The action entries.
    pub actions: Vec<EvaluationActionEntry>,
    /// The controlled vocabulary the entries draw from.
    pub vocabulary: BoundaryInspectorVocabulary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl InspectorEvaluationPacket {
    /// Builds the evaluation packet from the action inspectors.
    fn derive(
        packet_id: &str,
        generated_from: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        inspectors: &[ActionInspector],
    ) -> Self {
        let actions = inspectors
            .iter()
            .map(|i| EvaluationActionEntry {
                action: i.action,
                boundary_class: i.boundary_card.boundary_class,
                boundary_state: i.boundary_card.boundary_state,
                route_state: i.route_timeline.route_state,
                approval_state: i.approval_ticket.approval_state,
                capability_class: i.approval_ticket.capability_class,
                approving_authority: i.approval_ticket.approving_authority,
                effective_gate: i.effective_gate,
                effective_qualification: i.effective_qualification,
                crosses_trust_boundary: i.crosses_trust_boundary,
                proof_refs: vec![
                    i.boundary_card.proof_ref.clone(),
                    i.route_timeline.proof_ref.clone(),
                    i.approval_ticket.proof_ref.clone(),
                ],
            })
            .collect();
        Self {
            record_kind: M5_BOUNDARY_INSPECTOR_EVALUATION_RECORD_KIND.to_owned(),
            schema_version: M5_BOUNDARY_INSPECTOR_SCHEMA_VERSION,
            packet_id: packet_id.to_owned(),
            generated_from: generated_from.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            actions,
            vocabulary: BoundaryInspectorVocabulary::canonical(),
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the evaluation packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 boundary inspector evaluation packet serializes")
    }

    /// True when every token the packet carries is a member of the canonical vocabulary, so the
    /// export reuses the same grammar the UI shows.
    fn reuses_canonical_vocabulary(&self) -> bool {
        if !self.vocabulary.matches_canonical() {
            return false;
        }
        let vocab = &self.vocabulary;
        self.actions.iter().all(|a| {
            vocab.actions.contains(&a.action.as_str().to_owned())
                && vocab
                    .boundary_classes
                    .contains(&a.boundary_class.as_str().to_owned())
                && vocab
                    .boundary_states
                    .contains(&a.boundary_state.as_str().to_owned())
                && vocab
                    .route_states
                    .contains(&a.route_state.as_str().to_owned())
                && vocab
                    .approval_states
                    .contains(&a.approval_state.as_str().to_owned())
                && vocab
                    .capability_classes
                    .contains(&a.capability_class.as_str().to_owned())
                && vocab
                    .approval_authorities
                    .contains(&a.approving_authority.as_str().to_owned())
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryInspectorVocabulary {
    /// High-risk action tokens.
    pub actions: Vec<String>,
    /// Boundary-class tokens.
    pub boundary_classes: Vec<String>,
    /// Actor-class tokens.
    pub actor_classes: Vec<String>,
    /// Target-class tokens.
    pub target_classes: Vec<String>,
    /// Sensitive-data-class tokens.
    pub sensitive_data_classes: Vec<String>,
    /// Capability-boundary state tokens.
    pub boundary_states: Vec<String>,
    /// Hop-locality tokens.
    pub hop_localities: Vec<String>,
    /// Hop-role tokens.
    pub hop_roles: Vec<String>,
    /// Certificate-context tokens.
    pub certificate_contexts: Vec<String>,
    /// Hop-drift-marker tokens.
    pub drift_markers: Vec<String>,
    /// Route-hop state tokens.
    pub route_states: Vec<String>,
    /// Capability-class tokens.
    pub capability_classes: Vec<String>,
    /// Approval-authority tokens.
    pub approval_authorities: Vec<String>,
    /// Approval state tokens.
    pub approval_states: Vec<String>,
    /// Expiry-standing tokens.
    pub expiry_standings: Vec<String>,
    /// Ticket-action tokens.
    pub ticket_actions: Vec<String>,
    /// Evaluation-action tokens.
    pub evaluation_actions: Vec<String>,
    /// Inspector-facet tokens.
    pub facets: Vec<String>,
    /// Trust-boundary tokens.
    pub trust_boundaries: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl BoundaryInspectorVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            actions: tokens(&HighRiskAction::ALL, |a| a.as_str()),
            boundary_classes: tokens(&BoundaryClass::ALL, |b| b.as_str()),
            actor_classes: tokens(&ActorClass::ALL, |a| a.as_str()),
            target_classes: tokens(&TargetClass::ALL, |t| t.as_str()),
            sensitive_data_classes: tokens(&SensitiveDataClass::ALL, |c| c.as_str()),
            boundary_states: tokens(&CapabilityBoundaryState::ALL, |s| s.as_str()),
            hop_localities: tokens(&HopLocality::ALL, |l| l.as_str()),
            hop_roles: tokens(&HopRole::ALL, |r| r.as_str()),
            certificate_contexts: tokens(&CertificateContext::ALL, |c| c.as_str()),
            drift_markers: tokens(&HopDriftMarker::ALL, |d| d.as_str()),
            route_states: tokens(&RouteHopState::ALL, |s| s.as_str()),
            capability_classes: tokens(&CapabilityClass::ALL, |c| c.as_str()),
            approval_authorities: tokens(&ApprovalAuthority::ALL, |a| a.as_str()),
            approval_states: tokens(&ApprovalState::ALL, |s| s.as_str()),
            expiry_standings: tokens(&ExpiryStanding::ALL, |s| s.as_str()),
            ticket_actions: tokens(&TicketAction::ALL, |a| a.as_str()),
            evaluation_actions: tokens(&EvaluationAction::ALL, |a| a.as_str()),
            facets: tokens(&InspectorFacet::ALL, |f| f.as_str()),
            trust_boundaries: tokens(&TrustBoundary::ALL, |b| b.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact boundary-inspector summary — the scoreboard the renderers and exports read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryInspectorSummary {
    /// Total action inspectors.
    pub total_actions: u32,
    /// Inspectors fully governed.
    pub governed_actions: u32,
    /// Inspectors narrowed by a facet.
    pub narrowed_actions: u32,
    /// Inspectors blocked by a facet.
    pub blocked_actions: u32,
    /// Actions that cross the local-first trust boundary.
    pub crossing_actions: u32,
    /// Routes carrying at least one drift marker.
    pub drifted_routes: u32,
    /// Approval tickets that have expired.
    pub expired_approvals: u32,
    /// Total boundary cards.
    pub total_boundary_cards: u32,
    /// Total route timelines.
    pub total_route_timelines: u32,
    /// Total approval tickets.
    pub total_approval_tickets: u32,
    /// True when at least one inspector is blocked.
    pub blocks_stable_promotion: bool,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryInspectorConformance {
    /// Every boundary card declares class, actor, target, data, authority, and an export-safe summary.
    pub boundary_card_declares_class_actor_target_and_data: bool,
    /// Every route timeline is ordered and names locality, role, and certificate per hop.
    pub route_timeline_ordered_with_locality_per_hop: bool,
    /// Every approval ticket binds capability, authority, scope, and expiry.
    pub approval_ticket_binds_authority_scope_and_expiry: bool,
    /// Every inspector's gate is the worst of its three facets — it never overstates.
    pub inspector_gate_is_worst_of_facets: bool,
    /// A route-hop drift narrows the route timeline deterministically.
    pub route_drift_narrows_deterministically: bool,
    /// An unattributed route hop blocks the inspector.
    pub unattributed_route_blocks_stable_promotion: bool,
    /// An expired approval blocks the inspector.
    pub expired_approval_blocks_stable_promotion: bool,
    /// Every boundary card binds the capability-boundary vocabulary.
    pub boundary_state_bound_to_capability_vocabulary: bool,
    /// Every route timeline binds the route-hop vocabulary.
    pub route_state_bound_to_route_vocabulary: bool,
    /// Every approval ticket binds the runtime approval vocabulary.
    pub approval_state_bound_to_approval_vocabulary: bool,
    /// The exported evaluation packet reuses the in-product vocabulary.
    pub evaluation_packet_reuses_ui_vocabulary: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — route / proof lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl BoundaryInspectorConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.boundary_card_declares_class_actor_target_and_data
            && self.route_timeline_ordered_with_locality_per_hop
            && self.approval_ticket_binds_authority_scope_and_expiry
            && self.inspector_gate_is_worst_of_facets
            && self.route_drift_narrows_deterministically
            && self.unattributed_route_blocks_stable_promotion
            && self.expired_approval_blocks_stable_promotion
            && self.boundary_state_bound_to_capability_vocabulary
            && self.route_state_bound_to_route_vocabulary
            && self.approval_state_bound_to_approval_vocabulary
            && self.evaluation_packet_reuses_ui_vocabulary
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// Seed input for one action inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInspectorSeed {
    /// The action.
    pub action: HighRiskAction,
    /// The capability-boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// Freshness of the boundary evidence.
    pub boundary_freshness: FreshnessState,
    /// The route-hop state.
    pub route_state: RouteHopState,
    /// Freshness of the route evidence.
    pub route_freshness: FreshnessState,
    /// The ordered route hops.
    pub hops: Vec<RouteHopSeed>,
    /// The approval state.
    pub approval_state: ApprovalState,
    /// The approval's expiry standing.
    pub expiry_standing: ExpiryStanding,
    /// The approval's expiry date.
    pub expiry: String,
}

/// Constructor input for [`M5BoundaryInspector::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BoundaryInspectorInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-action inspector seeds.
    pub action_seeds: Vec<ActionInspectorSeed>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable boundary-inspector truth packet: the per-action inspectors
/// (each carrying a boundary summary card, a route-hop timeline, and an approval-ticket inspector), the
/// exported evaluation packet, the controlled vocabulary, a summary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BoundaryInspector {
    /// Record kind; must equal [`M5_BOUNDARY_INSPECTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BOUNDARY_INSPECTOR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-action inspectors, in action order.
    pub action_inspectors: Vec<ActionInspector>,
    /// The exported evaluation packet.
    pub evaluation_packet: InspectorEvaluationPacket,
    /// Controlled-vocabulary set.
    pub vocabulary: BoundaryInspectorVocabulary,
    /// Compact summary.
    pub summary: BoundaryInspectorSummary,
    /// Conformance review block.
    pub conformance: BoundaryInspectorConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BoundaryInspector {
    /// Builds a boundary-inspector packet from seed input, deriving each action inspector from its
    /// seed and the summary / conformance / evaluation packet from all of them.
    pub fn new(input: M5BoundaryInspectorInput) -> Self {
        let mut action_inspectors: Vec<ActionInspector> = input
            .action_seeds
            .iter()
            .map(ActionInspector::derive)
            .collect();
        action_inspectors.sort_by_key(|i| action_rank(i.action));
        action_inspectors.dedup_by_key(|i| i.action);

        let evaluation_packet = InspectorEvaluationPacket::derive(
            &format!("{}:eval", input.packet_id),
            &input.packet_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &action_inspectors,
        );

        let summary = derive_summary(&action_inspectors);
        let conformance = derive_conformance(&action_inspectors, &evaluation_packet);

        Self {
            record_kind: M5_BOUNDARY_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: M5_BOUNDARY_INSPECTOR_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            action_inspectors,
            evaluation_packet,
            vocabulary: BoundaryInspectorVocabulary::canonical(),
            summary,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion — at least one inspector is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.summary.blocks_stable_promotion
    }

    /// Finds an action inspector by action.
    pub fn inspector(&self, action: HighRiskAction) -> Option<&ActionInspector> {
        self.action_inspectors.iter().find(|i| i.action == action)
    }

    /// The boundary summary cards, in action order.
    pub fn boundary_cards(&self) -> Vec<&BoundarySummaryCard> {
        self.action_inspectors
            .iter()
            .map(|i| &i.boundary_card)
            .collect()
    }

    /// The route-hop timelines, in action order.
    pub fn route_timelines(&self) -> Vec<&RouteHopTimeline> {
        self.action_inspectors
            .iter()
            .map(|i| &i.route_timeline)
            .collect()
    }

    /// The approval-ticket inspectors, in action order.
    pub fn approval_tickets(&self) -> Vec<&ApprovalTicketInspector> {
        self.action_inspectors
            .iter()
            .map(|i| &i.approval_ticket)
            .collect()
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: BoundaryInspectorChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 boundary inspector serializes")
    }

    /// The exported evaluation packet's JSON.
    pub fn render_evaluation_packet(&self) -> String {
        self.evaluation_packet.export_safe_json()
    }

    /// Deterministic, machine-readable action / facet matrix CSV: one row per action, naming the
    /// boundary class / state, the route state and drift count, the approval state and expiry, and the
    /// inspector verdict.
    pub fn render_actions_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "action,boundary_class,boundary_state,actor,target_class,crosses_boundary,route_state,hop_count,drift_count,capability_class,approving_authority,approval_state,expiry_standing,effective_gate,effective_qualification\n",
        );
        for i in &self.action_inspectors {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                i.action.as_str(),
                i.boundary_card.boundary_class.as_str(),
                i.boundary_card.boundary_state.as_str(),
                i.boundary_card.actor.as_str(),
                i.boundary_card.target_class.as_str(),
                i.crosses_trust_boundary,
                i.route_timeline.route_state.as_str(),
                i.route_timeline.hops.len(),
                i.route_timeline.drift_marker_count,
                i.approval_ticket.capability_class.as_str(),
                i.approval_ticket.approving_authority.as_str(),
                i.approval_ticket.approval_state.as_str(),
                i.approval_ticket.expiry_standing.as_str(),
                i.effective_gate.as_str(),
                i.effective_qualification.as_str(),
            ));
        }
        out
    }

    /// Deterministic boundary-inspector overview document for review, support, docs, or evaluator
    /// handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Boundary Inspector\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Actions: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_actions,
            self.summary.governed_actions,
            self.summary.narrowed_actions,
            self.summary.blocked_actions
        ));
        out.push_str(&format!(
            "- Boundary crossings: {} | Drifted routes: {} | Expired approvals: {}\n",
            self.summary.crossing_actions,
            self.summary.drifted_routes,
            self.summary.expired_approvals
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Action inspectors\n\n");
        out.push_str(
            "| Action | Boundary class | Boundary state | Route state | Approval state | Gate | Qualification |\n",
        );
        out.push_str(
            "|--------|----------------|----------------|-------------|----------------|------|---------------|\n",
        );
        for i in &self.action_inspectors {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                i.action.as_str(),
                i.boundary_card.boundary_class.as_str(),
                i.boundary_card.boundary_state.as_str(),
                i.route_timeline.route_state.as_str(),
                i.approval_ticket.approval_state.as_str(),
                i.effective_gate.as_str(),
                i.effective_qualification.as_str(),
            ));
        }

        out.push_str("\n## Boundary summary cards\n\n");
        out.push_str(
            "| Action | Boundary class | Actor | Target | Data classes | Authority | Summary |\n",
        );
        out.push_str(
            "|--------|----------------|-------|--------|--------------|-----------|---------|\n",
        );
        for c in self.boundary_cards() {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | `{}` | {} |\n",
                c.action.as_str(),
                c.boundary_class.as_str(),
                c.actor.as_str(),
                c.target_class.as_str(),
                join_tokens(&c.sensitive_data_classes, |d| d.as_str()),
                c.approval_authority.as_str(),
                c.export_safe_summary,
            ));
        }

        out.push_str("\n## Route-hop timelines\n\n");
        for t in self.route_timelines() {
            out.push_str(&format!(
                "- `{}` — `{}` ({} hops, {} drift): ",
                t.action.as_str(),
                t.route_state.as_str(),
                t.hops.len(),
                t.drift_marker_count
            ));
            let path: Vec<String> = t
                .hops
                .iter()
                .map(|h| {
                    if matches!(h.drift_marker, HopDriftMarker::None) {
                        format!("{}[{}]", h.locality.as_str(), h.role.as_str())
                    } else {
                        format!(
                            "{}[{}!{}]",
                            h.locality.as_str(),
                            h.role.as_str(),
                            h.drift_marker.as_str()
                        )
                    }
                })
                .collect();
            out.push_str(&path.join(" → "));
            out.push('\n');
        }

        out.push_str("\n## Approval tickets\n\n");
        out.push_str(
            "| Action | Capability | Authority | Approval state | Expiry | Standing | Actions |\n",
        );
        out.push_str(
            "|--------|------------|-----------|----------------|--------|----------|---------|\n",
        );
        for a in self.approval_tickets() {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                a.action.as_str(),
                a.capability_class.as_str(),
                a.approving_authority.as_str(),
                a.approval_state.as_str(),
                a.expiry,
                a.expiry_standing.as_str(),
                join_tokens(&a.revoke_renew_actions, |x| x.as_str()),
            ));
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Boundary Inspector — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Actions: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_actions,
            self.summary.governed_actions,
            self.summary.narrowed_actions,
            self.summary.blocked_actions
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(&format!(
            "- Evaluation packet: `{}`\n",
            M5_BOUNDARY_INSPECTOR_EVALUATION_PACKET_REF
        ));
        out.push_str(&format!(
            "- Actions CSV: `{}`\n",
            M5_BOUNDARY_INSPECTOR_ACTIONS_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5BoundaryInspectorViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_BOUNDARY_INSPECTOR_RECORD_KIND {
            out.push(M5BoundaryInspectorViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BOUNDARY_INSPECTOR_SCHEMA_VERSION {
            out.push(M5BoundaryInspectorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5BoundaryInspectorViolation::MissingIdentity);
        }

        // Every action inspected exactly once and self-consistent.
        let mut seen = std::collections::BTreeSet::new();
        for inspector in &self.action_inspectors {
            if !seen.insert(inspector.action) {
                out.push(M5BoundaryInspectorViolation::DuplicateAction);
            }
            out.extend(inspector.validate());
        }
        for action in HighRiskAction::ALL {
            if !self.action_inspectors.iter().any(|i| i.action == action) {
                out.push(M5BoundaryInspectorViolation::ActionNotInspected);
            }
        }

        let expected_eval = InspectorEvaluationPacket::derive(
            &self.evaluation_packet.packet_id,
            &self.packet_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.action_inspectors,
        );
        if self.evaluation_packet != expected_eval
            || !self.evaluation_packet.reuses_canonical_vocabulary()
        {
            out.push(M5BoundaryInspectorViolation::EvaluationPacketDrift);
        }

        if !self.vocabulary.matches_canonical() {
            out.push(M5BoundaryInspectorViolation::VocabularyMismatch);
        }
        if self.summary != derive_summary(&self.action_inspectors) {
            out.push(M5BoundaryInspectorViolation::SummaryDrift);
        }
        if self.conformance != derive_conformance(&self.action_inspectors, &self.evaluation_packet)
            || !self.conformance.all_hold()
        {
            out.push(M5BoundaryInspectorViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 boundary inspector serializes"),
        ) {
            out.push(M5BoundaryInspectorViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel a boundary-inspector packet is produced on. Every channel produces
/// byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryInspectorChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl BoundaryInspectorChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------------------------

/// Derives the summary from the action inspectors.
fn derive_summary(inspectors: &[ActionInspector]) -> BoundaryInspectorSummary {
    let gate_count = |gate: DescriptorGate| -> u32 {
        inspectors
            .iter()
            .filter(|i| i.effective_gate == gate)
            .count() as u32
    };
    let blocked = gate_count(DescriptorGate::Blocked);
    BoundaryInspectorSummary {
        total_actions: inspectors.len() as u32,
        governed_actions: gate_count(DescriptorGate::Governed),
        narrowed_actions: gate_count(DescriptorGate::Narrowed),
        blocked_actions: blocked,
        crossing_actions: inspectors
            .iter()
            .filter(|i| i.crosses_trust_boundary)
            .count() as u32,
        drifted_routes: inspectors
            .iter()
            .filter(|i| i.route_timeline.drift_marker_count > 0)
            .count() as u32,
        expired_approvals: inspectors
            .iter()
            .filter(|i| matches!(i.approval_ticket.expiry_standing, ExpiryStanding::Expired))
            .count() as u32,
        total_boundary_cards: inspectors.len() as u32,
        total_route_timelines: inspectors.len() as u32,
        total_approval_tickets: inspectors.len() as u32,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    inspectors: &[ActionInspector],
    evaluation_packet: &InspectorEvaluationPacket,
) -> BoundaryInspectorConformance {
    let boundary_ok = inspectors.iter().all(|i| {
        let c = &i.boundary_card;
        !c.sensitive_data_classes.is_empty()
            && !c.export_safe_summary.trim().is_empty()
            && !c.proof_ref.trim().is_empty()
            && c.boundary_class == i.action.boundary_class()
            && c.actor == i.action.actor()
            && c.target_class == i.action.target_class()
            && c.approval_authority == i.action.approving_authority()
    });

    let route_ok = inspectors.iter().all(|i| {
        let t = &i.route_timeline;
        !t.hops.is_empty()
            && t.hops
                .iter()
                .enumerate()
                .all(|(idx, h)| h.index == idx as u32 && h.locality_label == h.locality.label())
            && !t.proof_ref.trim().is_empty()
    });

    let approval_ok = inspectors.iter().all(|i| {
        let a = &i.approval_ticket;
        !a.scope_summary.trim().is_empty()
            && !a.expiry.trim().is_empty()
            && !a.ticket_ref.trim().is_empty()
            && !a.revoke_renew_actions.is_empty()
            && a.capability_class == i.action.capability_class()
            && a.approving_authority == i.action.approving_authority()
    });

    let worst_of_facets = inspectors.iter().all(|i| {
        let expected = worse_gate(
            worse_gate(
                i.boundary_card.effective_gate,
                i.route_timeline.effective_gate,
            ),
            i.approval_ticket.effective_gate,
        );
        i.effective_gate == expected
    });

    // A route whose hops carry a narrowing drift must not read governed.
    let route_drift_narrows = inspectors.iter().all(|i| {
        let worst_drift = i
            .route_timeline
            .hops
            .iter()
            .map(|h| h.hop_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        match gap_kind_for_gate(worst_drift) {
            None => true,
            Some(InspectorGapKind::FacetNarrowed) => {
                i.route_timeline.effective_gate != DescriptorGate::Governed
            }
            Some(InspectorGapKind::FacetBlocked) => {
                i.route_timeline.effective_gate == DescriptorGate::Blocked
            }
        }
    });

    // An unattributed hop blocks; an expired approval blocks.
    let unattributed_blocks = inspectors.iter().all(|i| {
        let has_unattributed = i
            .route_timeline
            .hops
            .iter()
            .any(|h| matches!(h.drift_marker, HopDriftMarker::UnattributedHop));
        !has_unattributed || i.is_blocked()
    });
    let expired_blocks = inspectors.iter().all(|i| {
        !matches!(i.approval_ticket.expiry_standing, ExpiryStanding::Expired) || i.is_blocked()
    });

    let boundary_vocab = inspectors
        .iter()
        .all(|i| CapabilityBoundaryState::ALL.contains(&i.boundary_card.boundary_state));
    let route_vocab = inspectors
        .iter()
        .all(|i| RouteHopState::ALL.contains(&i.route_timeline.route_state));
    let approval_vocab = inspectors
        .iter()
        .all(|i| ApprovalState::ALL.contains(&i.approval_ticket.approval_state));

    let export_clean = !json_contains_forbidden_material(
        &serde_json::to_value(inspectors).expect("inspectors serialize"),
    );

    BoundaryInspectorConformance {
        boundary_card_declares_class_actor_target_and_data: boundary_ok,
        route_timeline_ordered_with_locality_per_hop: route_ok,
        approval_ticket_binds_authority_scope_and_expiry: approval_ok,
        inspector_gate_is_worst_of_facets: worst_of_facets,
        route_drift_narrows_deterministically: route_drift_narrows,
        unattributed_route_blocks_stable_promotion: unattributed_blocks,
        expired_approval_blocks_stable_promotion: expired_blocks,
        boundary_state_bound_to_capability_vocabulary: boundary_vocab,
        route_state_bound_to_route_vocabulary: route_vocab,
        approval_state_bound_to_approval_vocabulary: approval_vocab,
        evaluation_packet_reuses_ui_vocabulary: evaluation_packet.reuses_canonical_vocabulary(),
        controlled_enums_frozen: BoundaryInspectorVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

// ---------------------------------------------------------------------------------------------
// Ranking / token helpers
// ---------------------------------------------------------------------------------------------

/// Position of an action in the canonical ordering.
fn action_rank(action: HighRiskAction) -> usize {
    HighRiskAction::ALL
        .iter()
        .position(|a| *a == action)
        .unwrap_or(HighRiskAction::ALL.len())
}

/// Position of a data class in the canonical ordering.
fn data_class_rank(class: SensitiveDataClass) -> usize {
    SensitiveDataClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(SensitiveDataClass::ALL.len())
}

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// Joins a token list for table / CSV rendering, comma-space separated.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items.iter().map(|t| f(*t)).collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------------------------

/// Validation failures for the boundary-inspector lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryInspectorViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A boundary card drifted from a fresh derivation of its action.
    BoundaryCardDrift,
    /// A boundary card's gate or qualification drifted.
    BoundaryGateDrift,
    /// A boundary card omits data classes, a proof ref, or an export-safe summary.
    BoundaryDisclosureIncomplete,
    /// A route timeline has no hops.
    RouteTimelineEmpty,
    /// A route hop cites a field that does not match its position or locality.
    RouteHopDrift,
    /// A route state reads more attributable than its hops allow.
    RouteStateDriftMismatch,
    /// A route timeline's gate or qualification drifted.
    RouteGateDrift,
    /// A route timeline omits a proof ref or message id.
    RouteDisclosureIncomplete,
    /// An approval ticket drifted from a fresh derivation of its action.
    ApprovalTicketDrift,
    /// An approval ticket's gate, qualification, or offered actions drifted.
    ApprovalGateDrift,
    /// An approval ticket omits scope, expiry, a ticket ref, or actions.
    ApprovalDisclosureIncomplete,
    /// An action inspector cites a field that does not match its action.
    InspectorFieldMismatch,
    /// An action inspector's gate or qualification drifted from the worst of its facets.
    InspectorGateDrift,
    /// An action inspector's gaps do not name exactly the not-governed facets.
    InspectorGapDrift,
    /// Two inspectors name the same action.
    DuplicateAction,
    /// An action has no inspector.
    ActionNotInspected,
    /// The evaluation packet drifted from the inspectors or its vocabulary.
    EvaluationPacketDrift,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The summary disagrees with the inspectors.
    SummaryDrift,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5BoundaryInspectorViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BoundaryCardDrift => "boundary_card_drift",
            Self::BoundaryGateDrift => "boundary_gate_drift",
            Self::BoundaryDisclosureIncomplete => "boundary_disclosure_incomplete",
            Self::RouteTimelineEmpty => "route_timeline_empty",
            Self::RouteHopDrift => "route_hop_drift",
            Self::RouteStateDriftMismatch => "route_state_drift_mismatch",
            Self::RouteGateDrift => "route_gate_drift",
            Self::RouteDisclosureIncomplete => "route_disclosure_incomplete",
            Self::ApprovalTicketDrift => "approval_ticket_drift",
            Self::ApprovalGateDrift => "approval_gate_drift",
            Self::ApprovalDisclosureIncomplete => "approval_disclosure_incomplete",
            Self::InspectorFieldMismatch => "inspector_field_mismatch",
            Self::InspectorGateDrift => "inspector_gate_drift",
            Self::InspectorGapDrift => "inspector_gap_drift",
            Self::DuplicateAction => "duplicate_action",
            Self::ActionNotInspected => "action_not_inspected",
            Self::EvaluationPacketDrift => "evaluation_packet_drift",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of the
/// upstream descriptor / governance lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
