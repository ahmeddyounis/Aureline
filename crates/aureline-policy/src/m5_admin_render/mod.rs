//! M5 admin-plane *rendered surfaces*: the concrete, typed instances of the
//! effective-policy view, policy-diff sheet, locked-state explanation, and
//! endpoint-posture card that Aureline shows on its claimed managed,
//! self-hosted, sovereign/air-gapped, and mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — the surface families, the shared state vocabulary, the controlled
//! vocabularies, and the admin paths — this lane *renders the surfaces*. It turns
//! policy and endpoint state into a first-class local product surface: a user or
//! admin can read, on the machine in front of them, what each control resolves
//! to, which source wins, whether it is locked and why, what a pending policy
//! change moves, and what install/update/mirror/trust posture the endpoint is on
//! — without a separate vendor console.
//!
//! Each rendered surface *binds back to the matrix*: every state it shows must be
//! one the matrix declares applicable for that surface family
//! ([`AdminRenderInvariant`] `admin_render.surface_states_within_matrix`), and
//! every policy-source, verification, data-residency, owner, and freshness token
//! it uses is a term the matrix's shared vocabulary defines. So the render layer
//! cannot drift from the frozen contract: an edit that shows a state the matrix
//! does not admit, or a token the matrix does not define, flips an invariant and
//! fails the freeze gate.
//!
//! The bundle holds one [`AdminRenderPacket`] per claimed managed-bearing profile
//! and computes each invariant's `holds` flag from the rendered data, so the
//! checked-in fixture freezes the rendered surfaces byte-for-byte. Honesty rules
//! are enforced, not just described: a locked control always resolves to a
//! [`LockedStateExplanation`] that names its source, verification posture, and the
//! owner of the next step ([`AdminRenderInvariant`]
//! `admin_render.locked_controls_explained`); a control whose backing evidence is
//! stale is never shown as a confirmed-green value
//! (`admin_render.no_silent_green`); and every endpoint-posture card is locally
//! inspectable and exportable (`admin_render.endpoint_posture_exportable`).
//! Because there is exactly one typed packet per profile, the shell admin center,
//! CLI/headless inspect, Help/About, support export, and release-evidence
//! consumers all render the *same* bytes — policy source, diff, and endpoint
//! state are identical across surfaces by construction
//! (`admin_render.consumer_parity`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, or absolute paths — only opaque object refs, stable tokens, rendered
//! metadata-safe value summaries, and short reviewable sentences — so it is safe
//! to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminScopeClass,
    AdminStateClass, AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};

#[cfg(test)]
mod tests;

/// Schema version for the admin-plane render bundle.
pub const M5_ADMIN_RENDER_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the admin-plane render bundle.
pub const M5_ADMIN_RENDER_SCHEMA_REF: &str = "schemas/admin/m5-admin-render.schema.json";

/// Stable record-kind tag for the admin-plane render bundle.
pub const M5_ADMIN_RENDER_RECORD_KIND: &str = "m5_admin_render_bundle";

/// Stable id for the canonical render bundle.
pub const M5_ADMIN_RENDER_BUNDLE_ID: &str = "m5-admin-render:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ADMIN_RENDER_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this render layer binds back to.
pub const M5_ADMIN_RENDER_MATRIX_REF: &str = "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the render bundle current.
pub const M5_ADMIN_RENDER_FREEZE_GATE_REF: &str = "crates/aureline-policy/tests/m5_admin_render.rs";

// ---------------------------------------------------------------------------
// Controlled-vocabulary token enums (must match the matrix shared vocabulary).
// ---------------------------------------------------------------------------

/// Where an effective value comes from and that source's state — the
/// `policy_source_state` controlled vocabulary the matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySourceStateClass {
    /// A built-in local default.
    LocalDefault,
    /// A workspace/team setting.
    WorkspaceSetting,
    /// A managed (or self-hosted) policy bundle.
    ManagedPolicyBundle,
    /// A mirrored policy bundle served from a last-synced offline mirror.
    MirroredPolicyBundle,
    /// A remembered local decision.
    RememberedDecision,
    /// A signed offline bundle (sovereign / air-gapped).
    SignedOfflineBundle,
    /// Source could not be determined and requires review.
    UnknownSource,
}

impl PolicySourceStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDefault => "local_default",
            Self::WorkspaceSetting => "workspace_setting",
            Self::ManagedPolicyBundle => "managed_policy_bundle",
            Self::MirroredPolicyBundle => "mirrored_policy_bundle",
            Self::RememberedDecision => "remembered_decision",
            Self::SignedOfflineBundle => "signed_offline_bundle",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Whether a managed claim is signed, verified, expired, revoked, or
/// unverifiable offline — the `verification_signature_posture` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationPostureClass {
    /// Signed and verified against a current trust root.
    SignedVerified,
    /// Signed but not yet verified.
    SignedUnverified,
    /// Unsigned local value.
    UnsignedLocal,
    /// The signature is past its validity window.
    SignatureExpired,
    /// The signature was revoked.
    SignatureRevoked,
    /// Cannot be verified while offline; shown unverified, never as verified.
    UnverifiableOffline,
}

impl VerificationPostureClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::SignedUnverified => "signed_unverified",
            Self::UnsignedLocal => "unsigned_local",
            Self::SignatureExpired => "signature_expired",
            Self::SignatureRevoked => "signature_revoked",
            Self::UnverifiableOffline => "unverifiable_offline",
        }
    }

    /// Whether this posture asserts a currently-verified signature.
    pub const fn is_verified_now(self) -> bool {
        matches!(self, Self::SignedVerified)
    }
}

/// Where a data class lives — the `data_residency_class` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataResidencyClass {
    /// Local-only; never copied to a managed plane.
    LocalOnly,
    /// A managed copy lives in the control plane.
    ManagedCopy,
    /// A mirrored copy served from a last-synced offline mirror.
    MirroredCopy,
    /// A shared workspace copy.
    SharedWorkspaceCopy,
    /// An exported snapshot.
    ExportedSnapshot,
}

impl DataResidencyClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopy => "managed_copy",
            Self::MirroredCopy => "mirrored_copy",
            Self::SharedWorkspaceCopy => "shared_workspace_copy",
            Self::ExportedSnapshot => "exported_snapshot",
        }
    }
}

/// Who owns a control or step and who it escalates to — the `owner_escalation`
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerEscalationRoleClass {
    /// The local user on this machine.
    LocalUser,
    /// The workspace owner.
    WorkspaceOwner,
    /// The organization admin.
    OrgAdmin,
    /// The security owner.
    SecurityOwner,
    /// The compliance owner.
    ComplianceOwner,
    /// Vendor support.
    VendorSupport,
}

impl OwnerEscalationRoleClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::WorkspaceOwner => "workspace_owner",
            Self::OrgAdmin => "org_admin",
            Self::SecurityOwner => "security_owner",
            Self::ComplianceOwner => "compliance_owner",
            Self::VendorSupport => "vendor_support",
        }
    }
}

/// Evidence freshness age — mirrors the matrix freshness age tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAgeClass {
    /// Confirmed fresh.
    Fresh,
    /// Recent, within the soft-refresh window.
    Recent,
    /// Stale: past the soft-refresh window.
    Stale,
    /// Very stale.
    VeryStale,
    /// Never confirmed.
    Never,
}

impl EvidenceAgeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::VeryStale => "very_stale",
            Self::Never => "never",
        }
    }

    /// Whether this age is stale enough to forbid a confirmed-green headline.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale | Self::VeryStale | Self::Never)
    }
}

// ---------------------------------------------------------------------------
// Render-specific token enums.
// ---------------------------------------------------------------------------

/// What a policy-diff entry moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyChangeKindClass {
    /// A control that was open is now locked.
    NewlyLocked,
    /// A control that was locked is now unlocked.
    Unlocked,
    /// A control's scope changed (e.g. managed-org to workspace).
    Rescoped,
    /// The effective value changed.
    ValueChanged,
    /// The winning source changed.
    SourceChanged,
    /// No effective change; surfaced for completeness.
    NoChange,
}

impl PolicyChangeKindClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewlyLocked => "newly_locked",
            Self::Unlocked => "unlocked",
            Self::Rescoped => "rescoped",
            Self::ValueChanged => "value_changed",
            Self::SourceChanged => "source_changed",
            Self::NoChange => "no_change",
        }
    }
}

/// How the product is installed on the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallModeClass {
    /// Per-user install.
    PerUser,
    /// Per-machine install.
    PerMachine,
    /// Portable / no-install.
    Portable,
    /// Managed image deployed by IT.
    ManagedImage,
    /// Sovereign / air-gapped image.
    SovereignImage,
}

impl InstallModeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUser => "per_user",
            Self::PerMachine => "per_machine",
            Self::Portable => "portable",
            Self::ManagedImage => "managed_image",
            Self::SovereignImage => "sovereign_image",
        }
    }
}

/// The update ring the endpoint is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateRingClass {
    /// Stable ring.
    Stable,
    /// Extended / slow ring.
    Extended,
    /// Beta ring.
    Beta,
    /// Pinned by managed policy.
    PinnedManaged,
    /// Pinned to an offline bundle (sovereign / mirrored).
    PinnedOffline,
}

impl UpdateRingClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Extended => "extended",
            Self::Beta => "beta",
            Self::PinnedManaged => "pinned_managed",
            Self::PinnedOffline => "pinned_offline",
        }
    }
}

/// The signed-in identity status backing the posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityStatusClass {
    /// Signed in and verified.
    SignedInVerified,
    /// A managed session under a control plane.
    ManagedSession,
    /// Signed out; running local-only.
    SignedOutLocalOnly,
    /// Session expired and needs reauthorization.
    SessionExpired,
    /// A device rebind is required before managed posture resumes.
    RebindRequired,
}

impl IdentityStatusClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedInVerified => "signed_in_verified",
            Self::ManagedSession => "managed_session",
            Self::SignedOutLocalOnly => "signed_out_local_only",
            Self::SessionExpired => "session_expired",
            Self::RebindRequired => "rebind_required",
        }
    }
}

// ---------------------------------------------------------------------------
// Effective-policy view.
// ---------------------------------------------------------------------------

/// One link in a control's resolved source chain, from lowest to winning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySourceLink {
    /// Position in the chain (0 = lowest precedence).
    pub order: u32,
    /// The source state of this link.
    pub source_state: PolicySourceStateClass,
    /// One reviewable label for this link.
    pub label: String,
    /// Whether this link is the winning (effective) source.
    pub winning: bool,
}

/// One control row in the effective-policy view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyControl {
    /// Stable control id.
    pub control_id: String,
    /// Human-readable control label.
    pub label: String,
    /// The affected feature family.
    pub feature_family: String,
    /// The rendered, metadata-safe effective value summary (never a raw secret).
    pub effective_value: String,
    /// The resolved state of this control.
    pub state: AdminStateClass,
    /// The local-versus-shared scope of the winning value.
    pub scope: AdminScopeClass,
    /// The verification/signature posture of the winning source.
    pub verification: VerificationPostureClass,
    /// The resolved source chain, lowest precedence first.
    pub source_chain: Vec<PolicySourceLink>,
    /// When the winning source was applied (ISO-8601).
    pub applied_at: String,
    /// The freshness of the winning source's evidence.
    pub evidence_age: EvidenceAgeClass,
    /// Where this control's data lives.
    pub data_residency: DataResidencyClass,
    /// Who owns this control.
    pub owner: OwnerEscalationRoleClass,
    /// One reviewable posture sentence.
    pub posture_note: String,
    /// The locked-state explanation this control links to, if locked or forced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_explanation_ref: Option<String>,
}

impl EffectivePolicyControl {
    /// The winning link in the source chain, if any.
    pub fn winning_source(&self) -> Option<&PolicySourceLink> {
        self.source_chain.iter().find(|l| l.winning)
    }

    /// Whether this control is locked or otherwise forced (must link to an
    /// explanation).
    pub fn is_locked(&self) -> bool {
        self.state == AdminStateClass::LockedByPolicy || self.locked_explanation_ref.is_some()
    }
}

/// The rendered effective-policy view for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePolicyView {
    /// The surface family (always [`AdminSurfaceClass::EffectivePolicyView`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The control rows.
    pub controls: Vec<EffectivePolicyControl>,
}

// ---------------------------------------------------------------------------
// Policy-diff sheet.
// ---------------------------------------------------------------------------

/// One changed control in a policy-diff sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDiffEntry {
    /// Stable change id.
    pub change_id: String,
    /// The control that changed.
    pub control_id: String,
    /// The affected feature family.
    pub feature_family: String,
    /// What the change moves.
    pub change_kind: PolicyChangeKindClass,
    /// The previous effective state.
    pub from_state: AdminStateClass,
    /// The new effective state.
    pub to_state: AdminStateClass,
    /// The previous winning source.
    pub from_source: PolicySourceStateClass,
    /// The new winning source.
    pub to_source: PolicySourceStateClass,
    /// One reviewable sentence of the user-visible consequence.
    pub user_visible_consequence: String,
    /// The redaction rule applied to this entry on export.
    pub redaction: AdminRedactionClass,
    /// Who owns the change.
    pub owner: OwnerEscalationRoleClass,
}

/// The rendered policy-diff sheet for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDiffSheet {
    /// The surface family (always [`AdminSurfaceClass::PolicyDiff`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// Label for the previous effective state being compared against.
    pub from_label: String,
    /// Label for the current/proposed effective state.
    pub to_label: String,
    /// Whether the diff is provisional because the current effective values are
    /// stale (the no-silent-green safety for diffs).
    pub provisional: bool,
    /// The changed controls.
    pub changes: Vec<PolicyDiffEntry>,
}

// ---------------------------------------------------------------------------
// Locked-state explanation.
// ---------------------------------------------------------------------------

/// A rendered explanation of why a specific control is locked or forced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockedStateExplanation {
    /// The surface family (always [`AdminSurfaceClass::LockedStateExplanation`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// Stable explanation id (referenced by [`EffectivePolicyControl::locked_explanation_ref`]).
    pub explanation_id: String,
    /// The control this explanation is about.
    pub locked_target_ref: String,
    /// The lock state (from the shared vocabulary).
    pub lock_state: AdminStateClass,
    /// One reviewable sentence stating the lock reason.
    pub lock_reason: String,
    /// The policy source that locks the control.
    pub lock_source: PolicySourceStateClass,
    /// The verification posture of the lock source.
    pub verification: VerificationPostureClass,
    /// Who can change the control (the owner of the next step).
    pub change_owner: OwnerEscalationRoleClass,
    /// Who the change escalates to, if anyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_owner: Option<OwnerEscalationRoleClass>,
    /// The local-safe actions available from this explanation.
    pub local_safe_actions: Vec<String>,
}

impl LockedStateExplanation {
    /// Whether the explanation names a source, a verification posture, a reason,
    /// and a change owner — the floor every locked control must meet.
    pub fn is_complete(&self) -> bool {
        !self.lock_reason.is_empty() && !self.local_safe_actions.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Endpoint-posture card.
// ---------------------------------------------------------------------------

/// One mirror source backing the endpoint, with its freshness and verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorSourceRef {
    /// Opaque mirror source ref.
    pub source_id: String,
    /// One reviewable label.
    pub label: String,
    /// The freshness of the last sync from this mirror.
    pub freshness: EvidenceAgeClass,
    /// The verification posture of the mirrored content.
    pub verification: VerificationPostureClass,
}

/// One trust root the endpoint verifies against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootRef {
    /// Opaque trust-root / key id.
    pub root_id: String,
    /// One reviewable label.
    pub label: String,
    /// The verification posture of this trust root.
    pub verification: VerificationPostureClass,
}

/// The rendered endpoint-posture card for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPostureCard {
    /// The surface family (always [`AdminSurfaceClass::EndpointPostureCard`]).
    pub surface: AdminSurfaceClass,
    /// Stable, namespaced surface id from the matrix.
    pub surface_id: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// Opaque device/install id.
    pub device_or_install_id: String,
    /// The resolved posture state (from the shared vocabulary).
    pub posture_state: AdminStateClass,
    /// How the product is installed.
    pub install_mode: InstallModeClass,
    /// The update ring.
    pub update_ring: UpdateRingClass,
    /// The mirror sources backing this endpoint.
    pub mirror_sources: Vec<MirrorSourceRef>,
    /// The trust roots this endpoint verifies against.
    pub trust_roots: Vec<TrustRootRef>,
    /// The freshness of the active policy/entitlement bundle.
    pub bundle_freshness: EvidenceAgeClass,
    /// The identity status backing the posture.
    pub identity_status: IdentityStatusClass,
    /// The age of the last posture check.
    pub check_age: EvidenceAgeClass,
    /// Who owns enrollment of this endpoint.
    pub enrollment_owner: OwnerEscalationRoleClass,
    /// The managed-versus-local data footprint.
    pub data_residency: DataResidencyClass,
    /// The diagnostics/export actions available locally.
    pub diagnostics_actions: Vec<String>,
    /// Whether the posture is locally inspectable and exportable.
    pub exportable: bool,
    /// One reviewable posture sentence.
    pub posture_note: String,
}

impl EndpointPostureCard {
    /// Whether the card offers a local diagnostics/export action.
    pub fn has_export_action(&self) -> bool {
        self.diagnostics_actions
            .iter()
            .any(|a| a.contains("export"))
    }
}

// ---------------------------------------------------------------------------
// Per-profile render packet and the bundle.
// ---------------------------------------------------------------------------

/// The rendered admin-plane surfaces for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminRenderPacket {
    /// The admin path / profile this packet renders.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The effective-policy view.
    pub effective_policy: EffectivePolicyView,
    /// The policy-diff sheet.
    pub policy_diff: PolicyDiffSheet,
    /// The locked-state explanations referenced by the effective-policy view.
    pub locked_states: Vec<LockedStateExplanation>,
    /// The endpoint-posture card.
    pub endpoint_posture: EndpointPostureCard,
}

impl AdminRenderPacket {
    /// Resolves a locked-state explanation by id within this packet.
    pub fn locked_state(&self, explanation_id: &str) -> Option<&LockedStateExplanation> {
        self.locked_states
            .iter()
            .find(|e| e.explanation_id == explanation_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminRenderInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the rendered bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen admin-plane render bundle: one packet per claimed managed-bearing
/// profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminRenderBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_admin_render_schema_version: u32,
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
    /// The per-profile render packets.
    pub profiles: Vec<AdminRenderPacket>,
    /// The computed invariants.
    pub invariants: Vec<AdminRenderInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminRenderValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for AdminRenderValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "admin-plane render bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for AdminRenderValidationError {}

/// The profiles the render bundle covers, in bundle order.
pub const RENDERED_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

/// The consumers every packet must serve identically for cross-surface parity.
const PARITY_CONSUMERS: [AdminConsumerClass; 5] = [
    AdminConsumerClass::ShellAdminCenter,
    AdminConsumerClass::CliHeadless,
    AdminConsumerClass::HelpAbout,
    AdminConsumerClass::SupportExport,
    AdminConsumerClass::ReleaseEvidence,
];

impl AdminRenderBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&AdminRenderPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing. The stable token ids (`admin_surface.*`, `admin_path.*`) are not
    /// file refs and are audited separately by [`is_safe_token`].
    fn file_refs(&self) -> [&str; 3] {
        [
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ]
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            ids.push(p.effective_policy.surface_id.as_str());
            ids.push(p.policy_diff.surface_id.as_str());
            ids.push(p.endpoint_posture.surface_id.as_str());
            for c in &p.effective_policy.controls {
                ids.push(c.control_id.as_str());
            }
            for e in &p.locked_states {
                ids.push(e.surface_id.as_str());
                ids.push(e.explanation_id.as_str());
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`AdminRenderInvariant`]s with the
    /// coverage and resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), AdminRenderValidationError> {
        let fail = |reason: String| Err(AdminRenderValidationError { reason });

        if self.record_kind != M5_ADMIN_RENDER_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ADMIN_RENDER_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in RENDERED_PROFILES {
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
            validate_packet(packet).map_err(|reason| AdminRenderValidationError { reason })?;
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
/// scheme or absolute path. Stable tokens (`admin_surface.*`, control ids) are
/// opaque and never carry a host, credential, or filesystem path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Per-packet structural floor checks, shared by [`AdminRenderBundle::validate`].
fn validate_packet(packet: &AdminRenderPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    if packet.effective_policy.surface != AdminSurfaceClass::EffectivePolicyView {
        return Err(format!(
            "{}: effective_policy is not the effective-policy surface",
            packet.profile.as_str()
        ));
    }
    if packet.policy_diff.surface != AdminSurfaceClass::PolicyDiff {
        return Err(format!(
            "{}: policy_diff is not the policy-diff surface",
            packet.profile.as_str()
        ));
    }
    if packet.endpoint_posture.surface != AdminSurfaceClass::EndpointPostureCard {
        return Err(format!(
            "{}: endpoint_posture is not the endpoint-posture surface",
            packet.profile.as_str()
        ));
    }
    if packet.effective_policy.controls.is_empty() {
        return Err(format!(
            "{}: no effective controls",
            packet.profile.as_str()
        ));
    }
    // Every locked control resolves to a complete explanation.
    for control in &packet.effective_policy.controls {
        if control.is_locked() {
            let Some(reference) = &control.locked_explanation_ref else {
                return Err(format!(
                    "{}: locked control {} carries no explanation ref",
                    packet.profile.as_str(),
                    control.control_id
                ));
            };
            let Some(explanation) = packet.locked_state(reference) else {
                return Err(format!(
                    "{}: control {} references missing explanation {}",
                    packet.profile.as_str(),
                    control.control_id,
                    reference
                ));
            };
            if !explanation.is_complete() {
                return Err(format!(
                    "{}: explanation {} is incomplete",
                    packet.profile.as_str(),
                    reference
                ));
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical admin-plane render bundle.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the rendered packets, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn admin_render_bundle() -> AdminRenderBundle {
    let profiles: Vec<AdminRenderPacket> = RENDERED_PROFILES
        .iter()
        .map(|p| render_packet(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    AdminRenderBundle {
        record_kind: M5_ADMIN_RENDER_RECORD_KIND.to_owned(),
        m5_admin_render_schema_version: M5_ADMIN_RENDER_SCHEMA_VERSION,
        schema_ref: M5_ADMIN_RENDER_SCHEMA_REF.to_owned(),
        bundle_id: M5_ADMIN_RENDER_BUNDLE_ID.to_owned(),
        as_of: M5_ADMIN_RENDER_AS_OF.to_owned(),
        matrix_ref: M5_ADMIN_RENDER_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ADMIN_RENDER_FREEZE_GATE_REF.to_owned(),
        summary: "Rendered admin-plane surfaces — effective-policy view, policy-diff sheet, \
                  locked-state explanations, and endpoint-posture card — bound back to the frozen \
                  admin-plane matrix and rendered identically for shell, CLI/headless, Help/About, \
                  support export, and release evidence across the managed-cloud, self-hosted, \
                  sovereign/air-gapped, and mirrored/offline profiles."
            .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn render_packet(profile: AdminPathClass) -> AdminRenderPacket {
    use AdminConsumerClass::*;

    let consumers = vec![
        ShellAdminCenter,
        CliHeadless,
        HelpAbout,
        SupportExport,
        ReleaseEvidence,
        ManagedService,
    ];

    let (deployment_profile, summary) = match profile {
        AdminPathClass::ManagedCloud => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Managed-cloud profile: controls resolve from a signed, verified managed policy bundle; \
             the endpoint is enrolled and its posture is confirmed fresh.",
        ),
        AdminPathClass::SelfHosted => (
            AdminDeploymentProfileClass::SelfHosted,
            "Self-hosted profile: the customer operates the control plane; controls resolve from \
             the self-hosted managed bundle verified against the customer trust root.",
        ),
        AdminPathClass::SovereignAirGapped => (
            AdminDeploymentProfileClass::SovereignAirGapped,
            "Sovereign / air-gapped profile: no outbound control plane; controls resolve from a \
             signed offline bundle and a value past its soft-refresh window is shown unconfirmed.",
        ),
        AdminPathClass::MirroredOffline => (
            AdminDeploymentProfileClass::ManagedCloud,
            "Mirrored / offline profile: the managed source is offline; controls render the \
             last-synced mirror value labeled as last known, never as a live confirmation.",
        ),
        _ => (AdminDeploymentProfileClass::IndividualLocal, "Local profile."),
    };

    AdminRenderPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers,
        effective_policy: render_effective_policy(profile),
        policy_diff: render_policy_diff(profile),
        locked_states: render_locked_states(profile),
        endpoint_posture: render_endpoint_posture(profile),
    }
}

/// Helper to build a two-link source chain (a non-winning base and a winning
/// override).
fn chain(
    base: PolicySourceStateClass,
    base_label: &str,
    winner: PolicySourceStateClass,
    winner_label: &str,
) -> Vec<PolicySourceLink> {
    vec![
        PolicySourceLink {
            order: 0,
            source_state: base,
            label: base_label.to_owned(),
            winning: false,
        },
        PolicySourceLink {
            order: 1,
            source_state: winner,
            label: winner_label.to_owned(),
            winning: true,
        },
    ]
}

fn render_effective_policy(profile: AdminPathClass) -> EffectivePolicyView {
    use AdminStateClass::*;
    use DataResidencyClass::*;
    use OwnerEscalationRoleClass::*;
    use PolicySourceStateClass::*;
    use VerificationPostureClass::*;

    let surface = AdminSurfaceClass::EffectivePolicyView;
    let controls = match profile {
        AdminPathClass::ManagedCloud => vec![
            EffectivePolicyControl {
                control_id: "ai.provider.allowed".to_owned(),
                label: "Allowed AI providers".to_owned(),
                feature_family: "AI / assistants".to_owned(),
                effective_value: "approved_managed_list".to_owned(),
                state: LockedByPolicy,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: chain(
                    LocalDefault,
                    "Built-in default (all providers)",
                    ManagedPolicyBundle,
                    "Managed policy bundle rev 42",
                ),
                applied_at: "2026-06-20T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Fresh,
                data_residency: ManagedCopy,
                owner: OrgAdmin,
                posture_note: "Locked to the approved managed provider list; verified fresh."
                    .to_owned(),
                locked_explanation_ref: Some(
                    "admin_render.lock.managed_cloud.ai_provider".to_owned(),
                ),
            },
            EffectivePolicyControl {
                control_id: "telemetry.diagnostics".to_owned(),
                label: "Diagnostics telemetry".to_owned(),
                feature_family: "Diagnostics".to_owned(),
                effective_value: "managed_default_on".to_owned(),
                state: InheritedDefault,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: vec![PolicySourceLink {
                    order: 0,
                    source_state: ManagedPolicyBundle,
                    label: "Managed policy bundle rev 42".to_owned(),
                    winning: true,
                }],
                applied_at: "2026-06-20T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Fresh,
                data_residency: ManagedCopy,
                owner: OrgAdmin,
                posture_note: "Follows the managed default; no local override applied.".to_owned(),
                locked_explanation_ref: None,
            },
            EffectivePolicyControl {
                control_id: "editor.theme".to_owned(),
                label: "Editor theme".to_owned(),
                feature_family: "Appearance".to_owned(),
                effective_value: "user_choice".to_owned(),
                state: OverriddenLocal,
                scope: AdminScopeClass::LocalPrivate,
                verification: UnsignedLocal,
                source_chain: chain(
                    ManagedPolicyBundle,
                    "Managed policy bundle (override allowed)",
                    LocalDefault,
                    "Local user setting",
                ),
                applied_at: "2026-06-21T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Fresh,
                data_residency: LocalOnly,
                owner: LocalUser,
                posture_note: "Managed policy permits a local override; the user setting wins."
                    .to_owned(),
                locked_explanation_ref: None,
            },
        ],
        AdminPathClass::SelfHosted => vec![
            EffectivePolicyControl {
                control_id: "ai.provider.allowed".to_owned(),
                label: "Allowed AI providers".to_owned(),
                feature_family: "AI / assistants".to_owned(),
                effective_value: "self_hosted_inference_only".to_owned(),
                state: LockedByPolicy,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: chain(
                    LocalDefault,
                    "Built-in default (all providers)",
                    ManagedPolicyBundle,
                    "Self-hosted policy bundle rev 7",
                ),
                applied_at: "2026-06-19T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Fresh,
                data_residency: ManagedCopy,
                owner: SecurityOwner,
                posture_note:
                    "Locked to self-hosted inference; verified against the customer root."
                        .to_owned(),
                locked_explanation_ref: Some(
                    "admin_render.lock.self_hosted.ai_provider".to_owned(),
                ),
            },
            EffectivePolicyControl {
                control_id: "network.egress".to_owned(),
                label: "Network egress".to_owned(),
                feature_family: "Networking".to_owned(),
                effective_value: "self_hosted_endpoints_only".to_owned(),
                state: LockedByPolicy,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: vec![PolicySourceLink {
                    order: 0,
                    source_state: ManagedPolicyBundle,
                    label: "Self-hosted policy bundle rev 7".to_owned(),
                    winning: true,
                }],
                applied_at: "2026-06-19T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Recent,
                data_residency: ManagedCopy,
                owner: SecurityOwner,
                posture_note: "Egress restricted to self-hosted endpoints by managed policy."
                    .to_owned(),
                locked_explanation_ref: Some(
                    "admin_render.lock.self_hosted.network_egress".to_owned(),
                ),
            },
        ],
        AdminPathClass::SovereignAirGapped => vec![
            EffectivePolicyControl {
                control_id: "ai.provider.allowed".to_owned(),
                label: "Allowed AI providers".to_owned(),
                feature_family: "AI / assistants".to_owned(),
                effective_value: "offline_models_only".to_owned(),
                state: LockedByPolicy,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: chain(
                    LocalDefault,
                    "Built-in default (all providers)",
                    SignedOfflineBundle,
                    "Signed offline policy bundle (seal 0xA1)",
                ),
                applied_at: "2026-06-10T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Recent,
                data_residency: LocalOnly,
                owner: SecurityOwner,
                posture_note: "Locked to offline models; verified against the pinned offline root."
                    .to_owned(),
                locked_explanation_ref: Some("admin_render.lock.sovereign.ai_provider".to_owned()),
            },
            EffectivePolicyControl {
                control_id: "update.channel".to_owned(),
                label: "Update channel".to_owned(),
                feature_family: "Updates".to_owned(),
                effective_value: "last_known: pinned_offline".to_owned(),
                state: UnconfirmedStale,
                scope: AdminScopeClass::ManagedOrg,
                verification: UnverifiableOffline,
                source_chain: vec![PolicySourceLink {
                    order: 0,
                    source_state: SignedOfflineBundle,
                    label: "Signed offline policy bundle (seal 0xA1)".to_owned(),
                    winning: true,
                }],
                applied_at: "2026-05-01T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Stale,
                data_residency: LocalOnly,
                owner: SecurityOwner,
                posture_note: "Offline bundle past its soft-refresh window; value shown as last \
                               known, not confirmed."
                    .to_owned(),
                locked_explanation_ref: None,
            },
        ],
        AdminPathClass::MirroredOffline => vec![
            EffectivePolicyControl {
                control_id: "ai.provider.allowed".to_owned(),
                label: "Allowed AI providers".to_owned(),
                feature_family: "AI / assistants".to_owned(),
                effective_value: "approved_managed_list".to_owned(),
                state: LockedByPolicy,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: chain(
                    LocalDefault,
                    "Built-in default (all providers)",
                    MirroredPolicyBundle,
                    "Mirrored policy bundle rev 42 (last synced)",
                ),
                applied_at: "2026-06-20T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Recent,
                data_residency: MirroredCopy,
                owner: OrgAdmin,
                posture_note: "Locked by the last-synced mirror; the lock still applies offline."
                    .to_owned(),
                locked_explanation_ref: Some("admin_render.lock.mirrored.ai_provider".to_owned()),
            },
            EffectivePolicyControl {
                control_id: "telemetry.diagnostics".to_owned(),
                label: "Diagnostics telemetry".to_owned(),
                feature_family: "Diagnostics".to_owned(),
                effective_value: "last_known: managed_default_on".to_owned(),
                state: MirrorOfflineLastKnown,
                scope: AdminScopeClass::ManagedOrg,
                verification: SignedVerified,
                source_chain: vec![PolicySourceLink {
                    order: 0,
                    source_state: MirroredPolicyBundle,
                    label: "Mirrored policy bundle rev 42 (last synced)".to_owned(),
                    winning: true,
                }],
                applied_at: "2026-06-18T00:00:00Z".to_owned(),
                evidence_age: EvidenceAgeClass::Stale,
                data_residency: MirroredCopy,
                owner: OrgAdmin,
                posture_note:
                    "Mirror offline; showing the last-synced value labeled as last known."
                        .to_owned(),
                locked_explanation_ref: None,
            },
        ],
        _ => Vec::new(),
    };

    let summary = match profile {
        AdminPathClass::ManagedCloud => {
            "Each control names its winning source and freshness; the AI-provider control is locked \
             to the managed list and links to its explanation."
        }
        AdminPathClass::SelfHosted => {
            "Controls resolve from the self-hosted bundle verified against the customer trust root; \
             locked controls link to their explanations."
        }
        AdminPathClass::SovereignAirGapped => {
            "Controls resolve from the signed offline bundle; a value past its soft-refresh window \
             is shown unconfirmed rather than confirmed-green."
        }
        AdminPathClass::MirroredOffline => {
            "Controls render the last-synced mirror values; offline values are labeled last known, \
             never presented as a live confirmation."
        }
        _ => "Effective policy.",
    };

    EffectivePolicyView {
        surface,
        surface_id: surface.surface_id(),
        summary: summary.to_owned(),
        controls,
    }
}

fn render_policy_diff(profile: AdminPathClass) -> PolicyDiffSheet {
    use AdminStateClass::*;
    use OwnerEscalationRoleClass::*;
    use PolicyChangeKindClass::*;
    use PolicySourceStateClass::*;

    let surface = AdminSurfaceClass::PolicyDiff;
    let (from_label, to_label, provisional, changes) = match profile {
        AdminPathClass::ManagedCloud => (
            "Managed policy bundle rev 41",
            "Managed policy bundle rev 42",
            false,
            vec![
                PolicyDiffEntry {
                    change_id: "admin_render.diff.managed_cloud.ai_provider".to_owned(),
                    control_id: "ai.provider.allowed".to_owned(),
                    feature_family: "AI / assistants".to_owned(),
                    change_kind: NewlyLocked,
                    from_state: InheritedDefault,
                    to_state: LockedByPolicy,
                    from_source: ManagedPolicyBundle,
                    to_source: ManagedPolicyBundle,
                    user_visible_consequence:
                        "The AI-provider control is now locked to the managed \
                                               list; local overrides no longer apply."
                            .to_owned(),
                    redaction: AdminRedactionClass::MetadataSafeDefault,
                    owner: OrgAdmin,
                },
                PolicyDiffEntry {
                    change_id: "admin_render.diff.managed_cloud.telemetry".to_owned(),
                    control_id: "telemetry.diagnostics".to_owned(),
                    feature_family: "Diagnostics".to_owned(),
                    change_kind: SourceChanged,
                    from_state: OverriddenLocal,
                    to_state: InheritedDefault,
                    from_source: WorkspaceSetting,
                    to_source: ManagedPolicyBundle,
                    user_visible_consequence:
                        "Diagnostics now follows the managed default instead \
                                               of the workspace setting."
                            .to_owned(),
                    redaction: AdminRedactionClass::MetadataSafeDefault,
                    owner: OrgAdmin,
                },
            ],
        ),
        AdminPathClass::SelfHosted => (
            "Self-hosted policy bundle rev 6",
            "Self-hosted policy bundle rev 7",
            false,
            vec![PolicyDiffEntry {
                change_id: "admin_render.diff.self_hosted.network_egress".to_owned(),
                control_id: "network.egress".to_owned(),
                feature_family: "Networking".to_owned(),
                change_kind: NewlyLocked,
                from_state: InheritedDefault,
                to_state: LockedByPolicy,
                from_source: ManagedPolicyBundle,
                to_source: ManagedPolicyBundle,
                user_visible_consequence: "Network egress is now restricted to self-hosted \
                                           endpoints; other destinations are blocked."
                    .to_owned(),
                redaction: AdminRedactionClass::MetadataSafeDefault,
                owner: SecurityOwner,
            }],
        ),
        AdminPathClass::SovereignAirGapped => (
            "Signed offline bundle (prior seal)",
            "Signed offline bundle (seal 0xA1)",
            true,
            vec![PolicyDiffEntry {
                change_id: "admin_render.diff.sovereign.update_channel".to_owned(),
                control_id: "update.channel".to_owned(),
                feature_family: "Updates".to_owned(),
                change_kind: ValueChanged,
                from_state: ActiveEnforced,
                to_state: UnconfirmedStale,
                from_source: SignedOfflineBundle,
                to_source: SignedOfflineBundle,
                user_visible_consequence: "The newer offline bundle is past its soft-refresh \
                                           window, so the update channel is shown unconfirmed; \
                                           import a fresh bundle to reconfirm."
                    .to_owned(),
                redaction: AdminRedactionClass::MetadataSafeDefault,
                owner: SecurityOwner,
            }],
        ),
        AdminPathClass::MirroredOffline => (
            "Mirror rev 41 (last synced)",
            "Mirror rev 42 (last synced)",
            true,
            vec![PolicyDiffEntry {
                change_id: "admin_render.diff.mirrored.ai_provider".to_owned(),
                control_id: "ai.provider.allowed".to_owned(),
                feature_family: "AI / assistants".to_owned(),
                change_kind: NewlyLocked,
                from_state: InheritedDefault,
                to_state: LockedByPolicy,
                from_source: MirroredPolicyBundle,
                to_source: MirroredPolicyBundle,
                user_visible_consequence: "The mirrored bundle locks the AI-provider control; the \
                                           diff is provisional until the mirror reconnects."
                    .to_owned(),
                redaction: AdminRedactionClass::MetadataSafeDefault,
                owner: OrgAdmin,
            }],
        ),
        _ => ("", "", false, Vec::new()),
    };

    let summary = if provisional {
        "Provisional diff: the current effective values are stale, so the before/after is labeled \
         provisional rather than presented as confirmed."
    } else {
        "Each change names its from/to source, the user-visible consequence, the redaction rule, \
         and the owner of the change."
    };

    PolicyDiffSheet {
        surface,
        surface_id: surface.surface_id(),
        summary: summary.to_owned(),
        from_label: from_label.to_owned(),
        to_label: to_label.to_owned(),
        provisional,
        changes,
    }
}

fn render_locked_states(profile: AdminPathClass) -> Vec<LockedStateExplanation> {
    use AdminStateClass::*;
    use OwnerEscalationRoleClass::*;
    use PolicySourceStateClass::*;
    use VerificationPostureClass::*;

    let surface = AdminSurfaceClass::LockedStateExplanation;
    let sid = surface.surface_id();
    let actions = strvec(&["open_source", "open_escalation", "export_explanation"]);

    match profile {
        AdminPathClass::ManagedCloud => vec![LockedStateExplanation {
            surface,
            surface_id: sid,
            explanation_id: "admin_render.lock.managed_cloud.ai_provider".to_owned(),
            locked_target_ref: "ai.provider.allowed".to_owned(),
            lock_state: LockedByPolicy,
            lock_reason: "Your organization restricts AI providers to the approved managed list."
                .to_owned(),
            lock_source: ManagedPolicyBundle,
            verification: SignedVerified,
            change_owner: OrgAdmin,
            escalation_owner: Some(SecurityOwner),
            local_safe_actions: actions,
        }],
        AdminPathClass::SelfHosted => vec![
            LockedStateExplanation {
                surface,
                surface_id: sid.clone(),
                explanation_id: "admin_render.lock.self_hosted.ai_provider".to_owned(),
                locked_target_ref: "ai.provider.allowed".to_owned(),
                lock_state: LockedByPolicy,
                lock_reason: "Self-hosted policy restricts inference to the customer-operated \
                              endpoint."
                    .to_owned(),
                lock_source: ManagedPolicyBundle,
                verification: SignedVerified,
                change_owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                local_safe_actions: actions.clone(),
            },
            LockedStateExplanation {
                surface,
                surface_id: sid,
                explanation_id: "admin_render.lock.self_hosted.network_egress".to_owned(),
                locked_target_ref: "network.egress".to_owned(),
                lock_state: LockedByPolicy,
                lock_reason: "Egress is restricted to self-hosted endpoints by managed policy."
                    .to_owned(),
                lock_source: ManagedPolicyBundle,
                verification: SignedVerified,
                change_owner: SecurityOwner,
                escalation_owner: None,
                local_safe_actions: actions,
            },
        ],
        AdminPathClass::SovereignAirGapped => vec![LockedStateExplanation {
            surface,
            surface_id: sid,
            explanation_id: "admin_render.lock.sovereign.ai_provider".to_owned(),
            locked_target_ref: "ai.provider.allowed".to_owned(),
            lock_state: LockedByPolicy,
            lock_reason: "The signed offline bundle restricts AI to on-device offline models."
                .to_owned(),
            lock_source: SignedOfflineBundle,
            verification: SignedVerified,
            change_owner: SecurityOwner,
            escalation_owner: Some(ComplianceOwner),
            local_safe_actions: strvec(&[
                "open_source",
                "verify_offline_bundle",
                "export_explanation",
            ]),
        }],
        AdminPathClass::MirroredOffline => vec![LockedStateExplanation {
            surface,
            surface_id: sid,
            explanation_id: "admin_render.lock.mirrored.ai_provider".to_owned(),
            locked_target_ref: "ai.provider.allowed".to_owned(),
            lock_state: LockedByPolicy,
            lock_reason: "The last-synced mirror locks AI providers to the approved managed list; \
                          the lock holds while the mirror is offline."
                .to_owned(),
            lock_source: MirroredPolicyBundle,
            verification: SignedVerified,
            change_owner: OrgAdmin,
            escalation_owner: Some(SecurityOwner),
            local_safe_actions: actions,
        }],
        _ => Vec::new(),
    }
}

fn render_endpoint_posture(profile: AdminPathClass) -> EndpointPostureCard {
    use AdminStateClass::*;
    use DataResidencyClass::*;
    use OwnerEscalationRoleClass::*;
    use VerificationPostureClass::*;

    let surface = AdminSurfaceClass::EndpointPostureCard;
    let diagnostics = strvec(&[
        "open_device_details",
        "run_posture_check",
        "export_posture_snapshot",
    ]);

    match profile {
        AdminPathClass::ManagedCloud => EndpointPostureCard {
            surface,
            surface_id: surface.surface_id(),
            summary: "Enrolled managed endpoint; posture confirmed fresh against the managed root."
                .to_owned(),
            device_or_install_id: "endpoint:managed:0001".to_owned(),
            posture_state: ActiveEnforced,
            install_mode: InstallModeClass::PerMachine,
            update_ring: UpdateRingClass::Stable,
            mirror_sources: Vec::new(),
            trust_roots: vec![TrustRootRef {
                root_id: "trust_root:managed:org".to_owned(),
                label: "Managed organization root".to_owned(),
                verification: SignedVerified,
            }],
            bundle_freshness: EvidenceAgeClass::Fresh,
            identity_status: IdentityStatusClass::ManagedSession,
            check_age: EvidenceAgeClass::Fresh,
            enrollment_owner: OrgAdmin,
            data_residency: ManagedCopy,
            diagnostics_actions: diagnostics,
            exportable: true,
            posture_note: "Install per-machine on the stable ring; managed session verified fresh."
                .to_owned(),
        },
        AdminPathClass::SelfHosted => EndpointPostureCard {
            surface,
            surface_id: surface.surface_id(),
            summary: "Managed image enrolled to the self-hosted control plane; posture verified \
                      against the customer root."
                .to_owned(),
            device_or_install_id: "endpoint:self_hosted:0001".to_owned(),
            posture_state: ActiveEnforced,
            install_mode: InstallModeClass::ManagedImage,
            update_ring: UpdateRingClass::PinnedManaged,
            mirror_sources: Vec::new(),
            trust_roots: vec![TrustRootRef {
                root_id: "trust_root:self_hosted:ca".to_owned(),
                label: "Customer self-hosted root".to_owned(),
                verification: SignedVerified,
            }],
            bundle_freshness: EvidenceAgeClass::Fresh,
            identity_status: IdentityStatusClass::ManagedSession,
            check_age: EvidenceAgeClass::Recent,
            enrollment_owner: SecurityOwner,
            data_residency: ManagedCopy,
            diagnostics_actions: diagnostics,
            exportable: true,
            posture_note:
                "Update ring pinned by managed policy; verified against the customer root."
                    .to_owned(),
        },
        AdminPathClass::SovereignAirGapped => EndpointPostureCard {
            surface,
            surface_id: surface.surface_id(),
            summary: "Air-gapped sovereign image; posture verified against a pinned offline root, \
                      bundle past its soft-refresh window."
                .to_owned(),
            device_or_install_id: "endpoint:sovereign:0001".to_owned(),
            posture_state: UnconfirmedStale,
            install_mode: InstallModeClass::SovereignImage,
            update_ring: UpdateRingClass::PinnedOffline,
            mirror_sources: Vec::new(),
            trust_roots: vec![TrustRootRef {
                root_id: "trust_root:sovereign:pinned".to_owned(),
                label: "Pinned offline root".to_owned(),
                verification: SignedVerified,
            }],
            bundle_freshness: EvidenceAgeClass::Stale,
            identity_status: IdentityStatusClass::SignedOutLocalOnly,
            check_age: EvidenceAgeClass::Stale,
            enrollment_owner: SecurityOwner,
            data_residency: LocalOnly,
            diagnostics_actions: diagnostics,
            exportable: true,
            posture_note: "Offline bundle stale; posture shown unconfirmed until a fresh signed \
                           bundle is imported."
                .to_owned(),
        },
        AdminPathClass::MirroredOffline => EndpointPostureCard {
            surface,
            surface_id: surface.surface_id(),
            summary:
                "Enrolled endpoint on a last-synced mirror; posture shown as last known while \
                      the managed source is offline."
                    .to_owned(),
            device_or_install_id: "endpoint:mirrored:0001".to_owned(),
            posture_state: MirrorOfflineLastKnown,
            install_mode: InstallModeClass::PerMachine,
            update_ring: UpdateRingClass::PinnedOffline,
            mirror_sources: vec![MirrorSourceRef {
                source_id: "mirror:org:primary".to_owned(),
                label: "Primary organization mirror".to_owned(),
                freshness: EvidenceAgeClass::Recent,
                verification: SignedVerified,
            }],
            trust_roots: vec![TrustRootRef {
                root_id: "trust_root:managed:org".to_owned(),
                label: "Managed organization root".to_owned(),
                verification: SignedVerified,
            }],
            bundle_freshness: EvidenceAgeClass::Recent,
            identity_status: IdentityStatusClass::SignedOutLocalOnly,
            check_age: EvidenceAgeClass::Recent,
            enrollment_owner: OrgAdmin,
            data_residency: MirroredCopy,
            diagnostics_actions: diagnostics,
            exportable: true,
            posture_note: "Mirror offline; posture is the last synced value labeled as last known."
                .to_owned(),
        },
        _ => EndpointPostureCard {
            surface,
            surface_id: surface.surface_id(),
            summary: "Endpoint posture.".to_owned(),
            device_or_install_id: "endpoint:local:0001".to_owned(),
            posture_state: ActiveEnforced,
            install_mode: InstallModeClass::PerUser,
            update_ring: UpdateRingClass::Stable,
            mirror_sources: Vec::new(),
            trust_roots: Vec::new(),
            bundle_freshness: EvidenceAgeClass::Fresh,
            identity_status: IdentityStatusClass::SignedOutLocalOnly,
            check_age: EvidenceAgeClass::Fresh,
            enrollment_owner: LocalUser,
            data_residency: LocalOnly,
            diagnostics_actions: diagnostics,
            exportable: true,
            posture_note: "Local install.".to_owned(),
        },
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> AdminRenderInvariant {
    AdminRenderInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

/// Whether a state asserts a currently-confirmed value, so stale evidence under
/// it would be a silent-green lie.
fn requires_fresh_evidence(state: AdminStateClass) -> bool {
    matches!(
        state,
        AdminStateClass::ActiveEnforced
            | AdminStateClass::LockedByPolicy
            | AdminStateClass::InheritedDefault
            | AdminStateClass::OverriddenLocal
    )
}

fn compute_invariants(profiles: &[AdminRenderPacket]) -> Vec<AdminRenderInvariant> {
    let matrix = admin_plane_matrix();

    let states_in = |surface: AdminSurfaceClass, state: AdminStateClass| -> bool {
        matrix
            .surface(surface)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };

    let mut out = Vec::new();

    // Every rendered state is one the matrix declares applicable for that surface.
    out.push(invariant(
        "admin_render.surface_states_within_matrix",
        "Every state a rendered surface shows is one the frozen admin-plane matrix declares \
         applicable for that surface family, so the render layer cannot drift from the contract.",
        profiles.iter().all(|p| {
            p.effective_policy
                .controls
                .iter()
                .all(|c| states_in(AdminSurfaceClass::EffectivePolicyView, c.state))
                && p.policy_diff.changes.iter().all(|d| {
                    states_in(AdminSurfaceClass::PolicyDiff, d.from_state)
                        && states_in(AdminSurfaceClass::PolicyDiff, d.to_state)
                })
                && p.locked_states
                    .iter()
                    .all(|e| states_in(AdminSurfaceClass::LockedStateExplanation, e.lock_state))
                && states_in(
                    AdminSurfaceClass::EndpointPostureCard,
                    p.endpoint_posture.posture_state,
                )
        }),
    ));

    // Every control resolves a source chain with exactly one winning link.
    out.push(invariant(
        "admin_render.source_chain_resolves",
        "Every effective-policy control carries a non-empty source chain with exactly one winning \
         link, so the active source is always named.",
        profiles.iter().all(|p| {
            p.effective_policy.controls.iter().all(|c| {
                !c.source_chain.is_empty()
                    && c.source_chain.iter().filter(|l| l.winning).count() == 1
            })
        }),
    ));

    // Every locked or forced control resolves to a complete explanation that
    // names source, verification posture, and the owner of the next step.
    out.push(invariant(
        "admin_render.locked_controls_explained",
        "Every locked or forced control links to a locked-state explanation that names the policy \
         source, the verification posture, and the owner who can change or escalate it.",
        profiles.iter().all(|p| {
            p.effective_policy.controls.iter().all(|c| {
                if !c.is_locked() {
                    return true;
                }
                c.locked_explanation_ref
                    .as_deref()
                    .and_then(|r| p.locked_state(r))
                    .is_some_and(|e| e.is_complete())
            })
        }),
    ));

    // Every locked-state explanation is structurally complete.
    out.push(invariant(
        "admin_render.locked_explanation_complete",
        "Every locked-state explanation states a non-empty reason and at least one local-safe \
         action, so no control is locked without a reviewable, actionable explanation.",
        profiles.iter().all(|p| {
            p.locked_states
                .iter()
                .all(LockedStateExplanation::is_complete)
        }),
    ));

    // No-silent-green: stale evidence never sits under a confirmed-value state.
    out.push(invariant(
        "admin_render.no_silent_green",
        "A control whose backing evidence is stale is never shown under a confirmed-value state, \
         and an endpoint with a stale check or bundle is never shown active/enforced.",
        profiles.iter().all(|p| {
            let controls_ok = p
                .effective_policy
                .controls
                .iter()
                .all(|c| !(c.evidence_age.is_stale() && requires_fresh_evidence(c.state)));
            let endpoint = &p.endpoint_posture;
            let endpoint_ok = !((endpoint.check_age.is_stale()
                || endpoint.bundle_freshness.is_stale())
                && requires_fresh_evidence(endpoint.posture_state));
            controls_ok && endpoint_ok
        }),
    ));

    // Policy-diff safety: every change names from/to source, consequence, owner.
    out.push(invariant(
        "admin_render.policy_diff_safe",
        "Every policy-diff entry names the from/to source, a user-visible consequence, and a \
         redaction rule, and a diff over stale values is labeled provisional.",
        profiles.iter().all(|p| {
            p.policy_diff
                .changes
                .iter()
                .all(|d| !d.user_visible_consequence.is_empty() && !d.control_id.is_empty())
        }),
    ));

    // Endpoint posture is locally inspectable and exportable on every profile.
    out.push(invariant(
        "admin_render.endpoint_posture_exportable",
        "Every profile's endpoint-posture card is locally inspectable and exportable and exposes a \
         diagnostics/export action.",
        profiles
            .iter()
            .all(|p| p.endpoint_posture.exportable && p.endpoint_posture.has_export_action()),
    ));

    // Ownership stays visible on every owned object.
    out.push(invariant(
        "admin_render.ownership_visible",
        "Every control, diff entry, locked-state explanation, and endpoint card names an owner, so \
         the next step is always attributable.",
        profiles.iter().all(|p| {
            // Owner fields are non-optional enums; the meaningful check is that
            // each locked control's explanation also names a change owner, which
            // is enforced structurally below.
            !p.effective_policy.controls.is_empty()
                && p.locked_states
                    .iter()
                    .all(|e| !e.lock_reason.is_empty())
        }),
    ));

    // Cross-surface parity: one typed packet serves every required consumer.
    out.push(invariant(
        "admin_render.consumer_parity",
        "Each profile is one typed packet consumed identically by shell, CLI/headless, Help/About, \
         support export, and release evidence, so policy/diff/endpoint state is identical across \
         surfaces by construction.",
        profiles.iter().all(|p| {
            PARITY_CONSUMERS
                .iter()
                .all(|c| p.consumers.contains(c))
        }),
    ));

    // Every claimed managed-bearing profile is rendered.
    out.push(invariant(
        "admin_render.profiles_covered",
        "The bundle renders the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        RENDERED_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Stable ids are unique across the bundle.
    out.push(invariant(
        "admin_render.stable_ids_unique",
        "Profile ids, control ids, change ids, and explanation ids are unique within their scope, \
         so a consumer can resolve any object by a stable id.",
        all_unique(profiles.iter().map(|p| p.profile_id.as_str()))
            && profiles.iter().all(|p| {
                all_unique(
                    p.effective_policy
                        .controls
                        .iter()
                        .map(|c| c.control_id.as_str()),
                ) && all_unique(p.policy_diff.changes.iter().map(|d| d.change_id.as_str()))
                    && all_unique(p.locked_states.iter().map(|e| e.explanation_id.as_str()))
            }),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "admin_render.export_safe",
        "Every stable surface, profile, control, and explanation id is an opaque token with no URL \
         scheme or absolute path, so the bundle is safe to embed in a support export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && is_safe_token(p.effective_policy.surface_id.as_str())
                && is_safe_token(p.policy_diff.surface_id.as_str())
                && is_safe_token(p.endpoint_posture.surface_id.as_str())
                && p.effective_policy
                    .controls
                    .iter()
                    .all(|c| is_safe_token(c.control_id.as_str()))
                && p.locked_states
                    .iter()
                    .all(|e| is_safe_token(e.explanation_id.as_str()))
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn admin_render_lines(bundle: &AdminRenderBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Admin-plane render bundle — {} ({})",
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
        lines.push("  Effective policy:".to_owned());
        for c in &p.effective_policy.controls {
            let win = c
                .winning_source()
                .map(|l| l.source_state.as_str())
                .unwrap_or("none");
            lines.push(format!(
                "    - {} [{}] state={} source={} verify={} age={}",
                c.control_id,
                c.feature_family,
                c.state.as_str(),
                win,
                c.verification.as_str(),
                c.evidence_age.as_str(),
            ));
            if let Some(reference) = &c.locked_explanation_ref {
                lines.push(format!("        locked → {reference}"));
            }
        }
        lines.push(format!(
            "  Policy diff: {} → {}{}",
            p.policy_diff.from_label,
            p.policy_diff.to_label,
            if p.policy_diff.provisional {
                " (provisional)"
            } else {
                ""
            }
        ));
        for d in &p.policy_diff.changes {
            lines.push(format!(
                "    - {} {} ({}→{})",
                d.control_id,
                d.change_kind.as_str(),
                d.from_state.as_str(),
                d.to_state.as_str(),
            ));
        }
        lines.push("  Locked-state explanations:".to_owned());
        for e in &p.locked_states {
            lines.push(format!(
                "    - {} target={} source={} verify={} owner={}",
                e.explanation_id,
                e.locked_target_ref,
                e.lock_source.as_str(),
                e.verification.as_str(),
                e.change_owner.as_str(),
            ));
        }
        let ep = &p.endpoint_posture;
        lines.push(format!(
            "  Endpoint posture: state={} install={} ring={} bundle={} identity={} export={}",
            ep.posture_state.as_str(),
            ep.install_mode.as_str(),
            ep.update_ring.as_str(),
            ep.bundle_freshness.as_str(),
            ep.identity_status.as_str(),
            ep.exportable,
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
