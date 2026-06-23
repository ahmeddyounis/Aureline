//! M5 admin-plane *procurement / verification packets*, *renewal / trial /
//! seat-change summary cards*, and *admin-handoff bundles*: the concrete, typed
//! instances of the buyer-, renewal-, support-, and admin-handoff surfaces
//! Aureline shows on its claimed managed-cloud, self-hosted, sovereign/air-gapped,
//! and mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — including the
//! [`ProcurementVerificationPacket`](crate::m5_admin_plane::AdminSurfaceClass::ProcurementVerificationPacket)
//! surface family, the states it admits, the controlled vocabularies it binds,
//! and the proof packet that keeps it current — this lane *renders that surface*.
//! It turns procurement, verification, renewal, and admin handoff into first-class
//! local product surfaces: an evaluator, auditor, renewer, or support engineer can,
//! on the machine in front of them, read the deployment mode, supported export
//! paths, billing/owner scope, validity-window and signature posture, evidence
//! refs, residual dependencies, and support/renewal handoff data that prove current
//! posture; see each renewal, trial, or seat-change event with its effective date,
//! impacted managed features, as-of date, local-only path, and the export/support
//! next step; and export an admin-handoff packet with build/channel, install mode,
//! workspace archetype, bundle ids, and affected features — all without a separate
//! vendor console and without a still-active paid seat to recover user-owned data.
//!
//! Each packet binds back to the frozen [admin-plane matrix](crate::m5_admin_plane).
//! Every machine-readable state a verification packet, event card, admin-handoff
//! packet, or the coverage posture shows must be one the matrix declares applicable
//! for the procurement surface
//! ([`ProcurementInvariant`] `procurement.surface_states_within_matrix`), and every
//! owner and residency token it uses is a term the matrix's shared vocabulary
//! defines. So the render layer cannot drift from the frozen contract: an edit that
//! shows a state the matrix does not admit flips an invariant and fails the freeze
//! gate.
//!
//! The bundle holds one [`ProcurementProfilePacket`] per claimed managed-bearing
//! profile and computes each invariant's `holds` flag from the rendered data, so
//! the checked-in fixture freezes the rendered packets byte-for-byte. The spec's
//! honesty rules are enforced, not just described:
//!
//! - Verification packets, renewal/trial/seat-change cards, and admin-handoff
//!   packets are exportable directly with current owner scope, an as-of date, and
//!   evidence refs (`procurement.owner_scope_and_asof`,
//!   `procurement.evidence_refs_present`).
//! - A renewal, trial, or seat-change card never outranks the export, delete,
//!   support, or local-continuation actions in an entitlement-loss context: each
//!   card carries an ordered action list where every recovery action precedes any
//!   renewal/billing call-to-action and is flagged
//!   `outranks_recovery_actions = false` (`procurement.events_never_outrank_recovery`).
//! - Commercial, support, and admin packets reuse the same canonical managed-state
//!   objects — effective policy, entitlement/seat, retention/deletion,
//!   offboarding/continuity, endpoint posture, and decision history — by ref rather
//!   than restating them with drift-prone local copy
//!   (`procurement.reuses_canonical_objects`).
//! - A packet past its validity window or with an unverifiable signature is
//!   labeled, never presented as currently verified, and a stale-evidence packet is
//!   never shown under a confirmed active state
//!   (`procurement.validity_labeled`, `procurement.verification_no_silent_green`).
//! - Export and local-continuation paths stay reachable without a still-active paid
//!   seat (`procurement.no_paid_seat_for_recovery`) and every profile stays locally
//!   inspectable without a vendor console (`procurement.locally_inspectable_offline`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, raw record bodies, or absolute paths — only opaque object refs, stable
//! tokens, rendered metadata-safe summaries, and short reviewable sentences — so it
//! is safe to embed in a support, procurement, or renewal export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{
    DataResidencyClass, EvidenceAgeClass, InstallModeClass, OwnerEscalationRoleClass,
    UpdateRingClass, VerificationPostureClass,
};
// Reuse the completeness and export-form vocabularies the sibling render layers
// freeze, so procurement labels coverage and export forms with the same tokens
// every admin surface uses.
pub use crate::m5_decision_history::{CompletenessClass, ExportForm, ExportFormatClass};

#[cfg(test)]
mod tests;

/// Schema version for the procurement bundle.
pub const M5_PROCUREMENT_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the procurement bundle.
pub const M5_PROCUREMENT_SCHEMA_REF: &str = "schemas/admin/m5-procurement.schema.json";

/// Stable record-kind tag for the procurement bundle.
pub const M5_PROCUREMENT_RECORD_KIND: &str = "m5_procurement_bundle";

/// Stable id for the canonical procurement bundle.
pub const M5_PROCUREMENT_BUNDLE_ID: &str = "m5-procurement:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_PROCUREMENT_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this render layer binds back to.
pub const M5_PROCUREMENT_MATRIX_REF: &str = "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the procurement bundle current.
pub const M5_PROCUREMENT_FREEZE_GATE_REF: &str = "crates/aureline-policy/tests/m5_procurement.rs";

// ---------------------------------------------------------------------------
// Procurement token enums.
// ---------------------------------------------------------------------------

/// The commercial event a renewal / trial / seat-change card discloses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommercialEventClass {
    /// A subscription or contract renewal.
    Renewal,
    /// A trial begins.
    TrialStart,
    /// A trial is ending or has ended.
    TrialExpiry,
    /// Seats are added.
    SeatIncrease,
    /// Seats are removed.
    SeatDecrease,
    /// A plan is downgraded to fewer entitlements.
    PlanDowngrade,
    /// A subscription is cancelled.
    Cancellation,
}

impl CommercialEventClass {
    /// All commercial event classes, in vocabulary order.
    pub const ALL: [Self; 7] = [
        Self::Renewal,
        Self::TrialStart,
        Self::TrialExpiry,
        Self::SeatIncrease,
        Self::SeatDecrease,
        Self::PlanDowngrade,
        Self::Cancellation,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Renewal => "renewal",
            Self::TrialStart => "trial_start",
            Self::TrialExpiry => "trial_expiry",
            Self::SeatIncrease => "seat_increase",
            Self::SeatDecrease => "seat_decrease",
            Self::PlanDowngrade => "plan_downgrade",
            Self::Cancellation => "cancellation",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Renewal => "Renewal",
            Self::TrialStart => "Trial start",
            Self::TrialExpiry => "Trial expiry",
            Self::SeatIncrease => "Seat increase",
            Self::SeatDecrease => "Seat decrease",
            Self::PlanDowngrade => "Plan downgrade",
            Self::Cancellation => "Cancellation",
        }
    }

    /// Whether this event reduces entitlements — the context in which a card must
    /// never outrank the export, delete, support, or local-continuation actions.
    pub const fn is_entitlement_loss(self) -> bool {
        matches!(
            self,
            Self::TrialExpiry | Self::SeatDecrease | Self::PlanDowngrade | Self::Cancellation
        )
    }
}

/// A supported export path a buyer or auditor can use to recover user-owned data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPathClass {
    /// A direct local file export, available now and offline.
    LocalDirectExport,
    /// A signed offline bundle export (sovereign / air-gapped).
    OfflineBundleExport,
    /// A managed export deferred until the managed/mirror source returns.
    DeferredManagedExport,
    /// A support-assisted export when self-serve is unavailable.
    SupportAssistedExport,
    /// A read-only replay of an imported snapshot.
    ImportedSnapshotReplay,
}

impl ExportPathClass {
    /// All export paths, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::LocalDirectExport,
        Self::OfflineBundleExport,
        Self::DeferredManagedExport,
        Self::SupportAssistedExport,
        Self::ImportedSnapshotReplay,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDirectExport => "local_direct_export",
            Self::OfflineBundleExport => "offline_bundle_export",
            Self::DeferredManagedExport => "deferred_managed_export",
            Self::SupportAssistedExport => "support_assisted_export",
            Self::ImportedSnapshotReplay => "imported_snapshot_replay",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDirectExport => "Local direct export",
            Self::OfflineBundleExport => "Offline bundle export",
            Self::DeferredManagedExport => "Deferred managed export",
            Self::SupportAssistedExport => "Support-assisted export",
            Self::ImportedSnapshotReplay => "Imported snapshot replay",
        }
    }
}

/// The billing or owner scope a packet, card, or handoff is governed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingScopeClass {
    /// An individual, self-serve billing relationship.
    PersonalSelfServe,
    /// A workspace-billed relationship.
    WorkspaceBilled,
    /// An organization contract.
    OrgContract,
    /// A self-hosted license.
    SelfHostedLicense,
    /// A sovereign / air-gapped entitlement.
    SovereignEntitlement,
    /// A mirrored-entitlement relationship served from a last-synced mirror.
    MirroredEntitlement,
}

impl BillingScopeClass {
    /// All billing scopes, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::PersonalSelfServe,
        Self::WorkspaceBilled,
        Self::OrgContract,
        Self::SelfHostedLicense,
        Self::SovereignEntitlement,
        Self::MirroredEntitlement,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersonalSelfServe => "personal_self_serve",
            Self::WorkspaceBilled => "workspace_billed",
            Self::OrgContract => "org_contract",
            Self::SelfHostedLicense => "self_hosted_license",
            Self::SovereignEntitlement => "sovereign_entitlement",
            Self::MirroredEntitlement => "mirrored_entitlement",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PersonalSelfServe => "Personal (self-serve)",
            Self::WorkspaceBilled => "Workspace-billed",
            Self::OrgContract => "Organization contract",
            Self::SelfHostedLicense => "Self-hosted license",
            Self::SovereignEntitlement => "Sovereign entitlement",
            Self::MirroredEntitlement => "Mirrored entitlement",
        }
    }
}

/// One next action a renewal / trial / seat-change card offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextActionClass {
    /// Export user-owned data — a recovery action.
    ExportUserData,
    /// Delete user-owned data — a recovery action.
    DeleteUserData,
    /// Open support — a recovery action.
    OpenSupport,
    /// Continue local-only — a recovery action.
    ContinueLocalOnly,
    /// Review the renewal — a commercial call-to-action.
    ReviewRenewal,
    /// Contact billing — a commercial call-to-action.
    ContactBilling,
    /// Upgrade the plan — a commercial call-to-action.
    UpgradePlan,
}

impl NextActionClass {
    /// All next actions, in vocabulary order.
    pub const ALL: [Self; 7] = [
        Self::ExportUserData,
        Self::DeleteUserData,
        Self::OpenSupport,
        Self::ContinueLocalOnly,
        Self::ReviewRenewal,
        Self::ContactBilling,
        Self::UpgradePlan,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportUserData => "export_user_data",
            Self::DeleteUserData => "delete_user_data",
            Self::OpenSupport => "open_support",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::ReviewRenewal => "review_renewal",
            Self::ContactBilling => "contact_billing",
            Self::UpgradePlan => "upgrade_plan",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExportUserData => "Export user-owned data",
            Self::DeleteUserData => "Delete user-owned data",
            Self::OpenSupport => "Open support",
            Self::ContinueLocalOnly => "Continue local-only",
            Self::ReviewRenewal => "Review renewal",
            Self::ContactBilling => "Contact billing",
            Self::UpgradePlan => "Upgrade plan",
        }
    }

    /// Whether this is a recovery action — export, delete, support, or
    /// local-continuation — that a commercial call-to-action must never outrank.
    pub const fn is_recovery(self) -> bool {
        matches!(
            self,
            Self::ExportUserData
                | Self::DeleteUserData
                | Self::OpenSupport
                | Self::ContinueLocalOnly
        )
    }
}

/// The workspace archetype an admin-handoff packet describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceArchetypeClass {
    /// An individual, local-first workspace.
    IndividualWorkspace,
    /// A team workspace.
    TeamWorkspace,
    /// A managed-org workspace under a control plane.
    ManagedOrgWorkspace,
    /// A self-hosted workspace.
    SelfHostedWorkspace,
    /// A sovereign / air-gapped workspace.
    SovereignWorkspace,
    /// A mirrored workspace served from a last-synced mirror.
    MirroredWorkspace,
}

impl WorkspaceArchetypeClass {
    /// All workspace archetypes, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::IndividualWorkspace,
        Self::TeamWorkspace,
        Self::ManagedOrgWorkspace,
        Self::SelfHostedWorkspace,
        Self::SovereignWorkspace,
        Self::MirroredWorkspace,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualWorkspace => "individual_workspace",
            Self::TeamWorkspace => "team_workspace",
            Self::ManagedOrgWorkspace => "managed_org_workspace",
            Self::SelfHostedWorkspace => "self_hosted_workspace",
            Self::SovereignWorkspace => "sovereign_workspace",
            Self::MirroredWorkspace => "mirrored_workspace",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IndividualWorkspace => "Individual workspace",
            Self::TeamWorkspace => "Team workspace",
            Self::ManagedOrgWorkspace => "Managed-org workspace",
            Self::SelfHostedWorkspace => "Self-hosted workspace",
            Self::SovereignWorkspace => "Sovereign workspace",
            Self::MirroredWorkspace => "Mirrored workspace",
        }
    }
}

/// The kind of proof artifact an evidence ref points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKindClass {
    /// A signed release / offline verification packet.
    SignedReleasePacket,
    /// An admin audit export.
    AdminAuditExport,
    /// A destruction-receipt index from the retention/deletion lane.
    DestructionReceiptIndex,
    /// A policy-bundle / effective-policy proof.
    PolicyBundleProof,
    /// An endpoint-posture proof.
    EndpointPostureProof,
}

impl EvidenceKindClass {
    /// All evidence kinds, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::SignedReleasePacket,
        Self::AdminAuditExport,
        Self::DestructionReceiptIndex,
        Self::PolicyBundleProof,
        Self::EndpointPostureProof,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedReleasePacket => "signed_release_packet",
            Self::AdminAuditExport => "admin_audit_export",
            Self::DestructionReceiptIndex => "destruction_receipt_index",
            Self::PolicyBundleProof => "policy_bundle_proof",
            Self::EndpointPostureProof => "endpoint_posture_proof",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SignedReleasePacket => "Signed release packet",
            Self::AdminAuditExport => "Admin audit export",
            Self::DestructionReceiptIndex => "Destruction-receipt index",
            Self::PolicyBundleProof => "Policy-bundle proof",
            Self::EndpointPostureProof => "Endpoint-posture proof",
        }
    }

    /// The canonical boundary schema this evidence kind binds.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::SignedReleasePacket => "schemas/release/offline_verification_packet.schema.json",
            Self::AdminAuditExport => "schemas/admin/admin_audit_export.schema.json",
            Self::DestructionReceiptIndex => "schemas/admin/m5-retention-deletion.schema.json",
            Self::PolicyBundleProof => "schemas/admin/effective_policy_card.schema.json",
            Self::EndpointPostureProof => "schemas/admin/fleet_status_row.schema.json",
        }
    }
}

/// One canonical managed-state object family a packet, card, or handoff reuses
/// instead of restating it with drift-prone local copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalObjectClass {
    /// The effective-policy object family.
    EffectivePolicy,
    /// The entitlement / seat-lifecycle object family.
    EntitlementSeat,
    /// The retention / deletion matrix object family.
    RetentionDeletion,
    /// The offboarding / continuity object family.
    OffboardingContinuity,
    /// The endpoint-posture object family.
    EndpointPosture,
    /// The decision-history / audit object family.
    DecisionHistory,
}

impl CanonicalObjectClass {
    /// All canonical object families, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::EffectivePolicy,
        Self::EntitlementSeat,
        Self::RetentionDeletion,
        Self::OffboardingContinuity,
        Self::EndpointPosture,
        Self::DecisionHistory,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectivePolicy => "effective_policy",
            Self::EntitlementSeat => "entitlement_seat",
            Self::RetentionDeletion => "retention_deletion",
            Self::OffboardingContinuity => "offboarding_continuity",
            Self::EndpointPosture => "endpoint_posture",
            Self::DecisionHistory => "decision_history",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EffectivePolicy => "Effective policy",
            Self::EntitlementSeat => "Entitlement / seat",
            Self::RetentionDeletion => "Retention / deletion",
            Self::OffboardingContinuity => "Offboarding / continuity",
            Self::EndpointPosture => "Endpoint posture",
            Self::DecisionHistory => "Decision history",
        }
    }

    /// The canonical boundary schema this object family is rendered from.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::EffectivePolicy => "schemas/admin/effective_policy_card.schema.json",
            Self::EntitlementSeat => "schemas/admin/seat_lifecycle_row.schema.json",
            Self::RetentionDeletion => "schemas/admin/m5-retention-deletion.schema.json",
            Self::OffboardingContinuity => "schemas/admin/m5-offboarding.schema.json",
            Self::EndpointPosture => "schemas/admin/m5-admin-render.schema.json",
            Self::DecisionHistory => "schemas/admin/m5-decision-history.schema.json",
        }
    }
}

/// A residual dependency on a managed service a packet honestly discloses, with the
/// local-safe fallback that works without it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidualDependencyClass {
    /// A managed control plane.
    ManagedControlPlane,
    /// An identity / directory provider.
    IdentityProvider,
    /// A signing authority / trust root.
    SigningAuthority,
    /// A mirror upstream.
    MirrorUpstream,
    /// A billing processor.
    BillingProcessor,
}

impl ResidualDependencyClass {
    /// All residual-dependency classes, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::ManagedControlPlane,
        Self::IdentityProvider,
        Self::SigningAuthority,
        Self::MirrorUpstream,
        Self::BillingProcessor,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedControlPlane => "managed_control_plane",
            Self::IdentityProvider => "identity_provider",
            Self::SigningAuthority => "signing_authority",
            Self::MirrorUpstream => "mirror_upstream",
            Self::BillingProcessor => "billing_processor",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ManagedControlPlane => "Managed control plane",
            Self::IdentityProvider => "Identity provider",
            Self::SigningAuthority => "Signing authority",
            Self::MirrorUpstream => "Mirror upstream",
            Self::BillingProcessor => "Billing processor",
        }
    }
}

// ---------------------------------------------------------------------------
// Sub-records.
// ---------------------------------------------------------------------------

/// The validity window and labeling of a verification packet's signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidityWindow {
    /// When the validity window opens (ISO-8601 UTC).
    pub opens: String,
    /// When the validity window closes (ISO-8601 UTC).
    pub closes: String,
    /// Whether the packet is currently within its validity window.
    pub within_window: bool,
    /// One reviewable label for the window state (e.g. "Valid through 2026-12-31",
    /// "Past validity — labeled").
    pub window_label: String,
    /// One reviewable sentence stating the window rule and any labeled gap.
    pub note: String,
}

/// One supported export path a buyer or auditor can use to recover user-owned data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedExportPath {
    /// The export path class.
    pub path: ExportPathClass,
    /// One reviewable label.
    pub label: String,
    /// Where the export lands.
    pub lands_in: DataResidencyClass,
    /// Whether this path works fully offline.
    pub available_offline: bool,
    /// Whether this path needs a still-active paid seat (always false).
    pub requires_paid_seat: bool,
    /// One reviewable sentence describing the path.
    pub note: String,
}

/// One evidence ref — the proof artifact that backs current posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    /// Stable, opaque evidence id (export-safe).
    pub evidence_id: String,
    /// The kind of proof artifact.
    pub kind: EvidenceKindClass,
    /// The export-safe repo ref of the proof's boundary schema.
    pub schema_ref: String,
    /// One reviewable label.
    pub label: String,
    /// One reviewable sentence describing the evidence.
    pub note: String,
}

/// One canonical managed-state object the surface reuses by ref instead of
/// restating it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceRef {
    /// The canonical object family.
    pub object: CanonicalObjectClass,
    /// The export-safe repo ref of the family's boundary schema.
    pub schema_ref: String,
    /// One reviewable label.
    pub label: String,
    /// One reviewable sentence stating what is reused, not restated.
    pub note: String,
}

/// One residual dependency a packet honestly discloses, with its local-safe
/// fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependency {
    /// The residual-dependency class.
    pub dependency: ResidualDependencyClass,
    /// One reviewable label.
    pub label: String,
    /// What still works locally without this dependency.
    pub local_safe_fallback: String,
    /// One reviewable sentence describing the dependency.
    pub note: String,
}

/// The support / renewal handoff data a verification packet carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportHandoff {
    /// Who owns the support handoff.
    pub owner: OwnerEscalationRoleClass,
    /// Who owns the renewal.
    pub renewal_owner: OwnerEscalationRoleClass,
    /// The plain-language next step for a support or renewal handoff.
    pub next_step: String,
    /// The plain-language support route (never a URL, host, or credential).
    pub support_route: String,
    /// One reviewable sentence describing the handoff.
    pub note: String,
}

/// One ordered next action on a renewal / trial / seat-change card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventNextAction {
    /// The action class.
    pub action: NextActionClass,
    /// The 1-based rank in the ordered action list.
    pub order: u32,
    /// One reviewable label.
    pub label: String,
    /// Whether this is a recovery action (export, delete, support, local
    /// continuation) that no commercial call-to-action may outrank.
    pub recovery_action: bool,
    /// One reviewable sentence describing the action.
    pub note: String,
}

// ---------------------------------------------------------------------------
// The three rendered objects: verification packet, event card, admin handoff.
// ---------------------------------------------------------------------------

/// A rendered procurement / verification packet for one profile: the metadata-safe
/// posture proof a buyer or auditor needs, with deployment mode, supported export
/// paths, billing/owner scope, validity-window and signature posture, evidence
/// refs, residual dependencies, the canonical objects it reuses, and the
/// support/renewal handoff data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementPacket {
    /// Stable, opaque packet id (export-safe).
    pub packet_id: String,
    /// The deployment mode this packet proves.
    pub deployment_mode: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The verification / signature posture of the packet.
    pub verification_posture: VerificationPostureClass,
    /// The machine-readable state (must be one the matrix admits for this surface).
    pub machine_state: AdminStateClass,
    /// The validity window and its labeling.
    pub validity_window: ValidityWindow,
    /// The freshness of the evidence backing the packet.
    pub evidence_age: EvidenceAgeClass,
    /// The billing / owner scope this packet is governed under.
    pub billing_scope: BillingScopeClass,
    /// Who owns the packet.
    pub packet_owner: OwnerEscalationRoleClass,
    /// The as-of date the packet's posture was evaluated (ISO-8601 UTC).
    pub as_of: String,
    /// The supported export paths.
    pub supported_export_paths: Vec<SupportedExportPath>,
    /// The evidence refs that back current posture.
    pub evidence_refs: Vec<EvidenceRef>,
    /// The residual dependencies, honestly disclosed (empty when none remain).
    pub residual_dependencies: Vec<ResidualDependency>,
    /// The canonical managed-state objects this packet reuses by ref.
    pub canonical_sources: Vec<CanonicalSourceRef>,
    /// The support / renewal handoff data.
    pub support_handoff: SupportHandoff,
    /// Whether recovering user-owned data via export needs a still-active paid seat
    /// (always false).
    pub requires_paid_seat_for_export: bool,
    /// The schema that governs this packet.
    pub governing_schema_ref: String,
    /// One reviewable sentence noting how the packet is schema-governed.
    pub schema_note: String,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/procurement handoff sentence.
    pub plain_language: String,
}

impl ProcurementPacket {
    /// Whether the packet carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }

    /// Whether the packet asserts a currently-verified signature.
    pub fn is_verified_now(&self) -> bool {
        self.verification_posture.is_verified_now()
    }
}

/// A rendered renewal / trial / seat-change summary card for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommercialEventCard {
    /// Stable, opaque card id (export-safe).
    pub card_id: String,
    /// The commercial event this card discloses.
    pub event: CommercialEventClass,
    /// One reviewable label.
    pub label: String,
    /// When the event takes effect (ISO-8601 UTC).
    pub effective_date: String,
    /// The as-of date this card was evaluated (ISO-8601 UTC).
    pub as_of: String,
    /// The impacted managed features, in plain language.
    pub impacted_features: String,
    /// The billing / owner scope the event impacts.
    pub impacted_scope: BillingScopeClass,
    /// The machine-readable state (must be one the matrix admits for this surface).
    pub machine_state: AdminStateClass,
    /// The freshness of the evidence backing the card.
    pub evidence_age: EvidenceAgeClass,
    /// Whether this event reduces entitlements.
    pub entitlement_loss: bool,
    /// The local-only continuation path, in plain language.
    pub local_only_path: String,
    /// The ordered next actions; recovery actions always precede commercial CTAs.
    pub next_actions: Vec<EventNextAction>,
    /// Whether the commercial card outranks the recovery actions (always false).
    pub outranks_recovery_actions: bool,
    /// The plain-language export next step.
    pub export_next_step: String,
    /// The plain-language support next step.
    pub support_next_step: String,
    /// The canonical managed-state objects this card reuses by ref.
    pub canonical_sources: Vec<CanonicalSourceRef>,
    /// Whether recovering user-owned data needs a still-active paid seat (always
    /// false).
    pub requires_paid_seat_for_recovery: bool,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

impl CommercialEventCard {
    /// The maximum order among recovery actions, if any.
    pub fn max_recovery_order(&self) -> Option<u32> {
        self.next_actions
            .iter()
            .filter(|a| a.recovery_action)
            .map(|a| a.order)
            .max()
    }

    /// The minimum order among commercial (non-recovery) actions, if any.
    pub fn min_commercial_order(&self) -> Option<u32> {
        self.next_actions
            .iter()
            .filter(|a| !a.recovery_action)
            .map(|a| a.order)
            .min()
    }

    /// Whether every recovery action precedes every commercial call-to-action and
    /// the card never claims to outrank recovery.
    pub fn recovery_outranks_commercial(&self) -> bool {
        if self.outranks_recovery_actions {
            return false;
        }
        match (self.max_recovery_order(), self.min_commercial_order()) {
            (Some(max_recovery), Some(min_commercial)) => max_recovery < min_commercial,
            // No commercial CTA, or no recovery action — trivially satisfied for
            // the ordering check; presence is enforced separately.
            _ => true,
        }
    }

    /// Whether the card carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }
}

/// A rendered admin-handoff packet for one profile: build/channel, install mode,
/// workspace archetype, bundle ids, affected features, and an export-safe summary,
/// auto-derived from current managed state without manual curation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminHandoffPacket {
    /// Stable, opaque handoff id (export-safe).
    pub handoff_id: String,
    /// The opaque build ref (a stable token, never a path).
    pub build_ref: String,
    /// The release channel / update ring.
    pub channel: UpdateRingClass,
    /// The install mode.
    pub install_mode: InstallModeClass,
    /// The workspace archetype.
    pub workspace_archetype: WorkspaceArchetypeClass,
    /// The opaque bundle ids in scope.
    pub bundle_ids: Vec<String>,
    /// The affected managed features, in plain language.
    pub affected_features: Vec<String>,
    /// The machine-readable state (must be one the matrix admits for this surface).
    pub machine_state: AdminStateClass,
    /// The as-of date this handoff was evaluated (ISO-8601 UTC).
    pub as_of: String,
    /// Who owns the handoff.
    pub handoff_owner: OwnerEscalationRoleClass,
    /// The canonical managed-state objects this handoff reuses by ref.
    pub canonical_sources: Vec<CanonicalSourceRef>,
    /// The export-safe summary of the handoff.
    pub export_safe_summary: String,
    /// Whether the handoff is auto-derived from current state (always true).
    pub auto_derived: bool,
    /// The schema that governs this handoff.
    pub governing_schema_ref: String,
    /// One reviewable sentence noting how the handoff is schema-governed.
    pub schema_note: String,
    /// The export-safe machine-readable summary (stable tokens, never a secret).
    pub machine_summary: String,
    /// The plain-language support/admin handoff sentence.
    pub plain_language: String,
}

impl AdminHandoffPacket {
    /// Whether the handoff carries both export representations.
    pub fn has_export_parity(&self) -> bool {
        !self.machine_summary.is_empty() && !self.plain_language.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Coverage, per-profile packet, and the bundle.
// ---------------------------------------------------------------------------

/// The coverage posture of a rendered procurement surface for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementCoverage {
    /// The coverage state (must be one the matrix admits for this surface).
    pub coverage_state: AdminStateClass,
    /// How complete the rendered surface is.
    pub completeness: CompletenessClass,
    /// One reviewable label for the coverage window.
    pub window_label: String,
    /// One reviewable sentence stating the coverage rule and any labeled gap.
    pub coverage_note: String,
    /// Whether the surface is locally inspectable on this profile.
    pub locally_inspectable: bool,
    /// Whether the surface is available without a vendor console / control plane.
    pub vendor_console_independent: bool,
    /// Whether user-owned data is exportable without a still-active paid seat
    /// (always true).
    pub exportable_without_paid_seat: bool,
}

/// The rendered procurement surface for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementProfilePacket {
    /// The admin path / profile this packet renders.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The surface family (always
    /// [`AdminSurfaceClass::ProcurementVerificationPacket`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The procurement / verification packet.
    pub verification_packet: ProcurementPacket,
    /// The renewal / trial / seat-change summary cards.
    pub event_cards: Vec<CommercialEventCard>,
    /// The admin-handoff packet.
    pub admin_handoff: AdminHandoffPacket,
    /// The export forms offered.
    pub export_forms: Vec<ExportForm>,
    /// The coverage posture of the surface.
    pub coverage: ProcurementCoverage,
}

impl ProcurementProfilePacket {
    /// Whether the packet renders an event card for the given event class.
    pub fn has_event(&self, event: CommercialEventClass) -> bool {
        self.event_cards.iter().any(|c| c.event == event)
    }

    /// Whether the packet offers a given export format.
    pub fn offers(&self, format: ExportFormatClass) -> bool {
        self.export_forms.iter().any(|f| f.format == format)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the rendered bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen procurement bundle: one packet per claimed managed-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_procurement_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this render layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this render layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile procurement packets.
    pub profiles: Vec<ProcurementProfilePacket>,
    /// The computed invariants.
    pub invariants: Vec<ProcurementInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcurementValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for ProcurementValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "procurement bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for ProcurementValidationError {}

/// The profiles the procurement bundle covers, in bundle order.
pub const PROCUREMENT_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

impl ProcurementBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&ProcurementProfilePacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque token, never
    /// a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing. Stable token ids are audited separately by [`is_safe_token`].
    fn file_refs(&self) -> Vec<&str> {
        let mut refs = vec![
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ];
        for p in &self.profiles {
            let vp = &p.verification_packet;
            refs.push(vp.governing_schema_ref.as_str());
            for e in &vp.evidence_refs {
                refs.push(e.schema_ref.as_str());
            }
            for c in &vp.canonical_sources {
                refs.push(c.schema_ref.as_str());
            }
            for card in &p.event_cards {
                for c in &card.canonical_sources {
                    refs.push(c.schema_ref.as_str());
                }
            }
            let h = &p.admin_handoff;
            refs.push(h.governing_schema_ref.as_str());
            for c in &h.canonical_sources {
                refs.push(c.schema_ref.as_str());
            }
        }
        refs
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            ids.push(p.surface_id.as_str());
            let vp = &p.verification_packet;
            ids.push(vp.packet_id.as_str());
            for e in &vp.evidence_refs {
                ids.push(e.evidence_id.as_str());
            }
            for card in &p.event_cards {
                ids.push(card.card_id.as_str());
            }
            let h = &p.admin_handoff;
            ids.push(h.handoff_id.as_str());
            ids.push(h.build_ref.as_str());
            for b in &h.bundle_ids {
                ids.push(b.as_str());
            }
            for x in &p.export_forms {
                ids.push(x.artifact_ref.as_str());
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`ProcurementInvariant`]s with the coverage and
    /// resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), ProcurementValidationError> {
        let fail = |reason: String| Err(ProcurementValidationError { reason });

        if self.record_kind != M5_PROCUREMENT_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_PROCUREMENT_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in PROCUREMENT_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet).map_err(|reason| ProcurementValidationError { reason })?;
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Whether a state requires confirmed-fresh evidence — the confirmed-green class
/// for this surface. A stale-evidence object must never sit under such a state.
fn requires_fresh_evidence(state: AdminStateClass) -> bool {
    matches!(state, AdminStateClass::ActiveEnforced)
}

fn validate_packet(packet: &ProcurementProfilePacket) -> Result<(), String> {
    let profile = packet.profile.as_str();
    if packet.surface != AdminSurfaceClass::ProcurementVerificationPacket {
        return Err(format!("{profile}: unexpected surface"));
    }
    if packet.surface_id != AdminSurfaceClass::ProcurementVerificationPacket.surface_id() {
        return Err(format!("{profile}: unexpected surface_id"));
    }
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!("{profile}: unexpected profile_id"));
    }

    let vp = &packet.verification_packet;
    if vp.supported_export_paths.is_empty() {
        return Err(format!("{profile}: verification packet has no export path"));
    }
    if vp.evidence_refs.is_empty() {
        return Err(format!(
            "{profile}: verification packet has no evidence ref"
        ));
    }
    if vp.canonical_sources.is_empty() {
        return Err(format!(
            "{profile}: verification packet reuses no canonical source"
        ));
    }
    if !vp.has_export_parity() {
        return Err(format!(
            "{profile}: verification packet lacks export parity"
        ));
    }

    if packet.event_cards.is_empty() {
        return Err(format!("{profile}: no renewal/trial/seat-change card"));
    }
    for card in &packet.event_cards {
        if !card.recovery_outranks_commercial() {
            return Err(format!(
                "{profile}: event card {} lets a commercial action outrank recovery",
                card.event.as_str()
            ));
        }
        if card.canonical_sources.is_empty() {
            return Err(format!(
                "{profile}: event card {} reuses no canonical source",
                card.event.as_str()
            ));
        }
        if !card.has_export_parity() {
            return Err(format!(
                "{profile}: event card {} lacks export parity",
                card.event.as_str()
            ));
        }
    }

    let h = &packet.admin_handoff;
    if h.bundle_ids.is_empty() {
        return Err(format!("{profile}: admin handoff has no bundle ids"));
    }
    if h.affected_features.is_empty() {
        return Err(format!(
            "{profile}: admin handoff names no affected features"
        ));
    }
    if h.canonical_sources.is_empty() {
        return Err(format!(
            "{profile}: admin handoff reuses no canonical source"
        ));
    }
    if !h.has_export_parity() {
        return Err(format!("{profile}: admin handoff lacks export parity"));
    }

    if !packet.offers(ExportFormatClass::MachineReadableJson)
        || !packet.offers(ExportFormatClass::PlainLanguageHandoff)
    {
        return Err(format!("{profile}: missing an export form"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical procurement bundle.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the rendered packets, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn procurement_bundle() -> ProcurementBundle {
    let profiles: Vec<ProcurementProfilePacket> = PROCUREMENT_PROFILES
        .iter()
        .map(|p| profile_packet(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    ProcurementBundle {
        record_kind: M5_PROCUREMENT_RECORD_KIND.to_owned(),
        m5_procurement_schema_version: M5_PROCUREMENT_SCHEMA_VERSION,
        schema_ref: M5_PROCUREMENT_SCHEMA_REF.to_owned(),
        bundle_id: M5_PROCUREMENT_BUNDLE_ID.to_owned(),
        as_of: M5_PROCUREMENT_AS_OF.to_owned(),
        matrix_ref: M5_PROCUREMENT_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_PROCUREMENT_FREEZE_GATE_REF.to_owned(),
        summary:
            "Rendered procurement / verification packets, renewal / trial / seat-change summary \
             cards, and admin-handoff packets — bound back to the frozen admin-plane matrix and \
             rendered identically for commercial/procurement, Help/About, support export, release \
             evidence, and managed-service consumers across the managed-cloud, self-hosted, \
             sovereign/air-gapped, and mirrored/offline profiles. Each verification packet names \
             its deployment mode, supported export paths, billing/owner scope, validity window and \
             signature posture, evidence refs, residual dependencies, and support/renewal handoff \
             data; each event card discloses its effective date, impacted features, as-of date, \
             local-only path, and export/support next step while never outranking the export, \
             delete, support, or local-continuation actions; and each admin-handoff packet carries \
             build/channel, install mode, workspace archetype, bundle ids, and affected features. \
             Every surface reuses the same canonical managed-state objects by ref instead of \
             restating them, stays locally inspectable without a vendor console, and keeps export \
             reachable without a still-active paid seat."
                .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

/// The consumers every packet must serve identically; mirrors the matrix's
/// declared consumers for the procurement surface.
fn parity_consumers() -> Vec<AdminConsumerClass> {
    admin_plane_matrix()
        .surface(AdminSurfaceClass::ProcurementVerificationPacket)
        .map(|entry| entry.consumed_by.clone())
        .unwrap_or_default()
}

fn profile_packet(profile: AdminPathClass) -> ProcurementProfilePacket {
    let surface = AdminSurfaceClass::ProcurementVerificationPacket;
    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: a signed, currently-verified verification packet proves posture \
             live; renewal and seat-increase cards keep export, delete, support, and \
             local-continuation ahead of the renewal call-to-action; and the admin-handoff packet \
             is auto-derived from current managed state.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer's own control plane signs and verifies the packet; \
             a trial-start and a plan-downgrade card disclose impact while keeping recovery actions \
             first, and the handoff reuses the canonical managed-state objects by ref.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: the signed offline bundle is past its validity window \
             offline, so the packet is labeled signature-unverified rather than shown verified; the \
             trial-expiry and seat-decrease cards still recover user-owned data offline without a \
             paid seat.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the mirror serves a last-synced packet whose current \
             validity cannot be confirmed offline, so it is labeled unconfirmed-stale; the \
             cancellation card keeps local export and continuation reachable while the upstream is \
             unreachable.",
        ),
        _ => (
            AdminDeploymentProfileClass::IndividualLocal,
            "Local profile.",
        ),
    };

    ProcurementProfilePacket {
        profile,
        profile_id: profile.path_id(),
        surface,
        surface_id: surface.surface_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers: parity_consumers(),
        verification_packet: build_verification_packet(profile),
        event_cards: build_event_cards(profile),
        admin_handoff: build_admin_handoff(profile),
        export_forms: build_export_forms(profile),
        coverage: build_coverage(profile),
    }
}

// ---------------------------------------------------------------------------
// Per-profile rendering parameters.
// ---------------------------------------------------------------------------

/// The per-profile posture parameters that keep the three rendered objects
/// consistent for a profile.
struct ProfileParams {
    deployment: AdminDeploymentProfileClass,
    state: AdminStateClass,
    evidence_age: EvidenceAgeClass,
    posture: VerificationPostureClass,
    completeness: CompletenessClass,
    billing: BillingScopeClass,
    archetype: WorkspaceArchetypeClass,
    install: InstallModeClass,
    channel: UpdateRingClass,
    events: &'static [CommercialEventClass],
}

fn params(profile: AdminPathClass) -> ProfileParams {
    use AdminPathClass::*;
    match profile {
        ManagedCloud => ProfileParams {
            deployment: AdminDeploymentProfileClass::ManagedCloud,
            state: AdminStateClass::ActiveEnforced,
            evidence_age: EvidenceAgeClass::Fresh,
            posture: VerificationPostureClass::SignedVerified,
            completeness: CompletenessClass::Complete,
            billing: BillingScopeClass::OrgContract,
            archetype: WorkspaceArchetypeClass::ManagedOrgWorkspace,
            install: InstallModeClass::ManagedImage,
            channel: UpdateRingClass::PinnedManaged,
            events: &[
                CommercialEventClass::Renewal,
                CommercialEventClass::SeatIncrease,
            ],
        },
        SelfHosted => ProfileParams {
            deployment: AdminDeploymentProfileClass::SelfHosted,
            state: AdminStateClass::ActiveEnforced,
            evidence_age: EvidenceAgeClass::Fresh,
            posture: VerificationPostureClass::SignedVerified,
            completeness: CompletenessClass::Complete,
            billing: BillingScopeClass::SelfHostedLicense,
            archetype: WorkspaceArchetypeClass::SelfHostedWorkspace,
            install: InstallModeClass::PerMachine,
            channel: UpdateRingClass::Extended,
            events: &[
                CommercialEventClass::TrialStart,
                CommercialEventClass::PlanDowngrade,
            ],
        },
        SovereignAirGapped => ProfileParams {
            deployment: AdminDeploymentProfileClass::SovereignAirGapped,
            state: AdminStateClass::SignatureUnverified,
            evidence_age: EvidenceAgeClass::Stale,
            posture: VerificationPostureClass::SignatureExpired,
            completeness: CompletenessClass::PartialOffline,
            billing: BillingScopeClass::SovereignEntitlement,
            archetype: WorkspaceArchetypeClass::SovereignWorkspace,
            install: InstallModeClass::SovereignImage,
            channel: UpdateRingClass::PinnedOffline,
            events: &[
                CommercialEventClass::TrialExpiry,
                CommercialEventClass::SeatDecrease,
            ],
        },
        MirroredOffline => ProfileParams {
            deployment: AdminDeploymentProfileClass::ManagedCloud,
            state: AdminStateClass::UnconfirmedStale,
            evidence_age: EvidenceAgeClass::Stale,
            posture: VerificationPostureClass::UnverifiableOffline,
            completeness: CompletenessClass::PartialOffline,
            billing: BillingScopeClass::MirroredEntitlement,
            archetype: WorkspaceArchetypeClass::MirroredWorkspace,
            install: InstallModeClass::ManagedImage,
            channel: UpdateRingClass::PinnedOffline,
            events: &[CommercialEventClass::Cancellation],
        },
        _ => ProfileParams {
            deployment: AdminDeploymentProfileClass::IndividualLocal,
            state: AdminStateClass::ActiveEnforced,
            evidence_age: EvidenceAgeClass::Fresh,
            posture: VerificationPostureClass::UnsignedLocal,
            completeness: CompletenessClass::Complete,
            billing: BillingScopeClass::PersonalSelfServe,
            archetype: WorkspaceArchetypeClass::IndividualWorkspace,
            install: InstallModeClass::PerUser,
            channel: UpdateRingClass::Stable,
            events: &[CommercialEventClass::Renewal],
        },
    }
}

// ---------------------------------------------------------------------------
// Builders.
// ---------------------------------------------------------------------------

/// A stable token for a deployment profile, for metadata-safe summaries.
fn deployment_token(deployment: AdminDeploymentProfileClass) -> &'static str {
    match deployment {
        AdminDeploymentProfileClass::IndividualLocal => "individual_local",
        AdminDeploymentProfileClass::SelfHosted => "self_hosted",
        AdminDeploymentProfileClass::EnterpriseOnline => "enterprise_online",
        AdminDeploymentProfileClass::SovereignAirGapped => "sovereign_air_gapped",
        AdminDeploymentProfileClass::ManagedCloud => "managed_cloud",
    }
}

fn canonical_source(object: CanonicalObjectClass, note: &str) -> CanonicalSourceRef {
    CanonicalSourceRef {
        object,
        schema_ref: object.schema_ref().to_owned(),
        label: object.label().to_owned(),
        note: note.to_owned(),
    }
}

fn evidence_ref(profile_token: &str, kind: EvidenceKindClass, note: &str) -> EvidenceRef {
    EvidenceRef {
        evidence_id: format!("procurement.evidence.{profile_token}.{}", kind.as_str()),
        kind,
        schema_ref: kind.schema_ref().to_owned(),
        label: kind.label().to_owned(),
        note: note.to_owned(),
    }
}

fn export_path(
    path: ExportPathClass,
    lands_in: DataResidencyClass,
    available_offline: bool,
    note: &str,
) -> SupportedExportPath {
    SupportedExportPath {
        path,
        label: path.label().to_owned(),
        lands_in,
        available_offline,
        requires_paid_seat: false,
        note: note.to_owned(),
    }
}

fn build_verification_packet(profile: AdminPathClass) -> ProcurementPacket {
    let p = params(profile);
    let profile_token = profile.as_str();

    let (validity_window, residual_dependencies, support_handoff, summary, plain) = match profile {
        AdminPathClass::ManagedCloud => (
            ValidityWindow {
                opens: "2026-01-01T00:00:00Z".to_owned(),
                closes: "2026-12-31T23:59:59Z".to_owned(),
                within_window: true,
                window_label: "Valid through 2026-12-31".to_owned(),
                note: "The packet is signed and within its validity window, verified against a \
                       current trust root."
                    .to_owned(),
            },
            vec![
                ResidualDependency {
                    dependency: ResidualDependencyClass::ManagedControlPlane,
                    label: ResidualDependencyClass::ManagedControlPlane.label().to_owned(),
                    local_safe_fallback: "Local export and continuation complete without the \
                                          control plane."
                        .to_owned(),
                    note: "Live managed posture refreshes from the control plane; export stays \
                           local-safe if it is unreachable."
                        .to_owned(),
                },
                ResidualDependency {
                    dependency: ResidualDependencyClass::BillingProcessor,
                    label: ResidualDependencyClass::BillingProcessor.label().to_owned(),
                    local_safe_fallback: "Renewal and billing are handled out of band; data \
                                          recovery never depends on them."
                        .to_owned(),
                    note: "Billing changes are commercial only and never gate user-owned data \
                           recovery."
                        .to_owned(),
                },
            ],
            SupportHandoff {
                owner: OwnerEscalationRoleClass::OrgAdmin,
                renewal_owner: OwnerEscalationRoleClass::OrgAdmin,
                next_step: "Forward this packet with the renewal owner for procurement review."
                    .to_owned(),
                support_route: "Org admin opens a support handoff from the admin center."
                    .to_owned(),
                note: "Renewal and support handoff are owned by the org admin and do not gate \
                       export."
                    .to_owned(),
            },
            "Managed-cloud verification packet: signed, currently verified, within validity, with \
             local and support-assisted export paths and the canonical managed-state objects \
             reused by ref.",
            "Posture is signed and verified live; export user-owned data locally or via support, \
             and forward this packet for procurement or renewal review.",
        ),
        AdminPathClass::SelfHosted => (
            ValidityWindow {
                opens: "2026-01-01T00:00:00Z".to_owned(),
                closes: "2027-01-01T00:00:00Z".to_owned(),
                within_window: true,
                window_label: "Valid through 2027-01-01 (self-hosted trust root)".to_owned(),
                note: "The packet is signed by the customer's own trust root and within its \
                       validity window."
                    .to_owned(),
            },
            vec![
                ResidualDependency {
                    dependency: ResidualDependencyClass::IdentityProvider,
                    label: ResidualDependencyClass::IdentityProvider.label().to_owned(),
                    local_safe_fallback: "Local-only continuation works while the directory is \
                                          unreachable."
                        .to_owned(),
                    note: "Managed sign-in depends on the customer directory; local recovery does \
                           not."
                        .to_owned(),
                },
                ResidualDependency {
                    dependency: ResidualDependencyClass::SigningAuthority,
                    label: ResidualDependencyClass::SigningAuthority.label().to_owned(),
                    local_safe_fallback: "The pinned trust root verifies the packet locally."
                        .to_owned(),
                    note: "Signature verification uses the customer's pinned trust root, on the \
                           machine."
                        .to_owned(),
                },
            ],
            SupportHandoff {
                owner: OwnerEscalationRoleClass::OrgAdmin,
                renewal_owner: OwnerEscalationRoleClass::OrgAdmin,
                next_step: "Hand this packet to the self-hosted operator for license review."
                    .to_owned(),
                support_route: "Operator opens an internal support handoff from the admin center."
                    .to_owned(),
                note: "Support and renewal are owned by the self-hosted operator and never block \
                       export."
                    .to_owned(),
            },
            "Self-hosted verification packet: signed and verified against the customer trust root, \
             within validity, reusing the canonical managed-state objects by ref.",
            "Posture is signed and verified by the customer trust root; export locally and hand \
             the packet to the operator for license review.",
        ),
        AdminPathClass::SovereignAirGapped => (
            ValidityWindow {
                opens: "2025-01-01T00:00:00Z".to_owned(),
                closes: "2025-12-31T23:59:59Z".to_owned(),
                within_window: false,
                window_label: "Past validity (2025-12-31) — labeled, not shown verified".to_owned(),
                note: "The signed offline bundle is past its validity window and revocation cannot \
                       be confirmed offline, so the packet is labeled unverified rather than shown \
                       currently verified."
                    .to_owned(),
            },
            vec![ResidualDependency {
                dependency: ResidualDependencyClass::SigningAuthority,
                label: ResidualDependencyClass::SigningAuthority.label().to_owned(),
                local_safe_fallback: "The pinned trust root still verifies the bundle's signature \
                                      locally; only current validity is unconfirmable."
                    .to_owned(),
                note: "Air-gapped installs verify the signature against a pinned trust root but \
                       cannot confirm the current validity window."
                    .to_owned(),
            }],
            SupportHandoff {
                owner: OwnerEscalationRoleClass::SecurityOwner,
                renewal_owner: OwnerEscalationRoleClass::OrgAdmin,
                next_step: "Import a refreshed signed bundle to restore a currently-verified \
                            packet; export is unaffected meanwhile."
                    .to_owned(),
                support_route: "Security owner handles the offline bundle refresh out of band."
                    .to_owned(),
                note: "Renewal requires an offline bundle refresh; export stays available without \
                       it."
                    .to_owned(),
            },
            "Sovereign / air-gapped verification packet: signed offline bundle past its validity \
             window, labeled unverified rather than shown verified, with offline export paths and \
             the canonical objects reused by ref.",
            "The offline bundle is past validity and labeled unverified; export user-owned data \
             offline and refresh the signed bundle to restore verified posture.",
        ),
        AdminPathClass::MirroredOffline => (
            ValidityWindow {
                opens: "2026-01-01T00:00:00Z".to_owned(),
                closes: "2026-12-31T23:59:59Z".to_owned(),
                within_window: true,
                window_label: "Last-synced — current validity unconfirmable offline".to_owned(),
                note: "The mirror serves a last-synced packet whose current validity cannot be \
                       confirmed while the upstream is unreachable, so it is labeled \
                       unconfirmed-stale rather than shown verified."
                    .to_owned(),
            },
            vec![
                ResidualDependency {
                    dependency: ResidualDependencyClass::MirrorUpstream,
                    label: ResidualDependencyClass::MirrorUpstream.label().to_owned(),
                    local_safe_fallback: "The last-synced mirror is shown read-only and labeled; \
                                          export works from it."
                        .to_owned(),
                    note: "Current posture refreshes from the mirror upstream when it reconnects."
                        .to_owned(),
                },
                ResidualDependency {
                    dependency: ResidualDependencyClass::ManagedControlPlane,
                    label: ResidualDependencyClass::ManagedControlPlane.label().to_owned(),
                    local_safe_fallback: "Local export and continuation complete without the \
                                          control plane."
                        .to_owned(),
                    note: "The control plane is reached through the mirror; recovery never \
                           depends on it."
                        .to_owned(),
                },
            ],
            SupportHandoff {
                owner: OwnerEscalationRoleClass::OrgAdmin,
                renewal_owner: OwnerEscalationRoleClass::OrgAdmin,
                next_step: "Reconnect the mirror to confirm current validity; export and \
                            continuation are unaffected meanwhile."
                    .to_owned(),
                support_route: "Org admin opens a support handoff once the mirror reconnects."
                    .to_owned(),
                note: "Renewal review waits for the mirror to reconnect; export stays local-safe."
                    .to_owned(),
            },
            "Mirrored / offline verification packet: last-synced and labeled unconfirmed-stale \
             rather than shown verified, with local export paths and the canonical objects reused \
             by ref.",
            "The mirror packet is last-synced and labeled unconfirmed; export user-owned data \
             locally and reconnect the mirror to confirm current validity.",
        ),
        _ => (
            ValidityWindow {
                opens: M5_PROCUREMENT_AS_OF.to_owned(),
                closes: M5_PROCUREMENT_AS_OF.to_owned(),
                within_window: true,
                window_label: "Local".to_owned(),
                note: "Local packet.".to_owned(),
            },
            Vec::new(),
            SupportHandoff {
                owner: OwnerEscalationRoleClass::LocalUser,
                renewal_owner: OwnerEscalationRoleClass::LocalUser,
                next_step: "Local.".to_owned(),
                support_route: "Local.".to_owned(),
                note: "Local.".to_owned(),
            },
            "Local verification packet.",
            "Local.",
        ),
    };

    ProcurementPacket {
        packet_id: format!("procurement.packet.{profile_token}"),
        deployment_mode: p.deployment,
        summary: summary.to_owned(),
        verification_posture: p.posture,
        machine_state: p.state,
        validity_window,
        evidence_age: p.evidence_age,
        billing_scope: p.billing,
        packet_owner: OwnerEscalationRoleClass::OrgAdmin,
        as_of: M5_PROCUREMENT_AS_OF.to_owned(),
        supported_export_paths: build_export_paths(profile),
        evidence_refs: build_evidence_refs(profile),
        residual_dependencies,
        canonical_sources: build_packet_sources(),
        support_handoff,
        requires_paid_seat_for_export: false,
        governing_schema_ref: "schemas/release/offline_verification_packet.schema.json".to_owned(),
        schema_note: "The packet's signature posture and validity window are governed by the \
                      offline verification-packet schema; a past-validity or unverifiable packet \
                      is labeled, never shown verified."
            .to_owned(),
        machine_summary: format!(
            "packet={} deployment={} state={} posture={} billing={} age={}",
            profile_token,
            deployment_token(p.deployment),
            p.state.as_str(),
            p.posture.as_str(),
            p.billing.as_str(),
            p.evidence_age.as_str(),
        ),
        plain_language: plain.to_owned(),
    }
}

fn build_export_paths(profile: AdminPathClass) -> Vec<SupportedExportPath> {
    use AdminPathClass::*;
    match profile {
        ManagedCloud => vec![
            export_path(
                ExportPathClass::LocalDirectExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export user-owned artifacts to a local snapshot now, offline, with no paid seat.",
            ),
            export_path(
                ExportPathClass::SupportAssistedExport,
                DataResidencyClass::ExportedSnapshot,
                false,
                "Support can assist an export if self-serve is unavailable.",
            ),
        ],
        SelfHosted => vec![
            export_path(
                ExportPathClass::LocalDirectExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export user-owned artifacts to a local snapshot now, offline, with no paid seat.",
            ),
            export_path(
                ExportPathClass::OfflineBundleExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export a signed offline bundle from the self-hosted plane.",
            ),
        ],
        SovereignAirGapped => vec![
            export_path(
                ExportPathClass::OfflineBundleExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export a signed offline bundle; completes fully air-gapped with no paid seat.",
            ),
            export_path(
                ExportPathClass::LocalDirectExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export user-owned artifacts to a local snapshot now, offline.",
            ),
        ],
        MirroredOffline => vec![
            export_path(
                ExportPathClass::LocalDirectExport,
                DataResidencyClass::ExportedSnapshot,
                true,
                "Export user-owned artifacts to a local snapshot now, even with the mirror offline.",
            ),
            export_path(
                ExportPathClass::DeferredManagedExport,
                DataResidencyClass::MirroredCopy,
                false,
                "A managed export is queued to complete when the mirror reconnects; never lost.",
            ),
        ],
        _ => vec![export_path(
            ExportPathClass::LocalDirectExport,
            DataResidencyClass::ExportedSnapshot,
            true,
            "Local export.",
        )],
    }
}

fn build_evidence_refs(profile: AdminPathClass) -> Vec<EvidenceRef> {
    let t = profile.as_str();
    use AdminPathClass::*;
    match profile {
        ManagedCloud => vec![
            evidence_ref(
                t,
                EvidenceKindClass::SignedReleasePacket,
                "The signed release / offline verification packet proving build and signature.",
            ),
            evidence_ref(
                t,
                EvidenceKindClass::AdminAuditExport,
                "The admin audit export proving the current admin-plane decisions.",
            ),
            evidence_ref(
                t,
                EvidenceKindClass::PolicyBundleProof,
                "The effective-policy proof showing the active policy source.",
            ),
        ],
        SelfHosted => vec![
            evidence_ref(
                t,
                EvidenceKindClass::SignedReleasePacket,
                "The customer-signed release packet proving build and signature.",
            ),
            evidence_ref(
                t,
                EvidenceKindClass::DestructionReceiptIndex,
                "The destruction-receipt index proving retention/deletion posture.",
            ),
        ],
        SovereignAirGapped => vec![
            evidence_ref(
                t,
                EvidenceKindClass::SignedReleasePacket,
                "The signed offline bundle; its signature verifies locally but its validity is \
                 past, so the packet is labeled unverified.",
            ),
            evidence_ref(
                t,
                EvidenceKindClass::EndpointPostureProof,
                "The endpoint-posture proof for the air-gapped install.",
            ),
        ],
        MirroredOffline => vec![
            evidence_ref(
                t,
                EvidenceKindClass::SignedReleasePacket,
                "The last-synced signed packet; current validity is unconfirmable offline.",
            ),
            evidence_ref(
                t,
                EvidenceKindClass::AdminAuditExport,
                "The last-synced admin audit export, shown read-only and labeled.",
            ),
        ],
        _ => vec![evidence_ref(
            t,
            EvidenceKindClass::SignedReleasePacket,
            "Local signed packet.",
        )],
    }
}

fn build_packet_sources() -> Vec<CanonicalSourceRef> {
    vec![
        canonical_source(
            CanonicalObjectClass::EffectivePolicy,
            "Deployment mode and posture reuse the effective-policy object, not a local restatement.",
        ),
        canonical_source(
            CanonicalObjectClass::EntitlementSeat,
            "Billing/owner scope reuses the entitlement/seat object.",
        ),
        canonical_source(
            CanonicalObjectClass::EndpointPosture,
            "Verification posture reuses the endpoint-posture object.",
        ),
        canonical_source(
            CanonicalObjectClass::RetentionDeletion,
            "Supported export paths reuse the retention/deletion object's delete-export state.",
        ),
    ]
}

fn build_event_cards(profile: AdminPathClass) -> Vec<CommercialEventCard> {
    let p = params(profile);
    let profile_token = profile.as_str();
    p.events
        .iter()
        .map(|event| event_card(*event, &p, profile_token))
        .collect()
}

/// The commercial call-to-action that trails the recovery actions for an event.
fn commercial_cta(event: CommercialEventClass) -> NextActionClass {
    match event {
        CommercialEventClass::Renewal | CommercialEventClass::TrialStart => {
            NextActionClass::ReviewRenewal
        }
        CommercialEventClass::TrialExpiry => NextActionClass::ReviewRenewal,
        CommercialEventClass::SeatIncrease
        | CommercialEventClass::SeatDecrease
        | CommercialEventClass::Cancellation => NextActionClass::ContactBilling,
        CommercialEventClass::PlanDowngrade => NextActionClass::UpgradePlan,
    }
}

fn event_next_actions(event: CommercialEventClass) -> Vec<EventNextAction> {
    let action = |action: NextActionClass, order: u32, note: &str| EventNextAction {
        action,
        order,
        label: action.label().to_owned(),
        recovery_action: action.is_recovery(),
        note: note.to_owned(),
    };
    vec![
        action(
            NextActionClass::ExportUserData,
            1,
            "Export user-owned data — always first and free of a paid seat.",
        ),
        action(
            NextActionClass::DeleteUserData,
            2,
            "Delete user-owned data — ahead of any commercial call-to-action.",
        ),
        action(
            NextActionClass::OpenSupport,
            3,
            "Open support for export or continuity questions.",
        ),
        action(
            NextActionClass::ContinueLocalOnly,
            4,
            "Continue local-only, with no paid seat.",
        ),
        action(
            commercial_cta(event),
            5,
            "The commercial call-to-action ranks below every recovery action.",
        ),
    ]
}

fn event_card(
    event: CommercialEventClass,
    p: &ProfileParams,
    profile_token: &str,
) -> CommercialEventCard {
    let loss = event.is_entitlement_loss();
    let (effective_date, impacted_features, local_only_path) = match event {
        CommercialEventClass::Renewal => (
            "2026-12-31T23:59:59Z",
            "Managed policy bundles and seat entitlements continue through the renewal term.",
            "Local-only workspace continues unchanged regardless of the renewal outcome.",
        ),
        CommercialEventClass::TrialStart => (
            "2026-06-22T00:00:00Z",
            "Trial unlocks managed policy preview and additional seats for the trial term.",
            "Local-only features stay available with no trial dependency.",
        ),
        CommercialEventClass::TrialExpiry => (
            "2026-07-22T00:00:00Z",
            "Trial-only managed features lock at expiry; local features are unaffected.",
            "Local-only workspace and export stay available after the trial expires.",
        ),
        CommercialEventClass::SeatIncrease => (
            "2026-06-22T00:00:00Z",
            "Added seats extend managed entitlements to more members.",
            "Local-only continuation is unchanged.",
        ),
        CommercialEventClass::SeatDecrease => (
            "2026-08-01T00:00:00Z",
            "Removed seats end managed entitlements for affected members at the effective date.",
            "Affected members keep local-only continuation and export with no paid seat.",
        ),
        CommercialEventClass::PlanDowngrade => (
            "2026-08-01T00:00:00Z",
            "Downgraded plan reduces managed entitlements at the effective date.",
            "Local-only workspace, edit, and export stay available after the downgrade.",
        ),
        CommercialEventClass::Cancellation => (
            "2026-09-01T00:00:00Z",
            "Cancellation ends managed entitlements at the effective date.",
            "Local-only workspace and export stay available after cancellation.",
        ),
    };

    CommercialEventCard {
        card_id: format!("procurement.event.{profile_token}.{}", event.as_str()),
        event,
        label: event.label().to_owned(),
        effective_date: effective_date.to_owned(),
        as_of: M5_PROCUREMENT_AS_OF.to_owned(),
        impacted_features: impacted_features.to_owned(),
        impacted_scope: p.billing,
        machine_state: p.state,
        evidence_age: p.evidence_age,
        entitlement_loss: loss,
        local_only_path: local_only_path.to_owned(),
        next_actions: event_next_actions(event),
        outranks_recovery_actions: false,
        export_next_step: "Export user-owned data locally now; it never needs a paid seat."
            .to_owned(),
        support_next_step: "Open support for export, deletion, or continuity questions.".to_owned(),
        canonical_sources: vec![
            canonical_source(
                CanonicalObjectClass::EntitlementSeat,
                "Event type and impacted scope reuse the entitlement/seat object.",
            ),
            canonical_source(
                CanonicalObjectClass::OffboardingContinuity,
                "The local-only path reuses the offboarding/continuity object.",
            ),
        ],
        requires_paid_seat_for_recovery: false,
        machine_summary: format!(
            "event={} scope={} effective={} state={} entitlement_loss={} outranks_recovery=false",
            event.as_str(),
            p.billing.as_str(),
            effective_date,
            p.state.as_str(),
            loss,
        ),
        plain_language: format!(
            "{} on {profile_token}: {} Export, delete, support, and local-continuation come first; \
             the commercial step ranks below them.",
            event.label(),
            impacted_features,
        ),
    }
}

fn build_admin_handoff(profile: AdminPathClass) -> AdminHandoffPacket {
    let p = params(profile);
    let profile_token = profile.as_str();
    let build_ref = format!("build.{profile_token}.2026.06");
    let bundle_ids = vec![
        format!("bundle.policy.{profile_token}"),
        format!("bundle.entitlement.{profile_token}"),
    ];
    let affected_features = match profile {
        AdminPathClass::ManagedCloud => vec![
            "Managed policy bundles".to_owned(),
            "Seat entitlements".to_owned(),
            "Admin audit export".to_owned(),
        ],
        AdminPathClass::SelfHosted => vec![
            "Self-hosted policy bundles".to_owned(),
            "License entitlements".to_owned(),
        ],
        AdminPathClass::SovereignAirGapped => vec![
            "Offline policy bundle".to_owned(),
            "Sovereign entitlement".to_owned(),
        ],
        AdminPathClass::MirroredOffline => vec![
            "Mirrored policy bundle".to_owned(),
            "Mirrored entitlement".to_owned(),
        ],
        _ => vec!["Local features".to_owned()],
    };

    AdminHandoffPacket {
        handoff_id: format!("procurement.handoff.{profile_token}"),
        build_ref: build_ref.clone(),
        channel: p.channel,
        install_mode: p.install,
        workspace_archetype: p.archetype,
        bundle_ids,
        affected_features,
        machine_state: p.state,
        as_of: M5_PROCUREMENT_AS_OF.to_owned(),
        handoff_owner: OwnerEscalationRoleClass::OrgAdmin,
        canonical_sources: vec![
            canonical_source(
                CanonicalObjectClass::EffectivePolicy,
                "Build/channel and affected features reuse the effective-policy object.",
            ),
            canonical_source(
                CanonicalObjectClass::DecisionHistory,
                "The handoff summary reuses the decision-history object for audit lineage.",
            ),
        ],
        export_safe_summary: format!(
            "Admin handoff for {profile_token}: build {}, {} channel, {} install, {} archetype; \
             two bundle ids and the affected features, auto-derived from current managed state.",
            build_ref,
            p.channel.as_str(),
            p.install.as_str(),
            p.archetype.as_str(),
        ),
        auto_derived: true,
        governing_schema_ref: "schemas/admin/admin_audit_export.schema.json".to_owned(),
        schema_note: "The handoff is auto-derived and governed by the admin-audit-export schema; \
                      it carries opaque bundle ids and metadata-safe summaries only."
            .to_owned(),
        machine_summary: format!(
            "handoff={} build_channel={} install={} archetype={} state={}",
            profile_token,
            p.channel.as_str(),
            p.install.as_str(),
            p.archetype.as_str(),
            p.state.as_str(),
        ),
        plain_language: format!(
            "Admin-handoff packet for {profile_token}: build/channel, install mode, workspace \
             archetype, bundle ids, and affected features — auto-derived, export-safe, and ready \
             for a support or admin handoff.",
        ),
    }
}

fn build_export_forms(profile: AdminPathClass) -> Vec<ExportForm> {
    let profile_token = profile.as_str();
    vec![
        ExportForm {
            format: ExportFormatClass::MachineReadableJson,
            label: "Machine-readable summary".to_owned(),
            artifact_ref: format!("procurement.export.{profile_token}.machine"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "The verification packet, event cards, and admin-handoff packet as JSON \
                          summary objects, copyable or exportable for procurement tooling."
                .to_owned(),
        },
        ExportForm {
            format: ExportFormatClass::PlainLanguageHandoff,
            label: "Plain-language handoff packet".to_owned(),
            artifact_ref: format!("procurement.export.{profile_token}.handoff"),
            redaction: AdminRedactionClass::MetadataSafeDefault,
            description: "The same content as reviewable plain-language sentences for a \
                          procurement, renewal, support, or admin handoff, with no raw payloads."
                .to_owned(),
        },
    ]
}

fn build_coverage(profile: AdminPathClass) -> ProcurementCoverage {
    let p = params(profile);
    let (window_label, coverage_note) = match profile {
        AdminPathClass::ManagedCloud => (
            "Verified posture — live",
            "The signed packet is verified live; the renewal/seat cards and admin handoff are \
             current and exportable.",
        ),
        AdminPathClass::SelfHosted => (
            "Verified posture — self-hosted",
            "The packet is verified against the customer trust root; the cards and handoff are \
             current and exportable.",
        ),
        AdminPathClass::SovereignAirGapped => (
            "Offline — past validity, labeled",
            "The offline bundle is past validity and labeled unverified rather than shown verified; \
             export and recovery stay available offline.",
        ),
        AdminPathClass::MirroredOffline => (
            "Offline — last-synced, labeled",
            "The mirror packet is last-synced and labeled unconfirmed rather than shown verified; \
             local export and continuation stay available.",
        ),
        _ => ("Local", "Local procurement surface."),
    };

    ProcurementCoverage {
        coverage_state: p.state,
        completeness: p.completeness,
        window_label: window_label.to_owned(),
        coverage_note: coverage_note.to_owned(),
        locally_inspectable: true,
        vendor_console_independent: true,
        exportable_without_paid_seat: true,
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> ProcurementInvariant {
    ProcurementInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(profiles: &[ProcurementProfilePacket]) -> Vec<ProcurementInvariant> {
    let matrix = admin_plane_matrix();
    let admitted = |state: AdminStateClass| -> bool {
        matrix
            .surface(AdminSurfaceClass::ProcurementVerificationPacket)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };
    let declared_consumers = parity_consumers();
    let all_packets = || profiles.iter().map(|p| &p.verification_packet);
    let all_cards = || profiles.iter().flat_map(|p| p.event_cards.iter());
    let all_handoffs = || profiles.iter().map(|p| &p.admin_handoff);

    let mut out = Vec::new();

    // Every rendered state is one the matrix admits for this surface.
    out.push(invariant(
        "procurement.surface_states_within_matrix",
        "Every state a verification packet, event card, admin-handoff packet, or the coverage \
         posture shows is one the frozen admin-plane matrix declares applicable for the \
         procurement surface, so the render layer cannot drift from the contract.",
        all_packets().all(|vp| admitted(vp.machine_state))
            && all_cards().all(|c| admitted(c.machine_state))
            && all_handoffs().all(|h| admitted(h.machine_state))
            && profiles.iter().all(|p| admitted(p.coverage.coverage_state)),
    ));

    // Every claimed managed-bearing profile is rendered.
    out.push(invariant(
        "procurement.profiles_covered",
        "The bundle renders the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles, each with a verification packet, at least one renewal/trial/\
         seat-change card, and an admin-handoff packet.",
        PROCUREMENT_PROFILES.iter().all(|profile| {
            profiles
                .iter()
                .any(|p| p.profile == *profile && !p.event_cards.is_empty())
        }),
    ));

    // Consumer parity: one typed packet serves every declared consumer.
    out.push(invariant(
        "procurement.consumer_parity",
        "Each profile is one typed packet consumed identically by every consumer the matrix \
         declares for the procurement surface — commercial/procurement, Help/About, support \
         export, release evidence, and managed service — so commercial, support, and admin \
         surfaces reuse it rather than restating it.",
        !declared_consumers.is_empty()
            && profiles
                .iter()
                .all(|p| declared_consumers.iter().all(|c| p.consumers.contains(c))),
    ));

    // No-silent-green: stale evidence never sits under a confirmed active state.
    out.push(invariant(
        "procurement.verification_no_silent_green",
        "A verification packet, event card, or admin-handoff packet whose backing evidence is \
         stale is never shown under a confirmed active/enforced state, and a packet that is not \
         currently verified is never shown active/enforced.",
        all_packets().all(|vp| {
            !(vp.evidence_age.is_stale() && requires_fresh_evidence(vp.machine_state))
                && (vp.is_verified_now() || vp.machine_state != AdminStateClass::ActiveEnforced)
        }) && all_cards()
            .all(|c| !(c.evidence_age.is_stale() && requires_fresh_evidence(c.machine_state)))
            && profiles.iter().all(|p| {
                let age = p.verification_packet.evidence_age;
                !(age.is_stale() && requires_fresh_evidence(p.admin_handoff.machine_state))
            }),
    ));

    // Validity is labeled, never silently green.
    out.push(invariant(
        "procurement.validity_labeled",
        "Every verification packet states a validity window with a non-empty label; a packet past \
         its validity window or not currently verified uses a non-active state rather than being \
         presented as currently verified.",
        all_packets().all(|vp| {
            !vp.validity_window.window_label.is_empty()
                && (vp.validity_window.within_window
                    || vp.machine_state != AdminStateClass::ActiveEnforced)
                && (vp.is_verified_now() || vp.machine_state != AdminStateClass::ActiveEnforced)
        }),
    ));

    // Export paths are present, local-safe, and seat-free.
    out.push(invariant(
        "procurement.export_paths_present",
        "Every verification packet names at least one supported export path, at least one of which \
         works offline, and no export path requires a still-active paid seat.",
        all_packets().all(|vp| {
            !vp.supported_export_paths.is_empty()
                && vp.supported_export_paths.iter().any(|e| e.available_offline)
                && vp.supported_export_paths.iter().all(|e| !e.requires_paid_seat)
        }),
    ));

    // Owner scope and as-of date are named on every object.
    out.push(invariant(
        "procurement.owner_scope_and_asof",
        "Every verification packet, event card, and admin-handoff packet names an owner/billing \
         scope and a non-empty as-of date, so exported posture is always attributable and \
         time-stamped.",
        all_packets().all(|vp| !vp.as_of.is_empty())
            && all_cards().all(|c| !c.as_of.is_empty())
            && all_handoffs().all(|h| !h.as_of.is_empty()),
    ));

    // Evidence refs are present and export-safe.
    out.push(invariant(
        "procurement.evidence_refs_present",
        "Every verification packet carries at least one evidence ref, and every evidence ref's \
         schema ref is an export-safe repo ref, so current posture is always backed by named \
         proof.",
        all_packets().all(|vp| {
            !vp.evidence_refs.is_empty()
                && vp
                    .evidence_refs
                    .iter()
                    .all(|e| is_export_safe_ref(e.schema_ref.as_str()))
        }),
    ));

    // Events disclose impact, and every event class appears across the bundle.
    out.push(invariant(
        "procurement.events_disclose_impact",
        "Every renewal/trial/seat-change card discloses its event type, effective date, impacted \
         managed features, as-of date, local-only path, and export/support next step, and every \
         event class appears at least once across the bundle.",
        all_cards().all(|c| {
            !c.effective_date.is_empty()
                && !c.impacted_features.is_empty()
                && !c.local_only_path.is_empty()
                && !c.export_next_step.is_empty()
                && !c.support_next_step.is_empty()
        }) && CommercialEventClass::ALL.iter().all(|event| {
            profiles
                .iter()
                .any(|p| p.event_cards.iter().any(|c| c.event == *event))
        }),
    ));

    // AC2: a commercial card never outranks the recovery actions.
    out.push(invariant(
        "procurement.events_never_outrank_recovery",
        "Every renewal/trial/seat-change card keeps the export, delete, support, and \
         local-continuation actions ahead of any commercial call-to-action — each is flagged \
         outranks_recovery_actions=false, carries all four recovery actions, and orders every \
         recovery action before every commercial action — and recovery never requires a paid \
         seat; at least one entitlement-loss card appears.",
        all_cards().all(|c| {
            !c.outranks_recovery_actions
                && !c.requires_paid_seat_for_recovery
                && c.recovery_outranks_commercial()
                && [
                    NextActionClass::ExportUserData,
                    NextActionClass::DeleteUserData,
                    NextActionClass::OpenSupport,
                    NextActionClass::ContinueLocalOnly,
                ]
                .iter()
                .all(|action| c.next_actions.iter().any(|a| a.action == *action))
        }) && all_cards().any(|c| c.entitlement_loss),
    ));

    // Admin-handoff packets are complete and auto-derived.
    out.push(invariant(
        "procurement.handoff_complete",
        "Every admin-handoff packet names a build ref, release channel, install mode, workspace \
         archetype, at least one bundle id, the affected features, and an export-safe summary, and \
         is auto-derived from current state rather than manually curated.",
        all_handoffs().all(|h| {
            !h.build_ref.is_empty()
                && !h.bundle_ids.is_empty()
                && !h.affected_features.is_empty()
                && !h.export_safe_summary.is_empty()
                && h.auto_derived
        }),
    ));

    // AC3: surfaces reuse canonical managed-state objects rather than restating them.
    out.push(invariant(
        "procurement.reuses_canonical_objects",
        "Every verification packet, event card, and admin-handoff packet reuses at least one \
         canonical managed-state object by an export-safe schema ref rather than restating it with \
         local copy, and across the bundle every canonical object family appears.",
        all_packets().all(|vp| sources_ok(&vp.canonical_sources))
            && all_cards().all(|c| sources_ok(&c.canonical_sources))
            && all_handoffs().all(|h| sources_ok(&h.canonical_sources))
            && CanonicalObjectClass::ALL.iter().all(|object| {
                profiles.iter().any(|p| {
                    p.verification_packet
                        .canonical_sources
                        .iter()
                        .any(|c| c.object == *object)
                        || p.event_cards
                            .iter()
                            .any(|card| card.canonical_sources.iter().any(|c| c.object == *object))
                        || p.admin_handoff
                            .canonical_sources
                            .iter()
                            .any(|c| c.object == *object)
                })
            }),
    ));

    // No paid seat for recovery anywhere.
    out.push(invariant(
        "procurement.no_paid_seat_for_recovery",
        "No verification packet, event card, or coverage view requires a still-active paid seat to \
         recover user-owned data: export stays reachable and every coverage view is exportable \
         without a paid seat.",
        all_packets().all(|vp| !vp.requires_paid_seat_for_export)
            && all_cards().all(|c| !c.requires_paid_seat_for_recovery)
            && profiles
                .iter()
                .all(|p| p.coverage.exportable_without_paid_seat),
    ));

    // Locally inspectable without a vendor console on every profile.
    out.push(invariant(
        "procurement.locally_inspectable_offline",
        "Every profile — including self-hosted, sovereign/air-gapped, and mirrored/offline — keeps \
         a locally inspectable procurement surface that does not require a vendor console or \
         control plane and stays exportable without a paid seat.",
        profiles.iter().all(|p| {
            let c = &p.coverage;
            c.locally_inspectable && c.vendor_console_independent && c.exportable_without_paid_seat
        }),
    ));

    // Export parity: machine summary and plain-language on every object.
    out.push(invariant(
        "procurement.export_parity",
        "Every verification packet, event card, and admin-handoff packet carries both an \
         export-safe machine-readable summary and a plain-language handoff sentence, and every \
         profile offers both export forms.",
        all_packets().all(ProcurementPacket::has_export_parity)
            && all_cards().all(CommercialEventCard::has_export_parity)
            && all_handoffs().all(AdminHandoffPacket::has_export_parity)
            && profiles.iter().all(|p| {
                p.offers(ExportFormatClass::MachineReadableJson)
                    && p.offers(ExportFormatClass::PlainLanguageHandoff)
            }),
    ));

    // Partial coverage is labeled, never implied complete.
    out.push(invariant(
        "procurement.coverage_labeled",
        "A coverage view that is offline or past-validity is labeled with a non-complete \
         completeness class and a coverage note and a non-active coverage state, so a partial \
         surface is never presented as a confirmed-complete verified one.",
        profiles.iter().all(|p| {
            let c = &p.coverage;
            !c.coverage_note.is_empty()
                && (!c.completeness.is_partial()
                    || c.coverage_state != AdminStateClass::ActiveEnforced)
        }),
    ));

    // Residual dependencies are honestly disclosed with a local-safe fallback.
    out.push(invariant(
        "procurement.residual_dependencies_honest",
        "Every residual dependency a verification packet discloses names a local-safe fallback, so \
         a remaining managed dependency never implies user-owned recovery depends on it.",
        all_packets().all(|vp| {
            vp.residual_dependencies
                .iter()
                .all(|d| !d.local_safe_fallback.is_empty())
        }),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "procurement.export_safe",
        "Every stable surface, profile, packet, card, evidence, handoff, build, and bundle id is \
         an opaque token with no URL scheme or absolute path, and every governing and source \
         schema ref is a repo-relative ref, so the bundle is safe to embed in a support, \
         procurement, or renewal export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && is_safe_token(p.surface_id.as_str())
                && is_safe_token(p.verification_packet.packet_id.as_str())
                && is_export_safe_ref(p.verification_packet.governing_schema_ref.as_str())
                && p.verification_packet.evidence_refs.iter().all(|e| {
                    is_safe_token(e.evidence_id.as_str())
                        && is_export_safe_ref(e.schema_ref.as_str())
                })
                && p.event_cards
                    .iter()
                    .all(|c| is_safe_token(c.card_id.as_str()))
                && is_safe_token(p.admin_handoff.handoff_id.as_str())
                && is_safe_token(p.admin_handoff.build_ref.as_str())
                && p.admin_handoff
                    .bundle_ids
                    .iter()
                    .all(|b| is_safe_token(b.as_str()))
                && is_export_safe_ref(p.admin_handoff.governing_schema_ref.as_str())
                && p.export_forms
                    .iter()
                    .all(|x| is_safe_token(x.artifact_ref.as_str()))
        }),
    ));

    out
}

/// Whether a canonical-source list is present and all its schema refs are
/// export-safe.
fn sources_ok(sources: &[CanonicalSourceRef]) -> bool {
    !sources.is_empty()
        && sources
            .iter()
            .all(|c| is_export_safe_ref(c.schema_ref.as_str()))
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn procurement_lines(bundle: &ProcurementBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Procurement bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        let vp = &p.verification_packet;
        lines.push(format!(
            "  Verification: state={} posture={} within_window={} billing={} owner={} age={}",
            vp.machine_state.as_str(),
            vp.verification_posture.as_str(),
            vp.validity_window.within_window,
            vp.billing_scope.as_str(),
            vp.packet_owner.as_str(),
            vp.evidence_age.as_str(),
        ));
        lines.push("  Export paths:".to_owned());
        for e in &vp.supported_export_paths {
            lines.push(format!(
                "    - {} (offline={} seat_free={})",
                e.label, e.available_offline, !e.requires_paid_seat,
            ));
        }
        lines.push("  Evidence refs:".to_owned());
        for e in &vp.evidence_refs {
            lines.push(format!("    - {} [{}]", e.label, e.kind.as_str()));
        }
        lines.push("  Renewal/trial/seat-change cards:".to_owned());
        for c in &p.event_cards {
            lines.push(format!(
                "    - {} [{}] effective={} loss={} outranks_recovery={}",
                c.label,
                c.event.as_str(),
                c.effective_date,
                c.entitlement_loss,
                c.outranks_recovery_actions,
            ));
            let actions: Vec<String> = c
                .next_actions
                .iter()
                .map(|a| format!("{}.{}", a.order, a.action.as_str()))
                .collect();
            lines.push(format!("        actions: {}", actions.join(" → ")));
        }
        let h = &p.admin_handoff;
        lines.push(format!(
            "  Admin handoff: build={} channel={} install={} archetype={} bundles={} auto_derived={}",
            h.build_ref,
            h.channel.as_str(),
            h.install_mode.as_str(),
            h.workspace_archetype.as_str(),
            h.bundle_ids.len(),
            h.auto_derived,
        ));
        let coverage = &p.coverage;
        lines.push(format!(
            "  Coverage: state={} completeness={} window={} local={} console_independent={} \
             seat_free={}",
            coverage.coverage_state.as_str(),
            coverage.completeness.as_str(),
            coverage.window_label,
            coverage.locally_inspectable,
            coverage.vendor_console_independent,
            coverage.exportable_without_paid_seat,
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
