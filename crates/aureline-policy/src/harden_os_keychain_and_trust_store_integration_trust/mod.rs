//! Hardened OS keychain and trust-store integration, trust-store change
//! detection, and repair-safe copy proof packet.
//!
//! This module produces a stable proof packet that demonstrates trust-store
//! handling follows a layered model distinguishing OS roots, custom CA bundles,
//! pinned SSH host proofs, client-certificate material, and imported mirror
//! trust roots. Every trust-store change emits an attributable, repair-safe
//! event instead of silently changing route posture under active sessions.
//!
//! For any trust material change, the packet can explain:
//!
//! 1. **Which layer changed** — via a closed [`TrustStoreLayerClass`]
//!    vocabulary that names the exact layer tier and its governance source.
//! 2. **Which requests or sessions are affected** — via per-event opaque route
//!    refs and typed [`SessionImpactClass`] declarations; no raw hostnames,
//!    private keys, or certificate bodies cross this boundary.
//! 3. **What can continue locally** — via an explicit
//!    `local_continuity_explicit` flag and a `local_continuity_label` on each
//!    change event and layer row.
//! 4. **Which narrow repair or revalidation step is required** before protected
//!    network paths resume — via a typed [`TrustStoreRepairActionClass`] and a
//!    stable `repair_transaction_id` that ties the change event, the affected
//!    layer, and the repair outcome together across admin, support, and export
//!    surfaces.
//!
//! The five required trust-store layers are: `os_roots`, `custom_ca_bundle`,
//! `pinned_ssh_host_proof`, `client_certificate`, and `mirror_trust_root`.
//! Each layer row carries its current health state and any pending change
//! events, so a review surface can answer trust-material questions with one
//! record instead of reconstructing state from logs.
//!
//! Surfaces (admin/settings center, support export, shell trust summary,
//! headless inspector) read [`seeded_harden_os_keychain_trust_store_page`]
//! rather than minting parallel trust-store state checks.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md`
//! - Artifact: `artifacts/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md`
//! - Contract ref: [`HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF: &str =
    "policy:harden_os_keychain_trust_store:v1";

/// Record-kind tag for [`HardenOsKeychainTrustStorePage`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_PAGE_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_page_record";

/// Record-kind tag for [`TrustStoreLayerRow`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_ROW_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_row_record";

/// Record-kind tag for [`TrustStoreChangeEvent`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_CHANGE_EVENT_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_change_event_record";

/// Record-kind tag for [`HardenOsKeychainTrustStoreDefect`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_DEFECT_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_defect_record";

/// Record-kind tag for [`HardenOsKeychainTrustStoreSummary`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_SUMMARY_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_summary_record";

/// Record-kind tag for [`HardenOsKeychainTrustStoreSupportExport`] payloads.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_harden_os_keychain_trust_store_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_DOC_REF: &str =
    "docs/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md";

/// Repo-relative path of the artifact summary for this lane.
pub const HARDEN_OS_KEYCHAIN_TRUST_STORE_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md";

// ---------------------------------------------------------------------------
// Trust-store layer vocabulary
// ---------------------------------------------------------------------------

/// Layer tier in the trust-store model.
///
/// These five layers form the required coverage set for the hardened OS
/// keychain and trust-store proof packet. Each layer row names its current
/// health state, governance source, and any pending change event so a review
/// surface can answer "which trust-store layer changed and why?" without
/// inspecting raw certificate or key material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreLayerClass {
    /// OS platform CA trust roots (the system-level bundle managed by the OS
    /// vendor or distribution).
    OsRoots,
    /// Custom or org-overlay CA bundle declared by admin policy, augmenting or
    /// replacing the OS roots for managed traffic.
    CustomCaBundle,
    /// Pinned SSH host proofs (known-host fingerprints for Git-over-SSH and
    /// managed-endpoint SSH sessions).
    PinnedSshHostProof,
    /// Client-certificate material (mTLS certificates enrolled for managed
    /// endpoints; presented on outbound connections).
    ClientCertificate,
    /// Imported mirror trust roots (CA or signing roots for declared signed
    /// mirrors and air-gapped artifact repositories).
    MirrorTrustRoot,
}

impl TrustStoreLayerClass {
    /// All required trust-store layers in canonical order.
    pub const ALL: [Self; 5] = [
        Self::OsRoots,
        Self::CustomCaBundle,
        Self::PinnedSshHostProof,
        Self::ClientCertificate,
        Self::MirrorTrustRoot,
    ];

    /// Stable token recorded on rows and events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsRoots => "os_roots",
            Self::CustomCaBundle => "custom_ca_bundle",
            Self::PinnedSshHostProof => "pinned_ssh_host_proof",
            Self::ClientCertificate => "client_certificate",
            Self::MirrorTrustRoot => "mirror_trust_root",
        }
    }

    /// True when changes to this layer may affect managed or enterprise routes
    /// and require explicit local-continuity documentation.
    pub const fn requires_local_continuity_declaration(self) -> bool {
        matches!(
            self,
            Self::CustomCaBundle | Self::PinnedSshHostProof | Self::MirrorTrustRoot
        )
    }

    /// True when changes to this layer may carry managed authority (admin
    /// policy origin).
    pub const fn may_carry_managed_authority(self) -> bool {
        matches!(
            self,
            Self::CustomCaBundle | Self::ClientCertificate | Self::MirrorTrustRoot
        )
    }
}

// ---------------------------------------------------------------------------
// Trust-store layer health state
// ---------------------------------------------------------------------------

/// Current health state of a trust-store layer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreLayerHealthClass {
    /// The layer is active and all material passes current verification.
    Active,
    /// The layer is active but one or more items are pending revalidation
    /// after a recent change event.
    PendingRevalidation,
    /// The layer has no material enrolled; it is explicitly absent (expected
    /// for optional layers on a given profile).
    Absent,
    /// The layer material is expired or has a known revocation; routes
    /// depending on this layer are blocked until repair completes.
    ExpiredOrRevoked,
    /// The layer is degraded (e.g., the OS bundle is present but the org
    /// overlay failed verification); some routes continue with narrowed scope.
    Degraded,
    /// The layer is inaccessible (OS keychain locked, daemon down, or adapter
    /// misconfigured); all dependent routes are blocked.
    Inaccessible,
}

impl TrustStoreLayerHealthClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::PendingRevalidation => "pending_revalidation",
            Self::Absent => "absent",
            Self::ExpiredOrRevoked => "expired_or_revoked",
            Self::Degraded => "degraded",
            Self::Inaccessible => "inaccessible",
        }
    }

    /// True when this health state allows dependent routes to proceed without
    /// a repair action.
    pub const fn allows_routes(self) -> bool {
        matches!(self, Self::Active | Self::Absent)
    }
}

// ---------------------------------------------------------------------------
// Trust-store change vocabulary
// ---------------------------------------------------------------------------

/// Class of change observed on a trust-store layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreChangeClass {
    /// New trust material was added to the layer.
    Added,
    /// Existing trust material was removed from the layer.
    Removed,
    /// Existing trust material was modified (e.g., renewed or reconfigured).
    Modified,
    /// One item of trust material replaced another in the layer.
    Replaced,
    /// Trust material was explicitly revoked; dependent routes are blocked.
    Revoked,
}

impl TrustStoreChangeClass {
    /// Stable token recorded on events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
            Self::Replaced => "replaced",
            Self::Revoked => "revoked",
        }
    }

    /// True when this change class may immediately block dependent routes
    /// without an explicit user or admin action.
    pub const fn may_block_routes(self) -> bool {
        matches!(self, Self::Removed | Self::Revoked)
    }
}

// ---------------------------------------------------------------------------
// Trust-store change attribution vocabulary
// ---------------------------------------------------------------------------

/// Closed-vocabulary attribution for who or what caused a trust-store change.
///
/// Every change event must name at least one attribution class so the review
/// surface can answer "why did this trust-store layer change?" without
/// exposing raw admin identities, raw policy bundle content, or raw
/// certificate data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreChangeAttributionClass {
    /// The OS platform vendor pushed an update to the system CA bundle.
    OsPlatformUpdate,
    /// An admin-signed managed policy declared this trust material.
    AdminPolicyPush,
    /// A manual import performed by an admin or privileged user.
    ManualAdminImport,
    /// A mirror sync over a declared signed transport updated the layer.
    MirrorSync,
    /// The material expired according to its embedded validity period.
    MaterialExpiry,
    /// An explicit revocation signal was received (CRL, OCSP, policy).
    RevocationSignal,
    /// The OS keychain or credential store was relocked (e.g., session
    /// timeout, screensaver lock, or explicit lock).
    KeychainRelocked,
}

impl TrustStoreChangeAttributionClass {
    /// Stable token recorded on events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsPlatformUpdate => "os_platform_update",
            Self::AdminPolicyPush => "admin_policy_push",
            Self::ManualAdminImport => "manual_admin_import",
            Self::MirrorSync => "mirror_sync",
            Self::MaterialExpiry => "material_expiry",
            Self::RevocationSignal => "revocation_signal",
            Self::KeychainRelocked => "keychain_relocked",
        }
    }

    /// True when this attribution class implies a managed-authority origin
    /// that must be traced back to a policy ref.
    pub const fn implies_managed_authority(self) -> bool {
        matches!(
            self,
            Self::AdminPolicyPush | Self::MirrorSync
        )
    }
}

// ---------------------------------------------------------------------------
// Session and route impact vocabulary
// ---------------------------------------------------------------------------

/// Impact class on active sessions and network routes when a trust-store layer
/// changes.
///
/// Every change event must name a session impact class so the surface can
/// communicate "which requests or sessions are affected?" at the right
/// granularity without exposing raw connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionImpactClass {
    /// The change has no impact on active sessions or routes; traffic
    /// continues unaffected.
    NoImpact,
    /// Affected routes require revalidation before the next request; sessions
    /// already in flight complete normally.
    RevalidationRequired,
    /// Active sessions on affected routes must pause and revalidate before
    /// resuming; local-only work is unaffected.
    SessionMustPause,
    /// The affected routes are blocked until a named repair action completes;
    /// local-only continuity is explicitly preserved.
    RouteBlockedLocalContinuity,
    /// All network routes via this layer are blocked until a named repair
    /// completes; no local-only fallback applies for this layer.
    RouteBlockedNoLocalFallback,
}

impl SessionImpactClass {
    /// Stable token recorded on events.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::RevalidationRequired => "revalidation_required",
            Self::SessionMustPause => "session_must_pause",
            Self::RouteBlockedLocalContinuity => "route_blocked_local_continuity",
            Self::RouteBlockedNoLocalFallback => "route_blocked_no_local_fallback",
        }
    }

    /// True when local-only editing and file operations remain available.
    pub const fn local_work_continues(self) -> bool {
        matches!(
            self,
            Self::NoImpact
                | Self::RevalidationRequired
                | Self::SessionMustPause
                | Self::RouteBlockedLocalContinuity
        )
    }
}

// ---------------------------------------------------------------------------
// Trust-store repair action vocabulary
// ---------------------------------------------------------------------------

/// Typed repair or revalidation action required before a protected network
/// path resumes after a trust-store layer change.
///
/// The action must be named on every change event that blocks or narrows a
/// route so admin and support surfaces can surface actionable guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreRepairActionClass {
    /// No repair is required; the change is non-blocking.
    NoneRequired,
    /// Revalidate the affected routes against the updated OS root bundle.
    RevalidateOsRoots,
    /// Apply and revalidate the updated custom CA bundle declared by the
    /// current managed policy.
    ApplyAndRevalidateCustomCaBundle,
    /// Re-enroll the SSH host proof from the known-hosts or admin-signed
    /// source.
    ReenrollSshHostProof,
    /// Re-enroll or renew the expired or revoked client certificate.
    ReenrollClientCertificate,
    /// Refresh the mirror trust root over a managed signed transport.
    RefreshMirrorTrustRoot,
    /// Import a fresh signed mirror root snapshot from an out-of-band channel.
    ImportSignedMirrorRoot,
    /// Unlock the OS keychain to restore access to client-certificate and
    /// credential material.
    UnlockOsKeychain,
    /// Contact the workspace admin to push an updated trust bundle.
    ContactAdminForUpdatedBundle,
    /// Wait for the OS platform update to complete and then revalidate.
    WaitForOsPlatformUpdate,
}

impl TrustStoreRepairActionClass {
    /// Stable token recorded on events and rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::RevalidateOsRoots => "revalidate_os_roots",
            Self::ApplyAndRevalidateCustomCaBundle => "apply_and_revalidate_custom_ca_bundle",
            Self::ReenrollSshHostProof => "reenroll_ssh_host_proof",
            Self::ReenrollClientCertificate => "reenroll_client_certificate",
            Self::RefreshMirrorTrustRoot => "refresh_mirror_trust_root",
            Self::ImportSignedMirrorRoot => "import_signed_mirror_root",
            Self::UnlockOsKeychain => "unlock_os_keychain",
            Self::ContactAdminForUpdatedBundle => "contact_admin_for_updated_bundle",
            Self::WaitForOsPlatformUpdate => "wait_for_os_platform_update",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the hardened OS keychain and trust-store page.
///
/// The tier is derived, not asserted: it is set by the audit. A caller may
/// never assert `stable` without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenOsKeychainTrustStoreQualificationClass {
    /// All required conditions hold.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required trust-store layer has no row; coverage gap prevents any claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
    Withdrawn,
}

impl HardenOsKeychainTrustStoreQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`HardenOsKeychainTrustStoreQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenOsKeychainTrustStoreNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A required trust-store layer has no row; narrows to preview.
    MissingLayerCoverage,
    /// A change event is missing attribution; cannot explain who caused the change.
    ChangeEventMissingAttribution,
    /// A change event that affects a route or session is missing a repair action.
    ChangeEventMissingRepairAction,
    /// A change event that affects routes is missing affected-route declarations.
    ChangeEventMissingAffectedRoutes,
    /// A change event on a managed-authority layer is missing a policy ref.
    ChangeEventMissingManagedPolicyRef,
    /// A layer that requires local-continuity documentation is missing it.
    LocalContinuityNotExplicit,
    /// A change event is missing a stable repair-transaction ID.
    RepairTransactionIdMissing,
    /// Raw trust material (certificate bodies, private keys, raw fingerprints)
    /// was exposed in a row or event; withdraws the packet immediately.
    RawTrustMaterialExposed,
}

impl HardenOsKeychainTrustStoreNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::MissingLayerCoverage => "missing_layer_coverage",
            Self::ChangeEventMissingAttribution => "change_event_missing_attribution",
            Self::ChangeEventMissingRepairAction => "change_event_missing_repair_action",
            Self::ChangeEventMissingAffectedRoutes => "change_event_missing_affected_routes",
            Self::ChangeEventMissingManagedPolicyRef => "change_event_missing_managed_policy_ref",
            Self::LocalContinuityNotExplicit => "local_continuity_not_explicit",
            Self::RepairTransactionIdMissing => "repair_transaction_id_missing",
            Self::RawTrustMaterialExposed => "raw_trust_material_exposed",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawTrustMaterialExposed)
    }
}

// ---------------------------------------------------------------------------
// Trust-store change event
// ---------------------------------------------------------------------------

/// Attributable, repair-safe event emitted when a trust-store layer changes.
///
/// Change events are the mechanism by which Aureline explains, for any
/// trust-material change: which layer changed, why it changed, which routes
/// or sessions are affected, whether local work can continue, and which narrow
/// repair or revalidation step is required before protected network paths resume.
///
/// Raw certificate bodies, private keys, and raw fingerprints are never
/// included; only opaque refs, type tokens, and closed-vocabulary labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustStoreChangeEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable, durable repair-transaction ID that ties this event to repair
    /// outcome records across admin, support, and export surfaces.
    pub repair_transaction_id: String,
    /// Trust-store layer where the change occurred.
    pub layer: TrustStoreLayerClass,
    /// Stable token for [`Self::layer`].
    pub layer_token: String,
    /// Class of change on the layer.
    pub change_class: TrustStoreChangeClass,
    /// Stable token for [`Self::change_class`].
    pub change_class_token: String,
    /// Attribution: who or what caused this change.
    pub attribution: TrustStoreChangeAttributionClass,
    /// Stable token for [`Self::attribution`].
    pub attribution_token: String,
    /// Plain-language attribution label (export-safe; no raw identities).
    pub attribution_label: String,
    /// Opaque ref to the managed policy that declared this change (required
    /// when [`TrustStoreChangeAttributionClass::implies_managed_authority`]
    /// is true for the attribution class; otherwise may be empty).
    pub managed_policy_ref: String,
    /// Opaque refs to affected route families (closed-vocabulary tokens such
    /// as `tls_enterprise`, `ssh_managed`, `mirror_sync`; no raw hostnames).
    pub affected_route_refs: Vec<String>,
    /// Plain-language label describing which request or session categories
    /// are affected by this change.
    pub affected_sessions_label: String,
    /// Impact class on active sessions and routes.
    pub session_impact: SessionImpactClass,
    /// Stable token for [`Self::session_impact`].
    pub session_impact_token: String,
    /// True when local-only editing, file operations, and non-network work
    /// remain available through this change event.
    pub local_continuity_explicit: bool,
    /// Plain-language label describing what local work remains available.
    pub local_continuity_label: String,
    /// Repair or revalidation action required before protected network paths
    /// resume.
    pub repair_action: TrustStoreRepairActionClass,
    /// Stable token for [`Self::repair_action`].
    pub repair_action_token: String,
    /// Plain-language label describing the repair step.
    pub repair_action_label: String,
    /// True when raw trust material (cert bodies, private keys, fingerprints)
    /// is excluded from this event record.
    pub raw_trust_material_excluded: bool,
    /// UTC instant when the change was first observed.
    pub observed_at: String,
}

// ---------------------------------------------------------------------------
// Trust-store layer row
// ---------------------------------------------------------------------------

/// Trust-store layer row covering one of the five required layers.
///
/// Each row proves:
/// - which layer it covers and its current health state;
/// - the governance source and managed-attribution ref when applicable;
/// - whether local-core continuity is explicitly declared;
/// - that no raw trust material (certificate bodies, private keys, raw
///   fingerprints) is included.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustStoreLayerRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row ID.
    pub row_id: String,
    /// Trust-store layer this row covers.
    pub layer: TrustStoreLayerClass,
    /// Stable token for [`Self::layer`].
    pub layer_token: String,
    /// Current health state of the layer.
    pub health: TrustStoreLayerHealthClass,
    /// Stable token for [`Self::health`].
    pub health_token: String,
    /// Plain-language description of the current layer health.
    pub health_label: String,
    /// Governance source for this layer's trust material (e.g., "OS vendor",
    /// "admin policy bundle", "user-imported").
    pub governance_source_label: String,
    /// Opaque ref to the managed policy or bundle that governs this layer
    /// when [`TrustStoreLayerClass::may_carry_managed_authority`] is true;
    /// otherwise empty.
    pub managed_attribution_ref: String,
    /// True when local-core continuity is explicitly documented for this row.
    pub local_continuity_explicit: bool,
    /// Plain-language label describing what remains available locally when
    /// this layer is degraded or blocked.
    pub local_continuity_label: String,
    /// Repair action required when the layer is not in a healthy state.
    pub repair_action: TrustStoreRepairActionClass,
    /// Stable token for [`Self::repair_action`].
    pub repair_action_token: String,
    /// True when raw trust material (certificate bodies, private keys,
    /// fingerprints) is excluded from this record.
    pub raw_trust_material_excluded: bool,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed`).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate summary for the hardened OS keychain and trust-store page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HardenOsKeychainTrustStoreSummary {
    /// Total layer-row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Layer tokens present on the page.
    pub layers_covered: Vec<String>,
    /// Health state tokens present across rows.
    pub health_states_present: Vec<String>,
    /// Change event count across all layers.
    pub change_event_count: usize,
    /// Repair transaction IDs present across change events.
    pub repair_transaction_ids_present: Vec<String>,
    /// Number of rows with `local_continuity_explicit: true`.
    pub local_continuity_explicit_row_count: usize,
    /// Number of rows with `raw_trust_material_excluded: true`.
    pub raw_material_excluded_row_count: usize,
    /// Overall qualification token.
    pub overall_qualification_token: String,
}

impl HardenOsKeychainTrustStoreSummary {
    fn from_rows(
        rows: &[TrustStoreLayerRow],
        change_events: &[TrustStoreChangeEvent],
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut layers: BTreeSet<String> = BTreeSet::new();
        let mut health_states: BTreeSet<String> = BTreeSet::new();
        let mut tx_ids: BTreeSet<String> = BTreeSet::new();
        let mut local_ok = 0usize;
        let mut raw_ok = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            layers.insert(row.layer_token.clone());
            health_states.insert(row.health_token.clone());
            if row.local_continuity_explicit {
                local_ok += 1;
            }
            if row.raw_trust_material_excluded {
                raw_ok += 1;
            }
        }

        for event in change_events {
            if !event.repair_transaction_id.is_empty() {
                tx_ids.insert(event.repair_transaction_id.clone());
            }
        }

        let overall = if withdrawn > 0 {
            HardenOsKeychainTrustStoreQualificationClass::Withdrawn
        } else if preview > 0 {
            HardenOsKeychainTrustStoreQualificationClass::Preview
        } else if beta > 0 {
            HardenOsKeychainTrustStoreQualificationClass::Beta
        } else {
            HardenOsKeychainTrustStoreQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            layers_covered: layers.into_iter().collect(),
            health_states_present: health_states.into_iter().collect(),
            change_event_count: change_events.len(),
            repair_transaction_ids_present: tx_ids.into_iter().collect(),
            local_continuity_explicit_row_count: local_ok,
            raw_material_excluded_row_count: raw_ok,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the hardened OS keychain and trust-store audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenOsKeychainTrustStoreDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect ID.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: HardenOsKeychainTrustStoreNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject ID (row ID, event repair-transaction-id, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl HardenOsKeychainTrustStoreDefect {
    fn new(
        narrow_reason: HardenOsKeychainTrustStoreNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: HARDEN_OS_KEYCHAIN_TRUST_STORE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:harden-os-keychain-trust-store:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for hardened OS keychain and trust-store integration,
/// trust-store change detection, and repair-safe copy.
///
/// This is the single inspectable record that proves all five required
/// trust-store layers are covered, every trust-material change carries
/// attribution and a repair action, and local-core continuity is preserved
/// through every failure mode. Dashboards, docs, Help/About surfaces, and
/// support exports should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenOsKeychainTrustStorePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page ID.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows and events.
    pub summary: HardenOsKeychainTrustStoreSummary,
    /// Per-layer qualification rows (one per trust-store layer).
    pub rows: Vec<TrustStoreLayerRow>,
    /// Attributable trust-store change events embedded as evidence.
    pub change_events: Vec<TrustStoreChangeEvent>,
    /// Typed validation defects.
    pub defects: Vec<HardenOsKeychainTrustStoreDefect>,
}

impl HardenOsKeychainTrustStorePage {
    /// Build the page from a set of layer rows and change events.
    ///
    /// Defects are derived automatically from the audit.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<TrustStoreLayerRow>,
        change_events: Vec<TrustStoreChangeEvent>,
    ) -> Self {
        let defects = audit_trust_store_rows(&rows, &change_events);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = HardenOsKeychainTrustStoreSummary::from_rows(&qualified_rows, &change_events);
        Self {
            record_kind: HARDEN_OS_KEYCHAIN_TRUST_STORE_PAGE_RECORD_KIND.to_owned(),
            schema_version: HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            change_events,
            defects,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == HardenOsKeychainTrustStoreQualificationClass::Stable.as_str()
    }

    /// True when all five required trust-store layers are covered.
    pub fn covers_all_required_layers(&self) -> bool {
        let covered: BTreeSet<&str> = self.rows.iter().map(|r| r.layer_token.as_str()).collect();
        TrustStoreLayerClass::ALL
            .iter()
            .all(|l| covered.contains(l.as_str()))
    }

    /// True when every row excludes raw trust material.
    pub fn all_rows_exclude_raw_trust_material(&self) -> bool {
        self.rows.iter().all(|r| r.raw_trust_material_excluded)
    }

    /// True when every row that requires local-continuity documentation carries it.
    pub fn all_required_local_continuity_declared(&self) -> bool {
        self.rows.iter().all(|r| {
            if r.layer.requires_local_continuity_declaration() {
                r.local_continuity_explicit
            } else {
                true
            }
        })
    }

    /// True when all change events carry a non-empty repair-transaction ID.
    pub fn all_change_events_have_repair_transaction_ids(&self) -> bool {
        self.change_events
            .iter()
            .all(|e| !e.repair_transaction_id.is_empty())
    }

    /// True when every change event carries attribution.
    pub fn all_change_events_have_attribution(&self) -> bool {
        self.change_events
            .iter()
            .all(|e| !e.attribution_token.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper for the hardened OS keychain and trust-store page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenOsKeychainTrustStoreSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export ID.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The trust-store page embedded as evidence.
    pub page: HardenOsKeychainTrustStorePage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<HardenOsKeychainTrustStoreNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw trust material is excluded from the export.
    pub raw_trust_material_excluded: bool,
}

impl HardenOsKeychainTrustStoreSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: HardenOsKeychainTrustStorePage,
    ) -> Self {
        let mut reasons: Vec<HardenOsKeychainTrustStoreNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: HARDEN_OS_KEYCHAIN_TRUST_STORE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_trust_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the trust-store hardening audit over the rows and change events.
pub fn audit_harden_os_keychain_trust_store_page(
    page: &HardenOsKeychainTrustStorePage,
) -> Vec<HardenOsKeychainTrustStoreDefect> {
    audit_trust_store_rows(&page.rows, &page.change_events)
}

/// Validate the page; returns `Ok` when the audit is clean.
pub fn validate_harden_os_keychain_trust_store_page(
    page: &HardenOsKeychainTrustStorePage,
) -> Result<(), Vec<HardenOsKeychainTrustStoreDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded hardened OS keychain and trust-store page covering all five
/// required trust-store layers with attribution, repair actions, local-continuity
/// declarations, and repair-safe change events.
pub fn seeded_harden_os_keychain_trust_store_page() -> HardenOsKeychainTrustStorePage {
    let rows = seeded_rows();
    let events = seeded_change_events();
    HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:seeded:0001",
        "Hardened OS keychain and trust-store integration, change detection, and repair-safe copy",
        "2026-06-01T00:00:00Z",
        rows,
        events,
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_trust_store_rows(
    rows: &[TrustStoreLayerRow],
    change_events: &[TrustStoreChangeEvent],
) -> Vec<HardenOsKeychainTrustStoreDefect> {
    let mut defects: Vec<HardenOsKeychainTrustStoreDefect> = Vec::new();

    // Hard guardrail: raw trust material exposed in any row.
    for row in rows {
        if !row.raw_trust_material_excluded {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::RawTrustMaterialExposed,
                row.row_id.clone(),
                "row does not exclude raw trust material (certificate bodies, private keys, fingerprints); packet is withdrawn",
            ));
        }
    }

    // Hard guardrail: raw trust material exposed in any change event.
    for event in change_events {
        if !event.raw_trust_material_excluded {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::RawTrustMaterialExposed,
                event.repair_transaction_id.clone(),
                "change event does not exclude raw trust material; packet is withdrawn",
            ));
        }
    }

    // If any withdrawal reason was found, return immediately.
    if defects.iter().any(|d| d.narrow_reason.is_withdrawal_reason()) {
        return defects;
    }

    // Per-row checks.
    for row in rows {
        // Rows that carry managed authority must have a managed attribution ref.
        if row.layer.may_carry_managed_authority() && row.managed_attribution_ref.is_empty() {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingManagedPolicyRef,
                row.row_id.clone(),
                "managed-authority layer row must carry a managed_attribution_ref",
            ));
        }

        // Rows that require local-continuity documentation must carry it.
        if row.layer.requires_local_continuity_declaration() && !row.local_continuity_explicit {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::LocalContinuityNotExplicit,
                row.row_id.clone(),
                "this trust-store layer requires an explicit local_continuity_explicit: true declaration",
            ));
        }
    }

    // Per-change-event checks.
    for event in change_events {
        // Every change event must carry a non-empty repair-transaction ID.
        if event.repair_transaction_id.is_empty() {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::RepairTransactionIdMissing,
                "change_event",
                "change event is missing a stable repair_transaction_id",
            ));
        }

        // Every event must carry attribution.
        if event.attribution_token.is_empty() {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingAttribution,
                event.repair_transaction_id.clone(),
                "change event is missing an attribution token",
            ));
        }

        // Events that affect routes must name at least one affected route ref.
        let impacts_routes = !matches!(event.session_impact, SessionImpactClass::NoImpact);
        if impacts_routes && event.affected_route_refs.is_empty() {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingAffectedRoutes,
                event.repair_transaction_id.clone(),
                "change event that impacts routes must declare affected_route_refs",
            ));
        }

        // Events that block or pause routes must name a repair action.
        let requires_repair = matches!(
            event.session_impact,
            SessionImpactClass::SessionMustPause
                | SessionImpactClass::RouteBlockedLocalContinuity
                | SessionImpactClass::RouteBlockedNoLocalFallback
        );
        if requires_repair
            && matches!(event.repair_action, TrustStoreRepairActionClass::NoneRequired)
        {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingRepairAction,
                event.repair_transaction_id.clone(),
                "change event that blocks or pauses routes must name a typed repair action",
            ));
        }

        // Managed-authority attribution requires a managed policy ref.
        if event.attribution.implies_managed_authority() && event.managed_policy_ref.is_empty() {
            defects.push(HardenOsKeychainTrustStoreDefect::new(
                HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingManagedPolicyRef,
                event.repair_transaction_id.clone(),
                "change event with managed-authority attribution must carry a managed_policy_ref",
            ));
        }
    }

    // Coverage check: all five required layers must appear at least once.
    let required_layers: BTreeSet<&str> =
        TrustStoreLayerClass::ALL.iter().map(|l| l.as_str()).collect();
    let observed_layers: BTreeSet<&str> =
        rows.iter().map(|r| r.layer_token.as_str()).collect();
    for missing in required_layers.difference(&observed_layers) {
        defects.push(HardenOsKeychainTrustStoreDefect::new(
            HardenOsKeychainTrustStoreNarrowReasonClass::MissingLayerCoverage,
            "page",
            format!("missing row for required trust-store layer '{missing}'; packet narrowed to preview"),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<TrustStoreLayerRow>,
    page_defects: &[HardenOsKeychainTrustStoreDefect],
) -> Vec<TrustStoreLayerRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == HardenOsKeychainTrustStoreNarrowReasonClass::MissingLayerCoverage
    });

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(HardenOsKeychainTrustStoreNarrowReasonClass::RawTrustMaterialExposed);
        (HardenOsKeychainTrustStoreQualificationClass::Withdrawn, r)
    } else if has_preview {
        (
            HardenOsKeychainTrustStoreQualificationClass::Preview,
            HardenOsKeychainTrustStoreNarrowReasonClass::MissingLayerCoverage,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (HardenOsKeychainTrustStoreQualificationClass::Beta, r)
    } else {
        (
            HardenOsKeychainTrustStoreQualificationClass::Stable,
            HardenOsKeychainTrustStoreNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        let row_qual = if has_withdrawal {
            HardenOsKeychainTrustStoreQualificationClass::Withdrawn
        } else if has_preview {
            HardenOsKeychainTrustStoreQualificationClass::Preview
        } else {
            let row_has_defect = page_defects
                .iter()
                .any(|d| d.source == row.row_id);
            if row_has_defect || !page_defects.is_empty() {
                HardenOsKeychainTrustStoreQualificationClass::Beta
            } else {
                HardenOsKeychainTrustStoreQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(HardenOsKeychainTrustStoreNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary =
            build_row_summary(&row.row_id, &row.layer_token, row_qual, row_reason);
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    layer_token: &str,
    qual: HardenOsKeychainTrustStoreQualificationClass,
    narrow_reason: HardenOsKeychainTrustStoreNarrowReasonClass,
) -> String {
    match qual {
        HardenOsKeychainTrustStoreQualificationClass::Stable => format!(
            "Row '{row_id}' (layer: {layer_token}) qualifies stable: \
             layer covered, attribution declared, repair actions explicit, \
             local-core continuity documented, raw trust material excluded."
        ),
        HardenOsKeychainTrustStoreQualificationClass::Beta => format!(
            "Row '{row_id}' (layer: {layer_token}) narrowed to beta \
             (reason: {}): one or more required conditions are unmet.",
            narrow_reason.as_str()
        ),
        HardenOsKeychainTrustStoreQualificationClass::Preview => format!(
            "Row '{row_id}' (layer: {layer_token}) narrowed to preview: \
             a required trust-store layer is missing from the page."
        ),
        HardenOsKeychainTrustStoreQualificationClass::Withdrawn => format!(
            "Row '{row_id}' (layer: {layer_token}) is withdrawn \
             (reason: {}): hard guardrail triggered.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded layer rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<TrustStoreLayerRow> {
    vec![
        row_os_roots(),
        row_custom_ca_bundle(),
        row_pinned_ssh_host_proof(),
        row_client_certificate(),
        row_mirror_trust_root(),
    ]
}

fn make_row(
    row_id: &str,
    layer: TrustStoreLayerClass,
    health: TrustStoreLayerHealthClass,
    health_label: &str,
    governance_source_label: &str,
    managed_attribution_ref: &str,
    local_continuity_explicit: bool,
    local_continuity_label: &str,
    repair_action: TrustStoreRepairActionClass,
) -> TrustStoreLayerRow {
    TrustStoreLayerRow {
        record_kind: HARDEN_OS_KEYCHAIN_TRUST_STORE_ROW_RECORD_KIND.to_owned(),
        schema_version: HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
        shared_contract_ref: HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        layer,
        layer_token: layer.as_str().to_owned(),
        health,
        health_token: health.as_str().to_owned(),
        health_label: health_label.to_owned(),
        governance_source_label: governance_source_label.to_owned(),
        managed_attribution_ref: managed_attribution_ref.to_owned(),
        local_continuity_explicit,
        local_continuity_label: local_continuity_label.to_owned(),
        repair_action,
        repair_action_token: repair_action.as_str().to_owned(),
        raw_trust_material_excluded: true,
        // Filled in by qualify_rows.
        qualification_token: HardenOsKeychainTrustStoreQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: HardenOsKeychainTrustStoreNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn row_os_roots() -> TrustStoreLayerRow {
    make_row(
        "harden-os-keychain-trust-store:os_roots",
        TrustStoreLayerClass::OsRoots,
        TrustStoreLayerHealthClass::Active,
        "OS platform CA bundle is active and current; managed routes use system roots unless \
         a custom CA bundle is declared by admin policy.",
        "OS vendor / platform distribution",
        "",
        true,
        "Local editing, file operations, and non-TLS work are unaffected by OS root changes. \
         Only routes using TLS against OS roots require revalidation after an OS bundle update.",
        TrustStoreRepairActionClass::NoneRequired,
    )
}

fn row_custom_ca_bundle() -> TrustStoreLayerRow {
    make_row(
        "harden-os-keychain-trust-store:custom_ca_bundle",
        TrustStoreLayerClass::CustomCaBundle,
        TrustStoreLayerHealthClass::Active,
        "Custom CA bundle declared by admin policy is active; enterprise TLS routes validate \
         against this bundle.",
        "Admin policy bundle (managed authority)",
        "policy:trust-store:custom-ca-bundle:managed-ref:v1",
        true,
        "Local editing, local file operations, and OS-root-only routes remain available. \
         Only enterprise TLS routes pinned to the custom CA bundle require revalidation \
         on bundle change.",
        TrustStoreRepairActionClass::NoneRequired,
    )
}

fn row_pinned_ssh_host_proof() -> TrustStoreLayerRow {
    make_row(
        "harden-os-keychain-trust-store:pinned_ssh_host_proof",
        TrustStoreLayerClass::PinnedSshHostProof,
        TrustStoreLayerHealthClass::Active,
        "Pinned SSH host proofs are enrolled and match the current known-hosts for managed \
         Git-over-SSH endpoints.",
        "Admin-signed known-hosts declaration",
        "",
        true,
        "Local editing and HTTPS-based Git routes remain available. Only Git-over-SSH sessions \
         to managed endpoints require revalidation when a host proof changes.",
        TrustStoreRepairActionClass::NoneRequired,
    )
}

fn row_client_certificate() -> TrustStoreLayerRow {
    make_row(
        "harden-os-keychain-trust-store:client_certificate",
        TrustStoreLayerClass::ClientCertificate,
        TrustStoreLayerHealthClass::Active,
        "Client certificate material is enrolled in the OS keychain and accessible to managed \
         mTLS routes.",
        "Admin policy / OS keychain enrollment",
        "policy:trust-store:client-cert:managed-ref:v1",
        true,
        "Local editing and non-mTLS routes remain available. Only routes requiring mutual TLS \
         are blocked when client-certificate material becomes inaccessible or expires.",
        TrustStoreRepairActionClass::NoneRequired,
    )
}

fn row_mirror_trust_root() -> TrustStoreLayerRow {
    make_row(
        "harden-os-keychain-trust-store:mirror_trust_root",
        TrustStoreLayerClass::MirrorTrustRoot,
        TrustStoreLayerHealthClass::Active,
        "Mirror trust roots for declared signed mirrors and air-gapped artifact repositories \
         are active and verified.",
        "Admin-signed mirror trust bundle",
        "policy:trust-store:mirror-trust-root:managed-ref:v1",
        true,
        "Local editing and OS-root TLS routes remain available. Only mirror-sync and \
         air-gapped artifact routes require revalidation when a mirror trust root changes.",
        TrustStoreRepairActionClass::NoneRequired,
    )
}

// ---------------------------------------------------------------------------
// Seeded change events
// ---------------------------------------------------------------------------

fn seeded_change_events() -> Vec<TrustStoreChangeEvent> {
    vec![
        event_os_bundle_update(),
        event_custom_ca_policy_push(),
        event_ssh_host_proof_added(),
        event_client_cert_renewed(),
        event_mirror_trust_root_synced(),
    ]
}

fn make_event(
    repair_transaction_id: &str,
    layer: TrustStoreLayerClass,
    change_class: TrustStoreChangeClass,
    attribution: TrustStoreChangeAttributionClass,
    attribution_label: &str,
    managed_policy_ref: &str,
    affected_route_refs: Vec<&str>,
    affected_sessions_label: &str,
    session_impact: SessionImpactClass,
    local_continuity_label: &str,
    repair_action: TrustStoreRepairActionClass,
    repair_action_label: &str,
    observed_at: &str,
) -> TrustStoreChangeEvent {
    let local_continuity_explicit = session_impact.local_work_continues();
    TrustStoreChangeEvent {
        record_kind: HARDEN_OS_KEYCHAIN_TRUST_STORE_CHANGE_EVENT_RECORD_KIND.to_owned(),
        schema_version: HARDEN_OS_KEYCHAIN_TRUST_STORE_SCHEMA_VERSION,
        shared_contract_ref: HARDEN_OS_KEYCHAIN_TRUST_STORE_SHARED_CONTRACT_REF.to_owned(),
        repair_transaction_id: repair_transaction_id.to_owned(),
        layer,
        layer_token: layer.as_str().to_owned(),
        change_class,
        change_class_token: change_class.as_str().to_owned(),
        attribution,
        attribution_token: attribution.as_str().to_owned(),
        attribution_label: attribution_label.to_owned(),
        managed_policy_ref: managed_policy_ref.to_owned(),
        affected_route_refs: affected_route_refs.iter().map(|s| s.to_string()).collect(),
        affected_sessions_label: affected_sessions_label.to_owned(),
        session_impact,
        session_impact_token: session_impact.as_str().to_owned(),
        local_continuity_explicit,
        local_continuity_label: local_continuity_label.to_owned(),
        repair_action,
        repair_action_token: repair_action.as_str().to_owned(),
        repair_action_label: repair_action_label.to_owned(),
        raw_trust_material_excluded: true,
        observed_at: observed_at.to_owned(),
    }
}

fn event_os_bundle_update() -> TrustStoreChangeEvent {
    make_event(
        "txn:trust-store:os-bundle-update:2026-06-01:0001",
        TrustStoreLayerClass::OsRoots,
        TrustStoreChangeClass::Modified,
        TrustStoreChangeAttributionClass::OsPlatformUpdate,
        "OS platform vendor updated the system CA bundle via a platform security update.",
        "",
        vec!["tls_os_roots", "tls_enterprise"],
        "Enterprise TLS routes using OS roots require revalidation; local editing and \
         non-TLS work continue without interruption.",
        SessionImpactClass::RevalidationRequired,
        "Local editing, file operations, search, and non-TLS routes continue unaffected. \
         Enterprise TLS routes using OS roots will revalidate on the next connection \
         attempt.",
        TrustStoreRepairActionClass::RevalidateOsRoots,
        "Wait for the OS platform update to complete, then revalidate TLS routes against \
         the updated OS root bundle. No user action is required unless a route fails \
         after revalidation.",
        "2026-06-01T00:00:00Z",
    )
}

fn event_custom_ca_policy_push() -> TrustStoreChangeEvent {
    make_event(
        "txn:trust-store:custom-ca-policy-push:2026-06-01:0002",
        TrustStoreLayerClass::CustomCaBundle,
        TrustStoreChangeClass::Replaced,
        TrustStoreChangeAttributionClass::AdminPolicyPush,
        "Admin policy bundle was updated with a new custom CA bundle for enterprise TLS routes.",
        "policy:trust-store:custom-ca-bundle:managed-ref:v1",
        vec!["tls_enterprise", "tls_custom_ca"],
        "Enterprise TLS routes validated against the custom CA bundle require revalidation; \
         all other routes and local work are unaffected.",
        SessionImpactClass::RevalidationRequired,
        "Local editing and OS-root TLS routes remain fully available. Enterprise routes \
         pinned to the custom CA bundle revalidate on the next connection attempt.",
        TrustStoreRepairActionClass::ApplyAndRevalidateCustomCaBundle,
        "Apply the updated custom CA bundle declared by the current managed policy, then \
         revalidate enterprise TLS routes. Contact the workspace admin if revalidation fails.",
        "2026-06-01T00:00:00Z",
    )
}

fn event_ssh_host_proof_added() -> TrustStoreChangeEvent {
    make_event(
        "txn:trust-store:ssh-host-proof-added:2026-06-01:0003",
        TrustStoreLayerClass::PinnedSshHostProof,
        TrustStoreChangeClass::Added,
        TrustStoreChangeAttributionClass::AdminPolicyPush,
        "A new SSH host proof was added by admin policy for a managed Git-over-SSH endpoint.",
        "policy:trust-store:ssh-host-proof:managed-ref:v1",
        vec!["ssh_managed"],
        "New Git-over-SSH sessions to the enrolled managed endpoint are now protected by \
         the pinned host proof; existing sessions are unaffected.",
        SessionImpactClass::NoImpact,
        "Local editing, HTTPS-based Git routes, and all non-SSH work continue unaffected. \
         The new host proof only applies to future SSH sessions to the enrolled endpoint.",
        TrustStoreRepairActionClass::NoneRequired,
        "No repair action is required; the new SSH host proof was added successfully.",
        "2026-06-01T00:00:00Z",
    )
}

fn event_client_cert_renewed() -> TrustStoreChangeEvent {
    make_event(
        "txn:trust-store:client-cert-renewed:2026-06-01:0004",
        TrustStoreLayerClass::ClientCertificate,
        TrustStoreChangeClass::Replaced,
        TrustStoreChangeAttributionClass::AdminPolicyPush,
        "Client certificate was renewed by admin policy; the previous certificate approached \
         its expiry and was replaced with a fresh enrollment.",
        "policy:trust-store:client-cert:managed-ref:v1",
        vec!["tls_mtls_enterprise"],
        "Active mTLS sessions may need to re-present the renewed certificate on the next \
         handshake; local editing and non-mTLS routes are unaffected.",
        SessionImpactClass::RevalidationRequired,
        "Local editing, local file operations, and non-mTLS routes remain fully available. \
         Only mTLS routes to managed endpoints require re-presentation of the renewed \
         certificate on the next connection.",
        TrustStoreRepairActionClass::ReenrollClientCertificate,
        "The renewed client certificate has been enrolled in the OS keychain. Re-present \
         the certificate on the next mTLS connection attempt. No user action is required \
         if the OS keychain is unlocked.",
        "2026-06-01T00:00:00Z",
    )
}

fn event_mirror_trust_root_synced() -> TrustStoreChangeEvent {
    make_event(
        "txn:trust-store:mirror-trust-root-synced:2026-06-01:0005",
        TrustStoreLayerClass::MirrorTrustRoot,
        TrustStoreChangeClass::Modified,
        TrustStoreChangeAttributionClass::MirrorSync,
        "Mirror trust root was refreshed via a signed mirror-sync transport; the admin-signed \
         bundle was updated with a current mirror root.",
        "policy:trust-store:mirror-trust-root:managed-ref:v1",
        vec!["mirror_sync", "airgap_artifacts"],
        "Mirror-sync and air-gapped artifact routes will revalidate against the updated \
         mirror trust root on the next sync attempt; local editing is unaffected.",
        SessionImpactClass::RevalidationRequired,
        "Local editing, OS-root TLS routes, and enterprise routes not using mirror trust \
         roots remain fully available. Only mirror-sync and air-gapped artifact routes \
         revalidate on the next sync.",
        TrustStoreRepairActionClass::RefreshMirrorTrustRoot,
        "The mirror trust root has been refreshed via the signed mirror-sync transport. \
         Mirror-sync and air-gapped artifact routes will revalidate on the next sync \
         attempt. No user action is required.",
        "2026-06-01T00:00:00Z",
    )
}
